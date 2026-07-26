//! Shared SQLite WAL and canonical-row integrity support for V2 memory stores.

mod integrity;
mod opening;
mod schema;

use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use same_file::Handle;
use serde::Deserialize;
use serde::Serialize;
use serde::de::DeserializeOwned;
use sha2::Digest;
use sha2::Sha256;
use sqlx::Row;
use sqlx::SqlitePool;

pub(crate) use integrity::DurableIntegrityContext;
pub use integrity::DurableIntegrityKey;

const MONOTONIC_STATE_ROW_LIMIT: usize = 100_000;

/// Bounded authenticated projection used by an external monotonic anchor.
///
/// The generation advances inside every committed durable write transaction.
/// `state_hash` covers every authenticated row at that generation so equal
/// generations cannot silently name divergent database states.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurableMonotonicState {
    generation: u64,
    state_hash: String,
}

impl DurableMonotonicState {
    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn state_hash(&self) -> &str {
        &self.state_hash
    }
}

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

    pub(crate) async fn monotonic_state(
        &self,
    ) -> Result<DurableMonotonicState, DurableStorageError> {
        self.validate_identity()?;
        let mut transaction = self.pool.begin().await.map_err(|error| {
            DurableStorageError::persistence("begin durable monotonic snapshot", error)
        })?;
        let generation = sqlx::query_scalar::<_, i64>(
            "SELECT generation FROM hepta_v2_write_lock WHERE singleton = 1",
        )
        .fetch_one(&mut *transaction)
        .await
        .map_err(|error| {
            DurableStorageError::persistence("read durable monotonic generation", error)
        })?;
        let generation = u64::try_from(generation).map_err(|_| {
            DurableStorageError::corrupt("durable monotonic generation is negative")
        })?;
        let rows = sqlx::query(
            "SELECT table_name, key_a, key_b, key_c, row_state, storage_hash
             FROM (
               SELECT 'execution_effect_acks' AS table_name,
                      attempt_id AS key_a, idempotency_key AS key_b,
                      effect_plan_hash AS key_c, '' AS row_state, storage_hash
               FROM hepta_v2_execution_effect_acks
               UNION ALL
               SELECT 'execution_intents', attempt_id, idempotency_key, '', '', storage_hash
               FROM hepta_v2_execution_intents
               UNION ALL
               SELECT 'outcome_intents', attempt_id, receipt_id, '', state, storage_hash
               FROM hepta_v2_outcome_intents
               UNION ALL
               SELECT 'outcome_records', receipt_id, attempt_id, '', '', storage_hash
               FROM hepta_v2_outcome_records
               UNION ALL
               SELECT 'preference_genesis', preference_id, subject_id, '', '', storage_hash
               FROM hepta_v2_preference_genesis
               UNION ALL
               SELECT 'preference_heads', preference_id, subject_id, '', '', storage_hash
               FROM hepta_v2_preference_heads
               UNION ALL
               SELECT 'preference_transitions', CAST(sequence AS TEXT), transition_id,
                      evidence_id || ':' || receipt_id, '', storage_hash
               FROM hepta_v2_preference_transitions
             )
             ORDER BY table_name, key_a, key_b, key_c
             LIMIT 100001",
        )
        .fetch_all(&mut *transaction)
        .await
        .map_err(|error| {
            DurableStorageError::persistence("read bounded durable monotonic state", error)
        })?;
        if rows.len() > MONOTONIC_STATE_ROW_LIMIT {
            return Err(DurableStorageError::corrupt(format!(
                "durable monotonic state exceeds {MONOTONIC_STATE_ROW_LIMIT} rows"
            )));
        }
        let mut hasher = Sha256::new();
        update_monotonic_frame(&mut hasher, b"hepta.memory.durable-monotonic-state.v1");
        update_monotonic_frame(&mut hasher, &generation.to_be_bytes());
        for row in rows {
            for column in [
                "table_name",
                "key_a",
                "key_b",
                "key_c",
                "row_state",
                "storage_hash",
            ] {
                let value: String = row.try_get(column).map_err(|error| {
                    DurableStorageError::corrupt(format!(
                        "durable monotonic state column {column} is invalid: {error}"
                    ))
                })?;
                update_monotonic_frame(&mut hasher, value.as_bytes());
            }
        }
        self.validate_identity()?;
        Ok(DurableMonotonicState {
            generation,
            state_hash: format!("sha256:{:x}", hasher.finalize()),
        })
    }
}

fn update_monotonic_frame(hasher: &mut Sha256, value: &[u8]) {
    hasher.update((value.len() as u64).to_be_bytes());
    hasher.update(value);
}

pub(crate) struct CanonicalRow {
    pub(crate) payload_json: String,
    pub(crate) storage_hash: String,
}
