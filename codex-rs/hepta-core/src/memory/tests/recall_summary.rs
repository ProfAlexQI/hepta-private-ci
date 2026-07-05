use super::*;

#[test]
fn context_recall_report_from_bundle_preserves_compact_counts() {
    let request = ContextRecallRequest {
        session_id: SessionId("session-1".into()),
        query_text: Some("timeout".into()),
        recent_window_limit: 2,
        transcript_limit: 1,
        memory_limit: 2,
        allow_cross_session: true,
    };
    let bundle = ContextRecallBundle {
        request: request.clone(),
        recent_entries: vec![sample_transcript_entry(2, "timeout surfaced")],
        transcript_hits: vec![TranscriptSpan::from_entry(sample_transcript_entry(
            3,
            "timeout resolved",
        ))],
        durable_memory_hits: vec![MemoryRecord {
            id: "memory-1".into(),
            scope: MemoryScope::LongTerm,
            content: "timeout retry guidance".into(),
        }],
        summary_hits: vec![MemoryRecord {
            id: "memory-2".into(),
            scope: MemoryScope::Session,
            content: "timeout summary".into(),
        }],
        active_topic_sessions: Vec::new(),
        active_neurons: Vec::new(),
        budget: ContextBudget::default(),
        ranked_items: Vec::new(),
        omitted_by_budget: 0,
        truncated: true,
    };

    let report = ContextRecallReport::from_bundle(&bundle);

    assert_eq!(report.request, request);
    assert_eq!(report.source_counts, bundle.source_counts());
    assert_eq!(report.query_hit_count(), 3);
    assert_eq!(report.total_item_count(), 4);
    assert!(report.has_query_matches());
    assert!(!report.is_empty());
    assert!(report.truncated);
}

#[test]
fn context_recall_transcript_provenance_summary_splits_reason_lists() {
    let summary = ContextRecallTranscriptProvenanceSummary::from_span_refs(&[
        crate::TranscriptSpanRef {
            session_id: SessionId("session-1".into()),
            range: TranscriptRange {
                start_sequence: 1,
                end_sequence: 2,
            },
            reason: Some("recent_window, query_match".into()),
        },
        crate::TranscriptSpanRef {
            session_id: SessionId("session-1".into()),
            range: TranscriptRange {
                start_sequence: 3,
                end_sequence: 3,
            },
            reason: Some("query_match, active_topic_session".into()),
        },
        crate::TranscriptSpanRef {
            session_id: SessionId("session-2".into()),
            range: TranscriptRange {
                start_sequence: 1,
                end_sequence: 1,
            },
            reason: Some("   ".into()),
        },
        crate::TranscriptSpanRef {
            session_id: SessionId("   ".into()),
            range: TranscriptRange {
                start_sequence: 4,
                end_sequence: 4,
            },
            reason: None,
        },
    ]);

    assert_eq!(
        summary,
        ContextRecallTranscriptProvenanceSummary {
            span_count: 4,
            session_count: 2,
            spans_with_reason_count: 2,
            distinct_reason_count: 3,
        }
    );
    assert!(summary.has_spans());
    assert!(summary.has_reasons());
    assert!(!summary.is_empty());
}
