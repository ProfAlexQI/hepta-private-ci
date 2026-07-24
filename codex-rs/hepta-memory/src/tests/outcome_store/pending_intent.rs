use super::*;

use crate::DurableOutcomeStore;
use crate::OutcomeIntentStageResult;
use crate::OutcomeIntentState;

#[tokio::test]
async fn pending_exact_intent_survives_reopen_and_reconciles_without_reminting() -> TestResult {
    let directory = tempfile::tempdir()?;
    let database_path = directory.path().join("v2-pending-intent.sqlite3");
    let receipt = outcome_receipt(
        "receipt-intent-pending",
        "sha256:receipt-intent-pending",
        "sha256:outcome-intent-pending",
    )?;
    let evidence = ContentHash::new("sha256:evidence-intent-pending");
    let envelope = r#"{"terminal":"succeeded","intent":"pending"}"#;

    {
        let store = DurableOutcomeStore::bootstrap_new(&database_path).await?;
        assert_eq!(
            store
                .stage_intent(
                    "attempt-intent-pending",
                    receipt.clone(),
                    envelope,
                    evidence.clone(),
                )
                .await?,
            OutcomeIntentStageResult::Pending
        );
        let intents = store.pending_intents().await?;
        let [pending] = intents.as_slice() else {
            panic!("one exact intent must be durable");
        };
        assert_eq!(pending.state(), OutcomeIntentState::Pending);
        assert_eq!(pending.record().receipt(), &receipt);
        assert_eq!(pending.record().canonical_evidence(), envelope);
    }

    let reopened = DurableOutcomeStore::open_existing(&database_path).await?;
    let recovered = reopened
        .pending_intent("attempt-intent-pending")
        .await?
        .expect("pending exact material must survive reopen");
    assert_eq!(recovered.state(), OutcomeIntentState::Pending);
    assert_eq!(
        reopened
            .record(
                "attempt-intent-pending",
                receipt.clone(),
                envelope,
                evidence,
            )
            .await?,
        OutcomeRecordResult::Recorded
    );
    assert!(reopened.pending_intents().await?.is_empty());
    assert_eq!(
        reopened
            .read_by_attempt("attempt-intent-pending")
            .await?
            .expect("reconciled record")
            .receipt(),
        &receipt
    );
    Ok(())
}

#[tokio::test]
async fn committed_unacknowledged_intent_recovers_as_exact_replay() -> TestResult {
    let directory = tempfile::tempdir()?;
    let database_path = directory.path().join("v2-committed-intent.sqlite3");
    let receipt = outcome_receipt(
        "receipt-intent-committed",
        "sha256:receipt-intent-committed",
        "sha256:outcome-intent-committed",
    )?;
    let evidence = ContentHash::new("sha256:evidence-intent-committed");
    let envelope = r#"{"terminal":"succeeded","intent":"committed"}"#;

    {
        let store = DurableOutcomeStore::bootstrap_new(&database_path).await?;
        store
            .stage_intent(
                "attempt-intent-committed",
                receipt.clone(),
                envelope,
                evidence.clone(),
            )
            .await?;
        assert_eq!(
            store
                .commit_staged_intent(
                    "attempt-intent-committed".into(),
                    receipt.clone(),
                    envelope.into(),
                    evidence.clone(),
                )
                .await?,
            OutcomeRecordResult::Recorded
        );
    }

    let reopened = DurableOutcomeStore::open_existing(&database_path).await?;
    assert_eq!(
        reopened
            .pending_intent("attempt-intent-committed")
            .await?
            .expect("lost ACK marker")
            .state(),
        OutcomeIntentState::Committed
    );
    assert_eq!(
        reopened
            .record("attempt-intent-committed", receipt, envelope, evidence,)
            .await?,
        OutcomeRecordResult::AlreadyRecorded
    );
    assert!(reopened.pending_intents().await?.is_empty());
    Ok(())
}

#[tokio::test]
async fn pending_intent_storage_tampering_fails_closed_on_open() -> TestResult {
    let directory = tempfile::tempdir()?;
    let database_path = directory.path().join("v2-tampered-intent.sqlite3");
    let store = DurableOutcomeStore::bootstrap_new(&database_path).await?;
    store
        .stage_intent(
            "attempt-intent-tamper",
            outcome_receipt(
                "receipt-intent-tamper",
                "sha256:receipt-intent-tamper",
                "sha256:outcome-intent-tamper",
            )?,
            "{}",
            ContentHash::new("sha256:evidence-intent-tamper"),
        )
        .await?;
    drop(store);

    let options = sqlx::sqlite::SqliteConnectOptions::new().filename(&database_path);
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(options)
        .await?;
    sqlx::query(
        "UPDATE hepta_v2_outcome_intents
         SET storage_hash = 'sha256:tampered'
         WHERE attempt_id = 'attempt-intent-tamper'",
    )
    .execute(&pool)
    .await?;
    pool.close().await;

    assert!(matches!(
        DurableOutcomeStore::open_existing(&database_path).await,
        Err(OutcomeStoreError::Corrupt { detail })
            if detail.contains("outcome producer intent storage hash mismatch")
    ));
    Ok(())
}
