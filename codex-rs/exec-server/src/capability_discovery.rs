use std::collections::HashSet;
use std::io;

use codex_exec_server_protocol::CapabilityRootDiscoverRequest;
use codex_exec_server_protocol::CapabilityRootDiscovery;
use codex_exec_server_protocol::CapabilityRootsDiscoverParams;
use codex_exec_server_protocol::CapabilityRootsDiscoverResponse;
use codex_exec_server_protocol::CapabilityTextFile;
use codex_exec_server_protocol::DISCOVERABLE_PLUGIN_MANIFEST_PATHS;
use codex_exec_server_protocol::DiscoveredPluginFiles;
use codex_exec_server_protocol::DiscoveredSkillFiles;
use codex_file_system::ExecutorFileSystem;
use codex_file_system::FileSystemSandboxContext;
use codex_file_system::WalkEntryKind;
use codex_file_system::WalkOptions;
use codex_utils_path_uri::PathUri;
use futures::StreamExt;
use serde::Deserialize;
use serde_json::Value;

pub(crate) const MAX_ROOTS_PER_REQUEST: usize = 128;
const MAX_SCAN_DEPTH: usize = 6;
const MAX_DIRECTORIES_PER_ROOT: usize = 2_000;
const MAX_ENTRIES_PER_ROOT: usize = 20_000;
const MAX_FILE_BYTES: usize = 1024 * 1024;
const MAX_BUNDLE_BYTES_PER_ROOT: usize = 16 * 1024 * 1024;
const MAX_CONCURRENT_ROOTS: usize = 8;
const SKILL_FILE_NAME: &str = "SKILL.md";
const SKILL_METADATA_PATH: &str = "agents/openai.yaml";
const DEFAULT_MCP_CONFIG_PATH: &str = ".mcp.json";

#[derive(Debug, thiserror::Error)]
pub enum CapabilityDiscoveryError {
    #[error("capability root discovery accepts at most {MAX_ROOTS_PER_REQUEST} roots")]
    TooManyRoots,
}

/// Discovers and materializes capability manifests using one executor-local filesystem.
///
/// Product parsing and policy intentionally remain with the caller. This operation owns the
/// filesystem-expensive portion: bounded traversal, recognized-file selection, and reads.
#[tracing::instrument(
    name = "capability_roots.discover_v1",
    skip_all,
    fields(root_count = params.roots.len())
)]
pub async fn discover_capability_roots(
    file_system: &dyn ExecutorFileSystem,
    params: CapabilityRootsDiscoverParams,
) -> Result<CapabilityRootsDiscoverResponse, CapabilityDiscoveryError> {
    if params.roots.len() > MAX_ROOTS_PER_REQUEST {
        return Err(CapabilityDiscoveryError::TooManyRoots);
    }

    let roots = futures::stream::iter(params.roots)
        .map(|root| discover_root(file_system, root))
        .buffered(MAX_CONCURRENT_ROOTS)
        .collect()
        .await;
    Ok(CapabilityRootsDiscoverResponse { roots })
}

