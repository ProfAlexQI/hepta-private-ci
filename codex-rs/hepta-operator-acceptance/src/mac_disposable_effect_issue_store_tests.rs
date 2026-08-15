use super::*;
use crate::mac_disposable_lifecycle::CallbackOutcomeV2;
use crate::mac_disposable_lifecycle::DisposableLifecycleJournalV2;
use crate::mac_disposable_lifecycle::ReconciliationMatchV2;
use crate::mac_disposable_lifecycle::ReconciliationSnapshotV2;
use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::path::PathBuf;
use tempfile::TempDir;

const NONCE: &str = "1111111111111111111111111111111111111111111111111111111111111111";
const RESTART_NONCE: &str = "2222222222222222222222222222222222222222222222222222222222222222";
const PROCESS_NONCE: &str = "3333333333333333333333333333333333333333333333333333333333333333";
const RUNNER_NONCE: &str = "4444444444444444444444444444444444444444444444444444444444444444";

macro_rules! assert_not_impl_any {
    ($ty:ty: $($trait:path),+ $(,)?) => {
        const _: fn() = || {
            trait AmbiguousIfImpl<A> {
                fn some_item() {}
            }
            impl<T: ?Sized> AmbiguousIfImpl<()> for T {}
            $({
                struct Invalid;
                impl<T: ?Sized + $trait> AmbiguousIfImpl<Invalid> for T {}
            })+
            let _ = <$ty as AmbiguousIfImpl<_>>::some_item;
        };
    };
}

assert_not_impl_any!(
    RetainedDurableEffectIssueV3<'static>:
        Clone,
        Send,
        Sync,
        Serialize,
        serde::de::DeserializeOwned,
        AsRawFd,
        std::os::fd::AsFd,
        std::os::fd::IntoRawFd,
        From<File>,
        TryFrom<File>
);

fn digest(byte: char) -> String {
    byte.to_string().repeat(64)
}

fn append(
    journal: &mut DisposableLifecycleJournalV2,
    records: &mut Vec<Vec<u8>>,
    event: DisposableLifecycleEventV2,
) {
    journal
        .append_with(event, |_, bytes| {
            records.push(bytes.to_vec());
            Ok(())
        })
        .expect("append lifecycle record");
}

fn prepared(boot: &str) -> DisposableLifecycleEventV2 {
    DisposableLifecycleEventV2::OperationPrepared {
        baseline_inventory_sha256: digest('a'),
        backing_identity_sha256: digest('b'),
        boot_session_uuid: boot.to_string(),
        collector_policy_sha256: digest('c'),
        mountpoint_underlying_sha256: digest('d'),
    }
}

fn reconciliation_snapshot(boot: &str) -> ReconciliationSnapshotV2 {
    ReconciliationSnapshotV2 {
        backing_identity_sha256: digest('b'),
        boot_session_uuid: boot.to_string(),
        collector_policy_sha256: digest('c'),
        collector_receipt_sha256: digest('e'),
        iomedia_evidence_sha256: digest('f'),
        match_result: ReconciliationMatchV2::Unique { mounted: true },
        monotonic_after_nanoseconds: 102,
        monotonic_before_nanoseconds: 101,
        mount_evidence_sha256: digest('5'),
        mountpoint_underlying_sha256: digest('d'),
        operation_nonce: NONCE.to_string(),
        restart_epoch_nonce: RESTART_NONCE.to_string(),
    }
}

struct LifecycleFixture {
    boot: String,
    journal: DisposableLifecycleJournalV2,
    records: Vec<Vec<u8>>,
    before_issue: VerifiedLifecycleIssueRosterV3,
}

