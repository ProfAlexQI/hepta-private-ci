#[test]
fn hepta_telegram_base_composer_mention_send_boundary_uses_local_cache_payload() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_MENTION_SEND_LOCAL_BOUNDARY_MARKER,
        "hepta_telegram_composer_mention_send_local_boundary_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"mention_picker_send"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"mention_send_local_boundary_evidence"));
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_SEND_LOCAL_BOUNDARY_EVIDENCE
            .contains("Matrix Mentions")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_SEND_LOCAL_BOUNDARY_EVIDENCE
            .contains("RoomScreen room_members cache")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_SEND_LOCAL_BOUNDARY_EVIDENCE
            .contains("Full popup search")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_SEND_LOCAL_BOUNDARY_EVIDENCE
            .contains("up to three cached-member matches")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_SEND_LOCAL_BOUNDARY_EVIDENCE
            .contains("Tab/Enter")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_SEND_LOCAL_BOUNDARY_EVIDENCE
            .contains("ArrowUp/ArrowDown")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_SEND_LOCAL_BOUNDARY_EVIDENCE
            .contains("trailing space")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_SEND_LOCAL_BOUNDARY_EVIDENCE
            .contains("cached suggestion count")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_SEND_LOCAL_BOUNDARY_EVIDENCE
            .contains("no-match state")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_CACHED_SELECTION_EVIDENCE
            .contains("selected token")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_CACHED_SELECTION_EVIDENCE
            .contains("no-match state")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_CACHED_SELECTION_EVIDENCE
            .contains("no remote member lookup")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_SEND_LOCAL_BOUNDARY_EVIDENCE
            .contains("remote member lookup")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_SEND_LOCAL_BOUNDARY_LABEL
            .contains("completed-token pills")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "mention payload local-cache boundary evidence"
            && capability
                .notes
                .contains("full mention_picker_send remains a base gap")
            && capability.notes.contains("/html")
            && capability.notes.contains("/plain")
            && capability.notes.contains("add_mentions")
            && capability.notes.contains("Matrix Mentions")
            && capability.notes.contains("RoomScreen room_members")
            && capability
                .notes
                .contains("completed mentions no longer intercept Enter")
            && capability.notes.contains("no remote member lookup")
            && capability.notes.contains("extra SendMessage")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_composer_mention_send_payload_metadata_is_local() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_MENTION_SEND_PAYLOAD_METADATA_MARKER,
        "hepta_telegram_composer_mention_send_payload_metadata_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"mention_picker_send"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"mention_send_payload_metadata_preview")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_SEND_PAYLOAD_METADATA_EVIDENCE
            .contains("send-time mention payload metadata")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_SEND_PAYLOAD_METADATA_EVIDENCE
            .contains("deduped Matrix mention user count")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_SEND_PAYLOAD_METADATA_EVIDENCE
            .contains("cached RoomMember display/localpart match count")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_SEND_PAYLOAD_METADATA_EVIDENCE
            .contains("@room flag state")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_SEND_PAYLOAD_METADATA_EVIDENCE
            .contains("loaded room_members cache size")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_SEND_PAYLOAD_METADATA_LABEL
            .contains("loaded member-cache counts")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "mention send payload metadata"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("mention_send_payload_metadata_label")
            && capability
                .notes
                .contains("send-time mention payload metadata")
            && capability.notes.contains("scanned @token count")
            && capability
                .notes
                .contains("deduped Matrix mention user count")
            && capability.notes.contains("literal Matrix user-id tokens")
            && capability
                .notes
                .contains("cached RoomMember display/localpart matches")
            && capability.notes.contains("loaded room_members cache size")
            && capability
                .notes
                .contains("add_mentions attaches Matrix Mentions once")
            && capability.notes.contains("mentions_for_text")
            && capability.notes.contains("AttachmentConfig.mentions")
            && capability.notes.contains("remote member lookup")
            && capability.notes.contains("extra SendMessage")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_composer_mention_send_live_wiring_is_partial_live() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_MENTION_SEND_LIVE_WIRING_MARKER,
        "hepta_telegram_composer_mention_send_live_wiring_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"mention_picker_send"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"mention_send_live_payload_wiring"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "mention send live payload wiring"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("create_message_with_mentions")
            && capability
                .base_module
                .contains("MatrixRequest::SendMessage")
            && capability
                .base_module
                .contains("MatrixRequest::SendAttachment")
            && capability.base_module.contains("AttachmentConfig.mentions")
            && capability.notes.contains("partial-live")
            && capability.notes.contains("cached mention payloads")
            && capability
                .notes
                .contains("add_mentions attaches Matrix Mentions")
            && capability.notes.contains("AttachmentConfig.mentions")
            && capability.notes.contains("Timeline::send_attachment")
            && capability.notes.contains("already loaded room_members")
            && capability.notes.contains("normal user-initiated send path")
            && capability.notes.contains("server-side directory search")
            && capability.notes.contains("edit mention payload rewrite")
            && capability.notes.contains("extra SendMessage")
            && capability.notes.contains("gateway/runtime/auth")
            && capability
                .notes
                .contains("outside the existing SendMessage/SendAttachment payloads")
    }));
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|runway| {
        runway.gap_id == "mention_picker_send"
            && runway
                .current_path
                .contains("live SendMessage/add_mentions payload wiring")
            && runway
                .current_path
                .contains("live attachment caption AttachmentConfig.mentions wiring")
            && runway
                .next_ui_safe_step
                .contains("coordinate backend mention contracts")
    }));
}

#[test]
fn hepta_telegram_base_composer_mention_payload_scope_controls_are_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_MENTION_PAYLOAD_SCOPE_CONTROLS_MARKER,
        "hepta_telegram_composer_mention_payload_scope_controls_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"mention_picker_send"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"mention_payload_scope_controls_row"));
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_PAYLOAD_SCOPE_CONTROLS_EVIDENCE
            .contains("Send, Attach, Edit, Reply, Source, Packet, Contract, and Taxonomy")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_PAYLOAD_SCOPE_CONTROLS_EVIDENCE
            .contains("existing SendMessage/add_mentions source")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_PAYLOAD_SCOPE_CONTROLS_EVIDENCE
            .contains("attachment-caption AttachmentConfig.mentions source")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_PAYLOAD_SCOPE_CONTROLS_EVIDENCE
            .contains("rich attachment mention payload editor")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_PAYLOAD_SCOPE_CONTROLS_EVIDENCE
            .contains("edit-message mention payload rewrite")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_PAYLOAD_SCOPE_CONTROLS_EVIDENCE
            .contains("reply mention rewriting")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_PAYLOAD_SCOPE_CONTROLS_LABEL
            .contains("typed mention payload contract")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "mention payload scope controls row"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("mention_payload_scope_controls")
            && capability
                .notes
                .contains("Send, Attach, Edit, Reply, Source, Packet")
            && capability.notes.contains("Contract")
            && capability.notes.contains("active @query")
            && capability.notes.contains("cached suggestion count")
            && capability.notes.contains("selected token")
            && capability.notes.contains("loaded room_members cache size")
            && capability
                .notes
                .contains("existing SendMessage/add_mentions source")
            && capability
                .notes
                .contains("attachment-caption mentions_for_text sources")
            && capability.notes.contains("Packet persists drilldown")
            && capability.notes.contains("Contract maps it to typed")
            && capability
                .notes
                .contains("Taxonomy records remote hover/profile")
            && capability.notes.contains("rich attachment payload editors")
            && capability
                .notes
                .contains("edit-message mention payload rewrites")
            && capability.notes.contains("reply mention rewriting")
            && capability
                .notes
                .contains("server-side member directory search")
            && capability.notes.contains("SendAttachment")
            && capability.notes.contains("extra SendMessage")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|runway| {
        runway.gap_id == "mention_picker_send"
            && runway.current_path.contains("payload-scope controls")
            && runway
                .remaining_gap
                .contains("rich attachment payload editors")
    }));
}

#[test]
fn hepta_telegram_base_composer_mention_payload_drilldown_packet_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_MENTION_PAYLOAD_DRILLDOWN_PACKET_MARKER,
        "hepta_telegram_composer_mention_payload_drilldown_packet_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"mention_picker_send"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"mention_payload_drilldown_packet_action")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_PAYLOAD_DRILLDOWN_PACKET_EVIDENCE
            .contains("visible Packet control")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_PAYLOAD_DRILLDOWN_PACKET_EVIDENCE
            .contains("mention payload drilldown acceptance matrix")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_PAYLOAD_DRILLDOWN_PACKET_EVIDENCE
            .contains("rich picker, server directory")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_PAYLOAD_DRILLDOWN_PACKET_EVIDENCE
            .contains("attachment-caption AttachmentConfig.mentions")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_PAYLOAD_DRILLDOWN_PACKET_EVIDENCE
            .contains("rich attachment/edit/reply payload scopes")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_PAYLOAD_DRILLDOWN_PACKET_EVIDENCE
            .contains("Request/Result/Error/Retry/Source")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_PAYLOAD_DRILLDOWN_PACKET_LABEL
            .contains("rich picker, directory, preflight")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "mention payload drilldown packet"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("mention_payload_drilldown_button")
            && capability
                .base_module
                .contains("mention_picker_payload_drilldown_packet_label")
            && capability.notes.contains("Packet")
            && capability
                .notes
                .contains("mention payload drilldown acceptance matrix")
            && capability.notes.contains("rich picker")
            && capability.notes.contains("server directory")
            && capability.notes.contains("duplicate-name disambiguation")
            && capability.notes.contains("hover-card")
            && capability.notes.contains("tray")
            && capability.notes.contains("pills")
            && capability.notes.contains("SendMessage/add_mentions")
            && capability
                .notes
                .contains("rich attachment/edit/reply payload scope")
            && capability
                .notes
                .contains("Request/Result/Error/Retry/Source")
            && capability.notes.contains("remote member lookup")
            && capability
                .notes
                .contains("server-side member directory search")
            && capability.notes.contains("profile/avatar fetch")
            && capability.notes.contains("pill editor mutation")
            && capability
                .notes
                .contains("rich attachment/edit/reply mention payload rewrite")
            && capability
                .notes
                .contains("extra SendAttachment beyond the review-row handoff")
            && capability.notes.contains("extra SendMessage")
            && capability.notes.contains("retry automation")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|runway| {
        runway.gap_id == "mention_picker_send"
            && runway
                .current_path
                .contains("mention payload drilldown packet")
            && runway
                .remaining_gap
                .contains("typed rich/edit/reply payload scopes")
            && runway
                .next_ui_safe_step
                .contains("coordinate backend mention contracts")
    }));
}

#[test]
fn hepta_telegram_base_composer_mention_payload_typed_contract_packet_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_MENTION_PAYLOAD_TYPED_CONTRACT_PACKET_MARKER,
        "hepta_telegram_composer_mention_payload_typed_contract_packet_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"mention_picker_send"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"mention_payload_typed_contract_packet_action")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_PAYLOAD_TYPED_CONTRACT_PACKET_EVIDENCE
            .contains("visible Contract control")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_PAYLOAD_TYPED_CONTRACT_PACKET_EVIDENCE
            .contains("typed mention contract slots")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_PAYLOAD_TYPED_CONTRACT_PACKET_EVIDENCE
            .contains("server-directory lookup")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_PAYLOAD_TYPED_CONTRACT_PACKET_EVIDENCE
            .contains("attachment-caption AttachmentConfig.mentions handoff")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_PAYLOAD_TYPED_CONTRACT_PACKET_EVIDENCE
            .contains("rich attachment/edit/reply payload scopes")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_PAYLOAD_TYPED_CONTRACT_PACKET_EVIDENCE
            .contains("source-hash")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_PAYLOAD_TYPED_CONTRACT_PACKET_EVIDENCE
            .contains("stale-token handling")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_PAYLOAD_TYPED_CONTRACT_PACKET_LABEL
            .contains("typed rich-picker, directory")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "mention payload typed contract packet"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("mention_payload_contract_button")
            && capability
                .base_module
                .contains("mention_picker_payload_typed_contract_packet_label")
            && capability.notes.contains("Contract")
            && capability.notes.contains("typed mention contracts")
            && capability
                .notes
                .contains("rich picker request/result/error/retry/source")
            && capability.notes.contains("server directory lookup")
            && capability.notes.contains("duplicate-name disambiguation")
            && capability.notes.contains("hover-card source")
            && capability.notes.contains("tray state")
            && capability.notes.contains("pill draft")
            && capability.notes.contains("SendMessage/add_mentions")
            && capability.notes.contains("AttachmentConfig.mentions")
            && capability
                .notes
                .contains("rich attachment/edit/reply payload scopes")
            && capability.notes.contains("source-hash")
            && capability.notes.contains("stale-token handling")
            && capability.notes.contains("idempotency")
            && capability.notes.contains("remote member lookup")
            && capability
                .notes
                .contains("server-side member directory search")
            && capability.notes.contains("profile/avatar fetch")
            && capability.notes.contains("pill editor mutation")
            && capability
                .notes
                .contains("rich attachment/edit/reply mention payload rewrite")
            && capability
                .notes
                .contains("extra SendAttachment beyond the review-row handoff")
            && capability.notes.contains("extra SendMessage")
            && capability.notes.contains("retry automation")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|runway| {
        runway.gap_id == "mention_picker_send"
            && runway
                .current_path
                .contains("typed mention contract packet")
            && runway
                .next_ui_safe_step
                .contains("coordinate backend mention contracts")
    }));
}

