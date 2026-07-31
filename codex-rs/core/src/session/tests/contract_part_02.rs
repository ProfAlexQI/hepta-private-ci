#[test]
fn get_service_tier_defaults_enterprise_accounts_to_fast() {
    assert_eq!(
        get_service_tier(
            /*configured_service_tier*/ None,
            /*fast_default_opt_out*/ false,
            Some(AccountPlanType::Enterprise),
            /*fast_mode_enabled*/ true,
        ),
        Some(ServiceTier::Fast.request_value().to_string())
    );
    assert_eq!(
        get_service_tier(
            /*configured_service_tier*/ None,
            /*fast_default_opt_out*/ false,
            Some(AccountPlanType::EnterpriseCbpUsageBased),
            /*fast_mode_enabled*/ true,
        ),
        Some(ServiceTier::Fast.request_value().to_string())
    );
    assert_eq!(
        get_service_tier(
            /*configured_service_tier*/ None,
            /*fast_default_opt_out*/ false,
            Some(AccountPlanType::Business),
            /*fast_mode_enabled*/ true,
        ),
        Some(ServiceTier::Fast.request_value().to_string())
    );
    assert_eq!(
        get_service_tier(
            /*configured_service_tier*/ None,
            /*fast_default_opt_out*/ false,
            Some(AccountPlanType::Team),
            /*fast_mode_enabled*/ true,
        ),
        Some(ServiceTier::Fast.request_value().to_string())
    );
    assert_eq!(
        get_service_tier(
            /*configured_service_tier*/ None,
            /*fast_default_opt_out*/ false,
            Some(AccountPlanType::SelfServeBusinessUsageBased),
            /*fast_mode_enabled*/ true,
        ),
        Some(ServiceTier::Fast.request_value().to_string())
    );
}

#[test]
fn get_service_tier_respects_fast_default_opt_out() {
    assert_eq!(
        get_service_tier(
            /*configured_service_tier*/ None,
            /*fast_default_opt_out*/ true,
            Some(AccountPlanType::Enterprise),
            /*fast_mode_enabled*/ true,
        ),
        None
    );
}

#[test]
fn get_service_tier_does_not_default_non_enterprise_or_disabled_fast_mode() {
    assert_eq!(
        get_service_tier(
            /*configured_service_tier*/ None,
            /*fast_default_opt_out*/ false,
            Some(AccountPlanType::Pro),
            /*fast_mode_enabled*/ true,
        ),
        None
    );
    assert_eq!(
        get_service_tier(
            /*configured_service_tier*/ None,
            /*fast_default_opt_out*/ false,
            Some(AccountPlanType::Enterprise),
            /*fast_mode_enabled*/ false,
        ),
        None
    );
}

#[tokio::test]
async fn session_settings_null_service_tier_update_clears_service_tier() {
    let session_configuration = make_session_configuration_for_tests().await;

    let updated = session_configuration
        .apply(&SessionSettingsUpdate {
            service_tier: Some(None),
            ..Default::default()
        })
        .expect("null service tier update should apply");

    assert_eq!(updated.service_tier, None);
}

#[tokio::test]
async fn session_settings_legacy_fast_service_tier_update_uses_priority_request_value() {
    let session_configuration = make_session_configuration_for_tests().await;

    let updated = session_configuration
        .apply(&SessionSettingsUpdate {
            service_tier: Some(Some("fast".to_string())),
            ..Default::default()
        })
        .expect("legacy fast service tier update should apply");

    assert_eq!(
        updated.service_tier,
        Some(ServiceTier::Fast.request_value().to_string())
    );
}

pub(crate) async fn make_session_configuration_for_tests() -> SessionConfiguration {
    let codex_home = tempfile::tempdir().expect("create temp dir");
    let config = build_test_config(codex_home.path()).await;
    let config = Arc::new(config);
    let model = get_model_offline_for_tests(config.model.as_deref());
    let model_info =
        construct_model_info_offline_for_tests(model.as_str(), &config.to_models_manager_config());
    let reasoning_effort = config.model_reasoning_effort;
    let collaboration_mode = CollaborationMode {
        mode: ModeKind::Default,
        settings: Settings {
            model,
            reasoning_effort,
            developer_instructions: None,
        },
    };

    SessionConfiguration {
        provider: config.model_provider.clone(),
        collaboration_mode,
        model_reasoning_summary: config.model_reasoning_summary,
        developer_instructions: config.developer_instructions.clone(),
        user_instructions: config.user_instructions.clone(),
        service_tier: None,
        personality: config.personality,
        history_mode: ThreadHistoryMode::Legacy,
        base_instructions: config
            .base_instructions
            .clone()
            .unwrap_or_else(|| model_info.get_model_instructions(config.personality)),
        compact_prompt: config.compact_prompt.clone(),
        approval_policy: config.permissions.approval_policy.clone(),
        approvals_reviewer: config.approvals_reviewer,
        permission_profile_state: config.permissions.permission_profile_state().clone(),
        windows_sandbox_level: WindowsSandboxLevel::from_config(&config),
        cwd: config.cwd.clone().into(),
        workspace_roots: config.workspace_roots.clone(),
        codex_home: config.codex_home.clone(),
        thread_name: None,
        environments: Vec::new(),
        original_config_do_not_use: Arc::clone(&config),
        metrics_service_name: None,
        app_server_client_name: None,
        app_server_client_version: None,
        session_source: SessionSource::Exec,
        thread_source: None,
        dynamic_tools: Vec::new(),
        persist_extended_history: false,
        inherited_shell_snapshot: None,
        user_shell_override: None,
    }
}

fn turn_environments_for_tests(
    environment: &Arc<codex_exec_server::Environment>,
    cwd: &codex_utils_absolute_path::AbsolutePathBuf,
) -> crate::environment_selection::ResolvedTurnEnvironments {
    crate::environment_selection::ResolvedTurnEnvironments {
        turn_environments: vec![TurnEnvironment {
            environment_id: codex_exec_server::LOCAL_ENVIRONMENT_ID.to_string(),
            environment: Arc::clone(environment),
            cwd: cwd.clone().into(),
            shell: None,
        }],
    }
}

#[tokio::test]
async fn session_configuration_apply_preserves_profile_file_system_policy_on_cwd_only_update() {
    let mut session_configuration = make_session_configuration_for_tests().await;
    let workspace = tempfile::tempdir().expect("create temp dir");
    let project_root = workspace.path().join("project");
    let original_cwd = project_root.join("subdir");
    let docs_dir = original_cwd.join("docs");
    std::fs::create_dir_all(&docs_dir).expect("create docs dir");
    let docs_dir = docs_dir.abs();

    session_configuration.cwd = original_cwd.abs();
    let sandbox_policy = SandboxPolicy::WorkspaceWrite {
        writable_roots: Vec::new(),
        network_access: false,
        exclude_tmpdir_env_var: true,
        exclude_slash_tmp: true,
    };
    let file_system_sandbox_policy = FileSystemSandboxPolicy::restricted(vec![
        FileSystemSandboxEntry {
            path: FileSystemPath::Special {
                value: FileSystemSpecialPath::project_roots(/*subpath*/ None),
            },
            access: FileSystemAccessMode::Write,
        },
        FileSystemSandboxEntry {
            path: FileSystemPath::Path { path: docs_dir },
            access: FileSystemAccessMode::Read,
        },
    ]);
    let network_sandbox_policy = NetworkSandboxPolicy::from(&sandbox_policy);
    session_configuration
        .set_permission_profile_for_tests(
            PermissionProfile::from_runtime_permissions_with_enforcement(
                SandboxEnforcement::from_legacy_sandbox_policy(&sandbox_policy),
                &file_system_sandbox_policy,
                network_sandbox_policy,
            ),
        )
        .expect("set permission profile");
    let expected_file_system_sandbox_policy = file_system_sandbox_policy
        .materialize_project_roots_with_workspace_roots(&session_configuration.workspace_roots);

    let updated = session_configuration
        .apply(&SessionSettingsUpdate {
            cwd: Some(project_root),
            ..Default::default()
        })
        .expect("cwd-only update should succeed");

    assert_eq!(
        updated.file_system_sandbox_policy(),
        expected_file_system_sandbox_policy
    );
}

#[tokio::test]
async fn session_configuration_apply_permission_profile_preserves_existing_deny_read_entries() {
    let mut session_configuration = make_session_configuration_for_tests().await;
    let cwd = tempfile::tempdir().expect("create temp dir");
    session_configuration.cwd = cwd.path().abs();

    let workspace_policy = SandboxPolicy::new_workspace_write_policy();
    let deny_entry = FileSystemSandboxEntry {
        path: FileSystemPath::GlobPattern {
            pattern: "**/*.env".to_string(),
        },
        access: FileSystemAccessMode::None,
    };
    let mut existing_file_system_policy =
        FileSystemSandboxPolicy::from_legacy_sandbox_policy_for_cwd(
            &workspace_policy,
            session_configuration.cwd.as_path(),
        );
    existing_file_system_policy.glob_scan_max_depth = Some(2);
    existing_file_system_policy.entries.push(deny_entry.clone());
    session_configuration
        .set_permission_profile_for_tests(
            PermissionProfile::from_runtime_permissions_with_enforcement(
                SandboxEnforcement::from_legacy_sandbox_policy(&workspace_policy),
                &existing_file_system_policy,
                NetworkSandboxPolicy::Restricted,
            ),
        )
        .expect("set permission profile");

    let requested_file_system_policy = FileSystemSandboxPolicy::from_legacy_sandbox_policy_for_cwd(
        &workspace_policy,
        session_configuration.cwd.as_path(),
    );
    let permission_profile = codex_protocol::models::PermissionProfile::from_runtime_permissions(
        &requested_file_system_policy,
        NetworkSandboxPolicy::Restricted,
    );
    let updated = session_configuration
        .apply(&SessionSettingsUpdate {
            permission_profile: Some(permission_profile),
            ..Default::default()
        })
        .expect("permission profile update should succeed");

    let mut expected_file_system_policy = requested_file_system_policy
        .materialize_project_roots_with_workspace_roots(&session_configuration.workspace_roots);
    expected_file_system_policy.glob_scan_max_depth = Some(2);
    expected_file_system_policy.entries.push(deny_entry);
    assert_eq!(
        updated.file_system_sandbox_policy(),
        expected_file_system_policy
    );
}

#[tokio::test]
async fn session_configuration_apply_permission_profile_accepts_direct_write_roots() {
    let mut session_configuration = make_session_configuration_for_tests().await;
    let cwd = tempfile::tempdir().expect("create cwd");
    session_configuration.cwd = cwd.path().abs();
    let external_write_dir = tempfile::tempdir().expect("create external write root");
    let external_write_path = AbsolutePathBuf::from_absolute_path(
        codex_utils_absolute_path::canonicalize_preserving_symlinks(external_write_dir.path())
            .expect("canonical temp dir"),
    )
    .expect("canonical temp dir should be absolute");
    let file_system_sandbox_policy =
        FileSystemSandboxPolicy::restricted(vec![FileSystemSandboxEntry {
            path: FileSystemPath::Path {
                path: external_write_path.clone(),
            },
            access: FileSystemAccessMode::Write,
        }]);
    let permission_profile = PermissionProfile::from_runtime_permissions(
        &file_system_sandbox_policy,
        NetworkSandboxPolicy::Restricted,
    );

    let updated = session_configuration
        .apply(&SessionSettingsUpdate {
            permission_profile: Some(permission_profile.clone()),
            ..Default::default()
        })
        .expect("permission profile update should accept direct runtime permissions");

    assert_eq!(updated.permission_profile(), permission_profile);
    assert_eq!(
        updated.file_system_sandbox_policy(),
        file_system_sandbox_policy
    );
    assert_eq!(
        updated.sandbox_policy(),
        SandboxPolicy::WorkspaceWrite {
            writable_roots: vec![external_write_path],
            network_access: false,
            exclude_tmpdir_env_var: true,
            exclude_slash_tmp: true,
        }
    );
}

#[tokio::test]
async fn session_configuration_apply_rebinds_symbolic_profile_to_updated_workspace_roots() {
    let mut session_configuration = make_session_configuration_for_tests().await;
    let old_root = tempfile::tempdir().expect("create old root");
    let new_root = tempfile::tempdir().expect("create new root");
    let profile_root = tempfile::tempdir().expect("create profile root");
    let old_root = old_root.path().abs();
    let new_root = new_root.path().abs();
    let profile_root = profile_root.path().abs();
    session_configuration.workspace_roots = vec![old_root.clone()];

    let file_system_sandbox_policy =
        FileSystemSandboxPolicy::restricted(vec![FileSystemSandboxEntry {
            path: FileSystemPath::Special {
                value: FileSystemSpecialPath::project_roots(/*subpath*/ None),
            },
            access: FileSystemAccessMode::Write,
        }]);
    let permission_profile = PermissionProfile::from_runtime_permissions(
        &file_system_sandbox_policy,
        NetworkSandboxPolicy::Restricted,
    );

    let updated = session_configuration
        .apply(&SessionSettingsUpdate {
            workspace_roots: Some(vec![new_root.clone()]),
            permission_profile: Some(permission_profile),
            active_permission_profile: Some(ActivePermissionProfile::new("dev")),
            profile_workspace_roots: Some(vec![profile_root.clone()]),
            ..Default::default()
        })
        .expect("permission profile update should succeed");

    let updated_policy = updated.file_system_sandbox_policy();
    assert!(updated_policy.can_write_path_with_cwd(new_root.as_path(), updated.cwd.as_path()));
    assert!(!updated_policy.can_write_path_with_cwd(old_root.as_path(), updated.cwd.as_path()));
    assert_eq!(
        updated.active_permission_profile(),
        Some(ActivePermissionProfile::new("dev"))
    );
    assert_eq!(updated.profile_workspace_roots(), &[profile_root]);
}

