use pretty_assertions::assert_eq;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use tempfile::TempDir;

use crate::cognitive_test_support::agent_id;
use crate::cognitive_test_support::layout;
use crate::CognitiveStore;
use crate::LocalAdmission;
use crate::LocalAdmissionFault;
use crate::LocalLeaseAcquire;
use crate::LocalLeaseHeadDisposition;
use crate::LocalLeaseOutboxError;
use crate::LocalLeaseState;
use crate::LocalOutcomeState;
use crate::LocalReconcileOutcome;
use crate::LocalReplayFinalization;
use crate::LOCAL_LEASE_OUTBOX_EXTERNAL_EFFECTS;
use crate::LOCAL_LEASE_OUTBOX_KG_WRITE_AUTHORITY;
use crate::LOCAL_LEASE_OUTBOX_PRODUCTION_CALLER;
use codex_hepta_contracts::Sha256Digest;

async fn opened_store(temp: &TempDir, number: u8) -> CognitiveStore {
    let owner = agent_id(number);
    CognitiveStore::open(&layout(temp, &owner))
        .await
        .expect("cognitive store")
}

fn acquired(value: LocalLeaseAcquire) -> crate::LocalLeaseOutbox {
    match value {
        LocalLeaseAcquire::Acquired(handle) | LocalLeaseAcquire::Replay(handle) => handle,
    }
}

fn unix_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_secs()
}

#[tokio::test]
async fn inspect_local_lease_head_is_read_only_and_classifies_fences() {
    let temp = TempDir::new().expect("temp dir");
    let store = opened_store(&temp, 100).await;
    let missing = store
        .inspect_local_lease_head("lease:missing")
        .await
        .expect("missing inspection");
    assert_eq!(missing.disposition, LocalLeaseHeadDisposition::Missing);
    assert!(missing.head.is_none());

    let active = acquired(
        store
            .acquire_local_lease("lease:inspect-active", 1, "fence:inspect-active")
            .await
            .expect("active acquire"),
    );
    let before = active
        .snapshot_counts()
        .await
        .expect("counts before inspect");
    let active_read = store
        .inspect_local_lease_head("lease:inspect-active")
        .await
        .expect("active inspection");
    assert_eq!(active_read.disposition, LocalLeaseHeadDisposition::Active);
    let active_head = active_read.head.as_ref().expect("active head witness");
    assert_eq!(active_head.lease_id, "lease:inspect-active");
    assert_eq!(active_head.generation, 1);
    assert_eq!(active_head.fencing_token, "fence:inspect-active");
    assert_eq!(active_head.state, LocalLeaseState::Active);
    assert_eq!(
        active.snapshot_counts().await.expect("counts after inspect"),
        before,
        "inspection must not append lease/event/outbox rows"
    );
    active.release().await.expect("release active");
    let released = store
        .inspect_local_lease_head("lease:inspect-active")
        .await
        .expect("released inspection");
    assert_eq!(released.disposition, LocalLeaseHeadDisposition::Released);

    let expired = acquired(
        store
            .acquire_host_bound_lease(
                "lease:inspect-expired",
                1,
                1,
                1,
                "fence:inspect-expired",
                1,
            )
            .await
            .expect("expired active acquire"),
    );
    let expired_read = store
        .inspect_local_lease_head("lease:inspect-expired")
        .await
        .expect("expired inspection");
    assert_eq!(
        expired_read.disposition,
        LocalLeaseHeadDisposition::ExpiredActive
    );
    expired
        .expire_lease_at_unix_seconds(2)
        .await
        .expect("expire active");
    let rolled_back = store
        .inspect_local_lease_head("lease:inspect-expired")
        .await
        .expect("rolled back inspection");
    assert_eq!(
        rolled_back.disposition,
        LocalLeaseHeadDisposition::RolledBack
    );
}

