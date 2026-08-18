use std::ffi::OsString;
use std::time::Duration;
use std::time::Instant;

use codex_hepta_contracts::AgentId;
use pretty_assertions::assert_eq;

use super::UnixProcessDriver;
use crate::AgentCommand;
use crate::ManagedProcess;
use crate::ProcessDriver;
use crate::ProcessState;
use crate::ProcessStream;
use crate::SpawnSpec;

#[test]
fn unix_wrapper_captures_bounded_stdout_and_stderr() {
    let temp = tempfile::tempdir().expect("temporary directory");
    let command = AgentCommand::new(
        "/bin/sh",
        vec![
            OsString::from("-c"),
            OsString::from("printf stdout; printf stderr >&2"),
        ],
    )
    .expect("valid command");
    let spec = SpawnSpec {
        agent_id: AgentId::parse("018f4f72-5f8f-7cc1-8f55-df9fb3aa2c12").expect("valid agent id"),
        generation: 1,
        workspace: temp.path().to_path_buf(),
        home_root: temp.path().join("home"),
        run_root: temp.path().join("run"),
        logs_root: temp.path().join("logs"),
        command,
    };
    let mut process = UnixProcessDriver::new(8)
        .expect("valid driver")
        .spawn(&spec)
        .expect("spawn child")
        .process;
    let deadline = Instant::now() + Duration::from_secs(2);
    let mut logs = Vec::new();
    loop {
        let observation = process.poll(8).expect("poll child");
        logs.extend(observation.logs);
        if matches!(observation.state, ProcessState::Exited(_)) && logs.len() >= 2 {
            break;
        }
        assert!(Instant::now() < deadline, "child did not exit");
        std::thread::yield_now();
    }
    let streams: Vec<_> = logs.iter().map(|log| log.stream).collect();
    assert_eq!(streams.len(), 2);
    assert!(streams.contains(&ProcessStream::Stdout));
    assert!(streams.contains(&ProcessStream::Stderr));
}
