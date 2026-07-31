#[tokio::test]
async fn test_precedence_fixture_with_gpt5_profile() -> std::io::Result<()> {
    let fixture = create_test_fixture()?;

    let gpt5_profile_overrides = ConfigOverrides {
        config_profile: Some("gpt5".to_string()),
        cwd: Some(fixture.cwd_path()),
        ..Default::default()
    };
    let gpt5_profile_config = Config::load_from_base_config_with_overrides(
        fixture.cfg.clone(),
        gpt5_profile_overrides,
        fixture.codex_home(),
    )
    .await?;
    let expected_gpt5_profile_config = Config {
        config_generation: ConfigGeneration::default(),
        model: Some("gpt-5.4".to_string()),
        review_model: None,
        model_context_window: None,
        model_auto_compact_token_limit: None,
        service_tier: None,
        model_provider_id: "openai".to_string(),
        model_provider: fixture.openai_provider.clone(),
        permissions: Permissions {
            approval_policy: Constrained::allow_any(AskForApproval::OnFailure),
            permission_profile_state: active_permission_profile_state(
                PermissionProfile::read_only(),
                BUILT_IN_PERMISSION_PROFILE_READ_ONLY,
            ),
            workspace_roots: vec![fixture.cwd()],
            network: None,
            allow_login_shell: true,
            shell_environment_policy: ShellEnvironmentPolicy::default(),
            windows_sandbox_mode: None,
            windows_sandbox_private_desktop: true,
        },
        approvals_reviewer: ApprovalsReviewer::User,
        enforce_residency: Constrained::allow_any(/*initial_value*/ None),
        user_instructions: None,
        notify: None,
        cwd: fixture.cwd(),
        workspace_roots: vec![fixture.cwd()],
        workspace_roots_explicit: false,
        cli_auth_credentials_store_mode: Default::default(),
        mcp_servers: Constrained::allow_any(HashMap::new()),
        mcp_oauth_credentials_store_mode: resolve_mcp_oauth_credentials_store_mode(
            Default::default(),
            LOCAL_DEV_BUILD_VERSION,
        ),
        mcp_oauth_callback_port: None,
        mcp_oauth_callback_url: None,
        model_providers: fixture.model_provider_map.clone(),
        project_doc_max_bytes: AGENTS_MD_MAX_BYTES,
        project_doc_fallback_filenames: Vec::new(),
        tool_output_token_limit: None,
        agent_max_threads: DEFAULT_AGENT_MAX_THREADS,
        agent_max_depth: DEFAULT_AGENT_MAX_DEPTH,
        agent_roles: BTreeMap::new(),
        memories: MemoriesConfig::default(),
        agent_job_max_runtime_seconds: DEFAULT_AGENT_JOB_MAX_RUNTIME_SECONDS,
        agent_interrupt_message_enabled: true,
        codex_home: fixture.codex_home(),
        sqlite_home: fixture.codex_home().to_path_buf(),
        log_dir: fixture.codex_home().join("log").to_path_buf(),
        config_lock_export_dir: None,
        config_lock_allow_codex_version_mismatch: false,
        config_lock_save_fields_resolved_from_model_catalog: true,
        config_lock_toml: None,
        config_layer_stack: Default::default(),
        startup_warnings: Vec::new(),
        history: History::default(),
        ephemeral: false,
        bypass_hook_trust: false,
        file_opener: UriBasedFileOpener::VsCode,
        codex_self_exe: None,
        codex_linux_sandbox_exe: None,
        main_execve_wrapper_exe: None,
        zsh_path: None,
        hide_agent_reasoning: false,
        show_raw_agent_reasoning: false,
        model_reasoning_effort: Some(ReasoningEffort::High),
        plan_mode_reasoning_effort: None,
        model_reasoning_summary: Some(ReasoningSummary::Detailed),
        model_supports_reasoning_summaries: None,
        model_catalog: None,
        model_verbosity: Some(Verbosity::High),
        personality: Some(Personality::Pragmatic),
        chatgpt_base_url: "https://chatgpt.com/backend-api/".to_string(),
        apps_mcp_path_override: None,
        apps_mcp_product_sku: None,
        realtime_audio: RealtimeAudioConfig::default(),
        experimental_realtime_start_instructions: None,
        experimental_realtime_ws_base_url: None,
        experimental_realtime_ws_model: None,
        realtime: RealtimeConfig::default(),
        experimental_realtime_ws_backend_prompt: None,
        experimental_realtime_ws_startup_context: None,
        experimental_thread_config_endpoint: None,
        experimental_thread_store: ThreadStoreConfig::Local,
        base_instructions: None,
        developer_instructions: None,
        guardian_policy_config: None,
        include_permissions_instructions: true,
        include_apps_instructions: true,
        include_collaboration_mode_instructions: true,
        include_skill_instructions: true,
        include_environment_context: true,
        compact_prompt: None,
        forced_chatgpt_workspace_id: None,
        forced_login_method: None,
        web_search_mode: Constrained::allow_any(WebSearchMode::Cached),
        web_search_config: None,
        update_plan_enabled: true,
        use_experimental_unified_exec_tool: !cfg!(windows),
        background_terminal_max_timeout: DEFAULT_MAX_BACKGROUND_TERMINAL_TIMEOUT_MS,
        ghost_snapshot: GhostSnapshotConfig::default(),
        multi_agent_v2: MultiAgentV2Config::default(),
        token_budget: None,
        token_budget_has_explicit_settings: false,
        features: Features::with_defaults().into(),
        suppress_unstable_features_warning: false,
        active_profile: Some("gpt5".to_string()),
        active_project: ProjectConfig { trust_level: None },
        notices: Default::default(),
        check_for_update_on_startup: true,
        disable_paste_burst: false,
        tui_notifications: Default::default(),
        animations: true,
        show_tooltips: true,
        tui_vim_mode_default: false,
        tui_raw_output_mode: false,
        tui_keymap: TuiKeymap::default(),
        model_availability_nux: ModelAvailabilityNuxConfig::default(),
        terminal_resize_reflow: TerminalResizeReflowConfig::default(),
        analytics_enabled: Some(true),
        feedback_enabled: true,
        tool_suggest: ToolSuggestConfig::default(),
        tui_alternate_screen: AltScreenMode::Auto,
        tui_status_line: None,
        tui_status_line_use_colors: true,
        tui_terminal_title: None,
        tui_theme: None,
        tui_pet: None,
        tui_pet_anchor: TuiPetAnchor::Composer,
        tui_session_picker_view: SessionPickerViewMode::Dense,
        otel: OtelConfig::default(),
    };

    assert_eq!(expected_gpt5_profile_config, gpt5_profile_config);

    Ok(())
}

#[tokio::test]
async fn test_requirements_web_search_mode_allowlist_does_not_warn_when_unset() -> anyhow::Result<()>
{
    let fixture = create_test_fixture()?;

    let requirements_toml = codex_config::ConfigRequirementsToml {
        allowed_approval_policies: None,
        allowed_approvals_reviewers: None,
        allowed_sandbox_modes: None,
        remote_sandbox_config: None,
        allowed_web_search_modes: Some(vec![codex_config::WebSearchModeRequirement::Cached]),
        allow_managed_hooks_only: None,
        feature_requirements: None,
        hooks: None,
        mcp_servers: None,
        plugins: None,
        marketplaces: None,
        apps: None,
        rules: None,
        enforce_residency: None,
        network: None,
        permissions: None,
        guardian_policy_config: None,
    };
    let requirement_source = codex_config::RequirementSource::Unknown;
    let requirement_source_for_error = requirement_source.clone();
    let allowed = vec![WebSearchMode::Disabled, WebSearchMode::Cached];
    let constrained = Constrained::new(WebSearchMode::Cached, move |candidate| {
        if matches!(candidate, WebSearchMode::Cached | WebSearchMode::Disabled) {
            Ok(())
        } else {
            Err(ConstraintError::InvalidValue {
                field_name: "web_search_mode",
                candidate: format!("{candidate:?}"),
                allowed: format!("{allowed:?}"),
                requirement_source: requirement_source_for_error.clone(),
            })
        }
    })?;
    let requirements = codex_config::ConfigRequirements {
        web_search_mode: codex_config::ConstrainedWithSource::new(
            constrained,
            Some(requirement_source),
        ),
        ..Default::default()
    };
    let config_layer_stack =
        codex_config::ConfigLayerStack::new(Vec::new(), requirements, requirements_toml)
            .expect("config layer stack");

    let config = Config::load_config_with_layer_stack(
        LOCAL_FS.as_ref(),
        fixture.cfg.clone(),
        ConfigOverrides {
            cwd: Some(fixture.cwd_path()),
            ..Default::default()
        },
        fixture.codex_home(),
        config_layer_stack,
    )
    .await?;

    assert!(
        !config
            .startup_warnings
            .iter()
            .any(|warning| warning.contains("Configured value for `web_search_mode`")),
        "{:?}",
        config.startup_warnings
    );

    Ok(())
}

#[test]
fn test_set_project_trusted_writes_explicit_tables() -> anyhow::Result<()> {
    let project_dir = Path::new("/some/path");
    let mut doc = DocumentMut::new();

    set_project_trust_level_inner(&mut doc, project_dir, TrustLevel::Trusted)?;

    let contents = doc.to_string();

    let raw_path = project_dir.to_string_lossy();
    let path_str = if raw_path.contains('\\') {
        format!("'{raw_path}'")
    } else {
        format!("\"{raw_path}\"")
    };
    let expected = format!(
        r#"[projects.{path_str}]
trust_level = "trusted"
"#
    );
    assert_eq!(contents, expected);

    Ok(())
}

#[test]
fn test_set_project_trusted_converts_inline_to_explicit() -> anyhow::Result<()> {
    let project_dir = Path::new("/some/path");

    // Seed config.toml with an inline project entry under [projects]
    let raw_path = project_dir.to_string_lossy();
    let path_str = if raw_path.contains('\\') {
        format!("'{raw_path}'")
    } else {
        format!("\"{raw_path}\"")
    };
    // Use a quoted key so backslashes don't require escaping on Windows
    let initial = format!(
        r#"[projects]
{path_str} = {{ trust_level = "untrusted" }}
"#
    );
    let mut doc = initial.parse::<DocumentMut>()?;

    // Run the function; it should convert to explicit tables and set trusted
    set_project_trust_level_inner(&mut doc, project_dir, TrustLevel::Trusted)?;

    let contents = doc.to_string();

    // Assert exact output after conversion to explicit table
    let expected = format!(
        r#"[projects]

[projects.{path_str}]
trust_level = "trusted"
"#
    );
    assert_eq!(contents, expected);

    Ok(())
}

