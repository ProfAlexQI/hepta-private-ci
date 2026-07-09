use super::*;

#[test]
fn store_snapshot_recall_context_memory_temporal_facts_are_payload_light() {
    let snapshot = StoreSnapshot {
        sessions: vec![],
        memories: vec![memory_record(
            "memory-1",
            MemoryScope::LongTerm,
            "timeout retry guidance",
        )],
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

    let report = snapshot.recall_context_memory_temporal_fact_report(&request);

    assert!(report.has_temporal_fact_integrity());
    assert_eq!(report.facts.len(), 5);
    assert_eq!(
        report
            .facts
            .iter()
            .map(|fact| fact.fact_type)
            .collect::<Vec<_>>(),
        vec![
            ContextMemoryTemporalFactType::Attribute,
            ContextMemoryTemporalFactType::Preference,
            ContextMemoryTemporalFactType::TaskState,
            ContextMemoryTemporalFactType::Decision,
            ContextMemoryTemporalFactType::Summary,
        ]
    );
    assert!(report.facts.iter().all(|fact| {
        fact.provenance_span_count == 1
            && fact.valid_from_sequence == 2
            && fact.invalid_at_sequence.is_none()
            && fact.supersedes_fact_hash.is_none()
            && fact.privacy_class == "user_private"
            && fact.dry_run_only
            && !fact.production_write
            && fact.entity_hash.len() == 16
    }));

    let json = serde_json::to_string(&report).expect("temporal fact report should serialize");
    assert!(json.contains("attribute"));
    assert!(json.contains("task_state"));
    assert!(json.contains("summary"));
    assert!(!json.contains("timeout surfaced during tool run"));
    assert!(!json.contains("timeout retried successfully"));
    assert!(!json.contains("session-1"));
    assert!(!json.contains("memory-1"));
    assert!(!json.contains("fact_text"));
    assert!(!json.contains("transcript_text"));
    assert!(!json.contains("memory_text"));
    assert!(!json.contains("source_id"));
    assert!(!json.contains("memory_id"));
    assert!(!json.contains("query_text"));
    assert!(!json.contains("\"production_write\":true"));
}

#[test]
fn store_snapshot_recall_context_memory_temporal_fact_graph_is_payload_light() {
    let snapshot = StoreSnapshot {
        sessions: vec![],
        memories: vec![memory_record(
            "memory-1",
            MemoryScope::LongTerm,
            "timeout retry guidance",
        )],
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

    let graph = snapshot.recall_context_memory_temporal_fact_graph_report(&request);

    assert!(graph.has_graph_integrity());
    assert_eq!(graph.nodes.len(), 5);
    assert_eq!(graph.edges.len(), 10);
    assert_eq!(graph.provenance_edge_count(), 5);
    assert_eq!(graph.validity_window_edge_count(), 5);
    assert_eq!(graph.supersedes_edge_count(), 0);
    assert_eq!(graph.open_node_count(), 5);
    assert_eq!(graph.invalidated_node_count(), 0);
    assert!(!graph.production_write);
    assert!(!graph.graph_write);
    assert!(!graph.runtime_activation);
    assert!(!graph.prompt_assembly_change);
    assert!(graph.nodes.iter().all(|node| {
        node.fact_hash.len() == 16
            && node.provenance_span_count == 1
            && node.valid_from_sequence == 2
            && node.invalid_at_sequence.is_none()
            && node.dry_run_only
            && !node.production_write
            && !node.graph_write
    }));
    assert!(graph.edges.iter().all(|edge| {
        edge.edge_hash.len() == 16
            && edge.from_fact_hash.len() == 16
            && edge.provenance_span_count == 1
            && edge.valid_from_sequence == 2
            && edge.invalid_at_sequence.is_none()
            && edge.dry_run_only
            && !edge.production_write
            && !edge.graph_write
    }));

    let json = serde_json::to_string(&graph).expect("temporal fact graph should serialize");
    assert!(json.contains("validity_window"));
    assert!(json.contains("provenance"));
    assert!(!json.contains("entity_hash"));
    assert!(!json.contains("timeout surfaced during tool run"));
    assert!(!json.contains("timeout retried successfully"));
    assert!(!json.contains("session-1"));
    assert!(!json.contains("memory-1"));
    assert!(!json.contains("fact_text"));
    assert!(!json.contains("entity_text"));
    assert!(!json.contains("transcript_text"));
    assert!(!json.contains("memory_text"));
    assert!(!json.contains("source_id"));
    assert!(!json.contains("memory_id"));
    assert!(!json.contains("query_text"));
    assert!(!json.contains("\"production_write\":true"));
    assert!(!json.contains("\"graph_write\":true"));
    assert!(!json.contains("\"runtime_activation\":true"));
}

#[test]
fn store_snapshot_recall_context_memory_temporal_graph_shadow_store_is_payload_light() {
    let snapshot = StoreSnapshot {
        sessions: vec![],
        memories: vec![memory_record(
            "memory-1",
            MemoryScope::LongTerm,
            "timeout retry guidance",
        )],
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

    let store = snapshot.recall_context_memory_temporal_graph_shadow_store_report(&request);

    assert!(store.has_shadow_store_integrity());
    assert_eq!(store.node_count, 5);
    assert_eq!(store.edge_count, 10);
    assert_eq!(store.provenance_edge_count, 5);
    assert_eq!(store.validity_window_edge_count, 5);
    assert_eq!(store.supersedes_edge_count, 0);
    assert_eq!(store.open_node_count, 5);
    assert_eq!(store.invalidated_node_count, 0);
    assert_eq!(store.readiness_stage_projected_count(), 6);
    assert_eq!(store.store_digest.len(), 16);
    assert!(store.freshness_check_pass);
    assert!(store.replay_guard_pass);
    assert!(store.stale_replay_rejected);
    assert!(store.operator_approval_required);
    assert!(!store.operator_approval_recorded);
    assert_eq!(store.receipt_recorded_count(), 0);
    assert_eq!(store.receipt_persisted_count(), 0);
    assert_eq!(store.production_write_count(), 0);
    assert_eq!(store.graph_write_count(), 0);
    assert!(!store.production_write);
    assert!(!store.graph_write);
    assert!(!store.hot_path_write);
    assert!(!store.prompt_assembly_change);
    assert!(!store.runtime_activation);

    let json = serde_json::to_string(&store).expect("temporal graph shadow store should serialize");
    assert!(json.contains("store_digest"));
    assert!(json.contains("shadow_wal_projected"));
    assert!(json.contains("digest_freshness_projected"));
    assert!(!json.contains("timeout surfaced during tool run"));
    assert!(!json.contains("timeout retried successfully"));
    assert!(!json.contains("session-1"));
    assert!(!json.contains("memory-1"));
    assert!(!json.contains("entity_hash"));
    assert!(!json.contains("fact_text"));
    assert!(!json.contains("entity_text"));
    assert!(!json.contains("transcript_text"));
    assert!(!json.contains("memory_text"));
    assert!(!json.contains("source_id"));
    assert!(!json.contains("memory_id"));
    assert!(!json.contains("query_text"));
    assert!(!json.contains("\"production_route\":true"));
    assert!(!json.contains("\"production_write\":true"));
    assert!(!json.contains("\"graph_write\":true"));
    assert!(!json.contains("\"runtime_activation\":true"));
}

#[tokio::test]
async fn store_recall_context_memory_temporal_facts_match_snapshot_helper() {
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
        .recall_context_memory_temporal_fact_report(request.clone())
        .expect("context recall memory temporal facts should succeed");

    assert_eq!(
        from_store,
        snapshot.recall_context_memory_temporal_fact_report(&request)
    );
    assert!(from_store.has_temporal_fact_integrity());
    assert_eq!(from_store.facts.len(), 5);
    assert!(
        from_store
            .facts
            .iter()
            .all(|fact| fact.dry_run_only && !fact.production_write)
    );
}

#[tokio::test]
async fn store_recall_context_memory_temporal_fact_graph_matches_snapshot_helper() {
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
        .recall_context_memory_temporal_fact_graph_report(request.clone())
        .expect("context recall memory temporal fact graph should succeed");

    assert_eq!(
        from_store,
        snapshot.recall_context_memory_temporal_fact_graph_report(&request)
    );
    assert!(from_store.has_graph_integrity());
    assert_eq!(from_store.nodes.len(), 5);
    assert_eq!(from_store.edges.len(), 10);
    assert!(
        from_store
            .nodes
            .iter()
            .all(|node| node.dry_run_only && !node.production_write && !node.graph_write)
    );
    assert!(
        from_store
            .edges
            .iter()
            .all(|edge| edge.dry_run_only && !edge.production_write && !edge.graph_write)
    );
}

#[tokio::test]
async fn store_recall_context_memory_temporal_graph_shadow_store_matches_snapshot_helper() {
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
        .recall_context_memory_temporal_graph_shadow_store_report(request.clone())
        .expect("context recall memory temporal graph shadow store should succeed");

    assert_eq!(
        from_store,
        snapshot.recall_context_memory_temporal_graph_shadow_store_report(&request)
    );
    assert!(from_store.has_shadow_store_integrity());
    assert_eq!(from_store.readiness_stage_projected_count(), 6);
    assert_eq!(from_store.receipt_recorded_count(), 0);
    assert_eq!(from_store.receipt_persisted_count(), 0);
    assert_eq!(from_store.production_write_count(), 0);
    assert_eq!(from_store.graph_write_count(), 0);
}
