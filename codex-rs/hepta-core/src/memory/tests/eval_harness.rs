use super::*;

#[test]
fn context_memory_eval_harness_seed_is_payload_light_and_non_activating() {
    let report = ContextMemoryEvalHarnessReport::seeded();

    assert!(report.has_eval_integrity());
    assert_eq!(
        report.metrics,
        vec![
            ContextMemoryEvalMetric::RecallCoverage,
            ContextMemoryEvalMetric::MissingCriticalFact,
            ContextMemoryEvalMetric::Precision,
            ContextMemoryEvalMetric::Latency,
            ContextMemoryEvalMetric::TokenCost,
            ContextMemoryEvalMetric::TokenSaved,
            ContextMemoryEvalMetric::SafetyLeak,
            ContextMemoryEvalMetric::AnswerQualityRegression,
        ]
    );
    assert_eq!(report.fixture_count(), 2);
    assert_eq!(report.total_missing_critical_fact_count(), 2);
    assert_eq!(report.total_token_saved(), 1_100);
    assert_eq!(report.safety_leak_count(), 0);
    assert_eq!(report.answer_quality_regression_count(), 0);
    assert!(!report.production_write);
    assert!(!report.graph_write);
    assert!(!report.runtime_activation);
    assert!(!report.operator_activation_allowed);

    let synthetic = report
        .fixtures
        .iter()
        .find(|fixture| fixture.fixture_kind == ContextMemoryEvalFixtureKind::SyntheticLongSession)
        .expect("synthetic fixture should exist");
    assert!(synthetic.synthetic);
    assert!(!synthetic.redacted);
    assert_eq!(synthetic.recall_coverage_basis_points, 8000);
    assert_eq!(synthetic.precision_basis_points, 8000);
    assert_eq!(synthetic.missing_critical_fact_count, 1);
    assert_eq!(synthetic.observed_latency_ms, 42);
    assert_eq!(synthetic.latency_budget_ms, 100);

    let redacted = report
        .fixtures
        .iter()
        .find(|fixture| fixture.fixture_kind == ContextMemoryEvalFixtureKind::RedactedTrace)
        .expect("redacted trace fixture should exist");
    assert!(redacted.redacted);
    assert!(!redacted.synthetic);
    assert_eq!(redacted.recall_coverage_basis_points, 7500);
    assert_eq!(redacted.precision_basis_points, 7500);
    assert_eq!(redacted.missing_critical_fact_count, 1);
    assert_eq!(redacted.observed_latency_ms, 64);
    assert_eq!(redacted.latency_budget_ms, 150);

    let json = serde_json::to_string(&report).expect("eval harness report should serialize");
    assert!(json.contains("synthetic_long_session"));
    assert!(json.contains("redacted_trace"));
    assert!(json.contains("recall_coverage"));
    assert!(json.contains("missing_critical_fact"));
    assert!(json.contains("answer_quality_regression"));
    assert!(!json.contains("session-"));
    assert!(!json.contains("memory-"));
    assert!(!json.contains("source_id"));
    assert!(!json.contains("transcript_text"));
    assert!(!json.contains("memory_text"));
    assert!(!json.contains("prompt_text"));
    assert!(!json.contains("answer_text"));
    assert!(!json.contains("\"production_write\":true"));
    assert!(!json.contains("\"graph_write\":true"));
    assert!(!json.contains("\"runtime_activation\":true"));
    assert!(!json.contains("\"operator_activation_allowed\":true"));
}