#[tokio::test]
async fn session_configuration_apply_retargets_implicit_workspace_root_on_cwd_update() {
    let mut session_configuration = make_session_configuration_for_tests().await;
    let old_root = tempfile::tempdir().expect("create old root");
    let new_root = tempfile::tempdir().expect("create new root");
    let extra_root = tempfile::tempdir().expect("create extra root");
    let old_root = old_root.path().abs();
    let new_root = new_root.path().abs();
    let extra_root = extra_root.path().abs();
    session_configuration.cwd = old_root.clone();
    session_configuration.workspace_roots = vec![old_root.clone(), extra_root.clone()];

    let file_system_sandbox_policy =
        FileSystemSandboxPolicy::restricted(vec![FileSystemSandboxEntry {
            path: FileSystemPath::Special {
                value: FileSystemSpecialPath::project_roots(/*subpath*/ None),
            },
            access: FileSystemAccessMode::Write,
        }]);
    let permission_profile = PermissionProfile::from_runtime_permissions(
        &file_system_sandbox_policy,
        NetworkSandboxPolicy::Restricted,
    );
    session_configuration
        .set_permission_profile_for_tests(permission_profile)
        .expect("set permission profile");

    let updated = session_configuration
        .apply(&SessionSettingsUpdate {
            cwd: Some(new_root.to_path_buf()),
            ..Default::default()
        })
        .expect("cwd-only update should succeed");

    assert_eq!(
        updated.workspace_roots,
        vec![new_root.clone(), extra_root.clone()]
    );
    let updated_policy = updated.file_system_sandbox_policy();
    assert!(updated_policy.can_write_path_with_cwd(new_root.as_path(), updated.cwd.as_path()));
    assert!(updated_policy.can_write_path_with_cwd(extra_root.as_path(), updated.cwd.as_path()));
    assert!(!updated_policy.can_write_path_with_cwd(old_root.as_path(), updated.cwd.as_path()));
}

#[cfg_attr(windows, ignore)]
#[tokio::test]
async fn new_default_turn_uses_config_aware_skills_for_role_overrides() {
    let (session, _turn_context) = make_session_and_context().await;
    let parent_config = session.get_config().await;
    let codex_home = parent_config.codex_home.clone();
    let skill_dir = codex_home.join("skills").join("demo");
    std::fs::create_dir_all(&skill_dir).expect("create skill dir");
    let skill_path = skill_dir.join("SKILL.md");
    std::fs::write(
        &skill_path,
        "---\nname: demo-skill\ndescription: demo description\n---\n\n# Body\n",
    )
    .expect("write skill");

    let skill_fs = session
        .services
        .environment_manager
        .default_environment()
        .map(|environment| environment.get_filesystem())
        .unwrap_or_else(|| std::sync::Arc::clone(&codex_exec_server::LOCAL_FS));
    let parent_outcome = session
        .services
        .skills_manager
        .skills_for_cwd(
            &crate::skills_load_input_from_config(&parent_config, Vec::new()),
            /*force_reload*/ true,
            Some(Arc::clone(&skill_fs)),
        )
        .await;
    let parent_skill = parent_outcome
        .skills
        .iter()
        .find(|skill| skill.name == "demo-skill")
        .expect("demo skill should be discovered");
    assert_eq!(parent_outcome.is_skill_enabled(parent_skill), true);

    let role_path = codex_home.join("skills-role.toml");
    std::fs::write(
        &role_path,
        format!(
            r#"developer_instructions = "Stay focused"

[[skills.config]]
path = "{}"
enabled = false
"#,
            skill_path.display()
        ),
    )
    .expect("write role config");

    let mut child_config = (*parent_config).clone();
    child_config.agent_roles.insert(
        "custom".to_string(),
        crate::config::AgentRoleConfig {
            description: None,
            config_file: Some(role_path.to_path_buf()),
            agent_card_manifest_source: None,
            agent_card_manifest_version: None,
            agent_card_manifest: None,
            nickname_candidates: None,
        },
    );
    crate::agent::role::apply_role_to_config(&mut child_config, Some("custom"))
        .await
        .expect("custom role should apply");

    {
        let mut state = session.state.lock().await;
        state.session_configuration.original_config_do_not_use = Arc::new(child_config);
    }

    let child_turn = session
        .new_default_turn_with_sub_id("role-skill-turn".to_string())
        .await;
    let child_skill = child_turn
        .turn_skills
        .outcome
        .skills
        .iter()
        .find(|skill| skill.name == "demo-skill")
        .expect("demo skill should be discovered");
    assert_eq!(
        child_turn.turn_skills.outcome.is_skill_enabled(child_skill),
        false
    );
}

#[tokio::test]
async fn session_configuration_apply_retargets_legacy_workspace_root_on_cwd_update() {
    let mut session_configuration = make_session_configuration_for_tests().await;
    let workspace = tempfile::tempdir().expect("create temp dir");
    let original_cwd = workspace.path().join("repo-a").abs();
    let project_root = workspace.path().join("repo-b").abs();
    session_configuration.cwd = original_cwd.clone();
    session_configuration.workspace_roots = vec![session_configuration.cwd.clone()];
    let sandbox_policy = SandboxPolicy::WorkspaceWrite {
        writable_roots: Vec::new(),
        network_access: false,
        exclude_tmpdir_env_var: true,
        exclude_slash_tmp: true,
    };
    let file_system_sandbox_policy = FileSystemSandboxPolicy::from_legacy_sandbox_policy_for_cwd(
        &sandbox_policy,
        &session_configuration.cwd,
    );
    session_configuration
        .set_permission_profile_for_tests(
            PermissionProfile::from_runtime_permissions_with_enforcement(
                SandboxEnforcement::from_legacy_sandbox_policy(&sandbox_policy),
                &file_system_sandbox_policy,
                NetworkSandboxPolicy::from(&sandbox_policy),
            ),
        )
        .expect("set permission profile");

    let updated = session_configuration
        .apply(&SessionSettingsUpdate {
            cwd: Some(project_root.to_path_buf()),
            ..Default::default()
        })
        .expect("cwd-only update should succeed");

    assert_eq!(updated.workspace_roots, vec![project_root.clone()]);
    assert!(
        updated
            .file_system_sandbox_policy()
            .can_write_path_with_cwd(project_root.as_path(), updated.cwd.as_path()),
        "cwd-only update should keep the new cwd writable"
    );
    assert!(
        !updated
            .file_system_sandbox_policy()
            .can_write_path_with_cwd(original_cwd.as_path(), updated.cwd.as_path()),
        "cwd-only update should not keep the old implicit cwd writable"
    );
}

#[tokio::test]
async fn session_configuration_apply_preserves_absolute_cwd_write_root_on_cwd_update() {
    let mut session_configuration = make_session_configuration_for_tests().await;
    let workspace = tempfile::tempdir().expect("create temp dir");
    let original_cwd = workspace.path().join("repo-a");
    let next_cwd = workspace.path().join("repo-b");
    std::fs::create_dir_all(&original_cwd).expect("create original cwd");
    std::fs::create_dir_all(&next_cwd).expect("create next cwd");
    let original_cwd = original_cwd.abs();

    session_configuration.cwd = original_cwd.clone();
    let file_system_sandbox_policy = FileSystemSandboxPolicy::restricted(vec![
        FileSystemSandboxEntry {
            path: FileSystemPath::Special {
                value: FileSystemSpecialPath::Root,
            },
            access: FileSystemAccessMode::Read,
        },
        FileSystemSandboxEntry {
            path: FileSystemPath::Path {
                path: original_cwd.clone(),
            },
            access: FileSystemAccessMode::Write,
        },
    ]);
    session_configuration
        .set_permission_profile_for_tests(
            PermissionProfile::from_runtime_permissions_with_enforcement(
                SandboxEnforcement::Managed,
                &file_system_sandbox_policy,
                NetworkSandboxPolicy::Restricted,
            ),
        )
        .expect("set permission profile");

    let updated = session_configuration
        .apply(&SessionSettingsUpdate {
            cwd: Some(next_cwd.clone()),
            ..Default::default()
        })
        .expect("cwd-only update should succeed");

    assert_eq!(
        updated.file_system_sandbox_policy(),
        file_system_sandbox_policy
    );
    assert!(
        updated
            .file_system_sandbox_policy()
            .can_write_path_with_cwd(original_cwd.as_path(), updated.cwd.as_path()),
        "absolute grant to the old cwd must remain writable"
    );
    assert!(
        !updated
            .file_system_sandbox_policy()
            .can_write_path_with_cwd(next_cwd.as_path(), updated.cwd.as_path()),
        "cwd-only update must not reinterpret an absolute old-cwd grant as :workspace_roots"
    );
}

#[tokio::test]
async fn session_update_settings_does_not_rewrite_sticky_environment_cwds() {
    let (session, turn_context) = make_session_and_context().await;
    #[allow(deprecated)]
    let updated_cwd = turn_context.cwd.join("project");
    std::fs::create_dir_all(updated_cwd.as_path()).expect("create project dir");

    session
        .update_settings(SessionSettingsUpdate {
            cwd: Some(PathBuf::from("project")),
            ..Default::default()
        })
        .await
        .expect("cwd update should succeed");

    let session_cwd = {
        let state = session.state.lock().await;
        state.session_configuration.cwd.clone()
    };
    let config = session.get_config().await;
    let next_turn = session.new_default_turn().await;

    assert_eq!(session_cwd, updated_cwd);
    #[allow(deprecated)]
    let turn_cwd = turn_context.cwd.clone();
    #[allow(deprecated)]
    let next_turn_cwd = next_turn.cwd.clone();
    assert_eq!(config.cwd, turn_cwd);
    assert_eq!(next_turn_cwd, updated_cwd);
    assert_eq!(next_turn.config.cwd, updated_cwd);
}

#[tokio::test]
async fn relative_cwd_update_without_environments_resolves_under_session_cwd() {
    let (session, _turn_context) = make_session_and_context().await;
    let original_cwd = {
        let mut state = session.state.lock().await;
        state.session_configuration.environments = Vec::new();
        state.session_configuration.cwd.clone()
    };
    let updated_cwd = original_cwd.join("project");
    std::fs::create_dir_all(updated_cwd.as_path()).expect("create project dir");

    session
        .update_settings(SessionSettingsUpdate {
            cwd: Some(PathBuf::from("project")),
            ..Default::default()
        })
        .await
        .expect("cwd update should succeed");

    let state = session.state.lock().await;
    assert_eq!(state.session_configuration.cwd, updated_cwd);
    assert!(state.session_configuration.environments.is_empty());
}

#[tokio::test]
async fn cwd_update_does_not_rewrite_sticky_environment_cwd() {
    let (session, _turn_context) = make_session_and_context().await;
    let (original_cwd, environment_cwd) = {
        let mut state = session.state.lock().await;
        let original_cwd = state.session_configuration.cwd.clone();
        let environment_cwd = original_cwd.join("environment");
        state.session_configuration.environments = vec![TurnEnvironmentSelection {
            environment_id: codex_exec_server::LOCAL_ENVIRONMENT_ID.to_string(),
            cwd: environment_cwd.clone().into(),
        }];
        (original_cwd, environment_cwd)
    };
    let updated_cwd = original_cwd.join("project");
    std::fs::create_dir_all(updated_cwd.as_path()).expect("create project dir");

    session
        .update_settings(SessionSettingsUpdate {
            cwd: Some(PathBuf::from("project")),
            ..Default::default()
        })
        .await
        .expect("cwd update should succeed");

    let state = session.state.lock().await;
    assert_eq!(state.session_configuration.cwd, updated_cwd);
    assert_eq!(
        state.session_configuration.environments[0].cwd,
        environment_cwd.into()
    );
}

#[tokio::test]
async fn absolute_cwd_update_with_turn_environment_is_allowed() {
    let (session, _turn_context, _rx) = make_session_and_context_with_rx().await;
    let absolute_cwd = {
        let state = session.state.lock().await;
        state.session_configuration.cwd.join("absolute-turn")
    };
    std::fs::create_dir_all(absolute_cwd.as_path()).expect("create absolute turn dir");

    let turn_context = session
        .new_turn_with_sub_id(
            "sub-1".to_string(),
            SessionSettingsUpdate {
                cwd: Some(absolute_cwd.to_path_buf()),
                environments: Some(vec![TurnEnvironmentSelection {
                    environment_id: codex_exec_server::LOCAL_ENVIRONMENT_ID.to_string(),
                    cwd: absolute_cwd.clone().into(),
                }]),
                ..Default::default()
            },
        )
        .await
        .expect("absolute cwd with explicit environments should succeed");

    #[allow(deprecated)]
    let turn_cwd = turn_context.cwd.clone();
    assert_eq!(turn_cwd, absolute_cwd);
    assert_eq!(turn_context.config.cwd, absolute_cwd);
    assert_eq!(turn_context.environments.turn_environments.len(), 1);
}

