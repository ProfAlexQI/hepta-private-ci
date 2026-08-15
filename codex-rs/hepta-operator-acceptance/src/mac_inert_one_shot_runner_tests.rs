use super::*;
use std::collections::BTreeSet;
use std::fs::OpenOptions;
use std::io::Read;
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
        for target in CHILD_FIXED_FDS_V3 {
            assert_eq!(
                unsafe { libc::fcntl(target, libc::F_GETFD) },
                -1,
                "foreign exec inherited a reserved child target FD",
            );
        }
        let forbidden_anonymous = std::env::var("HEPTA_FORBIDDEN_ANON_FDS_V3")
            .expect("forbidden anonymous identities")
            .split(',')
            .filter(|value| !value.is_empty())
            .map(str::to_string)
            .collect::<BTreeSet<_>>();
        let forbidden_lease =
            std::env::var("HEPTA_FORBIDDEN_LEASE_FD_V3").expect("forbidden lease identity");
        for fd in 3..fd_scan_limit() {
            if unsafe { libc::fcntl(fd, libc::F_GETFD) } < 0 {
                continue;
            }
            let identity = fd_identity(fd).expect("identity of open foreign FD");
            assert_ne!(
                identity, forbidden_lease,
                "foreign exec inherited lease at FD {fd}",
            );
            if fd_is_anonymous(fd) {
                assert!(
                    !forbidden_anonymous.contains(&identity),
                    "foreign exec inherited a preallocated anonymous runner FD {fd} with identity {identity}",
                );
            }
        }
        thread::sleep(Duration::from_secs(30));
    }
}

#[test]
fn multithreaded_bootstrap_rejection_entry() {
    if std::env::var_os("HEPTA_MULTITHREADED_BOOTSTRAP_V3").is_none() {
        return;
    }
    let barrier = Arc::new(Barrier::new(2));
    let (release, wait) = mpsc::channel();
    let worker_barrier = Arc::clone(&barrier);
    let worker = thread::spawn(move || {
        worker_barrier.wait();
        wait.recv().expect("release bootstrap worker");
    });
    barrier.wait();
    let error = FreshProcessEpochV3::establish()
        .err()
        .expect("multithreaded bootstrap must fail closed");
    assert!(matches!(error, InertRunnerErrorV3::Invalid(_)));
    release.send(()).expect("release bootstrap worker");
    worker.join().expect("bootstrap worker");
}

fn fd_scan_limit() -> RawFd {
    let mut limit = MaybeUninit::<libc::rlimit>::zeroed();
    assert_eq!(
        unsafe { libc::getrlimit(libc::RLIMIT_NOFILE, limit.as_mut_ptr()) },
        0,
        "RLIMIT_NOFILE",
    );
    let limit = unsafe { limit.assume_init() };
    limit.rlim_cur.min(4096) as RawFd
}

fn fd_identity(fd: RawFd) -> Option<String> {
    let mut stat = MaybeUninit::<libc::stat>::zeroed();
    if unsafe { libc::fstat(fd, stat.as_mut_ptr()) } != 0 {
        return None;
    }
    let stat = unsafe { stat.assume_init() };
    Some(format!(
        "{:x}:{:x}:{:x}:{:x}",
        stat.st_dev, stat.st_ino, stat.st_mode, stat.st_rdev,
    ))
}

fn fd_is_anonymous(fd: RawFd) -> bool {
    let mut path = [0u8; libc::PATH_MAX as usize];
    (unsafe { libc::fcntl(fd, libc::F_GETPATH, path.as_mut_ptr()) }) < 0
}

fn preallocated_anonymous_fd_identities() -> String {
    let mut identities = BTreeSet::new();
    for slot in 0..PREALLOCATED_RUNNER_SLOTS_V3 {
        if PREALLOCATED_SLOT_TAKEN_V3[slot].load(Ordering::Acquire) != 0 {
            continue;
        }
        let base = slot * PREALLOCATED_SLOT_FDS_V3;
        let mut slot_identities = Vec::new();
        let mut complete = true;
        let mut all_close_on_exec = true;
        for offset in 0..PREALLOCATED_SLOT_FDS_V3 {
            let fd = PREALLOCATED_FD_TABLE_V3[base + offset].load(Ordering::Acquire);
            let flags = if fd >= 0 {
                unsafe { libc::fcntl(fd, libc::F_GETFD) }
            } else {
                -1
            };
            if flags < 0 {
                complete = false;
                break;
            }
            all_close_on_exec &= flags & libc::FD_CLOEXEC != 0;
            if fd_is_anonymous(fd)
                && let Some(identity) = fd_identity(fd)
            {
                slot_identities.push(identity);
            }
        }
        // Slot ownership only transitions 0 -> 1.  If it stayed untaken for
        // the whole scan, every identity above belongs to the immutable pool;
        // otherwise discard the racing snapshot instead of classifying a
        // subsequently reused descriptor as a pool object.
        if complete && PREALLOCATED_SLOT_TAKEN_V3[slot].load(Ordering::Acquire) == 0 {
            assert!(all_close_on_exec, "preallocated runner FD lost CLOEXEC");
            identities.extend(slot_identities);
        }
    }
    identities.into_iter().collect::<Vec<_>>().join(",")
}

