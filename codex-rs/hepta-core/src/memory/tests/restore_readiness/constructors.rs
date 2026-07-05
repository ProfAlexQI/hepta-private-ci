use super::*;

#[test]
fn snapshot_restore_readiness_and_safety_constructors_preserve_flags() {
    let impact = SnapshotRestoreImpact {
        change_totals: RestoreDeltaCounts {
            added_count: 1,
            removed_count: 1,
            updated_count: 0,
            unchanged_count: 2,
        },
        changed_domains: vec![
            SnapshotRestoreDomain::Sessions,
            SnapshotRestoreDomain::Memories,
        ],
        domain_impacts: vec![SnapshotRestoreDomainImpact {
            domain: SnapshotRestoreDomain::Sessions,
            counts: RestoreDeltaCounts {
                added_count: 1,
                removed_count: 0,
                updated_count: 0,
                unchanged_count: 1,
            },
        }],
        current_issue_count: 1,
        incoming_issue_count: 2,
    };

    let readiness = SnapshotRestoreReadiness::from_impact(&impact);
    let safety_from_impact = SnapshotRestoreSafety::from_impact(&impact);
    let safety_from_readiness = SnapshotRestoreSafety::from_readiness(&readiness);

    assert_eq!(readiness, impact.readiness());
    assert_eq!(safety_from_impact, impact.safety());
    assert_eq!(safety_from_readiness, readiness.safety());
    assert_eq!(readiness.change_count(), 2);
    assert_eq!(readiness.changed_domain_count, 2);
    assert!(readiness.has_changes());
    assert!(readiness.has_additions());
    assert!(readiness.has_removals());
    assert!(!readiness.has_updates());
    assert!(readiness.touches_existing_records());
    assert!(!readiness.is_additive_only());
    assert!(readiness.has_integrity_issues());
    assert!(!readiness.is_ready());

    assert!(safety_from_impact.has_changes);
    assert!(safety_from_impact.touches_existing_records);
    assert!(!safety_from_impact.additive_only);
    assert!(safety_from_impact.has_integrity_issues);
    assert!(!safety_from_impact.is_ready());
}
