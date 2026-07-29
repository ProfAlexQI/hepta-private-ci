#[tokio::test]
#[expect(
    clippy::await_holding_invalid_type,
    reason = "the test deliberately holds the startup-token guard to exercise publication retry"
)]
async fn mcp_refresh_publication_retries_without_partial_commit_when_startup_token_is_busy() {
    let (session, turn_context, _rx) = make_session_and_context_with_rx().await;
    let old_token = session.mcp_startup_cancellation_token().await;
    let initial_manager_generation = session
        .services
        .mcp_connection_manager
        .read()
        .await
        .generation();
    super::handlers::refresh_mcp_servers(
        &session,
        McpServerRefreshConfig {
            mcp_servers: json!({}),
            mcp_oauth_credentials_store_mode: serde_json::to_value(OAuthCredentialsStoreMode::Auto)
                .expect("serialize store mode"),
            elicitation_authority: None,
        },
    )
    .await;
    let intent_generation = session
        .mcp_server_refresh_state
        .lock()
        .await
        .pending
        .as_ref()
        .expect("refresh intent")
        .generation;

    let reached = Arc::new(tokio::sync::Barrier::new(2));
    let release = Arc::new(tokio::sync::Barrier::new(2));
    let retry_reached = Arc::new(tokio::sync::Notify::new());
    *session
        .mcp_server_refresh_publication_test_gate
        .lock()
        .await = Some((Arc::clone(&reached), Arc::clone(&release)));
    *session.mcp_server_refresh_retry_test_notify.lock().await = Some(Arc::clone(&retry_reached));
    let mut refresh_task = {
        let session = Arc::clone(&session);
        let turn_context = Arc::clone(&turn_context);
        tokio::spawn(async move {
            session
                .refresh_mcp_servers_if_requested(&turn_context, None)
                .await;
        })
    };
    reached.wait().await;
    let startup_token_guard = session.services.mcp_startup_cancellation_token.lock().await;
    release.wait().await;
    timeout(StdDuration::from_secs(2), retry_reached.notified())
        .await
        .expect("MCP refresh must enter startup-token retry");

    assert!(!refresh_task.is_finished());
    {
        let state = session.mcp_server_refresh_state.lock().await;
        assert_eq!(
            state.pending.as_ref().map(|intent| intent.generation),
            Some(intent_generation)
        );
        assert_eq!(state.applied_generation, 0);
    }
    assert_eq!(
        session
            .services
            .mcp_connection_manager
            .read()
            .await
            .generation(),
        initial_manager_generation
    );
    assert!(!old_token.is_cancelled());

    drop(startup_token_guard);
    timeout(StdDuration::from_secs(2), &mut refresh_task)
        .await
        .expect("MCP refresh should complete after startup token release")
        .expect("MCP refresh task");

    {
        let state = session.mcp_server_refresh_state.lock().await;
        assert!(state.pending.is_none());
        assert_eq!(state.applied_generation, intent_generation);
    }
    assert!(old_token.is_cancelled());
    assert_eq!(
        session
            .services
            .mcp_connection_manager
            .read()
            .await
            .generation(),
        initial_manager_generation + 1
    );
}

#[tokio::test]
#[expect(
    clippy::await_holding_invalid_type,
    reason = "the test deliberately holds the startup-token guard to exercise cancellation during retry"
)]
async fn cancelled_mcp_refresh_publication_retry_preserves_the_pending_intent() {
    let (session, turn_context, _rx) = make_session_and_context_with_rx().await;
    let old_token = session.mcp_startup_cancellation_token().await;
    let initial_manager_generation = session
        .services
        .mcp_connection_manager
        .read()
        .await
        .generation();
    super::handlers::refresh_mcp_servers(
        &session,
        McpServerRefreshConfig {
            mcp_servers: json!({}),
            mcp_oauth_credentials_store_mode: serde_json::to_value(OAuthCredentialsStoreMode::Auto)
                .expect("serialize store mode"),
            elicitation_authority: None,
        },
    )
    .await;
    let intent_generation = session
        .mcp_server_refresh_state
        .lock()
        .await
        .pending
        .as_ref()
        .expect("refresh intent")
        .generation;

    let reached = Arc::new(tokio::sync::Barrier::new(2));
    let release = Arc::new(tokio::sync::Barrier::new(2));
    let retry_reached = Arc::new(tokio::sync::Notify::new());
    *session
        .mcp_server_refresh_publication_test_gate
        .lock()
        .await = Some((Arc::clone(&reached), Arc::clone(&release)));
    *session.mcp_server_refresh_retry_test_notify.lock().await = Some(Arc::clone(&retry_reached));
    let refresh_task = {
        let session = Arc::clone(&session);
        let turn_context = Arc::clone(&turn_context);
        tokio::spawn(async move {
            session
                .refresh_mcp_servers_if_requested(&turn_context, None)
                .await;
        })
    };
    reached.wait().await;
    let startup_token_guard = session.services.mcp_startup_cancellation_token.lock().await;
    release.wait().await;
    timeout(StdDuration::from_secs(2), retry_reached.notified())
        .await
        .expect("MCP refresh must enter startup-token retry");
    refresh_task.abort();
    assert!(
        refresh_task
            .await
            .expect_err("MCP refresh task must be cancelled")
            .is_cancelled()
    );
    drop(startup_token_guard);

    {
        let state = session.mcp_server_refresh_state.lock().await;
        assert_eq!(
            state.pending.as_ref().map(|intent| intent.generation),
            Some(intent_generation)
        );
        assert_eq!(state.applied_generation, 0);
    }
    assert!(!old_token.is_cancelled());
    assert_eq!(
        session
            .services
            .mcp_connection_manager
            .read()
            .await
            .generation(),
        initial_manager_generation
    );

    session
        .refresh_mcp_servers_if_requested(&turn_context, None)
        .await;
    {
        let state = session.mcp_server_refresh_state.lock().await;
        assert!(state.pending.is_none());
        assert_eq!(state.applied_generation, intent_generation);
    }
    assert!(old_token.is_cancelled());
    assert_eq!(
        session
            .services
            .mcp_connection_manager
            .read()
            .await
            .generation(),
        initial_manager_generation + 1
    );
}

#[tokio::test]
async fn invalid_explicit_mcp_refresh_remains_replaceable_without_busy_loop() {
    let (session, turn_context, _rx) = make_session_and_context_with_rx().await;
    let old_token = session.mcp_startup_cancellation_token().await;
    super::handlers::refresh_mcp_servers(
        &session,
        McpServerRefreshConfig {
            mcp_servers: json!("invalid"),
            mcp_oauth_credentials_store_mode: serde_json::to_value(OAuthCredentialsStoreMode::Auto)
                .expect("serialize store mode"),
            elicitation_authority: None,
        },
    )
    .await;
    session
        .refresh_mcp_servers_if_requested(&turn_context, None)
        .await;
    let invalid_generation = session
        .mcp_server_refresh_state
        .lock()
        .await
        .pending
        .as_ref()
        .expect("invalid intent remains pending")
        .generation;
    assert!(!old_token.is_cancelled());

    super::handlers::refresh_mcp_servers(
        &session,
        McpServerRefreshConfig {
            mcp_servers: json!({}),
            mcp_oauth_credentials_store_mode: serde_json::to_value(OAuthCredentialsStoreMode::Auto)
                .expect("serialize store mode"),
            elicitation_authority: None,
        },
    )
    .await;
    session
        .refresh_mcp_servers_if_requested(&turn_context, None)
        .await;
    let state = session.mcp_server_refresh_state.lock().await;
    assert!(state.pending.is_none());
    assert!(state.applied_generation > invalid_generation);
    drop(state);
    assert!(old_token.is_cancelled());
}

#[tokio::test]
async fn explicit_catalog_republish_replaces_manager_and_advances_generation() {
    let (session, turn_context) = make_session_and_context().await;
    let session = Arc::new(session);
    let initial_generation = session
        .services
        .mcp_connection_manager
        .read()
        .await
        .generation();
    let old_token = session.mcp_startup_cancellation_token().await;

    assert!(session.republish_mcp_catalog_now(&turn_context).await);

    assert_eq!(
        session
            .services
            .mcp_connection_manager
            .read()
            .await
            .generation(),
        initial_generation + 1
    );
    assert!(old_token.is_cancelled());
}

#[tokio::test]
async fn api_key_replacement_requests_mcp_generation_refresh() {
    let auth_home = tempfile::tempdir().expect("create auth home");
    let auth_manager = AuthManager::from_auth_for_testing_with_home(
        CodexAuth::from_api_key("Test API Key"),
        auth_home.path().to_path_buf(),
    );
    let (mut session, turn_context) = make_session_and_context().await;
    session.services.auth_manager = Arc::clone(&auth_manager);
    let session = Arc::new(session);
    let initial_generation = session
        .services
        .mcp_connection_manager
        .read()
        .await
        .generation();

    codex_login::login_with_api_key(
        auth_home.path(),
        "replacement-api-key",
        codex_config::types::AuthCredentialsStoreMode::File,
    )
    .expect("store replacement API key");
    assert!(auth_manager.reload().await);

    session
        .refresh_mcp_servers_if_requested(&turn_context, None)
        .await;

    let latest_binding = crate::state::FrozenMcpAuthSnapshot::capture(auth_manager.as_ref())
        .await
        .expect("capture latest MCP auth snapshot")
        .binding();
    let manager = session.services.mcp_connection_manager.read().await;
    assert_eq!(manager.generation(), initial_generation + 1);
    assert!(manager.auth_matches(&latest_binding));
}

#[tokio::test]
async fn startup_in_flight_auth_change_discards_stale_runtime() {
    let auth_home = tempfile::tempdir().expect("create auth home");
    let auth_manager = AuthManager::from_auth_for_testing_with_home(
        CodexAuth::from_api_key("startup-old-api-key"),
        auth_home.path().to_path_buf(),
    );
    let reached = Arc::new(tokio::sync::Barrier::new(2));
    let release = Arc::new(tokio::sync::Barrier::new(2));
    super::session::install_mcp_startup_auth_test_gate(
        &auth_manager,
        Arc::clone(&reached),
        Arc::clone(&release),
    );
    let session_task = {
        let auth_manager = Arc::clone(&auth_manager);
        tokio::spawn(async move {
            make_session_with_config_and_auth_manager_and_rx(|_config| {}, Some(auth_manager)).await
        })
    };
    reached.wait().await;

    codex_login::login_with_api_key(
        auth_home.path(),
        "startup-new-api-key",
        codex_config::types::AuthCredentialsStoreMode::File,
    )
    .expect("store startup replacement API key");
    assert!(auth_manager.reload().await);
    release.wait().await;

    let (session, _rx) = session_task
        .await
        .expect("startup task")
        .expect("session startup");
    let latest_binding = crate::state::FrozenMcpAuthSnapshot::capture(auth_manager.as_ref())
        .await
        .expect("capture latest MCP auth snapshot")
        .binding();
    let manager = session.services.mcp_connection_manager.read().await;
    assert_eq!(manager.generation(), 1);
    assert!(manager.auth_matches(&latest_binding));
}

#[tokio::test]
async fn mcp_refresh_projection_uses_one_frozen_auth_snapshot() {
    let auth_home = tempfile::tempdir().expect("create auth home");
    let auth_manager = AuthManager::from_auth_for_testing_with_home(
        CodexAuth::from_api_key("projection-old-api-key"),
        auth_home.path().to_path_buf(),
    );
    let frozen = crate::state::FrozenMcpAuthSnapshot::capture(auth_manager.as_ref())
        .await
        .expect("capture frozen MCP auth snapshot");
    let frozen_cache_key = codex_mcp::codex_apps_tools_cache_key(frozen.auth());

    codex_login::login_with_api_key(
        auth_home.path(),
        "projection-new-api-key",
        codex_config::types::AuthCredentialsStoreMode::File,
    )
    .expect("store projection replacement API key");
    assert!(auth_manager.reload().await);
    let latest = crate::state::FrozenMcpAuthSnapshot::capture(auth_manager.as_ref())
        .await
        .expect("capture latest MCP auth snapshot");

    assert!(!frozen.matches(&latest));
    assert_eq!(
        codex_mcp::codex_apps_tools_cache_key(frozen.auth()),
        frozen_cache_key
    );
    assert_eq!(
        codex_mcp::codex_apps_tools_cache_key(frozen.auth()),
        codex_mcp::codex_apps_tools_cache_key(latest.auth())
    );
    assert_eq!(
        frozen.auth().and_then(CodexAuth::api_key),
        Some("projection-old-api-key")
    );
    assert_eq!(
        latest.auth().and_then(CodexAuth::api_key),
        Some("projection-new-api-key")
    );
}

