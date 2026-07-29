#[tokio::test]
async fn replace_mcp_servers_serializes_required_flag() -> anyhow::Result<()> {
    let codex_home = TempDir::new()?;

    let servers = BTreeMap::from([(
        "docs".to_string(),
        McpServerConfig {
            transport: McpServerTransportConfig::Stdio {
                command: "docs-server".to_string(),
                args: Vec::new(),
                env: None,
                env_vars: Vec::new(),
                cwd: None,
            },
            experimental_environment: None,
            enabled: true,
            required: true,
            supports_parallel_tool_calls: false,
            disabled_reason: None,
            startup_timeout_sec: None,
            tool_timeout_sec: None,
            default_tools_approval_mode: None,
            enabled_tools: None,
            disabled_tools: None,
            scopes: None,
            oauth: None,
            oauth_resource: None,
            tools: HashMap::new(),
        },
    )]);

    apply_blocking(
        codex_home.path(),
        /*profile*/ None,
        &[ConfigEdit::ReplaceMcpServers(servers.clone())],
    )?;

    let config_path = codex_home.path().join(CONFIG_TOML_FILE);
    let serialized = std::fs::read_to_string(&config_path)?;
    assert!(
        serialized.contains("required = true"),
        "serialized config missing required flag:\n{serialized}"
    );

    let loaded = load_global_mcp_servers(codex_home.path()).await?;
    let docs = loaded.get("docs").expect("docs entry");
    assert!(docs.required);

    Ok(())
}
#[tokio::test]
async fn replace_mcp_servers_serializes_tool_filters() -> anyhow::Result<()> {
    let codex_home = TempDir::new()?;

    let servers = BTreeMap::from([(
        "docs".to_string(),
        McpServerConfig {
            transport: McpServerTransportConfig::Stdio {
                command: "docs-server".to_string(),
                args: Vec::new(),
                env: None,
                env_vars: Vec::new(),
                cwd: None,
            },
            experimental_environment: None,
            enabled: true,
            required: false,
            supports_parallel_tool_calls: false,
            disabled_reason: None,
            startup_timeout_sec: None,
            tool_timeout_sec: None,
            default_tools_approval_mode: None,
            enabled_tools: Some(vec!["allowed".to_string()]),
            disabled_tools: Some(vec!["blocked".to_string()]),
            scopes: None,
            oauth: None,
            oauth_resource: None,
            tools: HashMap::new(),
        },
    )]);

    apply_blocking(
        codex_home.path(),
        /*profile*/ None,
        &[ConfigEdit::ReplaceMcpServers(servers.clone())],
    )?;

    let config_path = codex_home.path().join(CONFIG_TOML_FILE);
    let serialized = std::fs::read_to_string(&config_path)?;
    assert!(serialized.contains(r#"enabled_tools = ["allowed"]"#));
    assert!(serialized.contains(r#"disabled_tools = ["blocked"]"#));

    let loaded = load_global_mcp_servers(codex_home.path()).await?;
    let docs = loaded.get("docs").expect("docs entry");
    assert_eq!(
        docs.enabled_tools.as_ref(),
        Some(&vec!["allowed".to_string()])
    );
    assert_eq!(
        docs.disabled_tools.as_ref(),
        Some(&vec!["blocked".to_string()])
    );

    Ok(())
}

#[tokio::test]
async fn replace_mcp_servers_streamable_http_serializes_oauth_resource() -> anyhow::Result<()> {
    let codex_home = TempDir::new()?;

    let servers = BTreeMap::from([(
        "docs".to_string(),
        McpServerConfig {
            transport: McpServerTransportConfig::StreamableHttp {
                url: "https://example.com/mcp".to_string(),
                bearer_token_env_var: None,
                http_headers: None,
                env_http_headers: None,
            },
            experimental_environment: None,
            enabled: true,
            required: false,
            supports_parallel_tool_calls: false,
            disabled_reason: None,
            startup_timeout_sec: None,
            tool_timeout_sec: None,
            default_tools_approval_mode: None,
            enabled_tools: None,
            disabled_tools: None,
            scopes: None,
            oauth: Some(McpServerOAuthConfig {
                client_id: Some("eci-prd-pub-codex-123".to_string()),
            }),
            oauth_resource: Some("https://resource.example.com".to_string()),
            tools: HashMap::new(),
        },
    )]);

    apply_blocking(
        codex_home.path(),
        /*profile*/ None,
        &[ConfigEdit::ReplaceMcpServers(servers.clone())],
    )?;

    let config_path = codex_home.path().join(CONFIG_TOML_FILE);
    let serialized = std::fs::read_to_string(&config_path)?;
    assert!(serialized.contains("[mcp_servers.docs.oauth]"));
    assert!(serialized.contains(r#"client_id = "eci-prd-pub-codex-123""#));
    assert!(serialized.contains(r#"oauth_resource = "https://resource.example.com""#));

    let loaded = load_global_mcp_servers(codex_home.path()).await?;
    let docs = loaded.get("docs").expect("docs entry");
    assert_eq!(
        docs.oauth_resource.as_deref(),
        Some("https://resource.example.com")
    );
    assert_eq!(docs.oauth_client_id(), Some("eci-prd-pub-codex-123"));

    Ok(())
}

#[tokio::test]
async fn set_model_updates_defaults() -> anyhow::Result<()> {
    let codex_home = TempDir::new()?;

    ConfigEditsBuilder::new(codex_home.path())
        .set_model(Some("gpt-5.4"), Some(ReasoningEffort::High))
        .apply()
        .await?;

    let serialized = tokio::fs::read_to_string(codex_home.path().join(CONFIG_TOML_FILE)).await?;
    let parsed: ConfigToml = toml::from_str(&serialized)?;

    assert_eq!(parsed.model.as_deref(), Some("gpt-5.4"));
    assert_eq!(parsed.model_reasoning_effort, Some(ReasoningEffort::High));

    Ok(())
}

#[tokio::test]
async fn for_config_writes_selected_user_config_file() -> anyhow::Result<()> {
    let codex_home = TempDir::new()?;
    let base_config = codex_home.path().join(CONFIG_TOML_FILE);
    let selected_config = codex_home.path().join("work.config.toml");
    tokio::fs::write(&base_config, r#"model_provider = "openai""#).await?;
    tokio::fs::write(&selected_config, r#"model = "gpt-old""#).await?;

    let config = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .loader_overrides(LoaderOverrides {
            user_config_path: Some(selected_config.abs()),
            user_config_profile: Some("work".parse().expect("profile-v2 name")),
            ..LoaderOverrides::without_managed_config_for_tests()
        })
        .build()
        .await?;

    ConfigEditsBuilder::for_config(&config)
        .set_model(Some("gpt-new"), Some(ReasoningEffort::High))
        .apply()
        .await?;

    let selected_serialized = tokio::fs::read_to_string(&selected_config).await?;
    let selected: ConfigToml = toml::from_str(&selected_serialized)?;
    assert_eq!(selected.model.as_deref(), Some("gpt-new"));
    assert_eq!(selected.model_reasoning_effort, Some(ReasoningEffort::High));
    assert_eq!(
        tokio::fs::read_to_string(&base_config).await?,
        r#"model_provider = "openai""#
    );

    Ok(())
}

#[test]
fn profile_v2_config_path_resolves_validated_names() -> anyhow::Result<()> {
    let hepta_home = TempDir::new()?;
    let profile_name: ProfileV2Name = "work".parse()?;
    assert_eq!(
        resolve_profile_v2_config_path(hepta_home.path(), &profile_name),
        hepta_home.path().join("work.config.toml").abs()
    );
    Ok(())
}

#[tokio::test]
async fn set_model_overwrites_existing_model() -> anyhow::Result<()> {
    let codex_home = TempDir::new()?;
    let config_path = codex_home.path().join(CONFIG_TOML_FILE);

    tokio::fs::write(
        &config_path,
        r#"
model = "gpt-5.4"
model_reasoning_effort = "medium"

[profiles.dev]
model = "gpt-4.1"
"#,
    )
    .await?;

    ConfigEditsBuilder::new(codex_home.path())
        .set_model(Some("o4-mini"), Some(ReasoningEffort::High))
        .apply()
        .await?;

    let serialized = tokio::fs::read_to_string(config_path).await?;
    let parsed: ConfigToml = toml::from_str(&serialized)?;

    assert_eq!(parsed.model.as_deref(), Some("o4-mini"));
    assert_eq!(parsed.model_reasoning_effort, Some(ReasoningEffort::High));
    assert_eq!(
        parsed
            .profiles
            .get("dev")
            .and_then(|profile| profile.model.as_deref()),
        Some("gpt-4.1"),
    );

    Ok(())
}

#[tokio::test]
async fn set_model_updates_profile() -> anyhow::Result<()> {
    let codex_home = TempDir::new()?;

    ConfigEditsBuilder::new(codex_home.path())
        .with_profile(Some("dev"))
        .set_model(Some("gpt-5.4"), Some(ReasoningEffort::Medium))
        .apply()
        .await?;

    let serialized = tokio::fs::read_to_string(codex_home.path().join(CONFIG_TOML_FILE)).await?;
    let parsed: ConfigToml = toml::from_str(&serialized)?;
    let profile = parsed
        .profiles
        .get("dev")
        .expect("profile should be created");

    assert_eq!(profile.model.as_deref(), Some("gpt-5.4"));
    assert_eq!(
        profile.model_reasoning_effort,
        Some(ReasoningEffort::Medium)
    );

    Ok(())
}

#[tokio::test]
async fn set_model_updates_existing_profile() -> anyhow::Result<()> {
    let codex_home = TempDir::new()?;
    let config_path = codex_home.path().join(CONFIG_TOML_FILE);

    tokio::fs::write(
        &config_path,
        r#"
[profiles.dev]
model = "gpt-4"
model_reasoning_effort = "medium"

[profiles.prod]
model = "gpt-5.4"
"#,
    )
    .await?;

    ConfigEditsBuilder::new(codex_home.path())
        .with_profile(Some("dev"))
        .set_model(Some("o4-high"), Some(ReasoningEffort::Medium))
        .apply()
        .await?;

    let serialized = tokio::fs::read_to_string(config_path).await?;
    let parsed: ConfigToml = toml::from_str(&serialized)?;

    let dev_profile = parsed
        .profiles
        .get("dev")
        .expect("dev profile should survive updates");
    assert_eq!(dev_profile.model.as_deref(), Some("o4-high"));
    assert_eq!(
        dev_profile.model_reasoning_effort,
        Some(ReasoningEffort::Medium)
    );

    assert_eq!(
        parsed
            .profiles
            .get("prod")
            .and_then(|profile| profile.model.as_deref()),
        Some("gpt-5.4"),
    );

    Ok(())
}

#[tokio::test]
async fn set_feature_enabled_updates_profile() -> anyhow::Result<()> {
    let codex_home = TempDir::new()?;

    ConfigEditsBuilder::new(codex_home.path())
        .with_profile(Some("dev"))
        .set_feature_enabled("guardian_approval", /*enabled*/ true)
        .apply()
        .await?;

    let serialized = tokio::fs::read_to_string(codex_home.path().join(CONFIG_TOML_FILE)).await?;
    let parsed: ConfigToml = toml::from_str(&serialized)?;
    let profile = parsed
        .profiles
        .get("dev")
        .expect("profile should be created");

    assert_eq!(
        profile
            .features
            .as_ref()
            .and_then(|features| features.entries().get("guardian_approval").copied()),
        Some(true),
    );
    assert_eq!(
        parsed
            .features
            .as_ref()
            .and_then(|features| features.entries().get("guardian_approval").copied()),
        None,
    );

    Ok(())
}

#[tokio::test]
async fn set_feature_enabled_persists_feature_disable_in_profile() -> anyhow::Result<()> {
    let codex_home = TempDir::new()?;

    ConfigEditsBuilder::new(codex_home.path())
        .with_profile(Some("dev"))
        .set_feature_enabled("guardian_approval", /*enabled*/ true)
        .apply()
        .await?;

    ConfigEditsBuilder::new(codex_home.path())
        .with_profile(Some("dev"))
        .set_feature_enabled("guardian_approval", /*enabled*/ false)
        .apply()
        .await?;

    let serialized = tokio::fs::read_to_string(codex_home.path().join(CONFIG_TOML_FILE)).await?;
    let parsed: ConfigToml = toml::from_str(&serialized)?;
    let profile = parsed
        .profiles
        .get("dev")
        .expect("profile should be created");

    assert_eq!(
        profile
            .features
            .as_ref()
            .and_then(|features| features.entries().get("guardian_approval").copied()),
        Some(false),
    );
    assert_eq!(
        parsed
            .features
            .as_ref()
            .and_then(|features| features.entries().get("guardian_approval").copied()),
        None,
    );

    Ok(())
}

#[tokio::test]
async fn set_feature_enabled_profile_disable_overrides_root_enable() -> anyhow::Result<()> {
    let codex_home = TempDir::new()?;

    ConfigEditsBuilder::new(codex_home.path())
        .set_feature_enabled("guardian_approval", /*enabled*/ true)
        .apply()
        .await?;

    ConfigEditsBuilder::new(codex_home.path())
        .with_profile(Some("dev"))
        .set_feature_enabled("guardian_approval", /*enabled*/ false)
        .apply()
        .await?;

    let serialized = tokio::fs::read_to_string(codex_home.path().join(CONFIG_TOML_FILE)).await?;
    let parsed: ConfigToml = toml::from_str(&serialized)?;
    let profile = parsed
        .profiles
        .get("dev")
        .expect("profile should be created");

    assert_eq!(
        parsed
            .features
            .as_ref()
            .and_then(|features| features.entries().get("guardian_approval").copied()),
        Some(true),
    );
    assert_eq!(
        profile
            .features
            .as_ref()
            .and_then(|features| features.entries().get("guardian_approval").copied()),
        Some(false),
    );

    Ok(())
}

struct PrecedenceTestFixture {
    cwd: TempDir,
    codex_home: TempDir,
    cfg: ConfigToml,
    model_provider_map: HashMap<String, ModelProviderInfo>,
    openai_provider: ModelProviderInfo,
    openai_custom_provider: ModelProviderInfo,
}

impl PrecedenceTestFixture {
    fn cwd(&self) -> AbsolutePathBuf {
        self.cwd.abs()
    }

    fn cwd_path(&self) -> PathBuf {
        self.cwd.path().to_path_buf()
    }

    fn codex_home(&self) -> AbsolutePathBuf {
        self.codex_home.abs()
    }
}

#[tokio::test]
async fn cli_override_sets_compact_prompt() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let overrides = ConfigOverrides {
        compact_prompt: Some("Use the compact override".to_string()),
        ..Default::default()
    };

    let config = Config::load_from_base_config_with_overrides(
        ConfigToml::default(),
        overrides,
        codex_home.abs(),
    )
    .await?;

    assert_eq!(
        config.compact_prompt.as_deref(),
        Some("Use the compact override")
    );

    Ok(())
}

#[tokio::test]
async fn loads_compact_prompt_from_file() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let workspace = codex_home.path().join("workspace");
    std::fs::create_dir_all(&workspace)?;

    let prompt_path = workspace.join("compact_prompt.txt");
    std::fs::write(&prompt_path, "  summarize differently  ")?;

    let cfg = ConfigToml {
        experimental_compact_prompt_file: Some(prompt_path.abs()),
        ..Default::default()
    };

    let overrides = ConfigOverrides {
        cwd: Some(workspace),
        ..Default::default()
    };

    let config =
        Config::load_from_base_config_with_overrides(cfg, overrides, codex_home.abs()).await?;

    assert_eq!(
        config.compact_prompt.as_deref(),
        Some("summarize differently")
    );

    Ok(())
}

#[tokio::test]
async fn load_config_uses_requirements_guardian_policy_config() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let config_layer_stack = ConfigLayerStack::new(
        Vec::new(),
        Default::default(),
        codex_config::ConfigRequirementsToml {
            guardian_policy_config: Some(
                "  Use the workspace-managed guardian policy.  ".to_string(),
            ),
            ..Default::default()
        },
    )
    .map_err(std::io::Error::other)?;

    let config = Config::load_config_with_layer_stack(
        LOCAL_FS.as_ref(),
        ConfigToml::default(),
        ConfigOverrides {
            cwd: Some(codex_home.path().to_path_buf()),
            ..Default::default()
        },
        codex_home.abs(),
        config_layer_stack,
    )
    .await?;

    assert_eq!(
        config.guardian_policy_config.as_deref(),
        Some("Use the workspace-managed guardian policy.")
    );

    Ok(())
}

#[test]
fn config_toml_deserializes_auto_review_policy() {
    let cfg = toml::from_str::<ConfigToml>(
        r#"
[auto_review]
policy = "Use the user-configured guardian policy."
"#,
    )
    .expect("TOML deserialization should succeed");

    assert_eq!(
        cfg.auto_review
            .as_ref()
            .and_then(|auto_review| auto_review.policy.as_deref()),
        Some("Use the user-configured guardian policy.")
    );
}

#[tokio::test]
async fn load_config_uses_auto_review_guardian_policy_config() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let cfg = ConfigToml {
        auto_review: Some(AutoReviewToml {
            policy: Some("  Use the user-configured guardian policy.  ".to_string()),
        }),
        ..Default::default()
    };

    let config = Config::load_from_base_config_with_overrides(
        cfg,
        ConfigOverrides {
            cwd: Some(codex_home.path().to_path_buf()),
            ..Default::default()
        },
        codex_home.abs(),
    )
    .await?;

    assert_eq!(
        config.guardian_policy_config.as_deref(),
        Some("Use the user-configured guardian policy.")
    );

    Ok(())
}

#[tokio::test]
async fn requirements_guardian_policy_beats_auto_review() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let config_layer_stack = ConfigLayerStack::new(
        Vec::new(),
        Default::default(),
        codex_config::ConfigRequirementsToml {
            guardian_policy_config: Some("Use the managed guardian policy.".to_string()),
            ..Default::default()
        },
    )
    .map_err(std::io::Error::other)?;
    let cfg = ConfigToml {
        auto_review: Some(AutoReviewToml {
            policy: Some("Use the user-configured guardian policy.".to_string()),
        }),
        ..Default::default()
    };

    let config = Config::load_config_with_layer_stack(
        LOCAL_FS.as_ref(),
        cfg,
        ConfigOverrides {
            cwd: Some(codex_home.path().to_path_buf()),
            ..Default::default()
        },
        codex_home.abs(),
        config_layer_stack,
    )
    .await?;

    assert_eq!(
        config.guardian_policy_config.as_deref(),
        Some("Use the managed guardian policy.")
    );

    Ok(())
}

#[tokio::test]
async fn load_config_ignores_empty_auto_review_guardian_policy_config() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let cfg = ConfigToml {
        auto_review: Some(AutoReviewToml {
            policy: Some("   ".to_string()),
        }),
        ..Default::default()
    };

    let config = Config::load_from_base_config_with_overrides(
        cfg,
        ConfigOverrides {
            cwd: Some(codex_home.path().to_path_buf()),
            ..Default::default()
        },
        codex_home.abs(),
    )
    .await?;

    assert_eq!(config.guardian_policy_config, None);

    Ok(())
}

#[tokio::test]
async fn load_config_ignores_empty_requirements_guardian_policy_config() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let config_layer_stack = ConfigLayerStack::new(
        Vec::new(),
        Default::default(),
        codex_config::ConfigRequirementsToml {
            guardian_policy_config: Some("   ".to_string()),
            ..Default::default()
        },
    )
    .map_err(std::io::Error::other)?;

    let config = Config::load_config_with_layer_stack(
        LOCAL_FS.as_ref(),
        ConfigToml::default(),
        ConfigOverrides {
            cwd: Some(codex_home.path().to_path_buf()),
            ..Default::default()
        },
        codex_home.abs(),
        config_layer_stack,
    )
    .await?;

    assert_eq!(config.guardian_policy_config, None);

    Ok(())
}

#[tokio::test]
async fn load_config_rejects_missing_agent_role_config_file() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let missing_path = codex_home.path().join("agents").join("researcher.toml");
    let cfg = ConfigToml {
        agents: Some(AgentsToml {
            max_threads: None,
            max_depth: None,
            job_max_runtime_seconds: None,
            interrupt_message: None,
            roles: BTreeMap::from([(
                "researcher".to_string(),
                AgentRoleToml {
                    description: Some("Research role".to_string()),
                    config_file: Some(missing_path.abs()),
                    agent_card_manifest_source: None,
                    agent_card_manifest_version: None,
                    agent_card_manifest: None,
                    nickname_candidates: None,
                },
            )]),
        }),
        ..Default::default()
    };

    let result = Config::load_from_base_config_with_overrides(
        cfg,
        ConfigOverrides::default(),
        codex_home.abs(),
    )
    .await;
    let err = result.expect_err("missing role config file should be rejected");
    assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
    let message = err.to_string();
    assert!(message.contains("agents.researcher.config_file"));
    assert!(message.contains("must point to an existing file"));

    Ok(())
}

#[tokio::test]
async fn agent_role_relative_config_file_resolves_against_config_toml() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let role_config_path = codex_home.path().join("agents").join("researcher.toml");
    tokio::fs::create_dir_all(
        role_config_path
            .parent()
            .expect("role config should have a parent directory"),
    )
    .await?;
    tokio::fs::write(
        &role_config_path,
        "developer_instructions = \"Research carefully\"\nmodel = \"gpt-5\"",
    )
    .await?;
    tokio::fs::write(
        codex_home.path().join(CONFIG_TOML_FILE),
        r#"[agents.researcher]
description = "Research role"
config_file = "./agents/researcher.toml"
nickname_candidates = ["Hypatia", "Noether"]
"#,
    )
    .await?;

    let config = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .fallback_cwd(Some(codex_home.path().to_path_buf()))
        .build()
        .await?;
    assert_eq!(
        config
            .agent_roles
            .get("researcher")
            .and_then(|role| role.config_file.as_ref()),
        Some(&role_config_path)
    );
    assert_eq!(
        config
            .agent_roles
            .get("researcher")
            .and_then(|role| role.nickname_candidates.as_ref())
            .map(|candidates| candidates.iter().map(String::as_str).collect::<Vec<_>>()),
        Some(vec!["Hypatia", "Noether"])
    );

    Ok(())
}

#[tokio::test]
async fn agent_role_manifest_metadata_resolves_from_inline_and_role_file() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let role_config_path = codex_home.path().join("agents").join("researcher.toml");
    tokio::fs::create_dir_all(
        role_config_path
            .parent()
            .expect("role config should have a parent directory"),
    )
    .await?;
    tokio::fs::write(
        &role_config_path,
        r#"agent_card_manifest_source = "agent-card://file/researcher"
agent_card_manifest_version = "hepta.agent_card_manifest.v1"
developer_instructions = "Research carefully"
model = "gpt-5"

[agent_card_manifest]
schema_version = "hepta.agent_card_manifest.v1"
source_surface_id = "spawn_agents_on_csv"
capabilities = ["csv_row_processing", "task_result_reporting", "work_graph_shadow_event_emission"]
allowed_tools = ["report_agent_job_result"]
lane = "agent_jobs"
max_threads = 4
"#,
    )
    .await?;
    tokio::fs::write(
        codex_home.path().join(CONFIG_TOML_FILE),
        r#"[agents.researcher]
description = "Research role"
config_file = "./agents/researcher.toml"
agent_card_manifest_source = "agent-card://inline/researcher"
agent_card_manifest_version = "stale.v0"

[agents.researcher.agent_card_manifest]
schema_version = "stale.v0"
source_surface_id = "spawn_agent_v2"
capabilities = ["local_subagent_spawn"]
allowed_tools = ["send_message"]
lane = "subagent"
max_depth = 2

[agents.reviewer]
description = "Review role"
agent_card_manifest_source = "agent-card://inline/reviewer"
agent_card_manifest_version = "hepta.agent_card_manifest.v1"

[agents.reviewer.agent_card_manifest]
schema_version = "hepta.agent_card_manifest.v1"
source_surface_id = "spawn_agent_v2"
capabilities = ["local_subagent_spawn", "inter_agent_mailbox", "named_task_path"]
allowed_tools = ["send_message", "followup_task", "wait_agent", "close_agent"]
lane = "subagent"
max_threads = 2
"#,
    )
    .await?;

    let config = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .fallback_cwd(Some(codex_home.path().to_path_buf()))
        .build()
        .await?;
    let researcher = config
        .agent_roles
        .get("researcher")
        .expect("researcher role should load");
    assert_eq!(
        researcher.agent_card_manifest_source.as_deref(),
        Some("agent-card://file/researcher")
    );
    assert_eq!(
        researcher.agent_card_manifest_version.as_deref(),
        Some("hepta.agent_card_manifest.v1")
    );
    let researcher_manifest = researcher
        .agent_card_manifest
        .as_ref()
        .expect("researcher manifest should load from role file");
    assert_eq!(
        researcher_manifest.schema_version.as_deref(),
        Some("hepta.agent_card_manifest.v1")
    );
    assert_eq!(
        researcher_manifest.source_surface_id.as_deref(),
        Some("spawn_agents_on_csv")
    );
    assert_eq!(
        researcher_manifest.capabilities,
        vec![
            "csv_row_processing".to_string(),
            "task_result_reporting".to_string(),
            "work_graph_shadow_event_emission".to_string()
        ]
    );
    assert_eq!(
        researcher_manifest.allowed_tools,
        vec!["report_agent_job_result".to_string()]
    );
    assert_eq!(researcher_manifest.lane.as_deref(), Some("agent_jobs"));
    assert_eq!(researcher_manifest.max_threads, Some(4));
    assert_eq!(researcher_manifest.max_depth, None);
    let reviewer = config
        .agent_roles
        .get("reviewer")
        .expect("reviewer role should load");
    assert_eq!(
        reviewer.agent_card_manifest_source.as_deref(),
        Some("agent-card://inline/reviewer")
    );
    assert_eq!(
        reviewer.agent_card_manifest_version.as_deref(),
        Some("hepta.agent_card_manifest.v1")
    );
    let reviewer_manifest = reviewer
        .agent_card_manifest
        .as_ref()
        .expect("reviewer manifest should load inline");
    assert_eq!(
        reviewer_manifest.source_surface_id.as_deref(),
        Some("spawn_agent_v2")
    );
    assert_eq!(
        reviewer_manifest.allowed_tools,
        vec![
            "send_message".to_string(),
            "followup_task".to_string(),
            "wait_agent".to_string(),
            "close_agent".to_string()
        ]
    );
    assert_eq!(reviewer_manifest.lane.as_deref(), Some("subagent"));
    assert_eq!(reviewer_manifest.max_threads, Some(2));

    Ok(())
}

