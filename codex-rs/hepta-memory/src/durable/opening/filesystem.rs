//! Unix filesystem invariants for durable SQLite database artifacts.

use std::fs::File;
#[cfg(unix)]
use std::fs::OpenOptions;
use std::path::Path;
use std::path::PathBuf;

use same_file::Handle;

use super::super::DurableStorageError;

#[cfg(unix)]
use std::os::unix::fs::DirBuilderExt;
#[cfg(unix)]
use std::os::unix::fs::MetadataExt;
#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

#[cfg(unix)]
const PRIVATE_FILE_MODE: u32 = 0o600;
#[cfg(unix)]
const PRIVATE_DIRECTORY_MODE: u32 = 0o700;
const MAX_SIDECAR_VALIDATION_ATTEMPTS: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SidecarValidationProgress {
    Complete,
    Missing,
    Retry,
}

#[cfg(unix)]
pub(super) fn prepare_bootstrap_parent(path: &Path) -> Result<(), DurableStorageError> {
    let parent = database_parent(path);
    match std::fs::symlink_metadata(parent) {
        Ok(_) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            let mut builder = std::fs::DirBuilder::new();
            builder.recursive(true).mode(PRIVATE_DIRECTORY_MODE);
            builder.create(parent).map_err(|error| {
                DurableStorageError::persistence("create private database parent", error)
            })?;
        }
        Err(error) => {
            return Err(DurableStorageError::persistence(
                "inspect database parent for bootstrap",
                error,
            ));
        }
    }
    validate_database_parent(path)
}

#[cfg(not(unix))]
pub(super) fn prepare_bootstrap_parent(_path: &Path) -> Result<(), DurableStorageError> {
    Err(unsupported_platform())
}

#[cfg(unix)]
pub(super) fn validate_database_parent(path: &Path) -> Result<(), DurableStorageError> {
    let parent = database_parent(path);
    let metadata = std::fs::symlink_metadata(parent).map_err(|error| {
        DurableStorageError::persistence("inspect durable database parent", error)
    })?;
    if !metadata.file_type().is_dir() {
        return Err(DurableStorageError::corrupt(format!(
            "durable database parent is not a non-symlink directory: {}",
            parent.display()
        )));
    }
    let expected_uid = effective_uid();
    if metadata.uid() != expected_uid {
        return Err(DurableStorageError::corrupt(format!(
            "durable database parent {} is owned by uid {}, expected effective uid {expected_uid}",
            parent.display(),
            metadata.uid()
        )));
    }
    if metadata.nlink() == 0 {
        return Err(DurableStorageError::corrupt(format!(
            "durable database parent {} has an invalid zero link count",
            parent.display()
        )));
    }
    let mode = metadata.mode() & 0o7777;
    if mode != PRIVATE_DIRECTORY_MODE {
        return Err(DurableStorageError::corrupt(format!(
            "durable database parent {} must be an owner-only state directory with mode 0o700, \
             found {mode:#05o}; move the database into a private state directory",
            parent.display(),
        )));
    }
    Ok(())
}

#[cfg(not(unix))]
pub(super) fn validate_database_parent(_path: &Path) -> Result<(), DurableStorageError> {
    Err(unsupported_platform())
}

fn database_parent(path: &Path) -> &Path {
    path.parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."))
}

#[cfg(unix)]
pub(super) fn reserve_new_database_file(path: &Path) -> Result<Handle, DurableStorageError> {
    let reserved = OpenOptions::new()
        .read(true)
        .write(true)
        .create_new(true)
        .mode(PRIVATE_FILE_MODE)
        .custom_flags(libc::O_CLOEXEC | libc::O_NOFOLLOW)
        .open(path)
        .map_err(|error| DurableStorageError::persistence("reserve new database file", error))?;
    reserved
        .set_permissions(std::fs::Permissions::from_mode(PRIVATE_FILE_MODE))
        .map_err(|error| {
            DurableStorageError::persistence("set private database file permissions", error)
        })?;
    validate_secure_file(&reserved, path, "durable database")?;
    let identity = Handle::from_file(reserved).map_err(|error| {
        DurableStorageError::persistence("bind new database file identity", error)
    })?;
    let current = bind_existing_file(path)?;
    if identity != current {
        return Err(DurableStorageError::corrupt(format!(
            "durable database path changed while reserving it: {}",
            path.display()
        )));
    }
    Ok(identity)
}

