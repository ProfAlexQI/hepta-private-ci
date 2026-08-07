use std::path::PathBuf;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use codex_hepta_contracts::ActionId;
use codex_hepta_contracts::GOVERNANCE_SCHEMA_VERSION;
use codex_hepta_contracts::GovernanceDecisionRecord;
use codex_hepta_contracts::GovernanceReceipt;
use codex_hepta_contracts::HandlerOutcome;
use codex_hepta_contracts::PolicyPhase;
use codex_hepta_contracts::ReceiptId;
use codex_hepta_contracts::ToolAction;
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

async fn verify_quick_check(pool: &SqlitePool) -> Result<(), EvidenceError> {
    let results = sqlx::query_scalar::<_, String>("PRAGMA quick_check(1)")
        .fetch_all(pool)
        .await
        .map_err(classify_sqlx_error)?;
    if results.len() == 1 && results[0] == "ok" {
        Ok(())
    } else {
        Err(EvidenceError::Corrupt(
            "SQLite quick_check reported invalid evidence storage".to_string(),
        ))
    }
}

struct SchemaObjectSpec {
    name: &'static str,
    object_type: &'static str,
    table_name: &'static str,
    required_sql_fragments: &'static [&'static str],
}

const REQUIRED_SCHEMA_OBJECTS: &[SchemaObjectSpec] = &[
    SchemaObjectSpec {
        name: "governance_decisions",
        object_type: "table",
        table_name: "governance_decisions",
        required_sql_fragments: &["create table", "governance_decisions"],
    },
    SchemaObjectSpec {
        name: "governance_receipts",
        object_type: "table",
        table_name: "governance_receipts",
        required_sql_fragments: &["create table", "governance_receipts"],
    },
    SchemaObjectSpec {
        name: "governance_decisions_thread_seq",
        object_type: "index",
        table_name: "governance_decisions",
        required_sql_fragments: &["create index", "governance_decisions", "thread_id", "seq"],
    },
    SchemaObjectSpec {
        name: "governance_receipts_thread_seq",
        object_type: "index",
        table_name: "governance_receipts",
        required_sql_fragments: &["create index", "governance_receipts", "thread_id", "seq"],
    },
    SchemaObjectSpec {
        name: "governance_decisions_no_update",
        object_type: "trigger",
        table_name: "governance_decisions",
        required_sql_fragments: &[
            "before update",
            "on governance_decisions",
            "raise(abort",
            "governance decisions are immutable",
        ],
    },
    SchemaObjectSpec {
        name: "governance_decisions_no_delete",
        object_type: "trigger",
        table_name: "governance_decisions",
        required_sql_fragments: &[
            "before delete",
            "on governance_decisions",
            "raise(abort",
            "governance decisions are immutable",
        ],
    },
    SchemaObjectSpec {
        name: "governance_receipts_no_update",
        object_type: "trigger",
        table_name: "governance_receipts",
        required_sql_fragments: &[
            "before update",
            "on governance_receipts",
            "raise(abort",
            "governance receipts are immutable",
        ],
    },
    SchemaObjectSpec {
        name: "governance_receipts_no_delete",
        object_type: "trigger",
        table_name: "governance_receipts",
        required_sql_fragments: &[
            "before delete",
            "on governance_receipts",
            "raise(abort",
            "governance receipts are immutable",
        ],
    },
    SchemaObjectSpec {
        name: "provider_invocation_intents",
        object_type: "table",
        table_name: "provider_invocation_intents",
        required_sql_fragments: &[
            "create table",
            "provider_invocation_intents",
            "attempt_id",
            "request_binding_id",
            "host_request_binding_id_sha256",
            "payload_sha256",
        ],
    },
    SchemaObjectSpec {
        name: "provider_invocation_terminals",
        object_type: "table",
        table_name: "provider_invocation_terminals",
        required_sql_fragments: &[
            "create table",
            "provider_invocation_terminals",
            "foreign key",
            "provider_invocation_intents",
            "on delete restrict",
        ],
    },
    SchemaObjectSpec {
        name: "provider_invocation_intents_thread_seq",
        object_type: "index",
        table_name: "provider_invocation_intents",
        required_sql_fragments: &[
            "create index",
            "provider_invocation_intents",
            "thread_id",
            "seq",
        ],
    },
    SchemaObjectSpec {
        name: "provider_invocation_intents_binding_seq",
        object_type: "index",
        table_name: "provider_invocation_intents",
        required_sql_fragments: &[
            "create index",
            "provider_invocation_intents",
            "request_binding_id",
            "seq",
        ],
    },
    SchemaObjectSpec {
        name: "provider_invocation_intents_host_binding_seq",
        object_type: "index",
        table_name: "provider_invocation_intents",
        required_sql_fragments: &[
            "create index",
            "provider_invocation_intents",
            "host_request_binding_id_sha256",
            "seq",
        ],
    },
    SchemaObjectSpec {
        name: "provider_invocation_terminals_thread_seq",
        object_type: "index",
        table_name: "provider_invocation_terminals",
        required_sql_fragments: &[
            "create index",
            "provider_invocation_terminals",
            "thread_id",
            "seq",
        ],
    },
    SchemaObjectSpec {
        name: "provider_invocation_intents_host_binding_required",
        object_type: "trigger",
        table_name: "provider_invocation_intents",
        required_sql_fragments: &[
            "before insert",
            "on provider_invocation_intents",
            "host_request_binding_id_sha256",
            "raise(abort",
        ],
    },
    SchemaObjectSpec {
        name: "provider_invocation_intents_no_update",
        object_type: "trigger",
        table_name: "provider_invocation_intents",
        required_sql_fragments: &[
            "before update",
            "on provider_invocation_intents",
            "raise(abort",
            "provider invocation intents are immutable",
        ],
    },
    SchemaObjectSpec {
        name: "provider_invocation_intents_no_delete",
        object_type: "trigger",
        table_name: "provider_invocation_intents",
        required_sql_fragments: &[
            "before delete",
            "on provider_invocation_intents",
            "raise(abort",
            "provider invocation intents are immutable",
        ],
    },
    SchemaObjectSpec {
        name: "provider_invocation_terminals_no_update",
        object_type: "trigger",
        table_name: "provider_invocation_terminals",
        required_sql_fragments: &[
            "before update",
            "on provider_invocation_terminals",
            "raise(abort",
            "provider invocation terminals are immutable",
        ],
    },
    SchemaObjectSpec {
        name: "provider_invocation_terminals_no_delete",
        object_type: "trigger",
        table_name: "provider_invocation_terminals",
        required_sql_fragments: &[
            "before delete",
            "on provider_invocation_terminals",
            "raise(abort",
            "provider invocation terminals are immutable",
        ],
    },
    SchemaObjectSpec {
        name: "memory_mutation_shadow_observations",
        object_type: "table",
        table_name: "memory_mutation_shadow_observations",
        required_sql_fragments: &[
            "create table",
            "memory_mutation_shadow_observations",
            "dry_run_id",
            "proposal_id",
            "projected_memory_writes between 0 and 2",
            "unique(proposal_id, snapshot_sha256)",
            "disposition = 'blocked'",
            "reason = 'ready'",
            "evidence_sha256",
        ],
    },
    SchemaObjectSpec {
        name: "memory_mutation_shadow_proposal_seq",
        object_type: "index",
        table_name: "memory_mutation_shadow_observations",
        required_sql_fragments: &[
            "create index",
            "memory_mutation_shadow_observations",
            "proposal_id",
            "seq",
        ],
    },
    SchemaObjectSpec {
        name: "memory_mutation_shadow_turn_seq",
        object_type: "index",
        table_name: "memory_mutation_shadow_observations",
        required_sql_fragments: &[
            "create index",
            "memory_mutation_shadow_observations",
            "turn_sha256",
            "seq",
        ],
    },
    SchemaObjectSpec {
        name: "memory_mutation_shadow_no_update",
        object_type: "trigger",
        table_name: "memory_mutation_shadow_observations",
        required_sql_fragments: &[
            "before update",
            "on memory_mutation_shadow_observations",
            "raise(abort",
            "memory mutation shadow observations are immutable",
        ],
    },
    SchemaObjectSpec {
        name: "memory_mutation_shadow_no_delete",
        object_type: "trigger",
        table_name: "memory_mutation_shadow_observations",
        required_sql_fragments: &[
            "before delete",
            "on memory_mutation_shadow_observations",
            "raise(abort",
            "memory mutation shadow observations are immutable",
        ],
    },
    SchemaObjectSpec {
        name: "channel_ingress_events",
        object_type: "table",
        table_name: "channel_ingress_events",
        required_sql_fragments: &[
            "create table",
            "channel_ingress_events",
            "event_id text not null unique",
            "target_thread_sha256",
            "length(target_thread_sha256) = 64",
            "unique(scope_sha256, source_event_sha256)",
            "schema_version integer not null check (schema_version = 1)",
            "length(evidence_sha256) = 64",
            "evidence_sha256",
        ],
    },
    SchemaObjectSpec {
        name: "channel_ingress_receipts",
        object_type: "table",
        table_name: "channel_ingress_receipts",
        required_sql_fragments: &[
            "create table",
            "channel_ingress_receipts",
            "receipt_id text not null unique",
            "event_id text not null unique",
            "terminal_kind in ('accepted', 'rejected', 'indeterminate')",
            "terminal_kind = 'accepted' and thread_id is not null and turn_id is not null",
            "terminal_kind in ('rejected', 'indeterminate') and thread_id is null and turn_id is null",
            "schema_version integer not null check (schema_version = 1)",
            "length(evidence_sha256) = 64",
            "foreign key(event_id)",
            "channel_ingress_events(event_id)",
            "on update restrict",
            "on delete restrict",
        ],
    },
    SchemaObjectSpec {
        name: "channel_ingress_events_scope_seq",
        object_type: "index",
        table_name: "channel_ingress_events",
        required_sql_fragments: &[
            "create index",
            "channel_ingress_events",
            "scope_sha256",
            "seq",
        ],
    },
    SchemaObjectSpec {
        name: "channel_ingress_receipts_scope_seq",
        object_type: "index",
        table_name: "channel_ingress_receipts",
        required_sql_fragments: &[
            "create index",
            "channel_ingress_receipts",
            "scope_sha256",
            "seq",
        ],
    },
    SchemaObjectSpec {
        name: "channel_ingress_events_no_update",
        object_type: "trigger",
        table_name: "channel_ingress_events",
        required_sql_fragments: &[
            "before update",
            "on channel_ingress_events",
            "raise(abort",
            "channel ingress events are immutable",
        ],
    },
    SchemaObjectSpec {
        name: "channel_ingress_events_no_delete",
        object_type: "trigger",
        table_name: "channel_ingress_events",
        required_sql_fragments: &[
            "before delete",
            "on channel_ingress_events",
            "raise(abort",
            "channel ingress events are immutable",
        ],
    },
    SchemaObjectSpec {
        name: "channel_ingress_receipts_no_update",
        object_type: "trigger",
        table_name: "channel_ingress_receipts",
        required_sql_fragments: &[
            "before update",
            "on channel_ingress_receipts",
            "raise(abort",
            "channel ingress receipts are immutable",
        ],
    },
    SchemaObjectSpec {
        name: "channel_ingress_receipts_no_delete",
        object_type: "trigger",
        table_name: "channel_ingress_receipts",
        required_sql_fragments: &[
            "before delete",
            "on channel_ingress_receipts",
            "raise(abort",
            "channel ingress receipts are immutable",
        ],
    },
];

