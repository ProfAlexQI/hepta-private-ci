use super::*;

#[test]
fn inspected_store_snapshot_restore_helpers_delegate_to_snapshot_payload() {
    let current = StoreSnapshot {
        sessions: vec![session_record(
            "session-1",
            "Current foundation",
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
    let inspected = InspectedStoreSnapshot::from_snapshot(StoreSnapshot {
        sessions: vec![session_record(
            "session-1",
            "Updated foundation",
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
    });

    assert_eq!(
        inspected.restore_preview_against(&current),
        inspected.snapshot.restore_preview_against(&current)
    );
    assert_eq!(
        inspected.restore_impact_against(&current),
        inspected.snapshot.restore_impact_against(&current)
    );
    assert_eq!(
        inspected.restore_readiness_against(&current),
        inspected.snapshot.restore_readiness_against(&current)
    );
    assert_eq!(
        inspected.restore_safety_against(&current),
        inspected.snapshot.restore_safety_against(&current)
    );
    assert_eq!(
        inspected.restore_mutation_profile_against(&current),
        inspected
            .snapshot
            .restore_mutation_profile_against(&current)
    );
    assert_eq!(
        inspected.restore_domain_impacts_against(&current),
        inspected.snapshot.restore_domain_impacts_against(&current)
    );
    assert_eq!(
        inspected.restore_changed_domains_against(&current),
        inspected.snapshot.restore_changed_domains_against(&current)
    );
}

#[test]
fn inspected_store_snapshot_restore_helpers_ignore_drifted_inspection_state() {
    let current = StoreSnapshot {
        sessions: vec![session_record(
            "session-1",
            "Current foundation",
            Some("current"),
        )],
        memories: vec![memory_record(
            "memory-1",
            MemoryScope::Session,
            "current payload",
        )],
        transcripts: vec![transcript_entry(
            "session-1",
            1,
            TranscriptEntryKind::Message,
            "current message",
        )],
    };
    let incoming = StoreSnapshot {
        sessions: vec![session_record(
            "session-2",
            "Added foundation",
            Some("incoming"),
        )],
        memories: vec![memory_record(
            "memory-1",
            MemoryScope::LongTerm,
            "updated payload",
        )],
        transcripts: vec![transcript_entry(
            "session-2",
            1,
            TranscriptEntryKind::Summary,
            "added summary",
        )],
    };
    let drifted = InspectedStoreSnapshot {
        snapshot: incoming.clone(),
        inspection: SnapshotInspectionBundle::default(),
    };

    assert!(!drifted.inspection_matches_snapshot());
    assert_eq!(
        drifted.restore_preview_against(&current),
        incoming.restore_preview_against(&current)
    );
    assert_eq!(
        drifted.restore_impact_against(&current),
        incoming.restore_impact_against(&current)
    );
    assert_eq!(
        drifted.restore_readiness_against(&current),
        incoming.restore_readiness_against(&current)
    );
    assert_eq!(
        drifted.restore_safety_against(&current),
        incoming.restore_safety_against(&current)
    );
    assert_eq!(
        drifted.restore_mutation_profile_against(&current),
        incoming.restore_mutation_profile_against(&current)
    );
    assert_eq!(
        drifted.restore_domain_impacts_against(&current),
        incoming.restore_domain_impacts_against(&current)
    );
    assert_eq!(
        drifted.restore_changed_domains_against(&current),
        incoming.restore_changed_domains_against(&current)
    );
}
