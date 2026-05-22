use serde_json::Value;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

pub use hepta_runtime::{
    HEPTA_KERNEL_TELEGRAM_DELIVERY_MAX_RETRIES as TELEGRAM_DELIVERY_MAX_RETRIES,
    HEPTA_KERNEL_TELEGRAM_DELIVERY_STORE_IDENTIFIER as TELEGRAM_DELIVERY_STORE_IDENTIFIER,
    NativeTelegramDeliveryLedgerStatus,
};
use hepta_runtime::{
    NativeTelegramDeliveryLedgerStatusInput, build_native_telegram_delivery_ledger_status,
    native_telegram_delivery_backoff_ms, native_telegram_delivery_error_is_permanent,
    native_telegram_delivery_lifecycle_record,
};

pub fn telegram_delivery_ledger_status(
    requested: bool,
    path: &Path,
    logical_path: &'static str,
) -> NativeTelegramDeliveryLedgerStatus {
    if !requested {
        return build_native_telegram_delivery_ledger_status(
            NativeTelegramDeliveryLedgerStatusInput {
                requested,
                ledger_path: logical_path,
                ledger_file_present: false,
                ledger_updated_at_unix_ms: None,
                raw_jsonl: None,
                read_error: None,
            },
        );
    }

    telegram_delivery_ledger_status_from_path(path, logical_path)
}

pub fn telegram_delivery_ledger_status_from_path(
    path: &Path,
    logical_path: &'static str,
) -> NativeTelegramDeliveryLedgerStatus {
    let ledger_file_present = path.is_file();
    if !ledger_file_present {
        return build_native_telegram_delivery_ledger_status(
            NativeTelegramDeliveryLedgerStatusInput {
                requested: true,
                ledger_path: logical_path,
                ledger_file_present,
                ledger_updated_at_unix_ms: file_modified_unix_ms(path),
                raw_jsonl: None,
                read_error: None,
            },
        );
    }

    let raw = match fs::read_to_string(path) {
        Ok(raw) => raw,
        Err(error) => {
            let error = format!("failed to read Telegram delivery ledger: {error}");
            return build_native_telegram_delivery_ledger_status(
                NativeTelegramDeliveryLedgerStatusInput {
                    requested: true,
                    ledger_path: logical_path,
                    ledger_file_present,
                    ledger_updated_at_unix_ms: file_modified_unix_ms(path),
                    raw_jsonl: None,
                    read_error: Some(&error),
                },
            );
        }
    };
    build_native_telegram_delivery_ledger_status(NativeTelegramDeliveryLedgerStatusInput {
        requested: true,
        ledger_path: logical_path,
        ledger_file_present,
        ledger_updated_at_unix_ms: file_modified_unix_ms(path),
        raw_jsonl: Some(&raw),
        read_error: None,
    })
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
    native_telegram_delivery_lifecycle_record(
        stage,
        candidate_next_update_offset,
        model_output_present,
        provider_send_attempted,
        bot_api_ack,
        provider_message_id_present,
        error,
        now_unix_ms() / 1_000,
    )
}

pub fn telegram_delivery_backoff_ms(next_retry_count: u32) -> u64 {
    native_telegram_delivery_backoff_ms(next_retry_count)
}

pub fn telegram_delivery_error_is_permanent(error: Option<&str>) -> bool {
    native_telegram_delivery_error_is_permanent(error)
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
