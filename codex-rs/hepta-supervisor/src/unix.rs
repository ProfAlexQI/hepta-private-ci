use std::io::Read;
use std::process::Child;
use std::process::Command;
use std::process::Stdio;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::SyncSender;
use std::sync::mpsc::TryRecvError;

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
            Some(status) => ProcessState::Exited(ProcessExit {
                success: status.success(),
                code: status.code(),
            }),
            None => ProcessState::Running {
                healthy: true,
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
            .env("HEPTA_AGENT_ID", spec.agent_id.to_string())
            .env("HEPTA_AGENT_GENERATION", spec.generation.to_string())
            .env("HEPTA_AGENT_HOME", &spec.home_root)
            .env("HEPTA_AGENT_RUN_ROOT", &spec.run_root)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        let mut child = command.spawn()?;
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
            process: UnixManagedProcess { child, logs },
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
