use super::*;
use crate::durable::canonical_json;
use crate::durable::sha256;
use crate::mac_apfs_barrier_fixture::AttachedTopologyV1;
use crate::mac_apfs_barrier_fixture::DiskArbitrationTerminalV1;
use crate::mac_apfs_barrier_fixture::DiskInventoryV1;
use crate::mac_apfs_barrier_fixture::DiskNodeV1;
use crate::mac_apfs_barrier_fixture::FileIdentityV1;
use crate::mac_apfs_barrier_fixture::MountPhaseV1;
use crate::mac_apfs_barrier_fixture::RawUnmountReceiptV1;
use crate::mac_apfs_barrier_fixture::StatFsFactsV1;
use crate::mac_disposable_lifecycle::DisposableLifecycleEventV2;
use crate::mac_disposable_lifecycle::DisposableLifecycleJournalV2;
use crate::mac_disposable_lifecycle::EffectPurposeV2;
use crate::mac_disposable_lifecycle::FreshAbsenceObservationV2;
use crate::mac_disposable_lifecycle::TerminalDispositionV2;
use crate::mac_disposable_lifecycle::fresh_absence_sha256;
use crate::mac_privileged_broker::ObjectBindingV1;
use std::ffi::CString;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::os::fd::AsRawFd;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

macro_rules! assert_not_impl {
    ($type:ty, $trait:path) => {
        const _: fn() = || {
            trait AmbiguousIfImpl<A> {
                fn marker() {}
            }
            impl<T: ?Sized> AmbiguousIfImpl<()> for T {}
            impl<T: ?Sized + $trait> AmbiguousIfImpl<u8> for T {}
            let _ = <$type as AmbiguousIfImpl<_>>::marker;
        };
    };
}

type BlockingCensusForCompileAssertions =
    RetainedControlCensusV3<'static, BlockingOperationV3, StableMountStateV3>;
type CompletedCensusForCompileAssertions =
    RetainedControlCensusV3<'static, CompletedOperationV3, StableMountStateV3>;
type PendingMountCensusForCompileAssertions =
    RetainedControlCensusV3<'static, BlockingOperationV3, PendingMountDeltaV3>;
type PendingUnmountCensusForCompileAssertions =
    RetainedControlCensusV3<'static, BlockingOperationV3, PendingUnmountDeltaV3>;
type RecoveredControlLeaseSealForCompileAssertions = RecoveredControlLeaseSealV3;
type AdoptedCollectorPairForCompileAssertions = S1AdoptedCollectorPairV3;
type CollectorPairSinkForCompileAssertions = CollectorReceiptLifecyclePairSinkV3<'static, 'static>;
assert_not_impl!(CollectorPairSinkForCompileAssertions, Clone);
assert_not_impl!(CollectorPairSinkForCompileAssertions, Send);
assert_not_impl!(CollectorPairSinkForCompileAssertions, Sync);
assert_not_impl!(CollectorPairSinkForCompileAssertions, serde::Serialize);
assert_not_impl!(
    CollectorPairSinkForCompileAssertions,
    serde::de::DeserializeOwned
);
assert_not_impl!(CollectorPairSinkForCompileAssertions, std::os::fd::AsRawFd);
assert_not_impl!(CollectorPairSinkForCompileAssertions, From<std::fs::File>);
assert_not_impl!(AdoptedCollectorPairForCompileAssertions, Clone);
assert_not_impl!(AdoptedCollectorPairForCompileAssertions, Send);
assert_not_impl!(AdoptedCollectorPairForCompileAssertions, Sync);
assert_not_impl!(AdoptedCollectorPairForCompileAssertions, serde::Serialize);
assert_not_impl!(
    AdoptedCollectorPairForCompileAssertions,
    serde::de::DeserializeOwned
);
assert_not_impl!(
    AdoptedCollectorPairForCompileAssertions,
    std::os::fd::AsRawFd
);
assert_not_impl!(
    AdoptedCollectorPairForCompileAssertions,
    From<std::fs::File>
);
assert_not_impl!(RecoveredControlLeaseSealForCompileAssertions, Clone);
assert_not_impl!(RecoveredControlLeaseSealForCompileAssertions, Send);
assert_not_impl!(RecoveredControlLeaseSealForCompileAssertions, Sync);
assert_not_impl!(
    RecoveredControlLeaseSealForCompileAssertions,
    serde::Serialize
);
assert_not_impl!(
    RecoveredControlLeaseSealForCompileAssertions,
    std::os::fd::AsRawFd
);
assert_not_impl!(
    RecoveredControlLeaseSealForCompileAssertions,
    From<std::fs::File>
);
assert_not_impl!(BlockingCensusForCompileAssertions, Clone);
assert_not_impl!(BlockingCensusForCompileAssertions, Send);
assert_not_impl!(BlockingCensusForCompileAssertions, Sync);
assert_not_impl!(BlockingCensusForCompileAssertions, serde::Serialize);
assert_not_impl!(BlockingCensusForCompileAssertions, std::os::fd::AsRawFd);
assert_not_impl!(BlockingCensusForCompileAssertions, From<std::fs::File>);
assert_not_impl!(CompletedCensusForCompileAssertions, Clone);
assert_not_impl!(CompletedCensusForCompileAssertions, Send);
assert_not_impl!(CompletedCensusForCompileAssertions, Sync);
assert_not_impl!(CompletedCensusForCompileAssertions, serde::Serialize);
assert_not_impl!(CompletedCensusForCompileAssertions, std::os::fd::AsRawFd);
assert_not_impl!(CompletedCensusForCompileAssertions, From<std::fs::File>);
assert_not_impl!(PendingMountCensusForCompileAssertions, Clone);
assert_not_impl!(PendingMountCensusForCompileAssertions, Send);
assert_not_impl!(PendingMountCensusForCompileAssertions, Sync);
assert_not_impl!(PendingMountCensusForCompileAssertions, serde::Serialize);
assert_not_impl!(
    PendingMountCensusForCompileAssertions,
    serde::de::DeserializeOwned
);
assert_not_impl!(PendingMountCensusForCompileAssertions, std::os::fd::AsRawFd);
assert_not_impl!(PendingMountCensusForCompileAssertions, From<std::fs::File>);
assert_not_impl!(
    PendingMountCensusForCompileAssertions,
    From<Vec<MountBindingV3>>
);
assert_not_impl!(PendingMountCensusForCompileAssertions, From<String>);
assert_not_impl!(PendingUnmountCensusForCompileAssertions, Clone);
assert_not_impl!(PendingUnmountCensusForCompileAssertions, Send);
assert_not_impl!(PendingUnmountCensusForCompileAssertions, Sync);
assert_not_impl!(PendingUnmountCensusForCompileAssertions, serde::Serialize);
assert_not_impl!(
    PendingUnmountCensusForCompileAssertions,
    serde::de::DeserializeOwned
);
assert_not_impl!(
    PendingUnmountCensusForCompileAssertions,
    std::os::fd::AsRawFd
);
assert_not_impl!(
    PendingUnmountCensusForCompileAssertions,
    From<std::fs::File>
);
assert_not_impl!(
    PendingUnmountCensusForCompileAssertions,
    From<Vec<MountBindingV3>>
);
assert_not_impl!(PendingUnmountCensusForCompileAssertions, From<String>);

