use std::path::PathBuf;
use std::process::Stdio;
use std::time::Duration;

use tokio::io::BufReader;
use tokio::process::Child;
use tokio::process::ChildStdin;
use tokio::process::ChildStdout;
use tokio::process::Command;
use tokio::time::timeout;

use crate::BrowserActor;
use crate::BrowserRequest;
use crate::BrowserResponse;
use crate::BrowserSessionId;
use crate::FixtureBrowserEngine;
use crate::QualificationError;
use crate::browser_worker_protocol::BrowserWorkerParentEvent;
use crate::browser_worker_protocol::BrowserWorkerParentSession;
use crate::browser_worker_protocol::BrowserWorkerProtocolError;
use crate::browser_worker_protocol::BrowserWorkerServerEvent;
use crate::browser_worker_protocol::BrowserWorkerServerSession;
use crate::browser_worker_protocol::BrowserWorkerShutdownReason;
use crate::browser_worker_protocol::BrowserWorkerStartupCapability;
use crate::browser_worker_protocol::BrowserWorkerTransportKind;
use crate::browser_worker_protocol::read_browser_worker_frame;
use crate::browser_worker_protocol::write_browser_worker_frame;

pub const BROWSER_WORKER_SESSION_ID_ENV: &str = "HEPTA_BROWSER_WORKER_SESSION_ID";
pub const BROWSER_WORKER_GENERATION_ENV: &str = "HEPTA_BROWSER_WORKER_GENERATION";
pub const BROWSER_WORKER_CAPABILITY_SHA256_ENV: &str = "HEPTA_BROWSER_WORKER_CAPABILITY_SHA256";
pub const BROWSER_WORKER_MODE_ARGUMENT: &str = "--hepta-browser-worker-qualification";

const DEFAULT_STARTUP_TIMEOUT: Duration = Duration::from_secs(5);
const DEFAULT_IO_TIMEOUT: Duration = Duration::from_secs(5);
const MIN_TIMEOUT: Duration = Duration::from_millis(1);
const MAX_TIMEOUT: Duration = Duration::from_secs(60);

