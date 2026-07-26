use super::*;

use sha2::Digest;

use crate::DurableIntegrityKey;
use crate::DurableOutcomeStore;

fn durable_integrity_key(byte: u8) -> DurableIntegrityKey {
    DurableIntegrityKey::from_bytes([byte; 32])
}

mod fail_closed;
#[tokio::test]
async fn durable_outcome_monotonic_state_advances_and_survives_reopen() -> TestResult {
    let directory = tempfile::tempdir()?;
    let database_path = directory.path().join("v2-monotonic-outcome.sqlite3");
    let store =
        DurableOutcomeStore::bootstrap_new_keyed(&database_path, durable_integrity_key(0x21))
            .await?;
    let before = store.monotonic_state().await?;
    store
        .record(
            "attempt-monotonic",
            outcome_receipt(
                "receipt-monotonic",
                "sha256:receipt-monotonic",
                "sha256:outcome-monotonic",
            )?,
            r#"{"terminal":"succeeded"}"#,
            ContentHash::new("sha256:evidence-monotonic"),
        )
        .await?;
    let after = store.monotonic_state().await?;
    assert!(after.generation() > before.generation());
    assert_ne!(after.state_hash(), before.state_hash());
    drop(store);
    let reopened =
        DurableOutcomeStore::open_existing_keyed(&database_path, durable_integrity_key(0x21))
            .await?;
    assert_eq!(reopened.monotonic_state().await?, after);
    Ok(())
}

#[tokio::test]
async fn durable_outcomes_recover_and_preserve_exact_replay() -> TestResult {
    let directory = tempfile::tempdir()?;
    let database_path = directory.path().join("v2-memory.sqlite3");
    let receipt = outcome_receipt(
        "receipt-durable-replay",
        "sha256:receipt-durable-replay",
        "sha256:outcome-durable-replay",
    )?;
    let evidence = ContentHash::new("sha256:evidence-durable-replay");
    let envelope = r#"{"terminal":"succeeded","durable":true}"#;

    {
        let store = DurableOutcomeStore::bootstrap_new(&database_path).await?;
        assert_eq!(
            store
                .record(
                    "attempt-durable-replay",
                    receipt.clone(),
                    envelope,
                    evidence.clone(),
                )
                .await?,
            OutcomeRecordResult::Recorded
        );
    }

    assert!(matches!(
        DurableOutcomeStore::bootstrap_new(&database_path).await,
        Err(OutcomeStoreError::Persistence {
            operation: "reserve new database file",
            ..
        })
    ));
    let reopened = DurableOutcomeStore::open_existing(&database_path).await?;
    let by_receipt = reopened
        .read_by_receipt(receipt.id())
        .await?
        .expect("durable receipt should recover");
    let by_attempt = reopened
        .read_by_attempt("attempt-durable-replay")
        .await?
        .expect("durable attempt index should recover");
    assert_eq!(by_receipt, by_attempt);
    assert_eq!(by_receipt.receipt(), &receipt);
    assert_eq!(by_receipt.canonical_evidence(), envelope);
    assert_eq!(
        reopened
            .record("attempt-durable-replay", receipt, envelope, evidence)
            .await?,
        OutcomeRecordResult::AlreadyRecorded
    );
    Ok(())
}

