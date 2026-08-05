use codex_hepta_contracts::PROVIDER_EVIDENCE_SCHEMA_VERSION;
use codex_hepta_contracts::ProviderAttemptId;
use codex_hepta_contracts::ProviderInvocationIntent;
use codex_hepta_contracts::ProviderInvocationReceipt;
use codex_hepta_contracts::ProviderReceiptId;
use codex_hepta_contracts::ProviderTerminal;
use codex_hepta_contracts::RequestBindingId;
use codex_hepta_contracts::Sha256Digest;
use sqlx::Row;
use sqlx::Sqlite;
use sqlx::Transaction;
use sqlx::sqlite::SqliteRow;

use crate::AppendDisposition;
use crate::EvidenceError;
use crate::HeptaEvidenceStore;
use crate::canonical::canonical_json;
use crate::store::classify_sqlx_error;
use crate::store::now_millis;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StoredProviderIntent {
    pub seq: i64,
    pub intent: ProviderInvocationIntent,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StoredProviderReceipt {
    pub seq: i64,
    pub receipt: ProviderInvocationReceipt,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StoredProviderAttemptEvidence {
    pub intent: StoredProviderIntent,
    pub receipt: Option<StoredProviderReceipt>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProviderBindingState {
    Pending,
    Completed,
    Indeterminate,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProviderIntentClaimDisposition {
    Inserted,
    ExactReplay,
    BlockedByBinding(ProviderBindingState),
}

impl HeptaEvidenceStore {
    /// Atomically claims one enforce-mode provider attempt.
    ///
    /// A host logical-request binding with a pending, completed, or
    /// indeterminate attempt cannot be sent again, even if a retry changes
    /// transport or incremental wire encoding. Rejected and
    /// provably-not-dispatched attempts do not prevent a fresh physical send.
    /// The binding check and insert share one `BEGIN IMMEDIATE` transaction,
    /// so concurrent pools cannot both win.
    pub async fn claim_provider_intent(
        &self,
        intent: &ProviderInvocationIntent,
    ) -> Result<ProviderIntentClaimDisposition, EvidenceError> {
        validate_provider_intent(intent)?;
        let payload = canonical_json(intent)?;
        let payload_json = String::from_utf8(payload.clone())
            .map_err(|error| EvidenceError::Serialization(error.to_string()))?;
        let payload_sha256 = Sha256Digest::for_bytes(&payload);
        let mut transaction = self
            .pool
            .begin_with("BEGIN IMMEDIATE")
            .await
            .map_err(classify_sqlx_error)?;
        if let Some(state) = blocking_provider_binding_state(&mut transaction, intent).await? {
            transaction.commit().await.map_err(classify_sqlx_error)?;
            return Ok(ProviderIntentClaimDisposition::BlockedByBinding(state));
        }
        let inserted = insert_provider_intent(
            &mut transaction,
            intent,
            &payload_json,
            payload_sha256.as_str(),
        )
        .await?;
        transaction.commit().await.map_err(classify_sqlx_error)?;
        Ok(if inserted == AppendDisposition::Inserted {
            ProviderIntentClaimDisposition::Inserted
        } else {
            ProviderIntentClaimDisposition::ExactReplay
        })
    }

    pub async fn append_provider_intent(
        &self,
        intent: &ProviderInvocationIntent,
    ) -> Result<AppendDisposition, EvidenceError> {
        validate_provider_intent(intent)?;
        let payload = canonical_json(intent)?;
        let payload_json = String::from_utf8(payload.clone())
            .map_err(|error| EvidenceError::Serialization(error.to_string()))?;
        let payload_sha256 = Sha256Digest::for_bytes(&payload);
        let mut transaction = self
            .pool
            .begin_with("BEGIN IMMEDIATE")
            .await
            .map_err(classify_sqlx_error)?;
        let disposition = insert_provider_intent(
            &mut transaction,
            intent,
            &payload_json,
            payload_sha256.as_str(),
        )
        .await?;
        transaction.commit().await.map_err(classify_sqlx_error)?;
        Ok(disposition)
    }

    pub async fn append_provider_receipt(
        &self,
        receipt: &ProviderInvocationReceipt,
    ) -> Result<AppendDisposition, EvidenceError> {
        validate_provider_receipt(receipt)?;
        let payload = canonical_json(receipt)?;
        let payload_json = String::from_utf8(payload.clone())
            .map_err(|error| EvidenceError::Serialization(error.to_string()))?;
        let payload_sha256 = Sha256Digest::for_bytes(&payload);
        let mut transaction = self
            .pool
            .begin_with("BEGIN IMMEDIATE")
            .await
            .map_err(classify_sqlx_error)?;
        ensure_provider_intent(&mut transaction, &receipt.intent).await?;
        let insert = sqlx::query(
            "INSERT INTO provider_invocation_terminals (
                receipt_id, attempt_id, request_binding_id, thread_id, turn_id,
                terminal_kind, schema_version, payload_json, payload_sha256, recorded_at_ms
             ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
             ON CONFLICT DO NOTHING",
        )
        .bind(receipt.receipt_id.as_str())
        .bind(receipt.attempt_id.as_str())
        .bind(receipt.request_binding_id.as_str())
        .bind(&receipt.intent.binding.thread_id)
        .bind(&receipt.intent.binding.turn_id)
        .bind(receipt.terminal.kind())
        .bind(i64::from(PROVIDER_EVIDENCE_SCHEMA_VERSION))
        .bind(&payload_json)
        .bind(payload_sha256.as_str())
        .bind(now_millis()?)
        .execute(&mut *transaction)
        .await
        .map_err(classify_sqlx_error)?;
        let disposition = verify_provider_receipt(
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

    pub async fn get_provider_attempt(
        &self,
        attempt_id: &ProviderAttemptId,
    ) -> Result<Option<StoredProviderAttemptEvidence>, EvidenceError> {
        let Some(intent) = self.load_provider_intent(attempt_id).await? else {
            return Ok(None);
        };
        let receipt_row = sqlx::query(
            "SELECT seq, receipt_id, attempt_id, request_binding_id, thread_id, turn_id,
                    terminal_kind, schema_version, payload_json, payload_sha256
             FROM provider_invocation_terminals WHERE attempt_id = ?",
        )
        .bind(attempt_id.as_str())
        .fetch_optional(&self.pool)
        .await
        .map_err(classify_sqlx_error)?;
        let receipt = receipt_row
            .map(|row| {
                let seq = row.get("seq");
                decode_provider_receipt_row(&row)
                    .map(|receipt| StoredProviderReceipt { seq, receipt })
            })
            .transpose()?;
        if receipt
            .as_ref()
            .is_some_and(|stored| stored.receipt.intent != intent.intent)
        {
            return Err(EvidenceError::Corrupt(
                "provider terminal intent differs from authoritative intent row".to_string(),
            ));
        }
        Ok(Some(StoredProviderAttemptEvidence { intent, receipt }))
    }

    pub async fn get_provider_receipt(
        &self,
        receipt_id: &ProviderReceiptId,
    ) -> Result<Option<StoredProviderReceipt>, EvidenceError> {
        let row = sqlx::query(
            "SELECT seq, receipt_id, attempt_id, request_binding_id, thread_id, turn_id,
                    terminal_kind, schema_version, payload_json, payload_sha256
             FROM provider_invocation_terminals WHERE receipt_id = ?",
        )
        .bind(receipt_id.as_str())
        .fetch_optional(&self.pool)
        .await
        .map_err(classify_sqlx_error)?;
        let Some(row) = row else {
            return Ok(None);
        };
        let receipt = decode_provider_receipt_row(&row)?;
        let authoritative = self
            .load_provider_intent(&receipt.attempt_id)
            .await?
            .ok_or_else(|| {
                EvidenceError::Corrupt("provider terminal references a missing intent".to_string())
            })?;
        if authoritative.intent != receipt.intent {
            return Err(EvidenceError::Corrupt(
                "provider terminal intent differs from authoritative intent row".to_string(),
            ));
        }
        Ok(Some(StoredProviderReceipt {
            seq: row.get("seq"),
            receipt,
        }))
    }

    pub async fn list_pending_provider_intents(
        &self,
        thread_id: &str,
        after_seq: i64,
        limit: u32,
    ) -> Result<Vec<StoredProviderIntent>, EvidenceError> {
        if thread_id.trim().is_empty() {
            return Err(EvidenceError::InvalidRecord(
                "pending provider query requires a thread id".to_string(),
            ));
        }
        if !(1..=1_000).contains(&limit) {
            return Err(EvidenceError::InvalidRecord(
                "pending provider query limit must be between 1 and 1000".to_string(),
            ));
        }
        let rows = sqlx::query(
            "SELECT intents.seq, intents.attempt_id, intents.request_binding_id,
                    intents.attempt_nonce_sha256, intents.host_request_binding_id_sha256,
                    intents.thread_id, intents.turn_id,
                    intents.request_kind, intents.provider_id,
                    intents.provider_config_sha256, intents.model, intents.transport,
                    intents.endpoint_sha256, intents.logical_request_sha256,
                    intents.wire_semantic_sha256, intents.previous_response_id_sha256,
                    intents.generate, intents.schema_version,
                    intents.payload_json, intents.payload_sha256
             FROM provider_invocation_intents AS intents
             LEFT JOIN provider_invocation_terminals AS terminals
               ON terminals.attempt_id = intents.attempt_id
             WHERE intents.thread_id = ? AND intents.seq > ? AND terminals.attempt_id IS NULL
             ORDER BY intents.seq ASC LIMIT ?",
        )
        .bind(thread_id)
        .bind(after_seq)
        .bind(i64::from(limit))
        .fetch_all(&self.pool)
        .await
        .map_err(classify_sqlx_error)?;
        rows.into_iter()
            .map(|row| {
                let seq = row.get("seq");
                decode_provider_intent_row(&row).map(|intent| StoredProviderIntent { seq, intent })
            })
            .collect()
    }

    pub async fn pending_provider_attempt_count(&self) -> Result<i64, EvidenceError> {
        sqlx::query_scalar(
            "SELECT COUNT(*)
             FROM provider_invocation_intents AS intents
             LEFT JOIN provider_invocation_terminals AS terminals
               ON terminals.attempt_id = intents.attempt_id
             WHERE terminals.attempt_id IS NULL",
        )
        .fetch_one(&self.pool)
        .await
        .map_err(classify_sqlx_error)
    }

    async fn load_provider_intent(
        &self,
        attempt_id: &ProviderAttemptId,
    ) -> Result<Option<StoredProviderIntent>, EvidenceError> {
        let row = sqlx::query(
            "SELECT seq, attempt_id, request_binding_id, attempt_nonce_sha256,
                    host_request_binding_id_sha256, thread_id, turn_id,
                    request_kind, provider_id, provider_config_sha256, model, transport,
                    endpoint_sha256, logical_request_sha256, wire_semantic_sha256,
                    previous_response_id_sha256, generate, schema_version,
                    payload_json, payload_sha256
             FROM provider_invocation_intents WHERE attempt_id = ?",
        )
        .bind(attempt_id.as_str())
        .fetch_optional(&self.pool)
        .await
        .map_err(classify_sqlx_error)?;
        row.map(|row| {
            let seq = row.get("seq");
            decode_provider_intent_row(&row).map(|intent| StoredProviderIntent { seq, intent })
        })
        .transpose()
    }
}

async fn blocking_provider_binding_state(
    transaction: &mut Transaction<'_, Sqlite>,
    requested: &ProviderInvocationIntent,
) -> Result<Option<ProviderBindingState>, EvidenceError> {
    let rows = sqlx::query(
        "SELECT seq, attempt_id, request_binding_id, attempt_nonce_sha256,
                host_request_binding_id_sha256, thread_id, turn_id,
                request_kind, provider_id, provider_config_sha256, model, transport,
                endpoint_sha256, logical_request_sha256, wire_semantic_sha256,
                previous_response_id_sha256, generate, schema_version,
                payload_json, payload_sha256
         FROM provider_invocation_intents
         WHERE host_request_binding_id_sha256 = ? AND attempt_id <> ?
         ORDER BY seq ASC",
    )
    .bind(requested.binding.host_request_binding_id_sha256.as_str())
    .bind(requested.attempt_id.as_str())
    .fetch_all(&mut **transaction)
    .await
    .map_err(classify_sqlx_error)?;
    for row in rows {
        let existing = decode_provider_intent_row(&row)?;
        let terminal_row = sqlx::query(
            "SELECT seq, receipt_id, attempt_id, request_binding_id, thread_id, turn_id,
                    terminal_kind, schema_version, payload_json, payload_sha256
             FROM provider_invocation_terminals WHERE attempt_id = ?",
        )
        .bind(existing.attempt_id.as_str())
        .fetch_optional(&mut **transaction)
        .await
        .map_err(classify_sqlx_error)?;
        let Some(terminal_row) = terminal_row else {
            return Ok(Some(ProviderBindingState::Pending));
        };
        let receipt = decode_provider_receipt_row(&terminal_row)?;
        if receipt.intent != existing {
            return Err(EvidenceError::Corrupt(
                "provider binding terminal differs from its authoritative intent".to_string(),
            ));
        }
        match receipt.terminal {
            ProviderTerminal::Completed { .. } => {
                return Ok(Some(ProviderBindingState::Completed));
            }
            ProviderTerminal::Indeterminate { .. } => {
                return Ok(Some(ProviderBindingState::Indeterminate));
            }
            ProviderTerminal::Rejected { .. } | ProviderTerminal::NotDispatched { .. } => {}
        }
    }
    Ok(None)
}

async fn insert_provider_intent(
    transaction: &mut Transaction<'_, Sqlite>,
    intent: &ProviderInvocationIntent,
    payload_json: &str,
    payload_sha256: &str,
) -> Result<AppendDisposition, EvidenceError> {
    let binding = &intent.binding;
    let insert = sqlx::query(
        "INSERT INTO provider_invocation_intents (
            attempt_id, request_binding_id, attempt_nonce_sha256,
            host_request_binding_id_sha256, thread_id, turn_id,
            request_kind, provider_id, provider_config_sha256, model, transport,
            endpoint_sha256, logical_request_sha256, wire_semantic_sha256,
            previous_response_id_sha256, generate, schema_version,
            payload_json, payload_sha256, recorded_at_ms
         ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
         ON CONFLICT DO NOTHING",
    )
    .bind(intent.attempt_id.as_str())
    .bind(intent.request_binding_id.as_str())
    .bind(intent.attempt_nonce_sha256.as_str())
    .bind(binding.host_request_binding_id_sha256.as_str())
    .bind(&binding.thread_id)
    .bind(&binding.turn_id)
    .bind(binding.request_kind.as_str())
    .bind(&binding.provider_id)
    .bind(binding.provider_config_sha256.as_str())
    .bind(&binding.model)
    .bind(binding.transport.as_str())
    .bind(binding.endpoint_sha256.as_str())
    .bind(binding.logical_request_sha256.as_str())
    .bind(binding.wire_semantic_sha256.as_str())
    .bind(
        binding
            .previous_response_id_sha256
            .as_ref()
            .map(Sha256Digest::as_str),
    )
    .bind(binding.generate)
    .bind(i64::from(PROVIDER_EVIDENCE_SCHEMA_VERSION))
    .bind(payload_json)
    .bind(payload_sha256)
    .bind(now_millis()?)
    .execute(&mut **transaction)
    .await
    .map_err(classify_sqlx_error)?;
    verify_provider_intent(
        transaction,
        intent,
        payload_json,
        payload_sha256,
        insert.rows_affected() == 1,
    )
    .await
}

async fn ensure_provider_intent(
    transaction: &mut Transaction<'_, Sqlite>,
    intent: &ProviderInvocationIntent,
) -> Result<(), EvidenceError> {
    let payload = canonical_json(intent)?;
    let payload_json = String::from_utf8(payload.clone())
        .map_err(|error| EvidenceError::Serialization(error.to_string()))?;
    let digest = Sha256Digest::for_bytes(&payload);
    verify_provider_intent(transaction, intent, &payload_json, digest.as_str(), false)
        .await
        .map(|_| ())
}

async fn verify_provider_intent(
    transaction: &mut Transaction<'_, Sqlite>,
    intent: &ProviderInvocationIntent,
    payload_json: &str,
    payload_sha256: &str,
    inserted: bool,
) -> Result<AppendDisposition, EvidenceError> {
    let rows = sqlx::query(
        "SELECT attempt_id, request_binding_id, attempt_nonce_sha256,
                host_request_binding_id_sha256, thread_id, turn_id,
                request_kind, provider_id, provider_config_sha256, model, transport,
                endpoint_sha256, logical_request_sha256, wire_semantic_sha256,
                previous_response_id_sha256, generate, schema_version,
                payload_json, payload_sha256
         FROM provider_invocation_intents WHERE attempt_id = ?",
    )
    .bind(intent.attempt_id.as_str())
    .fetch_all(&mut **transaction)
    .await
    .map_err(classify_sqlx_error)?;
    if rows.len() != 1 {
        return Err(EvidenceError::Corrupt(format!(
            "expected one provider intent row for {} but found {}",
            intent.attempt_id.as_str(),
            rows.len()
        )));
    }
    let row = &rows[0];
    let stored = decode_provider_intent_row(row)?;
    let exact = row.get::<String, _>("attempt_id") == intent.attempt_id.as_str()
        && row.get::<String, _>("request_binding_id") == intent.request_binding_id.as_str()
        && row.get::<String, _>("payload_json") == payload_json
        && row.get::<String, _>("payload_sha256") == payload_sha256
        && stored == *intent;
    if !exact {
        return Err(EvidenceError::IdempotencyConflict {
            record_id: intent.attempt_id.as_str().to_string(),
        });
    }
    Ok(if inserted {
        AppendDisposition::Inserted
    } else {
        AppendDisposition::AlreadyPresent
    })
}

async fn verify_provider_receipt(
    transaction: &mut Transaction<'_, Sqlite>,
    receipt: &ProviderInvocationReceipt,
    payload_json: &str,
    payload_sha256: &str,
    inserted: bool,
) -> Result<AppendDisposition, EvidenceError> {
    let rows = sqlx::query(
        "SELECT receipt_id, attempt_id, request_binding_id, thread_id, turn_id,
                terminal_kind, schema_version, payload_json, payload_sha256
         FROM provider_invocation_terminals
         WHERE receipt_id = ? OR attempt_id = ?",
    )
    .bind(receipt.receipt_id.as_str())
    .bind(receipt.attempt_id.as_str())
    .fetch_all(&mut **transaction)
    .await
    .map_err(classify_sqlx_error)?;
    if rows.len() != 1 {
        return Err(EvidenceError::Corrupt(format!(
            "expected one provider terminal row for {} but found {}",
            receipt.receipt_id.as_str(),
            rows.len()
        )));
    }
    let row = &rows[0];
    let stored = decode_provider_receipt_row(row)?;
    let exact = row.get::<String, _>("receipt_id") == receipt.receipt_id.as_str()
        && row.get::<String, _>("attempt_id") == receipt.attempt_id.as_str()
        && row.get::<String, _>("request_binding_id") == receipt.request_binding_id.as_str()
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

fn decode_provider_intent_row(row: &SqliteRow) -> Result<ProviderInvocationIntent, EvidenceError> {
    let payload_json: String = row.get("payload_json");
    verify_stored_digest(&payload_json, row.get("payload_sha256"), "provider intent")?;
    let intent: ProviderInvocationIntent = serde_json::from_str(&payload_json)
        .map_err(|error| EvidenceError::Corrupt(error.to_string()))?;
    validate_provider_intent(&intent).map_err(invalid_as_corrupt)?;
    verify_canonical_payload(&intent, &payload_json, "provider intent")?;
    let binding = &intent.binding;
    if row.get::<String, _>("attempt_id") != intent.attempt_id.as_str()
        || row.get::<String, _>("request_binding_id") != intent.request_binding_id.as_str()
        || row.get::<String, _>("attempt_nonce_sha256") != intent.attempt_nonce_sha256.as_str()
        || row
            .get::<Option<String>, _>("host_request_binding_id_sha256")
            .as_deref()
            != Some(binding.host_request_binding_id_sha256.as_str())
        || row.get::<String, _>("thread_id") != binding.thread_id
        || row.get::<String, _>("turn_id") != binding.turn_id
        || row.get::<String, _>("request_kind") != binding.request_kind.as_str()
        || row.get::<String, _>("provider_id") != binding.provider_id
        || row.get::<String, _>("provider_config_sha256") != binding.provider_config_sha256.as_str()
        || row.get::<String, _>("model") != binding.model
        || row.get::<String, _>("transport") != binding.transport.as_str()
        || row.get::<String, _>("endpoint_sha256") != binding.endpoint_sha256.as_str()
        || row.get::<String, _>("logical_request_sha256") != binding.logical_request_sha256.as_str()
        || row.get::<String, _>("wire_semantic_sha256") != binding.wire_semantic_sha256.as_str()
        || row
            .get::<Option<String>, _>("previous_response_id_sha256")
            .as_deref()
            != binding
                .previous_response_id_sha256
                .as_ref()
                .map(Sha256Digest::as_str)
        || row.get::<bool, _>("generate") != binding.generate
        || row.get::<i64, _>("schema_version") != i64::from(intent.schema_version)
    {
        return Err(EvidenceError::Corrupt(
            "provider intent columns do not match canonical payload".to_string(),
        ));
    }
    Ok(intent)
}

fn decode_provider_receipt_row(
    row: &SqliteRow,
) -> Result<ProviderInvocationReceipt, EvidenceError> {
    let payload_json: String = row.get("payload_json");
    verify_stored_digest(
        &payload_json,
        row.get("payload_sha256"),
        "provider terminal",
    )?;
    let receipt: ProviderInvocationReceipt = serde_json::from_str(&payload_json)
        .map_err(|error| EvidenceError::Corrupt(error.to_string()))?;
    validate_provider_receipt(&receipt).map_err(invalid_as_corrupt)?;
    verify_canonical_payload(&receipt, &payload_json, "provider terminal")?;
    if row.get::<String, _>("receipt_id") != receipt.receipt_id.as_str()
        || row.get::<String, _>("attempt_id") != receipt.attempt_id.as_str()
        || row.get::<String, _>("request_binding_id") != receipt.request_binding_id.as_str()
        || row.get::<String, _>("thread_id") != receipt.intent.binding.thread_id
        || row.get::<String, _>("turn_id") != receipt.intent.binding.turn_id
        || row.get::<String, _>("terminal_kind") != receipt.terminal.kind()
        || row.get::<i64, _>("schema_version") != i64::from(receipt.schema_version)
    {
        return Err(EvidenceError::Corrupt(
            "provider terminal columns do not match canonical payload".to_string(),
        ));
    }
    Ok(receipt)
}

fn validate_provider_intent(intent: &ProviderInvocationIntent) -> Result<(), EvidenceError> {
    if intent.schema_version != PROVIDER_EVIDENCE_SCHEMA_VERSION
        || intent.binding.schema_version != PROVIDER_EVIDENCE_SCHEMA_VERSION
    {
        return invalid("unsupported provider evidence schema version");
    }
    for (label, value) in [
        ("attempt nonce digest", intent.attempt_nonce_sha256.as_str()),
        ("thread id", intent.binding.thread_id.as_str()),
        ("turn id", intent.binding.turn_id.as_str()),
        ("provider id", intent.binding.provider_id.as_str()),
        ("model", intent.binding.model.as_str()),
    ] {
        if value.trim().is_empty() {
            return invalid(format!("provider intent requires a non-empty {label}"));
        }
    }
    let expected_binding = RequestBindingId::for_request(&intent.binding);
    if intent.request_binding_id != expected_binding {
        return invalid("request binding id does not bind provider request semantics");
    }
    let expected_attempt =
        ProviderAttemptId::for_send(&intent.request_binding_id, &intent.attempt_nonce_sha256);
    if intent.attempt_id != expected_attempt {
        return invalid("provider attempt id does not bind request and send nonce");
    }
    for (label, digest) in [
        ("attempt nonce", &intent.attempt_nonce_sha256),
        (
            "host request binding id",
            &intent.binding.host_request_binding_id_sha256,
        ),
        ("provider config", &intent.binding.provider_config_sha256),
        ("endpoint", &intent.binding.endpoint_sha256),
        ("logical request", &intent.binding.logical_request_sha256),
        ("wire semantic", &intent.binding.wire_semantic_sha256),
    ] {
        validate_digest(label, digest)?;
    }
    if let Some(digest) = intent.binding.previous_response_id_sha256.as_ref() {
        validate_digest("previous response id", digest)?;
    }
    Ok(())
}

fn validate_provider_receipt(receipt: &ProviderInvocationReceipt) -> Result<(), EvidenceError> {
    validate_provider_intent(&receipt.intent)?;
    if receipt.schema_version != PROVIDER_EVIDENCE_SCHEMA_VERSION {
        return invalid("unsupported provider terminal schema version");
    }
    if receipt.attempt_id != receipt.intent.attempt_id
        || receipt.request_binding_id != receipt.intent.request_binding_id
    {
        return invalid("provider terminal does not bind its exact intent");
    }
    if receipt.receipt_id != ProviderReceiptId::for_attempt(&receipt.attempt_id) {
        return invalid("provider receipt id does not bind its attempt id");
    }
    match &receipt.terminal {
        ProviderTerminal::Completed {
            response_id_sha256,
            response_items_sha256,
            token_usage_sha256,
            ..
        } => {
            validate_digest("response id", response_id_sha256)?;
            validate_digest("response items", response_items_sha256)?;
            validate_digest("token usage", token_usage_sha256)?;
        }
        ProviderTerminal::Rejected { reason_code }
        | ProviderTerminal::NotDispatched { reason_code }
        | ProviderTerminal::Indeterminate { reason_code, .. } => {
            if reason_code.trim().is_empty() {
                return invalid("provider terminal requires a non-empty reason code");
            }
            if let ProviderTerminal::Indeterminate {
                partial_response_sha256: Some(digest),
                ..
            } = &receipt.terminal
            {
                validate_digest("partial response", digest)?;
            }
        }
    }
    Ok(())
}

fn validate_digest(label: &str, digest: &Sha256Digest) -> Result<(), EvidenceError> {
    if is_canonical_sha256(digest.as_str()) {
        Ok(())
    } else {
        invalid(format!("{label} digest is not canonical lowercase SHA-256"))
    }
}

fn verify_stored_digest(
    payload_json: &str,
    expected: &str,
    record_kind: &str,
) -> Result<(), EvidenceError> {
    let actual = Sha256Digest::for_bytes(payload_json.as_bytes());
    if actual.as_str() == expected {
        Ok(())
    } else {
        Err(EvidenceError::Corrupt(format!(
            "stored {record_kind} payload digest mismatch"
        )))
    }
}

fn verify_canonical_payload<T: serde::Serialize>(
    value: &T,
    stored: &str,
    record_kind: &str,
) -> Result<(), EvidenceError> {
    let canonical = canonical_json(value)?;
    if canonical == stored.as_bytes() {
        Ok(())
    } else {
        Err(EvidenceError::Corrupt(format!(
            "stored {record_kind} JSON is not canonical"
        )))
    }
}

fn invalid_as_corrupt(error: EvidenceError) -> EvidenceError {
    match error {
        EvidenceError::InvalidRecord(detail) => EvidenceError::Corrupt(detail),
        other => other,
    }
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
