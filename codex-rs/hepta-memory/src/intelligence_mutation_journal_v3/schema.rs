use std::collections::BTreeSet;

use codex_hepta_contracts::Sha256Digest;
use sha2::Digest;
use sha2::Sha256;
use sqlx::Acquire;
use sqlx::Executor;
use sqlx::Row;
use sqlx::Sqlite;
use sqlx::SqlitePool;
use sqlx::Transaction;
use sqlx::sqlite::SqlitePoolOptions;

use crate::CognitiveStoreError;
use crate::cognitive_store::unavailable;

use super::COMPONENT_MIGRATION_DESCRIPTION;
use super::COMPONENT_MIGRATION_SQL;
use super::COMPONENT_MIGRATION_VERSION;

const REQUIRED_MUTATION_JOURNAL_SCHEMA_OBJECTS: &[(&str, &str)] = &[
    ("cognitive_intelligence_mutation_migrations", "table"),
    (
        "cognitive_intelligence_mutation_migrations_no_update",
        "trigger",
    ),
    (
        "cognitive_intelligence_mutation_migrations_no_delete",
        "trigger",
    ),
    ("cognitive_intelligence_mutation_operations", "table"),
    (
        "cognitive_intelligence_mutation_operations_no_update",
        "trigger",
    ),
    (
        "cognitive_intelligence_mutation_operations_no_delete",
        "trigger",
    ),
    (
        "cognitive_intelligence_mutation_operations_owner_lookup",
        "index",
    ),
    (
        "cognitive_intelligence_mutation_operations_binding_lookup",
        "index",
    ),
    ("cognitive_intelligence_mutation_transitions", "table"),
    (
        "cognitive_intelligence_mutation_transitions_no_update",
        "trigger",
    ),
    (
        "cognitive_intelligence_mutation_transitions_no_delete",
        "trigger",
    ),
    (
        "cognitive_intelligence_mutation_transitions_chain_guard",
        "trigger",
    ),
    (
        "cognitive_intelligence_mutation_transitions_digest_lookup",
        "index",
    ),
    (
        "cognitive_intelligence_mutation_transitions_phase_lookup",
        "index",
    ),
];

pub(super) async fn ensure(pool: &SqlitePool) -> Result<(), CognitiveStoreError> {
    let mut transaction = pool
        .begin_with("BEGIN IMMEDIATE")
        .await
        .map_err(unavailable)?;
    let exists: bool = sqlx::query_scalar(
        "SELECT EXISTS(
             SELECT 1 FROM sqlite_schema
             WHERE type = 'table'
               AND name = 'cognitive_intelligence_mutation_migrations'
         )",
    )
    .fetch_one(&mut *transaction)
    .await
    .map_err(unavailable)?;
    let checksum = Sha256Digest::for_bytes(COMPONENT_MIGRATION_SQL.as_bytes());
    if !exists {
        sqlx::raw_sql(COMPONENT_MIGRATION_SQL)
            .execute(&mut *transaction)
            .await
            .map_err(unavailable)?;
        sqlx::query(
            "INSERT INTO cognitive_intelligence_mutation_migrations (
                version, description, checksum_sha256, applied_at_unix_seconds
             ) VALUES (?, ?, ?, unixepoch())",
        )
        .bind(COMPONENT_MIGRATION_VERSION)
        .bind(COMPONENT_MIGRATION_DESCRIPTION)
        .bind(checksum.as_str())
        .execute(&mut *transaction)
        .await
        .map_err(unavailable)?;
    } else {
        verify_migration_ledger_tx(&mut transaction, checksum.as_str()).await?;
    }
    transaction.commit().await.map_err(unavailable)?;
    verify(pool).await
}

pub(super) async fn verify(pool: &SqlitePool) -> Result<(), CognitiveStoreError> {
    verify_migration_ledger(
        pool,
        Sha256Digest::for_bytes(COMPONENT_MIGRATION_SQL.as_bytes()).as_str(),
    )
    .await?;
    verify_schema_oracle(pool).await
}

async fn verify_migration_ledger_tx(
    transaction: &mut Transaction<'_, Sqlite>,
    expected_checksum: &str,
) -> Result<(), CognitiveStoreError> {
    let rows = sqlx::query(
        "SELECT version, description, checksum_sha256
         FROM cognitive_intelligence_mutation_migrations
         ORDER BY version",
    )
    .fetch_all(&mut **transaction)
    .await
    .map_err(unavailable)?;
    verify_migration_rows(&rows, expected_checksum)
}

async fn verify_migration_ledger(
    pool: &SqlitePool,
    expected_checksum: &str,
) -> Result<(), CognitiveStoreError> {
    let rows = sqlx::query(
        "SELECT version, description, checksum_sha256
         FROM cognitive_intelligence_mutation_migrations
         ORDER BY version",
    )
    .fetch_all(pool)
    .await
    .map_err(unavailable)?;
    verify_migration_rows(&rows, expected_checksum)
}

