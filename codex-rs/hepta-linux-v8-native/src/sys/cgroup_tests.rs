use super::CgroupTypeV8;
use super::parse_cgroup_events_v8;
use super::parse_cgroup_procs_v8;
use super::parse_cgroup_type_v8;
use super::parse_self_cgroup_path_v8;
use super::parse_subtree_control_v8;

#[test]
fn cgroup_control_parsers_are_exact_and_duplicate_closed() {
    assert_eq!(
        parse_cgroup_type_v8(b"domain\n").expect("domain type"),
        CgroupTypeV8::Domain
    );
    assert!(parse_cgroup_type_v8(b"domain\nextra\n").is_err());
    assert_eq!(
        parse_subtree_control_v8(b"memory cpu\n").expect("controller set"),
        vec!["cpu".to_string(), "memory".to_string()]
    );
    assert!(parse_subtree_control_v8(b"cpu cpu\n").is_err());
    assert!(parse_subtree_control_v8(b"+cpu\n").is_err());

    let events = parse_cgroup_events_v8(b"populated 0\nfrozen 1\n").expect("exact events");
    assert!(!events.populated());
    assert!(events.frozen());
    assert!(parse_cgroup_events_v8(b"populated 0\npopulated 1\nfrozen 0\n").is_err());
    assert!(parse_cgroup_events_v8(b"populated 0\nfrozen 0\nunknown 0\n").is_err());

    assert_eq!(
        parse_cgroup_procs_v8(b"42\n7\n").expect("canonical pid set"),
        vec![7, 42]
    );
    assert!(parse_cgroup_procs_v8(b"7\n7\n").is_err());
    assert!(parse_cgroup_procs_v8(b"07\n").is_err());
}

#[test]
fn self_cgroup_parser_requires_one_canonical_unified_row() {
    assert_eq!(
        parse_self_cgroup_path_v8(b"0::/system.slice/hepta-linux-v8-admissiond.service\n")
            .expect("fixed admissiond row"),
        "/system.slice/hepta-linux-v8-admissiond.service"
    );
    assert_eq!(
        parse_self_cgroup_path_v8(b"0::/\n").expect("canonical unified-root membership"),
        "/"
    );
    assert!(parse_self_cgroup_path_v8(b"0::/user.slice/wrong.service\n0::/dup\n").is_err());
    assert!(parse_self_cgroup_path_v8(b"1:name=/system.slice/x\n").is_err());
    assert!(parse_self_cgroup_path_v8(b"0::/system.slice/../escape\n").is_err());
}

#[cfg(not(target_os = "linux"))]
#[test]
fn cgroup_probe_fails_closed_off_linux() {
    assert!(matches!(
        super::probe_fixed_cgroup_v2_root_v8(),
        Err(super::NativeSysErrorV8::UnsupportedPlatform(_))
    ));
}

#[cfg(target_os = "linux")]
mod linux {
    use super::super::PROCESS_FD_LIFETIME_TEST_MUTEX;
    use super::super::ProcfsRootV8;
    use super::super::abort_runner_scope_freeze_v8;
    use super::super::acquire_candidate_cgroup_namespace_lease_v8;
    use super::super::cleanup_candidate_cgroup_v8;
    use super::super::clone_candidate_into_cgroup_with_barrier_v8;
    use super::super::create_candidate_cgroup_leaf_after_bind_fault_for_test_v8;
    use super::super::create_candidate_cgroup_leaf_v8;
    use super::super::execute_runner_scope_continue_after_issue_fault_for_test_v8;
    use super::super::execute_runner_scope_continue_v8;
    use super::super::full_executable_hash_count_for_test_v8;
    use super::super::issue_candidate_cgroup_kill_v8;
    use super::super::issue_runner_scope_freeze_v8;
    use super::super::issue_runner_scope_partial_stop_fault_for_test_v8;
    use super::super::issue_runner_scope_stop_v8;
    use super::super::issue_runner_scope_unfreeze_v8;
    use super::super::mark_runner_restore_intent_for_test_v8;
    use super::super::mark_runner_stop_observed_for_test_v8;
    use super::super::observe_explicit_test_runner_scope_v8;
    use super::super::observe_process_exact_v8;
    use super::super::open_explicit_test_cgroup_root_v8;
    use super::super::open_explicit_test_systemd_scope_v8;
    use super::super::plan_runner_scope_continue_v8;
    use super::super::plan_runner_scope_recovery_continue_v8;
    use super::super::populate_candidate_cgroup_for_test_v8;
    use super::super::prepare_candidate_cgroup_empty_for_cleanup_v8;
    use super::super::recover_candidate_cgroup_create_v8;
    use super::super::reset_full_executable_hash_count_for_test_v8;
    use super::super::retry_runner_scope_freeze_v8;
    use super::super::wait_candidate_cgroup_empty_v8;
    use super::super::wait_runner_scope_freeze_aborted_v8;
    use super::super::wait_runner_scope_frozen_v8;
    use super::super::wait_runner_scope_unfrozen_stopped_v8;
    use crate::DurableRunnerStopIntentV8;
    use std::fs;
    use std::os::unix::fs::PermissionsExt as _;
    use std::os::unix::process::CommandExt as _;
    use std::path::Path;
    use std::process::Child;
    use std::process::Command;
    use std::time::Duration;
    use std::time::Instant;

