use super::*;

#[test]
fn memory_snapshot_integrity_report_detects_duplicate_and_blank_fields() {
    let report = MemorySnapshotIntegrityReport::from_records(
        &[
            SessionRecord {
                session_id: SessionId("session-1".into()),
                agent_id: AgentId("builder".into()),
                title: "Foundation".into(),
                created_at_unix_ms: 1,
                last_active_unix_ms: 2,
                last_user_intent_summary: Some("audit snapshot".into()),
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
            SessionRecord {
                session_id: SessionId("   ".into()),
                agent_id: AgentId("builder".into()),
                title: "Needs title".into(),
                created_at_unix_ms: 5,
                last_active_unix_ms: 6,
                last_user_intent_summary: None,
                archived_at_unix_ms: None,
            },
        ],
        &[
            MemoryRecord {
                id: "memory-1".into(),
                scope: MemoryScope::Session,
                content: "contract ready".into(),
            },
            MemoryRecord {
                id: "memory-1".into(),
                scope: MemoryScope::LongTerm,
                content: "   ".into(),
            },
            MemoryRecord {
                id: " ".into(),
                scope: MemoryScope::LongTerm,
                content: "manifest payload".into(),
            },
        ],
    );

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

#[test]
fn clean_memory_snapshot_integrity_report_has_no_issues() {
    let report = MemorySnapshotIntegrityReport::from_records(
        &[SessionRecord {
            session_id: SessionId("session-1".into()),
            agent_id: AgentId("builder".into()),
            title: "Foundation lane".into(),
            created_at_unix_ms: 1,
            last_active_unix_ms: 2,
            last_user_intent_summary: Some("stabilize contracts".into()),
            archived_at_unix_ms: None,
        }],
        &[MemoryRecord {
            id: "memory-1".into(),
            scope: MemoryScope::LongTerm,
            content: "snapshot contract ready".into(),
        }],
    );

    assert_eq!(report, MemorySnapshotIntegrityReport::default());
    assert_eq!(report.issue_count(), 0);
    assert!(report.is_clean());
}

#[test]
fn memory_snapshot_integrity_report_roundtrips_through_json() {
    let report = MemorySnapshotIntegrityReport {
        duplicate_session_ids: vec![SessionId("session-1".into())],
        duplicate_memory_ids: vec!["memory-1".into()],
        blank_session_id_count: 1,
        blank_memory_id_count: 0,
        blank_session_title_count: 2,
        blank_memory_content_count: 3,
    };

    let json = serde_json::to_string(&report).expect("integrity report should serialize");
    let parsed: MemorySnapshotIntegrityReport =
        serde_json::from_str(&json).expect("integrity report should deserialize");

    assert_eq!(parsed, report);
}
