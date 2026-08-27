use std::error::Error;
use std::ffi::OsStr;
use std::io::Read;
use std::io::Write;
use std::process::Child;
use std::process::Command;
use std::process::ExitStatus;
use std::process::Stdio;
use std::thread;
use std::time::Duration;
use std::time::Instant;

use hepta_browser_c1_artifact_gate_qualification::GateError;
use hepta_browser_c1_artifact_gate_qualification::HostAck;
use hepta_browser_c1_artifact_gate_qualification::LaunchHello;
use hepta_browser_c1_artifact_gate_qualification::Message;
use hepta_browser_c1_artifact_gate_qualification::WorkerConfirm;
use hepta_browser_c1_artifact_gate_qualification::binding_for_current_executable;
use hepta_browser_c1_artifact_gate_qualification::read_message;
use hepta_browser_c1_artifact_gate_qualification::validate_worker_hello;
use hepta_browser_c1_artifact_gate_qualification::write_message;

const BUILD_MANIFEST: &[u8] = include_bytes!("../../fixtures/qualification-build-manifest.json");
const SOURCE_RECEIPT: &[u8] = include_bytes!("../../fixtures/qualification-source-receipt.json");
const IO_TIMEOUT: Duration = Duration::from_secs(2);
const REAP_TIMEOUT: Duration = Duration::from_secs(2);
const POLL_INTERVAL: Duration = Duration::from_millis(10);

fn main() -> Result<(), Box<dyn Error>> {
    let arguments = std::env::args_os().skip(1).collect::<Vec<_>>();
    match arguments.as_slice() {
        [] => run_host(HostMode::Normal),
        [mode] if mode == OsStr::new("--force-kill-trial") => run_host(HostMode::ForceKill),
        [mode] if mode == OsStr::new("--worker") => run_worker(false),
        [mode] if mode == OsStr::new("--worker-hang") => run_worker(true),
        _ => Err(std::io::Error::other("unsupported artifact-bound trial arguments").into()),
    }
}

#[derive(Clone, Copy)]
enum HostMode {
    Normal,
    ForceKill,
}

#[cfg(unix)]
fn run_host(mode: HostMode) -> Result<(), Box<dyn Error>> {
    use std::os::fd::OwnedFd;
    use std::os::unix::net::UnixStream;

    let executable = std::env::current_exe()?;
    let expected_binding = binding_for_current_executable(BUILD_MANIFEST, SOURCE_RECEIPT)?;
    let mut challenge = [0_u8; 32];
    fill_private_random(&mut challenge)?;

    let (mut host_writer, child_stdin) = UnixStream::pair()?;
    let (child_stdout, mut host_reader) = UnixStream::pair()?;
    host_writer.set_write_timeout(Some(IO_TIMEOUT))?;
    host_reader.set_read_timeout(Some(IO_TIMEOUT))?;

    let child_stdin_fd: OwnedFd = child_stdin.into();
    let child_stdout_fd: OwnedFd = child_stdout.into();
    let child_mode = match mode {
        HostMode::Normal => "--worker",
        HostMode::ForceKill => "--worker-hang",
    };
    let child = Command::new(executable)
        .arg(child_mode)
        .stdin(Stdio::from(child_stdin_fd))
        .stdout(Stdio::from(child_stdout_fd))
        .stderr(Stdio::inherit())
        .spawn()?;
    let expected_pid = child.id();
    let mut guard = ChildGuard::new(child);

    host_writer.write_all(&challenge)?;
    host_writer.flush()?;

    let hello = match read_message(&mut host_reader)? {
        Message::WorkerHello(hello) => hello,
        _ => return Err(GateError::Invalid("host expected worker hello").into()),
    };
    validate_worker_hello(&hello, expected_pid, expected_binding, &challenge)?;
    write_message(&mut host_writer, &Message::HostAck(HostAck::new(challenge)?))?;
    let confirm = match read_message(&mut host_reader)? {
        Message::WorkerConfirm(confirm) => confirm,
        _ => return Err(GateError::Invalid("host expected worker confirmation").into()),
    };
    if !confirm.challenge_matches(&challenge) {
        return Err(GateError::AuthenticationFailed.into());
    }
    challenge.fill(0);

    write_message(&mut host_writer, &Message::Ping)?;
    match mode {
        HostMode::Normal => {
            if read_message(&mut host_reader)? != Message::Pong {
                return Err(GateError::Invalid("host expected exact pong").into());
            }
            write_message(&mut host_writer, &Message::Shutdown)?;
            if read_message(&mut host_reader)? != Message::ShutdownAck {
                return Err(GateError::Invalid("host expected exact shutdown acknowledgement").into());
            }
            drop(host_writer);
            drop(host_reader);
            let status = guard.wait_bounded(REAP_TIMEOUT)?;
            if !status.success() {
                return Err(GateError::ChildExit(status.to_string()).into());
            }
            guard.disarm();
            println!(
                "{{\"artifact_binding\":true,\"build_manifest_binding\":true,\"deadlines\":true,\"external_network\":false,\"forced_kill\":false,\"runtime_authority\":false,\"servo_linked\":false,\"source_receipt_binding\":true,\"status\":\"ARTIFACT_BOUND_QUALIFICATION_TRIAL_PASS\"}}"
            );
            Ok(())
        }
        HostMode::ForceKill => {
            let timeout_observed = match read_message(&mut host_reader) {
                Err(GateError::Io(error))
                    if matches!(
                        error.kind(),
                        std::io::ErrorKind::TimedOut | std::io::ErrorKind::WouldBlock
                    ) => true,
                Err(error) => return Err(error.into()),
                Ok(_) => false,
            };
            if !timeout_observed {
                return Err(GateError::Invalid("hung worker did not trigger a read deadline").into());
            }
            guard.kill()?;
            let status = guard.wait_bounded(REAP_TIMEOUT)?;
            guard.disarm();
            if status.success() {
                return Err(GateError::Invalid("forced-kill worker exited successfully").into());
            }
            println!(
                "{{\"artifact_binding\":true,\"deadline_observed\":true,\"external_network\":false,\"forced_kill\":true,\"reaped\":true,\"runtime_authority\":false,\"servo_linked\":false,\"status\":\"ARTIFACT_BOUND_FORCE_KILL_REAP_PASS\"}}"
            );
            Ok(())
        }
    }
}

