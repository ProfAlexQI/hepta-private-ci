use std::fs;
use std::path::Path;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::Serialize;
use serde_json::Value;

pub const DEFAULT_TELEGRAM_INGRESS_CURSOR_PATH: &str = ".hepta/telegram/ingress-drain-cursor.json";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NativeTelegramCursorStatus {
    pub product: &'static str,
    pub runtime: &'static str,
    pub requested: bool,
    pub status: &'static str,
    pub cursor_path: &'static str,
    pub cursor_file_present: bool,
    pub cursor_parse_ok: bool,
    pub next_update_offset: Option<i64>,
    pub cursor_updated_at_unix_ms: Option<u64>,
    pub last_delivered_next_update_offset: Option<i64>,
    pub durable_cursor_evidence_present: bool,
    pub cursor_represents_next_update_offset: bool,
    pub duplicate_suppression_rule_valid: bool,
    pub cursor_write_policy: &'static str,
    pub cursor_written: bool,
    pub raw_update_payload_persisted: bool,
    pub error: Option<String>,
    pub next_migration_slice: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NativeTelegramCursorPlan {
    pub cursor_path: &'static str,
    pub duplicate_suppression_ready: bool,
    pub duplicate_suppression_rule_valid: bool,
    pub cursor_represents_next_update_offset: bool,
    pub commit_offset_after_delivery: bool,
    pub raw_update_payload_persisted: bool,
}

impl NativeTelegramCursorPlan {
    pub fn disabled() -> Self {
        Self {
            cursor_path: DEFAULT_TELEGRAM_INGRESS_CURSOR_PATH,
            duplicate_suppression_ready: false,
            duplicate_suppression_rule_valid: true,
            cursor_represents_next_update_offset: true,
            commit_offset_after_delivery: false,
            raw_update_payload_persisted: false,
        }
    }

    pub fn ready() -> Self {
        Self {
            cursor_path: DEFAULT_TELEGRAM_INGRESS_CURSOR_PATH,
            duplicate_suppression_ready: true,
            duplicate_suppression_rule_valid: telegram_update_already_drained(41, Some(42))
                && !telegram_update_already_drained(42, Some(42)),
            cursor_represents_next_update_offset: true,
            commit_offset_after_delivery: true,
            raw_update_payload_persisted: false,
        }
    }
}

pub fn telegram_cursor_status(requested: bool, path: &Path) -> NativeTelegramCursorStatus {
    if !requested {
        return NativeTelegramCursorStatus {
            product: "Hepta",
            runtime: "hepta-codex",
            requested,
            status: "disabled",
            cursor_path: DEFAULT_TELEGRAM_INGRESS_CURSOR_PATH,
            cursor_file_present: false,
            cursor_parse_ok: false,
            next_update_offset: None,
            cursor_updated_at_unix_ms: None,
            last_delivered_next_update_offset: None,
            durable_cursor_evidence_present: false,
            cursor_represents_next_update_offset: true,
            duplicate_suppression_rule_valid: true,
            cursor_write_policy: "disabled",
            cursor_written: false,
            raw_update_payload_persisted: false,
            error: None,
            next_migration_slice: "enable Telegram plugin before reading cursor state",
        };
    }

    telegram_cursor_status_from_path(path)
}

pub fn telegram_cursor_status_from_path(path: &Path) -> NativeTelegramCursorStatus {
    let cursor_file_present = path.is_file();
    let mut status = NativeTelegramCursorStatus {
        product: "Hepta",
        runtime: "hepta-codex",
        requested: true,
        status: "missing",
        cursor_path: DEFAULT_TELEGRAM_INGRESS_CURSOR_PATH,
        cursor_file_present,
        cursor_parse_ok: false,
        next_update_offset: None,
        cursor_updated_at_unix_ms: None,
        last_delivered_next_update_offset: None,
        durable_cursor_evidence_present: false,
        cursor_represents_next_update_offset: true,
        duplicate_suppression_rule_valid: telegram_update_already_drained(41, Some(42))
            && !telegram_update_already_drained(42, Some(42)),
        cursor_write_policy: "write only after model output is delivered or duplicate suppression is recorded",
        cursor_written: false,
        raw_update_payload_persisted: false,
        error: None,
        next_migration_slice: "wire cursor write after gated send delivery success",
    };

    if !cursor_file_present {
        return status;
    }

    match fs::read_to_string(path)
        .map_err(|error| format!("failed to read Telegram cursor file: {error}"))
        .and_then(|raw| {
            let next_update_offset = parse_telegram_cursor_next_update_offset(&raw)?;
            Ok((raw, next_update_offset))
        }) {
        Ok((raw, next_update_offset)) => {
            let cursor_json = serde_json::from_str::<Value>(&raw).unwrap_or(Value::Null);
            let raw_update_payload_persisted = cursor_json
                .get("raw_update_payload_persisted")
                .and_then(Value::as_bool)
                .unwrap_or(false);
            let cursor_updated_at_unix_ms = cursor_json
                .get("updated_at_unix_ms")
                .and_then(Value::as_u64)
                .or_else(|| file_modified_unix_ms(path));
            let last_delivered_next_update_offset = cursor_json
                .get("last_delivered_next_update_offset")
                .and_then(Value::as_i64)
                .filter(|offset| *offset >= 0)
                .or(Some(next_update_offset));
            status.status = "ready";
            status.cursor_parse_ok = true;
            status.next_update_offset = Some(next_update_offset);
            status.cursor_updated_at_unix_ms = cursor_updated_at_unix_ms;
            status.last_delivered_next_update_offset = last_delivered_next_update_offset;
            status.durable_cursor_evidence_present = cursor_updated_at_unix_ms.is_some()
                && last_delivered_next_update_offset.is_some()
                && !raw_update_payload_persisted;
            status.raw_update_payload_persisted = raw_update_payload_persisted;
            status.next_migration_slice = "cursor is ready; continue active soak and expect writes only after delivery or duplicate suppression";
        }
        Err(error) => {
            status.status = "attention";
            status.error = Some(error);
        }
    }

    status
}

pub fn parse_telegram_cursor_next_update_offset(raw: &str) -> Result<i64, String> {
    let value: Value = serde_json::from_str(raw)
        .map_err(|error| format!("failed to parse Telegram cursor JSON: {error}"))?;
    let explicit_next_update_offset = value
        .get("next_update_offset")
        .or_else(|| value.get("nextUpdateOffset"))
        .and_then(Value::as_i64)
        .or_else(|| {
            value
                .get("next_server_offset")
                .or_else(|| value.get("nextServerOffset"))
                .and_then(Value::as_i64)
        });
    let legacy_last_drained_next_offset = value
        .get("last_drained_update_id")
        .or_else(|| value.get("lastDrainedUpdateId"))
        .and_then(Value::as_i64)
        .filter(|offset| *offset >= 0)
        .and_then(|offset| offset.checked_add(1));
    let offset = explicit_next_update_offset
        .or(legacy_last_drained_next_offset)
        .ok_or_else(|| {
            "Telegram cursor missing next_update_offset or legacy next_server_offset".to_string()
        })?;
    if offset < 0 {
        Err("Telegram cursor next_update_offset must be non-negative".to_string())
    } else {
        Ok(offset)
    }
}

pub fn write_telegram_cursor_next_update_offset(path: &Path, offset: i64) -> Result<(), String> {
    if offset < 0 {
        return Err("Telegram cursor next_update_offset must be non-negative".to_string());
    }
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        fs::create_dir_all(parent)
            .map_err(|error| format!("failed to create Telegram cursor directory: {error}"))?;
    }
    let body = serde_json::json!({
        "schema": "hepta.telegram.cursor.v1",
        "next_update_offset": offset,
        "updated_at_unix_ms": now_unix_ms(),
        "last_delivered_next_update_offset": offset,
        "raw_update_payload_persisted": false,
    });
    let raw = serde_json::to_string_pretty(&body)
        .map_err(|error| format!("failed to encode Telegram cursor JSON: {error}"))?;
    fs::write(path, format!("{raw}\n"))
        .map_err(|error| format!("failed to write Telegram cursor file: {error}"))
}

