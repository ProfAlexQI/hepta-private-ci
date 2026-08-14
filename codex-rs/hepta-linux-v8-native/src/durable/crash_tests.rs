use std::ffi::OsStr;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;

use super::*;
use crate::DirectoryAnchorV8;
use crate::PROCESS_FD_LIFETIME_TEST_MUTEX;
use crate::acquire_state_root_lock_v8;

const CHILD_ACTION_ENV: &str = "HEPTA_LINUX_V8_DISPOSABLE_CRASH_CHILD";
const CHILD_ROOT_ENV: &str = "HEPTA_LINUX_V8_DISPOSABLE_CRASH_ROOT";
const CHILD_EXIT_CODE: i32 = 91;
const ATTEMPT: &str = "1111111111111111111111111111111111111111111111111111111111111111";
const BOOT_ID: &str = "01234567-89ab-cdef-0123-456789abcdef";
const MACHINE: &str = "2222222222222222222222222222222222222222222222222222222222222222";
const RESTORE_PLAN: &str = "3333333333333333333333333333333333333333333333333333333333333333";
const NONCE: &str = "4444444444444444444444444444444444444444444444444444444444444444";
const STATEMENT: &str = "5555555555555555555555555555555555555555555555555555555555555555";
const SIGNATURE: &str = "6666666666666666666666666666666666666666666666666666666666666666";
const PUBLICATION_NONCE: &str = "7777777777777777777777777777777777777777777777777777777777777777";
const ZERO_SHA256: &str = "0000000000000000000000000000000000000000000000000000000000000000";

fn serialize_process_fd_lifetime() -> std::sync::MutexGuard<'static, ()> {
    PROCESS_FD_LIFETIME_TEST_MUTEX
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

struct TestStateRoot {
    path: std::path::PathBuf,
}

impl TestStateRoot {
    fn create(label: &str, with_layout: bool) -> Self {
        let path = std::env::temp_dir().join(format!(
            "hepta-linux-v8-crash-{label}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("system clock")
                .as_nanos()
        ));
        fs::create_dir(&path).expect("create disposable crash root");
        fs::set_permissions(&path, fs::Permissions::from_mode(0o700))
            .expect("set disposable crash root mode");
        if with_layout {
            fs::create_dir(path.join(NONCE_CLAIMS_DIRECTORY_V8)).expect("create nonce directory");
            fs::create_dir_all(
                path.join(ATTEMPTS_DIRECTORY_V8)
                    .join(ATTEMPT)
                    .join(JOURNAL_DIRECTORY_V8),
            )
            .expect("create journal directory");
        }
        Self { path }
    }
}

impl Drop for TestStateRoot {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

fn run_child(action: &str, root: &Path) -> std::process::ExitStatus {
    Command::new(std::env::current_exe().expect("current test executable"))
        .arg("--exact")
        .arg("durable::crash_tests::disposable_crash_child_fixture")
        .arg("--nocapture")
        .arg("--test-threads=1")
        .env(CHILD_ACTION_ENV, action)
        .env(CHILD_ROOT_ENV, root)
        .status()
        .expect("launch isolated crash child")
}

fn checkpoint_name(checkpoint: DurablePublicationCheckpointV8) -> &'static str {
    match checkpoint {
        DurablePublicationCheckpointV8::IncomingCreatedBeforeWrite => {
            "incoming-created-before-write"
        }
        DurablePublicationCheckpointV8::IncomingWrittenBeforeFileSync => {
            "incoming-written-before-file-sync"
        }
        DurablePublicationCheckpointV8::IncomingFileSyncedBeforeRename => {
            "incoming-file-synced-before-rename"
        }
        DurablePublicationCheckpointV8::RenamedBeforeDirectorySync => {
            "renamed-before-directory-sync"
        }
        DurablePublicationCheckpointV8::DirectorySyncedBeforeFinalReopen => {
            "directory-synced-before-final-reopen"
        }
        DurablePublicationCheckpointV8::FinalReopenVerified => "final-reopen-verified",
    }
}

