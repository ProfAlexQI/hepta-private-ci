use std::collections::BTreeSet;

use hepta_contracts::ContentHash;
use hepta_contracts::OutcomeReceipt;
use serde::Deserialize;
use serde::Serialize;
use sqlx::Row;
use sqlx::Sqlite;
use sqlx::Transaction;

use super::DurableOutcomeStore;
use super::OUTCOME_ROW_SCHEMA_VERSION;
use super::OutcomeRecordWire;
use super::decode_stored_row;
use super::fetch_by_attempt;
use super::fetch_by_receipt;
use crate::contract_codec::OutcomeReceiptWire;
use crate::durable::DurableDatabase;
use crate::durable::DurableStorageError;
use crate::outcome_store::OutcomeIntent;
use crate::outcome_store::OutcomeIntentStageResult;
use crate::outcome_store::OutcomeIntentState;
use crate::outcome_store::OutcomeRecord;
use crate::outcome_store::OutcomeRecordResult;
use crate::outcome_store::OutcomeStoreError;
use crate::outcome_store::classify_existing_record;
use crate::outcome_store::map_durable_error;

const OUTCOME_INTENT_ROW_SCHEMA_VERSION: u32 = 1;
pub(crate) const STAGE_INTENT_COMMIT_OPERATION: &str = "commit outcome producer intent";
pub(crate) const ACKNOWLEDGE_INTENT_COMMIT_OPERATION: &str =
    "commit outcome producer acknowledgement";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct OutcomeIntentWire {
    schema_version: u32,
    state: String,
    record: OutcomeRecordWire,
}

