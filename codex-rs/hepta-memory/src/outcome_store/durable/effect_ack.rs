use std::collections::BTreeSet;

use hepta_contracts::ContentHash;
use serde::Deserialize;
use serde::Serialize;
use sqlx::Row;
use sqlx::Sqlite;
use sqlx::Transaction;

use super::DurableOutcomeStore;
use super::execution_intent::decode_execution_intent_row;
use super::execution_intent::fetch_execution_intent_by_attempt;
use super::fetch_by_attempt;
use crate::durable::DurableDatabase;
use crate::durable::DurableStorageError;
use crate::outcome_store::ExecutionEffectAck;
use crate::outcome_store::ExecutionEffectAckParts;
use crate::outcome_store::ExecutionEffectAckRecordResult;
use crate::outcome_store::ExecutionIntent;
use crate::outcome_store::OutcomeStoreError;
use crate::outcome_store::map_durable_error;

const EFFECT_ACK_ROW_SCHEMA_VERSION: u32 = 1;
pub(crate) const RECORD_EFFECT_ACK_COMMIT_OPERATION: &str =
    "commit provider execution effect acknowledgement";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ExecutionEffectAckWire {
    schema_version: u32,
    attempt_id: String,
    idempotency_key: String,
    effect_plan_hash: String,
    canonical_provider_ack: String,
    ack_hash: String,
}

impl DurableOutcomeStore {
    pub async fn record_execution_effect_ack(
        &self,
        ack: ExecutionEffectAck,
    ) -> Result<ExecutionEffectAckRecordResult, OutcomeStoreError> {
        self.validate_database_identity()?;
        let mut transaction = self.database.pool().begin().await.map_err(|error| {
            map_durable_error(DurableStorageError::persistence(
                "begin execution effect ACK transaction",
                error,
            ))
        })?;
        self.validate_database_identity()?;
        DurableDatabase::acquire_write_serialization(&mut transaction)
            .await
            .map_err(map_durable_error)?;

        if let Some(row) = fetch_effect_ack_by_attempt(&mut transaction, ack.attempt_id()).await? {
            let existing = decode_effect_ack_row(&self.database, row)?;
            let result = if existing == ack {
                Ok(ExecutionEffectAckRecordResult::AlreadyRecorded)
            } else {
                Err(OutcomeStoreError::ExecutionEffectAckConflict {
                    attempt_id: ack.attempt_id().to_owned(),
                })
            };
            transaction.rollback().await.map_err(|error| {
                map_durable_error(DurableStorageError::persistence(
                    "rollback execution effect ACK replay",
                    error,
                ))
            })?;
            self.validate_database_identity()?;
            return result;
        }
        if let Some(existing_attempt) =
            fetch_effect_ack_attempt_by_idempotency(&mut transaction, ack.idempotency_key()).await?
        {
            transaction.rollback().await.map_err(|error| {
                map_durable_error(DurableStorageError::persistence(
                    "rollback execution effect ACK idempotency conflict",
                    error,
                ))
            })?;
            self.validate_database_identity()?;
            return Err(OutcomeStoreError::ExecutionIdempotencyConflict {
                idempotency_key: ack.idempotency_key().to_owned(),
                existing_attempt,
                attempted_attempt: ack.attempt_id().to_owned(),
            });
        }
        let Some(intent_row) =
            fetch_execution_intent_by_attempt(&mut transaction, ack.attempt_id()).await?
        else {
            transaction.rollback().await.map_err(|error| {
                map_durable_error(DurableStorageError::persistence(
                    "rollback execution effect ACK without intent",
                    error,
                ))
            })?;
            self.validate_database_identity()?;
            return Err(OutcomeStoreError::ExecutionEffectAckIntentMissing {
                attempt_id: ack.attempt_id().to_owned(),
            });
        };
        let intent = decode_execution_intent_row(&self.database, intent_row)?;
        validate_ack_binding(&intent, &ack)?;

        insert_effect_ack_in_transaction(&self.database, &mut transaction, &ack).await?;
        self.validate_database_identity()?;
        transaction.commit().await.map_err(|error| {
            map_durable_error(DurableStorageError::persistence(
                RECORD_EFFECT_ACK_COMMIT_OPERATION,
                error,
            ))
        })?;
        self.validate_database_identity()?;
        Ok(ExecutionEffectAckRecordResult::Recorded)
    }

    pub async fn execution_effect_ack(
        &self,
        attempt_id: &str,
    ) -> Result<Option<ExecutionEffectAck>, OutcomeStoreError> {
        self.validate_database_identity()?;
        let row = sqlx::query(
            "SELECT attempt_id, idempotency_key, effect_plan_hash, payload_json, storage_hash
             FROM hepta_v2_execution_effect_acks
             WHERE attempt_id = ?",
        )
        .bind(attempt_id)
        .fetch_optional(self.database.pool())
        .await
        .map_err(|error| {
            map_durable_error(DurableStorageError::persistence(
                "read execution effect ACK",
                error,
            ))
        })?;
        self.validate_database_identity()?;
        row.map(|row| decode_effect_ack_row(&self.database, row))
            .transpose()
    }