#[tokio::test]
async fn acquire_replay_and_atomic_admission_are_idempotent() {
    let temp = TempDir::new().expect("temp dir");
    let store = opened_store(&temp, 101).await;
    let first = store
        .acquire_local_lease("lease:admission", 1, "fence:1")
        .await
        .expect("acquire");
    assert!(matches!(first, LocalLeaseAcquire::Acquired(_)));
    let replay = store
        .acquire_local_lease("lease:admission", 1, "fence:1")
        .await
        .expect("replay acquire");
    assert!(matches!(replay, LocalLeaseAcquire::Replay(_)));
    let handle = acquired(first);
    let queued = handle
        .admit("occurrence:1", "local.topic", "{\"value\":1}")
        .await
        .expect("admit");
    let replayed = handle
        .admit("occurrence:1", "local.topic", "{\"value\":1}")
        .await
        .expect("replay admit");
    assert!(matches!(queued, LocalAdmission::Queued(_)));
    assert!(matches!(replayed, LocalAdmission::Replay(_)));
    let counts = handle.snapshot_counts().await.expect("counts");
    assert_eq!(counts.lease_rows, 1);
    assert_eq!(counts.event_rows, 1);
    assert_eq!(counts.outbox_rows, 1);
}

#[tokio::test]
async fn generation_cas_release_and_stale_handle_fail_closed() {
    let temp = TempDir::new().expect("temp dir");
    let store = opened_store(&temp, 102).await;
    let old = acquired(
        store
            .acquire_local_lease("lease:generation", 1, "fence:1")
            .await
            .expect("acquire"),
    );
    assert!(matches!(
        store
            .acquire_local_lease_after("lease:generation", 1, 2, "fence:2")
            .await,
        Err(LocalLeaseOutboxError::CasConflict(message))
            if message.contains("exact lease head")
    ));
    let released = old.release().await.expect("release");
    assert!(store
        .acquire_local_lease_after("lease:generation", 1, 2, "fence:1")
        .await
        .is_err());
    assert!(matches!(
        store
            .acquire_local_lease_after("lease:generation", 1, 2, "fence:2")
            .await,
        Err(LocalLeaseOutboxError::CasConflict(message))
            if message.contains("exact lease head")
    ));
    let next = acquired(
        store
            .acquire_local_lease_after_head("lease:generation", released, 2, "fence:2")
            .await
            .expect("next generation"),
    );
    assert!(old
        .admit("occurrence:stale", "topic", "payload")
        .await
        .is_err());
    assert!(next
        .admit("occurrence:current", "topic", "payload")
        .await
        .is_ok());
    assert!(store
        .acquire_local_lease_after("lease:generation", 1, 3, "fence:3")
        .await
        .is_err());
}

#[tokio::test]
async fn bound_expiry_is_explicit_timeout_rollback_and_exact_head_reopens_generation() {
    let temp = TempDir::new().expect("temp dir");
    let store = opened_store(&temp, 113).await;
    let expires_at = unix_seconds() + 3_600;
    let old = acquired(
        store
            .acquire_local_lease_bound(
                "lease:expiry-terminal",
                3,
                8,
                1,
                "fence:expiry-1",
                expires_at,
            )
            .await
            .expect("bound acquire"),
    );
    old.admit("occurrence:before-expiry", "topic", "payload")
        .await
        .expect("admit before expiry");

    assert!(matches!(
        old.expire_lease_at_unix_seconds(expires_at - 1).await,
        Err(LocalLeaseOutboxError::StaleFence(message))
            if message.contains("has not expired")
    ));

    // Deadline expiry makes ordinary host transitions stale; only the
    // explicit timeout operation may close a bound lease.  Use a separate
    // already-expired head so this assertion is independent of wall-clock
    // scheduling while the admitted lease below still exercises its child
    // journals during terminalization.
    let expired_host_transition = acquired(
        store
            .acquire_local_lease_bound(
                "lease:expired-host-transition",
                3,
                8,
                1,
                "fence:expired-host-transition",
                unix_seconds().saturating_sub(1),
            )
            .await
            .expect("already-expired bound acquire"),
    );
    assert!(matches!(
        expired_host_transition.release().await,
        Err(LocalLeaseOutboxError::StaleFence(message))
            if message.contains("has expired")
    ));
    assert!(matches!(
        expired_host_transition.rollback_lease().await,
        Err(LocalLeaseOutboxError::StaleFence(message))
            if message.contains("has expired")
    ));

    let expired = old
        .expire_lease_at_unix_seconds(expires_at)
        .await
        .expect("explicit expiry rollback");
    assert_eq!(expired.state, crate::LocalLeaseState::RolledBack);
    assert_eq!(expired.lease_sequence, 2);
    assert_eq!(expired.generation, 1);
    assert_eq!(expired.fencing_token, "fence:expiry-1");
    assert_eq!(expired.authority_epoch, Some(3));
    assert_eq!(expired.owner_epoch, Some(8));
    assert_eq!(expired.lease_expires_at_unix_seconds, Some(expires_at));

    // A host retry after a committed timeout is a replay, not another
    // terminal append.  The historical event/outbox fence remains valid.
    let replay = old
        .expire_lease_at_unix_seconds(expires_at)
        .await
        .expect("expiry replay");
    assert_eq!(replay, expired);
    assert_eq!(
        old.snapshot_counts()
            .await
            .expect("post-expiry counts")
            .lease_rows,
        2
    );

    // The pre-expiry writer remains fenced after timeout terminalization.
    assert!(matches!(
        old.admit("occurrence:stale-after-expiry", "topic", "payload")
            .await,
        Err(LocalLeaseOutboxError::StaleFence(_))
    ));

    let next_expires_at = unix_seconds() + 3_600;
    let next = acquired(
        store
            .acquire_local_lease_after_head_bound(
                "lease:expiry-terminal",
                expired.clone(),
                3,
                8,
                2,
                "fence:expiry-2",
                next_expires_at,
            )
            .await
            .expect("exact-head next generation"),
    );
    assert_eq!(next.generation(), 2);
    assert_eq!(next.fencing_token(), "fence:expiry-2");
    assert_eq!(
        next.binding()
            .expect("next bound lease")
            .lease_expires_at_unix_seconds,
        next_expires_at
    );
    assert!(next
        .admit("occurrence:next-generation", "topic", "payload")
        .await
        .is_ok());
}