fn telegram_update_already_drained(update_id: i64, next_update_offset: Option<i64>) -> bool {
    next_update_offset
        .map(|cursor| update_id < cursor)
        .unwrap_or(false)
}

fn duration_millis_u64(duration: Duration) -> u64 {
    duration.as_millis().min(u128::from(u64::MAX)) as u64
}

fn now_unix_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(duration_millis_u64)
        .unwrap_or(0)
}

fn file_modified_unix_ms(path: &Path) -> Option<u64> {
    fs::metadata(path)
        .ok()
        .and_then(|metadata| metadata.modified().ok())
        .and_then(|modified| modified.duration_since(UNIX_EPOCH).ok())
        .map(duration_millis_u64)
}

#[cfg(test)]
mod tests {
    use super::{
        parse_telegram_cursor_next_update_offset, telegram_cursor_status_from_path,
        write_telegram_cursor_next_update_offset,
    };
    use std::fs;

    #[test]
    fn cursor_status_reads_next_update_offset_without_writing() {
        let temp = tempfile::tempdir().expect("tempdir");
        let cursor_path = temp.path().join("cursor.json");
        fs::write(
            &cursor_path,
            r#"{"next_update_offset": 43, "updated_at_unix_ms": 123}"#,
        )
        .expect("write cursor");

        let status = telegram_cursor_status_from_path(&cursor_path);
        assert_eq!(status.status, "ready");
        assert!(status.cursor_file_present);
        assert!(status.cursor_parse_ok);
        assert_eq!(status.next_update_offset, Some(43));
        assert_eq!(status.cursor_updated_at_unix_ms, Some(123));
        assert_eq!(status.last_delivered_next_update_offset, Some(43));
        assert!(status.durable_cursor_evidence_present);
        assert!(status.cursor_represents_next_update_offset);
        assert!(status.duplicate_suppression_rule_valid);
        assert!(!status.cursor_written);
        assert!(!status.raw_update_payload_persisted);
    }

