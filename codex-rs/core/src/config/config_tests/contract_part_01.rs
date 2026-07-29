use crate::agents_md::DEFAULT_AGENTS_MD_FILENAME;
use crate::agents_md::LOCAL_AGENTS_MD_FILENAME;
use crate::config::ThreadStoreConfig;
use crate::config::edit::ConfigEdit;
use crate::config::edit::ConfigEditsBuilder;
use crate::config::edit::apply_blocking;
use assert_matches::assert_matches;
use codex_config::CONFIG_TOML_FILE;
use codex_config::ConfigLayerEntry;
use codex_config::ProfileV2Name;
use codex_config::RequirementSource;
use codex_config::config_toml::AgentRoleToml;
use codex_config::config_toml::AgentsToml;
use codex_config::config_toml::AutoReviewToml;
use codex_config::config_toml::ConfigToml;
use codex_config::config_toml::ProjectConfig;
use codex_config::config_toml::RealtimeAudioConfig;
use codex_config::config_toml::RealtimeConfig;
use codex_config::config_toml::RealtimeToml;
use codex_config::config_toml::RealtimeTransport;
use codex_config::config_toml::RealtimeWsMode;
use codex_config::config_toml::RealtimeWsVersion;
use codex_config::config_toml::ToolsToml;
use codex_config::config_toml::UpdatePlanToolConfig;
use codex_config::loader::project_trust_key;
use codex_config::permissions_toml::FilesystemPermissionToml;
use codex_config::permissions_toml::FilesystemPermissionsToml;
use codex_config::permissions_toml::NetworkDomainPermissionToml;
use codex_config::permissions_toml::NetworkDomainPermissionsToml;
use codex_config::permissions_toml::NetworkToml;
use codex_config::permissions_toml::PermissionProfileToml;
use codex_config::permissions_toml::PermissionsToml;
use codex_config::permissions_toml::WorkspaceRootsToml;
use codex_config::profile_toml::ConfigProfile;
use codex_config::types::AppToolApproval;
use codex_config::types::ApprovalsReviewer;
use codex_config::types::BundledSkillsConfig;
use codex_config::types::FeedbackConfigToml;
use codex_config::types::HistoryPersistence;
use codex_config::types::McpServerEnvVar;
use codex_config::types::McpServerOAuthConfig;
use codex_config::types::McpServerToolConfig;
use codex_config::types::McpServerTransportConfig;
use codex_config::types::MemoriesConfig;
use codex_config::types::MemoriesToml;
use codex_config::types::ModelAvailabilityNuxConfig;
use codex_config::types::Notice;
use codex_config::types::NotificationCondition;
use codex_config::types::NotificationMethod;
use codex_config::types::Notifications;
use codex_config::types::OtelConfig;
use codex_config::types::OtelConfigToml;
use codex_config::types::OtelExporterKind;
use codex_config::types::SandboxWorkspaceWrite;
use codex_config::types::SessionPickerViewMode;
use codex_config::types::SkillsConfig;
use codex_config::types::ToolSuggestDisabledTool;
use codex_config::types::ToolSuggestDiscoverableType;
use codex_config::types::Tui;
use codex_config::types::TuiKeymap;
use codex_config::types::TuiNotificationSettings;
use codex_config::types::TuiPetAnchor;
use codex_config::types::WindowsSandboxModeToml;
use codex_config::types::WindowsToml;
use codex_core_plugins::PluginsManager;
use codex_exec_server::LOCAL_FS;
use codex_features::Feature;
use codex_features::FeaturesToml;
use codex_model_provider_info::LMSTUDIO_OSS_PROVIDER_ID;
use codex_model_provider_info::OLLAMA_OSS_PROVIDER_ID;
use codex_model_provider_info::WireApi;
use codex_models_manager::bundled_models_response;
use codex_protocol::config_types::ServiceTier;
use codex_protocol::models::ActivePermissionProfile;
use codex_protocol::models::BUILT_IN_PERMISSION_PROFILE_DANGER_FULL_ACCESS;
use codex_protocol::models::BUILT_IN_PERMISSION_PROFILE_READ_ONLY;
use codex_protocol::models::BUILT_IN_PERMISSION_PROFILE_WORKSPACE;
use codex_protocol::models::ManagedFileSystemPermissions;
use codex_protocol::models::PermissionProfile;
use codex_protocol::models::SandboxEnforcement;
use codex_protocol::permissions::FileSystemAccessMode;
use codex_protocol::permissions::FileSystemPath;
use codex_protocol::permissions::FileSystemSandboxEntry;
use codex_protocol::permissions::FileSystemSandboxPolicy;
use codex_protocol::permissions::FileSystemSpecialPath;
use codex_protocol::permissions::NetworkSandboxPolicy;
use codex_protocol::protocol::NetworkAccess;
use codex_protocol::protocol::RealtimeVoice;
use codex_protocol::protocol::SandboxPolicy;
use serde::Deserialize;
use tempfile::tempdir;

use super::*;
use core_test_support::PathBufExt;
use core_test_support::PathExt;
use core_test_support::TempDirExt;
use core_test_support::test_absolute_path;
use rmcp::model::ElicitationCapability;
use rmcp::model::FormElicitationCapability;
use rmcp::model::UrlElicitationCapability;

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::path::Path;
use std::time::Duration;
use tempfile::TempDir;

fn active_permission_profile_state(
    permission_profile: PermissionProfile,
    profile_id: impl Into<String>,
) -> PermissionProfileState {
    PermissionProfileState::from_constrained_active_profile(
        Constrained::allow_any(permission_profile),
        Some(ActivePermissionProfile::new(profile_id)),
        Vec::new(),
    )
    .expect("active permission profile state should be valid")
}

