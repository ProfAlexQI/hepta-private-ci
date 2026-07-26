use hepta_core::ToolResult;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ProviderErrorAfterCommit {
    pub(super) error: String,
    pub(super) error_code: String,
}

pub(super) fn timeout_ms(expected_tool: &str, result: &ToolResult) -> Option<u64> {
    let structured_timeout = result
        .structured_json
        .as_deref()
        .and_then(|json| serde_json::from_str::<Value>(json).ok())
        .and_then(|value| {
            value.get("timeout_ms").and_then(Value::as_u64).or_else(|| {
                value
                    .get("result")
                    .and_then(|result| result.get("timeout_ms"))
                    .and_then(Value::as_u64)
            })
        });
    match structured_timeout {
        Some(0) => None,
        Some(timeout_ms) => Some(timeout_ms),
        None => {
            reserved_timeout_ms(expected_tool, &result.content).filter(|timeout_ms| *timeout_ms > 0)
        }
    }
}

fn reserved_timeout_ms(expected_tool: &str, content: &str) -> Option<u64> {
    let signature = content.strip_prefix("ToolTimeout/")?;
    let (tool, duration) = signature.split_once(" timed out after ")?;
    if tool != expected_tool || tool.chars().any(char::is_whitespace) {
        return None;
    }
    duration.strip_suffix(" ms")?.parse::<u64>().ok()
}

pub(super) fn tool_reported_failure(result: &ToolResult) -> Option<String> {
    let value = serde_json::from_str::<Value>(result.structured_json.as_deref()?).ok()?;
    let status = value.get("status").and_then(Value::as_str);
    let status_failed = matches!(status, Some("error" | "failed" | "failure"));
    let explicit_error = value.get("error").filter(|value| match value {
        Value::Null => false,
        Value::Bool(value) => *value,
        Value::String(value) => !value.trim().is_empty(),
        _ => true,
    });
    if !status_failed && explicit_error.is_none() {
        return None;
    }
    explicit_error
        .map(Value::to_string)
        .or_else(|| status.map(str::to_string))
        .or_else(|| Some(result.content.clone()))
}

pub(super) fn provider_error_after_commit(result: &ToolResult) -> Option<ProviderErrorAfterCommit> {
    let value = serde_json::from_str::<Value>(result.structured_json.as_deref()?).ok()?;
    if value
        .get("provider_error_after_commit")
        .and_then(Value::as_bool)
        != Some(true)
    {
        return None;
    }
    let error = value
        .get("error")
        .and_then(Value::as_str)
        .filter(|error| !error.trim().is_empty())
        .map(str::to_owned)?;
    let error_code = error
        .split_once(':')
        .map(|(code, _)| code)
        .filter(|code| {
            !code.is_empty()
                && code
                    .bytes()
                    .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
        })?
        .to_owned();
    Some(ProviderErrorAfterCommit { error, error_code })
}
