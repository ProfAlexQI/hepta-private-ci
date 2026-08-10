#[cfg(target_os = "macos")]
use codex_protocol::models::PermissionProfile;
#[cfg(target_os = "macos")]
use codex_protocol::permissions::ReadDenyMatcher;
use codex_utils_absolute_path::AbsolutePathBuf;
use codex_utils_path_uri::PathUri;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::LazyLock;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use tokio::io;
use tokio::io::AsyncReadExt;
#[cfg(target_os = "macos")]
use tokio::io::AsyncSeekExt;
use tokio_util::io::ReaderStream;

use crate::CopyOptions;
use crate::CreateDirectoryOptions;
use crate::ExecServerRuntimePaths;
use crate::ExecutorFileSystem;
use crate::ExecutorFileSystemFuture;
use crate::FILE_READ_CHUNK_SIZE;
use crate::FileMetadata;
use crate::FileSystemReadStream;
use crate::FileSystemResult;
use crate::FileSystemSandboxContext;
use crate::ReadDirectoryEntry;
use crate::RemoveOptions;
use crate::WalkOptions;
use crate::WalkOutcome;
use crate::regular_file;
use crate::sandboxed_file_system::SandboxedFileSystem;

const MAX_READ_FILE_BYTES: u64 = 512 * 1024 * 1024;

fn file_too_large_error() -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidInput,
        format!("file is too large to read: limit is {MAX_READ_FILE_BYTES} bytes"),
    )
}

pub static LOCAL_FS: LazyLock<Arc<dyn ExecutorFileSystem>> =
    LazyLock::new(|| -> Arc<dyn ExecutorFileSystem> { Arc::new(LocalFileSystem::unsandboxed()) });

#[derive(Clone, Default)]
pub(crate) struct DirectFileSystem;

#[derive(Clone, Default)]
pub(crate) struct UnsandboxedFileSystem {
    file_system: DirectFileSystem,
}

#[derive(Clone, Default)]
pub struct LocalFileSystem {
    unsandboxed: UnsandboxedFileSystem,
    sandboxed: Option<SandboxedFileSystem>,
}

impl LocalFileSystem {
    pub fn unsandboxed() -> Self {
        Self {
            unsandboxed: UnsandboxedFileSystem::default(),
            sandboxed: None,
        }
    }

    pub fn with_runtime_paths(runtime_paths: ExecServerRuntimePaths) -> Self {
        Self {
            unsandboxed: UnsandboxedFileSystem::default(),
            sandboxed: Some(SandboxedFileSystem::new(runtime_paths)),
        }
    }

    fn sandboxed(&self) -> io::Result<&SandboxedFileSystem> {
        self.sandboxed.as_ref().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "sandboxed filesystem operations require configured runtime paths",
            )
        })
    }

    fn file_system_for<'a>(
        &'a self,
        sandbox: Option<&'a FileSystemSandboxContext>,
    ) -> io::Result<(
        &'a dyn ExecutorFileSystem,
        Option<&'a FileSystemSandboxContext>,
    )> {
        if sandbox.is_some_and(FileSystemSandboxContext::should_run_in_sandbox) {
            Ok((self.sandboxed()?, sandbox))
        } else {
            Ok((&self.unsandboxed, sandbox))
        }
    }
}

