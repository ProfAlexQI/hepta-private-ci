use super::*;
use crate::mac_disposable_effect_issue_store::ExactDisposableCommandV3;
use crate::mac_disposable_lifecycle::DisposableAuthorityV2;
use crate::mac_disposable_lifecycle::ReconciliationMatchV2;
use crate::mac_disposable_lifecycle::fresh_absence_sha256;
use crate::mac_disposable_lifecycle::reconciliation_snapshot_sha256;
use crate::mac_iomedia_identity::current_boot_session_uuid;
use crate::mac_privileged_disposable_control::LivePrivilegedDisposablePolicyV2;
use std::fs;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use tempfile::TempDir;

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

type FreshOperationStoreForCompileAssertions = CensusBoundDurableLifecycleStoreV3<'static>;
type RestartOperationStoreForCompileAssertions = ReconciliationOperationStoreV3<'static, 'static>;
type ActiveRestartOperationStoreForCompileAssertions =
    ReconciliationOperationStoreV3<'static, 'static, ActiveRestartEpochV3>;
type RetainedLifecycleAppendForCompileAssertions = RetainedLifecycleRecordAppendV3;
type RetainedLifecycleIssueSourceForCompileAssertions = RetainedLifecycleIssueSourceV3<'static>;
type RetainedEffectIssueSourceForCompileAssertions = RetainedEffectIssueSourceV3;
type PersistedRunnerGrantForCompileAssertions =
    PersistedIssuedRunnerGrantV3<'static, 'static, PersistedUnmountEffectV3>;
type PersistedIssueLeaseSealForCompileAssertions = PersistedIssueLeaseSealV3;
type RecoveredIssueVerifierSealForCompileAssertions = RecoveredIssueVerifierSealV3;
type IssuedTransitionBindingForCompileAssertions =
    PersistedIssuedTransitionBindingV3<PersistedUnmountEffectV3>;
type SealedCollectorPlanForCompileAssertions =
    SealedOwnedCollectorEffectPlanV3<PersistedUnmountEffectV3>;
type SuccessfulTransitionSealForCompileAssertions = SuccessfulIssuedEffectTransitionSealV3;
type IssuedEffectSessionForCompileAssertions =
    IssuedEffectSessionV3<'static, 'static, PersistedUnmountEffectV3>;
type IssuedEffectFailureForCompileAssertions =
    IssuedEffectDispatchFailureV3<'static, 'static, PersistedUnmountEffectV3>;
type IssuedEffectDeathProvedForCompileAssertions =
    SuccessfulIssuedEffectDeathProvedV3<'static, 'static, PersistedUnmountEffectV3>;
type IssuedEffectFailureDeathProvedForCompileAssertions =
    IssuedEffectDispatchFailureDeathProvedV3<'static, 'static, PersistedUnmountEffectV3>;
type PendingUnmountCallbackStoreForCompileAssertions =
    PendingUnmountReconciliationOperationStoreV3<'static, 'static, AwaitingUnmountCallbackV3>;
type PendingUnmountObservationStoreForCompileAssertions =
    PendingUnmountReconciliationOperationStoreV3<'static, 'static, AwaitingUnmountObservationV3>;
type SuccessfulUnmountCallbackForCompileAssertions =
    SuccessfulRunnerCallbackV3<PersistedUnmountEffectV3>;
type CompletedOperationStoreForCompileAssertions =
    CompletedReconciliationOperationStoreV3<'static, 'static>;
type ExistingWiringForCompileAssertions = ExistingCensusStoreWiringV3<'static, 'static>;
assert_not_impl!(FreshOperationStoreForCompileAssertions, Clone);
assert_not_impl!(FreshOperationStoreForCompileAssertions, Send);
assert_not_impl!(FreshOperationStoreForCompileAssertions, Sync);
assert_not_impl!(FreshOperationStoreForCompileAssertions, serde::Serialize);
assert_not_impl!(
    FreshOperationStoreForCompileAssertions,
    std::os::fd::AsRawFd
);
assert_not_impl!(FreshOperationStoreForCompileAssertions, From<std::fs::File>);
assert_not_impl!(RestartOperationStoreForCompileAssertions, Clone);
assert_not_impl!(RestartOperationStoreForCompileAssertions, Send);
assert_not_impl!(RestartOperationStoreForCompileAssertions, Sync);
assert_not_impl!(RestartOperationStoreForCompileAssertions, serde::Serialize);
assert_not_impl!(
    RestartOperationStoreForCompileAssertions,
    std::os::fd::AsRawFd
);
assert_not_impl!(
    RestartOperationStoreForCompileAssertions,
    From<std::fs::File>
);
assert_not_impl!(ActiveRestartOperationStoreForCompileAssertions, Clone);
assert_not_impl!(ActiveRestartOperationStoreForCompileAssertions, Send);
assert_not_impl!(ActiveRestartOperationStoreForCompileAssertions, Sync);
assert_not_impl!(
    ActiveRestartOperationStoreForCompileAssertions,
    serde::Serialize
);
assert_not_impl!(
    ActiveRestartOperationStoreForCompileAssertions,
    std::os::fd::AsRawFd
);
assert_not_impl!(
    ActiveRestartOperationStoreForCompileAssertions,
    From<std::fs::File>
);
assert_not_impl!(RetainedLifecycleAppendForCompileAssertions, Clone);
assert_not_impl!(RetainedLifecycleAppendForCompileAssertions, Send);
assert_not_impl!(RetainedLifecycleAppendForCompileAssertions, Sync);
assert_not_impl!(
    RetainedLifecycleAppendForCompileAssertions,
    serde::Serialize
);
assert_not_impl!(RetainedLifecycleIssueSourceForCompileAssertions, Clone);
assert_not_impl!(RetainedLifecycleIssueSourceForCompileAssertions, Send);
assert_not_impl!(RetainedLifecycleIssueSourceForCompileAssertions, Sync);
assert_not_impl!(
    RetainedLifecycleIssueSourceForCompileAssertions,
    serde::Serialize
);
assert_not_impl!(
    RetainedLifecycleIssueSourceForCompileAssertions,
    std::os::fd::AsRawFd
);
assert_not_impl!(
    RetainedLifecycleIssueSourceForCompileAssertions,
    From<std::fs::File>
);
assert_not_impl!(RetainedEffectIssueSourceForCompileAssertions, Clone);
assert_not_impl!(RetainedEffectIssueSourceForCompileAssertions, Send);
assert_not_impl!(RetainedEffectIssueSourceForCompileAssertions, Sync);
assert_not_impl!(
    RetainedEffectIssueSourceForCompileAssertions,
    serde::Serialize
);
assert_not_impl!(
    RetainedEffectIssueSourceForCompileAssertions,
    std::os::fd::AsRawFd
);
assert_not_impl!(
    RetainedEffectIssueSourceForCompileAssertions,
    From<std::fs::File>
);
assert_not_impl!(IssuedTransitionBindingForCompileAssertions, Clone);
assert_not_impl!(IssuedTransitionBindingForCompileAssertions, Send);
assert_not_impl!(IssuedTransitionBindingForCompileAssertions, Sync);
assert_not_impl!(
    IssuedTransitionBindingForCompileAssertions,
    serde::Serialize
);
assert_not_impl!(
    IssuedTransitionBindingForCompileAssertions,
    std::os::fd::AsRawFd
);
assert_not_impl!(
    IssuedTransitionBindingForCompileAssertions,
    From<std::fs::File>
);
assert_not_impl!(PersistedRunnerGrantForCompileAssertions, Clone);
assert_not_impl!(PersistedRunnerGrantForCompileAssertions, Send);
assert_not_impl!(PersistedRunnerGrantForCompileAssertions, Sync);
assert_not_impl!(PersistedRunnerGrantForCompileAssertions, serde::Serialize);
assert_not_impl!(
    PersistedRunnerGrantForCompileAssertions,
    std::os::fd::AsRawFd
);
assert_not_impl!(
    PersistedRunnerGrantForCompileAssertions,
    From<std::fs::File>
);
assert_not_impl!(PersistedIssueLeaseSealForCompileAssertions, Clone);
assert_not_impl!(PersistedIssueLeaseSealForCompileAssertions, Send);
assert_not_impl!(PersistedIssueLeaseSealForCompileAssertions, Sync);
assert_not_impl!(
    PersistedIssueLeaseSealForCompileAssertions,
    serde::Serialize
);
assert_not_impl!(
    PersistedIssueLeaseSealForCompileAssertions,
    std::os::fd::AsRawFd
);
assert_not_impl!(
    PersistedIssueLeaseSealForCompileAssertions,
    From<std::fs::File>
);
assert_not_impl!(RecoveredIssueVerifierSealForCompileAssertions, Clone);
assert_not_impl!(RecoveredIssueVerifierSealForCompileAssertions, Send);
assert_not_impl!(RecoveredIssueVerifierSealForCompileAssertions, Sync);
assert_not_impl!(
    RecoveredIssueVerifierSealForCompileAssertions,
    serde::Serialize
);
assert_not_impl!(
    RecoveredIssueVerifierSealForCompileAssertions,
    std::os::fd::AsRawFd
);
assert_not_impl!(
    RecoveredIssueVerifierSealForCompileAssertions,
    From<std::fs::File>
);
assert_not_impl!(SealedCollectorPlanForCompileAssertions, Clone);
assert_not_impl!(SealedCollectorPlanForCompileAssertions, Send);
assert_not_impl!(SealedCollectorPlanForCompileAssertions, Sync);
assert_not_impl!(SealedCollectorPlanForCompileAssertions, serde::Serialize);
assert_not_impl!(
    SealedCollectorPlanForCompileAssertions,
    std::os::fd::AsRawFd
);
assert_not_impl!(SealedCollectorPlanForCompileAssertions, From<std::fs::File>);
assert_not_impl!(SuccessfulTransitionSealForCompileAssertions, Clone);
assert_not_impl!(SuccessfulTransitionSealForCompileAssertions, Send);
assert_not_impl!(SuccessfulTransitionSealForCompileAssertions, Sync);
assert_not_impl!(
    SuccessfulTransitionSealForCompileAssertions,
    serde::Serialize
);
assert_not_impl!(
    SuccessfulTransitionSealForCompileAssertions,
    std::os::fd::AsRawFd
);
assert_not_impl!(
    SuccessfulTransitionSealForCompileAssertions,
    From<std::fs::File>
);
assert_not_impl!(IssuedEffectSessionForCompileAssertions, Clone);
assert_not_impl!(IssuedEffectSessionForCompileAssertions, Send);
assert_not_impl!(IssuedEffectSessionForCompileAssertions, Sync);
assert_not_impl!(IssuedEffectSessionForCompileAssertions, serde::Serialize);
assert_not_impl!(
    IssuedEffectSessionForCompileAssertions,
    std::os::fd::AsRawFd
);
assert_not_impl!(IssuedEffectSessionForCompileAssertions, From<std::fs::File>);
assert_not_impl!(IssuedEffectFailureForCompileAssertions, Clone);
assert_not_impl!(IssuedEffectFailureForCompileAssertions, Send);
assert_not_impl!(IssuedEffectFailureForCompileAssertions, Sync);
assert_not_impl!(IssuedEffectFailureForCompileAssertions, serde::Serialize);
assert_not_impl!(
    IssuedEffectFailureForCompileAssertions,
    std::os::fd::AsRawFd
);
assert_not_impl!(IssuedEffectFailureForCompileAssertions, From<std::fs::File>);
assert_not_impl!(IssuedEffectDeathProvedForCompileAssertions, Clone);
assert_not_impl!(IssuedEffectDeathProvedForCompileAssertions, Send);
assert_not_impl!(IssuedEffectDeathProvedForCompileAssertions, Sync);
assert_not_impl!(
    IssuedEffectDeathProvedForCompileAssertions,
    serde::Serialize
);
assert_not_impl!(
    IssuedEffectDeathProvedForCompileAssertions,
    std::os::fd::AsRawFd
);
assert_not_impl!(
    IssuedEffectDeathProvedForCompileAssertions,
    From<std::fs::File>
);
assert_not_impl!(IssuedEffectFailureDeathProvedForCompileAssertions, Clone);
assert_not_impl!(IssuedEffectFailureDeathProvedForCompileAssertions, Send);
assert_not_impl!(IssuedEffectFailureDeathProvedForCompileAssertions, Sync);
assert_not_impl!(
    IssuedEffectFailureDeathProvedForCompileAssertions,
    serde::Serialize
);
assert_not_impl!(
    IssuedEffectFailureDeathProvedForCompileAssertions,
    std::os::fd::AsRawFd
);
assert_not_impl!(
    IssuedEffectFailureDeathProvedForCompileAssertions,
    From<std::fs::File>
);
assert_not_impl!(PendingUnmountCallbackStoreForCompileAssertions, Clone);
assert_not_impl!(PendingUnmountCallbackStoreForCompileAssertions, Send);
assert_not_impl!(PendingUnmountCallbackStoreForCompileAssertions, Sync);
assert_not_impl!(
    PendingUnmountCallbackStoreForCompileAssertions,
    serde::Serialize
);
assert_not_impl!(
    PendingUnmountCallbackStoreForCompileAssertions,
    std::os::fd::AsRawFd
);
assert_not_impl!(
    PendingUnmountCallbackStoreForCompileAssertions,
    From<std::fs::File>
);
assert_not_impl!(
    PendingUnmountCallbackStoreForCompileAssertions,
    From<Vec<u8>>
);
assert_not_impl!(
    PendingUnmountCallbackStoreForCompileAssertions,
    From<String>
);
assert_not_impl!(PendingUnmountObservationStoreForCompileAssertions, Clone);
assert_not_impl!(PendingUnmountObservationStoreForCompileAssertions, Send);
assert_not_impl!(PendingUnmountObservationStoreForCompileAssertions, Sync);
assert_not_impl!(
    PendingUnmountObservationStoreForCompileAssertions,
    serde::Serialize
);
assert_not_impl!(
    PendingUnmountObservationStoreForCompileAssertions,
    std::os::fd::AsRawFd
);
assert_not_impl!(
    PendingUnmountObservationStoreForCompileAssertions,
    From<std::fs::File>
);
assert_not_impl!(
    PendingUnmountObservationStoreForCompileAssertions,
    From<Vec<u8>>
);
assert_not_impl!(
    PendingUnmountObservationStoreForCompileAssertions,
    From<String>
);
assert_not_impl!(SuccessfulUnmountCallbackForCompileAssertions, Clone);
assert_not_impl!(SuccessfulUnmountCallbackForCompileAssertions, Send);
assert_not_impl!(SuccessfulUnmountCallbackForCompileAssertions, Sync);
assert_not_impl!(
    SuccessfulUnmountCallbackForCompileAssertions,
    serde::Serialize
);
assert_not_impl!(
    SuccessfulUnmountCallbackForCompileAssertions,
    std::os::fd::AsRawFd
);
assert_not_impl!(
    SuccessfulUnmountCallbackForCompileAssertions,
    From<std::fs::File>
);
assert_not_impl!(SuccessfulUnmountCallbackForCompileAssertions, From<Vec<u8>>);
assert_not_impl!(SuccessfulUnmountCallbackForCompileAssertions, From<String>);
assert_not_impl!(
    RetainedLifecycleAppendForCompileAssertions,
    std::os::fd::AsRawFd
);
assert_not_impl!(
    RetainedLifecycleAppendForCompileAssertions,
    From<std::fs::File>
);
assert_not_impl!(CompletedOperationStoreForCompileAssertions, Clone);
assert_not_impl!(CompletedOperationStoreForCompileAssertions, Send);
assert_not_impl!(CompletedOperationStoreForCompileAssertions, Sync);
assert_not_impl!(
    CompletedOperationStoreForCompileAssertions,
    serde::Serialize
);
assert_not_impl!(
    CompletedOperationStoreForCompileAssertions,
    std::os::fd::AsRawFd
);
assert_not_impl!(
    CompletedOperationStoreForCompileAssertions,
    From<std::fs::File>
);
assert_not_impl!(FreshCensusStoreWiringV3, Clone);
assert_not_impl!(FreshCensusStoreWiringV3, Send);
assert_not_impl!(FreshCensusStoreWiringV3, Sync);
assert_not_impl!(FreshCensusStoreWiringV3, serde::Serialize);
assert_not_impl!(FreshCensusStoreWiringV3, std::os::fd::AsRawFd);
assert_not_impl!(FreshCensusStoreWiringV3, From<std::fs::File>);
assert_not_impl!(PreparedFreshCensusStoreV3, Clone);
assert_not_impl!(PreparedFreshCensusStoreV3, Send);
assert_not_impl!(PreparedFreshCensusStoreV3, Sync);
assert_not_impl!(PreparedFreshCensusStoreV3, serde::Serialize);
assert_not_impl!(PreparedFreshCensusStoreV3, std::os::fd::AsRawFd);
assert_not_impl!(PreparedFreshCensusStoreV3, From<std::fs::File>);
assert_not_impl!(ExistingWiringForCompileAssertions, Clone);
assert_not_impl!(ExistingWiringForCompileAssertions, Send);
assert_not_impl!(ExistingWiringForCompileAssertions, Sync);
assert_not_impl!(ExistingWiringForCompileAssertions, serde::Serialize);
assert_not_impl!(ExistingWiringForCompileAssertions, std::os::fd::AsRawFd);
assert_not_impl!(ExistingWiringForCompileAssertions, From<std::fs::File>);
assert_not_impl!(
    FreshPreparedLifecycleEventV3,
    From<DisposableLifecycleEventV2>
);
assert_not_impl!(
    ReconciliationLifecycleEventV3,
    From<DisposableLifecycleEventV2>
);
assert_not_impl!(
    ReconciliationTerminalEventV3,
    From<DisposableLifecycleEventV2>
);

