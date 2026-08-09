use std::num::NonZeroU64;
use std::path::PathBuf;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use codex_hepta_contracts::ActionId;
use codex_hepta_contracts::GOVERNANCE_SCHEMA_VERSION;
use codex_hepta_contracts::GovernanceDecisionRecord;
use codex_hepta_contracts::GovernanceReceipt;
use codex_hepta_contracts::PolicyPhase;
use codex_hepta_contracts::ReceiptId;
use codex_hepta_contracts::Sha256Digest;
use codex_state::SqliteConfig;
use sqlx::Row;
use sqlx::SqlitePool;

use crate::EvidenceError;
use crate::canonical::canonical_json;
use crate::frozen_oracle_qualification::verify_frozen_oracle_qualification_integrity;
use crate::governance_store::decode_decision_row;
use crate::governance_store::decode_receipt_row;
use crate::governance_store::ensure_decision;
use crate::governance_store::verify_decision;
use crate::governance_store::verify_receipt;
use crate::governance_validation::validate_decision;
use crate::governance_validation::validate_receipt_binding;
use crate::live_product_shadow::verify_live_product_shadow_v2_integrity;
use crate::promotion_replay_store::SqlitePromotionReceiptReplayStore;
use crate::promotion_replay_store::verify_promotion_replay_integrity;
use crate::schema_validation::classify_migrate_error;
use crate::schema_validation::classify_sqlx_error;
use crate::schema_validation::verify_foreign_keys;
use crate::schema_validation::verify_provider_ephemeral_input_projection;
use crate::schema_validation::verify_provider_host_bindings;
use crate::schema_validation::verify_quick_check;
use crate::schema_validation::verify_schema_manifest;

// The filename is the store-lineage boundary. Frozen vNext used lineage 1 with
// a different meaning for migration 0004, so this migration set must never
// open or extend that database.
const EVIDENCE_DB_FILENAME: &str = "hepta_evidence_2.sqlite";
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
        if let Err(error) = verify_frozen_oracle_qualification_integrity(&pool).await {
            pool.close().await;
            return Err(error);
        }
        if let Err(error) = verify_promotion_replay_integrity(&pool).await {
            pool.close().await;
            return Err(error);
        }
        if let Err(error) = verify_live_product_shadow_v2_integrity(&pool).await {
            pool.close().await;
            return Err(error);
        }
        if let Err(error) = verify_provider_host_bindings(&pool).await {
            pool.close().await;
            return Err(error);
        }
        if let Err(error) = verify_provider_ephemeral_input_projection(&pool).await {
            pool.close().await;
            return Err(error);
        }
        if let Err(error) = verify_foreign_keys(&pool).await {
            pool.close().await;
            return Err(error);
        }
        Ok(Self { pool, path })
    }

    pub fn path(&self) -> &std::path::Path {
        &self.path
    }

    /// Creates the durable promotion replay facade with an explicit trusted
    /// clock forward-jump policy. No product default is selected here: an
    /// excessive jump fails closed and requires authenticated store recovery.
    pub fn promotion_receipt_replay_store(
        &self,
        max_forward_jump_seconds: NonZeroU64,
    ) -> SqlitePromotionReceiptReplayStore {
        SqlitePromotionReceiptReplayStore::new(self.pool.clone(), max_forward_jump_seconds)
    }

    pub async fn append_decision(
        &self,
        record: &GovernanceDecisionRecord,
    ) -> Result<AppendDisposition, EvidenceError> {
        validate_decision(record)?;
        let payload = canonical_json(record)?;
        let payload_json = String::from_utf8(payload.clone())
            .map_err(|error| EvidenceError::Serialization(error.to_string()))?;
        let payload_sha256 = Sha256Digest::for_bytes(&payload);
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
        let payload = canonical_json(receipt)?;
        let payload_json = String::from_utf8(payload.clone())
            .map_err(|error| EvidenceError::Serialization(error.to_string()))?;
        let payload_sha256 = Sha256Digest::for_bytes(&payload);
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
        let decision_rows = sqlx::query(
            "SELECT decision_id, action_id, thread_id, turn_id, call_id, phase,
                    schema_version, payload_json, payload_sha256
             FROM governance_decisions WHERE action_id = ? ORDER BY seq ASC",
        )
        .bind(action_id.as_str())
        .fetch_all(&self.pool)
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
        let receipt_row = sqlx::query(
            "SELECT seq, receipt_id, action_id, thread_id, turn_id, call_id,
                    admission_decision_id, admission_phase,
                    authorization_decision_id, authorization_phase,
                    schema_version, payload_json, payload_sha256
             FROM governance_receipts WHERE action_id = ?",
        )
        .bind(action_id.as_str())
        .fetch_optional(&self.pool)
        .await
        .map_err(classify_sqlx_error)?;
        let receipt = receipt_row
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

pub(crate) fn now_millis() -> Result<i64, EvidenceError> {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| EvidenceError::Unavailable(error.to_string()))?
        .as_millis();
    i64::try_from(millis).map_err(|error| EvidenceError::Unavailable(error.to_string()))
}
