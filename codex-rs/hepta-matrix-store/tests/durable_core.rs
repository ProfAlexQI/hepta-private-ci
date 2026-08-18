use std::error::Error;
use std::fs;

use codex_hepta_contracts::AgentId;
use codex_hepta_matrix_protocol::MatrixEventId;
use codex_hepta_matrix_protocol::MatrixRoomId;
use codex_hepta_matrix_protocol::MatrixTransactionId;
use codex_hepta_matrix_protocol::MatrixUserId;
use codex_hepta_matrix_protocol::client_user_message_id;
use codex_hepta_matrix_protocol::outbox_id as protocol_outbox_id;
use codex_hepta_matrix_protocol::room_project_idempotency_key;
use codex_hepta_matrix_protocol::transaction_id;
use codex_hepta_matrix_store::InboxAdmissionDraft;
use codex_hepta_matrix_store::InboxDispatchState;
use codex_hepta_matrix_store::InboxDisposition;
use codex_hepta_matrix_store::InboxDraft;
use codex_hepta_matrix_store::InboxQueuedDraft;
use codex_hepta_matrix_store::MatrixDurableConfig;
use codex_hepta_matrix_store::MatrixDurableError;
use codex_hepta_matrix_store::MatrixDurableStore;
use codex_hepta_matrix_store::OutboxDisposition;
use codex_hepta_matrix_store::OutboxDraft;
use codex_hepta_matrix_store::OutboxKind;
use codex_hepta_matrix_store::OutboxState;
use codex_hepta_matrix_store::RoomBindingDraft;
use codex_hepta_matrix_store::RoomThreadBindingDraft;
use codex_hepta_paths::HeptaAgentLayout;
use codex_hepta_paths::HeptaFleetRoot;
use pretty_assertions::assert_eq;
use tempfile::TempDir;

type TestResult<T = ()> = Result<T, Box<dyn Error>>;

const FIRST_AGENT: &str = "018f4f72-5f8f-7cc1-8f55-df9fb3aa2c12";
const SECOND_AGENT: &str = "019153a4-3088-7e03-a56a-9b1964f75dd3";
const THIRD_AGENT: &str = "019153a4-3088-7e03-a56a-9b1964f75dd4";

fn agent(value: &str) -> TestResult<AgentId> {
    Ok(AgentId::parse(value)?)
}

fn layout(temp: &TempDir, agent_id: &AgentId) -> TestResult<HeptaAgentLayout> {
    let fleet_root = temp.path().join("fleet");
    fs::create_dir_all(&fleet_root)?;
    let canonical = fleet_root.canonicalize()?;
    Ok(HeptaFleetRoot::parse(canonical)?.layout().agent(agent_id))
}

fn room(value: &str) -> TestResult<MatrixRoomId> {
    Ok(MatrixRoomId::parse(value)?)
}

fn user(value: &str) -> TestResult<MatrixUserId> {
    Ok(MatrixUserId::parse(value)?)
}

fn event(value: &str) -> TestResult<MatrixEventId> {
    Ok(MatrixEventId::parse(value)?)
}

async fn bind_room(
    store: &MatrixDurableStore,
    room_id: &MatrixRoomId,
    agent_user_id: &MatrixUserId,
    at_ms: u64,
) -> TestResult {
    let binding = store
        .bind_room(&RoomBindingDraft {
            room_id: room_id.clone(),
            agent_user_id: agent_user_id.clone(),
            expected_revision: None,
            generation: 1,
            changed_at_ms: at_ms,
        })
        .await?;
    assert_eq!(binding.revision, 1);
    assert_eq!(binding.generation, 1);
    Ok(())
}

async fn prepare_room_project(
    store: &MatrixDurableStore,
    agent_id: &AgentId,
    room_id: &MatrixRoomId,
    thread_id: Option<&str>,
    at_ms: u64,
) -> TestResult<String> {
    let project_id = room_project_idempotency_key(agent_id, room_id);
    let binding = store
        .bind_room_thread(&RoomThreadBindingDraft {
            room_id: room_id.clone(),
            binding_revision: 1,
            generation: 1,
            project_id: project_id.clone(),
            thread_id: thread_id.map(ToOwned::to_owned),
            changed_at_ms: at_ms,
        })
        .await?;
    assert_eq!(binding.project_id, project_id);
    Ok(project_id)
}