impl DurableOutcomeStore {
    /// Persists exact producer material before entering the bounded commit queue.
    pub async fn stage_intent(
        &self,
        attempt_id: impl Into<String>,
        receipt: OutcomeReceipt,
        canonical_evidence: impl Into<String>,
        canonical_evidence_hash: ContentHash,
    ) -> Result<OutcomeIntentStageResult, OutcomeStoreError> {
        self.validate_database_identity()?;
        let attempted = build_record(
            attempt_id.into(),
            receipt,
            canonical_evidence.into(),
            canonical_evidence_hash,
        );
        let pending_row = encode_intent(&self.database, &attempted, OutcomeIntentState::Pending)?;
        let mut transaction = self.begin_intent_transaction().await?;

        if let Some(row) = fetch_intent_by_attempt(&mut transaction, attempted.attempt_id()).await?
        {
            let existing = decode_intent_row(&self.database, row)?;
            let outcome = exact_intent_stage(&existing, &attempted);
            return self.rollback_intent(transaction, outcome).await;
        }
        if let Some(row) =
            fetch_intent_by_receipt(&mut transaction, attempted.receipt().id().as_str()).await?
        {
            let existing = decode_intent_row(&self.database, row)?;
            let outcome =
                exact_replay(existing.record(), &attempted).map(|_| stage_result(existing.state()));
            return self.rollback_intent(transaction, outcome).await;
        }
        if let Some(row) =
            fetch_by_receipt(&mut transaction, attempted.receipt().id().as_str()).await?
        {
            let existing = decode_stored_row(&self.database, row)?;
            let outcome = exact_replay(&existing, &attempted)
                .map(|_| OutcomeIntentStageResult::AlreadyRecorded);
            return self.rollback_intent(transaction, outcome).await;
        }
        if let Some(row) = fetch_by_attempt(&mut transaction, attempted.attempt_id()).await? {
            let existing = decode_stored_row(&self.database, row)?;
            let outcome = Err(OutcomeStoreError::AttemptAlreadyFinalized {
                attempt_id: attempted.attempt_id().to_owned(),
                existing_receipt: existing.receipt().id().clone(),
                attempted_receipt: attempted.receipt().id().clone(),
            });
            return self.rollback_intent(transaction, outcome).await;
        }

        let inserted = sqlx::query(
            "INSERT INTO hepta_v2_outcome_intents (
                attempt_id,
                receipt_id,
                state,
                payload_json,
                storage_hash
             ) VALUES (?, ?, 'pending', ?, ?)",
        )
        .bind(attempted.attempt_id())
        .bind(attempted.receipt().id().as_str())
        .bind(&pending_row.payload_json)
        .bind(&pending_row.storage_hash)
        .execute(&mut *transaction)
        .await
        .map_err(|error| {
            map_durable_error(DurableStorageError::persistence(
                "insert outcome producer intent",
                error,
            ))
        })?
        .rows_affected();
        if inserted != 1 {
            return self
                .rollback_intent(
                    transaction,
                    Err(OutcomeStoreError::Corrupt {
                        detail: format!("outcome producer-intent insert affected {inserted} rows"),
                    }),
                )
                .await;
        }
        self.commit_intent_transaction(
            transaction,
            OutcomeIntentStageResult::Pending,
            STAGE_INTENT_COMMIT_OPERATION,
        )
        .await
    }

    pub(crate) async fn commit_staged_intent(
        &self,
        attempt_id: String,
        receipt: OutcomeReceipt,
        canonical_evidence: String,
        canonical_evidence_hash: ContentHash,
    ) -> Result<OutcomeRecordResult, OutcomeStoreError> {
        self.commit_staged_intent_inner(
            attempt_id,
            receipt,
            canonical_evidence,
            canonical_evidence_hash,
            None,
        )
        .await
    }

    pub(crate) async fn commit_staged_intent_and_resolve_execution(
        &self,
        attempt_id: String,
        receipt: OutcomeReceipt,
        canonical_evidence: String,
        canonical_evidence_hash: ContentHash,
        execution_idempotency_key: String,
    ) -> Result<OutcomeRecordResult, OutcomeStoreError> {
        self.commit_staged_intent_inner(
            attempt_id,
            receipt,
            canonical_evidence,
            canonical_evidence_hash,
            Some(execution_idempotency_key),
        )
        .await
    }

    async fn commit_staged_intent_inner(
        &self,
        attempt_id: String,
        receipt: OutcomeReceipt,
        canonical_evidence: String,
        canonical_evidence_hash: ContentHash,
        execution_idempotency_key: Option<String>,
    ) -> Result<OutcomeRecordResult, OutcomeStoreError> {
        self.validate_database_identity()?;
        let attempted = build_record(
            attempt_id,
            receipt,
            canonical_evidence,
            canonical_evidence_hash,
        );
        let mut transaction = self.begin_intent_transaction().await?;
        let Some(row) = fetch_intent_by_attempt(&mut transaction, attempted.attempt_id()).await?
        else {
            return self.classify_without_intent(transaction, &attempted).await;
        };
        let existing_intent = decode_intent_row(&self.database, row)?;
        exact_replay(existing_intent.record(), &attempted)?;

        if matches!(existing_intent.state(), OutcomeIntentState::Committed) {
            let outcome =
                exact_committed_record(&self.database, &mut transaction, &attempted).await?;
            return self.rollback_intent(transaction, outcome).await;
        }

        let encoded_record = encode_record(&self.database, &attempted)?;
        let inserted = sqlx::query(
            "INSERT INTO hepta_v2_outcome_records (
                receipt_id,
                attempt_id,
                payload_json,
                storage_hash
             ) VALUES (?, ?, ?, ?)
             ON CONFLICT DO NOTHING",
        )
        .bind(attempted.receipt().id().as_str())
        .bind(attempted.attempt_id())
        .bind(&encoded_record.payload_json)
        .bind(&encoded_record.storage_hash)
        .execute(&mut *transaction)
        .await
        .map_err(|error| {
            map_durable_error(DurableStorageError::persistence(
                "insert staged outcome record",
                error,
            ))
        })?
        .rows_affected();
        let result = if inserted == 1 {
            OutcomeRecordResult::Recorded
        } else {
            exact_committed_record(&self.database, &mut transaction, &attempted).await??
        };

        let committed_row =
            encode_intent(&self.database, &attempted, OutcomeIntentState::Committed)?;
        let updated = sqlx::query(
            "UPDATE hepta_v2_outcome_intents
             SET state = 'committed', payload_json = ?, storage_hash = ?
             WHERE attempt_id = ? AND state = 'pending'",
        )
        .bind(&committed_row.payload_json)
        .bind(&committed_row.storage_hash)
        .bind(attempted.attempt_id())
        .execute(&mut *transaction)
        .await
        .map_err(|error| {
            map_durable_error(DurableStorageError::persistence(
                "mark outcome producer intent committed",
                error,
            ))
        })?
        .rows_affected();
        if updated != 1 {
            return self
                .rollback_intent(
                    transaction,
                    Err(OutcomeStoreError::Corrupt {
                        detail: format!("marking outcome intent committed affected {updated} rows"),
                    }),
                )
                .await;
        }
        if let Some(idempotency_key) = execution_idempotency_key {
            super::execution_intent::resolve_execution_intent_in_transaction(
                &self.database,
                &mut transaction,
                &attempted,
                &idempotency_key,
            )
            .await?;
        }
        self.commit_intent_transaction(transaction, result, super::OUTCOME_COMMIT_OPERATION)
            .await
    }

    pub(crate) async fn acknowledge_intent(
        &self,
        attempt_id: &str,
    ) -> Result<(), OutcomeStoreError> {
        self.validate_database_identity()?;
        let mut transaction = self.begin_intent_transaction().await?;
        let Some(row) = fetch_intent_by_attempt(&mut transaction, attempt_id).await? else {
            return self.rollback_intent(transaction, Ok(())).await;
        };
        let intent = decode_intent_row(&self.database, row)?;
        if !matches!(intent.state(), OutcomeIntentState::Committed) {
            return self
                .rollback_intent(
                    transaction,
                    Err(OutcomeStoreError::Corrupt {
                        detail: format!(
                            "cannot acknowledge uncommitted outcome intent {attempt_id}"
                        ),
                    }),
                )
                .await;
        }
        exact_committed_record(&self.database, &mut transaction, intent.record()).await??;
        let deleted = sqlx::query(
            "DELETE FROM hepta_v2_outcome_intents
             WHERE attempt_id = ? AND state = 'committed'",
        )
        .bind(attempt_id)
        .execute(&mut *transaction)
        .await
        .map_err(|error| {
            map_durable_error(DurableStorageError::persistence(
                "acknowledge outcome producer intent",
                error,
            ))
        })?
        .rows_affected();
        if deleted != 1 {
            return self
                .rollback_intent(
                    transaction,
                    Err(OutcomeStoreError::Corrupt {
                        detail: format!(
                            "acknowledging outcome intent {attempt_id} affected {deleted} rows"
                        ),
                    }),
                )
                .await;
        }
        self.commit_intent_transaction(transaction, (), ACKNOWLEDGE_INTENT_COMMIT_OPERATION)
            .await
    }

    /// Returns all exact intents that still require commit or acknowledgement.
    pub async fn pending_intents(&self) -> Result<Vec<OutcomeIntent>, OutcomeStoreError> {
        self.validate_database_identity()?;
        let rows = sqlx::query(
            "SELECT attempt_id, receipt_id, state, payload_json, storage_hash
             FROM hepta_v2_outcome_intents
             ORDER BY attempt_id",
        )
        .fetch_all(self.database.pool())
        .await
        .map_err(|error| {
            map_durable_error(DurableStorageError::persistence(
                "list pending outcome producer intents",
                error,
            ))
        })?;
        self.validate_database_identity()?;
        rows.into_iter()
            .map(|row| decode_intent_row(&self.database, row))
            .collect()
    }

    /// Reads one exact unresolved intent by execution-attempt identity.
    pub async fn pending_intent(
        &self,
        attempt_id: &str,
    ) -> Result<Option<OutcomeIntent>, OutcomeStoreError> {
        self.validate_database_identity()?;
        let row = sqlx::query(
            "SELECT attempt_id, receipt_id, state, payload_json, storage_hash
             FROM hepta_v2_outcome_intents
             WHERE attempt_id = ?",
        )
        .bind(attempt_id)
        .fetch_optional(self.database.pool())
        .await
        .map_err(|error| {
            map_durable_error(DurableStorageError::persistence(
                "read pending outcome producer intent",
                error,
            ))
        })?;
        self.validate_database_identity()?;
        row.map(|row| decode_intent_row(&self.database, row))
            .transpose()
    }

    pub(super) async fn verify_intent_recovery(&self) -> Result<(), OutcomeStoreError> {
        let mut transaction = self.database.pool().begin().await.map_err(|error| {
            map_durable_error(DurableStorageError::persistence(
                "begin outcome intent recovery snapshot",
                error,
            ))
        })?;
        let rows = sqlx::query(
            "SELECT attempt_id, receipt_id, state, payload_json, storage_hash
             FROM hepta_v2_outcome_intents
             ORDER BY attempt_id",
        )
        .fetch_all(&mut *transaction)
        .await
        .map_err(|error| {
            map_durable_error(DurableStorageError::persistence(
                "scan outcome intents during recovery",
                error,
            ))
        })?;
        let mut attempts = BTreeSet::new();
        let mut receipts = BTreeSet::new();
        for row in rows {
            let intent = decode_intent_row(&self.database, row)?;
            if !attempts.insert(intent.record().attempt_id().to_owned())
                || !receipts.insert(intent.record().receipt().id().clone())
            {
                return Err(OutcomeStoreError::Corrupt {
                    detail: "duplicate recovered outcome producer intent".into(),
                });
            }
            let outcome = fetch_by_attempt(&mut transaction, intent.record().attempt_id()).await?;
            match (intent.state(), outcome) {
                (OutcomeIntentState::Pending, None) => {}
                (OutcomeIntentState::Pending, Some(_)) => {
                    return Err(OutcomeStoreError::Corrupt {
                        detail: format!(
                            "pending outcome intent {} already has a terminal record",
                            intent.record().attempt_id()
                        ),
                    });
                }
                (OutcomeIntentState::Committed, Some(row)) => {
                    exact_replay(&decode_stored_row(&self.database, row)?, intent.record())?;
                }
                (OutcomeIntentState::Committed, None) => {
                    return Err(OutcomeStoreError::Corrupt {
                        detail: format!(
                            "committed outcome intent {} lacks its terminal record",
                            intent.record().attempt_id()
                        ),
                    });
                }
            }
        }
        transaction.rollback().await.map_err(|error| {
            map_durable_error(DurableStorageError::persistence(
                "close outcome intent recovery snapshot",
                error,
            ))
        })?;
        self.validate_database_identity()
    }

    async fn begin_intent_transaction(&self) -> Result<Transaction<'_, Sqlite>, OutcomeStoreError> {
        let mut transaction = self.database.pool().begin().await.map_err(|error| {
            map_durable_error(DurableStorageError::persistence(
                "begin outcome intent transaction",
                error,
            ))
        })?;
        self.validate_database_identity()?;
        DurableDatabase::acquire_write_serialization(&mut transaction)
            .await
            .map_err(map_durable_error)?;
        Ok(transaction)
    }

    async fn classify_without_intent(
        &self,
        mut transaction: Transaction<'_, Sqlite>,
        attempted: &OutcomeRecord,
    ) -> Result<OutcomeRecordResult, OutcomeStoreError> {
        let outcome = exact_committed_record(&self.database, &mut transaction, attempted).await?;
        self.rollback_intent(transaction, outcome).await
    }

    async fn commit_intent_transaction<T>(
        &self,
        transaction: Transaction<'_, Sqlite>,
        value: T,
        operation: &'static str,
    ) -> Result<T, OutcomeStoreError> {
        self.validate_database_identity()?;
        transaction.commit().await.map_err(|error| {
            map_durable_error(DurableStorageError::persistence(operation, error))
        })?;
        self.database
            .validate_identity()
            .map_err(|error| post_commit_error(error, operation))?;
        Ok(value)
    }

    async fn rollback_intent<T>(
        &self,
        transaction: Transaction<'_, Sqlite>,
        outcome: Result<T, OutcomeStoreError>,
    ) -> Result<T, OutcomeStoreError> {
        transaction.rollback().await.map_err(|error| {
            map_durable_error(DurableStorageError::persistence(
                "rollback outcome intent transaction",
                error,
            ))
        })?;
        self.validate_database_identity()?;
        outcome
    }
}