fn digest(byte: char) -> String {
    byte.to_string().repeat(64)
}

fn test_mount_binding(id: i32, source: &str, target: &str) -> MountBindingV3 {
    MountBindingV3 {
        filesystem_id: [id, id],
        filesystem_type: "apfs".to_string(),
        mount_flags: 0,
        mount_from: source.to_string(),
        mount_on: target.to_string(),
    }
}

#[test]
fn exact_mount_typestate_accepts_only_stable_or_one_pending_pair() {
    let existing = test_mount_binding(1, "/dev/disk1s1", "/");
    let target = test_mount_binding(9, "/dev/disk9s1", "/private/tmp/hepta-mount");
    let before = vec![existing.clone()];
    let mut after = vec![existing, target.clone()];
    after.sort();
    let mut third = after.clone();
    third.push(test_mount_binding(
        10,
        "/dev/disk10s1",
        "/private/tmp/foreign",
    ));
    third.sort();

    assert!(
        StableMountStateV3
            .validate_current(&before, &before)
            .is_ok()
    );
    assert!(
        StableMountStateV3
            .validate_current(&before, &after)
            .is_err()
    );

    let mounting = PendingMountDeltaV3 {
        command_sha256: digest('a'),
        expected_after: after.clone(),
        target: target.clone(),
        _not_send_or_sync: PhantomData,
    };
    assert!(mounting.validate_current(&before, &before).is_ok());
    assert!(mounting.validate_current(&before, &after).is_ok());
    assert!(mounting.validate_current(&before, &third).is_err());

    let unmounting = PendingUnmountDeltaV3 {
        command_sha256: digest('b'),
        expected_after: before.clone(),
        target,
        _not_send_or_sync: PhantomData,
    };
    assert!(unmounting.validate_current(&after, &after).is_ok());
    assert!(unmounting.validate_current(&after, &before).is_ok());
    assert!(unmounting.validate_current(&after, &third).is_err());
}

#[test]
fn receipt_root_claim_must_match_exact_sidecar_before_external_replay() {
    assert!(require_receipt_root_claim_matches_sidecar(false, false).is_ok());
    assert!(require_receipt_root_claim_matches_sidecar(true, true).is_ok());
    for (claim, sidecar) in [(false, true), (true, false)] {
        let error = require_receipt_root_claim_matches_sidecar(claim, sidecar)
            .expect_err("Some/None mismatch must fail closed");
        assert!(error.to_string().contains("exact prepared sidecar"));
    }
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

fn write_operation_events(
    root: &Path,
    operation_nonce: &str,
    events: Vec<DisposableLifecycleEventV2>,
) {
    let mut journal = DisposableLifecycleJournalV2::new(operation_nonce).expect("journal");
    let mut records = Vec::new();
    for event in events {
        journal
            .append_with(event, |_, canonical| {
                records.push(canonical.to_vec());
                Ok(())
            })
            .expect("append lifecycle event");
    }

    let operation = root
        .join(OPERATIONS_NAME)
        .join(format!("{OPERATION_PREFIX}{operation_nonce}"));
    fs::create_dir(&operation).expect("create operation");
    fs::set_permissions(&operation, fs::Permissions::from_mode(0o700)).expect("set operation mode");
    for (index, bytes) in records.iter().enumerate() {
        let mut record = OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o400)
            .open(operation.join(format!("{:08}.json", index + 1)))
            .expect("create operation record");
        record.write_all(bytes).expect("write operation record");
        record.sync_all().expect("sync operation record");
    }
}

fn completed_operation_events(operation_nonce: &str) -> Vec<DisposableLifecycleEventV2> {
    let boot = "12345678-1234-1234-1234-123456789abc";
    let observation = FreshAbsenceObservationV2 {
        artifact_evidence_sha256: digest('5'),
        baseline_inventory_sha256: digest('1'),
        backing_identity_sha256: digest('2'),
        boot_session_uuid: boot.to_string(),
        collector_policy_sha256: digest('3'),
        collector_receipt_file: None,
        collector_receipt_sha256: digest('6'),
        current_expected_absence_inventory_sha256: None,
        iomedia_evidence_sha256: digest('7'),
        monotonic_after_nanoseconds: 11,
        monotonic_before_nanoseconds: 10,
        mount_evidence_sha256: digest('8'),
        mountpoint_underlying_sha256: digest('4'),
        no_matching_iomedia: true,
        no_nested_mounts: true,
        operation_nonce: operation_nonce.to_string(),
        operation_artifacts_absent: true,
        post_inventory_sha256: digest('1'),
        reconciliation_snapshot_sha256: None,
        restart_epoch_nonce: None,
    };
    let absence_sha256 = fresh_absence_sha256(&observation).expect("absence digest");
    vec![
        prepared_event(),
        DisposableLifecycleEventV2::CreateIssuedOrUncertain { effect_id: 1 },
        DisposableLifecycleEventV2::CreateObserved {
            effect_id: 1,
            image_identity_sha256: digest('9'),
        },
        DisposableLifecycleEventV2::AttachIssuedOrUncertain { effect_id: 2 },
        DisposableLifecycleEventV2::AttachObserved {
            effect_id: 2,
            topology_sha256: digest('a'),
        },
        DisposableLifecycleEventV2::MountIssuedOrUncertain { effect_id: 3 },
        DisposableLifecycleEventV2::MountObserved {
            effect_id: 3,
            mount_observation_sha256: digest('b'),
        },
        DisposableLifecycleEventV2::UnmountIssuedOrUncertain {
            effect_id: 4,
            purpose: EffectPurposeV2::ForwardFlow,
        },
        DisposableLifecycleEventV2::UnmountCallbackObserved {
            effect_id: 4,
            outcome: crate::mac_disposable_lifecycle::CallbackOutcomeV2::Succeeded,
        },
        DisposableLifecycleEventV2::UnmountObserved {
            effect_id: 4,
            mount_absence_sha256: digest('c'),
            collector: None,
        },
        DisposableLifecycleEventV2::EjectIssuedOrUncertain {
            effect_id: 5,
            purpose: EffectPurposeV2::ForwardFlow,
        },
        DisposableLifecycleEventV2::EjectCallbackObserved {
            effect_id: 5,
            outcome: crate::mac_disposable_lifecycle::CallbackOutcomeV2::Succeeded,
        },
        DisposableLifecycleEventV2::EjectObserved {
            effect_id: 5,
            iomedia_absence_sha256: digest('d'),
            collector: None,
        },
        DisposableLifecycleEventV2::FreshAbsenceObserved { observation },
        DisposableLifecycleEventV2::TerminalAbsenceProved {
            disposition: TerminalDispositionV2::Completed,
            fresh_absence_sha256: absence_sha256,
        },
    ]
}