#[tokio::test]
async fn stale_auth_generation_does_not_publish_mcp_manager() {
    let auth_home = tempfile::tempdir().expect("create auth home");
    let auth_manager = AuthManager::from_auth_for_testing_with_home(
        CodexAuth::from_api_key("Test API Key"),
        auth_home.path().to_path_buf(),
    );
    let (mut session, turn_context) = make_session_and_context().await;
    session.services.auth_manager = Arc::clone(&auth_manager);
    let session = Arc::new(session);
    let turn_context = Arc::new(turn_context);
    let initial_manager_generation = session
        .services
        .mcp_connection_manager
        .read()
        .await
        .generation();
    super::handlers::refresh_mcp_servers(
        &session,
        McpServerRefreshConfig {
            mcp_servers: json!({}),
            mcp_oauth_credentials_store_mode: serde_json::to_value(OAuthCredentialsStoreMode::Auto)
                .expect("serialize store mode"),
            elicitation_authority: None,
        },
    )
    .await;
    let stale_intent_generation = session
        .mcp_server_refresh_state
        .lock()
        .await
        .pending
        .as_ref()
        .expect("stale refresh intent")
        .generation;

    let reached = Arc::new(tokio::sync::Barrier::new(2));
    let release = Arc::new(tokio::sync::Barrier::new(2));
    *session
        .mcp_server_refresh_publication_test_gate
        .lock()
        .await = Some((Arc::clone(&reached), Arc::clone(&release)));
    let refresh_task = {
        let session = Arc::clone(&session);
        let turn_context = Arc::clone(&turn_context);
        tokio::spawn(async move {
            session
                .refresh_mcp_servers_if_requested(&turn_context, None)
                .await;
        })
    };
    reached.wait().await;
    codex_login::login_with_api_key(
        auth_home.path(),
        "latest-api-key",
        codex_config::types::AuthCredentialsStoreMode::File,
    )
    .expect("store latest API key");
    assert!(auth_manager.reload().await);
    release.wait().await;
    refresh_task.await.expect("refresh task");

    {
        let refresh_state = session.mcp_server_refresh_state.lock().await;
        assert!(refresh_state.pending.is_none());
        assert!(refresh_state.applied_generation > stale_intent_generation);
    }
    let latest_binding = crate::state::FrozenMcpAuthSnapshot::capture(auth_manager.as_ref())
        .await
        .expect("capture latest MCP auth snapshot")
        .binding();
    let manager = session.services.mcp_connection_manager.read().await;
    assert_eq!(manager.generation(), initial_manager_generation + 1);
    assert!(manager.auth_matches(&latest_binding));
}

#[tokio::test]
async fn spawn_task_does_not_update_previous_turn_settings_for_non_run_turn_tasks() {
    let (sess, tc, _rx) = make_session_and_context_with_rx().await;
    sess.set_previous_turn_settings(/*previous_turn_settings*/ None)
        .await;
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
    assert_eq!(sess.previous_turn_settings().await, None);
}

#[tokio::test]
async fn build_settings_update_items_emits_environment_item_for_network_changes() {
    let (session, previous_context) = make_session_and_context().await;
    let previous_context = Arc::new(previous_context);
    let mut current_context = previous_context
        .with_model(
            previous_context.model_info.slug.clone(),
            &session.services.models_manager,
        )
        .await;

    let mut config = (*current_context.config).clone();
    let mut requirements = config.config_layer_stack.requirements().clone();
    requirements.network = Some(Sourced::new(
        NetworkConstraints {
            domains: Some(NetworkDomainPermissionsToml {
                entries: std::collections::BTreeMap::from([
                    (
                        "api.example.com".to_string(),
                        NetworkDomainPermissionToml::Allow,
                    ),
                    (
                        "blocked.example.com".to_string(),
                        NetworkDomainPermissionToml::Deny,
                    ),
                ]),
            }),
            ..Default::default()
        },
        RequirementSource::CloudRequirements,
    ));
    let layers = config
        .config_layer_stack
        .get_layers(
            ConfigLayerStackOrdering::LowestPrecedenceFirst,
            /*include_disabled*/ true,
        )
        .into_iter()
        .cloned()
        .collect();
    config.config_layer_stack = ConfigLayerStack::new(
        layers,
        requirements,
        config.config_layer_stack.requirements_toml().clone(),
    )
    .expect("rebuild config layer stack with network requirements");
    current_context.config = Arc::new(config);

    let reference_context_item = previous_context.to_turn_context_item();
    let update_items = session
        .build_settings_update_items(Some(&reference_context_item), &current_context)
        .await;

    let environment_update = user_input_texts(&update_items)
        .into_iter()
        .find(|text| text.contains("<environment_context>"))
        .expect("environment update item should be emitted");
    assert!(environment_update.contains(
        "<network enabled=\"true\"><allowed>api.example.com</allowed><denied>blocked.example.com</denied></network>"
    ));
}

#[tokio::test]
async fn environment_context_uses_session_shell_when_environment_shell_is_absent() {
    let (mut session, mut turn_context) = make_session_and_context().await;
    session.services.user_shell = Arc::new(crate::shell::Shell {
        shell_type: crate::shell::ShellType::PowerShell,
        shell_path: PathBuf::from("powershell"),
        shell_snapshot: crate::shell::empty_shell_snapshot_receiver(),
    });
    for environment in &mut turn_context.environments.turn_environments {
        environment.shell = None;
    }

    let session_shell = session.user_shell();
    let environment_context = crate::context::EnvironmentContext::from_turn_context(
        &turn_context,
        session_shell.as_ref(),
    )
    .render();
    assert!(
        environment_context.contains("<shell>powershell</shell>"),
        "{environment_context}"
    );

    let primary_environment = turn_context
        .environments
        .turn_environments
        .first_mut()
        .expect("primary environment");
    primary_environment.shell = Some("cmd".to_string());

    let environment_context = crate::context::EnvironmentContext::from_turn_context(
        &turn_context,
        session_shell.as_ref(),
    )
    .render();
    assert!(
        environment_context.contains("<shell>cmd</shell>"),
        "{environment_context}"
    );
}

#[tokio::test]
async fn build_settings_update_items_emits_environment_item_for_time_changes() {
    let (session, previous_context) = make_session_and_context().await;
    let previous_context = Arc::new(previous_context);
    let mut current_context = previous_context
        .with_model(
            previous_context.model_info.slug.clone(),
            &session.services.models_manager,
        )
        .await;
    current_context.current_date = Some("2026-02-27".to_string());
    current_context.timezone = Some("Europe/Berlin".to_string());

    let reference_context_item = previous_context.to_turn_context_item();
    let update_items = session
        .build_settings_update_items(Some(&reference_context_item), &current_context)
        .await;

    let environment_update = user_input_texts(&update_items)
        .into_iter()
        .find(|text| text.contains("<environment_context>"))
        .expect("environment update item should be emitted");
    assert!(environment_update.contains("<current_date>2026-02-27</current_date>"));
    assert!(environment_update.contains("<timezone>Europe/Berlin</timezone>"));
}

#[tokio::test]
async fn build_settings_update_items_manifest_uses_semantic_contribution_sources() {
    let (session, previous_context) = make_session_and_context().await;
    let previous_context = Arc::new(previous_context);
    let mut current_context = previous_context
        .with_model(
            previous_context.model_info.slug.clone(),
            &session.services.models_manager,
        )
        .await;
    current_context.permission_profile = PermissionProfile::Disabled;
    current_context.current_date = Some("2026-02-27".to_string());

    let previous_context_item = previous_context.to_turn_context_item();
    let update_items = session
        .build_settings_update_items(Some(&previous_context_item), &current_context)
        .await;
    let manifest = crate::context_manager::manifest::build_turn_context_manifest(&update_items)
        .expect("settings update items should produce a manifest");

    assert_eq!(
        manifest
            .entries
            .iter()
            .map(|entry| entry.source.as_str())
            .collect::<Vec<_>>(),
        vec![
            "turn_context:developer:permissions:0",
            "turn_context:contextual_user:environment:1",
        ]
    );
    assert_eq!(
        manifest
            .decision_ledger
            .iter()
            .map(|entry| entry.decision.as_str())
            .collect::<Vec<_>>(),
        vec![
            "included:always_include_safety_policy",
            "included:turn_environment",
            "policy:non_omitting_replay_baseline:within_budget",
        ]
    );
    assert_eq!(manifest.budget_tokens, Some(manifest.estimated_tokens));
    assert_eq!(manifest.omitted_entries, 0);
    assert!(manifest.omitted_sources.is_empty());
    assert!(!manifest.truncated);
    assert!(manifest.has_replay_integrity());
}

#[tokio::test]
async fn build_settings_update_items_uses_persisted_reference_model_for_model_switch_diff() {
    let (session, previous_context) = make_session_and_context().await;
    let next_model = if previous_context.model_info.slug == "gpt-5.4" {
        "gpt-5.2"
    } else {
        "gpt-5.4"
    };
    let current_context = previous_context
        .with_model(next_model.to_string(), &session.services.models_manager)
        .await;
    let mut previous_context_item = previous_context.to_turn_context_item();
    let previous_context_items = session.build_initial_context(&previous_context).await;
    previous_context_item.context_manifest =
        crate::context_manager::manifest::build_turn_context_manifest(&previous_context_items);
    session
        .set_previous_turn_settings(Some(PreviousTurnSettings {
            model: current_context.model_info.slug.clone(),
            realtime_active: Some(current_context.realtime_active),
        }))
        .await;

    let update_items = session
        .build_settings_update_items(Some(&previous_context_item), &current_context)
        .await;
    let developer_texts = developer_input_texts(&update_items);
    let manifest = crate::context_manager::manifest::build_turn_context_manifest(&update_items)
        .expect("model switch update should produce a manifest");

    assert!(
        developer_texts
            .iter()
            .any(|text| text.contains("<model_switch>")),
        "expected persisted reference context model to drive model-switch diff, got {developer_texts:?}"
    );
    assert_eq!(
        manifest
            .entries
            .iter()
            .map(|entry| entry.source.as_str())
            .collect::<Vec<_>>(),
        vec!["turn_context:developer:model_switch:0"]
    );
    assert!(manifest.has_replay_integrity());
}

#[tokio::test]
async fn build_settings_update_items_uses_manifest_hash_for_model_switch_instruction_diff() {
    let (session, previous_context) = make_session_and_context().await;
    let next_model = if previous_context.model_info.slug == "gpt-5.4" {
        "gpt-5.2"
    } else {
        "gpt-5.4"
    };
    let current_context = previous_context
        .with_model(next_model.to_string(), &session.services.models_manager)
        .await;
    let current_model_instructions = current_context
        .model_info
        .get_model_instructions(current_context.personality);
    assert!(
        !current_model_instructions.is_empty(),
        "test model should expose model-switch instructions"
    );

    let mut previous_context_item = current_context.to_turn_context_item();
    let stale_model_switch_item = ResponseItem::Message {
        id: None,
        role: "developer".to_string(),
        content: vec![ContentItem::InputText {
            text: crate::context::ModelSwitchInstructions::new("stale model guidance").render(),
        }],
        phase: None,
    };
    previous_context_item.context_manifest =
        crate::context_manager::manifest::build_turn_context_manifest(&[stale_model_switch_item]);
    session
        .set_previous_turn_settings(Some(PreviousTurnSettings {
            model: current_context.model_info.slug.clone(),
            realtime_active: Some(current_context.realtime_active),
        }))
        .await;

    let update_items = session
        .build_settings_update_items(Some(&previous_context_item), &current_context)
        .await;
    let developer_texts = developer_input_texts(&update_items);
    let manifest = crate::context_manager::manifest::build_turn_context_manifest(&update_items)
        .expect("model switch update should produce a manifest");

    assert!(
        developer_texts.iter().any(|text| {
            text.contains("<model_switch>")
                && text.contains(&current_model_instructions)
                && !text.contains("stale model guidance")
        }),
        "expected model switch update from manifest hash diff, got {developer_texts:?}"
    );
    assert_eq!(
        manifest
            .entries
            .iter()
            .map(|entry| entry.source.as_str())
            .collect::<Vec<_>>(),
        vec!["turn_context:developer:model_switch:0"]
    );
    assert!(manifest.has_replay_integrity());
}

#[tokio::test]
async fn build_settings_update_items_clears_model_switch_when_current_model_instructions_disappear_once()
 {
    let (session, mut current_context) = make_session_and_context().await;
    current_context.model_info.base_instructions = String::new();
    current_context.model_info.model_messages = None;
    assert!(
        current_context
            .model_info
            .get_model_instructions(current_context.personality)
            .is_empty(),
        "test model should expose no current model-switch instructions"
    );
    let mut previous_context_item = current_context.to_turn_context_item();
    let previous_model_switch_item = ResponseItem::Message {
        id: None,
        role: "developer".to_string(),
        content: vec![ContentItem::InputText {
            text: crate::context::ModelSwitchInstructions::new("previous model guidance").render(),
        }],
        phase: None,
    };
    previous_context_item.context_manifest =
        crate::context_manager::manifest::build_turn_context_manifest(&[
            previous_model_switch_item,
        ]);
    session
        .set_previous_turn_settings(Some(PreviousTurnSettings {
            model: current_context.model_info.slug.clone(),
            realtime_active: Some(current_context.realtime_active),
        }))
        .await;

    let update_items = session
        .build_settings_update_items(Some(&previous_context_item), &current_context)
        .await;
    let developer_texts = developer_input_texts(&update_items);
    let manifest = crate::context_manager::manifest::build_turn_context_manifest(&update_items)
        .expect("model switch clear should produce a manifest");

    assert!(
        developer_texts.iter().any(|text| {
            text.contains("<model_switch>")
                && text.contains("Model-specific switch instructions were cleared")
                && !text.contains("previous model guidance")
        }),
        "expected model switch clear from missing current instructions, got {developer_texts:?}"
    );
    assert_eq!(
        manifest
            .entries
            .iter()
            .map(|entry| entry.source.as_str())
            .collect::<Vec<_>>(),
        vec!["turn_context:developer:model_switch:0"]
    );
    assert!(manifest.has_replay_integrity());

    previous_context_item.context_manifest = Some(manifest);
    let repeated_update_items = session
        .build_settings_update_items(Some(&previous_context_item), &current_context)
        .await;
    assert!(
        repeated_update_items.is_empty(),
        "already-cleared model switch should not emit again: {repeated_update_items:?}"
    );
}

