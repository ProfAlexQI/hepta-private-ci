#[test]
fn tui_config_missing_notifications_field_defaults_to_enabled() {
    let cfg = r#"
[tui]
"#;

    let parsed =
        toml::from_str::<ConfigToml>(cfg).expect("TUI config without notifications should succeed");
    let tui = parsed.tui.expect("config should include tui section");

    assert_eq!(
        tui,
        Tui {
            notification_settings: TuiNotificationSettings::default(),
            animations: true,
            show_tooltips: true,
            vim_mode_default: false,
            raw_output_mode: false,
            alternate_screen: AltScreenMode::Auto,
            status_line: None,
            status_line_use_colors: true,
            terminal_title: None,
            theme: None,
            pet: None,
            pet_anchor: TuiPetAnchor::Composer,
            session_picker_view: None,
            keymap: TuiKeymap::default(),
            model_availability_nux: ModelAvailabilityNuxConfig::default(),
            terminal_resize_reflow_max_rows: None,
        }
    );
}

#[tokio::test]
async fn runtime_config_resolves_terminal_resize_reflow_defaults_and_overrides() {
    let cfg = Config::load_from_base_config_with_overrides(
        ConfigToml::default(),
        ConfigOverrides::default(),
        tempdir().expect("tempdir").abs(),
    )
    .await
    .expect("load default config");

    assert_eq!(
        cfg.terminal_resize_reflow,
        TerminalResizeReflowConfig::default()
    );
    assert_eq!(
        cfg.terminal_resize_reflow.max_rows,
        TerminalResizeReflowMaxRows::Auto
    );

    let cfg = Config::load_from_base_config_with_overrides(
        ConfigToml {
            tui: Some(Tui {
                terminal_resize_reflow_max_rows: Some(9000),
                ..Default::default()
            }),
            ..Default::default()
        },
        ConfigOverrides::default(),
        tempdir().expect("tempdir").abs(),
    )
    .await
    .expect("load overridden config");

    assert_eq!(
        cfg.terminal_resize_reflow.max_rows,
        TerminalResizeReflowMaxRows::Limit(9000)
    );

    let cfg = Config::load_from_base_config_with_overrides(
        ConfigToml {
            tui: Some(Tui {
                terminal_resize_reflow_max_rows: Some(0),
                ..Default::default()
            }),
            ..Default::default()
        },
        ConfigOverrides::default(),
        tempdir().expect("tempdir").abs(),
    )
    .await
    .expect("load config with disabled resize reflow limits");

    assert_eq!(
        cfg.terminal_resize_reflow.max_rows,
        TerminalResizeReflowMaxRows::Disabled
    );
}

