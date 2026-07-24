use crate::model::SkillError;
use crate::model::SkillFileSystemsByPath;
use crate::model::SkillLoadOutcome;
use crate::system::system_cache_root_dir;
use codex_app_server_protocol::ConfigLayerSource;
use codex_config::ConfigLayerStack;
use codex_config::ConfigLayerStackOrdering;
use codex_config::default_project_root_markers;
use codex_config::merge_toml_values;
use codex_config::project_root_markers_from_config;
use codex_exec_server::ExecutorFileSystem;
use codex_exec_server::LOCAL_FS;
use codex_protocol::protocol::SkillScope;
use codex_utils_absolute_path::AbsolutePathBuf;
use codex_utils_plugins::PluginSkillRoot;
use codex_utils_plugins::SkillDiscoveryMode;
use dirs::home_dir;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io;
use std::sync::Arc;
use toml::Value as TomlValue;
use tracing::error;

#[path = "direct_child_discovery.rs"]
mod direct_child_discovery;

#[path = "skill_parser.rs"]
mod skill_parser;

use skill_parser::SKILLS_FILENAME;
use skill_parser::parse_skill_file;

#[cfg(test)]
use crate::model::SkillDependencies;
#[cfg(test)]
use crate::model::SkillInterface;
#[cfg(test)]
use crate::model::SkillMetadata;
#[cfg(test)]
use crate::model::SkillPolicy;
#[cfg(test)]
use crate::model::SkillToolDependency;
#[cfg(test)]
use skill_parser::MAX_DEFAULT_PROMPT_LEN;
#[cfg(test)]
use skill_parser::MAX_DESCRIPTION_LEN;
#[cfg(test)]
use skill_parser::MAX_NAME_LEN;
#[cfg(test)]
use skill_parser::MAX_QUALIFIED_NAME_LEN;
#[cfg(test)]
use skill_parser::MAX_SHORT_DESCRIPTION_LEN;
#[cfg(test)]
use skill_parser::SKILLS_METADATA_DIR;
#[cfg(test)]
use skill_parser::SKILLS_METADATA_FILENAME;

const AGENTS_DIR_NAME: &str = ".agents";
const SKILLS_DIR_NAME: &str = "skills";
// Traversal depth from the skills root.
const MAX_SCAN_DEPTH: usize = 6;
const MAX_SKILLS_DIRS_PER_ROOT: usize = 2000;

pub struct SkillRoot {
    pub path: AbsolutePathBuf,
    pub scope: SkillScope,
    pub file_system: SkillRootFileSystem,
    pub plugin_id: Option<String>,
    pub plugin_namespace: Option<String>,
    pub plugin_root: Option<AbsolutePathBuf>,
    pub discovery_mode: SkillDiscoveryMode,
}

#[derive(Clone)]
pub enum SkillRootFileSystem {
    /// Use the caller-supplied executor filesystem. This is valid for recursive discovery only.
    Executor(Arc<dyn ExecutorFileSystem>),
    /// Use the host-local filesystem. Agent Plugin direct-child discovery requires this explicit
    /// authority because containment checks canonicalize local paths.
    Local,
}

impl SkillRootFileSystem {
    fn into_parts(self) -> (Arc<dyn ExecutorFileSystem>, bool) {
        match self {
            Self::Executor(fs) => (fs, false),
            Self::Local => (Arc::clone(&LOCAL_FS), true),
        }
    }
}

