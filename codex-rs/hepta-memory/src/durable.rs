//! Shared SQLite WAL and canonical-row integrity support for V2 memory stores.

mod integrity;
mod opening;
mod schema;

use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use same_file::Handle;
use serde::Serialize;
use serde::de::DeserializeOwned;
use sqlx::SqlitePool;

pub(crate) use integrity::DurableIntegrityContext;
pub use integrity::DurableIntegrityKey;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum DurableStorageError {
    Persistence {
        operation: &'static str,
        detail: String,
    },
    Corrupt {
        detail: String,
    },
}

impl DurableStorageError {
    pub(crate) fn persistence(operation: &'static str, error: impl std::fmt::Display) -> Self {
        Self::Persistence {
            operation,
            detail: error.to_string(),
        }
    }

    pub(crate) fn corrupt(detail: impl Into<String>) -> Self {
        Self::Corrupt {
            detail: detail.into(),
        }
    }
}

#[derive(Clone)]
pub(crate) struct DurableDatabase {
    pool: SqlitePool,
    path: Arc<PathBuf>,
    identity: DurableDatabaseIdentity,
    integrity: DurableIntegrityContext,
}

#[derive(Clone, Debug)]
pub(crate) struct DurableDatabaseIdentity(Arc<Handle>);

impl DurableDatabaseIdentity {
    fn new(handle: Handle) -> Self {
        Self(Arc::new(handle))
    }

    fn matches(&self, handle: &Handle) -> bool {
        self.0.as_ref() == handle
    }
}

impl DurableDatabase {
    pub(crate) fn pool(&self) -> &SqlitePool {
        &self.pool
    }

    pub(crate) fn path(&self) -> &Path {
        self.path.as_path()
    }

    pub(crate) fn identity(&self) -> DurableDatabaseIdentity {
        self.identity.clone()
    }

    pub(crate) fn encode_canonical_row<T: Serialize>(
        &self,
        value: &T,
    ) -> Result<CanonicalRow, DurableStorageError> {
        let payload_json = serde_json::to_string(value).map_err(|error| {
            DurableStorageError::persistence("encode canonical durable row", error)
        })?;
        let storage_hash = self.integrity.protect(&payload_json)?;
        Ok(CanonicalRow {
            payload_json,
            storage_hash,
        })
    }

    pub(crate) fn decode_canonical_row<T: DeserializeOwned>(
        &self,
        payload_json: &str,
        expected_storage_hash: &str,
        row_kind: &str,
    ) -> Result<T, DurableStorageError> {
        self.integrity
            .verify(payload_json, expected_storage_hash, row_kind)?;
        serde_json::from_str(payload_json).map_err(|error| {
            DurableStorageError::corrupt(format!(
                "{row_kind} canonical payload is invalid: {error}"
            ))
        })
    }

    pub(crate) async fn acquire_write_serialization(
        transaction: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    ) -> Result<(), DurableStorageError> {
        let updated = sqlx::query(
            "UPDATE hepta_v2_write_lock
             SET generation = generation + 1
             WHERE singleton = 1",
        )
        .execute(&mut **transaction)
        .await
        .map_err(|error| {
            DurableStorageError::persistence("acquire durable V2 write serialization", error)
        })?
        .rows_affected();
        if updated != 1 {
            return Err(DurableStorageError::corrupt(format!(
                "durable V2 write-lock update affected {updated} rows"
            )));
        }
        Ok(())
    }
}

pub(crate) struct CanonicalRow {
    pub(crate) payload_json: String,
    pub(crate) storage_hash: String,
}