impl LocalFileSystem {
    pub(crate) async fn open_file_for_read(
        &self,
        path: &PathUri,
        sandbox: Option<&FileSystemSandboxContext>,
    ) -> FileSystemResult<tokio::fs::File> {
        if sandbox.is_some_and(FileSystemSandboxContext::should_run_in_sandbox) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "streaming file reads do not support platform sandboxing",
            ));
        }
        self.unsandboxed.open_file_for_read(path, sandbox).await
    }

    async fn canonicalize(
        &self,
        path: &PathUri,
        sandbox: Option<&FileSystemSandboxContext>,
    ) -> FileSystemResult<PathUri> {
        let (file_system, sandbox) = self.file_system_for(sandbox)?;
        file_system.canonicalize(path, sandbox).await
    }

    async fn read_file(
        &self,
        path: &PathUri,
        sandbox: Option<&FileSystemSandboxContext>,
    ) -> FileSystemResult<Vec<u8>> {
        let (file_system, sandbox) = self.file_system_for(sandbox)?;
        file_system.read_file(path, sandbox).await
    }

    #[cfg(target_os = "macos")]
    async fn read_file_bounded_authorized(
        &self,
        path: &PathUri,
        sandbox: &FileSystemSandboxContext,
        max_bytes: usize,
    ) -> FileSystemResult<Vec<u8>> {
        if !stable_handle_authorized_read_available() {
            return Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "bounded authorized file reads are unsupported",
            ));
        }
        let read_limit = authorized_read_limit(max_bytes)?;
        let native_path = path
            .to_abs_path()
            .map_err(|_| authorized_read_error(io::ErrorKind::InvalidInput))?;
        let original_file = regular_file::open(native_path.as_path())
            .await
            .map_err(redact_file_access_error)?;
        let original_identity = unique_file_identity(&original_file).await?;
        let final_path = stable_file_path(&original_file)?;
        authorize_stable_file_path(final_path.as_path(), sandbox)?;
        let mut file =
            secure_reopen_matching_identity(final_path.as_path(), original_identity).await?;
        file.seek(std::io::SeekFrom::Start(0))
            .await
            .map_err(redact_file_access_error)?;

        let mut bytes = Vec::with_capacity(max_bytes.min(FILE_READ_CHUNK_SIZE));
        let mut limited = file.take(read_limit);
        limited
            .read_to_end(&mut bytes)
            .await
            .map_err(redact_file_access_error)?;
        if bytes.len() > max_bytes {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "authorized file read exceeds the requested limit",
            ));
        }
        Ok(bytes)
    }

    async fn read_file_stream(
        &self,
        path: &PathUri,
        sandbox: Option<&FileSystemSandboxContext>,
    ) -> FileSystemResult<FileSystemReadStream> {
        let (file_system, sandbox) = self.file_system_for(sandbox)?;
        file_system.read_file_stream(path, sandbox).await
    }

    async fn write_file(
        &self,
        path: &PathUri,
        contents: Vec<u8>,
        sandbox: Option<&FileSystemSandboxContext>,
    ) -> FileSystemResult<()> {
        let (file_system, sandbox) = self.file_system_for(sandbox)?;
        file_system.write_file(path, contents, sandbox).await
    }

    async fn create_directory(
        &self,
        path: &PathUri,
        options: CreateDirectoryOptions,
        sandbox: Option<&FileSystemSandboxContext>,
    ) -> FileSystemResult<()> {
        let (file_system, sandbox) = self.file_system_for(sandbox)?;
        file_system.create_directory(path, options, sandbox).await
    }

    async fn get_metadata(
        &self,
        path: &PathUri,
        sandbox: Option<&FileSystemSandboxContext>,
    ) -> FileSystemResult<FileMetadata> {
        let (file_system, sandbox) = self.file_system_for(sandbox)?;
        file_system.get_metadata(path, sandbox).await
    }

    async fn read_directory(
        &self,
        path: &PathUri,
        sandbox: Option<&FileSystemSandboxContext>,
    ) -> FileSystemResult<Vec<ReadDirectoryEntry>> {
        let (file_system, sandbox) = self.file_system_for(sandbox)?;
        file_system.read_directory(path, sandbox).await
    }

    async fn walk(
        &self,
        path: &PathUri,
        options: WalkOptions,
        sandbox: Option<&FileSystemSandboxContext>,
    ) -> FileSystemResult<WalkOutcome> {
        let (file_system, sandbox) = self.file_system_for(sandbox)?;
        file_system.walk(path, options, sandbox).await
    }

    async fn remove(
        &self,
        path: &PathUri,
        options: RemoveOptions,
        sandbox: Option<&FileSystemSandboxContext>,
    ) -> FileSystemResult<()> {
        let (file_system, sandbox) = self.file_system_for(sandbox)?;
        file_system.remove(path, options, sandbox).await
    }

    async fn copy(
        &self,
        source_path: &PathUri,
        destination_path: &PathUri,
        options: CopyOptions,
        sandbox: Option<&FileSystemSandboxContext>,
    ) -> FileSystemResult<()> {
        let (file_system, sandbox) = self.file_system_for(sandbox)?;
        file_system
            .copy(source_path, destination_path, options, sandbox)
            .await
    }
}

impl ExecutorFileSystem for LocalFileSystem {
    fn canonicalize<'a>(
        &'a self,
        path: &'a PathUri,
        sandbox: Option<&'a FileSystemSandboxContext>,
    ) -> ExecutorFileSystemFuture<'a, PathUri> {
        Box::pin(LocalFileSystem::canonicalize(self, path, sandbox))
    }

    fn read_file<'a>(
        &'a self,
        path: &'a PathUri,
        sandbox: Option<&'a FileSystemSandboxContext>,
    ) -> ExecutorFileSystemFuture<'a, Vec<u8>> {
        Box::pin(LocalFileSystem::read_file(self, path, sandbox))
    }

    #[cfg(target_os = "macos")]
    fn read_file_bounded_authorized<'a>(
        &'a self,
        path: &'a PathUri,
        sandbox: &'a FileSystemSandboxContext,
        max_bytes: usize,
    ) -> ExecutorFileSystemFuture<'a, Vec<u8>> {
        Box::pin(LocalFileSystem::read_file_bounded_authorized(
            self, path, sandbox, max_bytes,
        ))
    }

    fn read_file_stream<'a>(
        &'a self,
        path: &'a PathUri,
        sandbox: Option<&'a FileSystemSandboxContext>,
    ) -> ExecutorFileSystemFuture<'a, FileSystemReadStream> {
        Box::pin(LocalFileSystem::read_file_stream(self, path, sandbox))
    }

    fn write_file<'a>(
        &'a self,
        path: &'a PathUri,
        contents: Vec<u8>,
        sandbox: Option<&'a FileSystemSandboxContext>,
    ) -> ExecutorFileSystemFuture<'a, ()> {
        Box::pin(LocalFileSystem::write_file(self, path, contents, sandbox))
    }

    fn create_directory<'a>(
        &'a self,
        path: &'a PathUri,
        options: CreateDirectoryOptions,
        sandbox: Option<&'a FileSystemSandboxContext>,
    ) -> ExecutorFileSystemFuture<'a, ()> {
        Box::pin(LocalFileSystem::create_directory(
            self, path, options, sandbox,
        ))
    }

    fn get_metadata<'a>(
        &'a self,
        path: &'a PathUri,
        sandbox: Option<&'a FileSystemSandboxContext>,
    ) -> ExecutorFileSystemFuture<'a, FileMetadata> {
        Box::pin(LocalFileSystem::get_metadata(self, path, sandbox))
    }

    fn read_directory<'a>(
        &'a self,
        path: &'a PathUri,
        sandbox: Option<&'a FileSystemSandboxContext>,
    ) -> ExecutorFileSystemFuture<'a, Vec<ReadDirectoryEntry>> {
        Box::pin(LocalFileSystem::read_directory(self, path, sandbox))
    }

    fn walk<'a>(
        &'a self,
        path: &'a PathUri,
        options: WalkOptions,
        sandbox: Option<&'a FileSystemSandboxContext>,
    ) -> ExecutorFileSystemFuture<'a, WalkOutcome> {
        Box::pin(LocalFileSystem::walk(self, path, options, sandbox))
    }

    fn remove<'a>(
        &'a self,
        path: &'a PathUri,
        options: RemoveOptions,
        sandbox: Option<&'a FileSystemSandboxContext>,
    ) -> ExecutorFileSystemFuture<'a, ()> {
        Box::pin(LocalFileSystem::remove(self, path, options, sandbox))
    }

    fn copy<'a>(
        &'a self,
        source_path: &'a PathUri,
        destination_path: &'a PathUri,
        options: CopyOptions,
        sandbox: Option<&'a FileSystemSandboxContext>,
    ) -> ExecutorFileSystemFuture<'a, ()> {
        Box::pin(LocalFileSystem::copy(
            self,
            source_path,
            destination_path,
            options,
            sandbox,
        ))
    }
}

