use super::*;
use crate::mac_disposable_lifecycle::DisposableLifecycleEventV2;
use crate::mac_disposable_lifecycle::DisposableLifecycleJournalV2;
use std::ffi::CString;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::os::fd::AsRawFd;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

fn digest(byte: char) -> String {
    byte.to_string().repeat(64)
}

fn prepared_event() -> DisposableLifecycleEventV2 {
    DisposableLifecycleEventV2::OperationPrepared {
        baseline_inventory_sha256: digest('1'),
        backing_identity_sha256: digest('2'),
        boot_session_uuid: "12345678-1234-1234-1234-123456789abc".to_string(),
        collector_policy_sha256: digest('3'),
        mountpoint_underlying_sha256: digest('4'),
    }
}

fn create_root(parent: &Path) -> std::path::PathBuf {
    let root = parent.join("control");
    drop(LivePrivilegedDisposablePolicyV2::create_for_test(&root).expect("initialize control"));
    root
}

fn write_operation(root: &Path, directory_nonce: &str, journal_nonce: &str) {
    let mut bytes = Vec::new();
    let mut journal = DisposableLifecycleJournalV2::new(journal_nonce).expect("journal");
    journal
        .append_with(prepared_event(), |_, canonical| {
            bytes.extend_from_slice(canonical);
            Ok(())
        })
        .expect("persist prepared record");

    let operation = root
        .join(OPERATIONS_NAME)
        .join(format!("{OPERATION_PREFIX}{directory_nonce}"));
    fs::create_dir(&operation).expect("create operation");
    fs::set_permissions(&operation, fs::Permissions::from_mode(0o700)).expect("set operation mode");
    let mut record = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o400)
        .open(operation.join("00000001.json"))
        .expect("create operation record");
    record.write_all(&bytes).expect("write operation record");
    record.sync_all().expect("sync operation record");
}

fn assert_rejected(
    result: Result<LivePrivilegedDisposableExecutionV2<'_>, PrivilegedDisposableControlErrorV2>,
) {
    assert!(
        result.is_err(),
        "storage mutation unexpectedly passed census"
    );
}

#[test]
fn clean_storage_receipt_is_complete_but_never_grants_authority() {
    let temporary = tempfile::tempdir().expect("temporary root parent");
    let root = temporary.path().join("control");
    let control = LivePrivilegedDisposablePolicyV2::create_for_test(&root).expect("open control");
    let assessment = control.assess_read_only().expect("read-only assessment");
    let receipt = assessment.receipt();
    assert!(!receipt.admission_authority);
    assert!(!receipt.authority.any());
    assert!(receipt.closed_world_roster_verified);
    assert!(receipt.storage_precondition_satisfied);
    assert!(!receipt.new_operation_precondition_satisfied);
    assert!(receipt.blocking_operation_nonces.is_empty());
    assert!(receipt.completed_operation_nonces.is_empty());
}

#[test]
fn operation_directory_nonce_is_bound_to_v2_journal_nonce() {
    let temporary = tempfile::tempdir().expect("temporary root parent");
    let root = create_root(temporary.path());
    write_operation(&root, &digest('a'), &digest('b'));
    let control = LivePrivilegedDisposablePolicyV2::create_for_test(&root).expect("reopen control");
    let error = match control.assess_read_only() {
        Ok(_) => panic!("transplanted journal unexpectedly passed"),
        Err(error) => error,
    };
    assert!(error.to_string().contains("directory nonce differs"));
}

#[test]
fn blocking_operation_receipt_remains_storage_only() {
    let temporary = tempfile::tempdir().expect("temporary root parent");
    let root = create_root(temporary.path());
    let nonce = digest('a');
    write_operation(&root, &nonce, &nonce);
    let control = LivePrivilegedDisposablePolicyV2::create_for_test(&root).expect("reopen control");
    let assessment = control.assess_read_only().expect("assess operation");
    let receipt = assessment.receipt();
    assert_eq!(receipt.blocking_operation_nonces, [nonce]);
    assert!(receipt.closed_world_roster_verified);
    assert!(receipt.storage_precondition_satisfied);
    assert!(!receipt.admission_authority);
    assert!(!receipt.authority.any());
    assert!(!receipt.new_operation_precondition_satisfied);
}

#[derive(Clone, Copy, Debug)]
enum MetadataMutation {
    ExtendedAcl,
    ExtendedAttribute,
    HardLink,
    HiddenFlag,
    NonEmptyLock,
    WrongMode,
}