#[test]
fn test_set_project_trusted_migrates_top_level_inline_projects_preserving_entries()
-> anyhow::Result<()> {
    let initial = r#"toplevel = "baz"
projects = { "/Users/mbolin/code/codex4" = { trust_level = "trusted", foo = "bar" } , "/Users/mbolin/code/codex3" = { trust_level = "trusted" } }
model = "foo""#;
    let mut doc = initial.parse::<DocumentMut>()?;

    // Approve a new directory
    let new_project = Path::new("/Users/mbolin/code/codex2");
    set_project_trust_level_inner(&mut doc, new_project, TrustLevel::Trusted)?;

    let contents = doc.to_string();

    // Since we created the [projects] table as part of migration, it is kept implicit.
    // Expect explicit per-project tables, preserving prior entries and appending the new one.
    let new_project_key = project_trust_key(new_project);
    let expected = format!(
        r#"toplevel = "baz"
model = "foo"

[projects."/Users/mbolin/code/codex4"]
trust_level = "trusted"
foo = "bar"

[projects."/Users/mbolin/code/codex3"]
trust_level = "trusted"

[projects."{new_project_key}"]
trust_level = "trusted"
"#
    );
    assert_eq!(contents, expected);

    Ok(())
}

#[cfg(unix)]
#[tokio::test]
async fn active_project_does_not_match_configured_alias_for_canonical_cwd() -> anyhow::Result<()> {
    let tmp = tempdir()?;
    let project_root = tmp.path().join("project");
    let alias_root = tmp.path().join("project_alias");
    std::fs::create_dir_all(&project_root)?;
    std::os::unix::fs::symlink(&project_root, &alias_root)?;

    let config = ConfigToml {
        projects: Some(HashMap::from([(
            alias_root.to_string_lossy().to_string(),
            ProjectConfig {
                trust_level: Some(TrustLevel::Trusted),
            },
        )])),
        ..Default::default()
    };

    assert_eq!(
        config.get_active_project(&project_root, /*repo_root*/ None),
        None
    );

    Ok(())
}

#[test]
fn test_set_default_oss_provider() -> std::io::Result<()> {
    let temp_dir = TempDir::new()?;
    let codex_home = temp_dir.path();
    let config_path = codex_home.join(CONFIG_TOML_FILE);

    // Test setting valid provider on empty config
    set_default_oss_provider(codex_home, OLLAMA_OSS_PROVIDER_ID)?;
    let content = std::fs::read_to_string(&config_path)?;
    assert!(content.contains("oss_provider = \"ollama\""));

    // Test updating existing config
    std::fs::write(&config_path, "model = \"gpt-4\"\n")?;
    set_default_oss_provider(codex_home, LMSTUDIO_OSS_PROVIDER_ID)?;
    let content = std::fs::read_to_string(&config_path)?;
    assert!(content.contains("oss_provider = \"lmstudio\""));
    assert!(content.contains("model = \"gpt-4\""));

    // Test overwriting existing oss_provider
    set_default_oss_provider(codex_home, OLLAMA_OSS_PROVIDER_ID)?;
    let content = std::fs::read_to_string(&config_path)?;
    assert!(content.contains("oss_provider = \"ollama\""));
    assert!(!content.contains("oss_provider = \"lmstudio\""));

    // Test invalid provider
    let result = set_default_oss_provider(codex_home, "invalid_provider");
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput);
    assert!(error.to_string().contains("Invalid OSS provider"));
    assert!(error.to_string().contains("invalid_provider"));

    Ok(())
}

#[test]
fn test_set_default_oss_provider_rejects_legacy_ollama_chat_provider() -> std::io::Result<()> {
    let temp_dir = TempDir::new()?;
    let codex_home = temp_dir.path();

    let result = set_default_oss_provider(codex_home, LEGACY_OLLAMA_CHAT_PROVIDER_ID);
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput);
    assert!(
        error
            .to_string()
            .contains(OLLAMA_CHAT_PROVIDER_REMOVED_ERROR)
    );

    Ok(())
}

#[tokio::test]
async fn test_load_config_rejects_legacy_ollama_chat_provider_with_helpful_error()
-> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let cfg = ConfigToml {
        model_provider: Some(LEGACY_OLLAMA_CHAT_PROVIDER_ID.to_string()),
        ..Default::default()
    };

    let result = Config::load_from_base_config_with_overrides(
        cfg,
        ConfigOverrides::default(),
        codex_home.abs(),
    )
    .await;
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert_eq!(error.kind(), std::io::ErrorKind::NotFound);
    assert!(
        error
            .to_string()
            .contains(OLLAMA_CHAT_PROVIDER_REMOVED_ERROR)
    );

    Ok(())
}

#[tokio::test]
async fn test_untrusted_project_gets_workspace_write_sandbox() -> anyhow::Result<()> {
    let config_with_untrusted = r#"
[projects."/tmp/test"]
trust_level = "untrusted"
"#;

    let cfg = toml::from_str::<ConfigToml>(config_with_untrusted)
        .expect("TOML deserialization should succeed");
    let active_project = ProjectConfig {
        trust_level: Some(TrustLevel::Untrusted),
    };

    let resolution = derive_legacy_sandbox_policy_for_test(
        &cfg,
        /*sandbox_mode_override*/ None,
        /*profile_sandbox_mode*/ None,
        WindowsSandboxLevel::Disabled,
        Some(&active_project),
        /*permission_profile_constraint*/ None,
    )
    .await;

    // Verify that untrusted projects get WorkspaceWrite (or ReadOnly on Windows due to downgrade)
    if cfg!(target_os = "windows") {
        assert!(
            matches!(resolution, SandboxPolicy::ReadOnly { .. }),
            "Expected ReadOnly on Windows, got {resolution:?}"
        );
    } else {
        assert!(
            matches!(resolution, SandboxPolicy::WorkspaceWrite { .. }),
            "Expected WorkspaceWrite for untrusted project, got {resolution:?}"
        );
    }

    Ok(())
}

#[tokio::test]
async fn derive_sandbox_policy_falls_back_to_read_only_for_implicit_defaults() -> anyhow::Result<()>
{
    let project_dir = TempDir::new()?;
    let project_path = project_dir.path().to_path_buf();
    let project_key = project_path.to_string_lossy().to_string();
    let cfg = ConfigToml {
        projects: Some(HashMap::from([(
            project_key,
            ProjectConfig {
                trust_level: Some(TrustLevel::Trusted),
            },
        )])),
        ..Default::default()
    };
    let active_project = ProjectConfig {
        trust_level: Some(TrustLevel::Trusted),
    };
    let constrained = Constrained::new(PermissionProfile::read_only(), |candidate| {
        if candidate == &PermissionProfile::read_only() {
            Ok(())
        } else {
            Err(ConstraintError::InvalidValue {
                field_name: "sandbox_mode",
                candidate: format!("{candidate:?}"),
                allowed: "[ReadOnly]".to_string(),
                requirement_source: RequirementSource::Unknown,
            })
        }
    })?;

    let resolution = derive_legacy_sandbox_policy_for_test(
        &cfg,
        /*sandbox_mode_override*/ None,
        /*profile_sandbox_mode*/ None,
        WindowsSandboxLevel::Disabled,
        Some(&active_project),
        Some(&constrained),
    )
    .await;

    assert_eq!(resolution, SandboxPolicy::new_read_only_policy());
    Ok(())
}

#[tokio::test]
async fn derive_sandbox_policy_preserves_windows_downgrade_for_unsupported_fallback()
-> anyhow::Result<()> {
    let project_dir = TempDir::new()?;
    let project_path = project_dir.path().to_path_buf();
    let project_key = project_path.to_string_lossy().to_string();
    let cfg = ConfigToml {
        projects: Some(HashMap::from([(
            project_key,
            ProjectConfig {
                trust_level: Some(TrustLevel::Trusted),
            },
        )])),
        ..Default::default()
    };
    let active_project = ProjectConfig {
        trust_level: Some(TrustLevel::Trusted),
    };
    let constrained = Constrained::new(
        PermissionProfile::from_legacy_sandbox_policy(&SandboxPolicy::new_workspace_write_policy()),
        |candidate| {
            if matches!(
                candidate,
                PermissionProfile::Managed {
                    file_system: ManagedFileSystemPermissions::Restricted { entries, .. },
                    ..
                } if entries
                        .iter()
                        .any(|entry| entry.access.can_write())
            ) {
                Ok(())
            } else {
                Err(ConstraintError::InvalidValue {
                    field_name: "sandbox_mode",
                    candidate: format!("{candidate:?}"),
                    allowed: "[WorkspaceWrite]".to_string(),
                    requirement_source: RequirementSource::Unknown,
                })
            }
        },
    )?;

    let resolution = derive_legacy_sandbox_policy_for_test(
        &cfg,
        /*sandbox_mode_override*/ None,
        /*profile_sandbox_mode*/ None,
        WindowsSandboxLevel::Disabled,
        Some(&active_project),
        Some(&constrained),
    )
    .await;

    if cfg!(target_os = "windows") {
        assert_eq!(resolution, SandboxPolicy::new_read_only_policy());
    } else {
        assert_eq!(resolution, SandboxPolicy::new_workspace_write_policy());
    }
    Ok(())
}

#[test]
fn test_resolve_oss_provider_explicit_override() {
    let config_toml = ConfigToml::default();
    let result = resolve_oss_provider(
        Some("custom-provider"),
        &config_toml,
        /*config_profile*/ None,
    );
    assert_eq!(result, Some("custom-provider".to_string()));
}

#[test]
fn test_resolve_oss_provider_from_profile() {
    let mut profiles = std::collections::HashMap::new();
    let profile = ConfigProfile {
        oss_provider: Some("profile-provider".to_string()),
        ..Default::default()
    };
    profiles.insert("test-profile".to_string(), profile);
    let config_toml = ConfigToml {
        profiles,
        ..Default::default()
    };

    let result = resolve_oss_provider(
        /*explicit_provider*/ None,
        &config_toml,
        Some("test-profile".to_string()),
    );
    assert_eq!(result, Some("profile-provider".to_string()));
}

#[test]
fn test_resolve_oss_provider_from_global_config() {
    let config_toml = ConfigToml {
        oss_provider: Some("global-provider".to_string()),
        ..Default::default()
    };

    let result = resolve_oss_provider(
        /*explicit_provider*/ None,
        &config_toml,
        /*config_profile*/ None,
    );
    assert_eq!(result, Some("global-provider".to_string()));
}

