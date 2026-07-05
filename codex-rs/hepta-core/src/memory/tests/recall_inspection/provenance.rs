use super::*;

#[test]
fn context_recall_transcript_provenance_summary_tracks_sessions_and_reasons() {
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
        source_transcript_spans: vec![
            TranscriptSpanRef {
                session_id: SessionId("session-42".into()),
                range: TranscriptRange {
                    start_sequence: 7,
                    end_sequence: 8,
                },
                reason: Some("recent_window, query_match".into()),
            },
            TranscriptSpanRef {
                session_id: SessionId("session-99".into()),
                range: TranscriptRange {
                    start_sequence: 4,
                    end_sequence: 4,
                },
                reason: Some("active_topic_session, query_match".into()),
            },
            TranscriptSpanRef {
                session_id: SessionId(" ".into()),
                range: TranscriptRange {
                    start_sequence: 1,
                    end_sequence: 1,
                },
                reason: None,
            },
        ],
    };

    let summary = inspection.transcript_provenance_summary();

    assert_eq!(
        summary,
        ContextRecallTranscriptProvenanceSummary {
            span_count: 3,
            session_count: 2,
            spans_with_reason_count: 2,
            distinct_reason_count: 3,
        }
    );
    assert!(summary.has_spans());
    assert!(summary.has_reasons());
    assert!(!summary.is_empty());
}

#[test]
fn context_recall_bundle_transcript_provenance_summary_matches_inspection_summary() {
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
        durable_memory_hits: vec![],
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
        total_memory_match_count: 0,
    });

    assert_eq!(
        bundle.transcript_provenance_summary(),
        inspection.transcript_provenance_summary()
    );
}

#[test]
fn context_recall_bundle_builds_bounded_transcript_provenance_refs() {
    let bundle = ContextRecallBundle {
        request: ContextRecallRequest {
            session_id: SessionId("session-42".into()),
            query_text: Some("tool failure".into()),
            recent_window_limit: 3,
            transcript_limit: 2,
            memory_limit: 2,
            allow_cross_session: true,
        },
        recent_entries: vec![
            sample_transcript_entry(8, "run tool"),
            sample_transcript_entry(9, "tool timeout"),
            sample_transcript_entry(10, "retry requested"),
        ],
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
        durable_memory_hits: vec![],
        summary_hits: vec![],
        active_topic_sessions: vec![TopicSession {
            topic_session_id: "topic-session-bootstrap:session-42".into(),
            topic_id: crate::TopicId("topic-session-42".into()),
            topic_label: crate::TopicLabel("tool failure".into()),
            topic_embedding: None,
            linked_surface_session_ids: vec![SessionId("session-42".into())],
            linked_transcript_spans: vec![TranscriptSpanRef {
                session_id: SessionId("session-42".into()),
                range: TranscriptRange {
                    start_sequence: 8,
                    end_sequence: 10,
                },
                reason: Some("query_match".into()),
            }],
            open_loops: vec![],
            entities: BTreeMap::new(),
            graph_edges: vec![],
            durable_memory_refs: vec![],
            status: TopicSessionStatus::Active,
            created_at_unix_ms: 100,
            last_active_unix_ms: 110,
        }],
        active_neurons: Vec::new(),
        budget: ContextBudget::default(),
        ranked_items: Vec::new(),
        omitted_by_budget: 0,
        truncated: false,
    };

    let refs = bundle.source_transcript_spans();

    assert_eq!(refs.len(), 1);
    assert_eq!(refs[0].session_id.0, "session-42");
    assert_eq!(refs[0].range.start_sequence, 8);
    assert_eq!(refs[0].range.end_sequence, 10);
    assert_eq!(
        refs[0].reason.as_deref(),
        Some("recent_window, query_match, active_topic_session")
    );
}