fn stdio_mcp(command: &str) -> McpServerConfig {
    McpServerConfig {
        transport: McpServerTransportConfig::Stdio {
            command: command.to_string(),
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
        enabled_tools: None,
        disabled_tools: None,
        scopes: None,
        oauth: None,
        oauth_resource: None,
        tools: HashMap::new(),
    }
}

fn http_mcp(url: &str) -> McpServerConfig {
    McpServerConfig {
        transport: McpServerTransportConfig::StreamableHttp {
            url: url.to_string(),
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
    }
}

async fn derive_legacy_sandbox_policy_for_test(
    cfg: &ConfigToml,
    sandbox_mode_override: Option<SandboxMode>,
    profile_sandbox_mode: Option<SandboxMode>,
    windows_sandbox_level: WindowsSandboxLevel,
    active_project: Option<&ProjectConfig>,
    permission_profile_constraint: Option<&Constrained<PermissionProfile>>,
) -> SandboxPolicy {
    let permission_profile = cfg
        .derive_permission_profile(
            sandbox_mode_override,
            profile_sandbox_mode,
            windows_sandbox_level,
            active_project,
            permission_profile_constraint,
        )
        .await;
    permission_profile
        .to_legacy_sandbox_policy(Path::new("/"))
        .unwrap_or_else(|err| {
            tracing::warn!(
                error = %err,
                "derived permission profile cannot be represented as a legacy sandbox policy; falling back to read-only"
            );
            SandboxPolicy::new_read_only_policy()
        })
}

#[tokio::test]
async fn load_config_normalizes_relative_cwd_override() -> std::io::Result<()> {
    let expected_cwd = AbsolutePathBuf::relative_to_current_dir("nested")?;
    let codex_home = tempdir()?;
    let config = Config::load_from_base_config_with_overrides(
        ConfigToml::default(),
        ConfigOverrides {
            cwd: Some(PathBuf::from("nested")),
            ..Default::default()
        },
        codex_home.abs(),
    )
    .await?;

    assert_eq!(config.cwd, expected_cwd);
    Ok(())
}

#[tokio::test]
async fn load_config_loads_global_agents_instructions() -> std::io::Result<()> {
    let codex_home = tempdir()?;
    std::fs::write(
        codex_home.path().join(DEFAULT_AGENTS_MD_FILENAME),
        "\n  global instructions  \n",
    )?;

    let mut config = Config::load_from_base_config_with_overrides(
        ConfigToml::default(),
        ConfigOverrides::default(),
        codex_home.abs(),
    )
    .await?;
    let _ = config.features.enable(Feature::MemoryTool);

    assert_eq!(
        config.user_instructions.as_deref(),
        Some("global instructions")
    );
    Ok(())
}

#[tokio::test]
async fn load_config_prefers_global_agents_override_instructions() -> std::io::Result<()> {
    let codex_home = tempdir()?;
    std::fs::write(
        codex_home.path().join(DEFAULT_AGENTS_MD_FILENAME),
        "global instructions",
    )?;
    let global_agents_override_path = codex_home.path().join(LOCAL_AGENTS_MD_FILENAME);
    std::fs::write(&global_agents_override_path, "local override instructions")?;

    let config = Config::load_from_base_config_with_overrides(
        ConfigToml::default(),
        ConfigOverrides::default(),
        codex_home.abs(),
    )
    .await?;

    assert_eq!(
        config.user_instructions.as_deref(),
        Some("local override instructions")
    );
    Ok(())
}

#[tokio::test]
async fn test_toml_parsing() {
    let history_with_persistence = r#"
[history]
persistence = "save-all"
"#;
    let history_with_persistence_cfg = toml::from_str::<ConfigToml>(history_with_persistence)
        .expect("TOML deserialization should succeed");
    assert_eq!(
        Some(History {
            persistence: HistoryPersistence::SaveAll,
            max_bytes: None,
        }),
        history_with_persistence_cfg.history
    );

    let history_no_persistence = r#"
[history]
persistence = "none"
"#;

    let history_no_persistence_cfg = toml::from_str::<ConfigToml>(history_no_persistence)
        .expect("TOML deserialization should succeed");
    assert_eq!(
        Some(History {
            persistence: HistoryPersistence::None,
            max_bytes: None,
        }),
        history_no_persistence_cfg.history
    );

    let memories = r#"
[memories]
disable_on_external_context = true
generate_memories = false
use_memories = false
max_raw_memories_for_consolidation = 512
max_unused_days = 21
max_rollout_age_days = 42
max_rollouts_per_startup = 9
min_rollout_idle_hours = 24
min_rate_limit_remaining_percent = 12
extract_model = "gpt-5-mini"
consolidation_model = "gpt-5.2"
"#;
    let memories_cfg =
        toml::from_str::<ConfigToml>(memories).expect("TOML deserialization should succeed");
    assert_eq!(
        Some(MemoriesToml {
            disable_on_external_context: Some(true),
            generate_memories: Some(false),
            use_memories: Some(false),
            max_raw_memories_for_consolidation: Some(512),
            max_unused_days: Some(21),
            max_rollout_age_days: Some(42),
            max_rollouts_per_startup: Some(9),
            min_rollout_idle_hours: Some(24),
            min_rate_limit_remaining_percent: Some(12),
            extract_model: Some("gpt-5-mini".to_string()),
            consolidation_model: Some("gpt-5.2".to_string()),
        }),
        memories_cfg.memories
    );

    let config = Config::load_from_base_config_with_overrides(
        memories_cfg,
        ConfigOverrides::default(),
        tempdir().expect("tempdir").abs(),
    )
    .await
    .expect("load config from memories settings");
    assert_eq!(
        config.memories,
        MemoriesConfig {
            disable_on_external_context: true,
            generate_memories: false,
            use_memories: false,
            max_raw_memories_for_consolidation: 512,
            max_unused_days: 21,
            max_rollout_age_days: 42,
            max_rollouts_per_startup: 9,
            min_rollout_idle_hours: 24,
            min_rate_limit_remaining_percent: 12,
            extract_model: Some("gpt-5-mini".to_string()),
            consolidation_model: Some("gpt-5.2".to_string()),
        }
    );

    let legacy_memories_cfg =
        toml::from_str::<ConfigToml>("[memories]\nno_memories_if_mcp_or_web_search = true\n")
            .expect("legacy memories TOML should deserialize");
    assert!(
        MemoriesConfig::from(
            legacy_memories_cfg
                .memories
                .expect("legacy memories config")
        )
        .disable_on_external_context
    );
}

#[test]
fn parses_bundled_skills_config() {
    let cfg: ConfigToml = toml::from_str(
        r#"
[skills]
include_instructions = false

[skills.bundled]
enabled = false
"#,
    )
    .expect("TOML deserialization should succeed");

    assert_eq!(
        cfg.skills,
        Some(SkillsConfig {
            bundled: Some(BundledSkillsConfig { enabled: false }),
            include_instructions: Some(false),
            config: Vec::new(),
        })
    );
}

#[test]
fn tools_web_search_true_deserializes_to_none() {
    let cfg: ConfigToml = toml::from_str(
        r#"
[tools]
web_search = true
"#,
    )
    .expect("TOML deserialization should succeed");

    assert_eq!(
        cfg.tools,
        Some(ToolsToml {
            web_search: None,
            update_plan: None,
        })
    );
}

#[test]
fn tools_web_search_false_deserializes_to_none() {
    let cfg: ConfigToml = toml::from_str(
        r#"
[tools]
web_search = false
"#,
    )
    .expect("TOML deserialization should succeed");

    assert_eq!(
        cfg.tools,
        Some(ToolsToml {
            web_search: None,
            update_plan: None,
        })
    );
}

#[test]
fn tools_update_plan_defaults_to_enabled() {
    let cfg: ConfigToml = toml::from_str(
        r#"
[tools.update_plan]
"#,
    )
    .expect("TOML deserialization should succeed");

    assert_eq!(
        cfg.tools,
        Some(ToolsToml {
            web_search: None,
            update_plan: Some(UpdatePlanToolConfig { enabled: true }),
        })
    );
}

#[tokio::test]
async fn load_config_resolves_update_plan_enabled() {
    let config_toml: ConfigToml = toml::from_str(
        r#"
[tools.update_plan]
enabled = false
"#,
    )
    .expect("TOML deserialization should succeed");
    let config = Config::load_from_base_config_with_overrides(
        config_toml,
        ConfigOverrides::default(),
        tempdir().expect("tempdir").abs(),
    )
    .await
    .expect("load config from update_plan settings");

    assert!(!config.update_plan_enabled);
}

#[test]
fn rejects_provider_auth_with_env_key() {
    let err = toml::from_str::<ConfigToml>(
        r#"
[model_providers.corp]
name = "Corp"
env_key = "CORP_TOKEN"

[model_providers.corp.auth]
command = "print-token"
"#,
    )
    .unwrap_err();

    assert!(
        err.to_string()
            .contains("model_providers.corp: provider auth cannot be combined with env_key")
    );
}

#[test]
fn rejects_provider_aws_for_custom_provider() {
    let err = toml::from_str::<ConfigToml>(
        r#"
[model_providers.custom]
name = "Custom Provider"

[model_providers.custom.aws]
profile = "codex-bedrock"
"#,
    )
    .unwrap_err();

    assert!(
        err.to_string().contains(
            "model_providers.custom: provider aws is only supported for `amazon-bedrock`"
        )
    );
}

#[test]
fn accepts_amazon_bedrock_aws_profile_override() {
    let cfg = toml::from_str::<ConfigToml>(
        r#"
[model_providers.amazon-bedrock.aws]
profile = "codex-bedrock"
region = "us-west-2"
"#,
    )
    .expect("Amazon Bedrock AWS overrides should deserialize");

    assert_eq!(
        cfg.model_providers
            .get("amazon-bedrock")
            .and_then(|provider| provider.aws.as_ref())
            .and_then(|aws| aws.profile.as_deref()),
        Some("codex-bedrock")
    );
    assert_eq!(
        cfg.model_providers
            .get("amazon-bedrock")
            .and_then(|provider| provider.aws.as_ref())
            .and_then(|aws| aws.region.as_deref()),
        Some("us-west-2")
    );
}

#[test]
fn hepta_default_model_provider_policy_prefers_hepta_env() {
    let provider = hepta_default_model_provider_id_with(|key| match key {
        HEPTA_DEFAULT_MODEL_PROVIDER_ENV => Some(" ollama ".to_string()),
        LEGACY_CODEX_DEFAULT_MODEL_PROVIDER_ENV => Some("legacy-provider".to_string()),
        _ => None,
    });

    assert_eq!(provider, "ollama");
}

#[test]
fn hepta_default_model_provider_policy_keeps_legacy_fallback() {
    let provider = hepta_default_model_provider_id_with(|key| match key {
        HEPTA_DEFAULT_MODEL_PROVIDER_ENV => Some("  ".to_string()),
        LEGACY_CODEX_DEFAULT_MODEL_PROVIDER_ENV => Some("legacy-provider".to_string()),
        _ => None,
    });

    assert_eq!(provider, "legacy-provider");
}

#[test]
fn hepta_default_model_provider_policy_falls_back_to_openai() {
    assert_eq!(
        hepta_default_model_provider_id_with(|_| None),
        DEFAULT_MODEL_PROVIDER_ID
    );
}

#[test]
fn hepta_default_model_policy_prefers_hepta_env() {
    let model = hepta_default_model_from_env_with(|key| match key {
        HEPTA_DEFAULT_MODEL_ENV => Some(" gpt-5.5 ".to_string()),
        LEGACY_CODEX_DEFAULT_MODEL_ENV => Some("legacy-model".to_string()),
        _ => None,
    });

    assert_eq!(model.as_deref(), Some("gpt-5.5"));
}

#[test]
fn hepta_default_model_policy_keeps_legacy_fallback() {
    let model = hepta_default_model_from_env_with(|key| match key {
        HEPTA_DEFAULT_MODEL_ENV => Some("  ".to_string()),
        LEGACY_CODEX_DEFAULT_MODEL_ENV => Some("legacy-model".to_string()),
        _ => None,
    });

    assert_eq!(model.as_deref(), Some("legacy-model"));
}

#[tokio::test]
async fn load_config_applies_amazon_bedrock_aws_profile_override() {
    let cfg = toml::from_str::<ConfigToml>(
        r#"
model_provider = "amazon-bedrock"

[model_providers.amazon-bedrock.aws]
profile = "codex-bedrock"
region = "us-west-2"
"#,
    )
    .expect("Amazon Bedrock AWS overrides should deserialize");

    let config = Config::load_from_base_config_with_overrides(
        cfg,
        ConfigOverrides::default(),
        tempdir().expect("tempdir").abs(),
    )
    .await
    .expect("load config");

    assert_eq!(config.model_provider_id, "amazon-bedrock");
    assert_eq!(
        config
            .model_provider
            .aws
            .as_ref()
            .and_then(|aws| aws.profile.as_deref()),
        Some("codex-bedrock")
    );
    assert_eq!(
        config
            .model_provider
            .aws
            .as_ref()
            .and_then(|aws| aws.region.as_deref()),
        Some("us-west-2")
    );
}

#[tokio::test]
async fn load_config_rejects_unsupported_amazon_bedrock_overrides() {
    let cfg = toml::from_str::<ConfigToml>(
        r#"
model_provider = "amazon-bedrock"

[model_providers.amazon-bedrock]
name = "Custom Bedrock"
base_url = "https://bedrock.example.com/v1"
requires_openai_auth = true
supports_websockets = true

[model_providers.amazon-bedrock.aws]
profile = "codex-bedrock"
region = "us-west-2"
"#,
    )
    .expect("Amazon Bedrock unsupported overrides should deserialize");

    let err = Config::load_from_base_config_with_overrides(
        cfg,
        ConfigOverrides::default(),
        tempdir().expect("tempdir").abs(),
    )
    .await
    .unwrap_err();

    assert_eq!(err.kind(), std::io::ErrorKind::InvalidData);
    assert!(err.to_string().contains(
        "model_providers.amazon-bedrock only supports changing `aws.profile` and `aws.region`; other non-default provider fields are not supported"
    ));
}

#[test]
fn config_toml_deserializes_model_availability_nux() {
    let toml = r#"
[tui.model_availability_nux]
"gpt-foo" = 2
"gpt-bar" = 4
"#;
    let cfg: ConfigToml =
        toml::from_str(toml).expect("TOML deserialization should succeed for TUI NUX");

    assert_eq!(
        cfg.tui.expect("tui config should deserialize"),
        Tui {
            notification_settings: TuiNotificationSettings::default(),
            animations: true,
            show_tooltips: true,
            vim_mode_default: false,
            raw_output_mode: false,
            alternate_screen: AltScreenMode::default(),
            status_line: None,
            status_line_use_colors: true,
            terminal_title: None,
            theme: None,
            pet: None,
            pet_anchor: TuiPetAnchor::Composer,
            session_picker_view: None,
            keymap: TuiKeymap::default(),
            model_availability_nux: ModelAvailabilityNuxConfig {
                shown_count: HashMap::from([
                    ("gpt-bar".to_string(), 4),
                    ("gpt-foo".to_string(), 2),
                ]),
            },
            terminal_resize_reflow_max_rows: None,
        }
    );
}

#[test]
fn config_toml_status_line_use_colors_defaults_to_enabled() {
    let toml = r#"
[tui]
"#;
    let cfg: ConfigToml =
        toml::from_str(toml).expect("TOML deserialization should succeed for TUI config");

    assert!(
        cfg.tui
            .expect("tui config should deserialize")
            .status_line_use_colors
    );
}

#[test]
fn config_toml_deserializes_status_line_use_colors_disabled() {
    let toml = r#"
[tui]
status_line_use_colors = false
"#;
    let cfg: ConfigToml =
        toml::from_str(toml).expect("TOML deserialization should succeed for TUI config");

    assert!(
        !cfg.tui
            .expect("tui config should deserialize")
            .status_line_use_colors
    );
}

#[test]
fn config_toml_deserializes_terminal_resize_reflow_config() {
    let toml = r#"
[tui]
terminal_resize_reflow_max_rows = 9000
"#;
    let cfg: ConfigToml =
        toml::from_str(toml).expect("TOML deserialization should succeed for resize reflow config");

    assert_eq!(
        cfg.tui
            .expect("tui config should deserialize")
            .terminal_resize_reflow_max_rows,
        Some(9000)
    );
}

#[tokio::test]
async fn runtime_config_defaults_model_availability_nux() {
    let cfg = Config::load_from_base_config_with_overrides(
        ConfigToml::default(),
        ConfigOverrides::default(),
        tempdir().expect("tempdir").abs(),
    )
    .await
    .expect("load config");

    assert_eq!(
        cfg.model_availability_nux,
        ModelAvailabilityNuxConfig::default()
    );
}

#[test]
fn test_tui_vim_mode_default_defaults_to_false() {
    let toml = r#"
        [tui]
    "#;
    let parsed: ConfigToml = toml::from_str(toml).expect("deserialize empty [tui] table");
    assert!(
        !parsed
            .tui
            .expect("config should include tui section")
            .vim_mode_default
    );
}

#[test]
fn test_tui_vim_mode_default_true() {
    let toml = r#"
        [tui]
        vim_mode_default = true
    "#;
    let parsed: ConfigToml = toml::from_str(toml).expect("deserialize vim_mode_default=true");
    assert!(
        parsed
            .tui
            .expect("config should include tui section")
            .vim_mode_default
    );
}

#[test]
fn test_tui_raw_output_mode_defaults_to_false() {
    let toml = r#"
        [tui]
    "#;
    let parsed: ConfigToml = toml::from_str(toml).expect("deserialize empty [tui] table");
    assert!(
        !parsed
            .tui
            .expect("config should include tui section")
            .raw_output_mode
    );
}

#[test]
fn test_tui_raw_output_mode_true() {
    let toml = r#"
        [tui]
        raw_output_mode = true
    "#;
    let parsed: ConfigToml = toml::from_str(toml).expect("deserialize raw_output_mode=true");
    assert!(
        parsed
            .tui
            .expect("config should include tui section")
            .raw_output_mode
    );
}

#[tokio::test]
async fn runtime_config_uses_tui_raw_output_mode() {
    let toml = r#"
        [tui]
        raw_output_mode = true
    "#;
    let cfg_toml: ConfigToml = toml::from_str(toml).expect("deserialize raw_output_mode=true");
    let cfg = Config::load_from_base_config_with_overrides(
        cfg_toml,
        ConfigOverrides::default(),
        tempdir().expect("tempdir").abs(),
    )
    .await
    .expect("load config");

    assert!(cfg.tui_raw_output_mode);
}

#[test]
fn config_toml_deserializes_permission_profiles() {
    let toml = r#"
default_permissions = "workspace"

[permissions.workspace.workspace_roots]
"~/code/openai" = true
"~/code/ignored" = false

[permissions.workspace.filesystem]
":minimal" = "read"

[permissions.workspace.filesystem.":workspace_roots"]
"." = "write"
"docs" = "read"

[permissions.workspace.network]
enabled = true
proxy_url = "http://127.0.0.1:43128"
enable_socks5 = false
allow_upstream_proxy = false

[permissions.workspace.network.domains]
"openai.com" = "allow"
"#;
    let cfg: ConfigToml =
        toml::from_str(toml).expect("TOML deserialization should succeed for permissions profiles");

    assert_eq!(cfg.default_permissions.as_deref(), Some("workspace"));
    assert_eq!(
        cfg.permissions.expect("[permissions] should deserialize"),
        PermissionsToml {
            entries: BTreeMap::from([(
                "workspace".to_string(),
                PermissionProfileToml {
                    workspace_roots: Some(WorkspaceRootsToml {
                        entries: BTreeMap::from([
                            ("~/code/ignored".to_string(), false),
                            ("~/code/openai".to_string(), true),
                        ]),
                    }),
                    filesystem: Some(FilesystemPermissionsToml {
                        glob_scan_max_depth: None,
                        entries: BTreeMap::from([
                            (
                                ":minimal".to_string(),
                                FilesystemPermissionToml::Access(FileSystemAccessMode::Read),
                            ),
                            (
                                ":workspace_roots".to_string(),
                                FilesystemPermissionToml::Scoped(BTreeMap::from([
                                    (".".to_string(), FileSystemAccessMode::Write),
                                    ("docs".to_string(), FileSystemAccessMode::Read),
                                ])),
                            ),
                        ]),
                    }),
                    network: Some(NetworkToml {
                        enabled: Some(true),
                        proxy_url: Some("http://127.0.0.1:43128".to_string()),
                        enable_socks5: Some(false),
                        socks_url: None,
                        enable_socks5_udp: None,
                        allow_upstream_proxy: Some(false),
                        dangerously_allow_non_loopback_proxy: None,
                        dangerously_allow_all_unix_sockets: None,
                        mode: None,
                        domains: Some(NetworkDomainPermissionsToml {
                            entries: BTreeMap::from([(
                                "openai.com".to_string(),
                                NetworkDomainPermissionToml::Allow,
                            )]),
                        }),
                        unix_sockets: None,
                        allow_local_binding: None,
                    }),
                },
            )]),
        }
    );
}

#[tokio::test]
async fn permissions_profiles_proxy_policy_does_not_start_managed_network_proxy_without_feature()
-> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let cwd = TempDir::new()?;
    std::fs::write(cwd.path().join(".git"), "gitdir: nowhere")?;

    let config = Config::load_from_base_config_with_overrides(
        ConfigToml {
            default_permissions: Some("workspace".to_string()),
            permissions: Some(PermissionsToml {
                entries: BTreeMap::from([(
                    "workspace".to_string(),
                    PermissionProfileToml {
                        workspace_roots: None,
                        filesystem: Some(FilesystemPermissionsToml {
                            glob_scan_max_depth: None,
                            entries: BTreeMap::from([(
                                ":minimal".to_string(),
                                FilesystemPermissionToml::Access(FileSystemAccessMode::Read),
                            )]),
                        }),
                        network: Some(NetworkToml {
                            enabled: Some(true),
                            proxy_url: Some("http://127.0.0.1:43128".to_string()),
                            enable_socks5: Some(false),
                            ..Default::default()
                        }),
                    },
                )]),
            }),
            ..Default::default()
        },
        ConfigOverrides {
            cwd: Some(cwd.path().to_path_buf()),
            ..Default::default()
        },
        codex_home.abs(),
    )
    .await?;
    assert_eq!(
        config.permissions.network_sandbox_policy(),
        NetworkSandboxPolicy::Enabled
    );
    assert!(
        config.permissions.network.is_none(),
        "profile proxy policy should not start the managed network proxy without the feature"
    );
    Ok(())
}

#[tokio::test]
async fn network_proxy_feature_is_no_op_without_sandbox_network() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let cwd = TempDir::new()?;
    let config = Config::load_from_base_config_with_overrides(
        ConfigToml {
            features: Some(toml::from_str("network_proxy = true").expect("valid features")),
            ..Default::default()
        },
        ConfigOverrides {
            cwd: Some(cwd.path().to_path_buf()),
            ..Default::default()
        },
        codex_home.abs(),
    )
    .await?;

    assert_eq!(
        config.permissions.network_sandbox_policy(),
        NetworkSandboxPolicy::Restricted
    );
    assert!(
        config.permissions.network.is_none(),
        "network_proxy should not start the managed network proxy while network access is off"
    );
    Ok(())
}

