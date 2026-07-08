use super::*;

#[test]
fn context_recall_memory_taxonomy_report_maps_recall_counts_without_payloads() {
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
    let source_availability = ContextRecallSourceAvailability {
        recent_entry_count: 4,
        transcript_match_count: 3,
        durable_memory_match_count: 2,
        summary_memory_match_count: 3,
    };

    let taxonomy = inspection.memory_taxonomy_report(&source_availability, 2);

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
                    returned_count: 1,
                    available_count: 3,
                    omitted_count: 2,
                    provenance_span_count: 0,
                },
                ContextMemoryTaxonomyBucket {
                    class: ContextMemoryTaxonomyClass::Control,
                    source_count: 1,
                    returned_count: 0,
                    available_count: 2,
                    omitted_count: 2,
                    provenance_span_count: 0,
                },
                ContextMemoryTaxonomyBucket {
                    class: ContextMemoryTaxonomyClass::Transcript,
                    source_count: 2,
                    returned_count: 3,
                    available_count: 7,
                    omitted_count: 4,
                    provenance_span_count: 2,
                },
            ],
        }
    );

    let json = serde_json::to_string(&taxonomy).expect("taxonomy report should serialize");
    assert!(json.contains("semantic"));
    assert!(json.contains("episodic"));
    assert!(json.contains("control"));
    assert!(json.contains("transcript"));
    assert!(!json.contains("memory-"));
    assert!(!json.contains("payload"));
    assert!(!json.contains("source_id"));
    assert!(!json.contains("query_text"));
}

#[test]
fn context_memory_namespace_policy_report_defines_shadow_blocks_without_payloads() {
    let report = ContextMemoryNamespacePolicyReport::seeded();

    assert!(report.has_policy_integrity());
    assert_eq!(
        report.schema_version,
        CONTEXT_MEMORY_NAMESPACE_POLICY_SCHEMA_VERSION
    );
    assert_eq!(report.namespace_count(), 6);
    assert_eq!(report.operator_approval_required_count(), 6);
    assert_eq!(report.shadow_wal_required_count(), 6);
    assert_eq!(report.readback_required_count(), 6);
    assert_eq!(report.canary_required_count(), 6);
    assert_eq!(report.rollback_supported_count(), 6);
    assert_eq!(report.production_write_count(), 0);
    assert_eq!(report.graph_write_count(), 0);
    assert!(!report.production_write);
    assert!(!report.graph_write);
    assert!(!report.hot_path_write);
    assert!(!report.prompt_assembly_change);
    assert!(!report.runtime_activation);

    let namespaces = report
        .blocks
        .iter()
        .map(|block| block.namespace.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        namespaces,
        vec![
            "core",
            "session",
            "procedural",
            "semantic",
            "episodic",
            "archival"
        ]
    );
    assert!(
        report
            .blocks
            .iter()
            .any(|block| block.namespace == ContextMemoryNamespace::Core
                && block.owner == ContextMemoryNamespaceOwner::OperatorPolicy
                && block.ttl_policy == ContextMemoryNamespaceTtlPolicy::Indefinite
                && block.ttl_turns == 0
                && block.privacy_tier == ContextMemoryNamespacePrivacyTier::OperatorControlled
                && block.redaction_policy == ContextMemoryNamespaceRedactionPolicy::OperatorReview
                && block.write_policy == ContextMemoryNamespaceWritePolicy::ShadowProposalOnly)
    );
    assert!(
        report
            .blocks
            .iter()
            .any(|block| block.namespace == ContextMemoryNamespace::Archival
                && block.owner == ContextMemoryNamespaceOwner::ArchivalStore
                && block.ttl_policy == ContextMemoryNamespaceTtlPolicy::Archival
                && block.ttl_turns == 32768
                && block.rollback_supported)
    );

    let json = serde_json::to_string(&report).expect("namespace policy report should serialize");
    for namespace in [
        "core",
        "session",
        "procedural",
        "semantic",
        "episodic",
        "archival",
    ] {
        assert!(json.contains(namespace));
    }
    assert!(json.contains("shadow_proposal_only"));
    assert!(json.contains("operator_approval_required"));
    assert!(json.contains("shadow_wal_required"));
    assert!(!json.contains("candidate_text"));
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
fn context_memory_namespace_policy_report_rejects_write_or_namespace_drift() {
    let mut write_drift = ContextMemoryNamespacePolicyReport::seeded();
    write_drift.blocks[0].production_write = true;
    assert!(!write_drift.has_policy_integrity());

    let mut missing_namespace = ContextMemoryNamespacePolicyReport::seeded();
    missing_namespace.blocks.pop();
    assert!(!missing_namespace.has_policy_integrity());

    let mut duplicate_namespace = ContextMemoryNamespacePolicyReport::seeded();
    duplicate_namespace.blocks[0].namespace = ContextMemoryNamespace::Session;
    assert!(!duplicate_namespace.has_policy_integrity());
}
