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

use hepta_browser_c1_artifact_gate_qualification as artifact;
use hepta_browser_worker_protocol_qualification as browser;

const SERVO_COMMIT: &str = "0a48e298482659817eb50097df23841f2b8e3044";
const SERVO_TREE: &str = "b04d2f75b3217374d079d579c270177b57fa1389";
const BUILD_MANIFEST: &[u8] = include_bytes!("../../fixtures/qualification-build-manifest.json");
const SOURCE_RECEIPT: &[u8] = include_bytes!("../../fixtures/qualification-source-receipt.json");
const SESSION_BYTES: [u8; 32] = [0x93; 32];
const GENERATION: u64 = 1;
const OWNER_EPOCH: u64 = 1;
const ARTIFACT_CHALLENGE_BYTES: usize = 32;
const BROWSER_BOOTSTRAP_BYTES: usize = 64;
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
        _ => Err(std::io::Error::other("unsupported C1 startup-bridge arguments").into()),
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
    let expected_artifact_binding =
        artifact::binding_for_current_executable(BUILD_MANIFEST, SOURCE_RECEIPT)?;

    let mut artifact_challenge = [0_u8; ARTIFACT_CHALLENGE_BYTES];
    let mut capability_bytes = [0_u8; 32];
    let mut host_nonce = [0_u8; 32];
    fill_private_random(&mut artifact_challenge)?;
    fill_private_random(&mut capability_bytes)?;
    fill_private_random(&mut host_nonce)?;

    let (host_writer, child_stdin) = UnixStream::pair()?;
    let (child_stdout, host_reader) = UnixStream::pair()?;
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
    let mut io = SplitIo {
        reader: host_reader,
        writer: host_writer,
    };

    io.write_all(&artifact_challenge)?;
    io.flush()?;
    let hello = match artifact::read_message(&mut io)? {
        artifact::Message::WorkerHello(hello) => hello,
        _ => return Err(artifact::GateError::Invalid("host expected artifact worker hello").into()),
    };
    artifact::validate_worker_hello(
        &hello,
        expected_pid,
        expected_artifact_binding,
        &artifact_challenge,
    )?;
    artifact::write_message(
        &mut io,
        &artifact::Message::HostAck(artifact::HostAck::new(artifact_challenge)?),
    )?;
    let confirmation = match artifact::read_message(&mut io)? {
        artifact::Message::WorkerConfirm(confirmation) => confirmation,
        _ => {
            return Err(
                artifact::GateError::Invalid("host expected artifact worker confirmation").into(),
            );
        }
    };
    if !confirmation.challenge_matches(&artifact_challenge) {
        return Err(artifact::GateError::AuthenticationFailed.into());
    }
    artifact_challenge.fill(0);

    io.write_all(&capability_bytes)?;
    io.write_all(&host_nonce)?;
    io.flush()?;
    let expected_browser = expected_worker(capability_bytes, host_nonce)?;
    capability_bytes.fill(0);
    host_nonce.fill(0);

    let browser_binding = browser::host_handshake(&mut io, &expected_browser)?;
    if browser_binding.identity != expected_browser.identity
        || browser_binding.source_pin != expected_browser.source_pin
        || browser_binding.authority != browser::AuthorityPosture::qualification_only()
    {
        return Err(std::io::Error::other("browser handshake binding drifted after artifact gate").into());
    }

    let mut channel = browser::FramedChannel::new(io, browser_binding);
    channel.send(&browser::Message::Command(browser::CommandFrame::new(
        1,
        browser_binding.identity,
        1,
        browser::CommandKind::Ping,
    )?))?;

    match mode {
        HostMode::Normal => {
            require_completed_outcome(
                channel.receive()?,
                1,
                browser_binding.identity,
                1,
                "pong",
            )?;
            channel.send(&browser::Message::Command(browser::CommandFrame::new(
                2,
                browser_binding.identity,
                1,
                browser::CommandKind::Shutdown,
            )?))?;
            require_completed_outcome(
                channel.receive()?,
                2,
                browser_binding.identity,
                1,
                "shutdown_complete",
            )?;
            drop(channel);
            let status = guard.wait_bounded(REAP_TIMEOUT)?;
            if !status.success() {
                return Err(artifact::GateError::ChildExit(status.to_string()).into());
            }
            guard.disarm();
            println!(
                "{{\"artifact_binding\":true,\"browser_session_binding\":true,\"build_manifest_binding\":true,\"deadlines\":true,\"external_network\":false,\"forced_kill\":false,\"owner_epoch_binding\":true,\"production_caller\":false,\"reaped\":true,\"runtime_authority\":false,\"servo_linked\":false,\"source_pin_binding\":true,\"source_receipt_binding\":true,\"status\":\"ARTIFACT_TO_BROWSER_HANDOFF_QUALIFICATION_PASS\"}}"
            );
            Ok(())
        }
        HostMode::ForceKill => {
            let timeout_observed = match channel.receive() {
                Err(browser::ProtocolError::Io(error))
                    if matches!(
                        error.kind(),
                        std::io::ErrorKind::TimedOut | std::io::ErrorKind::WouldBlock
                    ) => true,
                Err(error) => return Err(error.into()),
                Ok(_) => false,
            };
            drop(channel);
            if !timeout_observed {
                return Err(std::io::Error::other(
                    "hung browser worker did not trigger the private-channel deadline",
                )
                .into());
            }
            guard.kill()?;
            let status = guard.wait_bounded(REAP_TIMEOUT)?;
            guard.disarm();
            if status.success() {
                return Err(std::io::Error::other(
                    "forced-kill startup-bridge worker exited successfully",
                )
                .into());
            }
            println!(
                "{{\"artifact_binding\":true,\"browser_session_binding\":true,\"deadline_observed\":true,\"external_network\":false,\"forced_kill\":true,\"production_caller\":false,\"reaped\":true,\"runtime_authority\":false,\"servo_linked\":false,\"status\":\"ARTIFACT_TO_BROWSER_HANDOFF_FORCE_KILL_REAP_PASS\"}}"
            );
            Ok(())
        }
    }
}

