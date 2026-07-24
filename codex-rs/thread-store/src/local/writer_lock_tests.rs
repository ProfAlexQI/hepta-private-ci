use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use codex_protocol::ThreadId;
use tempfile::TempDir;

use super::COORDINATION_LOCK_FILE;
use super::WRITER_LOCK_DIR;
use super::WriterLockCoordinator;
use crate::ThreadStoreError;

#[test]
fn writer_locks_reject_competing_owners_and_release_their_files() {
    let home = TempDir::new().expect("temp dir");
    let primary = Arc::new(WriterLockCoordinator::new(home.path()));
    let secondary = Arc::new(WriterLockCoordinator::new(home.path()));
    let thread_id = ThreadId::default();
    let other_thread_id = ThreadId::default();

    let owner = primary.acquire(thread_id).expect("acquire writer lock");
    let lock_path = home
        .path()
        .join(WRITER_LOCK_DIR)
        .join(format!("{thread_id}.lock"));
    assert!(lock_path.exists());

    let err = match secondary.acquire(thread_id) {
        Ok(_) => panic!("competing owner should fail"),
        Err(err) => err,
    };
    assert!(matches!(err, ThreadStoreError::Conflict { .. }));
    let other_owner = secondary
        .acquire(other_thread_id)
        .expect("other thread should acquire its own lock");

    drop(owner);
    assert!(!lock_path.exists());
    let next_owner = secondary
        .acquire(thread_id)
        .expect("released thread should accept another owner");
    drop(next_owner);
    drop(other_owner);

    let entries = fs::read_dir(home.path().join(WRITER_LOCK_DIR))
        .expect("read lock directory")
        .map(|entry| entry.expect("lock directory entry").file_name())
        .collect::<Vec<_>>();
    assert_eq!(entries, vec![COORDINATION_LOCK_FILE]);
}

#[test]
fn first_acquisition_removes_stale_locks_without_removing_active_locks() {
    let home = TempDir::new().expect("temp dir");
    let primary = Arc::new(WriterLockCoordinator::new(home.path()));
    let active_thread_id = ThreadId::default();
    let active_owner = primary
        .acquire(active_thread_id)
        .expect("acquire active writer lock");

    let stale_thread_id = ThreadId::default();
    let stale_path = home
        .path()
        .join(WRITER_LOCK_DIR)
        .join(format!("{stale_thread_id}.lock"));
    fs::File::create(&stale_path).expect("create stale writer lock");

    let secondary = Arc::new(WriterLockCoordinator::new(home.path()));
    let secondary_owner = secondary
        .acquire(ThreadId::default())
        .expect("acquire writer lock after cleanup");

    assert!(!stale_path.exists());
    let err = match secondary.acquire(active_thread_id) {
        Ok(_) => panic!("active writer should survive cleanup"),
        Err(err) => err,
    };
    assert!(matches!(err, ThreadStoreError::Conflict { .. }));

    drop(secondary_owner);
    drop(active_owner);
}

#[cfg(unix)]
#[test]
fn separate_process_owns_writer_until_drop() {
    const CHILD_MODE: &str = "CODEX_THREAD_STORE_WRITER_LOCK_CHILD";
    const HOME_ENV: &str = "CODEX_THREAD_STORE_WRITER_LOCK_HOME";
    const THREAD_ID_ENV: &str = "CODEX_THREAD_STORE_WRITER_LOCK_THREAD_ID";
    const READY_ENV: &str = "CODEX_THREAD_STORE_WRITER_LOCK_READY";
    const RELEASE_ENV: &str = "CODEX_THREAD_STORE_WRITER_LOCK_RELEASE";

    if std::env::var_os(CHILD_MODE).is_some() {
        let home = PathBuf::from(std::env::var_os(HOME_ENV).expect("child home"));
        let thread_id = ThreadId::from_string(
            std::env::var(THREAD_ID_ENV)
                .expect("child thread id")
                .as_str(),
        )
        .expect("valid child thread id");
        let ready = PathBuf::from(std::env::var_os(READY_ENV).expect("child ready path"));
        let release = PathBuf::from(std::env::var_os(RELEASE_ENV).expect("child release path"));
        let coordinator = Arc::new(WriterLockCoordinator::new(&home));
        let _owner = coordinator.acquire(thread_id).expect("child writer lock");
        fs::write(&ready, b"ready").expect("child ready marker");
        for _ in 0..1_500 {
            if release.exists() {
                return;
            }
            std::thread::sleep(Duration::from_millis(10));
        }
        panic!("parent never released child writer");
    }

    let home = TempDir::new().expect("temp dir");
    let ready = home.path().join("ready");
    let release = home.path().join("release");
    let thread_id = ThreadId::default();
    let test_name = "local::writer_lock::tests::separate_process_owns_writer_until_drop";
    let mut child = std::process::Command::new(std::env::current_exe().expect("test binary"))
        .arg("--exact")
        .arg(test_name)
        .arg("--test-threads=1")
        .env(CHILD_MODE, "1")
        .env(HOME_ENV, home.path())
        .env(THREAD_ID_ENV, thread_id.to_string())
        .env(READY_ENV, &ready)
        .env(RELEASE_ENV, &release)
        .spawn()
        .expect("spawn writer lock holder");

    for _ in 0..1_000 {
        if ready.exists() {
            break;
        }
        if child.try_wait().expect("child status").is_some() {
            break;
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    let ready_observed = ready.exists();
    let contender = if ready_observed {
        Arc::new(WriterLockCoordinator::new(home.path())).acquire(thread_id)
    } else {
        Err(ThreadStoreError::Internal {
            message: "child did not publish ready marker".to_string(),
        })
    };
    fs::write(&release, b"release").expect("release child");
    let child_status = child.wait().expect("wait for writer lock holder");

    assert!(ready_observed, "child did not acquire the writer lock");
    let err = match contender {
        Ok(_) => panic!("other process must be rejected"),
        Err(err) => err,
    };
    assert!(matches!(err, ThreadStoreError::Conflict { .. }));
    assert!(child_status.success(), "{child_status}");
    Arc::new(WriterLockCoordinator::new(home.path()))
        .acquire(thread_id)
        .expect("child drop should transfer ownership");
}
