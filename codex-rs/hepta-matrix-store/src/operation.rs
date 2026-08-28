use std::path::PathBuf;

use codex_hepta_contracts::AuthorityAction;
use codex_hepta_contracts::DestinationAcknowledgement;
use codex_hepta_contracts::IdempotencyKey;
use codex_hepta_contracts::OperationBinding;
use codex_hepta_contracts::OperationId;
use codex_hepta_contracts::OperationPhase;
use codex_hepta_contracts::OutboxEnvelope;
use codex_hepta_contracts::ProductComponentId;
use codex_hepta_contracts::RecoveryDecision;
use codex_hepta_contracts::Sha256Digest;
use codex_hepta_contracts::recovery_decision;
use codex_hepta_matrix_protocol::client_user_message_id;
use codex_state::SqliteConfig;
use codex_utils_absolute_path::AbsolutePathBuf;
use sqlx::Row;
use sqlx::SqlitePool;

use crate::InboxRecord;
use crate::MatrixDurableError;
use crate::MatrixDurableStore;
use crate::MatrixEventId;

const MATRIX_OPERATION_SEQUENCE: u64 = 1;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatrixOperationRecord {
    pub event_id: MatrixEventId,
    pub envelope: OutboxEnvelope,
    pub phase: OperationPhase,
    pub destination_receipt_sha256: Option<Sha256Digest>,
    pub updated_at_ms: u64,
}

