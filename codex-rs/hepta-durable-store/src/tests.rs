use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::os::unix::fs::symlink;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;
use anyhow::Result;
use tempfile::TempDir;

use super::*;

fn private_root() -> Result<TempDir> {
    let root = TempDir::new()?;
    fs::set_permissions(
        root.path(),
        fs::Permissions::from_mode(PRIVATE_DIRECTORY_MODE),
    )?;
    Ok(root)
}

#[test]
fn update_is_private_atomic_bounded_and_recoverable() -> Result<()> {
    let root = private_root()?;
    let path = root.path().join("journal.json");
    let store = AuthenticatedJournalStore::new(&path, 64, "journal")?;
    store.update(|current| {
        assert!(current.is_none());
        Ok((b"first".to_vec(), ()))
    })?;
    let before = store.read()?.context("published journal")?;
    assert_eq!(before, b"first");
    assert_eq!(
        fs::metadata(&path)?.permissions().mode() & 0o7777,
        PRIVATE_FILE_MODE
    );
    let failed = store.update::<()>(|_| anyhow::bail!("crash before publication"));
    assert!(failed.is_err());
    assert_eq!(store.read()?.context("preserved journal")?, before);
    assert!(store.publish(&[0; 65]).is_err());
    Ok(())
}

#[test]
fn symlink_redirection_and_non_private_parent_fail_closed() -> Result<()> {
    let root = private_root()?;
    let victim = root.path().join("victim");
    fs::write(&victim, b"victim")?;
    fs::set_permissions(&victim, fs::Permissions::from_mode(PRIVATE_FILE_MODE))?;
    let target = root.path().join("journal.json");
    symlink(&victim, &target)?;
    let store = AuthenticatedJournalStore::new(&target, 64, "journal")?;
    assert!(store.read().is_err());
    assert!(store.publish(b"replacement").is_err());
    assert_eq!(fs::read(&victim)?, b"victim");

    let public = root.path().join("public");
    fs::create_dir(&public)?;
    fs::set_permissions(&public, fs::Permissions::from_mode(0o755))?;
    let public_store = AuthenticatedJournalStore::new(public.join("journal.json"), 64, "journal")?;
    assert!(public_store.publish(b"denied").is_err());
    Ok(())
}

#[test]
fn legacy_migration_store_is_narrow_and_keeps_the_parent_mode_unchanged() -> Result<()> {
    let root = TempDir::new()?;
    fs::set_permissions(root.path(), fs::Permissions::from_mode(0o755))?;
    let path = root.path().join("journal.json");

    let normal = AuthenticatedJournalStore::new(&path, 64, "journal")?;
    assert!(normal.publish(b"denied").is_err());

    let legacy = LegacyJournalMigrationStore::new(&path, 64, "journal")?;
    let _lock = legacy.lock()?;
    legacy.publish(b"migration-only")?;
    assert_eq!(legacy.read()?.context("legacy journal")?, b"migration-only");
    assert_eq!(
        fs::metadata(root.path())?.permissions().mode() & 0o7777,
        0o755
    );
    assert_eq!(
        fs::metadata(&path)?.permissions().mode() & 0o7777,
        PRIVATE_FILE_MODE
    );
    Ok(())
}

#[test]
fn legacy_migration_store_rejects_writable_parents_and_links() -> Result<()> {
    let root = TempDir::new()?;
    fs::set_permissions(root.path(), fs::Permissions::from_mode(0o775))?;
    let path = root.path().join("journal.json");
    let legacy = LegacyJournalMigrationStore::new(&path, 64, "journal")?;
    assert!(legacy.lock().is_err());
    assert!(legacy.publish(b"denied").is_err());

    fs::set_permissions(root.path(), fs::Permissions::from_mode(0o755))?;
    let victim = root.path().join("victim");
    fs::write(&victim, b"victim")?;
    fs::set_permissions(&victim, fs::Permissions::from_mode(PRIVATE_FILE_MODE))?;
    symlink(&victim, &path)?;
    assert!(legacy.read().is_err());
    assert!(legacy.publish(b"replacement").is_err());
    assert_eq!(fs::read(&victim)?, b"victim");
    Ok(())
}

#[cfg(target_os = "macos")]
#[test]
fn macos_system_root_alias_support_keeps_nested_symlinks_fail_closed() -> Result<()> {
    assert_eq!(
        platform::normalize_macos_system_root_alias(Path::new("/var/tmp/journal.json")),
        PathBuf::from("/private/var/tmp/journal.json")
    );
    assert_eq!(
        platform::normalize_macos_system_root_alias(Path::new("/various/journal.json")),
        PathBuf::from("/various/journal.json")
    );

    let root = tempfile::Builder::new()
        .prefix("hepta-durable-store-")
        .tempdir_in("/var/tmp")?;
    fs::set_permissions(
        root.path(),
        fs::Permissions::from_mode(PRIVATE_DIRECTORY_MODE),
    )?;
    let store = AuthenticatedJournalStore::new(root.path().join("journal.json"), 64, "journal")?;
    store.publish(b"through-system-alias")?;
    assert_eq!(
        store.read()?.context("published journal")?,
        b"through-system-alias"
    );

    let outside = root.path().join("outside");
    let linked = root.path().join("linked");
    fs::create_dir(&outside)?;
    symlink(&outside, &linked)?;
    let redirected = AuthenticatedJournalStore::new(linked.join("journal.json"), 64, "journal")?;
    assert!(redirected.publish(b"denied").is_err());
    assert!(!outside.join("journal.json").exists());
    Ok(())
}