#[cfg(not(unix))]
fn run_host(_mode: HostMode) -> Result<(), Box<dyn Error>> {
    Err(std::io::Error::new(
        std::io::ErrorKind::Unsupported,
        "C1 startup bridge currently requires inherited Unix socketpairs",
    )
    .into())
}

fn run_worker(hang_after_browser_handshake: bool) -> Result<(), Box<dyn Error>> {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut io = SplitIo {
        reader: stdin.lock(),
        writer: stdout.lock(),
    };

    let mut artifact_challenge = [0_u8; ARTIFACT_CHALLENGE_BYTES];
    io.read_exact(&mut artifact_challenge)?;
    let artifact_binding =
        artifact::binding_for_current_executable(BUILD_MANIFEST, SOURCE_RECEIPT)?;
    artifact::write_message(
        &mut io,
        &artifact::Message::WorkerHello(artifact::LaunchHello::new(
            std::process::id(),
            artifact_binding,
            artifact_challenge,
        )?),
    )?;
    let acknowledgement = match artifact::read_message(&mut io)? {
        artifact::Message::HostAck(acknowledgement) => acknowledgement,
        _ => return Err(artifact::GateError::Invalid("worker expected artifact host ack").into()),
    };
    if !acknowledgement.challenge_matches(&artifact_challenge) {
        return Err(artifact::GateError::AuthenticationFailed.into());
    }
    artifact::write_message(
        &mut io,
        &artifact::Message::WorkerConfirm(artifact::WorkerConfirm::new(
            artifact_challenge,
        )?),
    )?;
    artifact_challenge.fill(0);

    let mut browser_bootstrap = [0_u8; BROWSER_BOOTSTRAP_BYTES];
    io.read_exact(&mut browser_bootstrap)?;
    let mut capability_bytes = [0_u8; 32];
    capability_bytes.copy_from_slice(&browser_bootstrap[..32]);
    let mut host_nonce = [0_u8; 32];
    host_nonce.copy_from_slice(&browser_bootstrap[32..]);
    browser_bootstrap.fill(0);

    let expected_browser = expected_worker(capability_bytes, host_nonce)?;
    capability_bytes.fill(0);
    host_nonce.fill(0);
    let browser_binding = browser::worker_handshake(&mut io, &expected_browser)?;
    if browser_binding.identity != expected_browser.identity
        || browser_binding.source_pin != expected_browser.source_pin
        || browser_binding.authority != browser::AuthorityPosture::qualification_only()
    {
        return Err(std::io::Error::other("worker browser binding drifted after artifact gate").into());
    }

    let mut channel = browser::FramedChannel::new(io, browser_binding);
    loop {
        let command = match channel.receive()? {
            browser::Message::Command(command) => command,
            _ => return Err(std::io::Error::other("worker received a non-command browser frame").into()),
        };
        if hang_after_browser_handshake && matches!(command.command, browser::CommandKind::Ping) {
            loop {
                thread::sleep(Duration::from_secs(60));
            }
        }
        let (status, code, shutdown) = match &command.command {
            browser::CommandKind::Ping => (browser::OutcomeStatus::Completed, "pong", false),
            browser::CommandKind::Shutdown => {
                (browser::OutcomeStatus::Completed, "shutdown_complete", true)
            }
            _ => (
                browser::OutcomeStatus::Denied,
                "startup_bridge_command_denied",
                false,
            ),
        };
        channel.send(&browser::Message::Outcome(browser::OutcomeFrame::new(
            command.request_id,
            command.identity,
            command.page_revision,
            status,
            code,
        )?))?;
        if shutdown {
            return Ok(());
        }
    }
}