impl MatrixOperationRecord {
    pub fn recovery_decision(&self) -> RecoveryDecision {
        recovery_decision(
            self.phase,
            self.phase != OperationPhase::OutboxPending,
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatrixOperationBegin {
    pub record: MatrixOperationRecord,
    pub created: bool,
}

/// Durable operation journal for Matrix ingress -> App Server admission.
///
/// The journal opens the same Agent-private Matrix SQLite database as
/// [`MatrixDurableStore`]. It does not create a second data owner or a second
/// ingress queue. The immutable operation binding is persisted before the
/// App Server admission boundary is crossed; after a delivery claim, recovery
/// is lookup-only.
#[derive(Clone, Debug)]
pub struct MatrixOperationJournal {
    database_path: PathBuf,
    owner_agent_id: codex_hepta_contracts::AgentId,
}

impl MatrixOperationJournal {
    pub fn new(store: &MatrixDurableStore) -> Self {
        Self {
            database_path: store.path().to_path_buf(),
            owner_agent_id: store.owner_agent_id().clone(),
        }
    }

    pub async fn begin(
        &self,
        inbox: &InboxRecord,
        project_id: &str,
        at_ms: u64,
    ) -> Result<MatrixOperationBegin, MatrixDurableError> {
        if inbox.generation == 0
            || inbox.binding_revision == 0
            || inbox.payload.is_empty()
            || project_id.is_empty()
        {
            return Err(MatrixDurableError::Invalid);
        }
        let idempotency_key = client_user_message_id(
            &self.owner_agent_id,
            &inbox.room_id,
            &inbox.event_id,
        );
        let operation_id = operation_id(&self.owner_agent_id, &inbox.event_id)?;
        let command_sha256 = command_digest(inbox, &idempotency_key);
        let command_bytes = u64::try_from(inbox.payload.len())
            .map_err(|_| MatrixDurableError::Invalid)?;
        let binding = OperationBinding::new(
            operation_id,
            IdempotencyKey::parse(idempotency_key)
                .map_err(|_| MatrixDurableError::Invalid)?,
            self.owner_agent_id.clone(),
            ProductComponentId::MatrixIngress,
            self.owner_agent_id.clone(),
            ProductComponentId::AppServer,
            AuthorityAction::ServeSession,
            inbox.binding_revision,
            inbox.generation,
            inbox.generation,
            fencing_digest(
                &self.owner_agent_id,
                inbox,
                project_id,
            ),
            command_sha256,
            command_bytes,
        )
        .map_err(|_| MatrixDurableError::Invalid)?;
        let envelope = OutboxEnvelope::pending(binding, MATRIX_OPERATION_SEQUENCE)
            .map_err(|_| MatrixDurableError::Invalid)?;

        let pool = self.open_pool().await?;
        let inserted = sqlx::query(
            "INSERT INTO matrix_operations (
                event_id, operation_id, idempotency_key, binding_sha256,
                authority_epoch, owner_epoch, generation, fencing_token_sha256,
                command_sha256, command_bytes, sequence, phase,
                destination_receipt_sha256, updated_at_ms
             ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 'outbox_pending', NULL, ?)
             ON CONFLICT(event_id) DO NOTHING",
        )
        .bind(inbox.event_id.as_str())
        .bind(envelope.binding.operation_id.as_str())
        .bind(envelope.binding.idempotency_key.as_str())
        .bind(envelope.binding_sha256.as_str())
        .bind(to_i64(envelope.binding.authority_epoch)?)
        .bind(to_i64(envelope.binding.owner_epoch)?)
        .bind(to_i64(envelope.binding.generation)?)
        .bind(envelope.binding.fencing_token_sha256.as_str())
        .bind(envelope.binding.command_sha256.as_str())
        .bind(to_i64(envelope.binding.command_bytes)?)
        .bind(to_i64(envelope.sequence)?)
        .bind(to_i64(at_ms)?)
        .execute(&pool)
        .await
        .map_err(|_| MatrixDurableError::Unavailable)?;
        let created = inserted.rows_affected() == 1;
        let record = load_record(&pool, &self.owner_agent_id, &inbox.event_id)
            .await?
            .ok_or(MatrixDurableError::Corrupt)?;
        pool.close().await;
        if record.envelope != envelope {
            return Err(MatrixDurableError::Conflict);
        }
        Ok(MatrixOperationBegin { record, created })
    }

    pub async fn load(
        &self,
        event_id: &MatrixEventId,
    ) -> Result<Option<MatrixOperationRecord>, MatrixDurableError> {
        let pool = self.open_pool().await?;
        let record = load_record(&pool, &self.owner_agent_id, event_id).await?;
        pool.close().await;
        Ok(record)
    }

    pub async fn claim_delivery(
        &self,
        event_id: &MatrixEventId,
        envelope: &OutboxEnvelope,
        at_ms: u64,
    ) -> Result<MatrixOperationRecord, MatrixDurableError> {
        let current = self.require_exact(event_id, envelope).await?;
        match current.phase {
            OperationPhase::OutboxPending => {
                if !current.phase.can_transition_to(OperationPhase::DeliveryClaimed) {
                    return Err(MatrixDurableError::Corrupt);
                }
                self.transition(
                    event_id,
                    envelope,
                    OperationPhase::OutboxPending,
                    OperationPhase::DeliveryClaimed,
                    None,
                    at_ms,
                )
                .await
            }
            OperationPhase::DeliveryClaimed
            | OperationPhase::Indeterminate
            | OperationPhase::Acknowledged
            | OperationPhase::ReconciledApplied
            | OperationPhase::ReconciledNotApplied
            | OperationPhase::Quarantined => Ok(current),
            _ => Err(MatrixDurableError::Corrupt),
        }
    }

    pub async fn mark_indeterminate(
        &self,
        event_id: &MatrixEventId,
        envelope: &OutboxEnvelope,
        at_ms: u64,
    ) -> Result<MatrixOperationRecord, MatrixDurableError> {
        let current = self.require_exact(event_id, envelope).await?;
        match current.phase {
            OperationPhase::DeliveryClaimed => {
                if !current.phase.can_transition_to(OperationPhase::Indeterminate) {
                    return Err(MatrixDurableError::Corrupt);
                }
                self.transition(
                    event_id,
                    envelope,
                    OperationPhase::DeliveryClaimed,
                    OperationPhase::Indeterminate,
                    None,
                    at_ms,
                )
                .await
            }
            OperationPhase::Indeterminate
            | OperationPhase::Acknowledged
            | OperationPhase::ReconciledApplied
            | OperationPhase::ReconciledNotApplied
            | OperationPhase::Quarantined => Ok(current),
            OperationPhase::OutboxPending => Err(MatrixDurableError::Conflict),
            _ => Err(MatrixDurableError::Corrupt),
        }
    }

    pub async fn acknowledge(
        &self,
        event_id: &MatrixEventId,
        envelope: &OutboxEnvelope,
        acknowledgement: &DestinationAcknowledgement,
        at_ms: u64,
    ) -> Result<MatrixOperationRecord, MatrixDurableError> {
        acknowledgement
            .validate_against(envelope)
            .map_err(|_| MatrixDurableError::Conflict)?;
        let current = self.require_exact(event_id, envelope).await?;
        match current.phase {
            OperationPhase::DeliveryClaimed => {
                if !OperationPhase::DeliveryClaimed
                    .can_transition_to(OperationPhase::DestinationCommitted)
                    || !OperationPhase::DestinationCommitted
                        .can_transition_to(OperationPhase::Acknowledged)
                {
                    return Err(MatrixDurableError::Corrupt);
                }
                self.transition(
                    event_id,
                    envelope,
                    OperationPhase::DeliveryClaimed,
                    OperationPhase::Acknowledged,
                    Some(&acknowledgement.destination_receipt_sha256),
                    at_ms,
                )
                .await
            }
            OperationPhase::Acknowledged
                if current.destination_receipt_sha256.as_ref()
                    == Some(&acknowledgement.destination_receipt_sha256) =>
            {
                Ok(current)
            }
            OperationPhase::Indeterminate => Err(MatrixDurableError::Conflict),
            _ => Err(MatrixDurableError::Conflict),
        }
    }

    pub async fn reconcile_applied(
        &self,
        event_id: &MatrixEventId,
        envelope: &OutboxEnvelope,
        destination_receipt_sha256: &Sha256Digest,
        at_ms: u64,
    ) -> Result<MatrixOperationRecord, MatrixDurableError> {
        let current = self.require_exact(event_id, envelope).await?;
        match current.phase {
            OperationPhase::Indeterminate => self
                .transition(
                    event_id,
                    envelope,
                    OperationPhase::Indeterminate,
                    OperationPhase::ReconciledApplied,
                    Some(destination_receipt_sha256),
                    at_ms,
                )
                .await,
            OperationPhase::ReconciledApplied
                if current.destination_receipt_sha256.as_ref()
                    == Some(destination_receipt_sha256) =>
            {
                Ok(current)
            }
            OperationPhase::Acknowledged
                if current.destination_receipt_sha256.as_ref()
                    == Some(destination_receipt_sha256) =>
            {
                Ok(current)
            }
            _ => Err(MatrixDurableError::Conflict),
        }
    }

    pub async fn reconcile_not_applied(
        &self,
        event_id: &MatrixEventId,
        envelope: &OutboxEnvelope,
        at_ms: u64,
    ) -> Result<MatrixOperationRecord, MatrixDurableError> {
        let current = self.require_exact(event_id, envelope).await?;
        match current.phase {
            OperationPhase::Indeterminate => self
                .transition(
                    event_id,
                    envelope,
                    OperationPhase::Indeterminate,
                    OperationPhase::ReconciledNotApplied,
                    None,
                    at_ms,
                )
                .await,
            OperationPhase::ReconciledNotApplied => Ok(current),
            _ => Err(MatrixDurableError::Conflict),
        }
    }

    async fn require_exact(
        &self,
        event_id: &MatrixEventId,
        envelope: &OutboxEnvelope,
    ) -> Result<MatrixOperationRecord, MatrixDurableError> {
        envelope
            .validate()
            .map_err(|_| MatrixDurableError::Invalid)?;
        let current = self
            .load(event_id)
            .await?
            .ok_or(MatrixDurableError::Conflict)?;
        if &current.envelope != envelope {
            return Err(MatrixDurableError::Conflict);
        }
        Ok(current)
    }

    #[allow(clippy::too_many_arguments)]
    async fn transition(
        &self,
        event_id: &MatrixEventId,
        envelope: &OutboxEnvelope,
        expected: OperationPhase,
        next: OperationPhase,
        destination_receipt_sha256: Option<&Sha256Digest>,
        at_ms: u64,
    ) -> Result<MatrixOperationRecord, MatrixDurableError> {
        let pool = self.open_pool().await?;
        let current = load_record(&pool, &self.owner_agent_id, event_id)
            .await?
            .ok_or(MatrixDurableError::Conflict)?;
        if current.envelope != *envelope || current.phase != expected || at_ms < current.updated_at_ms {
            pool.close().await;
            return Err(MatrixDurableError::Conflict);
        }
        let updated = sqlx::query(
            "UPDATE matrix_operations
             SET phase = ?, destination_receipt_sha256 = ?, updated_at_ms = ?
             WHERE event_id = ? AND phase = ? AND binding_sha256 = ? AND sequence = ?",
        )
        .bind(next.as_str())
        .bind(destination_receipt_sha256.map(Sha256Digest::as_str))
        .bind(to_i64(at_ms)?)
        .bind(event_id.as_str())
        .bind(expected.as_str())
        .bind(envelope.binding_sha256.as_str())
        .bind(to_i64(envelope.sequence)?)
        .execute(&pool)
        .await
        .map_err(|_| MatrixDurableError::Unavailable)?;
        if updated.rows_affected() != 1 {
            pool.close().await;
            return Err(MatrixDurableError::Conflict);
        }
        let record = load_record(&pool, &self.owner_agent_id, event_id)
            .await?
            .ok_or(MatrixDurableError::Corrupt)?;
        pool.close().await;
        Ok(record)
    }

    async fn open_pool(&self) -> Result<SqlitePool, MatrixDurableError> {
        let root = self
            .database_path
            .parent()
            .ok_or(MatrixDurableError::Invalid)?
            .to_path_buf();
        let sqlite_home =
            AbsolutePathBuf::try_from(root).map_err(|_| MatrixDurableError::Invalid)?;
        SqliteConfig::from_sqlite_home(sqlite_home)
            .open_durable_evidence_pool(&self.database_path)
            .await
            .map_err(|_| MatrixDurableError::Unavailable)
    }
}

async fn load_record(
    pool: &SqlitePool,
    owner_agent_id: &codex_hepta_contracts::AgentId,
    event_id: &MatrixEventId,
) -> Result<Option<MatrixOperationRecord>, MatrixDurableError> {
    let row = sqlx::query(
        "SELECT event_id, operation_id, idempotency_key, binding_sha256,
                authority_epoch, owner_epoch, generation, fencing_token_sha256,
                command_sha256, command_bytes, sequence, phase,
                destination_receipt_sha256, updated_at_ms
         FROM matrix_operations WHERE event_id = ?",
    )
    .bind(event_id.as_str())
    .fetch_optional(pool)
    .await
    .map_err(|_| MatrixDurableError::Unavailable)?;
    row.map(|row| record_from_row(owner_agent_id, &row))
        .transpose()
}

fn record_from_row(
    owner_agent_id: &codex_hepta_contracts::AgentId,
    row: &sqlx::sqlite::SqliteRow,
) -> Result<MatrixOperationRecord, MatrixDurableError> {
    let event_id = MatrixEventId::parse(
        row.try_get::<String, _>("event_id")
            .map_err(|_| MatrixDurableError::Corrupt)?,
    )
    .map_err(|_| MatrixDurableError::Corrupt)?;
    let binding_sha256 = parse_digest(row, "binding_sha256")?;
    let binding = OperationBinding::new(
        OperationId::parse(
            row.try_get::<String, _>("operation_id")
                .map_err(|_| MatrixDurableError::Corrupt)?,
        )
        .map_err(|_| MatrixDurableError::Corrupt)?,
        IdempotencyKey::parse(
            row.try_get::<String, _>("idempotency_key")
                .map_err(|_| MatrixDurableError::Corrupt)?,
        )
        .map_err(|_| MatrixDurableError::Corrupt)?,
        owner_agent_id.clone(),
        ProductComponentId::MatrixIngress,
        owner_agent_id.clone(),
        ProductComponentId::AppServer,
        AuthorityAction::ServeSession,
        row_u64(row, "authority_epoch")?,
        row_u64(row, "owner_epoch")?,
        row_u64(row, "generation")?,
        parse_digest(row, "fencing_token_sha256")?,
        parse_digest(row, "command_sha256")?,
        row_u64(row, "command_bytes")?,
    )
    .map_err(|_| MatrixDurableError::Corrupt)?;
    if binding.digest() != binding_sha256 {
        return Err(MatrixDurableError::Corrupt);
    }
    let envelope = OutboxEnvelope {
        binding,
        binding_sha256,
        sequence: row_u64(row, "sequence")?,
        phase: OperationPhase::OutboxPending,
    };
    envelope
        .validate()
        .map_err(|_| MatrixDurableError::Corrupt)?;
    let phase = parse_phase(
        &row.try_get::<String, _>("phase")
            .map_err(|_| MatrixDurableError::Corrupt)?,
    )?;
    let destination_receipt_sha256 = row
        .try_get::<Option<String>, _>("destination_receipt_sha256")
        .map_err(|_| MatrixDurableError::Corrupt)?
        .map(Sha256Digest::parse)
        .transpose()
        .map_err(|_| MatrixDurableError::Corrupt)?;
    if matches!(phase, OperationPhase::Acknowledged | OperationPhase::ReconciledApplied)
        != destination_receipt_sha256.is_some()
    {
        return Err(MatrixDurableError::Corrupt);
    }
    Ok(MatrixOperationRecord {
        event_id,
        envelope,
        phase,
        destination_receipt_sha256,
        updated_at_ms: row_u64(row, "updated_at_ms")?,
    })
}

fn operation_id(
    owner_agent_id: &codex_hepta_contracts::AgentId,
    event_id: &MatrixEventId,
) -> Result<OperationId, MatrixDurableError> {
    let mut bytes = Vec::new();
    frame(&mut bytes, b"hepta:matrix-operation-id:v1");
    frame(&mut bytes, owner_agent_id.as_str().as_bytes());
    frame(&mut bytes, event_id.as_str().as_bytes());
    OperationId::parse(format!(
        "matrix:admission:v1:{}",
        Sha256Digest::for_bytes(&bytes).as_str()
    ))
    .map_err(|_| MatrixDurableError::Invalid)
}

fn command_digest(inbox: &InboxRecord, idempotency_key: &str) -> Sha256Digest {
    let mut bytes = Vec::new();
    frame(&mut bytes, b"hepta:matrix-admission-command:v1");
    frame(&mut bytes, inbox.event_id.as_str().as_bytes());
    frame(&mut bytes, inbox.room_id.as_str().as_bytes());
    frame(&mut bytes, idempotency_key.as_bytes());
    frame(&mut bytes, &inbox.payload);
    Sha256Digest::for_bytes(&bytes)
}

fn fencing_digest(
    owner_agent_id: &codex_hepta_contracts::AgentId,
    inbox: &InboxRecord,
    project_id: &str,
) -> Sha256Digest {
    let mut bytes = Vec::new();
    frame(&mut bytes, b"hepta:matrix-operation-fence:v1");
    frame(&mut bytes, owner_agent_id.as_str().as_bytes());
    frame(&mut bytes, inbox.room_id.as_str().as_bytes());
    frame(&mut bytes, project_id.as_bytes());
    frame(&mut bytes, &inbox.binding_revision.to_be_bytes());
    frame(&mut bytes, &inbox.generation.to_be_bytes());
    Sha256Digest::for_bytes(&bytes)
}

fn parse_phase(value: &str) -> Result<OperationPhase, MatrixDurableError> {
    match value {
        "outbox_pending" => Ok(OperationPhase::OutboxPending),
        "delivery_claimed" => Ok(OperationPhase::DeliveryClaimed),
        "acknowledged" => Ok(OperationPhase::Acknowledged),
        "indeterminate" => Ok(OperationPhase::Indeterminate),
        "reconciled_applied" => Ok(OperationPhase::ReconciledApplied),
        "reconciled_not_applied" => Ok(OperationPhase::ReconciledNotApplied),
        "quarantined" => Ok(OperationPhase::Quarantined),
        _ => Err(MatrixDurableError::Corrupt),
    }
}

fn parse_digest(
    row: &sqlx::sqlite::SqliteRow,
    column: &str,
) -> Result<Sha256Digest, MatrixDurableError> {
    Sha256Digest::parse(
        row.try_get::<String, _>(column)
            .map_err(|_| MatrixDurableError::Corrupt)?,
    )
    .map_err(|_| MatrixDurableError::Corrupt)
}

fn row_u64(row: &sqlx::sqlite::SqliteRow, column: &str) -> Result<u64, MatrixDurableError> {
    let value = row
        .try_get::<i64, _>(column)
        .map_err(|_| MatrixDurableError::Corrupt)?;
    u64::try_from(value).map_err(|_| MatrixDurableError::Corrupt)
}

fn to_i64(value: u64) -> Result<i64, MatrixDurableError> {
    i64::try_from(value).map_err(|_| MatrixDurableError::Invalid)
}

fn frame(target: &mut Vec<u8>, part: &[u8]) {
    target.extend_from_slice(&(part.len() as u64).to_be_bytes());
    target.extend_from_slice(part);
}

#[cfg(test)]
mod tests {
    use codex_hepta_contracts::DestinationAcknowledgement;
    use codex_hepta_contracts::OperationPhase;
    use codex_hepta_contracts::RecoveryDecision;
    use codex_hepta_contracts::Sha256Digest;
    use codex_hepta_matrix_protocol::MatrixRoomId;
    use codex_hepta_matrix_protocol::MatrixUserId;
    use codex_hepta_paths::HeptaFleetRoot;

    use super::*;
    use crate::InboxDisposition;
    use crate::InboxDraft;
    use crate::MatrixDurableConfig;
    use crate::RoomBindingDraft;

    const AGENT_ID: &str = "018f4f72-5f8f-7cc1-8f55-df9fb3aa2c12";

    #[tokio::test]
    async fn matrix_operation_survives_reopen_and_changed_replay_fails_closed() {
        let temp = tempfile::tempdir().expect("tempdir");
        let root = temp.path().join("fleet");
        std::fs::create_dir_all(&root).expect("fleet root");
        let fleet = HeptaFleetRoot::parse(root.canonicalize().expect("canonical root"))
            .expect("fleet root");
        let agent_id = codex_hepta_contracts::AgentId::parse(AGENT_ID).expect("agent id");
        let layout = fleet.layout().agent(&agent_id);
        let store = MatrixDurableStore::open(&layout, MatrixDurableConfig::default())
            .await
            .expect("store");
        let room_id = MatrixRoomId::parse("!operation:example.test").expect("room");
        store
            .bind_room(&RoomBindingDraft {
                room_id: room_id.clone(),
                agent_user_id: MatrixUserId::parse("@agent:example.test").expect("mxid"),
                expected_revision: None,
                generation: 1,
                changed_at_ms: 1,
            })
            .await
            .expect("bind room");
        let event_id = MatrixEventId::parse("$operation-event").expect("event");
        let draft = InboxDraft {
            event_id: event_id.clone(),
            room_id,
            sender: MatrixUserId::parse("@owner:example.test").expect("sender"),
            event_type: "m.room.message".to_string(),
            payload: br#"{"msgtype":"m.text","body":"hello"}"#.to_vec(),
            binding_revision: 1,
            generation: 1,
            origin_server_ts_ms: 2,
            received_at_ms: 3,
        };
        assert!(matches!(
            store.ingest_inbox(&draft).await.expect("ingest"),
            InboxDisposition::Accepted(_)
        ));
        let inbox = store
            .inbox(&event_id)
            .await
            .expect("load inbox")
            .expect("inbox");
        let project_id = codex_hepta_matrix_protocol::room_project_idempotency_key(
            &agent_id,
            &inbox.room_id,
        );
        let journal = MatrixOperationJournal::new(&store);
        let begun = journal.begin(&inbox, &project_id, 4).await.expect("begin");
        assert!(begun.created);
        assert_eq!(begun.record.phase, OperationPhase::OutboxPending);
        let replay = journal.begin(&inbox, &project_id, 4).await.expect("replay");
        assert!(!replay.created);
        assert_eq!(replay.record.envelope, begun.record.envelope);

        let mut changed = inbox.clone();
        changed.payload = br#"{"msgtype":"m.text","body":"changed"}"#.to_vec();
        assert_eq!(
            journal.begin(&changed, &project_id, 5).await,
            Err(MatrixDurableError::Conflict)
        );

        let claimed = journal
            .claim_delivery(&event_id, &begun.record.envelope, 5)
            .await
            .expect("claim");
        assert_eq!(claimed.phase, OperationPhase::DeliveryClaimed);
        let indeterminate = journal
            .mark_indeterminate(&event_id, &begun.record.envelope, 6)
            .await
            .expect("indeterminate");
        assert_eq!(indeterminate.recovery_decision(), RecoveryDecision::LookupOnly);
        store.close().await;

        let reopened = MatrixDurableStore::open(&layout, MatrixDurableConfig::default())
            .await
            .expect("reopen");
        let reopened_journal = MatrixOperationJournal::new(&reopened);
        let persisted = reopened_journal
            .load(&event_id)
            .await
            .expect("load operation")
            .expect("operation");
        assert_eq!(persisted.phase, OperationPhase::Indeterminate);
        let destination = Sha256Digest::for_bytes(b"durable App Server receipt");
        let reconciled = reopened_journal
            .reconcile_applied(&event_id, &persisted.envelope, &destination, 7)
            .await
            .expect("reconcile");
        assert_eq!(reconciled.phase, OperationPhase::ReconciledApplied);
        let acknowledgement = DestinationAcknowledgement::committed(
            &persisted.envelope,
            destination.clone(),
        )
        .expect("ack shape");
        acknowledgement
            .validate_against(&persisted.envelope)
            .expect("exact acknowledgement");
        assert_eq!(reconciled.destination_receipt_sha256, Some(destination));
    }
}