const NONCE: &str = "1111111111111111111111111111111111111111111111111111111111111111";

struct Fixture {
    _root: TempDir,
    operations: File,
    operations_path: std::path::PathBuf,
}

impl Fixture {
    fn new() -> Self {
        let root = tempfile::tempdir().expect("temporary root");
        let operations_path = root.path().join("operations");
        fs::create_dir(&operations_path).expect("operations directory");
        fs::set_permissions(&operations_path, fs::Permissions::from_mode(0o700))
            .expect("operations permissions");
        let operations = File::open(&operations_path).expect("open operations");
        Self {
            _root: root,
            operations,
            operations_path,
        }
    }

    fn uid(&self) -> u32 {
        unsafe { libc::geteuid() }
    }

    fn gid(&self) -> u32 {
        unsafe { libc::getegid() }
    }

    fn create(&self, nonce: &str) -> DurableLifecycleStoreV3 {
        DurableLifecycleStoreV3::create(&self.operations, nonce, self.uid(), self.gid())
            .expect("create durable store")
    }

    fn operation_path(&self, nonce: &str) -> std::path::PathBuf {
        self.operations_path.join(format!("operation-{nonce}"))
    }
}

fn digest(byte: char) -> String {
    byte.to_string().repeat(64)
}

fn prepared() -> DisposableLifecycleEventV2 {
    DisposableLifecycleEventV2::OperationPrepared {
        baseline_inventory_sha256: digest('a'),
        backing_identity_sha256: digest('b'),
        boot_session_uuid: "12345678-1234-4abc-8abc-123456789abc".to_string(),
        collector_policy_sha256: digest('c'),
        mountpoint_underlying_sha256: digest('d'),
    }
}

fn sealed_prepared() -> FreshPreparedLifecycleEventV3 {
    FreshPreparedLifecycleEventV3::new(
        digest('a'),
        digest('b'),
        "12345678-1234-4abc-8abc-123456789abc".to_string(),
        digest('c'),
        digest('d'),
    )
}

fn sidecar_bound_prepared(store: &DurableLifecycleStoreV3) -> DisposableLifecycleEventV2 {
    let manifest = store
        .prepared_manifest
        .as_ref()
        .expect("prepared-manifest format retains its sidecar");
    FreshPreparedLifecycleEventV3::new_bound(
        digest('a'),
        digest('b'),
        "12345678-1234-4abc-8abc-123456789abc".to_string(),
        digest('c'),
        digest('d'),
        PreparedCollectorManifestBindingV3 {
            birthtime_nanoseconds: manifest.binding.birthtime_nsec,
            birthtime_seconds: manifest.binding.birthtime_sec,
            dev: manifest.binding.dev,
            generation: manifest.binding.generation,
            inode: manifest.binding.ino,
            receipt_root_initial: None,
            sha256: manifest.digest.clone(),
        },
    )
    .0
}