    fn write_parent_controls(root: &Path, include_type: bool) {
        fs::set_permissions(root, fs::Permissions::from_mode(0o700)).expect("secure explicit root");
        if include_type {
            fs::write(root.join("cgroup.type"), b"domain\n").expect("parent type");
            fs::set_permissions(root.join("cgroup.type"), fs::Permissions::from_mode(0o600))
                .expect("protect parent type");
        }
        fs::write(root.join("cgroup.controllers"), b"cpu memory pids\n")
            .expect("parent controllers");
        fs::write(root.join("cgroup.subtree_control"), b"cpu memory\n")
            .expect("parent subtree control");
        for name in ["cgroup.controllers", "cgroup.subtree_control"] {
            fs::set_permissions(root.join(name), fs::Permissions::from_mode(0o600))
                .expect("protect parent control fixture");
        }
    }

    fn write_scope_controls(root: &Path, pids: &[u32]) -> std::path::PathBuf {
        fs::set_permissions(root, fs::Permissions::from_mode(0o700)).expect("secure scope root");
        let scope = root.join("runner.scope");
        fs::create_dir(&scope).expect("create explicit runner scope");
        fs::set_permissions(&scope, fs::Permissions::from_mode(0o700))
            .expect("secure explicit runner scope");
        let procs = pids
            .iter()
            .map(u32::to_string)
            .collect::<Vec<_>>()
            .join("\n")
            + "\n";
        for (name, bytes) in [
            ("cgroup.type", b"domain\n".as_slice()),
            ("cgroup.subtree_control", b"".as_slice()),
            ("cgroup.events", b"populated 1\nfrozen 0\n".as_slice()),
            ("cgroup.procs", procs.as_bytes()),
            ("cgroup.freeze", b"0\n".as_slice()),
        ] {
            fs::write(scope.join(name), bytes).expect("write explicit scope control");
            fs::set_permissions(scope.join(name), fs::Permissions::from_mode(0o600))
                .expect("protect explicit scope control");
        }
        scope
    }

    struct IsolatedRunnerFixture {
        leader: Child,
        process_group_id: i32,
        pids: Vec<u32>,
    }

    impl IsolatedRunnerFixture {
        fn spawn() -> Self {
            let mut command = Command::new("/bin/sh");
            command
                .arg("-c")
                .arg("sleep 30 & sleep 30 & sleep 30 & sleep 30 & sleep 30 & wait");
            // SAFETY: setsid is async-signal-safe and has no pointer arguments.
            unsafe {
                command.pre_exec(|| {
                    if libc::setsid() < 0 {
                        return Err(std::io::Error::last_os_error());
                    }
                    Ok(())
                });
            }
            let leader = command.spawn().expect("spawn isolated runner session");
            let process_group_id = i32::try_from(leader.id()).expect("runner pgid fits pid_t");
            let root = ProcfsRootV8::open_fixed().expect("fixed procfs for runner fixture");
            let deadline = Instant::now() + Duration::from_secs(3);
            let pids = loop {
                let observed = root
                    .process_group_and_session_pids(leader.id(), leader.id())
                    .expect("enumerate isolated runner PGID/SID");
                if observed.0 == observed.1 && observed.0.len() == 6 {
                    break observed.0;
                }
                assert!(
                    Instant::now() < deadline,
                    "isolated runner did not reach exact two-process PGID/SID closure"
                );
                std::thread::sleep(Duration::from_millis(5));
            };
            Self {
                leader,
                process_group_id,
                pids,
            }
        }
    }

