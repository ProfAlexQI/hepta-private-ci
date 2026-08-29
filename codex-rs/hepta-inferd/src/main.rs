use std::env;
use std::io;
use std::io::ErrorKind;
use std::path::PathBuf;

use codex_hepta_infer_core::Digest;
use codex_hepta_inferd::DaemonConfig;

#[tokio::main]
async fn main() -> io::Result<()> {
    let socket_path = required_path("HEPTA_INFER_SOCKET")?;
    let receipt_dir = required_path("HEPTA_INFER_RECEIPT_DIR")?;
    let tuple = env::var("HEPTA_INFER_MODEL_TUPLE_DIGEST")
        .map_err(|_| io::Error::new(ErrorKind::InvalidInput, "INF_MODEL_TUPLE_REQUIRED"))?;
    let tuple =
        Digest::parse(&tuple).map_err(|error| io::Error::new(ErrorKind::InvalidInput, error))?;
    codex_hepta_inferd::serve_forever(DaemonConfig::qualification_only(
        socket_path,
        receipt_dir,
        tuple,
    ))
    .await
}

fn required_path(name: &str) -> io::Result<PathBuf> {
    let value = env::var_os(name)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| io::Error::new(ErrorKind::InvalidInput, format!("{name}_REQUIRED")))?;
    let path = PathBuf::from(value);
    if path.is_absolute() {
        Ok(path)
    } else {
        Err(io::Error::new(
            ErrorKind::InvalidInput,
            format!("{name}_MUST_BE_ABSOLUTE"),
        ))
    }
}