#[tokio::test]
async fn verify_current_rejects_expired_active_head_but_reopen_allows_explicit_expiry() {
    let temp = TempDir::new().expect("temp dir");
    let store = opened_store(&temp, 116).await;
    let expires_at = unix_seconds().saturating_sub(1);
    let lease = acquired(
        store
            .acquire_local_lease_bound(
                "lease:verify-expired",
                3,
                8,
                1,
                "fence:verify-expired",
                expires_at,
            )
            .await
            .expect("bound acquire"),
    );

    assert!(matches!(
        lease.verify_current().await,
        Err(LocalLeaseOutboxError::StaleFence(message))
            if message.contains("has expired")
    ));

    // Restart recovery intentionally remains available: a host can reopen
    // the expired active head and make the explicit timeout decision itself.
    let reopened = store
        .reopen_local_lease("lease:verify-expired", 1, "fence:verify-expired")
        .await
        .expect("reopen expired head for explicit expiry");
    let terminal = reopened
        .expire_lease()
        .await
        .expect("explicit expiry after reopen");
    assert_eq!(terminal.state, crate::LocalLeaseState::RolledBack);
    assert_eq!(terminal.lease_sequence, 2);
}

#[tokio::test]
async fn expiry_rejects_legacy_unbound_leases() {
    let temp = TempDir::new().expect("temp dir");
    let store = opened_store(&temp, 114).await;
    let legacy = acquired(
        store
            .acquire_local_lease("lease:unbound-expiry", 1, "fence:legacy")
            .await
            .expect("legacy acquire"),
    );
    assert!(matches!(
        legacy.expire_lease().await,
        Err(LocalLeaseOutboxError::Invalid(message))
            if message.contains("explicit authority/owner/expiry binding")
    ));
}

