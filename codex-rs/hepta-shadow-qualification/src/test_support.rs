use std::fs;
use std::path::Path;
use std::path::PathBuf;

use serde_json::Value;

use crate::CompletedPreSend;
use crate::DurablePreSendObserver;
use crate::QualificationError;
use crate::request::canonical_json;

pub(crate) const PROMPT: &str =
    "Run the controlled qualification command exactly once and report completion.";

pub(crate) fn completed_run() -> Result<(CompletedPreSend, tempfile::TempDir), QualificationError> {
    let temp = tempfile::tempdir()?;
    let root = temp.path().join("observer");
    let cwd = temp.path().join("work");
    fs::create_dir(&cwd)?;
    let mut observer = DurablePreSendObserver::create(&root, &cwd)?;
    for ordinal in 1..=2 {
        observer.record_app_server(&app_request(ordinal)?)?;
    }
    for ordinal in 1..=2 {
        observer.record_mcp(&mcp_request(ordinal, &cwd.to_string_lossy())?)?;
    }
    Ok((observer.finish()?, temp))
}

pub(crate) fn app_request(ordinal: u8) -> Result<Vec<u8>, QualificationError> {
    line(&serde_json::json!({
        "id": ordinal + 2,
        "method": "turn/start",
        "params": {
            "input": [{"text": PROMPT, "textElements": [], "type": "text"}],
            "threadId": format!("thread-{ordinal}"),
        },
    }))
}

pub(crate) fn mcp_request(ordinal: u8, cwd: &str) -> Result<Vec<u8>, QualificationError> {
    let request = match ordinal {
        1 => serde_json::json!({
            "id": 2,
            "jsonrpc": "2.0",
            "method": "tools/call",
            "params": {
                "arguments": {
                    "approval-policy": "never",
                    "base-instructions": "Execute only the exact requested controlled qualification command. Do not invoke any other tool or network service.",
                    "cwd": cwd,
                    "developer-instructions": "This is a controlled short trial, not a duration soak and not promotion authority.",
                    "model": "hepta-shadow-qualification",
                    "prompt": PROMPT,
                    "sandbox": "workspace-write",
                },
                "name": "codex",
            },
        }),
        2 => serde_json::json!({
            "id": 3,
            "jsonrpc": "2.0",
            "method": "tools/call",
            "params": {
                "arguments": {"prompt": PROMPT, "threadId": "thread-mcp"},
                "name": "codex-reply",
            },
        }),
        _ => {
            return Err(QualificationError::Invalid(
                "invalid MCP ordinal".to_string(),
            ));
        }
    };
    line(&request)
}

pub(crate) fn only_run_root(root: &Path) -> Result<PathBuf, QualificationError> {
    let mut entries = fs::read_dir(root)?;
    let entry = entries
        .next()
        .ok_or_else(|| QualificationError::State("missing run root".to_string()))??;
    if entries.next().is_some() {
        return Err(QualificationError::State("multiple run roots".to_string()));
    }
    Ok(entry.path())
}

fn line(value: &Value) -> Result<Vec<u8>, QualificationError> {
    let mut bytes = canonical_json(value)?;
    bytes.push(b'\n');
    Ok(bytes)
}
