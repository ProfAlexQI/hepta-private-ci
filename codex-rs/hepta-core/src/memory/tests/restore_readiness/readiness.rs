use super::*;

#[test]
fn snapshot_restore_readiness_matches_preview_and_impact_helpers() {
    let preview = SnapshotRestorePreview {
        current_audit: SnapshotAuditReport {
            memory_integrity: MemorySnapshotIntegrityReport {
                blank_session_title_count: 1,
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
            updated_session_ids: vec![SessionId("session-1".into())],
            unchanged_count: 2,
            ..SessionRestoreDelta::default()
        },
        memory_delta: MemoryRestoreDelta {
            added_memory_ids: vec!["memory-2".into()],
            unchanged_count: 1,
            ..MemoryRestoreDelta::default()
        },
        transcript_delta: TranscriptRestoreDelta {
            unchanged_count: 3,
            ..TranscriptRestoreDelta::default()
        },
    };

    let readiness = preview.readiness();

    assert_eq!(readiness, SnapshotRestoreReadiness::from_preview(&preview));
    assert_eq!(readiness, preview.impact().readiness());
    assert_eq!(readiness.change_totals, preview.change_totals());
    assert_eq!(readiness.changed_domain_count, 2);
    assert_eq!(readiness.change_count(), 2);
    assert!(readiness.has_changes());
    assert_eq!(readiness.current_issue_count, 1);
    assert_eq!(readiness.incoming_issue_count, 2);
    assert_eq!(readiness.total_issue_count(), 3);
    assert!(readiness.has_integrity_issues());
    assert!(!readiness.is_noop());
    assert!(!readiness.is_ready());
}

#[test]
fn snapshot_restore_readiness_roundtrips_through_json() {
    let readiness = SnapshotRestoreReadiness {
        change_totals: RestoreDeltaCounts {
            added_count: 1,
            removed_count: 2,
            updated_count: 3,
            unchanged_count: 4,
        },
        changed_domain_count: 2,
        current_issue_count: 5,
        incoming_issue_count: 6,
    };

    let json = serde_json::to_string(&readiness).expect("restore readiness should serialize");
    let parsed: SnapshotRestoreReadiness =
        serde_json::from_str(&json).expect("restore readiness should deserialize");

    assert_eq!(parsed, readiness);
    assert_eq!(parsed.change_count(), 6);
    assert_eq!(parsed.total_issue_count(), 11);
    assert!(parsed.has_changes());
    assert!(parsed.has_integrity_issues());
    assert!(!parsed.is_ready());
}

#[test]
fn snapshot_restore_readiness_deserializes_from_sparse_json() {
    let parsed: SnapshotRestoreReadiness = serde_json::from_str("{}")
        .expect("sparse restore readiness should deserialize with defaults");

    assert_eq!(parsed, SnapshotRestoreReadiness::default());
    assert_eq!(parsed.change_count(), 0);
    assert_eq!(parsed.changed_domain_count, 0);
    assert_eq!(parsed.total_issue_count(), 0);
    assert!(!parsed.has_changes());
    assert!(!parsed.has_integrity_issues());
    assert!(parsed.is_noop());
    assert!(parsed.is_ready());
}