#[tokio::test]
async fn session_new_fails_when_zsh_fork_enabled_without_zsh_path() {
    let codex_home = tempfile::tempdir().expect("create temp dir");
    let mut config = build_test_config(codex_home.path()).await;
    config
        .features
        .enable(Feature::ShellZshFork)
        .expect("test config should allow shell_zsh_fork");
    config.zsh_path = None;
    let config = Arc::new(config);

    let auth_manager = AuthManager::from_auth_for_testing(CodexAuth::from_api_key("Test API Key"));
    let models_manager = models_manager_with_provider(
        config.codex_home.to_path_buf(),
        auth_manager.clone(),
        config.model_provider.clone(),
    );
    let model = get_model_offline_for_tests(config.model.as_deref());
    let model_info =
        construct_model_info_offline_for_tests(model.as_str(), &config.to_models_manager_config());
    let collaboration_mode = CollaborationMode {
        mode: ModeKind::Default,
        settings: Settings {
            model,
            reasoning_effort: config.model_reasoning_effort,
            developer_instructions: None,
        },
    };
    let session_configuration = SessionConfiguration {
        provider: config.model_provider.clone(),
        collaboration_mode,
        model_reasoning_summary: config.model_reasoning_summary,
        developer_instructions: config.developer_instructions.clone(),
        user_instructions: config.user_instructions.clone(),
        service_tier: None,
        personality: config.personality,
        history_mode: ThreadHistoryMode::Legacy,
        base_instructions: config
            .base_instructions
            .clone()
            .unwrap_or_else(|| model_info.get_model_instructions(config.personality)),
        compact_prompt: config.compact_prompt.clone(),
        approval_policy: config.permissions.approval_policy.clone(),
        approvals_reviewer: config.approvals_reviewer,
        permission_profile_state: config.permissions.permission_profile_state().clone(),
        windows_sandbox_level: WindowsSandboxLevel::from_config(&config),
        cwd: config.cwd.clone().into(),
        workspace_roots: config.workspace_roots.clone(),
        codex_home: config.codex_home.clone(),
        thread_name: None,
        environments: Vec::new(),
        original_config_do_not_use: Arc::clone(&config),
        metrics_service_name: None,
        app_server_client_name: None,
        app_server_client_version: None,
        session_source: SessionSource::Exec,
        thread_source: None,
        dynamic_tools: Vec::new(),
        persist_extended_history: false,
        inherited_shell_snapshot: None,
        user_shell_override: None,
    };

    let (tx_event, _rx_event) = async_channel::unbounded();
    let (agent_status_tx, _agent_status_rx) = watch::channel(AgentStatus::PendingInit);
    let plugins_manager = Arc::new(PluginsManager::new(config.codex_home.to_path_buf()));
    let mcp_manager = Arc::new(McpManager::new(Arc::clone(&plugins_manager)));
    let skills_manager = Arc::new(SkillsManager::new(
        config.codex_home.clone(),
        /*bundled_skills_enabled*/ true,
    ));
    let result = Session::new(
        session_configuration,
        Arc::clone(&config),
        "11111111-1111-4111-8111-111111111111".to_string(),
        auth_manager,
        models_manager,
        Arc::new(ExecPolicyManager::default()),
        tx_event,
        agent_status_tx,
        InitialHistory::New,
        SessionSource::Exec,
        skills_manager,
        plugins_manager,
        mcp_manager,
        Arc::new(codex_extension_api::ExtensionRegistryBuilder::new().build()),
        AgentControl::default(),
        Arc::new(codex_exec_server::EnvironmentManager::default_for_tests()),
        /*analytics_events_client*/ None,
        Arc::new(codex_thread_store::LocalThreadStore::new(
            codex_thread_store::LocalThreadStoreConfig::from_config(config.as_ref()),
            /*state_db*/ None,
        )),
        codex_rollout_trace::ThreadTraceContext::disabled(),
        /*attestation_provider*/ None,
    )
    .await;

    let err = match result {
        Ok(_) => panic!("expected startup to fail"),
        Err(err) => err,
    };
    let msg = format!("{err:#}");
    assert!(msg.contains("zsh fork feature enabled, but `zsh_path` is not configured"));
}

// todo: use online model info
pub(crate) async fn make_session_and_context() -> (Session, TurnContext) {
    let (tx_event, _rx_event) = async_channel::unbounded();
    let codex_home = tempfile::tempdir().expect("create temp dir");
    let config = build_test_config(codex_home.path()).await;
    let config = Arc::new(config);
    let thread_id = ThreadId::default();
    let auth_manager = AuthManager::from_auth_for_testing(CodexAuth::from_api_key("Test API Key"));
    let models_manager = models_manager_with_provider(
        config.codex_home.to_path_buf(),
        auth_manager.clone(),
        config.model_provider.clone(),
    );
    let agent_control = AgentControl::default();
    let exec_policy = Arc::new(ExecPolicyManager::default());
    let (agent_status_tx, _agent_status_rx) = watch::channel(AgentStatus::PendingInit);
    let model = get_model_offline_for_tests(config.model.as_deref());
    let model_info =
        construct_model_info_offline_for_tests(model.as_str(), &config.to_models_manager_config());
    let reasoning_effort = config.model_reasoning_effort;
    let collaboration_mode = CollaborationMode {
        mode: ModeKind::Default,
        settings: Settings {
            model,
            reasoning_effort,
            developer_instructions: None,
        },
    };
    let default_environments = vec![TurnEnvironmentSelection {
        environment_id: codex_exec_server::LOCAL_ENVIRONMENT_ID.to_string(),
        cwd: config.cwd.clone().into(),
    }];
    let session_configuration = SessionConfiguration {
        provider: config.model_provider.clone(),
        collaboration_mode,
        model_reasoning_summary: config.model_reasoning_summary,
        developer_instructions: config.developer_instructions.clone(),
        user_instructions: config.user_instructions.clone(),
        service_tier: None,
        personality: config.personality,
        history_mode: ThreadHistoryMode::Legacy,
        base_instructions: config
            .base_instructions
            .clone()
            .unwrap_or_else(|| model_info.get_model_instructions(config.personality)),
        compact_prompt: config.compact_prompt.clone(),
        approval_policy: config.permissions.approval_policy.clone(),
        approvals_reviewer: config.approvals_reviewer,
        permission_profile_state: config.permissions.permission_profile_state().clone(),
        windows_sandbox_level: WindowsSandboxLevel::from_config(&config),
        cwd: config.cwd.clone().into(),
        workspace_roots: config.workspace_roots.clone(),
        codex_home: config.codex_home.clone(),
        thread_name: None,
        environments: default_environments,
        original_config_do_not_use: Arc::clone(&config),
        metrics_service_name: None,
        app_server_client_name: None,
        app_server_client_version: None,
        session_source: SessionSource::Exec,
        thread_source: None,
        dynamic_tools: Vec::new(),
        persist_extended_history: false,
        inherited_shell_snapshot: None,
        user_shell_override: None,
    };
    let per_turn_config =
        Session::build_per_turn_config(&session_configuration, session_configuration.cwd.clone());
    let model_info = construct_model_info_offline_for_tests(
        session_configuration.collaboration_mode.model(),
        &per_turn_config.to_models_manager_config(),
    );
    let session_telemetry = session_telemetry(
        thread_id,
        config.as_ref(),
        &model_info,
        session_configuration.session_source.clone(),
    );

    let state = SessionState::new(session_configuration.clone());
    let plugins_manager = Arc::new(PluginsManager::new(config.codex_home.to_path_buf()));
    let mcp_manager = Arc::new(McpManager::new(Arc::clone(&plugins_manager)));
    let skills_manager = Arc::new(SkillsManager::new(
        config.codex_home.clone(),
        /*bundled_skills_enabled*/ true,
    ));
    let network_approval = Arc::new(NetworkApprovalService::default());
    let environment = Arc::new(
        codex_exec_server::Environment::create_for_tests(/*exec_server_url*/ None)
            .expect("create environment"),
    );

    let services = SessionServices {
        mcp_connection_manager: Arc::new(RwLock::new(
            crate::state::PublishedMcpConnectionManager::new(
                McpConnectionManager::new_uninitialized_with_permission_profile(
                    &config.permissions.approval_policy,
                    config.permissions.permission_profile(),
                ),
                crate::state::FrozenMcpAuthSnapshot::capture(auth_manager.as_ref())
                    .await
                    .expect("capture MCP auth snapshot")
                    .binding(),
                codex_protocol::protocol::McpElicitationAuthority {
                    approval_policy: config.permissions.approval_policy.value(),
                    permission_profile: config.permissions.effective_permission_profile(),
                    approvals_reviewer: config.approvals_reviewer,
                    apps_approvals_reviewers: Default::default(),
                },
            ),
        )),
        mcp_startup_cancellation_token: Mutex::new(CancellationToken::new()),
        unified_exec_manager: UnifiedExecProcessManager::new(
            config.background_terminal_max_timeout,
        ),
        shell_zsh_path: None,
        main_execve_wrapper_exe: config.main_execve_wrapper_exe.clone(),
        analytics_events_client: AnalyticsEventsClient::new(
            Arc::clone(&auth_manager),
            config.chatgpt_base_url.trim_end_matches('/').to_string(),
            config.analytics_enabled,
        ),
        hooks: arc_swap::ArcSwap::from_pointee(Hooks::new(HooksConfig {
            legacy_notify_argv: config.notify.clone(),
            ..HooksConfig::default()
        })),
        rollout_thread_trace: codex_rollout_trace::ThreadTraceContext::disabled(),
        user_shell: Arc::new(default_user_shell()),
        shell_snapshot_tx: watch::channel(None).0,
        show_raw_agent_reasoning: config.show_raw_agent_reasoning,
        exec_policy,
        auth_manager: auth_manager.clone(),
        openai_file_upload_client_pool:
            codex_http_client::RouteAwareClientPool::new_without_request_logging(
                config.http_client_factory(),
                codex_http_client::ClientRouteClass::Api,
            )
            .with_legacy_custom_ca_fallback(),
        session_telemetry: session_telemetry.clone(),
        models_manager: Arc::clone(&models_manager),
        tool_approvals: Mutex::new(ApprovalStore::default()),
        guardian_rejections: Mutex::new(std::collections::HashMap::new()),
        guardian_rejection_circuit_breaker: Mutex::new(Default::default()),
        runtime_handle: tokio::runtime::Handle::current(),
        skills_manager,
        plugins_manager,
        mcp_manager,
        extensions: Arc::new(codex_extension_api::ExtensionRegistryBuilder::new().build()),
        session_extension_data: codex_extension_api::ExtensionData::new(
            agent_control.session_id().to_string(),
        ),
        thread_extension_data: codex_extension_api::ExtensionData::new(thread_id.to_string()),
        agent_control,
        network_proxy: None,
        network_approval: Arc::clone(&network_approval),
        state_db: None,
        live_thread: None,
        thread_store: Arc::new(codex_thread_store::LocalThreadStore::new(
            codex_thread_store::LocalThreadStoreConfig::from_config(config.as_ref()),
            /*state_db*/ None,
        )),
        attestation_provider: None,
        model_client: ModelClient::new(
            Some(auth_manager.clone()),
            thread_id.into(),
            thread_id,
            /*installation_id*/ "11111111-1111-4111-8111-111111111111".to_string(),
            session_configuration.provider.clone(),
            session_configuration.session_source.clone(),
            config.model_verbosity,
            config.features.enabled(Feature::EnableRequestCompression),
            config.features.enabled(Feature::RuntimeMetrics),
            Session::build_model_client_beta_features_header(config.as_ref()),
            /*attestation_provider*/ None,
        ),
        code_mode_service: crate::tools::code_mode::CodeModeService::new(),
        environment_manager: Arc::new(codex_exec_server::EnvironmentManager::default_for_tests()),
    };

    let plugin_outcome = services
        .plugins_manager
        .plugins_for_config(&per_turn_config.plugins_config_input())
        .await;
    let effective_skill_roots = plugin_outcome.effective_plugin_skill_roots();
    let skills_input =
        crate::skills_load_input_from_config(&per_turn_config, effective_skill_roots);
    let skill_fs = environment.get_filesystem();
    let skills_outcome = Arc::new(
        services
            .skills_manager
            .skills_for_config(&skills_input, Some(Arc::clone(&skill_fs)))
            .await,
    );
    let turn_environments = turn_environments_for_tests(&environment, &session_configuration.cwd);
    let turn_context = Session::make_turn_context(
        thread_id,
        SessionId::from(thread_id),
        Some(Arc::clone(&auth_manager)),
        &session_telemetry,
        session_configuration.provider.clone(),
        &session_configuration,
        services.user_shell.as_ref(),
        services.shell_zsh_path.as_ref(),
        services.main_execve_wrapper_exe.as_ref(),
        per_turn_config,
        model_info,
        &models_manager,
        /*network*/ None,
        turn_environments,
        session_configuration.cwd.clone(),
        "turn_id".to_string(),
        skills_outcome,
        /*goal_tools_supported*/ true,
    )
    .await;

    let (mailbox, mailbox_rx) = crate::agent::Mailbox::new();
    let session = Session {
        conversation_id: thread_id,
        installation_id: "11111111-1111-4111-8111-111111111111".to_string(),
        tx_event,
        agent_status: agent_status_tx,
        out_of_band_elicitation_paused: watch::channel(false).0,
        state: Mutex::new(state),
        managed_network_proxy_refresh_lock: Semaphore::new(/*permits*/ 1),
        features: config.features.clone(),
        mcp_server_refresh_lock: Semaphore::new(/*permits*/ 1),
        mcp_server_refresh_state: Mutex::new(super::session::McpServerRefreshState::default()),
        mcp_server_refresh_test_gate: Mutex::new(None),
        mcp_server_refresh_publication_test_gate: Mutex::new(None),
        mcp_server_refresh_retry_test_notify: Mutex::new(None),
        conversation: Arc::new(RealtimeConversationManager::new()),
        active_turn: Mutex::new(None),
        mailbox,
        mailbox_rx: Mutex::new(mailbox_rx),
        idle_pending_input: Mutex::new(Vec::new()),
        goal_runtime: crate::goals::GoalRuntimeState::new(),
        guardian_review_session: crate::guardian::GuardianReviewSessionManager::default(),
        services,
        next_internal_sub_id: AtomicU64::new(0),
    };

    (session, turn_context)
}

async fn make_session_with_config(
    mutator: impl FnOnce(&mut Config),
) -> anyhow::Result<Arc<Session>> {
    let (session, _rx_event) = make_session_with_config_and_rx(mutator).await?;
    Ok(session)
}

async fn load_latest_config_for_session(session: &Session) -> Config {
    let config = session.get_config().await;
    ConfigBuilder::default()
        .codex_home(config.codex_home.to_path_buf())
        .fallback_cwd(Some(config.cwd.to_path_buf()))
        .build()
        .await
        .expect("load latest config for session")
}

async fn make_session_with_config_and_rx(
    mutator: impl FnOnce(&mut Config),
) -> anyhow::Result<(Arc<Session>, async_channel::Receiver<Event>)> {
    make_session_with_config_and_auth_manager_and_rx(mutator, None).await
}

