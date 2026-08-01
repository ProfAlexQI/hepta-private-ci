#[test]
fn hepta_telegram_base_matrix_link_server_context_boundary_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_SERVER_CONTEXT_BOUNDARY_MARKER,
        "hepta_telegram_matrix_link_server_context_boundary_ready"
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_SERVER_CONTEXT_BOUNDARY_EVIDENCE
            .contains("clicked target metadata")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_SERVER_CONTEXT_BOUNDARY_EVIDENCE
            .contains("compact PreviewMatrixLinkTarget status")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_SERVER_CONTEXT_BOUNDARY_EVIDENCE
            .contains("cached Server context refresh")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_SERVER_CONTEXT_BOUNDARY_EVIDENCE
            .contains("event context fetch")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_SERVER_CONTEXT_BOUNDARY_EVIDENCE
            .contains("timeline pagination/reload")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_SERVER_CONTEXT_BOUNDARY_EVIDENCE
            .contains("MatrixRequest::BackwardsPaginateUntilEvent")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_SERVER_CONTEXT_BOUNDARY_EVIDENCE
            .contains("confirmed MatrixRequest::JoinRoomByIdOrAlias")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_SERVER_CONTEXT_BOUNDARY_EVIDENCE
            .contains("confirmed MatrixRequest::Knock")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_SERVER_CONTEXT_BOUNDARY_EVIDENCE
            .contains("confirmed MatrixRequest::InviteUser")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_SERVER_CONTEXT_BOUNDARY_EVIDENCE
            .contains("external browser handoff before confirmation")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_SERVER_CONTEXT_BOUNDARY_EVIDENCE
            .contains("source-only preview event source fetch stays on compact preview; full remote event source fetch")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_SERVER_CONTEXT_BOUNDARY_EVIDENCE
            .contains("gateway/runtime/auth")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_SERVER_CONTEXT_BOUNDARY_LABEL.contains(
            "cached room-or-alias Join/Knock, current-room user Invite, and source-only preview fetch are confirmed"
        )
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"matrix_link_server_context_boundary"));
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"matrix_link_resolution"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "Matrix link server context boundary"
            && capability
                .base_module
                .contains("matrix_link_server_context_boundary")
            && capability.notes.contains("PreviewMatrixLinkTarget")
            && capability.notes.contains("via server count")
            && capability.notes.contains("retry cache readiness")
            && capability
                .notes
                .contains("cached Server context refresh")
            && capability.notes.contains("event context fetch")
            && capability.notes.contains("timeline pagination/reload")
            && capability
                .notes
                .contains("MatrixRequest::BackwardsPaginateUntilEvent")
            && capability
                .notes
                .contains("confirmed MatrixRequest::JoinRoomByIdOrAlias")
            && capability.notes.contains("confirmed MatrixRequest::Knock")
            && capability.notes.contains("confirmed MatrixRequest::InviteUser")
            && capability
                .notes
                .contains("external browser handoff before confirmation")
            && capability.notes.contains("source-only preview event source fetch stays on compact preview; full remote event source fetch")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
            && capability.notes.contains("matrix_link_resolution")
    }));
}

#[test]
fn hepta_telegram_base_matrix_link_context_actions_row_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_CONTEXT_ACTIONS_ROW_MARKER,
        "hepta_telegram_matrix_link_context_actions_row_ready"
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_CONTEXT_ACTIONS_ROW_EVIDENCE
            .contains("Server, Event, Alias, Join, Knock, Invite, Browser, and Source")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_CONTEXT_ACTIONS_ROW_EVIDENCE
            .contains("Clicking Server uses the cached room id or alias target")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_CONTEXT_ACTIONS_ROW_EVIDENCE
            .contains("Clicking Join parses the cached room id or alias target")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_CONTEXT_ACTIONS_ROW_EVIDENCE
            .contains("Clicking Knock parses the same cached room id or alias target")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_CONTEXT_ACTIONS_ROW_EVIDENCE
            .contains("Clicking Invite parses the cached Matrix user target")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_CONTEXT_ACTIONS_ROW_EVIDENCE
            .contains("Clicking Browser builds a cached matrix.to URL")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_CONTEXT_ACTIONS_ROW_EVIDENCE
            .contains("Source may open the existing local EventSourceModal")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_CONTEXT_ACTIONS_ROW_EVIDENCE
            .contains("confirmed failed-state Retry")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_CONTEXT_ACTIONS_ROW_EVIDENCE
            .contains("MatrixRequest::BackwardsPaginateUntilEvent")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_CONTEXT_ACTIONS_ROW_EVIDENCE
            .contains("cached Server context refresh")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_CONTEXT_ACTIONS_ROW_EVIDENCE
            .contains("event context fetch")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_CONTEXT_ACTIONS_ROW_LABEL
            .contains("user Invite confirms")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"matrix_link_context_actions_row"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"matrix_link_room_or_alias_join_live_wiring")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"matrix_link_room_or_alias_knock_live_wiring")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"matrix_link_user_invite_live_wiring"));
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"matrix_link_resolution"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "Matrix link context actions row"
            && capability
                .base_module
                .contains("matrix_link_context_actions")
            && capability
                .base_module
                .contains("matrix_link_server_context_packet_snapshot_label")
            && capability
                .notes
                .contains("Server, Event, Alias, Join, Knock, Invite, Browser, and Source")
            && capability.notes.contains("Server uses a cached room id or alias target")
            && capability.notes.contains("Event renders a local Matrix link server-context packet snapshot")
            && capability
                .notes
                .contains("Clicking Join parses the cached room id or alias")
            && capability.notes.contains("MatrixLinkJoinResultAction")
            && capability
                .notes
                .contains("Clicking Knock parses the same cached room id or alias")
            && capability.notes.contains("MatrixRequest::Knock")
            && capability.notes.contains("KnockResultAction")
            && capability
                .notes
                .contains("Clicking Invite parses the cached Matrix user target")
            && capability.notes.contains("MatrixRequest::InviteUser")
            && capability.notes.contains("InviteResultAction")
            && capability
                .notes
                .contains("Browser builds a cached matrix.to URL")
            && capability.notes.contains("PositiveConfirmationModal")
            && capability.notes.contains("system opener")
            && capability.notes.contains("confirmed failed-state Retry")
            && capability
                .notes
                .contains("Source may open the existing EventSourceModal")
            && capability.notes.contains("BackwardsPaginateUntilEvent")
            && capability
                .notes
                .contains("cached Server context refresh")
            && capability.notes.contains("event context fetch")
            && capability.notes.contains("join")
            && capability.notes.contains("knock")
            && capability.notes.contains("unconfirmed browser handoff")
            && capability.notes.contains("source-only preview event source fetch stays on compact preview; full remote event source fetch")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
            && capability.notes.contains("matrix_link_resolution")
    }));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "Matrix link user Invite live wiring"
            && capability.base_module.contains("MatrixRequest::InviteUser")
            && capability.base_module.contains("InviteResultAction")
            && capability.notes.contains("cached Matrix user id targets")
            && capability.notes.contains("currently loaded room")
            && capability.notes.contains("PositiveConfirmationModal")
            && capability.notes.contains("failed-state room/user retry")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("matrix_link_resolution")
    }));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "Matrix link room-or-alias Join live wiring"
            && capability
                .base_module
                .contains("MatrixRequest::JoinRoomByIdOrAlias")
            && capability
                .base_module
                .contains("MatrixLinkJoinResultAction")
            && capability.notes.contains("cached room id or alias targets")
            && capability.notes.contains("PositiveConfirmationModal")
            && capability
                .notes
                .contains("failed-state room-or-alias retry")
            && capability.notes.contains("cached via servers")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("matrix_link_resolution")
    }));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "Matrix link room-or-alias Knock live wiring"
            && capability.base_module.contains("MatrixRequest::Knock")
            && capability.base_module.contains("KnockResultAction")
            && capability.notes.contains("cached room id or alias targets")
            && capability.notes.contains("PositiveConfirmationModal")
            && capability
                .notes
                .contains("failed-state room-or-alias retry")
            && capability.notes.contains("cached via servers")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("matrix_link_resolution")
    }));
}

#[test]
fn hepta_telegram_base_matrix_link_browser_handoff_confirmation_is_confirmed() {
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_BROWSER_HANDOFF_CONFIRMATION_MARKER,
        "hepta_telegram_matrix_link_browser_handoff_confirmation_ready"
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_BROWSER_HANDOFF_CONFIRMATION_EVIDENCE
            .contains("cached preview target")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_BROWSER_HANDOFF_CONFIRMATION_EVIDENCE
            .contains("matrix.to URL")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_BROWSER_HANDOFF_CONFIRMATION_EVIDENCE
            .contains("PositiveConfirmationModal")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_BROWSER_HANDOFF_CONFIRMATION_EVIDENCE
            .contains("system opener")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_BROWSER_HANDOFF_CONFIRMATION_EVIDENCE
            .contains("missing cached target and cancel stay warning-only/local")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_BROWSER_HANDOFF_CONFIRMATION_LABEL
            .contains("matrix.to system opener")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"matrix_link_browser_handoff_confirmation")
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"matrix_link_resolution"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "Matrix link browser handoff confirmation"
            && capability
                .base_module
                .contains("show_telegram_matrix_link_browser_confirmation")
            && capability.base_module.contains("PositiveConfirmationModal")
            && capability.base_module.contains("matrix.to system opener")
            && capability
                .notes
                .contains("cached RoomScreen preview-strip target")
            && capability.notes.contains("matrix.to URL")
            && capability.notes.contains("show_external_link_confirmation")
            && capability.notes.contains("robius_open system opener")
            && capability
                .notes
                .contains("cancel and missing cached target stay local")
            && capability.notes.contains("PreviewMatrixLinkTarget")
            && capability.notes.contains("BackwardsPaginateUntilEvent")
            && capability.notes.contains("server-side alias resolution")
            && capability.notes.contains("event context fetch")
            && capability.notes.contains("join")
            && capability.notes.contains("knock")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("Telegram delivery")
            && capability.notes.contains("live mutation")
            && capability.notes.contains("matrix_link_resolution")
    }));
}

#[test]
fn hepta_telegram_base_matrix_link_route_scope_controls_are_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_ROUTE_SCOPE_CONTROLS_MARKER,
        "hepta_telegram_matrix_link_route_scope_controls_ready"
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_ROUTE_SCOPE_CONTROLS_EVIDENCE
            .contains("Room, Event, Via, Preview, and Source")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_ROUTE_SCOPE_CONTROLS_EVIDENCE
            .contains("Packet, Contract, and Taxonomy")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_ROUTE_SCOPE_CONTROLS_EVIDENCE
            .contains("Clicking Room copies only the cached Matrix link target label")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_ROUTE_SCOPE_CONTROLS_EVIDENCE
            .contains("Clicking Via copies only the cached Matrix link via server list")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_ROUTE_SCOPE_CONTROLS_EVIDENCE
            .contains("Clicking Event copies only the cached requested Matrix event id")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_ROUTE_SCOPE_CONTROLS_EVIDENCE
            .contains("Clicking Preview copies only the already cached local preview metadata")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_ROUTE_SCOPE_CONTROLS_EVIDENCE
            .contains("Source may open the existing local EventSourceModal")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_ROUTE_SCOPE_CONTROLS_EVIDENCE
            .contains("cached Matrix link target label/status/via/event metadata")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_ROUTE_SCOPE_CONTROLS_EVIDENCE
            .contains("confirmed failed-state Retry")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_ROUTE_SCOPE_CONTROLS_EVIDENCE
            .contains("MatrixRequest::BackwardsPaginateUntilEvent")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_ROUTE_SCOPE_CONTROLS_EVIDENCE
            .contains("server-side alias resolution")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_ROUTE_SCOPE_CONTROLS_EVIDENCE
            .contains("event context fetch")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_ROUTE_SCOPE_CONTROLS_LABEL
            .contains("Room copies cached target metadata")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_ROUTE_SCOPE_CONTROLS_LABEL
            .contains("Taxonomy stay local")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"matrix_link_route_scope_controls_row"));
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"matrix_link_resolution"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "Matrix link route-scope controls row"
            && capability
                .base_module
                .contains("matrix_link_route_scope_controls")
            && capability
                .notes
                .contains("Room, Event, Via, Preview, Source, Packet, Contract, and Taxonomy")
            && capability
                .notes
                .contains("Room copies only the cached Matrix link target label")
            && capability
                .notes
                .contains("Via copies only the cached Matrix link via server list")
            && capability
                .notes
                .contains("Event copies only the cached requested Matrix event id")
            && capability
                .notes
                .contains("Preview copies only the already cached preview metadata")
            && capability
                .notes
                .contains("cached Matrix link target label/status/via/event metadata")
            && capability.notes.contains("confirmed failed-state Retry")
            && capability
                .notes
                .contains("Taxonomy adds route/event-context result slots")
            && capability
                .notes
                .contains("Source may open the existing EventSourceModal")
            && capability.notes.contains("BackwardsPaginateUntilEvent")
            && capability.notes.contains("server-side alias resolution")
            && capability.notes.contains("event context fetch")
            && capability.notes.contains("join")
            && capability.notes.contains("knock")
            && capability.notes.contains("source-only preview event source fetch stays on compact preview; full remote event source fetch")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
            && capability.notes.contains("matrix_link_resolution")
    }));
}

