use hepta_core::*;

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
fn context_memory_ranked_recall_shadow_eval_tracks_metrics_without_activation() {
    let report = ContextMemoryRankedRecallShadowEvalReport::seeded();

    assert!(report.has_ranked_recall_shadow_integrity());
    assert_eq!(
        report.schema_version,
        CONTEXT_MEMORY_RANKED_RECALL_SHADOW_EVAL_SCHEMA_VERSION
    );
    assert_eq!(
        report.mode,
        ContextMemoryRankedRecallShadowEvalMode::DeterministicShadow
    );
    assert_eq!(
        report.metrics,
        vec![
            ContextMemoryRankedRecallShadowEvalMetric::Recall,
            ContextMemoryRankedRecallShadowEvalMetric::Precision,
            ContextMemoryRankedRecallShadowEvalMetric::TokenSaved,
            ContextMemoryRankedRecallShadowEvalMetric::Latency,
            ContextMemoryRankedRecallShadowEvalMetric::Regret,
        ]
    );
    assert_eq!(
        report.hybrid_signals,
        vec![
            ContextMemoryRankedRecallShadowHybridSignal::LexicalBm25,
            ContextMemoryRankedRecallShadowHybridSignal::Recency,
            ContextMemoryRankedRecallShadowHybridSignal::SourceAuthority,
            ContextMemoryRankedRecallShadowHybridSignal::TemporalValidity,
            ContextMemoryRankedRecallShadowHybridSignal::Feedback,
        ]
    );
    assert_eq!(report.hybrid_signal_count(), 5);
    assert_eq!(report.fixture_count(), 4);
    assert_eq!(report.fixture_pass_count(), 4);
    assert_eq!(report.positive_fixture_count(), 3);
    assert_eq!(report.negative_fixture_count(), 1);
    assert_eq!(report.ranked_item_fixture_count(), 4);
    assert_eq!(report.regression_blocked_count(), 1);
    assert_eq!(report.positive_hybrid_signal_pass_count(), 15);
    assert_eq!(report.hybrid_regression_blocked_count(), 1);
    assert_eq!(report.calibrated_reranking_fixture_count(), 4);
    assert_eq!(report.calibrated_reranking_win_count(), 3);
    assert_eq!(report.calibrated_reranking_loss_count(), 1);
    assert_eq!(report.reranking_regression_blocked_count(), 1);
    assert_eq!(report.routing_diff_fixture_count(), 4);
    assert_eq!(report.routing_diff_shadow_only_count(), 4);
    assert_eq!(report.routing_diff_win_count(), 3);
    assert_eq!(report.routing_diff_loss_count(), 1);
    assert_eq!(report.routing_diff_regression_blocked_count(), 1);
    assert_eq!(report.real_workload_trace_fixture_count(), 4);
    assert_eq!(report.real_workload_trace_shadow_only_count(), 4);
    assert_eq!(report.real_workload_trace_slo_pass_count(), 3);
    assert_eq!(report.real_workload_trace_win_count(), 3);
    assert_eq!(report.real_workload_trace_loss_count(), 1);
    assert_eq!(
        report.real_workload_trace_operator_review_required_count(),
        4
    );
    assert_eq!(report.real_workload_trace_total_leak_count(), 0);
    assert_eq!(report.real_workload_trace_max_leak_rate_basis_points(), 0);
    assert_eq!(
        report.min_positive_real_workload_trace_coverage_basis_points(),
        8000
    );
    assert_eq!(
        report.min_positive_real_workload_trace_precision_basis_points(),
        8000
    );
    assert_eq!(
        report.total_positive_real_workload_trace_token_saved(),
        2_140
    );
    assert_eq!(report.max_positive_real_workload_trace_latency_ms(), 55);
    assert_eq!(report.real_workload_trace_regression_loss_count(), 1);
    assert_eq!(report.canary_precondition_fixture_count(), 4);
    assert_eq!(report.canary_precondition_shadow_only_count(), 4);
    assert_eq!(report.canary_precondition_pass_count(), 4);
    assert_eq!(report.canary_feature_flag_registered_count(), 4);
    assert_eq!(report.canary_feature_flag_disabled_count(), 4);
    assert_eq!(report.canary_kill_switch_registered_count(), 4);
    assert_eq!(report.canary_kill_switch_enabled_count(), 4);
    assert_eq!(report.canary_rollback_rehearsal_covered_count(), 4);
    assert_eq!(report.canary_activation_denial_covered_count(), 4);
    assert_eq!(
        report.canary_precondition_operator_review_required_count(),
        4
    );
    assert_eq!(report.canary_precondition_route_opened_count(), 0);
    assert_eq!(report.canary_precondition_rollback_write_count(), 0);
    assert_eq!(report.min_positive_recall_basis_points(), 8000);
    assert_eq!(report.min_positive_precision_basis_points(), 8000);
    assert_eq!(report.min_positive_hybrid_score_basis_points(), 7800);
    assert_eq!(report.min_positive_reranking_delta_basis_points(), 640);
    assert_eq!(report.max_positive_latency_delta_ms(), 10);
    assert_eq!(report.min_positive_token_tradeoff_basis_points(), 3_000);
    assert_eq!(report.min_positive_routing_diff_delta_basis_points(), 640);
    assert_eq!(report.max_positive_routing_diff_latency_delta_ms(), 10);
    assert_eq!(
        report.min_positive_routing_diff_token_tradeoff_basis_points(),
        3_000
    );
    assert_eq!(report.total_positive_token_saved(), 2_140);
    assert_eq!(report.max_positive_latency_ms(), 55);
    assert_eq!(report.max_positive_regret_basis_points(), 0);
    assert_eq!(report.recall_floor_basis_points, 7_000);
    assert_eq!(report.precision_floor_basis_points, 7_000);
    assert_eq!(report.token_saved_min, 300);
    assert_eq!(report.token_saved_min_basis_points, 1_000);
    assert_eq!(report.latency_max_ms, 100);
    assert_eq!(report.regret_max_basis_points, 0);
    assert_eq!(report.hybrid_signal_min_basis_points, 6_000);
    assert_eq!(report.reranking_delta_min_basis_points, 400);
    assert_eq!(report.latency_delta_max_ms, 20);
    assert_eq!(report.token_tradeoff_min_basis_points, 1_000);
    assert_eq!(report.routing_diff_delta_min_basis_points, 400);
    assert_eq!(report.routing_diff_latency_delta_max_ms, 20);
    assert_eq!(report.routing_diff_token_tradeoff_min_basis_points, 1_000);
    assert_eq!(report.real_workload_coverage_floor_basis_points, 7_000);
    assert_eq!(report.real_workload_precision_floor_basis_points, 7_000);
    assert_eq!(report.real_workload_leak_rate_max_basis_points, 0);
    assert_eq!(report.real_workload_token_saved_min, 300);
    assert_eq!(report.real_workload_latency_max_ms, 100);
    assert!(report.operator_approval_required);
    assert!(!report.production_route);
    assert!(!report.production_write);
    assert!(!report.graph_write);
    assert!(!report.runtime_activation);
    assert!(!report.prompt_assembly_change);
    assert!(!report.operator_activation_allowed);

    let query_match = report
        .fixture(ContextMemoryRankedRecallShadowEvalFixtureKind::QueryMatch)
        .expect("query-match fixture should exist");
    assert!(query_match.positive_fixture);
    assert_eq!(query_match.ranked_item_count, 5);
    assert_eq!(query_match.expected_relevant_count, 4);
    assert_eq!(query_match.recalled_relevant_count, 4);
    assert_eq!(query_match.predicted_relevant_count, 5);
    assert_eq!(query_match.false_positive_count, 1);
    assert_eq!(query_match.recall_basis_points, 10_000);
    assert_eq!(query_match.precision_basis_points, 8_000);
    assert_eq!(query_match.token_saved, 700);
    assert_eq!(query_match.token_saved_basis_points, 3_500);
    assert_eq!(query_match.latency_ms, 42);
    assert_eq!(query_match.regret_basis_points, 0);
    assert_eq!(query_match.lexical_bm25_score_basis_points, 9_200);
    assert_eq!(query_match.recency_score_basis_points, 7_600);
    assert_eq!(query_match.source_authority_score_basis_points, 8_100);
    assert_eq!(query_match.temporal_validity_score_basis_points, 7_800);
    assert_eq!(query_match.feedback_score_basis_points, 7_000);
    assert_eq!(query_match.hybrid_score_basis_points, 7_940);
    assert_eq!(query_match.hybrid_signal_pass_count, 5);
    assert!(query_match.calibrated_reranking_fixture);
    assert_eq!(query_match.baseline_rank_window_score_basis_points, 7_400);
    assert_eq!(query_match.hybrid_rank_window_score_basis_points, 8_140);
    assert_eq!(query_match.reranking_delta_basis_points, 740);
    assert!(query_match.reranking_win);
    assert!(!query_match.reranking_loss);
    assert_eq!(query_match.latency_delta_ms, 8);
    assert_eq!(query_match.token_tradeoff_basis_points, 3_500);
    assert!(query_match.routing_diff_fixture);
    assert!(query_match.routing_diff_shadow_only);
    assert_eq!(query_match.production_selection_score_basis_points, 7_400);
    assert_eq!(
        query_match.hybrid_calibrated_selection_score_basis_points,
        8_140
    );
    assert_eq!(query_match.routing_diff_delta_basis_points, 740);
    assert!(query_match.routing_diff_win);
    assert!(!query_match.routing_diff_loss);
    assert_eq!(query_match.routing_diff_latency_delta_ms, 8);
    assert_eq!(query_match.routing_diff_token_tradeoff_basis_points, 3_500);
    assert!(query_match.real_workload_trace_fixture);
    assert!(query_match.real_workload_trace_shadow_only);
    assert!(query_match.real_workload_trace_slo_pass);
    assert!(query_match.real_workload_trace_operator_review_required);
    assert_eq!(
        query_match.real_workload_trace_coverage_basis_points,
        10_000
    );
    assert_eq!(
        query_match.real_workload_trace_precision_basis_points,
        8_000
    );
    assert_eq!(query_match.real_workload_trace_leak_count, 0);
    assert_eq!(query_match.real_workload_trace_leak_rate_basis_points, 0);
    assert_eq!(query_match.real_workload_trace_token_saved, 700);
    assert_eq!(query_match.real_workload_trace_latency_ms, 42);
    assert!(query_match.real_workload_trace_win);
    assert!(!query_match.real_workload_trace_loss);
    assert!(query_match.canary_precondition_fixture);
    assert!(query_match.canary_precondition_shadow_only);
    assert!(query_match.canary_precondition_pass);
    assert!(query_match.canary_feature_flag_registered);
    assert!(query_match.canary_feature_flag_default_disabled);
    assert!(query_match.canary_kill_switch_registered);
    assert!(query_match.canary_kill_switch_default_enabled);
    assert!(query_match.canary_rollback_rehearsal_covered);
    assert!(query_match.canary_activation_denial_covered);
    assert!(query_match.canary_precondition_operator_review_required);
    assert!(!query_match.canary_precondition_route_opened);
    assert!(!query_match.canary_precondition_rollback_write);

    let regression = report
        .fixture(ContextMemoryRankedRecallShadowEvalFixtureKind::RegressionGuard)
        .expect("regression guard fixture should exist");
    assert!(regression.negative_fixture);
    assert!(regression.regression_fixture);
    assert!(regression.regression_blocked);
    assert_eq!(regression.recall_basis_points, 5_000);
    assert_eq!(regression.precision_basis_points, 3_333);
    assert_eq!(regression.token_saved, 0);
    assert_eq!(regression.latency_ms, 125);
    assert_eq!(regression.regret_basis_points, 500);
    assert_eq!(regression.hybrid_score_basis_points, 4_300);
    assert_eq!(regression.hybrid_signal_pass_count, 0);
    assert!(regression.calibrated_reranking_fixture);
    assert_eq!(regression.baseline_rank_window_score_basis_points, 6_500);
    assert_eq!(regression.hybrid_rank_window_score_basis_points, 4_300);
    assert_eq!(regression.reranking_delta_basis_points, -2_200);
    assert!(!regression.reranking_win);
    assert!(regression.reranking_loss);
    assert_eq!(regression.latency_delta_ms, 35);
    assert_eq!(regression.token_tradeoff_basis_points, 0);
    assert!(regression.routing_diff_fixture);
    assert!(regression.routing_diff_shadow_only);
    assert_eq!(regression.production_selection_score_basis_points, 6_500);
    assert_eq!(
        regression.hybrid_calibrated_selection_score_basis_points,
        4_300
    );
    assert_eq!(regression.routing_diff_delta_basis_points, -2_200);
    assert!(!regression.routing_diff_win);
    assert!(regression.routing_diff_loss);
    assert_eq!(regression.routing_diff_latency_delta_ms, 35);
    assert_eq!(regression.routing_diff_token_tradeoff_basis_points, 0);
    assert!(regression.real_workload_trace_fixture);
    assert!(regression.real_workload_trace_shadow_only);
    assert!(!regression.real_workload_trace_slo_pass);
    assert!(regression.real_workload_trace_operator_review_required);
    assert_eq!(regression.real_workload_trace_coverage_basis_points, 5_000);
    assert_eq!(regression.real_workload_trace_precision_basis_points, 3_333);
    assert_eq!(regression.real_workload_trace_leak_count, 0);
    assert_eq!(regression.real_workload_trace_leak_rate_basis_points, 0);
    assert_eq!(regression.real_workload_trace_token_saved, 0);
    assert_eq!(regression.real_workload_trace_latency_ms, 125);
    assert!(!regression.real_workload_trace_win);
    assert!(regression.real_workload_trace_loss);
    assert!(regression.canary_precondition_pass);
    assert!(!regression.canary_precondition_route_opened);
    assert!(!regression.canary_precondition_rollback_write);

    let json = serde_json::to_string(&report).expect("ranked recall report should serialize");
    assert!(json.contains("deterministic_shadow"));
    assert!(json.contains("lexical_bm25"));
    assert!(json.contains("source_authority"));
    assert!(json.contains("temporal_validity"));
    assert!(json.contains("feedback"));
    assert!(json.contains("query_match"));
    assert!(json.contains("recency_tie_break"));
    assert!(json.contains("budget_pressure"));
    assert!(json.contains("regression_guard"));
    assert!(json.contains("recall"));
    assert!(json.contains("precision"));
    assert!(json.contains("token_saved"));
    assert!(json.contains("latency"));
    assert!(json.contains("regret"));
    assert!(json.contains("ranked_item_count"));
    assert!(json.contains("hybrid_score_basis_points"));
    assert!(json.contains("hybrid_signal_pass_count"));
    assert!(json.contains("calibrated_reranking_fixture"));
    assert!(json.contains("reranking_delta_basis_points"));
    assert!(json.contains("token_tradeoff_basis_points"));
    assert!(json.contains("routing_diff_fixture"));
    assert!(json.contains("routing_diff_shadow_only"));
    assert!(json.contains("production_selection_score_basis_points"));
    assert!(json.contains("hybrid_calibrated_selection_score_basis_points"));
    assert!(json.contains("routing_diff_delta_basis_points"));
    assert!(json.contains("real_workload_trace_fixture"));
    assert!(json.contains("real_workload_trace_slo_pass"));
    assert!(json.contains("real_workload_trace_coverage_basis_points"));
    assert!(json.contains("real_workload_trace_precision_basis_points"));
    assert!(json.contains("real_workload_trace_leak_rate_basis_points"));
    assert!(json.contains("real_workload_trace_operator_review_required"));
    assert!(json.contains("canary_precondition_pass"));
    assert!(json.contains("canary_feature_flag_default_disabled"));
    assert!(json.contains("canary_kill_switch_default_enabled"));
    assert!(json.contains("canary_precondition_route_opened"));
    assert!(json.contains("token_saved_min_basis_points"));
    assert!(!json.contains("session-"));
    assert!(!json.contains("memory-"));
    assert!(!json.contains("source_id"));
    assert!(!json.contains("prompt_text"));
    assert!(!json.contains("transcript_text"));
    assert!(!json.contains("memory_text"));
    assert!(!json.contains("answer_text"));
    assert!(!json.contains("query_payload"));
    assert!(!json.contains("raw_ranked_payload"));
    assert!(!json.contains("rank_explanation"));
    assert!(!json.contains("score_reason"));
    assert!(!json.contains("tool_args"));
    assert!(!json.contains("tool_outputs"));
    assert!(!json.contains("trace_id"));
    assert!(!json.contains("operator_identity"));
    assert!(!json.contains("\"production_route\":true"));
    assert!(!json.contains("\"production_write\":true"));
    assert!(!json.contains("\"graph_write\":true"));
    assert!(!json.contains("\"runtime_activation\":true"));
    assert!(!json.contains("\"prompt_assembly_change\":true"));
    assert!(!json.contains("\"operator_activation_allowed\":true"));
}

