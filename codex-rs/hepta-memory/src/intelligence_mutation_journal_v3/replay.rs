use codex_hepta_contracts::Sha256Digest;
use sqlx::Row;
use sqlx::Sqlite;
use sqlx::SqlitePool;
use sqlx::Transaction;
use sqlx::sqlite::SqliteRow;

use crate::CognitiveStoreError;
use crate::cognitive_store::unavailable;

use super::IntelligenceMutationApplyDisposition;
use super::IntelligenceMutationBinding;
use super::IntelligenceMutationJournalError;
use super::IntelligenceMutationState;
use super::IntelligenceMutationTransitionReceipt;
use super::IntelligenceMutationTransitionRequest;
use super::MAX_JOURNALED_OPERATIONS;
use super::MAX_TRANSITIONS_PER_OPERATION;
use super::PersistedAction;
use super::bool_i64;
use super::digest;
use super::from_i64;
use super::parse_digest;
use super::parse_phase;
use super::to_i64;

pub(super) async fn verify_all(
    pool: &SqlitePool,
    owner_agent_id: &str,
) -> Result<(), IntelligenceMutationJournalError> {
    let operation_ids = sqlx::query_scalar::<_, String>(
        "SELECT operation_id
         FROM cognitive_intelligence_mutation_operations
         WHERE owner_agent_id = ?
         ORDER BY operation_id LIMIT ?",
    )
    .bind(owner_agent_id)
    .bind(limit_plus_one(MAX_JOURNALED_OPERATIONS)?)
    .fetch_all(pool)
    .await
    .map_err(unavailable)?;
    if operation_ids.len() > MAX_JOURNALED_OPERATIONS {
        return Err(CognitiveStoreError::Corrupt(format!(
            "intelligence mutation journal exceeds {MAX_JOURNALED_OPERATIONS} operations"
        ))
        .into());
    }
    let foreign_owner_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*)
         FROM cognitive_intelligence_mutation_operations
         WHERE owner_agent_id != ?",
    )
    .bind(owner_agent_id)
    .fetch_one(pool)
    .await
    .map_err(unavailable)?;
    if foreign_owner_count != 0 {
        return Err(CognitiveStoreError::Corrupt(
            "intelligence mutation journal contains a foreign owner".to_string(),
        )
        .into());
    }
    for operation_id in operation_ids {
        replay_operation_pool(pool, owner_agent_id, &operation_id).await?;
    }
    Ok(())
}

pub(super) async fn replay_operation_pool(
    pool: &SqlitePool,
    owner_agent_id: &str,
    operation_id: &str,
) -> Result<IntelligenceMutationState, IntelligenceMutationJournalError> {
    let operation = sqlx::query(operation_query())
        .bind(operation_id)
        .fetch_optional(pool)
        .await
        .map_err(unavailable)?
        .ok_or_else(|| {
            CognitiveStoreError::Invalid(
                "intelligence mutation operation does not exist".to_string(),
            )
        })?;
    let transitions = sqlx::query(transition_query())
        .bind(operation_id)
        .bind(limit_plus_one(MAX_TRANSITIONS_PER_OPERATION)?)
        .fetch_all(pool)
        .await
        .map_err(unavailable)?;
    replay_rows(&operation, transitions, owner_agent_id)
}

pub(super) async fn replay_operation_tx(
    transaction: &mut Transaction<'_, Sqlite>,
    owner_agent_id: &str,
    operation_id: &str,
) -> Result<IntelligenceMutationState, IntelligenceMutationJournalError> {
    let operation = sqlx::query(operation_query())
        .bind(operation_id)
        .fetch_optional(&mut **transaction)
        .await
        .map_err(unavailable)?
        .ok_or_else(|| {
            CognitiveStoreError::Invalid(
                "intelligence mutation operation does not exist".to_string(),
            )
        })?;
    let transitions = sqlx::query(transition_query())
        .bind(operation_id)
        .bind(limit_plus_one(MAX_TRANSITIONS_PER_OPERATION)?)
        .fetch_all(&mut **transaction)
        .await
        .map_err(unavailable)?;
    replay_rows(&operation, transitions, owner_agent_id)
}