#[test]
fn hepta_telegram_base_matrix_link_room_target_clipboard_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_ROOM_TARGET_CLIPBOARD_MARKER,
        "hepta_telegram_matrix_link_room_target_clipboard_ready"
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_ROOM_TARGET_CLIPBOARD_EVIDENCE
            .contains("cached Matrix link target label")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_ROOM_TARGET_CLIPBOARD_EVIDENCE
            .contains("local clipboard")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_ROOM_TARGET_CLIPBOARD_EVIDENCE
            .contains("Missing target label stays local-unavailable")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_ROOM_TARGET_CLIPBOARD_EVIDENCE
            .contains("PreviewMatrixLinkTarget is limited to compact preview, confirmed Retry, or cached Server context refresh")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_ROOM_TARGET_CLIPBOARD_LABEL
            .contains("cached target metadata")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"matrix_link_room_target_clipboard_action")
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"matrix_link_resolution"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "Matrix link room target clipboard"
            && capability
                .base_module
                .contains("copy_telegram_matrix_link_room_target")
            && capability.notes.contains("cached Matrix link target label")
            && capability.notes.contains("local clipboard")
            && capability
                .notes
                .contains("Missing target label stays local-unavailable")
            && capability
                .notes
                .contains("PreviewMatrixLinkTarget is limited to compact preview, confirmed Retry, or cached Server context refresh")
            && capability.notes.contains("BackwardsPaginateUntilEvent")
            && capability.notes.contains("server-side alias resolution")
            && capability.notes.contains("event context fetch")
            && capability.notes.contains("source-only preview event source fetch stays on compact preview; full remote event source fetch")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
            && capability.notes.contains("matrix_link_resolution")
    }));
}

#[test]
fn hepta_telegram_base_matrix_link_via_servers_clipboard_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_VIA_SERVERS_CLIPBOARD_MARKER,
        "hepta_telegram_matrix_link_via_servers_clipboard_ready"
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_VIA_SERVERS_CLIPBOARD_EVIDENCE
            .contains("cached Matrix link via server list")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_VIA_SERVERS_CLIPBOARD_EVIDENCE
            .contains("local clipboard")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_VIA_SERVERS_CLIPBOARD_EVIDENCE
            .contains("Missing via server list stays local-unavailable")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_VIA_SERVERS_CLIPBOARD_EVIDENCE
            .contains("PreviewMatrixLinkTarget is limited to compact preview, confirmed Retry, or cached Server context refresh")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_VIA_SERVERS_CLIPBOARD_LABEL
            .contains("cached via server list")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"matrix_link_via_servers_clipboard_action")
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"matrix_link_resolution"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "Matrix link via servers clipboard"
            && capability
                .base_module
                .contains("copy_telegram_matrix_link_via_servers")
            && capability
                .notes
                .contains("cached Matrix link via server list")
            && capability.notes.contains("local clipboard")
            && capability
                .notes
                .contains("Missing via server list stays local-unavailable")
            && capability
                .notes
                .contains("PreviewMatrixLinkTarget is limited to compact preview, confirmed Retry, or cached Server context refresh")
            && capability.notes.contains("BackwardsPaginateUntilEvent")
            && capability.notes.contains("server-side alias resolution")
            && capability.notes.contains("event context fetch")
            && capability.notes.contains("source-only preview event source fetch stays on compact preview; full remote event source fetch")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
            && capability.notes.contains("matrix_link_resolution")
    }));
}

#[test]
fn hepta_telegram_base_matrix_link_event_id_clipboard_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_EVENT_ID_CLIPBOARD_MARKER,
        "hepta_telegram_matrix_link_event_id_clipboard_ready"
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_EVENT_ID_CLIPBOARD_EVIDENCE
            .contains("cached requested Matrix event id")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_EVENT_ID_CLIPBOARD_EVIDENCE
            .contains("local clipboard")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_EVENT_ID_CLIPBOARD_EVIDENCE
            .contains("Missing event id stays local-unavailable")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_EVENT_ID_CLIPBOARD_EVIDENCE
            .contains("PreviewMatrixLinkTarget is limited to compact preview, confirmed Retry, or cached Server context refresh")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_EVENT_ID_CLIPBOARD_LABEL
            .contains("cached requested event id")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"matrix_link_event_id_clipboard_action")
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"matrix_link_resolution"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "Matrix link event id clipboard"
            && capability
                .base_module
                .contains("copy_telegram_matrix_link_event_id")
            && capability
                .notes
                .contains("cached requested Matrix event id")
            && capability.notes.contains("local clipboard")
            && capability
                .notes
                .contains("Missing event id stays local-unavailable")
            && capability
                .notes
                .contains("PreviewMatrixLinkTarget is limited to compact preview, confirmed Retry, or cached Server context refresh")
            && capability.notes.contains("BackwardsPaginateUntilEvent")
            && capability.notes.contains("server-side alias resolution")
            && capability.notes.contains("event context fetch")
            && capability.notes.contains("source-only preview event source fetch stays on compact preview; full remote event source fetch")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
            && capability.notes.contains("matrix_link_resolution")
    }));
}

#[test]
fn hepta_telegram_base_matrix_link_preview_metadata_clipboard_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_PREVIEW_METADATA_CLIPBOARD_MARKER,
        "hepta_telegram_matrix_link_preview_metadata_clipboard_ready"
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_PREVIEW_METADATA_CLIPBOARD_EVIDENCE
            .contains("already cached preview strip metadata")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_PREVIEW_METADATA_CLIPBOARD_EVIDENCE
            .contains("local clipboard")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_PREVIEW_METADATA_CLIPBOARD_EVIDENCE
            .contains("Missing metadata stays local-unavailable")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_PREVIEW_METADATA_CLIPBOARD_EVIDENCE
            .contains("PreviewMatrixLinkTarget is limited to compact preview, confirmed Retry, or cached Server context refresh")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_PREVIEW_METADATA_CLIPBOARD_LABEL
            .contains("cached local preview metadata")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"matrix_link_preview_metadata_clipboard_action")
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"matrix_link_resolution"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "Matrix link preview metadata clipboard"
            && capability
                .base_module
                .contains("copy_telegram_matrix_link_preview_metadata")
            && capability
                .notes
                .contains("already cached RoomScreen preview-strip metadata")
            && capability.notes.contains("local clipboard")
            && capability
                .notes
                .contains("Missing metadata stays local-unavailable")
            && capability
                .notes
                .contains("PreviewMatrixLinkTarget is limited to compact preview, confirmed Retry, or cached Server context refresh")
            && capability.notes.contains("BackwardsPaginateUntilEvent")
            && capability.notes.contains("server-side alias resolution")
            && capability.notes.contains("event context fetch")
            && capability.notes.contains("source-only preview event source fetch stays on compact preview; full remote event source fetch")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
            && capability.notes.contains("matrix_link_resolution")
    }));
}

#[test]
fn hepta_telegram_base_matrix_link_route_drilldown_packet_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_ROUTE_DRILLDOWN_PACKET_MARKER,
        "hepta_telegram_matrix_link_route_drilldown_packet_ready"
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_ROUTE_DRILLDOWN_PACKET_EVIDENCE
            .contains("per-target route drilldown")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_ROUTE_DRILLDOWN_PACKET_EVIDENCE
            .contains("cached preview-strip state only")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_ROUTE_DRILLDOWN_PACKET_EVIDENCE
            .contains("server-context packet")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_ROUTE_DRILLDOWN_PACKET_EVIDENCE
            .contains("alias resolution")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_ROUTE_DRILLDOWN_PACKET_EVIDENCE
            .contains("event context fetch")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_ROUTE_DRILLDOWN_PACKET_LABEL
            .contains("per-target route acceptance criteria")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"matrix_link_route_drilldown_packet_action")
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"matrix_link_resolution"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "Matrix link route drilldown packet"
            && capability
                .base_module
                .contains("copy_telegram_matrix_link_route_drilldown_packet")
            && capability.notes.contains("per-target route drilldown")
            && capability
                .notes
                .contains("cached RoomScreen preview-strip state")
            && capability.notes.contains("server-context packet")
            && capability.notes.contains("alias resolution")
            && capability.notes.contains("join/knock/invite")
            && capability
                .notes
                .contains("PreviewMatrixLinkTarget is limited to compact preview, confirmed Retry, or cached Server context refresh")
            && capability.notes.contains("BackwardsPaginateUntilEvent")
            && capability.notes.contains("event context fetch")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
            && capability.notes.contains("matrix_link_resolution")
    }));
}

#[test]
fn hepta_telegram_base_matrix_link_route_result_contract_packet_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_ROUTE_RESULT_CONTRACT_PACKET_MARKER,
        "hepta_telegram_matrix_link_route_result_contract_packet_ready"
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_ROUTE_RESULT_CONTRACT_PACKET_EVIDENCE
            .contains("typed route/result contract packet")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_ROUTE_RESULT_CONTRACT_PACKET_EVIDENCE
            .contains("server-context packet")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_ROUTE_RESULT_CONTRACT_PACKET_EVIDENCE
            .contains("alias resolution")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_ROUTE_RESULT_CONTRACT_PACKET_EVIDENCE
            .contains("event context")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_ROUTE_RESULT_CONTRACT_PACKET_EVIDENCE
            .contains("join/knock/invite")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_ROUTE_RESULT_CONTRACT_PACKET_LABEL
            .contains("typed route/result contracts")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"matrix_link_route_result_contract_packet_action")
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"matrix_link_resolution"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "Matrix link route result contract packet"
            && capability
                .base_module
                .contains("copy_telegram_matrix_link_route_result_contract_packet")
            && capability
                .base_module
                .contains("matrix_link_route_result_contract_packet_payload")
            && capability
                .notes
                .contains("typed route/result contract packet")
            && capability.notes.contains("target identity")
            && capability.notes.contains("preview request/result/error")
            && capability.notes.contains("loaded alias")
            && capability.notes.contains("event route")
            && capability.notes.contains("via route")
            && capability.notes.contains("join/knock/invite")
            && capability.notes.contains("external browser handoff")
            && capability.notes.contains("remote source")
            && capability.notes.contains("source-hash")
            && capability
                .notes
                .contains("PreviewMatrixLinkTarget is limited to compact preview, confirmed Retry, or cached Server context refresh")
            && capability.notes.contains("BackwardsPaginateUntilEvent")
            && capability.notes.contains("event context fetch")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
            && capability.notes.contains("matrix_link_resolution")
    }));
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|runway| {
        runway.gap_id == "matrix_link_resolution"
            && runway
                .current_path
                .contains("typed route/result contract packet")
            && runway
                .next_ui_safe_step
                .contains("Matrix route/result contracts")
    }));
}

#[test]
fn hepta_telegram_base_matrix_link_route_result_taxonomy_packet_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_ROUTE_RESULT_TAXONOMY_PACKET_MARKER,
        "hepta_telegram_matrix_link_route_result_taxonomy_packet_ready"
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_ROUTE_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("route/event-context result taxonomy packet")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_ROUTE_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("MatrixRequest::PreviewMatrixLinkTarget")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_ROUTE_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("MatrixRequest::BackwardsPaginateUntilEvent")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_ROUTE_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("non_current_room_event_context_operation_id")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_ROUTE_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("full_remote_source_request_id")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_ROUTE_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("no server-side alias resolution")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_ROUTE_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("no event-context fetch")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"matrix_link_route_result_taxonomy_packet_action")
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"matrix_link_resolution"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "Matrix link route result taxonomy packet"
            && capability
                .base_module
                .contains("copy_telegram_matrix_link_route_result_taxonomy_packet")
            && capability
                .base_module
                .contains("matrix_link_route_result_taxonomy_packet_payload")
            && capability
                .notes
                .contains("route/event-context result taxonomy packet")
            && capability.notes.contains("route_adapter_request_id")
            && capability.notes.contains("alias_resolution_operation_id")
            && capability
                .notes
                .contains("non_current_room_event_context_operation_id")
            && capability.notes.contains("full_remote_source_request_id")
            && capability.notes.contains("PreviewMatrixLinkTarget")
            && capability.notes.contains("BackwardsPaginateUntilEvent")
            && capability.notes.contains("server-side alias resolution")
            && capability.notes.contains("event-context fetch")
            && capability.notes.contains("gateway/runtime/auth/provider")
            && capability.notes.contains("live mutation")
            && capability.notes.contains("matrix_link_resolution")
    }));
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|runway| {
        runway.gap_id == "matrix_link_resolution"
            && runway
                .current_path
                .contains("route/event-context result taxonomy packet")
            && runway
                .next_ui_safe_step
                .contains("Matrix route/result contracts")
    }));
}

