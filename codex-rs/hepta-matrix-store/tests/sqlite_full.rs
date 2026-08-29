#![cfg(feature = "qualification-fault-injection")]

use codex_hepta_contracts::AgentId;
use codex_hepta_contracts::OperationPhase;
use codex_hepta_matrix_protocol::MatrixEventId;
use codex_hepta_matrix_protocol::MatrixRoomId;
use codex_hepta_matrix_protocol::MatrixUserId;
use codex_hepta_matrix_protocol::room_project_idempotency_key;
use codex_hepta_matrix_store::InboxDisposition;
use codex_hepta_matrix_store::InboxDraft;
use codex_hepta_matrix_store::MatrixDurableConfig;
use codex_hepta_matrix_store::MatrixDurableError;
use codex_hepta_matrix_store::MatrixDurableStore;
use codex_hepta_matrix_store::MatrixOperationJournal;
use codex_hepta_matrix_store::RoomBindingDraft;
use codex_hepta_paths::HeptaFleetRoot;
use codex_state::SqliteConfig;
use codex_utils_absolute_path::AbsolutePathBuf;

const AGENT_ID: &str = "018f4f72-5f8f-7cc1-8f55-df9fb3aa2c12";

#[tokio::test]
async fn real_matrix_sqlite_full_rolls_back_failed_inbox_and_preserves_operation_reopen() {
    let temp = tempfile::tempdir().expect("temporary root");
    let root = temp.path().join("fleet");
    std::fs::create_dir_all(&root).expect("create fleet root");
    let fleet =
        HeptaFleetRoot::parse(root.canonicalize().expect("canonical root")).expect("fleet root");
    let agent_id = AgentId::parse(AGENT_ID).expect("AgentId");
    let layout = fleet.layout().agent(&agent_id);
    let store = MatrixDurableStore::open(&layout, MatrixDurableConfig::default())
        .await
        .expect("open Matrix store");
    let room_id = MatrixRoomId::parse("!sqlite-full:example.test").expect("room id");
    store
        .bind_room(&RoomBindingDraft {
            room_id: room_id.clone(),
            agent_user_id: MatrixUserId::parse("@agent:example.test").expect("agent mxid"),
            expected_revision: None,
            generation: 1,
            changed_at_ms: 1,
        })
        .await
        .expect("bind room");

    let baseline_id = MatrixEventId::parse("$sqlite-full-baseline").expect("event id");
    let baseline = InboxDraft {
        event_id: baseline_id.clone(),
        room_id: room_id.clone(),
        sender: MatrixUserId::parse("@owner:example.test").expect("sender"),
        event_type: "m.room.message".to_string(),
        payload: br#"{"msgtype":"m.text","body":"baseline"}"#.to_vec(),
        binding_revision: 1,
        generation: 1,
        origin_server_ts_ms: 2,
        received_at_ms: 3,
    };
    assert!(matches!(
        store
            .ingest_inbox(&baseline)
            .await
            .expect("baseline ingest"),
        InboxDisposition::Accepted(_)
    ));
    let baseline_record = store
        .inbox(&baseline_id)
        .await
        .expect("load baseline")
        .expect("baseline record");
    let project_id = room_project_idempotency_key(&agent_id, &room_id);
    let journal = MatrixOperationJournal::new(&store);
    let baseline_operation = journal
        .begin(&baseline_record, &project_id, 4)
        .await
        .expect("begin baseline operation")
        .record;
    assert_eq!(baseline_operation.phase, OperationPhase::OutboxPending);

    let sqlite_home = AbsolutePathBuf::try_from(
        store
            .path()
            .parent()
            .expect("Matrix database parent")
            .to_path_buf(),
    )
    .expect("absolute SQLite home");
    let sqlite = SqliteConfig::from_sqlite_home(sqlite_home);
    let observation_pool = sqlite
        .open_durable_evidence_pool(store.path())
        .await
        .expect("open SQLite observation connection");
    let page_count: i64 = sqlx::query_scalar("PRAGMA page_count")
        .fetch_one(&observation_pool)
        .await
        .expect("page count");
    observation_pool.close().await;
    let max_page_count = u64::try_from(page_count).expect("non-negative page count");

    let failed_event = MatrixEventId::parse("$sqlite-full-failed").expect("event id");
    let failed = InboxDraft {
        event_id: failed_event.clone(),
        room_id: room_id.clone(),
        sender: MatrixUserId::parse("@owner:example.test").expect("sender"),
        event_type: "m.room.message".to_string(),
        payload: vec![b'x'; 512 * 1024],
        binding_revision: 1,
        generation: 1,
        origin_server_ts_ms: 10,
        received_at_ms: 20,
    };
    assert_eq!(
        store
            .ingest_inbox_with_max_page_count_for_qualification(&failed, max_page_count)
            .await,
        Err(MatrixDurableError::Unavailable),
        "the product write transaction must observe SQLITE_FULL on its own connection",
    );
    assert!(
        store
            .inbox(&failed_event)
            .await
            .expect("query failed event")
            .is_none(),
        "the failed product transaction must not leave a partial inbox row"
    );
    let still_exact = journal
        .load(&baseline_id)
        .await
        .expect("load baseline operation after SQLITE_FULL")
        .expect("baseline operation remains");
    assert_eq!(still_exact, baseline_operation);
    store.close().await;

    let reopened = MatrixDurableStore::open(&layout, MatrixDurableConfig::default())
        .await
        .expect("reopen Matrix store after SQLITE_FULL");
    let reopened_journal = MatrixOperationJournal::new(&reopened);
    let reopened_operation = reopened_journal
        .load(&baseline_id)
        .await
        .expect("load reopened operation")
        .expect("reopened operation remains");
    assert_eq!(reopened_operation, baseline_operation);
    reopened.close().await;
}