pub async fn load_skills_from_roots<I>(roots: I) -> SkillLoadOutcome
where
    I: IntoIterator<Item = SkillRoot>,
{
    let mut outcome = SkillLoadOutcome::default();
    let mut skill_roots: Vec<AbsolutePathBuf> = Vec::new();
    let mut skill_root_by_path: HashMap<AbsolutePathBuf, AbsolutePathBuf> = HashMap::new();
    let mut file_systems_by_skill_path: HashMap<AbsolutePathBuf, Arc<dyn ExecutorFileSystem>> =
        HashMap::new();
    for root in roots {
        let SkillRoot {
            path,
            scope,
            file_system,
            plugin_id,
            plugin_namespace,
            plugin_root,
            discovery_mode,
        } = root;
        let (fs, has_local_authority) = file_system.into_parts();
        if plugin_id.is_some() && plugin_namespace.is_none() {
            error!(
                "plugin skill root {} is missing its frozen plugin namespace",
                path.display()
            );
            continue;
        }
        if discovery_mode == SkillDiscoveryMode::DirectChildren && !has_local_authority {
            error!(
                "Agent Plugin direct-child discovery requires explicit local filesystem authority for {}",
                path.display()
            );
            continue;
        }
        let direct_child_root = match discovery_mode {
            SkillDiscoveryMode::Recursive => None,
            SkillDiscoveryMode::DirectChildren => {
                match direct_child_discovery::LocalDirectChildRoot::resolve(
                    &path,
                    plugin_root.as_ref(),
                ) {
                    Ok(root) => Some(root),
                    Err(message) => {
                        error!("{message}");
                        continue;
                    }
                }
            }
        };
        let root_path = direct_child_root
            .as_ref()
            .map(direct_child_discovery::LocalDirectChildRoot::skills_root)
            .cloned()
            .unwrap_or_else(|| canonicalize_for_skill_identity(&path));
        let skills_before_root = outcome.skills.len();
        if let Some(direct_child_root) = direct_child_root.as_ref() {
            direct_child_discovery::discover_skills(
                fs.as_ref(),
                direct_child_root,
                scope,
                plugin_id.as_deref(),
                plugin_namespace.as_deref(),
                &mut outcome,
            )
            .await;
        } else {
            discover_skills_under_root(
                fs.as_ref(),
                &root_path,
                scope,
                plugin_id.as_deref(),
                plugin_namespace.as_deref(),
                &mut outcome,
            )
            .await;
        }
        for skill in &outcome.skills[skills_before_root..] {
            if !skill_roots.contains(&root_path) {
                skill_roots.push(root_path.clone());
            }
            skill_root_by_path
                .entry(skill.path_to_skills_md.clone())
                .or_insert_with(|| root_path.clone());
            file_systems_by_skill_path
                .entry(skill.path_to_skills_md.clone())
                .or_insert_with(|| Arc::clone(&fs));
        }
    }

    let mut seen: HashSet<AbsolutePathBuf> = HashSet::new();
    outcome
        .skills
        .retain(|skill| seen.insert(skill.path_to_skills_md.clone()));
    let retained_skill_paths: HashSet<AbsolutePathBuf> = outcome
        .skills
        .iter()
        .map(|skill| skill.path_to_skills_md.clone())
        .collect();
    skill_root_by_path.retain(|path, _| retained_skill_paths.contains(path));
    let used_roots: HashSet<AbsolutePathBuf> = skill_root_by_path.values().cloned().collect();
    skill_roots.retain(|root| used_roots.contains(root));
    file_systems_by_skill_path.retain(|path, _| retained_skill_paths.contains(path));
    outcome.skill_roots = skill_roots;
    outcome.skill_root_by_path = Arc::new(skill_root_by_path);
    outcome.file_systems_by_skill_path = SkillFileSystemsByPath::new(file_systems_by_skill_path);

    fn scope_rank(scope: SkillScope) -> u8 {
        // Higher-priority scopes first (matches root scan order for dedupe).
        match scope {
            SkillScope::Repo => 0,
            SkillScope::User => 1,
            SkillScope::System => 2,
            SkillScope::Admin => 3,
        }
    }

    outcome.skills.sort_by(|a, b| {
        scope_rank(a.scope)
            .cmp(&scope_rank(b.scope))
            .then_with(|| a.name.cmp(&b.name))
            .then_with(|| a.path_to_skills_md.cmp(&b.path_to_skills_md))
    });

    outcome
}

pub(crate) async fn skill_roots(
    fs: Option<Arc<dyn ExecutorFileSystem>>,
    config_layer_stack: &ConfigLayerStack,
    cwd: &AbsolutePathBuf,
    plugin_skill_roots: Vec<PluginSkillRoot>,
) -> Vec<SkillRoot> {
    let home_dir =
        home_dir().and_then(|path| AbsolutePathBuf::from_absolute_path_checked(path).ok());
    skill_roots_with_home_dir(
        fs,
        config_layer_stack,
        cwd,
        home_dir.as_ref(),
        plugin_skill_roots,
    )
    .await
}