#[test]
fn hepta_telegram_base_matrix_link_unresolved_detail_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MATRIX_LINK_UNRESOLVED_DETAIL_MARKER,
        "hepta_telegram_matrix_link_unresolved_detail_ready"
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_UNRESOLVED_DETAIL_EVIDENCE
            .contains("MatrixRequest::PreviewMatrixLinkTarget")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_UNRESOLVED_DETAIL_EVIDENCE
            .contains("Server/Event/Alias/Join/Knock/Source")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_UNRESOLVED_DETAIL_EVIDENCE
            .contains("metadata character count")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_UNRESOLVED_DETAIL_EVIDENCE
            .contains("cached Server context refresh")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_UNRESOLVED_DETAIL_EVIDENCE
            .contains("event context fetch")
    );
    assert!(
        crate::home::room_screen::MATRIX_LINK_UNRESOLVED_DETAIL_LABEL
            .contains("Unresolved Matrix link detail is local")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"matrix_link_unresolved_detail_state"));
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"matrix_link_resolution"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "Matrix link unresolved detail state"
            && capability
                .base_module
                .contains("matrix_link_unresolved_detail")
            && capability.notes.contains("PreviewMatrixLinkTarget")
            && capability
                .notes
                .contains("Server/Event/Alias/Join/Knock/Source")
            && capability.notes.contains("metadata character count")
            && capability.notes.contains("server-side alias resolution")
            && capability.notes.contains("event context fetch")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
            && capability.notes.contains("matrix_link_resolution")
    }));
}

#[test]
fn hepta_telegram_base_external_link_is_confirmation_guarded() {
    assert_eq!(
        HEPTA_TELEGRAM_EXTERNAL_LINK_CONFIRMATION_MARKER,
        "hepta_telegram_external_link_confirmation_ready"
    );
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"external_link_confirmation"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"external_link_confirmation_guard"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "external link confirmation"
            && capability.notes.contains("ordinary URL links")
            && capability.notes.contains("unhandled Matrix links")
            && capability.notes.contains("local confirmation modal")
            && capability
                .notes
                .contains("before the existing external browser handoff is requested")
            && capability.notes.contains("opening the confirmation")
            && capability.notes.contains("Cancel")
            && capability.notes.contains("guard display")
            && capability.notes.contains("no browser handoff")
            && capability.notes.contains("Matrix event fetch")
            && capability.notes.contains("room preview fetch")
            && capability.notes.contains("message send")
            && capability.notes.contains("room-state")
            && capability.notes.contains("membership")
            && capability.notes.contains("live mutation request")
    }));
}

#[test]
fn hepta_telegram_base_event_source_surface_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_EVENT_SOURCE_LOCAL_SURFACE_MARKER,
        "hepta_telegram_event_source_local_surface_ready"
    );
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"event_source_fetch"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"event_source_local_surface"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "event source local surface"
            && capability.base_module == "EventSourceModal + NewMessageContextMenu"
            && capability
                .notes
                .contains("already loaded timeline event data")
            && capability
                .notes
                .contains("Copy Room ID, Copy Event ID, Copy Source")
            && capability.notes.contains("open, and Close")
            && capability.notes.contains("write local clipboard text")
            && capability.notes.contains("no Matrix event source request")
            && capability.notes.contains("event fetch")
            && capability.notes.contains("message send")
            && capability.notes.contains("room-state")
            && capability.notes.contains("membership")
            && capability.notes.contains("live mutation request")
    }));
}

#[test]
fn hepta_telegram_base_event_source_loaded_metadata_summary_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_EVENT_SOURCE_LOADED_METADATA_MARKER,
        "hepta_telegram_event_source_loaded_metadata_ready"
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"event_source_loaded_metadata_summary"));
    assert!(
        crate::home::event_source_modal::EVENT_SOURCE_LOADED_METADATA_EVIDENCE
            .contains("already loaded room id, event id")
    );
    assert!(
        crate::home::event_source_modal::EVENT_SOURCE_LOADED_METADATA_EVIDENCE
            .contains("latest JSON source availability")
    );
    assert!(
        crate::home::event_source_modal::EVENT_SOURCE_LOADED_METADATA_EVIDENCE
            .contains("byte/line counts")
    );
    assert!(
        crate::home::event_source_modal::EVENT_SOURCE_LOADED_METADATA_EVIDENCE
            .contains("no Matrix event source request")
    );
    assert!(
        crate::home::event_source_modal::EVENT_SOURCE_LOADED_METADATA_EVIDENCE
            .contains("gateway/runtime/auth")
    );
    assert!(
        crate::home::event_source_modal::EVENT_SOURCE_LOADED_METADATA_LABEL
            .contains("no event fetch")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "event source loaded metadata summary"
            && capability.base_module == "EventSourceModal loaded metadata label"
            && capability.notes.contains("already loaded room id")
            && capability.notes.contains("local source byte/line counts")
            && capability.notes.contains("no Matrix event source request")
            && capability.notes.contains("event context fetch")
            && capability.notes.contains("timeline pagination/reload")
            && capability.notes.contains("message send/edit/redact")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation request")
    }));
}

#[test]
fn hepta_telegram_base_message_copy_surface_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_COPY_LOCAL_SURFACE_MARKER,
        "hepta_telegram_message_copy_local_surface_ready"
    );
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"message_copy"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"message_copy_local_surface"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message copy local surface"
            && capability
                .notes
                .contains("Copy Text, Copy Text as HTML, and Copy Link")
            && capability.notes.contains("loaded timeline item data")
            && capability.notes.contains("loaded formatted bodies")
            && capability
                .notes
                .contains("locally constructed matrix.to URIs")
            && capability.notes.contains("only write local clipboard text")
            && capability.notes.contains("no Matrix event fetch")
            && capability.notes.contains("message send")
            && capability.notes.contains("edit")
            && capability.notes.contains("redact")
            && capability.notes.contains("room-state")
            && capability.notes.contains("membership")
            && capability.notes.contains("live mutation request")
    }));
}

#[test]
fn hepta_telegram_base_message_copy_loaded_metadata_summary_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_COPY_LOADED_METADATA_MARKER,
        "hepta_telegram_message_copy_loaded_metadata_ready"
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"message_copy_loaded_metadata_summary"));
    assert!(
        crate::home::room_screen::MESSAGE_COPY_LOADED_METADATA_EVIDENCE
            .contains("already loaded clipboard payload metadata")
    );
    assert!(
        crate::home::room_screen::MESSAGE_COPY_LOADED_METADATA_EVIDENCE
            .contains("payload kind, event-id availability, character count, and byte count")
    );
    assert!(
        crate::home::room_screen::MESSAGE_COPY_LOADED_METADATA_EVIDENCE
            .contains("same loaded timeline body")
    );
    assert!(
        crate::home::room_screen::MESSAGE_COPY_LOADED_METADATA_EVIDENCE
            .contains("locally constructed matrix.to URI")
    );
    assert!(
        crate::home::room_screen::MESSAGE_COPY_LOADED_METADATA_EVIDENCE
            .contains("no Matrix event fetch")
    );
    assert!(
        crate::home::room_screen::MESSAGE_COPY_LOADED_METADATA_EVIDENCE
            .contains("gateway/runtime/auth")
    );
    assert!(
        crate::home::room_screen::MESSAGE_COPY_LOADED_METADATA_LABEL.contains("no event fetch")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message copy loaded metadata summary"
            && capability.base_module == "RoomScreen clipboard popup metadata"
            && capability
                .notes
                .contains("already loaded clipboard payload metadata")
            && capability.notes.contains("payload kind")
            && capability.notes.contains("event-id availability")
            && capability.notes.contains("character count")
            && capability.notes.contains("byte count")
            && capability.notes.contains("loaded formatted body")
            && capability
                .notes
                .contains("locally constructed matrix.to URI")
            && capability.notes.contains("no Matrix event fetch")
            && capability.notes.contains("event source request")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation request")
    }));
}

#[test]
fn hepta_telegram_base_message_delete_is_confirmation_guarded() {
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_DELETE_CONFIRMATION_MARKER,
        "hepta_telegram_message_delete_confirmation_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message delete confirmation"
            && capability.base_module == "NewMessageContextMenu + ConfirmationModal"
            && capability
                .notes
                .contains("before the existing Matrix redaction path is requested")
            && capability.notes.contains("cancel keeps the request unsent")
    }));
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"message_delete"));
}

#[test]
fn hepta_telegram_base_message_pin_is_confirmation_guarded() {
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_PIN_CONFIRMATION_MARKER,
        "hepta_telegram_message_pin_confirmation_ready"
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"message_pin_confirmation_guard"));
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"message_pin"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message pin confirmation"
            && capability
                .notes
                .contains("before the existing Matrix PinEvent path is requested")
            && capability.notes.contains("Cancel keeps the request unsent")
    }));
}

#[test]
fn hepta_telegram_base_message_edit_is_confirmation_guarded() {
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_EDIT_CONFIRMATION_MARKER,
        "hepta_telegram_message_edit_confirmation_ready"
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"message_edit_confirmation_guard"));
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"edit_message"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message edit confirmation"
            && capability
                .notes
                .contains("before the existing Matrix EditMessage path")
            && capability
                .notes
                .contains("Cancel keeps the edit request unsent")
    }));
}

#[test]
fn hepta_telegram_base_message_edit_unsupported_features_are_local_evidence() {
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_EDIT_UNSUPPORTED_FEATURES_LOCAL_MARKER,
        "hepta_telegram_message_edit_unsupported_features_local_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_EDIT_SAVE_RESULT_MAPPING_PACKET_MARKER,
        "hepta_telegram_message_edit_save_result_mapping_packet_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_EDIT_ATTACHMENT_PREFLIGHT_PACKET_MARKER,
        "hepta_telegram_message_edit_attachment_preflight_packet_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_EDIT_MENTION_PAYLOAD_PREFLIGHT_PACKET_MARKER,
        "hepta_telegram_message_edit_mention_payload_preflight_packet_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_EDIT_MENTION_PAYLOAD_TYPED_CONTRACT_PACKET_MARKER,
        "hepta_telegram_message_edit_mention_payload_typed_contract_packet_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_EDIT_RETRY_ERROR_DRILLDOWN_PACKET_MARKER,
        "hepta_telegram_message_edit_retry_error_drilldown_packet_ready"
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_edit_unsupported_features_local_evidence")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"message_edit_detail_packet_preview"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_edit_attachment_preflight_packet_preview")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_edit_mention_payload_preflight_packet_preview")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_edit_mention_payload_typed_contract_packet_preview")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_edit_save_result_mapping_packet_preview")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_edit_retry_error_drilldown_packet_preview")
    );
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"edit_message"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message edit unsupported feature local evidence"
            && capability.base_module == "EditingPane"
            && capability.notes.contains("HTML/plain prefixes")
            && capability.notes.contains("attachment add/remove")
            && capability.notes.contains("mention extraction")
            && capability.notes.contains("poll answer edits")
            && capability.notes.contains("Save spinner")
            && capability.notes.contains("Edit/Poll detail packet")
            && capability.notes.contains("attachment preflight")
            && capability.notes.contains("mention payload preflight")
            && capability.notes.contains("mention payload typed contract")
            && capability.notes.contains("content_kind")
            && capability.notes.contains("attachment_edit_slot not_built")
            && capability.notes.contains("mention_payload_scope")
            && capability.notes.contains("poll_answer_edit_slot not_built")
            && capability
                .notes
                .contains("save_spinner_operation_id not_assigned")
            && capability.notes.contains("result_mapping not_wired")
            && capability.notes.contains("stale_result_policy")
            && capability
                .notes
                .contains("existing Matrix EditMessage confirmation path")
            && capability.notes.contains("no attachment upload/remove")
            && capability.notes.contains("Matrix mention payload")
            && capability.notes.contains("poll answer edit")
            && capability.notes.contains("timeline reload")
            && capability.notes.contains("room-state")
            && capability.notes.contains("membership request")
    }));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message edit attachment preflight packet"
            && capability.notes.contains("original_attachment_scope")
            && capability
                .notes
                .contains("selected_attachment_slot unavailable")
            && capability.notes.contains("add_attachment_slot not_built")
            && capability
                .notes
                .contains("remove_attachment_slot not_built")
            && capability
                .notes
                .contains("replace_attachment_slot not_built")
            && capability.notes.contains("upload_request_slot not_built")
            && capability.notes.contains("media_delete_slot not_built")
            && capability.notes.contains(
                "caption_edit_handoff existing_confirmed_MatrixRequest_EditMessage_body_only",
            )
            && capability.notes.contains("mime_size_probe not_started")
            && capability
                .notes
                .contains("retry_policy no_duplicate_upload_without_operation_id")
            && capability
                .notes
                .contains("cancel_policy leaves_original_media_and_local_selection_untouched")
            && capability.notes.contains("no SendAttachment")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message edit mention payload preflight packet"
            && capability.notes.contains("edited_at_token_count")
            && capability.notes.contains("literal_user_id_token_count")
            && capability.notes.contains("room_token_scope")
            && capability
                .notes
                .contains("completed_pill_reconcile_slot not_connected_to_editing_pane")
            && capability
                .notes
                .contains("directory_result_scope unavailable_in_editing_pane")
            && capability
                .notes
                .contains("fresh_mentions_payload_slot not_built")
            && capability.notes.contains("existing_mentions_handoff")
            && capability.notes.contains("reply_sendtime_state not_reused")
            && capability.notes.contains("retry_source_hash_slot missing")
            && capability
                .notes
                .contains("stale_token_policy backend_required_before_live_mentions")
            && capability
                .notes
                .contains("cancel_policy confirmation_cancel_no_request")
            && capability
                .notes
                .contains("no fresh Matrix Mentions payload")
            && capability.notes.contains("profile lookup")
            && capability.notes.contains("directory search")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message edit mention payload typed contract packet"
            && capability
                .notes
                .contains("mention_contract_version local_v0")
            && capability
                .notes
                .contains("token_scan_source edited_text_only")
            && capability.notes.contains("literal_user_id_contract_count")
            && capability.notes.contains("room_token_contract_scope")
            && capability
                .notes
                .contains("directory_snapshot_id_slot unavailable")
            && capability
                .notes
                .contains("completed_pill_snapshot_slot unavailable")
            && capability.notes.contains("source_hash_slot not_assigned")
            && capability
                .notes
                .contains("fresh_mentions_payload_result_slot not_built")
            && capability
                .notes
                .contains("retry_idempotency_key_slot missing")
            && capability
                .notes
                .contains("stale_result_guard body_source_hash_required_before_live_mentions")
            && capability
                .notes
                .contains("privacy_redaction token_counts_only")
            && capability
                .notes
                .contains("no fresh Matrix Mentions payload")
            && capability.notes.contains("directory snapshot reuse")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message edit save-result mapping packet"
            && capability.notes.contains("lifecycle_state")
            && capability.notes.contains("operation_id_slot not_assigned")
            && capability
                .notes
                .contains("request_slot existing_confirmed_MatrixRequest_EditMessage")
            && capability.notes.contains("spinner_slot not_rendered")
            && capability.notes.contains("saved_hide_pane")
            && capability.notes.contains("failed_popup")
            && capability.notes.contains("canceled_no_request")
            && capability.notes.contains("stale_event_id_ignored")
            && capability
                .notes
                .contains("ignored_late_result_without_matching_operation_id")
            && capability
                .notes
                .contains("stale_result_guard timeline_event_item_id_match_only")
            && capability
                .notes
                .contains("repeated_save_policy not_held_until_pending_operation_id")
            && capability.notes.contains("retry_slot not_built")
            && capability
                .notes
                .contains("confirmation-gated Matrix EditMessage")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message edit retry/error drilldown packet"
            && capability.notes.contains("failure_source")
            && capability
                .notes
                .contains("existing_MatrixRequest_EditMessage_result_only")
            && capability
                .notes
                .contains("error_redaction popup_text_not_persisted_or_reused")
            && capability.notes.contains("retry_request_slot not_built")
            && capability
                .notes
                .contains("retry_confirmation_slot not_built")
            && capability.notes.contains(
                "late_result_guard timeline_event_item_id_match_only_without_operation_id",
            )
            && capability
                .notes
                .contains("pending_operation_id missing_backend_contract")
            && capability.notes.contains("spinner_state not_rendered")
            && capability
                .notes
                .contains("cancel_state confirmation_cancel_no_request")
            && capability
                .notes
                .contains("stale_result_policy ignore_late_result_without_matching_operation_id")
            && capability
                .notes
                .contains("existing confirmation-gated Matrix EditMessage")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_composer_emoji_placeholder_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_EMOJI_PLACEHOLDER_MARKER,
        "hepta_telegram_composer_emoji_placeholder_local_only"
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"emoji_sticker_picker_composer"));
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"emoji_sticker_picker_composer"));
}

