use super::*;

#[test]
fn store_snapshot_context_memory_eval_harness_seed_is_payload_light() {
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

    let report = snapshot.context_memory_eval_harness_seed_report();

    assert!(report.has_eval_integrity());
    assert_eq!(report.fixture_count(), 2);
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
    assert!(report.fixtures.iter().any(|fixture| {
        fixture.fixture_kind == ContextMemoryEvalFixtureKind::SyntheticLongSession
            && fixture.synthetic
            && !fixture.redacted
    }));
    assert!(report.fixtures.iter().any(|fixture| {
        fixture.fixture_kind == ContextMemoryEvalFixtureKind::RedactedTrace
            && fixture.redacted
            && !fixture.synthetic
    }));
    assert_eq!(report.total_missing_critical_fact_count(), 2);
    assert_eq!(report.safety_leak_count(), 0);
    assert_eq!(report.answer_quality_regression_count(), 0);
    assert!(!report.production_write);
    assert!(!report.graph_write);
    assert!(!report.runtime_activation);

    let json = serde_json::to_string(&report).expect("eval harness report should serialize");
    assert!(json.contains("synthetic_long_session"));
    assert!(json.contains("redacted_trace"));
    assert!(json.contains("recall_coverage"));
    assert!(json.contains("answer_quality_regression"));
    assert!(!json.contains("timeout surfaced during tool run"));
    assert!(!json.contains("timeout retried successfully"));
    assert!(!json.contains("session-1"));
    assert!(!json.contains("memory-1"));
    assert!(!json.contains("source_id"));
    assert!(!json.contains("transcript_text"));
    assert!(!json.contains("memory_text"));
    assert!(!json.contains("answer_text"));
    assert!(!json.contains("\"production_write\":true"));
    assert!(!json.contains("\"graph_write\":true"));
    assert!(!json.contains("\"runtime_activation\":true"));
}

