use std::fs;
use std::path::Path;
use std::path::PathBuf;

use crate::CompletedPreSend;
use crate::DurablePreSendObserver;
use crate::QualificationError;
use crate::request::FIXED_PROMPT;
use crate::request::app_server_sample_request;
use crate::request::mcp_sample_request;

pub(crate) const PROMPT: &str = FIXED_PROMPT;

pub(crate) fn dynamic_receipt(
    oracle: &crate::FrozenOracle,
) -> Result<Vec<u8>, QualificationError> {
    let mut value: serde_json::Value = serde_json::from_slice(oracle.expected_normalized_receipt())
        .map_err(|error| QualificationError::Serialization(error.to_string()))?;
    let action_id = "tool:v1:8f6bcbfaa4fa03f850776b5ec940744f96bf50b6d78b17a04b436e196ea300d8";
    value["action_id"] = serde_json::Value::String(action_id.to_string());
    value["receipt_id"] = serde_json::Value::String(
        "receipt:v1:1ece195bbd3ee5519e25e28f123001fa786e44bed46dbea7144db6fe93f7b8b5".to_string(),
    );
    for (record, decision_id) in [
        (
            "admission",
            "decision:v1:657ce242e8205905bdc85678268fe3cb6ebaa2abdca12d6f588b87284e33e543",
        ),
        (
            "authorization",
            "decision:v1:bc77cf37cb529e1149ef433d1ab174ed4d096c9ca7bffaeeda5412a2e34243ae",
        ),
    ] {
        value[record]["decision_id"] = serde_json::Value::String(decision_id.to_string());
        value[record]["action"]["action_id"] = serde_json::Value::String(action_id.to_string());
        value[record]["action"]["thread_id"] =
            serde_json::Value::String("thread-live-1".to_string());
        value[record]["action"]["turn_id"] = serde_json::Value::String("turn-live-1".to_string());
        value[record]["action"]["call_id"] = serde_json::Value::String("call-live-1".to_string());
    }
    crate::request::canonical_json(&value)
}

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
    app_server_sample_request(ordinal, &format!("thread-{ordinal}"))
}

pub(crate) fn mcp_request(ordinal: u8, cwd: &str) -> Result<Vec<u8>, QualificationError> {
    let thread_id = (ordinal == 2).then_some("thread-mcp");
    mcp_sample_request(ordinal, cwd, thread_id)
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
