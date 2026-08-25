//! Agent-local, qualification-only H7 trajectory evidence.
//!
//! This module is the narrow durable bridge for the first H7a slice.  It
//! records observation/terminal lifecycle events against the exact local
//! lease and compact fence already owned by a host.  The table is an
//! immutable hash chain; no provider call, model-weight mutation, KG write,
//! policy action, or production authority is exposed here.

use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use codex_hepta_contracts::Sha256Digest;
use serde::Deserialize;
use serde::Serialize;
use sha2::Digest;
use sha2::Sha256;
use sqlx::Row;
use sqlx::Sqlite;
use sqlx::Transaction;
use thiserror::Error;

use crate::CognitiveStore;
use crate::CognitiveStoreError;
use crate::LocalCompactExecutor;
use crate::LocalLeaseOutbox;
use crate::LocalTurnLifecycleBinding;
use crate::LocalTurnLifecycleBindingError;
use crate::framing::frame_part;

pub const H7_TRAJECTORY_SCHEMA_VERSION: u32 = 1;
pub const H7_TRAJECTORY_NAMESPACE: &str = "local_qualification_only";
pub const H7_TRAJECTORY_EXTERNAL_EFFECTS: bool = false;
pub const H7_TRAJECTORY_KG_WRITE_AUTHORITY: bool = false;
pub const H7_TRAJECTORY_PRODUCTION_CALLER: bool = false;

const MAX_TRAJECTORY_ROWS: usize = 16_384;
const MAX_TEXT_BYTES: usize = 512;
const MAX_OUTCOME_BYTES: usize = 2_048;
const MAX_REASON_BYTES: usize = 512;
const MAX_METADATA_BYTES: usize = 65_536;
const MAX_PAYLOAD_BYTES: usize = 262_144;
const EVENT_DOMAIN: &[u8] = b"hepta-memory:h7-trajectory-event:v1";
const FENCING_TOKEN_DOMAIN: &[u8] = b"hepta-memory:local-turn-lifecycle-binding:v1";
const GENESIS: &[u8] = b"hepta-memory:h7-trajectory:genesis:v1";

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum H7TrajectoryEventKind {
    TurnStart,
    Feedback,
    Terminal,
}

impl H7TrajectoryEventKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TurnStart => "turn_start",
            Self::Feedback => "feedback",
            Self::Terminal => "terminal",
        }
    }

    fn parse(value: &str) -> Result<Self, H7TrajectoryStoreError> {
        match value {
            "turn_start" => Ok(Self::TurnStart),
            "feedback" => Ok(Self::Feedback),
            "terminal" => Ok(Self::Terminal),
            other => Err(corrupt(format!(
                "unknown H7 trajectory event kind {other:?}"
            ))),
        }
    }

    const fn is_terminal(self) -> bool {
        matches!(self, Self::Terminal)
    }
}

/// A caller-owned observation record.  Lease/fence fields are deliberately
/// absent: the bound append function derives them from the exact handles and
/// rejects guessed or cross-store authority.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct H7TrajectoryRecord {
    pub trajectory_id: String,
    pub event_seq: u32,
    pub event_id: String,
    pub event_kind: H7TrajectoryEventKind,
    pub turn_id: String,
    pub occurrence_key: String,
    pub causal_parent_seq: Option<u32>,
    pub causal_parent_sha256: Option<Sha256Digest>,
    pub state_digest: Sha256Digest,
    pub policy_digest: Sha256Digest,
    pub model_receipt_digest: Sha256Digest,
    pub receipt_sha256: Sha256Digest,
    pub outcome: String,
    pub reward_bps: i32,
    pub safety_ok: bool,
    pub terminal: bool,
    pub propensity_json: Option<String>,
    pub support_json: Option<String>,
    pub metadata_json: String,
    pub reason: String,
    pub external_effect_executed: bool,
    pub kg_write_authority: bool,
    pub production_caller: bool,
}

