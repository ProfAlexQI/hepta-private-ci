use super::*;

#[test]
fn restore_delta_counts_expose_additive_only_and_existing_record_flags() {
    let additive = RestoreDeltaCounts {
        added_count: 2,
        removed_count: 0,
        updated_count: 0,
        unchanged_count: 1,
    };
    let mixed = RestoreDeltaCounts {
        added_count: 1,
        removed_count: 1,
        updated_count: 1,
        unchanged_count: 0,
    };

    assert!(additive.has_additions());
    assert!(!additive.has_removals());
    assert!(!additive.has_updates());
    assert!(additive.has_changes());
    assert!(additive.is_additive_only());
    assert!(!additive.touches_existing_records());

    assert!(mixed.has_additions());
    assert!(mixed.has_removals());
    assert!(mixed.has_updates());
    assert!(mixed.has_changes());
    assert!(!mixed.is_additive_only());
    assert!(mixed.touches_existing_records());

    assert!(!RestoreDeltaCounts::default().has_changes());
    assert!(!RestoreDeltaCounts::default().is_additive_only());
    assert!(!RestoreDeltaCounts::default().touches_existing_records());
}

#[test]
fn restore_domain_impact_delegates_change_shape_helpers() {
    let impact = SnapshotRestoreDomainImpact {
        domain: SnapshotRestoreDomain::Memories,
        counts: RestoreDeltaCounts {
            added_count: 1,
            removed_count: 0,
            updated_count: 0,
            unchanged_count: 2,
        },
    };

    assert_eq!(impact.change_count(), 1);
    assert!(impact.has_changes());
    assert!(impact.has_additions());
    assert!(!impact.has_removals());
    assert!(!impact.has_updates());
    assert!(impact.is_additive_only());
    assert!(!impact.touches_existing_records());
}
