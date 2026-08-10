use std::fs;

use serde_json::Value;

use super::observer::DurablePreSendObserver;
use super::request::Surface;
use super::request::canonical_json;
use crate::QualificationError;

const PROMPT: &str = "Run the controlled qualification command exactly once and report completion.";

#[test]
fn returns_tokens_only_after_four_private_durable_pairs() -> Result<(), QualificationError> {
    let temp = tempfile::tempdir()?;
    let root = temp.path().join("observer");
    let cwd = temp.path().join("work");
    fs::create_dir(&cwd)?;
    let mut observer = DurablePreSendObserver::create(&root, &cwd)?;
    let mut tokens = Vec::new();
    for ordinal in 1..=2 {
        tokens.push(observer.record_app_server(&app_request(ordinal)?)?);
    }
    for ordinal in 1..=2 {
        tokens.push(observer.record_mcp(&mcp_request(ordinal, &cwd.to_string_lossy())?)?);
    }
    assert_eq!(tokens.len(), 4);
    assert!(tokens.iter().all(|token| token.token_sha256().len() == 64));
    assert_eq!(tokens[0].surface(), Surface::AppServer);
    assert_eq!(tokens[3].surface(), Surface::Mcp);
    assert_eq!(tokens[3].ordinal(), 2);
    assert_eq!(tokens[0].run_id(), tokens[3].run_id());
    let completed = observer.finish()?;
    assert_eq!(completed.token_count(), 4);
    assert_eq!(completed.run_id(), tokens[0].run_id());

    let mut names = fs::read_dir(completed.run_root())?
        .map(|entry| entry.map(|entry| entry.file_name()))
        .collect::<Result<Vec<_>, _>>()?;
    names.sort();
    assert_eq!(names.len(), 9);
    for surface in ["app_server", "mcp"] {
        for ordinal in 1..=2 {
            let receipt = completed
                .run_root()
                .join(format!("{surface}-{ordinal:02}.pre-send.json"));
            let value: Value = serde_json::from_slice(&fs::read(receipt)?)
                .map_err(|error| QualificationError::Serialization(error.to_string()))?;
            assert_eq!(value["authority"], false);
            assert_eq!(value["enforce"], false);
            assert_eq!(value["promotion"], false);
            assert_eq!(value["product_http_pre_send_claimed"], false);
        }
    }
    assert_private_tree(completed.run_root())?;
    Ok(())
}

#[test]
fn rejects_out_of_order_or_incomplete_runs_without_minting_tokens() -> Result<(), QualificationError>
{
    let temp = tempfile::tempdir()?;
    let root = temp.path().join("observer");
    let cwd = temp.path().join("work");
    fs::create_dir(&cwd)?;
    let mut observer = DurablePreSendObserver::create(&root, &cwd)?;
    assert!(
        observer
            .record_mcp(&mcp_request(1, &cwd.to_string_lossy())?)
            .is_err()
    );
    assert!(observer.finish().is_err());
    let run_root = only_run_root(&root)?;
    assert_eq!(fs::read_dir(run_root)?.count(), 1);
    Ok(())
}

#[test]
fn rejects_noncanonical_input_before_writing_an_artifact() -> Result<(), QualificationError> {
    let temp = tempfile::tempdir()?;
    let root = temp.path().join("observer");
    let cwd = temp.path().join("work");
    fs::create_dir(&cwd)?;
    let mut observer = DurablePreSendObserver::create(&root, &cwd)?;
    let mut request = app_request(1)?;
    request.insert(0, b' ');
    assert!(observer.record_app_server(&request).is_err());
    let run_root = only_run_root(&root)?;
    assert_eq!(fs::read_dir(run_root)?.count(), 1);
    Ok(())
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

fn line(value: &Value) -> Result<Vec<u8>, QualificationError> {
    let mut bytes = canonical_json(value)?;
    bytes.push(b'\n');
    Ok(bytes)
}

fn only_run_root(root: &std::path::Path) -> Result<std::path::PathBuf, QualificationError> {
    let mut entries = fs::read_dir(root)?;
    let entry = entries
        .next()
        .ok_or_else(|| QualificationError::State("missing run root".to_string()))??;
    if entries.next().is_some() {
        return Err(QualificationError::State("multiple run roots".to_string()));
    }
    Ok(entry.path())
}

fn assert_private_tree(root: &std::path::Path) -> Result<(), QualificationError> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        assert_eq!(fs::metadata(root)?.permissions().mode() & 0o077, 0);
        for entry in fs::read_dir(root)? {
            assert_eq!(entry?.metadata()?.permissions().mode() & 0o077, 0);
        }
    }
    Ok(())
}
