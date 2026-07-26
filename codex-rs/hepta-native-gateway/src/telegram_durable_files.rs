//! Descriptor-relative durable files for the live Telegram delivery pipeline.
//!
//! These helpers deliberately do not accept an already-resolved filesystem
//! target. Every path component is opened beneath a retained directory
//! descriptor with `O_NOFOLLOW`; cursor publication replaces a directory entry
//! atomically and delivery append is locked and fsynced before success.

use std::path::Path;
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use anyhow::Context;
use anyhow::Result;
use hepta_runtime::NativeTelegramCursorStatus;
use hepta_runtime::NativeTelegramCursorStatusInput;
use hepta_runtime::build_native_telegram_cursor_status;
use hepta_runtime::native_telegram_cursor_body;
use serde_json::Value;

const PRIVATE_DIRECTORY_MODE: u32 = 0o700;
const PRIVATE_FILE_MODE: u32 = 0o600;
const MAX_CURSOR_BYTES: u64 = 4096;
const MAX_DELIVERY_LEDGER_BYTES: u64 = 16 * 1024 * 1024;
pub(crate) fn cursor_status(
    requested: bool,
    path: &Path,
    logical_path: &'static str,
) -> NativeTelegramCursorStatus {
    if !requested {
        return build_native_telegram_cursor_status(NativeTelegramCursorStatusInput {
            requested,
            cursor_path: logical_path,
            cursor_file_present: false,
            cursor_updated_at_unix_ms: None,
            raw_json: None,
            read_error: None,
        });
    }

    match read_private_file(path, MAX_CURSOR_BYTES) {
        Ok(None) => build_native_telegram_cursor_status(NativeTelegramCursorStatusInput {
            requested: true,
            cursor_path: logical_path,
            cursor_file_present: false,
            cursor_updated_at_unix_ms: None,
            raw_json: None,
            read_error: None,
        }),
        Ok(Some(opened)) => {
            let raw = match String::from_utf8(opened.bytes) {
                Ok(raw) => raw,
                Err(_) => {
                    return build_native_telegram_cursor_status(NativeTelegramCursorStatusInput {
                        requested: true,
                        cursor_path: logical_path,
                        cursor_file_present: true,
                        cursor_updated_at_unix_ms: opened.modified_unix_ms,
                        raw_json: None,
                        read_error: Some(
                            "failed to read Telegram cursor file: cursor is not UTF-8",
                        ),
                    });
                }
            };
            build_native_telegram_cursor_status(NativeTelegramCursorStatusInput {
                requested: true,
                cursor_path: logical_path,
                cursor_file_present: true,
                cursor_updated_at_unix_ms: opened.modified_unix_ms,
                raw_json: Some(&raw),
                read_error: None,
            })
        }
        Err(error) => {
            let error = format!("failed to read Telegram cursor file securely: {error:#}");
            build_native_telegram_cursor_status(NativeTelegramCursorStatusInput {
                requested: true,
                cursor_path: logical_path,
                cursor_file_present: true,
                cursor_updated_at_unix_ms: None,
                raw_json: None,
                read_error: Some(&error),
            })
        }
    }
}

pub(crate) fn write_cursor_next_update_offset(path: &Path, offset: i64) -> Result<()> {
    let body = native_telegram_cursor_body(offset, now_unix_ms()).map_err(anyhow::Error::msg)?;
    let mut bytes = serde_json::to_vec_pretty(&body).context("encode Telegram cursor JSON")?;
    bytes.push(b'\n');
    if bytes.len() as u64 > MAX_CURSOR_BYTES {
        anyhow::bail!("Telegram cursor exceeds its bounded size");
    }
    write_private_file_atomically(path, &bytes, MAX_CURSOR_BYTES, ".hepta-telegram-cursor")
        .with_context(|| format!("publish secure Telegram cursor {}", path.display()))
}

pub(crate) fn update_private_state_atomically<T>(
    path: &Path,
    max_bytes: u64,
    temporary_prefix: &str,
    update: impl FnOnce(Option<&[u8]>) -> Result<(Vec<u8>, T)>,
) -> Result<T> {
    update_private_file_atomically(path, max_bytes, temporary_prefix, update)
}