fn v1_binding(uid: u32, gid: u32, mode: u32) -> ObjectBindingV1 {
    ObjectBindingV1 {
        ctime_nanoseconds: 2,
        ctime_seconds: 1,
        dev: 7,
        flags: 0,
        gid,
        inode: 11,
        mode,
        mtime_nanoseconds: 4,
        mtime_seconds: 3,
        nlink: 1,
        size: 512,
        uid,
    }
}

fn v1_inventory() -> DiskInventoryV1 {
    DiskInventoryV1 {
        all_disks: vec![
            "disk1".to_string(),
            "disk1s1".to_string(),
            "disk1s2".to_string(),
            "disk2".to_string(),
        ],
        all_whole_disks: vec!["disk1".to_string()],
        command_receipt_sha256: digest('c'),
        hdiutil_backing_paths: Vec::new(),
        hdiutil_info_command_sha256: digest('d'),
        schema: "hepta_mac_disk_inventory_v1".to_string(),
        t5_apfs_container_reference: "disk2".to_string(),
        t5_device_identifier: "disk1s1".to_string(),
        t5_parent_whole_disk: "disk1".to_string(),
        t5_physical_store_identifier: "disk1s2".to_string(),
        t5_volume_uuid: "fb804d1b-24cb-4d6e-aea7-a9e180807758".to_string(),
    }
}

fn v1_image() -> FileIdentityV1 {
    FileIdentityV1 {
        binding: v1_binding(0, 0, 0o600),
        path: "/Volumes/T5/frozen-v1-test.dmg".to_string(),
        sha256: digest('3'),
    }
}

fn v1_disk_node(identifier: &str, parent: &str, whole: bool) -> DiskNodeV1 {
    DiskNodeV1 {
        device_identifier: identifier.to_string(),
        device_node: format!("/dev/{identifier}"),
        disk_image: true,
        parent_whole_disk: parent.to_string(),
        size: 1_073_741_824,
        virtual_or_physical: "Virtual".to_string(),
        whole,
    }
}

fn v1_topology() -> AttachedTopologyV1 {
    let image = v1_image();
    let inventory = v1_inventory();
    AttachedTopologyV1 {
        apfs_container: v1_disk_node("disk10", "disk10", true),
        apfs_container_uuid: "11111111-1111-4111-8111-111111111111".to_string(),
        apfs_volume: v1_disk_node("disk10s1", "disk10", false),
        apfs_volume_uuid: "22222222-2222-4222-8222-222222222222".to_string(),
        hdiutil_info_command_sha256: digest('5'),
        image_backing_after: image.clone(),
        image_backing_before: image.clone(),
        image_path_from_hdiutil: image.path,
        iomedia_identity: None,
        iomedia_provenance: None,
        physical_store: v1_disk_node("disk9s1", "disk9", false),
        pre_attach_inventory_sha256: sha256(
            &canonical_json(&inventory).expect("canonical v1 inventory"),
        ),
        schema: "hepta_mac_attached_apfs_topology_v1".to_string(),
        whole_disk: v1_disk_node("disk9", "disk9", true),
    }
}

fn v1_mount_statfs(phase: MountPhaseV1) -> StatFsFactsV1 {
    const MNT_RDONLY: u64 = 0x0000_0001;
    const MNT_NOEXEC: u64 = 0x0000_0004;
    const MNT_NOSUID: u64 = 0x0000_0008;
    const MNT_NODEV: u64 = 0x0000_0010;
    const MNT_NOATIME: u64 = 0x1000_0000;
    StatFsFactsV1 {
        filesystem_id: [71, 72],
        filesystem_type: "apfs".to_string(),
        mount_flags: MNT_NODEV
            | MNT_NOSUID
            | MNT_NOEXEC
            | MNT_NOATIME
            | if phase == MountPhaseV1::ReadOnly {
                MNT_RDONLY
            } else {
                0
            },
        mount_from: "/dev/disk10s1".to_string(),
        mount_on: "/Volumes/T5/frozen-v1-test-mount".to_string(),
    }
}

fn v1_unmount() -> RawUnmountReceiptV1 {
    RawUnmountReceiptV1 {
        duration_microseconds: 1,
        errno: 0,
        flags: 0,
        mountpoint: "/Volumes/T5/frozen-v1-test-mount".to_string(),
        rc: 0,
    }
}

fn v1_terminal() -> DiskArbitrationTerminalV1 {
    DiskArbitrationTerminalV1 {
        devnode_lstat_errno: libc::ENOENT,
        diskutil_info_command_sha256: digest('6'),
        diskutil_info_exit_code: 1,
        hdiutil_info_command_sha256: digest('7'),
        mountpoint_underlying_after: v1_binding(0, 0, 0o700),
        nested_mounts_after: Vec::new(),
        post_inventory: v1_inventory(),
        schema: "hepta_mac_diskarbitration_terminal_v1".to_string(),
        whole_disk_identifier: "disk9".to_string(),
    }
}

fn write_canonical_file(path: &Path, value: &impl serde::Serialize) {
    let bytes = canonical_json(value).expect("canonical historical bytes");
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o400)
        .open(path)
        .expect("create historical record");
    file.write_all(&bytes).expect("write historical record");
    file.sync_all().expect("sync historical record");
}

