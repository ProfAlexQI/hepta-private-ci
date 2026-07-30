use std::collections::BTreeMap;
use std::collections::BTreeSet;

use super::*;

#[test]
fn topic_aware_model_router_prefers_coding_specialist_without_switching() {
    let report = route_topic_aware_model(TopicAwareModelRouterInput {
        session_id: "session-main".into(),
        intent: "debug rust compile failure in the CLI".into(),
        active_model: Some(ModelRef {
            provider: "demo".into(),
            model: "demo-chat".into(),
        }),
        available_models: vec![
            ModelRef {
                provider: "demo".into(),
                model: "demo-chat".into(),
            },
            ModelRef {
                provider: "openai-codex".into(),
                model: "gpt-5.5".into(),
            },
        ],
        topic_labels: vec!["Rust coding".into()],
        topic_ids: vec![TopicId("topic-rust-coding".into())],
        workflow_ids: vec!["compile-test".into()],
        skill_ids: vec![],
        safety_sensitive: false,
        model_feedback: vec![],
    });

    assert!(report.dry_run_only);
    assert_eq!(report.router_id, TOPIC_AWARE_MODEL_ROUTER_ID);
    assert_eq!(report.session_id, "session-main");
    assert!(report.dominant_specialties.contains(&"coding".into()));
    assert_eq!(
        report.recommended_model,
        Some(ModelRef {
            provider: "openai-codex".into(),
            model: "gpt-5.5".into(),
        })
    );
    assert!(report.should_switch);
    assert!(
        report
            .candidates
            .first()
            .expect("top candidate should exist")
            .matched_specialties
            .contains(&"coding".into())
    );
}

#[test]
fn topic_aware_model_router_applies_topic_model_feedback_delta() {
    let topic_id = TopicId("topic-rust-coding".into());
    let demo = ModelRef {
        provider: "demo".into(),
        model: "demo-chat".into(),
    };
    let codex = ModelRef {
        provider: "openai-codex".into(),
        model: "gpt-5.5".into(),
    };
    let summaries = summarize_topic_aware_model_feedback(&[
        TopicAwareModelFeedbackRecord {
            session_id: "session-main".into(),
            user_intent: "debug rust compile failure".into(),
            model: demo.clone(),
            outcome: TopicAwareModelFeedbackOutcome::ExecutedSuccess,
            topic_ids: vec![topic_id.clone()],
            weight_delta: TopicAwareModelFeedbackOutcome::ExecutedSuccess.weight_delta(),
            latency_ms: Some(900),
            cost: Some(0.01),
            safety_score: Some(0.9),
            user_acceptance: Some(0.9),
            reason: Some("demo was enough for this local task".into()),
            created_at_unix_ms: 1,
        },
        TopicAwareModelFeedbackRecord {
            session_id: "session-main".into(),
            user_intent: "debug rust compile failure".into(),
            model: codex.clone(),
            outcome: TopicAwareModelFeedbackOutcome::ExecutedFailed,
            topic_ids: vec![topic_id.clone()],
            weight_delta: TopicAwareModelFeedbackOutcome::ExecutedFailed.weight_delta(),
            latency_ms: Some(9000),
            cost: Some(0.20),
            safety_score: Some(0.4),
            user_acceptance: Some(0.2),
            reason: Some("overkill and failed this topic".into()),
            created_at_unix_ms: 2,
        },
    ]);

    let report = route_topic_aware_model(TopicAwareModelRouterInput {
        session_id: "session-main".into(),
        intent: "debug rust compile failure".into(),
        active_model: Some(demo.clone()),
        available_models: vec![demo.clone(), codex.clone()],
        topic_labels: vec!["Rust coding".into()],
        topic_ids: vec![topic_id],
        workflow_ids: vec![],
        skill_ids: vec![],
        safety_sensitive: false,
        model_feedback: summaries,
    });

    assert_eq!(report.recommended_model, Some(demo.clone()));
    let demo_candidate = report
        .candidates
        .iter()
        .find(|candidate| candidate.model == demo)
        .expect("demo candidate should exist");
    assert!(demo_candidate.feedback_score_delta > 0.0);
    let codex_candidate = report
        .candidates
        .iter()
        .find(|candidate| candidate.model == codex)
        .expect("codex candidate should exist");
    assert!(codex_candidate.feedback_score_delta < 0.0);
}

#[test]
fn corpora_have_release_hardening_coverage() {
    assert!(golden_eval_case_count() >= 50);
    assert!(stress_replay_turn_count() >= 20);
    assert!(
        GOLDEN_EVAL_CASES
            .iter()
            .any(|case| case.contains("learned signal"))
    );
    assert!(
        STRESS_REPLAY_TURNS
            .iter()
            .any(|turn| turn.contains("export import"))
    );
}

#[test]
fn stress_feedback_outcome_policy_is_deterministic() {
    assert_eq!(
        stress_replay_feedback_outcome("unsafe destructive write should be blocked"),
        IntuitionFeedbackOutcome::UnsafeBlocked
    );
    assert_eq!(
        stress_replay_feedback_outcome("tool failed while reading"),
        IntuitionFeedbackOutcome::ToolFailed
    );
    assert_eq!(
        stress_replay_feedback_outcome("reject wrong workflow"),
        IntuitionFeedbackOutcome::Rejected
    );
    assert_eq!(
        stress_replay_feedback_outcome("accepted workflow success"),
        IntuitionFeedbackOutcome::ExecutedSuccess
    );
}

#[test]
fn recall_evidence_summary_marks_provenance_and_limit_pressure() {
    let request = hepta_core::ContextRecallRequest {
        session_id: hepta_core::SessionId("alpha".into()),
        query_text: Some("adaptive memory".into()),
        recent_window_limit: 2,
        transcript_limit: 2,
        memory_limit: 2,
        allow_cross_session: true,
    };
    let entry = hepta_core::TranscriptEntry {
        entry_id: "entry-1".into(),
        session_id: request.session_id.clone(),
        sequence: 7,
        kind: hepta_core::TranscriptEntryKind::Message,
        role: Some(hepta_core::MessageRole::User),
        content: "adaptive memory needs transcript provenance".into(),
        created_at_unix_ms: 1,
        tool_name: None,
        correlation_id: None,
        summary_of_range: None,
    };
    let bundle = hepta_core::ContextRecallBundle {
        request: request.clone(),
        recent_entries: vec![entry.clone()],
        transcript_hits: vec![hepta_core::TranscriptSpan::from_entry(entry)],
        durable_memory_hits: Vec::new(),
        summary_hits: Vec::new(),
        active_topic_sessions: Vec::new(),
        active_neurons: Vec::new(),
        budget: hepta_core::ContextBudget::from_request(&request),
        ranked_items: Vec::new(),
        omitted_by_budget: 0,
        truncated: false,
    };

    let summary = recall_evidence_summary(
        &bundle,
        hepta_core::ContextRecallAvailability {
            total_recent_entry_count: 2,
            total_transcript_match_count: 3,
            total_memory_match_count: 1,
        },
    );

    assert_eq!(summary.readiness, RecallEvidenceReadiness::ProvenanceBacked);
    assert!(summary.evidence_ready());
    assert_eq!(summary.transcript_evidence_span_count, 1);
    assert_eq!(summary.transcript_provenance.session_count, 1);
    assert_eq!(summary.returned_query_hit_count, 1);
    assert_eq!(summary.matched_query_hit_count, 4);
    assert_eq!(summary.omitted_item_count, 4);
    assert!(
        summary
            .findings
            .iter()
            .any(|finding| finding.contains("omitted"))
    );
    assert!(
        summary
            .findings
            .iter()
            .any(|finding| finding.contains("cross-session"))
    );
}