#[tokio::test]
async fn host_bound_reopen_requires_exact_head_and_binding() {
    let temp = TempDir::new().expect("temp dir");
    let store = opened_store(&temp, 141).await;
    let expires_at = unix_seconds() + 3_600;
    let lease = acquired(
        store
            .acquire_host_bound_lease(
                "lease:host-reopen",
                10,
                20,
                1,
                "fence:host-reopen-1",
                expires_at,
            )
            .await
            .expect("host-bound acquire"),
    );
    let head = lease.head_witness().await.expect("active head witness");
    assert_eq!(head.state, crate::LocalLeaseState::Active);
    assert_eq!(head.authority_epoch, Some(10));
    assert_eq!(head.owner_epoch, Some(20));

    let reopened = store
        .reopen_host_bound_lease(head.clone(), 10, 20, expires_at)
        .await
        .expect("exact host-bound reopen");
    assert_eq!(reopened.head_witness().await.expect("reopened head"), head);

    let before = lease
        .snapshot_counts()
        .await
        .expect("counts before rejects");
    assert!(matches!(
        store
            .reopen_host_bound_lease(head.clone(), 10, 21, expires_at)
            .await,
        Err(LocalLeaseOutboxError::StaleFence(message))
            if message.contains("binding")
    ));

    let mut tampered_head = head.clone();
    tampered_head.lease_sha256 = Sha256Digest::for_bytes(b"tampered-host-head");
    assert!(matches!(
        store
            .reopen_host_bound_lease(tampered_head, 10, 20, expires_at)
            .await,
        Err(LocalLeaseOutboxError::StaleFence(message))
            if message.contains("no longer matches")
    ));
    assert_eq!(
        lease.snapshot_counts().await.expect("counts after rejects"),
        before,
        "reopen rejects must never append lease/event/outbox rows"
    );

    let legacy = acquired(
        store
            .acquire_local_lease("lease:host-reopen-legacy", 1, "fence:legacy")
            .await
            .expect("legacy acquire"),
    );
    let legacy_head = legacy.head_witness().await.expect("legacy head witness");
    assert!(matches!(
        store
            .reopen_host_bound_lease(legacy_head, 1, 1, expires_at)
            .await,
        Err(LocalLeaseOutboxError::StaleFence(message))
            if message.contains("binding")
    ));
}

#[tokio::test]
async fn host_bound_successor_cas_requires_lexicographically_newer_epochs() {
    let temp = TempDir::new().expect("temp dir");
    let store = opened_store(&temp, 142).await;
    let first = acquired(
        store
            .acquire_host_bound_lease(
                "lease:host-epochs",
                4,
                8,
                1,
                "fence:host-epochs-1",
                unix_seconds() + 3_600,
            )
            .await
            .expect("first host-bound acquire"),
    );
    let terminal = first.release().await.expect("terminal head");

    let before = first
        .snapshot_counts()
        .await
        .expect("counts before epoch rejects");
    assert!(matches!(
        store
            .acquire_host_bound_lease_after_head(
                "lease:host-epochs",
                terminal.clone(),
                4,
                8,
                2,
                "fence:host-epochs-replay",
                unix_seconds() + 3_600,
            )
            .await,
        Err(LocalLeaseOutboxError::CasConflict(message))
            if message.contains("epoch must advance")
    ));
    assert!(matches!(
        store
            .acquire_host_bound_lease_after_head(
                "lease:host-epochs",
                terminal.clone(),
                3,
                99,
                2,
                "fence:host-epochs-regressed-authority",
                unix_seconds() + 3_600,
            )
            .await,
        Err(LocalLeaseOutboxError::CasConflict(message))
            if message.contains("epoch must advance")
    ));
    assert_eq!(
        first
            .snapshot_counts()
            .await
            .expect("counts after epoch rejects"),
        before
    );

    let next = acquired(
        store
            .acquire_host_bound_lease_after_head(
                "lease:host-epochs",
                terminal.clone(),
                4,
                9,
                2,
                "fence:host-epochs-2",
                unix_seconds() + 3_600,
            )
            .await
            .expect("strict owner epoch successor"),
    );
    assert_eq!(next.generation(), 2);
    assert_eq!(next.binding().expect("next binding").owner_epoch, 9);
    assert!(
        matches!(
            first.head_witness().await,
            Err(LocalLeaseOutboxError::StaleFence(_))
        ),
        "a stale handle cannot witness a newer generation"
    );

    let stale_counts = next
        .snapshot_counts()
        .await
        .expect("counts before stale CAS");
    let stale_result = store
        .acquire_host_bound_lease_after_head(
            "lease:host-epochs",
            terminal,
            5,
            1,
            2,
            "fence:host-epochs-stale-head",
            unix_seconds() + 3_600,
        )
        .await;
    assert!(
        matches!(
            stale_result,
            Err(LocalLeaseOutboxError::CasConflict(ref message))
                if message.contains("no longer matches")
        ),
        "unexpected stale CAS result: {stale_result:?}"
    );
    assert_eq!(
        next.snapshot_counts()
            .await
            .expect("counts after stale CAS"),
        stale_counts
    );

    let next_terminal = next.release().await.expect("second terminal head");
    let authority_transfer = acquired(
        store
            .acquire_host_bound_lease_after_head(
                "lease:host-epochs",
                next_terminal,
                5,
                1,
                3,
                "fence:host-epochs-3",
                unix_seconds() + 3_600,
            )
            .await
            .expect("higher authority epoch permits explicit owner reset"),
    );
    assert_eq!(
        authority_transfer
            .binding()
            .expect("authority transfer binding")
            .owner_epoch,
        1
    );
}

