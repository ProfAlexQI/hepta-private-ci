use super::*;
use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;
use std::sync::Arc;
use std::sync::Barrier;
use std::sync::mpsc;
use std::thread;
use std::time::Instant;
use tempfile::TempDir;

const NONCE: &str = "1111111111111111111111111111111111111111111111111111111111111111";
const TIP: &str = "2222222222222222222222222222222222222222222222222222222222222222";

#[test]
fn inert_child_entry() {
    if std::env::var_os("HEPTA_INERT_RUNNER_COMMAND_FD_V3").is_some() {
        run_inert_child_from_environment().expect("inert child protocol");
    }
}

#[test]
fn slow_inert_child_entry() {
    if std::env::var_os("HEPTA_INERT_RUNNER_COMMAND_FD_V3").is_some() {
        run_inert_child_with_delay(Duration::from_secs(5)).expect("slow inert child protocol");
    }
}

#[test]
fn partial_receipt_child_entry() {
    if std::env::var_os("HEPTA_INERT_RUNNER_COMMAND_FD_V3").is_some() {
        run_inert_child_with_behavior(
            Duration::ZERO,
            InertChildResponseV3::PartialReceiptThenStall,
        )
        .expect("partial receipt helper");
    }
}

#[test]
fn drop_receipt_child_entry() {
    if std::env::var_os("HEPTA_INERT_RUNNER_COMMAND_FD_V3").is_some() {
        run_inert_child_with_behavior(Duration::ZERO, InertChildResponseV3::DropWithoutReceipt)
            .expect("drop receipt helper");
    }
}

#[test]
fn stalled_command_reader_child_entry() {
    if std::env::var_os("HEPTA_INERT_RUNNER_COMMAND_FD_V3").is_some() {
        run_inert_child_with_behavior(Duration::ZERO, InertChildResponseV3::StallBeforeCommand)
            .expect("stalled command reader helper");
    }
}

#[test]
fn partial_hello_child_entry() {
    if std::env::var_os("HEPTA_INERT_RUNNER_COMMAND_FD_V3").is_none() {
        return;
    }
    let command = inherited_fd("HEPTA_INERT_RUNNER_COMMAND_FD_V3").expect("command FD");
    let response = inherited_fd("HEPTA_INERT_RUNNER_RESPONSE_FD_V3").expect("response FD");
    let death = inherited_fd("HEPTA_INERT_RUNNER_DEATH_FD_V3").expect("death FD");
    let lease = inherited_fd("HEPTA_INERT_RUNNER_LEASE_FD_V3").expect("lease FD");
    let _command = unsafe { File::from_raw_fd(command) };
    let response = unsafe { File::from_raw_fd(response) };
    let _death = unsafe { File::from_raw_fd(death) };
    let _lease = unsafe { File::from_raw_fd(lease) };
    let prefix = [0u8; 4];
    assert_eq!(
        unsafe { libc::write(response.as_raw_fd(), prefix.as_ptr().cast(), prefix.len()) },
        prefix.len() as isize
    );
    thread::sleep(Duration::from_secs(5));
}

#[test]
fn foreign_exec_sleeper_entry() {
    if std::env::var_os("HEPTA_FOREIGN_EXEC_SLEEPER_V3").is_some() {
        thread::sleep(Duration::from_secs(3));
    }
}

fn helper_arguments(test_name: &'static str) -> [&'static str; 3] {
    ["--exact", test_name, "--nocapture"]
}

fn helper_program() -> PathBuf {
    std::env::current_exe().expect("current test executable")
}

fn test_lease() -> (TempDir, RetainedControlLeaseV3) {
    let directory = TempDir::new().expect("temporary lease directory");
    let path = directory.path().join("control.lock");
    let descriptor = OpenOptions::new()
        .create_new(true)
        .read(true)
        .write(true)
        .mode(0o600)
        .open(path)
        .expect("control lease file");
    let rc = unsafe { libc::flock(descriptor.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) };
    assert_eq!(rc, 0, "test lease must be exclusively locked");
    (directory, RetainedControlLeaseV3::for_test(descriptor))
}

