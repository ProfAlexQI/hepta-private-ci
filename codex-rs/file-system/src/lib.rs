use async_trait::async_trait;
use bytes::Bytes;
use codex_protocol::config_types::WindowsSandboxLevel;
use codex_protocol::models::PermissionProfile;
use codex_protocol::models::SandboxEnforcement;
use codex_protocol::permissions::FileSystemPath;
use codex_protocol::permissions::FileSystemSandboxKind;
use codex_protocol::permissions::FileSystemSandboxPolicy;
use codex_protocol::permissions::FileSystemSpecialPath;
use codex_protocol::permissions::NetworkSandboxPolicy;
use codex_protocol::protocol::SandboxPolicy;
use codex_utils_absolute_path::AbsolutePathBuf;
use futures::Stream;
use std::io;
use std::path::Path;
use std::pin::Pin;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CreateDirectoryOptions {
    pub recursive: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RemoveOptions {
    pub recursive: bool,
    pub force: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CopyOptions {
    pub recursive: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileMetadata {
    pub is_directory: bool,
    pub is_file: bool,
    pub is_symlink: bool,
    pub size: u64,
    pub created_at_ms: i64,
    pub modified_at_ms: i64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReadDirectoryEntry {
    pub file_name: String,
    pub is_directory: bool,
    pub is_file: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileSystemSandboxContext {
    pub permissions: PermissionProfile,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cwd: Option<AbsolutePathBuf>,
    #[serde(default)]
    pub workspace_roots: Vec<AbsolutePathBuf>,
    pub windows_sandbox_level: WindowsSandboxLevel,
    #[serde(default)]
    pub windows_sandbox_private_desktop: bool,
    #[serde(default)]
    pub use_legacy_landlock: bool,
}

impl FileSystemSandboxContext {
    pub fn from_legacy_sandbox_policy(sandbox_policy: SandboxPolicy, cwd: AbsolutePathBuf) -> Self {
        let file_system_sandbox_policy =
            FileSystemSandboxPolicy::from_legacy_sandbox_policy_for_cwd(&sandbox_policy, &cwd);
        let permissions = PermissionProfile::from_runtime_permissions_with_enforcement(
            SandboxEnforcement::from_legacy_sandbox_policy(&sandbox_policy),
            &file_system_sandbox_policy,
            NetworkSandboxPolicy::from(&sandbox_policy),
        );
        Self::from_permission_profile_with_cwd(permissions, cwd)
    }

    pub fn from_permission_profile(permissions: PermissionProfile) -> Self {
        Self::from_permissions_and_cwd(permissions, /*cwd*/ None)
    }

    pub fn from_permission_profile_with_cwd(
        permissions: PermissionProfile,
        cwd: AbsolutePathBuf,
    ) -> Self {
        Self::from_permissions_and_cwd(permissions, Some(cwd))
    }

    fn from_permissions_and_cwd(
        permissions: PermissionProfile,
        cwd: Option<AbsolutePathBuf>,
    ) -> Self {
        let workspace_roots = cwd.iter().cloned().collect();
        Self {
            permissions,
            cwd,
            workspace_roots,
            windows_sandbox_level: WindowsSandboxLevel::Disabled,
            windows_sandbox_private_desktop: false,
            use_legacy_landlock: false,
        }
    }

    pub fn should_run_in_sandbox(&self) -> bool {
        let file_system_policy = self.permissions.file_system_sandbox_policy();
        matches!(file_system_policy.kind, FileSystemSandboxKind::Restricted)
            && !file_system_policy.has_full_disk_write_access()
    }

    /// Resolves symbolic project-root permissions against the roots carried by
    /// this context. An explicitly empty root list stays empty and must not
    /// inherit the executor working directory.
    pub fn materialized_permissions(&self) -> PermissionProfile {
        self.permissions
            .clone()
            .materialize_project_roots_with_workspace_roots(&self.workspace_roots)
    }

    pub fn has_cwd_dependent_permissions(&self) -> bool {
        let file_system_policy = self.permissions.file_system_sandbox_policy();
        file_system_policy_has_cwd_dependent_entries(&file_system_policy)
    }

    pub fn drop_cwd_if_unused(mut self) -> Self {
        if !self.has_cwd_dependent_permissions() {
            self.cwd = None;
        }
        self
    }
}

fn file_system_policy_has_cwd_dependent_entries(
    file_system_policy: &FileSystemSandboxPolicy,
) -> bool {
    file_system_policy
        .entries
        .iter()
        .any(|entry| match &entry.path {
            FileSystemPath::GlobPattern { pattern } => !Path::new(pattern).is_absolute(),
            FileSystemPath::Special {
                value: FileSystemSpecialPath::ProjectRoots { .. },
            } => true,
            FileSystemPath::Path { .. } | FileSystemPath::Special { .. } => false,
        })
}

pub type FileSystemResult<T> = io::Result<T>;

/// Stream of immutable file chunks read through an executor filesystem capability.
pub type FileSystemReadStream =
    Pin<Box<dyn Stream<Item = FileSystemResult<Bytes>> + Send + 'static>>;

/// Abstract filesystem access used by components that may operate locally or via
/// a remote executor.
#[async_trait]
pub trait ExecutorFileSystem: Send + Sync {
    async fn read_file(
        &self,
        path: &AbsolutePathBuf,
        sandbox: Option<&FileSystemSandboxContext>,
    ) -> FileSystemResult<Vec<u8>>;

    /// Reads a file as a stream. Executors should override this to avoid buffering the whole file;
    /// the default preserves compatibility for legacy executors that only support bounded reads.
    async fn read_file_stream(
        &self,
        path: &AbsolutePathBuf,
        sandbox: Option<&FileSystemSandboxContext>,
    ) -> FileSystemResult<FileSystemReadStream> {
        let contents = self.read_file(path, sandbox).await?;
        Ok(Box::pin(futures::stream::once(async move {
            Ok(Bytes::from(contents))
        })))
    }

    /// Atomically opens `authority_root`, walks only normal relative path
    /// components without following symbolic links, and reads no more than
    /// `max_bytes + 1` bytes from a regular final file.
    ///
    /// Implementations must not fall back to [`Self::read_file`]. The default
    /// fails closed so executors that do not provide the required descriptor-
    /// relative semantics cannot accidentally claim this authority.
    async fn read_file_beneath(
        &self,
        authority_root: &AbsolutePathBuf,
        relative_path: &Path,
        max_bytes: u64,
        sandbox: Option<&FileSystemSandboxContext>,
    ) -> FileSystemResult<Vec<u8>> {
        let _ = (authority_root, relative_path, max_bytes, sandbox);
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "atomic bounded reads beneath an authority root are unsupported by this executor",
        ))
    }

    /// Reads a file and decodes it as UTF-8 text.
    async fn read_file_text(
        &self,
        path: &AbsolutePathBuf,
        sandbox: Option<&FileSystemSandboxContext>,
    ) -> FileSystemResult<String> {
        let bytes = self.read_file(path, sandbox).await?;
        String::from_utf8(bytes).map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))
    }

    async fn write_file(
        &self,
        path: &AbsolutePathBuf,
        contents: Vec<u8>,
        sandbox: Option<&FileSystemSandboxContext>,
    ) -> FileSystemResult<()>;

    async fn create_directory(
        &self,
        path: &AbsolutePathBuf,
        create_directory_options: CreateDirectoryOptions,
        sandbox: Option<&FileSystemSandboxContext>,
    ) -> FileSystemResult<()>;

    async fn get_metadata(
        &self,
        path: &AbsolutePathBuf,
        sandbox: Option<&FileSystemSandboxContext>,
    ) -> FileSystemResult<FileMetadata>;

    async fn read_directory(
        &self,
        path: &AbsolutePathBuf,
        sandbox: Option<&FileSystemSandboxContext>,
    ) -> FileSystemResult<Vec<ReadDirectoryEntry>>;

    async fn remove(
        &self,
        path: &AbsolutePathBuf,
        remove_options: RemoveOptions,
        sandbox: Option<&FileSystemSandboxContext>,
    ) -> FileSystemResult<()>;

    async fn copy(
        &self,
        source_path: &AbsolutePathBuf,
        destination_path: &AbsolutePathBuf,
        copy_options: CopyOptions,
        sandbox: Option<&FileSystemSandboxContext>,
    ) -> FileSystemResult<()>;
}
