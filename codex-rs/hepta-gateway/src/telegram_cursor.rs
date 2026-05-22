use std::fs;
use std::path::Path;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub use hepta_runtime::{
    HEPTA_KERNEL_TELEGRAM_INGRESS_CURSOR_PATH as DEFAULT_TELEGRAM_INGRESS_CURSOR_PATH,
    NativeTelegramCursorPlan, NativeTelegramCursorStatus,
};
use hepta_runtime::{
    NativeTelegramCursorStatusInput, build_native_telegram_cursor_status,
    native_telegram_cursor_body, parse_native_telegram_cursor_next_update_offset,
};

pub fn telegram_cursor_status(requested: bool, path: &Path) -> NativeTelegramCursorStatus {
    if !requested {
        return build_native_telegram_cursor_status(NativeTelegramCursorStatusInput {
            requested,
            cursor_path: DEFAULT_TELEGRAM_INGRESS_CURSOR_PATH,
            cursor_file_present: false,
            cursor_updated_at_unix_ms: None,
            raw_json: None,
            read_error: None,
        });
    }

    telegram_cursor_status_from_path(path)
}

pub fn telegram_cursor_status_from_path(path: &Path) -> NativeTelegramCursorStatus {
    let cursor_file_present = path.is_file();
    if !cursor_file_present {
        return build_native_telegram_cursor_status(NativeTelegramCursorStatusInput {
            requested: true,
            cursor_path: DEFAULT_TELEGRAM_INGRESS_CURSOR_PATH,
            cursor_file_present,
            cursor_updated_at_unix_ms: file_modified_unix_ms(path),
            raw_json: None,
            read_error: None,
        });
    }

    let raw = match fs::read_to_string(path) {
        Ok(raw) => raw,
        Err(error) => {
            let error = format!("failed to read Telegram cursor file: {error}");
            return build_native_telegram_cursor_status(NativeTelegramCursorStatusInput {
                requested: true,
                cursor_path: DEFAULT_TELEGRAM_INGRESS_CURSOR_PATH,
                cursor_file_present,
                cursor_updated_at_unix_ms: file_modified_unix_ms(path),
                raw_json: None,
                read_error: Some(&error),
            });
        }
    };

    build_native_telegram_cursor_status(NativeTelegramCursorStatusInput {
        requested: true,
        cursor_path: DEFAULT_TELEGRAM_INGRESS_CURSOR_PATH,
        cursor_file_present,
        cursor_updated_at_unix_ms: file_modified_unix_ms(path),
        raw_json: Some(&raw),
        read_error: None,
    })
}

pub fn parse_telegram_cursor_next_update_offset(raw: &str) -> Result<i64, String> {
    parse_native_telegram_cursor_next_update_offset(raw)
}

pub fn write_telegram_cursor_next_update_offset(path: &Path, offset: i64) -> Result<(), String> {
    let body = native_telegram_cursor_body(offset, now_unix_ms())?;
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        fs::create_dir_all(parent)
            .map_err(|error| format!("failed to create Telegram cursor directory: {error}"))?;
    }
    let raw = serde_json::to_string_pretty(&body)
        .map_err(|error| format!("failed to encode Telegram cursor JSON: {error}"))?;
    fs::write(path, format!("{raw}\n"))
        .map_err(|error| format!("failed to write Telegram cursor file: {error}"))
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
