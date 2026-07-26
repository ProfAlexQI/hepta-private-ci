use super::*;

use crate::DurableIntegrityKey;
use crate::DurablePreferenceStore;

fn durable_integrity_key(byte: u8) -> DurableIntegrityKey {
    DurableIntegrityKey::from_bytes([byte; 32])
}

mod wal_integrity;

#[tokio::test]
async fn durable_preference_monotonic_state_advances_and_survives_reopen() -> TestResult {
    let directory = tempfile::tempdir()?;
    let database_path = directory.path().join("v2-monotonic-preference.sqlite3");
    let store =
        DurablePreferenceStore::bootstrap_new_keyed(&database_path, durable_integrity_key(0x51))
            .await?;
    let before = store.monotonic_state().await?;
    store
        .get_or_init_genesis(
            PreferenceId::new("preference-monotonic"),
            PrincipalId::new("subject-monotonic"),
            preference_document(0, "sha256:genesis-monotonic", "reducer.v1", "{}"),
        )
        .await?;
    let after = store.monotonic_state().await?;
    assert!(after.generation() > before.generation());
    assert_ne!(after.state_hash(), before.state_hash());
    drop(store);
    let reopened =
        DurablePreferenceStore::open_existing_keyed(&database_path, durable_integrity_key(0x51))
            .await?;
    assert_eq!(reopened.monotonic_state().await?, after);
    Ok(())
}

#[tokio::test]
async fn durable_preference_wal_replays_head_and_historical_idempotency() -> TestResult {
    let directory = tempfile::tempdir()?;
    let database_path = directory.path().join("v2-memory.sqlite3");
    let preference = PreferenceId::new("preference-durable-replay");
    let subject = PrincipalId::new("subject-durable-replay");
    let genesis = preference_document(0, "sha256:g", "reducer.v1", "{\"score\":0}");
    let first_document = preference_document(1, "sha256:d1", "reducer.v1", "{\"score\":1}");
    let first_receipt = outcome_receipt("receipt-durable-pref-1", "sha256:receipt-durable-pref-1")?;
    let first = transition(
        "transition-durable-pref-1",
        preference.clone(),
        subject.clone(),
        genesis.state().clone(),
        first_document.state().clone(),
        &first_receipt,
    )?;
    {
        let store = DurablePreferenceStore::bootstrap_new(&database_path).await?;
        assert_eq!(
            store
                .get_or_init_genesis(preference.clone(), subject.clone(), genesis.clone(),)
                .await?,
            PreferenceGenesisOutcome::Initialized
        );
        store
            .commit_evidenced(first.clone(), first_document.clone())
            .await?;
        assert_eq!(
            store
                .get_or_init_genesis(preference.clone(), subject.clone(), genesis.clone(),)
                .await?,
            PreferenceGenesisOutcome::AlreadyInitialized
        );
        let conflicting_genesis =
            preference_document(0, "sha256:other-genesis", "reducer.v1", "{}");
        assert_eq!(
            store
                .get_or_init_genesis(
                    preference.clone(),
                    subject.clone(),
                    conflicting_genesis.clone(),
                )
                .await
                .expect_err("immutable genesis drift must conflict"),
            PreferenceCasError::GenesisConflict {
                existing: Box::new(genesis.clone()),
                attempted: Box::new(conflicting_genesis),
            }
        );
    }
    let reopened = DurablePreferenceStore::open_existing(&database_path).await?;
    assert_eq!(
        reopened
            .get_or_init_genesis(preference.clone(), subject.clone(), genesis,)
            .await?,
        PreferenceGenesisOutcome::AlreadyInitialized
    );
    assert_eq!(
        reopened.read_document(&preference, &subject).await?,
        Some(first_document.clone())
    );
    assert_eq!(
        reopened
            .commit_evidenced(first, first_document.clone())
            .await?,
        PreferenceDocumentCommitOutcome::AlreadyCommitted {
            document: first_document
        }
    );
    Ok(())
}

#[tokio::test]
async fn keyed_preference_wal_recovers_only_with_the_exact_key() -> TestResult {
    let directory = tempfile::tempdir()?;
    let database_path = directory.path().join("v2-keyed-preference.sqlite3");
    let preference = PreferenceId::new("preference-keyed-replay");
    let subject = PrincipalId::new("subject-keyed-replay");
    let genesis = preference_document(0, "sha256:keyed-genesis", "reducer.v1", "{}");
    {
        let store = DurablePreferenceStore::bootstrap_new_keyed(
            &database_path,
            durable_integrity_key(0x61),
        )
        .await?;
        store
            .get_or_init_genesis(preference.clone(), subject.clone(), genesis.clone())
            .await?;
    }

    let reopened =
        DurablePreferenceStore::open_existing_keyed(&database_path, durable_integrity_key(0x61))
            .await?;
    assert_eq!(
        reopened.read_document(&preference, &subject).await?,
        Some(genesis)
    );
    drop(reopened);

    assert!(matches!(
        DurablePreferenceStore::open_existing_keyed(
            &database_path,
            durable_integrity_key(0x62),
        )
        .await,
        Err(PreferenceCasError::Corrupt { detail })
            if detail.contains("integrity key or algorithm")
    ));
    Ok(())
}