#[test]
fn hepta_telegram_base_composer_mention_remote_result_taxonomy_packet_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_MENTION_REMOTE_RESULT_TAXONOMY_PACKET_MARKER,
        "hepta_telegram_composer_mention_remote_result_taxonomy_packet_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"mention_picker_send"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"mention_remote_result_taxonomy_packet_action")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_REMOTE_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("visible Taxonomy control")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_REMOTE_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("remote hover/profile/disambiguation/edit-reply")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_REMOTE_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("not-assigned or not-wired")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_REMOTE_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("SendMessage/add_mentions")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_REMOTE_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("AttachmentConfig.mentions")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_REMOTE_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("MatrixRequest::SearchUserDirectory")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_REMOTE_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("no profile/avatar fetch")
    );
    assert!(
        crate::shared::mentionable_text_input::MENTION_PICKER_REMOTE_RESULT_TAXONOMY_PACKET_LABEL
            .contains("local not-wired metadata")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "mention remote result taxonomy packet"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("mention_payload_taxonomy_button")
            && capability
                .base_module
                .contains("mention_picker_remote_result_taxonomy_packet_label")
            && capability.notes.contains("Taxonomy")
            && capability.notes.contains("SendMessage/add_mentions")
            && capability.notes.contains("AttachmentConfig.mentions")
            && capability
                .notes
                .contains("MatrixRequest::SearchUserDirectory")
            && capability
                .notes
                .contains("bounded directory result promotion")
            && capability
                .notes
                .contains("remote hover-card/profile result")
            && capability
                .notes
                .contains("edit/reply mention payload rewrite result")
            && capability.notes.contains("source-hash reconciliation")
            && capability.notes.contains("audit redaction")
            && capability
                .notes
                .contains("remote member lookup beyond explicit Directory")
            && capability.notes.contains("profile/avatar fetch")
            && capability.notes.contains("remote hover-card request")
            && capability
                .notes
                .contains("duplicate-name disambiguation workflow")
            && capability.notes.contains("extra SendMessage")
            && capability.notes.contains("extra SendAttachment")
            && capability.notes.contains("Telegram delivery")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|runway| {
        runway.gap_id == "mention_picker_send"
            && runway
                .current_path
                .contains("remote hover/profile/disambiguation/edit-reply result taxonomy packet")
            && runway
                .remaining_gap
                .contains("remote hover-card/profile adapter")
            && runway
                .next_ui_safe_step
                .contains("coordinate backend mention contracts")
    }));
}

#[test]
fn hepta_telegram_base_composer_send_shortcut_is_local_preference() {
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_SEND_SHORTCUT_LOCAL_PREFERENCE_MARKER,
        "hepta_telegram_composer_send_shortcut_local_preference_ready"
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"composer_send_shortcut_local_preference_evidence")
    );
    assert!(
        crate::settings::app_settings::SEND_SHORTCUT_LOCAL_PREFERENCE_EVIDENCE
            .contains("AppPreferencesAction::SendOnEnterChanged")
    );
    assert!(
        crate::settings::app_settings::SEND_SHORTCUT_LOCAL_PREFERENCE_EVIDENCE
            .contains("RoomInputBar and EditingPane update submit_on_enter")
    );
    assert!(
        crate::settings::app_settings::SEND_SHORTCUT_LOCAL_PREFERENCE_EVIDENCE
            .contains("no message send, typing notice")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "composer send shortcut local preference evidence"
            && capability.notes.contains("Send Message Keyboard Shortcut")
            && capability
                .notes
                .contains("AppPreferencesAction::SendOnEnterChanged")
            && capability.notes.contains("submit_on_enter")
            && capability.notes.contains("typing notice")
            && capability.notes.contains("room-state")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_message_send_operation_status_is_local_evidence() {
    assert!(
        crate::room::room_input_bar::MESSAGE_SEND_OPERATION_STATUS_EVIDENCE
            .contains("existing MatrixRequest send paths")
    );
    assert!(
        crate::room::room_input_bar::MESSAGE_SEND_OPERATION_STATUS_EVIDENCE
            .contains("queued/progress/failure labels")
    );
    assert!(
        crate::room::room_input_bar::MESSAGE_SEND_OPERATION_STATUS_EVIDENCE
            .contains("Attachment worker failure Retry is the one guarded exception")
    );
    assert!(
        crate::room::room_input_bar::MESSAGE_SEND_OPERATION_STATUS_EVIDENCE
            .contains("Retry never auto-runs")
    );
    assert!(
        crate::room::room_input_bar::MESSAGE_SEND_OPERATION_STATUS_EVIDENCE.contains(
            "room-state, membership, account, profile, gateway/runtime/auth, or live mutation"
        )
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_SEND_OPERATION_STATUS_LOCAL_MARKER,
        "hepta_telegram_message_send_operation_status_local_ready"
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_send_operation_status_local_evidence")
    );
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"message_send_operation_status"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message send operation status local evidence"
            && capability.base_module
                == "RoomInputBar + MatrixRequest::SendMessage + MatrixRequest::SendAttachment"
            && capability.notes.contains("existing MatrixRequest paths")
            && capability.notes.contains("queued/progress/failure labels")
            && capability.notes.contains("failed attachment handoff Retry")
            && capability.notes.contains("PositiveConfirmationModal")
            && capability.notes.contains("room-state")
            && capability.notes.contains("membership")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_composer_typing_notice_send_reuses_existing_path() {
    assert!(
        crate::room::room_input_bar::COMPOSER_TYPING_NOTICE_SEND_EVIDENCE
            .contains("existing MatrixRequest::SendTypingNotice path")
    );
    assert!(
        crate::room::room_input_bar::COMPOSER_TYPING_NOTICE_SEND_EVIDENCE
            .contains("reserved Hepta command previews suppress Matrix typing notices")
    );
    assert!(
        crate::room::room_input_bar::COMPOSER_TYPING_NOTICE_SEND_EVIDENCE
            .contains("set_typing_notice_status only update local labels")
    );
    assert!(
        crate::room::room_input_bar::COMPOSER_TYPING_NOTICE_SEND_EVIDENCE
            .contains("extra Matrix request beyond the intended typing notice")
    );
    assert_eq!(
        HEPTA_TELEGRAM_COMPOSER_TYPING_NOTICE_SEND_MARKER,
        "hepta_telegram_composer_typing_notice_send_ready"
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"composer_typing_notice_send_evidence"));
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"composer_typing_notice_send"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "composer typing notice send evidence"
            && capability.base_module == "RoomInputBar + MatrixRequest::SendTypingNotice"
            && capability
                .notes
                .contains("existing MatrixRequest::SendTypingNotice path")
            && capability
                .notes
                .contains("reserved Hepta command previews suppress typing notices")
            && capability.notes.contains("only update local labels")
            && capability.notes.contains("no message")
            && capability.notes.contains("room-state")
            && capability.notes.contains("retry")
            && capability.notes.contains("cancel")
            && capability.notes.contains("membership")
            && capability.notes.contains("extra Matrix request")
    }));
}

#[test]
fn hepta_telegram_base_location_send_confirmation_is_guarded() {
    assert!(
        crate::room::room_input_bar::LOCATION_SEND_CONFIRMATION_EVIDENCE
            .contains("local ConfirmationModal")
    );
    assert!(
        crate::room::room_input_bar::LOCATION_SEND_CONFIRMATION_EVIDENCE
            .contains("confirmed accept handler emits LocationSendConfirmed")
    );
    assert!(
        crate::room::room_input_bar::LOCATION_SEND_CONFIRMATION_EVIDENCE
            .contains("existing MatrixRequest::SendMessage location path")
    );
    assert!(
        crate::room::room_input_bar::LOCATION_SEND_CONFIRMATION_EVIDENCE
            .contains("no location SendMessage before confirmation")
    );
    assert_eq!(
        HEPTA_TELEGRAM_LOCATION_SEND_CONFIRMATION_MARKER,
        "hepta_telegram_location_send_confirmation_ready"
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"location_send_confirmation_guard"));
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"location_send"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "location send confirmation"
            && capability
                .notes
                .contains("existing Matrix location SendMessage path")
            && capability.notes.contains("confirmed accept handler")
            && capability.notes.contains("LocationSendConfirmed")
            && capability
                .notes
                .contains("no location SendMessage before confirmation")
            && capability.notes.contains("room-state")
            && capability.notes.contains("membership")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_live_location_continuous_updates_have_ui_boundary() {
    assert!(
        crate::home::location_preview::LIVE_LOCATION_CONTINUOUS_UPDATES_BOUNDARY_EVIDENCE
            .contains("LocationRequest::UpdateOnce")
    );
    assert!(
        crate::home::location_preview::LIVE_LOCATION_CONTINUOUS_UPDATES_BOUNDARY_EVIDENCE
            .contains("LocationRequest::StartUpdates")
    );
    assert!(
        crate::home::location_preview::LIVE_LOCATION_CONTINUOUS_UPDATES_BOUNDARY_EVIDENCE
            .contains("LocationRequest::StopUpdates")
    );
    assert!(
        crate::home::location_preview::LIVE_LOCATION_CONTINUOUS_UPDATES_BOUNDARY_EVIDENCE
            .contains("Start Device Updates")
    );
    assert!(
        crate::home::location_preview::LIVE_LOCATION_CONTINUOUS_UPDATES_BOUNDARY_EVIDENCE
            .contains("Stop Device Updates")
    );
    assert!(
        crate::home::location_preview::LIVE_LOCATION_CONTINUOUS_UPDATES_BOUNDARY_EVIDENCE
            .contains("do not create a live-location Matrix event")
    );
    assert!(
        crate::home::location_preview::LIVE_LOCATION_CONTINUOUS_UPDATES_BOUNDARY_LABEL
            .contains("device updates off")
    );
    assert!(
        crate::home::location_preview::LIVE_LOCATION_CONTINUOUS_UPDATES_ACTIVE_LABEL
            .contains("device updates on locally")
    );
    assert_eq!(
        HEPTA_TELEGRAM_LIVE_LOCATION_CONTINUOUS_UPDATES_BOUNDARY_MARKER,
        "hepta_telegram_live_location_continuous_updates_boundary_ready"
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"live_location_continuous_updates_boundary")
    );
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"live_location"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "live location continuous updates boundary"
            && capability.notes.contains("LocationRequest::StartUpdates")
            && capability.notes.contains("LocationRequest::StopUpdates")
            && capability.notes.contains("Start")
            && capability.notes.contains("Stop")
            && capability.notes.contains("live-location Matrix event")
            && capability.notes.contains("SendMessage")
            && capability.notes.contains("room-state")
            && capability.notes.contains("membership")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_message_report_surface_has_confirmed_report_content() {
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_REPORT_LOCAL_SURFACE_MARKER,
        "hepta_telegram_message_report_local_surface_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"message_report_send"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"message_report_surface"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message report/send"
            && capability
                .notes
                .contains("opening Report and Cancel remain local")
            && capability
                .notes
                .contains("Spam, Abuse, and Custom reason open a confirmation guard")
            && capability.notes.contains("MatrixRequest::ReportContent")
            && capability.notes.contains("Room::report_content")
            && capability
                .notes
                .contains("empty custom reason stays unsent")
    }));
}

#[test]
fn hepta_telegram_base_message_report_option_staging_confirms_before_send() {
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_REPORT_OPTION_STAGING_LOCAL_MARKER,
        "hepta_telegram_message_report_option_staging_local_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"message_report_send"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_report_option_staging_local_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_report_send_local_boundary_evidence")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message report option staging local evidence"
            && capability.notes.contains("Report opening and Cancel")
            && capability.notes.contains("local Telegram report preview")
            && capability
                .notes
                .contains("Spam, Abuse, and Custom reason require a confirmation modal")
            && capability.notes.contains("empty custom reason stays local")
            && capability.notes.contains("MatrixRequest::ReportContent")
    }));
}

