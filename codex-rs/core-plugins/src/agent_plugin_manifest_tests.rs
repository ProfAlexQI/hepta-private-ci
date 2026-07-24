use super::PluginManifest;
use super::PluginManifestHooks;
use super::load_plugin_manifest;
use codex_utils_absolute_path::AbsolutePathBuf;
use codex_utils_plugins::AGENT_PLUGIN_SCHEMA_URI;
use codex_utils_plugins::SkillDiscoveryMode;
use pretty_assertions::assert_eq;
use std::fs;
use std::path::Path;
use tempfile::tempdir;

fn write_agent_plugin_manifest(plugin_root: &Path, extra_fields: &str) {
    fs::create_dir_all(plugin_root).expect("create plugin root");
    fs::write(
        plugin_root.join("plugin.json"),
        format!(
            r#"{{
  "$schema": "{AGENT_PLUGIN_SCHEMA_URI}",
  "name": "demo-plugin"{extra_fields}
}}"#
        ),
    )
    .expect("write Agent Plugins manifest");
}

fn write_codex_overlay(plugin_root: &Path, contents: &str) {
    let overlay_path = plugin_root.join(".codex-plugin/plugin.json");
    fs::create_dir_all(overlay_path.parent().expect("overlay parent")).expect("create overlay dir");
    fs::write(overlay_path, contents).expect("write Codex overlay");
}

fn load_manifest(plugin_root: &Path) -> PluginManifest {
    load_plugin_manifest(plugin_root).expect("load plugin manifest")
}

#[test]
fn uses_portable_metadata_fixed_components_and_direct_child_discovery() {
    let tmp = tempdir().expect("tempdir");
    let plugin_root = tmp.path().join("demo-plugin");
    write_agent_plugin_manifest(
        &plugin_root,
        r#",
  "version": "release-2026-07",
  "description": "Portable demo",
  "author": {"name": "Portable Author"},
  "homepage": "https://example.com/plugin",
  "keywords": ["portable"]"#,
    );

    let manifest = load_manifest(&plugin_root);

    assert_eq!(manifest.name, "demo-plugin");
    assert_eq!(manifest.version.as_deref(), Some("release-2026-07"));
    assert_eq!(manifest.description.as_deref(), Some("Portable demo"));
    assert_eq!(manifest.keywords, vec!["portable"]);
    assert_eq!(
        manifest.paths.skills,
        Some(
            AbsolutePathBuf::from_absolute_path_checked(plugin_root.join("skills"))
                .expect("skills path")
        )
    );
    assert_eq!(
        manifest.paths.mcp_servers,
        Some(
            AbsolutePathBuf::from_absolute_path_checked(plugin_root.join("mcp.json"))
                .expect("MCP path")
        )
    );
    assert_eq!(
        manifest.skill_discovery_mode,
        SkillDiscoveryMode::DirectChildren
    );
    let interface = manifest.interface.expect("portable interface");
    assert_eq!(interface.display_name.as_deref(), Some("demo-plugin"));
    assert_eq!(
        interface.short_description.as_deref(),
        Some("Portable demo")
    );
    assert_eq!(interface.long_description.as_deref(), Some("Portable demo"));
    assert_eq!(interface.developer_name.as_deref(), Some("Portable Author"));
    assert_eq!(
        interface.website_url.as_deref(),
        Some("https://example.com/plugin")
    );
    assert_eq!(interface.category.as_deref(), Some("Other"));
}

#[test]
fn normalizes_absent_and_empty_optional_portable_metadata() {
    let tmp = tempdir().expect("tempdir");
    let plugin_root = tmp.path().join("demo-plugin");
    write_agent_plugin_manifest(
        &plugin_root,
        r#",
  "version": "",
  "description": " ",
  "author": {"name": " ", "email": ""},
  "homepage": "",
  "keywords": [""]"#,
    );

    let manifest = load_manifest(&plugin_root);

    assert_eq!(manifest.version, None);
    assert_eq!(manifest.description, None);
    assert_eq!(manifest.keywords, vec![""]);
    let interface = manifest.interface.expect("portable interface");
    assert_eq!(interface.display_name.as_deref(), Some("demo-plugin"));
    assert_eq!(interface.short_description, None);
    assert_eq!(interface.long_description, None);
    assert_eq!(interface.developer_name, None);
    assert_eq!(interface.website_url, None);
}

#[test]
fn codex_overlay_only_supplies_apps_hooks_and_interface() {
    let tmp = tempdir().expect("tempdir");
    let plugin_root = tmp.path().join("demo-plugin");
    write_agent_plugin_manifest(
        &plugin_root,
        r#",
  "version": "portable-version",
  "description": "Portable description""#,
    );
    write_codex_overlay(
        &plugin_root,
        r#"{
  "name": "different-name",
  "version": 42,
  "description": ["not", "portable"],
  "skills": [],
  "mcpServers": null,
  "apps": "./codex/apps.json",
  "hooks": "./codex/hooks.json",
  "toolSchemas": {
    "preview:mcp:demo@local:server": {
      "inputSchema": {"type": "object"},
      "outputSchema": {"type": "object"}
    }
  },
  "permissions": {
    "preview:mcp:demo@local:server": {"network": "local"}
  },
  "activationEvents": {
    "preview:mcp:demo@local:server": [{"type": "manual"}]
  },
  "toolPolicies": {
    "preview:mcp:demo@local:server": {
      "approval": {"kind": "onUse"},
      "ledger": {"required": true},
      "timeoutMs": 30000
    }
  },
  "interface": {"displayName": "Codex Demo"}
}"#,
    );

    let manifest = load_manifest(&plugin_root);

    assert_eq!(manifest.name, "demo-plugin");
    assert_eq!(manifest.version.as_deref(), Some("portable-version"));
    assert_eq!(
        manifest.description.as_deref(),
        Some("Portable description")
    );
    assert_eq!(
        manifest.paths.skills,
        Some(plugin_root.join("skills").try_into().expect("skills path"))
    );
    assert_eq!(
        manifest.paths.mcp_servers,
        Some(plugin_root.join("mcp.json").try_into().expect("MCP path"))
    );
    assert_eq!(
        manifest.paths.apps,
        Some(
            plugin_root
                .join("codex/apps.json")
                .try_into()
                .expect("apps path")
        )
    );
    assert_eq!(
        manifest.paths.hooks,
        Some(PluginManifestHooks::Paths(vec![
            plugin_root
                .join("codex/hooks.json")
                .try_into()
                .expect("hooks path")
        ]))
    );
    assert_eq!(
        manifest
            .interface
            .and_then(|interface| interface.display_name),
        Some("Codex Demo".to_string())
    );
    assert_eq!(
        manifest.tool_declarations.declared_candidate_ids(),
        Vec::<String>::new()
    );
}