#[test]
fn context_memory_ranked_recall_shadow_eval_blocks_regression_drift() {
    let mut report = ContextMemoryRankedRecallShadowEvalReport::seeded();
    let regression = report
        .fixtures
        .iter_mut()
        .find(|fixture| {
            fixture.fixture_kind == ContextMemoryRankedRecallShadowEvalFixtureKind::RegressionGuard
        })
        .expect("regression guard fixture should exist");

    regression.regression_blocked = false;

    assert!(!report.has_ranked_recall_shadow_integrity());
}

#[test]
fn context_memory_ranked_recall_shadow_eval_blocks_hybrid_signal_drift() {
    let mut missing_signal_report = ContextMemoryRankedRecallShadowEvalReport::seeded();
    missing_signal_report.hybrid_signals.pop();

    assert!(!missing_signal_report.has_ranked_recall_shadow_integrity());

    let mut low_signal_report = ContextMemoryRankedRecallShadowEvalReport::seeded();
    let query_match = low_signal_report
        .fixtures
        .iter_mut()
        .find(|fixture| {
            fixture.fixture_kind == ContextMemoryRankedRecallShadowEvalFixtureKind::QueryMatch
        })
        .expect("query-match fixture should exist");

    query_match.lexical_bm25_score_basis_points = 5_999;

    assert!(!low_signal_report.has_ranked_recall_shadow_integrity());
}