fn build_record(
    attempt_id: String,
    receipt: OutcomeReceipt,
    canonical_evidence: String,
    canonical_evidence_hash: ContentHash,
) -> OutcomeRecord {
    OutcomeRecord {
        attempt_id,
        receipt,
        canonical_evidence,
        canonical_evidence_hash,
    }
}

fn encode_record(
    database: &DurableDatabase,
    record: &OutcomeRecord,
) -> Result<crate::durable::CanonicalRow, OutcomeStoreError> {
    let wire = OutcomeRecordWire {
        schema_version: OUTCOME_ROW_SCHEMA_VERSION,
        attempt_id: record.attempt_id().to_owned(),
        receipt: OutcomeReceiptWire::from_contract(record.receipt()).map_err(map_durable_error)?,
        canonical_evidence: record.canonical_evidence().to_owned(),
        canonical_evidence_hash: record.canonical_evidence_hash().as_str().to_owned(),
    };
    database
        .encode_canonical_row(&wire)
        .map_err(map_durable_error)
}

fn encode_intent(
    database: &DurableDatabase,
    record: &OutcomeRecord,
    state: OutcomeIntentState,
) -> Result<crate::durable::CanonicalRow, OutcomeStoreError> {
    let record_wire = OutcomeRecordWire {
        schema_version: OUTCOME_ROW_SCHEMA_VERSION,
        attempt_id: record.attempt_id().to_owned(),
        receipt: OutcomeReceiptWire::from_contract(record.receipt()).map_err(map_durable_error)?,
        canonical_evidence: record.canonical_evidence().to_owned(),
        canonical_evidence_hash: record.canonical_evidence_hash().as_str().to_owned(),
    };
    database
        .encode_canonical_row(&OutcomeIntentWire {
            schema_version: OUTCOME_INTENT_ROW_SCHEMA_VERSION,
            state: state.as_str().to_owned(),
            record: record_wire,
        })
        .map_err(map_durable_error)
}

