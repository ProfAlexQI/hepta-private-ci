#[tokio::test]
async fn record_context_updates_and_set_reference_context_item_with_source_aware_policy_prunes_extension_capabilities_prompt_fragments()
 {
    let (mut session, turn_context) = make_session_and_context().await;
    let mut previous_context_item = turn_context.to_turn_context_item();
    let previous_extension_capabilities_items = vec![ResponseItem::Message {
        id: None,
        role: "developer".to_string(),
        content: vec![ContentItem::InputText {
            text: ExtensionPromptFragment::new(
                ExtensionPromptSlot::DeveloperCapabilities,
                "Previously visible extension capabilities with repeated dispatch metadata, routing hints, tool affordances, and policy reminders.",
            )
            .render(),
        }],
        phase: None,
    }];
    previous_context_item.context_manifest =
        crate::context_manager::manifest::build_turn_context_manifest(
            &previous_extension_capabilities_items,
        );
    {
        let mut state = session.state.lock().await;
        state.set_reference_context_item(Some(previous_context_item));
    }
    let rollout_path = attach_thread_persistence(&mut session).await;
    let assembly_policy =
        crate::context_manager::manifest::ContextAssemblyPolicy::source_aware_tool_prune_for_model_context_window(
            Some(1),
        );

    session
        .record_context_updates_and_set_reference_context_item_with_policy(
            &turn_context,
            crate::context_manager::manifest::TurnContextManifestOptions::default(),
            assembly_policy,
        )
        .await;

    let current_history = session.clone_history().await;
    let pruned_capabilities_text = developer_input_texts(current_history.raw_items())
        .into_iter()
        .find(|text| text.starts_with(crate::context::EXTENSION_DEVELOPER_CAPABILITIES_OPEN_TAG))
        .expect("extension capabilities should remain as pruned prompt text");
    let current_context = session
        .reference_context_item()
        .await
        .expect("reference context item should be set");
    let current_manifest = current_context
        .context_manifest
        .as_ref()
        .expect("context manifest should be attached");
    let capabilities_entry = current_manifest
        .entries
        .iter()
        .find(|entry| entry.source.contains(":extension_developer_capabilities:"))
        .expect("extension capabilities should remain as a pruned manifest entry");
    let expected_text_hash = codex_protocol::protocol::stable_turn_context_manifest_text_hash(
        &format!("text:{pruned_capabilities_text}\n"),
    );

    assert_eq!(current_history.raw_items().len(), 1);
    assert!(current_manifest.has_replay_integrity());
    assert!(pruned_capabilities_text.contains("[context pruned for budget]"));
    assert!(
        !pruned_capabilities_text
            .contains("extension developer capabilities extension prompt fragments were cleared")
    );
    assert!(!pruned_capabilities_text.contains("Previously visible extension capabilities"));
    assert!(!current_manifest.truncated);
    assert_eq!(current_manifest.omitted_entries, 0);
    assert!(current_manifest.omitted_sources.is_empty());
    assert_eq!(current_manifest.compression_stages.len(), 1);
    assert_eq!(
        current_manifest.compression_stages[0].kind,
        codex_protocol::protocol::TurnContextCompressionStageKind::Prune
    );
    assert_eq!(
        current_manifest.compression_stages[0].output_tokens,
        capabilities_entry.estimated_tokens
    );
    assert!(current_manifest.compression_stages[0].tokens_saved() > 0);
    assert_eq!(capabilities_entry.text_hash, expected_text_hash);
    assert!(
        !current_manifest
            .compression_candidates
            .iter()
            .any(|candidate| candidate.source_id == "extension_developer_capabilities")
    );

    session.ensure_rollout_materialized().await;
    session.flush_rollout().await.expect("rollout should flush");

    let InitialHistory::Resumed(resumed) = RolloutRecorder::get_rollout_history(&rollout_path)
        .await
        .expect("read rollout history")
    else {
        panic!("expected resumed rollout history");
    };
    let persisted_manifest = resumed
        .history
        .iter()
        .filter_map(|item| match item {
            RolloutItem::TurnContext(ctx) => ctx.context_manifest.clone(),
            _ => None,
        })
        .next_back();
    let persisted_response_items = resumed
        .history
        .iter()
        .filter_map(|item| match item {
            RolloutItem::ResponseItem(item) => Some(item.clone()),
            _ => None,
        })
        .collect::<Vec<_>>();
    let persisted_capabilities_text = developer_input_texts(&persisted_response_items)
        .into_iter()
        .find(|text| text.starts_with(crate::context::EXTENSION_DEVELOPER_CAPABILITIES_OPEN_TAG))
        .expect("persisted history should retain pruned extension capabilities");

    assert_eq!(persisted_manifest.as_ref(), Some(current_manifest));
    assert_eq!(persisted_capabilities_text, pruned_capabilities_text);
}

#[tokio::test]
async fn record_context_updates_and_set_reference_context_item_with_source_aware_policy_compresses_summary_defragment_and_prune_together()
 {
    let (mut session, turn_context) = make_session_and_context().await;
    let mut previous_context_item = turn_context.to_turn_context_item();
    let previous_compressible_items = vec![ResponseItem::Message {
        id: None,
        role: "developer".to_string(),
        content: vec![
            ContentItem::InputText {
                text: tagged_context_fragment(
                    codex_protocol::protocol::PLUGINS_INSTRUCTIONS_OPEN_TAG,
                    codex_protocol::protocol::PLUGINS_INSTRUCTIONS_CLOSE_TAG,
                    "Previously visible plugins with repeated capability metadata for shell, files, docs, and search.",
                ),
            },
            ContentItem::InputText {
                text: ExtensionPromptFragment::new(
                    ExtensionPromptSlot::DeveloperCapabilities,
                    "Previously visible extension capabilities with repeated dispatch metadata, routing hints, tool affordances, and policy reminders.",
                )
                .render(),
            },
        ],
        phase: None,
    }];
    previous_context_item.context_manifest =
        crate::context_manager::manifest::build_turn_context_manifest(&previous_compressible_items);
    {
        let mut state = session.state.lock().await;
        state.set_reference_context_item(Some(previous_context_item));
    }
    let rollout_path = attach_thread_persistence(&mut session).await;
    let mut selected_snippets = test_recall_selected_snippet_envelope();
    selected_snippets.snippets[0].text =
        "bounded memory summary with repeated project context, recent decisions, durable recall notes, and handoff details"
            .into();
    let assembly_policy =
        crate::context_manager::manifest::ContextAssemblyPolicy::source_aware_compression_for_model_context_window(
            Some(1),
        );

    session
        .record_context_updates_and_set_reference_context_item_with_policy(
            &turn_context,
            crate::context_manager::manifest::TurnContextManifestOptions {
                recall_provider_rollup: None,
                recall_selected_snippets: Some(
                    crate::context_manager::manifest::ContextRecallSelectedSnippetEnvelope {
                        envelope: selected_snippets.clone(),
                    },
                ),
                memory_taxonomy: Vec::new(),
                memory_formation_receipts: Vec::new(),
                memory_temporal_facts: Vec::new(),
            },
            assembly_policy,
        )
        .await;

    let current_history = session.clone_history().await;
    let current_texts = developer_input_texts(current_history.raw_items());
    let summarized_recall_text = current_texts
        .iter()
        .find(|text| text.starts_with("<selected_context_recall>"))
        .expect("selected recall should remain as summarized prompt text");
    let defragmented_plugins_text = current_texts
        .iter()
        .find(|text| text.starts_with(codex_protocol::protocol::PLUGINS_INSTRUCTIONS_OPEN_TAG))
        .expect("available plugins should remain as defragmented prompt text");
    let pruned_capabilities_text = current_texts
        .iter()
        .find(|text| text.starts_with(crate::context::EXTENSION_DEVELOPER_CAPABILITIES_OPEN_TAG))
        .expect("extension capabilities should remain as pruned prompt text");
    let current_context = session
        .reference_context_item()
        .await
        .expect("reference context item should be set");
    let current_manifest = current_context
        .context_manifest
        .as_ref()
        .expect("context manifest should be attached");
    let selected_entry = current_manifest
        .entries
        .iter()
        .find(|entry| entry.source.contains(":selected_context_recall:"))
        .expect("selected recall should remain as a summarized manifest entry");
    let plugins_entry = current_manifest
        .entries
        .iter()
        .find(|entry| entry.source.contains(":available_plugins:"))
        .expect("available plugins should remain as a defragmented manifest entry");
    let capabilities_entry = current_manifest
        .entries
        .iter()
        .find(|entry| entry.source.contains(":extension_developer_capabilities:"))
        .expect("extension capabilities should remain as a pruned manifest entry");

    assert!(current_manifest.has_replay_integrity());
    assert_eq!(
        current_manifest.recall_selected_snippets.as_ref(),
        Some(&selected_snippets)
    );
    assert!(summarized_recall_text.contains("[context summarized for budget]"));
    assert!(defragmented_plugins_text.contains("[context defragmented for budget]"));
    assert!(pruned_capabilities_text.contains("[context pruned for budget]"));
    assert!(!summarized_recall_text.contains("bounded memory summary"));
    assert!(!defragmented_plugins_text.contains("Previously visible plugins"));
    assert!(!pruned_capabilities_text.contains("Previously visible extension capabilities"));
    assert!(!current_manifest.truncated);
    assert_eq!(current_manifest.omitted_entries, 0);
    assert!(current_manifest.omitted_sources.is_empty());
    assert_eq!(current_manifest.compression_stages.len(), 3);
    assert_eq!(
        current_manifest
            .compression_stages
            .iter()
            .map(|stage| stage.kind)
            .collect::<Vec<_>>(),
        vec![
            codex_protocol::protocol::TurnContextCompressionStageKind::Defragment,
            codex_protocol::protocol::TurnContextCompressionStageKind::Prune,
            codex_protocol::protocol::TurnContextCompressionStageKind::Summary,
        ]
    );
    assert!(
        current_manifest
            .compression_stages
            .iter()
            .all(|stage| { stage.affected_entries == 1 && stage.tokens_saved() > 0 })
    );
    assert_eq!(
        selected_entry.text_hash,
        codex_protocol::protocol::stable_turn_context_manifest_text_hash(&format!(
            "text:{summarized_recall_text}\n"
        ))
    );
    assert_eq!(
        plugins_entry.text_hash,
        codex_protocol::protocol::stable_turn_context_manifest_text_hash(&format!(
            "text:{defragmented_plugins_text}\n"
        ))
    );
    assert_eq!(
        capabilities_entry.text_hash,
        codex_protocol::protocol::stable_turn_context_manifest_text_hash(&format!(
            "text:{pruned_capabilities_text}\n"
        ))
    );
    assert!(
        !current_manifest
            .compression_candidates
            .iter()
            .any(|candidate| matches!(
                candidate.source_id.as_str(),
                "selected_context_recall"
                    | "available_plugins"
                    | "extension_developer_capabilities"
            ))
    );

    session.ensure_rollout_materialized().await;
    session.flush_rollout().await.expect("rollout should flush");

    let InitialHistory::Resumed(resumed) = RolloutRecorder::get_rollout_history(&rollout_path)
        .await
        .expect("read rollout history")
    else {
        panic!("expected resumed rollout history");
    };
    let persisted_manifest = resumed
        .history
        .iter()
        .filter_map(|item| match item {
            RolloutItem::TurnContext(ctx) => ctx.context_manifest.clone(),
            _ => None,
        })
        .next_back();
    let persisted_response_items = resumed
        .history
        .iter()
        .filter_map(|item| match item {
            RolloutItem::ResponseItem(item) => Some(item.clone()),
            _ => None,
        })
        .collect::<Vec<_>>();
    let persisted_texts = developer_input_texts(&persisted_response_items);

    assert_eq!(persisted_manifest.as_ref(), Some(current_manifest));
    assert!(persisted_texts.contains(summarized_recall_text));
    assert!(persisted_texts.contains(defragmented_plugins_text));
    assert!(persisted_texts.contains(pruned_capabilities_text));
}

