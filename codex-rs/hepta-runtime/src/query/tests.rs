    use hepta_core::EventKind;
    use hepta_core::MemoryRecord;
    use hepta_core::MemoryScope;
    use hepta_core::MessageRole;

    use super::*;

    #[test]
    fn topic_route_shell_patch_application_updates_route_shell_fields() {
        let topic_id = TopicId("topic:patched".into());
        let mut route = BootstrapTopicRouteOutcome {
            primary_topic_id: None,
            active_topic_session_ids: Vec::new(),
            created_topic_session_ids: Vec::new(),
            revived_topic_session_ids: Vec::new(),
            activation_scores: Vec::new(),
            shift_event: TopicShiftEvent {
                kind: TopicShiftKind::Created,
                from_topic_id: None,
                to_topic_id: None,
                reason: Some("bootstrap".into()),
            },
            explanation: "bootstrap route".into(),
        };
        let patch = TopicRouteShellPatch::from_primary_topic(Some(topic_id.clone()))
            .with_shift_reason("patched shift reason")
            .with_explanation_replacement("replacement route")
            .with_explanation_suffix("patched by test");

        apply_topic_route_shell_patch(&mut route, &patch);

        assert_eq!(route.primary_topic_id, Some(topic_id.clone()));
        assert_eq!(route.shift_event.to_topic_id, Some(topic_id));
        assert_eq!(
            route.shift_event.reason.as_deref(),
            Some("patched shift reason")
        );
        assert!(route.explanation.starts_with("replacement route"));
        assert!(route.explanation.ends_with("; patched by test"));
    }

    #[tokio::test]
    async fn recent_session_window_and_query_transcript_follow_recorded_turns() {
        let runtime = RuntimeKernel::new();
        runtime
            .run_demo_turn("hello transcript layer")
            .await
            .expect("demo turn should succeed");

        let recent = runtime
            .recent_session_window("session-main", 4)
            .expect("recent window should load");
        assert!(recent.iter().any(|entry| {
            entry.role == Some(MessageRole::User) && entry.content == "hello transcript layer"
        }));
        assert!(
            recent
                .iter()
                .any(|entry| entry.role == Some(MessageRole::Assistant))
        );

        let report = runtime
            .query_transcript(Some("session-main"), "hello transcript layer", 10)
            .expect("transcript query should succeed");
        assert!(report.matched_count >= 1);
        assert!(report.hits.iter().any(|span| {
            span.entries
                .iter()
                .any(|entry| entry.content == "hello transcript layer")
        }));
    }

    #[tokio::test]
    async fn fresh_runtime_transcript_recall_and_activity_surfaces_are_empty_but_valid() {
        let runtime = RuntimeKernel::new();

        let recent = runtime
            .recent_session_window("session-main", 4)
            .expect("recent window should load for fresh runtime");
        assert!(recent.is_empty());

        let recall = runtime
            .context_recall_slice("session-main", Some("fresh recall"), 4, 4, 4, true)
            .expect("context recall slice should succeed for fresh runtime");
        assert_eq!(recall.recent_entry_count, 0);
        assert_eq!(recall.total_recent_entry_count, 0);
        assert_eq!(recall.transcript_matched_count, 0);
        assert_eq!(recall.transcript_returned_count, 0);
        assert_eq!(recall.memory_matched_count, 0);
        assert_eq!(recall.durable_memory_hit_count, 0);
        assert_eq!(recall.summary_hit_count, 0);
        assert_eq!(recall.memory_control_omitted_count, 0);
        assert!(recall.bundle.recent_entries.is_empty());
        assert!(recall.bundle.transcript_hits.is_empty());
        assert!(recall.bundle.durable_memory_hits.is_empty());
        assert!(recall.bundle.summary_hits.is_empty());

        let activity = runtime
            .activity_slice(None, 3, 3)
            .expect("activity slice should succeed for fresh runtime");
        assert!(activity.history.is_empty());
        assert!(!activity.events.is_empty());
    }

    #[tokio::test]
    async fn transcript_query_overview_rolls_up_returned_hits_by_session() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("shared transcript needle")
            .await
            .expect("alpha turn should succeed");
        runtime
            .run_demo_turn_in_session("beta", "shared transcript needle")
            .await
            .expect("beta turn should succeed");

        let overview = runtime
            .transcript_query_overview(None, "shared transcript needle", 10)
            .expect("transcript query overview should succeed");

        assert_eq!(overview.matched_sessions, 2);
        assert_eq!(overview.sessions.len(), 2);
        assert!(overview.returned_entries >= overview.report.returned_count);
        assert!(overview.sessions.iter().any(|session| {
            session.session_id == "alpha" && session.hit_count >= 1 && session.entry_count >= 1
        }));
        assert!(overview.sessions.iter().any(|session| {
            session.session_id == "beta" && session.hit_count >= 1 && session.entry_count >= 1
        }));
    }

    #[tokio::test]
    async fn recall_context_blends_recent_transcript_and_memory_hits() {
        let runtime = RuntimeKernel::new();
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("demo turn should succeed");

        let bundle = runtime
            .recall_context("session-main", Some("hello adaptive memory"), 6, 6, 6, true)
            .expect("context recall should succeed");

        assert!(!bundle.recent_entries.is_empty());
        assert!(!bundle.transcript_hits.is_empty());
        assert!(
            bundle
                .durable_memory_hits
                .iter()
                .any(|record| record.content.contains("hello adaptive memory"))
        );
        assert!(!bundle.is_empty());
    }

    #[tokio::test]
    async fn context_recall_slice_preserves_transcript_match_counts() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("turn should succeed");

        let recall = runtime
            .context_recall_slice("alpha", Some("hello adaptive memory"), 4, 4, 4, true)
            .expect("context recall slice should succeed");

        assert_eq!(recall.recent_entry_count, 2);
        assert_eq!(recall.bundle.recent_entries.len(), 2);
        assert_eq!(recall.transcript_matched_count, 2);
        assert_eq!(recall.transcript_returned_count, 2);
        assert_eq!(recall.bundle.transcript_hits.len(), 2);
        assert_eq!(recall.durable_memory_hit_count, 1);
        assert_eq!(recall.bundle.durable_memory_hits.len(), 1);
        assert_eq!(recall.summary_hit_count, 0);
        assert_eq!(recall.memory_control_omitted_count, 0);
        assert!(!recall.bundle.truncated);
        assert_eq!(recall.low_trust_ranked_item_count, 0);
        assert_eq!(recall.low_recency_ranked_item_count, 0);
    }

    #[tokio::test]
    async fn context_recall_slice_surfaces_memory_control_omission_pressure() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("control pressure")
            .await
            .expect("turn should succeed");
        runtime
            .memory
            .put(MemoryRecord {
                id: "control-tombstone".into(),
                scope: MemoryScope::LongTerm,
                content: "[hepta-memory:tombstone] control pressure retired memory".into(),
            })
            .await
            .expect("tombstone should store");
        runtime
            .memory
            .put(MemoryRecord {
                id: "control-conflict".into(),
                scope: MemoryScope::Session,
                content: "[hepta-memory:conflict] control pressure conflicting summary".into(),
            })
            .await
            .expect("conflict should store");

        let recall = runtime
            .context_recall_slice("alpha", Some("control pressure"), 4, 4, 4, true)
            .expect("context recall slice should succeed");
        let overview = runtime
            .provenance_overview("alpha")
            .expect("provenance overview should succeed");
        let serialized = serde_json::to_string(&overview).expect("overview should serialize");

        assert_eq!(recall.memory_control_omitted_count, 2);
        assert_eq!(overview.recall_memory_control_omitted_items, 2);
        assert!(recall.memory_matched_count >= 1);
        assert!(
            recall
                .bundle
                .durable_memory_hits
                .iter()
                .all(|record| !record.content.contains("[hepta-memory:"))
        );
        assert!(!serialized.contains("[hepta-memory:tombstone]"));
        assert!(!serialized.contains("[hepta-memory:conflict]"));
    }

    #[tokio::test]
    async fn context_recall_provider_rollup_maps_runtime_recall_to_payload_light_counts() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("provider rollup")
            .await
            .expect("turn should succeed");
        runtime
            .memory
            .put(MemoryRecord {
                id: "control-tombstone".into(),
                scope: MemoryScope::LongTerm,
                content: "[hepta-memory:tombstone] provider rollup retired memory".into(),
            })
            .await
            .expect("tombstone should store");
        runtime
            .memory
            .put(MemoryRecord {
                id: "control-conflict".into(),
                scope: MemoryScope::Session,
                content: "[hepta-memory:conflict] provider rollup conflicting summary".into(),
            })
            .await
            .expect("conflict should store");

        let recall = runtime
            .context_recall_slice("alpha", Some("provider rollup"), 4, 4, 4, true)
            .expect("context recall slice should succeed");
        let rollup = runtime
            .context_recall_provider_rollup("alpha", Some("provider rollup"), 4, 4, 4, true)
            .expect("provider rollup should build");
        let summary = &rollup.recall_selection;
        let serialized = serde_json::to_string(&rollup).expect("rollup should serialize");

        assert!(summary.has_count_integrity());
        assert_eq!(
            summary.ranked_item_count,
            u32::try_from(recall.bundle.ranked_items.len()).unwrap()
        );
        assert_eq!(
            summary.omitted_by_budget_count,
            u32::try_from(recall.bundle.omitted_by_budget).unwrap()
        );
        assert_eq!(summary.memory_control_omitted_count, 2);
        assert_eq!(
            summary.low_trust_ranked_item_count,
            u32::try_from(recall.low_trust_ranked_item_count).unwrap()
        );
        assert_eq!(
            summary.low_recency_ranked_item_count,
            u32::try_from(recall.low_recency_ranked_item_count).unwrap()
        );
        assert!(summary.source_diversity_met);
        assert!(!serialized.contains("source_id"));
        assert!(!serialized.contains("summary"));
        assert!(!serialized.contains("[hepta-memory:"));
        assert!(!serialized.contains("provider rollup"));
    }

    #[tokio::test]
    async fn context_recall_selected_snippet_envelope_redacts_and_bounds_shadow_snippets() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        let long_memory = format!("needle {}", "safe-context ".repeat(80));
        runtime
            .memory
            .put(MemoryRecord {
                id: "memory-long-source-id".into(),
                scope: MemoryScope::LongTerm,
                content: long_memory,
            })
            .await
            .expect("memory should store");
        runtime
            .memory
            .put(MemoryRecord {
                id: "control-tombstone".into(),
                scope: MemoryScope::LongTerm,
                content: "[hepta-memory:tombstone] needle retired memory".into(),
            })
            .await
            .expect("tombstone should store");
        runtime
            .memory
            .put(MemoryRecord {
                id: "control-conflict".into(),
                scope: MemoryScope::Session,
                content: "[hepta-memory:conflict] needle conflicting summary".into(),
            })
            .await
            .expect("conflict should store");

        let envelope = runtime
            .context_recall_selected_snippet_envelope(
                "alpha",
                Some("needle"),
                /*recent_window_limit*/ 4,
                /*transcript_limit*/ 4,
                /*memory_limit*/ 4,
                /*allow_cross_session*/ true,
            )
            .expect("snippet envelope should build");
        let serialized = serde_json::to_string(&envelope).expect("envelope should serialize");
        let protocol_envelope: CoreTurnContextRecallSelectedSnippetEnvelope =
            serde_json::from_str(&serialized)
                .expect("runtime envelope should match protocol shape");
        let mapped_core_envelope = envelope
            .clone()
            .into_core_envelope()
            .expect("runtime envelope should map to core protocol envelope");
        let opted_in_core_envelope =
            RuntimeContextRecallSelectedSnippetEnvelope::into_core_envelope_for_experimental_client(
                Some(envelope.clone()),
                true,
            )
            .expect("opted-in runtime envelope should map to core protocol envelope");

        assert_eq!(envelope.version, 1);
        assert_eq!(envelope.max_snippets, 4);
        assert_eq!(envelope.max_snippet_chars, 120);
        assert_eq!(envelope.selected_snippet_count, envelope.snippets.len());
        assert!(protocol_envelope.has_shadow_integrity());
        assert_eq!(mapped_core_envelope, protocol_envelope);
        assert_eq!(opted_in_core_envelope, protocol_envelope);
        assert!(
            RuntimeContextRecallSelectedSnippetEnvelope::into_core_envelope_for_experimental_client(
                Some(envelope.clone()),
                false,
            )
            .is_none()
        );
        let mut invalid_envelope = envelope.clone();
        invalid_envelope.selected_snippet_count += 1;
        assert!(invalid_envelope.into_core_envelope().is_none());
        assert_eq!(
            protocol_envelope.selected_snippet_count,
            u32::try_from(envelope.selected_snippet_count).unwrap()
        );
        assert_eq!(
            envelope.safety,
            RuntimeContextRecallSelectedSnippetSafety {
                ready_for_shadow_handoff: true,
                bounded: true,
                origin_identifiers_exposed: false,
                raw_ranked_payload_exposed: false,
                rank_explanation_exposed: false,
                control_marker_exposed: false,
                query_payload_exposed: false,
                per_origin_list_exposed: false,
            }
        );
        assert!(envelope.selected_snippet_count > 0);
        assert!(envelope.selected_snippet_count <= envelope.max_snippets);
        assert!(envelope.redacted_snippet_count > 0);
        assert!(envelope.truncated_snippet_count > 0);
        assert!(envelope.snippets.iter().all(|snippet| {
            snippet.text.chars().count() <= envelope.max_snippet_chars
                && snippet.snippet_hash.len() == 16
                && !snippet.text.contains("needle")
                && !snippet.text.contains("[hepta-memory:")
        }));
        assert!(
            envelope
                .snippets
                .iter()
                .any(|snippet| snippet.text.contains("[redacted-query]"))
        );
        assert!(!serialized.contains("needle"));
        assert!(!serialized.contains("memory-long-source-id"));
        assert!(!serialized.contains("control-tombstone"));
        assert!(!serialized.contains("control-conflict"));
        assert!(!serialized.contains("[hepta-memory:"));
        assert!(!serialized.contains("source_id"));
        assert!(!serialized.contains("source_memory_ids"));
        assert!(!serialized.contains("summary"));
        assert!(!serialized.contains("reason"));
    }

    #[tokio::test]
    async fn context_recall_turn_handoff_packages_rollup_and_opted_in_core_snippets() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        let long_memory = format!("needle {}", "safe-context ".repeat(80));
        runtime
            .memory
            .put(MemoryRecord {
                id: "runtime-handoff-source-id".into(),
                scope: MemoryScope::LongTerm,
                content: long_memory,
            })
            .await
            .expect("memory should store");
        runtime
            .memory
            .put(MemoryRecord {
                id: "runtime-handoff-control".into(),
                scope: MemoryScope::LongTerm,
                content: "[hepta-memory:tombstone] needle retired memory".into(),
            })
            .await
            .expect("control memory should store");

        let handoff = runtime
            .context_recall_turn_handoff(
                "alpha",
                Some("needle"),
                /*recent_window_limit*/ 4,
                /*transcript_limit*/ 4,
                /*memory_limit*/ 4,
                /*allow_cross_session*/ true,
                /*experimental_api_enabled*/ true,
            )
            .expect("turn handoff should build");
        let selected_snippets = handoff
            .selected_snippets
            .as_ref()
            .expect("opted-in handoff should include selected snippets");
        let direct_selected_snippets = runtime
            .context_recall_selected_snippet_envelope(
                "alpha",
                Some("needle"),
                /*recent_window_limit*/ 4,
                /*transcript_limit*/ 4,
                /*memory_limit*/ 4,
                /*allow_cross_session*/ true,
            )
            .expect("direct selected snippets should build")
            .into_core_envelope()
            .expect("direct selected snippets should map to core envelope");
        let debug = format!("{handoff:?}");

        assert!(
            handoff
                .provider_rollup
                .recall_selection
                .has_count_integrity()
        );
        assert!(selected_snippets.has_shadow_integrity());
        assert_eq!(selected_snippets, &direct_selected_snippets);
        assert!(selected_snippets.selected_snippet_count > 0);
        assert!(!debug.contains("needle"));
        assert!(!debug.contains("runtime-handoff-source-id"));
        assert!(!debug.contains("[hepta-memory:"));
        assert!(!debug.contains("source_id"));
        assert!(!debug.contains("summary"));
        assert!(!debug.contains("reason"));

        let no_opt_in_handoff = runtime
            .context_recall_turn_handoff(
                "alpha",
                Some("needle"),
                /*recent_window_limit*/ 4,
                /*transcript_limit*/ 4,
                /*memory_limit*/ 4,
                /*allow_cross_session*/ true,
                /*experimental_api_enabled*/ false,
            )
            .expect("no-opt-in turn handoff should build");

        assert!(
            no_opt_in_handoff
                .provider_rollup
                .recall_selection
                .has_count_integrity()
        );
        assert!(no_opt_in_handoff.selected_snippets.is_none());
    }

    #[tokio::test]
    async fn activate_neurons_bootstraps_direct_activation_from_recall_evidence() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .rename_active_session("Alpha workspace")
            .expect("rename should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("turn should succeed");

        let activations = runtime
            .activate_neurons("alpha", Some("hello adaptive memory"), 4, 4, 4, 2)
            .expect("activate neurons should succeed");

        assert_eq!(activations.len(), 1);
        let activation = &activations[0];
        assert_eq!(activation.neuron_id.0, "neuron-alpha");
        assert_eq!(activation.topic_id.0, "topic-alpha");
        assert!(activation.direct_score > 0.0);
        assert_eq!(activation.propagated_score, 0.0);
        assert_eq!(activation.inhibition_score, 0.0);
        assert_eq!(activation.final_score, activation.direct_score);
        assert_eq!(
            activation.source_topic_session_ids,
            vec!["topic-session-bootstrap:alpha"]
        );
        assert!(activation.source_neuron_ids.is_empty());
        assert!(
            activation
                .reason
                .as_deref()
                .unwrap_or_default()
                .contains("via routed topic session")
        );
        assert!(
            activation
                .reason
                .as_deref()
                .unwrap_or_default()
                .contains("compressed neuron")
        );
        assert!(
            activation
                .reason
                .as_deref()
                .unwrap_or_default()
                .contains("no additional propagated activation fired yet")
        );
    }

    #[tokio::test]
    async fn neuron_activation_overview_respects_zero_limit() {
        let runtime = RuntimeKernel::new();
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("turn should succeed");

        let overview = runtime
            .neuron_activation_overview("session-main", Some("hello adaptive memory"), 4, 4, 4, 0)
            .expect("neuron activation overview should succeed");

        assert_eq!(overview.recent_entry_count, 2);
        assert_eq!(overview.transcript_matched_count, 2);
        assert_eq!(overview.durable_memory_hit_count, 1);
        assert_eq!(overview.active_topic_session_count, 0);
        assert_eq!(overview.routed_topic_count, 0);
        assert!(overview.activations.is_empty());
    }

    #[tokio::test]
    async fn neuron_activation_overview_uses_topic_routing_state() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("turn should succeed");

        let overview = runtime
            .neuron_activation_overview("alpha", Some("hello adaptive memory"), 4, 4, 4, 2)
            .expect("neuron activation overview should succeed");

        assert_eq!(overview.active_topic_session_count, 1);
        assert_eq!(overview.routed_topic_count, 1);
        assert_eq!(overview.activations.len(), 1);
        assert_eq!(
            overview.activations[0].source_topic_session_ids,
            vec!["topic-session-bootstrap:alpha"]
        );
        assert!(!overview.activations[0].source_transcript_spans.is_empty());
        assert!(
            overview.activations[0]
                .source_transcript_spans
                .iter()
                .any(|span| {
                    span.session_id.0 == "alpha"
                        && span
                            .reason
                            .as_deref()
                            .is_some_and(|reason| reason.contains("query_match"))
                })
        );
    }

    #[tokio::test]
    async fn intuition_overview_returns_provenance_aware_bundle() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("turn should succeed");

        let overview = runtime
            .intuition_overview("alpha", "hello adaptive memory", 4, 4, 4, 2, 2, 2)
            .expect("intuition overview should succeed");

        assert_eq!(overview.session_id, "alpha");
        assert_eq!(overview.user_intent, "hello adaptive memory");
        assert_eq!(overview.active_topic_session_count, 1);
        assert_eq!(overview.routed_topic_count, 1);
        assert_eq!(overview.returned_neuron_activation_count, 1);
        assert_eq!(overview.bundle.request.surface_session_id.0, "alpha");
        assert_eq!(overview.bundle.request.user_intent, "hello adaptive memory");
        assert_eq!(overview.bundle.topic_activation_scores.len(), 1);
        assert_eq!(overview.bundle.neuron_activations.len(), 1);
        assert_eq!(
            overview.bundle.foreground_topic_session_ids,
            vec!["topic-session-bootstrap:alpha"]
        );
        assert!(!overview.bundle.source_transcript_spans.is_empty());
        assert!(overview.bundle.source_transcript_spans.iter().any(|span| {
            span.session_id.0 == "alpha"
                && span
                    .reason
                    .as_deref()
                    .is_some_and(|reason| reason.contains("query_match"))
        }));
        assert_eq!(overview.bundle.workflow_priors.len(), 1);
        assert_eq!(
            overview.bundle.workflow_priors[0].workflow_id,
            "workflow:memory-review"
        );
        assert!(overview.bundle.workflow_priors[0].exists_in_registry);
        assert_eq!(overview.bundle.workflow_priors[0].missing_capability, None);
        assert!(!overview.bundle.workflow_priors[0].requires_confirmation);
        assert_eq!(
            overview.bundle.workflow_priors[0].action_mode,
            IntuitionActionMode::Prepare
        );
        assert!(
            overview.bundle.workflow_priors[0]
                .reason
                .as_deref()
                .unwrap_or_default()
                .contains("workflow registry ranked")
        );
        assert_eq!(overview.bundle.skill_decisions.len(), 1);
        assert_eq!(
            overview.bundle.skill_decisions[0].skill_id,
            "skill-bootstrap:topic-alpha:followup"
        );
        assert!(
            overview.bundle.skill_decisions[0]
                .reason
                .as_deref()
                .unwrap_or_default()
                .contains("compressed neuron prior")
        );
        assert_eq!(
            overview.bundle.skill_decisions[0].source_topic_ids,
            vec![TopicId("topic-alpha".into())]
        );
        assert!(
            overview
                .bundle
                .explanation
                .as_deref()
                .unwrap_or_default()
                .contains("bootstrap intuition synthesized")
        );
        assert!(!overview.bundle.truncated);
    }

    #[tokio::test]
    async fn intuition_overview_reuses_single_routing_state_for_neuron_activation() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("first turn should succeed");
        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("first route should succeed");
        runtime
            .run_demo_turn("rust worker pipeline")
            .await
            .expect("second turn should succeed");
        runtime
            .route_topics("alpha", Some("rust worker pipeline"), 6, 6, 6, 1)
            .expect("second route should succeed");

        let overview = runtime
            .intuition_overview(
                "alpha",
                "hello adaptive memory and rust worker pipeline",
                8,
                8,
                8,
                2,
                1,
                2,
            )
            .expect("intuition overview should succeed");

        assert_eq!(overview.routed_topic_count, 2);
        assert_eq!(overview.bundle.foreground_topic_session_ids.len(), 2);
        assert_eq!(overview.bundle.neuron_activations.len(), 1);

        let topic_sessions = runtime
            .topic_sessions_for_surface("alpha")
            .expect("topic session overview should succeed");
        let active_topic_session_count = topic_sessions
            .iter()
            .filter(|topic_session| matches!(topic_session.status, TopicSessionStatus::Active))
            .count();
        assert_eq!(active_topic_session_count, 2);
    }

    #[tokio::test]
    async fn intuition_overview_uses_durable_neuron_store_and_feedback_calibration() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("turn should succeed");
        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("topic route should succeed");

        let (mut neuron, _) = runtime
            .compress_topic_to_neuron("alpha", "topic-alpha")
            .expect("topic compression should persist neuron");
        neuron.skill_priors[0].skill_id = "skill-custom:memory-review".into();
        neuron.workflow_priors[0].workflow_id = "workflow-custom:memory-review".into();
        runtime
            .upsert_neurons_for_session("alpha", vec![neuron.clone()])
            .expect("custom neuron prior should upsert");

        let stored = runtime
            .stored_neurons_for_session("alpha")
            .expect("stored neurons should be readable");
        assert_eq!(stored.len(), 1);
        assert_eq!(stored[0].neuron_id, neuron.neuron_id);

        let before = runtime
            .intuition_overview("alpha", "hello adaptive memory", 4, 4, 4, 1, 1, 1)
            .expect("intuition overview should succeed");
        assert_eq!(
            before.bundle.workflow_priors[0].workflow_id,
            "workflow-custom:memory-review"
        );
        assert!(
            before.bundle.workflow_priors[0]
                .reason
                .as_deref()
                .unwrap_or_default()
                .contains(MEMORY_NEURON_COMPRESSION_V2_POLICY)
        );
        assert_eq!(
            before.bundle.skill_decisions[0].skill_id,
            "skill-custom:memory-review"
        );
        assert!(
            before.bundle.skill_decisions[0]
                .reason
                .as_deref()
                .unwrap_or_default()
                .contains(MEMORY_NEURON_COMPRESSION_V2_POLICY)
        );
        assert_eq!(
            before.bundle.skill_decisions[0].workflow_id.as_deref(),
            Some("workflow-custom:memory-review")
        );
        let before_score = before.bundle.skill_decisions[0].score;

        runtime
            .record_intuition_feedback(
                "alpha",
                "hello adaptive memory",
                IntuitionFeedbackOutcome::Accepted,
                Some("skill-custom:memory-review"),
                Some("workflow-custom:memory-review"),
                before.bundle.skill_decisions[0].source_topic_ids.clone(),
                before.bundle.skill_decisions[0].source_neuron_ids.clone(),
                Some("user accepted custom intuition lane"),
            )
            .expect("feedback should record");

        let after = runtime
            .intuition_overview("alpha", "hello adaptive memory", 4, 4, 4, 1, 1, 1)
            .expect("intuition overview should succeed after feedback");
        assert_eq!(
            after.router_id,
            hepta_intelligence::SEMANTIC_ROUTER_LEARNED_ID
        );
        assert!(after.learned_router_signal_count > 0);
        assert!(after.bundle.skill_decisions[0].score > before_score);
        assert!(
            after.bundle.skill_decisions[0]
                .reason
                .as_deref()
                .unwrap_or_default()
                .contains("feedback +")
        );
    }

    #[tokio::test]
    async fn intuition_calibration_overview_groups_feedback_by_skill_and_workflow() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("turn should succeed");
        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("topic route should succeed");

        let before = runtime
            .intuition_overview("alpha", "hello adaptive memory", 4, 4, 4, 1, 1, 1)
            .expect("intuition overview should succeed");
        let skill = before.bundle.skill_decisions[0].clone();
        let workflow = before.bundle.workflow_priors[0].clone();

        runtime
            .record_intuition_feedback(
                "alpha",
                "hello adaptive memory",
                IntuitionFeedbackOutcome::ExecutedSuccess,
                Some(skill.skill_id.as_str()),
                Some(workflow.workflow_id.as_str()),
                skill.source_topic_ids.clone(),
                skill.source_neuron_ids.clone(),
                Some("execution succeeded"),
            )
            .expect("positive feedback should record");
        runtime
            .record_intuition_feedback(
                "alpha",
                "hello adaptive memory",
                IntuitionFeedbackOutcome::ToolFailed,
                Some(skill.skill_id.as_str()),
                Some(workflow.workflow_id.as_str()),
                skill.source_topic_ids.clone(),
                skill.source_neuron_ids.clone(),
                Some("tool failed once"),
            )
            .expect("negative feedback should record");

        let overview = runtime
            .intuition_calibration_overview("alpha")
            .expect("calibration overview should succeed");

        assert_eq!(overview.session_id, "alpha");
        assert_eq!(overview.feedback_record_count, 2);
        assert!(overview.closed_loop_ready);
        assert!(overview.learner_applied_update_count >= 2);
        assert!(overview.learned_topic_hint_count > 0);
        assert!(overview.learned_neuron_update_count > 0);
        assert!(overview.learning_findings.is_empty());
        assert_eq!(overview.positive_feedback_count, 1);
        assert_eq!(overview.negative_feedback_count, 1);
        assert_eq!(overview.neutral_feedback_count, 0);
        assert_eq!(
            overview.outcome_counts.get("executed_success").copied(),
            Some(1)
        );
        assert_eq!(overview.outcome_counts.get("tool_failed").copied(), Some(1));
        assert_eq!(overview.skill_targets.len(), 1);
        assert_eq!(overview.workflow_targets.len(), 1);
        assert_eq!(overview.skill_targets[0].target_id, skill.skill_id);
        assert_eq!(overview.workflow_targets[0].target_id, workflow.workflow_id);
        assert_eq!(overview.skill_targets[0].feedback_count, 2);
        assert_eq!(
            overview.skill_targets[0].source_topic_ids,
            vec!["topic-alpha"]
        );
        assert_eq!(
            overview.skill_targets[0].source_neuron_ids,
            vec!["neuron-alpha"]
        );
        assert!(overview.skill_targets[0].net_weight_delta > 0.0);
        assert!(overview.skill_targets[0].confidence_shift_count > 0);
        assert_eq!(overview.recent_feedback.len(), 2);
    }

    #[tokio::test]
    async fn neuron_lookup_revalidates_stored_neuron_when_topic_evidence_changes() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("first turn should succeed");
        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("initial route should succeed");

        let (initial, _) = runtime
            .compress_topic_to_neuron("alpha", "topic-alpha")
            .expect("initial compression should persist neuron");
        assert_eq!(initial.neuron_revision, 1);
        let initial_digest = initial.source_evidence_digest.clone();

        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("second turn should succeed");
        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 6, 6, 6, 1)
            .expect("updated route should succeed");

        let overview = runtime
            .neuron_activation_overview("alpha", Some("hello adaptive memory"), 6, 6, 6, 1)
            .expect("activation should refresh stored neuron");
        assert_eq!(overview.activations.len(), 1);

        let stored = runtime
            .stored_neurons_for_session("alpha")
            .expect("stored neurons should be readable");
        assert_eq!(stored.len(), 1);
        assert_eq!(stored[0].neuron_id, initial.neuron_id);
        assert_eq!(stored[0].neuron_revision, 2);
        assert_eq!(
            stored[0].last_refresh_reason.as_deref(),
            Some("bootstrap_revalidated_topic_session_evidence")
        );
        assert_ne!(stored[0].source_evidence_digest, initial_digest);
        assert!(
            stored[0].important_transcript_spans.len() > initial.important_transcript_spans.len()
        );
    }

    #[tokio::test]
    async fn intuition_overview_binds_file_intent_to_runtime_tool_registry() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("please read file architecture notes")
            .await
            .expect("turn should succeed");

        let overview = runtime
            .intuition_overview("alpha", "read file architecture notes", 4, 4, 4, 1, 1, 1)
            .expect("intuition overview should succeed");

        let decision = overview
            .bundle
            .skill_decisions
            .first()
            .expect("registered skill decision should be returned");
        assert_eq!(decision.skill_id, "read_file");
        assert!(decision.exists_in_registry);
        assert_eq!(decision.missing_capability, None);
        assert_eq!(decision.risk_tier, Some(RiskTier::Medium));
        assert!(decision.requires_confirmation);
        assert_eq!(decision.action_mode, IntuitionActionMode::SuggestOnly);
        assert!(
            decision
                .reason
                .as_deref()
                .unwrap_or_default()
                .contains("bound to runtime tool registry entry 'read_file'")
        );
    }

    #[tokio::test]
    async fn intuition_overview_applies_custom_policy_to_registered_skill_ranking() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .add_policy_rule(
                Some("alpha"),
                None,
                Some("read_file"),
                None,
                ApprovalRequirement::None,
                Some("alpha session may preflight read_file suggestions"),
            )
            .expect("policy rule should be accepted");
        runtime
            .run_demo_turn("please read file architecture notes")
            .await
            .expect("turn should succeed");

        let overview = runtime
            .intuition_overview("alpha", "read file architecture notes", 4, 4, 4, 1, 1, 1)
            .expect("intuition overview should succeed");

        let decision = overview
            .bundle
            .skill_decisions
            .first()
            .expect("registered skill decision should be returned");
        assert_eq!(decision.skill_id, "read_file");
        assert!(decision.exists_in_registry);
        assert!(!decision.requires_confirmation);
        assert_eq!(decision.action_mode, IntuitionActionMode::Prepare);
        let reason = decision.reason.as_deref().unwrap_or_default();
        assert!(reason.contains("policy-aware intuition ranked"));
        assert!(reason.contains("approval=none"));
        assert!(reason.contains("alpha session may preflight read_file suggestions"));
    }

    #[tokio::test]
    async fn intuition_overview_keeps_denied_write_skill_suggest_only() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("create file release notes")
            .await
            .expect("turn should succeed");

        let overview = runtime
            .intuition_overview("alpha", "create file release notes", 4, 4, 4, 1, 1, 1)
            .expect("intuition overview should succeed");

        let decision = overview
            .bundle
            .skill_decisions
            .first()
            .expect("registered skill decision should be returned");
        assert_eq!(decision.skill_id, "write_file");
        assert!(decision.exists_in_registry);
        assert_eq!(decision.risk_tier, Some(RiskTier::High));
        assert!(decision.requires_confirmation);
        assert_eq!(decision.action_mode, IntuitionActionMode::SuggestOnly);
        let reason = decision.reason.as_deref().unwrap_or_default();
        assert!(reason.contains("approval=deny"));
        assert!(reason.contains("denied by default"));
    }

    #[tokio::test]
    async fn intuition_overview_binds_workflow_priors_to_runtime_registry() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("rust worker pipeline needs semantic routing")
            .await
            .expect("turn should succeed");

        let overview = runtime
            .intuition_overview(
                "alpha",
                "rust worker pipeline needs semantic routing",
                4,
                4,
                4,
                1,
                1,
                1,
            )
            .expect("intuition overview should succeed");

        let prior = overview
            .bundle
            .workflow_priors
            .first()
            .expect("workflow prior should be returned");
        assert_eq!(prior.workflow_id, "workflow:engineering-change");
        assert!(prior.exists_in_registry);
        assert_eq!(prior.missing_capability, None);
        assert!(!prior.requires_confirmation);
        assert_eq!(prior.action_mode, IntuitionActionMode::Prepare);
        let reason = prior.reason.as_deref().unwrap_or_default();
        assert!(reason.contains("workflow registry ranked"));
        assert!(reason.contains("bound to workflow registry entry"));
        assert_eq!(
            overview.bundle.skill_decisions[0].workflow_id.as_deref(),
            Some("workflow:engineering-change")
        );
    }

    #[tokio::test]
    async fn intuition_overview_marks_mutating_workflow_prior_as_gated() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("create file release notes")
            .await
            .expect("turn should succeed");

        let overview = runtime
            .intuition_overview("alpha", "create file release notes", 4, 4, 4, 1, 1, 1)
            .expect("intuition overview should succeed");

        let prior = overview
            .bundle
            .workflow_priors
            .first()
            .expect("workflow prior should be returned");
        assert_eq!(prior.workflow_id, "workflow:file-change");
        assert!(prior.exists_in_registry);
        assert!(prior.requires_confirmation);
        assert_eq!(prior.action_mode, IntuitionActionMode::SuggestOnly);
        assert_eq!(overview.bundle.skill_decisions[0].skill_id, "write_file");
        assert_eq!(
            overview.bundle.skill_decisions[0].workflow_id.as_deref(),
            Some("workflow:file-change")
        );
    }

    #[tokio::test]
    async fn provenance_overview_summarizes_topic_recall_and_intuition_coverage() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("turn should succeed");
        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("topic route should succeed");

        let overview = runtime
            .provenance_overview("alpha")
            .expect("provenance overview should succeed");

        assert_eq!(overview.session_id, "alpha");
        assert_eq!(
            overview.last_user_intent_summary.as_deref(),
            Some("hello adaptive memory")
        );
        assert_eq!(overview.total_topic_sessions, 1);
        assert_eq!(overview.active_topic_sessions, 1);
        assert_eq!(overview.active_topic_sessions_with_transcript_provenance, 1);
        assert_eq!(
            overview.active_topic_sessions_missing_transcript_provenance,
            0
        );
        assert!(overview.recall_transcript_evidence_spans > 0);
        assert!(overview.recall_ranked_items > 0);
        assert_eq!(overview.recall_low_trust_ranked_items, 0);
        assert_eq!(overview.recall_low_recency_ranked_items, 0);
        assert_eq!(overview.recall_memory_control_omitted_items, 0);
        assert_eq!(overview.recall_omitted_items, 0);
        assert!(overview.intuition_transcript_evidence_spans > 0);
        assert_eq!(overview.intuition_foreground_topic_sessions, 1);
    }

    #[test]
    fn provenance_overview_rollup_surfaces_low_quality_recall_counts() {
        let overview = provenance_overview_rollup::build(
            provenance_overview_rollup::ProvenanceOverviewInputs {
                session_id: "alpha".into(),
                last_user_intent_summary: None,
                topic_sessions: Vec::new(),
                recall_ranked_items: 4,
                recall_low_trust_ranked_items: 1,
                recall_low_recency_ranked_items: 2,
                recall_memory_control_omitted_items: 6,
                recall_transcript_evidence_spans: 3,
                recall_omitted_items: 5,
                intuition_transcript_evidence_spans: 0,
                intuition_foreground_topic_sessions: 0,
            },
        );

        assert_eq!(overview.recall_ranked_items, 4);
        assert_eq!(overview.recall_low_trust_ranked_items, 1);
        assert_eq!(overview.recall_low_recency_ranked_items, 2);
        assert_eq!(overview.recall_memory_control_omitted_items, 6);
        assert_eq!(overview.recall_omitted_items, 5);
    }

    #[tokio::test]
    async fn intelligence_eval_overview_replays_recent_user_turns_through_core_loop() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("first turn should succeed");
        runtime
            .run_demo_turn("rust worker pipeline")
            .await
            .expect("second turn should succeed");

        let overview = runtime
            .intelligence_eval_overview("alpha", 2, 6, 6, 6, 2, 2, 2)
            .expect("eval overview should succeed");

        assert_eq!(overview.session_id, "alpha");
        assert_eq!(overview.evaluated_case_count, 2);
        assert_eq!(overview.failed_case_count, 0);
        assert_eq!(overview.passed_case_count, 2);
        assert!(overview.total_recall_ranked_items >= 2);
        assert!(overview.total_transcript_evidence_spans >= 2);
        assert_eq!(
            overview.total_active_neurons,
            overview
                .cases
                .iter()
                .map(|case| case.active_neuron_count)
                .sum::<usize>()
        );
        assert!(overview.total_routed_topics >= 2);
        assert!(overview.total_neuron_activations >= 2);
        assert!(overview.total_suggested_skills >= 2);
        assert!(overview.total_workflow_priors >= 2);
        assert!(overview.registered_workflow_prior_count >= 2);
        assert!(overview.prepared_workflow_prior_count >= 2);
        assert_eq!(overview.semantic_score, 100);
        assert_eq!(
            overview.total_semantic_expectations,
            overview.total_semantic_expectations_passed
        );
        assert!(overview.cases.iter().all(|case| case.passed));
        assert!(overview.cases.iter().all(|case| case.warnings.is_empty()));
        assert!(overview.cases.iter().all(|case| case.semantic_score == 100));
        assert!(
            overview
                .cases
                .iter()
                .all(|case| case.semantic_failures.is_empty())
        );
        assert!(
            overview
                .cases
                .iter()
                .all(|case| case.workflow_prior_count > 0)
        );
        assert!(
            overview
                .cases
                .iter()
                .all(|case| case.registered_workflow_prior_count > 0)
        );
        assert_eq!(overview.cases[0].query_text, "hello adaptive memory");
        assert_eq!(overview.cases[1].query_text, "rust worker pipeline");

        let forced_learned = runtime
            .intelligence_eval_overview_with_router(
                "alpha",
                1,
                6,
                6,
                6,
                2,
                2,
                2,
                Some(hepta_intelligence::SEMANTIC_ROUTER_LEARNED_ID),
            )
            .expect("forced router eval overview should succeed");
        assert_eq!(
            forced_learned.semantic_router_id,
            hepta_intelligence::SEMANTIC_ROUTER_LEARNED_ID
        );
        assert!(
            forced_learned
                .cases
                .iter()
                .all(|case| { case.router_id == hepta_intelligence::SEMANTIC_ROUTER_LEARNED_ID })
        );
    }

    #[tokio::test]
    async fn provenance_overview_materializes_fresh_active_session() {
        let runtime = RuntimeKernel::new();

        let overview = runtime
            .provenance_overview("session-main")
            .expect("provenance overview should succeed for fresh runtime");

        assert_eq!(overview.session_id, "session-main");
        assert_eq!(overview.last_user_intent_summary, None);
        assert_eq!(overview.total_topic_sessions, 0);
        assert_eq!(overview.active_topic_sessions, 0);
        assert_eq!(overview.active_topic_sessions_with_transcript_provenance, 0);
        assert_eq!(
            overview.active_topic_sessions_missing_transcript_provenance,
            0
        );
        assert_eq!(overview.recall_transcript_evidence_spans, 0);
        assert_eq!(overview.recall_ranked_items, 0);
        assert_eq!(overview.recall_low_trust_ranked_items, 0);
        assert_eq!(overview.recall_low_recency_ranked_items, 0);
        assert_eq!(overview.recall_memory_control_omitted_items, 0);
        assert_eq!(overview.recall_omitted_items, 0);
        assert_eq!(overview.intuition_transcript_evidence_spans, 0);
        assert_eq!(overview.intuition_foreground_topic_sessions, 0);
    }

    #[tokio::test]
    async fn route_topics_bootstraps_primary_topic_from_session_evidence() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .rename_active_session("Alpha workspace")
            .expect("rename should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("turn should succeed");

        let decision = runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 2)
            .expect("route topics should succeed");

        assert_eq!(decision.surface_session_id.0, "alpha");
        assert_eq!(
            decision
                .primary_topic_id
                .expect("primary topic should exist")
                .0,
            "topic-alpha"
        );
        assert_eq!(
            decision.active_topic_session_ids,
            vec!["topic-session-bootstrap:alpha"]
        );
        assert!(!decision.source_transcript_spans.is_empty());
        assert!(decision.source_transcript_spans.iter().any(|span| {
            span.session_id.0 == "alpha"
                && span
                    .reason
                    .as_deref()
                    .is_some_and(|reason| reason.contains("query_match"))
        }));
        assert!(decision.created_topic_session_ids.is_empty());
        assert!(decision.revived_topic_session_ids.is_empty());
        assert_eq!(decision.activation_scores.len(), 1);
        assert_eq!(
            decision.activation_scores[0].topic_label.0,
            "hello adaptive memory"
        );
        assert!(
            decision.activation_scores[0]
                .matched_terms
                .iter()
                .any(|term| term == "hello")
        );
        assert!(matches!(
            decision.shift_event.expect("shift event should exist").kind,
            TopicShiftKind::Stayed
        ));
        assert!(
            decision
                .explanation
                .as_deref()
                .unwrap_or_default()
                .contains("bootstrap topic routing")
        );
    }

    #[tokio::test]
    async fn topic_routing_overview_respects_zero_limit() {
        let runtime = RuntimeKernel::new();
        runtime
            .run_demo_turn("fresh topic route")
            .await
            .expect("turn should succeed");

        let overview = runtime
            .topic_routing_overview("session-main", Some("fresh topic route"), 2, 2, 2, 0)
            .expect("topic routing overview should succeed");

        assert_eq!(overview.session_id, "session-main");
        assert!(overview.decision.primary_topic_id.is_none());
        assert!(!overview.decision.source_transcript_spans.is_empty());
        assert!(overview.decision.active_topic_session_ids.is_empty());
        assert!(overview.decision.activation_scores.is_empty());
        assert!(
            overview
                .decision
                .explanation
                .as_deref()
                .unwrap_or_default()
                .contains("bootstrap topic routing")
        );
    }

    #[tokio::test]
    async fn topic_routing_overview_materializes_fresh_active_session() {
        let runtime = RuntimeKernel::new();

        let overview = runtime
            .topic_routing_overview("session-main", Some("fresh topic route"), 2, 2, 2, 1)
            .expect("topic routing overview should succeed for fresh runtime");

        assert_eq!(overview.session_id, "session-main");
        assert_eq!(overview.query_text.as_deref(), Some("fresh topic route"));
        assert_eq!(overview.recent_entry_count, 0);
        assert_eq!(overview.transcript_matched_count, 0);
        assert_eq!(overview.durable_memory_hit_count, 0);
        assert_eq!(overview.summary_hit_count, 0);
        assert_eq!(overview.decision.active_topic_session_ids.len(), 1);
        assert_eq!(overview.decision.activation_scores.len(), 1);
        assert_eq!(
            overview.decision.primary_topic_id,
            Some(TopicId("topic-session-main".into()))
        );
        assert!(
            overview
                .decision
                .explanation
                .as_deref()
                .unwrap_or_default()
                .contains("bootstrap topic routing")
        );
    }

    #[tokio::test]
    async fn route_topics_persists_bootstrap_topic_session_state() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("turn should succeed");

        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("route topics should succeed");
        let topic_sessions = runtime
            .topic_sessions_for_surface("alpha")
            .expect("topic sessions should load");

        assert_eq!(topic_sessions.len(), 1);
        assert_eq!(
            topic_sessions[0].topic_session_id,
            "topic-session-bootstrap:alpha"
        );
        assert_eq!(topic_sessions[0].topic_id.0, "topic-alpha");
        assert_eq!(topic_sessions[0].topic_label.0, "hello adaptive memory");
        assert_eq!(topic_sessions[0].linked_surface_session_ids[0].0, "alpha");
        assert_eq!(topic_sessions[0].durable_memory_refs.len(), 1);
        assert_eq!(topic_sessions[0].open_loops.len(), 1);
        assert!(!topic_sessions[0].linked_transcript_spans.is_empty());
        assert!(
            topic_sessions[0]
                .linked_transcript_spans
                .iter()
                .any(|span| {
                    span.session_id.0 == "alpha"
                        && span
                            .reason
                            .as_deref()
                            .is_some_and(|reason| reason.contains("recent_window"))
                })
        );
        assert!(
            topic_sessions[0]
                .linked_transcript_spans
                .iter()
                .any(|span| {
                    span.session_id.0 == "alpha"
                        && span
                            .reason
                            .as_deref()
                            .is_some_and(|reason| reason.contains("query_match"))
                })
        );
    }

    #[tokio::test]
    async fn route_topics_updates_existing_bootstrap_topic_session_instead_of_duplicating() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("turn should succeed");

        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("first route should succeed");
        let first_topic_sessions = runtime
            .topic_sessions_for_surface("alpha")
            .expect("topic sessions should load after first route");
        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("second route should succeed");

        let topic_sessions = runtime
            .topic_sessions_for_surface("alpha")
            .expect("topic sessions should load");

        assert_eq!(topic_sessions.len(), 1);
        assert_eq!(
            topic_sessions[0].topic_session_id,
            "topic-session-bootstrap:alpha"
        );
        assert_eq!(
            topic_sessions[0].linked_transcript_spans,
            first_topic_sessions[0].linked_transcript_spans
        );
    }

    #[tokio::test]
    async fn route_topics_creates_new_topic_session_when_query_shifts() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("first turn should succeed");
        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("first route should succeed");
        runtime
            .run_demo_turn("rust worker pipeline")
            .await
            .expect("second turn should succeed");

        let decision = runtime
            .route_topics("alpha", Some("rust worker pipeline"), 6, 6, 6, 1)
            .expect("shift route should succeed");
        let topic_sessions = runtime
            .topic_sessions_for_surface("alpha")
            .expect("topic sessions should load");

        assert_eq!(
            decision.active_topic_session_ids,
            vec!["topic-session-bootstrap:alpha:rust-worker-pipeline"]
        );
        assert_eq!(
            decision.created_topic_session_ids,
            vec!["topic-session-bootstrap:alpha:rust-worker-pipeline"]
        );
        assert!(decision.revived_topic_session_ids.is_empty());
        assert!(matches!(
            decision.shift_event.expect("shift event should exist").kind,
            TopicShiftKind::Shifted
        ));
        assert_eq!(topic_sessions.len(), 2);
        assert!(topic_sessions.iter().any(|topic_session| {
            topic_session.topic_session_id == "topic-session-bootstrap:alpha:rust-worker-pipeline"
                && matches!(topic_session.status, TopicSessionStatus::Active)
                && topic_session.topic_id.0 == "topic-alpha-rust-worker-pipeline"
        }));
        assert!(topic_sessions.iter().any(|topic_session| {
            topic_session.topic_session_id == "topic-session-bootstrap:alpha"
                && matches!(topic_session.status, TopicSessionStatus::Dormant)
        }));
    }

    #[tokio::test]
    async fn route_topics_revives_matching_dormant_topic_session() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("first turn should succeed");
        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("first route should succeed");
        runtime
            .run_demo_turn("rust worker pipeline")
            .await
            .expect("second turn should succeed");
        runtime
            .route_topics("alpha", Some("rust worker pipeline"), 6, 6, 6, 1)
            .expect("shift route should succeed");

        let decision = runtime
            .route_topics("alpha", Some("hello memory"), 6, 6, 6, 1)
            .expect("revive route should succeed");
        let topic_sessions = runtime
            .topic_sessions_for_surface("alpha")
            .expect("topic sessions should load");

        assert_eq!(
            decision.active_topic_session_ids,
            vec!["topic-session-bootstrap:alpha"]
        );
        assert!(decision.created_topic_session_ids.is_empty());
        assert_eq!(
            decision.revived_topic_session_ids,
            vec!["topic-session-bootstrap:alpha"]
        );
        assert!(matches!(
            decision.shift_event.expect("shift event should exist").kind,
            TopicShiftKind::Revived
        ));
        assert_eq!(topic_sessions.len(), 2);
        assert!(topic_sessions.iter().any(|topic_session| {
            topic_session.topic_session_id == "topic-session-bootstrap:alpha"
                && matches!(topic_session.status, TopicSessionStatus::Active)
        }));
        assert!(topic_sessions.iter().any(|topic_session| {
            topic_session.topic_session_id == "topic-session-bootstrap:alpha:rust-worker-pipeline"
                && matches!(topic_session.status, TopicSessionStatus::Dormant)
        }));
    }

    #[tokio::test]
    async fn route_topics_coactivates_multiple_existing_topic_sessions_for_mixed_query() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("first turn should succeed");
        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("first route should succeed");
        runtime
            .run_demo_turn("rust worker pipeline")
            .await
            .expect("second turn should succeed");
        runtime
            .route_topics("alpha", Some("rust worker pipeline"), 6, 6, 6, 1)
            .expect("second route should succeed");

        let decision = runtime
            .route_topics(
                "alpha",
                Some("hello adaptive memory and rust worker pipeline"),
                8,
                8,
                8,
                2,
            )
            .expect("mixed route should succeed");
        let topic_sessions = runtime
            .topic_sessions_for_surface("alpha")
            .expect("topic sessions should load");

        assert_eq!(
            decision.active_topic_session_ids,
            vec![
                "topic-session-bootstrap:alpha",
                "topic-session-bootstrap:alpha:rust-worker-pipeline"
            ]
        );
        assert!(decision.created_topic_session_ids.is_empty());
        assert_eq!(
            decision.revived_topic_session_ids,
            vec!["topic-session-bootstrap:alpha"]
        );
        assert_eq!(decision.activation_scores.len(), 2);
        assert!(decision.is_multi_topic());
        assert!(matches!(
            decision.shift_event.expect("shift event should exist").kind,
            TopicShiftKind::CoActivated
        ));
        assert!(topic_sessions.iter().any(|topic_session| {
            topic_session.topic_session_id == "topic-session-bootstrap:alpha"
                && matches!(topic_session.status, TopicSessionStatus::Active)
        }));
        assert!(topic_sessions.iter().any(|topic_session| {
            topic_session.topic_session_id == "topic-session-bootstrap:alpha:rust-worker-pipeline"
                && matches!(topic_session.status, TopicSessionStatus::Active)
        }));
    }

    #[tokio::test]
    async fn route_topics_detects_implicit_mixed_turn_without_explicit_delimiters() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("first turn should succeed");
        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("first route should succeed");
        runtime
            .run_demo_turn("rust worker pipeline")
            .await
            .expect("second turn should succeed");
        runtime
            .route_topics("alpha", Some("rust worker pipeline"), 6, 6, 6, 1)
            .expect("second route should succeed");

        let decision = runtime
            .route_topics(
                "alpha",
                Some("continue hello adaptive memory rust worker pipeline"),
                8,
                8,
                8,
                2,
            )
            .expect("implicit mixed route should succeed");

        assert_eq!(decision.active_topic_session_ids.len(), 2);
        assert!(
            decision
                .active_topic_session_ids
                .iter()
                .any(|id| id == "topic-session-bootstrap:alpha")
        );
        assert!(
            decision
                .active_topic_session_ids
                .iter()
                .any(|id| id == "topic-session-bootstrap:alpha:rust-worker-pipeline")
        );
        assert!(decision.created_topic_session_ids.is_empty());
        assert_eq!(
            decision.revived_topic_session_ids,
            vec!["topic-session-bootstrap:alpha"]
        );
        assert_eq!(decision.activation_scores.len(), 2);
        assert!(decision.is_multi_topic());
        assert!(matches!(
            decision.shift_event.expect("shift event should exist").kind,
            TopicShiftKind::CoActivated
        ));
        assert!(
            decision
                .activation_scores
                .iter()
                .any(|score| score.topic_id.0 == "topic-alpha")
        );
        assert!(
            decision
                .activation_scores
                .iter()
                .any(|score| score.topic_id.0 == "topic-alpha-rust-worker-pipeline")
        );
    }

    #[tokio::test]
    async fn route_topics_detects_semantic_mixed_turn_without_exact_label_overlap() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("first turn should succeed");
        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("first route should succeed");
        runtime
            .run_demo_turn("rust worker pipeline")
            .await
            .expect("second turn should succeed");
        runtime
            .route_topics("alpha", Some("rust worker pipeline"), 6, 6, 6, 1)
            .expect("second route should succeed");

        let decision = runtime
            .route_topics(
                "alpha",
                Some("continue adaptive recall while checking executor flow"),
                8,
                8,
                8,
                2,
            )
            .expect("semantic mixed route should succeed");

        assert_eq!(decision.active_topic_session_ids.len(), 2);
        assert!(
            decision
                .active_topic_session_ids
                .iter()
                .any(|id| id == "topic-session-bootstrap:alpha")
        );
        assert!(
            decision
                .active_topic_session_ids
                .iter()
                .any(|id| id == "topic-session-bootstrap:alpha:rust-worker-pipeline")
        );
        assert!(decision.created_topic_session_ids.is_empty());
        assert_eq!(
            decision.revived_topic_session_ids,
            vec!["topic-session-bootstrap:alpha"]
        );
        assert!(matches!(
            decision.shift_event.expect("shift event should exist").kind,
            TopicShiftKind::CoActivated
        ));
        assert!(decision.activation_scores.iter().any(|score| {
            score.topic_id.0 == "topic-alpha"
                && score
                    .reason
                    .as_deref()
                    .is_some_and(|reason| reason.contains("semantic"))
        }));
        assert!(decision.activation_scores.iter().any(|score| {
            score.topic_id.0 == "topic-alpha-rust-worker-pipeline"
                && score
                    .reason
                    .as_deref()
                    .is_some_and(|reason| reason.contains("semantic"))
        }));
    }

    #[tokio::test]
    async fn route_topics_learns_open_ended_semantic_aliases_from_matched_evidence() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("rust worker pipeline")
            .await
            .expect("first turn should succeed");

        let first_decision = runtime
            .route_topics("alpha", Some("rust worker pipeline"), 4, 4, 4, 1)
            .expect("first route should succeed");
        let worker_topic_session_id = first_decision.active_topic_session_ids[0].clone();
        let worker_topic_id = first_decision
            .primary_topic_id
            .expect("primary topic should exist");

        runtime
            .route_topics(
                "alpha",
                Some("rust worker pipeline queue backlog"),
                6,
                6,
                6,
                1,
            )
            .expect("alias-learning route should succeed");

        let learned_sessions = runtime
            .topic_sessions_for_surface("alpha")
            .expect("topic sessions should load after alias learning");
        let worker_topic = learned_sessions
            .iter()
            .find(|topic_session| topic_session.topic_session_id == worker_topic_session_id)
            .expect("worker topic session should exist");
        assert!(worker_topic.entities.values().any(|value| value == "queue"));
        assert!(
            worker_topic
                .entities
                .values()
                .any(|value| value == "backlog")
        );

        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("second turn should succeed");
        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 6, 6, 6, 1)
            .expect("shift route should succeed");

        let revived = runtime
            .route_topics("alpha", Some("queue backlog"), 6, 6, 6, 1)
            .expect("learned alias revive route should succeed");

        assert_eq!(revived.primary_topic_id, Some(worker_topic_id));
        assert_eq!(revived.created_topic_session_ids, Vec::<String>::new());
        assert_eq!(
            revived.revived_topic_session_ids,
            vec![worker_topic_session_id]
        );
        assert!(matches!(
            revived.shift_event.expect("shift event should exist").kind,
            TopicShiftKind::Revived
        ));
    }

    #[tokio::test]
    async fn route_topics_merges_multiple_topic_sessions_into_new_composite_topic() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("first turn should succeed");
        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("first route should succeed");
        runtime
            .run_demo_turn("rust worker pipeline")
            .await
            .expect("second turn should succeed");
        runtime
            .route_topics("alpha", Some("rust worker pipeline"), 6, 6, 6, 1)
            .expect("second route should succeed");

        let decision = runtime
            .route_topics(
                "alpha",
                Some("merge hello adaptive memory and rust worker pipeline"),
                8,
                8,
                8,
                2,
            )
            .expect("merge route should succeed");
        let topic_sessions = runtime
            .topic_sessions_for_surface("alpha")
            .expect("topic sessions should load");

        assert_eq!(
            decision
                .primary_topic_id
                .expect("primary topic should exist")
                .0,
            "topic-alpha-hello-adaptive-memory-rust-worker-pipeline"
        );
        assert_eq!(
            decision.active_topic_session_ids,
            vec!["topic-session-bootstrap:alpha:hello-adaptive-memory-rust-worker-pipeline"]
        );
        assert_eq!(
            decision.created_topic_session_ids,
            vec!["topic-session-bootstrap:alpha:hello-adaptive-memory-rust-worker-pipeline"]
        );
        assert!(decision.revived_topic_session_ids.is_empty());
        assert!(matches!(
            decision.shift_event.expect("shift event should exist").kind,
            TopicShiftKind::Merged
        ));
        assert!(topic_sessions.iter().any(|topic_session| {
            topic_session.topic_session_id
                == "topic-session-bootstrap:alpha:hello-adaptive-memory-rust-worker-pipeline"
                && matches!(topic_session.status, TopicSessionStatus::Active)
        }));
        assert!(topic_sessions.iter().any(|topic_session| {
            topic_session.topic_session_id == "topic-session-bootstrap:alpha"
                && matches!(topic_session.status, TopicSessionStatus::Merged)
        }));
        assert!(topic_sessions.iter().any(|topic_session| {
            topic_session.topic_session_id == "topic-session-bootstrap:alpha:rust-worker-pipeline"
                && matches!(topic_session.status, TopicSessionStatus::Merged)
        }));
    }

    #[tokio::test]
    async fn compress_topic_to_neuron_collects_provenance_and_component_links() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("first turn should succeed");
        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("first route should succeed");
        runtime
            .run_demo_turn("rust worker pipeline")
            .await
            .expect("second turn should succeed");
        runtime
            .route_topics("alpha", Some("rust worker pipeline"), 6, 6, 6, 1)
            .expect("second route should succeed");
        runtime
            .route_topics(
                "alpha",
                Some("merge hello adaptive memory and rust worker pipeline"),
                8,
                8,
                8,
                2,
            )
            .expect("merge route should succeed");

        let (neuron, report) = runtime
            .compress_topic_to_neuron(
                "alpha",
                "topic-alpha-hello-adaptive-memory-rust-worker-pipeline",
            )
            .expect("topic compression should succeed");

        assert_eq!(
            neuron.neuron_id.0,
            "neuron-alpha-hello-adaptive-memory-rust-worker-pipeline"
        );
        assert_eq!(report.created_neuron_id, Some(neuron.neuron_id.clone()));
        assert_eq!(
            neuron.compression_policy_version,
            MEMORY_NEURON_COMPRESSION_V2_POLICY
        );
        assert_eq!(
            report.compression_policy_version,
            MEMORY_NEURON_COMPRESSION_V2_POLICY
        );
        assert_eq!(report.source_evidence_digest, neuron.source_evidence_digest);
        assert_eq!(report.source_topic_session_ids.len(), 1);
        assert!(report.important_span_count >= 1);
        assert!(report.promoted_memory_count >= 1);
        assert!(
            report
                .merged_neuron_ids
                .iter()
                .any(|neuron_id| neuron_id.0 == "neuron-alpha")
        );
        assert!(
            report
                .merged_neuron_ids
                .iter()
                .any(|neuron_id| neuron_id.0 == "neuron-alpha-rust-worker-pipeline")
        );
        assert!(
            neuron
                .links
                .iter()
                .any(|link| link.target_neuron_id.0 == "neuron-alpha")
        );
        assert!(
            neuron
                .links
                .iter()
                .any(|link| { link.target_neuron_id.0 == "neuron-alpha-rust-worker-pipeline" })
        );
        assert_eq!(neuron.skill_priors.len(), 1);
        assert_eq!(neuron.workflow_priors.len(), 1);
        assert_eq!(report.skill_prior_count, 1);
        assert_eq!(report.workflow_prior_count, 1);
        assert!(report.typed_link_count >= 2);
        assert!(report.provenance_complete);
        assert!(report.intuition_ready);
        assert!(neuron.confidence > 0.0);
        assert!(neuron.freshness > 0.0);
    }

    #[tokio::test]
    async fn compress_active_topics_to_neurons_returns_unique_active_topics() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("first turn should succeed");
        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("first route should succeed");
        runtime
            .run_demo_turn("rust worker pipeline")
            .await
            .expect("second turn should succeed");
        runtime
            .route_topics("alpha", Some("rust worker pipeline"), 6, 6, 6, 1)
            .expect("second route should succeed");
        runtime
            .route_topics(
                "alpha",
                Some("hello adaptive memory and rust worker pipeline"),
                8,
                8,
                8,
                2,
            )
            .expect("mixed route should succeed");

        let neurons = runtime
            .compress_active_topics_to_neurons("alpha", 4)
            .expect("active neuron compression should succeed");

        assert_eq!(neurons.len(), 2);
        assert!(
            neurons
                .iter()
                .any(|neuron| neuron.neuron_id.0 == "neuron-alpha")
        );
        assert!(
            neurons
                .iter()
                .any(|neuron| neuron.neuron_id.0 == "neuron-alpha-rust-worker-pipeline")
        );
    }

    #[tokio::test]
    async fn neuron_lifecycle_overview_surfaces_stored_neuron_health() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("first turn should succeed");
        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("route should succeed");
        runtime
            .compress_active_topics_to_neurons("alpha", 4)
            .expect("compression should succeed");
        let mut neuron = runtime
            .stored_neurons_for_session("alpha")
            .expect("stored neuron should load")
            .pop()
            .expect("one neuron should exist");
        neuron.linked_session_ids.push(SessionId("beta".into()));
        runtime
            .upsert_neurons_for_session("alpha", vec![neuron])
            .expect("cross-session neuron should upsert");

        let overview = runtime
            .neuron_lifecycle_overview("alpha")
            .expect("lifecycle overview should succeed");

        assert_eq!(overview.session_id, "alpha");
        assert_eq!(overview.stored_neurons, 1);
        assert_eq!(overview.active_topic_sessions, 1);
        assert_eq!(overview.neurons_with_transcript_provenance, 1);
        assert_eq!(overview.neurons_with_memory_provenance, 1);
        assert_eq!(overview.neurons_with_evidence_digest, 1);
        assert_eq!(overview.v2_compressed_neurons, 1);
        assert_eq!(overview.neurons_with_skill_priors, 1);
        assert_eq!(overview.neurons_with_workflow_priors, 1);
        assert_eq!(overview.intuition_ready_neurons, 1);
        assert!(overview.neuron_upgrade_ready);
        assert_eq!(
            overview
                .compression_policy_versions
                .get(MEMORY_NEURON_COMPRESSION_V2_POLICY),
            Some(&1)
        );
        assert_eq!(overview.cross_session_stable_neurons, 1);
        assert_eq!(overview.cross_session_unstable_neurons, 0);
        assert!(overview.average_confidence > 0.0);
        assert!(overview.average_freshness > 0.0);
        assert_eq!(overview.stale_neurons, 0);
        assert_eq!(overview.low_confidence_neurons, 0);
        assert_eq!(overview.low_freshness_neurons, 0);
        assert!(overview.active_topics_without_neurons.is_empty());
        assert!(overview.healthy);
        assert!(overview.findings.is_empty());
    }

    #[tokio::test]
    async fn intelligence_phase2_gate_closes_memory_intelligence_next_phase() {
        let runtime = RuntimeKernel::new();

        let overview = runtime
            .intelligence_phase2_gate("phase2")
            .await
            .expect("phase2 gate should succeed");

        assert_eq!(overview.status, "complete");
        assert_eq!(overview.overall_percent, 100);
        assert!(overview.all_phase2_gates_ready);
        assert!(overview.blended_recall_ready);
        assert!(overview.provenance_memory_ready);
        assert!(overview.semantic_router_generalized);
        assert!(overview.neuron_compression_ready);
        assert!(overview.recall_source_count >= 4);
        assert!(overview.recall_ranked_items >= 4);
        assert_eq!(overview.recall_low_trust_ranked_items, 0);
        assert_eq!(overview.recall_low_recency_ranked_items, 0);
        assert_eq!(overview.recall_memory_control_omitted_items, 0);
        assert!(overview.recall_transcript_evidence_spans > 0);
        assert!(overview.durable_memory_hits > 0);
        assert!(overview.active_neurons > 0);
        assert!(overview.provenance_topic_sessions_with_transcript > 0);
        assert!(overview.supported_semantic_router_count >= 3);
        assert!(overview.learned_router_signal_count > 0);
        assert!(overview.compressed_neuron_count > 0);
        assert!(overview.neurons_with_evidence_digest >= overview.compressed_neuron_count);
        assert_eq!(overview.gates.len(), 4);
        assert!(overview.gates.iter().all(|gate| gate.ready));
        assert!(overview.findings.is_empty());
    }

    #[tokio::test]
    async fn intelligence_phase2_gate_surfaces_memory_control_omission_pressure() {
        let runtime = RuntimeKernel::new();
        runtime
            .memory
            .put(MemoryRecord {
                id: "phase2-tombstone".into(),
                scope: MemoryScope::LongTerm,
                content: "[hepta-memory:tombstone] hello adaptive memory retired path".into(),
            })
            .await
            .expect("tombstone should store");
        runtime
            .memory
            .put(MemoryRecord {
                id: "phase2-conflict".into(),
                scope: MemoryScope::Session,
                content: "[hepta-memory:conflict] hello adaptive memory stale summary".into(),
            })
            .await
            .expect("conflict should store");

        let overview = runtime
            .intelligence_phase2_gate("phase2-control")
            .await
            .expect("phase2 gate should succeed");

        assert_eq!(overview.recall_memory_control_omitted_items, 2);
        assert!(overview.gates.iter().any(|gate| {
            gate.id == "blended_recall" && gate.evidence.contains("control_omitted=2")
        }));
    }

    #[tokio::test]
    async fn knowledge_graph_dry_run_overview_exposes_candidates_without_live_write() {
        let runtime = RuntimeKernel::new();

        let report = runtime.knowledge_graph_dry_run_overview();

        assert_eq!(report.status, "ready");
        assert!(report.candidate_count > 0);
        assert_eq!(report.memory_unit_count, report.candidate_count);
        assert_eq!(report.live_write_enabled_count, 0);
        assert_eq!(report.external_side_effect_enabled_count, 0);
        assert!(report.checks.ready());
        assert!(report.checks.all_candidates_have_provenance);
        assert!(report.checks.all_candidates_have_graph_payload);
        assert!(report.checks.all_plans_are_dry_run);
        assert!(report.checks.no_live_write_enabled);
        assert!(report.checks.no_external_side_effects);
    }

    #[tokio::test]
    async fn knowledge_graph_adapter_dry_run_overview_projects_external_targets() {
        let runtime = RuntimeKernel::new();

        let report = runtime.knowledge_graph_adapter_dry_run_overview();

        assert_eq!(report.status, "ready");
        assert!(report.candidate_count > 0);
        assert_eq!(report.adapter_count, 3);
        assert_eq!(
            report.projection_count,
            report.candidate_count * report.adapter_count
        );
        assert_eq!(report.network_call_enabled_count, 0);
        assert_eq!(report.external_write_enabled_count, 0);
        assert_eq!(report.live_write_enabled_count, 0);
        assert!(report.checks.ready());
        assert!(
            report
                .projections
                .iter()
                .any(|plan| plan.adapter_id == "graphiti")
        );
        assert!(
            report
                .projections
                .iter()
                .any(|plan| plan.adapter_id == "neo4j")
        );
        assert!(
            report
                .projections
                .iter()
                .any(|plan| plan.adapter_id == "cocoindex")
        );
    }

    #[tokio::test]
    async fn knowledge_graph_adapter_staging_gate_overview_keeps_adapters_closed() {
        let runtime = RuntimeKernel::new();

        let report = runtime.knowledge_graph_adapter_staging_gate_overview();

        assert_eq!(report.status, "ready");
        assert!(report.candidate_count > 0);
        assert_eq!(report.adapter_count, 3);
        assert_eq!(
            report.staging_plan_count,
            report.candidate_count * report.adapter_count
        );
        assert_eq!(report.staging_ready_count, 0);
        assert_eq!(report.network_call_enabled_count, 0);
        assert_eq!(report.external_write_enabled_count, 0);
        assert_eq!(report.live_write_enabled_count, 0);
        assert!(report.checks.ready());
        assert!(report.checks.operator_review_required);
        assert!(report.checks.rollback_plan_required);
        assert!(report.checks.post_write_validation_required);
    }

    #[tokio::test]
    async fn knowledge_graph_adapter_client_overview_denies_disabled_clients() {
        let runtime = RuntimeKernel::new();

        let report = runtime.knowledge_graph_adapter_client_overview();

        assert_eq!(report.status, "ready");
        assert!(report.candidate_count > 0);
        assert_eq!(report.adapter_count, 3);
        assert_eq!(
            report.client_audit_count,
            report.candidate_count * report.adapter_count
        );
        assert_eq!(report.denied_client_count, report.client_audit_count);
        assert_eq!(report.network_call_attempted_count, 0);
        assert_eq!(report.external_write_attempted_count, 0);
        assert_eq!(report.live_write_attempted_count, 0);
        assert_eq!(report.persisted_record_count, 0);
        assert!(report.checks.ready());
        assert!(report.checks.all_supported_clients_present);
        assert!(report.checks.all_client_calls_denied_by_default);
    }

    #[tokio::test]
    async fn knowledge_graph_adapter_config_env_overview_reads_default_closed_snapshot() {
        let runtime = RuntimeKernel::new();

        let report = runtime.knowledge_graph_adapter_config_env_overview();

        assert_eq!(report.status, "ready");
        assert_eq!(report.adapter_count, 3);
        assert_eq!(report.config_read_count, 3);
        assert_eq!(report.feature_enabled_count, 0);
        assert_eq!(report.endpoint_configured_count, 0);
        assert_eq!(report.credentials_configured_count, 0);
        assert_eq!(report.fully_configured_count, 0);
        assert_eq!(report.live_write_requested_count, 0);
        assert_eq!(report.credential_value_captured_count, 0);
        assert_eq!(report.network_call_attempted_count, 0);
        assert_eq!(report.external_write_attempted_count, 0);
        assert_eq!(report.live_write_attempted_count, 0);
        assert!(report.checks.ready());
        assert!(report.checks.all_supported_adapters_read);
        assert!(report.checks.all_configs_closed_by_default);
    }

    #[tokio::test]
    async fn knowledge_graph_recall_plan_overview_stays_read_only() {
        let runtime = RuntimeKernel::new();

        let report = runtime.knowledge_graph_recall_plan_overview();

        assert_eq!(report.status, "ready");
        assert_eq!(report.query_count, 2);
        assert!(report.candidate_count > 0);
        assert!(report.entity_match_count > 0);
        assert!(report.relation_neighborhood_count > 0);
        assert!(report.timeline_slice_count > 0);
        assert!(report.evidence_path_count > 0);
        assert_eq!(report.external_read_enabled_count, 0);
        assert_eq!(report.network_call_enabled_count, 0);
        assert_eq!(report.live_write_enabled_count, 0);
        assert!(report.checks.ready());
        assert!(report.checks.all_plans_are_read_only);
        assert!(report.checks.no_external_reads_enabled);
        assert!(report.checks.no_network_calls_enabled);
        assert!(report.checks.no_live_writes_enabled);
    }

    #[tokio::test]
    async fn knowledge_graph_context_recall_bridge_overview_emits_kg_ranked_items() {
        let runtime = RuntimeKernel::new();

        let report = runtime.knowledge_graph_context_recall_bridge_overview();

        assert_eq!(report.status, "ready");
        assert_eq!(report.query_count, 2);
        assert!(report.kg_plan_count > 0);
        assert!(report.kg_evidence_path_count > 0);
        assert!(report.context_item_count > 0);
        assert!(report.transcript_span_count > 0);
        assert_eq!(report.external_read_enabled_count, 0);
        assert_eq!(report.network_call_enabled_count, 0);
        assert_eq!(report.live_write_enabled_count, 0);
        assert!(!report.model_invoked);
        assert!(!report.context_injection_performed);
        assert!(report.checks.ready());
        assert!(report.checks.all_items_have_kg_source);
        assert!(report.checks.transcript_provenance_preserved);
        assert!(report.checks.no_context_injection_performed);
    }

    #[tokio::test]
    async fn knowledge_graph_recall_evaluation_overview_keeps_quality_gate_report_only() {
        let runtime = RuntimeKernel::new();

        let report = runtime.knowledge_graph_recall_evaluation_overview();

        assert_eq!(report.status, "ready");
        assert_eq!(report.query_count, 2);
        assert!(report.evaluation_case_count > 0);
        assert_eq!(report.passed_case_count, report.evaluation_case_count);
        assert_eq!(report.failed_case_count, 0);
        assert_eq!(report.coverage_basis_points, 10_000);
        assert_eq!(report.precision_proxy_basis_points, 10_000);
        assert_eq!(report.score_stability_basis_points, 10_000);
        assert_eq!(report.duplicate_source_memory_id_count, 0);
        assert_eq!(report.score_order_violation_count, 0);
        assert_eq!(report.external_read_enabled_count, 0);
        assert_eq!(report.network_call_enabled_count, 0);
        assert_eq!(report.live_write_enabled_count, 0);
        assert!(!report.model_invoked);
        assert!(!report.context_injection_performed);
        assert!(report.checks.ready());
        assert!(report.checks.source_memory_ids_unique);
        assert!(report.checks.scores_stably_ordered);
        assert!(report.checks.no_context_injection_performed);
    }

    #[tokio::test]
    async fn knowledge_graph_context_injection_readiness_overview_blocks_injection() {
        let runtime = RuntimeKernel::new();

        let report = runtime.knowledge_graph_context_injection_readiness_overview();

        assert_eq!(report.status, "blocked");
        assert!(report.quality_gate_ready);
        assert_eq!(report.evaluation_case_count, report.passed_case_count);
        assert_eq!(report.failed_case_count, 0);
        assert_eq!(report.coverage_basis_points, 10_000);
        assert_eq!(report.precision_proxy_basis_points, 10_000);
        assert_eq!(report.score_stability_basis_points, 10_000);
        assert_eq!(report.quality_threshold_basis_points, 9_000);
        assert!(!report.operator_approved);
        assert!(!report.shadow_rank_enabled);
        assert!(!report.rollback_plan_ready);
        assert!(!report.kill_switch_ready);
        assert!(!report.context_injection_allowed);
        assert!(!report.context_injection_performed);
        assert!(!report.prompt_preview_rendered);
        assert!(!report.model_invoked);
        assert_eq!(report.external_read_enabled_count, 0);
        assert_eq!(report.network_call_enabled_count, 0);
        assert_eq!(report.live_write_enabled_count, 0);
        assert!(report.checks.ready());
        assert!(report.checks.activation_blocked_without_operator_approval);
        assert!(report.checks.no_context_injection_performed);
    }

    #[tokio::test]
    async fn knowledge_graph_shadow_rank_overview_observes_without_injection() {
        let runtime = RuntimeKernel::new();

        let report = runtime.knowledge_graph_shadow_rank_overview();

        assert_eq!(report.status, "ready");
        assert_eq!(report.injection_readiness_status, "blocked");
        assert!(report.context_item_count > 0);
        assert_eq!(report.ranked_item_count, report.context_item_count);
        assert_eq!(report.observed_only_count, report.ranked_item_count);
        assert_eq!(report.would_enter_prompt_context_count, 0);
        assert!(!report.prompt_preview_rendered);
        assert!(!report.model_invoked);
        assert!(!report.context_injection_performed);
        assert_eq!(report.external_read_enabled_count, 0);
        assert_eq!(report.network_call_enabled_count, 0);
        assert_eq!(report.live_write_enabled_count, 0);
        assert!(report.checks.ready());
        assert!(report.checks.injection_readiness_blocked);
        assert!(report.checks.all_items_observed_only);
        assert!(report.checks.no_items_enter_prompt_context);
        assert!(report.checks.scores_stably_ordered);
        assert!(report.checks.no_context_injection_performed);
    }

    #[tokio::test]
    async fn knowledge_graph_shadow_rank_comparison_overview_compares_without_injection() {
        let runtime = RuntimeKernel::new();

        let report = runtime.knowledge_graph_shadow_rank_comparison_overview();

        assert_eq!(report.status, "ready");
        assert_eq!(
            report.kg_shadow_rank_contract,
            hepta_intelligence::MEMORY_KG_SHADOW_RANK_V0_CONTRACT
        );
        assert_eq!(
            report.kg_context_injection_readiness_contract,
            hepta_intelligence::MEMORY_KG_CONTEXT_INJECTION_READINESS_V0_CONTRACT
        );
        assert!(report.kg_ranked_item_count > 0);
        assert_eq!(
            report.transcript_baseline_count,
            report.kg_ranked_item_count
        );
        assert_eq!(
            report.durable_memory_baseline_count,
            report.kg_ranked_item_count
        );
        assert_eq!(
            report.comparison_case_count,
            report.kg_ranked_item_count * 2
        );
        assert!(!report.prompt_preview_rendered);
        assert!(!report.model_invoked);
        assert!(!report.context_injection_performed);
        assert_eq!(report.external_read_enabled_count, 0);
        assert_eq!(report.network_call_enabled_count, 0);
        assert_eq!(report.live_write_enabled_count, 0);
        assert!(report.checks.ready());
        assert!(report.checks.shadow_rank_ready);
        assert!(report.checks.no_kg_items_enter_prompt_context);
        assert!(report.checks.no_baseline_items_enter_prompt_context);
        assert!(report.checks.no_context_injection_performed);
    }

    #[tokio::test]
    async fn knowledge_graph_shadow_rank_drift_overview_gates_regression_without_injection() {
        let runtime = RuntimeKernel::new();

        let report = runtime.knowledge_graph_shadow_rank_drift_overview();

        assert_eq!(report.status, "ready");
        assert_eq!(report.verdict, "stable");
        assert_eq!(
            report.kg_shadow_rank_comparison_contract,
            hepta_intelligence::MEMORY_KG_SHADOW_RANK_COMPARISON_V0_CONTRACT
        );
        assert_eq!(
            report.kg_shadow_rank_contract,
            hepta_intelligence::MEMORY_KG_SHADOW_RANK_V0_CONTRACT
        );
        assert!(report.top_n_kg_rank_count > 0);
        assert_eq!(report.expected_drift_case_count, report.drift_case_count);
        assert_eq!(report.stable_case_count, report.drift_case_count);
        assert_eq!(report.drifted_case_count, 0);
        assert_eq!(report.transcript_case_count, report.top_n_kg_rank_count);
        assert_eq!(report.durable_memory_case_count, report.top_n_kg_rank_count);
        assert!(!report.prompt_preview_rendered);
        assert!(!report.model_invoked);
        assert!(!report.context_injection_performed);
        assert_eq!(report.external_read_enabled_count, 0);
        assert_eq!(report.network_call_enabled_count, 0);
        assert_eq!(report.live_write_enabled_count, 0);
        assert!(report.checks.ready());
        assert!(report.checks.comparison_ready);
        assert!(report.checks.rank_order_stable);
        assert!(report.checks.score_delta_within_thresholds);
        assert!(report.checks.prompt_flags_stable);
        assert!(report.checks.no_context_injection_performed);
    }

    #[tokio::test]
    async fn knowledge_graph_prompt_preview_approval_packet_overview_blocks_prompt_preview() {
        let runtime = RuntimeKernel::new();

        let report = runtime.knowledge_graph_prompt_preview_approval_packet_overview();

        assert_eq!(report.status, "blocked");
        assert_eq!(
            report.contract,
            hepta_intelligence::MEMORY_KG_PROMPT_PREVIEW_APPROVAL_PACKET_V0_CONTRACT
        );
        assert_eq!(
            report.kg_shadow_rank_drift_contract,
            hepta_intelligence::MEMORY_KG_SHADOW_RANK_DRIFT_V0_CONTRACT
        );
        assert!(report.drift_case_count > 0);
        assert_eq!(report.approval_item_count, report.drift_case_count);
        assert_eq!(
            report.redacted_context_ref_count,
            report.approval_item_count
        );
        assert_eq!(report.drifted_case_count, 0);
        assert!(!report.operator_approval_recorded);
        assert!(!report.rollback_plan_ready);
        assert!(!report.kill_switch_ready);
        assert!(!report.approval_packet_accepted);
        assert!(!report.prompt_preview_allowed);
        assert!(!report.prompt_preview_rendered);
        assert!(!report.prompt_payload_materialized);
        assert!(!report.context_injection_allowed);
        assert!(!report.context_injection_performed);
        assert!(!report.model_invoked);
        assert_eq!(report.external_read_enabled_count, 0);
        assert_eq!(report.network_call_enabled_count, 0);
        assert_eq!(report.live_write_enabled_count, 0);
        assert!(report.checks.ready());
        assert!(report.checks.drift_gate_stable);
        assert!(report.checks.operator_approval_required);
        assert!(report.checks.prompt_preview_disabled_by_default);
        assert!(report.checks.prompt_payload_not_materialized);
        assert!(report.checks.context_injection_disabled_by_default);
        assert!(report.checks.no_context_injection_performed);
    }

    #[tokio::test]
    async fn knowledge_graph_prompt_preview_operator_evidence_overview_blocks_preview() {
        let runtime = RuntimeKernel::new();

        let report = runtime.knowledge_graph_prompt_preview_operator_evidence_overview();

        assert_eq!(report.status, "blocked");
        assert_eq!(
            report.contract,
            hepta_intelligence::MEMORY_KG_PROMPT_PREVIEW_OPERATOR_EVIDENCE_V0_CONTRACT
        );
        assert_eq!(
            report.approval_packet_contract,
            hepta_intelligence::MEMORY_KG_PROMPT_PREVIEW_APPROVAL_PACKET_V0_CONTRACT
        );
        assert_eq!(report.approval_packet_status, "blocked");
        assert_eq!(report.required_evidence_count, 7);
        assert_eq!(
            report.missing_evidence_count,
            report.required_evidence_count
        );
        assert!(!report.operator_approval_evidence_present);
        assert!(!report.rollback_plan_evidence_present);
        assert!(!report.kill_switch_evidence_present);
        assert!(report.reviewer_identity_redacted);
        assert!(!report.signed_approval_digest_present);
        assert!(!report.bounded_preview_scope_present);
        assert!(!report.prompt_preview_allowed);
        assert!(!report.prompt_preview_rendered);
        assert!(!report.prompt_payload_materialized);
        assert!(!report.context_injection_allowed);
        assert!(!report.context_injection_performed);
        assert!(!report.model_invoked);
        assert_eq!(report.external_read_enabled_count, 0);
        assert_eq!(report.network_call_enabled_count, 0);
        assert_eq!(report.live_write_enabled_count, 0);
        assert!(report.checks.ready());
        assert!(report.checks.approval_packet_not_accepted);
        assert!(report.checks.evidence_requirements_all_blocking);
        assert!(report.checks.operator_approval_evidence_required);
        assert!(report.checks.signed_approval_digest_required);
        assert!(report.checks.prompt_preview_disabled);
        assert!(report.checks.prompt_payload_not_materialized);
        assert!(report.checks.context_injection_disabled);
        assert!(report.checks.no_model_invoked);
        assert!(report.checks.no_context_injection_performed);
        assert!(report.checks.no_external_reads_enabled);
        assert!(report.checks.no_network_calls_enabled);
        assert!(report.checks.no_live_writes_enabled);
    }

    #[tokio::test]
    async fn knowledge_graph_prompt_preview_redaction_diff_overview_suppresses_raw_diff() {
        let runtime = RuntimeKernel::new();

        let report = runtime.knowledge_graph_prompt_preview_redaction_diff_overview();

        assert_eq!(report.status, "blocked");
        assert_eq!(
            report.contract,
            hepta_intelligence::MEMORY_KG_PROMPT_PREVIEW_REDACTION_DIFF_V0_CONTRACT
        );
        assert_eq!(
            report.operator_evidence_contract,
            hepta_intelligence::MEMORY_KG_PROMPT_PREVIEW_OPERATOR_EVIDENCE_V0_CONTRACT
        );
        assert_eq!(report.operator_evidence_status, "blocked");
        assert_eq!(report.required_evidence_count, 7);
        assert_eq!(
            report.missing_evidence_count,
            report.required_evidence_count
        );
        assert_eq!(report.diff_item_count, report.required_evidence_count);
        assert_eq!(report.redacted_ref_count, report.diff_item_count);
        assert_eq!(report.raw_prompt_diff_count, 0);
        assert_eq!(report.prompt_text_included_count, 0);
        assert_eq!(report.payload_text_included_count, 0);
        assert!(report.redacted_diff_reported);
        assert!(!report.prompt_preview_allowed);
        assert!(!report.prompt_preview_rendered);
        assert!(!report.prompt_payload_materialized);
        assert!(!report.context_injection_allowed);
        assert!(!report.context_injection_performed);
        assert!(!report.model_invoked);
        assert_eq!(report.external_read_enabled_count, 0);
        assert_eq!(report.network_call_enabled_count, 0);
        assert_eq!(report.live_write_enabled_count, 0);
        assert!(report.checks.ready());
        assert!(report.checks.operator_evidence_contract_linked);
        assert!(report.checks.operator_evidence_checks_ready);
        assert!(report.checks.operator_evidence_missing_requirements);
        assert!(report.checks.redacted_refs_present);
        assert!(report.checks.raw_prompt_diff_suppressed);
        assert!(report.checks.prompt_text_excluded);
        assert!(report.checks.payload_text_excluded);
        assert!(report.checks.prompt_preview_disabled);
        assert!(report.checks.prompt_payload_not_materialized);
        assert!(report.checks.context_injection_disabled);
        assert!(report.checks.no_model_invoked);
        assert!(report.checks.no_context_injection_performed);
        assert!(report.checks.no_external_reads_enabled);
        assert!(report.checks.no_network_calls_enabled);
        assert!(report.checks.no_live_writes_enabled);
    }

    #[tokio::test]
    async fn knowledge_graph_prompt_preview_rollback_kill_switch_overview_blocks_preview() {
        let runtime = RuntimeKernel::new();

        let report = runtime.knowledge_graph_prompt_preview_rollback_kill_switch_overview();

        assert_eq!(report.status, "blocked");
        assert_eq!(
            report.contract,
            hepta_intelligence::MEMORY_KG_PROMPT_PREVIEW_ROLLBACK_KILL_SWITCH_V0_CONTRACT
        );
        assert_eq!(
            report.redaction_diff_contract,
            hepta_intelligence::MEMORY_KG_PROMPT_PREVIEW_REDACTION_DIFF_V0_CONTRACT
        );
        assert_eq!(report.redaction_diff_status, "blocked");
        assert_eq!(report.required_evidence_count, 7);
        assert_eq!(
            report.missing_evidence_count,
            report.required_evidence_count
        );
        assert_eq!(report.required_control_count, 4);
        assert_eq!(report.missing_control_count, report.required_control_count);
        assert_eq!(report.rollback_control_count, 2);
        assert_eq!(report.kill_switch_control_count, 2);
        assert!(!report.rollback_plan_ready);
        assert!(!report.rollback_exercise_ready);
        assert!(!report.kill_switch_ready);
        assert!(!report.kill_switch_dry_run_ready);
        assert_eq!(report.raw_prompt_diff_count, 0);
        assert_eq!(report.prompt_text_included_count, 0);
        assert_eq!(report.payload_text_included_count, 0);
        assert!(!report.prompt_preview_allowed);
        assert!(!report.prompt_preview_rendered);
        assert!(!report.prompt_payload_materialized);
        assert!(!report.context_injection_allowed);
        assert!(!report.context_injection_performed);
        assert!(!report.model_invoked);
        assert_eq!(report.external_read_enabled_count, 0);
        assert_eq!(report.network_call_enabled_count, 0);
        assert_eq!(report.live_write_enabled_count, 0);
        assert!(report.checks.ready());
        assert!(report.checks.redaction_diff_contract_linked);
        assert!(report.checks.redaction_diff_checks_ready);
        assert!(report.checks.redaction_diff_blocked);
        assert!(report.checks.only_redacted_refs_reported);
        assert!(report.checks.rollback_controls_nonzero);
        assert!(report.checks.kill_switch_controls_nonzero);
        assert!(report.checks.controls_all_missing_and_blocking);
        assert!(report.checks.rollback_plan_required);
        assert!(report.checks.rollback_exercise_required);
        assert!(report.checks.kill_switch_required);
        assert!(report.checks.kill_switch_dry_run_required);
        assert!(report.checks.prompt_preview_disabled);
        assert!(report.checks.prompt_payload_not_materialized);
        assert!(report.checks.context_injection_disabled);
        assert!(report.checks.no_model_invoked);
        assert!(report.checks.no_context_injection_performed);
        assert!(report.checks.no_external_reads_enabled);
        assert!(report.checks.no_network_calls_enabled);
        assert!(report.checks.no_live_writes_enabled);
    }

    #[tokio::test]
    async fn knowledge_graph_prompt_preview_context_handoff_overview_blocks_context_injection() {
        let runtime = RuntimeKernel::new();

        let report = runtime.knowledge_graph_prompt_preview_context_handoff_overview();

        assert_eq!(report.status, "blocked");
        assert_eq!(
            report.contract,
            hepta_intelligence::MEMORY_KG_PROMPT_PREVIEW_CONTEXT_HANDOFF_V0_CONTRACT
        );
        assert_eq!(
            report.safety_gate_contract,
            hepta_intelligence::MEMORY_KG_PROMPT_PREVIEW_ROLLBACK_KILL_SWITCH_V0_CONTRACT
        );
        assert_eq!(report.safety_gate_status, "blocked");
        assert_eq!(
            report.redaction_diff_contract,
            hepta_intelligence::MEMORY_KG_PROMPT_PREVIEW_REDACTION_DIFF_V0_CONTRACT
        );
        assert_eq!(report.required_evidence_count, 7);
        assert_eq!(
            report.missing_evidence_count,
            report.required_evidence_count
        );
        assert_eq!(report.required_control_count, 4);
        assert_eq!(report.missing_control_count, report.required_control_count);
        assert_eq!(report.handoff_requirement_count, 6);
        assert_eq!(
            report.missing_handoff_requirement_count,
            report.handoff_requirement_count
        );
        assert_eq!(report.redacted_ref_count, report.required_evidence_count);
        assert_eq!(report.raw_prompt_diff_count, 0);
        assert_eq!(report.prompt_text_included_count, 0);
        assert_eq!(report.payload_text_included_count, 0);
        assert!(!report.redacted_diff_review_present);
        assert!(!report.context_handoff_approval_present);
        assert!(!report.prompt_preview_allowed);
        assert!(!report.prompt_preview_rendered);
        assert!(!report.prompt_payload_materialized);
        assert!(!report.context_injection_allowed);
        assert!(!report.context_injection_performed);
        assert!(!report.model_invoked);
        assert_eq!(report.external_read_enabled_count, 0);
        assert_eq!(report.network_call_enabled_count, 0);
        assert_eq!(report.live_write_enabled_count, 0);
        assert!(report.checks.ready());
        assert!(report.checks.safety_gate_contract_linked);
        assert!(report.checks.safety_gate_checks_ready);
        assert!(report.checks.safety_gate_blocked);
        assert!(report.checks.operator_evidence_incomplete);
        assert!(report.checks.safety_controls_incomplete);
        assert!(report.checks.handoff_requirements_nonzero);
        assert!(report.checks.handoff_requirements_all_missing_and_blocking);
        assert!(report.checks.redacted_refs_only);
        assert!(report.checks.redacted_diff_review_required);
        assert!(report.checks.context_handoff_approval_required);
        assert!(report.checks.prompt_preview_disabled);
        assert!(report.checks.prompt_payload_not_materialized);
        assert!(report.checks.context_injection_disabled);
        assert!(report.checks.no_model_invoked);
        assert!(report.checks.no_context_injection_performed);
        assert!(report.checks.no_external_reads_enabled);
        assert!(report.checks.no_network_calls_enabled);
        assert!(report.checks.no_live_writes_enabled);
    }

    #[tokio::test]
    async fn knowledge_graph_prompt_preview_preflight_overview_blocks_ci_promotion() {
        let runtime = RuntimeKernel::new();

        let report = runtime.knowledge_graph_prompt_preview_preflight_overview();

        assert_eq!(report.status, "blocked");
        assert_eq!(
            report.contract,
            hepta_intelligence::MEMORY_KG_PROMPT_PREVIEW_PREFLIGHT_V0_CONTRACT
        );
        assert_eq!(
            report.context_handoff_contract,
            hepta_intelligence::MEMORY_KG_PROMPT_PREVIEW_CONTEXT_HANDOFF_V0_CONTRACT
        );
        assert_eq!(report.context_handoff_status, "blocked");
        assert_eq!(report.source_gate_count, 5);
        assert_eq!(report.ready_source_gate_count, report.source_gate_count);
        assert_eq!(report.blocked_source_gate_count, report.source_gate_count);
        assert_eq!(
            report.report_only_source_gate_count,
            report.source_gate_count
        );
        assert_eq!(report.required_total_preflight_requirement_count, 19);
        assert_eq!(
            report.missing_total_preflight_requirement_count,
            report.required_total_preflight_requirement_count
        );
        assert_eq!(report.raw_prompt_diff_count, 0);
        assert_eq!(report.prompt_text_included_count, 0);
        assert_eq!(report.payload_text_included_count, 0);
        assert!(!report.redacted_diff_review_present);
        assert!(!report.context_handoff_approval_present);
        assert!(!report.prompt_preview_allowed);
        assert!(!report.prompt_preview_rendered);
        assert!(!report.prompt_payload_materialized);
        assert!(!report.context_injection_allowed);
        assert!(!report.context_injection_performed);
        assert!(!report.model_invoked);
        assert!(!report.ci_promotion_allowed);
        assert!(!report.preflight_execution_performed);
        assert_eq!(report.external_read_enabled_count, 0);
        assert_eq!(report.network_call_enabled_count, 0);
        assert_eq!(report.live_write_enabled_count, 0);
        assert!(report.checks.ready());
        assert!(report.checks.source_gates_nonzero);
        assert!(report.checks.source_gates_all_linked);
        assert!(report.checks.source_gates_all_checks_ready);
        assert!(report.checks.source_gates_all_blocked);
        assert!(report.checks.source_gates_all_report_only);
        assert!(report.checks.context_handoff_contract_linked);
        assert!(report.checks.context_handoff_checks_ready);
        assert!(report.checks.context_handoff_blocked);
        assert!(report.checks.operator_evidence_incomplete);
        assert!(report.checks.safety_controls_incomplete);
        assert!(report.checks.handoff_requirements_incomplete);
        assert!(report.checks.redacted_diff_review_required);
        assert!(report.checks.context_handoff_approval_required);
        assert!(report.checks.redacted_refs_only);
        assert!(report.checks.prompt_preview_disabled);
        assert!(report.checks.prompt_payload_not_materialized);
        assert!(report.checks.context_injection_disabled);
        assert!(report.checks.no_model_invoked);
        assert!(report.checks.no_context_injection_performed);
        assert!(report.checks.no_external_reads_enabled);
        assert!(report.checks.no_network_calls_enabled);
        assert!(report.checks.no_live_writes_enabled);
        assert!(report.checks.ci_promotion_disabled);
        assert!(report.checks.no_preflight_execution_performed);
    }

    #[tokio::test]
    async fn route_topics_splits_merged_topic_back_into_component_topics() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("first turn should succeed");
        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("first route should succeed");
        runtime
            .run_demo_turn("rust worker pipeline")
            .await
            .expect("second turn should succeed");
        runtime
            .route_topics("alpha", Some("rust worker pipeline"), 6, 6, 6, 1)
            .expect("second route should succeed");
        runtime
            .route_topics(
                "alpha",
                Some("merge hello adaptive memory and rust worker pipeline"),
                8,
                8,
                8,
                2,
            )
            .expect("merge route should succeed");

        let decision = runtime
            .route_topics(
                "alpha",
                Some("split hello adaptive memory and rust worker pipeline"),
                8,
                8,
                8,
                2,
            )
            .expect("split route should succeed");
        let topic_sessions = runtime
            .topic_sessions_for_surface("alpha")
            .expect("topic sessions should load");

        assert_eq!(
            decision.active_topic_session_ids,
            vec![
                "topic-session-bootstrap:alpha",
                "topic-session-bootstrap:alpha:rust-worker-pipeline"
            ]
        );
        assert!(decision.created_topic_session_ids.is_empty());
        assert_eq!(
            decision.revived_topic_session_ids,
            vec![
                "topic-session-bootstrap:alpha",
                "topic-session-bootstrap:alpha:rust-worker-pipeline"
            ]
        );
        assert!(matches!(
            decision.shift_event.expect("shift event should exist").kind,
            TopicShiftKind::Split
        ));
        assert!(topic_sessions.iter().any(|topic_session| {
            topic_session.topic_session_id == "topic-session-bootstrap:alpha"
                && matches!(topic_session.status, TopicSessionStatus::Active)
        }));
        assert!(topic_sessions.iter().any(|topic_session| {
            topic_session.topic_session_id == "topic-session-bootstrap:alpha:rust-worker-pipeline"
                && matches!(topic_session.status, TopicSessionStatus::Active)
        }));
        assert!(topic_sessions.iter().any(|topic_session| {
            topic_session.topic_session_id
                == "topic-session-bootstrap:alpha:hello-adaptive-memory-rust-worker-pipeline"
                && matches!(topic_session.status, TopicSessionStatus::Dormant)
        }));
    }

    #[tokio::test]
    async fn route_topics_graph_expands_component_query_to_adjacent_composite_topic() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("first turn should succeed");
        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("first route should succeed");
        runtime
            .run_demo_turn("rust worker pipeline")
            .await
            .expect("second turn should succeed");
        runtime
            .route_topics("alpha", Some("rust worker pipeline"), 6, 6, 6, 1)
            .expect("second route should succeed");
        runtime
            .route_topics(
                "alpha",
                Some("merge hello adaptive memory and rust worker pipeline"),
                8,
                8,
                8,
                2,
            )
            .expect("merge route should succeed");

        let decision = runtime
            .route_topics("alpha", Some("hello adaptive memory"), 8, 8, 8, 2)
            .expect("graph-expanded route should succeed");
        let topic_sessions = runtime
            .topic_sessions_for_surface("alpha")
            .expect("topic sessions should load");

        assert_eq!(decision.active_topic_session_ids.len(), 2);
        assert_eq!(
            decision.revived_topic_session_ids,
            vec!["topic-session-bootstrap:alpha"]
        );
        assert!(
            decision
                .active_topic_session_ids
                .iter()
                .any(|id| id == "topic-session-bootstrap:alpha")
        );
        assert!(decision.active_topic_session_ids.iter().any(|id| {
            id == "topic-session-bootstrap:alpha:hello-adaptive-memory-rust-worker-pipeline"
        }));
        assert!(matches!(
            decision.shift_event.expect("shift event should exist").kind,
            TopicShiftKind::CoActivated
        ));
        assert!(decision.activation_scores.iter().any(|score| {
            score.topic_id.0 == "topic-alpha-hello-adaptive-memory-rust-worker-pipeline"
                && score.reason.as_deref().is_some_and(|reason| {
                    reason.contains("bootstrap topic graph expanded 'hello adaptive memory'")
                })
        }));
        assert!(topic_sessions.iter().any(|topic_session| {
            topic_session.topic_session_id == "topic-session-bootstrap:alpha"
                && matches!(topic_session.status, TopicSessionStatus::Active)
        }));
        assert!(topic_sessions.iter().any(|topic_session| {
            topic_session.topic_session_id
                == "topic-session-bootstrap:alpha:hello-adaptive-memory-rust-worker-pipeline"
                && matches!(topic_session.status, TopicSessionStatus::Active)
        }));
        assert!(topic_sessions.iter().any(|topic_session| {
            topic_session.topic_session_id == "topic-session-bootstrap:alpha:rust-worker-pipeline"
                && matches!(topic_session.status, TopicSessionStatus::Merged)
        }));
    }

    #[tokio::test]
    async fn route_topics_graph_expands_single_topic_query_via_stored_coactivation_edge() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("first turn should succeed");
        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("first route should succeed");
        runtime
            .run_demo_turn("rust worker pipeline")
            .await
            .expect("second turn should succeed");
        runtime
            .route_topics("alpha", Some("rust worker pipeline"), 6, 6, 6, 1)
            .expect("second route should succeed");
        runtime
            .route_topics(
                "alpha",
                Some("hello adaptive memory and rust worker pipeline"),
                8,
                8,
                8,
                2,
            )
            .expect("mixed route should succeed");

        let decision = runtime
            .route_topics("alpha", Some("hello adaptive memory"), 8, 8, 8, 2)
            .expect("graph-expanded route should succeed");
        let topic_sessions = runtime
            .topic_sessions_for_surface("alpha")
            .expect("topic sessions should load");
        let raw_topic_sessions = runtime
            .topic_session_state
            .lock()
            .expect("topic session state lock should succeed")
            .sessions
            .clone();
        let topic_graph_edges = runtime
            .topic_graph_state
            .lock()
            .expect("topic graph state lock should succeed")
            .edges
            .clone();

        assert_eq!(decision.active_topic_session_ids.len(), 2);
        assert!(
            decision
                .active_topic_session_ids
                .iter()
                .any(|id| id == "topic-session-bootstrap:alpha")
        );
        assert!(
            decision
                .active_topic_session_ids
                .iter()
                .any(|id| id == "topic-session-bootstrap:alpha:rust-worker-pipeline")
        );
        assert!(decision.activation_scores.iter().any(|score| {
            score.topic_id.0 == "topic-alpha-rust-worker-pipeline"
                && score
                    .reason
                    .as_deref()
                    .is_some_and(|reason| reason.contains("stored co-activation edge"))
        }));
        assert!(topic_sessions.iter().any(|topic_session| {
            topic_session.topic_session_id == "topic-session-bootstrap:alpha"
                && bootstrap_topic_graph_edge_count(topic_session) >= 1
        }));
        assert!(topic_sessions.iter().any(|topic_session| {
            topic_session.topic_session_id == "topic-session-bootstrap:alpha:rust-worker-pipeline"
                && bootstrap_topic_graph_edge_count(topic_session) >= 1
        }));
        assert!(
            raw_topic_sessions
                .iter()
                .all(|topic_session| topic_session.graph_edges.is_empty())
        );
        assert!(topic_graph_edges.iter().any(|record| {
            record.source_topic_session_id == "topic-session-bootstrap:alpha"
                && record.edge.target_topic_session_id
                    == "topic-session-bootstrap:alpha:rust-worker-pipeline"
        }));
    }

    #[tokio::test]
    async fn neuron_activation_overview_returns_multiple_activations_for_coactivated_topics() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("first turn should succeed");
        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("first route should succeed");
        runtime
            .run_demo_turn("rust worker pipeline")
            .await
            .expect("second turn should succeed");
        runtime
            .route_topics("alpha", Some("rust worker pipeline"), 6, 6, 6, 1)
            .expect("second route should succeed");

        let overview = runtime
            .neuron_activation_overview(
                "alpha",
                Some("hello adaptive memory and rust worker pipeline"),
                8,
                8,
                8,
                3,
            )
            .expect("neuron activation overview should succeed");

        assert_eq!(overview.active_topic_session_count, 2);
        assert_eq!(overview.routed_topic_count, 2);
        assert_eq!(overview.activations.len(), 2);
        assert_eq!(
            overview.activations[0].source_topic_session_ids,
            vec![
                "topic-session-bootstrap:alpha",
                "topic-session-bootstrap:alpha:rust-worker-pipeline"
            ]
        );
        assert_eq!(
            overview.activations[1].source_topic_session_ids,
            vec![
                "topic-session-bootstrap:alpha:rust-worker-pipeline",
                "topic-session-bootstrap:alpha"
            ]
        );
        assert!(overview.activations[0].propagated_score > 0.0);
        assert!(overview.activations[1].propagated_score > 0.0);
        assert_eq!(overview.activations[0].inhibition_score, 0.0);
        assert_eq!(overview.activations[1].inhibition_score, 0.0);
        assert_eq!(
            overview.activations[0].source_neuron_ids,
            vec![NeuronId("neuron-alpha-rust-worker-pipeline".into())]
        );
        assert_eq!(
            overview.activations[1].source_neuron_ids,
            vec![NeuronId("neuron-alpha".into())]
        );
        assert!(
            overview
                .activations
                .iter()
                .all(|activation| !activation.source_transcript_spans.is_empty())
        );
    }

    #[tokio::test]
    async fn neuron_activation_overview_prefers_stored_topic_graph_edges_for_propagation() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("first turn should succeed");
        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("first route should succeed");
        runtime
            .run_demo_turn("rust worker pipeline")
            .await
            .expect("second turn should succeed");
        runtime
            .route_topics("alpha", Some("rust worker pipeline"), 6, 6, 6, 1)
            .expect("second route should succeed");
        runtime
            .route_topics(
                "alpha",
                Some("hello adaptive memory and rust worker pipeline"),
                8,
                8,
                8,
                2,
            )
            .expect("mixed route should succeed");

        let overview = runtime
            .neuron_activation_overview("alpha", Some("hello adaptive memory"), 8, 8, 8, 3)
            .expect("neuron activation overview should succeed");

        assert_eq!(overview.active_topic_session_count, 2);
        assert_eq!(overview.routed_topic_count, 2);
        assert_eq!(overview.activations.len(), 2);
        assert!(overview.activations.iter().all(|activation| {
            activation
                .source_link_reasons
                .iter()
                .any(|reason| reason.contains("stored co-activation edge"))
        }));
    }

    #[tokio::test]
    async fn neuron_activation_overview_applies_inhibitory_suppression_for_contrast_query() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("first turn should succeed");
        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("first route should succeed");
        runtime
            .run_demo_turn("rust worker pipeline")
            .await
            .expect("second turn should succeed");
        runtime
            .route_topics("alpha", Some("rust worker pipeline"), 6, 6, 6, 1)
            .expect("second route should succeed");

        let decision = runtime
            .route_topics(
                "alpha",
                Some("hello adaptive memory but not rust worker pipeline"),
                8,
                8,
                8,
                2,
            )
            .expect("contrast route should succeed");
        assert!(matches!(
            decision.shift_event.expect("shift event should exist").kind,
            TopicShiftKind::CoActivated
        ));

        let overview = runtime
            .neuron_activation_overview(
                "alpha",
                Some("hello adaptive memory but not rust worker pipeline"),
                8,
                8,
                8,
                3,
            )
            .expect("neuron activation overview should succeed");

        assert_eq!(overview.activations.len(), 2);
        assert_eq!(overview.activations[0].topic_id.0, "topic-alpha");
        assert_eq!(overview.activations[0].propagated_score, 0.0);
        assert_eq!(overview.activations[0].inhibition_score, 0.0);
        assert_eq!(
            overview.activations[1].topic_id.0,
            "topic-alpha-rust-worker-pipeline"
        );
        assert_eq!(overview.activations[1].propagated_score, 0.0);
        assert!(overview.activations[1].inhibition_score > 0.0);
        assert!(overview.activations[1].final_score < overview.activations[1].direct_score);
        assert_eq!(
            overview.activations[1].source_neuron_ids,
            vec![NeuronId("neuron-alpha".into())]
        );
        assert_eq!(
            overview.activations[1].source_link_kinds,
            vec![NeuronLinkKind::Inhibition]
        );
        assert!(
            overview.activations[1]
                .reason
                .as_deref()
                .unwrap_or_default()
                .contains("inhibitory suppression")
        );
    }

    #[tokio::test]
    async fn session_activity_overview_counts_active_archived_and_populated_sessions() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("alpha planning")
            .await
            .expect("alpha turn should succeed");
        runtime
            .rename_active_session("Alpha workspace")
            .expect("rename should succeed");
        runtime
            .run_demo_turn_in_session("beta", "beta follow-up")
            .await
            .expect("beta turn should succeed");
        runtime
            .route_topics("alpha", Some("alpha planning"), 4, 4, 4, 1)
            .expect("alpha route should succeed");
        runtime
            .archive_session(Some("beta"))
            .expect("archive should succeed");

        let overview = runtime
            .session_activity_overview(1, 2)
            .expect("session activity overview should succeed");

        assert_eq!(overview.sessions.len(), 2);
        assert_eq!(overview.active_sessions, 1);
        assert_eq!(overview.archived_sessions, 1);
        assert_eq!(overview.sessions_with_history, 2);
        assert_eq!(overview.sessions_with_events, 2);
        assert_eq!(overview.sessions_with_topic_state, 1);
        assert_eq!(overview.total_topic_sessions, 1);
        assert_eq!(overview.total_topic_graph_edges, 0);
    }

    #[tokio::test]
    async fn event_digest_rolls_up_recent_events_by_kind_and_session() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("alpha planning")
            .await
            .expect("alpha turn should succeed");
        runtime
            .rename_active_session("Alpha workspace")
            .expect("rename should succeed");
        runtime
            .run_demo_turn_in_session("beta", "beta follow-up")
            .await
            .expect("beta turn should succeed");

        let digest = runtime
            .event_digest(0)
            .expect("event digest should succeed");

        assert!(
            digest
                .kinds
                .iter()
                .any(|item| item.kind == "SessionRenamed" && item.count >= 1)
        );
        assert!(
            digest
                .sessions
                .iter()
                .any(|item| item.session_id.as_deref() == Some("bootstrap") && item.count >= 1)
        );
        let alpha = digest
            .sessions
            .iter()
            .find(|item| item.session_id.as_deref() == Some("alpha"))
            .expect("alpha session tally should exist");
        assert_eq!(alpha.latest_event.event.kind, EventKind::SessionRenamed);
        assert!(
            digest
                .events
                .iter()
                .any(|record| record.event.summary.contains("Alpha workspace"))
        );

        let sections = digest.summary_sections();
        assert_eq!(digest.recent_event_count(), digest.events.len());
        assert_eq!(digest.kind_count(), digest.kinds.len());
        assert_eq!(digest.session_scope_count(), digest.sessions.len());
        assert!(sections.iter().any(|line| line == "By kind:"));
        assert!(sections.iter().any(|line| line == "By session:"));
        assert!(sections.iter().any(|line| line == "Recent events:"));
        assert!(sections.iter().any(|line| line.contains("SessionRenamed")));
        assert!(sections.iter().any(|line| line.contains("Alpha workspace")));
    }