impl UnsandboxedFileSystem {
    async fn open_file_for_read(
        &self,
        path: &PathUri,
        sandbox: Option<&FileSystemSandboxContext>,
    ) -> FileSystemResult<tokio::fs::File> {
        reject_platform_sandbox_context(sandbox)?;
        self.file_system
            .open_file_for_read(path, /*sandbox*/ None)
            .await
    }

    async fn canonicalize(
        &self,
        path: &PathUri,
        sandbox: Option<&FileSystemSandboxContext>,
    ) -> FileSystemResult<PathUri> {
        reject_platform_sandbox_context(sandbox)?;
        self.file_system.canonicalize(path, /*sandbox*/ None).await
    }

    async fn read_file(
        &self,
        path: &PathUri,
        sandbox: Option<&FileSystemSandboxContext>,
    ) -> FileSystemResult<Vec<u8>> {
        reject_platform_sandbox_context(sandbox)?;
        self.file_system.read_file(path, /*sandbox*/ None).await
    }

    async fn read_file_stream(
        &self,
        path: &PathUri,
        sandbox: Option<&FileSystemSandboxContext>,
    ) -> FileSystemResult<FileSystemReadStream> {
        reject_platform_sandbox_context(sandbox)?;
        self.file_system
            .read_file_stream(path, /*sandbox*/ None)
            .await
    }

    async fn write_file(
        &self,
        path: &PathUri,
        contents: Vec<u8>,
        sandbox: Option<&FileSystemSandboxContext>,
    ) -> FileSystemResult<()> {
        reject_platform_sandbox_context(sandbox)?;
        self.file_system
            .write_file(path, contents, /*sandbox*/ None)
            .await
    }

    async fn create_directory(
        &self,
        path: &PathUri,
        options: CreateDirectoryOptions,
        sandbox: Option<&FileSystemSandboxContext>,
    ) -> FileSystemResult<()> {
        reject_platform_sandbox_context(sandbox)?;
        self.file_system
            .create_directory(path, options, /*sandbox*/ None)
            .await
    }

    async fn get_metadata(
        &self,
        path: &PathUri,
        sandbox: Option<&FileSystemSandboxContext>,
    ) -> FileSystemResult<FileMetadata> {
        reject_platform_sandbox_context(sandbox)?;
        self.file_system.get_metadata(path, /*sandbox*/ None).await
    }

    async fn read_directory(
        &self,
        path: &PathUri,
        sandbox: Option<&FileSystemSandboxContext>,
    ) -> FileSystemResult<Vec<ReadDirectoryEntry>> {
        reject_platform_sandbox_context(sandbox)?;
        self.file_system
            .read_directory(path, /*sandbox*/ None)
            .await
    }

    async fn remove(
        &self,
        path: &PathUri,
        options: RemoveOptions,
        sandbox: Option<&FileSystemSandboxContext>,
    ) -> FileSystemResult<()> {
        reject_platform_sandbox_context(sandbox)?;
        self.file_system
            .remove(path, options, /*sandbox*/ None)
            .await
    }

    async fn copy(
        &self,
        source_path: &PathUri,
        destination_path: &PathUri,
        options: CopyOptions,
        sandbox: Option<&FileSystemSandboxContext>,
    ) -> FileSystemResult<()> {
        reject_platform_sandbox_context(sandbox)?;
        self.file_system
            .copy(
                source_path,
                destination_path,
                options,
                /*sandbox*/ None,
            )
            .await
    }
}