#[test]
fn test_resolve_oss_provider_profile_fallback_to_global() {
    let mut profiles = std::collections::HashMap::new();
    let profile = ConfigProfile::default(); // No oss_provider set
    profiles.insert("test-profile".to_string(), profile);
    let config_toml = ConfigToml {
        oss_provider: Some("global-provider".to_string()),
        profiles,
        ..Default::default()
    };

    let result = resolve_oss_provider(
        /*explicit_provider*/ None,
        &config_toml,
        Some("test-profile".to_string()),
    );
    assert_eq!(result, Some("global-provider".to_string()));
}

#[test]
fn test_resolve_oss_provider_none_when_not_configured() {
    let config_toml = ConfigToml::default();
    let result = resolve_oss_provider(
        /*explicit_provider*/ None,
        &config_toml,
        /*config_profile*/ None,
    );
    assert_eq!(result, None);
}

#[test]
fn test_resolve_oss_provider_explicit_overrides_all() {
    let mut profiles = std::collections::HashMap::new();
    let profile = ConfigProfile {
        oss_provider: Some("profile-provider".to_string()),
        ..Default::default()
    };
    profiles.insert("test-profile".to_string(), profile);
    let config_toml = ConfigToml {
        oss_provider: Some("global-provider".to_string()),
        profiles,
        ..Default::default()
    };

    let result = resolve_oss_provider(
        Some("explicit-provider"),
        &config_toml,
        Some("test-profile".to_string()),
    );
    assert_eq!(result, Some("explicit-provider".to_string()));
}

#[test]
fn config_toml_deserializes_mcp_oauth_callback_port() {
    let toml = r#"mcp_oauth_callback_port = 4321"#;
    let cfg: ConfigToml =
        toml::from_str(toml).expect("TOML deserialization should succeed for callback port");
    assert_eq!(cfg.mcp_oauth_callback_port, Some(4321));
}

#[test]
fn config_toml_deserializes_mcp_oauth_callback_url() {
    let toml = r#"mcp_oauth_callback_url = "https://example.com/callback""#;
    let cfg: ConfigToml =
        toml::from_str(toml).expect("TOML deserialization should succeed for callback URL");
    assert_eq!(
        cfg.mcp_oauth_callback_url.as_deref(),
        Some("https://example.com/callback")
    );
}

#[tokio::test]
async fn config_loads_mcp_oauth_callback_port_from_toml() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let toml = r#"
model = "gpt-5.4"
mcp_oauth_callback_port = 5678
"#;
    let cfg: ConfigToml =
        toml::from_str(toml).expect("TOML deserialization should succeed for callback port");

    let config = Config::load_from_base_config_with_overrides(
        cfg,
        ConfigOverrides::default(),
        codex_home.abs(),
    )
    .await?;

    assert_eq!(config.mcp_oauth_callback_port, Some(5678));
    Ok(())
}

#[tokio::test]
async fn config_loads_allow_login_shell_from_toml() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let cfg: ConfigToml = toml::from_str(
        r#"
model = "gpt-5.4"
allow_login_shell = false
"#,
    )
    .expect("TOML deserialization should succeed for allow_login_shell");

    let config = Config::load_from_base_config_with_overrides(
        cfg,
        ConfigOverrides::default(),
        codex_home.abs(),
    )
    .await?;

    assert!(!config.permissions.allow_login_shell);
    Ok(())
}

#[tokio::test]
async fn config_loads_apps_mcp_path_override_from_feature_config() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let toml = r#"
model = "gpt-5.4"

[features.apps_mcp_path_override]
path = "/custom/mcp"
"#;
    let cfg: ConfigToml =
        toml::from_str(toml).expect("TOML deserialization should succeed for apps MCP feature");

    let config = Config::load_from_base_config_with_overrides(
        cfg,
        ConfigOverrides::default(),
        codex_home.abs(),
    )
    .await?;

    assert_eq!(
        config.apps_mcp_path_override.as_deref(),
        Some("/custom/mcp")
    );
    Ok(())
}

#[tokio::test]
async fn config_defaults_enabled_apps_mcp_path_override_to_plugin_service() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let toml = r#"
model = "gpt-5.4"

[features]
apps_mcp_path_override = true
"#;
    let cfg: ConfigToml =
        toml::from_str(toml).expect("TOML deserialization should succeed for apps MCP feature");

    let config = Config::load_from_base_config_with_overrides(
        cfg,
        ConfigOverrides::default(),
        codex_home.abs(),
    )
    .await?;

    assert!(config.features.enabled(Feature::AppsMcpPathOverride));
    assert_eq!(config.apps_mcp_path_override.as_deref(), Some("/ps/mcp"));
    Ok(())
}

#[tokio::test]
async fn config_preserves_explicit_apps_mcp_path_override_path() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let toml = r#"
model = "gpt-5.4"

[features.apps_mcp_path_override]
enabled = true
path = "/custom/mcp"
"#;
    let cfg: ConfigToml =
        toml::from_str(toml).expect("TOML deserialization should succeed for apps MCP feature");

    let config = Config::load_from_base_config_with_overrides(
        cfg,
        ConfigOverrides::default(),
        codex_home.abs(),
    )
    .await?;

    assert_eq!(
        config.apps_mcp_path_override.as_deref(),
        Some("/custom/mcp")
    );
    assert!(config.features.enabled(Feature::AppsMcpPathOverride));
    Ok(())
}

#[tokio::test]
async fn config_loads_apps_mcp_product_sku_from_toml() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let toml = r#"
model = "gpt-5.4"
apps_mcp_product_sku = "tpp"
"#;
    let cfg: ConfigToml =
        toml::from_str(toml).expect("TOML deserialization should succeed for apps MCP SKU");

    let config = Config::load_from_base_config_with_overrides(
        cfg,
        ConfigOverrides::default(),
        codex_home.abs(),
    )
    .await?;

    assert_eq!(config.apps_mcp_product_sku.as_deref(), Some("tpp"));
    Ok(())
}

#[tokio::test]
async fn config_loads_mcp_oauth_callback_url_from_toml() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let toml = r#"
model = "gpt-5.4"
mcp_oauth_callback_url = "https://example.com/callback"
"#;
    let cfg: ConfigToml =
        toml::from_str(toml).expect("TOML deserialization should succeed for callback URL");

    let config = Config::load_from_base_config_with_overrides(
        cfg,
        ConfigOverrides::default(),
        codex_home.abs(),
    )
    .await?;

    assert_eq!(
        config.mcp_oauth_callback_url.as_deref(),
        Some("https://example.com/callback")
    );
    Ok(())
}

#[tokio::test]
async fn test_untrusted_project_gets_unless_trusted_approval_policy() -> anyhow::Result<()> {
    let codex_home = TempDir::new()?;
    let test_project_dir = TempDir::new()?;
    let test_path = test_project_dir.path();

    let config = Config::load_from_base_config_with_overrides(
        ConfigToml {
            projects: Some(HashMap::from([(
                test_path.to_string_lossy().to_string(),
                ProjectConfig {
                    trust_level: Some(TrustLevel::Untrusted),
                },
            )])),
            ..Default::default()
        },
        ConfigOverrides {
            cwd: Some(test_path.to_path_buf()),
            ..Default::default()
        },
        codex_home.abs(),
    )
    .await?;

    // Verify that untrusted projects get UnlessTrusted approval policy
    assert_eq!(
        config.permissions.approval_policy.value(),
        AskForApproval::UnlessTrusted,
        "Expected UnlessTrusted approval policy for untrusted project"
    );

    // Verify that untrusted projects still get WorkspaceWrite sandbox (or ReadOnly on Windows)
    if cfg!(target_os = "windows") {
        assert!(
            matches!(
                &config.legacy_sandbox_policy(),
                SandboxPolicy::ReadOnly { .. }
            ),
            "Expected ReadOnly on Windows"
        );
    } else {
        assert!(
            matches!(
                &config.legacy_sandbox_policy(),
                SandboxPolicy::WorkspaceWrite { .. }
            ),
            "Expected WorkspaceWrite sandbox for untrusted project"
        );
    }

    Ok(())
}

#[tokio::test]
async fn requirements_disallowing_default_sandbox_falls_back_to_required_default()
-> std::io::Result<()> {
    let codex_home = TempDir::new()?;

    let config = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .cloud_requirements(CloudRequirementsLoader::new(async {
            Ok(Some(codex_config::ConfigRequirementsToml {
                allowed_sandbox_modes: Some(vec![codex_config::SandboxModeRequirement::ReadOnly]),
                ..Default::default()
            }))
        }))
        .build()
        .await?;
    assert_eq!(
        config.legacy_sandbox_policy(),
        SandboxPolicy::new_read_only_policy()
    );
    Ok(())
}

#[tokio::test]
async fn explicit_sandbox_mode_falls_back_when_disallowed_by_requirements() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    std::fs::write(
        codex_home.path().join(CONFIG_TOML_FILE),
        r#"sandbox_mode = "danger-full-access"
"#,
    )?;

    let requirements = codex_config::ConfigRequirementsToml {
        allowed_approval_policies: None,
        allowed_approvals_reviewers: None,
        allowed_sandbox_modes: Some(vec![codex_config::SandboxModeRequirement::ReadOnly]),
        remote_sandbox_config: None,
        allowed_web_search_modes: None,
        allow_managed_hooks_only: None,
        feature_requirements: None,
        hooks: None,
        mcp_servers: None,
        plugins: None,
        marketplaces: None,
        apps: None,
        rules: None,
        enforce_residency: None,
        network: None,
        permissions: None,
        guardian_policy_config: None,
    };

    let config = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .fallback_cwd(Some(codex_home.path().to_path_buf()))
        .cloud_requirements(CloudRequirementsLoader::new(async move {
            Ok(Some(requirements))
        }))
        .build()
        .await?;
    assert_eq!(
        config.legacy_sandbox_policy(),
        SandboxPolicy::new_read_only_policy()
    );
    Ok(())
}

#[tokio::test]
async fn permission_profile_override_falls_back_when_disallowed_by_requirements()
-> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let requirements = codex_config::ConfigRequirementsToml {
        allowed_sandbox_modes: Some(vec![codex_config::SandboxModeRequirement::ReadOnly]),
        ..Default::default()
    };

    let config = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .fallback_cwd(Some(codex_home.path().to_path_buf()))
        .harness_overrides(ConfigOverrides {
            permission_profile: Some(PermissionProfile::Disabled),
            ..Default::default()
        })
        .cloud_requirements(CloudRequirementsLoader::new(async move {
            Ok(Some(requirements))
        }))
        .build()
        .await?;

    let expected_sandbox_policy = SandboxPolicy::new_read_only_policy();
    assert_eq!(config.legacy_sandbox_policy(), expected_sandbox_policy);
    assert_eq!(
        config.permissions.effective_permission_profile(),
        PermissionProfile::read_only()
    );
    Ok(())
}