    pub(super) async fn verify_execution_effect_ack_recovery(
        &self,
    ) -> Result<(), OutcomeStoreError> {
        let mut transaction = self.database.pool().begin().await.map_err(|error| {
            map_durable_error(DurableStorageError::persistence(
                "begin execution effect ACK recovery snapshot",
                error,
            ))
        })?;
        let rows = sqlx::query(
            "SELECT attempt_id, idempotency_key, effect_plan_hash, payload_json, storage_hash
             FROM hepta_v2_execution_effect_acks
             ORDER BY attempt_id",
        )
        .fetch_all(&mut *transaction)
        .await
        .map_err(|error| {
            map_durable_error(DurableStorageError::persistence(
                "scan execution effect ACKs during recovery",
                error,
            ))
        })?;
        let mut attempts = BTreeSet::new();
        let mut idempotency_keys = BTreeSet::new();
        for row in rows {
            let ack = decode_effect_ack_row(&self.database, row)?;
            if !attempts.insert(ack.attempt_id().to_owned())
                || !idempotency_keys.insert(ack.idempotency_key().to_owned())
            {
                return Err(OutcomeStoreError::Corrupt {
                    detail: "duplicate recovered execution effect ACK".into(),
                });
            }
            if let Some(intent_row) =
                fetch_execution_intent_by_attempt(&mut transaction, ack.attempt_id()).await?
            {
                validate_ack_binding(
                    &decode_execution_intent_row(&self.database, intent_row)?,
                    &ack,
                )?;
            } else if fetch_by_attempt(&mut transaction, ack.attempt_id())
                .await?
                .is_none()
            {
                return Err(OutcomeStoreError::Corrupt {
                    detail: format!(
                        "execution effect ACK {} has neither a pending intent nor terminal outcome",
                        ack.attempt_id()
                    ),
                });
            }
        }
        transaction.rollback().await.map_err(|error| {
            map_durable_error(DurableStorageError::persistence(
                "close execution effect ACK recovery snapshot",
                error,
            ))
        })?;
        self.validate_database_identity()
    }
}

pub(super) async fn execution_effect_ack_for_intent(
    database: &DurableDatabase,
    transaction: &mut Transaction<'_, Sqlite>,
    intent: &ExecutionIntent,
) -> Result<Option<ExecutionEffectAck>, OutcomeStoreError> {
    let ack = fetch_effect_ack_by_attempt(transaction, intent.attempt_id())
        .await?
        .map(|row| decode_effect_ack_row(database, row))
        .transpose()?;
    match (intent.effect_plan_hash(), ack.as_ref()) {
        (None, None) => Ok(None),
        (None, Some(_)) => Err(OutcomeStoreError::Corrupt {
            detail: format!(
                "execution intent {} has an unplanned provider effect ACK",
                intent.attempt_id()
            ),
        }),
        (Some(_), None) => Err(OutcomeStoreError::ExecutionEffectAckIntentMissing {
            attempt_id: intent.attempt_id().to_owned(),
        }),
        (Some(_), Some(ack)) => {
            validate_ack_binding(intent, ack)?;
            Ok(Some(ack.clone()))
        }
    }
}

pub(super) fn validate_ack_binding(
    intent: &ExecutionIntent,
    ack: &ExecutionEffectAck,
) -> Result<(), OutcomeStoreError> {
    let Some(effect_plan_hash) = intent.effect_plan_hash() else {
        return Err(OutcomeStoreError::ExecutionEffectAckPlanMissing {
            attempt_id: intent.attempt_id().to_owned(),
        });
    };
    if ack.attempt_id() != intent.attempt_id()
        || ack.idempotency_key() != intent.idempotency_key()
        || ack.effect_plan_hash() != effect_plan_hash
    {
        return Err(OutcomeStoreError::ExecutionEffectAckBindingMismatch {
            attempt_id: intent.attempt_id().to_owned(),
        });
    }
    Ok(())
}

pub(super) async fn fetch_effect_ack_by_attempt(
    transaction: &mut Transaction<'_, Sqlite>,
    attempt_id: &str,
) -> Result<Option<sqlx::sqlite::SqliteRow>, OutcomeStoreError> {
    sqlx::query(
        "SELECT attempt_id, idempotency_key, effect_plan_hash, payload_json, storage_hash
         FROM hepta_v2_execution_effect_acks
         WHERE attempt_id = ?",
    )
    .bind(attempt_id)
    .fetch_optional(&mut **transaction)
    .await
    .map_err(|error| {
        map_durable_error(DurableStorageError::persistence(
            "read execution effect ACK by attempt",
            error,
        ))
    })
}