#[tokio::test]
async fn build_settings_update_items_uses_manifest_hash_for_permissions_exec_policy_diff() {
    let (mut session, mut previous_context) = make_session_and_context().await;
    previous_context.approval_policy =
        codex_config::Constrained::allow_any(AskForApproval::OnRequest);
    let mut previous_context_item = previous_context.to_turn_context_item();
    let previous_context_items = session.build_initial_context(&previous_context).await;
    previous_context_item.context_manifest =
        crate::context_manager::manifest::build_turn_context_manifest(&previous_context_items);

    let mut exec_policy = Policy::empty();
    exec_policy
        .add_prefix_rule(&["git".to_string(), "pull".to_string()], Decision::Allow)
        .expect("add approved prefix rule");
    session.services.exec_policy = Arc::new(ExecPolicyManager::new(Arc::new(exec_policy)));

    let update_items = session
        .build_settings_update_items(Some(&previous_context_item), &previous_context)
        .await;
    let developer_texts = developer_input_texts(&update_items);
    let manifest = crate::context_manager::manifest::build_turn_context_manifest(&update_items)
        .expect("permissions update should produce a manifest");

    assert!(
        developer_texts.iter().any(|text| {
            text.contains("Approved command prefixes") && text.contains(r#"["git", "pull"]"#)
        }),
        "expected exec-policy prefix update from manifest hash diff, got {developer_texts:?}"
    );
    assert_eq!(
        manifest
            .entries
            .iter()
            .map(|entry| entry.source.as_str())
            .collect::<Vec<_>>(),
        vec!["turn_context:developer:permissions:0"]
    );
    assert!(manifest.has_replay_integrity());
}

#[tokio::test]
async fn build_settings_update_items_uses_manifest_hash_for_personality_spec_diff() {
    let (session, turn_context, _rx_event) = make_session_and_context_with_auth_and_config_and_rx(
        CodexAuth::from_api_key("Test API Key"),
        Vec::new(),
        |config| {
            config.model = Some("exp-codex-personality".to_string());
            config.personality = Some(Personality::Pragmatic);
            config
                .features
                .enable(Feature::Personality)
                .expect("personality feature should be enableable in tests");
        },
    )
    .await;
    let mut previous_context_item = turn_context.to_turn_context_item();
    let stale_personality_item = ResponseItem::Message {
        id: None,
        role: "developer".to_string(),
        content: vec![ContentItem::InputText {
            text: crate::context::PersonalitySpecInstructions::new("stale personality guidance")
                .render(),
        }],
        phase: None,
    };
    previous_context_item.context_manifest =
        crate::context_manager::manifest::build_turn_context_manifest(&[stale_personality_item]);

    let update_items = session
        .build_settings_update_items(Some(&previous_context_item), turn_context.as_ref())
        .await;
    let developer_texts = developer_input_texts(&update_items);
    let manifest = crate::context_manager::manifest::build_turn_context_manifest(&update_items)
        .expect("personality update should produce a manifest");

    assert!(
        developer_texts.iter().any(|text| {
            text.contains("<personality_spec>")
                && text.contains("deeply pragmatic, effective software engineer")
        }),
        "expected personality spec update from manifest hash diff, got {developer_texts:?}"
    );
    assert_eq!(
        manifest
            .entries
            .iter()
            .map(|entry| entry.source.as_str())
            .collect::<Vec<_>>(),
        vec!["turn_context:developer:personality:0"]
    );
    assert!(manifest.has_replay_integrity());
}

#[tokio::test]
async fn build_settings_update_items_uses_manifest_hash_for_environment_shell_diff() {
    let (session, turn_context) = make_session_and_context().await;
    let session_shell = session.user_shell();
    let current_environment_text = crate::context::EnvironmentContext::from_turn_context(
        &turn_context,
        session_shell.as_ref(),
    )
    .render();
    let current_shell_tag = format!("<shell>{}</shell>", session_shell.name());
    assert!(
        current_environment_text.contains(&current_shell_tag),
        "expected rendered environment to contain current shell tag, got {current_environment_text}"
    );
    let stale_environment_text =
        current_environment_text.replace(&current_shell_tag, "<shell>stale-shell</shell>");
    assert_ne!(stale_environment_text, current_environment_text);

    let mut previous_context_item = turn_context.to_turn_context_item();
    let stale_environment_item = ResponseItem::Message {
        id: None,
        role: "user".to_string(),
        content: vec![ContentItem::InputText {
            text: stale_environment_text,
        }],
        phase: None,
    };
    previous_context_item.context_manifest =
        crate::context_manager::manifest::build_turn_context_manifest(&[stale_environment_item]);

    let update_items = session
        .build_settings_update_items(Some(&previous_context_item), &turn_context)
        .await;
    let user_texts = user_input_texts(&update_items);
    let manifest = crate::context_manager::manifest::build_turn_context_manifest(&update_items)
        .expect("environment update should produce a manifest");

    assert!(
        user_texts.iter().any(|text| {
            text.contains("<environment_context>")
                && text.contains(&current_shell_tag)
                && !text.contains("stale-shell")
        }),
        "expected environment shell update from manifest hash diff, got {user_texts:?}"
    );
    assert_eq!(
        manifest
            .entries
            .iter()
            .map(|entry| entry.source.as_str())
            .collect::<Vec<_>>(),
        vec!["turn_context:contextual_user:environment:0"]
    );
    assert!(manifest.has_replay_integrity());
}

#[tokio::test]
async fn build_settings_update_items_uses_manifest_hash_for_developer_instruction_diff() {
    let (session, mut turn_context) = make_session_and_context().await;
    turn_context.developer_instructions = Some("Use current developer guidance.".to_string());
    let mut previous_context_item = turn_context.to_turn_context_item();
    let stale_developer_item = ResponseItem::Message {
        id: None,
        role: "developer".to_string(),
        content: vec![ContentItem::InputText {
            text: "Use stale developer guidance.".to_string(),
        }],
        phase: None,
    };
    previous_context_item.context_manifest =
        crate::context_manager::manifest::build_turn_context_manifest(&[stale_developer_item]);

    let update_items = session
        .build_settings_update_items(Some(&previous_context_item), &turn_context)
        .await;
    let developer_texts = developer_input_texts(&update_items);
    let manifest = crate::context_manager::manifest::build_turn_context_manifest(&update_items)
        .expect("developer-instruction update should produce a manifest");

    assert!(
        developer_texts
            .iter()
            .any(|text| text.contains("Use current developer guidance.")),
        "expected developer instruction update from manifest hash diff, got {developer_texts:?}"
    );
    assert_eq!(
        manifest
            .entries
            .iter()
            .map(|entry| entry.source.as_str())
            .collect::<Vec<_>>(),
        vec!["turn_context:developer:developer_instructions:0"]
    );
    assert!(manifest.has_replay_integrity());
}

#[tokio::test]
async fn build_settings_update_items_uses_manifest_hash_for_collaboration_mode_diff() {
    let (session, mut turn_context) = make_session_and_context().await;
    turn_context.collaboration_mode = CollaborationMode {
        mode: ModeKind::Plan,
        settings: Settings {
            model: turn_context.model_info.slug.clone(),
            reasoning_effort: turn_context.reasoning_effort,
            developer_instructions: Some("Use current plan-mode guidance.".to_string()),
        },
    };
    let stale_collaboration_mode = CollaborationMode {
        mode: ModeKind::Plan,
        settings: Settings {
            model: turn_context.model_info.slug.clone(),
            reasoning_effort: turn_context.reasoning_effort,
            developer_instructions: Some("Use stale plan-mode guidance.".to_string()),
        },
    };
    let mut previous_context_item = turn_context.to_turn_context_item();
    let stale_collaboration_item = ResponseItem::Message {
        id: None,
        role: "developer".to_string(),
        content: vec![ContentItem::InputText {
            text: crate::context::CollaborationModeInstructions::from_collaboration_mode(
                &stale_collaboration_mode,
            )
            .expect("stale collaboration mode should render")
            .render(),
        }],
        phase: None,
    };
    previous_context_item.context_manifest =
        crate::context_manager::manifest::build_turn_context_manifest(&[stale_collaboration_item]);

    let update_items = session
        .build_settings_update_items(Some(&previous_context_item), &turn_context)
        .await;
    let developer_texts = developer_input_texts(&update_items);
    let manifest = crate::context_manager::manifest::build_turn_context_manifest(&update_items)
        .expect("collaboration-mode update should produce a manifest");

    assert!(
        developer_texts.iter().any(|text| {
            text.contains("<collaboration_mode>")
                && text.contains("Use current plan-mode guidance.")
        }),
        "expected collaboration-mode update from manifest hash diff, got {developer_texts:?}"
    );
    assert_eq!(
        manifest
            .entries
            .iter()
            .map(|entry| entry.source.as_str())
            .collect::<Vec<_>>(),
        vec!["turn_context:developer:collaboration_mode:0"]
    );
    assert!(manifest.has_replay_integrity());
}

#[tokio::test]
async fn build_settings_update_items_uses_manifest_hash_for_user_instruction_directory_diff() {
    let (session, mut turn_context) = make_session_and_context().await;
    turn_context.user_instructions = Some("Use the current workspace guidance.".to_string());
    #[allow(deprecated)]
    let current_directory = turn_context.cwd.to_string_lossy().into_owned();
    let stale_directory = "/tmp/stale-workspace";
    let current_user_text = crate::context::UserInstructions {
        directory: current_directory.clone(),
        text: turn_context
            .user_instructions
            .clone()
            .expect("user instructions should be set"),
    }
    .render();
    let stale_user_text = crate::context::UserInstructions {
        directory: stale_directory.to_string(),
        text: turn_context
            .user_instructions
            .clone()
            .expect("user instructions should be set"),
    }
    .render();
    assert_ne!(stale_user_text, current_user_text);

    let mut previous_context_item = turn_context.to_turn_context_item();
    let stale_user_item = ResponseItem::Message {
        id: None,
        role: "user".to_string(),
        content: vec![ContentItem::InputText {
            text: stale_user_text,
        }],
        phase: None,
    };
    previous_context_item.context_manifest =
        crate::context_manager::manifest::build_turn_context_manifest(&[stale_user_item]);

    let update_items = session
        .build_settings_update_items(Some(&previous_context_item), &turn_context)
        .await;
    let user_texts = user_input_texts(&update_items);
    let manifest = crate::context_manager::manifest::build_turn_context_manifest(&update_items)
        .expect("user-instruction update should produce a manifest");

    assert!(
        user_texts.iter().any(|text| {
            text.contains("# AGENTS.md instructions for ")
                && text.contains(&current_directory)
                && text.contains("Use the current workspace guidance.")
                && !text.contains(stale_directory)
        }),
        "expected user instruction update from manifest hash diff, got {user_texts:?}"
    );
    assert_eq!(
        manifest
            .entries
            .iter()
            .map(|entry| entry.source.as_str())
            .collect::<Vec<_>>(),
        vec!["turn_context:contextual_user:user_instructions:0"]
    );
    assert!(manifest.has_replay_integrity());
}

#[tokio::test]
async fn build_settings_update_items_emits_collaboration_mode_clear_when_instructions_disappear() {
    let (session, mut previous_context) = make_session_and_context().await;
    previous_context.collaboration_mode = CollaborationMode {
        mode: ModeKind::Plan,
        settings: Settings {
            model: previous_context.model_info.slug.clone(),
            reasoning_effort: previous_context.reasoning_effort,
            developer_instructions: Some("Use plan-mode guidance".to_string()),
        },
    };
    let mut current_context = previous_context
        .with_model(
            previous_context.model_info.slug.clone(),
            &session.services.models_manager,
        )
        .await;
    current_context.collaboration_mode =
        current_context
            .collaboration_mode
            .with_updates(None, None, Some(None));

    let update_items = session
        .build_settings_update_items(
            Some(&previous_context.to_turn_context_item()),
            &current_context,
        )
        .await;
    let developer_texts = developer_input_texts(&update_items);
    let manifest = crate::context_manager::manifest::build_turn_context_manifest(&update_items)
        .expect("collaboration-mode clear should produce a manifest");

    assert!(
        developer_texts.iter().any(|text| {
            text.contains("<collaboration_mode>")
                && text.contains("developer instructions were cleared")
        }),
        "expected collaboration mode clear update, got {developer_texts:?}"
    );
    assert_eq!(
        manifest
            .entries
            .iter()
            .map(|entry| entry.source.as_str())
            .collect::<Vec<_>>(),
        vec!["turn_context:developer:collaboration_mode:0"]
    );
    assert!(
        manifest
            .decision_ledger
            .iter()
            .any(|entry| entry.decision == "included:always_include_developer")
    );
    assert!(manifest.has_replay_integrity());
}

#[tokio::test]
async fn build_settings_update_items_emits_user_and_developer_instruction_clears() {
    let (session, mut previous_context) = make_session_and_context().await;
    previous_context.developer_instructions =
        Some("Keep previous developer guidance active.".to_string());
    previous_context.user_instructions =
        Some("Keep previous workspace guidance active.".to_string());
    let mut current_context = previous_context
        .with_model(
            previous_context.model_info.slug.clone(),
            &session.services.models_manager,
        )
        .await;
    current_context.developer_instructions = None;
    current_context.user_instructions = None;

    let update_items = session
        .build_settings_update_items(
            Some(&previous_context.to_turn_context_item()),
            &current_context,
        )
        .await;
    let developer_texts = developer_input_texts(&update_items);
    let user_texts = user_input_texts(&update_items);
    let manifest = crate::context_manager::manifest::build_turn_context_manifest(&update_items)
        .expect("instruction clears should produce a manifest");

    assert!(
        developer_texts
            .iter()
            .any(|text| text.contains("Developer instructions were cleared")),
        "expected developer-instruction clear update, got {developer_texts:?}"
    );
    assert!(
        user_texts.iter().any(|text| {
            text.contains("# AGENTS.md instructions for ")
                && text.contains("Workspace/user instructions were cleared")
        }),
        "expected user-instruction clear update, got {user_texts:?}"
    );
    assert_eq!(
        manifest
            .entries
            .iter()
            .map(|entry| entry.source.as_str())
            .collect::<Vec<_>>(),
        vec![
            "turn_context:developer:developer_instructions:0",
            "turn_context:contextual_user:user_instructions:1",
        ]
    );
    assert_eq!(
        manifest
            .decision_ledger
            .iter()
            .map(|entry| entry.decision.as_str())
            .collect::<Vec<_>>(),
        vec![
            "included:always_include_developer",
            "included:always_include_contextual_user",
            "policy:non_omitting_replay_baseline:within_budget",
        ]
    );
    assert!(manifest.has_replay_integrity());
}

#[tokio::test]
async fn build_settings_update_items_emits_capability_inventory_clears() {
    let (session, previous_context) = make_session_and_context().await;
    let mut previous_context_item = previous_context.to_turn_context_item();
    let previous_capability_items = vec![ResponseItem::Message {
        id: None,
        role: "developer".to_string(),
        content: vec![
            ContentItem::InputText {
                text: tagged_context_fragment(
                    codex_protocol::protocol::APPS_INSTRUCTIONS_OPEN_TAG,
                    codex_protocol::protocol::APPS_INSTRUCTIONS_CLOSE_TAG,
                    "Previously visible apps.",
                ),
            },
            ContentItem::InputText {
                text: tagged_context_fragment(
                    codex_protocol::protocol::SKILLS_INSTRUCTIONS_OPEN_TAG,
                    codex_protocol::protocol::SKILLS_INSTRUCTIONS_CLOSE_TAG,
                    "Previously visible skills.",
                ),
            },
            ContentItem::InputText {
                text: tagged_context_fragment(
                    codex_protocol::protocol::PLUGINS_INSTRUCTIONS_OPEN_TAG,
                    codex_protocol::protocol::PLUGINS_INSTRUCTIONS_CLOSE_TAG,
                    "Previously visible plugins.",
                ),
            },
        ],
        phase: None,
    }];
    previous_context_item.context_manifest =
        crate::context_manager::manifest::build_turn_context_manifest(&previous_capability_items);
    let mut current_context = previous_context
        .with_model(
            previous_context.model_info.slug.clone(),
            &session.services.models_manager,
        )
        .await;
    current_context.turn_skills = TurnSkillsContext::new(Arc::new(SkillLoadOutcome::default()));

    let update_items = session
        .build_settings_update_items(Some(&previous_context_item), &current_context)
        .await;
    let developer_texts = developer_input_texts(&update_items);
    let manifest = crate::context_manager::manifest::build_turn_context_manifest(&update_items)
        .expect("capability clears should produce a manifest");

    assert!(
        developer_texts.iter().any(|text| {
            text.contains("<apps_instructions>")
                && text.contains("Apps/connectors capability inventory was cleared")
        }),
        "expected apps clear update, got {developer_texts:?}"
    );
    assert!(
        developer_texts.iter().any(|text| {
            text.contains("<skills_instructions>")
                && text.contains("Available skills capability inventory was cleared")
        }),
        "expected skills clear update, got {developer_texts:?}"
    );
    assert!(
        developer_texts.iter().any(|text| {
            text.contains("<plugins_instructions>")
                && text.contains("Available plugins capability inventory was cleared")
        }),
        "expected plugins clear update, got {developer_texts:?}"
    );
    assert_eq!(
        manifest
            .entries
            .iter()
            .map(|entry| entry.source.as_str())
            .collect::<Vec<_>>(),
        vec![
            "turn_context:developer:apps:0:0",
            "turn_context:developer:available_skills:0:1",
            "turn_context:developer:available_plugins:0:2",
        ]
    );
    assert_eq!(
        manifest
            .decision_ledger
            .iter()
            .map(|entry| entry.decision.as_str())
            .collect::<Vec<_>>(),
        vec![
            "included:capability_inventory",
            "included:capability_inventory",
            "included:capability_inventory",
            "policy:non_omitting_replay_baseline:within_budget",
        ]
    );
    assert!(manifest.has_replay_integrity());

    previous_context_item.context_manifest = Some(manifest);
    let repeated_update_items = session
        .build_settings_update_items(Some(&previous_context_item), &current_context)
        .await;
    assert!(
        repeated_update_items.is_empty(),
        "already-cleared capability inventory should not be cleared again: {repeated_update_items:?}"
    );
}

fn tagged_context_fragment(open_tag: &str, close_tag: &str, body: &str) -> String {
    format!("{open_tag}\n{body}\n{close_tag}")
}

#[tokio::test]
async fn build_settings_update_items_omits_environment_item_when_disabled() {
    let (session, previous_context) = make_session_and_context().await;
    let previous_context = Arc::new(previous_context);
    let mut current_context = previous_context
        .with_model(
            previous_context.model_info.slug.clone(),
            &session.services.models_manager,
        )
        .await;
    let mut config = (*current_context.config).clone();
    config.include_environment_context = false;
    current_context.config = Arc::new(config);
    current_context.current_date = Some("2026-02-27".to_string());

    let reference_context_item = previous_context.to_turn_context_item();
    let update_items = session
        .build_settings_update_items(Some(&reference_context_item), &current_context)
        .await;

    let user_texts = user_input_texts(&update_items);
    assert!(
        !user_texts
            .iter()
            .any(|text| text.contains("<environment_context>")),
        "did not expect environment context updates when disabled, got {user_texts:?}"
    );
}

#[tokio::test]
async fn build_settings_update_items_emits_realtime_start_when_session_becomes_live() {
    let (session, previous_context) = make_session_and_context().await;
    let previous_context = Arc::new(previous_context);
    let mut current_context = previous_context
        .with_model(
            previous_context.model_info.slug.clone(),
            &session.services.models_manager,
        )
        .await;
    current_context.realtime_active = true;

    let update_items = session
        .build_settings_update_items(
            Some(&previous_context.to_turn_context_item()),
            &current_context,
        )
        .await;

    let developer_texts = developer_input_texts(&update_items);
    assert!(
        developer_texts
            .iter()
            .any(|text| text.contains("<realtime_conversation>")),
        "expected a realtime start update, got {developer_texts:?}"
    );
}

#[tokio::test]
async fn build_settings_update_items_emits_realtime_end_when_session_stops_being_live() {
    let (session, mut previous_context) = make_session_and_context().await;
    previous_context.realtime_active = true;
    let mut current_context = previous_context
        .with_model(
            previous_context.model_info.slug.clone(),
            &session.services.models_manager,
        )
        .await;
    current_context.realtime_active = false;

    let update_items = session
        .build_settings_update_items(
            Some(&previous_context.to_turn_context_item()),
            &current_context,
        )
        .await;

    let developer_texts = developer_input_texts(&update_items);
    assert!(
        developer_texts
            .iter()
            .any(|text| text.contains("Reason: inactive")),
        "expected a realtime end update, got {developer_texts:?}"
    );
}

#[tokio::test]
async fn build_settings_update_items_uses_previous_turn_settings_for_realtime_end() {
    let (session, previous_context) = make_session_and_context().await;
    let mut previous_context_item = previous_context.to_turn_context_item();
    previous_context_item.realtime_active = None;
    let previous_turn_settings = PreviousTurnSettings {
        model: previous_context.model_info.slug.clone(),
        realtime_active: Some(true),
    };
    let mut current_context = previous_context
        .with_model(
            previous_context.model_info.slug.clone(),
            &session.services.models_manager,
        )
        .await;
    current_context.realtime_active = false;

    session
        .set_previous_turn_settings(Some(previous_turn_settings))
        .await;
    let update_items = session
        .build_settings_update_items(Some(&previous_context_item), &current_context)
        .await;

    let developer_texts = developer_input_texts(&update_items);
    assert!(
        developer_texts
            .iter()
            .any(|text| text.contains("Reason: inactive")),
        "expected a realtime end update from previous turn settings, got {developer_texts:?}"
    );
}

#[tokio::test]
async fn build_settings_update_items_ignores_previous_turn_settings_for_realtime_end_when_manifest_baseline_exists()
 {
    let (session, previous_context) = make_session_and_context().await;
    let mut previous_context_item = previous_context.to_turn_context_item();
    previous_context_item.realtime_active = None;
    let previous_context_items = session.build_initial_context(&previous_context).await;
    previous_context_item.context_manifest =
        crate::context_manager::manifest::build_turn_context_manifest(&previous_context_items);
    let previous_turn_settings = PreviousTurnSettings {
        model: previous_context.model_info.slug.clone(),
        realtime_active: Some(true),
    };
    let mut current_context = previous_context
        .with_model(
            previous_context.model_info.slug.clone(),
            &session.services.models_manager,
        )
        .await;
    current_context.realtime_active = false;

    session
        .set_previous_turn_settings(Some(previous_turn_settings))
        .await;
    let update_items = session
        .build_settings_update_items(Some(&previous_context_item), &current_context)
        .await;
    let developer_texts = developer_input_texts(&update_items);
    assert!(
        !developer_texts.iter().any(|text| {
            text.contains("<realtime_conversation>") || text.contains("Reason: inactive")
        }),
        "did not expect stale previous turn settings to override a durable manifest baseline, got {developer_texts:?}"
    );
}

#[tokio::test]
async fn build_settings_update_items_uses_manifest_hash_for_realtime_start_instruction_diff() {
    let (session, mut turn_context) = make_session_and_context().await;
    turn_context.realtime_active = true;
    let mut config = (*turn_context.config).clone();
    config.experimental_realtime_start_instructions =
        Some("Use current realtime guidance.".to_string());
    turn_context.config = Arc::new(config);

    let mut previous_context_item = turn_context.to_turn_context_item();
    previous_context_item.realtime_active = Some(true);
    let stale_realtime_item = ResponseItem::Message {
        id: None,
        role: "developer".to_string(),
        content: vec![ContentItem::InputText {
            text: crate::context::RealtimeStartWithInstructions::new(
                "Use stale realtime guidance.",
            )
            .render(),
        }],
        phase: None,
    };
    previous_context_item.context_manifest =
        crate::context_manager::manifest::build_turn_context_manifest(&[stale_realtime_item]);

    let update_items = session
        .build_settings_update_items(Some(&previous_context_item), &turn_context)
        .await;
    let developer_texts = developer_input_texts(&update_items);
    let manifest = crate::context_manager::manifest::build_turn_context_manifest(&update_items)
        .expect("realtime update should produce a manifest");

    assert!(
        developer_texts.iter().any(|text| {
            text.contains("<realtime_conversation>")
                && text.contains("Use current realtime guidance.")
                && !text.contains("Use stale realtime guidance.")
        }),
        "expected realtime start update from manifest hash diff, got {developer_texts:?}"
    );
    assert_eq!(
        manifest
            .entries
            .iter()
            .map(|entry| entry.source.as_str())
            .collect::<Vec<_>>(),
        vec!["turn_context:developer:realtime:0"]
    );
    assert!(manifest.has_replay_integrity());
}

#[tokio::test]
async fn build_initial_context_uses_previous_realtime_state() {
    let (session, mut turn_context) = make_session_and_context().await;
    turn_context.realtime_active = true;

    let initial_context = session.build_initial_context(&turn_context).await;
    let developer_texts = developer_input_texts(&initial_context);
    assert!(
        developer_texts
            .iter()
            .any(|text| text.contains("<realtime_conversation>")),
        "expected initial context to describe active realtime state, got {developer_texts:?}"
    );

    let previous_context_item = turn_context.to_turn_context_item();
    {
        let mut state = session.state.lock().await;
        state.set_reference_context_item(Some(previous_context_item));
    }
    let resumed_context = session.build_initial_context(&turn_context).await;
    let resumed_developer_texts = developer_input_texts(&resumed_context);
    assert!(
        !resumed_developer_texts
            .iter()
            .any(|text| text.contains("<realtime_conversation>")),
        "did not expect a duplicate realtime update, got {resumed_developer_texts:?}"
    );
}

async fn make_multi_agent_v2_usage_hint_test_session(
    enable_multi_agent_v2: bool,
) -> (Arc<Session>, Arc<TurnContext>) {
    let (session, turn_context, _rx_event) = make_session_and_context_with_auth_and_config_and_rx(
        CodexAuth::from_api_key("Test API Key"),
        Vec::new(),
        |config| {
            if enable_multi_agent_v2 {
                let _ = config.features.enable(Feature::MultiAgentV2);
            }
            config.multi_agent_v2.root_agent_usage_hint_text = Some("Root guidance.".to_string());
            config.multi_agent_v2.subagent_usage_hint_text = Some("Subagent guidance.".to_string());
        },
    )
    .await;
    (session, turn_context)
}

struct PromptExtensionTestContributor;
struct PromptExtensionTestState;
struct PromptExtensionDiffState {
    fragments: Vec<codex_extension_api::PromptFragment>,
}

impl codex_extension_api::ContextContributor for PromptExtensionTestContributor {
    fn contribute<'a>(
        &'a self,
        _session_store: &'a codex_extension_api::ExtensionData,
        thread_store: &'a codex_extension_api::ExtensionData,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Vec<codex_extension_api::PromptFragment>> + Send + 'a>,
    > {
        Box::pin(async move {
            let mut fragments = Vec::new();
            if thread_store.get::<PromptExtensionTestState>().is_some() {
                fragments.push(codex_extension_api::PromptFragment::developer_policy(
                    "prompt extension enabled",
                ));
            }
            if let Some(state) = thread_store.get::<PromptExtensionDiffState>() {
                fragments.extend(state.fragments.iter().cloned());
            }
            fragments
        })
    }
}

fn prompt_extension_test_registry()
-> Arc<codex_extension_api::ExtensionRegistry<crate::config::Config>> {
    let mut builder = codex_extension_api::ExtensionRegistryBuilder::new();
    builder.prompt_contributor(Arc::new(PromptExtensionTestContributor));
    Arc::new(builder.build())
}

#[tokio::test]
async fn build_initial_context_includes_prompt_fragments_from_extensions() {
    let (mut session, turn_context) = make_session_and_context().await;
    session.services.extensions = prompt_extension_test_registry();
    session
        .services
        .thread_extension_data
        .insert(PromptExtensionTestState);

    let initial_context = session.build_initial_context(&turn_context).await;
    let developer_messages = developer_message_texts(&initial_context);

    assert!(
        developer_messages
            .iter()
            .flatten()
            .any(|text| text.contains("<extension_developer_policy>")
                && text.contains("prompt extension enabled")),
        "expected prompt extension developer text, got {developer_messages:?}"
    );
}

#[tokio::test]
async fn build_initial_context_omits_prompt_fragments_without_extension_state() {
    let (mut session, turn_context) = make_session_and_context().await;
    session.services.extensions = prompt_extension_test_registry();

    let initial_context = session.build_initial_context(&turn_context).await;
    let developer_messages = developer_message_texts(&initial_context);

    assert!(
        !developer_messages
            .iter()
            .flatten()
            .any(|text| text.contains("prompt extension enabled")),
        "did not expect prompt extension developer text, got {developer_messages:?}"
    );
}

#[tokio::test]
async fn build_settings_update_items_emits_extension_fragment_replacements_and_clears() {
    let (mut session, previous_context) = make_session_and_context().await;
    session.services.extensions = prompt_extension_test_registry();
    session
        .services
        .thread_extension_data
        .insert(PromptExtensionDiffState {
            fragments: vec![
                codex_extension_api::PromptFragment::developer_policy("next extension policy"),
                codex_extension_api::PromptFragment::developer_capability(
                    "next extension capabilities",
                ),
                codex_extension_api::PromptFragment::new(
                    codex_extension_api::PromptSlot::ContextualUser,
                    "next extension contextual user",
                ),
            ],
        });
    let mut previous_context_item = previous_context.to_turn_context_item();
    let previous_extension_items = vec![
        ResponseItem::Message {
            id: None,
            role: "developer".to_string(),
            content: vec![
                ContentItem::InputText {
                    text: ExtensionPromptFragment::new(
                        ExtensionPromptSlot::DeveloperPolicy,
                        "old extension policy",
                    )
                    .render(),
                },
                ContentItem::InputText {
                    text: ExtensionPromptFragment::new(
                        ExtensionPromptSlot::DeveloperCapabilities,
                        "old extension capabilities",
                    )
                    .render(),
                },
            ],
            phase: None,
        },
        ResponseItem::Message {
            id: None,
            role: "developer".to_string(),
            content: vec![ContentItem::InputText {
                text: ExtensionPromptFragment::new(
                    ExtensionPromptSlot::SeparateDeveloper,
                    "old extension separate developer",
                )
                .render(),
            }],
            phase: None,
        },
        ResponseItem::Message {
            id: None,
            role: "user".to_string(),
            content: vec![ContentItem::InputText {
                text: ExtensionPromptFragment::new(
                    ExtensionPromptSlot::ContextualUser,
                    "old extension contextual user",
                )
                .render(),
            }],
            phase: None,
        },
    ];
    previous_context_item.context_manifest =
        crate::context_manager::manifest::build_turn_context_manifest(&previous_extension_items);

    let update_items = session
        .build_settings_update_items(Some(&previous_context_item), &previous_context)
        .await;
    let developer_texts = developer_input_texts(&update_items);
    let user_texts = user_input_texts(&update_items);
    let manifest = crate::context_manager::manifest::build_turn_context_manifest(&update_items)
        .expect("extension updates should produce a manifest");

    assert!(
        developer_texts.iter().any(|text| {
            text.contains("<extension_developer_policy>") && text.contains("next extension policy")
        }),
        "expected extension policy replacement, got {developer_texts:?}"
    );
    assert!(
        developer_texts.iter().any(|text| {
            text.contains("<extension_developer_capabilities>")
                && text.contains("next extension capabilities")
        }),
        "expected extension capabilities replacement, got {developer_texts:?}"
    );
    assert!(
        developer_texts.iter().any(|text| {
            text.contains("<extension_separate_developer>")
                && text.contains(
                    "extension separate developer extension prompt fragments were cleared",
                )
        }),
        "expected extension separate-developer clear, got {developer_texts:?}"
    );
    assert!(
        user_texts.iter().any(|text| {
            text.contains("<extension_contextual_user>")
                && text.contains("next extension contextual user")
        }),
        "expected extension contextual-user replacement, got {user_texts:?}"
    );
    assert_eq!(
        manifest
            .entries
            .iter()
            .map(|entry| entry.source.as_str())
            .collect::<Vec<_>>(),
        vec![
            "turn_context:developer:extension_developer_policy:0:0",
            "turn_context:developer:extension_developer_capabilities:0:1",
            "turn_context:developer:extension_separate_developer:0:2",
            "turn_context:contextual_user:extension_contextual_user:1",
        ]
    );
    assert_eq!(
        manifest
            .decision_ledger
            .iter()
            .map(|entry| entry.decision.as_str())
            .collect::<Vec<_>>(),
        vec![
            "included:extension_prompt",
            "included:extension_prompt",
            "included:extension_prompt",
            "included:extension_prompt",
            "policy:non_omitting_replay_baseline:within_budget",
        ]
    );
    assert!(manifest.has_replay_integrity());

    previous_context_item.context_manifest = Some(manifest);
    let repeated_update_items = session
        .build_settings_update_items(Some(&previous_context_item), &previous_context)
        .await;
    assert!(
        repeated_update_items.is_empty(),
        "unchanged extension fragments and already-cleared source should not emit again: {repeated_update_items:?}"
    );
}

#[tokio::test]
async fn build_initial_context_adds_multi_agent_v2_root_usage_hint_as_developer_message() {
    let (session, turn_context) =
        make_multi_agent_v2_usage_hint_test_session(/*enable_multi_agent_v2*/ true).await;

    let initial_context = session.build_initial_context(turn_context.as_ref()).await;

    let developer_messages = developer_message_texts(&initial_context);
    assert!(
        developer_messages.iter().any(|message| {
            message.len() == 1
                && message[0].contains("<multi_agent_usage_hint>")
                && message[0].contains("Root guidance.")
        }),
        "expected standalone root usage hint developer message, got {developer_messages:?}"
    );
    assert!(
        !developer_messages.iter().any(|message| message
            .iter()
            .any(|text| text.contains("Subagent guidance."))),
        "did not expect subagent usage hint for root thread, got {developer_messages:?}"
    );
    let manifest = crate::context_manager::manifest::build_turn_context_manifest(&initial_context)
        .expect("initial context should produce a manifest");
    assert!(
        manifest
            .entries
            .iter()
            .any(|entry| entry.source.contains(":multi_agent_usage_hint:")),
        "expected multi-agent usage hint source in manifest, got {manifest:?}"
    );
}

#[tokio::test]
async fn build_initial_context_adds_multi_agent_v2_subagent_usage_hint_as_developer_message() {
    let (session, mut turn_context) =
        make_multi_agent_v2_usage_hint_test_session(/*enable_multi_agent_v2*/ true).await;
    let session_source = SessionSource::SubAgent(SubAgentSource::ThreadSpawn {
        parent_thread_id: ThreadId::new(),
        depth: 1,
        agent_path: Some(AgentPath::try_from("/root/worker").expect("agent path should parse")),
        agent_nickname: Some("worker".to_string()),
        agent_role: None,
    });
    session
        .state
        .lock()
        .await
        .session_configuration
        .session_source = session_source.clone();
    Arc::get_mut(&mut turn_context)
        .expect("turn context should not be shared")
        .session_source = session_source;

    let initial_context = session.build_initial_context(turn_context.as_ref()).await;

    let developer_messages = developer_message_texts(&initial_context);
    assert!(
        developer_messages.iter().any(|message| {
            message.len() == 1
                && message[0].contains("<multi_agent_usage_hint>")
                && message[0].contains("Subagent guidance.")
        }),
        "expected standalone subagent usage hint developer message, got {developer_messages:?}"
    );
    assert!(
        !developer_messages
            .iter()
            .any(|message| message.iter().any(|text| text.contains("Root guidance."))),
        "did not expect root usage hint for subagent thread, got {developer_messages:?}"
    );
    let manifest = crate::context_manager::manifest::build_turn_context_manifest(&initial_context)
        .expect("initial context should produce a manifest");
    assert!(
        manifest
            .entries
            .iter()
            .any(|entry| entry.source.contains(":multi_agent_usage_hint:")),
        "expected multi-agent usage hint source in manifest, got {manifest:?}"
    );
}

#[tokio::test]
async fn build_initial_context_omits_multi_agent_v2_usage_hints_when_feature_disabled() {
    let (session, turn_context) =
        make_multi_agent_v2_usage_hint_test_session(/*enable_multi_agent_v2*/ false).await;

    let initial_context = session.build_initial_context(turn_context.as_ref()).await;

    let developer_messages = developer_message_texts(&initial_context);
    assert!(
        !developer_messages.iter().any(|message| {
            matches!(
                message.as_slice(),
                ["Root guidance."] | ["Subagent guidance."]
            )
        }),
        "did not expect multi-agent v2 usage hint developer messages, got {developer_messages:?}"
    );
}

#[tokio::test]
async fn build_settings_update_items_diffs_multi_agent_v2_usage_hint_changes() {
    let (session, turn_context) =
        make_multi_agent_v2_usage_hint_test_session(/*enable_multi_agent_v2*/ true).await;
    let mut previous_context_item = turn_context.to_turn_context_item();
    let stale_hint_item = ResponseItem::Message {
        id: None,
        role: "developer".to_string(),
        content: vec![ContentItem::InputText {
            text: multi_agents::render_usage_hint("Stale root guidance."),
        }],
        phase: None,
    };
    previous_context_item.context_manifest =
        crate::context_manager::manifest::build_turn_context_manifest(&[stale_hint_item]);

    let update_items = session
        .build_settings_update_items(Some(&previous_context_item), &turn_context)
        .await;
    let developer_texts = developer_input_texts(&update_items);
    let manifest = crate::context_manager::manifest::build_turn_context_manifest(&update_items)
        .expect("multi-agent usage hint update should produce a manifest");

    assert!(
        developer_texts.iter().any(|text| {
            text.contains("<multi_agent_usage_hint>")
                && text.contains("Root guidance.")
                && !text.contains("Stale root guidance.")
        }),
        "expected current multi-agent usage hint update, got {developer_texts:?}"
    );
    assert_eq!(
        manifest
            .entries
            .iter()
            .map(|entry| entry.source.as_str())
            .collect::<Vec<_>>(),
        vec!["turn_context:developer:multi_agent_usage_hint:0"]
    );
    assert!(manifest.has_replay_integrity());
}

#[tokio::test]
async fn build_settings_update_items_diffs_cleared_multi_agent_v2_usage_hint_once() {
    let (session, turn_context) =
        make_multi_agent_v2_usage_hint_test_session(/*enable_multi_agent_v2*/ false).await;
    let mut previous_context_item = turn_context.to_turn_context_item();
    let previous_hint_item = ResponseItem::Message {
        id: None,
        role: "developer".to_string(),
        content: vec![ContentItem::InputText {
            text: multi_agents::render_usage_hint("Previously active root guidance."),
        }],
        phase: None,
    };
    previous_context_item.context_manifest =
        crate::context_manager::manifest::build_turn_context_manifest(&[previous_hint_item]);

    let update_items = session
        .build_settings_update_items(Some(&previous_context_item), &turn_context)
        .await;
    let developer_texts = developer_input_texts(&update_items);
    let manifest = crate::context_manager::manifest::build_turn_context_manifest(&update_items)
        .expect("multi-agent usage hint clear should produce a manifest");

    assert!(
        developer_texts.iter().any(|text| {
            text.contains("<multi_agent_usage_hint>")
                && text.contains("Multi-agent usage hint was cleared")
        }),
        "expected multi-agent usage hint clear, got {developer_texts:?}"
    );
    assert_eq!(
        manifest
            .entries
            .iter()
            .map(|entry| entry.source.as_str())
            .collect::<Vec<_>>(),
        vec!["turn_context:developer:multi_agent_usage_hint:0"]
    );
    assert!(manifest.has_replay_integrity());

    previous_context_item.context_manifest = Some(manifest);
    let repeated_update_items = session
        .build_settings_update_items(Some(&previous_context_item), &turn_context)
        .await;
    assert!(
        repeated_update_items.is_empty(),
        "already-cleared multi-agent usage hint should not emit again: {repeated_update_items:?}"
    );
}

#[tokio::test]
async fn configured_multi_agent_v2_usage_hint_texts_use_effective_enabled_feature_state() {
    let (mut session, _turn_context) =
        make_multi_agent_v2_usage_hint_test_session(/*enable_multi_agent_v2*/ false).await;
    let mut effective_features = Features::with_defaults();
    effective_features.enable(Feature::MultiAgentV2);
    Arc::get_mut(&mut session)
        .expect("session should not be shared")
        .features = effective_features.into();

    let hint_texts = session.configured_multi_agent_v2_usage_hint_texts().await;

    assert_eq!(
        hint_texts,
        vec![
            "Root guidance.".to_string(),
            "Subagent guidance.".to_string()
        ]
    );
}

#[tokio::test]
async fn configured_multi_agent_v2_usage_hint_texts_omit_effectively_disabled_feature() {
    let (mut session, _turn_context) =
        make_multi_agent_v2_usage_hint_test_session(/*enable_multi_agent_v2*/ true).await;
    Arc::get_mut(&mut session)
        .expect("session should not be shared")
        .features = Features::with_defaults().into();

    let hint_texts = session.configured_multi_agent_v2_usage_hint_texts().await;

    assert_eq!(hint_texts, Vec::<String>::new());
}

#[tokio::test]
async fn build_initial_context_omits_default_image_save_location_with_image_history() {
    let (session, turn_context) = make_session_and_context().await;
    session
        .replace_history(
            vec![ResponseItem::ImageGenerationCall {
                id: "ig-test".to_string(),
                status: "completed".to_string(),
                revised_prompt: Some("a tiny blue square".to_string()),
                result: "Zm9v".to_string(),
            }],
            /*reference_context_item*/ None,
        )
        .await;

    let initial_context = session.build_initial_context(&turn_context).await;
    let developer_texts = developer_input_texts(&initial_context);
    assert!(
        !developer_texts
            .iter()
            .any(|text| text.contains("Generated images are saved to")),
        "expected initial context to omit image save instructions even with image history, got {developer_texts:?}"
    );
}

#[tokio::test]
async fn build_initial_context_omits_default_image_save_location_without_image_history() {
    let (session, turn_context) = make_session_and_context().await;

    let initial_context = session.build_initial_context(&turn_context).await;
    let developer_texts = developer_input_texts(&initial_context);

    assert!(
        !developer_texts
            .iter()
            .any(|text| text.contains("Generated images are saved to")),
        "expected initial context to omit image save instructions without image history, got {developer_texts:?}"
    );
}

#[tokio::test]
async fn build_initial_context_trims_skill_metadata_from_context_window_budget() {
    let (session, mut turn_context) = make_session_and_context().await;
    let mut outcome = SkillLoadOutcome::default();
    outcome.skills = vec![
        SkillMetadata {
            name: "admin-skill".to_string(),
            description: "desc".to_string(),
            short_description: None,
            interface: None,
            dependencies: None,
            policy: None,
            path_to_skills_md: test_path_buf("/tmp/admin-skill/SKILL.md").abs(),
            scope: SkillScope::Admin,
            plugin_id: None,
        },
        SkillMetadata {
            name: "repo-skill".to_string(),
            description: "desc".to_string(),
            short_description: None,
            interface: None,
            dependencies: None,
            policy: None,
            path_to_skills_md: test_path_buf("/tmp/repo-skill/SKILL.md").abs(),
            scope: SkillScope::Repo,
            plugin_id: None,
        },
    ];
    turn_context.model_info.context_window = Some(100);
    turn_context.turn_skills = TurnSkillsContext::new(Arc::new(outcome));

    let initial_context = session.build_initial_context(&turn_context).await;
    let developer_texts = developer_input_texts(&initial_context);

    assert!(
        developer_texts
            .iter()
            .all(|text| !text.contains("Exceeded skills context budget")),
        "expected skill budget warning to stay out of the initial context, got {developer_texts:?}"
    );
    assert!(
        developer_texts
            .iter()
            .all(|text| !text.contains("- admin-skill:") && !text.contains("- repo-skill:")),
        "expected no skill metadata entries to fit the tiny budget, got {developer_texts:?}"
    );
}

#[test]
fn emit_thread_start_skill_metrics_records_enabled_kept_and_truncated_values() {
    let session_telemetry = test_session_telemetry_without_metadata();
    let mut outcome = SkillLoadOutcome::default();
    outcome.skills = vec![SkillMetadata {
        name: "repo-skill".to_string(),
        description: "desc".to_string(),
        short_description: None,
        interface: None,
        dependencies: None,
        policy: None,
        path_to_skills_md: test_path_buf("/tmp/repo-skill/SKILL.md").abs(),
        scope: SkillScope::Repo,
        plugin_id: None,
    }];
    let rendered = build_available_skills(
        &outcome,
        SkillMetadataBudget::Characters(1),
        SkillRenderSideEffects::ThreadStart {
            session_telemetry: &session_telemetry,
        },
    )
    .expect("skills should render");

    assert_eq!(
        rendered.warning_message,
        Some(
            "Exceeded skills context budget. All skill descriptions were removed and 1 additional skill was not included in the model-visible skills list."
                .to_string()
        )
    );
    let snapshot = session_telemetry
        .snapshot_metrics()
        .expect("runtime metrics snapshot");
    assert_eq!(
        histogram_sum(&snapshot, THREAD_SKILLS_ENABLED_TOTAL_METRIC),
        1
    );
    assert_eq!(histogram_sum(&snapshot, THREAD_SKILLS_KEPT_TOTAL_METRIC), 0);
    assert_eq!(histogram_sum(&snapshot, THREAD_SKILLS_TRUNCATED_METRIC), 1);
    assert_eq!(
        histogram_sum(&snapshot, THREAD_SKILLS_DESCRIPTION_TRUNCATED_CHARS_METRIC),
        4
    );
}

#[test]
fn emit_thread_start_skill_metrics_records_description_truncated_chars_without_omitted_skills() {
    let session_telemetry = test_session_telemetry_without_metadata();
    let alpha = SkillMetadata {
        name: "alpha-skill".to_string(),
        description: "abcdef".to_string(),
        short_description: None,
        interface: None,
        dependencies: None,
        policy: None,
        path_to_skills_md: test_path_buf("/tmp/alpha-skill/SKILL.md").abs(),
        scope: SkillScope::Repo,
        plugin_id: None,
    };
    let beta = SkillMetadata {
        name: "beta-skill".to_string(),
        description: "uvwxyz".to_string(),
        short_description: None,
        interface: None,
        dependencies: None,
        policy: None,
        path_to_skills_md: test_path_buf("/tmp/beta-skill/SKILL.md").abs(),
        scope: SkillScope::Repo,
        plugin_id: None,
    };
    let minimum_skill_line_cost = |skill: &SkillMetadata| {
        let path = skill.path_to_skills_md.to_string_lossy().replace('\\', "/");
        format!("- {}: (file: {})\n", skill.name, path)
            .chars()
            .count()
    };
    let minimum_budget = minimum_skill_line_cost(&alpha) + minimum_skill_line_cost(&beta);
    let mut outcome = SkillLoadOutcome::default();
    outcome.skills = vec![alpha, beta];

    let rendered = build_available_skills(
        &outcome,
        SkillMetadataBudget::Characters(minimum_budget + 6),
        SkillRenderSideEffects::ThreadStart {
            session_telemetry: &session_telemetry,
        },
    )
    .expect("skills should render");

    assert_eq!(rendered.report.omitted_count, 0);
    assert_eq!(rendered.report.truncated_description_chars, 8);
    let snapshot = session_telemetry
        .snapshot_metrics()
        .expect("runtime metrics snapshot");
    assert_eq!(histogram_sum(&snapshot, THREAD_SKILLS_TRUNCATED_METRIC), 0);
    assert_eq!(
        histogram_sum(&snapshot, THREAD_SKILLS_DESCRIPTION_TRUNCATED_CHARS_METRIC),
        8
    );
}

#[tokio::test]
async fn build_initial_context_emits_thread_start_skill_warning_on_repeated_builds() {
    let (session, turn_context, rx) = make_session_and_context_with_rx().await;
    let mut turn_context = Arc::into_inner(turn_context).expect("sole turn context owner");
    let mut outcome = SkillLoadOutcome::default();
    outcome.skills = vec![
        SkillMetadata {
            name: "admin-skill".to_string(),
            description: "desc".to_string(),
            short_description: None,
            interface: None,
            dependencies: None,
            policy: None,
            path_to_skills_md: test_path_buf("/tmp/admin-skill/SKILL.md").abs(),
            scope: SkillScope::Admin,
            plugin_id: None,
        },
        SkillMetadata {
            name: "repo-skill".to_string(),
            description: "desc".to_string(),
            short_description: None,
            interface: None,
            dependencies: None,
            policy: None,
            path_to_skills_md: test_path_buf("/tmp/repo-skill/SKILL.md").abs(),
            scope: SkillScope::Repo,
            plugin_id: None,
        },
    ];
    turn_context.model_info.context_window = Some(100);
    turn_context.turn_skills = TurnSkillsContext::new(Arc::new(outcome));

    let _ = session.build_initial_context(&turn_context).await;
    let warning_event = timeout(Duration::from_secs(1), rx.recv())
        .await
        .expect("warning event should arrive")
        .expect("warning event should be readable");
    assert!(matches!(
        warning_event.msg,
        EventMsg::Warning(WarningEvent { message })
            if message == "Exceeded skills context budget of 2%. All skill descriptions were removed and 2 additional skills were not included in the model-visible skills list."
    ));

    let _ = session.build_initial_context(&turn_context).await;
    let warning_event = timeout(Duration::from_secs(1), rx.recv())
        .await
        .expect("warning event should arrive on repeated build")
        .expect("warning event should be readable");
    assert!(matches!(
        warning_event.msg,
        EventMsg::Warning(WarningEvent { message })
            if message == "Exceeded skills context budget of 2%. All skill descriptions were removed and 2 additional skills were not included in the model-visible skills list."
    ));
}

#[tokio::test]
async fn handle_output_item_done_records_image_save_history_message() {
    let (session, turn_context) = make_session_and_context().await;
    let session = Arc::new(session);
    let turn_context = Arc::new(turn_context);
    let call_id = "ig_history_records_message";
    let expected_saved_path = crate::stream_events_utils::image_generation_artifact_path(
        &turn_context.config.codex_home,
        &session.conversation_id.to_string(),
        call_id,
    );
    let _ = std::fs::remove_file(&expected_saved_path);
    let item = ResponseItem::ImageGenerationCall {
        id: call_id.to_string(),
        status: "completed".to_string(),
        revised_prompt: Some("a tiny blue square".to_string()),
        result: "Zm9v".to_string(),
    };

    let mut ctx = HandleOutputCtx {
        sess: Arc::clone(&session),
        turn_context: Arc::clone(&turn_context),
        turn_store: Arc::new(codex_extension_api::ExtensionData::new(
            turn_context.sub_id.clone(),
        )),
        tool_runtime: test_tool_runtime(Arc::clone(&session), Arc::clone(&turn_context)),
        cancellation_token: CancellationToken::new(),
    };
    handle_output_item_done(&mut ctx, item.clone(), /*previously_active_item*/ None)
        .await
        .expect("image generation item should succeed");

    let history = session.clone_history().await;
    let image_output_path = crate::stream_events_utils::image_generation_artifact_path(
        &turn_context.config.codex_home,
        &session.conversation_id.to_string(),
        "<image_id>",
    );
    let image_output_dir = image_output_path
        .parent()
        .expect("generated image path should have a parent");
    let image_message: ResponseItem = crate::context::ContextualUserFragment::into(
        crate::context::ImageGenerationInstructions::new(
            image_output_dir.display(),
            image_output_path.display(),
        ),
    );
    assert_eq!(history.raw_items(), &[image_message, item]);
    assert_eq!(
        std::fs::read(&expected_saved_path).expect("saved file"),
        b"foo"
    );
    let _ = std::fs::remove_file(&expected_saved_path);
}

#[tokio::test]
async fn handle_output_item_done_skips_image_save_message_when_save_fails() {
    let (session, turn_context) = make_session_and_context().await;
    let session = Arc::new(session);
    let turn_context = Arc::new(turn_context);
    let call_id = "ig_history_no_message";
    let expected_saved_path = crate::stream_events_utils::image_generation_artifact_path(
        &turn_context.config.codex_home,
        &session.conversation_id.to_string(),
        call_id,
    );
    let _ = std::fs::remove_file(&expected_saved_path);
    let item = ResponseItem::ImageGenerationCall {
        id: call_id.to_string(),
        status: "completed".to_string(),
        revised_prompt: Some("broken payload".to_string()),
        result: "_-8".to_string(),
    };

    let mut ctx = HandleOutputCtx {
        sess: Arc::clone(&session),
        turn_context: Arc::clone(&turn_context),
        turn_store: Arc::new(codex_extension_api::ExtensionData::new(
            turn_context.sub_id.clone(),
        )),
        tool_runtime: test_tool_runtime(Arc::clone(&session), Arc::clone(&turn_context)),
        cancellation_token: CancellationToken::new(),
    };
    handle_output_item_done(&mut ctx, item.clone(), /*previously_active_item*/ None)
        .await
        .expect("image generation item should still complete");

    let history = session.clone_history().await;
    assert_eq!(history.raw_items(), &[item]);
    assert!(!expected_saved_path.exists());
}

#[tokio::test]
async fn build_initial_context_uses_previous_turn_settings_for_realtime_end() {
    let (session, turn_context) = make_session_and_context().await;
    let previous_turn_settings = PreviousTurnSettings {
        model: turn_context.model_info.slug.clone(),
        realtime_active: Some(true),
    };

    session
        .set_previous_turn_settings(Some(previous_turn_settings))
        .await;
    let initial_context = session.build_initial_context(&turn_context).await;
    let developer_texts = developer_input_texts(&initial_context);
    assert!(
        developer_texts
            .iter()
            .any(|text| text.contains("Reason: inactive")),
        "expected initial context to describe an ended realtime session, got {developer_texts:?}"
    );
}

#[tokio::test]
async fn build_initial_context_restates_realtime_start_when_reference_context_is_missing() {
    let (session, mut turn_context) = make_session_and_context().await;
    turn_context.realtime_active = true;
    let previous_turn_settings = PreviousTurnSettings {
        model: turn_context.model_info.slug.clone(),
        realtime_active: Some(true),
    };

    session
        .set_previous_turn_settings(Some(previous_turn_settings))
        .await;
    let initial_context = session.build_initial_context(&turn_context).await;
    let developer_texts = developer_input_texts(&initial_context);
    assert!(
        developer_texts
            .iter()
            .any(|text| text.contains("<realtime_conversation>")),
        "expected initial context to restate active realtime when the reference context is missing, got {developer_texts:?}"
    );
}

fn file_system_policy_with_unreadable_glob(turn_context: &TurnContext) -> FileSystemSandboxPolicy {
    #[allow(deprecated)]
    let mut policy = FileSystemSandboxPolicy::from_legacy_sandbox_policy_for_cwd(
        &turn_context.sandbox_policy(),
        &turn_context.cwd,
    );
    #[allow(deprecated)]
    let cwd_display = turn_context.cwd.as_path().display().to_string();
    policy.entries.push(FileSystemSandboxEntry {
        path: FileSystemPath::GlobPattern {
            pattern: format!("{cwd_display}/**/*.env"),
        },
        access: FileSystemAccessMode::None,
    });
    policy
}

#[tokio::test]
async fn turn_context_item_omits_legacy_equivalent_file_system_sandbox_policy() {
    let (_session, turn_context) = make_session_and_context().await;

    let item = turn_context.to_turn_context_item();

    assert_eq!(item.file_system_sandbox_policy, None);
    assert_eq!(
        item.permission_profile,
        Some(turn_context.permission_profile())
    );
}

#[tokio::test]
async fn turn_context_item_stores_split_file_system_sandbox_policy_when_different() {
    let (_session, mut turn_context) = make_session_and_context().await;
    let file_system_sandbox_policy = file_system_policy_with_unreadable_glob(&turn_context);
    turn_context.permission_profile = PermissionProfile::from_runtime_permissions_with_enforcement(
        turn_context.permission_profile.enforcement(),
        &file_system_sandbox_policy,
        turn_context.network_sandbox_policy(),
    );

    let item = turn_context.to_turn_context_item();

    assert_eq!(
        item.file_system_sandbox_policy,
        Some(file_system_sandbox_policy)
    );
    assert_eq!(
        item.permission_profile,
        Some(turn_context.permission_profile())
    );
}

#[tokio::test]
async fn record_context_updates_and_set_reference_context_item_injects_full_context_when_baseline_missing()
 {
    let (session, turn_context) = make_session_and_context().await;
    session
        .record_context_updates_and_set_reference_context_item(&turn_context)
        .await;
    let history = session.clone_history().await;
    let initial_context = session.build_initial_context(&turn_context).await;
    assert_eq!(history.raw_items().to_vec(), initial_context);

    let current_context = session.reference_context_item().await;
    let mut expected_context_item = turn_context.to_turn_context_item();
    let assembly_policy =
        crate::context_manager::manifest::ContextAssemblyPolicy::from_model_context_window(
            turn_context.model_context_window(),
        );
    expected_context_item.context_manifest =
        crate::context_manager::manifest::build_turn_context_manifest_with_policy(
            &initial_context,
            &assembly_policy,
        );
    assert_eq!(
        serde_json::to_value(current_context).expect("serialize current context item"),
        serde_json::to_value(Some(expected_context_item)).expect("serialize expected context item")
    );
}

#[tokio::test]
async fn record_context_updates_and_set_reference_context_item_persists_shadow_context_manifest() {
    let (mut session, turn_context) = make_session_and_context().await;
    let rollout_path = attach_thread_persistence(&mut session).await;
    let initial_context = session.build_initial_context(&turn_context).await;
    let first_context_text = initial_context.iter().find_map(|item| {
        let ResponseItem::Message { content, .. } = item else {
            return None;
        };
        content.iter().find_map(|content_item| {
            let ContentItem::InputText { text } = content_item else {
                return None;
            };
            Some(text.as_str())
        })
    });

    session
        .record_context_updates_and_set_reference_context_item(&turn_context)
        .await;

    let current_context = session
        .reference_context_item()
        .await
        .expect("reference context item should be set");
    let current_manifest = current_context
        .context_manifest
        .as_ref()
        .expect("context manifest should be attached");
    assert!(current_manifest.has_replay_integrity());
    assert!(!current_manifest.entries.is_empty());
    assert_eq!(
        current_manifest.decision_ledger.len(),
        current_manifest.entries.len() + 1
    );
    assert!(current_manifest.budget_tokens.is_some());
    assert_eq!(current_manifest.omitted_entries, 0);
    assert!(current_manifest.omitted_sources.is_empty());
    assert!(!current_manifest.truncated);
    assert!(current_manifest.decision_ledger.iter().any(|entry| {
        entry.source == "turn_context:assembly_policy"
            && entry
                .decision
                .starts_with("policy:non_omitting_replay_baseline:")
    }));
    assert!(
        current_manifest
            .decision_ledger
            .iter()
            .all(|entry| entry.reason_hash.is_some())
    );
    assert!(
        current_manifest
            .entries
            .iter()
            .all(|entry| entry.source.starts_with("turn_context:"))
    );

    let manifest_json =
        serde_json::to_string(current_manifest).expect("context manifest should serialize");
    if let Some(first_context_text) = first_context_text {
        assert!(!manifest_json.contains(first_context_text));
    }

    session.ensure_rollout_materialized().await;
    session.flush_rollout().await.expect("rollout should flush");

    let InitialHistory::Resumed(resumed) = RolloutRecorder::get_rollout_history(&rollout_path)
        .await
        .expect("read rollout history")
    else {
        panic!("expected resumed rollout history");
    };
    let persisted_manifest = resumed.history.iter().find_map(|item| match item {
        RolloutItem::TurnContext(ctx) => ctx.context_manifest.clone(),
        _ => None,
    });

    assert_eq!(persisted_manifest.as_ref(), Some(current_manifest));
}

#[tokio::test]
async fn record_context_updates_and_set_reference_context_item_carries_shadow_manifest_without_diffs()
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

    session
        .record_context_updates_and_set_reference_context_item(&turn_context)
        .await;

    assert_eq!(
        session.clone_history().await.raw_items().len(),
        history_after_first
    );
    assert_eq!(
        session
            .reference_context_item()
            .await
            .and_then(|item| item.context_manifest),
        Some(first_manifest)
    );
}

