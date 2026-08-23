use codex_hepta_contracts::ProviderEffectAck;
use codex_hepta_contracts::ProviderEffectAckStatus;
use codex_hepta_contracts::ProviderEffectBindingError;
use codex_hepta_contracts::ProviderEffectIdempotencyCapability;
use codex_hepta_contracts::ProviderEffectIntent;
use codex_hepta_contracts::ProviderEffectKey;
use codex_hepta_contracts::ProviderEffectLookup;
use codex_hepta_contracts::ProviderEffectState;
use codex_hepta_contracts::ProviderEffectUncertainty;
use codex_hepta_contracts::Sha256Digest;
use codex_hepta_contracts::reconcile_provider_lookup;
use sqlx::Row;
use sqlx::Sqlite;
use sqlx::SqlitePool;
use sqlx::Transaction;

use crate::AppendDisposition;
use crate::EvidenceError;
use crate::HeptaEvidenceStore;
use crate::canonical::canonical_json;
use crate::schema_validation::classify_sqlx_error;
use crate::store::now_millis;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StoredProviderEffectIntent {
    pub seq: i64,
    pub intent: ProviderEffectIntent,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StoredProviderEffectAck {
    pub seq: i64,
    pub ack: ProviderEffectAck,
    pub recorded_at_ms: i64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StoredProviderEffectUncertainty {
    pub seq: i64,
    pub uncertainty: ProviderEffectUncertainty,
    pub recorded_at_ms: i64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StoredProviderEffect {
    pub intent: StoredProviderEffectIntent,
    pub acknowledgements: Vec<StoredProviderEffectAck>,
    pub uncertainties: Vec<StoredProviderEffectUncertainty>,
}

impl StoredProviderEffect {
    pub fn state(&self) -> ProviderEffectState {
        let latest_ack = self.acknowledgements.last();
        let latest_uncertainty = self.uncertainties.last();
        match (latest_ack, latest_uncertainty) {
            (Some(ack), Some(uncertainty)) if uncertainty.recorded_at_ms > ack.recorded_at_ms => {
                ProviderEffectState::Indeterminate
            }
            (Some(ack), _) => ack.ack.state(),
            (None, Some(_)) => ProviderEffectState::Indeterminate,
            (None, None) => ProviderEffectState::Pending,
        }
    }
}

impl HeptaEvidenceStore {
    /// Persists the occurrence intent before any provider dispatch crosses the
    /// transport boundary.  Replaying the exact intent is idempotent; a
    /// same-key/different-payload attempt is rejected.
    pub async fn append_provider_effect_intent(
        &self,
        intent: &ProviderEffectIntent,
    ) -> Result<AppendDisposition, EvidenceError> {
        intent.validate().map_err(binding_invalid)?;
        let payload = canonical_json(intent)?;
        let payload_json = String::from_utf8(payload.clone())
            .map_err(|error| EvidenceError::Serialization(error.to_string()))?;
        let record_sha256 = Sha256Digest::for_bytes(&payload);
        let mut transaction = self
            .pool
            .begin_with("BEGIN IMMEDIATE")
            .await
            .map_err(classify_sqlx_error)?;
        let insert = sqlx::query(
            "INSERT INTO provider_effect_intents (
                effect_key, payload_sha256, schema_version,
                payload_json, record_sha256, recorded_at_ms
             ) VALUES (?, ?, ?, ?, ?, ?)
             ON CONFLICT DO NOTHING",
        )
        .bind(intent.key.as_str())
        .bind(intent.payload_sha256.as_str())
        .bind(i64::from(intent.schema_version))
        .bind(&payload_json)
        .bind(record_sha256.as_str())
        .bind(now_millis()?)
        .execute(&mut *transaction)
        .await
        .map_err(classify_sqlx_error)?;
        let disposition = verify_effect_intent(
            &mut transaction,
            intent,
            &payload_json,
            record_sha256.as_str(),
            insert.rows_affected() == 1,
        )
        .await?;
        transaction.commit().await.map_err(classify_sqlx_error)?;
        Ok(disposition)
    }

    /// Appends a provider ACK after validating it against the authoritative
    /// intent and the monotonic Accepted → Completed transition rules.
    pub async fn append_provider_effect_ack(
        &self,
        ack: &ProviderEffectAck,
    ) -> Result<AppendDisposition, EvidenceError> {
        let payload = canonical_json(ack)?;
        let payload_json = String::from_utf8(payload.clone())
            .map_err(|error| EvidenceError::Serialization(error.to_string()))?;
        let record_sha256 = Sha256Digest::for_bytes(&payload);
        let mut transaction = self
            .pool
            .begin_with("BEGIN IMMEDIATE")
            .await
            .map_err(classify_sqlx_error)?;
        let intent = load_effect_intent_in_transaction(&mut transaction, &ack.key).await?;
        let Some(intent) = intent else {
            transaction.rollback().await.map_err(classify_sqlx_error)?;
            return Err(EvidenceError::InvalidRecord(
                "provider effect acknowledgement has no authoritative intent".to_string(),
            ));
        };
        ack.validate_for(&intent.intent).map_err(binding_invalid)?;
        validate_ack_transition_in_transaction(&mut transaction, &intent.intent, ack).await?;
        let recorded_at_ms = next_effect_recorded_at_ms(&mut transaction, &ack.key).await?;
        let insert = sqlx::query(
            "INSERT INTO provider_effect_acknowledgements (
                effect_key, payload_sha256, provider_operation_id_sha256,
                status, schema_version, payload_json, record_sha256, recorded_at_ms
             ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
             ON CONFLICT DO NOTHING",
        )
        .bind(ack.key.as_str())
        .bind(ack.payload_sha256.as_str())
        .bind(ack.provider_operation_id_sha256.as_str())
        .bind(ack_status_as_str(ack.status))
        .bind(i64::from(ack.schema_version))
        .bind(&payload_json)
        .bind(record_sha256.as_str())
        .bind(recorded_at_ms)
        .execute(&mut *transaction)
        .await
        .map_err(classify_sqlx_error)?;
        let disposition = verify_effect_ack(
            &mut transaction,
            ack,
            &payload_json,
            record_sha256.as_str(),
            insert.rows_affected() == 1,
        )
        .await?;
        transaction.commit().await.map_err(classify_sqlx_error)?;
        Ok(disposition)
    }

    /// Appends a durable quarantine marker for an unknown provider outcome.
    ///
    /// This method is intentionally separate from ACK persistence: an
    /// uncertainty never claims that the provider applied (or rejected) the
    /// effect.  A later key-bound ACK may reconcile it.
    pub async fn mark_provider_effect_indeterminate(
        &self,
        key: &ProviderEffectKey,
        reason_code: impl Into<String>,
    ) -> Result<AppendDisposition, EvidenceError> {
        let mut transaction = self
            .pool
            .begin_with("BEGIN IMMEDIATE")
            .await
            .map_err(classify_sqlx_error)?;
        let intent = load_effect_intent_in_transaction(&mut transaction, key).await?;
        let Some(intent) = intent else {
            transaction.rollback().await.map_err(classify_sqlx_error)?;
            return Err(EvidenceError::InvalidRecord(
                "provider effect uncertainty has no authoritative intent".to_string(),
            ));
        };
        let uncertainty = ProviderEffectUncertainty::new(
            key.clone(),
            intent.intent.payload_sha256.clone(),
            reason_code,
        );
        uncertainty
            .validate_for(&intent.intent)
            .map_err(binding_invalid)?;
        if latest_ack_is_terminal(&mut transaction, &intent.intent, key).await? {
            transaction.rollback().await.map_err(classify_sqlx_error)?;
            return Err(EvidenceError::IdempotencyConflict {
                record_id: key.as_str().to_string(),
            });
        }
        let payload = canonical_json(&uncertainty)?;
        let payload_json = String::from_utf8(payload.clone())
            .map_err(|error| EvidenceError::Serialization(error.to_string()))?;
        let record_sha256 = Sha256Digest::for_bytes(&payload);
        let recorded_at_ms = next_effect_recorded_at_ms(&mut transaction, key).await?;
        let insert = sqlx::query(
            "INSERT INTO provider_effect_uncertainties (
                effect_key, payload_sha256, reason_code, schema_version,
                payload_json, record_sha256, recorded_at_ms
             ) VALUES (?, ?, ?, ?, ?, ?, ?)
             ON CONFLICT DO NOTHING",
        )
        .bind(key.as_str())
        .bind(uncertainty.payload_sha256.as_str())
        .bind(&uncertainty.reason_code)
        .bind(i64::from(uncertainty.schema_version))
        .bind(&payload_json)
        .bind(record_sha256.as_str())
        .bind(recorded_at_ms)
        .execute(&mut *transaction)
        .await
        .map_err(classify_sqlx_error)?;
        let disposition = verify_effect_uncertainty(
            &mut transaction,
            &uncertainty,
            &payload_json,
            record_sha256.as_str(),
            insert.rows_affected() == 1,
        )
        .await?;
        transaction.commit().await.map_err(classify_sqlx_error)?;
        Ok(disposition)
    }

    pub async fn get_provider_effect(
        &self,
        key: &ProviderEffectKey,
    ) -> Result<Option<StoredProviderEffect>, EvidenceError> {
        let mut transaction = self.pool.begin().await.map_err(classify_sqlx_error)?;
        let intent = load_effect_intent_in_transaction(&mut transaction, key).await?;
        let Some(intent) = intent else {
            transaction.commit().await.map_err(classify_sqlx_error)?;
            return Ok(None);
        };
        let rows = sqlx::query(
            "SELECT seq, effect_key, payload_sha256,
                    provider_operation_id_sha256, status, schema_version,
                    payload_json, record_sha256, recorded_at_ms
             FROM provider_effect_acknowledgements
             WHERE effect_key = ? ORDER BY seq ASC",
        )
        .bind(key.as_str())
        .fetch_all(&mut *transaction)
        .await
        .map_err(classify_sqlx_error)?;
        let mut acknowledgements = Vec::with_capacity(rows.len());
        for row in rows {
            acknowledgements.push(decode_effect_ack_row(&row, &intent.intent)?);
        }
        let uncertainty_rows = sqlx::query(
            "SELECT seq, effect_key, payload_sha256, reason_code,
                    schema_version, payload_json, record_sha256, recorded_at_ms
             FROM provider_effect_uncertainties
             WHERE effect_key = ? ORDER BY recorded_at_ms ASC, seq ASC",
        )
        .bind(key.as_str())
        .fetch_all(&mut *transaction)
        .await
        .map_err(classify_sqlx_error)?;
        let mut uncertainties = Vec::with_capacity(uncertainty_rows.len());
        for row in uncertainty_rows {
            uncertainties.push(decode_effect_uncertainty_row(&row, &intent.intent)?);
        }
        acknowledgements.sort_by_key(|ack| (ack.recorded_at_ms, ack.seq));
        transaction.commit().await.map_err(classify_sqlx_error)?;
        Ok(Some(StoredProviderEffect {
            intent,
            acknowledgements,
            uncertainties,
        }))
    }

    /// Reconciles a lookup and appends a matching ACK. Unknown, not-found,
    /// and conflict lookups persist a quarantine marker and remain fail-closed.
    pub async fn reconcile_provider_effect_lookup(
        &self,
        capability: ProviderEffectIdempotencyCapability,
        key: &ProviderEffectKey,
        lookup: ProviderEffectLookup,
    ) -> Result<ProviderEffectState, EvidenceError> {
        let stored = self.get_provider_effect(key).await?.ok_or_else(|| {
            EvidenceError::InvalidRecord("provider effect intent not found".into())
        })?;
        if capability == ProviderEffectIdempotencyCapability::Unsupported {
            self.mark_provider_effect_indeterminate(key, "provider_capability_unsupported")
                .await?;
            return Err(EvidenceError::InvalidRecord(
                "provider effect status lookup is unsupported".to_string(),
            ));
        }
        match lookup {
            ProviderEffectLookup::Ack(ack) => {
                let ack = reconcile_provider_lookup(
                    capability,
                    &stored.intent.intent,
                    ProviderEffectLookup::Ack(ack),
                )
                .map_err(binding_invalid)?;
                self.append_provider_effect_ack(&ack).await?;
            }
            ProviderEffectLookup::Unknown => {
                self.mark_provider_effect_indeterminate(key, "provider_lookup_unknown")
                    .await?;
            }
            ProviderEffectLookup::NotFound => {
                self.mark_provider_effect_indeterminate(key, "provider_status_not_found")
                    .await?;
            }
            ProviderEffectLookup::Conflict { .. } => {
                self.mark_provider_effect_indeterminate(key, "provider_payload_conflict")
                    .await?;
            }
        }
        Ok(self
            .get_provider_effect(key)
            .await?
            .map(|effect| effect.state())
            .unwrap_or(ProviderEffectState::Indeterminate))
    }
}

/// Verifies every durable effect row during evidence-store open.  A schema
/// check alone is insufficient: canonical payloads, projected columns, ACK
/// bindings, and quarantine reasons must all agree before a caller can trust
/// an effect state after restart.
pub(crate) async fn verify_provider_effect_rows(pool: &SqlitePool) -> Result<(), EvidenceError> {
    let intent_rows = sqlx::query(
        "SELECT seq, effect_key, payload_sha256, schema_version,
                payload_json, record_sha256
         FROM provider_effect_intents ORDER BY seq ASC",
    )
    .fetch_all(pool)
    .await
    .map_err(classify_sqlx_error)?;
    for intent_row in intent_rows {
        let intent = decode_effect_intent_row(&intent_row)?;
        let ack_rows = sqlx::query(
            "SELECT seq, effect_key, payload_sha256,
                    provider_operation_id_sha256, status, schema_version,
                    payload_json, record_sha256, recorded_at_ms
             FROM provider_effect_acknowledgements
             WHERE effect_key = ? ORDER BY recorded_at_ms ASC, seq ASC",
        )
        .bind(intent.intent.key.as_str())
        .fetch_all(pool)
        .await
        .map_err(classify_sqlx_error)?;
        let uncertainty_rows = sqlx::query(
            "SELECT seq, effect_key, payload_sha256, reason_code,
                    schema_version, payload_json, record_sha256, recorded_at_ms
             FROM provider_effect_uncertainties
             WHERE effect_key = ? ORDER BY recorded_at_ms ASC, seq ASC",
        )
        .bind(intent.intent.key.as_str())
        .fetch_all(pool)
        .await
        .map_err(classify_sqlx_error)?;
        let mut previous_ack: Option<StoredProviderEffectAck> = None;
        for row in ack_rows {
            let ack = decode_effect_ack_row(&row, &intent.intent)?;
            if let Some(previous) = previous_ack.as_ref() {
                let uncertainty_between = uncertainty_rows.iter().any(|uncertainty_row| {
                    let recorded_at_ms = uncertainty_row.get::<i64, _>("recorded_at_ms");
                    recorded_at_ms > previous.recorded_at_ms && recorded_at_ms < ack.recorded_at_ms
                });
                if previous.ack.state().is_terminal() {
                    return Err(EvidenceError::Corrupt(
                        "provider effect terminal ACK has a later ACK".to_string(),
                    ));
                }
                let legal_status_transition = matches!(
                    (previous.ack.status, ack.ack.status),
                    (
                        ProviderEffectAckStatus::Accepted,
                        ProviderEffectAckStatus::Accepted
                    ) | (
                        ProviderEffectAckStatus::Accepted,
                        ProviderEffectAckStatus::Completed
                    )
                );
                let operation_rebound = previous.ack.provider_operation_id_sha256
                    != ack.ack.provider_operation_id_sha256;
                let legal = if uncertainty_between {
                    // An uncertainty can justify rebinding the provider
                    // operation id, but never permits Accepted -> Rejected.
                    legal_status_transition
                } else {
                    legal_status_transition && !operation_rebound
                };
                if !legal {
                    return Err(EvidenceError::Corrupt(
                        "provider effect ACK transition is not monotonic".to_string(),
                    ));
                }
            }
            previous_ack = Some(ack);
        }
        for row in uncertainty_rows {
            let _ = decode_effect_uncertainty_row(&row, &intent.intent)?;
        }
    }
    Ok(())
}

async fn load_effect_intent_in_transaction(
    transaction: &mut Transaction<'_, Sqlite>,
    key: &ProviderEffectKey,
) -> Result<Option<StoredProviderEffectIntent>, EvidenceError> {
    let row = sqlx::query(
        "SELECT seq, effect_key, payload_sha256, schema_version,
                payload_json, record_sha256
         FROM provider_effect_intents WHERE effect_key = ?",
    )
    .bind(key.as_str())
    .fetch_optional(&mut **transaction)
    .await
    .map_err(classify_sqlx_error)?;
    row.map(|row| decode_effect_intent_row(&row)).transpose()
}

fn decode_effect_intent_row(
    row: &sqlx::sqlite::SqliteRow,
) -> Result<StoredProviderEffectIntent, EvidenceError> {
    let payload_json: String = row.get("payload_json");
    let record_sha256: String = row.get("record_sha256");
    verify_record_digest(&payload_json, &record_sha256, "provider effect intent")?;
    let intent: ProviderEffectIntent = serde_json::from_str(&payload_json)
        .map_err(|error| EvidenceError::Corrupt(error.to_string()))?;
    intent.validate().map_err(binding_corrupt)?;
    let payload_sha256: String = row.get("payload_sha256");
    if row.get::<String, _>("effect_key") != intent.key.as_str()
        || payload_sha256 != intent.payload_sha256.as_str()
        || row.get::<i64, _>("schema_version") != i64::from(intent.schema_version)
    {
        return Err(EvidenceError::Corrupt(
            "provider effect intent columns do not match canonical payload".to_string(),
        ));
    }
    Ok(StoredProviderEffectIntent {
        seq: row.get("seq"),
        intent,
    })
}

fn decode_effect_ack_row(
    row: &sqlx::sqlite::SqliteRow,
    intent: &ProviderEffectIntent,
) -> Result<StoredProviderEffectAck, EvidenceError> {
    let payload_json: String = row.get("payload_json");
    let record_sha256: String = row.get("record_sha256");
    verify_record_digest(
        &payload_json,
        &record_sha256,
        "provider effect acknowledgement",
    )?;
    let ack: ProviderEffectAck = serde_json::from_str(&payload_json)
        .map_err(|error| EvidenceError::Corrupt(error.to_string()))?;
    ack.validate_for(intent).map_err(binding_corrupt)?;
    let status: String = row.get("status");
    if row.get::<String, _>("effect_key") != ack.key.as_str()
        || row.get::<String, _>("payload_sha256") != ack.payload_sha256.as_str()
        || row.get::<String, _>("provider_operation_id_sha256")
            != ack.provider_operation_id_sha256.as_str()
        || status != ack_status_as_str(ack.status)
        || row.get::<i64, _>("schema_version") != i64::from(ack.schema_version)
    {
        return Err(EvidenceError::Corrupt(
            "provider effect acknowledgement columns do not match canonical payload".to_string(),
        ));
    }
    Ok(StoredProviderEffectAck {
        seq: row.get("seq"),
        ack,
        recorded_at_ms: row.get("recorded_at_ms"),
    })
}

fn decode_effect_uncertainty_row(
    row: &sqlx::sqlite::SqliteRow,
    intent: &ProviderEffectIntent,
) -> Result<StoredProviderEffectUncertainty, EvidenceError> {
    let payload_json: String = row.get("payload_json");
    let record_sha256: String = row.get("record_sha256");
    verify_record_digest(&payload_json, &record_sha256, "provider effect uncertainty")?;
    let uncertainty: ProviderEffectUncertainty = serde_json::from_str(&payload_json)
        .map_err(|error| EvidenceError::Corrupt(error.to_string()))?;
    uncertainty.validate_for(intent).map_err(binding_corrupt)?;
    if row.get::<String, _>("effect_key") != uncertainty.key.as_str()
        || row.get::<String, _>("payload_sha256") != uncertainty.payload_sha256.as_str()
        || row.get::<String, _>("reason_code") != uncertainty.reason_code
        || row.get::<i64, _>("schema_version") != i64::from(uncertainty.schema_version)
    {
        return Err(EvidenceError::Corrupt(
            "provider effect uncertainty columns do not match canonical payload".to_string(),
        ));
    }
    Ok(StoredProviderEffectUncertainty {
        seq: row.get("seq"),
        uncertainty,
        recorded_at_ms: row.get("recorded_at_ms"),
    })
}

async fn verify_effect_intent(
    transaction: &mut Transaction<'_, Sqlite>,
    intent: &ProviderEffectIntent,
    payload_json: &str,
    record_sha256: &str,
    inserted: bool,
) -> Result<AppendDisposition, EvidenceError> {
    let row = sqlx::query(
        "SELECT seq, effect_key, payload_sha256, schema_version,
                payload_json, record_sha256
         FROM provider_effect_intents WHERE effect_key = ?",
    )
    .bind(intent.key.as_str())
    .fetch_optional(&mut **transaction)
    .await
    .map_err(classify_sqlx_error)?
    .ok_or_else(|| EvidenceError::Corrupt("provider effect intent disappeared".to_string()))?;
    let stored = decode_effect_intent_row(&row)?;
    if stored.intent != *intent
        || row.get::<String, _>("payload_json") != payload_json
        || row.get::<String, _>("record_sha256") != record_sha256
    {
        return Err(EvidenceError::IdempotencyConflict {
            record_id: intent.key.as_str().to_string(),
        });
    }
    Ok(if inserted {
        AppendDisposition::Inserted
    } else {
        AppendDisposition::AlreadyPresent
    })
}

async fn verify_effect_ack(
    transaction: &mut Transaction<'_, Sqlite>,
    ack: &ProviderEffectAck,
    payload_json: &str,
    record_sha256: &str,
    inserted: bool,
) -> Result<AppendDisposition, EvidenceError> {
    let rows = sqlx::query(
        "SELECT seq, effect_key, payload_sha256,
                provider_operation_id_sha256, status, schema_version,
                payload_json, record_sha256, recorded_at_ms
         FROM provider_effect_acknowledgements
         WHERE effect_key = ? AND provider_operation_id_sha256 = ?
           AND status = ? AND payload_sha256 = ?",
    )
    .bind(ack.key.as_str())
    .bind(ack.provider_operation_id_sha256.as_str())
    .bind(ack_status_as_str(ack.status))
    .bind(ack.payload_sha256.as_str())
    .fetch_all(&mut **transaction)
    .await
    .map_err(classify_sqlx_error)?;
    if rows.len() != 1 {
        return Err(EvidenceError::Corrupt(format!(
            "expected one provider effect acknowledgement row but found {}",
            rows.len()
        )));
    }
    let intent = load_effect_intent_in_transaction(transaction, &ack.key)
        .await?
        .ok_or_else(|| EvidenceError::Corrupt("provider effect ACK lost its intent".to_string()))?;
    let stored = decode_effect_ack_row(&rows[0], &intent.intent)?;
    if stored.ack != *ack
        || rows[0].get::<String, _>("payload_json") != payload_json
        || rows[0].get::<String, _>("record_sha256") != record_sha256
    {
        return Err(EvidenceError::IdempotencyConflict {
            record_id: ack.key.as_str().to_string(),
        });
    }
    Ok(if inserted {
        AppendDisposition::Inserted
    } else {
        AppendDisposition::AlreadyPresent
    })
}

async fn verify_effect_uncertainty(
    transaction: &mut Transaction<'_, Sqlite>,
    uncertainty: &ProviderEffectUncertainty,
    payload_json: &str,
    record_sha256: &str,
    inserted: bool,
) -> Result<AppendDisposition, EvidenceError> {
    let rows = sqlx::query(
        "SELECT seq, effect_key, payload_sha256, reason_code,
                schema_version, payload_json, record_sha256, recorded_at_ms
         FROM provider_effect_uncertainties
         WHERE effect_key = ? AND payload_sha256 = ? AND reason_code = ?",
    )
    .bind(uncertainty.key.as_str())
    .bind(uncertainty.payload_sha256.as_str())
    .bind(&uncertainty.reason_code)
    .fetch_all(&mut **transaction)
    .await
    .map_err(classify_sqlx_error)?;
    if rows.len() != 1 {
        return Err(EvidenceError::Corrupt(format!(
            "expected one provider effect uncertainty row but found {}",
            rows.len()
        )));
    }
    let intent = load_effect_intent_in_transaction(transaction, &uncertainty.key)
        .await?
        .ok_or_else(|| {
            EvidenceError::Corrupt("provider effect uncertainty lost its intent".to_string())
        })?;
    let stored = decode_effect_uncertainty_row(&rows[0], &intent.intent)?;
    if stored.uncertainty != *uncertainty
        || rows[0].get::<String, _>("payload_json") != payload_json
        || rows[0].get::<String, _>("record_sha256") != record_sha256
    {
        return Err(EvidenceError::IdempotencyConflict {
            record_id: uncertainty.key.as_str().to_string(),
        });
    }
    Ok(if inserted {
        AppendDisposition::Inserted
    } else {
        AppendDisposition::AlreadyPresent
    })
}

async fn validate_ack_transition_in_transaction(
    transaction: &mut Transaction<'_, Sqlite>,
    intent: &ProviderEffectIntent,
    next: &ProviderEffectAck,
) -> Result<(), EvidenceError> {
    let rows = sqlx::query(
        "SELECT seq, effect_key, payload_sha256,
                provider_operation_id_sha256, status, schema_version,
                payload_json, record_sha256, recorded_at_ms
         FROM provider_effect_acknowledgements
         WHERE effect_key = ? ORDER BY seq ASC",
    )
    .bind(next.key.as_str())
    .fetch_all(&mut **transaction)
    .await
    .map_err(classify_sqlx_error)?;
    let mut previous = None;
    for row in rows {
        previous = Some(decode_effect_ack_row(&row, intent)?);
    }
    let uncertainty_recorded_at_ms: Option<i64> = sqlx::query_scalar(
        "SELECT MAX(recorded_at_ms) FROM provider_effect_uncertainties
         WHERE effect_key = ?",
    )
    .bind(next.key.as_str())
    .fetch_one(&mut **transaction)
    .await
    .map_err(classify_sqlx_error)?;
    let Some(previous) = previous else {
        return Ok(());
    };
    if previous.ack == *next {
        return Ok(());
    }
    if uncertainty_recorded_at_ms.is_some_and(|timestamp| timestamp > previous.recorded_at_ms) {
        if previous.ack.state().is_terminal() {
            return Err(EvidenceError::IdempotencyConflict {
                record_id: next.key.as_str().to_string(),
            });
        }
        // An uncertainty marker permits a later status lookup to bind the
        // provider's authoritative operation id, but it must not weaken the
        // local monotonic state machine.  Once an operation was accepted,
        // only another Accepted observation or a Completed receipt can close
        // it.  Accepted -> Rejected remains fail-closed because the earlier
        // admission leaves open the possibility that the provider applied the
        // effect before the response was lost.
        let legal_after_uncertainty = matches!(
            (previous.ack.status, next.status),
            (
                ProviderEffectAckStatus::Accepted,
                ProviderEffectAckStatus::Accepted
            ) | (
                ProviderEffectAckStatus::Accepted,
                ProviderEffectAckStatus::Completed
            )
        );
        if legal_after_uncertainty {
            return Ok(());
        }
        return Err(EvidenceError::IdempotencyConflict {
            record_id: next.key.as_str().to_string(),
        });
    }
    if previous.ack.provider_operation_id_sha256 != next.provider_operation_id_sha256 {
        return Err(EvidenceError::IdempotencyConflict {
            record_id: next.key.as_str().to_string(),
        });
    }
    let legal = matches!(
        (previous.ack.status, next.status),
        (
            ProviderEffectAckStatus::Accepted,
            ProviderEffectAckStatus::Completed
        ) | (
            ProviderEffectAckStatus::Accepted,
            ProviderEffectAckStatus::Accepted
        )
    );
    if legal {
        Ok(())
    } else {
        Err(EvidenceError::IdempotencyConflict {
            record_id: next.key.as_str().to_string(),
        })
    }
}

async fn latest_ack_is_terminal(
    transaction: &mut Transaction<'_, Sqlite>,
    intent: &ProviderEffectIntent,
    key: &ProviderEffectKey,
) -> Result<bool, EvidenceError> {
    let row = sqlx::query(
        "SELECT seq, effect_key, payload_sha256,
                provider_operation_id_sha256, status, schema_version,
                payload_json, record_sha256, recorded_at_ms
         FROM provider_effect_acknowledgements
         WHERE effect_key = ? ORDER BY recorded_at_ms DESC, seq DESC LIMIT 1",
    )
    .bind(key.as_str())
    .fetch_optional(&mut **transaction)
    .await
    .map_err(classify_sqlx_error)?;
    row.map(|row| decode_effect_ack_row(&row, intent).map(|ack| ack.ack.state().is_terminal()))
        .transpose()
        .map(|value| value.unwrap_or(false))
}

async fn next_effect_recorded_at_ms(
    transaction: &mut Transaction<'_, Sqlite>,
    key: &ProviderEffectKey,
) -> Result<i64, EvidenceError> {
    let latest_ack: Option<i64> = sqlx::query_scalar(
        "SELECT MAX(recorded_at_ms) FROM provider_effect_acknowledgements
         WHERE effect_key = ?",
    )
    .bind(key.as_str())
    .fetch_one(&mut **transaction)
    .await
    .map_err(classify_sqlx_error)?;
    let latest_uncertainty: Option<i64> = sqlx::query_scalar(
        "SELECT MAX(recorded_at_ms) FROM provider_effect_uncertainties
         WHERE effect_key = ?",
    )
    .bind(key.as_str())
    .fetch_one(&mut **transaction)
    .await
    .map_err(classify_sqlx_error)?;
    let now = now_millis()?;
    let previous = latest_ack.unwrap_or(0).max(latest_uncertainty.unwrap_or(0));
    Ok(now.max(previous.saturating_add(1)))
}

fn verify_record_digest(
    payload_json: &str,
    expected: &str,
    description: &str,
) -> Result<(), EvidenceError> {
    let actual = Sha256Digest::for_bytes(payload_json.as_bytes());
    if actual.as_str() == expected {
        Ok(())
    } else {
        Err(EvidenceError::Corrupt(format!(
            "{description} canonical record digest does not match stored digest"
        )))
    }
}

fn ack_status_as_str(status: ProviderEffectAckStatus) -> &'static str {
    match status {
        ProviderEffectAckStatus::Accepted => "accepted",
        ProviderEffectAckStatus::Completed => "completed",
        ProviderEffectAckStatus::Rejected => "rejected",
    }
}

fn binding_invalid(error: ProviderEffectBindingError) -> EvidenceError {
    EvidenceError::InvalidRecord(format!("provider effect binding is invalid: {error:?}"))
}

fn binding_corrupt(error: ProviderEffectBindingError) -> EvidenceError {
    EvidenceError::Corrupt(format!("provider effect binding is corrupt: {error:?}"))
}