#[tokio::test]
async fn record_context_updates_and_set_reference_context_item_honors_turn_scoped_source_aware_compression_opt_in()
 {
    let previous_compressible_items = || {
        vec![ResponseItem::Message {
            id: None,
            role: "developer".to_string(),
            content: vec![
                ContentItem::InputText {
                    text: tagged_context_fragment(
                        codex_protocol::protocol::PLUGINS_INSTRUCTIONS_OPEN_TAG,
                        codex_protocol::protocol::PLUGINS_INSTRUCTIONS_CLOSE_TAG,
                        "Previously visible plugins with repeated capability metadata for shell, files, docs, and search.",
                    ),
                },
                ContentItem::InputText {
                    text: ExtensionPromptFragment::new(
                        ExtensionPromptSlot::DeveloperCapabilities,
                        "Previously visible extension capabilities with repeated dispatch metadata, routing hints, tool affordances, and policy reminders.",
                    )
                    .render(),
                },
            ],
            phase: None,
        }]
    };
    let selected_snippets_for_budget_pressure = || {
        let mut selected_snippets = test_recall_selected_snippet_envelope();
        selected_snippets.snippets[0].text =
            "bounded memory summary with repeated project context, recent decisions, durable recall notes, and handoff details"
                .into();
        selected_snippets
    };
    fn assert_no_source_aware_compression_routing_metadata(rendered: &str) {
        assert!(!rendered.contains("TurnContextAssemblyPolicyOptIn"));
        assert!(!rendered.contains("SourceAwareCompression"));
        assert!(!rendered.contains("source_aware_compression_canary"));
    }

    let (mut baseline_session, mut baseline_context) = make_session_and_context().await;
    baseline_context.model_info.context_window = Some(1);
    baseline_context.model_info.effective_context_window_percent = 100;
    let mut previous_context_item = baseline_context.to_turn_context_item();
    previous_context_item.context_manifest =
        crate::context_manager::manifest::build_turn_context_manifest(
            &previous_compressible_items(),
        );
    {
        let mut state = baseline_session.state.lock().await;
        state.set_reference_context_item(Some(previous_context_item));
    }
    let baseline_selected_snippets = selected_snippets_for_budget_pressure();
    baseline_context
        .extension_data
        .insert(baseline_selected_snippets.clone());
    let baseline_rollout_path = attach_thread_persistence(&mut baseline_session).await;

    baseline_session
        .record_context_updates_and_set_reference_context_item(&baseline_context)
        .await;

    let baseline_history = baseline_session.clone_history().await;
    let baseline_history_json =
        serde_json::to_string(baseline_history.raw_items()).expect("history should serialize");
    let baseline_manifest = baseline_session
        .reference_context_item()
        .await
        .and_then(|item| item.context_manifest)
        .expect("baseline manifest should persist");

    assert!(baseline_manifest.has_replay_integrity());
    assert!(baseline_manifest.compression_stages.is_empty());
    assert_eq!(
        baseline_manifest
            .compression_candidates
            .iter()
            .map(|candidate| candidate.source_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "extension_developer_capabilities",
            "available_plugins",
            "selected_context_recall",
        ]
    );
    assert!(baseline_history_json.contains("Available plugins capability inventory was cleared"));
    assert!(
        baseline_history_json
            .contains("extension developer capabilities extension prompt fragments were cleared")
    );
    assert!(baseline_history_json.contains("bounded memory summary"));
    assert!(!baseline_history_json.contains("[context summarized for budget]"));
    assert!(!baseline_history_json.contains("[context defragmented for budget]"));
    assert!(!baseline_history_json.contains("[context pruned for budget]"));

    baseline_session.ensure_rollout_materialized().await;
    baseline_session
        .flush_rollout()
        .await
        .expect("baseline rollout should flush");

    let InitialHistory::Resumed(baseline_resumed) =
        RolloutRecorder::get_rollout_history(&baseline_rollout_path)
            .await
            .expect("read baseline rollout history")
    else {
        panic!("expected baseline resumed rollout history");
    };
    let baseline_persisted_manifest = baseline_resumed
        .history
        .iter()
        .filter_map(|item| match item {
            RolloutItem::TurnContext(ctx) => ctx.context_manifest.clone(),
            _ => None,
        })
        .next_back();
    let baseline_persisted_response_items = baseline_resumed
        .history
        .iter()
        .filter_map(|item| match item {
            RolloutItem::ResponseItem(item) => Some(item.clone()),
            _ => None,
        })
        .collect::<Vec<_>>();
    let baseline_persisted_json =
        serde_json::to_string(&baseline_persisted_response_items).expect("persisted serializes");
    let baseline_persisted_history_json =
        serde_json::to_string(&baseline_resumed.history).expect("history serializes");

    assert_eq!(
        baseline_persisted_manifest.as_ref(),
        Some(&baseline_manifest)
    );
    assert!(baseline_persisted_json.contains("bounded memory summary"));
    assert!(!baseline_persisted_json.contains("[context summarized for budget]"));
    assert_no_source_aware_compression_routing_metadata(&baseline_persisted_json);
    assert_no_source_aware_compression_routing_metadata(&baseline_persisted_history_json);

    let (mut marker_only_session, mut marker_only_context) = make_session_and_context().await;
    marker_only_context.model_info.context_window = Some(1);
    marker_only_context
        .model_info
        .effective_context_window_percent = 100;
    let mut previous_context_item = marker_only_context.to_turn_context_item();
    previous_context_item.context_manifest =
        crate::context_manager::manifest::build_turn_context_manifest(
            &previous_compressible_items(),
        );
    {
        let mut state = marker_only_session.state.lock().await;
        state.set_reference_context_item(Some(previous_context_item));
    }
    let marker_only_selected_snippets = selected_snippets_for_budget_pressure();
    marker_only_context
        .extension_data
        .insert(marker_only_selected_snippets.clone());
    crate::context_manager::manifest::insert_source_aware_compression_policy_opt_in_marker(
        marker_only_context.extension_data.as_ref(),
    );
    let marker_only_rollout_path = attach_thread_persistence(&mut marker_only_session).await;

    marker_only_session
        .record_context_updates_and_set_reference_context_item(&marker_only_context)
        .await;

    let marker_only_history = marker_only_session.clone_history().await;
    let marker_only_history_json =
        serde_json::to_string(marker_only_history.raw_items()).expect("history should serialize");
    let marker_only_manifest = marker_only_session
        .reference_context_item()
        .await
        .and_then(|item| item.context_manifest)
        .expect("marker-only manifest should persist");

    assert!(marker_only_manifest.has_replay_integrity());
    assert!(marker_only_manifest.compression_stages.is_empty());
    assert_eq!(
        marker_only_manifest
            .compression_candidates
            .iter()
            .map(|candidate| candidate.source_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "extension_developer_capabilities",
            "available_plugins",
            "selected_context_recall",
        ]
    );
    assert!(marker_only_history_json.contains("bounded memory summary"));
    assert!(!marker_only_history_json.contains("[context summarized for budget]"));
    assert!(!marker_only_history_json.contains("[context defragmented for budget]"));
    assert!(!marker_only_history_json.contains("[context pruned for budget]"));

    marker_only_session.ensure_rollout_materialized().await;
    marker_only_session
        .flush_rollout()
        .await
        .expect("marker-only rollout should flush");

    let InitialHistory::Resumed(marker_only_resumed) =
        RolloutRecorder::get_rollout_history(&marker_only_rollout_path)
            .await
            .expect("read marker-only rollout history")
    else {
        panic!("expected marker-only resumed rollout history");
    };
    let marker_only_persisted_manifest = marker_only_resumed
        .history
        .iter()
        .filter_map(|item| match item {
            RolloutItem::TurnContext(ctx) => ctx.context_manifest.clone(),
            _ => None,
        })
        .next_back();
    let marker_only_persisted_response_items = marker_only_resumed
        .history
        .iter()
        .filter_map(|item| match item {
            RolloutItem::ResponseItem(item) => Some(item.clone()),
            _ => None,
        })
        .collect::<Vec<_>>();
    let marker_only_persisted_json =
        serde_json::to_string(&marker_only_persisted_response_items).expect("persisted serializes");
    let marker_only_persisted_history_json =
        serde_json::to_string(&marker_only_resumed.history).expect("history serializes");

    assert_eq!(
        marker_only_persisted_manifest.as_ref(),
        Some(&marker_only_manifest)
    );
    assert!(marker_only_persisted_json.contains("bounded memory summary"));
    assert!(!marker_only_persisted_json.contains("[context summarized for budget]"));
    assert!(!marker_only_persisted_json.contains("[context defragmented for budget]"));
    assert!(!marker_only_persisted_json.contains("[context pruned for budget]"));
    assert_no_source_aware_compression_routing_metadata(&marker_only_persisted_json);
    assert_no_source_aware_compression_routing_metadata(&marker_only_persisted_history_json);

    let (mut feature_only_session, mut feature_only_context) = make_session_and_context().await;
    feature_only_session
        .features
        .enable(Feature::SourceAwareCompressionCanary)
        .expect("canary feature should enable");
    feature_only_context
        .features
        .enable(Feature::SourceAwareCompressionCanary)
        .expect("canary feature should enable");
    feature_only_context.model_info.context_window = Some(1);
    feature_only_context
        .model_info
        .effective_context_window_percent = 100;
    let mut previous_context_item = feature_only_context.to_turn_context_item();
    previous_context_item.context_manifest =
        crate::context_manager::manifest::build_turn_context_manifest(
            &previous_compressible_items(),
        );
    {
        let mut state = feature_only_session.state.lock().await;
        state.set_reference_context_item(Some(previous_context_item));
    }
    let feature_only_selected_snippets = selected_snippets_for_budget_pressure();
    feature_only_context
        .extension_data
        .insert(feature_only_selected_snippets.clone());
    let feature_only_rollout_path = attach_thread_persistence(&mut feature_only_session).await;

    feature_only_session
        .record_context_updates_and_set_reference_context_item(&feature_only_context)
        .await;

    let feature_only_history = feature_only_session.clone_history().await;
    let feature_only_history_json =
        serde_json::to_string(feature_only_history.raw_items()).expect("history should serialize");
    let feature_only_manifest = feature_only_session
        .reference_context_item()
        .await
        .and_then(|item| item.context_manifest)
        .expect("feature-only manifest should persist");

    assert!(feature_only_manifest.has_replay_integrity());
    assert!(feature_only_manifest.compression_stages.is_empty());
    assert_eq!(
        feature_only_manifest
            .compression_candidates
            .iter()
            .map(|candidate| candidate.source_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "extension_developer_capabilities",
            "available_plugins",
            "selected_context_recall",
        ]
    );
    assert!(feature_only_history_json.contains("bounded memory summary"));
    assert!(!feature_only_history_json.contains("[context summarized for budget]"));
    assert!(!feature_only_history_json.contains("[context defragmented for budget]"));
    assert!(!feature_only_history_json.contains("[context pruned for budget]"));

    feature_only_session.ensure_rollout_materialized().await;
    feature_only_session
        .flush_rollout()
        .await
        .expect("feature-only rollout should flush");

    let InitialHistory::Resumed(feature_only_resumed) =
        RolloutRecorder::get_rollout_history(&feature_only_rollout_path)
            .await
            .expect("read feature-only rollout history")
    else {
        panic!("expected feature-only resumed rollout history");
    };
    let feature_only_persisted_manifest = feature_only_resumed
        .history
        .iter()
        .filter_map(|item| match item {
            RolloutItem::TurnContext(ctx) => ctx.context_manifest.clone(),
            _ => None,
        })
        .next_back();
    let feature_only_persisted_response_items = feature_only_resumed
        .history
        .iter()
        .filter_map(|item| match item {
            RolloutItem::ResponseItem(item) => Some(item.clone()),
            _ => None,
        })
        .collect::<Vec<_>>();
    let feature_only_persisted_json = serde_json::to_string(&feature_only_persisted_response_items)
        .expect("persisted serializes");
    let feature_only_persisted_history_json =
        serde_json::to_string(&feature_only_resumed.history).expect("history serializes");

    assert_eq!(
        feature_only_persisted_manifest.as_ref(),
        Some(&feature_only_manifest)
    );
    assert!(feature_only_persisted_json.contains("bounded memory summary"));
    assert!(!feature_only_persisted_json.contains("[context summarized for budget]"));
    assert!(!feature_only_persisted_json.contains("[context defragmented for budget]"));
    assert!(!feature_only_persisted_json.contains("[context pruned for budget]"));
    assert_no_source_aware_compression_routing_metadata(&feature_only_persisted_json);
    assert_no_source_aware_compression_routing_metadata(&feature_only_persisted_history_json);

    let (mut canary_session, mut canary_context) = make_session_and_context().await;
    canary_session
        .features
        .enable(Feature::SourceAwareCompressionCanary)
        .expect("canary feature should enable");
    canary_context
        .features
        .enable(Feature::SourceAwareCompressionCanary)
        .expect("canary feature should enable");
    canary_context.model_info.context_window = Some(1);
    canary_context.model_info.effective_context_window_percent = 100;
    let mut previous_context_item = canary_context.to_turn_context_item();
    previous_context_item.context_manifest =
        crate::context_manager::manifest::build_turn_context_manifest(
            &previous_compressible_items(),
        );
    {
        let mut state = canary_session.state.lock().await;
        state.set_reference_context_item(Some(previous_context_item));
    }
    let canary_selected_snippets = selected_snippets_for_budget_pressure();
    canary_context
        .extension_data
        .insert(canary_selected_snippets.clone());
    crate::context_manager::manifest::insert_source_aware_compression_policy_opt_in_marker(
        canary_context.extension_data.as_ref(),
    );
    let canary_rollout_path = attach_thread_persistence(&mut canary_session).await;

    canary_session
        .record_context_updates_and_set_reference_context_item(&canary_context)
        .await;

    let canary_history = canary_session.clone_history().await;
    let canary_history_json =
        serde_json::to_string(canary_history.raw_items()).expect("history should serialize");
    let canary_manifest = canary_session
        .reference_context_item()
        .await
        .and_then(|item| item.context_manifest)
        .expect("canary manifest should persist");

    assert!(canary_manifest.has_replay_integrity());
    assert_eq!(canary_manifest.compression_stages.len(), 3);
    assert_eq!(
        canary_manifest
            .compression_stages
            .iter()
            .map(|stage| stage.kind)
            .collect::<Vec<_>>(),
        vec![
            codex_protocol::protocol::TurnContextCompressionStageKind::Defragment,
            codex_protocol::protocol::TurnContextCompressionStageKind::Prune,
            codex_protocol::protocol::TurnContextCompressionStageKind::Summary,
        ]
    );
    assert!(canary_manifest.compression_candidates.is_empty());
    assert!(canary_history_json.contains("[context summarized for budget]"));
    assert!(canary_history_json.contains("[context defragmented for budget]"));
    assert!(canary_history_json.contains("[context pruned for budget]"));
    assert!(!canary_history_json.contains("bounded memory summary"));
    assert!(!canary_history_json.contains("Available plugins capability inventory was cleared"));
    assert!(
        !canary_history_json
            .contains("extension developer capabilities extension prompt fragments were cleared")
    );

    canary_session.ensure_rollout_materialized().await;
    canary_session
        .flush_rollout()
        .await
        .expect("canary rollout should flush");

    let InitialHistory::Resumed(canary_resumed) =
        RolloutRecorder::get_rollout_history(&canary_rollout_path)
            .await
            .expect("read canary rollout history")
    else {
        panic!("expected canary resumed rollout history");
    };
    let canary_persisted_manifest = canary_resumed
        .history
        .iter()
        .filter_map(|item| match item {
            RolloutItem::TurnContext(ctx) => ctx.context_manifest.clone(),
            _ => None,
        })
        .next_back();
    let canary_persisted_response_items = canary_resumed
        .history
        .iter()
        .filter_map(|item| match item {
            RolloutItem::ResponseItem(item) => Some(item.clone()),
            _ => None,
        })
        .collect::<Vec<_>>();
    let canary_persisted_json =
        serde_json::to_string(&canary_persisted_response_items).expect("persisted serializes");
    let canary_persisted_history_json =
        serde_json::to_string(&canary_resumed.history).expect("history serializes");

    assert_eq!(canary_persisted_manifest.as_ref(), Some(&canary_manifest));
    assert!(canary_persisted_json.contains("[context summarized for budget]"));
    assert!(canary_persisted_json.contains("[context defragmented for budget]"));
    assert!(canary_persisted_json.contains("[context pruned for budget]"));
    assert!(!canary_persisted_json.contains("bounded memory summary"));
    assert_no_source_aware_compression_routing_metadata(&canary_persisted_json);
    assert_no_source_aware_compression_routing_metadata(&canary_persisted_history_json);
}