#[test]
fn neuron_lifecycle_health_summary_flags_unhealthy_lifecycle_state() {
    let topic_a = hepta_core::TopicId("topic:a".into());
    let topic_b = hepta_core::TopicId("topic:b".into());
    let topic_sessions = vec![
        hepta_core::TopicSession {
            topic_session_id: "ts:a".into(),
            topic_id: topic_a.clone(),
            topic_label: hepta_core::TopicLabel("alpha topic".into()),
            topic_embedding: None,
            linked_surface_session_ids: Vec::new(),
            linked_transcript_spans: Vec::new(),
            open_loops: Vec::new(),
            entities: BTreeMap::new(),
            graph_edges: Vec::new(),
            durable_memory_refs: Vec::new(),
            status: hepta_core::TopicSessionStatus::Active,
            created_at_unix_ms: 1,
            last_active_unix_ms: 2,
        },
        hepta_core::TopicSession {
            topic_session_id: "ts:b".into(),
            topic_id: topic_b,
            topic_label: hepta_core::TopicLabel("beta topic".into()),
            topic_embedding: None,
            linked_surface_session_ids: Vec::new(),
            linked_transcript_spans: Vec::new(),
            open_loops: Vec::new(),
            entities: BTreeMap::new(),
            graph_edges: Vec::new(),
            durable_memory_refs: Vec::new(),
            status: hepta_core::TopicSessionStatus::Active,
            created_at_unix_ms: 1,
            last_active_unix_ms: 2,
        },
    ];
    let neuron = hepta_core::HeptaNeuron {
        neuron_id: hepta_core::NeuronId("neuron:a".into()),
        topic_id: topic_a,
        topic_label: hepta_core::TopicLabel("alpha topic".into()),
        topic_embedding_centroid: None,
        linked_session_ids: vec![
            hepta_core::SessionId("alpha".into()),
            hepta_core::SessionId("beta".into()),
        ],
        linked_topic_session_ids: vec!["ts:a".into()],
        important_transcript_spans: Vec::new(),
        promoted_memory_refs: Vec::new(),
        entity_state: BTreeMap::new(),
        stable_preferences: Vec::new(),
        open_loops: Vec::new(),
        skill_priors: Vec::new(),
        workflow_priors: Vec::new(),
        links: Vec::new(),
        neuron_revision: 2,
        compression_policy_version: "bootstrap-v1".into(),
        source_evidence_digest: None,
        last_refresh_reason: Some("test".into()),
        staleness_score: 0.91,
        merged_from: vec![hepta_core::NeuronId("neuron:old".into())],
        split_from: Vec::new(),
        supersedes: Vec::new(),
        confidence: 0.10,
        freshness: 0.05,
        last_revalidated_unix_ms: 3,
    };

    let summary = neuron_lifecycle_health_summary(&topic_sessions, &[neuron]);

    assert!(!summary.healthy);
    assert_eq!(summary.total_topic_sessions, 2);
    assert_eq!(summary.active_topic_sessions, 2);
    assert_eq!(summary.stored_neurons, 1);
    assert_eq!(summary.active_topics_without_neurons, vec!["topic:b"]);
    assert_eq!(summary.lineage_neurons, 1);
    assert_eq!(summary.merged_neurons, 1);
    assert_eq!(summary.merge_split_lineage_edges, 1);
    assert_eq!(summary.stale_neurons, 1);
    assert_eq!(summary.low_confidence_neurons, 1);
    assert_eq!(summary.low_freshness_neurons, 1);
    assert_eq!(summary.cross_session_unstable_neurons, 1);
    assert!(
        summary
            .findings
            .iter()
            .any(|finding| finding.contains("active topics"))
    );
    assert!(
        summary
            .findings
            .iter()
            .any(|finding| finding.contains("transcript"))
    );
    assert!(
        summary
            .findings
            .iter()
            .any(|finding| finding.contains("stale"))
    );
}

#[test]
fn semantic_expectation_evaluator_scores_domain_expectations() {
    let bundle = hepta_core::IntuitionBundle {
        request: hepta_core::IntuitionRequest {
            surface_session_id: hepta_core::SessionId("alpha".into()),
            user_intent: "read file adaptive memory workflow intelligence".into(),
            topic_limit: 3,
            neuron_limit: 2,
            skill_limit: 2,
        },
        topic_activation_scores: Vec::new(),
        neuron_activations: Vec::new(),
        source_transcript_spans: Vec::new(),
        foreground_topic_session_ids: Vec::new(),
        skill_decisions: vec![hepta_core::SkillActivationDecision {
            skill_id: "read_file".into(),
            workflow_id: Some("workflow:file-inspection".into()),
            score: 1.0,
            exists_in_registry: true,
            missing_capability: None,
            risk_tier: None,
            requires_confirmation: false,
            action_mode: hepta_core::IntuitionActionMode::Prepare,
            source_topic_ids: Vec::new(),
            source_neuron_ids: Vec::new(),
            reason: None,
        }],
        workflow_priors: vec![hepta_core::WorkflowPrior {
            workflow_id: "workflow:file-inspection".into(),
            score: 1.0,
            exists_in_registry: true,
            missing_capability: None,
            requires_confirmation: false,
            action_mode: hepta_core::IntuitionActionMode::Prepare,
            source_topic_ids: Vec::new(),
            source_neuron_ids: Vec::new(),
            reason: None,
        }],
        explanation: None,
        truncated: false,
    };

    let passed = evaluate_intelligence_semantic_expectations(
        "read file adaptive memory workflow intelligence",
        1,
        1,
        1,
        1,
        1,
        1,
        &bundle,
    );
    assert_eq!(passed.score(), 100);
    assert!(passed.failures.is_empty());

    let failed = evaluate_intelligence_semantic_expectations(
        "write file release notes",
        1,
        1,
        1,
        1,
        1,
        1,
        &bundle,
    );
    assert!(failed.score() < 100);
    assert!(
        failed
            .failures
            .iter()
            .any(|failure| failure.contains("file-write"))
    );
}