fn decode_intent_row(
    database: &DurableDatabase,
    row: sqlx::sqlite::SqliteRow,
) -> Result<OutcomeIntent, OutcomeStoreError> {
    let attempt_id: String = decode_column(&row, "attempt_id", "intent attempt")?;
    let receipt_id: String = decode_column(&row, "receipt_id", "intent receipt")?;
    let state_column: String = decode_column(&row, "state", "intent state")?;
    let payload_json: String = decode_column(&row, "payload_json", "intent payload")?;
    let storage_hash: String = decode_column(&row, "storage_hash", "intent storage hash")?;
    let wire: OutcomeIntentWire = database
        .decode_canonical_row(&payload_json, &storage_hash, "outcome producer intent")
        .map_err(map_durable_error)?;
    if wire.schema_version != OUTCOME_INTENT_ROW_SCHEMA_VERSION {
        return Err(OutcomeStoreError::Corrupt {
            detail: format!(
                "unsupported outcome intent row schema version {}",
                wire.schema_version
            ),
        });
    }
    let state =
        OutcomeIntentState::parse(&wire.state).ok_or_else(|| OutcomeStoreError::Corrupt {
            detail: format!("unsupported outcome intent state {}", wire.state),
        })?;
    if state.as_str() != state_column {
        return Err(OutcomeStoreError::Corrupt {
            detail: format!(
                "outcome intent state column {state_column} disagrees with canonical payload {}",
                state.as_str()
            ),
        });
    }
    let record = decode_record_wire(wire.record, &attempt_id, &receipt_id)?;
    Ok(OutcomeIntent::new(state, record))
}

