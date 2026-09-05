use super::*;
use std::fs;
use std::fs::OpenOptions;
use std::path::PathBuf;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;

use pretty_assertions::assert_eq;

static NEXT: AtomicU64 = AtomicU64::new(0);

fn must<T, E: fmt::Debug>(result: Result<T, E>) -> T {
    match result {
        Ok(value) => value,
        Err(error) => panic!("artifact lock fixture failed: {error:?}"),
    }
}

fn id(value: &str) -> StableId {
    must(StableId::new(value))
}

fn binding() -> Digest32 {
    Digest32::of_bytes(b"artifact-lock-fixture-scope")
}

struct Fixture {
    root: PathBuf,
}

impl Fixture {
    fn new() -> Self {
        let serial = NEXT.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "hepta-artifact-read-boundary-{}-{serial}",
            std::process::id()
        ));
        must(fs::create_dir(&root));
        Self { root }
    }

    fn create(&self, name: &str) -> File {
        must(
            OpenOptions::new()
                .read(true)
                .write(true)
                .create_new(true)
                .open(self.root.join(name)),
        )
    }

    fn read(&self, name: &str) -> File {
        must(File::open(self.root.join(name)))
    }

    fn write(&self, name: &str) -> File {
        must(
            OpenOptions::new()
                .read(true)
                .write(true)
                .open(self.root.join(name)),
        )
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

#[test]
fn readonly_handles_load_registry_and_payload() {
    let fixture = Fixture::new();
    let bytes = b"candidate-policy-fixture";
    let mut registry = ArtifactRegistry::new();
    must(registry.append(ArtifactEvent::Register {
        event_id: id("register-one"),
        manifest: ArtifactManifest {
            artifact_id: id("candidate-one"),
            kind: ArtifactKind::Policy,
            generation: must(Generation::new(/*value*/ 1)),
            predecessor_id: None,
            content_digest: Digest32::of_bytes(bytes),
            objective_digest: Digest32::of_bytes(b"objective"),
            support_digest: Digest32::of_bytes(b"support"),
            producer_id: id("generator"),
            compatibility_digest: Digest32::of_bytes(b"compatibility"),
            encoded_size_bytes: bytes.len() as u64,
        },
    }));
    let witness = must(write_registry_snapshot(
        fixture.create("registry"),
        &registry,
        binding(),
    ));
    must(write_candidate_payload(
        fixture.create("payload"),
        &registry,
        &id("candidate-one"),
        bytes,
    ));
    let reopened = must(read_registry_snapshot(fixture.read("registry"), witness));
    assert_eq!(reopened.snapshot(), registry.snapshot());
    assert_eq!(
        must(read_candidate_payload(
            fixture.read("payload"),
            &reopened,
            &id("candidate-one"),
        )),
        bytes
    );
}

#[test]
fn shared_read_does_not_unlock_other_reader_or_permit_writer() {
    let fixture = Fixture::new();
    let registry = ArtifactRegistry::new();
    let witness = must(write_registry_snapshot(
        fixture.create("registry"),
        &registry,
        binding(),
    ));
    let shared = fixture.read("registry");
    must(shared.try_lock_shared());
    assert_eq!(
        must(read_registry_snapshot(fixture.read("registry"), witness)).snapshot(),
        registry.snapshot()
    );
    assert_eq!(
        write_registry_snapshot(fixture.write("registry"), &registry, binding()),
        Err(ArtifactStorageError::Busy)
    );
    must(shared.unlock());
    let exclusive = fixture.write("registry");
    must(exclusive.try_lock());
    assert_eq!(
        read_registry_snapshot(fixture.read("registry"), witness).err(),
        Some(ArtifactStorageError::Busy)
    );
    must(exclusive.unlock());
    assert_eq!(
        write_registry_snapshot(fixture.write("registry"), &registry, binding()),
        Err(ArtifactStorageError::AlreadyExists)
    );
}

#[cfg(target_os = "linux")]
#[test]
fn write_guard_releases_lock_with_transient_duplicate_retained() {
    let fixture = Fixture::new();
    let file = fixture.create("registry");
    let transient = must(file.try_clone());
    let registry = ArtifactRegistry::new();
    let witness = must(write_registry_snapshot(file, &registry, binding()));
    assert_eq!(
        must(read_registry_snapshot(fixture.read("registry"), witness)).snapshot(),
        registry.snapshot()
    );
    let exclusive = fixture.write("registry");
    must(exclusive.try_lock());
    drop(transient);
    assert_eq!(
        read_registry_snapshot(fixture.read("registry"), witness).err(),
        Some(ArtifactStorageError::Busy)
    );
    must(exclusive.unlock());
}

#[cfg(target_os = "linux")]
#[test]
fn failed_read_releases_own_lock_with_transient_duplicate_retained() {
    let fixture = Fixture::new();
    let registry = ArtifactRegistry::new();
    let mut witness = must(write_registry_snapshot(
        fixture.create("registry"),
        &registry,
        binding(),
    ));
    witness.file_digest = Digest32::of_bytes(b"wrong-witness");
    let file = fixture.read("registry");
    let transient = must(file.try_clone());
    assert_eq!(
        read_registry_snapshot(file, witness).err(),
        Some(ArtifactStorageError::Corrupt)
    );
    let exclusive = fixture.write("registry");
    must(exclusive.try_lock());
    drop(transient);
    assert_eq!(
        read_registry_snapshot(fixture.read("registry"), witness).err(),
        Some(ArtifactStorageError::Busy)
    );
    must(exclusive.unlock());
}
