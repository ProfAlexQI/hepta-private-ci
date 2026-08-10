use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use crate::QualificationError;

pub(crate) fn write_private_new(path: &Path, bytes: &[u8]) -> Result<(), QualificationError> {
    let mut options = OpenOptions::new();
    options.create_new(true).write(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options
            .mode(0o600)
            .custom_flags(libc::O_CLOEXEC | libc::O_NOFOLLOW);
    }
    let mut file = options.open(path)?;
    file.write_all(bytes)?;
    file.sync_all()?;
    verify_private_regular(path)?;
    sync_directory(
        path.parent()
            .ok_or_else(|| invalid("artifact has no parent"))?,
    )
}

pub(crate) fn create_or_verify_private_directory(path: &Path) -> Result<(), QualificationError> {
    if !path.exists() {
        create_private_directory(path)?;
    }
    verify_private_directory(path)
}

pub(crate) fn create_private_directory(path: &Path) -> Result<(), QualificationError> {
    let mut builder = std::fs::DirBuilder::new();
    #[cfg(unix)]
    {
        use std::os::unix::fs::DirBuilderExt;
        builder.mode(0o700);
    }
    builder.create(path)?;
    verify_private_directory(path)
}

pub(crate) fn verify_private_directory(path: &Path) -> Result<(), QualificationError> {
    let metadata = std::fs::symlink_metadata(path)?;
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err(invalid("private path must be a real directory"));
    }
    verify_private_mode(&metadata, "private directory")
}

pub(crate) fn verify_private_regular(path: &Path) -> Result<(), QualificationError> {
    let metadata = std::fs::symlink_metadata(path)?;
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err(invalid("private artifact must be a real regular file"));
    }
    verify_private_mode(&metadata, "private artifact")
}

pub(crate) fn sync_directory(path: &Path) -> Result<(), QualificationError> {
    File::open(path)?.sync_all()?;
    Ok(())
}

pub(crate) fn now_millis() -> Result<u128, QualificationError> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .map_err(|error| {
            QualificationError::State(format!("system clock is before UNIX epoch: {error}"))
        })
}

fn verify_private_mode(
    metadata: &std::fs::Metadata,
    label: &str,
) -> Result<(), QualificationError> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if metadata.permissions().mode() & 0o077 != 0 {
            return Err(invalid(format!(
                "{label} must not grant group or other access"
            )));
        }
    }
    Ok(())
}

fn invalid(message: impl Into<String>) -> QualificationError {
    QualificationError::Invalid(message.into())
}
