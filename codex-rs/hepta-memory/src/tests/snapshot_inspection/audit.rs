use super::*;

#[tokio::test]
async fn inspected_snapshot_matches_store_audit_report() {
    let store = InMemoryStore::default();
    store
        .upsert_session_sync(session_record(
            "session-1",
            "Foundation",
            Some("inspect live store"),
        ))
        .expect("upsert should succeed");
    store
        .put(memory_record(
            "memory-1",
            MemoryScope::LongTerm,
            "inspection payload",
        ))
        .await
        .expect("put should succeed");
    store
        .append(transcript_entry(
            "session-1",
            1,
            TranscriptEntryKind::Summary,
            "inspection summary",
        ))
        .await
        .expect("append should succeed");

    let inspected = store
        .inspected_snapshot()
        .expect("inspected snapshot should load");

    assert_eq!(
        inspected.snapshot,
        store.snapshot().expect("snapshot should load")
    );
    assert_eq!(
        inspected.audit_report(),
        store
            .snapshot_audit_report()
            .expect("audit report should load")
    );
    assert_eq!(
        inspected.issue_summary(),
        store
            .snapshot_issue_summary()
            .expect("issue summary should load")
    );
    assert!(inspected.is_clean());
}

#[test]
fn store_snapshot_audit_report_matches_clean_snapshot() {
    let snapshot = StoreSnapshot {
        sessions: vec![session_record(
            "session-1",
            "Foundation",
            Some("combined audit"),
        )],
        memories: vec![memory_record(
            "memory-1",
            MemoryScope::LongTerm,
            "snapshot audit payload",
        )],
        transcripts: vec![transcript_entry(
            "session-1",
            1,
            TranscriptEntryKind::Summary,
            "clean transcript summary",
        )],
    };

    let report = snapshot.audit_report();

    assert_eq!(report.memory_stats, snapshot.stats());
    assert_eq!(report.transcript_stats, snapshot.transcript_stats());
    assert_eq!(report.memory_integrity, snapshot.integrity_report());
    assert_eq!(
        report.transcript_integrity,
        snapshot.transcript_integrity_report()
    );
    assert_eq!(report.issue_summary(), snapshot.issue_summary());
    assert_eq!(report.issue_count(), 0);
    assert!(report.is_clean());
}

#[test]
fn store_snapshot_issue_summary_matches_audit_and_inspection_helpers() {
    let snapshot = StoreSnapshot {
        sessions: vec![session_record(
            "session-1",
            " ",
            Some("issue summary alignment"),
        )],
        memories: vec![memory_record("memory-1", MemoryScope::LongTerm, "   ")],
        transcripts: vec![transcript_entry(
            "session-1",
            1,
            TranscriptEntryKind::Summary,
            "   ",
        )],
    };

    let summary = snapshot.issue_summary();

    assert_eq!(summary, snapshot.audit_report().issue_summary());
    assert_eq!(summary, snapshot.inspection_bundle().issue_summary());
    assert_eq!(
        summary,
        SnapshotIssueSummary {
            memory_issue_count: 2,
            transcript_issue_count: 1,
            total_issue_count: 3,
            issue_domain_count: 2,
        }
    );
    assert!(summary.touches_memory());
    assert!(summary.touches_transcripts());
    assert!(summary.has_issues());
    assert!(!summary.is_clean());
}

#[test]
fn store_snapshot_inspection_bundle_matches_snapshot_helpers() {
    let snapshot = StoreSnapshot {
        sessions: vec![session_record(
            "session-1",
            "Foundation",
            Some("inspection bundle"),
        )],
        memories: vec![memory_record(
            "memory-1",
            MemoryScope::LongTerm,
            "snapshot audit payload",
        )],
        transcripts: vec![transcript_entry(
            "session-1",
            1,
            TranscriptEntryKind::Summary,
            "clean transcript summary",
        )],
    };

    let bundle = snapshot.inspection_bundle();

    assert_eq!(bundle.memory_manifest, snapshot.manifest());
    assert_eq!(bundle.memory_integrity, snapshot.integrity_report());
    assert_eq!(bundle.transcript_manifest, snapshot.transcript_manifest());
    assert_eq!(
        bundle.transcript_integrity,
        snapshot.transcript_integrity_report()
    );
    assert_eq!(bundle.issue_count(), 0);
    assert!(bundle.is_clean());
}

