//! Qualification-only MemoryAdmission saga over the Agent-local lease/event/
//! outbox journal.
//!
//! The CognitiveStore memory writer and the local outbox are deliberately
//! separate append-only transactions.  This module makes that boundary
//! explicit instead of pretending that two SQLite-facing APIs form a hidden
//! two-phase commit: a local command is first Pending (`Queued`), then becomes
//! Applied, Rejected, or Revoked through an immutable outcome event.  A retry
//! consults the local outcome and the deterministic memory identity before it
//! calls the target writer, so a successful candidate is never written twice.
//!
//! Nothing in this module dispatches an outbox row, writes a shared KG, or
//! grants production/caller/promotion authority.

use codex_hepta_contracts::Sha256Digest;
use serde::Serialize;
use serde_json::json;

use crate::CognitiveAccess;
use crate::CognitiveScope;
use crate::CognitiveStoreError;
use crate::LocalAdmission;
use crate::LocalLeaseOutbox;
use crate::LocalOutcomeState;
use crate::MemoryAdmissionReceipt;
use crate::MemoryCandidateDraft;
use crate::MemoryCandidateOrigin;
use crate::MemoryLifecycleState;
use crate::MemoryRevisionRecord;
use crate::StableMemoryId;

pub const LOCAL_MEMORY_ADMISSION_SAGA_SCHEMA_VERSION: u32 = 1;
pub const LOCAL_MEMORY_ADMISSION_SAGA_NAMESPACE: &str = "local_development_only";
pub const LOCAL_MEMORY_ADMISSION_SAGA_EXTERNAL_EFFECTS: bool = false;
pub const LOCAL_MEMORY_ADMISSION_SAGA_KG_WRITE_AUTHORITY: bool = false;
pub const LOCAL_MEMORY_ADMISSION_SAGA_PRODUCTION_CALLER: bool = false;
pub const LOCAL_MEMORY_ADMISSION_SAGA_PROMOTION: bool = false;

const ADMISSION_TOPIC: &str = "hepta.memory.admission.v1";
const TOMBSTONE_TOPIC: &str = "hepta.memory.tombstone.v1";

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalMemoryAdmissionState {
    Pending,
    Applied,
    Rejected,
    Revoked,
}

