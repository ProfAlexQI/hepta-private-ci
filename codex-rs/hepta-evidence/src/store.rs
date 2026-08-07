use std::path::PathBuf;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use codex_hepta_contracts::ActionId;
use codex_hepta_contracts::GOVERNANCE_SCHEMA_VERSION;
use codex_hepta_contracts::GovernanceDecisionRecord;
use codex_hepta_contracts::GovernanceReceipt;
use codex_hepta_contracts::PolicyPhase;
use codex_hepta_contracts::ReceiptId;
use codex_state::SqliteConfig;
use sqlx::Row;
use sqlx::Sqlite;
use sqlx::SqlitePool;
use sqlx::Transaction;
use sqlx::sqlite::SqliteRow;

use crate::EvidenceError;
use crate::canonical::canonical_storage_payload;
use crate::canonical::invalid_record_as_corrupt;
use crate::canonical::verify_canonical_storage_payload;
use crate::canonical::verify_storage_payload_digest;
use crate::governance_validation::validate_decision;
use crate::governance_validation::validate_receipt_binding;
use crate::schema_validation::verify_foreign_keys;
use crate::schema_validation::verify_provider_host_bindings;
use crate::schema_validation::verify_quick_check;
use crate::schema_validation::verify_schema_manifest;

const EVIDENCE_DB_FILENAME: &str = "hepta_evidence_1.sqlite";
static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AppendDisposition {
    Inserted,
    AlreadyPresent,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StoredReceipt {
    pub seq: i64,
    pub receipt: GovernanceReceipt,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StoredActionEvidence {
    pub admission: Option<GovernanceDecisionRecord>,
    pub authorization: Option<GovernanceDecisionRecord>,
    pub receipt: Option<StoredReceipt>,
}

#[derive(Clone)]
pub struct HeptaEvidenceStore {
    pub(crate) pool: SqlitePool,
    path: PathBuf,
}

impl HeptaEvidenceStore {
    pub async fn open(sqlite: &SqliteConfig) -> Result<Self, EvidenceError> {
        let path = sqlite.home().join(EVIDENCE_DB_FILENAME);
        let pool = sqlite
            .open_durable_evidence_pool(&path)
            .await
            .map_err(classify_sqlx_error)?;
        if let Err(error) = verify_quick_check(&pool).await {
            pool.close().await;
            return Err(error);
        }
        if let Err(error) = MIGRATOR.run(&pool).await {
            pool.close().await;
            return Err(classify_migrate_error(error));
        }
        if let Err(error) = verify_schema_manifest(&pool).await {
            pool.close().await;
            return Err(error);
        }
        if let Err(error) = verify_foreign_keys(&pool).await {
            pool.close().await;
            return Err(error);
        }
        if let Err(error) = verify_provider_host_bindings(&pool).await {
            pool.close().await;
            return Err(error);
        }
        Ok(Self { pool, path })
    }

    pub fn path(&self) -> &std::path::Path {
        &self.path
    }

    pub async fn append_decision(
        &self,
        record: &GovernanceDecisionRecord,
    ) -> Result<AppendDisposition, EvidenceError> {
        validate_decision(record)?;
        let (payload_json, payload_sha256) = canonical_storage_payload(record)?;
        let mut transaction = self.pool.begin().await.map_err(classify_sqlx_error)?;
        let insert = sqlx::query(
            "INSERT INTO governance_decisions (
                decision_id, action_id, thread_id, turn_id, call_id, phase,
                schema_version, payload_json, payload_sha256, recorded_at_ms
             ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
             ON CONFLICT DO NOTHING",
        )
        .bind(record.decision_id.as_str())
        .bind(record.action.action_id.as_str())
        .bind(&record.action.thread_id)
        .bind(&record.action.turn_id)
        .bind(&record.action.call_id)
        .bind(record.phase.as_str())
        .bind(i64::from(GOVERNANCE_SCHEMA_VERSION))
        .bind(&payload_json)
        .bind(payload_sha256.as_str())
        .bind(now_millis()?)
        .execute(&mut *transaction)
        .await
        .map_err(classify_sqlx_error)?;
        let disposition = verify_decision(
            &mut transaction,
            record,
            &payload_json,
            payload_sha256.as_str(),
            insert.rows_affected() == 1,
        )
        .await?;
        transaction.commit().await.map_err(classify_sqlx_error)?;
        Ok(disposition)
    }

    pub async fn append_receipt(
        &self,
        receipt: &GovernanceReceipt,
    ) -> Result<AppendDisposition, EvidenceError> {
        validate_receipt_binding(receipt)?;
        let (payload_json, payload_sha256) = canonical_storage_payload(receipt)?;
        let mut transaction = self.pool.begin().await.map_err(classify_sqlx_error)?;
        ensure_decision(&mut transaction, &receipt.admission).await?;
        if let Some(authorization) = receipt.authorization.as_ref() {
            ensure_decision(&mut transaction, authorization).await?;
        }
        let action = receipt
            .authorization
            .as_ref()
            .map_or(&receipt.admission.action, |record| &record.action);
        let insert = sqlx::query(
            "INSERT INTO governance_receipts (
                receipt_id, action_id, thread_id, turn_id, call_id,
                admission_decision_id, admission_phase,
                authorization_decision_id, authorization_phase,
                schema_version, payload_json, payload_sha256, recorded_at_ms
             ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
             ON CONFLICT DO NOTHING",
        )
        .bind(receipt.receipt_id.as_str())
        .bind(receipt.action_id.as_str())
        .bind(&action.thread_id)
        .bind(&action.turn_id)
        .bind(&action.call_id)
        .bind(receipt.admission.decision_id.as_str())
        .bind(receipt.admission.phase.as_str())
        .bind(
            receipt
                .authorization
                .as_ref()
                .map(|record| record.decision_id.as_str()),
        )
        .bind(
            receipt
                .authorization
                .as_ref()
                .map(|record| record.phase.as_str()),
        )
        .bind(i64::from(GOVERNANCE_SCHEMA_VERSION))
        .bind(&payload_json)
        .bind(payload_sha256.as_str())
        .bind(now_millis()?)
        .execute(&mut *transaction)
        .await
        .map_err(classify_sqlx_error)?;
        let disposition = verify_receipt(
            &mut transaction,
            receipt,
            &payload_json,
            payload_sha256.as_str(),
            insert.rows_affected() == 1,
        )
        .await?;
        transaction.commit().await.map_err(classify_sqlx_error)?;
        Ok(disposition)
    }

    pub async fn get_receipt(
        &self,
        receipt_id: &ReceiptId,
    ) -> Result<Option<StoredReceipt>, EvidenceError> {
        let row = sqlx::query(
            "SELECT seq, receipt_id, action_id, thread_id, turn_id, call_id,
                    admission_decision_id, admission_phase,
                    authorization_decision_id, authorization_phase,
                    schema_version, payload_json, payload_sha256
             FROM governance_receipts WHERE receipt_id = ?",
        )
        .bind(receipt_id.as_str())
        .fetch_optional(&self.pool)
        .await
        .map_err(classify_sqlx_error)?;
        let Some(row) = row else {
            return Ok(None);
        };
        let receipt = decode_receipt_row(&row)?;
        self.verify_receipt_decision_references(&receipt).await?;
        Ok(Some(StoredReceipt {
            seq: row.get("seq"),
            receipt,
        }))
    }

    pub async fn get_action_evidence(
        &self,
        action_id: &ActionId,
    ) -> Result<StoredActionEvidence, EvidenceError> {
        let mut transaction = self.pool.begin().await.map_err(classify_sqlx_error)?;
        let evidence = load_action_evidence_in_transaction(&mut transaction, action_id).await?;
        transaction.commit().await.map_err(classify_sqlx_error)?;
        Ok(evidence)
    }

    pub async fn pending_action_count(&self) -> Result<i64, EvidenceError> {
        sqlx::query_scalar(
            "SELECT COUNT(DISTINCT decisions.action_id)
             FROM governance_decisions AS decisions
             LEFT JOIN governance_receipts AS receipts
               ON receipts.action_id = decisions.action_id
             WHERE receipts.action_id IS NULL",
        )
        .fetch_one(&self.pool)
        .await
        .map_err(classify_sqlx_error)
    }

    async fn verify_receipt_decision_references(
        &self,
        receipt: &GovernanceReceipt,
    ) -> Result<(), EvidenceError> {
        let admission = self
            .load_decision(&receipt.admission.decision_id)
            .await?
            .ok_or_else(|| {
                EvidenceError::Corrupt(
                    "receipt references a missing admission decision".to_string(),
                )
            })?;
        if admission != receipt.admission {
            return Err(EvidenceError::Corrupt(
                "receipt admission differs from authoritative decision row".to_string(),
            ));
        }
        if let Some(expected) = receipt.authorization.as_ref() {
            let authorization = self
                .load_decision(&expected.decision_id)
                .await?
                .ok_or_else(|| {
                    EvidenceError::Corrupt(
                        "receipt references a missing authorization decision".to_string(),
                    )
                })?;
            if authorization != *expected {
                return Err(EvidenceError::Corrupt(
                    "receipt authorization differs from authoritative decision row".to_string(),
                ));
            }
        }
        Ok(())
    }

    async fn load_decision(
        &self,
        decision_id: &codex_hepta_contracts::DecisionId,
    ) -> Result<Option<GovernanceDecisionRecord>, EvidenceError> {
        let row = sqlx::query(
            "SELECT decision_id, action_id, thread_id, turn_id, call_id, phase,
                    schema_version, payload_json, payload_sha256
             FROM governance_decisions WHERE decision_id = ?",
        )
        .bind(decision_id.as_str())
        .fetch_optional(&self.pool)
        .await
        .map_err(classify_sqlx_error)?;
        row.map(|row| decode_decision_row(&row)).transpose()
    }
}

pub(crate) async fn load_action_evidence_in_transaction(
    transaction: &mut Transaction<'_, Sqlite>,
    action_id: &ActionId,
) -> Result<StoredActionEvidence, EvidenceError> {
    let decision_rows = sqlx::query(
        "SELECT decision_id, action_id, thread_id, turn_id, call_id, phase,
                    schema_version, payload_json, payload_sha256
             FROM governance_decisions WHERE action_id = ? ORDER BY seq ASC",
    )
    .bind(action_id.as_str())
    .fetch_all(&mut **transaction)
    .await
    .map_err(classify_sqlx_error)?;
    let mut admission = None;
    let mut authorization = None;
    for row in decision_rows {
        let record = decode_decision_row(&row)?;
        let slot = match record.phase {
            PolicyPhase::Admission => &mut admission,
            PolicyPhase::Authorization => &mut authorization,
        };
        if slot.replace(record).is_some() {
            return Err(EvidenceError::Corrupt(
                "multiple decisions exist for one action phase".to_string(),
            ));
        }
    }
    let receipt_rows = sqlx::query(
        "SELECT seq, receipt_id, action_id, thread_id, turn_id, call_id,
                    admission_decision_id, admission_phase,
                    authorization_decision_id, authorization_phase,
                    schema_version, payload_json, payload_sha256
             FROM governance_receipts WHERE action_id = ?",
    )
    .bind(action_id.as_str())
    .fetch_all(&mut **transaction)
    .await
    .map_err(classify_sqlx_error)?;
    if receipt_rows.len() > 1 {
        return Err(EvidenceError::Corrupt(
            "multiple governance receipts exist for one action".to_string(),
        ));
    }
    let receipt = receipt_rows
        .into_iter()
        .next()
        .map(|row| {
            let seq = row.get("seq");
            decode_receipt_row(&row).map(|receipt| StoredReceipt { seq, receipt })
        })
        .transpose()?;
    if let Some(stored_receipt) = receipt.as_ref()
        && (admission.as_ref() != Some(&stored_receipt.receipt.admission)
            || authorization.as_ref() != stored_receipt.receipt.authorization.as_ref())
    {
        return Err(EvidenceError::Corrupt(
            "receipt decision material differs from authoritative decision rows".to_string(),
        ));
    }
    Ok(StoredActionEvidence {
        admission,
        authorization,
        receipt,
    })
}

async fn ensure_decision(
    transaction: &mut Transaction<'_, Sqlite>,
    record: &GovernanceDecisionRecord,
) -> Result<(), EvidenceError> {
    let (payload_json, digest) = canonical_storage_payload(record)?;
    verify_decision(transaction, record, &payload_json, digest.as_str(), false)
        .await
        .map(|_| ())
}

async fn verify_decision(
    transaction: &mut Transaction<'_, Sqlite>,
    record: &GovernanceDecisionRecord,
    payload_json: &str,
    payload_sha256: &str,
    inserted: bool,
) -> Result<AppendDisposition, EvidenceError> {
    let rows = sqlx::query(
        "SELECT decision_id, action_id, thread_id, turn_id, call_id, phase,
                schema_version, payload_json, payload_sha256
         FROM governance_decisions
         WHERE decision_id = ? OR (action_id = ? AND phase = ?)",
    )
    .bind(record.decision_id.as_str())
    .bind(record.action.action_id.as_str())
    .bind(record.phase.as_str())
    .fetch_all(&mut **transaction)
    .await
    .map_err(classify_sqlx_error)?;
    if rows.len() != 1 {
        return Err(EvidenceError::Corrupt(format!(
            "expected one decision row for {} but found {}",
            record.decision_id.as_str(),
            rows.len()
        )));
    }
    let row = &rows[0];
    let stored = decode_decision_row(row)?;
    let exact = row.get::<String, _>("decision_id") == record.decision_id.as_str()
        && row.get::<String, _>("action_id") == record.action.action_id.as_str()
        && row.get::<String, _>("phase") == record.phase.as_str()
        && row.get::<String, _>("payload_json") == payload_json
        && row.get::<String, _>("payload_sha256") == payload_sha256
        && stored == *record;
    if !exact {
        return Err(EvidenceError::IdempotencyConflict {
            record_id: record.decision_id.as_str().to_string(),
        });
    }
    Ok(if inserted {
        AppendDisposition::Inserted
    } else {
        AppendDisposition::AlreadyPresent
    })
}

async fn verify_receipt(
    transaction: &mut Transaction<'_, Sqlite>,
    receipt: &GovernanceReceipt,
    payload_json: &str,
    payload_sha256: &str,
    inserted: bool,
) -> Result<AppendDisposition, EvidenceError> {
    let rows = sqlx::query(
        "SELECT receipt_id, action_id, thread_id, turn_id, call_id,
                admission_decision_id, admission_phase,
                authorization_decision_id, authorization_phase,
                schema_version, payload_json, payload_sha256
         FROM governance_receipts
         WHERE receipt_id = ? OR action_id = ?",
    )
    .bind(receipt.receipt_id.as_str())
    .bind(receipt.action_id.as_str())
    .fetch_all(&mut **transaction)
    .await
    .map_err(classify_sqlx_error)?;
    if rows.len() != 1 {
        return Err(EvidenceError::Corrupt(format!(
            "expected one receipt row for {} but found {}",
            receipt.receipt_id.as_str(),
            rows.len()
        )));
    }
    let row = &rows[0];
    let stored = decode_receipt_row(row)?;
    let exact = row.get::<String, _>("receipt_id") == receipt.receipt_id.as_str()
        && row.get::<String, _>("action_id") == receipt.action_id.as_str()
        && row.get::<String, _>("admission_decision_id") == receipt.admission.decision_id.as_str()
        && row.get::<String, _>("admission_phase") == receipt.admission.phase.as_str()
        && row
            .get::<Option<String>, _>("authorization_decision_id")
            .as_deref()
            == receipt
                .authorization
                .as_ref()
                .map(|record| record.decision_id.as_str())
        && row
            .get::<Option<String>, _>("authorization_phase")
            .as_deref()
            == receipt
                .authorization
                .as_ref()
                .map(|record| record.phase.as_str())
        && row.get::<String, _>("payload_json") == payload_json
        && row.get::<String, _>("payload_sha256") == payload_sha256
        && stored == *receipt;
    if !exact {
        return Err(EvidenceError::IdempotencyConflict {
            record_id: receipt.receipt_id.as_str().to_string(),
        });
    }
    Ok(if inserted {
        AppendDisposition::Inserted
    } else {
        AppendDisposition::AlreadyPresent
    })
}

fn decode_decision_row(row: &SqliteRow) -> Result<GovernanceDecisionRecord, EvidenceError> {
    let payload_json: String = row.get("payload_json");
    verify_storage_payload_digest(&payload_json, row.get("payload_sha256"), "governance")?;
    let record: GovernanceDecisionRecord = serde_json::from_str(&payload_json)
        .map_err(|error| EvidenceError::Corrupt(error.to_string()))?;
    validate_decision(&record).map_err(invalid_record_as_corrupt)?;
    verify_canonical_storage_payload(&record, &payload_json, "governance")?;
    if row.get::<String, _>("decision_id") != record.decision_id.as_str()
        || row.get::<String, _>("action_id") != record.action.action_id.as_str()
        || row.get::<String, _>("thread_id") != record.action.thread_id.as_str()
        || row.get::<String, _>("turn_id") != record.action.turn_id.as_str()
        || row.get::<String, _>("call_id") != record.action.call_id.as_str()
        || row.get::<String, _>("phase") != record.phase.as_str()
        || row.get::<i64, _>("schema_version") != i64::from(record.action.schema_version)
    {
        return Err(EvidenceError::Corrupt(
            "decision columns do not match canonical payload".to_string(),
        ));
    }
    Ok(record)
}

fn decode_receipt_row(row: &SqliteRow) -> Result<GovernanceReceipt, EvidenceError> {
    let payload_json: String = row.get("payload_json");
    verify_storage_payload_digest(&payload_json, row.get("payload_sha256"), "governance")?;
    let receipt: GovernanceReceipt = serde_json::from_str(&payload_json)
        .map_err(|error| EvidenceError::Corrupt(error.to_string()))?;
    validate_receipt_binding(&receipt).map_err(invalid_record_as_corrupt)?;
    verify_canonical_storage_payload(&receipt, &payload_json, "governance")?;
    let action = receipt
        .authorization
        .as_ref()
        .map_or(&receipt.admission.action, |record| &record.action);
    if row.get::<String, _>("receipt_id") != receipt.receipt_id.as_str()
        || row.get::<String, _>("action_id") != receipt.action_id.as_str()
        || row.get::<String, _>("thread_id") != action.thread_id.as_str()
        || row.get::<String, _>("turn_id") != action.turn_id.as_str()
        || row.get::<String, _>("call_id") != action.call_id.as_str()
        || row.get::<String, _>("admission_decision_id") != receipt.admission.decision_id.as_str()
        || row.get::<String, _>("admission_phase") != receipt.admission.phase.as_str()
        || row
            .get::<Option<String>, _>("authorization_decision_id")
            .as_deref()
            != receipt
                .authorization
                .as_ref()
                .map(|record| record.decision_id.as_str())
        || row
            .get::<Option<String>, _>("authorization_phase")
            .as_deref()
            != receipt
                .authorization
                .as_ref()
                .map(|record| record.phase.as_str())
        || row.get::<i64, _>("schema_version") != i64::from(GOVERNANCE_SCHEMA_VERSION)
    {
        return Err(EvidenceError::Corrupt(
            "receipt columns do not match canonical payload".to_string(),
        ));
    }
    Ok(receipt)
}

pub(crate) fn now_millis() -> Result<i64, EvidenceError> {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| EvidenceError::Unavailable(error.to_string()))?
        .as_millis();
    i64::try_from(millis).map_err(|error| EvidenceError::Unavailable(error.to_string()))
}

fn classify_migrate_error(error: sqlx::migrate::MigrateError) -> EvidenceError {
    let detail = error.to_string();
    match error {
        sqlx::migrate::MigrateError::Execute(error)
        | sqlx::migrate::MigrateError::ExecuteMigration(error, _) => classify_sqlx_error(error),
        sqlx::migrate::MigrateError::VersionMissing(_)
        | sqlx::migrate::MigrateError::VersionMismatch(_)
        | sqlx::migrate::MigrateError::VersionNotPresent(_)
        | sqlx::migrate::MigrateError::Dirty(_) => EvidenceError::Corrupt(detail),
        _ => EvidenceError::Unavailable(detail),
    }
}

pub(crate) fn classify_sqlx_error(error: sqlx::Error) -> EvidenceError {
    let detail = error.to_string();
    match sqlite_primary_code(&error) {
        // SQLITE_CORRUPT, SQLITE_SCHEMA, SQLITE_NOTADB. SQLx exposes the
        // extended numeric code, whose low byte is the primary result code.
        Some(11 | 17 | 26) => EvidenceError::Corrupt(detail),
        _ => EvidenceError::Unavailable(detail),
    }
}

fn sqlite_primary_code(error: &sqlx::Error) -> Option<i32> {
    error
        .as_database_error()?
        .code()?
        .parse::<i32>()
        .ok()
        .map(|code| code & 0xff)
}
