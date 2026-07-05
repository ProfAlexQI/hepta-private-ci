use super::*;

#[test]
fn context_recall_limit_pressure_rolls_up_inspection_flags_and_omissions() {
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
        source_transcript_spans: vec![],
    };

    let pressure = inspection.limit_pressure();

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
    assert!(!pressure.is_empty());
}

#[test]
fn context_recall_limit_pressure_roundtrips_through_json() {
    let pressure = ContextRecallLimitPressure {
        recent_entries_truncated: false,
        transcript_hits_truncated: true,
        memory_hits_truncated: false,
        omission_counts: ContextRecallOmissionCounts {
            recent_entry_count: 0,
            transcript_hit_count: 2,
            memory_hit_count: 0,
            query_hit_count: 2,
            total_item_count: 2,
        },
    };

    let json = serde_json::to_string(&pressure).expect("limit pressure should serialize");
    let parsed: ContextRecallLimitPressure =
        serde_json::from_str(&json).expect("limit pressure should deserialize");

    assert_eq!(parsed, pressure);
    assert!(parsed.query_hits_truncated());
    assert!(parsed.has_omissions());
    assert!(!parsed.is_complete());
    assert!(!parsed.is_empty());
}

#[test]
fn context_recall_limit_pressure_deserializes_from_sparse_json() {
    let parsed: ContextRecallLimitPressure =
        serde_json::from_str("{}").expect("sparse limit pressure should deserialize with defaults");

    assert_eq!(parsed, ContextRecallLimitPressure::default());
    assert!(!parsed.query_hits_truncated());
    assert!(!parsed.has_omissions());
    assert!(parsed.is_complete());
    assert!(parsed.is_empty());
}
