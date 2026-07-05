use super::*;

#[test]
fn store_snapshot_recall_context_availability_matches_inspection_helper() {
    let snapshot = StoreSnapshot {
        sessions: vec![],
        memories: vec![
            memory_record("memory-1", MemoryScope::LongTerm, "timeout retry guidance"),
            memory_record("memory-2", MemoryScope::Session, "session timeout summary"),
            memory_record("memory-3", MemoryScope::LongTerm, "timeout rollback note"),
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
                TranscriptEntryKind::Message,
                "timeout surfaced during tool run",
            ),
            transcript_entry(
                "session-1",
                3,
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

    let availability = snapshot.recall_context_availability(&request);

    assert_eq!(
        availability,
        ContextRecallAvailability {
            total_recent_entry_count: 3,
            total_transcript_match_count: 2,
            total_memory_match_count: 3,
        }
    );
    assert_eq!(
        availability,
        snapshot.recall_context_inspection(&request).availability
    );
    assert_eq!(availability.query_match_count(), 5);
    assert_eq!(availability.total_item_count(), 8);
    assert!(availability.has_query_matches());
    assert!(!availability.is_empty());
}

#[test]
fn store_snapshot_recall_context_source_availability_preserves_memory_scope_split() {
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

    let availability = snapshot.recall_context_source_availability(&request);

    assert_eq!(
        availability,
        ContextRecallSourceAvailability {
            recent_entry_count: 2,
            transcript_match_count: 1,
            durable_memory_match_count: 2,
            summary_memory_match_count: 2,
        }
    );
    assert_eq!(availability.memory_match_count(), 4);
    assert_eq!(availability.query_match_count(), 5);
    assert_eq!(availability.total_item_count(), 7);
    assert!(availability.has_query_matches());
    assert!(!availability.is_empty());
}
