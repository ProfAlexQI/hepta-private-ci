use codex_hepta_contracts::Sha256Digest;
use pretty_assertions::assert_eq;
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
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
use crate::LocalCompactExecutorError;
use crate::LocalLeaseAcquire;
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

fn unix_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_secs()
}

async fn open_bound_executor(
    owner_number: u8,
    expiry_offset_seconds: u64,
) -> (
    TempDir,
    CognitiveStore,
    crate::LocalLeaseOutbox,
    crate::LocalCompactExecutor,
    CompactFence,
    CompactCheckpoint,
) {
    let temp = TempDir::new().expect("temp dir");
    let owner = agent_id(owner_number);
    let store = CognitiveStore::open(&layout(&temp, &owner))
        .await
        .expect("store");
    let current_fence = fence(1, "bound-fence");
    let expires_at = unix_seconds() + expiry_offset_seconds;
    let lease = match store
        .acquire_local_lease_bound(
            "lease:bound-executor",
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
    let executor = store
        .open_local_compact_executor_bound("journal:bound-executor", current_fence.clone(), &lease)
        .await
        .expect("bound executor");
    let checkpoint = checkpoint(current_fence.clone());
    (temp, store, lease, executor, current_fence, checkpoint)
}

#[tokio::test]
async fn bound_compact_replay_is_idempotent_while_lease_active() {
    let (_temp, store, lease, executor, current_fence, checkpoint) =
        open_bound_executor(201, 3_600).await;
    assert!(executor.is_bound());
    let current = snapshot(current_fence);
    let operation_id = "op:bound-replay";
    assert_eq!(
        executor
            .append_intent(operation_id, &checkpoint, &current)
            .await
            .expect("bound intent"),
        CompactPersistenceAppend::Appended { sequence: 1 }
    );
    let digest = checkpoint_digest(&checkpoint).expect("checkpoint digest");
    assert_eq!(
        executor
            .commit_checkpoint(operation_id, &digest)
            .await
            .expect("bound commit"),
        CompactPersistenceAppend::Appended { sequence: 2 }
    );

    let first = executor
        .rehydrate(operation_id, &checkpoint, 0)
        .await
        .expect("bound rehydrate");
    assert_eq!(first.status, crate::RehydrationStatus::Complete);
    let replay = executor
        .rehydrate(operation_id, &checkpoint, 0)
        .await
        .expect("bound rehydrate replay");
    assert_eq!(replay.status, crate::RehydrationStatus::Complete);
    let durable = executor.snapshot().await.expect("bound replay snapshot");
    assert_eq!(durable.entries.len(), 3);
    assert_eq!(
        durable.entries.last().map(|entry| &entry.kind),
        Some(&crate::CompactPersistenceEventKind::Rehydrated {
            checkpoint_sha256: digest,
            expected_revision: 0,
        })
    );
    lease.verify_current().await.expect("active bound lease");
    store.pool.close().await;
}

#[tokio::test]
async fn bound_compact_mutations_reject_released_lease_without_compact_rows() {
    let (_temp, store, lease, executor, current_fence, checkpoint) =
        open_bound_executor(202, 3_600).await;
    assert!(executor.is_bound());
    let current = snapshot(current_fence);
    let operation_id = "op:bound-release";
    executor
        .append_intent(operation_id, &checkpoint, &current)
        .await
        .expect("bound intent");
    let digest = checkpoint_digest(&checkpoint).expect("checkpoint digest");
    executor
        .commit_checkpoint(operation_id, &digest)
        .await
        .expect("bound commit");
    let before = executor.snapshot().await.expect("before release snapshot");
    let before_count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM cognitive_compact_events WHERE journal_id = ?")
            .bind(executor.journal_id())
            .fetch_one(&store.pool)
            .await
            .expect("before release row count");

    lease.release().await.expect("release bound lease");

    assert!(matches!(
        executor
            .append_intent("op:after-release", &checkpoint, &current)
            .await,
        Err(crate::LocalCompactExecutorError::Lease(
            crate::LocalLeaseOutboxError::StaleFence(_)
        ))
    ));
    assert!(matches!(
        executor.commit_checkpoint(operation_id, &digest).await,
        Err(crate::LocalCompactExecutorError::Lease(
            crate::LocalLeaseOutboxError::StaleFence(_)
        ))
    ));
    assert!(matches!(
        executor
            .mark_indeterminate(operation_id, "after-release")
            .await,
        Err(crate::LocalCompactExecutorError::Lease(
            crate::LocalLeaseOutboxError::StaleFence(_)
        ))
    ));
    assert!(matches!(
        executor
            .reconcile(operation_id, CompactReconcileOutcome::Committed)
            .await,
        Err(crate::LocalCompactExecutorError::Lease(
            crate::LocalLeaseOutboxError::StaleFence(_)
        ))
    ));
    assert!(matches!(
        executor.rehydrate(operation_id, &checkpoint, 0).await,
        Err(crate::LocalCompactExecutorError::Lease(
            crate::LocalLeaseOutboxError::StaleFence(_)
        ))
    ));

    let after = executor.snapshot().await.expect("after release snapshot");
    let after_count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM cognitive_compact_events WHERE journal_id = ?")
            .bind(executor.journal_id())
            .fetch_one(&store.pool)
            .await
            .expect("after release row count");
    assert_eq!(after.entries, before.entries);
    assert_eq!(after.head_sha256, before.head_sha256);
    assert_eq!(after_count, before_count);
    store.pool.close().await;
}

#[tokio::test]
async fn bound_compact_mutation_rejects_lease_expiry_after_open() {
    let (_temp, store, lease, executor, current_fence, checkpoint) =
        open_bound_executor(203, 3).await;
    assert!(executor.is_bound());
    let current = snapshot(current_fence);
    let operation_id = "op:bound-expiry";
    executor
        .append_intent(operation_id, &checkpoint, &current)
        .await
        .expect("bound intent before expiry");
    let before = executor.snapshot().await.expect("before expiry snapshot");
    let expiry = lease
        .binding()
        .expect("explicit lease binding")
        .lease_expires_at_unix_seconds;
    for _ in 0..240 {
        if unix_seconds() >= expiry {
            break;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    assert!(
        unix_seconds() >= expiry,
        "test lease did not expire in time"
    );

    assert!(matches!(
        executor
            .append_intent("op:after-expiry", &checkpoint, &current)
            .await,
        Err(crate::LocalCompactExecutorError::Lease(
            crate::LocalLeaseOutboxError::StaleFence(_)
        ))
    ));
    assert!(matches!(
        executor
            .commit_checkpoint(
                operation_id,
                &checkpoint_digest(&checkpoint).expect("digest")
            )
            .await,
        Err(crate::LocalCompactExecutorError::Lease(
            crate::LocalLeaseOutboxError::StaleFence(_)
        ))
    ));

    // E20 closes the expired bound lease only through the explicit host
    // timeout transition.  The already-open compact writer remains fenced
    // after that terminal append; expiry never grants a takeover or a retry.
    let expired = lease
        .expire_lease()
        .await
        .expect("explicit expiry terminalization");
    assert_eq!(expired.state, crate::LocalLeaseState::RolledBack);
    assert!(matches!(
        executor
            .append_intent("op:after-explicit-expiry", &checkpoint, &current)
            .await,
        Err(crate::LocalCompactExecutorError::Lease(
            crate::LocalLeaseOutboxError::StaleFence(_)
        ))
    ));
    let after = executor.snapshot().await.expect("after expiry snapshot");
    assert_eq!(after.entries, before.entries);
    assert_eq!(after.head_sha256, before.head_sha256);
    store.pool.close().await;
}

#[tokio::test]
async fn compact_rotation_rejects_old_journal_and_accepts_new_journal_id() {
    let (_temp, store, lease, old_executor, old_fence, old_checkpoint) =
        open_bound_executor(205, 1).await;
    let old_current = snapshot(old_fence.clone());
    old_executor
        .append_intent("op:rotation-old", &old_checkpoint, &old_current)
        .await
        .expect("old generation intent");

    let expires_at = lease
        .binding()
        .expect("explicit old lease binding")
        .lease_expires_at_unix_seconds;
    for _ in 0..240 {
        if unix_seconds() >= expires_at {
            break;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    assert!(unix_seconds() >= expires_at, "old lease did not expire");
    let terminal = lease
        .expire_lease()
        .await
        .expect("old timeout terminalization");

    let next_fence = fence(2, "bound-fence-next");
    let next_expiry = unix_seconds() + 3_600;
    let next = match store
        .acquire_local_lease_after_head_bound(
            "lease:bound-executor",
            terminal,
            next_fence.authority_epoch,
            next_fence.owner_epoch,
            next_fence.generation,
            next_fence.fencing_token.clone(),
            next_expiry,
        )
        .await
        .expect("next generation lease")
    {
        LocalLeaseAcquire::Acquired(lease) | LocalLeaseAcquire::Replay(lease) => lease,
    };

    // Reusing the old journal id would mix generation-1 rows with the new
    // fence.  The opener must reject that history rather than rotate it
    // implicitly or overwrite it.
    assert!(matches!(
        store
            .open_local_compact_executor_bound("journal:bound-executor", next_fence.clone(), &next,)
            .await,
        Err(LocalCompactExecutorError::Corrupt(_))
    ));

    // Rotation is explicit at the host seam: a fresh journal id gives the
    // next generation an empty, independently bound compact history.
    let next_executor = store
        .open_local_compact_executor_bound(
            "journal:bound-executor-generation-2",
            next_fence.clone(),
            &next,
        )
        .await
        .expect("new journal id accepts next generation");
    assert!(next_executor.is_bound());
    let next_checkpoint = checkpoint(next_fence.clone());
    let next_current = snapshot(next_fence);
    next_executor
        .append_intent("op:rotation-next", &next_checkpoint, &next_current)
        .await
        .expect("next generation intent");
    store.pool.close().await;
}

#[tokio::test]
async fn sqlite_checkpoint_commit_reopen_and_rehydrate_replay() {
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
    assert!(!executor.is_bound());
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
    let first_snapshot = reopened.snapshot().await.expect("snapshot after rehydrate");
    assert_eq!(first_snapshot.entries.len(), 3);
    assert!(matches!(
        first_snapshot.entries.last().map(|entry| &entry.kind),
        Some(crate::CompactPersistenceEventKind::Rehydrated {
            checkpoint_sha256: _,
            expected_revision: 0,
        })
    ));
    assert_eq!(
        reopened
            .rehydration("op:commit")
            .await
            .expect("rehydration witness")
            .expect("witness")
            .sequence,
        3
    );
    let replay_plan = reopened
        .rehydrate("op:commit", &checkpoint, 0)
        .await
        .expect("idempotent rehydration replay");
    assert_eq!(replay_plan.status, crate::RehydrationStatus::Complete);
    assert_eq!(
        reopened
            .snapshot()
            .await
            .expect("replay snapshot")
            .entries
            .len(),
        3
    );
    let replay_row_count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM cognitive_compact_events WHERE journal_id = ?")
            .bind("journal:local-authoritative")
            .fetch_one(&reopened_store.pool)
            .await
            .expect("replay row count");
    assert_eq!(replay_row_count, 3);

    reopened_store.pool.close().await;
    drop(reopened);
    drop(reopened_store);
    let restarted_store = CognitiveStore::open(&layout(&temp, &owner))
        .await
        .expect("restart store");
    let restarted = restarted_store
        .open_local_compact_executor("journal:local-authoritative", fence(19, "fence:19"))
        .await
        .expect("restart executor");
    restarted
        .rehydrate("op:commit", &checkpoint, 0)
        .await
        .expect("restart rehydration replay");
    assert_eq!(
        restarted
            .snapshot()
            .await
            .expect("restart snapshot")
            .entries
            .len(),
        3
    );
    let restart_row_count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM cognitive_compact_events WHERE journal_id = ?")
            .bind("journal:local-authoritative")
            .fetch_one(&restarted_store.pool)
            .await
            .expect("restart row count");
    assert_eq!(restart_row_count, 3);
}

#[tokio::test]
async fn read_rehydration_is_pure_until_explicit_rehydrate() {
    let temp = TempDir::new().expect("temp dir");
    let owner = agent_id(95);
    let store = CognitiveStore::open(&layout(&temp, &owner))
        .await
        .expect("store");
    let current_fence = fence(35, "fence:35");
    let current = snapshot(current_fence.clone());
    let checkpoint = checkpoint(current_fence.clone());
    let executor = store
        .open_local_compact_executor("journal:read-only", current_fence)
        .await
        .expect("executor");
    executor
        .append_intent("op:read-only", &checkpoint, &current)
        .await
        .expect("intent");
    let digest = checkpoint_digest(&checkpoint).expect("digest");
    executor
        .commit_checkpoint("op:read-only", &digest)
        .await
        .expect("commit");

    let before = executor.snapshot().await.expect("before snapshot");
    let read = executor
        .read_rehydration("op:read-only", &checkpoint, 0)
        .await
        .expect("read-only plan");
    assert_eq!(read.plan.status, crate::RehydrationStatus::NotStarted);
    assert!(read.witness.is_none());
    assert_eq!(read.checkpoint_sha256, digest);
    let after = executor.snapshot().await.expect("after snapshot");
    assert_eq!(after.entries, before.entries);
    assert_eq!(after.head_sha256, before.head_sha256);

    executor
        .rehydrate("op:read-only", &checkpoint, 0)
        .await
        .expect("explicit rehydrate");
    let complete = executor
        .read_rehydration("op:read-only", &checkpoint, 0)
        .await
        .expect("completed read-only plan");
    assert_eq!(complete.plan.status, crate::RehydrationStatus::Complete);
    assert_eq!(
        complete
            .witness
            .as_ref()
            .expect("witness")
            .checkpoint_sha256,
        digest
    );
    let complete_again = executor
        .read_rehydration("op:read-only", &checkpoint, 0)
        .await
        .expect("replay read-only plan");
    assert_eq!(complete_again, complete);
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

#[tokio::test]
async fn compact_reopen_binds_authority_and_owner_epochs_and_rejects_legacy_nulls() {
    let temp = TempDir::new().expect("temp dir");
    let owner = agent_id(96);
    let store = CognitiveStore::open(&layout(&temp, &owner))
        .await
        .expect("store");
    let current_fence = fence(41, "fence:41");
    let current = snapshot(current_fence.clone());
    let checkpoint = checkpoint(current_fence.clone());
    let executor = store
        .open_local_compact_executor("journal:fence-epochs", current_fence.clone())
        .await
        .expect("executor");
    executor
        .append_intent("op:fence-epochs", &checkpoint, &current)
        .await
        .expect("intent");

    let stored_authority: i64 = sqlx::query_scalar(
        "SELECT authority_epoch FROM cognitive_compact_events
         WHERE journal_id = ? AND sequence = 1",
    )
    .bind("journal:fence-epochs")
    .fetch_one(&store.pool)
    .await
    .expect("authority epoch");
    let stored_owner: i64 = sqlx::query_scalar(
        "SELECT owner_epoch FROM cognitive_compact_events
         WHERE journal_id = ? AND sequence = 1",
    )
    .bind("journal:fence-epochs")
    .fetch_one(&store.pool)
    .await
    .expect("owner epoch");
    assert_eq!(stored_authority, 3);
    assert_eq!(stored_owner, 8);

    let authority_changed = store
        .open_local_compact_executor(
            "journal:fence-epochs",
            CompactFence::new(4, 8, 41, "fence:41").expect("authority-changed fence"),
        )
        .await;
    assert!(matches!(
        authority_changed,
        Err(crate::LocalCompactExecutorError::Corrupt(_))
    ));
    let owner_changed = store
        .open_local_compact_executor(
            "journal:fence-epochs",
            CompactFence::new(3, 9, 41, "fence:41").expect("owner-changed fence"),
        )
        .await;
    assert!(matches!(
        owner_changed,
        Err(crate::LocalCompactExecutorError::Corrupt(_))
    ));

    // A v1 row migrated through 0006 has NULL epoch columns.  It must not be
    // guessed or silently adopted by a v2 executor.
    sqlx::query("DROP TRIGGER cognitive_compact_events_no_update")
        .execute(&store.pool)
        .await
        .expect("drop test trigger");
    sqlx::query(
        "UPDATE cognitive_compact_events
         SET authority_epoch = NULL
         WHERE journal_id = 'journal:fence-epochs' AND sequence = 1",
    )
    .execute(&store.pool)
    .await
    .expect("null legacy epoch");
    let legacy_null = store
        .open_local_compact_executor("journal:fence-epochs", current_fence)
        .await;
    assert!(matches!(
        legacy_null,
        Err(crate::LocalCompactExecutorError::Corrupt(_))
    ));
}

#[tokio::test]
async fn rehydration_marker_tamper_fails_closed_on_restart() {
    let temp = TempDir::new().expect("temp dir");
    let owner = agent_id(94);
    let store = CognitiveStore::open(&layout(&temp, &owner))
        .await
        .expect("store");
    let current_fence = fence(33, "fence:33");
    let current = snapshot(current_fence.clone());
    let checkpoint = checkpoint(current_fence.clone());
    let executor = store
        .open_local_compact_executor("journal:rehydration-tamper", current_fence.clone())
        .await
        .expect("executor");
    executor
        .append_intent("op:rehydration-tamper", &checkpoint, &current)
        .await
        .expect("intent");
    let digest = checkpoint_digest(&checkpoint).expect("digest");
    executor
        .commit_checkpoint("op:rehydration-tamper", &digest)
        .await
        .expect("commit");
    executor
        .rehydrate("op:rehydration-tamper", &checkpoint, 0)
        .await
        .expect("rehydrate");

    let event_json: String = sqlx::query_scalar(
        "SELECT event_json FROM cognitive_compact_events
         WHERE journal_id = ? AND sequence = 3",
    )
    .bind("journal:rehydration-tamper")
    .fetch_one(&store.pool)
    .await
    .expect("rehydration event");
    let mut event: serde_json::Value = serde_json::from_str(&event_json).expect("event json");
    event["kind"]["expected_revision"] = serde_json::Value::from(1_u64);
    sqlx::query("DROP TRIGGER cognitive_compact_events_no_update")
        .execute(&store.pool)
        .await
        .expect("drop test trigger");
    sqlx::query(
        "UPDATE cognitive_compact_events SET event_json = ?
         WHERE journal_id = ? AND sequence = 3",
    )
    .bind(serde_json::to_string(&event).expect("tampered event json"))
    .bind("journal:rehydration-tamper")
    .execute(&store.pool)
    .await
    .expect("tamper event");

    let corrupt = store
        .open_local_compact_executor("journal:rehydration-tamper", current_fence)
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