#[cfg(not(unix))]
pub(super) fn reserve_new_database_file(_path: &Path) -> Result<Handle, DurableStorageError> {
    Err(unsupported_platform())
}

pub(super) fn bind_existing_file(path: &Path) -> Result<Handle, DurableStorageError> {
    let metadata = std::fs::symlink_metadata(path).map_err(|error| {
        DurableStorageError::persistence("inspect existing database file", error)
    })?;
    if !metadata.file_type().is_file() {
        return Err(DurableStorageError::corrupt(format!(
            "durable database path is not a regular file: {}",
            path.display()
        )));
    }
    let first_file = open_existing_no_follow(path, "open existing database file")?;
    validate_secure_file(&first_file, path, "durable database")?;
    let first = Handle::from_file(first_file).map_err(|error| {
        DurableStorageError::persistence("bind existing database file identity", error)
    })?;
    let metadata = std::fs::symlink_metadata(path).map_err(|error| {
        DurableStorageError::persistence("reinspect existing database file", error)
    })?;
    if !metadata.file_type().is_file() {
        return Err(DurableStorageError::corrupt(format!(
            "durable database path changed while binding identity: {}",
            path.display()
        )));
    }
    let current_file = open_existing_no_follow(path, "reopen existing database file")?;
    validate_secure_file(&current_file, path, "durable database")?;
    let current = Handle::from_file(current_file).map_err(|error| {
        DurableStorageError::persistence("rebind existing database file identity", error)
    })?;
    if first != current {
        return Err(DurableStorageError::corrupt(format!(
            "durable database path changed while binding identity: {}",
            path.display()
        )));
    }
    Ok(current)
}

#[cfg(unix)]
fn open_existing_no_follow(
    path: &Path,
    operation: &'static str,
) -> Result<File, DurableStorageError> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .custom_flags(libc::O_CLOEXEC | libc::O_NOFOLLOW)
        .open(path)
        .map_err(|error| DurableStorageError::persistence(operation, error))
}

#[cfg(not(unix))]
fn open_existing_no_follow(
    _path: &Path,
    _operation: &'static str,
) -> Result<File, DurableStorageError> {
    Err(unsupported_platform())
}

#[cfg(unix)]
fn validate_secure_file(file: &File, path: &Path, label: &str) -> Result<(), DurableStorageError> {
    let metadata = validate_secure_file_metadata(file, path, label)?;
    if metadata.nlink() != 1 {
        return Err(DurableStorageError::corrupt(format!(
            "{label} {} must have exactly one hard link, found {}",
            path.display(),
            metadata.nlink()
        )));
    }
    Ok(())
}

#[cfg(unix)]
fn validate_secure_file_metadata(
    file: &File,
    path: &Path,
    label: &str,
) -> Result<std::fs::Metadata, DurableStorageError> {
    let metadata = file
        .metadata()
        .map_err(|error| DurableStorageError::persistence("inspect opened durable file", error))?;
    if !metadata.file_type().is_file() {
        return Err(DurableStorageError::corrupt(format!(
            "{label} is not a regular file: {}",
            path.display()
        )));
    }
    let expected_uid = effective_uid();
    if metadata.uid() != expected_uid {
        return Err(DurableStorageError::corrupt(format!(
            "{label} {} is owned by uid {}, expected effective uid {expected_uid}",
            path.display(),
            metadata.uid()
        )));
    }
    let mode = metadata.mode() & 0o7777;
    if mode != PRIVATE_FILE_MODE {
        return Err(DurableStorageError::corrupt(format!(
            "{label} {} must have mode 0o600, found {mode:#05o}",
            path.display()
        )));
    }
    Ok(metadata)
}

#[cfg(not(unix))]
fn validate_secure_file(
    _file: &File,
    _path: &Path,
    _label: &str,
) -> Result<(), DurableStorageError> {
    Err(unsupported_platform())
}

fn sqlite_sidecar_paths(path: &Path) -> [PathBuf; 2] {
    ["-wal", "-shm"].map(|suffix| {
        let mut sidecar = path.as_os_str().to_os_string();
        sidecar.push(suffix);
        PathBuf::from(sidecar)
    })
}