async fn discover_root(
    file_system: &dyn ExecutorFileSystem,
    request: CapabilityRootDiscoverRequest,
) -> CapabilityRootDiscovery {
    let CapabilityRootDiscoverRequest { id, path, sandbox } = request;
    let sandbox = sandbox.as_ref();
    let mut discovery = CapabilityRootDiscovery {
        id,
        path: path.clone(),
        plugin: None,
        skills: Vec::new(),
        namespace_manifests: Vec::new(),
        warnings: Vec::new(),
        error: None,
    };

    #[cfg(target_os = "windows")]
    if sandbox.is_some_and(|context| {
        context.should_run_in_sandbox()
            && context.windows_sandbox_level
                == codex_protocol::config_types::WindowsSandboxLevel::Disabled
    }) {
        discovery.error = Some("filesystem sandbox is unavailable on this executor".to_string());
        return discovery;
    }

    match file_system.get_metadata(&path, sandbox).await {
        Ok(metadata) if metadata.is_directory => {}
        Ok(_) => {
            discovery.error = Some(format!("capability root {path} is not a directory"));
            return discovery;
        }
        Err(error) => {
            discovery.error = Some(format!("failed to inspect capability root {path}: {error}"));
            return discovery;
        }
    }

    let walk = match file_system
        .walk(
            &path,
            WalkOptions {
                max_depth: MAX_SCAN_DEPTH,
                max_directories: MAX_DIRECTORIES_PER_ROOT,
                max_entries: MAX_ENTRIES_PER_ROOT,
                follow_directory_symlinks: true,
                prune_hidden_directories: false,
            },
            sandbox,
        )
        .await
    {
        Ok(walk) => walk,
        Err(error) => {
            discovery.error = Some(format!("failed to scan capability root {path}: {error}"));
            return discovery;
        }
    };
    discovery
        .warnings
        .extend(walk.errors.into_iter().map(|error| {
            format!(
                "failed to scan capability path {}: {}",
                error.path, error.message
            )
        }));
    if walk.truncated {
        discovery.warnings.push(format!(
            "capability scan reached its traversal limit (root: {path})"
        ));
    }

    let mut skill_paths = Vec::new();
    let mut namespace_manifest_paths = Vec::new();
    for entry in walk.entries {
        if entry.kind != WalkEntryKind::File {
            continue;
        }
        if entry.path.basename().as_deref() == Some(SKILL_FILE_NAME) {
            skill_paths.push(entry.path.clone());
        }
        if is_plugin_manifest_path(&entry.path) {
            namespace_manifest_paths.push(entry.path);
        }
    }
    skill_paths.sort_unstable_by_key(PathUri::to_string);
    namespace_manifest_paths.sort_unstable_by(|left, right| {
        let left_root = plugin_root_for_manifest(left).map(|path| path.to_string());
        let right_root = plugin_root_for_manifest(right).map(|path| path.to_string());
        left_root
            .cmp(&right_root)
            .then_with(|| plugin_manifest_priority(left).cmp(&plugin_manifest_priority(right)))
    });

    let mut budget = BundleBudget::default();
    let root_manifest = read_first_plugin_manifest(
        file_system,
        &path,
        sandbox,
        &mut budget,
        &mut discovery.warnings,
    )
    .await;

    let inherited_manifest = match root_manifest.as_ref() {
        Some(manifest) => Some(manifest.clone()),
        None => {
            read_nearest_ancestor_manifest(
                file_system,
                &path,
                sandbox,
                &mut budget,
                &mut discovery.warnings,
            )
            .await
        }
    };
    let mut seen_namespace_roots = HashSet::new();
    if let Some(manifest) = inherited_manifest {
        if let Some(plugin_root) = plugin_root_for_manifest(&manifest.path) {
            seen_namespace_roots.insert(plugin_root);
        }
        discovery.namespace_manifests.push(manifest);
    }
    for manifest_path in namespace_manifest_paths {
        let Some(plugin_root) = plugin_root_for_manifest(&manifest_path) else {
            continue;
        };
        if !seen_namespace_roots.insert(plugin_root) {
            continue;
        }
        if let Some(manifest) = read_optional_text_file(
            file_system,
            manifest_path,
            sandbox,
            &mut budget,
            &mut discovery.warnings,
        )
        .await
        {
            discovery.namespace_manifests.push(manifest);
        }
    }

    if let Some(manifest) = root_manifest {
        let declarations = plugin_declaration_paths(&path, &manifest, &mut discovery.warnings);
        let mcp_path = if declarations.mcp_inline {
            None
        } else {
            declarations
                .mcp_config
                .or_else(|| path.join(DEFAULT_MCP_CONFIG_PATH).ok())
        };
        let mcp_config = match mcp_path {
            Some(path) => {
                read_optional_text_file(
                    file_system,
                    path,
                    sandbox,
                    &mut budget,
                    &mut discovery.warnings,
                )
                .await
            }
            None => None,
        };
        let apps_config = match declarations.apps_config {
            Some(path) => {
                read_optional_text_file(
                    file_system,
                    path,
                    sandbox,
                    &mut budget,
                    &mut discovery.warnings,
                )
                .await
            }
            None => None,
        };
        discovery.plugin = Some(DiscoveredPluginFiles {
            manifest,
            mcp_config,
            apps_config,
        });
    }

    for skill_path in skill_paths {
        let Some(instructions) = read_optional_text_file(
            file_system,
            skill_path.clone(),
            sandbox,
            &mut budget,
            &mut discovery.warnings,
        )
        .await
        else {
            continue;
        };
        let metadata = match skill_path
            .parent()
            .and_then(|skill_dir| skill_dir.join(SKILL_METADATA_PATH).ok())
        {
            Some(metadata_path) => {
                read_optional_text_file(
                    file_system,
                    metadata_path,
                    sandbox,
                    &mut budget,
                    &mut discovery.warnings,
                )
                .await
            }
            None => None,
        };
        discovery.skills.push(DiscoveredSkillFiles {
            instructions,
            metadata,
        });
    }

    discovery
}