#[test]
fn bootstrap_candidate_matching_helpers_score_surface_and_semantic_aliases() {
    let topic_session = hepta_core::TopicSession {
        topic_session_id: "ts:memory".into(),
        topic_id: hepta_core::TopicId("topic:memory".into()),
        topic_label: hepta_core::TopicLabel("adaptive memory".into()),
        topic_embedding: None,
        linked_surface_session_ids: Vec::new(),
        linked_transcript_spans: Vec::new(),
        open_loops: Vec::new(),
        entities: BTreeMap::new(),
        graph_edges: Vec::new(),
        durable_memory_refs: Vec::new(),
        status: hepta_core::TopicSessionStatus::Active,
        created_at_unix_ms: 1,
        last_active_unix_ms: 2,
    };

    assert_eq!(
        bootstrap_semantic_term("executor").as_deref(),
        Some("worker")
    );
    assert_eq!(
        extract_semantic_terms("recall history context", 4),
        vec!["memory"]
    );
    assert_eq!(
        bootstrap_candidate_topic_label(Some("  adaptive memory  "), "fallback"),
        "adaptive memory"
    );
    assert_eq!(
        bootstrap_candidate_topic_label(None, "fallback"),
        "fallback"
    );
    assert_eq!(
        bootstrap_candidate_topic_labels(
            Some("adaptive memory and rust worker pipeline, shallow"),
            "fallback",
            4,
        ),
        vec!["adaptive memory", "rust worker pipeline"]
    );
    assert_eq!(
        bootstrap_candidate_topic_labels(Some("memory and memory"), "fallback", 4),
        vec!["memory and memory"]
    );
    assert_eq!(
        extract_bootstrap_semantic_hints_for_match("adaptive recall context", &topic_session, 4,),
        vec!["recall", "context"]
    );
    assert_eq!(
        extract_bootstrap_semantic_hints_from_overlap("history notes", &topic_session, 4),
        vec!["history"]
    );

    let features = compute_bootstrap_topic_match_features(
        "history recall",
        "history-recall",
        &topic_session,
        "adaptive-memory",
    );

    assert!(features.score > 0.80);
    assert_eq!(features.matched_terms, vec!["memory"]);

    let selected = select_bootstrap_topic_match_candidate(
        vec![
            BootstrapTopicMatchCandidate {
                index: 1,
                features: BootstrapTopicMatchFeatures {
                    score: 0.40,
                    matched_terms: vec!["worker".into()],
                },
            },
            BootstrapTopicMatchCandidate {
                index: 2,
                features: features.clone(),
            },
        ],
        0.55,
    )
    .expect("best candidate should pass threshold");
    assert_eq!(selected.index, 2);

    assert_eq!(
        rank_bootstrap_graph_route_candidates(
            vec![
                BootstrapGraphRouteRankCandidate {
                    target_index: 4,
                    strength: 0.60,
                    last_active_unix_ms: 30,
                },
                BootstrapGraphRouteRankCandidate {
                    target_index: 2,
                    strength: 0.90,
                    last_active_unix_ms: 10,
                },
                BootstrapGraphRouteRankCandidate {
                    target_index: 3,
                    strength: 0.60,
                    last_active_unix_ms: 40,
                },
            ],
            2,
        ),
        vec![2, 3]
    );

    let persisted = infer_bootstrap_persisted_topic_graph_link(
        "adaptive memory",
        "memory",
        hepta_core::TopicGraphEdgeKind::CoActivation,
        "co_activation",
        0.81,
    );
    assert_eq!(persisted.strength, 0.81);
    assert_eq!(persisted.matched_terms, vec!["memory"]);
    assert!(persisted.reason.contains("stored co-activation edge"));

    let heuristic = infer_bootstrap_heuristic_topic_graph_link(
        "adaptive memory provenance",
        true,
        "bootstrap route",
        "adaptive memory",
        hepta_core::TopicSessionStatus::Active,
    )
    .expect("composite label adjacency should score");
    assert_eq!(heuristic.strength, 0.58);
    assert_eq!(heuristic.matched_terms, vec!["adaptive", "memory"]);
    assert!(heuristic.reason.contains("composite label adjacency"));

    let make_route = |label: &str, existing_index: Option<usize>| BootstrapTopicRouteCandidate {
        topic_id: hepta_core::TopicId(format!("topic:{label}")),
        topic_label: hepta_core::TopicLabel(label.to_string()),
        topic_session_id: format!("ts:{label}"),
        matched_terms: vec![label.to_string()],
        semantic_hints: Vec::new(),
        topic_score: 0.50,
        reason: format!("route {label}"),
        existing_index,
        was_active: existing_index.is_some(),
        graph_routed: false,
    };
    let mut builder_calls = Vec::new();
    let selection = select_bootstrap_initial_topic_routes(
        Vec::new(),
        vec!["alpha", "beta", "gamma"],
        2,
        |selected_existing_indices, label, has_prior_routes| {
            builder_calls.push((
                label.to_string(),
                selected_existing_indices
                    .iter()
                    .copied()
                    .collect::<Vec<_>>(),
                has_prior_routes,
            ));
            make_route(label, Some(if label == "alpha" { 1 } else { 2 }))
        },
    );
    assert_eq!(selection.routes.len(), 2);
    assert_eq!(selection.routes[0].topic_label.0, "alpha");
    assert_eq!(selection.routes[1].topic_label.0, "beta");
    assert_eq!(selection.selected_existing_indices, BTreeSet::from([1, 2]));
    assert_eq!(
        builder_calls,
        vec![
            ("alpha".to_string(), Vec::<usize>::new(), false),
            ("beta".to_string(), vec![1], true),
        ]
    );

    let implicit_selection = select_bootstrap_initial_topic_routes(
        vec![
            make_route("implicit-a", Some(7)),
            make_route("implicit-b", Some(8)),
        ],
        vec!["unused"],
        1,
        |_, _, _| panic!("implicit multi-route planning should not build explicit routes"),
    );
    assert_eq!(implicit_selection.routes.len(), 1);
    assert_eq!(
        implicit_selection.selected_existing_indices,
        BTreeSet::from([7])
    );

    let merged =
        select_bootstrap_merged_topic_route(&selection.routes, Some("merge"), |routes, marker| {
            assert_eq!(routes.len(), 2);
            assert_eq!(marker, "merge");
            make_route("merged", Some(2))
        })
        .expect("merge selection should be produced");
    assert_eq!(merged.selection.routes[0].topic_label.0, "merged");
    assert_eq!(
        merged.selection.selected_existing_indices,
        BTreeSet::from([2])
    );
    assert_eq!(merged.merged_source_indices, BTreeSet::from([1]));

    let appended = append_bootstrap_graph_topic_routes(
        merged.selection,
        vec![
            make_route("duplicate", Some(2)),
            make_route("graph", Some(3)),
        ],
        2,
    );
    assert_eq!(appended.routes.len(), 2);
    assert_eq!(appended.routes[1].topic_label.0, "graph");
    assert_eq!(appended.selected_existing_indices, BTreeSet::from([2, 3]));

    let mut facade_builder_calls = Vec::new();
    let mut facade_graph_calls = 0;
    let planner_outcome = BootstrapTopicRoutePlanner::new(3).plan(
        Vec::new(),
        vec!["alpha", "beta"],
        None,
        None,
        |selected_existing_indices, label, has_prior_routes| {
            facade_builder_calls.push((
                label.to_string(),
                selected_existing_indices
                    .iter()
                    .copied()
                    .collect::<Vec<_>>(),
                has_prior_routes,
            ));
            make_route(label, Some(if label == "alpha" { 1 } else { 2 }))
        },
        |_, _| panic!("merge route should not be built without marker"),
        |selected_existing_indices, routes| {
            facade_graph_calls += 1;
            assert_eq!(routes.len(), 2);
            assert_eq!(*selected_existing_indices, BTreeSet::from([1, 2]));
            vec![make_route("graph", Some(3))]
        },
    );
    assert_eq!(planner_outcome.routes.len(), 3);
    assert_eq!(planner_outcome.routes[2].topic_label.0, "graph");
    assert_eq!(
        planner_outcome.selected_existing_indices,
        BTreeSet::from([1, 2, 3])
    );
    assert!(planner_outcome.merged_source_indices.is_empty());
    assert_eq!(facade_graph_calls, 1);
    assert_eq!(facade_builder_calls.len(), 2);

    let merged_outcome = BootstrapTopicRoutePlanner::new(3).plan(
        Vec::new(),
        vec!["alpha", "beta"],
        Some("merge"),
        None,
        |_, label, _| make_route(label, Some(if label == "alpha" { 1 } else { 2 })),
        |routes, marker| {
            assert_eq!(routes.len(), 2);
            assert_eq!(marker, "merge");
            make_route("merged", None)
        },
        |_, _| panic!("graph routes should be skipped for merge planning"),
    );
    assert_eq!(merged_outcome.routes.len(), 1);
    assert_eq!(merged_outcome.routes[0].topic_label.0, "merged");
    assert!(merged_outcome.selected_existing_indices.is_empty());
    assert_eq!(merged_outcome.merged_source_indices, BTreeSet::from([1, 2]));

    struct TestMaterializer {
        builder_calls: Vec<(String, Vec<usize>, bool)>,
        graph_calls: usize,
    }

    impl TestMaterializer {
        fn route(label: &str, existing_index: Option<usize>) -> BootstrapTopicRouteCandidate {
            BootstrapTopicRouteCandidate {
                topic_id: hepta_core::TopicId(format!("topic:{label}")),
                topic_label: hepta_core::TopicLabel(label.to_string()),
                topic_session_id: format!("ts:{label}"),
                matched_terms: vec![label.to_string()],
                semantic_hints: Vec::new(),
                topic_score: 0.60,
                reason: format!("materialized {label}"),
                existing_index,
                was_active: existing_index.is_some(),
                graph_routed: false,
            }
        }
    }

    impl BootstrapTopicRouteMaterializer for TestMaterializer {
        fn build_candidate_route(
            &mut self,
            selected_existing_indices: &BTreeSet<usize>,
            candidate_label: &str,
            has_prior_routes: bool,
        ) -> BootstrapTopicRouteCandidate {
            self.builder_calls.push((
                candidate_label.to_string(),
                selected_existing_indices
                    .iter()
                    .copied()
                    .collect::<Vec<_>>(),
                has_prior_routes,
            ));
            Self::route(
                candidate_label,
                Some(if candidate_label == "alpha" { 1 } else { 2 }),
            )
        }

        fn build_merged_route(
            &mut self,
            _routes: &[BootstrapTopicRouteCandidate],
            marker: &'static str,
        ) -> BootstrapTopicRouteCandidate {
            Self::route(marker, None)
        }

        fn infer_graph_routes(
            &mut self,
            selected_existing_indices: &BTreeSet<usize>,
            routes: &[BootstrapTopicRouteCandidate],
        ) -> Vec<BootstrapTopicRouteCandidate> {
            self.graph_calls += 1;
            assert_eq!(routes.len(), 2);
            assert_eq!(*selected_existing_indices, BTreeSet::from([1, 2]));
            vec![Self::route("graph", Some(3))]
        }
    }

    let mut materializer = TestMaterializer {
        builder_calls: Vec::new(),
        graph_calls: 0,
    };
    let adapter_outcome = BootstrapTopicRoutePlanner::new(3).plan_with_materializer(
        Vec::new(),
        vec!["alpha", "beta"],
        None,
        None,
        &mut materializer,
    );
    assert_eq!(adapter_outcome.routes.len(), 3);
    assert_eq!(adapter_outcome.routes[2].topic_label.0, "graph");
    assert_eq!(
        adapter_outcome.selected_existing_indices,
        BTreeSet::from([1, 2, 3])
    );
    assert_eq!(materializer.graph_calls, 1);
    assert_eq!(
        materializer.builder_calls,
        vec![
            ("alpha".to_string(), Vec::<usize>::new(), false),
            ("beta".to_string(), vec![1], true),
        ]
    );

    let mut trait_materializer = TestMaterializer {
        builder_calls: Vec::new(),
        graph_calls: 0,
    };
    let router: &dyn SemanticRouter = &BootstrapSemanticRouter;
    assert_eq!(router.router_id(), SEMANTIC_ROUTER_BOOTSTRAP_ID);
    let trait_outcome = router.route(
        BootstrapSemanticRouterInput {
            implicit_routes: Vec::new(),
            candidate_labels: vec!["alpha".into(), "beta".into()],
            merge_marker: None,
            split_marker: None,
            limit: 3,
            learned_signals: Vec::new(),
        },
        &mut trait_materializer,
    );
    assert_eq!(trait_outcome.routes.len(), 3);
    assert_eq!(trait_outcome.routes[2].topic_label.0, "graph");
    assert_eq!(trait_materializer.graph_calls, 1);

    let registry = SemanticRouterRegistry::new();
    assert_eq!(registry.default_router_id(), SEMANTIC_ROUTER_BOOTSTRAP_ID);
    assert_eq!(
        registry.supported_router_ids(),
        &[
            SEMANTIC_ROUTER_BOOTSTRAP_ID,
            SEMANTIC_ROUTER_LEARNED_ID,
            SEMANTIC_ROUTER_NO_FEEDBACK_ID,
        ]
    );
    assert_eq!(
        registry
            .get(SEMANTIC_ROUTER_BOOTSTRAP_ID)
            .expect("bootstrap router should be registered")
            .router_id(),
        SEMANTIC_ROUTER_BOOTSTRAP_ID
    );
    assert_eq!(
        registry
            .get(SEMANTIC_ROUTER_LEARNED_ID)
            .expect("learned feedback router should be registered")
            .router_id(),
        SEMANTIC_ROUTER_LEARNED_ID
    );
    assert_eq!(
        registry
            .get(SEMANTIC_ROUTER_NO_FEEDBACK_ID)
            .expect("no-feedback router should be registered")
            .router_id(),
        SEMANTIC_ROUTER_NO_FEEDBACK_ID
    );
    assert!(registry.get("semantic-router:missing").is_none());

    let mut registry_materializer = TestMaterializer {
        builder_calls: Vec::new(),
        graph_calls: 0,
    };
    let registry_outcome = registry.select(None).route(
        BootstrapSemanticRouterInput {
            implicit_routes: Vec::new(),
            candidate_labels: vec!["alpha".into(), "beta".into()],
            merge_marker: None,
            split_marker: None,
            limit: 3,
            learned_signals: Vec::new(),
        },
        &mut registry_materializer,
    );
    assert_eq!(registry_outcome.routes.len(), 3);
    assert_eq!(registry_outcome.routes[2].topic_label.0, "graph");
    assert_eq!(registry_materializer.graph_calls, 1);
    assert_eq!(
        registry.select(Some("semantic-router:missing")).router_id(),
        SEMANTIC_ROUTER_BOOTSTRAP_ID
    );
    assert_eq!(
        registry.select_for_learned_signal_count(0).router_id(),
        SEMANTIC_ROUTER_BOOTSTRAP_ID
    );
    assert_eq!(
        registry.select_for_learned_signal_count(2).router_id(),
        SEMANTIC_ROUTER_LEARNED_ID
    );

    let mut learned_materializer = TestMaterializer {
        builder_calls: Vec::new(),
        graph_calls: 0,
    };
    let learned_outcome = registry.select(Some(SEMANTIC_ROUTER_LEARNED_ID)).route(
        BootstrapSemanticRouterInput {
            implicit_routes: Vec::new(),
            candidate_labels: vec!["alpha".into(), "beta".into()],
            merge_marker: None,
            split_marker: None,
            limit: 3,
            learned_signals: Vec::new(),
        },
        &mut learned_materializer,
    );
    assert_eq!(learned_outcome.routes.len(), 3);
    assert_eq!(learned_outcome.routes[2].topic_label.0, "graph");
    assert_eq!(learned_materializer.graph_calls, 1);

    let mut learned_signal_materializer = TestMaterializer {
        builder_calls: Vec::new(),
        graph_calls: 0,
    };
    let mut alpha_route = TestMaterializer::route("alpha", Some(1));
    alpha_route.topic_score = 0.62;
    let mut beta_route = TestMaterializer::route("beta", Some(2));
    beta_route.topic_score = 0.55;
    let learned_signal_outcome = LearnedSemanticRoutePlanner::new(3).plan_with_materializer(
        BootstrapSemanticRouterInput {
            implicit_routes: vec![alpha_route, beta_route],
            candidate_labels: Vec::new(),
            merge_marker: None,
            split_marker: None,
            limit: 3,
            learned_signals: vec![
                LearnedSemanticRouterSignal {
                    topic_id: hepta_core::TopicId("topic:beta".into()),
                    delta: 0.14,
                    matched_terms: vec!["beta".into(), "memory".into()],
                    source: "test-signal".into(),
                },
                LearnedSemanticRouterSignal {
                    topic_id: hepta_core::TopicId("topic:alpha".into()),
                    delta: -0.18,
                    matched_terms: vec!["alpha".into()],
                    source: "negative-test-signal".into(),
                },
            ],
        },
        &mut learned_signal_materializer,
    );
    assert_eq!(learned_signal_outcome.routes[0].topic_label.0, "beta");
    assert!(
        learned_signal_outcome.routes[0]
            .reason
            .contains("planning adjusted")
    );
    assert_eq!(learned_signal_outcome.routes[1].topic_label.0, "alpha");
    assert!(
        learned_signal_outcome.routes[1]
            .reason
            .contains("conflict suppressed")
    );
    assert!(learned_signal_outcome.routes[1].topic_score <= 0.40);
    assert_eq!(learned_signal_materializer.graph_calls, 1);

    let mut learned_label_materializer = TestMaterializer {
        builder_calls: Vec::new(),
        graph_calls: 0,
    };
    let _ = LearnedSemanticRoutePlanner::new(3).plan_with_materializer(
        BootstrapSemanticRouterInput {
            implicit_routes: Vec::new(),
            candidate_labels: vec!["alpha".into(), "beta".into()],
            merge_marker: None,
            split_marker: None,
            limit: 3,
            learned_signals: vec![LearnedSemanticRouterSignal {
                topic_id: hepta_core::TopicId("topic:beta".into()),
                delta: 0.10,
                matched_terms: vec!["beta".into(), "memory".into()],
                source: "test-signal".into(),
            }],
        },
        &mut learned_label_materializer,
    );
    assert_eq!(learned_label_materializer.builder_calls[0].0, "beta");

    let bootstrap_report = registry.learned_composition_report(&[]);
    assert_eq!(bootstrap_report.router_id, SEMANTIC_ROUTER_BOOTSTRAP_ID);
    assert_eq!(bootstrap_report.learned_signal_count, 0);
    assert!(bootstrap_report.learned_router_signals.is_empty());
    let learned_report = registry.learned_composition_report(&[
        LearnedSemanticRouterSignal {
            topic_id: hepta_core::TopicId("topic:memory".into()),
            delta: 0.10,
            matched_terms: vec!["memory".into(), "adaptive".into()],
            source: "topic-feedback-hints".into(),
        },
        LearnedSemanticRouterSignal {
            topic_id: hepta_core::TopicId("topic:worker".into()),
            delta: 0.04,
            matched_terms: vec!["worker".into()],
            source: "recent-feedback".into(),
        },
    ]);
    assert_eq!(learned_report.router_id, SEMANTIC_ROUTER_LEARNED_ID);
    assert_eq!(learned_report.learned_signal_count, 2);
    assert_eq!(learned_report.learned_router_signals.len(), 2);
    assert!(learned_report.learned_router_signals[0].contains("topic:memory"));
    let count_report = registry.learned_composition_report_from_count(3);
    assert_eq!(count_report.router_id, SEMANTIC_ROUTER_LEARNED_ID);
    assert_eq!(count_report.learned_signal_count, 3);
    assert!(count_report.learned_router_signals.is_empty());
    let forced_bootstrap_report = registry.learned_composition_report_for_router(
        Some(SEMANTIC_ROUTER_BOOTSTRAP_ID),
        &[LearnedSemanticRouterSignal {
            topic_id: hepta_core::TopicId("topic:memory".into()),
            delta: 0.08,
            matched_terms: vec!["memory".into()],
            source: "manual-router-selection".into(),
        }],
    );
    assert_eq!(
        forced_bootstrap_report.router_id,
        SEMANTIC_ROUTER_BOOTSTRAP_ID
    );
    assert_eq!(forced_bootstrap_report.learned_signal_count, 1);
    let forced_no_feedback_report = registry.learned_composition_report_for_router(
        Some(SEMANTIC_ROUTER_NO_FEEDBACK_ID),
        &[LearnedSemanticRouterSignal {
            topic_id: hepta_core::TopicId("topic:memory".into()),
            delta: 0.08,
            matched_terms: vec!["memory".into()],
            source: "manual-router-selection".into(),
        }],
    );
    assert_eq!(
        forced_no_feedback_report.router_id,
        SEMANTIC_ROUTER_NO_FEEDBACK_ID
    );
    assert_eq!(forced_no_feedback_report.learned_signal_count, 0);
    assert!(forced_no_feedback_report.learned_router_signals.is_empty());
    let forced_learned_report = registry
        .learned_composition_report_for_router_from_count(Some(SEMANTIC_ROUTER_LEARNED_ID), 0);
    assert_eq!(forced_learned_report.router_id, SEMANTIC_ROUTER_LEARNED_ID);
    assert_eq!(forced_learned_report.learned_signal_count, 0);
    let fallback_report = registry
        .learned_composition_report_for_router_from_count(Some("semantic-router:missing"), 5);
    assert_eq!(fallback_report.router_id, SEMANTIC_ROUTER_BOOTSTRAP_ID);
    assert_eq!(fallback_report.learned_signal_count, 5);

    assert_eq!(
        select_bootstrap_implicit_topic_match_candidates(
            vec![
                BootstrapImplicitTopicMatchCandidate {
                    index: 1,
                    score: 0.90,
                    matched_terms: vec!["memory".into(), "adaptive".into()],
                    was_active: false,
                    last_active_unix_ms: 50,
                },
                BootstrapImplicitTopicMatchCandidate {
                    index: 2,
                    score: 0.90,
                    matched_terms: vec!["worker".into(), "pipeline".into()],
                    was_active: true,
                    last_active_unix_ms: 10,
                },
                BootstrapImplicitTopicMatchCandidate {
                    index: 3,
                    score: 0.70,
                    matched_terms: vec!["router".into(), "semantic".into()],
                    was_active: true,
                    last_active_unix_ms: 70,
                },
                BootstrapImplicitTopicMatchCandidate {
                    index: 4,
                    score: 0.95,
                    matched_terms: vec!["solo".into()],
                    was_active: true,
                    last_active_unix_ms: 100,
                },
            ],
            3,
            0.52,
            2,
        )
        .into_iter()
        .map(|candidate| candidate.index)
        .collect::<Vec<_>>(),
        vec![2, 1, 3]
    );
    assert!(
        select_bootstrap_implicit_topic_match_candidates(
            vec![BootstrapImplicitTopicMatchCandidate {
                index: 1,
                score: 0.90,
                matched_terms: vec!["memory".into(), "adaptive".into()],
                was_active: true,
                last_active_unix_ms: 10,
            }],
            3,
            0.52,
            2,
        )
        .is_empty()
    );
}

