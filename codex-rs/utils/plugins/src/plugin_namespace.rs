//! Resolve plugin namespace from skill file paths by walking ancestors for `plugin.json`.

use codex_exec_server::ExecutorFileSystem;
use codex_utils_absolute_path::AbsolutePathBuf;
use std::fs;
use std::io;
use std::path::Component;
use std::path::Path;
use std::path::PathBuf;

const DISCOVERABLE_PLUGIN_MANIFEST_PATHS: &[&str] =
    &[".codex-plugin/plugin.json", ".claude-plugin/plugin.json"];

pub const AGENT_PLUGIN_MANIFEST_RELATIVE_PATH: &str = "plugin.json";
pub const AGENT_PLUGIN_SCHEMA_URI: &str =
    "https://agent-plugins.org/schemas/1.0.0/plugin.schema.json";
pub const SUPPORTED_AGENT_PLUGIN_SCHEMA_URIS: &[&str] = &[AGENT_PLUGIN_SCHEMA_URI];
pub const AGENT_PLUGIN_SCHEMA_PREFIX: &str = "https://agent-plugins.org/schemas/";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AgentPluginSchemaStatus {
    Supported,
    Unsupported,
    Unrelated,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PluginManifestPathResolution {
    Found(PathBuf),
    Rejected,
    NotFound,
}

pub fn agent_plugin_schema_status(contents: &str) -> AgentPluginSchemaStatus {
    let Ok(value) = serde_json::from_str::<serde_json::Value>(contents) else {
        return AgentPluginSchemaStatus::Unrelated;
    };
    let Some(schema) = value.get("$schema").and_then(serde_json::Value::as_str) else {
        return AgentPluginSchemaStatus::Unrelated;
    };
    if SUPPORTED_AGENT_PLUGIN_SCHEMA_URIS.contains(&schema) {
        AgentPluginSchemaStatus::Supported
    } else if schema.starts_with(AGENT_PLUGIN_SCHEMA_PREFIX) {
        AgentPluginSchemaStatus::Unsupported
    } else {
        AgentPluginSchemaStatus::Unrelated
    }
}

pub fn find_plugin_manifest_path(plugin_root: &Path) -> Option<PathBuf> {
    match resolve_plugin_manifest_path(plugin_root) {
        PluginManifestPathResolution::Found(path) => Some(path),
        PluginManifestPathResolution::Rejected | PluginManifestPathResolution::NotFound => None,
    }
}

pub fn resolve_plugin_manifest_path(plugin_root: &Path) -> PluginManifestPathResolution {
    let agent_manifest_relative_path = Path::new(AGENT_PLUGIN_MANIFEST_RELATIVE_PATH);
    match resolve_regular_plugin_manifest_candidate(plugin_root, agent_manifest_relative_path) {
        PluginManifestPathResolution::Found(agent_manifest_path) => {
            match fs::read_to_string(&agent_manifest_path) {
                Ok(contents)
                    if agent_plugin_schema_status(&contents)
                        != AgentPluginSchemaStatus::Unrelated =>
                {
                    return PluginManifestPathResolution::Found(agent_manifest_path);
                }
                Ok(_) => {}
                Err(_) => return PluginManifestPathResolution::Rejected,
            }
        }
        PluginManifestPathResolution::Rejected => {
            return PluginManifestPathResolution::Rejected;
        }
        PluginManifestPathResolution::NotFound => {}
    }

    for relative_path in DISCOVERABLE_PLUGIN_MANIFEST_PATHS {
        match resolve_regular_plugin_manifest_candidate(plugin_root, Path::new(relative_path)) {
            PluginManifestPathResolution::Found(manifest_path) => {
                return PluginManifestPathResolution::Found(manifest_path);
            }
            PluginManifestPathResolution::Rejected => {
                return PluginManifestPathResolution::Rejected;
            }
            PluginManifestPathResolution::NotFound => {}
        }
    }
    PluginManifestPathResolution::NotFound
}

/// Resolves a manifest candidate only when every component beneath `plugin_root` is a regular,
/// non-link path. Inspecting every component prevents a regular leaf from hiding behind a linked
/// `.codex-plugin`/`.claude-plugin` directory (including Windows directory reparse points).
pub fn resolve_regular_plugin_manifest_candidate(
    plugin_root: &Path,
    relative_path: &Path,
) -> PluginManifestPathResolution {
    let root_metadata = match fs::symlink_metadata(plugin_root) {
        Ok(metadata) => metadata,
        Err(error)
            if matches!(
                error.kind(),
                io::ErrorKind::NotFound | io::ErrorKind::NotADirectory
            ) =>
        {
            return PluginManifestPathResolution::NotFound;
        }
        Err(_) => return PluginManifestPathResolution::Rejected,
    };
    if root_metadata.file_type().is_symlink()
        || !root_metadata.file_type().is_dir()
        || is_windows_reparse_point(&root_metadata)
    {
        return PluginManifestPathResolution::Rejected;
    }

    let components = relative_path.components().collect::<Vec<_>>();
    if components.is_empty()
        || components
            .iter()
            .any(|component| !matches!(component, Component::Normal(_)))
    {
        return PluginManifestPathResolution::Rejected;
    }

    let mut candidate = plugin_root.to_path_buf();
    for (index, component) in components.iter().enumerate() {
        let Component::Normal(component) = component else {
            return PluginManifestPathResolution::Rejected;
        };
        candidate.push(component);
        let metadata = match fs::symlink_metadata(&candidate) {
            Ok(metadata) => metadata,
            Err(error)
                if matches!(
                    error.kind(),
                    io::ErrorKind::NotFound | io::ErrorKind::NotADirectory
                ) =>
            {
                return PluginManifestPathResolution::NotFound;
            }
            Err(_) => return PluginManifestPathResolution::Rejected,
        };
        if metadata.file_type().is_symlink() || is_windows_reparse_point(&metadata) {
            return PluginManifestPathResolution::Rejected;
        }
        let is_leaf = index + 1 == components.len();
        if (is_leaf && !metadata.file_type().is_file())
            || (!is_leaf && !metadata.file_type().is_dir())
        {
            return PluginManifestPathResolution::Rejected;
        }
    }
    PluginManifestPathResolution::Found(candidate)
}

#[cfg(windows)]
fn is_windows_reparse_point(metadata: &fs::Metadata) -> bool {
    use std::os::windows::fs::MetadataExt;

    const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x400;
    metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0
}

#[cfg(not(windows))]
fn is_windows_reparse_point(_metadata: &fs::Metadata) -> bool {
    false
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawPluginManifestName {
    #[serde(default)]
    name: String,
}

enum PluginManifestNameResolution {
    Found(String),
    Rejected,
    NotFound,
}

enum ExecutorManifestPathResolution {
    Found(AbsolutePathBuf),
    Rejected,
    NotFound,
}

async fn resolve_executor_manifest_candidate(
    fs: &dyn ExecutorFileSystem,
    plugin_root: &AbsolutePathBuf,
    relative_path: &Path,
) -> ExecutorManifestPathResolution {
    let root_metadata = match fs.get_metadata(plugin_root, /*sandbox*/ None).await {
        Ok(metadata) => metadata,
        Err(error)
            if matches!(
                error.kind(),
                io::ErrorKind::NotFound | io::ErrorKind::NotADirectory
            ) =>
        {
            return ExecutorManifestPathResolution::NotFound;
        }
        Err(_) => return ExecutorManifestPathResolution::Rejected,
    };
    if root_metadata.is_symlink || !root_metadata.is_directory {
        return ExecutorManifestPathResolution::Rejected;
    }

    let components = relative_path.components().collect::<Vec<_>>();
    if components.is_empty()
        || components
            .iter()
            .any(|component| !matches!(component, Component::Normal(_)))
    {
        return ExecutorManifestPathResolution::Rejected;
    }

    let mut candidate = plugin_root.clone();
    for (index, component) in components.iter().enumerate() {
        let Component::Normal(component) = component else {
            return ExecutorManifestPathResolution::Rejected;
        };
        candidate = candidate.join(component);
        let metadata = match fs.get_metadata(&candidate, /*sandbox*/ None).await {
            Ok(metadata) => metadata,
            Err(error)
                if matches!(
                    error.kind(),
                    io::ErrorKind::NotFound | io::ErrorKind::NotADirectory
                ) =>
            {
                return ExecutorManifestPathResolution::NotFound;
            }
            Err(_) => return ExecutorManifestPathResolution::Rejected,
        };
        if metadata.is_symlink {
            return ExecutorManifestPathResolution::Rejected;
        }
        let is_leaf = index + 1 == components.len();
        if (is_leaf && !metadata.is_file) || (!is_leaf && !metadata.is_directory) {
            return ExecutorManifestPathResolution::Rejected;
        }
    }
    ExecutorManifestPathResolution::Found(candidate)
}

async fn plugin_manifest_name(
    fs: &dyn ExecutorFileSystem,
    plugin_root: &AbsolutePathBuf,
) -> PluginManifestNameResolution {
    match resolve_executor_manifest_candidate(
        fs,
        plugin_root,
        Path::new(AGENT_PLUGIN_MANIFEST_RELATIVE_PATH),
    )
    .await
    {
        ExecutorManifestPathResolution::Found(agent_manifest_path) => {
            match fs
                .read_file_text(&agent_manifest_path, /*sandbox*/ None)
                .await
            {
                Ok(contents) => match agent_plugin_schema_status(&contents) {
                    AgentPluginSchemaStatus::Supported => {
                        return plugin_name_from_manifest_contents(plugin_root, &contents)
                            .map(PluginManifestNameResolution::Found)
                            .unwrap_or(PluginManifestNameResolution::Rejected);
                    }
                    AgentPluginSchemaStatus::Unsupported => {
                        return PluginManifestNameResolution::Rejected;
                    }
                    AgentPluginSchemaStatus::Unrelated => {}
                },
                Err(_) => return PluginManifestNameResolution::Rejected,
            }
        }
        ExecutorManifestPathResolution::Rejected => {
            return PluginManifestNameResolution::Rejected;
        }
        ExecutorManifestPathResolution::NotFound => {}
    }

    let mut manifest_path = None;
    for relative_path in DISCOVERABLE_PLUGIN_MANIFEST_PATHS {
        match resolve_executor_manifest_candidate(fs, plugin_root, Path::new(relative_path)).await {
            ExecutorManifestPathResolution::Rejected => {
                return PluginManifestNameResolution::Rejected;
            }
            ExecutorManifestPathResolution::Found(candidate) => {
                manifest_path = Some(candidate);
                break;
            }
            ExecutorManifestPathResolution::NotFound => {}
        }
    }
    let Some(manifest_path) = manifest_path else {
        return PluginManifestNameResolution::NotFound;
    };
    let Ok(contents) = fs.read_file_text(&manifest_path, /*sandbox*/ None).await else {
        return PluginManifestNameResolution::Rejected;
    };
    plugin_name_from_manifest_contents(plugin_root, &contents)
        .map(PluginManifestNameResolution::Found)
        .unwrap_or(PluginManifestNameResolution::Rejected)
}

fn plugin_name_from_manifest_contents(
    plugin_root: &AbsolutePathBuf,
    contents: &str,
) -> Option<String> {
    let RawPluginManifestName { name: raw_name } = serde_json::from_str(contents).ok()?;
    Some(
        plugin_root
            .file_name()
            .and_then(|entry| entry.to_str())
            .filter(|_| raw_name.trim().is_empty())
            .unwrap_or(raw_name.as_str())
            .to_string(),
    )
}

/// Returns the plugin manifest `name` for the nearest ancestor of `path` that contains a valid
/// plugin manifest (same `name` rules as full manifest loading in codex-core).
pub async fn plugin_namespace_for_skill_path(
    fs: &dyn ExecutorFileSystem,
    path: &AbsolutePathBuf,
) -> Option<String> {
    let parent = path.parent()?;
    for ancestor in parent.ancestors() {
        match plugin_manifest_name(fs, &ancestor).await {
            PluginManifestNameResolution::Found(name) => return Some(name),
            PluginManifestNameResolution::Rejected => return None,
            PluginManifestNameResolution::NotFound => {}
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::AGENT_PLUGIN_MANIFEST_RELATIVE_PATH;
    use super::AGENT_PLUGIN_SCHEMA_URI;
    use super::PluginManifestPathResolution;
    use super::find_plugin_manifest_path;
    use super::plugin_namespace_for_skill_path;
    use super::resolve_plugin_manifest_path;
    use codex_exec_server::LOCAL_FS;
    use codex_utils_absolute_path::test_support::PathBufExt;
    use std::fs;
    use tempfile::tempdir;

    const ALTERNATE_PLUGIN_MANIFEST_RELATIVE_PATH: &str = ".claude-plugin/plugin.json";

    #[tokio::test]
    async fn uses_manifest_name() {
        let tmp = tempdir().expect("tempdir");
        let plugin_root = tmp.path().join("plugins/sample");
        let skill_path = plugin_root.join("skills/search/SKILL.md");

        fs::create_dir_all(skill_path.parent().expect("parent")).expect("mkdir");
        fs::create_dir_all(plugin_root.join(".codex-plugin")).expect("mkdir manifest");
        fs::write(
            plugin_root.join(".codex-plugin/plugin.json"),
            r#"{"name":"sample"}"#,
        )
        .expect("write manifest");
        fs::write(&skill_path, "---\ndescription: search\n---\n").expect("write skill");

        assert_eq!(
            plugin_namespace_for_skill_path(LOCAL_FS.as_ref(), &skill_path.abs()).await,
            Some("sample".to_string())
        );
    }

    #[tokio::test]
    async fn uses_name_from_alternate_discoverable_manifest_path() {
        let tmp = tempdir().expect("tempdir");
        let plugin_root = tmp.path().join("plugins/sample");
        let skill_path = plugin_root.join("skills/search/SKILL.md");
        let manifest_path = plugin_root.join(ALTERNATE_PLUGIN_MANIFEST_RELATIVE_PATH);

        fs::create_dir_all(skill_path.parent().expect("parent")).expect("mkdir");
        fs::create_dir_all(manifest_path.parent().expect("manifest parent"))
            .expect("mkdir manifest");
        fs::write(&manifest_path, r#"{"name":"sample"}"#).expect("write manifest");
        fs::write(&skill_path, "---\ndescription: search\n---\n").expect("write skill");

        assert_eq!(
            plugin_namespace_for_skill_path(LOCAL_FS.as_ref(), &skill_path.abs()).await,
            Some("sample".to_string())
        );
        assert_eq!(find_plugin_manifest_path(&plugin_root), Some(manifest_path));
    }

    #[test]
    fn supported_agent_manifest_precedes_legacy_manifest() {
        let tmp = tempdir().expect("tempdir");
        let plugin_root = tmp.path().join("plugins/sample");
        let root_manifest = plugin_root.join(AGENT_PLUGIN_MANIFEST_RELATIVE_PATH);
        let legacy_manifest = plugin_root.join(".codex-plugin/plugin.json");
        fs::create_dir_all(legacy_manifest.parent().expect("legacy parent")).expect("mkdir");
        fs::write(
            &root_manifest,
            format!(r#"{{"$schema":"{AGENT_PLUGIN_SCHEMA_URI}","name":"sample"}}"#),
        )
        .expect("write Agent Plugin manifest");
        fs::write(legacy_manifest, r#"{"name":"legacy"}"#).expect("write legacy manifest");

        assert_eq!(find_plugin_manifest_path(&plugin_root), Some(root_manifest));
    }

    #[test]
    fn unsupported_agent_manifest_precedes_legacy_manifest_to_fail_closed() {
        let tmp = tempdir().expect("tempdir");
        let plugin_root = tmp.path().join("plugins/sample");
        let root_manifest = plugin_root.join(AGENT_PLUGIN_MANIFEST_RELATIVE_PATH);
        let legacy_manifest = plugin_root.join(".codex-plugin/plugin.json");
        fs::create_dir_all(legacy_manifest.parent().expect("legacy parent")).expect("mkdir");
        fs::write(
            &root_manifest,
            r#"{
  "$schema":"https://agent-plugins.org/schemas/2.0.0/plugin.schema.json",
  "name":"sample"
}"#,
        )
        .expect("write unsupported Agent Plugin manifest");
        fs::write(legacy_manifest, r#"{"name":"legacy"}"#).expect("write legacy manifest");

        assert_eq!(find_plugin_manifest_path(&plugin_root), Some(root_manifest));
    }

    #[test]
    fn unrelated_root_manifest_keeps_codex_then_claude_precedence() {
        let tmp = tempdir().expect("tempdir");
        let plugin_root = tmp.path().join("plugins/sample");
        let codex_manifest = plugin_root.join(".codex-plugin/plugin.json");
        let claude_manifest = plugin_root.join(".claude-plugin/plugin.json");
        fs::create_dir_all(codex_manifest.parent().expect("Codex parent")).expect("mkdir");
        fs::create_dir_all(claude_manifest.parent().expect("Claude parent")).expect("mkdir");
        fs::write(
            plugin_root.join(AGENT_PLUGIN_MANIFEST_RELATIVE_PATH),
            r#"{"name":"npm-package"}"#,
        )
        .expect("write unrelated root manifest");
        fs::write(&codex_manifest, r#"{"name":"codex"}"#).expect("write Codex manifest");
        fs::write(claude_manifest, r#"{"name":"claude"}"#).expect("write Claude manifest");

        assert_eq!(
            find_plugin_manifest_path(&plugin_root),
            Some(codex_manifest)
        );
    }

    #[tokio::test]
    async fn nonregular_legacy_manifest_fails_closed_before_other_fallbacks() {
        let tmp = tempdir().expect("tempdir");
        let outer_root = tmp.path().join("plugins/outer");
        let plugin_root = outer_root.join("sample");
        let skill_path = plugin_root.join("skills/search/SKILL.md");
        let codex_manifest = plugin_root.join(".codex-plugin/plugin.json");
        let claude_manifest = plugin_root.join(".claude-plugin/plugin.json");
        let outer_manifest = outer_root.join(".codex-plugin/plugin.json");
        fs::create_dir_all(&codex_manifest).expect("nonregular Codex manifest");
        fs::create_dir_all(claude_manifest.parent().expect("Claude parent")).expect("mkdir");
        fs::create_dir_all(outer_manifest.parent().expect("outer parent")).expect("mkdir");
        fs::create_dir_all(skill_path.parent().expect("skill parent")).expect("mkdir");
        fs::write(claude_manifest, r#"{"name":"claude"}"#).expect("write Claude manifest");
        fs::write(outer_manifest, r#"{"name":"outer"}"#).expect("write outer manifest");
        fs::write(&skill_path, "---\ndescription: search\n---\n").expect("write skill");

        assert_eq!(find_plugin_manifest_path(&plugin_root), None);
        assert_eq!(
            resolve_plugin_manifest_path(&plugin_root),
            PluginManifestPathResolution::Rejected
        );
        assert_eq!(
            plugin_namespace_for_skill_path(LOCAL_FS.as_ref(), &skill_path.abs()).await,
            None
        );
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn symlinked_legacy_manifest_fails_closed_before_other_fallbacks() {
        let tmp = tempdir().expect("tempdir");
        let outer_root = tmp.path().join("plugins/outer");
        let plugin_root = outer_root.join("sample");
        let skill_path = plugin_root.join("skills/search/SKILL.md");
        let codex_manifest = plugin_root.join(".codex-plugin/plugin.json");
        let claude_manifest = plugin_root.join(".claude-plugin/plugin.json");
        let outer_manifest = outer_root.join(".codex-plugin/plugin.json");
        let manifest_target = tmp.path().join("legacy-plugin.json");
        fs::create_dir_all(codex_manifest.parent().expect("Codex parent")).expect("mkdir");
        fs::create_dir_all(claude_manifest.parent().expect("Claude parent")).expect("mkdir");
        fs::create_dir_all(outer_manifest.parent().expect("outer parent")).expect("mkdir");
        fs::create_dir_all(skill_path.parent().expect("skill parent")).expect("mkdir");
        fs::write(&manifest_target, r#"{"name":"linked"}"#).expect("write target");
        std::os::unix::fs::symlink(&manifest_target, &codex_manifest).expect("symlink manifest");
        fs::write(claude_manifest, r#"{"name":"claude"}"#).expect("write Claude manifest");
        fs::write(outer_manifest, r#"{"name":"outer"}"#).expect("write outer manifest");
        fs::write(&skill_path, "---\ndescription: search\n---\n").expect("write skill");

        assert_eq!(find_plugin_manifest_path(&plugin_root), None);
        assert_eq!(
            resolve_plugin_manifest_path(&plugin_root),
            PluginManifestPathResolution::Rejected
        );
        assert_eq!(
            plugin_namespace_for_skill_path(LOCAL_FS.as_ref(), &skill_path.abs()).await,
            None
        );
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn symlinked_legacy_manifest_directory_fails_closed() {
        let tmp = tempdir().expect("tempdir");
        let plugin_root = tmp.path().join("plugins/sample");
        let skill_path = plugin_root.join("skills/search/SKILL.md");
        let linked_manifest_dir = tmp.path().join("linked-codex-plugin");
        fs::create_dir_all(&linked_manifest_dir).expect("linked manifest directory");
        fs::create_dir_all(skill_path.parent().expect("skill parent")).expect("skill directory");
        fs::write(
            linked_manifest_dir.join("plugin.json"),
            r#"{"name":"linked"}"#,
        )
        .expect("write linked manifest");
        std::os::unix::fs::symlink(&linked_manifest_dir, plugin_root.join(".codex-plugin"))
            .expect("symlink manifest directory");
        fs::write(&skill_path, "---\ndescription: search\n---\n").expect("write skill");

        assert_eq!(find_plugin_manifest_path(&plugin_root), None);
        assert_eq!(
            resolve_plugin_manifest_path(&plugin_root),
            PluginManifestPathResolution::Rejected
        );
        assert_eq!(
            plugin_namespace_for_skill_path(LOCAL_FS.as_ref(), &skill_path.abs()).await,
            None
        );
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn symlinked_plugin_root_fails_closed() {
        let tmp = tempdir().expect("tempdir");
        let real_plugin_root = tmp.path().join("real-plugin");
        let linked_plugin_root = tmp.path().join("linked-plugin");
        let real_skill_path = real_plugin_root.join("skills/search/SKILL.md");
        fs::create_dir_all(real_skill_path.parent().expect("skill parent"))
            .expect("skill directory");
        fs::create_dir_all(real_plugin_root.join(".codex-plugin")).expect("manifest directory");
        fs::write(
            real_plugin_root.join(".codex-plugin/plugin.json"),
            r#"{"name":"linked"}"#,
        )
        .expect("write manifest");
        fs::write(&real_skill_path, "---\ndescription: search\n---\n").expect("write skill");
        std::os::unix::fs::symlink(&real_plugin_root, &linked_plugin_root)
            .expect("symlink plugin root");
        let linked_skill_path = linked_plugin_root.join("skills/search/SKILL.md");

        assert_eq!(find_plugin_manifest_path(&linked_plugin_root), None);
        assert_eq!(
            resolve_plugin_manifest_path(&linked_plugin_root),
            PluginManifestPathResolution::Rejected
        );
        assert_eq!(
            plugin_namespace_for_skill_path(LOCAL_FS.as_ref(), &linked_skill_path.abs()).await,
            None
        );
    }

    #[cfg(windows)]
    #[test]
    fn junctioned_legacy_manifest_directory_fails_closed() {
        use std::process::Command;

        let tmp = tempdir().expect("tempdir");
        let plugin_root = tmp.path().join("plugins/sample");
        let junction_target = tmp.path().join("junction-codex-plugin");
        fs::create_dir_all(&plugin_root).expect("plugin root");
        fs::create_dir_all(&junction_target).expect("junction target");
        fs::write(
            junction_target.join("plugin.json"),
            r#"{"name":"junction"}"#,
        )
        .expect("write junction manifest");
        let output = Command::new("cmd")
            .arg("/C")
            .arg("mklink")
            .arg("/J")
            .arg(plugin_root.join(".codex-plugin"))
            .arg(&junction_target)
            .output()
            .expect("create manifest junction");
        assert!(
            output.status.success(),
            "mklink /J failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );

        assert_eq!(find_plugin_manifest_path(&plugin_root), None);
        assert_eq!(
            resolve_plugin_manifest_path(&plugin_root),
            PluginManifestPathResolution::Rejected
        );
    }

    #[cfg(windows)]
    #[test]
    fn junctioned_plugin_root_fails_closed() {
        use std::process::Command;

        let tmp = tempdir().expect("tempdir");
        let real_plugin_root = tmp.path().join("real-plugin");
        let junction_plugin_root = tmp.path().join("junction-plugin");
        fs::create_dir_all(real_plugin_root.join(".codex-plugin")).expect("manifest directory");
        fs::write(
            real_plugin_root.join(".codex-plugin/plugin.json"),
            r#"{"name":"junction"}"#,
        )
        .expect("write manifest");
        let output = Command::new("cmd")
            .arg("/C")
            .arg("mklink")
            .arg("/J")
            .arg(&junction_plugin_root)
            .arg(&real_plugin_root)
            .output()
            .expect("create plugin root junction");
        assert!(
            output.status.success(),
            "mklink /J failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );

        assert_eq!(find_plugin_manifest_path(&junction_plugin_root), None);
        assert_eq!(
            resolve_plugin_manifest_path(&junction_plugin_root),
            PluginManifestPathResolution::Rejected
        );
    }

    #[tokio::test]
    async fn nonregular_agent_manifest_fails_closed_before_legacy_fallback() {
        let tmp = tempdir().expect("tempdir");
        let plugin_root = tmp.path().join("plugins/sample");
        let skill_path = plugin_root.join("skills/search/SKILL.md");
        let legacy_manifest = plugin_root.join(".codex-plugin/plugin.json");
        fs::create_dir_all(plugin_root.join(AGENT_PLUGIN_MANIFEST_RELATIVE_PATH))
            .expect("root manifest directory");
        fs::create_dir_all(legacy_manifest.parent().expect("legacy parent")).expect("mkdir");
        fs::create_dir_all(skill_path.parent().expect("skill parent")).expect("skill directory");
        fs::write(legacy_manifest, r#"{"name":"legacy"}"#).expect("write legacy manifest");
        fs::write(&skill_path, "---\ndescription: search\n---\n").expect("write skill");

        assert_eq!(find_plugin_manifest_path(&plugin_root), None);
        assert_eq!(
            resolve_plugin_manifest_path(&plugin_root),
            PluginManifestPathResolution::Rejected
        );
        assert_eq!(
            plugin_namespace_for_skill_path(LOCAL_FS.as_ref(), &skill_path.abs()).await,
            None
        );
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn symlinked_agent_manifest_fails_closed_before_legacy_fallback() {
        let tmp = tempdir().expect("tempdir");
        let plugin_root = tmp.path().join("plugins/sample");
        let manifest_target = tmp.path().join("portable-plugin.json");
        let skill_path = plugin_root.join("skills/search/SKILL.md");
        let legacy_manifest = plugin_root.join(".codex-plugin/plugin.json");
        fs::create_dir_all(skill_path.parent().expect("skill parent")).expect("plugin root");
        fs::write(
            &manifest_target,
            format!(r#"{{"$schema":"{AGENT_PLUGIN_SCHEMA_URI}","name":"sample"}}"#),
        )
        .expect("write manifest target");
        std::os::unix::fs::symlink(
            &manifest_target,
            plugin_root.join(AGENT_PLUGIN_MANIFEST_RELATIVE_PATH),
        )
        .expect("symlink manifest");
        fs::create_dir_all(legacy_manifest.parent().expect("legacy parent")).expect("mkdir");
        fs::write(legacy_manifest, r#"{"name":"legacy"}"#).expect("write legacy manifest");
        fs::write(&skill_path, "---\ndescription: search\n---\n").expect("write skill");

        assert_eq!(find_plugin_manifest_path(&plugin_root), None);
        assert_eq!(
            resolve_plugin_manifest_path(&plugin_root),
            PluginManifestPathResolution::Rejected
        );
        assert_eq!(
            plugin_namespace_for_skill_path(LOCAL_FS.as_ref(), &skill_path.abs()).await,
            None
        );
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn unreadable_agent_manifest_fails_closed_before_legacy_fallback() {
        use std::os::unix::fs::PermissionsExt;

        let tmp = tempdir().expect("tempdir");
        let plugin_root = tmp.path().join("plugins/sample");
        let root_manifest = plugin_root.join(AGENT_PLUGIN_MANIFEST_RELATIVE_PATH);
        let skill_path = plugin_root.join("skills/search/SKILL.md");
        let legacy_manifest = plugin_root.join(".codex-plugin/plugin.json");
        fs::create_dir_all(skill_path.parent().expect("skill parent")).expect("skill directory");
        fs::create_dir_all(legacy_manifest.parent().expect("legacy parent")).expect("mkdir");
        fs::write(
            &root_manifest,
            format!(r#"{{"$schema":"{AGENT_PLUGIN_SCHEMA_URI}","name":"sample"}}"#),
        )
        .expect("write Agent Plugin manifest");
        fs::write(legacy_manifest, r#"{"name":"legacy"}"#).expect("write legacy manifest");
        fs::write(&skill_path, "---\ndescription: search\n---\n").expect("write skill");
        fs::set_permissions(&root_manifest, fs::Permissions::from_mode(0o000))
            .expect("make root manifest unreadable");

        assert_eq!(find_plugin_manifest_path(&plugin_root), None);
        assert_eq!(
            resolve_plugin_manifest_path(&plugin_root),
            PluginManifestPathResolution::Rejected
        );
        assert_eq!(
            plugin_namespace_for_skill_path(LOCAL_FS.as_ref(), &skill_path.abs()).await,
            None
        );

        fs::set_permissions(&root_manifest, fs::Permissions::from_mode(0o600))
            .expect("restore root manifest permissions");
    }

    #[tokio::test]
    async fn uses_agent_plugin_manifest_name() {
        let tmp = tempdir().expect("tempdir");
        let plugin_root = tmp.path().join("plugins/sample");
        let skill_path = plugin_root.join("skills/search/SKILL.md");
        fs::create_dir_all(skill_path.parent().expect("skill parent")).expect("mkdir");
        fs::write(
            plugin_root.join(AGENT_PLUGIN_MANIFEST_RELATIVE_PATH),
            format!(r#"{{"$schema":"{AGENT_PLUGIN_SCHEMA_URI}","name":"agent.sample"}}"#),
        )
        .expect("write Agent Plugin manifest");
        fs::write(&skill_path, "---\ndescription: search\n---\n").expect("write skill");

        assert_eq!(
            plugin_namespace_for_skill_path(LOCAL_FS.as_ref(), &skill_path.abs()).await,
            Some("agent.sample".to_string())
        );
    }

    #[tokio::test]
    async fn unsupported_agent_manifest_blocks_legacy_and_outer_namespaces() {
        let tmp = tempdir().expect("tempdir");
        let outer_root = tmp.path().join("plugins/outer");
        let plugin_root = outer_root.join("nested");
        let skill_path = plugin_root.join("skills/search/SKILL.md");
        let outer_manifest = outer_root.join(".codex-plugin/plugin.json");
        let legacy_manifest = plugin_root.join(".codex-plugin/plugin.json");
        fs::create_dir_all(outer_manifest.parent().expect("outer parent")).expect("mkdir");
        fs::create_dir_all(legacy_manifest.parent().expect("legacy parent")).expect("mkdir");
        fs::create_dir_all(skill_path.parent().expect("skill parent")).expect("mkdir");
        fs::write(outer_manifest, r#"{"name":"outer"}"#).expect("write outer manifest");
        fs::write(legacy_manifest, r#"{"name":"legacy"}"#).expect("write legacy manifest");
        fs::write(
            plugin_root.join(AGENT_PLUGIN_MANIFEST_RELATIVE_PATH),
            r#"{
  "$schema":"https://agent-plugins.org/schemas/2.0.0/plugin.schema.json",
  "name":"unsupported"
}"#,
        )
        .expect("write unsupported Agent Plugin manifest");
        fs::write(&skill_path, "---\ndescription: search\n---\n").expect("write skill");

        assert_eq!(
            plugin_namespace_for_skill_path(LOCAL_FS.as_ref(), &skill_path.abs()).await,
            None
        );
    }

    #[tokio::test]
    async fn malformed_or_unrelated_agent_root_uses_legacy_namespace() {
        for (case, root_contents) in [
            ("malformed", "{not-json"),
            ("unrelated", r#"{"name":"npm-package"}"#),
        ] {
            let tmp = tempdir().expect("tempdir");
            let plugin_root = tmp.path().join("plugins").join(case);
            let skill_path = plugin_root.join("skills/search/SKILL.md");
            let legacy_manifest = plugin_root.join(".codex-plugin/plugin.json");
            fs::create_dir_all(legacy_manifest.parent().expect("legacy parent")).expect("mkdir");
            fs::create_dir_all(skill_path.parent().expect("skill parent")).expect("mkdir");
            fs::write(
                plugin_root.join(AGENT_PLUGIN_MANIFEST_RELATIVE_PATH),
                root_contents,
            )
            .expect("write root manifest");
            fs::write(legacy_manifest, r#"{"name":"legacy"}"#).expect("write legacy manifest");
            fs::write(&skill_path, "---\ndescription: search\n---\n").expect("write skill");

            assert_eq!(
                plugin_namespace_for_skill_path(LOCAL_FS.as_ref(), &skill_path.abs()).await,
                Some("legacy".to_string()),
                "case {case}"
            );
        }
    }
}