#[tokio::test]
async fn record_context_updates_and_set_reference_context_item_persists_recall_provider_rollup_in_shadow_manifest()
 {
    let (session, turn_context) = make_session_and_context().await;
    let recall_selection = codex_protocol::protocol::TurnContextRecallSelectionSummary {
        returned_source_count: 4,
        selected_source_count: 3,
        ranked_source_count: 2,
        returned_unselected_source_count: 1,
        source_diversity_met: true,
        source_diversity_target: 3,
        max_per_source: 2,
        ranked_item_count: 4,
        omitted_by_budget_count: 1,
        memory_control_omitted_count: 2,
        low_trust_ranked_item_count: 1,
        low_recency_ranked_item_count: 1,
    };

    session
        .record_context_updates_and_set_reference_context_item_with_manifest_options(
            &turn_context,
            crate::context_manager::manifest::TurnContextManifestOptions {
                recall_provider_rollup: Some(
                    crate::context_manager::manifest::ContextRecallProviderRollup {
                        recall_selection: recall_selection.clone(),
                    },
                ),
                recall_selected_snippets: None,
                memory_taxonomy: Vec::new(),
                memory_formation_receipts: Vec::new(),
                memory_temporal_facts: Vec::new(),
            },
        )
        .await;

    let current_context = session
        .reference_context_item()
        .await
        .expect("reference context item should be set");
    let current_manifest = current_context
        .context_manifest
        .as_ref()
        .expect("context manifest should be attached");
    let manifest_value =
        serde_json::to_value(current_manifest).expect("context manifest should serialize");

    assert!(current_manifest.has_replay_integrity());
    assert_eq!(
        current_manifest.recall_selection.as_ref(),
        Some(&recall_selection)
    );
    assert!(
        manifest_value["recall_selection"]
            .get("source_id")
            .is_none()
    );
    assert!(manifest_value["recall_selection"].get("summary").is_none());
    assert!(manifest_value["recall_selection"].get("snippet").is_none());
}

