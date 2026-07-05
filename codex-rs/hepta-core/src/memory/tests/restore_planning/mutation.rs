use super::*;

#[test]
fn restore_mutation_profile_summarizes_domain_shape() {
    let impact = SnapshotRestoreImpact {
        change_totals: RestoreDeltaCounts {
            added_count: 3,
            removed_count: 1,
            updated_count: 1,
            unchanged_count: 5,
        },
        changed_domains: vec![
            SnapshotRestoreDomain::Sessions,
            SnapshotRestoreDomain::Memories,
        ],
        domain_impacts: vec![
            SnapshotRestoreDomainImpact {
                domain: SnapshotRestoreDomain::Sessions,
                counts: RestoreDeltaCounts {
                    added_count: 2,
                    removed_count: 0,
                    updated_count: 0,
                    unchanged_count: 1,
                },
            },
            SnapshotRestoreDomainImpact {
                domain: SnapshotRestoreDomain::Memories,
                counts: RestoreDeltaCounts {
                    added_count: 1,
                    removed_count: 1,
                    updated_count: 1,
                    unchanged_count: 0,
                },
            },
            SnapshotRestoreDomainImpact {
                domain: SnapshotRestoreDomain::Transcripts,
                counts: RestoreDeltaCounts {
                    added_count: 0,
                    removed_count: 0,
                    updated_count: 0,
                    unchanged_count: 4,
                },
            },
        ],
        current_issue_count: 1,
        incoming_issue_count: 2,
    };

    let profile = impact.mutation_profile();

    assert_eq!(profile.changed_domain_count, 2);
    assert_eq!(profile.unchanged_domain_count, 1);
    assert_eq!(profile.addition_domain_count, 2);
    assert_eq!(profile.additive_only_domain_count, 1);
    assert_eq!(profile.existing_record_domain_count, 1);
    assert_eq!(profile.removal_domain_count, 1);
    assert_eq!(profile.total_issue_count(), 3);
    assert!(profile.has_changes());
    assert!(profile.has_additive_domains());
    assert!(profile.touches_existing_records());
    assert!(profile.has_removals());
    assert!(!profile.is_additive_only());
    assert!(profile.has_integrity_issues());
    assert!(!profile.is_ready());
}

#[test]
fn restore_mutation_profile_matches_preview_helper() {
    let current_sessions = vec![SessionRecord {
        session_id: SessionId("session-1".into()),
        agent_id: AgentId("builder".into()),
        title: "Current".into(),
        created_at_unix_ms: 1,
        last_active_unix_ms: 2,
        last_user_intent_summary: None,
        archived_at_unix_ms: None,
    }];
    let incoming_sessions = vec![
        current_sessions[0].clone(),
        SessionRecord {
            session_id: SessionId("session-2".into()),
            agent_id: AgentId("builder".into()),
            title: "Added".into(),
            created_at_unix_ms: 3,
            last_active_unix_ms: 4,
            last_user_intent_summary: Some("additive restore".into()),
            archived_at_unix_ms: None,
        },
    ];
    let current_memories = vec![MemoryRecord {
        id: "memory-1".into(),
        scope: MemoryScope::LongTerm,
        content: "current payload".into(),
    }];
    let incoming_memories = vec![MemoryRecord {
        id: "memory-1".into(),
        scope: MemoryScope::LongTerm,
        content: "updated payload".into(),
    }];
    let preview = SnapshotRestorePreview::from_records_and_entries(
        &current_sessions,
        &current_memories,
        &[],
        &incoming_sessions,
        &incoming_memories,
        &[],
    );

    let profile = preview.mutation_profile();

    assert_eq!(profile, preview.impact().mutation_profile());
    assert_eq!(profile.changed_domain_count, 2);
    assert_eq!(profile.unchanged_domain_count, 1);
    assert_eq!(profile.addition_domain_count, 1);
    assert_eq!(profile.additive_only_domain_count, 1);
    assert_eq!(profile.existing_record_domain_count, 1);
    assert_eq!(profile.removal_domain_count, 0);
    assert!(profile.has_changes());
    assert!(profile.has_additive_domains());
    assert!(profile.touches_existing_records());
    assert!(!profile.has_removals());
    assert!(!profile.is_additive_only());
    assert!(profile.is_ready());
}

#[test]
fn restore_mutation_profile_roundtrips_through_json() {
    let profile = SnapshotRestoreMutationProfile {
        changed_domain_count: 2,
        unchanged_domain_count: 1,
        addition_domain_count: 2,
        additive_only_domain_count: 1,
        existing_record_domain_count: 1,
        removal_domain_count: 1,
        current_issue_count: 3,
        incoming_issue_count: 5,
    };

    let json = serde_json::to_string(&profile).expect("mutation profile should serialize");
    let parsed: SnapshotRestoreMutationProfile =
        serde_json::from_str(&json).expect("mutation profile should deserialize");

    assert_eq!(parsed, profile);
    assert_eq!(parsed.total_issue_count(), 8);
    assert!(parsed.has_changes());
    assert!(parsed.has_additive_domains());
    assert!(parsed.touches_existing_records());
    assert!(parsed.has_removals());
    assert!(!parsed.is_additive_only());
    assert!(parsed.has_integrity_issues());
    assert!(!parsed.is_noop());
    assert!(!parsed.is_ready());
}

#[test]
fn restore_mutation_profile_deserializes_from_sparse_json() {
    let parsed: SnapshotRestoreMutationProfile = serde_json::from_str("{}")
        .expect("sparse mutation profile should deserialize with defaults");

    assert_eq!(parsed, SnapshotRestoreMutationProfile::default());
    assert_eq!(parsed.changed_domain_count, 0);
    assert_eq!(parsed.unchanged_domain_count, 0);
    assert_eq!(parsed.total_issue_count(), 0);
    assert!(!parsed.has_changes());
    assert!(!parsed.has_additive_domains());
    assert!(!parsed.touches_existing_records());
    assert!(!parsed.has_removals());
    assert!(parsed.is_noop());
    assert!(parsed.is_ready());
}

#[test]
fn restore_mutation_profile_falls_back_to_impact_flags_without_domain_impacts() {
    let impact = SnapshotRestoreImpact {
        change_totals: RestoreDeltaCounts {
            added_count: 2,
            removed_count: 0,
            updated_count: 1,
            unchanged_count: 0,
        },
        changed_domains: vec![
            SnapshotRestoreDomain::Sessions,
            SnapshotRestoreDomain::Memories,
        ],
        domain_impacts: Vec::new(),
        current_issue_count: 0,
        incoming_issue_count: 0,
    };

    let profile = impact.mutation_profile();

    assert_eq!(profile.changed_domain_count, 2);
    assert_eq!(profile.unchanged_domain_count, 1);
    assert_eq!(profile.addition_domain_count, 1);
    assert_eq!(profile.additive_only_domain_count, 0);
    assert_eq!(profile.existing_record_domain_count, 1);
    assert_eq!(profile.removal_domain_count, 0);
    assert!(profile.has_changes());
    assert!(!profile.has_additive_domains());
    assert!(profile.touches_existing_records());
    assert!(!profile.has_removals());
    assert!(!profile.is_additive_only());
    assert!(profile.is_ready());
}
