use super::*;

#[test]
fn inspected_store_snapshot_normalized_rebuilds_drifted_inspection() {
    let snapshot = StoreSnapshot {
        sessions: vec![session_record(
            "session-1",
            "Foundation",
            Some("normalize inspection bundle"),
        )],
        memories: vec![memory_record(
            "memory-1",
            MemoryScope::LongTerm,
            "snapshot payload",
        )],
        transcripts: vec![transcript_entry(
            "session-1",
            1,
            TranscriptEntryKind::Summary,
            "summary payload",
        )],
    };
    let canonical = InspectedStoreSnapshot::from_snapshot(snapshot.clone());
    let drifted = InspectedStoreSnapshot {
        snapshot,
        inspection: SnapshotInspectionBundle::default(),
    };

    assert!(!drifted.inspection_matches_snapshot());
    assert_eq!(drifted.audit_report(), SnapshotAuditReport::default());

    let normalized = drifted.normalized();

    assert_eq!(normalized, canonical);
    assert!(normalized.inspection_matches_snapshot());
}

#[test]
fn store_snapshot_inspection_drift_report_tracks_section_level_drift() {
    let snapshot = StoreSnapshot {
        sessions: vec![session_record(
            "session-1",
            "Foundation",
            Some("track inspection drift"),
        )],
        memories: vec![memory_record(
            "memory-1",
            MemoryScope::LongTerm,
            "snapshot payload",
        )],
        transcripts: vec![transcript_entry(
            "session-1",
            1,
            TranscriptEntryKind::Summary,
            "summary payload",
        )],
    };

    let drift = snapshot.inspection_drift_report(&SnapshotInspectionBundle::default());

    assert_eq!(
        drift.mismatched_sections,
        vec![
            SnapshotInspectionSection::MemoryManifest,
            SnapshotInspectionSection::TranscriptManifest,
        ]
    );
    assert_eq!(drift.mismatch_count(), 2);
    assert!(!drift.is_aligned());
}