    impl Drop for IsolatedRunnerFixture {
        fn drop(&mut self) {
            // SAFETY: this targets only the disposable session/group created
            // above; CONT prevents a stopped fixture from blocking teardown.
            unsafe { libc::kill(-self.process_group_id, libc::SIGCONT) };
            // SAFETY: see above.
            unsafe { libc::kill(-self.process_group_id, libc::SIGKILL) };
            let _ = self.leader.wait();
        }
    }

    #[test]
    fn candidate_lease_clone_gate_kill_wait_retry_and_cleanup_are_consumptive() {
        let _serial = PROCESS_FD_LIFETIME_TEST_MUTEX
            .lock()
            .expect("candidate fixture mutex");
        let temporary = tempfile::tempdir().expect("explicit cgroup test root");
        write_parent_controls(temporary.path(), true);
        let root = open_explicit_test_cgroup_root_v8(temporary.path()).expect("test root proof");
        let lease = acquire_candidate_cgroup_namespace_lease_v8(root).expect("exclusive lease");
        let candidate =
            create_candidate_cgroup_leaf_v8(lease, [0x11; 32]).expect("attempt-derived candidate");
        assert_eq!(candidate.leaf(), format!("hepta-v8-{}", "11".repeat(32)));
        let clone_failure = clone_candidate_into_cgroup_with_barrier_v8(candidate)
            .expect_err("explicit fixture must not become a numeric clone fallback");
        assert!(
            clone_failure
                .cause()
                .to_string()
                .contains("never falls back")
        );
        let candidate = clone_failure
            .into_before_effect_candidate()
            .expect("explicit clone rejection is before effect");

        let mut child = Command::new("/bin/sleep")
            .arg("30")
            .spawn()
            .expect("disposable process");
        let observation = observe_process_exact_v8(child.id()).expect("pidfd exact child");
        let running = populate_candidate_cgroup_for_test_v8(candidate, &observation)
            .expect("explicit fixture population");
        assert_eq!(running.child_pid(), child.id());
        let issued = issue_candidate_cgroup_kill_v8(running).expect("exact kill obligation");
        assert!(!issued.issue_was_uncertain());
        assert_eq!(
            fs::read(temporary.path().join(format!(
                "{}/cgroup.kill",
                format!("hepta-v8-{}", "11".repeat(32))
            )))
            .expect("kill fixture"),
            b"1\n"
        );

        let timeout = wait_candidate_cgroup_empty_v8(issued, Duration::from_millis(1))
            .expect_err("populated fixture must retain wait obligation");
        assert!(timeout.cause().to_string().contains("did not become empty"));
        let issued = timeout.into_issued();
        child.kill().expect("kill disposable child");
        child.wait().expect("reap disposable child");
        let leaf_path = temporary
            .path()
            .join(format!("hepta-v8-{}", "11".repeat(32)));
        fs::write(leaf_path.join("cgroup.procs"), b"").expect("clear fixture procs");
        fs::write(leaf_path.join("cgroup.events"), b"populated 0\nfrozen 0\n")
            .expect("clear fixture events");
        let empty = wait_candidate_cgroup_empty_v8(issued, Duration::from_secs(2))
            .expect("retry exact empty wait");
        assert_eq!(empty.proof().observed_process_count(), 0);
        assert_ne!(empty.proof().observation_sha256(), [0; 32]);
        let terminal = cleanup_candidate_cgroup_v8(empty).expect("descriptor-bound cleanup");
        assert_eq!(terminal.absence_proof().child_link_count_after_cleanup(), 0);
        assert!(terminal.absence_proof().name_absent());
        assert_ne!(terminal.absence_proof().observation_sha256(), [0; 32]);
        assert!(!Path::new(terminal.absence_proof().absolute_path()).exists());
        let lease = terminal.into_namespace_lease();
        assert_eq!(lease.parent_path(), temporary.path().to_string_lossy());
    }

