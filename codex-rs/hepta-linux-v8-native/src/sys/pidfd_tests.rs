use super::pidfd::parse_pidfd_fdinfo_pid;
use super::pidfd::parse_proc_start_ticks;
use super::pidfd_open_verified;

#[test]
fn opens_current_process_without_signalling_it() {
    let pid = std::process::id();
    let handle = pidfd_open_verified(pid).expect("open current process pidfd");

    assert_eq!(handle.pid(), pid);
    assert!(handle.start_ticks() > 0);
    assert!(!handle.is_exited().expect("poll current process pidfd"));
}

#[test]
fn rejects_zero_pid() {
    let error = pidfd_open_verified(0).expect_err("pid zero must fail closed");
    assert!(error.to_string().contains("positive pid_t"));
}

#[test]
fn parses_comm_with_spaces_and_parentheses_from_the_end() {
    let fields_3_through_21 = [
        "S", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16",
        "17", "18",
    ]
    .join(" ");
    let stat = format!("123 (a process (name)) {fields_3_through_21} 424242 0 0");

    assert_eq!(
        parse_proc_start_ticks(&stat).expect("parse start ticks"),
        424242
    );
}

#[test]
fn rejects_truncated_proc_stat() {
    let error =
        parse_proc_start_ticks("123 (short) S 1 2").expect_err("truncated stat must fail closed");
    assert!(error.to_string().contains("field 22"));
}

#[test]
fn parses_exact_live_pidfd_fdinfo_pid() {
    let fixture = "pos:\t0\nflags:\t02000002\nmnt_id:\t5\nino:\t123\nPid:\t4242\nNSpid:\t4242\n";
    assert_eq!(
        parse_pidfd_fdinfo_pid(fixture).expect("parse live pidfd target"),
        4242
    );
}

#[test]
fn rejects_exited_or_ambiguous_pidfd_fdinfo() {
    assert!(parse_pidfd_fdinfo_pid("Pid:\t-1\nNSpid:\t-1\n").is_err());
    assert!(parse_pidfd_fdinfo_pid("Pid:\t1\nPid:\t2\n").is_err());
    assert!(parse_pidfd_fdinfo_pid("NSpid:\t1\n").is_err());
}
