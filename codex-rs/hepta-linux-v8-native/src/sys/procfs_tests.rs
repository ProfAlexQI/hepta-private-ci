use super::ProcessStateV8;
use super::parse_fdinfo_mount_id_v8;
use super::parse_mountinfo_topology_v8;
use super::parse_proc_stat_v8;
use super::parse_process_status_credentials_v8;
use super::validate_process_cmdline_v8;

fn stat_fixture(comm: &str) -> Vec<u8> {
    format!("4242 ({comm}) S 41 4242 4242 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 987654 0\n").into_bytes()
}

#[test]
fn parses_exact_stat_with_spaces_and_parentheses_in_comm() {
    let parsed = parse_proc_stat_v8(&stat_fixture("a tricky ) ( comm"), Some(4242))
        .expect("strict proc stat");
    assert_eq!(parsed.pid, 4242);
    assert_eq!(parsed.parent_pid, 41);
    assert_eq!(parsed.process_group_id, 4242);
    assert_eq!(parsed.session_id, 4242);
    assert_eq!(parsed.state, ProcessStateV8::Sleeping);
    assert_eq!(parsed.start_ticks, 987654);
}

#[test]
fn rejects_pid_mismatch_unknown_state_and_noncanonical_fields() {
    assert!(parse_proc_stat_v8(&stat_fixture("name"), Some(7)).is_err());
    let unknown = String::from_utf8(stat_fixture("name"))
        .expect("ASCII fixture")
        .replacen(") S ", ") ? ", 1);
    assert!(parse_proc_stat_v8(unknown.as_bytes(), Some(4242)).is_err());
    let duplicate_space = String::from_utf8(stat_fixture("name"))
        .expect("ASCII fixture")
        .replacen(") S ", ") S  ", 1);
    assert!(parse_proc_stat_v8(duplicate_space.as_bytes(), Some(4242)).is_err());
}

#[cfg(not(target_os = "linux"))]
#[test]
fn process_observation_fails_closed_off_linux() {
    assert!(matches!(
        super::observe_process_exact_v8(2),
        Err(super::NativeSysErrorV8::UnsupportedPlatform(_))
    ));
}

#[cfg(target_os = "linux")]
#[test]
fn current_process_identity_is_pidfd_and_executable_bound() {
    let _serial = super::PROCESS_FD_LIFETIME_TEST_MUTEX
        .lock()
        .expect("process observation fixture mutex");
    let observation = super::observe_process_exact_v8(std::process::id())
        .expect("exact current process observation");
    let before = observation.identity().clone();
    let after = observation.revalidate().expect("exact full revalidation");
    assert!(before.has_same_stable_identity(&after));
    assert_eq!(after.pid(), std::process::id());
    assert!(after.start_ticks() > 0);
    assert!(after.executable().device() > 0);
    assert!(after.executable().inode() > 0);
    assert_ne!(after.executable().sha256(), [0; 32]);
}

