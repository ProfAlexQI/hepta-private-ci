use super::*;
use crate::HEPTA_KERNEL_TELEGRAM_CURSOR_SCHEMA;
use crate::HEPTA_KERNEL_TELEGRAM_INGRESS_CURSOR_PATH;

#[test]
fn kernel_telegram_cursor_parser_accepts_current_and_legacy_shapes() {
    assert_eq!(
        parse_hepta_kernel_telegram_cursor_next_update_offset(r#"{"next_update_offset": 5}"#),
        Ok(5)
    );
    assert_eq!(
        parse_hepta_kernel_telegram_cursor_next_update_offset(r#"{"nextUpdateOffset": 6}"#),
        Ok(6)
    );
    assert_eq!(
        parse_hepta_kernel_telegram_cursor_next_update_offset(r#"{"next_server_offset": 7}"#),
        Ok(7)
    );
    assert_eq!(
        parse_hepta_kernel_telegram_cursor_next_update_offset(r#"{"nextServerOffset": 8}"#),
        Ok(8)
    );
    assert_eq!(
        parse_hepta_kernel_telegram_cursor_next_update_offset(r#"{"lastDrainedUpdateId": 8}"#),
        Ok(9)
    );
}

#[test]
fn kernel_telegram_cursor_policy_rejects_invalid_offsets_and_shapes() {
    assert!(
        parse_hepta_kernel_telegram_cursor_next_update_offset(r#"{"next_update_offset": -1}"#)
            .expect_err("negative offset should fail")
            .contains("next_update_offset must be non-negative")
    );
    assert!(
        parse_hepta_kernel_telegram_cursor_next_update_offset(r#"{"lastDrainedUpdateId": -1}"#)
            .expect_err("negative legacy offset should fail")
            .contains("missing next_update_offset")
    );
    assert!(
        parse_hepta_kernel_telegram_cursor_next_update_offset(r#"{}"#)
            .expect_err("missing offset should fail")
            .contains("missing next_update_offset")
    );
    assert!(
        hepta_kernel_telegram_cursor_body(-1, 123)
            .expect_err("negative body offset should fail")
            .contains("next_update_offset must be non-negative")
    );
}

#[test]
fn kernel_telegram_cursor_body_is_stable_and_payload_safe() {
    assert_eq!(
        HEPTA_KERNEL_TELEGRAM_INGRESS_CURSOR_PATH,
        ".hepta/telegram/ingress-drain-cursor.json"
    );
    assert_eq!(
        HEPTA_KERNEL_TELEGRAM_CURSOR_SCHEMA,
        "hepta.telegram.cursor.v1"
    );

    let body = hepta_kernel_telegram_cursor_body(77, 1_777_777).expect("cursor body");
    assert_eq!(body["schema"], HEPTA_KERNEL_TELEGRAM_CURSOR_SCHEMA);
    assert_eq!(body["next_update_offset"], 77);
    assert_eq!(body["updated_at_unix_ms"], 1_777_777);
    assert_eq!(body["last_delivered_next_update_offset"], 77);
    assert_eq!(body["raw_update_payload_persisted"], false);
    assert!(body.get("raw_update_payload").is_none());
    assert!(body.get("message").is_none());
    assert!(body.get("chat").is_none());
}

#[test]
fn kernel_telegram_cursor_status_summarizes_ready_cursor_without_raw_payload() {
    let status = build_hepta_kernel_telegram_cursor_status(HeptaKernelTelegramCursorStatusInput {
        requested: true,
        cursor_path: HEPTA_KERNEL_TELEGRAM_INGRESS_CURSOR_PATH,
        cursor_file_present: true,
        cursor_updated_at_unix_ms: Some(123),
        raw_json: Some(
            r#"{"next_update_offset": 77, "last_delivered_next_update_offset": 77, "raw_update_payload_persisted": false}"#,
        ),
        read_error: None,
    });

    assert_eq!(status.status, "ready");
    assert!(status.cursor_parse_ok);
    assert_eq!(status.next_update_offset, Some(77));
    assert_eq!(status.cursor_updated_at_unix_ms, Some(123));
    assert_eq!(status.last_delivered_next_update_offset, Some(77));
    assert!(status.durable_cursor_evidence_present);
    assert!(!status.raw_update_payload_persisted);
    assert!(status.duplicate_suppression_rule_valid);
    assert!(!status.cursor_written);
}

#[test]
fn kernel_telegram_cursor_status_flags_raw_payload_and_invalid_cursor() {
    let raw_payload_status =
        build_hepta_kernel_telegram_cursor_status(HeptaKernelTelegramCursorStatusInput {
            requested: true,
            cursor_path: HEPTA_KERNEL_TELEGRAM_INGRESS_CURSOR_PATH,
            cursor_file_present: true,
            cursor_updated_at_unix_ms: Some(123),
            raw_json: Some(r#"{"lastDrainedUpdateId": 6, "raw_update_payload_persisted": true}"#),
            read_error: None,
        });
    assert_eq!(raw_payload_status.status, "ready");
    assert_eq!(raw_payload_status.next_update_offset, Some(7));
    assert!(raw_payload_status.raw_update_payload_persisted);
    assert!(!raw_payload_status.durable_cursor_evidence_present);

    let invalid_status =
        build_hepta_kernel_telegram_cursor_status(HeptaKernelTelegramCursorStatusInput {
            requested: true,
            cursor_path: HEPTA_KERNEL_TELEGRAM_INGRESS_CURSOR_PATH,
            cursor_file_present: true,
            cursor_updated_at_unix_ms: Some(123),
            raw_json: Some(r#"{"next_update_offset": -1}"#),
            read_error: None,
        });
    assert_eq!(invalid_status.status, "attention");
    assert!(!invalid_status.cursor_parse_ok);
    assert!(
        invalid_status
            .error
            .as_deref()
            .unwrap_or_default()
            .contains("next_update_offset must be non-negative")
    );
}

#[test]
fn kernel_telegram_cursor_status_handles_disabled_missing_and_read_error() {
    let disabled =
        build_hepta_kernel_telegram_cursor_status(HeptaKernelTelegramCursorStatusInput {
            requested: false,
            cursor_path: HEPTA_KERNEL_TELEGRAM_INGRESS_CURSOR_PATH,
            cursor_file_present: true,
            cursor_updated_at_unix_ms: Some(123),
            raw_json: Some(r#"{"next_update_offset": 1}"#),
            read_error: None,
        });
    assert_eq!(disabled.status, "disabled");
    assert!(!disabled.cursor_file_present);
    assert_eq!(disabled.cursor_updated_at_unix_ms, None);

    let missing = build_hepta_kernel_telegram_cursor_status(HeptaKernelTelegramCursorStatusInput {
        requested: true,
        cursor_path: HEPTA_KERNEL_TELEGRAM_INGRESS_CURSOR_PATH,
        cursor_file_present: false,
        cursor_updated_at_unix_ms: None,
        raw_json: None,
        read_error: None,
    });
    assert_eq!(missing.status, "missing");
    assert!(!missing.cursor_parse_ok);

    let read_error =
        build_hepta_kernel_telegram_cursor_status(HeptaKernelTelegramCursorStatusInput {
            requested: true,
            cursor_path: HEPTA_KERNEL_TELEGRAM_INGRESS_CURSOR_PATH,
            cursor_file_present: true,
            cursor_updated_at_unix_ms: Some(123),
            raw_json: None,
            read_error: Some(
                "failed to read Telegram cursor file: 123456789:abcdefghijklmnopqrstuvwxyz",
            ),
        });
    assert_eq!(read_error.status, "attention");
    assert!(
        read_error
            .error
            .as_deref()
            .unwrap_or_default()
            .contains("[redacted-telegram-token]")
    );
}

#[test]
fn kernel_telegram_cursor_plan_is_bounded_and_payload_safe() {
    let disabled = HeptaKernelTelegramCursorPlan::disabled();
    assert_eq!(
        disabled.cursor_path,
        HEPTA_KERNEL_TELEGRAM_INGRESS_CURSOR_PATH
    );
    assert!(!disabled.duplicate_suppression_ready);
    assert!(disabled.duplicate_suppression_rule_valid);
    assert!(disabled.cursor_represents_next_update_offset);
    assert!(!disabled.commit_offset_after_delivery);
    assert!(!disabled.raw_update_payload_persisted);

    let ready = HeptaKernelTelegramCursorPlan::ready();
    assert_eq!(ready.cursor_path, HEPTA_KERNEL_TELEGRAM_INGRESS_CURSOR_PATH);
    assert!(ready.duplicate_suppression_ready);
    assert!(ready.duplicate_suppression_rule_valid);
    assert!(ready.cursor_represents_next_update_offset);
    assert!(ready.commit_offset_after_delivery);
    assert!(!ready.raw_update_payload_persisted);
}