impl ExecutorFileSystem for UnsandboxedFileSystem {
    fn canonicalize<'a>(
        &'a self,
        path: &'a PathUri,
        sandbox: Option<&'a FileSystemSandboxContext>,
    ) -> ExecutorFileSystemFuture<'a, PathUri> {
        Box::pin(UnsandboxedFileSystem::canonicalize(self, path, sandbox))
    }

    fn read_file<'a>(
        &'a self,
        path: &'a PathUri,
        sandbox: Option<&'a FileSystemSandboxContext>,
    ) -> ExecutorFileSystemFuture<'a, Vec<u8>> {
        Box::pin(UnsandboxedFileSystem::read_file(self, path, sandbox))
    }

    fn read_file_stream<'a>(
        &'a self,
        path: &'a PathUri,
        sandbox: Option<&'a FileSystemSandboxContext>,
    ) -> ExecutorFileSystemFuture<'a, FileSystemReadStream> {
        Box::pin(UnsandboxedFileSystem::read_file_stream(self, path, sandbox))
    }

    fn write_file<'a>(
        &'a self,
        path: &'a PathUri,
        contents: Vec<u8>,
        sandbox: Option<&'a FileSystemSandboxContext>,
    ) -> ExecutorFileSystemFuture<'a, ()> {
        Box::pin(UnsandboxedFileSystem::write_file(
            self, path, contents, sandbox,
        ))
    }

    fn create_directory<'a>(
        &'a self,
        path: &'a PathUri,
        options: CreateDirectoryOptions,
        sandbox: Option<&'a FileSystemSandboxContext>,
    ) -> ExecutorFileSystemFuture<'a, ()> {
        Box::pin(UnsandboxedFileSystem::create_directory(
            self, path, options, sandbox,
        ))
    }

    fn get_metadata<'a>(
        &'a self,
        path: &'a PathUri,
        sandbox: Option<&'a FileSystemSandboxContext>,
    ) -> ExecutorFileSystemFuture<'a, FileMetadata> {
        Box::pin(UnsandboxedFileSystem::get_metadata(self, path, sandbox))
    }

    fn read_directory<'a>(
        &'a self,
        path: &'a PathUri,
        sandbox: Option<&'a FileSystemSandboxContext>,
    ) -> ExecutorFileSystemFuture<'a, Vec<ReadDirectoryEntry>> {
        Box::pin(UnsandboxedFileSystem::read_directory(self, path, sandbox))
    }

    fn remove<'a>(
        &'a self,
        path: &'a PathUri,
        options: RemoveOptions,
        sandbox: Option<&'a FileSystemSandboxContext>,
    ) -> ExecutorFileSystemFuture<'a, ()> {
        Box::pin(UnsandboxedFileSystem::remove(self, path, options, sandbox))
    }

    fn copy<'a>(
        &'a self,
        source_path: &'a PathUri,
        destination_path: &'a PathUri,
        options: CopyOptions,
        sandbox: Option<&'a FileSystemSandboxContext>,
    ) -> ExecutorFileSystemFuture<'a, ()> {
        Box::pin(UnsandboxedFileSystem::copy(
            self,
            source_path,
            destination_path,
            options,
            sandbox,
        ))
    }
}

impl DirectFileSystem {
    async fn open_file_for_read(
        &self,
        path: &PathUri,
        sandbox: Option<&FileSystemSandboxContext>,
    ) -> FileSystemResult<tokio::fs::File> {
        reject_sandbox_context(sandbox)?;
        let path = path.to_abs_path()?;
        regular_file::open(path.as_path()).await
    }

    async fn canonicalize(
        &self,
        path: &PathUri,
        sandbox: Option<&FileSystemSandboxContext>,
    ) -> FileSystemResult<PathUri> {
        reject_sandbox_context(sandbox)?;
        let path = path.to_abs_path()?;
        let canonicalized =
            AbsolutePathBuf::from_absolute_path(tokio::fs::canonicalize(path.as_path()).await?)?;
        Ok(PathUri::from_abs_path(&canonicalized))
    }

    async fn read_file(
        &self,
        path: &PathUri,
        sandbox: Option<&FileSystemSandboxContext>,
    ) -> FileSystemResult<Vec<u8>> {
        let file = self.open_file_for_read(path, sandbox).await?;
        let metadata = file.metadata().await?;
        if metadata.len() > MAX_READ_FILE_BYTES {
            return Err(file_too_large_error());
        }
        let mut bytes = Vec::with_capacity(metadata.len() as usize);
        file.take(MAX_READ_FILE_BYTES + 1)
            .read_to_end(&mut bytes)
            .await?;
        if bytes.len() as u64 > MAX_READ_FILE_BYTES {
            return Err(file_too_large_error());
        }
        Ok(bytes)
    }

    async fn read_file_stream(
        &self,
        path: &PathUri,
        sandbox: Option<&FileSystemSandboxContext>,
    ) -> FileSystemResult<FileSystemReadStream> {
        let file = self.open_file_for_read(path, sandbox).await?;
        Ok(FileSystemReadStream::new(ReaderStream::with_capacity(
            file,
            FILE_READ_CHUNK_SIZE,
        )))
    }

    async fn write_file(
        &self,
        path: &PathUri,
        contents: Vec<u8>,
        sandbox: Option<&FileSystemSandboxContext>,
    ) -> FileSystemResult<()> {
        reject_sandbox_context(sandbox)?;
        let path = path.to_abs_path()?;
        tokio::fs::write(path.as_path(), contents).await
    }

    async fn create_directory(
        &self,
        path: &PathUri,
        options: CreateDirectoryOptions,
        sandbox: Option<&FileSystemSandboxContext>,
    ) -> FileSystemResult<()> {
        reject_sandbox_context(sandbox)?;
        let path = path.to_abs_path()?;
        if options.recursive {
            tokio::fs::create_dir_all(path.as_path()).await?;
        } else {
            tokio::fs::create_dir(path.as_path()).await?;
        }
        Ok(())
    }