#[tokio::test]
async fn record_context_updates_and_set_reference_context_item_rejects_prompt_unsafe_selected_snippets_under_source_aware_compression_opt_in()
 {
    let previous_compressible_items = || {
        vec![ResponseItem::Message {
            id: None,
            role: "developer".to_string(),
            content: vec![
                ContentItem::InputText {
                    text: tagged_context_fragment(
                        codex_protocol::protocol::PLUGINS_INSTRUCTIONS_OPEN_TAG,
                        codex_protocol::protocol::PLUGINS_INSTRUCTIONS_CLOSE_TAG,
                        "Previously visible plugins with repeated capability metadata for shell, files, docs, and search.",
                    ),
                },
                ContentItem::InputText {
                    text: ExtensionPromptFragment::new(
                        ExtensionPromptSlot::DeveloperCapabilities,
                        "Previously visible extension capabilities with repeated dispatch metadata, routing hints, tool affordances, and policy reminders.",
                    )
                    .render(),
                },
            ],
            phase: None,
        }]
    };
    fn assert_no_source_aware_compression_routing_metadata(rendered: &str) {
        assert!(!rendered.contains("TurnContextAssemblyPolicyOptIn"));
        assert!(!rendered.contains("SourceAwareCompression"));
        assert!(!rendered.contains("source_aware_compression_canary"));
    }

    let (mut session, mut turn_context) = make_session_and_context().await;
    session
        .features
        .enable(Feature::SourceAwareCompressionCanary)
        .expect("canary feature should enable");
    turn_context
        .features
        .enable(Feature::SourceAwareCompressionCanary)
        .expect("canary feature should enable");
    turn_context.model_info.context_window = Some(1);
    turn_context.model_info.effective_context_window_percent = 100;
    let mut previous_context_item = turn_context.to_turn_context_item();
    previous_context_item.context_manifest =
        crate::context_manager::manifest::build_turn_context_manifest(
            &previous_compressible_items(),
        );
    {
        let mut state = session.state.lock().await;
        state.set_reference_context_item(Some(previous_context_item));
    }
    let unsafe_selected_snippet_payload =
        "source_id unsafe-selected-snippet-live-compression-bait should not reach prompt";
    let mut selected_snippets = test_recall_selected_snippet_envelope();
    selected_snippets.snippets[0].text = unsafe_selected_snippet_payload.into();
    assert!(selected_snippets.has_shadow_integrity());
    turn_context.extension_data.insert(selected_snippets);
    crate::context_manager::manifest::insert_source_aware_compression_policy_opt_in_marker(
        turn_context.extension_data.as_ref(),
    );

    session
        .record_context_updates_and_set_reference_context_item(&turn_context)
        .await;

    let history = session.clone_history().await;
    let history_json =
        serde_json::to_string(history.raw_items()).expect("history should serialize");
    let manifest = session
        .reference_context_item()
        .await
        .and_then(|item| item.context_manifest)
        .expect("manifest should persist under source-aware compression");

    assert!(manifest.has_replay_integrity());
    assert_eq!(manifest.recall_selected_snippets, None);
    assert_eq!(
        manifest
            .compression_stages
            .iter()
            .map(|stage| stage.kind)
            .collect::<Vec<_>>(),
        vec![
            codex_protocol::protocol::TurnContextCompressionStageKind::Defragment,
            codex_protocol::protocol::TurnContextCompressionStageKind::Prune,
        ]
    );
    assert!(history_json.contains("[context defragmented for budget]"));
    assert!(history_json.contains("[context pruned for budget]"));
    assert!(!history_json.contains("[context summarized for budget]"));
    assert!(!history_json.contains("<selected_context_recall>"));
    assert!(!history_json.contains("unsafe-selected-snippet-live-compression-bait"));
    assert_no_source_aware_compression_routing_metadata(&history_json);
}

#[tokio::test]
async fn record_context_updates_and_set_reference_context_item_dedupes_repeated_selected_snippets_without_context_diffs()
 {
    let (session, turn_context) = make_session_and_context().await;
    session
        .record_context_updates_and_set_reference_context_item(&turn_context)
        .await;
    let selected_snippets = test_recall_selected_snippet_envelope();
    let manifest_options = crate::context_manager::manifest::TurnContextManifestOptions {
        recall_provider_rollup: None,
        recall_selected_snippets: Some(
            crate::context_manager::manifest::ContextRecallSelectedSnippetEnvelope {
                envelope: selected_snippets.clone(),
            },
        ),
        memory_taxonomy: Vec::new(),
        memory_formation_receipts: Vec::new(),
        memory_temporal_facts: Vec::new(),
    };

    session
        .record_context_updates_and_set_reference_context_item_with_manifest_options(
            &turn_context,
            manifest_options.clone(),
        )
        .await;
    let history_after_first_selected = session.clone_history().await;
    let first_selected_manifest = session
        .reference_context_item()
        .await
        .and_then(|item| item.context_manifest)
        .expect("selected manifest should be retained");
    let retry_turn_context = session.new_default_turn().await;

    session
        .record_context_updates_and_set_reference_context_item_with_manifest_options(
            retry_turn_context.as_ref(),
            manifest_options,
        )
        .await;

    let current_history = session.clone_history().await;
    let current_manifest = session
        .reference_context_item()
        .await
        .and_then(|item| item.context_manifest)
        .expect("selected manifest should be retained after duplicate turn");
    let selected_context_recall_count = developer_input_texts(current_history.raw_items())
        .into_iter()
        .filter(|text| text.contains("<selected_context_recall>"))
        .count();

    assert_eq!(
        current_history.raw_items().len(),
        history_after_first_selected.raw_items().len()
    );
    assert_eq!(selected_context_recall_count, 1);
    assert_eq!(
        current_manifest.recall_selected_snippets.as_ref(),
        Some(&selected_snippets)
    );
    assert_eq!(
        current_manifest.ledger_hash,
        first_selected_manifest.ledger_hash
    );
}

#[tokio::test]
async fn record_context_updates_and_set_reference_context_item_consumes_turn_scoped_selected_snippets()
 {
    let (session, turn_context) = make_session_and_context().await;
    session
        .record_context_updates_and_set_reference_context_item(&turn_context)
        .await;
    let first_manifest = session
        .reference_context_item()
        .await
        .and_then(|item| item.context_manifest)
        .expect("initial context should attach manifest");
    let history_after_first = session.clone_history().await.raw_items().len();
    let selected_snippets = test_recall_selected_snippet_envelope();
    turn_context
        .extension_data
        .insert(selected_snippets.clone());

    session
        .record_context_updates_and_set_reference_context_item(&turn_context)
        .await;

    let current_manifest = session
        .reference_context_item()
        .await
        .and_then(|item| item.context_manifest)
        .expect("context manifest should be retained");
    let current_history = session.clone_history().await;
    let history_json =
        serde_json::to_string(current_history.raw_items()).expect("history should serialize");

    assert_eq!(current_history.raw_items().len(), history_after_first + 1);
    assert_eq!(
        current_manifest.recall_selected_snippets.as_ref(),
        Some(&selected_snippets)
    );
    assert_ne!(current_manifest.ledger_hash, first_manifest.ledger_hash);
    assert!(history_json.contains("<selected_context_recall>"));
    assert!(history_json.contains("fedcba9876543210"));
    assert!(history_json.contains("[redacted-query] bounded memory"));
    assert!(!history_json.contains("source-memory-id"));
    assert!(!history_json.contains("source_id"));
    assert!(!history_json.contains("[hepta-memory:"));
    assert!(!history_json.contains("needle"));
}

#[tokio::test]
async fn user_input_with_turn_context_selected_snippets_reach_guarded_live_handoff() {
    let (session, turn_context) = make_session_and_context().await;
    session
        .record_context_updates_and_set_reference_context_item(&turn_context)
        .await;
    let history_after_first = session.clone_history().await.raw_items().len();
    let request_turn_context = session.new_default_turn().await;
    let selected_snippets = test_recall_selected_snippet_envelope();

    handlers::attach_context_recall_selected_snippets_for_turn(
        request_turn_context.as_ref(),
        Some(selected_snippets.clone()),
    );
    session
        .record_context_updates_and_set_reference_context_item(request_turn_context.as_ref())
        .await;

    let current_manifest = session
        .reference_context_item()
        .await
        .and_then(|item| item.context_manifest)
        .expect("context manifest should be retained");
    let current_history = session.clone_history().await;
    let history_json =
        serde_json::to_string(current_history.raw_items()).expect("history should serialize");

    assert_eq!(current_history.raw_items().len(), history_after_first + 1);
    assert_eq!(
        current_manifest.recall_selected_snippets.as_ref(),
        Some(&selected_snippets)
    );
    assert!(history_json.contains("<selected_context_recall>"));
    assert!(history_json.contains("fedcba9876543210"));
    assert!(history_json.contains("[redacted-query] bounded memory"));
    assert!(!history_json.contains("source-memory-id"));
    assert!(!history_json.contains("source_id"));
    assert!(!history_json.contains("[hepta-memory:"));
    assert!(!history_json.contains("needle"));
}

#[tokio::test]
async fn user_input_with_turn_context_selected_snippets_reject_prompt_unsafe_payload() {
    let (session, turn_context) = make_session_and_context().await;
    session
        .record_context_updates_and_set_reference_context_item(&turn_context)
        .await;
    let first_manifest = session
        .reference_context_item()
        .await
        .and_then(|item| item.context_manifest)
        .expect("initial context should attach manifest");
    let history_after_first = session.clone_history().await.raw_items().len();
    let request_turn_context = session.new_default_turn().await;
    let mut selected_snippets = test_recall_selected_snippet_envelope();
    selected_snippets.snippets[0].text = "source_id leaked into request snippet".into();
    assert!(selected_snippets.has_shadow_integrity());

    handlers::attach_context_recall_selected_snippets_for_turn(
        request_turn_context.as_ref(),
        Some(selected_snippets),
    );
    session
        .record_context_updates_and_set_reference_context_item(request_turn_context.as_ref())
        .await;

    let current_manifest = session
        .reference_context_item()
        .await
        .and_then(|item| item.context_manifest)
        .expect("context manifest should be retained");
    let current_history = session.clone_history().await;
    let history_json =
        serde_json::to_string(current_history.raw_items()).expect("history should serialize");

    assert_eq!(current_history.raw_items().len(), history_after_first);
    assert_eq!(current_manifest.recall_selected_snippets, None);
    assert_eq!(current_manifest.ledger_hash, first_manifest.ledger_hash);
    assert!(!history_json.contains("source_id leaked into request snippet"));
}

#[tokio::test]
async fn record_context_updates_and_set_reference_context_item_ignores_invalid_turn_scoped_selected_snippets()
 {
    let (session, turn_context) = make_session_and_context().await;
    session
        .record_context_updates_and_set_reference_context_item(&turn_context)
        .await;
    let first_manifest = session
        .reference_context_item()
        .await
        .and_then(|item| item.context_manifest)
        .expect("initial context should attach manifest");
    let history_after_first = session.clone_history().await.raw_items().len();
    let mut selected_snippets = test_recall_selected_snippet_envelope();
    selected_snippets.selected_snippet_count = 2;
    turn_context.extension_data.insert(selected_snippets);

    session
        .record_context_updates_and_set_reference_context_item(&turn_context)
        .await;

    let current_manifest = session
        .reference_context_item()
        .await
        .and_then(|item| item.context_manifest)
        .expect("context manifest should be retained");

    assert_eq!(
        session.clone_history().await.raw_items().len(),
        history_after_first
    );
    assert_eq!(current_manifest.recall_selected_snippets, None);
    assert_eq!(current_manifest.ledger_hash, first_manifest.ledger_hash);
}

#[tokio::test]
async fn record_context_updates_and_set_reference_context_item_rejects_selected_snippets_with_forbidden_prompt_payload()
 {
    let (session, turn_context) = make_session_and_context().await;
    session
        .record_context_updates_and_set_reference_context_item(&turn_context)
        .await;
    let first_manifest = session
        .reference_context_item()
        .await
        .and_then(|item| item.context_manifest)
        .expect("initial context should attach manifest");
    let history_after_first = session.clone_history().await.raw_items().len();
    let mut selected_snippets = test_recall_selected_snippet_envelope();
    selected_snippets.snippets[0].text = "source_id leaked into bounded snippet".into();
    assert!(selected_snippets.has_shadow_integrity());

    session
        .record_context_updates_and_set_reference_context_item_with_manifest_options(
            &turn_context,
            crate::context_manager::manifest::TurnContextManifestOptions {
                recall_provider_rollup: None,
                recall_selected_snippets: Some(
                    crate::context_manager::manifest::ContextRecallSelectedSnippetEnvelope {
                        envelope: selected_snippets,
                    },
                ),
                memory_taxonomy: Vec::new(),
                memory_formation_receipts: Vec::new(),
                memory_temporal_facts: Vec::new(),
            },
        )
        .await;

    let current_manifest = session
        .reference_context_item()
        .await
        .and_then(|item| item.context_manifest)
        .expect("context manifest should be retained");
    let current_history = session.clone_history().await;
    let history_json =
        serde_json::to_string(current_history.raw_items()).expect("history should serialize");

    assert_eq!(current_history.raw_items().len(), history_after_first);
    assert_eq!(current_manifest.recall_selected_snippets, None);
    assert_eq!(current_manifest.ledger_hash, first_manifest.ledger_hash);
    assert!(!history_json.contains("source_id leaked into bounded snippet"));
}

fn test_recall_selected_snippet_envelope()
-> codex_protocol::protocol::TurnContextRecallSelectedSnippetEnvelope {
    codex_protocol::protocol::TurnContextRecallSelectedSnippetEnvelope {
        version: codex_protocol::protocol::TURN_CONTEXT_RECALL_SELECTED_SNIPPET_ENVELOPE_VERSION,
        max_snippets: 4,
        max_snippet_chars: 120,
        selected_snippet_count: 1,
        omitted_snippet_count: 2,
        redacted_snippet_count: 1,
        truncated_snippet_count: 0,
        snippets: vec![codex_protocol::protocol::TurnContextRecallSelectedSnippet {
            snippet_hash: "fedcba9876543210".into(),
            text: "[redacted-query] bounded memory".into(),
            estimated_tokens: 8,
            redacted: true,
            truncated: false,
        }],
        safety: codex_protocol::protocol::TurnContextRecallSelectedSnippetSafety {
            ready_for_shadow_handoff: true,
            bounded: true,
            origin_identifiers_exposed: false,
            raw_ranked_payload_exposed: false,
            rank_explanation_exposed: false,
            control_marker_exposed: false,
            query_payload_exposed: false,
            per_origin_list_exposed: false,
        },
    }
}

#[tokio::test]
async fn record_context_updates_and_set_reference_context_item_reinjects_full_context_after_clear()
{
    let (session, turn_context) = make_session_and_context().await;
    let compacted_summary = ResponseItem::Message {
        id: None,
        role: "user".to_string(),
        content: vec![ContentItem::InputText {
            text: format!("{}\nsummary", crate::compact::SUMMARY_PREFIX),
        }],
        phase: None,
    };
    session
        .record_into_history(std::slice::from_ref(&compacted_summary), &turn_context)
        .await;
    session
        .record_context_updates_and_set_reference_context_item(&turn_context)
        .await;
    {
        let mut state = session.state.lock().await;
        state.set_reference_context_item(/*item*/ None);
    }
    session
        .replace_history(
            vec![compacted_summary.clone()],
            /*reference_context_item*/ None,
        )
        .await;

    session
        .record_context_updates_and_set_reference_context_item(&turn_context)
        .await;

    let history = session.clone_history().await;
    let mut expected_history = vec![compacted_summary];
    expected_history.extend(session.build_initial_context(&turn_context).await);
    assert_eq!(history.raw_items().to_vec(), expected_history);
}