#[test]
fn hepta_telegram_base_message_report_send_boundary_is_confirmed_and_narrow() {
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_REPORT_SEND_LOCAL_BOUNDARY_MARKER,
        "hepta_telegram_message_report_send_local_boundary_ready"
    );
    assert!(
        crate::home::new_message_context_menu::MESSAGE_REPORT_SEND_LOCAL_BOUNDARY_EVIDENCE
            .contains("confirmation before MatrixRequest::ReportContent")
    );
    assert!(
        crate::home::new_message_context_menu::MESSAGE_REPORT_SEND_LOCAL_BOUNDARY_EVIDENCE
            .contains("Custom reason")
    );
    assert!(
        crate::home::new_message_context_menu::MESSAGE_REPORT_SEND_LOCAL_BOUNDARY_EVIDENCE
            .contains("empty custom reason stays unsent")
    );
    assert!(
        crate::home::new_message_context_menu::MESSAGE_REPORT_SEND_LOCAL_BOUNDARY_EVIDENCE
            .contains("Room::report_content")
    );
    assert!(
        crate::home::new_message_context_menu::MESSAGE_REPORT_SEND_LOCAL_BOUNDARY_EVIDENCE
            .contains("moderation policy lookup")
    );
    assert!(
        crate::home::new_message_context_menu::MESSAGE_REPORT_SEND_LOCAL_BOUNDARY_EVIDENCE
            .contains("redact/delete")
    );
    assert!(
        crate::home::new_message_context_menu::MESSAGE_REPORT_SEND_LOCAL_BOUNDARY_LABEL
            .contains("Custom confirm before Matrix report_content")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_report_send_local_boundary_evidence")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message report send local boundary evidence"
            && capability.base_module.contains("Room::report_content")
            && capability.notes.contains("message_report_send")
            && capability.notes.contains("MatrixRequest::ReportContent")
            && capability.notes.contains("Room::report_content")
            && capability.notes.contains("moderation policy lookup")
            && capability.notes.contains("redact/delete")
            && capability.notes.contains("ban")
            && capability.notes.contains("kick")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_message_report_moderation_workflow_boundary_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_REPORT_MODERATION_WORKFLOW_BOUNDARY_MARKER,
        "hepta_telegram_message_report_moderation_workflow_boundary_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"message_report_send"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_report_moderation_workflow_boundary")
    );
    assert!(
        crate::home::new_message_context_menu::MESSAGE_REPORT_MODERATION_WORKFLOW_BOUNDARY_EVIDENCE
            .contains("loaded target row")
    );
    assert!(
        crate::home::new_message_context_menu::MESSAGE_REPORT_MODERATION_WORKFLOW_BOUNDARY_EVIDENCE
            .contains("local custom-reason readiness")
    );
    assert!(
        crate::home::new_message_context_menu::MESSAGE_REPORT_MODERATION_WORKFLOW_BOUNDARY_EVIDENCE
            .contains("Moderation queue controls")
    );
    assert!(
        crate::home::new_message_context_menu::MESSAGE_REPORT_MODERATION_WORKFLOW_BOUNDARY_EVIDENCE
            .contains("server policy lookup")
    );
    assert!(
        crate::home::new_message_context_menu::MESSAGE_REPORT_MODERATION_WORKFLOW_BOUNDARY_EVIDENCE
            .contains("redact/delete")
    );
    assert!(
        crate::home::new_message_context_menu::MESSAGE_REPORT_MODERATION_WORKFLOW_BOUNDARY_EVIDENCE
            .contains("reviewer assignment")
    );
    assert!(
        crate::home::new_message_context_menu::MESSAGE_REPORT_MODERATION_WORKFLOW_BOUNDARY_LABEL
            .contains("local blocked evidence")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message report moderation workflow boundary"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("report_moderation_boundary")
            && capability.notes.contains("loaded target row")
            && capability.notes.contains("custom-reason readiness")
            && capability.notes.contains("Moderation queue")
            && capability.notes.contains("server policy lookup")
            && capability.notes.contains("redact/delete")
            && capability.notes.contains("ban")
            && capability.notes.contains("kick")
            && capability.notes.contains("ignore/block")
            && capability.notes.contains("reviewer assignment")
            && capability.notes.contains("appeal flow")
            && capability.notes.contains("MatrixRequest::ReportContent")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_message_report_loaded_target_metadata_preview_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_REPORT_LOADED_TARGET_METADATA_MARKER,
        "hepta_telegram_message_report_loaded_target_metadata_ready"
    );
    assert!(
        crate::home::new_message_context_menu::MESSAGE_REPORT_LOADED_TARGET_METADATA_EVIDENCE
            .contains("selected loaded timeline row")
    );
    assert!(
        crate::home::new_message_context_menu::MESSAGE_REPORT_LOADED_TARGET_METADATA_EVIDENCE
            .contains("loaded body preview")
    );
    assert!(
        crate::home::new_message_context_menu::MESSAGE_REPORT_LOADED_TARGET_METADATA_EVIDENCE
            .contains("character count")
    );
    assert!(
        crate::home::new_message_context_menu::MESSAGE_REPORT_LOADED_TARGET_METADATA_EVIDENCE
            .contains("byte count")
    );
    assert!(
        crate::home::new_message_context_menu::MESSAGE_REPORT_LOADED_TARGET_METADATA_EVIDENCE
            .contains("local echo send-handle")
    );
    assert!(
        crate::home::new_message_context_menu::MESSAGE_REPORT_LOADED_TARGET_METADATA_EVIDENCE
            .contains("moderation policy lookup")
    );
    assert!(
        crate::home::new_message_context_menu::MESSAGE_REPORT_LOADED_TARGET_METADATA_EVIDENCE
            .contains("relations fetch")
    );
    assert!(
        crate::home::new_message_context_menu::MESSAGE_REPORT_LOADED_TARGET_METADATA_LABEL
            .contains("no moderation lookup or report before confirmation")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_report_loaded_target_metadata_preview")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message report loaded target metadata preview"
            && capability
                .base_module
                .contains("NewMessageContextMenu report preview loaded metadata")
            && capability.notes.contains("selected loaded row index")
            && capability.notes.contains("event-id availability")
            && capability.notes.contains("loaded body preview")
            && capability.notes.contains("character count")
            && capability.notes.contains("byte count")
            && capability.notes.contains("thread-root availability")
            && capability
                .notes
                .contains("local echo send-handle availability")
            && capability
                .notes
                .contains("Matrix report_content before confirmation")
            && capability.notes.contains("moderation policy lookup")
            && capability.notes.contains("relations fetch")
            && capability.notes.contains("event-context fetch")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_message_report_custom_reason_draft_metadata_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_REPORT_CUSTOM_REASON_DRAFT_METADATA_MARKER,
        "hepta_telegram_message_report_custom_reason_draft_metadata_ready"
    );
    assert!(
        crate::home::new_message_context_menu::MESSAGE_REPORT_CUSTOM_REASON_DRAFT_METADATA_EVIDENCE
            .contains("local text input")
    );
    assert!(
        crate::home::new_message_context_menu::MESSAGE_REPORT_CUSTOM_REASON_DRAFT_METADATA_EVIDENCE
            .contains("raw character count")
    );
    assert!(
        crate::home::new_message_context_menu::MESSAGE_REPORT_CUSTOM_REASON_DRAFT_METADATA_EVIDENCE
            .contains("raw byte count")
    );
    assert!(
        crate::home::new_message_context_menu::MESSAGE_REPORT_CUSTOM_REASON_DRAFT_METADATA_EVIDENCE
            .contains("240-character cap state")
    );
    assert!(
        crate::home::new_message_context_menu::MESSAGE_REPORT_CUSTOM_REASON_DRAFT_METADATA_EVIDENCE
            .contains("empty-versus-ready state")
    );
    assert!(
        crate::home::new_message_context_menu::MESSAGE_REPORT_CUSTOM_REASON_DRAFT_METADATA_EVIDENCE
            .contains("target event-id availability")
    );
    assert!(
        crate::home::new_message_context_menu::MESSAGE_REPORT_CUSTOM_REASON_DRAFT_METADATA_EVIDENCE
            .contains("Matrix report_content before confirmation")
    );
    assert!(
        crate::home::new_message_context_menu::MESSAGE_REPORT_CUSTOM_REASON_DRAFT_METADATA_LABEL
            .contains("no ReportContent before confirmation")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_report_custom_reason_draft_metadata_preview")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message report custom reason draft metadata"
            && capability.base_module.contains("custom reason input")
            && capability.notes.contains("local report reason text input")
            && capability.notes.contains("raw character count")
            && capability.notes.contains("raw byte count")
            && capability.notes.contains("240-character cap state")
            && capability.notes.contains("empty-versus-ready state")
            && capability.notes.contains("target event-id availability")
            && capability
                .notes
                .contains("Matrix report_content before confirmation")
            && capability.notes.contains("moderation policy lookup")
            && capability.notes.contains("relations fetch")
            && capability.notes.contains("event-context fetch")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_message_report_cancel_is_local_cleanup_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_REPORT_CANCEL_LOCAL_MARKER,
        "hepta_telegram_message_report_cancel_local_ready"
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"message_report_cancel_local_evidence"));
    assert!(
        crate::home::new_message_context_menu::MESSAGE_REPORT_CANCEL_LOCAL_EVIDENCE
            .contains("Report Cancel and Escape")
    );
    assert!(
        crate::home::new_message_context_menu::MESSAGE_REPORT_CANCEL_LOCAL_EVIDENCE
            .contains("hide the local report preview")
    );
    assert!(
        crate::home::new_message_context_menu::MESSAGE_REPORT_CANCEL_LOCAL_EVIDENCE
            .contains("MatrixRequest::ReportContent")
    );
    assert!(
        crate::home::new_message_context_menu::MESSAGE_REPORT_CANCEL_LOCAL_EVIDENCE
            .contains("moderation queue")
    );
    assert!(
        crate::home::new_message_context_menu::MESSAGE_REPORT_CANCEL_LOCAL_EVIDENCE
            .contains("moderation policy")
    );
    assert!(
        crate::home::new_message_context_menu::MESSAGE_REPORT_CANCEL_LOCAL_LABEL
            .contains("local preview cleanup")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message report cancel local evidence"
            && capability.base_module.contains("Cancel/Escape")
            && capability.notes.contains("hide the local report preview")
            && capability.notes.contains("restore the Report button")
            && capability.notes.contains("MatrixRequest::ReportContent")
            && capability.notes.contains("moderation queue")
            && capability.notes.contains("moderation policy")
            && capability.notes.contains("relations")
            && capability.notes.contains("event context")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_message_report_content_live_send_wiring_is_partial_live() {
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_REPORT_CONTENT_LIVE_SEND_WIRING_MARKER,
        "hepta_telegram_message_report_content_live_send_wiring_ready"
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"message_report_content_live_send_wiring")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message report content live send wiring"
            && capability
                .base_module
                .contains("MessageAction::Report/RetryReport")
            && capability
                .base_module
                .contains("MatrixRequest::ReportContent")
            && capability
                .base_module
                .contains("TimelineUpdate::MessageReportResult")
            && capability.notes.contains("Room::report_content")
            && capability.notes.contains("PositiveConfirmationModal")
            && capability.notes.contains("partial-live path")
            && capability.notes.contains("moderation queue")
            && capability.notes.contains("policy lookup")
            && capability.notes.contains("reviewer assignment")
            && capability.notes.contains("appeal/enforcement workflow")
            && capability.notes.contains("room-state mutation")
            && capability.notes.contains("membership mutation")
            && capability.notes.contains("gateway/runtime/auth/provider")
            && capability.notes.contains("Telegram delivery")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|runway| {
        runway.gap_id == "message_report_send"
            && runway
                .current_path
                .contains("live confirmed Spam/Abuse/Custom ReportContent")
            && runway.current_path.contains("confirmed failed-state Retry")
            && runway.remaining_gap.contains("moderation workflow")
            && runway.remaining_gap.contains("policy lookup")
            && runway.remaining_gap.contains("reviewer assignment")
    }));
}

#[test]
fn hepta_telegram_base_message_report_status_lifecycle_is_visible_result_metadata() {
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_REPORT_STATUS_LIFECYCLE_MARKER,
        "hepta_telegram_message_report_status_lifecycle_ready"
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_STATUS_LIFECYCLE_EVIDENCE
            .contains("MatrixRequest::ReportContent")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_STATUS_LIFECYCLE_EVIDENCE
            .contains("TimelineUpdate::MessageReportResult")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_STATUS_LIFECYCLE_EVIDENCE
            .contains("submitted, succeeded, or failed")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_STATUS_LIFECYCLE_EVIDENCE.contains("no retry")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_STATUS_LIFECYCLE_EVIDENCE
            .contains("moderation policy lookup")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_STATUS_LIFECYCLE_LABEL
            .contains("ReportContent result")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"message_report_status_lifecycle_surface")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message report status lifecycle"
            && capability
                .base_module
                .contains("TimelineUpdate::MessageReportResult")
            && capability.notes.contains("submitted")
            && capability.notes.contains("succeeded")
            && capability.notes.contains("failed")
            && capability.notes.contains("retry")
            && capability.notes.contains("cancel")
            && capability.notes.contains("moderation policy lookup")
            && capability.notes.contains("ban")
            && capability.notes.contains("kick")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_message_report_status_clipboard_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_REPORT_STATUS_CLIPBOARD_MARKER,
        "hepta_telegram_message_report_status_clipboard_ready"
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_STATUS_CLIPBOARD_EVIDENCE
            .contains("local clipboard")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_STATUS_CLIPBOARD_EVIDENCE.contains("status badge")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_STATUS_CLIPBOARD_EVIDENCE
            .contains("cached event id")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_STATUS_CLIPBOARD_EVIDENCE
            .contains("result/error text")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_STATUS_CLIPBOARD_EVIDENCE
            .contains("no extra MatrixRequest::ReportContent")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_STATUS_CLIPBOARD_EVIDENCE
            .contains("moderation policy lookup")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_STATUS_CLIPBOARD_LABEL
            .contains("local clipboard only")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"message_report_status_clipboard_action")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message report status clipboard action"
            && capability
                .base_module
                .contains("copy_telegram_message_report_status_summary")
            && capability.notes.contains("status badge")
            && capability.notes.contains("cached event id")
            && capability.notes.contains("result/error text")
            && capability.notes.contains("local clipboard")
            && capability
                .notes
                .contains("no extra MatrixRequest::ReportContent")
            && capability.notes.contains("PositiveConfirmationModal")
            && capability.notes.contains("moderation policy lookup")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|runway| {
        runway.gap_id == "message_report_send"
            && runway.current_path.contains("status clipboard")
            && runway.remaining_gap.contains("moderation workflow")
    }));
}

