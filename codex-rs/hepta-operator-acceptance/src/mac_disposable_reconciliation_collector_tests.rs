use super::*;

use crate::mac_disposable_lifecycle::DisposableLifecycleEventV2;
use crate::mac_disposable_lifecycle::DisposableLifecycleJournalV2;
use crate::mac_disposable_lifecycle::fresh_absence_sha256;
use crate::mac_iomedia_identity::BackingObjectBindingV1;
use crate::mac_iomedia_identity::BackingPathComponentV1;
use crate::mac_iomedia_identity::DiskArbitrationPropertiesV2;
use crate::mac_iomedia_identity::IOMediaRegistryIdentityV1;
use crate::mac_iomedia_identity::IOMediaRegistryPropertiesV2;
use crate::mac_iomedia_identity::IOMediaRegistryProvenanceV2;
use crate::mac_iomedia_identity::IORegistryAncestorV1;
use crate::mac_iomedia_identity::RestartDiskImageBackingIdentityV3;
use crate::mac_iomedia_identity::RestartIOMediaObjectV3;
use crate::mac_iomedia_identity::capture_restart_disk_image_url_identity_for_test;
use crate::mac_iomedia_identity::restart_disk_image_backing_matches_prepared_v3;
use std::ffi::CString;
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::fs::PermissionsExt;
use std::os::unix::fs::symlink;
use std::process::Command;

static LIVE_COLLECTOR_TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

macro_rules! assert_not_impl_any {
    ($ty:ty: $($trait:path),+ $(,)?) => {
        const _: fn() = || {
            trait AmbiguousIfImpl<A> {
                fn marker() {}
            }
            impl<T: ?Sized> AmbiguousIfImpl<()> for T {}
            $(
                {
                    struct Invalid;
                    impl<T: ?Sized + $trait> AmbiguousIfImpl<Invalid> for T {}
                }
            )+
            let _ = <$ty as AmbiguousIfImpl<_>>::marker;
        };
    };
}

assert_not_impl_any!(
    RetainedCollectorObservationV3:
        Clone,
        Send,
        Sync,
        serde::Serialize,
        serde::de::DeserializeOwned,
        std::os::fd::AsRawFd,
        std::os::fd::AsFd,
        std::os::fd::IntoRawFd,
        From<File>,
        TryFrom<File>
);
assert_not_impl_any!(
    RetainedCollectorAppendV3<'static>:
        Clone,
        Send,
        Sync,
        serde::Serialize,
        serde::de::DeserializeOwned,
        std::os::fd::AsRawFd,
        std::os::fd::AsFd,
        std::os::fd::IntoRawFd,
        From<File>,
        TryFrom<File>
);

fn id(value: u64) -> String {
    format!("{value:016x}")
}

fn synthetic_backing(path: &str) -> DiskImageBackingIdentityV2 {
    let mut paths = vec!["/".to_string()];
    let mut current = PathBuf::from("/");
    for component in Path::new(path).components() {
        if let std::path::Component::Normal(component) = component {
            current.push(component);
            paths.push(current.to_str().unwrap().to_string());
        }
    }
    let last = paths.len() - 1;
    let opened_components = paths
        .into_iter()
        .enumerate()
        .map(|(index, component_path)| {
            let directory = index != last;
            let digest = sha256(component_path.as_bytes());
            let binding = BackingObjectBindingV1 {
                content_sha256: (!directory).then(|| digest.clone()),
                ctime_nanoseconds: 1,
                ctime_seconds: 1,
                dev: 1,
                flags: 0,
                gid: 1,
                inode: u64::from_str_radix(&digest[..16], 16).unwrap().max(1),
                mode: if directory {
                    libc::S_IFDIR as u32 | 0o700
                } else {
                    libc::S_IFREG as u32 | 0o600
                },
                mtime_nanoseconds: 1,
                mtime_seconds: 1,
                nlink: 1,
                size: if directory { 0 } else { 1 },
                uid: 1,
            };
            BackingPathComponentV1 {
                directory,
                fd_binding: binding.clone(),
                path: component_path,
                path_binding_after: binding.clone(),
                path_binding_before: binding,
            }
        })
        .collect();
    DiskImageBackingIdentityV2 {
        authority_granted: false,
        canonical_path: path.to_string(),
        opened_components,
        path_authority_granted: false,
        schema: "hepta_mac_disk_image_backing_identity_v2".to_string(),
    }
}

fn candidate(device: u64, path: &str) -> RestartDiskImageCandidateV3 {
    let prepared = synthetic_backing(path);
    let mut file_binding = prepared
        .opened_components
        .last()
        .unwrap()
        .fd_binding
        .clone();
    file_binding.content_sha256 = None;
    RestartDiskImageCandidateV3 {
        backing_identity: RestartDiskImageBackingIdentityV3 {
            authority_granted: false,
            canonical_path: path.to_string(),
            file_binding,
            schema: "hepta_mac_restart_disk_image_backing_identity_v3".to_string(),
        },
        canonical_backing_path: path.to_string(),
        disk_image_device: IORegistryAncestorV1 {
            class_name: "AppleDiskImageDevice".to_string(),
            registry_entry_id: id(device),
            registry_path: Some(format!("IOService:/AppleDiskImageDevice-{device}")),
        },
        disk_image_url: format!("file://{path}"),
        disk_image_url_path: path.to_string(),
    }
}

fn object(
    node_id: u64,
    bsd: &str,
    candidate: Option<RestartDiskImageCandidateV3>,
) -> RestartIOMediaObjectV3 {
    let node_ancestor = IORegistryAncestorV1 {
        class_name: "IOMedia".to_string(),
        registry_entry_id: id(node_id),
        registry_path: Some(format!("IOService:/IOMedia-{node_id}")),
    };
    let mut ancestry = vec![node_ancestor];
    if let Some(candidate) = &candidate {
        ancestry.push(candidate.disk_image_device.clone());
    }
    ancestry.push(IORegistryAncestorV1 {
        class_name: "IORegistryEntry".to_string(),
        registry_entry_id: id(0xf000 + node_id),
        registry_path: None,
    });
    RestartIOMediaObjectV3 {
        authority_granted: false,
        candidate,
        provenance: IOMediaRegistryProvenanceV2 {
            ancestry,
            authority_granted: false,
            bsd_name: bsd.to_string(),
            conforms_to_iomedia: true,
            disk_arbitration: DiskArbitrationPropertiesV2 {
                block_size: None,
                content: None,
                ejectable: None,
                internal: None,
                leaf: None,
                media_uuid: None,
                removable: None,
                size: None,
                whole: None,
                writable: None,
            },
            iomedia: IOMediaRegistryPropertiesV2 {
                content: None,
                ejectable: None,
                leaf: None,
                preferred_block_size: None,
                removable: None,
                size: None,
                whole: None,
                writable: None,
            },
            registry_entry_id: id(node_id),
            registry_path: format!("IOService:/IOMedia-{node_id}"),
            whole_disk: IOMediaRegistryIdentityV1 {
                authority_granted: false,
                bsd_name: bsd.to_string(),
                registry_entry_id: id(node_id),
                schema: "hepta_mac_iomedia_registry_identity_v1".to_string(),
            },
            schema: "hepta_mac_iomedia_registry_provenance_v2".to_string(),
        },
    }
}