#[test]
fn exact_metadata_profile_rejection_matrix() {
    for mutation in [
        MetadataMutation::ExtendedAcl,
        MetadataMutation::ExtendedAttribute,
        MetadataMutation::HardLink,
        MetadataMutation::HiddenFlag,
        MetadataMutation::NonEmptyLock,
        MetadataMutation::WrongMode,
    ] {
        let temporary = tempfile::tempdir().expect("temporary root parent");
        let root = create_root(temporary.path());
        match mutation {
            MetadataMutation::ExtendedAcl => {
                let status = std::process::Command::new("/bin/chmod")
                    .args(["+a", "everyone allow read"])
                    .arg(root.join(OPERATIONS_NAME))
                    .status()
                    .expect("invoke chmod +a");
                assert!(status.success(), "install test ACL");
            }
            MetadataMutation::ExtendedAttribute => {
                let operations =
                    fs::File::open(root.join(OPERATIONS_NAME)).expect("open operations");
                let name = CString::new("com.hepta.storage-census-test").expect("xattr name");
                let value = b"present";
                let rc = unsafe {
                    libc::fsetxattr(
                        operations.as_raw_fd(),
                        name.as_ptr(),
                        value.as_ptr().cast(),
                        value.len(),
                        0,
                        0,
                    )
                };
                assert_eq!(rc, 0, "set xattr: {}", std::io::Error::last_os_error());
            }
            MetadataMutation::HardLink => {
                fs::hard_link(root.join(LOCK_NAME), temporary.path().join("lock-alias"))
                    .expect("hard-link lock");
            }
            MetadataMutation::HiddenFlag => {
                let operations =
                    fs::File::open(root.join(OPERATIONS_NAME)).expect("open operations");
                let rc = unsafe { libc::fchflags(operations.as_raw_fd(), libc::UF_HIDDEN) };
                assert_eq!(rc, 0, "set flags: {}", std::io::Error::last_os_error());
            }
            MetadataMutation::NonEmptyLock => {
                OpenOptions::new()
                    .append(true)
                    .open(root.join(LOCK_NAME))
                    .expect("open lock")
                    .write_all(b"not-empty")
                    .expect("write lock");
            }
            MetadataMutation::WrongMode => {
                fs::set_permissions(
                    root.join(OPERATIONS_NAME),
                    fs::Permissions::from_mode(0o750),
                )
                .expect("change operations mode");
            }
        }
        let reopened = LivePrivilegedDisposablePolicyV2::create_for_test(&root);
        if matches!(mutation, MetadataMutation::ExtendedAcl) {
            let status = std::process::Command::new("/bin/chmod")
                .arg("-N")
                .arg(root.join(OPERATIONS_NAME))
                .status()
                .expect("invoke chmod -N");
            assert!(status.success(), "remove test ACL");
        }
        assert!(
            reopened.is_err(),
            "{mutation:?} unexpectedly passed the exact metadata profile"
        );
    }
}

#[derive(Clone, Copy, Debug)]
enum UnsupportedNode {
    Fifo,
    Symlink,
}

fn create_fifo(path: &Path) {
    let path = CString::new(path.as_os_str().as_bytes()).expect("FIFO path");
    let rc = unsafe { libc::mkfifo(path.as_ptr(), 0o600) };
    assert_eq!(rc, 0, "create FIFO: {}", std::io::Error::last_os_error());
}

#[test]
fn symlink_and_special_operation_nodes_are_rejected() {
    for node in [UnsupportedNode::Symlink, UnsupportedNode::Fifo] {
        let temporary = tempfile::tempdir().expect("temporary root parent");
        let root = create_root(temporary.path());
        let entry = root
            .join(OPERATIONS_NAME)
            .join(format!("{OPERATION_PREFIX}{}", digest('c')));
        match node {
            UnsupportedNode::Symlink => {
                std::os::unix::fs::symlink(temporary.path(), &entry).expect("create symlink");
            }
            UnsupportedNode::Fifo => create_fifo(&entry),
        }
        let control =
            LivePrivilegedDisposablePolicyV2::create_for_test(&root).expect("reopen control");
        assert_rejected(control.assess_read_only());
    }
}

#[test]
fn final_revalidation_rejects_roster_mutation() {
    let temporary = tempfile::tempdir().expect("temporary root parent");
    let root = temporary.path().join("control");
    let control = LivePrivilegedDisposablePolicyV2::create_for_test(&root).expect("open control");
    let operations = root.join(OPERATIONS_NAME);
    let result = control.assess_read_only_with_hook(|| {
        fs::write(operations.join("late-entry"), b"mutation")?;
        Ok(())
    });
    assert_rejected(result);
}