fn spawn_runner(epoch: &FreshProcessEpochV3) -> (TempDir, LiveInertRunnerV3) {
    spawn_named_runner(
        epoch,
        "mac_inert_one_shot_runner::tests::inert_child_entry",
        Duration::from_secs(5),
    )
}

fn spawn_named_runner(
    epoch: &FreshProcessEpochV3,
    test_name: &'static str,
    startup_timeout: Duration,
) -> (TempDir, LiveInertRunnerV3) {
    let (directory, lease) = test_lease();
    let arguments = helper_arguments(test_name);
    let runner = LiveInertRunnerV3::spawn_program(
        epoch,
        &lease,
        &helper_program(),
        &arguments,
        startup_timeout,
    )
    .expect("spawn inert runner");
    (directory, runner)
}

fn spawn_slow_runner(epoch: &FreshProcessEpochV3) -> (TempDir, LiveInertRunnerV3) {
    spawn_named_runner(
        epoch,
        "mac_inert_one_shot_runner::tests::slow_inert_child_entry",
        Duration::from_secs(5),
    )
}

fn issue_once(
    runner: &mut LiveInertRunnerV3,
    epoch: &FreshProcessEpochV3,
) -> InertDispatchReceiptV3 {
    runner
        .issue_fresh_with(
            epoch,
            NONCE,
            7,
            Some(TIP.to_string()),
            b"inert:echo:qualification",
            Duration::from_secs(5),
            |record, bytes| {
                assert!(!record.authority.any());
                assert_eq!(
                    sha256(bytes),
                    digest_canonical(record).expect("record digest")
                );
                Ok(DurableIssuePersistenceReceiptV3::for_test(record, bytes))
            },
        )
        .expect("one inert command")
}

fn persist_for_test(
    record: &IssuedEffectRecordV3,
    bytes: &[u8],
) -> io::Result<DurableIssuePersistenceReceiptV3> {
    Ok(DurableIssuePersistenceReceiptV3::for_test(record, bytes))
}

#[test]
fn v3_record_and_envelope_bind_every_issue_dimension() {
    let record = IssuedEffectRecordV3 {
        authority: DisposableAuthorityV2::none(),
        command_sha256: sha256(b"command"),
        effect_id: 41,
        issue_context: EffectIssueContextV3::FreshProcess,
        journal_tip_before_sha256: Some(TIP.to_string()),
        operation_nonce: NONCE.to_string(),
        previous_record_sha256: Some(TIP.to_string()),
        process_epoch_sha256: "33".repeat(32),
        purpose: EffectPurposeV3::ForwardFlow,
        runner_epoch_sha256: "44".repeat(32),
        schema: ISSUE_SCHEMA_V3.to_string(),
        schema_version: 3,
    };
    record.validate().expect("valid issue record");
    let envelope = RunnerCommandEnvelopeV3 {
        command_sha256: record.command_sha256.clone(),
        effect_id: record.effect_id,
        issued_record_sha256: digest_canonical(&record).expect("record digest"),
        journal_tip_before_sha256: record.journal_tip_before_sha256.clone(),
        operation_nonce: record.operation_nonce.clone(),
        previous_record_sha256: record.previous_record_sha256.clone(),
        process_epoch_sha256: record.process_epoch_sha256.clone(),
        purpose: record.purpose,
        runner_epoch_sha256: record.runner_epoch_sha256.clone(),
        schema: ENVELOPE_SCHEMA_V3.to_string(),
        schema_version: 3,
    };
    envelope
        .validate_against(&record, b"command")
        .expect("exact envelope");

    let bytes = canonical_bytes(&record).expect("canonical record");
    let value: serde_json::Value = serde_json::from_slice(&bytes).expect("record JSON");
    assert_eq!(value["schema"], ISSUE_SCHEMA_V3);
    assert_eq!(value["schema_version"], 3);
    assert_eq!(value["authority"]["privileged_effect_authority"], false);
}