fn inventory(objects: Vec<RestartIOMediaObjectV3>) -> RestartIOMediaInventoryV3 {
    RestartIOMediaInventoryV3 {
        authority_granted: false,
        boot_session_uuid: "12345678-1234-4234-8234-123456789abc".to_string(),
        objects,
        schema: "hepta_mac_restart_iomedia_inventory_v3".to_string(),
    }
}

fn backing_artifact(basename: &str) -> PreparedArtifactBindingV3 {
    PreparedArtifactBindingV3::new(ArtifactRoleV3::BackingImage, basename).unwrap()
}

fn synthetic_policy() -> RestartCollectorPolicyV3 {
    RestartCollectorPolicyV3 {
        artifacts: vec![backing_artifact("created.img")],
        artifact_root: "/private/tmp/hepta-artifacts".to_string(),
        artifact_root_identity: StableDirectoryIdentityV3 {
            birthtime_nanoseconds: 1,
            birthtime_seconds: 1,
            dev: 1,
            flags: 0,
            generation: 1,
            gid: 1,
            inode: 2,
            mode: libc::S_IFDIR as u32 | 0o700,
            nlink: 1,
            roster_entries: 0,
            uid: 1,
        },
        authority: DisposableAuthorityV2::none(),
        backing_path: "/private/tmp/hepta.img".to_string(),
        max_iomedia_objects: 256,
        max_mount_entries: MAX_MOUNT_ENTRIES,
        mountpoint: "/private/tmp/hepta-mount".to_string(),
        protected_roots: vec![
            "/private/tmp/hepta-artifacts".to_string(),
            "/private/tmp/hepta-receipts".to_string(),
        ],
        receipt_root: "/private/tmp/hepta-receipts".to_string(),
        receipt_root_identity: StableDirectoryIdentityV3 {
            birthtime_nanoseconds: 1,
            birthtime_seconds: 1,
            dev: 1,
            flags: 0,
            generation: 1,
            gid: 1,
            inode: 3,
            mode: libc::S_IFDIR as u32 | 0o700,
            nlink: 1,
            roster_entries: 0,
            uid: 1,
        },
        schema: POLICY_SCHEMA.to_string(),
    }
}

fn mount(from: &str, on: &str) -> MountBindingV3 {
    MountBindingV3 {
        filesystem_id: [1, 2],
        filesystem_type: "apfs".to_string(),
        mount_flags: 0,
        mount_from: from.to_string(),
        mount_on: on.to_string(),
    }
}

struct LiveCollectorFixture {
    _fixture: tempfile::TempDir,
    _persistence: tempfile::TempDir,
    artifact_root: PathBuf,
    baseline: RestartBaselineInventoryV3,
    bindings: RestartCollectorBindingsV3,
    mountpoint_identity: MountpointIdentityV3,
    persistence_root: PathBuf,
    policy: RestartCollectorPolicyV3,
    prepared_backing: DiskImageBackingIdentityV2,
}

impl LiveCollectorFixture {
    fn new() -> Self {
        let fixture = tempfile::Builder::new()
            .prefix(".restart-collector-redteam-fixture-")
            .tempdir_in(env!("CARGO_MANIFEST_DIR"))
            .unwrap();
        let persistence = tempfile::Builder::new()
            .prefix(".restart-collector-redteam-receipts-")
            .tempdir_in(env!("CARGO_MANIFEST_DIR"))
            .unwrap();
        let fixture_root = std::fs::canonicalize(fixture.path()).unwrap();
        let persistence_root = std::fs::canonicalize(persistence.path()).unwrap();
        let backing_path = fixture_root.join("prepared.img");
        let mountpoint = fixture_root.join("mountpoint");
        let artifact_root = fixture_root.join("artifacts");
        std::fs::write(&backing_path, b"rootless-redteam-backing-v3").unwrap();
        std::fs::create_dir(&mountpoint).unwrap();
        std::fs::create_dir(&artifact_root).unwrap();
        let prepared_backing = capture_live_backing_identity_v2(&backing_path).unwrap();
        let mountpoint_identity = MountpointIdentityV3::capture(&mountpoint).unwrap();
        let baseline = capture_live_restart_baseline_v3().unwrap();
        let artifacts = [backing_artifact("operation-created.img")];
        let policy = RestartCollectorPolicyV3::new(
            &backing_path,
            &mountpoint,
            &artifact_root,
            &persistence_root,
            &artifacts,
            &[],
        )
        .unwrap();
        let bindings = RestartCollectorBindingsV3 {
            backing_identity_sha256: sha256(&canonical_json(&prepared_backing).unwrap()),
            baseline_inventory_sha256: baseline.sha256().unwrap(),
            boot_session_uuid: baseline.boot_session_uuid.clone(),
            collector_policy_sha256: policy.sha256().unwrap(),
            mountpoint_underlying_sha256: mountpoint_identity.sha256().unwrap(),
            operation_nonce: "5".repeat(64),
            restart_epoch_nonce: "6".repeat(64),
            restart_started_monotonic_nanoseconds: monotonic_nanoseconds().unwrap() - 1,
        };
        Self {
            _fixture: fixture,
            _persistence: persistence,
            artifact_root,
            baseline,
            bindings,
            mountpoint_identity,
            persistence_root,
            policy,
            prepared_backing,
        }
    }

    fn request(&self) -> LiveRestartCollectorRequestV3<'_> {
        LiveRestartCollectorRequestV3 {
            artifact_root: &self.artifact_root,
            baseline: &self.baseline,
            bindings: &self.bindings,
            mountpoint_identity: &self.mountpoint_identity,
            policy: &self.policy,
            prepared_backing: &self.prepared_backing,
            receipt_root: &self.persistence_root,
        }
    }
}

