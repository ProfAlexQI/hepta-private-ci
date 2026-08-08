use super::ExecutorFileSystem;
use super::FileSystemSandboxContext;
use super::LocalFileSystem;
use super::ensure_open_file_is_linked;
use codex_protocol::models::PermissionProfile;
use codex_protocol::permissions::FileSystemAccessMode;
use codex_protocol::permissions::FileSystemPath;
use codex_protocol::permissions::FileSystemSandboxEntry;
use codex_protocol::permissions::FileSystemSandboxPolicy;
use codex_protocol::permissions::NetworkSandboxPolicy;
use codex_utils_absolute_path::AbsolutePathBuf;
use codex_utils_path_uri::PathUri;
use std::io;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
const ALLOWED: &[u8] = b"ALLOWED";
const SECRET: &[u8] = b"SECRET";
fn read_sandbox(readable_root: &Path) -> io::Result<FileSystemSandboxContext> {
    let readable_root = AbsolutePathBuf::from_absolute_path(readable_root)?;
    let policy = FileSystemSandboxPolicy::restricted(vec![FileSystemSandboxEntry::new(
        FileSystemPath::Path {
            path: readable_root.clone(),
        },
        FileSystemAccessMode::Read,
    )]);
    let permissions =
        PermissionProfile::from_runtime_permissions(&policy, NetworkSandboxPolicy::Restricted);
    Ok(FileSystemSandboxContext::from_permission_profile_with_cwd(
        permissions,
        PathUri::from_abs_path(&readable_root),
    ))
}

async fn authorized_read(
    file_system: &LocalFileSystem,
    path: &Path,
    sandbox: &FileSystemSandboxContext,
    max_bytes: usize,
) -> io::Result<Vec<u8>> {
    ExecutorFileSystem::read_file_bounded_authorized(
        file_system,
        &PathUri::from_host_native_path(path)?,
        sandbox,
        max_bytes,
    )
    .await
}

#[tokio::test]
async fn stable_handle_read_authorizes_the_resolved_symlink_target() -> io::Result<()> {
    use std::os::unix::fs::symlink;
    let temp_dir = tempfile::tempdir()?;
    let allowed_dir = temp_dir.path().join("allowed");
    let denied_dir = temp_dir.path().join("denied");
    std::fs::create_dir_all(&allowed_dir)?;
    std::fs::create_dir_all(&denied_dir)?;
    let allowed_file = allowed_dir.join("allowed.txt");
    let secret_file = denied_dir.join("secret.txt");
    std::fs::write(&allowed_file, ALLOWED)?;
    std::fs::write(&secret_file, SECRET)?;
    let allowed_link = temp_dir.path().join("allowed-link");
    let denied_link = allowed_dir.join("denied-link");
    symlink(&allowed_file, &allowed_link)?;
    symlink(&secret_file, &denied_link)?;
    let file_system = LocalFileSystem::unsandboxed();
    let sandbox = read_sandbox(&allowed_dir)?;
    assert_eq!(
        authorized_read(&file_system, &allowed_link, &sandbox, ALLOWED.len()).await?,
        ALLOWED
    );
    let error = authorized_read(&file_system, &denied_link, &sandbox, SECRET.len())
        .await
        .expect_err("symlink escaping the readable root must be denied");
    assert_eq!(error.kind(), io::ErrorKind::PermissionDenied);
    let message = error.to_string();
    assert!(!message.contains("secret"));
    assert!(!message.contains(denied_dir.to_str().unwrap_or_default()));
    Ok(())
}

#[tokio::test]
async fn stable_handle_read_applies_deny_globs_to_the_resolved_target() -> io::Result<()> {
    let temp_dir = tempfile::tempdir()?;
    let allowed_dir = temp_dir.path().join("allowed");
    std::fs::create_dir_all(&allowed_dir)?;
    let denied_file = allowed_dir.join("blocked.secret");
    std::fs::write(&denied_file, SECRET)?;
    let mut sandbox = read_sandbox(&allowed_dir)?;
    let native_permissions: PermissionProfile = sandbox.permissions.clone().try_into()?;
    let mut policy = native_permissions.file_system_sandbox_policy();
    policy.entries.push(FileSystemSandboxEntry::new(
        FileSystemPath::GlobPattern {
            pattern: format!("{}/blocked.*", allowed_dir.display()),
        },
        FileSystemAccessMode::Deny,
    ));
    sandbox.permissions =
        PermissionProfile::from_runtime_permissions(&policy, NetworkSandboxPolicy::Restricted)
            .into();
    let error = authorized_read(
        &LocalFileSystem::unsandboxed(),
        &denied_file,
        &sandbox,
        SECRET.len(),
    )
    .await
    .expect_err("deny glob must override the readable root");
    assert_eq!(error.kind(), io::ErrorKind::PermissionDenied);
    assert!(!error.to_string().contains("blocked.secret"));
    Ok(())
}