async fn read_first_plugin_manifest(
    file_system: &dyn ExecutorFileSystem,
    root: &PathUri,
    sandbox: Option<&FileSystemSandboxContext>,
    budget: &mut BundleBudget,
    warnings: &mut Vec<String>,
) -> Option<CapabilityTextFile> {
    for relative_path in DISCOVERABLE_PLUGIN_MANIFEST_PATHS {
        let Ok(path) = root.join(relative_path) else {
            continue;
        };
        if let Some(manifest) =
            read_optional_text_file(file_system, path, sandbox, budget, warnings).await
        {
            return Some(manifest);
        }
    }
    None
}

async fn read_nearest_ancestor_manifest(
    file_system: &dyn ExecutorFileSystem,
    root: &PathUri,
    sandbox: Option<&FileSystemSandboxContext>,
    budget: &mut BundleBudget,
    warnings: &mut Vec<String>,
) -> Option<CapabilityTextFile> {
    let mut ancestor = root.parent();
    while let Some(path) = ancestor {
        if let Some(manifest) =
            read_first_plugin_manifest(file_system, &path, sandbox, budget, warnings).await
        {
            return Some(manifest);
        }
        ancestor = path.parent();
    }
    None
}

async fn read_optional_text_file(
    file_system: &dyn ExecutorFileSystem,
    path: PathUri,
    sandbox: Option<&FileSystemSandboxContext>,
    budget: &mut BundleBudget,
    warnings: &mut Vec<String>,
) -> Option<CapabilityTextFile> {
    let contents = if let Some(sandbox) = authorized_read_context(sandbox) {
        match file_system
            .read_file_bounded_authorized(&path, sandbox, MAX_FILE_BYTES)
            .await
        {
            Ok(contents) if contents.len() <= MAX_FILE_BYTES => contents,
            Ok(_) => {
                warnings.push("capability file exceeded its authorized read limit".to_string());
                return None;
            }
            Err(error) if error.kind() == io::ErrorKind::NotFound => return None,
            Err(_) => {
                warnings.push(
                    "capability file could not be read through the authorized sandbox".to_string(),
                );
                return None;
            }
        }
    } else {
        let metadata = match file_system.get_metadata(&path, sandbox).await {
            Ok(metadata) if metadata.is_file => metadata,
            Ok(_) => return None,
            Err(error) if error.kind() == io::ErrorKind::NotFound => return None,
            Err(error) => {
                warnings.push(format!("failed to inspect capability file {path}: {error}"));
                return None;
            }
        };
        let Ok(size) = usize::try_from(metadata.size) else {
            warnings.push(format!("capability file {path} is too large"));
            return None;
        };
        if size > MAX_FILE_BYTES {
            warnings.push(format!(
                "capability file {path} exceeds the {MAX_FILE_BYTES}-byte limit"
            ));
            return None;
        }
        if !budget.can_add(size) {
            warnings.push(format!(
                "capability root bundle exceeds the {MAX_BUNDLE_BYTES_PER_ROOT}-byte limit"
            ));
            return None;
        }
        let mut stream = match file_system.read_file_stream(&path, sandbox).await {
            Ok(stream) => stream,
            Err(error) => {
                warnings.push(format!("failed to read capability file {path}: {error}"));
                return None;
            }
        };
        let mut contents = Vec::with_capacity(size);
        while let Some(chunk) = stream.next().await {
            let chunk = match chunk {
                Ok(chunk) => chunk,
                Err(error) => {
                    warnings.push(format!("failed to read capability file {path}: {error}"));
                    return None;
                }
            };
            let Some(new_len) = contents.len().checked_add(chunk.len()) else {
                warnings.push(format!("capability file {path} exceeded its read limit"));
                return None;
            };
            if new_len > MAX_FILE_BYTES || !budget.can_add(new_len) {
                warnings.push(format!("capability file {path} exceeded its read limit"));
                return None;
            }
            contents.extend_from_slice(&chunk);
        }
        contents
    };
    if !budget.can_add(contents.len()) {
        warnings.push(format!(
            "capability root bundle exceeds the {MAX_BUNDLE_BYTES_PER_ROOT}-byte limit"
        ));
        return None;
    }
    let contents = match String::from_utf8(contents) {
        Ok(contents) => contents,
        Err(error) => {
            warnings.push(format!("capability file {path} is not UTF-8: {error}"));
            return None;
        }
    };
    budget.add(contents.len());
    Some(CapabilityTextFile { path, contents })
}

