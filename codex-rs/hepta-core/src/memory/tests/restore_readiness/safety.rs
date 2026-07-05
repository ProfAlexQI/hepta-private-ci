use super::*;

#[test]
fn snapshot_restore_safety_matches_preview_impact_and_readiness() {
    let preview = SnapshotRestorePreview {
        current_audit: SnapshotAuditReport {
            memory_integrity: MemorySnapshotIntegrityReport {
                blank_memory_content_count: 1,
                ..MemorySnapshotIntegrityReport::default()
            },
            ..SnapshotAuditReport::default()
        },
        incoming_audit: SnapshotAuditReport {
            transcript_integrity: TranscriptSnapshotIntegrityReport {
                blank_content_count: 2,
                ..TranscriptSnapshotIntegrityReport::default()
            },
            ..SnapshotAuditReport::default()
        },
        session_delta: SessionRestoreDelta {
            added_session_ids: vec![SessionId("session-2".into())],
            unchanged_count: 1,
            ..SessionRestoreDelta::default()
        },
        memory_delta: MemoryRestoreDelta {
            updated_memory_ids: vec!["memory-1".into()],
            unchanged_count: 2,
            ..MemoryRestoreDelta::default()
        },
        transcript_delta: TranscriptRestoreDelta {
            unchanged_count: 3,
            ..TranscriptRestoreDelta::default()
        },
    };

    let readiness = preview.readiness();
    let safety = preview.safety();

    assert_eq!(safety, SnapshotRestoreSafety::from_preview(&preview));
    assert_eq!(safety, preview.impact().safety());
    assert_eq!(safety, readiness.safety());
    assert_eq!(safety.change_totals, preview.change_totals());
    assert_eq!(safety.changed_domain_count, 2);
    assert_eq!(safety.change_count(), 2);
    assert!(safety.has_additions());
    assert!(safety.has_updates());
    assert!(!safety.has_removals());
    assert!(safety.has_changes);
    assert!(safety.touches_existing_records);
    assert!(!safety.additive_only);
    assert_eq!(safety.total_issue_count(), 3);
    assert!(safety.has_integrity_issues);
    assert!(!safety.is_ready());
    assert!(!safety.is_noop());
}

#[test]
fn snapshot_restore_safety_roundtrips_through_json() {
    let safety = SnapshotRestoreSafety {
        change_totals: RestoreDeltaCounts {
            added_count: 1,
            removed_count: 2,
            updated_count: 0,
            unchanged_count: 4,
        },
        changed_domain_count: 2,
        current_issue_count: 3,
        incoming_issue_count: 5,
        has_changes: true,
        touches_existing_records: true,
        additive_only: false,
        has_integrity_issues: true,
    };

    let json = serde_json::to_string(&safety).expect("restore safety should serialize");
    let parsed: SnapshotRestoreSafety =
        serde_json::from_str(&json).expect("restore safety should deserialize");

    assert_eq!(parsed, safety);
    assert_eq!(parsed.change_count(), 3);
    assert_eq!(parsed.total_issue_count(), 8);
    assert!(parsed.has_changes);
    assert!(parsed.touches_existing_records);
    assert!(parsed.has_integrity_issues);
    assert!(!parsed.is_ready());
}

#[test]
fn snapshot_restore_safety_deserializes_from_sparse_json() {
    let parsed: SnapshotRestoreSafety =
        serde_json::from_str("{}").expect("sparse restore safety should deserialize with defaults");

    assert_eq!(parsed, SnapshotRestoreSafety::default());
    assert_eq!(parsed.change_count(), 0);
    assert_eq!(parsed.changed_domain_count, 0);
    assert_eq!(parsed.total_issue_count(), 0);
    assert!(!parsed.has_changes);
    assert!(!parsed.touches_existing_records);
    assert!(!parsed.additive_only);
    assert!(!parsed.has_integrity_issues);
    assert!(parsed.is_ready());
    assert!(parsed.is_noop());
}