#[tokio::test]
async fn stable_handle_read_rejects_max_plus_one_without_returning_a_prefix() -> io::Result<()> {
    let temp_dir = tempfile::tempdir()?;
    let exact_file = temp_dir.path().join("exact.txt");
    let oversized_file = temp_dir.path().join("oversized.txt");
    std::fs::write(&exact_file, b"12345678")?;
    std::fs::write(&oversized_file, b"123456789")?;
    let sandbox = read_sandbox(temp_dir.path())?;
    let file_system = LocalFileSystem::unsandboxed();
    assert_eq!(
        authorized_read(&file_system, &exact_file, &sandbox, 8).await?,
        b"12345678"
    );
    let error = authorized_read(&file_system, &oversized_file, &sandbox, 8)
        .await
        .expect_err("max plus one must fail without a partial result");
    assert_eq!(error.kind(), io::ErrorKind::InvalidData);
    assert_eq!(
        error.to_string(),
        "authorized file read exceeds the requested limit"
    );
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn symlink_swap_never_returns_denied_contents() -> io::Result<()> {
    use std::os::unix::fs::symlink;
    let temp_dir = tempfile::tempdir()?;
    let allowed_dir = temp_dir.path().join("allowed");
    let denied_dir = temp_dir.path().join("denied");
    std::fs::create_dir_all(&allowed_dir)?;
    std::fs::create_dir_all(&denied_dir)?;
    let allowed_file = allowed_dir.join("allowed.txt");
    let secret_file = denied_dir.join("secret.txt");
    std::fs::write(&allowed_file, ALLOWED)?;
    std::fs::write(&secret_file, SECRET)?;
    let link = temp_dir.path().join("swap-link");
    symlink(&allowed_file, &link)?;
    let stop = Arc::new(AtomicBool::new(false));
    let toggler_stop = Arc::clone(&stop);
    let toggler_link = link.clone();
    let toggler = std::thread::spawn(move || -> io::Result<()> {
        let replacement = toggler_link.with_extension("next");
        let mut use_secret = true;
        while !toggler_stop.load(Ordering::Relaxed) {
            let _ = std::fs::remove_file(&replacement);
            let target = if use_secret {
                &secret_file
            } else {
                &allowed_file
            };
            symlink(target, &replacement)?;
            std::fs::rename(&replacement, &toggler_link)?;
            use_secret = !use_secret;
        }
        Ok(())
    });
    let sandbox = read_sandbox(&allowed_dir)?;
    let file_system = LocalFileSystem::unsandboxed();
    let mut unexpected = None;
    for _ in 0..256 {
        match authorized_read(&file_system, &link, &sandbox, ALLOWED.len()).await {
            Ok(contents) if contents == ALLOWED => {}
            Ok(contents) => {
                unexpected = Some(format!("unexpected contents: {contents:?}"));
                break;
            }
            Err(error)
                if matches!(
                    error.kind(),
                    io::ErrorKind::PermissionDenied | io::ErrorKind::NotFound
                ) => {}
            Err(error) => {
                unexpected = Some(format!("unexpected error kind: {:?}", error.kind()));
                break;
            }
        }
        tokio::task::yield_now().await;
    }
    stop.store(true, Ordering::Relaxed);
    toggler
        .join()
        .expect("symlink swap thread must not panic")?;
    assert_eq!(unexpected, None);
    Ok(())
}

#[tokio::test]
async fn authorized_read_errors_do_not_expose_requested_paths() -> io::Result<()> {
    let temp_dir = tempfile::tempdir()?;
    let missing = temp_dir.path().join("SECRET-missing-target.txt");
    let sandbox = read_sandbox(temp_dir.path())?;
    let error = authorized_read(&LocalFileSystem::unsandboxed(), &missing, &sandbox, 64)
        .await
        .expect_err("missing file must fail");
    assert_eq!(error.kind(), io::ErrorKind::NotFound);
    let message = error.to_string();
    assert_eq!(message, "authorized file read target was not found");
    assert!(!message.contains("SECRET"));
    assert!(!message.contains(temp_dir.path().to_str().unwrap_or_default()));
    Ok(())
}

#[tokio::test]
async fn an_unlinked_open_handle_fails_closed() -> io::Result<()> {
    let temp_dir = tempfile::tempdir()?;
    let path = temp_dir.path().join("unlinked.txt");
    std::fs::write(&path, ALLOWED)?;
    let file = crate::regular_file::open(&path).await?;
    std::fs::remove_file(&path)?;
    let error = ensure_open_file_is_linked(&file)
        .await
        .expect_err("an unlinked handle must fail closed regardless of path API behavior");
    assert_eq!(error.kind(), io::ErrorKind::NotFound);
    Ok(())
}
