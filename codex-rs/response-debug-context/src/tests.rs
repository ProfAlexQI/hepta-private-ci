use super::*;
use codex_api::ApiError;
use codex_api::TransportError;
use http::HeaderMap;
use http::HeaderValue;
use http::StatusCode;
use pretty_assertions::assert_eq;
use serde_json::json;

#[test]
fn extract_response_debug_context_decodes_identity_headers() {
    let mut headers = HeaderMap::new();
    headers.insert("x-oai-request-id", HeaderValue::from_static("req-auth"));
    headers.insert("cf-ray", HeaderValue::from_static("ray-auth"));
    headers.insert(
        "x-openai-authorization-error",
        HeaderValue::from_static("missing_authorization_header"),
    );
    headers.insert(
        "x-error-json",
        HeaderValue::from_static("eyJlcnJvciI6eyJjb2RlIjoidG9rZW5fZXhwaXJlZCJ9fQ=="),
    );

    let context = extract_response_debug_context(&TransportError::Http {
        status: StatusCode::UNAUTHORIZED,
        url: Some("https://chatgpt.com/backend-api/codex/models".to_string()),
        headers: Some(headers),
        body: Some(r#"{"error":{"message":"plain text error"},"status":401}"#.to_string()),
    });

    assert_eq!(
        context,
        ResponseDebugContext {
            request_id: Some("req-auth".to_string()),
            cf_ray: Some("ray-auth".to_string()),
            auth_error: Some("missing_authorization_header".to_string()),
            auth_error_code: Some("token_expired".to_string()),
        }
    );
}

#[test]
fn telemetry_error_messages_omit_http_bodies() {
    let transport = TransportError::Http {
        status: StatusCode::UNAUTHORIZED,
        url: Some("https://chatgpt.com/backend-api/codex/responses".to_string()),
        headers: None,
        body: Some(r#"{"error":{"message":"secret token leaked"}}"#.to_string()),
    };

    assert_eq!(telemetry_transport_error_message(&transport), "http 401");
    assert_eq!(
        telemetry_api_error_message(&ApiError::Transport(transport)),
        "http 401"
    );
}

#[test]
fn telemetry_error_messages_preserve_non_http_details() {
    let network = TransportError::Network("dns lookup failed".to_string());
    let build = TransportError::Build("invalid header value".to_string());
    let stream = ApiError::Stream("socket closed".to_string());

    assert_eq!(
        telemetry_transport_error_message(&network),
        "dns lookup failed"
    );
    assert_eq!(
        telemetry_transport_error_message(&build),
        "invalid header value"
    );
    assert_eq!(telemetry_api_error_message(&stream), "socket closed");
}

#[test]
fn rollout_context_debug_reads_manifest_nested_in_turn_context_item() {
    let context = json!({
        "type": "turn_context",
        "payload": {
            "model": "gpt-test",
            "context_manifest": {
                "version": 1,
                "estimated_tokens": 3,
                "recall_selection": {
                    "returned_source_count": 2,
                    "selected_source_count": 2,
                    "ranked_source_count": 0,
                    "returned_unselected_source_count": 0,
                    "source_diversity_met": true,
                    "source_diversity_target": 2,
                    "max_per_source": 2,
                    "ranked_item_count": 0,
                    "memory_control_omitted_count": 1,
                    "low_trust_ranked_item_count": 0,
                    "low_recency_ranked_item_count": 0,
                    "source_id": "nested-source-id-should-not-export"
                },
                "recall_selected_snippets": {
                    "version": 1,
                    "max_snippets": 4,
                    "max_snippet_chars": 120,
                    "selected_snippet_count": 1,
                    "omitted_snippet_count": 2,
                    "redacted_snippet_count": 1,
                    "truncated_snippet_count": 0,
                    "snippets": [{
                        "snippet_hash": "fedcba9876543210",
                        "text": "[redacted-query] nested selected snippet should not export",
                        "estimated_tokens": 12,
                        "redacted": true,
                        "truncated": false,
                        "source_id": "nested-snippet-source-id-should-not-export"
                    }],
                    "safety": {
                        "ready_for_shadow_handoff": true,
                        "bounded": true,
                        "origin_identifiers_exposed": false,
                        "raw_ranked_payload_exposed": false,
                        "rank_explanation_exposed": false,
                        "control_marker_exposed": false,
                        "query_payload_exposed": false,
                        "per_origin_list_exposed": false
                    }
                },
                "entries": [
                    {
                        "role": "developer",
                        "tier": "developer",
                        "source": "turn_context:developer:0",
                        "replay_key": "turn_context:developer:0:aaaaaaaaaaaaaaaa",
                        "text_hash": "aaaaaaaaaaaaaaaa",
                        "estimated_tokens": 3
                    }
                ]
            }
        }
    });
    let export = summarize_rollout_context_debug_jsonl(&format!("{context}\n"));

    assert!(export.audit.ok);
    assert_eq!(export.summary.manifest_count, 1);
    assert!(export.summary.latest_manifest_present);
    assert_eq!(
        export.summary.latest_manifest_sources,
        vec!["turn_context:developer:0".to_string()]
    );
    assert_eq!(
        export.summary.latest_manifest_tiers,
        vec!["developer".to_string()]
    );
    assert!(export.summary.latest_manifest_recall_selection_present);
    assert_eq!(
        export.summary.latest_manifest_recall_selected_source_count,
        2
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_recall_memory_control_omitted_count,
        1
    );
    assert!(
        export
            .summary
            .latest_manifest_recall_selected_snippets_present
    );
    assert!(
        !export
            .summary
            .latest_manifest_recall_selected_snippets_invalid
    );
    assert_eq!(
        export.summary.latest_manifest_recall_selected_snippet_count,
        1
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_recall_selected_snippet_omitted_count,
        2
    );
    assert!(export.summary.latest_manifest_recall_selected_snippet_ready);
    assert!(
        export
            .summary
            .latest_manifest_recall_selected_snippet_bounded
    );
    let export_json = serde_json::to_string(&export).expect("export should serialize");
    assert!(!export_json.contains("nested-source-id-should-not-export"));
    assert!(!export_json.contains("nested selected snippet should not export"));
    assert!(!export_json.contains("nested-snippet-source-id-should-not-export"));
}

#[test]
fn rollout_context_debug_summary_surfaces_recall_selection_pressure_without_payloads() {
    let context = json!({
        "type": "turn_context_manifest",
        "payload": {
            "version": 1,
            "estimated_tokens": 3,
            "budget_tokens": 4,
            "recall_selection": {
                "returned_source_count": 4,
                "selected_source_count": 3,
                "ranked_source_count": 3,
                "returned_unselected_source_count": 1,
                "source_diversity_met": true,
                "source_diversity_target": 3,
                "max_per_source": 2,
                "ranked_item_count": 3,
                "omitted_by_budget_count": 1,
                "memory_control_omitted_count": 2,
                "low_trust_ranked_item_count": 1,
                "low_recency_ranked_item_count": 2,
                "source_id": "summary-memory-id-should-not-export"
            },
            "entries": [
                {
                    "role": "developer",
                    "source": "initial_context:permissions:0",
                    "replay_key": "initial_context:permissions:0:aaaaaaaaaaaaaaaa",
                    "text_hash": "aaaaaaaaaaaaaaaa",
                    "estimated_tokens": 3
                }
            ]
        }
    });
    let export = summarize_rollout_context_debug_jsonl(&format!("{context}\n"));

    assert!(export.summary.latest_manifest_recall_selection_present);
    assert!(!export.summary.latest_manifest_recall_selection_invalid);
    assert_eq!(
        export.summary.latest_manifest_recall_returned_source_count,
        4
    );
    assert_eq!(
        export.summary.latest_manifest_recall_selected_source_count,
        3
    );
    assert_eq!(export.summary.latest_manifest_recall_ranked_source_count, 3);
    assert_eq!(
        export
            .summary
            .latest_manifest_recall_returned_unselected_source_count,
        1
    );
    assert!(export.summary.latest_manifest_recall_source_diversity_met);
    assert_eq!(
        export
            .summary
            .latest_manifest_recall_source_diversity_target,
        3
    );
    assert_eq!(export.summary.latest_manifest_recall_max_per_source, 2);
    assert_eq!(export.summary.latest_manifest_recall_ranked_item_count, 3);
    assert_eq!(
        export
            .summary
            .latest_manifest_recall_omitted_by_budget_count,
        1
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_recall_memory_control_omitted_count,
        2
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_recall_low_trust_ranked_item_count,
        1
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_recall_low_recency_ranked_item_count,
        2
    );

    let rendered = serde_json::to_string(&export).expect("export serializes");
    assert!(!rendered.contains("summary-memory-id-should-not-export"));
}

#[test]
fn rollout_context_debug_summary_combines_payload_light_surfaces_without_cross_surface_leaks() {
    let context = json!({
        "type": "turn_context_manifest",
        "payload": {
            "version": 1,
            "estimated_tokens": 72,
            "budget_tokens": 80,
            "truncated": true,
            "decision_ledger": [
                {
                    "source": "turn_context:developer:selected_context_recall:0",
                    "decision": "included:recall_selected_snippets",
                    "reason_hash": "aaaaaaaaaaaaaaaa"
                },
                {
                    "source": "turn_context:developer:selected_context_recall:0",
                    "decision": "truncated:selected_context_recall:original_tokens:24:tokens:3",
                    "reason_hash": "bbbbbbbbbbbbbbbb"
                }
            ],
            "recall_selection": {
                "returned_source_count": 4,
                "selected_source_count": 3,
                "ranked_source_count": 3,
                "returned_unselected_source_count": 1,
                "source_diversity_met": true,
                "source_diversity_target": 3,
                "max_per_source": 2,
                "ranked_item_count": 3,
                "omitted_by_budget_count": 1,
                "memory_control_omitted_count": 2,
                "low_trust_ranked_item_count": 1,
                "low_recency_ranked_item_count": 2,
                "source_id": "combined-recall-source-id-should-not-export"
            },
            "recall_selected_snippets": {
                "version": 1,
                "max_snippets": 4,
                "max_snippet_chars": 120,
                "selected_snippet_count": 1,
                "omitted_snippet_count": 2,
                "redacted_snippet_count": 1,
                "truncated_snippet_count": 0,
                "snippets": [{
                    "snippet_hash": "fedcba9876543210",
                    "text": "[redacted-query] combined selected snippet should not export",
                    "estimated_tokens": 12,
                    "redacted": true,
                    "truncated": false,
                    "source_id": "combined-snippet-source-id-should-not-export"
                }],
                "safety": {
                    "ready_for_shadow_handoff": true,
                    "bounded": true,
                    "origin_identifiers_exposed": false,
                    "raw_ranked_payload_exposed": false,
                    "rank_explanation_exposed": false,
                    "control_marker_exposed": false,
                    "query_payload_exposed": false,
                    "per_origin_list_exposed": false
                }
            },
            "memory_taxonomy": [
                {
                    "class": "semantic",
                    "source_count": 1,
                    "returned_count": 2,
                    "available_count": 3,
                    "omitted_count": 1,
                    "memory_id": "combined-memory-id-should-not-export",
                    "text": "combined memory payload should not export"
                },
                {
                    "class": "transcript",
                    "source_count": 2,
                    "returned_count": 3,
                    "available_count": 5,
                    "omitted_count": 2,
                    "provenance_span_count": 2,
                    "text": "combined transcript payload should not export"
                }
            ],
            "memory_formation_receipts": [{
                "candidate_type": "fact",
                "transcript_span_count": 2,
                "provenance_span_count": 2,
                "confidence_basis_points": 6400,
                "idempotency_key_hash": "1111111111111111",
                "privacy_class": "user_private",
                "queued_for_background": true,
                "transcript_text": "combined receipt transcript should not export",
                "memory_id": "combined-receipt-memory-id-should-not-export"
            }],
            "memory_temporal_facts": [{
                "fact_type": "attribute",
                "entity_hash": "2222222222222222",
                "provenance_span_count": 2,
                "valid_from_sequence": 8,
                "confidence_basis_points": 6200,
                "privacy_class": "user_private",
                "dry_run_only": true,
                "fact_text": "combined temporal fact should not export",
                "transcript_text": "combined temporal transcript should not export",
                "memory_text": "combined temporal memory should not export",
                "source_id": "combined-temporal-source-id-should-not-export",
                "memory_id": "combined-temporal-memory-id-should-not-export",
                "query": "combined temporal query should not export"
            }],
            "compression_candidates": [{
                "kind": "summary",
                "tier": "retrieved_snippets",
                "source_id": "selected_context_recall",
                "input_tokens": 40,
                "estimated_output_tokens": 12,
                "affected_entries": 1,
                "not_executed_reason": "budget_pressure_dry_run",
                "source": "combined-candidate-source-should-not-export",
                "text": "combined candidate payload should not export"
            }],
            "adaptive_budget_allocations": [{
                "tier": "retrieved_snippets",
                "source_id": "selected_context_recall",
                "budget_class": "bounded_recall",
                "input_tokens": 40,
                "reserve_tokens": 12,
                "proposed_budget_tokens": 12,
                "overflow_tokens": 28,
                "omit_priority": 50,
                "compression_kind": "summary",
                "estimated_compressed_tokens": 12,
                "current_heuristic_action": "drop",
                "proposed_action": "compress",
                "would_drop": false,
                "would_compress": true,
                "source": "combined-adaptive-source-should-not-export",
                "text": "combined adaptive payload should not export"
            }],
            "compression_stages": [{
                "kind": "summary",
                "input_tokens": 40,
                "output_tokens": 12,
                "affected_entries": 1,
                "loss_check_status": "marker_boundary_only",
                "rollback_source_text_hash": "3333333333333333",
                "protected_tier_invariant": "preserved",
                "source_id": "combined-stage-source-id-should-not-export",
                "replay_key": "combined-stage-replay-key-should-not-export",
                "text_hash": "combined-stage-text-hash-should-not-export",
                "text": "combined stage payload should not export"
            }],
            "entries": [{
                "role": "developer",
                "tier": "retrieved_snippets",
                "source": "turn_context:developer:selected_context_recall:0",
                "replay_key": "turn_context:developer:selected_context_recall:0:cccccccccccccccc",
                "text_hash": "cccccccccccccccc",
                "estimated_tokens": 72
            }]
        }
    });
    let export = summarize_rollout_context_debug_jsonl(&format!("{context}\n"));

    assert!(export.audit.ok);
    assert_eq!(export.summary.line_count, 1);
    assert_eq!(export.summary.manifest_count, 1);
    assert!(export.summary.latest_manifest_present);
    assert!(export.summary.latest_manifest_truncated);
    assert_eq!(export.summary.latest_manifest_decision_schema_version, 1);
    assert_eq!(export.summary.latest_manifest_decision_known_count, 2);
    assert_eq!(export.summary.latest_manifest_decision_included_count, 1);
    assert_eq!(export.summary.latest_manifest_decision_truncated_count, 1);
    assert_eq!(export.summary.latest_manifest_truncated_decision_count, 1);
    assert_eq!(
        export.summary.latest_manifest_truncated_sources,
        vec!["turn_context:developer:selected_context_recall:0".to_string()]
    );
    assert!(export.summary.latest_manifest_truncation_evidence_present);
    assert!(!export.summary.latest_manifest_truncation_evidence_invalid);

    assert!(export.summary.latest_manifest_recall_selection_present);
    assert_eq!(
        export.summary.latest_manifest_recall_returned_source_count,
        4
    );
    assert_eq!(
        export.summary.latest_manifest_recall_selected_source_count,
        3
    );
    assert_eq!(export.summary.latest_manifest_recall_ranked_source_count, 3);
    assert_eq!(
        export
            .summary
            .latest_manifest_recall_returned_unselected_source_count,
        1
    );
    assert!(export.summary.latest_manifest_recall_source_diversity_met);
    assert_eq!(
        export
            .summary
            .latest_manifest_recall_source_diversity_target,
        3
    );
    assert_eq!(export.summary.latest_manifest_recall_max_per_source, 2);
    assert_eq!(export.summary.latest_manifest_recall_ranked_item_count, 3);
    assert_eq!(
        export
            .summary
            .latest_manifest_recall_omitted_by_budget_count,
        1
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_recall_memory_control_omitted_count,
        2
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_recall_low_trust_ranked_item_count,
        1
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_recall_low_recency_ranked_item_count,
        2
    );

    assert!(
        export
            .summary
            .latest_manifest_recall_selected_snippets_present
    );
    assert_eq!(
        export.summary.latest_manifest_recall_selected_snippet_count,
        1
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_recall_selected_snippet_omitted_count,
        2
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_recall_selected_snippet_redacted_count,
        1
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_recall_selected_snippet_truncated_count,
        0
    );
    assert!(export.summary.latest_manifest_recall_selected_snippet_ready);
    assert!(
        export
            .summary
            .latest_manifest_recall_selected_snippet_bounded
    );

    assert_eq!(
        export
            .summary
            .latest_manifest_memory_taxonomy_schema_version,
        1
    );
    assert_eq!(export.summary.latest_manifest_memory_taxonomy_count, 2);
    assert_eq!(
        export.summary.latest_manifest_memory_taxonomy_classes,
        vec!["semantic".to_string(), "transcript".to_string()]
    );
    assert_eq!(
        export.summary.latest_manifest_memory_taxonomy_source_count,
        3
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_memory_taxonomy_returned_count,
        5
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_memory_taxonomy_available_count,
        8
    );
    assert_eq!(
        export.summary.latest_manifest_memory_taxonomy_omitted_count,
        3
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_memory_taxonomy_provenance_span_count,
        2
    );

    assert_eq!(
        export
            .summary
            .latest_manifest_memory_formation_receipt_schema_version,
        1
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_memory_formation_receipt_count,
        1
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_memory_formation_receipt_candidate_types,
        vec!["fact".to_string()]
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_memory_formation_receipt_privacy_classes,
        vec!["user_private".to_string()]
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_memory_formation_receipt_transcript_span_count,
        2
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_memory_formation_receipt_provenance_span_count,
        2
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_memory_formation_receipt_confidence_basis_points,
        6400
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_memory_formation_receipt_queued_count,
        1
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_memory_formation_receipt_production_write_count,
        0
    );

    assert_eq!(
        export
            .summary
            .latest_manifest_memory_temporal_fact_schema_version,
        1
    );
    assert_eq!(export.summary.latest_manifest_memory_temporal_fact_count, 1);
    assert_eq!(
        export.summary.latest_manifest_memory_temporal_fact_types,
        vec!["attribute".to_string()]
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_memory_temporal_fact_privacy_classes,
        vec!["user_private".to_string()]
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_memory_temporal_fact_provenance_span_count,
        2
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_memory_temporal_fact_confidence_basis_points,
        6200
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_memory_temporal_fact_open_count,
        1
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_memory_temporal_fact_invalidated_count,
        0
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_memory_temporal_fact_supersedes_count,
        0
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_memory_temporal_fact_dry_run_count,
        1
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_memory_temporal_fact_production_write_count,
        0
    );

    assert_eq!(
        export
            .summary
            .latest_manifest_compression_candidate_schema_version,
        1
    );
    assert_eq!(
        export.summary.latest_manifest_compression_candidate_count,
        1
    );
    assert_eq!(
        export.summary.latest_manifest_compression_candidate_stages,
        vec!["summary".to_string()]
    );
    assert_eq!(
        export.summary.latest_manifest_compression_candidate_tiers,
        vec!["retrieved_snippets".to_string()]
    );
    assert_eq!(
        export.summary.latest_manifest_compression_candidate_sources,
        vec!["selected_context_recall".to_string()]
    );
    assert_eq!(
        export.summary.latest_manifest_compression_candidate_reasons,
        vec!["budget_pressure_dry_run".to_string()]
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_compression_candidate_input_tokens,
        40
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_compression_candidate_output_tokens,
        12
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_compression_candidate_tokens_saved,
        28
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_compression_candidate_affected_entries,
        1
    );

    assert_eq!(
        export
            .summary
            .latest_manifest_adaptive_budget_allocation_schema_version,
        1
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_adaptive_budget_allocation_count,
        1
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_adaptive_budget_allocation_sources,
        vec!["selected_context_recall".to_string()]
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_adaptive_budget_allocation_budget_classes,
        vec!["bounded_recall".to_string()]
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_adaptive_budget_allocation_current_actions,
        vec!["drop".to_string()]
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_adaptive_budget_allocation_proposed_actions,
        vec!["compress".to_string()]
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_adaptive_budget_allocation_input_tokens,
        40
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_adaptive_budget_allocation_reserve_tokens,
        12
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_adaptive_budget_allocation_proposed_budget_tokens,
        12
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_adaptive_budget_allocation_overflow_tokens,
        28
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_adaptive_budget_allocation_would_drop_count,
        0
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_adaptive_budget_allocation_would_compress_count,
        1
    );

    assert_eq!(
        export
            .summary
            .latest_manifest_compression_stage_schema_version,
        2
    );
    assert_eq!(export.summary.latest_manifest_compression_stage_count, 1);
    assert_eq!(
        export.summary.latest_manifest_compression_stages,
        vec!["summary".to_string()]
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_compression_loss_check_statuses,
        vec!["marker_boundary_only".to_string()]
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_compression_rollback_source_text_hash_count,
        1
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_compression_protected_tier_invariants,
        vec!["preserved".to_string()]
    );
    assert_eq!(export.summary.latest_manifest_compression_input_tokens, 40);
    assert_eq!(export.summary.latest_manifest_compression_output_tokens, 12);
    assert_eq!(export.summary.latest_manifest_compression_tokens_saved, 28);
    assert_eq!(
        export.summary.latest_manifest_compression_affected_entries,
        1
    );

    let rendered = serde_json::to_string(&export).expect("export serializes");
    for leaked in [
        "aaaaaaaaaaaaaaaa",
        "bbbbbbbbbbbbbbbb",
        "cccccccccccccccc",
        "combined-recall-source-id-should-not-export",
        "combined selected snippet should not export",
        "combined-snippet-source-id-should-not-export",
        "fedcba9876543210",
        "combined-memory-id-should-not-export",
        "combined memory payload should not export",
        "combined transcript payload should not export",
        "1111111111111111",
        "combined receipt transcript should not export",
        "combined-receipt-memory-id-should-not-export",
        "2222222222222222",
        "combined temporal fact should not export",
        "combined temporal transcript should not export",
        "combined temporal memory should not export",
        "combined-temporal-source-id-should-not-export",
        "combined-temporal-memory-id-should-not-export",
        "combined temporal query should not export",
        "combined-candidate-source-should-not-export",
        "combined candidate payload should not export",
        "combined-adaptive-source-should-not-export",
        "combined adaptive payload should not export",
        "3333333333333333",
        "combined-stage-source-id-should-not-export",
        "combined-stage-replay-key-should-not-export",
        "combined-stage-text-hash-should-not-export",
        "combined stage payload should not export",
    ] {
        assert!(!rendered.contains(leaked), "export leaked {leaked}");
    }
}

#[test]
fn rollout_context_debug_summary_surfaces_memory_taxonomy_without_payloads() {
    let context = json!({
        "type": "turn_context_manifest",
        "payload": {
            "version": 1,
            "estimated_tokens": 34,
            "budget_tokens": 24,
            "memory_taxonomy": [
                {
                    "class": "semantic",
                    "source_count": 1,
                    "returned_count": 2,
                    "available_count": 3,
                    "omitted_count": 1,
                    "memory_id": "semantic-memory-id-should-not-export",
                    "text": "semantic memory payload should not export"
                },
                {
                    "class": "episodic",
                    "source_count": 1,
                    "returned_count": 1,
                    "available_count": 1,
                    "omitted_count": 0,
                    "source_id": "summary-source-id-should-not-export"
                },
                {
                    "class": "control",
                    "source_count": 1,
                    "returned_count": 0,
                    "available_count": 2,
                    "omitted_count": 2,
                    "query": "control query should not export"
                },
                {
                    "class": "transcript",
                    "source_count": 2,
                    "returned_count": 3,
                    "available_count": 5,
                    "omitted_count": 2,
                    "provenance_span_count": 2,
                    "text": "transcript payload should not export"
                }
            ],
            "entries": [{
                "role": "developer",
                "tier": "retrieved_snippets",
                "source": "turn_context:developer:selected_context_recall:0",
                "replay_key": "turn_context:developer:selected_context_recall:0:aaaaaaaaaaaaaaaa",
                "text_hash": "aaaaaaaaaaaaaaaa",
                "estimated_tokens": 34
            }]
        }
    });
    let export = summarize_rollout_context_debug_jsonl(&format!("{context}\n"));

    assert!(export.audit.ok);
    assert_eq!(
        export
            .summary
            .latest_manifest_memory_taxonomy_schema_version,
        1
    );
    assert_eq!(export.summary.latest_manifest_memory_taxonomy_count, 4);
    assert_eq!(
        export.summary.latest_manifest_memory_taxonomy_classes,
        vec![
            "semantic".to_string(),
            "episodic".to_string(),
            "control".to_string(),
            "transcript".to_string(),
        ]
    );
    assert_eq!(
        export.summary.latest_manifest_memory_taxonomy_source_count,
        5
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_memory_taxonomy_returned_count,
        6
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_memory_taxonomy_available_count,
        11
    );
    assert_eq!(
        export.summary.latest_manifest_memory_taxonomy_omitted_count,
        5
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_memory_taxonomy_provenance_span_count,
        2
    );
    assert!(!export.summary.latest_manifest_memory_taxonomy_invalid);

    let rendered = serde_json::to_string(&export).expect("export serializes");
    assert!(!rendered.contains("semantic-memory-id-should-not-export"));
    assert!(!rendered.contains("semantic memory payload should not export"));
    assert!(!rendered.contains("summary-source-id-should-not-export"));
    assert!(!rendered.contains("control query should not export"));
    assert!(!rendered.contains("transcript payload should not export"));
}

#[test]
fn rollout_context_debug_audit_rejects_invalid_memory_taxonomy() {
    let context = json!({
        "type": "turn_context_manifest",
        "payload": {
            "version": 1,
            "estimated_tokens": 10,
            "memory_taxonomy": [{
                "class": "semantic",
                "source_count": 1,
                "returned_count": 2,
                "available_count": 3,
                "omitted_count": 0
            }],
            "entries": [{
                "role": "developer",
                "tier": "cross_session_memory",
                "source": "turn_context:developer:memory:0",
                "replay_key": "turn_context:developer:memory:0:aaaaaaaaaaaaaaaa",
                "text_hash": "aaaaaaaaaaaaaaaa",
                "estimated_tokens": 10
            }]
        }
    });
    let export = summarize_rollout_context_debug_jsonl(&format!("{context}\n"));

    assert!(!export.audit.ok);
    assert!(export.summary.latest_manifest_memory_taxonomy_invalid);
    assert!(finding_codes(&export.audit).contains(&"manifest_memory_taxonomy_invalid"));
}

#[test]
fn rollout_context_debug_summary_surfaces_memory_formation_receipts_without_payloads() {
    let context = json!({
        "type": "turn_context_manifest",
        "payload": {
            "version": 1,
            "estimated_tokens": 18,
            "budget_tokens": 24,
            "memory_formation_receipts": [
                {
                    "candidate_type": "fact",
                    "transcript_span_count": 2,
                    "provenance_span_count": 2,
                    "confidence_basis_points": 6400,
                    "idempotency_key_hash": "0123456789abcdef",
                    "privacy_class": "user_private",
                    "queued_for_background": true,
                    "transcript_text": "receipt transcript payload should not export",
                    "memory_id": "receipt-memory-id-should-not-export"
                },
                {
                    "candidate_type": "summary",
                    "transcript_span_count": 2,
                    "provenance_span_count": 1,
                    "confidence_basis_points": 7000,
                    "idempotency_key_hash": "fedcba9876543210",
                    "privacy_class": "user_private",
                    "queued_for_background": true,
                    "source_id": "receipt-source-id-should-not-export",
                    "query": "receipt query should not export"
                }
            ],
            "entries": [{
                "role": "developer",
                "tier": "retrieved_snippets",
                "source": "turn_context:developer:selected_context_recall:0",
                "replay_key": "turn_context:developer:selected_context_recall:0:aaaaaaaaaaaaaaaa",
                "text_hash": "aaaaaaaaaaaaaaaa",
                "estimated_tokens": 18
            }]
        }
    });
    let export = summarize_rollout_context_debug_jsonl(&format!("{context}\n"));

    assert!(export.audit.ok);
    assert_eq!(
        export
            .summary
            .latest_manifest_memory_formation_receipt_schema_version,
        1
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_memory_formation_receipt_count,
        2
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_memory_formation_receipt_candidate_types,
        vec!["fact".to_string(), "summary".to_string()]
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_memory_formation_receipt_privacy_classes,
        vec!["user_private".to_string()]
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_memory_formation_receipt_transcript_span_count,
        4
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_memory_formation_receipt_provenance_span_count,
        3
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_memory_formation_receipt_confidence_basis_points,
        13400
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_memory_formation_receipt_queued_count,
        2
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_memory_formation_receipt_production_write_count,
        0
    );
    assert!(
        !export
            .summary
            .latest_manifest_memory_formation_receipt_invalid
    );

    let rendered = serde_json::to_string(&export).expect("export serializes");
    assert!(!rendered.contains("0123456789abcdef"));
    assert!(!rendered.contains("fedcba9876543210"));
    assert!(!rendered.contains("receipt transcript payload should not export"));
    assert!(!rendered.contains("receipt-memory-id-should-not-export"));
    assert!(!rendered.contains("receipt-source-id-should-not-export"));
    assert!(!rendered.contains("receipt query should not export"));
}

#[test]
fn rollout_context_debug_summary_surfaces_memory_temporal_facts_without_payloads() {
    let context = json!({
        "type": "turn_context_manifest",
        "payload": {
            "version": 1,
            "estimated_tokens": 18,
            "budget_tokens": 24,
            "memory_temporal_facts": [
                {
                    "fact_type": "attribute",
                    "entity_hash": "0123456789abcdef",
                    "provenance_span_count": 2,
                    "valid_from_sequence": 8,
                    "confidence_basis_points": 6200,
                    "privacy_class": "user_private",
                    "dry_run_only": true,
                    "fact_text": "temporal fact payload should not export",
                    "transcript_text": "temporal transcript payload should not export",
                    "memory_text": "temporal memory payload should not export",
                    "source_id": "temporal-source-id-should-not-export",
                    "memory_id": "temporal-memory-id-should-not-export",
                    "query": "temporal query should not export"
                },
                {
                    "fact_type": "summary",
                    "entity_hash": "fedcba9876543210",
                    "provenance_span_count": 1,
                    "valid_from_sequence": 9,
                    "invalid_at_sequence": 12,
                    "confidence_basis_points": 7000,
                    "supersedes_fact_hash": "aaaaaaaaaaaaaaaa",
                    "privacy_class": "user_private",
                    "dry_run_only": true,
                    "raw_fact": "raw temporal fact should not export",
                    "entity": "raw temporal entity should not export"
                }
            ],
            "entries": [{
                "role": "developer",
                "tier": "retrieved_snippets",
                "source": "turn_context:developer:selected_context_recall:0",
                "replay_key": "turn_context:developer:selected_context_recall:0:aaaaaaaaaaaaaaaa",
                "text_hash": "aaaaaaaaaaaaaaaa",
                "estimated_tokens": 18
            }]
        }
    });
    let export = summarize_rollout_context_debug_jsonl(&format!("{context}\n"));

    assert!(export.audit.ok);
    assert_eq!(
        export
            .summary
            .latest_manifest_memory_temporal_fact_schema_version,
        1
    );
    assert_eq!(export.summary.latest_manifest_memory_temporal_fact_count, 2);
    assert_eq!(
        export.summary.latest_manifest_memory_temporal_fact_types,
        vec!["attribute".to_string(), "summary".to_string()]
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_memory_temporal_fact_privacy_classes,
        vec!["user_private".to_string()]
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_memory_temporal_fact_provenance_span_count,
        3
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_memory_temporal_fact_confidence_basis_points,
        13200
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_memory_temporal_fact_open_count,
        1
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_memory_temporal_fact_invalidated_count,
        1
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_memory_temporal_fact_supersedes_count,
        1
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_memory_temporal_fact_dry_run_count,
        2
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_memory_temporal_fact_production_write_count,
        0
    );
    assert!(!export.summary.latest_manifest_memory_temporal_fact_invalid);

    let rendered = serde_json::to_string(&export).expect("export serializes");
    for leaked in [
        "0123456789abcdef",
        "fedcba9876543210",
        "aaaaaaaaaaaaaaaa",
        "temporal fact payload should not export",
        "temporal transcript payload should not export",
        "temporal memory payload should not export",
        "temporal-source-id-should-not-export",
        "temporal-memory-id-should-not-export",
        "temporal query should not export",
        "raw temporal fact should not export",
        "raw temporal entity should not export",
    ] {
        assert!(!rendered.contains(leaked), "export leaked {leaked}");
    }
}

#[test]
fn rollout_context_debug_ignores_memory_formation_candidate_preview_payloads() {
    let context = json!({
        "type": "turn_context_manifest",
        "payload": {
            "version": 1,
            "estimated_tokens": 18,
            "budget_tokens": 24,
            "memory_formation_receipts": [{
                "candidate_type": "fact",
                "transcript_span_count": 2,
                "provenance_span_count": 2,
                "confidence_basis_points": 6400,
                "idempotency_key_hash": "0123456789abcdef",
                "privacy_class": "user_private",
                "queued_for_background": true
            }],
            "memory_formation_candidate_previews": [{
                "candidate_type": "fact",
                "candidate_text": "candidate fact payload should not export",
                "transcript_text": "candidate transcript payload should not export",
                "memory_text": "candidate memory payload should not export",
                "tool_args": {
                    "command": "candidate tool args should not export"
                },
                "raw_idempotency_key": "raw-idempotency-key-should-not-export",
                "idempotency_key": "idempotency-key-should-not-export",
                "idempotency_key_hash": "candidate-preview-hash-should-not-export",
                "source_id": "candidate-preview-source-id-should-not-export",
                "source_ids": ["candidate-preview-source-list-should-not-export"],
                "memory_id": "candidate-preview-memory-id-should-not-export",
                "memory_ids": ["candidate-preview-memory-list-should-not-export"],
                "per_source_candidates": [{
                    "source_id": "candidate-preview-per-source-id-should-not-export"
                }],
                "email": "candidate-email@example.invalid",
                "phone": "+15550101010",
                "user_identifier": "candidate-user-identifier-should-not-export"
            }],
            "memory_formation_candidates": [{
                "candidate_text": "future candidate payload should not export",
                "raw_transcript": "future raw transcript should not export",
                "tool_arguments": "future tool arguments should not export"
            }],
            "entries": [{
                "role": "developer",
                "tier": "retrieved_snippets",
                "source": "turn_context:developer:selected_context_recall:0",
                "replay_key": "turn_context:developer:selected_context_recall:0:aaaaaaaaaaaaaaaa",
                "text_hash": "aaaaaaaaaaaaaaaa",
                "estimated_tokens": 18
            }]
        }
    });
    let export = summarize_rollout_context_debug_jsonl(&format!("{context}\n"));

    assert!(export.audit.ok);
    assert_eq!(
        export
            .summary
            .latest_manifest_memory_formation_receipt_count,
        1
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_memory_formation_receipt_candidate_types,
        vec!["fact".to_string()]
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_memory_formation_receipt_production_write_count,
        0
    );
    assert!(
        !export
            .summary
            .latest_manifest_memory_formation_receipt_invalid
    );

    let rendered = serde_json::to_string(&export).expect("export serializes");
    for leaked in [
        "memory_formation_candidate_previews",
        "memory_formation_candidates",
        "candidate fact payload should not export",
        "candidate transcript payload should not export",
        "candidate memory payload should not export",
        "candidate tool args should not export",
        "raw-idempotency-key-should-not-export",
        "idempotency-key-should-not-export",
        "candidate-preview-hash-should-not-export",
        "candidate-preview-source-id-should-not-export",
        "candidate-preview-source-list-should-not-export",
        "candidate-preview-memory-id-should-not-export",
        "candidate-preview-memory-list-should-not-export",
        "candidate-preview-per-source-id-should-not-export",
        "candidate-email@example.invalid",
        "+15550101010",
        "candidate-user-identifier-should-not-export",
        "future candidate payload should not export",
        "future raw transcript should not export",
        "future tool arguments should not export",
    ] {
        assert!(!rendered.contains(leaked), "export leaked {leaked}");
    }
}

#[test]
fn rollout_context_debug_audit_rejects_invalid_memory_temporal_facts() {
    let context = json!({
        "type": "turn_context_manifest",
        "payload": {
            "version": 1,
            "estimated_tokens": 10,
            "memory_temporal_facts": [{
                "fact_type": "attribute",
                "entity_hash": "0123456789abcdef",
                "provenance_span_count": 2,
                "valid_from_sequence": 8,
                "confidence_basis_points": 6200,
                "supersedes_fact_hash": "raw-fact-id",
                "privacy_class": "user_private",
                "dry_run_only": true
            }],
            "entries": [{
                "role": "developer",
                "tier": "retrieved_snippets",
                "source": "turn_context:developer:selected_context_recall:0",
                "replay_key": "turn_context:developer:selected_context_recall:0:aaaaaaaaaaaaaaaa",
                "text_hash": "aaaaaaaaaaaaaaaa",
                "estimated_tokens": 10
            }]
        }
    });
    let export = summarize_rollout_context_debug_jsonl(&format!("{context}\n"));

    assert!(!export.audit.ok);
    assert!(export.summary.latest_manifest_memory_temporal_fact_invalid);
    assert!(finding_codes(&export.audit).contains(&"manifest_memory_temporal_facts_invalid"));
}

#[test]
fn rollout_context_debug_audit_rejects_invalid_memory_formation_receipts() {
    let context = json!({
        "type": "turn_context_manifest",
        "payload": {
            "version": 1,
            "estimated_tokens": 10,
            "memory_formation_receipts": [{
                "candidate_type": "fact",
                "transcript_span_count": 2,
                "provenance_span_count": 2,
                "confidence_basis_points": 6400,
                "idempotency_key_hash": "0123456789abcdef",
                "privacy_class": "user_private",
                "queued_for_background": true,
                "production_write": true
            }],
            "entries": [{
                "role": "developer",
                "tier": "retrieved_snippets",
                "source": "turn_context:developer:selected_context_recall:0",
                "replay_key": "turn_context:developer:selected_context_recall:0:aaaaaaaaaaaaaaaa",
                "text_hash": "aaaaaaaaaaaaaaaa",
                "estimated_tokens": 10
            }]
        }
    });
    let export = summarize_rollout_context_debug_jsonl(&format!("{context}\n"));

    assert!(!export.audit.ok);
    assert!(
        export
            .summary
            .latest_manifest_memory_formation_receipt_invalid
    );
    assert!(finding_codes(&export.audit).contains(&"manifest_memory_formation_receipts_invalid"));
}

#[test]
fn rollout_context_debug_summary_surfaces_truncation_evidence_without_payloads() {
    let context = json!({
        "type": "turn_context_manifest",
        "payload": {
            "version": 1,
            "estimated_tokens": 3,
            "budget_tokens": 4,
            "truncated": true,
            "decision_ledger": [
                {
                    "source": "turn_context:developer:selected_context_recall:0",
                    "decision": "included:recall_selected_snippets",
                    "reason_hash": "aaaaaaaaaaaaaaaa"
                },
                {
                    "source": "turn_context:developer:selected_context_recall:0",
                    "decision": "truncated:selected_context_recall:original_tokens:24:tokens:3",
                    "reason_hash": "bbbbbbbbbbbbbbbb"
                }
            ],
            "entries": [
                {
                    "role": "developer",
                    "tier": "retrieved_snippets",
                    "source": "turn_context:developer:selected_context_recall:0",
                    "replay_key": "turn_context:developer:selected_context_recall:0:cccccccccccccccc",
                    "text_hash": "cccccccccccccccc",
                    "estimated_tokens": 3
                }
            ]
        }
    });
    let export = summarize_rollout_context_debug_jsonl(&format!("{context}\n"));

    assert!(export.audit.ok);
    assert!(export.summary.latest_manifest_truncated);
    assert_eq!(export.summary.latest_manifest_decision_schema_version, 1);
    assert_eq!(export.summary.latest_manifest_decision_known_count, 2);
    assert_eq!(export.summary.latest_manifest_decision_unknown_count, 0);
    assert_eq!(export.summary.latest_manifest_decision_included_count, 1);
    assert_eq!(export.summary.latest_manifest_decision_policy_count, 0);
    assert_eq!(
        export.summary.latest_manifest_decision_candidate_omit_count,
        0
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_decision_candidate_truncate_count,
        0
    );
    assert_eq!(export.summary.latest_manifest_decision_omitted_count, 0);
    assert_eq!(export.summary.latest_manifest_decision_truncated_count, 1);
    assert_eq!(export.summary.latest_manifest_truncated_decision_count, 1);
    assert_eq!(
        export.summary.latest_manifest_truncated_sources,
        vec!["turn_context:developer:selected_context_recall:0".to_string()]
    );
    assert_eq!(
        export.summary.latest_manifest_tiers,
        vec!["retrieved_snippets".to_string()]
    );
    assert!(export.summary.latest_manifest_truncation_evidence_present);
    assert!(!export.summary.latest_manifest_truncation_evidence_invalid);

    let rendered = serde_json::to_string(&export).expect("export serializes");
    assert!(!rendered.contains("truncated:selected_context_recall"));
    assert!(!rendered.contains("original_tokens"));
    assert!(!rendered.contains("cccccccccccccccc"));
}

#[test]
fn rollout_context_debug_summary_surfaces_compression_candidates_without_payloads() {
    let context = json!({
        "type": "turn_context_manifest",
        "payload": {
            "version": 1,
            "estimated_tokens": 72,
            "budget_tokens": 20,
            "compression_candidates": [
                {
                    "kind": "summary",
                    "tier": "retrieved_snippets",
                    "source_id": "selected_context_recall",
                    "input_tokens": 40,
                    "estimated_output_tokens": 12,
                    "affected_entries": 1,
                    "not_executed_reason": "budget_pressure_dry_run",
                    "source": "candidate-source-should-not-export",
                    "text": "compression candidate payload should not export"
                },
                {
                    "kind": "defragment",
                    "tier": "tool",
                    "source_id": "available_plugins",
                    "input_tokens": 12,
                    "estimated_output_tokens": 8,
                    "affected_entries": 1,
                    "not_executed_reason": "budget_pressure_dry_run",
                    "query": "compression candidate query should not export"
                }
            ],
            "entries": [{
                "role": "developer",
                "tier": "retrieved_snippets",
                "source": "turn_context:developer:selected_context_recall:0",
                "replay_key": "turn_context:developer:selected_context_recall:0:aaaaaaaaaaaaaaaa",
                "text_hash": "aaaaaaaaaaaaaaaa",
                "estimated_tokens": 40
            }]
        }
    });
    let export = summarize_rollout_context_debug_jsonl(&format!("{context}\n"));

    assert!(export.audit.ok);
    assert_eq!(
        export
            .summary
            .latest_manifest_compression_candidate_schema_version,
        1
    );
    assert_eq!(
        export.summary.latest_manifest_compression_candidate_count,
        2
    );
    assert_eq!(
        export.summary.latest_manifest_compression_candidate_stages,
        vec!["summary".to_string(), "defragment".to_string()]
    );
    assert_eq!(
        export.summary.latest_manifest_compression_candidate_tiers,
        vec!["retrieved_snippets".to_string(), "tool".to_string()]
    );
    assert_eq!(
        export.summary.latest_manifest_compression_candidate_sources,
        vec![
            "selected_context_recall".to_string(),
            "available_plugins".to_string()
        ]
    );
    assert_eq!(
        export.summary.latest_manifest_compression_candidate_reasons,
        vec!["budget_pressure_dry_run".to_string()]
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_compression_candidate_input_tokens,
        52
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_compression_candidate_output_tokens,
        20
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_compression_candidate_tokens_saved,
        32
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_compression_candidate_affected_entries,
        2
    );
    assert!(!export.summary.latest_manifest_compression_candidate_invalid);

    let rendered = serde_json::to_string(&export).expect("export serializes");
    assert!(!rendered.contains("candidate-source-should-not-export"));
    assert!(!rendered.contains("compression candidate payload should not export"));
    assert!(!rendered.contains("compression candidate query should not export"));
}

#[test]
fn rollout_context_debug_audit_rejects_invalid_compression_candidate() {
    let context = json!({
        "type": "turn_context_manifest",
        "payload": {
            "version": 1,
            "estimated_tokens": 10,
            "compression_candidates": [{
                "kind": "summary",
                "tier": "retrieved_snippets",
                "source_id": "selected_context_recall",
                "input_tokens": 4,
                "estimated_output_tokens": 12,
                "affected_entries": 1,
                "not_executed_reason": "budget_pressure_dry_run"
            }],
            "entries": [{
                "role": "developer",
                "tier": "retrieved_snippets",
                "source": "turn_context:developer:selected_context_recall:0",
                "replay_key": "turn_context:developer:selected_context_recall:0:aaaaaaaaaaaaaaaa",
                "text_hash": "aaaaaaaaaaaaaaaa",
                "estimated_tokens": 10
            }]
        }
    });
    let export = summarize_rollout_context_debug_jsonl(&format!("{context}\n"));

    assert!(!export.audit.ok);
    assert!(export.summary.latest_manifest_compression_candidate_invalid);
    assert!(finding_codes(&export.audit).contains(&"manifest_compression_candidates_invalid"));
}

#[test]
fn rollout_context_debug_summary_surfaces_adaptive_budget_allocations_without_payloads() {
    let context = json!({
        "type": "turn_context_manifest",
        "payload": {
            "version": 1,
            "estimated_tokens": 72,
            "budget_tokens": 20,
            "adaptive_budget_allocations": [
                {
                    "tier": "retrieved_snippets",
                    "source_id": "selected_context_recall",
                    "budget_class": "bounded_recall",
                    "input_tokens": 40,
                    "reserve_tokens": 12,
                    "proposed_budget_tokens": 12,
                    "overflow_tokens": 28,
                    "omit_priority": 50,
                    "compression_kind": "summary",
                    "estimated_compressed_tokens": 12,
                    "current_heuristic_action": "drop",
                    "proposed_action": "compress",
                    "would_drop": false,
                    "would_compress": true,
                    "source": "adaptive-source-should-not-export",
                    "text": "adaptive allocation payload should not export"
                },
                {
                    "tier": "tool",
                    "source_id": "available_plugins",
                    "budget_class": "tool_inventory",
                    "input_tokens": 12,
                    "reserve_tokens": 8,
                    "proposed_budget_tokens": 0,
                    "overflow_tokens": 12,
                    "omit_priority": 20,
                    "compression_kind": "defragment",
                    "estimated_compressed_tokens": 8,
                    "current_heuristic_action": "drop",
                    "proposed_action": "drop",
                    "would_drop": true,
                    "would_compress": false,
                    "query": "adaptive allocation query should not export"
                }
            ],
            "entries": [{
                "role": "developer",
                "tier": "retrieved_snippets",
                "source": "turn_context:developer:selected_context_recall:0",
                "replay_key": "turn_context:developer:selected_context_recall:0:aaaaaaaaaaaaaaaa",
                "text_hash": "aaaaaaaaaaaaaaaa",
                "estimated_tokens": 40
            }]
        }
    });
    let export = summarize_rollout_context_debug_jsonl(&format!("{context}\n"));

    assert!(export.audit.ok);
    assert_eq!(
        export
            .summary
            .latest_manifest_adaptive_budget_allocation_schema_version,
        1
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_adaptive_budget_allocation_count,
        2
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_adaptive_budget_allocation_sources,
        vec![
            "selected_context_recall".to_string(),
            "available_plugins".to_string()
        ]
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_adaptive_budget_allocation_budget_classes,
        vec!["bounded_recall".to_string(), "tool_inventory".to_string()]
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_adaptive_budget_allocation_current_actions,
        vec!["drop".to_string()]
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_adaptive_budget_allocation_proposed_actions,
        vec!["compress".to_string(), "drop".to_string()]
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_adaptive_budget_allocation_input_tokens,
        52
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_adaptive_budget_allocation_reserve_tokens,
        20
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_adaptive_budget_allocation_proposed_budget_tokens,
        12
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_adaptive_budget_allocation_overflow_tokens,
        40
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_adaptive_budget_allocation_would_drop_count,
        1
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_adaptive_budget_allocation_would_compress_count,
        1
    );
    assert!(
        !export
            .summary
            .latest_manifest_adaptive_budget_allocation_invalid
    );

    let rendered = serde_json::to_string(&export).expect("export serializes");
    assert!(!rendered.contains("adaptive-source-should-not-export"));
    assert!(!rendered.contains("adaptive allocation payload should not export"));
    assert!(!rendered.contains("adaptive allocation query should not export"));
}

#[test]
fn rollout_context_debug_audit_rejects_invalid_adaptive_budget_allocation() {
    let context = json!({
        "type": "turn_context_manifest",
        "payload": {
            "version": 1,
            "estimated_tokens": 10,
            "adaptive_budget_allocations": [{
                "tier": "retrieved_snippets",
                "source_id": "selected_context_recall",
                "budget_class": "bounded_recall",
                "input_tokens": 4,
                "reserve_tokens": 2,
                "proposed_budget_tokens": 12,
                "overflow_tokens": 0,
                "omit_priority": 50,
                "compression_kind": "summary",
                "estimated_compressed_tokens": 2,
                "current_heuristic_action": "drop",
                "proposed_action": "compress",
                "would_drop": false,
                "would_compress": true
            }],
            "entries": [{
                "role": "developer",
                "tier": "retrieved_snippets",
                "source": "turn_context:developer:selected_context_recall:0",
                "replay_key": "turn_context:developer:selected_context_recall:0:aaaaaaaaaaaaaaaa",
                "text_hash": "aaaaaaaaaaaaaaaa",
                "estimated_tokens": 10
            }]
        }
    });
    let export = summarize_rollout_context_debug_jsonl(&format!("{context}\n"));

    assert!(!export.audit.ok);
    assert!(
        export
            .summary
            .latest_manifest_adaptive_budget_allocation_invalid
    );
    assert!(finding_codes(&export.audit).contains(&"manifest_adaptive_budget_allocations_invalid"));
}

#[test]
fn rollout_context_debug_summary_surfaces_compression_stages_without_payloads() {
    let context = json!({
        "type": "turn_context_manifest",
        "payload": {
            "version": 1,
            "estimated_tokens": 10,
            "budget_tokens": 20,
            "compression_stages": [
                {
                    "kind": "summary",
                    "input_tokens": 40,
                    "output_tokens": 12,
                    "affected_entries": 2,
                    "loss_check_status": "marker_boundary_only",
                    "rollback_source_text_hash": "bbbbbbbbbbbbbbbb",
                    "protected_tier_invariant": "preserved",
                    "source_id": "compression-source-id-should-not-export",
                    "text": "compression prompt payload should not export"
                },
                {
                    "kind": "prune",
                    "input_tokens": 12,
                    "output_tokens": 10,
                    "affected_entries": 1,
                    "loss_check_status": "marker_boundary_only",
                    "rollback_source_text_hash": "cccccccccccccccc",
                    "protected_tier_invariant": "preserved",
                    "query": "compression query should not export"
                }
            ],
            "entries": [{
                "role": "developer",
                "tier": "summary",
                "source": "turn_context:developer:summary:0",
                "replay_key": "turn_context:developer:summary:0:aaaaaaaaaaaaaaaa",
                "text_hash": "aaaaaaaaaaaaaaaa",
                "estimated_tokens": 10
            }]
        }
    });
    let export = summarize_rollout_context_debug_jsonl(&format!("{context}\n"));

    assert!(export.audit.ok);
    assert_eq!(
        export
            .summary
            .latest_manifest_compression_stage_schema_version,
        2
    );
    assert_eq!(export.summary.latest_manifest_compression_stage_count, 2);
    assert_eq!(
        export.summary.latest_manifest_compression_stages,
        vec!["summary".to_string(), "prune".to_string()]
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_compression_loss_check_statuses,
        vec!["marker_boundary_only".to_string()]
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_compression_rollback_source_text_hash_count,
        2
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_compression_protected_tier_invariants,
        vec!["preserved".to_string()]
    );
    assert_eq!(export.summary.latest_manifest_compression_input_tokens, 52);
    assert_eq!(export.summary.latest_manifest_compression_output_tokens, 22);
    assert_eq!(export.summary.latest_manifest_compression_tokens_saved, 30);
    assert_eq!(
        export.summary.latest_manifest_compression_affected_entries,
        3
    );
    assert!(!export.summary.latest_manifest_compression_invalid);

    let rendered = serde_json::to_string(&export).expect("export serializes");
    assert!(!rendered.contains("compression-source-id-should-not-export"));
    assert!(!rendered.contains("compression prompt payload should not export"));
    assert!(!rendered.contains("compression query should not export"));
    assert!(!rendered.contains("bbbbbbbbbbbbbbbb"));
    assert!(!rendered.contains("cccccccccccccccc"));
}

#[test]
fn rollout_context_debug_summary_surfaces_executed_compression_stage_matrix_without_payloads() {
    let response_item = json!({
        "type": "response_item",
        "payload": {
            "type": "message",
            "role": "developer",
            "content": [{
                "type": "input_text",
                "text": "[context summarized for budget] rollout prompt text should not export"
            }],
            "source_id": "response-item-source-id-should-not-export",
            "turn_context_policy_canary_feature": "source_aware_compression_canary",
            "turn_context_policy_opt_in_marker": "TurnContextAssemblyPolicyOptIn::SourceAwareCompression",
            "turn_context_policy_opt_in_value": "SourceAwareCompression"
        }
    });
    let context = json!({
        "type": "turn_context",
        "payload": {
            "model": "gpt-test",
            "turn_context_policy_canary_feature": "source_aware_compression_canary",
            "turn_context_policy_opt_in_marker": "TurnContextAssemblyPolicyOptIn::SourceAwareCompression",
            "turn_context_policy_opt_in_value": "SourceAwareCompression",
            "context_manifest": {
                "version": 1,
                "estimated_tokens": 43,
                "budget_tokens": 20,
                "turn_context_policy_canary_feature": "source_aware_compression_canary",
                "turn_context_policy_opt_in_marker": "TurnContextAssemblyPolicyOptIn::SourceAwareCompression",
                "turn_context_policy_opt_in_value": "SourceAwareCompression",
                "compression_stages": [
                    {
                        "kind": "summary",
                        "input_tokens": 40,
                        "output_tokens": 12,
                        "affected_entries": 1,
                        "loss_check_status": "marker_boundary_only",
                        "rollback_source_text_hash": "bbbbbbbbbbbbbbbb",
                        "protected_tier_invariant": "preserved",
                        "source_id": "summary-stage-source-id-should-not-export",
                        "replay_key": "summary-stage-replay-key-should-not-export",
                        "text_hash": "summary-stage-text-hash-should-not-export",
                        "text": "[context summarized for budget] stage payload should not export"
                    },
                    {
                        "kind": "defragment",
                        "input_tokens": 30,
                        "output_tokens": 21,
                        "affected_entries": 1,
                        "loss_check_status": "marker_boundary_only",
                        "rollback_source_text_hash": "cccccccccccccccc",
                        "protected_tier_invariant": "preserved",
                        "source_id": "defragment-stage-source-id-should-not-export",
                        "text": "[context defragmented for budget] stage payload should not export"
                    },
                    {
                        "kind": "prune",
                        "input_tokens": 12,
                        "output_tokens": 10,
                        "affected_entries": 1,
                        "loss_check_status": "marker_boundary_only",
                        "rollback_source_text_hash": "dddddddddddddddd",
                        "protected_tier_invariant": "preserved",
                        "query": "prune stage query should not export",
                        "text": "[context pruned for budget] stage payload should not export"
                    }
                ],
                "entries": [{
                    "role": "developer",
                    "tier": "tool",
                    "source": "turn_context:developer:compression_stage_matrix:0",
                    "replay_key": "turn_context:developer:compression_stage_matrix:0:aaaaaaaaaaaaaaaa",
                    "text_hash": "aaaaaaaaaaaaaaaa",
                    "estimated_tokens": 43
                }]
            }
        }
    });
    let export = summarize_rollout_context_debug_jsonl(&format!("{response_item}\n{context}\n"));

    assert!(export.audit.ok);
    assert_eq!(export.summary.line_count, 2);
    assert_eq!(export.summary.manifest_count, 1);
    assert_eq!(
        export
            .summary
            .latest_manifest_compression_stage_schema_version,
        2
    );
    assert_eq!(export.summary.latest_manifest_compression_stage_count, 3);
    assert_eq!(
        export.summary.latest_manifest_compression_stages,
        vec![
            "summary".to_string(),
            "defragment".to_string(),
            "prune".to_string()
        ]
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_compression_loss_check_statuses,
        vec!["marker_boundary_only".to_string()]
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_compression_rollback_source_text_hash_count,
        3
    );
    assert_eq!(
        export
            .summary
            .latest_manifest_compression_protected_tier_invariants,
        vec!["preserved".to_string()]
    );
    assert_eq!(export.summary.latest_manifest_compression_input_tokens, 82);
    assert_eq!(export.summary.latest_manifest_compression_output_tokens, 43);
    assert_eq!(export.summary.latest_manifest_compression_tokens_saved, 39);
    assert_eq!(
        export.summary.latest_manifest_compression_affected_entries,
        3
    );
    assert!(!export.summary.latest_manifest_compression_invalid);
    assert_eq!(
        export.summary.latest_manifest_compression_candidate_count,
        0
    );

    let rendered = serde_json::to_string(&export).expect("export serializes");
    assert!(!rendered.contains("response-item-source-id-should-not-export"));
    assert!(!rendered.contains("rollout prompt text should not export"));
    assert!(!rendered.contains("source_aware_compression_canary"));
    assert!(!rendered.contains("TurnContextAssemblyPolicyOptIn"));
    assert!(!rendered.contains("SourceAwareCompression"));
    assert!(!rendered.contains("summary-stage-source-id-should-not-export"));
    assert!(!rendered.contains("summary-stage-replay-key-should-not-export"));
    assert!(!rendered.contains("summary-stage-text-hash-should-not-export"));
    assert!(!rendered.contains("defragment-stage-source-id-should-not-export"));
    assert!(!rendered.contains("prune stage query should not export"));
    assert!(!rendered.contains("bbbbbbbbbbbbbbbb"));
    assert!(!rendered.contains("cccccccccccccccc"));
    assert!(!rendered.contains("dddddddddddddddd"));
    assert!(!rendered.contains("[context summarized for budget]"));
    assert!(!rendered.contains("[context defragmented for budget]"));
    assert!(!rendered.contains("[context pruned for budget]"));
}

#[test]
fn rollout_context_debug_audit_rejects_invalid_compression_stage() {
    let context = json!({
        "type": "turn_context_manifest",
        "payload": {
            "version": 1,
            "estimated_tokens": 10,
            "compression_stages": [{
                "kind": "rewrite",
                "input_tokens": 12,
                "output_tokens": 4,
                "affected_entries": 1,
                "loss_check_status": "marker_boundary_only",
                "rollback_source_text_hash": "not-a-stable-hash",
                "protected_tier_invariant": "preserved"
            }],
            "entries": [{
                "role": "developer",
                "tier": "summary",
                "source": "turn_context:developer:summary:0",
                "replay_key": "turn_context:developer:summary:0:aaaaaaaaaaaaaaaa",
                "text_hash": "aaaaaaaaaaaaaaaa",
                "estimated_tokens": 10
            }]
        }
    });
    let export = summarize_rollout_context_debug_jsonl(&format!("{context}\n"));

    assert!(!export.audit.ok);
    assert!(export.summary.latest_manifest_compression_invalid);
    assert!(finding_codes(&export.audit).contains(&"manifest_compression_stages_invalid"));
}

#[test]
fn rollout_context_debug_accepts_unranked_selected_source_diversity() {
    let context = json!({
        "type": "turn_context_manifest",
        "payload": {
            "version": 1,
            "estimated_tokens": 3,
            "budget_tokens": 4,
            "recall_selection": {
                "returned_source_count": 2,
                "selected_source_count": 2,
                "ranked_source_count": 0,
                "returned_unselected_source_count": 0,
                "source_diversity_met": true,
                "source_diversity_target": 2,
                "ranked_item_count": 0,
                "memory_control_omitted_count": 0,
                "low_trust_ranked_item_count": 0,
                "low_recency_ranked_item_count": 0
            },
            "entries": [{
                "role": "developer",
                "source": "initial_context:permissions:0",
                "replay_key": "initial_context:permissions:0:aaaaaaaaaaaaaaaa",
                "text_hash": "aaaaaaaaaaaaaaaa",
                "estimated_tokens": 3
            }]
        }
    });

    let export = summarize_rollout_context_debug_jsonl(&format!("{context}\n"));

    assert!(export.audit.ok);
    assert!(!export.summary.latest_manifest_recall_selection_invalid);
    assert_eq!(
        export.summary.latest_manifest_recall_selected_source_count,
        2
    );
    assert_eq!(export.summary.latest_manifest_recall_ranked_source_count, 0);
    assert_eq!(export.summary.latest_manifest_recall_ranked_item_count, 0);
}

#[test]
fn rollout_context_debug_audit_rejects_source_diversity_mismatch() {
    let context = json!({
        "type": "turn_context_manifest",
        "payload": {
            "version": 1,
            "estimated_tokens": 3,
            "budget_tokens": 4,
            "recall_selection": {
                "returned_source_count": 2,
                "selected_source_count": 2,
                "ranked_source_count": 0,
                "returned_unselected_source_count": 0,
                "source_diversity_met": false,
                "source_diversity_target": 2,
                "ranked_item_count": 0,
                "memory_control_omitted_count": 0,
                "low_trust_ranked_item_count": 0,
                "low_recency_ranked_item_count": 0
            },
            "entries": [{
                "role": "developer",
                "source": "initial_context:permissions:0",
                "replay_key": "initial_context:permissions:0:aaaaaaaaaaaaaaaa",
                "text_hash": "aaaaaaaaaaaaaaaa",
                "estimated_tokens": 3
            }]
        }
    });

    let export = summarize_rollout_context_debug_jsonl(&format!("{context}\n"));

    assert!(!export.audit.ok);
    assert!(export.summary.latest_manifest_recall_selection_invalid);
    assert!(finding_codes(&export.audit).contains(&"manifest_recall_selection_invalid"));
}

#[test]
fn rollout_context_debug_audit_rejects_ranked_items_without_ranked_sources() {
    let context = json!({
        "type": "turn_context_manifest",
        "payload": {
            "version": 1,
            "estimated_tokens": 3,
            "budget_tokens": 4,
            "recall_selection": {
                "returned_source_count": 2,
                "selected_source_count": 1,
                "ranked_source_count": 0,
                "returned_unselected_source_count": 1,
                "source_diversity_met": false,
                "source_diversity_target": 2,
                "ranked_item_count": 1,
                "memory_control_omitted_count": 0,
                "low_trust_ranked_item_count": 0,
                "low_recency_ranked_item_count": 0
            },
            "entries": [{
                "role": "developer",
                "source": "initial_context:permissions:0",
                "replay_key": "initial_context:permissions:0:aaaaaaaaaaaaaaaa",
                "text_hash": "aaaaaaaaaaaaaaaa",
                "estimated_tokens": 3
            }]
        }
    });

    let export = summarize_rollout_context_debug_jsonl(&format!("{context}\n"));

    assert!(!export.audit.ok);
    assert!(export.summary.latest_manifest_recall_selection_invalid);
    assert!(finding_codes(&export.audit).contains(&"manifest_recall_selection_invalid"));
}

#[test]
fn rollout_context_debug_audit_rejects_invalid_selected_snippet_envelope() {
    let context = json!({
        "type": "turn_context_manifest",
        "payload": {
            "version": 1,
            "estimated_tokens": 3,
            "recall_selected_snippets": {
                "version": 1,
                "max_snippets": 4,
                "max_snippet_chars": 120,
                "selected_snippet_count": 2,
                "omitted_snippet_count": 0,
                "redacted_snippet_count": 1,
                "truncated_snippet_count": 0,
                "snippets": [{
                    "snippet_hash": "fedcba9876543210",
                    "text": "[redacted-query] bounded memory",
                    "estimated_tokens": 8,
                    "redacted": true,
                    "truncated": false
                }],
                "safety": {
                    "ready_for_shadow_handoff": true,
                    "bounded": true,
                    "origin_identifiers_exposed": false,
                    "raw_ranked_payload_exposed": false,
                    "rank_explanation_exposed": false,
                    "control_marker_exposed": false,
                    "query_payload_exposed": false,
                    "per_origin_list_exposed": false
                }
            },
            "entries": [{
                "role": "developer",
                "source": "initial_context:permissions:0",
                "replay_key": "initial_context:permissions:0:aaaaaaaaaaaaaaaa",
                "text_hash": "aaaaaaaaaaaaaaaa",
                "estimated_tokens": 3
            }]
        }
    });

    let export = summarize_rollout_context_debug_jsonl(&format!("{context}\n"));

    assert!(!export.audit.ok);
    assert!(
        export
            .summary
            .latest_manifest_recall_selected_snippets_invalid
    );
    assert!(finding_codes(&export.audit).contains(&"manifest_recall_selected_snippets_invalid"));
}

#[test]
fn rollout_context_debug_audit_rejects_truncated_manifest_without_evidence() {
    let context = json!({
        "type": "turn_context_manifest",
        "payload": {
            "version": 1,
            "estimated_tokens": 3,
            "truncated": true,
            "entries": [{
                "role": "developer",
                "source": "turn_context:developer:selected_context_recall:0",
                "replay_key": "turn_context:developer:selected_context_recall:0:cccccccccccccccc",
                "text_hash": "cccccccccccccccc",
                "estimated_tokens": 3
            }]
        }
    });

    let export = summarize_rollout_context_debug_jsonl(&format!("{context}\n"));

    assert!(!export.audit.ok);
    assert!(export.summary.latest_manifest_truncated);
    assert_eq!(export.summary.latest_manifest_truncated_decision_count, 0);
    assert!(!export.summary.latest_manifest_truncation_evidence_present);
    assert!(finding_codes(&export.audit).contains(&"manifest_truncation_evidence_missing"));
}

#[test]
fn rollout_context_debug_audit_rejects_truncation_evidence_without_manifest_flag() {
    let context = json!({
        "type": "turn_context_manifest",
        "payload": {
            "version": 1,
            "estimated_tokens": 3,
            "decision_ledger": [{
                "source": "turn_context:developer:selected_context_recall:0",
                "decision": "truncated:selected_context_recall:original_tokens:24:tokens:3",
                "reason_hash": "bbbbbbbbbbbbbbbb"
            }],
            "entries": [{
                "role": "developer",
                "source": "turn_context:developer:selected_context_recall:0",
                "replay_key": "turn_context:developer:selected_context_recall:0:cccccccccccccccc",
                "text_hash": "cccccccccccccccc",
                "estimated_tokens": 3
            }]
        }
    });

    let export = summarize_rollout_context_debug_jsonl(&format!("{context}\n"));

    assert!(!export.audit.ok);
    assert!(!export.summary.latest_manifest_truncated);
    assert!(export.summary.latest_manifest_truncation_evidence_present);
    assert!(finding_codes(&export.audit).contains(&"manifest_truncation_evidence_unexpected"));
}

#[test]
fn rollout_context_debug_audit_rejects_malformed_truncation_evidence() {
    let context = json!({
        "type": "turn_context_manifest",
        "payload": {
            "version": 1,
            "estimated_tokens": 3,
            "truncated": true,
            "decision_ledger": [{
                "source": "turn_context:developer:selected_context_recall:0",
                "decision": "truncated:selected_context_recall:original_tokens:3:tokens:24",
                "reason_hash": "bbbbbbbbbbbbbbbb"
            }],
            "entries": [{
                "role": "developer",
                "source": "turn_context:developer:selected_context_recall:0",
                "replay_key": "turn_context:developer:selected_context_recall:0:cccccccccccccccc",
                "text_hash": "cccccccccccccccc",
                "estimated_tokens": 3
            }]
        }
    });

    let export = summarize_rollout_context_debug_jsonl(&format!("{context}\n"));

    assert!(!export.audit.ok);
    assert!(export.summary.latest_manifest_truncation_evidence_invalid);
    assert!(finding_codes(&export.audit).contains(&"manifest_truncation_evidence_invalid"));
}

fn finding_codes(audit: &RolloutContextDebugAudit) -> Vec<&'static str> {
    audit.findings.iter().map(|finding| finding.code).collect()
}