#[test]
fn learned_router_signals_apply_feedback_hints_to_scores() {
    let topic_id = hepta_core::TopicId("topic:file".into());
    let mut entities = BTreeMap::new();
    entities.insert(SEMANTIC_ROUTER_LEARNED_KEY.into(), "true".into());
    entities.insert(SEMANTIC_ROUTER_NET_DELTA_KEY.into(), "0.20".into());
    entities.insert(format!("{}memory", SEMANTIC_HINT_PREFIX), "memory".into());

    let topic_session = hepta_core::TopicSession {
        topic_session_id: "ts:file".into(),
        topic_id: topic_id.clone(),
        topic_label: hepta_core::TopicLabel("file memory".into()),
        topic_embedding: None,
        linked_surface_session_ids: Vec::new(),
        linked_transcript_spans: Vec::new(),
        open_loops: Vec::new(),
        entities,
        graph_edges: Vec::new(),
        durable_memory_refs: Vec::new(),
        status: hepta_core::TopicSessionStatus::Active,
        created_at_unix_ms: 1,
        last_active_unix_ms: 1,
    };
    let mut scores = vec![hepta_core::TopicActivationScore {
        topic_id: topic_id.clone(),
        topic_label: hepta_core::TopicLabel("file memory".into()),
        score: 0.40,
        matched_terms: vec!["file".into()],
        reason: Some("bootstrap route".into()),
    }];

    let topic_sessions = vec![topic_session];
    let evidence = LearnedSemanticRouterEvidence::new(&topic_sessions, &[], &[]);
    assert!(!evidence.is_empty());
    let signals = evidence.collect_signals(Some("please recall memory"), &scores);
    let wrapper_signals = collect_learned_semantic_router_signals(
        Some("please recall memory"),
        &scores,
        &topic_sessions,
        &[],
        &[],
    );

    assert_eq!(signals, wrapper_signals);
    assert_eq!(signals.len(), 1);
    assert_eq!(signals[0].topic_id, topic_id);
    assert_eq!(signals[0].matched_terms, vec!["memory"]);
    let registry = SemanticRouterRegistry::new();
    let evidence_report =
        registry.learned_evidence_report(Some("please recall memory"), &scores, &evidence);
    assert_eq!(evidence_report.signals, signals);
    assert_eq!(
        evidence_report.composition.router_id,
        SEMANTIC_ROUTER_LEARNED_ID
    );
    assert_eq!(evidence_report.composition.learned_signal_count, 1);
    assert_eq!(evidence_report.composition.learned_router_signals.len(), 1);
    let forced_bootstrap_report = registry.learned_evidence_report_for_router(
        Some(SEMANTIC_ROUTER_BOOTSTRAP_ID),
        Some("please recall memory"),
        &scores,
        &evidence,
    );
    assert_eq!(
        forced_bootstrap_report.composition.router_id,
        SEMANTIC_ROUTER_BOOTSTRAP_ID
    );
    assert_eq!(forced_bootstrap_report.composition.learned_signal_count, 1);
    let forced_no_feedback_report = registry.learned_evidence_report_for_router(
        Some(SEMANTIC_ROUTER_NO_FEEDBACK_ID),
        Some("please recall memory"),
        &scores,
        &evidence,
    );
    assert_eq!(
        forced_no_feedback_report.composition.router_id,
        SEMANTIC_ROUTER_NO_FEEDBACK_ID
    );
    assert_eq!(
        forced_no_feedback_report.composition.learned_signal_count,
        0
    );
    assert!(forced_no_feedback_report.signals.is_empty());
    let learned_router = LearnedFeedbackSemanticRouter;
    let learned_router_report =
        learned_router.evidence_report(Some("please recall memory"), &scores, &evidence);
    assert_eq!(learned_router_report.signals, signals);
    assert_eq!(
        learned_router_report.composition.router_id,
        SEMANTIC_ROUTER_LEARNED_ID
    );
    let empty_learned_router_report = learned_router.evidence_report_from_signals(Vec::new());
    assert_eq!(
        empty_learned_router_report.composition.router_id,
        SEMANTIC_ROUTER_LEARNED_ID
    );
    assert_eq!(
        empty_learned_router_report.composition.learned_signal_count,
        0
    );
    let mut learned_run_scores = scores.clone();
    let learned_run_report = learned_router.run_report(
        Some("please recall memory"),
        &mut learned_run_scores,
        &evidence,
    );
    assert_eq!(learned_run_report.evidence.signals, signals);
    assert_eq!(
        learned_run_report.evidence.composition.router_id,
        SEMANTIC_ROUTER_LEARNED_ID
    );
    assert_eq!(learned_run_report.applied.applied_signal_count, 1);
    assert_eq!(
        learned_run_report.applied.primary_topic_id,
        Some(topic_id.clone())
    );
    let learned_route_update = learned_run_report.route_shell_update();
    assert_eq!(
        learned_route_update.primary_topic_id,
        Some(topic_id.clone())
    );
    assert_eq!(
        learned_route_update.shift_to_topic_id,
        Some(topic_id.clone())
    );
    assert_eq!(learned_route_update.applied_signal_count, 1);
    assert_eq!(
        learned_route_update.explanation_suffix.as_deref(),
        Some("1 learned semantic router signal(s) applied by semantic-router:learned-feedback-v1")
    );
    let learned_route_patch = learned_run_report.route_shell_patch();
    assert_eq!(
        learned_route_patch,
        learned_route_update.topic_route_shell_patch()
    );
    assert!(!learned_route_patch.is_empty());
    assert!(TopicRouteShellPatch::from_primary_topic(None).is_empty());
    assert_eq!(
        TopicRouteShellPatch::from_primary_topic(Some(topic_id.clone()))
            .without_explanation_suffix()
            .explanation_suffix,
        None
    );
    assert_eq!(learned_route_patch.primary_topic_id, Some(topic_id.clone()));
    assert_eq!(
        learned_route_patch.shift_to_topic_id,
        Some(topic_id.clone())
    );
    assert_eq!(
        learned_route_patch.explanation_suffix.as_deref(),
        Some("1 learned semantic router signal(s) applied by semantic-router:learned-feedback-v1")
    );
    assert!(learned_run_scores[0].score > 0.40);
    let mut registry_run_scores = scores.clone();
    let registry_run_report = registry.learned_run_report(
        Some("please recall memory"),
        &mut registry_run_scores,
        &evidence,
    );
    assert_eq!(registry_run_report.evidence.signals, signals);
    assert_eq!(
        registry_run_report.evidence.composition.router_id,
        SEMANTIC_ROUTER_LEARNED_ID
    );
    assert_eq!(registry_run_report.applied.applied_signal_count, 1);
    assert!(registry_run_scores[0].score > 0.40);
    let mut forced_bootstrap_run_scores = scores.clone();
    let forced_bootstrap_run_report = registry.learned_run_report_for_router(
        Some(SEMANTIC_ROUTER_BOOTSTRAP_ID),
        Some("please recall memory"),
        &mut forced_bootstrap_run_scores,
        &evidence,
    );
    assert_eq!(
        forced_bootstrap_run_report.evidence.composition.router_id,
        SEMANTIC_ROUTER_BOOTSTRAP_ID
    );
    assert_eq!(forced_bootstrap_run_report.applied.applied_signal_count, 1);
    assert_eq!(
        forced_bootstrap_run_report
            .route_shell_update()
            .explanation_suffix
            .as_deref(),
        Some("1 learned semantic router signal(s) applied by semantic-router:bootstrap-v1")
    );
    let mut forced_no_feedback_run_scores = scores.clone();
    let forced_no_feedback_run_report = registry.learned_run_report_for_router(
        Some(SEMANTIC_ROUTER_NO_FEEDBACK_ID),
        Some("please recall memory"),
        &mut forced_no_feedback_run_scores,
        &evidence,
    );
    assert_eq!(
        forced_no_feedback_run_report.evidence.composition.router_id,
        SEMANTIC_ROUTER_NO_FEEDBACK_ID
    );
    assert_eq!(
        forced_no_feedback_run_report.applied.applied_signal_count,
        0
    );
    assert_eq!(forced_no_feedback_run_scores[0].score, 0.40);
    assert!(
        forced_no_feedback_run_report
            .route_shell_update()
            .explanation_suffix
            .is_none()
    );
    let applied_report = learned_router.apply_signals_to_scores_report(&mut scores, &signals);
    assert_eq!(applied_report.signal_count, 1);
    assert_eq!(applied_report.applied_signal_count, 1);
    assert_eq!(applied_report.updated_score_count, 1);
    assert_eq!(applied_report.primary_topic_id, Some(topic_id.clone()));
    assert_eq!(applied_report.score_updates.len(), 1);
    assert_eq!(applied_report.score_updates[0].topic_id, topic_id);
    assert!((applied_report.score_updates[0].before_score - 0.40).abs() < 0.0001);
    assert!(applied_report.score_updates[0].after_score > 0.40);
    assert_eq!(applied_report.score_updates[0].signal_count, 1);
    assert_eq!(applied_report.score_updates[0].signal_summaries.len(), 1);
    assert!(scores[0].score > 0.40);
    assert!(scores[0].matched_terms.contains(&"memory".to_string()));
    assert!(
        scores[0]
            .reason
            .as_deref()
            .unwrap_or_default()
            .contains("learned semantic router applied")
    );
}