#[tokio::test]
async fn network_proxy_feature_matrix_preserves_sandbox_network_semantics() -> std::io::Result<()> {
    #[derive(Clone, Copy)]
    enum Surface {
        PermissionProfile,
        LegacyWorkspaceWrite,
    }

    struct Case {
        name: &'static str,
        surface: Surface,
        network_enabled: bool,
        proxy_enabled: bool,
        expected_network_policy: NetworkSandboxPolicy,
    }

    let cases = [
        Case {
            name: "permission profile network disabled without proxy",
            surface: Surface::PermissionProfile,
            network_enabled: false,
            proxy_enabled: false,
            expected_network_policy: NetworkSandboxPolicy::Restricted,
        },
        Case {
            name: "permission profile network disabled with proxy",
            surface: Surface::PermissionProfile,
            network_enabled: false,
            proxy_enabled: true,
            expected_network_policy: NetworkSandboxPolicy::Restricted,
        },
        Case {
            name: "permission profile network enabled without proxy",
            surface: Surface::PermissionProfile,
            network_enabled: true,
            proxy_enabled: false,
            expected_network_policy: NetworkSandboxPolicy::Enabled,
        },
        Case {
            name: "permission profile network enabled with proxy",
            surface: Surface::PermissionProfile,
            network_enabled: true,
            proxy_enabled: true,
            expected_network_policy: NetworkSandboxPolicy::Enabled,
        },
        Case {
            name: "legacy workspace write network disabled without proxy",
            surface: Surface::LegacyWorkspaceWrite,
            network_enabled: false,
            proxy_enabled: false,
            expected_network_policy: NetworkSandboxPolicy::Restricted,
        },
        Case {
            name: "legacy workspace write network disabled with proxy",
            surface: Surface::LegacyWorkspaceWrite,
            network_enabled: false,
            proxy_enabled: true,
            expected_network_policy: NetworkSandboxPolicy::Restricted,
        },
        Case {
            name: "legacy workspace write network enabled without proxy",
            surface: Surface::LegacyWorkspaceWrite,
            network_enabled: true,
            proxy_enabled: false,
            expected_network_policy: NetworkSandboxPolicy::Enabled,
        },
        Case {
            name: "legacy workspace write network enabled with proxy",
            surface: Surface::LegacyWorkspaceWrite,
            network_enabled: true,
            proxy_enabled: true,
            expected_network_policy: NetworkSandboxPolicy::Enabled,
        },
    ];

    for case in cases {
        let codex_home = TempDir::new()?;
        let cwd = TempDir::new()?;
        std::fs::write(cwd.path().join(".git"), "gitdir: nowhere")?;
        let features = case
            .proxy_enabled
            .then(|| toml::from_str("network_proxy = true").expect("valid features"));
        let base_config = match case.surface {
            Surface::PermissionProfile => ConfigToml {
                default_permissions: Some("workspace".to_string()),
                permissions: Some(PermissionsToml {
                    entries: BTreeMap::from([(
                        "workspace".to_string(),
                        PermissionProfileToml {
                            workspace_roots: None,
                            filesystem: Some(FilesystemPermissionsToml {
                                glob_scan_max_depth: None,
                                entries: BTreeMap::from([(
                                    ":minimal".to_string(),
                                    FilesystemPermissionToml::Access(FileSystemAccessMode::Read),
                                )]),
                            }),
                            network: Some(NetworkToml {
                                enabled: Some(case.network_enabled),
                                ..Default::default()
                            }),
                        },
                    )]),
                }),
                features,
                ..Default::default()
            },
            Surface::LegacyWorkspaceWrite => ConfigToml {
                sandbox_mode: Some(SandboxMode::WorkspaceWrite),
                sandbox_workspace_write: Some(SandboxWorkspaceWrite {
                    network_access: case.network_enabled,
                    ..Default::default()
                }),
                windows: Some(WindowsToml {
                    sandbox: Some(WindowsSandboxModeToml::Elevated),
                    sandbox_private_desktop: None,
                }),
                features,
                ..Default::default()
            },
        };
        let config = Config::load_from_base_config_with_overrides(
            base_config,
            ConfigOverrides {
                cwd: Some(cwd.path().to_path_buf()),
                ..Default::default()
            },
            codex_home.abs(),
        )
        .await?;

        assert_eq!(
            config.permissions.network_sandbox_policy(),
            case.expected_network_policy,
            "{}",
            case.name
        );
        assert_eq!(
            config.permissions.network.is_some(),
            case.network_enabled && case.proxy_enabled,
            "{}",
            case.name
        );
    }

    Ok(())
}