async fn make_session_with_config_and_auth_manager_and_rx(
    mutator: impl FnOnce(&mut Config),
    auth_manager: Option<Arc<AuthManager>>,
) -> anyhow::Result<(Arc<Session>, async_channel::Receiver<Event>)> {
    let codex_home = tempfile::tempdir().expect("create temp dir");
    let mut config = build_test_config(codex_home.path()).await;
    mutator(&mut config);
    let config = Arc::new(config);
    let auth_manager = auth_manager.unwrap_or_else(|| {
        AuthManager::from_auth_for_testing(CodexAuth::from_api_key("Test API Key"))
    });
    let models_manager = models_manager_with_provider(
        config.codex_home.to_path_buf(),
        auth_manager.clone(),
        config.model_provider.clone(),
    );
    let model = get_model_offline_for_tests(config.model.as_deref());
    let model_info =
        construct_model_info_offline_for_tests(model.as_str(), &config.to_models_manager_config());
    let collaboration_mode = CollaborationMode {
        mode: ModeKind::Default,
        settings: Settings {
            model,
            reasoning_effort: config.model_reasoning_effort,
            developer_instructions: None,
        },
    };
    let default_environments = vec![TurnEnvironmentSelection {
        environment_id: codex_exec_server::LOCAL_ENVIRONMENT_ID.to_string(),
        cwd: config.cwd.clone().into(),
    }];
    let session_configuration = SessionConfiguration {
        provider: config.model_provider.clone(),
        collaboration_mode,
        model_reasoning_summary: config.model_reasoning_summary,
        developer_instructions: config.developer_instructions.clone(),
        user_instructions: config.user_instructions.clone(),
        service_tier: None,
        personality: config.personality,
        history_mode: ThreadHistoryMode::Legacy,
        base_instructions: config
            .base_instructions
            .clone()
            .unwrap_or_else(|| model_info.get_model_instructions(config.personality)),
        compact_prompt: config.compact_prompt.clone(),
        approval_policy: config.permissions.approval_policy.clone(),
        approvals_reviewer: config.approvals_reviewer,
        permission_profile_state: config.permissions.permission_profile_state().clone(),
        windows_sandbox_level: WindowsSandboxLevel::from_config(&config),
        cwd: config.cwd.clone(),
        workspace_roots: config.workspace_roots.clone(),
        codex_home: config.codex_home.clone(),
        thread_name: None,
        environments: default_environments,
        original_config_do_not_use: Arc::clone(&config),
        metrics_service_name: None,
        app_server_client_name: None,
        app_server_client_version: None,
        session_source: SessionSource::Exec,
        thread_source: None,
        dynamic_tools: Vec::new(),
        persist_extended_history: false,
        inherited_shell_snapshot: None,
        user_shell_override: None,
    };

    let (tx_event, rx_event) = async_channel::unbounded();
    let (agent_status_tx, _agent_status_rx) = watch::channel(AgentStatus::PendingInit);
    let plugins_manager = Arc::new(PluginsManager::new(config.codex_home.to_path_buf()));
    let mcp_manager = Arc::new(McpManager::new(Arc::clone(&plugins_manager)));
    let skills_manager = Arc::new(SkillsManager::new(
        config.codex_home.clone(),
        /*bundled_skills_enabled*/ true,
    ));

    let session = Session::new(
        session_configuration,
        Arc::clone(&config),
        "11111111-1111-4111-8111-111111111111".to_string(),
        auth_manager,
        models_manager,
        Arc::new(ExecPolicyManager::default()),
        tx_event,
        agent_status_tx,
        InitialHistory::New,
        SessionSource::Exec,
        skills_manager,
        plugins_manager,
        mcp_manager,
        Arc::new(codex_extension_api::ExtensionRegistryBuilder::new().build()),
        AgentControl::default(),
        Arc::new(codex_exec_server::EnvironmentManager::default_for_tests()),
        /*analytics_events_client*/ None,
        Arc::new(codex_thread_store::LocalThreadStore::new(
            codex_thread_store::LocalThreadStoreConfig::from_config(config.as_ref()),
            /*state_db*/ None,
        )),
        codex_rollout_trace::ThreadTraceContext::disabled(),
        /*attestation_provider*/ None,
    )
    .await?;

    Ok((session, rx_event))
}

async fn make_session_with_history_source_and_agent_control_and_rx(
    initial_history: InitialHistory,
    session_source: SessionSource,
    agent_control: AgentControl,
) -> anyhow::Result<(Arc<Session>, async_channel::Receiver<Event>)> {
    let codex_home = tempfile::tempdir().expect("create temp dir");
    let mut config = build_test_config(codex_home.path()).await;
    config.ephemeral = true;
    let config = Arc::new(config);
    let auth_manager = AuthManager::from_auth_for_testing(CodexAuth::from_api_key("Test API Key"));
    let models_manager = models_manager_with_provider(
        config.codex_home.to_path_buf(),
        auth_manager.clone(),
        config.model_provider.clone(),
    );
    let model = get_model_offline_for_tests(config.model.as_deref());
    let model_info =
        construct_model_info_offline_for_tests(model.as_str(), &config.to_models_manager_config());
    let collaboration_mode = CollaborationMode {
        mode: ModeKind::Default,
        settings: Settings {
            model,
            reasoning_effort: config.model_reasoning_effort,
            developer_instructions: None,
        },
    };
    let default_environments = vec![TurnEnvironmentSelection {
        environment_id: codex_exec_server::LOCAL_ENVIRONMENT_ID.to_string(),
        cwd: config.cwd.clone().into(),
    }];
    let session_configuration = SessionConfiguration {
        provider: config.model_provider.clone(),
        collaboration_mode,
        model_reasoning_summary: config.model_reasoning_summary,
        developer_instructions: config.developer_instructions.clone(),
        user_instructions: config.user_instructions.clone(),
        service_tier: None,
        personality: config.personality,
        history_mode: ThreadHistoryMode::Legacy,
        base_instructions: config
            .base_instructions
            .clone()
            .unwrap_or_else(|| model_info.get_model_instructions(config.personality)),
        compact_prompt: config.compact_prompt.clone(),
        approval_policy: config.permissions.approval_policy.clone(),
        approvals_reviewer: config.approvals_reviewer,
        permission_profile_state: config.permissions.permission_profile_state().clone(),
        windows_sandbox_level: WindowsSandboxLevel::from_config(&config),
        cwd: config.cwd.clone(),
        workspace_roots: config.workspace_roots.clone(),
        codex_home: config.codex_home.clone(),
        thread_name: None,
        environments: default_environments,
        original_config_do_not_use: Arc::clone(&config),
        metrics_service_name: None,
        app_server_client_name: None,
        app_server_client_version: None,
        session_source: session_source.clone(),
        thread_source: None,
        dynamic_tools: Vec::new(),
        persist_extended_history: false,
        inherited_shell_snapshot: None,
        user_shell_override: None,
    };

    let (tx_event, rx_event) = async_channel::unbounded();
    let (agent_status_tx, _agent_status_rx) = watch::channel(AgentStatus::PendingInit);
    let plugins_manager = Arc::new(PluginsManager::new(config.codex_home.to_path_buf()));
    let mcp_manager = Arc::new(McpManager::new(Arc::clone(&plugins_manager)));
    let skills_manager = Arc::new(SkillsManager::new(
        config.codex_home.clone(),
        /*bundled_skills_enabled*/ true,
    ));

    let session = Session::new(
        session_configuration,
        Arc::clone(&config),
        "11111111-1111-4111-8111-111111111111".to_string(),
        auth_manager,
        models_manager,
        Arc::new(ExecPolicyManager::default()),
        tx_event,
        agent_status_tx,
        initial_history,
        session_source,
        skills_manager,
        plugins_manager,
        mcp_manager,
        Arc::new(codex_extension_api::ExtensionRegistryBuilder::new().build()),
        agent_control,
        Arc::new(codex_exec_server::EnvironmentManager::default_for_tests()),
        /*analytics_events_client*/ None,
        Arc::new(codex_thread_store::LocalThreadStore::new(
            codex_thread_store::LocalThreadStoreConfig::from_config(config.as_ref()),
            Some(
                codex_state::StateRuntime::init(
                    config.sqlite_home.clone(),
                    config.model_provider_id.clone(),
                )
                .await
                .expect("state db should initialize"),
            ),
        )),
        codex_rollout_trace::ThreadTraceContext::disabled(),
        /*attestation_provider*/ None,
    )
    .await?;

    Ok((session, rx_event))
}

#[tokio::test]
async fn resumed_root_session_uses_thread_id_as_session_id() {
    let thread_id = ThreadId::new();
    let (session, rx_event) = make_session_with_history_source_and_agent_control_and_rx(
        InitialHistory::Resumed(ResumedHistory {
            conversation_id: thread_id,
            history: Vec::new(),
            rollout_path: None,
        }),
        SessionSource::Exec,
        AgentControl::default(),
    )
    .await
    .expect("resume should succeed");

    assert_eq!(session.thread_id(), thread_id);
    assert_eq!(session.session_id(), SessionId::from(thread_id));

    let event = rx_event.recv().await.expect("session configured event");
    let EventMsg::SessionConfigured(event) = event.msg else {
        panic!("expected session configured event");
    };
    assert_eq!(event.session_id, SessionId::from(thread_id));
    assert_eq!(event.thread_id, thread_id);
}

#[tokio::test]
async fn resumed_subagent_session_keeps_inherited_session_id() {
    let parent_thread_id = ThreadId::new();
    let parent_session_id = SessionId::from(parent_thread_id);
    let thread_id = ThreadId::new();
    let session_source = SessionSource::SubAgent(SubAgentSource::ThreadSpawn {
        parent_thread_id,
        depth: 1,
        agent_path: None,
        agent_nickname: None,
        agent_role: None,
    });
    let (session, rx_event) = make_session_with_history_source_and_agent_control_and_rx(
        InitialHistory::Resumed(ResumedHistory {
            conversation_id: thread_id,
            history: Vec::new(),
            rollout_path: None,
        }),
        session_source,
        AgentControl::default().with_session_id(parent_session_id),
    )
    .await
    .expect("resume should succeed");

    assert_eq!(session.thread_id(), thread_id);
    assert_eq!(session.session_id(), parent_session_id);

    let event = rx_event.recv().await.expect("session configured event");
    let EventMsg::SessionConfigured(event) = event.msg else {
        panic!("expected session configured event");
    };
    assert_eq!(event.session_id, parent_session_id);
    assert_eq!(event.thread_id, thread_id);
}

#[tokio::test]
async fn notify_request_permissions_response_ignores_unmatched_call_id() {
    let (session, _turn_context) = make_session_and_context().await;
    *session.active_turn.lock().await = Some(ActiveTurn::default());

    session
        .notify_request_permissions_response(
            "missing",
            codex_protocol::request_permissions::RequestPermissionsResponse {
                permissions: RequestPermissionProfile {
                    network: Some(codex_protocol::models::NetworkPermissions {
                        enabled: Some(true),
                    }),
                    ..RequestPermissionProfile::default()
                },
                scope: PermissionGrantScope::Turn,
                strict_auto_review: false,
            },
        )
        .await;

    assert_eq!(session.granted_turn_permissions().await, None);
}

#[tokio::test]
async fn record_granted_request_permissions_for_turn_uses_originating_turn() {
    let (session, _turn_context) = make_session_and_context().await;
    let originating_active_turn = ActiveTurn::default();
    let originating_turn_state = Arc::clone(&originating_active_turn.turn_state);
    *session.active_turn.lock().await = Some(originating_active_turn);

    let current_active_turn = ActiveTurn::default();
    let current_turn_state = Arc::clone(&current_active_turn.turn_state);
    *session.active_turn.lock().await = Some(current_active_turn);

    let requested_permissions = RequestPermissionProfile {
        network: Some(codex_protocol::models::NetworkPermissions {
            enabled: Some(true),
        }),
        ..RequestPermissionProfile::default()
    };
    session
        .record_granted_request_permissions_for_turn(
            &codex_protocol::request_permissions::RequestPermissionsResponse {
                permissions: requested_permissions.clone(),
                scope: PermissionGrantScope::Turn,
                strict_auto_review: false,
            },
            Some(&originating_turn_state),
        )
        .await;

    assert_eq!(
        originating_turn_state.lock().await.granted_permissions(),
        Some(requested_permissions.into())
    );
    assert_eq!(current_turn_state.lock().await.granted_permissions(), None);
    assert_eq!(session.granted_turn_permissions().await, None);
}

#[tokio::test]
async fn enable_strict_auto_review_for_turn_uses_originating_turn() {
    let (session, _turn_context) = make_session_and_context().await;
    let originating_active_turn = ActiveTurn::default();
    let originating_turn_state = Arc::clone(&originating_active_turn.turn_state);
    *session.active_turn.lock().await = Some(originating_active_turn);

    let requested_permissions = RequestPermissionProfile {
        network: Some(codex_protocol::models::NetworkPermissions {
            enabled: Some(true),
        }),
        ..RequestPermissionProfile::default()
    };
    session
        .record_granted_request_permissions_for_turn(
            &codex_protocol::request_permissions::RequestPermissionsResponse {
                permissions: requested_permissions.clone(),
                scope: PermissionGrantScope::Turn,
                strict_auto_review: true,
            },
            Some(&originating_turn_state),
        )
        .await;

    assert!(
        originating_turn_state
            .lock()
            .await
            .strict_auto_review_enabled()
    );
}

#[test]
fn strict_auto_review_session_scope_grants_no_permissions() {
    let requested_permissions = RequestPermissionProfile {
        network: Some(codex_protocol::models::NetworkPermissions {
            enabled: Some(true),
        }),
        ..RequestPermissionProfile::default()
    };

    let response = Session::normalize_request_permissions_response(
        requested_permissions.clone(),
        codex_protocol::request_permissions::RequestPermissionsResponse {
            permissions: requested_permissions,
            scope: PermissionGrantScope::Session,
            strict_auto_review: true,
        },
        std::path::Path::new("/tmp"),
    );

    assert_eq!(
        response,
        codex_protocol::request_permissions::RequestPermissionsResponse {
            permissions: RequestPermissionProfile::default(),
            scope: PermissionGrantScope::Turn,
            strict_auto_review: false,
        }
    );
}

