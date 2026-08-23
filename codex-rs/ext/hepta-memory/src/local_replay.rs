//! Explicit, bounded local rehydration replay consumption.
//!
//! H14 produces a read-only [`LocalRehydrationRuntimePlan`].  H15 consumes
//! that value without making it an executor: it revalidates the H14 envelope,
//! binds it to the caller's turn and current compact fence, and returns a
//! typed disposition.  There is deliberately no write path in this module.
//! In particular, consuming a plan does not record a witness, append an event
//! or outbox row, touch the KG, register a runtime, route a request, or invoke
//! a provider/effect.

use codex_extension_api::ExtensionData;
use codex_hepta_contracts::Sha256Digest;
use codex_hepta_memory::CompactFence;
use codex_hepta_memory::LocalCompactExecutor;
use codex_hepta_memory::LocalLeaseOutbox;
use serde::Deserialize;
use serde::Serialize;
use sha2::Digest;
use sha2::Sha256;

use crate::LocalRehydrationHostInput;
use crate::LocalRehydrationRuntimeError;
use crate::LocalRehydrationRuntimePlan;
use crate::prepare_local_rehydration_runtime;

/// Schema version for the explicit H15 replay-consumer envelope.
pub const LOCAL_REHYDRATION_REPLAY_SCHEMA_VERSION: u32 = 1;
/// H15 remains local-development metadata, never a production runtime grant.
pub const LOCAL_REHYDRATION_REPLAY_NAMESPACE: &str = "local_development_only";
pub const LOCAL_REHYDRATION_REPLAY_EXTERNAL_EFFECTS: bool = false;
pub const LOCAL_REHYDRATION_REPLAY_KG_WRITE_AUTHORITY: bool = false;
pub const LOCAL_REHYDRATION_REPLAY_ROUTING_AUTHORITY: bool = false;
pub const LOCAL_REHYDRATION_REPLAY_PROVIDER_EFFECTS: bool = false;
pub const LOCAL_REHYDRATION_REPLAY_PRODUCTION_CALLER: bool = false;
/// A replay consumer is not installed as a runtime contributor.
pub const LOCAL_REHYDRATION_REPLAY_RUNTIME_REGISTERED: bool = false;
/// H15 observes a plan; it never performs the explicit H14 rehydrate write.
pub const LOCAL_REHYDRATION_REPLAY_PERFORMED: bool = false;

const H13_BINDING_DOMAIN: &[u8] = b"hepta-memory:local-rehydration-host:v1";
const H14_BINDING_DOMAIN: &[u8] = b"hepta-memory:local-rehydration-runtime:v1";
const REPLAY_BINDING_DOMAIN: &[u8] = b"hepta-memory:local-rehydration-replay:v1";

/// The only two states H15 can expose.  `NotStarted` means that an explicit
/// local rehydrate operation has not left a witness; it does not authorize
/// H15 to perform that operation.  `Complete` means that a witness is already
/// present and was observed through H14.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalRehydrationReplayDisposition {
    NotStarted,
    Complete,
}

/// Errors returned when a replay plan cannot be accepted as the exact local
/// turn/fence-bound H14 result.
#[derive(Debug)]
pub enum LocalRehydrationReplayError {
    Runtime(LocalRehydrationRuntimeError),
    TurnBindingMismatch,
    FenceMismatch(String),
    Invalid(String),
}

impl std::fmt::Display for LocalRehydrationReplayError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Runtime(error) => {
                write!(formatter, "local replay runtime plan rejected: {error}")
            }
            Self::TurnBindingMismatch => {
                formatter.write_str("local replay turn binding does not match")
            }
            Self::FenceMismatch(detail) => {
                write!(formatter, "local replay fence mismatch: {detail}")
            }
            Self::Invalid(detail) => write!(formatter, "invalid local replay plan: {detail}"),
        }
    }
}

impl std::error::Error for LocalRehydrationReplayError {}

impl From<LocalRehydrationRuntimeError> for LocalRehydrationReplayError {
    fn from(error: LocalRehydrationRuntimeError) -> Self {
        Self::Runtime(error)
    }
}