async fn skill_roots_with_home_dir(
    fs: Option<Arc<dyn ExecutorFileSystem>>,
    config_layer_stack: &ConfigLayerStack,
    cwd: &AbsolutePathBuf,
    home_dir: Option<&AbsolutePathBuf>,
    plugin_skill_roots: Vec<PluginSkillRoot>,
) -> Vec<SkillRoot> {
    let mut roots = skill_roots_from_layer_stack_inner(config_layer_stack, home_dir, fs.clone());
    roots.extend(plugin_skill_roots.into_iter().map(|root| SkillRoot {
        path: root.path,
        scope: SkillScope::User,
        file_system: SkillRootFileSystem::Local,
        plugin_id: Some(root.plugin_id),
        plugin_namespace: Some(root.plugin_namespace),
        plugin_root: Some(root.plugin_root),
        discovery_mode: root.discovery_mode,
    }));
    roots.extend(repo_agents_skill_roots(fs, config_layer_stack, cwd).await);
    dedupe_skill_roots_by_path(&mut roots);
    roots
}

fn skill_roots_from_layer_stack_inner(
    config_layer_stack: &ConfigLayerStack,
    home_dir: Option<&AbsolutePathBuf>,
    repo_fs: Option<Arc<dyn ExecutorFileSystem>>,
) -> Vec<SkillRoot> {
    let mut roots = Vec::new();

    for layer in config_layer_stack.get_layers(
        ConfigLayerStackOrdering::HighestPrecedenceFirst,
        /*include_disabled*/ true,
    ) {
        let Some(config_folder) = layer.config_folder() else {
            continue;
        };

        match &layer.name {
            ConfigLayerSource::Project { .. } => {
                if let Some(repo_fs) = &repo_fs {
                    roots.push(SkillRoot {
                        path: config_folder.join(SKILLS_DIR_NAME),
                        scope: SkillScope::Repo,
                        file_system: SkillRootFileSystem::Executor(Arc::clone(repo_fs)),
                        plugin_id: None,
                        plugin_namespace: None,
                        plugin_root: None,
                        discovery_mode: SkillDiscoveryMode::Recursive,
                    });
                }
            }
            ConfigLayerSource::User { .. } => {
                // Deprecated legacy user skills location (`$CODEX_HOME/skills`), kept for
                // backward compatibility.
                roots.push(SkillRoot {
                    path: config_folder.join(SKILLS_DIR_NAME),
                    scope: SkillScope::User,
                    file_system: SkillRootFileSystem::Local,
                    plugin_id: None,
                    plugin_namespace: None,
                    plugin_root: None,
                    discovery_mode: SkillDiscoveryMode::Recursive,
                });

                // `$HOME/.agents/skills` (user-installed skills).
                if let Some(home_dir) = home_dir {
                    roots.push(SkillRoot {
                        path: home_dir.join(AGENTS_DIR_NAME).join(SKILLS_DIR_NAME),
                        scope: SkillScope::User,
                        file_system: SkillRootFileSystem::Local,
                        plugin_id: None,
                        plugin_namespace: None,
                        plugin_root: None,
                        discovery_mode: SkillDiscoveryMode::Recursive,
                    });
                }

                // Embedded system skills are cached under `$HEPTA_HOME/skills/.system` and are a
                // special case (not a config layer).
                roots.push(SkillRoot {
                    path: system_cache_root_dir(&config_folder),
                    scope: SkillScope::System,
                    file_system: SkillRootFileSystem::Local,
                    plugin_id: None,
                    plugin_namespace: None,
                    plugin_root: None,
                    discovery_mode: SkillDiscoveryMode::Recursive,
                });
            }
            ConfigLayerSource::System { .. } => {
                // The system config layer lives under `/etc/hepta/` on Unix, so treat
                // `/etc/hepta/skills` as admin-scoped skills.
                roots.push(SkillRoot {
                    path: config_folder.join(SKILLS_DIR_NAME),
                    scope: SkillScope::Admin,
                    file_system: SkillRootFileSystem::Local,
                    plugin_id: None,
                    plugin_namespace: None,
                    plugin_root: None,
                    discovery_mode: SkillDiscoveryMode::Recursive,
                });
            }
            ConfigLayerSource::Mdm { .. }
            | ConfigLayerSource::SessionFlags
            | ConfigLayerSource::LegacyManagedConfigTomlFromFile { .. }
            | ConfigLayerSource::LegacyManagedConfigTomlFromMdm => {}
        }
    }

    roots
}