#[tokio::test]
async fn forced_chatgpt_workspace_id_empty_values_disable_runtime_restriction()
-> std::io::Result<()> {
    let cases: Vec<(&str, &str, Option<Vec<&str>>)> = vec![
        ("unset", "", None),
        ("empty string", r#"forced_chatgpt_workspace_id = """#, None),
        (
            "whitespace string",
            r#"forced_chatgpt_workspace_id = "   ""#,
            None,
        ),
        ("empty list", r#"forced_chatgpt_workspace_id = []"#, None),
        (
            "blank list entries",
            r#"forced_chatgpt_workspace_id = ["", "  "]"#,
            None,
        ),
        (
            "mixed list entries",
            r#"forced_chatgpt_workspace_id = ["", " 123e4567-e89b-42d3-a456-426614174000 ", "123e4567-e89b-42d3-a456-426614174001"]"#,
            Some(vec![
                "123e4567-e89b-42d3-a456-426614174000",
                "123e4567-e89b-42d3-a456-426614174001",
            ]),
        ),
    ];

    for (name, toml, expected) in cases {
        let cfg_toml: ConfigToml = toml::from_str(toml)
            .unwrap_or_else(|err| panic!("{name} should parse forced_chatgpt_workspace_id: {err}"));
        let config = Config::load_from_base_config_with_overrides(
            cfg_toml,
            ConfigOverrides::default(),
            tempdir().expect("tempdir").abs(),
        )
        .await?;

        let expected = expected.map(|values| {
            values
                .into_iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
        });
        assert_eq!(config.forced_chatgpt_workspace_id, expected, "{name}");
    }

    Ok(())
}
#[tokio::test]
async fn legacy_remote_thread_store_endpoint_is_rejected() {
    let cfg: ConfigToml =
        toml::from_str(r#"experimental_thread_store_endpoint = "https://example.com""#)
            .expect("legacy remote thread-store endpoint should still deserialize");

    let err = Config::load_from_base_config_with_overrides(
        cfg,
        ConfigOverrides::default(),
        tempdir().expect("tempdir").abs(),
    )
    .await
    .expect_err("legacy remote thread-store endpoint should be rejected at load time");

    assert!(
        err.to_string()
            .contains("experimental_thread_store_endpoint")
    );
    assert!(err.to_string().contains("no longer supported"));
}

#[test]
fn profile_tui_rejects_unsupported_settings() {
    let err = toml::from_str::<ConfigToml>(
        r#"profile = "work"

[profiles.work.tui]
theme = "dark"
"#,
    )
    .expect_err("profile TUI config should only accept supported fields");

    assert!(err.to_string().contains("unknown field"));
    assert!(err.to_string().contains("theme"));
}

#[tokio::test]
async fn runtime_config_resolves_session_picker_view_default_and_override() {
    let cfg = Config::load_from_base_config_with_overrides(
        ConfigToml::default(),
        ConfigOverrides::default(),
        tempdir().expect("tempdir").abs(),
    )
    .await
    .expect("load default config");

    assert_eq!(cfg.tui_session_picker_view, SessionPickerViewMode::Dense);

    let cfg = Config::load_from_base_config_with_overrides(
        ConfigToml {
            tui: Some(Tui {
                session_picker_view: Some(SessionPickerViewMode::Comfortable),
                ..Default::default()
            }),
            ..Default::default()
        },
        ConfigOverrides::default(),
        tempdir().expect("tempdir").abs(),
    )
    .await
    .expect("load root override config");

    assert_eq!(
        cfg.tui_session_picker_view,
        SessionPickerViewMode::Comfortable
    );

    let cfg_toml = toml::from_str::<ConfigToml>(
        r#"profile = "work"

[tui]
session_picker_view = "dense"

[profiles.work.tui]
session_picker_view = "comfortable"
"#,
    )
    .expect("parse profile scoped tui config");

    let cfg = Config::load_from_base_config_with_overrides(
        cfg_toml,
        ConfigOverrides::default(),
        tempdir().expect("tempdir").abs(),
    )
    .await
    .expect("load profile override config");

    assert_eq!(
        cfg.tui_session_picker_view,
        SessionPickerViewMode::Comfortable
    );
}

#[tokio::test]
async fn test_sandbox_config_parsing() {
    let sandbox_full_access = r#"
sandbox_mode = "danger-full-access"

[sandbox_workspace_write]
network_access = false  # This should be ignored.
"#;
    let sandbox_full_access_cfg = toml::from_str::<ConfigToml>(sandbox_full_access)
        .expect("TOML deserialization should succeed");
    let sandbox_mode_override = None;
    let resolution = derive_legacy_sandbox_policy_for_test(
        &sandbox_full_access_cfg,
        sandbox_mode_override,
        /*profile_sandbox_mode*/ None,
        WindowsSandboxLevel::Disabled,
        /*active_project*/ None,
        /*permission_profile_constraint*/ None,
    )
    .await;
    assert_eq!(resolution, SandboxPolicy::DangerFullAccess);

    let sandbox_read_only = r#"
sandbox_mode = "read-only"

[sandbox_workspace_write]
network_access = true  # This should be ignored.
"#;

    let sandbox_read_only_cfg = toml::from_str::<ConfigToml>(sandbox_read_only)
        .expect("TOML deserialization should succeed");
    let sandbox_mode_override = None;
    let resolution = derive_legacy_sandbox_policy_for_test(
        &sandbox_read_only_cfg,
        sandbox_mode_override,
        /*profile_sandbox_mode*/ None,
        WindowsSandboxLevel::Disabled,
        /*active_project*/ None,
        /*permission_profile_constraint*/ None,
    )
    .await;
    assert_eq!(resolution, SandboxPolicy::new_read_only_policy());

    let writable_root = test_absolute_path("/my/workspace");
    let sandbox_workspace_write = format!(
        r#"
sandbox_mode = "workspace-write"

[sandbox_workspace_write]
writable_roots = [
    {},
]
exclude_tmpdir_env_var = true
exclude_slash_tmp = true

[projects."/tmp/test"]
trust_level = "trusted"
"#,
        serde_json::json!(writable_root)
    );

    let sandbox_workspace_write_cfg = toml::from_str::<ConfigToml>(&sandbox_workspace_write)
        .expect("TOML deserialization should succeed");
    let sandbox_mode_override = None;
    let resolution = derive_legacy_sandbox_policy_for_test(
        &sandbox_workspace_write_cfg,
        sandbox_mode_override,
        /*profile_sandbox_mode*/ None,
        WindowsSandboxLevel::Disabled,
        /*active_project*/ None,
        /*permission_profile_constraint*/ None,
    )
    .await;
    if cfg!(target_os = "windows") {
        assert_eq!(resolution, SandboxPolicy::new_read_only_policy());
    } else {
        assert_eq!(
            resolution,
            SandboxPolicy::WorkspaceWrite {
                writable_roots: vec![writable_root.clone()],
                network_access: false,
                exclude_tmpdir_env_var: true,
                exclude_slash_tmp: true,
            }
        );
    }

    let sandbox_workspace_write = format!(
        r#"
sandbox_mode = "workspace-write"

[sandbox_workspace_write]
writable_roots = [
    {},
]
exclude_tmpdir_env_var = true
exclude_slash_tmp = true
"#,
        serde_json::json!(writable_root)
    );

    let sandbox_workspace_write_cfg = toml::from_str::<ConfigToml>(&sandbox_workspace_write)
        .expect("TOML deserialization should succeed");
    let sandbox_mode_override = None;
    let resolution = derive_legacy_sandbox_policy_for_test(
        &sandbox_workspace_write_cfg,
        sandbox_mode_override,
        /*profile_sandbox_mode*/ None,
        WindowsSandboxLevel::Disabled,
        /*active_project*/ None,
        /*permission_profile_constraint*/ None,
    )
    .await;
    if cfg!(target_os = "windows") {
        assert_eq!(resolution, SandboxPolicy::new_read_only_policy());
    } else {
        assert_eq!(
            resolution,
            SandboxPolicy::WorkspaceWrite {
                writable_roots: vec![writable_root],
                network_access: false,
                exclude_tmpdir_env_var: true,
                exclude_slash_tmp: true,
            }
        );
    }
}

#[tokio::test]
async fn legacy_sandbox_mode_builds_profiles_with_compatible_projection() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let cwd = TempDir::new()?;
    let extra_root = test_absolute_path("/tmp/legacy-extra-root");
    let cases = vec![
        (
            "danger-full-access".to_string(),
            r#"sandbox_mode = "danger-full-access"
"#
            .to_string(),
        ),
        (
            "read-only".to_string(),
            r#"sandbox_mode = "read-only"
"#
            .to_string(),
        ),
        (
            "workspace-write".to_string(),
            format!(
                r#"sandbox_mode = "workspace-write"

[sandbox_workspace_write]
writable_roots = [{}]
exclude_tmpdir_env_var = true
exclude_slash_tmp = true
"#,
                serde_json::json!(extra_root)
            ),
        ),
    ];

    for (name, config_toml) in cases {
        let cfg = toml::from_str::<ConfigToml>(&config_toml)
            .unwrap_or_else(|err| panic!("case `{name}` should parse: {err}"));
        let config = Config::load_from_base_config_with_overrides(
            cfg,
            ConfigOverrides {
                cwd: Some(cwd.path().to_path_buf()),
                ..Default::default()
            },
            codex_home.abs(),
        )
        .await?;

        let sandbox_policy = config.legacy_sandbox_policy();
        let file_system_policy = config.permissions.file_system_sandbox_policy();
        let network_policy = config.permissions.network_sandbox_policy();

        assert_eq!(
            network_policy,
            NetworkSandboxPolicy::from(&sandbox_policy),
            "case `{name}` should preserve network semantics from legacy config"
        );
        assert_eq!(
            file_system_policy
                .to_legacy_sandbox_policy(network_policy, cwd.path())
                .unwrap_or_else(|err| panic!("case `{name}` should round-trip: {err}")),
            sandbox_policy,
            "case `{name}` should preserve its legacy compatibility projection"
        );

        match name.as_str() {
            "danger-full-access" | "read-only" => {
                assert_eq!(
                    file_system_policy,
                    FileSystemSandboxPolicy::from_legacy_sandbox_policy_for_cwd(
                        &sandbox_policy,
                        cwd.path()
                    ),
                    "case `{name}` should match the legacy filesystem projection exactly"
                );
            }
            "workspace-write" => {
                if cfg!(target_os = "windows") {
                    assert_eq!(
                        sandbox_policy,
                        SandboxPolicy::new_read_only_policy(),
                        "legacy workspace-write should keep the existing Windows downgrade when \
                         the experimental Windows sandbox is disabled"
                    );
                    assert_eq!(
                        file_system_policy,
                        FileSystemSandboxPolicy::from_legacy_sandbox_policy_for_cwd(
                            &sandbox_policy,
                            cwd.path()
                        ),
                        "downgraded workspace-write should match the legacy read-only projection"
                    );
                    continue;
                }
                assert_eq!(
                    config.permissions.workspace_roots(),
                    &[cwd.abs(), extra_root.clone()]
                );
                assert!(
                    file_system_policy
                        .entries
                        .contains(&FileSystemSandboxEntry {
                            path: FileSystemPath::Path { path: cwd.abs() },
                            access: FileSystemAccessMode::Write,
                        })
                );
                assert!(
                    file_system_policy
                        .entries
                        .contains(&FileSystemSandboxEntry {
                            path: FileSystemPath::Path {
                                path: extra_root.clone(),
                            },
                            access: FileSystemAccessMode::Write,
                        })
                );
                for subpath in [".git", ".agents", ".codex"] {
                    assert!(
                        file_system_policy
                            .entries
                            .contains(&FileSystemSandboxEntry {
                                path: FileSystemPath::Path {
                                    path: AbsolutePathBuf::resolve_path_against_base(
                                        subpath,
                                        cwd.path()
                                    ),
                                },
                                access: FileSystemAccessMode::Read,
                            }),
                        "case `{name}` should materialize `{subpath}` for the runtime workspace \
                         root"
                    );
                }
            }
            _ => unreachable!("unexpected test case `{name}`"),
        }
    }

    Ok(())
}

#[test]
fn filter_mcp_servers_by_allowlist_enforces_identity_rules() {
    const MISMATCHED_COMMAND_SERVER: &str = "mismatched-command-should-disable";
    const MISMATCHED_URL_SERVER: &str = "mismatched-url-should-disable";
    const MATCHED_COMMAND_SERVER: &str = "matched-command-should-allow";
    const MATCHED_URL_SERVER: &str = "matched-url-should-allow";
    const DIFFERENT_NAME_SERVER: &str = "different-name-should-disable";

    const GOOD_CMD: &str = "good-cmd";
    const GOOD_URL: &str = "https://example.com/good";

    let mut servers = HashMap::from([
        (MISMATCHED_COMMAND_SERVER.to_string(), stdio_mcp("docs-cmd")),
        (
            MISMATCHED_URL_SERVER.to_string(),
            http_mcp("https://example.com/mcp"),
        ),
        (MATCHED_COMMAND_SERVER.to_string(), stdio_mcp(GOOD_CMD)),
        (MATCHED_URL_SERVER.to_string(), http_mcp(GOOD_URL)),
        (DIFFERENT_NAME_SERVER.to_string(), stdio_mcp("same-cmd")),
    ]);
    let source = RequirementSource::LegacyManagedConfigTomlFromMdm;
    let requirements = Sourced::new(
        BTreeMap::from([
            (
                MISMATCHED_URL_SERVER.to_string(),
                McpServerRequirement {
                    identity: McpServerIdentity::Url {
                        url: "https://example.com/other".to_string(),
                    },
                },
            ),
            (
                MISMATCHED_COMMAND_SERVER.to_string(),
                McpServerRequirement {
                    identity: McpServerIdentity::Command {
                        command: "other-cmd".to_string(),
                    },
                },
            ),
            (
                MATCHED_URL_SERVER.to_string(),
                McpServerRequirement {
                    identity: McpServerIdentity::Url {
                        url: GOOD_URL.to_string(),
                    },
                },
            ),
            (
                MATCHED_COMMAND_SERVER.to_string(),
                McpServerRequirement {
                    identity: McpServerIdentity::Command {
                        command: GOOD_CMD.to_string(),
                    },
                },
            ),
        ]),
        source.clone(),
    );
    filter_mcp_servers_by_requirements(&mut servers, Some(&requirements));

    let reason = Some(McpServerDisabledReason::Requirements { source });
    assert_eq!(
        servers
            .iter()
            .map(|(name, server)| (
                name.clone(),
                (server.enabled, server.disabled_reason.clone())
            ))
            .collect::<HashMap<String, (bool, Option<McpServerDisabledReason>)>>(),
        HashMap::from([
            (MISMATCHED_URL_SERVER.to_string(), (false, reason.clone())),
            (
                MISMATCHED_COMMAND_SERVER.to_string(),
                (false, reason.clone()),
            ),
            (MATCHED_URL_SERVER.to_string(), (true, None)),
            (MATCHED_COMMAND_SERVER.to_string(), (true, None)),
            (DIFFERENT_NAME_SERVER.to_string(), (false, reason)),
        ])
    );
}

#[test]
fn filter_mcp_servers_by_allowlist_allows_all_when_unset() {
    let mut servers = HashMap::from([
        ("server-a".to_string(), stdio_mcp("cmd-a")),
        ("server-b".to_string(), http_mcp("https://example.com/b")),
    ]);

    filter_mcp_servers_by_requirements(&mut servers, /*mcp_requirements*/ None);

    assert_eq!(
        servers
            .iter()
            .map(|(name, server)| (
                name.clone(),
                (server.enabled, server.disabled_reason.clone())
            ))
            .collect::<HashMap<String, (bool, Option<McpServerDisabledReason>)>>(),
        HashMap::from([
            ("server-a".to_string(), (true, None)),
            ("server-b".to_string(), (true, None)),
        ])
    );
}

#[test]
fn filter_mcp_servers_by_allowlist_blocks_all_when_empty() {
    let mut servers = HashMap::from([
        ("server-a".to_string(), stdio_mcp("cmd-a")),
        ("server-b".to_string(), http_mcp("https://example.com/b")),
    ]);

    let source = RequirementSource::LegacyManagedConfigTomlFromMdm;
    let requirements = Sourced::new(BTreeMap::new(), source.clone());
    filter_mcp_servers_by_requirements(&mut servers, Some(&requirements));

    let reason = Some(McpServerDisabledReason::Requirements { source });
    assert_eq!(
        servers
            .iter()
            .map(|(name, server)| (
                name.clone(),
                (server.enabled, server.disabled_reason.clone())
            ))
            .collect::<HashMap<String, (bool, Option<McpServerDisabledReason>)>>(),
        HashMap::from([
            ("server-a".to_string(), (false, reason.clone())),
            ("server-b".to_string(), (false, reason)),
        ])
    );
}

#[test]
fn filter_plugin_mcp_servers_by_allowlist_enforces_plugin_and_identity_rules() {
    const MATCHED_SERVER: &str = "matched-should-allow";
    const MISMATCHED_SERVER: &str = "mismatched-should-disable";
    const UNLISTED_SERVER: &str = "unlisted-should-disable";
    const GOOD_CMD: &str = "good-cmd";

    let mut servers = HashMap::from([
        (MATCHED_SERVER.to_string(), stdio_mcp(GOOD_CMD)),
        (MISMATCHED_SERVER.to_string(), stdio_mcp("bad-cmd")),
        (
            UNLISTED_SERVER.to_string(),
            http_mcp("https://example.com/mcp"),
        ),
    ]);
    let source = RequirementSource::CloudRequirements;
    let requirements = Sourced::new(
        BTreeMap::from([(
            "sample@test".to_string(),
            codex_config::PluginRequirementsToml {
                mcp_servers: Some(BTreeMap::from([
                    (
                        MATCHED_SERVER.to_string(),
                        McpServerRequirement {
                            identity: McpServerIdentity::Command {
                                command: GOOD_CMD.to_string(),
                            },
                        },
                    ),
                    (
                        MISMATCHED_SERVER.to_string(),
                        McpServerRequirement {
                            identity: McpServerIdentity::Command {
                                command: GOOD_CMD.to_string(),
                            },
                        },
                    ),
                ])),
            },
        )]),
        source.clone(),
    );

    filter_plugin_mcp_servers_by_requirements("sample@test", &mut servers, Some(&requirements));

    let reason = Some(McpServerDisabledReason::Requirements { source });
    assert_eq!(
        servers
            .iter()
            .map(|(name, server)| (
                name.clone(),
                (server.enabled, server.disabled_reason.clone())
            ))
            .collect::<HashMap<String, (bool, Option<McpServerDisabledReason>)>>(),
        HashMap::from([
            (MATCHED_SERVER.to_string(), (true, None)),
            (MISMATCHED_SERVER.to_string(), (false, reason.clone())),
            (UNLISTED_SERVER.to_string(), (false, reason)),
        ])
    );
}

#[test]
fn filter_plugin_mcp_servers_without_any_allowlist_preserves_all_servers() {
    let original_servers = HashMap::from([
        ("server-a".to_string(), stdio_mcp("cmd-a")),
        ("server-b".to_string(), http_mcp("https://example.com/b")),
    ]);
    let requirements = Sourced::new(
        BTreeMap::from([(
            "sites@openai-bundled".to_string(),
            codex_config::PluginRequirementsToml { mcp_servers: None },
        )]),
        RequirementSource::LegacyManagedConfigTomlFromMdm,
    );

    for plugin_name in ["sites@openai-bundled", "sample@test"] {
        let mut servers = original_servers.clone();
        filter_plugin_mcp_servers_by_requirements(plugin_name, &mut servers, Some(&requirements));
        assert_eq!(servers, original_servers);
    }
}

#[test]
fn filter_plugin_mcp_servers_by_explicit_empty_allowlist_blocks_all() {
    let mut servers = HashMap::from([
        ("server-a".to_string(), stdio_mcp("cmd-a")),
        ("server-b".to_string(), http_mcp("https://example.com/b")),
    ]);
    let source = RequirementSource::LegacyManagedConfigTomlFromMdm;
    let requirements = Sourced::new(
        BTreeMap::from([(
            "sample@test".to_string(),
            codex_config::PluginRequirementsToml {
                mcp_servers: Some(BTreeMap::new()),
            },
        )]),
        source.clone(),
    );

    filter_plugin_mcp_servers_by_requirements("sample@test", &mut servers, Some(&requirements));

    let reason = Some(McpServerDisabledReason::Requirements { source });
    assert_eq!(
        servers
            .iter()
            .map(|(name, server)| (
                name.clone(),
                (server.enabled, server.disabled_reason.clone())
            ))
            .collect::<HashMap<String, (bool, Option<McpServerDisabledReason>)>>(),
        HashMap::from([
            ("server-a".to_string(), (false, reason.clone())),
            ("server-b".to_string(), (false, reason)),
        ])
    );
}

#[test]
fn filter_plugin_mcp_servers_by_allowlist_blocks_unlisted_plugin() {
    let mut servers = HashMap::from([("server-a".to_string(), stdio_mcp("cmd-a"))]);
    let source = RequirementSource::CloudRequirements;
    let requirements = Sourced::new(
        BTreeMap::from([(
            "other@test".to_string(),
            codex_config::PluginRequirementsToml {
                mcp_servers: Some(BTreeMap::from([(
                    "server-a".to_string(),
                    McpServerRequirement {
                        identity: McpServerIdentity::Command {
                            command: "cmd-a".to_string(),
                        },
                    },
                )])),
            },
        )]),
        source.clone(),
    );

    filter_plugin_mcp_servers_by_requirements("sample@test", &mut servers, Some(&requirements));

    assert_eq!(
        servers
            .iter()
            .map(|(name, server)| (
                name.clone(),
                (server.enabled, server.disabled_reason.clone())
            ))
            .collect::<HashMap<String, (bool, Option<McpServerDisabledReason>)>>(),
        HashMap::from([(
            "server-a".to_string(),
            (
                false,
                Some(McpServerDisabledReason::Requirements { source })
            )
        )])
    );
}

#[tokio::test]
async fn rebuild_preserving_session_layers_refreshes_requirements() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let user_file = AbsolutePathBuf::resolve_path_against_base(CONFIG_TOML_FILE, codex_home.path());
    let project_dot_codex =
        AbsolutePathBuf::resolve_path_against_base("project/.codex", codex_home.path());
    let mcp_requirements = BTreeMap::from([
        (
            "session_overrides_user".to_string(),
            McpServerRequirement {
                identity: McpServerIdentity::Command {
                    command: "session-command".to_string(),
                },
            },
        ),
        (
            "managed_overrides_session".to_string(),
            McpServerRequirement {
                identity: McpServerIdentity::Command {
                    command: "managed-command".to_string(),
                },
            },
        ),
        (
            "fresh_global".to_string(),
            McpServerRequirement {
                identity: McpServerIdentity::Command {
                    command: "fresh-global-command".to_string(),
                },
            },
        ),
        (
            "fresh_project".to_string(),
            McpServerRequirement {
                identity: McpServerIdentity::Command {
                    command: "fresh-project-command".to_string(),
                },
            },
        ),
    ]);
    let requirements_toml = codex_config::ConfigRequirementsToml {
        mcp_servers: Some(mcp_requirements.clone()),
        ..Default::default()
    };
    let requirements = codex_config::ConfigRequirements {
        mcp_servers: Some(Sourced::new(mcp_requirements, RequirementSource::Unknown)),
        ..Default::default()
    };
    let refreshed_layer_stack = ConfigLayerStack::new(
        vec![
            ConfigLayerEntry::new(
                codex_app_server_protocol::ConfigLayerSource::User {
                    file: user_file.clone(),
                    profile: None,
                },
                toml::toml! {
                    [mcp_servers.session_overrides_user]
                    command = "new-user-command"
                    [mcp_servers.managed_overrides_session]
                    command = "new-user-command"
                    [mcp_servers.fresh_global]
                    command = "fresh-global-command"
                }
                .into(),
            ),
            ConfigLayerEntry::new(
                codex_app_server_protocol::ConfigLayerSource::Project {
                    dot_codex_folder: project_dot_codex.clone(),
                },
                toml::toml! {
                    [mcp_servers.fresh_project]
                    command = "fresh-project-command"
                }
                .into(),
            ),
            ConfigLayerEntry::new(
                codex_app_server_protocol::ConfigLayerSource::LegacyManagedConfigTomlFromMdm,
                toml::toml! {
                    [mcp_servers.managed_overrides_session]
                    command = "managed-command"
                }
                .into(),
            ),
        ],
        requirements,
        requirements_toml,
    )
    .map_err(std::io::Error::other)?;
    let refreshed_toml = refreshed_layer_stack
        .effective_config()
        .try_into()
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))?;
    let refreshed_config = Config::load_config_with_layer_stack(
        LOCAL_FS.as_ref(),
        refreshed_toml,
        ConfigOverrides {
            cwd: Some(codex_home.path().to_path_buf()),
            ..Default::default()
        },
        codex_home.abs(),
        refreshed_layer_stack,
    )
    .await?;
    let thread_layer_stack = ConfigLayerStack::new(
        vec![
            ConfigLayerEntry::new(
                codex_app_server_protocol::ConfigLayerSource::User {
                    file: user_file.clone(),
                    profile: None,
                },
                toml::toml! {
                    [mcp_servers.session_overrides_user]
                    command = "old-user-command"
                    [mcp_servers.managed_overrides_session]
                    command = "old-user-command"
                    [mcp_servers.fresh_global]
                    command = "old-global-command"
                }
                .into(),
            ),
            ConfigLayerEntry::new(
                codex_app_server_protocol::ConfigLayerSource::Project {
                    dot_codex_folder: project_dot_codex,
                },
                toml::toml! {
                    [mcp_servers.fresh_project]
                    command = "old-project-command"
                }
                .into(),
            ),
            ConfigLayerEntry::new(
                codex_app_server_protocol::ConfigLayerSource::SessionFlags,
                toml::toml! {
                    [mcp_servers.session_overrides_user]
                    command = "session-command"
                    [mcp_servers.managed_overrides_session]
                    command = "session-command"
                    [mcp_servers.blocked_session]
                    command = "blocked-session-command"
                }
                .into(),
            ),
            ConfigLayerEntry::new(
                codex_app_server_protocol::ConfigLayerSource::LegacyManagedConfigTomlFromMdm,
                toml::toml! {
                    [mcp_servers.managed_overrides_session]
                    command = "old-managed-command"
                }
                .into(),
            ),
        ],
        Default::default(),
        Default::default(),
    )
    .map_err(std::io::Error::other)?;
    let thread_toml = thread_layer_stack
        .effective_config()
        .try_into()
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))?;
    let thread_config = Config::load_config_with_layer_stack(
        LOCAL_FS.as_ref(),
        thread_toml,
        ConfigOverrides {
            cwd: Some(codex_home.path().to_path_buf()),
            ..Default::default()
        },
        codex_home.abs(),
        thread_layer_stack,
    )
    .await?;
    let config = thread_config
        .rebuild_preserving_session_layers(&refreshed_config)
        .await?;

    assert_eq!(
        config.mcp_servers.get(),
        &HashMap::from([
            (
                "session_overrides_user".to_string(),
                stdio_mcp("session-command"),
            ),
            (
                "managed_overrides_session".to_string(),
                stdio_mcp("managed-command"),
            ),
            (
                "fresh_global".to_string(),
                stdio_mcp("fresh-global-command"),
            ),
            (
                "fresh_project".to_string(),
                stdio_mcp("fresh-project-command"),
            ),
            (
                "blocked_session".to_string(),
                McpServerConfig {
                    enabled: false,
                    disabled_reason: Some(McpServerDisabledReason::Requirements {
                        source: RequirementSource::Unknown,
                    }),
                    ..stdio_mcp("blocked-session-command")
                },
            ),
        ])
    );

    Ok(())
}