    #[test]
    fn namespace_conflict_zero_digest_and_name_replacement_fail_closed_before_effect() {
        let missing = tempfile::tempdir().expect("missing-type root");
        write_parent_controls(missing.path(), false);
        assert!(open_explicit_test_cgroup_root_v8(missing.path()).is_err());

        let occupied = tempfile::tempdir().expect("occupied cgroup root");
        write_parent_controls(occupied.path(), true);
        fs::create_dir(occupied.path().join("hepta-v8-retained-obligation"))
            .expect("retained candidate name");
        let occupied_root =
            open_explicit_test_cgroup_root_v8(occupied.path()).expect("occupied root proof");
        assert!(acquire_candidate_cgroup_namespace_lease_v8(occupied_root).is_err());

        let temporary = tempfile::tempdir().expect("explicit cgroup test root");
        write_parent_controls(temporary.path(), true);
        let root = open_explicit_test_cgroup_root_v8(temporary.path()).expect("test root proof");
        let lease = acquire_candidate_cgroup_namespace_lease_v8(root).expect("exclusive lease");
        let zero = create_candidate_cgroup_leaf_v8(lease, [0; 32])
            .expect_err("all-zero attempt digest must fail before mkdir");
        let lease = zero
            .into_before_effect_lease()
            .expect("zero digest preserves retry lease");
        let candidate =
            create_candidate_cgroup_leaf_v8(lease, [0x22; 32]).expect("candidate after retry");
        let leaf_name = candidate.leaf().to_string();
        let original = temporary.path().join(&leaf_name);
        let moved = temporary.path().join("retained-original");
        fs::rename(&original, &moved).expect("move exact candidate aside");
        fs::create_dir(&original).expect("inject same-name replacement");
        let replacement = clone_candidate_into_cgroup_with_barrier_v8(candidate)
            .expect_err("same-name replacement must fail before clone3");
        let candidate = replacement
            .into_before_effect_candidate()
            .expect("replacement detected before clone effect");
        fs::remove_dir(&original).expect("remove injected replacement");
        fs::rename(&moved, &original).expect("restore exact candidate pathname");
        let empty = prepare_candidate_cgroup_empty_for_cleanup_v8(candidate)
            .expect("prepare restored candidate cleanup");
        let terminal = cleanup_candidate_cgroup_v8(empty).expect("cleanup restored candidate");
        assert!(terminal.absence_proof().name_absent());
    }

    #[test]
    fn post_mkdir_candidate_fault_retains_descriptor_bound_recovery_and_cleanup() {
        let temporary = tempfile::tempdir().expect("explicit cgroup test root");
        write_parent_controls(temporary.path(), true);
        let root = open_explicit_test_cgroup_root_v8(temporary.path()).expect("test root proof");
        let lease = acquire_candidate_cgroup_namespace_lease_v8(root).expect("exclusive lease");
        let failure = create_candidate_cgroup_leaf_after_bind_fault_for_test_v8(lease, [0x33; 32])
            .expect_err("post-mkdir fault must retain an issued obligation");
        assert!(failure.cause().to_string().contains("post-mkdir"));
        let obligation = failure
            .into_issued()
            .expect("post-mkdir fault cannot return retry authority");
        assert_eq!(
            obligation.leaf_name(),
            format!("hepta-v8-{}", "33".repeat(32))
        );
        let candidate = recover_candidate_cgroup_create_v8(obligation)
            .expect("recover retained descriptor/name binding");
        let empty = prepare_candidate_cgroup_empty_for_cleanup_v8(candidate)
            .expect("prove recovered candidate empty");
        let terminal = cleanup_candidate_cgroup_v8(empty)
            .expect("cleanup recovered candidate through descriptor chain");
        assert!(terminal.absence_proof().name_absent());
        assert_eq!(terminal.absence_proof().child_link_count_after_cleanup(), 0);
    }

