use std::ffi::OsString;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::os::unix::net::UnixListener;
use std::path::Path;
use std::time::Duration;
use std::time::Instant;

use codex_hepta_agent_protocol::AGENTD_CONTROL_SCHEMA_VERSION;
use codex_hepta_agent_protocol::AgentdPayload;
use codex_hepta_agent_protocol::AgentdRequest;
use codex_hepta_agent_protocol::AgentdResponse;
use codex_hepta_agent_protocol::HealthSnapshot;
use codex_hepta_contracts::AgentId;
use pretty_assertions::assert_eq;

use super::HealthProbeIdentity;
use super::UnixProcessDriver;
use super::probe_health_once;
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
        fleet_root: temp.path().join("fleet"),
        workspace: temp.path().to_path_buf(),
        home_root: temp.path().join("home"),
        run_root: temp.path().join("run"),
        control_socket: temp.path().join("run/agentd-control.sock"),
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

#[test]
fn health_probe_requires_exact_agent_generation_pid_and_roots() {
    let temp = tempfile::tempdir().expect("temporary directory");
    let expected_agent = AgentId::parse("018f4f72-5f8f-7cc1-8f55-df9fb3aa2c12").expect("agent id");
    let other_agent =
        AgentId::parse("019153a4-3088-7e03-a56a-9b1964f75dd3").expect("other agent id");
    let identity = HealthProbeIdentity {
        agent_id: expected_agent.clone(),
        spawn_generation: 7,
        process_id: 41,
        workspace: temp.path().join("workspace"),
        home_root: temp.path().join("home"),
        run_root: temp.path().join("run"),
        control_socket: temp.path().join("probe.sock"),
    };

    let exact = health_response(&identity, expected_agent.clone(), 7, 7, 41);
    assert!(serve_and_probe(&identity, 1, exact));

    let wrong_agent = health_response(&identity, other_agent, 7, 7, 41);
    assert!(!serve_and_probe(&identity, 2, wrong_agent));

    let wrong_generation = health_response(&identity, expected_agent.clone(), 7, 8, 41);
    assert!(!serve_and_probe(&identity, 3, wrong_generation));

    let wrong_pid = health_response(&identity, expected_agent, 7, 7, 42);
    assert!(!serve_and_probe(&identity, 4, wrong_pid));
}

fn health_response(
    identity: &HealthProbeIdentity,
    agent_id: AgentId,
    spawn_generation: u64,
    current_generation: u64,
    process_id: u32,
) -> AgentdResponse {
    AgentdResponse {
        schema_version: AGENTD_CONTROL_SCHEMA_VERSION,
        request_id: 0,
        agent_id,
        spawn_generation,
        current_generation,
        payload: AgentdPayload::Health(HealthSnapshot {
            promotion_ready: true,
            ready: false,
            fenced: false,
            process_id,
            workspace: identity.workspace.clone(),
            home_root: identity.home_root.clone(),
            run_root: identity.run_root.clone(),
        }),
    }
}

fn serve_and_probe(
    identity: &HealthProbeIdentity,
    request_id: u64,
    mut response: AgentdResponse,
) -> bool {
    remove_socket(&identity.control_socket);
    let listener = UnixListener::bind(&identity.control_socket).expect("bind probe socket");
    let worker = std::thread::spawn(move || {
        let (stream, _) = listener.accept().expect("accept probe");
        let mut reader = BufReader::new(stream);
        let mut request_bytes = Vec::new();
        reader
            .read_until(b'\n', &mut request_bytes)
            .expect("read request");
        let request: AgentdRequest =
            serde_json::from_slice(&request_bytes).expect("typed health request");
        response.request_id = request.request_id;
        let mut stream = reader.into_inner();
        serde_json::to_writer(&mut stream, &response).expect("write response");
        stream.write_all(b"\n").expect("terminate response");
    });
    let result = probe_health_once(identity, request_id).expect("health probe completes");
    worker.join().expect("probe server joins");
    remove_socket(&identity.control_socket);
    result
}

fn remove_socket(socket: &Path) {
    if let Err(error) = std::fs::remove_file(socket)
        && error.kind() != std::io::ErrorKind::NotFound
    {
        panic!("remove {}: {error}", socket.display());
    }
}