#[tokio::test]
async fn agent_role_relative_config_file_resolves_from_config_layer() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let role_config_path = codex_home.path().join("agents").join("researcher.toml");
    tokio::fs::create_dir_all(
        role_config_path
            .parent()
            .expect("role config should have a parent directory"),
    )
    .await?;
    tokio::fs::write(
        &role_config_path,
        "developer_instructions = \"Research carefully\"\nmodel = \"gpt-5\"",
    )
    .await?;
    let layer_config = toml::from_str(
        r#"[agents.researcher]
description = "Research role"
config_file = "./agents/researcher.toml"
"#,
    )
    .expect("agent role layer config should parse");
    let config_layer_stack = codex_config::ConfigLayerStack::new(
        vec![codex_config::ConfigLayerEntry::new(
            codex_app_server_protocol::ConfigLayerSource::User {
                file: codex_home.path().join(CONFIG_TOML_FILE).abs(),
                profile: None,
            },
            layer_config,
        )],
        Default::default(),
        codex_config::ConfigRequirementsToml::default(),
    )
    .map_err(std::io::Error::other)?;

    let config = Config::load_config_with_layer_stack(
        LOCAL_FS.as_ref(),
        ConfigToml::default(),
        ConfigOverrides {
            cwd: Some(codex_home.path().to_path_buf()),
            ..Default::default()
        },
        codex_home.abs(),
        config_layer_stack,
    )
    .await?;

    assert_eq!(
        config
            .agent_roles
            .get("researcher")
            .and_then(|role| role.config_file.as_ref()),
        Some(&role_config_path)
    );

    Ok(())
}