impl LifecycleFixture {
    fn new() -> Self {
        let boot = current_boot_session_uuid().expect("current boot UUID");
        let mut initial = DisposableLifecycleJournalV2::new(NONCE).expect("fresh journal");
        let mut records = Vec::new();
        append(&mut initial, &mut records, prepared(&boot));
        let mut journal =
            DisposableLifecycleJournalV2::resume_for_reconciliation(&records).expect("restart");
        append(
            &mut journal,
            &mut records,
            DisposableLifecycleEventV2::RestartReconciliationStarted {
                boot_session_uuid: boot.clone(),
                collector_policy_sha256: digest('c'),
                monotonic_nanoseconds: 100,
                restart_epoch_nonce: RESTART_NONCE.to_string(),
            },
        );
        append(
            &mut journal,
            &mut records,
            DisposableLifecycleEventV2::ReconciliationSnapshotObserved {
                snapshot: reconciliation_snapshot(&boot),
            },
        );
        let before_issue =
            VerifiedLifecycleIssueRosterV3::replay(&records).expect("collector-bound lifecycle");
        Self {
            boot,
            journal,
            records,
            before_issue,
        }
    }

    fn append_unmount(&mut self) -> VerifiedLifecycleIssueRosterV3 {
        append(
            &mut self.journal,
            &mut self.records,
            DisposableLifecycleEventV2::UnmountIssuedOrUncertain {
                effect_id: 1,
                purpose: EffectPurposeV2::Reconciliation,
            },
        );
        VerifiedLifecycleIssueRosterV3::replay(&self.records).expect("one issued effect")
    }

    fn append_eject(&mut self) -> VerifiedLifecycleIssueRosterV3 {
        append(
            &mut self.journal,
            &mut self.records,
            DisposableLifecycleEventV2::UnmountCallbackObserved {
                effect_id: 1,
                outcome: CallbackOutcomeV2::Succeeded,
            },
        );
        append(
            &mut self.journal,
            &mut self.records,
            DisposableLifecycleEventV2::UnmountObserved {
                effect_id: 1,
                mount_absence_sha256: digest('6'),
            },
        );
        append(
            &mut self.journal,
            &mut self.records,
            DisposableLifecycleEventV2::EjectIssuedOrUncertain {
                effect_id: 2,
                purpose: EffectPurposeV2::Reconciliation,
            },
        );
        VerifiedLifecycleIssueRosterV3::replay(&self.records).expect("two issued effects")
    }

    fn epochs(&self, salt: char) -> EffectEpochEvidenceV3 {
        EffectEpochEvidenceV3::bind_current_boot(
            &self.boot,
            PROCESS_NONCE,
            &digest(salt),
            RUNNER_NONCE,
            &digest(if salt == '7' { '8' } else { '9' }),
        )
        .expect("fresh epoch binding")
    }
}

struct StoreFixture {
    _temporary: TempDir,
    operation: File,
    operation_path: PathBuf,
    store: DurableEffectIssueStoreV3,
}

impl StoreFixture {
    fn new(lifecycle: &VerifiedLifecycleIssueRosterV3) -> Self {
        let temporary = tempfile::Builder::new()
            .prefix(".effect-issue-store-")
            .tempdir()
            .expect("temporary root");
        std::fs::set_permissions(temporary.path(), std::fs::Permissions::from_mode(0o700))
            .expect("private temporary root");
        let operation_path = temporary.path().join(format!("operation-{NONCE}"));
        std::fs::create_dir(&operation_path).expect("operation directory");
        std::fs::set_permissions(&operation_path, std::fs::Permissions::from_mode(0o700))
            .expect("private operation directory");
        let operation = File::open(&operation_path).expect("open operation directory");
        let store = DurableEffectIssueStoreV3::create_new(
            &operation,
            lifecycle,
            unsafe { libc::geteuid() },
            unsafe { libc::getegid() },
        )
        .expect("create V3 issue store");
        Self {
            _temporary: temporary,
            operation,
            operation_path,
            store,
        }
    }

    fn issue_root(&self) -> PathBuf {
        self.operation_path.join(ISSUE_DIRECTORY_NAME_V3)
    }

