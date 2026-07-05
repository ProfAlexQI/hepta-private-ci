use super::*;

#[test]
fn context_recall_bundle_roundtrips_through_json() {
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

    let json = serde_json::to_string(&bundle).expect("context recall bundle should serialize");
    let parsed: ContextRecallBundle =
        serde_json::from_str(&json).expect("context recall bundle should deserialize");

    assert_eq!(parsed, bundle);
    assert!(!parsed.is_empty());
    assert_eq!(parsed.active_topic_session_count(), 0);
    assert_eq!(parsed.transcript_hits[0].range.start_sequence, 8);
    assert_eq!(
        parsed.source_counts(),
        ContextRecallSourceCounts {
            recent_entry_count: 1,
            transcript_hit_count: 1,
            durable_memory_hit_count: 1,
            summary_hit_count: 1,
        }
    );
}

#[test]
fn context_recall_source_counts_roundtrip_and_totals_stay_compact() {
    let counts = ContextRecallSourceCounts {
        recent_entry_count: 2,
        transcript_hit_count: 3,
        durable_memory_hit_count: 1,
        summary_hit_count: 4,
    };

    let json = serde_json::to_string(&counts).expect("source counts should serialize");
    let parsed: ContextRecallSourceCounts =
        serde_json::from_str(&json).expect("source counts should deserialize");

    assert_eq!(parsed, counts);
    assert_eq!(parsed.query_hit_count(), 8);
    assert_eq!(parsed.total_item_count(), 10);
    assert!(parsed.has_query_matches());
    assert!(!parsed.is_empty());
}

#[test]
fn context_recall_source_counts_deserialize_from_sparse_json() {
    let parsed: ContextRecallSourceCounts =
        serde_json::from_str("{}").expect("sparse source counts should deserialize with defaults");

    assert_eq!(parsed, ContextRecallSourceCounts::default());
    assert_eq!(parsed.query_hit_count(), 0);
    assert_eq!(parsed.total_item_count(), 0);
    assert!(!parsed.has_query_matches());
    assert!(parsed.is_empty());
}

#[test]
fn context_recall_report_roundtrips_through_json() {
    let report = ContextRecallReport {
        request: ContextRecallRequest {
            session_id: SessionId("session-42".into()),
            query_text: Some("tool failure".into()),
            recent_window_limit: 8,
            transcript_limit: 3,
            memory_limit: 2,
            allow_cross_session: true,
        },
        source_counts: ContextRecallSourceCounts {
            recent_entry_count: 1,
            transcript_hit_count: 1,
            durable_memory_hit_count: 1,
            summary_hit_count: 1,
        },
        truncated: false,
    };

    let json = serde_json::to_string(&report).expect("context recall report should serialize");
    let parsed: ContextRecallReport =
        serde_json::from_str(&json).expect("context recall report should deserialize");

    assert_eq!(parsed, report);
    assert_eq!(parsed.query_hit_count(), 3);
    assert_eq!(parsed.total_item_count(), 4);
    assert!(parsed.has_query_matches());
    assert!(!parsed.is_empty());
}

#[test]
fn context_recall_availability_roundtrips_through_json() {
    let availability = ContextRecallAvailability {
        total_recent_entry_count: 5,
        total_transcript_match_count: 3,
        total_memory_match_count: 2,
    };

    let json = serde_json::to_string(&availability).expect("availability should serialize");
    let parsed: ContextRecallAvailability =
        serde_json::from_str(&json).expect("availability should deserialize");

    assert_eq!(parsed, availability);
    assert_eq!(parsed.query_match_count(), 5);
    assert_eq!(parsed.total_item_count(), 10);
    assert!(parsed.has_query_matches());
    assert!(!parsed.is_empty());
}

#[test]
fn context_recall_availability_deserializes_from_sparse_json() {
    let parsed: ContextRecallAvailability =
        serde_json::from_str("{}").expect("sparse availability should deserialize with defaults");

    assert_eq!(parsed, ContextRecallAvailability::default());
    assert_eq!(parsed.query_match_count(), 0);
    assert_eq!(parsed.total_item_count(), 0);
    assert!(!parsed.has_query_matches());
    assert!(parsed.is_empty());
}

#[test]
fn context_recall_source_availability_roundtrips_through_json() {
    let availability = ContextRecallSourceAvailability {
        recent_entry_count: 5,
        transcript_match_count: 3,
        durable_memory_match_count: 2,
        summary_memory_match_count: 4,
    };

    let json = serde_json::to_string(&availability).expect("source availability should serialize");
    let parsed: ContextRecallSourceAvailability =
        serde_json::from_str(&json).expect("source availability should deserialize");

    assert_eq!(parsed, availability);
    assert_eq!(parsed.memory_match_count(), 6);
    assert_eq!(parsed.query_match_count(), 9);
    assert_eq!(parsed.total_item_count(), 14);
    assert!(parsed.has_query_matches());
    assert!(!parsed.is_empty());
}

#[test]
fn context_recall_source_availability_deserializes_from_sparse_json() {
    let parsed: ContextRecallSourceAvailability = serde_json::from_str("{}")
        .expect("sparse source availability should deserialize with defaults");

    assert_eq!(parsed, ContextRecallSourceAvailability::default());
    assert_eq!(parsed.memory_match_count(), 0);
    assert_eq!(parsed.query_match_count(), 0);
    assert_eq!(parsed.total_item_count(), 0);
    assert!(!parsed.has_query_matches());
    assert!(parsed.is_empty());
}