#[test]
fn command_digest_or_journal_tip_drift_fails_closed() {
    let mut record = IssuedEffectRecordV3 {
        authority: DisposableAuthorityV2::none(),
        command_sha256: sha256(b"one"),
        effect_id: 9,
        issue_context: EffectIssueContextV3::FreshProcess,
        journal_tip_before_sha256: Some(TIP.to_string()),
        operation_nonce: NONCE.to_string(),
        previous_record_sha256: Some(TIP.to_string()),
        process_epoch_sha256: "33".repeat(32),
        purpose: EffectPurposeV3::ForwardFlow,
        runner_epoch_sha256: "44".repeat(32),
        schema: ISSUE_SCHEMA_V3.to_string(),
        schema_version: 3,
    };
    let mut envelope = RunnerCommandEnvelopeV3 {
        command_sha256: record.command_sha256.clone(),
        effect_id: record.effect_id,
        issued_record_sha256: digest_canonical(&record).expect("record digest"),
        journal_tip_before_sha256: record.journal_tip_before_sha256.clone(),
        operation_nonce: record.operation_nonce.clone(),
        previous_record_sha256: record.previous_record_sha256.clone(),
        process_epoch_sha256: record.process_epoch_sha256.clone(),
        purpose: record.purpose,
        runner_epoch_sha256: record.runner_epoch_sha256.clone(),
        schema: ENVELOPE_SCHEMA_V3.to_string(),
        schema_version: 3,
    };
    assert!(envelope.validate_against(&record, b"two").is_err());

    record.journal_tip_before_sha256 = Some("55".repeat(32));
    assert!(record.validate().is_err());

    record.journal_tip_before_sha256 = record.previous_record_sha256.clone();
    envelope.journal_tip_before_sha256 = Some("66".repeat(32));
    assert!(envelope.validate_against(&record, b"one").is_err());
}

#[test]
fn restart_record_requires_a_death_proof_digest_and_matching_purpose() {
    let base = IssuedEffectRecordV3 {
        authority: DisposableAuthorityV2::none(),
        command_sha256: sha256(b"reconcile"),
        effect_id: 4,
        issue_context: EffectIssueContextV3::FreshProcess,
        journal_tip_before_sha256: Some(TIP.to_string()),
        operation_nonce: NONCE.to_string(),
        previous_record_sha256: Some(TIP.to_string()),
        process_epoch_sha256: "33".repeat(32),
        purpose: EffectPurposeV3::RestartReconciliation,
        runner_epoch_sha256: "44".repeat(32),
        schema: ISSUE_SCHEMA_V3.to_string(),
        schema_version: 3,
    };
    assert!(base.validate().is_err());
    let mut valid = base;
    valid.issue_context = EffectIssueContextV3::RestartReconciliation {
        prior_runner_death_proof_sha256: "55".repeat(32),
    };
    valid.validate().expect("death proof bound reconciliation");
}

#[test]
fn fresh_process_epoch_is_kernel_bound_and_fork_inheritance_hits_pid_gate() {
    let epoch = FreshProcessEpochV3::establish().expect("fresh process epoch");
    epoch.validate_current().expect("same process epoch");
    let owner_pid = epoch.binding.pid;
    let child = unsafe { libc::fork() };
    assert!(child >= 0, "fork for the PID-gate test");
    if child == 0 {
        let inherited_is_rejected = unsafe { libc::getpid() } as u32 != owner_pid;
        unsafe { libc::_exit(if inherited_is_rejected { 0 } else { 1 }) };
    }
    let mut status = 0;
    assert_eq!(unsafe { libc::waitpid(child, &mut status, 0) }, child);
    assert!(libc::WIFEXITED(status));
    assert_eq!(libc::WEXITSTATUS(status), 0);
}