    fn only_issue_path(&self) -> PathBuf {
        let mut entries = std::fs::read_dir(self.issue_root())
            .expect("read issue root")
            .map(|entry| entry.expect("entry").path())
            .collect::<Vec<_>>();
        entries.sort();
        assert_eq!(entries.len(), 1);
        entries.remove(0)
    }
}

fn unmount_command() -> ExactDisposableCommandV3 {
    ExactDisposableCommandV3::UnmountVolume {
        mounted_binding_sha256: digest('a'),
    }
}

fn eject_command() -> ExactDisposableCommandV3 {
    ExactDisposableCommandV3::EjectImage {
        disk_image_group_sha256: digest('b'),
    }
}

fn persist_unmount(
    fixture: &mut StoreFixture,
    lifecycle: &VerifiedLifecycleIssueRosterV3,
    epochs: EffectEpochEvidenceV3,
) -> String {
    let retained = fixture
        .store
        .persist(lifecycle, unmount_command(), epochs, Some(digest('c')))
        .expect("persist unmount issue");
    assert_eq!(retained.effect_id(), 1);
    assert_eq!(retained.record().effect_id(), 1);
    assert_eq!(
        retained.record().command_sha256(),
        sha256(retained.record().command_canonical_bytes())
    );
    assert_eq!(
        sha256(retained.record_canonical_bytes()),
        retained.record_sha256()
    );
    assert_eq!(retained.record().operation_nonce(), NONCE);
    assert_eq!(retained.record().process_epoch_sha256(), &digest('7'));
    assert_eq!(retained.record().runner_epoch_sha256(), &digest('8'));
    assert!(matches!(
        retained.record().command(),
        ExactDisposableCommandV3::UnmountVolume { .. }
    ));
    retained.revalidate().expect("retained final replay");
    retained.record_sha256().to_string()
}

#[test]
fn normal_publish_is_canonical_private_retained_and_replayable() {
    let mut lifecycle = LifecycleFixture::new();
    let mut fixture = StoreFixture::new(&lifecycle.before_issue);
    let issued = lifecycle.append_unmount();
    let record_digest = persist_unmount(&mut fixture, &issued, lifecycle.epochs('7'));
    assert!(valid_digest(&record_digest));
    assert!(!fixture.store.poisoned());
    let issue_path = fixture.only_issue_path();
    let actual_issue_name = issue_path
        .file_name()
        .expect("issue name")
        .to_str()
        .expect("UTF-8 issue name");
    assert_eq!(actual_issue_name, issue_name(1, &record_digest));
    assert_eq!(
        std::fs::metadata(fixture.issue_root())
            .expect("issue root metadata")
            .permissions()
            .mode()
            & 0o7777,
        0o700
    );
    let metadata = std::fs::metadata(&issue_path).expect("issue metadata");
    assert_eq!(metadata.permissions().mode() & 0o7777, 0o400);
    use std::os::unix::fs::MetadataExt;
    assert_eq!(metadata.nlink(), 1);

    drop(fixture.store);
    let reopened = DurableEffectIssueStoreV3::open_existing_required(
        &fixture.operation,
        &issued,
        unsafe { libc::geteuid() },
        unsafe { libc::getegid() },
    )
    .expect("exact restart replay");
    let record = reopened.replayed_issue(1).expect("replayed issue data");
    assert_eq!(
        record.command_sha256,
        sha256(record.command_canonical_json.as_bytes())
    );
    assert!(!record.authority.any());
    assert_eq!(record.prior_collector_receipt_sha256, digest('e'));
    assert_eq!(record.lifecycle_issue_sequence, 4);
    assert_eq!(record.prior_collector_lifecycle_sequence, 3);
}

#[test]
fn lifecycle_roster_growth_does_not_look_like_operation_inode_replacement() {
    let mut lifecycle = LifecycleFixture::new();
    let mut fixture = StoreFixture::new(&lifecycle.before_issue);
    let issued = lifecycle.append_unmount();
    let lifecycle_path = fixture.operation_path.join("00000004.json");
    publish_test_bytes(&lifecycle_path, b"durable-v2-record-placeholder");
    persist_unmount(&mut fixture, &issued, lifecycle.epochs('7'));
    assert!(!fixture.store.poisoned());
}