#[tokio::test]
async fn active_profile_is_cleared_when_requirements_force_fallback() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let requirements = codex_config::ConfigRequirementsToml {
        allowed_sandbox_modes: Some(vec![codex_config::SandboxModeRequirement::ReadOnly]),
        ..Default::default()
    };

    let config = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .fallback_cwd(Some(codex_home.path().to_path_buf()))
        .harness_overrides(ConfigOverrides {
            default_permissions: Some(BUILT_IN_PERMISSION_PROFILE_DANGER_FULL_ACCESS.to_string()),
            ..Default::default()
        })
        .cloud_requirements(CloudRequirementsLoader::new(async move {
            Ok(Some(requirements))
        }))
        .build()
        .await?;

    assert_eq!(
        config.permissions.effective_permission_profile(),
        PermissionProfile::read_only()
    );
    assert_eq!(config.permissions.active_permission_profile(), None);
    assert!(
        config.startup_warnings.iter().any(|warning| warning
            .contains("Configured value for `permission_profile` is disallowed by requirements")),
        "{:?}",
        config.startup_warnings
    );
    Ok(())
}

#[tokio::test]
async fn bypass_hook_trust_adds_startup_warning() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;

    let config = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .fallback_cwd(Some(codex_home.path().to_path_buf()))
        .harness_overrides(ConfigOverrides {
            bypass_hook_trust: Some(true),
            ..Default::default()
        })
        .build()
        .await?;

    assert!(
        config.startup_warnings.iter().any(|warning| warning
            == "`--dangerously-bypass-hook-trust` is enabled. Enabled hooks may run without review for this invocation."),
        "{:?}",
        config.startup_warnings
    );
    Ok(())
}

#[tokio::test]
async fn permission_profile_override_preserves_split_write_roots() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let cwd = codex_home.path().join("workspace");
    let outside_root = codex_home.path().join("outside-write");
    std::fs::create_dir_all(&cwd)?;
    std::fs::create_dir_all(&outside_root)?;
    let outside_root =
        AbsolutePathBuf::from_absolute_path(outside_root).expect("outside root is absolute");
    let file_system_sandbox_policy = FileSystemSandboxPolicy::restricted(vec![
        FileSystemSandboxEntry {
            path: FileSystemPath::Special {
                value: FileSystemSpecialPath::Root,
            },
            access: FileSystemAccessMode::Read,
        },
        FileSystemSandboxEntry {
            path: FileSystemPath::Path {
                path: outside_root.clone(),
            },
            access: FileSystemAccessMode::Write,
        },
    ]);
    let permission_profile = PermissionProfile::from_runtime_permissions_with_enforcement(
        SandboxEnforcement::Managed,
        &file_system_sandbox_policy,
        NetworkSandboxPolicy::Restricted,
    );

    let config = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .fallback_cwd(Some(cwd))
        .harness_overrides(ConfigOverrides {
            permission_profile: Some(permission_profile),
            ..Default::default()
        })
        .build()
        .await?;

    assert!(
        config
            .permissions
            .file_system_sandbox_policy()
            .can_write_path_with_cwd(outside_root.as_path(), config.cwd.as_path())
    );
    assert!(matches!(
        &config.legacy_sandbox_policy(),
        SandboxPolicy::WorkspaceWrite { .. }
    ));
    assert_eq!(
        config.permissions.network_sandbox_policy(),
        NetworkSandboxPolicy::Restricted
    );
    Ok(())
}

#[tokio::test]
async fn requirements_web_search_mode_overrides_danger_full_access_default() -> std::io::Result<()>
{
    let codex_home = TempDir::new()?;
    std::fs::write(
        codex_home.path().join(CONFIG_TOML_FILE),
        r#"sandbox_mode = "danger-full-access"
"#,
    )?;

    let config = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .fallback_cwd(Some(codex_home.path().to_path_buf()))
        .cloud_requirements(CloudRequirementsLoader::new(async {
            Ok(Some(codex_config::ConfigRequirementsToml {
                allowed_web_search_modes: Some(vec![
                    codex_config::WebSearchModeRequirement::Cached,
                ]),
                ..Default::default()
            }))
        }))
        .build()
        .await?;

    assert_eq!(config.web_search_mode.value(), WebSearchMode::Cached);
    assert_eq!(
        resolve_web_search_mode_for_turn(
            &config.web_search_mode,
            &config.permissions.effective_permission_profile(),
        ),
        WebSearchMode::Cached,
    );
    Ok(())
}

#[tokio::test]
async fn requirements_disallowing_default_approval_falls_back_to_required_default()
-> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let workspace = TempDir::new()?;
    let workspace_key = workspace.path().to_string_lossy().replace('\\', "\\\\");
    std::fs::write(
        codex_home.path().join(CONFIG_TOML_FILE),
        format!(
            r#"
[projects."{workspace_key}"]
trust_level = "untrusted"
"#
        ),
    )?;

    let config = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .fallback_cwd(Some(workspace.path().to_path_buf()))
        .cloud_requirements(CloudRequirementsLoader::new(async {
            Ok(Some(codex_config::ConfigRequirementsToml {
                allowed_approval_policies: Some(vec![AskForApproval::OnRequest]),
                ..Default::default()
            }))
        }))
        .build()
        .await?;

    assert_eq!(
        config.permissions.approval_policy.value(),
        AskForApproval::OnRequest
    );
    Ok(())
}

#[tokio::test]
async fn explicit_approval_policy_falls_back_when_disallowed_by_requirements() -> std::io::Result<()>
{
    let codex_home = TempDir::new()?;
    std::fs::write(
        codex_home.path().join(CONFIG_TOML_FILE),
        r#"approval_policy = "untrusted"
"#,
    )?;

    let config = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .fallback_cwd(Some(codex_home.path().to_path_buf()))
        .cloud_requirements(CloudRequirementsLoader::new(async {
            Ok(Some(codex_config::ConfigRequirementsToml {
                allowed_approval_policies: Some(vec![AskForApproval::OnRequest]),
                ..Default::default()
            }))
        }))
        .build()
        .await?;
    assert_eq!(
        config.permissions.approval_policy.value(),
        AskForApproval::OnRequest
    );
    Ok(())
}

#[tokio::test]
async fn feature_requirements_normalize_effective_feature_values() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;

    let config = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .cloud_requirements(CloudRequirementsLoader::new(async {
            Ok(Some(codex_config::ConfigRequirementsToml {
                feature_requirements: Some(codex_config::FeatureRequirementsToml {
                    entries: BTreeMap::from([
                        ("personality".to_string(), true),
                        ("shell_tool".to_string(), false),
                    ]),
                }),
                ..Default::default()
            }))
        }))
        .build()
        .await?;

    assert!(config.features.enabled(Feature::Personality));
    assert!(!config.features.enabled(Feature::ShellTool));
    assert!(
        !config
            .startup_warnings
            .iter()
            .any(|warning| warning.contains("Configured value for `features`")),
        "{:?}",
        config.startup_warnings
    );

    Ok(())
}

#[tokio::test]
async fn feature_requirements_auto_review_disables_guardian_approval() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;

    let config = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .cloud_requirements(CloudRequirementsLoader::new(async {
            Ok(Some(codex_config::ConfigRequirementsToml {
                feature_requirements: Some(codex_config::FeatureRequirementsToml {
                    entries: BTreeMap::from([("auto_review".to_string(), false)]),
                }),
                ..Default::default()
            }))
        }))
        .build()
        .await?;

    assert!(!config.features.enabled(Feature::GuardianApproval));

    Ok(())
}

#[tokio::test]
async fn browser_feature_requirements_are_valid() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;

    let config = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .cloud_requirements(CloudRequirementsLoader::new(async {
            Ok(Some(codex_config::ConfigRequirementsToml {
                feature_requirements: Some(codex_config::FeatureRequirementsToml {
                    entries: BTreeMap::from([
                        ("in_app_browser".to_string(), false),
                        ("browser_use".to_string(), false),
                    ]),
                }),
                ..Default::default()
            }))
        }))
        .build()
        .await?;

    assert!(!config.features.enabled(Feature::InAppBrowser));
    assert!(!config.features.enabled(Feature::BrowserUse));

    Ok(())
}

#[tokio::test]
async fn debug_config_lockfile_export_settings_load_from_nested_table() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    std::fs::write(
        codex_home.path().join(CONFIG_TOML_FILE),
        r#"[debug.config_lockfile]
export_dir = "locks"
allow_codex_version_mismatch = true
save_fields_resolved_from_model_catalog = false
"#,
    )?;

    let config = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .fallback_cwd(Some(codex_home.path().to_path_buf()))
        .build()
        .await?;

    assert_eq!(
        config.config_lock_export_dir,
        Some(AbsolutePathBuf::resolve_path_against_base(
            "locks",
            codex_home.path()
        ))
    );
    assert!(config.config_lock_allow_codex_version_mismatch);
    assert!(!config.config_lock_save_fields_resolved_from_model_catalog);

    Ok(())
}

#[tokio::test]
async fn debug_config_lockfile_load_path_loads_lock_from_nested_table() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let lock_path = codex_home.path().join("session.config.lock.toml");
    std::fs::write(
        &lock_path,
        format!(
            r#"version = {}
codex_version = "older-version"

[config]
"#,
            crate::config_lock::CONFIG_LOCK_VERSION
        ),
    )?;
    std::fs::write(
        codex_home.path().join(CONFIG_TOML_FILE),
        format!(
            r#"[debug.config_lockfile]
load_path = '{}'
allow_codex_version_mismatch = true
save_fields_resolved_from_model_catalog = false
"#,
            lock_path.display()
        ),
    )?;

    let config = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .fallback_cwd(Some(codex_home.path().to_path_buf()))
        .build()
        .await?;

    assert!(config.config_lock_toml.is_some());
    assert!(config.config_lock_allow_codex_version_mismatch);
    assert!(!config.config_lock_save_fields_resolved_from_model_catalog);

    Ok(())
}