fn write_private(path: &Path, bytes: &[u8]) {
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .mode(0o600)
        .open(path)
        .unwrap();
    file.write_all(bytes).unwrap();
    file.sync_all().unwrap();
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600)).unwrap();
}

fn make_test_fifo(path: &Path) {
    let path = CString::new(path.as_os_str().as_encoded_bytes()).unwrap();
    assert_eq!(unsafe { libc::mkfifo(path.as_ptr(), 0o600) }, 0);
}

fn set_test_xattr(path: &Path) {
    let path = CString::new(path.as_os_str().as_encoded_bytes()).unwrap();
    let name = c"com.hepta.restart-collector-test";
    assert_eq!(
        unsafe { libc::setxattr(path.as_ptr(), name.as_ptr(), b"x".as_ptr().cast(), 1, 0, 0) },
        0
    );
}

fn remove_test_xattr(path: &Path) {
    let path = CString::new(path.as_os_str().as_encoded_bytes()).unwrap();
    let name = c"com.hepta.restart-collector-test";
    assert_eq!(
        unsafe { libc::removexattr(path.as_ptr(), name.as_ptr(), 0) },
        0
    );
}

fn set_test_acl(path: &Path) {
    assert!(
        Command::new("/bin/chmod")
            .args(["+a", "everyone deny delete"])
            .arg(path)
            .status()
            .unwrap()
            .success()
    );
}

fn remove_test_acl(path: &Path) {
    assert!(
        Command::new("/bin/chmod")
            .arg("-N")
            .arg(path)
            .status()
            .unwrap()
            .success()
    );
}

fn valid_snapshot() -> ReconciliationSnapshotV2 {
    ReconciliationSnapshotV2 {
        backing_identity_sha256: "a".repeat(64),
        boot_session_uuid: "12345678-1234-4234-8234-123456789abc".to_string(),
        collector_policy_sha256: "b".repeat(64),
        collector_receipt_sha256: "c".repeat(64),
        iomedia_evidence_sha256: "d".repeat(64),
        match_result: ReconciliationMatchV2::Zero,
        monotonic_after_nanoseconds: 2,
        monotonic_before_nanoseconds: 1,
        mount_evidence_sha256: "e".repeat(64),
        mountpoint_underlying_sha256: "f".repeat(64),
        operation_nonce: "1".repeat(64),
        restart_epoch_nonce: "2".repeat(64),
    }
}

#[test]
fn synthetic_table_classifies_zero_unique_and_ambiguous_by_device_and_backing() {
    let path = "/private/tmp/hepta.img";
    let prepared = synthetic_backing(path);
    let zero = inventory(vec![object(1, "disk1", None)]);
    assert!(
        classify_matching_groups(&zero, &prepared)
            .unwrap()
            .is_empty()
    );

    let unique = inventory(vec![
        object(1, "disk10", Some(candidate(100, path))),
        object(2, "disk10s1", Some(candidate(100, path))),
    ]);
    let groups = classify_matching_groups(&unique, &prepared).unwrap();
    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0].member_bsd_names, ["disk10", "disk10s1"]);
    assert_eq!(
        classify_mount_state(&groups, &[], &synthetic_policy()).unwrap(),
        (ReconciliationMatchV2::Unique { mounted: false }, false)
    );
    assert_eq!(
        classify_mount_state(
            &groups,
            &[mount("/dev/disk10s1", "/private/tmp/hepta-mount")],
            &synthetic_policy(),
        )
        .unwrap(),
        (ReconciliationMatchV2::Unique { mounted: true }, true)
    );

    let ambiguous = inventory(vec![
        object(1, "disk10", Some(candidate(100, path))),
        object(2, "disk11", Some(candidate(101, path))),
    ]);
    let groups = classify_matching_groups(&ambiguous, &prepared).unwrap();
    assert_eq!(groups.len(), 2);
    assert_eq!(
        classify_mount_state(&groups, &[], &synthetic_policy())
            .unwrap()
            .0,
        ReconciliationMatchV2::Ambiguous {
            matching_objects: 2
        }
    );
    assert!(groups.iter().all(|group| {
        !group.candidate.disk_image_device.class_name.is_empty()
            && group.member_registry_entry_ids.len() == 1
    }));
}

#[test]
fn synthetic_table_rejects_device_url_and_mount_policy_drift() {
    let path = "/private/tmp/hepta.img";
    let prepared = synthetic_backing(path);
    let inconsistent = inventory(vec![
        object(1, "disk10", Some(candidate(100, path))),
        object(
            2,
            "disk10s1",
            Some(candidate(100, "/private/tmp/other.img")),
        ),
    ]);
    assert!(classify_matching_groups(&inconsistent, &prepared).is_err());

    let unique = inventory(vec![object(1, "disk10", Some(candidate(100, path)))]);
    let groups = classify_matching_groups(&unique, &prepared).unwrap();
    assert!(
        classify_mount_state(
            &groups,
            &[mount("/dev/disk10", "/private/tmp/wrong")],
            &synthetic_policy(),
        )
        .is_err()
    );
    assert!(
        classify_mount_state(
            &groups,
            &[mount("/dev/disk999", "/private/tmp/hepta-mount")],
            &synthetic_policy(),
        )
        .is_err()
    );
    assert!(
        reject_nested_mounts(
            &[mount("devfs", "/private/tmp/hepta-artifacts/nested")],
            &synthetic_policy(),
        )
        .is_err()
    );
}

#[test]
fn synthetic_inventory_rejects_ambiguous_apple_disk_image_ancestry() {
    let path = "/private/tmp/hepta.img";
    let mut node = object(1, "disk10", Some(candidate(100, path)));
    node.provenance.ancestry.insert(
        1,
        IORegistryAncestorV1 {
            class_name: "AppleDiskImageDevice".to_string(),
            registry_entry_id: id(101),
            registry_path: Some("IOService:/AppleDiskImageDevice-101".to_string()),
        },
    );
    assert!(validate_restart_iomedia_inventory_v3(&inventory(vec![node])).is_err());
}

