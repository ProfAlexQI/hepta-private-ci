use super::*;

use crate::DurablePreferenceStore;

#[tokio::test]
async fn durable_preference_open_existing_never_creates_missing_path() -> TestResult {
    let directory = private_tempdir()?;
    let parent = directory.path().join("missing-parent");
    let database_path = parent.join("v2-memory.sqlite3");

    assert!(matches!(
        DurablePreferenceStore::open_existing(&database_path).await,
        Err(PreferenceCasError::Persistence {
            operation: "inspect durable database parent",
            ..
        })
    ));
    assert!(!parent.exists());
    Ok(())
}

#[tokio::test]
async fn durable_preference_bootstrap_refuses_existing_and_preserves_it() -> TestResult {
    let directory = private_tempdir()?;
    let database_path = directory.path().join("v2-memory.sqlite3");
    std::fs::write(&database_path, b"preference-sentinel")?;

    assert!(matches!(
        DurablePreferenceStore::bootstrap_new(&database_path).await,
        Err(PreferenceCasError::Persistence {
            operation: "reserve new database file",
            ..
        })
    ));
    assert_eq!(std::fs::read(&database_path)?, b"preference-sentinel");
    Ok(())
}

#[tokio::test]
async fn durable_preference_open_existing_rejects_invalid_schema_without_healing() -> TestResult {
    let directory = private_tempdir()?;
    let partial_path = directory.path().join("partial.sqlite3");
    let options = sqlx::sqlite::SqliteConnectOptions::new()
        .filename(&partial_path)
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
        std::fs::set_permissions(&partial_path, std::fs::Permissions::from_mode(0o600))?;
    }

    assert!(
        DurablePreferenceStore::open_existing(&partial_path)
            .await
            .is_err()
    );
    let options = sqlx::sqlite::SqliteConnectOptions::new().filename(&partial_path);
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(options)
        .await?;
    let created_tables = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM sqlite_schema
         WHERE type = 'table' AND name IN (
             'hepta_v2_write_lock',
             'hepta_v2_preference_genesis',
             'hepta_v2_preference_heads',
             'hepta_v2_preference_transitions'
         )",
    )
    .fetch_one(&pool)
    .await?;
    assert_eq!(created_tables, 0);
    pool.close().await;

    let invalid_path = directory.path().join("invalid.sqlite3");
    drop(DurablePreferenceStore::bootstrap_new(&invalid_path).await?);
    let options = sqlx::sqlite::SqliteConnectOptions::new().filename(&invalid_path);
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(options)
        .await?;
    sqlx::query("UPDATE hepta_v2_schema SET version = 99 WHERE singleton = 1")
        .execute(&pool)
        .await?;
    pool.close().await;
    assert!(matches!(
        DurablePreferenceStore::open_existing(&invalid_path).await,
        Err(PreferenceCasError::Corrupt { detail })
            if detail.contains("unsupported durable V2 schema version 99")
    ));
    let options = sqlx::sqlite::SqliteConnectOptions::new().filename(&invalid_path);
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(options)
        .await?;
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT version FROM hepta_v2_schema WHERE singleton = 1")
            .fetch_one(&pool)
            .await?,
        99
    );
    pool.close().await;
    Ok(())
}

#[tokio::test]
async fn durable_preference_store_fails_closed_after_path_deletion_or_replacement() -> TestResult {
    let directory = private_tempdir()?;
    let deleted_path = directory.path().join("deleted.sqlite3");
    let deleted = DurablePreferenceStore::bootstrap_new(&deleted_path).await?;
    std::fs::remove_file(&deleted_path)?;
    assert!(matches!(
        deleted
            .read_document(
                &PreferenceId::new("preference-after-delete"),
                &PrincipalId::new("subject-after-delete"),
            )
            .await,
        Err(PreferenceCasError::Corrupt { detail })
            if detail.contains("deleted or replaced")
    ));

    let replaced_path = directory.path().join("replaced.sqlite3");
    let displaced_path = directory.path().join("displaced.sqlite3");
    let replaced = DurablePreferenceStore::bootstrap_new(&replaced_path).await?;
    std::fs::rename(&replaced_path, &displaced_path)?;
    std::fs::write(&replaced_path, b"replacement")?;
    assert!(matches!(
        replaced
            .read_document(
                &PreferenceId::new("preference-after-replace"),
                &PrincipalId::new("subject-after-replace"),
            )
            .await,
        Err(PreferenceCasError::Corrupt { detail })
            if detail.contains("deleted or replaced")
    ));
    Ok(())
}