#[tokio::test]
async fn request_permissions_emits_event_when_granular_policy_allows_requests() {
    let (session, mut turn_context, rx) = make_session_and_context_with_rx().await;
    *session.active_turn.lock().await = Some(ActiveTurn::default());
    Arc::get_mut(&mut turn_context)
        .expect("single turn context ref")
        .approval_policy
        .set(AskForApproval::Granular(GranularApprovalConfig {
            sandbox_approval: true,
            rules: true,
            skill_approval: true,
            request_permissions: true,
            mcp_elicitations: true,
        }))
        .expect("test setup should allow updating approval policy");

    let session = Arc::new(session);
    let turn_context = Arc::new(turn_context);
    let call_id = "call-1".to_string();
    let expected_response = codex_protocol::request_permissions::RequestPermissionsResponse {
        permissions: RequestPermissionProfile {
            network: Some(codex_protocol::models::NetworkPermissions {
                enabled: Some(true),
            }),
            ..RequestPermissionProfile::default()
        },
        scope: PermissionGrantScope::Turn,
        strict_auto_review: false,
    };

    let handle = tokio::spawn({
        let session = Arc::clone(&session);
        let turn_context = Arc::clone(&turn_context);
        let call_id = call_id.clone();
        async move {
            session
                .request_permissions(
                    &turn_context,
                    call_id,
                    codex_protocol::request_permissions::RequestPermissionsArgs {
                        reason: Some("need network".to_string()),
                        permissions: RequestPermissionProfile {
                            network: Some(codex_protocol::models::NetworkPermissions {
                                enabled: Some(true),
                            }),
                            ..RequestPermissionProfile::default()
                        },
                    },
                    CancellationToken::new(),
                )
                .await
        }
    });

    let request_event = tokio::time::timeout(StdDuration::from_secs(1), rx.recv())
        .await
        .expect("request_permissions event timed out")
        .expect("request_permissions event missing");
    let EventMsg::RequestPermissions(request) = request_event.msg else {
        panic!("expected request_permissions event");
    };
    assert_eq!(request.call_id, call_id);
    #[allow(deprecated)]
    let turn_cwd = turn_context.cwd.clone();
    assert_eq!(request.cwd, Some(turn_cwd));

    session
        .notify_request_permissions_response(&request.call_id, expected_response.clone())
        .await;

    let response = tokio::time::timeout(StdDuration::from_secs(1), handle)
        .await
        .expect("request_permissions future timed out")
        .expect("request_permissions join error");

    assert_eq!(response, Some(expected_response));
}

#[tokio::test]
async fn request_permissions_response_materializes_session_cwd_grants_before_recording() {
    let (session, mut turn_context, rx) = make_session_and_context_with_rx().await;
    *session.active_turn.lock().await = Some(ActiveTurn::default());
    Arc::get_mut(&mut turn_context)
        .expect("single turn context ref")
        .approval_policy
        .set(AskForApproval::Granular(GranularApprovalConfig {
            sandbox_approval: true,
            rules: true,
            skill_approval: true,
            request_permissions: true,
            mcp_elicitations: true,
        }))
        .expect("test setup should allow updating approval policy");

    let session = Arc::new(session);
    let turn_context = Arc::new(turn_context);
    let call_id = "call-1".to_string();
    let requested_permissions = RequestPermissionProfile {
        file_system: Some(FileSystemPermissions {
            entries: vec![FileSystemSandboxEntry {
                path: FileSystemPath::Special {
                    value: FileSystemSpecialPath::project_roots(/*subpath*/ None),
                },
                access: FileSystemAccessMode::Write,
            }],
            glob_scan_max_depth: None,
        }),
        ..Default::default()
    };

    let handle = tokio::spawn({
        let session = Arc::clone(&session);
        let turn_context = Arc::clone(&turn_context);
        let call_id = call_id.clone();
        let requested_permissions = requested_permissions.clone();
        async move {
            session
                .request_permissions(
                    &turn_context,
                    call_id,
                    codex_protocol::request_permissions::RequestPermissionsArgs {
                        reason: Some("need cwd write".to_string()),
                        permissions: requested_permissions,
                    },
                    CancellationToken::new(),
                )
                .await
        }
    });

    let request_event = tokio::time::timeout(StdDuration::from_secs(1), rx.recv())
        .await
        .expect("request_permissions event timed out")
        .expect("request_permissions event missing");
    let EventMsg::RequestPermissions(request) = request_event.msg else {
        panic!("expected request_permissions event");
    };
    let request_cwd = request.cwd.clone().expect("request cwd");

    session
        .notify_request_permissions_response(
            &request.call_id,
            codex_protocol::request_permissions::RequestPermissionsResponse {
                permissions: request.permissions,
                scope: PermissionGrantScope::Session,
                strict_auto_review: false,
            },
        )
        .await;

    let expected_permissions = RequestPermissionProfile {
        file_system: Some(FileSystemPermissions::from_read_write_roots(
            /*read*/ None,
            Some(vec![request_cwd]),
        )),
        ..Default::default()
    };
    let expected_response = codex_protocol::request_permissions::RequestPermissionsResponse {
        permissions: expected_permissions.clone(),
        scope: PermissionGrantScope::Session,
        strict_auto_review: false,
    };

    let response = tokio::time::timeout(StdDuration::from_secs(1), handle)
        .await
        .expect("request_permissions future timed out")
        .expect("request_permissions join error");

    assert_eq!(response, Some(expected_response));
    assert_eq!(
        session.granted_session_permissions().await,
        Some(expected_permissions.into())
    );
}

#[tokio::test]
async fn request_permissions_is_auto_denied_when_granular_policy_blocks_tool_requests() {
    let (session, mut turn_context, rx) = make_session_and_context_with_rx().await;
    *session.active_turn.lock().await = Some(ActiveTurn::default());
    Arc::get_mut(&mut turn_context)
        .expect("single turn context ref")
        .approval_policy
        .set(AskForApproval::Granular(GranularApprovalConfig {
            sandbox_approval: true,
            rules: true,
            skill_approval: true,
            request_permissions: false,
            mcp_elicitations: true,
        }))
        .expect("test setup should allow updating approval policy");

    let session = Arc::new(session);
    let turn_context = Arc::new(turn_context);
    let call_id = "call-1".to_string();
    let response = session
        .request_permissions(
            &turn_context,
            call_id,
            codex_protocol::request_permissions::RequestPermissionsArgs {
                reason: Some("need network".to_string()),
                permissions: RequestPermissionProfile {
                    network: Some(codex_protocol::models::NetworkPermissions {
                        enabled: Some(true),
                    }),
                    ..RequestPermissionProfile::default()
                },
            },
            CancellationToken::new(),
        )
        .await;

    assert_eq!(
        response,
        Some(
            codex_protocol::request_permissions::RequestPermissionsResponse {
                permissions: RequestPermissionProfile::default(),
                scope: PermissionGrantScope::Turn,
                strict_auto_review: false,
            }
        )
    );
    assert!(
        tokio::time::timeout(StdDuration::from_millis(100), rx.recv())
            .await
            .is_err(),
        "request_permissions should not emit an event when granular.request_permissions is false"
    );
}

#[tokio::test]
async fn submit_with_id_captures_current_span_trace_context() {
    let (session, _turn_context) = make_session_and_context().await;
    let (tx_sub, rx_sub) = async_channel::bounded(1);
    let (_tx_event, rx_event) = async_channel::unbounded();
    let (_agent_status_tx, agent_status) = watch::channel(AgentStatus::PendingInit);
    let codex = Codex {
        tx_sub,
        rx_event,
        agent_status,
        session: Arc::new(session),
        session_loop_termination: completed_session_loop_termination(),
    };

    let _trace_test_context = install_test_tracing("codex-core-tests");

    let request_parent = W3cTraceContext {
        traceparent: Some("00-00000000000000000000000000000011-0000000000000022-01".into()),
        tracestate: Some("vendor=value".into()),
    };
    let request_span = info_span!("app_server.request");
    assert!(set_parent_from_w3c_trace_context(
        &request_span,
        &request_parent
    ));

    let expected_trace = async {
        let expected_trace =
            current_span_w3c_trace_context().expect("current span should have trace context");
        codex
            .submit_with_id(Submission {
                id: "sub-1".into(),
                op: Op::Interrupt,
                trace: None,
                parent_turn_id: None,
            })
            .await
            .expect("submit should succeed");
        expected_trace
    }
    .instrument(request_span)
    .await;

    let submitted = rx_sub.recv().await.expect("submission");
    assert_eq!(submitted.trace, Some(expected_trace));
}

#[tokio::test]
async fn new_default_turn_captures_current_span_trace_id() {
    let (session, _turn_context) = make_session_and_context().await;

    let _trace_test_context = install_test_tracing("codex-core-tests");

    let request_parent = W3cTraceContext {
        traceparent: Some("00-00000000000000000000000000000011-0000000000000022-01".into()),
        tracestate: Some("vendor=value".into()),
    };
    let request_span = info_span!("app_server.request");
    assert!(set_parent_from_w3c_trace_context(
        &request_span,
        &request_parent
    ));

    let turn_context_item = async {
        let expected_trace_id = Span::current()
            .context()
            .span()
            .span_context()
            .trace_id()
            .to_string();
        let turn_context = session.new_default_turn().await;
        let turn_context_item = turn_context.to_turn_context_item();
        assert_eq!(turn_context_item.trace_id, Some(expected_trace_id));
        turn_context_item
    }
    .instrument(request_span)
    .await;

    assert_eq!(
        turn_context_item.trace_id.as_deref(),
        Some("00000000000000000000000000000011")
    );
}

#[test]
fn submission_dispatch_span_prefers_submission_trace_context() {
    let _trace_test_context = install_test_tracing("codex-core-tests");

    let ambient_parent = W3cTraceContext {
        traceparent: Some("00-00000000000000000000000000000033-0000000000000044-01".into()),
        tracestate: None,
    };
    let ambient_span = info_span!("ambient");
    assert!(set_parent_from_w3c_trace_context(
        &ambient_span,
        &ambient_parent
    ));

    let submission_trace = W3cTraceContext {
        traceparent: Some("00-00000000000000000000000000000055-0000000000000066-01".into()),
        tracestate: Some("vendor=value".into()),
    };
    let dispatch_span = ambient_span.in_scope(|| {
        submission_dispatch_span(&Submission {
            id: "sub-1".into(),
            op: Op::Interrupt,
            trace: Some(submission_trace),
            parent_turn_id: None,
        })
    });

    let trace_id = dispatch_span.context().span().span_context().trace_id();
    assert_eq!(
        trace_id,
        TraceId::from_hex("00000000000000000000000000000055").expect("trace id")
    );
}

#[test]
fn submission_dispatch_span_uses_debug_for_realtime_audio() {
    let _trace_test_context = install_test_tracing("codex-core-tests");

    let dispatch_span = submission_dispatch_span(&Submission {
        id: "sub-1".into(),
        op: Op::RealtimeConversationAudio(ConversationAudioParams {
            frame: RealtimeAudioFrame {
                data: "ZmFrZQ==".into(),
                sample_rate: 16_000,
                num_channels: 1,
                samples_per_channel: Some(160),
                item_id: None,
            },
        }),
        trace: None,
        parent_turn_id: None,
    });

    assert_eq!(
        dispatch_span.metadata().expect("span metadata").level(),
        &tracing::Level::DEBUG
    );
}

#[test]
fn op_kind_distinguishes_turn_ops() {
    assert_eq!(
        Op::OverrideTurnContext {
            cwd: None,
            approval_policy: None,
            approvals_reviewer: None,
            sandbox_policy: None,
            permission_profile: None,
            windows_sandbox_level: None,
            model: None,
            effort: None,
            summary: None,
            service_tier: None,
            collaboration_mode: None,
            personality: None,
        }
        .kind(),
        "override_turn_context"
    );
    assert_eq!(
        Op::UserInput {
            environments: None,
            items: vec![],
            final_output_json_schema: None,
            responsesapi_client_metadata: None,
        }
        .kind(),
        "user_input"
    );
    assert_eq!(
        Op::UserInputWithTurnContext {
            environments: None,
            items: vec![],
            final_output_json_schema: None,
            responsesapi_client_metadata: None,
            context_recall_selected_snippets: None,
            cwd: None,
            workspace_roots: None,
            profile_workspace_roots: None,
            approval_policy: None,
            approvals_reviewer: None,
            sandbox_policy: None,
            permission_profile: None,
            active_permission_profile: None,
            windows_sandbox_level: None,
            model: None,
            effort: None,
            summary: None,
            service_tier: None,
            collaboration_mode: None,
            personality: None,
        }
        .kind(),
        "user_input_with_turn_context"
    );
}

#[tokio::test]
async fn user_turn_updates_approvals_reviewer() {
    let (session, turn_context, _rx) = make_session_and_context_with_rx().await;
    let config = session.get_config().await;

    handlers::user_input_or_turn(
        &session,
        "sub-1".to_string(),
        Op::UserTurn {
            environments: None,
            items: vec![UserInput::Text {
                text: "hello".to_string(),
                text_elements: Vec::new(),
            }],
            cwd: config.cwd.to_path_buf(),
            approval_policy: config.permissions.approval_policy.value(),
            approvals_reviewer: Some(codex_config::types::ApprovalsReviewer::AutoReview),
            sandbox_policy: config.legacy_sandbox_policy(),
            permission_profile: None,
            model: turn_context.model_info.slug.clone(),
            effort: config.model_reasoning_effort,
            summary: config.model_reasoning_summary,
            service_tier: None,
            final_output_json_schema: None,
            collaboration_mode: None,
            personality: config.personality,
        },
    )
    .await;

    let state = session.state.lock().await;
    assert_eq!(
        state.session_configuration.approvals_reviewer,
        codex_config::types::ApprovalsReviewer::AutoReview
    );
}

#[tokio::test]
async fn turn_environments_set_primary_environment() {
    let (session, _turn_context, _rx) = make_session_and_context_with_rx().await;
    let selected_cwd =
        AbsolutePathBuf::try_from(session.get_config().await.cwd.as_path().join("selected"))
            .expect("absolute path");

    let turn_context = session
        .new_turn_with_sub_id(
            "sub-1".to_string(),
            SessionSettingsUpdate {
                environments: Some(vec![TurnEnvironmentSelection {
                    environment_id: "local".to_string(),
                    cwd: selected_cwd.clone().into(),
                }]),
                ..Default::default()
            },
        )
        .await
        .expect("turn should start");

    let turn_environments = &turn_context.environments;
    assert_eq!(turn_environments.turn_environments.len(), 1);
    let turn_environment = turn_context
        .environments
        .primary()
        .expect("primary environment should be set");
    assert!(std::sync::Arc::ptr_eq(
        &turn_environment.environment,
        &turn_environments.turn_environments[0].environment
    ));
    assert!(!turn_context.environments.turn_environments.is_empty());
    #[allow(deprecated)]
    let turn_cwd = turn_context.cwd.clone();
    assert_eq!(turn_cwd.as_path(), selected_cwd.as_path());
    assert_eq!(turn_context.config.cwd.as_path(), selected_cwd.as_path());
}