/// A digest-bound, read-only H15 replay result.
///
/// The envelope intentionally carries only identities and the H14 source
/// binding, not a mutable executor or a write-capable handle.  Its authority
/// fields are fixed false and are checked by [`Self::validate`].
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalRehydrationReplayPlan {
    pub schema_version: u32,
    pub namespace: String,
    pub turn_id_sha256: Sha256Digest,
    pub lease_id_sha256: Sha256Digest,
    pub fencing_token_sha256: Sha256Digest,
    pub authority_epoch: u64,
    pub owner_epoch: u64,
    pub generation: u64,
    pub disposition: LocalRehydrationReplayDisposition,
    pub checkpoint_sha256: Sha256Digest,
    pub operation_id_sha256: Sha256Digest,
    pub witness_present: bool,
    pub source_runtime_binding_sha256: Sha256Digest,
    pub replay_binding_sha256: Sha256Digest,
    pub external_effects: bool,
    pub kg_write_authority: bool,
    pub routing_authority: bool,
    pub provider_effects: bool,
    pub production_caller: bool,
    pub runtime_registered: bool,
    pub replay_performed: bool,
}

impl LocalRehydrationReplayPlan {
    /// Revalidates the H15 envelope without consulting or mutating runtime
    /// state.  Consumers should call this even when the value came directly
    /// from [`consume_local_rehydration_runtime_plan`].
    pub fn validate(&self) -> Result<(), LocalRehydrationReplayError> {
        if self.schema_version != LOCAL_REHYDRATION_REPLAY_SCHEMA_VERSION
            || self.namespace != LOCAL_REHYDRATION_REPLAY_NAMESPACE
        {
            return Err(LocalRehydrationReplayError::Invalid(
                "unsupported schema or namespace".to_string(),
            ));
        }
        if self.external_effects
            || self.kg_write_authority
            || self.routing_authority
            || self.provider_effects
            || self.production_caller
            || self.runtime_registered
            || self.replay_performed
        {
            return Err(LocalRehydrationReplayError::Invalid(
                "replay consumer crosses the local-development read-only boundary".to_string(),
            ));
        }
        if self.authority_epoch == 0 || self.owner_epoch == 0 || self.generation == 0 {
            return Err(LocalRehydrationReplayError::Invalid(
                "replay fence epochs and generation must be non-zero".to_string(),
            ));
        }
        for digest in [
            &self.turn_id_sha256,
            &self.lease_id_sha256,
            &self.fencing_token_sha256,
            &self.checkpoint_sha256,
            &self.operation_id_sha256,
            &self.source_runtime_binding_sha256,
            &self.replay_binding_sha256,
        ] {
            validate_digest(digest)?;
        }
        let expected_witness = matches!(
            self.disposition,
            LocalRehydrationReplayDisposition::Complete
        );
        if self.witness_present != expected_witness {
            return Err(LocalRehydrationReplayError::Invalid(
                "replay disposition does not match witness presence".to_string(),
            ));
        }
        if self.replay_binding_sha256
            != replay_binding_digest(
                &self.turn_id_sha256,
                &self.lease_id_sha256,
                &self.fencing_token_sha256,
                self.authority_epoch,
                self.owner_epoch,
                self.generation,
                self.disposition,
                &self.checkpoint_sha256,
                &self.operation_id_sha256,
                self.witness_present,
                &self.source_runtime_binding_sha256,
            )
        {
            return Err(LocalRehydrationReplayError::Invalid(
                "replay binding digest mismatch".to_string(),
            ));
        }
        Ok(())
    }

    /// H15 never grants execution or write authority.
    pub fn is_local_read_only(&self) -> bool {
        self.namespace == LOCAL_REHYDRATION_REPLAY_NAMESPACE
            && !self.external_effects
            && !self.kg_write_authority
            && !self.routing_authority
            && !self.provider_effects
            && !self.production_caller
            && !self.runtime_registered
            && !self.replay_performed
    }
}