#[tokio::test]
async fn rebuild_preserving_session_layers_refreshes_plugin_derived_mcp_config()
-> anyhow::Result<()> {
    let codex_home = TempDir::new()?;
    let plugin_root = codex_home
        .path()
        .join("plugins/cache")
        .join("test/sample/local");
    std::fs::create_dir_all(plugin_root.join(".codex-plugin"))?;
    std::fs::write(
        plugin_root.join(".codex-plugin/plugin.json"),
        r#"{"name":"sample"}"#,
    )?;
    std::fs::write(
        plugin_root.join(".mcp.json"),
        r#"{
  "mcpServers": {
    "sample": {
      "type": "http",
      "url": "https://sample.example/mcp"
    }
  }
}"#,
    )?;

    let user_file = AbsolutePathBuf::resolve_path_against_base(CONFIG_TOML_FILE, codex_home.path());
    let refreshed_layer_stack = ConfigLayerStack::new(
        vec![ConfigLayerEntry::new(
            codex_app_server_protocol::ConfigLayerSource::User {
                file: user_file.clone(),
                profile: None,
            },
            toml::toml! {
                [features]
                plugins = true

                [plugins."sample@test"]
                enabled = true
            }
            .into(),
        )],
        Default::default(),
        Default::default(),
    )?;
    let refreshed_config = Config::load_config_with_layer_stack(
        LOCAL_FS.as_ref(),
        refreshed_layer_stack.effective_config().try_into()?,
        ConfigOverrides {
            cwd: Some(codex_home.path().to_path_buf()),
            ..Default::default()
        },
        codex_home.abs(),
        refreshed_layer_stack,
    )
    .await?;
    let thread_layer_stack = ConfigLayerStack::new(
        vec![ConfigLayerEntry::new(
            codex_app_server_protocol::ConfigLayerSource::User {
                file: user_file,
                profile: None,
            },
            toml::toml! {
                [features]
                plugins = false

                [plugins."sample@test"]
                enabled = true
            }
            .into(),
        )],
        Default::default(),
        Default::default(),
    )?;
    let thread_config = Config::load_config_with_layer_stack(
        LOCAL_FS.as_ref(),
        thread_layer_stack.effective_config().try_into()?,
        ConfigOverrides {
            cwd: Some(codex_home.path().to_path_buf()),
            ..Default::default()
        },
        codex_home.abs(),
        thread_layer_stack,
    )
    .await?;
    let config = thread_config
        .rebuild_preserving_session_layers(&refreshed_config)
        .await?;
    let plugins_manager = PluginsManager::new(codex_home.path().to_path_buf());
    let mcp_config = config.to_mcp_config(&plugins_manager).await;

    assert_eq!(
        mcp_config.configured_mcp_servers.get("sample"),
        Some(&http_mcp("https://sample.example/mcp"))
    );

    Ok(())
}

