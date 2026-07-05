use super::*;

#[test]
fn store_snapshot_recall_context_transcript_provenance_summary_matches_inspection_helper() {
    let snapshot = StoreSnapshot {
        sessions: vec![],
        memories: vec![memory_record(
            "memory-1",
            MemoryScope::LongTerm,
            "timeout retry guidance",
        )],
        transcripts: vec![
            transcript_entry(
                "session-1",
                1,
                TranscriptEntryKind::Message,
                "timeout surfaced during tool run",
            ),
            transcript_entry(
                "session-1",
                2,
                TranscriptEntryKind::Summary,
                "timeout retried successfully",
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

    let summary = snapshot.recall_context_transcript_provenance_summary(&request);

    assert_eq!(
        summary,
        snapshot
            .recall_context_inspection(&request)
            .transcript_provenance_summary()
    );
    assert_eq!(
        summary,
        ContextRecallTranscriptProvenanceSummary {
            span_count: 1,
            session_count: 1,
            spans_with_reason_count: 1,
            distinct_reason_count: 2,
        }
    );
}
