#![cfg(unix)]

use std::os::fd::OwnedFd;
use std::os::unix::net::UnixStream as StdUnixStream;
use std::process::Stdio;

use tokio::io::BufReader;
use tokio::net::UnixStream;
use tokio::process::Child;
use tokio::process::Command;
use tokio::time::timeout;

use crate::BrowserActor;
use crate::BrowserRequest;
use crate::BrowserResponse;
use crate::BrowserSessionId;
use crate::FixtureBrowserEngine;
use crate::browser_worker_harness::BROWSER_WORKER_CAPABILITY_SHA256_ENV;
use crate::browser_worker_harness::BROWSER_WORKER_GENERATION_ENV;
use crate::browser_worker_harness::BROWSER_WORKER_SESSION_ID_ENV;
use crate::browser_worker_harness::BrowserWorkerHarnessError;
use crate::browser_worker_harness::BrowserWorkerLaunchSpec;
use crate::browser_worker_protocol::BrowserWorkerParentEvent;
use crate::browser_worker_protocol::BrowserWorkerParentSession;
use crate::browser_worker_protocol::BrowserWorkerServerEvent;
use crate::browser_worker_protocol::BrowserWorkerServerSession;
use crate::browser_worker_protocol::BrowserWorkerShutdownReason;
use crate::browser_worker_protocol::BrowserWorkerStartupCapability;
use crate::browser_worker_protocol::BrowserWorkerTransportKind;
use crate::browser_worker_protocol::read_browser_worker_frame;
use crate::browser_worker_protocol::write_browser_worker_frame;

pub const BROWSER_WORKER_UNIX_MODE_ARGUMENT: &str =
    "--hepta-browser-worker-unix-socketpair-qualification";

#[derive(Debug)]
pub struct UnixQualificationBrowserWorker {
    child: Child,
    stream: UnixStream,
    protocol: BrowserWorkerParentSession,
    io_timeout: std::time::Duration,
    worker_pid: u32,
}

impl UnixQualificationBrowserWorker {
    pub async fn spawn(
        spec: BrowserWorkerLaunchSpec,
    ) -> Result<Self, BrowserWorkerHarnessError> {
        spec.validate()?;
        let startup_capability = BrowserWorkerStartupCapability::generate();
        let capability_sha256 = startup_capability.digest();
        let (protocol, hello) = BrowserWorkerParentSession::begin(
            spec.session_id.clone(),
            spec.generation,
            BrowserWorkerTransportKind::UnixInheritedSocketPair,
            startup_capability,
        )?;

        let (parent_stream, child_stream) = StdUnixStream::pair()?;
        parent_stream.set_nonblocking(true)?;
        let child_output_stream = child_stream.try_clone()?;
        let child_input_fd: OwnedFd = child_stream.into();
        let child_output_fd: OwnedFd = child_output_stream.into();

        let mut command = Command::new(&spec.program);
        command
            .arg(BROWSER_WORKER_UNIX_MODE_ARGUMENT)
            .env(BROWSER_WORKER_SESSION_ID_ENV, spec.session_id.as_str())
            .env(BROWSER_WORKER_GENERATION_ENV, spec.generation.to_string())
            .env(BROWSER_WORKER_CAPABILITY_SHA256_ENV, capability_sha256)
            .stdin(Stdio::from(child_input_fd))
            .stdout(Stdio::from(child_output_fd))
            .stderr(Stdio::inherit())
            .kill_on_drop(true);
        let child = command.spawn()?;
        let expected_pid = child.id().ok_or(BrowserWorkerHarnessError::Invalid(
            "Unix browser worker has no process identity".to_string(),
        ))?;
        let stream = UnixStream::from_std(parent_stream)?;
        let mut worker = Self {
            child,
            stream,
            protocol,
            io_timeout: spec.io_timeout,
            worker_pid: expected_pid,
        };

        timeout(
            spec.startup_timeout,
            write_browser_worker_frame(&mut worker.stream, &hello),
        )
        .await
        .map_err(|_| BrowserWorkerHarnessError::StartupTimeout)??;
        let ready = timeout(
            spec.startup_timeout,
            read_browser_worker_frame(&mut worker.stream),
        )
        .await
        .map_err(|_| BrowserWorkerHarnessError::StartupTimeout)??;
        match worker.protocol.accept(ready)? {
            BrowserWorkerParentEvent::Ready {
                worker_pid,
                transport: BrowserWorkerTransportKind::UnixInheritedSocketPair,
            } if worker_pid == expected_pid => Ok(worker),
            BrowserWorkerParentEvent::Ready { worker_pid, .. } => {
                Err(BrowserWorkerHarnessError::Invalid(format!(
                    "Unix worker ready PID {worker_pid} differs from spawned PID {expected_pid}"
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
            write_browser_worker_frame(&mut self.stream, &frame),
        )
        .await
        .map_err(|_| BrowserWorkerHarnessError::IoTimeout)??;
        let response = timeout(
            self.io_timeout,
            read_browser_worker_frame(&mut self.stream),
        )
        .await
        .map_err(|_| BrowserWorkerHarnessError::IoTimeout)??;
        match self.protocol.accept(response)? {
            BrowserWorkerParentEvent::Response(response) => Ok(response),
            BrowserWorkerParentEvent::ProtocolError { code, message } => {
                Err(BrowserWorkerHarnessError::ChildExit(format!(
                    "Unix worker protocol error {code:?}: {message}"
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
            write_browser_worker_frame(&mut self.stream, &frame),
        )
        .await
        .map_err(|_| BrowserWorkerHarnessError::IoTimeout)??;
        let acknowledgement = timeout(
            self.io_timeout,
            read_browser_worker_frame(&mut self.stream),
        )
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

pub async fn run_unix_qualification_browser_worker() -> Result<(), BrowserWorkerHarnessError> {
    let session_id = std::env::var(BROWSER_WORKER_SESSION_ID_ENV)
        .map_err(|_| {
            BrowserWorkerHarnessError::Invalid(
                "Unix browser worker session identity is missing".to_string(),
            )
        })
        .and_then(|value| {
            BrowserSessionId::parse(value).map_err(BrowserWorkerHarnessError::Qualification)
        })?;
    let generation = std::env::var(BROWSER_WORKER_GENERATION_ENV)
        .map_err(|_| {
            BrowserWorkerHarnessError::Invalid(
                "Unix browser worker generation is missing".to_string(),
            )
        })?
        .parse::<u64>()
        .map_err(|_| {
            BrowserWorkerHarnessError::Invalid(
                "Unix browser worker generation is invalid".to_string(),
            )
        })?;
    let capability_sha256 = std::env::var(BROWSER_WORKER_CAPABILITY_SHA256_ENV).map_err(|_| {
        BrowserWorkerHarnessError::Invalid(
            "Unix browser worker startup capability digest is missing".to_string(),
        )
    })?;

    let mut protocol = BrowserWorkerServerSession::new(
        session_id.clone(),
        generation,
        capability_sha256,
        BrowserWorkerTransportKind::UnixInheritedSocketPair,
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
                        "Unix browser worker logical clock overflowed".to_string(),
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
