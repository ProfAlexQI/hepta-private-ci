//! Small SQLite-backed local-development compact executor.
//!
//! This is the first real persistence seam behind the typed compact contract.
//! It owns one append-only journal in the Agent-local cognitive database and
//! applies the existing checkpoint CAS/fence rules inside `BEGIN IMMEDIATE`.
//! It deliberately does not write KG facts, invoke a scheduler, route a
//! workflow, or dispatch an external effect.  Rehydration is a read-only
//! reconstruction plan backed by a durable committed checkpoint.

use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use sqlx::Row;
use sqlx::Sqlite;
use sqlx::Transaction;
use thiserror::Error;

use crate::CognitiveStore;
use crate::CognitiveStoreError;
use crate::CompactCheckpoint;
use crate::CompactFence;
use crate::CompactParentSnapshot;
use crate::CompactPersistenceAppend;
use crate::CompactPersistenceError;
use crate::CompactPersistenceEvent;
use crate::CompactPersistenceEventKind;
use crate::CompactPersistenceJournal;
use crate::CompactPersistenceSnapshot;
use crate::CompactPersistenceState;
use crate::CompactReconcileOutcome;
use crate::CompactRehydrationRecord;
use crate::RehydrationPlan;
use crate::RehydrationStatus;
use crate::checkpoint_digest;

pub const LOCAL_COMPACT_EXECUTOR_NAMESPACE: &str = "local_development_only";
pub const LOCAL_COMPACT_EXECUTOR_SCHEMA_VERSION: u32 = 1;
pub const LOCAL_COMPACT_EXECUTOR_EXTERNAL_EFFECTS: bool = false;
pub const LOCAL_COMPACT_EXECUTOR_KG_WRITE_AUTHORITY: bool = false;
const MAX_JOURNAL_EVENTS: usize = 4_096;