#[tokio::test]
async fn explicit_feature_config_is_normalized_by_requirements() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    std::fs::write(
        codex_home.path().join(CONFIG_TOML_FILE),
        r#"
[features]
personality = false
shell_tool = true
"#,
    )?;

    let config = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .fallback_cwd(Some(codex_home.path().to_path_buf()))
        .cloud_requirements(CloudRequirementsLoader::new(async {
            Ok(Some(codex_config::ConfigRequirementsToml {
                feature_requirements: Some(codex_config::FeatureRequirementsToml {
                    entries: BTreeMap::from([
                        ("personality".to_string(), true),
                        ("shell_tool".to_string(), false),
                    ]),
                }),
                ..Default::default()
            }))
        }))
        .build()
        .await?;

    assert!(config.features.enabled(Feature::Personality));
    assert!(!config.features.enabled(Feature::ShellTool));
    assert!(
        !config
            .startup_warnings
            .iter()
            .any(|warning| warning.contains("Configured value for `features`")),
        "{:?}",
        config.startup_warnings
    );

    Ok(())
}

#[tokio::test]
async fn approvals_reviewer_defaults_to_manual_only_without_guardian_feature() -> std::io::Result<()>
{
    let codex_home = TempDir::new()?;

    let config = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .fallback_cwd(Some(codex_home.path().to_path_buf()))
        .build()
        .await?;

    assert_eq!(config.approvals_reviewer, ApprovalsReviewer::User);
    Ok(())
}

#[tokio::test]
async fn prompt_instruction_blocks_can_be_disabled_from_config_and_profiles() -> std::io::Result<()>
{
    let codex_home = TempDir::new()?;
    std::fs::write(
        codex_home.path().join(CONFIG_TOML_FILE),
        r#"include_permissions_instructions = false
include_apps_instructions = false
include_collaboration_mode_instructions = false
include_environment_context = false
profile = "chatty"

[skills]
include_instructions = false

[profiles.chatty]
include_permissions_instructions = true
include_collaboration_mode_instructions = true
include_environment_context = true
"#,
    )?;

    let config = ConfigBuilder::default()
        .codex_home(codex_home.path().to_path_buf())
        .fallback_cwd(Some(codex_home.path().to_path_buf()))
        .build()
        .await?;

    assert!(config.include_permissions_instructions);
    assert!(!config.include_apps_instructions);
    assert!(config.include_collaboration_mode_instructions);
    assert!(!config.include_skill_instructions);
    assert!(config.include_environment_context);
    Ok(())
}

#[tokio::test]
async fn approvals_reviewer_stays_manual_only_when_guardian_feature_is_enabled()
-> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    std::fs::write(
        codex_home.path().join(CONFIG_TOML_FILE),
        r#"[features]
guardian_approval = true
"#,
    )?;

    let config = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .fallback_cwd(Some(codex_home.path().to_path_buf()))
        .build()
        .await?;

    assert_eq!(config.approvals_reviewer, ApprovalsReviewer::User);
    Ok(())
}

#[tokio::test]
async fn approvals_reviewer_can_be_set_in_config_without_guardian_approval() -> std::io::Result<()>
{
    let codex_home = TempDir::new()?;
    std::fs::write(
        codex_home.path().join(CONFIG_TOML_FILE),
        r#"approvals_reviewer = "user"
"#,
    )?;

    let config = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .fallback_cwd(Some(codex_home.path().to_path_buf()))
        .build()
        .await?;

    assert_eq!(config.approvals_reviewer, ApprovalsReviewer::User);
    Ok(())
}

#[tokio::test]
async fn approvals_reviewer_can_be_set_in_profile_without_guardian_approval() -> std::io::Result<()>
{
    let codex_home = TempDir::new()?;
    std::fs::write(
        codex_home.path().join(CONFIG_TOML_FILE),
        r#"profile = "guardian"

[profiles.guardian]
approvals_reviewer = "guardian_subagent"
"#,
    )?;

    let config = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .fallback_cwd(Some(codex_home.path().to_path_buf()))
        .build()
        .await?;

    assert_eq!(config.approvals_reviewer, ApprovalsReviewer::AutoReview);
    Ok(())
}

#[tokio::test]
async fn requirements_disallowing_default_approvals_reviewer_falls_back_to_required_default()
-> std::io::Result<()> {
    let codex_home = TempDir::new()?;

    let config = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .cloud_requirements(CloudRequirementsLoader::new(async {
            Ok(Some(codex_config::ConfigRequirementsToml {
                allowed_approvals_reviewers: Some(vec![ApprovalsReviewer::AutoReview]),
                ..Default::default()
            }))
        }))
        .build()
        .await?;

    assert_eq!(config.approvals_reviewer, ApprovalsReviewer::AutoReview);
    Ok(())
}

#[tokio::test]
async fn root_approvals_reviewer_falls_back_when_disallowed_by_requirements() -> std::io::Result<()>
{
    let codex_home = TempDir::new()?;
    std::fs::write(
        codex_home.path().join(CONFIG_TOML_FILE),
        r#"approvals_reviewer = "user"
"#,
    )?;

    let config = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .fallback_cwd(Some(codex_home.path().to_path_buf()))
        .cloud_requirements(CloudRequirementsLoader::new(async {
            Ok(Some(codex_config::ConfigRequirementsToml {
                allowed_approvals_reviewers: Some(vec![ApprovalsReviewer::AutoReview]),
                ..Default::default()
            }))
        }))
        .build()
        .await?;

    assert_eq!(config.approvals_reviewer, ApprovalsReviewer::AutoReview);
    assert!(
        config.startup_warnings.iter().any(|warning| {
            warning
                .contains("Configured value for `approvals_reviewer` is disallowed by requirements")
        }),
        "{:?}",
        config.startup_warnings
    );
    Ok(())
}

#[tokio::test]
async fn profile_approvals_reviewer_falls_back_when_disallowed_by_requirements()
-> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    std::fs::write(
        codex_home.path().join(CONFIG_TOML_FILE),
        r#"profile = "default"

[profiles.default]
approvals_reviewer = "user"
"#,
    )?;

    let config = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .fallback_cwd(Some(codex_home.path().to_path_buf()))
        .cloud_requirements(CloudRequirementsLoader::new(async {
            Ok(Some(codex_config::ConfigRequirementsToml {
                allowed_approvals_reviewers: Some(vec![ApprovalsReviewer::AutoReview]),
                ..Default::default()
            }))
        }))
        .build()
        .await?;

    assert_eq!(config.approvals_reviewer, ApprovalsReviewer::AutoReview);
    Ok(())
}

#[tokio::test]
async fn approvals_reviewer_preserves_valid_user_choice_when_allowed_by_requirements()
-> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    std::fs::write(
        codex_home.path().join(CONFIG_TOML_FILE),
        r#"approvals_reviewer = "guardian_subagent"
"#,
    )?;

    let config = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .fallback_cwd(Some(codex_home.path().to_path_buf()))
        .cloud_requirements(CloudRequirementsLoader::new(async {
            Ok(Some(codex_config::ConfigRequirementsToml {
                allowed_approvals_reviewers: Some(vec![
                    ApprovalsReviewer::User,
                    ApprovalsReviewer::AutoReview,
                ]),
                ..Default::default()
            }))
        }))
        .build()
        .await?;

    assert_eq!(config.approvals_reviewer, ApprovalsReviewer::AutoReview);
    assert!(
        config
            .startup_warnings
            .iter()
            .all(|warning| !warning.contains("approvals_reviewer")),
        "{:?}",
        config.startup_warnings
    );
    Ok(())
}

#[tokio::test]
async fn smart_approvals_alias_is_ignored() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    std::fs::write(
        codex_home.path().join(CONFIG_TOML_FILE),
        r#"[features]
smart_approvals = true
"#,
    )?;

    let config = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .fallback_cwd(Some(codex_home.path().to_path_buf()))
        .build()
        .await?;

    assert!(config.features.enabled(Feature::GuardianApproval));
    assert_eq!(config.approvals_reviewer, ApprovalsReviewer::User);

    let serialized = tokio::fs::read_to_string(codex_home.path().join(CONFIG_TOML_FILE)).await?;
    assert!(serialized.contains("smart_approvals = true"));
    assert!(!serialized.contains("guardian_approval"));
    assert!(!serialized.contains("approvals_reviewer"));

    Ok(())
}

#[tokio::test]
async fn smart_approvals_alias_is_ignored_in_profiles() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    std::fs::write(
        codex_home.path().join(CONFIG_TOML_FILE),
        r#"profile = "guardian"

[profiles.guardian.features]
smart_approvals = true
"#,
    )?;

    let config = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .fallback_cwd(Some(codex_home.path().to_path_buf()))
        .build()
        .await?;

    assert!(config.features.enabled(Feature::GuardianApproval));
    assert_eq!(config.approvals_reviewer, ApprovalsReviewer::User);

    let serialized = tokio::fs::read_to_string(codex_home.path().join(CONFIG_TOML_FILE)).await?;
    assert!(serialized.contains("[profiles.guardian.features]"));
    assert!(serialized.contains("smart_approvals = true"));
    assert!(!serialized.contains("guardian_approval"));
    assert!(!serialized.contains("approvals_reviewer"));

    Ok(())
}

fn model_owned_token_budget_defaults() -> ModelTokenBudgetConfig {
    ModelTokenBudgetConfig {
        reminder_threshold_tokens: 6_144,
        reminder_message_template: "Model reminder: {n_remaining} tokens remain.".to_string(),
        guidance_message: "Preserve durable state before rollover.".to_string(),
        auto_compact_fallback_prompt: "Record the remaining state.".to_string(),
        auto_compact_fallback_buffer_tokens: 16_384,
    }
}

#[tokio::test]
async fn token_budget_uses_model_defaults_only_when_enabled() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    std::fs::write(
        codex_home.path().join(CONFIG_TOML_FILE),
        "[features.token_budget]\nenabled = true\n",
    )?;
    let config = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .fallback_cwd(Some(codex_home.path().to_path_buf()))
        .build()
        .await?;
    let model_defaults = model_owned_token_budget_defaults();

    assert!(!config.has_explicit_token_budget_settings());
    assert_eq!(
        config.resolve_token_budget_with_model_defaults(Some(&model_defaults)),
        Some(TokenBudgetConfig {
            reminder_threshold_tokens: Some(model_defaults.reminder_threshold_tokens),
            reminder_message_template: model_defaults.reminder_message_template,
            guidance_message: Some(model_defaults.guidance_message),
            auto_compact_fallback_prompt: Some(model_defaults.auto_compact_fallback_prompt),
            auto_compact_fallback_buffer_tokens: Some(
                model_defaults.auto_compact_fallback_buffer_tokens
            ),
        })
    );

    let disabled_home = TempDir::new()?;
    let disabled_config = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(disabled_home.path().to_path_buf())
        .fallback_cwd(Some(disabled_home.path().to_path_buf()))
        .build()
        .await?;
    assert_eq!(
        disabled_config
            .resolve_token_budget_with_model_defaults(Some(&model_owned_token_budget_defaults())),
        None
    );

    Ok(())
}

