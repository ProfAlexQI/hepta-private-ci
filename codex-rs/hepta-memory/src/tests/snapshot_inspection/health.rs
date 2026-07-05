use super::*;

#[test]
fn inspected_store_snapshot_inspection_health_uses_embedded_bundle() {
    let snapshot = StoreSnapshot {
        sessions: vec![session_record(
            "session-1",
            "Foundation",
            Some("embedded inspection health"),
        )],
        memories: vec![memory_record(
            "memory-1",
            MemoryScope::Session,
            "snapshot payload",
        )],
        transcripts: vec![transcript_entry(
            "session-1",
            1,
            TranscriptEntryKind::Message,
            "snapshot captured",
        )],
    };
    let aligned = InspectedStoreSnapshot::from_snapshot(snapshot.clone());
    let drifted = InspectedStoreSnapshot {
        snapshot,
        inspection: SnapshotInspectionBundle::default(),
    };

    assert_eq!(
        aligned.inspection_health(),
        aligned.snapshot.inspection_health(&aligned.inspection)
    );
    assert!(aligned.inspection_health().is_ready());

    let drifted_health = drifted.inspection_health();
    assert_eq!(
        drifted_health,
        drifted.snapshot.inspection_health(&drifted.inspection)
    );
    assert_eq!(drifted_health.issue_count(), 0);
    assert_eq!(drifted_health.mismatch_count(), 2);
    assert!(drifted_health.has_drift());
    assert!(!drifted_health.is_ready());
}
