use super::*;
use crate::mac_disposable_lifecycle::DisposableAuthorityV2;
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
type RetainedLifecycleAppendForCompileAssertions = RetainedLifecycleRecordAppendV3;
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
assert_not_impl!(RetainedLifecycleAppendForCompileAssertions, Clone);
assert_not_impl!(RetainedLifecycleAppendForCompileAssertions, Send);
assert_not_impl!(RetainedLifecycleAppendForCompileAssertions, Sync);
assert_not_impl!(
    RetainedLifecycleAppendForCompileAssertions,
    serde::Serialize
);
assert_not_impl!(
    RetainedLifecycleAppendForCompileAssertions,
    std::os::fd::AsRawFd
);
assert_not_impl!(
    RetainedLifecycleAppendForCompileAssertions,
    From<std::fs::File>
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
    assert!(store.census.prepare_store_creation().is_err());

    let record_digest = store
        .append(prepared())
        .expect("append while census and flock remain retained");
    assert_eq!(record_digest.len(), 64);
    assert!(!store.poisoned());
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
        .expect("production exact-census replay");
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
        .expect("open exact selected blocker");

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
