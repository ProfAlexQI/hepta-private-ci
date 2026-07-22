use crate::HEPTA_KERNEL_TELEGRAM_CURSOR_SCHEMA;
use crate::HEPTA_KERNEL_TELEGRAM_INGRESS_CURSOR_PATH;
use crate::redact_hepta_kernel_telegram_token_like_text;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use serde_json::json;

pub fn hepta_kernel_telegram_update_already_drained(
    update_id: i64,
    next_update_offset: Option<i64>,
) -> bool {
    next_update_offset
        .map(|cursor| update_id < cursor)
        .unwrap_or(false)
}

pub fn hepta_kernel_telegram_cursor_duplicate_rule_valid() -> bool {
    hepta_kernel_telegram_update_already_drained(41, Some(42))
        && !hepta_kernel_telegram_update_already_drained(42, Some(42))
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaKernelTelegramCursorPlan {
    pub cursor_path: &'static str,
    pub duplicate_suppression_ready: bool,
    pub duplicate_suppression_rule_valid: bool,
    pub cursor_represents_next_update_offset: bool,
    pub commit_offset_after_delivery: bool,
    pub raw_update_payload_persisted: bool,
}

impl HeptaKernelTelegramCursorPlan {
    pub fn disabled() -> Self {
        Self {
            cursor_path: HEPTA_KERNEL_TELEGRAM_INGRESS_CURSOR_PATH,
            duplicate_suppression_ready: false,
            duplicate_suppression_rule_valid: true,
            cursor_represents_next_update_offset: true,
            commit_offset_after_delivery: false,
            raw_update_payload_persisted: false,
        }
    }

    pub fn ready() -> Self {
        Self {
            cursor_path: HEPTA_KERNEL_TELEGRAM_INGRESS_CURSOR_PATH,
            duplicate_suppression_ready: true,
            duplicate_suppression_rule_valid: hepta_kernel_telegram_cursor_duplicate_rule_valid(),
            cursor_represents_next_update_offset: true,
            commit_offset_after_delivery: true,
            raw_update_payload_persisted: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaKernelTelegramCursorStatus {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HeptaKernelTelegramCursorStatusInput<'a> {
    pub requested: bool,
    pub cursor_path: &'static str,
    pub cursor_file_present: bool,
    pub cursor_updated_at_unix_ms: Option<u64>,
    pub raw_json: Option<&'a str>,
    pub read_error: Option<&'a str>,
}

pub fn build_hepta_kernel_telegram_cursor_status(
    input: HeptaKernelTelegramCursorStatusInput<'_>,
) -> HeptaKernelTelegramCursorStatus {
    if !input.requested {
        return HeptaKernelTelegramCursorStatus {
            product: "Hepta",
            runtime: "hepta",
            requested: false,
            status: "disabled",
            cursor_path: input.cursor_path,
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

    let mut status = HeptaKernelTelegramCursorStatus {
        product: "Hepta",
        runtime: "hepta",
        requested: true,
        status: "missing",
        cursor_path: input.cursor_path,
        cursor_file_present: input.cursor_file_present,
        cursor_parse_ok: false,
        next_update_offset: None,
        cursor_updated_at_unix_ms: input.cursor_updated_at_unix_ms,
        last_delivered_next_update_offset: None,
        durable_cursor_evidence_present: false,
        cursor_represents_next_update_offset: true,
        duplicate_suppression_rule_valid: hepta_kernel_telegram_cursor_duplicate_rule_valid(),
        cursor_write_policy: "write only after model output is delivered or duplicate suppression is recorded",
        cursor_written: false,
        raw_update_payload_persisted: false,
        error: None,
        next_migration_slice: "wire cursor write after gated send delivery success",
    };

    if !input.cursor_file_present {
        return status;
    }

    if let Some(error) = input.read_error {
        status.status = "attention";
        status.error = Some(redact_hepta_kernel_telegram_token_like_text(error));
        return status;
    }

    let Some(raw) = input.raw_json else {
        status.status = "attention";
        status.error =
            Some("Telegram cursor file was present but no JSON was provided".to_string());
        return status;
    };

    match parse_hepta_kernel_telegram_cursor_next_update_offset(raw) {
        Ok(next_update_offset) => {
            let cursor_json = serde_json::from_str::<Value>(raw).unwrap_or(Value::Null);
            let raw_update_payload_persisted = cursor_json
                .get("raw_update_payload_persisted")
                .and_then(Value::as_bool)
                .unwrap_or(false);
            let cursor_updated_at_unix_ms = cursor_json
                .get("updated_at_unix_ms")
                .and_then(Value::as_u64)
                .or(input.cursor_updated_at_unix_ms);
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
            status.error = Some(redact_hepta_kernel_telegram_token_like_text(&error));
        }
    }

    status
}

pub fn parse_hepta_kernel_telegram_cursor_next_update_offset(raw: &str) -> Result<i64, String> {
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

pub fn hepta_kernel_telegram_cursor_body(
    offset: i64,
    updated_at_unix_ms: u64,
) -> Result<Value, String> {
    if offset < 0 {
        return Err("Telegram cursor next_update_offset must be non-negative".to_string());
    }
    Ok(json!({
        "schema": HEPTA_KERNEL_TELEGRAM_CURSOR_SCHEMA,
        "next_update_offset": offset,
        "updated_at_unix_ms": updated_at_unix_ms,
        "last_delivered_next_update_offset": offset,
        "raw_update_payload_persisted": false,
    }))
}

#[cfg(test)]
#[path = "telegram_cursor_tests.rs"]
mod tests;
