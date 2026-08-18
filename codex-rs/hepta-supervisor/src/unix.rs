use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::net::Shutdown;
use std::path::PathBuf;
use std::process::Child;
use std::process::Command;
use std::process::Stdio;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::SyncSender;
use std::sync::mpsc::TryRecvError;
use std::time::Duration;

use codex_hepta_agent_protocol::AGENTD_CONTROL_SCHEMA_VERSION;
use codex_hepta_agent_protocol::AgentdPayload;
use codex_hepta_agent_protocol::AgentdRequest;
use codex_hepta_agent_protocol::AgentdResponse;
use codex_hepta_agent_protocol::MAX_CONTROL_FRAME_BYTES;
use codex_hepta_contracts::AgentId;

use crate::AdoptSpec;
use crate::Adoption;
use crate::ManagedProcess;
use crate::ProcessDriver;
use crate::ProcessDriverError;
use crate::ProcessExit;
use crate::ProcessIdentity;
use crate::ProcessLog;
use crate::ProcessObservation;
use crate::ProcessState;
use crate::ProcessStream;
use crate::SpawnSpec;
use crate::driver::SpawnedProcess;

const LOG_CHUNK_BYTES: usize = 4_096;
const HEALTH_PROBE_INTERVAL: Duration = Duration::from_millis(50);
const HEALTH_PROBE_IO_TIMEOUT: Duration = Duration::from_millis(200);

/// Unix child wrapper with non-blocking polling and bounded per-child log channels.
pub struct UnixProcessDriver {
    log_channel_capacity: usize,
}

impl UnixProcessDriver {
    pub fn new(log_channel_capacity: usize) -> Result<Self, ProcessDriverError> {
        if !(1..=16_384).contains(&log_channel_capacity) {
            return Err(ProcessDriverError::new(
                "Unix child log channel capacity must be between 1 and 16384",
            ));
        }
        Ok(Self {
            log_channel_capacity,
        })
    }
}

pub struct UnixManagedProcess {
    child: Child,
    logs: Receiver<ProcessLog>,
    health_probe: HealthProbe,
}

impl ManagedProcess for UnixManagedProcess {
    fn poll(&mut self, max_logs: usize) -> Result<ProcessObservation, ProcessDriverError> {
        let mut logs = Vec::with_capacity(max_logs);
        for _ in 0..max_logs {
            match self.logs.try_recv() {
                Ok(log) => logs.push(log),
                Err(TryRecvError::Empty | TryRecvError::Disconnected) => break,
            }
        }
        let state = match self.child.try_wait()? {
            Some(status) => {
                self.health_probe.shutdown();
                ProcessState::Exited(ProcessExit {
                    success: status.success(),
                    code: status.code(),
                })
            }
            None => ProcessState::Running {
                healthy: self.health_probe.ready(),
                drained: false,
            },
        };
        Ok(ProcessObservation { state, logs })
    }

    fn request_drain(&mut self) -> Result<(), ProcessDriverError> {
        send_signal(self.child.id(), libc::SIGTERM)
    }

    fn request_stop(&mut self) -> Result<(), ProcessDriverError> {
        send_signal(self.child.id(), libc::SIGTERM)
    }

    fn kill(&mut self) -> Result<(), ProcessDriverError> {
        self.child.kill().map_err(Into::into)
    }
}

impl ProcessDriver for UnixProcessDriver {
    type Process = UnixManagedProcess;