/// Consume an already-produced H14 plan without performing rehydration.
///
/// `turn_id`, `lease_id`, and `fence` are the caller's current identities;
/// they are required so a copied or stale H14 plan cannot be accepted merely
/// because its internal digest is self-consistent.  This function is pure and
/// does not read or write a database.
pub fn consume_local_rehydration_runtime_plan(
    plan: &LocalRehydrationRuntimePlan,
    turn_id: &str,
    lease_id: &str,
    fence: &CompactFence,
) -> Result<LocalRehydrationReplayPlan, LocalRehydrationReplayError> {
    // Do not replace this with `is_local_read_only()`: that helper is a
    // convenience predicate, while H15 requires cryptographic validation.
    plan.validate()?;
    validate_runtime_plan_digests(plan)?;
    validate_text(turn_id, "turn id", 256)?;
    validate_text(lease_id, "lease id", 512)?;
    validate_fence(fence)?;

    let expected_turn = identity_digest(H13_BINDING_DOMAIN, b"turn", turn_id);
    if plan.host_read.turn_id_sha256 != expected_turn {
        return Err(LocalRehydrationReplayError::TurnBindingMismatch);
    }
    let expected_lease = identity_digest(H14_BINDING_DOMAIN, b"lease", lease_id);
    if plan.lease_id_sha256 != expected_lease {
        return Err(LocalRehydrationReplayError::FenceMismatch(
            "lease identity does not match the current lease".to_string(),
        ));
    }
    let expected_fence =
        identity_digest(H14_BINDING_DOMAIN, b"fencing-token", &fence.fencing_token);
    if plan.fencing_token_sha256 != expected_fence {
        return Err(LocalRehydrationReplayError::FenceMismatch(
            "fencing token identity does not match the current fence".to_string(),
        ));
    }
    if plan.authority_epoch != fence.authority_epoch
        || plan.owner_epoch != fence.owner_epoch
        || plan.lease_generation != fence.generation
        || plan.compact_generation != fence.generation
    {
        return Err(LocalRehydrationReplayError::FenceMismatch(
            "runtime plan epochs or generation are stale".to_string(),
        ));
    }

    let disposition = match plan.disposition {
        crate::LocalRehydrationRuntimeDisposition::NotStarted => {
            LocalRehydrationReplayDisposition::NotStarted
        }
        crate::LocalRehydrationRuntimeDisposition::Complete => {
            LocalRehydrationReplayDisposition::Complete
        }
    };
    let mut replay = LocalRehydrationReplayPlan {
        schema_version: LOCAL_REHYDRATION_REPLAY_SCHEMA_VERSION,
        namespace: LOCAL_REHYDRATION_REPLAY_NAMESPACE.to_string(),
        turn_id_sha256: plan.host_read.turn_id_sha256.clone(),
        lease_id_sha256: plan.lease_id_sha256.clone(),
        fencing_token_sha256: plan.fencing_token_sha256.clone(),
        authority_epoch: plan.authority_epoch,
        owner_epoch: plan.owner_epoch,
        generation: plan.compact_generation,
        disposition,
        checkpoint_sha256: plan.host_read.checkpoint_sha256.clone(),
        operation_id_sha256: plan.host_read.operation_id_sha256.clone(),
        witness_present: plan.host_read.read.witness.is_some(),
        source_runtime_binding_sha256: plan.binding_sha256.clone(),
        replay_binding_sha256: Sha256Digest::for_bytes(b"pending"),
        external_effects: LOCAL_REHYDRATION_REPLAY_EXTERNAL_EFFECTS,
        kg_write_authority: LOCAL_REHYDRATION_REPLAY_KG_WRITE_AUTHORITY,
        routing_authority: LOCAL_REHYDRATION_REPLAY_ROUTING_AUTHORITY,
        provider_effects: LOCAL_REHYDRATION_REPLAY_PROVIDER_EFFECTS,
        production_caller: LOCAL_REHYDRATION_REPLAY_PRODUCTION_CALLER,
        runtime_registered: LOCAL_REHYDRATION_REPLAY_RUNTIME_REGISTERED,
        replay_performed: LOCAL_REHYDRATION_REPLAY_PERFORMED,
    };
    replay.replay_binding_sha256 = replay_binding_digest(
        &replay.turn_id_sha256,
        &replay.lease_id_sha256,
        &replay.fencing_token_sha256,
        replay.authority_epoch,
        replay.owner_epoch,
        replay.generation,
        replay.disposition,
        &replay.checkpoint_sha256,
        &replay.operation_id_sha256,
        replay.witness_present,
        &replay.source_runtime_binding_sha256,
    );
    replay.validate()?;
    Ok(replay)
}

/// Read H14's plan and immediately consume it as an H15 typed disposition.
///
/// This convenience wrapper performs only H14's existing read-only SQL
/// verification and then calls the pure consumer above.  It does not call
/// `rehydrate`, `admit`, `reconcile`, `release`, or any runtime registration.
pub async fn prepare_local_rehydration_replay(
    turn_store: &ExtensionData,
    lease: &LocalLeaseOutbox,
    executor: &LocalCompactExecutor,
    input: LocalRehydrationHostInput<'_>,
) -> Result<LocalRehydrationReplayPlan, LocalRehydrationReplayError> {
    let plan = prepare_local_rehydration_runtime(turn_store, lease, executor, input).await?;
    consume_local_rehydration_runtime_plan(
        &plan,
        turn_store.level_id(),
        lease.lease_id(),
        executor.fence(),
    )
}

