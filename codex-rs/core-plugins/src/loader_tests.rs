use super::*;
use crate::manifest::load_plugin_manifest;
use codex_config::ConfigLayerEntry;
use codex_config::ConfigLayerSource;
use codex_config::ConfigRequirements;
use codex_config::ConfigRequirementsToml;
use codex_plugin::PluginId;
use codex_utils_plugins::AGENT_PLUGIN_SCHEMA_URI;
use pretty_assertions::assert_eq;
use tempfile::TempDir;

fn user_config_path(temp_dir: &TempDir, file_name: &str) -> AbsolutePathBuf {
    AbsolutePathBuf::from_absolute_path(temp_dir.path().join(file_name))
        .expect("test user config path should be absolute")
}

fn user_layer(path: AbsolutePathBuf, config: &str) -> ConfigLayerEntry {
    ConfigLayerEntry::new(
        ConfigLayerSource::User {
            file: path,
            profile: None,
        },
        toml::from_str(config).expect("user config toml"),
    )
}

#[test]
fn curated_marketplace_eligibility_follows_auth_target() {
    assert!(plugin_is_eligible_for_target_marketplace(
        "github@openai-curated",
        TargetCuratedMarketplace::OpenAi,
    ));
    assert!(!plugin_is_eligible_for_target_marketplace(
        "github@openai-api-curated",
        TargetCuratedMarketplace::OpenAi,
    ));
    assert!(!plugin_is_eligible_for_target_marketplace(
        "github@chatgpt-global",
        TargetCuratedMarketplace::OpenAi,
    ));

    assert!(plugin_is_eligible_for_target_marketplace(
        "github@openai-curated",
        TargetCuratedMarketplace::OpenAiWithRemote,
    ));
    assert!(plugin_is_eligible_for_target_marketplace(
        "github@chatgpt-global",
        TargetCuratedMarketplace::OpenAiWithRemote,
    ));
    assert!(!plugin_is_eligible_for_target_marketplace(
        "github@openai-api-curated",
        TargetCuratedMarketplace::OpenAiWithRemote,
    ));

    assert!(plugin_is_eligible_for_target_marketplace(
        "github@openai-api-curated",
        TargetCuratedMarketplace::OpenAiApi,
    ));
    assert!(!plugin_is_eligible_for_target_marketplace(
        "github@openai-curated",
        TargetCuratedMarketplace::OpenAiApi,
    ));
    assert!(!plugin_is_eligible_for_target_marketplace(
        "github@chatgpt-global",
        TargetCuratedMarketplace::OpenAiApi,
    ));
    assert!(plugin_is_eligible_for_target_marketplace(
        "custom@example",
        TargetCuratedMarketplace::OpenAiApi,
    ));
}

#[test]
fn configured_plugins_from_stack_merges_user_layers() {
    let temp_dir = TempDir::new().expect("tempdir");
    let stack = ConfigLayerStack::new(
        vec![
            user_layer(
                user_config_path(&temp_dir, "config.toml"),
                "[plugins.base]\nenabled = true\n",
            ),
            user_layer(
                user_config_path(&temp_dir, "work.config.toml"),
                "[plugins.profile]\nenabled = false\n",
            ),
        ],
        ConfigRequirements::default(),
        ConfigRequirementsToml::default(),
    )
    .expect("valid config layer stack");

    let plugins = configured_plugins_from_stack(&stack, temp_dir.path());

    assert_eq!(
        plugins,
        HashMap::from([
            (
                "base".to_string(),
                PluginConfig {
                    enabled: true,
                    mcp_servers: HashMap::new(),
                },
            ),
            (
                "profile".to_string(),
                PluginConfig {
                    enabled: false,
                    mcp_servers: HashMap::new(),
                },
            ),
        ])
    );
}

#[test]
fn plugin_mcp_file_supports_mcp_servers_object_format() {
    let parsed = serde_json::from_str::<PluginMcpFile>(
        r#"{
  "mcpServers": {
    "sample": {
      "command": "sample-mcp"
    }
  }
}"#,
    )
    .expect("parse wrapped plugin mcp config")
    .into_mcp_servers();

    assert_eq!(
        parsed,
        HashMap::from([(
            "sample".to_string(),
            serde_json::json!({
                "command": "sample-mcp"
            }),
        )])
    );
}

