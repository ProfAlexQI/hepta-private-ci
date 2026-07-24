use std::collections::BTreeSet;

use hepta_contracts::ContentHash;
use serde::Deserialize;
use serde::Serialize;
use sqlx::Row;
use sqlx::Sqlite;
use sqlx::Transaction;

use super::DurableOutcomeStore;
use super::decode_stored_row;
use super::fetch_by_attempt;
use crate::durable::DurableDatabase;
use crate::durable::DurableStorageError;
use crate::outcome_store::ExecutionEffectAck;
use crate::outcome_store::ExecutionIntent;
use crate::outcome_store::ExecutionIntentParts;
use crate::outcome_store::ExecutionIntentResolveResult;
use crate::outcome_store::ExecutionIntentStageResult;
use crate::outcome_store::OutcomeRecord;
use crate::outcome_store::OutcomeStoreError;
use crate::outcome_store::map_durable_error;

const EXECUTION_INTENT_ROW_SCHEMA_VERSION: u32 = 4;
pub(crate) const STAGE_EXECUTION_INTENT_COMMIT_OPERATION: &str =
    "commit pre-dispatch execution intent";
pub(crate) const RESOLVE_EXECUTION_INTENT_COMMIT_OPERATION: &str =
    "commit execution-intent resolution";

mod terminal_evidence;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ExecutionIntentWire {
    schema_version: u32,
    attempt_id: String,
    session_id: String,
    correlation_id: String,
    tool_name: String,
    payload_hash: String,
    candidate_hash: String,
    candidate_reference_hash: String,
    kernel_candidate_hash: String,
    payload_set_hash: String,
    capability_id: String,
    capability_revision: u64,
    capability_provider: String,
    capability_operation: String,
    capability_manifest_hash: String,
    executor_principal: String,
    authorization_digest: String,
    admission_id: String,
    admission_revision: u64,
    admission_digest: String,
    canonical_resource_summary: String,
    resource_summary_hash: String,
    canonical_effect_plan: Option<String>,
    effect_plan_hash: Option<String>,
    idempotency_key: String,
}

