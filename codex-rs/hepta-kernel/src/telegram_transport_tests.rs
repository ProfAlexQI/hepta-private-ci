use super::*;
use crate::HeptaKernelTelegramConfigStatusInput;
use crate::build_hepta_kernel_telegram_config_status;

#[test]
fn kernel_telegram_token_redaction_and_retry_classification_are_bounded() {
    assert!(hepta_kernel_telegram_bot_token_shape_ok(
        "123456789:abcdefghijklmnopqrstuvwxyz"
    ));
    assert!(!hepta_kernel_telegram_bot_token_shape_ok("not-a-token"));
    assert_eq!(
        redact_hepta_kernel_telegram_token_like_text(
            "failed token=123456789:abcdefghijklmnopqrstuvwxyz!"
        ),
        "failed [redacted-telegram-token]"
    );
    assert_eq!(
        hepta_kernel_telegram_bot_api_http_status_error(
            "sendMessage",
            401,
            Some("Unauthorized token=123456789:abcdefghijklmnopqrstuvwxyz")
        ),
        "Telegram Bot API sendMessage HTTP status 401; description=Unauthorized [redacted-telegram-token]"
    );
    assert_eq!(
        hepta_kernel_telegram_bot_api_http_status_error("getUpdates", 500, None),
        "Telegram Bot API getUpdates HTTP status 500; description=missing"
    );
    assert_eq!(
        hepta_kernel_telegram_bot_api_request_failed_error(
            "getUpdates",
            "connection reset token=123456789:abcdefghijklmnopqrstuvwxyz"
        ),
        "Telegram Bot API getUpdates request failed: connection reset [redacted-telegram-token]"
    );
    assert_eq!(
        hepta_kernel_telegram_bot_api_client_build_error(
            "sendMessage",
            "bad proxy 123456789:abcdefghijklmnopqrstuvwxyz"
        ),
        "failed to build Telegram Bot API sendMessage client: bad proxy [redacted-telegram-token]"
    );
    assert_eq!(
        hepta_kernel_telegram_bot_api_json_parse_error(
            "sendMessage",
            "bad json 123456789:abcdefghijklmnopqrstuvwxyz"
        ),
        "failed to parse Telegram Bot API sendMessage response JSON: bad json [redacted-telegram-token]"
    );

    let acked = plan_hepta_kernel_telegram_send_provider_result(
        HeptaKernelTelegramSendProviderResultInput {
            attempt: 1,
            max_attempts: 3,
            api_result: Ok(&json!({"ok": true, "result": {"message_id": 99}})),
        },
    );
    assert_eq!(acked.bot_api_ack, Some(true));
    assert!(acked.external_send);
    assert!(!acked.should_retry);
    assert_eq!(acked.delivery_ledger_stage, Some("acked"));
    assert!(acked.provider_message_id_present);

    let retrying = plan_hepta_kernel_telegram_send_provider_result(
        HeptaKernelTelegramSendProviderResultInput {
            attempt: 1,
            max_attempts: 3,
            api_result: Ok(&json!({"ok": false, "description": "Too Many Requests"})),
        },
    );
    assert_eq!(retrying.bot_api_ack, Some(false));
    assert!(retrying.should_retry);
    assert_eq!(retrying.delivery_ledger_stage, None);
    assert_eq!(retrying.report_status, "sending");

    let terminal = plan_hepta_kernel_telegram_send_provider_result(
        HeptaKernelTelegramSendProviderResultInput {
            attempt: 3,
            max_attempts: 3,
            api_result: Err(
                "Telegram Bot API sendMessage HTTP status 503; token=123456789:abcdefghijklmnopqrstuvwxyz",
            ),
        },
    );
    assert_eq!(terminal.bot_api_ack, None);
    assert!(!terminal.should_retry);
    assert_eq!(terminal.delivery_ledger_stage, Some("failed"));
    assert_eq!(terminal.report_status, "attention");
    assert!(
        terminal
            .error
            .as_deref()
            .unwrap_or_default()
            .contains("[redacted-telegram-token]")
    );

    let read_ok = plan_hepta_kernel_telegram_get_updates_provider_result(
        HeptaKernelTelegramGetUpdatesProviderResultInput {
            attempt: 1,
            max_attempts: 3,
            api_result: Ok(&json!({"ok": true, "result": []})),
        },
    );
    assert_eq!(read_ok.bot_api_ok, Some(true));
    assert!(read_ok.external_read);
    assert!(!read_ok.should_retry);
    assert_eq!(read_ok.report_status, "provider_returned");

    let read_retry = plan_hepta_kernel_telegram_get_updates_provider_result(
        HeptaKernelTelegramGetUpdatesProviderResultInput {
            attempt: 1,
            max_attempts: 3,
            api_result: Err(
                "Telegram Bot API getUpdates request failed 123456789:abcdefghijklmnopqrstuvwxyz",
            ),
        },
    );
    assert!(read_retry.should_retry);
    assert_eq!(read_retry.report_status, "reading");
    assert!(
        read_retry
            .error
            .as_deref()
            .unwrap_or_default()
            .contains("[redacted-telegram-token]")
    );

    let conflict = "Telegram Bot API getUpdates HTTP status 409; description=Conflict: terminated by other getUpdates request";
    let read_conflict = plan_hepta_kernel_telegram_get_updates_provider_result(
        HeptaKernelTelegramGetUpdatesProviderResultInput {
            attempt: 1,
            max_attempts: 3,
            api_result: Err(conflict),
        },
    );
    assert!(!read_conflict.should_retry);
    assert_eq!(read_conflict.report_status, "busy");

    let auth_error = "Telegram Bot API sendMessage HTTP status 401";
    let transient = "Telegram Bot API sendMessage HTTP status 503";
    assert!(hepta_kernel_telegram_get_updates_error_is_conflict(
        conflict
    ));
    assert!(!hepta_kernel_telegram_get_updates_error_is_conflict(
        auth_error
    ));
    assert!(hepta_kernel_telegram_send_error_is_transient(transient));
    assert!(hepta_kernel_telegram_get_updates_error_is_transient(
        "request failed: timed out"
    ));
    assert!(!hepta_kernel_telegram_send_error_is_transient(auth_error));
    assert!(hepta_kernel_telegram_get_updates_should_retry(
        1, 2, transient
    ));
    assert!(!hepta_kernel_telegram_get_updates_should_retry(
        2, 2, transient
    ));
    assert!(!hepta_kernel_telegram_get_updates_should_retry(
        1, 2, conflict
    ));
    assert!(hepta_kernel_telegram_send_should_retry(1, 2, transient));
    assert!(!hepta_kernel_telegram_send_should_retry(2, 2, transient));
    assert!(!hepta_kernel_telegram_send_should_retry(1, 2, auth_error));
}

