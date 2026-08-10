use std::path::Path;

use serde::Serialize;
use sqlx::Connection;
use sqlx::Row;
use sqlx::SqliteConnection;
use sqlx::sqlite::SqliteConnectOptions;

use crate::QualificationError;
use crate::Surface;
use crate::digest::sha256;
use crate::durable::read_private_bounded;
use crate::durable::same_file_snapshot;
use crate::durable::verify_private_regular;
use crate::durable::write_private_new;
use crate::request::canonical_json;

const DATABASE_FILENAME: &str = "hepta_evidence_1.sqlite";
const MAX_DATABASE_BYTES: usize = 32 * 1024 * 1024;
const MIGRATION_BYTES: usize = 880;
const MIGRATION_SHA256: &str = "fa57edd23c048ee384f0a3dce8d06675102ef4fe18a529129b39e031b4cddf15";
const SCHEMA_BYTES: usize = 19_498;
const SCHEMA_SHA256: &str = "8023bf9b009f3de55a226f5a1e142e594147ab1ab22c08073c6cf92295bfa18f";

pub(crate) async fn snapshot_and_read(
    sqlite_root: &Path,
    surface: Surface,
    snapshot_root: &Path,
) -> Result<(String, Vec<ProductReceiptRow>), QualificationError> {
    let database_sha256 = snapshot_database(sqlite_root, surface, snapshot_root)?;
    let snapshot = snapshot_root.join(format!("{}-{DATABASE_FILENAME}", surface.as_str()));
    let rows = read_rows(&snapshot).await?;
    Ok((database_sha256, rows))
}

fn snapshot_database(
    sqlite_root: &Path,
    surface: Surface,
    snapshot_root: &Path,
) -> Result<String, QualificationError> {
    let source = sqlite_root.join(DATABASE_FILENAME);
    let wal = sqlite_root.join(format!("{DATABASE_FILENAME}-wal"));
    verify_private_regular(&source)?;
    verify_private_regular(&wal)?;
    if std::fs::metadata(&wal)?.len() != 0 {
        return Err(invalid(
            "product evidence WAL is not empty after clean child exit",
        ));
    }
    let before = std::fs::metadata(&source)?;
    let bytes = read_private_bounded(&source, MAX_DATABASE_BYTES)?;
    let after = std::fs::metadata(&source)?;
    if !same_file_snapshot(&before, &after) || !bytes.starts_with(b"SQLite format 3\0") {
        return Err(invalid(
            "product evidence database changed or lacks SQLite header",
        ));
    }
    let database_sha256 = sha256(&bytes);
    let destination = snapshot_root.join(format!("{}-{DATABASE_FILENAME}", surface.as_str()));
    write_private_new(&destination, &bytes)?;
    Ok(database_sha256)
}

#[expect(
    clippy::disallowed_methods,
    reason = "qualification-only verifier opens an immutable private snapshot and never production state"
)]
async fn read_rows(path: &Path) -> Result<Vec<ProductReceiptRow>, QualificationError> {
    let options = SqliteConnectOptions::new()
        .filename(path)
        .read_only(true)
        .immutable(true)
        .create_if_missing(false);
    let mut connection = SqliteConnection::connect_with(&options).await?;
    sqlx::query("PRAGMA query_only = ON")
        .execute(&mut connection)
        .await?;
    let quick_check: String = sqlx::query_scalar("PRAGMA quick_check")
        .fetch_one(&mut connection)
        .await?;
    if quick_check != "ok"
        || !sqlx::query("PRAGMA foreign_key_check")
            .fetch_all(&mut connection)
            .await?
            .is_empty()
    {
        return Err(invalid("product evidence SQLite integrity checks failed"));
    }
    verify_schema(&mut connection).await?;
    verify_counts(&mut connection).await?;
    let rows = sqlx::query(
        "SELECT seq,receipt_id,action_id,thread_id,turn_id,call_id,admission_decision_id,authorization_decision_id,schema_version,payload_json,payload_sha256 FROM governance_receipts ORDER BY seq",
    )
    .fetch_all(&mut connection)
    .await?
    .into_iter()
    .map(|row| {
        Ok(ProductReceiptRow {
            seq: row.try_get("seq")?,
            receipt_id: row.try_get("receipt_id")?,
            action_id: row.try_get("action_id")?,
            thread_id: row.try_get("thread_id")?,
            turn_id: row.try_get("turn_id")?,
            call_id: row.try_get("call_id")?,
            admission_decision_id: row.try_get("admission_decision_id")?,
            authorization_decision_id: row.try_get("authorization_decision_id")?,
            schema_version: row.try_get("schema_version")?,
            payload_json: row.try_get("payload_json")?,
            payload_sha256: row.try_get("payload_sha256")?,
        })
    })
    .collect::<Result<Vec<_>, sqlx::Error>>()?;
    connection.close().await?;
    Ok(rows)
}

