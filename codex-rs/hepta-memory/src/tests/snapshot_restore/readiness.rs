use super::*;

#[test]
fn store_snapshot_restore_readiness_matches_preview_and_impact_helpers() {
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

    let readiness = incoming.restore_readiness_against(&current);
    let preview = incoming.restore_preview_against(&current);

    assert_eq!(readiness, preview.readiness());
    assert_eq!(
        readiness,
        incoming.restore_impact_against(&current).readiness()
    );
    assert_eq!(
        readiness,
        SnapshotRestoreReadiness {
            change_totals: RestoreDeltaCounts {
                added_count: 0,
                removed_count: 0,
                updated_count: 3,
                unchanged_count: 0,
            },
            changed_domain_count: 3,
            current_issue_count: 0,
            incoming_issue_count: 0,
        }
    );
    assert_eq!(readiness.change_count(), 3);
    assert!(readiness.has_changes());
    assert!(!readiness.has_integrity_issues());
    assert!(!readiness.is_noop());
    assert!(readiness.is_ready());

    let safety = incoming.restore_safety_against(&current);

    assert_eq!(safety, preview.safety());
    assert_eq!(safety, incoming.restore_impact_against(&current).safety());
    assert_eq!(safety, readiness.safety());
    assert_eq!(
        safety,
        SnapshotRestoreSafety {
            change_totals: RestoreDeltaCounts {
                added_count: 0,
                removed_count: 0,
                updated_count: 3,
                unchanged_count: 0,
            },
            changed_domain_count: 3,
            current_issue_count: 0,
            incoming_issue_count: 0,
            has_changes: true,
            touches_existing_records: true,
            additive_only: false,
            has_integrity_issues: false,
        }
    );
    assert_eq!(safety.change_count(), 3);
    assert!(safety.touches_existing_records);
    assert!(!safety.additive_only);
    assert!(safety.is_ready());
}
