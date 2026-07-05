use super::*;

#[test]
fn session_agent_inventory_summarizes_sessions_by_agent() {
    let snapshot = StoreSnapshot {
        sessions: vec![
            SessionRecord {
                session_id: SessionId("session-2".into()),
                agent_id: AgentId("reviewer".into()),
                title: "Reviewer lane".into(),
                created_at_unix_ms: 9,
                last_active_unix_ms: 20,
                last_user_intent_summary: None,
                archived_at_unix_ms: None,
            },
            SessionRecord {
                archived_at_unix_ms: Some(30),
                last_active_unix_ms: 25,
                ..session_record("session-1", "Builder lane", Some("contracts"))
            },
            SessionRecord {
                session_id: SessionId("session-3".into()),
                agent_id: AgentId("builder".into()),
                title: "Builder follow-up".into(),
                created_at_unix_ms: 11,
                last_active_unix_ms: 40,
                last_user_intent_summary: None,
                archived_at_unix_ms: None,
            },
            SessionRecord {
                session_id: SessionId("session-4".into()),
                agent_id: AgentId("   ".into()),
                title: "Blank agent lane".into(),
                created_at_unix_ms: 12,
                last_active_unix_ms: 50,
                last_user_intent_summary: None,
                archived_at_unix_ms: None,
            },
        ],
        memories: vec![],
        transcripts: vec![],
    };

    let inventory = snapshot.session_agent_inventory();

    assert_eq!(inventory.total_session_count, 4);
    assert_eq!(inventory.blank_agent_id_session_count, 1);
    assert_eq!(inventory.agent_count(), 2);
    assert_eq!(inventory.inventoried_session_count(), 3);
    assert_eq!(inventory.agents[0].agent_id.0, "builder");
    assert_eq!(inventory.agents[0].session_count, 2);
    assert_eq!(inventory.agents[0].active_session_count, 1);
    assert_eq!(inventory.agents[0].archived_session_count, 1);
    assert_eq!(inventory.agents[0].latest_activity_unix_ms, 40);
    assert_eq!(inventory.agents[1].agent_id.0, "reviewer");
    assert_eq!(inventory.agents[1].session_count, 1);
}

#[tokio::test]
async fn store_session_agent_inventory_matches_snapshot_helper() {
    let store = InMemoryStore::default();
    store
        .upsert_session_sync(session_record(
            "session-1",
            "Builder lane",
            Some("capture agent inventory"),
        ))
        .expect("upsert should succeed");
    store
        .upsert_session_sync(SessionRecord {
            session_id: SessionId("session-2".into()),
            agent_id: AgentId("reviewer".into()),
            title: "Reviewer lane".into(),
            created_at_unix_ms: 11,
            last_active_unix_ms: 40,
            last_user_intent_summary: None,
            archived_at_unix_ms: Some(50),
        })
        .expect("upsert should succeed");

    let snapshot = store.snapshot().expect("snapshot should load");
    let from_store = store
        .session_agent_inventory()
        .expect("agent inventory should load");

    assert_eq!(from_store, snapshot.session_agent_inventory());
    assert_eq!(from_store.total_session_count, 2);
    assert_eq!(from_store.blank_agent_id_session_count, 0);
    assert_eq!(from_store.agent_count(), 2);
    assert_eq!(from_store.inventoried_session_count(), 2);
    assert_eq!(from_store.agents[0].agent_id.0, "builder");
    assert_eq!(from_store.agents[1].agent_id.0, "reviewer");
    assert_eq!(from_store.agents[1].archived_session_count, 1);
}

#[test]
fn transcript_session_inventory_summarizes_sessions_ranges_and_blank_ids() {
    let snapshot = StoreSnapshot {
        sessions: vec![],
        memories: vec![],
        transcripts: vec![
            transcript_entry(
                "session-z",
                3,
                TranscriptEntryKind::ToolResult,
                "tool result payload",
            ),
            transcript_entry(
                "session-a",
                2,
                TranscriptEntryKind::Summary,
                "summary payload",
            ),
            transcript_entry(
                "session-a",
                1,
                TranscriptEntryKind::Message,
                "message payload",
            ),
            TranscriptEntry {
                entry_id: "blank-session".into(),
                session_id: SessionId("   ".into()),
                sequence: 4,
                kind: TranscriptEntryKind::Event,
                role: None,
                content: "missing session id".into(),
                created_at_unix_ms: 104,
                tool_name: None,
                correlation_id: None,
                summary_of_range: None,
            },
        ],
    };

    let inventory = snapshot.transcript_session_inventory();

    assert_eq!(inventory.total_entry_count, 4);
    assert_eq!(inventory.blank_session_id_entry_count, 1);
    assert_eq!(inventory.session_count(), 2);
    assert_eq!(inventory.inventoried_entry_count(), 3);
    assert_eq!(inventory.sessions[0].session_id.0, "session-a");
    assert_eq!(inventory.sessions[0].entry_count, 2);
    assert_eq!(inventory.sessions[0].first_sequence, 1);
    assert_eq!(inventory.sessions[0].last_sequence, 2);
    assert_eq!(inventory.sessions[0].message_count, 1);
    assert_eq!(inventory.sessions[0].summary_count, 1);
    assert_eq!(inventory.sessions[1].session_id.0, "session-z");
    assert_eq!(inventory.sessions[1].tool_result_count, 1);
}

#[tokio::test]
async fn store_transcript_session_inventory_matches_snapshot_helper() {
    let store = InMemoryStore::default();
    store
        .append(transcript_entry(
            "session-b",
            5,
            TranscriptEntryKind::ToolCall,
            "write call",
        ))
        .await
        .expect("append should succeed");
    store
        .append(transcript_entry(
            "session-b",
            6,
            TranscriptEntryKind::ToolResult,
            "write ok",
        ))
        .await
        .expect("append should succeed");
    store
        .append(transcript_entry(
            "session-a",
            1,
            TranscriptEntryKind::Message,
            "hello",
        ))
        .await
        .expect("append should succeed");

    let snapshot = store.snapshot().expect("snapshot should load");
    let from_store = store
        .transcript_session_inventory()
        .expect("inventory should load");

    assert_eq!(from_store, snapshot.transcript_session_inventory());
    assert_eq!(from_store.total_entry_count, 3);
    assert_eq!(from_store.session_count(), 2);
    assert_eq!(from_store.sessions[0].session_id.0, "session-a");
    assert_eq!(from_store.sessions[1].session_id.0, "session-b");
    assert_eq!(from_store.sessions[1].first_sequence, 5);
    assert_eq!(from_store.sessions[1].last_sequence, 6);
    assert_eq!(from_store.sessions[1].tool_call_count, 1);
    assert_eq!(from_store.sessions[1].tool_result_count, 1);
}