#[test]
fn feedback_calibration_targets_group_and_sort_records() {
    let records = vec![
        hepta_core::IntuitionFeedbackRecord {
            decision_id: Some("d1".into()),
            surface_session_id: hepta_core::SessionId("alpha".into()),
            user_intent: "read file".into(),
            outcome: IntuitionFeedbackOutcome::ExecutedSuccess,
            skill_id: Some("read_file".into()),
            workflow_id: Some("workflow:file-inspection".into()),
            source_topic_ids: vec![hepta_core::TopicId("topic:file".into())],
            source_neuron_ids: vec![hepta_core::NeuronId("neuron:file".into())],
            weight_delta: 0.18,
            observed_outcome: None,
            latency_ms: None,
            cost: None,
            user_correction: None,
            confidence_before: Some(0.50),
            confidence_after: Some(0.68),
            reason: Some("ok".into()),
            created_at_unix_ms: 10,
        },
        hepta_core::IntuitionFeedbackRecord {
            decision_id: Some("d2".into()),
            surface_session_id: hepta_core::SessionId("alpha".into()),
            user_intent: "unsafe write".into(),
            outcome: IntuitionFeedbackOutcome::UnsafeBlocked,
            skill_id: Some("write_file".into()),
            workflow_id: Some("workflow:file-change".into()),
            source_topic_ids: vec![hepta_core::TopicId("topic:file".into())],
            source_neuron_ids: vec![hepta_core::NeuronId("neuron:file".into())],
            weight_delta: -0.16,
            observed_outcome: None,
            latency_ms: None,
            cost: None,
            user_correction: None,
            confidence_before: Some(0.70),
            confidence_after: Some(0.54),
            reason: Some("blocked".into()),
            created_at_unix_ms: 11,
        },
    ];

    let skill_targets = intuition_calibration_skill_targets(&records);

    assert_eq!(skill_targets.len(), 2);
    assert_eq!(skill_targets[0].target_kind, "skill");
    assert_eq!(skill_targets[0].feedback_count, 1);
    assert!(skill_targets.iter().any(|target| {
        target.target_id == "read_file"
            && target.positive_feedback_count == 1
            && target.confidence_shift_count == 1
    }));
    assert!(skill_targets.iter().any(|target| {
        target.target_id == "write_file"
            && target.negative_feedback_count == 1
            && target.outcome_counts.get("unsafe_blocked") == Some(&1)
    }));
    let topic_delta = compute_intuition_feedback_delta(
        &records,
        Some(&hepta_core::TopicId("topic:file".into())),
        None,
        None,
        None,
    );
    assert!((topic_delta - 0.02).abs() < 0.0001);
}