    async fn get_metadata(
        &self,
        path: &PathUri,
        sandbox: Option<&FileSystemSandboxContext>,
    ) -> FileSystemResult<FileMetadata> {
        reject_sandbox_context(sandbox)?;
        let path = path.to_abs_path()?;
        let symlink_metadata = tokio::fs::symlink_metadata(path.as_path()).await?;
        let is_symlink = symlink_metadata.is_symlink();
        let metadata = if is_symlink {
            tokio::fs::metadata(path.as_path()).await?
        } else {
            symlink_metadata
        };
        Ok(FileMetadata {
            is_directory: metadata.is_dir(),
            is_file: metadata.is_file(),
            is_symlink,
            size: metadata.len(),
            created_at_ms: metadata.created().ok().map_or(0, system_time_to_unix_ms),
            modified_at_ms: metadata.modified().ok().map_or(0, system_time_to_unix_ms),
        })
    }

    async fn read_directory(
        &self,
        path: &PathUri,
        sandbox: Option<&FileSystemSandboxContext>,
    ) -> FileSystemResult<Vec<ReadDirectoryEntry>> {
        reject_sandbox_context(sandbox)?;
        let path = path.to_abs_path()?;
        let mut entries = Vec::new();
        let mut read_dir = tokio::fs::read_dir(path.as_path()).await?;
        while let Some(entry) = read_dir.next_entry().await? {
            let Ok(mut file_type) = entry.file_type().await else {
                continue;
            };
            if file_type.is_symlink() {
                let Ok(metadata) = tokio::fs::metadata(entry.path()).await else {
                    continue;
                };
                file_type = metadata.file_type();
            }
            entries.push(ReadDirectoryEntry {
                file_name: entry.file_name().to_string_lossy().into_owned(),
                is_directory: file_type.is_dir(),
                is_file: file_type.is_file(),
            });
        }
        Ok(entries)
    }

    async fn remove(
        &self,
        path: &PathUri,
        options: RemoveOptions,
        sandbox: Option<&FileSystemSandboxContext>,
    ) -> FileSystemResult<()> {
        reject_sandbox_context(sandbox)?;
        let path = path.to_abs_path()?;
        match tokio::fs::symlink_metadata(path.as_path()).await {
            Ok(metadata) => {
                let file_type = metadata.file_type();
                if file_type.is_dir() {
                    if options.recursive {
                        tokio::fs::remove_dir_all(path.as_path()).await?;
                    } else {
                        tokio::fs::remove_dir(path.as_path()).await?;
                    }
                } else {
                    tokio::fs::remove_file(path.as_path()).await?;
                }
                Ok(())
            }
            Err(err) if err.kind() == io::ErrorKind::NotFound && options.force => Ok(()),
            Err(err) => Err(err),
        }
    }

    async fn copy(
        &self,
        source_path: &PathUri,
        destination_path: &PathUri,
        options: CopyOptions,
        sandbox: Option<&FileSystemSandboxContext>,
    ) -> FileSystemResult<()> {
        reject_sandbox_context(sandbox)?;
        let source_path = source_path.to_abs_path()?.into_path_buf();
        let destination_path = destination_path.to_abs_path()?.into_path_buf();
        tokio::task::spawn_blocking(move || -> FileSystemResult<()> {
            let metadata = std::fs::symlink_metadata(source_path.as_path())?;
            let file_type = metadata.file_type();

            if file_type.is_dir() {
                if !options.recursive {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "fs/copy requires recursive: true when sourcePath is a directory",
                    ));
                }
                if destination_is_same_or_descendant_of_source(
                    source_path.as_path(),
                    destination_path.as_path(),
                )? {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "fs/copy cannot copy a directory to itself or one of its descendants",
                    ));
                }
                copy_dir_recursive(source_path.as_path(), destination_path.as_path())?;
                return Ok(());
            }

            if file_type.is_symlink() {
                copy_symlink(source_path.as_path(), destination_path.as_path())?;
                return Ok(());
            }

            if file_type.is_file() {
                std::fs::copy(source_path.as_path(), destination_path.as_path())?;
                return Ok(());
            }

            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "fs/copy only supports regular files, directories, and symlinks",
            ))
        })
        .await
        .map_err(|err| io::Error::other(format!("filesystem task failed: {err}")))?
    }
}

