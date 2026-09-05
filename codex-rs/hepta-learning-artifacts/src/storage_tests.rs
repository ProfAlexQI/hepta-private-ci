use super::*;
use std::fs;
use std::fs::OpenOptions;
use std::path::PathBuf;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;

static COUNTER: AtomicU64 = AtomicU64::new(0);

struct TestFile(PathBuf);
impl TestFile {
    fn new() -> Self {
        let sequence = COUNTER.fetch_add(1, Ordering::Relaxed);
        let process = std::process::id();
        let time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!("hepta-artifact-{process}-{time}-{sequence}"));
        OpenOptions::new()
            .read(true)
            .write(true)
            .create_new(true)
            .open(&path)
            .unwrap();
        Self(path)
    }
    fn open(&self) -> File {
        OpenOptions::new()
            .read(true)
            .write(true)
            .open(&self.0)
            .unwrap()
    }
}
impl Drop for TestFile {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.0);
    }
}

fn id(value: &str) -> StableId {
    StableId::new(value).unwrap()
}

fn register(registry: &mut ArtifactRegistry, name: &str, predecessor: Option<&str>, bytes: &[u8]) {
    registry
        .append(ArtifactEvent::Register {
            event_id: id(&format!("register-{name}")),
            manifest: ArtifactManifest {
                artifact_id: id(name),
                kind: ArtifactKind::Policy,
                generation: Generation::new(if predecessor.is_some() { 2 } else { 1 }).unwrap(),
                predecessor_id: predecessor.map(id),
                content_digest: Digest32::of_bytes(bytes),
                objective_digest: Digest32::of_bytes(b"objective"),
                support_digest: Digest32::of_bytes(b"dataset"),
                producer_id: id("generator"),
                compatibility_digest: Digest32::of_bytes(b"compatibility"),
                encoded_size_bytes: bytes.len() as u64,
            },
        })
        .unwrap();
}

fn binding() -> Digest32 {
    Digest32::of_bytes(b"host-authenticated-scope-fixture-not-a-credential")
}

#[test]
fn snapshot_reopens_exact_history_and_revoked_ancestors() {
    let mut registry = ArtifactRegistry::new();
    register(&mut registry, "old", None, b"old");
    register(&mut registry, "new", Some("old"), b"new");
    registry
        .append(ArtifactEvent::Revoke(StateChange {
            event_id: id("revoke"),
            artifact_id: id("old"),
            evaluator_id: id("independent"),
            reason_digest: Digest32::of_bytes(b"revoked-dataset"),
        }))
        .unwrap();
    let file = TestFile::new();
    let receipt = write_registry_snapshot(file.open(), &registry, binding()).unwrap();
    let recovered = read_registry_snapshot(file.open(), receipt).unwrap();
    assert_eq!(recovered.snapshot(), registry.snapshot());
    assert!(!recovered.is_eligible(&id("new")));
    assert!(!recovered.is_eligible(&id("old")));
}

#[test]
fn payload_reopens_and_rejects_wrong_bytes_before_writing() {
    let mut registry = ArtifactRegistry::new();
    register(&mut registry, "policy", None, b"policy-v1");
    let file = TestFile::new();
    assert_eq!(
        write_candidate_payload(file.open(), &registry, &id("policy"), b"wrong"),
        Err(ArtifactStorageError::PayloadMismatch)
    );
    assert_eq!(fs::metadata(&file.0).unwrap().len(), 0);
    write_candidate_payload(file.open(), &registry, &id("policy"), b"policy-v1").unwrap();
    assert_eq!(
        read_candidate_payload(file.open(), &registry, &id("policy")).unwrap(),
        b"policy-v1"
    );
}

