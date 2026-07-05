use super::*;

#[test]
fn memory_snapshot_stats_roll_up_sessions_and_memories() {
    let sessions = vec![
        SessionRecord {
            session_id: SessionId("session-1".into()),
            agent_id: AgentId("builder".into()),
            title: "Active foundation lane".into(),
            created_at_unix_ms: 1,
            last_active_unix_ms: 2,
            last_user_intent_summary: Some("stabilize memory contracts".into()),
            archived_at_unix_ms: None,
        },
        SessionRecord {
            session_id: SessionId("session-2".into()),
            agent_id: AgentId("builder".into()),
            title: "Archived foundation lane".into(),
            created_at_unix_ms: 3,
            last_active_unix_ms: 4,
            last_user_intent_summary: None,
            archived_at_unix_ms: Some(5),
        },
    ];
    let memories = vec![
        MemoryRecord {
            id: "memory-1".into(),
            scope: MemoryScope::Session,
            content: "doctor snapshot ready".into(),
        },
        MemoryRecord {
            id: "memory-2".into(),
            scope: MemoryScope::LongTerm,
            content: "memory contract exported".into(),
        },
    ];

    let stats = MemorySnapshotStats::from_records(&sessions, &memories);

    assert_eq!(stats.session_count, 2);
    assert_eq!(stats.active_session_count, 1);
    assert_eq!(stats.archived_session_count, 1);
    assert_eq!(stats.total_memory_count, 2);
    assert_eq!(stats.session_memory_count, 1);
    assert_eq!(stats.long_term_memory_count, 1);
    assert!(!stats.is_empty());
}

#[test]
fn empty_memory_snapshot_stats_report_empty_state() {
    let stats = MemorySnapshotStats::from_records(&[], &[]);

    assert_eq!(stats, MemorySnapshotStats::default());
    assert!(stats.is_empty());
}