#[cfg(target_os = "linux")]
#[test]
fn repeated_procfs_enumeration_uses_independent_directory_offsets() {
    let root = super::ProcfsRootV8::open_fixed().expect("fixed procfs root");
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(2);
    loop {
        let before = root.numeric_process_ids().expect("first independent scan");
        let after = root.numeric_process_ids().expect("second independent scan");
        assert!(before.contains(&std::process::id()));
        assert!(after.contains(&std::process::id()));
        if before == after {
            break;
        }
        assert!(
            std::time::Instant::now() < deadline,
            "could not obtain two stable consecutive procfs snapshots"
        );
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}

#[test]
fn rejects_embedded_newline_truncation_and_noncanonical_numeric_fields() {
    let newline_comm = stat_fixture("line\nbreak");
    assert!(parse_proc_stat_v8(&newline_comm, Some(4242)).is_err());
    assert!(parse_proc_stat_v8(b"4242 (short) S 1 2\n", Some(4242)).is_err());
    let leading_zero = String::from_utf8(stat_fixture("name"))
        .expect("ASCII fixture")
        .replacen(" S 41 ", " S 041 ", 1);
    assert!(parse_proc_stat_v8(leading_zero.as_bytes(), Some(4242)).is_err());
}

#[test]
fn mountinfo_parser_binds_unique_unshifted_proc_and_cgroup_mounts() {
    let fixture = b"24 1 0:20 / /proc rw,nosuid - proc proc rw\n\
                    31 1 0:27 / /sys/fs/cgroup rw - cgroup2 cgroup rw\n";
    let parsed = parse_mountinfo_topology_v8(fixture).expect("exact mount topology");
    assert_eq!(parsed.proc_mount_id, 24);
    assert_eq!(parsed.cgroup_mount_id, 31);

    let legal_nsfs = b"24 1 0:20 / /proc rw,nosuid - proc proc rw\n\
                       31 1 0:27 / /sys/fs/cgroup rw - cgroup2 cgroup rw\n\
                       41 1 0:4 mnt:[4026532601] /run/example.mnt rw - nsfs nsfs rw\n";
    assert_eq!(
        parse_mountinfo_topology_v8(legal_nsfs).expect("non-target nsfs special root"),
        parsed
    );

    let root_double_slash = fixture
        .windows(b"0:20 / /proc".len())
        .position(|window| window == b"0:20 / /proc")
        .expect("proc row");
    let mut root_double_slash_fixture = fixture.to_vec();
    root_double_slash_fixture.splice(
        root_double_slash..root_double_slash + b"0:20 / /proc".len(),
        b"0:20 // /proc".iter().copied(),
    );
    assert!(parse_mountinfo_topology_v8(&root_double_slash_fixture).is_err());

    let shifted = b"24 1 0:20 /shifted /proc rw - proc proc rw\n\
                    31 1 0:27 / /sys/fs/cgroup rw - cgroup2 cgroup rw\n";
    assert!(parse_mountinfo_topology_v8(shifted).is_err());

    let duplicate = b"24 1 0:20 / /proc rw - proc proc rw\n\
                      25 1 0:20 / /proc rw - proc proc rw\n\
                      31 1 0:27 / /sys/fs/cgroup rw - cgroup2 cgroup rw\n";
    assert!(parse_mountinfo_topology_v8(duplicate).is_err());

    let escaped_alias = b"24 1 0:20 / /proc\\057alias rw - proc proc rw\n\
                          31 1 0:27 / /sys/fs/cgroup rw - cgroup2 cgroup rw\n";
    assert!(parse_mountinfo_topology_v8(escaped_alias).is_err());

    let invalid_nsfs_escape = b"24 1 0:20 / /proc rw - proc proc rw\n\
                                 31 1 0:27 / /sys/fs/cgroup rw - cgroup2 cgroup rw\n\
                                 41 1 0:4 mnt:\\057bad /run/example.mnt rw - nsfs nsfs rw\n";
    assert!(parse_mountinfo_topology_v8(invalid_nsfs_escape).is_err());
}

#[test]
fn fdinfo_mount_id_parser_rejects_missing_duplicate_and_noncanonical_values() {
    assert_eq!(
        parse_fdinfo_mount_id_v8(b"pos:\t0\nflags:\t02000000\nmnt_id:\t42\n")
            .expect("exact fdinfo mount id"),
        42
    );
    assert!(parse_fdinfo_mount_id_v8(b"pos:\t0\n").is_err());
    assert!(parse_fdinfo_mount_id_v8(b"mnt_id:\t1\nmnt_id:\t2\n").is_err());
    assert!(parse_fdinfo_mount_id_v8(b"mnt_id:\t042\n").is_err());
}

#[test]
fn process_credentials_and_cmdline_are_canonical_and_complete() {
    assert_eq!(
        parse_process_status_credentials_v8(
            b"Name:\trunner\nUid:\t1000\t1000\t1000\t1000\nGid:\t1001\t1001\t1001\t1001\n"
        )
        .expect("exact four-way credentials"),
        (1000, 1001)
    );
    assert!(
        parse_process_status_credentials_v8(
            b"Uid:\t1000\t1001\t1000\t1000\nGid:\t1001\t1001\t1001\t1001\n"
        )
        .is_err()
    );
    assert!(
        parse_process_status_credentials_v8(
            b"Uid:\t01000\t01000\t01000\t01000\nGid:\t1001\t1001\t1001\t1001\n"
        )
        .is_err()
    );
    assert!(
        parse_process_status_credentials_v8(
            b"Uid:\t1000\t1000\t1000\t1000\nUid:\t1000\t1000\t1000\t1000\nGid:\t1001\t1001\t1001\t1001\n"
        )
        .is_err()
    );
    assert!(parse_process_status_credentials_v8(b"Uid:\t1000\t1000\t1000\t1000\n").is_err());

    validate_process_cmdline_v8(b"/bin/runner\0--flag\0").expect("canonical process argv");
    assert!(validate_process_cmdline_v8(b"").is_err());
    assert!(validate_process_cmdline_v8(b"\0--flag\0").is_err());
    assert!(validate_process_cmdline_v8(b"/bin/runner").is_err());
}

#[test]
fn detached_workload_selector_is_canonical_and_non_root() {
    assert!(
        super::DetachedWorkloadSelectorV8::new("/system.slice/build.service".to_string(), 42, 42,)
            .is_ok()
    );
    for path in [
        "/",
        "relative",
        "/double//slash",
        "/dot/../escape",
        "/tail/",
    ] {
        assert!(
            super::DetachedWorkloadSelectorV8::new(path.to_string(), 42, 42).is_err(),
            "accepted malformed selector path {path}"
        );
    }
    assert!(
        super::DetachedWorkloadSelectorV8::new("/system.slice/build.service".to_string(), 1, 42,)
            .is_err()
    );
}

#[cfg(target_os = "linux")]
#[test]
fn retained_procfs_root_binds_numeric_observer_mounts_and_namespaces() {
    let root = super::ProcfsRootV8::open_fixed().expect("fixed procfs root");
    let binding = root.observer_binding().expect("numeric observer binding");
    assert_eq!(binding.observer_pid, std::process::id());
    // SAFETY: gettid has no pointer arguments or preconditions.
    assert_eq!(binding.observer_tid, unsafe { libc::gettid() } as u32);
    assert_ne!(binding.proc_mount_id, binding.cgroup_mount_id);
    assert!(binding.pid_namespace_device > 0);
    assert!(binding.pid_namespace_inode > 0);
    assert!(binding.cgroup_namespace_device > 0);
    assert!(binding.cgroup_namespace_inode > 0);
    assert!(binding.mount_namespace_device > 0);
    assert!(binding.mount_namespace_inode > 0);
}

#[cfg(target_os = "linux")]
#[test]
fn global_detached_workload_scan_binds_pgid_sid_cgroup_and_proc_identity() {
    use std::os::unix::process::CommandExt as _;
    use std::process::Command;

    let mut command = Command::new("/bin/sleep");
    command.arg("30");
    // SAFETY: setsid is async-signal-safe and has no pointer arguments.
    unsafe {
        command.pre_exec(|| {
            if libc::setsid() < 0 {
                return Err(std::io::Error::last_os_error());
            }
            Ok(())
        });
    }
    let mut child = command.spawn().expect("spawn detached-workload fixture");
    let selector = super::DetachedWorkloadSelectorV8::new(
        "/hepta-v8-test/nonexistent-cgroup".to_string(),
        child.id(),
        child.id(),
    )
    .expect("exact selector");
    let closure = super::observe_detached_workload_closure_v8(selector.clone())
        .expect("observe exact detached-workload closure");
    assert_eq!(closure.processes().len(), 1);
    assert_eq!(closure.processes()[0].identity().pid(), child.id());
    assert_ne!(closure.binding_sha256(), [0; 32]);
    closure.revalidate().expect("stable global closure");

    child.kill().expect("kill disposable fixture");
    child.wait().expect("reap disposable fixture");
    let empty = super::observe_detached_workload_closure_v8(selector)
        .expect("observe terminal absence closure");
    assert!(empty.processes().is_empty());
    assert_ne!(empty.binding_sha256(), closure.binding_sha256());
}