#[test]
fn context_memory_ranked_recall_shadow_eval_blocks_calibrated_reranking_drift() {
    let mut report = ContextMemoryRankedRecallShadowEvalReport::seeded();
    let query_match = report
        .fixtures
        .iter_mut()
        .find(|fixture| {
            fixture.fixture_kind == ContextMemoryRankedRecallShadowEvalFixtureKind::QueryMatch
        })
        .expect("query-match fixture should exist");

    query_match.hybrid_rank_window_score_basis_points = 7_799;
    query_match.reranking_delta_basis_points = 399;

    assert!(!report.has_ranked_recall_shadow_integrity());
}

#[test]
fn context_memory_ranked_recall_shadow_eval_blocks_routing_diff_drift() {
    let mut report = ContextMemoryRankedRecallShadowEvalReport::seeded();
    let query_match = report
        .fixtures
        .iter_mut()
        .find(|fixture| {
            fixture.fixture_kind == ContextMemoryRankedRecallShadowEvalFixtureKind::QueryMatch
        })
        .expect("query-match fixture should exist");

    query_match.routing_diff_shadow_only = false;

    assert!(!report.has_ranked_recall_shadow_integrity());

    let mut replay = ContextMemoryRankedRecallShadowEvalReport::seeded();
    let replay_fixture = replay
        .fixtures
        .iter_mut()
        .find(|fixture| {
            fixture.fixture_kind == ContextMemoryRankedRecallShadowEvalFixtureKind::QueryMatch
        })
        .expect("query-match fixture should exist");
    replay_fixture.routing_diff_delta_basis_points = 399;

    assert!(!replay.has_ranked_recall_shadow_integrity());
}

#[test]
fn context_memory_ranked_recall_shadow_eval_blocks_real_workload_slo_drift() {
    let mut missing_trace = ContextMemoryRankedRecallShadowEvalReport::seeded();
    let query_match = missing_trace
        .fixtures
        .iter_mut()
        .find(|fixture| {
            fixture.fixture_kind == ContextMemoryRankedRecallShadowEvalFixtureKind::QueryMatch
        })
        .expect("query-match fixture should exist");
    query_match.real_workload_trace_shadow_only = false;

    assert!(!missing_trace.has_ranked_recall_shadow_integrity());

    let mut leak_report = ContextMemoryRankedRecallShadowEvalReport::seeded();
    let leak_fixture = leak_report
        .fixtures
        .iter_mut()
        .find(|fixture| {
            fixture.fixture_kind == ContextMemoryRankedRecallShadowEvalFixtureKind::QueryMatch
        })
        .expect("query-match fixture should exist");
    leak_fixture.real_workload_trace_leak_count = 1;
    leak_fixture.real_workload_trace_leak_rate_basis_points = 2_000;

    assert!(!leak_report.has_ranked_recall_shadow_integrity());

    let mut replay = ContextMemoryRankedRecallShadowEvalReport::seeded();
    let replay_fixture = replay
        .fixtures
        .iter_mut()
        .find(|fixture| {
            fixture.fixture_kind == ContextMemoryRankedRecallShadowEvalFixtureKind::QueryMatch
        })
        .expect("query-match fixture should exist");
    replay_fixture.real_workload_trace_slo_pass = true;
    replay_fixture.real_workload_trace_coverage_basis_points = 6_999;

    assert!(!replay.has_ranked_recall_shadow_integrity());
}

#[test]
fn context_memory_ranked_recall_shadow_eval_blocks_canary_precondition_drift() {
    let mut missing_flag = ContextMemoryRankedRecallShadowEvalReport::seeded();
    let query_match = missing_flag
        .fixtures
        .iter_mut()
        .find(|fixture| {
            fixture.fixture_kind == ContextMemoryRankedRecallShadowEvalFixtureKind::QueryMatch
        })
        .expect("query-match fixture should exist");
    query_match.canary_feature_flag_default_disabled = false;

    assert!(!missing_flag.has_ranked_recall_shadow_integrity());

    let mut route_opened = ContextMemoryRankedRecallShadowEvalReport::seeded();
    let route_fixture = route_opened
        .fixtures
        .iter_mut()
        .find(|fixture| {
            fixture.fixture_kind == ContextMemoryRankedRecallShadowEvalFixtureKind::QueryMatch
        })
        .expect("query-match fixture should exist");
    route_fixture.canary_precondition_route_opened = true;

    assert!(!route_opened.has_ranked_recall_shadow_integrity());
}

