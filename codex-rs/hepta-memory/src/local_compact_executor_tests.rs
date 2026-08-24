use codex_hepta_contracts::AgentId;
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
use crate::CompactPersistenceJournal;
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

/// Small dependency-free generator for reproducible persistence stress.
///
/// This is deliberately not a fuzz source: a fixed seed keeps the exact
/// reopen/unknown-outcome schedule replayable in CI and in qualification
/// receipts.
struct SeededChoices(u64);

impl SeededChoices {
    fn next(&mut self) -> u64 {
        // xorshift64*: the test seed is non-zero, so the state cannot become
        // trapped at zero.
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_f491_4f6c_dd1d)
    }
}

async fn restart_unbound_executor(
    store: CognitiveStore,
    executor: crate::LocalCompactExecutor,
    temp: &TempDir,
    owner: &AgentId,
    journal_id: &str,
    current_fence: &CompactFence,
) -> (CognitiveStore, crate::LocalCompactExecutor) {
    store.pool.close().await;
    drop(executor);
    drop(store);

    let reopened_store = CognitiveStore::open(&layout(temp, owner))
        .await
        .expect("stress reopen store");
    let reopened_executor = reopened_store
        .open_local_compact_executor(journal_id, current_fence.clone())
        .await
        .expect("stress reopen executor");
    (reopened_store, reopened_executor)
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
async fn bound_lease_terminalization_without_compact_journal_is_allowed() {
    let (_temp, store, lease, _executor, _fence, _checkpoint) =
        open_bound_executor(129, 3_600).await;
    // Opening a bound executor does not create a journal row.  The lifecycle
    // audit must treat that empty case as valid rather than requiring a
    // compact witness that was never started.
    let released = lease.release().await.expect("release without compact rows");
    assert_eq!(released.state, crate::LocalLeaseState::Released);
    let compact_rows: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM cognitive_compact_events")
        .fetch_one(&store.pool)
        .await
        .expect("compact row count");
    assert_eq!(compact_rows, 0);
    store.pool.close().await;
}

#[tokio::test]
async fn terminal_bound_lease_transitions_audit_compact_journal_atomically() {
    // E.25: a damaged compact row must prevent every terminal lease path
    // from appending its terminal row.  The compact journal is tampered only
    // through a test-only trigger drop; the lifecycle transaction itself must
    // still leave the lease row count and compact row count unchanged.
    for (index, transition) in ["release", "rollback", "expire"].into_iter().enumerate() {
        let expiry_offset = if transition == "expire" { 1 } else { 3_600 };
        let (_temp, store, lease, executor, current_fence, checkpoint) =
            open_bound_executor(130 + index as u8, expiry_offset).await;
        let current = snapshot(current_fence);
        executor
            .append_intent("op:terminal-compact-tamper", &checkpoint, &current)
            .await
            .expect("bound compact intent");

        let lease_rows_before: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM cognitive_local_leases WHERE lease_id = ?")
                .bind(lease.lease_id())
                .fetch_one(&store.pool)
                .await
                .expect("lease rows before");
        let compact_rows_before: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM cognitive_compact_events WHERE journal_id = ?",
        )
        .bind(executor.journal_id())
        .fetch_one(&store.pool)
        .await
        .expect("compact rows before");

        // Corrupt the serialized event while retaining the immutable row
        // shape.  The compact loader must reject the digest/metadata mismatch
        // inside the same transaction as terminalization.
        sqlx::query("DROP TRIGGER cognitive_compact_events_no_update")
            .execute(&store.pool)
            .await
            .expect("drop compact update trigger");
        sqlx::query(
            "UPDATE cognitive_compact_events
             SET event_json = '{}'
             WHERE journal_id = ? AND sequence = 1",
        )
        .bind(executor.journal_id())
        .execute(&store.pool)
        .await
        .expect("tamper compact event");

        if transition == "expire" {
            let expiry = lease
                .binding()
                .expect("bound lease binding")
                .lease_expires_at_unix_seconds;
            for _ in 0..120 {
                if unix_seconds() >= expiry {
                    break;
                }
                tokio::time::sleep(Duration::from_millis(25)).await;
            }
            assert!(unix_seconds() >= expiry, "test lease did not expire");
        }

        let result = match transition {
            "release" => lease.release().await.map(|_| ()),
            "rollback" => lease.rollback_lease().await.map(|_| ()),
            "expire" => lease.expire_lease().await.map(|_| ()),
            _ => unreachable!("test transition"),
        };
        assert!(
            matches!(
                result,
                Err(crate::LocalLeaseOutboxError::Corrupt(ref message))
                    if message.contains("compact journal")
            ),
            "{transition} must fail closed on a tampered compact journal: {result:?}"
        );

        let lease_rows_after: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM cognitive_local_leases WHERE lease_id = ?")
                .bind(lease.lease_id())
                .fetch_one(&store.pool)
                .await
                .expect("lease rows after");
        let compact_rows_after: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM cognitive_compact_events WHERE journal_id = ?",
        )
        .bind(executor.journal_id())
        .fetch_one(&store.pool)
        .await
        .expect("compact rows after");
        assert_eq!(
            lease_rows_after, lease_rows_before,
            "{transition} appended lease"
        );
        assert_eq!(
            compact_rows_after, compact_rows_before,
            "{transition} changed compact row count"
        );
        let state: String = sqlx::query_scalar(
            "SELECT state FROM cognitive_local_leases
             WHERE lease_id = ? ORDER BY lease_sequence DESC LIMIT 1",
        )
        .bind(lease.lease_id())
        .fetch_one(&store.pool)
        .await
        .expect("lease state after");
        assert_eq!(state, "active", "{transition} changed lease state");
        store.pool.close().await;
    }
}