fn write_active_historical_obligation(publication: &Path, nonce: &str) {
    let obligation = publication.join(format!("{HISTORICAL_OBLIGATION_PREFIX}{nonce}"));
    fs::create_dir(&obligation).expect("create historical obligation");
    fs::set_permissions(&obligation, fs::Permissions::from_mode(0o700))
        .expect("set historical obligation mode");
    let record = AttachmentObligationRecordV1 {
        authority_granted: false,
        boot_session_uuid: "12345678-1234-1234-1234-123456789abc".to_string(),
        challenge_sha256: digest('e'),
        disposition: ObligationDispositionV1::Active,
        epoch_receipt_sha256: digest('f'),
        event: AttachmentObligationEventV1::Prepared {
            image_backing: FileIdentityV1 {
                binding: v1_binding(0, 0, 0o400),
                path: "/Volumes/T5/frozen-v1-test.dmg".to_string(),
                sha256: digest('1'),
            },
            mountpoint_underlying: v1_binding(0, 0, 0o700),
            namespace_statfs: StatFsFactsV1 {
                filesystem_id: [1, 2],
                filesystem_type: "apfs".to_string(),
                mount_flags: 0,
                mount_from: "/dev/disk1s1".to_string(),
                mount_on: "/Volumes/T5".to_string(),
            },
            nested_mounts_before: Vec::new(),
            pre_attach_inventory: v1_inventory(),
        },
        operation_nonce: nonce.to_string(),
        previous_record_sha256: None,
        schema: "hepta_mac_attachment_obligation_record_v1".to_string(),
        sequence: 1,
    };
    write_canonical_file(&obligation.join("00000001.json"), &record);
}

fn write_reconciled_historical_obligation(
    publication: &Path,
    nonce: &str,
    boot_session_uuid: &str,
) {
    let obligation = publication.join(format!("{HISTORICAL_OBLIGATION_PREFIX}{nonce}"));
    fs::create_dir(&obligation).expect("create historical obligation");
    fs::set_permissions(&obligation, fs::Permissions::from_mode(0o700))
        .expect("set historical obligation mode");

    let mut events = vec![(
        AttachmentObligationEventV1::Prepared {
            image_backing: v1_image(),
            mountpoint_underlying: v1_binding(0, 0, 0o700),
            namespace_statfs: StatFsFactsV1 {
                filesystem_id: [1, 2],
                filesystem_type: "apfs".to_string(),
                mount_flags: 0,
                mount_from: "/dev/disk1s2".to_string(),
                mount_on: "/Volumes/T5".to_string(),
            },
            nested_mounts_before: Vec::new(),
            pre_attach_inventory: v1_inventory(),
        },
        ObligationDispositionV1::Active,
    )];
    for phase in [MountPhaseV1::ReadWrite, MountPhaseV1::ReadOnly] {
        events.extend([
            (
                AttachmentObligationEventV1::AttachStarted { phase },
                ObligationDispositionV1::Active,
            ),
            (
                AttachmentObligationEventV1::Attached {
                    phase,
                    topology: v1_topology(),
                },
                ObligationDispositionV1::Active,
            ),
            (
                AttachmentObligationEventV1::MountStarted {
                    phase,
                    volume_identifier: "disk10s1".to_string(),
                },
                ObligationDispositionV1::Active,
            ),
            (
                AttachmentObligationEventV1::Mounted {
                    mountpoint_statfs: v1_mount_statfs(phase),
                    phase,
                },
                ObligationDispositionV1::Active,
            ),
            (
                AttachmentObligationEventV1::UnmountStarted { phase },
                ObligationDispositionV1::Active,
            ),
            (
                AttachmentObligationEventV1::Unmounted {
                    phase,
                    receipt: v1_unmount(),
                },
                ObligationDispositionV1::Active,
            ),
            (
                AttachmentObligationEventV1::DetachStarted {
                    phase,
                    whole_disk_identifier: "disk9".to_string(),
                },
                ObligationDispositionV1::Active,
            ),
            (
                AttachmentObligationEventV1::DiskArbitrationGone {
                    phase,
                    terminal: v1_terminal(),
                },
                ObligationDispositionV1::Active,
            ),
        ]);
    }
    events.push((
        AttachmentObligationEventV1::TerminalReconciled {
            post_inventory: v1_inventory(),
        },
        ObligationDispositionV1::Reconciled,
    ));

    let mut previous_record_sha256 = None;
    for (index, (event, disposition)) in events.into_iter().enumerate() {
        let record = AttachmentObligationRecordV1 {
            authority_granted: false,
            boot_session_uuid: boot_session_uuid.to_string(),
            challenge_sha256: digest('8'),
            disposition,
            epoch_receipt_sha256: digest('9'),
            event,
            operation_nonce: nonce.to_string(),
            previous_record_sha256: previous_record_sha256.clone(),
            schema: "hepta_mac_attachment_obligation_record_v1".to_string(),
            sequence: u32::try_from(index + 1).expect("v1 record sequence"),
        };
        let bytes = canonical_json(&record).expect("canonical reconciled obligation record");
        write_canonical_file(&obligation.join(format!("{:08}.json", index + 1)), &record);
        previous_record_sha256 = Some(sha256(&bytes));
    }
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
    assert!(receipt.new_operation_precondition_satisfied);
    assert!(receipt.blocking_operation_nonces.is_empty());
    assert!(receipt.completed_operation_nonces.is_empty());
}

#[test]
fn one_policy_and_flock_can_cast_only_one_live_s1_assessment() {
    let temporary = tempfile::tempdir().expect("temporary root parent");
    let root = temporary.path().join("control");
    let control = LivePrivilegedDisposablePolicyV2::create_for_test(&root).expect("open control");
    let first = control.assess_read_only().expect("first linear assessment");
    assert!(control.assess_read_only().is_err());
    drop(first);
    assert!(control.assess_read_only().is_err());
}

#[test]
fn stable_mount_typestate_retains_the_exact_full_snapshot() {
    let temporary = tempfile::tempdir().expect("temporary root parent");
    let root = temporary.path().join("control");
    let control = LivePrivilegedDisposablePolicyV2::create_for_test(&root).expect("open control");
    let mut census = control
        .assess_read_only()
        .expect("read-only assessment")
        .into_fresh_control_census()
        .expect("stable exact mount census");
    let _: &RetainedControlCensusV3<'_, FreshAdmissionV3, StableMountStateV3> = &census;
    assert!(!census.exact_mounts.expected_full_snapshot.is_empty());
    census.exact_mounts.expected_full_snapshot.pop();
    assert!(census.revalidate().is_err());
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
    assert!(assessment.into_fresh_control_census().is_err());
}