fn inbox_draft(
    event_id: MatrixEventId,
    room_id: MatrixRoomId,
    payload: &[u8],
    at_ms: u64,
) -> TestResult<InboxDraft> {
    Ok(InboxDraft {
        event_id,
        room_id,
        sender: user("@owner:example.test")?,
        event_type: "m.room.message".to_string(),
        payload: payload.to_vec(),
        binding_revision: 1,
        generation: 1,
        origin_server_ts_ms: at_ms,
        received_at_ms: at_ms + 1,
    })
}

fn outbox_draft(
    agent_id: &AgentId,
    room_id: &MatrixRoomId,
    item_id: &str,
    kind: OutboxKind,
    payload: &[u8],
    created_at_ms: u64,
) -> TestResult<OutboxDraft> {
    let kind_name = match kind {
        OutboxKind::TextDelta => "text_delta",
        OutboxKind::Final => "final",
        OutboxKind::ToolTransition => "tool_transition",
        OutboxKind::Approval => "approval",
        OutboxKind::Terminal => "terminal",
    };
    let logical_outbox_id =
        protocol_outbox_id(agent_id, room_id, "thread-1", "turn-1", item_id, kind_name);
    let txn_id = transaction_id(&logical_outbox_id, 1)?;
    Ok(OutboxDraft {
        logical_outbox_id,
        revision: 1,
        txn_id,
        room_id: room_id.clone(),
        kind,
        payload: payload.to_vec(),
        binding_revision: 1,
        generation: 1,
        created_at_ms,
    })
}

#[tokio::test]
async fn two_agents_never_share_stream_or_owner_database() -> TestResult {
    let temp = TempDir::new()?;
    let first = agent(FIRST_AGENT)?;
    let second = agent(SECOND_AGENT)?;
    let first_layout = layout(&temp, &first)?;
    let second_layout = layout(&temp, &second)?;
    let first_store =
        MatrixDurableStore::open(&first_layout, MatrixDurableConfig::default()).await?;
    let second_store =
        MatrixDurableStore::open(&second_layout, MatrixDurableConfig::default()).await?;
    let first_room = room("!first:example.test")?;
    let second_room = room("!second:example.test")?;
    bind_room(
        &first_store,
        &first_room,
        &user("@first-agent:example.test")?,
        10,
    )
    .await?;
    bind_room(
        &second_store,
        &second_room,
        &user("@second-agent:example.test")?,
        10,
    )
    .await?;
    let first_event = event("$first-event")?;
    first_store
        .ingest_inbox(&inbox_draft(
            first_event.clone(),
            first_room.clone(),
            b"only first",
            20,
        )?)
        .await?;

    assert_eq!(first_store.pending_inbox(10).await?.len(), 1);
    assert!(second_store.pending_inbox(10).await?.is_empty());
    assert!(second_store.room_binding(&first_room).await?.is_none());
    assert!(first_store.room_binding(&second_room).await?.is_none());

    first_store.close().await;
    second_store.close().await;
    let third = agent(THIRD_AGENT)?;
    let third_layout = layout(&temp, &third)?;
    fs::create_dir_all(third_layout.matrix_root())?;
    fs::copy(
        first_layout.matrix_root().join("matrix_1.sqlite3"),
        third_layout.matrix_root().join("matrix_1.sqlite3"),
    )?;
    let foreign = MatrixDurableStore::open(&third_layout, MatrixDurableConfig::default()).await;
    assert!(matches!(foreign, Err(MatrixDurableError::AccessDenied)));
    Ok(())
}