#[tokio::test]
async fn explicit_token_budget_setting_wins_even_when_equal_to_builtin_default()
-> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let default_template = TokenBudgetConfig::default().reminder_message_template;
    std::fs::write(
        codex_home.path().join(CONFIG_TOML_FILE),
        format!(
            "[features.token_budget]\nenabled = true\nreminder_message_template = {default_template:?}\n"
        ),
    )?;
    let config = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .fallback_cwd(Some(codex_home.path().to_path_buf()))
        .build()
        .await?;

    assert!(config.has_explicit_token_budget_settings());
    assert_eq!(
        config.resolve_token_budget_with_model_defaults(Some(&model_owned_token_budget_defaults())),
        config.token_budget
    );
    assert_eq!(
        config
            .token_budget
            .as_ref()
            .and_then(|token_budget| token_budget.guidance_message.as_deref()),
        None
    );

    Ok(())
}

#[tokio::test]
async fn profile_token_budget_fields_override_base_fields() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    std::fs::write(
        codex_home.path().join(CONFIG_TOML_FILE),
        r#"profile = "bounded"

[features.token_budget]
enabled = true
reminder_threshold_tokens = 4096
guidance_message = "base guidance"

[profiles.bounded.features.token_budget]
enabled = true
reminder_threshold_tokens = 2048
"#,
    )?;
    let config = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .fallback_cwd(Some(codex_home.path().to_path_buf()))
        .build()
        .await?;

    assert_eq!(
        config.token_budget,
        Some(TokenBudgetConfig {
            reminder_threshold_tokens: Some(2_048),
            reminder_message_template: DEFAULT_TOKEN_BUDGET_REMINDER_MESSAGE_TEMPLATE.to_string(),
            guidance_message: Some("base guidance".to_string()),
            auto_compact_fallback_prompt: None,
            auto_compact_fallback_buffer_tokens: None,
        })
    );
    assert!(config.has_explicit_token_budget_settings());

    Ok(())
}

#[tokio::test]
async fn invalid_model_owned_token_budget_defaults_are_ignored() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    std::fs::write(
        codex_home.path().join(CONFIG_TOML_FILE),
        "[features.token_budget]\nenabled = true\n",
    )?;
    let config = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .fallback_cwd(Some(codex_home.path().to_path_buf()))
        .build()
        .await?;
    let mut invalid_defaults = model_owned_token_budget_defaults();
    invalid_defaults.reminder_threshold_tokens = 0;

    assert_eq!(
        config.resolve_token_budget_with_model_defaults(Some(&invalid_defaults)),
        config.token_budget
    );

    Ok(())
}

#[tokio::test]
async fn explicit_token_budget_fallback_requires_positive_buffer() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    std::fs::write(
        codex_home.path().join(CONFIG_TOML_FILE),
        r#"[features.token_budget]
enabled = true
auto_compact_fallback_prompt = "Record the remaining state."
"#,
    )?;

    let error = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .fallback_cwd(Some(codex_home.path().to_path_buf()))
        .build()
        .await
        .expect_err("fallback prompt without a buffer should fail");

    assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput);
    assert!(
        error
            .to_string()
            .contains("auto_compact_fallback_buffer_tokens is required")
    );

    Ok(())
}

#[tokio::test]
async fn multi_agent_v2_config_from_feature_table() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    std::fs::write(
        codex_home.path().join(CONFIG_TOML_FILE),
        r#"[features.multi_agent_v2]
enabled = true
max_concurrent_threads_per_session = 5
min_wait_timeout_ms = 2500
max_wait_timeout_ms = 120000
default_wait_timeout_ms = 30000
usage_hint_enabled = false
usage_hint_text = "Custom delegation guidance."
root_agent_usage_hint_text = "Root guidance."
subagent_usage_hint_text = "Subagent guidance."
hide_spawn_agent_metadata = true
non_code_mode_only = true
"#,
    )?;

    let config = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .fallback_cwd(Some(codex_home.path().to_path_buf()))
        .build()
        .await?;

    assert!(config.features.enabled(Feature::MultiAgentV2));
    assert_eq!(config.multi_agent_v2.max_concurrent_threads_per_session, 5);
    assert_eq!(config.multi_agent_v2.min_wait_timeout_ms, 2500);
    assert_eq!(config.multi_agent_v2.max_wait_timeout_ms, 120000);
    assert_eq!(config.multi_agent_v2.default_wait_timeout_ms, 30000);
    assert_eq!(config.agent_max_threads, Some(4));
    assert!(!config.multi_agent_v2.usage_hint_enabled);
    assert_eq!(
        config.multi_agent_v2.usage_hint_text.as_deref(),
        Some("Custom delegation guidance.")
    );
    assert_eq!(
        config.multi_agent_v2.root_agent_usage_hint_text.as_deref(),
        Some("Root guidance.")
    );
    assert_eq!(
        config.multi_agent_v2.subagent_usage_hint_text.as_deref(),
        Some("Subagent guidance.")
    );
    assert!(config.multi_agent_v2.hide_spawn_agent_metadata);
    assert!(config.multi_agent_v2.non_code_mode_only);

    Ok(())
}

#[tokio::test]
async fn profile_multi_agent_v2_config_overrides_base() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    std::fs::write(
        codex_home.path().join(CONFIG_TOML_FILE),
        r#"profile = "no_hint"

[features.multi_agent_v2]
max_concurrent_threads_per_session = 4
min_wait_timeout_ms = 3000
max_wait_timeout_ms = 120000
default_wait_timeout_ms = 30000
usage_hint_enabled = true
usage_hint_text = "base hint"
root_agent_usage_hint_text = "base root hint"
subagent_usage_hint_text = "base subagent hint"
hide_spawn_agent_metadata = true
non_code_mode_only = false

[profiles.no_hint.features.multi_agent_v2]
max_concurrent_threads_per_session = 6
min_wait_timeout_ms = 1500
max_wait_timeout_ms = 90000
default_wait_timeout_ms = 15000
usage_hint_enabled = false
usage_hint_text = "profile hint"
root_agent_usage_hint_text = "profile root hint"
subagent_usage_hint_text = "profile subagent hint"
hide_spawn_agent_metadata = false
non_code_mode_only = true
"#,
    )?;

    let config = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .fallback_cwd(Some(codex_home.path().to_path_buf()))
        .build()
        .await?;

    assert_eq!(config.multi_agent_v2.max_concurrent_threads_per_session, 6);
    assert_eq!(config.multi_agent_v2.min_wait_timeout_ms, 1500);
    assert_eq!(config.multi_agent_v2.max_wait_timeout_ms, 90000);
    assert_eq!(config.multi_agent_v2.default_wait_timeout_ms, 15000);
    assert!(!config.multi_agent_v2.usage_hint_enabled);
    assert_eq!(
        config.multi_agent_v2.usage_hint_text.as_deref(),
        Some("profile hint")
    );
    assert_eq!(
        config.multi_agent_v2.root_agent_usage_hint_text.as_deref(),
        Some("profile root hint")
    );
    assert_eq!(
        config.multi_agent_v2.subagent_usage_hint_text.as_deref(),
        Some("profile subagent hint")
    );
    assert!(!config.multi_agent_v2.hide_spawn_agent_metadata);
    assert!(config.multi_agent_v2.non_code_mode_only);

    Ok(())
}

#[tokio::test]
async fn multi_agent_v2_default_session_thread_cap_counts_root() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    std::fs::write(
        codex_home.path().join(CONFIG_TOML_FILE),
        r#"[features.multi_agent_v2]
enabled = true
"#,
    )?;

    let config = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .fallback_cwd(Some(codex_home.path().to_path_buf()))
        .build()
        .await?;

    assert_eq!(config.multi_agent_v2.max_concurrent_threads_per_session, 4);
    assert_eq!(config.multi_agent_v2.min_wait_timeout_ms, 10_000);
    assert_eq!(config.multi_agent_v2.max_wait_timeout_ms, 3_600_000);
    assert_eq!(config.multi_agent_v2.default_wait_timeout_ms, 30_000);
    assert_eq!(config.agent_max_threads, Some(3));
    assert!(!config.multi_agent_v2.non_code_mode_only);

    Ok(())
}

#[tokio::test]
async fn multi_agent_v2_rejects_agents_max_threads() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    std::fs::write(
        codex_home.path().join(CONFIG_TOML_FILE),
        r#"[features.multi_agent_v2]
enabled = true

[agents]
max_threads = 3
"#,
    )?;

    let err = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .fallback_cwd(Some(codex_home.path().to_path_buf()))
        .build()
        .await
        .expect_err("agents.max_threads should conflict with multi_agent_v2");

    assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
    assert_eq!(
        err.to_string(),
        "agents.max_threads cannot be set when multi_agent_v2 is enabled"
    );

    Ok(())
}

