//! Provider effect acknowledgement and completion recovery tests.

use super::*;

use crate::DurableIntegrityKey;
use crate::DurableOutcomeStore;
use crate::ExecutionEffectAck;
use crate::ExecutionEffectAckParts;
use crate::ExecutionEffectAckRecordResult;
use crate::ExecutionIntentStageResult;
use crate::OutcomeIntentStageResult;
use crate::OutcomeStoreError;

use super::execution_intent::canonical_execution_outcome_with_ack;
use super::execution_intent::execution_intent_with_effect_plan;

#[tokio::test]
async fn durable_effect_ack_is_exact_replay_and_survives_reopen() -> TestResult {
    let directory = tempfile::tempdir()?;
    let database_path = directory.path().join("effect-ack.sqlite3");
    let intent = planned_intent("attempt-effect-ack")?;
    let ack = effect_ack(&intent, r#"{"status":"committed","target":"a"}"#)?;
    {
        let store = DurableOutcomeStore::bootstrap_new(&database_path).await?;
        assert_eq!(
            store.stage_execution_intent(intent.clone()).await?,
            ExecutionIntentStageResult::Staged
        );
        assert_eq!(
            store.record_execution_effect_ack(ack.clone()).await?,
            ExecutionEffectAckRecordResult::Recorded
        );
        assert_eq!(
            store.record_execution_effect_ack(ack.clone()).await?,
            ExecutionEffectAckRecordResult::AlreadyRecorded
        );
    }
    let reopened = DurableOutcomeStore::open_existing(&database_path).await?;
    assert_eq!(
        reopened.execution_effect_ack(intent.attempt_id()).await?,
        Some(ack)
    );
    Ok(())
}

#[tokio::test]
async fn durable_effect_ack_conflict_fails_closed() -> TestResult {
    let directory = tempfile::tempdir()?;
    let store =
        DurableOutcomeStore::bootstrap_new(directory.path().join("effect-conflict.sqlite3"))
            .await?;
    let intent = planned_intent("attempt-effect-conflict")?;
    store.stage_execution_intent(intent.clone()).await?;
    store
        .record_execution_effect_ack(effect_ack(
            &intent,
            r#"{"status":"committed","target":"a"}"#,
        )?)
        .await?;
    let error = store
        .record_execution_effect_ack(effect_ack(
            &intent,
            r#"{"status":"committed","target":"b"}"#,
        )?)
        .await
        .expect_err("changed provider ACK must not replay");
    assert_eq!(
        error,
        OutcomeStoreError::ExecutionEffectAckConflict {
            attempt_id: intent.attempt_id().to_owned()
        }
    );
    Ok(())
}

#[tokio::test]
async fn provider_ack_and_exact_terminal_capsule_commit_atomically_once() -> TestResult {
    let directory = tempfile::tempdir()?;
    let database_path = directory.path().join("provider-completion.sqlite3");
    let intent = planned_intent("attempt-provider-completion")?;
    let ack = effect_ack(&intent, r#"{"status":"committed","target":"completion"}"#)?;
    let store = DurableOutcomeStore::bootstrap_new(&database_path).await?;
    store.stage_execution_intent(intent.clone()).await?;
    let before = store.monotonic_state().await?;
    let (receipt, evidence, evidence_hash) =
        canonical_execution_outcome_with_ack(&intent, Some(&ack))?;

    assert_eq!(
        store
            .stage_provider_completion(
                ack.clone(),
                intent.attempt_id(),
                receipt.clone(),
                evidence.clone(),
                evidence_hash.clone(),
            )
            .await?,
        OutcomeIntentStageResult::Pending
    );
    let after = store.monotonic_state().await?;
    assert_eq!(after.generation(), before.generation() + 1);
    assert_eq!(
        store.execution_effect_ack(intent.attempt_id()).await?,
        Some(ack.clone())
    );
    assert_eq!(
        store
            .pending_intent(intent.attempt_id())
            .await?
            .expect("exact completion capsule")
            .record()
            .canonical_evidence(),
        evidence
    );

    assert_eq!(
        store
            .stage_provider_completion(ack, intent.attempt_id(), receipt, evidence, evidence_hash,)
            .await?,
        OutcomeIntentStageResult::Pending
    );
    assert_eq!(store.monotonic_state().await?, after);
    Ok(())
}

#[tokio::test]
async fn invalid_ack_rolls_back_terminal_capsule_without_partial_rows() -> TestResult {
    let directory = tempfile::tempdir()?;
    let store = DurableOutcomeStore::bootstrap_new(
        directory
            .path()
            .join("provider-completion-rollback.sqlite3"),
    )
    .await?;
    let intent = planned_intent("attempt-provider-completion-rollback")?;
    store.stage_execution_intent(intent.clone()).await?;
    let valid_ack = effect_ack(&intent, r#"{"status":"committed","target":"valid"}"#)?;
    let invalid_ack = ExecutionEffectAck::try_new(ExecutionEffectAckParts {
        attempt_id: intent.attempt_id().to_owned(),
        idempotency_key: intent.idempotency_key().to_owned(),
        effect_plan_hash: ContentHash::new("sha256:wrong-plan"),
        canonical_provider_ack: r#"{"status":"committed","target":"invalid"}"#.into(),
    })?;
    let (receipt, evidence, evidence_hash) =
        canonical_execution_outcome_with_ack(&intent, Some(&valid_ack))?;

    assert_eq!(
        store
            .stage_provider_completion(
                invalid_ack,
                intent.attempt_id(),
                receipt,
                evidence,
                evidence_hash,
            )
            .await
            .expect_err("invalid ACK binding must roll back both rows"),
        OutcomeStoreError::ExecutionEffectAckBindingMismatch {
            attempt_id: intent.attempt_id().to_owned(),
        }
    );
    assert!(
        store
            .execution_effect_ack(intent.attempt_id())
            .await?
            .is_none()
    );
    assert!(store.pending_intent(intent.attempt_id()).await?.is_none());
    Ok(())
}

#[tokio::test]
async fn altered_ack_or_terminal_capsule_conflicts_without_state_change() -> TestResult {
    let directory = tempfile::tempdir()?;
    let store = DurableOutcomeStore::bootstrap_new(
        directory
            .path()
            .join("provider-completion-conflict.sqlite3"),
    )
    .await?;
    let intent = planned_intent("attempt-provider-completion-conflict")?;
    store.stage_execution_intent(intent.clone()).await?;
    let ack = effect_ack(&intent, r#"{"status":"committed","target":"original"}"#)?;
    let (receipt, evidence, evidence_hash) =
        canonical_execution_outcome_with_ack(&intent, Some(&ack))?;
    store
        .stage_provider_completion(
            ack.clone(),
            intent.attempt_id(),
            receipt.clone(),
            evidence.clone(),
            evidence_hash.clone(),
        )
        .await?;
    let stable = store.monotonic_state().await?;

    let altered_ack = effect_ack(&intent, r#"{"status":"committed","target":"altered"}"#)?;
    assert!(matches!(
        store
            .stage_provider_completion(
                altered_ack,
                intent.attempt_id(),
                receipt.clone(),
                evidence.clone(),
                evidence_hash.clone(),
            )
            .await,
        Err(OutcomeStoreError::ExecutionEffectAckConflict { .. })
    ));
    assert!(matches!(
        store
            .stage_provider_completion(
                ack,
                intent.attempt_id(),
                receipt,
                format!("{evidence} "),
                ContentHash::new("sha256:altered-evidence"),
            )
            .await,
        Err(OutcomeStoreError::EvidenceEnvelopeAndHashConflict { .. })
    ));
    assert_eq!(store.monotonic_state().await?, stable);
    Ok(())
}

#[tokio::test]
async fn authenticated_provider_completion_capsule_tampering_fails_on_restart() -> TestResult {
    let directory = tempfile::tempdir()?;
    let database_path = directory.path().join("provider-completion-tamper.sqlite3");
    let intent = planned_intent("attempt-provider-completion-tamper")?;
    let ack = effect_ack(&intent, r#"{"status":"committed","target":"tamper"}"#)?;
    let (receipt, evidence, evidence_hash) =
        canonical_execution_outcome_with_ack(&intent, Some(&ack))?;
    {
        let store = DurableOutcomeStore::bootstrap_new_keyed(
            &database_path,
            DurableIntegrityKey::from_bytes([0x63; 32]),
        )
        .await?;
        store.stage_execution_intent(intent.clone()).await?;
        store
            .stage_provider_completion(ack, intent.attempt_id(), receipt, evidence, evidence_hash)
            .await?;
    }

    let options = sqlx::sqlite::SqliteConnectOptions::new().filename(&database_path);
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(options)
        .await?;
    sqlx::query(
        "UPDATE hepta_v2_outcome_intents
         SET storage_hash = 'hmac-sha256:00'
         WHERE attempt_id = ?",
    )
    .bind(intent.attempt_id())
    .execute(&pool)
    .await?;
    pool.close().await;

    let error = match DurableOutcomeStore::open_existing_keyed(
        &database_path,
        DurableIntegrityKey::from_bytes([0x63; 32]),
    )
    .await
    {
        Ok(_) => panic!("tampered provider completion capsule must fail closed"),
        Err(error) => error,
    };
    assert!(
        matches!(error, OutcomeStoreError::Corrupt { .. }),
        "unexpected tamper error: {error:?}"
    );
    Ok(())
}

#[tokio::test]
async fn legacy_ack_only_row_reopens_fail_closed_and_accepts_exact_capsule_upgrade() -> TestResult {
    let directory = tempfile::tempdir()?;
    let database_path = directory.path().join("provider-completion-legacy.sqlite3");
    let intent = planned_intent("attempt-provider-completion-legacy")?;
    let ack = effect_ack(&intent, r#"{"status":"committed","target":"legacy"}"#)?;
    {
        let store = DurableOutcomeStore::bootstrap_new(&database_path).await?;
        store.stage_execution_intent(intent.clone()).await?;
        store.record_execution_effect_ack(ack.clone()).await?;
    }

    let reopened = DurableOutcomeStore::open_existing(&database_path).await?;
    assert_eq!(
        reopened.execution_effect_ack(intent.attempt_id()).await?,
        Some(ack.clone())
    );
    assert!(
        reopened
            .pending_intent(intent.attempt_id())
            .await?
            .is_none()
    );
    let (receipt, evidence, evidence_hash) =
        canonical_execution_outcome_with_ack(&intent, Some(&ack))?;
    assert_eq!(
        reopened
            .stage_provider_completion(ack, intent.attempt_id(), receipt, evidence, evidence_hash,)
            .await?,
        OutcomeIntentStageResult::Pending
    );
    assert!(
        reopened
            .pending_intent(intent.attempt_id())
            .await?
            .is_some()
    );
    Ok(())
}

#[tokio::test]
async fn effect_ack_requires_a_staged_effect_plan() -> TestResult {
    let directory = tempfile::tempdir()?;
    let store =
        DurableOutcomeStore::bootstrap_new(directory.path().join("effect-plan-required.sqlite3"))
            .await?;
    let intent =
        execution_intent_with_effect_plan("attempt-effect-unplanned", "correlation", None)?;
    store.stage_execution_intent(intent.clone()).await?;
    let ack = ExecutionEffectAck::try_new(ExecutionEffectAckParts {
        attempt_id: intent.attempt_id().to_owned(),
        idempotency_key: intent.idempotency_key().to_owned(),
        effect_plan_hash: ContentHash::new("sha256:unplanned"),
        canonical_provider_ack: r#"{"status":"committed"}"#.into(),
    })?;
    assert_eq!(
        store
            .record_execution_effect_ack(ack)
            .await
            .expect_err("unplanned ACK must fail closed"),
        OutcomeStoreError::ExecutionEffectAckPlanMissing {
            attempt_id: intent.attempt_id().to_owned()
        }
    );
    Ok(())
}

#[tokio::test]
async fn planned_terminal_resolution_requires_exact_effect_ack() -> TestResult {
    let directory = tempfile::tempdir()?;
    let database_path = directory.path().join("effect-terminal-missing.sqlite3");
    let intent = planned_intent("attempt-effect-terminal")?;
    let store = DurableOutcomeStore::bootstrap_new(&database_path).await?;
    store.stage_execution_intent(intent.clone()).await?;
    let (receipt, evidence, evidence_hash) = canonical_execution_outcome_with_ack(&intent, None)?;
    assert_eq!(
        store
            .record(intent.attempt_id(), receipt, evidence, evidence_hash)
            .await?,
        OutcomeRecordResult::Recorded
    );
    assert!(matches!(
        store
            .resolve_execution_intent(intent.attempt_id(), intent.idempotency_key())
            .await,
        Err(OutcomeStoreError::ExecutionEffectAckIntentMissing { .. })
    ));

    let database_path = directory.path().join("effect-terminal-confirmed.sqlite3");
    let intent = planned_intent("attempt-effect-terminal-confirmed")?;
    let store = DurableOutcomeStore::bootstrap_new(&database_path).await?;
    store.stage_execution_intent(intent.clone()).await?;
    let ack = effect_ack(&intent, r#"{"status":"committed","target":"terminal"}"#)?;
    store.record_execution_effect_ack(ack.clone()).await?;
    let (receipt, evidence, evidence_hash) =
        canonical_execution_outcome_with_ack(&intent, Some(&ack))?;
    store
        .stage_intent(
            intent.attempt_id(),
            receipt.clone(),
            evidence.clone(),
            evidence_hash.clone(),
        )
        .await?;
    assert_eq!(
        store
            .commit_staged_intent_and_resolve_execution(
                intent.attempt_id().to_owned(),
                receipt,
                evidence,
                evidence_hash,
                intent.idempotency_key().to_owned(),
            )
            .await?,
        OutcomeRecordResult::Recorded
    );
    assert!(store.pending_execution_intents().await?.is_empty());
    Ok(())
}

fn planned_intent(attempt_id: &str) -> Result<crate::ExecutionIntent, Box<dyn std::error::Error>> {
    execution_intent_with_effect_plan(
        attempt_id,
        "correlation-effect-ack",
        Some(
            r#"{"after_content_hash":"sha256:after","before_content_hash":null,"mode":"create","operation":"native_write","schema_version":1,"secondary_effect_policy":null,"target_path":"/tmp/effect","tool":"write"}"#
                .into(),
        ),
    )
}

fn effect_ack(
    intent: &crate::ExecutionIntent,
    canonical_provider_ack: &str,
) -> Result<ExecutionEffectAck, Box<dyn std::error::Error>> {
    Ok(ExecutionEffectAck::try_new(ExecutionEffectAckParts {
        attempt_id: intent.attempt_id().to_owned(),
        idempotency_key: intent.idempotency_key().to_owned(),
        effect_plan_hash: intent.effect_plan_hash().expect("planned intent").clone(),
        canonical_provider_ack: canonical_provider_ack.into(),
    })?)
}
