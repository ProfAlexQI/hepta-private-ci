use std::fs;
use std::io::Write;

use serde_json::Value;

use super::importer::ImportCheckpoint;
use super::observer::DurablePreSendObserver;
use super::request::canonical_json;
use crate::QualificationError;

const PROMPT: &str = "Run the controlled qualification command exactly once and report completion.";

#[test]
fn imports_exactly_four_artifacts_and_writes_a_checkpoint() -> Result<(), QualificationError> {
    let (completed, _temp) = completed_run()?;
    let checkpoint = ImportCheckpoint::create(&completed)?;
    assert!(checkpoint.is_complete());
    assert_eq!(checkpoint.verified_count(), 4);
    assert!(checkpoint.failures().is_empty());
    assert_eq!(checkpoint.run_id(), completed.run_id());
    assert_eq!(checkpoint.evidence_set_sha256().len(), 64);
    assert!(
        checkpoint
            .run_root()
            .join("import-checkpoint.json")
            .is_file()
    );
    Ok(())
}

#[test]
fn inventories_all_broken_samples_before_checkpointing_failure() -> Result<(), QualificationError> {
    let (completed, _temp) = completed_run()?;
    fs::remove_file(completed.run_root().join("app_server-01.raw.json"))?;
    fs::write(completed.run_root().join("mcp-02.pre-send.json"), b"{}")?;
    let mut unexpected = fs::File::create(completed.run_root().join("unexpected.txt"))?;
    unexpected.write_all(b"unexpected")?;
    unexpected.sync_all()?;
    let checkpoint = ImportCheckpoint::create(&completed)?;
    assert!(!checkpoint.is_complete());
    assert_eq!(checkpoint.verified_count(), 2);
    assert_eq!(checkpoint.failures().len(), 3);
    for expected in ["app_server-01", "mcp-02", "unexpected.txt"] {
        assert!(
            checkpoint
                .failures()
                .iter()
                .any(|failure| failure.artifact == expected)
        );
    }
    Ok(())
}

fn completed_run() -> Result<(crate::CompletedPreSend, tempfile::TempDir), QualificationError> {
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

fn app_request(ordinal: u8) -> Result<Vec<u8>, QualificationError> {
    line(&serde_json::json!({
        "id": ordinal + 2,
        "method": "turn/start",
        "params": {
            "input": [{"text": PROMPT, "textElements": [], "type": "text"}],
            "threadId": format!("thread-{ordinal}"),
        },
    }))
}

fn mcp_request(ordinal: u8, cwd: &str) -> Result<Vec<u8>, QualificationError> {
    let request = if ordinal == 1 {
        serde_json::json!({
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
        })
    } else {
        serde_json::json!({
            "id": 3,
            "jsonrpc": "2.0",
            "method": "tools/call",
            "params": {
                "arguments": {"prompt": PROMPT, "threadId": "thread-mcp"},
                "name": "codex-reply",
            },
        })
    };
    line(&request)
}

fn line(value: &Value) -> Result<Vec<u8>, QualificationError> {
    let mut bytes = canonical_json(value)?;
    bytes.push(b'\n');
    Ok(bytes)
}
