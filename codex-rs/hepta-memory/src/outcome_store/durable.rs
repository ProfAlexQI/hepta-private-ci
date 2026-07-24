pub(crate) mod effect_ack;
pub(crate) mod execution_intent;
pub(crate) mod intent;

use std::collections::BTreeSet;
use std::path::Path;

use hepta_contracts::ContentHash;
use hepta_contracts::OutcomeReceipt;
use hepta_contracts::ReceiptId;
use serde::Deserialize;
use serde::Serialize;
use sqlx::Row;
use sqlx::Sqlite;
use sqlx::Transaction;

use super::OutcomeRecord;
use super::OutcomeRecordResult;
use super::OutcomeStoreError;
use super::map_durable_error;
use crate::contract_codec::OutcomeReceiptWire;
use crate::durable::DurableDatabase;
use crate::durable::DurableDatabaseIdentity;
use crate::durable::DurableIntegrityContext;
use crate::durable::DurableIntegrityKey;
use crate::durable::DurableStorageError;

const OUTCOME_ROW_SCHEMA_VERSION: u32 = 1;
pub(super) const OUTCOME_COMMIT_OPERATION: &str = "commit outcome transaction";

/// Recoverable SQLite-WAL outcome receipt store.
///
/// The producer-owned `receipt_hash` and evidence hash remain opaque bindings.
/// Keyed constructors authenticate complete canonical rows with caller-owned
/// HMAC-SHA-256 authority. Compatibility constructors retain storage-owned
/// SHA-256 only and are not sufficient against a hostile same-UID writer.
/// Writes are serialized in SQLite transactions with WAL and `synchronous=FULL`.
#[derive(Clone)]
pub struct DurableOutcomeStore {
    database: DurableDatabase,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct OutcomeRecordWire {
    schema_version: u32,
    attempt_id: String,
    receipt: OutcomeReceiptWire,
    canonical_evidence: String,
    canonical_evidence_hash: String,
}

impl DurableOutcomeStore {
    /// Exclusively reserves a new path, then bootstraps a durable V2 database.
    ///
    /// This refuses every pre-existing entry. Initialization failure leaves
    /// the reserved artifact in place so later attempts remain fail-closed.
    pub async fn bootstrap_new(path: impl AsRef<Path>) -> Result<Self, OutcomeStoreError> {
        Self::bootstrap_new_with_integrity(path, DurableIntegrityContext::unkeyed()).await
    }

    /// Bootstraps a database whose canonical rows require an external key.
    pub async fn bootstrap_new_keyed(
        path: impl AsRef<Path>,
        key: DurableIntegrityKey,
    ) -> Result<Self, OutcomeStoreError> {
        Self::bootstrap_new_with_integrity(path, key.into_context()).await
    }

    pub(crate) async fn bootstrap_new_with_integrity(
        path: impl AsRef<Path>,
        integrity: DurableIntegrityContext,
    ) -> Result<Self, OutcomeStoreError> {
        let database = DurableDatabase::bootstrap_new_with_integrity(path, integrity)
            .await
            .map_err(map_durable_error)?;
        Self::recover(database).await
    }

    /// Opens an existing durable V2 database without creating or migrating it.
    pub async fn open_existing(path: impl AsRef<Path>) -> Result<Self, OutcomeStoreError> {
        Self::open_existing_with_integrity(path, DurableIntegrityContext::unkeyed()).await
    }

    /// Opens a keyed database and rejects a wrong or missing integrity key.
    pub async fn open_existing_keyed(
        path: impl AsRef<Path>,
        key: DurableIntegrityKey,
    ) -> Result<Self, OutcomeStoreError> {
        Self::open_existing_with_integrity(path, key.into_context()).await
    }

    pub(crate) async fn open_existing_with_integrity(
        path: impl AsRef<Path>,
        integrity: DurableIntegrityContext,
    ) -> Result<Self, OutcomeStoreError> {
        let database = DurableDatabase::open_existing_with_integrity(path, integrity)
            .await
            .map_err(map_durable_error)?;
        Self::recover(database).await
    }

