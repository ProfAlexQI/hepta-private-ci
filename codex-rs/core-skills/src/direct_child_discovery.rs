use super::MAX_SKILLS_DIRS_PER_ROOT;
use super::SKILLS_FILENAME;
use super::parse_skill_file;
use crate::model::SkillError;
use crate::model::SkillLoadOutcome;
use codex_exec_server::ExecutorFileSystem;
use codex_protocol::protocol::SkillScope;
use codex_utils_absolute_path::AbsolutePathBuf;
use std::collections::HashSet;
use std::io;
use tracing::error;

/// Canonical local boundary for Agent Plugin direct-child discovery.
///
/// Construction uses host canonicalization and is therefore only valid after the caller has
/// checked [`super::SkillRootFileSystem::Local`]. Keeping the canonical roots in this type makes
/// the local-only authority and containment invariant explicit for every discovered child.
pub(super) struct LocalDirectChildRoot {
    skills_root: AbsolutePathBuf,
    plugin_root: AbsolutePathBuf,
}

impl LocalDirectChildRoot {
    pub(super) fn resolve(
        skills_root: &AbsolutePathBuf,
        plugin_root: Option<&AbsolutePathBuf>,
    ) -> Result<Self, String> {
        let plugin_root = plugin_root.ok_or_else(|| {
            format!(
                "Agent Plugin skills root {} is missing its plugin root",
                skills_root.display()
            )
        })?;
        let plugin_root = plugin_root.canonicalize().map_err(|err| {
            format!(
                "failed to resolve Agent Plugin root {}: {err}",
                plugin_root.display()
            )
        })?;
        let skills_root = skills_root.canonicalize().map_err(|err| {
            format!(
                "failed to resolve Agent Plugin skills root {}: {err}",
                skills_root.display()
            )
        })?;
        if !skills_root.as_path().starts_with(plugin_root.as_path()) {
            return Err(format!(
                "Agent Plugin skills root {} resolves outside plugin root {}",
                skills_root.display(),
                plugin_root.display()
            ));
        }
        Ok(Self {
            skills_root,
            plugin_root,
        })
    }

    pub(super) fn skills_root(&self) -> &AbsolutePathBuf {
        &self.skills_root
    }

    fn contains(&self, path: &AbsolutePathBuf) -> bool {
        path.as_path().starts_with(self.plugin_root.as_path())
    }

    pub(super) fn canonical_regular_file(
        &self,
        path: &AbsolutePathBuf,
        containing_root: &AbsolutePathBuf,
        label: &str,
    ) -> Option<AbsolutePathBuf> {
        let metadata = match std::fs::symlink_metadata(path.as_path()) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == io::ErrorKind::NotFound => return None,
            Err(error) => {
                tracing::warn!(
                    "ignoring {path}: failed to stat Agent Plugin {label}: {error}",
                    path = path.display()
                );
                return None;
            }
        };
        if metadata.file_type().is_symlink() || !metadata.file_type().is_file() {
            tracing::warn!(
                "ignoring {path}: Agent Plugin {label} must be a regular non-symlink file",
                path = path.display()
            );
            return None;
        }
        let canonical = match path.canonicalize() {
            Ok(canonical) => canonical,
            Err(error) => {
                tracing::warn!(
                    "ignoring {path}: failed to resolve Agent Plugin {label}: {error}",
                    path = path.display()
                );
                return None;
            }
        };
        if !self.contains(&canonical) || !canonical.as_path().starts_with(containing_root.as_path())
        {
            tracing::warn!(
                "ignoring {path}: Agent Plugin {label} resolves outside its skill boundary",
                path = canonical.display()
            );
            return None;
        }
        Some(canonical)
    }
}