fn checkpoint_from_name(value: &str) -> Option<DurablePublicationCheckpointV8> {
    [
        DurablePublicationCheckpointV8::IncomingCreatedBeforeWrite,
        DurablePublicationCheckpointV8::IncomingWrittenBeforeFileSync,
        DurablePublicationCheckpointV8::IncomingFileSyncedBeforeRename,
        DurablePublicationCheckpointV8::RenamedBeforeDirectorySync,
        DurablePublicationCheckpointV8::DirectorySyncedBeforeFinalReopen,
        DurablePublicationCheckpointV8::FinalReopenVerified,
    ]
    .into_iter()
    .find(|checkpoint| checkpoint_name(*checkpoint) == value)
}

fn is_pre_rename(checkpoint: DurablePublicationCheckpointV8) -> bool {
    matches!(
        checkpoint,
        DurablePublicationCheckpointV8::IncomingCreatedBeforeWrite
            | DurablePublicationCheckpointV8::IncomingWrittenBeforeFileSync
            | DurablePublicationCheckpointV8::IncomingFileSyncedBeforeRename
    )
}

fn all_checkpoints() -> [DurablePublicationCheckpointV8; 6] {
    [
        DurablePublicationCheckpointV8::IncomingCreatedBeforeWrite,
        DurablePublicationCheckpointV8::IncomingWrittenBeforeFileSync,
        DurablePublicationCheckpointV8::IncomingFileSyncedBeforeRename,
        DurablePublicationCheckpointV8::RenamedBeforeDirectorySync,
        DurablePublicationCheckpointV8::DirectorySyncedBeforeFinalReopen,
        DurablePublicationCheckpointV8::FinalReopenVerified,
    ]
}

fn publication_record() -> DurableJournalRecordV8 {
    DurableJournalRecordV8::new(
        ATTEMPT.to_string(),
        1,
        BOOT_ID.to_string(),
        1,
        ZERO_SHA256.to_string(),
        b"disposable-crash-publication".to_vec(),
    )
    .expect("construct crash publication record")
}

fn second_publication_record(first: &DurableJournalRecordV8) -> DurableJournalRecordV8 {
    DurableJournalRecordV8::new(
        ATTEMPT.to_string(),
        1,
        BOOT_ID.to_string(),
        2,
        first.record_sha256().expect("first journal digest"),
        b"disposable-crash-publication-two".to_vec(),
    )
    .expect("construct second crash publication record")
}

fn active_request() -> ActiveAttemptRequestV8 {
    ActiveAttemptRequestV8::new(
        ATTEMPT.to_string(),
        7,
        BOOT_ID.to_string(),
        MACHINE.to_string(),
        RESTORE_PLAN.to_string(),
    )
    .expect("construct active request")
}

fn nonce_claim() -> NonceClaimRecordV8 {
    NonceClaimRecordV8 {
        attempt_identity_sha256: ATTEMPT.to_string(),
        barrier_generation: 7,
        machine_id_sha256: MACHINE.to_string(),
        namespace: "hepta-linux-v8-execution".to_string(),
        nonce: NONCE.to_string(),
        schema: NONCE_CLAIM_SCHEMA_V8.to_string(),
        signature_sha256: SIGNATURE.to_string(),
        statement_sha256: STATEMENT.to_string(),
    }
}

fn publish_fresh_active(
    anchor: &DirectoryAnchorV8,
) -> (crate::StateRootLockV8, FreshActiveAttemptPublicationV8) {
    let mut lock = acquire_state_root_lock_v8(anchor, OsStr::new("state.lock"))
        .expect("acquire child state lock");
    let active = match publish_active_attempt_durably_v8(
        anchor,
        &mut lock,
        &active_request(),
        PUBLICATION_NONCE,
    )
    .expect("publish child active attempt")
    {
        ActiveAttemptPublicationOutcomeV8::Fresh(active) => active,
        ActiveAttemptPublicationOutcomeV8::ExistingRequiresRecovery(_) => {
            panic!("fresh child root unexpectedly required recovery")
        }
    };
    (lock, active)
}

fn assert_active_requires_recovery(anchor: &DirectoryAnchorV8, lock: &mut crate::StateRootLockV8) {
    let outcome = publish_active_attempt_durably_v8(
        anchor,
        lock,
        &active_request(),
        "8888888888888888888888888888888888888888888888888888888888888888",
    )
    .expect("read committed active attempt after crash");
    assert!(matches!(
        outcome,
        ActiveAttemptPublicationOutcomeV8::ExistingRequiresRecovery(_)
    ));
}

