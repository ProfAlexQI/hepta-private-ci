use super::*;

use crate::DurablePreferenceStore;

#[cfg(unix)]
use std::os::unix::fs::MetadataExt;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

#[cfg(unix)]
#[tokio::test]
async fn durable_bootstrap_creates_private_parent_database_and_sidecars() -> TestResult {
    let directory = private_tempdir()?;
    let private_parent = directory.path().join("nested").join("durable");
    let database_path = private_parent.join("v2-memory.sqlite3");
    let _store = DurablePreferenceStore::bootstrap_new(&database_path).await?;

    assert_eq!(
        std::fs::symlink_metadata(&private_parent)?.mode() & 0o7777,
        0o700
    );
    let database_metadata = std::fs::symlink_metadata(&database_path)?;
    assert_eq!(database_metadata.mode() & 0o7777, 0o600);
    assert_eq!(database_metadata.nlink(), 1);
    assert_eq!(database_metadata.uid(), current_euid());

    for suffix in ["-wal", "-shm"] {
        let sidecar_path = sqlite_sidecar_path(&database_path, suffix);
        let metadata = std::fs::symlink_metadata(&sidecar_path)?;
        assert!(metadata.file_type().is_file());
        assert_eq!(metadata.mode() & 0o7777, 0o600);
        assert_eq!(metadata.nlink(), 1);
        assert_eq!(metadata.uid(), current_euid());
    }
    Ok(())
}

#[cfg(unix)]
#[tokio::test]
async fn durable_open_existing_rejects_permissive_database_mode() -> TestResult {
    let directory = private_tempdir()?;
    let database_path = directory.path().join("v2-memory.sqlite3");
    let _store = DurablePreferenceStore::bootstrap_new(&database_path).await?;
    std::fs::set_permissions(&database_path, std::fs::Permissions::from_mode(0o644))?;

    assert!(matches!(
        DurablePreferenceStore::open_existing(&database_path).await,
        Err(PreferenceCasError::Corrupt { detail })
            if detail.contains("must have mode 0o600")
    ));
    std::fs::set_permissions(&database_path, std::fs::Permissions::from_mode(0o600))?;
    Ok(())
}

#[cfg(unix)]
#[tokio::test]
async fn durable_open_existing_rejects_hardlinked_database() -> TestResult {
    let directory = private_tempdir()?;
    let database_path = directory.path().join("v2-memory.sqlite3");
    let alias_path = directory.path().join("database-hardlink.sqlite3");
    let _store = DurablePreferenceStore::bootstrap_new(&database_path).await?;
    std::fs::hard_link(&database_path, &alias_path)?;

    assert!(matches!(
        DurablePreferenceStore::open_existing(&database_path).await,
        Err(PreferenceCasError::Corrupt { detail })
            if detail.contains("exactly one hard link")
    ));
    std::fs::remove_file(alias_path)?;
    Ok(())
}

#[cfg(unix)]
#[tokio::test]
async fn durable_bootstrap_rejects_unsafe_or_symlink_parent() -> TestResult {
    use std::os::unix::fs::symlink;

    let directory = private_tempdir()?;
    let unsafe_parent = directory.path().join("unsafe");
    std::fs::create_dir(&unsafe_parent)?;
    std::fs::set_permissions(&unsafe_parent, std::fs::Permissions::from_mode(0o777))?;
    let unsafe_path = unsafe_parent.join("v2-memory.sqlite3");
    assert!(matches!(
        DurablePreferenceStore::bootstrap_new(&unsafe_path).await,
        Err(PreferenceCasError::Corrupt { detail })
            if detail.contains("owner-only state directory with mode 0o700")
    ));
    assert!(!unsafe_path.exists());

    let shared_parent = directory.path().join("shared");
    std::fs::create_dir(&shared_parent)?;
    std::fs::set_permissions(&shared_parent, std::fs::Permissions::from_mode(0o755))?;
    let shared_path = shared_parent.join("v2-memory.sqlite3");
    assert!(matches!(
        DurablePreferenceStore::bootstrap_new(&shared_path).await,
        Err(PreferenceCasError::Corrupt { detail })
            if detail.contains("move the database into a private state directory")
    ));
    assert_eq!(
        std::fs::symlink_metadata(&shared_parent)?.mode() & 0o7777,
        0o755
    );
    assert!(!shared_path.exists());

    let real_parent = directory.path().join("real");
    let alias_parent = directory.path().join("alias");
    std::fs::create_dir(&real_parent)?;
    std::fs::set_permissions(&real_parent, std::fs::Permissions::from_mode(0o700))?;
    symlink(&real_parent, &alias_parent)?;
    let alias_path = alias_parent.join("v2-memory.sqlite3");
    assert!(matches!(
        DurablePreferenceStore::bootstrap_new(&alias_path).await,
        Err(PreferenceCasError::Corrupt { detail })
            if detail.contains("not a non-symlink directory")
    ));
    assert!(!real_parent.join("v2-memory.sqlite3").exists());
    Ok(())
}