#[tokio::test]
async fn network_proxy_cli_overrides_merge_toggle_with_proxy_config() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let cwd = TempDir::new()?;
    std::fs::write(
        codex_home.path().join(CONFIG_TOML_FILE),
        r#"
sandbox_mode = "workspace-write"

[sandbox_workspace_write]
network_access = true

[windows]
sandbox = "elevated"
"#,
    )?;
    let config = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .cli_overrides(vec![
            (
                "features.network_proxy.enabled".to_string(),
                toml::Value::Boolean(true),
            ),
            (
                "features.network_proxy.enable_socks5".to_string(),
                toml::Value::Boolean(false),
            ),
        ])
        .harness_overrides(ConfigOverrides {
            cwd: Some(cwd.path().to_path_buf()),
            ..Default::default()
        })
        .build()
        .await?;

    assert_eq!(
        config.permissions.network_sandbox_policy(),
        NetworkSandboxPolicy::Enabled
    );
    let network = config
        .permissions
        .network
        .as_ref()
        .expect("network_proxy should start the managed network proxy");
    assert_eq!(network.proxy_host_and_port(), "127.0.0.1:3128");
    assert!(!network.socks_enabled());
    Ok(())
}

#[tokio::test]
async fn experimental_network_requirements_enable_proxy_without_feature() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let config = ConfigBuilder::without_managed_config_for_tests()
        .codex_home(codex_home.path().to_path_buf())
        .fallback_cwd(Some(codex_home.path().to_path_buf()))
        .cloud_requirements(CloudRequirementsLoader::new(async {
            Ok(Some(codex_config::ConfigRequirementsToml {
                network: Some(codex_config::NetworkRequirementsToml {
                    enabled: Some(true),
                    ..Default::default()
                }),
                ..Default::default()
            }))
        }))
        .build()
        .await?;

    assert!(!config.features.enabled(Feature::NetworkProxy));
    assert!(config.managed_network_requirements_enabled());
    assert!(
        config
            .permissions
            .network
            .as_ref()
            .expect("experimental_network should configure the managed proxy")
            .enabled()
    );
    Ok(())
}

#[tokio::test]
async fn network_proxy_feature_uses_profile_network_proxy_settings() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let cwd = TempDir::new()?;
    let config = Config::load_from_base_config_with_overrides(
        ConfigToml {
            features: Some(toml::from_str("network_proxy = true").expect("valid features")),
            default_permissions: Some("workspace".to_string()),
            permissions: Some(PermissionsToml {
                entries: BTreeMap::from([(
                    "workspace".to_string(),
                    PermissionProfileToml {
                        workspace_roots: None,
                        filesystem: Some(FilesystemPermissionsToml {
                            glob_scan_max_depth: None,
                            entries: BTreeMap::from([(
                                ":minimal".to_string(),
                                FilesystemPermissionToml::Access(FileSystemAccessMode::Read),
                            )]),
                        }),
                        network: Some(NetworkToml {
                            enabled: Some(true),
                            proxy_url: Some("http://127.0.0.1:43128".to_string()),
                            enable_socks5: Some(false),
                            ..Default::default()
                        }),
                    },
                )]),
            }),
            ..Default::default()
        },
        ConfigOverrides {
            cwd: Some(cwd.path().to_path_buf()),
            ..Default::default()
        },
        codex_home.abs(),
    )
    .await?;

    assert_eq!(
        config.permissions.network_sandbox_policy(),
        NetworkSandboxPolicy::Enabled
    );
    let network = config
        .permissions
        .network
        .as_ref()
        .expect("network_proxy should start the managed network proxy");
    assert_eq!(network.proxy_host_and_port(), "127.0.0.1:43128");
    assert!(!network.socks_enabled());
    Ok(())
}

#[tokio::test]
async fn profile_network_proxy_disable_ignores_base_feature_config() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let cwd = TempDir::new()?;
    let config = Config::load_from_base_config_with_overrides(
        ConfigToml {
            features: Some(
                toml::from_str(
                    r#"
[network_proxy]
enabled = true
proxy_url = "http://127.0.0.1:43128"
"#,
                )
                .expect("valid base features"),
            ),
            profiles: HashMap::from([(
                "no_proxy".to_string(),
                ConfigProfile {
                    features: Some(
                        toml::from_str("network_proxy = false").expect("valid profile features"),
                    ),
                    ..Default::default()
                },
            )]),
            profile: Some("no_proxy".to_string()),
            ..Default::default()
        },
        ConfigOverrides {
            cwd: Some(cwd.path().to_path_buf()),
            ..Default::default()
        },
        codex_home.abs(),
    )
    .await?;

    assert!(!config.features.enabled(Feature::NetworkProxy));
    assert!(config.permissions.network.is_none());
    Ok(())
}

#[tokio::test]
async fn disabled_network_proxy_feature_does_not_start_profile_proxy_policy() -> std::io::Result<()>
{
    let codex_home = TempDir::new()?;
    let cwd = TempDir::new()?;
    let config = Config::load_from_base_config_with_overrides(
        ConfigToml {
            features: Some(
                toml::from_str(
                    r#"
[network_proxy]
enabled = false
"#,
                )
                .expect("valid features"),
            ),
            default_permissions: Some("workspace".to_string()),
            permissions: Some(PermissionsToml {
                entries: BTreeMap::from([(
                    "workspace".to_string(),
                    PermissionProfileToml {
                        workspace_roots: None,
                        filesystem: Some(FilesystemPermissionsToml {
                            glob_scan_max_depth: None,
                            entries: BTreeMap::from([(
                                ":minimal".to_string(),
                                FilesystemPermissionToml::Access(FileSystemAccessMode::Read),
                            )]),
                        }),
                        network: Some(NetworkToml {
                            enabled: Some(true),
                            proxy_url: Some("http://127.0.0.1:43128".to_string()),
                            enable_socks5: Some(false),
                            ..Default::default()
                        }),
                    },
                )]),
            }),
            ..Default::default()
        },
        ConfigOverrides {
            cwd: Some(cwd.path().to_path_buf()),
            ..Default::default()
        },
        codex_home.abs(),
    )
    .await?;

    assert!(!config.features.enabled(Feature::NetworkProxy));
    assert!(
        config.permissions.network.is_none(),
        "disabled feature should keep profile proxy policy from starting the managed proxy"
    );
    Ok(())
}

#[tokio::test]
async fn permissions_profiles_network_disabled_by_default_does_not_start_proxy()
-> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let cwd = TempDir::new()?;
    std::fs::write(cwd.path().join(".git"), "gitdir: nowhere")?;

    let config = Config::load_from_base_config_with_overrides(
        ConfigToml {
            default_permissions: Some("workspace".to_string()),
            permissions: Some(PermissionsToml {
                entries: BTreeMap::from([(
                    "workspace".to_string(),
                    PermissionProfileToml {
                        workspace_roots: None,
                        filesystem: Some(FilesystemPermissionsToml {
                            glob_scan_max_depth: None,
                            entries: BTreeMap::from([(
                                ":minimal".to_string(),
                                FilesystemPermissionToml::Access(FileSystemAccessMode::Read),
                            )]),
                        }),
                        network: Some(NetworkToml {
                            domains: Some(NetworkDomainPermissionsToml {
                                entries: BTreeMap::from([(
                                    "openai.com".to_string(),
                                    NetworkDomainPermissionToml::Allow,
                                )]),
                            }),
                            ..Default::default()
                        }),
                    },
                )]),
            }),
            ..Default::default()
        },
        ConfigOverrides {
            cwd: Some(cwd.path().to_path_buf()),
            ..Default::default()
        },
        codex_home.abs(),
    )
    .await?;

    assert!(config.permissions.network.is_none());
    Ok(())
}

#[tokio::test]
async fn default_permissions_profile_populates_runtime_sandbox_policy() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let cwd = TempDir::new()?;
    std::fs::create_dir_all(cwd.path().join("docs"))?;
    std::fs::write(cwd.path().join(".git"), "gitdir: nowhere")?;

    let cfg = ConfigToml {
        default_permissions: Some("workspace".to_string()),
        permissions: Some(PermissionsToml {
            entries: BTreeMap::from([(
                "workspace".to_string(),
                PermissionProfileToml {
                    workspace_roots: None,
                    filesystem: Some(FilesystemPermissionsToml {
                        glob_scan_max_depth: None,
                        entries: BTreeMap::from([
                            (
                                ":minimal".to_string(),
                                FilesystemPermissionToml::Access(FileSystemAccessMode::Read),
                            ),
                            (
                                ":workspace_roots".to_string(),
                                FilesystemPermissionToml::Scoped(BTreeMap::from([
                                    (".".to_string(), FileSystemAccessMode::Write),
                                    ("docs".to_string(), FileSystemAccessMode::Read),
                                ])),
                            ),
                        ]),
                    }),
                    network: None,
                },
            )]),
        }),
        ..Default::default()
    };

    let config = Config::load_from_base_config_with_overrides(
        cfg,
        ConfigOverrides {
            cwd: Some(cwd.path().to_path_buf()),
            ..Default::default()
        },
        codex_home.abs(),
    )
    .await?;

    let cwd_root = cwd.path().abs();
    let memories_root = codex_home.path().join("memories").abs();
    assert_eq!(
        config.permissions.file_system_sandbox_policy(),
        FileSystemSandboxPolicy::restricted(vec![
            FileSystemSandboxEntry {
                path: FileSystemPath::Special {
                    value: FileSystemSpecialPath::Minimal,
                },
                access: FileSystemAccessMode::Read,
            },
            FileSystemSandboxEntry {
                path: FileSystemPath::Path {
                    path: cwd_root.clone(),
                },
                access: FileSystemAccessMode::Write,
            },
            FileSystemSandboxEntry {
                path: FileSystemPath::Path {
                    path: cwd_root.join("docs"),
                },
                access: FileSystemAccessMode::Read,
            },
            FileSystemSandboxEntry {
                path: FileSystemPath::Path {
                    path: memories_root.clone(),
                },
                access: FileSystemAccessMode::Write,
            },
        ]),
    );
    assert_eq!(
        &config.legacy_sandbox_policy(),
        &SandboxPolicy::WorkspaceWrite {
            writable_roots: vec![memories_root],
            network_access: false,
            exclude_tmpdir_env_var: true,
            exclude_slash_tmp: true,
        }
    );
    assert!(
        !config
            .permissions
            .file_system_sandbox_policy()
            .can_write_path_with_cwd(&cwd.path().join(".git"), cwd.path())
    );
    assert_eq!(
        config.permissions.network_sandbox_policy(),
        NetworkSandboxPolicy::Restricted
    );
    assert_eq!(
        config
            .permissions
            .active_permission_profile()
            .as_ref()
            .map(|active| active.id.as_str()),
        Some("workspace")
    );
    Ok(())
}