    #[test]
    fn cursor_status_reads_legacy_next_server_offset_without_writing() {
        let temp = tempfile::tempdir().expect("tempdir");
        let cursor_path = temp.path().join("cursor.json");
        fs::write(
            &cursor_path,
            r#"{"next_server_offset": 917025960, "last_drained_update_id": 917025959}"#,
        )
        .expect("write cursor");

        let status = telegram_cursor_status_from_path(&cursor_path);
        assert_eq!(status.status, "ready");
        assert!(status.cursor_file_present);
        assert!(status.cursor_parse_ok);
        assert_eq!(status.next_update_offset, Some(917025960));
        assert!(status.cursor_updated_at_unix_ms.is_some());
        assert_eq!(status.last_delivered_next_update_offset, Some(917025960));
        assert!(status.durable_cursor_evidence_present);
        assert!(status.cursor_represents_next_update_offset);
        assert!(status.duplicate_suppression_rule_valid);
        assert!(!status.cursor_written);
        assert!(!status.raw_update_payload_persisted);
    }

    #[test]
    fn cursor_status_derives_legacy_next_offset_from_last_drained_id() {
        let temp = tempfile::tempdir().expect("tempdir");
        let cursor_path = temp.path().join("cursor.json");
        fs::write(&cursor_path, r#"{"last_drained_update_id": 917025959}"#).expect("write cursor");

        let status = telegram_cursor_status_from_path(&cursor_path);
        assert_eq!(status.status, "ready");
        assert!(status.cursor_file_present);
        assert!(status.cursor_parse_ok);
        assert_eq!(status.next_update_offset, Some(917025960));
        assert!(status.cursor_updated_at_unix_ms.is_some());
        assert_eq!(status.last_delivered_next_update_offset, Some(917025960));
        assert!(status.durable_cursor_evidence_present);
        assert!(!status.cursor_written);
        assert!(!status.raw_update_payload_persisted);
    }

    #[test]
    fn cursor_status_rejects_negative_offsets() {
        let temp = tempfile::tempdir().expect("tempdir");
        let cursor_path = temp.path().join("cursor.json");
        fs::write(&cursor_path, r#"{"next_update_offset": -1}"#).expect("write cursor");

        let status = telegram_cursor_status_from_path(&cursor_path);
        assert_eq!(status.status, "attention");
        assert!(status.cursor_file_present);
        assert!(!status.cursor_parse_ok);
        assert_eq!(status.next_update_offset, None);
        assert!(
            status
                .error
                .unwrap()
                .contains("next_update_offset must be non-negative")
        );
    }

    #[test]
    fn cursor_write_helper_persists_next_offset_without_raw_payload() {
        let temp = tempfile::tempdir().expect("tempdir");
        let cursor_path = temp.path().join("nested").join("cursor.json");

        write_telegram_cursor_next_update_offset(&cursor_path, 77).expect("write cursor");
        let raw = fs::read_to_string(&cursor_path).expect("read cursor");
        assert!(raw.contains("\"schema\": \"hepta.telegram.cursor.v1\""));
        assert!(raw.contains("\"next_update_offset\": 77"));
        assert!(raw.contains("\"updated_at_unix_ms\""));
        assert!(raw.contains("\"last_delivered_next_update_offset\": 77"));
        assert!(raw.contains("\"raw_update_payload_persisted\": false"));

        let status = telegram_cursor_status_from_path(&cursor_path);
        assert_eq!(status.status, "ready");
        assert_eq!(status.next_update_offset, Some(77));
        assert!(status.cursor_updated_at_unix_ms.is_some());
        assert_eq!(status.last_delivered_next_update_offset, Some(77));
        assert!(status.durable_cursor_evidence_present);
        assert!(!status.cursor_written);
        assert!(!status.raw_update_payload_persisted);
    }

    #[test]
    fn cursor_write_helper_rejects_negative_offsets() {
        let temp = tempfile::tempdir().expect("tempdir");
        let cursor_path = temp.path().join("cursor.json");

        let err = write_telegram_cursor_next_update_offset(&cursor_path, -1)
            .expect_err("negative offset should be rejected");
        assert!(err.contains("next_update_offset must be non-negative"));
        assert!(!cursor_path.exists());
    }

    #[test]
    fn parser_accepts_explicit_and_legacy_cursor_shapes() {
        assert_eq!(
            parse_telegram_cursor_next_update_offset(r#"{"next_update_offset": 5}"#),
            Ok(5)
        );
        assert_eq!(
            parse_telegram_cursor_next_update_offset(r#"{"nextServerOffset": 6}"#),
            Ok(6)
        );
        assert_eq!(
            parse_telegram_cursor_next_update_offset(r#"{"lastDrainedUpdateId": 6}"#),
            Ok(7)
        );
    }
}
