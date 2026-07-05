use super::*;

#[tokio::test]
async fn snapshot_manifest_tracks_sorted_records_and_sizes() {
    let store = InMemoryStore::default();
    store
        .upsert_session_sync(SessionRecord {
            archived_at_unix_ms: Some(30),
            ..session_record("session-b", "Archived foundation", None)
        })
        .expect("upsert should succeed");
    store
        .upsert_session_sync(session_record(
            "session-a",
            "Active foundation",
            Some("capture manifest"),
        ))
        .expect("upsert should succeed");
    store
        .put(memory_record(
            "memory-z",
            MemoryScope::LongTerm,
            "manifest export payload",
        ))
        .await
        .expect("put should succeed");
    store
        .put(memory_record(
            "memory-a",
            MemoryScope::Session,
            "session payload",
        ))
        .await
        .expect("put should succeed");

    let manifest = store
        .snapshot_manifest()
        .expect("manifest should be available");

    assert_eq!(manifest.stats.session_count, 2);
    assert_eq!(manifest.stats.archived_session_count, 1);
    assert_eq!(manifest.sessions.len(), 2);
    assert_eq!(manifest.sessions[0].session_id.0, "session-a");
    assert_eq!(manifest.sessions[1].session_id.0, "session-b");
    assert_eq!(manifest.memories.len(), 2);
    assert_eq!(manifest.memories[0].id, "memory-a");
    assert_eq!(manifest.memories[0].content_bytes, "session payload".len());
    assert_eq!(manifest.memories[1].id, "memory-z");
    assert_eq!(
        manifest.memories[1].content_bytes,
        "manifest export payload".len()
    );
}

#[tokio::test]
async fn transcript_snapshot_manifest_tracks_sorted_entries_and_sizes() {
    let store = InMemoryStore::default();
    store
        .append(transcript_entry(
            "session-z",
            3,
            TranscriptEntryKind::ToolResult,
            "tool result payload",
        ))
        .await
        .expect("append should succeed");
    store
        .append(transcript_entry(
            "session-a",
            2,
            TranscriptEntryKind::Summary,
            "summary payload",
        ))
        .await
        .expect("append should succeed");
    store
        .append(transcript_entry(
            "session-a",
            1,
            TranscriptEntryKind::Message,
            "message payload",
        ))
        .await
        .expect("append should succeed");

    let manifest = store
        .transcript_snapshot_manifest()
        .expect("transcript manifest should be available");

    assert_eq!(manifest.stats.total_entry_count, 3);
    assert_eq!(manifest.stats.session_count, 2);
    assert_eq!(manifest.entries.len(), 3);
    assert_eq!(manifest.entries[0].session_id.0, "session-a");
    assert_eq!(manifest.entries[0].sequence, 1);
    assert_eq!(manifest.entries[0].content_bytes, "message payload".len());
    assert_eq!(manifest.entries[1].sequence, 2);
    assert_eq!(manifest.entries[2].session_id.0, "session-z");
}

#[test]
fn store_snapshot_manifest_matches_snapshot_stats() {
    let snapshot = StoreSnapshot {
        sessions: vec![SessionRecord {
            archived_at_unix_ms: Some(30),
            ..session_record("session-1", "Foundation", Some("manifest alignment"))
        }],
        memories: vec![memory_record(
            "memory-1",
            MemoryScope::LongTerm,
            "manifest alignment payload",
        )],
        transcripts: vec![],
    };

    let manifest = snapshot.manifest();

    assert_eq!(manifest.stats, snapshot.stats());
    assert_eq!(manifest.sessions[0].title, "Foundation");
    assert_eq!(manifest.memories[0].id, "memory-1");
    assert_eq!(
        manifest.memories[0].content_bytes,
        "manifest alignment payload".len()
    );
}
