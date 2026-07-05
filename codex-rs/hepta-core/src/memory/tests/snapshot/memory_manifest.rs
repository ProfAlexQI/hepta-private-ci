use super::*;

#[test]
fn memory_snapshot_manifest_sorts_records_and_tracks_content_sizes() {
    let manifest = MemorySnapshotManifest::from_records(
        &[
            SessionRecord {
                session_id: SessionId("session-b".into()),
                agent_id: AgentId("builder".into()),
                title: "Later session".into(),
                created_at_unix_ms: 1,
                last_active_unix_ms: 2,
                last_user_intent_summary: None,
                archived_at_unix_ms: Some(5),
            },
            SessionRecord {
                session_id: SessionId("session-a".into()),
                agent_id: AgentId("builder".into()),
                title: "Earlier session".into(),
                created_at_unix_ms: 3,
                last_active_unix_ms: 4,
                last_user_intent_summary: Some("audit manifest".into()),
                archived_at_unix_ms: None,
            },
        ],
        &[
            MemoryRecord {
                id: "memory-z".into(),
                scope: MemoryScope::LongTerm,
                content: "doctor export manifest".into(),
            },
            MemoryRecord {
                id: "memory-a".into(),
                scope: MemoryScope::Session,
                content: "snapshot ok".into(),
            },
        ],
    );

    assert_eq!(manifest.stats.session_count, 2);
    assert_eq!(manifest.stats.archived_session_count, 1);
    assert_eq!(manifest.sessions.len(), 2);
    assert_eq!(manifest.sessions[0].session_id.0, "session-a");
    assert!(!manifest.sessions[0].archived);
    assert_eq!(manifest.sessions[1].session_id.0, "session-b");
    assert!(manifest.sessions[1].archived);
    assert_eq!(manifest.memories.len(), 2);
    assert_eq!(manifest.memories[0].id, "memory-a");
    assert_eq!(manifest.memories[0].content_bytes, "snapshot ok".len());
    assert_eq!(manifest.memories[1].id, "memory-z");
    assert_eq!(
        manifest.memories[1].content_bytes,
        "doctor export manifest".len()
    );
    assert!(!manifest.is_empty());
}

#[test]
fn memory_snapshot_manifest_roundtrips_through_json() {
    let manifest = MemorySnapshotManifest::from_records(
        &[SessionRecord {
            session_id: SessionId("session-1".into()),
            agent_id: AgentId("builder".into()),
            title: "Foundation lane".into(),
            created_at_unix_ms: 1,
            last_active_unix_ms: 2,
            last_user_intent_summary: Some("inspect snapshot manifest".into()),
            archived_at_unix_ms: None,
        }],
        &[MemoryRecord {
            id: "memory-1".into(),
            scope: MemoryScope::LongTerm,
            content: "export manifest ready".into(),
        }],
    );

    let json = serde_json::to_string(&manifest).expect("manifest should serialize");
    let parsed: MemorySnapshotManifest =
        serde_json::from_str(&json).expect("manifest should deserialize");

    assert_eq!(parsed, manifest);
}

#[test]
fn memory_snapshot_manifest_deserializes_from_sparse_json() {
    let parsed: MemorySnapshotManifest =
        serde_json::from_str("{}").expect("sparse manifest should deserialize with defaults");

    assert_eq!(parsed, MemorySnapshotManifest::default());
    assert!(parsed.is_empty());
}
