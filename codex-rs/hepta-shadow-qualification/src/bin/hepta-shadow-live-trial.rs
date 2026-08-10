use std::path::PathBuf;
use std::time::Duration;

use codex_hepta_shadow_qualification::QualificationTrial;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut arguments = std::env::args_os();
    let program = arguments.next().unwrap_or_default();
    let product = required_argument(&mut arguments, &program, "FROZEN_PRODUCT")?;
    let runtime_root = required_argument(&mut arguments, &program, "ABSENT_RUNTIME_ROOT")?;
    if arguments.next().is_some() {
        return Err(argument_error(usage(&program)).into());
    }
    let outcome = QualificationTrial::run(product, &runtime_root, Duration::from_secs(120)).await?;
    let document = serde_json::json!({
        "app_server": {
            "exit_code": outcome.app_server_child().exit_code(),
            "http_exchange_count": outcome.app_server_http().len(),
            "inbound_message_count": outcome.app_server_child().inbound_message_count(),
            "stderr_sha256": outcome.app_server_child().stderr_sha256(),
            "stderr_size_bytes": outcome.app_server_child().stderr_size_bytes(),
            "stderr_truncated": outcome.app_server_child().stderr_truncated(),
            "thread_id": outcome.app_server_thread_id(),
            "turn_ids": outcome.app_server_turn_ids(),
        },
        "authority": false,
        "enforce": false,
        "mcp": {
            "exit_code": outcome.mcp_child().exit_code(),
            "http_exchange_count": outcome.mcp_http().len(),
            "inbound_message_count": outcome.mcp_child().inbound_message_count(),
            "stderr_sha256": outcome.mcp_child().stderr_sha256(),
            "stderr_size_bytes": outcome.mcp_child().stderr_size_bytes(),
            "stderr_truncated": outcome.mcp_child().stderr_truncated(),
            "thread_id": outcome.mcp_thread_id(),
        },
        "outbound": false,
        "promotion": false,
        "run_id": outcome.completed().run_id(),
        "run_root": outcome.completed().run_root(),
        "runtime_root": runtime_root,
    });
    println!("{}", serde_json::to_string(&document)?);
    Ok(())
}

fn required_argument(
    arguments: &mut impl Iterator<Item = std::ffi::OsString>,
    program: &std::ffi::OsStr,
    name: &str,
) -> Result<PathBuf, std::io::Error> {
    arguments
        .next()
        .map(PathBuf::from)
        .ok_or_else(|| argument_error(format!("missing {name}; {}", usage(program))))
}

fn argument_error(message: impl Into<String>) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::InvalidInput, message.into())
}

fn usage(program: &std::ffi::OsStr) -> String {
    format!(
        "usage: {} FROZEN_PRODUCT ABSENT_RUNTIME_ROOT",
        PathBuf::from(program).display()
    )
}
