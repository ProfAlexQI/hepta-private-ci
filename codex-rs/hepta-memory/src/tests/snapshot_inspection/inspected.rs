use super::*;

#[test]
fn inspected_store_snapshot_matches_snapshot_helpers() {
    let snapshot = StoreSnapshot {
        sessions: vec![session_record(
            "session-1",
            "Foundation",
            Some("inspect snapshot"),
        )],
        memories: vec![memory_record(
            "memory-1",
            MemoryScope::LongTerm,
            "snapshot contract payload",
        )],
        transcripts: vec![transcript_entry(
            "session-1",
            1,
            TranscriptEntryKind::Summary,
            "inspection summary",
        )],
    };

    let inspected = InspectedStoreSnapshot::from_snapshot(snapshot.clone());

    assert_eq!(inspected.snapshot, snapshot);
    assert_eq!(inspected.inspection, snapshot.inspection_bundle());
    assert_eq!(inspected.audit_report(), snapshot.audit_report());
    assert!(snapshot.inspection_matches(&inspected.inspection));
    assert!(inspected.inspection_matches_snapshot());
    assert_eq!(inspected.issue_count(), 0);
    assert!(inspected.is_clean());
}

#[test]
fn inspected_store_snapshot_roundtrips_through_json() {
    let inspected = InspectedStoreSnapshot::from_snapshot(StoreSnapshot {
        sessions: vec![session_record(
            "session-1",
            "Foundation",
            Some("roundtrip inspected snapshot"),
        )],
        memories: vec![memory_record(
            "memory-1",
            MemoryScope::Session,
            "roundtrip contract payload",
        )],
        transcripts: vec![transcript_entry(
            "session-1",
            1,
            TranscriptEntryKind::Message,
            "roundtrip transcript",
        )],
    });

    let json = serde_json::to_string(&inspected).expect("inspected snapshot should serialize");
    let parsed: InspectedStoreSnapshot =
        serde_json::from_str(&json).expect("inspected snapshot should deserialize");

    assert_eq!(parsed, inspected);
    assert_eq!(parsed.audit_report(), inspected.audit_report());
    assert!(parsed.inspection_matches_snapshot());
}

#[test]
fn inspected_store_snapshot_deserializes_without_inspection_field() {
    let canonical = InspectedStoreSnapshot::from_snapshot(StoreSnapshot {
        sessions: vec![session_record(
            "session-1",
            "Foundation",
            Some("backfill inspection"),
        )],
        memories: vec![memory_record(
            "memory-1",
            MemoryScope::LongTerm,
            "snapshot payload",
        )],
        transcripts: vec![transcript_entry(
            "session-1",
            1,
            TranscriptEntryKind::Message,
            "snapshot captured",
        )],
    });
    let mut json = serde_json::to_value(&canonical).expect("snapshot should serialize");
    json.as_object_mut()
        .expect("inspected snapshot should serialize as an object")
        .remove("inspection");

    let parsed: InspectedStoreSnapshot =
        serde_json::from_value(json).expect("legacy inspected snapshot should deserialize");

    assert_eq!(parsed, canonical);
    assert!(parsed.inspection_matches_snapshot());
    assert_eq!(parsed.audit_report(), parsed.snapshot.audit_report());
}