#[test]
fn hepta_telegram_base_message_report_retry_confirmation_is_guarded() {
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_REPORT_RETRY_CONFIRMATION_MARKER,
        "hepta_telegram_message_report_retry_confirmation_ready"
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_RETRY_CONFIRMATION_EVIDENCE
            .contains("cached event id")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_RETRY_CONFIRMATION_EVIDENCE
            .contains("PositiveConfirmationModal")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_RETRY_CONFIRMATION_EVIDENCE
            .contains("MatrixRequest::ReportContent")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_RETRY_CONFIRMATION_EVIDENCE
            .contains("cancel queue")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_RETRY_CONFIRMATION_LABEL.contains("ReportContent")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"message_report_retry_confirmation_guard")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message report retry confirmation"
            && capability
                .base_module
                .contains("MatrixRequest::ReportContent")
            && capability.notes.contains("PositiveConfirmationModal")
            && capability.notes.contains("cached event id")
            && capability.notes.contains("cancel queue")
            && capability.notes.contains("moderation policy lookup")
            && capability.notes.contains("ban")
            && capability.notes.contains("kick")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_message_report_workflow_actions_row_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_REPORT_WORKFLOW_ACTIONS_ROW_MARKER,
        "hepta_telegram_message_report_workflow_actions_row_ready"
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_WORKFLOW_ACTIONS_ROW_EVIDENCE.contains("Queue")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_WORKFLOW_ACTIONS_ROW_EVIDENCE.contains("Policy")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_WORKFLOW_ACTIONS_ROW_EVIDENCE.contains("Assign")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_WORKFLOW_ACTIONS_ROW_EVIDENCE.contains("Appeal")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_WORKFLOW_ACTIONS_ROW_EVIDENCE.contains("Enforce")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_WORKFLOW_ACTIONS_ROW_EVIDENCE.contains("Contract")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_WORKFLOW_ACTIONS_ROW_EVIDENCE.contains("Taxonomy")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_WORKFLOW_ACTIONS_ROW_EVIDENCE
            .contains("does not cancel a moderation queue")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_WORKFLOW_ACTIONS_ROW_EVIDENCE
            .contains("server policy")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_WORKFLOW_ACTIONS_ROW_EVIDENCE.contains("reviewer")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_WORKFLOW_ACTIONS_ROW_EVIDENCE
            .contains("redact/delete")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_WORKFLOW_ACTIONS_ROW_LABEL
            .contains("local blocked report workflow controls")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"message_report_workflow_actions_row"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message report workflow actions row"
            && capability.base_module.contains("report_workflow_actions")
            && capability.notes.contains("Queue")
            && capability.notes.contains("Policy")
            && capability.notes.contains("Assign")
            && capability.notes.contains("Appeal")
            && capability.notes.contains("Enforce")
            && capability.notes.contains("Contract")
            && capability.notes.contains("Taxonomy")
            && capability
                .notes
                .contains("blocked queue/policy/reviewer/evidence/appeal/enforcement result slots")
            && capability.notes.contains("local report status metadata")
            && capability.notes.contains("moderation queue")
            && capability.notes.contains("server policy")
            && capability.notes.contains("reviewer")
            && capability.notes.contains("appeal workflow")
            && capability.notes.contains("redact/delete")
            && capability.notes.contains("ban")
            && capability.notes.contains("kick")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_message_report_moderation_reviewer_packet_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_REPORT_MODERATION_REVIEWER_PACKET_MARKER,
        "hepta_telegram_message_report_moderation_reviewer_packet_ready"
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_report_moderation_reviewer_packet_action")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_MODERATION_REVIEWER_PACKET_EVIDENCE
            .contains("moderation reviewer acceptance matrix")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_MODERATION_REVIEWER_PACKET_EVIDENCE
            .contains("queue persistence")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_MODERATION_REVIEWER_PACKET_EVIDENCE
            .contains("policy lookup")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_MODERATION_REVIEWER_PACKET_EVIDENCE
            .contains("reviewer assignment")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_MODERATION_REVIEWER_PACKET_EVIDENCE
            .contains("appeal workflow")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_MODERATION_REVIEWER_PACKET_EVIDENCE
            .contains("enforcement")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_MODERATION_REVIEWER_PACKET_EVIDENCE
            .contains("gateway/runtime/auth/provider")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_MODERATION_REVIEWER_PACKET_LABEL
            .contains("moderation/reviewer acceptance criteria")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message report moderation reviewer packet"
            && capability
                .base_module
                .contains("copy_telegram_message_report_moderation_reviewer_packet")
            && capability
                .base_module
                .contains("message_report_moderation_reviewer_packet_payload")
            && capability
                .notes
                .contains("cached ReportContent status strip")
            && capability.notes.contains("moderation queue persistence")
            && capability.notes.contains("policy lookup")
            && capability.notes.contains("reviewer assignment")
            && capability.notes.contains("evidence/source retention")
            && capability.notes.contains("reporter and target audit")
            && capability.notes.contains("appeal workflow")
            && capability.notes.contains("enforcement")
            && capability
                .notes
                .contains("no extra MatrixRequest::ReportContent")
            && capability.notes.contains("PositiveConfirmationModal")
            && capability.notes.contains("gateway/runtime/auth/provider")
            && capability.notes.contains("Telegram delivery")
            && capability.notes.contains("live mutation")
            && capability.notes.contains("message_report_send")
    }));
}

#[test]
fn hepta_telegram_base_message_report_workflow_result_contract_packet_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_REPORT_WORKFLOW_RESULT_CONTRACT_PACKET_MARKER,
        "hepta_telegram_message_report_workflow_result_contract_packet_ready"
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_report_workflow_result_contract_packet_action")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_WORKFLOW_RESULT_CONTRACT_PACKET_EVIDENCE
            .contains("typed moderation workflow/result contract packet")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_WORKFLOW_RESULT_CONTRACT_PACKET_EVIDENCE
            .contains("typed queue")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_WORKFLOW_RESULT_CONTRACT_PACKET_EVIDENCE
            .contains("policy")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_WORKFLOW_RESULT_CONTRACT_PACKET_EVIDENCE
            .contains("reviewer assignment")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_WORKFLOW_RESULT_CONTRACT_PACKET_EVIDENCE
            .contains("source-hash")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_WORKFLOW_RESULT_CONTRACT_PACKET_EVIDENCE
            .contains("gateway/runtime/auth/provider")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_WORKFLOW_RESULT_CONTRACT_PACKET_LABEL
            .contains("typed moderation workflow/result contracts")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message report workflow result contract packet"
            && capability
                .base_module
                .contains("copy_telegram_message_report_workflow_result_contract_packet")
            && capability
                .base_module
                .contains("message_report_workflow_result_contract_packet_payload")
            && capability
                .notes
                .contains("typed moderation workflow/result contract packet")
            && capability.notes.contains("source-hash")
            && capability
                .notes
                .contains("no extra MatrixRequest::ReportContent")
            && capability.notes.contains("PositiveConfirmationModal")
            && capability.notes.contains("gateway/runtime/auth/provider")
            && capability.notes.contains("Telegram delivery")
            && capability.notes.contains("message_report_send")
    }));
    let runway = HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY
        .iter()
        .find(|runway| runway.gap_id == "message_report_send")
        .expect("message_report_send runway should exist");
    assert!(
        runway
            .current_path
            .contains("typed moderation workflow/result contract packet")
    );
    assert!(
        runway
            .next_ui_safe_step
            .contains("backend moderation workflow/result contracts")
    );
}

#[test]
fn hepta_telegram_base_message_report_workflow_result_taxonomy_packet_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_REPORT_WORKFLOW_RESULT_TAXONOMY_PACKET_MARKER,
        "hepta_telegram_message_report_workflow_result_taxonomy_packet_ready"
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_report_workflow_result_taxonomy_packet_action")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_WORKFLOW_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("blocked moderation workflow result taxonomy packet")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_WORKFLOW_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("queue")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_WORKFLOW_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("policy")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_WORKFLOW_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("reviewer")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_WORKFLOW_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("source-hash")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_WORKFLOW_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("gateway/runtime/auth/provider")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_WORKFLOW_RESULT_TAXONOMY_PACKET_LABEL
            .contains("blocked moderation workflow result slots")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message report workflow result taxonomy packet"
            && capability
                .base_module
                .contains("copy_telegram_message_report_workflow_result_taxonomy_packet")
            && capability
                .base_module
                .contains("message_report_workflow_result_taxonomy_packet_payload")
            && capability
                .notes
                .contains("blocked moderation workflow result taxonomy packet")
            && capability
                .notes
                .contains("MatrixRequest::ReportContent send/result/retry")
            && capability
                .notes
                .contains("loaded-or-source-fetch EventSourceModal")
            && capability.notes.contains("source-hash")
            && capability
                .notes
                .contains("no extra MatrixRequest::ReportContent")
            && capability.notes.contains("PositiveConfirmationModal")
            && capability.notes.contains("gateway/runtime/auth/provider")
            && capability.notes.contains("Telegram delivery")
            && capability.notes.contains("message_report_send")
    }));
    let runway = HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY
        .iter()
        .find(|runway| runway.gap_id == "message_report_send")
        .expect("message_report_send runway should exist");
    assert!(
        runway
            .current_path
            .contains("workflow result taxonomy packet")
    );
    assert!(
        runway
            .next_ui_safe_step
            .contains("backend moderation workflow/result contracts")
    );
}

#[test]
fn hepta_telegram_base_message_report_preflight_detail_controls_are_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_REPORT_PREFLIGHT_DETAIL_CONTROLS_MARKER,
        "hepta_telegram_message_report_preflight_detail_controls_ready"
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_report_preflight_detail_controls_row")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
            .contains("Request, Result, Error, Retry, and Source")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
            .contains("Request, Result, Error, and Retry only update local")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
            .contains("Source is a real loaded-or-source-fetch modal handoff")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
            .contains("EventSourceModal")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
            .contains("MatrixRequest::FetchEventSource")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
            .contains("no extra MatrixRequest::ReportContent")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
            .contains("PositiveConfirmationModal")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
            .contains("cancel queue")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
            .contains("moderation policy lookup")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_PREFLIGHT_DETAIL_CONTROLS_LABEL
            .contains("stay local")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message report preflight detail controls row"
            && capability
                .base_module
                .contains("report_preflight_detail_controls")
            && capability
                .notes
                .contains("Request, Result, Error, Retry, and Source")
            && capability.notes.contains("status cache")
            && capability
                .notes
                .contains("loaded-or-source-fetch modal handoff")
            && capability.notes.contains("MatrixRequest::FetchEventSource")
            && capability
                .notes
                .contains("no extra MatrixRequest::ReportContent")
            && capability.notes.contains("PositiveConfirmationModal")
            && capability.notes.contains("cancel queue")
            && capability.notes.contains("moderation policy lookup")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("write-side live mutation")
    }));
}

#[test]
fn hepta_telegram_base_message_report_loaded_source_modal_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_REPORT_LOADED_SOURCE_MODAL_MARKER,
        "hepta_telegram_message_report_loaded_source_modal_ready"
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"message_report_loaded_source_modal_action")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_LOADED_SOURCE_MODAL_EVIDENCE
            .contains("real loaded-or-source-fetch modal handoff")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_LOADED_SOURCE_MODAL_EVIDENCE
            .contains("cached reported event id")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_LOADED_SOURCE_MODAL_EVIDENCE
            .contains("already loaded RoomScreen timeline row")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_LOADED_SOURCE_MODAL_EVIDENCE
            .contains("EventTimelineItem.latest_json")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_LOADED_SOURCE_MODAL_EVIDENCE
            .contains("MatrixRequest::FetchEventSource")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_LOADED_SOURCE_MODAL_EVIDENCE
            .contains("Room::load_or_fetch_event")
    );
    assert!(
        crate::home::room_screen::MESSAGE_REPORT_LOADED_SOURCE_MODAL_LABEL
            .contains("source-only current-room JSON")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message report loaded source modal action"
            && capability
                .base_module
                .contains("open_telegram_message_report_loaded_source")
            && capability
                .notes
                .contains("real loaded-or-source-fetch modal handoff")
            && capability.notes.contains("cached reported event id")
            && capability.notes.contains("EventSourceModal")
            && capability.notes.contains("EventTimelineItem.latest_json")
            && capability.notes.contains("MatrixRequest::FetchEventSource")
            && capability.notes.contains("Room::load_or_fetch_event")
            && capability.notes.contains("event-context fetch")
            && capability.notes.contains("extra ReportContent")
            && capability.notes.contains("moderation workflow")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("write-side live mutation")
    }));
}

