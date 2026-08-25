//! Agentd-owned qualification turn writer capability.
//!
//! The positive path exists only in the explicit
//! `qualification-cognitive-write` build.  It binds each durable Codex turn
//! identity to one Agent-local lease and compact executor, and records local
//! lifecycle metadata only.  The epochs below are process-local
//! qualification witnesses; they are not production supervisor authority.

use std::sync::Arc;

use codex_hepta_memory::CognitiveRuntime;
use codex_hepta_memory_extension::QualificationTurnWriterHost;

use crate::AgentdIdentity;
use crate::AgentdState;

#[cfg(feature = "qualification-cognitive-write")]
const CAPABILITY_ID: &str = "hepta-agentd:qualification-turn-writer:v1";

pub(crate) fn qualification_turn_writer_host(
    identity: &AgentdIdentity,
    state: Arc<AgentdState>,
    runtime: &CognitiveRuntime,
) -> Option<QualificationTurnWriterHost> {
    #[cfg(not(feature = "qualification-cognitive-write"))]
    {
        let _ = (identity, state, runtime);
        None
    }

    #[cfg(feature = "qualification-cognitive-write")]
    {
        let store = Arc::clone(runtime.available_store()?);
        if store.owner_agent_id() != &identity.agent_id
            || state.identity().agent_id != identity.agent_id
            || state.identity().spawn_generation != identity.spawn_generation
        {
            return None;
        }
        let agent_id = identity.agent_id.clone();
        let spawn_generation = identity.spawn_generation;
        Some(QualificationTurnWriterHost::from_fn(
            CAPABILITY_ID,
            move |turn_id| {
                let state = Arc::clone(&state);
                let store = Arc::clone(&store);
                let agent_id = agent_id.clone();
                async move {
                    prepare_qualification_turn_writer_input(
                        state,
                        store,
                        agent_id,
                        spawn_generation,
                        turn_id,
                    )
                    .await
                }
            },
        ))
    }
}

#[cfg(feature = "qualification-cognitive-write")]
pub(crate) async fn prepare_qualification_turn_writer_input(
    state: Arc<AgentdState>,
    store: Arc<codex_hepta_memory::CognitiveStore>,
    agent_id: codex_hepta_contracts::AgentId,
    spawn_generation: u64,
    turn_id: String,
) -> Result<
    codex_hepta_memory_extension::QualificationTurnWriterInput,
    codex_hepta_memory_extension::QualificationTurnWriterInputError,