#[test]
fn snapshot_shape_validator_rejects_every_forged_boundary() {
    let snapshot = valid_snapshot();
    validate_reconciliation_snapshot_shape_v3(&snapshot).unwrap();

    let mut forged = snapshot.clone();
    forged.backing_identity_sha256 = "bad".to_string();
    assert!(validate_reconciliation_snapshot_shape_v3(&forged).is_err());
    let mut forged = snapshot.clone();
    forged.boot_session_uuid = forged.boot_session_uuid.to_ascii_uppercase();
    assert!(validate_reconciliation_snapshot_shape_v3(&forged).is_err());
    let mut forged = snapshot.clone();
    forged.collector_policy_sha256 = "bad".to_string();
    assert!(validate_reconciliation_snapshot_shape_v3(&forged).is_err());
    let mut forged = snapshot.clone();
    forged.collector_receipt_sha256 = "bad".to_string();
    assert!(validate_reconciliation_snapshot_shape_v3(&forged).is_err());
    let mut forged = snapshot.clone();
    forged.iomedia_evidence_sha256 = "bad".to_string();
    assert!(validate_reconciliation_snapshot_shape_v3(&forged).is_err());
    let mut forged = snapshot.clone();
    forged.mount_evidence_sha256 = "bad".to_string();
    assert!(validate_reconciliation_snapshot_shape_v3(&forged).is_err());
    let mut forged = snapshot.clone();
    forged.mountpoint_underlying_sha256 = "bad".to_string();
    assert!(validate_reconciliation_snapshot_shape_v3(&forged).is_err());
    let mut forged = snapshot.clone();
    forged.operation_nonce = "0".repeat(64);
    assert!(validate_reconciliation_snapshot_shape_v3(&forged).is_err());
    let mut forged = snapshot.clone();
    forged.restart_epoch_nonce = "0".repeat(64);
    assert!(validate_reconciliation_snapshot_shape_v3(&forged).is_err());
    let mut forged = snapshot.clone();
    forged.monotonic_before_nanoseconds = 0;
    assert!(validate_reconciliation_snapshot_shape_v3(&forged).is_err());
    let mut forged = snapshot;
    forged.match_result = ReconciliationMatchV2::Ambiguous {
        matching_objects: 1,
    };
    assert!(validate_reconciliation_snapshot_shape_v3(&forged).is_err());
}

#[test]
fn policy_rejects_roots_or_backing_that_can_disappear_under_the_target_mount() {
    let fixture = tempfile::tempdir().unwrap();
    let root = std::fs::canonicalize(fixture.path()).unwrap();
    let backing = root.join("prepared.img");
    let mountpoint = root.join("mountpoint");
    let artifact = root.join("artifacts");
    let receipts = root.join("receipts");
    let artifact_under_mount = mountpoint.join("artifacts");
    let receipts_under_mount = mountpoint.join("receipts");
    let protected_under_mount = mountpoint.join("protected");
    let receipts_under_artifact = artifact.join("receipts");
    let backing_under_mount = mountpoint.join("prepared.img");
    let backing_under_artifact = artifact.join("prepared.img");
    std::fs::write(&backing, b"backing").unwrap();
    std::fs::create_dir(&mountpoint).unwrap();
    for directory in [
        &artifact,
        &receipts,
        &artifact_under_mount,
        &receipts_under_mount,
        &protected_under_mount,
        &receipts_under_artifact,
    ] {
        std::fs::create_dir(directory).unwrap();
    }
    std::fs::write(&backing_under_mount, b"covered backing").unwrap();
    std::fs::write(&backing_under_artifact, b"artifact backing").unwrap();
    let artifacts = [backing_artifact("operation-created.img")];

    let missing =
        RestartCollectorPolicyV3::new(&backing, &mountpoint, &artifact, &receipts, &[], &[])
            .unwrap_err();
    assert!(
        missing
            .to_string()
            .contains("exactly one prepared BackingImage")
    );
    let extra = [
        backing_artifact("operation-created.img"),
        PreparedArtifactBindingV3::new(ArtifactRoleV3::EffectIssueRecord, "effect-issue.json")
            .unwrap(),
    ];
    assert!(
        RestartCollectorPolicyV3::new(&backing, &mountpoint, &artifact, &receipts, &extra, &[],)
            .is_err()
    );
    let duplicate_role = [
        backing_artifact("operation-created.img"),
        backing_artifact("operation-created-copy.img"),
    ];
    assert!(
        RestartCollectorPolicyV3::new(
            &backing,
            &mountpoint,
            &artifact,
            &receipts,
            &duplicate_role,
            &[],
        )
        .is_err()
    );
    let alias = [
        backing_artifact("operation-created.img"),
        PreparedArtifactBindingV3::new(
            ArtifactRoleV3::MountpointUnderlying,
            "operation-created.img",
        )
        .unwrap(),
    ];
    assert!(
        RestartCollectorPolicyV3::new(&backing, &mountpoint, &artifact, &receipts, &alias, &[],)
            .is_err()
    );
    assert!(
        serde_json::from_slice::<PreparedArtifactBindingV3>(
            br#"{"basename":"operation-created.img","role":"unknown_role"}"#,
        )
        .is_err()
    );

    assert!(
        RestartCollectorPolicyV3::new(
            &backing,
            &mountpoint,
            &artifact_under_mount,
            &receipts,
            &artifacts,
            &[],
        )
        .is_err()
    );
    assert!(
        RestartCollectorPolicyV3::new(
            &backing_under_artifact,
            &mountpoint,
            &artifact,
            &receipts,
            &artifacts,
            &[],
        )
        .is_err()
    );
    assert!(
        RestartCollectorPolicyV3::new(
            &backing,
            &mountpoint,
            &artifact,
            &receipts_under_mount,
            &artifacts,
            &[],
        )
        .is_err()
    );
    assert!(
        RestartCollectorPolicyV3::new(
            &backing,
            &mountpoint,
            &artifact,
            &receipts,
            &artifacts,
            &[&protected_under_mount],
        )
        .is_err()
    );
    assert!(
        RestartCollectorPolicyV3::new(
            &backing_under_mount,
            &mountpoint,
            &artifact,
            &receipts,
            &artifacts,
            &[],
        )
        .is_err()
    );
    assert!(
        RestartCollectorPolicyV3::new(
            &backing,
            &mountpoint,
            &artifact,
            &receipts_under_artifact,
            &artifacts,
            &[],
        )
        .is_err()
    );
    let too_many_protected = vec![artifact.as_path(); MAX_PROTECTED_ROOTS.saturating_sub(1)];
    assert!(
        RestartCollectorPolicyV3::new(
            &backing,
            &mountpoint,
            &artifact,
            &receipts,
            &artifacts,
            &too_many_protected,
        )
        .is_err()
    );
    assert_eq!(
        checked_receipt_aggregate_bytes(0, MAX_RECEIPT_AGGREGATE_BYTES).unwrap(),
        MAX_RECEIPT_AGGREGATE_BYTES
    );
    assert!(checked_receipt_aggregate_bytes(MAX_RECEIPT_AGGREGATE_BYTES, 1).is_err());
}

