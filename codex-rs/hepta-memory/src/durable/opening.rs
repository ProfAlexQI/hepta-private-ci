//! Explicit filesystem-safe durable database opening modes.

use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use sqlx::SqlitePool;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::sqlite::SqliteJournalMode;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::sqlite::SqliteSynchronous;

use super::DurableDatabase;
use super::DurableDatabaseIdentity;
use super::DurableIntegrityContext;
use super::DurableIntegrityKey;
use super::DurableStorageError;

mod filesystem;

use filesystem::bind_existing_file;
use filesystem::harden_sqlite_sidecars;
use filesystem::prepare_bootstrap_parent;
use filesystem::reject_bootstrap_sidecars;
use filesystem::reserve_new_database_file;
use filesystem::validate_database_parent;
use filesystem::validate_existing_sidecars;

impl DurableDatabase {
    /// Exclusively reserves a new path, then initializes its durable database.
    ///
    /// Initialization failure deliberately leaves the reserved artifact so a
    /// retry cannot silently adopt or overwrite uncertain storage.
    pub(crate) async fn bootstrap_new(path: impl AsRef<Path>) -> Result<Self, DurableStorageError> {
        Self::bootstrap_new_with_integrity(path, DurableIntegrityContext::unkeyed()).await
    }

    pub(crate) async fn bootstrap_new_keyed(
        path: impl AsRef<Path>,
        key: DurableIntegrityKey,
    ) -> Result<Self, DurableStorageError> {
        Self::bootstrap_new_with_integrity(path, key.into_context()).await
    }

    pub(crate) async fn bootstrap_new_with_integrity(
        path: impl AsRef<Path>,
        integrity: DurableIntegrityContext,
    ) -> Result<Self, DurableStorageError> {
        let path = path.as_ref().to_path_buf();
        validate_database_path(&path)?;
        prepare_bootstrap_parent(&path)?;
        let identity = reserve_new_database_file(&path)?;
        reject_bootstrap_sidecars(&path)?;
        let pool = connect_pool(&path, false).await?;
        harden_sqlite_sidecars(&path)?;
        let database = Self {
            pool,
            path: Arc::new(path),
            identity: DurableDatabaseIdentity::new(identity),
            integrity,
        };
        database.validate_identity()?;
        database.initialize_schema().await?;
        harden_sqlite_sidecars(database.path())?;
        database.verify_durability().await?;
        harden_sqlite_sidecars(database.path())?;
        database.validate_identity()?;
        Ok(database)
    }

    /// Opens an initialized database without creating any filesystem entry or
    /// running schema creation/migration statements.
    pub(crate) async fn open_existing(path: impl AsRef<Path>) -> Result<Self, DurableStorageError> {
        Self::open_existing_with_integrity(path, DurableIntegrityContext::unkeyed()).await
    }

    pub(crate) async fn open_existing_keyed(
        path: impl AsRef<Path>,
        key: DurableIntegrityKey,
    ) -> Result<Self, DurableStorageError> {
        Self::open_existing_with_integrity(path, key.into_context()).await
    }

    pub(crate) async fn open_existing_with_integrity(
        path: impl AsRef<Path>,
        integrity: DurableIntegrityContext,
    ) -> Result<Self, DurableStorageError> {
        let path = path.as_ref().to_path_buf();
        validate_database_path(&path)?;
        validate_database_parent(&path)?;
        validate_existing_sidecars(&path)?;
        let identity = DurableDatabaseIdentity::new(bind_existing_file(&path)?);
        Self::open_existing_bound_with_integrity(path, identity, integrity).await
    }

    pub(crate) async fn open_existing_bound_with_integrity(
        path: impl AsRef<Path>,
        identity: DurableDatabaseIdentity,
        integrity: DurableIntegrityContext,
    ) -> Result<Self, DurableStorageError> {
        let path = path.as_ref().to_path_buf();
        validate_database_path(&path)?;
        validate_database_parent(&path)?;
        validate_existing_sidecars(&path)?;
        let current = bind_existing_file(&path)?;
        if !identity.matches(&current) {
            return Err(DurableStorageError::corrupt(format!(
                "durable database path was deleted or replaced: {}",
                path.display()
            )));
        }
        let pool = connect_pool(&path, false).await?;
        harden_sqlite_sidecars(&path)?;
        let database = Self {
            pool,
            path: Arc::new(path),
            identity,
            integrity,
        };
        database.validate_identity()?;
        database.verify_schema().await?;
        database.verify_durability().await?;
        database.validate_identity()?;
        Ok(database)
    }

    pub(crate) fn validate_identity(&self) -> Result<(), DurableStorageError> {
        validate_database_parent(self.path())?;
        let current = bind_existing_file(self.path()).map_err(|error| {
            DurableStorageError::corrupt(format!(
                "durable database path was deleted or replaced: {} ({})",
                self.path().display(),
                durable_error_detail(error),
            ))
        })?;
        if !self.identity.matches(&current) {
            return Err(DurableStorageError::corrupt(format!(
                "durable database path was deleted or replaced: {}",
                self.path().display()
            )));
        }
        validate_existing_sidecars(self.path())?;
        Ok(())
    }
}

fn durable_error_detail(error: DurableStorageError) -> String {
    match error {
        DurableStorageError::Persistence { operation, detail } => {
            format!("{operation}: {detail}")
        }
        DurableStorageError::Corrupt { detail } => detail,
    }
}

async fn connect_pool(
    path: &Path,
    create_if_missing: bool,
) -> Result<SqlitePool, DurableStorageError> {
    let options = SqliteConnectOptions::new()
        .filename(path)
        .create_if_missing(create_if_missing)
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Full)
        .foreign_keys(true)
        .busy_timeout(Duration::from_secs(5));
    SqlitePoolOptions::new()
        .max_connections(4)
        .connect_with(options)
        .await
        .map_err(|error| DurableStorageError::persistence("open SQLite WAL database", error))
}

fn validate_database_path(path: &Path) -> Result<(), DurableStorageError> {
    if path.as_os_str().is_empty() || path.file_name().is_none() {
        return Err(DurableStorageError::corrupt(
            "durable database path must name a file",
        ));
    }
    Ok(())
}