#[test]
fn context_memory_temporal_graph_shadow_eval_tracks_metrics_without_activation() {
    let report = ContextMemoryTemporalGraphShadowEvalReport::seeded();

    assert!(report.has_temporal_graph_shadow_integrity());
    assert_eq!(
        report.schema_version,
        CONTEXT_MEMORY_TEMPORAL_GRAPH_SHADOW_EVAL_SCHEMA_VERSION
    );
    assert_eq!(
        report.mode,
        ContextMemoryTemporalGraphShadowEvalMode::DeterministicShadow
    );
    assert_eq!(
        report.metrics,
        vec![
            ContextMemoryTemporalGraphShadowEvalMetric::NodeCoverage,
            ContextMemoryTemporalGraphShadowEvalMetric::EdgeCoverage,
            ContextMemoryTemporalGraphShadowEvalMetric::ValidityWindowCoverage,
            ContextMemoryTemporalGraphShadowEvalMetric::SupersedesCoverage,
            ContextMemoryTemporalGraphShadowEvalMetric::Latency,
            ContextMemoryTemporalGraphShadowEvalMetric::Regret,
        ]
    );
    assert_eq!(report.fixture_count(), 4);
    assert_eq!(report.fixture_pass_count(), 4);
    assert_eq!(report.positive_fixture_count(), 3);
    assert_eq!(report.negative_fixture_count(), 1);
    assert_eq!(report.regression_blocked_count(), 1);
    assert_eq!(report.min_positive_node_coverage_basis_points(), 10_000);
    assert_eq!(report.min_positive_edge_coverage_basis_points(), 10_000);
    assert_eq!(
        report.min_positive_validity_window_coverage_basis_points(),
        10_000
    );
    assert_eq!(
        report.min_positive_supersedes_coverage_basis_points(),
        10_000
    );
    assert_eq!(report.max_positive_latency_ms(), 47);
    assert_eq!(report.max_positive_regret_basis_points(), 0);
    assert_eq!(report.node_coverage_floor_basis_points, 10_000);
    assert_eq!(report.edge_coverage_floor_basis_points, 10_000);
    assert_eq!(report.validity_window_floor_basis_points, 10_000);
    assert_eq!(report.supersedes_floor_basis_points, 10_000);
    assert_eq!(report.latency_max_ms, 100);
    assert_eq!(report.regret_max_basis_points, 0);
    assert!(report.operator_approval_required);
    assert!(!report.production_route);
    assert!(!report.production_write);
    assert!(!report.graph_write);
    assert!(!report.runtime_activation);
    assert!(!report.prompt_assembly_change);
    assert!(!report.operator_activation_allowed);

    let topology = report
        .fixture(ContextMemoryTemporalGraphShadowEvalFixtureKind::TopologyCoverage)
        .expect("topology coverage fixture should exist");
    assert!(topology.positive_fixture);
    assert_eq!(topology.temporal_fact_count, 5);
    assert_eq!(topology.graph_node_count, 5);
    assert_eq!(topology.graph_edge_count, 10);
    assert_eq!(topology.observed_validity_window_edge_count, 5);
    assert_eq!(topology.observed_supersedes_edge_count, 0);
    assert_eq!(topology.node_coverage_basis_points, 10_000);
    assert_eq!(topology.edge_coverage_basis_points, 10_000);
    assert_eq!(topology.validity_window_coverage_basis_points, 10_000);
    assert_eq!(topology.supersedes_coverage_basis_points, 10_000);

    let supersedes = report
        .fixture(ContextMemoryTemporalGraphShadowEvalFixtureKind::SupersedesReplay)
        .expect("supersedes replay fixture should exist");
    assert!(supersedes.positive_fixture);
    assert_eq!(supersedes.observed_supersedes_edge_count, 1);
    assert_eq!(supersedes.supersedes_coverage_basis_points, 10_000);

    let regression = report
        .fixture(ContextMemoryTemporalGraphShadowEvalFixtureKind::RegressionGuard)
        .expect("regression guard fixture should exist");
    assert!(regression.negative_fixture);
    assert!(regression.regression_fixture);
    assert!(regression.regression_blocked);
    assert_eq!(regression.node_coverage_basis_points, 6_666);
    assert_eq!(regression.edge_coverage_basis_points, 5_714);
    assert_eq!(regression.validity_window_coverage_basis_points, 6_666);
    assert_eq!(regression.supersedes_coverage_basis_points, 0);
    assert_eq!(regression.latency_ms, 125);
    assert_eq!(regression.regret_basis_points, 400);

    let json = serde_json::to_string(&report).expect("temporal graph report should serialize");
    assert!(json.contains("deterministic_shadow"));
    assert!(json.contains("topology_coverage"));
    assert!(json.contains("validity_window_replay"));
    assert!(json.contains("supersedes_replay"));
    assert!(json.contains("regression_guard"));
    assert!(json.contains("node_coverage"));
    assert!(json.contains("edge_coverage"));
    assert!(json.contains("validity_window_coverage"));
    assert!(json.contains("supersedes_coverage"));
    assert!(json.contains("temporal_fact_count"));
    assert!(json.contains("graph_edge_count"));
    assert!(!json.contains("session-"));
    assert!(!json.contains("memory-"));
    assert!(!json.contains("source_id"));
    assert!(!json.contains("entity_text"));
    assert!(!json.contains("fact_text"));
    assert!(!json.contains("transcript_text"));
    assert!(!json.contains("memory_text"));
    assert!(!json.contains("prompt_text"));
    assert!(!json.contains("answer_text"));
    assert!(!json.contains("query_payload"));
    assert!(!json.contains("raw_graph_payload"));
    assert!(!json.contains("tool_args"));
    assert!(!json.contains("tool_outputs"));
    assert!(!json.contains("trace_id"));
    assert!(!json.contains("operator_identity"));
    assert!(!json.contains("\"production_route\":true"));
    assert!(!json.contains("\"production_write\":true"));
    assert!(!json.contains("\"graph_write\":true"));
    assert!(!json.contains("\"runtime_activation\":true"));
    assert!(!json.contains("\"prompt_assembly_change\":true"));
    assert!(!json.contains("\"operator_activation_allowed\":true"));
}

#[test]
fn context_memory_temporal_graph_shadow_eval_blocks_regression_drift() {
    let mut report = ContextMemoryTemporalGraphShadowEvalReport::seeded();
    let regression = report
        .fixtures
        .iter_mut()
        .find(|fixture| {
            fixture.fixture_kind == ContextMemoryTemporalGraphShadowEvalFixtureKind::RegressionGuard
        })
        .expect("regression guard fixture should exist");

    regression.regression_blocked = false;

    assert!(!report.has_temporal_graph_shadow_integrity());
}

fn memory_provider_report_fixture() -> MemoryProviderReport {
    MemoryProviderReport::from_update(
        MemoryProviderDescriptor::builtin(),
        MemoryProviderContextUpdateEnvelope {
            provider_id: "builtin".into(),
            mode: MemoryProviderContextUpdateMode::ShadowOnly,
            source_counts: ContextRecallSourceCounts {
                recent_entry_count: 1,
                transcript_hit_count: 1,
                durable_memory_hit_count: 1,
                summary_hit_count: 0,
            },
            limit_pressure: ContextRecallLimitPressure::default(),
            ranked_item_count: 3,
            selected_item_count: 3,
            estimated_token_count: 320,
            payload_light: true,
            operator_approval_required: true,
            prompt_payload_exported: false,
            query_payload_exported: false,
            ranked_payload_exported: false,
            write_performed: false,
            runtime_activation: false,
        },
    )
}

fn shadow_quality_summary_report_fixture() -> ContextMemoryShadowQualitySummaryReport {
    let ranked_recall = ContextMemoryRankedRecallShadowEvalReport::seeded();
    let temporal_graph = ContextMemoryTemporalGraphShadowEvalReport::seeded();
    let recall_quality = ContextMemoryRecallQualityGateReport::seeded();
    let provider = memory_provider_report_fixture();
    let dashboard = ContextMemoryShadowRegressionDashboardReport::from_reports(
        &ranked_recall,
        &temporal_graph,
        &recall_quality,
        &provider,
    );

    ContextMemoryShadowQualitySummaryReport::from_dashboard(&dashboard)
}