async fn repo_agents_skill_roots(
    fs: Option<Arc<dyn ExecutorFileSystem>>,
    config_layer_stack: &ConfigLayerStack,
    cwd: &AbsolutePathBuf,
) -> Vec<SkillRoot> {
    let Some(fs) = fs else {
        return Vec::new();
    };
    let project_root_markers = project_root_markers_from_stack(config_layer_stack);
    let project_root = find_project_root(fs.as_ref(), cwd, &project_root_markers).await;
    let dirs = dirs_between_project_root_and_cwd(cwd, &project_root);
    let mut roots = Vec::new();
    for dir in dirs {
        let agents_skills = dir.join(AGENTS_DIR_NAME).join(SKILLS_DIR_NAME);
        match fs.get_metadata(&agents_skills, /*sandbox*/ None).await {
            Ok(metadata) if metadata.is_directory => roots.push(SkillRoot {
                path: agents_skills,
                scope: SkillScope::Repo,
                file_system: SkillRootFileSystem::Executor(Arc::clone(&fs)),
                plugin_id: None,
                plugin_namespace: None,
                plugin_root: None,
                discovery_mode: SkillDiscoveryMode::Recursive,
            }),
            Ok(_) => {}
            Err(err) if err.kind() == io::ErrorKind::NotFound => {}
            Err(err) => {
                tracing::warn!(
                    "failed to stat repo skills root {}: {err:#}",
                    agents_skills.display()
                );
            }
        }
    }
    roots
}

fn project_root_markers_from_stack(config_layer_stack: &ConfigLayerStack) -> Vec<String> {
    let mut merged = TomlValue::Table(toml::map::Map::new());
    for layer in config_layer_stack.get_layers(
        ConfigLayerStackOrdering::LowestPrecedenceFirst,
        /*include_disabled*/ false,
    ) {
        if matches!(layer.name, ConfigLayerSource::Project { .. }) {
            continue;
        }
        merge_toml_values(&mut merged, &layer.config);
    }

    match project_root_markers_from_config(&merged) {
        Ok(Some(markers)) => markers,
        Ok(None) => default_project_root_markers(),
        Err(err) => {
            tracing::warn!("invalid project_root_markers: {err}");
            default_project_root_markers()
        }
    }
}

async fn find_project_root(
    fs: &dyn ExecutorFileSystem,
    cwd: &AbsolutePathBuf,
    project_root_markers: &[String],
) -> AbsolutePathBuf {
    if project_root_markers.is_empty() {
        return cwd.clone();
    }

    for ancestor in cwd.ancestors() {
        for marker in project_root_markers {
            let marker_path = ancestor.join(marker);
            match fs.get_metadata(&marker_path, /*sandbox*/ None).await {
                Ok(_) => return ancestor,
                Err(err) if err.kind() == io::ErrorKind::NotFound => {}
                Err(err) => {
                    tracing::warn!(
                        "failed to stat project root marker {}: {err:#}",
                        marker_path.display()
                    );
                }
            }
        }
    }

    cwd.clone()
}

fn dirs_between_project_root_and_cwd(
    cwd: &AbsolutePathBuf,
    project_root: &AbsolutePathBuf,
) -> Vec<AbsolutePathBuf> {
    let mut dirs = cwd
        .ancestors()
        .scan(false, |done, dir| {
            if *done {
                None
            } else {
                if &dir == project_root {
                    *done = true;
                }
                Some(dir)
            }
        })
        .collect::<Vec<_>>();
    dirs.reverse();
    dirs
}

fn dedupe_skill_roots_by_path(roots: &mut Vec<SkillRoot>) {
    let mut seen: HashSet<AbsolutePathBuf> = HashSet::new();
    roots.retain(|root| seen.insert(root.path.clone()));
}

fn canonicalize_for_skill_identity(path: &AbsolutePathBuf) -> AbsolutePathBuf {
    path.canonicalize().unwrap_or_else(|_| path.clone())
}

