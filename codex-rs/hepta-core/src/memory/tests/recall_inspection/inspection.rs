use super::*;

#[test]
fn context_recall_inspection_tracks_availability_and_limit_pressure() {
    let bundle = ContextRecallBundle {
        request: ContextRecallRequest {
            session_id: SessionId("session-42".into()),
            query_text: Some("tool failure".into()),
            recent_window_limit: 2,
            transcript_limit: 1,
            memory_limit: 1,
            allow_cross_session: true,
        },
        recent_entries: vec![
            sample_transcript_entry(9, "tool timeout"),
            sample_transcript_entry(10, "retry requested"),
        ],
        transcript_hits: vec![TranscriptSpan::from_entry(sample_transcript_entry(
            8,
            "tool failure span",
        ))],
        durable_memory_hits: vec![MemoryRecord {
            id: "memory-1".into(),
            scope: MemoryScope::LongTerm,
            content: "retry guidance".into(),
        }],
        summary_hits: vec![],
        active_topic_sessions: vec![],
        active_neurons: Vec::new(),
        budget: ContextBudget::default(),
        ranked_items: Vec::new(),
        omitted_by_budget: 0,
        truncated: true,
    };

    let inspection = bundle.inspection(ContextRecallAvailability {
        total_recent_entry_count: 4,
        total_transcript_match_count: 3,
        total_memory_match_count: 2,
    });

    assert_eq!(inspection.report, bundle.report());
    assert_eq!(inspection.source_transcript_spans.len(), 2);
    assert!(inspection.source_transcript_spans.iter().any(|span| {
        span.session_id.0 == "session-42"
            && span.range.start_sequence == 9
            && span.range.end_sequence == 10
            && span.reason.as_deref() == Some("recent_window")
    }));
    assert!(inspection.source_transcript_spans.iter().any(|span| {
        span.session_id.0 == "session-42"
            && span.range.start_sequence == 8
            && span.range.end_sequence == 8
            && span.reason.as_deref() == Some("query_match")
    }));
    assert_eq!(inspection.returned_memory_hit_count(), 1);
    assert_eq!(inspection.returned_query_hit_count(), 2);
    assert_eq!(inspection.returned_total_item_count(), 4);
    assert_eq!(inspection.omitted_recent_entry_count(), 2);
    assert_eq!(inspection.omitted_transcript_hit_count(), 2);
    assert_eq!(inspection.omitted_memory_hit_count(), 1);
    assert_eq!(inspection.omitted_query_hit_count(), 3);
    assert_eq!(inspection.omitted_total_item_count(), 5);
    assert_eq!(
        inspection.omission_counts(),
        ContextRecallOmissionCounts {
            recent_entry_count: 2,
            transcript_hit_count: 2,
            memory_hit_count: 1,
            query_hit_count: 3,
            total_item_count: 5,
        }
    );
    assert_eq!(inspection.matched_query_hit_count(), 5);
    assert_eq!(inspection.matched_total_item_count(), 9);
    assert!(inspection.has_query_matches());
    assert!(inspection.has_omissions());
    assert!(inspection.recent_entries_truncated());
    assert!(inspection.transcript_hits_truncated());
    assert!(inspection.memory_hits_truncated());
    assert!(!inspection.is_complete());
    assert!(!inspection.is_empty());
}

#[test]
fn context_recall_inspection_roundtrips_through_json() {
    let inspection = ContextRecallInspection {
        report: ContextRecallReport {
            request: ContextRecallRequest {
                session_id: SessionId("session-42".into()),
                query_text: Some("timeout".into()),
                recent_window_limit: 4,
                transcript_limit: 2,
                memory_limit: 2,
                allow_cross_session: true,
            },
            source_counts: ContextRecallSourceCounts {
                recent_entry_count: 2,
                transcript_hit_count: 1,
                durable_memory_hit_count: 1,
                summary_hit_count: 0,
            },
            truncated: false,
        },
        availability: ContextRecallAvailability {
            total_recent_entry_count: 2,
            total_transcript_match_count: 1,
            total_memory_match_count: 1,
        },
        source_transcript_spans: vec![TranscriptSpanRef {
            session_id: SessionId("session-42".into()),
            range: TranscriptRange {
                start_sequence: 7,
                end_sequence: 8,
            },
            reason: Some("recent_window, query_match".into()),
        }],
    };

    let json = serde_json::to_string(&inspection).expect("inspection should serialize");
    let parsed: ContextRecallInspection =
        serde_json::from_str(&json).expect("inspection should deserialize");

    assert_eq!(parsed, inspection);
    assert_eq!(parsed.omitted_recent_entry_count(), 0);
    assert_eq!(parsed.omitted_transcript_hit_count(), 0);
    assert_eq!(parsed.omitted_memory_hit_count(), 0);
    assert_eq!(parsed.omitted_query_hit_count(), 0);
    assert_eq!(parsed.omitted_total_item_count(), 0);
    assert!(!parsed.has_omissions());
    assert!(parsed.is_complete());
    assert!(!parsed.recent_entries_truncated());
    assert!(!parsed.transcript_hits_truncated());
    assert!(!parsed.memory_hits_truncated());
    assert_eq!(parsed.source_transcript_spans.len(), 1);
}