#[test]
fn route_outcome_draft_owns_shift_reason_and_shell_patch() {
    let routes = vec![
        BootstrapTopicRouteCandidate {
            topic_id: hepta_core::TopicId("topic:memory".into()),
            topic_label: hepta_core::TopicLabel("adaptive memory".into()),
            topic_session_id: "topic-session:memory".into(),
            matched_terms: vec!["memory".into()],
            semantic_hints: Vec::new(),
            topic_score: 0.82,
            reason: "semantic route matched adaptive memory".into(),
            existing_index: Some(1),
            was_active: true,
            graph_routed: false,
        },
        BootstrapTopicRouteCandidate {
            topic_id: hepta_core::TopicId("topic:provenance".into()),
            topic_label: hepta_core::TopicLabel("provenance review".into()),
            topic_session_id: "topic-session:provenance".into(),
            matched_terms: vec!["provenance".into()],
            semantic_hints: Vec::new(),
            topic_score: 0.63,
            reason: "graph expansion linked provenance".into(),
            existing_index: Some(2),
            was_active: false,
            graph_routed: true,
        },
    ];
    let activation_scores = routes
        .iter()
        .map(|route| hepta_core::TopicActivationScore {
            topic_id: route.topic_id.clone(),
            topic_label: route.topic_label.clone(),
            score: route.topic_score,
            matched_terms: route.matched_terms.clone(),
            reason: Some(route.reason.clone()),
        })
        .collect::<Vec<_>>();
    let previous = vec![hepta_core::TopicId("topic:old".into())];
    let active = vec![
        "topic-session:memory".into(),
        "topic-session:provenance".into(),
    ];
    let created = vec!["topic-session:provenance".into()];
    let revived = Vec::new();
    let draft = build_bootstrap_topic_route_outcome_draft(BootstrapTopicRouteOutcomeDraftInput {
        session_id: "session-main",
        routes: &routes,
        session_indices: &[0],
        previously_active_topic_ids: &previous,
        merged_source_indices: &BTreeSet::new(),
        merge_marker: None,
        split_marker: None,
        activation_scores: &activation_scores,
        active_topic_session_ids: &active,
        created_topic_session_ids: &created,
        revived_topic_session_ids: &revived,
        fallback_topic_label: "fallback",
        has_evidence: true,
        recent_entry_count: 3,
        transcript_matched_count: 2,
        durable_memory_hit_count: 1,
        summary_hit_count: 1,
    });

    assert_eq!(draft.shift_kind, hepta_core::TopicShiftKind::CoActivated);
    assert_eq!(draft.shift_from_topic_id, Some(previous[0].clone()));
    assert_eq!(draft.graph_route_count, 1);
    assert_eq!(draft.semantic_route_count, 1);
    assert_eq!(draft.output_created_topic_session_ids, created);
    assert_eq!(
        draft.route_shell_patch.primary_topic_id,
        Some(hepta_core::TopicId("topic:memory".into()))
    );
    assert!(
        draft
            .route_shell_patch
            .shift_reason
            .as_deref()
            .unwrap_or_default()
            .contains("graph expansion co-activated")
    );
    assert!(
        draft
            .route_shell_patch
            .explanation_replacement
            .as_deref()
            .unwrap_or_default()
            .contains("graph expansion contributed")
    );
}