#[tokio::test]
async fn terminal_or_indeterminate_occurrence_never_replays_as_queued() {
    let temp = TempDir::new().expect("temp dir");
    let store = opened_store(&temp, 106).await;
    let handle = acquired(
        store
            .acquire_local_lease("lease:terminal-replay", 1, "fence:1")
            .await
            .expect("acquire"),
    );

    for (occurrence, outcome) in [
        ("occurrence:committed", LocalReconcileOutcome::Committed),
        ("occurrence:rejected", LocalReconcileOutcome::Rejected),
    ] {
        handle
            .admit(occurrence, "topic", "payload")
            .await
            .expect("admit");
        handle
            .mark_indeterminate(occurrence, "lost local ack")
            .await
            .expect("mark indeterminate");
        handle
            .reconcile(occurrence, outcome)
            .await
            .expect("reconcile");
        assert!(matches!(
            handle.admit(occurrence, "topic", "payload").await,
            Err(LocalLeaseOutboxError::IllegalTransition(_))
        ));
    }

    handle
        .admit("occurrence:rolled-back", "topic", "payload")
        .await
        .expect("admit");
    handle
        .rollback_occurrence("occurrence:rolled-back", "operator rollback")
        .await
        .expect("rollback");
    assert!(matches!(
        handle
            .admit("occurrence:rolled-back", "topic", "payload")
            .await,
        Err(LocalLeaseOutboxError::IllegalTransition(_))
    ));

    handle
        .admit("occurrence:indeterminate", "topic", "payload")
        .await
        .expect("admit");
    handle
        .mark_indeterminate("occurrence:indeterminate", "lost local ack")
        .await
        .expect("mark indeterminate");
    assert!(matches!(
        handle
            .admit("occurrence:indeterminate", "topic", "payload")
            .await,
        Err(LocalLeaseOutboxError::IllegalTransition(_))
    ));
}

#[tokio::test]
async fn new_generation_retry_of_old_occurrence_is_stale_not_corrupt() {
    let temp = TempDir::new().expect("temp dir");
    let store = opened_store(&temp, 107).await;
    let old = acquired(
        store
            .acquire_local_lease("lease:cross-generation", 1, "fence:1")
            .await
            .expect("acquire"),
    );
    old.admit("occurrence:old", "topic", "payload")
        .await
        .expect("old admission");
    let released = old.release().await.expect("release");
    let next = acquired(
        store
            .acquire_local_lease_after_head("lease:cross-generation", released, 2, "fence:2")
            .await
            .expect("next generation"),
    );
    assert!(matches!(
        next.admit("occurrence:old", "topic", "payload").await,
        Err(LocalLeaseOutboxError::StaleFence(_))
    ));
}

#[tokio::test]
async fn lease_head_cas_rejects_stale_terminal_head_and_digest_mismatch() {
    let temp = TempDir::new().expect("temp dir");
    let store = opened_store(&temp, 108).await;
    let first = acquired(
        store
            .acquire_local_lease("lease:head-cas", 1, "fence:1")
            .await
            .expect("acquire"),
    );
    let first_terminal = first.release().await.expect("release first");
    let second = acquired(
        store
            .acquire_local_lease_after_head("lease:head-cas", first_terminal.clone(), 2, "fence:2")
            .await
            .expect("acquire second"),
    );
    let second_terminal = second.release().await.expect("release second");

    // The old generation-1 terminal head cannot cross the generation-2
    // terminal transition, even though its generation would otherwise imply
    // the requested next generation.
    assert!(matches!(
        store
            .acquire_local_lease_after_head("lease:head-cas", first_terminal.clone(), 2, "fence:3",)
            .await,
        Err(LocalLeaseOutboxError::CasConflict(_))
    ));

    let mut forged = second_terminal;
    forged.lease_sha256 = Sha256Digest::for_bytes(b"forged head");
    assert!(matches!(
        store
            .acquire_local_lease_after_head("lease:head-cas", forged, 3, "fence:3")
            .await,
        Err(LocalLeaseOutboxError::CasConflict(_))
    ));
}

