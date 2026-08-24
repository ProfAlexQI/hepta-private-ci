//! Explicit app-server owner for the local-development witness seam.
//!
//! The owner is intentionally not an extension contributor.  It validates a
//! closed-world policy and exposes one host-invoked method; callers must hand
//! it the lease/checkpoint/executor handles they already own.  Constructing an
//! owner therefore does not register callbacks, create a scheduler, or add a
//! production caller.

use codex_hepta_memory::LocalDevelopmentLifecyclePolicy;
use codex_hepta_memory::LocalDevelopmentLifecyclePolicyError;
use codex_hepta_memory_extension::LocalRehydrationWitnessLifecycleError;
use codex_hepta_memory_extension::LocalRehydrationWitnessLifecycleInput;
use codex_hepta_memory_extension::LocalRehydrationWitnessLifecycleResult;

pub const HEPTA_LOCAL_LIFECYCLE_OWNER_RUNTIME_REGISTERED: bool = false;
pub const HEPTA_LOCAL_LIFECYCLE_OWNER_PRODUCTION_CALLER: bool = false;
pub const HEPTA_LOCAL_LIFECYCLE_OWNER_EXTERNAL_EFFECTS: bool = false;
pub const HEPTA_LOCAL_LIFECYCLE_OWNER_KG_WRITE_AUTHORITY: bool = false;

#[derive(Debug)]
pub enum HeptaLocalDevelopmentLifecycleOwnerError {
    Policy(LocalDevelopmentLifecyclePolicyError),
    Lifecycle(LocalRehydrationWitnessLifecycleError),
}

impl std::fmt::Display for HeptaLocalDevelopmentLifecycleOwnerError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Policy(error) => write!(formatter, "local lifecycle policy rejected: {error}"),
            Self::Lifecycle(error) => write!(formatter, "local lifecycle write failed: {error}"),
        }
    }
}

impl std::error::Error for HeptaLocalDevelopmentLifecycleOwnerError {}

impl From<LocalDevelopmentLifecyclePolicyError> for HeptaLocalDevelopmentLifecycleOwnerError {
    fn from(error: LocalDevelopmentLifecyclePolicyError) -> Self {
        Self::Policy(error)
    }
}

impl From<LocalRehydrationWitnessLifecycleError> for HeptaLocalDevelopmentLifecycleOwnerError {
    fn from(error: LocalRehydrationWitnessLifecycleError) -> Self {
        Self::Lifecycle(error)
    }
}

/// Host-owned, qualification-only lifecycle owner.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HeptaLocalDevelopmentLifecycleOwner {
    policy: LocalDevelopmentLifecyclePolicy,
}

impl HeptaLocalDevelopmentLifecycleOwner {
    pub fn new(
        policy: LocalDevelopmentLifecyclePolicy,
    ) -> Result<Self, HeptaLocalDevelopmentLifecycleOwnerError> {
        policy.validate()?;
        Ok(Self { policy })
    }

    pub fn qualification_only() -> Self {
        // The canonical constructor is statically known to satisfy the gate;
        // keep the checked constructor above for untrusted embedding input.
        Self::new(LocalDevelopmentLifecyclePolicy::qualification_only())
            .expect("canonical local-development policy must validate")
    }

    pub const fn policy(&self) -> LocalDevelopmentLifecyclePolicy {
        self.policy
    }

    pub const fn runtime_registered(&self) -> bool {
        HEPTA_LOCAL_LIFECYCLE_OWNER_RUNTIME_REGISTERED
    }

    pub const fn production_caller(&self) -> bool {
        HEPTA_LOCAL_LIFECYCLE_OWNER_PRODUCTION_CALLER
    }