#[test]
fn context_memory_shadow_regression_dashboard_rolls_up_shadow_reports_without_activation() {
    let ranked_recall = ContextMemoryRankedRecallShadowEvalReport::seeded();
    let temporal_graph = ContextMemoryTemporalGraphShadowEvalReport::seeded();
    let recall_quality = ContextMemoryRecallQualityGateReport::seeded();
    let provider = memory_provider_report_fixture();

    let report = ContextMemoryShadowRegressionDashboardReport::from_reports(
        &ranked_recall,
        &temporal_graph,
        &recall_quality,
        &provider,
    );

    assert!(report.has_shadow_regression_dashboard_integrity());
    assert_eq!(
        report.schema_version,
        CONTEXT_MEMORY_SHADOW_REGRESSION_DASHBOARD_SCHEMA_VERSION
    );
    assert_eq!(
        report.mode,
        ContextMemoryShadowRegressionDashboardMode::ShadowOnly
    );
    assert_eq!(report.input_report_count, 4);
    assert_eq!(report.input_report_pass_count, 4);
    assert_eq!(report.regression_blocking_count, 0);
    assert_eq!(report.ranked_recall_fixture_count, 4);
    assert_eq!(report.ranked_recall_fixture_pass_count, 4);
    assert_eq!(report.ranked_recall_regression_blocked_count, 1);
    assert_eq!(report.ranked_recall_min_positive_recall_basis_points, 8000);
    assert_eq!(
        report.ranked_recall_min_positive_precision_basis_points,
        8000
    );
    assert_eq!(report.ranked_recall_total_positive_token_saved, 2_140);
    assert_eq!(report.ranked_recall_max_positive_latency_ms, 55);
    assert_eq!(report.ranked_recall_max_positive_regret_basis_points, 0);
    assert!(report.ranked_recall_comparison_summary_pass);
    assert_eq!(report.ranked_recall_hybrid_signal_count, 5);
    assert_eq!(report.ranked_recall_positive_hybrid_signal_pass_count, 15);
    assert_eq!(report.ranked_recall_hybrid_regression_blocked_count, 1);
    assert_eq!(
        report.ranked_recall_min_positive_hybrid_score_basis_points,
        7_800
    );
    assert_eq!(report.ranked_recall_calibrated_reranking_fixture_count, 4);
    assert_eq!(report.ranked_recall_calibrated_reranking_win_count, 3);
    assert_eq!(report.ranked_recall_calibrated_reranking_loss_count, 1);
    assert_eq!(
        report.ranked_recall_min_positive_reranking_delta_basis_points,
        640
    );
    assert_eq!(report.ranked_recall_max_positive_latency_delta_ms, 10);
    assert_eq!(
        report.ranked_recall_min_positive_token_tradeoff_basis_points,
        3_000
    );
    assert_eq!(report.ranked_recall_reranking_regression_blocked_count, 1);
    assert_eq!(report.ranked_recall_routing_diff_fixture_count, 4);
    assert_eq!(report.ranked_recall_routing_diff_shadow_only_count, 4);
    assert_eq!(report.ranked_recall_routing_diff_win_count, 3);
    assert_eq!(report.ranked_recall_routing_diff_loss_count, 1);
    assert_eq!(
        report.ranked_recall_min_positive_routing_diff_delta_basis_points,
        640
    );
    assert_eq!(
        report.ranked_recall_max_positive_routing_diff_latency_delta_ms,
        10
    );
    assert_eq!(
        report.ranked_recall_min_positive_routing_diff_token_tradeoff_basis_points,
        3_000
    );
    assert_eq!(
        report.ranked_recall_routing_diff_regression_blocked_count,
        1
    );
    assert_eq!(report.ranked_recall_real_workload_trace_fixture_count, 4);
    assert_eq!(
        report.ranked_recall_real_workload_trace_shadow_only_count,
        4
    );
    assert_eq!(report.ranked_recall_real_workload_trace_slo_pass_count, 3);
    assert_eq!(report.ranked_recall_real_workload_trace_win_count, 3);
    assert_eq!(report.ranked_recall_real_workload_trace_loss_count, 1);
    assert_eq!(
        report.ranked_recall_real_workload_trace_operator_review_required_count,
        4
    );
    assert_eq!(report.ranked_recall_real_workload_trace_total_leak_count, 0);
    assert_eq!(
        report.ranked_recall_real_workload_trace_max_leak_rate_basis_points,
        0
    );
    assert_eq!(
        report.ranked_recall_min_positive_real_workload_trace_coverage_basis_points,
        8_000
    );
    assert_eq!(
        report.ranked_recall_min_positive_real_workload_trace_precision_basis_points,
        8_000
    );
    assert_eq!(
        report.ranked_recall_total_positive_real_workload_trace_token_saved,
        2_140
    );
    assert_eq!(
        report.ranked_recall_max_positive_real_workload_trace_latency_ms,
        55
    );
    assert_eq!(
        report.ranked_recall_real_workload_trace_regression_loss_count,
        1
    );
    assert_eq!(report.temporal_graph_fixture_count, 4);
    assert_eq!(report.temporal_graph_fixture_pass_count, 4);
    assert_eq!(report.temporal_graph_regression_blocked_count, 1);
    assert_eq!(
        report.temporal_graph_min_positive_node_coverage_basis_points,
        10_000
    );
    assert_eq!(
        report.temporal_graph_min_positive_edge_coverage_basis_points,
        10_000
    );
    assert_eq!(
        report.temporal_graph_min_positive_validity_window_coverage_basis_points,
        10_000
    );
    assert_eq!(
        report.temporal_graph_min_positive_supersedes_coverage_basis_points,
        10_000
    );
    assert_eq!(report.temporal_graph_max_positive_latency_ms, 47);
    assert_eq!(report.temporal_graph_max_positive_regret_basis_points, 0);
    assert_eq!(report.recall_quality_fixture_count, 2);
    assert_eq!(report.recall_quality_fixture_pass_count, 2);
    assert_eq!(report.recall_quality_blocking_reason_count, 0);
    assert_eq!(
        report.recall_quality_missing_critical_fact_regression_count,
        0
    );
    assert_eq!(report.recall_quality_recall_regression_count, 0);
    assert_eq!(report.recall_quality_precision_regression_count, 0);
    assert_eq!(report.recall_quality_observed_recall_basis_points, 7777);
    assert_eq!(report.recall_quality_observed_precision_basis_points, 7777);
    assert!(report.provider_boundary_pass);
    assert!(report.provider_payload_light);
    assert_eq!(report.provider_selected_item_count, 3);
    assert_eq!(report.provider_ranked_item_count, 3);
    assert_eq!(report.provider_estimated_token_count, 320);
    assert!(report.operator_approval_required);
    assert!(!report.production_route);
    assert!(!report.production_write);
    assert!(!report.graph_write);
    assert!(!report.runtime_activation);
    assert!(!report.prompt_assembly_change);
    assert!(!report.operator_activation_allowed);

    let json =
        serde_json::to_string(&report).expect("shadow regression dashboard should serialize");
    assert!(json.contains("shadow_only"));
    assert!(json.contains("ranked_recall_fixture_count"));
    assert!(json.contains("ranked_recall_comparison_summary_pass"));
    assert!(json.contains("ranked_recall_min_positive_hybrid_score_basis_points"));
    assert!(json.contains("ranked_recall_min_positive_reranking_delta_basis_points"));
    assert!(json.contains("ranked_recall_min_positive_token_tradeoff_basis_points"));
    assert!(json.contains("ranked_recall_routing_diff_shadow_only_count"));
    assert!(json.contains("ranked_recall_min_positive_routing_diff_delta_basis_points"));
    assert!(json.contains("ranked_recall_min_positive_routing_diff_token_tradeoff_basis_points"));
    assert!(json.contains("ranked_recall_real_workload_trace_slo_pass_count"));
    assert!(json.contains("ranked_recall_min_positive_real_workload_trace_coverage_basis_points"));
    assert!(json.contains("ranked_recall_min_positive_real_workload_trace_precision_basis_points"));
    assert!(json.contains("ranked_recall_real_workload_trace_total_leak_count"));
    assert!(json.contains("ranked_recall_real_workload_trace_operator_review_required_count"));
    assert!(json.contains("temporal_graph_fixture_count"));
    assert!(json.contains("recall_quality_blocking_reason_count"));
    assert!(json.contains("provider_payload_light"));
    assert!(!json.contains("session-"));
    assert!(!json.contains("memory-"));
    assert!(!json.contains("source_id"));
    assert!(!json.contains("prompt_text"));
    assert!(!json.contains("transcript_text"));
    assert!(!json.contains("memory_text"));
    assert!(!json.contains("answer_text"));
    assert!(!json.contains("query_payload"));
    assert!(!json.contains("raw_ranked_payload"));
    assert!(!json.contains("raw_graph_payload"));
    assert!(!json.contains("operator_identity"));
    assert!(!json.contains("\"production_route\":true"));
    assert!(!json.contains("\"production_write\":true"));
    assert!(!json.contains("\"graph_write\":true"));
    assert!(!json.contains("\"runtime_activation\":true"));
    assert!(!json.contains("\"prompt_assembly_change\":true"));
    assert!(!json.contains("\"operator_activation_allowed\":true"));
}

#[test]
fn context_memory_shadow_regression_dashboard_blocks_input_regression_drift() {
    let ranked_recall = ContextMemoryRankedRecallShadowEvalReport::seeded();
    let temporal_graph = ContextMemoryTemporalGraphShadowEvalReport::seeded();
    let recall_quality = ContextMemoryRecallQualityGateReport::seeded();
    let mut provider = memory_provider_report_fixture();
    provider.update_context.runtime_activation = true;

    let report = ContextMemoryShadowRegressionDashboardReport::from_reports(
        &ranked_recall,
        &temporal_graph,
        &recall_quality,
        &provider,
    );

    assert_eq!(report.input_report_pass_count, 3);
    assert_eq!(report.regression_blocking_count, 1);
    assert!(report.runtime_activation);
    assert!(!report.has_shadow_regression_dashboard_integrity());
}

#[test]
fn context_memory_shadow_regression_dashboard_blocks_ranked_recall_comparison_false_green() {
    let mut ranked_recall = ContextMemoryRankedRecallShadowEvalReport::seeded();
    let first_positive = ranked_recall
        .fixtures
        .iter_mut()
        .find(|fixture| fixture.positive_fixture)
        .expect("positive ranked recall fixture should exist");
    first_positive.hybrid_score_basis_points = 7_700;
    first_positive.reranking_delta_basis_points = 300;
    let temporal_graph = ContextMemoryTemporalGraphShadowEvalReport::seeded();
    let recall_quality = ContextMemoryRecallQualityGateReport::seeded();
    let provider = memory_provider_report_fixture();

    let report = ContextMemoryShadowRegressionDashboardReport::from_reports(
        &ranked_recall,
        &temporal_graph,
        &recall_quality,
        &provider,
    );

    assert!(!ranked_recall.has_ranked_recall_shadow_integrity());
    assert_eq!(report.input_report_pass_count, 3);
    assert!(!report.ranked_recall_comparison_summary_pass);
    assert!(report.regression_blocking_count > 0);
    assert!(!report.has_shadow_regression_dashboard_integrity());
}