#[tokio::test]
async fn maximum_length_lease_id_keeps_generated_row_ids_bounded() {
    let temp = TempDir::new().expect("temp dir");
    let store = opened_store(&temp, 109).await;
    let lease_id = "l".repeat(512);
    let handle = acquired(
        store
            .acquire_local_lease(&lease_id, 1, "fence:1")
            .await
            .expect("acquire"),
    );
    let LocalAdmission::Queued(receipt) = handle
        .admit("occurrence:max-lease-id", "topic", "payload")
        .await
        .expect("admit")
    else {
        panic!("first admission must append");
    };
    assert!(receipt.event_id.len() <= 512);
    assert!(receipt.outbox_id.len() <= 512);
}

#[tokio::test]
async fn event_and_outbox_faults_leave_no_partial_rows() {
    let temp = TempDir::new().expect("temp dir");
    let store = opened_store(&temp, 103).await;
    let handle = acquired(
        store
            .acquire_local_lease("lease:atomic", 1, "fence:1")
            .await
            .expect("acquire"),
    );
    assert!(handle
        .admit_with_fault(
            "occurrence:fault",
            "topic",
            "payload",
            LocalAdmissionFault::AfterEventBeforeOutbox,
        )
        .await
        .is_err());
    assert_eq!(
        handle.snapshot_counts().await.expect("counts after fault"),
        crate::LocalLeaseOutboxCounts {
            lease_rows: 1,
            event_rows: 0,
            outbox_rows: 0,
        }
    );
    assert!(handle
        .admit_with_fault(
            "occurrence:fault-after-outbox",
            "topic",
            "payload",
            LocalAdmissionFault::AfterOutboxBeforeCommit,
        )
        .await
        .is_err());
    assert_eq!(
        handle
            .snapshot_counts()
            .await
            .expect("counts after outbox fault"),
        crate::LocalLeaseOutboxCounts {
            lease_rows: 1,
            event_rows: 0,
            outbox_rows: 0,
        }
    );
    assert!(matches!(
        handle.admit("occurrence:fault", "topic", "payload").await,
        Ok(LocalAdmission::Queued(_))
    ));
}

#[tokio::test]
async fn unknown_reconcile_survives_reopen_and_never_claims_effect() {
    let temp = TempDir::new().expect("temp dir");
    let owner = agent_id(104);
    let store = CognitiveStore::open(&layout(&temp, &owner))
        .await
        .expect("store");
    let handle = acquired(
        store
            .acquire_local_lease("lease:unknown", 1, "fence:1")
            .await
            .expect("acquire"),
    );
    let admission = handle
        .admit("occurrence:unknown", "topic", "payload")
        .await
        .expect("admit");
    let LocalAdmission::Queued(receipt) = admission else {
        panic!("first admission must append");
    };
    assert!(!receipt.external_effect);
    handle
        .mark_indeterminate("occurrence:unknown", "lost-local-ack")
        .await
        .expect("indeterminate");
    assert_eq!(
        handle.status("occurrence:unknown").await.expect("status"),
        LocalOutcomeState::Indeterminate
    );
    store.pool.close().await;
    drop(handle);
    drop(store);

    let reopened_store = CognitiveStore::open(&layout(&temp, &owner))
        .await
        .expect("reopen store");
    let reopened = reopened_store
        .reopen_local_lease("lease:unknown", 1, "fence:1")
        .await
        .expect("reopen lease");
    let outcome = reopened
        .reconcile("occurrence:unknown", LocalReconcileOutcome::Committed)
        .await
        .expect("reconcile");
    assert!(!outcome.external_effect);
    assert_eq!(
        reopened.status("occurrence:unknown").await.expect("status"),
        LocalOutcomeState::Committed
    );
}

