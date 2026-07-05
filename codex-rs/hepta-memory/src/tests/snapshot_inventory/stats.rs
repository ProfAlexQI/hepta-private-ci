use super::*;

#[test]
fn transcript_snapshot_stats_summarize_kinds_and_sessions() {
    let snapshot = StoreSnapshot {
        sessions: vec![],
        memories: vec![],
        transcripts: vec![
            transcript_entry(
                "session-1",
                1,
                TranscriptEntryKind::Message,
                "user asks for a report",
            ),
            transcript_entry(
                "session-1",
                2,
                TranscriptEntryKind::Summary,
                "summary written",
            ),
            transcript_entry(
                "session-2",
                1,
                TranscriptEntryKind::Event,
                "session archived",
            ),
        ],
    };

    let stats = snapshot.transcript_stats();

    assert_eq!(stats.total_entry_count, 3);
    assert_eq!(stats.session_count, 2);
    assert_eq!(stats.message_count, 1);
    assert_eq!(stats.summary_count, 1);
    assert_eq!(stats.event_count, 1);
    assert!(!stats.is_empty());
}

#[test]
fn snapshot_stats_summarize_active_archived_and_memory_scope_counts() {
    let snapshot = StoreSnapshot {
        sessions: vec![
            session_record("session-1", "Foundation", Some("audit memory")),
            SessionRecord {
                archived_at_unix_ms: Some(30),
                ..session_record("session-2", "Archived foundation", None)
            },
        ],
        memories: vec![
            memory_record(
                "memory-1",
                MemoryScope::Session,
                "doctor snapshot integrity",
            ),
            memory_record("memory-2", MemoryScope::LongTerm, "export manifest"),
        ],
        transcripts: vec![],
    };

    let stats = snapshot.stats();

    assert_eq!(stats.session_count, 2);
    assert_eq!(stats.active_session_count, 1);
    assert_eq!(stats.archived_session_count, 1);
    assert_eq!(stats.total_memory_count, 2);
    assert_eq!(stats.session_memory_count, 1);
    assert_eq!(stats.long_term_memory_count, 1);
}

#[tokio::test]
async fn snapshot_stats_follow_store_updates() {
    let store = InMemoryStore::default();
    store
        .upsert_session_sync(session_record(
            "session-1",
            "Foundation",
            Some("sync stats"),
        ))
        .expect("upsert should succeed");
    store
        .put(memory_record(
            "memory-1",
            MemoryScope::LongTerm,
            "contract scaffolding",
        ))
        .await
        .expect("put should succeed");

    let stats = store.snapshot_stats().expect("stats should load");

    assert_eq!(stats.session_count, 1);
    assert_eq!(stats.active_session_count, 1);
    assert_eq!(stats.archived_session_count, 0);
    assert_eq!(stats.total_memory_count, 1);
    assert_eq!(stats.session_memory_count, 0);
    assert_eq!(stats.long_term_memory_count, 1);
}