    fn spawn(
        &mut self,
        spec: &SpawnSpec,
    ) -> Result<SpawnedProcess<Self::Process>, ProcessDriverError> {
        let mut command = Command::new(&spec.command.program);
        command
            .args(&spec.command.args)
            .current_dir(&spec.workspace)
            .env("CODEX_HOME", &spec.home_root)
            .env("HEPTA_AGENT_ID", spec.agent_id.to_string())
            .env("HEPTA_AGENT_GENERATION", spec.generation.to_string())
            .env("HEPTA_FLEET_ROOT", &spec.fleet_root)
            .env("HEPTA_AGENT_HOME", &spec.home_root)
            .env("HEPTA_AGENT_RUN_ROOT", &spec.run_root)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        let mut child = command.spawn()?;
        let health_probe = match HealthProbe::spawn(spec, child.id()) {
            Ok(probe) => probe,
            Err(error) => {
                let _ = child.kill();
                return Err(error);
            }
        };
        let Some(stdout) = child.stdout.take() else {
            let _ = child.kill();
            return Err(ProcessDriverError::new("child stdout pipe is missing"));
        };
        let Some(stderr) = child.stderr.take() else {
            let _ = child.kill();
            return Err(ProcessDriverError::new("child stderr pipe is missing"));
        };
        let (sender, logs) = std::sync::mpsc::sync_channel(self.log_channel_capacity);
        spawn_log_reader(stdout, ProcessStream::Stdout, sender.clone());
        spawn_log_reader(stderr, ProcessStream::Stderr, sender);
        let identity = ProcessIdentity::new(
            u64::from(child.id()),
            format!("unix-pid-{}-generation-{}", child.id(), spec.generation),
        )
        .map_err(|error| ProcessDriverError::new(error.to_string()))?;
        Ok(SpawnedProcess {
            identity,
            process: UnixManagedProcess {
                child,
                logs,
                health_probe,
            },
        })
    }

    fn adopt(&mut self, spec: &AdoptSpec) -> Result<Adoption<Self::Process>, ProcessDriverError> {
        // std::process cannot safely recover wait ownership or prove PID incarnation after a
        // supervisor restart. Refuse adoption until agentd provides a signed UDS handshake.
        if process_exists(spec.identity.system_id())? {
            Ok(Adoption::Rejected)
        } else {
            Ok(Adoption::Missing)
        }
    }
}

struct HealthProbe {
    ready: Arc<AtomicBool>,
    shutdown: Arc<AtomicBool>,
}

impl HealthProbe {
    fn spawn(spec: &SpawnSpec, process_id: u32) -> Result<Self, ProcessDriverError> {
        let identity = HealthProbeIdentity {
            agent_id: spec.agent_id.clone(),
            spawn_generation: spec.generation,
            process_id,
            workspace: spec.workspace.clone(),
            home_root: spec.home_root.clone(),
            run_root: spec.run_root.clone(),
            control_socket: spec.control_socket.clone(),
        };
        let ready = Arc::new(AtomicBool::new(false));
        let shutdown = Arc::new(AtomicBool::new(false));
        let worker_ready = Arc::clone(&ready);
        let worker_shutdown = Arc::clone(&shutdown);
        std::thread::Builder::new()
            .name(format!("hepta-health-{}", identity.agent_id))
            .spawn(move || run_health_probe(identity, worker_ready, worker_shutdown))
            .map_err(ProcessDriverError::from)?;
        Ok(Self { ready, shutdown })
    }

    fn ready(&self) -> bool {
        self.ready.load(Ordering::Acquire)
    }

    fn shutdown(&self) {
        self.shutdown.store(true, Ordering::Release);
        self.ready.store(false, Ordering::Release);
    }
}

impl Drop for HealthProbe {
    fn drop(&mut self) {
        self.shutdown();
    }
}

struct HealthProbeIdentity {
    agent_id: AgentId,
    spawn_generation: u64,
    process_id: u32,
    workspace: PathBuf,
    home_root: PathBuf,
    run_root: PathBuf,
    control_socket: PathBuf,
}

fn run_health_probe(
    identity: HealthProbeIdentity,
    ready: Arc<AtomicBool>,
    shutdown: Arc<AtomicBool>,
) {
    let mut request_id = 1_u64;
    while !shutdown.load(Ordering::Acquire) {
        ready.store(
            probe_health_once(&identity, request_id).unwrap_or(false),
            Ordering::Release,
        );
        request_id = request_id.wrapping_add(1).max(1);
        std::thread::sleep(HEALTH_PROBE_INTERVAL);
    }
    ready.store(false, Ordering::Release);
}

