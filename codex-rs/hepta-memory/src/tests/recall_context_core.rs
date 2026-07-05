use super::*;

#[test]
fn store_snapshot_recall_context_uses_recent_window_query_hits_and_scope_split() {
    let snapshot = StoreSnapshot {
        sessions: vec![],
        memories: vec![
            memory_record("memory-1", MemoryScope::LongTerm, "timeout retry guidance"),
            memory_record("memory-2", MemoryScope::Session, "session timeout summary"),
            memory_record("memory-3", MemoryScope::LongTerm, "unrelated note"),
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
        memory_limit: 3,
        allow_cross_session: true,
    };

    let bundle = snapshot.recall_context(&request);

    assert_eq!(bundle.recent_entries.len(), 2);
    assert_eq!(bundle.recent_entries[0].sequence, 2);
    assert_eq!(bundle.recent_entries[1].sequence, 3);
    assert_eq!(bundle.transcript_hits.len(), 1);
    assert_eq!(bundle.transcript_hits[0].range.start_sequence, 3);
    assert_eq!(bundle.durable_memory_hits.len(), 1);
    assert_eq!(bundle.durable_memory_hits[0].id, "memory-1");
    assert_eq!(bundle.summary_hits.len(), 1);
    assert_eq!(bundle.summary_hits[0].id, "memory-2");
    assert_eq!(
        bundle.source_counts(),
        ContextRecallSourceCounts {
            recent_entry_count: 2,
            transcript_hit_count: 1,
            durable_memory_hit_count: 1,
            summary_hit_count: 1,
        }
    );
    assert_eq!(bundle.query_hit_count(), 3);
    assert_eq!(bundle.total_item_count(), 5);
    assert!(bundle.truncated);
}

#[test]
fn store_snapshot_recall_context_treats_blank_query_as_memory_only_default_search() {
    let snapshot = StoreSnapshot {
        sessions: vec![],
        memories: vec![
            memory_record(
                "memory-1",
                MemoryScope::LongTerm,
                "always included for blank query",
            ),
            memory_record("memory-2", MemoryScope::Session, "session summary fallback"),
        ],
        transcripts: vec![transcript_entry(
            "session-1",
            1,
            TranscriptEntryKind::Message,
            "timeout surfaced during tool run",
        )],
    };
    let request = ContextRecallRequest {
        session_id: SessionId("session-1".into()),
        query_text: Some("   ".into()),
        recent_window_limit: 4,
        transcript_limit: 5,
        memory_limit: 1,
        allow_cross_session: true,
    };

    let bundle = snapshot.recall_context(&request);

    assert_eq!(bundle.recent_entries.len(), 1);
    assert!(bundle.transcript_hits.is_empty());
    assert_eq!(bundle.durable_memory_hits.len(), 1);
    assert_eq!(bundle.durable_memory_hits[0].id, "memory-1");
    assert!(bundle.summary_hits.is_empty());
    assert_eq!(
        bundle.source_counts(),
        ContextRecallSourceCounts {
            recent_entry_count: 1,
            transcript_hit_count: 0,
            durable_memory_hit_count: 1,
            summary_hit_count: 0,
        }
    );
    assert!(bundle.has_query_matches());
    assert!(bundle.truncated);
}

#[test]
fn store_snapshot_recall_context_treats_cross_session_flag_as_advisory_only() {
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
                "session-2",
                1,
                TranscriptEntryKind::Message,
                "timeout in another session",
            ),
        ],
    };
    let session_scoped = ContextRecallRequest {
        session_id: SessionId("session-1".into()),
        query_text: Some("timeout".into()),
        recent_window_limit: 4,
        transcript_limit: 4,
        memory_limit: 4,
        allow_cross_session: false,
    };
    let mut cross_session = session_scoped.clone();
    cross_session.allow_cross_session = true;

    let session_scoped_bundle = snapshot.recall_context(&session_scoped);
    let cross_session_bundle = snapshot.recall_context(&cross_session);

    assert_eq!(
        session_scoped_bundle.recent_entries,
        cross_session_bundle.recent_entries
    );
    assert_eq!(
        session_scoped_bundle.transcript_hits,
        cross_session_bundle.transcript_hits
    );
    assert_eq!(
        session_scoped_bundle.durable_memory_hits,
        cross_session_bundle.durable_memory_hits
    );
    assert_eq!(
        session_scoped_bundle.summary_hits,
        cross_session_bundle.summary_hits
    );
    assert_eq!(session_scoped_bundle.transcript_hits.len(), 1);
    assert_eq!(
        session_scoped_bundle.transcript_hits[0].session_id,
        SessionId("session-1".into())
    );
    assert_eq!(session_scoped_bundle.query_hit_count(), 3);
    assert_eq!(cross_session_bundle.query_hit_count(), 3);
    assert!(!session_scoped_bundle.truncated);
    assert!(!cross_session_bundle.truncated);
    assert!(!session_scoped_bundle.request.allow_cross_session);
    assert!(cross_session_bundle.request.allow_cross_session);
}