pub(super) fn reject_bootstrap_sidecars(path: &Path) -> Result<(), DurableStorageError> {
    for sidecar in sqlite_sidecar_paths(path) {
        match std::fs::symlink_metadata(&sidecar) {
            Ok(_) => {
                return Err(DurableStorageError::corrupt(format!(
                    "new durable database has a pre-existing SQLite sidecar: {}",
                    sidecar.display()
                )));
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => {
                return Err(DurableStorageError::persistence(
                    "inspect bootstrap SQLite sidecar",
                    error,
                ));
            }
        }
    }
    Ok(())
}

pub(super) fn validate_existing_sidecars(path: &Path) -> Result<(), DurableStorageError> {
    for sidecar in sqlite_sidecar_paths(path) {
        validate_one_sidecar(&sidecar, false)?;
    }
    Ok(())
}

pub(super) fn harden_sqlite_sidecars(path: &Path) -> Result<(), DurableStorageError> {
    for sidecar in sqlite_sidecar_paths(path) {
        validate_one_sidecar(&sidecar, true)?;
    }
    Ok(())
}

fn validate_one_sidecar(path: &Path, harden: bool) -> Result<(), DurableStorageError> {
    for _ in 0..MAX_SIDECAR_VALIDATION_ATTEMPTS {
        match validate_one_sidecar_attempt(path, harden)? {
            SidecarValidationProgress::Complete | SidecarValidationProgress::Missing => {
                return Ok(());
            }
            SidecarValidationProgress::Retry => {}
        }
    }
    Err(DurableStorageError::corrupt(format!(
        "SQLite sidecar changed repeatedly while validating it: {}",
        path.display()
    )))
}

fn validate_one_sidecar_attempt(
    path: &Path,
    harden: bool,
) -> Result<SidecarValidationProgress, DurableStorageError> {
    let metadata = match std::fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(SidecarValidationProgress::Missing);
        }
        Err(error) => {
            return Err(DurableStorageError::persistence(
                "inspect SQLite sidecar",
                error,
            ));
        }
    };
    if !metadata.file_type().is_file() {
        return Err(DurableStorageError::corrupt(format!(
            "SQLite sidecar is not a regular file: {}",
            path.display()
        )));
    }
    let file = match open_existing_no_follow(path, "open SQLite sidecar without following links") {
        Ok(file) => file,
        Err(_) if sidecar_is_missing(path)? => return Ok(SidecarValidationProgress::Missing),
        Err(error) => return Err(error),
    };
    if harden {
        #[cfg(unix)]
        file.set_permissions(std::fs::Permissions::from_mode(PRIVATE_FILE_MODE))
            .map_err(|error| {
                DurableStorageError::persistence("set private SQLite sidecar permissions", error)
            })?;
        #[cfg(not(unix))]
        return Err(unsupported_platform());
    }
    match validate_opened_sidecar(&file, path)? {
        SidecarValidationProgress::Complete => {}
        progress => return Ok(progress),
    }
    let first = Handle::from_file(file.try_clone().map_err(|error| {
        DurableStorageError::persistence("retain opened SQLite sidecar", error)
    })?)
    .map_err(|error| DurableStorageError::persistence("bind SQLite sidecar identity", error))?;
    let current_file = match open_existing_no_follow(path, "reopen SQLite sidecar") {
        Ok(file) => file,
        Err(_) if sidecar_is_missing(path)? => return Ok(SidecarValidationProgress::Missing),
        Err(error) => return Err(error),
    };
    match validate_opened_sidecar(&current_file, path)? {
        SidecarValidationProgress::Complete => {}
        progress => return Ok(progress),
    }
    let current = Handle::from_file(current_file.try_clone().map_err(|error| {
        DurableStorageError::persistence("retain reopened SQLite sidecar", error)
    })?)
    .map_err(|error| DurableStorageError::persistence("rebind SQLite sidecar identity", error))?;
    if first != current {
        for opened in [&file, &current_file] {
            match validate_opened_sidecar(opened, path)? {
                SidecarValidationProgress::Complete => {}
                progress => return Ok(progress),
            }
        }
        return Err(DurableStorageError::corrupt(format!(
            "SQLite sidecar changed while validating identity: {}",
            path.display()
        )));
    }
    validate_opened_sidecar(&current_file, path)
}