#[test]
fn a_second_issue_extends_both_rosters_without_losing_the_first_capsule() {
    let mut lifecycle = LifecycleFixture::new();
    let mut fixture = StoreFixture::new(&lifecycle.before_issue);
    let unmount = lifecycle.append_unmount();
    persist_unmount(&mut fixture, &unmount, lifecycle.epochs('7'));
    let eject = lifecycle.append_eject();
    {
        let retained = fixture
            .store
            .persist(
                &eject,
                eject_command(),
                lifecycle.epochs('8'),
                Some(digest('d')),
            )
            .expect("persist eject issue");
        assert_eq!(retained.effect_id(), 2);
        retained.revalidate().expect("two-file replay");
    }
    drop(fixture.store);
    let reopened = DurableEffectIssueStoreV3::open_existing_required(
        &fixture.operation,
        &eject,
        unsafe { libc::geteuid() },
        unsafe { libc::getegid() },
    )
    .expect("two-way exact replay");
    assert_eq!(
        reopened.replayed_issue(1).expect("unmount").effect_kind,
        EffectKindV3::Unmount
    );
    assert_eq!(
        reopened.replayed_issue(2).expect("eject").effect_kind,
        EffectKindV3::Eject
    );
}

#[test]
fn no_issue_can_be_persisted_without_a_prior_collector_observation() {
    let boot = current_boot_session_uuid().expect("boot UUID");
    let mut journal = DisposableLifecycleJournalV2::new(NONCE).expect("journal");
    let mut records = Vec::new();
    append(&mut journal, &mut records, prepared(&boot));
    let before = VerifiedLifecycleIssueRosterV3::replay(&records).expect("prepared replay");
    let mut fixture = StoreFixture::new(&before);
    append(
        &mut journal,
        &mut records,
        DisposableLifecycleEventV2::CreateIssuedOrUncertain { effect_id: 1 },
    );
    let issued = VerifiedLifecycleIssueRosterV3::replay(&records).expect("issued replay");
    let epochs = EffectEpochEvidenceV3::bind_current_boot(
        &boot,
        PROCESS_NONCE,
        &digest('7'),
        RUNNER_NONCE,
        &digest('8'),
    )
    .expect("epochs");
    assert!(
        fixture
            .store
            .persist(
                &issued,
                ExactDisposableCommandV3::CreateImage {
                    prepared_image_sha256: digest('a'),
                    size_bytes: 4096,
                },
                epochs,
                None,
            )
            .is_err()
    );
    assert!(
        fixture
            .issue_root()
            .read_dir()
            .expect("issue root")
            .next()
            .is_none()
    );
}

#[test]
fn typed_command_kind_and_epoch_aliases_fail_before_publication() {
    let mut lifecycle = LifecycleFixture::new();
    let mut fixture = StoreFixture::new(&lifecycle.before_issue);
    let issued = lifecycle.append_unmount();
    assert!(
        fixture
            .store
            .persist(&issued, eject_command(), lifecycle.epochs('7'), None)
            .is_err()
    );
    assert!(!fixture.store.poisoned());
    assert!(
        EffectEpochEvidenceV3::bind_current_boot(
            &lifecycle.boot,
            PROCESS_NONCE,
            &digest('7'),
            PROCESS_NONCE,
            &digest('8'),
        )
        .is_err()
    );
    assert!(
        EffectEpochEvidenceV3::bind_current_boot(
            "aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa",
            PROCESS_NONCE,
            &digest('7'),
            RUNNER_NONCE,
            &digest('8'),
        )
        .is_err()
    );
}

