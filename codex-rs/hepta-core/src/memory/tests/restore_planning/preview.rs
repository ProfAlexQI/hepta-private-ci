use super::*;

#[test]
fn snapshot_restore_preview_deserializes_from_sparse_json() {
    let parsed: SnapshotRestorePreview = serde_json::from_str("{}")
        .expect("sparse restore preview should deserialize with defaults");

    assert_eq!(parsed, SnapshotRestorePreview::default());
    assert_eq!(parsed.change_count(), 0);
    assert!(parsed.changed_domains().is_empty());
    assert_eq!(parsed.changed_domain_count(), 0);
    assert!(!parsed.touches(SnapshotRestoreDomain::Sessions));
    assert_eq!(
        parsed.impact_for(SnapshotRestoreDomain::Transcripts),
        Some(SnapshotRestoreDomainImpact {
            domain: SnapshotRestoreDomain::Transcripts,
            counts: RestoreDeltaCounts::default(),
        })
    );
    assert!(parsed.is_noop());
    assert!(!parsed.has_integrity_issues());
}

#[test]
fn noop_restore_preview_keeps_zeroed_domain_impacts_in_stable_order() {
    let preview = SnapshotRestorePreview::default();

    assert_eq!(
        preview.domain_impacts(),
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
    assert!(preview.changed_domains().is_empty());
    assert_eq!(
        preview.impact_for(SnapshotRestoreDomain::Sessions),
        Some(SnapshotRestoreDomainImpact {
            domain: SnapshotRestoreDomain::Sessions,
            counts: RestoreDeltaCounts::default(),
        })
    );
    assert_eq!(
        preview.impact_for(SnapshotRestoreDomain::Memories),
        Some(SnapshotRestoreDomainImpact {
            domain: SnapshotRestoreDomain::Memories,
            counts: RestoreDeltaCounts::default(),
        })
    );
    assert_eq!(
        preview.impact_for(SnapshotRestoreDomain::Transcripts),
        Some(SnapshotRestoreDomainImpact {
            domain: SnapshotRestoreDomain::Transcripts,
            counts: RestoreDeltaCounts::default(),
        })
    );
}