#[tokio::test]
async fn permission_profile_override_populates_runtime_permissions() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let cwd = TempDir::new()?;
    let permission_profile = PermissionProfile::Disabled;

    let config = Config::load_from_base_config_with_overrides(
        ConfigToml::default(),
        ConfigOverrides {
            cwd: Some(cwd.path().to_path_buf()),
            permission_profile: Some(permission_profile.clone()),
            ..Default::default()
        },
        codex_home.abs(),
    )
    .await?;

    assert_eq!(
        config.permissions.effective_permission_profile(),
        permission_profile
    );
    assert_eq!(config.permissions.active_permission_profile(), None);
    assert_eq!(
        &config.legacy_sandbox_policy(),
        &SandboxPolicy::DangerFullAccess
    );
    Ok(())
}

#[test]
fn permission_snapshot_setter_preserves_permission_constraints() {
    let initial_profile = PermissionProfile::read_only();
    let mut permissions = Permissions::from_approval_and_profile(
        Constrained::allow_any(AskForApproval::Never),
        Constrained::allow_only(initial_profile.clone()),
    )
    .expect("initial permissions should satisfy constraints");

    let err = permissions
        .set_permission_profile_from_session_snapshot(PermissionProfileSnapshot::active(
            PermissionProfile::workspace_write(),
            ActivePermissionProfile::new(BUILT_IN_PERMISSION_PROFILE_WORKSPACE),
        ))
        .expect_err("workspace profile should violate read-only constraint");

    assert_eq!(permissions.permission_profile(), &initial_profile);
    assert_eq!(permissions.active_permission_profile(), None);
    assert!(
        matches!(err, ConstraintError::InvalidValue { .. }),
        "expected invalid value constraint error, got {err:?}"
    );
}

#[tokio::test]
async fn permission_profile_override_preserves_managed_unrestricted_filesystem()
-> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let cwd = TempDir::new()?;
    let permission_profile = PermissionProfile::Managed {
        file_system: ManagedFileSystemPermissions::Unrestricted,
        network: NetworkSandboxPolicy::Restricted,
    };

    let config = Config::load_from_base_config_with_overrides(
        ConfigToml::default(),
        ConfigOverrides {
            cwd: Some(cwd.path().to_path_buf()),
            permission_profile: Some(permission_profile.clone()),
            ..Default::default()
        },
        codex_home.abs(),
    )
    .await?;

    assert_eq!(
        config.permissions.effective_permission_profile(),
        permission_profile
    );
    assert_eq!(
        &config.legacy_sandbox_policy(),
        &SandboxPolicy::ExternalSandbox {
            network_access: NetworkAccess::Restricted,
        }
    );
    Ok(())
}

#[tokio::test]
async fn managed_unrestricted_permission_profile_still_enables_network_requirements()
-> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let cwd = TempDir::new()?;
    let permission_profile = PermissionProfile::Managed {
        file_system: ManagedFileSystemPermissions::Unrestricted,
        network: NetworkSandboxPolicy::Enabled,
    };

    let mut config = Config::load_from_base_config_with_overrides(
        ConfigToml::default(),
        ConfigOverrides {
            cwd: Some(cwd.path().to_path_buf()),
            permission_profile: Some(permission_profile),
            ..Default::default()
        },
        codex_home.abs(),
    )
    .await?;
    assert_eq!(
        &config.legacy_sandbox_policy(),
        &SandboxPolicy::DangerFullAccess,
        "the legacy projection is intentionally lossy for managed unrestricted profiles"
    );

    let layers = config
        .config_layer_stack
        .get_layers(
            ConfigLayerStackOrdering::LowestPrecedenceFirst,
            /*include_disabled*/ true,
        )
        .into_iter()
        .cloned()
        .collect();
    let mut requirements = config.config_layer_stack.requirements().clone();
    requirements.network = Some(Sourced::new(
        codex_config::NetworkConstraints {
            enabled: Some(true),
            ..Default::default()
        },
        RequirementSource::CloudRequirements,
    ));
    let mut requirements_toml = config.config_layer_stack.requirements_toml().clone();
    requirements_toml.network = Some(codex_config::NetworkRequirementsToml {
        enabled: Some(true),
        ..Default::default()
    });
    config.config_layer_stack = ConfigLayerStack::new(layers, requirements, requirements_toml)
        .expect("config layer stack with network requirements");

    assert!(config.managed_network_requirements_enabled());
    Ok(())
}

#[tokio::test]
async fn permission_profile_override_applies_runtime_roots_to_legacy_projection()
-> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let cwd = TempDir::new()?;
    let permission_profile = PermissionProfile::from_runtime_permissions(
        &FileSystemSandboxPolicy::restricted(vec![
            FileSystemSandboxEntry {
                path: FileSystemPath::Special {
                    value: FileSystemSpecialPath::Root,
                },
                access: FileSystemAccessMode::Read,
            },
            FileSystemSandboxEntry {
                path: FileSystemPath::Special {
                    value: FileSystemSpecialPath::project_roots(/*subpath*/ None),
                },
                access: FileSystemAccessMode::Write,
            },
        ]),
        NetworkSandboxPolicy::Restricted,
    );

    let config = Config::load_from_base_config_with_overrides(
        ConfigToml::default(),
        ConfigOverrides {
            cwd: Some(cwd.path().to_path_buf()),
            permission_profile: Some(permission_profile),
            ..Default::default()
        },
        codex_home.abs(),
    )
    .await?;

    let memories_root = codex_home.path().join("memories").abs();
    assert!(
        config
            .permissions
            .file_system_sandbox_policy()
            .can_write_path_with_cwd(memories_root.as_path(), cwd.path())
    );
    assert_eq!(
        &config.legacy_sandbox_policy(),
        &SandboxPolicy::WorkspaceWrite {
            writable_roots: vec![memories_root],
            network_access: false,
            exclude_tmpdir_env_var: true,
            exclude_slash_tmp: true,
        }
    );
    Ok(())
}

#[tokio::test]
async fn permission_profile_override_preserves_configured_network_policy_without_starting_proxy()
-> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let cwd = TempDir::new()?;
    let permission_profile = PermissionProfile::Disabled;

    let config = Config::load_from_base_config_with_overrides(
        ConfigToml {
            default_permissions: Some("workspace".to_string()),
            permissions: Some(PermissionsToml {
                entries: BTreeMap::from([(
                    "workspace".to_string(),
                    PermissionProfileToml {
                        workspace_roots: None,
                        filesystem: Some(FilesystemPermissionsToml {
                            glob_scan_max_depth: None,
                            entries: BTreeMap::from([(
                                ":minimal".to_string(),
                                FilesystemPermissionToml::Access(FileSystemAccessMode::Read),
                            )]),
                        }),
                        network: Some(NetworkToml {
                            enabled: Some(true),
                            proxy_url: Some("http://127.0.0.1:43128".to_string()),
                            enable_socks5: Some(false),
                            allow_upstream_proxy: Some(false),
                            domains: Some(NetworkDomainPermissionsToml {
                                entries: BTreeMap::from([(
                                    "openai.com".to_string(),
                                    NetworkDomainPermissionToml::Allow,
                                )]),
                            }),
                            ..Default::default()
                        }),
                    },
                )]),
            }),
            ..Default::default()
        },
        ConfigOverrides {
            cwd: Some(cwd.path().to_path_buf()),
            permission_profile: Some(permission_profile.clone()),
            ..Default::default()
        },
        codex_home.abs(),
    )
    .await?;
    assert!(
        config.permissions.network.is_none(),
        "profile network.enabled should not start the managed network proxy"
    );
    assert_eq!(
        config.permissions.effective_permission_profile(),
        permission_profile
    );
    Ok(())
}

#[tokio::test]
async fn workspace_root_glob_none_compiles_to_filesystem_pattern_entry() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let cwd = TempDir::new()?;
    let extra_root = TempDir::new()?;
    tokio::fs::write(cwd.path().join(".git"), "gitdir: nowhere").await?;
    tokio::fs::write(extra_root.path().join(".git"), "gitdir: nowhere").await?;

    let config = Config::load_from_base_config_with_overrides(
        ConfigToml {
            default_permissions: Some("workspace".to_string()),
            permissions: Some(PermissionsToml {
                entries: BTreeMap::from([(
                    "workspace".to_string(),
                    PermissionProfileToml {
                        workspace_roots: None,
                        filesystem: Some(FilesystemPermissionsToml {
                            glob_scan_max_depth: Some(2),
                            entries: BTreeMap::from([(
                                ":workspace_roots".to_string(),
                                FilesystemPermissionToml::Scoped(BTreeMap::from([
                                    (".".to_string(), FileSystemAccessMode::Write),
                                    ("**/*.env".to_string(), FileSystemAccessMode::None),
                                ])),
                            )]),
                        }),
                        network: None,
                    },
                )]),
            }),
            ..Default::default()
        },
        ConfigOverrides {
            cwd: Some(cwd.path().to_path_buf()),
            additional_writable_roots: vec![extra_root.path().to_path_buf()],
            ..Default::default()
        },
        codex_home.abs(),
    )
    .await?;

    assert_eq!(
        config
            .permissions
            .file_system_sandbox_policy()
            .glob_scan_max_depth,
        Some(2)
    );
    for root in [cwd.path(), extra_root.path()] {
        let expected_pattern = AbsolutePathBuf::resolve_path_against_base("**/*.env", root)
            .to_string_lossy()
            .into_owned();
        assert!(
            config
                .permissions
                .file_system_sandbox_policy()
                .entries
                .contains(&FileSystemSandboxEntry {
                    path: FileSystemPath::GlobPattern {
                        pattern: expected_pattern,
                    },
                    access: FileSystemAccessMode::None,
                })
        );
    }
    assert!(
        !config
            .permissions
            .file_system_sandbox_policy()
            .entries
            .iter()
            .any(|entry| matches!(
                &entry.path,
                FileSystemPath::Special {
                    value: FileSystemSpecialPath::ProjectRoots { subpath: Some(subpath) },
                } if subpath == std::path::Path::new("**/*.env")
            )),
        "glob should compile to a filesystem pattern entry, not a literal filesystem entry"
    );
    Ok(())
}