#[test]
fn every_publish_cutpoint_returns_no_capsule_and_poisoned_state() {
    for cutpoint in [
        PublishCutpointV3::TemporaryCreated,
        PublishCutpointV3::BytesWritten,
        PublishCutpointV3::FileSynced,
        PublishCutpointV3::Renamed,
        PublishCutpointV3::DirectorySynced,
        PublishCutpointV3::FinalReopened,
        PublishCutpointV3::FinalReplayed,
    ] {
        let mut lifecycle = LifecycleFixture::new();
        let mut fixture = StoreFixture::new(&lifecycle.before_issue);
        let issued = lifecycle.append_unmount();
        let failed = fixture
            .store
            .persist_with_hook(
                &issued,
                unmount_command(),
                lifecycle.epochs('7'),
                Some(digest('c')),
                |seen| {
                    if seen == cutpoint {
                        Err(io::Error::other("injected cutpoint failure"))
                    } else {
                        Ok(())
                    }
                },
            )
            .is_err();
        assert!(failed, "cutpoint {cutpoint:?} returned a capsule");
        assert!(fixture.store.poisoned(), "cutpoint {cutpoint:?}");
        let issue_root = fixture.issue_root();
        drop(fixture.store);
        let reopened = DurableEffectIssueStoreV3::open_existing_required(
            &fixture.operation,
            &issued,
            unsafe { libc::geteuid() },
            unsafe { libc::getegid() },
        );
        if matches!(
            cutpoint,
            PublishCutpointV3::Renamed
                | PublishCutpointV3::DirectorySynced
                | PublishCutpointV3::FinalReopened
                | PublishCutpointV3::FinalReplayed
        ) {
            let replayed = reopened.expect("surviving final issue replays as uncertain evidence");
            assert!(replayed.replayed_issue(1).is_some());
        } else {
            assert!(
                reopened.is_err(),
                "pre-rename residue must block exact replay at {cutpoint:?}"
            );
            assert!(
                issue_root.read_dir().expect("issue root").next().is_some(),
                "cutpoint must leave explicit issued-or-uncertain residue"
            );
        }
    }
}

fn persisted_disk() -> (TempDir, File, PathBuf, VerifiedLifecycleIssueRosterV3) {
    let mut lifecycle = LifecycleFixture::new();
    let mut fixture = StoreFixture::new(&lifecycle.before_issue);
    let issued = lifecycle.append_unmount();
    persist_unmount(&mut fixture, &issued, lifecycle.epochs('7'));
    let StoreFixture {
        _temporary,
        operation,
        operation_path,
        store,
    } = fixture;
    drop(store);
    (_temporary, operation, operation_path, issued)
}

fn reopen(
    operation: &File,
    lifecycle: &VerifiedLifecycleIssueRosterV3,
) -> Result<DurableEffectIssueStoreV3, DurableEffectIssueStoreErrorV3> {
    DurableEffectIssueStoreV3::open_existing_required(
        operation,
        lifecycle,
        unsafe { libc::geteuid() },
        unsafe { libc::getegid() },
    )
}

fn only_path(root: &Path) -> PathBuf {
    let entries = std::fs::read_dir(root)
        .expect("read issue root")
        .map(|entry| entry.expect("entry").path())
        .collect::<Vec<_>>();
    assert_eq!(entries.len(), 1);
    entries[0].clone()
}

fn publish_test_bytes(path: &Path, bytes: &[u8]) {
    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .mode(0o400)
        .open(path)
        .expect("publish mutated issue");
    file.write_all(bytes).expect("write mutated issue");
    file.sync_all().expect("sync mutated issue");
    File::open(path.parent().expect("issue root"))
        .expect("open issue root")
        .sync_all()
        .expect("sync issue root");
}

#[derive(Clone, Copy, Debug)]
enum RecordMutation {
    Authority,
    Boot,
    CommandBytes,
    CommandDigest,
    CommandValue,
    Effect,
    Kind,
    LifecycleDigest,
    LifecycleSequence,
    Operation,
    PriorCollectorDigest,
    PriorCollectorReceipt,
    PriorCollectorSequence,
    Purpose,
    Tip,
    UniqueBinding,
    ProcessEpoch,
    RunnerEpoch,
    Schema,
}