#[test]
fn parent_persists_exact_issue_before_one_inert_dispatch() {
    let epoch = FreshProcessEpochV3::establish().expect("fresh process epoch");
    let (_directory, mut runner) = spawn_runner(&epoch);
    let runner_epoch = runner.runner_epoch_sha256().to_string();
    let receipt = issue_once(&mut runner, &epoch);
    assert_eq!(receipt.dispatch_count, 1);
    assert_eq!(receipt.runner_epoch_sha256, runner_epoch);
    assert!(!receipt.authority.any());
    let proof = runner
        .prove_dead(Duration::from_secs(5))
        .expect("composite death proof");
    assert!(proof.receipt().kqueue_note_exit_observed);
    assert!(proof.receipt().death_pipe_eof_observed);
    assert!(proof.receipt().waitpid_observed);
    assert!(proof.receipt().kernel_identity_absent);
    assert_eq!(proof.receipt().operation_nonce, NONCE);
    assert_eq!(proof.receipt().effect_id, 7);
    assert_eq!(
        proof.receipt().command_sha256,
        sha256(b"inert:echo:qualification")
    );
    assert_eq!(
        proof.receipt().journal_tip_before_sha256.as_deref(),
        Some(TIP)
    );
    require_sha256(
        &proof.receipt().issued_record_sha256,
        "death proof issued record",
    )
    .expect("issued record binding");
}

#[test]
fn child_retains_the_inherited_control_flock_until_runner_exit() {
    let epoch = FreshProcessEpochV3::establish().expect("fresh process epoch");
    let (directory, mut runner) = spawn_runner(&epoch);
    let contender = OpenOptions::new()
        .read(true)
        .write(true)
        .open(directory.path().join("control.lock"))
        .expect("open lock contender");
    let rc = unsafe { libc::flock(contender.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) };
    assert_eq!(rc, -1, "child must retain the supervisor's flock");
    assert_eq!(
        io::Error::last_os_error().raw_os_error(),
        Some(libc::EWOULDBLOCK)
    );
    issue_once(&mut runner, &epoch);
    runner
        .prove_dead(Duration::from_secs(5))
        .expect("death proof after retained lease exits");
    let rc = unsafe { libc::flock(contender.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) };
    assert_eq!(rc, 0, "flock becomes available only after child exit");
}

#[test]
fn concurrent_foreign_execs_never_inherit_the_runner_lease() {
    let epoch = FreshProcessEpochV3::establish().expect("fresh process epoch");
    let barrier = Arc::new(Barrier::new(2));
    let (sender, receiver) = mpsc::channel();
    let foreign_barrier = Arc::clone(&barrier);
    let foreign_program = helper_program();
    let foreign = thread::spawn(move || {
        foreign_barrier.wait();
        let mut children = Vec::new();
        for _ in 0..24 {
            let child = Command::new(&foreign_program)
                .args([
                    "--exact",
                    "mac_inert_one_shot_runner::tests::foreign_exec_sleeper_entry",
                    "--nocapture",
                ])
                .env("HEPTA_FOREIGN_EXEC_SLEEPER_V3", "1")
                .env_remove("HEPTA_INERT_RUNNER_COMMAND_FD_V3")
                .env_remove("HEPTA_INERT_RUNNER_RESPONSE_FD_V3")
                .env_remove("HEPTA_INERT_RUNNER_DEATH_FD_V3")
                .env_remove("HEPTA_INERT_RUNNER_LEASE_FD_V3")
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .expect("spawn foreign exec helper");
            children.push(child);
        }
        sender.send(children).expect("send foreign children");
    });

    barrier.wait();
    let (directory, mut runner) = spawn_runner(&epoch);
    issue_once(&mut runner, &epoch);
    runner
        .prove_dead(Duration::from_secs(5))
        .expect("runner death proof");
    let mut foreign_children = receiver.recv().expect("receive foreign children");
    foreign.join().expect("foreign spawner");
    assert!(
        foreign_children
            .iter_mut()
            .any(|child| child.try_wait().expect("foreign child status").is_none()),
        "at least one foreign helper must still be alive during the flock check",
    );

    let contender = OpenOptions::new()
        .read(true)
        .write(true)
        .open(directory.path().join("control.lock"))
        .expect("open contender after runner exit");
    assert_eq!(
        unsafe { libc::flock(contender.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) },
        0,
        "foreign exec inherited a lease FD from the parent spawn window",
    );
    for child in &mut foreign_children {
        let _ = child.kill();
        let _ = child.wait();
    }
}