#[tokio::test]
async fn indeterminate_replay_is_status_aware_and_releases_without_dispatch() {
    let temp = TempDir::new().expect("temp dir");
    let owner = agent_id(110);
    let store = CognitiveStore::open(&layout(&temp, &owner))
        .await
        .expect("store");
    let first = acquired(
        store
            .acquire_local_lease("lease:replay-recovery", 1, "fence:1")
            .await
            .expect("acquire"),
    );
    let LocalAdmission::Queued(receipt) = first
        .admit("occurrence:replay-recovery", "topic", "payload")
        .await
        .expect("admit")
    else {
        panic!("first admission must append");
    };
    assert!(!receipt.external_effect);
    first
        .mark_indeterminate("occurrence:replay-recovery", "simulated-crash-window")
        .await
        .expect("indeterminate");

    // Simulate process death after the durable indeterminate row and before
    // the lease release.  Reopening the same generation must not call admit
    // again or turn the quarantined outbox into a dispatchable receipt.
    drop(first);
    store.pool.close().await;
    drop(store);
    let reopened_store = CognitiveStore::open(&layout(&temp, &owner))
        .await
        .expect("reopen store");
    let replay = match reopened_store
        .acquire_local_lease("lease:replay-recovery", 1, "fence:1")
        .await
        .expect("replay acquire")
    {
        LocalLeaseAcquire::Replay(handle) => handle,
        LocalLeaseAcquire::Acquired(_) => panic!("reopen must replay the active lease"),
    };
    let finalized = replay
        .finalize_replayed_occurrence("occurrence:replay-recovery")
        .await
        .expect("status-aware finalization");
    let LocalReplayFinalization::Released {
        outcome,
        external_effect,
        ..
    } = finalized
    else {
        panic!("indeterminate occurrence must be released");
    };
    assert_eq!(outcome, LocalOutcomeState::Indeterminate);
    assert!(!external_effect);
    assert_eq!(
        replay.snapshot_counts().await.expect("counts"),
        crate::LocalLeaseOutboxCounts {
            lease_rows: 2,
            event_rows: 2,
            outbox_rows: 1,
        }
    );
    assert!(reopened_store
        .acquire_local_lease("lease:replay-recovery", 1, "fence:1")
        .await
        .is_err());
}

#[tokio::test]
async fn queued_replay_returns_original_receipt_and_keeps_lease_active() {
    let temp = TempDir::new().expect("temp dir");
    let store = opened_store(&temp, 111).await;
    let first = acquired(
        store
            .acquire_local_lease("lease:queued-replay", 1, "fence:1")
            .await
            .expect("acquire"),
    );
    let LocalAdmission::Queued(original) = first
        .admit("occurrence:queued-replay", "topic", "payload")
        .await
        .expect("admit")
    else {
        panic!("first admission must append");
    };
    let replay = acquired(
        store
            .acquire_local_lease("lease:queued-replay", 1, "fence:1")
            .await
            .expect("replay"),
    );
    let recovered = replay
        .finalize_replayed_occurrence("occurrence:queued-replay")
        .await
        .expect("queued replay");
    let LocalReplayFinalization::Queued(receipt) = recovered else {
        panic!("queued occurrence must remain active");
    };
    assert_eq!(receipt, original);
    assert!(!receipt.external_effect);
    assert_eq!(
        replay.status("occurrence:queued-replay").await.unwrap(),
        LocalOutcomeState::Queued
    );
}

#[tokio::test]
async fn replay_without_admission_is_explicit_and_keeps_lease_writable() {
    let temp = TempDir::new().expect("temp dir");
    let store = opened_store(&temp, 112).await;
    let replay = acquired(
        store
            .acquire_local_lease("lease:admit-crash-window", 1, "fence:1")
            .await
            .expect("initial acquire"),
    );

    // The process exits after acquire commits but before admit starts.  The
    // next acquire is a replay, yet there is no occurrence row to finalize.
    let recovered = replay
        .finalize_replayed_occurrence("occurrence:admit-crash-window")
        .await
        .expect("not-admitted replay");
    assert_eq!(recovered, LocalReplayFinalization::NotAdmitted);
    assert_eq!(
        replay.snapshot_counts().await.expect("counts before admit"),
        crate::LocalLeaseOutboxCounts {
            lease_rows: 1,
            event_rows: 0,
            outbox_rows: 0,
        }
    );

    let LocalAdmission::Queued(receipt) = replay
        .admit("occurrence:admit-crash-window", "topic", "payload")
        .await
        .expect("retry original admission")
    else {
        panic!("not-admitted replay must permit the first admission");
    };
    assert!(!receipt.external_effect);
    assert_eq!(
        replay.snapshot_counts().await.expect("counts after admit"),
        crate::LocalLeaseOutboxCounts {
            lease_rows: 1,
            event_rows: 1,
            outbox_rows: 1,
        }
    );
}

