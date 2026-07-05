use super::*;

#[test]
fn snapshot_inspection_health_is_ready_when_bundle_is_clean_and_aligned() {
    let sessions = vec![SessionRecord {
        session_id: SessionId("session-1".into()),
        agent_id: AgentId("builder".into()),
        title: "Foundation lane".into(),
        created_at_unix_ms: 1,
        last_active_unix_ms: 2,
        last_user_intent_summary: Some("inspection health ready".into()),
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

    let health = bundle.health_against_records(&sessions, &memories, &transcripts);

    assert_eq!(health.issue_summary, SnapshotIssueSummary::default());
    assert_eq!(
        health.drift_impact,
        SnapshotInspectionDriftImpact::default()
    );
    assert_eq!(health.issue_count(), 0);
    assert_eq!(health.mismatch_count(), 0);
    assert_eq!(health.changed_domain_count(), 0);
    assert!(!health.touches_memory());
    assert!(!health.touches_transcripts());
    assert!(!health.has_issues());
    assert!(!health.has_drift());
    assert!(health.inspection_aligned());
    assert!(health.is_clean());
    assert!(health.is_ready());
}

#[test]
fn snapshot_inspection_health_tracks_issue_domains_even_without_drift() {
    let sessions = vec![SessionRecord {
        session_id: SessionId("session-1".into()),
        agent_id: AgentId("builder".into()),
        title: "Foundation lane".into(),
        created_at_unix_ms: 1,
        last_active_unix_ms: 2,
        last_user_intent_summary: Some("inspection health issues".into()),
        archived_at_unix_ms: None,
    }];
    let memories = vec![MemoryRecord {
        id: "memory-1".into(),
        scope: MemoryScope::LongTerm,
        content: " ".into(),
    }];
    let transcripts = vec![TranscriptEntry {
        entry_id: "entry-1".into(),
        session_id: SessionId("session-1".into()),
        sequence: 1,
        kind: TranscriptEntryKind::Message,
        role: Some(MessageRole::Assistant),
        content: " ".into(),
        created_at_unix_ms: 3,
        tool_name: None,
        correlation_id: None,
        summary_of_range: None,
    }];
    let bundle =
        SnapshotInspectionBundle::from_records_and_entries(&sessions, &memories, &transcripts);

    let health = bundle.health_against_records(&sessions, &memories, &transcripts);

    assert_eq!(health.issue_count(), 2);
    assert_eq!(health.mismatch_count(), 0);
    assert_eq!(health.changed_domain_count(), 2);
    assert!(health.touches_memory());
    assert!(health.touches_transcripts());
    assert!(health.has_issues());
    assert!(!health.has_drift());
    assert!(health.inspection_aligned());
    assert!(!health.is_clean());
    assert!(!health.is_ready());
}

#[test]
fn snapshot_inspection_health_tracks_drift_domains_even_without_issues() {
    let sessions = vec![SessionRecord {
        session_id: SessionId("session-1".into()),
        agent_id: AgentId("builder".into()),
        title: "Foundation lane".into(),
        created_at_unix_ms: 1,
        last_active_unix_ms: 2,
        last_user_intent_summary: Some("inspection health drift".into()),
        archived_at_unix_ms: None,
    }];
    let memories = vec![MemoryRecord {
        id: "memory-1".into(),
        scope: MemoryScope::Session,
        content: "snapshot payload".into(),
    }];
    let transcripts = vec![sample_transcript_entry(1, "snapshot captured")];

    let health = SnapshotInspectionBundle::default().health_against_records(
        &sessions,
        &memories,
        &transcripts,
    );

    assert_eq!(health.issue_count(), 0);
    assert_eq!(health.mismatch_count(), 2);
    assert_eq!(health.changed_domain_count(), 2);
    assert!(health.touches_memory());
    assert!(health.touches_transcripts());
    assert!(!health.has_issues());
    assert!(health.has_drift());
    assert!(!health.inspection_aligned());
    assert!(health.is_clean());
    assert!(!health.is_ready());
}

#[test]
fn snapshot_inspection_health_roundtrips_through_json() {
    let health = SnapshotInspectionHealth {
        issue_summary: SnapshotIssueSummary {
            memory_issue_count: 1,
            transcript_issue_count: 0,
            total_issue_count: 1,
            issue_domain_count: 1,
        },
        drift_impact: SnapshotInspectionDriftImpact {
            mismatch_count: 2,
            memory_mismatch_count: 1,
            transcript_mismatch_count: 1,
        },
    };

    let json = serde_json::to_string(&health).expect("inspection health should serialize");
    let parsed: SnapshotInspectionHealth =
        serde_json::from_str(&json).expect("inspection health should deserialize");

    assert_eq!(parsed, health);
    assert_eq!(parsed.issue_count(), 1);
    assert_eq!(parsed.mismatch_count(), 2);
    assert_eq!(parsed.changed_domain_count(), 2);
    assert!(parsed.touches_memory());
    assert!(parsed.touches_transcripts());
    assert!(parsed.has_issues());
    assert!(parsed.has_drift());
    assert!(!parsed.inspection_aligned());
    assert!(!parsed.is_clean());
    assert!(!parsed.is_ready());
}

#[test]
fn snapshot_inspection_drift_impact_and_health_constructors_match_helpers() {
    let sessions = vec![SessionRecord {
        session_id: SessionId("session-1".into()),
        agent_id: AgentId("builder".into()),
        title: "Foundation".into(),
        created_at_unix_ms: 1,
        last_active_unix_ms: 2,
        last_user_intent_summary: Some("verify inspection health".into()),
        archived_at_unix_ms: None,
    }];
    let memories = vec![MemoryRecord {
        id: "memory-1".into(),
        scope: MemoryScope::LongTerm,
        content: "snapshot payload".into(),
    }];
    let transcripts = vec![sample_transcript_entry(1, "snapshot captured")];
    let inspection =
        SnapshotInspectionBundle::from_records_and_entries(&sessions, &memories, &transcripts);
    let report = SnapshotInspectionDriftReport {
        mismatched_sections: vec![
            SnapshotInspectionSection::MemoryManifest,
            SnapshotInspectionSection::TranscriptManifest,
            SnapshotInspectionSection::TranscriptIntegrity,
        ],
    };

    let impact = SnapshotInspectionDriftImpact::from_report(&report);
    let health = SnapshotInspectionHealth::from_bundle_and_records(
        &inspection,
        &sessions,
        &memories,
        &transcripts,
    );

    assert_eq!(
        impact,
        SnapshotInspectionDriftImpact {
            mismatch_count: 3,
            memory_mismatch_count: 1,
            transcript_mismatch_count: 2,
        }
    );
    assert_eq!(impact.changed_domain_count(), 2);
    assert!(impact.touches_memory());
    assert!(impact.touches_transcripts());
    assert!(!impact.is_aligned());

    assert_eq!(
        health,
        inspection.health_against_records(&sessions, &memories, &transcripts)
    );
    assert_eq!(health.issue_count(), 0);
    assert_eq!(health.mismatch_count(), 0);
    assert!(health.inspection_aligned());
    assert!(health.is_clean());
    assert!(health.is_ready());
}