impl DurableOutcomeStore {
    /// Persists one exact execution plan before provider invocation.
    pub async fn stage_execution_intent(
        &self,
        intent: ExecutionIntent,
    ) -> Result<ExecutionIntentStageResult, OutcomeStoreError> {
        self.validate_database_identity()?;
        let encoded = encode_execution_intent(&self.database, &intent)?;
        let mut transaction = self.begin_execution_intent_transaction().await?;

        if let Some(row) =
            fetch_execution_intent_by_attempt(&mut transaction, intent.attempt_id()).await?
        {
            let existing = decode_execution_intent_row(&self.database, row)?;
            let result = if existing == intent {
                Ok(ExecutionIntentStageResult::AlreadyStaged)
            } else {
                Err(OutcomeStoreError::ExecutionIntentConflict {
                    attempt_id: intent.attempt_id().to_owned(),
                })
            };
            return self
                .rollback_execution_intent_transaction(transaction, result)
                .await;
        }
        if let Some(row) =
            fetch_execution_intent_by_idempotency(&mut transaction, intent.idempotency_key())
                .await?
        {
            let existing = decode_execution_intent_row(&self.database, row)?;
            let result = Err(OutcomeStoreError::ExecutionIdempotencyConflict {
                idempotency_key: intent.idempotency_key().to_owned(),
                existing_attempt: existing.attempt_id().to_owned(),
                attempted_attempt: intent.attempt_id().to_owned(),
            });
            return self
                .rollback_execution_intent_transaction(transaction, result)
                .await;
        }
        if let Some(existing_attempt) = first_execution_intent_attempt(&mut transaction).await? {
            let result = Err(OutcomeStoreError::ExecutionIntentOutstanding {
                existing_attempt,
                attempted_attempt: intent.attempt_id().to_owned(),
            });
            return self
                .rollback_execution_intent_transaction(transaction, result)
                .await;
        }
        if fetch_by_attempt(&mut transaction, intent.attempt_id())
            .await?
            .is_some()
        {
            let result = Err(OutcomeStoreError::ExecutionIntentAfterFinalization {
                attempt_id: intent.attempt_id().to_owned(),
            });
            return self
                .rollback_execution_intent_transaction(transaction, result)
                .await;
        }

        let inserted = sqlx::query(
            "INSERT INTO hepta_v2_execution_intents (
                attempt_id,
                idempotency_key,
                payload_json,
                storage_hash
             ) VALUES (?, ?, ?, ?)",
        )
        .bind(intent.attempt_id())
        .bind(intent.idempotency_key())
        .bind(&encoded.payload_json)
        .bind(&encoded.storage_hash)
        .execute(&mut *transaction)
        .await
        .map_err(|error| {
            map_durable_error(DurableStorageError::persistence(
                "insert pre-dispatch execution intent",
                error,
            ))
        })?
        .rows_affected();
        if inserted != 1 {
            return self
                .rollback_execution_intent_transaction(
                    transaction,
                    Err(OutcomeStoreError::Corrupt {
                        detail: format!(
                            "pre-dispatch execution-intent insert affected {inserted} rows"
                        ),
                    }),
                )
                .await;
        }
        self.commit_execution_intent_transaction(
            transaction,
            ExecutionIntentStageResult::Staged,
            STAGE_EXECUTION_INTENT_COMMIT_OPERATION,
        )
        .await
    }

    /// Returns every unresolved pre-dispatch intent in deterministic order.
    pub async fn pending_execution_intents(
        &self,
    ) -> Result<Vec<ExecutionIntent>, OutcomeStoreError> {
        self.validate_database_identity()?;
        let rows = sqlx::query(
            "SELECT attempt_id, idempotency_key, payload_json, storage_hash
             FROM hepta_v2_execution_intents
             ORDER BY attempt_id",
        )
        .fetch_all(self.database.pool())
        .await
        .map_err(|error| {
            map_durable_error(DurableStorageError::persistence(
                "list pre-dispatch execution intents",
                error,
            ))
        })?;
        self.validate_database_identity()?;
        rows.into_iter()
            .map(|row| decode_execution_intent_row(&self.database, row))
            .collect()
    }

    /// Reads one unresolved pre-dispatch intent by exact attempt identity.
    pub async fn pending_execution_intent(
        &self,
        attempt_id: &str,
    ) -> Result<Option<ExecutionIntent>, OutcomeStoreError> {
        self.validate_database_identity()?;
        let row = sqlx::query(
            "SELECT attempt_id, idempotency_key, payload_json, storage_hash
             FROM hepta_v2_execution_intents
             WHERE attempt_id = ?",
        )
        .bind(attempt_id)
        .fetch_optional(self.database.pool())
        .await
        .map_err(|error| {
            map_durable_error(DurableStorageError::persistence(
                "read pre-dispatch execution intent",
                error,
            ))
        })?;
        self.validate_database_identity()?;
        row.map(|row| decode_execution_intent_row(&self.database, row))
            .transpose()
    }

    /// Resolves one plan only after its exact terminal record is durable.
    pub async fn resolve_execution_intent(
        &self,
        attempt_id: &str,
        idempotency_key: &str,
    ) -> Result<ExecutionIntentResolveResult, OutcomeStoreError> {
        self.validate_database_identity()?;
        let mut transaction = self.begin_execution_intent_transaction().await?;
        let Some(row) = fetch_execution_intent_by_attempt(&mut transaction, attempt_id).await?
        else {
            let result = if fetch_by_attempt(&mut transaction, attempt_id)
                .await?
                .is_some()
            {
                Ok(ExecutionIntentResolveResult::AlreadyResolved)
            } else {
                Err(OutcomeStoreError::ExecutionIntentOutcomeMissing {
                    attempt_id: attempt_id.to_owned(),
                })
            };
            return self
                .rollback_execution_intent_transaction(transaction, result)
                .await;
        };
        let intent = decode_execution_intent_row(&self.database, row)?;
        if intent.idempotency_key() != idempotency_key {
            let result = Err(OutcomeStoreError::ExecutionIntentConflict {
                attempt_id: attempt_id.to_owned(),
            });
            return self
                .rollback_execution_intent_transaction(transaction, result)
                .await;
        }
        let Some(outcome_row) = fetch_by_attempt(&mut transaction, attempt_id).await? else {
            let result = Err(OutcomeStoreError::ExecutionIntentOutcomeMissing {
                attempt_id: attempt_id.to_owned(),
            });
            return self
                .rollback_execution_intent_transaction(transaction, result)
                .await;
        };
        let effect_ack = super::effect_ack::execution_effect_ack_for_intent(
            &self.database,
            &mut transaction,
            &intent,
        )
        .await?;
        validate_terminal_binding(
            &intent,
            &decode_stored_row(&self.database, outcome_row)?,
            effect_ack.as_ref(),
        )?;

        let deleted = sqlx::query(
            "DELETE FROM hepta_v2_execution_intents
             WHERE attempt_id = ? AND idempotency_key = ?",
        )
        .bind(attempt_id)
        .bind(idempotency_key)
        .execute(&mut *transaction)
        .await
        .map_err(|error| {
            map_durable_error(DurableStorageError::persistence(
                "resolve pre-dispatch execution intent",
                error,
            ))
        })?
        .rows_affected();
        if deleted != 1 {
            return self
                .rollback_execution_intent_transaction(
                    transaction,
                    Err(OutcomeStoreError::Corrupt {
                        detail: format!(
                            "resolving execution intent {attempt_id} affected {deleted} rows"
                        ),
                    }),
                )
                .await;
        }
        self.commit_execution_intent_transaction(
            transaction,
            ExecutionIntentResolveResult::Resolved,
            RESOLVE_EXECUTION_INTENT_COMMIT_OPERATION,
        )
        .await
    }

    pub(super) async fn verify_execution_intent_recovery(&self) -> Result<(), OutcomeStoreError> {
        let mut transaction = self.database.pool().begin().await.map_err(|error| {
            map_durable_error(DurableStorageError::persistence(
                "begin execution-intent recovery snapshot",
                error,
            ))
        })?;
        let rows = sqlx::query(
            "SELECT attempt_id, idempotency_key, payload_json, storage_hash
             FROM hepta_v2_execution_intents
             ORDER BY attempt_id",
        )
        .fetch_all(&mut *transaction)
        .await
        .map_err(|error| {
            map_durable_error(DurableStorageError::persistence(
                "scan execution intents during recovery",
                error,
            ))
        })?;
        let mut attempts = BTreeSet::new();
        let mut idempotency_keys = BTreeSet::new();
        for row in rows {
            let intent = decode_execution_intent_row(&self.database, row)?;
            if !attempts.insert(intent.attempt_id().to_owned())
                || !idempotency_keys.insert(intent.idempotency_key().to_owned())
            {
                return Err(OutcomeStoreError::Corrupt {
                    detail: "duplicate recovered pre-dispatch execution intent".into(),
                });
            }
            if let Some(outcome) = fetch_by_attempt(&mut transaction, intent.attempt_id()).await? {
                let effect_ack = super::effect_ack::execution_effect_ack_for_intent(
                    &self.database,
                    &mut transaction,
                    &intent,
                )
                .await?;
                validate_terminal_binding(
                    &intent,
                    &decode_stored_row(&self.database, outcome)?,
                    effect_ack.as_ref(),
                )?;
            }
        }
        if attempts.len() > 1 {
            return Err(OutcomeStoreError::Corrupt {
                detail: "multiple unresolved pre-dispatch execution intents recovered".into(),
            });
        }
        transaction.rollback().await.map_err(|error| {
            map_durable_error(DurableStorageError::persistence(
                "close execution-intent recovery snapshot",
                error,
            ))
        })?;
        self.validate_database_identity()
    }

    async fn begin_execution_intent_transaction(
        &self,
    ) -> Result<Transaction<'_, Sqlite>, OutcomeStoreError> {
        let mut transaction = self.database.pool().begin().await.map_err(|error| {
            map_durable_error(DurableStorageError::persistence(
                "begin execution-intent transaction",
                error,
            ))
        })?;
        self.validate_database_identity()?;
        DurableDatabase::acquire_write_serialization(&mut transaction)
            .await
            .map_err(map_durable_error)?;
        Ok(transaction)
    }

    async fn commit_execution_intent_transaction<T>(
        &self,
        transaction: Transaction<'_, Sqlite>,
        value: T,
        operation: &'static str,
    ) -> Result<T, OutcomeStoreError> {
        self.validate_database_identity()?;
        transaction.commit().await.map_err(|error| {
            map_durable_error(DurableStorageError::persistence(operation, error))
        })?;
        self.validate_database_identity()?;
        Ok(value)
    }

    async fn rollback_execution_intent_transaction<T>(
        &self,
        transaction: Transaction<'_, Sqlite>,
        outcome: Result<T, OutcomeStoreError>,
    ) -> Result<T, OutcomeStoreError> {
        transaction.rollback().await.map_err(|error| {
            map_durable_error(DurableStorageError::persistence(
                "rollback execution-intent transaction",
                error,
            ))
        })?;
        self.validate_database_identity()?;
        outcome
    }
}