fn mutate(record: &mut IssuedEffectRecordV3, mutation: RecordMutation) {
    match mutation {
        RecordMutation::Authority => record.authority.privileged_effect_authority = true,
        RecordMutation::Boot => {
            record.boot_session_uuid = "aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa".to_string()
        }
        RecordMutation::CommandBytes => record.command_canonical_json = "{}".to_string(),
        RecordMutation::CommandDigest => record.command_sha256 = digest('0'),
        RecordMutation::CommandValue => {
            record.command = ExactDisposableCommandV3::EjectImage {
                disk_image_group_sha256: digest('b'),
            }
        }
        RecordMutation::Effect => record.effect_id = 2,
        RecordMutation::Kind => record.effect_kind = EffectKindV3::Attach,
        RecordMutation::LifecycleDigest => record.lifecycle_issue_record_sha256 = digest('0'),
        RecordMutation::LifecycleSequence => record.lifecycle_issue_sequence += 1,
        RecordMutation::Operation => record.operation_nonce = digest('0'),
        RecordMutation::PriorCollectorDigest => {
            record.prior_collector_lifecycle_record_sha256 = digest('0')
        }
        RecordMutation::PriorCollectorReceipt => {
            record.prior_collector_receipt_sha256 = digest('0')
        }
        RecordMutation::PriorCollectorSequence => record.prior_collector_lifecycle_sequence = 1,
        RecordMutation::Purpose => record.purpose = EffectPurposeV3::ForwardFlow,
        RecordMutation::Tip => record.lifecycle_tip_before_sha256 = digest('0'),
        RecordMutation::UniqueBinding => record.unique_binding_sha256 = Some("BAD".to_string()),
        RecordMutation::ProcessEpoch => {
            record.process_epoch_nonce = record.runner_epoch_nonce.clone()
        }
        RecordMutation::RunnerEpoch => {
            record.runner_epoch_sha256 = record.process_epoch_sha256.clone()
        }
        RecordMutation::Schema => record.schema = "wrong".to_string(),
    }
}

#[test]
fn every_bound_record_dimension_rejects_a_canonical_rewrite() {
    for mutation in [
        RecordMutation::Authority,
        RecordMutation::Boot,
        RecordMutation::CommandBytes,
        RecordMutation::CommandDigest,
        RecordMutation::CommandValue,
        RecordMutation::Effect,
        RecordMutation::Kind,
        RecordMutation::LifecycleDigest,
        RecordMutation::LifecycleSequence,
        RecordMutation::Operation,
        RecordMutation::PriorCollectorDigest,
        RecordMutation::PriorCollectorReceipt,
        RecordMutation::PriorCollectorSequence,
        RecordMutation::Purpose,
        RecordMutation::Tip,
        RecordMutation::UniqueBinding,
        RecordMutation::ProcessEpoch,
        RecordMutation::RunnerEpoch,
        RecordMutation::Schema,
    ] {
        let (_temporary, operation, operation_path, lifecycle) = persisted_disk();
        let root = operation_path.join(ISSUE_DIRECTORY_NAME_V3);
        let original = only_path(&root);
        let bytes = std::fs::read(&original).expect("read original issue");
        let mut record: IssuedEffectRecordV3 =
            serde_json::from_slice(&bytes).expect("decode original issue");
        mutate(&mut record, mutation);
        let bytes = canonical_json(&record).expect("canonical mutated issue");
        let replacement = root.join(issue_name(record.effect_id, &sha256(&bytes)));
        std::fs::remove_file(&original).expect("remove original issue");
        publish_test_bytes(&replacement, &bytes);
        assert!(
            reopen(&operation, &lifecycle).is_err(),
            "canonical mutation {mutation:?} was accepted"
        );
    }
}