fn create_recoverable_issue(
    root: &std::path::Path,
    epochs: EffectEpochEvidenceV3,
) -> std::path::PathBuf {
    {
        let control =
            LivePrivilegedDisposablePolicyV2::create_for_test(root).expect("create S1 control");
        let census = control
            .assess_read_only()
            .expect("fresh assessment")
            .into_fresh_control_census()
            .expect("fresh census");
        let mut store =
            CensusBoundDurableLifecycleStoreV3::create(census, NONCE).expect("fresh operation");
        store.append(prepared()).expect("prepared record");
    }

    {
        let control =
            LivePrivilegedDisposablePolicyV2::create_for_test(root).expect("old supervisor S1");
        let census = control
            .assess_read_only()
            .expect("blocking assessment")
            .into_blocking_control_census(NONCE)
            .expect("blocking census");
        let epoch = FreshProcessEpochV3::establish().expect("old supervisor epoch");
        let mut store = ReconciliationOperationStoreV3::open_existing(census, &epoch)
            .expect("old supervisor operation")
            .activate_without_admission_for_test()
            .expect("test active restart typestate");
        let boot = current_boot_session_uuid().expect("current boot");
        store
            .append_reconciliation(ReconciliationLifecycleEventV3::restart_started(
                boot.clone(),
                digest('c'),
                100,
                digest('e'),
            ))
            .expect("restart record");
        store
            .append_reconciliation(ReconciliationLifecycleEventV3::snapshot_observed(
                ReconciliationSnapshotV2 {
                    backing_identity_sha256: digest('b'),
                    boot_session_uuid: boot,
                    collector_policy_sha256: digest('c'),
                    collector_receipt_file: None,
                    collector_receipt_sha256: digest('f'),
                    current_expected_absence_inventory_sha256: Some(digest('9')),
                    iomedia_evidence_sha256: digest('1'),
                    match_result: ReconciliationMatchV2::Unique { mounted: true },
                    monotonic_after_nanoseconds: 102,
                    monotonic_before_nanoseconds: 101,
                    mount_evidence_sha256: digest('2'),
                    mountpoint_underlying_sha256: digest('d'),
                    operation_nonce: NONCE.to_string(),
                    restart_epoch_nonce: digest('e'),
                },
            ))
            .expect("collector observation");
        store
            .append_reconciliation_inner(ReconciliationLifecycleEventV3::unmount_issued(1), true)
            .expect("durable V2 issue");
        let lifecycle = VerifiedLifecycleIssueRosterV3::capture_from_s2(store.store.issue_source())
            .expect("exact V2 issue roster");
        let mut retained = store
            .issues
            .persist(
                &lifecycle,
                ExactDisposableCommandV3::UnmountVolume {
                    mounted_binding_sha256: digest('7'),
                },
                epochs,
                Some(digest('8')),
            )
            .expect("durable V3 issue");
        let sink = store
            .census
            .selected_effect_issue_sink()
            .expect("exact S1 issue sink");
        retained.adopt_into_s1(sink).expect("S1 adopts exact issue");
        retained
            .require_s1_adopted()
            .expect("issue remains S1-adopted");
    }

    root.join("operations")
        .join(format!("operation-{NONCE}"))
        .join("effect-issues-v3")
}

fn incoming_record_path(fixture: &Fixture, nonce: &str) -> std::path::PathBuf {
    fixture
        .operation_path(nonce)
        .join(".incoming-00000001.json")
}

fn final_record_path(fixture: &Fixture, nonce: &str) -> std::path::PathBuf {
    fixture.operation_path(nonce).join("00000001.json")
}

#[test]
fn publishes_exact_no_authority_record_and_reopens_for_reconciliation_only() {
    let fixture = Fixture::new();
    let mut store = fixture.create(NONCE);
    let mut journal = DisposableLifecycleJournalV2::new(NONCE).expect("fresh journal");

    let digest = store.append(&mut journal, prepared()).expect("append");
    assert_eq!(digest.len(), 64);
    assert!(!store.poisoned());
    assert!(!journal.persistence_uncertain());
    assert!(!incoming_record_path(&fixture, NONCE).exists());

    let final_path = final_record_path(&fixture, NONCE);
    let metadata = fs::symlink_metadata(&final_path).expect("final metadata");
    assert_eq!(metadata.mode() & 0o7777, 0o400);
    assert_eq!(metadata.nlink(), 1);

    let reopened = DurableLifecycleStoreV3::open_existing(
        &fixture.operations,
        NONCE,
        fixture.uid(),
        fixture.gid(),
    )
    .expect("reopen");
    assert_eq!(reopened.operation_nonce(), NONCE);
    let resumed = reopened
        .resume_for_reconciliation()
        .expect("reconciliation journal");
    assert_eq!(resumed.last_effect_id(), 0);
    assert!(!resumed.persistence_uncertain());
    let inspection = inspect_lifecycle_v2(
        &reopened
            .records
            .iter()
            .map(|record| record.bytes.clone())
            .collect::<Vec<_>>(),
    )
    .expect("inspection");
    assert_eq!(inspection.authority, DisposableAuthorityV2::none());
    assert!(inspection.blocks_new_operations);
    assert!(!inspection.restart_forward_flow_authority);
}

#[test]
fn operation_directory_creation_is_no_replace() {
    let fixture = Fixture::new();
    let store = fixture.create(NONCE);
    let original = binding(&store.directory).expect("original binding");

    let error = match DurableLifecycleStoreV3::create(
        &fixture.operations,
        NONCE,
        fixture.uid(),
        fixture.gid(),
    ) {
        Ok(_) => panic!("same operation must not be replaced"),
        Err(error) => error,
    };
    assert!(error.to_string().contains("already occupied"));
    assert_eq!(
        binding(&store.directory).expect("retained binding"),
        original
    );
    store.revalidate().expect("original survives");
}

#[test]
fn operation_creation_crash_cutpoints_never_create_an_implicit_authority_path() {
    for cutpoint in [
        CreateCutpointV3::TemporaryCreated,
        CreateCutpointV3::TemporaryOpened,
        CreateCutpointV3::TemporarySynced,
    ] {
        let fixture = Fixture::new();
        let result = DurableLifecycleStoreV3::create_with_hook(
            &fixture.operations,
            NONCE,
            fixture.uid(),
            fixture.gid(),
            |observed| {
                if observed == cutpoint {
                    Err(io::Error::other("injected pre-rename create crash"))
                } else {
                    Ok(())
                }
            },
        );
        assert!(result.is_err(), "{cutpoint:?}");
        assert!(
            fixture
                .operations_path
                .join(format!(".incoming-operation-{NONCE}"))
                .is_dir(),
            "{cutpoint:?}"
        );
        assert!(!fixture.operation_path(NONCE).exists(), "{cutpoint:?}");
        assert!(
            DurableLifecycleStoreV3::open_existing(
                &fixture.operations,
                NONCE,
                fixture.uid(),
                fixture.gid(),
            )
            .is_err(),
            "{cutpoint:?}"
        );
    }

    for cutpoint in [
        CreateCutpointV3::Renamed,
        CreateCutpointV3::ParentSynced,
        CreateCutpointV3::FinalReopened,
        CreateCutpointV3::FinalRevalidated,
    ] {
        let fixture = Fixture::new();
        let result = DurableLifecycleStoreV3::create_with_hook(
            &fixture.operations,
            NONCE,
            fixture.uid(),
            fixture.gid(),
            |observed| {
                if observed == cutpoint {
                    Err(io::Error::other("injected post-rename create crash"))
                } else {
                    Ok(())
                }
            },
        );
        assert!(result.is_err(), "{cutpoint:?}");
        assert!(fixture.operation_path(NONCE).is_dir(), "{cutpoint:?}");
        assert!(
            !fixture
                .operations_path
                .join(format!(".incoming-operation-{NONCE}"))
                .exists(),
            "{cutpoint:?}"
        );
        assert!(
            DurableLifecycleStoreV3::open_existing(
                &fixture.operations,
                NONCE,
                fixture.uid(),
                fixture.gid(),
            )
            .is_err(),
            "empty final directory must not replay: {cutpoint:?}"
        );
        assert!(
            DurableLifecycleStoreV3::create(
                &fixture.operations,
                NONCE,
                fixture.uid(),
                fixture.gid(),
            )
            .is_err(),
            "final name must remain no-replace: {cutpoint:?}"
        );
    }
}

#[test]
fn prepared_manifest_publish_cutpoints_never_publish_the_operation() {
    let manifest = br#"{"schema":"modeled-prepared-collector-manifest-v3"}"#;
    for cutpoint in [
        CreateCutpointV3::PreparedManifestTemporaryCreated,
        CreateCutpointV3::PreparedManifestBytesWritten,
        CreateCutpointV3::PreparedManifestFileSynced,
        CreateCutpointV3::PreparedManifestRenamed,
        CreateCutpointV3::PreparedManifestDirectorySynced,
        CreateCutpointV3::PreparedManifestFinalReopened,
        CreateCutpointV3::PreparedManifestFinalRevalidated,
    ] {
        let fixture = Fixture::new();
        let result = DurableLifecycleStoreV3::create_new_format_with_hook(
            &fixture.operations,
            NONCE,
            fixture.uid(),
            fixture.gid(),
            Some(manifest),
            |observed| {
                if observed == cutpoint {
                    Err(io::Error::other("injected prepared-manifest crash"))
                } else {
                    Ok(())
                }
            },
        );
        assert!(result.is_err(), "{cutpoint:?}");
        assert!(!fixture.operation_path(NONCE).exists(), "{cutpoint:?}");
        assert!(
            fixture
                .operations_path
                .join(format!(".incoming-operation-{NONCE}"))
                .is_dir(),
            "{cutpoint:?}"
        );
    }
}

#[test]
fn prepared_manifest_is_fixed_replayed_and_bound_by_the_first_record() {
    let fixture = Fixture::new();
    let manifest = br#"{"schema":"modeled-prepared-collector-manifest-v3"}"#;
    let (mut store, _issues) = DurableLifecycleStoreV3::create_new_format_with_hook(
        &fixture.operations,
        NONCE,
        fixture.uid(),
        fixture.gid(),
        Some(manifest),
        |_| Ok(()),
    )
    .expect("publish prepared-manifest operation");
    let sidecar = fixture
        .operation_path(NONCE)
        .join(PREPARED_MANIFEST_NAME_V3);
    let metadata = fs::symlink_metadata(&sidecar).expect("prepared sidecar metadata");
    assert_eq!(metadata.mode() & 0o7777, 0o400);
    assert_eq!(metadata.nlink(), 1);
    assert_eq!(
        fs::read(&sidecar).expect("prepared sidecar bytes"),
        manifest
    );

    let event = sidecar_bound_prepared(&store);
    let mut journal = DisposableLifecycleJournalV2::new(NONCE).expect("journal");
    store
        .append(&mut journal, event)
        .expect("bound first record");
    store.revalidate().expect("exact prepared replay");

    let reopened = DurableLifecycleStoreV3::open_existing(
        &fixture.operations,
        NONCE,
        fixture.uid(),
        fixture.gid(),
    )
    .expect("reopen exact bound prepared sidecar");
    assert_eq!(
        reopened
            .prepared_manifest
            .as_ref()
            .expect("retained sidecar")
            .digest,
        sha256(manifest)
    );
}