pub(super) fn verify_operation_binding_row(
    row: &SqliteRow,
    owner_agent_id: &str,
    binding: &IntelligenceMutationBinding,
    expected_binding_sha256: &Sha256Digest,
) -> Result<(), CognitiveStoreError> {
    let stored_owner: String = row.try_get("owner_agent_id").map_err(unavailable)?;
    let stored_lease_id: String = row.try_get("lease_id").map_err(unavailable)?;
    let stored_lease_epoch = from_i64(
        row.try_get("lease_epoch").map_err(unavailable)?,
        "lease epoch",
    )?;
    let stored_expected_revision = row
        .try_get::<Option<i64>, _>("expected_revision")
        .map_err(unavailable)?
        .map(|value| from_i64(value, "expected revision"))
        .transpose()?;
    let stored_generation = from_i64(
        row.try_get("starting_projection_generation")
            .map_err(unavailable)?,
        "starting projection generation",
    )?;
    let stored_root: String = row.try_get("causal_root_sha256").map_err(unavailable)?;
    let stored_binding: String = row.try_get("binding_sha256").map_err(unavailable)?;
    if stored_owner != owner_agent_id
        || stored_lease_id != binding.lease_id
        || stored_lease_epoch != binding.lease_epoch
        || stored_expected_revision != binding.expected_revision
        || stored_generation != binding.starting_projection_generation
        || stored_root != binding.causal_root_sha256.as_str()
        || stored_binding != expected_binding_sha256.as_str()
    {
        return Err(CognitiveStoreError::Conflict(
            "intelligence mutation operation ID is already bound to different inputs"
                .to_string(),
        ));
    }
    Ok(())
}

fn replay_rows(
    operation: &SqliteRow,
    transitions: Vec<SqliteRow>,
    owner_agent_id: &str,
) -> Result<IntelligenceMutationState, IntelligenceMutationJournalError> {
    if transitions.len() > MAX_TRANSITIONS_PER_OPERATION {
        return Err(CognitiveStoreError::Corrupt(format!(
            "intelligence mutation operation exceeds {MAX_TRANSITIONS_PER_OPERATION} transitions"
        ))
        .into());
    }
    let binding = binding_from_row(operation, owner_agent_id)?;
    let expected_binding_sha256 = digest::binding_digest(&binding);
    verify_operation_binding_row(
        operation,
        owner_agent_id,
        &binding,
        &expected_binding_sha256,
    )?;
    let mut state = IntelligenceMutationState::new(binding.clone())?;
    for row in transitions {
        let sequence = from_i64(
            row.try_get("sequence").map_err(unavailable)?,
            "transition sequence",
        )?;
        let action_json: String = row.try_get("action_payload_json").map_err(unavailable)?;
        let persisted: PersistedAction = serde_json::from_str(&action_json).map_err(|error| {
            CognitiveStoreError::Corrupt(format!(
                "invalid persisted intelligence mutation action: {error}"
            ))
        })?;
        let stored_action: String = row.try_get("action").map_err(unavailable)?;
        if stored_action != persisted.kind() {
            return Err(CognitiveStoreError::Corrupt(
                "persisted intelligence mutation action kind drifted".to_string(),
            )
            .into());
        }
        let causal_parent_sha256 = row
            .try_get::<Option<String>, _>("causal_parent_sha256")
            .map_err(unavailable)?
            .map(|value| parse_digest(value, "causal parent digest"))
            .transpose()?;
        let request = IntelligenceMutationTransitionRequest {
            binding: binding.clone(),
            sequence,
            causal_parent_sha256,
            action: persisted.into_action()?,
        };
        let result = state.apply(request)?;
        if result.disposition != IntelligenceMutationApplyDisposition::Applied {
            return Err(CognitiveStoreError::Corrupt(
                "journal contains a duplicate transition row".to_string(),
            )
            .into());
        }
        verify_transition_row(&row, &result.receipt)?;
    }
    state.validate()?;
    Ok(state)
}

