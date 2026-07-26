//! SQLite schema initialization and verification for durable V2 memory stores.

use sqlx::SqlitePool;

use super::DurableDatabase;
use super::DurableStorageError;

impl DurableDatabase {
    pub(super) async fn initialize_schema(&self) -> Result<(), DurableStorageError> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS hepta_v2_schema (
                singleton INTEGER PRIMARY KEY CHECK (singleton = 1),
                version INTEGER NOT NULL
            )",
        )
        .execute(&self.pool)
        .await
        .map_err(|error| DurableStorageError::persistence("initialize durable V2 schema", error))?;
        sqlx::query(
            "INSERT INTO hepta_v2_schema (singleton, version)
             VALUES (1, ?)
             ON CONFLICT(singleton) DO NOTHING",
        )
        .bind(self.integrity.schema_version())
        .execute(&self.pool)
        .await
        .map_err(|error| {
            DurableStorageError::persistence("initialize durable V2 schema version", error)
        })?;
        if let Some(key_id) = self.integrity.key_id() {
            sqlx::query(
                "CREATE TABLE IF NOT EXISTS hepta_v2_integrity (
                    singleton INTEGER PRIMARY KEY CHECK (singleton = 1),
                    algorithm TEXT NOT NULL,
                    key_id TEXT NOT NULL
                )",
            )
            .execute(&self.pool)
            .await
            .map_err(|error| {
                DurableStorageError::persistence("initialize durable V2 integrity metadata", error)
            })?;
            sqlx::query(
                "INSERT INTO hepta_v2_integrity (singleton, algorithm, key_id)
                 VALUES (1, ?, ?)
                 ON CONFLICT(singleton) DO NOTHING",
            )
            .bind(self.integrity.algorithm())
            .bind(key_id)
            .execute(&self.pool)
            .await
            .map_err(|error| {
                DurableStorageError::persistence("bind durable V2 integrity key", error)
            })?;
        }
        for statement in [
            "CREATE TABLE IF NOT EXISTS hepta_v2_write_lock (
                singleton INTEGER PRIMARY KEY CHECK (singleton = 1),
                generation INTEGER NOT NULL
            )",
            "INSERT INTO hepta_v2_write_lock (singleton, generation)
             VALUES (1, 0)
             ON CONFLICT(singleton) DO NOTHING",
            "CREATE TABLE IF NOT EXISTS hepta_v2_outcome_records (
                receipt_id TEXT PRIMARY KEY,
                attempt_id TEXT NOT NULL UNIQUE,
                payload_json TEXT NOT NULL,
                storage_hash TEXT NOT NULL
            )",
            "CREATE TABLE IF NOT EXISTS hepta_v2_outcome_intents (
                attempt_id TEXT PRIMARY KEY,
                receipt_id TEXT NOT NULL UNIQUE,
                state TEXT NOT NULL CHECK (state IN ('pending', 'committed')),
                payload_json TEXT NOT NULL,
                storage_hash TEXT NOT NULL
            )",
            "CREATE TABLE IF NOT EXISTS hepta_v2_execution_intents (
                attempt_id TEXT PRIMARY KEY,
                idempotency_key TEXT NOT NULL UNIQUE,
                payload_json TEXT NOT NULL,
                storage_hash TEXT NOT NULL
            )",
            "CREATE TABLE IF NOT EXISTS hepta_v2_execution_effect_acks (
                attempt_id TEXT PRIMARY KEY,
                idempotency_key TEXT NOT NULL UNIQUE,
                effect_plan_hash TEXT NOT NULL,
                payload_json TEXT NOT NULL,
                storage_hash TEXT NOT NULL
            )",
            "CREATE TABLE IF NOT EXISTS hepta_v2_preference_genesis (
                preference_id TEXT NOT NULL,
                subject_id TEXT NOT NULL,
                payload_json TEXT NOT NULL,
                storage_hash TEXT NOT NULL,
                PRIMARY KEY (preference_id, subject_id)
            )",
            "CREATE TABLE IF NOT EXISTS hepta_v2_preference_heads (
                preference_id TEXT NOT NULL,
                subject_id TEXT NOT NULL,
                payload_json TEXT NOT NULL,
                storage_hash TEXT NOT NULL,
                PRIMARY KEY (preference_id, subject_id)
            )",
            "CREATE TABLE IF NOT EXISTS hepta_v2_preference_transitions (
                sequence INTEGER PRIMARY KEY AUTOINCREMENT,
                transition_id TEXT NOT NULL UNIQUE,
                evidence_id TEXT NOT NULL UNIQUE,
                receipt_id TEXT NOT NULL UNIQUE,
                preference_id TEXT NOT NULL,
                subject_id TEXT NOT NULL,
                payload_json TEXT NOT NULL,
                storage_hash TEXT NOT NULL
            )",
            "CREATE INDEX IF NOT EXISTS
                idx_hepta_v2_preference_transition_key_sequence
             ON hepta_v2_preference_transitions (
                preference_id,
                subject_id,
                sequence
             )",
        ] {
            sqlx::query(statement)
                .execute(&self.pool)
                .await
                .map_err(|error| {
                    DurableStorageError::persistence("initialize durable V2 schema", error)
                })?;
        }

        self.verify_schema().await
    }

    pub(super) async fn verify_schema(&self) -> Result<(), DurableStorageError> {
        let version =
            sqlx::query_scalar::<_, i64>("SELECT version FROM hepta_v2_schema WHERE singleton = 1")
                .fetch_optional(&self.pool)
                .await
                .map_err(|error| {
                    DurableStorageError::persistence("read durable V2 schema version", error)
                })?
                .ok_or_else(|| {
                    DurableStorageError::corrupt("durable V2 schema version row is missing")
                })?;
        if version != self.integrity.schema_version() {
            return Err(DurableStorageError::corrupt(format!(
                "unsupported durable V2 schema version {version}; expected {} for the requested integrity mode",
                self.integrity.schema_version()
            )));
        }
        self.verify_integrity_binding().await?;
        for statement in [
            "SELECT singleton, generation FROM hepta_v2_write_lock LIMIT 0",
            "SELECT receipt_id, attempt_id, payload_json, storage_hash
             FROM hepta_v2_outcome_records LIMIT 0",
            "SELECT attempt_id, receipt_id, state, payload_json, storage_hash
             FROM hepta_v2_outcome_intents LIMIT 0",
            "SELECT attempt_id, idempotency_key, payload_json, storage_hash
             FROM hepta_v2_execution_intents LIMIT 0",
            "SELECT attempt_id, idempotency_key, effect_plan_hash, payload_json, storage_hash
             FROM hepta_v2_execution_effect_acks LIMIT 0",
        ] {
            sqlx::query(statement)
                .execute(&self.pool)
                .await
                .map_err(|error| {
                    DurableStorageError::corrupt(format!(
                        "durable V2 schema shape is invalid: {error}"
                    ))
                })?;
        }
        let lock_rows = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM hepta_v2_write_lock
             WHERE singleton = 1 AND generation >= 0",
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|error| {
            DurableStorageError::persistence("verify durable V2 write-lock row", error)
        })?;
        if lock_rows != 1 {
            return Err(DurableStorageError::corrupt(format!(
                "durable V2 write-lock row count is {lock_rows}, expected 1"
            )));
        }
        for (column, unique_count) in [
            (
                "receipt_id",
                unique_single_column_index_count(
                    &self.pool,
                    "hepta_v2_outcome_records",
                    "receipt_id",
                )
                .await?,
            ),
            (
                "attempt_id",
                unique_single_column_index_count(
                    &self.pool,
                    "hepta_v2_outcome_records",
                    "attempt_id",
                )
                .await?,
            ),
        ] {
            if unique_count < 1 {
                return Err(DurableStorageError::corrupt(format!(
                    "durable outcome column {column} lacks a single-column unique constraint"
                )));
            }
        }
        for column in ["attempt_id", "receipt_id"] {
            if unique_single_column_index_count(&self.pool, "hepta_v2_outcome_intents", column)
                .await?
                < 1
            {
                return Err(DurableStorageError::corrupt(format!(
                    "durable outcome intent {column} lacks a single-column unique constraint"
                )));
            }
        }
        for column in ["attempt_id", "idempotency_key"] {
            if unique_single_column_index_count(&self.pool, "hepta_v2_execution_intents", column)
                .await?
                < 1
            {
                return Err(DurableStorageError::corrupt(format!(
                    "durable execution intent {column} lacks a single-column unique constraint"
                )));
            }
        }
        for column in ["attempt_id", "idempotency_key"] {
            if unique_single_column_index_count(
                &self.pool,
                "hepta_v2_execution_effect_acks",
                column,
            )
            .await?
                < 1
            {
                return Err(DurableStorageError::corrupt(format!(
                    "durable execution effect ACK {column} lacks a single-column unique constraint"
                )));
            }
        }
        Ok(())
    }

    async fn verify_integrity_binding(&self) -> Result<(), DurableStorageError> {
        let Some(expected_key_id) = self.integrity.key_id() else {
            return Ok(());
        };
        let row = sqlx::query_as::<_, (String, String)>(
            "SELECT algorithm, key_id
             FROM hepta_v2_integrity
             WHERE singleton = 1",
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|error| {
            DurableStorageError::corrupt(format!(
                "durable V2 integrity metadata is missing or invalid: {error}"
            ))
        })?
        .ok_or_else(|| {
            DurableStorageError::corrupt("durable V2 integrity metadata row is missing")
        })?;
        if row.0 != self.integrity.algorithm() || row.1 != expected_key_id {
            return Err(DurableStorageError::corrupt(
                "durable V2 integrity key or algorithm does not match the database",
            ));
        }
        Ok(())
    }

    pub(super) async fn verify_durability(&self) -> Result<(), DurableStorageError> {
        let journal_mode = sqlx::query_scalar::<_, String>("PRAGMA journal_mode")
            .fetch_one(&self.pool)
            .await
            .map_err(|error| DurableStorageError::persistence("read SQLite journal mode", error))?;
        if !journal_mode.eq_ignore_ascii_case("wal") {
            return Err(DurableStorageError::corrupt(format!(
                "durable V2 database journal mode is {journal_mode}, expected wal"
            )));
        }

        let synchronous = sqlx::query_scalar::<_, i64>("PRAGMA synchronous")
            .fetch_one(&self.pool)
            .await
            .map_err(|error| {
                DurableStorageError::persistence("read SQLite synchronous mode", error)
            })?;
        if synchronous != 2 {
            return Err(DurableStorageError::corrupt(format!(
                "durable V2 database synchronous mode is {synchronous}, expected FULL (2)"
            )));
        }

        let foreign_keys = sqlx::query_scalar::<_, i64>("PRAGMA foreign_keys")
            .fetch_one(&self.pool)
            .await
            .map_err(|error| {
                DurableStorageError::persistence("read SQLite foreign-key mode", error)
            })?;
        if foreign_keys != 1 {
            return Err(DurableStorageError::corrupt(
                "durable V2 database foreign-key enforcement is disabled",
            ));
        }

        let integrity = sqlx::query_scalar::<_, String>("PRAGMA integrity_check")
            .fetch_all(&self.pool)
            .await
            .map_err(|error| {
                DurableStorageError::persistence("run SQLite integrity check", error)
            })?;
        if integrity.as_slice() != ["ok"] {
            return Err(DurableStorageError::corrupt(format!(
                "SQLite integrity check failed: {}",
                integrity.join("; ")
            )));
        }
        Ok(())
    }
}

async fn unique_single_column_index_count(
    pool: &SqlitePool,
    table: &str,
    column: &str,
) -> Result<i64, DurableStorageError> {
    sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*)
         FROM pragma_index_list(?) AS index_list
         WHERE index_list.\"unique\" = 1
           AND (
             SELECT COUNT(*)
             FROM pragma_index_info(index_list.name)
           ) = 1
           AND (
             SELECT name
             FROM pragma_index_info(index_list.name)
             LIMIT 1
           ) = ?",
    )
    .bind(table)
    .bind(column)
    .fetch_one(pool)
    .await
    .map_err(|error| DurableStorageError::persistence("verify durable outcome uniqueness", error))
}