#[test]
fn s1_replays_legacy_manifest_without_receipt_root_generation_as_a_blocker() {
    let temporary = tempfile::tempdir().expect("control root parent");
    let control_root = temporary.path().join("control");
    let initial_control =
        LivePrivilegedDisposablePolicyV2::create_for_test(&control_root).expect("control root");
    drop(initial_control);
    let operations = File::open(control_root.join("operations")).expect("operations descriptor");
    let manifest = br#"{"schema":"modeled-prepared-collector-manifest-v3"}"#;
    let (mut store, issues) = DurableLifecycleStoreV3::create_new_format_with_hook(
        &operations,
        NONCE,
        unsafe { libc::geteuid() },
        unsafe { libc::getegid() },
        Some(manifest),
        |_| Ok(()),
    )
    .expect("publish legacy manifest-bound operation");
    let event = sidecar_bound_prepared(&store);
    let mut journal = DisposableLifecycleJournalV2::new(NONCE).expect("journal");
    store
        .append(&mut journal, event)
        .expect("append legacy manifest-bound first record");
    drop(issues);
    drop(store);
    drop(operations);

    let control =
        LivePrivilegedDisposablePolicyV2::create_for_test(&control_root).expect("reopen control");
    let assessment = control
        .assess_read_only()
        .expect("legacy receipt-root None remains readable and fail closed");
    let receipt = assessment.receipt();
    assert!(!receipt.admission_authority);
    assert!(!receipt.authority.any());
    assert!(!receipt.new_operation_precondition_satisfied);
    assert_eq!(receipt.blocking_operation_nonces, [NONCE.to_string()]);
    let census = assessment
        .into_blocking_control_census(NONCE)
        .expect("legacy manifest remains an exact blocking S1 capsule");
    let epoch = FreshProcessEpochV3::establish().expect("fresh process epoch");
    assert!(
        ReconciliationOperationStoreV3::open_existing(census, &epoch).is_err(),
        "legacy receipt-root None must not mint a production restart store"
    );
}

#[test]
fn restart_admission_rejects_same_byte_prepared_manifest_inode_replacement() {
    let root = tempfile::tempdir().expect("control root parent");
    let control_root = root.path().join("control");
    let initial_control =
        LivePrivilegedDisposablePolicyV2::create_for_test(&control_root).expect("control root");
    drop(initial_control);
    let operations_path = control_root.join("operations");
    let operations = File::open(&operations_path).expect("operations descriptor");
    let uid = unsafe { libc::geteuid() };
    let gid = unsafe { libc::getegid() };
    let manifest = br#"{"schema":"modeled-prepared-collector-manifest-v3"}"#;
    let (mut store, issues) = DurableLifecycleStoreV3::create_new_format_with_hook(
        &operations,
        NONCE,
        uid,
        gid,
        Some(manifest),
        |_| Ok(()),
    )
    .expect("publish prepared operation");
    let event = sidecar_bound_prepared(&store);
    let mut journal = DisposableLifecycleJournalV2::new(NONCE).expect("journal");
    store
        .append(&mut journal, event)
        .expect("bound first record");
    drop(issues);
    drop(store);

    let sidecar = operations_path
        .join(format!("operation-{NONCE}"))
        .join(PREPARED_MANIFEST_NAME_V3);
    let displaced = root.path().join("displaced-prepared-manifest");
    fs::rename(&sidecar, &displaced).expect("displace exact manifest inode");
    fs::write(&sidecar, manifest).expect("same-byte replacement");
    fs::set_permissions(&sidecar, fs::Permissions::from_mode(0o400))
        .expect("replacement permissions");

    let control =
        LivePrivilegedDisposablePolicyV2::create_for_test(&control_root).expect("restart control");
    let error = match control.assess_read_only() {
        Ok(_) => panic!("S1 accepted same-byte manifest inode replacement"),
        Err(error) => error,
    };
    assert!(
        error.to_string().contains("prepared-manifest binding"),
        "unexpected rejection: {error}"
    );
}

#[test]
fn prepared_manifest_aliases_and_missing_issue_root_are_blocking() {
    for name in [
        PREPARED_MANIFEST_TEMPORARY_NAME_V3,
        "prepared-collector-manifest-v3.json.alias",
    ] {
        let fixture = Fixture::new();
        let operation = fixture.operation_path(NONCE);
        fs::create_dir(&operation).expect("operation directory");
        fs::set_permissions(&operation, fs::Permissions::from_mode(0o700))
            .expect("operation permissions");
        fs::write(operation.join(name), b"manifest").expect("modeled alias");
        fs::set_permissions(operation.join(name), fs::Permissions::from_mode(0o400))
            .expect("alias permissions");
        assert!(
            DurableLifecycleStoreV3::open_existing(
                &fixture.operations,
                NONCE,
                fixture.uid(),
                fixture.gid(),
            )
            .is_err(),
            "{name}"
        );
    }
}

#[test]
fn operation_creation_never_overwrites_a_racing_destination() {
    let fixture = Fixture::new();
    let destination = fixture.operation_path(NONCE);
    let sentinel = destination.join("sentinel");
    let result = DurableLifecycleStoreV3::create_with_hook(
        &fixture.operations,
        NONCE,
        fixture.uid(),
        fixture.gid(),
        |cutpoint| {
            if cutpoint == CreateCutpointV3::TemporarySynced {
                fs::create_dir(&destination)?;
                fs::write(&sentinel, b"must survive")?;
            }
            Ok(())
        },
    );
    assert!(result.is_err());
    assert_eq!(fs::read(&sentinel).expect("sentinel"), b"must survive");
    assert!(
        fixture
            .operations_path
            .join(format!(".incoming-operation-{NONCE}"))
            .is_dir()
    );
}

#[test]
fn operation_creation_rejects_a_late_same_nonce_crash_temporary() {
    for injected_at in [
        CreateCutpointV3::Renamed,
        CreateCutpointV3::FinalRevalidated,
    ] {
        let fixture = Fixture::new();
        let incoming = fixture
            .operations_path
            .join(format!(".incoming-operation-{NONCE}"));
        let result = DurableLifecycleStoreV3::create_with_hook(
            &fixture.operations,
            NONCE,
            fixture.uid(),
            fixture.gid(),
            |cutpoint| {
                if cutpoint == injected_at {
                    fs::create_dir(&incoming)?;
                }
                Ok(())
            },
        );
        assert!(result.is_err(), "{injected_at:?}");
        assert!(fixture.operation_path(NONCE).is_dir(), "{injected_at:?}");
        assert!(incoming.is_dir(), "{injected_at:?}");
    }
}

#[test]
fn pre_rename_crash_cutpoints_leave_only_a_reconciliation_blocking_temporary() {
    for cutpoint in [
        PublishCutpointV3::TemporaryCreated,
        PublishCutpointV3::BytesWritten,
        PublishCutpointV3::FileSynced,
    ] {
        let fixture = Fixture::new();
        let mut store = fixture.create(NONCE);
        let mut journal = DisposableLifecycleJournalV2::new(NONCE).expect("journal");
        let result = store.append_mode_with_hook(&mut journal, prepared(), |observed| {
            if observed == cutpoint {
                Err(io::Error::other("injected pre-rename crash"))
            } else {
                Ok(())
            }
        });

        assert!(result.is_err(), "{cutpoint:?}");
        assert!(store.poisoned(), "{cutpoint:?}");
        assert!(journal.persistence_uncertain(), "{cutpoint:?}");
        assert!(
            incoming_record_path(&fixture, NONCE).exists(),
            "{cutpoint:?}"
        );
        assert!(!final_record_path(&fixture, NONCE).exists(), "{cutpoint:?}");
        assert!(
            DurableLifecycleStoreV3::open_existing(
                &fixture.operations,
                NONCE,
                fixture.uid(),
                fixture.gid(),
            )
            .is_err(),
            "{cutpoint:?}"
        );
    }
}

#[test]
fn post_rename_crash_cutpoints_replay_exactly_but_never_resume_forward_flow() {
    for cutpoint in [
        PublishCutpointV3::Renamed,
        PublishCutpointV3::ParentSynced,
        PublishCutpointV3::FinalReopened,
        PublishCutpointV3::FinalRevalidated,
        PublishCutpointV3::CapsuleRetained,
    ] {
        let fixture = Fixture::new();
        let mut store = fixture.create(NONCE);
        let mut journal = DisposableLifecycleJournalV2::new(NONCE).expect("journal");
        let result = store.append_mode_with_hook(&mut journal, prepared(), |observed| {
            if observed == cutpoint {
                Err(io::Error::other("injected post-rename crash"))
            } else {
                Ok(())
            }
        });

        assert!(result.is_err(), "{cutpoint:?}");
        assert!(store.poisoned(), "{cutpoint:?}");
        assert!(journal.persistence_uncertain(), "{cutpoint:?}");
        assert!(
            !incoming_record_path(&fixture, NONCE).exists(),
            "{cutpoint:?}"
        );
        assert!(final_record_path(&fixture, NONCE).exists(), "{cutpoint:?}");

        let reopened = DurableLifecycleStoreV3::open_existing(
            &fixture.operations,
            NONCE,
            fixture.uid(),
            fixture.gid(),
        )
        .expect("exact post-rename replay");
        let resumed = reopened
            .resume_for_reconciliation()
            .expect("reconciliation-only journal");
        assert_eq!(resumed.last_effect_id(), 0);
        assert!(!resumed.persistence_uncertain());
    }
}

#[test]
fn record_publication_never_overwrites_a_racing_destination() {
    let fixture = Fixture::new();
    let mut store = fixture.create(NONCE);
    let mut journal = DisposableLifecycleJournalV2::new(NONCE).expect("journal");
    let destination = final_record_path(&fixture, NONCE);
    let sentinel = b"racing destination must survive";

    let result = store.append_mode_with_hook(&mut journal, prepared(), |cutpoint| {
        if cutpoint == PublishCutpointV3::FileSynced {
            fs::write(&destination, sentinel)?;
        }
        Ok(())
    });

    assert!(result.is_err());
    assert!(store.poisoned());
    assert!(journal.persistence_uncertain());
    assert_eq!(fs::read(&destination).expect("sentinel"), sentinel);
    assert!(incoming_record_path(&fixture, NONCE).exists());
}

#[test]
fn append_requires_the_exact_in_memory_journal_nonce_count_and_predecessor() {
    let fixture = Fixture::new();
    let mut store = fixture.create(NONCE);
    let other_nonce = digest('2');
    let mut wrong_nonce =
        DisposableLifecycleJournalV2::new(&other_nonce).expect("wrong-nonce journal");
    assert!(store.append(&mut wrong_nonce, prepared()).is_err());
    assert!(!store.poisoned());
    assert!(!wrong_nonce.persistence_uncertain());
    assert!(
        read_directory_names(store.directory.as_raw_fd(), MAX_RECORDS)
            .expect("empty roster")
            .is_empty()
    );

    let mut exact = DisposableLifecycleJournalV2::new(NONCE).expect("exact journal");
    store.append(&mut exact, prepared()).expect("exact append");

    let mut divergent = DisposableLifecycleJournalV2::new(NONCE).expect("divergent journal");
    let mut divergent_prepared = prepared();
    if let DisposableLifecycleEventV2::OperationPrepared {
        baseline_inventory_sha256,
        ..
    } = &mut divergent_prepared
    {
        *baseline_inventory_sha256 = digest('f');
    }
    divergent
        .append_with(divergent_prepared, |_, _| Ok(()))
        .expect("build divergent predecessor");
    assert_eq!(divergent.record_count(), exact.record_count());
    assert_ne!(
        divergent.terminal_record_sha256(),
        exact.terminal_record_sha256()
    );
    assert!(
        store
            .append(
                &mut divergent,
                DisposableLifecycleEventV2::CreateIssuedOrUncertain { effect_id: 1 },
            )
            .is_err()
    );
    assert!(!store.poisoned());
    assert!(!divergent.persistence_uncertain());

    store
        .append(
            &mut exact,
            DisposableLifecycleEventV2::CreateIssuedOrUncertain { effect_id: 1 },
        )
        .expect("exact chain remains usable");
}