fn probe_health_once(
    identity: &HealthProbeIdentity,
    request_id: u64,
) -> Result<bool, ProcessDriverError> {
    let request = AgentdRequest::health(request_id, identity.spawn_generation);
    let mut bytes = serde_json::to_vec(&request)?;
    bytes.push(b'\n');
    if bytes.len() as u64 > MAX_CONTROL_FRAME_BYTES {
        return Ok(false);
    }

    let mut stream = std::os::unix::net::UnixStream::connect(&identity.control_socket)?;
    stream.set_read_timeout(Some(HEALTH_PROBE_IO_TIMEOUT))?;
    stream.set_write_timeout(Some(HEALTH_PROBE_IO_TIMEOUT))?;
    stream.write_all(&bytes)?;
    stream.shutdown(Shutdown::Write)?;

    let mut reader = BufReader::new(stream).take(MAX_CONTROL_FRAME_BYTES + 1);
    let mut response_bytes = Vec::new();
    let count = reader.read_until(b'\n', &mut response_bytes)?;
    if count == 0 || count as u64 > MAX_CONTROL_FRAME_BYTES || !response_bytes.ends_with(b"\n") {
        return Ok(false);
    }
    let response: AgentdResponse = serde_json::from_slice(&response_bytes)?;
    let exact_envelope = response.schema_version == AGENTD_CONTROL_SCHEMA_VERSION
        && response.request_id == request_id
        && response.agent_id == identity.agent_id
        && response.spawn_generation == identity.spawn_generation
        && response.current_generation == identity.spawn_generation;
    let AgentdPayload::Health(health) = response.payload else {
        return Ok(false);
    };
    Ok(exact_envelope
        && health.promotion_ready
        && !health.fenced
        && health.process_id == identity.process_id
        && health.workspace == identity.workspace
        && health.home_root == identity.home_root
        && health.run_root == identity.run_root)
}

fn spawn_log_reader(
    mut reader: impl Read + Send + 'static,
    stream: ProcessStream,
    sender: SyncSender<ProcessLog>,
) {
    std::thread::spawn(move || {
        let mut buffer = [0_u8; LOG_CHUNK_BYTES];
        loop {
            let count = match reader.read(&mut buffer) {
                Ok(0) | Err(_) => break,
                Ok(count) => count,
            };
            let _ = sender.try_send(ProcessLog {
                stream,
                bytes: buffer[..count].to_vec(),
            });
        }
    });
}

fn send_signal(pid: u32, signal: i32) -> Result<(), ProcessDriverError> {
    let pid = i32::try_from(pid)
        .map_err(|_| ProcessDriverError::new("child PID does not fit Unix pid_t"))?;
    // SAFETY: `pid` comes from std::process::Child and `signal` is a fixed libc signal constant.
    if unsafe { libc::kill(pid, signal) } == -1 {
        Err(std::io::Error::last_os_error().into())
    } else {
        Ok(())
    }
}

fn process_exists(system_id: u64) -> Result<bool, ProcessDriverError> {
    let pid = i32::try_from(system_id)
        .map_err(|_| ProcessDriverError::new("stored child PID does not fit Unix pid_t"))?;
    // SAFETY: signal 0 performs an existence/permission check without delivering a signal.
    if unsafe { libc::kill(pid, 0) } == 0 {
        return Ok(true);
    }
    let error = std::io::Error::last_os_error();
    if error.raw_os_error() == Some(libc::ESRCH) {
        Ok(false)
    } else if error.raw_os_error() == Some(libc::EPERM) {
        Ok(true)
    } else {
        Err(error.into())
    }
}

#[cfg(test)]
#[path = "unix_tests.rs"]
mod tests;