pub(crate) fn read_private_state(path: &Path, max_bytes: u64) -> Result<Option<Vec<u8>>> {
    read_private_file(path, max_bytes).map(|opened| opened.map(|opened| opened.bytes))
}

pub(crate) fn append_delivery_lifecycle_record(path: &Path, record: &Value) -> Result<()> {
    let mut bytes =
        serde_json::to_vec(record).context("render secure Telegram delivery ledger record")?;
    bytes.push(b'\n');
    append_private_file(path, &bytes)
        .with_context(|| format!("append secure Telegram delivery ledger {}", path.display()))
}

struct OpenedPrivateFile {
    bytes: Vec<u8>,
    modified_unix_ms: Option<u64>,
}

#[cfg(unix)]
mod unix {
    use std::ffi::CString;
    use std::fs::File;
    use std::io;
    use std::io::Read;
    use std::io::Write;
    use std::os::fd::AsRawFd;
    use std::os::fd::FromRawFd;
    use std::os::unix::ffi::OsStrExt;
    use std::os::unix::fs::MetadataExt;
    use std::os::unix::fs::PermissionsExt;
    use std::path::Component;
    use std::path::Path;
    use std::sync::atomic::AtomicU64;
    use std::sync::atomic::Ordering;

    use anyhow::Context;
    use anyhow::Result;

    use super::MAX_DELIVERY_LEDGER_BYTES;
    use super::OpenedPrivateFile;
    use super::PRIVATE_DIRECTORY_MODE;
    use super::PRIVATE_FILE_MODE;
    use super::duration_millis_u64;

    static TEMPORARY_SEQUENCE: AtomicU64 = AtomicU64::new(0);

    pub(super) fn read_private_file(
        path: &Path,
        max_bytes: u64,
    ) -> Result<Option<OpenedPrivateFile>> {
        let (parent, name) = match open_parent(path, false) {
            Ok(value) => value,
            Err(error)
                if error
                    .downcast_ref::<io::Error>()
                    .is_some_and(|error| error.kind() == io::ErrorKind::NotFound) =>
            {
                return Ok(None);
            }
            Err(error) => return Err(error),
        };
        let file = match open_file_at(
            &parent,
            &name,
            libc::O_RDONLY | libc::O_NONBLOCK | libc::O_CLOEXEC | libc::O_NOFOLLOW,
            0,
        ) {
            Ok(file) => file,
            Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
            Err(error) => return Err(error).context("open private Telegram file without links"),
        };
        validate_private_file(&file, max_bytes)?;
        let metadata = file.metadata().context("inspect private Telegram file")?;
        let modified_unix_ms = metadata
            .modified()
            .ok()
            .and_then(|modified| modified.duration_since(std::time::UNIX_EPOCH).ok())
            .map(duration_millis_u64);
        let mut bytes = Vec::with_capacity(metadata.len() as usize);
        file.take(max_bytes + 1)
            .read_to_end(&mut bytes)
            .context("read bounded private Telegram file")?;
        if bytes.len() as u64 > max_bytes {
            anyhow::bail!("private Telegram file exceeds its bounded size");
        }
        Ok(Some(OpenedPrivateFile {
            bytes,
            modified_unix_ms,
        }))
    }

    pub(super) fn append_private_file(path: &Path, bytes: &[u8]) -> Result<()> {
        let (parent, name) = open_parent(path, true)?;
        let mut file = open_file_at(
            &parent,
            &name,
            libc::O_WRONLY
                | libc::O_APPEND
                | libc::O_CREAT
                | libc::O_NONBLOCK
                | libc::O_CLOEXEC
                | libc::O_NOFOLLOW,
            PRIVATE_FILE_MODE,
        )
        .context("open private Telegram delivery ledger without links")?;
        lock_file(&file)?;
        validate_private_file(&file, MAX_DELIVERY_LEDGER_BYTES)?;
        let current = file.metadata()?.len();
        if current.saturating_add(bytes.len() as u64) > MAX_DELIVERY_LEDGER_BYTES {
            anyhow::bail!("Telegram delivery ledger exceeds its bounded size");
        }
        file.write_all(bytes)
            .context("write Telegram delivery ledger record")?;
        file.sync_all()
            .context("fsync Telegram delivery ledger record")?;
        parent
            .sync_all()
            .context("fsync Telegram delivery ledger parent")?;
        Ok(())
    }

