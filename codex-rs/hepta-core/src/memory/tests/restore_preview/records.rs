use super::*;

#[test]
fn snapshot_restore_preview_classifies_added_removed_updated_and_unchanged_records() {
    let current_sessions = vec![
        SessionRecord {
            session_id: SessionId("session-1".into()),
            agent_id: AgentId("builder".into()),
            title: "Current title".into(),
            created_at_unix_ms: 1,
            last_active_unix_ms: 2,
            last_user_intent_summary: Some("current".into()),
            archived_at_unix_ms: None,
        },
        SessionRecord {
            session_id: SessionId("session-2".into()),
            agent_id: AgentId("builder".into()),
            title: "Unchanged session".into(),
            created_at_unix_ms: 3,
            last_active_unix_ms: 4,
            last_user_intent_summary: None,
            archived_at_unix_ms: None,
        },
    ];
    let incoming_sessions = vec![
        SessionRecord {
            session_id: SessionId("session-1".into()),
            agent_id: AgentId("builder".into()),
            title: "Updated title".into(),
            created_at_unix_ms: 1,
            last_active_unix_ms: 20,
            last_user_intent_summary: Some("incoming".into()),
            archived_at_unix_ms: None,
        },
        SessionRecord {
            session_id: SessionId("session-2".into()),
            agent_id: AgentId("builder".into()),
            title: "Unchanged session".into(),
            created_at_unix_ms: 3,
            last_active_unix_ms: 4,
            last_user_intent_summary: None,
            archived_at_unix_ms: None,
        },
        SessionRecord {
            session_id: SessionId("session-3".into()),
            agent_id: AgentId("builder".into()),
            title: "Added session".into(),
            created_at_unix_ms: 5,
            last_active_unix_ms: 6,
            last_user_intent_summary: Some("added".into()),
            archived_at_unix_ms: Some(7),
        },
    ];
    let current_memories = vec![
        MemoryRecord {
            id: "memory-1".into(),
            scope: MemoryScope::Session,
            content: "unchanged".into(),
        },
        MemoryRecord {
            id: "memory-2".into(),
            scope: MemoryScope::LongTerm,
            content: "removed".into(),
        },
        MemoryRecord {
            id: "memory-3".into(),
            scope: MemoryScope::LongTerm,
            content: "before update".into(),
        },
    ];
    let incoming_memories = vec![
        MemoryRecord {
            id: "memory-1".into(),
            scope: MemoryScope::Session,
            content: "unchanged".into(),
        },
        MemoryRecord {
            id: "memory-3".into(),
            scope: MemoryScope::LongTerm,
            content: "after update".into(),
        },
        MemoryRecord {
            id: "memory-4".into(),
            scope: MemoryScope::Session,
            content: "added".into(),
        },
    ];
    let current_transcripts = vec![
        sample_transcript_entry(1, "unchanged transcript"),
        sample_transcript_entry(2, "removed transcript"),
        sample_transcript_entry(3, "before update"),
    ];
    let incoming_transcripts = vec![
        sample_transcript_entry(1, "unchanged transcript"),
        TranscriptEntry {
            content: "after update".into(),
            ..sample_transcript_entry(3, "before update")
        },
        sample_transcript_entry(4, "added transcript"),
    ];

    let preview = SnapshotRestorePreview::from_records_and_entries(
        &current_sessions,
        &current_memories,
        &current_transcripts,
        &incoming_sessions,
        &incoming_memories,
        &incoming_transcripts,
    );

    assert_eq!(
        preview.session_delta.added_session_ids,
        vec![SessionId("session-3".into())]
    );
    assert!(preview.session_delta.removed_session_ids.is_empty());
    assert_eq!(
        preview.session_delta.updated_session_ids,
        vec![SessionId("session-1".into())]
    );
    assert_eq!(preview.session_delta.unchanged_count, 1);

    assert_eq!(
        preview.memory_delta.added_memory_ids,
        vec!["memory-4".to_string()]
    );
    assert_eq!(
        preview.memory_delta.removed_memory_ids,
        vec!["memory-2".to_string()]
    );
    assert_eq!(
        preview.memory_delta.updated_memory_ids,
        vec!["memory-3".to_string()]
    );
    assert_eq!(preview.memory_delta.unchanged_count, 1);

    assert_eq!(
        preview.transcript_delta.added_entry_ids,
        vec!["entry-4".to_string()]
    );
    assert_eq!(
        preview.transcript_delta.removed_entry_ids,
        vec!["entry-2".to_string()]
    );
    assert_eq!(
        preview.transcript_delta.updated_entry_ids,
        vec!["entry-3".to_string()]
    );
    assert_eq!(preview.transcript_delta.unchanged_count, 1);

    assert_eq!(preview.change_count(), 8);
    assert!(!preview.is_noop());
    assert!(!preview.has_integrity_issues());
}

#[test]
fn snapshot_restore_preview_detects_noop_restore() {
    let sessions = vec![SessionRecord {
        session_id: SessionId("session-1".into()),
        agent_id: AgentId("builder".into()),
        title: "Foundation lane".into(),
        created_at_unix_ms: 1,
        last_active_unix_ms: 2,
        last_user_intent_summary: Some("noop restore".into()),
        archived_at_unix_ms: None,
    }];
    let memories = vec![MemoryRecord {
        id: "memory-1".into(),
        scope: MemoryScope::LongTerm,
        content: "same payload".into(),
    }];
    let transcripts = vec![sample_transcript_entry(1, "same transcript")];

    let preview = SnapshotRestorePreview::from_records_and_entries(
        &sessions,
        &memories,
        &transcripts,
        &sessions,
        &memories,
        &transcripts,
    );

    assert_eq!(preview.change_count(), 0);
    assert_eq!(
        preview.change_totals(),
        RestoreDeltaCounts {
            added_count: 0,
            removed_count: 0,
            updated_count: 0,
            unchanged_count: 3,
        }
    );
    assert!(preview.is_noop());
    assert!(!preview.has_integrity_issues());
}