#[derive(Debug, Error)]
pub enum LocalCompactExecutorError {
    #[error(transparent)]
    Store(#[from] CognitiveStoreError),
    #[error(transparent)]
    Persistence(#[from] CompactPersistenceError),
    #[error("invalid local compact executor input: {0}")]
    Invalid(String),
    #[error("local compact executor journal is corrupt: {0}")]
    Corrupt(String),
    #[error("local compact executor serialization failed: {0}")]
    Serialization(String),
    #[error("local compact executor clock failed: {0}")]
    Clock(String),
}

/// One Agent-local append-only checkpoint executor.
///
/// The executor is intentionally explicit about its scope.  It is suitable
/// for local development, replay, and bounded shadow runs only; callers must
/// not treat its successful commit as a production promotion or an external
/// effect receipt.
#[derive(Clone)]
pub struct LocalCompactExecutor {
    store: CognitiveStore,
    journal_id: String,
    fence: CompactFence,
}

impl LocalCompactExecutor {
    pub(crate) async fn open(
        store: &CognitiveStore,
        journal_id: impl Into<String>,
        fence: CompactFence,
    ) -> Result<Self, LocalCompactExecutorError> {
        let journal_id = journal_id.into();
        validate_text(&journal_id, "journal id", 512)?;
        validate_fence(&fence)?;
        let executor = Self {
            store: store.clone(),
            journal_id,
            fence,
        };
        // Opening always verifies the complete chain.  This is intentionally
        // done before any mutation so a damaged journal fails closed.
        let mut transaction = executor
            .store
            .pool
            .begin()
            .await
            .map_err(crate::cognitive_store::unavailable)?;
        let _ = executor.load_journal(&mut transaction).await?;
        transaction
            .commit()
            .await
            .map_err(crate::cognitive_store::unavailable)?;
        Ok(executor)
    }

    pub fn journal_id(&self) -> &str {
        &self.journal_id
    }

    pub fn fence(&self) -> &CompactFence {
        &self.fence
    }

    /// Appends an idempotent checkpoint intent under the current parent CAS.
    pub async fn append_intent(
        &self,
        operation_id: impl Into<String>,
        checkpoint: &CompactCheckpoint,
        current: &CompactParentSnapshot,
    ) -> Result<CompactPersistenceAppend, LocalCompactExecutorError> {
        let operation_id = operation_id.into();
        self.mutate(|journal| journal.append_intent(operation_id, checkpoint, current))
            .await
    }

    /// Commits a previously appended checkpoint intent.
    pub async fn commit_checkpoint(
        &self,
        operation_id: &str,
        checkpoint_sha256: &codex_hepta_contracts::Sha256Digest,
    ) -> Result<CompactPersistenceAppend, LocalCompactExecutorError> {
        self.mutate(|journal| journal.commit_checkpoint(operation_id, checkpoint_sha256))
            .await
    }

    /// Quarantines an uncertain local commit.  Unknown outcomes never become
    /// a successful receipt implicitly.
    pub async fn mark_indeterminate(
        &self,
        operation_id: &str,
        reason_code: impl Into<String>,
    ) -> Result<CompactPersistenceAppend, LocalCompactExecutorError> {
        let reason_code = reason_code.into();
        self.mutate(|journal| journal.mark_indeterminate(operation_id, reason_code))
            .await
    }

    /// Reconciles a quarantined operation explicitly.
    pub async fn reconcile(
        &self,
        operation_id: &str,
        outcome: CompactReconcileOutcome,
    ) -> Result<CompactPersistenceAppend, LocalCompactExecutorError> {
        self.mutate(|journal| journal.reconcile(operation_id, outcome))
            .await
    }

    /// Reopens and verifies the durable append-only journal.
    pub async fn snapshot(&self) -> Result<CompactPersistenceSnapshot, LocalCompactExecutorError> {
        let mut transaction = self
            .store
            .pool
            .begin()
            .await
            .map_err(crate::cognitive_store::unavailable)?;
        let journal = self.load_journal(&mut transaction).await?;
        transaction
            .commit()
            .await
            .map_err(crate::cognitive_store::unavailable)?;
        Ok(journal.snapshot())
    }

    /// Returns the durable state for one operation after chain verification.
    pub async fn state(
        &self,
        operation_id: &str,
    ) -> Result<Option<CompactPersistenceState>, LocalCompactExecutorError> {
        validate_text(operation_id, "operation id", 512)?;
        let snapshot = self.snapshot().await?;
        let journal = CompactPersistenceJournal::reopen(snapshot)?;
        Ok(journal.state(operation_id))
    }

    /// Reads the committed checkpoint's rehydration plan and any durable local
    /// witness without appending an event.
    ///
    /// This is the extension-facing read seam.  It verifies the complete
    /// journal, the committed operation state, and the exact checkpoint
    /// digest/revision binding.  A missing witness is represented as
    /// `NotStarted`; a present witness upgrades the returned plan to
    /// `Complete`.  The method never writes KG/projection rows, routes a
    /// request, invokes a provider, or mutates the compact journal.
    pub async fn read_rehydration(
        &self,
        operation_id: &str,
        checkpoint: &CompactCheckpoint,
        expected_revision: u64,
    ) -> Result<LocalRehydrationRead, LocalCompactExecutorError> {
        validate_text(operation_id, "operation id", 512)?;
        let plan = checkpoint
            .rehydration_plan(expected_revision)
            .map_err(|error| LocalCompactExecutorError::Invalid(error.to_string()))?;
        let snapshot = self.snapshot().await?;
        let journal = CompactPersistenceJournal::reopen(snapshot)?;
        if journal.state(operation_id) != Some(CompactPersistenceState::Committed) {
            return Err(LocalCompactExecutorError::Invalid(format!(
                "operation {operation_id} is not durably committed"
            )));
        }
        let expected_digest = checkpoint_digest(checkpoint)?;
        let intent_matches = journal.entries().iter().any(|entry| {
            entry.operation_id == operation_id
                && matches!(
                    &entry.kind,
                    CompactPersistenceEventKind::Intent {
                        checkpoint_sha256,
                        ..
                    } if *checkpoint_sha256 == expected_digest
                )
        });
        if !intent_matches {
            return Err(LocalCompactExecutorError::Corrupt(format!(
                "operation {operation_id} is committed with a different checkpoint"
            )));
        }
        let witness = journal.rehydration(operation_id).cloned();
        if let Some(witness) = witness.as_ref()
            && (witness.checkpoint_sha256 != expected_digest
                || witness.expected_revision != expected_revision
                || witness.sequence == 0)
        {
            return Err(LocalCompactExecutorError::Corrupt(format!(
                "operation {operation_id} has an invalid rehydration witness"
            )));
        }
        let status = if witness.is_some() {
            RehydrationStatus::Complete
        } else {
            RehydrationStatus::NotStarted
        };
        Ok(LocalRehydrationRead {
            plan: RehydrationPlan { status, ..plan },
            checkpoint_sha256: expected_digest,
            witness,
        })
    }

    /// Executes the local rehydration step for a committed checkpoint.  The
    /// plan remains read-only, while an append-only local witness is persisted
    /// so restart/reopen can replay the acknowledgement without duplicating
    /// metadata.  No KG/projection row or external effect is written or
    /// claimed.
    pub async fn rehydrate(
        &self,
        operation_id: &str,
        checkpoint: &CompactCheckpoint,
        expected_revision: u64,
    ) -> Result<RehydrationPlan, LocalCompactExecutorError> {
        let read = self
            .read_rehydration(operation_id, checkpoint, expected_revision)
            .await?;
        let _ = self
            .mutate(|journal| {
                journal.record_rehydration(operation_id, &read.checkpoint_sha256, expected_revision)
            })
            .await?;
        Ok(RehydrationPlan {
            status: RehydrationStatus::Complete,
            ..read.plan
        })
    }

    /// Returns the durable local rehydration witness after reopening and
    /// verifying the complete hash chain.
    pub async fn rehydration(
        &self,
        operation_id: &str,
    ) -> Result<Option<CompactRehydrationRecord>, LocalCompactExecutorError> {
        validate_text(operation_id, "operation id", 512)?;
        let snapshot = self.snapshot().await?;
        let journal = CompactPersistenceJournal::reopen(snapshot)?;
        Ok(journal.rehydration(operation_id).cloned())
    }

    async fn mutate<F>(
        &self,
        mutation: F,
    ) -> Result<CompactPersistenceAppend, LocalCompactExecutorError>
    where
        F: FnOnce(
            &mut CompactPersistenceJournal,
        ) -> Result<CompactPersistenceAppend, CompactPersistenceError>,
    {
        let mut transaction = self
            .store
            .pool
            .begin_with("BEGIN IMMEDIATE")
            .await
            .map_err(crate::cognitive_store::unavailable)?;
        let mut journal = self.load_journal(&mut transaction).await?;
        let before = journal.entries().len();
        let result = mutation(&mut journal)?;
        for entry in &journal.entries()[before..] {
            self.insert_event(&mut transaction, entry).await?;
        }
        transaction
            .commit()
            .await
            .map_err(crate::cognitive_store::unavailable)?;
        Ok(result)
    }

    async fn load_journal(
        &self,
        transaction: &mut Transaction<'_, Sqlite>,
    ) -> Result<CompactPersistenceJournal, LocalCompactExecutorError> {
        let rows = sqlx::query(
            "SELECT sequence, owner_agent_id, authority_epoch, owner_epoch,
                    generation, fencing_token,
                    event_json, previous_sha256, event_sha256
             FROM cognitive_compact_events
             WHERE journal_id = ?
               AND owner_agent_id = ?
             ORDER BY sequence",
        )
        .bind(&self.journal_id)
        .bind(self.store.owner_agent_id().as_str())
        .fetch_all(&mut **transaction)
        .await
        .map_err(crate::cognitive_store::unavailable)?;
        if rows.len() > MAX_JOURNAL_EVENTS {
            return Err(LocalCompactExecutorError::Corrupt(format!(
                "journal exceeds {MAX_JOURNAL_EVENTS} event reopen limit"
            )));
        }
        let mut entries = Vec::with_capacity(rows.len());
        for row in rows {
            let sequence: i64 = row
                .try_get("sequence")
                .map_err(crate::cognitive_store::unavailable)?;
            let owner_agent_id: String = row
                .try_get("owner_agent_id")
                .map_err(crate::cognitive_store::unavailable)?;
            let authority_epoch: Option<i64> = row
                .try_get("authority_epoch")
                .map_err(crate::cognitive_store::unavailable)?;
            let owner_epoch: Option<i64> = row
                .try_get("owner_epoch")
                .map_err(crate::cognitive_store::unavailable)?;
            let generation: i64 = row
                .try_get("generation")
                .map_err(crate::cognitive_store::unavailable)?;
            let fencing_token: String = row
                .try_get("fencing_token")
                .map_err(crate::cognitive_store::unavailable)?;
            let event_json: String = row
                .try_get("event_json")
                .map_err(crate::cognitive_store::unavailable)?;
            let previous_sha256: String = row
                .try_get("previous_sha256")
                .map_err(crate::cognitive_store::unavailable)?;
            let event_sha256: String = row
                .try_get("event_sha256")
                .map_err(crate::cognitive_store::unavailable)?;
            let authority_epoch = authority_epoch
                .and_then(|value| u64::try_from(value).ok())
                .ok_or_else(|| {
                    LocalCompactExecutorError::Corrupt(
                        "event row is missing a valid authority epoch".to_string(),
                    )
                })?;
            let owner_epoch = owner_epoch
                .and_then(|value| u64::try_from(value).ok())
                .ok_or_else(|| {
                    LocalCompactExecutorError::Corrupt(
                        "event row is missing a valid owner epoch".to_string(),
                    )
                })?;
            if owner_agent_id != self.store.owner_agent_id().as_str()
                || authority_epoch != self.fence.authority_epoch
                || owner_epoch != self.fence.owner_epoch
                || generation != i64::try_from(self.fence.generation).unwrap_or(i64::MAX)
                || fencing_token != self.fence.fencing_token
            {
                return Err(LocalCompactExecutorError::Corrupt(
                    "event owner or fence does not match executor".to_string(),
                ));
            }
            let entry: CompactPersistenceEvent = serde_json::from_str(&event_json)
                .map_err(|error| LocalCompactExecutorError::Serialization(error.to_string()))?;
            if sequence != i64::try_from(entry.sequence).unwrap_or(i64::MAX)
                || authority_epoch != entry.authority_epoch
                || owner_epoch != entry.owner_epoch
                || previous_sha256 != entry.previous_sha256.as_str()
                || event_sha256 != entry.event_sha256.as_str()
            {
                return Err(LocalCompactExecutorError::Corrupt(
                    "event row metadata does not match its serialized event".to_string(),
                ));
            }
            entries.push(entry);
        }
        let head_sha256 = entries
            .last()
            .map(|entry| entry.event_sha256.clone())
            .unwrap_or_else(|| {
                codex_hepta_contracts::Sha256Digest::for_bytes(
                    b"hepta-memory:compact-persistence:empty:v1",
                )
            });
        let snapshot = CompactPersistenceSnapshot {
            schema_version: crate::COMPACT_PERSISTENCE_SCHEMA_VERSION,
            namespace: crate::COMPACT_PERSISTENCE_NAMESPACE.to_string(),
            fence: self.fence.clone(),
            entries,
            head_sha256,
        };
        CompactPersistenceJournal::reopen(snapshot).map_err(Into::into)
    }

    async fn insert_event(
        &self,
        transaction: &mut Transaction<'_, Sqlite>,
        entry: &CompactPersistenceEvent,
    ) -> Result<(), LocalCompactExecutorError> {
        let event_json = serde_json::to_string(entry)
            .map_err(|error| LocalCompactExecutorError::Serialization(error.to_string()))?;
        let recorded_at_unix_seconds = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|error| LocalCompactExecutorError::Clock(error.to_string()))?
            .as_secs();
        let recorded_at_unix_seconds = i64::try_from(recorded_at_unix_seconds)
            .map_err(|_| LocalCompactExecutorError::Clock("timestamp overflow".to_string()))?;
        sqlx::query(
            "INSERT INTO cognitive_compact_events (
                journal_id, owner_agent_id, authority_epoch, owner_epoch,
                sequence, generation, fencing_token,
                event_json, previous_sha256, event_sha256, recorded_at_unix_seconds
             ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&self.journal_id)
        .bind(self.store.owner_agent_id().as_str())
        .bind(i64::try_from(self.fence.authority_epoch).map_err(|_| {
            LocalCompactExecutorError::Invalid("authority epoch overflow".to_string())
        })?)
        .bind(
            i64::try_from(self.fence.owner_epoch).map_err(|_| {
                LocalCompactExecutorError::Invalid("owner epoch overflow".to_string())
            })?,
        )
        .bind(i64::try_from(entry.sequence).map_err(|_| {
            LocalCompactExecutorError::Invalid("event sequence overflow".to_string())
        })?)
        .bind(i64::try_from(entry.generation).map_err(|_| {
            LocalCompactExecutorError::Invalid("event generation overflow".to_string())
        })?)
        .bind(&entry.fencing_token)
        .bind(event_json)
        .bind(entry.previous_sha256.as_str())
        .bind(entry.event_sha256.as_str())
        .bind(recorded_at_unix_seconds)
        .execute(&mut **transaction)
        .await
        .map_err(crate::cognitive_store::unavailable)?;
        Ok(())
    }
}

/// Pure read result for a local compact rehydration attempt.
///
/// The optional witness is local append-only metadata.  This value itself is
/// not an authority grant and is safe to pass to an explicit host adapter that
/// retains only digest metadata.
#[derive(Clone, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
pub struct LocalRehydrationRead {
    pub plan: RehydrationPlan,
    pub checkpoint_sha256: codex_hepta_contracts::Sha256Digest,
    pub witness: Option<CompactRehydrationRecord>,
}

impl CognitiveStore {
    /// Opens the local-development-only compact checkpoint executor.
    pub async fn open_local_compact_executor(
        &self,
        journal_id: impl Into<String>,
        fence: CompactFence,
    ) -> Result<LocalCompactExecutor, LocalCompactExecutorError> {
        LocalCompactExecutor::open(self, journal_id, fence).await
    }
}

fn validate_fence(fence: &CompactFence) -> Result<(), LocalCompactExecutorError> {
    if fence.authority_epoch == 0 || fence.owner_epoch == 0 || fence.generation == 0 {
        return Err(LocalCompactExecutorError::Invalid(
            "compact fence epochs must be non-zero".to_string(),
        ));
    }
    validate_text(&fence.fencing_token, "fencing token", 256)
}

fn validate_text(
    value: &str,
    label: &str,
    max_bytes: usize,
) -> Result<(), LocalCompactExecutorError> {
    if value.trim().is_empty() || value.len() > max_bytes || value.as_bytes().contains(&0) {
        return Err(LocalCompactExecutorError::Invalid(format!(
            "{label} must contain 1..={max_bytes} non-NUL bytes"
        )));
    }
    Ok(())
}