#[test]
fn reopened_typestate_rejects_a_rebuilt_matching_fresh_journal() {
    let fixture = Fixture::new();
    let mut fresh_store = fixture.create(NONCE);
    let mut original = DisposableLifecycleJournalV2::new(NONCE).expect("original journal");
    fresh_store
        .append(&mut original, prepared())
        .expect("durable prepared");
    let durable_digest = original.terminal_record_sha256().map(str::to_string);
    drop(original);
    drop(fresh_store);

    let mut reopened = DurableLifecycleStoreV3::open_existing(
        &fixture.operations,
        NONCE,
        fixture.uid(),
        fixture.gid(),
    )
    .expect("reopened reconciliation store");
    let mut forged_fresh = DisposableLifecycleJournalV2::new(NONCE).expect("fresh forgery");
    forged_fresh
        .append_with(prepared(), |_, _| Ok(()))
        .expect("rebuild matching public V2 chain");
    assert_eq!(
        forged_fresh.terminal_record_sha256(),
        durable_digest.as_deref()
    );
    assert_eq!(
        forged_fresh.process_mode(),
        LifecycleProcessModeV2::FreshProcess
    );
    assert!(
        reopened
            .append_mode_with_hook(
                &mut forged_fresh,
                DisposableLifecycleEventV2::CreateIssuedOrUncertain { effect_id: 1 },
                |_| Ok(()),
            )
            .is_err()
    );
    assert!(!forged_fresh.persistence_uncertain());
    let roster_before =
        read_directory_names(reopened.directory.as_raw_fd(), MAX_RECORDS).expect("reopened roster");
    assert!(
        reopened
            .append_reconciliation(
                &mut forged_fresh,
                ReconciliationLifecycleEventV3::restart_started(
                    "aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa".to_string(),
                    digest('c'),
                    100,
                    digest('e'),
                ),
            )
            .is_err()
    );
    assert_eq!(
        read_directory_names(reopened.directory.as_raw_fd(), MAX_RECORDS)
            .expect("unchanged reopened roster"),
        roster_before
    );

    let mut reconciliation = reopened
        .resume_for_reconciliation()
        .expect("genuine reconciliation journal");
    assert_eq!(
        reconciliation.process_mode(),
        LifecycleProcessModeV2::RestartReconcileOnly
    );
    reopened
        .append_reconciliation(
            &mut reconciliation,
            ReconciliationLifecycleEventV3::restart_started(
                "aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa".to_string(),
                digest('c'),
                100,
                digest('e'),
            ),
        )
        .expect("typed reconciliation event");
}

#[test]
fn observed_descriptor_drift_poisons_the_live_store_before_any_append() {
    let fixture = Fixture::new();
    let mut store = fixture.create(NONCE);
    let mut journal = DisposableLifecycleJournalV2::new(NONCE).expect("journal");
    fs::write(fixture.operation_path(NONCE).join("late-entry"), b"drift")
        .expect("inject roster drift");

    assert!(store.append(&mut journal, prepared()).is_err());
    assert!(store.poisoned());
    assert!(!journal.persistence_uncertain());
    fs::remove_file(fixture.operation_path(NONCE).join("late-entry")).expect("remove drift");
    assert!(store.append(&mut journal, prepared()).is_err());
}

#[test]
fn replay_rejects_roster_mutation_restored_inside_the_validation_window() {
    let fixture = Fixture::new();
    let store = fixture.create(NONCE);
    let racing_entry = fixture.operation_path(NONCE).join("transient-entry");

    let result = store.revalidate_with_hook(|| {
        fs::write(&racing_entry, b"transient")?;
        fs::remove_file(&racing_entry)?;
        Ok(())
    });

    assert!(result.is_err());
    assert!(!racing_entry.exists());
}

#[test]
fn replay_rejects_record_mutation_after_its_first_validation_pass() {
    let fixture = Fixture::new();
    let mut store = fixture.create(NONCE);
    let mut journal = DisposableLifecycleJournalV2::new(NONCE).expect("journal");
    store.append(&mut journal, prepared()).expect("append");
    let record_path = final_record_path(&fixture, NONCE);

    let result = store.revalidate_with_hook(|| {
        fs::set_permissions(&record_path, fs::Permissions::from_mode(0o600))?;
        fs::set_permissions(&record_path, fs::Permissions::from_mode(0o400))?;
        Ok(())
    });

    assert!(result.is_err());
}

#[test]
fn retained_replay_rejects_hardlinks_xattrs_and_content_mutation() {
    {
        let fixture = Fixture::new();
        let mut store = fixture.create(NONCE);
        let mut journal = DisposableLifecycleJournalV2::new(NONCE).expect("journal");
        store.append(&mut journal, prepared()).expect("append");
        fs::hard_link(
            final_record_path(&fixture, NONCE),
            fixture.operation_path(NONCE).join("alias"),
        )
        .expect("create hardlink");
        assert!(store.revalidate().is_err());
    }

    {
        let fixture = Fixture::new();
        let mut store = fixture.create(NONCE);
        let mut journal = DisposableLifecycleJournalV2::new(NONCE).expect("journal");
        store.append(&mut journal, prepared()).expect("append");
        let record_path = final_record_path(&fixture, NONCE);
        fs::set_permissions(&record_path, fs::Permissions::from_mode(0o600))
            .expect("make record metadata writable");
        let path = CString::new(record_path.as_os_str().as_bytes()).expect("record path");
        let attribute = c"com.hepta.test";
        let value = b"1";
        let rc = unsafe {
            libc::setxattr(
                path.as_ptr(),
                attribute.as_ptr(),
                value.as_ptr().cast(),
                value.len(),
                0,
                0,
            )
        };
        assert_eq!(rc, 0, "set xattr: {}", io::Error::last_os_error());
        fs::set_permissions(&record_path, fs::Permissions::from_mode(0o400))
            .expect("restore exact mode");
        // Normalize the stat snapshot so this reaches the explicit xattr
        // closed-world check rather than rejecting on the changed ctime.
        store.records[0].binding = binding(&store.records[0].file).expect("new binding");
        assert!(store.revalidate().is_err());
    }

    {
        let fixture = Fixture::new();
        let mut store = fixture.create(NONCE);
        let mut journal = DisposableLifecycleJournalV2::new(NONCE).expect("journal");
        store.append(&mut journal, prepared()).expect("append");
        let record_path = final_record_path(&fixture, NONCE);
        fs::set_permissions(&record_path, fs::Permissions::from_mode(0o600))
            .expect("make record writable");
        fs::write(&record_path, b"corrupted").expect("mutate record");
        assert!(store.revalidate().is_err());
    }
}

#[test]
fn rejects_nil_or_noncanonical_operation_nonces() {
    let fixture = Fixture::new();
    for nonce in [
        "0000000000000000000000000000000000000000000000000000000000000000",
        "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
        "1234",
    ] {
        assert!(
            DurableLifecycleStoreV3::create(
                &fixture.operations,
                nonce,
                fixture.uid(),
                fixture.gid(),
            )
            .is_err(),
            "{nonce}"
        );
    }
}

#[test]
fn fresh_store_requires_and_retains_the_exact_s1_census() {
    let temporary = tempfile::tempdir().expect("control parent");
    let root = temporary.path().join("control");
    let control =
        LivePrivilegedDisposablePolicyV2::create_for_test(&root).expect("create S1 control");
    let assessment = control.assess_read_only().expect("closed-world S1 census");
    assert!(assessment.receipt().new_operation_precondition_satisfied);
    assert!(!assessment.receipt().admission_authority);
    assert!(!assessment.receipt().authority.any());
    let census = assessment
        .into_fresh_control_census()
        .expect("consume live S1 census");
    let mut store = CensusBoundDurableLifecycleStoreV3::create(census, NONCE)
        .expect("create census-bound S2 store");
    assert_eq!(store.operation_nonce(), NONCE);
    assert!(!store.poisoned());
    store.census.revalidate().expect("retained admitted census");
    let operation_path = root.join("operations").join(format!("operation-{NONCE}"));
    let issue_root = operation_path.join("effect-issues-v3");
    let issue_metadata = fs::metadata(&issue_root).expect("mandatory V3 issue root");
    assert!(issue_metadata.is_dir());
    assert_eq!(issue_metadata.permissions().mode() & 0o7777, 0o700);
    assert_eq!(fs::read_dir(&issue_root).expect("issue roster").count(), 0);
    let mut operation_roster = fs::read_dir(&operation_path)
        .expect("operation roster")
        .map(|entry| entry.unwrap().file_name())
        .collect::<Vec<_>>();
    operation_roster.sort();
    assert_eq!(
        operation_roster,
        [std::ffi::OsString::from("effect-issues-v3")]
    );

    let record_digest = store
        .persist_prepared(sealed_prepared())
        .expect("append while census and flock remain retained");
    assert_eq!(record_digest.len(), 64);
    assert!(!store.poisoned());
}

#[test]
fn unmarked_legacy_operation_is_blocked_from_the_v3_restart_path() {
    let temporary = tempfile::tempdir().expect("control parent");
    let root = temporary.path().join("control");
    let control =
        LivePrivilegedDisposablePolicyV2::create_for_test(&root).expect("create S1 control");
    drop(control);
    let operations = File::open(root.join("operations")).expect("open operations");
    let mut legacy =
        DurableLifecycleStoreV3::create(&operations, NONCE, unsafe { libc::geteuid() }, unsafe {
            libc::getegid()
        })
        .expect("test-only unmarked V2 operation");
    let mut journal = DisposableLifecycleJournalV2::new(NONCE).expect("legacy journal");
    legacy
        .append(&mut journal, prepared())
        .expect("legacy prepared record");
    drop(legacy);
    drop(operations);

    let control =
        LivePrivilegedDisposablePolicyV2::create_for_test(&root).expect("reopen S1 control");
    let assessment = control
        .assess_read_only()
        .expect("S1 retains unmarked operation as a blocker");
    let census = assessment
        .into_blocking_control_census(NONCE)
        .expect("select exact unmarked blocker");
    let epoch = FreshProcessEpochV3::establish().expect("fresh process epoch");
    assert!(
        ReconciliationOperationStoreV3::open_existing(census, &epoch).is_err(),
        "caller absence or a boolean silently upgraded an unmarked V2 operation"
    );
}