#[tokio::test]
async fn agent_role_file_metadata_overrides_config_toml_metadata() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let role_config_path = codex_home.path().join("agents").join("researcher.toml");
    tokio::fs::create_dir_all(
        role_config_path
            .parent()
            .expect("role config should have a parent directory"),
    )
    .await?;
    tokio::fs::write(
        &role_config_path,
        r#"
description = "Role metadata from file"
nickname_candidates = ["Hypatia"]
developer_instructions = "Research carefully"
model = "gpt-5.2"
"#,
    )
    .await?;
    tokio::fs::write(
        codex_home.path().join(CONFIG_TOML_FILE),
        r#"[agents.researcher]
description = "Research role from config"
config_file = "./agents/researcher.toml"
nickname_candidates = ["Noether"]
"#,
    )
    .await?;

    let config = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .fallback_cwd(Some(codex_home.path().to_path_buf()))
        .build()
        .await?;
    let role = config
        .agent_roles
        .get("researcher")
        .expect("researcher role should load");
    assert_eq!(role.description.as_deref(), Some("Role metadata from file"));
    assert_eq!(role.config_file.as_ref(), Some(&role_config_path));
    assert_eq!(
        role.nickname_candidates
            .as_ref()
            .map(|candidates| candidates.iter().map(String::as_str).collect::<Vec<_>>()),
        Some(vec!["Hypatia"])
    );

    Ok(())
}

