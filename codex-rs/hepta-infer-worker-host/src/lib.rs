//! Isolated child-process host for the qualification-only Hepta native inference lane.
//!
//! The crate has no TCP listener, model downloader, remote fallback, Memory/KG writer,
//! effect executor, route writer, or fleet authority. The daemon communicates with one
//! child through inherited stdin/stdout handles and a bounded versioned private protocol.

mod abi;
mod protocol;

use std::fmt;
use std::io::ErrorKind;
use std::path::Path;
use std::process::Stdio;
use std::time::Duration;

use codex_hepta_infer_core::Digest;
use codex_hepta_infer_core::RequestId;
use tokio::io::BufReader;
use tokio::process::Child;
use tokio::process::ChildStdin;
use tokio::process::ChildStdout;
use tokio::process::Command;
use tokio::time;

pub use abi::NativeAbiError;
pub use abi::NativeAbiResult;
pub use abi::NativeRuntimeBindingReceipt;
pub use abi::NativeRuntimeLoader;
pub use abi::NativeRuntimeManifest;
pub use abi::digest_bytes;
pub use protocol::MAX_PRIVATE_WORKER_FRAME_BYTES;
pub use protocol::PRIVATE_WORKER_PROTOCOL_VERSION;
pub use protocol::ProtocolError;
pub use protocol::ProtocolResult;
pub use protocol::WorkerFrame;
pub use protocol::read_frame;
pub use protocol::write_frame;

pub type WorkerHostResult<T> = std::result::Result<T, WorkerHostError>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorkerHostError {
    Config,
    Io,
    Protocol(ProtocolError),
    ProtocolFence,
    Timeout,
    WorkerExited,
    WorkerFailure(String),
}

impl WorkerHostError {
    pub const fn code(&self) -> &'static str {
        match self {
            Self::Config => "INF_WORKER_HOST_CONFIG_INVALID",
            Self::Io => "INF_WORKER_HOST_IO",
            Self::Protocol(_) => "INF_WORKER_HOST_PROTOCOL",
            Self::ProtocolFence => "INF_WORKER_HOST_PROTOCOL_FENCE",
            Self::Timeout => "INF_WORKER_HOST_TIMEOUT",
            Self::WorkerExited => "INF_WORKER_HOST_EXITED",
            Self::WorkerFailure(_) => "INF_WORKER_HOST_FAILURE",
        }
    }
}

impl fmt::Display for WorkerHostError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Protocol(error) => write!(formatter, "{}: {error}", self.code()),
            Self::WorkerFailure(code) => write!(formatter, "{}: {code}", self.code()),
            _ => formatter.write_str(self.code()),
        }
    }
}

impl std::error::Error for WorkerHostError {}