#[tokio::test]
async fn record_context_updates_and_set_reference_context_item_persists_baseline_without_emitting_diffs()
 {
    let (mut session, previous_context) = make_session_and_context().await;
    let next_model = if previous_context.model_info.slug == "gpt-5.4" {
        "gpt-5.2"
    } else {
        "gpt-5.4"
    };
    let turn_context = previous_context
        .with_model(next_model.to_string(), &session.services.models_manager)
        .await;
    let previous_context_item = previous_context.to_turn_context_item();
    {
        let mut state = session.state.lock().await;
        state.set_reference_context_item(Some(previous_context_item.clone()));
    }
    let rollout_path = attach_thread_persistence(&mut session).await;

    let update_items = session
        .build_settings_update_items(Some(&previous_context_item), &turn_context)
        .await;
    assert_eq!(update_items, Vec::new());

    session
        .record_context_updates_and_set_reference_context_item(&turn_context)
        .await;

    assert_eq!(
        session.clone_history().await.raw_items().to_vec(),
        Vec::new()
    );
    assert_eq!(
        serde_json::to_value(session.reference_context_item().await)
            .expect("serialize current context item"),
        serde_json::to_value(Some(turn_context.to_turn_context_item()))
            .expect("serialize expected context item")
    );
    session.ensure_rollout_materialized().await;
    session.flush_rollout().await.expect("rollout should flush");

    let InitialHistory::Resumed(resumed) = RolloutRecorder::get_rollout_history(&rollout_path)
        .await
        .expect("read rollout history")
    else {
        panic!("expected resumed rollout history");
    };
    let persisted_turn_context = resumed.history.iter().find_map(|item| match item {
        RolloutItem::TurnContext(ctx) => Some(ctx.clone()),
        _ => None,
    });
    assert_eq!(
        serde_json::to_value(persisted_turn_context)
            .expect("serialize persisted turn context item"),
        serde_json::to_value(Some(turn_context.to_turn_context_item()))
            .expect("serialize expected turn context item")
    );
}

#[tokio::test]
async fn record_context_updates_and_set_reference_context_item_persists_split_file_system_policy_to_rollout()
 {
    let (mut session, mut turn_context) = make_session_and_context().await;
    let file_system_sandbox_policy = file_system_policy_with_unreadable_glob(&turn_context);
    turn_context.permission_profile = PermissionProfile::from_runtime_permissions_with_enforcement(
        turn_context.permission_profile.enforcement(),
        &file_system_sandbox_policy,
        turn_context.network_sandbox_policy(),
    );
    let rollout_path = attach_thread_persistence(&mut session).await;

    session
        .record_context_updates_and_set_reference_context_item(&turn_context)
        .await;
    session.ensure_rollout_materialized().await;
    session.flush_rollout().await.expect("rollout should flush");

    let InitialHistory::Resumed(resumed) = RolloutRecorder::get_rollout_history(&rollout_path)
        .await
        .expect("read rollout history")
    else {
        panic!("expected resumed rollout history");
    };
    let persisted_file_system_sandbox_policy = resumed.history.iter().find_map(|item| match item {
        RolloutItem::TurnContext(ctx) => ctx.file_system_sandbox_policy.clone(),
        _ => None,
    });
    assert_eq!(
        persisted_file_system_sandbox_policy,
        Some(file_system_sandbox_policy)
    );
}

#[tokio::test]
async fn build_initial_context_prepends_model_switch_message() {
    let (session, turn_context) = make_session_and_context().await;
    let previous_turn_settings = PreviousTurnSettings {
        model: "previous-regular-model".to_string(),
        realtime_active: None,
    };

    session
        .set_previous_turn_settings(Some(previous_turn_settings))
        .await;
    let initial_context = session.build_initial_context(&turn_context).await;

    let ResponseItem::Message { role, content, .. } = &initial_context[0] else {
        panic!("expected developer message");
    };
    assert_eq!(role, "developer");
    let [ContentItem::InputText { text }, ..] = content.as_slice() else {
        panic!("expected developer text");
    };
    assert!(text.contains("<model_switch>"));
}

#[tokio::test]
async fn record_context_updates_and_set_reference_context_item_persists_full_reinjection_to_rollout()
 {
    let (mut session, previous_context) = make_session_and_context().await;
    let next_model = if previous_context.model_info.slug == "gpt-5.4" {
        "gpt-5.2"
    } else {
        "gpt-5.4"
    };
    let turn_context = previous_context
        .with_model(next_model.to_string(), &session.services.models_manager)
        .await;
    let rollout_path = attach_thread_persistence(&mut session).await;

    session
        .persist_rollout_items(&[RolloutItem::EventMsg(EventMsg::UserMessage(
            UserMessageEvent {
                message: "seed rollout".to_string(),
                images: None,
                local_images: Vec::new(),
                text_elements: Vec::new(),
                ..Default::default()
            },
        ))])
        .await;
    {
        let mut state = session.state.lock().await;
        state.set_reference_context_item(/*item*/ None);
    }

    session
        .set_previous_turn_settings(Some(PreviousTurnSettings {
            model: previous_context.model_info.slug.clone(),
            realtime_active: Some(previous_context.realtime_active),
        }))
        .await;
    let expected_context_items = session.build_initial_context(&turn_context).await;
    let mut expected_turn_context = turn_context.to_turn_context_item();
    let assembly_policy =
        crate::context_manager::manifest::ContextAssemblyPolicy::from_model_context_window(
            turn_context.model_context_window(),
        );
    expected_turn_context.context_manifest =
        crate::context_manager::manifest::build_turn_context_manifest_with_policy(
            &expected_context_items,
            &assembly_policy,
        );
    session
        .record_context_updates_and_set_reference_context_item(&turn_context)
        .await;
    session.ensure_rollout_materialized().await;
    session.flush_rollout().await.expect("rollout should flush");

    let InitialHistory::Resumed(resumed) = RolloutRecorder::get_rollout_history(&rollout_path)
        .await
        .expect("read rollout history")
    else {
        panic!("expected resumed rollout history");
    };
    let persisted_turn_context = resumed.history.iter().find_map(|item| match item {
        RolloutItem::TurnContext(ctx) => Some(ctx.clone()),
        _ => None,
    });

    assert_eq!(
        serde_json::to_value(persisted_turn_context)
            .expect("serialize persisted turn context item"),
        serde_json::to_value(Some(expected_turn_context))
            .expect("serialize expected turn context item")
    );
}

#[tokio::test]
async fn run_user_shell_command_does_not_set_reference_context_item() {
    let (session, _turn_context, rx) = make_session_and_context_with_rx().await;
    {
        let mut state = session.state.lock().await;
        state.set_reference_context_item(/*item*/ None);
    }

    handlers::run_user_shell_command(&session, "sub-id".to_string(), "echo shell".to_string())
        .await;

    let deadline = StdDuration::from_secs(15);
    let start = std::time::Instant::now();
    loop {
        let remaining = deadline.saturating_sub(start.elapsed());
        let evt = tokio::time::timeout(remaining, rx.recv())
            .await
            .expect("timeout waiting for event")
            .expect("event");
        if matches!(evt.msg, EventMsg::TurnComplete(_)) {
            break;
        }
    }

    assert!(
        session.reference_context_item().await.is_none(),
        "standalone shell tasks should not mutate previous context"
    );
}

#[tokio::test]
async fn realtime_conversation_list_voices_emits_builtin_list() {
    let (session, _turn_context, rx) = make_session_and_context_with_rx().await;

    handlers::realtime_conversation_list_voices(&session, "sub-id".to_string()).await;

    let event = rx.recv().await.expect("event");
    let voices = match event.msg {
        EventMsg::RealtimeConversationListVoicesResponse(
            RealtimeConversationListVoicesResponseEvent { voices },
        ) => voices,
        msg => panic!("expected list voices response, got {msg:?}"),
    };
    assert_eq!(
        voices,
        RealtimeVoicesList {
            v1: vec![
                RealtimeVoice::Juniper,
                RealtimeVoice::Maple,
                RealtimeVoice::Spruce,
                RealtimeVoice::Ember,
                RealtimeVoice::Vale,
                RealtimeVoice::Breeze,
                RealtimeVoice::Arbor,
                RealtimeVoice::Sol,
                RealtimeVoice::Cove,
            ],
            v2: vec![
                RealtimeVoice::Alloy,
                RealtimeVoice::Ash,
                RealtimeVoice::Ballad,
                RealtimeVoice::Coral,
                RealtimeVoice::Echo,
                RealtimeVoice::Sage,
                RealtimeVoice::Shimmer,
                RealtimeVoice::Verse,
                RealtimeVoice::Marin,
                RealtimeVoice::Cedar,
            ],
            default_v1: RealtimeVoice::Cove,
            default_v2: RealtimeVoice::Marin,
        },
    );
}

#[derive(Clone, Copy)]
struct NeverEndingTask {
    kind: TaskKind,
    listen_to_cancellation_token: bool,
}

impl SessionTask for NeverEndingTask {
    fn kind(&self) -> TaskKind {
        self.kind
    }

    fn span_name(&self) -> &'static str {
        "session_task.never_ending"
    }

    async fn run(
        self: Arc<Self>,
        _session: Arc<SessionTaskContext>,
        _ctx: Arc<TurnContext>,
        _input: Vec<UserInput>,
        cancellation_token: CancellationToken,
    ) -> Option<String> {
        if self.listen_to_cancellation_token {
            cancellation_token.cancelled().await;
            return None;
        }
        loop {
            sleep(Duration::from_secs(60)).await;
        }
    }
}

#[derive(Clone, Copy)]
struct GuardianDeniedApprovalTask;

impl SessionTask for GuardianDeniedApprovalTask {
    fn kind(&self) -> TaskKind {
        TaskKind::Regular
    }

