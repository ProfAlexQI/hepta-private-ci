use super::DirectoryAnchorV8;
use super::ExecutableIdentityV8;
use super::PROCESS_FD_LIFETIME_TEST_MUTEX;
use super::execveat::MAX_VERIFIED_EXECUTABLE_BYTES_V8;
use super::execveat_verified;
use super::verify_executable_beneath;
use sha2::Digest;
use sha2::Sha256;
use std::ffi::CString;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::path::PathBuf;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;

static NEXT_TEMP_ID: AtomicU64 = AtomicU64::new(0);

fn serialize_process_fd_lifetime() -> std::sync::MutexGuard<'static, ()> {
    PROCESS_FD_LIFETIME_TEST_MUTEX
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

struct TestDirectory {
    path: PathBuf,
}

impl TestDirectory {
    fn create(label: &str) -> Self {
        let sequence = NEXT_TEMP_ID.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "hepta-linux-v8-execveat-{label}-{}-{sequence}",
            std::process::id()
        ));
        fs::create_dir(&path).expect("create test directory");
        Self { path }
    }
}

impl Drop for TestDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

fn install_true_fixture(directory: &TestDirectory) {
    let source = ["/bin/true", "/usr/bin/true"]
        .into_iter()
        .find(|candidate| Path::new(candidate).is_file())
        .expect("find system true binary to copy as isolated fixture");
    fs::copy(source, directory.path.join("fixture")).expect("copy true fixture");
    fs::set_permissions(
        directory.path.join("fixture"),
        fs::Permissions::from_mode(0o700),
    )
    .expect("make fixture executable");
}

fn expected_identity(anchor: &DirectoryAnchorV8) -> ExecutableIdentityV8 {
    let file = anchor
        .open_regular_readonly_beneath(Path::new("fixture"))
        .expect("open fixture for frozen identity");
    let bytes = file
        .read_all(file.identity().size())
        .expect("read fixture for digest");
    ExecutableIdentityV8::new(file.identity(), Sha256::digest(bytes).into())
        .expect("construct frozen executable identity")
}

#[test]
fn rejects_wrong_frozen_digest() {
    let _process_guard = serialize_process_fd_lifetime();
    let temporary = TestDirectory::create("wrong-digest");
    install_true_fixture(&temporary);
    let anchor = DirectoryAnchorV8::open(&temporary.path).expect("open anchor");
    let measured = expected_identity(&anchor);
    let wrong = ExecutableIdentityV8::new(measured.file(), [0x55; 32])
        .expect("construct wrong expected identity");

    let error = verify_executable_beneath(&anchor, Path::new("fixture"), &wrong)
        .expect_err("wrong digest must fail closed");
    assert!(error.to_string().contains("SHA-256"));
}

#[test]
fn executes_isolated_verified_elf_fixture() {
    let _process_guard = serialize_process_fd_lifetime();
    let temporary = TestDirectory::create("execute");
    install_true_fixture(&temporary);
    let anchor = DirectoryAnchorV8::open(&temporary.path).expect("open anchor");
    let expected = expected_identity(&anchor);
    let executable = verify_executable_beneath(&anchor, Path::new("fixture"), &expected)
        .expect("verify fixture");

    // SAFETY: fork is used only inside this isolated test; the child invokes
    // the verified-FD execution boundary and exits immediately if it fails.
    let child = unsafe { libc::fork() };
    assert!(
        child >= 0,
        "fork failed: {}",
        std::io::Error::last_os_error()
    );
    if child == 0 {
        let arguments = [CString::new("hepta-v8-exec-fixture").expect("static argv")];
        match execveat_verified(&executable, &arguments, &[]) {
            Ok(never) => match never {},
            // SAFETY: this is the isolated child and execveat failed.
            Err(_) => unsafe { libc::_exit(127) },
        }
    }

    let mut status = 0;
    // SAFETY: `child` is the direct child returned by fork and `status` is
    // writable for waitpid.
    let waited = unsafe { libc::waitpid(child, &mut status, 0) };
    assert_eq!(waited, child);
    assert!(libc::WIFEXITED(status));
    assert_eq!(libc::WEXITSTATUS(status), 0);
}

#[test]
fn path_replacement_cannot_change_the_executable_fd() {
    let _process_guard = serialize_process_fd_lifetime();
    let temporary = TestDirectory::create("path-replacement");
    install_true_fixture(&temporary);
    let anchor = DirectoryAnchorV8::open(&temporary.path).expect("open anchor");
    let expected = expected_identity(&anchor);
    let executable = verify_executable_beneath(&anchor, Path::new("fixture"), &expected)
        .expect("verify fixture");
    fs::rename(
        temporary.path.join("fixture"),
        temporary.path.join("detached-fixture"),
    )
    .expect("rename fixture after verification");
    fs::write(temporary.path.join("fixture"), b"not the verified ELF")
        .expect("install replacement path");

    // SAFETY: fork is isolated to this test. Success proves execveat used the
    // already verified descriptor rather than reopening the replaced path.
    let child = unsafe { libc::fork() };
    assert!(
        child >= 0,
        "fork failed: {}",
        std::io::Error::last_os_error()
    );
    if child == 0 {
        let arguments = [CString::new("hepta-v8-exec-fixture").expect("static argv")];
        match execveat_verified(&executable, &arguments, &[]) {
            Ok(never) => match never {},
            // SAFETY: this is the isolated child and execveat failed.
            Err(_) => unsafe { libc::_exit(127) },
        }
    }
    let mut status = 0;
    // SAFETY: `child` is the direct child returned by fork and `status` is
    // writable for waitpid.
    let waited = unsafe { libc::waitpid(child, &mut status, 0) };
    assert_eq!(waited, child);
    assert!(libc::WIFEXITED(status));
    assert_eq!(libc::WEXITSTATUS(status), 0);
}