#[test]
fn learned_contrast_cases_cover_three_feedback_focuses() {
    let focuses = LEARNED_CONTRAST_EVAL_CASES
        .iter()
        .filter_map(|case| learned_feedback_contrast_focus(case))
        .fold(
            BTreeMap::<&'static str, usize>::new(),
            |mut counts, focus| {
                *counts.entry(focus).or_insert(0) += 1;
                counts
            },
        );

    assert_eq!(focuses.get("accepted-feedback-boost"), Some(&4));
    assert_eq!(focuses.get("stale-topic-recovery"), Some(&4));
    assert_eq!(focuses.get("rejected-unsafe-suppression"), Some(&4));
    assert_eq!(learned_contrast_eval_case_count(), 12);
    assert_eq!(
        learned_feedback_contrast_expected_signal_direction(
            "learned contrast accepted feedback boost should amplify semantic router neuron"
        ),
        Some("positive")
    );
    assert_eq!(
        learned_feedback_contrast_expected_signal_direction(
            "learned contrast rejected unsafe suppression should block unsafe workflow recurrence"
        ),
        Some("negative")
    );
    assert_eq!(
        learned_contrast_feedback_outcome(
            "learned contrast stale-topic recovery should explain corrected stale route"
        ),
        hepta_core::IntuitionFeedbackOutcome::Corrected
    );
}