#[tokio::test]
async fn duplicate_event_and_transaction_are_exact_or_conflict() -> TestResult {
    let temp = TempDir::new()?;
    let agent_id = agent(FIRST_AGENT)?;
    let layout = layout(&temp, &agent_id)?;
    let store = MatrixDurableStore::open(&layout, MatrixDurableConfig::default()).await?;
    let room_id = room("!idempotent:example.test")?;
    bind_room(&store, &room_id, &user("@agent:example.test")?, 10).await?;
    let event_id = event("$same-event")?;
    let inbox = inbox_draft(event_id, room_id.clone(), b"SECRET-INBOX", 20)?;
    assert!(matches!(
        store.ingest_inbox(&inbox).await?,
        InboxDisposition::Accepted(_)
    ));
    assert!(matches!(
        store.ingest_inbox(&inbox).await?,
        InboxDisposition::Duplicate(_)
    ));
    let mut conflicting_inbox = inbox.clone();
    conflicting_inbox.payload = b"different".to_vec();
    assert_eq!(
        store.ingest_inbox(&conflicting_inbox).await,
        Err(MatrixDurableError::Conflict)
    );

    let outbox = outbox_draft(
        &agent_id,
        &room_id,
        "same-item",
        OutboxKind::Final,
        b"SECRET-OUTBOX",
        30,
    )?;
    assert!(matches!(
        store.enqueue_outbox(&outbox).await?,
        OutboxDisposition::Enqueued(_)
    ));
    assert!(matches!(
        store.enqueue_outbox(&outbox).await?,
        OutboxDisposition::Duplicate(_)
    ));
    let mut conflicting_outbox = outbox.clone();
    conflicting_outbox.payload = b"different".to_vec();
    assert_eq!(
        store.enqueue_outbox(&conflicting_outbox).await,
        Err(MatrixDurableError::Conflict)
    );

    assert!(!format!("{inbox:?}").contains("SECRET-INBOX"));
    assert!(!format!("{outbox:?}").contains("SECRET-OUTBOX"));
    assert!(!MatrixDurableError::Conflict.to_string().contains("SECRET"));
    Ok(())
}

#[tokio::test]
async fn ten_thousand_deltas_are_bounded_and_final_is_exact() -> TestResult {
    let temp = TempDir::new()?;
    let agent_id = agent(FIRST_AGENT)?;
    let layout = layout(&temp, &agent_id)?;
    let store = MatrixDurableStore::open(
        &layout,
        MatrixDurableConfig {
            delta_coalesce_window_ms: 150,
            max_delta_batch_bytes: 256,
            event_capacity: 65_536,
        },
    )
    .await?;
    let room_id = room("!bounded:example.test")?;
    bind_room(&store, &room_id, &user("@agent:example.test")?, 10).await?;

    let logical_delta = protocol_outbox_id(
        &agent_id,
        &room_id,
        "thread-1",
        "turn-1",
        "delta-stream",
        "text_delta",
    );
    for index in 0..10_000_u64 {
        let revision = index + 1;
        let draft = OutboxDraft {
            logical_outbox_id: logical_delta.clone(),
            revision,
            txn_id: transaction_id(&logical_delta, revision)?,
            room_id: room_id.clone(),
            kind: OutboxKind::TextDelta,
            payload: vec![b'x'],
            binding_revision: 1,
            generation: 1,
            created_at_ms: 20,
        };
        store.enqueue_outbox(&draft).await?;
    }
    let final_draft = outbox_draft(
        &agent_id,
        &room_id,
        "final",
        OutboxKind::Final,
        b"EXACT-FINAL",
        21,
    )?;
    store.enqueue_outbox(&final_draft).await?;

    let records = store.pending_outbox(100).await?;
    let deltas: Vec<_> = records
        .iter()
        .filter(|record| record.kind == OutboxKind::TextDelta)
        .collect();
    assert_eq!(deltas.len(), 40);
    assert!(deltas.iter().all(|record| record.payload.len() <= 256));
    assert_eq!(
        deltas
            .iter()
            .map(|record| record.logical_txn_count)
            .sum::<u64>(),
        10_000
    );
    assert_eq!(
        deltas
            .iter()
            .flat_map(|record| record.payload.iter().copied())
            .collect::<Vec<_>>(),
        vec![b'x'; 10_000]
    );
    let finals: Vec<_> = records
        .iter()
        .filter(|record| record.kind == OutboxKind::Final)
        .collect();
    assert_eq!(finals.len(), 1);
    assert_eq!(finals[0].payload, b"EXACT-FINAL");
    assert!(OutboxKind::Final.is_critical());
    Ok(())
}

