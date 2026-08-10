use pretty_assertions::assert_eq;

use super::request::Surface;
use super::request::canonical_json;
use super::request::parse_request;
use crate::QualificationError;

const PROMPT: &str = "Run the controlled qualification command exactly once and report completion.";

#[test]
fn parses_the_frozen_app_server_pair() -> Result<(), QualificationError> {
    for ordinal in 1..=2 {
        let request = serde_json::json!({
            "id": ordinal + 2,
            "method": "turn/start",
            "params": {
                "input": [{"text": PROMPT, "textElements": [], "type": "text"}],
                "threadId": format!("thread-{ordinal}"),
            },
        });
        let mut bytes = canonical_json(&request)?;
        bytes.push(b'\n');
        let parsed = parse_request(Surface::AppServer, ordinal, &bytes, "/unused")?;
        assert_eq!(parsed.body_sha256.len(), 64);
        assert_eq!(parsed.provider_semantic_sha256.len(), 64);
        assert_eq!(parsed.sample_token_sha256.len(), 64);
    }
    Ok(())
}

#[test]
fn parses_the_frozen_mcp_pair() -> Result<(), QualificationError> {
    let cwd = "/private/tmp/hepta-shadow";
    let requests = [
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
        }),
        serde_json::json!({
            "id": 3,
            "jsonrpc": "2.0",
            "method": "tools/call",
            "params": {
                "arguments": {"prompt": PROMPT, "threadId": "thread-mcp"},
                "name": "codex-reply",
            },
        }),
    ];
    for (index, request) in requests.into_iter().enumerate() {
        let mut bytes = canonical_json(&request)?;
        bytes.push(b'\n');
        let parsed = parse_request(Surface::Mcp, index as u8 + 1, &bytes, cwd)?;
        assert_eq!(parsed.body_sha256.len(), 64);
    }
    Ok(())
}

#[test]
fn rejects_noncanonical_or_multiline_requests() -> Result<(), QualificationError> {
    let noncanonical = format!(
        "{{\"method\":\"turn/start\",\"id\":3,\"params\":{{\"threadId\":\"thread\",\"input\":[{{\"type\":\"text\",\"text\":{PROMPT:?},\"textElements\":[]}}]}}}}\n"
    );
    assert!(parse_request(Surface::AppServer, 1, noncanonical.as_bytes(), "/unused").is_err());

    let multiline = noncanonical.replace("\"id\":3", "\n\"id\":3");
    assert!(parse_request(Surface::AppServer, 1, multiline.as_bytes(), "/unused").is_err());
    Ok(())
}