#[test]
fn store_snapshot_recall_context_prefers_fresh_transcript_in_mixed_tier_drift() {
    let snapshot = StoreSnapshot {
        sessions: vec![],
        memories: vec![
            memory_record(
                "old-durable",
                MemoryScope::LongTerm,
                "quartz rollout used the retired alpha plan",
            ),
            memory_record(
                "low-quality-summary",
                MemoryScope::Session,
                "low-quality quartz summary still mentions alpha",
            ),
        ],
        transcripts: vec![
            transcript_entry(
                "session-1",
                1,
                TranscriptEntryKind::Message,
                "quartz rollout initially considered alpha",
            ),
            transcript_entry(
                "session-1",
                2,
                TranscriptEntryKind::Message,
                "unrelated status check",
            ),
            transcript_entry(
                "session-1",
                3,
                TranscriptEntryKind::Message,
                "quartz rollout now uses the beta plan",
            ),
            transcript_entry(
                "session-2",
                99,
                TranscriptEntryKind::Message,
                "quartz from another session should not satisfy this request",
            ),
        ],
    };
    let request = ContextRecallRequest {
        session_id: SessionId("session-1".into()),
        query_text: Some("quartz".into()),
        recent_window_limit: 1,
        transcript_limit: 1,
        memory_limit: 1,
        allow_cross_session: true,
    };

    let bundle = snapshot.recall_context(&request);
    let inspection = snapshot.recall_context_inspection(&request);
    let source_availability = snapshot.recall_context_source_availability(&request);
    let pressure = snapshot.recall_context_limit_pressure(&request);

    assert_eq!(bundle.recent_entries.len(), 1);
    assert_eq!(bundle.recent_entries[0].sequence, 3);
    assert_eq!(bundle.transcript_hits.len(), 1);
    assert_eq!(
        bundle.transcript_hits[0].session_id,
        SessionId("session-1".into())
    );
    assert_eq!(bundle.transcript_hits[0].range.start_sequence, 3);
    assert_eq!(bundle.durable_memory_hits.len(), 1);
    assert_eq!(bundle.durable_memory_hits[0].id, "old-durable");
    assert!(bundle.summary_hits.is_empty());
    assert_eq!(
        bundle.source_counts(),
        ContextRecallSourceCounts {
            recent_entry_count: 1,
            transcript_hit_count: 1,
            durable_memory_hit_count: 1,
            summary_hit_count: 0,
        }
    );
    assert_eq!(inspection.availability.total_transcript_match_count, 2);
    assert_eq!(inspection.availability.total_memory_match_count, 2);
    assert_eq!(
        source_availability,
        ContextRecallSourceAvailability {
            recent_entry_count: 3,
            transcript_match_count: 2,
            durable_memory_match_count: 1,
            summary_memory_match_count: 1,
        }
    );
    assert!(pressure.transcript_hits_truncated);
    assert!(pressure.memory_hits_truncated);
    assert!(pressure.has_omissions());
}