pub(super) async fn fetch_effect_ack_attempt_by_idempotency(
    transaction: &mut Transaction<'_, Sqlite>,
    idempotency_key: &str,
) -> Result<Option<String>, OutcomeStoreError> {
    sqlx::query_scalar::<_, String>(
        "SELECT attempt_id
         FROM hepta_v2_execution_effect_acks
         WHERE idempotency_key = ?",
    )
    .bind(idempotency_key)
    .fetch_optional(&mut **transaction)
    .await
    .map_err(|error| {
        map_durable_error(DurableStorageError::persistence(
            "read execution effect ACK by idempotency key",
            error,
        ))
    })
}

pub(super) fn decode_effect_ack_row(
    database: &DurableDatabase,
    row: sqlx::sqlite::SqliteRow,
) -> Result<ExecutionEffectAck, OutcomeStoreError> {
    let attempt_id: String = row.try_get("attempt_id").map_err(|error| {
        map_durable_error(DurableStorageError::persistence(
            "decode execution effect ACK attempt",
            error,
        ))
    })?;
    let idempotency_key: String = row.try_get("idempotency_key").map_err(|error| {
        map_durable_error(DurableStorageError::persistence(
            "decode execution effect ACK idempotency key",
            error,
        ))
    })?;
    let effect_plan_hash: String = row.try_get("effect_plan_hash").map_err(|error| {
        map_durable_error(DurableStorageError::persistence(
            "decode execution effect ACK plan hash",
            error,
        ))
    })?;
    let payload_json: String = row.try_get("payload_json").map_err(|error| {
        map_durable_error(DurableStorageError::persistence(
            "decode execution effect ACK payload",
            error,
        ))
    })?;
    let storage_hash: String = row.try_get("storage_hash").map_err(|error| {
        map_durable_error(DurableStorageError::persistence(
            "decode execution effect ACK storage hash",
            error,
        ))
    })?;
    let wire: ExecutionEffectAckWire = database
        .decode_canonical_row(&payload_json, &storage_hash, "execution effect ACK")
        .map_err(map_durable_error)?;
    if wire.schema_version != EFFECT_ACK_ROW_SCHEMA_VERSION {
        return Err(OutcomeStoreError::Corrupt {
            detail: format!(
                "unsupported execution effect ACK row schema version {}",
                wire.schema_version
            ),
        });
    }
    let ack = wire.clone().into_ack()?;
    if attempt_id != ack.attempt_id()
        || idempotency_key != ack.idempotency_key()
        || effect_plan_hash != ack.effect_plan_hash().as_str()
        || wire.ack_hash != ack.ack_hash().as_str()
    {
        return Err(OutcomeStoreError::Corrupt {
            detail: format!(
                "execution effect ACK indexed or derived binding disagrees for attempt {attempt_id}"
            ),
        });
    }
    Ok(ack)
}

impl ExecutionEffectAckWire {
    fn from_ack(ack: &ExecutionEffectAck) -> Self {
        Self {
            schema_version: EFFECT_ACK_ROW_SCHEMA_VERSION,
            attempt_id: ack.attempt_id().to_owned(),
            idempotency_key: ack.idempotency_key().to_owned(),
            effect_plan_hash: ack.effect_plan_hash().as_str().to_owned(),
            canonical_provider_ack: ack.canonical_provider_ack().to_owned(),
            ack_hash: ack.ack_hash().as_str().to_owned(),
        }
    }

    fn into_ack(self) -> Result<ExecutionEffectAck, OutcomeStoreError> {
        ExecutionEffectAck::try_new(ExecutionEffectAckParts {
            attempt_id: self.attempt_id,
            idempotency_key: self.idempotency_key,
            effect_plan_hash: ContentHash::new(self.effect_plan_hash),
            canonical_provider_ack: self.canonical_provider_ack,
        })
        .map_err(|error| OutcomeStoreError::Corrupt {
            detail: format!("invalid recovered execution effect ACK: {error}"),
        })
    }
}

pub(super) async fn insert_effect_ack_in_transaction(
    database: &DurableDatabase,
    transaction: &mut Transaction<'_, Sqlite>,
    ack: &ExecutionEffectAck,
) -> Result<(), OutcomeStoreError> {
    let encoded = database
        .encode_canonical_row(&ExecutionEffectAckWire::from_ack(ack))
        .map_err(map_durable_error)?;
    let inserted = sqlx::query(
        "INSERT INTO hepta_v2_execution_effect_acks (
            attempt_id,
            idempotency_key,
            effect_plan_hash,
            payload_json,
            storage_hash
         ) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(ack.attempt_id())
    .bind(ack.idempotency_key())
    .bind(ack.effect_plan_hash().as_str())
    .bind(&encoded.payload_json)
    .bind(&encoded.storage_hash)
    .execute(&mut **transaction)
    .await
    .map_err(|error| {
        map_durable_error(DurableStorageError::persistence(
            "insert execution effect ACK",
            error,
        ))
    })?
    .rows_affected();
    if inserted != 1 {
        return Err(OutcomeStoreError::Corrupt {
            detail: format!("execution effect ACK insert affected {inserted} rows"),
        });
    }
    Ok(())
}