#[tokio::test]
async fn tampered_event_chain_is_rejected_on_reopen() {
    let temp = TempDir::new().expect("temp dir");
    let store = opened_store(&temp, 105).await;
    let handle = acquired(
        store
            .acquire_local_lease("lease:tamper", 1, "fence:1")
            .await
            .expect("acquire"),
    );
    handle
        .admit("occurrence:tamper", "topic", "payload")
        .await
        .expect("admit");
    sqlx::query("DROP TRIGGER cognitive_local_events_no_update")
        .execute(&store.pool)
        .await
        .expect("drop test trigger");
    sqlx::query(
        "UPDATE cognitive_local_events SET payload_json = 'changed' WHERE lease_id = 'lease:tamper'",
    )
    .execute(&store.pool)
    .await
    .expect("tamper");
    assert!(store
        .reopen_local_lease("lease:tamper", 1, "fence:1")
        .await
        .is_err());
}

#[tokio::test]
async fn tampered_child_chain_cannot_be_terminalized_by_release_or_rollback() {
    // Exercise both host terminal decisions against each append-only child
    // journal.  A corrupt child must fail before the lease terminal row is
    // appended; otherwise a caller could use release/rollback to hide the
    // damaged history from subsequent readers.
    for (index, (transition, child)) in [
        ("release", "event"),
        ("rollback", "event"),
        ("release", "outbox"),
        ("rollback", "outbox"),
    ]
    .into_iter()
    .enumerate()
    {
        let temp = TempDir::new().expect("temp dir");
        let store = opened_store(&temp, 115 + index as u8).await;
        let lease_id = format!("lease:terminal-tamper-{transition}-{child}");
        let handle = acquired(
            store
                .acquire_local_lease(&lease_id, 1, "fence:tamper")
                .await
                .expect("acquire"),
        );
        handle
            .admit("occurrence:terminal-tamper", "topic", "payload")
            .await
            .expect("admit");

        if child == "event" {
            sqlx::query("DROP TRIGGER cognitive_local_events_no_update")
                .execute(&store.pool)
                .await
                .expect("drop event update trigger");
            sqlx::query("UPDATE cognitive_local_events SET payload_json = ? WHERE lease_id = ?")
                .bind("tampered event payload")
                .bind(&lease_id)
                .execute(&store.pool)
                .await
                .expect("tamper event payload");
        } else {
            sqlx::query("DROP TRIGGER cognitive_local_outbox_no_update")
                .execute(&store.pool)
                .await
                .expect("drop outbox update trigger");
            sqlx::query("UPDATE cognitive_local_outbox SET payload_json = ? WHERE lease_id = ?")
                .bind("tampered outbox payload")
                .bind(&lease_id)
                .execute(&store.pool)
                .await
                .expect("tamper outbox payload");
        }

        let before = handle
            .snapshot_counts()
            .await
            .expect("counts before transition");
        let result = match transition {
            "release" => handle.release().await,
            "rollback" => handle.rollback_lease().await,
            _ => unreachable!("test transition"),
        };
        assert!(
            matches!(result, Err(LocalLeaseOutboxError::Corrupt(_))),
            "{transition} must fail closed for tampered {child} chain: {result:?}"
        );
        assert_eq!(
            handle
                .snapshot_counts()
                .await
                .expect("counts after transition"),
            before,
            "{transition} must not append a terminal lease for tampered {child} chain"
        );
    }
}

#[test]
fn local_lease_outbox_has_no_production_authority() {
    assert!(!LOCAL_LEASE_OUTBOX_EXTERNAL_EFFECTS);
    assert!(!LOCAL_LEASE_OUTBOX_KG_WRITE_AUTHORITY);
    assert!(!LOCAL_LEASE_OUTBOX_PRODUCTION_CALLER);
}