#[test]
fn store_snapshot_inspection_match_helper_tracks_alignment() {
    let snapshot = StoreSnapshot {
        sessions: vec![session_record(
            "session-1",
            "Foundation",
            Some("inspection alignment"),
        )],
        memories: vec![memory_record(
            "memory-1",
            MemoryScope::LongTerm,
            "snapshot audit payload",
        )],
        transcripts: vec![transcript_entry(
            "session-1",
            1,
            TranscriptEntryKind::Summary,
            "clean transcript summary",
        )],
    };

    let canonical = snapshot.inspection_bundle();
    let drifted = SnapshotInspectionBundle {
        memory_manifest: MemorySnapshotManifest::default(),
        ..canonical.clone()
    };

    assert!(snapshot.inspection_matches(&canonical));
    assert_eq!(
        snapshot.inspection_matches(&canonical),
        canonical.matches_records_and_entries(
            &snapshot.sessions,
            &snapshot.memories,
            &snapshot.transcripts,
        )
    );
    assert!(!snapshot.inspection_matches(&drifted));
}

#[tokio::test]
async fn snapshot_audit_report_tracks_memory_and_transcript_issues_together() {
    let store = InMemoryStore::default();

    store
        .create(session_record(
            "session-1",
            "Foundation",
            Some("audit combined snapshot"),
        ))
        .await
        .expect("create should succeed");
    store
        .create(session_record("session-1", "   ", None))
        .await
        .expect("create should succeed");
    store
        .put(memory_record(
            "memory-1",
            MemoryScope::LongTerm,
            "manifest payload",
        ))
        .await
        .expect("put should succeed");
    store
        .put(memory_record("memory-1", MemoryScope::Session, "   "))
        .await
        .expect("put should succeed");
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

    let report = store
        .snapshot_audit_report()
        .expect("audit report should load");
    let summary = store
        .snapshot_issue_summary()
        .expect("issue summary should load");
    let inspection = store
        .snapshot_inspection_bundle()
        .expect("inspection bundle should load");

    assert_eq!(report.memory_stats.session_count, 2);
    assert_eq!(report.memory_stats.total_memory_count, 2);
    assert_eq!(report.transcript_stats.total_entry_count, 2);
    assert_eq!(report.memory_integrity.issue_count(), 4);
    assert_eq!(report.transcript_integrity.issue_count(), 2);
    assert_eq!(report.memory_issue_count(), 4);
    assert_eq!(report.transcript_issue_count(), 2);
    assert_eq!(report.issue_count(), 6);
    assert_eq!(report.issue_domain_count(), 2);
    assert!(report.touches_memory());
    assert!(report.touches_transcripts());
    assert!(!report.is_clean());

    assert_eq!(summary, report.issue_summary());
    assert_eq!(summary, inspection.issue_summary());
    assert_eq!(summary.memory_issue_count, 4);
    assert_eq!(summary.transcript_issue_count, 2);
    assert_eq!(summary.total_issue_count, 6);
    assert_eq!(summary.issue_domain_count, 2);
    assert!(summary.has_issues());
    assert!(!summary.is_clean());

    assert_eq!(inspection.memory_manifest.stats, report.memory_stats);
    assert_eq!(inspection.memory_integrity, report.memory_integrity);
    assert_eq!(
        inspection.transcript_manifest.stats,
        report.transcript_stats
    );
    assert_eq!(inspection.transcript_integrity, report.transcript_integrity);
    assert_eq!(inspection.memory_issue_count(), 4);
    assert_eq!(inspection.transcript_issue_count(), 2);
    assert_eq!(inspection.issue_count(), 6);
    assert_eq!(inspection.issue_domain_count(), 2);
    assert!(inspection.touches_memory());
    assert!(inspection.touches_transcripts());
    assert!(!inspection.is_clean());
}