#[test]
fn hepta_telegram_base_message_report_custom_reason_is_confirmation_guarded() {
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_REPORT_CUSTOM_REASON_CONFIRMATION_MARKER,
        "hepta_telegram_message_report_custom_reason_confirmation_ready"
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_report_custom_reason_confirmation_guard")
    );
    assert!(
        crate::home::new_message_context_menu::MESSAGE_REPORT_SEND_LOCAL_BOUNDARY_EVIDENCE
            .contains("Spam, Abuse, and Custom reason require confirmation")
    );
    assert!(
        crate::home::new_message_context_menu::MESSAGE_REPORT_STAGING_COMPACT_LABEL
            .contains("empty custom reason stays local")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message report send local boundary evidence"
            && capability.notes.contains("empty custom reason")
            && capability.notes.contains("Spam/Abuse/Custom confirmation")
            && capability.notes.contains("Room::report_content")
    }));
}

#[test]
fn hepta_telegram_base_message_edit_history_surface_has_summary_read() {
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_EDIT_HISTORY_LOCAL_SURFACE_MARKER,
        "hepta_telegram_message_edit_history_local_surface_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_EDIT_HISTORY_CLICK_LOCAL_MARKER,
        "hepta_telegram_message_edit_history_click_local_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_EDIT_HISTORY_LOADED_PREVIEW_MARKER,
        "hepta_telegram_message_edit_history_loaded_preview_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"message_edit_history"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"message_edit_history_surface"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"message_edit_history_click_local_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_edit_history_loaded_original_preview")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_edit_history_local_boundary_evidence")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message edit history"
            && capability.notes.contains("MatrixRequest::FetchEditHistory")
            && capability
                .notes
                .contains("already loaded original plaintext preview")
            && capability.notes.contains("full history modal UI")
    }));
}

#[test]
fn hepta_telegram_base_message_edit_history_click_reads_replace_summary() {
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_EDIT_HISTORY_CLICK_LOCAL_MARKER,
        "hepta_telegram_message_edit_history_click_local_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"message_edit_history"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"message_edit_history_click_local_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_edit_history_loaded_original_preview")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message edit history click summary read"
            && capability.notes.contains("MatrixRequest::FetchEditHistory")
            && capability.notes.contains("m.replace relations")
            && capability
                .notes
                .contains("loaded original plaintext preview")
            && capability.notes.contains("local diff hint")
            && capability.notes.contains("timeline reload")
            && capability.notes.contains("event-context fetch")
            && capability.notes.contains("message send, edit, redact")
            && capability.notes.contains("room-state request")
            && capability
                .notes
                .contains("full edit history UI remains TODO")
    }));
}

#[test]
fn hepta_telegram_base_message_edit_history_compact_summary_live_read_wiring_is_partial_live() {
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_EDIT_HISTORY_COMPACT_SUMMARY_LIVE_READ_WIRING_MARKER,
        "hepta_telegram_message_edit_history_compact_summary_live_read_wiring_ready"
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_edit_history_compact_summary_live_read_wiring")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message edit history compact summary live read wiring"
            && capability.notes.contains("MatrixRequest::FetchEditHistory")
            && capability.notes.contains("Room::relations")
            && capability.notes.contains("RelationType::Replacement")
            && capability
                .notes
                .contains("TimelineUpdate::EditHistoryFetched")
            && capability.notes.contains("PositiveConfirmationModal")
            && capability.notes.contains("remote full history modal")
            && capability.notes.contains("event-context fetch")
            && capability
                .notes
                .contains("relation pages fetched/exhausted state")
            && capability.notes.contains("live mutation outside")
    }));
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|gap| {
        gap.gap_id == "message_edit_history"
            && gap.current_path.contains("live paginated FetchEditHistory")
            && gap.current_path.contains("read/result wiring")
            && gap
                .current_path
                .contains("relation pages/exhausted metadata")
            && gap.current_path.contains("confirmed failed-state Retry")
            && gap
                .remaining_gap
                .contains("remote/server-backed full history result adapter")
            && gap.current_path.contains("MatrixRequest::FetchEventSource")
            && gap.remaining_gap.contains("source reconciliation")
    }));
}

#[test]
fn hepta_telegram_base_message_edit_history_loaded_target_metadata_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_EDIT_HISTORY_LOADED_TARGET_METADATA_MARKER,
        "hepta_telegram_message_edit_history_loaded_target_metadata_ready"
    );
    assert!(
        crate::home::edited_indicator::MESSAGE_EDIT_HISTORY_LOADED_TARGET_METADATA_EVIDENCE
            .contains("loaded edit-history target metadata")
    );
    assert!(
        crate::home::edited_indicator::MESSAGE_EDIT_HISTORY_LOADED_TARGET_METADATA_EVIDENCE
            .contains("already loaded timeline row")
    );
    assert!(
        crate::home::edited_indicator::MESSAGE_EDIT_HISTORY_LOADED_TARGET_METADATA_EVIDENCE
            .contains("loaded original plaintext preview")
    );
    assert!(
        crate::home::edited_indicator::MESSAGE_EDIT_HISTORY_LOADED_TARGET_METADATA_EVIDENCE
            .contains("character count")
    );
    assert!(
        crate::home::edited_indicator::MESSAGE_EDIT_HISTORY_LOADED_TARGET_METADATA_EVIDENCE
            .contains("byte count")
    );
    assert!(
        crate::home::edited_indicator::MESSAGE_EDIT_HISTORY_LOADED_TARGET_METADATA_EVIDENCE
            .contains("latest edit timestamp availability")
    );
    assert!(
        crate::home::edited_indicator::MESSAGE_EDIT_HISTORY_LOADED_TARGET_METADATA_EVIDENCE
            .contains("MatrixRequest::FetchEditHistory")
    );
    assert!(
        crate::home::edited_indicator::MESSAGE_EDIT_HISTORY_LOADED_TARGET_METADATA_LABEL
            .contains("no event context, reload, or mutation")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_edit_history_loaded_target_metadata_preview")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message edit history loaded target metadata preview"
            && capability
                .base_module
                .contains("EditedIndicator loaded target metadata")
            && capability.notes.contains("loaded event-id availability")
            && capability
                .notes
                .contains("loaded original plaintext preview")
            && capability.notes.contains("character count")
            && capability.notes.contains("byte count")
            && capability
                .notes
                .contains("latest edit timestamp availability")
            && capability.notes.contains("MatrixRequest::FetchEditHistory")
            && capability.notes.contains("event-context fetch")
            && capability.notes.contains("timeline pagination/reload")
            && capability.notes.contains("event source open")
            && capability
                .notes
                .contains("remote full-history modal request")
            && capability.notes.contains("full diff rendering")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_message_edit_history_detail_surface_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_EDIT_HISTORY_DETAIL_SURFACE_MARKER,
        "hepta_telegram_message_edit_history_detail_surface_ready"
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_DETAIL_SURFACE_EVIDENCE
            .contains("Telegram edit-history detail strip")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_DETAIL_SURFACE_EVIDENCE
            .contains("MatrixRequest::FetchEditHistory")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_DETAIL_SURFACE_EVIDENCE
            .contains("replacement count")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_DETAIL_SURFACE_EVIDENCE
            .contains("local preview-diff hint")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_DETAIL_SURFACE_EVIDENCE
            .contains("event-context fetch")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_DETAIL_SURFACE_EVIDENCE
            .contains("timeline pagination/reload")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_DETAIL_SURFACE_EVIDENCE
            .contains("gateway/runtime/auth")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_DETAIL_SURFACE_EVIDENCE
            .contains("live mutation")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"message_edit_history_detail_surface"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message edit history detail surface"
            && capability
                .base_module
                .contains("telegram_message_edit_history_strip")
            && capability.notes.contains("replacement count")
            && capability
                .notes
                .contains("latest replacement event/timestamp")
            && capability.notes.contains("loaded original preview")
            && capability.notes.contains("local preview-diff hint")
            && capability.notes.contains("MatrixRequest::FetchEditHistory")
            && capability
                .notes
                .contains("remote full-history modal request")
            && capability.notes.contains("event-context fetch")
            && capability.notes.contains("timeline pagination/reload")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_message_edit_history_full_modal_boundary_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_EDIT_HISTORY_FULL_MODAL_BOUNDARY_MARKER,
        "hepta_telegram_message_edit_history_full_modal_boundary_ready"
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_FULL_MODAL_BOUNDARY_EVIDENCE
            .contains("paginated m.replace read state")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_FULL_MODAL_BOUNDARY_EVIDENCE
            .contains("synthetic full snapshot JSON")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_FULL_MODAL_BOUNDARY_EVIDENCE
            .contains("Remote/server-backed Full history modal UI")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_FULL_MODAL_BOUNDARY_EVIDENCE
            .contains("side-by-side full diff rendering")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_FULL_MODAL_BOUNDARY_EVIDENCE
            .contains("event-context fetch")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_FULL_MODAL_BOUNDARY_EVIDENCE
            .contains("timeline pagination/reload")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_FULL_MODAL_BOUNDARY_EVIDENCE
            .contains("relation pages fetched/exhausted state")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_FULL_MODAL_BOUNDARY_EVIDENCE
            .contains("MatrixRequest::FetchEditHistory")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_FULL_MODAL_BOUNDARY_LABEL
            .contains("complete m.replace pagination is live")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"message_edit_history_full_modal_boundary")
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"message_edit_history"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message edit history full modal boundary"
            && capability
                .base_module
                .contains("edit_history_full_modal_boundary")
            && capability.notes.contains("paginated m.replace read state")
            && capability.notes.contains("synthetic full snapshot JSON")
            && capability
                .notes
                .contains("Remote/server-backed Full history modal UI")
            && capability
                .notes
                .contains("side-by-side full diff rendering")
            && capability.notes.contains("event-context fetch")
            && capability.notes.contains("timeline pagination/reload")
            && capability
                .notes
                .contains("relation pages fetched/exhausted state")
            && capability.notes.contains("MatrixRequest::FetchEditHistory")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_message_edit_history_local_full_snapshot_modal_is_live_local() {
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_EDIT_HISTORY_LOCAL_FULL_SNAPSHOT_MODAL_MARKER,
        "hepta_telegram_message_edit_history_local_full_snapshot_modal_ready"
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_LOCAL_FULL_SNAPSHOT_MODAL_EVIDENCE
            .contains("local synthetic EventSourceModal snapshot")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_LOCAL_FULL_SNAPSHOT_MODAL_EVIDENCE
            .contains("cached MatrixRequest::FetchEditHistory state")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_LOCAL_FULL_SNAPSHOT_MODAL_EVIDENCE
            .contains("no extra MatrixRequest::FetchEditHistory")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_edit_history_local_full_snapshot_modal_action")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message edit history local full snapshot modal"
            && capability
                .base_module
                .contains("open_telegram_message_edit_history_local_full_snapshot_modal")
            && capability.notes.contains("synthetic JSON snapshot")
            && capability
                .notes
                .contains("cached MatrixRequest::FetchEditHistory state")
            && capability.notes.contains("EventSourceModal")
            && capability
                .notes
                .contains("no extra MatrixRequest::FetchEditHistory")
            && capability
                .notes
                .contains("no remote full-history modal request")
            && capability.notes.contains("event-context fetch")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("write-side live mutation")
    }));
}

