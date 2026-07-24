use super::*;

use sha2::Digest;

pub(super) async fn durable_preference_wal_document_must_match_transition_next_state() -> TestResult
{
    let directory = tempfile::tempdir()?;
    let database_path = directory.path().join("v2-memory.sqlite3");
    let preference = PreferenceId::new("preference-durable-wal-binding");
    let subject = PrincipalId::new("subject-durable-wal-binding");
    let genesis = preference_document(0, "sha256:g", "reducer.v1", "{}");
    let next_document = preference_document(1, "sha256:next", "reducer.v1", "{\"v\":1}");
    let receipt = outcome_receipt(
        "receipt-durable-wal-binding",
        "sha256:receipt-durable-wal-binding",
    )?;
    let store = DurablePreferenceStore::bootstrap_new(&database_path).await?;
    store
        .get_or_init_genesis(preference.clone(), subject.clone(), genesis.clone())
        .await?;
    store
        .commit_evidenced(
            transition(
                "transition-durable-wal-binding",
                preference,
                subject,
                genesis.state().clone(),
                next_document.state().clone(),
                &receipt,
            )?,
            next_document,
        )
        .await?;
    drop(store);

    let options = sqlx::sqlite::SqliteConnectOptions::new().filename(&database_path);
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(options)
        .await?;
    let payload_json = sqlx::query_scalar::<_, String>(
        "SELECT payload_json
         FROM hepta_v2_preference_transitions
         WHERE transition_id = 'transition-durable-wal-binding'",
    )
    .fetch_one(&pool)
    .await?;
    let mut payload: serde_json::Value = serde_json::from_str(&payload_json)?;
    payload["document"]["state"]["content_hash"] =
        serde_json::Value::String("sha256:forged-document-state".into());
    let payload_json = serde_json::to_string(&payload)?;
    let storage_hash = format!("sha256:{:x}", sha2::Sha256::digest(payload_json.as_bytes()));
    sqlx::query(
        "UPDATE hepta_v2_preference_transitions
         SET payload_json = ?, storage_hash = ?
         WHERE transition_id = 'transition-durable-wal-binding'",
    )
    .bind(payload_json)
    .bind(storage_hash)
    .execute(&pool)
    .await?;
    pool.close().await;

    assert!(matches!(
        DurablePreferenceStore::open_existing(&database_path).await,
        Err(PreferenceCasError::Corrupt { detail })
            if detail.contains("document does not match committed next state")
    ));
    Ok(())
}

pub(super) async fn durable_preference_head_must_equal_immutable_wal_replay() -> TestResult {
    let directory = tempfile::tempdir()?;
    let database_path = directory.path().join("v2-memory.sqlite3");
    DurablePreferenceStore::bootstrap_new(&database_path)
        .await?
        .get_or_init_genesis(
            PreferenceId::new("preference-durable-projection"),
            PrincipalId::new("subject-durable-projection"),
            preference_document(0, "sha256:g", "reducer.v1", "{}"),
        )
        .await?;

    let options = sqlx::sqlite::SqliteConnectOptions::new().filename(&database_path);
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(options)
        .await?;
    sqlx::query("DELETE FROM hepta_v2_preference_heads")
        .execute(&pool)
        .await?;
    pool.close().await;

    assert!(matches!(
        DurablePreferenceStore::open_existing(&database_path).await,
        Err(PreferenceCasError::Corrupt { detail })
            if detail.contains("head projection")
    ));
    Ok(())
}
