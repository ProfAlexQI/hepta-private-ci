use super::*;

#[tokio::test]
async fn snapshot_restore_roundtrip_recovers_sessions_and_memories() {
    let source = InMemoryStore::default();
    let session = session_record("session-1", "Foundation", Some("capture snapshot"));
    let memory = memory_record(
        "memory-1",
        MemoryScope::Session,
        "snapshot integrity contract",
    );
    let transcript = transcript_entry(
        "session-1",
        1,
        TranscriptEntryKind::Summary,
        "snapshot integrity contract",
    );

    source
        .upsert_session_sync(session.clone())
        .expect("upsert should succeed");
    source
        .put(memory.clone())
        .await
        .expect("memory put should succeed");
    source
        .append(transcript.clone())
        .await
        .expect("transcript append should succeed");

    let snapshot = source.snapshot().expect("snapshot should succeed");
    let restored = InMemoryStore::default();
    restored
        .restore(snapshot.clone())
        .expect("restore should succeed");

    assert_eq!(snapshot.sessions, vec![session]);
    assert_eq!(snapshot.memories, vec![memory]);
    assert_eq!(snapshot.transcripts, vec![transcript]);
    assert_eq!(
        restored.snapshot().expect("snapshot should succeed"),
        snapshot
    );
}