impl H7TrajectoryRecord {
    /// Build a lifecycle observation.  Current qualification events are not
    /// policy actions, so propensity/support are explicitly absent and the
    /// reason is recorded as `not_applicable`.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        trajectory_id: impl Into<String>,
        event_seq: u32,
        event_id: impl Into<String>,
        event_kind: H7TrajectoryEventKind,
        turn_id: impl Into<String>,
        occurrence_key: impl Into<String>,
        causal_parent_seq: Option<u32>,
        causal_parent_sha256: Option<Sha256Digest>,
        state_digest: Sha256Digest,
        policy_digest: Sha256Digest,
        model_receipt_digest: Sha256Digest,
        receipt_sha256: Sha256Digest,
        outcome: impl Into<String>,
        reward_bps: i32,
        safety_ok: bool,
        metadata_json: impl Into<String>,
        reason: impl Into<String>,
    ) -> Result<Self, H7TrajectoryStoreError> {
        let record = Self {
            trajectory_id: trajectory_id.into(),
            event_seq,
            event_id: event_id.into(),
            event_kind,
            turn_id: turn_id.into(),
            occurrence_key: occurrence_key.into(),
            causal_parent_seq,
            causal_parent_sha256,
            state_digest,
            policy_digest,
            model_receipt_digest,
            receipt_sha256,
            outcome: outcome.into(),
            reward_bps,
            safety_ok,
            terminal: event_kind.is_terminal(),
            propensity_json: None,
            support_json: None,
            metadata_json: metadata_json.into(),
            reason: reason.into(),
            external_effect_executed: false,
            kg_write_authority: false,
            production_caller: false,
        };
        record.validate().map(|()| record)
    }

    pub fn turn_start(
        trajectory_id: impl Into<String>,
        event_id: impl Into<String>,
        turn_id: impl Into<String>,
        occurrence_key: impl Into<String>,
        state_digest: Sha256Digest,
        policy_digest: Sha256Digest,
        model_receipt_digest: Sha256Digest,
        receipt_sha256: Sha256Digest,
        metadata_json: impl Into<String>,
    ) -> Result<Self, H7TrajectoryStoreError> {
        Self::new(
            trajectory_id,
            1,
            event_id,
            H7TrajectoryEventKind::TurnStart,
            turn_id,
            occurrence_key,
            None,
            None,
            state_digest,
            policy_digest,
            model_receipt_digest,
            receipt_sha256,
            "turn_started",
            0,
            true,
            metadata_json,
            "not_applicable",
        )
    }

    pub fn terminal(
        trajectory_id: impl Into<String>,
        event_seq: u32,
        event_id: impl Into<String>,
        turn_id: impl Into<String>,
        occurrence_key: impl Into<String>,
        causal_parent_seq: u32,
        causal_parent_sha256: Sha256Digest,
        state_digest: Sha256Digest,
        policy_digest: Sha256Digest,
        model_receipt_digest: Sha256Digest,
        receipt_sha256: Sha256Digest,
        outcome: impl Into<String>,
        reason: impl Into<String>,
        metadata_json: impl Into<String>,
    ) -> Result<Self, H7TrajectoryStoreError> {
        Self::new(
            trajectory_id,
            event_seq,
            event_id,
            H7TrajectoryEventKind::Terminal,
            turn_id,
            occurrence_key,
            Some(causal_parent_seq),
            Some(causal_parent_sha256),
            state_digest,
            policy_digest,
            model_receipt_digest,
            receipt_sha256,
            outcome,
            0,
            true,
            metadata_json,
            reason,
        )
    }

    pub fn validate(&self) -> Result<(), H7TrajectoryStoreError> {
        validate_text(&self.trajectory_id, "trajectory id", MAX_TEXT_BYTES)?;
        validate_text(&self.event_id, "event id", MAX_TEXT_BYTES)?;
        validate_text(&self.turn_id, "turn id", MAX_TEXT_BYTES)?;
        validate_text(&self.occurrence_key, "occurrence key", MAX_TEXT_BYTES)?;
        validate_text(&self.outcome, "outcome", MAX_OUTCOME_BYTES)?;
        validate_text(&self.reason, "reason", MAX_REASON_BYTES)?;
        if self.event_seq == 0 {
            return Err(invalid("trajectory event sequence must be non-zero"));
        }
        if self.event_seq == 1 && self.event_kind != H7TrajectoryEventKind::TurnStart {
            return Err(invalid("first trajectory event must be turn_start"));
        }
        if self.event_seq > 1 && self.event_kind == H7TrajectoryEventKind::TurnStart {
            return Err(invalid("turn_start is only valid as the first event"));
        }
        if self.event_kind == H7TrajectoryEventKind::Feedback {
            return Err(H7TrajectoryStoreError::PolicyActionNotQualified);
        }
        if self.terminal != self.event_kind.is_terminal() {
            return Err(invalid("terminal flag does not match event kind"));
        }
        if self.external_effect_executed || self.kg_write_authority || self.production_caller {
            return Err(H7TrajectoryStoreError::BoundaryViolation);
        }
        if self.propensity_json.is_some() || self.support_json.is_some() {
            return Err(H7TrajectoryStoreError::PolicyActionNotQualified);
        }
        if self.reward_bps != 0 {
            return Err(H7TrajectoryStoreError::PolicyActionNotQualified);
        }
        if self.reason.trim().is_empty() {
            return Err(invalid("trajectory reason must be non-empty"));
        }
        validate_metadata(&self.metadata_json)?;
        validate_digest(&self.state_digest, "state digest")?;
        validate_digest(&self.policy_digest, "policy digest")?;
        validate_digest(&self.model_receipt_digest, "model receipt digest")?;
        validate_digest(&self.receipt_sha256, "receipt digest")?;
        if let Some(parent) = &self.causal_parent_sha256 {
            validate_digest(parent, "causal parent digest")?;
        }
        if self.event_seq == 1
            && (self.causal_parent_seq.is_some() || self.causal_parent_sha256.is_some())
        {
            return Err(invalid(
                "first trajectory event cannot have a causal parent",
            ));
        }
        if self.event_seq > 1
            && (self.causal_parent_seq != Some(self.event_seq - 1)
                || self.causal_parent_sha256.is_none())
        {
            return Err(invalid(
                "non-first trajectory event must point to the immediately prior event",
            ));
        }
        Ok(())
    }

    fn payload_json(&self) -> Result<String, H7TrajectoryStoreError> {
        serde_json::to_string(self)
            .map_err(|error| H7TrajectoryStoreError::Serialization(error.to_string()))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum H7TrajectoryAppend {
    Inserted {
        event_seq: u32,
        event_sha256: Sha256Digest,
    },
    Replay {
        event_seq: u32,
        event_sha256: Sha256Digest,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct H7TrajectoryRead {
    pub trajectory_id: String,
    pub events: Vec<H7TrajectoryRecord>,
    pub head_sha256: Sha256Digest,
}

#[derive(Debug, Error)]
pub enum H7TrajectoryStoreError {
    #[error(transparent)]
    Store(#[from] CognitiveStoreError),
    #[error(transparent)]
    Binding(#[from] LocalTurnLifecycleBindingError),
    #[error("invalid H7 trajectory record: {0}")]
    Invalid(String),
    #[error("H7 trajectory store is corrupt: {0}")]
    Corrupt(String),
    #[error("H7 trajectory CAS conflict: {0}")]
    CasConflict(String),
    #[error("H7 trajectory lease/fence is stale: {0}")]
    StaleFence(String),
    #[error("H7 trajectory contains an unqualified policy action")]
    PolicyActionNotQualified,
    #[error("H7 trajectory crosses the local qualification boundary")]
    BoundaryViolation,
    #[error("H7 trajectory serialization failed: {0}")]
    Serialization(String),
    #[error("H7 trajectory clock failed: {0}")]
    Clock(String),
    #[error("H7 trajectory local lease failed: {0}")]
    Lease(String),
}

/// Append one H7 trajectory event while holding the exact local lease/fence
/// transaction.  This is the only mutating API exposed to extension hosts.
/// Every event in one immutable trajectory must retain the same binding tuple;
/// a successor generation must use a new trajectory identity.
pub async fn append_h7_trajectory_event_bound(
    lease: &LocalLeaseOutbox,
    executor: &LocalCompactExecutor,
    binding: &LocalTurnLifecycleBinding,
    record: &H7TrajectoryRecord,
) -> Result<H7TrajectoryAppend, H7TrajectoryStoreError> {
    record.validate()?;
    binding.validate()?;
    if !executor.is_bound_to_lease(lease) || !lease.is_bound_to_store(executor.store()) {
        return Err(H7TrajectoryStoreError::StaleFence(
            "trajectory handles are not bound to one local store/lease".to_string(),
        ));
    }
    if binding.turn_id != record.turn_id {
        return Err(H7TrajectoryStoreError::CasConflict(
            "trajectory record turn does not match lifecycle binding".to_string(),
        ));
    }
    binding.verify_current(lease, executor).await?;
    let store = executor.store();
    let mut transaction = store
        .pool
        .begin_with("BEGIN IMMEDIATE")
        .await
        .map_err(crate::cognitive_store::unavailable)?;
    let current = lease
        .verify_current_in_transaction(&mut transaction)
        .await
        .map_err(|error| H7TrajectoryStoreError::Lease(error.to_string()))?;
    verify_current_binding(binding, lease, executor, &current)?;
    let existing = load_trajectory_rows(&mut transaction, store, &record.trajectory_id).await?;
    let binding_tuple = binding_tuple(binding);
    if existing
        .iter()
        .any(|row| &row.binding_tuple() != &binding_tuple)
    {
        return Err(corrupt(
            "trajectory binding does not match the current lifecycle binding",
        ));
    }
    let payload_json = record.payload_json()?;
    if payload_json.len() > MAX_PAYLOAD_BYTES || payload_json.as_bytes().contains(&0) {
        return Err(invalid("trajectory payload exceeds the bounded size"));
    }
    let payload_sha256 = Sha256Digest::for_bytes(payload_json.as_bytes());
    if let Some(previous) = existing.last() {
        if previous.record.event_seq == record.event_seq {
            let expected = event_digest(
                store.owner_agent_id().as_str(),
                &record.trajectory_id,
                record,
                &payload_sha256,
                &previous.previous_sha256,
                &binding_tuple,
            );
            if previous.record == *record
                && previous.payload_sha256 == payload_sha256
                && previous.event_sha256 == expected
                && previous.lease_id == binding.lease_id
                && previous.lease_head_sha256 == binding.lease_head_sha256
            {
                transaction
                    .commit()
                    .await
                    .map_err(crate::cognitive_store::unavailable)?;
                return Ok(H7TrajectoryAppend::Replay {
                    event_seq: record.event_seq,
                    event_sha256: previous.event_sha256.clone(),
                });
            }
            return Err(H7TrajectoryStoreError::CasConflict(
                "trajectory event replay changed its payload or binding".to_string(),
            ));
        }
    }
    let expected_seq = existing
        .last()
        .map_or(1, |row| row.record.event_seq.saturating_add(1));
    if record.event_seq != expected_seq {
        return Err(H7TrajectoryStoreError::CasConflict(format!(
            "trajectory event sequence is not contiguous (expected {expected_seq}, got {})",
            record.event_seq
        )));
    }
    if let Some(previous) = existing.last() {
        if previous.record.terminal {
            return Err(H7TrajectoryStoreError::CasConflict(
                "trajectory is already terminal".to_string(),
            ));
        }
        if record.causal_parent_sha256.as_ref() != Some(&previous.event_sha256)
            || record.causal_parent_seq != Some(previous.record.event_seq)
            || previous.record.turn_id != record.turn_id
        {
            return Err(H7TrajectoryStoreError::CasConflict(
                "trajectory causal parent does not match the durable head".to_string(),
            ));
        }
    }
    let previous_sha256 = existing
        .last()
        .map(|row| row.event_sha256.clone())
        .unwrap_or_else(genesis_digest);
    let event_sha256 = event_digest(
        store.owner_agent_id().as_str(),
        &record.trajectory_id,
        record,
        &payload_sha256,
        &previous_sha256,
        &binding_tuple,
    );
    let recorded_at = now_unix_seconds()?;
    insert_row(
        &mut transaction,
        store,
        record,
        &binding_tuple,
        &payload_json,
        &payload_sha256,
        &previous_sha256,
        &event_sha256,
        recorded_at,
    )
    .await?;
    transaction
        .commit()
        .await
        .map_err(crate::cognitive_store::unavailable)?;
    Ok(H7TrajectoryAppend::Inserted {
        event_seq: record.event_seq,
        event_sha256,
    })
}

/// Digest the exact local event/outbox admission receipt that a qualification
/// host is about to reference from a trajectory row.  This is provenance
/// metadata only; it is never an external-effect receipt.
pub fn h7_trajectory_local_receipt_digest(receipt: &crate::QueuedReceipt) -> Sha256Digest {
    let mut hasher = Sha256::new();
    frame_part(&mut hasher, b"hepta-memory:h7-trajectory:local-receipt:v1");
    for part in [
        receipt.lease_id.as_bytes(),
        receipt.occurrence_key.as_bytes(),
        receipt.event_id.as_bytes(),
        receipt.outbox_id.as_bytes(),
        receipt.owner_agent_id.as_str().as_bytes(),
        receipt.payload_sha256.as_str().as_bytes(),
        receipt.fencing_token.as_bytes(),
    ] {
        frame_part(&mut hasher, part);
    }
    frame_part(&mut hasher, &receipt.generation.to_be_bytes());
    Sha256Digest::from_sha256_output(hasher.finalize())
}

impl CognitiveStore {
    /// Reopen and verify one complete Agent-local trajectory hash chain.
    pub async fn read_h7_trajectory(
        &self,
        trajectory_id: impl Into<String>,
    ) -> Result<Option<H7TrajectoryRead>, H7TrajectoryStoreError> {
        let trajectory_id = trajectory_id.into();
        validate_text(&trajectory_id, "trajectory id", MAX_TEXT_BYTES)?;
        let mut transaction = self
            .pool
            .begin()
            .await
            .map_err(crate::cognitive_store::unavailable)?;
        let result =
            read_h7_trajectory_in_transaction(&mut transaction, self, &trajectory_id, None).await?;
        transaction
            .commit()
            .await
            .map_err(crate::cognitive_store::unavailable)?;
        Ok(result)
    }
}

/// Reopen and verify one trajectory while retaining the exact local
/// lease/compact binding used by its writer.  This is a read-only recovery
/// helper: it grants no new authority and never appends or releases anything.
/// A caller can use it before replaying a queued occurrence so an already
/// terminal H7 observation cannot be mistaken for a fresh turn start.
pub async fn read_h7_trajectory_bound(
    lease: &LocalLeaseOutbox,
    executor: &LocalCompactExecutor,
    binding: &LocalTurnLifecycleBinding,
    trajectory_id: impl Into<String>,
) -> Result<Option<H7TrajectoryRead>, H7TrajectoryStoreError> {
    binding.validate()?;
    if !executor.is_bound_to_lease(lease) || !lease.is_bound_to_store(executor.store()) {
        return Err(H7TrajectoryStoreError::StaleFence(
            "trajectory handles are not bound to one local store/lease".to_string(),
        ));
    }
    let trajectory_id = trajectory_id.into();
    validate_text(&trajectory_id, "trajectory id", MAX_TEXT_BYTES)?;
    let store = executor.store();
    let mut transaction = store
        .pool
        .begin_with("BEGIN IMMEDIATE")
        .await
        .map_err(crate::cognitive_store::unavailable)?;
    let current = lease
        .verify_current_in_transaction(&mut transaction)
        .await
        .map_err(|error| H7TrajectoryStoreError::Lease(error.to_string()))?;
    verify_current_binding(binding, lease, executor, &current)?;
    let expected_binding = binding_tuple(binding);
    let result = read_h7_trajectory_in_transaction(
        &mut transaction,
        store,
        &trajectory_id,
        Some(&expected_binding),
    )
    .await?;
    transaction
        .commit()
        .await
        .map_err(crate::cognitive_store::unavailable)?;
    Ok(result)
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct BindingTuple {
    lease_id: String,
    lease_head_sha256: Sha256Digest,
    authority_epoch: u64,
    owner_epoch: u64,
    generation: u64,
    fencing_token_sha256: Sha256Digest,
}

#[derive(Clone, Debug)]
struct StoredRow {
    record: H7TrajectoryRecord,
    lease_id: String,
    lease_head_sha256: Sha256Digest,
    authority_epoch: u64,
    owner_epoch: u64,
    generation: u64,
    fencing_token_sha256: Sha256Digest,
    payload_sha256: Sha256Digest,
    previous_sha256: Sha256Digest,
    event_sha256: Sha256Digest,
}

impl StoredRow {
    fn binding_tuple(&self) -> BindingTuple {
        BindingTuple {
            lease_id: self.lease_id.clone(),
            lease_head_sha256: self.lease_head_sha256.clone(),
            authority_epoch: self.authority_epoch,
            owner_epoch: self.owner_epoch,
            generation: self.generation,
            fencing_token_sha256: self.fencing_token_sha256.clone(),
        }
    }
}

fn binding_tuple(binding: &LocalTurnLifecycleBinding) -> BindingTuple {
    BindingTuple {
        lease_id: binding.lease_id.clone(),
        lease_head_sha256: binding.lease_head_sha256.clone(),
        authority_epoch: binding.fence.authority_epoch,
        owner_epoch: binding.fence.owner_epoch,
        generation: binding.fence.generation,
        fencing_token_sha256: binding.fencing_token_sha256.clone(),
    }
}

fn verify_current_binding(
    binding: &LocalTurnLifecycleBinding,
    lease: &LocalLeaseOutbox,
    executor: &LocalCompactExecutor,
    current: &crate::LocalLease,
) -> Result<(), H7TrajectoryStoreError> {
    let Some(lease_binding) = lease.binding() else {
        return Err(H7TrajectoryStoreError::StaleFence(
            "H7 trajectory requires an explicitly bound lease".to_string(),
        ));
    };
    let Some(compact_binding) = executor.lease_binding() else {
        return Err(H7TrajectoryStoreError::StaleFence(
            "H7 trajectory requires a bound compact executor".to_string(),
        ));
    };
    if current.lease_id != binding.lease_id
        || current.lease_sha256 != binding.lease_head_sha256
        || current.generation != binding.fence.generation
        || current.fencing_token != binding.fence.fencing_token
        || current.authority_epoch != Some(binding.fence.authority_epoch)
        || current.owner_epoch != Some(binding.fence.owner_epoch)
        || lease_binding.authority_epoch != binding.fence.authority_epoch
        || lease_binding.owner_epoch != binding.fence.owner_epoch
        || compact_binding.lease_id != binding.lease_id
        || compact_binding.lease_head_sha256 != binding.lease_head_sha256
    {
        return Err(H7TrajectoryStoreError::StaleFence(
            "current lease/compact head does not match trajectory binding".to_string(),
        ));
    }
    Ok(())
}

async fn load_trajectory_rows(
    transaction: &mut Transaction<'_, Sqlite>,
    store: &CognitiveStore,
    trajectory_id: &str,
) -> Result<Vec<StoredRow>, H7TrajectoryStoreError> {
    let rows = sqlx::query(
        "SELECT owner_agent_id, trajectory_id, event_seq, event_id, event_kind,
                turn_id, occurrence_key, causal_parent_seq, causal_parent_sha256,
                state_digest, policy_digest, model_receipt_digest, receipt_sha256,
                outcome, reward_bps, safety_ok, terminal, propensity_json,
                support_json, metadata_json, reason, external_effect_executed,
                kg_write_authority, production_caller, lease_id, lease_head_sha256,
                authority_epoch, owner_epoch, generation, fencing_token_sha256,
                payload_json, payload_sha256, previous_sha256, event_sha256
         FROM cognitive_h7_trajectory_events
         WHERE owner_agent_id = ? AND trajectory_id = ?
         ORDER BY event_seq",
    )
    .bind(store.owner_agent_id().as_str())
    .bind(trajectory_id)
    .fetch_all(&mut **transaction)
    .await
    .map_err(crate::cognitive_store::unavailable)?;
    if rows.len() > MAX_TRAJECTORY_ROWS {
        return Err(corrupt(format!(
            "trajectory exceeds {MAX_TRAJECTORY_ROWS} event rows"
        )));
    }
    let mut result: Vec<StoredRow> = Vec::with_capacity(rows.len());
    let mut previous = genesis_digest();
    for (index, row) in rows.iter().enumerate() {
        let owner: String = row.try_get("owner_agent_id").map_err(unavailable)?;
        let stored_trajectory: String = row.try_get("trajectory_id").map_err(unavailable)?;
        if owner != store.owner_agent_id().as_str() || stored_trajectory != trajectory_id {
            return Err(corrupt("trajectory row owner or id mismatch"));
        }
        let event_seq = read_u32(row, "event_seq")?;
        if event_seq != u32::try_from(index + 1).unwrap_or(u32::MAX) {
            return Err(corrupt("trajectory event sequence is not contiguous"));
        }
        let event_kind = H7TrajectoryEventKind::parse(
            row.try_get::<String, _>("event_kind")
                .map_err(unavailable)?
                .as_str(),
        )?;
        let causal_parent_seq = row
            .try_get::<Option<i64>, _>("causal_parent_seq")
            .map_err(unavailable)?
            .map(|value| {
                u32::try_from(value).map_err(|_| corrupt("causal parent sequence is invalid"))
            })
            .transpose()?;
        let causal_parent_sha256 = optional_digest(row, "causal_parent_sha256")?;
        let record = H7TrajectoryRecord {
            trajectory_id: stored_trajectory,
            event_seq,
            event_id: row.try_get("event_id").map_err(unavailable)?,
            event_kind,
            turn_id: row.try_get("turn_id").map_err(unavailable)?,
            occurrence_key: row.try_get("occurrence_key").map_err(unavailable)?,
            causal_parent_seq,
            causal_parent_sha256,
            state_digest: digest_from_row(row, "state_digest")?,
            policy_digest: digest_from_row(row, "policy_digest")?,
            model_receipt_digest: digest_from_row(row, "model_receipt_digest")?,
            receipt_sha256: digest_from_row(row, "receipt_sha256")?,
            outcome: row.try_get("outcome").map_err(unavailable)?,
            reward_bps: row.try_get("reward_bps").map_err(unavailable)?,
            safety_ok: read_bool(row, "safety_ok")?,
            terminal: read_bool(row, "terminal")?,
            propensity_json: row.try_get("propensity_json").map_err(unavailable)?,
            support_json: row.try_get("support_json").map_err(unavailable)?,
            metadata_json: row.try_get("metadata_json").map_err(unavailable)?,
            reason: row.try_get("reason").map_err(unavailable)?,
            external_effect_executed: read_bool(row, "external_effect_executed")?,
            kg_write_authority: read_bool(row, "kg_write_authority")?,
            production_caller: read_bool(row, "production_caller")?,
        };
        record.validate()?;
        if index > 0 {
            let prior = result
                .last()
                .ok_or_else(|| corrupt("missing prior trajectory row"))?;
            if prior.record.terminal {
                return Err(corrupt("trajectory contains an event after terminal"));
            }
            if record.turn_id != prior.record.turn_id
                || record.causal_parent_seq != Some(prior.record.event_seq)
                || record.causal_parent_sha256.as_ref() != Some(&prior.event_sha256)
            {
                return Err(corrupt("trajectory causal parent chain is invalid"));
            }
        }
        let lease_id: String = row.try_get("lease_id").map_err(unavailable)?;
        let lease_head_sha256 = digest_from_row(row, "lease_head_sha256")?;
        let authority_epoch = read_u64(row, "authority_epoch")?;
        let owner_epoch = read_u64(row, "owner_epoch")?;
        let generation = read_u64(row, "generation")?;
        let fencing_token_sha256 = digest_from_row(row, "fencing_token_sha256")?;
        verify_historical_lease(
            transaction,
            store,
            &lease_id,
            &lease_head_sha256,
            authority_epoch,
            owner_epoch,
            generation,
            &fencing_token_sha256,
        )
        .await?;
        let payload_json: String = row.try_get("payload_json").map_err(unavailable)?;
        if payload_json.len() > MAX_PAYLOAD_BYTES || payload_json.as_bytes().contains(&0) {
            return Err(corrupt("trajectory payload exceeds bounds"));
        }
        let payload_sha256 = digest_from_row(row, "payload_sha256")?;
        if payload_sha256 != Sha256Digest::for_bytes(payload_json.as_bytes())
            || serde_json::from_str::<H7TrajectoryRecord>(&payload_json)
                .map_err(|error| H7TrajectoryStoreError::Serialization(error.to_string()))?
                != record
        {
            return Err(corrupt(
                "trajectory payload does not match projected fields",
            ));
        }
        let previous_sha256 = digest_from_row(row, "previous_sha256")?;
        if previous_sha256 != previous {
            return Err(corrupt("trajectory previous digest mismatch"));
        }
        let event_sha256 = digest_from_row(row, "event_sha256")?;
        let binding_tuple = BindingTuple {
            lease_id: lease_id.clone(),
            lease_head_sha256: lease_head_sha256.clone(),
            authority_epoch,
            owner_epoch,
            generation,
            fencing_token_sha256: fencing_token_sha256.clone(),
        };
        if let Some(prior) = result.last()
            && prior.binding_tuple() != binding_tuple
        {
            return Err(corrupt(
                "trajectory binding changed across its immutable event chain",
            ));
        }
        let expected_event = event_digest(
            store.owner_agent_id().as_str(),
            trajectory_id,
            &record,
            &payload_sha256,
            &previous_sha256,
            &binding_tuple,
        );
        if event_sha256 != expected_event {
            return Err(corrupt("trajectory event digest mismatch"));
        }
        previous = event_sha256.clone();
        result.push(StoredRow {
            record,
            lease_id,
            lease_head_sha256,
            authority_epoch,
            owner_epoch,
            generation,
            fencing_token_sha256,
            payload_sha256,
            previous_sha256,
            event_sha256,
        });
    }
    Ok(result)
}

async fn read_h7_trajectory_in_transaction(
    transaction: &mut Transaction<'_, Sqlite>,
    store: &CognitiveStore,
    trajectory_id: &str,
    expected_binding: Option<&BindingTuple>,
) -> Result<Option<H7TrajectoryRead>, H7TrajectoryStoreError> {
    let rows = load_trajectory_rows(transaction, store, trajectory_id).await?;
    if rows.is_empty() {
        return Ok(None);
    }
    if let Some(expected_binding) = expected_binding {
        let head = rows
            .last()
            .ok_or_else(|| corrupt("trajectory head disappeared"))?;
        if &head.binding_tuple() != expected_binding {
            return Err(H7TrajectoryStoreError::StaleFence(
                "trajectory head does not match the current lifecycle binding".to_string(),
            ));
        }
    }
    let head_sha256 = rows
        .last()
        .map(|row| row.event_sha256.clone())
        .ok_or_else(|| corrupt("trajectory head disappeared"))?;
    Ok(Some(H7TrajectoryRead {
        trajectory_id: trajectory_id.to_string(),
        events: rows.into_iter().map(|row| row.record).collect(),
        head_sha256,
    }))
}

async fn verify_historical_lease(
    transaction: &mut Transaction<'_, Sqlite>,
    store: &CognitiveStore,
    lease_id: &str,
    lease_head_sha256: &Sha256Digest,
    authority_epoch: u64,
    owner_epoch: u64,
    generation: u64,
    fencing_token_sha256: &Sha256Digest,
) -> Result<(), H7TrajectoryStoreError> {
    let row = sqlx::query(
        "SELECT fencing_token, lease_expires_at_unix_seconds
         FROM cognitive_local_leases
         WHERE lease_id = ? AND owner_agent_id = ? AND lease_sha256 = ?
           AND authority_epoch = ? AND owner_epoch = ? AND generation = ?",
    )
    .bind(lease_id)
    .bind(store.owner_agent_id().as_str())
    .bind(lease_head_sha256.as_str())
    .bind(to_i64(authority_epoch, "authority epoch")?)
    .bind(to_i64(owner_epoch, "owner epoch")?)
    .bind(to_i64(generation, "lease generation")?)
    .fetch_optional(&mut **transaction)
    .await
    .map_err(crate::cognitive_store::unavailable)?
    .ok_or_else(|| corrupt("trajectory row is not bound to a historical lease head"))?;
    let fencing_token: String = row.try_get("fencing_token").map_err(unavailable)?;
    let expiry: i64 = row
        .try_get("lease_expires_at_unix_seconds")
        .map_err(unavailable)?;
    if expiry <= 0 || fencing_token_sha256 != &fencing_token_identity_digest(&fencing_token) {
        return Err(corrupt(
            "trajectory lease binding token or expiry is invalid",
        ));
    }
    Ok(())
}

async fn insert_row(
    transaction: &mut Transaction<'_, Sqlite>,
    store: &CognitiveStore,
    record: &H7TrajectoryRecord,
    binding: &BindingTuple,
    payload_json: &str,
    payload_sha256: &Sha256Digest,
    previous_sha256: &Sha256Digest,
    event_sha256: &Sha256Digest,
    recorded_at: i64,
) -> Result<(), H7TrajectoryStoreError> {
    sqlx::query(
        "INSERT INTO cognitive_h7_trajectory_events (
            owner_agent_id, trajectory_id, event_seq, event_id, event_kind,
            turn_id, occurrence_key, causal_parent_seq, causal_parent_sha256,
            state_digest, policy_digest, model_receipt_digest, receipt_sha256,
            outcome, reward_bps, safety_ok, terminal, propensity_json,
            support_json, metadata_json, reason, external_effect_executed,
            kg_write_authority, production_caller, lease_id, lease_head_sha256,
            authority_epoch, owner_epoch, generation, fencing_token_sha256,
            payload_json, payload_sha256, previous_sha256, event_sha256,
            recorded_at_unix_seconds
         ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?,
                   ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(store.owner_agent_id().as_str())
    .bind(&record.trajectory_id)
    .bind(to_i64(u64::from(record.event_seq), "event sequence")?)
    .bind(&record.event_id)
    .bind(record.event_kind.as_str())
    .bind(&record.turn_id)
    .bind(&record.occurrence_key)
    .bind(
        record
            .causal_parent_seq
            .map(u64::from)
            .map(|value| to_i64(value, "causal parent sequence"))
            .transpose()?,
    )
    .bind(
        record
            .causal_parent_sha256
            .as_ref()
            .map(Sha256Digest::as_str),
    )
    .bind(record.state_digest.as_str())
    .bind(record.policy_digest.as_str())
    .bind(record.model_receipt_digest.as_str())
    .bind(record.receipt_sha256.as_str())
    .bind(&record.outcome)
    .bind(i64::from(record.reward_bps))
    .bind(record.safety_ok)
    .bind(record.terminal)
    .bind(record.propensity_json.as_deref())
    .bind(record.support_json.as_deref())
    .bind(&record.metadata_json)
    .bind(&record.reason)
    .bind(record.external_effect_executed)
    .bind(record.kg_write_authority)
    .bind(record.production_caller)
    .bind(&binding.lease_id)
    .bind(binding.lease_head_sha256.as_str())
    .bind(to_i64(binding.authority_epoch, "authority epoch")?)
    .bind(to_i64(binding.owner_epoch, "owner epoch")?)
    .bind(to_i64(binding.generation, "lease generation")?)
    .bind(binding.fencing_token_sha256.as_str())
    .bind(payload_json)
    .bind(payload_sha256.as_str())
    .bind(previous_sha256.as_str())
    .bind(event_sha256.as_str())
    .bind(recorded_at)
    .execute(&mut **transaction)
    .await
    .map_err(crate::cognitive_store::unavailable)?;
    Ok(())
}

fn event_digest(
    owner_agent_id: &str,
    trajectory_id: &str,
    record: &H7TrajectoryRecord,
    payload_sha256: &Sha256Digest,
    previous_sha256: &Sha256Digest,
    binding: &BindingTuple,
) -> Sha256Digest {
    let mut hasher = Sha256::new();
    frame_part(&mut hasher, EVENT_DOMAIN);
    for part in [
        owner_agent_id.as_bytes(),
        trajectory_id.as_bytes(),
        record.event_id.as_bytes(),
        record.event_kind.as_str().as_bytes(),
        record.turn_id.as_bytes(),
        record.occurrence_key.as_bytes(),
        record.state_digest.as_str().as_bytes(),
        record.policy_digest.as_str().as_bytes(),
        record.model_receipt_digest.as_str().as_bytes(),
        record.receipt_sha256.as_str().as_bytes(),
        record.outcome.as_bytes(),
        record.metadata_json.as_bytes(),
        record.reason.as_bytes(),
        payload_sha256.as_str().as_bytes(),
        previous_sha256.as_str().as_bytes(),
        binding.lease_id.as_bytes(),
        binding.lease_head_sha256.as_str().as_bytes(),
        binding.fencing_token_sha256.as_str().as_bytes(),
    ] {
        frame_part(&mut hasher, part);
    }
    frame_part(&mut hasher, &record.event_seq.to_be_bytes());
    frame_part(
        &mut hasher,
        &record.causal_parent_seq.unwrap_or(0).to_be_bytes(),
    );
    frame_part(
        &mut hasher,
        record
            .causal_parent_sha256
            .as_ref()
            .map_or(b"".as_slice(), |digest| digest.as_str().as_bytes()),
    );
    frame_part(&mut hasher, &record.reward_bps.to_be_bytes());
    frame_part(&mut hasher, &[u8::from(record.safety_ok)]);
    frame_part(&mut hasher, &[u8::from(record.terminal)]);
    frame_part(&mut hasher, &binding.authority_epoch.to_be_bytes());
    frame_part(&mut hasher, &binding.owner_epoch.to_be_bytes());
    frame_part(&mut hasher, &binding.generation.to_be_bytes());
    Sha256Digest::from_sha256_output(hasher.finalize())
}

fn fencing_token_identity_digest(token: &str) -> Sha256Digest {
    let mut hasher = Sha256::new();
    frame_part(&mut hasher, FENCING_TOKEN_DOMAIN);
    frame_part(&mut hasher, b"fencing-token");
    frame_part(&mut hasher, token.as_bytes());
    // Keep this exactly aligned with LocalTurnLifecycleBinding's private
    // identity digest (which intentionally hashes the framed digest bytes).
    Sha256Digest::for_bytes(&hasher.finalize())
}

fn genesis_digest() -> Sha256Digest {
    Sha256Digest::for_bytes(GENESIS)
}

fn validate_metadata(metadata: &str) -> Result<(), H7TrajectoryStoreError> {
    if metadata.len() > MAX_METADATA_BYTES || metadata.as_bytes().contains(&0) {
        return Err(invalid("trajectory metadata exceeds bounds"));
    }
    serde_json::from_str::<serde_json::Value>(metadata)
        .map_err(|error| H7TrajectoryStoreError::Serialization(error.to_string()))?;
    Ok(())
}

fn validate_text(value: &str, label: &str, max_bytes: usize) -> Result<(), H7TrajectoryStoreError> {
    if value.trim().is_empty() || value.len() > max_bytes || value.as_bytes().contains(&0) {
        return Err(invalid(format!(
            "{label} must contain 1..={max_bytes} non-NUL bytes"
        )));
    }
    Ok(())
}

fn validate_digest(value: &Sha256Digest, label: &str) -> Result<(), H7TrajectoryStoreError> {
    Sha256Digest::parse(value.as_str().to_string())
        .map(|_| ())
        .map_err(|_| invalid(format!("{label} is not a lowercase SHA-256 digest")))
}

fn now_unix_seconds() -> Result<i64, H7TrajectoryStoreError> {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| H7TrajectoryStoreError::Clock(error.to_string()))?
        .as_secs();
    i64::try_from(seconds)
        .map_err(|_| H7TrajectoryStoreError::Clock("timestamp overflow".to_string()))
}

fn to_i64(value: u64, label: &str) -> Result<i64, H7TrajectoryStoreError> {
    i64::try_from(value).map_err(|_| invalid(format!("{label} overflows SQLite INTEGER")))
}

fn read_u64(row: &sqlx::sqlite::SqliteRow, column: &str) -> Result<u64, H7TrajectoryStoreError> {
    let value: i64 = row.try_get(column).map_err(unavailable)?;
    u64::try_from(value).map_err(|_| corrupt(format!("{column} is negative")))
}

fn read_u32(row: &sqlx::sqlite::SqliteRow, column: &str) -> Result<u32, H7TrajectoryStoreError> {
    let value = read_u64(row, column)?;
    u32::try_from(value).map_err(|_| corrupt(format!("{column} overflows u32")))
}

fn read_bool(row: &sqlx::sqlite::SqliteRow, column: &str) -> Result<bool, H7TrajectoryStoreError> {
    let value: bool = row.try_get(column).map_err(unavailable)?;
    Ok(value)
}

fn digest_from_row(
    row: &sqlx::sqlite::SqliteRow,
    column: &str,
) -> Result<Sha256Digest, H7TrajectoryStoreError> {
    let value: String = row.try_get(column).map_err(unavailable)?;
    Sha256Digest::parse(value).map_err(corrupt)
}

fn optional_digest(
    row: &sqlx::sqlite::SqliteRow,
    column: &str,
) -> Result<Option<Sha256Digest>, H7TrajectoryStoreError> {
    let value: Option<String> = row.try_get(column).map_err(unavailable)?;
    value
        .map(|value| Sha256Digest::parse(value).map_err(corrupt))
        .transpose()
}

fn unavailable(error: impl std::fmt::Display) -> H7TrajectoryStoreError {
    H7TrajectoryStoreError::Store(CognitiveStoreError::Unavailable(error.to_string()))
}

fn invalid(message: impl Into<String>) -> H7TrajectoryStoreError {
    H7TrajectoryStoreError::Invalid(message.into())
}

fn corrupt(message: impl Into<String>) -> H7TrajectoryStoreError {
    H7TrajectoryStoreError::Corrupt(message.into())
}