fn authorized_read_context(
    sandbox: Option<&FileSystemSandboxContext>,
) -> Option<&FileSystemSandboxContext> {
    sandbox.filter(|context| context.should_run_in_sandbox())
}

fn is_plugin_manifest_path(path: &PathUri) -> bool {
    plugin_manifest_priority(path).is_some()
}

fn plugin_manifest_priority(path: &PathUri) -> Option<usize> {
    if path.basename().as_deref() != Some("plugin.json") {
        return None;
    }
    let manifest_directory = path.parent()?.basename()?;
    DISCOVERABLE_PLUGIN_MANIFEST_PATHS
        .iter()
        .position(|relative_path| {
            relative_path.strip_suffix("/plugin.json") == Some(manifest_directory.as_str())
        })
}

fn plugin_root_for_manifest(path: &PathUri) -> Option<PathUri> {
    path.parent()?.parent()
}

#[derive(Default)]
struct BundleBudget {
    bytes: usize,
}

impl BundleBudget {
    fn can_add(&self, bytes: usize) -> bool {
        self.bytes
            .checked_add(bytes)
            .is_some_and(|total| total <= MAX_BUNDLE_BYTES_PER_ROOT)
    }

    fn add(&mut self, bytes: usize) {
        self.bytes += bytes;
    }
}

