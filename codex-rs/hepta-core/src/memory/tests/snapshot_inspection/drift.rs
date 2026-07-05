use super::*;

#[test]
fn snapshot_inspection_bundle_drift_impact_matches_report_helper() {
    let sessions = vec![SessionRecord {
        session_id: SessionId("session-1".into()),
        agent_id: AgentId("builder".into()),
        title: "Foundation lane".into(),
        created_at_unix_ms: 1,
        last_active_unix_ms: 2,
        last_user_intent_summary: Some("drift impact alignment".into()),
        archived_at_unix_ms: None,
    }];
    let memories = vec![MemoryRecord {
        id: "memory-1".into(),
        scope: MemoryScope::LongTerm,
        content: "snapshot payload".into(),
    }];
    let transcripts = vec![sample_transcript_entry(1, "snapshot captured")];
    let canonical =
        SnapshotInspectionBundle::from_records_and_entries(&sessions, &memories, &transcripts);
    let drifted = SnapshotInspectionBundle {
        memory_manifest: MemorySnapshotManifest::default(),
        ..canonical.clone()
    };

    assert_eq!(
        canonical.drift_impact_against_records(&sessions, &memories, &transcripts),
        canonical
            .drift_report(&sessions, &memories, &transcripts)
            .impact()
    );
    assert!(
        canonical
            .drift_impact_against_records(&sessions, &memories, &transcripts)
            .is_aligned()
    );

    let impact = drifted.drift_impact_against_records(&sessions, &memories, &transcripts);

    assert_eq!(
        impact,
        drifted
            .drift_report(&sessions, &memories, &transcripts)
            .impact()
    );
    assert_eq!(impact.mismatch_count, 1);
    assert_eq!(impact.memory_mismatch_count, 1);
    assert_eq!(impact.transcript_mismatch_count, 0);
    assert_eq!(impact.changed_domain_count(), 1);
    assert!(impact.touches_memory());
    assert!(!impact.touches_transcripts());
    assert!(!impact.is_aligned());
}

#[test]
fn snapshot_inspection_drift_report_is_empty_for_aligned_bundle() {
    let sessions = vec![SessionRecord {
        session_id: SessionId("session-1".into()),
        agent_id: AgentId("builder".into()),
        title: "Foundation lane".into(),
        created_at_unix_ms: 1,
        last_active_unix_ms: 2,
        last_user_intent_summary: Some("aligned inspection bundle".into()),
        archived_at_unix_ms: None,
    }];
    let memories = vec![MemoryRecord {
        id: "memory-1".into(),
        scope: MemoryScope::LongTerm,
        content: "inspection payload".into(),
    }];
    let transcripts = vec![sample_transcript_entry(1, "snapshot captured")];
    let bundle =
        SnapshotInspectionBundle::from_records_and_entries(&sessions, &memories, &transcripts);

    let drift = bundle.drift_report(&sessions, &memories, &transcripts);

    assert_eq!(drift, SnapshotInspectionDriftReport::default());
    assert_eq!(drift.mismatch_count(), 0);
    assert!(drift.is_aligned());
    assert!(!drift.mismatches(SnapshotInspectionSection::MemoryManifest));
}