fn identity_digest(domain: &[u8], kind: &[u8], value: &str) -> Sha256Digest {
    let mut hasher = Sha256::new();
    hash_part(&mut hasher, domain);
    hash_part(&mut hasher, kind);
    hash_part(&mut hasher, value.as_bytes());
    Sha256Digest::for_bytes(&hasher.finalize())
}

fn replay_binding_digest(
    turn_id: &Sha256Digest,
    lease_id: &Sha256Digest,
    fencing_token: &Sha256Digest,
    authority_epoch: u64,
    owner_epoch: u64,
    generation: u64,
    disposition: LocalRehydrationReplayDisposition,
    checkpoint: &Sha256Digest,
    operation: &Sha256Digest,
    witness_present: bool,
    source_runtime_binding: &Sha256Digest,
) -> Sha256Digest {
    let mut hasher = Sha256::new();
    hash_part(&mut hasher, REPLAY_BINDING_DOMAIN);
    for digest in [
        turn_id,
        lease_id,
        fencing_token,
        checkpoint,
        operation,
        source_runtime_binding,
    ] {
        hash_part(&mut hasher, digest.as_str().as_bytes());
    }
    for value in [authority_epoch, owner_epoch, generation] {
        hash_part(&mut hasher, value.to_be_bytes().as_slice());
    }
    hash_part(
        &mut hasher,
        match disposition {
            LocalRehydrationReplayDisposition::NotStarted => b"not_started",
            LocalRehydrationReplayDisposition::Complete => b"complete",
        },
    );
    hash_part(&mut hasher, &[u8::from(witness_present)]);
    Sha256Digest::for_bytes(&hasher.finalize())
}

fn validate_fence(fence: &CompactFence) -> Result<(), LocalRehydrationReplayError> {
    if fence.authority_epoch == 0 || fence.owner_epoch == 0 || fence.generation == 0 {
        return Err(LocalRehydrationReplayError::Invalid(
            "fence epochs and generation must be non-zero".to_string(),
        ));
    }
    validate_text(&fence.fencing_token, "fencing token", 256)
}

fn validate_text(
    value: &str,
    label: &str,
    max_bytes: usize,
) -> Result<(), LocalRehydrationReplayError> {
    if value.trim().is_empty() || value.len() > max_bytes || value.as_bytes().contains(&0) {
        return Err(LocalRehydrationReplayError::Invalid(format!(
            "{label} must contain 1..={max_bytes} non-NUL bytes"
        )));
    }
    Ok(())
}

fn validate_digest(digest: &Sha256Digest) -> Result<(), LocalRehydrationReplayError> {
    Sha256Digest::parse(digest.as_str().to_string()).map_err(|_| {
        LocalRehydrationReplayError::Invalid(
            "replay identity must be a lowercase SHA-256 digest".to_string(),
        )
    })?;
    Ok(())
}

fn validate_runtime_plan_digests(
    plan: &LocalRehydrationRuntimePlan,
) -> Result<(), LocalRehydrationReplayError> {
    let host = &plan.host_read;
    let mut digests = vec![
        &plan.lease_id_sha256,
        &plan.fencing_token_sha256,
        &plan.binding_sha256,
        &host.turn_id_sha256,
        &host.journal_id_sha256,
        &host.operation_id_sha256,
        &host.checkpoint_sha256,
        &host.read_sha256,
        &host.binding_sha256,
        &host.read.checkpoint_sha256,
        &host.read.plan.summary_sha256,
    ];
    if let Some(witness) = host.read.witness.as_ref() {
        digests.push(&witness.checkpoint_sha256);
    }
    for digest in digests {
        Sha256Digest::parse(digest.as_str().to_string()).map_err(|_| {
            LocalRehydrationReplayError::Invalid(
                "H14 runtime identity must be a lowercase SHA-256 digest".to_string(),
            )
        })?;
    }
    Ok(())
}

fn hash_part(hasher: &mut Sha256, value: &[u8]) {
    hasher.update((value.len() as u64).to_be_bytes());
    hasher.update(value);
}

#[cfg(test)]
mod tests {
    use std::fs;

