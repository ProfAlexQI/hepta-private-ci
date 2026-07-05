use super::*;

#[test]
fn snapshot_audit_report_rolls_up_memory_and_transcript_health() {
    let report = SnapshotAuditReport::from_records_and_entries(
        &[
            SessionRecord {
                session_id: SessionId("session-1".into()),
                agent_id: AgentId("builder".into()),
                title: "Foundation".into(),
                created_at_unix_ms: 1,
                last_active_unix_ms: 2,
                last_user_intent_summary: Some("audit combined snapshot".into()),
                archived_at_unix_ms: None,
            },
            SessionRecord {
                session_id: SessionId("session-1".into()),
                agent_id: AgentId("builder".into()),
                title: "   ".into(),
                created_at_unix_ms: 3,
                last_active_unix_ms: 4,
                last_user_intent_summary: None,
                archived_at_unix_ms: None,
            },
        ],
        &[
            MemoryRecord {
                id: "memory-1".into(),
                scope: MemoryScope::LongTerm,
                content: "manifest payload".into(),
            },
            MemoryRecord {
                id: "memory-1".into(),
                scope: MemoryScope::Session,
                content: "   ".into(),
            },
        ],
        &[
            TranscriptEntry {
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
            },
            TranscriptEntry {
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
            },
        ],
    );

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
    assert!(!report.is_empty());
}

#[test]
fn snapshot_audit_report_roundtrips_through_json() {
    let report = SnapshotAuditReport::from_records_and_entries(
        &[SessionRecord {
            session_id: SessionId("session-1".into()),
            agent_id: AgentId("builder".into()),
            title: "Foundation lane".into(),
            created_at_unix_ms: 1,
            last_active_unix_ms: 2,
            last_user_intent_summary: Some("combined audit".into()),
            archived_at_unix_ms: None,
        }],
        &[MemoryRecord {
            id: "memory-1".into(),
            scope: MemoryScope::LongTerm,
            content: "snapshot contract ready".into(),
        }],
        &[sample_transcript_entry(1, "snapshot captured")],
    );

    let json = serde_json::to_string(&report).expect("audit report should serialize");
    let parsed: SnapshotAuditReport =
        serde_json::from_str(&json).expect("audit report should deserialize");

    assert_eq!(parsed, report);
    assert!(parsed.is_clean());
}

#[test]
fn snapshot_audit_report_deserializes_from_sparse_json() {
    let parsed: SnapshotAuditReport =
        serde_json::from_str("{}").expect("sparse audit report should deserialize with defaults");

    assert_eq!(parsed, SnapshotAuditReport::default());
    assert!(parsed.is_empty());
    assert!(parsed.is_clean());
}

#[test]
fn snapshot_issue_summary_compacts_audit_issue_counts() {
    let report = SnapshotAuditReport::from_records_and_entries(
        &[SessionRecord {
            session_id: SessionId("session-1".into()),
            agent_id: AgentId("builder".into()),
            title: " ".into(),
            created_at_unix_ms: 1,
            last_active_unix_ms: 2,
            last_user_intent_summary: Some("compact audit issue summary".into()),
            archived_at_unix_ms: None,
        }],
        &[MemoryRecord {
            id: "memory-1".into(),
            scope: MemoryScope::LongTerm,
            content: " ".into(),
        }],
        &[TranscriptEntry {
            entry_id: "entry-1".into(),
            session_id: SessionId("session-1".into()),
            sequence: 1,
            kind: TranscriptEntryKind::Message,
            role: Some(MessageRole::Assistant),
            content: " ".into(),
            created_at_unix_ms: 3,
            tool_name: None,
            correlation_id: None,
            summary_of_range: None,
        }],
    );

    let summary = report.issue_summary();

    assert_eq!(summary.memory_issue_count, 2);
    assert_eq!(summary.transcript_issue_count, 1);
    assert_eq!(summary.total_issue_count, 3);
    assert_eq!(summary.issue_domain_count, 2);
    assert!(summary.touches_memory());
    assert!(summary.touches_transcripts());
    assert!(summary.has_issues());
    assert!(!summary.is_clean());
}

#[test]
fn snapshot_issue_summary_matches_bundle_and_audit_views() {
    let sessions = vec![SessionRecord {
        session_id: SessionId("session-1".into()),
        agent_id: AgentId("builder".into()),
        title: "Foundation lane".into(),
        created_at_unix_ms: 1,
        last_active_unix_ms: 2,
        last_user_intent_summary: Some("issue summary alignment".into()),
        archived_at_unix_ms: None,
    }];
    let memories = vec![MemoryRecord {
        id: "memory-1".into(),
        scope: MemoryScope::Session,
        content: "payload".into(),
    }];
    let transcripts = vec![sample_transcript_entry(1, "snapshot captured")];

    let report = SnapshotAuditReport::from_records_and_entries(&sessions, &memories, &transcripts);
    let bundle =
        SnapshotInspectionBundle::from_records_and_entries(&sessions, &memories, &transcripts);

    assert_eq!(
        report.issue_summary(),
        SnapshotIssueSummary::from_audit_report(&report)
    );
    assert_eq!(
        bundle.issue_summary(),
        SnapshotIssueSummary::from_inspection(&bundle)
    );
    assert_eq!(report.issue_summary(), bundle.issue_summary());
    assert!(report.issue_summary().is_clean());
}

#[test]
fn snapshot_issue_summary_roundtrips_through_json() {
    let summary = SnapshotIssueSummary {
        memory_issue_count: 2,
        transcript_issue_count: 1,
        total_issue_count: 3,
        issue_domain_count: 2,
    };

    let json = serde_json::to_string(&summary).expect("issue summary should serialize");
    let parsed: SnapshotIssueSummary =
        serde_json::from_str(&json).expect("issue summary should deserialize");

    assert_eq!(parsed, summary);
    assert!(parsed.has_issues());
    assert!(!parsed.is_clean());
}

#[test]
fn snapshot_issue_summary_deserializes_from_sparse_json() {
    let parsed: SnapshotIssueSummary =
        serde_json::from_str("{}").expect("sparse issue summary should deserialize with defaults");

    assert_eq!(parsed, SnapshotIssueSummary::default());
    assert!(!parsed.has_issues());
    assert!(parsed.is_clean());
}
