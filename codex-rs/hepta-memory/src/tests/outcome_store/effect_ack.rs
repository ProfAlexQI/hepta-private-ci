use super::*;

use crate::DurableOutcomeStore;
use crate::ExecutionEffectAck;
use crate::ExecutionEffectAckParts;
use crate::ExecutionEffectAckRecordResult;
use crate::ExecutionIntentStageResult;
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