#[test]
fn hepta_telegram_base_composer_emoji_surface_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_EMOJI_LOCAL_SURFACE_MARKER,
        "hepta_telegram_composer_emoji_local_surface_ready"
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"emoji_sticker_surface"));
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"emoji_sticker_surface"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "composer"
            && capability.notes.contains("local emoji/sticker surface")
            && capability.notes.contains("Smile, Thumbs, Heart, Sticker")
    }));
}

#[test]
fn hepta_telegram_base_composer_emoji_send_boundary_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_EMOJI_SEND_LOCAL_BOUNDARY_MARKER,
        "hepta_telegram_composer_emoji_send_local_boundary_ready"
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"emoji_sticker_send_local_boundary_evidence")
    );
    assert!(
        crate::room::room_input_bar::EMOJI_STICKER_SEND_LOCAL_BOUNDARY_EVIDENCE
            .contains("do not insert composer text")
    );
    assert!(
        crate::room::room_input_bar::EMOJI_STICKER_SEND_LOCAL_BOUNDARY_EVIDENCE
            .contains("submit MatrixRequest::SendMessage")
    );
    assert!(
        crate::room::room_input_bar::EMOJI_STICKER_SEND_LOCAL_BOUNDARY_EVIDENCE
            .contains("submit MatrixRequest::SendAttachment")
    );
    assert!(
        crate::room::room_input_bar::EMOJI_STICKER_SEND_LOCAL_BOUNDARY_LABEL
            .contains("no composer insert")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "emoji/sticker send local boundary evidence"
            && capability.notes.contains("repeated selection")
            && capability.notes.contains("Close")
            && capability.notes.contains("SendMessage")
            && capability.notes.contains("SendAttachment")
            && capability.notes.contains("SDK send-queue")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_composer_emoji_lifecycle_metadata_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_EMOJI_LIFECYCLE_METADATA_MARKER,
        "hepta_telegram_composer_emoji_lifecycle_metadata_ready"
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"emoji_sticker_lifecycle_metadata_preview")
    );
    assert!(
        crate::room::room_input_bar::EMOJI_STICKER_LIFECYCLE_METADATA_EVIDENCE
            .contains("panel visibility")
    );
    assert!(
        crate::room::room_input_bar::EMOJI_STICKER_LIFECYCLE_METADATA_EVIDENCE
            .contains("last staged choice")
    );
    assert!(
        crate::room::room_input_bar::EMOJI_STICKER_LIFECYCLE_METADATA_EVIDENCE
            .contains("staged choice count")
    );
    assert!(
        crate::room::room_input_bar::EMOJI_STICKER_LIFECYCLE_METADATA_EVIDENCE
            .contains("remote picker/search")
    );
    assert!(
        crate::room::room_input_bar::EMOJI_STICKER_LIFECYCLE_METADATA_LABEL
            .contains("close/reopen state")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "emoji/sticker lifecycle metadata"
            && capability.base_module.contains("emoji_lifecycle_metadata")
            && capability.notes.contains("Opening")
            && capability
                .notes
                .contains("repeated Smile/Thumbs/Heart/Sticker")
            && capability.notes.contains("last staged choice")
            && capability.notes.contains("staged choice count")
            && capability.notes.contains("remote picker/search")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation request")
    }));
}

#[test]
fn hepta_telegram_base_composer_voice_placeholder_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_VOICE_PLACEHOLDER_MARKER,
        "hepta_telegram_composer_voice_placeholder_local_only"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"voice_message_send"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"voice_message_composer"));
}

#[test]
fn hepta_telegram_base_composer_voice_surface_is_local_only() {
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
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"voice_message_send"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"voice_message_surface"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "voice message send"
            && capability.notes.contains("guarded voice surface")
            && capability
                .notes
                .contains("Send can choose an existing desktop audio file")
            && capability
                .notes
                .contains("true voice-message recorder UX remain TODO")
    }));
}

#[test]
fn hepta_telegram_base_composer_voice_option_staging_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_VOICE_OPTION_STAGING_LOCAL_MARKER,
        "hepta_telegram_composer_voice_option_staging_local_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_VOICE_OPTION_STAGING_AUDIO_MARKER,
        "hepta_telegram_composer_voice_option_staging_audio_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"voice_message_send"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"voice_option_staging_local_evidence"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "voice option staging local evidence"
            && capability.notes.contains("Record, Lock, Cancel, and Close")
            && capability
                .notes
                .contains("Send opens a confirmation before the desktop audio-file picker")
            && capability.notes.contains("no microphone permission")
            && capability
                .notes
                .contains("true recorder-based voice message send remains TODO")
    }));
}

#[test]
fn hepta_telegram_base_composer_voice_send_is_locally_blocked() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_VOICE_SEND_LOCAL_BLOCKED_MARKER,
        "hepta_telegram_composer_voice_send_local_blocked_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_VOICE_SEND_AUDIO_HANDOFF_MARKER,
        "hepta_telegram_composer_voice_send_audio_handoff_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"voice_message_send"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"voice_message_send_local_blocked_evidence")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_SEND_LOCAL_BLOCKED_EVIDENCE
            .contains("never requests microphone permission")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_SEND_LOCAL_BLOCKED_EVIDENCE
            .contains("Voice Send can open a desktop audio-file confirmation")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_SEND_LOCAL_BLOCKED_EVIDENCE
            .contains("sends a caption/text fallback")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_SEND_LOCAL_BLOCKED_LABEL
            .contains("confirmed review SendAttachment")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "voice message send local blocked evidence"
            && capability
                .notes
                .contains("voice_message_send remains a base gap")
            && capability.notes.contains("no microphone permission")
            && capability.notes.contains("MatrixRequest::SendAttachment")
            && capability.notes.contains("caption/text fallback")
            && capability
                .notes
                .contains("hidden SDK send-queue work before review Send")
    }));
}

#[test]
fn hepta_telegram_base_composer_voice_send_live_wiring_is_partial_live() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_VOICE_SEND_LIVE_WIRING_MARKER,
        "hepta_telegram_composer_voice_send_live_wiring_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"voice_message_send"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"voice_send_live_attachment_wiring"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "voice send live attachment wiring"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("MatrixRequest::SendAttachment")
            && capability
                .base_module
                .contains("Timeline::send_attachment().use_send_queue()")
            && capability.notes.contains("partial-live")
            && capability
                .notes
                .contains("desktop picker stages a Voice attachment review")
            && capability.notes.contains("MatrixRequest::SendAttachment")
            && capability
                .notes
                .contains("Timeline::send_attachment().use_send_queue()")
            && capability.notes.contains("PositiveConfirmationModal")
            && capability.notes.contains("failed-handoff Retry")
            && capability.notes.contains("Microphone permission")
            && capability.notes.contains("captured upload")
            && capability.notes.contains("mobile picker")
            && capability.notes.contains("accepted SDK queue controls")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|runway| {
        runway.gap_id == "voice_message_send"
            && runway
                .current_path
                .contains("live desktop audio SendAttachment/use_send_queue wiring")
            && runway.remaining_gap.contains("real recorder")
            && runway
                .next_ui_safe_step
                .contains("coordinate backend recorder/upload contracts")
    }));
}

#[test]
fn hepta_telegram_base_composer_voice_selected_audio_metadata_is_local_preview() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_VOICE_SELECTED_AUDIO_METADATA_MARKER,
        "hepta_telegram_composer_voice_selected_audio_metadata_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"voice_message_send"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"voice_selected_audio_metadata_preview")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_SELECTED_AUDIO_METADATA_EVIDENCE
            .contains("filename, MIME type, extension, local file size")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_SELECTED_AUDIO_METADATA_EVIDENCE
            .contains("duration status")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_SELECTED_AUDIO_METADATA_EVIDENCE
            .contains("codec/container status")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_SELECTED_AUDIO_METADATA_EVIDENCE
            .contains("bounded local WAV PCM waveform peaks")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_SELECTED_AUDIO_METADATA_EVIDENCE
            .contains("Simple WAV header duration")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_SELECTED_AUDIO_METADATA_EVIDENCE
            .contains("media decode")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_SELECTED_AUDIO_METADATA_LABEL
            .contains("duration, codec")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "voice selected audio metadata preview"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability.base_module.contains("attachment_review_preview")
            && capability
                .notes
                .contains("already selected local file path")
            && capability.notes.contains("duration status")
            && capability.notes.contains("codec/container status")
            && capability.notes.contains("bounded WAV PCM waveform peaks")
            && capability.notes.contains("no microphone permission")
            && capability.notes.contains("media decode")
            && capability.notes.contains("hidden SDK send-queue work")
    }));
}

#[test]
fn hepta_telegram_base_composer_voice_selected_audio_waveform_codec_is_local_preview() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_VOICE_SELECTED_AUDIO_WAVEFORM_CODEC_MARKER,
        "hepta_telegram_composer_voice_selected_audio_waveform_codec_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"voice_message_send"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"voice_selected_audio_waveform_codec_preview")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_SELECTED_AUDIO_WAVEFORM_CODEC_EVIDENCE
            .contains("already selected desktop Voice attachment")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_SELECTED_AUDIO_WAVEFORM_CODEC_EVIDENCE
            .contains("RIFF/fmt/data metadata")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_SELECTED_AUDIO_WAVEFORM_CODEC_EVIDENCE
            .contains("coarse PCM peak buckets")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_SELECTED_AUDIO_WAVEFORM_CODEC_EVIDENCE
            .contains("submits SendAttachment before review Send")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_SELECTED_AUDIO_WAVEFORM_CODEC_LABEL
            .contains("capped local WAV header + PCM peaks")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "voice selected audio waveform codec preview"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("voice_audio_waveform_codec_label")
            && capability
                .notes
                .contains("capped local WAV RIFF/fmt/data parsing")
            && capability.notes.contains("coarse PCM peak buckets")
            && capability.notes.contains("Non-WAV")
            && capability.notes.contains("no microphone permission")
            && capability.notes.contains("captured file write")
            && capability.notes.contains("compressed media decode")
            && capability
                .notes
                .contains("extra SendAttachment before review Send")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|runway| {
        runway.gap_id == "voice_message_send"
            && runway
                .current_path
                .contains("selected-audio duration/codec/bounded WAV PCM waveform analysis")
            && runway.remaining_gap.contains("recorder waveform capture")
            && runway
                .next_ui_safe_step
                .contains("coordinate backend recorder/upload contracts")
    }));
}