#[derive(Default)]
struct PluginDeclarationPaths {
    mcp_config: Option<PathUri>,
    mcp_inline: bool,
    apps_config: Option<PathUri>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawPluginDeclarations {
    #[serde(default)]
    mcp_servers: Option<Value>,
    #[serde(default)]
    apps: Option<Value>,
}

fn plugin_declaration_paths(
    root: &PathUri,
    manifest: &CapabilityTextFile,
    warnings: &mut Vec<String>,
) -> PluginDeclarationPaths {
    let declarations = match serde_json::from_str::<RawPluginDeclarations>(&manifest.contents) {
        Ok(declarations) => declarations,
        Err(_) => return PluginDeclarationPaths::default(),
    };
    PluginDeclarationPaths {
        mcp_config: declarations.mcp_servers.as_ref().and_then(|value| {
            declared_file_path(root, "mcpServers", value, &manifest.path, warnings)
        }),
        mcp_inline: declarations
            .mcp_servers
            .as_ref()
            .is_some_and(Value::is_object),
        apps_config: declarations
            .apps
            .as_ref()
            .and_then(|value| declared_file_path(root, "apps", value, &manifest.path, warnings)),
    }
}

fn declared_file_path(
    root: &PathUri,
    field: &str,
    value: &Value,
    manifest_path: &PathUri,
    warnings: &mut Vec<String>,
) -> Option<PathUri> {
    let Value::String(path) = value else {
        return None;
    };
    let Some(relative_path) = path.strip_prefix("./") else {
        warnings.push(format!(
            "ignoring {field} in {manifest_path}: path must start with `./`"
        ));
        return None;
    };
    if relative_path.is_empty()
        || relative_path
            .split(['/', '\\'])
            .any(|component| component == "..")
    {
        warnings.push(format!(
            "ignoring {field} in {manifest_path}: path must remain below the capability root"
        ));
        return None;
    }
    match root.join(relative_path) {
        Ok(path) if path.starts_with(root) => Some(path),
        Ok(_) | Err(_) => {
            warnings.push(format!(
                "ignoring {field} in {manifest_path}: path must remain below the capability root"
            ));
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::AtomicUsize;
    use std::sync::atomic::Ordering;

    use codex_protocol::models::PermissionProfile;

    use super::*;
    use crate::CopyOptions;
    use crate::CreateDirectoryOptions;
    use crate::ExecutorFileSystemFuture;
    use crate::FileMetadata;
    use crate::FileSystemReadStream;
    use crate::ReadDirectoryEntry;
    use crate::RemoveOptions;

    type Sandbox<'a> = Option<&'a FileSystemSandboxContext>;

    struct AuthorizedOnlyFileSystem {
        contents: Result<Vec<u8>, io::ErrorKind>,
        authorized_calls: AtomicUsize,
        legacy_calls: AtomicUsize,
        max_bytes: AtomicUsize,
    }

    impl AuthorizedOnlyFileSystem {
        fn new(contents: Result<Vec<u8>, io::ErrorKind>) -> Self {
            Self {
                contents,
                authorized_calls: AtomicUsize::new(0),
                legacy_calls: AtomicUsize::new(0),
                max_bytes: AtomicUsize::new(0),
            }
        }

        fn legacy<T: Send + 'static>(&self) -> ExecutorFileSystemFuture<'_, T> {
            self.legacy_calls.fetch_add(1, Ordering::Relaxed);
            Box::pin(async { Err(io::Error::other("legacy filesystem path")) })
        }
    }

    macro_rules! legacy_method {
        ($name:ident($($arg:ident: $arg_type:ty),*) -> $result:ty) => {
            fn $name<'a>(
                &'a self,
                $($arg: $arg_type),*
            ) -> ExecutorFileSystemFuture<'a, $result> {
                self.legacy()
            }
        };
    }

    impl ExecutorFileSystem for AuthorizedOnlyFileSystem {
        legacy_method!(canonicalize(_path: &'a PathUri, _sandbox: Sandbox<'a>) -> PathUri);
        legacy_method!(read_file(_path: &'a PathUri, _sandbox: Sandbox<'a>) -> Vec<u8>);

        fn read_file_bounded_authorized<'a>(
            &'a self,
            _path: &'a PathUri,
            _sandbox: &'a FileSystemSandboxContext,
            max_bytes: usize,
        ) -> ExecutorFileSystemFuture<'a, Vec<u8>> {
            self.authorized_calls.fetch_add(1, Ordering::Relaxed);
            self.max_bytes.store(max_bytes, Ordering::Relaxed);
            let contents = self.contents.clone();
            Box::pin(async move {
                contents.map_err(|kind| io::Error::new(kind, "/private/secret/provider-path"))
            })
        }

