use super::*;

#[test]
fn store_snapshot_recall_context_coverage_matches_inspection_helper() {
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

    let coverage = snapshot.recall_context_coverage(&request);

    assert_eq!(
        coverage,
        snapshot.recall_context_inspection(&request).coverage()
    );
    assert_eq!(
        coverage,
        ContextRecallCoverage {
            recent_entries: ContextRecallCoverageCounts {
                returned_count: 2,
                available_count: 3,
            },
            transcript_hits: ContextRecallCoverageCounts {
                returned_count: 1,
                available_count: 2,
            },
            memory_hits: ContextRecallCoverageCounts {
                returned_count: 2,
                available_count: 3,
            },
            query_hits: ContextRecallCoverageCounts {
                returned_count: 3,
                available_count: 5,
            },
            total_items: ContextRecallCoverageCounts {
                returned_count: 5,
                available_count: 8,
            },
        }
    );
    assert_eq!(coverage.omitted_total_item_count(), 3);
    assert_eq!(
        coverage.omission_counts(),
        ContextRecallOmissionCounts {
            recent_entry_count: 1,
            transcript_hit_count: 1,
            memory_hit_count: 1,
            query_hit_count: 2,
            total_item_count: 3,
        }
    );
    assert!(coverage.has_omissions());
    assert!(!coverage.is_complete());
}