#[tokio::test]
async fn record_context_updates_and_set_reference_context_item_updates_recall_provider_rollup_without_context_diffs()
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
    let recall_selection = codex_protocol::protocol::TurnContextRecallSelectionSummary {
        returned_source_count: 2,
        selected_source_count: 2,
        ranked_source_count: 0,
        returned_unselected_source_count: 0,
        source_diversity_met: true,
        source_diversity_target: 2,
        max_per_source: 2,
        ranked_item_count: 0,
        omitted_by_budget_count: 0,
        memory_control_omitted_count: 1,
        low_trust_ranked_item_count: 0,
        low_recency_ranked_item_count: 0,
    };

    session
        .record_context_updates_and_set_reference_context_item_with_manifest_options(
            &turn_context,
            crate::context_manager::manifest::TurnContextManifestOptions {
                recall_provider_rollup: Some(
                    crate::context_manager::manifest::ContextRecallProviderRollup {
                        recall_selection: recall_selection.clone(),
                    },
                ),
                recall_selected_snippets: None,
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
    assert_eq!(
        session.clone_history().await.raw_items().len(),
        history_after_first
    );
    assert_eq!(
        current_manifest.recall_selection.as_ref(),
        Some(&recall_selection)
    );
    assert_ne!(current_manifest.ledger_hash, first_manifest.ledger_hash);
}

#[tokio::test]
async fn record_context_updates_and_set_reference_context_item_consumes_turn_scoped_recall_provider_rollup()
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
    let recall_selection = codex_protocol::protocol::TurnContextRecallSelectionSummary {
        returned_source_count: 3,
        selected_source_count: 3,
        ranked_source_count: 3,
        returned_unselected_source_count: 0,
        source_diversity_met: true,
        source_diversity_target: 3,
        max_per_source: 2,
        ranked_item_count: 4,
        omitted_by_budget_count: 1,
        memory_control_omitted_count: 2,
        low_trust_ranked_item_count: 1,
        low_recency_ranked_item_count: 1,
    };
    turn_context.extension_data.insert(recall_selection.clone());

    session
        .record_context_updates_and_set_reference_context_item(&turn_context)
        .await;

    let current_manifest = session
        .reference_context_item()
        .await
        .and_then(|item| item.context_manifest)
        .expect("context manifest should be retained");
    let manifest_json =
        serde_json::to_string(&current_manifest).expect("context manifest should serialize");

    assert_eq!(
        session.clone_history().await.raw_items().len(),
        history_after_first
    );
    assert_eq!(
        current_manifest.recall_selection.as_ref(),
        Some(&recall_selection)
    );
    assert_ne!(current_manifest.ledger_hash, first_manifest.ledger_hash);
    assert!(!manifest_json.contains("source_id"));
    assert!(!manifest_json.contains("snippet"));
}

