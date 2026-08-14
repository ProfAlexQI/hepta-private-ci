#[cfg(not(target_os = "linux"))]
#[test]
fn process_group_signals_fail_closed_off_linux() {
    assert!(matches!(
        super::observe_exact_process_group_v8(&[2, 3]),
        Err(super::NativeSysErrorV8::UnsupportedPlatform(_))
    ));
}

#[cfg(target_os = "linux")]
mod linux {
    use super::super::PROCESS_FD_LIFETIME_TEST_MUTEX as SIGNAL_FIXTURE_MUTEX;
    use super::super::ProcessGroupSignalFailureV8;
    use super::super::ProcessGroupSignalSuccessV8;
    use super::super::SignalPostIssueTestFaultV8;
    use super::super::execute_process_group_signal_v8;
    use super::super::execute_process_group_signal_with_fault_for_test_v8;
    use super::super::full_executable_hash_count_for_test_v8;
    use super::super::observe_exact_process_group_v8;
    use super::super::plan_continue_stopped_process_group_v8;
    use super::super::plan_restore_post_stop_v8;
    use super::super::plan_stop_process_group_v8;
    use super::super::reset_full_executable_hash_count_for_test_v8;
    use super::super::validate_expected_group_inputs_for_test_v8;
    use std::os::unix::process::CommandExt as _;
    use std::process::Child;
    use std::process::Command;
    use std::time::Duration;
    use std::time::Instant;

    struct ProcessGroupFixture {
        children: Vec<Child>,
        pgid: i32,
    }

    impl ProcessGroupFixture {
        fn spawn(member_count: usize) -> Self {
            assert!(member_count >= 2);
            let mut children = Vec::new();
            let leader = spawn_sleep_in_group(0);
            let pgid = i32::try_from(leader.id()).expect("leader pid_t");
            children.push(leader);
            wait_for_pgid(children[0].id(), pgid);
            for _ in 1..member_count {
                children.push(spawn_sleep_in_group(pgid));
                let index = children.len() - 1;
                wait_for_pgid(children[index].id(), pgid);
            }
            Self { children, pgid }
        }

        fn pids(&self) -> Vec<u32> {
            self.children.iter().map(Child::id).collect()
        }
    }

    impl Drop for ProcessGroupFixture {
        fn drop(&mut self) {
            // SAFETY: this is a disposable, externally separated test group.
            unsafe { libc::kill(-self.pgid, libc::SIGCONT) };
            // SAFETY: see above.
            unsafe { libc::kill(-self.pgid, libc::SIGKILL) };
            for child in &mut self.children {
                let _ = child.wait();
            }
        }
    }

    fn spawn_sleep_in_group(pgid: i32) -> Child {
        let mut command = Command::new("/bin/sleep");
        command.arg("30");
        // SAFETY: the closure performs only async-signal-safe setpgid and
        // constructs an io::Error from thread-local errno before exec.
        unsafe {
            command.pre_exec(move || {
                if libc::setpgid(0, pgid) != 0 {
                    return Err(std::io::Error::last_os_error());
                }
                Ok(())
            });
        }
        command.spawn().expect("spawn isolated sleep")
    }

    fn wait_for_pgid(pid: u32, expected: i32) {
        let deadline = Instant::now() + Duration::from_secs(2);
        loop {
            // SAFETY: getpgid receives one positive disposable pid.
            let observed = unsafe { libc::getpgid(i32::try_from(pid).expect("pid_t")) };
            if observed == expected {
                return;
            }
            assert!(Instant::now() < deadline, "process group setup timed out");
            std::thread::sleep(Duration::from_millis(5));
        }
    }

    fn stop_success(pids: &[u32]) -> super::super::StoppedProcessGroupV8 {
        let group = observe_exact_process_group_v8(pids).expect("exact complete group");
        let plan = plan_stop_process_group_v8(group).expect("STOP-only plan");
        match execute_process_group_signal_v8(plan).expect("STOP execution") {
            ProcessGroupSignalSuccessV8::Stopped(stopped) => stopped,
            ProcessGroupSignalSuccessV8::Continued(_) => panic!("STOP returned CONT result"),
        }
    }