#[test]
fn hepta_telegram_base_message_edit_history_full_controls_are_visible_local_buttons() {
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_EDIT_HISTORY_FULL_CONTROLS_MARKER,
        "hepta_telegram_message_edit_history_full_controls_ready"
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_FULL_CONTROLS_EVIDENCE
            .contains("Full, Diff, Context, Source, Packet, Contract, and Taxonomy")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_FULL_CONTROLS_EVIDENCE
            .contains("Full opens the existing local EventSourceModal")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_FULL_CONTROLS_EVIDENCE
            .contains("real loaded side-by-side preview diff handoff")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_FULL_CONTROLS_EVIDENCE
            .contains("real loaded edit-source modal handoff")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_FULL_CONTROLS_EVIDENCE
            .contains("event-context fetch")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_FULL_CONTROLS_EVIDENCE
            .contains("MatrixRequest::FetchEditHistory")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_FULL_CONTROLS_LABEL
            .contains("Diff opens a loaded side-by-side preview diff modal")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_FULL_CONTROLS_LABEL
            .contains("Source opens cached latest replacement JSON or loaded original JSON")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_FULL_CONTROLS_LABEL
            .contains("Packet copies acceptance contract")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_FULL_CONTROLS_LABEL
            .contains("Contract maps typed full-history result contracts")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_FULL_CONTROLS_LABEL
            .contains("Taxonomy copies remote full-history/source result slots")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"message_edit_history_full_controls_row")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message edit history full controls row"
            && capability
                .base_module
                .contains("edit_history_full_controls")
            && capability.notes.contains("Full opens")
            && capability.notes.contains("Context updates only")
            && capability
                .notes
                .contains("Full, Diff, Context, Source, Packet, Contract, and Taxonomy")
            && capability
                .notes
                .contains("loaded side-by-side preview diff")
            && capability
                .notes
                .contains("real loaded edit-source modal handoff")
            && capability.notes.contains("MatrixRequest::FetchEventSource")
            && capability.notes.contains("Room::load_or_fetch_event")
            && capability.notes.contains("local clipboard")
            && capability.notes.contains("EventSourceModal")
            && capability
                .notes
                .contains("loaded/full diff remote modal acceptance contract")
            && capability.notes.contains("typed full-history modal/result")
            && capability
                .notes
                .contains("source reconciliation, server-backed diff")
            && capability.notes.contains("event-context fetch")
            && capability.notes.contains("MatrixRequest::FetchEditHistory")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_message_edit_history_loaded_source_modal_reuses_loaded_event_source() {
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_EDIT_HISTORY_LOADED_SOURCE_MODAL_MARKER,
        "hepta_telegram_message_edit_history_loaded_source_modal_ready"
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_LOADED_SOURCE_MODAL_EVIDENCE
            .contains("real loaded edit-source modal handoff")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_LOADED_SOURCE_MODAL_EVIDENCE
            .contains("EventSourceModal")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_LOADED_SOURCE_MODAL_EVIDENCE
            .contains("MatrixRequest::FetchEventSource")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_LOADED_SOURCE_MODAL_EVIDENCE
            .contains("Room::load_or_fetch_event")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_LOADED_SOURCE_MODAL_EVIDENCE
            .contains("cached raw JSON")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_LOADED_SOURCE_MODAL_EVIDENCE
            .contains("EventTimelineItem.latest_json")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_LOADED_SOURCE_MODAL_EVIDENCE
            .contains("missing latest_json")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_LOADED_SOURCE_MODAL_EVIDENCE
            .contains("already loaded original edited event row")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_LOADED_SOURCE_MODAL_EVIDENCE
            .contains("paginated edit-history latest replacement event id")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_LOADED_SOURCE_MODAL_LABEL
            .contains("latest replacement JSON")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_edit_history_loaded_source_modal_action")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message edit history loaded edit source modal"
            && capability
                .base_module
                .contains("open_telegram_message_edit_history_loaded_source")
            && capability.notes.contains("EventSourceModal")
            && capability
                .notes
                .contains("real loaded edit-source modal handoff")
            && capability
                .notes
                .contains("cached latest replacement raw JSON")
            && capability
                .notes
                .contains("loaded EventTimelineItem.latest_json")
            && capability.notes.contains("Missing event id")
            && capability.notes.contains("MatrixRequest::FetchEventSource")
            && capability.notes.contains("Room::load_or_fetch_event")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("write-side live mutation")
    }));
}

#[test]
fn hepta_telegram_base_message_edit_history_loaded_diff_detail_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_EDIT_HISTORY_LOADED_DIFF_DETAIL_MARKER,
        "hepta_telegram_message_edit_history_loaded_diff_detail_ready"
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_LOADED_DIFF_DETAIL_EVIDENCE
            .contains("MatrixRequest::FetchEditHistory")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_LOADED_DIFF_DETAIL_EVIDENCE
            .contains("already loaded original timeline row")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_LOADED_DIFF_DETAIL_EVIDENCE
            .contains("Full/Diff/Context/Source")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_LOADED_DIFF_DETAIL_EVIDENCE
            .contains("local clipboard")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_LOADED_DIFF_DETAIL_EVIDENCE
            .contains("event-context fetch")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_LOADED_DIFF_DETAIL_LABEL
            .contains("Loaded diff detail is local")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_edit_history_loaded_diff_detail_state")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message edit history loaded diff detail state"
            && capability
                .base_module
                .contains("edit_history_loaded_diff_detail")
            && capability.notes.contains("MatrixRequest::FetchEditHistory")
            && capability.notes.contains("Full/Diff/Context/Source")
            && capability.notes.contains("replacement count")
            && capability.notes.contains("local clipboard")
            && capability.notes.contains("event-context fetch")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_message_edit_history_loaded_diff_clipboard_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_EDIT_HISTORY_LOADED_DIFF_CLIPBOARD_MARKER,
        "hepta_telegram_message_edit_history_loaded_diff_clipboard_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_EDIT_HISTORY_LOADED_SIDE_BY_SIDE_DIFF_MODAL_MARKER,
        "hepta_telegram_message_edit_history_loaded_side_by_side_diff_modal_ready"
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_LOADED_DIFF_CLIPBOARD_EVIDENCE
            .contains("real loaded side-by-side preview diff modal")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_LOADED_SIDE_BY_SIDE_DIFF_MODAL_EVIDENCE
            .contains("read-only loaded side-by-side preview diff snapshot")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_LOADED_DIFF_CLIPBOARD_EVIDENCE
            .contains("local clipboard")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_LOADED_DIFF_CLIPBOARD_EVIDENCE
            .contains("loaded original/latest preview")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_LOADED_DIFF_CLIPBOARD_EVIDENCE
            .contains("server-backed side-by-side full diff rendering")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_LOADED_DIFF_CLIPBOARD_EVIDENCE
            .contains("event-context fetch")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_LOADED_DIFF_CLIPBOARD_LABEL
            .contains("local clipboard")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_edit_history_loaded_diff_clipboard_action")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_edit_history_loaded_side_by_side_diff_modal_action")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message edit history loaded side-by-side diff modal action"
            && capability
                .base_module
                .contains("copy_telegram_message_edit_history_loaded_diff")
            && capability
                .notes
                .contains("real loaded side-by-side preview diff modal")
            && capability.notes.contains("local clipboard")
            && capability.notes.contains("loaded original preview rows")
            && capability.notes.contains("loaded preview data exists")
            && capability.notes.contains("event-context fetch")
            && capability.notes.contains("replacement event source fetch")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_message_edit_history_full_diff_packet_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_EDIT_HISTORY_FULL_DIFF_PACKET_MARKER,
        "hepta_telegram_message_edit_history_full_diff_packet_ready"
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_edit_history_full_diff_packet_action")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_FULL_DIFF_PACKET_EVIDENCE
            .contains("loaded/full diff remote modal acceptance contract")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_FULL_DIFF_PACKET_EVIDENCE
            .contains("remote full-history modal")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_FULL_DIFF_PACKET_EVIDENCE
            .contains("relation pages fetched/exhausted state")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_FULL_DIFF_PACKET_EVIDENCE
            .contains("side-by-side full diff rendering")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_FULL_DIFF_PACKET_EVIDENCE
            .contains("event context")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_FULL_DIFF_PACKET_EVIDENCE
            .contains("replacement event source")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_FULL_DIFF_PACKET_EVIDENCE
            .contains("gateway/runtime/auth/provider")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_FULL_DIFF_PACKET_LABEL
            .contains("loaded/full diff remote modal acceptance")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message edit history loaded/full diff packet"
            && capability
                .base_module
                .contains("copy_telegram_message_edit_history_full_diff_packet")
            && capability
                .base_module
                .contains("edit_history_full_diff_packet_payload")
            && capability.notes.contains("acceptance criteria")
            && capability
                .notes
                .contains("no extra MatrixRequest::FetchEditHistory")
            && capability.notes.contains("PositiveConfirmationModal")
            && capability.notes.contains("gateway/runtime/auth/provider")
            && capability.notes.contains("Telegram delivery")
            && capability.notes.contains("live mutation")
            && capability.notes.contains("message_edit_history")
    }));
}

#[test]
fn hepta_telegram_base_message_edit_history_full_history_result_contract_packet_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_EDIT_HISTORY_FULL_HISTORY_RESULT_CONTRACT_PACKET_MARKER,
        "hepta_telegram_message_edit_history_full_history_result_contract_packet_ready"
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_edit_history_full_history_result_contract_packet_action")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_FULL_HISTORY_RESULT_CONTRACT_PACKET_EVIDENCE
            .contains("typed full-history modal/result contract packet")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_FULL_HISTORY_RESULT_CONTRACT_PACKET_EVIDENCE
            .contains("source-hash")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_FULL_HISTORY_RESULT_CONTRACT_PACKET_EVIDENCE
            .contains("stale target")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_FULL_HISTORY_RESULT_CONTRACT_PACKET_EVIDENCE
            .contains("gateway/runtime/auth/provider")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_FULL_HISTORY_RESULT_CONTRACT_PACKET_LABEL
            .contains("typed full-history modal/result contracts")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message edit history full-history result contract packet"
            && capability
                .base_module
                .contains("copy_telegram_message_edit_history_full_history_result_contract_packet")
            && capability
                .base_module
                .contains("edit_history_full_history_result_contract_packet_payload")
            && capability
                .notes
                .contains("typed request/result/error/retry/source")
            && capability
                .notes
                .contains("no extra MatrixRequest::FetchEditHistory")
            && capability.notes.contains("PositiveConfirmationModal")
            && capability.notes.contains("gateway/runtime/auth/provider")
            && capability.notes.contains("Telegram delivery")
            && capability.notes.contains("live mutation")
            && capability.notes.contains("message_edit_history")
    }));
}

#[test]
fn hepta_telegram_base_message_edit_history_remote_result_taxonomy_packet_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_EDIT_HISTORY_REMOTE_RESULT_TAXONOMY_PACKET_MARKER,
        "hepta_telegram_message_edit_history_remote_result_taxonomy_packet_ready"
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_edit_history_remote_result_taxonomy_packet_action")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_REMOTE_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("remote full-history/source reconciliation result taxonomy")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_REMOTE_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("MatrixRequest::FetchEditHistory")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_REMOTE_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("MatrixRequest::FetchEventSource")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_REMOTE_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("not-assigned/not-wired")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_REMOTE_RESULT_TAXONOMY_PACKET_EVIDENCE
            .contains("gateway/runtime/auth/provider")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_REMOTE_RESULT_TAXONOMY_PACKET_LABEL
            .contains("remote full-history/source reconciliation result slots")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message edit history remote result taxonomy packet"
            && capability
                .base_module
                .contains("copy_telegram_message_edit_history_remote_result_taxonomy_packet")
            && capability
                .base_module
                .contains("edit_history_remote_result_taxonomy_packet_payload")
            && capability.notes.contains("remote_full_history_request_id")
            && capability
                .notes
                .contains("server_backed_full_diff_operation_id")
            && capability
                .notes
                .contains("replacement_source_reconciliation_operation_id")
            && capability.notes.contains("source-hash policy")
            && capability
                .notes
                .contains("no extra MatrixRequest::FetchEditHistory")
            && capability.notes.contains("gateway/runtime/auth/provider")
            && capability.notes.contains("Telegram delivery")
            && capability.notes.contains("live mutation")
            && capability.notes.contains("message_edit_history")
    }));
}

#[test]
fn hepta_telegram_base_message_edit_history_preflight_detail_controls_are_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_EDIT_HISTORY_PREFLIGHT_DETAIL_CONTROLS_MARKER,
        "hepta_telegram_message_edit_history_preflight_detail_controls_ready"
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
            .contains("Request, Result, Error, Retry, and Source")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
            .contains("MatrixRequest::FetchEditHistory")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
            .contains("PositiveConfirmationModal")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
            .contains("event-context fetch")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_PREFLIGHT_DETAIL_CONTROLS_LABEL
            .contains("stay local")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_edit_history_preflight_detail_controls_row")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message edit history preflight detail controls row"
            && capability
                .base_module
                .contains("edit_history_preflight_controls")
            && capability
                .notes
                .contains("Request, Result, Error, Retry, and Source")
            && capability
                .notes
                .contains("no extra MatrixRequest::FetchEditHistory")
            && capability.notes.contains("PositiveConfirmationModal")
            && capability.notes.contains("event-context fetch")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_message_edit_history_retry_confirmation_is_guarded() {
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_EDIT_HISTORY_RETRY_CONFIRMATION_MARKER,
        "hepta_telegram_message_edit_history_retry_confirmation_ready"
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_RETRY_CONFIRMATION_EVIDENCE
            .contains("cached event id")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_RETRY_CONFIRMATION_EVIDENCE
            .contains("TimelineKind")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_RETRY_CONFIRMATION_EVIDENCE
            .contains("PositiveConfirmationModal")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_RETRY_CONFIRMATION_EVIDENCE
            .contains("MatrixRequest::FetchEditHistory")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_RETRY_CONFIRMATION_EVIDENCE
            .contains("event-context fetch")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_RETRY_CONFIRMATION_EVIDENCE
            .contains("timeline pagination/reload")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_RETRY_CONFIRMATION_EVIDENCE
            .contains("gateway/runtime/auth")
    );
    assert!(
        crate::home::room_screen::MESSAGE_EDIT_HISTORY_RETRY_CONFIRMATION_LABEL
            .contains("FetchEditHistory")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_edit_history_retry_confirmation_guard")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message edit history retry confirmation"
            && capability
                .notes
                .contains("TimelineUpdate::EditHistoryFetched")
            && capability.notes.contains("cached the event id")
            && capability.notes.contains("TimelineKind")
            && capability.notes.contains("PositiveConfirmationModal")
            && capability.notes.contains("MatrixRequest::FetchEditHistory")
            && capability
                .notes
                .contains("remote full-history modal request")
            && capability.notes.contains("event-context fetch")
            && capability.notes.contains("timeline pagination/reload")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation request")
    }));
}

