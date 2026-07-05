use super::*;

#[test]
fn store_snapshot_roundtrips_through_json() {
    let snapshot = StoreSnapshot {
        sessions: vec![session_record(
            "session-1",
            "Foundation",
            Some("audit memory"),
        )],
        memories: vec![memory_record(
            "memory-1",
            MemoryScope::LongTerm,
            "doctor snapshot integrity",
        )],
        transcripts: vec![transcript_entry(
            "session-1",
            1,
            TranscriptEntryKind::Message,
            "snapshot captured",
        )],
    };

    let json = serde_json::to_string(&snapshot).expect("snapshot should serialize");
    let parsed: StoreSnapshot = serde_json::from_str(&json).expect("snapshot should deserialize");

    assert_eq!(parsed, snapshot);
}

#[test]
fn store_snapshot_deserializes_without_transcripts_field() {
    let parsed: StoreSnapshot = serde_json::from_str(r#"{"sessions":[],"memories":[]}"#)
        .expect("legacy snapshot should deserialize");

    assert!(parsed.transcripts.is_empty());
}

#[test]
fn store_snapshot_transcript_helpers_match_clean_snapshot() {
    let snapshot = StoreSnapshot {
        sessions: vec![],
        memories: vec![],
        transcripts: vec![transcript_entry(
            "session-1",
            1,
            TranscriptEntryKind::Summary,
            "clean transcript summary",
        )],
    };

    let stats = snapshot.transcript_stats();
    let manifest = snapshot.transcript_manifest();
    let report = snapshot.transcript_integrity_report();

    assert_eq!(stats.total_entry_count, 1);
    assert_eq!(manifest.stats, stats);
    assert_eq!(manifest.entries.len(), 1);
    assert_eq!(manifest.entries[0].entry_id, "session-1-1");
    assert_eq!(report, TranscriptSnapshotIntegrityReport::default());
    assert!(report.is_clean());
}