#[tokio::test]
async fn permissions_profiles_require_default_permissions() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let cwd = TempDir::new()?;
    std::fs::write(cwd.path().join(".git"), "gitdir: nowhere")?;

    let err = Config::load_from_base_config_with_overrides(
        ConfigToml {
            permissions: Some(PermissionsToml {
                entries: BTreeMap::from([(
                    "workspace".to_string(),
                    PermissionProfileToml {
                        workspace_roots: None,
                        filesystem: Some(FilesystemPermissionsToml {
                            glob_scan_max_depth: None,
                            entries: BTreeMap::from([(
                                ":minimal".to_string(),
                                FilesystemPermissionToml::Access(FileSystemAccessMode::Read),
                            )]),
                        }),
                        network: None,
                    },
                )]),
            }),
            ..Default::default()
        },
        ConfigOverrides {
            cwd: Some(cwd.path().to_path_buf()),
            ..Default::default()
        },
        codex_home.abs(),
    )
    .await
    .expect_err("missing default_permissions should be rejected");

    assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
    assert_eq!(
        err.to_string(),
        "config defines `[permissions]` profiles but does not set `default_permissions`"
    );
    Ok(())
}

#[tokio::test]
async fn default_permissions_can_select_builtin_profile_without_permissions_table()
-> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let cwd = TempDir::new()?;

    let config = Config::load_from_base_config_with_overrides(
        ConfigToml {
            default_permissions: Some(BUILT_IN_PERMISSION_PROFILE_WORKSPACE.to_string()),
            ..Default::default()
        },
        ConfigOverrides {
            cwd: Some(cwd.path().to_path_buf()),
            ..Default::default()
        },
        codex_home.abs(),
    )
    .await?;

    let policy = config.permissions.file_system_sandbox_policy();
    assert_eq!(
        config
            .permissions
            .active_permission_profile()
            .as_ref()
            .map(|active| active.id.as_str()),
        Some(BUILT_IN_PERMISSION_PROFILE_WORKSPACE)
    );
    assert!(
        policy.can_write_path_with_cwd(cwd.path(), cwd.path()),
        "expected :workspace to allow writing the project root, policy: {policy:?}"
    );
    assert!(
        !policy.can_write_path_with_cwd(&cwd.path().join(".git"), cwd.path()),
        "expected :workspace to protect project metadata, policy: {policy:?}"
    );
    Ok(())
}

#[tokio::test]
async fn default_permissions_read_only_keeps_add_dir_read_only() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let cwd = TempDir::new()?;
    let extra_root = TempDir::new()?;
    let extra_root = extra_root.path().abs();

    let config = Config::load_from_base_config_with_overrides(
        ConfigToml {
            default_permissions: Some(BUILT_IN_PERMISSION_PROFILE_READ_ONLY.to_string()),
            ..Default::default()
        },
        ConfigOverrides {
            cwd: Some(cwd.path().to_path_buf()),
            additional_writable_roots: vec![extra_root.to_path_buf()],
            ..Default::default()
        },
        codex_home.abs(),
    )
    .await?;

    let policy = config.permissions.file_system_sandbox_policy();
    assert!(
        !policy.can_write_path_with_cwd(extra_root.as_path(), cwd.path()),
        "expected :read-only to stay read-only for runtime workspace roots, policy: {policy:?}"
    );
    assert_eq!(
        config.permissions.active_permission_profile(),
        Some(ActivePermissionProfile::new(
            BUILT_IN_PERMISSION_PROFILE_READ_ONLY,
        ))
    );
    Ok(())
}

#[tokio::test]
async fn workspace_profile_applies_rules_to_runtime_and_profile_workspace_roots()
-> std::io::Result<()> {
    let temp_dir = TempDir::new()?;
    let codex_home = temp_dir.path().join("codex-home");
    let cwd = temp_dir.path().join("frontend");
    let runtime_root = temp_dir.path().join("backend");
    let profile_root = temp_dir.path().join("shared");
    for root in [&cwd, &runtime_root, &profile_root] {
        std::fs::create_dir_all(root.join(".git"))?;
        std::fs::create_dir_all(root.join(".codex"))?;
    }

    let config = Config::load_from_base_config_with_overrides(
        ConfigToml {
            default_permissions: Some("dev".to_string()),
            permissions: Some(PermissionsToml {
                entries: BTreeMap::from([(
                    "dev".to_string(),
                    PermissionProfileToml {
                        workspace_roots: Some(WorkspaceRootsToml {
                            entries: BTreeMap::from([(
                                profile_root.to_string_lossy().into_owned(),
                                true,
                            )]),
                        }),
                        filesystem: Some(FilesystemPermissionsToml {
                            glob_scan_max_depth: None,
                            entries: BTreeMap::from([(
                                ":workspace_roots".to_string(),
                                FilesystemPermissionToml::Scoped(BTreeMap::from([
                                    (".".to_string(), FileSystemAccessMode::Write),
                                    (".git".to_string(), FileSystemAccessMode::Read),
                                    (".codex".to_string(), FileSystemAccessMode::Read),
                                ])),
                            )]),
                        }),
                        network: None,
                    },
                )]),
            }),
            ..Default::default()
        },
        ConfigOverrides {
            cwd: Some(cwd.clone()),
            additional_writable_roots: vec![runtime_root.clone()],
            ..Default::default()
        },
        codex_home.abs(),
    )
    .await?;

    let cwd_abs = cwd.abs();
    let runtime_root_abs = runtime_root.abs();
    let profile_root_abs = profile_root.abs();
    assert_eq!(
        config.workspace_roots,
        vec![cwd_abs.clone(), runtime_root_abs.clone()]
    );
    assert_eq!(
        config.permissions.workspace_roots(),
        &[cwd_abs.clone(), runtime_root_abs.clone()]
    );
    assert_eq!(
        config.effective_workspace_roots(),
        vec![
            cwd_abs.clone(),
            runtime_root_abs.clone(),
            profile_root_abs.clone()
        ]
    );

    let policy = config.permissions.file_system_sandbox_policy();
    for root in [cwd_abs, runtime_root_abs, profile_root_abs.clone()] {
        assert!(
            policy.can_write_path_with_cwd(root.as_path(), cwd.as_path()),
            "expected workspace root to be writable, policy: {policy:?}"
        );
        assert!(
            !policy.can_write_path_with_cwd(&root.join(".git"), cwd.as_path()),
            "expected .git carveout under {root:?}, policy: {policy:?}"
        );
        assert!(
            !policy.can_write_path_with_cwd(&root.join(".codex"), cwd.as_path()),
            "expected .codex carveout under {root:?}, policy: {policy:?}"
        );
    }
    assert_eq!(
        config.permissions.profile_workspace_roots(),
        std::slice::from_ref(&profile_root_abs)
    );
    assert_eq!(
        config.permissions.active_permission_profile(),
        Some(ActivePermissionProfile::new("dev"))
    );
    Ok(())
}

#[tokio::test]
async fn explicit_builtin_workspace_profile_ignores_legacy_workspace_write_settings()
-> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let cwd = TempDir::new()?;
    let extra_root = TempDir::new()?;

    let config = Config::load_from_base_config_with_overrides(
        ConfigToml {
            default_permissions: Some(BUILT_IN_PERMISSION_PROFILE_WORKSPACE.to_string()),
            sandbox_workspace_write: Some(SandboxWorkspaceWrite {
                writable_roots: vec![extra_root.path().abs()],
                network_access: true,
                exclude_tmpdir_env_var: true,
                exclude_slash_tmp: true,
            }),
            ..Default::default()
        },
        ConfigOverrides {
            cwd: Some(cwd.path().to_path_buf()),
            ..Default::default()
        },
        codex_home.abs(),
    )
    .await?;

    let policy = config.permissions.file_system_sandbox_policy();
    assert_eq!(
        config.permissions.network_sandbox_policy(),
        NetworkSandboxPolicy::Restricted
    );
    assert!(
        !policy.entries.iter().any(|entry| matches!(
            &entry.path,
            FileSystemPath::Path { path } if path.as_path() == extra_root.path()
        )),
        "explicit :workspace should not inherit sandbox_workspace_write roots as concrete grants, \
         policy: {policy:?}"
    );
    Ok(())
}

#[tokio::test]
async fn empty_config_defaults_to_builtin_profile_for_trusted_project() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let cwd = TempDir::new()?;
    let project_key = cwd.path().to_string_lossy().to_string();

    let config = Config::load_from_base_config_with_overrides(
        ConfigToml {
            projects: Some(HashMap::from([(
                project_key,
                ProjectConfig {
                    trust_level: Some(TrustLevel::Trusted),
                },
            )])),
            ..Default::default()
        },
        ConfigOverrides {
            cwd: Some(cwd.path().to_path_buf()),
            ..Default::default()
        },
        codex_home.abs(),
    )
    .await?;

    let policy = config.permissions.file_system_sandbox_policy();
    assert_eq!(
        config
            .permissions
            .active_permission_profile()
            .as_ref()
            .map(|active| active.id.as_str()),
        Some(if cfg!(target_os = "windows") {
            BUILT_IN_PERMISSION_PROFILE_READ_ONLY
        } else {
            BUILT_IN_PERMISSION_PROFILE_WORKSPACE
        })
    );
    if cfg!(target_os = "windows") {
        assert!(
            !policy.can_write_path_with_cwd(cwd.path(), cwd.path()),
            "expected trusted project fallback to stay read-only without Windows sandbox support, policy: {policy:?}"
        );
    } else {
        assert!(
            policy.can_write_path_with_cwd(cwd.path(), cwd.path()),
            "expected trusted project fallback to use :workspace, policy: {policy:?}"
        );
        assert!(
            !policy.can_write_path_with_cwd(&cwd.path().join(".codex"), cwd.path()),
            "expected :workspace metadata carveouts, policy: {policy:?}"
        );
    }
    Ok(())
}