#[test]
fn disposable_crash_child_fixture() {
    let Ok(action) = std::env::var(CHILD_ACTION_ENV) else {
        return;
    };
    let root = std::env::var_os(CHILD_ROOT_ENV).expect("crash child root");
    let anchor = DirectoryAnchorV8::open(Path::new(&root)).expect("open crash child root");

    if action == "full-attempt" {
        let (mut lock, active) = publish_fresh_active(&anchor);
        assert!(matches!(
            claim_nonce_durably_v8(&anchor, &mut lock, &active, &nonce_claim())
                .expect("publish child nonce claim"),
            DurableNonceClaimOutcomeV8::FreshPublication(_)
        ));
        let record = publication_record();
        append_journal_record_durably_v8(&anchor, &mut lock, &active, &record, PUBLICATION_NONCE)
            .expect("append child journal record");
        // SAFETY: this process is the isolated test fixture. `_exit` models an
        // abrupt daemon death without running Rust destructors or releasing
        // the lock through its normal typestate path.
        unsafe { libc::_exit(CHILD_EXIT_CODE) }
    }

    if let Some(suffix) = action.strip_prefix("nonce-") {
        let desired = checkpoint_from_name(suffix).expect("known nonce checkpoint");
        let (mut lock, active) = publish_fresh_active(&anchor);
        claim_nonce_durably_observed_v8(&anchor, &mut lock, &active, &nonce_claim(), |observed| {
            if observed == desired {
                // SAFETY: isolated disposable crash child.
                unsafe { libc::_exit(CHILD_EXIT_CODE) }
            }
        })
        .expect("nonce publication reached crash checkpoint");
        panic!("nonce publication returned without hitting checkpoint");
    }

    if let Some(suffix) = action.strip_prefix("journal-two-") {
        let desired = checkpoint_from_name(suffix).expect("known journal checkpoint");
        let (mut lock, active) = publish_fresh_active(&anchor);
        let first = publication_record();
        append_journal_record_durably_v8(&anchor, &mut lock, &active, &first, PUBLICATION_NONCE)
            .expect("append first child journal record");
        let second = second_publication_record(&first);
        append_journal_record_durably_observed_v8(
            &anchor,
            &mut lock,
            &active,
            &second,
            "9999999999999999999999999999999999999999999999999999999999999999",
            |observed| {
                if observed == desired {
                    // SAFETY: isolated disposable crash child.
                    unsafe { libc::_exit(CHILD_EXIT_CODE) }
                }
            },
        )
        .expect("second journal publication reached crash checkpoint");
        panic!("second journal publication returned without hitting checkpoint");
    }

    if let Some(suffix) = action.strip_prefix("active-") {
        let desired = checkpoint_from_name(suffix)
            .filter(|checkpoint| is_pre_rename(*checkpoint))
            .expect("known active pre-rename checkpoint");
        let mut lock = acquire_state_root_lock_v8(&anchor, OsStr::new("state.lock"))
            .expect("acquire active crash child lock");
        publish_active_attempt_durably_observed_v8(
            &anchor,
            &mut lock,
            &active_request(),
            PUBLICATION_NONCE,
            |observed| {
                if observed == desired {
                    // SAFETY: isolated disposable crash child.
                    unsafe { libc::_exit(CHILD_EXIT_CODE) }
                }
            },
        )
        .expect("active publication reached crash checkpoint");
        panic!("active publication returned without hitting checkpoint");
    }

    let desired = checkpoint_from_name(&action).expect("known publication checkpoint");
    let record = publication_record();
    publish_record_noreplace_observed_v8(
        &anchor,
        "00000000000000000001.record",
        PUBLICATION_NONCE,
        &record.canonical_bytes().expect("canonical crash record"),
        |observed| {
            if observed == desired {
                // SAFETY: this is the isolated child process described above.
                unsafe { libc::_exit(CHILD_EXIT_CODE) }
            }
        },
    )
    .expect("publication reached requested crash checkpoint");
    panic!("publication returned without hitting requested crash checkpoint");
}