async fn discover_skills_under_root(
    fs: &dyn ExecutorFileSystem,
    root: &AbsolutePathBuf,
    scope: SkillScope,
    plugin_id: Option<&str>,
    plugin_namespace: Option<&str>,
    outcome: &mut SkillLoadOutcome,
) {
    let root = canonicalize_for_skill_identity(root);
    match fs.get_metadata(&root, /*sandbox*/ None).await {
        Ok(metadata) if metadata.is_directory => {}
        Ok(_) => return,
        Err(err) if err.kind() == io::ErrorKind::NotFound => return,
        Err(err) => {
            error!("failed to stat skills root {}: {err:#}", root.display());
            return;
        }
    }

    fn enqueue_dir(
        queue: &mut VecDeque<(AbsolutePathBuf, usize)>,
        visited_dirs: &mut HashSet<AbsolutePathBuf>,
        truncated_by_dir_limit: &mut bool,
        path: AbsolutePathBuf,
        depth: usize,
    ) {
        if depth > MAX_SCAN_DEPTH {
            return;
        }
        if visited_dirs.len() >= MAX_SKILLS_DIRS_PER_ROOT {
            *truncated_by_dir_limit = true;
            return;
        }
        if visited_dirs.insert(path.clone()) {
            queue.push_back((path, depth));
        }
    }

    // Follow symlinked directories for user, admin, and repo skills. System skills are written by Hepta itself.
    let follow_symlinks = matches!(
        scope,
        SkillScope::Repo | SkillScope::User | SkillScope::Admin
    );

    let mut visited_dirs: HashSet<AbsolutePathBuf> = HashSet::new();
    visited_dirs.insert(root.clone());

    let mut queue = VecDeque::from([(root.clone(), 0)]);
    let mut truncated_by_dir_limit = false;

    while let Some((dir, depth)) = queue.pop_front() {
        let entries = match fs.read_directory(&dir, /*sandbox*/ None).await {
            Ok(entries) => entries,
            Err(e) => {
                error!("failed to read skills dir {}: {e:#}", dir.display());
                continue;
            }
        };

        for entry in entries {
            let file_name = entry.file_name;
            if file_name.starts_with('.') {
                continue;
            }

            let path = dir.join(&file_name);
            let metadata = match fs.get_metadata(&path, /*sandbox*/ None).await {
                Ok(metadata) => metadata,
                Err(e) => {
                    error!("failed to stat skills path {}: {e:#}", path.display());
                    continue;
                }
            };

            if metadata.is_symlink {
                if !follow_symlinks {
                    continue;
                }
                match fs.read_directory(&path, /*sandbox*/ None).await {
                    Ok(_) => {
                        let resolved_dir = canonicalize_for_skill_identity(&path);
                        enqueue_dir(
                            &mut queue,
                            &mut visited_dirs,
                            &mut truncated_by_dir_limit,
                            resolved_dir,
                            depth + 1,
                        );
                    }
                    Err(err)
                        if matches!(
                            err.kind(),
                            io::ErrorKind::NotADirectory | io::ErrorKind::NotFound
                        ) => {}
                    Err(err) => {
                        error!(
                            "failed to read skills symlink dir {}: {err:#}",
                            path.display()
                        );
                    }
                }
                continue;
            }

            if metadata.is_directory {
                let resolved_dir = canonicalize_for_skill_identity(&path);
                enqueue_dir(
                    &mut queue,
                    &mut visited_dirs,
                    &mut truncated_by_dir_limit,
                    resolved_dir,
                    depth + 1,
                );
                continue;
            }

            if metadata.is_file && file_name == SKILLS_FILENAME {
                let skill_path = path;
                match parse_skill_file(fs, &skill_path, scope, plugin_id, plugin_namespace, None)
                    .await
                {
                    Ok(skill) => {
                        outcome.skills.push(skill);
                    }
                    Err(err) => {
                        if scope != SkillScope::System {
                            outcome.errors.push(SkillError {
                                path: skill_path,
                                message: err.to_string(),
                            });
                        }
                    }
                }
            }
        }
    }

    if truncated_by_dir_limit {
        tracing::warn!(
            "skills scan truncated after {} directories (root: {})",
            MAX_SKILLS_DIRS_PER_ROOT,
            root.display()
        );
    }
}

#[cfg(test)]
pub(crate) async fn skill_roots_from_layer_stack(
    fs: Arc<dyn ExecutorFileSystem>,
    config_layer_stack: &ConfigLayerStack,
    cwd: &AbsolutePathBuf,
    home_dir: Option<&AbsolutePathBuf>,
) -> Vec<SkillRoot> {
    skill_roots_with_home_dir(Some(fs), config_layer_stack, cwd, home_dir, Vec::new()).await
}

#[cfg(test)]
#[path = "loader_tests.rs"]
mod tests;