#[test]
fn inspected_store_snapshot_inspection_drift_report_matches_alignment_state() {
    let snapshot = StoreSnapshot {
        sessions: vec![session_record(
            "session-1",
            "Foundation",
            Some("inspect drift state"),
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

    assert!(aligned.inspection_drift_report().is_aligned());
    assert_eq!(aligned.inspection_drift_report().mismatch_count(), 0);

    let drift = drifted.inspection_drift_report();
    assert!(!drift.is_aligned());
    assert_eq!(drift.mismatch_count(), 2);
    assert!(drift.mismatches(SnapshotInspectionSection::MemoryManifest));
    assert!(drift.mismatches(SnapshotInspectionSection::TranscriptManifest));
    assert!(!drift.mismatches(SnapshotInspectionSection::MemoryIntegrity));
    assert!(!drift.mismatches(SnapshotInspectionSection::TranscriptIntegrity));
}

#[test]
fn store_snapshot_inspection_drift_impact_collapses_sections_by_domain() {
    let snapshot = StoreSnapshot {
        sessions: vec![session_record(
            "session-1",
            "Foundation",
            Some("collapse drift domains"),
        )],
        memories: vec![memory_record(
            "memory-1",
            MemoryScope::LongTerm,
            "snapshot payload",
        )],
        transcripts: vec![transcript_entry(
            "session-1",
            1,
            TranscriptEntryKind::Summary,
            "summary payload",
        )],
    };

    let impact = snapshot.inspection_drift_impact(&SnapshotInspectionBundle::default());

    assert_eq!(
        impact,
        SnapshotInspectionDriftImpact {
            mismatch_count: 2,
            memory_mismatch_count: 1,
            transcript_mismatch_count: 1,
        }
    );
    assert_eq!(impact.changed_domain_count(), 2);
    assert!(impact.touches_memory());
    assert!(impact.touches_transcripts());
    assert!(!impact.is_aligned());
}

#[test]
fn inspected_store_snapshot_inspection_drift_impact_matches_report_impact() {
    let snapshot = StoreSnapshot {
        sessions: vec![session_record(
            "session-1",
            "Foundation",
            Some("impact alignment"),
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
        aligned.inspection_drift_impact(),
        aligned.inspection_drift_report().impact()
    );
    assert!(aligned.inspection_drift_impact().is_aligned());

    let impact = drifted.inspection_drift_impact();
    assert_eq!(impact, drifted.inspection_drift_report().impact());
    assert_eq!(impact.mismatch_count, 2);
    assert_eq!(impact.changed_domain_count(), 2);
    assert!(impact.touches_memory());
    assert!(impact.touches_transcripts());
}

#[test]
fn store_snapshot_inspection_health_combines_issue_summary_and_drift_impact() {
    let snapshot = StoreSnapshot {
        sessions: vec![session_record(
            "session-1",
            "Foundation",
            Some("health summary"),
        )],
        memories: vec![memory_record("memory-1", MemoryScope::LongTerm, " ")],
        transcripts: vec![transcript_entry(
            "session-1",
            1,
            TranscriptEntryKind::Message,
            " ",
        )],
    };

    let canonical = snapshot.inspection_bundle();
    let inspection = SnapshotInspectionBundle {
        memory_manifest: MemorySnapshotManifest::default(),
        memory_integrity: canonical.memory_integrity,
        transcript_manifest: TranscriptSnapshotManifest::default(),
        transcript_integrity: canonical.transcript_integrity,
    };

    let health = snapshot.inspection_health(&inspection);

    assert_eq!(
        health,
        SnapshotInspectionHealth {
            issue_summary: inspection.issue_summary(),
            drift_impact: snapshot.inspection_drift_impact(&inspection),
        }
    );
    assert_eq!(health.issue_count(), 2);
    assert_eq!(health.mismatch_count(), 2);
    assert_eq!(health.changed_domain_count(), 2);
    assert!(health.touches_memory());
    assert!(health.touches_transcripts());
    assert!(health.has_issues());
    assert!(health.has_drift());
    assert!(!health.inspection_aligned());
    assert!(!health.is_clean());
    assert!(!health.is_ready());
}

#[test]
fn store_snapshot_inspection_drift_helpers_match_snapshot_helpers() {
    let store = InMemoryStore::default();
    store
        .restore(StoreSnapshot {
            sessions: vec![session_record(
                "session-1",
                "Foundation",
                Some("store drift helper alignment"),
            )],
            memories: vec![memory_record(
                "memory-1",
                MemoryScope::LongTerm,
                "snapshot payload",
            )],
            transcripts: vec![transcript_entry(
                "session-1",
                1,
                TranscriptEntryKind::Summary,
                "clean transcript summary",
            )],
        })
        .expect("restore should succeed");

    let snapshot = store.snapshot().expect("snapshot should load");
    let canonical = snapshot.inspection_bundle();
    let drifted = SnapshotInspectionBundle {
        transcript_manifest: TranscriptSnapshotManifest::default(),
        ..canonical.clone()
    };

    assert!(
        store
            .snapshot_inspection_matches(&canonical)
            .expect("canonical inspection match should succeed")
    );
    assert!(snapshot.inspection_matches(&canonical));
    assert!(
        !store
            .snapshot_inspection_matches(&drifted)
            .expect("drifted inspection match should succeed")
    );
    assert_eq!(
        store
            .snapshot_inspection_drift_report(&drifted)
            .expect("drift report should succeed"),
        snapshot.inspection_drift_report(&drifted)
    );
    assert_eq!(
        store
            .snapshot_inspection_drift_impact(&drifted)
            .expect("drift impact should succeed"),
        snapshot.inspection_drift_impact(&drifted)
    );
    assert_eq!(
        store
            .snapshot_inspection_health(&drifted)
            .expect("inspection health should succeed"),
        snapshot.inspection_health(&drifted)
    );
    assert_eq!(
        store
            .snapshot_inspection_drift_impact(&canonical)
            .expect("canonical drift impact should succeed"),
        SnapshotInspectionDriftImpact::default()
    );
    assert!(
        store
            .snapshot_inspection_health(&canonical)
            .expect("canonical inspection health should succeed")
            .is_ready()
    );
}
