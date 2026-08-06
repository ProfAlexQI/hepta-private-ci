use codex_hepta_contracts::CHANNEL_EVIDENCE_SCHEMA_VERSION;
use codex_hepta_contracts::ChannelIngressEvent;
use codex_hepta_contracts::ChannelIngressEventId;
use codex_hepta_contracts::ChannelIngressReceipt;
use codex_hepta_contracts::ChannelIngressTerminal;
use codex_hepta_contracts::ChannelScope;
use codex_hepta_contracts::Sha256Digest;
use codex_hepta_contracts::validate_ingress_event;
use codex_hepta_contracts::validate_ingress_receipt;
use serde::Serialize;
use serde::de::DeserializeOwned;
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChannelIngressState {
    Pending,
    Accepted,
    Rejected,
    Indeterminate,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ChannelIngressClaimDisposition {
    Inserted,
    ExactReplay(ChannelIngressState),
    BlockedByUnresolved {
        event_id: ChannelIngressEventId,
        state: ChannelIngressState,
    },
    CursorMismatch {
        expected: Option<Sha256Digest>,
        observed: Option<Sha256Digest>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StoredChannelIngressEvent {
    pub seq: i64,
    pub event: ChannelIngressEvent,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StoredChannelIngressReceipt {
    pub seq: i64,
    pub receipt: ChannelIngressReceipt,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StoredChannelIngressEvidence {
    pub event: StoredChannelIngressEvent,
    pub receipt: Option<StoredChannelIngressReceipt>,
}

struct ValidatedIngressSnapshot {
    events: Vec<StoredChannelIngressEvent>,
    receipts: Vec<StoredChannelIngressReceipt>,
}

impl ValidatedIngressSnapshot {
    fn event(&self, event_id: &ChannelIngressEventId) -> Option<&StoredChannelIngressEvent> {
        self.events
            .iter()
            .find(|stored| stored.event.event_id == *event_id)
    }

    fn receipt(&self, event_id: &ChannelIngressEventId) -> Option<&StoredChannelIngressReceipt> {
        self.receipts
            .iter()
            .find(|stored| stored.receipt.event_id == *event_id)
    }

    fn state(&self, event_id: &ChannelIngressEventId) -> ChannelIngressState {
        self.receipt(event_id)
            .map_or(ChannelIngressState::Pending, |stored| {
                match stored.receipt.terminal {
                    ChannelIngressTerminal::Accepted { .. } => ChannelIngressState::Accepted,
                    ChannelIngressTerminal::Rejected { .. } => ChannelIngressState::Rejected,
                    ChannelIngressTerminal::Indeterminate { .. } => {
                        ChannelIngressState::Indeterminate
                    }
                }
            })
    }

    fn unresolved(
        &self,
        scope_sha256: &Sha256Digest,
    ) -> Option<(ChannelIngressEventId, ChannelIngressState)> {
        self.events.iter().find_map(|stored| {
            if stored.event.scope.binding_sha256() != *scope_sha256 {
                return None;
            }
            let state = self.state(&stored.event.event_id);
            matches!(
                state,
                ChannelIngressState::Pending | ChannelIngressState::Indeterminate
            )
            .then(|| (stored.event.event_id.clone(), state))
        })
    }

    fn current_cursor(&self, scope_sha256: &Sha256Digest) -> Option<Sha256Digest> {
        self.events.iter().rev().find_map(|stored| {
            if stored.event.scope.binding_sha256() != *scope_sha256 {
                return None;
            }
            matches!(
                self.state(&stored.event.event_id),
                ChannelIngressState::Accepted | ChannelIngressState::Rejected
            )
            .then(|| stored.event.next_cursor_sha256.clone())
        })
    }

    fn validate_cross_row_integrity(&self) -> Result<(), EvidenceError> {
        let mut event_ids = BTreeSet::new();
        let mut source_identities = BTreeSet::new();
        for stored in &self.events {
            let scope_sha256 = stored.event.scope.binding_sha256();
            if !event_ids.insert(stored.event.event_id.as_str().to_string())
                || !source_identities.insert((
                    scope_sha256.as_str().to_string(),
                    stored.event.source_event_sha256.as_str().to_string(),
                ))
            {
                return Err(EvidenceError::Corrupt(
                    "duplicate channel ingress event identity".to_string(),
                ));
            }
        }

        let mut receipt_ids = BTreeSet::new();
        let mut receipt_event_ids = BTreeSet::new();
        for receipt in &self.receipts {
            if !receipt_ids.insert(receipt.receipt.receipt_id.as_str().to_string())
                || !receipt_event_ids.insert(receipt.receipt.event_id.as_str().to_string())
            {
                return Err(EvidenceError::Corrupt(
                    "duplicate channel ingress receipt identity".to_string(),
                ));
            }
            let Some(event) = self.event(&receipt.receipt.event_id) else {
                return Err(EvidenceError::Corrupt(
                    "channel ingress receipt references a missing canonical event".to_string(),
                ));
            };
            if receipt.receipt.event != event.event {
                return Err(EvidenceError::Corrupt(
                    "channel ingress receipt differs from its canonical event".to_string(),
                ));
            }
        }

        let mut last_by_scope = BTreeMap::new();
        for stored in &self.events {
            let scope_sha256 = stored.event.scope.binding_sha256();
            let predecessor = stored.event.predecessor_cursor_sha256.as_ref();
            match last_by_scope.get(scope_sha256.as_str()) {
                None if predecessor.is_none() => {}
                Some((cursor, state))
                    if matches!(
                        state,
                        ChannelIngressState::Accepted | ChannelIngressState::Rejected
                    ) && predecessor == Some(cursor) => {}
                _ => {
                    return Err(EvidenceError::Corrupt(
                        "channel ingress rows do not form one definitive cursor chain".to_string(),
                    ));
                }
            }
            last_by_scope.insert(
                scope_sha256.as_str().to_string(),
                (
                    stored.event.next_cursor_sha256.clone(),
                    self.state(&stored.event.event_id),
                ),
            );
        }
        Ok(())
    }
}

impl HeptaEvidenceStore {
    pub async fn claim_channel_ingress_event(
        &self,
        event: &ChannelIngressEvent,
    ) -> Result<ChannelIngressClaimDisposition, EvidenceError> {
        validate_ingress_event(event).map_err(invalid_record)?;
        let payload_json = canonical_payload_json(event)?;
        let evidence_sha256 = Sha256Digest::for_bytes(payload_json.as_bytes());
        let scope_sha256 = event.scope.binding_sha256();
        let mut transaction = self
            .pool
            .begin_with("BEGIN IMMEDIATE")
            .await
            .map_err(classify_sqlx_error)?;
        let snapshot = load_ingress_snapshot(&mut transaction).await?;

        if let Some(existing) = snapshot.event(&event.event_id) {
            if existing.event != *event {
                return Err(identity_conflict(event.event_id.as_str()));
            }
            let state = snapshot.state(&event.event_id);
            transaction.commit().await.map_err(classify_sqlx_error)?;
            return Ok(ChannelIngressClaimDisposition::ExactReplay(state));
        }

        if let Some((event_id, state)) = snapshot.unresolved(&scope_sha256) {
            transaction.commit().await.map_err(classify_sqlx_error)?;
            return Ok(ChannelIngressClaimDisposition::BlockedByUnresolved { event_id, state });
        }

        let current_cursor = snapshot.current_cursor(&scope_sha256);
        if current_cursor != event.predecessor_cursor_sha256 {
            transaction.commit().await.map_err(classify_sqlx_error)?;
            return Ok(ChannelIngressClaimDisposition::CursorMismatch {
                expected: event.predecessor_cursor_sha256.clone(),
                observed: current_cursor,
            });
        }

        let insert = sqlx::query(
            "INSERT INTO channel_ingress_events (
                event_id, scope_sha256, adapter_id, source_event_sha256,
                event_payload_sha256, target_thread_sha256,
                predecessor_cursor_sha256, next_cursor_sha256,
                received_at_unix_ms, schema_version, payload_json, evidence_sha256,
                recorded_at_ms
             ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
             ON CONFLICT DO NOTHING",
        )
        .bind(event.event_id.as_str())
        .bind(scope_sha256.as_str())
        .bind(event.scope.adapter_id.as_str())
        .bind(event.source_event_sha256.as_str())
        .bind(event.payload_sha256.as_str())
        .bind(event.target_thread_sha256.as_str())
        .bind(
            event
                .predecessor_cursor_sha256
                .as_ref()
                .map(Sha256Digest::as_str),
        )
        .bind(event.next_cursor_sha256.as_str())
        .bind(i64::try_from(event.received_at_unix_ms).map_err(|error| {
            EvidenceError::InvalidRecord(format!("channel receive time exceeds i64: {error}"))
        })?)
        .bind(i64::from(CHANNEL_EVIDENCE_SCHEMA_VERSION))
        .bind(&payload_json)
        .bind(evidence_sha256.as_str())
        .bind(now_millis()?)
        .execute(&mut *transaction)
        .await
        .map_err(classify_sqlx_error)?;
        if insert.rows_affected() != 1 {
            return Err(identity_conflict(event.event_id.as_str()));
        }
        let stored = load_ingress_event(&mut transaction, &event.event_id)
            .await?
            .ok_or_else(|| {
                EvidenceError::Corrupt(
                    "channel ingress insert succeeded but row is missing".to_string(),
                )
            })?;
        if stored.event != *event {
            return Err(EvidenceError::Corrupt(
                "channel ingress row differs after insert".to_string(),
            ));
        }
        transaction.commit().await.map_err(classify_sqlx_error)?;
        Ok(ChannelIngressClaimDisposition::Inserted)
    }

    pub async fn append_channel_ingress_receipt(
        &self,
        receipt: &ChannelIngressReceipt,
    ) -> Result<AppendDisposition, EvidenceError> {
        validate_ingress_receipt(receipt).map_err(invalid_record)?;
        let payload_json = canonical_payload_json(receipt)?;
        let evidence_sha256 = Sha256Digest::for_bytes(payload_json.as_bytes());
        let scope_sha256 = receipt.event.scope.binding_sha256();
        let mut transaction = self
            .pool
            .begin_with("BEGIN IMMEDIATE")
            .await
            .map_err(classify_sqlx_error)?;
        let snapshot = load_ingress_snapshot(&mut transaction).await?;
        let existing_event = snapshot.event(&receipt.event_id).ok_or_else(|| {
            EvidenceError::InvalidRecord(
                "channel ingress receipt references a missing event".to_string(),
            )
        })?;
        if existing_event.event != receipt.event {
            return Err(identity_conflict(receipt.event_id.as_str()));
        }
        if let Some(existing) = snapshot.receipt(&receipt.event_id) {
            if existing.receipt != *receipt {
                return Err(identity_conflict(receipt.receipt_id.as_str()));
            }
            transaction.commit().await.map_err(classify_sqlx_error)?;
            return Ok(AppendDisposition::AlreadyPresent);
        }
        if receipt.terminal.advances_cursor() {
            let current_cursor = snapshot.current_cursor(&scope_sha256);
            if current_cursor != receipt.event.predecessor_cursor_sha256 {
                return Err(EvidenceError::InvalidRecord(
                    "channel ingress receipt cannot advance a stale cursor predecessor".to_string(),
                ));
            }
        }
        let (thread_id, turn_id) = match &receipt.terminal {
            ChannelIngressTerminal::Accepted { thread_id, turn_id } => {
                (Some(thread_id.as_str()), Some(turn_id.as_str()))
            }
            ChannelIngressTerminal::Rejected { .. }
            | ChannelIngressTerminal::Indeterminate { .. } => (None, None),
        };
        let insert = sqlx::query(
            "INSERT INTO channel_ingress_receipts (
                receipt_id, event_id, scope_sha256, terminal_kind, thread_id,
                turn_id, schema_version, payload_json, evidence_sha256, recorded_at_ms
             ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
             ON CONFLICT DO NOTHING",
        )
        .bind(receipt.receipt_id.as_str())
        .bind(receipt.event_id.as_str())
        .bind(scope_sha256.as_str())
        .bind(receipt.terminal.kind())
        .bind(thread_id)
        .bind(turn_id)
        .bind(i64::from(CHANNEL_EVIDENCE_SCHEMA_VERSION))
        .bind(&payload_json)
        .bind(evidence_sha256.as_str())
        .bind(now_millis()?)
        .execute(&mut *transaction)
        .await
        .map_err(classify_sqlx_error)?;
        let inserted = insert.rows_affected() == 1;
        let stored = load_ingress_receipt(&mut transaction, &receipt.event_id)
            .await?
            .ok_or_else(|| {
                EvidenceError::Corrupt("channel ingress receipt insert returned no row".to_string())
            })?;
        if stored.receipt != *receipt {
            return Err(identity_conflict(receipt.receipt_id.as_str()));
        }
        transaction.commit().await.map_err(classify_sqlx_error)?;
        Ok(if inserted {
            AppendDisposition::Inserted
        } else {
            AppendDisposition::AlreadyPresent
        })
    }

    pub async fn get_channel_ingress_event(
        &self,
        event_id: &ChannelIngressEventId,
    ) -> Result<Option<StoredChannelIngressEvidence>, EvidenceError> {
        let mut transaction = self.pool.begin().await.map_err(classify_sqlx_error)?;
        let evidence =
            load_channel_ingress_evidence_in_transaction(&mut transaction, event_id).await?;
        transaction.commit().await.map_err(classify_sqlx_error)?;
        Ok(evidence)
    }

    pub async fn current_channel_cursor(
        &self,
        scope: &ChannelScope,
    ) -> Result<Option<Sha256Digest>, EvidenceError> {
        let mut transaction = self.pool.begin().await.map_err(classify_sqlx_error)?;
        let snapshot = load_ingress_snapshot(&mut transaction).await?;
        let cursor = snapshot.current_cursor(&scope.binding_sha256());
        transaction.commit().await.map_err(classify_sqlx_error)?;
        Ok(cursor)
    }
}

pub(crate) async fn load_channel_ingress_evidence_in_transaction(
    transaction: &mut Transaction<'_, Sqlite>,
    event_id: &ChannelIngressEventId,
) -> Result<Option<StoredChannelIngressEvidence>, EvidenceError> {
    let snapshot = load_ingress_snapshot(transaction).await?;
    let Some(event) = snapshot.event(event_id).cloned() else {
        return Ok(None);
    };
    let receipt = snapshot.receipt(event_id).cloned();
    Ok(Some(StoredChannelIngressEvidence { event, receipt }))
}

async fn load_ingress_snapshot(
    transaction: &mut Transaction<'_, Sqlite>,
) -> Result<ValidatedIngressSnapshot, EvidenceError> {
    let event_rows = sqlx::query(
        "SELECT seq, event_id, scope_sha256, adapter_id, source_event_sha256,
                event_payload_sha256, target_thread_sha256,
                predecessor_cursor_sha256, next_cursor_sha256,
                received_at_unix_ms, schema_version, payload_json, evidence_sha256
         FROM channel_ingress_events ORDER BY seq ASC",
    )
    .fetch_all(&mut **transaction)
    .await
    .map_err(classify_sqlx_error)?;
    let events = event_rows
        .into_iter()
        .map(|row| {
            let seq = row.get("seq");
            decode_ingress_event_row(&row).map(|event| StoredChannelIngressEvent { seq, event })
        })
        .collect::<Result<Vec<_>, _>>()?;

    let receipt_rows = sqlx::query(
        "SELECT seq, receipt_id, event_id, scope_sha256, terminal_kind,
                thread_id, turn_id, schema_version, payload_json, evidence_sha256
         FROM channel_ingress_receipts ORDER BY seq ASC",
    )
    .fetch_all(&mut **transaction)
    .await
    .map_err(classify_sqlx_error)?;
    let receipts = receipt_rows
        .into_iter()
        .map(|row| {
            let seq = row.get("seq");
            decode_ingress_receipt_row(&row)
                .map(|receipt| StoredChannelIngressReceipt { seq, receipt })
        })
        .collect::<Result<Vec<_>, _>>()?;

    let snapshot = ValidatedIngressSnapshot { events, receipts };
    snapshot.validate_cross_row_integrity()?;
    Ok(snapshot)
}

async fn load_ingress_event(
    transaction: &mut Transaction<'_, Sqlite>,
    event_id: &ChannelIngressEventId,
) -> Result<Option<StoredChannelIngressEvent>, EvidenceError> {
    let row = sqlx::query(
        "SELECT seq, event_id, scope_sha256, adapter_id, source_event_sha256,
                event_payload_sha256, target_thread_sha256,
                predecessor_cursor_sha256, next_cursor_sha256,
                received_at_unix_ms, schema_version, payload_json, evidence_sha256
         FROM channel_ingress_events WHERE event_id = ?",
    )
    .bind(event_id.as_str())
    .fetch_optional(&mut **transaction)
    .await
    .map_err(classify_sqlx_error)?;
    row.map(|row| {
        let seq = row.get("seq");
        decode_ingress_event_row(&row).map(|event| StoredChannelIngressEvent { seq, event })
    })
    .transpose()
}

async fn load_ingress_receipt(
    transaction: &mut Transaction<'_, Sqlite>,
    event_id: &ChannelIngressEventId,
) -> Result<Option<StoredChannelIngressReceipt>, EvidenceError> {
    let row = sqlx::query(
        "SELECT seq, receipt_id, event_id, scope_sha256, terminal_kind,
                thread_id, turn_id, schema_version, payload_json, evidence_sha256
         FROM channel_ingress_receipts WHERE event_id = ?",
    )
    .bind(event_id.as_str())
    .fetch_optional(&mut **transaction)
    .await
    .map_err(classify_sqlx_error)?;
    row.map(|row| {
        let seq = row.get("seq");
        decode_ingress_receipt_row(&row).map(|receipt| StoredChannelIngressReceipt { seq, receipt })
    })
    .transpose()
}

fn decode_ingress_event_row(row: &SqliteRow) -> Result<ChannelIngressEvent, EvidenceError> {
    let event = decode_payload::<ChannelIngressEvent>(row)?;
    validate_ingress_event(&event).map_err(corrupt_record)?;
    let predecessor: Option<String> = row.get("predecessor_cursor_sha256");
    if row.get::<String, _>("event_id") != event.event_id.as_str()
        || row.get::<String, _>("scope_sha256") != event.scope.binding_sha256().as_str()
        || row.get::<String, _>("adapter_id") != event.scope.adapter_id.as_str()
        || row.get::<String, _>("source_event_sha256") != event.source_event_sha256.as_str()
        || row.get::<String, _>("event_payload_sha256") != event.payload_sha256.as_str()
        || row.get::<String, _>("target_thread_sha256") != event.target_thread_sha256.as_str()
        || predecessor.as_deref()
            != event
                .predecessor_cursor_sha256
                .as_ref()
                .map(Sha256Digest::as_str)
        || row.get::<String, _>("next_cursor_sha256") != event.next_cursor_sha256.as_str()
        || row.get::<i64, _>("received_at_unix_ms")
            != i64::try_from(event.received_at_unix_ms).map_err(|error| {
                EvidenceError::Corrupt(format!("channel receive time exceeds i64: {error}"))
            })?
        || row.get::<i64, _>("schema_version") != i64::from(CHANNEL_EVIDENCE_SCHEMA_VERSION)
    {
        return Err(EvidenceError::Corrupt(
            "channel ingress columns differ from canonical payload".to_string(),
        ));
    }
    Ok(event)
}

fn decode_ingress_receipt_row(row: &SqliteRow) -> Result<ChannelIngressReceipt, EvidenceError> {
    let receipt = decode_payload::<ChannelIngressReceipt>(row)?;
    validate_ingress_receipt(&receipt).map_err(corrupt_record)?;
    let (thread_id, turn_id) = match &receipt.terminal {
        ChannelIngressTerminal::Accepted { thread_id, turn_id } => {
            (Some(thread_id.as_str()), Some(turn_id.as_str()))
        }
        ChannelIngressTerminal::Rejected { .. } | ChannelIngressTerminal::Indeterminate { .. } => {
            (None, None)
        }
    };
    if row.get::<String, _>("receipt_id") != receipt.receipt_id.as_str()
        || row.get::<String, _>("event_id") != receipt.event_id.as_str()
        || row.get::<String, _>("scope_sha256") != receipt.event.scope.binding_sha256().as_str()
        || row.get::<String, _>("terminal_kind") != receipt.terminal.kind()
        || row.get::<Option<String>, _>("thread_id").as_deref() != thread_id
        || row.get::<Option<String>, _>("turn_id").as_deref() != turn_id
        || row.get::<i64, _>("schema_version") != i64::from(CHANNEL_EVIDENCE_SCHEMA_VERSION)
    {
        return Err(EvidenceError::Corrupt(
            "channel ingress receipt columns differ from canonical payload".to_string(),
        ));
    }
    Ok(receipt)
}

fn decode_payload<T>(row: &SqliteRow) -> Result<T, EvidenceError>
where
    T: DeserializeOwned + Serialize,
{
    let payload_json: String = row.get("payload_json");
    let evidence_sha256: String = row.get("evidence_sha256");
    let expected_digest = Sha256Digest::for_bytes(payload_json.as_bytes());
    if evidence_sha256 != expected_digest.as_str() {
        return Err(EvidenceError::Corrupt(
            "channel evidence payload digest mismatch".to_string(),
        ));
    }
    let value = serde_json::from_str::<T>(&payload_json).map_err(|error| {
        EvidenceError::Corrupt(format!("invalid channel evidence JSON: {error}"))
    })?;
    if canonical_payload_json(&value)? != payload_json {
        return Err(EvidenceError::Corrupt(
            "channel evidence payload is not canonical JSON".to_string(),
        ));
    }
    Ok(value)
}

fn canonical_payload_json<T: Serialize>(value: &T) -> Result<String, EvidenceError> {
    let payload = canonical_json(value)?;
    String::from_utf8(payload).map_err(|error| EvidenceError::Serialization(error.to_string()))
}

fn invalid_record(error: String) -> EvidenceError {
    EvidenceError::InvalidRecord(error)
}

fn corrupt_record(error: String) -> EvidenceError {
    EvidenceError::Corrupt(error)
}

fn identity_conflict(record_id: &str) -> EvidenceError {
    EvidenceError::IdempotencyConflict {
        record_id: record_id.to_string(),
    }
}
use std::collections::BTreeMap;
use std::collections::BTreeSet;