        legacy_method!(read_file_stream(_path: &'a PathUri, _sandbox: Sandbox<'a>)
            -> FileSystemReadStream);
        legacy_method!(write_file(
            _path: &'a PathUri,
            _contents: Vec<u8>,
            _sandbox: Sandbox<'a>
        ) -> ());
        legacy_method!(create_directory(
            _path: &'a PathUri,
            _options: CreateDirectoryOptions,
            _sandbox: Sandbox<'a>
        ) -> ());
        legacy_method!(get_metadata(_path: &'a PathUri, _sandbox: Sandbox<'a>) -> FileMetadata);
        legacy_method!(read_directory(_path: &'a PathUri, _sandbox: Sandbox<'a>)
            -> Vec<ReadDirectoryEntry>);
        legacy_method!(remove(
            _path: &'a PathUri,
            _options: RemoveOptions,
            _sandbox: Sandbox<'a>
        ) -> ());
        legacy_method!(copy(
            _source_path: &'a PathUri,
            _destination_path: &'a PathUri,
            _options: CopyOptions,
            _sandbox: Sandbox<'a>
        ) -> ());
    }

    async fn sandboxed_read(
        file_system: &AuthorizedOnlyFileSystem,
        budget: &mut BundleBudget,
    ) -> (Option<CapabilityTextFile>, Vec<String>) {
        let mut warnings = Vec::new();
        let result = read_optional_text_file(
            file_system,
            PathUri::parse("file:///capabilities/SKILL.md").expect("test path"),
            Some(&FileSystemSandboxContext::from_permission_profile(
                PermissionProfile::default(),
            )),
            budget,
            &mut warnings,
        )
        .await;
        (result, warnings)
    }

    #[test]
    fn authorized_reads_require_platform_sandbox_enforcement() {
        let restricted =
            FileSystemSandboxContext::from_permission_profile(PermissionProfile::read_only());
        let disabled =
            FileSystemSandboxContext::from_permission_profile(PermissionProfile::Disabled);

        assert!(authorized_read_context(Some(&restricted)).is_some());
        assert!(authorized_read_context(Some(&disabled)).is_none());
        assert!(authorized_read_context(None).is_none());
    }

    #[tokio::test]
    async fn sandboxed_capability_read_uses_only_the_authorized_seam_and_rechecks_bounds() {
        let file_system = AuthorizedOnlyFileSystem::new(Ok(b"capability".to_vec()));
        let mut budget = BundleBudget::default();
        let (file, warnings) = sandboxed_read(&file_system, &mut budget).await;
        assert_eq!(file.expect("authorized contents").contents, "capability");
        assert!(warnings.is_empty());
        assert_eq!(budget.bytes, "capability".len());
        assert_eq!(file_system.authorized_calls.load(Ordering::Relaxed), 1);
        assert_eq!(
            file_system.max_bytes.load(Ordering::Relaxed),
            MAX_FILE_BYTES
        );
        assert_eq!(file_system.legacy_calls.load(Ordering::Relaxed), 0);

        let file_system = AuthorizedOnlyFileSystem::new(Ok(vec![b'x'; MAX_FILE_BYTES + 1]));
        let (file, warnings) = sandboxed_read(&file_system, &mut BundleBudget::default()).await;
        assert!(file.is_none(), "an over-limit prefix must not be returned");
        assert_eq!(
            warnings,
            ["capability file exceeded its authorized read limit"]
        );
        assert_eq!(file_system.legacy_calls.load(Ordering::Relaxed), 0);

        let file_system = AuthorizedOnlyFileSystem::new(Ok(b"ab".to_vec()));
        let mut budget = BundleBudget {
            bytes: MAX_BUNDLE_BYTES_PER_ROOT - 1,
        };
        let (file, warnings) = sandboxed_read(&file_system, &mut budget).await;
        assert!(file.is_none());
        assert_eq!(
            warnings,
            [format!(
                "capability root bundle exceeds the {MAX_BUNDLE_BYTES_PER_ROOT}-byte limit"
            )]
        );
    }

    #[tokio::test]
    async fn sandboxed_capability_read_fails_closed_without_fallback_or_path_leaks() {
        for kind in [io::ErrorKind::Unsupported, io::ErrorKind::PermissionDenied] {
            let file_system = AuthorizedOnlyFileSystem::new(Err(kind));
            let (file, warnings) = sandboxed_read(&file_system, &mut BundleBudget::default()).await;
            assert!(file.is_none());
            assert_eq!(
                warnings,
                ["capability file could not be read through the authorized sandbox"]
            );
            assert!(!warnings[0].contains("private/secret"));
            assert_eq!(file_system.legacy_calls.load(Ordering::Relaxed), 0);
        }

        let file_system = AuthorizedOnlyFileSystem::new(Err(io::ErrorKind::NotFound));
        let (file, warnings) = sandboxed_read(&file_system, &mut BundleBudget::default()).await;
        assert!(file.is_none());
        assert!(warnings.is_empty(), "atomic NotFound remains optional");
        assert_eq!(file_system.legacy_calls.load(Ordering::Relaxed), 0);
    }
}