#[test]
fn context_memory_shadow_regression_dashboard_blocks_routing_diff_false_green() {
    let mut ranked_recall = ContextMemoryRankedRecallShadowEvalReport::seeded();
    let first_positive = ranked_recall
        .fixtures
        .iter_mut()
        .find(|fixture| fixture.positive_fixture)
        .expect("positive ranked recall fixture should exist");
    first_positive.routing_diff_shadow_only = false;
    let temporal_graph = ContextMemoryTemporalGraphShadowEvalReport::seeded();
    let recall_quality = ContextMemoryRecallQualityGateReport::seeded();
    let provider = memory_provider_report_fixture();

    let report = ContextMemoryShadowRegressionDashboardReport::from_reports(
        &ranked_recall,
        &temporal_graph,
        &recall_quality,
        &provider,
    );

    assert!(!ranked_recall.has_ranked_recall_shadow_integrity());
    assert_eq!(report.input_report_pass_count, 3);
    assert!(!report.ranked_recall_comparison_summary_pass);
    assert!(report.regression_blocking_count > 0);
    assert!(!report.has_shadow_regression_dashboard_integrity());
}

#[test]
fn context_memory_shadow_regression_dashboard_blocks_real_workload_slo_false_green() {
    let mut ranked_recall = ContextMemoryRankedRecallShadowEvalReport::seeded();
    let first_positive = ranked_recall
        .fixtures
        .iter_mut()
        .find(|fixture| fixture.positive_fixture)
        .expect("positive ranked recall fixture should exist");
    first_positive.real_workload_trace_slo_pass = true;
    first_positive.real_workload_trace_leak_count = 1;
    first_positive.real_workload_trace_leak_rate_basis_points = 2_000;
    let temporal_graph = ContextMemoryTemporalGraphShadowEvalReport::seeded();
    let recall_quality = ContextMemoryRecallQualityGateReport::seeded();
    let provider = memory_provider_report_fixture();

    let report = ContextMemoryShadowRegressionDashboardReport::from_reports(
        &ranked_recall,
        &temporal_graph,
        &recall_quality,
        &provider,
    );

    assert!(!ranked_recall.has_ranked_recall_shadow_integrity());
    assert_eq!(report.input_report_pass_count, 3);
    assert!(!report.ranked_recall_comparison_summary_pass);
    assert!(report.regression_blocking_count > 0);
    assert!(!report.has_shadow_regression_dashboard_integrity());
}

#[test]
fn context_memory_shadow_quality_summary_rolls_up_dashboard_without_activation() {
    let ranked_recall = ContextMemoryRankedRecallShadowEvalReport::seeded();
    let temporal_graph = ContextMemoryTemporalGraphShadowEvalReport::seeded();
    let recall_quality = ContextMemoryRecallQualityGateReport::seeded();
    let provider = memory_provider_report_fixture();
    let dashboard = ContextMemoryShadowRegressionDashboardReport::from_reports(
        &ranked_recall,
        &temporal_graph,
        &recall_quality,
        &provider,
    );

    let report = ContextMemoryShadowQualitySummaryReport::from_dashboard(&dashboard);

    assert!(report.has_shadow_quality_summary_integrity());
    assert_eq!(
        report.schema_version,
        CONTEXT_MEMORY_SHADOW_QUALITY_SUMMARY_SCHEMA_VERSION
    );
    assert_eq!(
        report.mode,
        ContextMemoryShadowQualitySummaryMode::ShadowOnly
    );
    assert_eq!(
        report.quality_trend,
        ContextMemoryShadowQualityTrend::StablePass
    );
    assert_eq!(
        report.operator_summary,
        ContextMemoryShadowQualityOperatorSummary::ReadyShadowOnly
    );
    assert!(report.source_dashboard_pass);
    assert_eq!(report.source_input_report_count, 4);
    assert_eq!(report.source_input_report_pass_count, 4);
    assert_eq!(report.quality_signal_count, 4);
    assert_eq!(report.quality_signal_pass_count, 4);
    assert_eq!(report.regression_blocking_count, 0);
    assert_eq!(report.operator_summary_line_count, 4);
    assert!(report.operator_summary_redacted);
    assert!(report.ranked_recall_signal_pass);
    assert_eq!(report.ranked_recall_min_positive_recall_basis_points, 8000);
    assert_eq!(
        report.ranked_recall_min_positive_precision_basis_points,
        8000
    );
    assert_eq!(report.ranked_recall_total_positive_token_saved, 2_140);
    assert_eq!(report.ranked_recall_max_positive_latency_ms, 55);
    assert!(report.ranked_recall_comparison_summary_pass);
    assert_eq!(report.ranked_recall_hybrid_signal_count, 5);
    assert_eq!(report.ranked_recall_positive_hybrid_signal_pass_count, 15);
    assert_eq!(report.ranked_recall_hybrid_regression_blocked_count, 1);
    assert_eq!(
        report.ranked_recall_min_positive_hybrid_score_basis_points,
        7_800
    );
    assert_eq!(report.ranked_recall_calibrated_reranking_win_count, 3);
    assert_eq!(report.ranked_recall_calibrated_reranking_loss_count, 1);
    assert_eq!(
        report.ranked_recall_min_positive_reranking_delta_basis_points,
        640
    );
    assert_eq!(report.ranked_recall_max_positive_latency_delta_ms, 10);
    assert_eq!(
        report.ranked_recall_min_positive_token_tradeoff_basis_points,
        3_000
    );
    assert_eq!(report.ranked_recall_reranking_regression_blocked_count, 1);
    assert_eq!(report.ranked_recall_routing_diff_shadow_only_count, 4);
    assert_eq!(report.ranked_recall_routing_diff_win_count, 3);
    assert_eq!(report.ranked_recall_routing_diff_loss_count, 1);
    assert_eq!(
        report.ranked_recall_min_positive_routing_diff_delta_basis_points,
        640
    );
    assert_eq!(
        report.ranked_recall_max_positive_routing_diff_latency_delta_ms,
        10
    );
    assert_eq!(
        report.ranked_recall_min_positive_routing_diff_token_tradeoff_basis_points,
        3_000
    );
    assert_eq!(
        report.ranked_recall_routing_diff_regression_blocked_count,
        1
    );
    assert!(report.temporal_graph_signal_pass);
    assert_eq!(
        report.temporal_graph_min_positive_node_coverage_basis_points,
        10_000
    );
    assert_eq!(
        report.temporal_graph_min_positive_edge_coverage_basis_points,
        10_000
    );
    assert_eq!(report.temporal_graph_max_positive_latency_ms, 47);
    assert!(report.recall_quality_signal_pass);
    assert_eq!(report.recall_quality_observed_recall_basis_points, 7777);
    assert_eq!(report.recall_quality_observed_precision_basis_points, 7777);
    assert!(report.provider_boundary_signal_pass);
    assert_eq!(report.provider_estimated_token_count, 320);
    assert!(report.operator_approval_required);
    assert!(!report.production_route);
    assert!(!report.production_write);
    assert!(!report.graph_write);
    assert!(!report.runtime_activation);
    assert!(!report.prompt_assembly_change);
    assert!(!report.operator_activation_allowed);

    let json = serde_json::to_string(&report).expect("shadow quality summary should serialize");
    assert!(json.contains("stable_pass"));
    assert!(json.contains("ready_shadow_only"));
    assert!(json.contains("quality_signal_count"));
    assert!(json.contains("operator_summary_line_count"));
    assert!(json.contains("ranked_recall_signal_pass"));
    assert!(json.contains("ranked_recall_comparison_summary_pass"));
    assert!(json.contains("ranked_recall_min_positive_hybrid_score_basis_points"));
    assert!(json.contains("ranked_recall_min_positive_reranking_delta_basis_points"));
    assert!(json.contains("ranked_recall_min_positive_token_tradeoff_basis_points"));
    assert!(json.contains("ranked_recall_min_positive_routing_diff_delta_basis_points"));
    assert!(json.contains("ranked_recall_min_positive_routing_diff_token_tradeoff_basis_points"));
    assert!(json.contains("ranked_recall_real_workload_trace_slo_pass_count"));
    assert!(json.contains("ranked_recall_min_positive_real_workload_trace_coverage_basis_points"));
    assert!(json.contains("ranked_recall_real_workload_trace_total_leak_count"));
    assert!(json.contains("temporal_graph_signal_pass"));
    assert!(json.contains("recall_quality_signal_pass"));
    assert!(json.contains("provider_boundary_signal_pass"));
    assert!(!json.contains("session-"));
    assert!(!json.contains("memory-"));
    assert!(!json.contains("source_id"));
    assert!(!json.contains("prompt_text"));
    assert!(!json.contains("transcript_text"));
    assert!(!json.contains("memory_text"));
    assert!(!json.contains("answer_text"));
    assert!(!json.contains("query_text"));
    assert!(!json.contains("raw_ranked_payload"));
    assert!(!json.contains("raw_graph_payload"));
    assert!(!json.contains("operator_identity"));
    assert!(!json.contains("\"production_route\":true"));
    assert!(!json.contains("\"production_write\":true"));
    assert!(!json.contains("\"graph_write\":true"));
    assert!(!json.contains("\"runtime_activation\":true"));
}