#[test]
fn fresh_wrapper_rejects_missing_aliased_or_replaced_issue_roots() {
    for mutation in ["missing", "alias", "replace"] {
        let temporary = tempfile::tempdir().expect("control parent");
        let root = temporary.path().join("control");
        let control =
            LivePrivilegedDisposablePolicyV2::create_for_test(&root).expect("create S1 control");
        let census = control
            .assess_read_only()
            .expect("fresh assessment")
            .into_fresh_control_census()
            .expect("fresh census");
        let mut store = CensusBoundDurableLifecycleStoreV3::create(census, NONCE)
            .expect("new-format operation");
        let operation = root.join("operations").join(format!("operation-{NONCE}"));
        let issue_root = operation.join("effect-issues-v3");
        match mutation {
            "missing" => fs::remove_dir(&issue_root).expect("remove retained issue root"),
            "alias" => {
                let alias = operation.join(".incoming-effect-issues-v3");
                fs::create_dir(&alias).expect("create issue-root alias");
                fs::set_permissions(&alias, fs::Permissions::from_mode(0o700)).expect("alias mode");
            }
            "replace" => {
                let displaced = operation.join("effect-issues-v3-displaced");
                fs::rename(&issue_root, &displaced).expect("displace issue root");
                fs::create_dir(&issue_root).expect("replace issue root");
                fs::set_permissions(&issue_root, fs::Permissions::from_mode(0o700))
                    .expect("replacement mode");
            }
            _ => unreachable!(),
        }
        assert!(
            store.append(prepared()).is_err(),
            "{mutation} issue root was accepted by the retained fresh wrapper"
        );
        assert!(store.poisoned(), "{mutation} did not poison the wrapper");
    }
}

#[test]
fn census_bound_store_poisoned_by_foreign_operations_roster_mutation() {
    let temporary = tempfile::tempdir().expect("control parent");
    let root = temporary.path().join("control");
    let control =
        LivePrivilegedDisposablePolicyV2::create_for_test(&root).expect("create S1 control");
    let census = control
        .assess_read_only()
        .expect("closed-world S1 census")
        .into_fresh_control_census()
        .expect("fresh census");
    let mut store = CensusBoundDurableLifecycleStoreV3::create(census, NONCE)
        .expect("create census-bound store");
    let rogue = root
        .join("operations")
        .join(format!("operation-{}", digest('e')));
    fs::create_dir(&rogue).expect("inject foreign operation");
    fs::set_permissions(&rogue, fs::Permissions::from_mode(0o700)).expect("rogue mode");

    assert!(store.append(prepared()).is_err());
    assert!(store.poisoned());
    fs::remove_dir(&rogue).expect("remove rogue operation");
    assert!(store.append(prepared()).is_err());
}

#[test]
fn production_reconciliation_open_consumes_exact_blocking_census_and_owns_replay_journal() {
    let temporary = tempfile::tempdir().expect("control parent");
    let root = temporary.path().join("control");
    {
        let control =
            LivePrivilegedDisposablePolicyV2::create_for_test(&root).expect("create S1 control");
        let census = control
            .assess_read_only()
            .expect("fresh S1 assessment")
            .into_fresh_control_census()
            .expect("fresh census");
        let mut store = CensusBoundDurableLifecycleStoreV3::create(census, NONCE)
            .expect("fresh census-bound store");
        store.append(prepared()).expect("durable prepared record");
        assert_eq!(
            store.journal.process_mode(),
            LifecycleProcessModeV2::FreshProcess
        );
    }

    let control =
        LivePrivilegedDisposablePolicyV2::create_for_test(&root).expect("restart S1 control");
    let assessment = control.assess_read_only().expect("blocking S1 assessment");
    assert_eq!(assessment.receipt().blocking_operation_nonces, [NONCE]);
    let census = assessment
        .into_blocking_control_census(NONCE)
        .expect("exact blocking census");
    let epoch = FreshProcessEpochV3::establish().expect("fresh process epoch");
    let mut store = ReconciliationOperationStoreV3::open_existing(census, &epoch)
        .expect("production exact-census replay")
        .activate_without_admission_for_test()
        .expect("test active restart typestate");
    assert_eq!(store.operation_nonce(), NONCE);
    assert_eq!(
        store.journal.process_mode(),
        LifecycleProcessModeV2::RestartReconcileOnly
    );
    assert!(!store.poisoned());

    let restart_record_digest = store
        .append_reconciliation(ReconciliationLifecycleEventV3::restart_started(
            current_boot_session_uuid().expect("current boot UUID"),
            digest('c'),
            100,
            digest('e'),
        ))
        .expect("append with wrapper-owned replay journal");
    assert_eq!(restart_record_digest.len(), 64);
    let second_digest = store
        .append_reconciliation(ReconciliationLifecycleEventV3::manual_intervention(digest(
            'f',
        )))
        .expect("retain a second exact lifecycle append");
    assert_eq!(second_digest.len(), 64);
    assert_eq!(store.store.records.len(), 3);
    assert_eq!(store.census.selected_record_count(), 3);
    let inspection = inspect_lifecycle_v2(
        &store
            .store
            .records
            .iter()
            .map(|record| record.bytes.clone())
            .collect::<Vec<_>>(),
    )
    .expect("inspect exact no-authority chain");
    assert_eq!(inspection.authority, DisposableAuthorityV2::none());
    assert!(inspection.blocks_new_operations);
    assert!(!inspection.restart_forward_flow_authority);
}

#[test]
fn recovered_death_proof_consumes_exact_s1_issue_and_reacquired_lease() {
    let temporary = tempfile::tempdir().expect("control parent");
    let root = temporary.path().join("control");
    let boot = current_boot_session_uuid().expect("current boot");
    let epochs = EffectEpochEvidenceV3::bind_current_boot(
        &boot,
        &digest('3'),
        &digest('4'),
        &digest('5'),
        &digest('6'),
    )
    .expect("old issued epoch");
    create_recoverable_issue(&root, epochs);

    let control =
        LivePrivilegedDisposablePolicyV2::create_for_test(&root).expect("recovery S1 control");
    let census = control
        .assess_read_only()
        .expect("recovery blocking assessment")
        .into_blocking_control_census(NONCE)
        .expect("recovery blocking census");
    let epoch = FreshProcessEpochV3::establish().expect("fresh recovery epoch");
    let mut store = ReconciliationOperationStoreV3::open_existing(census, &epoch)
        .expect("exact recovered operation")
        .activate_without_admission_for_test()
        .expect("test active restart typestate");
    let proof = store
        .recover_latest_runner_death()
        .expect("same-boot old PID/start identities are absent");
    assert_eq!(proof.sha256().expect("sealed recovered digest").len(), 64);
    drop(proof);
    assert!(!store.poisoned());
}

#[test]
fn recovered_death_proof_final_replay_rejects_same_bytes_issue_swap() {
    let temporary = tempfile::tempdir().expect("control parent");
    let root = temporary.path().join("control");
    let boot = current_boot_session_uuid().expect("current boot");
    let epochs = EffectEpochEvidenceV3::bind_current_boot(
        &boot,
        &digest('3'),
        &digest('4'),
        &digest('5'),
        &digest('6'),
    )
    .expect("old issued epoch");
    let issue_root = create_recoverable_issue(&root, epochs);

    let control =
        LivePrivilegedDisposablePolicyV2::create_for_test(&root).expect("recovery S1 control");
    let census = control
        .assess_read_only()
        .expect("recovery blocking assessment")
        .into_blocking_control_census(NONCE)
        .expect("recovery blocking census");
    let epoch = FreshProcessEpochV3::establish().expect("fresh recovery epoch");
    let mut store = ReconciliationOperationStoreV3::open_existing(census, &epoch)
        .expect("exact recovered operation")
        .activate_without_admission_for_test()
        .expect("test active restart typestate");
    let issue = fs::read_dir(&issue_root)
        .expect("issue roster")
        .next()
        .expect("one issue")
        .expect("issue entry")
        .path();
    let displaced = issue_root.join("displaced-after-absence");
    let result = store.recover_latest_runner_death_with_hook(|| {
        let bytes = fs::read(&issue)?;
        fs::rename(&issue, &displaced)?;
        fs::write(&issue, bytes)?;
        fs::set_permissions(&issue, fs::Permissions::from_mode(0o400))?;
        Ok(())
    });
    assert!(
        result.is_err(),
        "post-absence same-bytes swap minted a proof"
    );
}

#[test]
fn fresh_append_pre_retain_emfile_cutpoint_is_permanently_poisoned() {
    let temporary = tempfile::tempdir().expect("control parent");
    let root = temporary.path().join("control");
    let control =
        LivePrivilegedDisposablePolicyV2::create_for_test(&root).expect("create S1 control");
    let census = control
        .assess_read_only()
        .expect("fresh S1 assessment")
        .into_fresh_control_census()
        .expect("fresh census");
    let mut store = CensusBoundDurableLifecycleStoreV3::create(census, NONCE)
        .expect("fresh census-bound store");

    let result = store.append_with_pre_retain_hook(prepared(), || {
        Err(io::Error::from_raw_os_error(libc::EMFILE))
    });

    assert!(result.is_err(), "post-durable EMFILE cutpoint was accepted");
    assert!(store.poisoned(), "post-durable EMFILE did not poison S2");
    assert_eq!(store.store.records.len(), 1, "V2 append was not durable");
    assert_eq!(
        store.census.selected_record_count(),
        0,
        "S1 adopted a record after the pre-retain failure"
    );
    assert!(
        store.append(prepared()).is_err(),
        "poisoned fresh wrapper accepted a retry"
    );
}

#[test]
fn reconciliation_append_same_bytes_swap_before_retain_is_permanently_poisoned() {
    let temporary = tempfile::tempdir().expect("control parent");
    let root = temporary.path().join("control");
    {
        let control =
            LivePrivilegedDisposablePolicyV2::create_for_test(&root).expect("create S1 control");
        let census = control
            .assess_read_only()
            .expect("fresh S1 assessment")
            .into_fresh_control_census()
            .expect("fresh census");
        let mut store = CensusBoundDurableLifecycleStoreV3::create(census, NONCE)
            .expect("fresh census-bound store");
        store.append(prepared()).expect("durable prepared record");
    }

    let control =
        LivePrivilegedDisposablePolicyV2::create_for_test(&root).expect("restart S1 control");
    let census = control
        .assess_read_only()
        .expect("blocking S1 assessment")
        .into_blocking_control_census(NONCE)
        .expect("exact blocking census");
    let epoch = FreshProcessEpochV3::establish().expect("fresh process epoch");
    let mut store = ReconciliationOperationStoreV3::open_existing(census, &epoch)
        .expect("production exact-census replay")
        .activate_without_admission_for_test()
        .expect("test active restart typestate");
    let operation = root.join("operations").join(format!("operation-{NONCE}"));
    let record = operation.join("00000002.json");
    let displaced = operation.join("00000002.pre-retain-displaced");
    let result = store.append_reconciliation_with_pre_retain_hook(
        ReconciliationLifecycleEventV3::restart_started(
            current_boot_session_uuid().expect("current boot UUID"),
            digest('c'),
            100,
            digest('e'),
        ),
        || {
            let bytes = fs::read(&record)?;
            fs::rename(&record, &displaced)?;
            fs::write(&record, bytes)?;
            fs::set_permissions(&record, fs::Permissions::from_mode(0o400))?;
            Ok(())
        },
    );

    assert!(result.is_err(), "pre-retain same-bytes swap was accepted");
    assert!(store.poisoned(), "pre-retain swap did not poison S2");
    assert_eq!(store.store.records.len(), 2, "V2 append was not durable");
    assert_eq!(
        store.census.selected_record_count(),
        1,
        "S1 adopted the swapped record"
    );
    assert!(
        store
            .append_reconciliation(ReconciliationLifecycleEventV3::manual_intervention(digest(
                'f'
            )))
            .is_err(),
        "poisoned reconciliation wrapper accepted a retry"
    );
}