#[tokio::test]
async fn terminal_bound_lease_transition_rejects_foreign_compact_row() {
    let (_temp, store, lease, executor, current_fence, checkpoint) =
        open_bound_executor(133, 3_600).await;
    let current = snapshot(current_fence);
    executor
        .append_intent("op:foreign-terminal", &checkpoint, &current)
        .await
        .expect("bound compact intent");

    let event_json: String = sqlx::query_scalar(
        "SELECT event_json FROM cognitive_compact_events
         WHERE journal_id = ? AND sequence = 1",
    )
    .bind(executor.journal_id())
    .fetch_one(&store.pool)
    .await
    .expect("event json");
    let previous_sha256: String = sqlx::query_scalar(
        "SELECT previous_sha256 FROM cognitive_compact_events
         WHERE journal_id = ? AND sequence = 1",
    )
    .bind(executor.journal_id())
    .fetch_one(&store.pool)
    .await
    .expect("previous digest");
    sqlx::query(
        "INSERT INTO cognitive_compact_events (
            journal_id, owner_agent_id, authority_epoch, owner_epoch,
            sequence, generation, fencing_token, event_json,
            previous_sha256, event_sha256, recorded_at_unix_seconds
         ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(executor.journal_id())
    .bind(agent_id(134).as_str())
    .bind(3_i64)
    .bind(8_i64)
    .bind(2_i64)
    .bind(i64::try_from(1_u64).expect("generation"))
    .bind("bound-fence")
    .bind(event_json)
    .bind(previous_sha256)
    .bind("f".repeat(64))
    .bind(i64::try_from(unix_seconds()).expect("timestamp"))
    .execute(&store.pool)
    .await
    .expect("foreign compact row");

    let lease_rows_before: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM cognitive_local_leases WHERE lease_id = ?")
            .bind(lease.lease_id())
            .fetch_one(&store.pool)
            .await
            .expect("lease rows before");
    let result = lease.release().await;
    assert!(matches!(
        result,
        Err(crate::LocalLeaseOutboxError::Corrupt(_))
    ));
    let lease_rows_after: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM cognitive_local_leases WHERE lease_id = ?")
            .bind(lease.lease_id())
            .fetch_one(&store.pool)
            .await
            .expect("lease rows after");
    assert_eq!(lease_rows_after, lease_rows_before);
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

/// Qualification-only persistence stress.  This exercises one thousand
/// unique compact operations against the real Agent-local SQLite journal.
/// Every operation closes and reopens the store at a deterministic seeded
/// state-machine boundary, then idempotently replays a seeded historical
/// operation.  It is intentionally ignored by the ordinary unit gate because
/// loading and verifying the complete append-only chain is quadratic across
/// this many events.
///
/// This is not evidence of one thousand host kills: the test does not kill an
/// OS process, interrupt SQLite syscalls, or exercise agentd supervision.  It
/// proves the narrower deterministic close/reopen/hash-chain/replay boundary.
#[tokio::test]
#[ignore = "qualification stress: 1000 SQLite operations with deterministic reopen/replay"]
async fn sqlite_1000_operation_seeded_reopen_replay_stress() {
    const OPERATIONS: usize = 1_000;
    const SEED: u64 = 0x4845_5054_415f_5250;
    const JOURNAL_ID: &str = "journal:seeded-1000-reopen-replay";

    let temp = TempDir::new().expect("temp dir");
    let owner = agent_id(96);
    let current_fence = fence(77, "fence:seeded-1000");
    let current = snapshot(current_fence.clone());
    let checkpoint = checkpoint(current_fence.clone());
    let checkpoint_sha256 = checkpoint_digest(&checkpoint).expect("checkpoint digest");
    let mut store = CognitiveStore::open(&layout(&temp, &owner))
        .await
        .expect("stress store");
    let mut executor = store
        .open_local_compact_executor(JOURNAL_ID, current_fence.clone())
        .await
        .expect("stress executor");
    let mut choices = SeededChoices(SEED);
    let mut expected_events = 0_usize;
    let mut intent_sequences = Vec::with_capacity(OPERATIONS);
    let mut witness_sequences = Vec::with_capacity(OPERATIONS);
    let mut direct_operations = 0_usize;
    let mut indeterminate_operations = 0_usize;
    let mut operation_reopens = 0_usize;

    for operation_index in 0..OPERATIONS {
        let choice = choices.next();
        let reopen_stage = choice & 0b11;
        let becomes_indeterminate = choice & 0b100 != 0;
        let operation_id = format!("op:seeded:{operation_index:04}");

        let intent_sequence = expected_events as u64 + 1;
        assert_eq!(
            executor
                .append_intent(&operation_id, &checkpoint, &current)
                .await
                .expect("append seeded intent"),
            CompactPersistenceAppend::Appended {
                sequence: intent_sequence,
            }
        );
        expected_events += 1;
        intent_sequences.push(intent_sequence);
        assert_eq!(
            executor
                .append_intent(&operation_id, &checkpoint, &current)
                .await
                .expect("replay seeded intent"),
            CompactPersistenceAppend::Replay {
                sequence: intent_sequence,
            }
        );

        if reopen_stage == 0 {
            (store, executor) = restart_unbound_executor(
                store,
                executor,
                &temp,
                &owner,
                JOURNAL_ID,
                &current_fence,
            )
            .await;
            operation_reopens += 1;
            assert_eq!(
                executor.state(&operation_id).await.expect("pending state"),
                Some(CompactPersistenceState::Pending)
            );
        }

        if becomes_indeterminate {
            indeterminate_operations += 1;
            assert_eq!(
                executor
                    .mark_indeterminate(&operation_id, "seeded-unknown-outcome")
                    .await
                    .expect("mark seeded operation indeterminate"),
                CompactPersistenceAppend::Appended {
                    sequence: expected_events as u64 + 1,
                }
            );
            expected_events += 1;
            if reopen_stage == 1 {
                (store, executor) = restart_unbound_executor(
                    store,
                    executor,
                    &temp,
                    &owner,
                    JOURNAL_ID,
                    &current_fence,
                )
                .await;
                operation_reopens += 1;
                assert_eq!(
                    executor
                        .state(&operation_id)
                        .await
                        .expect("indeterminate state"),
                    Some(CompactPersistenceState::Indeterminate)
                );
            }
            assert_eq!(
                executor
                    .reconcile(&operation_id, CompactReconcileOutcome::Committed)
                    .await
                    .expect("reconcile seeded operation"),
                CompactPersistenceAppend::Appended {
                    sequence: expected_events as u64 + 1,
                }
            );
            expected_events += 1;
        } else {
            direct_operations += 1;
            assert_eq!(
                executor
                    .commit_checkpoint(&operation_id, &checkpoint_sha256)
                    .await
                    .expect("commit seeded operation"),
                CompactPersistenceAppend::Appended {
                    sequence: expected_events as u64 + 1,
                }
            );
            expected_events += 1;
            if reopen_stage == 1 {
                (store, executor) = restart_unbound_executor(
                    store,
                    executor,
                    &temp,
                    &owner,
                    JOURNAL_ID,
                    &current_fence,
                )
                .await;
                operation_reopens += 1;
            }
        }

        if reopen_stage == 2 {
            (store, executor) = restart_unbound_executor(
                store,
                executor,
                &temp,
                &owner,
                JOURNAL_ID,
                &current_fence,
            )
            .await;
            operation_reopens += 1;
        }
        assert_eq!(
            executor
                .state(&operation_id)
                .await
                .expect("committed state"),
            Some(CompactPersistenceState::Committed)
        );
        assert_eq!(
            executor
                .rehydrate(&operation_id, &checkpoint, 0)
                .await
                .expect("rehydrate seeded operation")
                .status,
            crate::RehydrationStatus::Complete
        );
        expected_events += 1;
        witness_sequences.push(expected_events as u64);

        if reopen_stage == 3 {
            (store, executor) = restart_unbound_executor(
                store,
                executor,
                &temp,
                &owner,
                JOURNAL_ID,
                &current_fence,
            )
            .await;
            operation_reopens += 1;
        }

        // Replay a seeded historical operation after this operation's chosen
        // reopen boundary.  None of these calls may append another row or
        // advance the digest-chain head.
        let replay_index = (choices.next() as usize) % (operation_index + 1);
        let replay_operation_id = format!("op:seeded:{replay_index:04}");
        let before_replay = executor.snapshot().await.expect("before replay snapshot");
        assert_eq!(before_replay.entries.len(), expected_events);
        assert_eq!(
            executor
                .append_intent(&replay_operation_id, &checkpoint, &current)
                .await
                .expect("replay historical intent"),
            CompactPersistenceAppend::Replay {
                sequence: intent_sequences[replay_index],
            }
        );
        assert_eq!(
            executor
                .commit_checkpoint(&replay_operation_id, &checkpoint_sha256)
                .await
                .expect("replay historical commit"),
            CompactPersistenceAppend::Replay {
                sequence: witness_sequences[replay_index],
            }
        );
        assert_eq!(
            executor
                .rehydrate(&replay_operation_id, &checkpoint, 0)
                .await
                .expect("replay historical rehydration")
                .status,
            crate::RehydrationStatus::Complete
        );
        let after_replay = executor.snapshot().await.expect("after replay snapshot");
        assert_eq!(after_replay.entries.len(), expected_events);
        assert_eq!(after_replay.head_sha256, before_replay.head_sha256);
    }

    assert_eq!(operation_reopens, OPERATIONS);
    assert!(direct_operations > 0);
    assert!(indeterminate_operations > 0);
    assert_eq!(expected_events, OPERATIONS * 3 + indeterminate_operations);
    // The executor rejects journals over 4096 rows on reopen.  The fixed
    // schedule leaves at least 96 rows of margin below that production guard.
    assert!(expected_events <= 4_000);

    let before_final_reopen = executor.snapshot().await.expect("final stress snapshot");
    assert_eq!(before_final_reopen.entries.len(), expected_events);
    (store, executor) =
        restart_unbound_executor(store, executor, &temp, &owner, JOURNAL_ID, &current_fence).await;
    let after_final_reopen = executor
        .snapshot()
        .await
        .expect("post-restart stress snapshot");
    assert_eq!(after_final_reopen.entries, before_final_reopen.entries);
    assert_eq!(
        after_final_reopen.head_sha256,
        before_final_reopen.head_sha256
    );
    let reopened = CompactPersistenceJournal::reopen(after_final_reopen.clone())
        .expect("reopen final in-memory journal");
    for operation_index in 0..OPERATIONS {
        let operation_id = format!("op:seeded:{operation_index:04}");
        assert_eq!(
            reopened.state(&operation_id),
            Some(CompactPersistenceState::Committed)
        );
        assert_eq!(
            reopened
                .rehydration(&operation_id)
                .expect("final rehydration witness")
                .sequence,
            witness_sequences[operation_index]
        );
    }
    let row_count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM cognitive_compact_events WHERE journal_id = ?")
            .bind(JOURNAL_ID)
            .fetch_one(&store.pool)
            .await
            .expect("final compact event row count");
    assert_eq!(row_count, expected_events as i64);
    store.pool.close().await;
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
async fn compact_reopen_rejects_foreign_owner_row_sharing_journal_id() {
    let temp = TempDir::new().expect("temp dir");
    let owner = agent_id(97);
    let store = CognitiveStore::open(&layout(&temp, &owner))
        .await
        .expect("store");
    let current_fence = fence(43, "fence:foreign-owner");
    let current = snapshot(current_fence.clone());
    let checkpoint = checkpoint(current_fence.clone());
    let executor = store
        .open_local_compact_executor("journal:foreign-owner", current_fence.clone())
        .await
        .expect("executor");
    executor
        .append_intent("op:foreign-owner", &checkpoint, &current)
        .await
        .expect("intent");

    // Test-only insertion: the foreign row uses a distinct event digest and
    // a later sequence, so an owner-filtered loader would silently ignore it.
    let event_json: String = sqlx::query_scalar(
        "SELECT event_json FROM cognitive_compact_events
         WHERE journal_id = ? AND sequence = 1",
    )
    .bind("journal:foreign-owner")
    .fetch_one(&store.pool)
    .await
    .expect("event json");
    let previous_sha256: String = sqlx::query_scalar(
        "SELECT previous_sha256 FROM cognitive_compact_events
         WHERE journal_id = ? AND sequence = 1",
    )
    .bind("journal:foreign-owner")
    .fetch_one(&store.pool)
    .await
    .expect("previous digest");
    let foreign_owner = agent_id(98);
    sqlx::query(
        "INSERT INTO cognitive_compact_events (
            journal_id, owner_agent_id, authority_epoch, owner_epoch,
            sequence, generation, fencing_token, event_json,
            previous_sha256, event_sha256, recorded_at_unix_seconds
         ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind("journal:foreign-owner")
    .bind(foreign_owner.as_str())
    .bind(i64::from(current_fence.authority_epoch as u32))
    .bind(i64::from(current_fence.owner_epoch as u32))
    .bind(2_i64)
    .bind(i64::try_from(current_fence.generation).expect("generation"))
    .bind(&current_fence.fencing_token)
    .bind(event_json)
    .bind(previous_sha256)
    .bind("f".repeat(64))
    .bind(i64::try_from(unix_seconds()).expect("timestamp"))
    .execute(&store.pool)
    .await
    .expect("foreign compact row");

    let reopened = store
        .open_local_compact_executor("journal:foreign-owner", current_fence)
        .await;
    assert!(matches!(
        reopened,
        Err(crate::LocalCompactExecutorError::Corrupt(message))
            if message.contains("owner")
    ));
    let audit = crate::local_compact_executor::verify_local_compact_events(
        &store.pool,
        store.owner_agent_id(),
    )
    .await;
    assert!(
        matches!(audit, Err(crate::CognitiveStoreError::Corrupt(message)) if message.contains("owner"))
    );
}

#[tokio::test]
async fn compact_reopen_rejects_orphan_bound_lease_head() {
    let (_temp, store, _lease, executor, current_fence, checkpoint) =
        open_bound_executor(99, 3_600).await;
    let current = snapshot(current_fence);
    executor
        .append_intent("op:orphan-bound", &checkpoint, &current)
        .await
        .expect("bound intent");

    // Test-only tamper: preserve the row's shape but point its immutable
    // binding at a lease head that was never granted by this owner/fence.
    sqlx::query("DROP TRIGGER cognitive_compact_events_no_update")
        .execute(&store.pool)
        .await
        .expect("drop test trigger");
    sqlx::query(
        "UPDATE cognitive_compact_events
         SET lease_id = 'lease:does-not-exist'
         WHERE journal_id = ? AND sequence = 1",
    )
    .bind(executor.journal_id())
    .execute(&store.pool)
    .await
    .expect("orphan lease id");

    let audit = crate::local_compact_executor::verify_local_compact_events(
        &store.pool,
        store.owner_agent_id(),
    )
    .await;
    assert!(
        matches!(audit, Err(crate::CognitiveStoreError::Corrupt(message)) if message.contains("historical lease head"))
    );
}

#[tokio::test]
async fn bound_compact_journal_survives_terminal_lease_and_store_reopen() {
    let (temp, store, lease, executor, current_fence, checkpoint) =
        open_bound_executor(100, 3_600).await;
    let current = snapshot(current_fence);
    executor
        .append_intent("op:terminal-reopen", &checkpoint, &current)
        .await
        .expect("bound intent");
    lease.release().await.expect("explicit terminal release");
    store.pool.close().await;
    drop(executor);
    drop(lease);
    drop(store);

    // The compact event remains valid historical evidence after the host has
    // explicitly terminalized the lease; reopening must not require an
    // active lease or silently discard the bound journal.
    let reopened_store = CognitiveStore::open(&layout(&temp, &agent_id(100)))
        .await
        .expect("reopen store with terminal lease");
    let audit = crate::local_compact_executor::verify_local_compact_events(
        &reopened_store.pool,
        reopened_store.owner_agent_id(),
    )
    .await;
    assert!(audit.is_ok(), "terminal bound compact journal must verify");
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