#[tokio::test]
async fn default_turn_overlays_session_cwd_onto_stored_thread_environments() {
    let (session, _turn_context, _rx) = make_session_and_context_with_rx().await;
    let session_cwd = session.get_config().await.cwd.clone();
    let selected_cwd =
        AbsolutePathBuf::try_from(session_cwd.as_path().join("selected")).expect("absolute path");

    {
        let mut state = session.state.lock().await;
        state.session_configuration.environments = vec![TurnEnvironmentSelection {
            environment_id: "local".to_string(),
            cwd: selected_cwd.clone().into(),
        }];
    }

    let turn_context = session.new_default_turn().await;

    let turn_environments = &turn_context.environments;
    assert_eq!(turn_environments.turn_environments.len(), 1);
    let turn_environment = turn_context
        .environments
        .primary()
        .expect("primary environment should be set");
    assert!(std::sync::Arc::ptr_eq(
        &turn_environment.environment,
        &turn_environments.turn_environments[0].environment
    ));
    #[allow(deprecated)]
    let turn_cwd = turn_context.cwd.clone();
    assert_eq!(turn_cwd, session_cwd);
    assert_eq!(turn_context.config.cwd, session_cwd);
}

#[tokio::test]
async fn default_turn_honors_empty_stored_thread_environments() {
    let (session, _turn_context, _rx) = make_session_and_context_with_rx().await;
    let session_cwd = session.get_config().await.cwd.clone();

    {
        let mut state = session.state.lock().await;
        state.session_configuration.environments = Vec::new();
    }

    let turn_context = session.new_default_turn().await;

    assert!(turn_context.environments.primary().is_none());
    assert!(turn_context.environments.turn_environments.is_empty());
    #[allow(deprecated)]
    let turn_cwd = turn_context.cwd.clone();
    assert_eq!(turn_cwd, session_cwd);
    assert_eq!(turn_context.config.cwd, session_cwd);
    assert_eq!(turn_context.environments.turn_environments.len(), 0);
}

#[tokio::test]
async fn primary_environment_uses_first_turn_environment() {
    let (_session, mut turn_context) = make_session_and_context().await;
    let first_environment = turn_context.environments.turn_environments[0].clone();
    #[allow(deprecated)]
    let second_cwd = turn_context.cwd.join("second");
    turn_context
        .environments
        .turn_environments
        .push(TurnEnvironment {
            environment_id: "second".to_string(),
            environment: Arc::clone(&first_environment.environment),
            cwd: second_cwd.clone().into(),
            shell: None,
        });

    assert_eq!(
        turn_context
            .environments
            .primary()
            .expect("primary environment")
            .environment_id,
        first_environment.environment_id
    );
    assert_eq!(
        turn_context
            .environments
            .turn_environments
            .iter()
            .find(|environment| environment.environment_id == "second")
            .expect("second environment")
            .cwd,
        second_cwd.clone().into()
    );
    assert_eq!(turn_context.environments.turn_environments.len(), 2);
    assert_eq!(
        turn_context.environments.turn_environments[1].cwd,
        second_cwd.into()
    );
}

#[tokio::test]
async fn empty_turn_environments_clear_primary_environment() {
    let (session, _turn_context, _rx) = make_session_and_context_with_rx().await;

    let turn_context = session
        .new_turn_with_sub_id(
            "sub-1".to_string(),
            SessionSettingsUpdate {
                environments: Some(vec![]),
                ..Default::default()
            },
        )
        .await
        .expect("turn should start");

    assert!(turn_context.environments.primary().is_none());
    assert!(turn_context.environments.turn_environments.is_empty());
    #[allow(deprecated)]
    let turn_cwd = turn_context.cwd.clone();
    assert_eq!(turn_cwd, session.get_config().await.cwd);
    assert_eq!(turn_context.config.cwd, session.get_config().await.cwd);
}

#[tokio::test]
async fn unknown_turn_environment_returns_error() {
    let (session, _turn_context, _rx) = make_session_and_context_with_rx().await;
    let original_configuration = {
        let state = session.state.lock().await;
        state.session_configuration.clone()
    };

    let err = session
        .new_turn_with_sub_id(
            "sub-1".to_string(),
            SessionSettingsUpdate {
                environments: Some(vec![TurnEnvironmentSelection {
                    environment_id: "missing".to_string(),
                    cwd: original_configuration.cwd.clone().into(),
                }]),
                ..Default::default()
            },
        )
        .await
        .expect_err("unknown environment should fail");

    let current_configuration = {
        let state = session.state.lock().await;
        state.session_configuration.clone()
    };
    assert!(matches!(err, CodexErr::InvalidRequest(_)));
    assert!(err.to_string().contains("missing"));
    assert_eq!(current_configuration.cwd, original_configuration.cwd);
    assert_eq!(
        current_configuration.environments,
        original_configuration.environments
    );
}

#[tokio::test]
async fn duplicate_turn_environment_returns_error_without_mutating_session() {
    let (session, _turn_context, _rx) = make_session_and_context_with_rx().await;
    let original_configuration = {
        let state = session.state.lock().await;
        state.session_configuration.clone()
    };

    let err = session
        .new_turn_with_sub_id(
            "sub-1".to_string(),
            SessionSettingsUpdate {
                environments: Some(vec![
                    TurnEnvironmentSelection {
                        environment_id: "local".to_string(),
                        cwd: original_configuration.cwd.clone().into(),
                    },
                    TurnEnvironmentSelection {
                        environment_id: "local".to_string(),
                        cwd: original_configuration.cwd.join("second").into(),
                    },
                ]),
                ..Default::default()
            },
        )
        .await
        .expect_err("duplicate environment should fail");

    let current_configuration = {
        let state = session.state.lock().await;
        state.session_configuration.clone()
    };
    assert!(matches!(err, CodexErr::InvalidRequest(_)));
    assert!(err.to_string().contains("duplicate"));
    assert_eq!(current_configuration.cwd, original_configuration.cwd);
    assert_eq!(
        current_configuration.environments,
        original_configuration.environments
    );
}

#[tokio::test]
async fn spawn_task_turn_span_inherits_dispatch_trace_context() {
    struct TraceCaptureTask {
        captured_trace: Arc<std::sync::Mutex<Option<W3cTraceContext>>>,
    }

    impl SessionTask for TraceCaptureTask {
        fn kind(&self) -> TaskKind {
            TaskKind::Regular
        }

        fn span_name(&self) -> &'static str {
            "session_task.trace_capture"
        }

        async fn run(
            self: Arc<Self>,
            _session: Arc<SessionTaskContext>,
            _ctx: Arc<TurnContext>,
            _input: Vec<UserInput>,
            _cancellation_token: CancellationToken,
        ) -> Option<String> {
            let mut trace = self
                .captured_trace
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            *trace = current_span_w3c_trace_context();
            None
        }
    }

    let _trace_test_context = install_test_tracing("codex-core-tests");

    let request_parent = W3cTraceContext {
        traceparent: Some("00-00000000000000000000000000000011-0000000000000022-01".into()),
        tracestate: Some("vendor=value".into()),
    };
    let request_span = tracing::info_span!("app_server.request");
    assert!(set_parent_from_w3c_trace_context(
        &request_span,
        &request_parent
    ));

    let submission_trace =
        async { current_span_w3c_trace_context().expect("request span should have trace context") }
            .instrument(request_span)
            .await;

    let dispatch_span = submission_dispatch_span(&Submission {
        id: "sub-1".into(),
        op: Op::Interrupt,
        trace: Some(submission_trace.clone()),
        parent_turn_id: None,
    });
    let dispatch_span_id = dispatch_span.context().span().span_context().span_id();

    let (sess, tc, rx) = make_session_and_context_with_rx().await;
    let captured_trace = Arc::new(std::sync::Mutex::new(None));

    async {
        sess.spawn_task(
            Arc::clone(&tc),
            vec![UserInput::Text {
                text: "hello".to_string(),
                text_elements: Vec::new(),
            }],
            TraceCaptureTask {
                captured_trace: Arc::clone(&captured_trace),
            },
        )
        .await;
    }
    .instrument(dispatch_span)
    .await;

    let evt = tokio::time::timeout(StdDuration::from_secs(2), rx.recv())
        .await
        .expect("timeout waiting for turn completion")
        .expect("event");
    assert!(matches!(evt.msg, EventMsg::TurnComplete(_)));

    let task_trace = captured_trace
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .clone()
        .expect("turn task should capture the current span trace context");
    let submission_context =
        codex_otel::context_from_w3c_trace_context(&submission_trace).expect("submission");
    let task_context = codex_otel::context_from_w3c_trace_context(&task_trace).expect("task trace");

    assert_eq!(
        task_context.span().span_context().trace_id(),
        submission_context.span().span_context().trace_id()
    );
    assert_ne!(
        task_context.span().span_context().span_id(),
        dispatch_span_id
    );
}

#[cfg(debug_assertions)]
#[tokio::test]
async fn shutdown_complete_does_not_append_to_thread_store_after_shutdown() {
    let (mut session, _turn_context) = make_session_and_context().await;
    let store = Arc::new(codex_thread_store::InMemoryThreadStore::default());
    let thread_store: Arc<dyn codex_thread_store::ThreadStore> = store.clone();
    let config = session.get_config().await;
    let live_thread = LiveThread::create(
        Arc::clone(&thread_store),
        CreateThreadParams {
            thread_id: session.conversation_id,
            forked_from_id: None,
            source: SessionSource::Exec,
            thread_source: None,
            base_instructions: BaseInstructions::default(),
            dynamic_tools: Vec::new(),
            history_mode: ThreadHistoryMode::Legacy,
            metadata: ThreadPersistenceMetadata {
                cwd: Some(config.cwd.to_path_buf()),
                model_provider: config.model_provider_id.clone(),
                memory_mode: if config.memories.generate_memories {
                    ThreadMemoryMode::Enabled
                } else {
                    ThreadMemoryMode::Disabled
                },
            },
            event_persistence_mode: ThreadEventPersistenceMode::Limited,
        },
    )
    .await
    .expect("create thread persistence");
    session.services.thread_store = thread_store;
    session.services.live_thread = Some(live_thread);
    let session = Arc::new(session);

    assert!(handlers::shutdown(&session, "sub-1".to_string()).await);

    assert_eq!(
        codex_thread_store::InMemoryThreadStoreCalls {
            create_thread: 1,
            shutdown_thread: 1,
            ..Default::default()
        },
        store.calls().await
    );
}

#[tokio::test]
async fn submission_loop_channel_close_emits_thread_stop_lifecycle() {
    struct SessionStopMarker;
    struct ThreadStopMarker;

    struct ThreadStopRecorder {
        calls: Arc<std::sync::atomic::AtomicUsize>,
        expected_thread_id: ThreadId,
    }

    impl codex_extension_api::ThreadLifecycleContributor<crate::config::Config> for ThreadStopRecorder {
        fn on_thread_stop(&self, input: codex_extension_api::ThreadStopInput<'_>) {
            assert_eq!(
                self.expected_thread_id.to_string(),
                input.thread_store.level_id()
            );
            assert!(input.session_store.get::<SessionStopMarker>().is_some());
            assert!(input.thread_store.get::<ThreadStopMarker>().is_some());
            self.calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        }
    }

    let (mut session, turn_context) = make_session_and_context().await;
    let calls = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let mut builder = codex_extension_api::ExtensionRegistryBuilder::<crate::config::Config>::new();
    builder.thread_lifecycle_contributor(Arc::new(ThreadStopRecorder {
        calls: Arc::clone(&calls),
        expected_thread_id: session.conversation_id,
    }));
    session.services.extensions = Arc::new(builder.build());
    session
        .services
        .session_extension_data
        .insert(SessionStopMarker);
    session
        .services
        .thread_extension_data
        .insert(ThreadStopMarker);

    let (tx_sub, rx_sub) = async_channel::bounded(1);
    drop(tx_sub);
    let session = Arc::new(session);
    submission_loop(session, Arc::clone(&turn_context.config), rx_sub).await;

    assert_eq!(1, calls.load(std::sync::atomic::Ordering::SeqCst));
}

#[tokio::test]
async fn submission_loop_channel_close_aborts_active_turn_before_thread_stop_lifecycle() {
    struct LifecycleRecorder {
        calls: Arc<std::sync::Mutex<Vec<&'static str>>>,
        expected_thread_id: ThreadId,
        expected_turn_id: String,
    }

    impl codex_extension_api::ThreadLifecycleContributor<crate::config::Config> for LifecycleRecorder {
        fn on_thread_stop(&self, input: codex_extension_api::ThreadStopInput<'_>) {
            assert_eq!(
                self.expected_thread_id.to_string(),
                input.thread_store.level_id()
            );
            self.calls
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .push("thread_stop");
        }
    }

    impl codex_extension_api::TurnLifecycleContributor for LifecycleRecorder {
        fn on_turn_abort(&self, input: codex_extension_api::TurnAbortInput<'_>) {
            assert_eq!(
                self.expected_thread_id.to_string(),
                input.thread_store.level_id()
            );
            assert_eq!(self.expected_turn_id, input.turn_store.level_id());
            assert_eq!(TurnAbortReason::Interrupted, input.reason);
            self.calls
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .push("turn_abort");
        }
    }

    let (mut session, turn_context) = make_session_and_context().await;
    let calls = Arc::new(std::sync::Mutex::new(Vec::new()));
    let recorder = Arc::new(LifecycleRecorder {
        calls: Arc::clone(&calls),
        expected_thread_id: session.conversation_id,
        expected_turn_id: turn_context.sub_id.clone(),
    });
    let mut builder = codex_extension_api::ExtensionRegistryBuilder::<crate::config::Config>::new();
    builder.thread_lifecycle_contributor(recorder.clone());
    builder.turn_lifecycle_contributor(recorder);
    session.services.extensions = Arc::new(builder.build());

    let session = Arc::new(session);
    session
        .spawn_task(
            Arc::new(turn_context),
            Vec::new(),
            NeverEndingTask {
                kind: TaskKind::Regular,
                listen_to_cancellation_token: true,
            },
        )
        .await;

    let (tx_sub, rx_sub) = async_channel::bounded(1);
    drop(tx_sub);
    submission_loop(Arc::clone(&session), session.get_config().await, rx_sub).await;

    assert_eq!(
        vec!["turn_abort", "thread_stop"],
        *calls
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    );
}