    pub(crate) async fn open_existing_bound_with_integrity(
        path: impl AsRef<Path>,
        identity: DurableDatabaseIdentity,
        integrity: DurableIntegrityContext,
    ) -> Result<Self, OutcomeStoreError> {
        let database =
            DurableDatabase::open_existing_bound_with_integrity(path, identity, integrity)
                .await
                .map_err(map_durable_error)?;
        Self::recover(database).await
    }

    async fn recover(database: DurableDatabase) -> Result<Self, OutcomeStoreError> {
        let store = Self { database };
        store.verify_recovery().await?;
        Ok(store)
    }

    /// Returns the SQLite database path backing this store.
    pub fn path(&self) -> &Path {
        self.database.path()
    }

    pub(crate) fn database_identity(&self) -> DurableDatabaseIdentity {
        self.database.identity()
    }

    /// Atomically records one terminal outcome in SQLite WAL.
    ///
    /// Exact replay remains idempotent. Receipt, attempt, evidence, and
    /// envelope conflicts use the same typed classification as the in-memory
    /// reference implementation.
    pub async fn record(
        &self,
        attempt_id: impl Into<String>,
        receipt: OutcomeReceipt,
        canonical_evidence: impl Into<String>,
        canonical_evidence_hash: ContentHash,
    ) -> Result<OutcomeRecordResult, OutcomeStoreError> {
        let attempt_id = attempt_id.into();
        let canonical_evidence = canonical_evidence.into();
        if matches!(
            self.stage_intent(
                attempt_id.clone(),
                receipt.clone(),
                canonical_evidence.clone(),
                canonical_evidence_hash.clone(),
            )
            .await?,
            super::OutcomeIntentStageResult::AlreadyRecorded
        ) {
            return Ok(OutcomeRecordResult::AlreadyRecorded);
        }
        let result = self
            .commit_staged_intent(
                attempt_id.clone(),
                receipt,
                canonical_evidence,
                canonical_evidence_hash,
            )
            .await?;
        self.acknowledge_intent(&attempt_id).await?;
        Ok(result)
    }

    /// Reads a durable record by exact receipt identity.
    pub async fn read_by_receipt(
        &self,
        receipt: &ReceiptId,
    ) -> Result<Option<OutcomeRecord>, OutcomeStoreError> {
        self.validate_database_identity()?;
        let row = sqlx::query(
            "SELECT receipt_id, attempt_id, payload_json, storage_hash
             FROM hepta_v2_outcome_records
             WHERE receipt_id = ?",
        )
        .bind(receipt.as_str())
        .fetch_optional(self.database.pool())
        .await
        .map_err(|error| {
            map_durable_error(DurableStorageError::persistence(
                "read outcome by receipt",
                error,
            ))
        })?;
        self.validate_database_identity()?;
        row.map(|row| decode_stored_row(&self.database, row))
            .transpose()
    }

    /// Reads a durable record by execution-attempt identity.
    pub async fn read_by_attempt(
        &self,
        attempt_id: &str,
    ) -> Result<Option<OutcomeRecord>, OutcomeStoreError> {
        self.validate_database_identity()?;
        let row = sqlx::query(
            "SELECT receipt_id, attempt_id, payload_json, storage_hash
             FROM hepta_v2_outcome_records
             WHERE attempt_id = ?",
        )
        .bind(attempt_id)
        .fetch_optional(self.database.pool())
        .await
        .map_err(|error| {
            map_durable_error(DurableStorageError::persistence(
                "read outcome by attempt",
                error,
            ))
        })?;
        self.validate_database_identity()?;
        row.map(|row| decode_stored_row(&self.database, row))
            .transpose()
    }

    async fn verify_recovery(&self) -> Result<(), OutcomeStoreError> {
        self.validate_database_identity()?;
        let rows = sqlx::query(
            "SELECT receipt_id, attempt_id, payload_json, storage_hash
             FROM hepta_v2_outcome_records
             ORDER BY receipt_id",
        )
        .fetch_all(self.database.pool())
        .await
        .map_err(|error| {
            map_durable_error(DurableStorageError::persistence(
                "scan durable outcomes during recovery",
                error,
            ))
        })?;
        let mut receipt_ids = BTreeSet::new();
        let mut attempt_ids = BTreeSet::new();
        for row in rows {
            let record = decode_stored_row(&self.database, row)?;
            if !receipt_ids.insert(record.receipt.id().clone()) {
                return Err(OutcomeStoreError::Corrupt {
                    detail: format!(
                        "duplicate recovered receipt identity {}",
                        record.receipt.id()
                    ),
                });
            }
            if !attempt_ids.insert(record.attempt_id.clone()) {
                return Err(OutcomeStoreError::Corrupt {
                    detail: format!(
                        "duplicate recovered execution attempt {}",
                        record.attempt_id
                    ),
                });
            }
        }
        self.verify_intent_recovery().await?;
        self.verify_execution_intent_recovery().await?;
        self.verify_execution_effect_ack_recovery().await?;
        self.validate_database_identity()?;
        Ok(())
    }