#[test]
fn hepta_telegram_base_composer_voice_lifecycle_metadata_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_VOICE_LIFECYCLE_METADATA_MARKER,
        "hepta_telegram_composer_voice_lifecycle_metadata_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"voice_message_send"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"voice_lifecycle_metadata_preview"));
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_LIFECYCLE_METADATA_EVIDENCE
            .contains("local voice panel visibility")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_LIFECYCLE_METADATA_EVIDENCE
            .contains("confirmation and picker state")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_LIFECYCLE_METADATA_EVIDENCE
            .contains("pending selected desktop audio filename")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_LIFECYCLE_METADATA_EVIDENCE
            .contains("existing attachment review row")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_LIFECYCLE_METADATA_EVIDENCE
            .contains("duration capture from a recorder")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_LIFECYCLE_METADATA_LABEL
            .contains("confirmation/picker state")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "voice lifecycle metadata"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("voice_message_lifecycle_metadata_label")
            && capability.notes.contains("local voice panel visibility")
            && capability.notes.contains("Record/Lock/Cancel/Close status")
            && capability.notes.contains("confirmation and picker state")
            && capability.notes.contains("existing attachment review row")
            && capability.notes.contains("microphone permission")
            && capability.notes.contains("audio session activation")
            && capability.notes.contains("opus/aac encoding")
            && capability.notes.contains("hidden SDK send-queue work")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_composer_voice_confirmation_cancel_metadata_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_VOICE_CONFIRMATION_CANCEL_METADATA_MARKER,
        "hepta_telegram_composer_voice_confirmation_cancel_metadata_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"voice_message_send"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"voice_confirmation_cancel_metadata_preview")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_CONFIRMATION_CANCEL_METADATA_EVIDENCE
            .contains("PositiveConfirmationModal cancel")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_CONFIRMATION_CANCEL_METADATA_EVIDENCE
            .contains("pending attachment already exists")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_CONFIRMATION_CANCEL_METADATA_EVIDENCE
            .contains("SendAttachment")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_CONFIRMATION_CANCEL_METADATA_LABEL
            .contains("local voice/picker state")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "voice confirmation cancel metadata"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability.base_module.contains("AttachmentHandoffCanceled")
            && capability.notes.contains("repaints the voice panel")
            && capability
                .notes
                .contains("pending attachment review is loaded it is preserved")
            && capability
                .notes
                .contains("waiting picker preview is hidden")
            && capability.notes.contains("microphone permission")
            && capability.notes.contains("SendAttachment")
            && capability.notes.contains("SDK queue cancel")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_composer_voice_permission_recording_boundary_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_VOICE_PERMISSION_RECORDING_LOCAL_BOUNDARY_MARKER,
        "hepta_telegram_composer_voice_permission_recording_local_boundary_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_VOICE_RECORDING_BOUNDARY_MARKER,
        "hepta_telegram_composer_voice_recording_boundary_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"voice_message_send"));
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_PERMISSION_RECORDING_LOCAL_BOUNDARY_EVIDENCE
            .contains("voice_message_send remains a base gap")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_PERMISSION_RECORDING_LOCAL_BOUNDARY_EVIDENCE
            .contains("privacy entitlement")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_PERMISSION_RECORDING_LOCAL_BOUNDARY_EVIDENCE
            .contains("audio session activation")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_PERMISSION_RECORDING_LOCAL_BOUNDARY_EVIDENCE
            .contains("local audio file creation")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_PERMISSION_RECORDING_LOCAL_BOUNDARY_EVIDENCE
            .contains("waveform sampling")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_PERMISSION_RECORDING_LOCAL_BOUNDARY_EVIDENCE
            .contains("opus/aac encoding")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_PERMISSION_RECORDING_LOCAL_BOUNDARY_LABEL
            .contains("audio session")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"voice_permission_recording_local_boundary_evidence")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "voice permission recording local boundary evidence"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("telegram_voice_message_panel")
            && capability
                .notes
                .contains("voice_message_send remains a base gap")
            && capability.notes.contains("microphone permission")
            && capability.notes.contains("privacy entitlement")
            && capability.notes.contains("audio session activation")
            && capability.notes.contains("platform recorder")
            && capability.notes.contains("local audio file creation")
            && capability.notes.contains("temporary file write")
            && capability.notes.contains("waveform sampling")
            && capability.notes.contains("duration capture")
            && capability.notes.contains("opus/aac encoding")
            && capability
                .notes
                .contains("confirmed desktop audio-file selection")
            && capability.notes.contains("SDK send-queue work")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_composer_voice_recorder_waveform_codec_boundary_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_VOICE_RECORDER_WAVEFORM_CODEC_BOUNDARY_MARKER,
        "hepta_telegram_composer_voice_recorder_waveform_codec_boundary_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"voice_message_send"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"voice_recorder_waveform_codec_boundary")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_RECORDER_WAVEFORM_CODEC_BOUNDARY_EVIDENCE
            .contains("voice_message_send remains a base gap")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_RECORDER_WAVEFORM_CODEC_BOUNDARY_EVIDENCE
            .contains("waveform capture/render")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_RECORDER_WAVEFORM_CODEC_BOUNDARY_EVIDENCE
            .contains("encoder/codec selection")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_RECORDER_WAVEFORM_CODEC_BOUNDARY_EVIDENCE
            .contains("opus/ogg/amr conversion")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_RECORDER_WAVEFORM_CODEC_BOUNDARY_EVIDENCE
            .contains("transcription")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_RECORDER_WAVEFORM_CODEC_BOUNDARY_EVIDENCE
            .contains("upload progress")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_RECORDER_WAVEFORM_CODEC_BOUNDARY_EVIDENCE
            .contains("background recording")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_RECORDER_WAVEFORM_CODEC_BOUNDARY_EVIDENCE
            .contains("attachment/edit voice payload")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_RECORDER_WAVEFORM_CODEC_BOUNDARY_EVIDENCE
            .contains("hidden SDK queue controls")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_RECORDER_WAVEFORM_CODEC_BOUNDARY_EVIDENCE
            .contains("MatrixRequest::SendAttachment")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_RECORDER_WAVEFORM_CODEC_BOUNDARY_LABEL
            .contains("codec conversion")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "voice recorder waveform codec boundary"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("voice_message_recorder_waveform_codec_boundary_label")
            && capability
                .notes
                .contains("voice_message_send remains a base gap")
            && capability.notes.contains("microphone permission prompt")
            && capability.notes.contains("audio session activation")
            && capability.notes.contains("waveform capture/render")
            && capability.notes.contains("encoder/codec selection")
            && capability.notes.contains("opus/ogg/amr conversion")
            && capability.notes.contains("silence trimming")
            && capability.notes.contains("transcription")
            && capability.notes.contains("upload progress")
            && capability.notes.contains("background recording")
            && capability.notes.contains("hidden SDK queue controls")
            && capability.notes.contains("MatrixRequest::SendAttachment")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_composer_voice_recorder_status_controls_are_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_VOICE_RECORDER_STATUS_CONTROLS_MARKER,
        "hepta_telegram_composer_voice_recorder_status_controls_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"voice_message_send"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"voice_recorder_status_controls_row"));
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_RECORDER_STATUS_CONTROLS_EVIDENCE
            .contains("Timer, Waveform, Transcript, Progress, and Codec")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_RECORDER_STATUS_CONTROLS_EVIDENCE
            .contains("visible local voice recorder status controls")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_RECORDER_STATUS_CONTROLS_EVIDENCE
            .contains("microphone permission")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_RECORDER_STATUS_CONTROLS_EVIDENCE
            .contains("audio session activation")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_RECORDER_STATUS_CONTROLS_EVIDENCE
            .contains("platform recorder")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_RECORDER_STATUS_CONTROLS_EVIDENCE
            .contains("waveform sampling")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_RECORDER_STATUS_CONTROLS_EVIDENCE
            .contains("transcript service")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_RECORDER_STATUS_CONTROLS_EVIDENCE
            .contains("upload progress subscription")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_RECORDER_STATUS_CONTROLS_EVIDENCE
            .contains("SendAttachment")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_RECORDER_STATUS_CONTROLS_LABEL
            .contains("Timer, Waveform, Transcript, Progress, and Codec")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "voice recorder status controls row"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("voice_recorder_status_controls")
            && capability
                .notes
                .contains("Timer, Waveform, Transcript, Progress, and Codec")
            && capability
                .notes
                .contains("visible local voice recorder status controls")
            && capability.notes.contains("panel visibility")
            && capability
                .notes
                .contains("pending desktop audio review state")
            && capability.notes.contains("microphone permission")
            && capability.notes.contains("audio session activation")
            && capability.notes.contains("platform recorder")
            && capability.notes.contains("captured local audio file")
            && capability.notes.contains("waveform sampling")
            && capability.notes.contains("transcription service")
            && capability.notes.contains("codec conversion")
            && capability.notes.contains("upload progress subscription")
            && capability.notes.contains("SDK queue control")
            && capability.notes.contains("SendAttachment")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_composer_voice_capture_lifecycle_controls_are_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_VOICE_CAPTURE_LIFECYCLE_CONTROLS_MARKER,
        "hepta_telegram_composer_voice_capture_lifecycle_controls_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"voice_message_send"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"voice_capture_lifecycle_controls_row"));
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_CAPTURE_LIFECYCLE_CONTROLS_EVIDENCE.contains(
            "Permission, Capture, Encode, Review, Upload, Packet, Contract, and Taxonomy"
        )
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_CAPTURE_LIFECYCLE_CONTROLS_EVIDENCE
            .contains("local voice capture/request packet snapshot")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_CAPTURE_LIFECYCLE_CONTROLS_EVIDENCE
            .contains("Packet renders a recorder lifecycle drilldown packet")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_CAPTURE_LIFECYCLE_CONTROLS_EVIDENCE
            .contains("Taxonomy records permission/capture/encode/review/upload result slots")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_CAPTURE_LIFECYCLE_CONTROLS_EVIDENCE
            .contains("Permission requests no microphone permission")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_CAPTURE_LIFECYCLE_CONTROLS_EVIDENCE
            .contains("Capture starts no platform recorder")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_CAPTURE_LIFECYCLE_CONTROLS_EVIDENCE
            .contains("Upload submits no SendAttachment")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_CAPTURE_LIFECYCLE_CONTROLS_LABEL
            .contains("Taxonomy records permission/capture/upload result slots")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "voice capture lifecycle controls row"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("voice_capture_lifecycle_controls")
            && capability
                .base_module
                .contains("voice_message_capture_request_packet_snapshot_label")
            && capability.notes.contains(
                "Permission, Capture, Encode, Review, Upload, Packet, Contract, and Taxonomy",
            )
            && capability
                .notes
                .contains("visible local voice capture lifecycle controls")
            && capability
                .notes
                .contains("local voice capture/request packet snapshot")
            && capability
                .notes
                .contains("Packet renders a recorder lifecycle drilldown packet")
            && capability
                .notes
                .contains("Taxonomy records permission/capture/upload result slots")
            && capability.notes.contains("panel visibility")
            && capability
                .notes
                .contains("pending desktop audio review state")
            && capability
                .notes
                .contains("Permission requests no microphone permission")
            && capability
                .notes
                .contains("Capture starts no platform recorder")
            && capability
                .notes
                .contains("Encode performs no codec conversion")
            && capability
                .notes
                .contains("Review creates no captured voice payload")
            && capability
                .notes
                .contains("Upload submits no SendAttachment")
            && capability.notes.contains("SendMessage fallback")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
            && capability.notes.contains(
                "confirmed positive path remains desktop audio-file review SendAttachment",
            )
    }));
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|runway| {
        runway.gap_id == "voice_message_send"
            && runway.current_path.contains("capture lifecycle")
            && runway
                .current_path
                .contains("local voice capture/request packet snapshot")
            && runway.remaining_gap.contains("microphone permission")
    }));
}