#[test]
fn plugin_mcp_file_supports_mcp_servers_object_format_with_metadata() {
    let parsed = serde_json::from_str::<PluginMcpFile>(
        r#"{
  "$schema": "https://example.com/plugin-mcp.schema.json",
  "mcpServers": {
    "sample": {
      "command": "sample-mcp"
    }
  }
}"#,
    )
    .expect("parse plugin mcp config with metadata")
    .into_mcp_servers();

    assert_eq!(
        parsed,
        HashMap::from([(
            "sample".to_string(),
            serde_json::json!({
                "command": "sample-mcp"
            }),
        )])
    );
}

#[test]
fn plugin_mcp_file_supports_top_level_server_map_format() {
    let parsed = serde_json::from_str::<PluginMcpFile>(
        r#"{
  "linear": {
    "type": "http",
    "url": "https://mcp.linear.app/mcp"
  }
}"#,
    )
    .expect("parse flat plugin mcp config")
    .into_mcp_servers();

    assert_eq!(
        parsed,
        HashMap::from([(
            "linear".to_string(),
            serde_json::json!({
                "type": "http",
                "url": "https://mcp.linear.app/mcp"
            }),
        )])
    );
}

#[tokio::test]
async fn plugin_mcp_loading_uses_the_captured_manifest_paths() {
    let (_tmp, plugin_root) = plugin_root();
    write_manifest(
        &plugin_root,
        r#"{"name":"demo-plugin","mcpServers":"./mcp-a.json"}"#,
    );
    fs::write(
        plugin_root.join("mcp-a.json"),
        r#"{"a":{"command":"mcp-a"}}"#,
    )
    .expect("write first MCP config");
    fs::write(
        plugin_root.join("mcp-b.json"),
        r#"{"b":{"command":"mcp-b"}}"#,
    )
    .expect("write second MCP config");
    let manifest = load_plugin_manifest(plugin_root.as_path()).expect("initial manifest");

    write_manifest(
        &plugin_root,
        r#"{"name":"demo-plugin","mcpServers":"./mcp-b.json"}"#,
    );

    let mut names =
        load_plugin_mcp_servers_from_manifest_paths(plugin_root.as_path(), &manifest.paths)
            .await
            .into_keys()
            .collect::<Vec<_>>();
    names.sort_unstable();
    assert_eq!(names, vec!["a".to_string()]);
}

#[test]
fn curated_plugin_cache_version_shortens_full_git_sha() {
    assert_eq!(
        curated_plugin_cache_version("0123456789abcdef0123456789abcdef01234567"),
        "01234567"
    );
}

#[test]
fn curated_plugin_cache_version_preserves_non_git_sha_versions() {
    assert_eq!(
        curated_plugin_cache_version("export-backup"),
        "export-backup"
    );
    assert_eq!(curated_plugin_cache_version("0123456"), "0123456");
}

fn plugin_id() -> PluginId {
    PluginId::parse("demo-plugin@test-marketplace").expect("plugin id")
}

fn plugin_root() -> (tempfile::TempDir, AbsolutePathBuf) {
    let tmp = tempfile::tempdir().expect("tempdir");
    let plugin_root =
        AbsolutePathBuf::try_from(tmp.path().join("demo-plugin")).expect("plugin root");
    fs::create_dir_all(plugin_root.join(".codex-plugin")).expect("create manifest dir");
    fs::create_dir_all(plugin_root.join("hooks")).expect("create hooks dir");
    (tmp, plugin_root)
}

fn write_manifest(plugin_root: &AbsolutePathBuf, manifest: &str) {
    fs::write(plugin_root.join(".codex-plugin/plugin.json"), manifest).expect("write manifest");
}

fn write_hook_file(plugin_root: &AbsolutePathBuf, relative_path: &str, event: &str, command: &str) {
    fs::write(
        plugin_root.join(relative_path),
        format!(
            r#"{{
  "hooks": {{
    "{event}": [
      {{
        "hooks": [{{ "type": "command", "command": "{command}" }}]
      }}
    ]
  }}
}}"#
        ),
    )
    .expect("write hooks");
}

fn load_sources(plugin_root: &AbsolutePathBuf) -> (Vec<PluginHookSource>, Vec<String>) {
    let manifest = load_plugin_manifest(plugin_root.as_path()).expect("manifest");
    let plugin_data_root = AbsolutePathBuf::try_from(
        plugin_root
            .as_path()
            .parent()
            .expect("plugin root parent")
            .join("plugin-data"),
    )
    .expect("plugin data root");
    load_plugin_hooks(
        plugin_root,
        &plugin_id(),
        &plugin_data_root,
        &manifest.paths,
        manifest.skill_discovery_mode,
    )
}