pub(super) async fn fetch_execution_intent_by_attempt(
    transaction: &mut Transaction<'_, Sqlite>,
    attempt_id: &str,
) -> Result<Option<sqlx::sqlite::SqliteRow>, OutcomeStoreError> {
    sqlx::query(
        "SELECT attempt_id, idempotency_key, payload_json, storage_hash
         FROM hepta_v2_execution_intents
         WHERE attempt_id = ?",
    )
    .bind(attempt_id)
    .fetch_optional(&mut **transaction)
    .await
    .map_err(|error| {
        map_durable_error(DurableStorageError::persistence(
            "read execution intent by attempt",
            error,
        ))
    })
}

async fn fetch_execution_intent_by_idempotency(
    transaction: &mut Transaction<'_, Sqlite>,
    idempotency_key: &str,
) -> Result<Option<sqlx::sqlite::SqliteRow>, OutcomeStoreError> {
    sqlx::query(
        "SELECT attempt_id, idempotency_key, payload_json, storage_hash
         FROM hepta_v2_execution_intents
         WHERE idempotency_key = ?",
    )
    .bind(idempotency_key)
    .fetch_optional(&mut **transaction)
    .await
    .map_err(|error| {
        map_durable_error(DurableStorageError::persistence(
            "read execution intent by idempotency key",
            error,
        ))
    })
}

