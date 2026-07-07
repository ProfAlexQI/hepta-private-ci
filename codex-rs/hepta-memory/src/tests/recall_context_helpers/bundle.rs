use super::*;

#[tokio::test]
async fn store_recall_context_matches_snapshot_helper() {
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

    let request = ContextRecallRequest {
        session_id: SessionId("session-1".into()),
        query_text: Some("timeout".into()),
        recent_window_limit: 4,
        transcript_limit: 2,
        memory_limit: 4,
        allow_cross_session: true,
    };
    let snapshot = store.snapshot().expect("snapshot should load");
    let from_store = store
        .recall_context(request.clone())
        .expect("context recall should succeed");

    assert_eq!(from_store, snapshot.recall_context(&request));
    assert_eq!(
        from_store.source_counts(),
        ContextRecallSourceCounts {
            recent_entry_count: 1,
            transcript_hit_count: 1,
            durable_memory_hit_count: 1,
            summary_hit_count: 1,
        }
    );
    assert_eq!(from_store.query_hit_count(), 3);
    assert_eq!(from_store.total_item_count(), 4);
    assert!(!from_store.truncated);
}

#[tokio::test]
async fn store_recall_context_populates_payload_light_ranked_items() {
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

    let request = ContextRecallRequest {
        session_id: SessionId("session-1".into()),
        query_text: Some("timeout".into()),
        recent_window_limit: 4,
        transcript_limit: 2,
        memory_limit: 4,
        allow_cross_session: true,
    };

    let bundle = store
        .recall_context(request)
        .expect("context recall should succeed");

    assert_eq!(bundle.ranked_items.len(), 4);
    assert_eq!(
        bundle.ranked_items[0].source,
        ContextRecallSource::DurableMemory
    );
    assert_eq!(bundle.ranked_items[0].source_id, "durable_memory:memory-1");
    assert!(bundle.ranked_items[0].score.final_score >= bundle.ranked_items[1].score.final_score);
    assert!(
        bundle
            .ranked_items
            .iter()
            .all(|item| !item.summary.contains("timeout")
                && !item.summary.contains("retry guidance")
                && item.summary.contains("content_bytes="))
    );
    assert!(
        bundle
            .ranked_items
            .iter()
            .all(|item| item.score.reason.is_some())
    );
    assert_eq!(bundle.omitted_by_budget, 0);
}

#[tokio::test]
async fn store_recall_context_report_matches_snapshot_helper() {
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

    let request = ContextRecallRequest {
        session_id: SessionId("session-1".into()),
        query_text: Some("timeout".into()),
        recent_window_limit: 4,
        transcript_limit: 2,
        memory_limit: 4,
        allow_cross_session: true,
    };
    let snapshot = store.snapshot().expect("snapshot should load");
    let from_store = store
        .recall_context_report(request.clone())
        .expect("context recall report should succeed");

    assert_eq!(from_store, snapshot.recall_context_report(&request));
    assert_eq!(from_store.request, request);
    assert_eq!(from_store.query_hit_count(), 2);
    assert_eq!(from_store.total_item_count(), 3);
    assert!(from_store.has_query_matches());
    assert!(!from_store.truncated);
}

#[tokio::test]
async fn store_recall_context_inspection_matches_snapshot_helper() {
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
        .recall_context_inspection(request.clone())
        .expect("context recall inspection should succeed");

    assert_eq!(from_store, snapshot.recall_context_inspection(&request));
    assert_eq!(
        from_store.report,
        snapshot.recall_context(&request).report()
    );
    assert_eq!(from_store.availability.total_recent_entry_count, 2);
    assert_eq!(from_store.availability.total_transcript_match_count, 2);
    assert_eq!(from_store.availability.total_memory_match_count, 2);
    assert_eq!(from_store.omitted_recent_entry_count(), 1);
    assert_eq!(from_store.omitted_transcript_hit_count(), 1);
    assert_eq!(from_store.omitted_memory_hit_count(), 1);
    assert_eq!(from_store.omitted_query_hit_count(), 2);
    assert_eq!(from_store.omitted_total_item_count(), 3);
    assert_eq!(
        from_store.omission_counts(),
        ContextRecallOmissionCounts {
            recent_entry_count: 1,
            transcript_hit_count: 1,
            memory_hit_count: 1,
            query_hit_count: 2,
            total_item_count: 3,
        }
    );
    assert!(from_store.has_omissions());
    assert!(from_store.recent_entries_truncated());
    assert!(from_store.transcript_hits_truncated());
    assert!(from_store.memory_hits_truncated());
    assert!(!from_store.is_complete());
}
