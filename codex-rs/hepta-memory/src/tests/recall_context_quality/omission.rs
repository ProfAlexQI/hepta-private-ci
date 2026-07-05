use super::*;

#[test]
fn store_snapshot_recall_context_omission_counts_match_coverage_helper() {
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

    let omission_counts = snapshot.recall_context_omission_counts(&request);

    assert_eq!(
        omission_counts,
        snapshot.recall_context_coverage(&request).omission_counts()
    );
    assert_eq!(
        omission_counts,
        ContextRecallOmissionCounts {
            recent_entry_count: 1,
            transcript_hit_count: 1,
            memory_hit_count: 1,
            query_hit_count: 2,
            total_item_count: 3,
        }
    );
    assert!(omission_counts.has_omissions());
    assert!(!omission_counts.is_empty());
}

#[test]
fn store_snapshot_recall_context_with_zero_limits_reports_full_omission_pressure() {
    let snapshot = StoreSnapshot {
        sessions: vec![],
        memories: vec![
            memory_record("memory-1", MemoryScope::LongTerm, "timeout retry guidance"),
            memory_record("memory-2", MemoryScope::Session, "session timeout summary"),
        ],
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
        recent_window_limit: 0,
        transcript_limit: 0,
        memory_limit: 0,
        allow_cross_session: true,
    };

    let bundle = snapshot.recall_context(&request);
    let inspection = snapshot.recall_context_inspection(&request);
    let coverage = snapshot.recall_context_coverage(&request);
    let pressure = snapshot.recall_context_limit_pressure(&request);

    assert!(bundle.recent_entries.is_empty());
    assert!(bundle.transcript_hits.is_empty());
    assert!(bundle.durable_memory_hits.is_empty());
    assert!(bundle.summary_hits.is_empty());
    assert!(bundle.truncated);
    assert_eq!(inspection.availability.total_recent_entry_count, 2);
    assert_eq!(inspection.availability.total_transcript_match_count, 2);
    assert_eq!(inspection.availability.total_memory_match_count, 2);
    assert_eq!(
        coverage,
        ContextRecallCoverage {
            recent_entries: ContextRecallCoverageCounts {
                returned_count: 0,
                available_count: 2,
            },
            transcript_hits: ContextRecallCoverageCounts {
                returned_count: 0,
                available_count: 2,
            },
            memory_hits: ContextRecallCoverageCounts {
                returned_count: 0,
                available_count: 2,
            },
            query_hits: ContextRecallCoverageCounts {
                returned_count: 0,
                available_count: 4,
            },
            total_items: ContextRecallCoverageCounts {
                returned_count: 0,
                available_count: 6,
            },
        }
    );
    assert_eq!(
        pressure,
        ContextRecallLimitPressure {
            recent_entries_truncated: true,
            transcript_hits_truncated: true,
            memory_hits_truncated: true,
            omission_counts: ContextRecallOmissionCounts {
                recent_entry_count: 2,
                transcript_hit_count: 2,
                memory_hit_count: 2,
                query_hit_count: 4,
                total_item_count: 6,
            },
        }
    );
    assert!(pressure.query_hits_truncated());
    assert!(pressure.has_omissions());
    assert!(!pressure.is_complete());
}
