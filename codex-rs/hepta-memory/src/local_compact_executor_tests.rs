use codex_hepta_contracts::Sha256Digest;
use pretty_assertions::assert_eq;
use tempfile::TempDir;

use crate::CognitiveStore;
use crate::CompactCheckpoint;
use crate::CompactFence;
use crate::CompactLease;
use crate::CompactLossReport;
use crate::CompactParentSnapshot;
use crate::CompactPersistenceAppend;
use crate::CompactPersistenceState;
use crate::CompactProtectedRef;
use crate::CompactReconcileOutcome;
use crate::CompactSummaryReceipt;
use crate::LOCAL_COMPACT_EXECUTOR_EXTERNAL_EFFECTS;
use crate::LOCAL_COMPACT_EXECUTOR_KG_WRITE_AUTHORITY;
use crate::LOCAL_COMPACT_EXECUTOR_NAMESPACE;
use crate::cognitive_test_support::agent_id;
use crate::cognitive_test_support::layout;
use crate::compact_persistence::checkpoint_digest;

fn fence(generation: u64, token: &str) -> CompactFence {
    CompactFence::new(3, 8, generation, token).expect("fence")
}

fn snapshot(fence: CompactFence) -> CompactParentSnapshot {
    CompactParentSnapshot::new(
        "ctx:local-authoritative",
        20,
        30,
        7,
        Sha256Digest::for_bytes(b"parent-state"),
        fence,
    )
    .expect("snapshot")
}

fn checkpoint(fence: CompactFence) -> CompactCheckpoint {
    CompactCheckpoint::new(
        "ctxcp:local-authoritative",
        CompactLease::from_snapshot(snapshot(fence)),
        vec![CompactProtectedRef::new("approval:1", "approval", true).expect("ref")],
        CompactSummaryReceipt::new(
            Sha256Digest::for_bytes(b"summary"),
            Sha256Digest::for_bytes(b"model"),
            Sha256Digest::for_bytes(b"policy"),
        ),
        CompactLossReport::new(vec!["event:29".to_string()], 1, Vec::new(), 0).expect("loss"),
        0,
    )
    .expect("checkpoint")
}

#[tokio::test]
async fn sqlite_checkpoint_commit_reopen_and_read_only_rehydrate() {
    let temp = TempDir::new().expect("temp dir");
    let owner = agent_id(91);
    let store = CognitiveStore::open(&layout(&temp, &owner))
        .await
        .expect("store");
    let current_fence = fence(19, "fence:19");
    let current = snapshot(current_fence.clone());
    let checkpoint = checkpoint(current_fence.clone());
    let executor = store
        .open_local_compact_executor("journal:local-authoritative", current_fence.clone())
        .await
        .expect("executor");
    assert_eq!(
        executor
            .append_intent("op:commit", &checkpoint, &current)
            .await
            .expect("intent"),
        CompactPersistenceAppend::Appended { sequence: 1 }
    );
    let digest = checkpoint_digest(&checkpoint).expect("checkpoint digest");
    assert_eq!(
        executor
            .commit_checkpoint("op:commit", &digest)
            .await
            .expect("commit"),
        CompactPersistenceAppend::Appended { sequence: 2 }
    );
    store.pool.close().await;
    drop(executor);
    drop(store);

    let reopened_store = CognitiveStore::open(&layout(&temp, &owner))
        .await
        .expect("reopen store");
    let reopened = reopened_store
        .open_local_compact_executor("journal:local-authoritative", current_fence)
        .await
        .expect("reopen executor");
    assert_eq!(
        reopened.state("op:commit").await.expect("state"),
        Some(CompactPersistenceState::Committed)
    );
    let plan = reopened
        .rehydrate("op:commit", &checkpoint, 0)
        .await
        .expect("rehydration");
    assert_eq!(plan.status, crate::RehydrationStatus::Complete);
    assert_eq!(plan.checkpoint_id, checkpoint.checkpoint_id);
}

