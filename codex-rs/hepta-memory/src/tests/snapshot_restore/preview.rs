use super::*;

#[test]
fn store_snapshot_restore_preview_matches_core_report() {
    let current = StoreSnapshot {
        sessions: vec![session_record(
            "session-1",
            "Current title",
            Some("current"),
        )],
        memories: vec![memory_record(
            "memory-1",
            MemoryScope::LongTerm,
            "current payload",
        )],
        transcripts: vec![transcript_entry(
            "session-1",
            1,
            TranscriptEntryKind::Summary,
            "current summary",
        )],
    };
    let incoming = StoreSnapshot {
        sessions: vec![session_record(
            "session-1",
            "Updated title",
            Some("incoming"),
        )],
        memories: vec![memory_record(
            "memory-1",
            MemoryScope::LongTerm,
            "incoming payload",
        )],
        transcripts: vec![transcript_entry(
            "session-1",
            1,
            TranscriptEntryKind::Summary,
            "incoming summary",
        )],
    };

    let preview = incoming.restore_preview_against(&current);

    assert_eq!(
        preview,
        SnapshotRestorePreview::from_records_and_entries(
            &current.sessions,
            &current.memories,
            &current.transcripts,
            &incoming.sessions,
            &incoming.memories,
            &incoming.transcripts,
        )
    );
    assert_eq!(
        preview.session_delta.updated_session_ids,
        vec![SessionId("session-1".into())]
    );
    assert_eq!(
        preview.memory_delta.updated_memory_ids,
        vec!["memory-1".to_string()]
    );
    assert_eq!(
        preview.transcript_delta.updated_entry_ids,
        vec!["session-1-1".to_string()]
    );
    assert_eq!(
        incoming.restore_changed_domains_against(&current),
        vec![
            SnapshotRestoreDomain::Sessions,
            SnapshotRestoreDomain::Memories,
            SnapshotRestoreDomain::Transcripts,
        ]
    );
    assert_eq!(preview.changed_domain_count(), 3);
}

#[test]
fn store_snapshot_restore_helpers_flag_additive_only_previews() {
    let current = StoreSnapshot {
        sessions: vec![],
        memories: vec![],
        transcripts: vec![],
    };
    let incoming = StoreSnapshot {
        sessions: vec![session_record(
            "session-1",
            "Added foundation",
            Some("additive restore"),
        )],
        memories: vec![memory_record(
            "memory-1",
            MemoryScope::LongTerm,
            "added payload",
        )],
        transcripts: vec![transcript_entry(
            "session-1",
            1,
            TranscriptEntryKind::Summary,
            "added summary",
        )],
    };
    let inspected = InspectedStoreSnapshot::from_snapshot(incoming.clone());

    assert!(incoming.restore_is_additive_only_against(&current));
    assert!(!incoming.restore_touches_existing_records_against(&current));
    assert!(inspected.restore_is_additive_only_against(&current));
    assert!(!inspected.restore_touches_existing_records_against(&current));
}

#[tokio::test]
async fn store_preview_restore_helpers_detect_existing_record_changes() {
    let store = InMemoryStore::default();
    store
        .upsert_session_sync(session_record(
            "session-1",
            "Current foundation",
            Some("existing session"),
        ))
        .expect("upsert should succeed");
    store
        .put(memory_record(
            "memory-1",
            MemoryScope::Session,
            "current payload",
        ))
        .await
        .expect("put should succeed");

    let incoming = StoreSnapshot {
        sessions: vec![session_record(
            "session-1",
            "Updated foundation",
            Some("updated session"),
        )],
        memories: vec![memory_record(
            "memory-1",
            MemoryScope::LongTerm,
            "updated payload",
        )],
        transcripts: vec![transcript_entry(
            "session-1",
            1,
            TranscriptEntryKind::Summary,
            "added summary",
        )],
    };
    let current = store.snapshot().expect("snapshot should load");
    let inspected = InspectedStoreSnapshot::from_snapshot(incoming.clone());

    assert!(!incoming.restore_is_additive_only_against(&current));
    assert!(incoming.restore_touches_existing_records_against(&current));
    assert!(!inspected.restore_is_additive_only_against(&current));
    assert!(inspected.restore_touches_existing_records_against(&current));
    assert!(
        !store
            .preview_restore_is_additive_only(&incoming)
            .expect("restore additive-only helper should succeed")
    );
    assert!(
        store
            .preview_restore_touches_existing_records(&incoming)
            .expect("restore existing-record helper should succeed")
    );
}
