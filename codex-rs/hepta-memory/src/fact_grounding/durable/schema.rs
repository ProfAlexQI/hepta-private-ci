use super::*;

const REQUIRED_GROUNDING_SCHEMA_OBJECTS: &[(&str, &str)] = &[
    ("cognitive_fact_grounding_migrations", "table"),
    ("cognitive_fact_grounding_migrations_no_delete", "trigger"),
    ("cognitive_fact_grounding_migrations_no_update", "trigger"),
    ("kg_revision_fact_grounding_receipts", "table"),
    (
        "kg_revision_fact_grounding_receipts_binding_guard",
        "trigger",
    ),
    ("kg_revision_fact_grounding_receipts_digest_lookup", "index"),
    ("kg_revision_fact_grounding_receipts_no_delete", "trigger"),
    ("kg_revision_fact_grounding_receipts_no_update", "trigger"),
    ("kg_revision_fact_grounding_receipts_source_lookup", "index"),
    ("kg_revision_fact_grounding_spans", "table"),
    ("kg_revision_fact_grounding_spans_digest_lookup", "index"),
    ("kg_revision_fact_grounding_spans_fact_guard", "trigger"),
    ("kg_revision_fact_grounding_spans_fact_lookup", "index"),
    ("kg_revision_fact_grounding_spans_no_delete", "trigger"),
    ("kg_revision_fact_grounding_spans_no_update", "trigger"),
    ("kg_revision_fact_grounding_spans_ordinal_guard", "trigger"),
    ("kg_revision_fact_grounding_spans_range_guard", "trigger"),
    ("kg_revision_fact_grounding_spans_total_guard", "trigger"),
];

const REQUIRED_GROUNDING_SCHEMA_ORACLE_SHA256: &str =
    "67bbe2776e2bae9ace02e2a258b878159183735075688334cde1ef1f81dba44a";

pub(super) async fn ensure(pool: &SqlitePool) -> Result<(), CognitiveStoreError> {
    let mut transaction = pool
        .begin_with("BEGIN IMMEDIATE")
        .await
        .map_err(unavailable)?;
    let exists: bool = sqlx::query_scalar(
        "SELECT EXISTS(
             SELECT 1 FROM sqlite_schema
             WHERE type = 'table'
               AND name = 'cognitive_fact_grounding_migrations'
         )",
    )
    .fetch_one(&mut *transaction)
    .await
    .map_err(unavailable)?;
    let migration_checksum = Sha256Digest::for_bytes(COMPONENT_MIGRATION_SQL.as_bytes());
    if !exists {
        sqlx::raw_sql(COMPONENT_MIGRATION_SQL)
            .execute(&mut *transaction)
            .await
            .map_err(unavailable)?;
        sqlx::query(
            "INSERT INTO cognitive_fact_grounding_migrations (
                version, description, checksum_sha256, applied_at_unix_seconds
             ) VALUES (?, ?, ?, unixepoch())",
        )
        .bind(COMPONENT_MIGRATION_VERSION)
        .bind(COMPONENT_MIGRATION_DESCRIPTION)
        .bind(migration_checksum.as_str())
        .execute(&mut *transaction)
        .await
        .map_err(unavailable)?;
    } else {
        verify_migration_ledger_connection(&mut *transaction, migration_checksum.as_str()).await?;
    }
    transaction.commit().await.map_err(unavailable)?;
    verify(pool).await
}

pub(super) async fn verify(pool: &SqlitePool) -> Result<(), CognitiveStoreError> {
    let mut transaction = pool.begin().await.map_err(unavailable)?;
    verify_tx(&mut transaction).await?;
    transaction.rollback().await.map_err(unavailable)
}

pub(super) async fn verify_tx(
    transaction: &mut Transaction<'_, Sqlite>,
) -> Result<(), CognitiveStoreError> {
    verify_schema_oracle_connection(&mut **transaction).await?;
    verify_migration_ledger_connection(
        &mut **transaction,
        Sha256Digest::for_bytes(COMPONENT_MIGRATION_SQL.as_bytes()).as_str(),
    )
    .await
}

async fn verify_migration_ledger_connection(
    connection: &mut SqliteConnection,
    expected_checksum: &str,
) -> Result<(), CognitiveStoreError> {
    let rows = sqlx::query(
        "SELECT version, description, checksum_sha256
         FROM cognitive_fact_grounding_migrations
         ORDER BY version",
    )
    .fetch_all(&mut *connection)
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
            "fact-grounding component migration ledger is incomplete or has unknown entries"
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
            "fact-grounding component migration 0011 does not match the current lineage"
                .to_string(),
        ));
    }
    Ok(())
}

async fn verify_schema_oracle_connection(
    connection: &mut SqliteConnection,
) -> Result<(), CognitiveStoreError> {
    let mut parts = Vec::with_capacity(REQUIRED_GROUNDING_SCHEMA_OBJECTS.len());
    for (name, expected_type) in REQUIRED_GROUNDING_SCHEMA_OBJECTS {
        let row = sqlx::query("SELECT type, sql FROM sqlite_schema WHERE name = ?")
            .bind(name)
            .fetch_optional(&mut *connection)
            .await
            .map_err(unavailable)?
            .ok_or_else(|| {
                CognitiveStoreError::Corrupt(format!(
                    "required fact-grounding schema object `{name}` is missing"
                ))
            })?;
        let object_type: String = row.try_get("type").map_err(unavailable)?;
        let sql: Option<String> = row.try_get("sql").map_err(unavailable)?;
        let Some(sql) = sql.filter(|value| !value.is_empty()) else {
            return Err(CognitiveStoreError::Corrupt(format!(
                "required fact-grounding schema object `{name}` has no definition"
            )));
        };
        if object_type != *expected_type {
            return Err(CognitiveStoreError::Corrupt(format!(
                "required fact-grounding schema object `{name}` has the wrong type"
            )));
        }
        parts.push(((*name).to_string(), object_type, sql));
    }
    parts.sort_by(|left, right| left.0.cmp(&right.0));
    let mut hasher = Sha256::new();
    super::super::frame_part(
        &mut hasher,
        b"hepta:cognitive:fact-grounding-required-schema-oracle:v1",
    );
    super::super::frame_part(
        &mut hasher,
        &u64::try_from(parts.len()).unwrap_or(u64::MAX).to_be_bytes(),
    );
    for (name, object_type, sql) in &parts {
        super::super::frame_part(&mut hasher, name.as_bytes());
        super::super::frame_part(&mut hasher, object_type.as_bytes());
        super::super::frame_part(&mut hasher, sql.as_bytes());
    }
    let actual = Sha256Digest::from_sha256_output(hasher.finalize());
    if actual.as_str() != REQUIRED_GROUNDING_SCHEMA_ORACLE_SHA256 {
        return Err(CognitiveStoreError::Corrupt(format!(
            "fact-grounding schema oracle mismatch: {}",
            actual.as_str()
        )));
    }
    Ok(())
}