#[tokio::test]
async fn to_mcp_config_applies_plugin_mcp_cloud_requirements() -> anyhow::Result<()> {
    let codex_home = TempDir::new()?;
    let plugin_root = codex_home
        .path()
        .join("plugins/cache")
        .join("test/sample/local");
    std::fs::create_dir_all(plugin_root.join(".codex-plugin"))?;
    std::fs::write(
        plugin_root.join(".codex-plugin/plugin.json"),
        r#"{"name":"sample"}"#,
    )?;
    std::fs::write(
        plugin_root.join(".mcp.json"),
        r#"{
  "mcpServers": {
    "sample": {
      "type": "http",
      "url": "https://sample.example/mcp"
    },
    "unlisted": {
      "type": "http",
      "url": "https://unlisted.example/mcp"
    }
  }
}"#,
    )?;
    std::fs::write(
        codex_home.path().join(CONFIG_TOML_FILE),
        r#"
[features]
plugins = true

[plugins."sample@test"]
enabled = true
"#,
    )?;

    let requirements = codex_config::ConfigRequirementsToml {
        plugins: Some(BTreeMap::from([(
            "sample@test".to_string(),
            codex_config::PluginRequirementsToml {
                mcp_servers: Some(BTreeMap::from([(
                    "sample".to_string(),
                    McpServerRequirement {
                        identity: McpServerIdentity::Url {
                            url: "https://sample.example/mcp".to_string(),
                        },
                    },
                )])),
            },
        )])),
        ..Default::default()
    };
    let config = ConfigBuilder::default()
        .codex_home(codex_home.path().to_path_buf())
        .cloud_requirements(CloudRequirementsLoader::new(async move {
            Ok(Some(requirements))
        }))
        .build()
        .await?;
    let plugins_manager = PluginsManager::new(codex_home.path().to_path_buf());
    let mcp_config = config.to_mcp_config(&plugins_manager).await;

    assert_eq!(
        mcp_config
            .configured_mcp_servers
            .get("sample")
            .map(|server| (server.enabled, server.disabled_reason.clone())),
        Some((true, None))
    );
    assert_eq!(
        mcp_config
            .configured_mcp_servers
            .get("unlisted")
            .map(|server| (server.enabled, server.disabled_reason.clone())),
        Some((
            false,
            Some(McpServerDisabledReason::Requirements {
                source: RequirementSource::CloudRequirements,
            })
        ))
    );
    Ok(())
}

#[tokio::test]
async fn to_mcp_config_empty_mcp_requirements_disable_plugin_mcps() -> anyhow::Result<()> {
    let codex_home = TempDir::new()?;
    let plugin_root = codex_home
        .path()
        .join("plugins/cache")
        .join("test/sample/local");
    std::fs::create_dir_all(plugin_root.join(".codex-plugin"))?;
    std::fs::write(
        plugin_root.join(".codex-plugin/plugin.json"),
        r#"{"name":"sample"}"#,
    )?;
    std::fs::write(
        plugin_root.join(".mcp.json"),
        r#"{
  "mcpServers": {
    "sample": {
      "type": "http",
      "url": "https://sample.example/mcp"
    }
  }
}"#,
    )?;
    std::fs::write(
        codex_home.path().join(CONFIG_TOML_FILE),
        r#"
[features]
plugins = true

[plugins."sample@test"]
enabled = true
"#,
    )?;

    let requirements = codex_config::ConfigRequirementsToml {
        mcp_servers: Some(BTreeMap::new()),
        ..Default::default()
    };
    let config = ConfigBuilder::default()
        .codex_home(codex_home.path().to_path_buf())
        .cloud_requirements(CloudRequirementsLoader::new(async move {
            Ok(Some(requirements))
        }))
        .build()
        .await?;
    let plugins_manager = PluginsManager::new(codex_home.path().to_path_buf());
    let mcp_config = config.to_mcp_config(&plugins_manager).await;

    assert_eq!(
        mcp_config
            .configured_mcp_servers
            .get("sample")
            .map(|server| (server.enabled, server.disabled_reason.clone())),
        Some((
            false,
            Some(McpServerDisabledReason::Requirements {
                source: RequirementSource::CloudRequirements,
            })
        ))
    );
    Ok(())
}

#[tokio::test]
async fn add_dir_override_extends_workspace_writable_roots() -> std::io::Result<()> {
    let temp_dir = TempDir::new()?;
    let frontend = temp_dir.path().join("frontend");
    let backend = temp_dir.path().join("backend");
    std::fs::create_dir_all(&frontend)?;
    std::fs::create_dir_all(&backend)?;

    let overrides = ConfigOverrides {
        cwd: Some(frontend),
        sandbox_mode: Some(SandboxMode::WorkspaceWrite),
        additional_writable_roots: vec![PathBuf::from("../backend"), backend.clone()],
        ..Default::default()
    };

    let config = Config::load_from_base_config_with_overrides(
        ConfigToml::default(),
        overrides,
        temp_dir.path().abs(),
    )
    .await?;

    let expected_backend = backend.abs();
    if cfg!(target_os = "windows") {
        match &config.legacy_sandbox_policy() {
            SandboxPolicy::ReadOnly { .. } => {}
            other => panic!("expected read-only policy on Windows, got {other:?}"),
        }
    } else {
        match &config.legacy_sandbox_policy() {
            SandboxPolicy::WorkspaceWrite { writable_roots, .. } => {
                assert_eq!(
                    writable_roots
                        .iter()
                        .filter(|root| **root == expected_backend)
                        .count(),
                    1,
                    "expected single writable root entry for {}",
                    expected_backend.display()
                );
            }
            other => panic!("expected workspace-write policy, got {other:?}"),
        }
    }

    Ok(())
}

#[tokio::test]
async fn sqlite_home_defaults_to_codex_home_for_workspace_write() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let config = Config::load_from_base_config_with_overrides(
        ConfigToml::default(),
        ConfigOverrides {
            sandbox_mode: Some(SandboxMode::WorkspaceWrite),
            ..Default::default()
        },
        codex_home.abs(),
    )
    .await?;

    assert_eq!(config.sqlite_home, codex_home.path().to_path_buf());

    Ok(())
}