#[test]
fn context_memory_shadow_quality_summary_blocks_dashboard_regression_drift() {
    let ranked_recall = ContextMemoryRankedRecallShadowEvalReport::seeded();
    let temporal_graph = ContextMemoryTemporalGraphShadowEvalReport::seeded();
    let recall_quality = ContextMemoryRecallQualityGateReport::seeded();
    let mut provider = memory_provider_report_fixture();
    provider.update_context.prompt_payload_exported = true;
    let dashboard = ContextMemoryShadowRegressionDashboardReport::from_reports(
        &ranked_recall,
        &temporal_graph,
        &recall_quality,
        &provider,
    );

    let report = ContextMemoryShadowQualitySummaryReport::from_dashboard(&dashboard);

    assert!(!dashboard.has_shadow_regression_dashboard_integrity());
    assert!(!report.source_dashboard_pass);
    assert_eq!(
        report.quality_trend,
        ContextMemoryShadowQualityTrend::RegressionBlocked
    );
    assert_eq!(
        report.operator_summary,
        ContextMemoryShadowQualityOperatorSummary::BlockedRegression
    );
    assert!(report.regression_blocking_count > 0);
    assert!(!report.has_shadow_quality_summary_integrity());
}

#[test]
fn context_memory_shadow_quality_trend_snapshot_rolls_up_summary_window_without_activation() {
    let summary = shadow_quality_summary_report_fixture();

    let report = ContextMemoryShadowQualityTrendSnapshotReport::from_summary(&summary);

    assert!(report.has_shadow_quality_trend_snapshot_integrity());
    assert_eq!(
        report.schema_version,
        CONTEXT_MEMORY_SHADOW_QUALITY_TREND_SNAPSHOT_SCHEMA_VERSION
    );
    assert_eq!(
        report.mode,
        ContextMemoryShadowQualityTrendSnapshotMode::ShadowOnly
    );
    assert!(report.source_summary_pass);
    assert_eq!(
        report.current_quality_trend,
        ContextMemoryShadowQualityTrend::StablePass
    );
    assert_eq!(
        report.current_operator_summary,
        ContextMemoryShadowQualityOperatorSummary::ReadyShadowOnly
    );
    assert_eq!(report.current_regression_blocking_count, 0);
    assert_eq!(report.window_observation_count, 3);
    assert_eq!(report.required_pass_streak, 3);
    assert_eq!(report.observed_pass_streak, 3);
    assert_eq!(report.stable_observation_count, 3);
    assert_eq!(report.regression_window_blocking_count, 0);
    assert_eq!(
        report.trend_window_verdict,
        ContextMemoryShadowQualityTrendWindowVerdict::StableWindow
    );
    assert_eq!(report.operator_snapshot_line_count, 5);
    assert!(report.operator_snapshot_redacted);
    assert_eq!(report.quality_signal_count, 4);
    assert_eq!(report.quality_signal_window_pass_count, 12);
    assert_eq!(report.ranked_recall_window_pass_count, 3);
    assert_eq!(report.temporal_graph_window_pass_count, 3);
    assert_eq!(report.recall_quality_window_pass_count, 3);
    assert_eq!(report.provider_boundary_window_pass_count, 3);
    assert_eq!(report.ranked_recall_min_positive_recall_basis_points, 8000);
    assert_eq!(
        report.ranked_recall_min_positive_precision_basis_points,
        8000
    );
    assert_eq!(report.ranked_recall_total_positive_token_saved, 2_140);
    assert_eq!(report.ranked_recall_max_positive_latency_ms, 55);
    assert_eq!(report.ranked_recall_comparison_window_pass_count, 3);
    assert_eq!(report.ranked_recall_routing_diff_window_pass_count, 3);
    assert_eq!(
        report.ranked_recall_real_workload_trace_window_pass_count,
        3
    );
    assert_eq!(
        report.ranked_recall_min_positive_hybrid_score_basis_points,
        7_800
    );
    assert_eq!(
        report.ranked_recall_min_positive_reranking_delta_basis_points,
        640
    );
    assert_eq!(report.ranked_recall_max_positive_latency_delta_ms, 10);
    assert_eq!(
        report.ranked_recall_min_positive_token_tradeoff_basis_points,
        3_000
    );
    assert_eq!(
        report.ranked_recall_min_positive_routing_diff_delta_basis_points,
        640
    );
    assert_eq!(
        report.ranked_recall_max_positive_routing_diff_latency_delta_ms,
        10
    );
    assert_eq!(
        report.ranked_recall_min_positive_routing_diff_token_tradeoff_basis_points,
        3_000
    );
    assert_eq!(report.ranked_recall_real_workload_trace_slo_pass_count, 3);
    assert_eq!(report.ranked_recall_real_workload_trace_win_count, 3);
    assert_eq!(report.ranked_recall_real_workload_trace_loss_count, 1);
    assert_eq!(
        report.ranked_recall_real_workload_trace_operator_review_required_count,
        4
    );
    assert_eq!(report.ranked_recall_real_workload_trace_total_leak_count, 0);
    assert_eq!(
        report.ranked_recall_real_workload_trace_max_leak_rate_basis_points,
        0
    );
    assert_eq!(
        report.ranked_recall_min_positive_real_workload_trace_coverage_basis_points,
        8_000
    );
    assert_eq!(
        report.ranked_recall_min_positive_real_workload_trace_precision_basis_points,
        8_000
    );
    assert_eq!(
        report.ranked_recall_total_positive_real_workload_trace_token_saved,
        2_140
    );
    assert_eq!(
        report.ranked_recall_max_positive_real_workload_trace_latency_ms,
        55
    );
    assert_eq!(
        report.temporal_graph_min_positive_node_coverage_basis_points,
        10_000
    );
    assert_eq!(
        report.temporal_graph_min_positive_edge_coverage_basis_points,
        10_000
    );
    assert_eq!(report.temporal_graph_max_positive_latency_ms, 47);
    assert_eq!(report.recall_quality_observed_recall_basis_points, 7777);
    assert_eq!(report.recall_quality_observed_precision_basis_points, 7777);
    assert_eq!(report.provider_estimated_token_count, 320);
    assert!(report.operator_approval_required);
    assert!(!report.history_persistence_write);
    assert!(!report.production_route);
    assert!(!report.production_write);
    assert!(!report.graph_write);
    assert!(!report.runtime_activation);
    assert!(!report.prompt_assembly_change);
    assert!(!report.operator_activation_allowed);

    let json =
        serde_json::to_string(&report).expect("shadow quality trend snapshot should serialize");
    assert!(json.contains("stable_window"));
    assert!(json.contains("window_observation_count"));
    assert!(json.contains("observed_pass_streak"));
    assert!(json.contains("quality_signal_window_pass_count"));
    assert!(json.contains("ranked_recall_comparison_window_pass_count"));
    assert!(json.contains("ranked_recall_routing_diff_window_pass_count"));
    assert!(json.contains("ranked_recall_real_workload_trace_window_pass_count"));
    assert!(json.contains("ranked_recall_min_positive_hybrid_score_basis_points"));
    assert!(json.contains("ranked_recall_min_positive_reranking_delta_basis_points"));
    assert!(json.contains("ranked_recall_min_positive_routing_diff_delta_basis_points"));
    assert!(json.contains("ranked_recall_min_positive_real_workload_trace_coverage_basis_points"));
    assert!(json.contains("operator_snapshot_redacted"));
    assert!(!json.contains("session-"));
    assert!(!json.contains("memory-"));
    assert!(!json.contains("source_id"));
    assert!(!json.contains("prompt_text"));
    assert!(!json.contains("transcript_text"));
    assert!(!json.contains("memory_text"));
    assert!(!json.contains("answer_text"));
    assert!(!json.contains("query_text"));
    assert!(!json.contains("raw_ranked_payload"));
    assert!(!json.contains("raw_graph_payload"));
    assert!(!json.contains("operator_identity"));
    assert!(!json.contains("\"history_persistence_write\":true"));
    assert!(!json.contains("\"production_route\":true"));
    assert!(!json.contains("\"production_write\":true"));
    assert!(!json.contains("\"graph_write\":true"));
    assert!(!json.contains("\"runtime_activation\":true"));
}

