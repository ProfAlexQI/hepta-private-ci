use super::*;

#[test]
fn context_recall_memory_taxonomy_report_maps_recall_counts_without_payloads() {
    let inspection = ContextRecallInspection {
        report: ContextRecallReport {
            request: ContextRecallRequest {
                session_id: SessionId("session-42".into()),
                query_text: Some("timeout".into()),
                recent_window_limit: 2,
                transcript_limit: 1,
                memory_limit: 2,
                allow_cross_session: true,
            },
            source_counts: ContextRecallSourceCounts {
                recent_entry_count: 2,
                transcript_hit_count: 1,
                durable_memory_hit_count: 1,
                summary_hit_count: 1,
            },
            truncated: true,
        },
        availability: ContextRecallAvailability {
            total_recent_entry_count: 4,
            total_transcript_match_count: 3,
            total_memory_match_count: 5,
        },
        source_transcript_spans: vec![
            TranscriptSpanRef {
                session_id: SessionId("session-42".into()),
                range: TranscriptRange {
                    start_sequence: 9,
                    end_sequence: 10,
                },
                reason: Some("recent_window".into()),
            },
            TranscriptSpanRef {
                session_id: SessionId("session-42".into()),
                range: TranscriptRange {
                    start_sequence: 8,
                    end_sequence: 8,
                },
                reason: Some("query_match".into()),
            },
        ],
    };
    let source_availability = ContextRecallSourceAvailability {
        recent_entry_count: 4,
        transcript_match_count: 3,
        durable_memory_match_count: 2,
        summary_memory_match_count: 3,
    };

    let taxonomy = inspection.memory_taxonomy_report(&source_availability, 2);

    assert!(taxonomy.has_count_integrity());
    assert_eq!(
        taxonomy,
        ContextMemoryTaxonomyReport {
            buckets: vec![
                ContextMemoryTaxonomyBucket {
                    class: ContextMemoryTaxonomyClass::Semantic,
                    source_count: 1,
                    returned_count: 1,
                    available_count: 2,
                    omitted_count: 1,
                    provenance_span_count: 0,
                },
                ContextMemoryTaxonomyBucket {
                    class: ContextMemoryTaxonomyClass::Episodic,
                    source_count: 1,
                    returned_count: 1,
                    available_count: 3,
                    omitted_count: 2,
                    provenance_span_count: 0,
                },
                ContextMemoryTaxonomyBucket {
                    class: ContextMemoryTaxonomyClass::Control,
                    source_count: 1,
                    returned_count: 0,
                    available_count: 2,
                    omitted_count: 2,
                    provenance_span_count: 0,
                },
                ContextMemoryTaxonomyBucket {
                    class: ContextMemoryTaxonomyClass::Transcript,
                    source_count: 2,
                    returned_count: 3,
                    available_count: 7,
                    omitted_count: 4,
                    provenance_span_count: 2,
                },
            ],
        }
    );

    let json = serde_json::to_string(&taxonomy).expect("taxonomy report should serialize");
    assert!(json.contains("semantic"));
    assert!(json.contains("episodic"));
    assert!(json.contains("control"));
    assert!(json.contains("transcript"));
    assert!(!json.contains("memory-"));
    assert!(!json.contains("payload"));
    assert!(!json.contains("source_id"));
    assert!(!json.contains("query_text"));
}
