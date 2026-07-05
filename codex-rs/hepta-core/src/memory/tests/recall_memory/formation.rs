use super::*;

#[test]
fn context_recall_memory_formation_receipts_are_payload_light_and_non_writing() {
    let inspection = ContextRecallInspection {
        report: ContextRecallReport {
            request: ContextRecallRequest {
                session_id: SessionId("session-42".into()),
                query_text: Some("timeout".into()),
                recent_window_limit: 2,
                transcript_limit: 1,
                memory_limit: 2,
                allow_cross_session: true,
            },
            source_counts: ContextRecallSourceCounts {
                recent_entry_count: 2,
                transcript_hit_count: 1,
                durable_memory_hit_count: 1,
                summary_hit_count: 1,
            },
            truncated: true,
        },
        availability: ContextRecallAvailability {
            total_recent_entry_count: 4,
            total_transcript_match_count: 3,
            total_memory_match_count: 5,
        },
        source_transcript_spans: vec![
            TranscriptSpanRef {
                session_id: SessionId("session-42".into()),
                range: TranscriptRange {
                    start_sequence: 9,
                    end_sequence: 10,
                },
                reason: Some("recent_window".into()),
            },
            TranscriptSpanRef {
                session_id: SessionId("session-42".into()),
                range: TranscriptRange {
                    start_sequence: 8,
                    end_sequence: 8,
                },
                reason: Some("query_match".into()),
            },
        ],
    };

    let receipts = inspection.memory_formation_receipt_report();

    assert!(receipts.has_receipt_integrity());
    assert_eq!(
        receipts,
        ContextMemoryFormationReceiptReport {
            receipts: vec![
                ContextMemoryFormationReceipt {
                    candidate_type: ContextMemoryFormationCandidateType::Fact,
                    transcript_span_count: 2,
                    provenance_span_count: 2,
                    confidence_basis_points: 6400,
                    idempotency_key_hash: stable_receipt_hash(&[
                        "memory_formation",
                        "fact",
                        "session-42",
                        "2",
                        "2",
                    ]),
                    privacy_class: "user_private".into(),
                    queued_for_background: true,
                    production_write: false,
                },
                ContextMemoryFormationReceipt {
                    candidate_type: ContextMemoryFormationCandidateType::Task,
                    transcript_span_count: 2,
                    provenance_span_count: 2,
                    confidence_basis_points: 5200,
                    idempotency_key_hash: stable_receipt_hash(&[
                        "memory_formation",
                        "task",
                        "session-42",
                        "2",
                        "2",
                    ]),
                    privacy_class: "user_private".into(),
                    queued_for_background: true,
                    production_write: false,
                },
                ContextMemoryFormationReceipt {
                    candidate_type: ContextMemoryFormationCandidateType::Preference,
                    transcript_span_count: 2,
                    provenance_span_count: 2,
                    confidence_basis_points: 5200,
                    idempotency_key_hash: stable_receipt_hash(&[
                        "memory_formation",
                        "preference",
                        "session-42",
                        "2",
                        "2",
                    ]),
                    privacy_class: "user_private".into(),
                    queued_for_background: true,
                    production_write: false,
                },
                ContextMemoryFormationReceipt {
                    candidate_type: ContextMemoryFormationCandidateType::Decision,
                    transcript_span_count: 2,
                    provenance_span_count: 2,
                    confidence_basis_points: 5800,
                    idempotency_key_hash: stable_receipt_hash(&[
                        "memory_formation",
                        "decision",
                        "session-42",
                        "2",
                        "2",
                    ]),
                    privacy_class: "user_private".into(),
                    queued_for_background: true,
                    production_write: false,
                },
                ContextMemoryFormationReceipt {
                    candidate_type: ContextMemoryFormationCandidateType::Summary,
                    transcript_span_count: 2,
                    provenance_span_count: 2,
                    confidence_basis_points: 7000,
                    idempotency_key_hash: stable_receipt_hash(&[
                        "memory_formation",
                        "summary",
                        "session-42",
                        "2",
                        "2",
                    ]),
                    privacy_class: "user_private".into(),
                    queued_for_background: true,
                    production_write: false,
                },
            ],
        }
    );

    let json = serde_json::to_string(&receipts).expect("receipt report should serialize");
    assert!(json.contains("fact"));
    assert!(json.contains("task"));
    assert!(json.contains("preference"));
    assert!(json.contains("decision"));
    assert!(json.contains("summary"));
    assert!(!json.contains("timeout"));
    assert!(!json.contains("session-42"));
    assert!(!json.contains("transcript_text"));
    assert!(!json.contains("source_id"));
    assert!(!json.contains("memory_id"));
    assert!(!json.contains("query_text"));
    assert!(!json.contains("\"production_write\":true"));
}

