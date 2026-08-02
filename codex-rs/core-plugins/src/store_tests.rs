use super::*;
use codex_plugin::PluginId;
use pretty_assertions::assert_eq;
use tempfile::tempdir;

fn write_plugin_with_version(
    root: &Path,
    dir_name: &str,
    manifest_name: &str,
    manifest_version: Option<&str>,
) {
    let plugin_root = root.join(dir_name);
    fs::create_dir_all(plugin_root.join(".codex-plugin")).unwrap();
    fs::create_dir_all(plugin_root.join("skills")).unwrap();
    let version = manifest_version
        .map(|manifest_version| format!(r#","version":"{manifest_version}""#))
        .unwrap_or_default();
    fs::write(
        plugin_root.join(".codex-plugin/plugin.json"),
        format!(r#"{{"name":"{manifest_name}"{version}}}"#),
    )
    .unwrap();
    fs::write(plugin_root.join("skills/SKILL.md"), "skill").unwrap();
    fs::write(plugin_root.join(".mcp.json"), r#"{"mcpServers":{}}"#).unwrap();
}

fn write_plugin(root: &Path, dir_name: &str, manifest_name: &str) {
    write_plugin_with_version(
        root,
        dir_name,
        manifest_name,
        /*manifest_version*/ None,
    );
}

fn write_agent_plugin_with_version(
    root: &Path,
    dir_name: &str,
    manifest_name: &str,
    manifest_version: Option<&str>,
) {
    let plugin_root = root.join(dir_name);
    fs::create_dir_all(plugin_root.join("skills")).unwrap();
    let version = manifest_version
        .map(|manifest_version| format!(r#","version":"{manifest_version}""#))
        .unwrap_or_default();
    fs::write(
        plugin_root.join("plugin.json"),
        format!(
            r#"{{"$schema":"https://agent-plugins.org/schemas/1.0.0/plugin.schema.json","name":"{manifest_name}"{version}}}"#
        ),
    )
    .unwrap();
    fs::write(plugin_root.join("skills/SKILL.md"), "skill").unwrap();
}

#[test]
fn try_new_rejects_relative_codex_home() {
    let err = PluginStore::try_new(PathBuf::from("relative"))
        .expect_err("relative codex home should fail");
    let err = err.to_string().replace('\\', "/");

    assert_eq!(
        err,
        "failed to resolve plugin cache root: path is not absolute: relative/plugins/cache"
    );
}

#[test]
fn install_copies_plugin_into_default_marketplace() {
    let tmp = tempdir().unwrap();
    write_plugin(tmp.path(), "sample-plugin", "sample-plugin");
    let plugin_id = PluginId::new("sample-plugin".to_string(), "debug".to_string()).unwrap();

    let result = PluginStore::new(tmp.path().to_path_buf())
        .install(
            AbsolutePathBuf::try_from(tmp.path().join("sample-plugin")).unwrap(),
            plugin_id.clone(),
        )
        .unwrap();

    let installed_path = tmp.path().join("plugins/cache/debug/sample-plugin/local");
    assert_eq!(
        result,
        PluginInstallResult {
            plugin_id,
            plugin_version: "local".to_string(),
            installed_path: AbsolutePathBuf::try_from(installed_path.clone()).unwrap(),
        }
    );
    assert!(installed_path.join(".codex-plugin/plugin.json").is_file());
    assert!(installed_path.join("skills/SKILL.md").is_file());
}

#[test]
fn install_uses_manifest_name_for_destination_and_key() {
    let tmp = tempdir().unwrap();
    write_plugin(tmp.path(), "source-dir", "manifest-name");
    let plugin_id = PluginId::new("manifest-name".to_string(), "market".to_string()).unwrap();

    let result = PluginStore::new(tmp.path().to_path_buf())
        .install(
            AbsolutePathBuf::try_from(tmp.path().join("source-dir")).unwrap(),
            plugin_id.clone(),
        )
        .unwrap();

    assert_eq!(
        result,
        PluginInstallResult {
            plugin_id,
            plugin_version: "local".to_string(),
            installed_path: AbsolutePathBuf::try_from(
                tmp.path().join("plugins/cache/market/manifest-name/local"),
            )
            .unwrap(),
        }
    );
}

#[test]
fn install_supports_dotted_agent_plugin_names() {
    let tmp = tempdir().unwrap();
    write_plugin(tmp.path(), "source-dir", "acme.tools");
    let plugin_id = PluginId::new("acme.tools".to_string(), "market".to_string()).unwrap();

    let result = PluginStore::new(tmp.path().to_path_buf())
        .install(
            AbsolutePathBuf::try_from(tmp.path().join("source-dir")).unwrap(),
            plugin_id.clone(),
        )
        .unwrap();

    assert_eq!(
        result,
        PluginInstallResult {
            plugin_id,
            plugin_version: "local".to_string(),
            installed_path: AbsolutePathBuf::try_from(
                tmp.path().join("plugins/cache/market/acme.tools/local"),
            )
            .unwrap(),
        }
    );
}

#[test]
fn plugin_root_derives_path_from_key_and_version() {
    let tmp = tempdir().unwrap();
    let store = PluginStore::new(tmp.path().to_path_buf());
    let plugin_id = PluginId::new("sample".to_string(), "debug".to_string()).unwrap();

    assert_eq!(
        store.plugin_root(&plugin_id, "local").as_path(),
        tmp.path().join("plugins/cache/debug/sample/local")
    );
}

#[test]
fn plugin_data_root_derives_path_from_key() {
    let tmp = tempdir().unwrap();
    let store = PluginStore::new(tmp.path().to_path_buf());
    let plugin_id = PluginId::new("sample".to_string(), "debug".to_string()).unwrap();

    assert_eq!(
        store.plugin_data_root(&plugin_id).as_path(),
        tmp.path().join("plugins/data/sample-debug")
    );
}

#[test]
fn install_with_version_uses_requested_cache_version() {
    let tmp = tempdir().unwrap();
    write_plugin(tmp.path(), "sample-plugin", "sample-plugin");
    let plugin_id =
        PluginId::new("sample-plugin".to_string(), "openai-curated".to_string()).unwrap();
    let plugin_version = "0123456789abcdef".to_string();

    let result = PluginStore::new(tmp.path().to_path_buf())
        .install_with_version(
            AbsolutePathBuf::try_from(tmp.path().join("sample-plugin")).unwrap(),
            plugin_id.clone(),
            plugin_version.clone(),
        )
        .unwrap();

    let installed_path = tmp.path().join(format!(
        "plugins/cache/openai-curated/sample-plugin/{plugin_version}"
    ));
    assert_eq!(
        result,
        PluginInstallResult {
            plugin_id,
            plugin_version,
            installed_path: AbsolutePathBuf::try_from(installed_path.clone()).unwrap(),
        }
    );
    assert!(installed_path.join(".codex-plugin/plugin.json").is_file());
}

#[test]
fn install_uses_manifest_version_when_present() {
    let tmp = tempdir().unwrap();
    write_plugin_with_version(
        tmp.path(),
        "sample-plugin",
        "sample-plugin",
        Some("1.2.3-beta+7"),
    );
    let plugin_id = PluginId::new("sample-plugin".to_string(), "debug".to_string()).unwrap();

    let result = PluginStore::new(tmp.path().to_path_buf())
        .install(
            AbsolutePathBuf::try_from(tmp.path().join("sample-plugin")).unwrap(),
            plugin_id.clone(),
        )
        .unwrap();

    let installed_path = tmp
        .path()
        .join("plugins/cache/debug/sample-plugin/1.2.3-beta+7");
    assert_eq!(
        result,
        PluginInstallResult {
            plugin_id,
            plugin_version: "1.2.3-beta+7".to_string(),
            installed_path: AbsolutePathBuf::try_from(installed_path.clone()).unwrap(),
        }
    );
    assert!(installed_path.join(".codex-plugin/plugin.json").is_file());
}

#[test]
fn agent_plugin_without_version_uses_portable_default() {
    let tmp = tempdir().unwrap();
    write_agent_plugin_with_version(tmp.path(), "portable", "portable", None);

    assert_eq!(
        plugin_version_for_source(&tmp.path().join("portable")).unwrap(),
        "1.0.0"
    );
}

#[test]
fn agent_plugin_blank_version_uses_portable_default() {
    let tmp = tempdir().unwrap();
    write_agent_plugin_with_version(tmp.path(), "portable", "portable", Some("   "));

    assert_eq!(
        plugin_version_for_source(&tmp.path().join("portable")).unwrap(),
        "1.0.0"
    );
}

#[test]
fn agent_plugin_unsafe_version_uses_stable_directory_safe_digest() {
    let tmp = tempdir().unwrap();
    write_agent_plugin_with_version(tmp.path(), "portable", "portable", Some("release/2026"));

    assert_eq!(
        plugin_version_for_source(&tmp.path().join("portable")).unwrap(),
        "agent-plugins-e0c8bf07e64cc64c33ddff90"
    );
    assert_eq!(
        fs::read_to_string(tmp.path().join("portable/plugin.json")).unwrap(),
        r#"{"$schema":"https://agent-plugins.org/schemas/1.0.0/plugin.schema.json","name":"portable","version":"release/2026"}"#
    );
}

#[test]
fn agent_plugin_cross_platform_unsafe_versions_use_stable_directory_safe_digests() {
    let tmp = tempdir().unwrap();
    let mut unsafe_versions = vec![
        "CON".to_string(),
        "con.txt".to_string(),
        "PrN".to_string(),
        "AUX.plugin".to_string(),
        "nul".to_string(),
        "COM1".to_string(),
        "com9.log".to_string(),
        "LPT1".to_string(),
        "lPt9.ext".to_string(),
        "1.0.0.".to_string(),
        "1.0.0 ".to_string(),
    ];
    unsafe_versions.push("a".repeat(256));

    for (index, unsafe_version) in unsafe_versions.iter().enumerate() {
        let dir_name = format!("portable-{index}");
        write_agent_plugin_with_version(tmp.path(), &dir_name, &dir_name, Some(unsafe_version));

        let resolved = plugin_version_for_source(&tmp.path().join(dir_name)).unwrap();
        assert_ne!(&resolved, unsafe_version);
        assert!(resolved.starts_with("agent-plugins-"));
        assert_eq!(resolved.len(), "agent-plugins-".len() + 24);
        assert!(validate_plugin_version_segment(&resolved).is_ok());
    }

    assert_eq!(
        plugin_version_for_source(&tmp.path().join("portable-0")).unwrap(),
        "agent-plugins-a3dbc4b644a9a2c51e74509d"
    );
}

#[test]
fn cross_platform_version_validation_allows_non_device_names() {
    for version in ["COM0", "COM10", "LPT0", "LPT10", "CONSOLE", "NULLED"] {
        assert_eq!(validate_plugin_version_segment(version), Ok(()));
    }
}

#[test]
fn legacy_plugin_cross_platform_unsafe_version_is_rejected() {
    let tmp = tempdir().unwrap();
    write_plugin_with_version(tmp.path(), "portable", "portable", Some("CON"));

    let err = plugin_version_for_source(&tmp.path().join("portable"))
        .expect_err("legacy Windows device name must fail closed");

    assert_eq!(
        err.to_string(),
        "invalid plugin version: `CON` is reserved on Windows"
    );
}

#[test]
fn install_rejects_blank_manifest_version() {
    let tmp = tempdir().unwrap();
    write_plugin_with_version(tmp.path(), "sample-plugin", "sample-plugin", Some("   "));
    let plugin_id = PluginId::new("sample-plugin".to_string(), "debug".to_string()).unwrap();

    let err = PluginStore::new(tmp.path().to_path_buf())
        .install(
            AbsolutePathBuf::try_from(tmp.path().join("sample-plugin")).unwrap(),
            plugin_id,
        )
        .expect_err("blank manifest version should be rejected");
    let err = err.to_string().replace('\\', "/");

    assert_eq!(
        err,
        "invalid plugin version in plugin.json: must not be blank"
    );
}

#[test]
fn active_plugin_version_reads_version_directory_name() {
    let tmp = tempdir().unwrap();
    write_plugin(
        &tmp.path().join("plugins/cache/debug"),
        "sample-plugin/local",
        "sample-plugin",
    );
    let store = PluginStore::new(tmp.path().to_path_buf());
    let plugin_id = PluginId::new("sample-plugin".to_string(), "debug".to_string()).unwrap();

    assert_eq!(
        store.active_plugin_version(&plugin_id),
        Some("local".to_string())
    );
    assert_eq!(
        store.active_plugin_root(&plugin_id).unwrap().as_path(),
        tmp.path().join("plugins/cache/debug/sample-plugin/local")
    );
}

#[test]
fn active_plugin_version_prefers_default_local_version_when_multiple_versions_exist() {
    let tmp = tempdir().unwrap();
    write_plugin(
        &tmp.path().join("plugins/cache/debug"),
        "sample-plugin/0123456789abcdef",
        "sample-plugin",
    );
    write_plugin(
        &tmp.path().join("plugins/cache/debug"),
        "sample-plugin/local",
        "sample-plugin",
    );
    let store = PluginStore::new(tmp.path().to_path_buf());
    let plugin_id = PluginId::new("sample-plugin".to_string(), "debug".to_string()).unwrap();

    assert_eq!(
        store.active_plugin_version(&plugin_id),
        Some("local".to_string())
    );
}

#[test]
fn active_plugin_version_returns_latest_fallback_version_when_default_is_missing() {
    let tmp = tempdir().unwrap();
    write_plugin(
        &tmp.path().join("plugins/cache/debug"),
        "sample-plugin/0123456789abcdef",
        "sample-plugin",
    );
    write_plugin(
        &tmp.path().join("plugins/cache/debug"),
        "sample-plugin/fedcba9876543210",
        "sample-plugin",
    );
    let store = PluginStore::new(tmp.path().to_path_buf());
    let plugin_id = PluginId::new("sample-plugin".to_string(), "debug".to_string()).unwrap();

    assert_eq!(
        store.active_plugin_version(&plugin_id),
        Some("fedcba9876543210".to_string())
    );
}

#[test]
fn active_plugin_version_compares_semver_versions_semantically() {
    let tmp = tempdir().unwrap();
    write_plugin(
        &tmp.path().join("plugins/cache/debug"),
        "sample-plugin/9.0.0",
        "sample-plugin",
    );
    write_plugin(
        &tmp.path().join("plugins/cache/debug"),
        "sample-plugin/10.0.0",
        "sample-plugin",
    );
    let store = PluginStore::new(tmp.path().to_path_buf());
    let plugin_id = PluginId::new("sample-plugin".to_string(), "debug".to_string()).unwrap();

    assert_eq!(
        store.active_plugin_version(&plugin_id),
        Some("10.0.0".to_string())
    );
}

#[test]
fn active_marker_rejects_invalid_utf8_multiline_nonregular_and_missing_target() {
    let tmp = tempdir().unwrap();

    let invalid_utf8_root = tmp.path().join("invalid-utf8");
    fs::create_dir_all(invalid_utf8_root.join("1.0.0")).unwrap();
    fs::write(
        invalid_utf8_root.join(ACTIVE_PLUGIN_VERSION_FILE),
        [0xff, 0xfe],
    )
    .unwrap();
    assert_eq!(active_plugin_version_in_root(&invalid_utf8_root), None);

    let multiline_root = tmp.path().join("multiline");
    fs::create_dir_all(multiline_root.join("1.0.0")).unwrap();
    fs::write(
        multiline_root.join(ACTIVE_PLUGIN_VERSION_FILE),
        "1.0.0\n2.0.0\n",
    )
    .unwrap();
    assert_eq!(active_plugin_version_in_root(&multiline_root), None);

    let nonregular_root = tmp.path().join("nonregular");
    fs::create_dir_all(nonregular_root.join("1.0.0")).unwrap();
    fs::create_dir_all(nonregular_root.join(ACTIVE_PLUGIN_VERSION_FILE)).unwrap();
    assert_eq!(active_plugin_version_in_root(&nonregular_root), None);

    let missing_target_root = tmp.path().join("missing-target");
    fs::create_dir_all(&missing_target_root).unwrap();
    fs::write(
        missing_target_root.join(ACTIVE_PLUGIN_VERSION_FILE),
        "1.0.0\n",
    )
    .unwrap();
    assert_eq!(active_plugin_version_in_root(&missing_target_root), None);
}

#[cfg(unix)]
#[test]
fn active_marker_and_target_symlinks_fail_closed() {
    let tmp = tempdir().unwrap();
    let marker_link_root = tmp.path().join("marker-link");
    let marker_target = tmp.path().join("marker-target");
    fs::create_dir_all(marker_link_root.join("1.0.0")).unwrap();
    fs::write(&marker_target, "1.0.0\n").unwrap();
    std::os::unix::fs::symlink(
        &marker_target,
        marker_link_root.join(ACTIVE_PLUGIN_VERSION_FILE),
    )
    .unwrap();
    assert_eq!(active_plugin_version_in_root(&marker_link_root), None);

    let version_link_root = tmp.path().join("version-link");
    let version_target = tmp.path().join("version-target");
    fs::create_dir_all(&version_link_root).unwrap();
    fs::create_dir_all(&version_target).unwrap();
    fs::write(
        version_link_root.join(ACTIVE_PLUGIN_VERSION_FILE),
        "1.0.0\n",
    )
    .unwrap();
    std::os::unix::fs::symlink(&version_target, version_link_root.join("1.0.0")).unwrap();
    assert_eq!(active_plugin_version_in_root(&version_link_root), None);
}

#[test]
fn version_comparator_breaks_equal_semver_and_invalid_versions_deterministically() {
    assert_eq!(
        compare_plugin_versions("1.0.0+alpha", "1.0.0+beta"),
        Ordering::Less
    );
    assert_eq!(
        compare_plugin_versions("1.0.0+beta", "1.0.0+alpha"),
        Ordering::Greater
    );
    assert_eq!(
        compare_plugin_versions("release-9", "release-10"),
        Ordering::Greater
    );
    assert_eq!(
        compare_plugin_versions("release-10", "release-9"),
        Ordering::Less
    );
}

#[test]
fn install_with_new_version_publishes_marker_and_retains_old_generation() {
    let tmp = tempdir().unwrap();
    let store = PluginStore::new(tmp.path().to_path_buf());
    let plugin_id = PluginId::new("sample-plugin".to_string(), "debug".to_string()).unwrap();

    write_plugin_with_version(tmp.path(), "v1", "sample-plugin", Some("1.0.0"));
    store
        .install(
            AbsolutePathBuf::try_from(tmp.path().join("v1")).unwrap(),
            plugin_id.clone(),
        )
        .unwrap();
    let plugin_base_root = store.plugin_base_root(&plugin_id);
    fs::write(plugin_base_root.join("root-sentinel"), "keep plugin root").unwrap();

    write_plugin_with_version(tmp.path(), "v2", "sample-plugin", Some("2.0.0"));
    store
        .install(
            AbsolutePathBuf::try_from(tmp.path().join("v2")).unwrap(),
            plugin_id.clone(),
        )
        .unwrap();

    assert_eq!(
        store.active_plugin_version(&plugin_id),
        Some("2.0.0".to_string())
    );
    assert!(plugin_base_root.join("root-sentinel").is_file());
    assert!(plugin_base_root.join("2.0.0").is_dir());
    assert!(plugin_base_root.join("1.0.0").is_dir());
    assert_eq!(
        fs::read_to_string(plugin_base_root.join(ACTIVE_PLUGIN_VERSION_FILE)).unwrap(),
        "2.0.0\n"
    );
}

#[test]
fn retry_rejects_inactive_orphan_without_replacing_active_generation() {
    let tmp = tempdir().unwrap();
    let store = PluginStore::new(tmp.path().to_path_buf());
    let plugin_id = PluginId::new("sample-plugin".to_string(), "debug".to_string()).unwrap();
    write_plugin_with_version(tmp.path(), "v1", "sample-plugin", Some("1.0.0"));
    fs::write(tmp.path().join("v1/skills/SKILL.md"), "active-version-one").unwrap();
    store
        .install(
            AbsolutePathBuf::try_from(tmp.path().join("v1")).unwrap(),
            plugin_id.clone(),
        )
        .unwrap();

    let plugin_base_root = store.plugin_base_root(&plugin_id);
    let inactive_orphan = plugin_base_root.join("2.0.0");
    fs::create_dir_all(&inactive_orphan).unwrap();
    fs::write(inactive_orphan.join("orphan-sentinel"), "preserve orphan").unwrap();
    write_plugin_with_version(tmp.path(), "v2", "sample-plugin", Some("2.0.0"));
    fs::write(
        tmp.path().join("v2/skills/SKILL.md"),
        "candidate-version-two",
    )
    .unwrap();

    let err = store
        .install(
            AbsolutePathBuf::try_from(tmp.path().join("v2")).unwrap(),
            plugin_id.clone(),
        )
        .expect_err("retry must not replace an inactive orphan generation");

    assert!(
        err.to_string()
            .contains("inactive plugin cache version collision")
    );
    assert_eq!(
        store.active_plugin_version(&plugin_id),
        Some("1.0.0".to_string())
    );
    assert_eq!(
        fs::read_to_string(plugin_base_root.join(ACTIVE_PLUGIN_VERSION_FILE)).unwrap(),
        "1.0.0\n"
    );
    assert_eq!(
        fs::read_to_string(plugin_base_root.join("1.0.0/skills/SKILL.md")).unwrap(),
        "active-version-one"
    );
    assert_eq!(
        fs::read_to_string(inactive_orphan.join("orphan-sentinel")).unwrap(),
        "preserve orphan"
    );
}

#[cfg(unix)]
#[test]
fn install_with_new_version_activation_failure_preserves_existing_version() {
    use std::os::unix::fs::PermissionsExt;

    let tmp = tempdir().unwrap();
    let store = PluginStore::new(tmp.path().to_path_buf());
    let plugin_id = PluginId::new("sample-plugin".to_string(), "debug".to_string()).unwrap();
    write_plugin_with_version(tmp.path(), "v1", "sample-plugin", Some("1.0.0"));
    store
        .install(
            AbsolutePathBuf::try_from(tmp.path().join("v1")).unwrap(),
            plugin_id.clone(),
        )
        .unwrap();
    let plugin_base_root = store.plugin_base_root(&plugin_id);
    let original_permissions = fs::metadata(&plugin_base_root).unwrap().permissions();
    let mut blocked_permissions = original_permissions.clone();
    blocked_permissions.set_mode(original_permissions.mode() & !0o222);
    fs::set_permissions(&plugin_base_root, blocked_permissions).unwrap();

    write_plugin_with_version(tmp.path(), "v2", "sample-plugin", Some("2.0.0"));
    let result = store.install(
        AbsolutePathBuf::try_from(tmp.path().join("v2")).unwrap(),
        plugin_id.clone(),
    );
    fs::set_permissions(&plugin_base_root, original_permissions).unwrap();
    let err = result.expect_err("additive activation into a read-only root must fail");

    assert!(
        err.to_string()
            .contains("failed to activate updated plugin cache version")
    );
    assert_eq!(
        store.active_plugin_version(&plugin_id),
        Some("1.0.0".to_string())
    );
    assert!(plugin_base_root.join("1.0.0/skills/SKILL.md").is_file());
    assert!(!plugin_base_root.join("2.0.0").exists());
}

#[test]
fn mixed_semver_and_opaque_version_order_is_total_across_all_permutations() {
    let permutations = [
        ["2.0.0", "10.0.0", "15x"],
        ["2.0.0", "15x", "10.0.0"],
        ["10.0.0", "2.0.0", "15x"],
        ["10.0.0", "15x", "2.0.0"],
        ["15x", "2.0.0", "10.0.0"],
        ["15x", "10.0.0", "2.0.0"],
    ];

    for permutation in permutations {
        let selected = permutation
            .into_iter()
            .max_by(|left, right| compare_plugin_versions(left, right));
        assert_eq!(selected, Some("10.0.0"), "permutation {permutation:?}");
    }
    assert_eq!(compare_plugin_versions("10.0.0", "15x"), Ordering::Greater);
    assert_eq!(compare_plugin_versions("15x", "10.0.0"), Ordering::Less);
}

#[test]
fn explicit_marker_allows_safe_downgrade_without_deleting_newer_generation() {
    let tmp = tempdir().unwrap();
    let store = PluginStore::new(tmp.path().to_path_buf());
    let plugin_id = PluginId::new("sample-plugin".to_string(), "debug".to_string()).unwrap();
    write_plugin_with_version(tmp.path(), "v2", "sample-plugin", Some("2.0.0"));
    store
        .install(
            AbsolutePathBuf::try_from(tmp.path().join("v2")).unwrap(),
            plugin_id.clone(),
        )
        .unwrap();
    write_plugin_with_version(tmp.path(), "v1", "sample-plugin", Some("1.0.0"));

    store
        .install(
            AbsolutePathBuf::try_from(tmp.path().join("v1")).unwrap(),
            plugin_id.clone(),
        )
        .unwrap();

    let plugin_root = store.plugin_base_root(&plugin_id);
    assert_eq!(
        store.active_plugin_version(&plugin_id),
        Some("1.0.0".to_string())
    );
    assert!(plugin_root.join("1.0.0").is_dir());
    assert!(plugin_root.join("2.0.0").is_dir());
}

#[test]
fn reader_can_continue_using_old_generation_after_distinct_version_activation() {
    let tmp = tempdir().unwrap();
    let store = PluginStore::new(tmp.path().to_path_buf());
    let plugin_id = PluginId::new("sample-plugin".to_string(), "debug".to_string()).unwrap();
    write_plugin_with_version(tmp.path(), "v1", "sample-plugin", Some("1.0.0"));
    fs::write(tmp.path().join("v1/skills/SKILL.md"), "version-one").unwrap();
    store
        .install(
            AbsolutePathBuf::try_from(tmp.path().join("v1")).unwrap(),
            plugin_id.clone(),
        )
        .unwrap();
    let old_skill = store
        .active_plugin_root(&plugin_id)
        .unwrap()
        .join("skills/SKILL.md");
    let (ready_tx, ready_rx) = std::sync::mpsc::channel();
    let (activated_tx, activated_rx) = std::sync::mpsc::channel();
    let reader = std::thread::spawn(move || {
        assert_eq!(fs::read_to_string(&old_skill).unwrap(), "version-one");
        ready_tx.send(()).unwrap();
        activated_rx.recv().unwrap();
        fs::read_to_string(&old_skill)
    });
    ready_rx.recv().unwrap();
    write_plugin_with_version(tmp.path(), "v2", "sample-plugin", Some("2.0.0"));
    fs::write(tmp.path().join("v2/skills/SKILL.md"), "version-two").unwrap();

    store
        .install(
            AbsolutePathBuf::try_from(tmp.path().join("v2")).unwrap(),
            plugin_id.clone(),
        )
        .unwrap();
    activated_tx.send(()).unwrap();

    assert_eq!(reader.join().unwrap().unwrap(), "version-one");
    assert_eq!(
        store.active_plugin_version(&plugin_id),
        Some("2.0.0".to_string())
    );
    assert_eq!(
        fs::read_to_string(
            store
                .active_plugin_root(&plugin_id)
                .unwrap()
                .join("skills/SKILL.md")
        )
        .unwrap(),
        "version-two"
    );
}

#[test]
fn plugin_root_rejects_path_separators_in_key_segments() {
    let err = PluginId::parse("../../etc@debug").unwrap_err();
    assert_eq!(
        err.to_string(),
        "invalid plugin name: only ASCII letters, digits, `_`, `-`, and `.` are allowed in `../../etc@debug`"
    );

    let err = PluginId::parse("sample@../../etc").unwrap_err();
    assert_eq!(
        err.to_string(),
        "invalid marketplace name: only ASCII letters, digits, `_`, `-`, and `.` are allowed in `sample@../../etc`"
    );
}

#[test]
fn install_rejects_manifest_names_with_path_separators() {
    let tmp = tempdir().unwrap();
    write_plugin(tmp.path(), "source-dir", "../../etc");

    let err = PluginStore::new(tmp.path().to_path_buf())
        .install(
            AbsolutePathBuf::try_from(tmp.path().join("source-dir")).unwrap(),
            PluginId::new("source-dir".to_string(), "debug".to_string()).unwrap(),
        )
        .unwrap_err();

    assert_eq!(
        err.to_string(),
        "invalid plugin name: only ASCII letters, digits, `_`, `-`, and `.` are allowed"
    );
}

#[test]
fn install_rejects_marketplace_names_with_path_separators() {
    let err = PluginId::new("sample-plugin".to_string(), "../../etc".to_string()).unwrap_err();

    assert_eq!(
        err.to_string(),
        "invalid marketplace name: only ASCII letters, digits, `_`, `-`, and `.` are allowed"
    );
}

#[test]
fn install_rejects_manifest_names_that_do_not_match_marketplace_plugin_name() {
    let tmp = tempdir().unwrap();
    write_plugin(tmp.path(), "source-dir", "manifest-name");

    let err = PluginStore::new(tmp.path().to_path_buf())
        .install(
            AbsolutePathBuf::try_from(tmp.path().join("source-dir")).unwrap(),
            PluginId::new("different-name".to_string(), "debug".to_string()).unwrap(),
        )
        .unwrap_err();

    assert_eq!(
        err.to_string(),
        "plugin.json name `manifest-name` does not match marketplace plugin name `different-name`"
    );
}

#[cfg(unix)]
#[test]
fn install_rejects_source_root_symlink_without_replacing_existing_plugin() {
    let tmp = tempdir().unwrap();
    let store = PluginStore::new(tmp.path().to_path_buf());
    let plugin_id = PluginId::new("sample-plugin".to_string(), "debug".to_string()).unwrap();
    write_plugin_with_version(tmp.path(), "old-source", "sample-plugin", Some("1.0.0"));
    store
        .install(
            AbsolutePathBuf::try_from(tmp.path().join("old-source")).unwrap(),
            plugin_id.clone(),
        )
        .unwrap();
    write_plugin_with_version(tmp.path(), "new-source", "sample-plugin", Some("2.0.0"));
    let source_link = tmp.path().join("source-link");
    std::os::unix::fs::symlink(tmp.path().join("new-source"), &source_link).unwrap();

    let err = store
        .install(
            AbsolutePathBuf::try_from(source_link).unwrap(),
            plugin_id.clone(),
        )
        .expect_err("symlinked plugin root must fail closed");

    assert!(
        err.to_string()
            .contains("plugin source contains unsupported symbolic link")
    );
    assert_eq!(
        store.active_plugin_version(&plugin_id),
        Some("1.0.0".to_string())
    );
}

#[test]
fn install_rejects_non_directory_source_without_replacing_existing_plugin() {
    let tmp = tempdir().unwrap();
    let store = PluginStore::new(tmp.path().to_path_buf());
    let plugin_id = PluginId::new("sample-plugin".to_string(), "debug".to_string()).unwrap();
    write_plugin_with_version(tmp.path(), "old-source", "sample-plugin", Some("1.0.0"));
    store
        .install(
            AbsolutePathBuf::try_from(tmp.path().join("old-source")).unwrap(),
            plugin_id.clone(),
        )
        .unwrap();
    let source_file = tmp.path().join("not-a-directory");
    fs::write(&source_file, "not a plugin directory").unwrap();

    let err = store
        .install_with_version(
            AbsolutePathBuf::try_from(source_file).unwrap(),
            plugin_id.clone(),
            "2.0.0".to_string(),
        )
        .expect_err("non-directory plugin root must fail closed");

    assert!(err.to_string().contains("is not a directory"));
    assert_eq!(
        store.active_plugin_version(&plugin_id),
        Some("1.0.0".to_string())
    );
}

#[test]
fn install_rejects_missing_source_without_replacing_existing_plugin() {
    let tmp = tempdir().unwrap();
    let store = PluginStore::new(tmp.path().to_path_buf());
    let plugin_id = PluginId::new("sample-plugin".to_string(), "debug".to_string()).unwrap();
    write_plugin_with_version(tmp.path(), "old-source", "sample-plugin", Some("1.0.0"));
    store
        .install(
            AbsolutePathBuf::try_from(tmp.path().join("old-source")).unwrap(),
            plugin_id.clone(),
        )
        .unwrap();

    let err = store
        .install_with_version(
            AbsolutePathBuf::try_from(tmp.path().join("missing-source")).unwrap(),
            plugin_id.clone(),
            "2.0.0".to_string(),
        )
        .expect_err("missing plugin root must fail closed");

    assert!(err.to_string().contains("is not a directory"));
    assert_eq!(
        store.active_plugin_version(&plugin_id),
        Some("1.0.0".to_string())
    );
}

#[cfg(unix)]
#[test]
fn install_rejects_nested_symlink_without_replacing_existing_plugin() {
    let tmp = tempdir().unwrap();
    let store = PluginStore::new(tmp.path().to_path_buf());
    let plugin_id = PluginId::new("sample-plugin".to_string(), "debug".to_string()).unwrap();
    write_plugin_with_version(tmp.path(), "old-source", "sample-plugin", Some("1.0.0"));
    store
        .install(
            AbsolutePathBuf::try_from(tmp.path().join("old-source")).unwrap(),
            plugin_id.clone(),
        )
        .unwrap();
    write_plugin_with_version(tmp.path(), "new-source", "sample-plugin", Some("2.0.0"));
    let new_source = tmp.path().join("new-source");
    std::os::unix::fs::symlink(
        new_source.join("skills/SKILL.md"),
        new_source.join("linked-skill.md"),
    )
    .unwrap();

    let err = store
        .install(
            AbsolutePathBuf::try_from(new_source).unwrap(),
            plugin_id.clone(),
        )
        .expect_err("nested symlink must fail closed");

    assert!(
        err.to_string()
            .contains("plugin source contains unsupported symbolic link")
    );
    assert_eq!(
        store.active_plugin_version(&plugin_id),
        Some("1.0.0".to_string())
    );
}

#[cfg(unix)]
#[test]
fn install_rejects_nested_fifo_without_replacing_existing_plugin() {
    use std::ffi::CString;
    use std::os::unix::ffi::OsStrExt;

    let tmp = tempdir().unwrap();
    let store = PluginStore::new(tmp.path().to_path_buf());
    let plugin_id = PluginId::new("sample-plugin".to_string(), "debug".to_string()).unwrap();
    write_plugin_with_version(tmp.path(), "old-source", "sample-plugin", Some("1.0.0"));
    store
        .install(
            AbsolutePathBuf::try_from(tmp.path().join("old-source")).unwrap(),
            plugin_id.clone(),
        )
        .unwrap();
    write_plugin_with_version(tmp.path(), "new-source", "sample-plugin", Some("2.0.0"));
    let new_source = tmp.path().join("new-source");
    let fifo_path = new_source.join("events.fifo");
    let fifo_path_c = CString::new(fifo_path.as_os_str().as_bytes()).unwrap();
    let result = unsafe { libc::mkfifo(fifo_path_c.as_ptr(), 0o600) };
    assert_eq!(result, 0, "mkfifo failed: {}", io::Error::last_os_error());

    let err = store
        .install(
            AbsolutePathBuf::try_from(new_source).unwrap(),
            plugin_id.clone(),
        )
        .expect_err("nested FIFO must fail closed");

    assert!(
        err.to_string()
            .contains("plugin source contains unsupported file type")
    );
    assert_eq!(
        store.active_plugin_version(&plugin_id),
        Some("1.0.0".to_string())
    );
}

#[cfg(unix)]
#[test]
fn install_rejects_nested_socket_without_replacing_existing_plugin() {
    use std::os::unix::net::UnixListener;

    let tmp = tempdir().unwrap();
    let store = PluginStore::new(tmp.path().to_path_buf());
    let plugin_id = PluginId::new("sample-plugin".to_string(), "debug".to_string()).unwrap();
    write_plugin_with_version(tmp.path(), "old-source", "sample-plugin", Some("1.0.0"));
    store
        .install(
            AbsolutePathBuf::try_from(tmp.path().join("old-source")).unwrap(),
            plugin_id.clone(),
        )
        .unwrap();
    write_plugin_with_version(tmp.path(), "new-source", "sample-plugin", Some("2.0.0"));
    let new_source = tmp.path().join("new-source");
    let _listener = UnixListener::bind(new_source.join("events.sock")).unwrap();

    let err = store
        .install(
            AbsolutePathBuf::try_from(new_source).unwrap(),
            plugin_id.clone(),
        )
        .expect_err("nested socket must fail closed");

    assert!(
        err.to_string()
            .contains("plugin source contains unsupported file type")
    );
    assert_eq!(
        store.active_plugin_version(&plugin_id),
        Some("1.0.0".to_string())
    );
}

#[cfg(windows)]
#[test]
fn install_rejects_source_root_junction_without_replacing_existing_plugin() {
    use std::process::Command;

    let tmp = tempdir().unwrap();
    let store = PluginStore::new(tmp.path().to_path_buf());
    let plugin_id = PluginId::new("sample-plugin".to_string(), "debug".to_string()).unwrap();
    write_plugin_with_version(tmp.path(), "old-source", "sample-plugin", Some("1.0.0"));
    store
        .install(
            AbsolutePathBuf::try_from(tmp.path().join("old-source")).unwrap(),
            plugin_id.clone(),
        )
        .unwrap();
    write_plugin_with_version(
        tmp.path(),
        "junction-target",
        "sample-plugin",
        Some("2.0.0"),
    );
    let junction_path = tmp.path().join("source-junction");
    let output = Command::new("cmd")
        .arg("/C")
        .arg("mklink")
        .arg("/J")
        .arg(&junction_path)
        .arg(tmp.path().join("junction-target"))
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "mklink /J failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let err = store
        .install(
            AbsolutePathBuf::try_from(junction_path).unwrap(),
            plugin_id.clone(),
        )
        .expect_err("source-root junction must fail closed");

    assert!(
        err.to_string()
            .contains("plugin source contains unsupported Windows reparse point")
    );
    assert_eq!(
        store.active_plugin_version(&plugin_id),
        Some("1.0.0".to_string())
    );
}

#[cfg(windows)]
#[test]
fn install_rejects_nested_junction_without_replacing_existing_plugin() {
    use std::process::Command;

    let tmp = tempdir().unwrap();
    let store = PluginStore::new(tmp.path().to_path_buf());
    let plugin_id = PluginId::new("sample-plugin".to_string(), "debug".to_string()).unwrap();
    write_plugin_with_version(tmp.path(), "old-source", "sample-plugin", Some("1.0.0"));
    store
        .install(
            AbsolutePathBuf::try_from(tmp.path().join("old-source")).unwrap(),
            plugin_id.clone(),
        )
        .unwrap();
    write_plugin_with_version(tmp.path(), "new-source", "sample-plugin", Some("2.0.0"));
    let new_source = tmp.path().join("new-source");
    let junction_target = tmp.path().join("junction-target");
    let junction_path = new_source.join("junction");
    fs::create_dir_all(&junction_target).unwrap();
    let output = Command::new("cmd")
        .arg("/C")
        .arg("mklink")
        .arg("/J")
        .arg(&junction_path)
        .arg(&junction_target)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "mklink /J failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let err = store
        .install(
            AbsolutePathBuf::try_from(new_source).unwrap(),
            plugin_id.clone(),
        )
        .expect_err("nested junction must fail closed");

    assert!(
        err.to_string()
            .contains("plugin source contains unsupported Windows reparse point")
    );
    assert_eq!(
        store.active_plugin_version(&plugin_id),
        Some("1.0.0".to_string())
    );
}
