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