    pub(super) fn write_private_file_atomically(
        path: &Path,
        bytes: &[u8],
        max_bytes: u64,
        temporary_prefix: &str,
    ) -> Result<()> {
        update_private_file_atomically(path, max_bytes, temporary_prefix, |_| {
            Ok((bytes.to_vec(), ()))
        })
    }

    pub(super) fn update_private_file_atomically<T>(
        path: &Path,
        max_bytes: u64,
        temporary_prefix: &str,
        update: impl FnOnce(Option<&[u8]>) -> Result<(Vec<u8>, T)>,
    ) -> Result<T> {
        let (parent, name) = open_parent(path, true)?;
        let lock_name = sibling_name(&name, ".lock")?;
        let lock = open_file_at(
            &parent,
            &lock_name,
            libc::O_RDWR | libc::O_CREAT | libc::O_NONBLOCK | libc::O_CLOEXEC | libc::O_NOFOLLOW,
            PRIVATE_FILE_MODE,
        )
        .context("open private Telegram cursor lock without links")?;
        lock_file(&lock)?;
        validate_private_file(&lock, 0)?;
        let current = read_existing_target(&parent, &name, max_bytes)?;
        let (bytes, output) = update(current.as_deref())?;
        if bytes.len() as u64 > max_bytes {
            anyhow::bail!("private atomic file exceeds its bounded size");
        }
        let temporary_prefix = temporary_prefix
            .strip_prefix('.')
            .unwrap_or(temporary_prefix);
        if temporary_prefix.is_empty()
            || !temporary_prefix
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
        {
            anyhow::bail!("private atomic staging prefix is invalid");
        }

        let mut temporary = None;
        for _ in 0..32 {
            let sequence = TEMPORARY_SEQUENCE.fetch_add(1, Ordering::Relaxed);
            let candidate = CString::new(format!(
                ".{temporary_prefix}-{}-{sequence}.tmp",
                std::process::id()
            ))
            .context("encode private atomic staging name")?;
            match open_file_at(
                &parent,
                &candidate,
                libc::O_WRONLY
                    | libc::O_CREAT
                    | libc::O_EXCL
                    | libc::O_NONBLOCK
                    | libc::O_CLOEXEC
                    | libc::O_NOFOLLOW,
                PRIVATE_FILE_MODE,
            ) {
                Ok(file) => {
                    temporary = Some((candidate, file));
                    break;
                }
                Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
                Err(error) => {
                    return Err(error).context("create private Telegram cursor staging file");
                }
            }
        }
        let (temporary_name, mut temporary_file) =
            temporary.context("exhausted private Telegram cursor staging names")?;
        let cleanup = TemporaryEntry {
            parent_fd: parent.as_raw_fd(),
            name: &temporary_name,
            armed: true,
        };
        temporary_file
            .write_all(&bytes)
            .context("write private Telegram cursor staging file")?;
        temporary_file
            .sync_all()
            .context("fsync private Telegram cursor staging file")?;
        validate_private_file(&temporary_file, max_bytes)?;
        rename_at(&parent, &temporary_name, &name)?;
        cleanup.disarm();
        parent.sync_all().context("fsync Telegram cursor parent")?;
        Ok(output)
    }