#[test]
fn hepta_telegram_base_composer_voice_recorder_lifecycle_drilldown_packet_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_VOICE_RECORDER_LIFECYCLE_DRILLDOWN_PACKET_MARKER,
        "hepta_telegram_composer_voice_recorder_lifecycle_drilldown_packet_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"voice_message_send"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"voice_recorder_lifecycle_drilldown_packet_action")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_RECORDER_LIFECYCLE_DRILLDOWN_PACKET_EVIDENCE
            .contains("visible Packet control")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_RECORDER_LIFECYCLE_DRILLDOWN_PACKET_EVIDENCE
            .contains("microphone permission, privacy entitlement, audio session")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_RECORDER_LIFECYCLE_DRILLDOWN_PACKET_EVIDENCE
            .contains("recorder start/lock/cancel")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_RECORDER_LIFECYCLE_DRILLDOWN_PACKET_EVIDENCE
            .contains("waveform sampling/rendering")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_RECORDER_LIFECYCLE_DRILLDOWN_PACKET_EVIDENCE
            .contains("codec/encoding/transcription")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_RECORDER_LIFECYCLE_DRILLDOWN_PACKET_EVIDENCE
            .contains("upload queue")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_RECORDER_LIFECYCLE_DRILLDOWN_PACKET_EVIDENCE
            .contains("confirmed desktop audio review SendAttachment acceptance criteria")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_RECORDER_LIFECYCLE_DRILLDOWN_PACKET_EVIDENCE
            .contains("extra MatrixRequest::SendAttachment")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_RECORDER_LIFECYCLE_DRILLDOWN_PACKET_EVIDENCE
            .contains("gateway/runtime/auth")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_RECORDER_LIFECYCLE_DRILLDOWN_PACKET_LABEL
            .contains("acceptance criteria stay local")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "voice recorder lifecycle drilldown packet"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("packet_voice_capture_button")
            && capability
                .base_module
                .contains("voice_message_recorder_lifecycle_drilldown_packet_label")
            && capability
                .notes
                .contains("voice_message_send remains a base gap")
            && capability.notes.contains("microphone permission")
            && capability.notes.contains("privacy entitlement")
            && capability.notes.contains("audio session")
            && capability.notes.contains("recorder start/lock/cancel")
            && capability
                .notes
                .contains("temporary capture file lifecycle")
            && capability.notes.contains("waveform sampling/rendering")
            && capability.notes.contains("timer/duration capture")
            && capability.notes.contains("codec/encoding/transcription")
            && capability.notes.contains("review playback/drop cleanup")
            && capability.notes.contains("mobile picker/share sheet")
            && capability.notes.contains("upload queue")
            && capability
                .notes
                .contains("confirmed desktop audio review SendAttachment acceptance criteria")
            && capability
                .notes
                .contains("extra MatrixRequest::SendAttachment")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|runway| {
        runway.gap_id == "voice_message_send"
            && runway
                .current_path
                .contains("recorder lifecycle drilldown packet")
            && runway
                .next_ui_safe_step
                .contains("coordinate backend recorder/upload contracts")
            && runway.next_ui_safe_step.contains("captured upload")
    }));
}

#[test]
fn hepta_telegram_base_composer_voice_recorder_typed_contract_packet_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_VOICE_RECORDER_TYPED_CONTRACT_PACKET_MARKER,
        "hepta_telegram_composer_voice_recorder_typed_contract_packet_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"voice_message_send"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"voice_recorder_typed_contract_packet_action")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_RECORDER_TYPED_CONTRACT_PACKET_EVIDENCE
            .contains("visible Contract control")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_RECORDER_TYPED_CONTRACT_PACKET_EVIDENCE
            .contains("microphone permission and privacy entitlement request/result/error slots")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_RECORDER_TYPED_CONTRACT_PACKET_EVIDENCE
            .contains("audio session and recorder session lifecycle")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_RECORDER_TYPED_CONTRACT_PACKET_EVIDENCE
            .contains("capture file identity and cleanup")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_RECORDER_TYPED_CONTRACT_PACKET_EVIDENCE
            .contains("waveform/timer sampling")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_RECORDER_TYPED_CONTRACT_PACKET_EVIDENCE
            .contains("codec/encoding/transcription results")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_RECORDER_TYPED_CONTRACT_PACKET_EVIDENCE
            .contains("upload queue progress/result/error/retry/source slots")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_RECORDER_TYPED_CONTRACT_PACKET_EVIDENCE
            .contains("confirmed desktop audio review SendAttachment result mapping")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_RECORDER_TYPED_CONTRACT_PACKET_EVIDENCE
            .contains("adapter promotion blockers")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_RECORDER_TYPED_CONTRACT_PACKET_EVIDENCE
            .contains("extra MatrixRequest::SendAttachment")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_RECORDER_TYPED_CONTRACT_PACKET_LABEL
            .contains("contracts stay local")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "voice recorder typed contract packet"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("contract_voice_capture_button")
            && capability
                .base_module
                .contains("voice_message_recorder_typed_contract_packet_label")
            && capability
                .notes
                .contains("voice_message_send remains a base gap")
            && capability.notes.contains("microphone permission")
            && capability.notes.contains("privacy entitlement")
            && capability.notes.contains("audio session")
            && capability.notes.contains("recorder session")
            && capability.notes.contains("capture file")
            && capability.notes.contains("waveform/timer")
            && capability.notes.contains("codec/encoding/transcription")
            && capability.notes.contains("mobile picker/share sheet")
            && capability.notes.contains("upload queue")
            && capability.notes.contains("SendAttachment result")
            && capability.notes.contains("stale capture")
            && capability.notes.contains("idempotency")
            && capability.notes.contains("adapter promotion blocker")
            && capability
                .notes
                .contains("extra MatrixRequest::SendAttachment")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|runway| {
        runway.gap_id == "voice_message_send"
            && runway
                .current_path
                .contains("typed recorder/upload contract packet")
            && runway
                .next_ui_safe_step
                .contains("coordinate backend recorder/upload contracts")
            && runway.next_ui_safe_step.contains("captured upload")
    }));
}

#[test]
fn hepta_telegram_base_composer_voice_recorder_result_taxonomy_packet_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_VOICE_RECORDER_RESULT_TAXONOMY_PACKET_MARKER,
        "hepta_telegram_composer_voice_recorder_result_taxonomy_packet_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"voice_message_send"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"voice_recorder_result_taxonomy_packet_action")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_RECORDER_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("visible Taxonomy control")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_RECORDER_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("confirmed desktop audio review MatrixRequest::SendAttachment")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_RECORDER_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("Timeline::send_attachment().use_send_queue()")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_RECORDER_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("review Play local system-opener handoff")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_RECORDER_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("microphone permission operation id")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_RECORDER_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("audio session id")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_RECORDER_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("codec/transcription result")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_RECORDER_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("captured upload queue item")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_RECORDER_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("audit redaction")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_RECORDER_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("extra MatrixRequest::SendAttachment")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_RECORDER_RESULT_TAXONOMY_PACKET_LABEL
            .contains("results stay local")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "voice recorder result taxonomy packet"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("taxonomy_voice_capture_button")
            && capability
                .base_module
                .contains("voice_message_recorder_result_taxonomy_packet_label")
            && capability
                .notes
                .contains("voice_message_send remains a base gap")
            && capability
                .notes
                .contains("confirmed desktop audio review MatrixRequest::SendAttachment")
            && capability
                .notes
                .contains("Timeline::send_attachment().use_send_queue()")
            && capability.notes.contains("failed-handoff Retry")
            && capability.notes.contains("bounded WAV metadata")
            && capability.notes.contains("local system-opener handoff")
            && capability.notes.contains("pending-audio local cleanup")
            && capability
                .notes
                .contains("Microphone permission operation id")
            && capability.notes.contains("privacy entitlement result")
            && capability.notes.contains("audio session id")
            && capability.notes.contains("recorder session id")
            && capability.notes.contains("capture file identity")
            && capability.notes.contains("waveform/timer result")
            && capability.notes.contains("codec/transcription result")
            && capability.notes.contains("review player result")
            && capability.notes.contains("mobile picker/share result")
            && capability.notes.contains("captured upload queue item")
            && capability.notes.contains("delivery result")
            && capability.notes.contains("stale capture result")
            && capability.notes.contains("retry/cancel result")
            && capability.notes.contains("audit redaction")
            && capability
                .notes
                .contains("extra MatrixRequest::SendAttachment")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|runway| {
        runway.gap_id == "voice_message_send"
            && runway
                .current_path
                .contains("permission/capture/upload result taxonomy packet")
            && runway
                .next_ui_safe_step
                .contains("coordinate backend recorder/upload contracts")
            && runway.next_ui_safe_step.contains("captured upload")
    }));
}

#[test]
fn hepta_telegram_base_composer_voice_mobile_picker_controls_are_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_VOICE_MOBILE_PICKER_CONTROLS_MARKER,
        "hepta_telegram_composer_voice_mobile_picker_controls_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"voice_message_send"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"voice_mobile_picker_controls_row"));
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_MOBILE_PICKER_CONTROLS_EVIDENCE
            .contains("Mic, Files, Library, Retake, and Share")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_MOBILE_PICKER_CONTROLS_EVIDENCE
            .contains("Files opens no mobile document picker")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_MOBILE_PICKER_CONTROLS_EVIDENCE
            .contains("Library opens no photo/audio library picker")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_MOBILE_PICKER_CONTROLS_EVIDENCE
            .contains("Share opens no system share sheet")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_MOBILE_PICKER_CONTROLS_LABEL
            .contains("no mobile permission")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "voice mobile picker controls row"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("voice_mobile_picker_controls")
            && capability
                .notes
                .contains("Mic, Files, Library, Retake, and Share")
            && capability
                .notes
                .contains("visible local voice mobile picker controls")
            && capability
                .notes
                .contains("pending desktop audio review state")
            && capability.notes.contains("no mobile microphone permission")
            && capability
                .notes
                .contains("Files opens no mobile document picker")
            && capability
                .notes
                .contains("Library opens no photo/audio library picker")
            && capability.notes.contains("Retake deletes no captured clip")
            && capability
                .notes
                .contains("Share opens no system share sheet")
            && capability.notes.contains("SendAttachment")
            && capability.notes.contains("SDK send-queue work")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|runway| {
        runway.gap_id == "voice_message_send"
            && runway
                .current_path
                .contains("mobile picker boundary controls")
            && runway.remaining_gap.contains("mobile picker")
    }));
}

#[test]
fn hepta_telegram_base_composer_voice_send_preflight_detail_controls_are_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_VOICE_SEND_PREFLIGHT_DETAIL_CONTROLS_MARKER,
        "hepta_telegram_composer_voice_send_preflight_detail_controls_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"voice_message_send"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"voice_send_preflight_detail_controls_row")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_SEND_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
            .contains("Request, Result, Error, Retry, and Source")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_SEND_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
            .contains("local voice capture/request packet snapshot")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_SEND_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
            .contains("pending desktop audio review")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_SEND_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
            .contains("extra MatrixRequest::SendAttachment")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_SEND_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
            .contains("unconfirmed retry")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_SEND_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
            .contains("gateway/runtime/auth")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_SEND_PREFLIGHT_DETAIL_CONTROLS_LABEL
            .contains("confirmed desktop audio review Send")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "voice send preflight detail controls row"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("voice_send_preflight_detail_controls")
            && capability
                .base_module
                .contains("voice_message_capture_request_packet_snapshot_label")
            && capability
                .notes
                .contains("Request, Result, Error, Retry, and Source")
            && capability
                .notes
                .contains("Request renders a local voice capture/request packet snapshot")
            && capability.notes.contains("pending desktop audio review")
            && capability.notes.contains("retry-cache readiness")
            && capability.notes.contains("microphone permission")
            && capability.notes.contains("audio session activation")
            && capability.notes.contains("platform recorder")
            && capability.notes.contains("captured local audio file")
            && capability.notes.contains("waveform sampling")
            && capability.notes.contains("transcription service")
            && capability.notes.contains("codec conversion")
            && capability.notes.contains("upload progress subscription")
            && capability.notes.contains("SDK queue control")
            && capability
                .notes
                .contains("extra MatrixRequest::SendAttachment")
            && capability.notes.contains("unconfirmed retry")
            && capability.notes.contains("SendMessage fallback")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_composer_voice_review_playback_controls_are_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_VOICE_REVIEW_PLAYBACK_CONTROLS_MARKER,
        "hepta_telegram_composer_voice_review_playback_controls_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_VOICE_REVIEW_PLAYBACK_OPENER_MARKER,
        "hepta_telegram_composer_voice_review_playback_opener_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"voice_message_send"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"voice_review_playback_controls_row"));
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_REVIEW_PLAYBACK_CONTROLS_EVIDENCE
            .contains("Play, Pause, Scrub, Speed, and Drop")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_REVIEW_PLAYBACK_CONTROLS_EVIDENCE
            .contains("visible voice review playback controls")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_REVIEW_PLAYBACK_CONTROLS_EVIDENCE
            .contains("system opener")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_REVIEW_PLAYBACK_CONTROLS_EVIDENCE
            .contains("readable regular local file")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_REVIEW_PLAYBACK_CONTROLS_EVIDENCE
            .contains("start no inline audio player")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_REVIEW_PLAYBACK_CONTROLS_EVIDENCE
            .contains("media decode")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_REVIEW_PLAYBACK_CONTROLS_EVIDENCE
            .contains("playback position subscription")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_REVIEW_PLAYBACK_CONTROLS_EVIDENCE
            .contains("local file deletion")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_REVIEW_PLAYBACK_CONTROLS_EVIDENCE
            .contains("Drop is a real local cleanup handoff")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_REVIEW_PLAYBACK_CONTROLS_LABEL
            .contains("Play opens pending desktop audio")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "voice review playback controls row"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("voice_review_playback_controls")
            && capability
                .notes
                .contains("Play, Pause, Scrub, Speed, and Drop")
            && capability.notes.contains("pending desktop audio review")
            && capability.notes.contains("local OS handoff")
            && capability.notes.contains("system opener")
            && capability.notes.contains("readable regular local file")
            && capability.notes.contains("inline audio player")
            && capability.notes.contains("media decode")
            && capability.notes.contains("playback position subscription")
            && capability.notes.contains("speed transform")
            && capability.notes.contains("scrubber timeline")
            && capability.notes.contains("local file deletion")
            && capability
                .notes
                .contains("Drop is a real pending-audio cleanup handoff")
            && capability.notes.contains("SendAttachment")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|runway| {
        runway.gap_id == "voice_message_send"
            && runway.current_path.contains("review playback/drop cleanup")
            && runway
                .current_path
                .contains("review Play local system-opener handoff")
    }));
}