#[test]
fn exact_blocking_census_rejects_unknown_completed_and_noncanonical_targets() {
    {
        let temporary = tempfile::tempdir().expect("temporary root parent");
        let root = create_root(temporary.path());
        let nonce = digest('a');
        write_operation(&root, &nonce, &nonce);
        let control =
            LivePrivilegedDisposablePolicyV2::create_for_test(&root).expect("open control");
        let assessment = control.assess_read_only().expect("blocking assessment");
        let error = assessment
            .into_blocking_control_census(&digest('b'))
            .err()
            .expect("unknown target must fail");
        assert!(error.to_string().contains("unknown"));
    }

    {
        let temporary = tempfile::tempdir().expect("temporary root parent");
        let root = create_root(temporary.path());
        let nonce = digest('c');
        write_operation_events(&root, &nonce, completed_operation_events(&nonce));
        let control =
            LivePrivilegedDisposablePolicyV2::create_for_test(&root).expect("open control");
        let assessment = control.assess_read_only().expect("completed assessment");
        assert_eq!(
            assessment.receipt().completed_operation_nonces,
            [nonce.clone()]
        );
        let error = assessment
            .into_blocking_control_census(&nonce)
            .err()
            .expect("completed target must fail");
        assert!(error.to_string().contains("completed"));
    }

    {
        let temporary = tempfile::tempdir().expect("temporary root parent");
        let root = create_root(temporary.path());
        let nonce = digest('d');
        write_operation(&root, &nonce, &nonce);
        let control =
            LivePrivilegedDisposablePolicyV2::create_for_test(&root).expect("open control");
        let assessment = control.assess_read_only().expect("blocking assessment");
        assert!(
            assessment
                .into_blocking_control_census(&nonce.to_ascii_uppercase())
                .is_err()
        );
    }
}

#[test]
fn blocking_census_rejects_duplicate_or_subset_receipt_claims() {
    {
        let temporary = tempfile::tempdir().expect("temporary root parent");
        let root = create_root(temporary.path());
        let nonce = digest('a');
        write_operation(&root, &nonce, &nonce);
        let control =
            LivePrivilegedDisposablePolicyV2::create_for_test(&root).expect("open control");
        let mut assessment = control.assess_read_only().expect("blocking assessment");
        assessment
            .receipt
            .blocking_operation_nonces
            .push(nonce.clone());
        assert!(assessment.into_blocking_control_census(&nonce).is_err());
    }

    {
        let temporary = tempfile::tempdir().expect("temporary root parent");
        let root = create_root(temporary.path());
        let first = digest('b');
        let second = digest('c');
        write_operation(&root, &first, &first);
        write_operation(&root, &second, &second);
        let control =
            LivePrivilegedDisposablePolicyV2::create_for_test(&root).expect("open control");
        let mut assessment = control.assess_read_only().expect("blocking assessment");
        assessment
            .receipt
            .blocking_operation_nonces
            .retain(|nonce| nonce == &first);
        assert!(assessment.into_blocking_control_census(&first).is_err());
    }
}

#[test]
fn selected_blocker_retains_every_other_operation_capsule() {
    let temporary = tempfile::tempdir().expect("temporary root parent");
    let root = create_root(temporary.path());
    let selected = digest('a');
    let other = digest('b');
    write_operation(&root, &selected, &selected);
    write_operation(&root, &other, &other);
    let control = LivePrivilegedDisposablePolicyV2::create_for_test(&root).expect("open control");
    let assessment = control
        .assess_read_only()
        .expect("multi-blocker assessment");
    let census = assessment
        .into_blocking_control_census(&selected)
        .expect("select one exact blocker");
    let _: &RetainedControlCensusV3<'_, BlockingOperationV3, StableMountStateV3> = &census;
    assert_eq!(census.assessment._operations.len(), 2);
    assert_eq!(
        census.assessment._operation_blockers,
        [selected, other.clone()]
    );

    let other_record = root
        .join(OPERATIONS_NAME)
        .join(format!("{OPERATION_PREFIX}{other}"))
        .join("00000001.json");
    fs::set_permissions(&other_record, fs::Permissions::from_mode(0o600))
        .expect("mutate non-selected blocker");
    assert!(census.revalidate().is_err());
}

#[test]
fn blocking_census_rejects_foreign_roster_and_selected_inode_replacement() {
    {
        let temporary = tempfile::tempdir().expect("temporary root parent");
        let root = create_root(temporary.path());
        let nonce = digest('a');
        write_operation(&root, &nonce, &nonce);
        let control =
            LivePrivilegedDisposablePolicyV2::create_for_test(&root).expect("open control");
        let census = control
            .assess_read_only()
            .expect("blocking assessment")
            .into_blocking_control_census(&nonce)
            .expect("blocking census");
        let foreign = root
            .join(OPERATIONS_NAME)
            .join(format!("{OPERATION_PREFIX}{}", digest('f')));
        fs::create_dir(&foreign).expect("foreign operation");
        fs::set_permissions(&foreign, fs::Permissions::from_mode(0o700))
            .expect("foreign operation mode");
        assert!(census.revalidate().is_err());
    }

    {
        let temporary = tempfile::tempdir().expect("temporary root parent");
        let root = create_root(temporary.path());
        let nonce = digest('b');
        write_operation(&root, &nonce, &nonce);
        let control =
            LivePrivilegedDisposablePolicyV2::create_for_test(&root).expect("open control");
        let census = control
            .assess_read_only()
            .expect("blocking assessment")
            .into_blocking_control_census(&nonce)
            .expect("blocking census");
        let original = root
            .join(OPERATIONS_NAME)
            .join(format!("{OPERATION_PREFIX}{nonce}"));
        fs::rename(&original, temporary.path().join("stale-operation"))
            .expect("move retained inode aside");
        write_operation(&root, &nonce, &nonce);
        assert!(census.revalidate().is_err());
    }
}