#[tokio::test]
async fn agent_role_file_without_developer_instructions_is_dropped_with_warning()
-> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let repo_root = TempDir::new()?;
    let nested_cwd = repo_root.path().join("packages").join("app");
    std::fs::create_dir_all(repo_root.path().join(".git"))?;
    std::fs::create_dir_all(&nested_cwd)?;

    let workspace_key = repo_root.path().to_string_lossy().replace('\\', "\\\\");
    tokio::fs::write(
        codex_home.path().join(CONFIG_TOML_FILE),
        format!(
            r#"[projects."{workspace_key}"]
trust_level = "trusted"
"#
        ),
    )
    .await?;

    let standalone_agents_dir = repo_root.path().join(".codex").join("agents");
    tokio::fs::create_dir_all(&standalone_agents_dir).await?;
    tokio::fs::write(
        standalone_agents_dir.join("researcher.toml"),
        r#"
name = "researcher"
description = "Role metadata from file"
model = "gpt-5.2"
"#,
    )
    .await?;
    tokio::fs::write(
        standalone_agents_dir.join("reviewer.toml"),
        r#"
name = "reviewer"
description = "Review role"
developer_instructions = "Review carefully"
model = "gpt-5.2"
"#,
    )
    .await?;

    let config = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .harness_overrides(ConfigOverrides {
            cwd: Some(nested_cwd),
            ..Default::default()
        })
        .build()
        .await?;
    assert!(!config.agent_roles.contains_key("researcher"));
    assert_eq!(
        config
            .agent_roles
            .get("reviewer")
            .and_then(|role| role.description.as_deref()),
        Some("Review role")
    );
    assert!(
        config
            .startup_warnings
            .iter()
            .any(|warning| warning.contains("must define `developer_instructions`"))
    );

    Ok(())
}

#[tokio::test]
async fn legacy_agent_role_config_file_allows_missing_developer_instructions() -> std::io::Result<()>
{
    let codex_home = TempDir::new()?;
    let role_config_path = codex_home.path().join("agents").join("researcher.toml");
    tokio::fs::create_dir_all(
        role_config_path
            .parent()
            .expect("role config should have a parent directory"),
    )
    .await?;
    tokio::fs::write(
        &role_config_path,
        r#"
model = "gpt-5.2"
model_reasoning_effort = "high"
"#,
    )
    .await?;
    tokio::fs::write(
        codex_home.path().join(CONFIG_TOML_FILE),
        r#"[agents.researcher]
description = "Research role from config"
config_file = "./agents/researcher.toml"
"#,
    )
    .await?;

    let config = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .fallback_cwd(Some(codex_home.path().to_path_buf()))
        .build()
        .await?;
    assert_eq!(
        config
            .agent_roles
            .get("researcher")
            .and_then(|role| role.description.as_deref()),
        Some("Research role from config")
    );
    assert_eq!(
        config
            .agent_roles
            .get("researcher")
            .and_then(|role| role.config_file.as_ref()),
        Some(&role_config_path)
    );

    Ok(())
}

#[tokio::test]
async fn agent_role_without_description_after_merge_is_dropped_with_warning() -> std::io::Result<()>
{
    let codex_home = TempDir::new()?;
    let role_config_path = codex_home.path().join("agents").join("researcher.toml");
    tokio::fs::create_dir_all(
        role_config_path
            .parent()
            .expect("role config should have a parent directory"),
    )
    .await?;
    tokio::fs::write(
        &role_config_path,
        r#"
developer_instructions = "Research carefully"
model = "gpt-5.2"
"#,
    )
    .await?;
    tokio::fs::write(
        codex_home.path().join(CONFIG_TOML_FILE),
        r#"[agents.researcher]
config_file = "./agents/researcher.toml"

[agents.reviewer]
description = "Review role"
"#,
    )
    .await?;

    let config = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .fallback_cwd(Some(codex_home.path().to_path_buf()))
        .build()
        .await?;
    assert!(!config.agent_roles.contains_key("researcher"));
    assert_eq!(
        config
            .agent_roles
            .get("reviewer")
            .and_then(|role| role.description.as_deref()),
        Some("Review role")
    );
    assert!(
        config
            .startup_warnings
            .iter()
            .any(|warning| warning.contains("agent role `researcher` must define a description"))
    );

    Ok(())
}

#[tokio::test]
async fn discovered_agent_role_file_without_name_is_dropped_with_warning() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let repo_root = TempDir::new()?;
    let nested_cwd = repo_root.path().join("packages").join("app");
    std::fs::create_dir_all(repo_root.path().join(".git"))?;
    std::fs::create_dir_all(&nested_cwd)?;

    let workspace_key = repo_root.path().to_string_lossy().replace('\\', "\\\\");
    tokio::fs::write(
        codex_home.path().join(CONFIG_TOML_FILE),
        format!(
            r#"[projects."{workspace_key}"]
trust_level = "trusted"
"#
        ),
    )
    .await?;

    let standalone_agents_dir = repo_root.path().join(".codex").join("agents");
    tokio::fs::create_dir_all(&standalone_agents_dir).await?;
    tokio::fs::write(
        standalone_agents_dir.join("researcher.toml"),
        r#"
description = "Role metadata from file"
developer_instructions = "Research carefully"
"#,
    )
    .await?;
    tokio::fs::write(
        standalone_agents_dir.join("reviewer.toml"),
        r#"
name = "reviewer"
description = "Review role"
developer_instructions = "Review carefully"
"#,
    )
    .await?;

    let config = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .harness_overrides(ConfigOverrides {
            cwd: Some(nested_cwd),
            ..Default::default()
        })
        .build()
        .await?;
    assert!(!config.agent_roles.contains_key("researcher"));
    assert_eq!(
        config
            .agent_roles
            .get("reviewer")
            .and_then(|role| role.description.as_deref()),
        Some("Review role")
    );
    assert!(
        config
            .startup_warnings
            .iter()
            .any(|warning| warning.contains("must define a non-empty `name`"))
    );

    Ok(())
}