async fn verify_schema(connection: &mut SqliteConnection) -> Result<(), QualificationError> {
    let rows = sqlx::query("SELECT type,name,tbl_name,sql FROM sqlite_schema ORDER BY type,name")
        .fetch_all(&mut *connection)
        .await?
        .into_iter()
        .map(|row| {
            Ok(SchemaRow {
                kind: row.try_get("type")?,
                name: row.try_get("name")?,
                table: row.try_get("tbl_name")?,
                sql: row.try_get("sql")?,
            })
        })
        .collect::<Result<Vec<_>, sqlx::Error>>()?;
    let bytes = canonical_json(&rows)?;
    if rows.len() != 50 || bytes.len() != SCHEMA_BYTES || sha256(&bytes) != SCHEMA_SHA256 {
        return Err(invalid(
            "product evidence schema differs from frozen catalog",
        ));
    }
    let migrations = sqlx::query(
        "SELECT version,description,success,lower(hex(checksum)) AS checksum_hex FROM _sqlx_migrations ORDER BY version",
    )
    .fetch_all(&mut *connection)
    .await?
    .into_iter()
    .map(|row| {
        Ok(MigrationRow {
            checksum_hex: row.try_get("checksum_hex")?,
            description: row.try_get("description")?,
            success: row.try_get("success")?,
            version: row.try_get("version")?,
        })
    })
    .collect::<Result<Vec<_>, sqlx::Error>>()?;
    let bytes = canonical_json(&migrations)?;
    if migrations.len() != 5 || bytes.len() != MIGRATION_BYTES || sha256(&bytes) != MIGRATION_SHA256
    {
        return Err(invalid(
            "product evidence migrations differ from frozen vector",
        ));
    }
    Ok(())
}

async fn verify_counts(connection: &mut SqliteConnection) -> Result<(), QualificationError> {
    let row = sqlx::query(
        "SELECT (SELECT count(*) FROM governance_decisions) decisions,(SELECT count(*) FROM governance_receipts) receipts,(SELECT count(*) FROM provider_invocation_intents) intents,(SELECT count(*) FROM provider_invocation_terminals) terminals,(SELECT count(*) FROM memory_mutation_shadow_observations) memories,(SELECT count(*) FROM channel_ingress_events) ingress_events,(SELECT count(*) FROM channel_ingress_receipts) ingress_receipts",
    )
    .fetch_one(connection)
    .await?;
    let exact = row.try_get::<i64, _>("decisions")? == 4
        && row.try_get::<i64, _>("receipts")? == 2
        && row.try_get::<i64, _>("intents")? == 4
        && row.try_get::<i64, _>("terminals")? == 4
        && row.try_get::<i64, _>("memories")? == 0
        && row.try_get::<i64, _>("ingress_events")? == 0
        && row.try_get::<i64, _>("ingress_receipts")? == 0;
    if !exact {
        return Err(invalid(
            "product evidence table cardinalities differ from exact trial",
        ));
    }
    Ok(())
}

pub(crate) struct ProductReceiptRow {
    pub(crate) action_id: String,
    pub(crate) admission_decision_id: String,
    pub(crate) authorization_decision_id: String,
    pub(crate) call_id: String,
    pub(crate) payload_json: String,
    pub(crate) payload_sha256: String,
    pub(crate) receipt_id: String,
    pub(crate) schema_version: i64,
    pub(crate) seq: i64,
    pub(crate) thread_id: String,
    pub(crate) turn_id: String,
}

#[derive(Serialize)]
struct SchemaRow {
    #[serde(rename = "type")]
    kind: String,
    name: String,
    #[serde(rename = "tbl_name")]
    table: String,
    sql: Option<String>,
}

#[derive(Serialize)]
struct MigrationRow {
    checksum_hex: String,
    description: String,
    success: i64,
    version: i64,
}

fn invalid(message: impl Into<String>) -> QualificationError {
    QualificationError::Invalid(message.into())
}
