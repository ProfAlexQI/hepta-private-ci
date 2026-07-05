use super::*;

#[tokio::test]
async fn store_recall_context_limit_pressure_matches_snapshot_helper() {
    let store = InMemoryStore::default();
    store
        .put(memory_record(
            "memory-1",
            MemoryScope::LongTerm,
            "timeout retry guidance",
        ))
        .await
        .expect("put should succeed");
    store
        .put(memory_record(
            "memory-2",
            MemoryScope::Session,
            "session timeout summary",
        ))
        .await
        .expect("put should succeed");
    store
        .append(transcript_entry(
            "session-1",
            1,
            TranscriptEntryKind::Message,
            "timeout surfaced during tool run",
        ))
        .await
        .expect("append should succeed");
    store
        .append(transcript_entry(
            "session-1",
            2,
            TranscriptEntryKind::Summary,
            "timeout retried successfully",
        ))
        .await
        .expect("append should succeed");

    let request = ContextRecallRequest {
        session_id: SessionId("session-1".into()),
        query_text: Some("timeout".into()),
        recent_window_limit: 1,
        transcript_limit: 1,
        memory_limit: 1,
        allow_cross_session: true,
    };
    let snapshot = store.snapshot().expect("snapshot should load");
    let from_store = store
        .recall_context_limit_pressure(request.clone())
        .expect("context recall limit pressure should succeed");

    assert_eq!(from_store, snapshot.recall_context_limit_pressure(&request));
    assert_eq!(
        from_store,
        ContextRecallLimitPressure {
            recent_entries_truncated: true,
            transcript_hits_truncated: true,
            memory_hits_truncated: true,
            omission_counts: ContextRecallOmissionCounts {
                recent_entry_count: 1,
                transcript_hit_count: 1,
                memory_hit_count: 1,
                query_hit_count: 2,
                total_item_count: 3,
            },
        }
    );
    assert!(from_store.query_hits_truncated());
    assert!(from_store.has_omissions());
    assert!(!from_store.is_complete());
}