    /// Invoke the extension seam exactly once for this host call.
    ///
    /// The input's policy is replaced with the owner's validated policy so a
    /// host cannot accidentally downgrade the gate between owner creation and
    /// the write.  No callback or background task is installed here.
    pub async fn write_local_rehydration_witness(
        &self,
        mut input: LocalRehydrationWitnessLifecycleInput<'_>,
    ) -> Result<LocalRehydrationWitnessLifecycleResult, HeptaLocalDevelopmentLifecycleOwnerError>
    {
        self.policy.validate()?;
        input.policy = self.policy;
        Ok(
            codex_hepta_memory_extension::write_local_rehydration_witness_at_lifecycle(input)
                .await?,
        )
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::SystemTime;
    use std::time::UNIX_EPOCH;

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
    use codex_hepta_memory::LocalLeaseAcquire;
    use codex_hepta_memory::LocalRehydrationWitnessWrite;
    use codex_hepta_memory::checkpoint_digest;
    use codex_hepta_memory_extension::LocalRehydrationReplayDisposition;
    use codex_hepta_memory_extension::LocalRehydrationWitnessLifecycleInput;
    use codex_hepta_paths::HeptaFleetRoot;
    use tempfile::TempDir;

    use super::*;

    fn fence() -> CompactFence {
        CompactFence::new(3, 4, 1, "e19-owner-fence").expect("fence")
    }

    fn checkpoint(fence: CompactFence) -> CompactCheckpoint {
        CompactCheckpoint::new(
            "checkpoint:e19-owner",
            CompactLease::from_snapshot(
                CompactParentSnapshot::new(
                    "ctx:e19-owner",
                    1,
                    2,
                    3,
                    Sha256Digest::for_bytes(b"state:e19-owner"),
                    fence,
                )
                .expect("parent snapshot"),
            ),
            Vec::new(),
            CompactSummaryReceipt::new(
                Sha256Digest::for_bytes(b"summary:e19-owner"),
                Sha256Digest::for_bytes(b"model:e19-owner"),
                Sha256Digest::for_bytes(b"policy:e19-owner"),
            ),
            CompactLossReport::new(Vec::new(), 0, Vec::new(), 0).expect("loss report"),
            0,
        )
        .expect("checkpoint")
    }

    async fn prepared_owner_inputs() -> (
        TempDir,
        CognitiveStore,
        ExtensionData,
        codex_hepta_memory::LocalLeaseOutbox,
        codex_hepta_memory::LocalCompactExecutor,
        CompactCheckpoint,
    ) {
        let temp = TempDir::new().expect("temp");
        let fleet_root = temp.path().join("fleet");
        fs::create_dir_all(&fleet_root).expect("fleet root");
        let fleet = HeptaFleetRoot::parse(fleet_root).expect("fleet");
        let owner_id = AgentId::parse("00000000-0000-4000-8000-000000000919").expect("owner id");
        let store = CognitiveStore::open(&fleet.layout().agent(&owner_id))
            .await
            .expect("store");
        let current_fence = fence();
        let checkpoint = checkpoint(current_fence.clone());
        let expires_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_secs()
            + 3_600;
        let lease = match store
            .acquire_local_lease_bound(
                "lease:e19-owner",
                current_fence.authority_epoch,
                current_fence.owner_epoch,
                current_fence.generation,
                current_fence.fencing_token.clone(),
                expires_at,
            )
            .await
            .expect("bound lease")
        {
            LocalLeaseAcquire::Acquired(lease) | LocalLeaseAcquire::Replay(lease) => lease,
        };
        lease
            .admit(
                "occurrence:e19-owner",
                "local.rehydration.witness.v1",
                "{\"external_effect\":false}",
            )
            .await
            .expect("local admission");
        let executor = store
            .open_local_compact_executor_bound("journal:e19-owner", current_fence, &lease)
            .await
            .expect("bound executor");
        let current = checkpoint.lease.snapshot.clone();
        executor
            .append_intent("operation:e19-owner", &checkpoint, &current)
            .await
            .expect("intent");
        let digest = checkpoint_digest(&checkpoint).expect("checkpoint digest");
        executor
            .commit_checkpoint("operation:e19-owner", &digest)
            .await
            .expect("commit");
        (
            temp,
            store,
            ExtensionData::new("turn:e19-owner"),
            lease,
            executor,
            checkpoint,
        )
    }

    #[test]
    fn owner_is_explicit_and_caller_zero() {
        let owner = HeptaLocalDevelopmentLifecycleOwner::qualification_only();
        assert!(owner.policy().permits_explicit_witness_write());
        assert!(!owner.runtime_registered());
        assert!(!owner.production_caller());
        assert!(!HEPTA_LOCAL_LIFECYCLE_OWNER_EXTERNAL_EFFECTS);
        assert!(!HEPTA_LOCAL_LIFECYCLE_OWNER_KG_WRITE_AUTHORITY);
    }

    #[test]
    fn owner_rejects_production_policy() {
        let mut policy = LocalDevelopmentLifecyclePolicy::qualification_only();
        policy.production_activation = true;
        assert!(matches!(
            HeptaLocalDevelopmentLifecycleOwner::new(policy),
            Err(HeptaLocalDevelopmentLifecycleOwnerError::Policy(_))
        ));
    }

    #[tokio::test]
    async fn explicit_owner_writes_bound_witness_and_replays_without_registration() {
        let (_temp, store, turn_store, lease, executor, checkpoint) = prepared_owner_inputs().await;
        let owner = HeptaLocalDevelopmentLifecycleOwner::qualification_only();
        let input = || {
            LocalRehydrationWitnessLifecycleInput::new(
                "turn:e19-owner",
                &turn_store,
                "operation:e19-owner",
                0,
                &checkpoint,
                &lease,
                &executor,
            )
        };

        let first = owner
            .write_local_rehydration_witness(input())
            .await
            .expect("first explicit owner write");
        assert_eq!(
            first.observed_disposition,
            LocalRehydrationReplayDisposition::NotStarted
        );
        assert!(!first.witness.replayed);
        assert!(!first.external_effects);
        assert!(!first.kg_write_authority);
        assert!(!first.production_caller);
        assert!(!first.lifecycle_registered);
        assert!(first.policy_gate_bound);

        let second = owner
            .write_local_rehydration_witness(input())
            .await
            .expect("replayed explicit owner write");
        assert_eq!(
            second.observed_disposition,
            LocalRehydrationReplayDisposition::Complete
        );
        assert!(second.witness.replayed);
        assert_eq!(
            second.witness.witness_sequence,
            first.witness.witness_sequence
        );
        assert!(matches!(
            executor
                .rehydration("operation:e19-owner")
                .await
                .expect("witness lookup"),
            Some(_)
        ));
        let snapshot = executor.snapshot().await.expect("compact snapshot");
        assert_eq!(snapshot.entries.len(), 3, "one rehydration witness row");
        assert!(matches!(
            snapshot.entries.last().map(|entry| &entry.kind),
            Some(codex_hepta_memory::CompactPersistenceEventKind::Rehydrated { .. })
        ));
        assert!(matches!(
            codex_hepta_memory::write_local_rehydration_witness(
                &lease,
                &executor,
                "operation:e19-owner",
                &checkpoint,
                0,
            )
            .await
            .expect("core replay"),
            LocalRehydrationWitnessWrite::Replay(_)
        ));
        drop(store);
    }
}
