use serde::Serialize;
use serde_json::Value;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

pub const TELEGRAM_DELIVERY_STORE_IDENTIFIER: &str = "/store/delivery";
pub const TELEGRAM_DELIVERY_MAX_RETRIES: u32 = 5;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NativeTelegramDeliveryLedgerStatus {
    pub product: &'static str,
    pub runtime: &'static str,
    pub requested: bool,
    pub status: &'static str,
    pub ledger_path: &'static str,
    pub ledger_file_present: bool,
    pub jsonl_readable: bool,
    pub jsonl_valid: bool,
    pub line_count: usize,
    pub valid_json_line_count: usize,
    pub invalid_json_line_count: usize,
    pub acked_count: usize,
    pub failed_count: usize,
    pub latest_stage: Option<String>,
    pub latest_created_unix_seconds: Option<u64>,
    pub latest_acked_created_unix_seconds: Option<u64>,
    pub ledger_updated_at_unix_ms: Option<u64>,
    pub provider_message_id_present: bool,
    pub durable_delivery_evidence_present: bool,
    pub raw_response_text_logged: bool,
    pub raw_chat_id_logged: bool,
    pub raw_message_id_logged: bool,
    pub raw_token_logged: bool,
    pub error: Option<String>,
    pub next_migration_slice: &'static str,
}

pub fn telegram_delivery_ledger_status(
    requested: bool,
    path: &Path,
    logical_path: &'static str,
) -> NativeTelegramDeliveryLedgerStatus {
    if !requested {
        return NativeTelegramDeliveryLedgerStatus {
            product: "Hepta",
            runtime: "hepta-codex",
            requested,
            status: "disabled",
            ledger_path: logical_path,
            ledger_file_present: false,
            jsonl_readable: false,
            jsonl_valid: false,
            line_count: 0,
            valid_json_line_count: 0,
            invalid_json_line_count: 0,
            acked_count: 0,
            failed_count: 0,
            latest_stage: None,
            latest_created_unix_seconds: None,
            latest_acked_created_unix_seconds: None,
            ledger_updated_at_unix_ms: None,
            provider_message_id_present: false,
            durable_delivery_evidence_present: false,
            raw_response_text_logged: false,
            raw_chat_id_logged: false,
            raw_message_id_logged: false,
            raw_token_logged: false,
            error: None,
            next_migration_slice: "enable Telegram plugin before reading delivery ledger state",
        };
    }

    telegram_delivery_ledger_status_from_path(path, logical_path)
}