    fn span_name(&self) -> &'static str {
        "session_task.guardian_denied_approval"
    }

    async fn run(
        self: Arc<Self>,
        session: Arc<SessionTaskContext>,
        ctx: Arc<TurnContext>,
        _input: Vec<UserInput>,
        cancellation_token: CancellationToken,
    ) -> Option<String> {
        let session = session.clone_session();
        for _ in 0..3 {
            crate::guardian::record_guardian_denial_for_test(&session, &ctx, &ctx.sub_id).await;
        }

        cancellation_token.cancelled().await;
        None
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn guardian_auto_review_interrupts_after_three_consecutive_denials() {
    let (sess, tc, rx) = make_session_and_context_with_rx().await;
    let input = vec![UserInput::Text {
        text: "trigger guardian denials".to_string(),
        text_elements: Vec::new(),
    }];
    sess.spawn_task(Arc::clone(&tc), input, GuardianDeniedApprovalTask)
        .await;

    let mut observed = Vec::new();
    let aborted = tokio::time::timeout(std::time::Duration::from_secs(5), async {
        loop {
            let event = rx.recv().await.expect("event");
            if let EventMsg::TurnAborted(event) = &event.msg {
                let event = event.clone();
                observed.push(EventMsg::TurnAborted(event.clone()));
                break event;
            }
            observed.push(event.msg);
        }
    })
    .await
    .unwrap_or_else(|_| {
        panic!(
            "guardian denial circuit breaker should interrupt the turn; observed events: {observed:?}"
        )
    });
    assert_eq!(aborted.reason, TurnAbortReason::Interrupted);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn guardian_helper_review_interrupts_after_three_consecutive_denials() {
    let (sess, tc, rx) = make_session_and_context_with_rx().await;
    let input = vec![UserInput::Text {
        text: "keep turn active for helper reviews".to_string(),
        text_elements: Vec::new(),
    }];
    sess.spawn_task(
        Arc::clone(&tc),
        input,
        NeverEndingTask {
            kind: TaskKind::Regular,
            listen_to_cancellation_token: true,
        },
    )
    .await;

    let session_for_review = Arc::clone(&sess);
    let turn_for_review = Arc::clone(&tc);
    let turn_id = tc.sub_id.clone();
    let review_thread = std::thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("helper review runtime");
        runtime.block_on(async move {
            for _ in 0..3 {
                crate::guardian::record_guardian_denial_for_test(
                    &session_for_review,
                    &turn_for_review,
                    &turn_id,
                )
                .await;
            }
        });
    });
    review_thread.join().expect("helper review thread");

    let mut observed = Vec::new();
    let aborted = timeout(StdDuration::from_secs(5), async {
        loop {
            let event = rx.recv().await.expect("event");
            if let EventMsg::TurnAborted(event) = &event.msg {
                let event = event.clone();
                observed.push(EventMsg::TurnAborted(event.clone()));
                break event;
            }
            observed.push(event.msg);
        }
    })
    .await
    .unwrap_or_else(|_| {
        panic!(
            "helper review circuit breaker should interrupt the turn; observed events: {observed:?}"
        )
    });
    assert_eq!(aborted.reason, TurnAbortReason::Interrupted);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[test_log::test]
async fn abort_regular_task_emits_turn_aborted_only() {
    let (sess, tc, rx) = make_session_and_context_with_rx().await;
    let input = vec![UserInput::Text {
        text: "hello".to_string(),
        text_elements: Vec::new(),
    }];
    sess.spawn_task(
        Arc::clone(&tc),
        input,
        NeverEndingTask {
            kind: TaskKind::Regular,
            listen_to_cancellation_token: false,
        },
    )
    .await;

    sess.abort_all_tasks(TurnAbortReason::Interrupted).await;

    // Interrupts persist a model-visible `<turn_aborted>` marker into history, but there is no
    // separate client-visible event for that marker (only `EventMsg::TurnAborted`).
    let evt = tokio::time::timeout(std::time::Duration::from_secs(2), rx.recv())
        .await
        .expect("timeout waiting for event")
        .expect("event");
    match evt.msg {
        EventMsg::TurnAborted(e) => assert_eq!(TurnAbortReason::Interrupted, e.reason),
        other => panic!("unexpected event: {other:?}"),
    }
    // No extra events should be emitted after an abort.
    assert!(rx.try_recv().is_err());
}

#[tokio::test]
async fn abort_gracefully_emits_turn_aborted_only() {
    let (sess, tc, rx) = make_session_and_context_with_rx().await;
    let input = vec![UserInput::Text {
        text: "hello".to_string(),
        text_elements: Vec::new(),
    }];
    sess.spawn_task(
        Arc::clone(&tc),
        input,
        NeverEndingTask {
            kind: TaskKind::Regular,
            listen_to_cancellation_token: true,
        },
    )
    .await;

    sess.abort_all_tasks(TurnAbortReason::Interrupted).await;

    // Even if tasks handle cancellation gracefully, interrupts still result in `TurnAborted`
    // being the only client-visible signal.
    let evt = tokio::time::timeout(std::time::Duration::from_secs(2), rx.recv())
        .await
        .expect("timeout waiting for event")
        .expect("event");
    match evt.msg {
        EventMsg::TurnAborted(e) => assert_eq!(TurnAbortReason::Interrupted, e.reason),
        other => panic!("unexpected event: {other:?}"),
    }
    // No extra events should be emitted after an abort.
    assert!(rx.try_recv().is_err());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn task_finish_emits_turn_item_lifecycle_for_leftover_pending_user_input() {
    let (sess, tc, rx) = make_session_and_context_with_rx().await;
    let input = vec![UserInput::Text {
        text: "hello".to_string(),
        text_elements: Vec::new(),
    }];
    sess.spawn_task(
        Arc::clone(&tc),
        input,
        NeverEndingTask {
            kind: TaskKind::Regular,
            listen_to_cancellation_token: false,
        },
    )
    .await;

    while rx.try_recv().is_ok() {}

    sess.inject_response_items(vec![ResponseInputItem::Message {
        role: "user".to_string(),
        content: vec![ContentItem::InputText {
            text: "late pending input".to_string(),
        }],
        phase: None,
    }])
    .await
    .expect("inject pending input into active turn");

    sess.on_task_finished(Arc::clone(&tc), /*last_agent_message*/ None)
        .await;

    let history = sess.clone_history().await;
    let expected = ResponseItem::Message {
        id: None,
        role: "user".to_string(),
        content: vec![ContentItem::InputText {
            text: "late pending input".to_string(),
        }],
        phase: None,
    };
    assert!(
        history.raw_items().iter().any(|item| item == &expected),
        "expected pending input to be persisted into history on turn completion"
    );

    let first = tokio::time::timeout(std::time::Duration::from_secs(2), rx.recv())
        .await
        .expect("expected raw response item event")
        .expect("channel open");
    assert!(matches!(first.msg, EventMsg::RawResponseItem(_)));

    let second = tokio::time::timeout(std::time::Duration::from_secs(2), rx.recv())
        .await
        .expect("expected item started event")
        .expect("channel open");
    assert!(matches!(
        second.msg,
        EventMsg::ItemStarted(ItemStartedEvent {
            item: TurnItem::UserMessage(UserMessageItem { content, .. }),
            ..
        }) if content == vec![UserInput::Text {
            text: "late pending input".to_string(),
            text_elements: Vec::new(),
        }]
    ));

    let third = tokio::time::timeout(std::time::Duration::from_secs(2), rx.recv())
        .await
        .expect("expected item completed event")
        .expect("channel open");
    assert!(matches!(
        third.msg,
        EventMsg::ItemCompleted(ItemCompletedEvent {
            item: TurnItem::UserMessage(UserMessageItem { content, .. }),
            ..
        }) if content == vec![UserInput::Text {
            text: "late pending input".to_string(),
            text_elements: Vec::new(),
        }]
    ));

    let fourth = tokio::time::timeout(std::time::Duration::from_secs(2), rx.recv())
        .await
        .expect("expected legacy user message event")
        .expect("channel open");
    assert!(matches!(
        fourth.msg,
        EventMsg::UserMessage(UserMessageEvent {
            message,
            images,
            text_elements,
            local_images,
            ..
        }) if message == "late pending input"
            && images == Some(Vec::new())
            && text_elements.is_empty()
            && local_images.is_empty()
    ));

    let fifth = tokio::time::timeout(std::time::Duration::from_secs(2), rx.recv())
        .await
        .expect("expected turn complete event")
        .expect("channel open");
    assert!(matches!(
        fifth.msg,
        EventMsg::TurnComplete(TurnCompleteEvent {
            error: None,
            turn_id,
            last_agent_message: None,
            time_to_first_token_ms: None,
            ..
        }) if turn_id == tc.sub_id
    ));
}

#[tokio::test]
async fn steer_input_requires_active_turn() {
    let (sess, _tc, _rx) = make_session_and_context_with_rx().await;
    let input = vec![UserInput::Text {
        text: "steer".to_string(),
        text_elements: Vec::new(),
    }];

    let err = sess
        .steer_input(
            input, /*expected_turn_id*/ None, /*responsesapi_client_metadata*/ None,
        )
        .await
        .expect_err("steering without active turn should fail");

    assert!(matches!(err, SteerInputError::NoActiveTurn(_)));
}

#[tokio::test]
async fn steer_input_enforces_expected_turn_id() {
    let (sess, tc, _rx) = make_session_and_context_with_rx().await;
    let input = vec![UserInput::Text {
        text: "hello".to_string(),
        text_elements: Vec::new(),
    }];
    sess.spawn_task(
        Arc::clone(&tc),
        input,
        NeverEndingTask {
            kind: TaskKind::Regular,
            listen_to_cancellation_token: false,
        },
    )
    .await;

    let steer_input = vec![UserInput::Text {
        text: "steer".to_string(),
        text_elements: Vec::new(),
    }];
    let err = sess
        .steer_input(
            steer_input,
            Some("different-turn-id"),
            /*responsesapi_client_metadata*/ None,
        )
        .await
        .expect_err("mismatched expected turn id should fail");

    match err {
        SteerInputError::ExpectedTurnMismatch { expected, actual } => {
            assert_eq!(
                (expected, actual),
                ("different-turn-id".to_string(), tc.sub_id.clone())
            );
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

#[tokio::test]
async fn steer_input_rejects_non_regular_turns() {
    for (task_kind, turn_kind) in [
        (TaskKind::Review, NonSteerableTurnKind::Review),
        (TaskKind::Compact, NonSteerableTurnKind::Compact),
    ] {
        let (sess, _tc, _rx) = make_session_and_context_with_rx().await;
        let input = vec![UserInput::Text {
            text: "hello".to_string(),
            text_elements: Vec::new(),
        }];
        let turn_context = sess.new_default_turn_with_sub_id("turn".to_string()).await;
        sess.spawn_task(
            turn_context,
            input,
            NeverEndingTask {
                kind: task_kind,
                listen_to_cancellation_token: true,
            },
        )
        .await;

        let steer_input = vec![UserInput::Text {
            text: "steer".to_string(),
            text_elements: Vec::new(),
        }];
        let err = sess
            .steer_input(
                steer_input,
                /*expected_turn_id*/ None,
                /*responsesapi_client_metadata*/ None,
            )
            .await
            .expect_err("steering a non-regular turn should fail");

        assert_eq!(err, SteerInputError::ActiveTurnNotSteerable { turn_kind });

        sess.abort_all_tasks(TurnAbortReason::Interrupted).await;
    }
}

#[tokio::test]
async fn steer_input_returns_active_turn_id() {
    let (sess, tc, _rx) = make_session_and_context_with_rx().await;
    let input = vec![UserInput::Text {
        text: "hello".to_string(),
        text_elements: Vec::new(),
    }];
    sess.spawn_task(
        Arc::clone(&tc),
        input,
        NeverEndingTask {
            kind: TaskKind::Regular,
            listen_to_cancellation_token: false,
        },
    )
    .await;

    let steer_input = vec![UserInput::Text {
        text: "steer".to_string(),
        text_elements: Vec::new(),
    }];
    let turn_id = sess
        .steer_input(
            steer_input,
            Some(&tc.sub_id),
            /*responsesapi_client_metadata*/ None,
        )
        .await
        .expect("steering with matching expected turn id should succeed");

    assert_eq!(turn_id, tc.sub_id);
    assert!(sess.has_pending_input().await);
}

#[tokio::test]
async fn prepend_pending_input_keeps_older_tail_ahead_of_newer_input() {
    let (sess, tc, _rx) = make_session_and_context_with_rx().await;
    let input = vec![UserInput::Text {
        text: "hello".to_string(),
        text_elements: Vec::new(),
    }];
    sess.spawn_task(
        Arc::clone(&tc),
        input,
        NeverEndingTask {
            kind: TaskKind::Regular,
            listen_to_cancellation_token: false,
        },
    )
    .await;

    let blocked = ResponseInputItem::Message {
        role: "user".to_string(),
        content: vec![ContentItem::InputText {
            text: "blocked queued prompt".to_string(),
        }],
        phase: None,
    };
    let later = ResponseInputItem::Message {
        role: "user".to_string(),
        content: vec![ContentItem::InputText {
            text: "later queued prompt".to_string(),
        }],
        phase: None,
    };
    let newer = ResponseInputItem::Message {
        role: "user".to_string(),
        content: vec![ContentItem::InputText {
            text: "newer queued prompt".to_string(),
        }],
        phase: None,
    };

    sess.inject_response_items(vec![blocked.clone(), later.clone()])
        .await
        .expect("inject initial pending input into active turn");

    let drained = sess.get_pending_input().await;
    assert_eq!(drained, vec![blocked, later.clone()]);

    sess.inject_response_items(vec![newer.clone()])
        .await
        .expect("inject newer pending input into active turn");

    let mut drained_iter = drained.into_iter();
    let _blocked = drained_iter.next().expect("blocked prompt should exist");
    sess.prepend_pending_input(drained_iter.collect())
        .await
        .expect("requeue later pending input at the front of the queue");

    assert_eq!(sess.get_pending_input().await, vec![later, newer]);
}

#[tokio::test]
async fn queued_response_items_for_next_turn_move_into_next_active_turn() {
    let (sess, tc, _rx) = make_session_and_context_with_rx().await;
    let queued_item = ResponseInputItem::Message {
        role: "assistant".to_string(),
        content: vec![ContentItem::InputText {
            text: "queued before wake".to_string(),
        }],
        phase: None,
    };

    sess.queue_response_items_for_next_turn(vec![queued_item.clone()])
        .await;

    sess.spawn_task(
        Arc::clone(&tc),
        Vec::new(),
        NeverEndingTask {
            kind: TaskKind::Regular,
            listen_to_cancellation_token: false,
        },
    )
    .await;

    assert_eq!(sess.get_pending_input().await, vec![queued_item]);
}

#[tokio::test]
async fn idle_interrupt_does_not_wake_queued_next_turn_items() {
    let (sess, _tc, _rx) = make_session_and_context_with_rx().await;
    let queued_item = ResponseInputItem::Message {
        role: "assistant".to_string(),
        content: vec![ContentItem::InputText {
            text: "queued before interrupt".to_string(),
        }],
        phase: None,
    };

    sess.queue_response_items_for_next_turn(vec![queued_item])
        .await;

    sess.abort_all_tasks(TurnAbortReason::Interrupted).await;

    assert!(sess.active_turn.lock().await.is_none());
    assert!(sess.has_queued_response_items_for_next_turn().await);
}

#[tokio::test]
async fn abort_empty_active_turn_preserves_pending_input() {
    let (sess, _tc, _rx) = make_session_and_context_with_rx().await;
    let pending_item = ResponseInputItem::Message {
        role: "user".to_string(),
        content: vec![ContentItem::InputText {
            text: "late pending input".to_string(),
        }],
        phase: None,
    };
    let turn_state = {
        let mut active = sess.active_turn.lock().await;
        let active_turn = active.get_or_insert_with(ActiveTurn::default);
        Arc::clone(&active_turn.turn_state)
    };
    turn_state
        .lock()
        .await
        .push_pending_input(pending_item.clone());

    sess.abort_all_tasks(TurnAbortReason::Replaced).await;

    assert!(sess.active_turn.lock().await.is_none());
    assert_eq!(
        turn_state.lock().await.take_pending_input(),
        vec![pending_item]
    );
}

#[tokio::test]
async fn interrupt_accounts_active_goal_before_pausing() -> anyhow::Result<()> {
    let (sess, tc, _rx, _codex_home) = make_goal_session_and_context_with_rx().await;
    sess.set_thread_goal(
        tc.as_ref(),
        SetGoalRequest {
            objective: Some("Keep improving the benchmark".to_string()),
            status: None,
            token_budget: None,
        },
    )
    .await?;

    sess.spawn_task(
        Arc::clone(&tc),
        Vec::new(),
        NeverEndingTask {
            kind: TaskKind::Regular,
            listen_to_cancellation_token: false,
        },
    )
    .await;
    set_total_token_usage(&sess, post_goal_token_usage()).await;

    sess.abort_all_tasks(TurnAbortReason::Interrupted).await;

    let goal = sess
        .get_thread_goal()
        .await?
        .expect("goal should remain persisted after interrupt");
    assert_eq!(
        codex_protocol::protocol::ThreadGoalStatus::Paused,
        goal.status
    );
    assert_eq!(70, goal.tokens_used);

    assert!(sess.active_turn.lock().await.is_none());

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn active_goal_continuation_runs_again_after_no_tool_turn() -> anyhow::Result<()> {
    let server = start_mock_server().await;
    let mut builder = test_codex().with_config(|config| {
        config
            .features
            .enable(Feature::Goals)
            .expect("goal mode should be enableable in tests");
    });
    let test = builder.build(&server).await?;
    let responses = mount_sse_sequence(
        &server,
        vec![
            sse(vec![
                ev_response_created("resp-1"),
                ev_function_call(
                    "call-create-goal",
                    "create_goal",
                    r#"{"objective":"write a benchmark note"}"#,
                ),
                ev_completed("resp-1"),
            ]),
            sse(vec![
                ev_assistant_message("msg-1", "Draft ready."),
                ev_completed("resp-2"),
            ]),
            sse(vec![
                ev_assistant_message("msg-2", "I am still working on the benchmark note."),
                ev_completed("resp-3"),
            ]),
            sse(vec![
                ev_response_created("resp-4"),
                ev_function_call(
                    "call-complete-goal",
                    "update_goal",
                    r#"{"status":"complete"}"#,
                ),
                ev_completed("resp-4"),
            ]),
            sse(vec![
                ev_assistant_message("msg-3", "Goal complete."),
                ev_completed("resp-5"),
            ]),
        ],
    )
    .await;

    test.codex
        .submit(Op::UserInput {
            environments: None,
            items: vec![UserInput::Text {
                text: "write a benchmark note".into(),
                text_elements: Vec::new(),
            }],
            final_output_json_schema: None,
            responsesapi_client_metadata: None,
        })
        .await?;

    let mut completed_turns = 0;
    tokio::time::timeout(std::time::Duration::from_secs(8), async {
        loop {
            let event = test.codex.next_event().await?;
            if matches!(event.msg, EventMsg::TurnComplete(_)) {
                completed_turns += 1;
                if completed_turns == 3 {
                    return anyhow::Ok(());
                }
            }
        }
    })
    .await??;

    let continuation_request = responses
        .requests()
        .into_iter()
        .find(|request| request.body_contains_text("<goal_context>"))
        .expect("expected a goal continuation request");
    let body = continuation_request.body_json();
    let goal_context_message = body["input"]
        .as_array()
        .expect("input should be an array")
        .iter()
        .find(|item| item.to_string().contains("<goal_context>"))
        .expect("goal context message should be present");
    assert_eq!(goal_context_message["role"].as_str(), Some("user"));
    assert!(
        goal_context_message
            .to_string()
            .contains("Continue working toward the active thread goal.")
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn pending_request_user_input_does_not_spawn_extra_goal_continuation() -> anyhow::Result<()> {
    let server = start_mock_server().await;
    let mut builder = test_codex().with_config(|config| {
        config
            .features
            .enable(Feature::Goals)
            .expect("goal mode should be enableable in tests");
        config
            .features
            .enable(Feature::DefaultModeRequestUserInput)
            .expect("default-mode request_user_input should be enableable in tests");
    });
    let test = builder.build(&server).await?;
    let responses = mount_sse_sequence(
        &server,
        vec![
            sse(vec![
                ev_response_created("resp-1"),
                ev_function_call(
                    "call-create-goal",
                    "create_goal",
                    r#"{"objective":"write a benchmark note"}"#,
                ),
                ev_completed("resp-1"),
            ]),
            sse(vec![
                ev_assistant_message("msg-1", "Draft ready."),
                ev_completed("resp-2"),
            ]),
            sse(vec![
                ev_response_created("resp-3"),
                ev_function_call(
                    "call-ask-user",
                    "request_user_input",
                    r#"{"questions":[{"header":"Choice","id":"next_step","question":"Pick one","options":[{"label":"Outline","description":"Start with an outline."},{"label":"Draft","description":"Write a full draft."}]}]}"#,
                ),
                ev_completed("resp-3"),
            ]),
            sse(vec![
                ev_response_created("resp-4"),
                ev_function_call(
                    "call-complete-goal",
                    "update_goal",
                    r#"{"status":"complete"}"#,
                ),
                ev_completed("resp-4"),
            ]),
            sse(vec![
                ev_assistant_message("msg-2", "Goal complete."),
                ev_completed("resp-5"),
            ]),
        ],
    )
    .await;

    test.codex
        .submit(Op::UserInput {
            environments: None,
            items: vec![UserInput::Text {
                text: "write a benchmark note".into(),
                text_elements: Vec::new(),
            }],
            final_output_json_schema: None,
            responsesapi_client_metadata: None,
        })
        .await?;

    let request_user_input_event = wait_for_event_match(&test.codex, |event| match event {
        EventMsg::RequestUserInput(event) => Some(event.clone()),
        _ => None,
    })
    .await;
    assert_eq!(3, responses.requests().len());
    assert!(
        timeout(Duration::from_millis(200), test.codex.next_event())
            .await
            .is_err(),
        "waiting for request_user_input should keep the turn open without emitting more events"
    );
    assert_eq!(
        3,
        responses.requests().len(),
        "waiting for request_user_input should not start another continuation request"
    );

    test.codex
        .submit(Op::UserInputAnswer {
            id: request_user_input_event.turn_id,
            response: RequestUserInputResponse {
                answers: std::collections::HashMap::from([(
                    "next_step".to_string(),
                    RequestUserInputAnswer {
                        answers: vec!["Outline".to_string()],
                    },
                )]),
            },
        })
        .await?;

    let mut completed_turns = 0;
    timeout(Duration::from_secs(8), async {
        loop {
            let event = test.codex.next_event().await?;
            if matches!(event.msg, EventMsg::TurnComplete(_)) {
                completed_turns += 1;
                if completed_turns == 1 {
                    return anyhow::Ok(());
                }
            }
        }
    })
    .await??;

    assert_eq!(5, responses.requests().len());

    Ok(())
}

async fn set_total_token_usage(sess: &Session, total_token_usage: TokenUsage) {
    let mut state = sess.state.lock().await;
    state.set_token_info(Some(TokenUsageInfo {
        total_token_usage,
        last_token_usage: TokenUsage::default(),
        model_context_window: None,
    }));
}

fn post_goal_token_usage() -> TokenUsage {
    TokenUsage {
        input_tokens: 50,
        cached_input_tokens: 10,
        output_tokens: 30,
        reasoning_output_tokens: 5,
        total_tokens: 75,
    }
}

async fn goal_test_state_db(sess: &Session) -> anyhow::Result<crate::StateDbHandle> {
    if let Some(state_db) = sess.state_db() {
        return Ok(state_db);
    }
    let config = sess.get_config().await;
    codex_state::StateRuntime::init(config.sqlite_home.clone(), config.model_provider_id.clone())
        .await
}

#[tokio::test]
async fn budget_limited_accounting_steers_active_turn_without_aborting() -> anyhow::Result<()> {
    let (sess, tc, rx, _codex_home) = make_goal_session_and_context_with_rx().await;
    sess.set_thread_goal(
        tc.as_ref(),
        SetGoalRequest {
            objective: Some("Keep improving the benchmark".to_string()),
            status: None,
            token_budget: Some(Some(10)),
        },
    )
    .await?;
    sess.goal_runtime_apply(GoalRuntimeEvent::TurnStarted {
        turn_context: tc.as_ref(),
        token_usage: TokenUsage::default(),
    })
    .await?;
    sess.spawn_task(
        Arc::clone(&tc),
        Vec::new(),
        NeverEndingTask {
            kind: TaskKind::Regular,
            listen_to_cancellation_token: false,
        },
    )
    .await;
    while rx.try_recv().is_ok() {}

    set_total_token_usage(
        &sess,
        TokenUsage {
            input_tokens: 20,
            cached_input_tokens: 0,
            output_tokens: 5,
            reasoning_output_tokens: 0,
            total_tokens: 25,
        },
    )
    .await;

    sess.goal_runtime_apply(GoalRuntimeEvent::ToolCompleted {
        turn_context: tc.as_ref(),
        tool_name: "shell_command",
    })
    .await?;

    let pending_input = sess.get_pending_input().await;
    let [ResponseInputItem::Message { role, content, .. }] = pending_input.as_slice() else {
        panic!("expected one budget-limit steering message, got {pending_input:#?}");
    };
    assert_eq!("user", role);
    let [ContentItem::InputText { text }] = content.as_slice() else {
        panic!("expected one text span in budget-limit steering message, got {content:#?}");
    };
    assert!(text.starts_with("<goal_context>"));
    assert!(text.trim_end().ends_with("</goal_context>"));
    assert!(text.contains("budget_limited"));
    assert!(text.to_lowercase().contains("wrap up this turn soon"));
    assert!(sess.active_turn.lock().await.is_some());
    while let Ok(event) = rx.try_recv() {
        assert!(
            !matches!(event.msg, EventMsg::TurnAborted(_)),
            "budget limit should steer the active turn instead of aborting it"
        );
    }

    let state_db = goal_test_state_db(sess.as_ref()).await?;
    let goal = state_db
        .get_thread_goal(sess.conversation_id)
        .await?
        .expect("goal should remain persisted after accounting");
    assert_eq!(codex_state::ThreadGoalStatus::BudgetLimited, goal.status);
    assert_eq!(25, goal.tokens_used);

    set_total_token_usage(
        &sess,
        TokenUsage {
            input_tokens: 30,
            cached_input_tokens: 0,
            output_tokens: 10,
            reasoning_output_tokens: 0,
            total_tokens: 40,
        },
    )
    .await;
    sess.goal_runtime_apply(GoalRuntimeEvent::ToolCompletedGoal {
        turn_context: tc.as_ref(),
    })
    .await?;

    let goal = state_db
        .get_thread_goal(sess.conversation_id)
        .await?
        .expect("goal should remain persisted after follow-up accounting");
    assert_eq!(codex_state::ThreadGoalStatus::BudgetLimited, goal.status);
    assert_eq!(40, goal.tokens_used);

    sess.abort_all_tasks(TurnAbortReason::Interrupted).await;

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn external_goal_mutation_accounts_active_turn_before_status_change() -> anyhow::Result<()> {
    let (sess, tc, _rx, _codex_home) = make_goal_session_and_context_with_rx().await;
    sess.set_thread_goal(
        tc.as_ref(),
        SetGoalRequest {
            objective: Some("Keep improving the benchmark".to_string()),
            status: None,
            token_budget: None,
        },
    )
    .await?;
    sess.spawn_task(
        Arc::clone(&tc),
        Vec::new(),
        NeverEndingTask {
            kind: TaskKind::Regular,
            listen_to_cancellation_token: false,
        },
    )
    .await;
    set_total_token_usage(&sess, post_goal_token_usage()).await;

    sess.goal_runtime_apply(GoalRuntimeEvent::ExternalMutationStarting)
        .await?;

    let state_db = goal_test_state_db(sess.as_ref()).await?;
    let goal = state_db
        .get_thread_goal(sess.conversation_id)
        .await?
        .expect("goal should remain persisted");
    assert_eq!(70, goal.tokens_used);

    let previous_goal = goal.clone();
    let goal_id = goal.goal_id.clone();
    let updated_goal = state_db
        .update_thread_goal(
            sess.conversation_id,
            codex_state::ThreadGoalUpdate {
                objective: None,
                status: Some(codex_state::ThreadGoalStatus::Complete),
                token_budget: None,
                expected_goal_id: Some(goal_id),
            },
        )
        .await?
        .expect("goal status update should succeed");
    sess.goal_runtime_apply(GoalRuntimeEvent::ExternalSet {
        external_set: ExternalGoalSet {
            goal: updated_goal,
            previous_status: ExternalGoalPreviousStatus::from(&previous_goal),
        },
    })
    .await?;

    assert!(sess.active_turn.lock().await.is_some());
    let goal = state_db
        .get_thread_goal(sess.conversation_id)
        .await?
        .expect("goal should remain persisted");
    assert_eq!(codex_state::ThreadGoalStatus::Complete, goal.status);
    assert_eq!(70, goal.tokens_used);

    sess.abort_all_tasks(TurnAbortReason::Replaced).await;

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn external_objective_change_steers_active_turn() -> anyhow::Result<()> {
    let (sess, tc, _rx, _codex_home) = make_goal_session_and_context_with_rx().await;
    sess.spawn_task(
        Arc::clone(&tc),
        Vec::new(),
        NeverEndingTask {
            kind: TaskKind::Regular,
            listen_to_cancellation_token: false,
        },
    )
    .await;

    let state_db = goal_test_state_db(sess.as_ref()).await?;
    let old_goal = state_db
        .replace_thread_goal(
            sess.conversation_id,
            "Keep improving the benchmark",
            codex_state::ThreadGoalStatus::Active,
            /*token_budget*/ Some(10_000),
        )
        .await?;
    let new_goal = state_db
        .replace_thread_goal(
            sess.conversation_id,
            "Write a concise benchmark summary",
            codex_state::ThreadGoalStatus::Active,
            /*token_budget*/ Some(10_000),
        )
        .await?;

    sess.goal_runtime_apply(GoalRuntimeEvent::ExternalSet {
        external_set: ExternalGoalSet {
            goal: new_goal,
            previous_status: ExternalGoalPreviousStatus::from(&old_goal),
        },
    })
    .await?;

    let pending_input = sess.get_pending_input().await;
    assert!(
        pending_input.iter().any(|item| {
            matches!(
                item,
                ResponseInputItem::Message { role, content, .. }
                    if role == "user"
                        && content.iter().any(|content| matches!(
                            content,
                            ContentItem::InputText { text }
                                if text.starts_with("<goal_context>")
                                    && text.trim_end().ends_with("</goal_context>")
                                    && text.contains("The active thread goal objective was edited")
                                    && text.contains("Write a concise benchmark summary")
                        ))
            )
        }),
        "expected objective-updated steering prompt in pending input: {pending_input:?}"
    );

    sess.abort_all_tasks(TurnAbortReason::Replaced).await;

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn external_active_goal_set_marks_current_turn_for_accounting() -> anyhow::Result<()> {
    let (sess, tc, _rx, _codex_home) = make_goal_session_and_context_with_rx().await;
    sess.spawn_task(
        Arc::clone(&tc),
        Vec::new(),
        NeverEndingTask {
            kind: TaskKind::Regular,
            listen_to_cancellation_token: false,
        },
    )
    .await;
    set_total_token_usage(&sess, post_goal_token_usage()).await;

    let state_db = goal_test_state_db(sess.as_ref()).await?;
    let goal = state_db
        .replace_thread_goal(
            sess.conversation_id,
            "Keep improving the benchmark",
            codex_state::ThreadGoalStatus::Active,
            /*token_budget*/ None,
        )
        .await?;
    sess.goal_runtime_apply(GoalRuntimeEvent::ExternalSet {
        external_set: ExternalGoalSet {
            goal,
            previous_status: ExternalGoalPreviousStatus::NewGoal,
        },
    })
    .await?;

    set_total_token_usage(
        &sess,
        TokenUsage {
            input_tokens: 65,
            cached_input_tokens: 10,
            output_tokens: 40,
            reasoning_output_tokens: 5,
            total_tokens: 110,
        },
    )
    .await;
    sess.goal_runtime_apply(GoalRuntimeEvent::ToolCompleted {
        turn_context: tc.as_ref(),
        tool_name: "shell_command",
    })
    .await?;

    let goal = state_db
        .get_thread_goal(sess.conversation_id)
        .await?
        .expect("goal should remain persisted");
    assert_eq!(codex_state::ThreadGoalStatus::Active, goal.status);
    assert_eq!(25, goal.tokens_used);

    sess.abort_all_tasks(TurnAbortReason::Replaced).await;

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn completed_goal_accounts_current_turn_tokens_before_tool_response() -> anyhow::Result<()> {
    let server = start_mock_server().await;
    let mut builder = test_codex().with_config(|config| {
        config
            .features
            .enable(Feature::Goals)
            .expect("goal mode should be enableable in tests");
    });
    let test = builder.build(&server).await?;
    let responses = mount_sse_sequence(
        &server,
        vec![
            sse(vec![
                ev_response_created("resp-1"),
                ev_function_call(
                    "call-create-goal",
                    "create_goal",
                    r#"{"objective":"write a report","token_budget":500}"#,
                ),
                ev_completed("resp-1"),
            ]),
            sse(vec![
                ev_response_created("resp-2"),
                ev_function_call(
                    "call-complete-goal",
                    "update_goal",
                    r#"{"status":"complete"}"#,
                ),
                ev_completed_with_tokens("resp-2", /*total_tokens*/ 580),
            ]),
            sse(vec![
                ev_assistant_message("msg-1", "Goal complete."),
                ev_completed("resp-3"),
            ]),
        ],
    )
    .await;

    test.codex
        .submit(Op::UserInput {
            environments: None,
            items: vec![UserInput::Text {
                text: "write a report".into(),
                text_elements: Vec::new(),
            }],
            final_output_json_schema: None,
            responsesapi_client_metadata: None,
        })
        .await?;

    tokio::time::timeout(std::time::Duration::from_secs(8), async {
        loop {
            let event = test.codex.next_event().await?;
            if matches!(event.msg, EventMsg::TurnComplete(_)) {
                return anyhow::Ok(());
            }
        }
    })
    .await??;

    let complete_output = responses
        .function_call_output_text("call-complete-goal")
        .expect("complete tool output should be sent to the model");
    let complete_output: serde_json::Value = serde_json::from_str(&complete_output)?;
    assert_eq!(complete_output["goal"]["tokensUsed"], 580);
    assert_eq!(complete_output["goal"]["status"], "complete");
    assert_eq!(complete_output["remainingTokens"], 0);
    assert_eq!(
        complete_output["completionBudgetReport"],
        "Goal achieved. Report final budget usage to the user: tokens used: 580 of 500."
    );
    let requests = responses.requests();
    let completion_followup_request = requests
        .last()
        .expect("completion tool output should be sent in a follow-up request");
    assert!(
        !completion_followup_request.body_contains_text("budget_limited"),
        "completion follow-up should not include budget-limit steering"
    );

    let state_db = codex_state::StateRuntime::init(
        test.config.sqlite_home.clone(),
        test.config.model_provider_id.clone(),
    )
    .await?;
    let persisted_goal = state_db
        .get_thread_goal(test.session_configured.thread_id)
        .await?
        .expect("goal should be persisted");
    assert_eq!(
        codex_state::ThreadGoalStatus::Complete,
        persisted_goal.status
    );
    assert_eq!(580, persisted_goal.tokens_used);

    Ok(())
}

#[tokio::test]
async fn queue_only_mailbox_mail_waits_for_next_turn_after_answer_boundary() {
    let (sess, tc, _rx) = make_session_and_context_with_rx().await;
    let communication = InterAgentCommunication::new(
        AgentPath::try_from("/root/worker").expect("worker path should parse"),
        AgentPath::root(),
        Vec::new(),
        "late queue-only update".to_string(),
        /*trigger_turn*/ false,
    );
    sess.spawn_task(
        Arc::clone(&tc),
        Vec::new(),
        NeverEndingTask {
            kind: TaskKind::Regular,
            listen_to_cancellation_token: true,
        },
    )
    .await;

    sess.defer_mailbox_delivery_to_next_turn(&tc.sub_id).await;
    sess.enqueue_mailbox_communication(communication.clone());

    assert!(
        !sess.has_pending_input().await,
        "queue-only mailbox mail should stay buffered once the current turn emitted its answer"
    );
    assert_eq!(sess.get_pending_input().await, Vec::new());

    sess.abort_all_tasks(TurnAbortReason::Replaced).await;

    assert_eq!(
        sess.get_pending_input().await,
        vec![communication.to_response_input_item()],
    );
}

#[tokio::test]
async fn trigger_turn_mailbox_mail_waits_for_next_turn_after_answer_boundary() {
    let (sess, tc, _rx) = make_session_and_context_with_rx().await;
    sess.spawn_task(
        Arc::clone(&tc),
        Vec::new(),
        NeverEndingTask {
            kind: TaskKind::Regular,
            listen_to_cancellation_token: true,
        },
    )
    .await;

    sess.defer_mailbox_delivery_to_next_turn(&tc.sub_id).await;
    sess.enqueue_mailbox_communication(InterAgentCommunication::new(
        AgentPath::try_from("/root/worker").expect("worker path should parse"),
        AgentPath::root(),
        Vec::new(),
        "late trigger update".to_string(),
        /*trigger_turn*/ true,
    ));

    assert!(
        !sess.has_pending_input().await,
        "trigger-turn mailbox mail should not extend the current turn after its answer boundary"
    );

    sess.abort_all_tasks(TurnAbortReason::Replaced).await;

    assert!(sess.has_trigger_turn_mailbox_items().await);
}

#[tokio::test]
async fn steered_input_reopens_mailbox_delivery_for_current_turn() {
    let (sess, tc, _rx) = make_session_and_context_with_rx().await;
    let communication = InterAgentCommunication::new(
        AgentPath::try_from("/root/worker").expect("worker path should parse"),
        AgentPath::root(),
        Vec::new(),
        "queued child update".to_string(),
        /*trigger_turn*/ false,
    );
    sess.spawn_task(
        Arc::clone(&tc),
        Vec::new(),
        NeverEndingTask {
            kind: TaskKind::Regular,
            listen_to_cancellation_token: true,
        },
    )
    .await;

    sess.defer_mailbox_delivery_to_next_turn(&tc.sub_id).await;
    sess.enqueue_mailbox_communication(communication.clone());
    sess.steer_input(
        vec![UserInput::Text {
            text: "follow up".to_string(),
            text_elements: Vec::new(),
        }],
        Some(&tc.sub_id),
        /*responsesapi_client_metadata*/ None,
    )
    .await
    .expect("steered input should be accepted");

    assert_eq!(
        sess.get_pending_input().await,
        vec![
            ResponseInputItem::from(vec![UserInput::Text {
                text: "follow up".to_string(),
                text_elements: Vec::new(),
            }]),
            communication.to_response_input_item(),
        ],
    );
}

#[tokio::test]
async fn stale_defer_mailbox_delivery_does_not_override_steered_input() {
    let (sess, tc, _rx) = make_session_and_context_with_rx().await;
    let communication = InterAgentCommunication::new(
        AgentPath::try_from("/root/worker").expect("worker path should parse"),
        AgentPath::root(),
        Vec::new(),
        "queued child update".to_string(),
        /*trigger_turn*/ false,
    );
    sess.spawn_task(
        Arc::clone(&tc),
        Vec::new(),
        NeverEndingTask {
            kind: TaskKind::Regular,
            listen_to_cancellation_token: true,
        },
    )
    .await;

    sess.defer_mailbox_delivery_to_next_turn(&tc.sub_id).await;
    sess.enqueue_mailbox_communication(communication.clone());
    sess.steer_input(
        vec![UserInput::Text {
            text: "follow up".to_string(),
            text_elements: Vec::new(),
        }],
        Some(&tc.sub_id),
        /*responsesapi_client_metadata*/ None,
    )
    .await
    .expect("steered input should be accepted");

    sess.defer_mailbox_delivery_to_next_turn(&tc.sub_id).await;

    assert_eq!(
        sess.get_pending_input().await,
        vec![
            ResponseInputItem::from(vec![UserInput::Text {
                text: "follow up".to_string(),
                text_elements: Vec::new(),
            }]),
            communication.to_response_input_item(),
        ],
    );
}

#[tokio::test]
async fn tool_calls_reopen_mailbox_delivery_for_current_turn() {
    let (sess, tc, _rx) = make_session_and_context_with_rx().await;
    let communication = InterAgentCommunication::new(
        AgentPath::try_from("/root/worker").expect("worker path should parse"),
        AgentPath::root(),
        Vec::new(),
        "queued child update".to_string(),
        /*trigger_turn*/ false,
    );
    sess.spawn_task(
        Arc::clone(&tc),
        Vec::new(),
        NeverEndingTask {
            kind: TaskKind::Regular,
            listen_to_cancellation_token: true,
        },
    )
    .await;

    sess.defer_mailbox_delivery_to_next_turn(&tc.sub_id).await;
    sess.enqueue_mailbox_communication(communication.clone());

    let item = ResponseItem::FunctionCall {
        id: None,
        name: "test_tool".to_string(),
        namespace: None,
        arguments: "{}".to_string(),
        encrypted_function_args: None,
        call_id: "call-1".to_string(),
    };
    let mut ctx = HandleOutputCtx {
        sess: Arc::clone(&sess),
        turn_context: Arc::clone(&tc),
        turn_store: Arc::new(codex_extension_api::ExtensionData::new(tc.sub_id.clone())),
        tool_runtime: test_tool_runtime(Arc::clone(&sess), Arc::clone(&tc)),
        cancellation_token: CancellationToken::new(),
    };

    let output = handle_output_item_done(&mut ctx, item, /*previously_active_item*/ None)
        .await
        .expect("tool call should be handled");

    assert!(output.needs_follow_up);
    assert!(output.tool_future.is_some());
    assert_eq!(
        sess.get_pending_input().await,
        vec![communication.to_response_input_item()],
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn abort_review_task_emits_exited_then_aborted_and_records_history() {
    let (sess, tc, rx) = make_session_and_context_with_rx().await;
    let input = vec![UserInput::Text {
        text: "start review".to_string(),
        text_elements: Vec::new(),
    }];
    sess.spawn_task(Arc::clone(&tc), input, ReviewTask::new())
        .await;

    sess.abort_all_tasks(TurnAbortReason::Interrupted).await;

    // Aborting a review task should exit review mode before surfacing the abort to the client.
    // We scan for these events (rather than relying on fixed ordering) since unrelated events
    // may interleave.
    let mut exited_review_mode_idx = None;
    let mut turn_aborted_idx = None;
    let mut idx = 0usize;
    let deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(3);
    while tokio::time::Instant::now() < deadline {
        let remaining = deadline.saturating_duration_since(tokio::time::Instant::now());
        let evt = tokio::time::timeout(remaining, rx.recv())
            .await
            .expect("timeout waiting for event")
            .expect("event");
        let event_idx = idx;
        idx = idx.saturating_add(1);
        match evt.msg {
            EventMsg::ExitedReviewMode(ev) => {
                assert!(ev.review_output.is_none());
                exited_review_mode_idx = Some(event_idx);
            }
            EventMsg::TurnAborted(ev) => {
                assert_eq!(TurnAbortReason::Interrupted, ev.reason);
                turn_aborted_idx = Some(event_idx);
                break;
            }
            _ => {}
        }
    }
    assert!(
        exited_review_mode_idx.is_some(),
        "expected ExitedReviewMode after abort"
    );
    assert!(
        turn_aborted_idx.is_some(),
        "expected TurnAborted after abort"
    );
    assert!(
        exited_review_mode_idx.unwrap() < turn_aborted_idx.unwrap(),
        "expected ExitedReviewMode before TurnAborted"
    );

    let history = sess.clone_history().await;
    // The `<turn_aborted>` marker is silent in the event stream, so verify it is still
    // recorded in history for the model.
    assert!(
        history.raw_items().iter().any(|item| {
            let ResponseItem::Message { role, content, .. } = item else {
                return false;
            };
            if role != "user" {
                return false;
            }
            content.iter().any(|content_item| {
                let ContentItem::InputText { text } = content_item else {
                    return false;
                };
                TurnAborted::matches_text(text)
            })
        }),
        "expected a model-visible turn aborted marker in history after interrupt"
    );
}

#[tokio::test]
#[expect(
    clippy::await_holding_invalid_type,
    reason = "test builds a router from session-owned MCP manager state"
)]
async fn fatal_tool_error_stops_turn_and_reports_error() {
    let (session, turn_context, _rx) = make_session_and_context_with_rx().await;
    let tools = {
        session
            .services
            .mcp_connection_manager
            .read()
            .await
            .list_all_tools()
            .await
    };
    let deferred_mcp_tools = Some(tools.clone());
    let router = ToolRouter::from_config(
        &turn_context.tools_config,
        crate::tools::router::ToolRouterParams {
            deferred_mcp_tools,
            mcp_tools: Some(tools),
            discoverable_tools: None,
            extension_tool_executors: Vec::new(),
            dynamic_tools: turn_context.dynamic_tools.as_slice(),
        },
    );
    let item = ResponseItem::CustomToolCall {
        id: None,
        status: None,
        call_id: "call-1".to_string(),
        name: "shell_command".to_string(),
        input: "{}".to_string(),
    };

    let call = ToolRouter::build_tool_call(item.clone())
        .expect("build tool call")
        .expect("tool call present");
    let tracker = Arc::new(tokio::sync::Mutex::new(TurnDiffTracker::new()));
    let err = router
        .dispatch_tool_call_with_code_mode_result(
            Arc::clone(&session),
            Arc::clone(&turn_context),
            CancellationToken::new(),
            tracker,
            call,
            ToolCallSource::Direct,
        )
        .await
        .err()
        .expect("expected fatal error");

    match err {
        FunctionCallError::Fatal(message) => {
            assert_eq!(
                message,
                "tool shell_command invoked with incompatible payload"
            );
        }
        other => panic!("expected FunctionCallError::Fatal, got {other:?}"),
    }
}

async fn sample_rollout(
    session: &Session,
    _turn_context: &TurnContext,
) -> (Vec<RolloutItem>, Vec<ResponseItem>) {
    let mut rollout_items = Vec::new();
    let mut live_history = ContextManager::new();

    // Use the same turn_context source as record_initial_history so model_info (and thus
    // personality_spec) matches reconstruction.
    let reconstruction_turn = session.new_default_turn().await;
    let mut initial_context = session
        .build_initial_context(reconstruction_turn.as_ref())
        .await;
    // Ensure personality_spec is present when Personality is enabled, so expected matches
    // what reconstruction produces (build_initial_context may omit it when baked into model).
    if !initial_context.iter().any(|m| {
        matches!(m, ResponseItem::Message { role, content, .. }
        if role == "developer"
            && content.iter().any(|c| {
                matches!(c, ContentItem::InputText { text } if text.contains("<personality_spec>"))
            }))
    }) && let Some(p) = reconstruction_turn.personality
        && session.features.enabled(Feature::Personality)
        && let Some(personality_message) = reconstruction_turn
            .model_info
            .model_messages
            .as_ref()
            .and_then(|m| m.get_personality_message(Some(p)).filter(|s| !s.is_empty()))
    {
        let msg = crate::context::ContextualUserFragment::into(
            crate::context::PersonalitySpecInstructions::new(personality_message),
        );
        let insert_at = initial_context
            .iter()
            .position(|m| matches!(m, ResponseItem::Message { role, .. } if role == "developer"))
            .map(|i| i + 1)
            .unwrap_or(0);
        initial_context.insert(insert_at, msg);
    }
    for item in &initial_context {
        rollout_items.push(RolloutItem::ResponseItem(item.clone()));
    }
    live_history.record_items(
        initial_context.iter(),
        reconstruction_turn.truncation_policy,
    );

    let user1 = ResponseItem::Message {
        id: None,
        role: "user".to_string(),
        content: vec![ContentItem::InputText {
            text: "first user".to_string(),
        }],
        phase: None,
    };
    live_history.record_items(
        std::iter::once(&user1),
        reconstruction_turn.truncation_policy,
    );
    rollout_items.push(RolloutItem::ResponseItem(user1.clone()));

    let assistant1 = ResponseItem::Message {
        id: None,
        role: "assistant".to_string(),
        content: vec![ContentItem::OutputText {
            text: "assistant reply one".to_string(),
        }],
        phase: None,
    };
    live_history.record_items(
        std::iter::once(&assistant1),
        reconstruction_turn.truncation_policy,
    );
    rollout_items.push(RolloutItem::ResponseItem(assistant1.clone()));

    let summary1 = "summary one";
    let snapshot1 = live_history
        .clone()
        .for_prompt(&reconstruction_turn.model_info.input_modalities);
    let user_messages1 = collect_user_messages(&snapshot1);
    let rebuilt1 = compact::build_compacted_history(Vec::new(), &user_messages1, summary1);
    live_history.replace(rebuilt1);
    rollout_items.push(RolloutItem::Compacted(CompactedItem {
        message: summary1.to_string(),
        replacement_history: None,
    }));

    let user2 = ResponseItem::Message {
        id: None,
        role: "user".to_string(),
        content: vec![ContentItem::InputText {
            text: "second user".to_string(),
        }],
        phase: None,
    };
    live_history.record_items(
        std::iter::once(&user2),
        reconstruction_turn.truncation_policy,
    );
    rollout_items.push(RolloutItem::ResponseItem(user2.clone()));

    let assistant2 = ResponseItem::Message {
        id: None,
        role: "assistant".to_string(),
        content: vec![ContentItem::OutputText {
            text: "assistant reply two".to_string(),
        }],
        phase: None,
    };
    live_history.record_items(
        std::iter::once(&assistant2),
        reconstruction_turn.truncation_policy,
    );
    rollout_items.push(RolloutItem::ResponseItem(assistant2.clone()));

    let summary2 = "summary two";
    let snapshot2 = live_history
        .clone()
        .for_prompt(&reconstruction_turn.model_info.input_modalities);
    let user_messages2 = collect_user_messages(&snapshot2);
    let rebuilt2 = compact::build_compacted_history(Vec::new(), &user_messages2, summary2);
    live_history.replace(rebuilt2);
    rollout_items.push(RolloutItem::Compacted(CompactedItem {
        message: summary2.to_string(),
        replacement_history: None,
    }));

    let user3 = ResponseItem::Message {
        id: None,
        role: "user".to_string(),
        content: vec![ContentItem::InputText {
            text: "third user".to_string(),
        }],
        phase: None,
    };
    live_history.record_items(
        std::iter::once(&user3),
        reconstruction_turn.truncation_policy,
    );
    rollout_items.push(RolloutItem::ResponseItem(user3));

    let assistant3 = ResponseItem::Message {
        id: None,
        role: "assistant".to_string(),
        content: vec![ContentItem::OutputText {
            text: "assistant reply three".to_string(),
        }],
        phase: None,
    };
    live_history.record_items(
        std::iter::once(&assistant3),
        reconstruction_turn.truncation_policy,
    );
    rollout_items.push(RolloutItem::ResponseItem(assistant3));

    (
        rollout_items,
        live_history.for_prompt(&reconstruction_turn.model_info.input_modalities),
    )
}

#[tokio::test]
async fn create_goal_tool_rejects_existing_goal() {
    let (session, turn_context, _rx, _codex_home) = make_goal_session_and_context_with_rx().await;
    let tracker = Arc::new(tokio::sync::Mutex::new(TurnDiffTracker::new()));
    let handler = CreateGoalHandler;

    handler
        .handle(ToolInvocation {
            session: Arc::clone(&session),
            turn: Arc::clone(&turn_context),
            cancellation_token: CancellationToken::new(),
            tracker: Arc::clone(&tracker),
            call_id: "create-goal-1".to_string(),
            tool_name: codex_tools::ToolName::plain("create_goal"),
            source: ToolCallSource::Direct,
            payload: ToolPayload::Function {
                arguments: serde_json::json!({
                    "objective": "Keep the watcher alive",
                    "token_budget": 123,
                })
                .to_string(),
            },
        })
        .await
        .expect("initial create_goal should succeed");

    let response = handler
        .handle(ToolInvocation {
            session: Arc::clone(&session),
            turn: Arc::clone(&turn_context),
            cancellation_token: CancellationToken::new(),
            tracker,
            call_id: "create-goal-2".to_string(),
            tool_name: codex_tools::ToolName::plain("create_goal"),
            source: ToolCallSource::Direct,
            payload: ToolPayload::Function {
                arguments: serde_json::json!({
                    "objective": "Replace the watcher",
                    "token_budget": 456,
                })
                .to_string(),
            },
        })
        .await;

    let Err(FunctionCallError::RespondToModel(output)) = response else {
        panic!("expected create_goal to reject an existing goal");
    };
    assert_eq!(
        output,
        "cannot create a new goal because this thread already has a goal; use update_goal only when the existing goal is complete"
    );

    let goal = session
        .get_thread_goal()
        .await
        .expect("read thread goal")
        .expect("goal should still exist");
    assert_eq!(goal.objective, "Keep the watcher alive");
    assert_eq!(goal.token_budget, Some(123));
}

#[tokio::test]
async fn update_goal_tool_rejects_pausing_goal() {
    let (session, turn_context, _rx, _codex_home) = make_goal_session_and_context_with_rx().await;
    let tracker = Arc::new(tokio::sync::Mutex::new(TurnDiffTracker::new()));
    let create_handler = CreateGoalHandler;
    let update_handler = UpdateGoalHandler;

    create_handler
        .handle(ToolInvocation {
            session: Arc::clone(&session),
            turn: Arc::clone(&turn_context),
            cancellation_token: CancellationToken::new(),
            tracker: Arc::clone(&tracker),
            call_id: "create-goal".to_string(),
            tool_name: codex_tools::ToolName::plain("create_goal"),
            source: ToolCallSource::Direct,
            payload: ToolPayload::Function {
                arguments: serde_json::json!({
                    "objective": "Keep the watcher alive",
                    "token_budget": 123,
                })
                .to_string(),
            },
        })
        .await
        .expect("initial create_goal should succeed");

    let response = update_handler
        .handle(ToolInvocation {
            session: Arc::clone(&session),
            turn: Arc::clone(&turn_context),
            cancellation_token: CancellationToken::new(),
            tracker,
            call_id: "pause-goal".to_string(),
            tool_name: codex_tools::ToolName::plain("update_goal"),
            source: ToolCallSource::Direct,
            payload: ToolPayload::Function {
                arguments: serde_json::json!({
                    "status": "paused",
                })
                .to_string(),
            },
        })
        .await;

    let Err(FunctionCallError::RespondToModel(output)) = response else {
        panic!("expected update_goal to reject pausing a goal");
    };
    assert_eq!(
        output,
        "update_goal can only mark the existing goal complete; pause, resume, and budget-limited status changes are controlled by the user or system"
    );

    let goal = session
        .get_thread_goal()
        .await
        .expect("read thread goal")
        .expect("goal should still exist");
    assert_eq!(goal.status, ThreadGoalStatus::Active);
}

#[tokio::test]
async fn update_goal_tool_marks_goal_complete() {
    let (session, turn_context, _rx, _codex_home) = make_goal_session_and_context_with_rx().await;
    let tracker = Arc::new(tokio::sync::Mutex::new(TurnDiffTracker::new()));
    let create_handler = CreateGoalHandler;
    let update_handler = UpdateGoalHandler;

    create_handler
        .handle(ToolInvocation {
            session: Arc::clone(&session),
            turn: Arc::clone(&turn_context),
            cancellation_token: CancellationToken::new(),
            tracker: Arc::clone(&tracker),
            call_id: "create-goal".to_string(),
            tool_name: codex_tools::ToolName::plain("create_goal"),
            source: ToolCallSource::Direct,
            payload: ToolPayload::Function {
                arguments: serde_json::json!({
                    "objective": "Keep the watcher alive",
                    "token_budget": 123,
                })
                .to_string(),
            },
        })
        .await
        .expect("initial create_goal should succeed");

    update_handler
        .handle(ToolInvocation {
            session: Arc::clone(&session),
            turn: Arc::clone(&turn_context),
            cancellation_token: CancellationToken::new(),
            tracker,
            call_id: "complete-goal".to_string(),
            tool_name: codex_tools::ToolName::plain("update_goal"),
            source: ToolCallSource::Direct,
            payload: ToolPayload::Function {
                arguments: serde_json::json!({
                    "status": "complete",
                })
                .to_string(),
            },
        })
        .await
        .expect("update_goal should mark the goal complete");

    let goal = session
        .get_thread_goal()
        .await
        .expect("read thread goal")
        .expect("goal should still exist");
    assert_eq!(goal.status, ThreadGoalStatus::Complete);
}

#[tokio::test]
async fn rejects_escalated_permissions_when_policy_not_on_request() {
    use crate::exec_policy::ExecApprovalRequest;
    use crate::sandboxing::SandboxPermissions;
    use crate::tools::sandboxing::ExecApprovalRequirement;
    use crate::turn_diff_tracker::TurnDiffTracker;
    use codex_protocol::protocol::AskForApproval;
    use codex_tools::ShellCommandBackendConfig;

    let (session, mut turn_context_raw) = make_session_and_context().await;
    // Ensure policy is NOT OnRequest so the early rejection path triggers
    turn_context_raw
        .approval_policy
        .set(AskForApproval::OnFailure)
        .expect("test setup should allow updating approval policy");
    let session = Arc::new(session);
    let mut turn_context = Arc::new(turn_context_raw);

    let command_script = "echo hi";
    let timeout_ms = 1000;
    let sandbox_permissions = SandboxPermissions::RequireEscalated;

    let turn_diff_tracker = Arc::new(tokio::sync::Mutex::new(TurnDiffTracker::new()));

    let tool_name = "shell_command";
    let call_id = "test-call".to_string();

    let handler = ShellCommandHandler::from(ShellCommandBackendConfig::Classic);
    #[allow(deprecated)]
    let workdir = Some(turn_context.cwd.to_string_lossy().to_string());
    let resp = handler
        .handle(ToolInvocation {
            session: Arc::clone(&session),
            turn: Arc::clone(&turn_context),
            cancellation_token: CancellationToken::new(),
            tracker: Arc::clone(&turn_diff_tracker),
            call_id,
            tool_name: codex_tools::ToolName::plain(tool_name),
            source: crate::tools::context::ToolCallSource::Direct,
            payload: ToolPayload::Function {
                arguments: serde_json::json!({
                    "command": command_script,
                    "workdir": workdir,
                    "timeout_ms": timeout_ms,
                    "sandbox_permissions": sandbox_permissions,
                    "justification": Some("test"),
                })
                .to_string(),
            },
        })
        .await;

    let Err(FunctionCallError::RespondToModel(output)) = resp else {
        panic!("expected error result");
    };

    let expected = format!(
        "approval policy is {policy:?}; reject command — you should not ask for escalated permissions if the approval policy is {policy:?}",
        policy = turn_context.approval_policy.value()
    );

    pretty_assertions::assert_eq!(output, expected);
    pretty_assertions::assert_eq!(session.granted_turn_permissions().await, None);

    // The rejection should not poison the non-escalated path for the same
    // command. Force DangerFullAccess so this check stays focused on approval
    // policy rather than platform-specific sandbox behavior.
    let turn_context_mut = Arc::get_mut(&mut turn_context).expect("unique turn context Arc");
    turn_context_mut.permission_profile = PermissionProfile::Disabled;

    let file_system_sandbox_policy = turn_context.file_system_sandbox_policy();
    let command = session
        .user_shell()
        .derive_exec_args(command_script, turn_context.tools_config.allow_login_shell);
    let exec_approval_requirement = session
        .services
        .exec_policy
        .create_exec_approval_requirement_for_command(ExecApprovalRequest {
            command: &command,
            approval_policy: turn_context.approval_policy.value(),
            permission_profile: turn_context.permission_profile(),
            file_system_sandbox_policy: &file_system_sandbox_policy,
            #[allow(deprecated)]
            sandbox_cwd: turn_context.cwd.as_path(),
            sandbox_permissions: SandboxPermissions::UseDefault,
            prefix_rule: None,
        })
        .await;
    assert!(matches!(
        exec_approval_requirement,
        ExecApprovalRequirement::Skip { .. }
    ));
}
#[tokio::test]
async fn unified_exec_rejects_escalated_permissions_when_policy_not_on_request() {
    use crate::sandboxing::SandboxPermissions;
    use crate::turn_diff_tracker::TurnDiffTracker;
    use codex_protocol::protocol::AskForApproval;

    let (session, mut turn_context_raw) = make_session_and_context().await;
    turn_context_raw
        .approval_policy
        .set(AskForApproval::OnFailure)
        .expect("test setup should allow updating approval policy");
    let session = Arc::new(session);
    let turn_context = Arc::new(turn_context_raw);
    let tracker = Arc::new(tokio::sync::Mutex::new(TurnDiffTracker::new()));

    let handler = ExecCommandHandler::default();
    let resp = handler
        .handle(ToolInvocation {
            session: Arc::clone(&session),
            turn: Arc::clone(&turn_context),
            cancellation_token: CancellationToken::new(),
            tracker: Arc::clone(&tracker),
            call_id: "exec-call".to_string(),
            tool_name: codex_tools::ToolName::plain("exec_command"),
            source: crate::tools::context::ToolCallSource::Direct,
            payload: ToolPayload::Function {
                arguments: serde_json::json!({
                    "cmd": "echo hi",
                    "sandbox_permissions": SandboxPermissions::RequireEscalated,
                    "justification": "need unsandboxed execution",
                })
                .to_string(),
            },
        })
        .await;

    let Err(FunctionCallError::RespondToModel(output)) = resp else {
        panic!("expected error result");
    };

    let expected = format!(
        "approval policy is {policy:?}; reject command — you cannot ask for escalated permissions if the approval policy is {policy:?}",
        policy = turn_context.approval_policy.value()
    );

    pretty_assertions::assert_eq!(output, expected);
}

#[tokio::test]
async fn session_start_hooks_only_load_from_trusted_project_layers() -> std::io::Result<()> {
    let temp = tempfile::tempdir()?;
    let codex_home = temp.path().join("home");
    let project_root = temp.path().join("project");
    let nested = project_root.join("nested");
    let root_dot_codex = project_root.join(".codex");
    let nested_dot_codex = nested.join(".codex");

    std::fs::create_dir_all(&codex_home)?;
    std::fs::create_dir_all(&nested_dot_codex)?;
    std::fs::write(project_root.join(".git"), "gitdir: here")?;
    write_project_hooks(&root_dot_codex)?;
    write_project_hooks(&nested_dot_codex)?;
    write_project_trust_config(&codex_home, &[(&nested, TrustLevel::Trusted)]).await?;

    let config = ConfigBuilder::default()
        .codex_home(codex_home)
        .fallback_cwd(Some(nested))
        .build()
        .await?;

    let hook_list = codex_hooks::list_hooks(codex_hooks::HooksConfig {
        feature_enabled: true,
        config_layer_stack: Some(config.config_layer_stack.clone()),
        ..codex_hooks::HooksConfig::default()
    });
    let expected_source_path = codex_utils_absolute_path::AbsolutePathBuf::from_absolute_path(
        nested_dot_codex.join("hooks.json"),
    )?;
    assert_eq!(
        hook_list
            .hooks
            .iter()
            .map(|hook| &hook.source_path)
            .collect::<Vec<_>>(),
        vec![&expected_source_path],
    );
    assert_eq!(
        hook_list.hooks[0].trust_status,
        codex_protocol::protocol::HookTrustStatus::Untrusted
    );
    assert!(preview_session_start_hooks(&config).await?.is_empty());

    Ok(())
}

#[tokio::test]
async fn session_start_hooks_require_project_trust_without_config_toml() -> std::io::Result<()> {
    let temp = tempfile::tempdir()?;
    let project_root = temp.path().join("project");
    let nested = project_root.join("nested");
    let dot_codex = project_root.join(".codex");
    std::fs::create_dir_all(&nested)?;
    std::fs::write(project_root.join(".git"), "gitdir: here")?;
    write_project_hooks(&dot_codex)?;

    let cases = [
        ("unknown", Vec::<(&Path, TrustLevel)>::new(), 0_usize),
        (
            "untrusted",
            vec![(&project_root as &Path, TrustLevel::Untrusted)],
            0_usize,
        ),
        (
            "trusted",
            vec![(&project_root as &Path, TrustLevel::Trusted)],
            1_usize,
        ),
    ];

    for (name, trust_entries, expected_hooks) in cases {
        let codex_home = temp.path().join(format!("home_{name}"));
        std::fs::create_dir_all(&codex_home)?;
        write_project_trust_config(&codex_home, &trust_entries).await?;

        let config = ConfigBuilder::default()
            .codex_home(codex_home)
            .fallback_cwd(Some(nested.clone()))
            .build()
            .await?;

        let hook_list = codex_hooks::list_hooks(codex_hooks::HooksConfig {
            feature_enabled: true,
            config_layer_stack: Some(config.config_layer_stack.clone()),
            ..codex_hooks::HooksConfig::default()
        });
        assert_eq!(
            hook_list.hooks.len(),
            expected_hooks,
            "unexpected discovered hook count for {name}",
        );
        assert!(preview_session_start_hooks(&config).await?.is_empty());
        if expected_hooks == 1 {
            assert_eq!(
                hook_list.hooks[0].trust_status,
                codex_protocol::protocol::HookTrustStatus::Untrusted
            );
        }
    }

    Ok(())
}