    fn validate_database_identity(&self) -> Result<(), OutcomeStoreError> {
        self.database.validate_identity().map_err(map_durable_error)
    }
}

async fn fetch_by_receipt(
    transaction: &mut Transaction<'_, Sqlite>,
    receipt_id: &str,
) -> Result<Option<sqlx::sqlite::SqliteRow>, OutcomeStoreError> {
    sqlx::query(
        "SELECT receipt_id, attempt_id, payload_json, storage_hash
         FROM hepta_v2_outcome_records
         WHERE receipt_id = ?",
    )
    .bind(receipt_id)
    .fetch_optional(&mut **transaction)
    .await
    .map_err(|error| {
        map_durable_error(DurableStorageError::persistence(
            "read conflicting outcome receipt",
            error,
        ))
    })
}

async fn fetch_by_attempt(
    transaction: &mut Transaction<'_, Sqlite>,
    attempt_id: &str,
) -> Result<Option<sqlx::sqlite::SqliteRow>, OutcomeStoreError> {
    sqlx::query(
        "SELECT receipt_id, attempt_id, payload_json, storage_hash
         FROM hepta_v2_outcome_records
         WHERE attempt_id = ?",
    )
    .bind(attempt_id)
    .fetch_optional(&mut **transaction)
    .await
    .map_err(|error| {
        map_durable_error(DurableStorageError::persistence(
            "read conflicting outcome attempt",
            error,
        ))
    })
}

fn decode_stored_row(
    database: &DurableDatabase,
    row: sqlx::sqlite::SqliteRow,
) -> Result<OutcomeRecord, OutcomeStoreError> {
    let receipt_id: String = row.try_get("receipt_id").map_err(|error| {
        map_durable_error(DurableStorageError::persistence(
            "decode outcome receipt column",
            error,
        ))
    })?;
    let attempt_id: String = row.try_get("attempt_id").map_err(|error| {
        map_durable_error(DurableStorageError::persistence(
            "decode outcome attempt column",
            error,
        ))
    })?;
    let payload_json: String = row.try_get("payload_json").map_err(|error| {
        map_durable_error(DurableStorageError::persistence(
            "decode outcome payload column",
            error,
        ))
    })?;
    let storage_hash: String = row.try_get("storage_hash").map_err(|error| {
        map_durable_error(DurableStorageError::persistence(
            "decode outcome storage hash column",
            error,
        ))
    })?;
    let wire: OutcomeRecordWire = database
        .decode_canonical_row(&payload_json, &storage_hash, "outcome record")
        .map_err(map_durable_error)?;
    if wire.schema_version != OUTCOME_ROW_SCHEMA_VERSION {
        return Err(OutcomeStoreError::Corrupt {
            detail: format!(
                "unsupported outcome row schema version {}",
                wire.schema_version
            ),
        });
    }
    if wire.attempt_id != attempt_id {
        return Err(OutcomeStoreError::Corrupt {
            detail: format!(
                "outcome row attempt column {attempt_id} disagrees with canonical payload {}",
                wire.attempt_id
            ),
        });
    }
    let receipt = wire.receipt.into_contract().map_err(map_durable_error)?;
    if receipt.id().as_str() != receipt_id {
        return Err(OutcomeStoreError::Corrupt {
            detail: format!(
                "outcome row receipt column {receipt_id} disagrees with canonical payload {}",
                receipt.id()
            ),
        });
    }
    Ok(OutcomeRecord {
        attempt_id,
        receipt,
        canonical_evidence: wire.canonical_evidence,
        canonical_evidence_hash: ContentHash::new(wire.canonical_evidence_hash),
    })
}
