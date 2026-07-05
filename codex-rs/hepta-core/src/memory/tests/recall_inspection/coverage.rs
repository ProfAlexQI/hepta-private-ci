use super::*;

#[test]
fn context_recall_coverage_counts_track_omissions_and_completeness() {
    let counts = ContextRecallCoverageCounts {
        returned_count: 2,
        available_count: 5,
    };

    assert_eq!(counts.omitted_count(), 3);
    assert!(counts.is_truncated());
    assert!(!counts.is_complete());
    assert!(!counts.is_empty());
}

#[test]
fn context_recall_coverage_rolls_up_inspection_counts() {
    let inspection = ContextRecallInspection {
        report: ContextRecallReport {
            request: ContextRecallRequest {
                session_id: SessionId("session-42".into()),
                query_text: Some("timeout".into()),
                recent_window_limit: 2,
                transcript_limit: 1,
                memory_limit: 1,
                allow_cross_session: true,
            },
            source_counts: ContextRecallSourceCounts {
                recent_entry_count: 2,
                transcript_hit_count: 1,
                durable_memory_hit_count: 1,
                summary_hit_count: 0,
            },
            truncated: true,
        },
        availability: ContextRecallAvailability {
            total_recent_entry_count: 4,
            total_transcript_match_count: 3,
            total_memory_match_count: 2,
        },
        source_transcript_spans: vec![TranscriptSpanRef {
            session_id: SessionId("session-42".into()),
            range: TranscriptRange {
                start_sequence: 9,
                end_sequence: 10,
            },
            reason: Some("recent_window".into()),
        }],
    };

    let coverage = inspection.coverage();

    assert_eq!(
        coverage,
        ContextRecallCoverage {
            recent_entries: ContextRecallCoverageCounts {
                returned_count: 2,
                available_count: 4,
            },
            transcript_hits: ContextRecallCoverageCounts {
                returned_count: 1,
                available_count: 3,
            },
            memory_hits: ContextRecallCoverageCounts {
                returned_count: 1,
                available_count: 2,
            },
            query_hits: ContextRecallCoverageCounts {
                returned_count: 2,
                available_count: 5,
            },
            total_items: ContextRecallCoverageCounts {
                returned_count: 4,
                available_count: 9,
            },
        }
    );
    assert_eq!(coverage.omitted_total_item_count(), 5);
    assert_eq!(
        coverage.omission_counts(),
        ContextRecallOmissionCounts {
            recent_entry_count: 2,
            transcript_hit_count: 2,
            memory_hit_count: 1,
            query_hit_count: 3,
            total_item_count: 5,
        }
    );
    assert!(coverage.has_omissions());
    assert!(!coverage.is_complete());
    assert!(!coverage.is_empty());
}

#[test]
fn context_recall_coverage_roundtrips_through_json() {
    let coverage = ContextRecallCoverage {
        recent_entries: ContextRecallCoverageCounts {
            returned_count: 2,
            available_count: 2,
        },
        transcript_hits: ContextRecallCoverageCounts {
            returned_count: 1,
            available_count: 1,
        },
        memory_hits: ContextRecallCoverageCounts {
            returned_count: 3,
            available_count: 3,
        },
        query_hits: ContextRecallCoverageCounts {
            returned_count: 4,
            available_count: 4,
        },
        total_items: ContextRecallCoverageCounts {
            returned_count: 6,
            available_count: 6,
        },
    };

    let json = serde_json::to_string(&coverage).expect("coverage should serialize");
    let parsed: ContextRecallCoverage =
        serde_json::from_str(&json).expect("coverage should deserialize");

    assert_eq!(parsed, coverage);
    assert!(!parsed.has_omissions());
    assert!(parsed.is_complete());
    assert!(!parsed.is_empty());
}

#[test]
fn context_recall_coverage_deserializes_from_sparse_json() {
    let parsed: ContextRecallCoverage =
        serde_json::from_str("{}").expect("sparse coverage should deserialize with defaults");

    assert_eq!(parsed, ContextRecallCoverage::default());
    assert_eq!(parsed.omitted_total_item_count(), 0);
    assert!(!parsed.has_omissions());
    assert!(parsed.is_complete());
    assert!(parsed.is_empty());
}

#[test]
fn context_recall_coverage_limit_pressure_matches_coverage_omissions() {
    let coverage = ContextRecallCoverage {
        recent_entries: ContextRecallCoverageCounts {
            returned_count: 2,
            available_count: 4,
        },
        transcript_hits: ContextRecallCoverageCounts {
            returned_count: 1,
            available_count: 3,
        },
        memory_hits: ContextRecallCoverageCounts {
            returned_count: 1,
            available_count: 2,
        },
        query_hits: ContextRecallCoverageCounts {
            returned_count: 2,
            available_count: 5,
        },
        total_items: ContextRecallCoverageCounts {
            returned_count: 4,
            available_count: 9,
        },
    };

    let pressure = coverage.limit_pressure();

    assert_eq!(
        pressure,
        ContextRecallLimitPressure {
            recent_entries_truncated: true,
            transcript_hits_truncated: true,
            memory_hits_truncated: true,
            omission_counts: ContextRecallOmissionCounts {
                recent_entry_count: 2,
                transcript_hit_count: 2,
                memory_hit_count: 1,
                query_hit_count: 3,
                total_item_count: 5,
            },
        }
    );
    assert!(pressure.query_hits_truncated());
    assert!(pressure.has_omissions());
    assert!(!pressure.is_complete());
}