#[tokio::test]
async fn workspace_write_always_includes_memories_root_once() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let memories_root = codex_home.path().join("memories");
    let config = Config::load_from_base_config_with_overrides(
        ConfigToml {
            sandbox_workspace_write: Some(SandboxWorkspaceWrite {
                writable_roots: vec![memories_root.abs()],
                ..Default::default()
            }),
            ..Default::default()
        },
        ConfigOverrides {
            sandbox_mode: Some(SandboxMode::WorkspaceWrite),
            ..Default::default()
        },
        codex_home.abs(),
    )
    .await?;

    if cfg!(target_os = "windows") {
        match &config.legacy_sandbox_policy() {
            SandboxPolicy::ReadOnly { .. } => {}
            other => panic!("expected read-only policy on Windows, got {other:?}"),
        }
    } else {
        assert!(
            memories_root.is_dir(),
            "expected memories root directory to exist at {}",
            memories_root.display()
        );
        let expected_memories_root = memories_root.abs();
        match &config.legacy_sandbox_policy() {
            SandboxPolicy::WorkspaceWrite { writable_roots, .. } => {
                assert_eq!(
                    writable_roots
                        .iter()
                        .filter(|root| **root == expected_memories_root)
                        .count(),
                    1,
                    "expected single writable root entry for {}",
                    expected_memories_root.display()
                );
            }
            other => panic!("expected workspace-write policy, got {other:?}"),
        }
    }

    Ok(())
}

#[tokio::test]
async fn config_defaults_to_file_cli_auth_store_mode() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let cfg = ConfigToml::default();

    let config = Config::load_from_base_config_with_overrides(
        cfg,
        ConfigOverrides::default(),
        codex_home.abs(),
    )
    .await?;

    assert_eq!(
        config.cli_auth_credentials_store_mode,
        AuthCredentialsStoreMode::File,
    );

    Ok(())
}

#[tokio::test]
async fn config_resolves_explicit_keyring_auth_store_mode() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let cfg = ConfigToml {
        cli_auth_credentials_store: Some(AuthCredentialsStoreMode::Keyring),
        ..Default::default()
    };

    let config = Config::load_from_base_config_with_overrides(
        cfg,
        ConfigOverrides::default(),
        codex_home.abs(),
    )
    .await?;

    assert_eq!(
        config.cli_auth_credentials_store_mode,
        resolve_cli_auth_credentials_store_mode(
            AuthCredentialsStoreMode::Keyring,
            env!("CARGO_PKG_VERSION"),
        ),
    );

    Ok(())
}

#[tokio::test]
async fn config_resolves_default_oauth_store_mode() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let cfg = ConfigToml::default();

    let config = Config::load_from_base_config_with_overrides(
        cfg,
        ConfigOverrides::default(),
        codex_home.abs(),
    )
    .await?;

    assert_eq!(
        config.mcp_oauth_credentials_store_mode,
        resolve_mcp_oauth_credentials_store_mode(
            OAuthCredentialsStoreMode::Auto,
            env!("CARGO_PKG_VERSION"),
        ),
    );

    Ok(())
}

#[test]
fn local_dev_builds_force_file_cli_auth_store_modes() {
    assert_eq!(
        resolve_cli_auth_credentials_store_mode(
            AuthCredentialsStoreMode::Keyring,
            LOCAL_DEV_BUILD_VERSION,
        ),
        AuthCredentialsStoreMode::File,
    );
    assert_eq!(
        resolve_cli_auth_credentials_store_mode(
            AuthCredentialsStoreMode::Auto,
            LOCAL_DEV_BUILD_VERSION,
        ),
        AuthCredentialsStoreMode::File,
    );
    assert_eq!(
        resolve_cli_auth_credentials_store_mode(
            AuthCredentialsStoreMode::Ephemeral,
            LOCAL_DEV_BUILD_VERSION,
        ),
        AuthCredentialsStoreMode::Ephemeral,
    );
    assert_eq!(
        resolve_cli_auth_credentials_store_mode(AuthCredentialsStoreMode::Keyring, "1.2.3"),
        AuthCredentialsStoreMode::Keyring,
    );
}

#[test]
fn local_dev_builds_force_file_mcp_oauth_store_modes() {
    assert_eq!(
        resolve_mcp_oauth_credentials_store_mode(
            OAuthCredentialsStoreMode::Keyring,
            LOCAL_DEV_BUILD_VERSION,
        ),
        OAuthCredentialsStoreMode::File,
    );
    assert_eq!(
        resolve_mcp_oauth_credentials_store_mode(
            OAuthCredentialsStoreMode::Auto,
            LOCAL_DEV_BUILD_VERSION,
        ),
        OAuthCredentialsStoreMode::File,
    );
    assert_eq!(
        resolve_mcp_oauth_credentials_store_mode(OAuthCredentialsStoreMode::Keyring, "1.2.3"),
        OAuthCredentialsStoreMode::Keyring,
    );
}

#[tokio::test]
async fn feedback_enabled_defaults_to_true() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let cfg = ConfigToml {
        feedback: Some(FeedbackConfigToml::default()),
        ..Default::default()
    };

    let config = Config::load_from_base_config_with_overrides(
        cfg,
        ConfigOverrides::default(),
        codex_home.abs(),
    )
    .await?;

    assert_eq!(config.feedback_enabled, true);

    Ok(())
}

#[test]
fn web_search_mode_defaults_to_none_if_unset() {
    let cfg = ConfigToml::default();
    let profile = ConfigProfile::default();
    let features = Features::with_defaults();

    assert_eq!(resolve_web_search_mode(&cfg, &profile, &features), None);
}

#[test]
fn web_search_mode_prefers_profile_over_legacy_flags() {
    let cfg = ConfigToml::default();
    let profile = ConfigProfile {
        web_search: Some(WebSearchMode::Live),
        ..Default::default()
    };
    let mut features = Features::with_defaults();
    features.enable(Feature::WebSearchCached);

    assert_eq!(
        resolve_web_search_mode(&cfg, &profile, &features),
        Some(WebSearchMode::Live)
    );
}

#[test]
fn web_search_mode_disabled_overrides_legacy_request() {
    let cfg = ConfigToml {
        web_search: Some(WebSearchMode::Disabled),
        ..Default::default()
    };
    let profile = ConfigProfile::default();
    let mut features = Features::with_defaults();
    features.enable(Feature::WebSearchRequest);

    assert_eq!(
        resolve_web_search_mode(&cfg, &profile, &features),
        Some(WebSearchMode::Disabled)
    );
}

#[test]
fn web_search_mode_for_turn_uses_preference_for_read_only() {
    let web_search_mode = Constrained::allow_any(WebSearchMode::Cached);
    let permission_profile =
        PermissionProfile::from_legacy_sandbox_policy(&SandboxPolicy::new_read_only_policy());
    let mode = resolve_web_search_mode_for_turn(&web_search_mode, &permission_profile);

    assert_eq!(mode, WebSearchMode::Cached);
}

#[test]
fn web_search_mode_for_turn_prefers_live_for_disabled_permissions() {
    let web_search_mode = Constrained::allow_any(WebSearchMode::Cached);
    let mode = resolve_web_search_mode_for_turn(&web_search_mode, &PermissionProfile::Disabled);

    assert_eq!(mode, WebSearchMode::Live);
}

#[test]
fn web_search_mode_for_turn_respects_disabled_for_disabled_permissions() {
    let web_search_mode = Constrained::allow_any(WebSearchMode::Disabled);
    let mode = resolve_web_search_mode_for_turn(&web_search_mode, &PermissionProfile::Disabled);

    assert_eq!(mode, WebSearchMode::Disabled);
}

#[test]
fn web_search_mode_for_turn_falls_back_when_live_is_disallowed() -> anyhow::Result<()> {
    let allowed = [WebSearchMode::Disabled, WebSearchMode::Cached];
    let web_search_mode = Constrained::new(WebSearchMode::Cached, move |candidate| {
        if allowed.contains(candidate) {
            Ok(())
        } else {
            Err(ConstraintError::InvalidValue {
                field_name: "web_search_mode",
                candidate: format!("{candidate:?}"),
                allowed: format!("{allowed:?}"),
                requirement_source: RequirementSource::Unknown,
            })
        }
    })?;
    let mode = resolve_web_search_mode_for_turn(&web_search_mode, &PermissionProfile::Disabled);

    assert_eq!(mode, WebSearchMode::Cached);
    Ok(())
}

#[tokio::test]
async fn project_profiles_are_ignored() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let workspace = TempDir::new()?;
    let workspace_key = workspace.path().to_string_lossy().replace('\\', "\\\\");
    std::fs::write(
        codex_home.path().join(CONFIG_TOML_FILE),
        format!(
            r#"
profile = "global"

[profiles.global]
model = "gpt-global"

[profiles.project]
model = "gpt-project"

[projects."{workspace_key}"]
trust_level = "trusted"
"#,
        ),
    )?;
    let project_config_dir = workspace.path().join(".codex");
    std::fs::create_dir_all(&project_config_dir)?;
    std::fs::write(
        project_config_dir.join(CONFIG_TOML_FILE),
        r#"
profile = "project"

[profiles.project]
model = "gpt-project-local"
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

    assert_eq!(config.active_profile.as_deref(), Some("global"));
    assert_eq!(config.model.as_deref(), Some("gpt-global"));
    assert!(
        config.startup_warnings.iter().any(|warning| {
            warning.contains("profile")
                && warning.contains("profiles")
                && warning.contains(
                    "If you want these settings to apply, manually set them in your user-level config.toml."
                )
        }),
        "expected warning for ignored project-local profile keys: {:?}",
        config.startup_warnings
    );

    Ok(())
}

#[tokio::test]
async fn profile_sandbox_mode_overrides_base() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let mut profiles = HashMap::new();
    profiles.insert(
        "work".to_string(),
        ConfigProfile {
            sandbox_mode: Some(SandboxMode::DangerFullAccess),
            ..Default::default()
        },
    );
    let cfg = ConfigToml {
        profiles,
        profile: Some("work".to_string()),
        sandbox_mode: Some(SandboxMode::ReadOnly),
        ..Default::default()
    };

    let config = Config::load_from_base_config_with_overrides(
        cfg,
        ConfigOverrides::default(),
        codex_home.abs(),
    )
    .await?;

    assert!(matches!(
        &config.legacy_sandbox_policy(),
        &SandboxPolicy::DangerFullAccess
    ));

    Ok(())
}