async fn verify_provider_host_bindings(pool: &SqlitePool) -> Result<(), EvidenceError> {
    let missing: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM provider_invocation_intents
         WHERE host_request_binding_id_sha256 IS NULL",
    )
    .fetch_one(pool)
    .await
    .map_err(classify_sqlx_error)?;
    if missing == 0 {
        Ok(())
    } else {
        Err(EvidenceError::Corrupt(format!(
            "{missing} provider intent rows predate host request binding evidence; explicit migration is required"
        )))
    }
}

async fn verify_schema_manifest(pool: &SqlitePool) -> Result<(), EvidenceError> {
    for spec in REQUIRED_SCHEMA_OBJECTS {
        let row = sqlx::query(
            "SELECT type AS object_type, tbl_name, sql
             FROM sqlite_schema WHERE name = ?",
        )
        .bind(spec.name)
        .fetch_optional(pool)
        .await
        .map_err(classify_sqlx_error)?
        .ok_or_else(|| {
            EvidenceError::Corrupt(format!(
                "required SQLite schema object {} is missing",
                spec.name
            ))
        })?;
        let object_type: String = row.get("object_type");
        let table_name: String = row.get("tbl_name");
        let sql: Option<String> = row.get("sql");
        let Some(sql) = sql else {
            return Err(EvidenceError::Corrupt(format!(
                "required SQLite schema object {} has no definition",
                spec.name
            )));
        };
        let normalized_sql = sql.to_ascii_lowercase();
        if object_type != spec.object_type
            || table_name != spec.table_name
            || spec
                .required_sql_fragments
                .iter()
                .any(|fragment| !normalized_sql.contains(fragment))
        {
            return Err(EvidenceError::Corrupt(format!(
                "required SQLite schema object {} has an invalid definition",
                spec.name
            )));
        }
    }
    Ok(())
}

