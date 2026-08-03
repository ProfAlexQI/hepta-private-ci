#![deny(unsafe_op_in_unsafe_fn)]
//! Descriptor-relative private durable storage for authenticated journals.

const PRIVATE_DIRECTORY_MODE: u32 = 0o700;
const PRIVATE_FILE_MODE: u32 = 0o600;

mod store;

pub use store::AuthenticatedJournalStore;
pub use store::AuthenticatedJournalStoreLock;
pub use store::DurableFileSnapshot;
pub use store::LegacyJournalMigrationStore;

#[cfg(unix)]
mod platform {
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
    use std::time::UNIX_EPOCH;

    use anyhow::Context;
    use anyhow::Result;

    use super::AuthenticatedJournalStoreLock;
    use super::DurableFileSnapshot;
    use super::PRIVATE_DIRECTORY_MODE;
    use super::PRIVATE_FILE_MODE;

    static TEMPORARY_SEQUENCE: AtomicU64 = AtomicU64::new(0);

    #[derive(Clone, Copy)]
    enum ParentDirectoryPolicy {
        Private,
        LegacyMigration,
    }

    pub(super) fn read_private_file(
        path: &Path,
        max_bytes: u64,
    ) -> Result<Option<DurableFileSnapshot>> {
        read_file_with_parent_policy(path, max_bytes, ParentDirectoryPolicy::Private)
    }

    pub(super) fn read_legacy_private_file(
        path: &Path,
        max_bytes: u64,
    ) -> Result<Option<DurableFileSnapshot>> {
        read_file_with_parent_policy(path, max_bytes, ParentDirectoryPolicy::LegacyMigration)
    }

    fn read_file_with_parent_policy(
        path: &Path,
        max_bytes: u64,
        policy: ParentDirectoryPolicy,
    ) -> Result<Option<DurableFileSnapshot>> {
        let (parent, name) = match open_parent(path, false, policy) {
            Ok(value) => value,
            Err(error) if is_not_found(&error) => return Ok(None),
            Err(error) => return Err(error),
        };
        read_existing_target(&parent, &name, max_bytes)
    }

    pub(super) fn write_private_file_atomically(
        path: &Path,
        bytes: &[u8],
        max_bytes: u64,
        staging_prefix: &str,
    ) -> Result<()> {
        write_file_with_parent_policy(
            path,
            bytes,
            max_bytes,
            staging_prefix,
            ParentDirectoryPolicy::Private,
        )
    }

    pub(super) fn write_legacy_private_file_atomically(
        path: &Path,
        bytes: &[u8],
        max_bytes: u64,
        staging_prefix: &str,
    ) -> Result<()> {
        write_file_with_parent_policy(
            path,
            bytes,
            max_bytes,
            staging_prefix,
            ParentDirectoryPolicy::LegacyMigration,
        )
    }