#[test]
fn partial_hello_prefix_hits_one_absolute_startup_deadline() {
    let epoch = FreshProcessEpochV3::establish().expect("fresh process epoch");
    let (directory, lease) = test_lease();
    let arguments = helper_arguments("mac_inert_one_shot_runner::tests::partial_hello_child_entry");
    let started = Instant::now();
    let error = match LiveInertRunnerV3::spawn_program(
        &epoch,
        &lease,
        &helper_program(),
        &arguments,
        Duration::from_millis(100),
    ) {
        Ok(_) => panic!("partial hello cannot stall startup"),
        Err(error) => error,
    };
    assert!(matches!(error, InertRunnerErrorV3::StartupFailed));
    assert!(started.elapsed() < Duration::from_secs(2));
    drop(lease);
    let contender = OpenOptions::new()
        .read(true)
        .write(true)
        .open(directory.path().join("control.lock"))
        .expect("open post-timeout contender");
    assert_eq!(
        unsafe { libc::flock(contender.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) },
        0,
        "startup timeout must kill and reap the partial-frame child",
    );
}

#[test]
fn persistence_failure_sends_nothing_and_poisoned_epoch_cannot_retry() {
    let epoch = FreshProcessEpochV3::establish().expect("fresh process epoch");
    let (_directory, mut runner) = spawn_runner(&epoch);
    let error = runner
        .issue_fresh_with(
            &epoch,
            NONCE,
            1,
            Some(TIP.to_string()),
            b"never-crosses-pipe",
            Duration::from_millis(100),
            |_record, _bytes| Err(io::Error::other("fsync uncertain")),
        )
        .expect_err("persistence uncertainty");
    assert!(matches!(error, InertRunnerErrorV3::PersistenceUncertain(_)));
    let second = runner
        .issue_fresh_with(
            &epoch,
            NONCE,
            2,
            Some(TIP.to_string()),
            b"retry-forbidden",
            Duration::from_millis(100),
            persist_for_test,
        )
        .expect_err("poisoned runner epoch cannot retry");
    assert!(matches!(second, InertRunnerErrorV3::Invalid(_)));
    let proof_error = runner
        .terminate_and_prove_dead(Duration::from_secs(1))
        .expect_err("uncertain persistence is not a durable reconciliation proof");
    assert!(matches!(proof_error, InertRunnerErrorV3::Invalid(_)));
}

#[test]
fn persistence_panic_is_caught_and_permanently_poisoned() {
    let epoch = FreshProcessEpochV3::establish().expect("fresh process epoch");
    let (_directory, mut runner) = spawn_runner(&epoch);
    let error = runner
        .issue_fresh_with(
            &epoch,
            NONCE,
            1,
            Some(TIP.to_string()),
            b"never-crosses-pipe-after-panic",
            Duration::from_millis(100),
            |_record, _bytes| -> io::Result<DurableIssuePersistenceReceiptV3> {
                panic!("post-write state unknown")
            },
        )
        .expect_err("persistence panic is uncertainty");
    assert!(matches!(error, InertRunnerErrorV3::PersistenceUncertain(_)));
    assert_eq!(runner.state, RunnerStateV3::IssuedOrUncertain);
}

#[test]
fn mismatched_durable_receipt_cannot_upgrade_an_issue_or_death_proof() {
    let epoch = FreshProcessEpochV3::establish().expect("fresh process epoch");
    let (_directory, mut runner) = spawn_runner(&epoch);
    let error = runner
        .issue_fresh_with(
            &epoch,
            NONCE,
            2,
            Some(TIP.to_string()),
            b"forged-durable-receipt",
            Duration::from_secs(1),
            |record, bytes| {
                let mut receipt = DurableIssuePersistenceReceiptV3::for_test(record, bytes);
                receipt.issued_record_sha256 = "aa".repeat(32);
                Ok(receipt)
            },
        )
        .expect_err("mismatched durable receipt");
    assert!(matches!(error, InertRunnerErrorV3::PersistenceUncertain(_)));
    let proof_error = runner
        .terminate_and_prove_dead(Duration::from_secs(1))
        .expect_err("mismatched durable receipt has no death proof authority");
    assert!(matches!(proof_error, InertRunnerErrorV3::Invalid(_)));
}