async fn first_execution_intent_attempt(
    transaction: &mut Transaction<'_, Sqlite>,
) -> Result<Option<String>, OutcomeStoreError> {
    sqlx::query_scalar::<_, String>(
        "SELECT attempt_id
         FROM hepta_v2_execution_intents
         ORDER BY attempt_id
         LIMIT 1",
    )
    .fetch_optional(&mut **transaction)
    .await
    .map_err(|error| {
        map_durable_error(DurableStorageError::persistence(
            "find outstanding execution intent",
            error,
        ))
    })
}

pub(super) async fn resolve_execution_intent_in_transaction(
    database: &DurableDatabase,
    transaction: &mut Transaction<'_, Sqlite>,
    outcome: &OutcomeRecord,
    idempotency_key: &str,
) -> Result<(), OutcomeStoreError> {
    let Some(row) = fetch_execution_intent_by_attempt(transaction, outcome.attempt_id()).await?
    else {
        return Err(OutcomeStoreError::ExecutionIntentConflict {
            attempt_id: outcome.attempt_id().to_owned(),
        });
    };
    let intent = decode_execution_intent_row(database, row)?;
    if intent.idempotency_key() != idempotency_key {
        return Err(OutcomeStoreError::ExecutionIntentConflict {
            attempt_id: outcome.attempt_id().to_owned(),
        });
    }
    let effect_ack =
        super::effect_ack::execution_effect_ack_for_intent(database, transaction, &intent).await?;
    validate_terminal_binding(&intent, outcome, effect_ack.as_ref())?;
    let deleted = sqlx::query(
        "DELETE FROM hepta_v2_execution_intents
         WHERE attempt_id = ? AND idempotency_key = ?",
    )
    .bind(outcome.attempt_id())
    .bind(idempotency_key)
    .execute(&mut **transaction)
    .await
    .map_err(|error| {
        map_durable_error(DurableStorageError::persistence(
            "resolve execution intent with terminal commit",
            error,
        ))
    })?
    .rows_affected();
    if deleted != 1 {
        return Err(OutcomeStoreError::Corrupt {
            detail: format!(
                "terminal commit resolving execution intent {} affected {deleted} rows",
                outcome.attempt_id()
            ),
        });
    }
    Ok(())
}

