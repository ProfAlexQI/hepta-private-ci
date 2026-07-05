use super::*;

#[test]
fn store_snapshot_recall_context_memory_taxonomy_maps_sources_without_payloads() {
    let snapshot = StoreSnapshot {
        sessions: vec![],
        memories: vec![
            memory_record("memory-1", MemoryScope::LongTerm, "timeout retry guidance"),
            memory_record("memory-2", MemoryScope::Session, "session timeout summary"),
            memory_record("memory-3", MemoryScope::LongTerm, "timeout rollback note"),
            memory_record(
                "memory-4",
                MemoryScope::Session,
                "timeout summary follow-up",
            ),
            memory_record(
                "memory-5",
                MemoryScope::LongTerm,
                "[hepta-memory:tombstone] timeout hidden control",
            ),
        ],
        transcripts: vec![
            transcript_entry(
                "session-1",
                1,
                TranscriptEntryKind::Message,
                "start diagnosis",
            ),
            transcript_entry(
                "session-1",
                2,
                TranscriptEntryKind::Summary,
                "timeout retried successfully",
            ),
            transcript_entry(
                "session-2",
                1,
                TranscriptEntryKind::Message,
                "timeout in another session",
            ),
        ],
    };
    let request = ContextRecallRequest {
        session_id: SessionId("session-1".into()),
        query_text: Some("timeout".into()),
        recent_window_limit: 1,
        transcript_limit: 1,
        memory_limit: 1,
        allow_cross_session: true,
    };

    let taxonomy = snapshot.recall_context_memory_taxonomy_report(&request);

    assert!(taxonomy.has_count_integrity());
    assert_eq!(
        taxonomy,
        ContextMemoryTaxonomyReport {
            buckets: vec![
                ContextMemoryTaxonomyBucket {
                    class: ContextMemoryTaxonomyClass::Semantic,
                    source_count: 1,
                    returned_count: 1,
                    available_count: 2,
                    omitted_count: 1,
                    provenance_span_count: 0,
                },
                ContextMemoryTaxonomyBucket {
                    class: ContextMemoryTaxonomyClass::Episodic,
                    source_count: 1,
                    returned_count: 0,
                    available_count: 2,
                    omitted_count: 2,
                    provenance_span_count: 0,
                },
                ContextMemoryTaxonomyBucket {
                    class: ContextMemoryTaxonomyClass::Control,
                    source_count: 1,
                    returned_count: 0,
                    available_count: 1,
                    omitted_count: 1,
                    provenance_span_count: 0,
                },
                ContextMemoryTaxonomyBucket {
                    class: ContextMemoryTaxonomyClass::Transcript,
                    source_count: 2,
                    returned_count: 2,
                    available_count: 3,
                    omitted_count: 1,
                    provenance_span_count: 1,
                },
            ],
        }
    );

    let json = serde_json::to_string(&taxonomy).expect("taxonomy should serialize");
    assert!(json.contains("semantic"));
    assert!(json.contains("episodic"));
    assert!(json.contains("control"));
    assert!(json.contains("transcript"));
    assert!(!json.contains("timeout retry guidance"));
    assert!(!json.contains("session timeout summary"));
    assert!(!json.contains("hepta-memory:tombstone"));
    assert!(!json.contains("memory-"));
}

#[tokio::test]
async fn store_recall_context_memory_taxonomy_matches_snapshot_helper() {
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
            "[hepta-memory:conflict] timeout hidden control",
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
        .recall_context_memory_taxonomy_report(request.clone())
        .expect("context recall memory taxonomy should succeed");

    assert_eq!(
        from_store,
        snapshot.recall_context_memory_taxonomy_report(&request)
    );
    assert!(from_store.has_count_integrity());
    assert!(
        from_store
            .buckets
            .iter()
            .any(|bucket| bucket.class == ContextMemoryTaxonomyClass::Control
                && bucket.omitted_count == 1)
    );
}
