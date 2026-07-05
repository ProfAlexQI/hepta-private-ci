use super::*;

#[test]
fn context_memory_recall_quality_gate_enforces_thresholds_without_activation() {
    let shadow = ContextMemoryAdaptiveAllocatorEvalShadowReport::seeded();
    let report = ContextMemoryRecallQualityGateReport::from_shadow(&shadow);

    assert!(report.has_quality_gate_integrity());
    assert_eq!(
        report.schema_version,
        CONTEXT_MEMORY_RECALL_QUALITY_GATE_SCHEMA_VERSION
    );
    assert_eq!(
        report.verdict,
        ContextMemoryRecallQualityGateVerdict::GatePass
    );
    assert_eq!(report.metric_count, 8);
    assert_eq!(report.fixture_count, 2);
    assert_eq!(report.fixture_matrix.len(), 2);
    assert_eq!(report.fixture_gate_pass_count, 2);
    assert_eq!(report.fixture_blocked_count, 0);
    assert_eq!(report.blocking_reason_count, 0);
    assert_eq!(report.critical_fact_count, 9);
    assert_eq!(report.recalled_critical_fact_count, 7);
    assert_eq!(report.missing_critical_fact_count, 2);
    assert_eq!(report.missing_critical_fact_regression_count, 0);
    assert_eq!(report.predicted_relevant_count, 9);
    assert_eq!(report.false_positive_count, 2);
    assert_eq!(report.recall_coverage_floor_basis_points, 7_000);
    assert_eq!(report.observed_recall_coverage_basis_points, 7777);
    assert_eq!(report.recall_regression_count, 0);
    assert_eq!(report.precision_floor_basis_points, 7_000);
    assert_eq!(report.observed_precision_basis_points, 7777);
    assert_eq!(report.precision_regression_count, 0);
    assert_eq!(report.missing_critical_fact_limit, 2);
    assert_eq!(report.safety_leak_count, 0);
    assert_eq!(report.answer_quality_regression_count, 0);
    assert!(!report.production_write);
    assert!(!report.graph_write);
    assert!(!report.runtime_activation);
    assert!(!report.adaptive_allocator_runtime_activation);
    assert!(!report.source_aware_runtime_activation);
    assert!(!report.prompt_assembly_change);
    assert!(!report.operator_activation_allowed);

    let synthetic = report
        .fixture_matrix
        .iter()
        .find(|fixture| fixture.fixture_kind == ContextMemoryEvalFixtureKind::SyntheticLongSession)
        .expect("synthetic quality fixture should exist");
    assert!(synthetic.has_fixture_gate_integrity());
    assert!(synthetic.blocking_reasons.is_empty());
    assert_eq!(
        synthetic.verdict,
        ContextMemoryRecallQualityGateVerdict::GatePass
    );
    assert_eq!(synthetic.proposed_critical_fact_count, 5);
    assert_eq!(synthetic.proposed_recalled_critical_fact_count, 4);
    assert_eq!(synthetic.proposed_missing_critical_fact_count, 1);
    assert_eq!(synthetic.current_missing_critical_fact_count, 1);
    assert_eq!(synthetic.proposed_recall_coverage_basis_points, 8000);
    assert_eq!(synthetic.current_recall_coverage_basis_points, 8000);
    assert_eq!(synthetic.proposed_precision_basis_points, 8000);
    assert_eq!(synthetic.current_precision_basis_points, 8000);
    assert!(!synthetic.missing_critical_fact_regression);
    assert!(!synthetic.recall_regression);
    assert!(!synthetic.precision_regression);

    let redacted = report
        .fixture_matrix
        .iter()
        .find(|fixture| fixture.fixture_kind == ContextMemoryEvalFixtureKind::RedactedTrace)
        .expect("redacted trace quality fixture should exist");
    assert!(redacted.has_fixture_gate_integrity());
    assert_eq!(redacted.proposed_critical_fact_count, 4);
    assert_eq!(redacted.proposed_recalled_critical_fact_count, 3);
    assert_eq!(redacted.proposed_missing_critical_fact_count, 1);
    assert_eq!(redacted.proposed_recall_coverage_basis_points, 7500);
    assert_eq!(redacted.proposed_precision_basis_points, 7500);

    let json = serde_json::to_string(&report).expect("quality gate report should serialize");
    assert!(json.contains("gate_pass"));
    assert!(json.contains("fixture_matrix"));
    assert!(json.contains("blocking_reason_count"));
    assert!(json.contains("blocking_reasons"));
    assert!(json.contains("synthetic_long_session"));
    assert!(json.contains("redacted_trace"));
    assert!(json.contains("recall_coverage_floor_basis_points"));
    assert!(json.contains("precision_floor_basis_points"));
    assert!(json.contains("missing_critical_fact_limit"));
    assert!(json.contains("missing_critical_fact_regression_count"));
    assert!(json.contains("precision_regression_count"));
    assert!(json.contains("answer_quality_regression_count"));
    assert!(!json.contains("session-"));
    assert!(!json.contains("memory-"));
    assert!(!json.contains("fixture_id_hash"));
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

#[test]
fn context_memory_recall_quality_gate_fixture_matrix_blocks_regressions_without_activation() {
    let mut shadow = ContextMemoryAdaptiveAllocatorEvalShadowReport::seeded();
    let proposed_synthetic = shadow
        .shadow_results
        .iter_mut()
        .find(|result| {
            result.arm == ContextMemoryAdaptiveAllocatorEvalArm::ProposedAdaptive
                && result.fixture_kind == ContextMemoryEvalFixtureKind::SyntheticLongSession
        })
        .expect("proposed synthetic fixture should exist");
    proposed_synthetic.recalled_critical_fact_count = 3;
    proposed_synthetic.missing_critical_fact_count = 2;
    proposed_synthetic.false_positive_count = 2;
    proposed_synthetic.recall_coverage_basis_points = 6000;
    proposed_synthetic.precision_basis_points = 6000;

    let report = ContextMemoryRecallQualityGateReport::from_shadow(&shadow);

    assert!(!report.has_quality_gate_integrity());
    assert_eq!(
        report.verdict,
        ContextMemoryRecallQualityGateVerdict::Blocked
    );
    assert_eq!(report.fixture_count, 2);
    assert_eq!(report.fixture_gate_pass_count, 1);
    assert_eq!(report.fixture_blocked_count, 1);
    assert_eq!(report.blocking_reason_count, 3);
    assert_eq!(report.missing_critical_fact_count, 3);
    assert_eq!(report.missing_critical_fact_regression_count, 1);
    assert_eq!(report.recall_regression_count, 1);
    assert_eq!(report.precision_regression_count, 1);
    assert_eq!(report.observed_recall_coverage_basis_points, 6666);
    assert_eq!(report.observed_precision_basis_points, 6666);
    assert_eq!(report.safety_leak_count, 0);
    assert_eq!(report.answer_quality_regression_count, 0);
    assert!(!report.production_write);
    assert!(!report.graph_write);
    assert!(!report.runtime_activation);
    assert!(!report.adaptive_allocator_runtime_activation);
    assert!(!report.source_aware_runtime_activation);
    assert!(!report.prompt_assembly_change);
    assert!(!report.operator_activation_allowed);

    let synthetic = report
        .fixture_matrix
        .iter()
        .find(|fixture| fixture.fixture_kind == ContextMemoryEvalFixtureKind::SyntheticLongSession)
        .expect("synthetic quality fixture should exist");
    assert!(!synthetic.has_fixture_gate_integrity());
    assert_eq!(
        synthetic.verdict,
        ContextMemoryRecallQualityGateVerdict::Blocked
    );
    assert!(synthetic.missing_critical_fact_regression);
    assert!(synthetic.recall_regression);
    assert!(synthetic.precision_regression);
    assert_eq!(
        synthetic.blocking_reasons,
        vec![
            ContextMemoryRecallQualityGateBlockerReason::MissingCriticalFactRegression,
            ContextMemoryRecallQualityGateBlockerReason::RecallCoverageRegression,
            ContextMemoryRecallQualityGateBlockerReason::PrecisionRegression,
        ]
    );
    assert!(!synthetic.production_write);
    assert!(!synthetic.graph_write);
    assert!(!synthetic.runtime_activation);

    let json = serde_json::to_string(&report).expect("quality gate report should serialize");
    assert!(json.contains("fixture_matrix"));
    assert!(json.contains("blocking_reason_count"));
    assert!(json.contains("recall_coverage_regression"));
    assert!(json.contains("missing_critical_fact_regression"));
    assert!(json.contains("blocked"));
    assert!(!json.contains("fixture_id_hash"));
    assert!(!json.contains("source_id"));
    assert!(!json.contains("transcript_text"));
    assert!(!json.contains("memory_text"));
    assert!(!json.contains("prompt_text"));
    assert!(!json.contains("answer_text"));
    assert!(!json.contains("\"production_write\":true"));
    assert!(!json.contains("\"graph_write\":true"));
    assert!(!json.contains("\"runtime_activation\":true"));
}

#[test]
fn context_memory_recall_quality_gate_delta_matrix_blocks_answer_quality_and_side_effects() {
    let mut shadow = ContextMemoryAdaptiveAllocatorEvalShadowReport::seeded();
    let proposed_redacted = shadow
        .shadow_results
        .iter_mut()
        .find(|result| {
            result.arm == ContextMemoryAdaptiveAllocatorEvalArm::ProposedAdaptive
                && result.fixture_kind == ContextMemoryEvalFixtureKind::RedactedTrace
        })
        .expect("proposed redacted trace fixture should exist");
    proposed_redacted.answer_quality_regression_count = 2;
    proposed_redacted.prompt_assembly_change = true;

    let report = ContextMemoryRecallQualityGateReport::from_shadow(&shadow);

    assert!(!report.has_quality_gate_integrity());
    assert_eq!(
        report.verdict,
        ContextMemoryRecallQualityGateVerdict::Blocked
    );
    assert_eq!(report.fixture_gate_pass_count, 1);
    assert_eq!(report.fixture_blocked_count, 1);
    assert_eq!(report.blocking_reason_count, 2);
    assert_eq!(report.answer_quality_regression_count, 2);
    assert!(report.prompt_assembly_change);
    assert!(!report.production_write);
    assert!(!report.graph_write);
    assert!(!report.runtime_activation);
    assert!(!report.adaptive_allocator_runtime_activation);
    assert!(!report.source_aware_runtime_activation);
    assert!(!report.operator_activation_allowed);

    let redacted = report
        .fixture_matrix
        .iter()
        .find(|fixture| fixture.fixture_kind == ContextMemoryEvalFixtureKind::RedactedTrace)
        .expect("redacted trace quality fixture should exist");
    assert!(!redacted.has_fixture_gate_integrity());
    assert_eq!(
        redacted.verdict,
        ContextMemoryRecallQualityGateVerdict::Blocked
    );
    assert_eq!(redacted.answer_quality_regression_count, 2);
    assert!(redacted.prompt_assembly_change);
    assert_eq!(
        redacted.blocking_reasons,
        vec![
            ContextMemoryRecallQualityGateBlockerReason::AnswerQualityRegression,
            ContextMemoryRecallQualityGateBlockerReason::SideEffectFlagEnabled,
        ]
    );

    let json = serde_json::to_string(&report).expect("quality gate report should serialize");
    assert!(json.contains("answer_quality_regression"));
    assert!(json.contains("side_effect_flag_enabled"));
    assert!(json.contains("\"prompt_assembly_change\":true"));
    assert!(!json.contains("fixture_id_hash"));
    assert!(!json.contains("source_id"));
    assert!(!json.contains("transcript_text"));
    assert!(!json.contains("memory_text"));
    assert!(!json.contains("prompt_text"));
    assert!(!json.contains("answer_text"));
    assert!(!json.contains("\"production_write\":true"));
    assert!(!json.contains("\"graph_write\":true"));
    assert!(!json.contains("\"runtime_activation\":true"));
}