#[tokio::test]
async fn agent_role_file_name_takes_precedence_over_config_key() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let role_config_path = codex_home.path().join("agents").join("researcher.toml");
    tokio::fs::create_dir_all(
        role_config_path
            .parent()
            .expect("role config should have a parent directory"),
    )
    .await?;
    tokio::fs::write(
        &role_config_path,
        r#"
name = "archivist"
description = "Role metadata from file"
developer_instructions = "Research carefully"
model = "gpt-5.2"
"#,
    )
    .await?;
    tokio::fs::write(
        codex_home.path().join(CONFIG_TOML_FILE),
        r#"[agents.researcher]
description = "Research role from config"
config_file = "./agents/researcher.toml"
"#,
    )
    .await?;

    let config = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .fallback_cwd(Some(codex_home.path().to_path_buf()))
        .build()
        .await?;
    assert_eq!(config.agent_roles.contains_key("researcher"), false);
    let role = config
        .agent_roles
        .get("archivist")
        .expect("role should use file-provided name");
    assert_eq!(role.description.as_deref(), Some("Role metadata from file"));
    assert_eq!(role.config_file.as_ref(), Some(&role_config_path));

    Ok(())
}

#[tokio::test]
async fn loads_legacy_split_agent_roles_from_config_toml() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let researcher_path = codex_home.path().join("agents").join("researcher.toml");
    let reviewer_path = codex_home.path().join("agents").join("reviewer.toml");
    tokio::fs::create_dir_all(
        researcher_path
            .parent()
            .expect("role config should have a parent directory"),
    )
    .await?;
    tokio::fs::write(
        &researcher_path,
        "developer_instructions = \"Research carefully\"\nmodel = \"gpt-5\"",
    )
    .await?;
    tokio::fs::write(
        &reviewer_path,
        "developer_instructions = \"Review carefully\"\nmodel = \"gpt-4.1\"",
    )
    .await?;
    tokio::fs::write(
        codex_home.path().join(CONFIG_TOML_FILE),
        r#"[agents.researcher]
description = "Research role"
config_file = "./agents/researcher.toml"
nickname_candidates = ["Hypatia", "Noether"]

[agents.reviewer]
description = "Review role"
config_file = "./agents/reviewer.toml"
nickname_candidates = ["Atlas"]
"#,
    )
    .await?;

    let config = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .fallback_cwd(Some(codex_home.path().to_path_buf()))
        .build()
        .await?;

    assert_eq!(
        config
            .agent_roles
            .get("researcher")
            .and_then(|role| role.description.as_deref()),
        Some("Research role")
    );
    assert_eq!(
        config
            .agent_roles
            .get("researcher")
            .and_then(|role| role.config_file.as_ref()),
        Some(&researcher_path)
    );
    assert_eq!(
        config
            .agent_roles
            .get("researcher")
            .and_then(|role| role.nickname_candidates.as_ref())
            .map(|candidates| candidates.iter().map(String::as_str).collect::<Vec<_>>()),
        Some(vec!["Hypatia", "Noether"])
    );
    assert_eq!(
        config
            .agent_roles
            .get("reviewer")
            .and_then(|role| role.description.as_deref()),
        Some("Review role")
    );
    assert_eq!(
        config
            .agent_roles
            .get("reviewer")
            .and_then(|role| role.config_file.as_ref()),
        Some(&reviewer_path)
    );
    assert_eq!(
        config
            .agent_roles
            .get("reviewer")
            .and_then(|role| role.nickname_candidates.as_ref())
            .map(|candidates| candidates.iter().map(String::as_str).collect::<Vec<_>>()),
        Some(vec!["Atlas"])
    );

    Ok(())
}

#[tokio::test]
async fn discovers_multiple_standalone_agent_role_files() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let repo_root = TempDir::new()?;
    let nested_cwd = repo_root.path().join("packages").join("app");
    std::fs::create_dir_all(repo_root.path().join(".git"))?;
    std::fs::create_dir_all(&nested_cwd)?;

    let workspace_key = repo_root.path().to_string_lossy().replace('\\', "\\\\");
    std::fs::write(
        codex_home.path().join(CONFIG_TOML_FILE),
        format!(
            r#"[projects."{workspace_key}"]
trust_level = "trusted"
"#
        ),
    )?;

    let root_agent = repo_root
        .path()
        .join(".codex")
        .join("agents")
        .join("root.toml");
    std::fs::create_dir_all(
        root_agent
            .parent()
            .expect("root agent should have a parent directory"),
    )?;
    std::fs::write(
        &root_agent,
        r#"
name = "researcher"
description = "from root"
developer_instructions = "Research carefully"
"#,
    )?;

    let nested_agent = repo_root
        .path()
        .join("packages")
        .join(".codex")
        .join("agents")
        .join("review")
        .join("nested.toml");
    std::fs::create_dir_all(
        nested_agent
            .parent()
            .expect("nested agent should have a parent directory"),
    )?;
    std::fs::write(
        &nested_agent,
        r#"
name = "reviewer"
description = "from nested"
nickname_candidates = ["Atlas"]
developer_instructions = "Review carefully"
"#,
    )?;

    let sibling_agent = repo_root
        .path()
        .join("packages")
        .join(".codex")
        .join("agents")
        .join("writer.toml");
    std::fs::create_dir_all(
        sibling_agent
            .parent()
            .expect("sibling agent should have a parent directory"),
    )?;
    std::fs::write(
        &sibling_agent,
        r#"
name = "writer"
description = "from sibling"
nickname_candidates = ["Sagan"]
developer_instructions = "Write carefully"
"#,
    )?;

    let config = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .harness_overrides(ConfigOverrides {
            cwd: Some(nested_cwd),
            ..Default::default()
        })
        .build()
        .await?;

    assert_eq!(
        config
            .agent_roles
            .get("researcher")
            .and_then(|role| role.description.as_deref()),
        Some("from root")
    );
    assert_eq!(
        config
            .agent_roles
            .get("reviewer")
            .and_then(|role| role.description.as_deref()),
        Some("from nested")
    );
    assert_eq!(
        config
            .agent_roles
            .get("reviewer")
            .and_then(|role| role.nickname_candidates.as_ref())
            .map(|candidates| candidates.iter().map(String::as_str).collect::<Vec<_>>()),
        Some(vec!["Atlas"])
    );
    assert_eq!(
        config
            .agent_roles
            .get("writer")
            .and_then(|role| role.description.as_deref()),
        Some("from sibling")
    );
    assert_eq!(
        config
            .agent_roles
            .get("writer")
            .and_then(|role| role.nickname_candidates.as_ref())
            .map(|candidates| candidates.iter().map(String::as_str).collect::<Vec<_>>()),
        Some(vec!["Sagan"])
    );

    Ok(())
}