    #[test]
    fn runner_scope_replacement_timeout_abort_stop_unfreeze_and_unique_continue_are_closed() {
        let _serial = PROCESS_FD_LIFETIME_TEST_MUTEX
            .lock()
            .expect("runner scope fixture mutex");
        let runner = IsolatedRunnerFixture::spawn();
        let temporary = tempfile::tempdir().expect("explicit runner scope root");
        let scope_path = write_scope_controls(temporary.path(), &runner.pids);
        let scope = open_explicit_test_systemd_scope_v8(temporary.path(), "/runner.scope")
            .expect("descriptor-bound explicit scope");
        let observed = observe_explicit_test_runner_scope_v8(scope, &runner.pids)
            .expect("exact PGID/SID/runtime scope closure");
        assert_eq!(
            observed.unit_name(),
            "explicit-runner-scope-fixture.service"
        );

        reset_full_executable_hash_count_for_test_v8();
        let intent = DurableRunnerStopIntentV8::test_only_for_observed(&observed)
            .expect("bind fixture durable STOP manifest");
        let issued = issue_runner_scope_freeze_v8(observed, intent).expect("issue fixture freeze");
        let expected_intent_record_sha256 = "4".repeat(64);
        assert_eq!(
            issued.intent_record_sha256(),
            Some(expected_intent_record_sha256.as_str())
        );
        assert!(!issued.issue_was_uncertain());
        fs::write(
            scope_path.join("cgroup.procs"),
            format!("{}\n", runner.pids[0]),
        )
        .expect("inject scope roster replacement");
        let replacement = wait_runner_scope_frozen_v8(issued, Duration::from_secs(2))
            .expect_err("roster replacement must fail immediately");
        assert!(replacement.cause().to_string().contains("roster"));
        let issued = replacement.into_issued();
        fs::write(
            scope_path.join("cgroup.procs"),
            runner
                .pids
                .iter()
                .map(u32::to_string)
                .collect::<Vec<_>>()
                .join("\n")
                + "\n",
        )
        .expect("restore exact scope roster");
        let abort = abort_runner_scope_freeze_v8(issued).expect("issue freeze abort");
        let observed = wait_runner_scope_freeze_aborted_v8(abort, Duration::from_secs(2))
            .expect("wait freeze abort");

        let issued = retry_runner_scope_freeze_v8(observed).expect("retry second freeze");
        fs::write(scope_path.join("cgroup.events"), b"populated 1\nfrozen 0\n")
            .expect("delay fixture frozen state");
        let timeout = wait_runner_scope_frozen_v8(issued, Duration::from_millis(1))
            .expect_err("unreached frozen state must retain retry token");
        let issued = timeout.into_issued();
        fs::write(scope_path.join("cgroup.events"), b"populated 1\nfrozen 1\n")
            .expect("complete fixture frozen state");
        let frozen =
            wait_runner_scope_frozen_v8(issued, Duration::from_secs(2)).expect("retry frozen wait");
        let stopped = issue_runner_scope_stop_v8(frozen).expect("issue exact group STOP");
        assert!(!stopped.issue_was_uncertain());
        let unfreeze = issue_runner_scope_unfreeze_v8(stopped).expect("issue cgroup unfreeze");
        assert!(!unfreeze.stop_issue_was_uncertain());
        assert!(!unfreeze.unfreeze_issue_was_uncertain());
        let stopped = wait_runner_scope_unfrozen_stopped_v8(unfreeze, Duration::from_secs(2))
            .expect("verify unfrozen exact T closure");
        let observed_stopped =
            mark_runner_stop_observed_for_test_v8(stopped).expect("close fixture STOP intent");
        assert_eq!(observed_stopped.observation_record_sha256(), "5".repeat(64));
        let restore_authorized = mark_runner_restore_intent_for_test_v8(observed_stopped)
            .expect("bind fixture durable RESTORE intent");
        let plan = plan_runner_scope_continue_v8(restore_authorized).expect("single-use CONT plan");
        let execution = execute_runner_scope_continue_v8(plan).expect("unique CONT execution");
        assert!(execution.runner_restore_intent().is_some());
        execution
            .capture_runner_restore_observation_v8()
            .expect("capture typed running observation");
        assert_eq!(execution.process_group_id, runner.pids[0]);
        assert!(!execution.recovery);
        assert!(
            execution
                .after
                .iter()
                .all(|identity| !identity.state().is_stopped())
        );
        assert_eq!(
            full_executable_hash_count_for_test_v8(),
            0,
            "no executable hashing is allowed after the first freeze effect"
        );
    }