#[test]
fn runner_epoch_accepts_exactly_one_command() {
    let epoch = FreshProcessEpochV3::establish().expect("fresh process epoch");
    let (_directory, mut runner) = spawn_runner(&epoch);
    issue_once(&mut runner, &epoch);
    let second = runner
        .issue_fresh_with(
            &epoch,
            NONCE,
            8,
            Some(TIP.to_string()),
            b"second-command",
            Duration::from_millis(100),
            persist_for_test,
        )
        .expect_err("second command rejected");
    assert!(matches!(second, InertRunnerErrorV3::Invalid(_)));
    runner
        .prove_dead(Duration::from_secs(5))
        .expect("runner death after one command");
}

#[test]
fn process_epoch_mismatch_is_rejected_before_persistence() {
    let owner = FreshProcessEpochV3::establish().expect("owner epoch");
    let other = FreshProcessEpochV3::establish().expect("other epoch");
    let (_directory, mut runner) = spawn_runner(&owner);
    let mut persisted = false;
    let error = runner
        .issue_fresh_with(
            &other,
            NONCE,
            3,
            Some(TIP.to_string()),
            b"wrong-process-epoch",
            Duration::from_millis(100),
            |record, bytes| {
                persisted = true;
                Ok(DurableIssuePersistenceReceiptV3::for_test(record, bytes))
            },
        )
        .expect_err("wrong process epoch");
    assert!(matches!(error, InertRunnerErrorV3::Invalid(_)));
    assert!(!persisted);
}

#[test]
fn live_runner_cannot_mint_a_death_proof() {
    let epoch = FreshProcessEpochV3::establish().expect("fresh process epoch");
    let (_directory, runner) = spawn_runner(&epoch);
    let error = runner
        .prove_dead(Duration::ZERO)
        .expect_err("live runner has no death proof");
    assert!(matches!(error, InertRunnerErrorV3::Invalid(_)));
}

#[test]
fn exited_dummy_runner_without_a_durable_issue_cannot_mint_a_proof() {
    let epoch = FreshProcessEpochV3::establish().expect("fresh process epoch");
    let (_directory, runner) = spawn_runner(&epoch);
    let pid = runner.child.id() as libc::pid_t;
    assert_eq!(unsafe { libc::kill(-pid, libc::SIGKILL) }, 0);
    let error = runner
        .prove_dead(Duration::from_secs(1))
        .expect_err("dummy runner has no durable issued binding");
    assert!(matches!(error, InertRunnerErrorV3::Invalid(_)));
}

#[test]
fn same_supervisor_sequential_reconciliation_is_reachable_and_exact_bound() {
    let epoch = FreshProcessEpochV3::establish().expect("fresh process epoch");
    let (_first_directory, mut first) = spawn_runner(&epoch);
    issue_once(&mut first, &epoch);
    let proof = first
        .prove_dead(Duration::from_secs(5))
        .expect("first runner proof");
    let prior_issue = proof.receipt().issued_record_sha256.clone();
    let prior_proof_sha256 = proof.sha256().to_string();

    let (_second_directory, mut second) = spawn_runner(&epoch);
    let receipt = second
        .issue_same_supervisor_reconciliation_with(
            &epoch,
            proof,
            NONCE,
            8,
            Some(prior_issue.clone()),
            b"inert:reconcile:qualification",
            Duration::from_secs(5),
            |record, bytes| {
                assert_eq!(
                    record.issue_context,
                    EffectIssueContextV3::RestartReconciliation {
                        prior_runner_death_proof_sha256: prior_proof_sha256.clone(),
                    }
                );
                assert_eq!(
                    record.previous_record_sha256.as_deref(),
                    Some(prior_issue.as_str())
                );
                Ok(DurableIssuePersistenceReceiptV3::for_test(record, bytes))
            },
        )
        .expect("same supervisor reconciliation");
    assert_eq!(receipt.dispatch_count, 1);
    second
        .prove_dead(Duration::from_secs(5))
        .expect("second runner proof");
}

fn make_issued_proof(epoch: &FreshProcessEpochV3) -> SameSupervisorRunnerDeathProofV3 {
    let (_directory, mut runner) = spawn_runner(epoch);
    issue_once(&mut runner, epoch);
    runner
        .prove_dead(Duration::from_secs(5))
        .expect("issued runner proof")
}