#[test]
fn missing_orphan_duplicate_temporary_and_unknown_rosters_are_blocking() {
    // Missing V3 issue for an already durable V2 issued record.
    let mut lifecycle = LifecycleFixture::new();
    let fixture = StoreFixture::new(&lifecycle.before_issue);
    let issued = lifecycle.append_unmount();
    assert!(reopen(&fixture.operation, &issued).is_err());

    // Orphan V3 issue when the exact lifecycle prefix has no issued record.
    let (temporary, operation, operation_path, issued) = persisted_disk();
    let boot = current_boot_session_uuid().expect("boot");
    let mut journal = DisposableLifecycleJournalV2::new(NONCE).expect("journal");
    let mut records = Vec::new();
    append(&mut journal, &mut records, prepared(&boot));
    let no_issue = VerifiedLifecycleIssueRosterV3::replay(&records).expect("no issue roster");
    assert!(reopen(&operation, &no_issue).is_err());

    let root = operation_path.join(ISSUE_DIRECTORY_NAME_V3);
    let original = only_path(&root);
    let bytes = std::fs::read(&original).expect("read issue");
    let duplicate = root.join(issue_name(2, &sha256(&bytes)));
    publish_test_bytes(&duplicate, &bytes);
    assert!(reopen(&operation, &issued).is_err());
    std::fs::remove_file(&duplicate).expect("remove duplicate");

    publish_test_bytes(&root.join("unknown.json"), b"{}");
    assert!(reopen(&operation, &issued).is_err());
    std::fs::remove_file(root.join("unknown.json")).expect("remove unknown");

    publish_test_bytes(
        &root.join(".incoming-effect-00000000000000000001-deadbeef.json"),
        b"{}",
    );
    assert!(reopen(&operation, &issued).is_err());
    drop(temporary);
}

#[test]
fn filename_digest_and_noncanonical_json_mutation_are_blocking() {
    let (_temporary, operation, operation_path, lifecycle) = persisted_disk();
    let path = only_path(&operation_path.join(ISSUE_DIRECTORY_NAME_V3));
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600))
        .expect("make issue writable");
    let mut file = OpenOptions::new()
        .append(true)
        .open(&path)
        .expect("open issue");
    file.write_all(b" ").expect("append whitespace");
    file.sync_all().expect("sync mutation");
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o400))
        .expect("restore issue mode");
    assert!(reopen(&operation, &lifecycle).is_err());
}

#[test]
fn permissions_xattrs_aliases_and_inode_replacement_are_blocking() {
    // File permission drift.
    let (_temporary, operation, operation_path, lifecycle) = persisted_disk();
    let root = operation_path.join(ISSUE_DIRECTORY_NAME_V3);
    let path = only_path(&root);
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600))
        .expect("change issue mode");
    assert!(reopen(&operation, &lifecycle).is_err());

    // Directory permission drift.
    let (_temporary, operation, operation_path, lifecycle) = persisted_disk();
    let root = operation_path.join(ISSUE_DIRECTORY_NAME_V3);
    std::fs::set_permissions(&root, std::fs::Permissions::from_mode(0o755))
        .expect("change issue root mode");
    assert!(reopen(&operation, &lifecycle).is_err());

    // Extended metadata on a final issue.
    let (_temporary, operation, operation_path, lifecycle) = persisted_disk();
    let path = only_path(&operation_path.join(ISSUE_DIRECTORY_NAME_V3));
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600))
        .expect("make issue writable for xattr injection");
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(&path)
        .expect("open issue for xattr");
    let name = c"com.hepta.effect-issue-test";
    let value = [1_u8];
    assert_eq!(
        unsafe {
            libc::fsetxattr(
                file.as_raw_fd(),
                name.as_ptr(),
                value.as_ptr().cast(),
                value.len(),
                0,
                0,
            )
        },
        0
    );
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o400))
        .expect("restore issue permissions after xattr injection");
    assert!(reopen(&operation, &lifecycle).is_err());

    // Aliased issue-directory name.
    let (_temporary, operation, operation_path, lifecycle) = persisted_disk();
    std::fs::create_dir(operation_path.join("effect-issues-v3-copy"))
        .expect("create issue-root alias");
    std::fs::set_permissions(
        operation_path.join("effect-issues-v3-copy"),
        std::fs::Permissions::from_mode(0o700),
    )
    .expect("private alias");
    assert!(reopen(&operation, &lifecycle).is_err());

    // Exact named directory replaced with another inode.
    let (_temporary, operation, operation_path, lifecycle) = persisted_disk();
    let root = operation_path.join(ISSUE_DIRECTORY_NAME_V3);
    std::fs::rename(&root, operation_path.join("old-root")).expect("move retained root");
    std::fs::create_dir(&root).expect("replacement root");
    std::fs::set_permissions(&root, std::fs::Permissions::from_mode(0o700))
        .expect("private replacement root");
    assert!(reopen(&operation, &lifecycle).is_err());
}

