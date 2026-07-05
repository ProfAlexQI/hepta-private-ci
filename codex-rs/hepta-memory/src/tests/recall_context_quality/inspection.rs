use super::*;

#[test]
fn store_snapshot_recall_context_inspection_tracks_availability_counts() {
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
        recent_window_limit: 2,
        transcript_limit: 1,
        memory_limit: 2,
        allow_cross_session: true,
    };

    let inspection = snapshot.recall_context_inspection(&request);
    let bundle = snapshot.recall_context(&request);

    assert_eq!(inspection.report, bundle.report());
    assert_eq!(inspection.availability.total_recent_entry_count, 3);
    assert_eq!(inspection.availability.total_transcript_match_count, 2);
    assert_eq!(inspection.availability.total_memory_match_count, 3);
    assert_eq!(inspection.returned_query_hit_count(), 3);
    assert_eq!(inspection.omitted_recent_entry_count(), 1);
    assert_eq!(inspection.omitted_transcript_hit_count(), 1);
    assert_eq!(inspection.omitted_memory_hit_count(), 1);
    assert_eq!(inspection.omitted_query_hit_count(), 2);
    assert_eq!(inspection.matched_query_hit_count(), 5);
    assert_eq!(inspection.returned_total_item_count(), 5);
    assert_eq!(inspection.matched_total_item_count(), 8);
    assert_eq!(inspection.omitted_total_item_count(), 3);
    assert_eq!(
        inspection.omission_counts(),
        ContextRecallOmissionCounts {
            recent_entry_count: 1,
            transcript_hit_count: 1,
            memory_hit_count: 1,
            query_hit_count: 2,
            total_item_count: 3,
        }
    );
    assert!(inspection.has_omissions());
    assert!(inspection.recent_entries_truncated());
    assert!(inspection.transcript_hits_truncated());
    assert!(inspection.memory_hits_truncated());
    assert!(inspection.has_query_matches());
    assert!(!inspection.is_complete());
    assert!(!inspection.is_empty());
}