    #[test]
    fn durable_runner_stop_manifest_cannot_authorize_a_different_scope() {
        let _serial = PROCESS_FD_LIFETIME_TEST_MUTEX
            .lock()
            .expect("runner scope fixture mutex");
        let first_runner = IsolatedRunnerFixture::spawn();
        let second_runner = IsolatedRunnerFixture::spawn();
        let first_root = tempfile::tempdir().expect("first explicit runner scope root");
        let second_root = tempfile::tempdir().expect("second explicit runner scope root");
        write_scope_controls(first_root.path(), &first_runner.pids);
        write_scope_controls(second_root.path(), &second_runner.pids);
        let first_scope = open_explicit_test_systemd_scope_v8(first_root.path(), "/runner.scope")
            .expect("first descriptor-bound scope");
        let second_scope = open_explicit_test_systemd_scope_v8(second_root.path(), "/runner.scope")
            .expect("second descriptor-bound scope");
        let first_observed = observe_explicit_test_runner_scope_v8(first_scope, &first_runner.pids)
            .expect("first exact runner closure");
        let second_observed =
            observe_explicit_test_runner_scope_v8(second_scope, &second_runner.pids)
                .expect("second exact runner closure");
        let first_intent = DurableRunnerStopIntentV8::test_only_for_observed(&first_observed)
            .expect("bind first durable STOP manifest");
        let conflict = issue_runner_scope_freeze_v8(second_observed, first_intent)
            .expect_err("scope A durable intent must not mutate scope B");
        assert!(
            conflict
                .cause()
                .expect("cross-scope conflict cause")
                .to_string()
                .contains("durable STOP manifest")
        );
        assert!(conflict.into_intent_conflict().is_some());
    }

    #[test]
    fn post_continue_fault_returns_quarantine_evidence_without_resend_authority() {
        let _serial = PROCESS_FD_LIFETIME_TEST_MUTEX
            .lock()
            .expect("runner scope fixture mutex");
        let runner = IsolatedRunnerFixture::spawn();
        let temporary = tempfile::tempdir().expect("explicit runner scope root");
        write_scope_controls(temporary.path(), &runner.pids);
        let scope = open_explicit_test_systemd_scope_v8(temporary.path(), "/runner.scope")
            .expect("descriptor-bound explicit scope");
        let observed = observe_explicit_test_runner_scope_v8(scope, &runner.pids)
            .expect("exact PGID/SID/runtime scope closure");

        reset_full_executable_hash_count_for_test_v8();
        let intent = DurableRunnerStopIntentV8::test_only_for_observed(&observed)
            .expect("bind fixture durable STOP manifest");
        let issued = issue_runner_scope_freeze_v8(observed, intent).expect("issue fixture freeze");
        let frozen = wait_runner_scope_frozen_v8(issued, Duration::from_secs(2))
            .expect("wait exact frozen scope");
        let stopped = issue_runner_scope_stop_v8(frozen).expect("issue exact group STOP");
        let unfreeze = issue_runner_scope_unfreeze_v8(stopped).expect("unfreeze after STOP");
        let stopped = wait_runner_scope_unfrozen_stopped_v8(unfreeze, Duration::from_secs(2))
            .expect("verify exact stopped closure");
        let stopped =
            mark_runner_stop_observed_for_test_v8(stopped).expect("close fixture STOP intent");
        let restore_authorized = mark_runner_restore_intent_for_test_v8(stopped)
            .expect("bind fixture durable RESTORE intent");
        let plan = plan_runner_scope_continue_v8(restore_authorized).expect("single-use CONT plan");
        let quarantine = execute_runner_scope_continue_after_issue_fault_for_test_v8(plan)
            .expect_err("post-CONT fault must quarantine, never return resend authority");
        assert!(quarantine.cause().to_string().contains("post-CONT"));
        assert!(quarantine.runner_restore_intent().is_some());
        assert_eq!(quarantine.before().len(), runner.pids.len());
        assert_eq!(
            quarantine
                .revalidate_for_quarantine()
                .expect("descriptor-bound quarantine revalidation")
                .len(),
            runner.pids.len()
        );
        assert_eq!(
            full_executable_hash_count_for_test_v8(),
            0,
            "post-effect quarantine must remain on the fast no-hash path"
        );
    }