#[test]
fn death_proof_transplant_across_operation_or_tip_is_rejected_before_persistence() {
    let epoch = FreshProcessEpochV3::establish().expect("fresh process epoch");
    let proof = make_issued_proof(&epoch);
    let proof_tip = proof.receipt().issued_record_sha256.clone();
    let (_directory, mut runner) = spawn_runner(&epoch);
    let mut persisted = false;
    let error = runner
        .issue_same_supervisor_reconciliation_with(
            &epoch,
            proof,
            &"aa".repeat(32),
            8,
            Some(proof_tip),
            b"wrong-operation",
            Duration::from_secs(1),
            |record, bytes| {
                persisted = true;
                Ok(DurableIssuePersistenceReceiptV3::for_test(record, bytes))
            },
        )
        .expect_err("operation transplant");
    assert!(matches!(error, InertRunnerErrorV3::Invalid(_)));
    assert!(!persisted);

    let proof = make_issued_proof(&epoch);
    let (_directory, mut runner) = spawn_runner(&epoch);
    let error = runner
        .issue_same_supervisor_reconciliation_with(
            &epoch,
            proof,
            NONCE,
            8,
            Some("bb".repeat(32)),
            b"wrong-tip",
            Duration::from_secs(1),
            persist_for_test,
        )
        .expect_err("journal-tip transplant");
    assert!(matches!(error, InertRunnerErrorV3::Invalid(_)));
}

#[test]
fn restart_death_proof_is_scoped_to_the_original_supervisor_epoch() {
    let first_epoch = FreshProcessEpochV3::establish().expect("first epoch");
    let (_first_directory, mut first_runner) = spawn_runner(&first_epoch);
    issue_once(&mut first_runner, &first_epoch);
    let proof = first_runner
        .prove_dead(Duration::from_secs(5))
        .expect("first death proof");
    require_sha256(proof.sha256(), "proof digest").expect("proof digest shape");

    let serialized = canonical_bytes(proof.receipt()).expect("death receipt JSON");
    let replayed: PriorRunnerDeathReceiptV3 =
        parse_canonical(&serialized, "death receipt").expect("replayed evidence");
    assert_eq!(replayed, *proof.receipt());

    let second_epoch = FreshProcessEpochV3::establish().expect("second epoch");
    let (_second_directory, mut second_runner) = spawn_runner(&second_epoch);
    let proof_tip = proof.receipt().issued_record_sha256.clone();
    let error = second_runner
        .issue_same_supervisor_reconciliation_with(
            &second_epoch,
            proof,
            NONCE,
            10,
            Some(proof_tip),
            b"reconciliation",
            Duration::from_millis(100),
            persist_for_test,
        )
        .expect_err("foreign supervisor proof rejected");
    assert!(matches!(error, InertRunnerErrorV3::Invalid(_)));

    // `replayed` is intentionally not accepted by the reconciliation API:
    // it requires the non-serializable same-supervisor wrapper.  A fresh
    // supervisor instead needs the sealed S1 durable bridge, not this proof.
}

#[test]
fn timeout_kills_and_reaps_the_independent_group_without_retry_authority() {
    let epoch = FreshProcessEpochV3::establish().expect("fresh process epoch");
    let (_directory, mut runner) = spawn_slow_runner(&epoch);
    let pid = runner.runner_identity.pid;
    let error = runner
        .issue_fresh_with(
            &epoch,
            NONCE,
            12,
            Some(TIP.to_string()),
            b"inert-slow-command",
            Duration::from_millis(50),
            persist_for_test,
        )
        .expect_err("bounded timeout");
    assert!(matches!(
        error,
        InertRunnerErrorV3::TimeoutIssuedOrUncertain
    ));
    assert_eq!(runner.state, RunnerStateV3::Reaped);
    assert!(kernel_process_identity(pid).is_err());
    let proof = runner
        .prove_dead(Duration::from_secs(1))
        .expect("timeout retains composite death proof");
    assert_eq!(proof.receipt().operation_nonce, NONCE);
    assert_eq!(proof.receipt().effect_id, 12);
    assert_eq!(
        proof.receipt().command_sha256,
        sha256(b"inert-slow-command")
    );
}

