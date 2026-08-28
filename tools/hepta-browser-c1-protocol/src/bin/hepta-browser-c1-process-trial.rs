use std::error::Error;
use std::ffi::OsStr;
use std::io::Read;
use std::io::Write;
use std::process::ChildStdin;
use std::process::ChildStdout;
use std::process::Command;
use std::process::Stdio;

use hepta_browser_worker_protocol_qualification::AuthorityPosture;
use hepta_browser_worker_protocol_qualification::BrowserSessionId;
use hepta_browser_worker_protocol_qualification::CommandFrame;
use hepta_browser_worker_protocol_qualification::CommandKind;
use hepta_browser_worker_protocol_qualification::FramedChannel;
use hepta_browser_worker_protocol_qualification::HostExpectedWorker;
use hepta_browser_worker_protocol_qualification::Message;
use hepta_browser_worker_protocol_qualification::OutcomeFrame;
use hepta_browser_worker_protocol_qualification::OutcomeStatus;
use hepta_browser_worker_protocol_qualification::SourcePin;
use hepta_browser_worker_protocol_qualification::StartupCapability;
use hepta_browser_worker_protocol_qualification::WorkerIdentity;
use hepta_browser_worker_protocol_qualification::host_handshake;
use hepta_browser_worker_protocol_qualification::worker_handshake;

const SERVO_COMMIT: &str = "0a48e298482659817eb50097df23841f2b8e3044";
const SERVO_TREE: &str = "b04d2f75b3217374d079d579c270177b57fa1389";
const BOOTSTRAP_BYTES: usize = 64;
const SESSION_BYTES: [u8; 32] = [0x71; 32];
const GENERATION: u64 = 1;
const OWNER_EPOCH: u64 = 1;

fn main() -> Result<(), Box<dyn Error>> {
    let arguments = std::env::args_os().skip(1).collect::<Vec<_>>();
    match arguments.as_slice() {
        [] => run_host(),
        [mode] if mode == OsStr::new("--worker") => run_worker(),
        _ => Err(std::io::Error::other("unsupported C1 process-trial arguments").into()),
    }
}

fn run_host() -> Result<(), Box<dyn Error>> {
    let executable = std::env::current_exe()?;
    let mut child = Command::new(executable)
        .arg("--worker")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()?;

    let mut child_stdin = child
        .stdin
        .take()
        .ok_or_else(|| std::io::Error::other("child stdin was not piped"))?;
    let child_stdout = child
        .stdout
        .take()
        .ok_or_else(|| std::io::Error::other("child stdout was not piped"))?;

    let mut capability_bytes = [0_u8; 32];
    let mut host_nonce = [0_u8; 32];
    fill_private_random(&mut capability_bytes)?;
    fill_private_random(&mut host_nonce)?;

    child_stdin.write_all(&capability_bytes)?;
    child_stdin.write_all(&host_nonce)?;
    child_stdin.flush()?;

    let expected = expected_worker(capability_bytes, host_nonce)?;
    capability_bytes.fill(0);
    host_nonce.fill(0);

    let mut io = SplitIo {
        reader: child_stdout,
        writer: child_stdin,
    };
    let binding = host_handshake(&mut io, &expected)?;
    if binding.authority != AuthorityPosture::qualification_only() {
        return Err(std::io::Error::other("worker returned a positive authority posture").into());
    }

    let mut channel = FramedChannel::new(io, binding);
    channel.send(&Message::Command(CommandFrame::new(
        1,
        binding.identity,
        1,
        CommandKind::Ping,
    )?))?;
    require_completed_outcome(channel.receive()?, 1, binding.identity, 1, "pong")?;

    channel.send(&Message::Command(CommandFrame::new(
        2,
        binding.identity,
        1,
        CommandKind::Shutdown,
    )?))?;
    require_completed_outcome(
        channel.receive()?,
        2,
        binding.identity,
        1,
        "shutdown_complete",
    )?;
    drop(channel);

    let status = child.wait()?;
    if !status.success() {
        return Err(std::io::Error::other(format!(
            "C1 worker process exited unsuccessfully: {status}"
        ))
        .into());
    }

    println!(
        "{{\"authority\":false,\"external_network\":false,\"process_boundary\":\"anonymous_inherited_pipes\",\"servo_linked\":false,\"status\":\"QUALIFICATION_ONLY_PROCESS_TRIAL_PASS\"}}"
    );
    Ok(())
}

fn run_worker() -> Result<(), Box<dyn Error>> {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut reader = stdin.lock();
    let writer = stdout.lock();

    let mut bootstrap = [0_u8; BOOTSTRAP_BYTES];
    reader.read_exact(&mut bootstrap)?;
    let mut capability_bytes = [0_u8; 32];
    capability_bytes.copy_from_slice(&bootstrap[..32]);
    let mut host_nonce = [0_u8; 32];
    host_nonce.copy_from_slice(&bootstrap[32..]);
    bootstrap.fill(0);

    let expected = expected_worker(capability_bytes, host_nonce)?;
    capability_bytes.fill(0);
    host_nonce.fill(0);

    let mut io = SplitIo { reader, writer };
    let binding = worker_handshake(&mut io, &expected)?;
    let mut channel = FramedChannel::new(io, binding);

    loop {
        let command = match channel.receive()? {
            Message::Command(command) => command,
            _ => return Err(std::io::Error::other("worker received a non-command frame").into()),
        };
        let (status, code, shutdown) = match &command.command {
            CommandKind::Ping => (OutcomeStatus::Completed, "pong", false),
            CommandKind::Shutdown => (OutcomeStatus::Completed, "shutdown_complete", true),
            _ => (
                OutcomeStatus::Denied,
                "qualification_trial_command_denied",
                false,
            ),
        };
        channel.send(&Message::Outcome(OutcomeFrame::new(
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
) -> Result<HostExpectedWorker, Box<dyn Error>> {
    Ok(HostExpectedWorker::new(
        WorkerIdentity::new(
            BrowserSessionId::new(SESSION_BYTES)?,
            GENERATION,
            OWNER_EPOCH,
        )?,
        SourcePin::new(SERVO_COMMIT, SERVO_TREE)?,
        StartupCapability::new(capability_bytes)?,
        host_nonce,
    )?)
}

fn require_completed_outcome(
    message: Message,
    request_id: u64,
    identity: WorkerIdentity,
    page_revision: u64,
    code: &str,
) -> Result<(), Box<dyn Error>> {
    let Message::Outcome(outcome) = message else {
        return Err(std::io::Error::other("host received a non-outcome frame").into());
    };
    if outcome.request_id != request_id
        || outcome.identity != identity
        || outcome.page_revision != page_revision
        || outcome.status != OutcomeStatus::Completed
        || outcome.code != code
    {
        return Err(std::io::Error::other("worker outcome did not match the exact request").into());
    }
    Ok(())
}

#[cfg(unix)]
fn fill_private_random(output: &mut [u8]) -> Result<(), Box<dyn Error>> {
    let mut random = std::fs::File::open("/dev/urandom")?;
    random.read_exact(output)?;
    Ok(())
}

#[cfg(not(unix))]
fn fill_private_random(_output: &mut [u8]) -> Result<(), Box<dyn Error>> {
    Err(std::io::Error::new(
        std::io::ErrorKind::Unsupported,
        "C1 process trial currently requires a Unix private-random source",
    )
    .into())
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

#[allow(dead_code)]
fn _assert_host_pipe_types(_stdin: ChildStdin, _stdout: ChildStdout) {}