#[cfg(not(unix))]
fn run_host(_mode: HostMode) -> Result<(), Box<dyn Error>> {
    Err(std::io::Error::new(
        std::io::ErrorKind::Unsupported,
        "artifact-bound process trial currently requires Unix socketpair transport",
    )
    .into())
}

fn run_worker(hang_after_ping: bool) -> Result<(), Box<dyn Error>> {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut reader = stdin.lock();
    let mut writer = stdout.lock();

    let mut challenge = [0_u8; 32];
    reader.read_exact(&mut challenge)?;
    let binding = binding_for_current_executable(BUILD_MANIFEST, SOURCE_RECEIPT)?;
    write_message(
        &mut writer,
        &Message::WorkerHello(LaunchHello::new(std::process::id(), binding, challenge)?),
    )?;
    let acknowledgement = match read_message(&mut reader)? {
        Message::HostAck(acknowledgement) => acknowledgement,
        _ => return Err(GateError::Invalid("worker expected host acknowledgement").into()),
    };
    if !acknowledgement.challenge_matches(&challenge) {
        return Err(GateError::AuthenticationFailed.into());
    }
    write_message(
        &mut writer,
        &Message::WorkerConfirm(WorkerConfirm::new(challenge)?),
    )?;
    challenge.fill(0);

    loop {
        match read_message(&mut reader)? {
            Message::Ping if hang_after_ping => loop {
                thread::sleep(Duration::from_secs(60));
            },
            Message::Ping => write_message(&mut writer, &Message::Pong)?,
            Message::Shutdown => {
                write_message(&mut writer, &Message::ShutdownAck)?;
                return Ok(());
            }
            _ => return Err(GateError::Invalid("worker received an unexpected established message").into()),
        }
    }
}

#[cfg(unix)]
fn fill_private_random(output: &mut [u8]) -> Result<(), Box<dyn Error>> {
    let mut random = std::fs::File::open("/dev/urandom")?;
    random.read_exact(output)?;
    Ok(())
}

struct ChildGuard {
    child: Option<Child>,
}

impl ChildGuard {
    fn new(child: Child) -> Self {
        Self { child: Some(child) }
    }

    fn child_mut(&mut self) -> Result<&mut Child, GateError> {
        self.child
            .as_mut()
            .ok_or(GateError::Invalid("child guard is disarmed"))
    }

    fn kill(&mut self) -> Result<(), GateError> {
        match self.child_mut()?.kill() {
            Ok(()) => Ok(()),
            Err(error) if error.kind() == std::io::ErrorKind::InvalidInput => Ok(()),
            Err(error) => Err(error.into()),
        }
    }

    fn wait_bounded(&mut self, duration: Duration) -> Result<ExitStatus, GateError> {
        let deadline = Instant::now() + duration;
        loop {
            if let Some(status) = self.child_mut()?.try_wait()? {
                return Ok(status);
            }
            if Instant::now() >= deadline {
                self.kill()?;
                let kill_deadline = Instant::now() + duration;
                loop {
                    if let Some(status) = self.child_mut()?.try_wait()? {
                        return Ok(status);
                    }
                    if Instant::now() >= kill_deadline {
                        return Err(GateError::DeadlineExceeded);
                    }
                    thread::sleep(POLL_INTERVAL);
                }
            }
            thread::sleep(POLL_INTERVAL);
        }
    }

    fn disarm(&mut self) {
        self.child.take();
    }
}

impl Drop for ChildGuard {
    fn drop(&mut self) {
        if let Some(child) = self.child.as_mut() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}
