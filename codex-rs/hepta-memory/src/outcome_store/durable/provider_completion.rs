//! Atomic provider ACK and exact terminal-completion staging.

use hepta_contracts::ContentHash;
use hepta_contracts::OutcomeReceipt;
use sqlx::Sqlite;
use sqlx::Transaction;

use super::DurableOutcomeStore;
use super::effect_ack::decode_effect_ack_row;
use super::effect_ack::fetch_effect_ack_attempt_by_idempotency;
use super::effect_ack::fetch_effect_ack_by_attempt;
use super::effect_ack::insert_effect_ack_in_transaction;
use super::effect_ack::validate_ack_binding;
use super::execution_intent::decode_execution_intent_row;
use super::execution_intent::fetch_execution_intent_by_attempt;
use super::intent::build_record;
use super::intent::stage_pending_intent_in_transaction;
use crate::durable::DurableDatabase;
use crate::durable::DurableStorageError;
use crate::outcome_store::ExecutionEffectAck;
use crate::outcome_store::OutcomeIntentStageResult;
use crate::outcome_store::OutcomeStoreError;
use crate::outcome_store::map_durable_error;

pub(crate) const STAGE_PROVIDER_COMPLETION_COMMIT_OPERATION: &str =
    "commit provider effect ACK and exact terminal completion";

impl DurableOutcomeStore {
    /// Atomically persists the provider-owned effect ACK and exact terminal
    /// producer material in one serialized SQLite transaction.
    ///
    /// This deliberately reuses the existing authenticated outcome-intent row
    /// as the completion capsule. Older databases and ACK-only rows remain
    /// readable; presenting their exact terminal material upgrades them
    /// without replaying the provider.
    pub async fn stage_provider_completion(
        &self,
        ack: ExecutionEffectAck,
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
        if attempted.attempt_id() != ack.attempt_id() {
            return Err(OutcomeStoreError::ExecutionEffectAckBindingMismatch {
                attempt_id: attempted.attempt_id().to_owned(),
            });
        }

        let mut transaction = self.database.pool().begin().await.map_err(|error| {
            map_durable_error(DurableStorageError::persistence(
                "begin provider completion transaction",
                error,
            ))
        })?;
        self.validate_database_identity()?;
        DurableDatabase::acquire_write_serialization(&mut transaction)
            .await
            .map_err(map_durable_error)?;

        let (stage_result, intent_inserted) =
            stage_pending_intent_in_transaction(&self.database, &mut transaction, &attempted)
                .await?;
        let ack_inserted = stage_effect_ack_in_transaction(self, &mut transaction, &ack).await?;

        if !intent_inserted && !ack_inserted {
            rollback(
                self,
                transaction,
                "rollback exact provider completion replay",
            )
            .await?;
            return Ok(stage_result);
        }

        self.validate_database_identity()?;
        transaction.commit().await.map_err(|error| {
            map_durable_error(DurableStorageError::persistence(
                STAGE_PROVIDER_COMPLETION_COMMIT_OPERATION,
                error,
            ))
        })?;
        self.validate_database_identity()?;
        Ok(stage_result)
    }
}

async fn stage_effect_ack_in_transaction(
    store: &DurableOutcomeStore,
    transaction: &mut Transaction<'_, Sqlite>,
    ack: &ExecutionEffectAck,
) -> Result<bool, OutcomeStoreError> {
    if let Some(row) = fetch_effect_ack_by_attempt(transaction, ack.attempt_id()).await? {
        let existing = decode_effect_ack_row(&store.database, row)?;
        if existing != *ack {
            return Err(OutcomeStoreError::ExecutionEffectAckConflict {
                attempt_id: ack.attempt_id().to_owned(),
            });
        }
        if let Some(intent_row) =
            fetch_execution_intent_by_attempt(transaction, ack.attempt_id()).await?
        {
            validate_ack_binding(
                &decode_execution_intent_row(&store.database, intent_row)?,
                ack,
            )?;
        }
        return Ok(false);
    }
    if let Some(existing_attempt) =
        fetch_effect_ack_attempt_by_idempotency(transaction, ack.idempotency_key()).await?
    {
        return Err(OutcomeStoreError::ExecutionIdempotencyConflict {
            idempotency_key: ack.idempotency_key().to_owned(),
            existing_attempt,
            attempted_attempt: ack.attempt_id().to_owned(),
        });
    }
    let Some(intent_row) = fetch_execution_intent_by_attempt(transaction, ack.attempt_id()).await?
    else {
        return Err(OutcomeStoreError::ExecutionEffectAckIntentMissing {
            attempt_id: ack.attempt_id().to_owned(),
        });
    };
    validate_ack_binding(
        &decode_execution_intent_row(&store.database, intent_row)?,
        ack,
    )?;
    insert_effect_ack_in_transaction(&store.database, transaction, ack).await?;
    Ok(true)
}

async fn rollback(
    store: &DurableOutcomeStore,
    transaction: Transaction<'_, Sqlite>,
    operation: &'static str,
) -> Result<(), OutcomeStoreError> {
    transaction
        .rollback()
        .await
        .map_err(|error| map_durable_error(DurableStorageError::persistence(operation, error)))?;
    store.validate_database_identity()
}