#[tokio::test]
async fn implicit_builtin_workspace_profile_preserves_sandbox_workspace_write_settings()
-> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let cwd = TempDir::new()?;
    let extra_root = TempDir::new()?;
    let extra_root = extra_root.path().abs();
    let project_key = cwd.path().to_string_lossy().to_string();

    let config = Config::load_from_base_config_with_overrides(
        ConfigToml {
            projects: Some(HashMap::from([(
                project_key,
                ProjectConfig {
                    trust_level: Some(TrustLevel::Trusted),
                },
            )])),
            sandbox_workspace_write: Some(SandboxWorkspaceWrite {
                writable_roots: vec![extra_root.clone()],
                network_access: true,
                exclude_tmpdir_env_var: true,
                exclude_slash_tmp: false,
            }),
            windows: Some(WindowsToml {
                sandbox: Some(WindowsSandboxModeToml::Elevated),
                sandbox_private_desktop: None,
            }),
            ..Default::default()
        },
        ConfigOverrides {
            cwd: Some(cwd.path().to_path_buf()),
            ..Default::default()
        },
        codex_home.abs(),
    )
    .await?;

    let policy = config.permissions.file_system_sandbox_policy();
    assert!(
        policy.can_write_path_with_cwd(extra_root.as_path(), cwd.path()),
        "expected implicit :workspace to preserve sandbox_workspace_write.writable_roots, policy: {policy:?}"
    );
    assert_eq!(
        config.permissions.network_sandbox_policy(),
        NetworkSandboxPolicy::Enabled
    );
    assert_eq!(
        config.permissions.active_permission_profile(),
        None,
        "implicit :workspace cannot be faithfully re-selected when it includes \
         legacy sandbox_workspace_write settings"
    );
    match config.legacy_sandbox_policy() {
        SandboxPolicy::WorkspaceWrite {
            writable_roots,
            network_access,
            exclude_tmpdir_env_var,
            exclude_slash_tmp,
        } => {
            assert!(writable_roots.contains(&extra_root));
            assert!(network_access);
            assert!(exclude_tmpdir_env_var);
            assert!(!exclude_slash_tmp);
        }
        sandbox_policy => panic!("expected workspace-write projection, got {sandbox_policy:?}"),
    }
    Ok(())
}

#[tokio::test]
async fn implicit_builtin_workspace_profile_preserves_add_dir_metadata_carveouts()
-> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let cwd = TempDir::new()?;
    let extra_root = TempDir::new()?;
    for subpath in [".git", ".agents", ".codex"] {
        std::fs::create_dir_all(extra_root.path().join(subpath))?;
    }
    let project_key = cwd.path().to_string_lossy().to_string();

    let config = Config::load_from_base_config_with_overrides(
        ConfigToml {
            projects: Some(HashMap::from([(
                project_key,
                ProjectConfig {
                    trust_level: Some(TrustLevel::Trusted),
                },
            )])),
            windows: Some(WindowsToml {
                sandbox: Some(WindowsSandboxModeToml::Elevated),
                sandbox_private_desktop: None,
            }),
            ..Default::default()
        },
        ConfigOverrides {
            cwd: Some(cwd.path().to_path_buf()),
            additional_writable_roots: vec![extra_root.path().to_path_buf()],
            ..Default::default()
        },
        codex_home.abs(),
    )
    .await?;

    let policy = config.permissions.file_system_sandbox_policy();
    let extra_root = extra_root.path().abs();
    assert!(
        policy.can_write_path_with_cwd(extra_root.as_path(), cwd.path()),
        "expected implicit :workspace to preserve additional writable roots, policy: {policy:?}"
    );
    for subpath in [".git", ".agents", ".codex"] {
        assert!(
            !policy.can_write_path_with_cwd(&extra_root.join(subpath), cwd.path()),
            "expected implicit :workspace to preserve legacy metadata carveout for {subpath}, \
             policy: {policy:?}"
        );
    }
    Ok(())
}

#[tokio::test]
async fn empty_config_defaults_to_builtin_read_only_without_trust_decision() -> std::io::Result<()>
{
    let codex_home = TempDir::new()?;
    let cwd = TempDir::new()?;

    let config = Config::load_from_base_config_with_overrides(
        ConfigToml::default(),
        ConfigOverrides {
            cwd: Some(cwd.path().to_path_buf()),
            ..Default::default()
        },
        codex_home.abs(),
    )
    .await?;

    let policy = config.permissions.file_system_sandbox_policy();
    assert!(
        policy.can_read_path_with_cwd(cwd.path(), cwd.path()),
        "expected :read-only to allow reads, policy: {policy:?}"
    );
    assert!(
        !policy.can_write_path_with_cwd(cwd.path(), cwd.path()),
        "expected :read-only to deny writes, policy: {policy:?}"
    );
    Ok(())
}

#[tokio::test]
async fn default_permissions_can_select_builtin_full_access_profile() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let cwd = TempDir::new()?;

    let config = Config::load_from_base_config_with_overrides(
        ConfigToml {
            default_permissions: Some(BUILT_IN_PERMISSION_PROFILE_DANGER_FULL_ACCESS.to_string()),
            ..Default::default()
        },
        ConfigOverrides {
            cwd: Some(cwd.path().to_path_buf()),
            ..Default::default()
        },
        codex_home.abs(),
    )
    .await?;

    assert_eq!(
        config.permissions.effective_permission_profile(),
        PermissionProfile::Disabled
    );
    assert_eq!(
        config
            .permissions
            .active_permission_profile()
            .as_ref()
            .map(|active| active.id.as_str()),
        Some(BUILT_IN_PERMISSION_PROFILE_DANGER_FULL_ACCESS)
    );
    Ok(())
}

#[tokio::test]
async fn legacy_danger_no_sandbox_is_rejected() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let cwd = TempDir::new()?;

    let err = Config::load_from_base_config_with_overrides(
        ConfigToml {
            default_permissions: Some(":danger-no-sandbox".to_string()),
            ..Default::default()
        },
        ConfigOverrides {
            cwd: Some(cwd.path().to_path_buf()),
            ..Default::default()
        },
        codex_home.abs(),
    )
    .await
    .expect_err("legacy full-access alias should be rejected");

    assert_eq!(
        err.to_string(),
        "default_permissions refers to unknown built-in profile `:danger-no-sandbox`"
    );
    Ok(())
}

#[tokio::test]
async fn user_defined_permission_profile_names_cannot_use_builtin_prefix() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let cwd = TempDir::new()?;

    let err = Config::load_from_base_config_with_overrides(
        ConfigToml {
            default_permissions: Some(":custom".to_string()),
            permissions: Some(PermissionsToml {
                entries: BTreeMap::from([(
                    ":custom".to_string(),
                    PermissionProfileToml::default(),
                )]),
            }),
            ..Default::default()
        },
        ConfigOverrides {
            cwd: Some(cwd.path().to_path_buf()),
            ..Default::default()
        },
        codex_home.abs(),
    )
    .await
    .expect_err("reserved profile name should be rejected");

    assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
    assert_eq!(
        err.to_string(),
        "permissions profile `:custom` uses a reserved built-in profile prefix"
    );
    Ok(())
}

#[tokio::test]
async fn unknown_builtin_permission_profile_name_is_rejected() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let cwd = TempDir::new()?;

    let err = Config::load_from_base_config_with_overrides(
        ConfigToml {
            default_permissions: Some(":unknown".to_string()),
            ..Default::default()
        },
        ConfigOverrides {
            cwd: Some(cwd.path().to_path_buf()),
            ..Default::default()
        },
        codex_home.abs(),
    )
    .await
    .expect_err("unknown built-in profile name should be rejected");

    assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
    assert_eq!(
        err.to_string(),
        "default_permissions refers to unknown built-in profile `:unknown`"
    );
    Ok(())
}

#[tokio::test]
async fn permissions_profiles_allow_direct_write_roots_outside_workspace_root()
-> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let cwd = TempDir::new()?;
    std::fs::write(cwd.path().join(".git"), "gitdir: nowhere")?;
    let external_write_dir = TempDir::new()?;
    let external_write_path =
        AbsolutePathBuf::from_absolute_path(std::fs::canonicalize(external_write_dir.path())?)?;

    let config = Config::load_from_base_config_with_overrides(
        ConfigToml {
            default_permissions: Some("workspace".to_string()),
            permissions: Some(PermissionsToml {
                entries: BTreeMap::from([(
                    "workspace".to_string(),
                    PermissionProfileToml {
                        workspace_roots: None,
                        filesystem: Some(FilesystemPermissionsToml {
                            glob_scan_max_depth: None,
                            entries: BTreeMap::from([(
                                external_write_path.to_string_lossy().into_owned(),
                                FilesystemPermissionToml::Access(FileSystemAccessMode::Write),
                            )]),
                        }),
                        network: None,
                    },
                )]),
            }),
            ..Default::default()
        },
        ConfigOverrides {
            cwd: Some(cwd.path().to_path_buf()),
            ..Default::default()
        },
        codex_home.abs(),
    )
    .await?;

    let memories_root = AbsolutePathBuf::from_absolute_path(std::fs::canonicalize(
        codex_home.path().join("memories"),
    )?)?;
    assert!(
        config
            .permissions
            .file_system_sandbox_policy()
            .can_write_path_with_cwd(external_write_path.as_path(), cwd.path())
    );
    assert_eq!(
        &config.legacy_sandbox_policy(),
        &SandboxPolicy::WorkspaceWrite {
            writable_roots: vec![external_write_path, memories_root],
            network_access: false,
            exclude_tmpdir_env_var: true,
            exclude_slash_tmp: true,
        }
    );
    Ok(())
}

#[tokio::test]
async fn permissions_profiles_reject_nested_entries_for_non_workspace_roots() -> std::io::Result<()>
{
    let codex_home = TempDir::new()?;
    let cwd = TempDir::new()?;
    std::fs::write(cwd.path().join(".git"), "gitdir: nowhere")?;

    let err = Config::load_from_base_config_with_overrides(
        ConfigToml {
            default_permissions: Some("workspace".to_string()),
            permissions: Some(PermissionsToml {
                entries: BTreeMap::from([(
                    "workspace".to_string(),
                    PermissionProfileToml {
                        workspace_roots: None,
                        filesystem: Some(FilesystemPermissionsToml {
                            glob_scan_max_depth: None,
                            entries: BTreeMap::from([(
                                ":minimal".to_string(),
                                FilesystemPermissionToml::Scoped(BTreeMap::from([(
                                    "docs".to_string(),
                                    FileSystemAccessMode::Read,
                                )])),
                            )]),
                        }),
                        network: None,
                    },
                )]),
            }),
            ..Default::default()
        },
        ConfigOverrides {
            cwd: Some(cwd.path().to_path_buf()),
            ..Default::default()
        },
        codex_home.abs(),
    )
    .await
    .expect_err("nested entries outside :workspace_roots should be rejected");

    assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
    assert_eq!(
        err.to_string(),
        "filesystem path `:minimal` does not support nested entries"
    );
    Ok(())
}