fn encode_execution_intent(
    database: &DurableDatabase,
    intent: &ExecutionIntent,
) -> Result<crate::durable::CanonicalRow, OutcomeStoreError> {
    database
        .encode_canonical_row(&ExecutionIntentWire::from_intent(intent))
        .map_err(map_durable_error)
}

pub(super) fn decode_execution_intent_row(
    database: &DurableDatabase,
    row: sqlx::sqlite::SqliteRow,
) -> Result<ExecutionIntent, OutcomeStoreError> {
    let attempt_id = decode_column(&row, "attempt_id", "execution-intent attempt")?;
    let idempotency_key = decode_column(&row, "idempotency_key", "execution-intent key")?;
    let payload_json = decode_column(&row, "payload_json", "execution-intent payload")?;
    let storage_hash = decode_column(&row, "storage_hash", "execution-intent storage hash")?;
    let wire: ExecutionIntentWire = database
        .decode_canonical_row(
            &payload_json,
            &storage_hash,
            "pre-dispatch execution intent",
        )
        .map_err(map_durable_error)?;
    if wire.schema_version != EXECUTION_INTENT_ROW_SCHEMA_VERSION {
        return Err(OutcomeStoreError::Corrupt {
            detail: format!(
                "unsupported execution-intent row schema version {}",
                wire.schema_version
            ),
        });
    }
    let intent = wire.clone().into_intent()?;
    if attempt_id != intent.attempt_id()
        || idempotency_key != intent.idempotency_key()
        || wire.resource_summary_hash != intent.resource_summary_hash().as_str()
        || wire.effect_plan_hash.as_deref() != intent.effect_plan_hash().map(ContentHash::as_str)
        || wire.idempotency_key != intent.idempotency_key()
    {
        return Err(OutcomeStoreError::Corrupt {
            detail: format!(
                "execution-intent indexed or derived binding disagrees for attempt {attempt_id}"
            ),
        });
    }
    Ok(intent)
}

fn decode_column(
    row: &sqlx::sqlite::SqliteRow,
    column: &str,
    description: &'static str,
) -> Result<String, OutcomeStoreError> {
    row.try_get(column)
        .map_err(|error| map_durable_error(DurableStorageError::persistence(description, error)))
}