#[test]
fn rootless_live_collection_rejects_same_path_prepared_root_replacement() {
    let _lock = LIVE_COLLECTOR_TEST_LOCK
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);

    let fixture = LiveCollectorFixture::new();
    let held_artifact_root = HeldDirectoryV3::capture(&fixture.artifact_root, "test artifact root")
        .expect("capture prepared artifact root");
    let artifact_roster =
        list_directory(held_artifact_root.file.as_raw_fd(), MAX_ARTIFACT_ENTRIES).unwrap();
    let mut forged_generation = held_artifact_root.binding;
    forged_generation.generation = forged_generation.generation.wrapping_add(1);
    assert!(
        !fixture
            .policy
            .artifact_root_identity
            .matches_binding(&forged_generation, artifact_roster.len())
    );
    let mut forged_birth = held_artifact_root.binding;
    forged_birth.birthtime_nanoseconds = (forged_birth.birthtime_nanoseconds + 1) % 1_000_000_000;
    assert!(
        !fixture
            .policy
            .artifact_root_identity
            .matches_binding(&forged_birth, artifact_roster.len())
    );
    drop(held_artifact_root);
    let displaced_artifact_root = fixture
        .artifact_root
        .parent()
        .unwrap()
        .join("prepared-artifact-root-original");
    std::fs::rename(&fixture.artifact_root, &displaced_artifact_root).unwrap();
    std::fs::create_dir(&fixture.artifact_root).unwrap();
    let error = match collect_reconciliation_snapshot_v3(fixture.request()) {
        Err(error) => error,
        Ok(_) => panic!("same-path artifact-root replacement was accepted"),
    };
    assert!(
        error
            .to_string()
            .contains("policy artifact root differs from its prepared stable identity"),
        "{error}"
    );
    std::fs::remove_dir(&fixture.artifact_root).unwrap();
    std::fs::rename(&displaced_artifact_root, &fixture.artifact_root).unwrap();
    drop(fixture);

    let fixture = LiveCollectorFixture::new();
    let displaced_receipt_root = fixture.persistence_root.parent().unwrap().join(format!(
        "{}.prepared-root-original",
        fixture
            .persistence_root
            .file_name()
            .unwrap()
            .to_string_lossy()
    ));
    std::fs::rename(&fixture.persistence_root, &displaced_receipt_root).unwrap();
    std::fs::create_dir(&fixture.persistence_root).unwrap();
    let error = match collect_reconciliation_snapshot_v3(fixture.request()) {
        Err(error) => error,
        Ok(_) => panic!("same-path receipt-root replacement was accepted"),
    };
    assert!(
        error
            .to_string()
            .contains("policy receipt root differs from its prepared stable identity"),
        "{error}"
    );
    std::fs::remove_dir(&fixture.persistence_root).unwrap();
    std::fs::rename(&displaced_receipt_root, &fixture.persistence_root).unwrap();
}

#[test]
fn retained_observation_keeps_live_evidence_after_persistence() {
    let _lock = LIVE_COLLECTOR_TEST_LOCK
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);

    let fixture = LiveCollectorFixture::new();
    let retained = collect_reconciliation_snapshot_v3(fixture.request())
        .unwrap()
        .persist_and_retain()
        .unwrap();
    retained.revalidate().unwrap();
    {
        let append = retained.append_capability().unwrap();
        assert_eq!(append.operation_nonce(), fixture.bindings.operation_nonce);
    }
    assert!(retained.revalidate_bound().is_err());

    let late_artifact = fixture
        .artifact_root
        .join(&fixture.policy.artifacts[0].basename);
    write_private(&late_artifact, b"late operation artifact");
    assert!(retained.revalidate().is_err());
}

#[test]
fn deferred_underlying_mountpoint_guard_reopens_only_the_prepared_child() {
    let _lock = LIVE_COLLECTOR_TEST_LOCK
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let fixture = tempfile::Builder::new()
        .prefix(".deferred-mountpoint-guard-")
        .tempdir_in(env!("CARGO_MANIFEST_DIR"))
        .unwrap();
    let root = std::fs::canonicalize(fixture.path()).unwrap();
    let parent = root.join("mount-parent");
    let mountpoint = parent.join("mountpoint");
    std::fs::create_dir(&parent).unwrap();
    std::fs::create_dir(&mountpoint).unwrap();
    let expected = MountpointIdentityV3::capture(&mountpoint).unwrap();
    let guard = UnderlyingMountpointGuardV3::capture_deferred(&expected).unwrap();
    guard.revalidate().unwrap();
    let reopened = guard.reopen_underlying_after_unmount().unwrap();
    assert_eq!(mountpoint_identity_from_held(&reopened).unwrap(), expected);
    drop(reopened);

    let displaced = parent.join("original-mountpoint");
    std::fs::rename(&mountpoint, &displaced).unwrap();
    std::fs::create_dir(&mountpoint).unwrap();
    assert!(guard.reopen_underlying_after_unmount().is_err());
}

#[test]
fn live_disk_image_url_symlink_alias_matches_the_prepared_held_file() {
    let _lock = LIVE_COLLECTOR_TEST_LOCK
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    // The production backing guard intentionally retains and replays every
    // ancestor. Keep this fixture out of the shared TMPDIR, whose metadata is
    // legitimately churned by unrelated parallel tests.
    let fixture = tempfile::Builder::new()
        .prefix(".disk-image-url-alias-")
        .tempdir_in(env!("CARGO_MANIFEST_DIR"))
        .unwrap();
    let root = std::fs::canonicalize(fixture.path()).unwrap();
    let target = root.join("prepared.img");
    let other = root.join("other.img");
    let alias = root.join("alias.img");
    let fifo = root.join("blocking.img");
    std::fs::write(&target, b"same live backing").unwrap();
    std::fs::write(&other, b"different live backing").unwrap();
    symlink(&target, &alias).unwrap();
    let prepared = capture_live_backing_identity_v2(&target).unwrap();
    let alias_url = format!("file://{}", alias.to_str().unwrap());
    let alias_identity = capture_restart_disk_image_url_identity_for_test(&alias_url).unwrap();
    assert_eq!(
        alias_identity.canonical_path,
        target.to_str().unwrap().to_string()
    );
    assert!(restart_disk_image_backing_matches_prepared_v3(&alias_identity, &prepared).unwrap());
    let mut alias_candidate = candidate(100, target.to_str().unwrap());
    alias_candidate.backing_identity = alias_identity;
    alias_candidate.canonical_backing_path = target.to_str().unwrap().to_string();
    alias_candidate.disk_image_url = alias_url;
    alias_candidate.disk_image_url_path = alias.to_str().unwrap().to_string();
    let groups = classify_matching_groups(
        &inventory(vec![object(1, "disk10", Some(alias_candidate))]),
        &prepared,
    )
    .unwrap();
    assert_eq!(groups.len(), 1);

    std::fs::remove_file(&alias).unwrap();
    symlink(&other, &alias).unwrap();
    let other_identity = capture_restart_disk_image_url_identity_for_test(&format!(
        "file://{}",
        alias.to_str().unwrap()
    ))
    .unwrap();
    assert!(!restart_disk_image_backing_matches_prepared_v3(&other_identity, &prepared).unwrap());

    make_test_fifo(&fifo);
    assert!(capture_live_backing_identity_v2(&fifo).is_err());
    assert!(
        capture_restart_disk_image_url_identity_for_test(&format!(
            "file://{}",
            fifo.to_str().unwrap()
        ))
        .is_err()
    );
}

