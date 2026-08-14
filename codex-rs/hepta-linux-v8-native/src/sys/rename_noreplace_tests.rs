use super::DirectoryAnchorV8;
use super::rename_noreplace_at;
use std::ffi::OsStr;
use std::fs;
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
            "hepta-linux-v8-rename-{label}-{}-{sequence}",
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
fn atomically_publishes_new_leaf() {
    let temporary = TestDirectory::create("publish");
    fs::write(temporary.path.join("incoming"), b"immutable payload").expect("write incoming");
    let anchor = DirectoryAnchorV8::open(&temporary.path).expect("open anchor");

    let published = rename_noreplace_at(
        &anchor,
        OsStr::new("incoming"),
        &anchor,
        OsStr::new("final"),
    )
    .expect("publish no-replace");

    assert_eq!(published.source_leaf(), OsStr::new("incoming"));
    assert_eq!(published.destination_leaf(), OsStr::new("final"));
    assert!(!temporary.path.join("incoming").exists());
    assert_eq!(
        fs::read(temporary.path.join("final")).expect("read final"),
        b"immutable payload"
    );
}

#[test]
fn never_replaces_existing_destination() {
    let temporary = TestDirectory::create("collision");
    fs::write(temporary.path.join("incoming"), b"new").expect("write incoming");
    fs::write(temporary.path.join("final"), b"existing").expect("write final");
    let anchor = DirectoryAnchorV8::open(&temporary.path).expect("open anchor");

    let error = rename_noreplace_at(
        &anchor,
        OsStr::new("incoming"),
        &anchor,
        OsStr::new("final"),
    )
    .expect_err("existing destination must fail closed");

    assert!(error.to_string().contains("RENAME_NOREPLACE"));
    assert_eq!(
        fs::read(temporary.path.join("incoming")).expect("read incoming"),
        b"new"
    );
    assert_eq!(
        fs::read(temporary.path.join("final")).expect("read final"),
        b"existing"
    );
}

#[test]
fn rejects_paths_instead_of_single_leaf_names() {
    let temporary = TestDirectory::create("leaf-validation");
    let anchor = DirectoryAnchorV8::open(&temporary.path).expect("open anchor");

    for invalid in ["", ".", "..", "nested/name"] {
        let error = rename_noreplace_at(&anchor, OsStr::new(invalid), &anchor, OsStr::new("final"))
            .expect_err("invalid leaf must fail closed");
        assert!(error.to_string().contains("leaf"));
    }
}