#[tokio::test]
async fn record_context_updates_and_set_reference_context_item_consumes_turn_scoped_recall_rollup_and_selected_snippets_without_drift()
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
    let recall_selection = codex_protocol::protocol::TurnContextRecallSelectionSummary {
        returned_source_count: 4,
        selected_source_count: 3,
        ranked_source_count: 3,
        returned_unselected_source_count: 1,
        source_diversity_met: true,
        source_diversity_target: 3,
        max_per_source: 2,
        ranked_item_count: 5,
        omitted_by_budget_count: 1,
        memory_control_omitted_count: 2,
        low_trust_ranked_item_count: 1,
        low_recency_ranked_item_count: 1,
    };
    let selected_snippets = test_recall_selected_snippet_envelope();
    turn_context.extension_data.insert(recall_selection.clone());
    turn_context
        .extension_data
        .insert(selected_snippets.clone());

    session
        .record_context_updates_and_set_reference_context_item(&turn_context)
        .await;

    let current_history = session.clone_history().await;
    let current_manifest = session
        .reference_context_item()
        .await
        .and_then(|item| item.context_manifest)
        .expect("combined handoff manifest should be retained");
    let history_json =
        serde_json::to_string(current_history.raw_items()).expect("history should serialize");
    let manifest_json =
        serde_json::to_string(&current_manifest).expect("manifest should serialize");

    assert_eq!(current_history.raw_items().len(), history_after_first + 1);
    assert_eq!(
        current_manifest.recall_selection.as_ref(),
        Some(&recall_selection)
    );
    assert_eq!(
        current_manifest.recall_selected_snippets.as_ref(),
        Some(&selected_snippets)
    );
    assert_ne!(current_manifest.ledger_hash, first_manifest.ledger_hash);
    assert_eq!(history_json.matches("<selected_context_recall>").count(), 1);
    assert!(history_json.contains("[redacted-query] bounded memory"));
    assert!(history_json.contains("fedcba9876543210"));
    assert!(!manifest_json.contains("source-memory-id"));
    assert!(!manifest_json.contains("[hepta-memory:"));
    assert!(!manifest_json.contains("needle"));
    assert!(!history_json.contains("source-memory-id"));
    assert!(!history_json.contains("source_id"));
    assert!(!history_json.contains("[hepta-memory:"));
    assert!(!history_json.contains("needle"));

    let refreshed_recall_selection = codex_protocol::protocol::TurnContextRecallSelectionSummary {
        returned_source_count: 3,
        selected_source_count: 2,
        ranked_source_count: 2,
        returned_unselected_source_count: 1,
        source_diversity_met: true,
        source_diversity_target: 2,
        max_per_source: 2,
        ranked_item_count: 4,
        omitted_by_budget_count: 2,
        memory_control_omitted_count: 1,
        low_trust_ranked_item_count: 0,
        low_recency_ranked_item_count: 1,
    };
    turn_context
        .extension_data
        .insert(refreshed_recall_selection.clone());
    let history_after_combined = current_history.raw_items().len();

    session
        .record_context_updates_and_set_reference_context_item(&turn_context)
        .await;

    let refreshed_history = session.clone_history().await;
    let refreshed_manifest = session
        .reference_context_item()
        .await
        .and_then(|item| item.context_manifest)
        .expect("refreshed rollup manifest should be retained");
    let refreshed_history_json =
        serde_json::to_string(refreshed_history.raw_items()).expect("history should serialize");

    assert_eq!(refreshed_history.raw_items().len(), history_after_combined);
    assert_eq!(
        refreshed_manifest.recall_selection.as_ref(),
        Some(&refreshed_recall_selection)
    );
    assert_eq!(
        refreshed_manifest.recall_selected_snippets.as_ref(),
        Some(&selected_snippets)
    );
    assert_eq!(
        refreshed_history_json
            .matches("<selected_context_recall>")
            .count(),
        1
    );
    assert!(!refreshed_history_json.contains("source-memory-id"));
    assert!(!refreshed_history_json.contains("[hepta-memory:"));
    assert!(!refreshed_history_json.contains("needle"));
}