#[test]
fn no_overwrite_and_cooperating_writer_fence() {
    let registry = ArtifactRegistry::new();
    let file = TestFile::new();
    let held = file.open();
    held.try_lock().unwrap();
    assert_eq!(
        write_registry_snapshot(file.open(), &registry, binding()),
        Err(ArtifactStorageError::Busy)
    );
    drop(held);
    write_registry_snapshot(file.open(), &registry, binding()).unwrap();
    let before = fs::read(&file.0).unwrap();
    assert_eq!(
        write_registry_snapshot(file.open(), &registry, binding()),
        Err(ArtifactStorageError::AlreadyExists)
    );
    assert_eq!(before, fs::read(&file.0).unwrap());
}

#[test]
fn every_truncation_and_wrong_external_witness_rejects_without_repair() {
    let mut registry = ArtifactRegistry::new();
    register(&mut registry, "policy", None, b"policy");
    let file = TestFile::new();
    let receipt = write_registry_snapshot(file.open(), &registry, binding()).unwrap();
    let full = fs::read(&file.0).unwrap();
    for cut in 0..full.len() {
        fs::write(&file.0, &full[..cut]).unwrap();
        assert!(read_registry_snapshot(file.open(), receipt).is_err());
        assert_eq!(fs::read(&file.0).unwrap(), &full[..cut]);
    }
    fs::write(&file.0, &full).unwrap();
    let wrong = RegistrySnapshotReceipt {
        head_digest: Digest32::of_bytes(b"wrong"),
        ..receipt
    };
    assert!(read_registry_snapshot(file.open(), wrong).is_err());
    assert_eq!(fs::read(&file.0).unwrap(), full);
}

#[test]
fn noncanonical_encoding_rejects_even_with_rehashed_file_receipt() {
    let registry = ArtifactRegistry::new();
    let file = TestFile::new();
    let mut receipt = write_registry_snapshot(file.open(), &registry, binding()).unwrap();
    let altered = format!("HEPTAR01\n{}\n00\n", binding()).into_bytes();
    receipt.file_digest = Digest32::of_bytes(&altered);
    receipt.encoded_bytes = altered.len();
    fs::write(&file.0, altered).unwrap();
    assert!(read_registry_snapshot(file.open(), receipt).is_err());
}

#[test]
fn revoked_payload_cannot_be_loaded_from_current_registry() {
    let mut registry = ArtifactRegistry::new();
    register(&mut registry, "policy", None, b"policy");
    let file = TestFile::new();
    write_candidate_payload(file.open(), &registry, &id("policy"), b"policy").unwrap();
    registry
        .append(ArtifactEvent::Revoke(StateChange {
            event_id: id("revoke"),
            artifact_id: id("policy"),
            evaluator_id: id("independent"),
            reason_digest: Digest32::of_bytes(b"reason"),
        }))
        .unwrap();
    assert_eq!(
        read_candidate_payload(file.open(), &registry, &id("policy")),
        Err(ArtifactStorageError::Unavailable)
    );
}

#[test]
fn payload_corruption_and_invalid_receipt_reject() {
    let mut registry = ArtifactRegistry::new();
    register(&mut registry, "policy", None, b"policy");
    let file = TestFile::new();
    write_candidate_payload(file.open(), &registry, &id("policy"), b"policy").unwrap();
    fs::write(&file.0, b"tamper").unwrap();
    assert_eq!(
        read_candidate_payload(file.open(), &registry, &id("policy")),
        Err(ArtifactStorageError::PayloadMismatch)
    );
    let invalid = RegistrySnapshotReceipt {
        binding: Digest32::ZERO,
        head_digest: Digest32::ZERO,
        file_digest: Digest32::ZERO,
        records: 0,
        encoded_bytes: 0,
    };
    assert!(matches!(
        read_registry_snapshot(file.open(), invalid),
        Err(ArtifactStorageError::InvalidReceipt)
    ));
}

#[test]
fn zero_binding_leaves_file_empty() {
    let file = TestFile::new();
    assert_eq!(
        write_registry_snapshot(file.open(), &ArtifactRegistry::new(), Digest32::ZERO),
        Err(ArtifactStorageError::InvalidBinding)
    );
    assert_eq!(fs::metadata(&file.0).unwrap().len(), 0);
}