#[tokio::test]
async fn mixed_legacy_and_standalone_agent_role_sources_merge_with_precedence()
-> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let repo_root = TempDir::new()?;
    let nested_cwd = repo_root.path().join("packages").join("app");
    std::fs::create_dir_all(repo_root.path().join(".git"))?;
    std::fs::create_dir_all(&nested_cwd)?;

    let workspace_key = repo_root.path().to_string_lossy().replace('\\', "\\\\");
    tokio::fs::write(
        codex_home.path().join(CONFIG_TOML_FILE),
        format!(
            r#"[projects."{workspace_key}"]
trust_level = "trusted"

[agents.researcher]
description = "Research role from config"
config_file = "./agents/researcher.toml"
nickname_candidates = ["Noether"]

[agents.critic]
description = "Critic role from config"
config_file = "./agents/critic.toml"
nickname_candidates = ["Ada"]
"#
        ),
    )
    .await?;

    let home_agents_dir = codex_home.path().join("agents");
    tokio::fs::create_dir_all(&home_agents_dir).await?;
    tokio::fs::write(
        home_agents_dir.join("researcher.toml"),
        r#"
developer_instructions = "Research carefully"
model = "gpt-5.2"
"#,
    )
    .await?;
    tokio::fs::write(
        home_agents_dir.join("critic.toml"),
        r#"
developer_instructions = "Critique carefully"
model = "gpt-4.1"
"#,
    )
    .await?;

    let standalone_agents_dir = repo_root.path().join(".codex").join("agents");
    tokio::fs::create_dir_all(&standalone_agents_dir).await?;
    tokio::fs::write(
        standalone_agents_dir.join("researcher.toml"),
        r#"
name = "researcher"
description = "Research role from file"
nickname_candidates = ["Hypatia"]
developer_instructions = "Research from file"
model = "gpt-5-mini"
"#,
    )
    .await?;
    tokio::fs::write(
        standalone_agents_dir.join("writer.toml"),
        r#"
name = "writer"
description = "Writer role from file"
nickname_candidates = ["Sagan"]
developer_instructions = "Write carefully"
model = "gpt-5.2"
"#,
    )
    .await?;

    let config = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .harness_overrides(ConfigOverrides {
            cwd: Some(nested_cwd),
            ..Default::default()
        })
        .build()
        .await?;

    assert_eq!(
        config
            .agent_roles
            .get("researcher")
            .and_then(|role| role.description.as_deref()),
        Some("Research role from file")
    );
    assert_eq!(
        config
            .agent_roles
            .get("researcher")
            .and_then(|role| role.config_file.as_ref()),
        Some(&standalone_agents_dir.join("researcher.toml"))
    );
    assert_eq!(
        config
            .agent_roles
            .get("researcher")
            .and_then(|role| role.nickname_candidates.as_ref())
            .map(|candidates| candidates.iter().map(String::as_str).collect::<Vec<_>>()),
        Some(vec!["Hypatia"])
    );
    assert_eq!(
        config
            .agent_roles
            .get("critic")
            .and_then(|role| role.description.as_deref()),
        Some("Critic role from config")
    );
    assert_eq!(
        config
            .agent_roles
            .get("critic")
            .and_then(|role| role.config_file.as_ref()),
        Some(&home_agents_dir.join("critic.toml"))
    );
    assert_eq!(
        config
            .agent_roles
            .get("critic")
            .and_then(|role| role.nickname_candidates.as_ref())
            .map(|candidates| candidates.iter().map(String::as_str).collect::<Vec<_>>()),
        Some(vec!["Ada"])
    );
    assert_eq!(
        config
            .agent_roles
            .get("writer")
            .and_then(|role| role.description.as_deref()),
        Some("Writer role from file")
    );
    assert_eq!(
        config
            .agent_roles
            .get("writer")
            .and_then(|role| role.nickname_candidates.as_ref())
            .map(|candidates| candidates.iter().map(String::as_str).collect::<Vec<_>>()),
        Some(vec!["Sagan"])
    );

    Ok(())
}

#[tokio::test]
async fn higher_precedence_agent_role_can_inherit_description_from_lower_layer()
-> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let repo_root = TempDir::new()?;
    let nested_cwd = repo_root.path().join("packages").join("app");
    std::fs::create_dir_all(repo_root.path().join(".git"))?;
    std::fs::create_dir_all(&nested_cwd)?;

    let workspace_key = repo_root.path().to_string_lossy().replace('\\', "\\\\");
    tokio::fs::write(
        codex_home.path().join(CONFIG_TOML_FILE),
        format!(
            r#"[projects."{workspace_key}"]
trust_level = "trusted"

[agents.researcher]
description = "Research role from config"
config_file = "./agents/researcher.toml"
"#
        ),
    )
    .await?;

    let home_agents_dir = codex_home.path().join("agents");
    tokio::fs::create_dir_all(&home_agents_dir).await?;
    tokio::fs::write(
        home_agents_dir.join("researcher.toml"),
        r#"
developer_instructions = "Research carefully"
model = "gpt-5.2"
"#,
    )
    .await?;

    let standalone_agents_dir = repo_root.path().join(".codex").join("agents");
    tokio::fs::create_dir_all(&standalone_agents_dir).await?;
    tokio::fs::write(
        standalone_agents_dir.join("researcher.toml"),
        r#"
name = "researcher"
nickname_candidates = ["Hypatia"]
developer_instructions = "Research from file"
model = "gpt-5-mini"
"#,
    )
    .await?;

    let config = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .harness_overrides(ConfigOverrides {
            cwd: Some(nested_cwd),
            ..Default::default()
        })
        .build()
        .await?;

    assert_eq!(
        config
            .agent_roles
            .get("researcher")
            .and_then(|role| role.description.as_deref()),
        Some("Research role from config")
    );
    assert_eq!(
        config
            .agent_roles
            .get("researcher")
            .and_then(|role| role.config_file.as_ref()),
        Some(&standalone_agents_dir.join("researcher.toml"))
    );
    assert_eq!(
        config
            .agent_roles
            .get("researcher")
            .and_then(|role| role.nickname_candidates.as_ref())
            .map(|candidates| candidates.iter().map(String::as_str).collect::<Vec<_>>()),
        Some(vec!["Hypatia"])
    );

    Ok(())
}

#[tokio::test]
async fn load_config_resolves_agent_interrupt_message() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let cfg = ConfigToml {
        agents: Some(AgentsToml {
            interrupt_message: Some(false),
            ..Default::default()
        }),
        ..Default::default()
    };

    let config = Config::load_from_base_config_with_overrides(
        cfg,
        ConfigOverrides::default(),
        codex_home.abs(),
    )
    .await?;

    assert!(!config.agent_interrupt_message_enabled);

    Ok(())
}

#[tokio::test]
async fn load_config_normalizes_agent_role_nickname_candidates() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let cfg = ConfigToml {
        agents: Some(AgentsToml {
            max_threads: None,
            max_depth: None,
            job_max_runtime_seconds: None,
            interrupt_message: None,
            roles: BTreeMap::from([(
                "researcher".to_string(),
                AgentRoleToml {
                    description: Some("Research role".to_string()),
                    config_file: None,
                    agent_card_manifest_source: None,
                    agent_card_manifest_version: None,
                    agent_card_manifest: None,
                    nickname_candidates: Some(vec![
                        "  Hypatia  ".to_string(),
                        "Noether".to_string(),
                    ]),
                },
            )]),
        }),
        ..Default::default()
    };

    let config = Config::load_from_base_config_with_overrides(
        cfg,
        ConfigOverrides::default(),
        codex_home.abs(),
    )
    .await?;

    assert_eq!(
        config
            .agent_roles
            .get("researcher")
            .and_then(|role| role.nickname_candidates.as_ref())
            .map(|candidates| candidates.iter().map(String::as_str).collect::<Vec<_>>()),
        Some(vec!["Hypatia", "Noether"])
    );

    Ok(())
}

#[tokio::test]
async fn load_config_rejects_empty_agent_role_nickname_candidates() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let cfg = ConfigToml {
        agents: Some(AgentsToml {
            max_threads: None,
            max_depth: None,
            job_max_runtime_seconds: None,
            interrupt_message: None,
            roles: BTreeMap::from([(
                "researcher".to_string(),
                AgentRoleToml {
                    description: Some("Research role".to_string()),
                    config_file: None,
                    agent_card_manifest_source: None,
                    agent_card_manifest_version: None,
                    agent_card_manifest: None,
                    nickname_candidates: Some(Vec::new()),
                },
            )]),
        }),
        ..Default::default()
    };

    let result = Config::load_from_base_config_with_overrides(
        cfg,
        ConfigOverrides::default(),
        codex_home.abs(),
    )
    .await;
    let err = result.expect_err("empty nickname candidates should be rejected");
    assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
    assert!(
        err.to_string()
            .contains("agents.researcher.nickname_candidates")
    );

    Ok(())
}

#[tokio::test]
async fn load_config_rejects_duplicate_agent_role_nickname_candidates() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let cfg = ConfigToml {
        agents: Some(AgentsToml {
            max_threads: None,
            max_depth: None,
            job_max_runtime_seconds: None,
            interrupt_message: None,
            roles: BTreeMap::from([(
                "researcher".to_string(),
                AgentRoleToml {
                    description: Some("Research role".to_string()),
                    config_file: None,
                    agent_card_manifest_source: None,
                    agent_card_manifest_version: None,
                    agent_card_manifest: None,
                    nickname_candidates: Some(vec!["Hypatia".to_string(), " Hypatia ".to_string()]),
                },
            )]),
        }),
        ..Default::default()
    };

    let result = Config::load_from_base_config_with_overrides(
        cfg,
        ConfigOverrides::default(),
        codex_home.abs(),
    )
    .await;
    let err = result.expect_err("duplicate nickname candidates should be rejected");
    assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
    assert!(
        err.to_string()
            .contains("agents.researcher.nickname_candidates cannot contain duplicates")
    );

    Ok(())
}

#[tokio::test]
async fn load_config_rejects_unsafe_agent_role_nickname_candidates() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let cfg = ConfigToml {
        agents: Some(AgentsToml {
            max_threads: None,
            max_depth: None,
            job_max_runtime_seconds: None,
            interrupt_message: None,
            roles: BTreeMap::from([(
                "researcher".to_string(),
                AgentRoleToml {
                    description: Some("Research role".to_string()),
                    config_file: None,
                    agent_card_manifest_source: None,
                    agent_card_manifest_version: None,
                    agent_card_manifest: None,
                    nickname_candidates: Some(vec!["Agent <One>".to_string()]),
                },
            )]),
        }),
        ..Default::default()
    };

    let result = Config::load_from_base_config_with_overrides(
        cfg,
        ConfigOverrides::default(),
        codex_home.abs(),
    )
    .await;
    let err = result.expect_err("unsafe nickname candidates should be rejected");
    assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
    assert!(err.to_string().contains(
            "agents.researcher.nickname_candidates may only contain ASCII letters, digits, spaces, hyphens, and underscores"
        ));

    Ok(())
}