#[test]
fn blocking_census_admits_only_one_digest_bound_next_record() {
    {
        let temporary = tempfile::tempdir().expect("temporary root parent");
        let root = create_root(temporary.path());
        let nonce = digest('a');
        write_operation(&root, &nonce, &nonce);
        let control =
            LivePrivilegedDisposablePolicyV2::create_for_test(&root).expect("open control");
        let mut census = control
            .assess_read_only()
            .expect("blocking assessment")
            .into_blocking_control_census(&nonce)
            .expect("blocking census");
        let operation = root
            .join(OPERATIONS_NAME)
            .join(format!("{OPERATION_PREFIX}{nonce}"));
        for sequence in [2, 3] {
            let mut record = OpenOptions::new()
                .write(true)
                .create_new(true)
                .mode(0o400)
                .open(operation.join(format!("{sequence:08}.json")))
                .expect("inject extra record");
            record.write_all(b"unexpected").expect("write extra record");
            record.sync_all().expect("sync extra record");
        }
        assert!(
            census
                .admit_selected_lifecycle_append(&digest('f'))
                .is_err()
        );
    }

    {
        let temporary = tempfile::tempdir().expect("temporary root parent");
        let root = create_root(temporary.path());
        let nonce = digest('b');
        write_operation(&root, &nonce, &nonce);
        let control =
            LivePrivilegedDisposablePolicyV2::create_for_test(&root).expect("open control");
        let mut census = control
            .assess_read_only()
            .expect("blocking assessment")
            .into_blocking_control_census(&nonce)
            .expect("blocking census");
        let operation = root
            .join(OPERATIONS_NAME)
            .join(format!("{OPERATION_PREFIX}{nonce}"));
        let mut record = OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o400)
            .open(operation.join("00000002.json"))
            .expect("inject next record");
        record
            .write_all(b"digest mismatch")
            .expect("write next record");
        record.sync_all().expect("sync next record");
        assert!(
            census
                .admit_selected_lifecycle_append(&digest('f'))
                .is_err()
        );
    }
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
    fs::remove_file(operations.join("late-entry")).expect("restore test roster");
    assert_rejected(control.assess_read_only());
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
        let mut retained = RetainedFdBudget::after_existing(MAX_RETAINED_FDS, 0)
            .expect("fixture descriptor budget");
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
                assert_eq!(retained.retained(), 4);
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
    let mut retained =
        RetainedFdBudget::after_existing(MAX_RETAINED_FDS, 0).expect("barrier descriptor budget");
    let (roster, nodes) = control
        .open_barrier_journal(&barrier, &mut total_bytes, &mut retained)
        .expect("census barrier");
    assert_eq!(roster, ["00000000000000000001.json"]);
    assert_eq!(total_bytes, 3);
    assert_eq!(retained.retained(), 1);
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

#[test]
fn retained_fd_budget_stops_before_the_2049th_open_without_exhausting_process_fds() {
    let mut budget =
        RetainedFdBudget::after_existing(MAX_RETAINED_FDS, 0).expect("empty descriptor budget");
    for _ in 0..MAX_RETAINED_FDS {
        budget
            .reserve("modeled retained descriptor")
            .expect("within bound");
    }
    assert_eq!(budget.retained(), MAX_RETAINED_FDS);
    let error = budget
        .reserve("modeled 2049th retained descriptor")
        .expect_err("2049th descriptor must fail before open");
    assert!(error.to_string().contains("before opening"));
    assert!(RetainedFdBudget::after_existing(MAX_RETAINED_FDS, MAX_RETAINED_FDS + 1).is_err());

    let temporary = tempfile::tempdir().expect("temporary root parent");
    let root = create_root(temporary.path());
    let nonce = digest('a');
    write_operation(&root, &nonce, &nonce);
    let control = LivePrivilegedDisposablePolicyV2::create_for_test(&root).expect("reopen control");
    let error = match control.assess_read_only_with_fd_limit(6) {
        Ok(_) => panic!("assessment exceeded its injected retained-FD limit"),
        Err(error) => error,
    };
    assert!(
        error
            .to_string()
            .contains("before opening operation or obligation record")
    );
}

#[test]
fn sealed_fixtures_share_one_incremental_retained_fd_budget() {
    let temporary = tempfile::tempdir().expect("temporary fixture parent");
    create_sealed_fixture(temporary.path(), "fixture-a", None);
    create_sealed_fixture(temporary.path(), "fixture-b", None);
    let parent = open_path_directory(temporary.path()).expect("open fixture parent");
    let filesystem = filesystem_binding(&parent).expect("bind fixture filesystem");
    let mut budget = RetainedFdBudget::after_existing(7, 0).expect("shared fixture budget");
    let first = open_sealed_fixture_node(
        parent.as_raw_fd(),
        "fixture-a",
        ".",
        0,
        &mut budget,
        unsafe { libc::geteuid() },
        unsafe { libc::getegid() },
        &filesystem,
    )
    .expect("first fixture fits");
    assert_eq!(first.descriptor_count(), 4);
    assert_eq!(budget.retained(), 4);
    let second = open_sealed_fixture_node(
        parent.as_raw_fd(),
        "fixture-b",
        ".",
        0,
        &mut budget,
        unsafe { libc::geteuid() },
        unsafe { libc::getegid() },
        &filesystem,
    );
    assert!(second.is_err(), "second fixture reset the shared budget");
    assert_eq!(budget.retained(), 7);

    for name in ["fixture-a", "fixture-b"] {
        fs::set_permissions(
            temporary.path().join(name).join("logs"),
            fs::Permissions::from_mode(0o700),
        )
        .expect("unseal fixture logs");
        fs::set_permissions(
            temporary.path().join(name),
            fs::Permissions::from_mode(0o700),
        )
        .expect("unseal fixture");
    }
}

#[derive(Clone, Copy, Debug)]
enum HistoricalSemanticMutation {
    BarrierOrderGap,
    NonCanonicalPublication,
    PriorBootReconciled,
    TransplantedObligation,
    UnknownSemantics,
}

