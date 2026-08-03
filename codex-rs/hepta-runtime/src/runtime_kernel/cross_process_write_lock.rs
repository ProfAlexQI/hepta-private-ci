#[cfg(unix)]
use std::collections::BTreeMap;
use std::ffi::OsString;
#[cfg(unix)]
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use hepta_core::HeptaError;
#[cfg(unix)]
use sha2::Digest as _;
#[cfg(unix)]
use sha2::Sha256;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum CrossProcessLockMode {
    Shared,
    Exclusive,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct CrossProcessLockRequest {
    key: Vec<u8>,
    mode: CrossProcessLockMode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CrossProcessTargetIdentity {
    canonical_namespace: PathBuf,
    anchor_device: u64,
    anchor_inode: u64,
    anchor_suffix: Vec<OsString>,
    existing_target: Option<(u64, u64)>,
}

impl CrossProcessTargetIdentity {
    pub(crate) fn new(
        canonical_namespace: PathBuf,
        anchor_device: u64,
        anchor_inode: u64,
        anchor_suffix: Vec<OsString>,
        existing_target: Option<(u64, u64)>,
    ) -> Self {
        Self {
            canonical_namespace,
            anchor_device,
            anchor_inode,
            anchor_suffix,
            existing_target,
        }
    }
}

impl CrossProcessLockRequest {
    pub(super) fn shared(key: impl Into<Vec<u8>>) -> Self {
        Self {
            key: key.into(),
            mode: CrossProcessLockMode::Shared,
        }
    }

    pub(super) fn exclusive(key: impl Into<Vec<u8>>) -> Self {
        Self {
            key: key.into(),
            mode: CrossProcessLockMode::Exclusive,
        }
    }
}

/// An OS-owned advisory-lock witness.
///
/// The files deliberately remain open for the full authorized mutation
/// lifetime. Dropping this non-cloneable value is the only release path.
#[derive(Debug)]
pub(crate) struct CrossProcessWriteLease {
    #[cfg(unix)]
    _files: Vec<fs::File>,
}

pub(super) fn acquire_cross_process_write_lease(
    workspace_root: &Path,
    requests: impl IntoIterator<Item = CrossProcessLockRequest>,
) -> Result<CrossProcessWriteLease, HeptaError> {
    acquire_cross_process_write_lease_impl(workspace_root, requests)
}

pub(crate) fn acquire_cross_process_target_lease(
    workspace_root: &Path,
    identities: &[CrossProcessTargetIdentity],
) -> Result<CrossProcessWriteLease, HeptaError> {
    let requests = identities
        .iter()
        .flat_map(cross_process_target_lock_requests)
        .collect::<Vec<_>>();
    acquire_cross_process_write_lease(workspace_root, requests)
}

#[cfg(unix)]
fn cross_process_target_lock_requests(
    identity: &CrossProcessTargetIdentity,
) -> Vec<CrossProcessLockRequest> {
    use std::os::unix::ffi::OsStrExt as _;

    let namespace_components = identity
        .canonical_namespace
        .components()
        .map(|component| component.as_os_str().as_bytes().to_vec())
        .collect::<Vec<_>>();
    let anchor_components = identity
        .anchor_suffix
        .iter()
        .map(|component| component.as_os_str().as_bytes().to_vec())
        .collect::<Vec<_>>();
    let mut requests = prefix_lock_requests(
        b"hepta.runtime.cross-process-namespace.v1\0",
        &[],
        &namespace_components,
    );
    let mut anchor_prefix = Vec::with_capacity(16);
    anchor_prefix.extend_from_slice(&identity.anchor_device.to_le_bytes());
    anchor_prefix.extend_from_slice(&identity.anchor_inode.to_le_bytes());
    requests.extend(prefix_lock_requests(
        b"hepta.runtime.cross-process-anchor.v1\0",
        &anchor_prefix,
        &anchor_components,
    ));
    if let Some((device, inode)) = identity.existing_target {
        let mut key = Vec::with_capacity(64);
        key.extend_from_slice(b"hepta.runtime.cross-process-inode.v1\0");
        key.extend_from_slice(&device.to_le_bytes());
        key.extend_from_slice(&inode.to_le_bytes());
        requests.push(CrossProcessLockRequest::exclusive(key));
    }
    requests
}

#[cfg(not(unix))]
fn cross_process_target_lock_requests(
    _identity: &CrossProcessTargetIdentity,
) -> Vec<CrossProcessLockRequest> {
    Vec::new()
}

#[cfg(unix)]
fn prefix_lock_requests(
    domain: &[u8],
    stable_prefix: &[u8],
    components: &[Vec<u8>],
) -> Vec<CrossProcessLockRequest> {
    let mut prefix = Vec::new();
    prefix.extend_from_slice(domain);
    prefix.extend_from_slice(&(stable_prefix.len() as u64).to_le_bytes());
    prefix.extend_from_slice(stable_prefix);
    components
        .iter()
        .enumerate()
        .map(|(index, component)| {
            prefix.extend_from_slice(&(component.len() as u64).to_le_bytes());
            prefix.extend_from_slice(component);
            if index + 1 == components.len() {
                CrossProcessLockRequest::exclusive(prefix.clone())
            } else {
                CrossProcessLockRequest::shared(prefix.clone())
            }
        })
        .collect()
}

#[cfg(unix)]
fn acquire_cross_process_write_lease_impl(
    workspace_root: &Path,
    requests: impl IntoIterator<Item = CrossProcessLockRequest>,
) -> Result<CrossProcessWriteLease, HeptaError> {
    use std::os::fd::AsRawFd as _;
    use std::os::unix::fs::MetadataExt as _;
    use std::os::unix::fs::OpenOptionsExt as _;

    let lock_root = secure_lock_root(workspace_root)?;
    let mut unique = BTreeMap::<String, CrossProcessLockMode>::new();
    for request in requests {
        let filename = lock_filename(&request.key);
        unique
            .entry(filename)
            .and_modify(|mode| {
                if request.mode == CrossProcessLockMode::Exclusive {
                    *mode = CrossProcessLockMode::Exclusive;
                }
            })
            .or_insert(request.mode);
    }
    if unique.is_empty() {
        return Err(HeptaError(
            "cross-process mutation reservation requires at least one identity".into(),
        ));
    }

    let expected_uid = unsafe { libc::geteuid() };
    let mut files = Vec::with_capacity(unique.len());
    for (filename, mode) in unique {
        let lock_path = lock_root.join(filename);
        let mut options = fs::OpenOptions::new();
        options
            .read(true)
            .write(true)
            .create(true)
            .mode(0o600)
            .custom_flags(libc::O_CLOEXEC | libc::O_NOFOLLOW);
        let file = options.open(&lock_path).map_err(|error| {
            HeptaError(format!(
                "cross-process mutation lock open failed for {}: {error}",
                lock_path.display()
            ))
        })?;
        let metadata = file.metadata().map_err(|error| {
            HeptaError(format!(
                "cross-process mutation lock metadata failed for {}: {error}",
                lock_path.display()
            ))
        })?;
        if !metadata.file_type().is_file()
            || metadata.uid() != expected_uid
            || metadata.nlink() != 1
            || metadata.mode() & 0o077 != 0
        {
            return Err(HeptaError(format!(
                "cross-process mutation lock has unsafe identity or permissions: {}",
                lock_path.display()
            )));
        }
        let operation = match mode {
            CrossProcessLockMode::Shared => libc::LOCK_SH,
            CrossProcessLockMode::Exclusive => libc::LOCK_EX,
        } | libc::LOCK_NB;
        if unsafe { libc::flock(file.as_raw_fd(), operation) } != 0 {
            let error = std::io::Error::last_os_error();
            return Err(HeptaError(format!(
                "cross-process mutation reservation conflict for {}: {error}",
                lock_path.display()
            )));
        }
        files.push(file);
    }
    Ok(CrossProcessWriteLease { _files: files })
}

#[cfg(not(unix))]
fn acquire_cross_process_write_lease_impl(
    _workspace_root: &Path,
    _requests: impl IntoIterator<Item = CrossProcessLockRequest>,
) -> Result<CrossProcessWriteLease, HeptaError> {
    Err(HeptaError(
        "live filesystem mutation requires supported cross-process advisory locks".into(),
    ))
}

#[cfg(unix)]
fn secure_lock_root(workspace_root: &Path) -> Result<PathBuf, HeptaError> {
    use std::os::unix::fs::DirBuilderExt as _;
    use std::os::unix::fs::MetadataExt as _;

    let canonical_workspace = fs::canonicalize(workspace_root).map_err(|error| {
        HeptaError(format!(
            "cross-process mutation workspace canonicalization failed for {}: {error}",
            workspace_root.display()
        ))
    })?;
    let workspace_metadata = fs::symlink_metadata(&canonical_workspace).map_err(|error| {
        HeptaError(format!(
            "cross-process mutation workspace metadata failed for {}: {error}",
            canonical_workspace.display()
        ))
    })?;
    if !workspace_metadata.file_type().is_dir() {
        return Err(HeptaError(format!(
            "cross-process mutation workspace is not a directory: {}",
            canonical_workspace.display()
        )));
    }

    let expected_uid = unsafe { libc::geteuid() };
    let mut hasher = Sha256::new();
    hasher.update(b"hepta.runtime.cross-process-workspace.v1\0");
    hasher.update(workspace_metadata.dev().to_le_bytes());
    hasher.update(workspace_metadata.ino().to_le_bytes());
    let workspace_key = format!("{:x}", hasher.finalize());
    let lock_root = PathBuf::from("/tmp").join(format!(
        "hepta-v2-write-locks-{}-{}",
        expected_uid, workspace_key
    ));

    match fs::symlink_metadata(&lock_root) {
        Ok(_) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            let mut builder = fs::DirBuilder::new();
            builder.mode(0o700);
            if let Err(create_error) = builder.create(&lock_root)
                && create_error.kind() != std::io::ErrorKind::AlreadyExists
            {
                return Err(HeptaError(format!(
                    "cross-process mutation lock directory creation failed for {}: {create_error}",
                    lock_root.display()
                )));
            }
        }
        Err(error) => {
            return Err(HeptaError(format!(
                "cross-process mutation lock directory metadata failed for {}: {error}",
                lock_root.display()
            )));
        }
    }
    let metadata = fs::symlink_metadata(&lock_root).map_err(|error| {
        HeptaError(format!(
            "cross-process mutation lock directory metadata failed for {}: {error}",
            lock_root.display()
        ))
    })?;
    if !metadata.file_type().is_dir()
        || metadata.uid() != expected_uid
        || metadata.mode() & 0o077 != 0
    {
        return Err(HeptaError(format!(
            "cross-process mutation lock directory has unsafe identity or permissions: {}",
            lock_root.display()
        )));
    }
    Ok(lock_root)
}

#[cfg(unix)]
fn lock_filename(key: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"hepta.runtime.cross-process-lock.v1\0");
    hasher.update((key.len() as u64).to_le_bytes());
    hasher.update(key);
    format!("{:x}.lock", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(unix)]
    #[test]
    fn exact_exclusive_identity_conflicts_and_releases() {
        let workspace = crate::tool_workspace_root_path();
        let key = format!("test-exclusive-{}", uuid::Uuid::new_v4()).into_bytes();
        let first = acquire_cross_process_write_lease(
            &workspace,
            [CrossProcessLockRequest::exclusive(key.clone())],
        )
        .expect("first exact identity");
        let conflict = acquire_cross_process_write_lease(
            &workspace,
            [CrossProcessLockRequest::exclusive(key.clone())],
        )
        .expect_err("second exact identity must conflict");
        assert!(
            conflict.0.contains("reservation conflict"),
            "{}",
            conflict.0
        );
        drop(first);
        acquire_cross_process_write_lease(&workspace, [CrossProcessLockRequest::exclusive(key)])
            .expect("released identity");
    }

    #[cfg(unix)]
    #[test]
    fn shared_ancestors_allow_unrelated_exclusive_leaves() {
        let workspace = crate::tool_workspace_root_path();
        let suffix = uuid::Uuid::new_v4();
        let ancestor = format!("test-ancestor-{suffix}").into_bytes();
        let first_leaf = format!("test-leaf-a-{suffix}").into_bytes();
        let second_leaf = format!("test-leaf-b-{suffix}").into_bytes();
        let first = acquire_cross_process_write_lease(
            &workspace,
            [
                CrossProcessLockRequest::shared(ancestor.clone()),
                CrossProcessLockRequest::exclusive(first_leaf),
            ],
        )
        .expect("first leaf");
        let second = acquire_cross_process_write_lease(
            &workspace,
            [
                CrossProcessLockRequest::shared(ancestor),
                CrossProcessLockRequest::exclusive(second_leaf),
            ],
        )
        .expect("unrelated leaf");
        drop((first, second));
    }

    #[cfg(unix)]
    #[test]
    fn target_identity_prefixes_block_ancestor_and_exact_aliases() {
        let workspace = crate::tool_workspace_root_path();
        let suffix = uuid::Uuid::new_v4().to_string();
        let anchor = (73, 91);
        let parent = CrossProcessTargetIdentity::new(
            workspace.join("artifacts").join(&suffix),
            anchor.0,
            anchor.1,
            vec![OsString::from(&suffix)],
            None,
        );
        let child = CrossProcessTargetIdentity::new(
            workspace.join("artifacts").join(&suffix).join("child.txt"),
            anchor.0,
            anchor.1,
            vec![OsString::from(&suffix), OsString::from("child.txt")],
            None,
        );
        let parent_lease =
            acquire_cross_process_target_lease(&workspace, std::slice::from_ref(&parent))
                .expect("parent identity");
        acquire_cross_process_target_lease(&workspace, std::slice::from_ref(&parent))
            .expect_err("exact alias must conflict");
        acquire_cross_process_target_lease(&workspace, std::slice::from_ref(&child))
            .expect_err("descendant must conflict with exclusive parent");
        drop(parent_lease);
        acquire_cross_process_target_lease(&workspace, &[child]).expect("released descendant");
    }

    #[cfg(unix)]
    #[test]
    fn separate_process_holds_exact_identity_until_drop() {
        const CHILD_MODE: &str = "HEPTA_V2_CROSS_PROCESS_LOCK_CHILD";
        const KEY_ENV: &str = "HEPTA_V2_CROSS_PROCESS_LOCK_KEY";
        const READY_ENV: &str = "HEPTA_V2_CROSS_PROCESS_LOCK_READY";
        const RELEASE_ENV: &str = "HEPTA_V2_CROSS_PROCESS_LOCK_RELEASE";

        let workspace = crate::tool_workspace_root_path();
        if std::env::var_os(CHILD_MODE).is_some() {
            let key = std::env::var(KEY_ENV).expect("child key").into_bytes();
            let ready = PathBuf::from(std::env::var_os(READY_ENV).expect("child ready path"));
            let release = PathBuf::from(std::env::var_os(RELEASE_ENV).expect("child release path"));
            let _lease = acquire_cross_process_write_lease(
                &workspace,
                [CrossProcessLockRequest::exclusive(key)],
            )
            .expect("child exact lease");
            fs::write(&ready, b"ready").expect("child ready marker");
            for _ in 0..1_500 {
                if release.exists() {
                    return;
                }
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
            panic!("parent never released child lock");
        }

        let temp = tempfile::tempdir().expect("cross-process test directory");
        let ready = temp.path().join("ready");
        let release = temp.path().join("release");
        let key = format!("test-cross-process-{}", uuid::Uuid::new_v4());
        let test_name = "runtime_kernel::cross_process_write_lock::tests::separate_process_holds_exact_identity_until_drop";
        let mut child = std::process::Command::new(std::env::current_exe().expect("test binary"))
            .arg("--exact")
            .arg(test_name)
            .arg("--test-threads=1")
            .env(CHILD_MODE, "1")
            .env(KEY_ENV, &key)
            .env(READY_ENV, &ready)
            .env(RELEASE_ENV, &release)
            .spawn()
            .expect("spawn lock holder");

        for _ in 0..1_000 {
            if ready.exists() {
                break;
            }
            if child.try_wait().expect("child status").is_some() {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        let ready_observed = ready.exists();
        let contender = if ready_observed {
            acquire_cross_process_write_lease(
                &workspace,
                [CrossProcessLockRequest::exclusive(key.clone().into_bytes())],
            )
        } else {
            Err(HeptaError("child did not publish ready marker".into()))
        };
        fs::write(&release, b"release").expect("release child");
        let child_status = child.wait().expect("wait for lock holder");

        assert!(ready_observed, "child did not acquire the lock");
        let conflict = contender.expect_err("other process must hold exact identity");
        assert!(
            conflict.0.contains("reservation conflict"),
            "{}",
            conflict.0
        );
        assert!(child_status.success(), "{child_status}");
        acquire_cross_process_write_lease(
            &workspace,
            [CrossProcessLockRequest::exclusive(key.into_bytes())],
        )
        .expect("child drop releases OS lock");
    }
}