#[test]
fn store_snapshot_recall_context_blocks_tombstone_conflict_memory_controls() {
    let snapshot = StoreSnapshot {
        sessions: vec![],
        memories: vec![
            memory_record(
                "memory-live",
                MemoryScope::LongTerm,
                "rollback policy current canonical decision",
            ),
            memory_record(
                "memory-tombstone",
                MemoryScope::LongTerm,
                "[hepta-memory:tombstone] rollback policy obsolete decision",
            ),
            memory_record(
                "summary-conflict",
                MemoryScope::Session,
                "[hepta-memory:conflict] rollback policy stale session summary",
            ),
        ],
        transcripts: vec![],
    };
    let request = ContextRecallRequest {
        session_id: SessionId("session-1".into()),
        query_text: Some("rollback policy".into()),
        recent_window_limit: 4,
        transcript_limit: 4,
        memory_limit: 4,
        allow_cross_session: true,
    };

    let bundle = snapshot.recall_context(&request);
    let inspection = snapshot.recall_context_inspection(&request);
    let source_availability = snapshot.recall_context_source_availability(&request);
    let serialized = serde_json::to_string(&bundle).expect("bundle should serialize");

    assert_eq!(bundle.durable_memory_hits.len(), 1);
    assert_eq!(bundle.durable_memory_hits[0].id, "memory-live");
    assert!(bundle.summary_hits.is_empty());
    assert_eq!(bundle.query_hit_count(), 1);
    assert_eq!(bundle.total_item_count(), 1);
    assert!(!bundle.truncated);
    assert_eq!(
        bundle.source_counts(),
        ContextRecallSourceCounts {
            recent_entry_count: 0,
            transcript_hit_count: 0,
            durable_memory_hit_count: 1,
            summary_hit_count: 0,
        }
    );
    assert_eq!(inspection.availability.total_memory_match_count, 1);
    assert_eq!(
        source_availability,
        ContextRecallSourceAvailability {
            recent_entry_count: 0,
            transcript_match_count: 0,
            durable_memory_match_count: 1,
            summary_memory_match_count: 0,
        }
    );
    assert!(!serialized.contains(MEMORY_RECALL_TOMBSTONE_MARKER));
    assert!(!serialized.contains(MEMORY_RECALL_CONFLICT_MARKER));
}

#[test]
fn store_snapshot_recall_context_report_matches_bundle_report() {
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
        recent_window_limit: 2,
        transcript_limit: 1,
        memory_limit: 5,
        allow_cross_session: true,
    };

    let report = snapshot.recall_context_report(&request);
    let bundle = snapshot.recall_context(&request);

    assert_eq!(report, bundle.report());
    assert_eq!(report.request, request);
    assert_eq!(report.source_counts, bundle.source_counts());
    assert_eq!(report.query_hit_count(), 3);
    assert_eq!(report.total_item_count(), 5);
    assert!(report.has_query_matches());
    assert!(!report.is_empty());
    assert!(report.truncated);
}

#[test]
fn store_snapshot_recall_context_report_is_payload_light_across_query_boundaries() {
    let durable_payload = "apollo durable memory payload should not serialize";
    let summary_payload = "apollo session summary payload should not serialize";
    let transcript_payload = "apollo transcript payload should not serialize";
    let tombstone_payload = "[hepta-memory:tombstone] apollo tombstone payload";
    let conflict_payload = "[hepta-memory:conflict] apollo conflict payload";
    let snapshot = StoreSnapshot {
        sessions: vec![],
        memories: vec![
            memory_record("memory-durable", MemoryScope::LongTerm, durable_payload),
            memory_record("memory-summary", MemoryScope::Session, summary_payload),
            memory_record("memory-tombstone", MemoryScope::LongTerm, tombstone_payload),
            memory_record("memory-conflict", MemoryScope::Session, conflict_payload),
        ],
        transcripts: vec![
            transcript_entry(
                "session-1",
                1,
                TranscriptEntryKind::Message,
                transcript_payload,
            ),
            transcript_entry(
                "session-1",
                2,
                TranscriptEntryKind::Summary,
                "apollo second transcript payload should not serialize",
            ),
        ],
    };
    let request = ContextRecallRequest {
        session_id: SessionId("session-1".into()),
        query_text: Some("apollo".into()),
        recent_window_limit: 1,
        transcript_limit: 1,
        memory_limit: 1,
        allow_cross_session: true,
    };

    let report = snapshot.recall_context_report(&request);
    let serialized = serde_json::to_string(&report).expect("report should serialize");

    assert_eq!(report.request, request);
    assert_eq!(
        report.source_counts,
        ContextRecallSourceCounts {
            recent_entry_count: 1,
            transcript_hit_count: 1,
            durable_memory_hit_count: 1,
            summary_hit_count: 0,
        }
    );
    assert_eq!(report.query_hit_count(), 2);
    assert_eq!(report.total_item_count(), 3);
    assert!(report.truncated);
    for forbidden in [
        durable_payload,
        summary_payload,
        transcript_payload,
        tombstone_payload,
        conflict_payload,
        "second transcript payload",
        "memory-durable",
        "memory-summary",
        "memory-tombstone",
        "memory-conflict",
        MEMORY_RECALL_TOMBSTONE_MARKER,
        MEMORY_RECALL_CONFLICT_MARKER,
    ] {
        assert!(
            !serialized.contains(forbidden),
            "recall report leaked payload marker: {forbidden}"
        );
    }
}