#[test]
fn injected_cross_filesystem_binding_is_rejected() {
    let temporary = tempfile::tempdir().expect("temporary root parent");
    let root = temporary.path().join("control");
    let mut control =
        LivePrivilegedDisposablePolicyV2::create_for_test(&root).expect("open control");
    control.filesystem.dev ^= 1;
    assert_rejected(control.assess_read_only());
}

fn create_sealed_fixture(parent: &Path, name: &str, unsupported: Option<UnsupportedNode>) {
    let fixture = parent.join(name);
    fs::create_dir(&fixture).expect("create fixture");
    let logs = fixture.join("logs");
    let mountpoint = fixture.join("mountpoint");
    fs::create_dir(&logs).expect("create logs");
    fs::create_dir(&mountpoint).expect("create mountpoint");
    let mut receipt = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o400)
        .open(logs.join("receipt.json"))
        .expect("create fixture receipt");
    receipt.write_all(b"{}\n").expect("write fixture receipt");
    receipt.sync_all().expect("sync fixture receipt");
    if let Some(unsupported) = unsupported {
        let bad = logs.join("unsupported");
        match unsupported {
            UnsupportedNode::Symlink => {
                std::os::unix::fs::symlink(&mountpoint, bad).expect("create fixture symlink");
            }
            UnsupportedNode::Fifo => create_fifo(&bad),
        }
    }
    fs::set_permissions(&logs, fs::Permissions::from_mode(0o500)).expect("seal logs");
    fs::set_permissions(&mountpoint, fs::Permissions::from_mode(0o700)).expect("seal mountpoint");
    fs::set_permissions(&fixture, fs::Permissions::from_mode(0o500)).expect("seal fixture");
}

#[test]
fn sealed_fixture_recursive_descriptor_census_matrix() {
    for unsupported in [
        None,
        Some(UnsupportedNode::Symlink),
        Some(UnsupportedNode::Fifo),
    ] {
        let temporary = tempfile::tempdir().expect("temporary fixture parent");
        create_sealed_fixture(temporary.path(), "fixture", unsupported);
        let parent = open_path_directory(temporary.path()).expect("open fixture parent");
        let filesystem = filesystem_binding(&parent).expect("bind fixture filesystem");
        let mut retained = 0;
        let result = open_sealed_fixture_node(
            parent.as_raw_fd(),
            "fixture",
            ".",
            0,
            &mut retained,
            unsafe { libc::geteuid() },
            unsafe { libc::getegid() },
            &filesystem,
        );
        match unsupported {
            None => {
                let node = result.expect("census valid fixture");
                node.revalidate(
                    parent.as_raw_fd(),
                    unsafe { libc::geteuid() },
                    unsafe { libc::getegid() },
                    &filesystem,
                )
                .expect("revalidate valid fixture");
                let mut registry = CensusRegistry::default();
                node.register(&mut registry, "publication")
                    .expect("register fixture identities");
                assert_eq!(retained, 4);
            }
            Some(_) => assert!(result.is_err(), "unsupported fixture node passed census"),
        }
        fs::set_permissions(
            temporary.path().join("fixture/logs"),
            fs::Permissions::from_mode(0o700),
        )
        .expect("unseal fixture logs for cleanup");
        fs::set_permissions(
            temporary.path().join("fixture"),
            fs::Permissions::from_mode(0o700),
        )
        .expect("unseal fixture for cleanup");
    }
}

#[test]
fn historical_barrier_records_are_descriptor_bound_and_revalidated() {
    let temporary = tempfile::tempdir().expect("temporary barrier parent");
    let root = temporary.path().join("control");
    let control = LivePrivilegedDisposablePolicyV2::create_for_test(&root).expect("open control");
    let barrier_path = temporary.path().join("barrier");
    fs::create_dir(&barrier_path).expect("create barrier");
    fs::set_permissions(&barrier_path, fs::Permissions::from_mode(0o700))
        .expect("set barrier mode");
    let mut record = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o400)
        .open(barrier_path.join("00000000000000000001.json"))
        .expect("create barrier record");
    record.write_all(b"{}\n").expect("write barrier record");
    record.sync_all().expect("sync barrier record");
    let barrier = open_path_directory(&barrier_path).expect("open barrier");
    let mut total_bytes = 0;
    let (roster, nodes) = control
        .open_barrier_journal(&barrier, &mut total_bytes)
        .expect("census barrier");
    assert_eq!(roster, ["00000000000000000001.json"]);
    assert_eq!(total_bytes, 3);
    let mut registry = CensusRegistry::default();
    for node in &nodes {
        node.revalidate(
            barrier.as_raw_fd(),
            control.expected_uid,
            control.expected_gid,
            &control.filesystem,
        )
        .expect("revalidate barrier record");
        node.register(&mut registry, "barrier-journal")
            .expect("register barrier record");
    }
}