#[test]
fn kernel_telegram_transport_request_shapes_are_bounded() {
    assert_eq!(
        hepta_kernel_telegram_get_updates_query(999, None),
        vec![
            ("timeout", "0".to_string()),
            ("limit", "20".to_string()),
            (
                "allowed_updates",
                HEPTA_KERNEL_TELEGRAM_ALLOWED_UPDATES.to_string()
            ),
        ]
    );
    assert_eq!(
        hepta_kernel_telegram_get_updates_query(5, Some(43)),
        vec![
            ("timeout", "0".to_string()),
            ("limit", "5".to_string()),
            (
                "allowed_updates",
                HEPTA_KERNEL_TELEGRAM_ALLOWED_UPDATES.to_string()
            ),
            ("offset", "43".to_string()),
        ]
    );
    assert!(
        !hepta_kernel_telegram_get_updates_query(5, Some(-1))
            .iter()
            .any(|(name, _)| *name == "offset")
    );

    let send_body = hepta_kernel_telegram_send_message_request_body(
        "  private model response text  ",
        6476198178,
        Some(11),
    )
    .expect("send body");
    assert_eq!(
        send_body.get("chat_id").and_then(Value::as_i64),
        Some(6476198178)
    );
    assert_eq!(
        send_body.get("text").and_then(Value::as_str),
        Some("private model response text")
    );
    assert_eq!(
        send_body
            .pointer("/reply_parameters/message_id")
            .and_then(Value::as_i64),
        Some(11)
    );
    assert_eq!(
        send_body
            .pointer("/reply_parameters/allow_sending_without_reply")
            .and_then(Value::as_bool),
        Some(true)
    );
    assert_eq!(
        send_body
            .get("disable_web_page_preview")
            .and_then(Value::as_bool),
        Some(true)
    );
    assert!(send_body.get("parse_mode").is_none());
    assert!(
        hepta_kernel_telegram_send_message_request_body("   ", 6476198178, Some(11))
            .expect_err("empty text rejected")
            .contains("text must be non-empty")
    );
    assert!(
        hepta_kernel_telegram_send_message_request_body("text", 6476198178, Some(0))
            .expect_err("bad reply id rejected")
            .contains("reply message id must be positive")
    );

    let typing_body =
        hepta_kernel_telegram_send_chat_action_request_body(6476198178).expect("typing body");
    assert_eq!(
        typing_body.get("chat_id").and_then(Value::as_i64),
        Some(6476198178)
    );
    assert_eq!(
        typing_body.get("action").and_then(Value::as_str),
        Some("typing")
    );
    assert!(
        hepta_kernel_telegram_send_chat_action_request_body(0)
            .expect_err("bad chat id rejected")
            .contains("chat id must be non-zero")
    );
}