pub(super) async fn discover_skills(
    fs: &dyn ExecutorFileSystem,
    root: &LocalDirectChildRoot,
    scope: SkillScope,
    plugin_id: Option<&str>,
    plugin_namespace: Option<&str>,
    outcome: &mut SkillLoadOutcome,
) {
    match fs.get_metadata(root.skills_root(), /*sandbox*/ None).await {
        Ok(metadata) if metadata.is_directory => {}
        Ok(_) => return,
        Err(err) if err.kind() == io::ErrorKind::NotFound => return,
        Err(err) => {
            error!(
                "failed to stat Agent Plugin skills root {}: {err:#}",
                root.skills_root().display()
            );
            return;
        }
    }

    let entries = match fs
        .read_directory(root.skills_root(), /*sandbox*/ None)
        .await
    {
        Ok(entries) => entries,
        Err(err) => {
            error!(
                "failed to read Agent Plugin skills root {}: {err:#}",
                root.skills_root().display()
            );
            return;
        }
    };

    let mut visited_dirs = HashSet::from([root.skills_root.clone()]);
    let mut truncated = false;
    for entry in entries {
        if entry.file_name.starts_with('.') {
            continue;
        }
        let candidate = root.skills_root.join(&entry.file_name);
        let metadata = match fs.get_metadata(&candidate, /*sandbox*/ None).await {
            Ok(metadata) => metadata,
            Err(err) => {
                error!(
                    "failed to stat Agent Plugin skill directory {}: {err:#}",
                    candidate.display()
                );
                continue;
            }
        };

        if !metadata.is_directory && !metadata.is_symlink {
            continue;
        }
        if metadata.is_symlink {
            match fs.read_directory(&candidate, /*sandbox*/ None).await {
                Ok(_) => {}
                Err(err)
                    if matches!(
                        err.kind(),
                        io::ErrorKind::NotADirectory | io::ErrorKind::NotFound
                    ) =>
                {
                    continue;
                }
                Err(err) => {
                    error!(
                        "failed to read Agent Plugin skill directory symlink {}: {err:#}",
                        candidate.display()
                    );
                    continue;
                }
            }
        }

        let skill_dir = match candidate.canonicalize() {
            Ok(path) => path,
            Err(err) => {
                error!(
                    "failed to resolve Agent Plugin skill directory {}: {err}",
                    candidate.display()
                );
                continue;
            }
        };
        if !root.contains(&skill_dir) {
            error!(
                "Agent Plugin skill directory {} resolves outside plugin root {}",
                skill_dir.display(),
                root.plugin_root.display()
            );
            continue;
        }
        if visited_dirs.len() >= MAX_SKILLS_DIRS_PER_ROOT {
            truncated = true;
            break;
        }
        if !visited_dirs.insert(skill_dir.clone()) {
            continue;
        }

        let skill_path = skill_dir.join(SKILLS_FILENAME);
        let metadata = match fs.get_metadata(&skill_path, /*sandbox*/ None).await {
            Ok(metadata) => metadata,
            Err(err) if err.kind() == io::ErrorKind::NotFound => continue,
            Err(err) => {
                error!(
                    "failed to stat Agent Plugin skill path {}: {err:#}",
                    skill_path.display()
                );
                continue;
            }
        };
        if metadata.is_symlink || !metadata.is_file {
            continue;
        }
        let skill_path = match skill_path.canonicalize() {
            Ok(path) => path,
            Err(err) => {
                error!(
                    "failed to resolve Agent Plugin skill path {}: {err}",
                    skill_path.display()
                );
                continue;
            }
        };
        if !root.contains(&skill_path) {
            error!(
                "Agent Plugin skill path {} resolves outside plugin root {}",
                skill_path.display(),
                root.plugin_root.display()
            );
            continue;
        }

        match parse_skill_file(
            fs,
            &skill_path,
            scope,
            plugin_id,
            plugin_namespace,
            Some(root),
        )
        .await
        {
            Ok(skill) => outcome.skills.push(skill),
            Err(err) if scope != SkillScope::System => outcome.errors.push(SkillError {
                path: skill_path,
                message: err.to_string(),
            }),
            Err(_) => {}
        }
    }

    if truncated {
        tracing::warn!(
            "Agent Plugin skills scan truncated after {} directories (root: {})",
            MAX_SKILLS_DIRS_PER_ROOT,
            root.skills_root().display()
        );
    }
}
