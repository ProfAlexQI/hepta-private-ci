use super::*;

#[test]
fn store_snapshot_recall_context_limit_pressure_matches_coverage_helper() {
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

    let pressure = snapshot.recall_context_limit_pressure(&request);

    assert_eq!(
        pressure,
        snapshot.recall_context_coverage(&request).limit_pressure()
    );
    assert_eq!(
        pressure,
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
    assert!(pressure.query_hits_truncated());
    assert!(pressure.has_omissions());
    assert!(!pressure.is_complete());
}