    use codex_extension_api::ExtensionData;
    use codex_hepta_contracts::AgentId;
    use codex_hepta_contracts::Sha256Digest;
    use codex_hepta_memory::CognitiveStore;
    use codex_hepta_memory::CompactCheckpoint;
    use codex_hepta_memory::CompactFence;
    use codex_hepta_memory::CompactLease;
    use codex_hepta_memory::CompactLossReport;
    use codex_hepta_memory::CompactParentSnapshot;
    use codex_hepta_memory::CompactSummaryReceipt;
    use codex_hepta_memory::RehydrationStatus;
    use codex_hepta_memory::checkpoint_digest;
    use codex_hepta_paths::HeptaFleetRoot;
    use tempfile::TempDir;

    use super::*;

    fn fence(token: &str) -> CompactFence {
        CompactFence::new(3, 4, 1, token).expect("fence")
    }

    fn checkpoint(fence: CompactFence) -> CompactCheckpoint {
        CompactCheckpoint::new(
            "ctxcp:h15",
            CompactLease::from_snapshot(
                CompactParentSnapshot::new(
                    "ctx:h15",
                    1,
                    2,
                    3,
                    Sha256Digest::for_bytes(b"state:h15"),
                    fence,
                )
                .expect("parent"),
            ),
            Vec::new(),
            CompactSummaryReceipt::new(
                Sha256Digest::for_bytes(b"summary:h15"),
                Sha256Digest::for_bytes(b"model:h15"),
                Sha256Digest::for_bytes(b"policy:h15"),
            ),
            CompactLossReport::new(Vec::new(), 0, Vec::new(), 0).expect("loss"),
            0,
        )
        .expect("checkpoint")
    }

    async fn opened_store(temp: &TempDir) -> CognitiveStore {
        let fleet_root = temp.path().join("fleet");
        fs::create_dir_all(&fleet_root).expect("fleet root");
        let fleet = HeptaFleetRoot::parse(fleet_root).expect("fleet");
        let owner = AgentId::parse("00000000-0000-4000-8000-000000000915").expect("owner");
        CognitiveStore::open(&fleet.layout().agent(&owner))
            .await
            .expect("store")
    }

    async fn prepared() -> (
        TempDir,
        ExtensionData,
        codex_hepta_memory::LocalLeaseOutbox,
        LocalCompactExecutor,
        CompactCheckpoint,
        CompactFence,
    ) {
        let temp = TempDir::new().expect("temp");
        let store = opened_store(&temp).await;
        let current_fence = fence("h15-fence");
        let checkpoint = checkpoint(current_fence.clone());
        let lease = match store
            .acquire_local_lease("lease:h15", 1, "h15-fence")
            .await
            .expect("lease")
        {
            codex_hepta_memory::LocalLeaseAcquire::Acquired(lease)
            | codex_hepta_memory::LocalLeaseAcquire::Replay(lease) => lease,
        };
        lease
            .admit(
                "occurrence:h15",
                "local.rehydration.replay.v1",
                "{\"external_effect\":false}",
            )
            .await
            .expect("local admission");
        let executor = store
            .open_local_compact_executor("journal:h15", current_fence.clone())
            .await
            .expect("executor");
        let current = checkpoint.lease.snapshot.clone();
        executor
            .append_intent("op:h15", &checkpoint, &current)
            .await
            .expect("intent");
        let digest = checkpoint_digest(&checkpoint).expect("digest");
        executor
            .commit_checkpoint("op:h15", &digest)
            .await
            .expect("commit");
        (
            temp,
            ExtensionData::new("turn:h15"),
            lease,
            executor,
            checkpoint,
            current_fence,
        )
    }

    #[tokio::test]
    async fn consumes_not_started_read_without_writes() {
        let (_temp, turn_store, lease, executor, checkpoint, current_fence) = prepared().await;
        let before_counts = lease.snapshot_counts().await.expect("before counts");
        let before_snapshot = executor.snapshot().await.expect("before snapshot");
        let replay = prepare_local_rehydration_replay(
            &turn_store,
            &lease,
            &executor,
            LocalRehydrationHostInput::new("turn:h15", "op:h15", &checkpoint, 0),
        )
        .await
        .expect("replay plan");
        assert_eq!(
            replay.disposition,
            LocalRehydrationReplayDisposition::NotStarted
        );
        assert!(!replay.witness_present);
        assert!(replay.is_local_read_only());
        assert_eq!(replay.generation, current_fence.generation);
        assert_eq!(
            lease.snapshot_counts().await.expect("after counts"),
            before_counts
        );
        assert_eq!(
            executor.snapshot().await.expect("after snapshot"),
            before_snapshot
        );
        assert!(turn_store.get::<LocalRehydrationReplayPlan>().is_none());
    }