#[tokio::test]
async fn keyed_durable_outcomes_recover_only_with_the_exact_key() -> TestResult {
    let directory = tempfile::tempdir()?;
    let database_path = directory.path().join("v2-keyed-memory.sqlite3");
    let receipt = outcome_receipt(
        "receipt-keyed-replay",
        "sha256:receipt-keyed-replay",
        "sha256:outcome-keyed-replay",
    )?;
    {
        let store =
            DurableOutcomeStore::bootstrap_new_keyed(&database_path, durable_integrity_key(0x31))
                .await?;
        store
            .record(
                "attempt-keyed-replay",
                receipt.clone(),
                r#"{"terminal":"succeeded","integrity":"keyed"}"#,
                ContentHash::new("sha256:evidence-keyed-replay"),
            )
            .await?;
    }

    let reopened =
        DurableOutcomeStore::open_existing_keyed(&database_path, durable_integrity_key(0x31))
            .await?;
    assert_eq!(
        reopened
            .read_by_attempt("attempt-keyed-replay")
            .await?
            .expect("keyed outcome must recover")
            .receipt(),
        &receipt
    );
    drop(reopened);

    assert!(matches!(
        DurableOutcomeStore::open_existing_keyed(&database_path, durable_integrity_key(0x32)).await,
        Err(OutcomeStoreError::Corrupt { detail })
            if detail.contains("integrity key or algorithm")
    ));
    assert!(matches!(
        DurableOutcomeStore::open_existing(&database_path).await,
        Err(OutcomeStoreError::Corrupt { detail })
            if detail.contains("requested integrity mode")
    ));
    Ok(())
}

