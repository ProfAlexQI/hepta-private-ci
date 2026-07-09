use super::*;

#[test]
fn store_snapshot_recall_context_memory_taxonomy_maps_sources_without_payloads() {
    let snapshot = StoreSnapshot {
        sessions: vec![],
        memories: vec![
            memory_record("memory-1", MemoryScope::LongTerm, "timeout retry guidance"),
            memory_record("memory-2", MemoryScope::Session, "session timeout summary"),
            memory_record("memory-3", MemoryScope::LongTerm, "timeout rollback note"),
            memory_record(
                "memory-4",
                MemoryScope::Session,
                "timeout summary follow-up",
            ),
            memory_record(
                "memory-5",
                MemoryScope::LongTerm,
                "[hepta-memory:tombstone] timeout hidden control",
            ),
        ],
        transcripts: vec![
            transcript_entry(
                "session-1",
                1,
                TranscriptEntryKind::Message,
                "start diagnosis",
            ),
            transcript_entry(
                "session-1",
                2,
                TranscriptEntryKind::Summary,
                "timeout retried successfully",
            ),
            transcript_entry(
                "session-2",
                1,
                TranscriptEntryKind::Message,
                "timeout in another session",
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

    let taxonomy = snapshot.recall_context_memory_taxonomy_report(&request);

    assert!(taxonomy.has_count_integrity());
    assert_eq!(
        taxonomy,
        ContextMemoryTaxonomyReport {
            buckets: vec![
                ContextMemoryTaxonomyBucket {
                    class: ContextMemoryTaxonomyClass::Semantic,
                    source_count: 1,
                    returned_count: 1,
                    available_count: 2,
                    omitted_count: 1,
                    provenance_span_count: 0,
                },
                ContextMemoryTaxonomyBucket {
                    class: ContextMemoryTaxonomyClass::Episodic,
                    source_count: 1,
                    returned_count: 0,
                    available_count: 2,
                    omitted_count: 2,
                    provenance_span_count: 0,
                },
                ContextMemoryTaxonomyBucket {
                    class: ContextMemoryTaxonomyClass::Control,
                    source_count: 1,
                    returned_count: 0,
                    available_count: 1,
                    omitted_count: 1,
                    provenance_span_count: 0,
                },
                ContextMemoryTaxonomyBucket {
                    class: ContextMemoryTaxonomyClass::Transcript,
                    source_count: 2,
                    returned_count: 2,
                    available_count: 3,
                    omitted_count: 1,
                    provenance_span_count: 1,
                },
            ],
        }
    );

    let json = serde_json::to_string(&taxonomy).expect("taxonomy should serialize");
    assert!(json.contains("semantic"));
    assert!(json.contains("episodic"));
    assert!(json.contains("control"));
    assert!(json.contains("transcript"));
    assert!(!json.contains("timeout retry guidance"));
    assert!(!json.contains("session timeout summary"));
    assert!(!json.contains("hepta-memory:tombstone"));
    assert!(!json.contains("memory-"));
}

#[tokio::test]
async fn store_recall_context_memory_taxonomy_matches_snapshot_helper() {
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
        .put(memory_record(
            "memory-3",
            MemoryScope::LongTerm,
            "[hepta-memory:conflict] timeout hidden control",
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
        .recall_context_memory_taxonomy_report(request.clone())
        .expect("context recall memory taxonomy should succeed");

    assert_eq!(
        from_store,
        snapshot.recall_context_memory_taxonomy_report(&request)
    );
    assert!(from_store.has_count_integrity());
    assert!(
        from_store
            .buckets
            .iter()
            .any(|bucket| bucket.class == ContextMemoryTaxonomyClass::Control
                && bucket.omitted_count == 1)
    );
}

#[test]
fn store_snapshot_context_memory_namespace_policy_is_payload_light() {
    let snapshot = StoreSnapshot {
        sessions: vec![],
        memories: vec![memory_record(
            "memory-namespace-payload",
            MemoryScope::LongTerm,
            "namespace policy must not include this memory payload",
        )],
        transcripts: vec![transcript_entry(
            "session-namespace",
            1,
            TranscriptEntryKind::Message,
            "namespace policy must not include this transcript payload",
        )],
    };

    let report = snapshot.context_memory_namespace_policy_report();

    assert!(report.has_policy_integrity());
    assert_eq!(report.namespace_count(), 6);
    assert_eq!(report.operator_approval_required_count(), 6);
    assert_eq!(report.shadow_wal_required_count(), 6);
    assert_eq!(report.readback_required_count(), 6);
    assert_eq!(report.canary_required_count(), 6);
    assert_eq!(report.production_write_count(), 0);
    assert_eq!(report.graph_write_count(), 0);
    assert!(report.blocks.iter().any(|block| block.namespace
        == ContextMemoryNamespace::Procedural
        && block.privacy_tier == ContextMemoryNamespacePrivacyTier::WorkspacePrivate
        && block.write_policy == ContextMemoryNamespaceWritePolicy::ShadowProposalOnly));

    let json = serde_json::to_string(&report).expect("namespace policy should serialize");
    assert!(json.contains("core"));
    assert!(json.contains("session"));
    assert!(json.contains("procedural"));
    assert!(json.contains("semantic"));
    assert!(json.contains("episodic"));
    assert!(json.contains("archival"));
    assert!(!json.contains("namespace policy must not include this memory payload"));
    assert!(!json.contains("namespace policy must not include this transcript payload"));
    assert!(!json.contains("memory-namespace-payload"));
    assert!(!json.contains("source_id"));
    assert!(!json.contains("memory_id"));
    assert!(!json.contains("\"production_write\":true"));
    assert!(!json.contains("\"graph_write\":true"));
}

#[tokio::test]
async fn store_context_memory_namespace_policy_matches_snapshot_helper() {
    let store = InMemoryStore::default();
    store
        .put(memory_record(
            "memory-namespace-payload",
            MemoryScope::LongTerm,
            "namespace policy must remain static and payload-light",
        ))
        .await
        .expect("put should succeed");
    store
        .append(transcript_entry(
            "session-namespace",
            1,
            TranscriptEntryKind::Message,
            "namespace policy should not inspect transcript payload",
        ))
        .await
        .expect("append should succeed");

    let snapshot = store.snapshot().expect("snapshot should load");
    let from_store = store
        .context_memory_namespace_policy_report()
        .expect("context memory namespace policy should succeed");

    assert_eq!(
        from_store,
        snapshot.context_memory_namespace_policy_report()
    );
    assert!(from_store.has_policy_integrity());
}

#[test]
fn store_snapshot_context_memory_write_chain_readiness_is_payload_light() {
    let snapshot = StoreSnapshot {
        sessions: vec![],
        memories: vec![memory_record(
            "memory-write-chain-payload",
            MemoryScope::LongTerm,
            "write-chain readiness must not include this memory payload",
        )],
        transcripts: vec![transcript_entry(
            "session-write-chain",
            1,
            TranscriptEntryKind::Message,
            "write-chain readiness must not include this transcript payload",
        )],
    };

    let report = snapshot.context_memory_write_chain_readiness_report();

    assert!(report.has_readiness_integrity());
    assert_eq!(report.namespace_count(), 6);
    assert_eq!(report.stage_required_count(), 6);
    assert_eq!(report.stage_pass_count(), 6);
    assert_eq!(report.propose_write_ready_count(), 6);
    assert_eq!(report.shadow_wal_ready_count(), 6);
    assert_eq!(report.readback_ready_count(), 6);
    assert_eq!(report.canary_ready_count(), 6);
    assert_eq!(report.production_write_count(), 0);
    assert_eq!(report.graph_write_count(), 0);

    let json = serde_json::to_string(&report).expect("write-chain report should serialize");
    assert!(json.contains("core"));
    assert!(json.contains("session"));
    assert!(json.contains("procedural"));
    assert!(json.contains("semantic"));
    assert!(json.contains("episodic"));
    assert!(json.contains("archival"));
    assert!(!json.contains("write-chain readiness must not include this memory payload"));
    assert!(!json.contains("write-chain readiness must not include this transcript payload"));
    assert!(!json.contains("memory-write-chain-payload"));
    assert!(!json.contains("source_id"));
    assert!(!json.contains("memory_id"));
    assert!(!json.contains("\"production_write\":true"));
    assert!(!json.contains("\"graph_write\":true"));
}

#[tokio::test]
async fn store_context_memory_write_chain_readiness_matches_snapshot_helper() {
    let store = InMemoryStore::default();
    store
        .put(memory_record(
            "memory-write-chain-payload",
            MemoryScope::LongTerm,
            "write-chain readiness should remain static and payload-light",
        ))
        .await
        .expect("put should succeed");
    store
        .append(transcript_entry(
            "session-write-chain",
            1,
            TranscriptEntryKind::Message,
            "write-chain readiness should not inspect transcript payload",
        ))
        .await
        .expect("append should succeed");

    let snapshot = store.snapshot().expect("snapshot should load");
    let from_store = store
        .context_memory_write_chain_readiness_report()
        .expect("context memory write-chain readiness should succeed");

    assert_eq!(
        from_store,
        snapshot.context_memory_write_chain_readiness_report()
    );
    assert!(from_store.has_readiness_integrity());
}

#[test]
fn store_snapshot_context_memory_write_chain_receipt_freshness_is_payload_light() {
    let snapshot = StoreSnapshot {
        sessions: vec![],
        memories: vec![memory_record(
            "memory-write-chain-receipt-payload",
            MemoryScope::LongTerm,
            "write-chain receipt freshness must not include this memory payload",
        )],
        transcripts: vec![transcript_entry(
            "session-write-chain-receipt",
            1,
            TranscriptEntryKind::Message,
            "write-chain receipt freshness must not include this transcript payload",
        )],
    };

    let report = snapshot.context_memory_write_chain_receipt_freshness_report();

    assert!(report.has_receipt_integrity());
    assert_eq!(report.namespace_count(), 6);
    assert_eq!(report.receipt_required_count(), 18);
    assert_eq!(report.receipt_projected_count(), 18);
    assert_eq!(report.receipt_digest_count(), 6);
    assert_eq!(report.freshness_pass_count(), 6);
    assert_eq!(report.replay_guard_pass_count(), 6);
    assert_eq!(report.stale_replay_rejected_count(), 6);
    assert_eq!(report.recorded_receipt_count(), 0);
    assert_eq!(report.persisted_receipt_count(), 0);

    let json = serde_json::to_string(&report).expect("receipt report should serialize");
    assert!(json.contains("receipt_digest"));
    assert!(json.contains("shadow_wal_receipt_projected"));
    assert!(!json.contains("write-chain receipt freshness must not include this memory payload"));
    assert!(
        !json.contains("write-chain receipt freshness must not include this transcript payload")
    );
    assert!(!json.contains("memory-write-chain-receipt-payload"));
    assert!(!json.contains("source_id"));
    assert!(!json.contains("memory_id"));
    assert!(!json.contains("\"recorded_receipt\":true"));
    assert!(!json.contains("\"persisted_receipt\":true"));
    assert!(!json.contains("\"production_write\":true"));
    assert!(!json.contains("\"graph_write\":true"));
}

#[tokio::test]
async fn store_context_memory_write_chain_receipt_freshness_matches_snapshot_helper() {
    let store = InMemoryStore::default();
    store
        .put(memory_record(
            "memory-write-chain-receipt-payload",
            MemoryScope::LongTerm,
            "write-chain receipt freshness should remain static and payload-light",
        ))
        .await
        .expect("put should succeed");
    store
        .append(transcript_entry(
            "session-write-chain-receipt",
            1,
            TranscriptEntryKind::Message,
            "write-chain receipt freshness should not inspect transcript payload",
        ))
        .await
        .expect("append should succeed");

    let snapshot = store.snapshot().expect("snapshot should load");
    let from_store = store
        .context_memory_write_chain_receipt_freshness_report()
        .expect("context memory write-chain receipt freshness should succeed");

    assert_eq!(
        from_store,
        snapshot.context_memory_write_chain_receipt_freshness_report()
    );
    assert!(from_store.has_receipt_integrity());
}