#[test]
fn caught_outer_transaction_panic_after_v2_adoption_is_permanently_poisoned() {
    let temporary = tempfile::tempdir().expect("control parent");
    let root = temporary.path().join("control");
    {
        let control =
            LivePrivilegedDisposablePolicyV2::create_for_test(&root).expect("create S1 control");
        let census = control
            .assess_read_only()
            .expect("fresh S1 assessment")
            .into_fresh_control_census()
            .expect("fresh census");
        let mut store = CensusBoundDurableLifecycleStoreV3::create(census, NONCE)
            .expect("fresh census-bound store");
        store.append(prepared()).expect("durable prepared record");
    }

    let control =
        LivePrivilegedDisposablePolicyV2::create_for_test(&root).expect("restart S1 control");
    let census = control
        .assess_read_only()
        .expect("blocking S1 assessment")
        .into_blocking_control_census(NONCE)
        .expect("exact blocking census");
    let epoch = FreshProcessEpochV3::establish().expect("fresh process epoch");
    let mut store = ReconciliationOperationStoreV3::open_existing(census, &epoch)
        .expect("production exact-census replay")
        .activate_without_admission_for_test()
        .expect("test active restart typestate");

    let panic = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _ = store.append_reconciliation_outer_transaction_with_hook(
            ReconciliationLifecycleEventV3::restart_started(
                current_boot_session_uuid().expect("current boot UUID"),
                digest('c'),
                100,
                digest('e'),
            ),
            || -> io::Result<()> { panic!("outer post-V2 cutpoint") },
        );
    }));

    assert!(panic.is_err(), "outer cutpoint did not panic");
    assert!(store.poisoned(), "caught outer panic disarmed the wrapper");
    assert_eq!(store.store.records.len(), 2, "V2 append was not durable");
    assert_eq!(
        store.census.selected_record_count(),
        2,
        "outer cutpoint ran before exact S1 adoption"
    );
    assert!(
        store
            .append_reconciliation(ReconciliationLifecycleEventV3::manual_intervention(digest(
                'f'
            )))
            .is_err(),
        "caught outer panic left a reusable wrapper"
    );
}

#[test]
fn reconciliation_append_rejects_same_bytes_swap_before_s1_adoption() {
    let temporary = tempfile::tempdir().expect("control parent");
    let root = temporary.path().join("control");
    {
        let control =
            LivePrivilegedDisposablePolicyV2::create_for_test(&root).expect("create S1 control");
        let census = control
            .assess_read_only()
            .expect("fresh S1 assessment")
            .into_fresh_control_census()
            .expect("fresh census");
        let mut store = CensusBoundDurableLifecycleStoreV3::create(census, NONCE)
            .expect("fresh census-bound store");
        store.append(prepared()).expect("durable prepared record");
    }

    let control =
        LivePrivilegedDisposablePolicyV2::create_for_test(&root).expect("restart S1 control");
    let census = control
        .assess_read_only()
        .expect("blocking S1 assessment")
        .into_blocking_control_census(NONCE)
        .expect("exact blocking census");
    let epoch = FreshProcessEpochV3::establish().expect("fresh process epoch");
    let mut store = ReconciliationOperationStoreV3::open_existing(census, &epoch)
        .expect("production exact-census replay")
        .activate_without_admission_for_test()
        .expect("test active restart typestate");
    let operation = root.join("operations").join(format!("operation-{NONCE}"));
    let record = operation.join("00000002.json");
    let displaced = operation.join("00000002.displaced");
    let result = store.append_reconciliation_with_s1_adoption_hook(
        ReconciliationLifecycleEventV3::restart_started(
            current_boot_session_uuid().expect("current boot UUID"),
            digest('c'),
            100,
            digest('e'),
        ),
        || {
            let bytes = fs::read(&record)?;
            fs::rename(&record, &displaced)?;
            fs::write(&record, bytes)?;
            fs::set_permissions(&record, fs::Permissions::from_mode(0o400))?;
            Ok(())
        },
    );
    assert!(result.is_err());
    assert!(store.poisoned());
    assert_eq!(store.census.selected_record_count(), 1);
}

#[test]
fn effect_issue_append_rejects_same_bytes_swap_before_s1_adoption() {
    let temporary = tempfile::tempdir().expect("control parent");
    let root = temporary.path().join("control");
    {
        let control =
            LivePrivilegedDisposablePolicyV2::create_for_test(&root).expect("create S1 control");
        let census = control
            .assess_read_only()
            .expect("fresh S1 assessment")
            .into_fresh_control_census()
            .expect("fresh census");
        let mut store = CensusBoundDurableLifecycleStoreV3::create(census, NONCE)
            .expect("fresh census-bound store");
        store.append(prepared()).expect("durable prepared record");
    }

    let control =
        LivePrivilegedDisposablePolicyV2::create_for_test(&root).expect("restart S1 control");
    let census = control
        .assess_read_only()
        .expect("blocking S1 assessment")
        .into_blocking_control_census(NONCE)
        .expect("exact blocking census");
    let epoch = FreshProcessEpochV3::establish().expect("fresh process epoch");
    let mut store = ReconciliationOperationStoreV3::open_existing(census, &epoch)
        .expect("production exact-census replay")
        .activate_without_admission_for_test()
        .expect("test active restart typestate");
    let boot = current_boot_session_uuid().expect("current boot UUID");
    store
        .append_reconciliation(ReconciliationLifecycleEventV3::restart_started(
            boot.clone(),
            digest('c'),
            100,
            digest('e'),
        ))
        .expect("restart epoch");
    store
        .append_reconciliation(ReconciliationLifecycleEventV3::snapshot_observed(
            ReconciliationSnapshotV2 {
                backing_identity_sha256: digest('b'),
                boot_session_uuid: boot.clone(),
                collector_policy_sha256: digest('c'),
                collector_receipt_file: None,
                collector_receipt_sha256: digest('f'),
                current_expected_absence_inventory_sha256: Some(digest('9')),
                iomedia_evidence_sha256: digest('1'),
                match_result: ReconciliationMatchV2::Unique { mounted: true },
                monotonic_after_nanoseconds: 102,
                monotonic_before_nanoseconds: 101,
                mount_evidence_sha256: digest('2'),
                mountpoint_underlying_sha256: digest('d'),
                operation_nonce: NONCE.to_string(),
                restart_epoch_nonce: digest('e'),
            },
        ))
        .expect("unique mounted snapshot");
    store
        .append_reconciliation_inner(ReconciliationLifecycleEventV3::unmount_issued(1), true)
        .expect("durable V2 issue capsule");
    let lifecycle = VerifiedLifecycleIssueRosterV3::capture_from_s2(store.store.issue_source())
        .expect("exact V2 issue roster");
    let epochs = EffectEpochEvidenceV3::bind_current_boot(
        &boot,
        &digest('3'),
        &digest('4'),
        &digest('5'),
        &digest('6'),
    )
    .expect("test epoch evidence");
    let mut retained = store
        .issues
        .persist(
            &lifecycle,
            ExactDisposableCommandV3::UnmountVolume {
                mounted_binding_sha256: digest('7'),
            },
            epochs,
            Some(digest('8')),
        )
        .expect("persist exact V3 issue");
    let issue_root = root
        .join("operations")
        .join(format!("operation-{NONCE}"))
        .join("effect-issues-v3");
    let issue = fs::read_dir(&issue_root)
        .expect("issue roster")
        .next()
        .expect("one issue")
        .expect("issue entry")
        .path();
    let displaced = issue_root.join("displaced-issue");
    let bytes = fs::read(&issue).expect("issue bytes");
    fs::rename(&issue, &displaced).expect("displace exact S2 issue inode");
    fs::write(&issue, bytes).expect("publish same bytes replacement");
    fs::set_permissions(&issue, fs::Permissions::from_mode(0o400)).expect("replacement mode");
    let sink = store
        .census
        .selected_effect_issue_sink()
        .expect("sealed S1 issue sink");
    assert!(retained.adopt_into_s1(sink).is_err());
    assert_eq!(store.census.selected_effect_issue_count(), 0);
}

#[test]
fn reconciliation_open_rejects_same_bytes_path_replacement_after_s1_descriptor_capture() {
    let temporary = tempfile::tempdir().expect("control parent");
    let root = temporary.path().join("control");
    {
        let control =
            LivePrivilegedDisposablePolicyV2::create_for_test(&root).expect("create S1 control");
        let census = control
            .assess_read_only()
            .expect("fresh assessment")
            .into_fresh_control_census()
            .expect("fresh census");
        let mut store = CensusBoundDurableLifecycleStoreV3::create(census, NONCE)
            .expect("fresh census-bound store");
        store.append(prepared()).expect("durable prepared record");
    }

    let control =
        LivePrivilegedDisposablePolicyV2::create_for_test(&root).expect("restart S1 control");
    let census = control
        .assess_read_only()
        .expect("blocking assessment")
        .into_blocking_control_census(NONCE)
        .expect("blocking census");
    let epoch = FreshProcessEpochV3::establish().expect("fresh process epoch");
    let operation = root.join("operations").join(format!("operation-{NONCE}"));
    let stale = temporary.path().join("stale-selected-operation");
    let original_record = fs::read(operation.join("00000001.json")).expect("original bytes");
    let result = ReconciliationOperationStoreV3::open_existing_with_hook(census, &epoch, || {
        fs::rename(&operation, &stale)?;
        fs::create_dir(&operation)?;
        fs::set_permissions(&operation, fs::Permissions::from_mode(0o700))?;
        fs::write(operation.join("00000001.json"), &original_record)?;
        fs::set_permissions(
            operation.join("00000001.json"),
            fs::Permissions::from_mode(0o400),
        )?;
        File::open(operation.join("00000001.json"))?.sync_all()?;
        File::open(&operation)?.sync_all()?;
        Ok(())
    });
    let error = result.err().expect("same-byte path replacement must fail");
    assert!(
        error.to_string().contains("exact retained S1")
            || error.to_string().contains("descriptor replay")
            || error.to_string().contains("changed"),
        "unexpected error: {error}"
    );
}