#[cfg(unix)]
#[tokio::test]
async fn durable_open_existing_rejects_parent_that_became_unsafe() -> TestResult {
    let directory = private_tempdir()?;
    let private_parent = directory.path().join("private");
    std::fs::create_dir(&private_parent)?;
    std::fs::set_permissions(&private_parent, std::fs::Permissions::from_mode(0o700))?;
    let database_path = private_parent.join("v2-memory.sqlite3");
    drop(DurablePreferenceStore::bootstrap_new(&database_path).await?);
    std::fs::set_permissions(&private_parent, std::fs::Permissions::from_mode(0o777))?;

    assert!(matches!(
        DurablePreferenceStore::open_existing(&database_path).await,
        Err(PreferenceCasError::Corrupt { detail })
            if detail.contains("owner-only state directory with mode 0o700")
    ));
    std::fs::set_permissions(&private_parent, std::fs::Permissions::from_mode(0o700))?;
    Ok(())
}

#[cfg(unix)]
#[tokio::test]
async fn durable_open_existing_rejects_insecure_sqlite_sidecar() -> TestResult {
    let directory = private_tempdir()?;
    let database_path = directory.path().join("v2-memory.sqlite3");
    let _store = DurablePreferenceStore::bootstrap_new(&database_path).await?;
    let wal_path = sqlite_sidecar_path(&database_path, "-wal");
    std::fs::set_permissions(&wal_path, std::fs::Permissions::from_mode(0o644))?;

    assert!(matches!(
        DurablePreferenceStore::open_existing(&database_path).await,
        Err(PreferenceCasError::Corrupt { detail })
            if detail.contains("SQLite sidecar") && detail.contains("must have mode 0o600")
    ));
    std::fs::set_permissions(&wal_path, std::fs::Permissions::from_mode(0o600))?;
    Ok(())
}

#[cfg(unix)]
#[tokio::test]
async fn durable_open_existing_rejects_hardlinked_sqlite_sidecar() -> TestResult {
    let directory = private_tempdir()?;
    let database_path = directory.path().join("v2-memory.sqlite3");
    let _store = DurablePreferenceStore::bootstrap_new(&database_path).await?;
    let wal_path = sqlite_sidecar_path(&database_path, "-wal");
    let alias_path = directory.path().join("wal-hardlink");
    std::fs::hard_link(&wal_path, &alias_path)?;

    assert!(matches!(
        DurablePreferenceStore::open_existing(&database_path).await,
        Err(PreferenceCasError::Corrupt { detail })
            if detail.contains("SQLite sidecar") && detail.contains("exactly one hard link")
    ));
    std::fs::remove_file(alias_path)?;
    Ok(())
}

#[cfg(unix)]
fn sqlite_sidecar_path(database_path: &std::path::Path, suffix: &str) -> std::path::PathBuf {
    let mut sidecar = database_path.as_os_str().to_os_string();
    sidecar.push(suffix);
    sidecar.into()
}

#[cfg(unix)]
fn current_euid() -> u32 {
    // SAFETY: `geteuid` has no preconditions and only reads process credentials.
    unsafe { libc::geteuid() }
}