#[tokio::test]
async fn durable_preference_receipt_single_use_survives_recovery() -> TestResult {
    let directory = tempfile::tempdir()?;
    let database_path = directory.path().join("v2-memory.sqlite3");
    let store = DurablePreferenceStore::bootstrap_new(&database_path).await?;
    let subject = PrincipalId::new("subject-durable-receipt");
    let first_preference = PreferenceId::new("preference-durable-receipt-a");
    let second_preference = PreferenceId::new("preference-durable-receipt-b");
    let genesis = preference_document(0, "sha256:g", "reducer.v1", "{}");
    store
        .get_or_init_genesis(first_preference.clone(), subject.clone(), genesis.clone())
        .await?;
    store
        .get_or_init_genesis(second_preference.clone(), subject.clone(), genesis.clone())
        .await?;
    let receipt = outcome_receipt(
        "receipt-durable-single-use",
        "sha256:receipt-durable-single-use",
    )?;
    let first_document = preference_document(1, "sha256:a", "reducer.v1", "{\"a\":1}");
    store
        .commit_evidenced(
            transition(
                "transition-durable-receipt-a",
                first_preference,
                subject.clone(),
                genesis.state().clone(),
                first_document.state().clone(),
                &receipt,
            )?,
            first_document,
        )
        .await?;
    drop(store);

    let reopened = DurablePreferenceStore::open_existing(&database_path).await?;
    let second_document = preference_document(1, "sha256:b", "reducer.v1", "{\"b\":1}");
    assert!(matches!(
        reopened
            .commit_evidenced(
                transition(
                    "transition-durable-receipt-b",
                    second_preference,
                    subject,
                    genesis.state().clone(),
                    second_document.state().clone(),
                    &receipt,
                )?,
                second_document,
            )
            .await,
        Err(PreferenceCasError::ReceiptReuseConflict { receipt, .. })
            if receipt == ReceiptId::new("receipt-durable-single-use")
    ));
    Ok(())
}

#[tokio::test]
async fn independently_opened_durable_stores_serialize_one_exact_cas_winner() -> TestResult {
    let directory = tempfile::tempdir()?;
    let database_path = directory.path().join("v2-memory.sqlite3");
    let preference = PreferenceId::new("preference-durable-race");
    let subject = PrincipalId::new("subject-durable-race");
    let genesis = preference_document(0, "sha256:g", "reducer.v1", "{}");
    DurablePreferenceStore::bootstrap_new(&database_path)
        .await?
        .get_or_init_genesis(preference.clone(), subject.clone(), genesis.clone())
        .await?;

    let first_store = DurablePreferenceStore::open_existing(&database_path).await?;
    let second_store = DurablePreferenceStore::open_existing(&database_path).await?;
    let first_document = preference_document(1, "sha256:a", "reducer.v1", "{\"v\":1}");
    let second_document = preference_document(1, "sha256:b", "reducer.v1", "{\"v\":2}");
    let first_receipt = outcome_receipt("receipt-durable-race-a", "sha256:receipt-durable-race-a")?;
    let second_receipt =
        outcome_receipt("receipt-durable-race-b", "sha256:receipt-durable-race-b")?;
    let first = transition(
        "transition-durable-race-a",
        preference.clone(),
        subject.clone(),
        genesis.state().clone(),
        first_document.state().clone(),
        &first_receipt,
    )?;
    let second = transition(
        "transition-durable-race-b",
        preference,
        subject,
        genesis.state().clone(),
        second_document.state().clone(),
        &second_receipt,
    )?;

    let (first_outcome, second_outcome) = tokio::join!(
        first_store.commit_evidenced(first, first_document),
        second_store.commit_evidenced(second, second_document),
    );
    let outcomes = [first_outcome, second_outcome];
    assert_eq!(
        outcomes
            .iter()
            .filter(|outcome| matches!(
                outcome,
                Ok(PreferenceDocumentCommitOutcome::Committed { .. })
            ))
            .count(),
        1
    );
    assert_eq!(
        outcomes
            .iter()
            .filter(|outcome| matches!(outcome, Err(PreferenceCasError::StateConflict { .. })))
            .count(),
        1
    );
    Ok(())
}

#[tokio::test]
async fn durable_preference_storage_hash_tampering_fails_closed() -> TestResult {
    let directory = tempfile::tempdir()?;
    let database_path = directory.path().join("v2-memory.sqlite3");
    let preference = PreferenceId::new("preference-durable-tamper");
    let subject = PrincipalId::new("subject-durable-tamper");
    DurablePreferenceStore::bootstrap_new(&database_path)
        .await?
        .get_or_init_genesis(
            preference,
            subject,
            preference_document(0, "sha256:g", "reducer.v1", "{}"),
        )
        .await?;

    let options = sqlx::sqlite::SqliteConnectOptions::new().filename(&database_path);
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(options)
        .await?;
    sqlx::query(
        "UPDATE hepta_v2_preference_heads
         SET storage_hash = 'sha256:tampered'",
    )
    .execute(&pool)
    .await?;
    pool.close().await;

    assert!(matches!(
        DurablePreferenceStore::open_existing(&database_path).await,
        Err(PreferenceCasError::Corrupt { detail })
            if detail.contains("storage hash mismatch")
    ));
    Ok(())
}

#[tokio::test]
async fn durable_preference_wal_document_must_match_transition_next_state() -> TestResult {
    wal_integrity::durable_preference_wal_document_must_match_transition_next_state().await
}

#[tokio::test]
async fn durable_preference_head_must_equal_immutable_wal_replay() -> TestResult {
    wal_integrity::durable_preference_head_must_equal_immutable_wal_replay().await
}