#[tokio::test]
async fn shutdown_and_wait_allows_multiple_waiters() {
    let (session, _turn_context) = make_session_and_context().await;
    let (tx_sub, rx_sub) = async_channel::bounded(4);
    let (_tx_event, rx_event) = async_channel::unbounded();
    let (_agent_status_tx, agent_status) = watch::channel(AgentStatus::PendingInit);
    let session_loop_handle = tokio::spawn(async move {
        let shutdown: Submission = rx_sub.recv().await.expect("shutdown submission");
        assert_eq!(shutdown.op, Op::Shutdown);
        tokio::time::sleep(StdDuration::from_millis(50)).await;
    });
    let codex = Arc::new(Codex {
        tx_sub,
        rx_event,
        agent_status,
        session: Arc::new(session),
        session_loop_termination: session_loop_termination_from_handle(session_loop_handle),
    });

    let waiter_1 = {
        let codex = Arc::clone(&codex);
        tokio::spawn(async move { codex.shutdown_and_wait().await })
    };
    let waiter_2 = {
        let codex = Arc::clone(&codex);
        tokio::spawn(async move { codex.shutdown_and_wait().await })
    };

    waiter_1
        .await
        .expect("first shutdown waiter join")
        .expect("first shutdown waiter");
    waiter_2
        .await
        .expect("second shutdown waiter join")
        .expect("second shutdown waiter");
}

#[tokio::test]
async fn shutdown_and_wait_waits_when_shutdown_is_already_in_progress() {
    let (session, _turn_context) = make_session_and_context().await;
    let (tx_sub, rx_sub) = async_channel::bounded(4);
    drop(rx_sub);
    let (_tx_event, rx_event) = async_channel::unbounded();
    let (_agent_status_tx, agent_status) = watch::channel(AgentStatus::PendingInit);
    let (shutdown_complete_tx, shutdown_complete_rx) = tokio::sync::oneshot::channel();
    let session_loop_handle = tokio::spawn(async move {
        let _ = shutdown_complete_rx.await;
    });
    let codex = Arc::new(Codex {
        tx_sub,
        rx_event,
        agent_status,
        session: Arc::new(session),
        session_loop_termination: session_loop_termination_from_handle(session_loop_handle),
    });

    let waiter = {
        let codex = Arc::clone(&codex);
        tokio::spawn(async move { codex.shutdown_and_wait().await })
    };

    tokio::time::sleep(StdDuration::from_millis(10)).await;
    assert!(!waiter.is_finished());

    shutdown_complete_tx
        .send(())
        .expect("session loop should still be waiting to terminate");

    waiter
        .await
        .expect("shutdown waiter join")
        .expect("shutdown waiter");
}

#[tokio::test]
async fn shutdown_and_wait_shuts_down_cached_guardian_subagent() {
    let (parent_session, parent_turn_context) = make_session_and_context().await;
    let parent_session = Arc::new(parent_session);
    let parent_config = Arc::clone(&parent_turn_context.config);
    let (parent_tx_sub, parent_rx_sub) = async_channel::bounded(4);
    let (_parent_tx_event, parent_rx_event) = async_channel::unbounded();
    let (_parent_status_tx, parent_agent_status) = watch::channel(AgentStatus::PendingInit);
    let parent_session_for_loop = Arc::clone(&parent_session);
    let parent_session_loop_handle = tokio::spawn(async move {
        submission_loop(parent_session_for_loop, parent_config, parent_rx_sub).await;
    });
    let parent_codex = Codex {
        tx_sub: parent_tx_sub,
        rx_event: parent_rx_event,
        agent_status: parent_agent_status,
        session: Arc::clone(&parent_session),
        session_loop_termination: session_loop_termination_from_handle(parent_session_loop_handle),
    };

    let (child_session, _child_turn_context) = make_session_and_context().await;
    let (child_tx_sub, child_rx_sub) = async_channel::bounded(4);
    let (_child_tx_event, child_rx_event) = async_channel::unbounded();
    let (_child_status_tx, child_agent_status) = watch::channel(AgentStatus::PendingInit);
    let (child_shutdown_tx, child_shutdown_rx) = tokio::sync::oneshot::channel();
    let child_session_loop_handle = tokio::spawn(async move {
        let shutdown: Submission = child_rx_sub
            .recv()
            .await
            .expect("child shutdown submission");
        assert_eq!(shutdown.op, Op::Shutdown);
        child_shutdown_tx
            .send(())
            .expect("child shutdown signal should be delivered");
    });
    let child_codex = Codex {
        tx_sub: child_tx_sub,
        rx_event: child_rx_event,
        agent_status: child_agent_status,
        session: Arc::new(child_session),
        session_loop_termination: session_loop_termination_from_handle(child_session_loop_handle),
    };
    parent_session
        .guardian_review_session
        .cache_for_test(child_codex)
        .await;

    parent_codex
        .shutdown_and_wait()
        .await
        .expect("parent shutdown should succeed");

    child_shutdown_rx
        .await
        .expect("guardian subagent should receive a shutdown op");
}

#[tokio::test]
async fn cached_guardian_subagent_exposes_its_rollout_path() {
    let (parent_session, _parent_turn_context) = make_session_and_context().await;
    let parent_session = Arc::new(parent_session);

    let (mut child_session, _child_turn_context) = make_session_and_context().await;
    let child_rollout_path = attach_thread_persistence(&mut child_session).await;
    let (child_tx_sub, _child_rx_sub) = async_channel::bounded(4);
    let (_child_tx_event, child_rx_event) = async_channel::unbounded();
    let (_child_status_tx, child_agent_status) = watch::channel(AgentStatus::PendingInit);
    let child_session_loop_handle = tokio::spawn(async {});
    let child_codex = Codex {
        tx_sub: child_tx_sub,
        rx_event: child_rx_event,
        agent_status: child_agent_status,
        session: Arc::new(child_session),
        session_loop_termination: session_loop_termination_from_handle(child_session_loop_handle),
    };
    parent_session
        .guardian_review_session
        .cache_for_test(child_codex)
        .await;

    assert_eq!(
        parent_session
            .guardian_review_session
            .trunk_rollout_path()
            .await,
        Some(child_rollout_path)
    );
}

#[tokio::test]
async fn shutdown_and_wait_shuts_down_tracked_ephemeral_guardian_review() {
    let (parent_session, parent_turn_context) = make_session_and_context().await;
    let parent_session = Arc::new(parent_session);
    let parent_config = Arc::clone(&parent_turn_context.config);
    let (parent_tx_sub, parent_rx_sub) = async_channel::bounded(4);
    let (_parent_tx_event, parent_rx_event) = async_channel::unbounded();
    let (_parent_status_tx, parent_agent_status) = watch::channel(AgentStatus::PendingInit);
    let parent_session_for_loop = Arc::clone(&parent_session);
    let parent_session_loop_handle = tokio::spawn(async move {
        submission_loop(parent_session_for_loop, parent_config, parent_rx_sub).await;
    });
    let parent_codex = Codex {
        tx_sub: parent_tx_sub,
        rx_event: parent_rx_event,
        agent_status: parent_agent_status,
        session: Arc::clone(&parent_session),
        session_loop_termination: session_loop_termination_from_handle(parent_session_loop_handle),
    };

    let (child_session, _child_turn_context) = make_session_and_context().await;
    let (child_tx_sub, child_rx_sub) = async_channel::bounded(4);
    let (_child_tx_event, child_rx_event) = async_channel::unbounded();
    let (_child_status_tx, child_agent_status) = watch::channel(AgentStatus::PendingInit);
    let (child_shutdown_tx, child_shutdown_rx) = tokio::sync::oneshot::channel();
    let child_session_loop_handle = tokio::spawn(async move {
        let shutdown: Submission = child_rx_sub
            .recv()
            .await
            .expect("child shutdown submission");
        assert_eq!(shutdown.op, Op::Shutdown);
        child_shutdown_tx
            .send(())
            .expect("child shutdown signal should be delivered");
    });
    let child_codex = Codex {
        tx_sub: child_tx_sub,
        rx_event: child_rx_event,
        agent_status: child_agent_status,
        session: Arc::new(child_session),
        session_loop_termination: session_loop_termination_from_handle(child_session_loop_handle),
    };
    parent_session
        .guardian_review_session
        .register_ephemeral_for_test(child_codex)
        .await;

    parent_codex
        .shutdown_and_wait()
        .await
        .expect("parent shutdown should succeed");

    child_shutdown_rx
        .await
        .expect("ephemeral guardian review should receive a shutdown op");
}

async fn make_session_and_context_with_auth_and_config_and_rx<F>(
    auth: CodexAuth,
    dynamic_tools: Vec<DynamicToolSpec>,
    configure_config: F,
) -> (
    Arc<Session>,
    Arc<TurnContext>,
    async_channel::Receiver<Event>,
)
where
    F: FnOnce(&mut Config),
{
    let codex_home = tempfile::tempdir().expect("create temp dir");
    make_session_and_context_with_auth_config_home_and_rx(
        auth,
        dynamic_tools,
        codex_home.path(),
        configure_config,
    )
    .await
}