#[test]
fn kernel_telegram_transport_plan_is_side_effect_free() {
    let disabled = HeptaKernelTelegramTransportPlan::disabled();
    assert!(!disabled.bot_api_transport_plan_ready);
    assert_eq!(
        disabled.allowed_updates,
        HEPTA_KERNEL_TELEGRAM_ALLOWED_UPDATES
    );
    assert!(!disabled.external_network_performed_by_status);
    assert!(!disabled.raw_token_exposed);

    let ready = HeptaKernelTelegramTransportPlan::for_config_state(true, true, true);
    assert!(ready.bot_api_transport_plan_ready);
    assert_eq!(ready.get_updates_method, "getUpdates");
    assert_eq!(ready.send_message_method, "sendMessage");
    assert_eq!(ready.send_chat_action_method, "sendChatAction");
    assert!(!ready.external_network_performed_by_status);
    assert!(!ready.raw_token_exposed);
    assert!(
        !HeptaKernelTelegramTransportPlan::for_config_state(true, true, false)
            .bot_api_transport_plan_ready
    );

    let config = build_hepta_kernel_telegram_config_status(HeptaKernelTelegramConfigStatusInput {
        config_path: Some("private/config/openclaw.json".to_string()),
        config_found: true,
        enabled: true,
        dm_policy: "allowlist".to_string(),
        group_policy: "allowlist".to_string(),
        allow_from_count: 1,
        group_count: 1,
        token_source: "secret_file",
        token_secret_ref_present: true,
        token_secret_provider: Some("telegram".to_string()),
        token_secret_id_present: true,
        token_file_present: true,
        token_file_mode_0600: true,
        token_file_security_ready: true,
        token_shape_ok: true,
        error: None,
    });
    assert!(
        hepta_kernel_telegram_transport_plan_for_config_status(&config)
            .bot_api_transport_plan_ready
    );
}

#[test]
fn kernel_telegram_transport_keepalive_and_rate_limit_policies_are_bounded() {
    let token = "123456789:abcdefghijklmnopqrstuvwxyz";
    assert!(hepta_kernel_telegram_typing_keepalive_should_start(
        true, token, 6476198178
    ));
    assert!(!hepta_kernel_telegram_typing_keepalive_should_start(
        false, token, 6476198178
    ));
    assert!(!hepta_kernel_telegram_typing_keepalive_should_start(
        true,
        "not-a-token",
        6476198178
    ));
    assert!(!hepta_kernel_telegram_typing_keepalive_should_start(
        true, token, 0
    ));

    assert_eq!(
        hepta_kernel_telegram_send_rate_limit_sleep_for(None, Duration::from_millis(750)),
        Duration::default()
    );
    assert_eq!(
        hepta_kernel_telegram_send_rate_limit_sleep_for(
            Some(Duration::from_millis(250)),
            Duration::from_millis(750)
        ),
        Duration::from_millis(500)
    );
    assert_eq!(
        hepta_kernel_telegram_send_rate_limit_sleep_for(
            Some(Duration::from_millis(900)),
            Duration::from_millis(750)
        ),
        Duration::default()
    );
    assert_eq!(
        hepta_kernel_telegram_send_rate_limit_sleep_for(Some(Duration::ZERO), Duration::ZERO),
        Duration::default()
    );
}