#[derive(Debug, thiserror::Error)]
pub enum BrowserWorkerHarnessError {
    #[error("browser worker protocol failed: {0}")]
    Protocol(#[from] BrowserWorkerProtocolError),
    #[error("browser worker qualification failed: {0}")]
    Qualification(#[from] QualificationError),
    #[error("browser worker process I/O failed: {0}")]
    Io(#[from] std::io::Error),
    #[error("browser worker launch specification is invalid: {0}")]
    Invalid(String),
    #[error("browser worker startup timed out")]
    StartupTimeout,
    #[error("browser worker request timed out")]
    IoTimeout,
    #[error("browser worker process is missing a required private pipe")]
    MissingPipe,
    #[error("browser worker returned an unexpected protocol event")]
    UnexpectedEvent,
    #[error("browser worker exited unsuccessfully: {0}")]
    ChildExit(String),
}

#[derive(Clone, Debug)]
pub struct BrowserWorkerLaunchSpec {
    pub program: PathBuf,
    pub session_id: BrowserSessionId,
    pub generation: u64,
    pub startup_timeout: Duration,
    pub io_timeout: Duration,
}

impl BrowserWorkerLaunchSpec {
    pub fn new(
        program: PathBuf,
        session_id: BrowserSessionId,
        generation: u64,
    ) -> Result<Self, BrowserWorkerHarnessError> {
        let spec = Self {
            program,
            session_id,
            generation,
            startup_timeout: DEFAULT_STARTUP_TIMEOUT,
            io_timeout: DEFAULT_IO_TIMEOUT,
        };
        spec.validate()?;
        Ok(spec)
    }

    pub fn with_timeouts(
        mut self,
        startup_timeout: Duration,
        io_timeout: Duration,
    ) -> Result<Self, BrowserWorkerHarnessError> {
        self.startup_timeout = startup_timeout;
        self.io_timeout = io_timeout;
        self.validate()?;
        Ok(self)
    }

    pub fn validate(&self) -> Result<(), BrowserWorkerHarnessError> {
        if !self.program.is_absolute() {
            return Err(BrowserWorkerHarnessError::Invalid(
                "worker executable path must be absolute".to_string(),
            ));
        }
        let metadata = std::fs::symlink_metadata(&self.program).map_err(|error| {
            BrowserWorkerHarnessError::Invalid(format!(
                "worker executable cannot be inspected: {error}"
            ))
        })?;
        if metadata.file_type().is_symlink() || !metadata.is_file() {
            return Err(BrowserWorkerHarnessError::Invalid(
                "worker executable must be an existing non-symlink regular file".to_string(),
            ));
        }
        if self.generation == 0 {
            return Err(BrowserWorkerHarnessError::Invalid(
                "worker generation must be nonzero".to_string(),
            ));
        }
        if !(MIN_TIMEOUT..=MAX_TIMEOUT).contains(&self.startup_timeout)
            || !(MIN_TIMEOUT..=MAX_TIMEOUT).contains(&self.io_timeout)
        {
            return Err(BrowserWorkerHarnessError::Invalid(
                "worker timeouts must be within one millisecond and sixty seconds".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct QualificationBrowserWorker {
    child: Child,
    writer: ChildStdin,
    reader: BufReader<ChildStdout>,
    protocol: BrowserWorkerParentSession,
    io_timeout: Duration,
    worker_pid: u32,
}

impl QualificationBrowserWorker {
    pub async fn spawn(spec: BrowserWorkerLaunchSpec) -> Result<Self, BrowserWorkerHarnessError> {
        spec.validate()?;
        let startup_capability = BrowserWorkerStartupCapability::generate();
        let capability_sha256 = startup_capability.digest();
        let (protocol, hello) = BrowserWorkerParentSession::begin(
            spec.session_id.clone(),
            spec.generation,
            BrowserWorkerTransportKind::QualificationStdioPipe,
            startup_capability,
        )?;

        let mut command = Command::new(&spec.program);
        command
            .arg(BROWSER_WORKER_MODE_ARGUMENT)
            .env(BROWSER_WORKER_SESSION_ID_ENV, spec.session_id.as_str())
            .env(BROWSER_WORKER_GENERATION_ENV, spec.generation.to_string())
            .env(BROWSER_WORKER_CAPABILITY_SHA256_ENV, capability_sha256)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .kill_on_drop(true);
        let mut child = command.spawn()?;
        let expected_pid = child.id().ok_or_else(|| {
            BrowserWorkerHarnessError::Invalid(
                "browser worker has no spawned process identity".to_string(),
            )
        })?;
        let writer = child
            .stdin
            .take()
            .ok_or(BrowserWorkerHarnessError::MissingPipe)?;
        let reader = child
            .stdout
            .take()
            .map(BufReader::new)
            .ok_or(BrowserWorkerHarnessError::MissingPipe)?;

        let mut worker = Self {
            child,
            writer,
            reader,
            protocol,
            io_timeout: spec.io_timeout,
            worker_pid: expected_pid,
        };
        timeout(
            spec.startup_timeout,
            write_browser_worker_frame(&mut worker.writer, &hello),
        )
        .await
        .map_err(|_| BrowserWorkerHarnessError::StartupTimeout)??;
        let ready = timeout(
            spec.startup_timeout,
            read_browser_worker_frame(&mut worker.reader),
        )
        .await
        .map_err(|_| BrowserWorkerHarnessError::StartupTimeout)??;
        match worker.protocol.accept(ready)? {
            BrowserWorkerParentEvent::Ready {
                worker_pid,
                transport: BrowserWorkerTransportKind::QualificationStdioPipe,
            } if worker_pid == expected_pid => Ok(worker),
            BrowserWorkerParentEvent::Ready { worker_pid, .. } => {
                Err(BrowserWorkerHarnessError::Invalid(format!(
                    "worker ready PID {worker_pid} differs from spawned PID {expected_pid}"
                )))
            }
            _ => Err(BrowserWorkerHarnessError::UnexpectedEvent),
        }
    }

    pub fn process_id(&self) -> u32 {
        self.worker_pid
    }

    pub fn is_ready(&self) -> bool {
        self.protocol.is_ready()
    }

    pub async fn request(
        &mut self,
        request: BrowserRequest,
    ) -> Result<BrowserResponse, BrowserWorkerHarnessError> {
        let frame = self.protocol.next_request(request)?;
        timeout(
            self.io_timeout,
            write_browser_worker_frame(&mut self.writer, &frame),
        )
        .await
        .map_err(|_| BrowserWorkerHarnessError::IoTimeout)??;
        let response = timeout(self.io_timeout, read_browser_worker_frame(&mut self.reader))
            .await
            .map_err(|_| BrowserWorkerHarnessError::IoTimeout)??;
        match self.protocol.accept(response)? {
            BrowserWorkerParentEvent::Response(response) => Ok(response),
            BrowserWorkerParentEvent::ProtocolError { code, message } => {
                Err(BrowserWorkerHarnessError::ChildExit(format!(
                    "worker protocol error {code:?}: {message}"
                )))
            }
            _ => Err(BrowserWorkerHarnessError::UnexpectedEvent),
        }
    }

    pub async fn shutdown(
        &mut self,
        reason: BrowserWorkerShutdownReason,
    ) -> Result<(), BrowserWorkerHarnessError> {
        let frame = self.protocol.next_shutdown(reason)?;
        timeout(
            self.io_timeout,
            write_browser_worker_frame(&mut self.writer, &frame),
        )
        .await
        .map_err(|_| BrowserWorkerHarnessError::IoTimeout)??;
        let acknowledgement = timeout(self.io_timeout, read_browser_worker_frame(&mut self.reader))
            .await
            .map_err(|_| BrowserWorkerHarnessError::IoTimeout)??;
        if !matches!(
            self.protocol.accept(acknowledgement)?,
            BrowserWorkerParentEvent::ShutdownAck
        ) {
            return Err(BrowserWorkerHarnessError::UnexpectedEvent);
        }
        let status = timeout(self.io_timeout, self.child.wait())
            .await
            .map_err(|_| BrowserWorkerHarnessError::IoTimeout)??;
        if !status.success() {
            return Err(BrowserWorkerHarnessError::ChildExit(status.to_string()));
        }
        Ok(())
    }
}

pub async fn run_qualification_browser_worker() -> Result<(), BrowserWorkerHarnessError> {
    let session_id = std::env::var(BROWSER_WORKER_SESSION_ID_ENV)
        .map_err(|_| {
            BrowserWorkerHarnessError::Invalid(
                "browser worker session identity is missing".to_string(),
            )
        })
        .and_then(|value| {
            BrowserSessionId::parse(value).map_err(BrowserWorkerHarnessError::Qualification)
        })?;
    let generation = std::env::var(BROWSER_WORKER_GENERATION_ENV)
        .map_err(|_| {
            BrowserWorkerHarnessError::Invalid("browser worker generation is missing".to_string())
        })?
        .parse::<u64>()
        .map_err(|_| {
            BrowserWorkerHarnessError::Invalid("browser worker generation is invalid".to_string())
        })?;
    let capability_sha256 = std::env::var(BROWSER_WORKER_CAPABILITY_SHA256_ENV).map_err(|_| {
        BrowserWorkerHarnessError::Invalid(
            "browser worker startup capability digest is missing".to_string(),
        )
    })?;

    let mut protocol = BrowserWorkerServerSession::new(
        session_id.clone(),
        generation,
        capability_sha256,
        BrowserWorkerTransportKind::QualificationStdioPipe,
    )?;
    let mut reader = BufReader::new(tokio::io::stdin());
    let mut writer = tokio::io::stdout();
    let hello = read_browser_worker_frame(&mut reader).await?;
    let ready = match protocol.accept(hello)? {
        BrowserWorkerServerEvent::HandshakeAccepted { ready, .. } => ready,
        _ => return Err(BrowserWorkerHarnessError::UnexpectedEvent),
    };
    write_browser_worker_frame(&mut writer, &ready).await?;

    let mut actor = BrowserActor::new(session_id, generation, FixtureBrowserEngine::default())?;
    let mut logical_now_ms = 1_u64;
    loop {
        let frame = read_browser_worker_frame(&mut reader).await?;
        match protocol.accept(frame)? {
            BrowserWorkerServerEvent::Request(request) => {
                let response = actor.handle(request, logical_now_ms)?;
                logical_now_ms = logical_now_ms.checked_add(1).ok_or_else(|| {
                    BrowserWorkerHarnessError::Invalid(
                        "browser worker logical clock overflowed".to_string(),
                    )
                })?;
                let response = protocol.next_response(response)?;
                write_browser_worker_frame(&mut writer, &response).await?;
            }
            BrowserWorkerServerEvent::Shutdown(_) => {
                let acknowledgement = protocol.next_shutdown_ack()?;
                write_browser_worker_frame(&mut writer, &acknowledgement).await?;
                return Ok(());
            }
            BrowserWorkerServerEvent::HandshakeAccepted { .. } => {
                return Err(BrowserWorkerHarnessError::UnexpectedEvent);
            }
        }
    }
}