impl ExecutorFileSystem for DirectFileSystem {
    fn canonicalize<'a>(
        &'a self,
        path: &'a PathUri,
        sandbox: Option<&'a FileSystemSandboxContext>,
    ) -> ExecutorFileSystemFuture<'a, PathUri> {
        Box::pin(DirectFileSystem::canonicalize(self, path, sandbox))
    }

    fn read_file<'a>(
        &'a self,
        path: &'a PathUri,
        sandbox: Option<&'a FileSystemSandboxContext>,
    ) -> ExecutorFileSystemFuture<'a, Vec<u8>> {
        Box::pin(DirectFileSystem::read_file(self, path, sandbox))
    }

    fn read_file_stream<'a>(
        &'a self,
        path: &'a PathUri,
        sandbox: Option<&'a FileSystemSandboxContext>,
    ) -> ExecutorFileSystemFuture<'a, FileSystemReadStream> {
        Box::pin(DirectFileSystem::read_file_stream(self, path, sandbox))
    }

    fn write_file<'a>(
        &'a self,
        path: &'a PathUri,
        contents: Vec<u8>,
        sandbox: Option<&'a FileSystemSandboxContext>,
    ) -> ExecutorFileSystemFuture<'a, ()> {
        Box::pin(DirectFileSystem::write_file(self, path, contents, sandbox))
    }

    fn create_directory<'a>(
        &'a self,
        path: &'a PathUri,
        options: CreateDirectoryOptions,
        sandbox: Option<&'a FileSystemSandboxContext>,
    ) -> ExecutorFileSystemFuture<'a, ()> {
        Box::pin(DirectFileSystem::create_directory(
            self, path, options, sandbox,
        ))
    }

    fn get_metadata<'a>(
        &'a self,
        path: &'a PathUri,
        sandbox: Option<&'a FileSystemSandboxContext>,
    ) -> ExecutorFileSystemFuture<'a, FileMetadata> {
        Box::pin(DirectFileSystem::get_metadata(self, path, sandbox))
    }

    fn read_directory<'a>(
        &'a self,
        path: &'a PathUri,
        sandbox: Option<&'a FileSystemSandboxContext>,
    ) -> ExecutorFileSystemFuture<'a, Vec<ReadDirectoryEntry>> {
        Box::pin(DirectFileSystem::read_directory(self, path, sandbox))
    }

    fn remove<'a>(
        &'a self,
        path: &'a PathUri,
        options: RemoveOptions,
        sandbox: Option<&'a FileSystemSandboxContext>,
    ) -> ExecutorFileSystemFuture<'a, ()> {
        Box::pin(DirectFileSystem::remove(self, path, options, sandbox))
    }

    fn copy<'a>(
        &'a self,
        source_path: &'a PathUri,
        destination_path: &'a PathUri,
        options: CopyOptions,
        sandbox: Option<&'a FileSystemSandboxContext>,
    ) -> ExecutorFileSystemFuture<'a, ()> {
        Box::pin(DirectFileSystem::copy(
            self,
            source_path,
            destination_path,
            options,
            sandbox,
        ))
    }
}

fn reject_sandbox_context(sandbox: Option<&FileSystemSandboxContext>) -> io::Result<()> {
    if sandbox.is_some() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "direct filesystem operations do not accept sandbox context",
        ));
    }
    Ok(())
}

fn reject_platform_sandbox_context(sandbox: Option<&FileSystemSandboxContext>) -> io::Result<()> {
    if sandbox.is_some_and(FileSystemSandboxContext::should_run_in_sandbox) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "sandboxed filesystem operations require configured runtime paths",
        ));
    }
    Ok(())
}

#[cfg(target_os = "macos")]
fn authorized_read_limit(max_bytes: usize) -> io::Result<u64> {
    let read_limit = max_bytes
        .checked_add(1)
        .and_then(|limit| u64::try_from(limit).ok())
        .filter(|_| {
            max_bytes > 0 && u64::try_from(max_bytes).is_ok_and(|max| max <= MAX_READ_FILE_BYTES)
        })
        .ok_or_else(|| authorized_read_error(io::ErrorKind::InvalidInput))?;
    Ok(read_limit)
}

#[cfg(target_os = "macos")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct UniqueFileIdentity {
    device: u64,
    inode: u64,
}

#[cfg(target_os = "macos")]
async fn unique_file_identity(file: &tokio::fs::File) -> io::Result<UniqueFileIdentity> {
    use std::os::unix::fs::MetadataExt;

    let metadata = file.metadata().await.map_err(redact_file_access_error)?;
    if !metadata.is_file() || metadata.nlink() != 1 {
        return Err(authorized_read_error(io::ErrorKind::PermissionDenied));
    }
    Ok(UniqueFileIdentity {
        device: metadata.dev(),
        inode: metadata.ino(),
    })
}

#[cfg(target_os = "macos")]
fn unique_std_file_identity(file: &std::fs::File) -> io::Result<UniqueFileIdentity> {
    use std::os::unix::fs::MetadataExt;

    let metadata = file.metadata().map_err(redact_file_access_error)?;
    if !metadata.is_file() || metadata.nlink() != 1 {
        return Err(authorized_read_error(io::ErrorKind::PermissionDenied));
    }
    Ok(UniqueFileIdentity {
        device: metadata.dev(),
        inode: metadata.ino(),
    })
}

#[cfg(target_os = "macos")]
async fn secure_reopen_final_path(path: &Path) -> io::Result<tokio::fs::File> {
    let path = path.to_path_buf();
    let file = tokio::task::spawn_blocking(move || secure_open_unique_without_symlinks(&path))
        .await
        .map_err(|_| authorized_read_error(io::ErrorKind::PermissionDenied))?
        .map_err(redact_file_access_error)?;
    Ok(tokio::fs::File::from_std(file))
}

#[cfg(target_os = "macos")]
async fn secure_reopen_matching_identity(
    path: &Path,
    expected_identity: UniqueFileIdentity,
) -> io::Result<tokio::fs::File> {
    let file = secure_reopen_final_path(path).await?;
    if unique_file_identity(&file).await? != expected_identity
        || stable_file_path(&file)?.as_path() != path
    {
        return Err(authorized_read_error(io::ErrorKind::PermissionDenied));
    }
    Ok(file)
}

#[cfg(target_os = "macos")]
fn secure_open_unique_without_symlinks(path: &Path) -> io::Result<std::fs::File> {
    use std::os::unix::fs::OpenOptionsExt;

    // O_UNIQUE is an open(2)-only Darwin flag that makes path lookup fail when
    // the resolved vnode has more than one hard link. libc does not currently
    // expose the constant, so keep it aligned with <sys/fcntl.h>.
    const O_UNIQUE: libc::c_int = 0x0000_2000;

    let mut options = std::fs::OpenOptions::new();
    options
        .read(true)
        .custom_flags(libc::O_NONBLOCK | libc::O_CLOEXEC | libc::O_NOFOLLOW_ANY | O_UNIQUE);
    options.open(path)
}