#[test]
fn context_memory_shadow_quality_trend_snapshot_blocks_summary_regression_drift() {
    let mut summary = shadow_quality_summary_report_fixture();
    summary.regression_blocking_count = 1;

    let report = ContextMemoryShadowQualityTrendSnapshotReport::from_summary(&summary);

    assert!(!report.source_summary_pass);
    assert_eq!(report.observed_pass_streak, 0);
    assert_eq!(report.stable_observation_count, 0);
    assert!(report.regression_window_blocking_count > 0);
    assert_eq!(
        report.trend_window_verdict,
        ContextMemoryShadowQualityTrendWindowVerdict::RegressionBlocked
    );
    assert!(!report.has_shadow_quality_trend_snapshot_integrity());
}

#[test]
fn context_memory_shadow_canary_promotion_readiness_rehearses_without_activation() {
    let summary = shadow_quality_summary_report_fixture();
    let trend_snapshot = ContextMemoryShadowQualityTrendSnapshotReport::from_summary(&summary);

    let report =
        ContextMemoryShadowCanaryPromotionReadinessReport::from_trend_snapshot(&trend_snapshot);

    assert!(report.has_shadow_canary_promotion_readiness_integrity());
    assert_eq!(
        report.schema_version,
        CONTEXT_MEMORY_SHADOW_CANARY_PROMOTION_READINESS_SCHEMA_VERSION
    );
    assert_eq!(
        report.mode,
        ContextMemoryShadowCanaryPromotionMode::ShadowOnly
    );
    assert!(report.source_trend_snapshot_pass);
    assert_eq!(
        report.source_trend_window_verdict,
        ContextMemoryShadowQualityTrendWindowVerdict::StableWindow
    );
    assert_eq!(report.required_stable_window_count, 1);
    assert_eq!(report.observed_stable_window_count, 1);
    assert_eq!(report.required_pass_streak, 3);
    assert_eq!(report.observed_pass_streak, 3);
    assert_eq!(
        report.promotion_decision,
        ContextMemoryShadowCanaryPromotionDecision::ReadyShadowOnly
    );
    assert_eq!(report.promotion_blocker_count, 0);
    assert_eq!(report.regression_window_blocking_count, 0);
    assert_eq!(
        report.rollback_rehearsal_verdict,
        ContextMemoryShadowCanaryRehearsalVerdict::Covered
    );
    assert_eq!(report.rollback_rehearsal_count, 3);
    assert_eq!(report.rollback_rehearsal_pass_count, 3);
    assert_eq!(report.rollback_rehearsal_blocking_count, 0);
    assert_eq!(
        report.kill_switch_rehearsal_verdict,
        ContextMemoryShadowCanaryRehearsalVerdict::Covered
    );
    assert_eq!(report.kill_switch_rehearsal_count, 3);
    assert_eq!(report.kill_switch_rehearsal_pass_count, 3);
    assert_eq!(
        report.soak_readback_verdict,
        ContextMemoryShadowCanaryRehearsalVerdict::Covered
    );
    assert_eq!(report.soak_readback_window_count, 3);
    assert_eq!(report.soak_readback_pass_count, 3);
    assert_eq!(report.operator_packet_line_count, 6);
    assert!(report.operator_packet_redacted);
    assert!(report.operator_approval_required);
    assert!(!report.history_persistence_write);
    assert!(!report.production_route);
    assert!(!report.production_write);
    assert!(!report.graph_write);
    assert!(!report.runtime_activation);
    assert!(!report.prompt_assembly_change);
    assert!(!report.operator_activation_allowed);
    assert!(!report.canary_promotion_route_opened);
    assert!(!report.rollback_write);

    let json =
        serde_json::to_string(&report).expect("shadow canary promotion readiness should serialize");
    assert!(json.contains("ready_shadow_only"));
    assert!(json.contains("rollback_rehearsal_verdict"));
    assert!(json.contains("kill_switch_rehearsal_verdict"));
    assert!(json.contains("soak_readback_verdict"));
    assert!(json.contains("operator_packet_redacted"));
    assert!(!json.contains("session-"));
    assert!(!json.contains("memory-"));
    assert!(!json.contains("source_id"));
    assert!(!json.contains("prompt_text"));
    assert!(!json.contains("transcript_text"));
    assert!(!json.contains("memory_text"));
    assert!(!json.contains("answer_text"));
    assert!(!json.contains("query_text"));
    assert!(!json.contains("raw_ranked_payload"));
    assert!(!json.contains("raw_graph_payload"));
    assert!(!json.contains("operator_identity"));
    assert!(!json.contains("activation_command"));
    assert!(!json.contains("\"history_persistence_write\":true"));
    assert!(!json.contains("\"production_route\":true"));
    assert!(!json.contains("\"production_write\":true"));
    assert!(!json.contains("\"graph_write\":true"));
    assert!(!json.contains("\"runtime_activation\":true"));
    assert!(!json.contains("\"canary_promotion_route_opened\":true"));
    assert!(!json.contains("\"rollback_write\":true"));
}

#[test]
fn context_memory_shadow_canary_promotion_readiness_blocks_trend_regression_drift() {
    let mut summary = shadow_quality_summary_report_fixture();
    summary.regression_blocking_count = 1;
    let trend_snapshot = ContextMemoryShadowQualityTrendSnapshotReport::from_summary(&summary);

    let report =
        ContextMemoryShadowCanaryPromotionReadinessReport::from_trend_snapshot(&trend_snapshot);

    assert!(!report.source_trend_snapshot_pass);
    assert_eq!(
        report.source_trend_window_verdict,
        ContextMemoryShadowQualityTrendWindowVerdict::RegressionBlocked
    );
    assert_eq!(report.observed_stable_window_count, 0);
    assert_eq!(
        report.promotion_decision,
        ContextMemoryShadowCanaryPromotionDecision::BlockedRegression
    );
    assert!(report.promotion_blocker_count > 0);
    assert_eq!(
        report.rollback_rehearsal_verdict,
        ContextMemoryShadowCanaryRehearsalVerdict::Blocked
    );
    assert_eq!(report.rollback_rehearsal_pass_count, 0);
    assert_eq!(report.rollback_rehearsal_blocking_count, 3);
    assert_eq!(
        report.kill_switch_rehearsal_verdict,
        ContextMemoryShadowCanaryRehearsalVerdict::Blocked
    );
    assert_eq!(
        report.soak_readback_verdict,
        ContextMemoryShadowCanaryRehearsalVerdict::Blocked
    );
    assert!(!report.has_shadow_canary_promotion_readiness_integrity());
}

#[test]
fn context_memory_shadow_canary_promotion_negative_rehearsal_blocks_activation_shaped_side_effects()
{
    let summary = shadow_quality_summary_report_fixture();
    let trend_snapshot = ContextMemoryShadowQualityTrendSnapshotReport::from_summary(&summary);
    let report =
        ContextMemoryShadowCanaryPromotionReadinessReport::from_trend_snapshot(&trend_snapshot);
    assert!(report.has_shadow_canary_promotion_readiness_integrity());

    let mut history_write = report.clone();
    history_write.history_persistence_write = true;
    assert!(!history_write.has_shadow_canary_promotion_readiness_integrity());

    let mut production_route = report.clone();
    production_route.production_route = true;
    assert!(!production_route.has_shadow_canary_promotion_readiness_integrity());

    let mut production_write = report.clone();
    production_write.production_write = true;
    assert!(!production_write.has_shadow_canary_promotion_readiness_integrity());

    let mut graph_write = report.clone();
    graph_write.graph_write = true;
    assert!(!graph_write.has_shadow_canary_promotion_readiness_integrity());

    let mut runtime_activation = report.clone();
    runtime_activation.runtime_activation = true;
    assert!(!runtime_activation.has_shadow_canary_promotion_readiness_integrity());

    let mut prompt_assembly_change = report.clone();
    prompt_assembly_change.prompt_assembly_change = true;
    assert!(!prompt_assembly_change.has_shadow_canary_promotion_readiness_integrity());

    let mut operator_activation = report.clone();
    operator_activation.operator_activation_allowed = true;
    assert!(!operator_activation.has_shadow_canary_promotion_readiness_integrity());

    let mut canary_promotion_route = report.clone();
    canary_promotion_route.canary_promotion_route_opened = true;
    assert!(!canary_promotion_route.has_shadow_canary_promotion_readiness_integrity());

    let mut rollback_write = report;
    rollback_write.rollback_write = true;
    assert!(!rollback_write.has_shadow_canary_promotion_readiness_integrity());
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