#[tokio::test]
async fn cli_override_takes_precedence_over_profile_sandbox_mode() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let mut profiles = HashMap::new();
    profiles.insert(
        "work".to_string(),
        ConfigProfile {
            sandbox_mode: Some(SandboxMode::DangerFullAccess),
            ..Default::default()
        },
    );
    let cfg = ConfigToml {
        profiles,
        profile: Some("work".to_string()),
        ..Default::default()
    };

    let overrides = ConfigOverrides {
        sandbox_mode: Some(SandboxMode::WorkspaceWrite),
        ..Default::default()
    };

    let config =
        Config::load_from_base_config_with_overrides(cfg, overrides, codex_home.abs()).await?;

    if cfg!(target_os = "windows") {
        assert!(matches!(
            &config.legacy_sandbox_policy(),
            SandboxPolicy::ReadOnly { .. }
        ));
    } else {
        assert!(matches!(
            &config.legacy_sandbox_policy(),
            SandboxPolicy::WorkspaceWrite { .. }
        ));
    }

    Ok(())
}

#[tokio::test]
async fn feature_table_overrides_legacy_flags() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let mut entries = BTreeMap::new();
    entries.insert("apply_patch_freeform".to_string(), false);
    let cfg = ConfigToml {
        features: Some(FeaturesToml::from(entries)),
        ..Default::default()
    };

    let config = Config::load_from_base_config_with_overrides(
        cfg,
        ConfigOverrides::default(),
        codex_home.abs(),
    )
    .await?;

    assert!(!config.features.enabled(Feature::ApplyPatchFreeform));

    Ok(())
}

#[tokio::test]
async fn legacy_toggles_map_to_features() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let cfg = ConfigToml {
        experimental_use_unified_exec_tool: Some(true),
        ..Default::default()
    };

    let config = Config::load_from_base_config_with_overrides(
        cfg,
        ConfigOverrides::default(),
        codex_home.abs(),
    )
    .await?;

    assert!(config.features.enabled(Feature::UnifiedExec));

    assert!(config.use_experimental_unified_exec_tool);

    Ok(())
}

#[tokio::test]
async fn responses_websocket_features_do_not_change_wire_api() -> std::io::Result<()> {
    for feature_key in ["responses_websockets", "responses_websockets_v2"] {
        let codex_home = TempDir::new()?;
        let mut entries = BTreeMap::new();
        entries.insert(feature_key.to_string(), true);
        let cfg = ConfigToml {
            features: Some(FeaturesToml::from(entries)),
            ..Default::default()
        };

        let config = Config::load_from_base_config_with_overrides(
            cfg,
            ConfigOverrides::default(),
            codex_home.abs(),
        )
        .await?;

        assert_eq!(config.model_provider.wire_api, WireApi::Responses);
    }

    Ok(())
}

#[tokio::test]
async fn config_honors_explicit_file_oauth_store_mode() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let cfg = ConfigToml {
        mcp_oauth_credentials_store: Some(OAuthCredentialsStoreMode::File),
        ..Default::default()
    };

    let config = Config::load_from_base_config_with_overrides(
        cfg,
        ConfigOverrides::default(),
        codex_home.abs(),
    )
    .await?;

    assert_eq!(
        config.mcp_oauth_credentials_store_mode,
        OAuthCredentialsStoreMode::File,
    );

    Ok(())
}

#[tokio::test]
async fn managed_config_overrides_oauth_store_mode() -> anyhow::Result<()> {
    let codex_home = TempDir::new()?;
    let managed_path = codex_home.path().join("managed_config.toml");
    let config_path = codex_home.path().join(CONFIG_TOML_FILE);

    std::fs::write(&config_path, "mcp_oauth_credentials_store = \"file\"\n")?;
    std::fs::write(&managed_path, "mcp_oauth_credentials_store = \"keyring\"\n")?;

    let overrides = LoaderOverrides::with_managed_config_path_for_tests(managed_path.clone());

    let cwd = codex_home.path().abs();
    let config_layer_stack = load_config_layers_state(
        LOCAL_FS.as_ref(),
        codex_home.path(),
        Some(cwd),
        &Vec::new(),
        overrides,
        CloudRequirementsLoader::default(),
        &codex_config::NoopThreadConfigLoader,
    )
    .await?;
    let cfg =
        deserialize_config_toml_with_base(config_layer_stack.effective_config(), codex_home.path())
            .map_err(|e| {
                tracing::error!("Failed to deserialize overridden config: {e}");
                e
            })?;
    assert_eq!(
        cfg.mcp_oauth_credentials_store,
        Some(OAuthCredentialsStoreMode::Keyring),
    );

    let final_config = Config::load_from_base_config_with_overrides(
        cfg,
        ConfigOverrides::default(),
        codex_home.abs(),
    )
    .await?;
    assert_eq!(
        final_config.mcp_oauth_credentials_store_mode,
        resolve_mcp_oauth_credentials_store_mode(
            OAuthCredentialsStoreMode::Keyring,
            env!("CARGO_PKG_VERSION"),
        ),
    );

    Ok(())
}

#[tokio::test]
async fn load_global_mcp_servers_returns_empty_if_missing() -> anyhow::Result<()> {
    let codex_home = TempDir::new()?;

    let servers = load_global_mcp_servers(codex_home.path()).await?;
    assert!(servers.is_empty());

    Ok(())
}

#[tokio::test]
async fn replace_mcp_servers_round_trips_entries() -> anyhow::Result<()> {
    let codex_home = TempDir::new()?;

    let mut servers = BTreeMap::new();
    servers.insert(
        "docs".to_string(),
        McpServerConfig {
            transport: McpServerTransportConfig::Stdio {
                command: "echo".to_string(),
                args: vec!["hello".to_string()],
                env: None,
                env_vars: Vec::new(),
                cwd: None,
            },
            experimental_environment: Some("remote".to_string()),
            enabled: true,
            required: false,
            supports_parallel_tool_calls: false,
            disabled_reason: None,
            startup_timeout_sec: Some(Duration::from_secs(3)),
            tool_timeout_sec: Some(Duration::from_secs(5)),
            default_tools_approval_mode: None,
            enabled_tools: None,
            disabled_tools: None,
            scopes: None,
            oauth: None,
            oauth_resource: None,
            tools: HashMap::new(),
        },
    );

    apply_blocking(
        codex_home.path(),
        /*profile*/ None,
        &[ConfigEdit::ReplaceMcpServers(servers.clone())],
    )?;

    let loaded = load_global_mcp_servers(codex_home.path()).await?;
    assert_eq!(loaded.len(), 1);
    let docs = loaded.get("docs").expect("docs entry");
    match &docs.transport {
        McpServerTransportConfig::Stdio {
            command,
            args,
            env,
            env_vars,
            cwd,
        } => {
            assert_eq!(command, "echo");
            assert_eq!(args, &vec!["hello".to_string()]);
            assert!(env.is_none());
            assert!(env_vars.is_empty());
            assert!(cwd.is_none());
        }
        other => panic!("unexpected transport {other:?}"),
    }
    assert_eq!(docs.startup_timeout_sec, Some(Duration::from_secs(3)));
    assert_eq!(docs.tool_timeout_sec, Some(Duration::from_secs(5)));
    assert_eq!(docs.experimental_environment.as_deref(), Some("remote"));
    assert!(docs.enabled);

    let empty = BTreeMap::new();
    apply_blocking(
        codex_home.path(),
        /*profile*/ None,
        &[ConfigEdit::ReplaceMcpServers(empty.clone())],
    )?;
    let loaded = load_global_mcp_servers(codex_home.path()).await?;
    assert!(loaded.is_empty());

    Ok(())
}

#[tokio::test]
async fn managed_config_wins_over_cli_overrides() -> anyhow::Result<()> {
    let codex_home = TempDir::new()?;
    let managed_path = codex_home.path().join("managed_config.toml");

    std::fs::write(
        codex_home.path().join(CONFIG_TOML_FILE),
        "model = \"base\"\n",
    )?;
    std::fs::write(&managed_path, "model = \"managed_config\"\n")?;

    let overrides = LoaderOverrides::with_managed_config_path_for_tests(managed_path);

    let cwd = codex_home.path().abs();
    let config_layer_stack = load_config_layers_state(
        LOCAL_FS.as_ref(),
        codex_home.path(),
        Some(cwd),
        &[("model".to_string(), TomlValue::String("cli".to_string()))],
        overrides,
        CloudRequirementsLoader::default(),
        &codex_config::NoopThreadConfigLoader,
    )
    .await?;

    let cfg =
        deserialize_config_toml_with_base(config_layer_stack.effective_config(), codex_home.path())
            .map_err(|e| {
                tracing::error!("Failed to deserialize overridden config: {e}");
                e
            })?;

    assert_eq!(cfg.model.as_deref(), Some("managed_config"));
    Ok(())
}

#[tokio::test]
async fn load_global_mcp_servers_accepts_legacy_ms_field() -> anyhow::Result<()> {
    let codex_home = TempDir::new()?;
    let config_path = codex_home.path().join(CONFIG_TOML_FILE);

    std::fs::write(
        &config_path,
        r#"
[mcp_servers]
[mcp_servers.docs]
command = "echo"
startup_timeout_ms = 2500
"#,
    )?;

    let servers = load_global_mcp_servers(codex_home.path()).await?;
    let docs = servers.get("docs").expect("docs entry");
    assert_eq!(docs.startup_timeout_sec, Some(Duration::from_millis(2500)));

    Ok(())
}