#[tokio::test]
async fn model_catalog_json_loads_from_path() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let catalog_path = codex_home.path().join("catalog.json");
    let mut catalog = bundled_models_response()
        .unwrap_or_else(|err| panic!("bundled models.json should parse: {err}"));
    catalog.models = catalog.models.into_iter().take(1).collect();
    std::fs::write(
        &catalog_path,
        serde_json::to_string(&catalog).expect("serialize catalog"),
    )?;

    let cfg = ConfigToml {
        model_catalog_json: Some(catalog_path.abs()),
        ..Default::default()
    };

    let config = Config::load_from_base_config_with_overrides(
        cfg,
        ConfigOverrides::default(),
        codex_home.abs(),
    )
    .await?;

    assert_eq!(config.model_catalog, Some(catalog));
    Ok(())
}

#[tokio::test]
async fn model_catalog_json_rejects_empty_catalog() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let catalog_path = codex_home.path().join("catalog.json");
    std::fs::write(&catalog_path, r#"{"models":[]}"#)?;

    let cfg = ConfigToml {
        model_catalog_json: Some(catalog_path.abs()),
        ..Default::default()
    };

    let err = Config::load_from_base_config_with_overrides(
        cfg,
        ConfigOverrides::default(),
        codex_home.abs(),
    )
    .await
    .expect_err("empty custom catalog should fail config load");

    assert_eq!(err.kind(), ErrorKind::InvalidData);
    assert!(
        err.to_string().contains("must contain at least one model"),
        "unexpected error: {err}"
    );
    Ok(())
}

fn create_test_fixture() -> std::io::Result<PrecedenceTestFixture> {
    let toml = r#"
model = "o3"
approval_policy = "untrusted"

# Can be used to determine which profile to use if not specified by
# `ConfigOverrides`.
profile = "gpt3"

[analytics]
enabled = true

[model_providers.openai-custom]
name = "OpenAI custom"
base_url = "https://api.openai.com/v1"
env_key = "OPENAI_API_KEY"
wire_api = "responses"
request_max_retries = 4            # retry failed HTTP requests
stream_max_retries = 10            # retry dropped SSE streams
stream_idle_timeout_ms = 300000    # 5m idle timeout
websocket_connect_timeout_ms = 15000

[profiles.o3]
model = "o3"
model_provider = "openai"
approval_policy = "never"
model_reasoning_effort = "high"
model_reasoning_summary = "detailed"

[profiles.gpt3]
model = "gpt-3.5-turbo"
model_provider = "openai-custom"

[profiles.zdr]
model = "o3"
model_provider = "openai"
approval_policy = "on-failure"

[profiles.zdr.analytics]
enabled = false

[profiles.gpt5]
model = "gpt-5.4"
model_provider = "openai"
approval_policy = "on-failure"
model_reasoning_effort = "high"
model_reasoning_summary = "detailed"
model_verbosity = "high"
"#;

    let cfg: ConfigToml = toml::from_str(toml).expect("TOML deserialization should succeed");

    // Use a temporary directory for the cwd so it does not contain an
    // AGENTS.md file.
    let cwd_temp_dir = TempDir::new().unwrap();
    let cwd = cwd_temp_dir.path().to_path_buf();
    // Make it look like a Git repo so it does not search for AGENTS.md in
    // a parent folder, either.
    std::fs::write(cwd.join(".git"), "gitdir: nowhere")?;

    let codex_home_temp_dir = TempDir::new().unwrap();

    let openai_custom_provider = ModelProviderInfo {
        name: "OpenAI custom".to_string(),
        base_url: Some("https://api.openai.com/v1".to_string()),
        env_key: Some("OPENAI_API_KEY".to_string()),
        wire_api: WireApi::Responses,
        env_key_instructions: None,
        experimental_bearer_token: None,
        auth: None,
        aws: None,
        query_params: None,
        http_headers: None,
        env_http_headers: None,
        request_max_retries: Some(4),
        stream_max_retries: Some(10),
        stream_idle_timeout_ms: Some(300_000),
        websocket_connect_timeout_ms: Some(15_000),
        requires_openai_auth: false,
        supports_websockets: false,
    };
    let model_provider_map = {
        let mut model_provider_map =
            built_in_model_providers(/* openai_base_url */ /*openai_base_url*/ None);
        model_provider_map.insert("openai-custom".to_string(), openai_custom_provider.clone());
        model_provider_map
    };

    let openai_provider = model_provider_map
        .get("openai")
        .expect("openai provider should exist")
        .clone();

    Ok(PrecedenceTestFixture {
        cwd: cwd_temp_dir,
        codex_home: codex_home_temp_dir,
        cfg,
        model_provider_map,
        openai_provider,
        openai_custom_provider,
    })
}

