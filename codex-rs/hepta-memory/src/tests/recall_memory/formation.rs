use super::*;

#[test]
fn store_snapshot_recall_context_memory_formation_receipts_are_payload_light() {
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

    let report = snapshot.recall_context_memory_formation_receipt_report(&request);

    assert!(report.has_receipt_integrity());
    assert_eq!(report.receipts.len(), 5);
    assert_eq!(
        report
            .receipts
            .iter()
            .map(|receipt| receipt.candidate_type)
            .collect::<Vec<_>>(),
        vec![
            ContextMemoryFormationCandidateType::Fact,
            ContextMemoryFormationCandidateType::Task,
            ContextMemoryFormationCandidateType::Preference,
            ContextMemoryFormationCandidateType::Decision,
            ContextMemoryFormationCandidateType::Summary,
        ]
    );
    assert!(report.receipts.iter().all(|receipt| {
        receipt.transcript_span_count == 1
            && receipt.provenance_span_count == 1
            && receipt.privacy_class == "user_private"
            && receipt.queued_for_background
            && !receipt.production_write
            && receipt.idempotency_key_hash.len() == 16
    }));

    let json = serde_json::to_string(&report).expect("receipt report should serialize");
    assert!(json.contains("fact"));
    assert!(json.contains("summary"));
    assert!(!json.contains("timeout surfaced during tool run"));
    assert!(!json.contains("timeout retried successfully"));
    assert!(!json.contains("session-1"));
    assert!(!json.contains("memory-1"));
    assert!(!json.contains("source_id"));
    assert!(!json.contains("memory_id"));
    assert!(!json.contains("query_text"));
    assert!(!json.contains("\"production_write\":true"));
}

#[test]
fn store_snapshot_recall_context_memory_formation_queue_is_payload_light() {
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

    let report = snapshot.recall_context_memory_formation_queue_report(&request);

    assert!(report.has_queue_integrity());
    assert_eq!(report.items.len(), 5);
    assert_eq!(report.queued_count(), 5);
    assert_eq!(report.revocable_count(), 5);
    assert_eq!(report.operator_review_required_count(), 5);
    assert!(report.items.iter().all(|item| {
        item.transcript_span_count == 1
            && item.provenance_span_count == 1
            && item.privacy_class == "user_private"
            && item.queued_for_background
            && item.dry_run_only
            && item.idempotency_enforced
            && item.can_revoke_before_commit
            && !item.production_write
            && !item.graph_write
            && !item.hot_path_write
            && item.idempotency_key_hash.len() == 16
            && item.source_receipt_hash.len() == 16
            && item.revocation_key_hash.len() == 16
    }));
    assert!(!report.production_write);
    assert!(!report.graph_write);
    assert!(!report.runtime_activation);
    assert!(!report.hot_path_write);

    let json = serde_json::to_string(&report).expect("queue report should serialize");
    assert!(json.contains("operator_review_required"));
    assert!(json.contains("source_receipt_hash"));
    assert!(json.contains("revocation_key_hash"));
    assert!(json.contains("can_revoke_before_commit"));
    assert!(!json.contains("timeout surfaced during tool run"));
    assert!(!json.contains("timeout retried successfully"));
    assert!(!json.contains("session-1"));
    assert!(!json.contains("memory-1"));
    assert!(!json.contains("source_id"));
    assert!(!json.contains("memory_id"));
    assert!(!json.contains("candidate_text"));
    assert!(!json.contains("transcript_text"));
    assert!(!json.contains("raw_idempotency_key"));
    assert!(!json.contains("\"production_write\":true"));
    assert!(!json.contains("\"graph_write\":true"));
    assert!(!json.contains("\"runtime_activation\":true"));
    assert!(!json.contains("\"hot_path_write\":true"));
}

#[tokio::test]
async fn store_recall_context_memory_formation_receipts_match_snapshot_helper() {
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
        .recall_context_memory_formation_receipt_report(request.clone())
        .expect("context recall memory formation receipts should succeed");

    assert_eq!(
        from_store,
        snapshot.recall_context_memory_formation_receipt_report(&request)
    );
    assert!(from_store.has_receipt_integrity());
    assert_eq!(from_store.receipts.len(), 5);
    assert!(
        from_store
            .receipts
            .iter()
            .all(|receipt| receipt.queued_for_background && !receipt.production_write)
    );
}

#[tokio::test]
async fn store_recall_context_memory_formation_queue_matches_snapshot_helper() {
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
        .recall_context_memory_formation_queue_report(request.clone())
        .expect("context recall memory formation queue should succeed");

    assert_eq!(
        from_store,
        snapshot.recall_context_memory_formation_queue_report(&request)
    );
    assert!(from_store.has_queue_integrity());
    assert_eq!(from_store.items.len(), 5);
    assert!(from_store.items.iter().all(|item| {
        item.queued_for_background
            && item.dry_run_only
            && item.idempotency_enforced
            && item.can_revoke_before_commit
            && !item.production_write
            && !item.graph_write
            && !item.hot_path_write
    }));
}