fn expected_worker(
    capability_bytes: [u8; 32],
    host_nonce: [u8; 32],
) -> Result<browser::HostExpectedWorker, Box<dyn Error>> {
    Ok(browser::HostExpectedWorker::new(
        browser::WorkerIdentity::new(
            browser::BrowserSessionId::new(SESSION_BYTES)?,
            GENERATION,
            OWNER_EPOCH,
        )?,
        browser::SourcePin::new(SERVO_COMMIT, SERVO_TREE)?,
        browser::StartupCapability::new(capability_bytes)?,
        host_nonce,
    )?)
}

fn require_completed_outcome(
    message: browser::Message,
    request_id: u64,
    identity: browser::WorkerIdentity,
    page_revision: u64,
    code: &str,
) -> Result<(), Box<dyn Error>> {
    let browser::Message::Outcome(outcome) = message else {
        return Err(std::io::Error::other("host received a non-outcome browser frame").into());
    };
    if outcome.request_id != request_id
        || outcome.identity != identity
        || outcome.page_revision != page_revision
        || outcome.status != browser::OutcomeStatus::Completed
        || outcome.code != code
    {
        return Err(std::io::Error::other(
            "browser outcome did not match the artifact-bound request",
        )
        .into());
    }
    Ok(())
}

#[cfg(unix)]
fn fill_private_random(output: &mut [u8]) -> Result<(), Box<dyn Error>> {
    let mut random = std::fs::File::open("/dev/urandom")?;
    random.read_exact(output)?;
    Ok(())
}

struct SplitIo<R, W> {
    reader: R,
    writer: W,
}

impl<R: Read, W> Read for SplitIo<R, W> {
    fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
        self.reader.read(buffer)
    }
}

impl<R, W: Write> Write for SplitIo<R, W> {
    fn write(&mut self, buffer: &[u8]) -> std::io::Result<usize> {
        self.writer.write(buffer)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

struct ChildGuard {
    child: Option<Child>,
}

impl ChildGuard {
    fn new(child: Child) -> Self {
        Self { child: Some(child) }
    }

    fn child_mut(&mut self) -> Result<&mut Child, artifact::GateError> {
        self.child
            .as_mut()
            .ok_or(artifact::GateError::Invalid("child guard is disarmed"))
    }

    fn kill(&mut self) -> Result<(), artifact::GateError> {
        match self.child_mut()?.kill() {
            Ok(()) => Ok(()),
            Err(error) if error.kind() == std::io::ErrorKind::InvalidInput => Ok(()),
            Err(error) => Err(error.into()),
        }
    }

    fn wait_bounded(&mut self, duration: Duration) -> Result<ExitStatus, artifact::GateError> {
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
                        return Err(artifact::GateError::DeadlineExceeded);
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
