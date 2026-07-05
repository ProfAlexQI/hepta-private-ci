use super::*;

#[test]
fn transcript_snapshot_integrity_report_detects_duplicates_blank_and_collisions() {
    let report = TranscriptSnapshotIntegrityReport::from_entries(&[
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
        TranscriptEntry {
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
        },
    ]);

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
    assert_eq!(
        report.duplicate_sequence_collisions[0].entry_ids,
        vec!["entry-1".to_string(), "entry-1".to_string()]
    );
    assert_eq!(report.issue_count(), 5);
    assert!(!report.is_clean());
}

#[test]
fn clean_transcript_snapshot_integrity_report_has_no_issues() {
    let report = TranscriptSnapshotIntegrityReport::from_entries(&[
        sample_transcript_entry(1, "approval granted"),
        TranscriptEntry {
            entry_id: "entry-2".into(),
            session_id: SessionId("session-99".into()),
            sequence: 1,
            kind: TranscriptEntryKind::Summary,
            role: Some(MessageRole::Assistant),
            content: "clean summary".into(),
            created_at_unix_ms: 2,
            tool_name: None,
            correlation_id: Some("corr-2".into()),
            summary_of_range: Some(TranscriptRange {
                start_sequence: 1,
                end_sequence: 1,
            }),
        },
    ]);

    assert_eq!(report, TranscriptSnapshotIntegrityReport::default());
    assert_eq!(report.issue_count(), 0);
    assert!(report.is_clean());
}