#[cfg(target_os = "macos")]
static STABLE_HANDLE_AUTHORIZED_READ_AVAILABLE: LazyLock<bool> = LazyLock::new(|| {
    use std::os::unix::fs::DirBuilderExt;
    use std::os::unix::fs::symlink;

    let probe_dir = std::env::temp_dir().join(format!(
        "codex-authorized-read-probe-{}-{}",
        std::process::id(),
        uuid::Uuid::new_v4()
    ));
    let mut builder = std::fs::DirBuilder::new();
    builder.mode(0o700);
    if builder.create(&probe_dir).is_err() {
        return false;
    }

    let result = (|| -> io::Result<bool> {
        let probe_dir = std::fs::canonicalize(&probe_dir)?;
        let original_path = probe_dir.join("original");
        let hardlink_path = probe_dir.join("hardlink");
        let symlink_parent = probe_dir.join("symlink-parent");
        std::fs::write(&original_path, b"probe")?;
        std::fs::hard_link(&original_path, &hardlink_path)?;

        // A kernel that does not implement O_UNIQUE may silently accept the
        // flag. Require a multi-link vnode to be rejected before advertising.
        let hardlink_was_rejected = secure_open_unique_without_symlinks(&original_path).is_err();
        std::fs::remove_file(&hardlink_path)?;

        let reopened = secure_open_unique_without_symlinks(&original_path)?;
        let reopened_identity = unique_std_file_identity(&reopened)?;
        let reopened_path = stable_file_path_from_fd(&reopened)?;

        symlink(&probe_dir, &symlink_parent)?;
        let symlink_was_rejected =
            secure_open_unique_without_symlinks(&symlink_parent.join("original")).is_err();

        Ok(hardlink_was_rejected
            && symlink_was_rejected
            && reopened_identity
                == unique_std_file_identity(&std::fs::File::open(&original_path)?)?
            && reopened_path.as_path() == original_path)
    })()
    .unwrap_or(false);

    let _ = std::fs::remove_dir_all(&probe_dir);
    result
});

#[cfg(target_os = "macos")]
pub(crate) fn stable_handle_authorized_read_available() -> bool {
    *STABLE_HANDLE_AUTHORIZED_READ_AVAILABLE
}

#[cfg(not(target_os = "macos"))]
pub(crate) fn stable_handle_authorized_read_available() -> bool {
    false
}

#[cfg(target_os = "macos")]
fn authorize_stable_file_path(
    final_path: &Path,
    sandbox: &FileSystemSandboxContext,
) -> io::Result<()> {
    let cwd = match sandbox.cwd.as_ref() {
        Some(cwd) => cwd
            .to_abs_path()
            .map_err(|_| authorized_read_error(io::ErrorKind::InvalidInput))?,
        None if sandbox.has_cwd_dependent_permissions() => {
            return Err(authorized_read_error(io::ErrorKind::InvalidInput));
        }
        None => AbsolutePathBuf::from_absolute_path(
            current_sandbox_cwd()
                .map_err(|_| authorized_read_error(io::ErrorKind::InvalidInput))?,
        )
        .map_err(|_| authorized_read_error(io::ErrorKind::InvalidInput))?,
    };
    let workspace_roots = sandbox
        .workspace_roots
        .iter()
        .map(|root| {
            root.to_abs_path()
                .map_err(|_| authorized_read_error(io::ErrorKind::InvalidInput))
        })
        .collect::<io::Result<Vec<_>>>()?;
    let permissions: PermissionProfile = sandbox
        .permissions
        .clone()
        .try_into()
        .map_err(|_| authorized_read_error(io::ErrorKind::InvalidInput))?;
    let policy = permissions
        .materialize_project_roots_with_workspace_roots(&workspace_roots)
        .file_system_sandbox_policy();
    let denied_by_pattern = ReadDenyMatcher::new(&policy, cwd.as_path())
        .is_some_and(|matcher| matcher.is_read_denied(final_path));
    if denied_by_pattern || !policy.can_read_path_with_cwd(final_path, cwd.as_path()) {
        return Err(authorized_read_error(io::ErrorKind::PermissionDenied));
    }
    Ok(())
}

#[cfg(target_os = "macos")]
fn stable_file_path(file: &tokio::fs::File) -> io::Result<AbsolutePathBuf> {
    use std::os::fd::AsRawFd;

    stable_file_path_from_raw_fd(file.as_raw_fd())
}

#[cfg(target_os = "macos")]
fn stable_file_path_from_fd(file: &std::fs::File) -> io::Result<AbsolutePathBuf> {
    use std::os::fd::AsRawFd;

    stable_file_path_from_raw_fd(file.as_raw_fd())
}

#[cfg(target_os = "macos")]
fn stable_file_path_from_raw_fd(fd: std::os::fd::RawFd) -> io::Result<AbsolutePathBuf> {
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;

    let mut buffer = [0_u8; libc::PATH_MAX as usize];
    // SAFETY: the caller owns a live fd and `buffer` is writable for PATH_MAX bytes.
    if unsafe { libc::fcntl(fd, libc::F_GETPATH, buffer.as_mut_ptr()) } == -1 {
        return Err(redact_file_access_error(io::Error::last_os_error()));
    }
    let length = buffer
        .iter()
        .position(|byte| *byte == 0)
        .ok_or_else(|| authorized_read_error(io::ErrorKind::PermissionDenied))?;
    let path = PathBuf::from(OsStr::from_bytes(&buffer[..length]));
    AbsolutePathBuf::from_absolute_path(path)
        .map_err(|_| authorized_read_error(io::ErrorKind::PermissionDenied))
}