impl LocalMemoryAdmissionState {
    fn from_local(state: LocalOutcomeState) -> Result<Self, LocalMemoryAdmissionError> {
        match state {
            LocalOutcomeState::Queued => Ok(Self::Pending),
            LocalOutcomeState::Committed => Ok(Self::Applied),
            LocalOutcomeState::Rejected => Ok(Self::Rejected),
            LocalOutcomeState::RolledBack => Ok(Self::Revoked),
            LocalOutcomeState::Indeterminate => Err(LocalMemoryAdmissionError::Indeterminate),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum LocalMemoryAdmissionError {
    #[error(transparent)]
    Store(#[from] CognitiveStoreError),
    #[error(transparent)]
    Lease(#[from] crate::LocalLeaseOutboxError),
    #[error("memory admission target outcome is indeterminate")]
    Indeterminate,
    #[error("invalid local memory admission saga: {0}")]
    Invalid(String),
}

/// A local saga receipt.  `admission` is present only for the transaction that
/// actually wrote the candidate; retries expose the stable identity and
/// revision while avoiding a second CognitiveStore write.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalMemoryAdmissionReceipt {
    pub schema_version: u32,
    pub namespace: String,
    pub command_id: String,
    pub occurrence_key: String,
    pub candidate_id: StableMemoryId,
    pub revision: Option<u64>,
    pub state: LocalMemoryAdmissionState,
    pub admission: Option<MemoryAdmissionReceipt>,
    pub rejection_reason: Option<String>,
    pub external_effects: bool,
    pub kg_write_authority: bool,
    pub production_caller: bool,
    pub promotion: bool,
}

#[derive(Serialize)]
struct CandidateCommand<'a> {
    schema_version: u32,
    kind: &'static str,
    stable_key: &'a str,
    scope: &'a CognitiveScope,
    content_sha256: Sha256Digest,
    source_event_key: &'a str,
    observed_at_unix_seconds: i64,
    origin: MemoryCandidateOrigin,
}

#[derive(Serialize)]
struct TombstoneCommand<'a> {
    schema_version: u32,
    kind: &'static str,
    memory_id: &'a str,
    expected_revision: u64,
    reason_sha256: Sha256Digest,
    valid_from_unix_seconds: i64,
    origin: MemoryCandidateOrigin,
}

#[derive(Clone)]
struct CommandIdentity {
    command_id: String,
    occurrence_key: String,
    payload_json: String,
    candidate_id: StableMemoryId,
}

impl LocalLeaseOutbox {
    /// Admit a model/compaction candidate through the existing local
    /// lease/event/outbox seam.  The target CognitiveStore write is local and
    /// provisional; the saga only records Applied after that write returns.
    pub async fn admit_memory_candidate_saga(
        &self,
        access: &CognitiveAccess,
        draft: &MemoryCandidateDraft,
    ) -> Result<LocalMemoryAdmissionReceipt, LocalMemoryAdmissionError> {
        let identity = candidate_identity(self, draft)?;
        let existing = self
            .inspect_occurrence(identity.occurrence_key.clone())
            .await?;
        if let Some(state) = existing {
            match state {
                LocalOutcomeState::Queued => {}
                _ => return self.replay_candidate(access, draft, identity, state).await,
            }
        }

        let _queued = match self
            .admit(
                identity.occurrence_key.clone(),
                ADMISSION_TOPIC,
                identity.payload_json.clone(),
            )
            .await?
        {
            LocalAdmission::Queued(receipt) | LocalAdmission::Replay(receipt) => receipt,
        };

        match self.store().admit_memory_candidate(access, draft).await {
            Ok(admission) => {
                let receipt_digest = admission.write.memory.content_sha256.clone();
                match self
                    .apply(
                        identity.occurrence_key.clone(),
                        format!("candidate:{}", receipt_digest.as_str()),
                    )
                    .await
                {
                    Ok(_) => Ok(LocalMemoryAdmissionReceipt {
                        schema_version: LOCAL_MEMORY_ADMISSION_SAGA_SCHEMA_VERSION,
                        namespace: LOCAL_MEMORY_ADMISSION_SAGA_NAMESPACE.to_string(),
                        command_id: identity.command_id,
                        occurrence_key: identity.occurrence_key,
                        candidate_id: admission.candidate_id.clone(),
                        revision: Some(admission.revision),
                        state: LocalMemoryAdmissionState::Applied,
                        admission: Some(admission),
                        rejection_reason: None,
                        external_effects: LOCAL_MEMORY_ADMISSION_SAGA_EXTERNAL_EFFECTS,
                        kg_write_authority: LOCAL_MEMORY_ADMISSION_SAGA_KG_WRITE_AUTHORITY,
                        production_caller: LOCAL_MEMORY_ADMISSION_SAGA_PRODUCTION_CALLER,
                        promotion: LOCAL_MEMORY_ADMISSION_SAGA_PROMOTION,
                    }),
                    Err(error) => {
                        self.recover_apply_race(
                            access,
                            &identity,
                            &admission.candidate_id,
                            Some(admission.revision),
                            error,
                        )
                        .await
                    }
                }
            }
            Err(error) => {
                // A concurrent retry may have completed the target write after
                // our inspection.  Prefer the matching durable candidate over
                // recording a false Rejected outcome.
                if let Some(current) = self
                    .matching_candidate(access, &identity.candidate_id, draft)
                    .await?
                {
                    return self
                        .apply_existing_candidate(
                            &identity,
                            current.id.revision,
                            format!("candidate:{}", current.content_sha256.as_str()),
                        )
                        .await;
                }
                let reason = bounded_reason(error.to_string());
                self.reject_saga(identity, reason).await
            }
        }
    }

    /// Tombstone a candidate through the same Pending -> Applied/Rejected
    /// target-ack path.  The underlying CognitiveStore forget operation is
    /// append-only and cannot resurrect a tombstoned head.
    pub async fn tombstone_memory_candidate_saga(
        &self,
        access: &CognitiveAccess,
        memory_id: &StableMemoryId,
        expected_revision: u64,
        origin: MemoryCandidateOrigin,
        source: &crate::SourceDraft,
        reason: String,
        valid_from_unix_seconds: i64,
    ) -> Result<LocalMemoryAdmissionReceipt, LocalMemoryAdmissionError> {
        let identity = tombstone_identity(
            memory_id,
            expected_revision,
            origin,
            source,
            &reason,
            valid_from_unix_seconds,
        )?;
        if let Some(state) = self
            .inspect_occurrence(identity.occurrence_key.clone())
            .await?
        {
            return self
                .replay_tombstone(access, identity, state, memory_id, reason)
                .await;
        }
        let _queued = self
            .admit(
                identity.occurrence_key.clone(),
                TOMBSTONE_TOPIC,
                identity.payload_json.clone(),
            )
            .await?;
        match self
            .store()
            .tombstone_memory_candidate(
                access,
                memory_id,
                expected_revision,
                origin,
                source,
                reason.clone(),
                valid_from_unix_seconds,
            )
            .await
        {
            Ok(admission) => {
                let digest = admission.write.memory.content_sha256.clone();
                match self
                    .apply(
                        identity.occurrence_key.clone(),
                        format!("tombstone:{}", digest.as_str()),
                    )
                    .await
                {
                    Ok(_) => Ok(self.receipt(
                        identity,
                        Some(admission.revision),
                        LocalMemoryAdmissionState::Applied,
                        Some(admission),
                        None,
                    )),
                    Err(error) => {
                        self.recover_apply_race(access, &identity, memory_id, None, error)
                            .await
                    }
                }
            }
            Err(error) => {
                if let Some(current) = self
                    .tombstoned_candidate(access, memory_id, &reason)
                    .await?
                {
                    return self
                        .apply_existing_candidate(
                            &identity,
                            current.id.revision,
                            format!("tombstone:{}", current.content_sha256.as_str()),
                        )
                        .await;
                }
                self.reject_saga(identity, bounded_reason(error.to_string()))
                    .await
            }
        }
    }

    /// Revoke a still-pending candidate command without touching the target
    /// store.  A committed or rejected command is never rewritten as revoked.
    pub async fn revoke_memory_candidate_saga(
        &self,
        draft: &MemoryCandidateDraft,
        reason: impl Into<String>,
    ) -> Result<LocalMemoryAdmissionReceipt, LocalMemoryAdmissionError> {
        let identity = candidate_identity(self, draft)?;
        let state = self
            .inspect_occurrence(identity.occurrence_key.clone())
            .await?
            .ok_or_else(|| {
                LocalMemoryAdmissionError::Invalid(
                    "cannot revoke a candidate command that was never admitted".to_string(),
                )
            })?;
        match state {
            LocalOutcomeState::Queued | LocalOutcomeState::Indeterminate => {
                let _ =
                    LocalLeaseOutbox::revoke(self, identity.occurrence_key.clone(), reason.into())
                        .await?;
                Ok(self.receipt(
                    identity,
                    None,
                    LocalMemoryAdmissionState::Revoked,
                    None,
                    None,
                ))
            }
            LocalOutcomeState::Committed => Err(LocalMemoryAdmissionError::Invalid(
                "an applied candidate cannot be revoked through the pending-command seam"
                    .to_string(),
            )),
            LocalOutcomeState::Rejected | LocalOutcomeState::RolledBack => Ok(self.receipt(
                identity,
                None,
                LocalMemoryAdmissionState::from_local(state)?,
                None,
                None,
            )),
        }
    }

    async fn replay_candidate(
        &self,
        access: &CognitiveAccess,
        draft: &MemoryCandidateDraft,
        identity: CommandIdentity,
        state: LocalOutcomeState,
    ) -> Result<LocalMemoryAdmissionReceipt, LocalMemoryAdmissionError> {
        match state {
            LocalOutcomeState::Queued => Err(LocalMemoryAdmissionError::Invalid(
                "queued candidate replay must pass through the admission writer".to_string(),
            )),
            LocalOutcomeState::Committed => {
                let current = self
                    .matching_candidate(access, &identity.candidate_id, draft)
                    .await?
                    .ok_or_else(|| {
                        LocalMemoryAdmissionError::Invalid(
                            "local outcome is Applied but candidate target is missing or changed"
                                .to_string(),
                        )
                    })?;
                Ok(self.receipt(
                    identity,
                    Some(current.id.revision),
                    LocalMemoryAdmissionState::Applied,
                    None,
                    None,
                ))
            }
            LocalOutcomeState::Rejected | LocalOutcomeState::RolledBack => Ok(self.receipt(
                identity,
                None,
                LocalMemoryAdmissionState::from_local(state)?,
                None,
                None,
            )),
            LocalOutcomeState::Indeterminate => Err(LocalMemoryAdmissionError::Indeterminate),
        }
    }

    async fn replay_tombstone(
        &self,
        access: &CognitiveAccess,
        identity: CommandIdentity,
        state: LocalOutcomeState,
        memory_id: &StableMemoryId,
        reason: String,
    ) -> Result<LocalMemoryAdmissionReceipt, LocalMemoryAdmissionError> {
        match state {
            LocalOutcomeState::Committed => {
                let current = self
                    .tombstoned_candidate(access, memory_id, &reason)
                    .await?
                    .ok_or_else(|| {
                        LocalMemoryAdmissionError::Invalid(
                            "local outcome is Applied but tombstone target is missing".to_string(),
                        )
                    })?;
                Ok(self.receipt(
                    identity,
                    Some(current.id.revision),
                    LocalMemoryAdmissionState::Applied,
                    None,
                    None,
                ))
            }
            LocalOutcomeState::Queued => Err(LocalMemoryAdmissionError::Invalid(
                "tombstone command is still Pending; retry requires explicit target input"
                    .to_string(),
            )),
            LocalOutcomeState::Rejected | LocalOutcomeState::RolledBack => Ok(self.receipt(
                identity,
                None,
                LocalMemoryAdmissionState::from_local(state)?,
                None,
                None,
            )),
            LocalOutcomeState::Indeterminate => Err(LocalMemoryAdmissionError::Indeterminate),
        }
    }

    async fn recover_apply_race(
        &self,
        access: &CognitiveAccess,
        identity: &CommandIdentity,
        candidate_id: &StableMemoryId,
        revision: Option<u64>,
        error: crate::LocalLeaseOutboxError,
    ) -> Result<LocalMemoryAdmissionReceipt, LocalMemoryAdmissionError> {
        if self
            .inspect_occurrence(identity.occurrence_key.clone())
            .await?
            == Some(LocalOutcomeState::Committed)
        {
            let revision = match revision {
                Some(revision) => revision,
                None => {
                    self.store()
                        .latest_memory(access, candidate_id)
                        .await?
                        .id
                        .revision
                }
            };
            return Ok(self.receipt(
                identity.clone(),
                Some(revision),
                LocalMemoryAdmissionState::Applied,
                None,
                None,
            ));
        }
        Err(error.into())
    }

    async fn apply_existing_candidate(
        &self,
        identity: &CommandIdentity,
        revision: u64,
        receipt: String,
    ) -> Result<LocalMemoryAdmissionReceipt, LocalMemoryAdmissionError> {
        match self.apply(identity.occurrence_key.clone(), receipt).await {
            Ok(_) => Ok(self.receipt(
                identity.clone(),
                Some(revision),
                LocalMemoryAdmissionState::Applied,
                None,
                None,
            )),
            Err(error) => {
                if self
                    .inspect_occurrence(identity.occurrence_key.clone())
                    .await?
                    == Some(LocalOutcomeState::Committed)
                {
                    Ok(self.receipt(
                        identity.clone(),
                        Some(revision),
                        LocalMemoryAdmissionState::Applied,
                        None,
                        None,
                    ))
                } else {
                    Err(error.into())
                }
            }
        }
    }

    async fn matching_candidate(
        &self,
        access: &CognitiveAccess,
        candidate_id: &StableMemoryId,
        draft: &MemoryCandidateDraft,
    ) -> Result<Option<MemoryRevisionRecord>, LocalMemoryAdmissionError> {
        let current = match self.store().latest_memory(access, candidate_id).await {
            Ok(current) => current,
            Err(CognitiveStoreError::Invalid(message)) if message == "memory does not exist" => {
                return Ok(None)
            }
            Err(error) => return Err(error.into()),
        };
        if current.scope == draft.scope
            && current.content == draft.content
            && current.verification == crate::MemoryVerification::Provisional
            && current.lifecycle == MemoryLifecycleState::Active
        {
            Ok(Some(current))
        } else {
            Ok(None)
        }
    }

    async fn tombstoned_candidate(
        &self,
        access: &CognitiveAccess,
        memory_id: &StableMemoryId,
        reason: &str,
    ) -> Result<Option<MemoryRevisionRecord>, LocalMemoryAdmissionError> {
        let current = match self.store().latest_memory(access, memory_id).await {
            Ok(current) => current,
            Err(CognitiveStoreError::Invalid(message)) if message == "memory does not exist" => {
                return Ok(None)
            }
            Err(error) => return Err(error.into()),
        };
        if current.lifecycle
            == (MemoryLifecycleState::Tombstoned {
                reason: reason.to_string(),
            })
        {
            Ok(Some(current))
        } else {
            Ok(None)
        }
    }

    async fn reject_saga(
        &self,
        identity: CommandIdentity,
        reason: String,
    ) -> Result<LocalMemoryAdmissionReceipt, LocalMemoryAdmissionError> {
        self.reject_inner(&identity.occurrence_key, &reason).await?;
        Ok(self.receipt(
            identity,
            None,
            LocalMemoryAdmissionState::Rejected,
            None,
            Some(reason),
        ))
    }

    async fn reject_inner(
        &self,
        occurrence_key: &str,
        reason: &str,
    ) -> Result<(), LocalMemoryAdmissionError> {
        match LocalLeaseOutbox::reject(self, occurrence_key.to_string(), reason.to_string()).await {
            Ok(_) => Ok(()),
            Err(error) => {
                if self.inspect_occurrence(occurrence_key.to_string()).await?
                    == Some(LocalOutcomeState::Rejected)
                {
                    Ok(())
                } else {
                    Err(error.into())
                }
            }
        }
    }

    fn receipt(
        &self,
        identity: CommandIdentity,
        revision: Option<u64>,
        state: LocalMemoryAdmissionState,
        admission: Option<MemoryAdmissionReceipt>,
        rejection_reason: Option<String>,
    ) -> LocalMemoryAdmissionReceipt {
        LocalMemoryAdmissionReceipt {
            schema_version: LOCAL_MEMORY_ADMISSION_SAGA_SCHEMA_VERSION,
            namespace: LOCAL_MEMORY_ADMISSION_SAGA_NAMESPACE.to_string(),
            command_id: identity.command_id,
            occurrence_key: identity.occurrence_key,
            candidate_id: identity.candidate_id,
            revision,
            state,
            admission,
            rejection_reason,
            external_effects: LOCAL_MEMORY_ADMISSION_SAGA_EXTERNAL_EFFECTS,
            kg_write_authority: LOCAL_MEMORY_ADMISSION_SAGA_KG_WRITE_AUTHORITY,
            production_caller: LOCAL_MEMORY_ADMISSION_SAGA_PRODUCTION_CALLER,
            promotion: LOCAL_MEMORY_ADMISSION_SAGA_PROMOTION,
        }
    }
}

fn candidate_identity(
    lease: &LocalLeaseOutbox,
    draft: &MemoryCandidateDraft,
) -> Result<CommandIdentity, LocalMemoryAdmissionError> {
    crate::cognitive_store::validate_key(&draft.stable_key, "candidate stable key")
        .map_err(LocalMemoryAdmissionError::Store)?;
    let content_sha256 = Sha256Digest::for_bytes(draft.content.as_bytes());
    let command = CandidateCommand {
        schema_version: LOCAL_MEMORY_ADMISSION_SAGA_SCHEMA_VERSION,
        kind: "memory_candidate_admission",
        stable_key: &draft.stable_key,
        scope: &draft.scope,
        content_sha256,
        source_event_key: &draft.source_event_key,
        observed_at_unix_seconds: draft.observed_at_unix_seconds,
        origin: draft.origin,
    };
    let payload_json = serde_json::to_string(&json!({
        "schema_version": LOCAL_MEMORY_ADMISSION_SAGA_SCHEMA_VERSION,
        "kind": "memory_candidate_admission",
        "command": command,
        "external_effects": LOCAL_MEMORY_ADMISSION_SAGA_EXTERNAL_EFFECTS,
        "kg_write_authority": LOCAL_MEMORY_ADMISSION_SAGA_KG_WRITE_AUTHORITY,
        "production_caller": LOCAL_MEMORY_ADMISSION_SAGA_PRODUCTION_CALLER,
        "promotion": LOCAL_MEMORY_ADMISSION_SAGA_PROMOTION,
    }))
    .map_err(|error| LocalMemoryAdmissionError::Invalid(error.to_string()))?;
    let digest = Sha256Digest::for_bytes(payload_json.as_bytes());
    let command_id = format!("memcmd:v1:{}", digest.as_str());
    let occurrence_key = format!("memadm:v1:{}", digest.as_str());
    let candidate_id = StableMemoryId::for_key(
        lease.store().owner_agent_id(),
        &draft.scope,
        &draft.stable_key,
    );
    Ok(CommandIdentity {
        command_id,
        occurrence_key,
        payload_json,
        candidate_id,
    })
}

fn tombstone_identity(
    memory_id: &StableMemoryId,
    expected_revision: u64,
    origin: MemoryCandidateOrigin,
    source: &crate::SourceDraft,
    reason: &str,
    valid_from_unix_seconds: i64,
) -> Result<CommandIdentity, LocalMemoryAdmissionError> {
    if reason.trim().is_empty() || reason.len() > 256 || reason.as_bytes().contains(&0) {
        return Err(LocalMemoryAdmissionError::Invalid(
            "tombstone reason must contain 1..=256 non-NUL bytes".to_string(),
        ));
    }
    let command = TombstoneCommand {
        schema_version: LOCAL_MEMORY_ADMISSION_SAGA_SCHEMA_VERSION,
        kind: "memory_candidate_tombstone",
        memory_id: memory_id.as_str(),
        expected_revision,
        reason_sha256: Sha256Digest::for_bytes(reason.as_bytes()),
        valid_from_unix_seconds,
        origin,
    };
    let payload_json = serde_json::to_string(&json!({
        "schema_version": LOCAL_MEMORY_ADMISSION_SAGA_SCHEMA_VERSION,
        "kind": "memory_candidate_tombstone",
        "command": command,
        "source_event_key": source.event_key,
        "source_kind": source.kind,
        "external_effects": LOCAL_MEMORY_ADMISSION_SAGA_EXTERNAL_EFFECTS,
        "kg_write_authority": LOCAL_MEMORY_ADMISSION_SAGA_KG_WRITE_AUTHORITY,
        "production_caller": LOCAL_MEMORY_ADMISSION_SAGA_PRODUCTION_CALLER,
        "promotion": LOCAL_MEMORY_ADMISSION_SAGA_PROMOTION,
    }))
    .map_err(|error| LocalMemoryAdmissionError::Invalid(error.to_string()))?;
    let digest = Sha256Digest::for_bytes(payload_json.as_bytes());
    Ok(CommandIdentity {
        command_id: format!("memcmd:v1:{}", digest.as_str()),
        occurrence_key: format!("memtomb:v1:{}", digest.as_str()),
        payload_json,
        candidate_id: memory_id.clone(),
    })
}

fn bounded_reason(reason: String) -> String {
    reason.chars().take(512).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cognitive_test_support::agent_id;
    use crate::cognitive_test_support::layout;
    use crate::CognitiveScope;
    use crate::CognitiveStore;
    use crate::LedgerSourceKind;
    use crate::LocalLeaseAcquire;
    use crate::SourceDraft;
    use tempfile::TempDir;

    async fn setup(number: u8) -> (TempDir, CognitiveStore, LocalLeaseOutbox) {
        let temp = TempDir::new().expect("temp");
        let owner = agent_id(number);
        let store = CognitiveStore::open(&layout(&temp, &owner))
            .await
            .expect("store");
        let lease = match store
            .acquire_local_lease("lease:memory-saga", 1, "fence:memory-saga")
            .await
            .expect("lease")
        {
            LocalLeaseAcquire::Acquired(lease) | LocalLeaseAcquire::Replay(lease) => lease,
        };
        (temp, store, lease)
    }

    fn draft() -> MemoryCandidateDraft {
        MemoryCandidateDraft {
            stable_key: "candidate:saga:1".to_string(),
            scope: CognitiveScope::AgentPrivate,
            content: "A local candidate is not yet a fact.".to_string(),
            source_event_key: "turn:event:1".to_string(),
            observed_at_unix_seconds: 100,
            origin: MemoryCandidateOrigin::CompactionSummary,
        }
    }

    #[tokio::test]
    async fn candidate_saga_applies_once_and_replays_without_double_write() {
        let (_temp, store, lease) = setup(211).await;
        let owner = store.owner_agent_id().clone();
        let access = CognitiveAccess::agent_private(owner);
        let first = lease
            .admit_memory_candidate_saga(&access, &draft())
            .await
            .expect("first admission");
        assert_eq!(first.state, LocalMemoryAdmissionState::Applied);
        assert!(!first.external_effects);
        assert!(!first.kg_write_authority);
        assert!(!first.production_caller);
        assert!(!first.promotion);
        let second = lease
            .admit_memory_candidate_saga(&access, &draft())
            .await
            .expect("idempotent replay");
        assert_eq!(second.state, LocalMemoryAdmissionState::Applied);
        assert_eq!(second.candidate_id, first.candidate_id);
        assert_eq!(second.revision, Some(1));
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM memory_revisions")
            .fetch_one(&store.pool)
            .await
            .expect("memory count");
        assert_eq!(count, 1);
        let counts = lease.snapshot_counts().await.expect("journal counts");
        assert_eq!(counts.event_rows, 2);
        assert_eq!(counts.outbox_rows, 1);
    }

    #[tokio::test]
    async fn pending_candidate_can_be_revoked_without_target_write() {
        let (_temp, store, lease) = setup(212).await;
        let candidate = draft();
        let identity = candidate_identity(&lease, &candidate).expect("identity");
        lease
            .admit(
                identity.occurrence_key.clone(),
                ADMISSION_TOPIC,
                identity.payload_json,
            )
            .await
            .expect("pending admission");
        let revoked = lease
            .revoke_memory_candidate_saga(&candidate, "qualification revoked")
            .await
            .expect("revoke");
        assert_eq!(revoked.state, LocalMemoryAdmissionState::Revoked);
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM memory_revisions")
            .fetch_one(&store.pool)
            .await
            .expect("memory count");
        assert_eq!(count, 0);
    }

    #[tokio::test]
    async fn queued_target_success_recovery_does_not_append_a_second_revision() {
        let (_temp, store, lease) = setup(214).await;
        let candidate = draft();
        let identity = candidate_identity(&lease, &candidate).expect("identity");
        lease
            .admit(
                identity.occurrence_key.clone(),
                ADMISSION_TOPIC,
                identity.payload_json,
            )
            .await
            .expect("pending admission");
        // Model the crash window after the target transaction committed but
        // before the local Applied outcome was appended.  The saga retry will
        // receive the deterministic stable-key conflict and must adopt the
        // existing target revision rather than writing another one.
        let owner = store.owner_agent_id().clone();
        store
            .admit_memory_candidate(&CognitiveAccess::agent_private(owner), &candidate)
            .await
            .expect("target-side candidate");
        let applied = lease
            .admit_memory_candidate_saga(
                &CognitiveAccess::agent_private(store.owner_agent_id().clone()),
                &candidate,
            )
            .await
            .expect("recover queued target");
        assert_eq!(applied.state, LocalMemoryAdmissionState::Applied);
        assert_eq!(applied.revision, Some(1));
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM memory_revisions")
            .fetch_one(&store.pool)
            .await
            .expect("memory count");
        assert_eq!(count, 1);
    }

    #[tokio::test]
    async fn tombstone_saga_is_append_only_and_idempotent() {
        let (_temp, store, lease) = setup(213).await;
        let owner = store.owner_agent_id().clone();
        let access = CognitiveAccess::agent_private(owner);
        let candidate = lease
            .admit_memory_candidate_saga(&access, &draft())
            .await
            .expect("candidate");
        let source = SourceDraft {
            scope: CognitiveScope::AgentPrivate,
            kind: LedgerSourceKind::ExplicitMemoryDirective,
            event_key: "forget:event:1".to_string(),
            content: b"privacy erasure".to_vec(),
            observed_at_unix_seconds: 101,
        };
        let first = lease
            .tombstone_memory_candidate_saga(
                &access,
                &candidate.candidate_id,
                1,
                MemoryCandidateOrigin::ModelProposal,
                &source,
                "privacy erasure".to_string(),
                101,
            )
            .await
            .expect("tombstone");
        assert_eq!(first.state, LocalMemoryAdmissionState::Applied);
        let second = lease
            .tombstone_memory_candidate_saga(
                &access,
                &candidate.candidate_id,
                1,
                MemoryCandidateOrigin::ModelProposal,
                &source,
                "privacy erasure".to_string(),
                101,
            )
            .await
            .expect("tombstone replay");
        assert_eq!(second.state, LocalMemoryAdmissionState::Applied);
        assert_eq!(second.revision, Some(2));
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM memory_revisions")
            .fetch_one(&store.pool)
            .await
            .expect("memory count");
        assert_eq!(count, 2);
    }
}