/// Users can specify config values at multiple levels that have the
/// following precedence:
///
/// 1. custom command-line argument, e.g. `--model o3`
/// 2. as part of a profile, where the `--profile` is specified via a CLI
///    (or in the config file itself)
/// 3. as an entry in `config.toml`, e.g. `model = "o3"`
/// 4. the default value for a required field defined in code.
///
/// Note that profiles are the recommended way to specify a group of
/// configuration options together.
#[tokio::test]
async fn test_precedence_fixture_with_o3_profile() -> std::io::Result<()> {
    let fixture = create_test_fixture()?;

    let o3_profile_overrides = ConfigOverrides {
        config_profile: Some("o3".to_string()),
        cwd: Some(fixture.cwd_path()),
        ..Default::default()
    };
    let o3_profile_config: Config = Config::load_from_base_config_with_overrides(
        fixture.cfg.clone(),
        o3_profile_overrides,
        fixture.codex_home(),
    )
    .await?;
    assert_eq!(
        Config {
            config_generation: ConfigGeneration::default(),
            model: Some("o3".to_string()),
            review_model: None,
            model_context_window: None,
            model_auto_compact_token_limit: None,
            service_tier: None,
            model_provider_id: "openai".to_string(),
            model_provider: fixture.openai_provider.clone(),
            permissions: Permissions {
                approval_policy: Constrained::allow_any(AskForApproval::Never),
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
            model_verbosity: None,
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
            active_profile: Some("o3".to_string()),
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
        },
        o3_profile_config
    );
    Ok(())
}

#[tokio::test]
async fn metrics_exporter_defaults_to_statsig_when_missing() -> std::io::Result<()> {
    let fixture = create_test_fixture()?;

    let config = Config::load_from_base_config_with_overrides(
        fixture.cfg.clone(),
        ConfigOverrides {
            cwd: Some(fixture.cwd_path()),
            ..Default::default()
        },
        fixture.codex_home(),
    )
    .await?;

    assert_eq!(config.otel.metrics_exporter, OtelExporterKind::Statsig);
    Ok(())
}

#[tokio::test]
async fn trace_exporter_defaults_to_none_when_log_exporter_is_set() -> std::io::Result<()> {
    let fixture = create_test_fixture()?;
    let mut cfg = fixture.cfg.clone();
    cfg.otel = Some(OtelConfigToml {
        exporter: Some(OtelExporterKind::OtlpHttp {
            endpoint: "http://localhost:14318/v1/logs".to_string(),
            headers: HashMap::new(),
            protocol: codex_config::types::OtelHttpProtocol::Binary,
            tls: None,
        }),
        metrics_exporter: Some(OtelExporterKind::None),
        ..Default::default()
    });

    let config = Config::load_from_base_config_with_overrides(
        cfg,
        ConfigOverrides {
            cwd: Some(fixture.cwd_path()),
            ..Default::default()
        },
        fixture.codex_home(),
    )
    .await?;

    assert!(matches!(
        config.otel.exporter,
        OtelExporterKind::OtlpHttp { .. }
    ));
    assert_eq!(config.otel.trace_exporter, OtelExporterKind::None);
    Ok(())
}

#[tokio::test]
async fn load_config_applies_otel_trace_metadata() -> std::io::Result<()> {
    let mut fixture = create_test_fixture()?;
    fixture.cfg = toml::from_str(
        r#"
[otel.span_attributes]
"example.trace_attr" = "enabled"

[otel.tracestate.example]
alpha = "one"
beta = "two"
"#,
    )
    .expect("TOML deserialization should succeed");

    let config = Config::load_from_base_config_with_overrides(
        fixture.cfg.clone(),
        ConfigOverrides {
            cwd: Some(fixture.cwd_path()),
            ..Default::default()
        },
        fixture.codex_home(),
    )
    .await?;

    assert_eq!(
        config.otel.span_attributes,
        BTreeMap::from([("example.trace_attr".to_string(), "enabled".to_string())])
    );
    assert_eq!(
        config.otel.tracestate,
        BTreeMap::from([(
            "example".to_string(),
            BTreeMap::from([
                ("alpha".to_string(), "one".to_string()),
                ("beta".to_string(), "two".to_string()),
            ]),
        )])
    );
    Ok(())
}

#[tokio::test]
async fn load_config_drops_invalid_otel_trace_metadata_entries() -> std::io::Result<()> {
    let mut fixture = create_test_fixture()?;
    fixture.cfg = toml::from_str(
        r#"
[otel]
environment = "test"

[otel.span_attributes]
"" = "missing-key"
"example.trace_attr" = "enabled"

[otel.tracestate.example]
alpha = "one"
beta = "two\ntoo"

[otel.tracestate.bad]
alpha = "one\ntwo"
"#,
    )
    .expect("TOML deserialization should succeed");

    let config = Config::load_from_base_config_with_overrides(
        fixture.cfg.clone(),
        ConfigOverrides {
            cwd: Some(fixture.cwd_path()),
            ..Default::default()
        },
        fixture.codex_home(),
    )
    .await?;

    assert_eq!(config.otel.environment, "test");
    assert_eq!(
        config.otel.span_attributes,
        BTreeMap::from([("example.trace_attr".to_string(), "enabled".to_string())])
    );
    assert_eq!(
        config.otel.tracestate,
        BTreeMap::from([(
            "example".to_string(),
            BTreeMap::from([("alpha".to_string(), "one".to_string())]),
        )])
    );
    assert!(
        config.startup_warnings.iter().any(|warning| {
            warning.contains("Ignoring invalid `otel.span_attributes` config")
                && warning.contains("configured span attribute key must not be empty")
        }),
        "{:?}",
        config.startup_warnings
    );
    assert!(
        config.startup_warnings.iter().any(|warning| {
            warning.contains("Ignoring invalid `otel.tracestate` config")
                && warning.contains("invalid configured tracestate value for example.beta")
        }),
        "{:?}",
        config.startup_warnings
    );
    assert!(
        config.startup_warnings.iter().any(|warning| {
            warning.contains("Ignoring invalid `otel.tracestate` config")
                && warning.contains("invalid configured tracestate value for bad.alpha")
        }),
        "{:?}",
        config.startup_warnings
    );
    Ok(())
}

#[tokio::test]
async fn explicit_null_service_tier_override_sets_fast_default_opt_out() -> std::io::Result<()> {
    let fixture = create_test_fixture()?;

    let config = Config::load_from_base_config_with_overrides(
        fixture.cfg.clone(),
        ConfigOverrides {
            cwd: Some(fixture.cwd_path()),
            service_tier: Some(None),
            ..Default::default()
        },
        fixture.codex_home(),
    )
    .await?;

    assert_eq!(config.service_tier, None);
    assert_eq!(config.notices.fast_default_opt_out, Some(true));
    Ok(())
}

#[tokio::test]
async fn legacy_fast_service_tier_override_uses_priority_request_value() -> std::io::Result<()> {
    let fixture = create_test_fixture()?;

    let config = Config::load_from_base_config_with_overrides(
        fixture.cfg.clone(),
        ConfigOverrides {
            cwd: Some(fixture.cwd_path()),
            service_tier: Some(Some("fast".to_string())),
            ..Default::default()
        },
        fixture.codex_home(),
    )
    .await?;

    assert_eq!(
        config.service_tier,
        Some(ServiceTier::Fast.request_value().to_string())
    );
    Ok(())
}

#[tokio::test]
async fn config_toml_priority_service_tier_uses_priority_request_value() -> std::io::Result<()> {
    let mut fixture = create_test_fixture()?;
    fixture.cfg.service_tier = Some(ServiceTier::Fast.request_value().to_string());
    let cwd = fixture.cwd_path();
    let codex_home = fixture.codex_home();

    let config = Config::load_from_base_config_with_overrides(
        fixture.cfg,
        ConfigOverrides {
            cwd: Some(cwd),
            ..Default::default()
        },
        codex_home,
    )
    .await?;

    assert_eq!(
        config.service_tier,
        Some(ServiceTier::Fast.request_value().to_string())
    );
    Ok(())
}

#[tokio::test]
async fn config_toml_service_tier_accepts_arbitrary_string() -> std::io::Result<()> {
    let mut fixture = create_test_fixture()?;
    fixture.cfg.service_tier = Some("experimental-tier-id".to_string());
    let cwd = fixture.cwd_path();
    let codex_home = fixture.codex_home();

    let config = Config::load_from_base_config_with_overrides(
        fixture.cfg,
        ConfigOverrides {
            cwd: Some(cwd),
            ..Default::default()
        },
        codex_home,
    )
    .await?;

    assert_eq!(
        config.service_tier,
        Some("experimental-tier-id".to_string())
    );
    Ok(())
}

#[tokio::test]
async fn config_toml_legacy_fast_service_tier_uses_priority_request_value() -> std::io::Result<()> {
    let mut fixture = create_test_fixture()?;
    fixture.cfg.service_tier = Some("fast".to_string());
    let cwd = fixture.cwd_path();
    let codex_home = fixture.codex_home();

    let config = Config::load_from_base_config_with_overrides(
        fixture.cfg,
        ConfigOverrides {
            cwd: Some(cwd),
            ..Default::default()
        },
        codex_home,
    )
    .await?;

    assert_eq!(
        config.service_tier,
        Some(ServiceTier::Fast.request_value().to_string())
    );
    Ok(())
}

#[tokio::test]
async fn fast_default_opt_out_notice_config_is_respected() -> std::io::Result<()> {
    let fixture = create_test_fixture()?;
    let mut cfg = fixture.cfg.clone();
    cfg.notice = Some(Notice {
        fast_default_opt_out: Some(true),
        ..Default::default()
    });

    let config = Config::load_from_base_config_with_overrides(
        cfg,
        ConfigOverrides {
            cwd: Some(fixture.cwd_path()),
            ..Default::default()
        },
        fixture.codex_home(),
    )
    .await?;

    assert_eq!(config.service_tier, None);
    assert_eq!(config.notices.fast_default_opt_out, Some(true));
    Ok(())
}

#[tokio::test]
async fn test_precedence_fixture_with_gpt3_profile() -> std::io::Result<()> {
    let fixture = create_test_fixture()?;

    let gpt3_profile_overrides = ConfigOverrides {
        config_profile: Some("gpt3".to_string()),
        cwd: Some(fixture.cwd_path()),
        ..Default::default()
    };
    let gpt3_profile_config = Config::load_from_base_config_with_overrides(
        fixture.cfg.clone(),
        gpt3_profile_overrides,
        fixture.codex_home(),
    )
    .await?;
    let expected_gpt3_profile_config = Config {
        config_generation: ConfigGeneration::default(),
        model: Some("gpt-3.5-turbo".to_string()),
        review_model: None,
        model_context_window: None,
        model_auto_compact_token_limit: None,
        service_tier: None,
        model_provider_id: "openai-custom".to_string(),
        model_provider: fixture.openai_custom_provider.clone(),
        permissions: Permissions {
            approval_policy: Constrained::allow_any(AskForApproval::UnlessTrusted),
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
        model_reasoning_effort: None,
        plan_mode_reasoning_effort: None,
        model_reasoning_summary: None,
        model_supports_reasoning_summaries: None,
        model_catalog: None,
        model_verbosity: None,
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
        active_profile: Some("gpt3".to_string()),
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

    assert_eq!(expected_gpt3_profile_config, gpt3_profile_config);

    // Verify that loading without specifying a profile in ConfigOverrides
    // uses the default profile from the config file (which is "gpt3").
    let default_profile_overrides = ConfigOverrides {
        cwd: Some(fixture.cwd_path()),
        ..Default::default()
    };

    let default_profile_config = Config::load_from_base_config_with_overrides(
        fixture.cfg.clone(),
        default_profile_overrides,
        fixture.codex_home(),
    )
    .await?;

    assert_eq!(expected_gpt3_profile_config, default_profile_config);
    Ok(())
}

#[tokio::test]
async fn test_precedence_fixture_with_zdr_profile() -> std::io::Result<()> {
    let fixture = create_test_fixture()?;

    let zdr_profile_overrides = ConfigOverrides {
        config_profile: Some("zdr".to_string()),
        cwd: Some(fixture.cwd_path()),
        ..Default::default()
    };
    let zdr_profile_config = Config::load_from_base_config_with_overrides(
        fixture.cfg.clone(),
        zdr_profile_overrides,
        fixture.codex_home(),
    )
    .await?;
    let expected_zdr_profile_config = Config {
        config_generation: ConfigGeneration::default(),
        model: Some("o3".to_string()),
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
        model_reasoning_effort: None,
        plan_mode_reasoning_effort: None,
        model_reasoning_summary: None,
        model_supports_reasoning_summaries: None,
        model_catalog: None,
        model_verbosity: None,
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
        active_profile: Some("zdr".to_string()),
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
        analytics_enabled: Some(false),
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

    assert_eq!(expected_zdr_profile_config, zdr_profile_config);

    Ok(())
}
