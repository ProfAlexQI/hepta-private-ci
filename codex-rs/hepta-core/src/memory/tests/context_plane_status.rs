use super::*;

fn context_plane_status_report_fixture(
    allocator_shadow: &ContextMemoryAdaptiveAllocatorEvalShadowReport,
    recall_quality_gate: &ContextMemoryRecallQualityGateReport,
) -> ContextPlaneStatusReport {
    let taxonomy = ContextMemoryTaxonomyReport {
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
                class: ContextMemoryTaxonomyClass::Transcript,
                source_count: 1,
                returned_count: 1,
                available_count: 1,
                omitted_count: 0,
                provenance_span_count: 1,
            },
        ],
    };
    let formation_receipts = ContextMemoryFormationReceiptReport {
        receipts: vec![ContextMemoryFormationReceipt {
            candidate_type: ContextMemoryFormationCandidateType::Fact,
            transcript_span_count: 1,
            provenance_span_count: 1,
            confidence_basis_points: 6400,
            idempotency_key_hash: stable_receipt_hash(&[
                "memory_formation",
                "fact",
                "status-test",
                "1",
                "1",
            ]),
            privacy_class: "user_private".into(),
            queued_for_background: true,
            production_write: false,
        }],
    };
    let formation_queue = ContextMemoryFormationQueueReport::from_receipts(&formation_receipts);
    let temporal_facts = ContextMemoryTemporalFactReport {
        facts: vec![ContextMemoryTemporalFact {
            fact_type: ContextMemoryTemporalFactType::Attribute,
            entity_hash: stable_receipt_hash(&[
                "memory_temporal_fact_entity",
                "attribute",
                "status-test",
                "1",
                "1",
            ]),
            provenance_span_count: 1,
            valid_from_sequence: 1,
            invalid_at_sequence: None,
            confidence_basis_points: 6200,
            supersedes_fact_hash: None,
            privacy_class: "user_private".into(),
            dry_run_only: true,
            production_write: false,
        }],
    };
    let temporal_fact_graph =
        ContextMemoryTemporalFactGraphReport::from_temporal_facts(&temporal_facts);
    let temporal_graph_shadow_eval = ContextMemoryTemporalGraphShadowEvalReport::seeded();
    let eval_seed = ContextMemoryEvalHarnessReport::seeded();
    let provider_report = MemoryProviderReport::from_update(
        MemoryProviderDescriptor::builtin(),
        MemoryProviderContextUpdateEnvelope {
            provider_id: "builtin".into(),
            mode: MemoryProviderContextUpdateMode::ShadowOnly,
            source_counts: ContextRecallSourceCounts::default(),
            limit_pressure: ContextRecallLimitPressure::default(),
            ranked_item_count: 2,
            selected_item_count: 1,
            estimated_token_count: 256,
            payload_light: true,
            operator_approval_required: true,
            prompt_payload_exported: false,
            query_payload_exported: false,
            ranked_payload_exported: false,
            write_performed: false,
            runtime_activation: false,
        },
    );
    let ranked_recall = ContextMemoryRankedRecallShadowEvalReport::seeded();
    let dashboard = ContextMemoryShadowRegressionDashboardReport::from_reports(
        &ranked_recall,
        &temporal_graph_shadow_eval,
        recall_quality_gate,
        &provider_report,
    );
    let shadow_quality_summary =
        ContextMemoryShadowQualitySummaryReport::from_dashboard(&dashboard);
    let shadow_quality_trend_snapshot =
        ContextMemoryShadowQualityTrendSnapshotReport::from_summary(&shadow_quality_summary);

    ContextPlaneStatusReport::from_reports(ContextPlaneStatusReportInput {
        taxonomy: &taxonomy,
        formation_receipts: &formation_receipts,
        formation_queue: &formation_queue,
        temporal_facts: &temporal_facts,
        temporal_fact_graph: &temporal_fact_graph,
        temporal_graph_shadow_eval: &temporal_graph_shadow_eval,
        eval_seed: &eval_seed,
        allocator_shadow,
        recall_quality_gate,
        provider_report: &provider_report,
        shadow_quality_trend_snapshot: &shadow_quality_trend_snapshot,
    })
}