pub fn telegram_delivery_ledger_status_from_path(
    path: &Path,
    logical_path: &'static str,
) -> NativeTelegramDeliveryLedgerStatus {
    let ledger_file_present = path.is_file();
    let mut status = NativeTelegramDeliveryLedgerStatus {
        product: "Hepta",
        runtime: "hepta-codex",
        requested: true,
        status: "missing",
        ledger_path: logical_path,
        ledger_file_present,
        jsonl_readable: false,
        jsonl_valid: false,
        line_count: 0,
        valid_json_line_count: 0,
        invalid_json_line_count: 0,
        acked_count: 0,
        failed_count: 0,
        latest_stage: None,
        latest_created_unix_seconds: None,
        latest_acked_created_unix_seconds: None,
        ledger_updated_at_unix_ms: file_modified_unix_ms(path),
        provider_message_id_present: false,
        durable_delivery_evidence_present: false,
        raw_response_text_logged: false,
        raw_chat_id_logged: false,
        raw_message_id_logged: false,
        raw_token_logged: false,
        error: None,
        next_migration_slice: "delivery ledger is empty until native Telegram send is approved and delivered",
    };

    if !ledger_file_present {
        return status;
    }

    let raw = match fs::read_to_string(path) {
        Ok(raw) => raw,
        Err(error) => {
            status.status = "attention";
            status.error = Some(redact_token_like_text(&format!(
                "failed to read Telegram delivery ledger: {error}"
            )));
            return status;
        }
    };
    status.jsonl_readable = true;
    for line in raw.lines().filter(|line| !line.trim().is_empty()) {
        status.line_count = status.line_count.saturating_add(1);
        let Ok(record) = serde_json::from_str::<Value>(line) else {
            status.invalid_json_line_count = status.invalid_json_line_count.saturating_add(1);
            continue;
        };
        status.valid_json_line_count = status.valid_json_line_count.saturating_add(1);
        let stage = record
            .get("stage")
            .and_then(Value::as_str)
            .unwrap_or("unknown")
            .to_string();
        let record_created_unix_seconds =
            record.get("created_unix_seconds").and_then(Value::as_u64);
        if stage == "acked" {
            status.acked_count = status.acked_count.saturating_add(1);
            if let Some(created) = record_created_unix_seconds {
                status.latest_acked_created_unix_seconds = Some(
                    status
                        .latest_acked_created_unix_seconds
                        .map_or(created, |latest| latest.max(created)),
                );
            }
        } else if stage == "failed" {
            status.failed_count = status.failed_count.saturating_add(1);
        }
        status.latest_stage = Some(stage);
        if let Some(created) = record_created_unix_seconds {
            status.latest_created_unix_seconds = Some(
                status
                    .latest_created_unix_seconds
                    .map_or(created, |latest| latest.max(created)),
            );
        }
        status.provider_message_id_present |= record
            .get("provider_message_id_present")
            .and_then(Value::as_bool)
            .unwrap_or(false);
        status.raw_response_text_logged |= record
            .get("content_logged")
            .and_then(Value::as_bool)
            .unwrap_or(false)
            || record
                .get("message_text_logged")
                .and_then(Value::as_bool)
                .unwrap_or(false);
        status.raw_chat_id_logged |= record
            .get("raw_chat_id_logged")
            .and_then(Value::as_bool)
            .unwrap_or(false);
        status.raw_message_id_logged |= record
            .get("raw_message_id_logged")
            .and_then(Value::as_bool)
            .unwrap_or(false);
        status.raw_token_logged |= record
            .get("raw_token_logged")
            .and_then(Value::as_bool)
            .unwrap_or(false);
    }

    status.jsonl_valid = status.invalid_json_line_count == 0;
    status.durable_delivery_evidence_present =
        status.acked_count > 0 && status.provider_message_id_present && status.jsonl_valid;
    status.status = if !status.jsonl_valid
        || status.raw_response_text_logged
        || status.raw_chat_id_logged
        || status.raw_message_id_logged
        || status.raw_token_logged
    {
        "attention"
    } else if status.durable_delivery_evidence_present {
        "ready"
    } else {
        "empty"
    };
    status.next_migration_slice = if status.status == "ready" {
        "delivery ledger has durable redacted ack evidence; keep it aligned with cursor commits"
    } else {
        "write redacted enqueued/acked delivery records before committing Telegram cursor offsets"
    };
    status
}

pub fn append_telegram_delivery_lifecycle_record(
    path: &Path,
    record: &Value,
) -> Result<(), String> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        fs::create_dir_all(parent).map_err(|error| {
            format!(
                "failed to create Telegram delivery ledger directory {}: {error}",
                parent.display()
            )
        })?;
    }
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|error| {
            format!(
                "failed to open Telegram delivery ledger {}: {error}",
                path.display()
            )
        })?;
    let bytes = serde_json::to_vec(record)
        .map_err(|error| format!("failed to render Telegram delivery ledger record: {error}"))?;
    file.write_all(&bytes).map_err(|error| {
        format!(
            "failed to write Telegram delivery ledger {}: {error}",
            path.display()
        )
    })?;
    file.write_all(b"\n").map_err(|error| {
        format!(
            "failed to finalize Telegram delivery ledger {}: {error}",
            path.display()
        )
    })
}