#[test]
fn hepta_telegram_base_composer_voice_review_drop_pending_audio_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_VOICE_REVIEW_DROP_PENDING_AUDIO_MARKER,
        "hepta_telegram_composer_voice_review_drop_pending_audio_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"voice_message_send"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"voice_review_drop_pending_audio_action")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_REVIEW_DROP_PENDING_AUDIO_EVIDENCE
            .contains("Option::take()")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_REVIEW_DROP_PENDING_AUDIO_EVIDENCE
            .contains("does not discard Photo/File pending attachments")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_REVIEW_DROP_PENDING_AUDIO_EVIDENCE
            .contains("delete local files")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_REVIEW_DROP_PENDING_AUDIO_EVIDENCE
            .contains("SDK send-queue work")
    );
    assert!(
        crate::room::room_input_bar::VOICE_MESSAGE_REVIEW_DROP_PENDING_AUDIO_LABEL
            .contains("no file deletion")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "voice review drop pending audio action"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("drop_telegram_voice_review_audio")
            && capability.notes.contains("Option::take()")
            && capability
                .notes
                .contains("voice failed-handoff retry metadata")
            && capability
                .notes
                .contains("Photo/File pending attachments untouched")
            && capability.notes.contains("deletes no local file")
            && capability.notes.contains("MatrixRequest::SendAttachment")
            && capability.notes.contains("caption-only SendMessage")
            && capability.notes.contains("SDK send-queue work")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_composer_local_surface_close_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_LOCAL_SURFACE_CLOSE_MARKER,
        "hepta_telegram_composer_local_surface_close_ready"
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"composer_local_surface_close_evidence")
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"file_upload_send"));
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"voice_message_send"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "composer"
            && capability.notes.contains("local Close evidence")
            && capability.notes.contains("composer preview panels")
    }));
}

#[test]
fn hepta_telegram_base_composer_mention_surface_is_local_only() {
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
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"mention_picker_send"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"mention_helper_surface"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"mention_option_staging_local_evidence")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"mention_cached_selection_preview"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"mention_loaded_identity_preview"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "mention picker/send"
            && capability.notes.contains("compact @mention payload helper")
            && capability.notes.contains("Matrix Mentions")
            && capability
                .notes
                .contains("existing RoomInputBar SendMessage")
            && capability.notes.contains("minimal local suggestion row")
            && capability
                .notes
                .contains("Full mention_picker_send remains a base gap")
            && capability
                .notes
                .contains("up to three cached member tokens")
            && capability
                .notes
                .contains("suggestion count/selected token/no-match state")
            && capability.notes.contains("Tab/Enter")
            && capability.notes.contains("ArrowUp/ArrowDown")
            && capability.notes.contains("trailing space releases Enter")
            && capability.notes.contains("remote member lookup")
            && capability.notes.contains("completed-token pill tray")
            && capability.notes.contains("removable local pills")
    }));
}

#[test]
fn hepta_telegram_base_composer_mention_option_staging_uses_local_cache_payload() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_MENTION_OPTION_STAGING_LOCAL_MARKER,
        "hepta_telegram_composer_mention_option_staging_local_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"mention_picker_send"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"mention_option_staging_local_evidence")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"mention_cached_selection_preview"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "mention option payload local-cache evidence"
            && capability
                .notes
                .contains("@room and cached @user mentions attach Matrix Mentions")
            && capability.notes.contains("up to three cached @user tokens")
            && capability.notes.contains("cached suggestion count")
            && capability.notes.contains("selected token")
            && capability.notes.contains("no-match state")
            && capability.notes.contains("ArrowUp/ArrowDown")
            && capability.notes.contains("Tab/Enter")
            && capability.notes.contains("local suggestion row inserts")
            && capability.notes.contains("RoomScreen room_members cache")
            && capability.notes.contains("without remote member lookup")
            && capability.notes.contains("full popup search")
            && capability.notes.contains("rich popup highlight styling")
            && capability.notes.contains("removable local pills")
            && capability.notes.contains("existing composer path")
    }));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "mention cached selection preview"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability.notes.contains("cached suggestion count")
            && capability.notes.contains("currently selected token")
            && capability.notes.contains("no-match state")
            && capability.notes.contains("loaded room_members")
            && capability.notes.contains("no remote member lookup")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_composer_mention_loaded_identity_preview_is_local() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_MENTION_LOADED_IDENTITY_MARKER,
        "hepta_telegram_composer_mention_loaded_identity_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"mention_picker_send"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"mention_loaded_identity_preview"));
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_LOADED_IDENTITY_EVIDENCE
            .contains("display name availability")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_LOADED_IDENTITY_EVIDENCE
            .contains("Matrix user id")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_LOADED_IDENTITY_EVIDENCE
            .contains("localpart")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_LOADED_IDENTITY_EVIDENCE
            .contains("avatar MXC presence")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_LOADED_IDENTITY_EVIDENCE
            .contains("@room row")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_LOADED_IDENTITY_EVIDENCE
            .contains("remote members")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_LOADED_IDENTITY_EVIDENCE
            .contains("fetch avatars")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_LOADED_IDENTITY_EVIDENCE
            .contains("account/profile")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_LOADED_IDENTITY_EVIDENCE
            .contains("gateway/runtime/auth")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_LOADED_IDENTITY_LABEL
            .contains("display name, user id, localpart")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "mention loaded identity preview"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability.notes.contains("display name availability")
            && capability.notes.contains("Matrix user id")
            && capability.notes.contains("localpart")
            && capability.notes.contains("avatar MXC presence")
            && capability.notes.contains("@room selection")
            && capability.notes.contains("remote member lookup")
            && capability.notes.contains("profile fetch")
            && capability.notes.contains("avatar fetch")
            && capability.notes.contains("gateway/runtime/auth")
    }));
}

#[test]
fn hepta_telegram_base_composer_mention_local_candidate_rows_are_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_MENTION_LOCAL_CANDIDATE_ROWS_MARKER,
        "hepta_telegram_composer_mention_local_candidate_rows_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"mention_picker_send"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"mention_local_candidate_rows_preview"));
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_LOCAL_CANDIDATE_ROWS_EVIDENCE
            .contains("local candidate rows")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_LOCAL_CANDIDATE_ROWS_EVIDENCE
            .contains("selected state")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_LOCAL_CANDIDATE_ROWS_EVIDENCE
            .contains("Matrix user id")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_LOCAL_CANDIDATE_ROWS_EVIDENCE
            .contains("avatar MXC status")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_LOCAL_CANDIDATE_ROWS_EVIDENCE
            .contains("server-side directory search")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_LOCAL_CANDIDATE_ROWS_EVIDENCE
            .contains("extra SendMessage")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_LOCAL_CANDIDATE_ROWS_LABEL
            .contains("rank, selection, token")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "mention local candidate rows"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("mention_candidate_rows_preview")
            && capability.notes.contains("active @query")
            && capability.notes.contains("@room power-level state")
            && capability
                .notes
                .contains("up to three cached RoomMember matches")
            && capability.notes.contains("rank")
            && capability.notes.contains("selected state")
            && capability.notes.contains("display name availability")
            && capability.notes.contains("Matrix user id")
            && capability.notes.contains("avatar MXC status")
            && capability.notes.contains("remote member lookup")
            && capability
                .notes
                .contains("server-side member directory search")
            && capability.notes.contains("profile/avatar fetch")
            && capability.notes.contains("duplicate-name disambiguation")
            && capability.notes.contains("extra SendMessage")
            && capability.notes.contains("retry automation")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|runway| {
        runway.gap_id == "mention_picker_send"
            && runway.current_path.contains("local candidate rows")
            && runway
                .next_ui_safe_step
                .contains("coordinate backend mention contracts")
    }));
}

#[test]
fn hepta_telegram_base_composer_mention_local_duplicate_hints_are_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_MENTION_LOCAL_DUPLICATE_HINTS_MARKER,
        "hepta_telegram_composer_mention_local_duplicate_hints_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"mention_picker_send"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"mention_local_duplicate_hints_preview")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_LOCAL_DUPLICATE_HINTS_EVIDENCE
            .contains("local duplicate-name hints")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_LOCAL_DUPLICATE_HINTS_EVIDENCE
            .contains("display-name collisions")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_LOCAL_DUPLICATE_HINTS_EVIDENCE
            .contains("selected display collision count")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_LOCAL_DUPLICATE_HINTS_EVIDENCE
            .contains("localpart and Matrix user id")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_LOCAL_DUPLICATE_HINTS_EVIDENCE
            .contains("rich duplicate-name disambiguation UI")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_LOCAL_DUPLICATE_HINTS_EVIDENCE
            .contains("extra SendMessage")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_LOCAL_DUPLICATE_HINTS_LABEL
            .contains("Local duplicate hints")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "mention local duplicate hints"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("mention_duplicate_hints_preview")
            && capability.notes.contains("active @query")
            && capability.notes.contains("already loaded room_members")
            && capability
                .notes
                .contains("duplicate display-name group count")
            && capability
                .notes
                .contains("selected display collision count")
            && capability.notes.contains("localpart/Matrix user-id clues")
            && capability
                .notes
                .contains("server-side member directory search")
            && capability.notes.contains("profile/avatar fetch")
            && capability.notes.contains("pill editor")
            && capability.notes.contains("extra SendMessage")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|runway| {
        runway.gap_id == "mention_picker_send"
            && runway.current_path.contains("local duplicate-name hints")
            && runway
                .next_ui_safe_step
                .contains("coordinate backend mention contracts")
    }));
}

#[test]
fn hepta_telegram_base_composer_mention_lifecycle_metadata_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_MENTION_LIFECYCLE_METADATA_MARKER,
        "hepta_telegram_composer_mention_lifecycle_metadata_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"mention_picker_send"));
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_LIFECYCLE_METADATA_EVIDENCE
            .contains("active @query")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_LIFECYCLE_METADATA_EVIDENCE
            .contains("cached suggestion count")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_LIFECYCLE_METADATA_EVIDENCE
            .contains("selected token")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_LIFECYCLE_METADATA_EVIDENCE
            .contains("@room power-level allowance")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_LIFECYCLE_METADATA_EVIDENCE
            .contains("trailing-space send release")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_LIFECYCLE_METADATA_EVIDENCE
            .contains("completed mentions no longer intercept Enter")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_LIFECYCLE_METADATA_LABEL
            .contains("keyboard/click insertion")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"mention_lifecycle_metadata_preview"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "mention lifecycle metadata"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("mention_picker_lifecycle_metadata_label")
            && capability.notes.contains("active @query")
            && capability.notes.contains("cached suggestion count")
            && capability.notes.contains("selected token")
            && capability.notes.contains("@room power-level allowance")
            && capability.notes.contains("keyboard selection")
            && capability.notes.contains("Tab/Enter insertion")
            && capability.notes.contains("click insertion")
            && capability.notes.contains("trailing-space send release")
            && capability.notes.contains("selected_suggestion_index")
            && capability.notes.contains("extra SendMessage")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_composer_mention_keyboard_selection_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_MENTION_KEYBOARD_SELECTION_MARKER,
        "hepta_telegram_composer_mention_keyboard_selection_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"mention_picker_send"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"mention_keyboard_selection_boundary"));
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_KEYBOARD_SELECTION_EVIDENCE
            .contains("ArrowUp/ArrowDown selection")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_KEYBOARD_SELECTION_EVIDENCE
            .contains("Tab/Enter insertion")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_KEYBOARD_SELECTION_EVIDENCE
            .contains("selected_suggestion_index")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_KEYBOARD_SELECTION_EVIDENCE
            .contains("trailing space")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_KEYBOARD_SELECTION_LABEL
            .contains("active-token insertion")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "mention keyboard selection boundary"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability.base_module.contains("active-token selection")
            && capability.notes.contains("ArrowUp/ArrowDown selection")
            && capability.notes.contains("Tab/Enter insertion")
            && capability.notes.contains("selected_suggestion_index")
            && capability
                .notes
                .contains("existing RoomInputBar SendMessage path")
            && capability.notes.contains("remote member lookup")
            && capability
                .notes
                .contains("server-side member directory search")
            && capability.notes.contains("profile/avatar fetch")
            && capability.notes.contains("attachment/edit mention payload")
            && capability.notes.contains("extra SendMessage")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|runway| {
        runway.gap_id == "mention_picker_send"
            && runway.current_path.contains("keyboard selection")
            && runway.remaining_gap.contains("rich popup search")
    }));
}

