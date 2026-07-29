//! Durable file adapters for the live Telegram delivery pipeline.

use std::path::Path;
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use anyhow::Context;
use anyhow::Result;
use hepta_authority::AuthenticatedJournalStore;
use hepta_runtime::NativeTelegramCursorStatus;
use hepta_runtime::NativeTelegramCursorStatusInput;
use hepta_runtime::build_native_telegram_cursor_status;
use hepta_runtime::native_telegram_cursor_body;
use serde_json::Value;

const MAX_CURSOR_BYTES: u64 = 4096;
const MAX_DELIVERY_LEDGER_BYTES: u64 = 16 * 1024 * 1024;

pub(crate) fn cursor_status(
    requested: bool,
    path: &Path,
    logical_path: &'static str,
) -> NativeTelegramCursorStatus {
    if !requested {
        return build_native_telegram_cursor_status(NativeTelegramCursorStatusInput {
            requested,
            cursor_path: logical_path,
            cursor_file_present: false,
            cursor_updated_at_unix_ms: None,
            raw_json: None,
            read_error: None,
        });
    }

    let store = private_store(path, MAX_CURSOR_BYTES, "hepta-telegram-cursor");
    match store.and_then(|store| store.read_snapshot()) {
        Ok(None) => build_native_telegram_cursor_status(NativeTelegramCursorStatusInput {
            requested: true,
            cursor_path: logical_path,
            cursor_file_present: false,
            cursor_updated_at_unix_ms: None,
            raw_json: None,
            read_error: None,
        }),
        Ok(Some(opened)) => {
            let raw = match String::from_utf8(opened.bytes) {
                Ok(raw) => raw,
                Err(_) => {
                    return build_native_telegram_cursor_status(NativeTelegramCursorStatusInput {
                        requested: true,
                        cursor_path: logical_path,
                        cursor_file_present: true,
                        cursor_updated_at_unix_ms: opened.modified_unix_ms,
                        raw_json: None,
                        read_error: Some(
                            "failed to read Telegram cursor file: cursor is not UTF-8",
                        ),
                    });
                }
            };
            build_native_telegram_cursor_status(NativeTelegramCursorStatusInput {
                requested: true,
                cursor_path: logical_path,
                cursor_file_present: true,
                cursor_updated_at_unix_ms: opened.modified_unix_ms,
                raw_json: Some(&raw),
                read_error: None,
            })
        }
        Err(error) => {
            let error = format!("failed to read Telegram cursor file securely: {error:#}");
            build_native_telegram_cursor_status(NativeTelegramCursorStatusInput {
                requested: true,
                cursor_path: logical_path,
                cursor_file_present: true,
                cursor_updated_at_unix_ms: None,
                raw_json: None,
                read_error: Some(&error),
            })
        }
    }
}

pub(crate) fn write_cursor_next_update_offset(path: &Path, offset: i64) -> Result<()> {
    let body = native_telegram_cursor_body(offset, now_unix_ms()).map_err(anyhow::Error::msg)?;
    let mut bytes = serde_json::to_vec_pretty(&body).context("encode Telegram cursor JSON")?;
    bytes.push(b'\n');
    private_store(path, MAX_CURSOR_BYTES, "hepta-telegram-cursor")?
        .update(|_| Ok((bytes, ())))
        .with_context(|| format!("publish secure Telegram cursor {}", path.display()))
}

pub(crate) fn update_private_state_atomically<T>(
    path: &Path,
    max_bytes: u64,
    temporary_prefix: &str,
    update: impl FnOnce(Option<&[u8]>) -> Result<(Vec<u8>, T)>,
) -> Result<T> {
    private_store(path, max_bytes, temporary_prefix)?.update(update)
}

pub(crate) fn read_private_state(path: &Path, max_bytes: u64) -> Result<Option<Vec<u8>>> {
    private_store(path, max_bytes, "hepta-private-state")?.read()
}

pub(crate) fn append_delivery_lifecycle_record(path: &Path, record: &Value) -> Result<()> {
    let mut bytes =
        serde_json::to_vec(record).context("render secure Telegram delivery ledger record")?;
    bytes.push(b'\n');
    private_store(path, MAX_DELIVERY_LEDGER_BYTES, "hepta-telegram-delivery")?
        .append(&bytes)
        .with_context(|| format!("append secure Telegram delivery ledger {}", path.display()))
}

fn private_store(
    path: &Path,
    max_bytes: u64,
    staging_prefix: &str,
) -> Result<AuthenticatedJournalStore> {
    AuthenticatedJournalStore::new(path, max_bytes, staging_prefix)
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

#[cfg(all(test, unix))]
#[path = "../tests/unit/telegram_durable_files.rs"]
mod tests;
