use std::collections::HashMap;

use codex_protocol::protocol::HookEventName;
use codex_protocol::protocol::HookSource;
use codex_utils_absolute_path::AbsolutePathBuf;
use tempfile::tempdir;

use super::CommandShell;
use super::ConfiguredHandler;
use super::run_command;

#[tokio::test]
async fn fast_exiting_hook_preserves_stdout_when_stdin_is_not_consumed() {
    let temp = tempdir().expect("create temp dir");
    let source_path = AbsolutePathBuf::try_from(temp.path().join("hooks.json"))
        .expect("absolute hook configuration path");
    let handler = ConfiguredHandler {
        event_name: HookEventName::SessionStart,
        matcher: None,
        command: "echo hook-ran".to_string(),
        timeout_sec: 10,
        status_message: None,
        source_path,
        source: HookSource::User,
        display_order: 0,
        env: HashMap::new(),
    };
    let shell = CommandShell {
        program: String::new(),
        args: Vec::new(),
    };
    let input_json = format!(r#"{{"padding":"{}"}}"#, "x".repeat(1024 * 1024));

    let result = run_command(&shell, &handler, &input_json, temp.path()).await;

    assert_eq!(result.exit_code, Some(0), "stderr: {}", result.stderr);
    assert_eq!(result.stdout.trim(), "hook-ran");
    assert_eq!(result.error, None);
}
