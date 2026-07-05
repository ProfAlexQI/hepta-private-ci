use super::*;

#[test]
fn snapshot_inspection_bundle_keeps_manifests_and_integrity_reports_aligned() {
    let bundle = SnapshotInspectionBundle::from_records_and_entries(
        &[SessionRecord {
            session_id: SessionId("session-1".into()),
            agent_id: AgentId("builder".into()),
            title: "Foundation lane".into(),
            created_at_unix_ms: 1,
            last_active_unix_ms: 2,
            last_user_intent_summary: Some("inspect snapshot".into()),
            archived_at_unix_ms: None,
        }],
        &[MemoryRecord {
            id: "memory-1".into(),
            scope: MemoryScope::Session,
            content: "manifest payload".into(),
        }],
        &[sample_transcript_entry(1, "snapshot captured")],
    );

    assert_eq!(bundle.memory_manifest.stats.session_count, 1);
    assert_eq!(bundle.memory_manifest.stats.total_memory_count, 1);
    assert_eq!(bundle.transcript_manifest.stats.total_entry_count, 1);
    assert_eq!(
        bundle.memory_integrity,
        MemorySnapshotIntegrityReport::default()
    );
    assert_eq!(
        bundle.transcript_integrity,
        TranscriptSnapshotIntegrityReport::default()
    );
    assert_eq!(bundle.memory_issue_count(), 0);
    assert_eq!(bundle.transcript_issue_count(), 0);
    assert_eq!(bundle.issue_count(), 0);
    assert_eq!(bundle.issue_domain_count(), 0);
    assert_eq!(bundle.issue_summary(), SnapshotIssueSummary::default());
    assert!(!bundle.touches_memory());
    assert!(!bundle.touches_transcripts());
    assert!(bundle.is_clean());
    assert!(!bundle.is_empty());
}

#[test]
fn snapshot_inspection_bundle_roundtrips_through_json() {
    let bundle = SnapshotInspectionBundle::from_records_and_entries(
        &[SessionRecord {
            session_id: SessionId("session-1".into()),
            agent_id: AgentId("builder".into()),
            title: "Foundation lane".into(),
            created_at_unix_ms: 1,
            last_active_unix_ms: 2,
            last_user_intent_summary: Some("inspect snapshot".into()),
            archived_at_unix_ms: Some(3),
        }],
        &[MemoryRecord {
            id: "memory-1".into(),
            scope: MemoryScope::LongTerm,
            content: "snapshot contract ready".into(),
        }],
        &[sample_transcript_entry(1, "snapshot captured")],
    );

    let json = serde_json::to_string(&bundle).expect("inspection bundle should serialize");
    let parsed: SnapshotInspectionBundle =
        serde_json::from_str(&json).expect("inspection bundle should deserialize");

    assert_eq!(parsed, bundle);
    assert!(parsed.is_clean());
}

#[test]
fn snapshot_inspection_bundle_deserializes_from_sparse_json() {
    let parsed: SnapshotInspectionBundle = serde_json::from_str("{}")
        .expect("sparse inspection bundle should deserialize with defaults");

    assert_eq!(parsed, SnapshotInspectionBundle::default());
    assert!(parsed.is_empty());
    assert!(parsed.is_clean());
}

#[test]
fn snapshot_inspection_bundle_reconstructs_audit_report() {
    let sessions = vec![SessionRecord {
        session_id: SessionId("session-1".into()),
        agent_id: AgentId("builder".into()),
        title: "Foundation lane".into(),
        created_at_unix_ms: 1,
        last_active_unix_ms: 2,
        last_user_intent_summary: Some("reconstruct audit report".into()),
        archived_at_unix_ms: Some(3),
    }];
    let memories = vec![MemoryRecord {
        id: "memory-1".into(),
        scope: MemoryScope::LongTerm,
        content: "snapshot contract ready".into(),
    }];
    let transcripts = vec![sample_transcript_entry(1, "snapshot captured")];

    let bundle =
        SnapshotInspectionBundle::from_records_and_entries(&sessions, &memories, &transcripts);
    let report = bundle.audit_report();

    assert_eq!(
        report,
        SnapshotAuditReport::from_records_and_entries(&sessions, &memories, &transcripts)
    );
    assert_eq!(bundle.memory_issue_count(), report.memory_issue_count());
    assert_eq!(
        bundle.transcript_issue_count(),
        report.transcript_issue_count()
    );
    assert_eq!(bundle.issue_summary(), report.issue_summary());
    assert_eq!(bundle.issue_domain_count(), report.issue_domain_count());
    assert_eq!(bundle.touches_memory(), report.touches_memory());
    assert_eq!(bundle.touches_transcripts(), report.touches_transcripts());
    assert!(report.is_clean());
}

#[test]
fn snapshot_inspection_bundle_matches_records_and_entries_only_when_aligned() {
    let sessions = vec![SessionRecord {
        session_id: SessionId("session-1".into()),
        agent_id: AgentId("builder".into()),
        title: "Foundation lane".into(),
        created_at_unix_ms: 1,
        last_active_unix_ms: 2,
        last_user_intent_summary: Some("match inspection bundle".into()),
        archived_at_unix_ms: None,
    }];
    let memories = vec![MemoryRecord {
        id: "memory-1".into(),
        scope: MemoryScope::Session,
        content: "inspection payload".into(),
    }];
    let transcripts = vec![sample_transcript_entry(1, "snapshot captured")];
    let bundle =
        SnapshotInspectionBundle::from_records_and_entries(&sessions, &memories, &transcripts);

    assert!(bundle.matches_records_and_entries(&sessions, &memories, &transcripts));

    let drifted_transcripts = vec![sample_transcript_entry(2, "snapshot changed")];

    assert!(!bundle.matches_records_and_entries(&sessions, &memories, &drifted_transcripts,));
}