#[tokio::test]
async fn multi_agent_v2_rejects_invalid_wait_timeouts() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    std::fs::write(
        codex_home.path().join(CONFIG_TOML_FILE),
        r#"[features.multi_agent_v2]
enabled = true
min_wait_timeout_ms = 0
max_wait_timeout_ms = 0
default_wait_timeout_ms = 0
"#,
    )?;

    let config = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .fallback_cwd(Some(codex_home.path().to_path_buf()))
        .build()
        .await?;

    assert_eq!(config.multi_agent_v2.min_wait_timeout_ms, 0);
    assert_eq!(config.multi_agent_v2.max_wait_timeout_ms, 0);
    assert_eq!(config.multi_agent_v2.default_wait_timeout_ms, 0);

    std::fs::write(
        codex_home.path().join(CONFIG_TOML_FILE),
        r#"[features.multi_agent_v2]
enabled = true
min_wait_timeout_ms = -1
"#,
    )?;

    let err = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .fallback_cwd(Some(codex_home.path().to_path_buf()))
        .build()
        .await
        .expect_err("negative min_wait_timeout_ms should be rejected");

    assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
    assert_eq!(
        err.to_string(),
        "features.multi_agent_v2.min_wait_timeout_ms must be at least 0"
    );

    std::fs::write(
        codex_home.path().join(CONFIG_TOML_FILE),
        r#"[features.multi_agent_v2]
enabled = true
min_wait_timeout_ms = 3600001
"#,
    )?;

    let err = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .fallback_cwd(Some(codex_home.path().to_path_buf()))
        .build()
        .await
        .expect_err("too large min_wait_timeout_ms should be rejected");

    assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
    assert_eq!(
        err.to_string(),
        "features.multi_agent_v2.min_wait_timeout_ms must be at most 3600000"
    );

    std::fs::write(
        codex_home.path().join(CONFIG_TOML_FILE),
        r#"[features.multi_agent_v2]
enabled = true
max_wait_timeout_ms = -1
"#,
    )?;

    let err = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .fallback_cwd(Some(codex_home.path().to_path_buf()))
        .build()
        .await
        .expect_err("negative max_wait_timeout_ms should be rejected");

    assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
    assert_eq!(
        err.to_string(),
        "features.multi_agent_v2.max_wait_timeout_ms must be at least 0"
    );

    std::fs::write(
        codex_home.path().join(CONFIG_TOML_FILE),
        r#"[features.multi_agent_v2]
enabled = true
max_wait_timeout_ms = 3600001
"#,
    )?;

    let err = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .fallback_cwd(Some(codex_home.path().to_path_buf()))
        .build()
        .await
        .expect_err("too large max_wait_timeout_ms should be rejected");

    assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
    assert_eq!(
        err.to_string(),
        "features.multi_agent_v2.max_wait_timeout_ms must be at most 3600000"
    );

    std::fs::write(
        codex_home.path().join(CONFIG_TOML_FILE),
        r#"[features.multi_agent_v2]
enabled = true
default_wait_timeout_ms = -1
"#,
    )?;

    let err = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .fallback_cwd(Some(codex_home.path().to_path_buf()))
        .build()
        .await
        .expect_err("negative default_wait_timeout_ms should be rejected");

    assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
    assert_eq!(
        err.to_string(),
        "features.multi_agent_v2.default_wait_timeout_ms must be at least 0"
    );

    std::fs::write(
        codex_home.path().join(CONFIG_TOML_FILE),
        r#"[features.multi_agent_v2]
enabled = true
min_wait_timeout_ms = 1000
max_wait_timeout_ms = 500
"#,
    )?;

    let err = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .fallback_cwd(Some(codex_home.path().to_path_buf()))
        .build()
        .await
        .expect_err("min greater than max should be rejected");

    assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
    assert_eq!(
        err.to_string(),
        "features.multi_agent_v2.min_wait_timeout_ms must be at most features.multi_agent_v2.max_wait_timeout_ms"
    );

    std::fs::write(
        codex_home.path().join(CONFIG_TOML_FILE),
        r#"[features.multi_agent_v2]
enabled = true
min_wait_timeout_ms = 1000
max_wait_timeout_ms = 2000
default_wait_timeout_ms = 500
"#,
    )?;

    let err = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .fallback_cwd(Some(codex_home.path().to_path_buf()))
        .build()
        .await
        .expect_err("default less than min should be rejected");

    assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
    assert_eq!(
        err.to_string(),
        "features.multi_agent_v2.default_wait_timeout_ms must be at least features.multi_agent_v2.min_wait_timeout_ms"
    );

    std::fs::write(
        codex_home.path().join(CONFIG_TOML_FILE),
        r#"[features.multi_agent_v2]
enabled = true
min_wait_timeout_ms = 1000
max_wait_timeout_ms = 2000
default_wait_timeout_ms = 2500
"#,
    )?;

    let err = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .fallback_cwd(Some(codex_home.path().to_path_buf()))
        .build()
        .await
        .expect_err("default greater than max should be rejected");

    assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
    assert_eq!(
        err.to_string(),
        "features.multi_agent_v2.default_wait_timeout_ms must be at most features.multi_agent_v2.max_wait_timeout_ms"
    );

    Ok(())
}

#[tokio::test]
async fn multi_agent_v2_session_thread_cap_one_disallows_subagents() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    std::fs::write(
        codex_home.path().join(CONFIG_TOML_FILE),
        r#"[features.multi_agent_v2]
enabled = true
max_concurrent_threads_per_session = 1
"#,
    )?;

    let config = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .fallback_cwd(Some(codex_home.path().to_path_buf()))
        .build()
        .await?;

    assert_eq!(config.multi_agent_v2.max_concurrent_threads_per_session, 1);
    assert_eq!(config.agent_max_threads, Some(0));

    Ok(())
}

#[tokio::test]
async fn feature_requirements_normalize_runtime_feature_mutations() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;

    let mut config = ConfigBuilder::default()
        .codex_home(codex_home.path().to_path_buf())
        .cloud_requirements(CloudRequirementsLoader::new(async {
            Ok(Some(codex_config::ConfigRequirementsToml {
                feature_requirements: Some(codex_config::FeatureRequirementsToml {
                    entries: BTreeMap::from([
                        ("personality".to_string(), true),
                        ("shell_tool".to_string(), false),
                    ]),
                }),
                ..Default::default()
            }))
        }))
        .build()
        .await?;

    let mut requested = config.features.get().clone();
    requested
        .disable(Feature::Personality)
        .enable(Feature::ShellTool);
    assert!(config.features.can_set(&requested).is_ok());
    config
        .features
        .set(requested)
        .expect("managed feature mutations should normalize successfully");

    assert!(config.features.enabled(Feature::Personality));
    assert!(!config.features.enabled(Feature::ShellTool));

    Ok(())
}

#[tokio::test]
async fn feature_requirements_warn_on_collab_legacy_alias() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;

    let config = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .cloud_requirements(CloudRequirementsLoader::new(async {
            Ok(Some(codex_config::ConfigRequirementsToml {
                feature_requirements: Some(codex_config::FeatureRequirementsToml {
                    entries: BTreeMap::from([("collab".to_string(), true)]),
                }),
                ..Default::default()
            }))
        }))
        .build()
        .await?;

    assert!(config.features.enabled(Feature::Collab));
    assert!(
        config.startup_warnings.iter().any(|warning| {
            warning.contains("Using legacy `features` requirement `collab`")
                && warning.contains("prefer canonical feature key `multi_agent`")
        }),
        "{:?}",
        config.startup_warnings
    );

    Ok(())
}

#[tokio::test]
async fn feature_requirements_warn_and_ignore_unknown_feature() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;

    let config = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .cloud_requirements(CloudRequirementsLoader::new(async {
            Ok(Some(codex_config::ConfigRequirementsToml {
                feature_requirements: Some(codex_config::FeatureRequirementsToml {
                    entries: BTreeMap::from([("made_up_feature".to_string(), true)]),
                }),
                ..Default::default()
            }))
        }))
        .build()
        .await?;

    assert!(
        config
            .startup_warnings
            .iter()
            .any(|warning| warning
                .contains("Ignoring unknown `features` requirement `made_up_feature`")),
        "{:?}",
        config.startup_warnings
    );

    Ok(())
}

#[tokio::test]
async fn tool_suggest_discoverables_load_from_config_toml() -> std::io::Result<()> {
    let cfg: ConfigToml = toml::from_str(
        r#"
[tool_suggest]
discoverables = [
  { type = "connector", id = "connector_alpha" },
  { type = "plugin", id = "plugin_alpha@openai-curated" },
  { type = "connector", id = "   " }
]
"#,
    )
    .expect("TOML deserialization should succeed");

    assert_eq!(
        cfg.tool_suggest,
        Some(ToolSuggestConfig {
            discoverables: vec![
                ToolSuggestDiscoverable {
                    kind: ToolSuggestDiscoverableType::Connector,
                    id: "connector_alpha".to_string(),
                },
                ToolSuggestDiscoverable {
                    kind: ToolSuggestDiscoverableType::Plugin,
                    id: "plugin_alpha@openai-curated".to_string(),
                },
                ToolSuggestDiscoverable {
                    kind: ToolSuggestDiscoverableType::Connector,
                    id: "   ".to_string(),
                },
            ],
            disabled_tools: Vec::new(),
        })
    );

    let codex_home = TempDir::new()?;
    let config = Config::load_from_base_config_with_overrides(
        cfg,
        ConfigOverrides::default(),
        codex_home.abs(),
    )
    .await?;

    assert_eq!(
        config.tool_suggest,
        ToolSuggestConfig {
            discoverables: vec![
                ToolSuggestDiscoverable {
                    kind: ToolSuggestDiscoverableType::Connector,
                    id: "connector_alpha".to_string(),
                },
                ToolSuggestDiscoverable {
                    kind: ToolSuggestDiscoverableType::Plugin,
                    id: "plugin_alpha@openai-curated".to_string(),
                },
            ],
            disabled_tools: Vec::new(),
        }
    );
    Ok(())
}

#[tokio::test]
async fn tool_suggest_disabled_tools_load_from_config_toml() -> std::io::Result<()> {
    let cfg: ConfigToml = toml::from_str(
        r#"
[tool_suggest]
disabled_tools = [
  { type = "connector", id = " connector_calendar " },
  { type = "connector", id = "connector_calendar" },
  { type = "connector", id = "   " },
  { type = "plugin", id = "slack@openai-curated" }
]
"#,
    )
    .expect("TOML deserialization should succeed");

    assert_eq!(
        cfg.tool_suggest,
        Some(ToolSuggestConfig {
            discoverables: Vec::new(),
            disabled_tools: vec![
                ToolSuggestDisabledTool::connector(" connector_calendar "),
                ToolSuggestDisabledTool::connector("connector_calendar"),
                ToolSuggestDisabledTool::connector("   "),
                ToolSuggestDisabledTool::plugin("slack@openai-curated"),
            ],
        })
    );

    let codex_home = TempDir::new()?;
    let config = Config::load_from_base_config_with_overrides(
        cfg,
        ConfigOverrides::default(),
        codex_home.abs(),
    )
    .await?;

    assert_eq!(
        config.tool_suggest,
        ToolSuggestConfig {
            discoverables: Vec::new(),
            disabled_tools: vec![
                ToolSuggestDisabledTool::connector("connector_calendar"),
                ToolSuggestDisabledTool::plugin("slack@openai-curated"),
            ],
        }
    );
    Ok(())
}

#[tokio::test]
async fn tool_suggest_disabled_tools_merge_across_config_layers() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let workspace = TempDir::new()?;
    let workspace_key = workspace.path().to_string_lossy().replace('\\', "\\\\");
    std::fs::write(
        codex_home.path().join(CONFIG_TOML_FILE),
        format!(
            r#"
[projects."{workspace_key}"]
trust_level = "trusted"

[tool_suggest]
disabled_tools = [
  {{ type = "connector", id = " user_connector " }},
  {{ type = "plugin", id = "shared_plugin" }},
  {{ type = "connector", id = "project_connector" }},
]
"#
        ),
    )?;

    let project_config_dir = workspace.path().join(".codex");
    std::fs::create_dir_all(&project_config_dir)?;
    std::fs::write(
        project_config_dir.join(CONFIG_TOML_FILE),
        r#"
[tool_suggest]
disabled_tools = [
  { type = "connector", id = "project_connector" },
  { type = "plugin", id = "project_plugin" },
  { type = "plugin", id = "shared_plugin" },
]
"#,
    )?;

    let config = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .harness_overrides(ConfigOverrides {
            cwd: Some(workspace.path().to_path_buf()),
            ..Default::default()
        })
        .build()
        .await?;

    assert_eq!(
        config.tool_suggest.disabled_tools,
        vec![
            ToolSuggestDisabledTool::connector("user_connector"),
            ToolSuggestDisabledTool::plugin("shared_plugin"),
            ToolSuggestDisabledTool::connector("project_connector"),
            ToolSuggestDisabledTool::plugin("project_plugin"),
        ]
    );
    Ok(())
}