    #[test]
    fn partial_stop_and_roster_drift_retain_the_only_recovery_continue_token() {
        let _serial = PROCESS_FD_LIFETIME_TEST_MUTEX
            .lock()
            .expect("runner scope fixture mutex");
        let runner = IsolatedRunnerFixture::spawn();
        let temporary = tempfile::tempdir().expect("explicit runner scope root");
        let scope_path = write_scope_controls(temporary.path(), &runner.pids);
        let scope = open_explicit_test_systemd_scope_v8(temporary.path(), "/runner.scope")
            .expect("descriptor-bound explicit scope");
        let observed = observe_explicit_test_runner_scope_v8(scope, &runner.pids)
            .expect("exact PGID/SID/runtime scope closure");

        let intent = DurableRunnerStopIntentV8::test_only_for_observed(&observed)
            .expect("bind fixture durable STOP manifest");
        let issued = issue_runner_scope_freeze_v8(observed, intent).expect("issue fixture freeze");
        let frozen = wait_runner_scope_frozen_v8(issued, Duration::from_secs(2))
            .expect("wait exact frozen scope");
        let stopped = issue_runner_scope_partial_stop_fault_for_test_v8(frozen)
            .expect("partial STOP still returns an issued/uncertain token");
        assert!(stopped.issue_was_uncertain());

        let partial_deadline = Instant::now() + Duration::from_secs(2);
        loop {
            let stopped_members = runner
                .pids
                .iter()
                .filter(|pid| {
                    observe_process_exact_v8(**pid)
                        .expect("reobserve partial STOP member")
                        .identity()
                        .state()
                        .is_stopped()
                })
                .count();
            if stopped_members == 1 {
                break;
            }
            assert!(
                Instant::now() < partial_deadline,
                "partial STOP did not reach one exact stopped member"
            );
            std::thread::sleep(Duration::from_millis(5));
        }

        let unfreeze = issue_runner_scope_unfreeze_v8(stopped)
            .expect("unfreeze retains the ambiguous STOP recovery route");
        let recovery = wait_runner_scope_unfrozen_stopped_v8(unfreeze, Duration::from_millis(25))
            .expect_err("mixed T/non-T closure must not become a normal CONT token");
        assert!(recovery.cause().to_string().contains("did not all reach T"));

        fs::write(
            scope_path.join("cgroup.procs"),
            format!("{}\n", runner.pids[0]),
        )
        .expect("inject recovery roster drift");
        let planning = plan_runner_scope_recovery_continue_v8(recovery)
            .expect_err("roster drift must retain the recovery token");
        assert!(planning.cause().to_string().contains("roster"));
        let recovery = match planning {
            super::super::RunnerScopeContinuePlanFailureV8::Recovery { failure, .. } => failure,
            super::super::RunnerScopeContinuePlanFailureV8::Stopped { .. } => {
                panic!("recovery planning returned the normal stopped variant")
            }
        };

        fs::write(
            scope_path.join("cgroup.procs"),
            runner
                .pids
                .iter()
                .map(u32::to_string)
                .collect::<Vec<_>>()
                .join("\n")
                + "\n",
        )
        .expect("restore exact recovery roster");
        let plan = plan_runner_scope_recovery_continue_v8(recovery)
            .expect("retained token plans the unique recovery CONT");
        let execution = execute_runner_scope_continue_v8(plan)
            .expect("unique recovery CONT restores every exact member");
        assert!(execution.recovery);
        assert!(
            execution.unresolved_runner_stop_intent().is_some(),
            "recovery CONT must retain the unresolved durable STOP obligation"
        );
        assert_eq!(execution.after.len(), runner.pids.len());
        assert!(
            execution
                .after
                .iter()
                .all(|identity| !identity.state().is_stopped())
        );
    }
}