#[test]
fn snapshot_inspection_drift_report_identifies_mismatched_sections() {
    let sessions = vec![SessionRecord {
        session_id: SessionId("session-1".into()),
        agent_id: AgentId("builder".into()),
        title: "Foundation lane".into(),
        created_at_unix_ms: 1,
        last_active_unix_ms: 2,
        last_user_intent_summary: Some("drift inspection bundle".into()),
        archived_at_unix_ms: None,
    }];
    let memories = vec![MemoryRecord {
        id: "memory-1".into(),
        scope: MemoryScope::Session,
        content: "snapshot payload".into(),
    }];
    let transcripts = vec![sample_transcript_entry(1, "snapshot captured")];
    let drifted = SnapshotInspectionBundle {
        memory_manifest: MemorySnapshotManifest::default(),
        memory_integrity: MemorySnapshotIntegrityReport::default(),
        transcript_manifest: TranscriptSnapshotManifest::default(),
        transcript_integrity: TranscriptSnapshotIntegrityReport::default(),
    };

    let drift = drifted.drift_report(&sessions, &memories, &transcripts);

    assert_eq!(
        drift.mismatched_sections,
        vec![
            SnapshotInspectionSection::MemoryManifest,
            SnapshotInspectionSection::TranscriptManifest,
        ]
    );
    assert_eq!(drift.mismatch_count(), 2);
    assert!(!drift.is_aligned());
    assert!(drift.mismatches(SnapshotInspectionSection::MemoryManifest));
    assert!(!drift.mismatches(SnapshotInspectionSection::MemoryIntegrity));
    assert!(drift.mismatches(SnapshotInspectionSection::TranscriptManifest));
    assert!(!drift.mismatches(SnapshotInspectionSection::TranscriptIntegrity));
}

#[test]
fn snapshot_inspection_drift_report_roundtrips_through_json() {
    let report = SnapshotInspectionDriftReport {
        mismatched_sections: vec![
            SnapshotInspectionSection::MemoryIntegrity,
            SnapshotInspectionSection::TranscriptIntegrity,
        ],
    };

    let json = serde_json::to_string(&report).expect("drift report should serialize");
    let parsed: SnapshotInspectionDriftReport =
        serde_json::from_str(&json).expect("drift report should deserialize");

    assert_eq!(parsed, report);
    assert_eq!(parsed.mismatch_count(), 2);
    assert!(!parsed.is_aligned());
}

#[test]
fn snapshot_inspection_drift_report_exposes_domain_level_counts_and_impact() {
    let report = SnapshotInspectionDriftReport {
        mismatched_sections: vec![
            SnapshotInspectionSection::MemoryManifest,
            SnapshotInspectionSection::MemoryIntegrity,
            SnapshotInspectionSection::TranscriptManifest,
        ],
    };

    assert_eq!(report.memory_mismatch_count(), 2);
    assert_eq!(report.transcript_mismatch_count(), 1);
    assert_eq!(report.changed_domain_count(), 2);
    assert!(report.touches_memory());
    assert!(report.touches_transcripts());

    assert_eq!(
        report.impact(),
        SnapshotInspectionDriftImpact {
            mismatch_count: 3,
            memory_mismatch_count: 2,
            transcript_mismatch_count: 1,
        }
    );
}

#[test]
fn snapshot_inspection_drift_impact_roundtrips_through_json() {
    let impact = SnapshotInspectionDriftImpact {
        mismatch_count: 2,
        memory_mismatch_count: 0,
        transcript_mismatch_count: 2,
    };

    let json = serde_json::to_string(&impact).expect("drift impact should serialize");
    let parsed: SnapshotInspectionDriftImpact =
        serde_json::from_str(&json).expect("drift impact should deserialize");

    assert_eq!(parsed, impact);
    assert_eq!(parsed.changed_domain_count(), 1);
    assert!(!parsed.touches_memory());
    assert!(parsed.touches_transcripts());
    assert!(!parsed.is_aligned());
}

#[test]
fn snapshot_inspection_drift_impact_deserializes_from_sparse_json() {
    let parsed: SnapshotInspectionDriftImpact =
        serde_json::from_str("{}").expect("sparse drift impact should deserialize with defaults");

    assert_eq!(parsed, SnapshotInspectionDriftImpact::default());
    assert_eq!(parsed.changed_domain_count(), 0);
    assert!(!parsed.touches_memory());
    assert!(!parsed.touches_transcripts());
    assert!(parsed.is_aligned());
}

#[test]
fn snapshot_inspection_drift_report_deserializes_from_sparse_json() {
    let parsed: SnapshotInspectionDriftReport =
        serde_json::from_str("{}").expect("sparse drift report should deserialize with defaults");

    assert_eq!(parsed, SnapshotInspectionDriftReport::default());
    assert_eq!(parsed.mismatch_count(), 0);
    assert!(parsed.is_aligned());
}