async fn verify_foreign_keys(pool: &SqlitePool) -> Result<(), EvidenceError> {
    let violation = sqlx::query("PRAGMA foreign_key_check")
        .fetch_optional(pool)
        .await
        .map_err(classify_sqlx_error)?;
    if violation.is_some() {
        Err(EvidenceError::Corrupt(
            "SQLite foreign_key_check found invalid evidence references".to_string(),
        ))
    } else {
        Ok(())
    }
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

fn validate_decision(record: &GovernanceDecisionRecord) -> Result<(), EvidenceError> {
    if record.action.schema_version != GOVERNANCE_SCHEMA_VERSION {
        return invalid("unsupported action schema version");
    }
    let expected_action = codex_hepta_contracts::ActionId::for_tool_call(
        &record.action.thread_id,
        &record.action.turn_id,
        &record.action.call_id,
    );
    if record.action.action_id != expected_action {
        return invalid("action id does not bind thread, turn, and call ids");
    }
    let expected_decision = codex_hepta_contracts::DecisionId::for_action(
        &record.action.action_id,
        record.phase.as_str(),
    );
    if record.decision_id != expected_decision {
        return invalid("decision id does not bind action and policy phase");
    }
    for (label, digest) in [
        ("payload", &record.action.payload_sha256),
        ("policy", &record.policy.content_sha256),
    ] {
        if !is_canonical_sha256(digest.as_str()) {
            return invalid(format!("{label} digest is not canonical lowercase SHA-256"));
        }
    }
    if record.policy.policy_id.trim().is_empty() || record.policy.revision == 0 {
        return invalid("policy stamp requires a non-empty id and positive revision");
    }
    Ok(())
}

fn validate_receipt_binding(receipt: &GovernanceReceipt) -> Result<(), EvidenceError> {
    validate_decision(&receipt.admission)?;
    if receipt.admission.phase != PolicyPhase::Admission {
        return invalid("receipt admission record has the wrong policy phase");
    }
    if receipt.action_id != receipt.admission.action.action_id {
        return invalid("receipt action id does not match admission");
    }
    if receipt.receipt_id != ReceiptId::for_action(&receipt.action_id) {
        return invalid("receipt id does not bind its action id");
    }
    if let Some(authorization) = receipt.authorization.as_ref() {
        validate_decision(authorization)?;
        if authorization.phase != PolicyPhase::Authorization {
            return invalid("receipt authorization record has the wrong policy phase");
        }
        if !same_action_binding(&receipt.admission.action, &authorization.action) {
            return invalid("authorization does not bind the admitted action identity");
        }
    }
    if matches!(
        receipt.outcome,
        HandlerOutcome::HandlerCompleted { .. }
            | HandlerOutcome::HandlerFailed {
                handler_executed: true
            }
    ) && (!receipt.host_accepted || receipt.authorization.is_none())
    {
        return invalid("handler outcome requires accepted and authorized execution");
    }
    if matches!(
        receipt.admission.decision,
        codex_hepta_contracts::GovernanceDecision::Block { .. }
    ) && (receipt.host_accepted || receipt.authorization.is_some())
    {
        return invalid("blocked admission cannot be host-accepted or authorized");
    }
    if receipt.authorization.as_ref().is_some_and(|authorization| {
        matches!(
            authorization.decision,
            codex_hepta_contracts::GovernanceDecision::Block { .. }
        ) && !matches!(receipt.outcome, HandlerOutcome::Blocked)
    }) {
        return invalid("blocked authorization requires a blocked terminal outcome");
    }
    Ok(())
}

fn same_action_binding(left: &ToolAction, right: &ToolAction) -> bool {
    left.schema_version == right.schema_version
        && left.action_id == right.action_id
        && left.thread_id == right.thread_id
        && left.turn_id == right.turn_id
        && left.call_id == right.call_id
        && left.tool_name == right.tool_name
        && left.source == right.source
}

fn is_canonical_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn invalid<T>(detail: impl Into<String>) -> Result<T, EvidenceError> {
    Err(EvidenceError::InvalidRecord(detail.into()))
}