pub fn telegram_delivery_lifecycle_record(
    stage: &'static str,
    candidate_next_update_offset: Option<i64>,
    model_output_present: bool,
    provider_send_attempted: bool,
    bot_api_ack: Option<bool>,
    provider_message_id_present: bool,
    error: Option<&str>,
) -> Value {
    let acked = stage == "acked" && bot_api_ack == Some(true);
    let failed = stage == "failed";
    let permanent_error = failed && telegram_delivery_error_is_permanent(error);
    let retry_scheduled = failed && !permanent_error;
    let next_retry_count = if retry_scheduled { 1 } else { 0 };
    serde_json::json!({
        "schema_version": 1,
        "store_identifier": TELEGRAM_DELIVERY_STORE_IDENTIFIER,
        "entry_id": candidate_next_update_offset
            .map(|offset| format!("telegram:next-offset:{offset}"))
            .unwrap_or_else(|| "telegram:next-offset:missing".to_string()),
        "idempotency_key": candidate_next_update_offset
            .map(|offset| format!("telegram:next-offset:{offset}"))
            .unwrap_or_else(|| "telegram:next-offset:missing".to_string()),
        "stage": stage,
        "created_unix_seconds": now_unix_ms() / 1_000,
        "channel": "telegram",
        "session_key_shape": "agent:main:telegram:[redacted]",
        "payload_count": usize::from(model_output_present),
        "payload_text_chunk_count": usize::from(model_output_present),
        "payload_media_count": 0,
        "payload_button_count": 0,
        "content_logged": false,
        "message_text_logged": false,
        "raw_chat_id_logged": false,
        "raw_message_id_logged": false,
        "raw_token_logged": false,
        "enqueue_before_provider_send": true,
        "active_claim_required": true,
        "active_claim_acquired": true,
        "provider_send_attempted": provider_send_attempted,
        "provider_message_id_present": provider_message_id_present,
        "ack_after_provider_message_id": acked,
        "acked": acked,
        "failed": failed,
        "retry_scheduled": retry_scheduled,
        "next_retry_count": next_retry_count,
        "next_retry_backoff_ms": retry_scheduled.then(|| telegram_delivery_backoff_ms(next_retry_count)),
        "max_retries": TELEGRAM_DELIVERY_MAX_RETRIES,
        "permanent_error_moved_to_failed": permanent_error,
        "recovery_replay_supported": true,
        "store_mutated": true,
        "external_send_attempted": provider_send_attempted,
        "error": error.map(redact_token_like_text),
    })
}

pub fn telegram_delivery_backoff_ms(next_retry_count: u32) -> u64 {
    match next_retry_count {
        0 => 0,
        1 => 5_000,
        2 => 25_000,
        3 => 120_000,
        _ => 600_000,
    }
}

pub fn telegram_delivery_error_is_permanent(error: Option<&str>) -> bool {
    let Some(error) = error.map(str::to_ascii_lowercase) else {
        return false;
    };
    error.contains("unauthorized")
        || error.contains("forbidden")
        || error.contains("bot was blocked")
        || error.contains("chat not found")
        || error.contains("bad request")
}

fn file_modified_unix_ms(path: &Path) -> Option<u64> {
    fs::metadata(path)
        .ok()
        .and_then(|metadata| metadata.modified().ok())
        .and_then(|modified| modified.duration_since(UNIX_EPOCH).ok())
        .map(|duration| duration.as_millis().min(u128::from(u64::MAX)) as u64)
}

fn now_unix_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis().min(u128::from(u64::MAX)) as u64)
        .unwrap_or(0)
}

