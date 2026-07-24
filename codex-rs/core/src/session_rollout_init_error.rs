use std::io::ErrorKind;
use std::path::Path;

use crate::rollout::SESSIONS_SUBDIR;
use codex_protocol::error::CodexErr;
use codex_thread_store::ThreadStoreError;

pub(crate) fn map_session_init_error(err: &anyhow::Error, codex_home: &Path) -> CodexErr {
    if let Some(ThreadStoreError::Conflict { message }) = err
        .chain()
        .find_map(|cause| cause.downcast_ref::<ThreadStoreError>())
    {
        return CodexErr::InvalidRequest(message.clone());
    }

    if let Some(mapped) = err
        .chain()
        .filter_map(|cause| cause.downcast_ref::<std::io::Error>())
        .find_map(|io_err| map_rollout_io_error(io_err, codex_home))
    {
        return mapped;
    }

    CodexErr::Fatal(format!("Failed to initialize session: {err:#}"))
}

fn map_rollout_io_error(io_err: &std::io::Error, codex_home: &Path) -> Option<CodexErr> {
    let sessions_dir = codex_home.join(SESSIONS_SUBDIR);
    let hint = match io_err.kind() {
        ErrorKind::PermissionDenied => format!(
            "Hepta cannot access session files at {} (permission denied). If sessions were created using sudo, fix ownership: sudo chown -R $(whoami) {}",
            sessions_dir.display(),
            codex_home.display()
        ),
        ErrorKind::NotFound => format!(
            "Session storage missing at {}. Create the directory or choose a different Hepta home.",
            sessions_dir.display()
        ),
        ErrorKind::AlreadyExists => format!(
            "Session storage path {} is blocked by an existing file. Remove or rename it so Hepta can create sessions.",
            sessions_dir.display()
        ),
        ErrorKind::InvalidData | ErrorKind::InvalidInput => format!(
            "Session data under {} looks corrupt or unreadable. Clearing the sessions directory may help (this will remove saved threads).",
            sessions_dir.display()
        ),
        ErrorKind::IsADirectory | ErrorKind::NotADirectory => format!(
            "Session storage path {} has an unexpected type. Ensure it is a directory Hepta can use for session files.",
            sessions_dir.display()
        ),
        _ => return None,
    };

    Some(CodexErr::Fatal(format!(
        "{hint} (underlying error: {io_err})"
    )))
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use codex_protocol::error::CodexErr;
    use codex_thread_store::ThreadStoreError;

    use super::map_session_init_error;

    #[test]
    fn thread_store_conflict_maps_to_invalid_request_through_context() {
        let err = anyhow::Error::new(ThreadStoreError::Conflict {
            message: "thread already has an active writer".to_string(),
        })
        .context("session persistence initialization failed");

        assert!(matches!(
            map_session_init_error(&err, Path::new("/tmp/hepta")),
            CodexErr::InvalidRequest(message)
                if message == "thread already has an active writer"
        ));
    }

    #[test]
    fn non_conflict_thread_store_error_remains_fatal() {
        let err = anyhow::Error::new(ThreadStoreError::Internal {
            message: "storage failed".to_string(),
        });

        assert!(matches!(
            map_session_init_error(&err, Path::new("/tmp/hepta")),
            CodexErr::Fatal(message)
                if message == "Failed to initialize session: thread-store internal error: storage failed"
        ));
    }
}