#[test]
fn context_recall_memory_formation_queue_is_payload_light_reversible_and_non_writing() {
    let receipts = ContextMemoryFormationReceiptReport {
        receipts: vec![
            ContextMemoryFormationReceipt {
                candidate_type: ContextMemoryFormationCandidateType::Fact,
                transcript_span_count: 2,
                provenance_span_count: 2,
                confidence_basis_points: 6400,
                idempotency_key_hash: stable_receipt_hash(&[
                    "memory_formation",
                    "fact",
                    "queue-test",
                    "2",
                    "2",
                ]),
                privacy_class: "user_private".into(),
                queued_for_background: true,
                production_write: false,
            },
            ContextMemoryFormationReceipt {
                candidate_type: ContextMemoryFormationCandidateType::Summary,
                transcript_span_count: 2,
                provenance_span_count: 1,
                confidence_basis_points: 7000,
                idempotency_key_hash: stable_receipt_hash(&[
                    "memory_formation",
                    "summary",
                    "queue-test",
                    "2",
                    "1",
                ]),
                privacy_class: "user_private".into(),
                queued_for_background: true,
                production_write: false,
            },
        ],
    };

    let queue = ContextMemoryFormationQueueReport::from_receipts(&receipts);

    assert!(queue.has_queue_integrity());
    assert_eq!(
        queue.schema_version,
        CONTEXT_MEMORY_FORMATION_QUEUE_SCHEMA_VERSION
    );
    assert_eq!(queue.items.len(), 2);
    assert_eq!(queue.queued_count(), 2);
    assert_eq!(queue.revocable_count(), 2);
    assert_eq!(queue.operator_review_required_count(), 2);
    assert!(queue.items.iter().all(|item| {
        item.operator_policy == ContextMemoryFormationQueueOperatorPolicy::OperatorReviewRequired
            && item.retention_ttl_turns == 64
            && item.dry_run_only
            && item.idempotency_enforced
            && item.can_revoke_before_commit
            && !item.production_write
            && !item.graph_write
            && !item.hot_path_write
            && stable_receipt_hash_is_valid(&item.source_receipt_hash)
            && stable_receipt_hash_is_valid(&item.revocation_key_hash)
    }));
    assert!(!queue.production_write);
    assert!(!queue.graph_write);
    assert!(!queue.runtime_activation);
    assert!(!queue.hot_path_write);

    let json = serde_json::to_string(&queue).expect("queue report should serialize");
    assert!(json.contains("operator_review_required"));
    assert!(json.contains("source_receipt_hash"));
    assert!(json.contains("revocation_key_hash"));
    assert!(json.contains("idempotency_enforced"));
    assert!(json.contains("can_revoke_before_commit"));
    assert!(!json.contains("queue-test"));
    assert!(!json.contains("candidate_text"));
    assert!(!json.contains("transcript_text"));
    assert!(!json.contains("memory_text"));
    assert!(!json.contains("source_id"));
    assert!(!json.contains("memory_id"));
    assert!(!json.contains("raw_idempotency_key"));
    assert!(!json.contains("\"production_write\":true"));
    assert!(!json.contains("\"graph_write\":true"));
    assert!(!json.contains("\"runtime_activation\":true"));
    assert!(!json.contains("\"hot_path_write\":true"));
}