fn decode_record_wire(
    wire: OutcomeRecordWire,
    attempt_id: &str,
    receipt_id: &str,
) -> Result<OutcomeRecord, OutcomeStoreError> {
    if wire.schema_version != OUTCOME_ROW_SCHEMA_VERSION || wire.attempt_id != attempt_id {
        return Err(OutcomeStoreError::Corrupt {
            detail: format!("outcome intent payload disagrees with attempt {attempt_id}"),
        });
    }
    let receipt = wire.receipt.into_contract().map_err(map_durable_error)?;
    if receipt.id().as_str() != receipt_id {
        return Err(OutcomeStoreError::Corrupt {
            detail: format!(
                "outcome intent receipt column {receipt_id} disagrees with canonical payload {}",
                receipt.id()
            ),
        });
    }
    Ok(build_record(
        attempt_id.to_owned(),
        receipt,
        wire.canonical_evidence,
        ContentHash::new(wire.canonical_evidence_hash),
    ))
}

async fn exact_committed_record(
    database: &DurableDatabase,
    transaction: &mut Transaction<'_, Sqlite>,
    attempted: &OutcomeRecord,
) -> Result<Result<OutcomeRecordResult, OutcomeStoreError>, OutcomeStoreError> {
    if let Some(row) = fetch_by_receipt(transaction, attempted.receipt().id().as_str()).await? {
        let existing = decode_stored_row(database, row)?;
        return Ok(exact_replay(&existing, attempted));
    }
    if let Some(row) = fetch_by_attempt(transaction, attempted.attempt_id()).await? {
        let existing = decode_stored_row(database, row)?;
        return Ok(Err(OutcomeStoreError::AttemptAlreadyFinalized {
            attempt_id: attempted.attempt_id().to_owned(),
            existing_receipt: existing.receipt().id().clone(),
            attempted_receipt: attempted.receipt().id().clone(),
        }));
    }
    Ok(Err(OutcomeStoreError::Corrupt {
        detail: format!(
            "outcome intent {} has no matching terminal record",
            attempted.attempt_id()
        ),
    }))
}

