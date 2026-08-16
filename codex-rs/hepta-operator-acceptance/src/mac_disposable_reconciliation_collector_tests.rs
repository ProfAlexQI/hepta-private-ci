use super::*;

use crate::mac_disposable_lifecycle::DisposableLifecycleEventV2;
use crate::mac_disposable_lifecycle::DisposableLifecycleJournalV2;
use crate::mac_disposable_lifecycle::PreparedCollectorManifestBindingV3;
use crate::mac_disposable_lifecycle_store::CensusBoundDurableLifecycleStoreV3;
use crate::mac_disposable_lifecycle_store::DurableLifecycleStoreV3;
use crate::mac_disposable_lifecycle_store::ReconciliationDurableLifecycleStoreV3;
use crate::mac_disposable_lifecycle_store::ReconciliationOperationStoreV3;
use crate::mac_disposable_lifecycle_store::RestartAdmissionPublishCutpointV3;
use crate::mac_inert_one_shot_runner::FreshProcessEpochV3;
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
use crate::mac_privileged_disposable_control::LivePrivilegedDisposablePolicyV2;
use std::ffi::CString;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::fs::PermissionsExt;
use std::os::unix::fs::symlink;
use std::process::Command;

static LIVE_COLLECTOR_TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

fn synthetic_receipt_file_binding(receipt_sha256: &str) -> CollectorReceiptFileBindingV3 {
    let root_after = FilesystemObjectBindingV3 {
        birthtime_nanoseconds: 1,
        birthtime_seconds: 1,
        ctime_nanoseconds: 1,
        ctime_seconds: 1,
        dev: 1,
        flags: 0,
        generation: 1,
        gid: 1,
        inode: 1,
        mode: u32::from(libc::S_IFDIR | 0o700),
        mtime_nanoseconds: 1,
        mtime_seconds: 1,
        nlink: 3,
        size: 1,
        uid: 1,
    };
    CollectorReceiptFileBindingV3::from_retained_collector(
        CollectorReceiptFileBindingSealV3 { _private: () },
        receipt_sha256.to_string(),
        format!("collector-{receipt_sha256}.json"),
        FilesystemObjectBindingV3 {
            birthtime_nanoseconds: 1,
            birthtime_seconds: 1,
            ctime_nanoseconds: 1,
            ctime_seconds: 1,
            dev: 1,
            flags: 0,
            generation: 1,
            gid: 1,
            inode: 2,
            mode: u32::from(libc::S_IFREG | 0o600),
            mtime_nanoseconds: 1,
            mtime_seconds: 1,
            nlink: 1,
            size: 1,
            uid: 1,
        },
        root_after,
        1,
    )
}

struct LiveCollectorTestGuard {
    _thread: std::sync::MutexGuard<'static, ()>,
    process: File,
}

impl Drop for LiveCollectorTestGuard {
    fn drop(&mut self) {
        assert_eq!(
            unsafe { libc::flock(self.process.as_raw_fd(), libc::LOCK_UN) },
            0
        );
    }
}

fn live_collector_test_lock() -> LiveCollectorTestGuard {
    let thread = LIVE_COLLECTOR_TEST_LOCK
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let process = std::fs::OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .mode(0o600)
        .open("/tmp/hepta-mac-restart-collector-test.lock")
        .expect("open cross-process live collector test lock");
    assert_eq!(
        unsafe { libc::flock(process.as_raw_fd(), libc::LOCK_EX) },
        0
    );
    LiveCollectorTestGuard {
        _thread: thread,
        process,
    }
}

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
        TryFrom<File>,
        From<Vec<MountBindingV3>>,
        From<String>
);
assert_not_impl_any!(
    RetainedCollectorLineageV3:
        Clone,
        Send,
        Sync,
        serde::Serialize,
        serde::de::DeserializeOwned,
        std::os::fd::AsRawFd,
        std::os::fd::AsFd,
        std::os::fd::IntoRawFd,
        From<File>,
        TryFrom<File>,
        From<RetainedCollectorObservationV3>,
        From<String>
);
assert_not_impl_any!(
    RetainedPreparedCollectorCapabilityV3:
        Clone,
        Send,
        Sync,
        serde::Serialize,
        serde::de::DeserializeOwned,
        std::os::fd::AsRawFd,
        std::os::fd::AsFd,
        std::os::fd::IntoRawFd,
        From<File>,
        TryFrom<File>,
        From<Vec<u8>>,
        From<String>
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
        TryFrom<File>,
        From<Vec<MountBindingV3>>,
        From<String>
);
assert_not_impl_any!(
    RetainedCollectorIssueBindingV3<'static>:
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
    RetainedCollectorMountDeltaV3<MountingV3>:
        Clone,
        Send,
        Sync,
        serde::Serialize,
        serde::de::DeserializeOwned,
        std::os::fd::AsRawFd,
        std::os::fd::AsFd,
        std::os::fd::IntoRawFd,
        From<File>,
        TryFrom<File>,
        From<Vec<MountBindingV3>>,
        From<String>
);
assert_not_impl_any!(
    RetainedCollectorMountDeltaV3<UnmountingV3>:
        Clone,
        Send,
        Sync,
        serde::Serialize,
        serde::de::DeserializeOwned,
        std::os::fd::AsRawFd,
        std::os::fd::AsFd,
        std::os::fd::IntoRawFd,
        From<File>,
        TryFrom<File>,
        From<Vec<MountBindingV3>>,
        From<String>
);
assert_not_impl_any!(
    SealedMountDeltaPlanV3<'static, MountingV3>:
        Clone,
        Send,
        Sync,
        serde::Serialize,
        serde::de::DeserializeOwned,
        std::os::fd::AsRawFd,
        std::os::fd::AsFd,
        std::os::fd::IntoRawFd,
        From<File>,
        TryFrom<File>,
        From<Vec<MountBindingV3>>,
        From<String>
);
assert_not_impl_any!(
    SealedMountDeltaAdvanceV3<'static, UnmountingV3>:
        Clone,
        Send,
        Sync,
        serde::Serialize,
        serde::de::DeserializeOwned,
        std::os::fd::AsRawFd,
        std::os::fd::AsFd,
        std::os::fd::IntoRawFd,
        From<File>,
        TryFrom<File>,
        From<Vec<MountBindingV3>>,
        From<String>
);
assert_not_impl_any!(
    SealedMountDeltaObservationV3<'static, UnmountingV3>:
        Clone,
        Send,
        Sync,
        serde::Serialize,
        serde::de::DeserializeOwned,
        std::os::fd::AsRawFd,
        std::os::fd::AsFd,
        std::os::fd::IntoRawFd,
        From<File>,
        TryFrom<File>,
        From<Vec<MountBindingV3>>,
        From<String>
);

fn id(value: u64) -> String {
    format!("{value:016x}")
}

fn synthetic_mount(
    filesystem_id: [i32; 2],
    mount_from: &str,
    mount_on: &str,
    mount_flags: u64,
) -> MountBindingV3 {
    MountBindingV3 {
        filesystem_id,
        filesystem_type: "apfs".to_string(),
        mount_flags,
        mount_from: mount_from.to_string(),
        mount_on: mount_on.to_string(),
    }
}

