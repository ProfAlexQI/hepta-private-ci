use std::fmt;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
#[cfg(unix)]
use std::os::unix::fs::MetadataExt;
#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;

use serde::Deserialize;
use serde::Serialize;

mod monotonic_state;
pub use monotonic_state::RuntimeStateMonotonicState;

use crate::StoreSnapshot;
use crate::durable::DurableIntegrityContext;
use crate::durable::DurableIntegrityKey;

const RUNTIME_STATE_VERSION: u32 = 1;
const PRIVATE_FILE_MODE: u32 = 0o600;
static TEMP_FILE_SEQUENCE: AtomicU64 = AtomicU64::new(0);

#[derive(Debug)]
pub enum RuntimeStateStoreError {
    Persistence {
        operation: &'static str,
        detail: String,
    },
    Corrupt {
        detail: String,
    },
}

impl RuntimeStateStoreError {
    fn persistence(operation: &'static str, error: impl fmt::Display) -> Self {
        Self::Persistence {
            operation,
            detail: error.to_string(),
        }
    }

    fn corrupt(detail: impl Into<String>) -> Self {
        Self::Corrupt {
            detail: detail.into(),
        }
    }
}

impl fmt::Display for RuntimeStateStoreError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Persistence { operation, detail } => {
                write!(formatter, "{operation} failed: {detail}")
            }
            Self::Corrupt { detail } => write!(formatter, "runtime state is corrupt: {detail}"),
        }
    }
}

impl std::error::Error for RuntimeStateStoreError {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct RuntimeStatePayload {
    version: u32,
    generation: u64,
    snapshot: StoreSnapshot,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct RuntimeStateEnvelope {
    payload: RuntimeStatePayload,
    integrity_tag: String,
}

pub(crate) struct RuntimeStatePersistence {
    path: PathBuf,
    integrity: DurableIntegrityContext,
    generation: Mutex<u64>,
    identity: Mutex<FileIdentity>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FileIdentity {
    device: u64,
    inode: u64,
}

impl RuntimeStatePersistence {
    pub(crate) fn bootstrap_new(
        path: &Path,
        integrity_key: DurableIntegrityKey,
    ) -> Result<(Self, StoreSnapshot), RuntimeStateStoreError> {
        validate_absolute_path(path)?;
        validate_parent(path)?;
        if path_exists(path)? {
            return Err(RuntimeStateStoreError::corrupt(format!(
                "bootstrap target already exists: {}",
                path.display()
            )));
        }
        let integrity = integrity_key.into_context();
        let snapshot = StoreSnapshot {
            sessions: Vec::new(),
            memories: Vec::new(),
            transcripts: Vec::new(),
        };
        let identity = write_atomic(path, &integrity, 0, &snapshot, false)?;
        Ok((
            Self {
                path: path.to_path_buf(),
                integrity,
                generation: Mutex::new(0),
                identity: Mutex::new(identity),
            },
            snapshot,
        ))
    }

    pub(crate) fn open_existing(
        path: &Path,
        integrity_key: DurableIntegrityKey,
    ) -> Result<(Self, StoreSnapshot), RuntimeStateStoreError> {
        validate_absolute_path(path)?;
        validate_parent(path)?;
        let integrity = integrity_key.into_context();
        let (envelope, identity) = read_envelope(path)?;
        verify_envelope(&integrity, &envelope)?;
        Ok((
            Self {
                path: path.to_path_buf(),
                integrity,
                generation: Mutex::new(envelope.payload.generation),
                identity: Mutex::new(identity),
            },
            envelope.payload.snapshot,
        ))
    }

