#[cfg(test)]
mod notifications_live_result_tests {
    use super::*;

    #[test]
    fn notification_mode_write_result_popup_message_reports_success() {
        let label = notification_mode_write_result_popup_message(
            "Hepta UI",
            RoomNotificationMode::MentionsAndKeywordsOnly,
            &Ok(()),
        );

        assert_eq!(label, "Notification mode updated for Hepta UI: Mentions.");
    }

    #[test]
    fn notification_mode_write_result_popup_message_reports_failure() {
        let label = notification_mode_write_result_popup_message(
            "Hepta UI",
            RoomNotificationMode::Mute,
            &Err("M_FORBIDDEN".to_string()),
        );

        assert!(label.contains("Notification mode update failed for Hepta UI: Mute."));
        assert!(label.contains("M_FORBIDDEN"));
    }

    #[test]
    fn notification_default_room_mode_write_result_popup_message_reports_readback() {
        let summary = NotificationDefaultRoomModeSummary {
            mode: RoomNotificationMode::AllMessages,
            is_encrypted: true,
            is_one_to_one: true,
            active_members_count: 2,
        };
        let label = notification_default_room_mode_write_result_popup_message(
            "Hepta UI",
            RoomNotificationMode::AllMessages,
            &Ok(summary),
        );

        assert!(label.contains("Default notification mode updated for Hepta UI"));
        assert!(label.contains("All messages default for encrypted one-to-one rooms"));
    }

    #[test]
    fn notification_default_room_mode_write_result_popup_message_reports_failure() {
        let label = notification_default_room_mode_write_result_popup_message(
            "Hepta UI",
            RoomNotificationMode::Mute,
            &Err("M_FORBIDDEN".to_string()),
        );

        assert!(label.contains("Default notification mode update failed for Hepta UI: Mute."));
        assert!(label.contains("M_FORBIDDEN"));
    }
}

#[cfg(test)]
mod notifications_close_refresh_metadata_tests {
    use super::*;

    #[test]
    fn notifications_close_refresh_metadata_label_summarizes_refresh_state() {
        let label = notifications_close_refresh_metadata_label(
            "Hepta UI",
            "refresh",
            "Current Matrix mode: all messages",
            Some("Refreshing current mode"),
            true,
            true,
        );

        assert!(label.contains("Notification refresh metadata for Hepta UI"));
        assert!(label.contains("Current Matrix mode: all messages"));
        assert!(label.contains("local status Refreshing current mode"));
        assert!(label.contains("loaded attention metadata ready"));
        assert!(label.contains("timeline loaded for mode read"));
        assert!(label.contains(NOTIFICATIONS_CLOSE_REFRESH_METADATA_LABEL));
    }

    #[test]
    fn notifications_close_refresh_metadata_label_reports_close_waiting_state() {
        let label = notifications_close_refresh_metadata_label(
            "this chat",
            "close",
            "Current Matrix mode: loading read-only",
            None,
            false,
            false,
        );

        assert!(label.contains("Notification close metadata for this chat"));
        assert!(label.contains("no local status staged"));
        assert!(label.contains("loaded attention metadata waiting"));
        assert!(label.contains("timeline waiting for mode read"));
        assert!(label.contains("no notification mode write"));
    }
}

#[cfg(test)]
mod notifications_timed_global_boundary_tests {
    use super::*;

    #[test]
    fn notifications_timed_global_boundary_label_summarizes_loaded_state() {
        let label = notifications_timed_global_boundary_label(
            "Current Matrix mode: mentions and keywords",
            "Loaded attention: 3 unread, 1 mentions, manual unread",
            Some("Confirmation open: Mentions"),
        );

        assert!(label.contains("Current Matrix mode: mentions and keywords"));
        assert!(label.contains("Loaded attention: 3 unread, 1 mentions, manual unread"));
        assert!(label.contains("local status Confirmation open: Mentions"));
        assert!(label.contains("timed mute"));
        assert!(label.contains("Global/Defaults"));
        assert!(label.contains("Default All/Mentions/Mute"));
        assert!(label.contains("write that default only after confirmation"));
        assert!(label.contains("push gateway/device"));
        assert!(label.contains("pusher config"));
        assert!(label.contains(NOTIFICATIONS_TIMED_GLOBAL_BOUNDARY_LABEL));
    }

    #[test]
    fn notifications_timed_global_boundary_label_reports_waiting_state() {
        let label = notifications_timed_global_boundary_label(
            "Current Matrix mode: loading read-only",
            "Loaded attention: waiting for room-list unread state",
            None,
        );

        assert!(label.contains("Current Matrix mode: loading read-only"));
        assert!(label.contains("Loaded attention: waiting for room-list unread state"));
        assert!(label.contains("no local status staged"));
        assert!(label.contains("unwired"));
    }

