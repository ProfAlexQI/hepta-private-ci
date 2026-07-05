use super::*;

#[tokio::test]
async fn preview_restore_safety_matches_snapshot_helper() {
    let store = InMemoryStore::default();
    store
        .upsert_session_sync(session_record(
            "session-1",
            "Current foundation",
            Some("preview restore safety"),
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
        sessions: vec![
            session_record("session-1", "Updated foundation", Some("incoming")),
            session_record("session-2", "Added foundation", None),
        ],
        memories: vec![memory_record(
            "memory-1",
            MemoryScope::LongTerm,
            "updated payload",
        )],
        transcripts: vec![],
    };

    let from_store = store
        .preview_restore_safety(&incoming)
        .expect("restore safety should succeed");
    let current = store.snapshot().expect("snapshot should load");

    assert_eq!(from_store, incoming.restore_safety_against(&current));
    assert_eq!(
        from_store,
        incoming.restore_preview_against(&current).safety()
    );
    assert_eq!(from_store.change_count(), 3);
    assert!(from_store.has_changes);
    assert!(from_store.touches_existing_records);
    assert!(!from_store.additive_only);
    assert!(!from_store.has_integrity_issues);
    assert!(from_store.is_ready());
}

#[tokio::test]
async fn preview_restore_mutation_profile_matches_snapshot_helper() {
    let store = InMemoryStore::default();
    store
        .upsert_session_sync(session_record(
            "session-1",
            "Current foundation",
            Some("preview restore mutation profile"),
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
        sessions: vec![
            session_record(
                "session-1",
                "Current foundation",
                Some("preview restore mutation profile"),
            ),
            session_record("session-2", "Added foundation", Some("incoming")),
        ],
        memories: vec![memory_record(
            "memory-1",
            MemoryScope::Session,
            "updated payload",
        )],
        transcripts: vec![],
    };
    let current = store.snapshot().expect("snapshot should load");

    let from_store = store
        .preview_restore_mutation_profile(&incoming)
        .expect("restore mutation profile should succeed");

    assert_eq!(
        from_store,
        incoming.restore_mutation_profile_against(&current)
    );
    assert_eq!(
        from_store,
        SnapshotRestoreMutationProfile {
            changed_domain_count: 2,
            unchanged_domain_count: 1,
            addition_domain_count: 1,
            additive_only_domain_count: 1,
            existing_record_domain_count: 1,
            removal_domain_count: 0,
            current_issue_count: 0,
            incoming_issue_count: 0,
        }
    );
    assert!(from_store.has_changes());
    assert!(from_store.has_additive_domains());
    assert!(from_store.touches_existing_records());
    assert!(!from_store.has_removals());
    assert!(!from_store.is_additive_only());
}

#[tokio::test]
async fn preview_restore_domain_impacts_match_snapshot_helper() {
    let store = InMemoryStore::default();
    store
        .upsert_session_sync(session_record(
            "session-1",
            "Current foundation",
            Some("preview restore domains"),
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
    store
        .append(transcript_entry(
            "session-1",
            1,
            TranscriptEntryKind::Message,
            "current message",
        ))
        .await
        .expect("append should succeed");

    let incoming = StoreSnapshot {
        sessions: vec![
            session_record("session-1", "Updated foundation", Some("incoming")),
            session_record("session-2", "Added foundation", None),
        ],
        memories: vec![
            memory_record("memory-1", MemoryScope::Session, "updated payload"),
            memory_record("memory-2", MemoryScope::LongTerm, "added payload"),
        ],
        transcripts: vec![
            transcript_entry(
                "session-1",
                1,
                TranscriptEntryKind::Message,
                "updated message",
            ),
            transcript_entry(
                "session-2",
                1,
                TranscriptEntryKind::Summary,
                "added summary",
            ),
        ],
    };

    let from_store = store
        .preview_restore_domain_impacts(&incoming)
        .expect("restore domain impacts should succeed");

    assert_eq!(
        from_store,
        incoming.restore_domain_impacts_against(&store.snapshot().expect("snapshot should load"))
    );
    assert_eq!(
        from_store,
        vec![
            SnapshotRestoreDomainImpact {
                domain: SnapshotRestoreDomain::Sessions,
                counts: RestoreDeltaCounts {
                    added_count: 1,
                    removed_count: 0,
                    updated_count: 1,
                    unchanged_count: 0,
                },
            },
            SnapshotRestoreDomainImpact {
                domain: SnapshotRestoreDomain::Memories,
                counts: RestoreDeltaCounts {
                    added_count: 1,
                    removed_count: 0,
                    updated_count: 1,
                    unchanged_count: 0,
                },
            },
            SnapshotRestoreDomainImpact {
                domain: SnapshotRestoreDomain::Transcripts,
                counts: RestoreDeltaCounts {
                    added_count: 1,
                    removed_count: 0,
                    updated_count: 1,
                    unchanged_count: 0,
                },
            },
        ]
    );

    let changed_domains = store
        .preview_restore_changed_domains(&incoming)
        .expect("restore changed domains should succeed");

    assert_eq!(
        changed_domains,
        incoming.restore_changed_domains_against(&store.snapshot().expect("snapshot should load"))
    );
    assert_eq!(
        changed_domains,
        vec![
            SnapshotRestoreDomain::Sessions,
            SnapshotRestoreDomain::Memories,
            SnapshotRestoreDomain::Transcripts,
        ]
    );
}

#[tokio::test]
async fn preview_restore_summarizes_replace_style_changes_before_restore() {
    let store = InMemoryStore::default();
    store
        .upsert_session_sync(session_record(
            "session-1",
            "Current foundation",
            Some("preview restore"),
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
    store
        .append(transcript_entry(
            "session-1",
            1,
            TranscriptEntryKind::Message,
            "current message",
        ))
        .await
        .expect("append should succeed");

    let incoming = StoreSnapshot {
        sessions: vec![
            session_record("session-1", "Updated foundation", Some("incoming")),
            session_record("session-2", "Added foundation", None),
        ],
        memories: vec![
            memory_record("memory-1", MemoryScope::Session, "updated payload"),
            memory_record("memory-2", MemoryScope::LongTerm, "added payload"),
        ],
        transcripts: vec![
            transcript_entry(
                "session-1",
                1,
                TranscriptEntryKind::Message,
                "updated message",
            ),
            transcript_entry(
                "session-2",
                1,
                TranscriptEntryKind::Summary,
                "added summary",
            ),
        ],
    };

    let preview = store
        .preview_restore(&incoming)
        .expect("restore preview should succeed");

    assert_eq!(
        preview.session_delta.added_session_ids,
        vec![SessionId("session-2".into())]
    );
    assert_eq!(
        preview.session_delta.updated_session_ids,
        vec![SessionId("session-1".into())]
    );
    assert_eq!(
        preview.memory_delta.added_memory_ids,
        vec!["memory-2".to_string()]
    );
    assert_eq!(
        preview.memory_delta.updated_memory_ids,
        vec!["memory-1".to_string()]
    );
    assert_eq!(
        preview.transcript_delta.added_entry_ids,
        vec!["session-2-1".to_string()]
    );
    assert_eq!(
        preview.transcript_delta.updated_entry_ids,
        vec!["session-1-1".to_string()]
    );
    assert_eq!(
        preview.session_delta.counts(),
        RestoreDeltaCounts {
            added_count: 1,
            removed_count: 0,
            updated_count: 1,
            unchanged_count: 0,
        }
    );
    assert_eq!(
        preview.memory_delta.counts(),
        RestoreDeltaCounts {
            added_count: 1,
            removed_count: 0,
            updated_count: 1,
            unchanged_count: 0,
        }
    );
    assert_eq!(
        preview.transcript_delta.counts(),
        RestoreDeltaCounts {
            added_count: 1,
            removed_count: 0,
            updated_count: 1,
            unchanged_count: 0,
        }
    );
    assert_eq!(
        preview.change_totals(),
        RestoreDeltaCounts {
            added_count: 3,
            removed_count: 0,
            updated_count: 3,
            unchanged_count: 0,
        }
    );
    assert_eq!(preview.change_count(), 6);
    assert!(!preview.is_noop());
    assert!(!preview.has_integrity_issues());
}

#[tokio::test]
async fn preview_restore_impact_compacts_store_restore_summary() {
    let store = InMemoryStore::default();
    store
        .upsert_session_sync(session_record(
            "session-1",
            "Current foundation",
            Some("preview restore impact"),
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
            "session-2",
            "Added foundation",
            Some("incoming"),
        )],
        memories: vec![memory_record(
            "memory-1",
            MemoryScope::LongTerm,
            "updated payload",
        )],
        transcripts: vec![transcript_entry(
            "session-2",
            1,
            TranscriptEntryKind::Summary,
            "added summary",
        )],
    };

    let impact = store
        .preview_restore_impact(&incoming)
        .expect("restore impact should succeed");
    let preview = store
        .preview_restore(&incoming)
        .expect("restore preview should succeed");

    assert_eq!(impact, preview.impact());
    assert_eq!(
        impact.changed_domains,
        vec![
            SnapshotRestoreDomain::Sessions,
            SnapshotRestoreDomain::Memories,
            SnapshotRestoreDomain::Transcripts,
        ]
    );
    assert_eq!(
        impact.change_totals,
        RestoreDeltaCounts {
            added_count: 2,
            removed_count: 1,
            updated_count: 1,
            unchanged_count: 0,
        }
    );
    assert_eq!(impact.change_count(), 4);
    assert_eq!(impact.total_issue_count(), 0);
    assert!(!impact.has_integrity_issues());
    assert!(!impact.is_noop());
}

#[tokio::test]
async fn preview_restore_readiness_compacts_store_restore_summary() {
    let store = InMemoryStore::default();
    store
        .upsert_session_sync(session_record(
            "session-1",
            "Current foundation",
            Some("preview restore readiness"),
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
            "session-2",
            "Added foundation",
            Some("incoming"),
        )],
        memories: vec![memory_record(
            "memory-1",
            MemoryScope::LongTerm,
            "updated payload",
        )],
        transcripts: vec![transcript_entry(
            "session-2",
            1,
            TranscriptEntryKind::Summary,
            "added summary",
        )],
    };

    let readiness = store
        .preview_restore_readiness(&incoming)
        .expect("restore readiness should succeed");
    let preview = store
        .preview_restore(&incoming)
        .expect("restore preview should succeed");

    assert_eq!(readiness, preview.readiness());
    assert_eq!(
        readiness,
        incoming.restore_readiness_against(&store.snapshot().expect("snapshot should load"))
    );
    assert_eq!(
        readiness.change_totals,
        RestoreDeltaCounts {
            added_count: 2,
            removed_count: 1,
            updated_count: 1,
            unchanged_count: 0,
        }
    );
    assert_eq!(readiness.changed_domain_count, 3);
    assert_eq!(readiness.change_count(), 4);
    assert_eq!(readiness.total_issue_count(), 0);
    assert!(readiness.has_changes());
    assert!(!readiness.has_integrity_issues());
    assert!(!readiness.is_noop());
    assert!(readiness.is_ready());
}