fn assert_sources(sources: &[PluginHookSource], expected_relative_paths: &[&str]) {
    assert_eq!(
        sources
            .iter()
            .map(|source| source.plugin_id.clone())
            .collect::<Vec<_>>(),
        vec![plugin_id(); expected_relative_paths.len()]
    );
    assert_eq!(
        sources
            .iter()
            .map(|source| source.source_relative_path.as_str())
            .collect::<Vec<_>>(),
        expected_relative_paths
    );
    assert_eq!(
        sources
            .iter()
            .map(|source| source.hooks.handler_count())
            .collect::<Vec<_>>(),
        vec![1; expected_relative_paths.len()]
    );
}

#[tokio::test]
async fn agent_plugin_does_not_implicitly_activate_default_apps_or_hooks() {
    let (_tmp, plugin_root) = plugin_root();
    fs::write(
        plugin_root.join("plugin.json"),
        format!(r#"{{"$schema":"{AGENT_PLUGIN_SCHEMA_URI}","name":"demo-plugin"}}"#),
    )
    .expect("write Agent Plugin manifest");
    fs::write(
        plugin_root.join(".app.json"),
        r#"{"apps":{"default":{"id":"connector_default"}}}"#,
    )
    .expect("write default app config");
    write_hook_file(
        &plugin_root,
        "hooks/hooks.json",
        "PreToolUse",
        "echo default",
    );

    assert_eq!(
        load_plugin_apps(plugin_root.as_path()).await,
        Vec::<AppConnectorId>::new()
    );
    let (sources, warnings) = load_sources(&plugin_root);
    assert_eq!(sources, Vec::<PluginHookSource>::new());
    assert_eq!(warnings, Vec::<String>::new());
}

#[tokio::test]
async fn invalid_agent_plugin_manifest_does_not_fall_back_to_default_apps() {
    let (_tmp, plugin_root) = plugin_root();
    fs::write(
        plugin_root.join("plugin.json"),
        r#"{
  "$schema": "https://agent-plugins.org/schemas/2.0.0/plugin.schema.json",
  "name": "demo-plugin"
}"#,
    )
    .expect("write unsupported Agent Plugin manifest");
    fs::write(
        plugin_root.join(".app.json"),
        r#"{"apps":{"default":{"id":"connector_default"}}}"#,
    )
    .expect("write default app config");

    assert_eq!(
        load_plugin_apps(plugin_root.as_path()).await,
        Vec::<AppConnectorId>::new()
    );
}

#[cfg(unix)]
#[tokio::test]
async fn unreadable_agent_plugin_manifest_does_not_fall_back_to_default_apps() {
    use std::os::unix::fs::PermissionsExt;

    let (_tmp, plugin_root) = plugin_root();
    let root_manifest = plugin_root.join("plugin.json");
    fs::write(
        &root_manifest,
        format!(r#"{{"$schema":"{AGENT_PLUGIN_SCHEMA_URI}","name":"demo-plugin"}}"#),
    )
    .expect("write Agent Plugin manifest");
    fs::write(
        plugin_root.join(".app.json"),
        r#"{"apps":{"default":{"id":"connector_default"}}}"#,
    )
    .expect("write default app config");
    fs::set_permissions(&root_manifest, fs::Permissions::from_mode(0o000))
        .expect("make root manifest unreadable");

    assert_eq!(
        load_plugin_apps(plugin_root.as_path()).await,
        Vec::<AppConnectorId>::new()
    );

    fs::set_permissions(&root_manifest, fs::Permissions::from_mode(0o600))
        .expect("restore root manifest permissions");
}

#[tokio::test]
async fn agent_plugin_codex_overlay_explicitly_activates_apps_and_hooks() {
    let (_tmp, plugin_root) = plugin_root();
    fs::write(
        plugin_root.join("plugin.json"),
        format!(r#"{{"$schema":"{AGENT_PLUGIN_SCHEMA_URI}","name":"demo-plugin"}}"#),
    )
    .expect("write Agent Plugin manifest");
    write_manifest(
        &plugin_root,
        r#"{
  "apps": "./explicit.app.json",
  "hooks": "./hooks/explicit.json"
}"#,
    );
    fs::write(
        plugin_root.join("explicit.app.json"),
        r#"{"apps":{"explicit":{"id":"connector_explicit"}}}"#,
    )
    .expect("write explicit app config");
    write_hook_file(
        &plugin_root,
        "hooks/explicit.json",
        "PreToolUse",
        "echo explicit",
    );

    assert_eq!(
        load_plugin_apps(plugin_root.as_path()).await,
        vec![AppConnectorId("connector_explicit".to_string())]
    );
    let (sources, warnings) = load_sources(&plugin_root);
    assert_eq!(warnings, Vec::<String>::new());
    assert_sources(&sources, &["hooks/explicit.json"]);
}

#[test]
fn load_plugin_hooks_discovers_default_hooks_file() {
    let (_tmp, plugin_root) = plugin_root();
    write_manifest(&plugin_root, r#"{ "name": "demo-plugin" }"#);
    fs::write(
        plugin_root.join("hooks/hooks.json"),
        r#"{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [{ "type": "command", "command": "echo default" }]
      }
    ]
  }
}"#,
    )
    .expect("write hooks");

    let (sources, warnings) = load_sources(&plugin_root);

    assert_eq!(warnings, Vec::<String>::new());
    assert_sources(&sources, &["hooks/hooks.json"]);
}

