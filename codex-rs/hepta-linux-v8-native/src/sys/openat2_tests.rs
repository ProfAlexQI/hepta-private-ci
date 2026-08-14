use super::DirectoryAnchorV8;
use std::ffi::OsStr;
use std::fs;
use std::os::unix::fs::symlink;
use std::path::Path;
use std::path::PathBuf;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;

static NEXT_TEMP_ID: AtomicU64 = AtomicU64::new(0);

struct TestDirectory {
    path: PathBuf,
}

impl TestDirectory {
    fn create(label: &str) -> Self {
        let sequence = NEXT_TEMP_ID.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "hepta-linux-v8-openat2-{label}-{}-{sequence}",
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

#[test]
fn opens_regular_file_beneath_anchor_and_reads_exact_bytes() {
    let temporary = TestDirectory::create("read");
    fs::create_dir(temporary.path.join("nested")).expect("create nested directory");
    fs::write(temporary.path.join("nested/evidence"), b"sealed evidence").expect("write fixture");
    let anchor = DirectoryAnchorV8::open(&temporary.path).expect("open anchor");

    let nested = anchor
        .open_directory_beneath(Path::new("nested"))
        .expect("open nested directory");
    let file = nested
        .open_regular_readonly_beneath(Path::new("evidence"))
        .expect("open evidence");

    assert_eq!(
        file.read_all(64).expect("read evidence"),
        b"sealed evidence"
    );
    assert_eq!(file.identity().size(), 15);
}

#[test]
fn rejects_parent_absolute_and_empty_paths_before_syscall() {
    let temporary = TestDirectory::create("invalid-path");
    let anchor = DirectoryAnchorV8::open(&temporary.path).expect("open anchor");

    for invalid in [
        Path::new(""),
        Path::new("../escape"),
        Path::new("a/../b"),
        Path::new("/etc/passwd"),
    ] {
        let error = anchor
            .open_regular_readonly_beneath(invalid)
            .expect_err("invalid path must fail closed");
        assert!(error.to_string().contains("relative"));
    }
}

#[test]
fn rejects_symlink_in_any_path_component() {
    let temporary = TestDirectory::create("symlink");
    fs::create_dir(temporary.path.join("real")).expect("create real directory");
    fs::write(temporary.path.join("real/file"), b"data").expect("write fixture");
    symlink("real", temporary.path.join("linked-directory")).expect("create directory symlink");
    symlink("real/file", temporary.path.join("linked-file")).expect("create file symlink");
    let anchor = DirectoryAnchorV8::open(&temporary.path).expect("open anchor");

    assert!(
        anchor
            .open_regular_readonly_beneath(Path::new("linked-directory/file"))
            .is_err()
    );
    assert!(
        anchor
            .open_regular_readonly_beneath(Path::new("linked-file"))
            .is_err()
    );
}

#[test]
fn initial_anchor_rejects_parent_symlinks_and_filesystem_root() {
    let temporary = TestDirectory::create("anchor-parent-symlink");
    fs::create_dir(temporary.path.join("real-parent")).expect("create real parent");
    fs::create_dir(temporary.path.join("real-parent/state")).expect("create state root");
    symlink("real-parent", temporary.path.join("linked-parent")).expect("create parent symlink");

    assert!(DirectoryAnchorV8::open(&temporary.path.join("linked-parent/state")).is_err());
    assert!(DirectoryAnchorV8::open(Path::new("/")).is_err());
}

#[test]
fn enforces_caller_size_limit() {
    let temporary = TestDirectory::create("size-limit");
    fs::write(temporary.path.join("file"), b"12345").expect("write fixture");
    let anchor = DirectoryAnchorV8::open(&temporary.path).expect("open anchor");
    let file = anchor
        .open_regular_readonly_beneath(Path::new("file"))
        .expect("open fixture");

    let error = file
        .read_all(4)
        .expect_err("oversize read must fail closed");
    assert!(error.to_string().contains("exceeds maximum"));
}

#[test]
fn exclusive_leaf_typestate_writes_fsyncs_and_reopens() {
    let temporary = TestDirectory::create("exclusive-leaf");
    let anchor = DirectoryAnchorV8::open(&temporary.path).expect("open anchor");
    let created = anchor
        .create_regular_leaf_exclusive(OsStr::new("record.incoming"))
        .expect("create exclusive leaf");
    let synced = created
        .write_all_and_sync(b"canonical record")
        .expect("write and sync leaf");
    anchor.sync_directory().expect("sync directory");

    assert_eq!(synced.leaf(), OsStr::new("record.incoming"));
    assert_eq!(synced.identity().mode(), 0o600);
    assert_eq!(synced.identity().link_count(), 1);
    assert_eq!(synced.identity().size(), 16);
    synced.revalidate().expect("revalidate synced descriptor");
    let reopened = anchor
        .open_regular_readonly_beneath(Path::new("record.incoming"))
        .expect("reopen final leaf through openat2");
    assert_eq!(
        reopened.read_all(16).expect("read reopened leaf"),
        b"canonical record"
    );

    let error = anchor
        .create_regular_leaf_exclusive(OsStr::new("record.incoming"))
        .expect_err("O_EXCL must reject existing leaf");
    assert!(error.to_string().contains("exclusive"));
}

#[test]
fn lists_one_level_in_deterministic_order() {
    let temporary = TestDirectory::create("list");
    fs::write(temporary.path.join("z-final"), b"z").expect("write z");
    fs::write(temporary.path.join("a.incoming"), b"a").expect("write a");
    fs::create_dir(temporary.path.join("middle")).expect("create middle directory");
    fs::write(temporary.path.join("middle/not-a-leaf"), b"nested").expect("write nested");
    let anchor = DirectoryAnchorV8::open(&temporary.path).expect("open anchor");

    assert_eq!(
        anchor.list_leaf_names().expect("list leaves"),
        vec![
            OsStr::new("a.incoming").to_owned(),
            OsStr::new("middle").to_owned(),
            OsStr::new("z-final").to_owned(),
        ]
    );
}

#[test]
fn bounded_listing_rejects_the_first_excess_leaf() {
    let temporary = TestDirectory::create("bounded-list");
    fs::write(temporary.path.join("one"), b"1").expect("write first leaf");
    fs::write(temporary.path.join("two"), b"2").expect("write second leaf");
    let anchor = DirectoryAnchorV8::open(&temporary.path).expect("open anchor");
    assert!(anchor.list_leaf_names_bounded(1).is_err());
    assert_eq!(
        anchor
            .list_leaf_names_bounded(2)
            .expect("bounded list")
            .len(),
        2
    );
    assert!(anchor.list_leaf_names_bounded(0).is_err());
}