#[test]
fn mcp_servers_toml_parses_per_tool_approval_overrides() {
    let config = toml::from_str::<ConfigToml>(
        r#"
[mcp_servers.docs]
command = "docs-server"
name = "Docs"
default_tools_approval_mode = "prompt"

[mcp_servers.docs.tools.search]
approval_mode = "approve"
"#,
    )
    .expect("TOML deserialization should succeed");
    let server = config
        .mcp_servers
        .get("docs")
        .expect("docs server config exists");

    assert_eq!(
        server.default_tools_approval_mode,
        Some(AppToolApproval::Prompt)
    );

    assert_eq!(
        server.tools.get("search"),
        Some(&McpServerToolConfig {
            approval_mode: Some(AppToolApproval::Approve),
        })
    );
}

#[test]
fn mcp_servers_toml_ignores_unknown_server_fields() {
    let config = toml::from_str::<ConfigToml>(
        r#"
[mcp_servers.docs]
command = "docs-server"
trust_level = "trusted"
"#,
    )
    .expect("unknown MCP server fields should be ignored");

    assert_eq!(
        config.mcp_servers.get("docs"),
        Some(&stdio_mcp("docs-server"))
    );
}

#[test]
fn mcp_servers_toml_parses_tool_approval_override_for_reserved_name() {
    let config = toml::from_str::<ConfigToml>(
        r#"
[mcp_servers.docs]
command = "docs-server"

[mcp_servers.docs.tools.command]
approval_mode = "approve"
"#,
    )
    .expect("TOML deserialization should succeed");
    let tool = config
        .mcp_servers
        .get("docs")
        .and_then(|server| server.tools.get("command"))
        .expect("docs/command tool config exists");

    assert_eq!(
        tool,
        &McpServerToolConfig {
            approval_mode: Some(AppToolApproval::Approve),
        }
    );
}

#[test]
fn desktop_toml_round_trips_opaque_nested_values() -> anyhow::Result<()> {
    let parsed = toml::from_str::<ConfigToml>(
        r#"
[desktop]
appearanceTheme = "dark"
selected-avatar-id = "codex"
recentViews = ["threads", "settings"]

[desktop.workspace]
collapsed = true
width = 320
pane = { selected = "console", expanded = false }
"#,
    )?;

    let desktop = parsed
        .desktop
        .as_ref()
        .expect("desktop settings should deserialize");
    assert_eq!(
        desktop.get("appearanceTheme"),
        Some(&serde_json::json!("dark"))
    );
    assert_eq!(
        desktop.get("selected-avatar-id"),
        Some(&serde_json::json!("codex"))
    );
    assert_eq!(
        desktop.get("recentViews"),
        Some(&serde_json::json!(["threads", "settings"]))
    );
    assert_eq!(
        desktop.get("workspace"),
        Some(&serde_json::json!({
            "collapsed": true,
            "width": 320,
            "pane": {
                "selected": "console",
                "expanded": false,
            },
        }))
    );

    let serialized = toml::to_string(&parsed)?;
    let reparsed = toml::from_str::<ConfigToml>(&serialized)?;
    assert_eq!(reparsed.desktop, parsed.desktop);

    Ok(())
}

#[tokio::test]
async fn to_mcp_config_preserves_apps_feature_from_config() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let mut config = Config::load_from_base_config_with_overrides(
        ConfigToml::default(),
        ConfigOverrides::default(),
        codex_home.abs(),
    )
    .await?;
    let plugins_manager = PluginsManager::new(codex_home.path().to_path_buf());

    config.apps_mcp_path_override = Some("/custom/mcp".to_string());
    config.apps_mcp_product_sku = Some("tpp".to_string());
    let mcp_config = config.to_mcp_config(&plugins_manager).await;
    assert!(mcp_config.apps_enabled);
    assert_eq!(
        mcp_config.apps_mcp_path_override.as_deref(),
        Some("/custom/mcp")
    );
    assert_eq!(mcp_config.apps_mcp_product_sku.as_deref(), Some("tpp"));

    let _ = config.features.disable(Feature::Apps);
    let mcp_config = config.to_mcp_config(&plugins_manager).await;
    assert!(!mcp_config.apps_enabled);

    let _ = config.features.enable(Feature::Apps);
    let mcp_config = config.to_mcp_config(&plugins_manager).await;
    assert!(mcp_config.apps_enabled);

    Ok(())
}

#[tokio::test]
async fn to_mcp_config_preserves_auth_elicitation_feature_from_config() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let mut config = Config::load_from_base_config_with_overrides(
        ConfigToml::default(),
        ConfigOverrides::default(),
        codex_home.abs(),
    )
    .await?;
    let plugins_manager = PluginsManager::new(codex_home.path().to_path_buf());

    let mcp_config = config.to_mcp_config(&plugins_manager).await;
    assert_eq!(
        mcp_config.client_elicitation_capability,
        ElicitationCapability::default()
    );

    let _ = config.features.enable(Feature::AuthElicitation);
    let mcp_config = config.to_mcp_config(&plugins_manager).await;
    assert_eq!(
        mcp_config.client_elicitation_capability,
        ElicitationCapability {
            form: Some(FormElicitationCapability::default()),
            url: Some(UrlElicitationCapability::default()),
        }
    );

    Ok(())
}

#[tokio::test]
async fn load_global_mcp_servers_rejects_inline_bearer_token() -> anyhow::Result<()> {
    let codex_home = TempDir::new()?;
    let config_path = codex_home.path().join(CONFIG_TOML_FILE);

    std::fs::write(
        &config_path,
        r#"
[mcp_servers.docs]
url = "https://example.com/mcp"
bearer_token = "secret"
"#,
    )?;

    let err = load_global_mcp_servers(codex_home.path())
        .await
        .expect_err("bearer_token entries should be rejected");

    assert_eq!(err.kind(), std::io::ErrorKind::InvalidData);
    assert!(err.to_string().contains("bearer_token"));
    assert!(err.to_string().contains("bearer_token_env_var"));

    Ok(())
}