#[test]
fn context_plane_status_report_unifies_readiness_without_payloads_or_activation() {
    let allocator_shadow = ContextMemoryAdaptiveAllocatorEvalShadowReport::seeded();
    let recall_quality_gate = ContextMemoryRecallQualityGateReport::from_shadow(&allocator_shadow);
    let report = context_plane_status_report_fixture(&allocator_shadow, &recall_quality_gate);

    assert!(report.has_status_integrity());
    assert_eq!(report.sections.len(), 14);
    assert_eq!(report.ready_section_count(), 8);
    assert_eq!(report.shadow_section_count(), 5);
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
        report.section_status(ContextPlaneStatusSection::MemoryShadowCanaryReadiness),
        Some(ContextPlaneStatusKind::Shadow)
    );
    assert_eq!(
        report.section_status(ContextPlaneStatusSection::MemoryTemporalGraphShadowEval),
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
    assert!(json.contains("memory_temporal_fact_graph"));
    assert!(json.contains("memory_temporal_graph_shadow_eval"));
    assert!(json.contains("eval_harness_seed"));
    assert!(json.contains("adaptive_allocator_eval_shadow"));
    assert!(json.contains("recall_quality_gate"));
    assert!(json.contains("memory_provider_boundary"));
    assert!(json.contains("memory_shadow_canary_readiness"));
    assert!(json.contains("recall_quality_blocking_reason_count"));
    assert!(json.contains("recall_quality_blocking_reasons"));
    assert!(json.contains("source_aware_front_door"));
    assert!(!json.contains("prompt_text"));
    assert!(!json.contains("transcript_text"));
    assert!(!json.contains("memory_text"));
    assert!(!json.contains("answer_text"));
    assert!(!json.contains("source_id"));
    assert!(!json.contains("session-"));
    assert!(!json.contains("memory-"));
    assert!(!json.contains("status-test"));
    assert!(!json.contains("\"production_write\":true"));
    assert!(!json.contains("\"graph_write\":true"));
    assert!(!json.contains("\"runtime_activation\":true"));
    assert!(!json.contains("\"adaptive_allocator_runtime_activation\":true"));
    assert!(!json.contains("\"source_aware_runtime_activation\":true"));
    assert!(!json.contains("\"prompt_assembly_change\":true"));
    assert!(!json.contains("\"operator_activation_allowed\":true"));
}

#[test]
fn context_plane_status_report_rolls_up_recall_quality_blockers_without_payloads() {
    let allocator_shadow = ContextMemoryAdaptiveAllocatorEvalShadowReport::seeded();
    let mut recall_quality_shadow = allocator_shadow.clone();
    let proposed_redacted = recall_quality_shadow
        .shadow_results
        .iter_mut()
        .find(|result| {
            result.arm == ContextMemoryAdaptiveAllocatorEvalArm::ProposedAdaptive
                && result.fixture_kind == ContextMemoryEvalFixtureKind::RedactedTrace
        })
        .expect("proposed redacted trace fixture should exist");
    proposed_redacted.answer_quality_regression_count = 2;
    proposed_redacted.prompt_assembly_change = true;

    let recall_quality_gate =
        ContextMemoryRecallQualityGateReport::from_shadow(&recall_quality_shadow);
    let report = context_plane_status_report_fixture(&allocator_shadow, &recall_quality_gate);

    assert!(!report.has_status_integrity());
    assert_eq!(
        report.section_status(ContextPlaneStatusSection::RecallQualityGate),
        Some(ContextPlaneStatusKind::Blocked)
    );
    assert_eq!(report.blocker_count(), 21);
    assert!(!report.production_write);
    assert!(!report.graph_write);
    assert!(!report.runtime_activation);
    assert!(!report.adaptive_allocator_runtime_activation);
    assert!(!report.source_aware_runtime_activation);
    assert!(report.prompt_assembly_change);
    assert!(!report.operator_activation_allowed);

    let recall_entry = report
        .sections
        .iter()
        .find(|entry| entry.section == ContextPlaneStatusSection::RecallQualityGate)
        .expect("recall quality status row should exist");
    assert_eq!(recall_entry.status, ContextPlaneStatusKind::Blocked);
    assert_eq!(recall_entry.blocker_count, 2);
    assert_eq!(recall_entry.recall_quality_blocking_reason_count, 2);
    assert_eq!(
        recall_entry.recall_quality_blocking_reasons,
        vec![
            ContextMemoryRecallQualityGateBlockerReason::AnswerQualityRegression,
            ContextMemoryRecallQualityGateBlockerReason::SideEffectFlagEnabled,
        ]
    );
    assert!(!recall_entry.production_write);
    assert!(!recall_entry.graph_write);
    assert!(!recall_entry.runtime_activation);
    assert!(recall_entry.prompt_assembly_change);
    assert!(!recall_entry.operator_activation_allowed);
    let canary_entry = report
        .sections
        .iter()
        .find(|entry| entry.section == ContextPlaneStatusSection::MemoryShadowCanaryReadiness)
        .expect("memory shadow canary readiness status row should exist");
    assert_eq!(canary_entry.status, ContextPlaneStatusKind::Blocked);
    assert_eq!(canary_entry.blocker_count, 19);
    assert_eq!(canary_entry.omitted_count, 19);

    let json = serde_json::to_string(&report).expect("context plane status should serialize");
    assert!(json.contains("recall_quality_blocking_reason_count"));
    assert!(json.contains("recall_quality_blocking_reasons"));
    assert!(json.contains("answer_quality_regression"));
    assert!(json.contains("side_effect_flag_enabled"));
    assert!(json.contains("\"prompt_assembly_change\":true"));
    assert!(!json.contains("fixture_id_hash"));
    assert!(!json.contains("source_id"));
    assert!(!json.contains("session-"));
    assert!(!json.contains("memory-"));
    assert!(!json.contains("status-test"));
    assert!(!json.contains("transcript_text"));
    assert!(!json.contains("memory_text"));
    assert!(!json.contains("prompt_text"));
    assert!(!json.contains("answer_text"));
    assert!(!json.contains("\"production_write\":true"));
    assert!(!json.contains("\"graph_write\":true"));
    assert!(!json.contains("\"runtime_activation\":true"));
    assert!(!json.contains("\"adaptive_allocator_runtime_activation\":true"));
    assert!(!json.contains("\"source_aware_runtime_activation\":true"));
    assert!(!json.contains("\"operator_activation_allowed\":true"));
}
