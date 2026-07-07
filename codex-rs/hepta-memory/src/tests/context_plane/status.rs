use super::*;

#[test]
fn store_snapshot_context_plane_status_report_is_payload_light() {
    let snapshot = StoreSnapshot {
        sessions: vec![],
        memories: vec![
            memory_record("memory-1", MemoryScope::LongTerm, "timeout retry guidance"),
            memory_record("memory-2", MemoryScope::Session, "session timeout summary"),
        ],
        transcripts: vec![
            transcript_entry(
                "session-1",
                1,
                TranscriptEntryKind::Message,
                "timeout surfaced during tool run",
            ),
            transcript_entry(
                "session-1",
                2,
                TranscriptEntryKind::Summary,
                "timeout retried successfully",
            ),
        ],
    };
    let request = ContextRecallRequest {
        session_id: SessionId("session-1".into()),
        query_text: Some("timeout".into()),
        recent_window_limit: 1,
        transcript_limit: 1,
        memory_limit: 1,
        allow_cross_session: true,
    };

    let report = snapshot.context_plane_status_report(&request);

    assert!(report.has_status_integrity());
    assert_eq!(report.sections.len(), 12);
    assert_eq!(report.ready_section_count(), 8);
    assert_eq!(report.shadow_section_count(), 3);
    assert_eq!(report.disabled_section_count(), 1);
    assert_eq!(report.blocker_count(), 0);
    assert_eq!(
        report.section_status(ContextPlaneStatusSection::SourceRegistry),
        Some(ContextPlaneStatusKind::Ready)
    );
    assert_eq!(
        report.section_status(ContextPlaneStatusSection::AdaptiveBudgetAllocation),
        Some(ContextPlaneStatusKind::Shadow)
    );
    assert_eq!(
        report.section_status(ContextPlaneStatusSection::RecallQualityGate),
        Some(ContextPlaneStatusKind::Ready)
    );
    assert_eq!(
        report.section_status(ContextPlaneStatusSection::MemoryProviderBoundary),
        Some(ContextPlaneStatusKind::Shadow)
    );
    assert_eq!(
        report.section_status(ContextPlaneStatusSection::SourceAwareFrontDoor),
        Some(ContextPlaneStatusKind::Disabled)
    );
    let recall_entry = report
        .sections
        .iter()
        .find(|entry| entry.section == ContextPlaneStatusSection::RecallQualityGate)
        .expect("recall quality status row should exist");
    assert_eq!(recall_entry.recall_quality_blocking_reason_count, 0);
    assert!(recall_entry.recall_quality_blocking_reasons.is_empty());
    assert!(!report.production_write);
    assert!(!report.graph_write);
    assert!(!report.runtime_activation);
    assert!(!report.adaptive_allocator_runtime_activation);
    assert!(!report.source_aware_runtime_activation);
    assert!(!report.prompt_assembly_change);
    assert!(!report.operator_activation_allowed);

    let json = serde_json::to_string(&report).expect("context plane status should serialize");
    assert!(json.contains("source_registry"));
    assert!(json.contains("adaptive_budget_allocation"));
    assert!(json.contains("memory_taxonomy"));
    assert!(json.contains("memory_formation_receipts"));
    assert!(json.contains("memory_formation_queue"));
    assert!(json.contains("memory_temporal_facts"));
    assert!(json.contains("eval_harness_seed"));
    assert!(json.contains("adaptive_allocator_eval_shadow"));
    assert!(json.contains("recall_quality_gate"));
    assert!(json.contains("memory_provider_boundary"));
    assert!(json.contains("recall_quality_blocking_reason_count"));
    assert!(json.contains("recall_quality_blocking_reasons"));
    assert!(json.contains("source_aware_front_door"));
    assert!(!json.contains("timeout surfaced during tool run"));
    assert!(!json.contains("timeout retried successfully"));
    assert!(!json.contains("session-1"));
    assert!(!json.contains("memory-1"));
    assert!(!json.contains("source_id"));
    assert!(!json.contains("transcript_text"));
    assert!(!json.contains("memory_text"));
    assert!(!json.contains("prompt_text"));
    assert!(!json.contains("answer_text"));
    assert!(!json.contains("\"production_write\":true"));
    assert!(!json.contains("\"graph_write\":true"));
    assert!(!json.contains("\"runtime_activation\":true"));
    assert!(!json.contains("\"adaptive_allocator_runtime_activation\":true"));
    assert!(!json.contains("\"source_aware_runtime_activation\":true"));
    assert!(!json.contains("\"prompt_assembly_change\":true"));
    assert!(!json.contains("\"operator_activation_allowed\":true"));
}

#[tokio::test]
async fn store_context_plane_status_report_matches_snapshot_helper() {
    let store = InMemoryStore::default();
    store
        .put(memory_record(
            "memory-1",
            MemoryScope::LongTerm,
            "timeout retry guidance",
        ))
        .await
        .expect("put should succeed");
    store
        .put(memory_record(
            "memory-2",
            MemoryScope::Session,
            "session timeout summary",
        ))
        .await
        .expect("put should succeed");
    store
        .append(transcript_entry(
            "session-1",
            1,
            TranscriptEntryKind::Message,
            "timeout surfaced during tool run",
        ))
        .await
        .expect("append should succeed");
    store
        .append(transcript_entry(
            "session-1",
            2,
            TranscriptEntryKind::Summary,
            "timeout retried successfully",
        ))
        .await
        .expect("append should succeed");

    let request = ContextRecallRequest {
        session_id: SessionId("session-1".into()),
        query_text: Some("timeout".into()),
        recent_window_limit: 1,
        transcript_limit: 1,
        memory_limit: 1,
        allow_cross_session: true,
    };
    let snapshot = store.snapshot().expect("snapshot should load");
    let from_store = store
        .context_plane_status_report(request.clone())
        .expect("context plane status should succeed");

    assert_eq!(from_store, snapshot.context_plane_status_report(&request));
    assert!(from_store.has_status_integrity());
    assert_eq!(from_store.sections.len(), 12);
    assert_eq!(from_store.blocker_count(), 0);
    assert_eq!(
        from_store.section_status(ContextPlaneStatusSection::RecallQualityGate),
        Some(ContextPlaneStatusKind::Ready)
    );
    assert_eq!(
        from_store.section_status(ContextPlaneStatusSection::MemoryProviderBoundary),
        Some(ContextPlaneStatusKind::Shadow)
    );
    assert_eq!(
        from_store.section_status(ContextPlaneStatusSection::SourceAwareFrontDoor),
        Some(ContextPlaneStatusKind::Disabled)
    );
    let recall_entry = from_store
        .sections
        .iter()
        .find(|entry| entry.section == ContextPlaneStatusSection::RecallQualityGate)
        .expect("recall quality status row should exist");
    assert_eq!(recall_entry.recall_quality_blocking_reason_count, 0);
    assert!(recall_entry.recall_quality_blocking_reasons.is_empty());
    assert!(!from_store.production_write);
    assert!(!from_store.graph_write);
    assert!(!from_store.runtime_activation);
    assert!(!from_store.adaptive_allocator_runtime_activation);
    assert!(!from_store.source_aware_runtime_activation);
    assert!(!from_store.prompt_assembly_change);
    assert!(!from_store.operator_activation_allowed);
}