#[tokio::test]
async fn crash_reopen_reconciles_dispatch_and_expired_outbox_lease() -> TestResult {
    let temp = TempDir::new()?;
    let agent_id = agent(FIRST_AGENT)?;
    let layout = layout(&temp, &agent_id)?;
    let room_id = room("!recovery:example.test")?;
    let event_id = event("$recover-event")?;
    let store = MatrixDurableStore::open(&layout, MatrixDurableConfig::default()).await?;
    bind_room(&store, &room_id, &user("@agent:example.test")?, 10).await?;
    let project_id = prepare_room_project(&store, &agent_id, &room_id, None, 11).await?;
    store
        .ingest_inbox(&inbox_draft(
            event_id.clone(),
            room_id.clone(),
            b"recover me",
            20,
        )?)
        .await?;
    let begun = store.begin_inbox_dispatch(&event_id, 22).await?;
    assert_eq!(
        begun.client_user_message_id,
        client_user_message_id(&agent_id, &room_id, &event_id)
    );
    assert_eq!(begun.state, InboxDispatchState::Begun);
    assert_eq!(begun.project_id, project_id);
    assert!(begun.thread_id.is_none());

    let outbox = outbox_draft(
        &agent_id,
        &room_id,
        "recover-outbox",
        OutboxKind::Terminal,
        b"terminal",
        23,
    )?;
    store.enqueue_outbox(&outbox).await?;
    let first_claim = store.claim_outbox(24, 5, 10).await?;
    assert_eq!(first_claim.len(), 1);
    assert_eq!(first_claim[0].attempts, 1);
    store.close().await;

    let reopened = MatrixDurableStore::open(&layout, MatrixDurableConfig::default()).await?;
    let pending = reopened.pending_dispatches(10).await?;
    assert_eq!(pending.len(), 1);
    assert_eq!(pending[0], begun);
    let queued = InboxQueuedDraft {
        event_id: event_id.clone(),
        client_user_message_id: begun.client_user_message_id.clone(),
        project_id: project_id.clone(),
        thread_id: "thread-recovered".to_string(),
        queued_submission_id: "queued-recovered".to_string(),
        queued_at_ms: 30,
    };
    let queued_record = reopened.record_inbox_queued(&queued).await?;
    assert_eq!(queued_record.state, InboxDispatchState::Queued);
    assert_eq!(
        reopened.record_inbox_queued(&queued).await?.state,
        InboxDispatchState::Queued
    );
    let admission = InboxAdmissionDraft {
        event_id: event_id.clone(),
        client_user_message_id: begun.client_user_message_id.clone(),
        project_id: project_id.clone(),
        thread_id: "thread-recovered".to_string(),
        queued_submission_id: Some("queued-recovered".to_string()),
        turn_id: "turn-recovered".to_string(),
        admitted_at_ms: 40,
    };
    assert_eq!(
        reopened.complete_inbox_dispatch(&admission, 41).await,
        Err(MatrixDurableError::Conflict),
        "queued work must never masquerade as an admitted turn"
    );
    let mut conflicting_queued = queued.clone();
    conflicting_queued.thread_id = "other-thread".to_string();
    assert_eq!(
        reopened.record_inbox_queued(&conflicting_queued).await,
        Err(MatrixDurableError::Conflict)
    );
    assert_eq!(
        reopened
            .room_thread(&room_id, 1, 1)
            .await?
            .ok_or("missing room thread")?
            .thread_id
            .as_deref(),
        Some("thread-recovered")
    );
    let second_claim = reopened.claim_outbox(30, 5, 10).await?;
    assert_eq!(second_claim.len(), 1);
    assert_eq!(second_claim[0].attempts, 2);
    reopened
        .mark_outbox_retry(&outbox.txn_id, 2, 31, 40)
        .await?;
    reopened.close().await;

    let reopened = MatrixDurableStore::open(&layout, MatrixDurableConfig::default()).await?;
    assert_eq!(
        reopened.pending_dispatches(10).await?[0].state,
        InboxDispatchState::Queued
    );
    let admitted = reopened.record_inbox_admitted(&admission).await?;
    assert_eq!(admitted.state, InboxDispatchState::Admitted);
    reopened.close().await;

    let reopened = MatrixDurableStore::open(&layout, MatrixDurableConfig::default()).await?;
    assert_eq!(
        reopened.pending_dispatches(10).await?[0].state,
        InboxDispatchState::Admitted
    );
    let completed = reopened.complete_inbox_dispatch(&admission, 42).await?;
    assert_eq!(completed.state, InboxDispatchState::Completed);
    assert!(reopened.pending_dispatches(10).await?.is_empty());
    assert!(reopened.pending_inbox(10).await?.is_empty());
    let persisted = reopened
        .inbox_dispatch(&event_id)
        .await?
        .ok_or("missing completed dispatch")?;
    assert_eq!(persisted, completed);
    let retried = reopened
        .outbox_for_txn(&outbox.txn_id)
        .await?
        .ok_or("missing retry outbox")?;
    assert_eq!(retried.state, OutboxState::RetryScheduled);
    assert_eq!(retried.attempts, 2);
    let metrics = reopened.queue_metrics(50).await?;
    assert_eq!(metrics.pending_dispatch_depth, 0);
    assert_eq!(metrics.pending_outbox_depth, 1);
    assert_eq!(metrics.oldest_outbox_age_ms, Some(27));
    Ok(())
}