#[test]
fn self_optimization_supervisor_reaches_100_with_full_runtime_signals() {
    let report = self_optimization_supervisor_report(SelfOptimizationSignals {
        skills_tools_maturity_percent: 100,
        skill_workshop_ready: true,
        tool_generation_ready: true,
        runtime_tool_count: 11,
        required_runtime_tool_count: 11,
        golden_semantic_score: 100,
        stress_semantic_score: 100,
        contrast_semantic_score: 100,
        golden_passed_case_count: GOLDEN_EVAL_CASES.len(),
        stress_passed_case_count: STRESS_REPLAY_TURNS.len(),
        contrast_passed_case_count: LEARNED_CONTRAST_EVAL_CASES.len(),
        calibration_closed_loop_ready: true,
        calibration_feedback_record_count: 82,
        multi_agent_overall_percent: 100,
        multi_agent_all_ratings_100: true,
        worker_patch_transactions_ready: true,
        evidence_replay_ready: true,
        external_boundary_respected: true,
        promotion_ledger_ready: true,
    });

    assert_eq!(report.status, "complete");
    assert_eq!(report.capability_count, 10);
    assert_eq!(report.ready_capability_count, 10);
    assert_eq!(report.overall_percent, 100);
    assert!(report.all_self_optimization_ratings_100);
    assert_eq!(report.skill_tool_coordination_percent, 100);
    assert_eq!(report.learned_ranking_percent, 100);
    assert_eq!(report.calibration_feedback_percent, 100);
    assert_eq!(report.multi_agent_supervisor_percent, 100);
    assert_eq!(report.worker_patch_promotion_percent, 100);
    assert_eq!(report.safety_boundary_percent, 100);
    assert!(report.next_blockers.is_empty());
}

#[test]
fn self_optimization_supervisor_does_not_overclaim_without_calibration_or_boundary() {
    let report = self_optimization_supervisor_report(SelfOptimizationSignals {
        skills_tools_maturity_percent: 100,
        skill_workshop_ready: true,
        tool_generation_ready: true,
        runtime_tool_count: 11,
        required_runtime_tool_count: 11,
        golden_semantic_score: 100,
        stress_semantic_score: 100,
        contrast_semantic_score: 100,
        golden_passed_case_count: GOLDEN_EVAL_CASES.len(),
        stress_passed_case_count: STRESS_REPLAY_TURNS.len(),
        contrast_passed_case_count: LEARNED_CONTRAST_EVAL_CASES.len(),
        calibration_closed_loop_ready: false,
        calibration_feedback_record_count: 0,
        multi_agent_overall_percent: 100,
        multi_agent_all_ratings_100: true,
        worker_patch_transactions_ready: true,
        evidence_replay_ready: true,
        external_boundary_respected: false,
        promotion_ledger_ready: true,
    });

    assert_eq!(report.status, "incomplete");
    assert!(report.overall_percent < 100);
    assert!(!report.all_self_optimization_ratings_100);
    assert!(
        report
            .next_blockers
            .iter()
            .any(|blocker| blocker.contains("feedback calibration"))
    );
    assert!(
        report
            .next_blockers
            .iter()
            .any(|blocker| blocker.contains("cognition layer"))
    );
}
