use super::*;

#[test]
fn store_snapshot_restore_impact_matches_preview_impact() {
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

    let impact = incoming.restore_impact_against(&current);
    let preview = incoming.restore_preview_against(&current);

    assert_eq!(impact, preview.impact());
    assert_eq!(
        impact.domain_impacts,
        vec![
            SnapshotRestoreDomainImpact {
                domain: SnapshotRestoreDomain::Sessions,
                counts: RestoreDeltaCounts {
                    added_count: 0,
                    removed_count: 0,
                    updated_count: 1,
                    unchanged_count: 0,
                },
            },
            SnapshotRestoreDomainImpact {
                domain: SnapshotRestoreDomain::Memories,
                counts: RestoreDeltaCounts {
                    added_count: 0,
                    removed_count: 0,
                    updated_count: 1,
                    unchanged_count: 0,
                },
            },
            SnapshotRestoreDomainImpact {
                domain: SnapshotRestoreDomain::Transcripts,
                counts: RestoreDeltaCounts {
                    added_count: 0,
                    removed_count: 0,
                    updated_count: 1,
                    unchanged_count: 0,
                },
            },
        ]
    );
    assert_eq!(impact.changed_domain_count(), 3);
    assert!(impact.touches(SnapshotRestoreDomain::Sessions));
    assert!(impact.touches(SnapshotRestoreDomain::Memories));
    assert!(impact.touches(SnapshotRestoreDomain::Transcripts));
    assert_eq!(impact.change_count(), 3);
    assert!(!impact.is_noop());
}

#[test]
fn store_snapshot_restore_mutation_profile_matches_preview_and_impact_helpers() {
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
        sessions: vec![
            session_record("session-1", "Current title", Some("current")),
            session_record("session-2", "Added title", Some("incoming")),
        ],
        memories: vec![memory_record(
            "memory-1",
            MemoryScope::LongTerm,
            "incoming payload",
        )],
        transcripts: vec![transcript_entry(
            "session-1",
            1,
            TranscriptEntryKind::Summary,
            "current summary",
        )],
    };

    let profile = incoming.restore_mutation_profile_against(&current);
    let preview = incoming.restore_preview_against(&current);

    assert_eq!(profile, preview.mutation_profile());
    assert_eq!(
        profile,
        incoming.restore_impact_against(&current).mutation_profile()
    );
    assert_eq!(
        profile,
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
    assert!(profile.has_changes());
    assert!(profile.has_additive_domains());
    assert!(profile.touches_existing_records());
    assert!(!profile.has_removals());
    assert!(!profile.is_additive_only());
    assert!(profile.is_ready());
}