fn verify_migration_rows(
    rows: &[sqlx::sqlite::SqliteRow],
    expected_checksum: &str,
) -> Result<(), CognitiveStoreError> {
    if rows.len() != 1 {
        return Err(CognitiveStoreError::Corrupt(
            "intelligence mutation component migration ledger is incomplete or has unknown entries"
                .to_string(),
        ));
    }
    let version: i64 = rows[0].try_get("version").map_err(unavailable)?;
    let description: String = rows[0].try_get("description").map_err(unavailable)?;
    let checksum: String = rows[0].try_get("checksum_sha256").map_err(unavailable)?;
    if version != COMPONENT_MIGRATION_VERSION
        || description != COMPONENT_MIGRATION_DESCRIPTION
        || checksum != expected_checksum
    {
        return Err(CognitiveStoreError::Corrupt(
            "intelligence mutation component migration 0012 does not match the current lineage"
                .to_string(),
        ));
    }
    Ok(())
}

#[allow(
    clippy::disallowed_methods,
    reason = "the isolated in-memory schema oracle does not create a runtime state database"
)]
async fn verify_schema_oracle(pool: &SqlitePool) -> Result<(), CognitiveStoreError> {
    let expected_names = REQUIRED_MUTATION_JOURNAL_SCHEMA_OBJECTS
        .iter()
        .map(|(name, _)| *name)
        .collect::<BTreeSet<_>>();
    let actual_names = sqlx::query_scalar::<_, String>(
        "SELECT name
         FROM sqlite_schema
         WHERE name LIKE 'cognitive_intelligence_mutation_%'
         ORDER BY name",
    )
    .fetch_all(pool)
    .await
    .map_err(unavailable)?
    .into_iter()
    .collect::<BTreeSet<_>>();
    let actual_name_views = actual_names
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    if actual_name_views != expected_names {
        return Err(CognitiveStoreError::Corrupt(
            "intelligence mutation schema inventory contains missing or unknown objects"
                .to_string(),
        ));
    }

    let actual = schema_digest(pool).await?;
    let scratch = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .map_err(unavailable)?;
    sqlx::raw_sql(COMPONENT_MIGRATION_SQL)
        .execute(&scratch)
        .await
        .map_err(unavailable)?;
    let expected = schema_digest(&scratch).await?;
    scratch.close().await;
    if actual != expected {
        return Err(CognitiveStoreError::Corrupt(format!(
            "intelligence mutation schema oracle mismatch: expected {}, received {}",
            expected.as_str(),
            actual.as_str()
        )));
    }
    Ok(())
}

async fn schema_digest(pool: &SqlitePool) -> Result<Sha256Digest, CognitiveStoreError> {
    let mut definitions = Vec::with_capacity(REQUIRED_MUTATION_JOURNAL_SCHEMA_OBJECTS.len());
    for (name, expected_type) in REQUIRED_MUTATION_JOURNAL_SCHEMA_OBJECTS {
        let row = sqlx::query("SELECT type, sql FROM sqlite_schema WHERE name = ?")
            .bind(name)
            .fetch_optional(pool)
            .await
            .map_err(unavailable)?
            .ok_or_else(|| {
                CognitiveStoreError::Corrupt(format!(
                    "required intelligence mutation schema object `{name}` is missing"
                ))
            })?;
        let object_type: String = row.try_get("type").map_err(unavailable)?;
        let sql: Option<String> = row.try_get("sql").map_err(unavailable)?;
        let Some(sql) = sql.filter(|value| !value.is_empty()) else {
            return Err(CognitiveStoreError::Corrupt(format!(
                "required intelligence mutation schema object `{name}` has no definition"
            )));
        };
        if object_type != *expected_type {
            return Err(CognitiveStoreError::Corrupt(format!(
                "required intelligence mutation schema object `{name}` has the wrong type"
            )));
        }
        definitions.push(((*name).to_string(), object_type, sql));
    }
    definitions.sort_by(|left, right| left.0.cmp(&right.0));
    let mut hasher = Sha256::new();
    super::super::frame_part(
        &mut hasher,
        b"hepta:cognitive:intelligence-mutation-required-schema-oracle:v1",
    );
    super::super::frame_part(
        &mut hasher,
        &u64::try_from(definitions.len())
            .unwrap_or(u64::MAX)
            .to_be_bytes(),
    );
    for (name, object_type, sql) in definitions {
        super::super::frame_part(&mut hasher, name.as_bytes());
        super::super::frame_part(&mut hasher, object_type.as_bytes());
        super::super::frame_part(&mut hasher, sql.as_bytes());
    }
    Ok(Sha256Digest::from_sha256_output(hasher.finalize()))
}