#[test]
fn context_memory_adaptive_allocator_eval_shadow_compares_without_activation() {
    let report = ContextMemoryAdaptiveAllocatorEvalShadowReport::seeded();

    assert!(report.has_eval_shadow_integrity());
    assert_eq!(
        report.metrics,
        vec![
            ContextMemoryEvalMetric::RecallCoverage,
            ContextMemoryEvalMetric::MissingCriticalFact,
            ContextMemoryEvalMetric::Precision,
            ContextMemoryEvalMetric::Latency,
            ContextMemoryEvalMetric::TokenCost,
            ContextMemoryEvalMetric::TokenSaved,
            ContextMemoryEvalMetric::SafetyLeak,
            ContextMemoryEvalMetric::AnswerQualityRegression,
        ]
    );
    assert_eq!(
        report.result_count_for_arm(ContextMemoryAdaptiveAllocatorEvalArm::CurrentHeuristic),
        2
    );
    assert_eq!(
        report.result_count_for_arm(ContextMemoryAdaptiveAllocatorEvalArm::ProposedAdaptive),
        2
    );
    assert_eq!(
        report.total_missing_critical_fact_count_for_arm(
            ContextMemoryAdaptiveAllocatorEvalArm::CurrentHeuristic
        ),
        2
    );
    assert_eq!(
        report.total_missing_critical_fact_count_for_arm(
            ContextMemoryAdaptiveAllocatorEvalArm::ProposedAdaptive
        ),
        2
    );
    assert_eq!(
        report.total_token_cost_for_arm(ContextMemoryAdaptiveAllocatorEvalArm::CurrentHeuristic),
        3_200
    );
    assert_eq!(
        report.total_token_cost_for_arm(ContextMemoryAdaptiveAllocatorEvalArm::ProposedAdaptive),
        2_560
    );
    assert_eq!(
        report.total_token_saved_for_arm(ContextMemoryAdaptiveAllocatorEvalArm::CurrentHeuristic),
        1_100
    );
    assert_eq!(
        report.total_token_saved_for_arm(ContextMemoryAdaptiveAllocatorEvalArm::ProposedAdaptive),
        1_740
    );
    assert_eq!(
        report.total_latency_ms_for_arm(ContextMemoryAdaptiveAllocatorEvalArm::CurrentHeuristic),
        106
    );
    assert_eq!(
        report.total_latency_ms_for_arm(ContextMemoryAdaptiveAllocatorEvalArm::ProposedAdaptive),
        96
    );
    assert!(report.comparison_verdict.has_shadow_threshold_integrity());
    assert_eq!(
        report.comparison_verdict.verdict,
        ContextMemoryAdaptiveAllocatorEvalShadowVerdict::ShadowThresholdPass
    );
    assert_eq!(report.comparison_verdict.current_result_count, 2);
    assert_eq!(report.comparison_verdict.proposed_result_count, 2);
    assert_eq!(
        report
            .comparison_verdict
            .current_missing_critical_fact_count,
        2
    );
    assert_eq!(
        report
            .comparison_verdict
            .proposed_missing_critical_fact_count,
        2
    );
    assert_eq!(report.comparison_verdict.current_token_cost, 3_200);
    assert_eq!(report.comparison_verdict.proposed_token_cost, 2_560);
    assert_eq!(report.comparison_verdict.current_token_saved, 1_100);
    assert_eq!(report.comparison_verdict.proposed_token_saved, 1_740);
    assert_eq!(report.comparison_verdict.current_latency_ms, 106);
    assert_eq!(report.comparison_verdict.proposed_latency_ms, 96);
    assert_eq!(
        report
            .comparison_verdict
            .missing_critical_fact_regression_count,
        0
    );
    assert_eq!(report.comparison_verdict.recall_regression_count, 0);
    assert_eq!(report.comparison_verdict.precision_regression_count, 0);
    assert_eq!(report.comparison_verdict.latency_regression_count, 0);
    assert_eq!(report.comparison_verdict.token_cost_regression_count, 0);
    assert_eq!(report.comparison_verdict.token_saved_regression_count, 0);
    assert_eq!(report.safety_leak_count(), 0);
    assert_eq!(report.answer_quality_regression_count(), 0);
    assert!(!report.production_write);
    assert!(!report.graph_write);
    assert!(!report.runtime_activation);
    assert!(!report.adaptive_allocator_runtime_activation);
    assert!(!report.source_aware_runtime_activation);
    assert!(!report.prompt_assembly_change);
    assert!(!report.operator_activation_allowed);
    assert!(!report.comparison_verdict.prompt_assembly_change);

    let proposed_synthetic = report
        .shadow_result(
            ContextMemoryAdaptiveAllocatorEvalArm::ProposedAdaptive,
            ContextMemoryEvalFixtureKind::SyntheticLongSession,
        )
        .expect("proposed synthetic fixture should exist");
    assert_eq!(proposed_synthetic.recall_coverage_basis_points, 8000);
    assert_eq!(proposed_synthetic.precision_basis_points, 8000);
    assert_eq!(proposed_synthetic.missing_critical_fact_count, 1);
    assert_eq!(proposed_synthetic.token_cost, 1_440);
    assert_eq!(proposed_synthetic.token_saved, 980);
    assert_eq!(proposed_synthetic.observed_latency_ms, 38);

    let json = serde_json::to_string(&report).expect("eval shadow report should serialize");
    assert!(json.contains("current_heuristic"));
    assert!(json.contains("proposed_adaptive"));
    assert!(json.contains("synthetic_long_session"));
    assert!(json.contains("redacted_trace"));
    assert!(json.contains("recall_coverage"));
    assert!(json.contains("answer_quality_regression"));
    assert!(json.contains("comparison_verdict"));
    assert!(json.contains("shadow_threshold_pass"));
    assert!(json.contains("missing_critical_fact_regression_count"));
    assert!(json.contains("token_saved_regression_count"));
    assert!(!json.contains("session-"));
    assert!(!json.contains("memory-"));
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
    assert!(!json.contains("\"operator_activation_allowed\":true"));
}