    #[test]
    fn exact_group_stop_and_token_owned_continue_use_fast_poll_then_final_hash() {
        let _serial = SIGNAL_FIXTURE_MUTEX.lock().expect("signal fixture mutex");
        let fixture = ProcessGroupFixture::spawn(2);
        let pids = fixture.pids();
        let group = observe_exact_process_group_v8(&pids).expect("exact group");
        assert_eq!(group.revalidate().expect("same-group second scan").len(), 2);
        let plan = plan_stop_process_group_v8(group).expect("STOP-only plan");
        reset_full_executable_hash_count_for_test_v8();
        let stopped = match execute_process_group_signal_v8(plan).expect("STOP") {
            ProcessGroupSignalSuccessV8::Stopped(stopped) => stopped,
            ProcessGroupSignalSuccessV8::Continued(_) => panic!("wrong success variant"),
        };
        // One full before snapshot and one final full snapshot; each exact
        // capture hashes the executable twice. Poll iterations add zero hashes.
        assert_eq!(full_executable_hash_count_for_test_v8(), pids.len() * 4);
        assert!(
            stopped
                .stop_execution()
                .after()
                .iter()
                .all(|identity| identity.state().is_stopped())
        );

        let continue_plan =
            plan_continue_stopped_process_group_v8(stopped).expect("token-owned CONT plan");
        match execute_process_group_signal_v8(continue_plan).expect("CONT") {
            ProcessGroupSignalSuccessV8::Continued(execution) => assert!(
                execution
                    .after()
                    .iter()
                    .all(|identity| !identity.state().is_stopped())
            ),
            ProcessGroupSignalSuccessV8::Stopped(_) => panic!("CONT returned STOP result"),
        }
    }

    #[test]
    fn duplicate_self_partial_and_mixed_groups_fail_before_signal() {
        let _serial = SIGNAL_FIXTURE_MUTEX.lock().expect("signal fixture mutex");
        assert!(validate_expected_group_inputs_for_test_v8(&[10, 10], 999).is_err());
        let self_pid = std::process::id();
        assert!(validate_expected_group_inputs_for_test_v8(&[self_pid, 10], self_pid).is_err());
        assert!(validate_expected_group_inputs_for_test_v8(&[1, 10], self_pid).is_err());

        let fixture = ProcessGroupFixture::spawn(3);
        let pids = fixture.pids();
        assert!(observe_exact_process_group_v8(&pids[..2]).is_err());

        let first = ProcessGroupFixture::spawn(2);
        let second = ProcessGroupFixture::spawn(2);
        assert!(observe_exact_process_group_v8(&[first.pids()[0], second.pids()[0]]).is_err());
    }

    #[test]
    fn post_stop_timeout_and_identity_faults_retain_unique_restore_obligation() {
        let _serial = SIGNAL_FIXTURE_MUTEX.lock().expect("signal fixture mutex");
        for fault in [
            SignalPostIssueTestFaultV8::Timeout,
            SignalPostIssueTestFaultV8::IdentityDrift,
        ] {
            let fixture = ProcessGroupFixture::spawn(2);
            let group = observe_exact_process_group_v8(&fixture.pids()).expect("exact group");
            let plan = plan_stop_process_group_v8(group).expect("STOP plan");
            let obligation = match execute_process_group_signal_with_fault_for_test_v8(plan, fault)
                .expect_err("injected post-STOP failure")
            {
                ProcessGroupSignalFailureV8::PostStop(obligation) => obligation,
                other => panic!("wrong failure variant: {other:?}"),
            };
            assert!(obligation.signal_issued());
            assert_eq!(obligation.member_identities().len(), 2);
            let restore =
                plan_restore_post_stop_v8(obligation).expect("obligation-owned recovery CONT plan");
            assert!(matches!(
                execute_process_group_signal_v8(restore).expect("recovery CONT"),
                ProcessGroupSignalSuccessV8::Continued(_)
            ));
        }
    }

    #[test]
    fn post_continue_failure_is_distinct_and_exposes_no_resend_path() {
        let _serial = SIGNAL_FIXTURE_MUTEX.lock().expect("signal fixture mutex");
        let fixture = ProcessGroupFixture::spawn(2);
        let stopped = stop_success(&fixture.pids());
        let continue_plan = plan_continue_stopped_process_group_v8(stopped).expect("CONT plan");
        let quarantine = match execute_process_group_signal_with_fault_for_test_v8(
            continue_plan,
            SignalPostIssueTestFaultV8::IdentityDrift,
        )
        .expect_err("injected post-CONT failure")
        {
            ProcessGroupSignalFailureV8::PostContinue(obligation) => obligation,
            other => panic!("wrong failure variant: {other:?}"),
        };
        assert!(quarantine.signal_issued());
        assert_eq!(
            quarantine
                .revalidate_for_quarantine()
                .expect("pidfds retained")
                .len(),
            2
        );
    }
}
