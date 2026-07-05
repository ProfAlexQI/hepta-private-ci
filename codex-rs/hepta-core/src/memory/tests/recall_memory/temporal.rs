use super::*;

#[test]
fn context_recall_memory_temporal_facts_are_payload_light_and_non_writing() {
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

    let report = inspection.memory_temporal_fact_report();

    assert!(report.has_temporal_fact_integrity());
    assert_eq!(
        report,
        ContextMemoryTemporalFactReport {
            facts: vec![
                ContextMemoryTemporalFact {
                    fact_type: ContextMemoryTemporalFactType::Attribute,
                    entity_hash: stable_receipt_hash(&[
                        "memory_temporal_fact_entity",
                        "attribute",
                        "session-42",
                        "2",
                        "8",
                    ]),
                    provenance_span_count: 2,
                    valid_from_sequence: 8,
                    invalid_at_sequence: None,
                    confidence_basis_points: 6200,
                    supersedes_fact_hash: None,
                    privacy_class: "user_private".into(),
                    dry_run_only: true,
                    production_write: false,
                },
                ContextMemoryTemporalFact {
                    fact_type: ContextMemoryTemporalFactType::Preference,
                    entity_hash: stable_receipt_hash(&[
                        "memory_temporal_fact_entity",
                        "preference",
                        "session-42",
                        "2",
                        "8",
                    ]),
                    provenance_span_count: 2,
                    valid_from_sequence: 8,
                    invalid_at_sequence: None,
                    confidence_basis_points: 5600,
                    supersedes_fact_hash: None,
                    privacy_class: "user_private".into(),
                    dry_run_only: true,
                    production_write: false,
                },
                ContextMemoryTemporalFact {
                    fact_type: ContextMemoryTemporalFactType::TaskState,
                    entity_hash: stable_receipt_hash(&[
                        "memory_temporal_fact_entity",
                        "task_state",
                        "session-42",
                        "2",
                        "8",
                    ]),
                    provenance_span_count: 2,
                    valid_from_sequence: 8,
                    invalid_at_sequence: None,
                    confidence_basis_points: 5400,
                    supersedes_fact_hash: None,
                    privacy_class: "user_private".into(),
                    dry_run_only: true,
                    production_write: false,
                },
                ContextMemoryTemporalFact {
                    fact_type: ContextMemoryTemporalFactType::Decision,
                    entity_hash: stable_receipt_hash(&[
                        "memory_temporal_fact_entity",
                        "decision",
                        "session-42",
                        "2",
                        "8",
                    ]),
                    provenance_span_count: 2,
                    valid_from_sequence: 8,
                    invalid_at_sequence: None,
                    confidence_basis_points: 5800,
                    supersedes_fact_hash: None,
                    privacy_class: "user_private".into(),
                    dry_run_only: true,
                    production_write: false,
                },
                ContextMemoryTemporalFact {
                    fact_type: ContextMemoryTemporalFactType::Summary,
                    entity_hash: stable_receipt_hash(&[
                        "memory_temporal_fact_entity",
                        "summary",
                        "session-42",
                        "2",
                        "8",
                    ]),
                    provenance_span_count: 2,
                    valid_from_sequence: 8,
                    invalid_at_sequence: None,
                    confidence_basis_points: 7000,
                    supersedes_fact_hash: None,
                    privacy_class: "user_private".into(),
                    dry_run_only: true,
                    production_write: false,
                },
            ],
        }
    );

    let json = serde_json::to_string(&report).expect("temporal fact report should serialize");
    assert!(json.contains("attribute"));
    assert!(json.contains("preference"));
    assert!(json.contains("task_state"));
    assert!(json.contains("decision"));
    assert!(json.contains("summary"));
    assert!(!json.contains("timeout"));
    assert!(!json.contains("session-42"));
    assert!(!json.contains("fact_text"));
    assert!(!json.contains("transcript_text"));
    assert!(!json.contains("memory_text"));
    assert!(!json.contains("source_id"));
    assert!(!json.contains("memory_id"));
    assert!(!json.contains("query_text"));
    assert!(!json.contains("\"production_write\":true"));
}

#[test]
fn context_recall_memory_temporal_fact_graph_is_payload_light_reversible_and_non_writing() {
    let superseded_fact_hash = stable_receipt_hash(&["memory_temporal_fact_graph_prior"]);
    let fact_report = ContextMemoryTemporalFactReport {
        facts: vec![
            ContextMemoryTemporalFact {
                fact_type: ContextMemoryTemporalFactType::Attribute,
                entity_hash: stable_receipt_hash(&[
                    "memory_temporal_fact_entity",
                    "attribute",
                    "session-graph",
                ]),
                provenance_span_count: 2,
                valid_from_sequence: 8,
                invalid_at_sequence: None,
                confidence_basis_points: 6200,
                supersedes_fact_hash: None,
                privacy_class: "user_private".into(),
                dry_run_only: true,
                production_write: false,
            },
            ContextMemoryTemporalFact {
                fact_type: ContextMemoryTemporalFactType::Preference,
                entity_hash: stable_receipt_hash(&[
                    "memory_temporal_fact_entity",
                    "preference",
                    "session-graph",
                ]),
                provenance_span_count: 1,
                valid_from_sequence: 9,
                invalid_at_sequence: Some(12),
                confidence_basis_points: 5600,
                supersedes_fact_hash: Some(superseded_fact_hash.clone()),
                privacy_class: "user_private".into(),
                dry_run_only: true,
                production_write: false,
            },
        ],
    };

    let graph = ContextMemoryTemporalFactGraphReport::from_temporal_facts(&fact_report);

    assert!(graph.has_graph_integrity());
    assert_eq!(
        graph.schema_version,
        CONTEXT_MEMORY_TEMPORAL_FACT_GRAPH_SCHEMA_VERSION
    );
    assert_eq!(graph.nodes.len(), 2);
    assert_eq!(graph.edges.len(), 5);
    assert_eq!(graph.provenance_edge_count(), 2);
    assert_eq!(graph.validity_window_edge_count(), 2);
    assert_eq!(graph.supersedes_edge_count(), 1);
    assert_eq!(graph.open_node_count(), 1);
    assert_eq!(graph.invalidated_node_count(), 1);
    assert!(!graph.production_write);
    assert!(!graph.graph_write);
    assert!(!graph.runtime_activation);
    assert!(!graph.prompt_assembly_change);
    assert!(graph.nodes.iter().all(|node| {
        node.fact_hash.len() == 16
            && node.dry_run_only
            && !node.production_write
            && !node.graph_write
    }));
    assert!(graph.edges.iter().all(|edge| {
        edge.edge_hash.len() == 16
            && edge.from_fact_hash.len() == 16
            && edge.dry_run_only
            && !edge.production_write
            && !edge.graph_write
    }));

    let json = serde_json::to_string(&graph).expect("temporal fact graph should serialize");
    assert!(json.contains("validity_window"));
    assert!(json.contains("provenance"));
    assert!(json.contains("supersedes"));
    assert!(json.contains("fact_hash"));
    assert!(json.contains("edge_hash"));
    assert!(!json.contains("entity_hash"));
    assert!(!json.contains("session-graph"));
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
    assert!(!json.contains("\"prompt_assembly_change\":true"));
}