#[tokio::test]
async fn unknown_outcome_survives_reopen_until_explicit_reconcile() {
    let temp = TempDir::new().expect("temp dir");
    let owner = agent_id(92);
    let store = CognitiveStore::open(&layout(&temp, &owner))
        .await
        .expect("store");
    let current_fence = fence(21, "fence:21");
    let current = snapshot(current_fence.clone());
    let checkpoint = checkpoint(current_fence.clone());
    let executor = store
        .open_local_compact_executor("journal:unknown", current_fence.clone())
        .await
        .expect("executor");
    executor
        .append_intent("op:unknown", &checkpoint, &current)
        .await
        .expect("intent");
    executor
        .mark_indeterminate("op:unknown", "lost-local-ack")
        .await
        .expect("quarantine");
    assert_eq!(
        executor.state("op:unknown").await.expect("state"),
        Some(CompactPersistenceState::Indeterminate)
    );
    store.pool.close().await;
    drop(executor);
    drop(store);

    let reopened_store = CognitiveStore::open(&layout(&temp, &owner))
        .await
        .expect("reopen store");
    let reopened = reopened_store
        .open_local_compact_executor("journal:unknown", current_fence)
        .await
        .expect("reopen executor");
    assert!(
        reopened
            .rehydrate("op:unknown", &checkpoint, 0)
            .await
            .is_err()
    );
    reopened
        .reconcile("op:unknown", CompactReconcileOutcome::Committed)
        .await
        .expect("reconcile");
    assert_eq!(
        reopened.state("op:unknown").await.expect("state"),
        Some(CompactPersistenceState::Committed)
    );
    assert_eq!(
        reopened
            .rehydrate("op:unknown", &checkpoint, 0)
            .await
            .expect("rehydrate after reconcile")
            .status,
        crate::RehydrationStatus::Complete
    );
}

#[tokio::test]
async fn stale_fence_and_sqlite_tamper_fail_closed() {
    let temp = TempDir::new().expect("temp dir");
    let owner = agent_id(93);
    let store = CognitiveStore::open(&layout(&temp, &owner))
        .await
        .expect("store");
    let old_fence = fence(31, "fence:31");
    let current = snapshot(old_fence.clone());
    let checkpoint = checkpoint(old_fence.clone());
    let executor = store
        .open_local_compact_executor("journal:tamper", old_fence.clone())
        .await
        .expect("executor");
    executor
        .append_intent("op:tamper", &checkpoint, &current)
        .await
        .expect("intent");

    let stale = store
        .open_local_compact_executor("journal:tamper", fence(32, "fence:32"))
        .await;
    assert!(matches!(
        stale,
        Err(crate::LocalCompactExecutorError::Corrupt(_))
    ));

    // Test-only tamper: remove the immutable trigger, alter the serialized
    // event, and ensure the executor's digest-chain reopen still rejects it.
    sqlx::query("DROP TRIGGER cognitive_compact_events_no_update")
        .execute(&store.pool)
        .await
        .expect("drop test trigger");
    sqlx::query(
        "UPDATE cognitive_compact_events
         SET event_json = replace(event_json, 'op:tamper', 'op:changed')
         WHERE journal_id = 'journal:tamper'",
    )
    .execute(&store.pool)
    .await
    .expect("tamper event");
    let corrupt = store
        .open_local_compact_executor("journal:tamper", old_fence)
        .await;
    assert!(matches!(
        corrupt,
        Err(crate::LocalCompactExecutorError::Persistence(_))
            | Err(crate::LocalCompactExecutorError::Corrupt(_))
            | Err(crate::LocalCompactExecutorError::Serialization(_))
    ));
}

#[test]
fn local_executor_keeps_production_boundaries_closed() {
    assert_eq!(LOCAL_COMPACT_EXECUTOR_NAMESPACE, "local_development_only");
    assert!(!LOCAL_COMPACT_EXECUTOR_EXTERNAL_EFFECTS);
    assert!(!LOCAL_COMPACT_EXECUTOR_KG_WRITE_AUTHORITY);
}