#[test]
fn retained_capsule_detects_live_file_and_directory_replacement() {
    let mut lifecycle = LifecycleFixture::new();
    let mut fixture = StoreFixture::new(&lifecycle.before_issue);
    let issued = lifecycle.append_unmount();
    let epochs = lifecycle.epochs('7');
    let issue_root = fixture.issue_root();
    let retained = fixture
        .store
        .persist(&issued, unmount_command(), epochs, Some(digest('c')))
        .expect("retained issue");
    let path = only_path(&issue_root);
    std::fs::rename(&path, issue_root.join("old")).expect("replace retained pathname");
    std::fs::copy(issue_root.join("old"), &path).expect("copy replacement inode");
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o400))
        .expect("replacement mode");
    assert!(retained.revalidate().is_err());
}

#[test]
fn fixed_resource_budgets_reject_oversized_inputs_before_allocation_or_replay() {
    let records = vec![Vec::new(); MAX_ISSUES_V3 * 8 + 1];
    assert!(VerifiedLifecycleIssueRosterV3::replay(&records).is_err());

    let (_temporary, operation, operation_path, lifecycle) = persisted_disk();
    let root = operation_path.join(ISSUE_DIRECTORY_NAME_V3);
    let original = only_path(&root);
    std::fs::remove_file(&original).expect("remove original issue");
    let oversized = root.join(issue_name(1, &digest('a')));
    let file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .mode(0o400)
        .open(&oversized)
        .expect("oversized issue");
    file.set_len((MAX_ISSUE_BYTES_V3 + 1) as u64)
        .expect("size oversized issue");
    file.sync_all().expect("sync oversized issue");
    assert!(reopen(&operation, &lifecycle).is_err());
}

#[test]
fn new_format_directory_is_mandatory_but_historical_v2_is_not_mutated() {
    let lifecycle = LifecycleFixture::new();
    let temporary = tempfile::Builder::new()
        .prefix(".legacy-v2-operation-")
        .tempdir()
        .expect("legacy root");
    std::fs::set_permissions(temporary.path(), std::fs::Permissions::from_mode(0o700))
        .expect("private legacy root");
    let operation_path = temporary.path().join(format!("operation-{NONCE}"));
    std::fs::create_dir(&operation_path).expect("legacy operation");
    std::fs::set_permissions(&operation_path, std::fs::Permissions::from_mode(0o700))
        .expect("private legacy operation");
    let operation = File::open(&operation_path).expect("open legacy operation");
    assert!(
        DurableEffectIssueStoreV3::open_existing_required(
            &operation,
            &lifecycle.before_issue,
            unsafe { libc::geteuid() },
            unsafe { libc::getegid() },
        )
        .is_err(),
        "a caller that selects V3-required mode must fail on a missing directory"
    );
    assert!(
        !operation_path.join(ISSUE_DIRECTORY_NAME_V3).exists(),
        "verification must not retrofit or mutate a historical V2 operation"
    );
}