fn redact_token_like_text(text: &str) -> String {
    text.split_whitespace()
        .map(|part| {
            let trimmed = part.trim_matches(|ch: char| {
                !ch.is_ascii_alphanumeric() && ch != ':' && ch != '_' && ch != '-'
            });
            if token_shape_ok(trimmed) {
                "[redacted-telegram-token]".to_string()
            } else {
                part.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn token_shape_ok(value: &str) -> bool {
    let Some((bot_id, secret)) = value.split_once(':') else {
        return false;
    };
    bot_id.len() >= 6
        && bot_id.chars().all(|ch| ch.is_ascii_digit())
        && secret.len() >= 20
        && secret
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '_' | '-'))
}

#[cfg(test)]
mod tests {
    use super::{
        append_telegram_delivery_lifecycle_record, telegram_delivery_backoff_ms,
        telegram_delivery_error_is_permanent, telegram_delivery_ledger_status,
        telegram_delivery_ledger_status_from_path, telegram_delivery_lifecycle_record,
    };

    #[test]
    fn delivery_ledger_reports_ready_for_redacted_ack() {
        let temp = tempfile::tempdir().expect("tempdir");
        let path = temp.path().join("delivery-ledger.jsonl");
        std::fs::write(
            &path,
            concat!(
                r#"{"stage":"enqueued","created_unix_seconds":1,"provider_message_id_present":false,"content_logged":false,"raw_chat_id_logged":false,"raw_message_id_logged":false,"raw_token_logged":false}"#,
                "\n",
                r#"{"stage":"acked","created_unix_seconds":2,"provider_message_id_present":true,"content_logged":false,"raw_chat_id_logged":false,"raw_message_id_logged":false,"raw_token_logged":false}"#,
                "\n",
            ),
        )
        .expect("write ledger");

        let status = telegram_delivery_ledger_status_from_path(
            &path,
            ".hepta/telegram/delivery-ledger.jsonl",
        );

        assert_eq!(status.status, "ready");
        assert_eq!(status.line_count, 2);
        assert_eq!(status.acked_count, 1);
        assert_eq!(status.failed_count, 0);
        assert!(status.durable_delivery_evidence_present);
        assert!(!status.raw_response_text_logged);
    }

    #[test]
    fn delivery_ledger_disabled_status_is_side_effect_free() {
        let temp = tempfile::tempdir().expect("tempdir");

        let status = telegram_delivery_ledger_status(
            false,
            &temp.path().join("missing.jsonl"),
            ".hepta/telegram/delivery-ledger.jsonl",
        );

        assert_eq!(status.status, "disabled");
        assert_eq!(status.ledger_file_present, false);
        assert_eq!(status.line_count, 0);
    }

    #[test]
    fn delivery_ledger_flags_invalid_json_and_raw_logging() {
        let temp = tempfile::tempdir().expect("tempdir");
        let path = temp.path().join("delivery-ledger.jsonl");
        std::fs::write(
            &path,
            concat!(
                r#"{"stage":"acked","created_unix_seconds":2,"provider_message_id_present":true,"content_logged":true}"#,
                "\n",
                "not-json",
                "\n",
            ),
        )
        .expect("write ledger");

        let status = telegram_delivery_ledger_status_from_path(
            &path,
            ".hepta/telegram/delivery-ledger.jsonl",
        );

        assert_eq!(status.status, "attention");
        assert!(!status.jsonl_valid);
        assert_eq!(status.invalid_json_line_count, 1);
        assert!(status.raw_response_text_logged);
    }

    #[test]
    fn lifecycle_record_redacts_errors_and_classifies_retry() {
        let record = telegram_delivery_lifecycle_record(
            "failed",
            Some(42),
            true,
            true,
            Some(false),
            false,
            Some("transient 123456:ABCDEFGHIJKLMNOPQRSTUVWXYZ_bot_token timeout"),
        );

        assert_eq!(record["store_identifier"], "/store/delivery");
        assert_eq!(record["entry_id"], "telegram:next-offset:42");
        assert_eq!(record["payload_text_chunk_count"], 1);
        assert_eq!(record["failed"], true);
        assert_eq!(record["retry_scheduled"], true);
        assert_eq!(record["next_retry_count"], 1);
        assert_eq!(record["next_retry_backoff_ms"], 5_000);
        assert_eq!(
            record["error"],
            "transient [redacted-telegram-token] timeout"
        );
    }

    #[test]
    fn lifecycle_append_writes_jsonl_and_status_reads_ack() {
        let temp = tempfile::tempdir().expect("tempdir");
        let path = temp.path().join("nested").join("delivery-ledger.jsonl");
        let enqueued =
            telegram_delivery_lifecycle_record("enqueued", Some(7), true, false, None, false, None);
        let acked = telegram_delivery_lifecycle_record(
            "acked",
            Some(7),
            true,
            true,
            Some(true),
            true,
            None,
        );

        append_telegram_delivery_lifecycle_record(&path, &enqueued).expect("append enqueued");
        append_telegram_delivery_lifecycle_record(&path, &acked).expect("append acked");

        let raw = std::fs::read_to_string(&path).expect("read ledger");
        assert_eq!(raw.lines().count(), 2);
        let status = telegram_delivery_ledger_status_from_path(
            &path,
            ".hepta/telegram/delivery-ledger.jsonl",
        );
        assert_eq!(status.status, "ready");
        assert_eq!(status.acked_count, 1);
        assert_eq!(status.latest_stage.as_deref(), Some("acked"));
    }

    #[test]
    fn delivery_error_classification_and_backoff_are_stable() {
        assert!(telegram_delivery_error_is_permanent(Some(
            "Forbidden: bot was blocked by the user"
        )));
        assert!(telegram_delivery_error_is_permanent(Some(
            "Bad Request: chat not found"
        )));
        assert!(!telegram_delivery_error_is_permanent(Some(
            "Too Many Requests: retry after 1"
        )));
        assert_eq!(telegram_delivery_backoff_ms(0), 0);
        assert_eq!(telegram_delivery_backoff_ms(1), 5_000);
        assert_eq!(telegram_delivery_backoff_ms(2), 25_000);
        assert_eq!(telegram_delivery_backoff_ms(3), 120_000);
        assert_eq!(telegram_delivery_backoff_ms(4), 600_000);
    }
}