impl From<ProtocolError> for WorkerHostError {
    fn from(error: ProtocolError) -> Self {
        Self::Protocol(error)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixtureExecutionReceipt {
    pub request_id: RequestId,
    pub request_generation: u64,
    pub backend_generation: u64,
    pub session_digest: Digest,
    pub grant_digest: Digest,
    pub token_digest: Digest,
    pub result_digest: Digest,
    pub output_tokens: u32,
    pub fixture_only: bool,
    pub real_native_model_executed: bool,
    pub remote_fallback_attempted: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkerCancelOutcome {
    Acknowledged,
    ForcedKill,
    WorkerExited,
}

pub struct WorkerHostProcess {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
    backend_generation: u64,
    session_digest: Digest,
    max_frame_bytes: usize,
    exchange_timeout: Duration,
}

impl WorkerHostProcess {
    pub async fn spawn_fixture(
        program: &Path,
        mode: &str,
        backend_generation: u64,
        session_digest: Digest,
        max_frame_bytes: usize,
        exchange_timeout: Duration,
    ) -> WorkerHostResult<Self> {
        if !program.is_absolute()
            || mode.is_empty()
            || mode.len() > 64
            || !mode
                .bytes()
                .all(|byte| byte.is_ascii_lowercase() || byte == b'-')
            || backend_generation == 0
            || max_frame_bytes == 0
            || max_frame_bytes > MAX_PRIVATE_WORKER_FRAME_BYTES
            || exchange_timeout.is_zero()
        {
            return Err(WorkerHostError::Config);
        }
        let mut command = Command::new(program);
        command
            .arg("--fixture-mode")
            .arg(mode)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .kill_on_drop(true);
        let mut child = command.spawn().map_err(|_| WorkerHostError::Io)?;
        let stdin = child.stdin.take().ok_or(WorkerHostError::Io)?;
        let stdout = child.stdout.take().ok_or(WorkerHostError::Io)?;
        let mut process = Self {
            child,
            stdin,
            stdout: BufReader::new(stdout),
            backend_generation,
            session_digest,
            max_frame_bytes,
            exchange_timeout,
        };
        process.handshake().await?;
        Ok(process)
    }

    pub fn child_id(&self) -> Option<u32> {
        self.child.id()
    }

    pub async fn submit_fixture(
        &mut self,
        request_id: RequestId,
        request_generation: u64,
        initial_sequence: u64,
        grant_digest: Digest,
        prompt_digest: Digest,
        output_token_limit: u32,
    ) -> WorkerHostResult<FixtureExecutionReceipt> {
        if request_generation == 0 || initial_sequence == 0 || output_token_limit == 0 {
            return Err(WorkerHostError::Config);
        }
        self.write(&WorkerFrame::Submit {
            request_id: request_id.clone(),
            request_generation,
            backend_generation: self.backend_generation,
            sequence: initial_sequence,
            grant_digest: grant_digest.clone(),
            prompt_digest,
            output_token_limit,
        })
        .await?;
        let token = self.read().await?;
        let (token_digest, token_sequence) = match token {
            WorkerFrame::Token {
                request_id: observed_id,
                request_generation: observed_request_generation,
                backend_generation,
                sequence,
                token_digest,
                token_bytes,
            } if observed_id == request_id
                && observed_request_generation == request_generation
                && backend_generation == self.backend_generation
                && sequence == initial_sequence.saturating_add(1)
                && token_bytes > 0 => (token_digest, sequence),
            WorkerFrame::Failure {
                request_id: observed_id,
                request_generation: observed_request_generation,
                backend_generation,
                code,
                forced_worker_termination: _,
            } if observed_id == request_id
                && observed_request_generation == request_generation
                && backend_generation == self.backend_generation => {
                return Err(WorkerHostError::WorkerFailure(code));
            }
            _ => return Err(WorkerHostError::ProtocolFence),
        };
        let complete = self.read().await?;
        let (result_digest, output_tokens) = match complete {
            WorkerFrame::Complete {
                request_id: observed_id,
                request_generation: observed_request_generation,
                backend_generation,
                sequence,
                result_digest,
                output_tokens,
                fixture,
            } if observed_id == request_id
                && observed_request_generation == request_generation
                && backend_generation == self.backend_generation
                && sequence == token_sequence.saturating_add(1)
                && output_tokens == 1
                && output_tokens <= output_token_limit
                && fixture => (result_digest, output_tokens),
            _ => return Err(WorkerHostError::ProtocolFence),
        };
        if result_digest != token_digest {
            return Err(WorkerHostError::ProtocolFence);
        }
        Ok(FixtureExecutionReceipt {
            request_id,
            request_generation,
            backend_generation: self.backend_generation,
            session_digest: self.session_digest.clone(),
            grant_digest,
            token_digest,
            result_digest,
            output_tokens,
            fixture_only: true,
            real_native_model_executed: false,
            remote_fallback_attempted: false,
        })
    }

    pub async fn cancel(
        &mut self,
        request_id: RequestId,
        request_generation: u64,
        cancel_generation: u64,
    ) -> WorkerHostResult<WorkerCancelOutcome> {
        if request_generation == 0 || cancel_generation == 0 {
            return Err(WorkerHostError::Config);
        }
        self.write(&WorkerFrame::Cancel {
            request_id: request_id.clone(),
            request_generation,
            cancel_generation,
            backend_generation: self.backend_generation,
        })
        .await?;
        match time::timeout(
            self.exchange_timeout,
            read_frame(&mut self.stdout, self.max_frame_bytes),
        )
        .await
        {
            Err(_) => {
                self.kill_and_wait().await?;
                Ok(WorkerCancelOutcome::ForcedKill)
            }
            Ok(Err(ProtocolError::Io)) => {
                self.reap().await?;
                Ok(WorkerCancelOutcome::WorkerExited)
            }
            Ok(Err(error)) => Err(error.into()),
            Ok(Ok(WorkerFrame::CancelAck {
                request_id: observed_id,
                request_generation: observed_request_generation,
                cancel_generation: observed_cancel_generation,
                backend_generation,
            })) if observed_id == request_id
                && observed_request_generation == request_generation
                && observed_cancel_generation == cancel_generation
                && backend_generation == self.backend_generation => {
                Ok(WorkerCancelOutcome::Acknowledged)
            }
            Ok(Ok(WorkerFrame::Failure {
                request_id: observed_id,
                request_generation: observed_request_generation,
                backend_generation,
                code,
                forced_worker_termination: _,
            })) if observed_id == request_id
                && observed_request_generation == request_generation
                && backend_generation == self.backend_generation => {
                Err(WorkerHostError::WorkerFailure(code))
            }
            Ok(Ok(_)) => Err(WorkerHostError::ProtocolFence),
        }
    }

    pub async fn shutdown(mut self) -> WorkerHostResult<()> {
        self.write(&WorkerFrame::Shutdown).await?;
        match time::timeout(self.exchange_timeout, self.child.wait()).await {
            Ok(Ok(_)) => Ok(()),
            Ok(Err(_)) => Err(WorkerHostError::Io),
            Err(_) => {
                self.kill_and_wait().await?;
                Ok(())
            }
        }
    }

    async fn handshake(&mut self) -> WorkerHostResult<()> {
        self.write(&WorkerFrame::Hello {
            backend_generation: self.backend_generation,
            session_digest: self.session_digest.clone(),
        })
        .await?;
        match self.read().await? {
            WorkerFrame::Ready {
                backend_generation,
                session_digest,
            } if backend_generation == self.backend_generation
                && session_digest == self.session_digest => Ok(()),
            _ => Err(WorkerHostError::ProtocolFence),
        }
    }

    async fn write(&mut self, frame: &WorkerFrame) -> WorkerHostResult<()> {
        time::timeout(
            self.exchange_timeout,
            write_frame(&mut self.stdin, frame, self.max_frame_bytes),
        )
        .await
        .map_err(|_| WorkerHostError::Timeout)??;
        Ok(())
    }

    async fn read(&mut self) -> WorkerHostResult<WorkerFrame> {
        match time::timeout(
            self.exchange_timeout,
            read_frame(&mut self.stdout, self.max_frame_bytes),
        )
        .await
        {
            Err(_) => Err(WorkerHostError::Timeout),
            Ok(Err(ProtocolError::Io)) => Err(WorkerHostError::WorkerExited),
            Ok(Err(error)) => Err(error.into()),
            Ok(Ok(frame)) => Ok(frame),
        }
    }

    async fn kill_and_wait(&mut self) -> WorkerHostResult<()> {
        match self.child.start_kill() {
            Ok(()) => {}
            Err(error) if error.kind() == ErrorKind::InvalidInput => {}
            Err(_) => return Err(WorkerHostError::Io),
        }
        self.reap().await
    }

    async fn reap(&mut self) -> WorkerHostResult<()> {
        self.child
            .wait()
            .await
            .map(|_| ())
            .map_err(|_| WorkerHostError::Io)
    }
}