#[test]
fn inline_openai_extension_wholly_precedes_codex_overlay() {
    let tmp = tempdir().expect("tempdir");
    let plugin_root = tmp.path().join("demo-plugin");
    write_agent_plugin_manifest(
        &plugin_root,
        r#",
  "extensions": {
    "com.openai": {
      "apps": "./inline/apps.json",
      "interface": {"displayName": "Inline Codex"}
    }
  }"#,
    );
    write_codex_overlay(
        &plugin_root,
        r#"{
  "apps": "./fallback/apps.json",
  "hooks": "./fallback/hooks.json",
  "interface": {"displayName": "Fallback Codex"}
}"#,
    );

    let manifest = load_manifest(&plugin_root);

    assert_eq!(
        manifest.paths.apps,
        Some(
            plugin_root
                .join("inline/apps.json")
                .try_into()
                .expect("inline apps path")
        )
    );
    assert_eq!(manifest.paths.hooks, None);
    assert_eq!(
        manifest
            .interface
            .and_then(|interface| interface.display_name),
        Some("Inline Codex".to_string())
    );
}

#[test]
fn non_object_openai_extension_uses_codex_overlay_fallback() {
    let tmp = tempdir().expect("tempdir");
    let plugin_root = tmp.path().join("demo-plugin");
    write_agent_plugin_manifest(
        &plugin_root,
        r#",
  "extensions": {"com.openai": false}"#,
    );
    write_codex_overlay(
        &plugin_root,
        r#"{"interface":{"displayName":"Fallback Codex"}}"#,
    );

    assert_eq!(
        load_manifest(&plugin_root)
            .interface
            .and_then(|interface| interface.display_name),
        Some("Fallback Codex".to_string())
    );
}

#[test]
fn unsupported_agent_schema_fails_closed_without_legacy_fallback() {
    let tmp = tempdir().expect("tempdir");
    let plugin_root = tmp.path().join("demo-plugin");
    fs::create_dir_all(&plugin_root).expect("create plugin root");
    fs::write(
        plugin_root.join("plugin.json"),
        r#"{
  "$schema": "https://agent-plugins.org/schemas/2.0.0/plugin.schema.json",
  "name": "demo-plugin"
}"#,
    )
    .expect("write unsupported manifest");
    write_codex_overlay(&plugin_root, r#"{"name":"legacy-fallback"}"#);

    assert_eq!(load_plugin_manifest(&plugin_root), None);
}

#[test]
fn unrelated_root_manifest_preserves_legacy_precedence() {
    let tmp = tempdir().expect("tempdir");
    let plugin_root = tmp.path().join("demo-plugin");
    fs::create_dir_all(plugin_root.join(".claude-plugin")).expect("create Claude dir");
    fs::write(
        plugin_root.join("plugin.json"),
        r#"{"name":"unrelated-package","private":true}"#,
    )
    .expect("write unrelated root manifest");
    write_codex_overlay(&plugin_root, r#"{"name":"codex-legacy"}"#);
    fs::write(
        plugin_root.join(".claude-plugin/plugin.json"),
        r#"{"name":"claude-legacy"}"#,
    )
    .expect("write Claude manifest");

    let manifest = load_manifest(&plugin_root);

    assert_eq!(manifest.name, "codex-legacy");
    assert_eq!(manifest.skill_discovery_mode, SkillDiscoveryMode::Recursive);
}

#[test]
fn rejects_invalid_names_and_wrong_portable_metadata_types() {
    let tmp = tempdir().expect("tempdir");
    let plugin_root = tmp.path().join("demo-plugin");
    write_agent_plugin_manifest(&plugin_root, r#", "homepage": 42"#);
    assert_eq!(load_plugin_manifest(&plugin_root), None);

    write_agent_plugin_manifest(&plugin_root, r#", "version": null"#);
    assert_eq!(load_plugin_manifest(&plugin_root), None);

    write_agent_plugin_manifest(&plugin_root, r#", "author": {"name": null}"#);
    assert_eq!(load_plugin_manifest(&plugin_root), None);

    fs::write(
        plugin_root.join("plugin.json"),
        format!(
            r#"{{"$schema":"{AGENT_PLUGIN_SCHEMA_URI}","name":"{}"}}"#,
            "a".repeat(65)
        ),
    )
    .expect("write overlong name");
    assert_eq!(load_plugin_manifest(&plugin_root), None);
}