#[tokio::test]
async fn record_context_updates_and_set_reference_context_item_injects_guarded_selected_snippets_without_context_diffs()
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

    session
        .record_context_updates_and_set_reference_context_item_with_manifest_options(
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
    let manifest_json =
        serde_json::to_string(&current_manifest).expect("manifest should serialize");

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
    assert!(!manifest_json.contains("source-memory-id"));
    assert!(!manifest_json.contains("[hepta-memory:"));
    assert!(!manifest_json.contains("needle"));
}

#[tokio::test]
async fn record_context_updates_and_set_reference_context_item_with_source_aware_policy_filters_omitted_prompt_fragments()
 {
    let (mut session, turn_context) = make_session_and_context().await;
    let rollout_path = attach_thread_persistence(&mut session).await;
    let selected_snippets = test_recall_selected_snippet_envelope();
    let assembly_policy =
        crate::context_manager::manifest::ContextAssemblyPolicy::source_aware_omission_for_model_context_window(
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
    let history_json =
        serde_json::to_string(current_history.raw_items()).expect("history should serialize");
    let current_context = session
        .reference_context_item()
        .await
        .expect("reference context item should be set");
    let current_manifest = current_context
        .context_manifest
        .as_ref()
        .expect("context manifest should be attached");
    let selected_recall_omitted = current_manifest
        .omitted_sources
        .iter()
        .any(|source| source.contains(":selected_context_recall:"));

    assert!(current_manifest.has_replay_integrity());
    assert_eq!(
        current_manifest.recall_selected_snippets.as_ref(),
        Some(&selected_snippets)
    );
    assert!(selected_recall_omitted);
    assert!(
        !current_manifest
            .entries
            .iter()
            .any(|entry| entry.source.contains(":selected_context_recall:"))
    );
    assert!(current_manifest.decision_ledger.iter().any(|entry| {
        entry.source.contains(":selected_context_recall:")
            && entry
                .decision
                .starts_with("omitted:selected_context_recall:")
    }));
    assert!(!history_json.contains("<selected_context_recall>"));
    assert!(!history_json.contains("fedcba9876543210"));
    assert!(!history_json.contains("[redacted-query] bounded memory"));

    session.ensure_rollout_materialized().await;
    session.flush_rollout().await.expect("rollout should flush");

    let InitialHistory::Resumed(resumed) = RolloutRecorder::get_rollout_history(&rollout_path)
        .await
        .expect("read rollout history")
    else {
        panic!("expected resumed rollout history");
    };
    let persisted_manifest = resumed.history.iter().find_map(|item| match item {
        RolloutItem::TurnContext(ctx) => ctx.context_manifest.clone(),
        _ => None,
    });

    assert_eq!(persisted_manifest.as_ref(), Some(current_manifest));
}

#[tokio::test]
async fn record_context_updates_and_set_reference_context_item_with_source_aware_policy_truncates_prompt_fragments()
 {
    let (mut session, turn_context) = make_session_and_context().await;
    let rollout_path = attach_thread_persistence(&mut session).await;
    session
        .record_context_updates_and_set_reference_context_item(&turn_context)
        .await;
    let history_after_baseline = session.clone_history().await.raw_items().len();
    let selected_snippets = test_recall_selected_snippet_envelope();
    let assembly_policy =
        crate::context_manager::manifest::ContextAssemblyPolicy::source_aware_omission_and_truncation_for_model_context_window(
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
    let history_json =
        serde_json::to_string(current_history.raw_items()).expect("history should serialize");
    let current_context = session
        .reference_context_item()
        .await
        .expect("reference context item should be set");
    let current_manifest = current_context
        .context_manifest
        .as_ref()
        .expect("context manifest should be attached");
    let selected_recall_entry = current_manifest
        .entries
        .iter()
        .find(|entry| entry.source.contains(":selected_context_recall:"))
        .expect("selected recall should remain as a truncated manifest entry");

    assert_eq!(
        current_history.raw_items().len(),
        history_after_baseline + 1
    );
    assert!(current_manifest.has_replay_integrity());
    assert_eq!(
        current_manifest.recall_selected_snippets.as_ref(),
        Some(&selected_snippets)
    );
    assert!(current_manifest.truncated);
    assert_eq!(current_manifest.omitted_entries, 0);
    assert!(current_manifest.omitted_sources.is_empty());
    assert!(history_json.contains("<selected_context_recall>"));
    assert!(history_json.contains("[context truncated for budget]"));
    assert!(!history_json.contains("fedcba9876543210"));
    assert!(!history_json.contains("[redacted-query] bounded memory"));
    assert!(current_manifest.decision_ledger.iter().any(|entry| {
        entry.source == selected_recall_entry.source
            && entry
                .decision
                .starts_with("truncated:selected_context_recall:original_tokens:")
            && entry.reason_hash.is_some()
    }));

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

    assert_eq!(persisted_manifest.as_ref(), Some(current_manifest));
}

#[tokio::test]
async fn record_context_updates_and_set_reference_context_item_with_source_aware_policy_summarizes_prompt_fragments()
 {
    let (mut session, turn_context) = make_session_and_context().await;
    let rollout_path = attach_thread_persistence(&mut session).await;
    session
        .record_context_updates_and_set_reference_context_item(&turn_context)
        .await;
    let history_after_baseline = session.clone_history().await.raw_items().len();
    let mut selected_snippets = test_recall_selected_snippet_envelope();
    selected_snippets.snippets[0].text =
        "bounded memory summary with project preference, recent context details, and durable recall notes"
            .into();
    let assembly_policy =
        crate::context_manager::manifest::ContextAssemblyPolicy::source_aware_summary_for_model_context_window(
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
    let selected_recall_text = developer_input_texts(current_history.raw_items())
        .into_iter()
        .find(|text| text.starts_with("<selected_context_recall>"))
        .expect("selected recall should remain as summarized prompt text");
    let current_context = session
        .reference_context_item()
        .await
        .expect("reference context item should be set");
    let current_manifest = current_context
        .context_manifest
        .as_ref()
        .expect("context manifest should be attached");
    let selected_recall_entry = current_manifest
        .entries
        .iter()
        .find(|entry| entry.source.contains(":selected_context_recall:"))
        .expect("selected recall should remain as a summarized manifest entry");
    let expected_text_hash = codex_protocol::protocol::stable_turn_context_manifest_text_hash(
        &format!("text:{selected_recall_text}\n"),
    );

    assert_eq!(
        current_history.raw_items().len(),
        history_after_baseline + 1
    );
    assert!(current_manifest.has_replay_integrity());
    assert_eq!(
        current_manifest.recall_selected_snippets.as_ref(),
        Some(&selected_snippets)
    );
    assert!(selected_recall_text.contains("[context summarized for budget]"));
    assert!(!selected_recall_text.contains("fedcba9876543210"));
    assert!(!selected_recall_text.contains("bounded memory summary"));
    assert!(!current_manifest.truncated);
    assert_eq!(current_manifest.omitted_entries, 0);
    assert!(current_manifest.omitted_sources.is_empty());
    assert_eq!(current_manifest.compression_stages.len(), 1);
    assert_eq!(
        current_manifest.compression_stages[0].kind,
        codex_protocol::protocol::TurnContextCompressionStageKind::Summary
    );
    assert_eq!(
        current_manifest.compression_stages[0].output_tokens,
        selected_recall_entry.estimated_tokens
    );
    assert!(current_manifest.compression_stages[0].tokens_saved() > 0);
    assert_eq!(selected_recall_entry.text_hash, expected_text_hash);
    assert!(
        !current_manifest
            .compression_candidates
            .iter()
            .any(|candidate| candidate.source_id == "selected_context_recall")
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
    let persisted_selected_recall_text = developer_input_texts(&persisted_response_items)
        .into_iter()
        .find(|text| text.starts_with("<selected_context_recall>"))
        .expect("persisted history should retain summarized selected recall");

    assert_eq!(persisted_manifest.as_ref(), Some(current_manifest));
    assert_eq!(persisted_selected_recall_text, selected_recall_text);
}

#[tokio::test]
async fn record_context_updates_and_set_reference_context_item_with_source_aware_policy_defragments_tool_inventory_prompt_fragments()
 {
    let (mut session, turn_context) = make_session_and_context().await;
    let mut previous_context_item = turn_context.to_turn_context_item();
    let previous_tool_inventory_items = vec![ResponseItem::Message {
        id: None,
        role: "developer".to_string(),
        content: vec![ContentItem::InputText {
            text: tagged_context_fragment(
                codex_protocol::protocol::PLUGINS_INSTRUCTIONS_OPEN_TAG,
                codex_protocol::protocol::PLUGINS_INSTRUCTIONS_CLOSE_TAG,
                "Previously visible plugins with repeated capability metadata for shell, files, docs, and search.",
            ),
        }],
        phase: None,
    }];
    previous_context_item.context_manifest =
        crate::context_manager::manifest::build_turn_context_manifest(
            &previous_tool_inventory_items,
        );
    {
        let mut state = session.state.lock().await;
        state.set_reference_context_item(Some(previous_context_item));
    }
    let rollout_path = attach_thread_persistence(&mut session).await;
    let assembly_policy =
        crate::context_manager::manifest::ContextAssemblyPolicy::source_aware_tool_defragment_for_model_context_window(
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
    let defragmented_plugins_text = developer_input_texts(current_history.raw_items())
        .into_iter()
        .find(|text| text.starts_with(codex_protocol::protocol::PLUGINS_INSTRUCTIONS_OPEN_TAG))
        .expect("available plugins should remain as defragmented prompt text");
    let current_context = session
        .reference_context_item()
        .await
        .expect("reference context item should be set");
    let current_manifest = current_context
        .context_manifest
        .as_ref()
        .expect("context manifest should be attached");
    let plugins_entry = current_manifest
        .entries
        .iter()
        .find(|entry| entry.source.contains(":available_plugins:"))
        .expect("available plugins should remain as a defragmented manifest entry");
    let expected_text_hash = codex_protocol::protocol::stable_turn_context_manifest_text_hash(
        &format!("text:{defragmented_plugins_text}\n"),
    );

    assert_eq!(current_history.raw_items().len(), 1);
    assert!(current_manifest.has_replay_integrity());
    assert!(defragmented_plugins_text.contains("[context defragmented for budget]"));
    assert!(
        !defragmented_plugins_text.contains("Available plugins capability inventory was cleared")
    );
    assert!(!defragmented_plugins_text.contains("Previously visible plugins"));
    assert!(!current_manifest.truncated);
    assert_eq!(current_manifest.omitted_entries, 0);
    assert!(current_manifest.omitted_sources.is_empty());
    assert_eq!(current_manifest.compression_stages.len(), 1);
    assert_eq!(
        current_manifest.compression_stages[0].kind,
        codex_protocol::protocol::TurnContextCompressionStageKind::Defragment
    );
    assert_eq!(
        current_manifest.compression_stages[0].output_tokens,
        plugins_entry.estimated_tokens
    );
    assert!(current_manifest.compression_stages[0].tokens_saved() > 0);
    assert_eq!(plugins_entry.text_hash, expected_text_hash);
    assert!(
        !current_manifest
            .compression_candidates
            .iter()
            .any(|candidate| candidate.source_id == "available_plugins")
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
    let persisted_plugins_text = developer_input_texts(&persisted_response_items)
        .into_iter()
        .find(|text| text.starts_with(codex_protocol::protocol::PLUGINS_INSTRUCTIONS_OPEN_TAG))
        .expect("persisted history should retain defragmented plugins inventory");

    assert_eq!(persisted_manifest.as_ref(), Some(current_manifest));
    assert_eq!(persisted_plugins_text, defragmented_plugins_text);
}