    pub(crate) fn persist(&self, snapshot: &StoreSnapshot) -> Result<(), RuntimeStateStoreError> {
        let mut generation = self
            .generation
            .lock()
            .map_err(|_| RuntimeStateStoreError::corrupt("generation mutex poisoned"))?;
        let mut identity = self
            .identity
            .lock()
            .map_err(|_| RuntimeStateStoreError::corrupt("identity mutex poisoned"))?;
        validate_namespace_identity(&self.path, *identity)?;
        let next_generation = generation
            .checked_add(1)
            .ok_or_else(|| RuntimeStateStoreError::corrupt("runtime state generation exhausted"))?;
        let next_identity =
            write_atomic(&self.path, &self.integrity, next_generation, snapshot, true)?;
        *generation = next_generation;
        *identity = next_identity;
        Ok(())
    }
}

fn verify_envelope(
    integrity: &DurableIntegrityContext,
    envelope: &RuntimeStateEnvelope,
) -> Result<(), RuntimeStateStoreError> {
    if envelope.payload.version != RUNTIME_STATE_VERSION {
        return Err(RuntimeStateStoreError::corrupt(format!(
            "unsupported version {}; expected {RUNTIME_STATE_VERSION}",
            envelope.payload.version
        )));
    }
    let payload_json = serde_json::to_string(&envelope.payload)
        .map_err(|error| RuntimeStateStoreError::persistence("serialize runtime state", error))?;
    integrity
        .verify(
            &payload_json,
            &envelope.integrity_tag,
            "runtime state envelope",
        )
        .map_err(|error| RuntimeStateStoreError::corrupt(format!("{error:?}")))
}

fn write_atomic(
    path: &Path,
    integrity: &DurableIntegrityContext,
    generation: u64,
    snapshot: &StoreSnapshot,
    replace_existing: bool,
) -> Result<FileIdentity, RuntimeStateStoreError> {
    let payload = RuntimeStatePayload {
        version: RUNTIME_STATE_VERSION,
        generation,
        snapshot: snapshot.clone(),
    };
    let payload_json = serde_json::to_string(&payload)
        .map_err(|error| RuntimeStateStoreError::persistence("serialize runtime state", error))?;
    let envelope = RuntimeStateEnvelope {
        integrity_tag: integrity
            .protect(&payload_json)
            .map_err(|error| RuntimeStateStoreError::corrupt(format!("{error:?}")))?,
        payload,
    };
    let bytes = serde_json::to_vec(&envelope)
        .map_err(|error| RuntimeStateStoreError::persistence("serialize state envelope", error))?;
    let parent = path.parent().ok_or_else(|| {
        RuntimeStateStoreError::corrupt(format!("state path has no parent: {}", path.display()))
    })?;
    let file_name = path
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| {
            RuntimeStateStoreError::corrupt(format!(
                "state path has invalid filename: {}",
                path.display()
            ))
        })?;
    let sequence = TEMP_FILE_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let temporary_path = parent.join(format!(
        ".{file_name}.tmp-{}-{sequence}",
        std::process::id()
    ));
    let write_result = (|| {
        let mut temporary = open_new_private(&temporary_path)?;
        temporary
            .write_all(&bytes)
            .map_err(|error| RuntimeStateStoreError::persistence("write runtime state", error))?;
        temporary
            .sync_all()
            .map_err(|error| RuntimeStateStoreError::persistence("sync runtime state", error))?;
        if replace_existing {
            std::fs::rename(&temporary_path, path).map_err(|error| {
                RuntimeStateStoreError::persistence("publish runtime state", error)
            })?;
        } else {
            std::fs::hard_link(&temporary_path, path).map_err(|error| {
                RuntimeStateStoreError::persistence("publish new runtime state", error)
            })?;
            std::fs::remove_file(&temporary_path).map_err(|error| {
                RuntimeStateStoreError::persistence("retire runtime state temporary", error)
            })?;
        }
        sync_directory(parent)?;
        validate_secure_file(path)
    })();
    if temporary_path.exists() {
        let _ = std::fs::remove_file(&temporary_path);
    }
    write_result
}

fn read_envelope(
    path: &Path,
) -> Result<(RuntimeStateEnvelope, FileIdentity), RuntimeStateStoreError> {
    let mut file = open_existing_private(path)?;
    let identity = validate_opened_file(&file, path)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)
        .map_err(|error| RuntimeStateStoreError::persistence("read runtime state", error))?;
    validate_namespace_identity(path, identity)?;
    let envelope = serde_json::from_slice(&bytes).map_err(|error| {
        RuntimeStateStoreError::corrupt(format!("invalid envelope JSON: {error}"))
    })?;
    Ok((envelope, identity))
}

fn validate_absolute_path(path: &Path) -> Result<(), RuntimeStateStoreError> {
    if !path.is_absolute() {
        return Err(RuntimeStateStoreError::corrupt(format!(
            "runtime state path must be absolute: {}",
            path.display()
        )));
    }
    Ok(())
}

fn validate_parent(path: &Path) -> Result<(), RuntimeStateStoreError> {
    let parent = path.parent().ok_or_else(|| {
        RuntimeStateStoreError::corrupt(format!("state path has no parent: {}", path.display()))
    })?;
    let metadata = std::fs::symlink_metadata(parent).map_err(|error| {
        RuntimeStateStoreError::persistence("inspect runtime state parent", error)
    })?;
    if !metadata.is_dir() || metadata.file_type().is_symlink() {
        return Err(RuntimeStateStoreError::corrupt(format!(
            "runtime state parent must be a non-symlink directory: {}",
            parent.display()
        )));
    }
    #[cfg(unix)]
    {
        if metadata.uid() != unsafe { libc::geteuid() } {
            return Err(RuntimeStateStoreError::corrupt(format!(
                "runtime state parent owner mismatch: {}",
                parent.display()
            )));
        }
        if metadata.nlink() == 0 {
            return Err(RuntimeStateStoreError::corrupt(format!(
                "runtime state parent has zero links: {}",
                parent.display()
            )));
        }
        let mode = metadata.permissions().mode() & 0o7777;
        if mode != 0o700 {
            return Err(RuntimeStateStoreError::corrupt(format!(
                "runtime state parent {} must have mode 0o700, found {mode:#05o}",
                parent.display()
            )));
        }
    }
    #[cfg(not(unix))]
    return Err(RuntimeStateStoreError::corrupt(
        "durable runtime state requires Unix directory identity semantics",
    ));
    Ok(())
}

fn path_exists(path: &Path) -> Result<bool, RuntimeStateStoreError> {
    match std::fs::symlink_metadata(path) {
        Ok(_) => Ok(true),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(error) => Err(RuntimeStateStoreError::persistence(
            "inspect runtime state path",
            error,
        )),
    }
}

#[cfg(unix)]
fn open_new_private(path: &Path) -> Result<File, RuntimeStateStoreError> {
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(PRIVATE_FILE_MODE)
        .custom_flags(libc::O_NOFOLLOW)
        .open(path)
        .map_err(|error| RuntimeStateStoreError::persistence("create private runtime state", error))
}

#[cfg(not(unix))]
fn open_new_private(_path: &Path) -> Result<File, RuntimeStateStoreError> {
    Err(RuntimeStateStoreError::corrupt(
        "durable runtime state requires Unix no-follow file semantics",
    ))
}

#[cfg(unix)]
fn open_existing_private(path: &Path) -> Result<File, RuntimeStateStoreError> {
    OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NOFOLLOW)
        .open(path)
        .map_err(|error| RuntimeStateStoreError::persistence("open private runtime state", error))
}