#[test]
fn load_plugin_hooks_supports_manifest_hook_path() {
    let (_tmp, plugin_root) = plugin_root();
    write_manifest(
        &plugin_root,
        r#"{
  "name": "demo-plugin",
  "hooks": "./hooks/one.json"
}"#,
    );
    write_hook_file(&plugin_root, "hooks/one.json", "PreToolUse", "echo one");

    let (sources, warnings) = load_sources(&plugin_root);

    assert_eq!(warnings, Vec::<String>::new());
    assert_sources(&sources, &["hooks/one.json"]);
}

#[test]
fn load_plugin_hooks_manifest_paths_replace_default_hooks_file() {
    let (_tmp, plugin_root) = plugin_root();
    write_manifest(
        &plugin_root,
        r#"{
  "name": "demo-plugin",
  "hooks": ["./hooks/one.json", "./hooks/two.json"]
}"#,
    );
    write_hook_file(
        &plugin_root,
        "hooks/hooks.json",
        "PreToolUse",
        "echo ignored",
    );
    write_hook_file(&plugin_root, "hooks/one.json", "PreToolUse", "echo one");
    write_hook_file(&plugin_root, "hooks/two.json", "PostToolUse", "echo two");

    let (sources, warnings) = load_sources(&plugin_root);

    assert_eq!(warnings, Vec::<String>::new());
    assert_sources(&sources, &["hooks/one.json", "hooks/two.json"]);
}

#[test]
fn load_plugin_hooks_supports_inline_manifest_hooks() {
    let (_tmp, plugin_root) = plugin_root();
    write_manifest(
        &plugin_root,
        r#"{
  "name": "demo-plugin",
  "hooks": {
    "hooks": {
      "SessionStart": [
        {
          "matcher": "startup",
          "hooks": [{ "type": "command", "command": "echo inline" }]
        }
      ]
    }
  }
}"#,
    );

    let (sources, warnings) = load_sources(&plugin_root);

    assert_eq!(warnings, Vec::<String>::new());
    assert_sources(&sources, &[".codex-plugin/plugin.json#hooks[0]"]);
}

#[test]
fn load_plugin_hooks_reports_invalid_hook_file() {
    let (_tmp, plugin_root) = plugin_root();
    write_manifest(&plugin_root, r#"{ "name": "demo-plugin" }"#);
    fs::write(plugin_root.join("hooks/hooks.json"), "{ not-json").expect("write invalid hooks");

    let (sources, warnings) = load_sources(&plugin_root);

    assert_eq!(sources, Vec::<PluginHookSource>::new());
    assert_eq!(
        warnings,
        vec![format!(
            "failed to parse plugin hooks config {}: key must be a string at line 1 column 3",
            plugin_root.join("hooks/hooks.json").display()
        )]
    );
}

#[test]
fn load_plugin_hooks_supports_inline_manifest_hook_list() {
    let (_tmp, plugin_root) = plugin_root();
    write_manifest(
        &plugin_root,
        r#"{
  "name": "demo-plugin",
  "hooks": [
    {
      "hooks": {
        "SessionStart": [
          {
            "hooks": [{ "type": "command", "command": "echo inline one" }]
          }
        ]
      }
    },
    {
      "hooks": {
        "Stop": [
          {
            "hooks": [{ "type": "command", "command": "echo inline two" }]
          }
        ]
      }
    }
  ]
}"#,
    );

    let (sources, warnings) = load_sources(&plugin_root);

    assert_eq!(warnings, Vec::<String>::new());
    assert_sources(
        &sources,
        &[
            ".codex-plugin/plugin.json#hooks[0]",
            ".codex-plugin/plugin.json#hooks[1]",
        ],
    );
}