async fn make_session_and_context_with_auth_config_home_and_rx<F>(
    auth: CodexAuth,
    dynamic_tools: Vec<DynamicToolSpec>,
    codex_home: &Path,
    configure_config: F,
) -> (
    Arc<Session>,
    Arc<TurnContext>,
    async_channel::Receiver<Event>,
)
where
    F: FnOnce(&mut Config),
{
    let (tx_event, rx_event) = async_channel::unbounded();
    let mut config = build_test_config(codex_home).await;
    configure_config(&mut config);
    let state_db = if config.features.enabled(Feature::Goals) {
        Some(
            codex_state::StateRuntime::init(
                config.sqlite_home.clone(),
                config.model_provider_id.clone(),
            )
            .await
            .expect("goal tests should initialize sqlite state db"),
        )
    } else {
        None
    };
    let config = Arc::new(config);
    let thread_id = ThreadId::default();
    let auth_manager = AuthManager::from_auth_for_testing(auth);
    let models_manager = models_manager_with_provider(
        config.codex_home.to_path_buf(),
        auth_manager.clone(),
        config.model_provider.clone(),
    );
    let agent_control = AgentControl::default();
    let exec_policy = Arc::new(ExecPolicyManager::default());
    let (agent_status_tx, _agent_status_rx) = watch::channel(AgentStatus::PendingInit);
    let model = get_model_offline_for_tests(config.model.as_deref());
    let model_info =
        construct_model_info_offline_for_tests(model.as_str(), &config.to_models_manager_config());
    let reasoning_effort = config.model_reasoning_effort;
    let collaboration_mode = CollaborationMode {
        mode: ModeKind::Default,
        settings: Settings {
            model,
            reasoning_effort,
            developer_instructions: None,
        },
    };
    let default_environments = vec![TurnEnvironmentSelection {
        environment_id: codex_exec_server::LOCAL_ENVIRONMENT_ID.to_string(),
        cwd: config.cwd.clone().into(),
    }];
    let session_configuration = SessionConfiguration {
        provider: config.model_provider.clone(),
        collaboration_mode,
        model_reasoning_summary: config.model_reasoning_summary,
        developer_instructions: config.developer_instructions.clone(),
        user_instructions: config.user_instructions.clone(),
        service_tier: None,
        personality: config.personality,
        history_mode: ThreadHistoryMode::Legacy,
        base_instructions: config
            .base_instructions
            .clone()
            .unwrap_or_else(|| model_info.get_model_instructions(config.personality)),
        compact_prompt: config.compact_prompt.clone(),
        approval_policy: config.permissions.approval_policy.clone(),
        approvals_reviewer: config.approvals_reviewer,
        permission_profile_state: config.permissions.permission_profile_state().clone(),
        windows_sandbox_level: WindowsSandboxLevel::from_config(&config),
        cwd: config.cwd.clone(),
        workspace_roots: config.workspace_roots.clone(),
        codex_home: config.codex_home.clone(),
        thread_name: None,
        environments: default_environments,
        original_config_do_not_use: Arc::clone(&config),
        metrics_service_name: None,
        app_server_client_name: None,
        app_server_client_version: None,
        session_source: SessionSource::Exec,
        thread_source: None,
        dynamic_tools,
        persist_extended_history: false,
        inherited_shell_snapshot: None,
        user_shell_override: None,
    };
    let per_turn_config =
        Session::build_per_turn_config(&session_configuration, session_configuration.cwd.clone());
    let model_info = construct_model_info_offline_for_tests(
        session_configuration.collaboration_mode.model(),
        &per_turn_config.to_models_manager_config(),
    );
    let session_telemetry = session_telemetry(
        thread_id,
        config.as_ref(),
        &model_info,
        session_configuration.session_source.clone(),
    );

    let state = SessionState::new(session_configuration.clone());
    let plugins_manager = Arc::new(PluginsManager::new(config.codex_home.to_path_buf()));
    let mcp_manager = Arc::new(McpManager::new(Arc::clone(&plugins_manager)));
    let skills_manager = Arc::new(SkillsManager::new(
        config.codex_home.clone(),
        /*bundled_skills_enabled*/ true,
    ));
    let network_approval = Arc::new(NetworkApprovalService::default());
    let environment = Arc::new(
        codex_exec_server::Environment::create_for_tests(/*exec_server_url*/ None)
            .expect("create environment"),
    );

    let services = SessionServices {
        mcp_connection_manager: Arc::new(RwLock::new(
            crate::state::PublishedMcpConnectionManager::new(
                McpConnectionManager::new_uninitialized_with_permission_profile(
                    &config.permissions.approval_policy,
                    config.permissions.permission_profile(),
                ),
                crate::state::FrozenMcpAuthSnapshot::capture(auth_manager.as_ref())
                    .await
                    .expect("capture MCP auth snapshot")
                    .binding(),
                codex_protocol::protocol::McpElicitationAuthority {
                    approval_policy: config.permissions.approval_policy.value(),
                    permission_profile: config.permissions.effective_permission_profile(),
                    approvals_reviewer: config.approvals_reviewer,
                    apps_approvals_reviewers: Default::default(),
                },
            ),
        )),
        mcp_startup_cancellation_token: Mutex::new(CancellationToken::new()),
        unified_exec_manager: UnifiedExecProcessManager::new(
            config.background_terminal_max_timeout,
        ),
        shell_zsh_path: None,
        main_execve_wrapper_exe: config.main_execve_wrapper_exe.clone(),
        analytics_events_client: AnalyticsEventsClient::new(
            Arc::clone(&auth_manager),
            config.chatgpt_base_url.trim_end_matches('/').to_string(),
            config.analytics_enabled,
        ),
        hooks: arc_swap::ArcSwap::from_pointee(Hooks::new(HooksConfig {
            legacy_notify_argv: config.notify.clone(),
            ..HooksConfig::default()
        })),
        rollout_thread_trace: codex_rollout_trace::ThreadTraceContext::disabled(),
        user_shell: Arc::new(default_user_shell()),
        shell_snapshot_tx: watch::channel(None).0,
        show_raw_agent_reasoning: config.show_raw_agent_reasoning,
        exec_policy,
        auth_manager: Arc::clone(&auth_manager),
        openai_file_upload_client_pool:
            codex_http_client::RouteAwareClientPool::new_without_request_logging(
                config.http_client_factory(),
                codex_http_client::ClientRouteClass::Api,
            )
            .with_legacy_custom_ca_fallback(),
        session_telemetry: session_telemetry.clone(),
        models_manager: Arc::clone(&models_manager),
        tool_approvals: Mutex::new(ApprovalStore::default()),
        guardian_rejections: Mutex::new(std::collections::HashMap::new()),
        guardian_rejection_circuit_breaker: Mutex::new(Default::default()),
        runtime_handle: tokio::runtime::Handle::current(),
        skills_manager,
        plugins_manager,
        mcp_manager,
        extensions: Arc::new(codex_extension_api::ExtensionRegistryBuilder::new().build()),
        session_extension_data: codex_extension_api::ExtensionData::new(
            agent_control.session_id().to_string(),
        ),
        thread_extension_data: codex_extension_api::ExtensionData::new(thread_id.to_string()),
        agent_control,
        network_proxy: None,
        network_approval: Arc::clone(&network_approval),
        state_db: state_db.clone(),
        live_thread: None,
        thread_store: Arc::new(codex_thread_store::LocalThreadStore::new(
            codex_thread_store::LocalThreadStoreConfig::from_config(config.as_ref()),
            state_db,
        )),
        attestation_provider: None,
        model_client: ModelClient::new(
            Some(Arc::clone(&auth_manager)),
            thread_id.into(),
            thread_id,
            /*installation_id*/ "11111111-1111-4111-8111-111111111111".to_string(),
            session_configuration.provider.clone(),
            session_configuration.session_source.clone(),
            config.model_verbosity,
            config.features.enabled(Feature::EnableRequestCompression),
            config.features.enabled(Feature::RuntimeMetrics),
            Session::build_model_client_beta_features_header(config.as_ref()),
            /*attestation_provider*/ None,
        ),
        code_mode_service: crate::tools::code_mode::CodeModeService::new(),
        environment_manager: Arc::new(codex_exec_server::EnvironmentManager::default_for_tests()),
    };

    let plugin_outcome = services
        .plugins_manager
        .plugins_for_config(&per_turn_config.plugins_config_input())
        .await;
    let effective_skill_roots = plugin_outcome.effective_plugin_skill_roots();
    let skills_input =
        crate::skills_load_input_from_config(&per_turn_config, effective_skill_roots);
    let skill_fs = environment.get_filesystem();
    let skills_outcome = Arc::new(
        services
            .skills_manager
            .skills_for_config(&skills_input, Some(Arc::clone(&skill_fs)))
            .await,
    );
    let turn_environments = turn_environments_for_tests(&environment, &session_configuration.cwd);
    let turn_context = Arc::new(
        Session::make_turn_context(
            thread_id,
            SessionId::from(thread_id),
            Some(Arc::clone(&auth_manager)),
            &session_telemetry,
            session_configuration.provider.clone(),
            &session_configuration,
            services.user_shell.as_ref(),
            services.shell_zsh_path.as_ref(),
            services.main_execve_wrapper_exe.as_ref(),
            per_turn_config,
            model_info,
            &models_manager,
            /*network*/ None,
            turn_environments,
            session_configuration.cwd.clone(),
            "turn_id".to_string(),
            skills_outcome,
            /*goal_tools_supported*/ true,
        )
        .await,
    );

    let (mailbox, mailbox_rx) = crate::agent::Mailbox::new();
    let session = Arc::new(Session {
        conversation_id: thread_id,
        installation_id: "11111111-1111-4111-8111-111111111111".to_string(),
        tx_event,
        agent_status: agent_status_tx,
        out_of_band_elicitation_paused: watch::channel(false).0,
        state: Mutex::new(state),
        managed_network_proxy_refresh_lock: Semaphore::new(/*permits*/ 1),
        features: config.features.clone(),
        mcp_server_refresh_lock: Semaphore::new(/*permits*/ 1),
        mcp_server_refresh_state: Mutex::new(super::session::McpServerRefreshState::default()),
        mcp_server_refresh_test_gate: Mutex::new(None),
        mcp_server_refresh_publication_test_gate: Mutex::new(None),
        mcp_server_refresh_retry_test_notify: Mutex::new(None),
        conversation: Arc::new(RealtimeConversationManager::new()),
        active_turn: Mutex::new(None),
        mailbox,
        mailbox_rx: Mutex::new(mailbox_rx),
        idle_pending_input: Mutex::new(Vec::new()),
        goal_runtime: crate::goals::GoalRuntimeState::new(),
        guardian_review_session: crate::guardian::GuardianReviewSessionManager::default(),
        services,
        next_internal_sub_id: AtomicU64::new(0),
    });

    (session, turn_context, rx_event)
}

pub(crate) async fn make_session_and_context_with_dynamic_tools_and_rx(
    dynamic_tools: Vec<DynamicToolSpec>,
) -> (
    Arc<Session>,
    Arc<TurnContext>,
    async_channel::Receiver<Event>,
) {
    make_session_and_context_with_auth_and_config_and_rx(
        CodexAuth::from_api_key("Test API Key"),
        dynamic_tools,
        |_config| {},
    )
    .await
}

async fn make_goal_session_and_context_with_rx() -> (
    Arc<Session>,
    Arc<TurnContext>,
    async_channel::Receiver<Event>,
    tempfile::TempDir,
) {
    let codex_home = tempfile::tempdir().expect("create temp dir");
    let (session, turn_context, rx) = make_session_and_context_with_auth_config_home_and_rx(
        CodexAuth::from_api_key("Test API Key"),
        Vec::new(),
        codex_home.path(),
        |config| {
            config
                .features
                .enable(Feature::Goals)
                .expect("goal mode should be enableable in tests");
        },
    )
    .await;
    upsert_goal_test_thread(session.as_ref()).await;
    (session, turn_context, rx, codex_home)
}

async fn upsert_goal_test_thread(session: &Session) {
    let config = session.get_config().await;
    let state_db = session
        .state_db()
        .expect("goal test session should have a state db");
    let mut builder = codex_state::ThreadMetadataBuilder::new(
        session.conversation_id,
        config
            .codex_home
            .join("goal-test-rollout.jsonl")
            .to_path_buf(),
        chrono::Utc::now(),
        SessionSource::Cli,
    );
    builder.cwd = config.cwd.to_path_buf();
    builder.model_provider = Some(config.model_provider_id.clone());
    let metadata = builder.build(config.model_provider_id.as_str());
    state_db
        .upsert_thread(&metadata)
        .await
        .expect("goal test thread should be upserted");
}

#[tokio::test]
async fn item_completion_without_start_uses_completion_timestamp() {
    let (session, turn_context, rx) = make_session_and_context_with_rx().await;
    let item = TurnItem::UserMessage(codex_protocol::items::UserMessageItem {
        id: "missing-start".to_string(),
        content: Vec::new(),
    });

    session.emit_turn_item_completed(&turn_context, item).await;

    let completed = rx.recv().await.expect("completed item event");
    let EventMsg::ItemCompleted(event) = completed.msg else {
        panic!("expected completed item event");
    };
    assert_eq!(event.started_at_ms, Some(event.completed_at_ms));
}

// Like make_session_and_context, but returns Arc<Session> and the event receiver
// so tests can assert on emitted events.
pub(crate) async fn make_session_and_context_with_rx() -> (
    Arc<Session>,
    Arc<TurnContext>,
    async_channel::Receiver<Event>,
) {
    make_session_and_context_with_dynamic_tools_and_rx(Vec::new()).await
}

#[tokio::test]
async fn explicit_refresh_rebuilds_mcp_connections_on_each_next_turn() {
    let (session, turn_context, _rx) = make_session_and_context_with_rx().await;
    let old_token = session.mcp_startup_cancellation_token().await;
    assert!(!old_token.is_cancelled());

    let mcp_oauth_credentials_store_mode =
        serde_json::to_value(OAuthCredentialsStoreMode::Auto).expect("serialize store mode");
    let refresh_config = McpServerRefreshConfig {
        mcp_servers: json!({}),
        mcp_oauth_credentials_store_mode,
        elicitation_authority: None,
    };
    super::handlers::refresh_mcp_servers(&session, refresh_config.clone()).await;

    assert!(!old_token.is_cancelled());
    assert!(
        session
            .mcp_server_refresh_state
            .lock()
            .await
            .pending
            .is_some()
    );

    session
        .refresh_mcp_servers_if_requested(&turn_context, /*elicitation_reviewer*/ None)
        .await;

    assert!(old_token.is_cancelled());
    assert!(
        session
            .mcp_server_refresh_state
            .lock()
            .await
            .pending
            .is_none()
    );
    let new_token = session.mcp_startup_cancellation_token().await;
    assert!(!new_token.is_cancelled());

    super::handlers::refresh_mcp_servers(&session, refresh_config).await;
    session
        .refresh_mcp_servers_if_requested(&turn_context, /*elicitation_reviewer*/ None)
        .await;

    assert!(new_token.is_cancelled());
    let newest_token = session.mcp_startup_cancellation_token().await;
    assert!(!newest_token.is_cancelled());
}

#[tokio::test]
async fn explicit_refresh_publishes_current_mcp_elicitation_authority() {
    let (session, old_turn, _rx) = make_session_and_context_with_rx().await;
    assert_ne!(old_turn.approval_policy.value(), AskForApproval::Never);
    assert_eq!(old_turn.config.approvals_reviewer, ApprovalsReviewer::User);
    session
        .spawn_task(
            Arc::clone(&old_turn),
            Vec::new(),
            NeverEndingTask {
                kind: TaskKind::Regular,
                listen_to_cancellation_token: true,
            },
        )
        .await;

    let authority = codex_protocol::protocol::McpElicitationAuthority {
        approval_policy: AskForApproval::Never,
        permission_profile: PermissionProfile::Disabled,
        approvals_reviewer: ApprovalsReviewer::AutoReview,
        apps_approvals_reviewers: Default::default(),
    };
    super::handlers::refresh_mcp_servers(
        &session,
        McpServerRefreshConfig {
            mcp_servers: json!({}),
            mcp_oauth_credentials_store_mode: serde_json::to_value(OAuthCredentialsStoreMode::Auto)
                .expect("serialize store mode"),
            elicitation_authority: Some(authority.clone()),
        },
    )
    .await;
    session
        .refresh_mcp_servers_if_requested(&old_turn, None)
        .await;

    assert_eq!(
        session
            .services
            .mcp_connection_manager
            .read()
            .await
            .elicitation_authority(),
        &authority
    );
    let response = session
        .mcp_elicitation_reviewer()
        .review(codex_mcp::ElicitationReviewRequest {
            server_name: "browser-use".to_string(),
            request_id: rmcp::model::NumberOrString::Number(7),
            elicitation: rmcp::model::CreateElicitationRequestParams::FormElicitationParams {
                meta: None,
                message: "Allow origin?".to_string(),
                requested_schema: rmcp::model::ElicitationSchema::builder()
                    .build()
                    .expect("schema should build"),
            },
        })
        .await
        .expect("elicitation review should succeed");
    assert_eq!(
        response,
        Some(ElicitationResponse {
            action: ElicitationAction::Accept,
            content: Some(json!({})),
            meta: None,
        })
    );

    session.abort_all_tasks(TurnAbortReason::Interrupted).await;
}

#[tokio::test]
async fn cancelled_explicit_mcp_refresh_preserves_intent_until_publication() {
    let (session, turn_context, _rx) = make_session_and_context_with_rx().await;
    let old_token = session.mcp_startup_cancellation_token().await;
    let initial_manager_generation = session
        .services
        .mcp_connection_manager
        .read()
        .await
        .generation();
    let refresh_config = McpServerRefreshConfig {
        mcp_servers: json!({}),
        mcp_oauth_credentials_store_mode: serde_json::to_value(OAuthCredentialsStoreMode::Auto)
            .expect("serialize store mode"),
        elicitation_authority: None,
    };
    super::handlers::refresh_mcp_servers(&session, refresh_config).await;
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
    *session.mcp_server_refresh_test_gate.lock().await = Some((Arc::clone(&reached), release));
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
    refresh_task.abort();
    assert!(
        refresh_task
            .await
            .expect_err("refresh task must be cancelled")
            .is_cancelled()
    );

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
async fn rapid_explicit_mcp_refresh_only_publishes_latest_generation() {
    let (session, turn_context, _rx) = make_session_and_context_with_rx().await;
    let initial_manager_generation = session
        .services
        .mcp_connection_manager
        .read()
        .await
        .generation();
    let refresh_config = McpServerRefreshConfig {
        mcp_servers: json!({}),
        mcp_oauth_credentials_store_mode: serde_json::to_value(OAuthCredentialsStoreMode::Auto)
            .expect("serialize store mode"),
        elicitation_authority: None,
    };
    super::handlers::refresh_mcp_servers(&session, refresh_config.clone()).await;

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
    super::handlers::refresh_mcp_servers(&session, refresh_config).await;
    let latest_generation = session
        .mcp_server_refresh_state
        .lock()
        .await
        .pending
        .as_ref()
        .expect("latest refresh intent")
        .generation;
    release.wait().await;
    refresh_task.await.expect("refresh task");

    {
        let state = session.mcp_server_refresh_state.lock().await;
        assert!(state.pending.is_none());
        assert_eq!(state.applied_generation, latest_generation);
        assert_eq!(state.next_generation, latest_generation);
    }
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
