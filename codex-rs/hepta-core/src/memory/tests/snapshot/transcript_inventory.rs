use super::*;

#[test]
fn transcript_session_inventory_rolls_up_ranges_and_entry_kinds_per_session() {
    let inventory = TranscriptSessionInventory::from_entries(&[
        TranscriptEntry {
            entry_id: "entry-b".into(),
            session_id: SessionId("session-z".into()),
            sequence: 8,
            kind: TranscriptEntryKind::ToolResult,
            role: Some(MessageRole::Tool),
            content: "tool result payload".into(),
            created_at_unix_ms: 200,
            tool_name: Some("write".into()),
            correlation_id: None,
            summary_of_range: None,
        },
        TranscriptEntry {
            entry_id: "entry-a".into(),
            session_id: SessionId("session-a".into()),
            sequence: 3,
            kind: TranscriptEntryKind::Message,
            role: Some(MessageRole::User),
            content: "hello".into(),
            created_at_unix_ms: 100,
            tool_name: None,
            correlation_id: None,
            summary_of_range: None,
        },
        TranscriptEntry {
            entry_id: "entry-c".into(),
            session_id: SessionId("session-a".into()),
            sequence: 9,
            kind: TranscriptEntryKind::Summary,
            role: Some(MessageRole::Assistant),
            content: "summary".into(),
            created_at_unix_ms: 300,
            tool_name: None,
            correlation_id: None,
            summary_of_range: Some(TranscriptRange {
                start_sequence: 1,
                end_sequence: 8,
            }),
        },
        TranscriptEntry {
            entry_id: "entry-d".into(),
            session_id: SessionId("   ".into()),
            sequence: 1,
            kind: TranscriptEntryKind::Event,
            role: None,
            content: "missing session".into(),
            created_at_unix_ms: 400,
            tool_name: None,
            correlation_id: None,
            summary_of_range: None,
        },
    ]);

    assert_eq!(inventory.total_entry_count, 4);
    assert_eq!(inventory.blank_session_id_entry_count, 1);
    assert_eq!(inventory.session_count(), 2);
    assert_eq!(inventory.inventoried_entry_count(), 3);
    assert!(!inventory.is_empty());
    assert_eq!(inventory.sessions[0].session_id.0, "session-a");
    assert_eq!(inventory.sessions[0].entry_count, 2);
    assert_eq!(inventory.sessions[0].first_sequence, 3);
    assert_eq!(inventory.sessions[0].last_sequence, 9);
    assert_eq!(inventory.sessions[0].message_count, 1);
    assert_eq!(inventory.sessions[0].summary_count, 1);
    assert_eq!(inventory.sessions[1].session_id.0, "session-z");
    assert_eq!(inventory.sessions[1].entry_count, 1);
    assert_eq!(inventory.sessions[1].tool_result_count, 1);
}

#[test]
fn transcript_session_inventory_roundtrips_through_json() {
    let inventory = TranscriptSessionInventory::from_entries(&[
        sample_transcript_entry(1, "first"),
        sample_transcript_entry(2, "second"),
    ]);

    let json = serde_json::to_string(&inventory).expect("inventory should serialize");
    let parsed: TranscriptSessionInventory =
        serde_json::from_str(&json).expect("inventory should deserialize");

    assert_eq!(parsed, inventory);
}

#[test]
fn transcript_session_inventory_deserializes_from_sparse_json() {
    let parsed: TranscriptSessionInventory =
        serde_json::from_str("{}").expect("sparse inventory should deserialize with defaults");

    assert_eq!(parsed, TranscriptSessionInventory::default());
    assert_eq!(parsed.session_count(), 0);
    assert_eq!(parsed.inventoried_entry_count(), 0);
    assert!(parsed.is_empty());
}
