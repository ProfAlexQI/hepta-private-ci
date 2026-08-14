use std::ffi::OsStr;
use std::fs;
use std::os::unix::fs::PermissionsExt;

use super::DirectoryAnchorV8;
use super::PROCESS_FD_LIFETIME_TEST_MUTEX;
use super::acquire_state_root_lock_v8;
use super::open_existing_state_root_lock_v8;

fn serialize_process_fd_lifetime() -> std::sync::MutexGuard<'static, ()> {
    PROCESS_FD_LIFETIME_TEST_MUTEX
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

fn temporary_directory() -> std::path::PathBuf {
    let path = std::env::temp_dir().join(format!(
        "hepta-linux-v8-lock-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir(&path).unwrap();
    path
}

#[test]
fn singleton_lock_is_exclusive_and_released_only_on_drop() {
    let _process_guard = serialize_process_fd_lifetime();
    let root = temporary_directory();
    let anchor = DirectoryAnchorV8::open(&root).unwrap();
    let lock = acquire_state_root_lock_v8(&anchor, OsStr::new("state.lock")).unwrap();
    assert_eq!(lock.identity().mode(), 0o600);
    assert_eq!(lock.owner_pid(), std::process::id());
    assert!(acquire_state_root_lock_v8(&anchor, OsStr::new("state.lock")).is_err());
    drop(lock);
    let reacquired = acquire_state_root_lock_v8(&anchor, OsStr::new("state.lock")).unwrap();
    drop(reacquired);
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn lock_rejects_traversal_nonempty_or_hardlinked_files() {
    let _process_guard = serialize_process_fd_lifetime();
    let root = temporary_directory();
    let anchor = DirectoryAnchorV8::open(&root).unwrap();
    assert!(acquire_state_root_lock_v8(&anchor, OsStr::new("../lock")).is_err());

    fs::write(root.join("nonempty.lock"), b"caller facts").unwrap();
    assert!(acquire_state_root_lock_v8(&anchor, OsStr::new("nonempty.lock")).is_err());
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn unlinked_or_replaced_lock_name_invalidates_the_live_token() {
    let _process_guard = serialize_process_fd_lifetime();
    let root = temporary_directory();
    let anchor = DirectoryAnchorV8::open(&root).unwrap();
    let lock = acquire_state_root_lock_v8(&anchor, OsStr::new("state.lock")).unwrap();
    fs::remove_file(root.join("state.lock")).unwrap();
    fs::write(root.join("state.lock"), b"").unwrap();
    let mut permissions = fs::metadata(root.join("state.lock")).unwrap().permissions();
    permissions.set_mode(0o600);
    fs::set_permissions(root.join("state.lock"), permissions).unwrap();
    assert!(lock.revalidate_for_root(&anchor).is_err());
    drop(lock);
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn production_lock_open_never_creates_and_requires_exact_existing_leaf() {
    let _process_guard = serialize_process_fd_lifetime();
    let root = temporary_directory();
    let anchor = DirectoryAnchorV8::open(&root).unwrap();
    let leaf = OsStr::new("state.lock");
    assert!(open_existing_state_root_lock_v8(&anchor, leaf).is_err());
    assert!(!root.join(leaf).exists());

    fs::write(root.join(leaf), b"").unwrap();
    let mut permissions = fs::metadata(root.join(leaf)).unwrap().permissions();
    permissions.set_mode(0o600);
    fs::set_permissions(root.join(leaf), permissions).unwrap();
    let lock = open_existing_state_root_lock_v8(&anchor, leaf).unwrap();
    assert!(open_existing_state_root_lock_v8(&anchor, leaf).is_err());
    lock.revalidate_for_root(&anchor).unwrap();
    drop(lock);
    fs::remove_dir_all(root).unwrap();
}