fn assert_flock_available_within(descriptor: &File, timeout: Duration, message: &str) {
    let deadline = Instant::now() + timeout;
    loop {
        if unsafe { libc::flock(descriptor.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } == 0 {
            return;
        }
        assert_eq!(
            io::Error::last_os_error().raw_os_error(),
            Some(libc::EWOULDBLOCK),
        );
        assert!(Instant::now() < deadline, "{message}");
        thread::sleep(Duration::from_millis(5));
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
fn fresh_process_pool_bootstrap_rejects_a_multithreaded_process() {
    let status = Command::new(helper_program())
        .args(helper_arguments(
            "mac_inert_one_shot_runner::tests::multithreaded_bootstrap_rejection_entry",
        ))
        .env(SKIP_PREMAIN_BOOTSTRAP_ENV_V3, "1")
        .env("HEPTA_MULTITHREADED_BOOTSTRAP_V3", "1")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .expect("spawn isolated multithreaded bootstrap helper");
    assert!(
        status.success(),
        "bootstrap rejection helper failed: {status}"
    );
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
    assert_flock_available_within(
        &contender,
        Duration::from_secs(2),
        "flock did not become available after child exit",
    );
}

#[test]
fn concurrent_foreign_execs_never_inherit_the_runner_lease() {
    let epoch = FreshProcessEpochV3::establish().expect("fresh process epoch");
    let (directory, lease) = test_lease();
    let forbidden_lease = fd_identity(lease.descriptor.as_raw_fd()).expect("lease identity");
    let forbidden_anonymous = preallocated_anonymous_fd_identities();
    let (ready_sender, ready_receiver) = mpsc::channel();
    let (sender, receiver) = mpsc::channel();
    let foreign_program = helper_program();
    let foreign = thread::spawn(move || {
        let mut children = Vec::new();
        for index in 0..24 {
            let child = Command::new(&foreign_program)
                .args([
                    "--exact",
                    "mac_inert_one_shot_runner::tests::foreign_exec_sleeper_entry",
                    "--nocapture",
                ])
                .env("HEPTA_FOREIGN_EXEC_SLEEPER_V3", "1")
                .env(SKIP_PREMAIN_BOOTSTRAP_ENV_V3, "1")
                .env("HEPTA_FORBIDDEN_ANON_FDS_V3", &forbidden_anonymous)
                .env("HEPTA_FORBIDDEN_LEASE_FD_V3", &forbidden_lease)
                .env_remove("HEPTA_INERT_RUNNER_COMMAND_FD_V3")
                .env_remove("HEPTA_INERT_RUNNER_RESPONSE_FD_V3")
                .env_remove("HEPTA_INERT_RUNNER_DEATH_FD_V3")
                .env_remove("HEPTA_INERT_RUNNER_LEASE_FD_V3")
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::piped())
                .spawn()
                .expect("spawn foreign exec helper");
            children.push(child);
            if index == 0 {
                ready_sender.send(()).expect("signal first foreign exec");
            }
        }
        sender.send(children).expect("send foreign children");
    });

    ready_receiver.recv().expect("first foreign exec started");
    let arguments = helper_arguments("mac_inert_one_shot_runner::tests::inert_child_entry");
    let mut runner = LiveInertRunnerV3::spawn_program(
        &epoch,
        &lease,
        &helper_program(),
        &arguments,
        Duration::from_secs(5),
    )
    .expect("spawn runner during foreign exec churn");
    issue_once(&mut runner, &epoch);
    let proof_started = Instant::now();
    runner
        .prove_dead(Duration::from_millis(500))
        .expect("runner death proof");
    assert!(
        proof_started.elapsed() < Duration::from_secs(2),
        "foreign helpers delayed runner death EOF",
    );
    let mut foreign_children = receiver.recv().expect("receive foreign children");
    foreign.join().expect("foreign spawner");
    let mut early_failures = Vec::new();
    for child in &mut foreign_children {
        if let Some(status) = child.try_wait().expect("foreign child status") {
            let mut stderr = String::new();
            if let Some(mut stream) = child.stderr.take() {
                stream.read_to_string(&mut stderr).expect("foreign stderr");
            }
            early_failures.push(format!("{status}: {stderr}"));
        }
    }
    assert!(
        early_failures.is_empty(),
        "foreign helper detected an inherited FD or exited before the proof timeout: {early_failures:?}",
    );

    drop(lease);
    let contender = OpenOptions::new()
        .read(true)
        .write(true)
        .open(directory.path().join("control.lock"))
        .expect("open contender after runner exit");
    assert_flock_available_within(
        &contender,
        Duration::from_secs(2),
        "foreign exec inherited a lease FD from the parent spawn window",
    );
    for child in &mut foreign_children {
        let _ = child.kill();
        let _ = child.wait();
    }
}

#[test]
fn high_fd_pressure_and_exec_failure_do_not_collide_with_fixed_targets() {
    let epoch = FreshProcessEpochV3::establish().expect("fresh process epoch");
    let mut pressure = Vec::new();
    let mut highest = 0;
    while highest < CHILD_COMMAND_FD_V3 - 1 {
        let descriptor = File::open("/dev/null").expect("open CLOEXEC pressure descriptor");
        let flags = unsafe { libc::fcntl(descriptor.as_raw_fd(), libc::F_GETFD) };
        assert!(flags >= 0 && flags & libc::FD_CLOEXEC != 0);
        highest = highest.max(descriptor.as_raw_fd());
        pressure.push(descriptor);
        assert!(
            pressure.len() < 2048,
            "bounded pressure must reach fixed FDs"
        );
    }
    for target in CHILD_FIXED_FDS_V3 {
        let flags = unsafe { libc::fcntl(target, libc::F_GETFD) };
        assert!(
            flags >= 0 && flags & libc::FD_CLOEXEC != 0,
            "fixed target reservation changed under FD pressure",
        );
    }

    let (directory, lease) = test_lease();
    let missing = directory.path().join("missing-inert-runner-binary");
    let error = match LiveInertRunnerV3::spawn_program(
        &epoch,
        &lease,
        &missing,
        &[],
        Duration::from_millis(200),
    ) {
        Ok(_) => panic!("forced exec failure unexpectedly spawned"),
        Err(error) => error,
    };
    assert!(
        matches!(&error, InertRunnerErrorV3::Io(io_error) if io_error.raw_os_error() == Some(libc::ENOENT)),
        "parent did not receive the exact exec failure: {error}",
    );
    drop(lease);
    let contender = OpenOptions::new()
        .read(true)
        .write(true)
        .open(directory.path().join("control.lock"))
        .expect("open contender after exec failure");
    assert_flock_available_within(
        &contender,
        Duration::from_secs(2),
        "failed exec retained or polluted the control lease",
    );

    let (_valid_directory, mut runner) = spawn_runner(&epoch);
    issue_once(&mut runner, &epoch);
    runner
        .prove_dead(Duration::from_secs(1))
        .expect("next preallocated slot remains usable after exec failure");
    for target in CHILD_FIXED_FDS_V3 {
        let flags = unsafe { libc::fcntl(target, libc::F_GETFD) };
        assert!(flags >= 0 && flags & libc::FD_CLOEXEC != 0);
    }
    drop(pressure);
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
    assert_flock_available_within(
        &contender,
        Duration::from_secs(2),
        "startup timeout did not kill and reap the partial-frame child",
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

#[test]
fn kqueue_exit_event_requires_exact_pid_filter_note_and_no_error() {
    let expected_pid = 4242;
    let mut event = libc::kevent {
        ident: expected_pid as libc::uintptr_t,
        filter: libc::EVFILT_PROC,
        flags: 0,
        fflags: libc::NOTE_EXIT,
        data: 0,
        udata: std::ptr::null_mut(),
    };
    assert!(validate_kqueue_exit_event(&event, expected_pid).expect("exact NOTE_EXIT"));
    assert!(!validate_kqueue_exit_event(&event, expected_pid + 1).expect("foreign PID"));
    event.filter = libc::EVFILT_READ;
    assert!(!validate_kqueue_exit_event(&event, expected_pid).expect("foreign filter"));
    event.filter = libc::EVFILT_PROC;
    event.fflags = 0;
    assert!(!validate_kqueue_exit_event(&event, expected_pid).expect("missing NOTE_EXIT"));
    event.fflags = libc::NOTE_EXIT;
    event.flags = libc::EV_ERROR;
    assert!(validate_kqueue_exit_event(&event, expected_pid).is_err());
}
