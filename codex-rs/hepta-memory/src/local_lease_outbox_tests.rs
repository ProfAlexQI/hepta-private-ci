use pretty_assertions::assert_eq;
use tempfile::TempDir;

use crate::CognitiveStore;
use crate::LOCAL_LEASE_OUTBOX_EXTERNAL_EFFECTS;
use crate::LOCAL_LEASE_OUTBOX_KG_WRITE_AUTHORITY;
use crate::LOCAL_LEASE_OUTBOX_PRODUCTION_CALLER;
use crate::LocalAdmission;
use crate::LocalAdmissionFault;
use crate::LocalLeaseAcquire;
use crate::LocalLeaseOutboxError;
use crate::LocalOutcomeState;
use crate::LocalReconcileOutcome;
use crate::LocalReplayFinalization;
use crate::cognitive_test_support::agent_id;
use crate::cognitive_test_support::layout;
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
    assert!(
        store
            .acquire_local_lease_after("lease:generation", 1, 2, "fence:1")
            .await
            .is_err()
    );
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
    assert!(
        old.admit("occurrence:stale", "topic", "payload")
            .await
            .is_err()
    );
    assert!(
        next.admit("occurrence:current", "topic", "payload")
            .await
            .is_ok()
    );
    assert!(
        store
            .acquire_local_lease_after("lease:generation", 1, 3, "fence:3")
            .await
            .is_err()
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
    assert!(
        handle
            .admit_with_fault(
                "occurrence:fault",
                "topic",
                "payload",
                LocalAdmissionFault::AfterEventBeforeOutbox,
            )
            .await
            .is_err()
    );
    assert_eq!(
        handle.snapshot_counts().await.expect("counts after fault"),
        crate::LocalLeaseOutboxCounts {
            lease_rows: 1,
            event_rows: 0,
            outbox_rows: 0,
        }
    );
    assert!(
        handle
            .admit_with_fault(
                "occurrence:fault-after-outbox",
                "topic",
                "payload",
                LocalAdmissionFault::AfterOutboxBeforeCommit,
            )
            .await
            .is_err()
    );
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
    assert!(
        reopened_store
            .acquire_local_lease("lease:replay-recovery", 1, "fence:1")
            .await
            .is_err()
    );
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
    assert!(
        store
            .reopen_local_lease("lease:tamper", 1, "fence:1")
            .await
            .is_err()
    );
}

#[test]
fn local_lease_outbox_has_no_production_authority() {
    assert!(!LOCAL_LEASE_OUTBOX_EXTERNAL_EFFECTS);
    assert!(!LOCAL_LEASE_OUTBOX_KG_WRITE_AUTHORITY);
    assert!(!LOCAL_LEASE_OUTBOX_PRODUCTION_CALLER);
}
