use pretty_assertions::assert_eq;
use tempfile::TempDir;

use crate::CognitiveStore;
use crate::LOCAL_LEASE_OUTBOX_EXTERNAL_EFFECTS;
use crate::LOCAL_LEASE_OUTBOX_KG_WRITE_AUTHORITY;
use crate::LOCAL_LEASE_OUTBOX_PRODUCTION_CALLER;
use crate::LocalAdmission;
use crate::LocalAdmissionFault;
use crate::LocalLeaseAcquire;
use crate::LocalOutcomeState;
use crate::LocalReconcileOutcome;
use crate::cognitive_test_support::agent_id;
use crate::cognitive_test_support::layout;

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
    assert!(
        store
            .acquire_local_lease_after("lease:generation", 1, 2, "fence:2")
            .await
            .is_err()
    );
    old.release().await.expect("release");
    assert!(
        store
            .acquire_local_lease_after("lease:generation", 1, 2, "fence:1")
            .await
            .is_err()
    );
    let next = acquired(
        store
            .acquire_local_lease_after("lease:generation", 1, 2, "fence:2")
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