    struct TemporaryEntry<'a> {
        parent_fd: libc::c_int,
        name: &'a CString,
        armed: bool,
    }

    impl TemporaryEntry<'_> {
        fn disarm(mut self) {
            self.armed = false;
        }
    }

    impl Drop for TemporaryEntry<'_> {
        fn drop(&mut self) {
            if self.armed {
                // SAFETY: `parent_fd` is live for the guard lifetime and `name`
                // is a NUL-terminated component owned by the caller.
                let _ = unsafe { libc::unlinkat(self.parent_fd, self.name.as_ptr(), 0) };
            }
        }
    }

    fn read_existing_target(
        parent: &File,
        name: &CString,
        max_bytes: u64,
    ) -> Result<Option<Vec<u8>>> {
        match open_file_at(
            parent,
            name,
            libc::O_RDONLY | libc::O_NONBLOCK | libc::O_CLOEXEC | libc::O_NOFOLLOW,
            0,
        ) {
            Ok(mut file) => {
                validate_private_file(&file, max_bytes)?;
                let mut bytes = Vec::with_capacity(file.metadata()?.len() as usize);
                Read::by_ref(&mut file)
                    .take(max_bytes + 1)
                    .read_to_end(&mut bytes)
                    .context("read existing private atomic file")?;
                if bytes.len() as u64 > max_bytes {
                    anyhow::bail!("existing private atomic file exceeds its bounded size");
                }
                Ok(Some(bytes))
            }
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(error) => Err(error).context("inspect existing Telegram cursor without links"),
        }
    }

    fn open_parent(path: &Path, create: bool) -> Result<(File, CString)> {
        let mut components = path.components().peekable();
        let absolute = matches!(components.peek(), Some(Component::RootDir));
        if absolute {
            components.next();
        }
        let mut names = Vec::new();
        for component in components {
            match component {
                Component::CurDir => {}
                Component::Normal(value) => names.push(
                    CString::new(value.as_bytes())
                        .context("Telegram durable path contains an interior NUL")?,
                ),
                Component::ParentDir | Component::Prefix(_) | Component::RootDir => {
                    anyhow::bail!("Telegram durable path must not contain traversal");
                }
            }
        }
        let name = names
            .pop()
            .context("Telegram durable path must name a file")?;
        let start = CString::new(if absolute { "/" } else { "." })?;
        let mut directory =
            open_directory_at(libc::AT_FDCWD, &start).context("open Telegram durable path root")?;
        for component in names {
            if create {
                // SAFETY: the retained directory descriptor and component are
                // valid; mkdirat does not follow the final component.
                let rc = unsafe {
                    libc::mkdirat(
                        directory.as_raw_fd(),
                        component.as_ptr(),
                        PRIVATE_DIRECTORY_MODE as libc::mode_t,
                    )
                };
                if rc != 0 {
                    let error = io::Error::last_os_error();
                    if error.kind() != io::ErrorKind::AlreadyExists {
                        return Err(error).context("create private Telegram durable directory");
                    }
                }
            }
            directory = open_directory_at(directory.as_raw_fd(), &component)
                .context("open Telegram durable directory without links")?;
        }
        validate_private_directory(&directory)?;
        Ok((directory, name))
    }

    fn open_directory_at(parent_fd: libc::c_int, name: &CString) -> io::Result<File> {
        // SAFETY: `name` is NUL terminated and the returned descriptor is
        // uniquely transferred into `File`.
        let fd = unsafe {
            libc::openat(
                parent_fd,
                name.as_ptr(),
                libc::O_RDONLY | libc::O_DIRECTORY | libc::O_CLOEXEC | libc::O_NOFOLLOW,
            )
        };
        if fd < 0 {
            return Err(io::Error::last_os_error());
        }
        // SAFETY: `fd` was just returned as an owned descriptor.
        Ok(unsafe { File::from_raw_fd(fd) })
    }

    fn open_file_at(
        parent: &File,
        name: &CString,
        flags: libc::c_int,
        mode: u32,
    ) -> io::Result<File> {
        // SAFETY: the retained parent descriptor and NUL-terminated component
        // are valid; the returned descriptor is uniquely transferred to File.
        let fd = unsafe {
            libc::openat(
                parent.as_raw_fd(),
                name.as_ptr(),
                flags,
                mode as libc::c_uint,
            )
        };
        if fd < 0 {
            return Err(io::Error::last_os_error());
        }
        // SAFETY: `fd` was just returned as an owned descriptor.
        Ok(unsafe { File::from_raw_fd(fd) })
    }

    fn rename_at(parent: &File, source: &CString, target: &CString) -> Result<()> {
        // SAFETY: both names are NUL-terminated entries under the retained
        // directory descriptor.
        if unsafe {
            libc::renameat(
                parent.as_raw_fd(),
                source.as_ptr(),
                parent.as_raw_fd(),
                target.as_ptr(),
            )
        } != 0
        {
            return Err(io::Error::last_os_error())
                .context("atomically publish private Telegram cursor");
        }
        Ok(())
    }

    fn sibling_name(name: &CString, suffix: &str) -> Result<CString> {
        let mut bytes = name.as_bytes().to_vec();
        bytes.extend_from_slice(suffix.as_bytes());
        CString::new(bytes).context("encode Telegram sibling file name")
    }

    fn validate_private_directory(directory: &File) -> Result<()> {
        let metadata = directory
            .metadata()
            .context("inspect Telegram durable directory")?;
        if !metadata.file_type().is_dir()
            || metadata.uid() != effective_uid()
            || metadata.permissions().mode() & 0o7777 != PRIVATE_DIRECTORY_MODE
        {
            anyhow::bail!("Telegram durable parent must be an owned mode-0700 directory");
        }
        Ok(())
    }

    fn validate_private_file(file: &File, max_bytes: u64) -> Result<()> {
        let metadata = file.metadata().context("inspect private Telegram file")?;
        if !metadata.file_type().is_file()
            || metadata.uid() != effective_uid()
            || metadata.nlink() != 1
            || metadata.permissions().mode() & 0o7777 != PRIVATE_FILE_MODE
            || metadata.len() > max_bytes
        {
            anyhow::bail!("private Telegram file secure invariant failed");
        }
        Ok(())
    }

    fn lock_file(file: &File) -> Result<()> {
        // SAFETY: flock consumes a live descriptor and a constant operation.
        if unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX) } != 0 {
            return Err(io::Error::last_os_error()).context("lock private Telegram file");
        }
        Ok(())
    }

    fn effective_uid() -> u32 {
        // SAFETY: geteuid has no preconditions.
        unsafe { libc::geteuid() }
    }
}