#[test]
fn rootless_closed_world_receipts_prior_projection_and_metadata_are_fail_closed() {
    let _lock = LIVE_COLLECTOR_TEST_LOCK
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let fixture = LiveCollectorFixture::new();

    let garbage = fixture.persistence_root.join("garbage");
    write_private(&garbage, b"garbage");
    assert!(collect_reconciliation_snapshot_v3(fixture.request()).is_err());
    std::fs::remove_file(&garbage).unwrap();

    let named_garbage = fixture
        .persistence_root
        .join(format!("collector-{}.json", "9".repeat(64)));
    write_private(&named_garbage, b"garbage with a receipt-shaped name");
    assert!(collect_reconciliation_snapshot_v3(fixture.request()).is_err());
    std::fs::remove_file(&named_garbage).unwrap();

    let receipt_budget = (0..MAX_RECEIPT_FILES)
        .map(|index| {
            fixture
                .persistence_root
                .join(format!("collector-{index:064x}.json"))
        })
        .collect::<Vec<_>>();
    for path in &receipt_budget {
        write_private(path, b"");
    }
    assert!(collect_reconciliation_snapshot_v3(fixture.request()).is_err());
    for path in &receipt_budget {
        std::fs::remove_file(path).unwrap();
    }

    let directory = fixture
        .persistence_root
        .join(format!("collector-{}.json", "a".repeat(64)));
    std::fs::create_dir(&directory).unwrap();
    assert!(collect_reconciliation_snapshot_v3(fixture.request()).is_err());
    std::fs::remove_dir(&directory).unwrap();

    let fifo = fixture
        .persistence_root
        .join(format!("collector-{}.json", "c".repeat(64)));
    make_test_fifo(&fifo);
    assert!(collect_reconciliation_snapshot_v3(fixture.request()).is_err());
    std::fs::remove_file(&fifo).unwrap();

    let symlink_target = fixture.artifact_root.join("receipt-symlink-target");
    write_private(&symlink_target, b"not a receipt");
    let symlink_receipt = fixture
        .persistence_root
        .join(format!("collector-{}.json", "b".repeat(64)));
    symlink(&symlink_target, &symlink_receipt).unwrap();
    assert!(collect_reconciliation_snapshot_v3(fixture.request()).is_err());
    std::fs::remove_file(&symlink_receipt).unwrap();
    std::fs::remove_file(&symlink_target).unwrap();

    set_test_xattr(&fixture.persistence_root);
    assert!(collect_reconciliation_snapshot_v3(fixture.request()).is_err());
    remove_test_xattr(&fixture.persistence_root);
    set_test_acl(&fixture.persistence_root);
    assert!(collect_reconciliation_snapshot_v3(fixture.request()).is_err());
    remove_test_acl(&fixture.persistence_root);

    let incoming = fixture
        .persistence_root
        .join(".incoming-collector-redteam-crash");
    write_private(&incoming, b"uncertain");
    assert!(collect_reconciliation_snapshot_v3(fixture.request()).is_err());
    std::fs::remove_file(&incoming).unwrap();

    let collision = collect_reconciliation_snapshot_v3(fixture.request()).unwrap();
    let collision_bytes = canonical_json(collision.receipt()).unwrap();
    let collision_path = fixture
        .persistence_root
        .join(format!("collector-{}.json", sha256(&collision_bytes)));
    write_private(&collision_path, &collision_bytes);
    assert!(collision.persist_and_retain().is_err());
    std::fs::remove_file(&collision_path).unwrap();

    let transient = collect_reconciliation_snapshot_v3(fixture.request()).unwrap();
    let transient_path = fixture.persistence_root.join("transient-create-delete");
    assert!(
        transient
            .persist_and_retain_with_hook(|| {
                std::fs::write(&transient_path, b"transient")?;
                std::fs::remove_file(&transient_path)?;
                Ok(())
            })
            .is_err()
    );

    let metadata_drift = collect_reconciliation_snapshot_v3(fixture.request()).unwrap();
    let metadata_path = fixture.persistence_root.join(format!(
        "collector-{}.json",
        sha256(&canonical_json(metadata_drift.receipt()).unwrap())
    ));
    assert!(
        metadata_drift
            .persist_and_retain_with_hook(|| {
                set_test_xattr(&metadata_path);
                remove_test_xattr(&metadata_path);
                Ok(())
            })
            .is_err()
    );

    let pending = collect_reconciliation_snapshot_v3(fixture.request()).unwrap();
    let snapshot_receipt_model = pending.receipt().clone();
    let snapshot_receipt_bytes = canonical_json(pending.receipt()).unwrap();
    let retained_snapshot = pending.persist_and_retain().unwrap();
    retained_snapshot.revalidate().unwrap();
    let snapshot = match retained_snapshot.observation_for_test() {
        FinalizedRestartObservationV3::ReconciliationSnapshot(snapshot) => snapshot.clone(),
        FinalizedRestartObservationV3::FreshAbsence(_) => panic!("wrong observation"),
    };
    let snapshot_receipt = fixture.persistence_root.join(format!(
        "collector-{}.json",
        snapshot.collector_receipt_sha256
    ));

    let mut cross_baseline_receipt = snapshot_receipt_model;
    cross_baseline_receipt.baseline_inventory.registry_entry_ids =
        if cross_baseline_receipt.baseline_inventory.registry_entry_ids == ["0000000000000001"] {
            vec!["0000000000000002".to_string()]
        } else {
            vec!["0000000000000001".to_string()]
        };
    cross_baseline_receipt.baseline_inventory_sha256 =
        cross_baseline_receipt.baseline_inventory.sha256().unwrap();
    cross_baseline_receipt.baseline_restored = false;
    validate_receipt(&cross_baseline_receipt).unwrap();
    let cross_baseline_bytes = canonical_json(&cross_baseline_receipt).unwrap();
    let cross_baseline_sha = sha256(&cross_baseline_bytes);
    let cross_baseline_snapshot =
        reconciliation_snapshot_from_receipt(&cross_baseline_receipt, &cross_baseline_sha).unwrap();
    let cross_baseline_path = fixture
        .persistence_root
        .join(format!("collector-{cross_baseline_sha}.json"));
    write_private(&cross_baseline_path, &cross_baseline_bytes);
    assert!(collect_fresh_absence_v3(fixture.request(), &cross_baseline_snapshot).is_err());
    std::fs::remove_file(&cross_baseline_path).unwrap();

    let premature = collect_fresh_absence_v3(fixture.request(), &snapshot).unwrap();
    let mut premature_receipt = premature.receipt().clone();
    drop(premature);
    premature_receipt.monotonic_before_nanoseconds = snapshot.monotonic_after_nanoseconds;
    validate_receipt(&premature_receipt).unwrap();
    let premature_bytes = canonical_json(&premature_receipt).unwrap();
    let premature_path = fixture
        .persistence_root
        .join(format!("collector-{}.json", sha256(&premature_bytes)));
    write_private(&premature_path, &premature_bytes);
    assert!(collect_reconciliation_snapshot_v3(fixture.request()).is_err());
    std::fs::remove_file(&premature_path).unwrap();

    let mut forged = snapshot.clone();
    forged.iomedia_evidence_sha256 = "bad".to_string();
    assert!(collect_fresh_absence_v3(fixture.request(), &forged).is_err());
    let mut forged = snapshot.clone();
    forged.collector_receipt_sha256 = "f".repeat(64);
    assert!(collect_fresh_absence_v3(fixture.request(), &forged).is_err());

    std::fs::remove_file(&snapshot_receipt).unwrap();
    assert!(collect_fresh_absence_v3(fixture.request(), &snapshot).is_err());
    write_private(&snapshot_receipt, &snapshot_receipt_bytes);

    write_private(&snapshot_receipt, b"tampered receipt");
    assert!(collect_fresh_absence_v3(fixture.request(), &snapshot).is_err());
    write_private(&snapshot_receipt, &snapshot_receipt_bytes);

    set_test_xattr(&snapshot_receipt);
    assert!(collect_fresh_absence_v3(fixture.request(), &snapshot).is_err());
    remove_test_xattr(&snapshot_receipt);
    set_test_acl(&snapshot_receipt);
    assert!(collect_fresh_absence_v3(fixture.request(), &snapshot).is_err());
    remove_test_acl(&snapshot_receipt);
    std::fs::set_permissions(&snapshot_receipt, std::fs::Permissions::from_mode(0o644)).unwrap();
    assert!(collect_fresh_absence_v3(fixture.request(), &snapshot).is_err());
    std::fs::set_permissions(&snapshot_receipt, std::fs::Permissions::from_mode(0o600)).unwrap();
    let hardlink = fixture.artifact_root.join("receipt-hardlink");
    std::fs::hard_link(&snapshot_receipt, &hardlink).unwrap();
    assert!(collect_fresh_absence_v3(fixture.request(), &snapshot).is_err());
    std::fs::remove_file(&hardlink).unwrap();

    let absence = collect_fresh_absence_v3(fixture.request(), &snapshot).unwrap();
    let retained_absence = absence.persist_and_retain().unwrap();
    retained_absence.revalidate().unwrap();
    assert!(matches!(
        retained_absence.observation_for_test(),
        FinalizedRestartObservationV3::FreshAbsence(_)
    ));
    std::fs::remove_file(&snapshot_receipt).unwrap();
    assert!(collect_reconciliation_snapshot_v3(fixture.request()).is_err());
    write_private(&snapshot_receipt, &snapshot_receipt_bytes);
}