#[test]
fn reconciliation_open_rejects_same_bytes_record_replacement_after_s1_descriptor_capture() {
    let temporary = tempfile::tempdir().expect("control parent");
    let root = temporary.path().join("control");
    {
        let control =
            LivePrivilegedDisposablePolicyV2::create_for_test(&root).expect("create S1 control");
        let census = control
            .assess_read_only()
            .expect("fresh assessment")
            .into_fresh_control_census()
            .expect("fresh census");
        let mut store = CensusBoundDurableLifecycleStoreV3::create(census, NONCE)
            .expect("fresh census-bound store");
        store.append(prepared()).expect("durable prepared record");
    }

    let control =
        LivePrivilegedDisposablePolicyV2::create_for_test(&root).expect("restart S1 control");
    let census = control
        .assess_read_only()
        .expect("blocking assessment")
        .into_blocking_control_census(NONCE)
        .expect("blocking census");
    let epoch = FreshProcessEpochV3::establish().expect("fresh process epoch");
    let record = root
        .join("operations")
        .join(format!("operation-{NONCE}"))
        .join("00000001.json");
    let stale = temporary.path().join("stale-selected-record.json");
    let original_bytes = fs::read(&record).expect("original record bytes");
    let result = ReconciliationOperationStoreV3::open_existing_with_hook(census, &epoch, || {
        fs::rename(&record, &stale)?;
        fs::write(&record, &original_bytes)?;
        fs::set_permissions(&record, fs::Permissions::from_mode(0o400))?;
        File::open(&record)?.sync_all()?;
        Ok(())
    });
    let error = result
        .err()
        .expect("same-byte record replacement must fail");
    assert!(
        error.to_string().contains("exact retained S1")
            || error.to_string().contains("descriptor")
            || error.to_string().contains("changed"),
        "unexpected error: {error}"
    );
}

#[test]
fn reconciliation_open_rejects_issue_root_replacement_after_s1_descriptor_capture() {
    let temporary = tempfile::tempdir().expect("control parent");
    let root = temporary.path().join("control");
    {
        let control =
            LivePrivilegedDisposablePolicyV2::create_for_test(&root).expect("create S1 control");
        let census = control
            .assess_read_only()
            .expect("fresh assessment")
            .into_fresh_control_census()
            .expect("fresh census");
        let mut store = CensusBoundDurableLifecycleStoreV3::create(census, NONCE)
            .expect("fresh census-bound store");
        store.append(prepared()).expect("durable prepared record");
    }

    let control =
        LivePrivilegedDisposablePolicyV2::create_for_test(&root).expect("restart S1 control");
    let census = control
        .assess_read_only()
        .expect("blocking assessment")
        .into_blocking_control_census(NONCE)
        .expect("blocking census");
    let epoch = FreshProcessEpochV3::establish().expect("fresh process epoch");
    let operation = root.join("operations").join(format!("operation-{NONCE}"));
    let issue_root = operation.join("effect-issues-v3");
    let stale = temporary.path().join("stale-effect-issues-v3");
    let result = ReconciliationOperationStoreV3::open_existing_with_hook(census, &epoch, || {
        fs::rename(&issue_root, &stale)?;
        fs::create_dir(&issue_root)?;
        fs::set_permissions(&issue_root, fs::Permissions::from_mode(0o700))?;
        File::open(&issue_root)?.sync_all()?;
        File::open(&operation)?.sync_all()?;
        Ok(())
    });
    let error = result
        .err()
        .expect("replacement issue root must not replace retained S1 evidence");
    assert!(
        error.to_string().contains("exact retained S1")
            || error.to_string().contains("descriptor")
            || error.to_string().contains("changed"),
        "unexpected error: {error}"
    );
}

#[test]
fn terminal_append_consumes_blocking_store_into_completed_census() {
    let temporary = tempfile::tempdir().expect("control parent");
    let root = temporary.path().join("control");
    {
        let control =
            LivePrivilegedDisposablePolicyV2::create_for_test(&root).expect("create S1 control");
        let census = control
            .assess_read_only()
            .expect("fresh assessment")
            .into_fresh_control_census()
            .expect("fresh census");
        let mut store = CensusBoundDurableLifecycleStoreV3::create(census, NONCE)
            .expect("fresh census-bound store");
        store.append(prepared()).expect("durable prepared record");
    }

    let control =
        LivePrivilegedDisposablePolicyV2::create_for_test(&root).expect("restart S1 control");
    let census = control
        .assess_read_only()
        .expect("blocking assessment")
        .into_blocking_control_census(NONCE)
        .expect("blocking census");
    let epoch = FreshProcessEpochV3::establish().expect("fresh process epoch");
    let mut store = ReconciliationOperationStoreV3::open_existing(census, &epoch)
        .expect("exact descriptor replay")
        .activate_without_admission_for_test()
        .expect("test active restart typestate");
    let boot = current_boot_session_uuid().expect("current boot UUID");
    let restart_epoch = digest('e');
    store
        .append_reconciliation(ReconciliationLifecycleEventV3::restart_started(
            boot.clone(),
            digest('c'),
            100,
            restart_epoch.clone(),
        ))
        .expect("restart epoch");
    let snapshot = ReconciliationSnapshotV2 {
        backing_identity_sha256: digest('b'),
        boot_session_uuid: boot.clone(),
        collector_policy_sha256: digest('c'),
        collector_receipt_file: None,
        collector_receipt_sha256: digest('f'),
        current_expected_absence_inventory_sha256: Some(digest('1')),
        iomedia_evidence_sha256: digest('1'),
        match_result: ReconciliationMatchV2::Zero,
        monotonic_after_nanoseconds: 102,
        monotonic_before_nanoseconds: 101,
        mount_evidence_sha256: digest('2'),
        mountpoint_underlying_sha256: digest('d'),
        operation_nonce: NONCE.to_string(),
        restart_epoch_nonce: restart_epoch.clone(),
    };
    let snapshot_sha256 = reconciliation_snapshot_sha256(&snapshot).expect("snapshot digest");
    store
        .append_reconciliation(ReconciliationLifecycleEventV3::snapshot_observed(snapshot))
        .expect("zero-match snapshot");
    let absence = FreshAbsenceObservationV2 {
        artifact_evidence_sha256: digest('3'),
        baseline_inventory_sha256: digest('a'),
        backing_identity_sha256: digest('b'),
        boot_session_uuid: boot,
        collector_policy_sha256: digest('c'),
        collector_receipt_file: None,
        collector_receipt_sha256: digest('4'),
        current_expected_absence_inventory_sha256: Some(digest('1')),
        iomedia_evidence_sha256: digest('1'),
        monotonic_after_nanoseconds: 104,
        monotonic_before_nanoseconds: 103,
        mount_evidence_sha256: digest('6'),
        mountpoint_underlying_sha256: digest('d'),
        no_matching_iomedia: true,
        no_nested_mounts: true,
        operation_nonce: NONCE.to_string(),
        operation_artifacts_absent: true,
        post_inventory_sha256: digest('a'),
        reconciliation_snapshot_sha256: Some(snapshot_sha256),
        restart_epoch_nonce: Some(restart_epoch),
    };
    let absence_sha256 = fresh_absence_sha256(&absence).expect("absence digest");
    store
        .append_reconciliation(ReconciliationLifecycleEventV3::fresh_absence_observed(
            absence,
        ))
        .expect("fresh absence");
    let records_before = store.store.records.len();
    assert!(
        store
            .append_reconciliation(ReconciliationLifecycleEventV3(
                DisposableLifecycleEventV2::TerminalAbsenceProved {
                    disposition: TerminalDispositionV2::Aborted,
                    fresh_absence_sha256: absence_sha256.clone(),
                },
            ))
            .is_err()
    );
    assert_eq!(store.store.records.len(), records_before);

    let completed = store
        .complete_reconciliation(ReconciliationTerminalEventV3::absence_proved(
            TerminalDispositionV2::Aborted,
            absence_sha256.clone(),
        ))
        .expect("consume blocking state into completed state");
    completed.revalidate().expect("retained completed state");
    let (nonce, terminal, _) = completed.census.completed_binding();
    assert_eq!(nonce, NONCE);
    assert_eq!(Some(terminal), completed.journal.terminal_record_sha256());
    assert_eq!(
        completed.census.completed_operation_name(),
        format!("operation-{NONCE}")
    );
    assert!(
        completed
            .census
            .completed_receipt()
            .blocking_operation_nonces
            .is_empty()
    );
    assert_eq!(
        completed
            .census
            .completed_receipt()
            .completed_operation_nonces,
        [NONCE]
    );
}

#[test]
fn reconciliation_wrapper_is_poisoned_by_nonselected_blocker_drift() {
    let temporary = tempfile::tempdir().expect("control parent");
    let root = temporary.path().join("control");
    let other_nonce = digest('2');
    {
        let control =
            LivePrivilegedDisposablePolicyV2::create_for_test(&root).expect("create S1 control");
        let first = control
            .assess_read_only()
            .expect("fresh assessment")
            .into_fresh_control_census()
            .expect("fresh census");
        let mut first_store =
            CensusBoundDurableLifecycleStoreV3::create(first, NONCE).expect("first operation");
        first_store.append(prepared()).expect("first prepared");
    }
    {
        let control =
            LivePrivilegedDisposablePolicyV2::create_for_test(&root).expect("second S1 control");
        let assessment = control
            .assess_read_only()
            .expect("first blocker assessment");
        assert!(assessment.into_fresh_control_census().is_err());
    }
    // Add a second valid blocking operation through the raw constructor used
    // only by tests; production has no path around S1 admission.
    let operations = File::open(root.join("operations")).expect("open operations");
    let mut raw = DurableLifecycleStoreV3::create(
        &operations,
        &other_nonce,
        unsafe { libc::geteuid() },
        unsafe { libc::getegid() },
    )
    .expect("test-only second operation");
    let mut raw_journal =
        DisposableLifecycleJournalV2::new(&other_nonce).expect("test-only journal");
    raw.append(&mut raw_journal, prepared())
        .expect("second prepared");
    drop(raw);
    drop(operations);

    let control =
        LivePrivilegedDisposablePolicyV2::create_for_test(&root).expect("restart S1 control");
    let census = control
        .assess_read_only()
        .expect("two-blocker assessment")
        .into_blocking_control_census(NONCE)
        .expect("select first blocker");
    let epoch = FreshProcessEpochV3::establish().expect("fresh process epoch");
    let mut store = ReconciliationOperationStoreV3::open_existing(census, &epoch)
        .expect("open exact selected blocker")
        .activate_without_admission_for_test()
        .expect("test active restart typestate");

    let other_record = root
        .join("operations")
        .join(format!("operation-{other_nonce}"))
        .join("00000001.json");
    fs::set_permissions(&other_record, fs::Permissions::from_mode(0o600))
        .expect("mutate nonselected blocker");
    assert!(
        store
            .append_reconciliation(ReconciliationLifecycleEventV3::restart_started(
                current_boot_session_uuid().expect("current boot UUID"),
                digest('c'),
                100,
                digest('f'),
            ))
            .is_err()
    );
    assert!(store.poisoned());
}