#[test]
fn store_snapshot_context_memory_adaptive_allocator_eval_shadow_is_payload_light() {
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

    let report = snapshot.context_memory_adaptive_allocator_eval_shadow_report();

    assert!(report.has_eval_shadow_integrity());
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
        report.total_missing_critical_fact_count_for_arm(
            ContextMemoryAdaptiveAllocatorEvalArm::ProposedAdaptive
        )
    );
    assert!(
        report.total_token_cost_for_arm(ContextMemoryAdaptiveAllocatorEvalArm::ProposedAdaptive)
            < report
                .total_token_cost_for_arm(ContextMemoryAdaptiveAllocatorEvalArm::CurrentHeuristic)
    );
    assert!(
        report.total_token_saved_for_arm(ContextMemoryAdaptiveAllocatorEvalArm::ProposedAdaptive)
            > report
                .total_token_saved_for_arm(ContextMemoryAdaptiveAllocatorEvalArm::CurrentHeuristic)
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
    assert!(!report.operator_activation_allowed);

    let json = serde_json::to_string(&report).expect("eval shadow report should serialize");
    assert!(json.contains("current_heuristic"));
    assert!(json.contains("proposed_adaptive"));
    assert!(json.contains("synthetic_long_session"));
    assert!(json.contains("redacted_trace"));
    assert!(json.contains("comparison_verdict"));
    assert!(json.contains("shadow_threshold_pass"));
    assert!(json.contains("missing_critical_fact_regression_count"));
    assert!(json.contains("token_saved_regression_count"));
    assert!(!json.contains("timeout surfaced during tool run"));
    assert!(!json.contains("timeout retried successfully"));
    assert!(!json.contains("session-1"));
    assert!(!json.contains("memory-1"));
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
fn store_snapshot_context_memory_recall_quality_gate_is_payload_light() {
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

    let report = snapshot.context_memory_recall_quality_gate_report();

    assert!(report.has_quality_gate_integrity());
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
    assert_eq!(report.observed_recall_coverage_basis_points, 7777);
    assert_eq!(report.recall_regression_count, 0);
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

    assert!(report.fixture_matrix.iter().any(|fixture| {
        fixture.fixture_kind == ContextMemoryEvalFixtureKind::SyntheticLongSession
            && fixture.has_fixture_gate_integrity()
            && fixture.proposed_missing_critical_fact_count == 1
            && fixture.current_missing_critical_fact_count == 1
            && !fixture.missing_critical_fact_regression
            && !fixture.recall_regression
            && !fixture.precision_regression
            && fixture.blocking_reasons.is_empty()
    }));
    assert!(report.fixture_matrix.iter().any(|fixture| {
        fixture.fixture_kind == ContextMemoryEvalFixtureKind::RedactedTrace
            && fixture.has_fixture_gate_integrity()
            && fixture.proposed_missing_critical_fact_count == 1
            && !fixture.missing_critical_fact_regression
            && !fixture.recall_regression
            && !fixture.precision_regression
            && fixture.blocking_reasons.is_empty()
    }));

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
    assert!(!json.contains("timeout surfaced during tool run"));
    assert!(!json.contains("timeout retried successfully"));
    assert!(!json.contains("session-1"));
    assert!(!json.contains("memory-1"));
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
fn store_snapshot_context_memory_ranked_recall_shadow_eval_is_payload_light() {
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

    let report = snapshot.context_memory_ranked_recall_shadow_eval_report();

    assert!(report.has_ranked_recall_shadow_integrity());
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
    assert_eq!(report.fixture_count(), 4);
    assert_eq!(report.fixture_pass_count(), 4);
    assert_eq!(report.positive_fixture_count(), 3);
    assert_eq!(report.negative_fixture_count(), 1);
    assert_eq!(report.ranked_item_fixture_count(), 4);
    assert_eq!(report.regression_blocked_count(), 1);
    assert_eq!(report.min_positive_recall_basis_points(), 8000);
    assert_eq!(report.min_positive_precision_basis_points(), 8000);
    assert_eq!(report.total_positive_token_saved(), 2_140);
    assert_eq!(report.max_positive_latency_ms(), 55);
    assert_eq!(report.max_positive_regret_basis_points(), 0);
    assert!(report.operator_approval_required);
    assert!(!report.production_route);
    assert!(!report.production_write);
    assert!(!report.graph_write);
    assert!(!report.runtime_activation);
    assert!(!report.prompt_assembly_change);
    assert!(!report.operator_activation_allowed);

    let regression = report
        .fixture(ContextMemoryRankedRecallShadowEvalFixtureKind::RegressionGuard)
        .expect("regression guard fixture should exist");
    assert!(regression.negative_fixture);
    assert!(regression.regression_fixture);
    assert!(regression.regression_blocked);

    let json = serde_json::to_string(&report).expect("ranked recall report should serialize");
    assert!(json.contains("deterministic_shadow"));
    assert!(json.contains("query_match"));
    assert!(json.contains("recency_tie_break"));
    assert!(json.contains("budget_pressure"));
    assert!(json.contains("regression_guard"));
    assert!(json.contains("ranked_item_count"));
    assert!(!json.contains("timeout surfaced during tool run"));
    assert!(!json.contains("timeout retried successfully"));
    assert!(!json.contains("session-1"));
    assert!(!json.contains("memory-1"));
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
}

#[test]
fn store_snapshot_context_memory_selected_recall_summary_canary_eval_is_payload_light() {
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

    let report = snapshot.context_memory_selected_recall_summary_canary_eval_report();

    assert!(report.has_canary_eval_integrity());
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

    let regression = report
        .fixture(ContextMemorySelectedRecallSummaryCanaryEvalFixtureKind::RegressionGuard)
        .expect("regression guard fixture should exist");
    assert!(regression.negative_fixture);
    assert!(regression.regression_fixture);
    assert!(regression.regression_blocked);

    let json = serde_json::to_string(&report).expect("selected recall report should serialize");
    assert!(json.contains("golden_replay_shadow"));
    assert!(json.contains("summary_baseline"));
    assert!(json.contains("summary_candidate"));
    assert!(json.contains("rollback_readback"));
    assert!(json.contains("regression_guard"));
    assert!(!json.contains("timeout surfaced during tool run"));
    assert!(!json.contains("timeout retried successfully"));
    assert!(!json.contains("session-1"));
    assert!(!json.contains("memory-1"));
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
}

#[tokio::test]
async fn store_context_memory_eval_harness_seed_matches_snapshot_helper() {
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

    let snapshot = store.snapshot().expect("snapshot should load");
    let from_store = store
        .context_memory_eval_harness_seed_report()
        .expect("context memory eval harness should succeed");

    assert_eq!(
        from_store,
        snapshot.context_memory_eval_harness_seed_report()
    );
    assert!(from_store.has_eval_integrity());
    assert_eq!(from_store.fixture_count(), 2);
    assert_eq!(from_store.total_missing_critical_fact_count(), 2);
    assert_eq!(from_store.safety_leak_count(), 0);
    assert_eq!(from_store.answer_quality_regression_count(), 0);
    assert!(
        from_store
            .fixtures
            .iter()
            .all(|fixture| !fixture.production_write
                && !fixture.graph_write
                && !fixture.runtime_activation)
    );
}

#[tokio::test]
async fn store_context_memory_adaptive_allocator_eval_shadow_matches_snapshot_helper() {
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

    let snapshot = store.snapshot().expect("snapshot should load");
    let from_store = store
        .context_memory_adaptive_allocator_eval_shadow_report()
        .expect("context memory adaptive allocator eval shadow should succeed");

    assert_eq!(
        from_store,
        snapshot.context_memory_adaptive_allocator_eval_shadow_report()
    );
    assert!(from_store.has_eval_shadow_integrity());
    assert_eq!(
        from_store.result_count_for_arm(ContextMemoryAdaptiveAllocatorEvalArm::CurrentHeuristic),
        2
    );
    assert_eq!(
        from_store.result_count_for_arm(ContextMemoryAdaptiveAllocatorEvalArm::ProposedAdaptive),
        2
    );
    assert!(
        from_store
            .comparison_verdict
            .has_shadow_threshold_integrity()
    );
    assert_eq!(
        from_store.comparison_verdict.verdict,
        ContextMemoryAdaptiveAllocatorEvalShadowVerdict::ShadowThresholdPass
    );
    assert_eq!(
        from_store
            .comparison_verdict
            .missing_critical_fact_regression_count,
        0
    );
    assert_eq!(from_store.comparison_verdict.recall_regression_count, 0);
    assert_eq!(from_store.comparison_verdict.precision_regression_count, 0);
    assert_eq!(from_store.comparison_verdict.latency_regression_count, 0);
    assert_eq!(from_store.comparison_verdict.token_cost_regression_count, 0);
    assert_eq!(
        from_store.comparison_verdict.token_saved_regression_count,
        0
    );
    assert_eq!(from_store.safety_leak_count(), 0);
    assert_eq!(from_store.answer_quality_regression_count(), 0);
    assert!(!from_store.production_write);
    assert!(!from_store.graph_write);
    assert!(!from_store.runtime_activation);
    assert!(!from_store.adaptive_allocator_runtime_activation);
    assert!(!from_store.source_aware_runtime_activation);
    assert!(!from_store.operator_activation_allowed);
}

#[tokio::test]
async fn store_context_memory_recall_quality_gate_matches_snapshot_helper() {
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

    let snapshot = store.snapshot().expect("snapshot should load");
    let from_store = store
        .context_memory_recall_quality_gate_report()
        .expect("context memory recall quality gate should succeed");

    assert_eq!(
        from_store,
        snapshot.context_memory_recall_quality_gate_report()
    );
    assert!(from_store.has_quality_gate_integrity());
    assert_eq!(
        from_store.verdict,
        ContextMemoryRecallQualityGateVerdict::GatePass
    );
    assert_eq!(from_store.fixture_count, 2);
    assert_eq!(from_store.blocking_reason_count, 0);
    assert_eq!(from_store.missing_critical_fact_count, 2);
    assert_eq!(from_store.observed_recall_coverage_basis_points, 7777);
    assert_eq!(from_store.observed_precision_basis_points, 7777);
    assert_eq!(from_store.safety_leak_count, 0);
    assert_eq!(from_store.answer_quality_regression_count, 0);
    assert!(!from_store.production_write);
    assert!(!from_store.graph_write);
    assert!(!from_store.runtime_activation);
    assert!(!from_store.adaptive_allocator_runtime_activation);
    assert!(!from_store.source_aware_runtime_activation);
    assert!(!from_store.prompt_assembly_change);
    assert!(!from_store.operator_activation_allowed);
}

#[tokio::test]
async fn store_context_memory_ranked_recall_shadow_eval_matches_snapshot_helper() {
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

    let snapshot = store.snapshot().expect("snapshot should load");
    let from_store = store
        .context_memory_ranked_recall_shadow_eval_report()
        .expect("ranked recall shadow eval should succeed");

    assert_eq!(
        from_store,
        snapshot.context_memory_ranked_recall_shadow_eval_report()
    );
    assert!(from_store.has_ranked_recall_shadow_integrity());
    assert_eq!(from_store.fixture_count(), 4);
    assert_eq!(from_store.fixture_pass_count(), 4);
    assert_eq!(from_store.positive_fixture_count(), 3);
    assert_eq!(from_store.negative_fixture_count(), 1);
    assert_eq!(from_store.ranked_item_fixture_count(), 4);
    assert_eq!(from_store.regression_blocked_count(), 1);
    assert_eq!(from_store.min_positive_recall_basis_points(), 8000);
    assert_eq!(from_store.min_positive_precision_basis_points(), 8000);
    assert_eq!(from_store.total_positive_token_saved(), 2_140);
    assert_eq!(from_store.max_positive_latency_ms(), 55);
    assert_eq!(from_store.max_positive_regret_basis_points(), 0);
    assert!(from_store.operator_approval_required);
    assert!(!from_store.production_route);
    assert!(!from_store.production_write);
    assert!(!from_store.graph_write);
    assert!(!from_store.runtime_activation);
    assert!(!from_store.prompt_assembly_change);
    assert!(!from_store.operator_activation_allowed);
}

#[tokio::test]
async fn store_context_memory_selected_recall_summary_canary_eval_matches_snapshot_helper() {
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

    let snapshot = store.snapshot().expect("snapshot should load");
    let from_store = store
        .context_memory_selected_recall_summary_canary_eval_report()
        .expect("selected recall canary eval should succeed");

    assert_eq!(
        from_store,
        snapshot.context_memory_selected_recall_summary_canary_eval_report()
    );
    assert!(from_store.has_canary_eval_integrity());
    assert_eq!(from_store.fixture_count(), 4);
    assert_eq!(from_store.fixture_pass_count(), 4);
    assert_eq!(from_store.fixture_blocked_count(), 0);
    assert_eq!(from_store.positive_fixture_count(), 3);
    assert_eq!(from_store.negative_fixture_count(), 1);
    assert_eq!(from_store.shadow_vs_live_pair_count(), 3);
    assert_eq!(from_store.rollback_readback_fixture_count(), 1);
    assert_eq!(from_store.regression_blocked_count(), 1);
    assert!(from_store.prompt_input_proof_covered());
    assert!(from_store.response_debug_proof_covered());
    assert!(from_store.operator_approval_required);
    assert!(!from_store.production_route);
    assert!(!from_store.production_write);
    assert!(!from_store.graph_write);
    assert!(!from_store.runtime_activation);
    assert!(!from_store.prompt_assembly_change);
    assert!(!from_store.operator_activation_allowed);
}

#[tokio::test]
async fn store_memory_provider_update_context_is_payload_light_without_activation() {
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
        query_text: Some("timeout retry guidance".into()),
        recent_window_limit: 2,
        transcript_limit: 2,
        memory_limit: 2,
        allow_cross_session: false,
    };

    let bundle = hepta_core::MemoryProvider::query(&store, request.clone())
        .await
        .expect("provider query should succeed");
    let update = hepta_core::MemoryProvider::update_context(&store, request.clone())
        .await
        .expect("provider update_context should succeed");
    let report = hepta_core::MemoryProvider::report(&store, request)
        .await
        .expect("provider report should succeed");

    assert_eq!(update.provider_id, "builtin");
    assert_eq!(update.mode, MemoryProviderContextUpdateMode::ShadowOnly);
    assert_eq!(update.source_counts, bundle.source_counts());
    assert_eq!(update.ranked_item_count, bundle.ranked_items.len());
    assert_eq!(update.selected_item_count, bundle.total_item_count());
    assert!(update.estimated_token_count > 0);
    assert!(update.has_payload_light_boundary());
    assert!(report.has_provider_boundary_integrity());
    assert_eq!(report.descriptor.id, "builtin");
    assert_eq!(report.update_context, update);

    let json =
        serde_json::to_string(&report).expect("provider report should serialize as payload light");
    assert!(!json.contains("timeout retry guidance"));
    assert!(!json.contains("timeout surfaced during tool run"));
    assert!(!json.contains("timeout retried successfully"));
    assert!(!json.contains("session-1"));
    assert!(!json.contains("memory-1"));
    assert!(!json.contains("\"prompt_payload_exported\":true"));
    assert!(!json.contains("\"query_payload_exported\":true"));
    assert!(!json.contains("\"ranked_payload_exported\":true"));
    assert!(!json.contains("\"write_performed\":true"));
    assert!(!json.contains("\"runtime_activation\":true"));
}

#[tokio::test]
async fn store_memory_provider_clear_is_dry_run_or_blocked_without_store_mutation() {
    let store = InMemoryStore::default();
    store
        .put(memory_record(
            "memory-1",
            MemoryScope::LongTerm,
            "timeout retry guidance",
        ))
        .await
        .expect("put should succeed");

    let before = store.list_memories().expect("memories should list");
    let dry_run = hepta_core::MemoryProvider::clear(
        &store,
        MemoryProviderClearRequest {
            scope: MemoryProviderClearScope::All,
            dry_run: true,
            operator_approval_granted: false,
        },
    )
    .await
    .expect("dry-run clear should report");
    let blocked = hepta_core::MemoryProvider::clear(
        &store,
        MemoryProviderClearRequest {
            scope: MemoryProviderClearScope::All,
            dry_run: false,
            operator_approval_granted: false,
        },
    )
    .await
    .expect("blocked clear should report");
    let after = store.list_memories().expect("memories should list");

    assert_eq!(before, after);
    assert!(dry_run.dry_run);
    assert!(!dry_run.blocked);
    assert!(dry_run.operator_approval_required);
    assert!(dry_run.has_no_side_effects());
    assert!(!blocked.dry_run);
    assert!(blocked.blocked);
    assert!(blocked.operator_approval_required);
    assert!(blocked.has_no_side_effects());
}