#[tokio::test]
async fn keyed_durable_outcomes_reject_plain_hash_forgery() -> TestResult {
    let directory = tempfile::tempdir()?;
    let database_path = directory.path().join("v2-keyed-forgery.sqlite3");
    {
        let store =
            DurableOutcomeStore::bootstrap_new_keyed(&database_path, durable_integrity_key(0x41))
                .await?;
        store
            .record(
                "attempt-keyed-forgery",
                outcome_receipt(
                    "receipt-keyed-forgery",
                    "sha256:receipt-keyed-forgery",
                    "sha256:outcome-keyed-forgery",
                )?,
                r#"{"terminal":"succeeded"}"#,
                ContentHash::new("sha256:evidence-keyed-forgery"),
            )
            .await?;
    }

    let options = sqlx::sqlite::SqliteConnectOptions::new().filename(&database_path);
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(options)
        .await?;
    let payload_json = sqlx::query_scalar::<_, String>(
        "SELECT payload_json
         FROM hepta_v2_outcome_records
         WHERE receipt_id = 'receipt-keyed-forgery'",
    )
    .fetch_one(&pool)
    .await?;
    let mut payload: serde_json::Value = serde_json::from_str(&payload_json)?;
    payload["canonical_evidence"] = serde_json::Value::String(r#"{"forged":true}"#.into());
    let payload_json = serde_json::to_string(&payload)?;
    let forged_tag = format!(
        "hmac-sha256:{:x}",
        sha2::Sha256::digest(payload_json.as_bytes())
    );
    sqlx::query(
        "UPDATE hepta_v2_outcome_records
         SET payload_json = ?, storage_hash = ?
         WHERE receipt_id = 'receipt-keyed-forgery'",
    )
    .bind(payload_json)
    .bind(forged_tag)
    .execute(&pool)
    .await?;
    pool.close().await;

    assert!(matches!(
        DurableOutcomeStore::open_existing_keyed(&database_path, durable_integrity_key(0x41)).await,
        Err(OutcomeStoreError::Corrupt { detail })
            if detail.contains("keyed integrity verification failed")
    ));
    Ok(())
}

#[tokio::test]
async fn keyed_and_compatibility_databases_cannot_be_cross_opened() -> TestResult {
    let directory = tempfile::tempdir()?;
    let database_path = directory.path().join("v2-compatibility-memory.sqlite3");
    drop(DurableOutcomeStore::bootstrap_new(&database_path).await?);

    assert!(matches!(
        DurableOutcomeStore::open_existing_keyed(&database_path, durable_integrity_key(0x51)).await,
        Err(OutcomeStoreError::Corrupt { detail })
            if detail.contains("requested integrity mode")
    ));
    Ok(())
}

#[tokio::test]
async fn durable_outcome_conflicts_survive_reopen() -> TestResult {
    let directory = tempfile::tempdir()?;
    let database_path = directory.path().join("v2-memory.sqlite3");
    let first = outcome_receipt(
        "receipt-durable-conflict",
        "sha256:receipt-durable-conflict",
        "sha256:outcome-durable-conflict",
    )?;
    DurableOutcomeStore::bootstrap_new(&database_path)
        .await?
        .record(
            "attempt-durable-conflict",
            first,
            "{}",
            ContentHash::new("sha256:evidence-durable-conflict"),
        )
        .await?;

    let reopened = DurableOutcomeStore::open_existing(&database_path).await?;
    let second = outcome_receipt(
        "receipt-durable-second",
        "sha256:receipt-durable-second",
        "sha256:outcome-durable-second",
    )?;
    assert!(matches!(
        reopened
            .record(
                "attempt-durable-conflict",
                second,
                "{\"changed\":true}",
                ContentHash::new("sha256:evidence-durable-second"),
            )
            .await,
        Err(OutcomeStoreError::AttemptAlreadyFinalized {
            existing_receipt,
            ..
        }) if existing_receipt == ReceiptId::new("receipt-durable-conflict")
    ));
    Ok(())
}

#[tokio::test]
async fn independently_opened_durable_outcome_stores_finalize_one_attempt_once() -> TestResult {
    let directory = tempfile::tempdir()?;
    let database_path = directory.path().join("v2-memory.sqlite3");
    let first_store = DurableOutcomeStore::bootstrap_new(&database_path).await?;
    let second_store = DurableOutcomeStore::open_existing(&database_path).await?;
    let first = outcome_receipt(
        "receipt-durable-race-a",
        "sha256:receipt-durable-race-a",
        "sha256:outcome-durable-race-a",
    )?;
    let second = outcome_receipt(
        "receipt-durable-race-b",
        "sha256:receipt-durable-race-b",
        "sha256:outcome-durable-race-b",
    )?;

    let (first_outcome, second_outcome) = tokio::join!(
        first_store.record(
            "attempt-durable-race",
            first,
            "{\"winner\":\"a\"}",
            ContentHash::new("sha256:evidence-durable-race-a"),
        ),
        second_store.record(
            "attempt-durable-race",
            second,
            "{\"winner\":\"b\"}",
            ContentHash::new("sha256:evidence-durable-race-b"),
        ),
    );
    let outcomes = [first_outcome, second_outcome];
    assert_eq!(
        outcomes
            .iter()
            .filter(|outcome| matches!(outcome, Ok(OutcomeRecordResult::Recorded)))
            .count(),
        1
    );
    assert_eq!(
        outcomes
            .iter()
            .filter(|outcome| matches!(
                outcome,
                Err(OutcomeStoreError::AttemptAlreadyFinalized { .. })
            ))
            .count(),
        1
    );
    Ok(())
}

#[tokio::test]
async fn durable_outcome_storage_hash_tampering_fails_closed_on_reopen() -> TestResult {
    fail_closed::durable_outcome_storage_hash_tampering_fails_closed_on_reopen().await
}

#[tokio::test]
async fn durable_outcome_open_existing_never_creates_missing_path() -> TestResult {
    fail_closed::durable_outcome_open_existing_never_creates_missing_path().await
}

#[tokio::test]
async fn durable_outcome_bootstrap_refuses_to_overwrite_existing_file() -> TestResult {
    fail_closed::durable_outcome_bootstrap_refuses_to_overwrite_existing_file().await
}

#[tokio::test]
async fn durable_outcome_store_fails_closed_after_database_deletion() -> TestResult {
    fail_closed::durable_outcome_store_fails_closed_after_database_deletion().await
}

#[tokio::test]
async fn durable_outcome_store_fails_closed_after_database_replacement() -> TestResult {
    fail_closed::durable_outcome_store_fails_closed_after_database_replacement().await
}

#[tokio::test]
async fn durable_outcome_open_existing_rejects_partial_schema_without_healing() -> TestResult {
    fail_closed::durable_outcome_open_existing_rejects_partial_schema_without_healing().await
}

#[cfg(unix)]
#[tokio::test]
async fn durable_outcome_open_existing_rejects_final_symlink() -> TestResult {
    fail_closed::durable_outcome_open_existing_rejects_final_symlink().await
}
