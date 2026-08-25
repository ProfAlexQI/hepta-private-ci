use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use codex_hepta_contracts::AgentId;
use codex_hepta_contracts::Sha256Digest;
use codex_hepta_paths::HeptaFleetRoot;
use tempfile::TempDir;

use crate::CognitiveStore;
use crate::CompactFence;
use crate::H7TrajectoryAppend;
use crate::H7TrajectoryRecord;
use crate::H7TrajectoryStoreError;
use crate::LocalTurnLifecycleBinding;
use crate::append_h7_trajectory_event_bound;
use crate::read_h7_trajectory_bound;

fn digest(label: &str) -> Sha256Digest {
    Sha256Digest::for_bytes(label.as_bytes())
}

async fn prepared() -> (
    TempDir,
    CognitiveStore,
    crate::LocalLeaseOutbox,
    crate::LocalCompactExecutor,
    LocalTurnLifecycleBinding,
) {
    let temp = TempDir::new().expect("temp");
    let root = temp.path().join("fleet");
    fs::create_dir_all(&root).expect("fleet root");
    let fleet = HeptaFleetRoot::parse(root).expect("fleet");
    let owner = AgentId::parse("00000000-0000-4000-8000-000000000971").expect("owner");
    let store = CognitiveStore::open(&fleet.layout().agent(&owner))
        .await
        .expect("store");
    let fence = CompactFence::new(31, 37, 1, "h7-trajectory-fence").expect("fence");
    let lease = store
        .acquire_host_bound_lease(
            "lease:h7-trajectory",
            fence.authority_epoch,
            fence.owner_epoch,
            fence.generation,
            fence.fencing_token.clone(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("clock")
                .as_secs()
                + 3_600,
        )
        .await
        .expect("lease")
        .into_handle();
    let executor = store
        .open_local_compact_executor_bound("journal:h7-trajectory", fence, &lease)
        .await
        .expect("executor");
    let binding = LocalTurnLifecycleBinding::from_handles("turn:h7-trajectory", &lease, &executor)
        .expect("binding");
    (temp, store, lease, executor, binding)
}

fn start_record() -> H7TrajectoryRecord {
    H7TrajectoryRecord::turn_start(
        "trajectory:h7-trajectory",
        "event:h7-trajectory:1",
        "turn:h7-trajectory",
        "occurrence:h7-trajectory:start",
        digest("state:1"),
        digest("policy:none"),
        digest("model:none"),
        digest("receipt:start"),
        "{}",
    )
    .expect("start record")
}

#[tokio::test]
async fn bound_trajectory_is_append_only_causal_and_reopenable() {
    let (temp, store, lease, executor, binding) = prepared().await;
    let start = start_record();
    let start_result = append_h7_trajectory_event_bound(&lease, &executor, &binding, &start)
        .await
        .expect("append start");
    let H7TrajectoryAppend::Inserted {
        event_sha256: parent,
        ..
    } = start_result
    else {
        panic!("first event must be inserted")
    };
    let terminal = H7TrajectoryRecord::terminal(
        "trajectory:h7-trajectory",
        2,
        "event:h7-trajectory:2",
        "turn:h7-trajectory",
        "occurrence:h7-trajectory:terminal",
        1,
        parent,
        digest("state:2"),
        digest("policy:none"),
        digest("model:none"),
        digest("receipt:terminal"),
        "stopped",
        "turn_stopped",
        "{\"terminal\":true}",
    )
    .expect("terminal record");
    let terminal_result = append_h7_trajectory_event_bound(&lease, &executor, &binding, &terminal)
        .await
        .expect("append terminal");
    assert!(matches!(
        terminal_result,
        H7TrajectoryAppend::Inserted { .. }
    ));
    let replay = append_h7_trajectory_event_bound(&lease, &executor, &binding, &terminal)
        .await
        .expect("terminal replay");
    let replay_hash = match &replay {
        H7TrajectoryAppend::Replay {
            event_seq: 2,
            event_sha256,
        } => event_sha256.clone(),
        other => panic!("terminal replay expected, got {other:?}"),
    };
    let after_terminal = H7TrajectoryRecord::terminal(
        "trajectory:h7-trajectory",
        3,
        "event:h7-trajectory:3",
        "turn:h7-trajectory",
        "occurrence:h7-trajectory:after-terminal",
        2,
        replay_hash,
        digest("state:3"),
        digest("policy:none"),
        digest("model:none"),
        digest("receipt:after-terminal"),
        "stopped",
        "turn_stopped_again",
        "{}",
    )
    .expect("after-terminal record");
    assert!(matches!(
        append_h7_trajectory_event_bound(&lease, &executor, &binding, &after_terminal).await,
        Err(H7TrajectoryStoreError::CasConflict(message))
            if message.contains("already terminal")
    ));

    let read = store
        .read_h7_trajectory("trajectory:h7-trajectory")
        .await
        .expect("read trajectory")
        .expect("trajectory exists");
    assert_eq!(read.events, vec![start, terminal]);
    assert_eq!(read.events.len(), 2);

    lease.release().await.expect("release");
    drop(executor);
    drop(lease);
    drop(store);
    let owner = AgentId::parse("00000000-0000-4000-8000-000000000971").expect("owner");
    let fleet = HeptaFleetRoot::parse(temp.path().join("fleet")).expect("fleet reopen");
    let reopened = CognitiveStore::open(&fleet.layout().agent(&owner))
        .await
        .expect("reopen store");
    let reopened_read = reopened
        .read_h7_trajectory("trajectory:h7-trajectory")
        .await
        .expect("reopen read")
        .expect("reopened trajectory");
    assert_eq!(reopened_read.events.len(), 2);
}

#[tokio::test]
async fn bound_read_rejects_terminal_trajectory_from_prior_lease_head() {
    let (_temp, store, lease, executor, binding) = prepared().await;
    let start = start_record();
    let H7TrajectoryAppend::Inserted {
        event_sha256: parent,
        ..
    } = append_h7_trajectory_event_bound(&lease, &executor, &binding, &start)
        .await
        .expect("append start")
    else {
        panic!("first event must be inserted")
    };
    let terminal = H7TrajectoryRecord::terminal(
        "trajectory:h7-trajectory",
        2,
        "event:h7-trajectory:2",
        "turn:h7-trajectory",
        "occurrence:h7-trajectory:terminal",
        1,
        parent,
        digest("state:2"),
        digest("policy:none"),
        digest("model:none"),
        digest("receipt:terminal"),
        "stopped",
        "turn_stopped",
        "{}",
    )
    .expect("terminal record");
    append_h7_trajectory_event_bound(&lease, &executor, &binding, &terminal)
        .await
        .expect("append terminal");
    let released = lease.release().await.expect("release generation one");

    let next = store
        .acquire_host_bound_lease_after_head(
            "lease:h7-trajectory",
            released,
            31,
            38,
            2,
            "h7-trajectory-fence-2",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("clock")
                .as_secs()
                + 3_600,
        )
        .await
        .expect("acquire generation two")
        .into_handle();
    let fence = CompactFence::new(31, 38, 2, "h7-trajectory-fence-2").expect("fence");
    let next_executor = store
        .open_local_compact_executor_bound("journal:h7-trajectory-2", fence, &next)
        .await
        .expect("executor generation two");
    let next_binding =
        LocalTurnLifecycleBinding::from_handles("turn:h7-trajectory", &next, &next_executor)
            .expect("binding generation two");
    let error = read_h7_trajectory_bound(
        &next,
        &next_executor,
        &next_binding,
        "trajectory:h7-trajectory",
    )
    .await
    .expect_err("prior terminal head must not be reused by a new generation");
    assert!(matches!(
        error,
        H7TrajectoryStoreError::StaleFence(message)
            if message.contains("does not match the current lifecycle binding")
    ));
}

#[tokio::test]
async fn trajectory_rejects_mixed_generation_event_chain() {
    let (_temp, store, lease, executor, binding) = prepared().await;
    let start = start_record();
    let H7TrajectoryAppend::Inserted {
        event_sha256: parent,
        ..
    } = append_h7_trajectory_event_bound(&lease, &executor, &binding, &start)
        .await
        .expect("append start")
    else {
        panic!("first event must be inserted")
    };
    let released = lease.release().await.expect("release generation one");
    let next = store
        .acquire_host_bound_lease_after_head(
            "lease:h7-trajectory",
            released,
            31,
            38,
            2,
            "h7-trajectory-fence-2",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("clock")
                .as_secs()
                + 3_600,
        )
        .await
        .expect("acquire generation two")
        .into_handle();
    let fence = CompactFence::new(31, 38, 2, "h7-trajectory-fence-2").expect("fence");
    let next_executor = store
        .open_local_compact_executor_bound("journal:h7-trajectory-2", fence, &next)
        .await
        .expect("executor generation two");
    let next_binding =
        LocalTurnLifecycleBinding::from_handles("turn:h7-trajectory", &next, &next_executor)
            .expect("binding generation two");
    let terminal = H7TrajectoryRecord::terminal(
        "trajectory:h7-trajectory",
        2,
        "event:h7-trajectory:2",
        "turn:h7-trajectory",
        "occurrence:h7-trajectory:terminal",
        1,
        parent,
        digest("state:2"),
        digest("policy:none"),
        digest("model:none"),
        digest("receipt:terminal"),
        "stopped",
        "turn_stopped",
        "{}",
    )
    .expect("terminal record");
    let error = append_h7_trajectory_event_bound(&next, &next_executor, &next_binding, &terminal)
        .await
        .expect_err("one trajectory cannot cross lease generations");
    assert!(matches!(
        error,
        H7TrajectoryStoreError::Corrupt(message)
            if message.contains("does not match the current lifecycle binding")
    ));
    next.release().await.expect("release generation two");
}

#[tokio::test]
async fn trajectory_rejects_gap_parent_policy_and_stale_binding() {
    let (_temp, store, lease, executor, binding) = prepared().await;
    let start = start_record();
    append_h7_trajectory_event_bound(&lease, &executor, &binding, &start)
        .await
        .expect("start");
    let mut gap = H7TrajectoryRecord::terminal(
        "trajectory:h7-trajectory",
        3,
        "event:h7-trajectory:3",
        "turn:h7-trajectory",
        "occurrence:h7-trajectory:gap",
        2,
        digest("wrong-parent"),
        digest("state:3"),
        digest("policy:none"),
        digest("model:none"),
        digest("receipt:gap"),
        "stopped",
        "turn_stopped",
        "{}",
    )
    .expect("gap record");
    assert!(matches!(
        append_h7_trajectory_event_bound(&lease, &executor, &binding, &gap).await,
        Err(H7TrajectoryStoreError::CasConflict(_))
    ));
    gap.propensity_json = Some("{}".to_string());
    assert!(matches!(
        gap.validate(),
        Err(H7TrajectoryStoreError::PolicyActionNotQualified)
    ));
    lease.release().await.expect("release");
    let stale = H7TrajectoryRecord::terminal(
        "trajectory:h7-trajectory",
        2,
        "event:h7-trajectory:2",
        "turn:h7-trajectory",
        "occurrence:h7-trajectory:stale",
        1,
        digest("parent"),
        digest("state:2"),
        digest("policy:none"),
        digest("model:none"),
        digest("receipt:stale"),
        "stopped",
        "turn_stopped",
        "{}",
    )
    .expect("stale record");
    assert!(matches!(
        append_h7_trajectory_event_bound(&lease, &executor, &binding, &stale).await,
        Err(H7TrajectoryStoreError::Binding(_))
            | Err(H7TrajectoryStoreError::StaleFence(_))
            | Err(H7TrajectoryStoreError::Lease(_))
    ));
    assert!(
        store
            .read_h7_trajectory("trajectory:h7-trajectory")
            .await
            .expect("read after stale")
            .is_some()
    );
}

#[test]
fn trajectory_record_rejects_non_observation_flags() {
    let mut record = start_record();
    record.production_caller = true;
    assert!(matches!(
        record.validate(),
        Err(H7TrajectoryStoreError::BoundaryViolation)
    ));

    let feedback = H7TrajectoryRecord::new(
        "trajectory:h7-feedback",
        2,
        "event:h7-feedback:2",
        crate::H7TrajectoryEventKind::Feedback,
        "turn:h7-feedback",
        "occurrence:h7-feedback",
        Some(1),
        Some(digest("feedback-parent")),
        digest("state:feedback"),
        digest("policy:feedback"),
        digest("model:feedback"),
        digest("receipt:feedback"),
        "feedback",
        0,
        true,
        "{}",
        "not_applicable",
    )
    .expect_err("untyped feedback must stay outside the local observation slice");
    assert!(matches!(
        feedback,
        H7TrajectoryStoreError::PolicyActionNotQualified
    ));

    let mut rewarded = start_record();
    rewarded.reward_bps = 1;
    assert!(matches!(
        rewarded.validate(),
        Err(H7TrajectoryStoreError::PolicyActionNotQualified)
    ));
}