#[test]
fn agent_plugin_inline_overlay_hooks_preserve_frozen_overlay_provenance() {
    let (_tmp, plugin_root) = plugin_root();
    fs::write(
        plugin_root.join("plugin.json"),
        format!(r#"{{"$schema":"{AGENT_PLUGIN_SCHEMA_URI}","name":"demo-plugin"}}"#),
    )
    .expect("write Agent Plugin manifest");
    write_manifest(
        &plugin_root,
        r#"{
  "hooks": {
    "hooks": {
      "SessionStart": [
        {
          "hooks": [{ "type": "command", "command": "echo overlay" }]
        }
      ]
    }
  }
}"#,
    );

    let (sources, warnings) = load_sources(&plugin_root);

    assert_eq!(warnings, Vec::<String>::new());
    assert_sources(&sources, &[".codex-plugin/plugin.json#hooks[0]"]);
    assert_eq!(
        sources[0].source_path,
        plugin_root.join(".codex-plugin/plugin.json")
    );
}

#[test]
fn agent_plugin_inline_extension_hooks_preserve_frozen_extension_provenance() {
    let (_tmp, plugin_root) = plugin_root();
    fs::write(
        plugin_root.join("plugin.json"),
        format!(
            r#"{{
  "$schema": "{AGENT_PLUGIN_SCHEMA_URI}",
  "name": "demo-plugin",
  "extensions": {{
    "com.openai": {{
      "hooks": {{
        "hooks": {{
          "SessionStart": [
            {{
              "hooks": [{{ "type": "command", "command": "echo extension" }}]
            }}
          ]
        }}
      }}
    }}
  }}
}}"#
        ),
    )
    .expect("write Agent Plugin manifest");
    write_manifest(
        &plugin_root,
        r#"{
  "hooks": {
    "hooks": {
      "Stop": [{"hooks": [{ "type": "command", "command": "echo ignored" }]}]
    }
  }
}"#,
    );

    let (sources, warnings) = load_sources(&plugin_root);

    assert_eq!(warnings, Vec::<String>::new());
    assert_sources(&sources, &["plugin.json#extensions.com.openai.hooks[0]"]);
    assert_eq!(sources[0].source_path, plugin_root.join("plugin.json"));
}

#[test]
fn materialize_git_subdir_uses_sparse_checkout() {
    let codex_home = tempfile::tempdir().expect("create codex home");
    let repo = tempfile::tempdir().expect("create git repo");
    let plugin_dir = repo.path().join("plugins/toolkit");
    fs::create_dir_all(&plugin_dir).expect("create plugin directory");
    fs::create_dir_all(repo.path().join("plugins/other")).expect("create other plugin");
    fs::write(plugin_dir.join("marker.txt"), "toolkit").expect("write plugin marker");
    fs::write(repo.path().join("plugins/other/marker.txt"), "other").expect("write other marker");
    fs::write(repo.path().join("root.txt"), "root").expect("write root marker");

    run_git(&["init"], Some(repo.path())).expect("init git repo");
    run_git(
        &["config", "user.email", "test@example.com"],
        Some(repo.path()),
    )
    .expect("configure git email");
    run_git(&["config", "user.name", "Test User"], Some(repo.path())).expect("configure git name");
    run_git(&["add", "."], Some(repo.path())).expect("stage git repo");
    run_git(&["commit", "-m", "init"], Some(repo.path())).expect("commit git repo");

    let materialized = materialize_marketplace_plugin_source(
        codex_home.path(),
        &MarketplacePluginSource::Git {
            url: repo.path().display().to_string(),
            path: Some("plugins/toolkit".to_string()),
            ref_name: None,
            sha: None,
        },
    )
    .expect("materialize git source");

    assert_eq!(
        plugin_dir.file_name(),
        materialized.path.as_path().file_name()
    );
    assert!(materialized.path.as_path().join("marker.txt").is_file());
    let checkout_root = materialized
        .path
        .as_path()
        .parent()
        .and_then(Path::parent)
        .expect("materialized path should be nested under checkout root");
    assert!(!checkout_root.join("root.txt").exists());
    assert!(!checkout_root.join("plugins/other/marker.txt").exists());
}
