use super::*;

#[test]
fn transcript_snapshot_manifest_sorts_entries_and_tracks_sizes() {
    let manifest = TranscriptSnapshotManifest::from_entries(&[
        TranscriptEntry {
            entry_id: "entry-b".into(),
            session_id: SessionId("session-z".into()),
            sequence: 5,
            kind: TranscriptEntryKind::ToolResult,
            role: Some(MessageRole::Tool),
            content: "tool result payload".into(),
            created_at_unix_ms: 200,
            tool_name: Some("write".into()),
            correlation_id: None,
            summary_of_range: None,
        },
        TranscriptEntry {
            entry_id: "entry-a".into(),
            session_id: SessionId("session-a".into()),
            sequence: 2,
            kind: TranscriptEntryKind::Message,
            role: Some(MessageRole::User),
            content: "hello".into(),
            created_at_unix_ms: 100,
            tool_name: None,
            correlation_id: None,
            summary_of_range: None,
        },
        TranscriptEntry {
            entry_id: "entry-c".into(),
            session_id: SessionId("session-a".into()),
            sequence: 1,
            kind: TranscriptEntryKind::Event,
            role: None,
            content: "created".into(),
            created_at_unix_ms: 99,
            tool_name: None,
            correlation_id: None,
            summary_of_range: None,
        },
    ]);

    assert_eq!(manifest.stats.total_entry_count, 3);
    assert_eq!(manifest.stats.session_count, 2);
    assert_eq!(manifest.entries.len(), 3);
    assert_eq!(manifest.entries[0].entry_id, "entry-c");
    assert_eq!(manifest.entries[0].session_id.0, "session-a");
    assert_eq!(manifest.entries[0].sequence, 1);
    assert_eq!(manifest.entries[0].content_bytes, "created".len());
    assert_eq!(manifest.entries[1].entry_id, "entry-a");
    assert_eq!(manifest.entries[2].entry_id, "entry-b");
    assert!(!manifest.is_empty());
}

#[test]
fn transcript_snapshot_manifest_roundtrips_through_json() {
    let manifest =
        TranscriptSnapshotManifest::from_entries(&[sample_transcript_entry(1, "manifest payload")]);

    let json = serde_json::to_string(&manifest).expect("manifest should serialize");
    let parsed: TranscriptSnapshotManifest =
        serde_json::from_str(&json).expect("manifest should deserialize");

    assert_eq!(parsed, manifest);
}

#[test]
fn transcript_snapshot_manifest_deserializes_from_sparse_json() {
    let parsed: TranscriptSnapshotManifest =
        serde_json::from_str("{}").expect("sparse manifest should deserialize with defaults");

    assert_eq!(parsed, TranscriptSnapshotManifest::default());
    assert!(parsed.is_empty());
}