fn validate_terminal_binding(
    intent: &ExecutionIntent,
    outcome: &OutcomeRecord,
    effect_ack: Option<&ExecutionEffectAck>,
) -> Result<(), OutcomeStoreError> {
    let receipt = outcome.receipt();
    if receipt.authorization().content_hash() != intent.authorization_digest()
        || receipt.candidate().content_hash() != intent.candidate_hash()
        || receipt.payload_set_hash() != intent.payload_set_hash()
        || receipt.executed_by().as_str() != intent.executor_principal()
        || receipt.outcome_hash() != outcome.canonical_evidence_hash()
    {
        return Err(OutcomeStoreError::Corrupt {
            detail: format!(
                "terminal outcome does not match pre-dispatch intent {}",
                intent.attempt_id()
            ),
        });
    }
    terminal_evidence::validate(intent, outcome, effect_ack)
}

impl ExecutionIntentWire {
    fn from_intent(intent: &ExecutionIntent) -> Self {
        Self {
            schema_version: EXECUTION_INTENT_ROW_SCHEMA_VERSION,
            attempt_id: intent.attempt_id().to_owned(),
            session_id: intent.session_id().to_owned(),
            correlation_id: intent.correlation_id().to_owned(),
            tool_name: intent.tool_name().to_owned(),
            payload_hash: intent.payload_hash().as_str().to_owned(),
            candidate_hash: intent.candidate_hash().as_str().to_owned(),
            candidate_reference_hash: intent.candidate_reference_hash().as_str().to_owned(),
            kernel_candidate_hash: intent.kernel_candidate_hash().as_str().to_owned(),
            payload_set_hash: intent.payload_set_hash().as_str().to_owned(),
            capability_id: intent.capability_id().to_owned(),
            capability_revision: intent.capability_revision(),
            capability_provider: intent.capability_provider().to_owned(),
            capability_operation: intent.capability_operation().to_owned(),
            capability_manifest_hash: intent.capability_manifest_hash().as_str().to_owned(),
            executor_principal: intent.executor_principal().to_owned(),
            authorization_digest: intent.authorization_digest().as_str().to_owned(),
            admission_id: intent.admission_id().to_owned(),
            admission_revision: intent.admission_revision(),
            admission_digest: intent.admission_digest().as_str().to_owned(),
            canonical_resource_summary: intent.canonical_resource_summary().to_owned(),
            resource_summary_hash: intent.resource_summary_hash().as_str().to_owned(),
            canonical_effect_plan: intent.canonical_effect_plan().map(str::to_owned),
            effect_plan_hash: intent
                .effect_plan_hash()
                .map(|hash| hash.as_str().to_owned()),
            idempotency_key: intent.idempotency_key().to_owned(),
        }
    }

    fn into_intent(self) -> Result<ExecutionIntent, OutcomeStoreError> {
        ExecutionIntent::try_new(ExecutionIntentParts {
            attempt_id: self.attempt_id,
            session_id: self.session_id,
            correlation_id: self.correlation_id,
            tool_name: self.tool_name,
            payload_hash: ContentHash::new(self.payload_hash),
            candidate_hash: ContentHash::new(self.candidate_hash),
            candidate_reference_hash: ContentHash::new(self.candidate_reference_hash),
            kernel_candidate_hash: ContentHash::new(self.kernel_candidate_hash),
            payload_set_hash: ContentHash::new(self.payload_set_hash),
            capability_id: self.capability_id,
            capability_revision: self.capability_revision,
            capability_provider: self.capability_provider,
            capability_operation: self.capability_operation,
            capability_manifest_hash: ContentHash::new(self.capability_manifest_hash),
            executor_principal: self.executor_principal,
            authorization_digest: ContentHash::new(self.authorization_digest),
            admission_id: self.admission_id,
            admission_revision: self.admission_revision,
            admission_digest: ContentHash::new(self.admission_digest),
            canonical_resource_summary: self.canonical_resource_summary,
            canonical_effect_plan: self.canonical_effect_plan,
        })
        .map_err(|error| OutcomeStoreError::Corrupt {
            detail: format!("invalid recovered execution intent: {error}"),
        })
    }
}