#[test]
fn rootless_live_zero_requires_persistence_and_final_replay_for_both_observations() {
    let _lock = LIVE_COLLECTOR_TEST_LOCK
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    // Keep the descriptor-bound fixture out of the shared TMPDIR: unrelated parallel
    // tests legitimately churn that directory's metadata, which the production
    // ancestor replay must continue to reject.
    let fixture = tempfile::Builder::new()
        .prefix(".restart-collector-fixture-")
        .tempdir_in(env!("CARGO_MANIFEST_DIR"))
        .unwrap();
    let fixture_root = std::fs::canonicalize(fixture.path()).unwrap();
    let persistence = tempfile::Builder::new()
        .prefix(".restart-collector-receipts-")
        .tempdir_in(env!("CARGO_MANIFEST_DIR"))
        .unwrap();
    let persistence_root = std::fs::canonicalize(persistence.path()).unwrap();
    let backing_path = fixture_root.join("prepared.img");
    let mountpoint = fixture_root.join("mountpoint");
    let artifact_root = fixture_root.join("artifacts");
    std::fs::write(&backing_path, b"rootless-restart-backing-v3").unwrap();
    std::fs::create_dir(&mountpoint).unwrap();
    std::fs::create_dir(&artifact_root).unwrap();

    let prepared_backing = capture_live_backing_identity_v2(&backing_path).unwrap();
    let mountpoint_identity = MountpointIdentityV3::capture(&mountpoint).unwrap();
    let baseline = capture_live_restart_baseline_v3().unwrap();
    let artifacts = [backing_artifact("operation-created.img")];
    let policy = RestartCollectorPolicyV3::new(
        &backing_path,
        &mountpoint,
        &artifact_root,
        &persistence_root,
        &artifacts,
        &[],
    )
    .unwrap();
    let bindings = RestartCollectorBindingsV3 {
        backing_identity_sha256: sha256(&canonical_json(&prepared_backing).unwrap()),
        baseline_inventory_sha256: baseline.sha256().unwrap(),
        boot_session_uuid: baseline.boot_session_uuid.clone(),
        collector_policy_sha256: policy.sha256().unwrap(),
        mountpoint_underlying_sha256: mountpoint_identity.sha256().unwrap(),
        operation_nonce: "3".repeat(64),
        restart_epoch_nonce: "4".repeat(64),
        restart_started_monotonic_nanoseconds: monotonic_nanoseconds().unwrap() - 1,
    };
    let request = || LiveRestartCollectorRequestV3 {
        artifact_root: &artifact_root,
        baseline: &baseline,
        bindings: &bindings,
        mountpoint_identity: &mountpoint_identity,
        policy: &policy,
        prepared_backing: &prepared_backing,
        receipt_root: &persistence_root,
    };

    let failed = collect_reconciliation_snapshot_v3(request()).unwrap();
    let uncertain = persistence_root.join(".incoming-collector-crash");
    std::fs::write(&uncertain, b"uncertain").unwrap();
    assert!(failed.persist_and_retain().is_err());
    std::fs::remove_file(&uncertain).unwrap();

    let drifted = collect_reconciliation_snapshot_v3(request()).unwrap();
    let forbidden = artifact_root.join("operation-created.img");
    let drift_result = drifted.persist_and_retain_with_hook(|| {
        std::fs::write(&forbidden, b"late artifact")?;
        Ok(())
    });
    assert!(drift_result.is_err());
    if forbidden.exists() {
        std::fs::remove_file(&forbidden).unwrap();
    }

    let pending = collect_reconciliation_snapshot_v3(request()).unwrap();
    assert_eq!(pending.receipt().match_result, ReconciliationMatchV2::Zero);
    assert!(!pending.receipt().authority.any());
    let mut tampered = pending.receipt().clone();
    tampered.baseline_restored = false;
    assert!(validate_receipt(&tampered).is_err());
    let mut tampered = pending.receipt().clone();
    tampered.authority.privileged_effect_authority = true;
    assert!(validate_receipt(&tampered).is_err());
    let mut tampered = pending.receipt().clone();
    tampered.artifact_evidence.roster = vec!["duplicate".to_string(); 2];
    tampered.artifact_evidence_sha256 =
        sha256(&canonical_json(&tampered.artifact_evidence).unwrap());
    assert!(validate_receipt(&tampered).is_err());
    let mut tampered = pending.receipt().clone();
    tampered.artifact_evidence.root_binding.mode = libc::S_IFREG as u32 | 0o600;
    tampered.artifact_evidence_sha256 =
        sha256(&canonical_json(&tampered.artifact_evidence).unwrap());
    assert!(validate_receipt(&tampered).is_err());
    let mut tampered = pending.receipt().clone();
    tampered.mountpoint_underlying.path = artifact_root.to_str().unwrap().to_string();
    tampered.mountpoint_underlying_sha256 = tampered.mountpoint_underlying.sha256().unwrap();
    assert!(validate_receipt(&tampered).is_err());
    let mut tampered = pending.receipt().clone();
    tampered.baseline_inventory.boot_session_uuid =
        "87654321-4321-4321-8321-cba987654321".to_string();
    tampered.baseline_inventory_sha256 = tampered.baseline_inventory.sha256().unwrap();
    assert!(validate_receipt(&tampered).is_err());
    let retained_snapshot = pending.persist_and_retain().unwrap();
    retained_snapshot.revalidate().unwrap();
    let snapshot = match retained_snapshot.observation_for_test() {
        FinalizedRestartObservationV3::ReconciliationSnapshot(snapshot) => snapshot.clone(),
        FinalizedRestartObservationV3::FreshAbsence(_) => panic!("wrong observation"),
    };
    assert_eq!(snapshot.match_result, ReconciliationMatchV2::Zero);

    let mut records = Vec::new();
    let mut journal = DisposableLifecycleJournalV2::new(&bindings.operation_nonce).unwrap();
    let mut append = |event| {
        journal
            .append_with(event, |_, bytes| {
                records.push(bytes.to_vec());
                Ok(())
            })
            .unwrap()
    };
    append(DisposableLifecycleEventV2::OperationPrepared {
        baseline_inventory_sha256: bindings.baseline_inventory_sha256.clone(),
        backing_identity_sha256: bindings.backing_identity_sha256.clone(),
        boot_session_uuid: bindings.boot_session_uuid.clone(),
        collector_policy_sha256: bindings.collector_policy_sha256.clone(),
        mountpoint_underlying_sha256: bindings.mountpoint_underlying_sha256.clone(),
    });
    drop(append);
    let mut resumed = DisposableLifecycleJournalV2::resume_for_reconciliation(&records).unwrap();
    resumed
        .append_with(
            DisposableLifecycleEventV2::RestartReconciliationStarted {
                boot_session_uuid: bindings.boot_session_uuid.clone(),
                collector_policy_sha256: bindings.collector_policy_sha256.clone(),
                monotonic_nanoseconds: bindings.restart_started_monotonic_nanoseconds,
                restart_epoch_nonce: bindings.restart_epoch_nonce.clone(),
            },
            |_, bytes| {
                records.push(bytes.to_vec());
                Ok(())
            },
        )
        .unwrap();
    resumed
        .append_with(
            DisposableLifecycleEventV2::ReconciliationSnapshotObserved {
                snapshot: snapshot.clone(),
            },
            |_, bytes| {
                records.push(bytes.to_vec());
                Ok(())
            },
        )
        .unwrap();

    let pending = collect_fresh_absence_v3(request(), &snapshot).unwrap();
    let retained_absence = pending.persist_and_retain().unwrap();
    retained_absence.revalidate().unwrap();
    let absence = match retained_absence.observation_for_test() {
        FinalizedRestartObservationV3::FreshAbsence(observation) => observation.clone(),
        FinalizedRestartObservationV3::ReconciliationSnapshot(_) => panic!("wrong observation"),
    };
    assert!(absence.no_matching_iomedia);
    assert!(absence.no_nested_mounts);
    assert!(absence.operation_artifacts_absent);
    assert_eq!(
        absence.reconciliation_snapshot_sha256,
        Some(reconciliation_snapshot_sha256(&snapshot).unwrap())
    );
    let absence_sha = fresh_absence_sha256(&absence).unwrap();
    resumed
        .append_with(
            DisposableLifecycleEventV2::FreshAbsenceObserved {
                observation: absence,
            },
            |_, bytes| {
                records.push(bytes.to_vec());
                Ok(())
            },
        )
        .unwrap();
    assert!(valid_digest(&absence_sha));

    let mut receipt_names = std::fs::read_dir(&persistence_root)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .collect::<Vec<_>>();
    receipt_names.sort();
    assert_eq!(receipt_names.len(), 3);
    assert!(receipt_names.iter().all(|name| {
        name.starts_with("collector-") && name.ends_with(".json") && !name.starts_with(".incoming-")
    }));
}