#[test]
fn partial_receipt_prefix_cannot_bypass_dispatch_deadline_and_retains_proof() {
    let epoch = FreshProcessEpochV3::establish().expect("fresh process epoch");
    let (_directory, mut runner) = spawn_named_runner(
        &epoch,
        "mac_inert_one_shot_runner::tests::partial_receipt_child_entry",
        Duration::from_secs(5),
    );
    let started = Instant::now();
    let error = runner
        .issue_fresh_with(
            &epoch,
            NONCE,
            20,
            Some(TIP.to_string()),
            b"partial-receipt",
            Duration::from_millis(100),
            persist_for_test,
        )
        .expect_err("partial receipt deadline");
    assert!(matches!(
        error,
        InertRunnerErrorV3::TimeoutIssuedOrUncertain
    ));
    assert!(started.elapsed() < Duration::from_secs(2));
    let proof = runner
        .prove_dead(Duration::from_secs(1))
        .expect("partial receipt retains death proof");
    assert_eq!(proof.receipt().effect_id, 20);
    assert_eq!(proof.receipt().command_sha256, sha256(b"partial-receipt"));
}

#[test]
fn blocked_command_write_uses_the_same_absolute_deadline_and_retains_proof() {
    let epoch = FreshProcessEpochV3::establish().expect("fresh process epoch");
    let (_directory, mut runner) = spawn_named_runner(
        &epoch,
        "mac_inert_one_shot_runner::tests::stalled_command_reader_child_entry",
        Duration::from_secs(5),
    );
    let command = vec![0xff; 128 * 1024];
    let started = Instant::now();
    let error = runner
        .issue_fresh_with(
            &epoch,
            NONCE,
            22,
            Some(TIP.to_string()),
            &command,
            Duration::from_millis(100),
            persist_for_test,
        )
        .expect_err("blocked command write deadline");
    assert!(matches!(
        error,
        InertRunnerErrorV3::TimeoutIssuedOrUncertain
    ));
    assert!(started.elapsed() < Duration::from_secs(2));
    let proof = runner
        .prove_dead(Duration::from_secs(1))
        .expect("blocked write retains death proof");
    assert_eq!(proof.receipt().effect_id, 22);
    assert_eq!(proof.receipt().command_sha256, sha256(&command));
}

#[test]
fn channel_loss_kills_reaps_and_retains_exact_death_proof() {
    let epoch = FreshProcessEpochV3::establish().expect("fresh process epoch");
    let (_directory, mut runner) = spawn_named_runner(
        &epoch,
        "mac_inert_one_shot_runner::tests::drop_receipt_child_entry",
        Duration::from_secs(5),
    );
    let error = runner
        .issue_fresh_with(
            &epoch,
            NONCE,
            21,
            Some(TIP.to_string()),
            b"lost-channel",
            Duration::from_secs(1),
            persist_for_test,
        )
        .expect_err("child exited before receipt");
    assert!(matches!(
        error,
        InertRunnerErrorV3::ChannelLostIssuedOrUncertain(_)
    ));
    let proof = runner
        .prove_dead(Duration::from_secs(1))
        .expect("channel loss retains death proof");
    assert_eq!(proof.receipt().effect_id, 21);
    assert_eq!(proof.receipt().operation_nonce, NONCE);
    assert_eq!(proof.receipt().command_sha256, sha256(b"lost-channel"));
}

#[test]
fn issue_record_rejects_any_authority_bit() {
    let mut record = IssuedEffectRecordV3 {
        authority: DisposableAuthorityV2::none(),
        command_sha256: sha256(b"command"),
        effect_id: 41,
        issue_context: EffectIssueContextV3::FreshProcess,
        journal_tip_before_sha256: None,
        operation_nonce: NONCE.to_string(),
        previous_record_sha256: None,
        process_epoch_sha256: "33".repeat(32),
        purpose: EffectPurposeV3::ForwardFlow,
        runner_epoch_sha256: "44".repeat(32),
        schema: ISSUE_SCHEMA_V3.to_string(),
        schema_version: 3,
    };
    record.authority.privileged_effect_authority = true;
    assert!(record.validate().is_err());
}
