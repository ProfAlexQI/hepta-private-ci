use std::fs;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use std::sync::Arc;
use std::sync::Barrier;

use serde::Serialize;
use static_assertions::assert_not_impl_any;

use crate::DurableReplayPublicationInspectionV1;
use crate::ReplayStoreAnchorV1;
use crate::ReplayStoreErrorV1;
use crate::ReplayStorePolicyV1;
use crate::open_production_replay_store;
use crate::open_replay_store;
use crate::store::publish_material_for_tests;

assert_not_impl_any!(ReplayStorePolicyV1: Clone, Copy, Serialize, serde::de::DeserializeOwned);
assert_not_impl_any!(ReplayStoreAnchorV1: Clone, Copy, Serialize, serde::de::DeserializeOwned);
assert_not_impl_any!(DurableReplayPublicationInspectionV1: Clone, Copy, Serialize, serde::de::DeserializeOwned);

#[test]
fn production_store_policy_blocks_before_any_caller_path_exists() {
    let error = open_production_replay_store().expect_err("production policy must remain absent");
    assert!(matches!(error, ReplayStoreErrorV1::Blocked(_)));
}

#[test]
fn exact_claim_is_published_read_back_and_never_removed_on_drop() {
    let fixture = StoreFixture::new("success").expect("success fixture");
    let store = fixture.open().expect("open success store");
    let slot = digest('a');
    let record = b"hepta replay record v1";
    let token = publish_material_for_tests(&store, &slot, record, store.identity_sha256())
        .expect("durable exact publication");
    assert_eq!(token.replay_slot_sha256(), slot);
    assert_eq!(token.store_identity_sha256(), store.identity_sha256());
    assert_eq!(token.retained_descriptor_count(), 3);
    assert!(token.exact_publication_read_back_observed());
    assert!(!token.crash_reboot_qualified());
    assert!(!token.authorizes_live());
    let final_path = fixture.namespace_path().join(format!("{slot}.claim-v1"));
    assert_eq!(fs::read(&final_path).expect("read final claim"), record);
    assert_eq!(
        fs::symlink_metadata(&final_path)
            .expect("final metadata")
            .permissions()
            .mode()
            & 0o777,
        0o600
    );
    drop(token);
    assert_eq!(
        fs::read(final_path).expect("claim survives token drop"),
        record
    );
}

#[test]
fn existing_final_blocks_exact_and_changed_replay_without_recovery() {
    let fixture = StoreFixture::new("final-replay").expect("final-replay fixture");
    let store = fixture.open().expect("open final-replay store");
    let slot = digest('b');
    publish_material_for_tests(&store, &slot, b"first", store.identity_sha256())
        .expect("first publication");
    for record in [b"first".as_slice(), b"changed".as_slice()] {
        let error = publish_material_for_tests(&store, &slot, record, store.identity_sha256())
            .expect_err("every existing final must block");
        assert!(matches!(
            error,
            ReplayStoreErrorV1::ExistingFinalBlocksReplay
        ));
    }
}

#[test]
fn incoming_residue_and_symlink_final_fail_closed() {
    let fixture = StoreFixture::new("residue").expect("residue fixture");
    let store = fixture.open().expect("open residue store");
    let residue_slot = digest('c');
    let residue_path = fixture
        .namespace_path()
        .join(format!("{residue_slot}.incoming-v1"));
    fs::write(&residue_path, b"partial").expect("write residue");
    fs::set_permissions(&residue_path, fs::Permissions::from_mode(0o600)).expect("chmod residue");
    let error =
        publish_material_for_tests(&store, &residue_slot, b"candidate", store.identity_sha256())
            .expect_err("incoming residue must block");
    assert!(matches!(error, ReplayStoreErrorV1::IncomingResidueBlocks));

    let symlink_slot = digest('d');
    let final_path = fixture
        .namespace_path()
        .join(format!("{symlink_slot}.claim-v1"));
    std::os::unix::fs::symlink("nonexistent-target", &final_path).expect("create final symlink");
    let error =
        publish_material_for_tests(&store, &symlink_slot, b"candidate", store.identity_sha256())
            .expect_err("symlink final must block");
    assert!(matches!(
        error,
        ReplayStoreErrorV1::ExistingFinalBlocksReplay
    ));
}