#[tokio::test]
async fn replace_mcp_servers_serializes_env_sorted() -> anyhow::Result<()> {
    let codex_home = TempDir::new()?;

    let servers = BTreeMap::from([(
        "docs".to_string(),
        McpServerConfig {
            transport: McpServerTransportConfig::Stdio {
                command: "docs-server".to_string(),
                args: vec!["--verbose".to_string()],
                env: Some(HashMap::from([
                    ("ZIG_VAR".to_string(), "3".to_string()),
                    ("ALPHA_VAR".to_string(), "1".to_string()),
                ])),
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
    assert_eq!(
        serialized,
        r#"[mcp_servers.docs]
command = "docs-server"
args = ["--verbose"]

[mcp_servers.docs.env]
ALPHA_VAR = "1"
ZIG_VAR = "3"
"#
    );

    let loaded = load_global_mcp_servers(codex_home.path()).await?;
    let docs = loaded.get("docs").expect("docs entry");
    match &docs.transport {
        McpServerTransportConfig::Stdio {
            command,
            args,
            env,
            env_vars,
            cwd,
        } => {
            assert_eq!(command, "docs-server");
            assert_eq!(args, &vec!["--verbose".to_string()]);
            let env = env
                .as_ref()
                .expect("env should be preserved for stdio transport");
            assert_eq!(env.get("ALPHA_VAR"), Some(&"1".to_string()));
            assert_eq!(env.get("ZIG_VAR"), Some(&"3".to_string()));
            assert!(env_vars.is_empty());
            assert!(cwd.is_none());
        }
        other => panic!("unexpected transport {other:?}"),
    }

    Ok(())
}

#[tokio::test]
async fn replace_mcp_servers_serializes_env_vars() -> anyhow::Result<()> {
    let codex_home = TempDir::new()?;

    let servers = BTreeMap::from([(
        "docs".to_string(),
        McpServerConfig {
            transport: McpServerTransportConfig::Stdio {
                command: "docs-server".to_string(),
                args: Vec::new(),
                env: None,
                env_vars: vec!["ALPHA".into(), "BETA".into()],
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
        serialized.contains(r#"env_vars = ["ALPHA", "BETA"]"#),
        "serialized config missing env_vars field:\n{serialized}"
    );

    let loaded = load_global_mcp_servers(codex_home.path()).await?;
    let docs = loaded.get("docs").expect("docs entry");
    match &docs.transport {
        McpServerTransportConfig::Stdio { env_vars, .. } => {
            assert_eq!(env_vars, &vec!["ALPHA".into(), "BETA".into()]);
        }
        other => panic!("unexpected transport {other:?}"),
    }

    Ok(())
}

#[tokio::test]
async fn replace_mcp_servers_serializes_sourced_env_vars() -> anyhow::Result<()> {
    let codex_home = TempDir::new()?;

    let servers = BTreeMap::from([(
        "docs".to_string(),
        McpServerConfig {
            transport: McpServerTransportConfig::Stdio {
                command: "docs-server".to_string(),
                args: Vec::new(),
                env: None,
                env_vars: vec![
                    "LEGACY".into(),
                    McpServerEnvVar::Config {
                        name: "REMOTE_TOKEN".to_string(),
                        source: Some("remote".to_string()),
                    },
                ],
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
        serialized
            .contains(r#"env_vars = ["LEGACY", { name = "REMOTE_TOKEN", source = "remote" }]"#),
        "serialized config missing sourced env_vars field:\n{serialized}"
    );

    let loaded = load_global_mcp_servers(codex_home.path()).await?;
    assert_eq!(loaded, servers);

    Ok(())
}

#[tokio::test]
async fn replace_mcp_servers_serializes_cwd() -> anyhow::Result<()> {
    let codex_home = TempDir::new()?;

    let cwd_path = PathBuf::from("/tmp/codex-mcp");
    let servers = BTreeMap::from([(
        "docs".to_string(),
        McpServerConfig {
            transport: McpServerTransportConfig::Stdio {
                command: "docs-server".to_string(),
                args: Vec::new(),
                env: None,
                env_vars: Vec::new(),
                cwd: Some(cwd_path.clone()),
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
        serialized.contains(r#"cwd = "/tmp/codex-mcp""#),
        "serialized config missing cwd field:\n{serialized}"
    );

    let loaded = load_global_mcp_servers(codex_home.path()).await?;
    let docs = loaded.get("docs").expect("docs entry");
    match &docs.transport {
        McpServerTransportConfig::Stdio { cwd, .. } => {
            assert_eq!(cwd.as_deref(), Some(Path::new("/tmp/codex-mcp")));
        }
        other => panic!("unexpected transport {other:?}"),
    }

    Ok(())
}

#[tokio::test]
async fn replace_mcp_servers_streamable_http_serializes_bearer_token() -> anyhow::Result<()> {
    let codex_home = TempDir::new()?;

    let servers = BTreeMap::from([(
        "docs".to_string(),
        McpServerConfig {
            transport: McpServerTransportConfig::StreamableHttp {
                url: "https://example.com/mcp".to_string(),
                bearer_token_env_var: Some("MCP_TOKEN".to_string()),
                http_headers: None,
                env_http_headers: None,
            },
            experimental_environment: None,
            enabled: true,
            required: false,
            supports_parallel_tool_calls: false,
            disabled_reason: None,
            startup_timeout_sec: Some(Duration::from_secs(2)),
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
    assert_eq!(
        serialized,
        r#"[mcp_servers.docs]
url = "https://example.com/mcp"
bearer_token_env_var = "MCP_TOKEN"
startup_timeout_sec = 2.0
"#
    );

    let loaded = load_global_mcp_servers(codex_home.path()).await?;
    let docs = loaded.get("docs").expect("docs entry");
    match &docs.transport {
        McpServerTransportConfig::StreamableHttp {
            url,
            bearer_token_env_var,
            http_headers,
            env_http_headers,
        } => {
            assert_eq!(url, "https://example.com/mcp");
            assert_eq!(bearer_token_env_var.as_deref(), Some("MCP_TOKEN"));
            assert!(http_headers.is_none());
            assert!(env_http_headers.is_none());
        }
        other => panic!("unexpected transport {other:?}"),
    }
    assert_eq!(docs.startup_timeout_sec, Some(Duration::from_secs(2)));

    Ok(())
}

#[tokio::test]
async fn replace_mcp_servers_streamable_http_serializes_custom_headers() -> anyhow::Result<()> {
    let codex_home = TempDir::new()?;

    let servers = BTreeMap::from([(
        "docs".to_string(),
        McpServerConfig {
            transport: McpServerTransportConfig::StreamableHttp {
                url: "https://example.com/mcp".to_string(),
                bearer_token_env_var: Some("MCP_TOKEN".to_string()),
                http_headers: Some(HashMap::from([("X-Doc".to_string(), "42".to_string())])),
                env_http_headers: Some(HashMap::from([(
                    "X-Auth".to_string(),
                    "DOCS_AUTH".to_string(),
                )])),
            },
            experimental_environment: None,
            enabled: true,
            required: false,
            supports_parallel_tool_calls: false,
            disabled_reason: None,
            startup_timeout_sec: Some(Duration::from_secs(2)),
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
    assert_eq!(
        serialized,
        r#"[mcp_servers.docs]
url = "https://example.com/mcp"
bearer_token_env_var = "MCP_TOKEN"
startup_timeout_sec = 2.0

[mcp_servers.docs.http_headers]
X-Doc = "42"

[mcp_servers.docs.env_http_headers]
X-Auth = "DOCS_AUTH"
"#
    );

    let loaded = load_global_mcp_servers(codex_home.path()).await?;
    let docs = loaded.get("docs").expect("docs entry");
    match &docs.transport {
        McpServerTransportConfig::StreamableHttp {
            http_headers,
            env_http_headers,
            ..
        } => {
            assert_eq!(
                http_headers,
                &Some(HashMap::from([("X-Doc".to_string(), "42".to_string())]))
            );
            assert_eq!(
                env_http_headers,
                &Some(HashMap::from([(
                    "X-Auth".to_string(),
                    "DOCS_AUTH".to_string()
                )]))
            );
        }
        other => panic!("unexpected transport {other:?}"),
    }

    Ok(())
}

#[tokio::test]
async fn replace_mcp_servers_streamable_http_removes_optional_sections() -> anyhow::Result<()> {
    let codex_home = TempDir::new()?;

    let config_path = codex_home.path().join(CONFIG_TOML_FILE);

    let mut servers = BTreeMap::from([(
        "docs".to_string(),
        McpServerConfig {
            transport: McpServerTransportConfig::StreamableHttp {
                url: "https://example.com/mcp".to_string(),
                bearer_token_env_var: Some("MCP_TOKEN".to_string()),
                http_headers: Some(HashMap::from([("X-Doc".to_string(), "42".to_string())])),
                env_http_headers: Some(HashMap::from([(
                    "X-Auth".to_string(),
                    "DOCS_AUTH".to_string(),
                )])),
            },
            experimental_environment: None,
            enabled: true,
            required: false,
            supports_parallel_tool_calls: false,
            disabled_reason: None,
            startup_timeout_sec: Some(Duration::from_secs(2)),
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
    let serialized_with_optional = std::fs::read_to_string(&config_path)?;
    assert!(serialized_with_optional.contains("bearer_token_env_var = \"MCP_TOKEN\""));
    assert!(serialized_with_optional.contains("[mcp_servers.docs.http_headers]"));
    assert!(serialized_with_optional.contains("[mcp_servers.docs.env_http_headers]"));

    servers.insert(
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
            oauth: None,
            oauth_resource: None,
            tools: HashMap::new(),
        },
    );
    apply_blocking(
        codex_home.path(),
        /*profile*/ None,
        &[ConfigEdit::ReplaceMcpServers(servers.clone())],
    )?;

    let serialized = std::fs::read_to_string(&config_path)?;
    assert_eq!(
        serialized,
        r#"[mcp_servers.docs]
url = "https://example.com/mcp"
"#
    );

    let loaded = load_global_mcp_servers(codex_home.path()).await?;
    let docs = loaded.get("docs").expect("docs entry");
    match &docs.transport {
        McpServerTransportConfig::StreamableHttp {
            url,
            bearer_token_env_var,
            http_headers,
            env_http_headers,
        } => {
            assert_eq!(url, "https://example.com/mcp");
            assert!(bearer_token_env_var.is_none());
            assert!(http_headers.is_none());
            assert!(env_http_headers.is_none());
        }
        other => panic!("unexpected transport {other:?}"),
    }

    assert!(docs.startup_timeout_sec.is_none());

    Ok(())
}

#[tokio::test]
async fn replace_mcp_servers_streamable_http_isolates_headers_between_servers() -> anyhow::Result<()>
{
    let codex_home = TempDir::new()?;
    let config_path = codex_home.path().join(CONFIG_TOML_FILE);

    let servers = BTreeMap::from([
        (
            "docs".to_string(),
            McpServerConfig {
                transport: McpServerTransportConfig::StreamableHttp {
                    url: "https://example.com/mcp".to_string(),
                    bearer_token_env_var: Some("MCP_TOKEN".to_string()),
                    http_headers: Some(HashMap::from([("X-Doc".to_string(), "42".to_string())])),
                    env_http_headers: Some(HashMap::from([(
                        "X-Auth".to_string(),
                        "DOCS_AUTH".to_string(),
                    )])),
                },
                experimental_environment: None,
                enabled: true,
                required: false,
                supports_parallel_tool_calls: false,
                disabled_reason: None,
                startup_timeout_sec: Some(Duration::from_secs(2)),
                tool_timeout_sec: None,
                default_tools_approval_mode: None,
                enabled_tools: None,
                disabled_tools: None,
                scopes: None,
                oauth: None,
                oauth_resource: None,
                tools: HashMap::new(),
            },
        ),
        (
            "logs".to_string(),
            McpServerConfig {
                transport: McpServerTransportConfig::Stdio {
                    command: "logs-server".to_string(),
                    args: vec!["--follow".to_string()],
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
                enabled_tools: None,
                disabled_tools: None,
                scopes: None,
                oauth: None,
                oauth_resource: None,
                tools: HashMap::new(),
            },
        ),
    ]);

    apply_blocking(
        codex_home.path(),
        /*profile*/ None,
        &[ConfigEdit::ReplaceMcpServers(servers.clone())],
    )?;

    let serialized = std::fs::read_to_string(&config_path)?;
    assert!(
        serialized.contains("[mcp_servers.docs.http_headers]"),
        "serialized config missing docs headers section:\n{serialized}"
    );
    assert!(
        !serialized.contains("[mcp_servers.logs.http_headers]"),
        "serialized config should not add logs headers section:\n{serialized}"
    );
    assert!(
        !serialized.contains("[mcp_servers.logs.env_http_headers]"),
        "serialized config should not add logs env headers section:\n{serialized}"
    );
    assert!(
        !serialized.contains("mcp_servers.logs.bearer_token_env_var"),
        "serialized config should not add bearer token to logs:\n{serialized}"
    );

    let loaded = load_global_mcp_servers(codex_home.path()).await?;
    let docs = loaded.get("docs").expect("docs entry");
    match &docs.transport {
        McpServerTransportConfig::StreamableHttp {
            http_headers,
            env_http_headers,
            ..
        } => {
            assert_eq!(
                http_headers,
                &Some(HashMap::from([("X-Doc".to_string(), "42".to_string())]))
            );
            assert_eq!(
                env_http_headers,
                &Some(HashMap::from([(
                    "X-Auth".to_string(),
                    "DOCS_AUTH".to_string()
                )]))
            );
        }
        other => panic!("unexpected transport {other:?}"),
    }
    let logs = loaded.get("logs").expect("logs entry");
    match &logs.transport {
        McpServerTransportConfig::Stdio { env, .. } => {
            assert!(env.is_none());
        }
        other => panic!("unexpected transport {other:?}"),
    }

    Ok(())
}

#[tokio::test]
async fn replace_mcp_servers_serializes_disabled_flag() -> anyhow::Result<()> {
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
            enabled: false,
            required: false,
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
        serialized.contains("enabled = false"),
        "serialized config missing disabled flag:\n{serialized}"
    );

    let loaded = load_global_mcp_servers(codex_home.path()).await?;
    let docs = loaded.get("docs").expect("docs entry");
    assert!(!docs.enabled);

    Ok(())
}
