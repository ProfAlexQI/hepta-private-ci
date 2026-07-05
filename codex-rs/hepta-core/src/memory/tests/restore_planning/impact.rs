use super::*;

#[test]
fn snapshot_restore_impact_deserializes_without_domain_impacts_field() {
    let parsed: SnapshotRestoreImpact = serde_json::from_str(
        r#"{"change_totals":{"added_count":1},"changed_domains":["sessions"]}"#,
    )
    .expect("legacy restore impact should deserialize");

    assert_eq!(
        parsed,
        SnapshotRestoreImpact {
            change_totals: RestoreDeltaCounts {
                added_count: 1,
                ..RestoreDeltaCounts::default()
            },
            changed_domains: vec![SnapshotRestoreDomain::Sessions],
            domain_impacts: Vec::new(),
            current_issue_count: 0,
            incoming_issue_count: 0,
        }
    );
}

#[test]
fn snapshot_restore_impact_deserializes_from_sparse_json() {
    let parsed: SnapshotRestoreImpact =
        serde_json::from_str("{}").expect("sparse restore impact should deserialize with defaults");

    assert_eq!(parsed, SnapshotRestoreImpact::default());
    assert_eq!(parsed.change_count(), 0);
    assert_eq!(parsed.changed_domain_count(), 0);
    assert_eq!(parsed.total_issue_count(), 0);
    assert!(parsed.is_noop());
    assert!(!parsed.has_integrity_issues());
}

#[test]
fn noop_restore_impact_keeps_zeroed_domain_impacts_in_stable_order() {
    let impact = SnapshotRestorePreview::default().impact();

    assert!(impact.changed_domains.is_empty());
    assert_eq!(
        impact.domain_impacts,
        vec![
            SnapshotRestoreDomainImpact {
                domain: SnapshotRestoreDomain::Sessions,
                counts: RestoreDeltaCounts::default(),
            },
            SnapshotRestoreDomainImpact {
                domain: SnapshotRestoreDomain::Memories,
                counts: RestoreDeltaCounts::default(),
            },
            SnapshotRestoreDomainImpact {
                domain: SnapshotRestoreDomain::Transcripts,
                counts: RestoreDeltaCounts::default(),
            },
        ]
    );
    assert_eq!(
        impact
            .impact_for(SnapshotRestoreDomain::Sessions)
            .expect("session impact should be present"),
        &SnapshotRestoreDomainImpact {
            domain: SnapshotRestoreDomain::Sessions,
            counts: RestoreDeltaCounts::default(),
        }
    );
    assert_eq!(
        impact
            .impact_for(SnapshotRestoreDomain::Memories)
            .expect("memory impact should be present"),
        &SnapshotRestoreDomainImpact {
            domain: SnapshotRestoreDomain::Memories,
            counts: RestoreDeltaCounts::default(),
        }
    );
    assert_eq!(
        impact
            .impact_for(SnapshotRestoreDomain::Transcripts)
            .expect("transcript impact should be present"),
        &SnapshotRestoreDomainImpact {
            domain: SnapshotRestoreDomain::Transcripts,
            counts: RestoreDeltaCounts::default(),
        }
    );
    assert!(impact.is_noop());
}

#[test]
fn restore_impact_and_readiness_surface_change_shape_flags() {
    let additive_impact = SnapshotRestoreImpact {
        change_totals: RestoreDeltaCounts {
            added_count: 3,
            removed_count: 0,
            updated_count: 0,
            unchanged_count: 4,
        },
        changed_domains: vec![SnapshotRestoreDomain::Sessions],
        domain_impacts: vec![SnapshotRestoreDomainImpact {
            domain: SnapshotRestoreDomain::Sessions,
            counts: RestoreDeltaCounts {
                added_count: 3,
                removed_count: 0,
                updated_count: 0,
                unchanged_count: 4,
            },
        }],
        current_issue_count: 0,
        incoming_issue_count: 0,
    };
    let mixed_impact = SnapshotRestoreImpact {
        change_totals: RestoreDeltaCounts {
            added_count: 1,
            removed_count: 1,
            updated_count: 1,
            unchanged_count: 0,
        },
        changed_domains: vec![
            SnapshotRestoreDomain::Sessions,
            SnapshotRestoreDomain::Memories,
            SnapshotRestoreDomain::Transcripts,
        ],
        domain_impacts: Vec::new(),
        current_issue_count: 0,
        incoming_issue_count: 0,
    };
    let additive_readiness = additive_impact.readiness();
    let mixed_readiness = mixed_impact.readiness();

    assert!(additive_impact.has_additions());
    assert!(!additive_impact.has_removals());
    assert!(!additive_impact.has_updates());
    assert!(additive_impact.is_additive_only());
    assert!(!additive_impact.touches_existing_records());

    assert!(additive_readiness.has_additions());
    assert!(!additive_readiness.has_removals());
    assert!(!additive_readiness.has_updates());
    assert!(additive_readiness.is_additive_only());
    assert!(!additive_readiness.touches_existing_records());

    assert!(mixed_impact.has_additions());
    assert!(mixed_impact.has_removals());
    assert!(mixed_impact.has_updates());
    assert!(!mixed_impact.is_additive_only());
    assert!(mixed_impact.touches_existing_records());

    assert!(mixed_readiness.has_additions());
    assert!(mixed_readiness.has_removals());
    assert!(mixed_readiness.has_updates());
    assert!(!mixed_readiness.is_additive_only());
    assert!(mixed_readiness.touches_existing_records());
}
