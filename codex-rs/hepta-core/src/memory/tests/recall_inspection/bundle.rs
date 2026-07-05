use super::*;

#[test]
fn context_recall_bundle_reports_query_hit_and_total_item_counts() {
    let bundle = ContextRecallBundle {
        request: ContextRecallRequest {
            session_id: SessionId("session-42".into()),
            query_text: Some("tool failure".into()),
            recent_window_limit: 8,
            transcript_limit: 3,
            memory_limit: 2,
            allow_cross_session: true,
        },
        recent_entries: vec![sample_transcript_entry(10, "tool failed with timeout")],
        transcript_hits: vec![TranscriptSpan {
            session_id: SessionId("session-42".into()),
            range: TranscriptRange {
                start_sequence: 8,
                end_sequence: 10,
            },
            entry_count: 3,
            excerpt: Some("failure span".into()),
            entries: vec![
                sample_transcript_entry(8, "run tool"),
                sample_transcript_entry(9, "tool timeout"),
                sample_transcript_entry(10, "retry requested"),
            ],
        }],
        durable_memory_hits: vec![MemoryRecord {
            id: "memory-1".into(),
            scope: MemoryScope::LongTerm,
            content: "user prefers retry with bounded timeout".into(),
        }],
        summary_hits: vec![MemoryRecord {
            id: "memory-2".into(),
            scope: MemoryScope::Session,
            content: "earlier tool failure cluster summary".into(),
        }],
        active_topic_sessions: vec![],
        active_neurons: Vec::new(),
        budget: ContextBudget::default(),
        ranked_items: Vec::new(),
        omitted_by_budget: 0,
        truncated: false,
    };

    assert_eq!(
        bundle.source_counts(),
        ContextRecallSourceCounts {
            recent_entry_count: 1,
            transcript_hit_count: 1,
            durable_memory_hit_count: 1,
            summary_hit_count: 1,
        }
    );

    assert_eq!(bundle.query_hit_count(), 3);
    assert_eq!(bundle.total_item_count(), 4);
    assert!(bundle.has_query_matches());
    assert_eq!(bundle.report().request, bundle.request);
    assert_eq!(bundle.report().source_counts, bundle.source_counts());
    assert!(!bundle.report().truncated);
}