#[cfg(not(unix))]
fn open_existing_private(_path: &Path) -> Result<File, RuntimeStateStoreError> {
    Err(RuntimeStateStoreError::corrupt(
        "durable runtime state requires Unix no-follow file semantics",
    ))
}

fn validate_secure_file(path: &Path) -> Result<FileIdentity, RuntimeStateStoreError> {
    let file = open_existing_private(path)?;
    validate_opened_file(&file, path)
}

#[cfg(unix)]
fn validate_opened_file(file: &File, path: &Path) -> Result<FileIdentity, RuntimeStateStoreError> {
    let metadata = file.metadata().map_err(|error| {
        RuntimeStateStoreError::persistence("inspect opened runtime state", error)
    })?;
    if !metadata.is_file() || metadata.nlink() != 1 {
        return Err(RuntimeStateStoreError::corrupt(format!(
            "runtime state must be a regular file with one link: {}",
            path.display()
        )));
    }
    if metadata.uid() != unsafe { libc::geteuid() } {
        return Err(RuntimeStateStoreError::corrupt(format!(
            "runtime state owner mismatch: {}",
            path.display()
        )));
    }
    let mode = metadata.permissions().mode() & 0o777;
    if mode != PRIVATE_FILE_MODE {
        return Err(RuntimeStateStoreError::corrupt(format!(
            "runtime state {} must have mode 0o600, found {mode:#05o}",
            path.display()
        )));
    }
    Ok(FileIdentity {
        device: metadata.dev(),
        inode: metadata.ino(),
    })
}

#[cfg(not(unix))]
fn validate_opened_file(
    _file: &File,
    _path: &Path,
) -> Result<FileIdentity, RuntimeStateStoreError> {
    Err(RuntimeStateStoreError::corrupt(
        "durable runtime state requires Unix file identity semantics",
    ))
}

fn validate_namespace_identity(
    path: &Path,
    expected: FileIdentity,
) -> Result<(), RuntimeStateStoreError> {
    let actual = validate_secure_file(path)?;
    if actual != expected {
        return Err(RuntimeStateStoreError::corrupt(format!(
            "runtime state path identity changed: {}",
            path.display()
        )));
    }
    Ok(())
}

fn sync_directory(path: &Path) -> Result<(), RuntimeStateStoreError> {
    File::open(path)
        .and_then(|directory| directory.sync_all())
        .map_err(|error| RuntimeStateStoreError::persistence("sync runtime state parent", error))
}