#[test]
fn pre_rename_active_attempt_crash_can_never_reissue_fresh_authority() {
    let _process_guard = serialize_process_fd_lifetime();
    for checkpoint in [
        DurablePublicationCheckpointV8::IncomingCreatedBeforeWrite,
        DurablePublicationCheckpointV8::IncomingWrittenBeforeFileSync,
        DurablePublicationCheckpointV8::IncomingFileSyncedBeforeRename,
    ] {
        let root = TestStateRoot::create("active-pre-rename", false);
        let action = format!("active-{}", checkpoint_name(checkpoint));
        let status = run_child(&action, &root.path);
        assert_eq!(status.code(), Some(CHILD_EXIT_CODE));

        let anchor = DirectoryAnchorV8::open(&root.path).expect("reopen active crash root");
        let mut lock = acquire_state_root_lock_v8(&anchor, OsStr::new("state.lock"))
            .expect("acquire lock after active crash");
        let outcome = publish_active_attempt_durably_v8(
            &anchor,
            &mut lock,
            &active_request(),
            "8888888888888888888888888888888888888888888888888888888888888888",
        );
        assert!(outcome.is_err());
        assert!(!root.path.join(ACTIVE_ATTEMPT_LEAF_V8).exists());
    }
}

#[test]
fn nonce_crash_preserves_evidence_but_never_restores_fresh_active_authority() {
    let _process_guard = serialize_process_fd_lifetime();
    let claim = nonce_claim();
    let canonical = claim.canonical_bytes().unwrap();
    let final_leaf = format!("{NONCE}.claim");
    let incoming = incoming_name_v8(&final_leaf, NONCE).unwrap();

    for checkpoint in all_checkpoints() {
        let root = TestStateRoot::create("nonce-checkpoint", true);
        let status = run_child(
            &format!("nonce-{}", checkpoint_name(checkpoint)),
            &root.path,
        );
        assert_eq!(status.code(), Some(CHILD_EXIT_CODE));

        let anchor = DirectoryAnchorV8::open(&root.path).unwrap();
        let mut lock = acquire_state_root_lock_v8(&anchor, OsStr::new("state.lock")).unwrap();
        assert_active_requires_recovery(&anchor, &mut lock);
        let claims = anchor
            .open_directory_beneath(Path::new(NONCE_CLAIMS_DIRECTORY_V8))
            .unwrap();
        let names = claims.list_leaf_names_bounded(4).unwrap();
        if is_pre_rename(checkpoint) {
            assert!(names.iter().any(|name| name == OsStr::new(&incoming)));
            assert!(
                !root
                    .path
                    .join(NONCE_CLAIMS_DIRECTORY_V8)
                    .join(&final_leaf)
                    .exists()
            );
            let observed =
                fs::read(root.path.join(NONCE_CLAIMS_DIRECTORY_V8).join(&incoming)).unwrap();
            if checkpoint == DurablePublicationCheckpointV8::IncomingCreatedBeforeWrite {
                assert!(observed.is_empty());
            } else {
                assert_eq!(observed, canonical);
            }
        } else {
            assert!(!names.iter().any(|name| name == OsStr::new(&incoming)));
            assert_eq!(
                fs::read(root.path.join(NONCE_CLAIMS_DIRECTORY_V8).join(&final_leaf)).unwrap(),
                canonical
            );
        }
    }
}

#[test]
fn second_journal_record_crash_replays_exact_prefix_or_exact_extended_chain() {
    let _process_guard = serialize_process_fd_lifetime();
    let first = publication_record();
    let second = second_publication_record(&first);

    for checkpoint in all_checkpoints() {
        let root = TestStateRoot::create("journal-two-checkpoint", true);
        let status = run_child(
            &format!("journal-two-{}", checkpoint_name(checkpoint)),
            &root.path,
        );
        assert_eq!(status.code(), Some(CHILD_EXIT_CODE));

        let anchor = DirectoryAnchorV8::open(&root.path).unwrap();
        let mut lock = acquire_state_root_lock_v8(&anchor, OsStr::new("state.lock")).unwrap();
        assert_active_requires_recovery(&anchor, &mut lock);
        let scan = scan_durable_journal_v8(&anchor, &lock, ATTEMPT).unwrap();
        if is_pre_rename(checkpoint) {
            assert_eq!(scan.record_count(), 1);
            assert!(scan.incoming_residue_detected());
            assert_eq!(scan.tip_sha256(), first.record_sha256().unwrap());
        } else {
            assert_eq!(scan.record_count(), 2);
            assert!(!scan.incoming_residue_detected());
            assert_eq!(scan.tip_sha256(), second.record_sha256().unwrap());
        }
    }
}