#[tokio::test]
async fn experimental_realtime_start_instructions_load_from_config_toml() -> std::io::Result<()> {
    let cfg: ConfigToml = toml::from_str(
        r#"
experimental_realtime_start_instructions = "start instructions from config"
"#,
    )
    .expect("TOML deserialization should succeed");

    assert_eq!(
        cfg.experimental_realtime_start_instructions.as_deref(),
        Some("start instructions from config")
    );

    let codex_home = TempDir::new()?;
    let config = Config::load_from_base_config_with_overrides(
        cfg,
        ConfigOverrides::default(),
        codex_home.abs(),
    )
    .await?;

    assert_eq!(
        config.experimental_realtime_start_instructions.as_deref(),
        Some("start instructions from config")
    );
    Ok(())
}

#[tokio::test]
async fn experimental_thread_config_endpoint_loads_from_config_toml() -> std::io::Result<()> {
    let cfg: ConfigToml = toml::from_str(
        r#"
experimental_thread_config_endpoint = "http://127.0.0.1:8061"
"#,
    )
    .expect("TOML deserialization should succeed");

    assert_eq!(
        cfg.experimental_thread_config_endpoint.as_deref(),
        Some("http://127.0.0.1:8061")
    );

    let codex_home = TempDir::new()?;
    let config = Config::load_from_base_config_with_overrides(
        cfg,
        ConfigOverrides::default(),
        codex_home.abs(),
    )
    .await?;

    assert_eq!(
        config.experimental_thread_config_endpoint.as_deref(),
        Some("http://127.0.0.1:8061")
    );
    Ok(())
}

#[tokio::test]
async fn experimental_realtime_ws_base_url_loads_from_config_toml() -> std::io::Result<()> {
    let cfg: ConfigToml = toml::from_str(
        r#"
experimental_realtime_ws_base_url = "http://127.0.0.1:8011"
"#,
    )
    .expect("TOML deserialization should succeed");

    assert_eq!(
        cfg.experimental_realtime_ws_base_url.as_deref(),
        Some("http://127.0.0.1:8011")
    );

    let codex_home = TempDir::new()?;
    let config = Config::load_from_base_config_with_overrides(
        cfg,
        ConfigOverrides::default(),
        codex_home.abs(),
    )
    .await?;

    assert_eq!(
        config.experimental_realtime_ws_base_url.as_deref(),
        Some("http://127.0.0.1:8011")
    );
    Ok(())
}

#[tokio::test]
async fn experimental_realtime_ws_backend_prompt_loads_from_config_toml() -> std::io::Result<()> {
    let cfg: ConfigToml = toml::from_str(
        r#"
experimental_realtime_ws_backend_prompt = "prompt from config"
"#,
    )
    .expect("TOML deserialization should succeed");

    assert_eq!(
        cfg.experimental_realtime_ws_backend_prompt.as_deref(),
        Some("prompt from config")
    );

    let codex_home = TempDir::new()?;
    let config = Config::load_from_base_config_with_overrides(
        cfg,
        ConfigOverrides::default(),
        codex_home.abs(),
    )
    .await?;

    assert_eq!(
        config.experimental_realtime_ws_backend_prompt.as_deref(),
        Some("prompt from config")
    );
    Ok(())
}

#[tokio::test]
async fn experimental_realtime_ws_startup_context_loads_from_config_toml() -> std::io::Result<()> {
    let cfg: ConfigToml = toml::from_str(
        r#"
experimental_realtime_ws_startup_context = "startup context from config"
"#,
    )
    .expect("TOML deserialization should succeed");

    assert_eq!(
        cfg.experimental_realtime_ws_startup_context.as_deref(),
        Some("startup context from config")
    );

    let codex_home = TempDir::new()?;
    let config = Config::load_from_base_config_with_overrides(
        cfg,
        ConfigOverrides::default(),
        codex_home.abs(),
    )
    .await?;

    assert_eq!(
        config.experimental_realtime_ws_startup_context.as_deref(),
        Some("startup context from config")
    );
    Ok(())
}

#[tokio::test]
async fn experimental_realtime_ws_model_loads_from_config_toml() -> std::io::Result<()> {
    let cfg: ConfigToml = toml::from_str(
        r#"
experimental_realtime_ws_model = "realtime-test-model"
"#,
    )
    .expect("TOML deserialization should succeed");

    assert_eq!(
        cfg.experimental_realtime_ws_model.as_deref(),
        Some("realtime-test-model")
    );

    let codex_home = TempDir::new()?;
    let config = Config::load_from_base_config_with_overrides(
        cfg,
        ConfigOverrides::default(),
        codex_home.abs(),
    )
    .await?;

    assert_eq!(
        config.experimental_realtime_ws_model.as_deref(),
        Some("realtime-test-model")
    );
    Ok(())
}

#[tokio::test]
async fn realtime_config_partial_table_uses_realtime_defaults() -> std::io::Result<()> {
    let cfg: ConfigToml = toml::from_str(
        r#"
[realtime]
voice = "marin"
"#,
    )
    .expect("TOML deserialization should succeed");

    let codex_home = TempDir::new()?;
    let config = Config::load_from_base_config_with_overrides(
        cfg,
        ConfigOverrides::default(),
        codex_home.abs(),
    )
    .await?;

    assert_eq!(
        config.realtime,
        RealtimeConfig {
            voice: Some(RealtimeVoice::Marin),
            ..RealtimeConfig::default()
        }
    );
    Ok(())
}

#[tokio::test]
async fn realtime_loads_from_config_toml() -> std::io::Result<()> {
    let cfg: ConfigToml = toml::from_str(
        r#"
[realtime]
version = "v2"
type = "transcription"
transport = "webrtc"
voice = "cedar"
"#,
    )
    .expect("TOML deserialization should succeed");

    assert_eq!(
        cfg.realtime,
        Some(RealtimeToml {
            version: Some(RealtimeWsVersion::V2),
            session_type: Some(RealtimeWsMode::Transcription),
            transport: Some(RealtimeTransport::WebRtc),
            voice: Some(RealtimeVoice::Cedar),
        })
    );

    let codex_home = TempDir::new()?;
    let config = Config::load_from_base_config_with_overrides(
        cfg,
        ConfigOverrides::default(),
        codex_home.abs(),
    )
    .await?;

    assert_eq!(
        config.realtime,
        RealtimeConfig {
            version: RealtimeWsVersion::V2,
            session_type: RealtimeWsMode::Transcription,
            transport: RealtimeTransport::WebRtc,
            voice: Some(RealtimeVoice::Cedar),
        }
    );
    Ok(())
}

#[tokio::test]
async fn realtime_audio_loads_from_config_toml() -> std::io::Result<()> {
    let cfg: ConfigToml = toml::from_str(
        r#"
[audio]
microphone = "USB Mic"
speaker = "Desk Speakers"
"#,
    )
    .expect("TOML deserialization should succeed");

    let realtime_audio = cfg
        .audio
        .as_ref()
        .expect("realtime audio config should be present");
    assert_eq!(realtime_audio.microphone.as_deref(), Some("USB Mic"));
    assert_eq!(realtime_audio.speaker.as_deref(), Some("Desk Speakers"));

    let codex_home = TempDir::new()?;
    let config = Config::load_from_base_config_with_overrides(
        cfg,
        ConfigOverrides::default(),
        codex_home.abs(),
    )
    .await?;

    assert_eq!(config.realtime_audio.microphone.as_deref(), Some("USB Mic"));
    assert_eq!(
        config.realtime_audio.speaker.as_deref(),
        Some("Desk Speakers")
    );
    Ok(())
}

#[derive(Deserialize, Debug, PartialEq)]
struct TuiTomlTest {
    #[serde(default, flatten)]
    notifications: TuiNotificationSettings,
}

#[derive(Deserialize, Debug, PartialEq)]
struct RootTomlTest {
    tui: TuiTomlTest,
}

#[test]
fn test_tui_notifications_true() {
    let toml = r#"
            [tui]
            notifications = true
        "#;
    let parsed: RootTomlTest = toml::from_str(toml).expect("deserialize notifications=true");
    assert_matches!(
        parsed.tui.notifications.notifications,
        Notifications::Enabled(true)
    );
}

#[test]
fn test_tui_notifications_custom_array() {
    let toml = r#"
            [tui]
            notifications = ["foo"]
        "#;
    let parsed: RootTomlTest = toml::from_str(toml).expect("deserialize notifications=[\"foo\"]");
    assert_matches!(
        parsed.tui.notifications.notifications,
        Notifications::Custom(ref v) if v == &vec!["foo".to_string()]
    );
}

#[test]
fn test_tui_notification_method() {
    let toml = r#"
            [tui]
            notification_method = "bel"
        "#;
    let parsed: RootTomlTest =
        toml::from_str(toml).expect("deserialize notification_method=\"bel\"");
    assert_eq!(parsed.tui.notifications.method, NotificationMethod::Bel);
}

#[test]
fn test_tui_notification_condition_defaults_to_unfocused() {
    let toml = r#"
            [tui]
        "#;
    let parsed: RootTomlTest =
        toml::from_str(toml).expect("deserialize default notification condition");
    assert_eq!(
        parsed.tui.notifications.condition,
        NotificationCondition::Unfocused
    );
}

#[test]
fn test_tui_notification_condition_always() {
    let toml = r#"
            [tui]
            notification_condition = "always"
        "#;
    let parsed: RootTomlTest =
        toml::from_str(toml).expect("deserialize notification_condition=\"always\"");
    assert_eq!(
        parsed.tui.notifications.condition,
        NotificationCondition::Always
    );
}

#[test]
fn test_tui_notification_condition_rejects_unknown_value() {
    let toml = r#"
            [tui]
            notification_condition = "background"
        "#;
    let err = toml::from_str::<RootTomlTest>(toml).expect_err("reject unknown condition");
    let err = err.to_string();
    assert!(
        err.contains("unknown variant `background`")
            && err.contains("unfocused")
            && err.contains("always"),
        "unexpected error: {err}"
    );
}