fn exact_intent_stage(
    existing: &OutcomeIntent,
    attempted: &OutcomeRecord,
) -> Result<OutcomeIntentStageResult, OutcomeStoreError> {
    if existing.record().receipt().id() != attempted.receipt().id() {
        return Err(OutcomeStoreError::AttemptAlreadyFinalized {
            attempt_id: attempted.attempt_id().to_owned(),
            existing_receipt: existing.record().receipt().id().clone(),
            attempted_receipt: attempted.receipt().id().clone(),
        });
    }
    exact_replay(existing.record(), attempted).map(|_| stage_result(existing.state()))
}

fn exact_replay(
    existing: &OutcomeRecord,
    attempted: &OutcomeRecord,
) -> Result<OutcomeRecordResult, OutcomeStoreError> {
    classify_existing_record(
        existing,
        attempted.attempt_id().to_owned(),
        attempted.receipt().clone(),
        attempted.canonical_evidence().to_owned(),
        attempted.canonical_evidence_hash().clone(),
    )
}

const fn stage_result(state: OutcomeIntentState) -> OutcomeIntentStageResult {
    match state {
        OutcomeIntentState::Pending => OutcomeIntentStageResult::Pending,
        OutcomeIntentState::Committed => OutcomeIntentStageResult::Committed,
    }
}

async fn fetch_intent_by_attempt(
    transaction: &mut Transaction<'_, Sqlite>,
    attempt_id: &str,
) -> Result<Option<sqlx::sqlite::SqliteRow>, OutcomeStoreError> {
    fetch_intent(transaction, "attempt_id", attempt_id).await
}

async fn fetch_intent_by_receipt(
    transaction: &mut Transaction<'_, Sqlite>,
    receipt_id: &str,
) -> Result<Option<sqlx::sqlite::SqliteRow>, OutcomeStoreError> {
    fetch_intent(transaction, "receipt_id", receipt_id).await
}

async fn fetch_intent(
    transaction: &mut Transaction<'_, Sqlite>,
    column: &str,
    value: &str,
) -> Result<Option<sqlx::sqlite::SqliteRow>, OutcomeStoreError> {
    let query = format!(
        "SELECT attempt_id, receipt_id, state, payload_json, storage_hash
         FROM hepta_v2_outcome_intents
         WHERE {column} = ?"
    );
    sqlx::query(&query)
        .bind(value)
        .fetch_optional(&mut **transaction)
        .await
        .map_err(|error| {
            map_durable_error(DurableStorageError::persistence(
                "read outcome producer intent",
                error,
            ))
        })
}

fn decode_column<T>(
    row: &sqlx::sqlite::SqliteRow,
    column: &str,
    label: &'static str,
) -> Result<T, OutcomeStoreError>
where
    for<'row> T: sqlx::Decode<'row, Sqlite> + sqlx::Type<Sqlite>,
{
    row.try_get(column)
        .map_err(|error| map_durable_error(DurableStorageError::persistence(label, error)))
}

fn post_commit_error(error: DurableStorageError, operation: &'static str) -> OutcomeStoreError {
    let detail = match error {
        DurableStorageError::Persistence { operation, detail } => {
            format!("{operation}: {detail}")
        }
        DurableStorageError::Corrupt { detail } => detail,
    };
    OutcomeStoreError::Persistence { operation, detail }
}
