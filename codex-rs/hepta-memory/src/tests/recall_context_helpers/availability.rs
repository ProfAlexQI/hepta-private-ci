use super::*;

#[tokio::test]
async fn store_recall_context_availability_matches_snapshot_helper() {
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
        .recall_context_availability(request.clone())
        .expect("context recall availability should succeed");

    assert_eq!(from_store, snapshot.recall_context_availability(&request));
    assert_eq!(
        from_store,
        ContextRecallAvailability {
            total_recent_entry_count: 2,
            total_transcript_match_count: 2,
            total_memory_match_count: 2,
        }
    );
    assert_eq!(
        from_store,
        store
            .recall_context_inspection(request)
            .expect("context recall inspection should succeed")
            .availability
    );
}

#[tokio::test]
async fn store_recall_context_source_availability_matches_snapshot_helper() {
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
        .put(memory_record(
            "memory-3",
            MemoryScope::LongTerm,
            "timeout rollback note",
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
        .recall_context_source_availability(request.clone())
        .expect("context recall source availability should succeed");

    assert_eq!(
        from_store,
        snapshot.recall_context_source_availability(&request)
    );
    assert_eq!(
        from_store,
        ContextRecallSourceAvailability {
            recent_entry_count: 2,
            transcript_match_count: 2,
            durable_memory_match_count: 2,
            summary_memory_match_count: 1,
        }
    );
    assert_eq!(from_store.memory_match_count(), 3);
    assert_eq!(from_store.query_match_count(), 5);
    assert_eq!(from_store.total_item_count(), 7);
}