fn create_historical_root(parent: &Path, mutation: HistoricalSemanticMutation) -> String {
    let nonce = digest('a');
    let root = parent.join(format!("{HISTORICAL_ROOT_PREFIX}{nonce}"));
    let barrier = root.join("barrier-journal");
    let publication = root.join("publication");
    fs::create_dir(&root).expect("create historical root");
    fs::set_permissions(&root, fs::Permissions::from_mode(0o700))
        .expect("set historical root mode");
    fs::create_dir(&barrier).expect("create historical barrier");
    fs::set_permissions(&barrier, fs::Permissions::from_mode(0o700))
        .expect("set historical barrier mode");
    fs::create_dir(&publication).expect("create historical publication");
    fs::set_permissions(&publication, fs::Permissions::from_mode(0o700))
        .expect("set historical publication mode");

    let barrier_name = if matches!(mutation, HistoricalSemanticMutation::BarrierOrderGap) {
        "00000000000000000002.json"
    } else {
        "00000000000000000001.json"
    };
    write_canonical_file(&barrier.join(barrier_name), &serde_json::json!({}));

    let obligation_nonce = if matches!(mutation, HistoricalSemanticMutation::TransplantedObligation)
    {
        digest('b')
    } else {
        nonce.clone()
    };
    if matches!(mutation, HistoricalSemanticMutation::PriorBootReconciled) {
        let current_boot = current_boot_session_uuid().expect("current boot session UUID");
        let prior_boot = [
            "aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa",
            "bbbbbbbb-bbbb-4bbb-8bbb-bbbbbbbbbbbb",
        ]
        .into_iter()
        .find(|candidate| *candidate != current_boot)
        .expect("one fixed UUID differs from the current boot");
        write_reconciled_historical_obligation(&publication, &obligation_nonce, prior_boot);
    } else {
        write_active_historical_obligation(&publication, &obligation_nonce);
    }
    create_sealed_fixture(&publication, &format!("apfs-fixture-{nonce}"), None);
    for kind in ["prepared", "mechanism-receipt", "terminal-receipt"] {
        let path = publication.join(format!("hepta-operation-{nonce}.{kind}.json"));
        if matches!(
            mutation,
            HistoricalSemanticMutation::NonCanonicalPublication
        ) && kind == "prepared"
        {
            let mut file = OpenOptions::new()
                .write(true)
                .create_new(true)
                .mode(0o400)
                .open(path)
                .expect("create noncanonical publication record");
            file.write_all(b"{ }\n")
                .expect("write noncanonical publication record");
            file.sync_all()
                .expect("sync noncanonical publication record");
        } else {
            write_canonical_file(&path, &serde_json::json!({}));
        }
    }
    nonce
}

#[test]
fn historical_v1_unknown_or_noncanonical_semantics_are_explicit_blockers() {
    for mutation in [
        HistoricalSemanticMutation::UnknownSemantics,
        HistoricalSemanticMutation::NonCanonicalPublication,
    ] {
        let temporary = tempfile::tempdir().expect("temporary historical volume");
        let root = create_root(temporary.path());
        let nonce = create_historical_root(temporary.path(), mutation);
        let control =
            LivePrivilegedDisposablePolicyV2::create_for_test(&root).expect("reopen control");
        let assessment = control
            .assess_read_only()
            .expect("unproved frozen semantics remain a fail-closed receipt blocker");
        let receipt = assessment.receipt();
        assert!(!receipt.admission_authority);
        assert!(!receipt.authority.any());
        assert!(!receipt.new_operation_precondition_satisfied);
        assert!(receipt.historical_closure_bindings.is_empty());
        assert!(
            receipt
                .blocking_operation_nonces
                .iter()
                .any(|blocker| blocker.contains("historical_v1_barrier_semantics_unproved"))
        );
        assert!(
            receipt
                .blocking_operation_nonces
                .iter()
                .any(|blocker| blocker.contains("historical_v1_publication_semantics_unproved"))
        );
        assert!(
            receipt
                .blocking_operation_nonces
                .iter()
                .any(|blocker| blocker.contains("historical_v1_fixture"))
        );
        assert!(
            receipt
                .blocking_operation_nonces
                .iter()
                .any(|blocker| blocker.contains(&nonce))
        );
    }
}

#[test]
fn historical_v1_obligation_rejects_a_v3_sidecar_before_opening_it() {
    let temporary = tempfile::tempdir().expect("temporary historical volume");
    let root = create_root(temporary.path());
    let nonce = create_historical_root(
        temporary.path(),
        HistoricalSemanticMutation::UnknownSemantics,
    );
    let sidecar = temporary
        .path()
        .join(format!("{HISTORICAL_ROOT_PREFIX}{nonce}"))
        .join("publication")
        .join(format!("{HISTORICAL_OBLIGATION_PREFIX}{nonce}"))
        .join(EFFECT_ISSUE_ROOT_V3);
    write_canonical_file(&sidecar, &serde_json::json!({}));

    let control = LivePrivilegedDisposablePolicyV2::create_for_test(&root).expect("reopen control");
    let error = match control.assess_read_only() {
        Ok(_) => panic!("historical V1 obligation accepted a V3 sidecar"),
        Err(error) => error,
    };
    assert!(
        error
            .to_string()
            .contains("historical obligation contains a V3 operation sidecar"),
        "unexpected rejection: {error}"
    );
}

#[test]
fn historical_v1_order_and_nonce_transplants_fail_before_closure() {
    for mutation in [
        HistoricalSemanticMutation::BarrierOrderGap,
        HistoricalSemanticMutation::TransplantedObligation,
    ] {
        let temporary = tempfile::tempdir().expect("temporary historical volume");
        let root = create_root(temporary.path());
        create_historical_root(temporary.path(), mutation);
        let control =
            LivePrivilegedDisposablePolicyV2::create_for_test(&root).expect("reopen control");
        let error = match control.assess_read_only() {
            Ok(_) => panic!("{mutation:?} unexpectedly passed historical census"),
            Err(error) => error,
        };
        let text = error.to_string();
        assert!(
            text.contains("gap")
                || text.contains("exactly its own obligation")
                || text.contains("obligation nonce differs"),
            "unexpected {mutation:?} error: {text}"
        );
    }
}

fn closure_absence(operation_nonce: &str) -> FreshAbsenceObservationV2 {
    FreshAbsenceObservationV2 {
        artifact_evidence_sha256: digest('5'),
        baseline_inventory_sha256: digest('6'),
        backing_identity_sha256: digest('7'),
        boot_session_uuid: "87654321-4321-4321-4321-cba987654321".to_string(),
        collector_policy_sha256: digest('8'),
        collector_receipt_file: None,
        collector_receipt_sha256: digest('9'),
        current_expected_absence_inventory_sha256: None,
        iomedia_evidence_sha256: digest('a'),
        monotonic_after_nanoseconds: 12,
        monotonic_before_nanoseconds: 11,
        mount_evidence_sha256: digest('b'),
        mountpoint_underlying_sha256: digest('c'),
        no_matching_iomedia: true,
        no_nested_mounts: true,
        operation_artifacts_absent: true,
        operation_nonce: operation_nonce.to_string(),
        post_inventory_sha256: digest('6'),
        reconciliation_snapshot_sha256: None,
        restart_epoch_nonce: None,
    }
}