> {
    use std::time::SystemTime;
    use std::time::UNIX_EPOCH;

    use codex_hepta_contracts::Sha256Digest;
    use codex_hepta_memory::CompactFence;
    use codex_hepta_memory::LocalLeaseHeadDisposition;
    use codex_hepta_memory::LocalTurnLifecycleBinding;
    use codex_hepta_memory_extension::QualificationTurnWriterInput;
    use codex_hepta_memory_extension::QualificationTurnWriterInputError;

    const LOCAL_QUALIFICATION_AUTHORITY_EPOCH: u64 = 1;
    const LEASE_TTL_SECONDS: u64 = 3_600;

    fn fenced() -> QualificationTurnWriterInputError {
        QualificationTurnWriterInputError::Invalid(
            "Agentd generation fence rejected the qualification turn writer".to_string(),
        )
    }

    let fleet_generation = state.qualification_turn_authority().map_err(|_| fenced())?;
    if state.identity().spawn_generation != spawn_generation
        || state.identity().agent_id != agent_id
        || store.owner_agent_id() != &agent_id
    {
        return Err(fenced());
    }

    // LocalLeaseOutbox::generation is the append-only generation of this
    // lease, not the Agent/fleet lifecycle generation. Each turn receives a
    // fresh lease id, so its first local generation is exactly one. The fleet
    // generation is carried by the explicit owner epoch below.
    const LOCAL_LEASE_GENERATION: u64 = 1;
    let fencing_token = Sha256Digest::for_bytes(
        format!(
            "hepta-agentd:qualification-turn-fence:v1:{}:{fleet_generation}:{spawn_generation}:{turn_id}",
            agent_id.as_str()
        )
        .as_bytes(),
    )
    .as_str()
    .to_string();

    let expires_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| {
            QualificationTurnWriterInputError::Invalid(
                "system clock is before the Unix epoch".to_string(),
            )
        })?
        .as_secs()
        .checked_add(LEASE_TTL_SECONDS)
        .ok_or_else(|| {
            QualificationTurnWriterInputError::Invalid(
                "qualification lease expiry overflowed".to_string(),
            )
        })?;
    let authority_epoch = LOCAL_QUALIFICATION_AUTHORITY_EPOCH;
    let owner_epoch = fleet_generation;
    let lease_id = format!("qualification-turn:{spawn_generation}:{turn_id}");
    let journal_id = format!("qualification-turn-journal:{spawn_generation}:{turn_id}");
    let occurrence_key = format!("qualification-turn-start:{spawn_generation}:{turn_id}");

    // A repeated prepare for one same-spawn turn must inspect the durable
    // head before attempting acquisition.  In particular, the TTL is a
    // binding field: recomputing `expires_at` and passing it to an exact
    // replay would reject a perfectly valid active head after the wall clock
    // crossed a second.  Missing is the only state that may mint the first
    // lease row.  Every other state must use the exact persisted witness or
    // fail closed.
    let inspected = store.inspect_local_lease_head(&lease_id).await?;
    let (lease, authority_epoch, owner_epoch, lease_generation, fencing_token) = match inspected
        .disposition
    {
        LocalLeaseHeadDisposition::Missing => {
            let lease = store
                .acquire_host_bound_lease(
                    lease_id.clone(),
                    authority_epoch,
                    owner_epoch,
                    LOCAL_LEASE_GENERATION,
                    fencing_token.clone(),
                    expires_at,
                )
                .await?
                .into_handle();
            (
                lease,
                authority_epoch,
                owner_epoch,
                LOCAL_LEASE_GENERATION,
                fencing_token,
            )
        }
        LocalLeaseHeadDisposition::Active | LocalLeaseHeadDisposition::ExpiredActive => {
            let head = inspected.head.ok_or_else(fenced)?;
            let persisted_authority_epoch = head.authority_epoch.ok_or_else(fenced)?;
            let persisted_owner_epoch = head.owner_epoch.ok_or_else(fenced)?;
            let persisted_generation = head.generation;
            let persisted_fencing_token = head.fencing_token.clone();
            let persisted_expiry = head.lease_expires_at_unix_seconds.ok_or_else(fenced)?;

            // The lease id is spawn-generation scoped by design.  A
            // matching id with a different generation/token/authority is
            // not an invitation to adopt that attempt; it is stale or
            // foreign state and must remain fenced.
            if head.owner_agent_id != agent_id
                || head.generation != LOCAL_LEASE_GENERATION
                || head.fencing_token != fencing_token
                || persisted_authority_epoch != authority_epoch
                || persisted_owner_epoch != owner_epoch
            {
                return Err(fenced());
            }

            if inspected.disposition == LocalLeaseHeadDisposition::ExpiredActive {
                // An expired attempt may be closed only after the
                // restart-safe, head-scoped H7 gate proves a complete
                // local `turn_start -> terminal` chain.  Missing or
                // non-terminal evidence remains untouched for explicit
                // operator/audit handling; never blind-rollback it here.
                let witness = codex_hepta_memory::inspect_expired_terminal_h7(
                    &store,
                    &head,
                    &journal_id,
                    format!("qualification:trajectory:{turn_id}"),
                    &turn_id,
                    &occurrence_key,
                )
                .await
                .map_err(|error| QualificationTurnWriterInputError::Invalid(error.to_string()))?;
                if witness.is_none() {
                    return Err(fenced());
                }
                let lease = store
                    .reopen_host_bound_lease(
                        head,
                        persisted_authority_epoch,
                        persisted_owner_epoch,
                        persisted_expiry,
                    )
                    .await?;
                lease.expire_lease().await?;
                return Err(QualificationTurnWriterInputError::Invalid(
                    "qualification lease terminalized after verified H7 terminal".to_string(),
                ));
            }

            let lease = store
                .reopen_host_bound_lease(
                    head,
                    persisted_authority_epoch,
                    persisted_owner_epoch,
                    persisted_expiry,
                )
                .await?;

            (
                lease,
                persisted_authority_epoch,
                persisted_owner_epoch,
                persisted_generation,
                persisted_fencing_token,
            )
        }
        LocalLeaseHeadDisposition::Released | LocalLeaseHeadDisposition::RolledBack => {
            return Err(fenced());
        }
    };

    let fence = CompactFence::new(
        authority_epoch,
        owner_epoch,
        lease_generation,
        fencing_token.clone(),
    )
    .map_err(|error| QualificationTurnWriterInputError::Invalid(error.to_string()))?;
    let executor = match store
        .open_local_compact_executor_bound(journal_id, fence, &lease)
        .await
    {
        Ok(executor) => executor,
        Err(error) => {
            let _ = lease.release().await;
            return Err(error.into());
        }
    };
    let current_fleet_generation = match state.qualification_turn_authority() {
        Ok(generation) => generation,
        Err(_) => {
            let _ = lease.release().await;
            return Err(fenced());
        }
    };
    if current_fleet_generation != fleet_generation {
        let _ = lease.release().await;
        return Err(fenced());
    }
    let binding = match LocalTurnLifecycleBinding::from_handles(&turn_id, &lease, &executor) {
        Ok(binding) => binding,
        Err(error) => {
            let _ = lease.release().await;
            return Err(error.into());
        }
    };
    let payload_json = serde_json::json!({
        "schema_version": 1,
        "turn_id_sha256": binding.turn_id_sha256.as_str(),
        "fleet_generation": fleet_generation,
        "spawn_generation": spawn_generation,
        "external_effect": false,
        "kg_write_authority": false,
        "production_caller": false
    })
    .to_string();
    match QualificationTurnWriterInput::new(
        turn_id,
        binding,
        lease.clone(),
        executor,
        occurrence_key,
        payload_json,
    ) {
        Ok(input) => Ok(input),
        Err(error) => {
            let _ = lease.release().await;
            Err(error)
        }
    }
}