#[cfg(target_os = "macos")]
fn redact_file_access_error(error: io::Error) -> io::Error {
    if error.kind() == io::ErrorKind::NotFound {
        authorized_read_error(io::ErrorKind::NotFound)
    } else {
        authorized_read_error(io::ErrorKind::PermissionDenied)
    }
}

#[cfg(target_os = "macos")]
fn authorized_read_error(kind: io::ErrorKind) -> io::Error {
    let message = match kind {
        io::ErrorKind::NotFound => "authorized file read target was not found",
        io::ErrorKind::PermissionDenied => "authorized file read denied",
        io::ErrorKind::InvalidInput => "authorized file read request is invalid",
        _ => "authorized file read failed",
    };
    io::Error::new(kind, message)
}

fn copy_dir_recursive(source: &Path, target: &Path) -> io::Result<()> {
    std::fs::create_dir_all(target)?;
    for entry in std::fs::read_dir(source)? {
        let entry = entry?;
        let source_path = entry.path();
        let target_path = target.join(entry.file_name());
        let file_type = entry.file_type()?;

        if file_type.is_dir() {
            copy_dir_recursive(&source_path, &target_path)?;
        } else if file_type.is_file() {
            std::fs::copy(&source_path, &target_path)?;
        } else if file_type.is_symlink() {
            copy_symlink(&source_path, &target_path)?;
        }
    }
    Ok(())
}

fn destination_is_same_or_descendant_of_source(
    source: &Path,
    destination: &Path,
) -> io::Result<bool> {
    let source = std::fs::canonicalize(source)?;
    let destination = resolve_existing_path(destination)?;
    Ok(destination.starts_with(&source))
}

pub(crate) fn resolve_existing_path(path: &Path) -> io::Result<PathBuf> {
    let mut unresolved_suffix = Vec::new();
    let mut existing_path = path;
    while !existing_path.exists() {
        let Some(file_name) = existing_path.file_name() else {
            break;
        };
        unresolved_suffix.push(file_name.to_os_string());
        let Some(parent) = existing_path.parent() else {
            break;
        };
        existing_path = parent;
    }

    let mut resolved = std::fs::canonicalize(existing_path)?;
    for file_name in unresolved_suffix.iter().rev() {
        resolved.push(file_name);
    }
    Ok(resolved)
}

pub(crate) fn current_sandbox_cwd() -> io::Result<PathBuf> {
    let cwd = std::env::current_dir()
        .map_err(|err| io::Error::other(format!("failed to read current dir: {err}")))?;
    resolve_existing_path(cwd.as_path())
}

fn copy_symlink(source: &Path, target: &Path) -> io::Result<()> {
    let link_target = std::fs::read_link(source)?;
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(&link_target, target)
    }
    #[cfg(windows)]
    {
        if symlink_points_to_directory(source)? {
            std::os::windows::fs::symlink_dir(&link_target, target)
        } else {
            std::os::windows::fs::symlink_file(&link_target, target)
        }
    }
    #[cfg(not(any(unix, windows)))]
    {
        let _ = link_target;
        let _ = target;
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "copying symlinks is unsupported on this platform",
        ))
    }
}

#[cfg(windows)]
fn symlink_points_to_directory(source: &Path) -> io::Result<bool> {
    use std::os::windows::fs::FileTypeExt;

    Ok(std::fs::symlink_metadata(source)?
        .file_type()
        .is_symlink_dir())
}

fn system_time_to_unix_ms(time: SystemTime) -> i64 {
    time.duration_since(UNIX_EPOCH)
        .ok()
        .and_then(|duration| i64::try_from(duration.as_millis()).ok())
        .unwrap_or(0)
}

#[cfg(all(test, any(unix, windows)))]
#[path = "local_file_system_path_uri_tests.rs"]
mod path_uri_tests;

#[cfg(all(test, target_os = "macos"))]
#[path = "local_file_system_authorized_read_tests.rs"]
mod authorized_read_tests;

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::os::unix::fs::symlink;

    #[test]
    fn resolve_existing_path_handles_symlink_parent_dotdot_escape() -> io::Result<()> {
        let temp_dir = tempfile::TempDir::new()?;
        let allowed_dir = temp_dir.path().join("allowed");
        let outside_dir = temp_dir.path().join("outside");
        std::fs::create_dir_all(&allowed_dir)?;
        std::fs::create_dir_all(&outside_dir)?;
        symlink(&outside_dir, allowed_dir.join("link"))?;

        let resolved = resolve_existing_path(
            allowed_dir
                .join("link")
                .join("..")
                .join("secret.txt")
                .as_path(),
        )?;

        assert_eq!(
            resolved,
            resolve_existing_path(temp_dir.path())?.join("secret.txt")
        );
        Ok(())
    }
}

#[cfg(all(test, windows))]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn symlink_points_to_directory_handles_dangling_directory_symlinks() -> io::Result<()> {
        use std::os::windows::fs::symlink_dir;

        let temp_dir = tempfile::TempDir::new()?;
        let source_dir = temp_dir.path().join("source");
        let link_path = temp_dir.path().join("source-link");
        std::fs::create_dir(&source_dir)?;

        if symlink_dir(&source_dir, &link_path).is_err() {
            return Ok(());
        }

        std::fs::remove_dir(&source_dir)?;

        assert_eq!(symlink_points_to_directory(&link_path)?, true);
        Ok(())
    }
}