async fn load_workspace_permission_profile(
    profile: PermissionProfileToml,
) -> std::io::Result<Config> {
    let codex_home = TempDir::new()?;
    let cwd = TempDir::new()?;
    std::fs::write(cwd.path().join(".git"), "gitdir: nowhere")?;

    Config::load_from_base_config_with_overrides(
        ConfigToml {
            default_permissions: Some("workspace".to_string()),
            permissions: Some(PermissionsToml {
                entries: BTreeMap::from([("workspace".to_string(), profile)]),
            }),
            ..Default::default()
        },
        ConfigOverrides {
            cwd: Some(cwd.path().to_path_buf()),
            ..Default::default()
        },
        codex_home.abs(),
    )
    .await
}

#[tokio::test]
async fn permissions_profiles_allow_unknown_special_paths() -> std::io::Result<()> {
    let config = load_workspace_permission_profile(PermissionProfileToml {
        workspace_roots: None,
        filesystem: Some(FilesystemPermissionsToml {
            glob_scan_max_depth: None,
            entries: BTreeMap::from([(
                ":future_special_path".to_string(),
                FilesystemPermissionToml::Access(FileSystemAccessMode::Read),
            )]),
        }),
        network: None,
    })
    .await?;

    assert_eq!(
        config.permissions.file_system_sandbox_policy(),
        FileSystemSandboxPolicy::restricted(vec![FileSystemSandboxEntry {
            path: FileSystemPath::Special {
                value: FileSystemSpecialPath::unknown(
                    ":future_special_path",
                    /*subpath*/ None
                ),
            },
            access: FileSystemAccessMode::Read,
        }]),
    );
    assert_eq!(
        &config.legacy_sandbox_policy(),
        &SandboxPolicy::ReadOnly {
            network_access: false,
        }
    );
    assert!(
        config.startup_warnings.iter().any(|warning| warning.contains(
            "Configured filesystem path `:future_special_path` is not recognized by this version of Hepta and will be ignored."
        )),
        "{:?}",
        config.startup_warnings
    );
    Ok(())
}

#[tokio::test]
async fn permissions_profiles_allow_unknown_special_paths_with_nested_entries()
-> std::io::Result<()> {
    let config = load_workspace_permission_profile(PermissionProfileToml {
        workspace_roots: None,
        filesystem: Some(FilesystemPermissionsToml {
            glob_scan_max_depth: None,
            entries: BTreeMap::from([(
                ":future_special_path".to_string(),
                FilesystemPermissionToml::Scoped(BTreeMap::from([(
                    "docs".to_string(),
                    FileSystemAccessMode::Read,
                )])),
            )]),
        }),
        network: None,
    })
    .await?;

    assert_eq!(
        config.permissions.file_system_sandbox_policy(),
        FileSystemSandboxPolicy::restricted(vec![FileSystemSandboxEntry {
            path: FileSystemPath::Special {
                value: FileSystemSpecialPath::unknown(":future_special_path", Some("docs".into())),
            },
            access: FileSystemAccessMode::Read,
        }]),
    );
    assert!(
        config.startup_warnings.iter().any(|warning| warning.contains(
            "Configured filesystem path `:future_special_path` with nested entry `docs` is not recognized by this version of Hepta and will be ignored."
        )),
        "{:?}",
        config.startup_warnings
    );
    Ok(())
}

#[tokio::test]
async fn permissions_profiles_allow_missing_filesystem_with_warning() -> std::io::Result<()> {
    let config = load_workspace_permission_profile(PermissionProfileToml {
        workspace_roots: None,
        filesystem: None,
        network: None,
    })
    .await?;

    assert_eq!(
        config.permissions.file_system_sandbox_policy(),
        FileSystemSandboxPolicy::restricted(Vec::new())
    );
    assert_eq!(
        &config.legacy_sandbox_policy(),
        &SandboxPolicy::ReadOnly {
            network_access: false,
        }
    );
    assert!(
        config.startup_warnings.iter().any(|warning| warning.contains(
            "Permissions profile `workspace` does not define any recognized filesystem entries for this version of Hepta."
        )),
        "{:?}",
        config.startup_warnings
    );
    Ok(())
}

#[tokio::test]
async fn permissions_profiles_allow_empty_filesystem_with_warning() -> std::io::Result<()> {
    let config = load_workspace_permission_profile(PermissionProfileToml {
        workspace_roots: None,
        filesystem: Some(FilesystemPermissionsToml {
            glob_scan_max_depth: None,
            entries: BTreeMap::new(),
        }),
        network: None,
    })
    .await?;

    assert_eq!(
        config.permissions.file_system_sandbox_policy(),
        FileSystemSandboxPolicy::restricted(Vec::new())
    );
    assert!(
        config.startup_warnings.iter().any(|warning| warning.contains(
            "Permissions profile `workspace` does not define any recognized filesystem entries for this version of Hepta."
        )),
        "{:?}",
        config.startup_warnings
    );
    Ok(())
}

#[tokio::test]
async fn permissions_profiles_reject_workspace_root_parent_traversal() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let cwd = TempDir::new()?;
    std::fs::write(cwd.path().join(".git"), "gitdir: nowhere")?;

    let err = Config::load_from_base_config_with_overrides(
        ConfigToml {
            default_permissions: Some("workspace".to_string()),
            permissions: Some(PermissionsToml {
                entries: BTreeMap::from([(
                    "workspace".to_string(),
                    PermissionProfileToml {
                        workspace_roots: None,
                        filesystem: Some(FilesystemPermissionsToml {
                            glob_scan_max_depth: None,
                            entries: BTreeMap::from([(
                                ":workspace_roots".to_string(),
                                FilesystemPermissionToml::Scoped(BTreeMap::from([(
                                    "../sibling".to_string(),
                                    FileSystemAccessMode::Read,
                                )])),
                            )]),
                        }),
                        network: None,
                    },
                )]),
            }),
            ..Default::default()
        },
        ConfigOverrides {
            cwd: Some(cwd.path().to_path_buf()),
            ..Default::default()
        },
        codex_home.abs(),
    )
    .await
    .expect_err("parent traversal should be rejected for project root subpaths");

    assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
    assert_eq!(
        err.to_string(),
        "filesystem subpath `../sibling` must be a descendant path without `.` or `..` components"
    );
    Ok(())
}

#[tokio::test]
async fn permissions_profiles_allow_network_enablement() -> std::io::Result<()> {
    let codex_home = TempDir::new()?;
    let cwd = TempDir::new()?;
    std::fs::write(cwd.path().join(".git"), "gitdir: nowhere")?;

    let config = Config::load_from_base_config_with_overrides(
        ConfigToml {
            default_permissions: Some("workspace".to_string()),
            permissions: Some(PermissionsToml {
                entries: BTreeMap::from([(
                    "workspace".to_string(),
                    PermissionProfileToml {
                        workspace_roots: None,
                        filesystem: Some(FilesystemPermissionsToml {
                            glob_scan_max_depth: None,
                            entries: BTreeMap::from([(
                                ":minimal".to_string(),
                                FilesystemPermissionToml::Access(FileSystemAccessMode::Read),
                            )]),
                        }),
                        network: Some(NetworkToml {
                            enabled: Some(true),
                            ..Default::default()
                        }),
                    },
                )]),
            }),
            ..Default::default()
        },
        ConfigOverrides {
            cwd: Some(cwd.path().to_path_buf()),
            ..Default::default()
        },
        codex_home.abs(),
    )
    .await?;

    assert!(
        config.permissions.network_sandbox_policy().is_enabled(),
        "expected network sandbox policy to be enabled",
    );
    assert!(config.legacy_sandbox_policy().has_full_network_access());
    Ok(())
}

#[test]
fn tui_theme_deserializes_from_toml() {
    let cfg = r#"
[tui]
theme = "dracula"
"#;
    let parsed = toml::from_str::<ConfigToml>(cfg).expect("TOML deserialization should succeed");
    assert_eq!(
        parsed.tui.as_ref().and_then(|t| t.theme.as_deref()),
        Some("dracula"),
    );
}

#[test]
fn tui_theme_defaults_to_none() {
    let cfg = r#"
[tui]
"#;
    let parsed = toml::from_str::<ConfigToml>(cfg).expect("TOML deserialization should succeed");
    assert_eq!(parsed.tui.as_ref().and_then(|t| t.theme.as_deref()), None);
}

#[test]
fn tui_session_picker_view_deserializes_from_toml() {
    let cfg = r#"
[tui]
session_picker_view = "dense"
"#;
    let parsed = toml::from_str::<ConfigToml>(cfg).expect("TOML deserialization should succeed");
    assert_eq!(
        parsed.tui.as_ref().and_then(|t| t.session_picker_view),
        Some(SessionPickerViewMode::Dense),
    );
}

#[test]
fn tui_pet_deserializes_from_toml() {
    let cfg = r#"
[tui]
pet = "chefito"
"#;
    let parsed = toml::from_str::<ConfigToml>(cfg).expect("TOML deserialization should succeed");
    assert_eq!(
        parsed.tui.as_ref().and_then(|t| t.pet.as_deref()),
        Some("chefito"),
    );
}

#[test]
fn tui_session_picker_view_defaults_to_none() {
    let cfg = r#"
[tui]
"#;
    let parsed = toml::from_str::<ConfigToml>(cfg).expect("TOML deserialization should succeed");
    assert_eq!(
        parsed.tui.as_ref().and_then(|t| t.session_picker_view),
        None,
    );
}

#[test]
fn tui_pet_defaults_to_none() {
    let cfg = r#"
[tui]
"#;
    let parsed = toml::from_str::<ConfigToml>(cfg).expect("TOML deserialization should succeed");
    assert_eq!(parsed.tui.as_ref().and_then(|t| t.pet.as_deref()), None);
}

#[test]
fn tui_pet_anchor_deserializes_from_toml() {
    let cfg = r#"
[tui]
pet_anchor = "screen-bottom"
"#;
    let parsed = toml::from_str::<ConfigToml>(cfg).expect("TOML deserialization should succeed");
    assert_eq!(
        parsed.tui.as_ref().map(|t| t.pet_anchor),
        Some(TuiPetAnchor::ScreenBottom),
    );
}

#[test]
fn tui_pet_anchor_defaults_to_composer() {
    let cfg = r#"
[tui]
"#;
    let parsed = toml::from_str::<ConfigToml>(cfg).expect("TOML deserialization should succeed");
    assert_eq!(
        parsed.tui.as_ref().map(|t| t.pet_anchor),
        Some(TuiPetAnchor::Composer),
    );
}

#[test]
fn tui_pet_anchor_rejects_unknown_value() {
    let cfg = r#"
[tui]
pet_anchor = "bottom"
"#;
    let err = toml::from_str::<ConfigToml>(cfg).expect_err("reject unknown pet anchor");
    let err = err.to_string();
    assert!(
        err.contains("unknown variant `bottom`")
            && err.contains("composer")
            && err.contains("screen-bottom"),
        "unexpected error: {err}"
    );
}