#[tokio::test]
async fn cursor_gap_requires_snapshot_then_resumes_exactly() -> TestResult {
    let temp = TempDir::new()?;
    let agent_id = agent(FIRST_AGENT)?;
    let layout = layout(&temp, &agent_id)?;
    let store = MatrixDurableStore::open(
        &layout,
        MatrixDurableConfig {
            delta_coalesce_window_ms: 150,
            max_delta_batch_bytes: 256,
            event_capacity: 4,
        },
    )
    .await?;
    let room_id = room("!cursor:example.test")?;
    bind_room(&store, &room_id, &user("@agent:example.test")?, 10).await?;
    prepare_room_project(&store, &agent_id, &room_id, Some("thread-1"), 11).await?;
    for index in 0..5_u64 {
        store
            .ingest_inbox(&inbox_draft(
                event(format!("$cursor-{index}").as_str())?,
                room_id.clone(),
                b"event",
                20 + index,
            )?)
            .await?;
    }
    let gap = store.read_changes(0, 100).await?;
    assert!(gap.gap);
    assert!(gap.events.is_empty());

    let snapshot = store.snapshot(100, 100).await?;
    assert_eq!(snapshot.pending_inbox.len(), 5);
    assert_eq!(snapshot.room_threads.len(), 1);
    let final_draft = outbox_draft(
        &agent_id,
        &room_id,
        "after-snapshot",
        OutboxKind::Final,
        b"after",
        101,
    )?;
    store.enqueue_outbox(&final_draft).await?;
    let resumed = store.read_changes(snapshot.cursor, 100).await?;
    assert!(!resumed.gap);
    assert_eq!(resumed.events.len(), 1);
    assert_eq!(resumed.events[0].txn_id, Some(final_draft.txn_id));
    Ok(())
}

#[test]
fn canonical_transaction_type_is_used_by_public_store_models() -> TestResult {
    let txn = MatrixTransactionId::parse("hepta-v1-0123456789abcdef")?;
    let _: MatrixTransactionId = txn;
    Ok(())
}
