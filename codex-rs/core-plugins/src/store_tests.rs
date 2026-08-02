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
fn active_plugin_version_returns_last_sorted_version_when_default_is_missing() {
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
