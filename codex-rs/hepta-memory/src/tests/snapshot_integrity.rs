use super::*;

#[tokio::test]
async fn snapshot_integrity_report_flags_duplicate_and_blank_records() {
    let store = InMemoryStore::default();

    store
        .create(session_record(
            "session-1",
            "Foundation",
            Some("audit integrity"),
        ))
        .await
        .expect("create should succeed");
    store
        .create(session_record("session-1", "   ", None))
        .await
        .expect("create should succeed");
    store
        .create(session_record("   ", "Blank session id", None))
        .await
        .expect("create should succeed");
    store
        .put(memory_record(
            "memory-1",
            MemoryScope::Session,
            "snapshot payload",
        ))
        .await
        .expect("put should succeed");
    store
        .put(memory_record("memory-1", MemoryScope::LongTerm, "   "))
        .await
        .expect("put should succeed");
    store
        .put(memory_record(
            " ",
            MemoryScope::LongTerm,
            "manifest payload",
        ))
        .await
        .expect("put should succeed");

    let report = store
        .snapshot_integrity_report()
        .expect("integrity report should load");

    assert_eq!(
        report.duplicate_session_ids,
        vec![SessionId("session-1".into())]
    );
    assert_eq!(report.duplicate_memory_ids, vec!["memory-1".to_string()]);
    assert_eq!(report.blank_session_id_count, 1);
    assert_eq!(report.blank_memory_id_count, 1);
    assert_eq!(report.blank_session_title_count, 1);
    assert_eq!(report.blank_memory_content_count, 1);
    assert_eq!(report.issue_count(), 6);
    assert!(!report.is_clean());
}

#[tokio::test]
async fn transcript_snapshot_integrity_report_flags_duplicate_and_blank_entries() {
    let store = InMemoryStore::default();

    store
        .append(TranscriptEntry {
            entry_id: "entry-1".into(),
            session_id: SessionId("session-1".into()),
            sequence: 1,
            kind: TranscriptEntryKind::Message,
            role: Some(MessageRole::User),
            content: "hello".into(),
            created_at_unix_ms: 1,
            tool_name: None,
            correlation_id: None,
            summary_of_range: None,
        })
        .await
        .expect("append should succeed");
    store
        .append(TranscriptEntry {
            entry_id: "entry-1".into(),
            session_id: SessionId("session-1".into()),
            sequence: 1,
            kind: TranscriptEntryKind::ToolResult,
            role: Some(MessageRole::Tool),
            content: "result".into(),
            created_at_unix_ms: 2,
            tool_name: Some("write".into()),
            correlation_id: None,
            summary_of_range: None,
        })
        .await
        .expect("append should succeed");
    store
        .append(TranscriptEntry {
            entry_id: "   ".into(),
            session_id: SessionId("   ".into()),
            sequence: 2,
            kind: TranscriptEntryKind::Event,
            role: None,
            content: "   ".into(),
            created_at_unix_ms: 3,
            tool_name: None,
            correlation_id: None,
            summary_of_range: None,
        })
        .await
        .expect("append should succeed");

    let report = store
        .transcript_snapshot_integrity_report()
        .expect("transcript integrity report should load");

    assert_eq!(report.duplicate_entry_ids, vec!["entry-1".to_string()]);
    assert_eq!(report.blank_entry_id_count, 1);
    assert_eq!(report.blank_session_id_count, 1);
    assert_eq!(report.blank_content_count, 1);
    assert_eq!(report.duplicate_sequence_collisions.len(), 1);
    assert_eq!(
        report.duplicate_sequence_collisions[0].session_id.0,
        "session-1"
    );
    assert_eq!(report.duplicate_sequence_collisions[0].sequence, 1);
    assert_eq!(report.issue_count(), 5);
    assert!(!report.is_clean());
}

#[test]
fn store_snapshot_integrity_report_matches_clean_snapshot() {
    let snapshot = StoreSnapshot {
        sessions: vec![session_record(
            "session-1",
            "Foundation",
            Some("manifest alignment"),
        )],
        memories: vec![memory_record(
            "memory-1",
            MemoryScope::LongTerm,
            "manifest alignment payload",
        )],
        transcripts: vec![],
    };

    let report = snapshot.integrity_report();

    assert_eq!(report, MemorySnapshotIntegrityReport::default());
    assert!(report.is_clean());
}