#[test]
fn abrupt_exit_at_each_publication_checkpoint_is_reconstructible_or_fail_closed() {
    let _process_guard = serialize_process_fd_lifetime();
    let record = publication_record();
    let canonical = record.canonical_bytes().expect("canonical record");
    let incoming =
        incoming_name_v8("00000000000000000001.record", PUBLICATION_NONCE).expect("incoming name");

    for checkpoint in all_checkpoints() {
        let root = TestStateRoot::create(checkpoint_name(checkpoint), false);
        let status = run_child(checkpoint_name(checkpoint), &root.path);
        assert_eq!(status.code(), Some(CHILD_EXIT_CODE));
        let anchor = DirectoryAnchorV8::open(&root.path).expect("reopen crash root");
        let names = anchor
            .list_leaf_names_bounded(4)
            .expect("list crash outcome");

        match checkpoint {
            DurablePublicationCheckpointV8::IncomingCreatedBeforeWrite => {
                assert!(names.iter().any(|name| name == OsStr::new(&incoming)));
                assert_eq!(fs::metadata(root.path.join(&incoming)).unwrap().len(), 0);
                assert!(!root.path.join("00000000000000000001.record").exists());
                assert!(scan_journal_directory_v8(&anchor, ATTEMPT, anchor.identity()).is_err());
            }
            DurablePublicationCheckpointV8::IncomingWrittenBeforeFileSync
            | DurablePublicationCheckpointV8::IncomingFileSyncedBeforeRename => {
                assert_eq!(fs::read(root.path.join(&incoming)).unwrap(), canonical);
                assert!(!root.path.join("00000000000000000001.record").exists());
                assert!(scan_journal_directory_v8(&anchor, ATTEMPT, anchor.identity()).is_err());
            }
            DurablePublicationCheckpointV8::RenamedBeforeDirectorySync
            | DurablePublicationCheckpointV8::DirectorySyncedBeforeFinalReopen
            | DurablePublicationCheckpointV8::FinalReopenVerified => {
                assert!(!names.iter().any(|name| name == OsStr::new(&incoming)));
                assert_eq!(
                    fs::read(root.path.join("00000000000000000001.record")).unwrap(),
                    canonical
                );
                let scan = scan_journal_directory_v8(&anchor, ATTEMPT, anchor.identity()).unwrap();
                assert_eq!(scan.record_count(), 1);
                assert_eq!(scan.tip_sha256(), record.record_sha256().unwrap());
            }
        }
    }
}

#[test]
fn daemon_crash_destroys_fresh_authority_but_preserves_recovery_evidence() {
    let _process_guard = serialize_process_fd_lifetime();
    let root = TestStateRoot::create("full-attempt", true);
    let status = run_child("full-attempt", &root.path);
    assert_eq!(status.code(), Some(CHILD_EXIT_CODE));

    let anchor = DirectoryAnchorV8::open(&root.path).expect("reopen crashed state root");
    let mut lock = acquire_state_root_lock_v8(&anchor, OsStr::new("state.lock"))
        .expect("reacquire state lock after crash");
    let outcome = publish_active_attempt_durably_v8(
        &anchor,
        &mut lock,
        &active_request(),
        "8888888888888888888888888888888888888888888888888888888888888888",
    )
    .expect("read existing active attempt after crash");
    let existing = match outcome {
        ActiveAttemptPublicationOutcomeV8::ExistingRequiresRecovery(existing) => existing,
        ActiveAttemptPublicationOutcomeV8::Fresh(_) => {
            panic!("crashed active attempt must never yield fresh authority")
        }
    };
    assert_eq!(existing.attempt_identity_sha256(), ATTEMPT);
    assert_eq!(existing.barrier_generation(), 7);

    let claim = nonce_claim();
    let claim_file = anchor
        .open_regular_readonly_beneath(Path::new(&claim.relative_path().unwrap()))
        .expect("reopen durable nonce claim");
    assert_eq!(
        claim_file.read_all(64 * 1024).unwrap(),
        claim.canonical_bytes().unwrap()
    );

    let scan = scan_durable_journal_v8(&anchor, &lock, ATTEMPT)
        .expect("replay durable journal after crash");
    assert_eq!(scan.record_count(), 1);
    assert_eq!(scan.last_boot_epoch(), 1);
    assert_eq!(scan.last_boot_id(), BOOT_ID);
    assert!(!scan.incoming_residue_detected());
    assert_eq!(
        scan.tip_sha256(),
        publication_record().record_sha256().unwrap()
    );
}