fn closure_expectation(
    root_identity: Identity,
    operation_nonce: &str,
    semantics_replayed: bool,
) -> LegacyClosureExpectation {
    LegacyClosureExpectation {
        attestation_name: format!(
            "{LEGACY_CLOSURE_PREFIX}{}-{operation_nonce}.json",
            digest('d')
        ),
        backing_identity_sha256: digest('7'),
        baseline_inventory_sha256: digest('6'),
        boot_session_uuid: "12345678-1234-1234-1234-123456789abc".to_string(),
        mountpoint_underlying_sha256: digest('c'),
        operation_nonce: operation_nonce.to_string(),
        root_identity,
        root_name: format!("{HISTORICAL_ROOT_PREFIX}{}", digest('d')),
        semantics_replayed,
        terminal_record_sha256: digest('e'),
    }
}

fn closure_attestation(
    expectation: &LegacyClosureExpectation,
    corrupt_digest: bool,
) -> LegacyClosureAttestationV2 {
    let fresh_absence = closure_absence(&expectation.operation_nonce);
    LegacyClosureAttestationV2 {
        authority: DisposableAuthorityV2::none(),
        fresh_absence_sha256: if corrupt_digest {
            digest('0')
        } else {
            fresh_absence_sha256(&fresh_absence).expect("fresh absence digest")
        },
        fresh_absence,
        historical_boot_session_uuid: expectation.boot_session_uuid.clone(),
        historical_operation_nonce: expectation.operation_nonce.clone(),
        historical_root_ctime_nanoseconds: expectation.root_identity.ctime_nsec,
        historical_root_ctime_seconds: expectation.root_identity.ctime_sec,
        historical_root_dev: expectation.root_identity.dev,
        historical_root_inode: expectation.root_identity.ino,
        historical_root_name: expectation.root_name.clone(),
        historical_terminal_record_sha256: expectation.terminal_record_sha256.clone(),
        schema: "hepta_mac_legacy_closure_attestation_v2".to_string(),
        schema_version: 2,
    }
}

#[test]
fn legacy_closure_never_closes_unproved_v1_semantics_and_rejects_digest_forgery() {
    for (semantics_replayed, corrupt_digest) in [(false, false), (true, true)] {
        let temporary = tempfile::tempdir().expect("temporary closure root");
        let root = temporary.path().join("control");
        let control =
            LivePrivilegedDisposablePolicyV2::create_for_test(&root).expect("open control");
        let expectation =
            closure_expectation(control.root_identity, &digest('a'), semantics_replayed);
        let attestation = closure_attestation(&expectation, corrupt_digest);
        write_canonical_file(
            &root
                .join(PUBLICATION_NAME)
                .join(&expectation.attestation_name),
            &attestation,
        );
        let mut total_bytes = 0;
        let mut retained =
            RetainedFdBudget::after_existing(MAX_RETAINED_FDS, 5).expect("closure budget");
        let result = control.verify_legacy_closures(
            std::slice::from_ref(&expectation),
            &mut total_bytes,
            &mut retained,
        );
        if corrupt_digest {
            assert!(result.is_err(), "forged closure digest was accepted");
        } else {
            let (closed, records, roster) = result.expect("valid storage-only closure record");
            assert!(
                closed.is_empty(),
                "unproved v1 semantics were marked closed"
            );
            assert_eq!(records.len(), 1);
            assert_eq!(roster, [expectation.attestation_name.clone()]);
        }
    }
}

#[test]
fn current_boot_released_barrier_cannot_close_same_nonce_prior_boot_obligation() {
    let nonce = digest('a');
    let terminal = digest('e');
    // A successful BarrierJournal replay already proves that the barrier is a
    // current-boot Released terminal.  That fact must not be allowed to lend
    // its boot freshness to an otherwise matching historical obligation and
    // fixture chain.
    let prior_boot = AttachmentObligationVerificationV1 {
        authority_granted: false,
        boot_session_uuid: "12345678-1234-1234-1234-123456789abc".to_string(),
        current_boot: false,
        disposition: ObligationDispositionV1::Reconciled,
        operation_nonce: nonce.clone(),
        records: 18,
        requires_privileged_reconciliation: true,
        schema: "hepta_mac_attachment_obligation_verification_v1".to_string(),
        terminal_record_sha256: terminal.clone(),
    };
    assert!(!historical_obligation_closure_eligible(&prior_boot));
    assert_eq!(
        historical_obligation_semantic_blocker("historical-root", &prior_boot),
        Some(format!(
            "historical-root/{nonce}:historical_v1_prior_boot_or_reconciliation_required"
        ))
    );

    let current_boot = AttachmentObligationVerificationV1 {
        current_boot: true,
        requires_privileged_reconciliation: false,
        ..prior_boot
    };
    assert_eq!(current_boot.operation_nonce, nonce);
    assert_eq!(current_boot.terminal_record_sha256, terminal);
    assert!(historical_obligation_closure_eligible(&current_boot));
    assert!(historical_obligation_semantic_blocker("historical-root", &current_boot).is_none());
}

#[test]
fn prior_boot_reconciled_obligation_is_in_final_execution_blockers() {
    let temporary = tempfile::tempdir().expect("temporary historical volume");
    let root = create_root(temporary.path());
    let nonce = create_historical_root(
        temporary.path(),
        HistoricalSemanticMutation::PriorBootReconciled,
    );
    let historical_root = format!("{HISTORICAL_ROOT_PREFIX}{nonce}");
    let expected_blocker =
        format!("{historical_root}/{nonce}:historical_v1_prior_boot_or_reconciliation_required");
    let control = LivePrivilegedDisposablePolicyV2::create_for_test(&root)
        .expect("reopen control with prior-boot reconciled obligation");
    let assessment = control
        .assess_read_only()
        .expect("prior-boot reconciliation remains an explicit receipt blocker");
    let receipt = assessment.receipt();

    assert!(!receipt.admission_authority);
    assert!(!receipt.authority.any());
    assert!(!receipt.new_operation_precondition_satisfied);
    assert_eq!(receipt.historical_roots_scanned, 1);
    assert!(receipt.historical_closure_bindings.is_empty());
    assert!(
        receipt
            .legacy_v1_verified_but_awaiting_v2_closure
            .is_empty(),
        "a prior-boot obligation must not produce a closure expectation"
    );
    assert_eq!(
        receipt
            .blocking_operation_nonces
            .iter()
            .filter(|blocker| *blocker == &expected_blocker)
            .count(),
        1,
        "final receipt omitted or duplicated the stable prior-boot blocker: {:?}",
        receipt.blocking_operation_nonces
    );
}