#[cfg(unix)]
use unix::append_private_file;
#[cfg(unix)]
use unix::read_private_file;
#[cfg(unix)]
use unix::update_private_file_atomically;
#[cfg(unix)]
use unix::write_private_file_atomically;

#[cfg(not(unix))]
fn read_private_file(_path: &Path, _max_bytes: u64) -> Result<Option<OpenedPrivateFile>> {
    anyhow::bail!("secure Telegram durable files require Unix descriptor semantics")
}

#[cfg(not(unix))]
fn append_private_file(_path: &Path, _bytes: &[u8]) -> Result<()> {
    anyhow::bail!("secure Telegram durable files require Unix descriptor semantics")
}

#[cfg(not(unix))]
fn write_private_file_atomically(
    _path: &Path,
    _bytes: &[u8],
    _max_bytes: u64,
    _temporary_prefix: &str,
) -> Result<()> {
    anyhow::bail!("secure Telegram durable files require Unix descriptor semantics")
}

#[cfg(not(unix))]
fn update_private_file_atomically<T>(
    _path: &Path,
    _max_bytes: u64,
    _temporary_prefix: &str,
    _update: impl FnOnce(Option<&[u8]>) -> Result<(Vec<u8>, T)>,
) -> Result<T> {
    anyhow::bail!("secure Telegram durable files require Unix descriptor semantics")
}

fn duration_millis_u64(duration: Duration) -> u64 {
    duration.as_millis().min(u128::from(u64::MAX)) as u64
}

fn now_unix_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(duration_millis_u64)
        .unwrap_or(0)
}

#[cfg(all(test, unix))]
#[path = "../tests/unit/telegram_durable_files.rs"]
mod tests;