    fn write_file_with_parent_policy(
        path: &Path,
        bytes: &[u8],
        max_bytes: u64,
        staging_prefix: &str,
        policy: ParentDirectoryPolicy,
    ) -> Result<()> {
        if bytes.len() as u64 > max_bytes {
            anyhow::bail!("authenticated journal file exceeds its bounded size");
        }
        let (parent, name) = open_parent(path, true, policy)?;
        let prefix = staging_prefix.strip_prefix('.').unwrap_or(staging_prefix);
        let mut temporary = None;
        for _ in 0..32 {
            let sequence = TEMPORARY_SEQUENCE.fetch_add(1, Ordering::Relaxed);
            let candidate =
                CString::new(format!(".{prefix}-{}-{sequence}.tmp", std::process::id()))
                    .context("encode authenticated journal staging name")?;
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
                    return Err(error).context("create authenticated journal staging file");
                }
            }
        }
        let (temporary_name, mut temporary_file) =
            temporary.context("exhausted authenticated journal staging names")?;
        let cleanup = TemporaryEntry {
            parent_fd: parent.as_raw_fd(),
            name: &temporary_name,
            armed: true,
        };
        temporary_file
            .write_all(bytes)
            .context("write authenticated journal staging file")?;
        temporary_file
            .sync_all()
            .context("fsync authenticated journal staging file")?;
        validate_private_file(&temporary_file, max_bytes)?;
        rename_at(&parent, &temporary_name, &name)?;
        cleanup.disarm();
        parent
            .sync_all()
            .context("fsync authenticated journal parent")?;
        Ok(())
    }

    pub(super) fn append_private_file(path: &Path, bytes: &[u8], max_bytes: u64) -> Result<()> {
        let (parent, name) = open_parent(path, true, ParentDirectoryPolicy::Private)?;
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
        .context("open authenticated journal append target without links")?;
        lock_file(&file)?;
        validate_private_file(&file, max_bytes)?;
        let current = file.metadata()?.len();
        if current.saturating_add(bytes.len() as u64) > max_bytes {
            anyhow::bail!("authenticated journal append exceeds its bounded size");
        }
        file.write_all(bytes)
            .context("write authenticated journal append record")?;
        file.sync_all()
            .context("fsync authenticated journal append record")?;
        parent
            .sync_all()
            .context("fsync authenticated journal append parent")?;
        Ok(())
    }

    pub(super) fn lock_private_file(path: &Path) -> Result<AuthenticatedJournalStoreLock> {
        lock_file_with_parent_policy(path, ParentDirectoryPolicy::Private)
    }

    pub(super) fn lock_legacy_private_file(path: &Path) -> Result<AuthenticatedJournalStoreLock> {
        lock_file_with_parent_policy(path, ParentDirectoryPolicy::LegacyMigration)
    }

    fn lock_file_with_parent_policy(
        path: &Path,
        policy: ParentDirectoryPolicy,
    ) -> Result<AuthenticatedJournalStoreLock> {
        let (parent, name) = open_parent(path, true, policy)?;
        let mut file = None;
        for _ in 0..32 {
            match open_file_at(
                &parent,
                &name,
                libc::O_RDWR | libc::O_NONBLOCK | libc::O_CLOEXEC | libc::O_NOFOLLOW,
                0,
            ) {
                Ok(existing) => {
                    file = Some(existing);
                    break;
                }
                Err(error) if error.kind() == io::ErrorKind::NotFound => {}
                Err(error) => {
                    return Err(error).context("open authenticated journal lock without links");
                }
            }
            match open_file_at(
                &parent,
                &name,
                libc::O_RDWR
                    | libc::O_CREAT
                    | libc::O_EXCL
                    | libc::O_NONBLOCK
                    | libc::O_CLOEXEC
                    | libc::O_NOFOLLOW,
                PRIVATE_FILE_MODE,
            ) {
                Ok(created) => {
                    file = Some(created);
                    break;
                }
                Err(error)
                    if matches!(
                        error.kind(),
                        io::ErrorKind::AlreadyExists | io::ErrorKind::NotFound
                    ) =>
                {
                    continue;
                }
                Err(error) => {
                    return Err(error).context("create authenticated journal lock without links");
                }
            }
        }
        let file = file.context("exhausted authenticated journal lock creation attempts")?;
        validate_private_file(&file, 0)?;
        lock_file(&file)?;
        Ok(AuthenticatedJournalStoreLock { file })
    }

    fn read_existing_target(
        parent: &File,
        name: &CString,
        max_bytes: u64,
    ) -> Result<Option<DurableFileSnapshot>> {
        match open_file_at(
            parent,
            name,
            libc::O_RDONLY | libc::O_NONBLOCK | libc::O_CLOEXEC | libc::O_NOFOLLOW,
            0,
        ) {
            Ok(mut file) => {
                validate_private_file(&file, max_bytes)?;
                let metadata = file.metadata()?;
                let modified_unix_ms = metadata
                    .modified()
                    .ok()
                    .and_then(|modified| modified.duration_since(UNIX_EPOCH).ok())
                    .map(|duration| duration.as_millis().min(u128::from(u64::MAX)) as u64);
                let mut bytes = Vec::with_capacity(metadata.len() as usize);
                Read::by_ref(&mut file)
                    .take(max_bytes.saturating_add(1))
                    .read_to_end(&mut bytes)
                    .context("read bounded authenticated journal file")?;
                if bytes.len() as u64 > max_bytes {
                    anyhow::bail!("authenticated journal file exceeds its bounded size");
                }
                Ok(Some(DurableFileSnapshot {
                    bytes,
                    modified_unix_ms,
                }))
            }
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(error) => Err(error).context("inspect authenticated journal target without links"),
        }
    }

    fn open_parent(
        path: &Path,
        create: bool,
        policy: ParentDirectoryPolicy,
    ) -> Result<(File, CString)> {
        #[cfg(target_os = "macos")]
        let normalized_path = normalize_macos_system_root_alias(path);
        #[cfg(target_os = "macos")]
        let path = normalized_path.as_path();
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
                        .context("authenticated journal path contains an interior NUL")?,
                ),
                Component::ParentDir | Component::Prefix(_) | Component::RootDir => {
                    anyhow::bail!("authenticated journal path must not contain traversal");
                }
            }
        }
        let name = names
            .pop()
            .context("authenticated journal path must name a file")?;
        let start = CString::new(if absolute { "/" } else { "." })?;
        let mut directory = open_directory_at(libc::AT_FDCWD, &start)
            .context("open authenticated journal path root")?;
        for component in names {
            if create {
                let result = unsafe {
                    libc::mkdirat(
                        directory.as_raw_fd(),
                        component.as_ptr(),
                        PRIVATE_DIRECTORY_MODE as libc::mode_t,
                    )
                };
                if result != 0 {
                    let error = io::Error::last_os_error();
                    if error.kind() != io::ErrorKind::AlreadyExists {
                        return Err(error).context("create authenticated journal directory");
                    }
                }
            }
            directory = open_directory_at(directory.as_raw_fd(), &component)
                .context("open authenticated journal directory without links")?;
        }
        match policy {
            ParentDirectoryPolicy::Private => validate_private_directory(&directory)?,
            ParentDirectoryPolicy::LegacyMigration => {
                validate_legacy_migration_directory(&directory)?
            }
        }
        Ok((directory, name))
    }

    #[cfg(target_os = "macos")]
    pub(super) fn normalize_macos_system_root_alias(path: &Path) -> std::path::PathBuf {
        for (alias, canonical) in [
            ("/etc", "/private/etc"),
            ("/tmp", "/private/tmp"),
            ("/var", "/private/var"),
        ] {
            if let Ok(remainder) = path.strip_prefix(alias) {
                return Path::new(canonical).join(remainder);
            }
        }
        path.to_path_buf()
    }

    fn open_directory_at(parent_fd: libc::c_int, name: &CString) -> io::Result<File> {
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
        Ok(unsafe { File::from_raw_fd(fd) })
    }

    fn open_file_at(
        parent: &File,
        name: &CString,
        flags: libc::c_int,
        mode: u32,
    ) -> io::Result<File> {
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
        Ok(unsafe { File::from_raw_fd(fd) })
    }

    fn rename_at(parent: &File, source: &CString, target: &CString) -> Result<()> {
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
                .context("atomically publish authenticated journal file");
        }
        Ok(())
    }

    fn validate_private_directory(directory: &File) -> Result<()> {
        let metadata = directory
            .metadata()
            .context("inspect authenticated journal directory")?;
        if !metadata.file_type().is_dir()
            || metadata.uid() != effective_uid()
            || metadata.permissions().mode() & 0o7777 != PRIVATE_DIRECTORY_MODE
        {
            anyhow::bail!("authenticated journal parent must be an owned mode-0700 directory");
        }
        Ok(())
    }

    fn validate_legacy_migration_directory(directory: &File) -> Result<()> {
        let metadata = directory
            .metadata()
            .context("inspect legacy authenticated journal directory")?;
        if !metadata.file_type().is_dir()
            || metadata.uid() != effective_uid()
            || metadata.permissions().mode() & 0o7022 != 0
        {
            anyhow::bail!(
                "legacy authenticated journal parent must be owned and not writable by group or other users"
            );
        }
        Ok(())
    }

    fn validate_private_file(file: &File, max_bytes: u64) -> Result<()> {
        let metadata = file
            .metadata()
            .context("inspect authenticated journal private file")?;
        if !metadata.file_type().is_file()
            || metadata.uid() != effective_uid()
            || metadata.nlink() != 1
            || metadata.permissions().mode() & 0o7777 != PRIVATE_FILE_MODE
            || metadata.len() > max_bytes
        {
            anyhow::bail!("authenticated journal private file secure invariant failed");
        }
        Ok(())
    }

    fn lock_file(file: &File) -> Result<()> {
        if unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX) } != 0 {
            return Err(io::Error::last_os_error()).context("lock authenticated journal file");
        }
        Ok(())
    }

    fn is_not_found(error: &anyhow::Error) -> bool {
        error
            .downcast_ref::<io::Error>()
            .is_some_and(|error| error.kind() == io::ErrorKind::NotFound)
    }

    fn effective_uid() -> u32 {
        unsafe { libc::geteuid() }
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
                let _ = unsafe { libc::unlinkat(self.parent_fd, self.name.as_ptr(), 0) };
            }
        }
    }

    impl Drop for AuthenticatedJournalStoreLock {
        fn drop(&mut self) {
            let _ = unsafe { libc::flock(self.file.as_raw_fd(), libc::LOCK_UN) };
        }
    }
}