#[cfg(unix)]
fn validate_opened_sidecar(
    file: &File,
    path: &Path,
) -> Result<SidecarValidationProgress, DurableStorageError> {
    let metadata = validate_secure_file_metadata(file, path, "SQLite sidecar")?;
    match metadata.nlink() {
        1 => Ok(SidecarValidationProgress::Complete),
        0 => recheck_unlinked_sidecar_namespace(path),
        links => Err(DurableStorageError::corrupt(format!(
            "SQLite sidecar {} must have exactly one hard link, found {links}",
            path.display()
        ))),
    }
}

#[cfg(not(unix))]
fn validate_opened_sidecar(
    _file: &File,
    _path: &Path,
) -> Result<SidecarValidationProgress, DurableStorageError> {
    Err(unsupported_platform())
}

fn recheck_unlinked_sidecar_namespace(
    path: &Path,
) -> Result<SidecarValidationProgress, DurableStorageError> {
    let metadata = match std::fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(SidecarValidationProgress::Missing);
        }
        Err(error) => {
            return Err(DurableStorageError::persistence(
                "reinspect unlinked SQLite sidecar namespace",
                error,
            ));
        }
    };
    if !metadata.file_type().is_file() {
        return Err(DurableStorageError::corrupt(format!(
            "SQLite sidecar replacement is not a regular file: {}",
            path.display()
        )));
    }
    #[cfg(unix)]
    if metadata.nlink() > 1 {
        return Err(DurableStorageError::corrupt(format!(
            "SQLite sidecar {} must have exactly one hard link, found {}",
            path.display(),
            metadata.nlink()
        )));
    }
    Ok(SidecarValidationProgress::Retry)
}

fn sidecar_is_missing(path: &Path) -> Result<bool, DurableStorageError> {
    match std::fs::symlink_metadata(path) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(true),
        Ok(metadata) if metadata.file_type().is_file() => Ok(false),
        Ok(_) => Err(DurableStorageError::corrupt(format!(
            "SQLite sidecar replacement is not a regular file: {}",
            path.display()
        ))),
        Err(error) => Err(DurableStorageError::persistence(
            "reinspect SQLite sidecar namespace",
            error,
        )),
    }
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;

    use std::os::unix::fs::OpenOptionsExt;

    #[test]
    fn unlinked_open_sidecar_is_missing_only_after_no_follow_recheck() {
        let directory = tempfile::tempdir().expect("create sidecar test directory");
        let sidecar = directory.path().join("database.sqlite3-wal");
        let opened = OpenOptions::new()
            .read(true)
            .write(true)
            .create_new(true)
            .mode(PRIVATE_FILE_MODE)
            .open(&sidecar)
            .expect("create secure sidecar");
        std::fs::remove_file(&sidecar).expect("unlink opened sidecar");
        assert_eq!(
            opened.metadata().expect("inspect unlinked sidecar").nlink(),
            0
        );
        assert_eq!(
            validate_opened_sidecar(&opened, &sidecar).expect("classify unlinked sidecar"),
            SidecarValidationProgress::Missing
        );
    }

    #[test]
    fn unlinked_open_sidecar_does_not_hide_hardlinked_replacement() {
        let directory = tempfile::tempdir().expect("create sidecar test directory");
        let sidecar = directory.path().join("database.sqlite3-wal");
        let opened = OpenOptions::new()
            .read(true)
            .write(true)
            .create_new(true)
            .mode(PRIVATE_FILE_MODE)
            .open(&sidecar)
            .expect("create secure sidecar");
        std::fs::remove_file(&sidecar).expect("unlink opened sidecar");

        drop(
            OpenOptions::new()
                .read(true)
                .write(true)
                .create_new(true)
                .mode(PRIVATE_FILE_MODE)
                .open(&sidecar)
                .expect("create replacement sidecar"),
        );
        let alias = directory.path().join("wal-hardlink");
        std::fs::hard_link(&sidecar, &alias).expect("hardlink replacement sidecar");

        assert!(matches!(
            validate_opened_sidecar(&opened, &sidecar),
            Err(DurableStorageError::Corrupt { detail })
                if detail.contains("exactly one hard link")
        ));
    }
}

#[cfg(unix)]
fn effective_uid() -> u32 {
    // SAFETY: `geteuid` has no preconditions and reads process credentials.
    unsafe { libc::geteuid() }
}

#[cfg(not(unix))]
fn unsupported_platform() -> DurableStorageError {
    DurableStorageError::corrupt(
        "durable SQLite storage requires Unix no-follow and ownership semantics",
    )
}
