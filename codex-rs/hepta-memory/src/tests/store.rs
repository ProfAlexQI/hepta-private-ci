use super::*;

#[test]
fn upsert_session_sync_replaces_existing_record_without_duplication() {
    let store = InMemoryStore::default();

    store
        .upsert_session_sync(session_record("session-1", "Initial", Some("draft")))
        .expect("first upsert should succeed");
    store
        .upsert_session_sync(session_record("session-1", "Renamed", Some("finalize")))
        .expect("second upsert should succeed");

    let sessions = store.list_sessions().expect("sessions should load");
    assert_eq!(sessions.len(), 1);
    assert_eq!(sessions[0].title, "Renamed");
    assert_eq!(
        sessions[0].last_user_intent_summary.as_deref(),
        Some("finalize")
    );
}

#[test]
fn remove_session_sync_returns_removed_record_and_updates_store() {
    let store = InMemoryStore::default();
    let record = session_record("session-1", "Foundation", None);
    let session_id = record.session_id.clone();

    store
        .upsert_session_sync(record.clone())
        .expect("upsert should succeed");

    let removed = store
        .remove_session_sync(&session_id)
        .expect("remove should succeed");

    assert_eq!(removed, Some(record));
    assert!(
        store
            .list_sessions()
            .expect("sessions should load")
            .is_empty()
    );
}

#[test]
fn sync_listing_helpers_surface_memories_and_transcripts() {
    let store = InMemoryStore::default();
    let memory = memory_record(
        "memory-1",
        MemoryScope::LongTerm,
        "listed through sync helper",
    );
    let transcript = transcript_entry(
        "session-1",
        1,
        TranscriptEntryKind::Summary,
        "appended through sync helper",
    );

    store
        .restore(StoreSnapshot {
            sessions: vec![session_record(
                "session-1",
                "Foundation",
                Some("sync listing helpers"),
            )],
            memories: vec![memory.clone()],
            transcripts: vec![],
        })
        .expect("restore should succeed");
    store
        .append_transcript_sync(transcript.clone())
        .expect("sync transcript append should succeed");

    assert_eq!(
        store.list_memories().expect("memory list should load"),
        vec![memory]
    );
    assert_eq!(
        store
            .list_transcript_entries()
            .expect("transcript list should load"),
        vec![transcript]
    );
    assert_eq!(
        store.snapshot().expect("snapshot should load").transcripts,
        store
            .list_transcript_entries()
            .expect("transcript list should load")
    );
}

#[test]
fn memory_context_safety_keeps_transcript_recall_session_scoped_with_cross_session_memory() {
    let store = InMemoryStore::default();
    store
        .restore(StoreSnapshot {
            sessions: vec![
                session_record("session-main", "Main", Some("needle main")),
                session_record("session-other", "Other", Some("needle other")),
            ],
            memories: vec![
                memory_record(
                    "memory-long-term",
                    MemoryScope::LongTerm,
                    "needle durable preference",
                ),
                memory_record(
                    "memory-summary",
                    MemoryScope::Session,
                    "needle session summary",
                ),
            ],
            transcripts: vec![
                transcript_entry(
                    "session-main",
                    1,
                    TranscriptEntryKind::Message,
                    "needle local transcript",
                ),
                transcript_entry(
                    "session-other",
                    1,
                    TranscriptEntryKind::Message,
                    "needle other transcript should stay out",
                ),
            ],
        })
        .expect("restore should succeed");

    let request = ContextRecallRequest {
        session_id: SessionId("session-main".into()),
        query_text: Some("needle".into()),
        recent_window_limit: 5,
        transcript_limit: 5,
        memory_limit: 5,
        allow_cross_session: true,
    };
    let bundle = store
        .recall_context(request.clone())
        .expect("context recall should succeed");
    let report = store
        .recall_context_report(request)
        .expect("context recall report should succeed");

    assert_eq!(bundle.recent_entries.len(), 1);
    assert_eq!(bundle.transcript_hits.len(), 1);
    assert!(
        bundle
            .recent_entries
            .iter()
            .all(|entry| entry.session_id.0 == "session-main")
    );
    assert!(
        bundle
            .transcript_hits
            .iter()
            .all(|span| span.session_id.0 == "session-main")
    );
    assert_eq!(bundle.durable_memory_hits.len(), 1);
    assert_eq!(bundle.summary_hits.len(), 1);
    assert_eq!(report.source_counts.transcript_hit_count, 1);
    assert_eq!(report.source_counts.durable_memory_hit_count, 1);
    assert_eq!(report.source_counts.summary_hit_count, 1);
}

#[test]
fn memory_context_safety_reports_limit_pressure_without_leaking_hidden_context() {
    let store = InMemoryStore::default();
    store
        .restore(StoreSnapshot {
            sessions: vec![session_record(
                "session-main",
                "Main",
                Some("limit pressure"),
            )],
            memories: vec![
                memory_record("memory-1", MemoryScope::LongTerm, "needle alpha"),
                memory_record("memory-2", MemoryScope::LongTerm, "needle beta"),
                memory_record("memory-3", MemoryScope::Session, "needle gamma"),
            ],
            transcripts: vec![
                transcript_entry(
                    "session-main",
                    1,
                    TranscriptEntryKind::Message,
                    "needle transcript alpha",
                ),
                transcript_entry(
                    "session-main",
                    2,
                    TranscriptEntryKind::ToolResult,
                    "needle transcript beta",
                ),
            ],
        })
        .expect("restore should succeed");

    let request = ContextRecallRequest {
        session_id: SessionId("session-main".into()),
        query_text: Some("needle".into()),
        recent_window_limit: 1,
        transcript_limit: 1,
        memory_limit: 1,
        allow_cross_session: false,
    };
    let inspection = store
        .recall_context_inspection(request.clone())
        .expect("inspection should succeed");
    let pressure = store
        .recall_context_limit_pressure(request)
        .expect("pressure should succeed");

    assert!(inspection.report.truncated);
    assert!(pressure.transcript_hits_truncated);
    assert!(pressure.memory_hits_truncated);
    assert_eq!(inspection.availability.total_transcript_match_count, 2);
    assert_eq!(inspection.availability.total_memory_match_count, 3);
    assert_eq!(
        store
            .snapshot()
            .expect("snapshot should load")
            .memories
            .iter()
            .filter(|record| record.content.contains("hidden_runtime_context"))
            .count(),
        0
    );
}
