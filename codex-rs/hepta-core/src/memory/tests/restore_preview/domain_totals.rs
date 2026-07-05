use super::*;

#[test]
fn snapshot_restore_preview_change_totals_roll_up_each_domain() {
    let preview = SnapshotRestorePreview {
        current_audit: SnapshotAuditReport::default(),
        incoming_audit: SnapshotAuditReport::default(),
        session_delta: SessionRestoreDelta {
            added_session_ids: vec![SessionId("session-2".into())],
            removed_session_ids: vec![SessionId("session-3".into())],
            updated_session_ids: vec![SessionId("session-1".into())],
            unchanged_count: 4,
        },
        memory_delta: MemoryRestoreDelta {
            added_memory_ids: vec!["memory-2".into()],
            removed_memory_ids: vec![],
            updated_memory_ids: vec!["memory-1".into()],
            unchanged_count: 5,
        },
        transcript_delta: TranscriptRestoreDelta {
            added_entry_ids: vec!["entry-3".into()],
            removed_entry_ids: vec!["entry-4".into()],
            updated_entry_ids: vec![],
            unchanged_count: 6,
        },
    };

    assert_eq!(
        preview.session_delta.counts(),
        RestoreDeltaCounts {
            added_count: 1,
            removed_count: 1,
            updated_count: 1,
            unchanged_count: 4,
        }
    );
    assert_eq!(
        preview.memory_delta.counts(),
        RestoreDeltaCounts {
            added_count: 1,
            removed_count: 0,
            updated_count: 1,
            unchanged_count: 5,
        }
    );
    assert_eq!(
        preview.transcript_delta.counts(),
        RestoreDeltaCounts {
            added_count: 1,
            removed_count: 1,
            updated_count: 0,
            unchanged_count: 6,
        }
    );
    assert_eq!(
        preview.change_totals(),
        RestoreDeltaCounts {
            added_count: 3,
            removed_count: 2,
            updated_count: 2,
            unchanged_count: 15,
        }
    );
    assert_eq!(
        preview.change_totals().change_count(),
        preview.change_count()
    );
    assert_eq!(
        preview.domain_impacts(),
        vec![
            SnapshotRestoreDomainImpact {
                domain: SnapshotRestoreDomain::Sessions,
                counts: RestoreDeltaCounts {
                    added_count: 1,
                    removed_count: 1,
                    updated_count: 1,
                    unchanged_count: 4,
                },
            },
            SnapshotRestoreDomainImpact {
                domain: SnapshotRestoreDomain::Memories,
                counts: RestoreDeltaCounts {
                    added_count: 1,
                    removed_count: 0,
                    updated_count: 1,
                    unchanged_count: 5,
                },
            },
            SnapshotRestoreDomainImpact {
                domain: SnapshotRestoreDomain::Transcripts,
                counts: RestoreDeltaCounts {
                    added_count: 1,
                    removed_count: 1,
                    updated_count: 0,
                    unchanged_count: 6,
                },
            },
        ]
    );
    assert_eq!(
        preview.changed_domains(),
        vec![
            SnapshotRestoreDomain::Sessions,
            SnapshotRestoreDomain::Memories,
            SnapshotRestoreDomain::Transcripts,
        ]
    );
    assert_eq!(preview.changed_domain_count(), 3);
    assert!(preview.touches(SnapshotRestoreDomain::Sessions));
    assert!(preview.touches(SnapshotRestoreDomain::Memories));
    assert!(preview.touches(SnapshotRestoreDomain::Transcripts));
    assert_eq!(
        preview.impact_for(SnapshotRestoreDomain::Memories),
        Some(SnapshotRestoreDomainImpact {
            domain: SnapshotRestoreDomain::Memories,
            counts: RestoreDeltaCounts {
                added_count: 1,
                removed_count: 0,
                updated_count: 1,
                unchanged_count: 5,
            },
        })
    );
}
