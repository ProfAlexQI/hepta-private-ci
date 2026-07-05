use super::*;

#[tokio::test]
async fn store_recall_context_transcript_provenance_summary_matches_snapshot_helper() {
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
        .recall_context_transcript_provenance_summary(request.clone())
        .expect("context recall provenance summary should succeed");

    assert_eq!(
        from_store,
        snapshot.recall_context_transcript_provenance_summary(&request)
    );
    assert_eq!(
        from_store,
        ContextRecallTranscriptProvenanceSummary {
            span_count: 1,
            session_count: 1,
            spans_with_reason_count: 1,
            distinct_reason_count: 2,
        }
    );
}
