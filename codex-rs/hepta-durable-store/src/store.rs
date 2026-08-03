use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;
use anyhow::Result;

use super::platform;

#[derive(Debug, Clone)]
pub struct AuthenticatedJournalStore {
    path: PathBuf,
    lock_path: PathBuf,
    max_bytes: u64,
    staging_prefix: String,
}

/// Narrow compatibility store for moving an existing private journal out of a
/// user-owned legacy directory that was not required to be mode 0700.
///
/// This type intentionally does not expose append or update operations. New
/// journal traffic must use [`AuthenticatedJournalStore`]; this store exists
/// only so a migration can lock, authenticate, copy, and tombstone legacy
/// files in a directory that is owned by the current user and is not writable
/// by group or other users.
#[derive(Debug, Clone)]
pub struct LegacyJournalMigrationStore {
    inner: AuthenticatedJournalStore,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DurableFileSnapshot {
    pub bytes: Vec<u8>,
    pub modified_unix_ms: Option<u64>,
}

#[cfg(unix)]
#[derive(Debug)]
pub struct AuthenticatedJournalStoreLock {
    pub(super) file: std::fs::File,
}

#[cfg(not(unix))]
#[derive(Debug)]
pub struct AuthenticatedJournalStoreLock;

impl AuthenticatedJournalStore {
    pub fn new(
        path: impl Into<PathBuf>,
        max_bytes: u64,
        staging_prefix: impl Into<String>,
    ) -> Result<Self> {
        let path = path.into();
        let file_name = path
            .file_name()
            .context("authenticated journal path must name a file")?;
        let mut lock_name = file_name.to_os_string();
        lock_name.push(".lock");
        let lock_path = path.with_file_name(lock_name);
        let store = Self {
            path,
            lock_path,
            max_bytes,
            staging_prefix: staging_prefix.into(),
        };
        store.validate()?;
        Ok(store)
    }

    pub fn with_lock_path(mut self, lock_path: impl Into<PathBuf>) -> Result<Self> {
        self.lock_path = lock_path.into();
        self.validate()?;
        Ok(self)
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn read(&self) -> Result<Option<Vec<u8>>> {
        self.read_snapshot()
            .map(|snapshot| snapshot.map(|snapshot| snapshot.bytes))
    }

    pub fn read_snapshot(&self) -> Result<Option<DurableFileSnapshot>> {
        platform::read_private_file(&self.path, self.max_bytes)
    }

    pub fn publish(&self, bytes: &[u8]) -> Result<()> {
        self.read_snapshot()?;
        platform::write_private_file_atomically(
            &self.path,
            bytes,
            self.max_bytes,
            &self.staging_prefix,
        )
    }

    pub fn append(&self, bytes: &[u8]) -> Result<()> {
        platform::append_private_file(&self.path, bytes, self.max_bytes)
    }

    pub fn update<T>(
        &self,
        update: impl FnOnce(Option<&[u8]>) -> Result<(Vec<u8>, T)>,
    ) -> Result<T> {
        let _lock = self.lock()?;
        let current = self.read()?;
        let (bytes, output) = update(current.as_deref())?;
        self.publish(&bytes)?;
        Ok(output)
    }

    pub fn lock(&self) -> Result<AuthenticatedJournalStoreLock> {
        platform::lock_private_file(&self.lock_path)
    }

    fn validate(&self) -> Result<()> {
        if self.max_bytes == 0 {
            anyhow::bail!("authenticated journal byte limit must be positive");
        }
        let target_parent = self
            .path
            .parent()
            .context("authenticated journal path has no parent")?;
        if self.lock_path.parent() != Some(target_parent) {
            anyhow::bail!("authenticated journal lock must share the target parent");
        }
        let prefix = self
            .staging_prefix
            .strip_prefix('.')
            .unwrap_or(&self.staging_prefix);
        if prefix.is_empty()
            || !prefix
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
        {
            anyhow::bail!("authenticated journal staging prefix is invalid");
        }
        Ok(())
    }
}

impl LegacyJournalMigrationStore {
    pub fn new(
        path: impl Into<PathBuf>,
        max_bytes: u64,
        staging_prefix: impl Into<String>,
    ) -> Result<Self> {
        Ok(Self {
            inner: AuthenticatedJournalStore::new(path, max_bytes, staging_prefix)?,
        })
    }

    pub fn with_lock_path(mut self, lock_path: impl Into<PathBuf>) -> Result<Self> {
        self.inner = self.inner.with_lock_path(lock_path)?;
        Ok(self)
    }

    pub fn path(&self) -> &Path {
        self.inner.path()
    }

    pub fn read(&self) -> Result<Option<Vec<u8>>> {
        platform::read_legacy_private_file(&self.inner.path, self.inner.max_bytes)
            .map(|snapshot| snapshot.map(|snapshot| snapshot.bytes))
    }

    pub fn publish(&self, bytes: &[u8]) -> Result<()> {
        platform::read_legacy_private_file(&self.inner.path, self.inner.max_bytes)?;
        platform::write_legacy_private_file_atomically(
            &self.inner.path,
            bytes,
            self.inner.max_bytes,
            &self.inner.staging_prefix,
        )
    }

    pub fn lock(&self) -> Result<AuthenticatedJournalStoreLock> {
        platform::lock_legacy_private_file(&self.inner.lock_path)
    }
}
