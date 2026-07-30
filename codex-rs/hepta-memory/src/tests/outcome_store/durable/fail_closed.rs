use super::super::tempfile;
use super::*;

pub(super) async fn durable_outcome_storage_hash_tampering_fails_closed_on_reopen() -> TestResult {
    let directory = tempfile::tempdir()?;
    let database_path = directory.path().join("v2-memory.sqlite3");
    let receipt = outcome_receipt(
        "receipt-durable-tamper",
        "sha256:receipt-durable-tamper",
        "sha256:outcome-durable-tamper",
    )?;
    {
        let store = DurableOutcomeStore::bootstrap_new(&database_path).await?;
        store
            .record(
                "attempt-durable-tamper",
                receipt,
                "{}",
                ContentHash::new("sha256:evidence-durable-tamper"),
            )
            .await?;
    }

    let options = sqlx::sqlite::SqliteConnectOptions::new().filename(&database_path);
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(options)
        .await?;
    sqlx::query(
        "UPDATE hepta_v2_outcome_records
         SET storage_hash = 'sha256:tampered'
         WHERE receipt_id = 'receipt-durable-tamper'",
    )
    .execute(&pool)
    .await?;
    pool.close().await;

    assert!(matches!(
        DurableOutcomeStore::open_existing(&database_path).await,
        Err(OutcomeStoreError::Corrupt { detail })
            if detail.contains("storage hash mismatch")
    ));
    Ok(())
}

pub(super) async fn durable_outcome_open_existing_never_creates_missing_path() -> TestResult {
    let directory = tempfile::tempdir()?;
    let parent = directory.path().join("missing-parent");
    let database_path = parent.join("v2-memory.sqlite3");

    assert!(matches!(
        DurableOutcomeStore::open_existing(&database_path).await,
        Err(OutcomeStoreError::Persistence {
            operation: "inspect durable database parent",
            ..
        })
    ));
    assert!(!parent.exists());
    Ok(())
}

pub(super) async fn durable_outcome_bootstrap_refuses_to_overwrite_existing_file() -> TestResult {
    let directory = tempfile::tempdir()?;
    let database_path = directory.path().join("v2-memory.sqlite3");
    std::fs::write(&database_path, b"sentinel")?;

    assert!(matches!(
        DurableOutcomeStore::bootstrap_new(&database_path).await,
        Err(OutcomeStoreError::Persistence {
            operation: "reserve new database file",
            ..
        })
    ));
    assert_eq!(std::fs::read(&database_path)?, b"sentinel");
    Ok(())
}

pub(super) async fn durable_outcome_store_fails_closed_after_database_deletion() -> TestResult {
    let directory = tempfile::tempdir()?;
    let database_path = directory.path().join("v2-memory.sqlite3");
    let store = DurableOutcomeStore::bootstrap_new(&database_path).await?;
    std::fs::remove_file(&database_path)?;

    assert!(matches!(
        store.read_by_attempt("attempt-after-delete").await,
        Err(OutcomeStoreError::Corrupt { detail })
            if detail.contains("deleted or replaced")
    ));
    Ok(())
}

pub(super) async fn durable_outcome_store_fails_closed_after_database_replacement() -> TestResult {
    let directory = tempfile::tempdir()?;
    let database_path = directory.path().join("v2-memory.sqlite3");
    let displaced_path = directory.path().join("displaced.sqlite3");
    let store = DurableOutcomeStore::bootstrap_new(&database_path).await?;
    std::fs::rename(&database_path, &displaced_path)?;
    std::fs::write(&database_path, b"replacement")?;

    assert!(matches!(
        store.read_by_attempt("attempt-after-replace").await,
        Err(OutcomeStoreError::Corrupt { detail })
            if detail.contains("deleted or replaced")
    ));
    Ok(())
}

pub(super) async fn durable_outcome_open_existing_rejects_partial_schema_without_healing()
-> TestResult {
    let directory = tempfile::tempdir()?;
    let database_path = directory.path().join("partial.sqlite3");
    let options = sqlx::sqlite::SqliteConnectOptions::new()
        .filename(&database_path)
        .create_if_missing(true);
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(options)
        .await?;
    sqlx::query(
        "CREATE TABLE hepta_v2_schema (
            singleton INTEGER PRIMARY KEY,
            version INTEGER NOT NULL
        )",
    )
    .execute(&pool)
    .await?;
    sqlx::query("INSERT INTO hepta_v2_schema (singleton, version) VALUES (1, 1)")
        .execute(&pool)
        .await?;
    pool.close().await;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&database_path, std::fs::Permissions::from_mode(0o600))?;
    }

    assert!(matches!(
        DurableOutcomeStore::open_existing(&database_path).await,
        Err(OutcomeStoreError::Corrupt { detail })
            if detail.contains("unsupported durable V2 schema version")
    ));
    let options = sqlx::sqlite::SqliteConnectOptions::new().filename(&database_path);
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(options)
        .await?;
    let created_tables = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM sqlite_schema
         WHERE type = 'table' AND name IN (
             'hepta_v2_write_lock',
             'hepta_v2_outcome_records'
         )",
    )
    .fetch_one(&pool)
    .await?;
    assert_eq!(created_tables, 0);
    pool.close().await;
    Ok(())
}

#[cfg(unix)]
pub(super) async fn durable_outcome_open_existing_rejects_final_symlink() -> TestResult {
    use std::os::unix::fs::symlink;

    let directory = tempfile::tempdir()?;
    let database_path = directory.path().join("v2-memory.sqlite3");
    let symlink_path = directory.path().join("alias.sqlite3");
    drop(DurableOutcomeStore::bootstrap_new(&database_path).await?);
    symlink(&database_path, &symlink_path)?;

    assert!(matches!(
        DurableOutcomeStore::open_existing(&symlink_path).await,
        Err(OutcomeStoreError::Corrupt { detail })
            if detail.contains("not a regular file")
    ));
    Ok(())
}