fn binding_from_row(
    row: &SqliteRow,
    owner_agent_id: &str,
) -> Result<IntelligenceMutationBinding, CognitiveStoreError> {
    let stored_owner: String = row.try_get("owner_agent_id").map_err(unavailable)?;
    if stored_owner != owner_agent_id {
        return Err(CognitiveStoreError::Corrupt(
            "intelligence mutation operation owner binding changed".to_string(),
        ));
    }
    Ok(IntelligenceMutationBinding {
        operation_id: row.try_get("operation_id").map_err(unavailable)?,
        lease_id: row.try_get("lease_id").map_err(unavailable)?,
        lease_epoch: from_i64(
            row.try_get("lease_epoch").map_err(unavailable)?,
            "lease epoch",
        )?,
        expected_revision: row
            .try_get::<Option<i64>, _>("expected_revision")
            .map_err(unavailable)?
            .map(|value| from_i64(value, "expected revision"))
            .transpose()?,
        starting_projection_generation: from_i64(
            row.try_get("starting_projection_generation")
                .map_err(unavailable)?,
            "starting projection generation",
        )?,
        causal_root_sha256: parse_digest(
            row.try_get("causal_root_sha256").map_err(unavailable)?,
            "causal root digest",
        )?,
    })
}

fn verify_transition_row(
    row: &SqliteRow,
    receipt: &IntelligenceMutationTransitionReceipt,
) -> Result<(), CognitiveStoreError> {
    let stored_from: String = row.try_get("from_phase").map_err(unavailable)?;
    let stored_to: String = row.try_get("to_phase").map_err(unavailable)?;
    let stored_request: String = row.try_get("request_sha256").map_err(unavailable)?;
    let stored_parent: Option<String> = row
        .try_get("causal_parent_sha256")
        .map_err(unavailable)?;
    let stored_transition: String = row.try_get("transition_sha256").map_err(unavailable)?;
    let stored_intent_appended: i64 = row
        .try_get("durable_intent_appended")
        .map_err(unavailable)?;
    let stored_intent_settled: i64 = row
        .try_get("durable_intent_settled")
        .map_err(unavailable)?;
    let stored_memory_writes: i64 = row.try_get("memory_write_count").map_err(unavailable)?;
    let stored_projection_publishes: i64 = row
        .try_get("projection_publish_count")
        .map_err(unavailable)?;
    let stored_generation: i64 = row
        .try_get("last_published_generation")
        .map_err(unavailable)?;
    if parse_phase(&stored_from)? != receipt.from_phase
        || parse_phase(&stored_to)? != receipt.to_phase
        || stored_request != receipt.request_sha256.as_str()
        || stored_parent.as_deref()
            != receipt
                .causal_parent_sha256
                .as_ref()
                .map(Sha256Digest::as_str)
        || stored_transition != receipt.transition_sha256.as_str()
        || stored_intent_appended != bool_i64(receipt.durable_intent_appended)
        || stored_intent_settled != bool_i64(receipt.durable_intent_settled)
        || stored_memory_writes != i64::from(receipt.memory_write_count)
        || stored_projection_publishes != i64::from(receipt.projection_publish_count)
        || stored_generation != to_i64(
            receipt.last_published_generation,
            "last published generation",
        )?
    {
        return Err(CognitiveStoreError::Corrupt(
            "persisted intelligence mutation receipt failed exact replay".to_string(),
        ));
    }
    Ok(())
}

fn operation_query() -> &'static str {
    "SELECT operation_id, owner_agent_id, lease_id, lease_epoch,
            expected_revision, starting_projection_generation,
            causal_root_sha256, binding_sha256
     FROM cognitive_intelligence_mutation_operations
     WHERE operation_id = ?"
}

fn transition_query() -> &'static str {
    "SELECT sequence, from_phase, to_phase, action, action_payload_json,
            request_sha256, causal_parent_sha256, transition_sha256,
            durable_intent_appended, durable_intent_settled,
            memory_write_count, projection_publish_count,
            last_published_generation
     FROM cognitive_intelligence_mutation_transitions
     WHERE operation_id = ?
     ORDER BY sequence LIMIT ?"
}

fn limit_plus_one(value: usize) -> Result<i64, CognitiveStoreError> {
    value
        .checked_add(1)
        .and_then(|next| i64::try_from(next).ok())
        .ok_or_else(|| CognitiveStoreError::Corrupt("journal limit exceeds i64".to_string()))
}