#[test]
fn hepta_telegram_base_message_edit_history_boundary_keeps_remaining_gaps() {
    assert_eq!(
        HEPTA_TELEGRAM_MESSAGE_EDIT_HISTORY_LOCAL_BOUNDARY_MARKER,
        "hepta_telegram_message_edit_history_local_boundary_ready"
    );
    assert!(
        crate::home::edited_indicator::MESSAGE_EDIT_HISTORY_LOCAL_BOUNDARY_EVIDENCE
            .contains("Matrix m.replace relations read")
    );
    assert!(
        crate::home::edited_indicator::MESSAGE_EDIT_HISTORY_LOCAL_BOUNDARY_EVIDENCE
            .contains("already loaded latest edit timestamp")
    );
    assert!(
        crate::home::edited_indicator::MESSAGE_EDIT_HISTORY_LOCAL_BOUNDARY_EVIDENCE
            .contains("already loaded original plaintext preview")
    );
    assert!(
        crate::home::edited_indicator::MESSAGE_EDIT_HISTORY_LOCAL_BOUNDARY_EVIDENCE
            .contains("MatrixRequest::FetchEditHistory")
    );
    assert!(
        crate::home::edited_indicator::MESSAGE_EDIT_HISTORY_LOCAL_BOUNDARY_LABEL
            .contains("loaded original preview")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_edit_history_local_boundary_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"message_edit_history_loaded_original_preview")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "message edit history local boundary evidence"
            && capability.base_module.contains("EditedIndicator")
            && capability
                .notes
                .contains("already loaded latest edit timestamp")
            && capability
                .notes
                .contains("already loaded original plaintext preview")
            && capability
                .notes
                .contains("cached latest replacement raw JSON")
            && capability.notes.contains("local diff/source evidence")
            && capability.notes.contains("message_edit_history")
            && capability.notes.contains("MatrixRequest::FetchEditHistory")
            && capability
                .notes
                .contains("complete paginated m.replace relations")
            && capability.notes.contains("event-context fetch")
            && capability.notes.contains("timeline pagination/reload")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_tsp_identity_surface_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_TSP_IDENTITY_LOCAL_SURFACE_MARKER,
        "hepta_telegram_tsp_identity_local_surface_ready"
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"tsp_identity_preview_surface"));
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"tsp_identity_preview_surface"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "TSP identity indicator"
            && capability.notes.contains("local-only identity preview")
            && capability.notes.contains("without TSP profile lookup")
            && capability.notes.contains("DID resolution")
            && capability.notes.contains("Matrix request")
    }));
}

#[test]
fn hepta_telegram_base_tsp_wallet_pending_cancel_is_local_evidence() {
    assert_eq!(
        HEPTA_TELEGRAM_TSP_WALLET_PENDING_CANCEL_LOCAL_MARKER,
        "hepta_telegram_tsp_wallet_pending_cancel_local_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_TSP_PENDING_CANCEL_OPERATION_PACKET_MARKER,
        "hepta_telegram_tsp_pending_cancel_operation_packet_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_TSP_WALLET_OPEN_RETRY_MARKER,
        "hepta_telegram_tsp_wallet_open_retry_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_TSP_WALLET_SET_DEFAULT_CONFIRMATION_METADATA_MARKER,
        "hepta_telegram_tsp_wallet_set_default_confirmation_metadata_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_TSP_WALLET_REMOVE_CONFIRMATION_METADATA_MARKER,
        "hepta_telegram_tsp_wallet_remove_confirmation_metadata_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_TSP_WALLET_DELETE_BLOCKED_METADATA_MARKER,
        "hepta_telegram_tsp_wallet_delete_blocked_metadata_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_TSP_WALLET_DELETE_PREFLIGHT_RESULT_PACKET_MARKER,
        "hepta_telegram_tsp_wallet_delete_preflight_result_packet_ready"
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"tsp_wallet_pending_cancel_local_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"tsp_pending_cancel_operation_packet_preview")
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"tsp_wallet_open_retry_evidence"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"tsp_wallet_set_default_confirmation_metadata_preview")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"tsp_wallet_remove_confirmation_metadata_preview")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"tsp_wallet_delete_blocked_metadata_preview")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"tsp_wallet_delete_preflight_result_packet_preview")
    );
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"tsp_wallet_pending_cancel"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "TSP wallet pending cancel local evidence"
            && capability.notes.contains("loaded wallet name")
            && capability.notes.contains("URL/path availability")
            && capability.notes.contains("default-wallet metadata")
            && capability.notes.contains("no TspRequest::DeleteWallet")
            && capability.notes.contains("filesystem delete")
            && capability
                .notes
                .contains("TspRequest cancellation is not wired")
            && capability.notes.contains("no cancel request is sent")
            && capability
                .notes
                .contains("operation_id missing_backend_contract")
            && capability
                .notes
                .contains("cancel_state disabled_no_request")
            && capability
                .notes
                .contains("stale_result_policy backend_operation_id_required")
    }));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "TSP wallet delete preflight/result packet"
            && capability.notes.contains("wallet_identity")
            && capability
                .notes
                .contains("path_validation_slot backend_required")
            && capability
                .notes
                .contains("ownership_scope backend_required")
            && capability.notes.contains("open_wallet_closure_slot")
            && capability.notes.contains("default_fallback_slot")
            && capability
                .notes
                .contains("persistence_result_slot not_started")
            && capability.notes.contains("filesystem_result_taxonomy")
            && capability.notes.contains("permission_denied")
            && capability.notes.contains("partial_failure")
            && capability.notes.contains("retry_cancel_policy")
            && capability.notes.contains("audit_redaction_policy")
            && capability.notes.contains("no TspRequest::DeleteWallet")
            && capability.notes.contains("TSP state mutation")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "TSP pending cancel operation packet"
            && capability
                .notes
                .contains("CreateWalletModal and CreateDidModal pending states")
            && capability
                .notes
                .contains("operation_id missing_backend_contract")
            && capability.notes.contains("local_operation_key")
            && capability
                .notes
                .contains("cancel_state disabled_no_request")
            && capability
                .notes
                .contains("stale_result_policy backend_operation_id_required")
            && capability.notes.contains("password_redacted")
            && capability.notes.contains("secret_redacted")
            && capability.notes.contains("no TspRequest cancel")
            && capability.notes.contains("DID rollback")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "TSP wallet open retry"
            && capability
                .notes
                .contains("Open Wallet only for loaded NotFound wallet rows")
            && capability.notes.contains("TspRequest::OpenWallet")
            && capability.notes.contains("known wallet name/path metadata")
            && capability
                .notes
                .contains("Opened rows show local already-open metadata")
            && capability.notes.contains("no file picker")
            && capability.notes.contains("Import Existing Wallet")
            && capability.notes.contains("SetDefaultWallet")
            && capability.notes.contains("TspRequest::DeleteWallet")
            && capability.notes.contains("runtime mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "TSP wallet set default confirmation metadata"
            && capability.notes.contains("loaded wallet metadata")
            && capability.notes.contains("wallet name")
            && capability.notes.contains("URL/path availability")
            && capability.notes.contains("default-wallet state")
            && capability.notes.contains("PositiveConfirmationModal")
            && capability.notes.contains("TspRequest::SetDefaultWallet")
            && capability
                .notes
                .contains("active/default wallet switch path")
            && capability
                .notes
                .contains("confirmation cancel sends no SetDefaultWallet")
            && capability.notes.contains("TspRequest::DeleteWallet")
            && capability.notes.contains("filesystem delete")
            && capability.notes.contains("wallet database delete")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "TSP wallet remove confirmation metadata"
            && capability.notes.contains("loaded wallet metadata")
            && capability.notes.contains("wallet name")
            && capability.notes.contains("URL/path availability")
            && capability.notes.contains("default-wallet state")
            && capability.notes.contains("TspRequest::RemoveWallet")
            && capability.notes.contains("list/default-slot path")
            && capability.notes.contains("TspRequest::DeleteWallet")
            && capability.notes.contains("filesystem delete")
            && capability.notes.contains("wallet database delete")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_tsp_wallet_import_is_blocked_local_evidence() {
    assert_eq!(
        HEPTA_TELEGRAM_TSP_WALLET_IMPORT_LOCAL_MARKER,
        "hepta_telegram_tsp_wallet_import_local_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_TSP_WALLET_IMPORT_BLOCKED_METADATA_MARKER,
        "hepta_telegram_tsp_wallet_import_blocked_metadata_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_TSP_WALLET_IMPORT_PREFLIGHT_PACKET_MARKER,
        "hepta_telegram_tsp_wallet_import_preflight_packet_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_TSP_WALLET_IMPORT_RESULT_TAXONOMY_PACKET_MARKER,
        "hepta_telegram_tsp_wallet_import_result_taxonomy_packet_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_TSP_WORKER_RECEIPT_RESULT_PACKET_MARKER,
        "hepta_telegram_tsp_worker_receipt_result_packet_ready"
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"tsp_wallet_import_blocked_local_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"tsp_wallet_import_blocked_metadata_preview")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"tsp_wallet_import_preflight_packet_preview")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"tsp_wallet_import_result_taxonomy_packet_preview")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"tsp_worker_receipt_result_packet_preview")
    );
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"tsp_wallet_import"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "TSP wallet import blocked local evidence"
            && capability.notes.contains("loaded wallet count")
            && capability.notes.contains("active-wallet availability")
            && capability.notes.contains("active identity metadata")
            && capability.notes.contains("no file picker")
            && capability.notes.contains("wallet database")
            && capability.notes.contains("TspRequest")
            && capability.notes.contains("filesystem read/write")
            && capability.notes.contains("runtime mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "TSP wallet import preflight packet"
            && capability.notes.contains("PositiveConfirmationModal")
            && capability.notes.contains("picker_result not_started")
            && capability.notes.contains("selected_path unavailable")
            && capability.notes.contains("password_state not_collected")
            && capability.notes.contains("vault_open not_started")
            && capability.notes.contains("persistence_result not_started")
            && capability.notes.contains("duplicate-policy metadata")
            && capability.notes.contains("no file picker")
            && capability.notes.contains("password capture")
            && capability.notes.contains("TspRequest")
            && capability.notes.contains("filesystem read/write")
            && capability.notes.contains("runtime mutation")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "TSP wallet import result taxonomy packet"
            && capability.notes.contains("operation_id_slot not_assigned")
            && capability.notes.contains("picker_result canceled")
            && capability
                .notes
                .contains("auth_result password_not_collected")
            && capability.notes.contains("vault_open_result opened")
            && capability.notes.contains("duplicate_path")
            && capability.notes.contains("persistence_result saved")
            && capability
                .notes
                .contains("retry_policy selected_path_reused_password_fresh_backend_required")
            && capability
                .notes
                .contains("cancel_policy local_dismiss_no_request")
            && capability
                .notes
                .contains("stale_result_policy backend_operation_id_required_before_import_live")
            && capability.notes.contains("audit_redaction_policy")
            && capability.notes.contains("no file picker")
            && capability.notes.contains("password capture")
            && capability.notes.contains("TspRequest")
            && capability.notes.contains("filesystem read/write")
            && capability.notes.contains("runtime mutation")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "TSP worker receipt/result packet"
            && capability.notes.contains("operation_id_slot not_assigned")
            && capability.notes.contains("worker_receipt Cx_post_action")
            && capability
                .notes
                .contains("result_state success/error/canceled/stale taxonomy")
            && capability
                .notes
                .contains("retry_slot existing_guarded_paths_only")
            && capability
                .notes
                .contains("stale_result_policy local_screen_cache_match_only")
            && capability.notes.contains("audit_redaction_policy")
            && capability.notes.contains("no new TspRequest")
            && capability.notes.contains("filesystem delete")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_tsp_association_cancel_is_local_evidence() {
    assert_eq!(
        HEPTA_TELEGRAM_TSP_ASSOCIATION_CANCEL_LOCAL_MARKER,
        "hepta_telegram_tsp_association_cancel_local_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_TSP_ASSOCIATION_BLOCKED_METADATA_MARKER,
        "hepta_telegram_tsp_association_blocked_metadata_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_TSP_ASSOCIATION_CANCEL_REMOVE_PACKET_MARKER,
        "hepta_telegram_tsp_association_cancel_remove_packet_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_TSP_ASSOCIATION_RESULT_TAXONOMY_PACKET_MARKER,
        "hepta_telegram_tsp_association_result_taxonomy_packet_ready"
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"tsp_association_cancel_local_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"tsp_association_blocked_metadata_preview")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"tsp_association_cancel_remove_packet_preview")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"tsp_association_result_taxonomy_packet_preview")
    );
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"tsp_association_cancel"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "TSP association cancel local evidence"
            && capability.notes.contains("AssociateDidWithUserId")
            && capability.notes.contains("loaded target user id")
            && capability.notes.contains("DID availability")
            && capability.notes.contains("local association state")
            && capability
                .notes
                .contains("local association cancel/remove packet")
            && capability.notes.contains("CancelAssociateDidRequest")
            && capability.notes.contains("VerificationCancel")
            && capability.notes.contains("wallet database write")
            && capability.notes.contains("runtime mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "TSP association cancel/remove packet"
            && capability
                .notes
                .contains("request_id missing_backend_contract")
            && capability
                .notes
                .contains("cancel_state disabled_no_request")
            && capability
                .notes
                .contains("persistence_scope backend_required")
            && capability
                .notes
                .contains("receive_loop_scope backend_required")
            && capability
                .notes
                .contains("stale_result_policy backend_request_id_required")
            && capability.notes.contains("CancelAssociateDidRequest")
            && capability.notes.contains("VerificationCancel")
            && capability.notes.contains("TspRequest cancel")
            && capability.notes.contains("wallet database write")
            && capability.notes.contains("filesystem write")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "TSP association result taxonomy packet"
            && capability.notes.contains("local_only_cancel_not_sent")
            && capability.notes.contains("remote_cancel_not_sent")
            && capability.notes.contains("already_answered_local_state")
            && capability.notes.contains("failed_cancel_not_started")
            && capability.notes.contains("stale_request_blocked")
            && capability.notes.contains("remove_not_started")
            && capability
                .notes
                .contains("persistence_result_slot not_started")
            && capability
                .notes
                .contains("receive_loop_result_slot not_started")
            && capability
                .notes
                .contains("responder_notification_slot not_sent")
            && capability
                .notes
                .contains("retry_policy blocked_until_backend_request_id")
            && capability
                .notes
                .contains("audit_redaction target_did_presence_only")
            && capability.notes.contains("CancelAssociateDidRequest")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_tsp_verification_request_metadata_is_loaded_only() {
    assert_eq!(
        HEPTA_TELEGRAM_TSP_VERIFICATION_REQUEST_METADATA_MARKER,
        "hepta_telegram_tsp_verification_request_metadata_ready"
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"tsp_verification_request_metadata_preview")
    );
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"tsp_verification_request_metadata"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "TSP verification request loaded metadata"
            && capability.notes.contains("TspVerificationDetails")
            && capability.notes.contains("current Matrix identity")
            && capability.notes.contains("wallet VID cache")
            && capability.notes.contains("current-user match")
            && capability
                .notes
                .contains("wallet responding VID availability")
            && capability.notes.contains("no extra TspRequest")
            && capability.notes.contains("wallet database write")
            && capability.notes.contains("RespondToDidAssociationRequest")
            && capability.notes.contains("runtime mutation")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_crypto_verification_request_metadata_is_loaded_only() {
    assert_eq!(
        HEPTA_TELEGRAM_CRYPTO_VERIFICATION_REQUEST_METADATA_MARKER,
        "hepta_telegram_crypto_verification_request_metadata_ready"
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"crypto_verification_request_metadata_preview")
    );
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"crypto_verification_request_metadata"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "Matrix crypto verification request metadata"
            && capability.notes.contains("VerificationRequest")
            && capability.notes.contains("own user")
            && capability.notes.contains("other user")
            && capability.notes.contains("supported-method count")
            && capability.notes.contains("no Matrix verification accept")
            && capability.notes.contains("device trust write")
            && capability.notes.contains("response_sender")
            && capability.notes.contains("async verification handler")
            && capability.notes.contains("runtime mutation")
            && capability.notes.contains("live mutation")
    }));
}

#[test]
fn hepta_telegram_base_login_auto_cancel_is_local_evidence() {
    assert_eq!(
        HEPTA_TELEGRAM_LOGIN_AUTO_CANCEL_LOCAL_MARKER,
        "hepta_telegram_login_auto_cancel_local_ready"
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"login_auto_cancel_local_evidence"));
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"login_auto_cancel"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "login auto cancel local evidence"
            && capability.notes.contains("CLI auto-login")
            && capability.notes.contains("disabled Cancel")
            && capability
                .notes
                .contains("password login Cancel only closes")
            && capability
                .notes
                .contains("SSO Cancel keeps the existing local redirect-server")
            && capability.notes.contains("No Matrix login cancel request")
            && capability.notes.contains("runtime mutation")
    }));
}

