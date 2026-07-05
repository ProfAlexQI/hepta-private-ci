use super::*;

#[test]
fn transcript_snapshot_stats_roll_up_entry_kinds_and_sessions() {
    let entries = vec![
        sample_transcript_entry(1, "user message"),
        TranscriptEntry {
            entry_id: "entry-2".into(),
            session_id: SessionId("session-42".into()),
            sequence: 2,
            kind: TranscriptEntryKind::ToolCall,
            role: Some(MessageRole::Assistant),
            content: "call write".into(),
            created_at_unix_ms: 102,
            tool_name: Some("write".into()),
            correlation_id: Some("corr-1".into()),
            summary_of_range: None,
        },
        TranscriptEntry {
            entry_id: "entry-3".into(),
            session_id: SessionId("session-42".into()),
            sequence: 3,
            kind: TranscriptEntryKind::ToolResult,
            role: Some(MessageRole::Tool),
            content: "write ok".into(),
            created_at_unix_ms: 103,
            tool_name: Some("write".into()),
            correlation_id: Some("corr-1".into()),
            summary_of_range: None,
        },
        TranscriptEntry {
            entry_id: "entry-4".into(),
            session_id: SessionId("session-42".into()),
            sequence: 4,
            kind: TranscriptEntryKind::Approval,
            role: Some(MessageRole::Assistant),
            content: "approval granted".into(),
            created_at_unix_ms: 104,
            tool_name: None,
            correlation_id: Some("corr-2".into()),
            summary_of_range: None,
        },
        TranscriptEntry {
            entry_id: "entry-5".into(),
            session_id: SessionId("session-77".into()),
            sequence: 1,
            kind: TranscriptEntryKind::Summary,
            role: Some(MessageRole::Assistant),
            content: "session summary".into(),
            created_at_unix_ms: 105,
            tool_name: None,
            correlation_id: Some("corr-3".into()),
            summary_of_range: Some(TranscriptRange {
                start_sequence: 1,
                end_sequence: 4,
            }),
        },
        TranscriptEntry {
            entry_id: "entry-6".into(),
            session_id: SessionId("session-77".into()),
            sequence: 2,
            kind: TranscriptEntryKind::Event,
            role: None,
            content: "session archived".into(),
            created_at_unix_ms: 106,
            tool_name: None,
            correlation_id: None,
            summary_of_range: None,
        },
    ];

    let stats = TranscriptSnapshotStats::from_entries(&entries);

    assert_eq!(stats.total_entry_count, 6);
    assert_eq!(stats.session_count, 2);
    assert_eq!(stats.message_count, 1);
    assert_eq!(stats.tool_call_count, 1);
    assert_eq!(stats.tool_result_count, 1);
    assert_eq!(stats.approval_count, 1);
    assert_eq!(stats.summary_count, 1);
    assert_eq!(stats.event_count, 1);
    assert!(!stats.is_empty());
}