#[test]
fn root_namespace_mode_symlink_and_store_identity_mismatch_are_rejected() {
    let wrong_mode = StoreFixture::new("wrong-mode").expect("wrong-mode fixture");
    fs::set_permissions(&wrong_mode.root, fs::Permissions::from_mode(0o755))
        .expect("chmod wrong root");
    assert!(matches!(
        open_replay_store(&wrong_mode.policy()),
        Err(ReplayStoreErrorV1::IdentityMismatch(_))
    ));

    let symlinked = StoreFixture::new_without_namespace("symlink-namespace")
        .expect("symlink-namespace fixture");
    let target = symlinked.root.join("target");
    fs::create_dir(&target).expect("target directory");
    fs::set_permissions(&target, fs::Permissions::from_mode(0o700)).expect("chmod target");
    std::os::unix::fs::symlink("target", symlinked.namespace_path()).expect("namespace symlink");
    assert!(open_replay_store(&symlinked.policy()).is_err());

    let mismatch = StoreFixture::new("identity-mismatch").expect("identity-mismatch fixture");
    let store = mismatch.open().expect("open identity-mismatch store");
    let error = publish_material_for_tests(&store, &digest('e'), b"record", &digest('f'))
        .expect_err("wrong store identity must block");
    assert!(matches!(error, ReplayStoreErrorV1::IdentityMismatch(_)));
}

#[test]
fn concurrent_same_slot_publication_has_exactly_one_winner() {
    let fixture = StoreFixture::new("race").expect("race fixture");
    let store = Arc::new(fixture.open().expect("open race store"));
    let barrier = Arc::new(Barrier::new(3));
    let slot = digest('1');
    let mut joins = Vec::new();
    for record in [b"racer-one".to_vec(), b"racer-two".to_vec()] {
        let store = Arc::clone(&store);
        let barrier = Arc::clone(&barrier);
        let slot = slot.clone();
        joins.push(std::thread::spawn(move || {
            barrier.wait();
            publish_material_for_tests(&store, &slot, &record, store.identity_sha256())
        }));
    }
    barrier.wait();
    let results: Vec<_> = joins
        .into_iter()
        .map(|join| join.join().expect("race thread"))
        .collect();
    assert_eq!(results.iter().filter(|result| result.is_ok()).count(), 1);
    assert_eq!(results.iter().filter(|result| result.is_err()).count(), 1);
    let loser = results
        .into_iter()
        .find_map(Result::err)
        .expect("one losing publication");
    assert!(matches!(
        loser,
        ReplayStoreErrorV1::ExistingFinalBlocksReplay
            | ReplayStoreErrorV1::IncomingResidueBlocks
            | ReplayStoreErrorV1::RaceDetected(_)
    ));
}

struct StoreFixture {
    _temp: tempfile::TempDir,
    expected_gid: u32,
    expected_uid: u32,
    namespace: String,
    root: std::path::PathBuf,
}

impl StoreFixture {
    fn new(label: &str) -> std::io::Result<Self> {
        let fixture = Self::new_without_namespace(label)?;
        fs::create_dir(fixture.namespace_path())?;
        fs::set_permissions(fixture.namespace_path(), fs::Permissions::from_mode(0o700))?;
        Ok(fixture)
    }

    fn new_without_namespace(label: &str) -> std::io::Result<Self> {
        let temp = tempfile::tempdir()?;
        let root = temp.path().join(format!("store-{label}"));
        fs::create_dir(&root)?;
        fs::set_permissions(&root, fs::Permissions::from_mode(0o700))?;
        let metadata = fs::symlink_metadata(&root)?;
        Ok(Self {
            _temp: temp,
            expected_gid: metadata.gid(),
            expected_uid: metadata.uid(),
            namespace: "claims-v1".to_string(),
            root,
        })
    }

    fn namespace_path(&self) -> std::path::PathBuf {
        self.root.join(&self.namespace)
    }

    fn policy(&self) -> ReplayStorePolicyV1 {
        ReplayStorePolicyV1::for_tests(
            self.root.clone(),
            self.namespace.clone(),
            self.expected_uid,
            self.expected_gid,
        )
    }

    fn open(&self) -> crate::ReplayStoreResultV1<ReplayStoreAnchorV1> {
        open_replay_store(&self.policy())
    }
}

fn digest(character: char) -> String {
    character.to_string().repeat(64)
}