#[test]
fn context_memory_selected_recall_summary_canary_eval_replays_without_activation() {
    let report = ContextMemorySelectedRecallSummaryCanaryEvalReport::seeded();

    assert!(report.has_canary_eval_integrity());
    assert_eq!(
        report.schema_version,
        CONTEXT_MEMORY_SELECTED_RECALL_SUMMARY_CANARY_EVAL_SCHEMA_VERSION
    );
    assert_eq!(
        report.mode,
        ContextMemorySelectedRecallSummaryCanaryEvalMode::GoldenReplayShadow
    );
    assert_eq!(
        report.metrics,
        vec![
            ContextMemorySelectedRecallSummaryCanaryEvalMetric::ShadowVsLive,
            ContextMemorySelectedRecallSummaryCanaryEvalMetric::TokenSaved,
            ContextMemorySelectedRecallSummaryCanaryEvalMetric::LatencyDelta,
            ContextMemorySelectedRecallSummaryCanaryEvalMetric::QualityDelta,
            ContextMemorySelectedRecallSummaryCanaryEvalMetric::RollbackReadback,
            ContextMemorySelectedRecallSummaryCanaryEvalMetric::PromptInputProof,
            ContextMemorySelectedRecallSummaryCanaryEvalMetric::ResponseDebugProof,
            ContextMemorySelectedRecallSummaryCanaryEvalMetric::RegressionBlocked,
        ]
    );
    assert_eq!(report.fixture_count(), 4);
    assert_eq!(report.fixture_pass_count(), 4);
    assert_eq!(report.fixture_blocked_count(), 0);
    assert_eq!(report.positive_fixture_count(), 3);
    assert_eq!(report.negative_fixture_count(), 1);
    assert_eq!(report.shadow_vs_live_pair_count(), 3);
    assert_eq!(report.rollback_readback_fixture_count(), 1);
    assert_eq!(report.regression_blocked_count(), 1);
    assert!(report.prompt_input_proof_covered());
    assert!(report.response_debug_proof_covered());
    assert_eq!(report.token_saved_min_basis_points, 1_000);
    assert_eq!(report.latency_delta_max_ms, 250);
    assert_eq!(report.quality_delta_min_basis_points, 0);
    assert!(report.operator_approval_required);
    assert!(!report.production_route);
    assert!(!report.production_write);
    assert!(!report.graph_write);
    assert!(!report.runtime_activation);
    assert!(!report.prompt_assembly_change);
    assert!(!report.operator_activation_allowed);

    let rollback = report
        .fixture(ContextMemorySelectedRecallSummaryCanaryEvalFixtureKind::RollbackReadback)
        .expect("rollback-readback fixture should exist");
    assert!(rollback.positive_fixture);
    assert!(rollback.shadow_vs_live_pair);
    assert!(rollback.rollback_readback_fixture);
    assert!(rollback.token_saved_basis_points >= report.token_saved_min_basis_points);
    assert!(rollback.latency_delta_ms <= report.latency_delta_max_ms);
    assert!(rollback.quality_delta_basis_points >= report.quality_delta_min_basis_points);

    let regression = report
        .fixture(ContextMemorySelectedRecallSummaryCanaryEvalFixtureKind::RegressionGuard)
        .expect("regression guard fixture should exist");
    assert!(regression.negative_fixture);
    assert!(regression.regression_fixture);
    assert!(regression.regression_blocked);
    assert!(!regression.shadow_vs_live_pair);
    assert!(!regression.rollback_readback_fixture);

    let json = serde_json::to_string(&report).expect("selected recall report should serialize");
    assert!(json.contains("golden_replay_shadow"));
    assert!(json.contains("summary_baseline"));
    assert!(json.contains("summary_candidate"));
    assert!(json.contains("rollback_readback"));
    assert!(json.contains("regression_guard"));
    assert!(json.contains("prompt_input_proof"));
    assert!(json.contains("response_debug_proof"));
    assert!(json.contains("regression_blocked"));
    assert!(json.contains("token_saved_min_basis_points"));
    assert!(json.contains("latency_delta_max_ms"));
    assert!(json.contains("quality_delta_min_basis_points"));
    assert!(!json.contains("session-"));
    assert!(!json.contains("memory-"));
    assert!(!json.contains("source_id"));
    assert!(!json.contains("prompt_text"));
    assert!(!json.contains("transcript_text"));
    assert!(!json.contains("memory_text"));
    assert!(!json.contains("answer_text"));
    assert!(!json.contains("query_payload"));
    assert!(!json.contains("tool_args"));
    assert!(!json.contains("tool_outputs"));
    assert!(!json.contains("trace_id"));
    assert!(!json.contains("operator_identity"));
    assert!(!json.contains("rollback_hash"));
    assert!(!json.contains("\"production_route\":true"));
    assert!(!json.contains("\"production_write\":true"));
    assert!(!json.contains("\"graph_write\":true"));
    assert!(!json.contains("\"runtime_activation\":true"));
    assert!(!json.contains("\"prompt_assembly_change\":true"));
    assert!(!json.contains("\"operator_activation_allowed\":true"));
}

#[test]
fn context_memory_selected_recall_summary_canary_eval_blocks_regression_drift() {
    let mut report = ContextMemorySelectedRecallSummaryCanaryEvalReport::seeded();
    let regression = report
        .fixtures
        .iter_mut()
        .find(|fixture| {
            fixture.fixture_kind
                == ContextMemorySelectedRecallSummaryCanaryEvalFixtureKind::RegressionGuard
        })
        .expect("regression guard fixture should exist");

    regression.regression_blocked = false;

    assert!(!report.has_canary_eval_integrity());
}