#[test]
fn hepta_telegram_base_profile_read_receipt_surface_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_PROFILE_READ_RECEIPT_LOCAL_SURFACE_MARKER,
        "hepta_telegram_profile_read_receipt_local_surface_ready"
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"profile_read_receipt_surface"));
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"profile_read_receipt_surface"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "profile/avatar"
            && capability
                .notes
                .contains("local-only read receipt jump preview")
    }));
}

#[test]
fn hepta_telegram_base_profile_member_read_path_is_read_only() {
    assert_eq!(
        HEPTA_TELEGRAM_PROFILE_MEMBER_READ_MARKER,
        "hepta_telegram_profile_member_read_ready"
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"profile_member_read_evidence"));
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"profile_member_read"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "profile/member read path"
            && capability.notes.contains("loaded user_profile_cache state")
            && capability
                .notes
                .contains("existing Matrix GetUserProfile/profile-member read path")
            && capability
                .notes
                .contains("Requested entries suppress duplicates")
            && capability
                .notes
                .contains("fetch_if_missing only submits GetUserProfile read requests")
            && capability
                .notes
                .contains("starts no profile mutation, ignore/block")
            && capability
                .notes
                .contains("direct-message, message, room-state, or membership mutation request")
    }));
}

#[test]
fn hepta_telegram_base_profile_account_identity_clipboard_is_local_only() {
    assert_eq!(
        HEPTA_TELEGRAM_PROFILE_ACCOUNT_IDENTITY_CLIPBOARD_MARKER,
        "hepta_telegram_profile_account_identity_clipboard_ready"
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"profile_account_identity_clipboard_surface")
    );
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"profile_account_identity_clipboard"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "profile/account identity clipboard"
            && capability.notes.contains("loaded user id")
            && capability.notes.contains("loaded own profile id")
            && capability.notes.contains("write clipboard text locally")
            && capability.notes.contains("without Matrix profile lookup")
            && capability.notes.contains("account request")
            && capability.notes.contains("event fetch")
            && capability.notes.contains("message send")
            && capability.notes.contains("room-state")
            && capability.notes.contains("membership")
            && capability.notes.contains("live mutation request")
    }));
}

#[test]
fn hepta_telegram_base_profile_direct_message_confirmation_is_guarded() {
    assert_eq!(
        HEPTA_TELEGRAM_PROFILE_DIRECT_MESSAGE_CONFIRMATION_MARKER,
        "hepta_telegram_profile_direct_message_confirmation_ready"
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"profile_direct_message_confirmation_guard")
    );
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"profile_direct_message"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "profile direct message confirmation"
            && capability
                .notes
                .contains("before the existing Matrix OpenOrCreateDirectMessage lookup path")
            && capability.notes.contains("allow_create=false")
            && capability.notes.contains("Cancel keeps the request unsent")
    }));
}

#[test]
fn hepta_telegram_base_direct_message_create_confirmation_is_guarded() {
    assert_eq!(
        HEPTA_TELEGRAM_DIRECT_MESSAGE_CREATE_CONFIRMATION_MARKER,
        "hepta_telegram_direct_message_create_confirmation_ready"
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"direct_message_create_confirmation_guard")
    );
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"direct_message_create"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "direct message create confirmation"
            && capability
                .notes
                .contains("OpenOrCreateDirectMessage create path")
            && capability.notes.contains("allow_create=true")
            && capability
                .notes
                .contains("Cancel keeps the create request unsent")
    }));
}

#[test]
fn hepta_telegram_base_profile_ignore_confirmation_is_guarded() {
    assert_eq!(
        HEPTA_TELEGRAM_PROFILE_IGNORE_CONFIRMATION_MARKER,
        "hepta_telegram_profile_ignore_confirmation_ready"
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"profile_ignore_confirmation_guard"));
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"profile_ignore_block"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "profile ignore/block confirmation"
            && capability
                .notes
                .contains("before the existing Matrix IgnoreUser path is requested")
            && capability.notes.contains("Cancel keeps the request unsent")
    }));
}

#[test]
fn hepta_telegram_base_account_display_name_staging_is_guarded() {
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_DISPLAY_NAME_STAGING_MARKER,
        "hepta_telegram_account_display_name_staging_ready"
    );
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_DISPLAY_NAME_CONFIRMATION_MARKER,
        "hepta_telegram_account_display_name_confirmation_ready"
    );
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"account_display_name_staging_surface"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"account_display_name_confirmation_guard")
    );
    assert!(!HEPTA_TELEGRAM_BASE_GAPS.contains(&"account_display_name"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "account display name"
            && capability
                .notes
                .contains("stages display name drafts locally")
            && capability.notes.contains(
                "Matrix SetDisplayName is requested only from the confirmed accept handler",
            )
            && capability
                .notes
                .contains("DisplayNameChanged repaints cached profile/input state locally")
            && capability
                .notes
                .contains("Cancel/reset keeps SetDisplayName")
            && capability.notes.contains("avatar, account")
            && capability.notes.contains("device/session-management")
            && capability
                .notes
                .contains("message, room-state, and membership requests unsent")
    }));
}

#[test]
fn hepta_telegram_base_account_avatar_upload_surface_has_confirmed_upload_path() {
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_AVATAR_UPLOAD_LOCAL_SURFACE_MARKER,
        "hepta_telegram_account_avatar_upload_local_surface_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"account_avatar_upload"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"account_avatar_upload_surface"));
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"account_avatar_upload_option_staging_local_evidence")
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"account_avatar_upload_lifecycle_metadata_preview")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "account avatar upload"
            && capability.notes.contains("desktop image picker")
            && capability.notes.contains("selected-file metadata preview")
            && capability.notes.contains("MatrixRequest::UploadAvatar")
            && capability.notes.contains("client.account().upload_avatar")
            && capability
                .notes
                .contains("Account::set_avatar_url(Some(mxc))")
            && capability.notes.contains("direct MXC editor")
            && capability.notes.contains("MatrixRequest::SetAvatar(Some)")
            && capability.notes.contains("Crop, Cancel, picker cancel")
            && capability
                .notes
                .contains("mobile camera/photo-library capture")
    }));
}

#[test]
fn hepta_telegram_base_account_avatar_upload_live_wiring_is_partial_live() {
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_AVATAR_UPLOAD_LIVE_WIRING_MARKER,
        "hepta_telegram_account_avatar_upload_live_wiring_ready"
    );
    assert!(HEPTA_TELEGRAM_BASE_GAPS.contains(&"account_avatar_upload"));
    assert!(HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES.contains(&"account_avatar_upload_live_wiring"));
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "account avatar upload live wiring"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability
                .base_module
                .contains("MatrixRequest::UploadAvatar")
            && capability
                .base_module
                .contains("client.account().upload_avatar")
            && capability.notes.contains("PositiveConfirmationModal")
            && capability.notes.contains("MatrixRequest::UploadAvatar")
            && capability.notes.contains("client.account().upload_avatar")
            && capability
                .notes
                .contains("Account::set_avatar_url(Some(mxc))")
            && capability.notes.contains("AvatarChanged(Some(mxc))")
            && capability.notes.contains("Failed Retry")
            && capability.notes.contains("cropper/editor")
            && capability.notes.contains("camera/photo-library")
            && capability.notes.contains("live mutation")
    }));
    assert_eq!(
        HEPTA_TELEGRAM_ACCOUNT_AVATAR_DIRECT_MXC_SET_MARKER,
        "hepta_telegram_account_avatar_direct_mxc_set_ready"
    );
    assert!(
        HEPTA_TELEGRAM_LOCAL_GAP_AFFORDANCES
            .contains(&"account_avatar_direct_mxc_setavatar_live_wiring")
    );
    assert!(HEPTA_TELEGRAM_BASE_CAPABILITIES.iter().any(|capability| {
        capability.telegram_surface == "account avatar direct MXC SetAvatar(Some)"
            && capability.status == HeptaTelegramBaseStatus::DirectReuse
            && capability.base_module.contains("avatar_direct_mxc_input")
            && capability
                .base_module
                .contains("MatrixRequest::SetAvatar(Some)")
            && capability
                .base_module
                .contains("client.account().set_avatar_url(Some)")
            && capability
                .notes
                .contains("validates an existing mxc:// URI")
            && capability.notes.contains("PositiveConfirmationModal")
            && capability.notes.contains("AvatarChangeFailed")
            && capability.notes.contains("confirmed Retry")
            && capability.notes.contains("gateway/runtime/auth")
            && capability.notes.contains("unconfirmed live mutation")
    }));
    assert!(HEPTA_TELEGRAM_BASE_GAP_PRODUCT_RUNWAY.iter().any(|runway| {
        runway.gap_id == "account_avatar_upload"
            && runway
                .current_path
                .contains("live confirmed UploadAvatar/client.account().upload_avatar wiring")
            && runway
                .current_path
                .contains("SDK Account::set_avatar_url(Some(mxc))")
            && runway
                .current_path
                .contains("direct MXC MatrixRequest::SetAvatar(Some)")
            && runway
                .current_path
                .contains("bounded in-memory thumbnail/full-size pixel decode")
            && runway.remaining_gap.contains("cropper/editor")
            && runway.remaining_gap.contains("camera/photo-library")
            && runway
                .remaining_gap
                .contains("persistent thumbnail artifact mapping")
            && runway.remaining_gap.contains("editor result mapping")
    }));
}
