use super::*;

#[test]
fn snapshot_restore_impact_compacts_changed_domains_counts_and_issue_flags() {
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

    let impact = preview.impact();

    assert_eq!(impact.change_totals, preview.change_totals());
    assert_eq!(
        impact.changed_domains,
        vec![
            SnapshotRestoreDomain::Sessions,
            SnapshotRestoreDomain::Memories,
        ]
    );
    assert_eq!(
        impact.domain_impacts,
        vec![
            SnapshotRestoreDomainImpact {
                domain: SnapshotRestoreDomain::Sessions,
                counts: RestoreDeltaCounts {
                    added_count: 0,
                    removed_count: 0,
                    updated_count: 1,
                    unchanged_count: 2,
                },
            },
            SnapshotRestoreDomainImpact {
                domain: SnapshotRestoreDomain::Memories,
                counts: RestoreDeltaCounts {
                    added_count: 1,
                    removed_count: 0,
                    updated_count: 0,
                    unchanged_count: 1,
                },
            },
            SnapshotRestoreDomainImpact {
                domain: SnapshotRestoreDomain::Transcripts,
                counts: RestoreDeltaCounts {
                    added_count: 0,
                    removed_count: 0,
                    updated_count: 0,
                    unchanged_count: 3,
                },
            },
        ]
    );
    assert_eq!(impact.changed_domain_count(), 2);
    assert!(impact.touches(SnapshotRestoreDomain::Sessions));
    assert!(impact.touches(SnapshotRestoreDomain::Memories));
    assert!(!impact.touches(SnapshotRestoreDomain::Transcripts));
    assert_eq!(
        impact
            .impact_for(SnapshotRestoreDomain::Memories)
            .expect("memory impact should be present")
            .counts,
        RestoreDeltaCounts {
            added_count: 1,
            removed_count: 0,
            updated_count: 0,
            unchanged_count: 1,
        }
    );
    assert_eq!(impact.current_issue_count, 1);
    assert_eq!(impact.incoming_issue_count, 2);
    assert_eq!(impact.total_issue_count(), 3);
    assert!(impact.has_integrity_issues());
    assert_eq!(impact.change_count(), 2);
    assert!(!impact.is_noop());
}

#[test]
fn snapshot_restore_impact_roundtrips_through_json() {
    let impact = SnapshotRestoreImpact {
        change_totals: RestoreDeltaCounts {
            added_count: 1,
            removed_count: 2,
            updated_count: 3,
            unchanged_count: 4,
        },
        changed_domains: vec![
            SnapshotRestoreDomain::Memories,
            SnapshotRestoreDomain::Transcripts,
        ],
        domain_impacts: vec![
            SnapshotRestoreDomainImpact {
                domain: SnapshotRestoreDomain::Sessions,
                counts: RestoreDeltaCounts {
                    added_count: 0,
                    removed_count: 0,
                    updated_count: 0,
                    unchanged_count: 1,
                },
            },
            SnapshotRestoreDomainImpact {
                domain: SnapshotRestoreDomain::Memories,
                counts: RestoreDeltaCounts {
                    added_count: 1,
                    removed_count: 0,
                    updated_count: 1,
                    unchanged_count: 2,
                },
            },
            SnapshotRestoreDomainImpact {
                domain: SnapshotRestoreDomain::Transcripts,
                counts: RestoreDeltaCounts {
                    added_count: 0,
                    removed_count: 2,
                    updated_count: 2,
                    unchanged_count: 1,
                },
            },
        ],
        current_issue_count: 5,
        incoming_issue_count: 6,
    };

    let json = serde_json::to_string(&impact).expect("restore impact should serialize");
    let parsed: SnapshotRestoreImpact =
        serde_json::from_str(&json).expect("restore impact should deserialize");

    assert_eq!(parsed, impact);
    assert_eq!(parsed.change_count(), 6);
    assert_eq!(parsed.changed_domain_count(), 2);
    assert_eq!(parsed.domain_impacts.len(), 3);
    assert_eq!(parsed.total_issue_count(), 11);
    assert!(parsed.has_integrity_issues());
}