#[test]
fn hepta_telegram_base_composer_mention_rich_picker_boundary_is_metadata_only() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_MENTION_RICH_PICKER_BOUNDARY_MARKER,
        "hepta_telegram_composer_mention_rich_picker_boundary_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"mention_picker_send"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"mention_rich_picker_boundary_evidence")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_RICH_POPUP_BOUNDARY_EVIDENCE
            .contains("boundary metadata")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_RICH_POPUP_BOUNDARY_EVIDENCE
            .contains("floating popup search")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_RICH_POPUP_BOUNDARY_EVIDENCE
            .contains("rich highlighted result list")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_RICH_POPUP_BOUNDARY_EVIDENCE
            .contains("pill editor")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_RICH_POPUP_BOUNDARY_EVIDENCE
            .contains("remote member lookup")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_RICH_POPUP_BOUNDARY_EVIDENCE
            .contains("attachment/edit mention payload editor")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_RICH_POPUP_BOUNDARY_LABEL
            .contains("compact cache row stays local")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "mention rich picker boundary metadata"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("mention_picker_rich_popup_boundary_label")
            && capability.notes.contains("compact cached row")
            && capability.notes.contains("floating popup search")
            && capability.notes.contains("rich highlighted result list")
            && capability.notes.contains("pill editor")
            && capability.notes.contains("remote member lookup")
            && capability.notes.contains("profile/avatar fetch")
            && capability
                .notes
                .contains("attachment/edit mention payload editor")
            && capability.notes.contains("membership mutation")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_composer_mention_directory_disambiguation_boundary_keeps_rich_parts_blocked()
{
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_MENTION_DIRECTORY_DISAMBIGUATION_BOUNDARY_MARKER,
        "hepta_telegram_composer_mention_directory_disambiguation_boundary_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"mention_picker_send"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"mention_directory_disambiguation_boundary")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_DIRECTORY_DISAMBIGUATION_BOUNDARY_EVIDENCE
            .contains("MatrixRequest::SearchUserDirectory")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_DIRECTORY_DISAMBIGUATION_BOUNDARY_EVIDENCE
            .contains("client.search_users")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_DIRECTORY_DISAMBIGUATION_BOUNDARY_EVIDENCE
            .contains("duplicate display-name disambiguation UI")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_DIRECTORY_DISAMBIGUATION_BOUNDARY_EVIDENCE
            .contains("profile hover cards")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_DIRECTORY_DISAMBIGUATION_BOUNDARY_EVIDENCE
            .contains("multi-select mention tray")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_DIRECTORY_DISAMBIGUATION_BOUNDARY_EVIDENCE
            .contains("existing SendMessage add_mentions path")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_DIRECTORY_DISAMBIGUATION_BOUNDARY_LABEL
            .contains("Directory live reads Matrix user-directory metadata")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "mention directory disambiguation boundary"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("mention_picker_directory_disambiguation_boundary_label")
            && capability.notes.contains("active @query")
            && capability
                .notes
                .contains("MatrixRequest::SearchUserDirectory")
            && capability.notes.contains("client.search_users")
            && capability
                .notes
                .contains("UserDirectorySearchAction::Searched")
            && capability.notes.contains("loaded RoomScreen room_members")
            && capability
                .notes
                .contains("duplicate display-name disambiguation UI")
            && capability.notes.contains("profile hover cards")
            && capability.notes.contains("avatar/profile fetch")
            && capability.notes.contains("multi-select mention tray")
            && capability.notes.contains("pill editor")
            && capability
                .notes
                .contains("attachment/edit mention payload editor")
            && capability.notes.contains("extra SendMessage")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_composer_mention_rich_directory_controls_include_directory_live_read() {
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
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"mention_picker_send"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"mention_rich_directory_controls_row"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"mention_directory_search_live_wiring"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"mention_directory_result_promotion_live")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"mention_hover_card_snapshot_live"));
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_RICH_DIRECTORY_CONTROLS_EVIDENCE
            .contains("Rich, Directory, Hover, Tray, and Pills")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_RICH_DIRECTORY_CONTROLS_EVIDENCE
            .contains("visible controls")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_RICH_DIRECTORY_CONTROLS_EVIDENCE
            .contains("local rich mention packet snapshot")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_RICH_DIRECTORY_CONTROLS_EVIDENCE
            .contains("MatrixRequest::SearchUserDirectory")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_RICH_DIRECTORY_CONTROLS_EVIDENCE
            .contains("client.search_users")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_RICH_DIRECTORY_CONTROLS_EVIDENCE
            .contains("bounded directory result promotion row")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_RICH_DIRECTORY_CONTROLS_EVIDENCE
            .contains("local hover-card snapshot")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_RICH_DIRECTORY_CONTROLS_EVIDENCE
            .contains("UserDirectorySearchAction::Searched")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_RICH_DIRECTORY_CONTROLS_EVIDENCE
            .contains("profile hover card")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_RICH_DIRECTORY_CONTROLS_EVIDENCE
            .contains("multi-select mention tray")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_RICH_DIRECTORY_CONTROLS_EVIDENCE
            .contains("extra SendMessage")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_RICH_DIRECTORY_CONTROLS_LABEL
            .contains("Directory reads Matrix user-directory metadata")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_RICH_DIRECTORY_CONTROLS_LABEL
            .contains("Hover renders local cached/directory hover-card snapshots")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_HOVER_CARD_SNAPSHOT_LIVE_EVIDENCE
            .contains("live local hover-card snapshot")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "mention rich directory controls row"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("mention_rich_directory_controls_row")
            && capability
                .base_module
                .contains("mention_directory_result_promotion_row")
            && capability
                .base_module
                .contains("MatrixRequest::SearchUserDirectory")
            && capability
                .base_module
                .contains("UserDirectorySearchAction::Searched")
            && capability
                .base_module
                .contains("mention_picker_rich_mention_packet_snapshot_label")
            && capability
                .base_module
                .contains("mention_picker_hover_card_snapshot_label")
            && capability
                .notes
                .contains("Rich, Directory, Hover, Tray, and Pills")
            && capability
                .notes
                .contains("local rich mention packet snapshot")
            && capability.notes.contains("local hover-card snapshot")
            && capability.notes.contains("client.search_users")
            && capability.notes.contains("read-only result/error metadata")
            && capability
                .notes
                .contains("insert literal Matrix user-id tokens")
            && capability.notes.contains("active @query")
            && capability.notes.contains("cached suggestion count")
            && capability.notes.contains("selected token")
            && capability.notes.contains("@room power-level allowance")
            && capability.notes.contains("loaded room_members cache size")
            && capability.notes.contains("floating popup search")
            && capability.notes.contains("duplicate-name disambiguation")
            && capability.notes.contains("profile hover card")
            && capability.notes.contains("multi-select tray")
            && capability.notes.contains("pill editor mutation")
            && capability.notes.contains("attachment/edit mention payload")
            && capability.notes.contains("extra SendMessage")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "mention hover-card snapshot live"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("mention_picker_hover_card_snapshot_label")
            && capability
                .notes
                .contains("already available @mention metadata")
            && capability
                .notes
                .contains("cached Matrix user-directory result rows")
            && capability.notes.contains("selected cached RoomMember")
            && capability.notes.contains("@room power-level state")
            && capability.notes.contains("avatar MXC presence")
            && capability
                .notes
                .contains("no MatrixRequest::SearchUserDirectory")
            && capability.notes.contains("no profile/avatar fetch")
            && capability.notes.contains("no remote hover-card request")
            && capability.notes.contains("no SendMessage")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_composer_mention_directory_result_promotion_is_live_local_insert() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_MENTION_DIRECTORY_RESULT_PROMOTION_LIVE_MARKER,
        "hepta_telegram_composer_mention_directory_result_promotion_live_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"mention_picker_send"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"mention_directory_result_promotion_live")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_DIRECTORY_RESULT_PROMOTION_LIVE_EVIDENCE
            .contains("MatrixRequest::SearchUserDirectory")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_DIRECTORY_RESULT_PROMOTION_LIVE_EVIDENCE
            .contains("literal Matrix user id")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_DIRECTORY_RESULT_PROMOTION_LIVE_EVIDENCE
            .contains("insert_mention_token")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_DIRECTORY_RESULT_PROMOTION_LIVE_EVIDENCE
            .contains("no automatic insertion")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_DIRECTORY_RESULT_PROMOTION_LIVE_EVIDENCE
            .contains("existing SendMessage/add_mentions")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_DIRECTORY_RESULT_PROMOTION_LIVE_EVIDENCE
            .contains("extra SendMessage")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "mention directory result promotion live"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("mention_directory_result_promotion_row")
            && capability
                .base_module
                .contains("handle_directory_result_promotion_click")
            && capability.base_module.contains("insert_mention_token")
            && capability
                .notes
                .contains("UserDirectorySearchAction::Searched")
            && capability.notes.contains("client.search_users results")
            && capability.notes.contains("literal Matrix user id")
            && capability.notes.contains("appends trailing space")
            && capability.notes.contains("completed mention pill tray")
            && capability
                .notes
                .contains("Search completion does not auto-insert")
            && capability.notes.contains("extra SendMessage")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation request")
    }));
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|runway| {
        runway.gap_id == "mention_picker_send"
            && runway.current_path.contains("directory result promotion")
            && runway
                .current_path
                .contains("literal Matrix user-id tokens")
            && runway.remaining_gap.contains("richer directory result UI")
            && runway
                .next_ui_safe_step
                .contains("richer UI promotion contracts")
    }));
}

#[test]
fn hepta_telegram_base_composer_mention_local_pill_tray_is_live_local_text_editing() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_MENTION_LOCAL_PILL_TRAY_LIVE_MARKER,
        "hepta_telegram_composer_mention_local_pill_tray_live_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"mention_picker_send"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"mention_local_pill_tray_live"));
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_LOCAL_PILL_TRAY_LIVE_EVIDENCE
            .contains("live local completed-mention pill tray")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_LOCAL_PILL_TRAY_LIVE_EVIDENCE
            .contains("remove that completed token from composer text")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_LOCAL_PILL_TRAY_LIVE_EVIDENCE
            .contains("existing send-time Mentions payload preview")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_LOCAL_PILL_TRAY_LIVE_EVIDENCE
            .contains("server-side directory search")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_LOCAL_PILL_TRAY_LIVE_EVIDENCE
            .contains("SendMessage")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_LOCAL_PILL_TRAY_LIVE_LABEL
            .contains("completed @tokens")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "mention local pill tray live"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability.base_module.contains("mention_pill_tray")
            && capability
                .base_module
                .contains("completed_mention_pills_for_text")
            && capability
                .base_module
                .contains("remove_completed_mention_token")
            && capability.notes.contains("Completed @room")
            && capability.notes.contains("literal Matrix user-id")
            && capability.notes.contains("loaded-member display/localpart")
            && capability.notes.contains("unmatched local @tokens")
            && capability.notes.contains("remove that completed token")
            && capability.notes.contains("updates TextInput state")
            && capability
                .notes
                .contains("send-time Mentions payload preview")
            && capability
                .notes
                .contains("server-side member directory search")
            && capability.notes.contains("SendMessage")
            && capability.notes.contains("SendAttachment")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation request")
    }));
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|runway| {
        runway.gap_id == "mention_picker_send"
            && runway
                .current_path
                .contains("live completed-token pill tray removal")
            && !runway.remaining_gap.contains("pills,")
    }));
}

#[test]
fn hepta_telegram_base_composer_mention_preflight_detail_controls_are_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_MENTION_PREFLIGHT_DETAIL_CONTROLS_MARKER,
        "hepta_telegram_composer_mention_preflight_detail_controls_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"mention_picker_send"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"mention_preflight_detail_controls_row")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
            .contains("Request, Result, Error, Retry, and Source")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
            .contains("active @query")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
            .contains("existing SendMessage/add_mentions source")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
            .contains("server-side member directory search")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
            .contains("profile/avatar fetch")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
            .contains("extra SendMessage")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
            .contains("retry automation")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_PREFLIGHT_DETAIL_CONTROLS_LABEL
            .contains("Request/Result/Error/Retry/Source")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "mention preflight detail controls row"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("mention_preflight_detail_controls")
            && capability
                .notes
                .contains("Request, Result, Error, Retry, and Source")
            && capability.notes.contains("active @query")
            && capability.notes.contains("cached suggestion count")
            && capability.notes.contains("selected token")
            && capability.notes.contains("@room power-level allowance")
            && capability.notes.contains("loaded room_members cache size")
            && capability
                .notes
                .contains("existing SendMessage/add_mentions source")
            && capability.notes.contains("remote member lookup")
            && capability
                .notes
                .contains("server-side member directory search")
            && capability.notes.contains("profile/avatar fetch")
            && capability.notes.contains("attachment/edit mention payload")
            && capability.notes.contains("extra SendMessage")
            && capability.notes.contains("retry automation")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|runway| {
        runway.gap_id == "mention_picker_send"
            && runway
                .current_path
                .contains("rich/directory/preflight/payload-scope controls")
    }));
}