    #[tokio::test]
    async fn consumes_complete_read_without_rehydrating_again() {
        let (_temp, turn_store, lease, executor, checkpoint, current_fence) = prepared().await;
        executor
            .rehydrate("op:h15", &checkpoint, 0)
            .await
            .expect("explicit witness");
        let before_counts = lease.snapshot_counts().await.expect("before counts");
        let before_snapshot = executor.snapshot().await.expect("before snapshot");
        let replay = prepare_local_rehydration_replay(
            &turn_store,
            &lease,
            &executor,
            LocalRehydrationHostInput::new("turn:h15", "op:h15", &checkpoint, 0),
        )
        .await
        .expect("replay plan");
        assert_eq!(
            replay.disposition,
            LocalRehydrationReplayDisposition::Complete
        );
        assert!(replay.witness_present);
        assert_eq!(replay.generation, current_fence.generation);
        assert_eq!(
            lease.snapshot_counts().await.expect("after counts"),
            before_counts
        );
        assert_eq!(
            executor.snapshot().await.expect("after snapshot"),
            before_snapshot
        );
    }

    #[tokio::test]
    async fn stale_fence_and_turn_binding_fail_closed() {
        let (_temp, turn_store, lease, executor, checkpoint, current_fence) = prepared().await;
        let runtime = prepare_local_rehydration_runtime(
            &turn_store,
            &lease,
            &executor,
            LocalRehydrationHostInput::new("turn:h15", "op:h15", &checkpoint, 0),
        )
        .await
        .expect("runtime plan");
        let stale = CompactFence::new(3, 4, 2, "next-fence").expect("stale fence");
        let error =
            consume_local_rehydration_runtime_plan(&runtime, "turn:h15", lease.lease_id(), &stale)
                .expect_err("stale fence must fail closed");
        assert!(matches!(
            error,
            LocalRehydrationReplayError::FenceMismatch(_)
        ));
        let error = consume_local_rehydration_runtime_plan(
            &runtime,
            "turn:other",
            lease.lease_id(),
            &current_fence,
        )
        .expect_err("turn mismatch must fail closed");
        assert!(matches!(
            error,
            LocalRehydrationReplayError::TurnBindingMismatch
        ));
    }

    #[tokio::test]
    async fn tampered_runtime_plan_and_negative_flags_fail_closed() {
        let (_temp, turn_store, lease, executor, checkpoint, current_fence) = prepared().await;
        let runtime = prepare_local_rehydration_runtime(
            &turn_store,
            &lease,
            &executor,
            LocalRehydrationHostInput::new("turn:h15", "op:h15", &checkpoint, 0),
        )
        .await
        .expect("runtime plan");
        let mut tampered = runtime.clone();
        tampered.namespace = "production".to_string();
        let error = consume_local_rehydration_runtime_plan(
            &tampered,
            "turn:h15",
            lease.lease_id(),
            &current_fence,
        )
        .expect_err("namespace tamper must fail closed");
        assert!(matches!(error, LocalRehydrationReplayError::Runtime(_)));

        let mut binding = runtime.clone();
        binding.binding_sha256 = Sha256Digest::for_bytes(b"tampered-binding");
        let error = consume_local_rehydration_runtime_plan(
            &binding,
            "turn:h15",
            lease.lease_id(),
            &current_fence,
        )
        .expect_err("binding tamper must fail closed");
        assert!(matches!(error, LocalRehydrationReplayError::Runtime(_)));

        let mut authority = runtime.clone();
        authority.provider_effects = true;
        let error = consume_local_rehydration_runtime_plan(
            &authority,
            "turn:h15",
            lease.lease_id(),
            &current_fence,
        )
        .expect_err("negative flag tamper must fail closed");
        assert!(matches!(error, LocalRehydrationReplayError::Runtime(_)));

        let replay = consume_local_rehydration_runtime_plan(
            &runtime,
            "turn:h15",
            lease.lease_id(),
            &current_fence,
        )
        .expect("valid replay");
        let mut replay_tampered = replay.clone();
        replay_tampered.replay_performed = true;
        assert!(replay_tampered.validate().is_err());
        assert_eq!(
            runtime.host_read.read.plan.status,
            RehydrationStatus::NotStarted
        );
    }
}