#[cfg(not(unix))]
mod platform {
    use std::path::Path;

    use anyhow::Result;

    use super::AuthenticatedJournalStoreLock;
    use super::DurableFileSnapshot;

    pub(super) fn read_private_file(
        _path: &Path,
        _max_bytes: u64,
    ) -> Result<Option<DurableFileSnapshot>> {
        anyhow::bail!("authenticated journal durable stores require Unix descriptor semantics")
    }

    pub(super) fn read_legacy_private_file(
        _path: &Path,
        _max_bytes: u64,
    ) -> Result<Option<DurableFileSnapshot>> {
        anyhow::bail!("legacy authenticated journal migrations require Unix descriptor semantics")
    }

    pub(super) fn write_private_file_atomically(
        _path: &Path,
        _bytes: &[u8],
        _max_bytes: u64,
        _staging_prefix: &str,
    ) -> Result<()> {
        anyhow::bail!("authenticated journal durable stores require Unix descriptor semantics")
    }

    pub(super) fn write_legacy_private_file_atomically(
        _path: &Path,
        _bytes: &[u8],
        _max_bytes: u64,
        _staging_prefix: &str,
    ) -> Result<()> {
        anyhow::bail!("legacy authenticated journal migrations require Unix descriptor semantics")
    }

    pub(super) fn append_private_file(_path: &Path, _bytes: &[u8], _max_bytes: u64) -> Result<()> {
        anyhow::bail!("authenticated journal durable stores require Unix descriptor semantics")
    }

    pub(super) fn lock_private_file(_path: &Path) -> Result<AuthenticatedJournalStoreLock> {
        anyhow::bail!("authenticated journal durable stores require Unix descriptor semantics")
    }

    pub(super) fn lock_legacy_private_file(_path: &Path) -> Result<AuthenticatedJournalStoreLock> {
        anyhow::bail!("legacy authenticated journal migrations require Unix descriptor semantics")
    }
}

#[cfg(all(test, unix))]
mod tests;