#[test]
fn exact_mount_delta_helpers_reject_extra_changed_and_unsorted_states() {
    let existing = synthetic_mount([1, 1], "/dev/disk1s1", "/", 0);
    let target = synthetic_mount(
        [9, 4],
        "/dev/disk9s4",
        "/private/tmp/hepta-mount",
        libc::MNT_RDONLY as u64,
    );
    let before = vec![existing.clone()];
    let mut after = vec![existing.clone(), target.clone()];
    after.sort();
    assert_eq!(exact_added_mount(&before, &after), Some(&target));
    assert_eq!(exact_removed_mount(&after, &before), Some(&target));

    let mut changed = after.clone();
    let existing_index = changed
        .iter()
        .position(|entry| entry.mount_on == "/")
        .unwrap();
    changed[existing_index].mount_flags ^= libc::MNT_RDONLY as u64;
    changed.sort();
    assert!(exact_added_mount(&before, &changed).is_none());

    let mut extra = after.clone();
    extra.push(synthetic_mount(
        [10, 1],
        "/dev/disk10s1",
        "/private/tmp/foreign",
        0,
    ));
    extra.sort();
    assert!(exact_added_mount(&before, &extra).is_none());
    assert!(exact_removed_mount(&extra, &before).is_none());

    let mut unsorted = after.clone();
    unsorted.reverse();
    assert!(exact_added_mount(&before, &unsorted).is_none());
    assert!(exact_removed_mount(&unsorted, &before).is_none());
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

fn receipt_for_test_generation(
    template: &RestartCollectorReceiptV3,
    ordinal: u64,
) -> RestartCollectorReceiptV3 {
    let mut receipt = template.clone();
    receipt.monotonic_before_nanoseconds = template
        .monotonic_after_nanoseconds
        .checked_add(ordinal.saturating_mul(2))
        .expect("test receipt timestamp overflow");
    receipt.monotonic_after_nanoseconds = receipt
        .monotonic_before_nanoseconds
        .checked_add(1)
        .expect("test receipt timestamp overflow");
    validate_receipt(&receipt).expect("test receipt generation remains canonical");
    receipt
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
    backing_path: PathBuf,
    baseline: RestartBaselineInventoryV3,
    bindings: RestartCollectorBindingsV3,
    mountpoint_identity: MountpointIdentityV3,
    mountpoint_path: PathBuf,
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
        let mountpoint = fixture_root.join("mountpoint");
        let artifact_root = fixture_root.join("artifacts");
        std::fs::create_dir(&mountpoint).unwrap();
        std::fs::create_dir(&artifact_root).unwrap();
        let backing_path = artifact_root.join("operation-created.img");
        std::fs::write(&backing_path, b"rootless-redteam-backing-v3").unwrap();
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
            backing_path,
            baseline,
            bindings,
            mountpoint_identity,
            mountpoint_path: mountpoint,
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

    fn prepared_capability(&self, operation_nonce: &str) -> RetainedPreparedCollectorCapabilityV3 {
        RetainedPreparedCollectorCapabilityV3::capture(
            operation_nonce,
            &self.backing_path,
            &self.mountpoint_path,
            &self.artifact_root,
            &self.persistence_root,
            "operation-created.img",
            &[],
        )
        .expect("capture exact prepared collector capability")
    }
}

fn publish_exact_prepared_operation(
    fixture: &LiveCollectorFixture,
    control_root: &Path,
    operation_nonce: &str,
) {
    let control = LivePrivilegedDisposablePolicyV2::create_for_test(control_root)
        .expect("create exact S1 control");
    let census = control
        .assess_read_only()
        .expect("fresh S1 assessment")
        .into_fresh_control_census()
        .expect("fresh S1 census");
    let prepared = fixture.prepared_capability(operation_nonce);
    let mut operation = CensusBoundDurableLifecycleStoreV3::create_prepared(census, prepared)
        .expect("create exact prepared operation");
    operation
        .persist_retained_prepared()
        .expect("persist exact prepared lifecycle and sidecar");
}

struct ReceiptGenerationStoreHarness {
    _operations: File,
    journal: DisposableLifecycleJournalV2,
    store: ReconciliationDurableLifecycleStoreV3,
}

fn open_receipt_generation_store(
    fixture: &mut LiveCollectorFixture,
    control_root: &Path,
    operation_nonce: &str,
) -> ReceiptGenerationStoreHarness {
    assert_eq!(operation_nonce, fixture.bindings.operation_nonce);
    let process_epoch = FreshProcessEpochV3::establish()
        .expect("establish exact process epoch for receipt-generation gate");
    let restart_epoch = process_epoch
        .bind_restart_admission()
        .expect("bind exact restart admission for receipt-generation gate");
    fixture.bindings.boot_session_uuid = restart_epoch.boot_session_uuid().to_string();
    fixture.bindings.restart_epoch_nonce = restart_epoch.restart_epoch_nonce().to_string();
    fixture.bindings.restart_started_monotonic_nanoseconds =
        restart_epoch.restart_started_monotonic_nanoseconds();
    publish_exact_prepared_operation(fixture, control_root, operation_nonce);
    assert_eq!(
        capture_live_backing_identity_v2(&fixture.backing_path)
            .expect("revalidate backing after exact prepared publication"),
        fixture.prepared_backing,
        "independent control publication must not change prepared path lineage"
    );
    let operations = File::open(control_root.join("operations"))
        .expect("open exact operations directory for receipt-generation gate");
    let mut store = DurableLifecycleStoreV3::open_existing(
        &operations,
        operation_nonce,
        unsafe { libc::geteuid() },
        unsafe { libc::getegid() },
    )
    .expect("open exact prepared store for receipt-generation gate");
    let mut journal = store
        .resume_for_reconciliation()
        .expect("resume exact receipt-generation lifecycle");
    store
        .append_restart_for_receipt_gate(&mut journal, restart_epoch)
        .expect("persist exact restart epoch for receipt-generation gate");
    ReceiptGenerationStoreHarness {
        _operations: operations,
        journal,
        store,
    }
}

fn persist_snapshot_for_receipt_generation(
    fixture: &LiveCollectorFixture,
    harness: &mut ReceiptGenerationStoreHarness,
) -> (ReconciliationSnapshotV2, RestartCollectorReceiptV3) {
    let pending = collect_reconciliation_snapshot_v3(fixture.request())
        .expect("collect exact G1 snapshot for receipt-generation gate");
    let receipt = pending.receipt().clone();
    let retained = pending
        .persist_and_retain()
        .expect("persist exact G1 receipt and retain its final-stat capsule");
    let snapshot = match retained.observation_for_test() {
        FinalizedRestartObservationV3::ReconciliationSnapshot(snapshot) => snapshot.clone(),
        FinalizedRestartObservationV3::FreshAbsence(_) => panic!("wrong G1 observation"),
    };
    assert!(snapshot.collector_receipt_file.is_some());
    harness
        .store
        .append_reconciliation_for_receipt_gate(
            &mut harness.journal,
            DisposableLifecycleEventV2::ReconciliationSnapshotObserved {
                snapshot: snapshot.clone(),
            },
        )
        .expect("append capsule-derived exact G1 lifecycle binding");
    drop(retained);
    (snapshot, receipt)
}

fn persist_g1_and_drop_all_capsules(
    fixture: &mut LiveCollectorFixture,
    control_root: &Path,
    operation_nonce: &str,
) -> (PathBuf, Vec<u8>, RestartCollectorReceiptV3) {
    let mut harness = open_receipt_generation_store(fixture, control_root, operation_nonce);
    let (_snapshot, receipt) = persist_snapshot_for_receipt_generation(fixture, &mut harness);
    let receipt_sha256 = sha256(&canonical_json(&receipt).unwrap());
    let path = fixture
        .persistence_root
        .join(format!("collector-{receipt_sha256}.json"));
    let bytes = std::fs::read(&path).expect("read exact G1 receipt");
    drop(harness);
    (path, bytes, receipt)
}

fn restart_admission_roster(control_root: &Path, operation_nonce: &str) -> Vec<String> {
    let root = control_root
        .join("operations")
        .join(format!("operation-{operation_nonce}"))
        .join("restart-admissions-v3");
    let mut names = std::fs::read_dir(root)
        .expect("restart admission roster")
        .map(|entry| {
            entry
                .expect("restart admission entry")
                .file_name()
                .into_string()
                .expect("UTF-8 restart admission name")
        })
        .collect::<Vec<_>>();
    names.sort();
    names
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
        collector_receipt_file: None,
        collector_receipt_sha256: "c".repeat(64),
        current_expected_absence_inventory_sha256: Some("d".repeat(64)),
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
fn current_boot_expected_absence_is_full_exact_and_unique_subtraction_is_closed_world() {
    let path = "/private/tmp/hepta.img";
    let prepared = synthetic_backing(path);
    let unrelated = object(1, "disk1", None);
    let current = inventory(vec![
        unrelated.clone(),
        object(10, "disk10", Some(candidate(100, path))),
        object(11, "disk10s1", Some(candidate(100, path))),
    ]);
    let groups = classify_matching_groups(&current, &prepared).unwrap();
    assert_eq!(groups.len(), 1);

    let expected = derive_current_expected_absence_v3(
        &current,
        &ReconciliationMatchV2::Unique { mounted: false },
        &groups,
    )
    .unwrap()
    .expect("Unique has one exact expected absence");
    assert_eq!(expected, inventory(vec![unrelated]));
    assert_ne!(
        sha256(&canonical_json(&expected).unwrap()),
        sha256(&canonical_json(&current).unwrap())
    );
    let expected_sha256 = sha256(&canonical_json(&expected).unwrap());
    validate_exact_expected_absence_inventory(
        &expected,
        &expected_sha256,
        &expected,
        &expected_sha256,
    )
    .unwrap();
    let mut nonmember_property_drift = expected.clone();
    nonmember_property_drift.objects[0]
        .provenance
        .disk_arbitration
        .internal = Some(false);
    validate_restart_iomedia_inventory_v3(&nonmember_property_drift).unwrap();
    assert_eq!(
        expected
            .objects
            .iter()
            .map(|object| &object.provenance.registry_entry_id)
            .collect::<Vec<_>>(),
        nonmember_property_drift
            .objects
            .iter()
            .map(|object| &object.provenance.registry_entry_id)
            .collect::<Vec<_>>()
    );
    let drift_sha256 = sha256(&canonical_json(&nonmember_property_drift).unwrap());
    assert!(
        validate_exact_expected_absence_inventory(
            &expected,
            &expected_sha256,
            &nonmember_property_drift,
            &drift_sha256,
        )
        .is_err()
    );

    assert_eq!(
        derive_current_expected_absence_v3(&current, &ReconciliationMatchV2::Zero, &[],).unwrap(),
        Some(current.clone())
    );
    assert_eq!(
        derive_current_expected_absence_v3(
            &current,
            &ReconciliationMatchV2::Ambiguous {
                matching_objects: 2,
            },
            &[
                groups[0].clone(),
                MatchingDiskImageGroupV3 {
                    candidate: candidate(101, path),
                    member_bsd_names: vec!["disk11".to_string()],
                    member_registry_entry_ids: vec![id(12)],
                },
            ],
        )
        .unwrap(),
        None
    );

    let mut duplicate_member = groups.clone();
    let duplicate_id = duplicate_member[0].member_registry_entry_ids[0].clone();
    duplicate_member[0]
        .member_registry_entry_ids
        .push(duplicate_id);
    assert!(
        derive_current_expected_absence_v3(
            &current,
            &ReconciliationMatchV2::Unique { mounted: false },
            &duplicate_member,
        )
        .is_err()
    );

    let mut foreign_member = groups;
    foreign_member[0].member_registry_entry_ids.push(id(999));
    foreign_member[0].member_registry_entry_ids.sort();
    assert!(
        derive_current_expected_absence_v3(
            &current,
            &ReconciliationMatchV2::Unique { mounted: false },
            &foreign_member,
        )
        .is_err()
    );
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
    forged.current_expected_absence_inventory_sha256 = None;
    assert!(validate_reconciliation_snapshot_shape_v3(&forged).is_err());
    let mut forged = snapshot.clone();
    forged.match_result = ReconciliationMatchV2::Unique { mounted: false };
    assert!(validate_reconciliation_snapshot_shape_v3(&forged).is_err());
    let mut ambiguous = snapshot.clone();
    ambiguous.match_result = ReconciliationMatchV2::Ambiguous {
        matching_objects: 2,
    };
    ambiguous.current_expected_absence_inventory_sha256 = None;
    validate_reconciliation_snapshot_shape_v3(&ambiguous).unwrap();
    ambiguous.current_expected_absence_inventory_sha256 = Some("9".repeat(64));
    assert!(validate_reconciliation_snapshot_shape_v3(&ambiguous).is_err());

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
fn prepared_collector_capability_reopens_only_its_exact_manifest() {
    let _lock = live_collector_test_lock();
    let fixture = LiveCollectorFixture::new();
    let operation_nonce = "7".repeat(64);
    let retained = fixture.prepared_capability(&operation_nonce);
    retained.revalidate().expect("live prepared replay");
    let manifest_bytes = retained.manifest_bytes().to_vec();
    let manifest_sha256 = retained.manifest_sha256().to_string();
    let profile_sha256 = retained.profile_sha256().to_string();
    assert_eq!(retained.operation_nonce(), operation_nonce);
    assert_eq!(
        retained.boot_session_uuid(),
        fixture.baseline.boot_session_uuid
    );
    drop(retained);

    let reopened = RetainedPreparedCollectorCapabilityV3::reopen_from_exact_manifest(
        &operation_nonce,
        &manifest_bytes,
        &manifest_sha256,
        &profile_sha256,
    )
    .expect("reopen exact durable manifest candidate");
    reopened.revalidate().expect("reopened prepared replay");
    drop(reopened);

    assert!(
        RetainedPreparedCollectorCapabilityV3::reopen_from_exact_manifest(
            &"8".repeat(64),
            &manifest_bytes,
            &manifest_sha256,
            &profile_sha256,
        )
        .is_err(),
        "prepared manifest must not move across operations"
    );
    assert!(
        RetainedPreparedCollectorCapabilityV3::reopen_from_exact_manifest(
            &operation_nonce,
            &manifest_bytes,
            &"9".repeat(64),
            &profile_sha256,
        )
        .is_err(),
        "caller-computed manifest digest must not replace the durable commitment"
    );
    assert!(
        RetainedPreparedCollectorCapabilityV3::reopen_from_exact_manifest(
            &operation_nonce,
            &manifest_bytes,
            &manifest_sha256,
            &"a".repeat(64),
        )
        .is_err(),
        "collector profile commitment must be exact"
    );
}

#[test]
fn prepared_collector_rejects_unknown_initial_artifact_roster() {
    let _lock = live_collector_test_lock();
    let fixture = LiveCollectorFixture::new();
    std::fs::write(fixture.artifact_root.join("foreign"), b"not-a-profile-role")
        .expect("inject unknown initial artifact");
    assert!(
        RetainedPreparedCollectorCapabilityV3::capture(
            &"7".repeat(64),
            &fixture.backing_path,
            &fixture.mountpoint_path,
            &fixture.artifact_root,
            &fixture.persistence_root,
            "operation-created.img",
            &[],
        )
        .is_err(),
        "fresh capture must not turn an arbitrary initial file into a prepared role"
    );
}

#[test]
fn exact_prepared_capability_drives_durable_creation_and_restart_replay() {
    let _lock = live_collector_test_lock();
    let fixture = LiveCollectorFixture::new();
    let operation_nonce = "7".repeat(64);
    let control_root = fixture._fixture.path().join("control");
    {
        let control = LivePrivilegedDisposablePolicyV2::create_for_test(&control_root)
            .expect("create exact S1 control");
        let prepared = fixture.prepared_capability(&operation_nonce);
        let census = control
            .assess_read_only()
            .expect("fresh S1 assessment")
            .into_fresh_control_census()
            .expect("fresh S1 census");
        let mut operation = CensusBoundDurableLifecycleStoreV3::create_prepared(census, prepared)
            .expect("create from exact retained prepared capability");
        operation
            .persist_retained_prepared()
            .expect("persist exact prepared record and sidecar binding");
        assert!(!operation.poisoned());
    }

    let control = LivePrivilegedDisposablePolicyV2::create_for_test(&control_root)
        .expect("reopen exact S1 control");
    let census = control
        .assess_read_only()
        .expect("blocking S1 assessment")
        .into_blocking_control_census(&operation_nonce)
        .expect("exact blocking S1 census");
    let epoch = FreshProcessEpochV3::establish().expect("fresh restart process epoch");
    let operation = ReconciliationOperationStoreV3::open_existing(census, &epoch)
        .expect("restart reopens the retained prepared capability from its exact sidecar");
    let operation = operation
        .begin_restart(&epoch)
        .expect("restart admission durably binds the exact S1/S2 operation before activation");
    assert_eq!(operation.operation_nonce(), operation_nonce);
    assert!(!operation.poisoned());
    let admissions = control_root
        .join("operations")
        .join(format!("operation-{operation_nonce}"))
        .join("restart-admissions-v3");
    assert_eq!(
        std::fs::read_dir(admissions)
            .expect("restart admission roster")
            .count(),
        1,
        "one durable V2 RestartStarted record must have one exact V3 admission"
    );
}

#[test]
fn restart_admission_cutpoints_never_return_active_and_replay_fail_closed() {
    let _lock = live_collector_test_lock();
    for cutpoint in [
        RestartAdmissionPublishCutpointV3::TemporaryCreated,
        RestartAdmissionPublishCutpointV3::BytesWritten,
        RestartAdmissionPublishCutpointV3::FileSynced,
        RestartAdmissionPublishCutpointV3::Renamed,
        RestartAdmissionPublishCutpointV3::DirectorySynced,
        RestartAdmissionPublishCutpointV3::FinalReopened,
        RestartAdmissionPublishCutpointV3::FinalRevalidated,
        RestartAdmissionPublishCutpointV3::CapsuleRetained,
        RestartAdmissionPublishCutpointV3::FinalReplayed,
    ] {
        let fixture = LiveCollectorFixture::new();
        let operation_nonce = "7".repeat(64);
        let control_root = fixture._fixture.path().join("control-cutpoint");
        publish_exact_prepared_operation(&fixture, &control_root, &operation_nonce);

        let control = LivePrivilegedDisposablePolicyV2::create_for_test(&control_root)
            .expect("reopen exact S1 control");
        let census = control
            .assess_read_only()
            .expect("blocking S1 assessment")
            .into_blocking_control_census(&operation_nonce)
            .expect("blocking S1 census");
        let epoch = FreshProcessEpochV3::establish().expect("cutpoint process epoch");
        let needs = ReconciliationOperationStoreV3::open_existing(census, &epoch)
            .expect("open NeedsRestartEpoch");
        let result = needs.begin_restart_with_admission_hook(&epoch, |observed| {
            if observed == cutpoint {
                Err(std::io::Error::other("injected restart-admission cutpoint"))
            } else {
                Ok(())
            }
        });
        assert!(result.is_err(), "{cutpoint:?} must not return Active");
        drop(control);
        let replay_control = LivePrivilegedDisposablePolicyV2::create_for_test(&control_root)
            .expect("reopen control after consumed cutpoint assessment");

        let roster = restart_admission_roster(&control_root, &operation_nonce);
        let pre_rename = matches!(
            cutpoint,
            RestartAdmissionPublishCutpointV3::TemporaryCreated
                | RestartAdmissionPublishCutpointV3::BytesWritten
                | RestartAdmissionPublishCutpointV3::FileSynced
        );
        if pre_rename {
            assert_eq!(roster.len(), 1, "{cutpoint:?}");
            assert!(roster[0].starts_with(".incoming-"), "{cutpoint:?}");
            assert!(
                replay_control.assess_read_only().is_err(),
                "temporary or missing admission must block exact S1 replay: {cutpoint:?}"
            );
        } else {
            assert_eq!(roster.len(), 1, "{cutpoint:?}");
            assert!(roster[0].starts_with("restart-"), "{cutpoint:?}");
            let census = replay_control
                .assess_read_only()
                .expect("post-rename pair replays")
                .into_blocking_control_census(&operation_nonce)
                .expect("post-rename blocking census");
            let next_epoch = FreshProcessEpochV3::establish().expect("next process epoch");
            let active = ReconciliationOperationStoreV3::open_existing(census, &next_epoch)
                .expect("reopen post-rename exact pair")
                .begin_restart(&next_epoch)
                .expect("append a fresh superseding restart admission");
            assert!(!active.poisoned(), "{cutpoint:?}");
            assert_eq!(
                restart_admission_roster(&control_root, &operation_nonce).len(),
                2,
                "replay must retain the first pair and append a second: {cutpoint:?}"
            );
        }
    }
}

#[test]
fn multiple_restart_epochs_replay_as_an_exact_append_only_bijection() {
    let _lock = live_collector_test_lock();
    let fixture = LiveCollectorFixture::new();
    let operation_nonce = "7".repeat(64);
    let control_root = fixture._fixture.path().join("control-multi-epoch");
    publish_exact_prepared_operation(&fixture, &control_root, &operation_nonce);
    for ordinal in 1..=2 {
        let control = LivePrivilegedDisposablePolicyV2::create_for_test(&control_root)
            .expect("reopen exact S1 control");
        let census = control
            .assess_read_only()
            .expect("restart S1 assessment")
            .into_blocking_control_census(&operation_nonce)
            .expect("restart blocking census");
        let epoch = FreshProcessEpochV3::establish().expect("fresh process epoch");
        let active = ReconciliationOperationStoreV3::open_existing(census, &epoch)
            .expect("replay exact admission roster")
            .begin_restart(&epoch)
            .expect("append restart epoch");
        assert!(!active.poisoned());
        drop(active);
        drop(control);
        assert_eq!(
            restart_admission_roster(&control_root, &operation_nonce).len(),
            ordinal,
        );
    }

    let roster = restart_admission_roster(&control_root, &operation_nonce);
    let root = control_root
        .join("operations")
        .join(format!("operation-{operation_nonce}"))
        .join("restart-admissions-v3");
    let records = roster
        .iter()
        .map(|name| {
            serde_json::from_slice::<serde_json::Value>(
                &std::fs::read(root.join(name)).expect("read restart admission"),
            )
            .expect("parse restart admission")
        })
        .collect::<Vec<_>>();
    assert_ne!(
        records[0]["restart_epoch_nonce"], records[1]["restart_epoch_nonce"],
        "each V2 RestartStarted must bind a fresh V3 restart epoch"
    );
    assert_ne!(
        records[0]["process_epoch_sha256"], records[1]["process_epoch_sha256"],
        "each admission must bind its exact process epoch"
    );
}

#[test]
fn active_collector_rejects_inventory_mutation_and_cross_epoch_seed_transplant() {
    let _lock = live_collector_test_lock();
    let fixture = LiveCollectorFixture::new();
    let operation_nonce = "7".repeat(64);
    let control_root = fixture._fixture.path().join("control-seed");
    publish_exact_prepared_operation(&fixture, &control_root, &operation_nonce);
    let control = LivePrivilegedDisposablePolicyV2::create_for_test(&control_root)
        .expect("reopen exact S1 control");
    let census = control
        .assess_read_only()
        .expect("restart S1 assessment")
        .into_blocking_control_census(&operation_nonce)
        .expect("restart blocking census");
    let epoch = FreshProcessEpochV3::establish().expect("fresh process epoch");
    let mut active = ReconciliationOperationStoreV3::open_existing(census, &epoch)
        .expect("open NeedsRestartEpoch")
        .begin_restart(&epoch)
        .expect("begin admitted restart");

    assert!(
        active.reject_cross_epoch_collector_seed_for_test().is_err(),
        "a held live-before seed must remain bound to its exact Active owner"
    );
    assert!(
        active
            .collect_reconciliation_with_post_capture_substitution_for_test()
            .is_err(),
        "a post-capture inventory substitution must fail the exact admitted-seed comparison"
    );
    active.poison_live_before_for_test();
    assert!(
        active.collect_reconciliation_snapshot().is_err(),
        "inventory mutation between admission and first collection must fail closed"
    );
    assert!(
        std::fs::read_dir(&fixture.persistence_root)
            .expect("receipt root")
            .next()
            .is_none(),
        "failed seed replay must not publish a collector receipt"
    );
}

#[test]
fn active_collector_reservation_blocks_duplicate_pending_even_after_drop() {
    let _lock = live_collector_test_lock();
    let fixture = LiveCollectorFixture::new();
    let operation_nonce = "7".repeat(64);
    let control_root = fixture._fixture.path().join("control-reservation");
    publish_exact_prepared_operation(&fixture, &control_root, &operation_nonce);
    let control = LivePrivilegedDisposablePolicyV2::create_for_test(&control_root)
        .expect("reopen exact S1 control");
    let census = control
        .assess_read_only()
        .expect("restart S1 assessment")
        .into_blocking_control_census(&operation_nonce)
        .expect("restart blocking census");
    let epoch = FreshProcessEpochV3::establish().expect("fresh process epoch");
    let mut active = ReconciliationOperationStoreV3::open_existing(census, &epoch)
        .expect("open NeedsRestartEpoch")
        .begin_restart(&epoch)
        .expect("begin admitted restart");

    let pending = active
        .collect_reconciliation_snapshot()
        .expect("first exact collector reservation");
    assert!(
        active.collect_reconciliation_snapshot().is_err(),
        "one Active epoch may not mint two simultaneous Pending observations"
    );
    drop(pending);
    assert!(
        active.collect_reconciliation_snapshot().is_err(),
        "dropping Pending must remain fail-closed instead of silently restoring the seed"
    );
}

#[test]
fn first_lineage_receipt_unlink_or_same_byte_replacement_is_permanently_rejected() {
    let _lock = live_collector_test_lock();
    for replace in [false, true] {
        let _control_fixture = tempfile::Builder::new()
            .prefix(".restart-collector-redteam-control-")
            .tempdir_in(env!("CARGO_MANIFEST_DIR"))
            .expect("create independent control-root owner");
        let mut fixture = LiveCollectorFixture::new();
        let operation_nonce = fixture.bindings.operation_nonce.clone();
        let control_root = _control_fixture.path().join(if replace {
            "control-replace"
        } else {
            "control-unlink"
        });
        let (receipt_path, bytes, _receipt) =
            persist_g1_and_drop_all_capsules(&mut fixture, &control_root, &operation_nonce);
        std::fs::remove_file(&receipt_path).expect("unlink first receipt");
        if replace {
            let mut replacement = std::fs::OpenOptions::new()
                .create_new(true)
                .write(true)
                .mode(0o600)
                .open(&receipt_path)
                .expect("create same-byte replacement receipt");
            replacement
                .write_all(&bytes)
                .expect("write replacement receipt");
            replacement.sync_all().expect("sync replacement receipt");
            std::fs::set_permissions(&receipt_path, std::fs::Permissions::from_mode(0o600))
                .expect("seal replacement receipt");
        }
        File::open(&fixture.persistence_root)
            .unwrap()
            .sync_all()
            .expect("sync mutated first-lineage root");
        let control = LivePrivilegedDisposablePolicyV2::create_for_test(&control_root)
            .expect("reopen exact S1 after first-lineage mutation");
        assert!(
            control.assess_read_only().is_err(),
            "first retained receipt mutation was accepted (replace={replace})"
        );
    }
}

#[test]
fn receipt_root_g0_g1_survives_drop_reopen_and_rejects_legal_same_byte_swap() {
    let _lock = live_collector_test_lock();
    let _control_fixture = tempfile::Builder::new()
        .prefix(".restart-collector-redteam-control-")
        .tempdir_in(env!("CARGO_MANIFEST_DIR"))
        .expect("create independent control-root owner");
    let mut fixture = LiveCollectorFixture::new();
    let operation_nonce = fixture.bindings.operation_nonce.clone();
    let control_root = _control_fixture.path().join("control-generation-reopen");
    let root_initial = lstat_binding(&fixture.persistence_root, "prepared receipt root")
        .expect("capture exact G0 root");
    let mut harness = open_receipt_generation_store(&mut fixture, &control_root, &operation_nonce);
    let (snapshot, first_receipt) = persist_snapshot_for_receipt_generation(&fixture, &mut harness);
    assert_eq!(first_receipt.match_result, ReconciliationMatchV2::Zero);
    let first_binding = snapshot
        .collector_receipt_file
        .as_ref()
        .expect("real G1 snapshot retains its final-stat receipt capsule");
    assert_eq!(first_binding.root_generation_ordinal(), 1);
    assert!(same_directory_object(
        root_initial,
        first_binding.root_after()
    ));
    assert_eq!(
        root_initial.nlink.checked_add(1),
        Some(first_binding.root_after().nlink)
    );
    assert_eq!(
        first_binding.root_after(),
        lstat_binding(&fixture.persistence_root, "G1 receipt root")
            .expect("capture exact G1 root endpoint")
    );
    let first_sha256 = sha256(&canonical_json(&first_receipt).unwrap());
    let first_path = fixture
        .persistence_root
        .join(format!("collector-{first_sha256}.json"));
    let first_bytes = std::fs::read(&first_path).expect("read retained G1 receipt");
    let first_inode = std::fs::metadata(&first_path).unwrap().ino();
    assert_eq!(
        std::fs::read_dir(&fixture.persistence_root)
            .unwrap()
            .count(),
        1,
        "real G1 publication must create exactly one retained receipt"
    );

    drop(harness);
    let control = LivePrivilegedDisposablePolicyV2::create_for_test(&control_root)
        .expect("reopen S1 after dropping all G1 live capabilities");
    let census = control
        .assess_read_only()
        .expect("replay exact G0->G1 root after full drop")
        .into_blocking_control_census(&operation_nonce)
        .expect("retain exact one-generation blocker");
    census
        .revalidate()
        .expect("revalidate exact one-generation S1");
    drop(census);
    drop(control);

    std::fs::remove_file(&first_path).expect("unlink retained G1 pathname");
    let mut replacement = std::fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .mode(0o600)
        .open(&first_path)
        .expect("create legal-mode same-byte G1 replacement");
    replacement
        .write_all(&first_bytes)
        .expect("write same G1 bytes");
    replacement.sync_all().expect("sync replacement G1 inode");
    drop(replacement);
    std::fs::set_permissions(&first_path, std::fs::Permissions::from_mode(0o600))
        .expect("retain legal production receipt mode");
    File::open(&fixture.persistence_root)
        .unwrap()
        .sync_all()
        .expect("sync receipt root after replacement");
    assert_ne!(first_inode, std::fs::metadata(&first_path).unwrap().ino());

    let control = LivePrivilegedDisposablePolicyV2::create_for_test(&control_root)
        .expect("open S1 control after path swap");
    let error = control
        .assess_read_only()
        .err()
        .expect("same-byte replacement must fail exact S1 replay");
    assert!(
        error.to_string().contains("collector receipt")
            || error.to_string().contains("receipt-root"),
        "drop-all-FD S1 replay accepted a legal-mode same-byte new receipt inode"
    );
}

#[test]
fn test_only_owner_g2_keeps_the_historical_g1_capsule_valid() {
    let _lock = live_collector_test_lock();
    let fixture = LiveCollectorFixture::new();
    let pending = collect_reconciliation_snapshot_v3(fixture.request())
        .expect("collect test-only G1 receipt");
    let mut retained = pending
        .persist_and_retain()
        .expect("persist test-only G1 receipt");
    let (root, second_receipt) = {
        let evidence = retained.evidence_mut();
        assert_eq!(
            evidence
                .durable
                .lifecycle_binding()
                .root_generation_ordinal(),
            1
        );
        let second = receipt_for_test_generation(&evidence.receipt, 2);
        let root = evidence
            .receipt_root
            .take()
            .expect("G1 evidence owns the unique receipt-root generation");
        (root, second)
    };
    let second_bytes = canonical_json(&second_receipt).expect("canonical G2 test receipt");
    let second_sha256 = sha256(&second_bytes);
    // This direct private call is a test-only owner harness. It proves the A
    // capsule invariant without adding production successor orchestration.
    let (second_durable, root) =
        DurableCollectorReceiptV3::persist(root, &second_receipt, second_bytes, &second_sha256)
            .expect("advance the test-only owner to G2");
    assert_eq!(
        second_durable.lifecycle_binding().root_generation_ordinal(),
        2
    );
    second_durable
        .revalidate(&root)
        .expect("G2 capsule is owned by the current root owner");
    retained.evidence_mut().receipt_root = Some(root);
    retained
        .revalidate()
        .expect("current G2 owner keeps its historical G1 capsule valid");
}

#[test]
fn receipt_root_capacity_allows_exact_64_reopen_but_rejects_any_65th_entry() {
    let _lock = live_collector_test_lock();
    let fixture = LiveCollectorFixture::new();
    let PendingRestartObservationV3 {
        guard: _,
        receipt: template,
        receipt_root,
    } = collect_reconciliation_snapshot_v3(fixture.request())
        .expect("collect capacity-test receipt template");
    let initial_binding = receipt_root.initial_binding;
    let stable_identity = receipt_root.stable_identity;
    drop(receipt_root);
    for ordinal in 1..=u64::try_from(MAX_RECEIPT_FILES).unwrap() {
        let receipt = receipt_for_test_generation(&template, ordinal);
        let bytes = canonical_json(&receipt).expect("canonical capacity-test receipt");
        let digest = sha256(&bytes);
        write_private(
            &fixture
                .persistence_root
                .join(format!("collector-{digest}.json")),
            &bytes,
        );
    }
    File::open(&fixture.persistence_root)
        .expect("open full test receipt root")
        .sync_all()
        .expect("sync full test receipt root");

    let full_root =
        RetainedReceiptRootV3::capture(&fixture.persistence_root, stable_identity, initial_binding)
            .expect("an exact 64-entry root remains reopenable");
    assert_eq!(full_root.snapshot.entries.len(), MAX_RECEIPT_FILES);
    let overflow_receipt =
        receipt_for_test_generation(&template, u64::try_from(MAX_RECEIPT_FILES).unwrap() + 1);
    let overflow_bytes = canonical_json(&overflow_receipt).expect("canonical overflow receipt");
    let overflow_sha256 = sha256(&overflow_bytes);
    assert!(
        DurableCollectorReceiptV3::persist(
            full_root,
            &overflow_receipt,
            overflow_bytes,
            &overflow_sha256,
        )
        .is_err(),
        "a full 64-entry owner admitted a 65th publication"
    );
    assert_eq!(
        std::fs::read_dir(&fixture.persistence_root)
            .expect("read full receipt root")
            .count(),
        MAX_RECEIPT_FILES
    );
    RetainedReceiptRootV3::capture(&fixture.persistence_root, stable_identity, initial_binding)
        .expect("failed 65th publication leaves the exact 64-entry root reopenable");

    let extra_bytes = b"test-only-over-capacity-entry";
    let extra_name = format!("collector-{}.json", sha256(extra_bytes));
    write_private(&fixture.persistence_root.join(extra_name), extra_bytes);
    assert!(
        RetainedReceiptRootV3::capture(
            &fixture.persistence_root,
            stable_identity,
            initial_binding,
        )
        .is_err(),
        "a 65-entry receipt root bypassed the closed-world reopen bound"
    );
}

#[test]
fn drop_all_fd_reopen_rejects_missing_orphan_temp_extra_and_net_zero_root_drift() {
    let _lock = live_collector_test_lock();
    for mutation in ["missing", "orphan", "temp", "extra", "endpoint-drift"] {
        let _control_fixture = tempfile::Builder::new()
            .prefix(".restart-collector-redteam-control-")
            .tempdir_in(env!("CARGO_MANIFEST_DIR"))
            .expect("create independent control-root owner");
        let mut fixture = LiveCollectorFixture::new();
        let operation_nonce = fixture.bindings.operation_nonce.clone();
        let control_root = _control_fixture
            .path()
            .join(format!("control-reopen-{mutation}"));
        let (receipt_path, _receipt_bytes, receipt) =
            persist_g1_and_drop_all_capsules(&mut fixture, &control_root, &operation_nonce);
        match mutation {
            "missing" => {
                std::fs::remove_file(&receipt_path).expect("remove referenced receipt");
            }
            "orphan" => {
                let mut orphan = receipt;
                orphan.monotonic_before_nanoseconds += 10;
                orphan.monotonic_after_nanoseconds += 10;
                validate_receipt(&orphan).expect("modeled orphan receipt remains valid");
                let bytes = canonical_json(&orphan).unwrap();
                let digest = sha256(&bytes);
                let path = fixture
                    .persistence_root
                    .join(format!("collector-{digest}.json"));
                write_private(&path, &bytes);
            }
            "temp" => {
                write_private(
                    &fixture.persistence_root.join(".incoming-collector-test"),
                    b"temporary",
                );
            }
            "extra" => {
                write_private(&fixture.persistence_root.join("unexpected"), b"extra");
            }
            "endpoint-drift" => {
                let drift = fixture.persistence_root.join(".root-generation-drift");
                write_private(&drift, b"drift");
                std::fs::remove_file(&drift).expect("remove net-zero roster mutation");
            }
            _ => unreachable!(),
        }
        File::open(&fixture.persistence_root)
            .unwrap()
            .sync_all()
            .expect("sync mutated receipt root");
        let control = LivePrivilegedDisposablePolicyV2::create_for_test(&control_root)
            .expect("open S1 control for drop-all-FD replay");
        let error = control
            .assess_read_only()
            .err()
            .unwrap_or_else(|| panic!("{mutation} survived exact drop-all-FD S1 reopen"));
        assert!(
            error.to_string().contains("collector receipt")
                || error.to_string().contains("receipt-root"),
            "{mutation} failed for an unrelated reason: {error}"
        );
    }
}

#[test]
fn restart_admission_replay_rejects_missing_orphan_and_same_bytes_inode_swap() {
    let _lock = live_collector_test_lock();
    let operation_nonce = "7".repeat(64);

    // A V2 RestartStarted without its exact V3 peer is blocking even when no
    // temporary file remains.
    {
        let fixture = LiveCollectorFixture::new();
        let control_root = fixture._fixture.path().join("control-missing-admission");
        publish_exact_prepared_operation(&fixture, &control_root, &operation_nonce);
        let control = LivePrivilegedDisposablePolicyV2::create_for_test(&control_root)
            .expect("open missing-admission control");
        let census = control
            .assess_read_only()
            .expect("missing-admission initial assessment")
            .into_blocking_control_census(&operation_nonce)
            .expect("missing-admission initial census");
        let epoch = FreshProcessEpochV3::establish().expect("missing-admission epoch");
        let active = ReconciliationOperationStoreV3::open_existing(census, &epoch)
            .expect("missing-admission Needs")
            .begin_restart(&epoch)
            .expect("persist pair before removing V3");
        drop(active);
        drop(control);
        let admission_root = control_root
            .join("operations")
            .join(format!("operation-{operation_nonce}"))
            .join("restart-admissions-v3");
        let name = restart_admission_roster(&control_root, &operation_nonce).remove(0);
        std::fs::remove_file(admission_root.join(name)).expect("remove exact V3 peer");
        let replay = LivePrivilegedDisposablePolicyV2::create_for_test(&control_root)
            .expect("reopen missing-admission control");
        assert!(
            replay.assess_read_only().is_err(),
            "missing V3 peer must fail the V2↔V3 bijection"
        );
    }

    // An extra canonical-looking, same-byte admission is still an orphan and
    // cannot acquire meaning merely from its filename/digest.
    {
        let fixture = LiveCollectorFixture::new();
        let control_root = fixture._fixture.path().join("control-orphan-admission");
        publish_exact_prepared_operation(&fixture, &control_root, &operation_nonce);
        let control = LivePrivilegedDisposablePolicyV2::create_for_test(&control_root)
            .expect("open orphan-admission control");
        let census = control
            .assess_read_only()
            .expect("orphan-admission initial assessment")
            .into_blocking_control_census(&operation_nonce)
            .expect("orphan-admission initial census");
        let epoch = FreshProcessEpochV3::establish().expect("orphan-admission epoch");
        let active = ReconciliationOperationStoreV3::open_existing(census, &epoch)
            .expect("orphan-admission Needs")
            .begin_restart(&epoch)
            .expect("persist exact pair before orphan injection");
        drop(active);
        drop(control);
        let admission_root = control_root
            .join("operations")
            .join(format!("operation-{operation_nonce}"))
            .join("restart-admissions-v3");
        let first = restart_admission_roster(&control_root, &operation_nonce).remove(0);
        let bytes = std::fs::read(admission_root.join(&first)).expect("read first admission");
        let digest = first
            .strip_suffix(".json")
            .and_then(|name| name.rsplit_once('-'))
            .map(|(_, digest)| digest)
            .expect("canonical first admission name");
        let orphan = admission_root.join(format!("restart-{:020}-{digest}.json", 2usize));
        let mut file = std::fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .mode(0o400)
            .open(&orphan)
            .expect("create same-byte orphan admission");
        file.write_all(&bytes).expect("write orphan admission");
        file.sync_all().expect("sync orphan admission");
        std::fs::set_permissions(&orphan, std::fs::Permissions::from_mode(0o400))
            .expect("seal orphan admission");
        File::open(&admission_root)
            .expect("open admission root")
            .sync_all()
            .expect("sync admission root");
        let replay = LivePrivilegedDisposablePolicyV2::create_for_test(&control_root)
            .expect("reopen orphan-admission control");
        assert!(
            replay.assess_read_only().is_err(),
            "an extra canonical same-byte V3 record must remain an orphan"
        );
    }

    // A retained Active census binds the exact admission inode, not only its
    // bytes. Replacing that pathname with a same-byte inode is detected before
    // the first collector can mint Pending.
    {
        let fixture = LiveCollectorFixture::new();
        let control_root = fixture._fixture.path().join("control-inode-swap");
        publish_exact_prepared_operation(&fixture, &control_root, &operation_nonce);
        let control = LivePrivilegedDisposablePolicyV2::create_for_test(&control_root)
            .expect("open inode-swap control");
        let census = control
            .assess_read_only()
            .expect("inode-swap initial assessment")
            .into_blocking_control_census(&operation_nonce)
            .expect("inode-swap initial census");
        let epoch = FreshProcessEpochV3::establish().expect("inode-swap epoch");
        let mut active = ReconciliationOperationStoreV3::open_existing(census, &epoch)
            .expect("inode-swap Needs")
            .begin_restart(&epoch)
            .expect("persist exact pair before inode swap");
        let admission_root = control_root
            .join("operations")
            .join(format!("operation-{operation_nonce}"))
            .join("restart-admissions-v3");
        let name = restart_admission_roster(&control_root, &operation_nonce).remove(0);
        let path = admission_root.join(name);
        let bytes = std::fs::read(&path).expect("read retained admission bytes");
        std::fs::remove_file(&path).expect("unlink retained admission inode");
        let mut replacement = std::fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .mode(0o400)
            .open(&path)
            .expect("create same-byte replacement inode");
        replacement
            .write_all(&bytes)
            .expect("write same admission bytes");
        replacement.sync_all().expect("sync replacement admission");
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o400))
            .expect("seal replacement admission");
        assert!(
            active.collect_reconciliation_snapshot().is_err(),
            "same-byte admission inode replacement must invalidate retained S1"
        );
    }
}

#[test]
fn prepared_lifecycle_wiring_fails_closed_if_live_capability_drifts() {
    let _lock = live_collector_test_lock();
    let fixture = LiveCollectorFixture::new();
    let operation_nonce = "7".repeat(64);
    let control_root = fixture._fixture.path().join("control");
    let control = LivePrivilegedDisposablePolicyV2::create_for_test(&control_root)
        .expect("create exact S1 control");
    let prepared = fixture.prepared_capability(&operation_nonce);
    std::fs::write(&fixture.backing_path, b"changed-after-prepared-capture")
        .expect("mutate backing after prepared capture");

    let census = control
        .assess_read_only()
        .expect("fresh S1 assessment")
        .into_fresh_control_census()
        .expect("fresh S1 census");
    assert!(
        CensusBoundDurableLifecycleStoreV3::create_prepared(census, prepared).is_err(),
        "live drift must return a closed error instead of panicking or publishing an operation"
    );
    assert_eq!(
        std::fs::read_dir(control_root.join("operations"))
            .expect("operations roster")
            .count(),
        0
    );
}

#[test]
fn prepared_collector_rejects_full_component_generation_or_birthtime_drift() {
    let _lock = live_collector_test_lock();
    let fixture = LiveCollectorFixture::new();
    let operation_nonce = "7".repeat(64);
    let retained = fixture.prepared_capability(&operation_nonce);
    let profile_sha256 = retained.profile_sha256().to_string();
    for (label, terminal, generation) in [
        ("ancestor-generation", false, true),
        ("ancestor-birthtime", false, false),
        ("terminal-generation", true, true),
        ("terminal-birthtime", true, false),
    ] {
        let mut changed = retained.manifest.clone();
        let index = if terminal {
            changed.backing_exact.opened_components.len() - 1
        } else {
            0
        };
        let binding = &mut changed.backing_exact.opened_components[index].binding;
        if generation {
            binding.generation = binding.generation.wrapping_add(1);
        } else {
            binding.birthtime_nanoseconds = (binding.birthtime_nanoseconds + 1) % 1_000_000_000;
        }
        let (bytes, digest) =
            canonical_prepared_manifest(&changed).expect("modeled full binding remains canonical");
        assert!(
            RetainedPreparedCollectorCapabilityV3::reopen_from_exact_manifest(
                &operation_nonce,
                &bytes,
                &digest,
                &profile_sha256,
            )
            .is_err(),
            "live replay accepted modeled {label} drift"
        );
    }
}

#[test]
fn prepared_collector_manifest_keeps_historical_boot_separate_from_restart_boot() {
    let _lock = live_collector_test_lock();
    let fixture = LiveCollectorFixture::new();
    let operation_nonce = "7".repeat(64);
    let retained = fixture.prepared_capability(&operation_nonce);
    let mut historical = retained.manifest.clone();
    historical.prepared_boot_session_uuid = "12345678-1234-4abc-8abc-123456789abc".to_string();
    historical.baseline.boot_session_uuid = historical.prepared_boot_session_uuid.clone();
    assert_ne!(
        historical.prepared_boot_session_uuid,
        current_boot_session_uuid().expect("current restart boot")
    );
    let (bytes, digest) = canonical_prepared_manifest(&historical)
        .expect("historical prepared boot remains a valid durable commitment");
    drop(retained);
    let reopened = RetainedPreparedCollectorCapabilityV3::reopen_from_exact_manifest(
        &operation_nonce,
        &bytes,
        &digest,
        &historical.profile_sha256,
    )
    .expect("reopen exact historical-boot prepared candidate");
    assert_eq!(
        reopened.boot_session_uuid(),
        historical.prepared_boot_session_uuid
    );
}

#[test]
fn legacy_prepared_manifest_without_initial_root_round_trips_but_cannot_reopen_live() {
    let _lock = live_collector_test_lock();
    let fixture = LiveCollectorFixture::new();
    let operation_nonce = "7".repeat(64);
    let retained = fixture.prepared_capability(&operation_nonce);
    let mut legacy = retained.manifest.clone();
    legacy.receipt_root_initial_binding = None;
    let (bytes, digest) =
        canonical_prepared_manifest(&legacy).expect("legacy manifest remains canonical");
    assert!(
        !String::from_utf8(bytes.clone())
            .unwrap()
            .contains("receipt_root_initial_binding")
    );
    let replayed: PreparedCollectorManifestV3 = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(canonical_json(&replayed).unwrap(), bytes);
    drop(retained);
    std::fs::remove_dir_all(&fixture.artifact_root).expect("remove legacy artifact root");
    std::fs::remove_dir(&fixture.mountpoint_path).expect("remove legacy mountpoint");
    std::fs::remove_dir_all(&fixture.persistence_root).expect("remove legacy receipt root");
    let error = match RetainedPreparedCollectorCapabilityV3::reopen_from_exact_manifest(
        &operation_nonce,
        &bytes,
        &digest,
        &legacy.profile_sha256,
    ) {
        Ok(_) => panic!("legacy manifest without G0 must remain blocking/read-only"),
        Err(error) => error,
    };
    assert!(
        error
            .to_string()
            .contains("legacy prepared collector manifest cannot enter active receipt generation"),
        "legacy manifest must fail before observing any prepared path: {error}"
    );
}

#[test]
fn reconciliation_replay_accepts_historical_prepared_baseline_and_seals_current_boot_zero() {
    let _lock = live_collector_test_lock();
    let mut fixture = LiveCollectorFixture::new();
    let current_boot = fixture.bindings.boot_session_uuid.clone();
    let historical_boot = "12345678-1234-4abc-8abc-123456789abc".to_string();
    assert_ne!(historical_boot, current_boot);
    fixture.baseline.boot_session_uuid = historical_boot.clone();
    fixture.bindings.baseline_inventory_sha256 = fixture.baseline.sha256().unwrap();

    let pending = collect_reconciliation_snapshot_v3(fixture.request())
        .expect("historical prepared baseline must not poison current-boot collection");
    let receipt = pending.receipt();
    assert_eq!(
        receipt.baseline_inventory.boot_session_uuid,
        historical_boot
    );
    assert_eq!(receipt.boot_session_uuid, current_boot);
    assert_eq!(receipt.match_result, ReconciliationMatchV2::Zero);
    assert!(receipt.baseline_restored);
    assert_eq!(
        receipt.current_expected_absence_inventory.as_ref(),
        Some(&receipt.iomedia_inventory)
    );
    assert_eq!(
        receipt.current_expected_absence_inventory_sha256.as_deref(),
        Some(receipt.iomedia_evidence_sha256.as_str())
    );
    validate_receipt(receipt).expect("cross-boot receipt exact replay");

    let receipt_sha256 = sha256(&canonical_json(receipt).unwrap());
    let snapshot = reconciliation_snapshot_from_receipt(
        receipt,
        &receipt_sha256,
        synthetic_receipt_file_binding(&receipt_sha256),
    )
    .unwrap();
    assert_eq!(
        snapshot.current_expected_absence_inventory_sha256,
        Some(snapshot.iomedia_evidence_sha256)
    );
}

#[test]
fn prepared_collector_capability_rejects_live_candidate_mutation_or_swap() {
    let _lock = live_collector_test_lock();
    for mutation in ["backing", "artifact_roster", "receipt_mode", "mountpoint"] {
        let fixture = LiveCollectorFixture::new();
        let operation_nonce = "7".repeat(64);
        let retained = fixture.prepared_capability(&operation_nonce);
        let manifest_bytes = retained.manifest_bytes().to_vec();
        let manifest_sha256 = retained.manifest_sha256().to_string();
        let profile_sha256 = retained.profile_sha256().to_string();
        drop(retained);

        match mutation {
            "backing" => {
                let displaced = fixture.backing_path.with_extension("displaced");
                std::fs::rename(&fixture.backing_path, displaced).expect("displace backing");
                std::fs::write(&fixture.backing_path, b"rootless-redteam-backing-v3")
                    .expect("same-byte replacement backing");
            }
            "artifact_roster" => {
                std::fs::write(fixture.artifact_root.join("foreign"), b"foreign")
                    .expect("mutate artifact roster");
            }
            "receipt_mode" => {
                std::fs::set_permissions(
                    &fixture.persistence_root,
                    std::fs::Permissions::from_mode(0o755),
                )
                .expect("mutate receipt root mode");
            }
            "mountpoint" => {
                let displaced = fixture.mountpoint_path.with_extension("displaced");
                std::fs::rename(&fixture.mountpoint_path, displaced).expect("displace mountpoint");
                std::fs::create_dir(&fixture.mountpoint_path).expect("replace mountpoint");
            }
            _ => unreachable!(),
        }
        assert!(
            RetainedPreparedCollectorCapabilityV3::reopen_from_exact_manifest(
                &operation_nonce,
                &manifest_bytes,
                &manifest_sha256,
                &profile_sha256,
            )
            .is_err(),
            "prepared live mutation must fail: {mutation}"
        );
    }
}

#[test]
fn rootless_live_collection_rejects_same_path_prepared_root_replacement() {
    let _lock = live_collector_test_lock();

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
    assert!(
        collect_reconciliation_snapshot_v3(fixture.request()).is_err(),
        "same-path artifact-root replacement was accepted"
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
    let _lock = live_collector_test_lock();

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
    let _lock = live_collector_test_lock();
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
    let _lock = live_collector_test_lock();
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
    let _lock = live_collector_test_lock();
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

    let symlink_receipt = fixture
        .persistence_root
        .join(format!("collector-{}.json", "b".repeat(64)));
    symlink(&fixture.backing_path, &symlink_receipt).unwrap();
    assert!(collect_reconciliation_snapshot_v3(fixture.request()).is_err());
    std::fs::remove_file(&symlink_receipt).unwrap();

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
    let transient_receipt_path = fixture.persistence_root.join(format!(
        "collector-{}.json",
        sha256(&canonical_json(transient.receipt()).unwrap())
    ));
    let transient_marker_path = fixture.persistence_root.join("transient-create-delete");
    assert!(
        transient
            .persist_and_retain_with_hook(|| {
                std::fs::write(&transient_marker_path, b"transient")?;
                std::fs::remove_file(&transient_marker_path)?;
                Ok(())
            })
            .is_err()
    );
    assert!(
        transient_receipt_path.is_file(),
        "the post-publish cutpoint must leave an exact orphan receipt"
    );
    let orphan_blocker = collect_reconciliation_snapshot_v3(fixture.request()).unwrap();
    let error = match orphan_blocker.persist_and_retain() {
        Ok(_) => panic!("a receipt without an exact lifecycle binding admitted a successor"),
        Err(error) => error,
    };
    assert!(
        error
            .to_string()
            .contains("receipt-root owner lost a retained generation binding"),
        "an orphan receipt failed for the wrong reason: {error}"
    );
    assert_eq!(
        std::fs::read_dir(&fixture.persistence_root)
            .expect("read orphaned receipt root")
            .count(),
        1,
        "orphan rejection published another receipt"
    );
    std::fs::remove_file(&transient_receipt_path).unwrap();

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
    assert!(
        metadata_path.is_file(),
        "the metadata-drift cutpoint must leave an exact orphan receipt"
    );
    std::fs::remove_file(&metadata_path).unwrap();

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
    cross_baseline_receipt.baseline_restored = true;
    validate_receipt(&cross_baseline_receipt).unwrap();
    let cross_baseline_bytes = canonical_json(&cross_baseline_receipt).unwrap();
    let cross_baseline_sha = sha256(&cross_baseline_bytes);
    let cross_baseline_snapshot = reconciliation_snapshot_from_receipt(
        &cross_baseline_receipt,
        &cross_baseline_sha,
        synthetic_receipt_file_binding(&cross_baseline_sha),
    )
    .unwrap();
    let cross_baseline_path = fixture
        .persistence_root
        .join(format!("collector-{cross_baseline_sha}.json"));
    write_private(&cross_baseline_path, &cross_baseline_bytes);
    assert!(collect_fresh_absence_v3(fixture.request(), &cross_baseline_snapshot).is_err());
    std::fs::remove_file(&cross_baseline_path).unwrap();

    assert!(
        collect_fresh_absence_v3(fixture.request(), &snapshot).is_err(),
        "exact-profile FreshAbsence must fail closed until the retained backing is unlinked"
    );
    std::fs::remove_file(&snapshot_receipt).unwrap();
    assert!(retained_snapshot.revalidate().is_err());
    write_private(&snapshot_receipt, &snapshot_receipt_bytes);
}

#[test]
fn rootless_live_zero_requires_persistence_and_fails_closed_before_backing_unlink_transition() {
    let _lock = live_collector_test_lock();
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
    let mountpoint = fixture_root.join("mountpoint");
    let artifact_root = fixture_root.join("artifacts");
    std::fs::create_dir(&mountpoint).unwrap();
    std::fs::create_dir(&artifact_root).unwrap();
    let backing_path = artifact_root.join("operation-created.img");
    std::fs::write(&backing_path, b"rootless-restart-backing-v3").unwrap();

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
    let receipt_root_initial = lstat_binding(&persistence_root, "test initial receipt root")
        .expect("capture test-only G0 receipt-root binding");
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
    validate_receipt(&tampered)
        .expect("historical prepared boot is independent from current-boot restoration");
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
    append(
        DisposableLifecycleEventV2::OperationPreparedWithManifestV3 {
            baseline_inventory_sha256: bindings.baseline_inventory_sha256.clone(),
            backing_identity_sha256: bindings.backing_identity_sha256.clone(),
            boot_session_uuid: bindings.boot_session_uuid.clone(),
            collector_policy_sha256: bindings.collector_policy_sha256.clone(),
            mountpoint_underlying_sha256: bindings.mountpoint_underlying_sha256.clone(),
            prepared_manifest: PreparedCollectorManifestBindingV3 {
                birthtime_nanoseconds: receipt_root_initial.birthtime_nanoseconds,
                birthtime_seconds: receipt_root_initial.birthtime_seconds,
                dev: receipt_root_initial.dev,
                generation: receipt_root_initial.generation,
                inode: receipt_root_initial.inode,
                receipt_root_initial: Some(receipt_root_initial),
                sha256: sha256(b"rootless-test-only-prepared-manifest-v3"),
            },
        },
    );
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

    assert!(
        collect_fresh_absence_v3(request(), &snapshot).is_err(),
        "FreshAbsence must remain fail-closed until the exact backing path is unlinked while its descriptor stays retained"
    );

    let mut receipt_names = std::fs::read_dir(&persistence_root)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .collect::<Vec<_>>();
    receipt_names.sort();
    assert_eq!(receipt_names.len(), 1);
    assert!(receipt_names.iter().all(|name| {
        name.starts_with("collector-") && name.ends_with(".json") && !name.starts_with(".incoming-")
    }));
}