    #[test]
    fn notifications_pusher_keyword_boundary_label_lists_blocked_controls() {
        let label = notifications_pusher_keyword_boundary_label(
            "Current Matrix mode: muted",
            "Loaded attention: 3 unread, 1 mentions, manual unread",
            true,
            Some("Update failed: network"),
        );

        assert!(label.contains("Pusher/keyword boundary"));
        assert!(label.contains("Current Matrix mode: muted"));
        assert!(label.contains("Loaded attention: 3 unread, 1 mentions, manual unread"));
        assert!(label.contains("retry cache ready"));
        assert!(label.contains("local status Update failed: network"));
        assert!(label.contains("Keyword rules"));
        assert!(label.contains("Global/Defaults"));
        assert!(label.contains("Default All/Mentions/Mute"));
        assert!(label.contains("live SDK default room-mode writes"));
        assert!(label.contains("timed mute presets"));
        assert!(label.contains("push gateway/device setup"));
        assert!(label.contains("pusher enable/disable"));
        assert!(label.contains("sound/badge tuning"));
        assert!(label.contains("room-list notification indication"));
        assert!(label.contains(NOTIFICATIONS_PUSHER_KEYWORD_BOUNDARY_LABEL));
        assert!(label.contains("notification rule account-data edit"));
        assert!(label.contains("pusher mutation"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
    }

    #[test]
    fn notifications_pusher_keyword_boundary_label_reports_empty_cache() {
        let label = notifications_pusher_keyword_boundary_label(
            "Current Matrix mode: loading read-only",
            "Loaded attention: waiting for room-list unread state",
            false,
            None,
        );

        assert!(label.contains("retry cache empty"));
        assert!(label.contains("no local status staged"));
        assert!(label.contains("stay local blocked controls"));
    }

    #[test]
    fn notification_keyword_rules_summary_label_limits_enabled_keywords() {
        let summary = NotificationKeywordRulesSummary {
            has_enabled_keywords: true,
            enabled_keywords: vec![
                "alpha".to_string(),
                "beta".to_string(),
                "gamma".to_string(),
                "delta".to_string(),
                "epsilon".to_string(),
                "zeta".to_string(),
            ],
        };
        let label = notification_keyword_rules_summary_label(&summary);

        assert!(label.contains("6 enabled custom keyword rule(s)"));
        assert!(label.contains("alpha, beta, gamma, delta, epsilon"));
        assert!(label.contains("+1 more"));
    }

    #[test]
    fn notification_keyword_rules_summary_label_reports_empty_state() {
        let summary = NotificationKeywordRulesSummary {
            has_enabled_keywords: false,
            enabled_keywords: Vec::new(),
        };

        assert_eq!(
            notification_keyword_rules_summary_label(&summary),
            "no enabled custom keyword rules"
        );
    }

    #[test]
    fn notifications_keyword_rules_live_read_requested_label_mentions_matrix_request() {
        let label = notifications_keyword_rules_live_read_requested_label(
            "Hepta UI",
            "Keyword list",
            "Current Matrix mode: all messages",
            "Loaded attention: 1 unread, 0 mentions",
        );

        assert!(label.contains("Keyword list live read submitted for Hepta UI"));
        assert!(label.contains("MatrixRequest::GetNotificationKeywordRules"));
        assert!(label.contains("NotificationSettings::enabled_keywords"));
        assert!(label.contains("no unconfirmed keyword write"));
    }

    #[test]
    fn notifications_keyword_rules_unavailable_label_stays_local() {
        let label = notifications_keyword_rules_unavailable_label(
            "Hepta UI",
            "Keywords",
            "Current Matrix mode: loading read-only",
            "Loaded attention: waiting for room-list unread state",
        );

        assert!(label.contains("Keywords live read unavailable for Hepta UI"));
        assert!(label.contains("timeline waiting"));
        assert!(label.contains("No MatrixRequest::GetNotificationKeywordRules"));
        assert!(label.contains("live mutation"));
    }

    #[test]
    fn notifications_keyword_rules_live_result_label_reports_sdk_read() {
        let summary = NotificationKeywordRulesSummary {
            has_enabled_keywords: true,
            enabled_keywords: vec!["release".to_string(), "urgent".to_string()],
        };
        let label = notifications_keyword_rules_live_result_label(
            "Hepta UI",
            &summary,
            "Current Matrix mode: mentions and keywords",
            "Loaded attention: 3 unread, 1 mentions",
            Some("Keywords live read submitted"),
        );

        assert!(label.contains("Keyword list live result for Hepta UI"));
        assert!(label.contains("2 enabled custom keyword rule(s): release, urgent"));
        assert!(label.contains("NotificationSettings::contains_keyword_rules"));
        assert!(label.contains("enabled_keywords"));
        assert!(label.contains("no unconfirmed add/remove keyword rule write"));
    }

    #[test]
    fn notifications_keyword_mutation_confirmation_label_mentions_sdk_write() {
        let label = notifications_keyword_mutation_confirmation_label(
            "Hepta UI",
            "urgent",
            NotificationKeywordMutation::Add,
        );

        assert!(label.contains("Add keyword rule for Hepta UI: urgent"));
        assert!(label.contains("MatrixRequest::SetNotificationKeywordRule"));
        assert!(label.contains("NotificationSettings::add_keyword/remove_keyword"));
    }

    #[test]
    fn notifications_keyword_mutation_result_label_reports_retry_on_failure() {
        let label = notifications_keyword_mutation_result_label(
            "Hepta UI",
            "release",
            NotificationKeywordMutation::Remove,
            &Err("M_FORBIDDEN".to_string()),
            "Current Matrix mode: mentions and keywords",
            "Loaded attention: 3 unread, 1 mentions",
        );

        assert!(label.contains("Remove keyword failed for Hepta UI: release"));
        assert!(label.contains("Failed-state Retry"));
        assert!(label.contains("PositiveConfirmationModal"));
        assert!(label.contains("no automatic retry"));
    }

    #[test]
    fn notification_pusher_status_summary_label_reports_supported_state() {
        let summary = NotificationPusherStatusSummary {
            encrypted_event_to_device_push: Ok(true),
        };

        assert_eq!(
            notification_pusher_status_summary_label(&summary),
            "homeserver supports encrypted push-to-device capability"
        );
    }

    #[test]
    fn notification_pusher_status_summary_label_reports_error_state() {
        let summary = NotificationPusherStatusSummary {
            encrypted_event_to_device_push: Err("M_UNKNOWN".to_string()),
        };

        let label = notification_pusher_status_summary_label(&summary);
        assert!(label.contains("capability read failed"));
        assert!(label.contains("M_UNKNOWN"));
    }

    #[test]
    fn notifications_pusher_status_live_read_requested_label_mentions_matrix_request() {
        let label = notifications_pusher_status_live_read_requested_label(
            "Hepta UI",
            "Pushers",
            "Current Matrix mode: all messages",
            "Loaded attention: 1 unread, 0 mentions",
        );

        assert!(label.contains("Pushers live read submitted for Hepta UI"));
        assert!(label.contains("MatrixRequest::GetNotificationPusherStatus"));
        assert!(label.contains("Client::can_homeserver_push_encrypted_event_to_device"));
        assert!(label.contains("no pusher set/delete mutation"));
    }

    #[test]
    fn notifications_pusher_status_unavailable_label_stays_local() {
        let label = notifications_pusher_status_unavailable_label(
            "Hepta UI",
            "Device push",
            "Current Matrix mode: loading read-only",
            "Loaded attention: waiting for room-list unread state",
        );

        assert!(label.contains("Device push live read unavailable for Hepta UI"));
        assert!(label.contains("timeline waiting"));
        assert!(label.contains("No MatrixRequest::GetNotificationPusherStatus"));
        assert!(label.contains("live mutation"));
    }

    #[test]
    fn notifications_pusher_status_live_result_label_reports_sdk_read() {
        let summary = NotificationPusherStatusSummary {
            encrypted_event_to_device_push: Ok(false),
        };
        let label = notifications_pusher_status_live_result_label(
            "Hepta UI",
            &summary,
            "Current Matrix mode: mentions and keywords",
            "Loaded attention: 3 unread, 1 mentions",
            Some("Pushers live read submitted"),
        );

        assert!(label.contains("Pusher status live result for Hepta UI"));
        assert!(label.contains("does not advertise encrypted push-to-device capability"));
        assert!(label.contains("Client::can_homeserver_push_encrypted_event_to_device"));
        assert!(label.contains("no pusher set/delete mutation"));
    }

    #[test]
    fn notification_default_room_mode_summary_label_reports_room_class() {
        let summary = NotificationDefaultRoomModeSummary {
            mode: RoomNotificationMode::MentionsAndKeywordsOnly,
            is_encrypted: true,
            is_one_to_one: false,
            active_members_count: 8,
        };
        let label = notification_default_room_mode_summary_label(&summary);

        assert!(label.contains("Mentions default"));
        assert!(label.contains("encrypted group rooms"));
        assert!(label.contains("8 active members"));
    }

    #[test]
    fn notifications_default_room_mode_live_read_requested_label_mentions_matrix_request() {
        let label = notifications_default_room_mode_live_read_requested_label(
            "Hepta UI",
            "Defaults",
            "Current Matrix mode: all messages",
            "Loaded attention: 1 unread, 0 mentions",
        );

        assert!(label.contains("Defaults live read submitted for Hepta UI"));
        assert!(label.contains("MatrixRequest::GetDefaultRoomNotificationMode"));
        assert!(label.contains("NotificationSettings::get_default_room_notification_mode"));
        assert!(label.contains("no default preference write"));
    }

    #[test]
    fn notifications_default_room_mode_live_result_label_reports_sdk_read() {
        let summary = NotificationDefaultRoomModeSummary {
            mode: RoomNotificationMode::AllMessages,
            is_encrypted: false,
            is_one_to_one: true,
            active_members_count: 2,
        };
        let label = notifications_default_room_mode_live_result_label(
            "Hepta UI",
            &Ok(summary),
            "Current Matrix mode: all messages",
            "Loaded attention: 3 unread, 1 mentions",
            Some("Defaults live read submitted"),
        );

        assert!(label.contains("Default notification mode live result for Hepta UI"));
        assert!(label.contains("All messages default for unencrypted one-to-one rooms"));
        assert!(label.contains("NotificationSettings::get_default_room_notification_mode"));
        assert!(label.contains("no default preference write"));
    }

    #[test]
    fn notifications_default_room_mode_write_confirmation_label_mentions_sdk_write() {
        let label = notifications_default_room_mode_write_confirmation_label(
            "Hepta UI",
            RoomNotificationMode::MentionsAndKeywordsOnly,
        );

        assert!(label.contains("Set default notification mode for Hepta UI to Mentions"));
        assert!(label.contains("MatrixRequest::SetDefaultRoomNotificationMode"));
        assert!(label.contains("NotificationSettings::set_default_room_notification_mode"));
        assert!(label.contains("PositiveConfirmationModal"));
    }

    #[test]
    fn notifications_default_room_mode_write_result_label_reports_sdk_readback() {
        let summary = NotificationDefaultRoomModeSummary {
            mode: RoomNotificationMode::Mute,
            is_encrypted: true,
            is_one_to_one: false,
            active_members_count: 8,
        };
        let label = notifications_default_room_mode_write_result_label(
            "Hepta UI",
            RoomNotificationMode::Mute,
            &Ok(summary),
            "Current Matrix mode: mentions and keywords",
            "Loaded attention: 3 unread, 1 mentions",
        );

        assert!(label.contains("Default notification mode write result for Hepta UI"));
        assert!(label.contains("requested Mute"));
        assert!(label.contains("Mute default for encrypted group rooms"));
        assert!(label.contains("NotificationSettings::set_default_room_notification_mode"));
        assert!(label.contains("SDK default room mode was read back"));
    }

    #[test]
    fn notifications_default_room_mode_write_result_label_reports_retry_on_failure() {
        let label = notifications_default_room_mode_write_result_label(
            "Hepta UI",
            RoomNotificationMode::AllMessages,
            &Err("M_FORBIDDEN".to_string()),
            "Current Matrix mode: muted",
            "Loaded attention: 3 unread, 1 mentions",
        );

        assert!(label.contains("Default All messages write failed for Hepta UI"));
        assert!(label.contains("Failed-state Retry"));
        assert!(label.contains("SetDefaultRoomNotificationMode"));
        assert!(label.contains("no automatic retry"));
    }

    #[test]
    fn notifications_default_room_mode_retry_confirmation_label_names_default_request() {
        let label = notifications_default_room_mode_retry_confirmation_label(
            "Hepta UI",
            RoomNotificationMode::Mute,
        );

        assert!(label.contains("Retry default notification mode update for Hepta UI"));
        assert!(
            label.contains("Failed-state Retry confirms before SetDefaultRoomNotificationMode")
        );
        assert!(label.contains("timed mute"));
        assert!(label.contains("pusher writes"));
        assert!(label.contains("gateway/runtime/auth"));
    }

    #[test]
    fn notifications_mode_clipboard_payload_uses_loaded_mode_and_attention() {
        let payload = notifications_mode_clipboard_payload(
            "Hepta UI",
            "Mute",
            "Loaded attention: 3 unread, 1 mentions, manual unread",
            Some("Updated: Mute"),
        )
        .expect("loaded mode should produce a clipboard payload");

        assert!(payload.contains("Notification mode"));
        assert!(payload.contains("Room: Hepta UI"));
        assert!(payload.contains("Mode: Mute"));
        assert!(payload.contains("Loaded attention: 3 unread, 1 mentions, manual unread"));
        assert!(payload.contains("Local status: Updated: Mute"));
        assert!(payload.contains("loaded RoomScreen notification state"));
        assert!(notifications_mode_clipboard_payload("Hepta UI", "   ", "", None).is_none());
    }

    #[test]
    fn notifications_mode_clipboard_label_reports_copy_and_unavailable_states() {
        let copied = notifications_mode_clipboard_label(
            true,
            "Hepta UI",
            Some("Mentions"),
            "Loaded attention: 3 unread, 1 mentions, manual unread",
            true,
            Some("Confirmation open: Mentions"),
        );

        assert!(copied.contains("Notification mode clipboard for Hepta UI"));
        assert!(copied.contains("clipboard payload copied"));
        assert!(copied.contains("loaded mode Mentions"));
        assert!(copied.contains("timeline loaded"));
        assert!(copied.contains("local status Confirmation open: Mentions"));
        assert!(copied.contains(NOTIFICATIONS_MODE_CLIPBOARD_LABEL));
        assert!(copied.contains("No SetRoomNotificationMode"));
        assert!(copied.contains("push gateway/device"));
        assert!(copied.contains("gateway/runtime/auth"));
        assert!(copied.contains("live mutation"));

        let unavailable =
            notifications_mode_clipboard_label(false, "this chat", None, "", false, None);

        assert!(unavailable.contains("clipboard payload unavailable"));
        assert!(unavailable.contains("loaded mode waiting"));
        assert!(unavailable.contains("Loaded attention: unavailable"));
        assert!(unavailable.contains("timeline waiting"));
        assert!(unavailable.contains("no local status staged"));
    }

    #[test]
    fn notifications_advanced_control_boundary_label_is_local_only() {
        let label = notifications_advanced_control_boundary_label(
            "Hepta UI",
            "Keyword rules",
            "Current Matrix mode: mentions and keywords",
            "Loaded attention: 3 unread, 1 mentions, manual unread",
        );

        assert!(label.contains("Keyword rules staged for Hepta UI"));
        assert!(label.contains("Current Matrix mode: mentions and keywords"));
        assert!(label.contains("Loaded attention: 3 unread, 1 mentions, manual unread"));
        assert!(label.contains(NOTIFICATIONS_ADVANCED_CONTROLS_LABEL));
        assert!(label.contains("live read-only GetNotificationKeywordRules"));
        assert!(label.contains("notification rule account-data edit"));
        assert!(label.contains("pusher mutation"));
        assert!(label.contains("push gateway/device configuration"));
        assert!(label.contains("unconfirmed keyword write"));
        assert!(label.contains("timed mute write"));
        assert!(label.contains("unconfirmed default write"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(NOTIFICATIONS_ADVANCED_CONTROLS_EVIDENCE.contains("Timed, Keywords"));
        assert!(
            NOTIFICATIONS_ADVANCED_CONTROLS_EVIDENCE
                .contains("confirmed All/Mentions/Mute SetRoomNotificationMode")
        );
    }

    #[test]
    fn notifications_advanced_detail_control_label_is_local_only() {
        let label = notifications_advanced_detail_control_label(
            "Hepta UI",
            "Device push",
            "Current Matrix mode: all messages",
            "Loaded attention: 3 unread, 1 mentions, manual unread",
            true,
        );

        assert!(label.contains("Device push detail staged for Hepta UI"));
        assert!(label.contains("Current Matrix mode: all messages"));
        assert!(label.contains("Loaded attention: 3 unread, 1 mentions, manual unread"));
        assert!(label.contains("retry cache ready"));
        assert!(label.contains(NOTIFICATIONS_ADVANCED_DETAIL_CONTROLS_LABEL));
        assert!(label.contains("live read-only GetNotificationKeywordRules"));
        assert!(label.contains("notification rule account-data edit"));
        assert!(label.contains("push-rule write"));
        assert!(label.contains("pusher mutation"));
        assert!(label.contains("push gateway/device configuration"));
        assert!(label.contains("timed mute write"));
        assert!(label.contains("unconfirmed default write"));
        assert!(label.contains("sound/badge tuning"));
        assert!(label.contains("retry automation"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(
            NOTIFICATIONS_ADVANCED_DETAIL_CONTROLS_EVIDENCE.contains("Quiet hours, Keyword list")
        );
        assert!(
            NOTIFICATIONS_ADVANCED_DETAIL_CONTROLS_EVIDENCE.contains("failed-state Retry paths")
        );
    }

    #[test]
    fn notifications_advanced_detail_control_label_uses_safe_fallbacks() {
        let label = notifications_advanced_detail_control_label(
            "this chat",
            "   ",
            "Current Matrix mode: loading read-only",
            "Loaded attention: waiting for room-list unread state",
            false,
        );

        assert!(label.contains("Advanced notification detail"));
        assert!(label.contains("retry cache empty"));
        assert!(label.contains("no notification rule account-data edit"));
    }

    #[test]
    fn notifications_result_detail_control_label_summarizes_failed_retry_state() {
        let label = notifications_result_detail_control_label(
            "Hepta UI",
            Some("Failure"),
            "Current Matrix mode: muted",
            "Loaded attention: 3 unread, 1 mentions, manual unread",
            Some("Mute"),
            true,
            true,
            Some("Update failed: Mute"),
        );

        assert!(label.contains("Notification result detail for Hepta UI"));
        assert!(label.contains("Failure detail selected"));
        assert!(label.contains("Current Matrix mode: muted"));
        assert!(label.contains("Loaded attention: 3 unread, 1 mentions, manual unread"));
        assert!(label.contains("requested mode Mute"));
        assert!(label.contains("retry cache ready"));
        assert!(label.contains("timeline loaded"));
        assert!(label.contains("local status 19 chars"));
        assert!(label.contains(NOTIFICATIONS_RESULT_DETAIL_CONTROLS_LABEL));
        assert!(label.contains("no extra read"));
        assert!(label.contains("no unconfirmed SetRoomNotificationMode"));
        assert!(label.contains("timed mute"));
        assert!(label.contains("push-rule"));
        assert!(label.contains("pusher mutation"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(
            NOTIFICATIONS_RESULT_DETAIL_CONTROLS_EVIDENCE
                .contains("Result, Requested, Retry cache, Failure, and Source")
        );
        assert!(
            NOTIFICATIONS_RESULT_DETAIL_CONTROLS_EVIDENCE
                .contains("current loaded room notification mode")
        );
        assert!(NOTIFICATIONS_RESULT_DETAIL_CONTROLS_EVIDENCE.contains("failed-state Retry paths"));
    }

    #[test]
    fn notifications_result_detail_control_label_reports_waiting_state() {
        let label = notifications_result_detail_control_label(
            "this chat",
            Some("   "),
            "Current Matrix mode: loading read-only",
            "Loaded attention: waiting for room-list unread state",
            None,
            false,
            false,
            None,
        );

        assert!(label.contains("no notification result detail selected"));
        assert!(label.contains("no requested mode staged"));
        assert!(label.contains("retry cache empty"));
        assert!(label.contains("timeline waiting"));
        assert!(label.contains("local status 0 chars"));
        assert!(label.contains("Result, Requested, Retry cache, Failure, and Source"));
    }

    #[test]
    fn notifications_preflight_detail_control_label_summarizes_pusher_scope() {
        let label = notifications_preflight_detail_control_label(
            "Hepta UI",
            Some("Pushers"),
            "Current Matrix mode: mentions and keywords",
            "Loaded attention: 3 unread, 1 mentions, manual unread",
            Some("Mentions"),
            true,
            true,
            Some("Confirmation open: Mentions"),
        );

        assert!(label.contains("Notification timed/global/pusher preflight for Hepta UI"));
        assert!(label.contains("Pushers preflight selected"));
        assert!(label.contains("Current Matrix mode: mentions and keywords"));
        assert!(label.contains("Loaded attention: 3 unread, 1 mentions, manual unread"));
        assert!(label.contains("requested mode Mentions"));
        assert!(label.contains("retry cache ready"));
        assert!(label.contains("timeline loaded for current mode"));
        assert!(label.contains("local status 27 chars"));
        assert!(label.contains(NOTIFICATIONS_PREFLIGHT_DETAIL_CONTROLS_LABEL));
        assert!(label.contains("notification rule account-data write"));
        assert!(label.contains("push-rule write"));
        assert!(label.contains("pusher mutation"));
        assert!(label.contains("push gateway/device configuration"));
        assert!(label.contains("extra GetRoomNotificationMode"));
        assert!(label.contains("unconfirmed SetRoomNotificationMode"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(
            NOTIFICATIONS_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE.contains(
                "Schedule, Packet, Contract, Account data, Keywords, Pushers, and Defaults"
            )
        );
        assert!(
            NOTIFICATIONS_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
                .contains("typed account-data/push-rule/pusher/result contract packet")
        );
        assert!(
            NOTIFICATIONS_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
                .contains("live read-only Matrix SDK handoff")
        );
        assert!(
            NOTIFICATIONS_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
                .contains("local notification schedule packet snapshot")
        );
    }

    #[test]
    fn notifications_schedule_local_snapshot_label_summarizes_loaded_packet() {
        let label = notifications_schedule_local_snapshot_label(
            "Hepta UI",
            "Current Matrix mode: all messages",
            "Loaded attention: 3 unread, 1 mentions, manual unread",
            Some("Mute"),
            true,
            true,
            Some("Confirmation open: Mute"),
        );

        assert!(label.contains("Local notification schedule snapshot for Hepta UI"));
        assert!(label.contains("Current Matrix mode: all messages"));
        assert!(label.contains("Loaded attention: 3 unread, 1 mentions, manual unread"));
        assert!(label.contains("requested mode Mute"));
        assert!(label.contains("retry cache ready"));
        assert!(label.contains("timeline loaded for current mode"));
        assert!(label.contains("local status 23 chars"));
        assert!(label.contains("timed mute window not selected"));
        assert!(label.contains(NOTIFICATIONS_PREFLIGHT_DETAIL_CONTROLS_LABEL));
        assert!(label.contains("Schedule renders this loaded local schedule packet"));
        assert!(label.contains("notification rule account-data read or write"));
        assert!(label.contains("timed mute write"));
        assert!(label.contains("pusher mutation"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
    }

    #[test]
    fn notifications_rule_packet_payload_lists_result_retry_acceptance() {
        let payload = notifications_rule_packet_payload(
            "Hepta UI",
            "Current Matrix mode: mentions and keywords",
            "Loaded attention: 3 unread, 1 mentions, manual unread",
            Some("Mute"),
            true,
            true,
            Some("Update failed: network"),
        );

        assert!(payload.contains("Notification rule packet"));
        assert!(payload.contains("Current Matrix mode: mentions and keywords"));
        assert!(payload.contains("Loaded attention: 3 unread, 1 mentions, manual unread"));
        assert!(payload.contains("requested mode Mute"));
        assert!(payload.contains("retry cache ready"));
        assert!(payload.contains("Room mode request/result/error/retry"));
        assert!(payload.contains("Timed mute request/result/error/retry"));
        assert!(payload.contains("Default room-mode read/write request/result/error/retry"));
        assert!(payload.contains("Pusher/device request/result/error/retry"));
        assert!(payload.contains("Sound/badge request/result/error/retry"));
        assert!(payload.contains("NotificationSettings::get_default_room_notification_mode"));
        assert!(payload.contains("SetDefaultRoomNotificationMode"));
        assert!(payload.contains("remaining timed mute, raw account-data, pusher"));
        assert!(payload.contains(NOTIFICATIONS_RULE_PACKET_DRILLDOWN_LABEL));
        assert!(payload.contains("No Matrix notification rule account-data read or write outside SDK keyword/default APIs"));
        assert!(payload.contains("no unconfirmed SetRoomNotificationMode"));
        assert!(payload.contains("SetDefaultRoomNotificationMode"));
        assert!(payload.contains("no retry automation"));
    }

    #[test]
    fn notifications_rule_packet_clipboard_label_is_local_only() {
        let label = notifications_rule_packet_clipboard_label(
            "Hepta UI",
            true,
            "Current Matrix mode: muted",
            "Loaded attention: 3 unread, 1 mentions, manual unread",
            Some("Mute"),
            true,
            true,
            Some("packet copied"),
        );

        assert!(label.contains("packet copied"));
        assert!(label.contains("requested mode Mute"));
        assert!(label.contains("retry cache ready"));
        assert!(label.contains(NOTIFICATIONS_RULE_PACKET_DRILLDOWN_LABEL));
        assert!(label.contains("No notification rule account-data read/write"));
        assert!(label.contains("push-rule write"));
        assert!(label.contains("pusher mutation"));
        assert!(label.contains("unconfirmed SetRoomNotificationMode"));
        assert!(label.contains("live mutation"));
    }

    #[test]
    fn notifications_rule_contract_packet_payload_lists_typed_contract_slots() {
        let payload = notifications_rule_contract_packet_payload(
            "Hepta UI",
            "Current Matrix mode: mentions and keywords",
            "Loaded attention: 3 unread, 1 mentions, manual unread",
            Some("Mute"),
            true,
            true,
            Some("Update failed: network"),
        );

        assert!(payload.contains("Notification account-data/pusher typed contract"));
        assert!(payload.contains("Current Matrix mode: mentions and keywords"));
        assert!(payload.contains("requested mode Mute"));
        assert!(payload.contains("Account-data rules"));
        assert!(payload.contains("Push-rule keywords"));
        assert!(payload.contains("Pusher/device"));
        assert!(payload.contains("Default room mode"));
        assert!(payload.contains("Timed mute"));
        assert!(payload.contains("Sound/badge/result reconciliation"));
        assert!(payload.contains("remaining raw account-data, pusher, sound/badge, timed mute"));
        assert!(payload.contains(NOTIFICATIONS_RULE_CONTRACT_PACKET_LABEL));
        assert!(payload.contains("No Matrix notification rule account-data read or write outside SDK keyword/default APIs"));
        assert!(payload.contains("no unconfirmed SetRoomNotificationMode"));
        assert!(payload.contains("SetDefaultRoomNotificationMode"));
        assert!(payload.contains("no retry automation"));
    }

    #[test]
    fn notifications_rule_contract_packet_clipboard_label_is_local_only() {
        let label = notifications_rule_contract_packet_clipboard_label(
            "Hepta UI",
            true,
            "Current Matrix mode: muted",
            "Loaded attention: 3 unread, 1 mentions, manual unread",
            Some("Mute"),
            true,
            true,
            Some("contract copied"),
        );

        assert!(label.contains("contract copied"));
        assert!(label.contains("requested mode Mute"));
        assert!(label.contains("retry cache ready"));
        assert!(label.contains(NOTIFICATIONS_RULE_CONTRACT_PACKET_LABEL));
        assert!(label.contains("No notification account-data read/write"));
        assert!(label.contains("push-rule write"));
        assert!(label.contains("pusher mutation"));
        assert!(label.contains("unconfirmed SetRoomNotificationMode"));
        assert!(label.contains("live mutation"));
    }

    #[test]
    fn notifications_result_taxonomy_packet_payload_lists_unwired_result_slots() {
        let payload = notifications_result_taxonomy_packet_payload(
            "Hepta UI",
            "Current Matrix mode: mentions and keywords",
            "Loaded attention: 3 unread, 1 mentions, manual unread",
            Some("Mute"),
            true,
            true,
            Some("Update failed: network"),
        );

        assert!(payload.contains("Notification timed/global/pusher result taxonomy packet"));
        assert!(payload.contains("requested mode Mute"));
        assert!(payload.contains("Live result references"));
        assert!(payload.contains("SetRoomNotificationMode"));
        assert!(payload.contains("SetNotificationKeywordRule"));
        assert!(payload.contains("SetDefaultRoomNotificationMode"));
        assert!(payload.contains("timed_mute_operation_id: not_assigned"));
        assert!(
            payload.contains(
                "timed_mute_result: scheduled, applied, expired, failed, stale not_wired"
            )
        );
        assert!(payload.contains("raw_account_data_operation_id: not_assigned"));
        assert!(
            payload.contains("pusher_device_result: enabled, disabled, failed, stale not_wired")
        );
        assert!(payload.contains("sound_badge_result: applied, failed, stale not_wired"));
        assert!(payload.contains("backend_generation_required_before_raw_rule_or_pusher_write"));
        assert!(payload.contains(NOTIFICATIONS_RESULT_TAXONOMY_PACKET_LABEL));
        assert!(payload.contains("No Matrix notification rule account-data read or write outside SDK keyword/default APIs"));
        assert!(payload.contains("no pusher mutation"));
        assert!(payload.contains("no timed mute write"));
        assert!(payload.contains("no retry automation"));
    }

    #[test]
    fn notifications_result_taxonomy_packet_clipboard_label_is_local_only() {
        let label = notifications_result_taxonomy_packet_clipboard_label(
            "Hepta UI",
            true,
            "Current Matrix mode: muted",
            "Loaded attention: 3 unread, 1 mentions, manual unread",
            Some("Mute"),
            true,
            true,
            Some("taxonomy copied"),
        );

        assert!(label.contains("taxonomy copied"));
        assert!(label.contains("requested mode Mute"));
        assert!(label.contains("retry cache ready"));
        assert!(label.contains(NOTIFICATIONS_RESULT_TAXONOMY_PACKET_LABEL));
        assert!(label.contains("No notification account-data read/write"));
        assert!(label.contains("pusher mutation"));
        assert!(label.contains("timed mute write"));
        assert!(label.contains("sound/badge tuning"));
        assert!(label.contains("unconfirmed SetRoomNotificationMode"));
        assert!(label.contains("live mutation"));
    }

    #[test]
    fn notifications_preflight_detail_control_label_reports_waiting_state() {
        let label = notifications_preflight_detail_control_label(
            "this chat",
            Some("   "),
            "Current Matrix mode: loading read-only",
            "Loaded attention: waiting for room-list unread state",
            None,
            false,
            false,
            None,
        );

        assert!(label.contains("no notification preflight selected"));
        assert!(label.contains("no requested mode staged"));
        assert!(label.contains("retry cache empty"));
        assert!(label.contains("timeline waiting for current mode"));
        assert!(label.contains("local status 0 chars"));
        assert!(
            label.contains(
                "Schedule, Packet, Contract, Account data, Keywords, Pushers, and Defaults"
            )
        );
    }

    #[test]
    fn notifications_retry_confirmation_label_keeps_global_gaps_explicit() {
        let label = notifications_retry_confirmation_label("Hepta UI", "Mute");

        assert!(label.contains("Retry notification mode update for Hepta UI"));
        assert!(label.contains("Mute"));
        assert!(label.contains("Retry confirms before SetRoomNotificationMode"));
        assert!(label.contains("timed mute"));
        assert!(label.contains("global preference"));
        assert!(label.contains("push gateway/device"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
    }

    #[test]
    fn notifications_mode_target_metadata_label_summarizes_staged_mode() {
        let label = notifications_mode_target_metadata_label(
            "Hepta UI",
            "Current Matrix mode: mentions and keywords",
            Some("Mute"),
            true,
            true,
            true,
            Some("Confirmation open: Mute"),
        );

        assert!(label.contains("Notification target metadata for Hepta UI"));
        assert!(label.contains("Current Matrix mode: mentions and keywords"));
        assert!(label.contains("requested mode Mute"));
        assert!(label.contains("loaded attention ready"));
        assert!(label.contains("retry cache ready"));
        assert!(label.contains("timeline loaded"));
        assert!(label.contains("local status Confirmation open: Mute"));
        assert!(label.contains("SetRoomNotificationMode waits for confirmation"));
    }

    #[test]
    fn notifications_mode_target_metadata_label_reports_waiting_state() {
        let label = notifications_mode_target_metadata_label(
            "this chat",
            "Current Matrix mode: loading read-only",
            None,
            false,
            false,
            false,
            None,
        );

        assert!(label.contains("no requested mode staged"));
        assert!(label.contains("loaded attention waiting"));
        assert!(label.contains("retry cache empty"));
        assert!(label.contains("timeline waiting"));
        assert!(label.contains("no local status staged"));
        assert!(label.contains("Mode target metadata is local"));
    }
}

#[cfg(test)]
mod message_search_metadata_tests {
    use super::*;

    #[test]
    fn loaded_message_search_query_lifecycle_label_summarizes_local_reset() {
        let label = loaded_message_search_query_lifecycle_label(" Launch ", 42, 3, 0, true, true);

        assert!(label.contains("Search query lifecycle metadata"));
        assert!(label.contains("surface visible"));
        assert!(label.contains("query 6 chars"));
        assert!(label.contains("normalized local token \"launch\""));
        assert!(label.contains("timeline loaded"));
        assert!(label.contains("42 loaded items"));
        assert!(label.contains("3 local matches"));
        assert!(label.contains("active index 1 of 3"));
        assert!(label.contains("Query edits reset active_match to 0"));
        assert!(label.contains("Close/Escape clears query and matches locally"));
        assert!(label.contains(MESSAGE_SEARCH_QUERY_LIFECYCLE_METADATA_LABEL));
        assert!(label.contains(MESSAGE_SEARCH_COMPACT_LABEL));
        assert!(label.contains("No Matrix-backed search"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(
            MESSAGE_SEARCH_QUERY_LIFECYCLE_METADATA_EVIDENCE.contains("query lifecycle metadata")
        );
        assert!(MESSAGE_SEARCH_QUERY_LIFECYCLE_METADATA_EVIDENCE.contains("resets active_match"));
        assert!(
            MESSAGE_SEARCH_QUERY_LIFECYCLE_METADATA_EVIDENCE
                .contains("already loaded RoomScreen tl_state")
        );
        assert!(MESSAGE_SEARCH_QUERY_LIFECYCLE_METADATA_EVIDENCE.contains("Close/Escape clears"));
        assert!(
            MESSAGE_SEARCH_QUERY_LIFECYCLE_METADATA_EVIDENCE.contains("server-side history query")
        );
    }

    #[test]
    fn loaded_message_search_query_lifecycle_label_reports_empty_hidden_state() {
        let label = loaded_message_search_query_lifecycle_label("   ", 0, 0, 0, false, false);

        assert!(label.contains("surface hidden"));
        assert!(label.contains("query empty"));
        assert!(label.contains("timeline waiting"));
        assert!(label.contains("0 loaded items"));
        assert!(label.contains("0 local matches"));
        assert!(label.contains("active index reset to 0 with no local match"));
    }

    #[test]
    fn loaded_message_search_metadata_label_summarizes_active_match() {
        let label = loaded_message_search_metadata_label("launch", 42, 3, 1, Some(17), true);

        assert!(label.contains("query 6 chars"));
        assert!(label.contains("42 loaded items"));
        assert!(label.contains("3 matches"));
        assert!(label.contains("active 2/3 at loaded index 17"));
        assert!(label.contains("event id loaded"));
        assert!(label.contains(MESSAGE_SEARCH_LOADED_METADATA_LABEL));
    }

    #[test]
    fn loaded_message_search_metadata_label_reports_empty_query_without_active_match() {
        let label = loaded_message_search_metadata_label("   ", 8, 0, 0, None, false);

        assert!(label.contains("query empty"));
        assert!(label.contains("8 loaded items"));
        assert!(label.contains("0 matches"));
        assert!(label.contains("no active match"));
        assert!(label.contains("no server-side search"));
    }

    #[test]
    fn loaded_message_search_active_result_label_summarizes_loaded_match() {
        let label = loaded_message_search_active_result_label(
            "launch",
            4,
            2,
            Some(17),
            Some("$event:example.org"),
            Some("Launch plan ready. Launch review remains local."),
        );

        assert!(label.contains("active 3/4"));
        assert!(label.contains("loaded index 17"));
        assert!(label.contains("event id $event:example.org"));
        assert!(label.contains("query 6 chars"));
        assert!(label.contains("2 local occurrence(s)"));
        assert!(label.contains("snippet: Launch plan ready"));
        assert!(label.contains(MESSAGE_SEARCH_ACTIVE_RESULT_DETAIL_LABEL));
    }

    #[test]
    fn loaded_message_search_active_result_label_keeps_boundaries_explicit() {
        let empty = loaded_message_search_active_result_label("   ", 0, 0, None, None, None);
        let none = loaded_message_search_active_result_label("launch", 0, 0, None, None, None);

        assert!(empty.contains("query empty"));
        assert!(none.contains("no loaded local match"));
        assert!(none.contains("Close/Escape stays local"));
        assert!(none.contains("no server-side search"));
        assert!(none.contains("event context"));
        assert!(none.contains("pagination"));
        assert!(none.contains("mutation"));
    }

    #[test]
    fn loaded_message_search_result_action_controls_label_keeps_actions_local() {
        let label = loaded_message_search_result_action_controls_label(
            Some("Sender"),
            "launch",
            42,
            3,
            1,
            Some(17),
            true,
            Some("Launch plan ready. Launch review remains local."),
        );

        assert!(label.contains("Sender selected"));
        assert!(label.contains("controls Jump, Copy, Source, Thread, Sender"));
        assert!(label.contains("query 6 chars"));
        assert!(label.contains("42 loaded items"));
        assert!(label.contains("active 2 of 3"));
        assert!(label.contains("loaded index 17"));
        assert!(label.contains("event id loaded"));
        assert!(label.contains("snippet: Launch plan ready"));
        assert!(label.contains(MESSAGE_SEARCH_RESULT_ACTION_CONTROLS_LABEL));
        assert!(label.contains("profile mutation"));
        assert!(label.contains("Matrix-backed search"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(
            MESSAGE_SEARCH_RESULT_ACTION_CONTROLS_EVIDENCE
                .contains("Jump, Copy, Source, Thread, and Sender")
        );
        assert!(
            MESSAGE_SEARCH_RESULT_ACTION_CONTROLS_EVIDENCE
                .contains("current loaded timeline match")
        );
        assert!(
            MESSAGE_SEARCH_RESULT_ACTION_CONTROLS_EVIDENCE
                .contains("Jump scrolls/highlights the active loaded match locally")
        );
        assert!(
            MESSAGE_SEARCH_RESULT_ACTION_CONTROLS_EVIDENCE
                .contains("Source opens the existing local EventSourceModal")
        );
        assert!(
            MESSAGE_SEARCH_RESULT_ACTION_CONTROLS_EVIDENCE
                .contains("Thread opens the existing thread-focused timeline path")
        );
        assert!(
            MESSAGE_SEARCH_RESULT_ACTION_CONTROLS_EVIDENCE
                .contains("Sender opens the existing UserProfileSlidingPane")
        );
    }

    #[test]
    fn loaded_message_search_result_action_controls_label_reports_empty_state() {
        let label = loaded_message_search_result_action_controls_label(
            None, "   ", 0, 0, 0, None, false, None,
        );

        assert!(label.contains("no result action selected"));
        assert!(label.contains("query empty"));
        assert!(label.contains("0 loaded items"));
        assert!(label.contains("no active loaded match"));
        assert!(label.contains("loaded index missing"));
        assert!(label.contains("event id missing"));
        assert!(label.contains("loaded snippet unavailable"));
    }

    #[test]
    fn loaded_message_search_result_jump_loaded_match_label_summarizes_loaded_jump() {
        let label = loaded_message_search_result_jump_loaded_match_label(
            "launch",
            42,
            3,
            1,
            Some(17),
            true,
            Some("Launch plan ready. Launch review remains local."),
            true,
        );

        assert!(label.contains("Search result Jump"));
        assert!(label.contains("scrolled and queued highlight"));
        assert!(label.contains("query 6 chars"));
        assert!(label.contains("42 loaded items"));
        assert!(label.contains("active 2 of 3"));
        assert!(label.contains("loaded index 17"));
        assert!(label.contains("event id loaded"));
        assert!(label.contains("snippet: Launch plan ready"));
        assert!(label.contains(MESSAGE_SEARCH_RESULT_JUMP_LOADED_MATCH_LABEL));
        assert!(label.contains("No Matrix-backed search"));
        assert!(label.contains("server-side history query"));
        assert!(label.contains("MatrixRequest::PaginateTimeline"));
        assert!(label.contains("thread timeline open"));
        assert!(label.contains("sender/profile lookup"));
        assert!(MESSAGE_SEARCH_RESULT_JUMP_LOADED_MATCH_EVIDENCE.contains("PortalList"));
        assert!(
            MESSAGE_SEARCH_RESULT_JUMP_LOADED_MATCH_EVIDENCE
                .contains("message highlight animation")
        );
    }

    #[test]
    fn loaded_message_search_result_jump_loaded_match_label_reports_missing_jump() {
        let label = loaded_message_search_result_jump_loaded_match_label(
            "   ", 0, 0, 0, None, false, None, false,
        );

        assert!(label.contains("jump unavailable"));
        assert!(label.contains("query empty"));
        assert!(label.contains("0 loaded items"));
        assert!(label.contains("no active loaded match"));
        assert!(label.contains("loaded index missing"));
        assert!(label.contains("event id missing"));
        assert!(label.contains("loaded snippet unavailable"));
    }

    #[test]
    fn loaded_message_search_result_thread_open_label_summarizes_loaded_thread() {
        let label = loaded_message_search_result_thread_open_label(
            "launch",
            42,
            3,
            1,
            Some(17),
            true,
            Some("Launch plan ready. Thread review remains local."),
            Some("$thread:example.org"),
            true,
        );

        assert!(label.contains("Search result Thread"));
        assert!(label.contains("thread timeline selected"));
        assert!(label.contains("query 6 chars"));
        assert!(label.contains("42 loaded items"));
        assert!(label.contains("active 2 of 3"));
        assert!(label.contains("loaded index 17"));
        assert!(label.contains("event id loaded"));
        assert!(label.contains("thread root $thread:example.org"));
        assert!(label.contains("snippet: Launch plan ready"));
        assert!(label.contains(MESSAGE_SEARCH_RESULT_THREAD_OPEN_LABEL));
        assert!(label.contains("RoomsListAction::Selected"));
        assert!(label.contains("SelectedRoom::Thread"));
        assert!(label.contains("CreateThreadTimeline"));
        assert!(label.contains("No Matrix-backed search"));
        assert!(label.contains("MatrixRequest::PaginateTimeline"));
        assert!(label.contains("sender/profile lookup"));
        assert!(MESSAGE_SEARCH_RESULT_THREAD_OPEN_EVIDENCE.contains("MsgLikeContent.thread_root"));
        assert!(MESSAGE_SEARCH_RESULT_THREAD_OPEN_EVIDENCE.contains("SelectedRoom::Thread"));
    }

    #[test]
    fn loaded_message_search_result_thread_open_label_reports_missing_thread() {
        let label = loaded_message_search_result_thread_open_label(
            "   ", 0, 0, 0, None, false, None, None, false,
        );

        assert!(label.contains("thread open unavailable"));
        assert!(label.contains("query empty"));
        assert!(label.contains("0 loaded items"));
        assert!(label.contains("no active loaded match"));
        assert!(label.contains("loaded index missing"));
        assert!(label.contains("event id missing"));
        assert!(label.contains("thread root unavailable"));
        assert!(label.contains("loaded snippet unavailable"));
    }

    #[test]
    fn loaded_message_search_result_sender_profile_pane_label_summarizes_sender() {
        let label = loaded_message_search_result_sender_profile_pane_label(
            "launch",
            42,
            3,
            1,
            Some(17),
            true,
            Some("Launch plan ready. Sender review remains local."),
            Some("@alice:example.org"),
            Some("Alice"),
            true,
            true,
            true,
        );

        assert!(label.contains("Search result Sender"));
        assert!(label.contains("profile pane opened"));
        assert!(label.contains("query 6 chars"));
        assert!(label.contains("42 loaded items"));
        assert!(label.contains("active 2 of 3"));
        assert!(label.contains("loaded index 17"));
        assert!(label.contains("event id loaded"));
        assert!(label.contains("sender @alice:example.org"));
        assert!(label.contains("display \"Alice\""));
        assert!(label.contains("loaded sender_profile ready"));
        assert!(label.contains("local room member loaded"));
        assert!(label.contains("snippet: Launch plan ready"));
        assert!(label.contains(MESSAGE_SEARCH_RESULT_SENDER_PROFILE_PANE_LABEL));
        assert!(label.contains("UserProfileSlidingPane"));
        assert!(label.contains("GetUserProfile"));
        assert!(label.contains("No Matrix-backed search"));
        assert!(label.contains("MatrixRequest::PaginateTimeline"));
        assert!(label.contains("profile mutation"));
        assert!(MESSAGE_SEARCH_RESULT_SENDER_PROFILE_PANE_EVIDENCE.contains("sender_profile"));
        assert!(
            MESSAGE_SEARCH_RESULT_SENDER_PROFILE_PANE_EVIDENCE
                .contains("GetUserProfile/profile-member read path")
        );
    }

    #[test]
    fn loaded_message_search_result_sender_profile_pane_label_reports_missing_sender() {
        let label = loaded_message_search_result_sender_profile_pane_label(
            "   ", 0, 0, 0, None, false, None, None, None, false, false, false,
        );

        assert!(label.contains("profile pane unavailable"));
        assert!(label.contains("query empty"));
        assert!(label.contains("0 loaded items"));
        assert!(label.contains("no active loaded match"));
        assert!(label.contains("loaded index missing"));
        assert!(label.contains("event id missing"));
        assert!(label.contains("sender unavailable"));
        assert!(label.contains("display unavailable"));
        assert!(label.contains("sender_profile not ready"));
        assert!(label.contains("local room member missing"));
        assert!(label.contains("loaded snippet unavailable"));
    }

    #[test]
    fn loaded_message_search_result_copy_clipboard_label_summarizes_loaded_payload() {
        let label = loaded_message_search_result_copy_clipboard_label(
            "launch",
            42,
            3,
            1,
            Some(17),
            true,
            Some("Launch plan ready. Launch review remains local."),
        );

        assert!(label.contains("Search result Copy clipboard"));
        assert!(label.contains("copied loaded plaintext"));
        assert!(label.contains("query 6 chars"));
        assert!(label.contains("42 loaded items"));
        assert!(label.contains("active 2 of 3"));
        assert!(label.contains("loaded index 17"));
        assert!(label.contains("event id loaded"));
        assert!(label.contains("47 chars"));
        assert!(label.contains("47 bytes"));
        assert!(label.contains("snippet: Launch plan ready"));
        assert!(label.contains(MESSAGE_SEARCH_RESULT_COPY_CLIPBOARD_LABEL));
        assert!(label.contains("No Matrix-backed search"));
        assert!(label.contains("server-side history query"));
        assert!(label.contains("MatrixRequest::PaginateTimeline"));
        assert!(label.contains("sender/profile lookup"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(MESSAGE_SEARCH_RESULT_COPY_CLIPBOARD_EVIDENCE.contains("local clipboard"));
        assert!(
            MESSAGE_SEARCH_RESULT_COPY_CLIPBOARD_EVIDENCE
                .contains("plaintext_body_of_timeline_item")
        );
    }

    #[test]
    fn loaded_message_search_result_copy_clipboard_label_reports_missing_match() {
        let label =
            loaded_message_search_result_copy_clipboard_label("   ", 0, 0, 0, None, false, None);

        assert!(label.contains("copy unavailable"));
        assert!(label.contains("query empty"));
        assert!(label.contains("0 loaded items"));
        assert!(label.contains("no active loaded match"));
        assert!(label.contains("loaded index missing"));
        assert!(label.contains("event id missing"));
        assert!(label.contains("0 chars"));
        assert!(label.contains("0 bytes"));
        assert!(label.contains("loaded snippet unavailable"));
    }

    #[test]
    fn loaded_message_search_result_source_modal_label_summarizes_loaded_source() {
        let label = loaded_message_search_result_source_modal_label(
            "launch",
            42,
            3,
            1,
            Some(17),
            true,
            Some("{\n  \"type\": \"m.room.message\"\n}"),
            None,
            true,
            false,
        );

        assert!(label.contains("Search result Source modal"));
        assert!(label.contains("loaded source modal opened"));
        assert!(label.contains("query \"launch\""));
        assert!(label.contains("42 loaded items"));
        assert!(label.contains("active match 2 of 3"));
        assert!(label.contains("loaded index 17"));
        assert!(label.contains("event id loaded"));
        assert!(label.contains("latest JSON"));
        assert!(label.contains("across 3 lines"));
        assert!(label.contains(MESSAGE_SEARCH_RESULT_SOURCE_MODAL_LABEL));
        assert!(label.contains("MatrixRequest::FetchEventSource"));
        assert!(label.contains("no new Matrix-backed search"));
        assert!(label.contains("event context fetch"));
        assert!(label.contains("MatrixRequest::PaginateTimeline"));
        assert!(label.contains("thread timeline open"));
        assert!(label.contains("sender/profile lookup"));
        assert!(MESSAGE_SEARCH_RESULT_SOURCE_MODAL_EVIDENCE.contains("EventSourceModal"));
        assert!(
            MESSAGE_SEARCH_RESULT_SOURCE_MODAL_EVIDENCE
                .contains("cached Matrix /search server result")
        );
    }

    #[test]
    fn loaded_message_search_result_source_modal_label_summarizes_server_source() {
        let label = loaded_message_search_result_source_modal_label(
            "launch",
            42,
            3,
            1,
            Some(17),
            false,
            None,
            Some("{\n  \"event_id\": \"$remote:example.org\"\n}"),
            true,
            false,
        );

        assert!(label.contains("server-result source modal opened"));
        assert!(label.contains("server result JSON"));
        assert!(label.contains("across 3 lines"));
        assert!(label.contains("no new Matrix-backed search"));
        assert!(MESSAGE_SEARCH_RESULT_SOURCE_MODAL_EVIDENCE.contains("raw event JSON cached"));
    }

    #[test]
    fn loaded_message_search_result_source_modal_label_summarizes_source_refetch() {
        let label = loaded_message_search_result_source_modal_label(
            "launch",
            42,
            3,
            1,
            Some(17),
            true,
            None,
            None,
            false,
            true,
        );

        assert!(label.contains("source-only MatrixRequest::FetchEventSource requested"));
        assert!(label.contains("source fetch pending"));
        assert!(label.contains("current-room event JSON"));
        assert!(MESSAGE_SEARCH_RESULT_SOURCE_MODAL_EVIDENCE.contains("Room::load_or_fetch_event"));
    }

    #[test]
    fn loaded_message_search_result_source_modal_label_reports_missing_source() {
        let label = loaded_message_search_result_source_modal_label(
            "   ", 0, 0, 0, None, false, None, None, false, false,
        );

        assert!(label.contains("source unavailable"));
        assert!(label.contains("query empty"));
        assert!(label.contains("0 loaded items"));
        assert!(label.contains("no active match"));
        assert!(label.contains("no loaded index"));
        assert!(label.contains("event id unavailable"));
        assert!(label.contains("latest JSON unavailable"));
    }

    #[test]
    fn loaded_message_search_server_context_boundary_reports_live_cursor_state() {
        let label =
            loaded_message_search_server_context_boundary_label("launch", 42, 3, true, true);

        assert!(label.contains("Server/context boundary"));
        assert!(label.contains("query 6 chars"));
        assert!(label.contains("42 loaded items"));
        assert!(label.contains("3 local matches"));
        assert!(label.contains("timeline loaded"));
        assert!(label.contains("next_batch cursor available"));
        assert!(label.contains(MESSAGE_SEARCH_SERVER_CONTEXT_BOUNDARY_LABEL));
        assert!(label.contains("server context-window previews are parsed"));
        assert!(label.contains("Context uses cached current-room server hit events"));
        assert!(label.contains("MatrixRequest::PaginateTimeline"));
        assert!(label.contains("Source can open cached Matrix /search raw event JSON"));
        assert!(label.contains("MatrixRequest::FetchEventSource"));
        assert!(label.contains("full result rendering"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(MESSAGE_SEARCH_SERVER_CONTEXT_BOUNDARY_EVIDENCE.contains("Server submits"));
        assert!(
            MESSAGE_SEARCH_SERVER_CONTEXT_BOUNDARY_EVIDENCE
                .contains("event_context before/after preview snippets")
        );
        assert!(
            MESSAGE_SEARCH_SERVER_CONTEXT_BOUNDARY_EVIDENCE
                .contains("Load older submits the returned next_batch")
        );
        assert!(
            MESSAGE_SEARCH_SERVER_CONTEXT_BOUNDARY_EVIDENCE
                .contains("Context can use the first cached current-room server hit")
        );
    }

    #[test]
    fn loaded_message_search_server_context_boundary_reports_waiting_state() {
        let label = loaded_message_search_server_context_boundary_label("   ", 0, 0, false, false);

        assert!(label.contains("query empty"));
        assert!(label.contains("0 loaded items"));
        assert!(label.contains("0 local matches"));
        assert!(label.contains("timeline waiting"));
        assert!(label.contains("next_batch cursor unavailable"));
        assert!(label.contains("Server/Older use live Matrix search reads"));
        assert!(label.contains("server context-window previews are parsed"));
    }

    #[test]
    fn loaded_message_search_server_context_controls_label_reports_live_older() {
        let label = loaded_message_search_server_context_controls_label(
            Some("Load older"),
            "launch",
            42,
            3,
            1,
            true,
            true,
        );

        assert!(label.contains("Load older selected"));
        assert!(label.contains("query 6 chars"));
        assert!(label.contains("42 loaded items"));
        assert!(label.contains("3 local matches"));
        assert!(label.contains("active index 2 of 3 local match"));
        assert!(label.contains("timeline loaded"));
        assert!(label.contains("Older can submit next_batch"));
        assert!(label.contains(MESSAGE_SEARCH_SERVER_CONTEXT_CONTROLS_LABEL));
        assert!(label.contains("server context-window previews are parsed"));
        assert!(label.contains("Context reuses BackwardsPaginateUntilEvent"));
        assert!(label.contains("MatrixRequest::PaginateTimeline"));
        assert!(label.contains("Source can open cached Matrix /search raw event JSON"));
        assert!(label.contains("MatrixRequest::FetchEventSource"));
        assert!(label.contains("full result rendering"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(
            MESSAGE_SEARCH_SERVER_CONTEXT_CONTROLS_EVIDENCE
                .contains("Server submits the first live MatrixRequest::SearchMessagesServer page")
        );
        assert!(
            MESSAGE_SEARCH_SERVER_CONTEXT_CONTROLS_EVIDENCE
                .contains("Older submits the returned next_batch cursor")
        );
        assert!(
            MESSAGE_SEARCH_SERVER_CONTEXT_CONTROLS_EVIDENCE
                .contains("event_context before/after windows")
        );
        assert!(
            MESSAGE_SEARCH_SERVER_CONTEXT_CONTROLS_EVIDENCE
                .contains("Context parses the first cached server hit event id")
        );
    }

    #[test]
    fn loaded_message_search_server_context_controls_label_reports_empty_state() {
        let label =
            loaded_message_search_server_context_controls_label(None, "   ", 0, 0, 0, false, false);

        assert!(label.contains("no server/context control selected"));
        assert!(label.contains("query empty"));
        assert!(label.contains("0 loaded items"));
        assert!(label.contains("0 local matches"));
        assert!(label.contains("active index reset to 0"));
        assert!(label.contains("timeline waiting"));
        assert!(label.contains("Older waits for next_batch"));
    }

    #[test]
    fn loaded_message_search_advanced_filter_controls_label_splits_live_media_filter() {
        let label = loaded_message_search_advanced_filter_controls_label(
            Some("Media"),
            "launch",
            42,
            3,
            1,
            true,
            2,
            MessageSearchLoadedScope::PinnedLoaded,
        );

        assert!(label.contains("Media selected"));
        assert!(label.contains("Filter, From, Date, Media, Pins"));
        assert!(label.contains("query 6 chars"));
        assert!(label.contains("42 loaded items"));
        assert!(label.contains("3 local matches"));
        assert!(label.contains("active index 2 of 3 local match"));
        assert!(label.contains("timeline loaded"));
        assert!(label.contains("2 subscribed pinned event"));
        assert!(label.contains("loaded scope loaded pinned events"));
        assert!(label.contains(MESSAGE_SEARCH_ADVANCED_FILTER_CONTROLS_LABEL));
        assert!(label.contains("From can submit MatrixRequest::SearchMessagesServer"));
        assert!(label.contains("RoomEventFilter::senders"));
        assert!(label.contains("Media can submit MatrixRequest::SearchMessagesServer"));
        assert!(label.contains("RoomEventFilter::url_filter=EventsWithUrl"));
        assert!(label.contains("Date applies the latest loaded-day window"));
        assert!(label.contains("Pins applies the existing pinned-event subscription"));
        assert!(label.contains("remote date index query"));
        assert!(label.contains("pinned event fetch"));
        assert!(label.contains("PinEvent"));
        assert!(label.contains("MatrixRequest::PaginateTimeline"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(
            MESSAGE_SEARCH_ADVANCED_FILTER_CONTROLS_EVIDENCE
                .contains("From is the live sender filter")
        );
        assert!(
            MESSAGE_SEARCH_ADVANCED_FILTER_CONTROLS_EVIDENCE.contains("RoomEventFilter::senders")
        );
        assert!(
            MESSAGE_SEARCH_ADVANCED_FILTER_CONTROLS_EVIDENCE
                .contains("RoomEventFilter::url_filter=EventsWithUrl")
        );
        assert!(
            MESSAGE_SEARCH_ADVANCED_FILTER_CONTROLS_EVIDENCE.contains("Filter, Date, and Pins")
        );
        assert!(
            MESSAGE_SEARCH_ADVANCED_FILTER_CONTROLS_EVIDENCE.contains("live loaded-scope filters")
        );
    }

    #[test]
    fn loaded_message_search_advanced_filter_controls_label_reports_empty_state() {
        let label = loaded_message_search_advanced_filter_controls_label(
            None,
            "   ",
            0,
            0,
            0,
            false,
            0,
            MessageSearchLoadedScope::AllLoaded,
        );

        assert!(label.contains("no advanced filter selected"));
        assert!(label.contains("query empty"));
        assert!(label.contains("0 loaded items"));
        assert!(label.contains("0 local matches"));
        assert!(label.contains("active index reset to 0"));
        assert!(label.contains("timeline waiting"));
        assert!(label.contains("0 subscribed pinned event"));
        assert!(label.contains("loaded scope all loaded messages"));
    }

    #[test]
    fn message_search_loaded_scope_filter_label_summarizes_date_and_pins() {
        let date = message_search_loaded_scope_filter_label(
            "Date",
            MessageSearchLoadedScope::LatestLoadedDay,
            42,
            3,
            2,
        );
        let pins = message_search_loaded_scope_filter_label(
            "Pins",
            MessageSearchLoadedScope::PinnedLoaded,
            42,
            1,
            2,
        );

        assert!(date.contains("Date loaded-scope filter applied"));
        assert!(date.contains("latest loaded-day window"));
        assert!(date.contains("3 local match"));
        assert!(pins.contains("loaded pinned events"));
        assert!(pins.contains("existing pinned-event subscription"));
        assert!(pins.contains("no remote date index query"));
        assert!(pins.contains("pinned event fetch"));
        assert!(pins.contains("PinEvent"));
    }

    #[test]
    fn loaded_message_search_server_preflight_controls_label_keeps_metadata_local() {
        let label = loaded_message_search_server_preflight_controls_label(
            Some("Retry"),
            "launch",
            42,
            3,
            1,
            true,
            2,
            "Search server/context controls cached",
        );

        assert!(label.contains("Retry selected"));
        assert!(
            label.contains("Server query, Packet, Contract, Result, Error, Retry, Scope, Taxonomy")
        );
        assert!(label.contains("query 6 chars"));
        assert!(label.contains("42 loaded items"));
        assert!(label.contains("3 local matches"));
        assert!(label.contains("active index 2 of 3 local match"));
        assert!(label.contains("timeline loaded"));
        assert!(label.contains("2 subscribed pinned event"));
        assert!(label.contains("server/context metadata"));
        assert!(label.contains(MESSAGE_SEARCH_SERVER_PREFLIGHT_CONTROLS_LABEL));
        assert!(label.contains("Context pagination is owned by the server/context controls"));
        assert!(label.contains("timeline reload outside BackwardsPaginateUntilEvent"));
        assert!(label.contains("search scope fetch"));
        assert!(label.contains("sender/profile lookup"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
    }

    #[test]
    fn loaded_message_search_server_query_local_snapshot_label_summarizes_loaded_packet() {
        let label = loaded_message_search_server_query_local_snapshot_label(
            "launch",
            42,
            3,
            1,
            true,
            2,
            "Search server/context controls cached",
        );

        assert!(label.contains("Local message search server-query packet"));
        assert!(label.contains("query 6 chars"));
        assert!(label.contains("42 loaded items"));
        assert!(label.contains("3 local matches"));
        assert!(label.contains("active index 2 of 3 local match"));
        assert!(label.contains("timeline loaded for local preview"));
        assert!(label.contains("2 subscribed pinned event"));
        assert!(label.contains("server/context metadata"));
        assert!(label.contains("server request body not built"));
        assert!(label.contains("result cursor not allocated"));
        assert!(label.contains(MESSAGE_SEARCH_SERVER_PREFLIGHT_CONTROLS_LABEL));
        assert!(label.contains("Server query renders this loaded local request snapshot"));
        assert!(
            label.contains("No Matrix-backed search") || label.contains("no Matrix-backed search")
        );
        assert!(label.contains("server-side history query"));
        assert!(label.contains("event context fetch"));
        assert!(label.contains("MatrixRequest::PaginateTimeline"));
        assert!(label.contains("result pagination"));
        assert!(label.contains("search scope fetch"));
        assert!(label.contains("sender/profile lookup"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
    }

    #[test]
    fn message_search_server_packet_clipboard_payload_uses_loaded_state_only() {
        let payload = message_search_server_packet_clipboard_payload(
            "launch",
            42,
            3,
            1,
            true,
            2,
            "Search server/context controls cached",
            "Server query snapshot cached",
        );

        assert!(payload.contains("Message search server packet"));
        assert!(payload.contains("Query: launch"));
        assert!(payload.contains("Loaded items: 42"));
        assert!(payload.contains("Local matches: 3"));
        assert!(payload.contains("Active match: 2 of 3"));
        assert!(payload.contains("Timeline: loaded"));
        assert!(payload.contains("Pinned events: 2"));
        assert!(payload.contains("Server/context metadata: Search server/context controls cached"));
        assert!(payload.contains("Server preflight metadata: Server query snapshot cached"));
        assert!(payload.contains("Request body: not built"));
        assert!(payload.contains("Result cursor: not allocated"));
        assert!(payload.contains("no Matrix-backed search"));
        assert!(payload.contains("MatrixRequest::PaginateTimeline"));
        assert!(payload.contains("gateway/runtime/auth"));
        assert!(payload.contains("live mutation"));
    }

    #[test]
    fn message_search_server_packet_clipboard_label_reports_local_copy() {
        let label = message_search_server_packet_clipboard_label(
            true,
            "launch",
            42,
            3,
            "Message search server packet",
        );

        assert!(label.contains("copied local query/result packet to clipboard"));
        assert!(label.contains("query 6 chars"));
        assert!(label.contains("42 loaded items"));
        assert!(label.contains("3 local matches"));
        assert!(label.contains(MESSAGE_SEARCH_SERVER_PACKET_CLIPBOARD_LABEL));
        assert!(label.contains("No Matrix-backed search"));
        assert!(label.contains("MatrixRequest::PaginateTimeline"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(
            MESSAGE_SEARCH_SERVER_PACKET_CLIPBOARD_EVIDENCE
                .contains("local server query/result packet")
        );
        assert!(MESSAGE_SEARCH_SERVER_PACKET_CLIPBOARD_EVIDENCE.contains("result cursor"));
        assert!(MESSAGE_SEARCH_SERVER_PACKET_CLIPBOARD_EVIDENCE.contains("local clipboard"));
    }

    #[test]
    fn message_search_matrix_contract_acceptance_label_maps_packet_to_typed_slots() {
        let label = message_search_matrix_contract_acceptance_label(
            "launch",
            42,
            3,
            1,
            true,
            2,
            "Search server/context controls cached",
            "Server packet metadata cached",
        );

        assert!(label.contains("Matrix search acceptance contract stayed local"));
        assert!(label.contains("query 6 chars"));
        assert!(label.contains("42 loaded items"));
        assert!(label.contains("3 local matches"));
        assert!(label.contains("active index 2 of 3 local match"));
        assert!(label.contains("timeline loaded for local contract"));
        assert!(label.contains("2 subscribed pinned event"));
        assert!(label.contains("server/context metadata"));
        assert!(label.contains("packet/preflight metadata"));
        assert!(label.contains("Request slots"));
        assert!(label.contains("room scope"));
        assert!(label.contains("next_batch cursor"));
        assert!(label.contains("Result slots"));
        assert!(label.contains("event id"));
        assert!(label.contains("highlights"));
        assert!(label.contains("Error slots"));
        assert!(label.contains("rate-limited"));
        assert!(label.contains("Retry slots"));
        assert!(label.contains("idempotency"));
        assert!(label.contains(MESSAGE_SEARCH_MATRIX_CONTRACT_PACKET_LABEL));
        assert!(label.contains("No Matrix search request body"));
        assert!(label.contains("result cursor allocation"));
        assert!(label.contains("MatrixRequest::PaginateTimeline"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
    }

    #[test]
    fn message_search_matrix_contract_evidence_names_typed_boundary() {
        assert!(
            MESSAGE_SEARCH_MATRIX_CONTRACT_PACKET_EVIDENCE
                .contains("typed Matrix search acceptance contract")
        );
        assert!(
            MESSAGE_SEARCH_MATRIX_CONTRACT_PACKET_EVIDENCE.contains("request slots for room scope")
        );
        assert!(
            MESSAGE_SEARCH_MATRIX_CONTRACT_PACKET_EVIDENCE.contains("result slots for event id")
        );
        assert!(
            MESSAGE_SEARCH_MATRIX_CONTRACT_PACKET_EVIDENCE.contains("error slots for forbidden")
        );
        assert!(
            MESSAGE_SEARCH_MATRIX_CONTRACT_PACKET_EVIDENCE.contains("retry slots for confirmation")
        );
        assert!(
            MESSAGE_SEARCH_MATRIX_CONTRACT_PACKET_EVIDENCE
                .contains("builds no Matrix search request body")
        );
        assert!(MESSAGE_SEARCH_MATRIX_CONTRACT_PACKET_EVIDENCE.contains("result cursor"));
        assert!(MESSAGE_SEARCH_MATRIX_CONTRACT_PACKET_EVIDENCE.contains("live mutation"));
    }

    #[test]
    fn message_search_remote_result_taxonomy_packet_label_lists_blocked_adapters() {
        let label = message_search_remote_result_taxonomy_packet_label(
            "launch",
            42,
            3,
            1,
            true,
            2,
            "Search server/context controls cached",
            "Server packet metadata cached",
        );

        assert!(label.contains("Message search remote-result taxonomy packet stayed local"));
        assert!(label.contains("query 6 chars"));
        assert!(label.contains("42 loaded items"));
        assert!(label.contains("3 local matches"));
        assert!(label.contains("active index 2 of 3 local match"));
        assert!(label.contains("timeline loaded for taxonomy"));
        assert!(label.contains("2 subscribed pinned event"));
        assert!(label.contains("server/context metadata"));
        assert!(label.contains("packet/preflight metadata"));
        assert!(label.contains("MatrixRequest::SearchMessagesServer first page"));
        assert!(label.contains("next_batch Older pagination"));
        assert!(label.contains("BackwardsPaginateUntilEvent"));
        assert!(label.contains("remote_date_index_operation_id not_assigned"));
        assert!(label.contains("remote_pinned_fetch_operation_id not_assigned"));
        assert!(label.contains("cross_room_scope_request_id not_assigned"));
        assert!(label.contains("full_result_cursor_id not_assigned"));
        assert!(label.contains("full_result_render_result not_wired"));
        assert!(label.contains("audit_redaction"));
        assert!(label.contains(MESSAGE_SEARCH_REMOTE_RESULT_TAXONOMY_PACKET_LABEL));
        assert!(label.contains("No extra Matrix search"));
        assert!(label.contains("remote date index query"));
        assert!(label.contains("remote pinned event fetch"));
        assert!(label.contains("cross-room scope search"));
        assert!(label.contains("gateway/runtime/auth/provider"));
        assert!(label.contains("live mutation"));
    }

    #[test]
    fn message_search_remote_result_taxonomy_evidence_names_live_boundary() {
        assert!(
            MESSAGE_SEARCH_REMOTE_RESULT_TAXONOMY_PACKET_EVIDENCE
                .contains("local remote date/pins/scope/full-result taxonomy packet")
        );
        assert!(
            MESSAGE_SEARCH_REMOTE_RESULT_TAXONOMY_PACKET_EVIDENCE
                .contains("MatrixRequest::SearchMessagesServer first page")
        );
        assert!(
            MESSAGE_SEARCH_REMOTE_RESULT_TAXONOMY_PACKET_EVIDENCE
                .contains("SubscribeToPinnedEvents")
        );
        assert!(
            MESSAGE_SEARCH_REMOTE_RESULT_TAXONOMY_PACKET_EVIDENCE
                .contains("remote_date_index_operation_id")
        );
        assert!(
            MESSAGE_SEARCH_REMOTE_RESULT_TAXONOMY_PACKET_EVIDENCE
                .contains("cross_room_scope_request_id")
        );
        assert!(
            MESSAGE_SEARCH_REMOTE_RESULT_TAXONOMY_PACKET_EVIDENCE
                .contains("full_result_render_result")
        );
        assert!(
            MESSAGE_SEARCH_REMOTE_RESULT_TAXONOMY_PACKET_EVIDENCE
                .contains("no remote date index query")
        );
        assert!(
            MESSAGE_SEARCH_REMOTE_RESULT_TAXONOMY_PACKET_EVIDENCE
                .contains("no cross-room scope search")
        );
    }

    #[test]
    fn loaded_message_search_server_preflight_controls_label_reports_waiting_state() {
        let label = loaded_message_search_server_preflight_controls_label(
            None, "   ", 0, 0, 0, false, 0, "",
        );

        assert!(label.contains("no server preflight control selected"));
        assert!(label.contains("query empty"));
        assert!(label.contains("0 loaded items"));
        assert!(label.contains("0 local matches"));
        assert!(label.contains("active index reset to 0"));
        assert!(label.contains("timeline waiting"));
        assert!(label.contains("0 subscribed pinned event"));
        assert!(label.contains("server/context metadata empty"));
    }

    #[test]
    fn message_search_server_preflight_evidence_names_live_retry_boundary() {
        assert!(
            MESSAGE_SEARCH_SERVER_PREFLIGHT_CONTROLS_EVIDENCE.contains(
                "Server query, Packet, Contract, Result, Error, Retry, Scope, and Taxonomy"
            )
        );
        assert!(
            MESSAGE_SEARCH_SERVER_PREFLIGHT_CONTROLS_EVIDENCE
                .contains("Retry resubmits the current query from the first page")
        );
        assert!(
            MESSAGE_SEARCH_SERVER_PREFLIGHT_CONTROLS_EVIDENCE
                .contains("Older owns next_batch pagination")
        );
        assert!(
            MESSAGE_SEARCH_SERVER_PREFLIGHT_CONTROLS_EVIDENCE
                .contains("Context owns cached current-room hit pagination")
        );
        assert!(MESSAGE_SEARCH_SERVER_PREFLIGHT_CONTROLS_EVIDENCE.contains("search scope fetch"));
        assert!(MESSAGE_SEARCH_SERVER_PREFLIGHT_CONTROLS_EVIDENCE.contains("Taxonomy records"));
        assert!(
            MESSAGE_SEARCH_SERVER_PREFLIGHT_CONTROLS_LABEL.contains("live Matrix search reads")
        );
    }

    #[test]
    fn message_search_server_next_page_unavailable_label_explains_cursor_requirement() {
        let label = message_search_server_next_page_unavailable_label("needle", false, "M_LIMIT");

        assert!(label.contains("Older server search page unavailable"));
        assert!(label.contains("query 6 chars"));
        assert!(label.contains("server search idle"));
        assert!(label.contains("next_batch cursor missing"));
        assert!(label.contains("cached error 7 chars"));
        assert!(label.contains("Run Server first"));
        assert!(label.contains("Context pagination uses cached current-room hits only"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
    }

    #[test]
    fn message_search_server_context_event_unavailable_label_names_cached_hit_requirement() {
        let label = message_search_server_context_event_unavailable_label(
            "needle",
            false,
            0,
            "no cached server search hits",
        );

        assert!(label.contains("Search server Context unavailable"));
        assert!(label.contains("query 6 chars"));
        assert!(label.contains("server search idle"));
        assert!(label.contains("0 cached server hit"));
        assert!(label.contains("Run Server first"));
        assert!(label.contains("current-room hit with an event id"));
        assert!(label.contains("parsed Matrix /search context-window previews"));
        assert!(label.contains("Source can open cached Matrix /search raw event JSON"));
        assert!(label.contains("MatrixRequest::FetchEventSource"));
        assert!(label.contains("live mutation"));
    }

    #[test]
    fn message_search_server_context_event_request_label_names_live_pagination_path() {
        let event_id = EventId::parse("$search:example.org").unwrap();
        let label =
            message_search_server_context_event_request_label("needle", &event_id, 42, 3, false);

        assert!(label.contains("Search server Context requested"));
        assert!(label.contains("$search:example.org"));
        assert!(label.contains("42 loaded items"));
        assert!(label.contains("3 cached server hit"));
        assert!(label.contains("BackwardsPaginateUntilEvent requested"));
        assert!(label.contains("MatrixRequest::PaginateTimeline"));
        assert!(label.contains("current-room context only"));
        assert!(label.contains("parsed Matrix /search context-window previews"));
        assert!(label.contains("Source can open cached Matrix /search raw event JSON"));
        assert!(label.contains("MatrixRequest::FetchEventSource"));
    }

    #[test]
    fn message_search_server_context_event_found_label_summarizes_loaded_jump() {
        let event_id = EventId::parse("$search:example.org").unwrap();
        let label = message_search_server_context_event_found_label(
            "needle",
            &event_id,
            12,
            42,
            3,
            "Server hit body preview",
            true,
        );

        assert!(label.contains("Search server Context found"));
        assert!(label.contains("event $search:example.org"));
        assert!(label.contains("loaded index 12 of 42"));
        assert!(label.contains("3 cached server hit"));
        assert!(label.contains("found after BackwardsPaginateUntilEvent pagination"));
        assert!(label.contains("Current-room context jump is live"));
        assert!(label.contains("MatrixRequest::PaginateTimeline"));
        assert!(label.contains("parsed Matrix /search context-window previews"));
        assert!(label.contains("Source can open cached Matrix /search raw event JSON"));
        assert!(label.contains("MatrixRequest::FetchEventSource"));
    }

    #[test]
    fn message_search_server_result_label_summarizes_live_hits_and_cursor() {
        let response = MessageSearchServerResponse {
            query: "alice".to_string(),
            room_id: "!room:example.org".to_string(),
            filter: MessageSearchServerFilter {
                sender: Some("@alice:example.org".to_string()),
                media_only: true,
            },
            count: Some("12".to_string()),
            next_batch: Some("cursor".to_string()),
            highlights: vec!["alice".to_string()],
            hits: vec![MessageSearchServerHit {
                event_id: Some("$event:example.org".to_string()),
                sender: Some("@alice:example.org".to_string()),
                origin_server_ts: Some("1710000000000".to_string()),
                body: "Alice wrote a very useful Matrix search result.".to_string(),
                source_json: Some("{\"event_id\":\"$event:example.org\"}".to_string()),
                rank: Some(0.9),
                context_before_count: 1,
                context_after_count: 1,
                context_before_previews: vec![
                    "$before:example.org from @bob:example.org: Context before".to_string(),
                ],
                context_after_previews: vec![
                    "$after:example.org from @carol:example.org: Context after".to_string(),
                ],
            }],
        };

        let label = message_search_server_result_label(&response);
        assert!(label.contains("Live Matrix search returned 1 hit"));
        assert!(label.contains("sender filter @alice:example.org"));
        assert!(label.contains("media/url filter"));
        assert!(label.contains("server count 12"));
        assert!(label.contains("next cursor available"));
        assert!(label.contains("$event:example.org"));
        assert!(label.contains("@alice:example.org"));
        assert!(label.contains("Context window"));
        assert!(label.contains("$before:example.org"));
        assert!(label.contains("$after:example.org"));

        let preflight = message_search_server_live_preflight_label(&response);
        assert!(preflight.contains("MatrixRequest::SearchMessagesServer"));
        assert!(preflight.contains("sender filter @alice:example.org"));
        assert!(preflight.contains("media/url filter"));
        assert!(preflight.contains("/_matrix/client/v3/search"));
        assert!(preflight.contains("1 hit(s) with server event context"));
        assert!(preflight.contains("1 hit(s) with parsed context-window previews"));
        assert!(preflight.contains("next_batch 6 chars"));
        assert!(preflight.contains("no gateway/runtime/auth/provider mutation"));

        let boundary = message_search_server_live_boundary_label(&response);
        assert!(boundary.contains("parsed context-window previews for 1 hit"));
        assert!(boundary.contains("cross-room context"));
        assert!(boundary.contains("remote date/pins/scope adapters"));
    }

    #[test]
    fn message_search_server_error_label_reports_retry_path() {
        let label = message_search_server_error_label("needle", "M_FORBIDDEN");

        assert!(label.contains("Live Matrix search failed"));
        assert!(label.contains("query 6 chars"));
        assert!(label.contains("M_FORBIDDEN"));
        assert!(label.contains("Retry resubmits MatrixRequest::SearchMessagesServer"));
    }
}

#[cfg(test)]
mod message_copy_metadata_tests {
    use super::*;

    #[test]
    fn loaded_message_copy_metadata_label_summarizes_payload() {
        let label = loaded_message_copy_metadata_label(
            "plain text",
            "Hello from Hepta",
            Some("$event:example.org"),
        );

        assert!(label.contains("plain text"));
        assert!(label.contains("event id loaded"));
        assert!(label.contains("16 chars"));
        assert!(label.contains("16 bytes"));
        assert!(label.contains(MESSAGE_COPY_LOADED_METADATA_LABEL));
    }

    #[test]
    fn loaded_message_copy_metadata_label_counts_unicode_bytes() {
        let label = loaded_message_copy_metadata_label("HTML", "Hi 齐", None);

        assert!(label.contains("HTML"));
        assert!(label.contains("event id missing"));
        assert!(label.contains("4 chars"));
        assert!(label.contains("6 bytes"));
    }
}

#[cfg(test)]
mod matrix_link_target_metadata_tests {
    use super::*;

    #[test]
    fn matrix_link_target_metadata_label_summarizes_preview_target() {
        let label = matrix_link_target_metadata_label(
            "event",
            "$event:example.org in !room:example.org",
            2,
            "event room is current room",
            "event target not available in loaded timeline",
            "event id missing from loaded rows",
            "compact PreviewMatrixLinkTarget room preview read requested",
        );

        assert!(label.contains("kind event"));
        assert!(label.contains("$event:example.org"));
        assert!(label.contains("2 via servers"));
        assert!(label.contains("event room is current room"));
        assert!(label.contains("event target not available in loaded timeline"));
        assert!(label.contains("event id missing from loaded rows"));
        assert!(label.contains("compact PreviewMatrixLinkTarget room preview read requested"));
        assert!(label.contains(MATRIX_LINK_TARGET_METADATA_LABEL));
    }

    #[test]
    fn matrix_link_target_metadata_label_reports_zero_and_one_via() {
        let no_via = matrix_link_target_metadata_label(
            "room alias",
            "#hepta:example.org",
            0,
            "alias is not loaded as current room",
            "no loaded RoomsList alias match",
            "no event id",
            "compact PreviewMatrixLinkTarget room preview read requested",
        );
        let one_via = matrix_link_target_metadata_label(
            "room id",
            "!room:example.org",
            1,
            "target is a different joined room",
            "known room in loaded RoomsList",
            "no event id",
            "local NavigateToRoom",
        );

        assert!(no_via.contains("no via servers"));
        assert!(one_via.contains("1 via server"));
        assert!(one_via.contains("local NavigateToRoom"));
    }

    #[test]
    fn matrix_link_loaded_event_context_metadata_label_summarizes_loaded_row() {
        let event_id = EventId::parse("$event:example.org").unwrap();
        let label = matrix_link_loaded_event_context_metadata_label(
            &event_id,
            12,
            "event room is current room",
            "loaded event id matches target",
            "Loaded event body preview",
        );

        assert!(label.contains("$event:example.org"));
        assert!(label.contains("loaded index 12"));
        assert!(label.contains("event room is current room"));
        assert!(label.contains("loaded event id matches target"));
        assert!(label.contains("local scroll/highlight"));
        assert!(label.contains("Loaded event body preview"));
        assert!(label.contains(MATRIX_LINK_LOADED_EVENT_CONTEXT_METADATA_LABEL));
    }

    #[test]
    fn matrix_link_loaded_event_context_metadata_label_keeps_boundaries_explicit() {
        let event_id = EventId::parse("$event:example.org").unwrap();
        let label = matrix_link_loaded_event_context_metadata_label(
            &event_id,
            0,
            "event room is not current room",
            "loaded event row unavailable",
            "loaded event snippet unavailable",
        );

        assert!(label.contains("event room is not current room"));
        assert!(label.contains("loaded event row unavailable"));
        assert!(label.contains("no event-context fetch"));
        assert!(label.contains("pagination"));
        assert!(label.contains("source-only preview event source fetch stays on compact preview; full remote event source fetch"));
        assert!(label.contains("mutation"));
    }
}

#[cfg(test)]
mod matrix_link_preview_result_metadata_tests {
    use super::*;

    fn fetched_preview_fixture() -> FetchedRoomPreview {
        FetchedRoomPreview {
            room_name_id: RoomNameId::new(
                RoomDisplayName::Named("Hepta Preview".to_string()),
                OwnedRoomId::try_from("!hepta:example.org").unwrap(),
            ),
            room_avatar: FetchedRoomAvatar::Text("H".to_string()),
            canonical_alias: Some(ruma::OwnedRoomAliasId::try_from("#hepta:example.org").unwrap()),
            topic: Some("Matrix preview metadata".to_string()),
            num_joined_members: 42,
            num_active_members: Some(45),
            room_type: Some(RoomType::Space),
            join_rule: Some(JoinRuleSummary::Public),
            is_world_readable: Some(false),
            state: Some(RoomState::Joined),
            is_direct: Some(false),
            heroes: Some(Vec::new()),
        }
    }

    #[test]
    fn matrix_link_preview_result_metadata_label_summarizes_fetched_room_preview() {
        let event_id = OwnedEventId::try_from("$event:example.org").unwrap();
        let label =
            matrix_link_preview_result_metadata_label(&fetched_preview_fixture(), Some(&event_id));

        assert!(label.contains("Hepta Preview"));
        assert!(label.contains("42 joined members"));
        assert!(label.contains("45 active members"));
        assert!(label.contains("canonical alias loaded: #hepta:example.org"));
        assert!(label.contains("topic loaded: 23 chars"));
        assert!(label.contains("room type space"));
        assert!(label.contains("join rule public"));
        assert!(label.contains("non-world-readable history"));
        assert!(label.contains("current-user state joined"));
        assert!(label.contains("not direct"));
        assert!(label.contains("0 heroes loaded"));
        assert!(label.contains("avatar text fallback"));
        assert!(label.contains("event $event:example.org context fetch still not wired"));
        assert!(label.contains(MATRIX_LINK_PREVIEW_RESULT_METADATA_LABEL));
    }

    #[test]
    fn matrix_link_preview_result_metadata_label_reports_unknown_preview_fields() {
        let mut fetched = fetched_preview_fixture();
        fetched.room_avatar = FetchedRoomAvatar::Image(
            (
                OwnedMxcUri::try_from("mxc://example.org/avatar").unwrap(),
                Arc::<[u8]>::from([1_u8, 2, 3]),
            )
                .into(),
        );
        fetched.canonical_alias = None;
        fetched.topic = Some("   ".to_string());
        fetched.num_active_members = None;
        fetched.room_type = None;
        fetched.join_rule = None;
        fetched.is_world_readable = None;
        fetched.state = None;
        fetched.is_direct = None;
        fetched.heroes = None;

        let label = matrix_link_preview_result_metadata_label(&fetched, None);

        assert!(label.contains("canonical alias missing"));
        assert!(label.contains("topic missing"));
        assert!(label.contains("active member count unknown"));
        assert!(label.contains("room type regular"));
        assert!(label.contains("join rule unknown"));
        assert!(label.contains("history visibility unknown"));
        assert!(label.contains("current-user state unknown"));
        assert!(label.contains("direct flag unknown"));
        assert!(label.contains("heroes unknown"));
        assert!(label.contains("avatar image loaded: 3 bytes"));
        assert!(label.contains("no event id requested"));
    }
}

#[cfg(test)]
mod matrix_link_preview_failure_metadata_tests {
    use super::*;

    #[test]
    fn matrix_link_preview_failure_metadata_label_summarizes_failed_event_preview() {
        let event_id = OwnedEventId::try_from("$missing:example.org").unwrap();
        let label = matrix_link_preview_failure_metadata_label(
            "!room:example.org",
            2,
            Some(&event_id),
            "M_FORBIDDEN: room preview denied",
        );

        assert!(label.contains("target !room:example.org"));
        assert!(label.contains("2 via servers"));
        assert!(label.contains("event $missing:example.org was requested"));
        assert!(label.contains("event context fetch still not wired"));
        assert!(label.contains("error message 32 chars"));
        assert!(label.contains(MATRIX_LINK_PREVIEW_FAILURE_METADATA_LABEL));
    }

    #[test]
    fn matrix_link_preview_failure_metadata_label_reports_empty_error_and_single_via() {
        let no_via =
            matrix_link_preview_failure_metadata_label("#alias:example.org", 0, None, "   ");
        let one_via =
            matrix_link_preview_failure_metadata_label("#alias:example.org", 1, None, "timeout");

        assert!(no_via.contains("no via servers"));
        assert!(no_via.contains("no event id requested"));
        assert!(no_via.contains("error message empty"));
        assert!(one_via.contains("1 via server"));
        assert!(one_via.contains("error message 7 chars"));
    }

    #[test]
    fn matrix_link_preview_retry_confirmation_label_is_confirmed_and_narrow() {
        let event_id = OwnedEventId::try_from("$missing:example.org").unwrap();
        let label =
            matrix_link_preview_retry_confirmation_label("!room:example.org", 2, Some(&event_id));

        assert!(label.contains("Retry Matrix link preview for !room:example.org"));
        assert!(label.contains("Cached target only"));
        assert!(label.contains("2 via servers"));
        assert!(label.contains("$missing:example.org"));
        assert!(label.contains("Retry confirms before PreviewMatrixLinkTarget"));
        assert!(label.contains("event context"));
        assert!(label.contains("join"));
        assert!(label.contains("knock"));
        assert!(label.contains("browser handoff"));
        assert!(label.contains("mutation"));
    }

    #[test]
    fn matrix_link_server_context_boundary_label_keeps_remaining_gap_visible() {
        let event_id = OwnedEventId::try_from("$missing:example.org").unwrap();
        let label = matrix_link_server_context_boundary_label(
            "compact preview failed",
            2,
            Some(&event_id),
            true,
        );

        assert!(label.contains("compact preview failed"));
        assert!(label.contains("2 via servers"));
        assert!(label.contains("$missing:example.org"));
        assert!(label.contains("retry cache ready"));
        assert!(label.contains(MATRIX_LINK_SERVER_CONTEXT_BOUNDARY_LABEL));
        assert!(
            label.contains(
                "cached Server context refresh may use PreviewMatrixLinkTarget read-only"
            )
        );
        assert!(label.contains("event context fetch"));
        assert!(label.contains("timeline pagination/reload"));
        assert!(label.contains("BackwardsPaginateUntilEvent"));
        assert!(label.contains("Join"));
        assert!(label.contains("Knock"));
        assert!(label.contains("invite"));
        assert!(label.contains("browser handoff"));
        assert!(label.contains("source-only preview event source fetch stays on compact preview; full remote event source fetch"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
    }

    #[test]
    fn matrix_link_server_context_boundary_label_reports_retry_cache_cleared() {
        let label =
            matrix_link_server_context_boundary_label("compact preview resolved", 0, None, false);

        assert!(label.contains("compact preview resolved"));
        assert!(label.contains("no via servers"));
        assert!(label.contains("no event id requested"));
        assert!(label.contains("retry cache cleared"));
    }

    #[test]
    fn matrix_link_context_actions_row_label_keeps_actions_local() {
        let event_id = OwnedEventId::try_from("$missing:example.org").unwrap();
        let label = matrix_link_context_actions_row_label(
            Some("Event context"),
            "failed",
            2,
            Some(&event_id),
            true,
        );

        assert!(label.contains("Event context selected"));
        assert!(label.contains("status failed"));
        assert!(label.contains("2 via servers"));
        assert!(label.contains("$missing:example.org"));
        assert!(label.contains("retry cache ready"));
        assert!(label.contains("Server, Event, Alias, Join, Knock, Invite, Browser, Source"));
        assert!(label.contains("Browser confirms before matrix.to system opener handoff"));
        assert!(label.contains("Invite confirms before current-room MatrixRequest::InviteUser"));
        assert!(label.contains(MATRIX_LINK_CONTEXT_ACTIONS_ROW_LABEL));
        assert!(label.contains("Server refreshes cached room-or-alias targets through PreviewMatrixLinkTarget read-only"));
        assert!(label.contains("event context fetch"));
        assert!(label.contains("BackwardsPaginateUntilEvent"));
        assert!(label.contains("Join"));
        assert!(label.contains("Knock"));
        assert!(label.contains("Invite"));
        assert!(label.contains("event source"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(MATRIX_LINK_CONTEXT_ACTIONS_ROW_EVIDENCE.contains("Server, Event, Alias"));
        assert!(MATRIX_LINK_CONTEXT_ACTIONS_ROW_EVIDENCE.contains("Clicking Browser builds"));
        assert!(MATRIX_LINK_CONTEXT_ACTIONS_ROW_EVIDENCE.contains("confirmed failed-state Retry"));
        assert!(
            MATRIX_LINK_CONTEXT_ACTIONS_ROW_EVIDENCE
                .contains("MatrixRequest::BackwardsPaginateUntilEvent")
        );
    }

    #[test]
    fn matrix_link_context_actions_row_label_reports_empty_state() {
        let label = matrix_link_context_actions_row_label(None, "   ", 0, None, false);

        assert!(label.contains("no context action selected"));
        assert!(label.contains("status preview"));
        assert!(label.contains("no via servers"));
        assert!(label.contains("no event id requested"));
        assert!(label.contains("retry cache cleared"));
    }

    #[test]
    fn matrix_link_invite_user_confirmation_label_names_current_room_guard() {
        let room_id = owned_room_id!("!room:example.org");
        let user_id = OwnedUserId::try_from("@ada:example.org").unwrap();
        let label = matrix_link_invite_user_confirmation_label(&room_id, &user_id, "user", 1);

        assert!(label.contains("Invite Matrix user @ada:example.org"));
        assert!(label.contains("current room !room:example.org"));
        assert!(label.contains("1 via server"));
        assert!(label.contains("PositiveConfirmationModal"));
        assert!(label.contains("MatrixRequest::InviteUser"));
        assert!(label.contains(MATRIX_LINK_INVITE_USER_CONFIRMATION_LABEL));
        assert!(MATRIX_LINK_INVITE_USER_CONFIRMATION_EVIDENCE.contains("InviteResultAction"));
    }

    #[test]
    fn matrix_link_user_invite_target_accepts_only_user_ids() {
        let user_id = matrix_link_user_invite_target("@ada:example.org").unwrap();
        assert_eq!(user_id.as_str(), "@ada:example.org");
        assert!(matrix_link_user_invite_target("!room:example.org").is_err());
        assert!(matrix_link_user_invite_target("#alias:example.org").is_err());
    }

    #[test]
    fn matrix_link_invite_user_result_label_keeps_retry_explicit() {
        let room_id = owned_room_id!("!room:example.org");
        let user_id = OwnedUserId::try_from("@ada:example.org").unwrap();
        let success = matrix_link_invite_user_result_label(&room_id, &user_id, true, None);
        let failed =
            matrix_link_invite_user_result_label(&room_id, &user_id, false, Some("forbidden"));

        assert!(success.contains("Invite succeeded"));
        assert!(success.contains("InviteResultAction::Sent"));
        assert!(failed.contains("Invite failed"));
        assert!(failed.contains("forbidden"));
        assert!(failed.contains("confirms before MatrixRequest::InviteUser"));
    }

    #[test]
    fn matrix_link_server_context_packet_snapshot_label_summarizes_local_packet() {
        let label = matrix_link_server_context_packet_snapshot_label(
            "Server context",
            "failed",
            "!room:example.org",
            2,
            "example.org, matrix.org",
            "$missing:example.org",
            128,
            Some(32),
            true,
            false,
        );

        assert!(label.contains("Local Matrix link server-context packet snapshot"));
        assert!(label.contains("Server context selected"));
        assert!(label.contains("status failed"));
        assert!(label.contains("target !room:example.org"));
        assert!(label.contains("2 via servers"));
        assert!(label.contains("via list example.org, matrix.org"));
        assert!(label.contains("event id $missing:example.org requested"));
        assert!(label.contains("preview metadata 128 chars"));
        assert!(label.contains("error metadata 32 chars"));
        assert!(label.contains("retry cache ready"));
        assert!(label.contains("loaded current-room source unavailable"));
        assert!(label.contains("request body"));
        assert!(label.contains("alias lookup"));
        assert!(label.contains("event context route"));
        assert!(label.contains("pagination cursor"));
        assert!(label.contains("join result"));
        assert!(label.contains("PreviewMatrixLinkTarget is limited to compact preview, confirmed failed-state Retry, or cached Server context refresh"));
        assert!(label.contains("BackwardsPaginateUntilEvent"));
        assert!(label.contains("cached Server context refresh"));
        assert!(label.contains("server-side event context fetch"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(
            MATRIX_LINK_CONTEXT_ACTIONS_ROW_EVIDENCE
                .contains("local Matrix link server-context packet snapshot")
        );

        let empty = matrix_link_server_context_packet_snapshot_label(
            "", "", "", 0, "", "", 0, None, false, true,
        );
        assert!(empty.contains("Server context selected"));
        assert!(empty.contains("status preview"));
        assert!(empty.contains("target waiting"));
        assert!(empty.contains("no via servers"));
        assert!(empty.contains("via list waiting"));
        assert!(empty.contains("no event id requested"));
        assert!(empty.contains("error metadata unavailable"));
        assert!(empty.contains("retry cache cleared"));
        assert!(empty.contains("loaded current-room source available"));
    }

    #[test]
    fn matrix_link_route_scope_controls_label_keeps_route_scope_local() {
        let label = matrix_link_route_scope_controls_label(
            Some("Via servers"),
            "failed",
            "!room:example.org",
            2,
            "$missing:example.org",
            128,
            true,
        );

        assert!(label.contains("Via servers route scope selected"));
        assert!(label.contains("status failed"));
        assert!(label.contains("target !room:example.org"));
        assert!(label.contains("2 via servers"));
        assert!(label.contains("event id $missing:example.org requested"));
        assert!(label.contains("preview metadata 128 chars"));
        assert!(label.contains("retry cache ready"));
        assert!(label.contains("Room, Event, Via, Preview, Source, Packet, Contract, Taxonomy"));
        assert!(label.contains(MATRIX_LINK_ROUTE_SCOPE_CONTROLS_LABEL));
        assert!(label.contains("Event copies cached requested event id"));
        assert!(label.contains("Preview copies cached local preview metadata"));
        assert!(label.contains("Packet copies per-target route acceptance criteria"));
        assert!(label.contains("Taxonomy copies route/event-context result slots"));
        assert!(label.contains("PreviewMatrixLinkTarget is limited to compact preview, confirmed Retry, or cached Server context refresh"));
        assert!(label.contains("BackwardsPaginateUntilEvent"));
        assert!(label.contains("PreviewMatrixLinkTarget is limited to compact preview, confirmed Retry, or cached Server context refresh"));
        assert!(label.contains("server-side event context fetch"));
        assert!(label.contains("source-only preview event source fetch stays on compact preview; full remote event source fetch"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(MATRIX_LINK_ROUTE_SCOPE_CONTROLS_EVIDENCE.contains("Room, Event, Via"));
        assert!(
            MATRIX_LINK_ROUTE_SCOPE_CONTROLS_EVIDENCE.contains("Packet, Contract, and Taxonomy")
        );
        assert!(MATRIX_LINK_ROUTE_SCOPE_CONTROLS_EVIDENCE.contains("BackwardsPaginateUntilEvent"));
    }

    #[test]
    fn matrix_link_route_scope_controls_label_reports_empty_waiting_state() {
        let label = matrix_link_route_scope_controls_label(None, "", "", 0, "", 0, false);

        assert!(label.contains("no route scope selected"));
        assert!(label.contains("status preview"));
        assert!(label.contains("target waiting"));
        assert!(label.contains("no via servers"));
        assert!(label.contains("no event id requested"));
        assert!(label.contains("preview metadata 0 chars"));
        assert!(label.contains("retry cache cleared"));
    }

    #[test]
    fn matrix_link_route_drilldown_packet_payload_lists_per_target_routes() {
        let payload = matrix_link_route_drilldown_packet_payload(
            "failed",
            "!room:example.org",
            2,
            "example.org, matrix.org",
            "$missing:example.org",
            128,
            Some(32),
            true,
            false,
        );

        assert!(payload.contains("Matrix link route drilldown packet"));
        assert!(payload.contains("Status: failed"));
        assert!(payload.contains("Target: target !room:example.org"));
        assert!(payload.contains("Via: 2 via servers; via list example.org, matrix.org"));
        assert!(payload.contains("Event: event id $missing:example.org requested"));
        assert!(payload.contains("Preview: metadata 128 chars; error metadata 32 chars"));
        assert!(payload.contains("Retry: retry cache ready"));
        assert!(payload.contains("Loaded source: loaded current-room source unavailable"));
        assert!(payload.contains("Room route acceptance"));
        assert!(payload.contains("Event route acceptance"));
        assert!(payload.contains("Via route acceptance"));
        assert!(payload.contains("Preview route acceptance"));
        assert!(payload.contains("Server context acceptance"));
        assert!(payload.contains("Alias route acceptance"));
        assert!(payload.contains("Join route acceptance"));
        assert!(payload.contains("Source route acceptance"));
        assert!(payload.contains(MATRIX_LINK_ROUTE_DRILLDOWN_PACKET_LABEL));
        assert!(
            payload.contains("PreviewMatrixLinkTarget limited to compact preview, confirmed Retry, or cached Server context refresh")
        );
        assert!(payload.contains("BackwardsPaginateUntilEvent"));
        assert!(payload.contains("server-side alias resolution"));
        assert!(payload.contains("server-side event context fetch"));
        assert!(payload.contains("non-current-room timeline pagination/reload"));
        assert!(payload.contains("external browser handoff"));
        assert!(payload.contains("source-only preview event source fetch stays on compact preview; full remote event source fetch"));
        assert!(payload.contains("gateway/runtime/auth"));
        assert!(payload.contains("live mutation"));
    }

    #[test]
    fn matrix_link_route_drilldown_packet_label_reports_copy_state() {
        let label = matrix_link_route_drilldown_packet_label(
            "resolved",
            "#alias:example.org",
            1,
            "",
            64,
            false,
        );

        assert!(label.contains("Matrix link route Packet copied"));
        assert!(label.contains("status resolved"));
        assert!(label.contains("target #alias:example.org"));
        assert!(label.contains("1 via server"));
        assert!(label.contains("no event id requested"));
        assert!(label.contains("preview metadata 64 chars"));
        assert!(label.contains("retry cache cleared"));
        assert!(label.contains(MATRIX_LINK_ROUTE_DRILLDOWN_PACKET_LABEL));
        assert!(label.contains("PreviewMatrixLinkTarget is limited to compact preview, confirmed Retry, or cached Server context refresh"));
        assert!(label.contains("BackwardsPaginateUntilEvent"));
        assert!(label.contains("cached Server context refresh"));
        assert!(label.contains("server-side event context fetch"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
    }

    #[test]
    fn matrix_link_route_result_contract_packet_payload_lists_typed_contracts() {
        let payload = matrix_link_route_result_contract_packet_payload(
            "failed",
            "!room:example.org",
            2,
            "example.org, matrix.org",
            "$missing:example.org",
            128,
            Some(32),
            true,
            false,
        );

        assert!(payload.contains("Matrix link typed route/result contract"));
        assert!(payload.contains("Status: failed"));
        assert!(payload.contains("Target: target !room:example.org"));
        assert!(payload.contains("Via: 2 via servers; via list example.org, matrix.org"));
        assert!(payload.contains("Event: event id $missing:example.org requested"));
        assert!(payload.contains("Preview: metadata 128 chars; error metadata 32 chars"));
        assert!(payload.contains("Retry: retry cache ready"));
        assert!(payload.contains("Loaded source: loaded current-room source unavailable"));
        assert!(payload.contains("Target identity"));
        assert!(payload.contains("Preview contract"));
        assert!(payload.contains("Alias route contract"));
        assert!(payload.contains("Room route contract"));
        assert!(payload.contains("Event route contract"));
        assert!(payload.contains("Via route contract"));
        assert!(payload.contains("Join, Knock, and Invite contract"));
        assert!(payload.contains("Source contract"));
        assert!(payload.contains("Browser handoff contract"));
        assert!(payload.contains("typed route/result contracts"));
        assert!(payload.contains(MATRIX_LINK_ROUTE_RESULT_CONTRACT_PACKET_LABEL));
        assert!(
            payload.contains("PreviewMatrixLinkTarget limited to compact preview, confirmed Retry, or cached Server context refresh")
        );
        assert!(payload.contains("BackwardsPaginateUntilEvent"));
        assert!(payload.contains("server-side alias lookup result"));
        assert!(payload.contains("server-side event context fetch"));
        assert!(payload.contains("non-current-room timeline pagination/reload"));
        assert!(payload.contains("external browser handoff"));
        assert!(payload.contains("source-only preview event source fetch stays on compact preview; full remote event source fetch"));
        assert!(payload.contains("gateway/runtime/auth"));
        assert!(payload.contains("live mutation"));
    }

    #[test]
    fn matrix_link_route_result_contract_packet_label_reports_copy_state() {
        let label = matrix_link_route_result_contract_packet_label(
            "resolved",
            "#alias:example.org",
            1,
            "",
            64,
            false,
        );

        assert!(label.contains("Matrix link route Contract copied"));
        assert!(label.contains("status resolved"));
        assert!(label.contains("target #alias:example.org"));
        assert!(label.contains("1 via server"));
        assert!(label.contains("no event id requested"));
        assert!(label.contains("preview metadata 64 chars"));
        assert!(label.contains("retry cache cleared"));
        assert!(label.contains(MATRIX_LINK_ROUTE_RESULT_CONTRACT_PACKET_LABEL));
        assert!(label.contains("PreviewMatrixLinkTarget is limited to compact preview, confirmed Retry, or cached Server context refresh"));
        assert!(label.contains("BackwardsPaginateUntilEvent"));
        assert!(label.contains("cached Server context refresh"));
        assert!(label.contains("event context fetch"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
    }

    #[test]
    fn matrix_link_route_result_taxonomy_packet_payload_lists_result_slots() {
        let payload = matrix_link_route_result_taxonomy_packet_payload(
            "failed",
            "!room:example.org",
            2,
            "example.org, matrix.org",
            "$missing:example.org",
            128,
            Some(32),
            true,
            false,
        );

        assert!(payload.contains("Matrix link route/event-context result taxonomy"));
        assert!(payload.contains("Status: failed"));
        assert!(payload.contains("Target: target !room:example.org"));
        assert!(payload.contains("Via: 2 via servers; via list example.org, matrix.org"));
        assert!(payload.contains("Event: event id $missing:example.org requested"));
        assert!(payload.contains("Preview: metadata 128 chars; error metadata 32 chars"));
        assert!(payload.contains("Retry: retry cache ready"));
        assert!(payload.contains("loaded current-room source unavailable"));
        assert!(payload.contains("loaded_alias_navigation_result"));
        assert!(payload.contains("compact_preview_result"));
        assert!(payload.contains("source_only_preview_fetch_result"));
        assert!(payload.contains("join_knock_invite_result"));
        assert!(payload.contains("route_adapter_request_id: not_assigned"));
        assert!(payload.contains("alias_resolution_operation_id: not_assigned"));
        assert!(payload.contains("non_current_room_event_context_operation_id: not_assigned"));
        assert!(payload.contains("via_route_request_id: not_assigned"));
        assert!(payload.contains("full_remote_source_request_id: not_assigned"));
        assert!(payload.contains("event_context_window_result: not_wired"));
        assert!(payload.contains("alias_resolution_result: not_wired"));
        assert!(payload.contains("via_resolution_result: not_wired"));
        assert!(payload.contains("full_remote_source_result: not_wired"));
        assert!(payload.contains("stale_target_result: not_wired"));
        assert!(payload.contains(MATRIX_LINK_ROUTE_RESULT_TAXONOMY_PACKET_LABEL));
        assert!(payload.contains("No PreviewMatrixLinkTarget beyond explicit compact preview"));
        assert!(payload.contains("no server-side alias resolution"));
        assert!(payload.contains("no event-context fetch"));
        assert!(payload.contains("no full remote source fetch"));
        assert!(payload.contains("no gateway/runtime/auth/provider"));
        assert!(payload.contains("live mutation"));
    }

    #[test]
    fn matrix_link_route_result_taxonomy_packet_label_reports_copy_state() {
        let label = matrix_link_route_result_taxonomy_packet_label(
            "resolved",
            "#alias:example.org",
            1,
            "",
            64,
            false,
        );

        assert!(label.contains("Matrix link Taxonomy copied"));
        assert!(label.contains("status resolved"));
        assert!(label.contains("target #alias:example.org"));
        assert!(label.contains("1 via server"));
        assert!(label.contains("no event id requested"));
        assert!(label.contains("preview metadata 64 chars"));
        assert!(label.contains("retry cache cleared"));
        assert!(label.contains(MATRIX_LINK_ROUTE_RESULT_TAXONOMY_PACKET_LABEL));
        assert!(label.contains("route_adapter_request_id not_assigned"));
        assert!(label.contains("non_current_room_event_context_operation_id not_assigned"));
        assert!(label.contains("event_context_window_result not_wired"));
        assert!(label.contains("full_remote_source_result not_wired"));
        assert!(label.contains("No server-side alias resolution"));
        assert!(label.contains("gateway/runtime/auth/provider"));
        assert!(label.contains("live mutation"));
    }

    #[test]
    fn matrix_link_route_result_taxonomy_evidence_names_live_boundary() {
        assert!(
            MATRIX_LINK_ROUTE_RESULT_TAXONOMY_PACKET_EVIDENCE.contains("loaded alias navigation")
        );
        assert!(
            MATRIX_LINK_ROUTE_RESULT_TAXONOMY_PACKET_EVIDENCE
                .contains("MatrixRequest::PreviewMatrixLinkTarget")
        );
        assert!(
            MATRIX_LINK_ROUTE_RESULT_TAXONOMY_PACKET_EVIDENCE
                .contains("MatrixRequest::BackwardsPaginateUntilEvent")
        );
        assert!(
            MATRIX_LINK_ROUTE_RESULT_TAXONOMY_PACKET_EVIDENCE.contains("route_adapter_request_id")
        );
        assert!(
            MATRIX_LINK_ROUTE_RESULT_TAXONOMY_PACKET_EVIDENCE
                .contains("non_current_room_event_context_operation_id")
        );
        assert!(
            MATRIX_LINK_ROUTE_RESULT_TAXONOMY_PACKET_EVIDENCE
                .contains("full_remote_source_request_id")
        );
        assert!(
            MATRIX_LINK_ROUTE_RESULT_TAXONOMY_PACKET_EVIDENCE
                .contains("no server-side alias resolution")
        );
        assert!(
            MATRIX_LINK_ROUTE_RESULT_TAXONOMY_PACKET_EVIDENCE.contains("no event-context fetch")
        );
    }

    #[test]
    fn matrix_link_room_target_clipboard_payload_is_cached_local_only() {
        let payload = matrix_link_room_target_clipboard_payload(
            "resolved",
            "!room:example.org",
            2,
            "$event:example.org",
            true,
        )
        .expect("cached Matrix link target should be copied");

        assert!(payload.contains("Matrix link room target"));
        assert!(payload.contains("Status: resolved"));
        assert!(payload.contains("Target: !room:example.org"));
        assert!(payload.contains("Via: 2 via servers"));
        assert!(payload.contains("Event: event id $event:example.org requested"));
        assert!(payload.contains("Retry: retry cache ready"));
        assert!(payload.contains(MATRIX_LINK_ROOM_TARGET_CLIPBOARD_LABEL));
        assert!(payload.contains("PreviewMatrixLinkTarget is limited to compact preview, confirmed Retry, or cached Server context refresh"));
        assert!(payload.contains("BackwardsPaginateUntilEvent"));
        assert!(payload.contains("server-side alias resolution"));
        assert!(payload.contains("event context fetch"));
        assert!(payload.contains("source-only preview event source fetch stays on compact preview; full remote event source fetch"));
        assert!(payload.contains("gateway/runtime/auth"));
        assert!(payload.contains("live mutation"));
    }

    #[test]
    fn matrix_link_room_target_clipboard_payload_requires_cached_target() {
        assert!(
            matrix_link_room_target_clipboard_payload("resolved", "   ", 0, "", false).is_none()
        );
    }

    #[test]
    fn matrix_link_room_target_clipboard_label_reports_copy_state() {
        let copied = matrix_link_room_target_clipboard_label(
            true,
            "failed",
            "#room:example.org",
            1,
            "",
            false,
        );
        assert!(copied.contains("copied cached target metadata to local clipboard"));
        assert!(copied.contains("status failed"));
        assert!(copied.contains("target #room:example.org"));
        assert!(copied.contains("1 via server"));
        assert!(copied.contains("no event id requested"));
        assert!(copied.contains("retry cache cleared"));
        assert!(copied.contains(MATRIX_LINK_ROOM_TARGET_CLIPBOARD_LABEL));

        let unavailable = matrix_link_room_target_clipboard_label(false, "", "", 0, "", false);
        assert!(unavailable.contains("target clipboard unavailable"));
        assert!(unavailable.contains("preview status waiting"));
        assert!(unavailable.contains("target waiting"));
    }

    #[test]
    fn matrix_link_via_servers_clipboard_payload_is_cached_local_only() {
        let payload = matrix_link_via_servers_clipboard_payload(
            "resolved",
            "!room:example.org",
            2,
            "example.org, matrix.org",
            "$event:example.org",
            true,
        )
        .expect("cached via server list should be copied");

        assert!(payload.contains("Matrix link via servers"));
        assert!(payload.contains("Status: resolved"));
        assert!(payload.contains("Target: target !room:example.org"));
        assert!(payload.contains("Via: 2 via servers"));
        assert!(payload.contains("Servers: example.org, matrix.org"));
        assert!(payload.contains("Event: event id $event:example.org requested"));
        assert!(payload.contains("Retry: retry cache ready"));
        assert!(payload.contains(MATRIX_LINK_VIA_SERVERS_CLIPBOARD_LABEL));
        assert!(payload.contains("PreviewMatrixLinkTarget is limited to compact preview, confirmed Retry, or cached Server context refresh"));
        assert!(payload.contains("BackwardsPaginateUntilEvent"));
        assert!(payload.contains("server-side alias resolution"));
        assert!(payload.contains("event context fetch"));
        assert!(payload.contains("source-only preview event source fetch stays on compact preview; full remote event source fetch"));
        assert!(payload.contains("gateway/runtime/auth"));
        assert!(payload.contains("live mutation"));
    }

    #[test]
    fn matrix_link_via_servers_clipboard_payload_requires_cached_via_list() {
        assert!(
            matrix_link_via_servers_clipboard_payload(
                "resolved",
                "!room:example.org",
                0,
                "   ",
                "",
                false
            )
            .is_none()
        );
    }

    #[test]
    fn matrix_link_via_servers_clipboard_label_reports_copy_state() {
        let copied = matrix_link_via_servers_clipboard_label(
            true,
            "failed",
            "#room:example.org",
            1,
            "example.org",
            "",
            false,
        );
        assert!(copied.contains("copied cached via server list to local clipboard"));
        assert!(copied.contains("status failed"));
        assert!(copied.contains("target #room:example.org"));
        assert!(copied.contains("1 via server"));
        assert!(copied.contains("via list example.org"));
        assert!(copied.contains("no event id requested"));
        assert!(copied.contains("retry cache cleared"));
        assert!(copied.contains(MATRIX_LINK_VIA_SERVERS_CLIPBOARD_LABEL));

        let unavailable = matrix_link_via_servers_clipboard_label(false, "", "", 0, "", "", false);
        assert!(unavailable.contains("via server clipboard unavailable"));
        assert!(unavailable.contains("preview status waiting"));
        assert!(unavailable.contains("target waiting"));
        assert!(unavailable.contains("via list waiting"));
    }

    #[test]
    fn matrix_link_event_id_clipboard_payload_is_cached_local_only() {
        let payload = matrix_link_event_id_clipboard_payload(
            "resolved",
            "$event:example.org in !room:example.org",
            2,
            "$event:example.org",
            true,
        )
        .expect("cached event id should be copied");

        assert!(payload.contains("Matrix link event id"));
        assert!(payload.contains("Status: resolved"));
        assert!(payload.contains("Target: target $event:example.org in !room:example.org"));
        assert!(payload.contains("Via: 2 via servers"));
        assert!(payload.contains("Event: $event:example.org"));
        assert!(payload.contains("Retry: retry cache ready"));
        assert!(payload.contains(MATRIX_LINK_EVENT_ID_CLIPBOARD_LABEL));
        assert!(payload.contains("PreviewMatrixLinkTarget is limited to compact preview, confirmed Retry, or cached Server context refresh"));
        assert!(payload.contains("BackwardsPaginateUntilEvent"));
        assert!(payload.contains("server-side alias resolution"));
        assert!(payload.contains("event context fetch"));
        assert!(payload.contains("source-only preview event source fetch stays on compact preview; full remote event source fetch"));
        assert!(payload.contains("gateway/runtime/auth"));
        assert!(payload.contains("live mutation"));
    }

    #[test]
    fn matrix_link_event_id_clipboard_payload_requires_cached_event_id() {
        assert!(
            matrix_link_event_id_clipboard_payload(
                "resolved",
                "!room:example.org",
                0,
                "   ",
                false
            )
            .is_none()
        );
    }

    #[test]
    fn matrix_link_event_id_clipboard_label_reports_copy_state() {
        let copied = matrix_link_event_id_clipboard_label(
            true,
            "failed",
            "$event:example.org",
            1,
            "$event:example.org",
            false,
        );
        assert!(copied.contains("copied cached event id to local clipboard"));
        assert!(copied.contains("status failed"));
        assert!(copied.contains("target $event:example.org"));
        assert!(copied.contains("1 via server"));
        assert!(copied.contains("event id $event:example.org"));
        assert!(copied.contains("retry cache cleared"));
        assert!(copied.contains(MATRIX_LINK_EVENT_ID_CLIPBOARD_LABEL));

        let unavailable = matrix_link_event_id_clipboard_label(false, "", "", 0, "", false);
        assert!(unavailable.contains("event id clipboard unavailable"));
        assert!(unavailable.contains("preview status waiting"));
        assert!(unavailable.contains("target waiting"));
        assert!(unavailable.contains("event id waiting"));
    }

    #[test]
    fn matrix_link_preview_metadata_clipboard_payload_is_cached_local_only() {
        let payload = matrix_link_preview_metadata_clipboard_payload(
            "resolved",
            "!room:example.org",
            2,
            "$event:example.org",
            "canonical alias #room:example.org; joined members 4; event context unavailable",
            true,
        )
        .expect("loaded preview metadata should be copied");

        assert!(payload.contains("Matrix link preview metadata"));
        assert!(payload.contains("Status: resolved"));
        assert!(payload.contains("Target: target !room:example.org"));
        assert!(payload.contains("Via: 2 via servers"));
        assert!(payload.contains("Event: event id $event:example.org requested"));
        assert!(payload.contains("Retry: retry cache ready"));
        assert!(payload.contains("canonical alias #room:example.org"));
        assert!(payload.contains(MATRIX_LINK_PREVIEW_METADATA_CLIPBOARD_LABEL));
        assert!(payload.contains("PreviewMatrixLinkTarget is limited to compact preview, confirmed Retry, or cached Server context refresh"));
        assert!(payload.contains("BackwardsPaginateUntilEvent"));
        assert!(payload.contains("server-side alias resolution"));
        assert!(payload.contains("event context fetch"));
        assert!(payload.contains("source-only preview event source fetch stays on compact preview; full remote event source fetch"));
        assert!(payload.contains("gateway/runtime/auth"));
        assert!(payload.contains("live mutation"));
    }

    #[test]
    fn matrix_link_preview_metadata_clipboard_payload_requires_cached_metadata() {
        assert!(
            matrix_link_preview_metadata_clipboard_payload(
                "resolved",
                "!room:example.org",
                0,
                "",
                "   ",
                false
            )
            .is_none()
        );
    }

    #[test]
    fn matrix_link_preview_metadata_clipboard_label_reports_copy_state() {
        let copied = matrix_link_preview_metadata_clipboard_label(
            true,
            "failed",
            "!room:example.org",
            1,
            "",
            "failed: event context unavailable",
            false,
        );
        assert!(copied.contains("copied cached preview metadata to local clipboard"));
        assert!(copied.contains("status failed"));
        assert!(copied.contains("target !room:example.org"));
        assert!(copied.contains("1 via server"));
        assert!(copied.contains("no event id requested"));
        assert!(copied.contains("metadata"));
        assert!(copied.contains("retry cache cleared"));
        assert!(copied.contains(MATRIX_LINK_PREVIEW_METADATA_CLIPBOARD_LABEL));

        let unavailable =
            matrix_link_preview_metadata_clipboard_label(false, "", "", 0, "", "", false);
        assert!(unavailable.contains("preview metadata clipboard unavailable"));
        assert!(unavailable.contains("preview status waiting"));
        assert!(unavailable.contains("target waiting"));
        assert!(unavailable.contains("metadata waiting"));
    }

    #[test]
    fn matrix_link_unresolved_detail_label_summarizes_cached_target() {
        let label = matrix_link_unresolved_detail_label(
            Some("Join or knock"),
            "failed",
            "!room:example.org",
            2,
            "$missing:example.org",
            128,
            Some(32),
            true,
        );

        assert!(label.contains("Join or knock detail selected"));
        assert!(label.contains("status failed"));
        assert!(label.contains("target !room:example.org"));
        assert!(label.contains("2 via servers"));
        assert!(label.contains("event id $missing:example.org requested"));
        assert!(label.contains("preview metadata 128 chars"));
        assert!(label.contains("error metadata 32 chars"));
        assert!(label.contains("retry cache ready"));
        assert!(label.contains(MATRIX_LINK_UNRESOLVED_DETAIL_LABEL));
        assert!(label.contains("PreviewMatrixLinkTarget is limited to compact preview, confirmed Retry, or cached Server context refresh"));
        assert!(label.contains("BackwardsPaginateUntilEvent"));
        assert!(label.contains("server-side alias resolution"));
        assert!(label.contains("event context fetch"));
        assert!(label.contains("source-only preview event source fetch stays on compact preview; full remote event source fetch"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(
            MATRIX_LINK_UNRESOLVED_DETAIL_EVIDENCE.contains("Server/Event/Alias/Join/Knock/Source")
        );
        assert!(
            MATRIX_LINK_UNRESOLVED_DETAIL_EVIDENCE
                .contains("MatrixRequest::PreviewMatrixLinkTarget")
        );
    }

    #[test]
    fn matrix_link_unresolved_detail_label_reports_empty_waiting_state() {
        let label = matrix_link_unresolved_detail_label(None, "", "", 0, "", 0, None, false);

        assert!(label.contains("no Server/Event/Alias/Join/Knock/Source detail selected"));
        assert!(label.contains("status preview"));
        assert!(label.contains("target waiting"));
        assert!(label.contains("no via servers"));
        assert!(label.contains("no event id requested"));
        assert!(label.contains("preview metadata 0 chars"));
        assert!(label.contains("error metadata unavailable"));
        assert!(label.contains("retry cache cleared"));
    }

    #[test]
    fn matrix_link_loaded_event_source_modal_label_summarizes_loaded_source() {
        let label = matrix_link_loaded_event_source_modal_label(
            "Event source",
            "$event:example.org in !room:example.org",
            2,
            "$event:example.org",
            Some(7),
            Some("{\n  \"type\": \"m.room.message\"\n}"),
            true,
        );

        assert!(label.contains("Event source selected"));
        assert!(label.contains("opened loaded local EventSourceModal"));
        assert!(label.contains("target $event:example.org in !room:example.org"));
        assert!(label.contains("2 via servers"));
        assert!(label.contains("event id $event:example.org cached"));
        assert!(label.contains("loaded index 7"));
        assert!(label.contains("latest_json loaded"));
        assert!(label.contains(MATRIX_LINK_LOADED_EVENT_SOURCE_MODAL_LABEL));
        assert!(label.contains("Source click sends no follow-up Matrix request"));
        assert!(label.contains("Room::load_or_fetch_event"));
        assert!(label.contains("BackwardsPaginateUntilEvent"));
        assert!(label.contains("event-context window fetch"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(MATRIX_LINK_LOADED_EVENT_SOURCE_MODAL_EVIDENCE.contains("EventSourceModal"));
        assert!(MATRIX_LINK_LOADED_EVENT_SOURCE_MODAL_EVIDENCE.contains("latest_json"));
    }

    #[test]
    fn matrix_link_loaded_event_source_modal_label_reports_missing_loaded_event() {
        let label = matrix_link_loaded_event_source_modal_label(
            "Source route",
            "",
            0,
            "",
            None,
            None,
            false,
        );

        assert!(label.contains("Source route selected"));
        assert!(label.contains("Source stayed local"));
        assert!(label.contains("target waiting"));
        assert!(label.contains("no via servers"));
        assert!(label.contains("event id missing"));
        assert!(label.contains("loaded row unavailable"));
        assert!(label.contains("latest_json unavailable"));
    }

    #[test]
    fn matrix_link_browser_handoff_url_builds_matrix_to_event_via_url() {
        let url = matrix_link_browser_handoff_url(
            "$event:example.org in !room:example.org",
            "$event:example.org",
            "example.org, matrix.org",
        )
        .expect("cached Matrix event link should build matrix.to URL");

        assert_eq!(
            url,
            "https://matrix.to/#/%21room%3Aexample.org/%24event%3Aexample.org?via=example.org&via=matrix.org"
        );
    }

    #[test]
    fn matrix_link_browser_handoff_url_requires_cached_target() {
        assert!(
            matrix_link_browser_handoff_url("   ", "$event:example.org", "example.org").is_none()
        );
    }

    #[test]
    fn matrix_link_browser_handoff_confirmation_label_names_guard_and_boundaries() {
        let label = matrix_link_browser_handoff_confirmation_label(
            "#room:example.org",
            "",
            1,
            Some("https://matrix.to/#/%23room%3Aexample.org?via=example.org"),
        );

        assert!(label.contains("Matrix link Browser handoff"));
        assert!(label.contains("target #room:example.org"));
        assert!(label.contains("1 via server"));
        assert!(label.contains("no event id requested"));
        assert!(label.contains("PositiveConfirmationModal"));
        assert!(label.contains("system opener"));
        assert!(label.contains("matrix.to"));
        assert!(label.contains(MATRIX_LINK_BROWSER_HANDOFF_CONFIRMATION_LABEL));
        assert!(label.contains("No PreviewMatrixLinkTarget"));
        assert!(label.contains("BackwardsPaginateUntilEvent"));
        assert!(label.contains("server-side alias resolution"));
        assert!(label.contains("event context fetch"));
        assert!(label.contains("no join/knock/invite"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(MATRIX_LINK_BROWSER_HANDOFF_CONFIRMATION_EVIDENCE.contains("system opener"));
        assert!(
            MATRIX_LINK_BROWSER_HANDOFF_CONFIRMATION_EVIDENCE.contains("PositiveConfirmationModal")
        );
    }
}

/// Clears all UI-related timeline states for all known rooms.
///
/// This function requires passing in a reference to `Cx`,
/// which isn't used, but acts as a guarantee that this function
/// must only be called by the main UI thread.
pub fn clear_timeline_states(_cx: &mut Cx) {
    // Clear timeline states cache
    TIMELINE_STATES.with_borrow_mut(|states| {
        states.clear();
    });
}

/// Invalidates the UI-side cached state for a timeline whose backend was just closed.
pub fn invalidate_timeline_state(_cx: &mut Cx, kind: &TimelineKind) {
    TIMELINE_STATES.with_borrow_mut(|states| {
        states.remove(kind);
    });
}
