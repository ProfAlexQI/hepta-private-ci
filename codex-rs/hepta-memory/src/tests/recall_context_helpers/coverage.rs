use super::*;

#[tokio::test]
async fn store_recall_context_coverage_matches_snapshot_helper() {
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
        .recall_context_coverage(request.clone())
        .expect("context recall coverage should succeed");

    assert_eq!(from_store, snapshot.recall_context_coverage(&request));
    assert_eq!(
        from_store,
        ContextRecallCoverage {
            recent_entries: ContextRecallCoverageCounts {
                returned_count: 1,
                available_count: 2,
            },
            transcript_hits: ContextRecallCoverageCounts {
                returned_count: 1,
                available_count: 2,
            },
            memory_hits: ContextRecallCoverageCounts {
                returned_count: 1,
                available_count: 2,
            },
            query_hits: ContextRecallCoverageCounts {
                returned_count: 2,
                available_count: 4,
            },
            total_items: ContextRecallCoverageCounts {
                returned_count: 3,
                available_count: 6,
            },
        }
    );
    assert!(from_store.has_omissions());
    assert!(!from_store.is_complete());
}