#[test]
fn in_place_source_mutation_cannot_change_the_sealed_executable() {
    let _process_guard = serialize_process_fd_lifetime();
    let temporary = TestDirectory::create("in-place-mutation");
    install_true_fixture(&temporary);
    let anchor = DirectoryAnchorV8::open(&temporary.path).expect("open anchor");
    let expected = expected_identity(&anchor);
    let executable = verify_executable_beneath(&anchor, Path::new("fixture"), &expected)
        .expect("verify fixture into sealed memfd");
    fs::write(
        temporary.path.join("fixture"),
        vec![0_u8; expected.file().size() as usize],
    )
    .expect("mutate source inode after sealed verification");

    // SAFETY: fork is isolated to this test. A zero exit proves execution used
    // the immutable memfd bytes rather than the subsequently modified source.
    let child = unsafe { libc::fork() };
    assert!(
        child >= 0,
        "fork failed: {}",
        std::io::Error::last_os_error()
    );
    if child == 0 {
        let arguments = [CString::new("hepta-v8-exec-fixture").expect("static argv")];
        match execveat_verified(&executable, &arguments, &[]) {
            Ok(never) => match never {},
            // SAFETY: this is the isolated child and execveat failed.
            Err(_) => unsafe { libc::_exit(127) },
        }
    }
    let mut status = 0;
    // SAFETY: `child` is the direct child and status is writable.
    let waited = unsafe { libc::waitpid(child, &mut status, 0) };
    assert_eq!(waited, child);
    assert!(libc::WIFEXITED(status));
    assert_eq!(libc::WEXITSTATUS(status), 0);
}

#[test]
fn rejects_non_elf_and_group_or_world_writable_sources() {
    let _process_guard = serialize_process_fd_lifetime();
    let non_elf = TestDirectory::create("non-elf");
    fs::write(non_elf.path.join("fixture"), b"#!/bin/sh\nexit 0\n").expect("write script");
    fs::set_permissions(
        non_elf.path.join("fixture"),
        fs::Permissions::from_mode(0o700),
    )
    .expect("make script executable");
    let anchor = DirectoryAnchorV8::open(&non_elf.path).expect("open non-elf anchor");
    let expected = expected_identity(&anchor);
    assert!(verify_executable_beneath(&anchor, Path::new("fixture"), &expected).is_err());

    let writable = TestDirectory::create("writable");
    install_true_fixture(&writable);
    fs::set_permissions(
        writable.path.join("fixture"),
        fs::Permissions::from_mode(0o722),
    )
    .expect("make fixture group/world writable");
    let anchor = DirectoryAnchorV8::open(&writable.path).expect("open writable anchor");
    let expected = expected_identity(&anchor);
    assert!(verify_executable_beneath(&anchor, Path::new("fixture"), &expected).is_err());
}

#[test]
fn rejects_sparse_oversize_executable_before_allocating_its_contents() {
    let _process_guard = serialize_process_fd_lifetime();
    let temporary = TestDirectory::create("oversize");
    let fixture = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(temporary.path.join("fixture"))
        .expect("create sparse executable fixture");
    fixture
        .set_len(MAX_VERIFIED_EXECUTABLE_BYTES_V8 + 1)
        .expect("make sparse oversize fixture");
    drop(fixture);
    fs::set_permissions(
        temporary.path.join("fixture"),
        fs::Permissions::from_mode(0o700),
    )
    .expect("make sparse fixture executable");
    let anchor = DirectoryAnchorV8::open(&temporary.path).expect("open anchor");
    let file = anchor
        .open_regular_readonly_beneath(Path::new("fixture"))
        .expect("open sparse fixture");
    let expected = ExecutableIdentityV8::new(file.identity(), [0x44; 32])
        .expect("construct frozen oversize identity");

    let error = verify_executable_beneath(&anchor, Path::new("fixture"), &expected)
        .expect_err("oversize executable must fail before content allocation");
    assert!(error.to_string().contains("frozen"));
}

#[test]
fn requires_nonempty_argv() {
    let _process_guard = serialize_process_fd_lifetime();
    let temporary = TestDirectory::create("argv");
    install_true_fixture(&temporary);
    let anchor = DirectoryAnchorV8::open(&temporary.path).expect("open anchor");
    let expected = expected_identity(&anchor);
    let executable = verify_executable_beneath(&anchor, Path::new("fixture"), &expected)
        .expect("verify fixture");

    let error =
        execveat_verified(&executable, &[], &[]).expect_err("empty argv must fail before execveat");
    assert!(error.to_string().contains("argv[0]"));
}
