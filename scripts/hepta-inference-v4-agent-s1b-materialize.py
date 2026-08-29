#!/usr/bin/env python3
from __future__ import annotations

from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

PRIVATE = r'''//! Authenticated inherited private worker channel for `hepta-inferd`.
//!
//! No filesystem endpoint is created by this module. A caller must explicitly
//! inject an already-inherited bidirectional stream plus a daemon-side server
//! state. The public UDS remains unprivileged and cannot mint or consume grants.

use std::collections::HashMap;
use std::io;
use std::io::ErrorKind;
use std::sync::Arc;
use std::time::Duration;

use codex_hepta_infer_core::CapabilityKey;
use codex_hepta_infer_core::Controller;
use codex_hepta_infer_core::DaemonToWorker;
use codex_hepta_infer_core::Digest;
use codex_hepta_infer_core::EventFence;
use codex_hepta_infer_core::GrantPurpose;
use codex_hepta_infer_core::InferError;
use codex_hepta_infer_core::MAX_GRANT_TTL_MS;
use codex_hepta_infer_core::MAX_PRIVATE_FRAME_BYTES;
use codex_hepta_infer_core::RequestGrantFence;
use codex_hepta_infer_core::RequestGrantLedger;
use codex_hepta_infer_core::RequestId;
use codex_hepta_infer_core::Result;
use codex_hepta_infer_core::WorkerBootstrapToken;
use codex_hepta_infer_core::WorkerHandshakeFence;
use codex_hepta_infer_core::WorkerToDaemon;
use codex_hepta_infer_core::generate_daemon_challenge_digest;
use tokio::io::AsyncRead;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWrite;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;
use tokio::time;

use super::ReceiptStore;
use super::infer_error_to_io;
use super::persist_and_forget;
use super::unix_time_ms;

const DEFAULT_HANDSHAKE_TTL: Duration = Duration::from_secs(5);
const DEFAULT_PRIVATE_IO_TIMEOUT: Duration = Duration::from_secs(5);
const DEFAULT_GRANT_TTL: Duration = Duration::from_secs(5);
const DEFAULT_MAX_GRANTS: usize = 1_024;

#[derive(Clone, Debug)]
pub struct PrivateWorkerConfig {
    pub max_frame_bytes: usize,
    pub handshake_ttl: Duration,
    pub io_timeout: Duration,
    pub grant_ttl: Duration,
    pub max_grants: usize,
}

impl Default for PrivateWorkerConfig {
    fn default() -> Self {
        Self {
            max_frame_bytes: MAX_PRIVATE_FRAME_BYTES,
            handshake_ttl: DEFAULT_HANDSHAKE_TTL,
            io_timeout: DEFAULT_PRIVATE_IO_TIMEOUT,
            grant_ttl: DEFAULT_GRANT_TTL,
            max_grants: DEFAULT_MAX_GRANTS,
        }
    }
}

impl PrivateWorkerConfig {
    pub fn validate(&self) -> Result<()> {
        let grant_ttl_ms = duration_ms(self.grant_ttl)?;
        if self.max_frame_bytes == 0
            || self.max_frame_bytes > MAX_PRIVATE_FRAME_BYTES
            || self.handshake_ttl.is_zero()
            || self.io_timeout.is_zero()
            || grant_ttl_ms == 0
            || grant_ttl_ms > MAX_GRANT_TTL_MS
            || self.max_grants == 0
        {
            return Err(InferError::InvalidControllerConfig);
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct PendingHandshake {
    worker_pid: u32,
    backend_generation: u64,
    worker_nonce_digest: Digest,
    daemon_challenge_digest: Digest,
    expires_at_unix_ms: u64,
}

#[derive(Clone, Debug)]
struct WorkerSession {
    backend_generation: u64,
    digest: Digest,
}

#[derive(Clone, Debug)]
struct PendingDispatch {
    request_id: RequestId,
    request_generation: u64,
    backend_generation: u64,
    worker_session_digest: Digest,
    grant_digest: Digest,
}

#[derive(Clone, Debug)]
struct ActiveLease {
    request_generation: u64,
    backend_generation: u64,
    worker_session_digest: Digest,
}

pub struct PrivateWorkerServer {
    config: PrivateWorkerConfig,
    backend_generation: u64,
    epoch_key: CapabilityKey,
    bootstrap_key: CapabilityKey,
    grants: RequestGrantLedger,
    pending_handshake: Option<PendingHandshake>,
    session: Option<WorkerSession>,
    pending_dispatch: Option<PendingDispatch>,
    active: HashMap<RequestId, ActiveLease>,
}

impl PrivateWorkerServer {
    pub fn new_random(
        backend_generation: u64,
        config: PrivateWorkerConfig,
    ) -> Result<(Self, WorkerBootstrapToken)> {
        config.validate()?;
        if backend_generation == 0 {
            return Err(InferError::InvalidGeneration);
        }
        let epoch_key = CapabilityKey::generate()?;
        let (bootstrap_key, token) = CapabilityKey::generate_worker_bootstrap()?;
        let max_grants = config.max_grants;
        Ok((
            Self {
                config,
                backend_generation,
                epoch_key,
                bootstrap_key,
                grants: RequestGrantLedger::new(max_grants)?,
                pending_handshake: None,
                session: None,
                pending_dispatch: None,
                active: HashMap::new(),
            },
            token,
        ))
    }

    pub const fn backend_generation(&self) -> u64 {
        self.backend_generation
    }

    pub fn active_leases(&self) -> usize {
        self.active.len()
    }

    pub fn rotate_generation(
        &mut self,
        backend_generation: u64,
    ) -> Result<WorkerBootstrapToken> {
        if backend_generation <= self.backend_generation {
            return Err(InferError::InvalidGeneration);
        }
        let old_generation = self.backend_generation;
        let epoch_key = CapabilityKey::generate()?;
        let (bootstrap_key, token) = CapabilityKey::generate_worker_bootstrap()?;
        self.grants.invalidate_backend_generation(old_generation);
        self.backend_generation = backend_generation;
        self.epoch_key = epoch_key;
        self.bootstrap_key = bootstrap_key;
        self.pending_handshake = None;
        self.session = None;
        self.pending_dispatch = None;
        self.active.clear();
        Ok(token)
    }

    pub fn process_message(
        &mut self,
        controller: &mut Controller,
        message: WorkerToDaemon,
        now_unix_ms: u64,
    ) -> Result<DaemonToWorker> {
        match message {
            WorkerToDaemon::Hello {
                worker_pid,
                backend_generation,
                worker_nonce_digest,
            } => self.begin_handshake(
                worker_pid,
                backend_generation,
                worker_nonce_digest,
                now_unix_ms,
            ),
            WorkerToDaemon::Authenticate {
                worker_pid,
                backend_generation,
                worker_nonce_digest,
                daemon_challenge_digest,
                authentication,
            } => self.authenticate(
                worker_pid,
                backend_generation,
                worker_nonce_digest,
                daemon_challenge_digest,
                &authentication,
                now_unix_ms,
            ),
            WorkerToDaemon::Ready {
                worker_session_digest,
            } => self.issue_next_dispatch(controller, &worker_session_digest, now_unix_ms),
            WorkerToDaemon::StartAck {
                request_id,
                request_generation,
                backend_generation,
                sequence,
                worker_session_digest,
                grant,
            } => self
                .claim_start(
                    controller,
                    &request_id,
                    request_generation,
                    backend_generation,
                    sequence,
                    &worker_session_digest,
                    &grant,
                    now_unix_ms,
                )
                .map(DaemonToWorker::State),
            WorkerToDaemon::Token {
                request_id,
                request_generation,
                backend_generation,
                sequence,
                worker_session_digest,
                token_digest,
                token_byte_length,
            } => self
                .publish_token(
                    controller,
                    &request_id,
                    request_generation,
                    backend_generation,
                    sequence,
                    &worker_session_digest,
                    &token_digest,
                    token_byte_length,
                )
                .map(DaemonToWorker::State),
            WorkerToDaemon::Complete {
                request_id,
                request_generation,
                backend_generation,
                sequence,
                worker_session_digest,
                result_digest,
                output_tokens,
            } => self
                .complete(
                    controller,
                    &request_id,
                    request_generation,
                    backend_generation,
                    sequence,
                    &worker_session_digest,
                    result_digest,
                    output_tokens,
                )
                .map(DaemonToWorker::Receipt),
        }
    }

    fn begin_handshake(
        &mut self,
        worker_pid: u32,
        backend_generation: u64,
        worker_nonce_digest: Digest,
        now_unix_ms: u64,
    ) -> Result<DaemonToWorker> {
        self.validate_generation(backend_generation)?;
        if worker_pid == 0 || self.session.is_some() {
            return Err(InferError::InvalidCapability);
        }
        if self
            .pending_handshake
            .as_ref()
            .is_some_and(|pending| pending.expires_at_unix_ms > now_unix_ms)
        {
            return Err(InferError::InvalidTransition);
        }
        let expires_at_unix_ms = now_unix_ms
            .checked_add(duration_ms(self.config.handshake_ttl)?)
            .ok_or(InferError::InvalidCapability)?;
        let daemon_challenge_digest = generate_daemon_challenge_digest()?;
        self.pending_handshake = Some(PendingHandshake {
            worker_pid,
            backend_generation,
            worker_nonce_digest,
            daemon_challenge_digest: daemon_challenge_digest.clone(),
            expires_at_unix_ms,
        });
        Ok(DaemonToWorker::Challenge {
            backend_generation,
            daemon_challenge_digest,
            expires_at_unix_ms,
        })
    }

    fn authenticate(
        &mut self,
        worker_pid: u32,
        backend_generation: u64,
        worker_nonce_digest: Digest,
        daemon_challenge_digest: Digest,
        authentication: &codex_hepta_infer_core::WorkerAuthenticationTag,
        now_unix_ms: u64,
    ) -> Result<DaemonToWorker> {
        self.validate_generation(backend_generation)?;
        let pending = self
            .pending_handshake
            .as_ref()
            .ok_or(InferError::CapabilityUnknown)?;
        if now_unix_ms >= pending.expires_at_unix_ms {
            self.pending_handshake = None;
            return Err(InferError::CapabilityExpired);
        }
        if pending.worker_pid != worker_pid
            || pending.backend_generation != backend_generation
            || pending.worker_nonce_digest != worker_nonce_digest
            || pending.daemon_challenge_digest != daemon_challenge_digest
        {
            return Err(InferError::InvalidCapability);
        }
        let fence = WorkerHandshakeFence {
            worker_pid,
            backend_generation,
            worker_nonce_digest: &worker_nonce_digest,
            daemon_challenge_digest: &daemon_challenge_digest,
        };
        if !self
            .bootstrap_key
            .verify_worker_authentication(fence, authentication)
        {
            return Err(InferError::InvalidCapability);
        }
        let digest = self.bootstrap_key.worker_session_digest(fence)?;
        self.session = Some(WorkerSession {
            backend_generation,
            digest: digest.clone(),
        });
        self.pending_handshake = None;
        Ok(DaemonToWorker::Authenticated {
            worker_session_digest: digest,
        })
    }

    fn issue_next_dispatch(
        &mut self,
        controller: &Controller,
        worker_session_digest: &Digest,
        now_unix_ms: u64,
    ) -> Result<DaemonToWorker> {
        self.require_session(worker_session_digest, self.backend_generation)?;
        if self.pending_dispatch.is_some() {
            return Err(InferError::InvalidTransition);
        }
        let Some(request) = controller.next_queued_request()? else {
            return Ok(DaemonToWorker::Idle);
        };
        if controller.backend_generation() != self.backend_generation {
            return Err(InferError::StaleBackendGeneration);
        }
        let fence = RequestGrantFence {
            request: &request,
            backend_generation: self.backend_generation,
            worker_session_digest,
        };
        let grant = self.grants.issue(
            &self.epoch_key,
            fence,
            GrantPurpose::Execute,
            now_unix_ms,
            duration_ms(self.config.grant_ttl)?,
        )?;
        let grant_digest = grant.digest()?;
        self.pending_dispatch = Some(PendingDispatch {
            request_id: request.identity.request_id.clone(),
            request_generation: request.request_generation,
            backend_generation: self.backend_generation,
            worker_session_digest: worker_session_digest.clone(),
            grant_digest,
        });
        Ok(DaemonToWorker::Dispatch {
            request,
            backend_generation: self.backend_generation,
            worker_session_digest: worker_session_digest.clone(),
            grant,
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn claim_start(
        &mut self,
        controller: &mut Controller,
        request_id: &RequestId,
        request_generation: u64,
        backend_generation: u64,
        sequence: u64,
        worker_session_digest: &Digest,
        grant: &codex_hepta_infer_core::RequestGrant,
        now_unix_ms: u64,
    ) -> Result<codex_hepta_infer_core::StateEvent> {
        self.validate_generation(backend_generation)?;
        self.require_session(worker_session_digest, backend_generation)?;
        if self.active.contains_key(request_id) {
            return Err(InferError::CapabilityReplay);
        }
        let request = controller.request_clone(request_id, request_generation)?;
        let pending = self
            .pending_dispatch
            .as_ref()
            .ok_or(InferError::CapabilityUnknown)?;
        let grant_digest = grant.digest()?;
        if pending.request_id != *request_id
            || pending.request_generation != request_generation
            || pending.backend_generation != backend_generation
            || pending.worker_session_digest != *worker_session_digest
            || pending.grant_digest != grant_digest
        {
            return Err(InferError::InvalidCapability);
        }
        if controller.expected_next_sequence(request_id, request_generation)? != sequence {
            return Err(InferError::StaleSequence);
        }
        self.grants.claim(
            &self.epoch_key,
            RequestGrantFence {
                request: &request,
                backend_generation,
                worker_session_digest,
            },
            GrantPurpose::Execute,
            now_unix_ms,
            grant,
        )?;
        let event = controller.start(request_id, request_generation, backend_generation)?;
        self.pending_dispatch = None;
        self.active.insert(
            request_id.clone(),
            ActiveLease {
                request_generation,
                backend_generation,
                worker_session_digest: worker_session_digest.clone(),
            },
        );
        Ok(event)
    }

    #[allow(clippy::too_many_arguments)]
    fn publish_token(
        &mut self,
        controller: &mut Controller,
        request_id: &RequestId,
        request_generation: u64,
        backend_generation: u64,
        sequence: u64,
        worker_session_digest: &Digest,
        token_digest: &Digest,
        token_byte_length: u64,
    ) -> Result<codex_hepta_infer_core::StateEvent> {
        self.require_active(
            request_id,
            request_generation,
            backend_generation,
            worker_session_digest,
        )?;
        controller.publish_token(
            EventFence {
                request_id,
                request_generation,
                backend_generation,
                sequence,
            },
            token_digest,
            token_byte_length,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn complete(
        &mut self,
        controller: &mut Controller,
        request_id: &RequestId,
        request_generation: u64,
        backend_generation: u64,
        sequence: u64,
        worker_session_digest: &Digest,
        result_digest: Digest,
        output_tokens: u32,
    ) -> Result<codex_hepta_infer_core::TerminalReceipt> {
        self.require_active(
            request_id,
            request_generation,
            backend_generation,
            worker_session_digest,
        )?;
        let receipt = controller.complete(
            EventFence {
                request_id,
                request_generation,
                backend_generation,
                sequence,
            },
            result_digest,
            output_tokens,
        )?;
        self.active.remove(request_id);
        Ok(receipt)
    }

    fn validate_generation(&self, backend_generation: u64) -> Result<()> {
        if backend_generation == self.backend_generation {
            Ok(())
        } else {
            Err(InferError::StaleBackendGeneration)
        }
    }

    fn require_session(
        &self,
        worker_session_digest: &Digest,
        backend_generation: u64,
    ) -> Result<()> {
        let session = self.session.as_ref().ok_or(InferError::InvalidCapability)?;
        if session.backend_generation == backend_generation
            && session.digest == *worker_session_digest
        {
            Ok(())
        } else {
            Err(InferError::InvalidCapability)
        }
    }

    fn require_active(
        &self,
        request_id: &RequestId,
        request_generation: u64,
        backend_generation: u64,
        worker_session_digest: &Digest,
    ) -> Result<()> {
        self.validate_generation(backend_generation)?;
        self.require_session(worker_session_digest, backend_generation)?;
        let active = self
            .active
            .get(request_id)
            .ok_or(InferError::InvalidCapability)?;
        if active.request_generation == request_generation
            && active.backend_generation == backend_generation
            && active.worker_session_digest == *worker_session_digest
        {
            Ok(())
        } else {
            Err(InferError::InvalidCapability)
        }
    }
}

pub(crate) trait PrivateWorkerIo: AsyncRead + AsyncWrite + Unpin + Send {}

impl<T> PrivateWorkerIo for T where T: AsyncRead + AsyncWrite + Unpin + Send {}

pub(crate) struct PrivateWorkerAttachment {
    server: PrivateWorkerServer,
    stream: Box<dyn PrivateWorkerIo>,
}

impl PrivateWorkerAttachment {
    pub(crate) fn new<S>(server: PrivateWorkerServer, stream: S) -> Self
    where
        S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    {
        Self {
            server,
            stream: Box::new(stream),
        }
    }
}

pub(crate) async fn run_private_worker_channel(
    mut attachment: PrivateWorkerAttachment,
    controller: Arc<Mutex<Controller>>,
    receipt_store: Arc<ReceiptStore>,
) -> io::Result<()> {
    loop {
        let bytes = time::timeout(
            attachment.server.config.io_timeout,
            read_private_frame(
                &mut *attachment.stream,
                attachment.server.config.max_frame_bytes,
            ),
        )
        .await
        .map_err(|_| io::Error::new(ErrorKind::TimedOut, "INF_PRIVATE_READ_TIMEOUT"))??;
        let message = WorkerToDaemon::decode_canonical(&bytes).map_err(infer_error_to_io)?;
        let now_unix_ms = unix_time_ms()?;
        let result = {
            let mut controller = controller.lock().await;
            attachment
                .server
                .process_message(&mut controller, message, now_unix_ms)
        };
        let (response, close_code) = match result {
            Ok(response) => (response, None),
            Err(error) => (
                DaemonToWorker::Error {
                    code: error.code().to_owned(),
                },
                Some(error.code()),
            ),
        };
        if let DaemonToWorker::Receipt(receipt) = &response {
            persist_and_forget(
                &receipt_store,
                &controller,
                std::slice::from_ref(receipt),
            )
            .await?;
        }
        let bytes = response.encode_canonical().map_err(infer_error_to_io)?;
        time::timeout(
            attachment.server.config.io_timeout,
            write_private_frame(
                &mut *attachment.stream,
                &bytes,
                attachment.server.config.max_frame_bytes,
            ),
        )
        .await
        .map_err(|_| io::Error::new(ErrorKind::TimedOut, "INF_PRIVATE_WRITE_TIMEOUT"))??;
        if let Some(code) = close_code {
            return Err(io::Error::new(ErrorKind::PermissionDenied, code));
        }
    }
}

async fn read_private_frame<S>(stream: &mut S, max_frame_bytes: usize) -> io::Result<Vec<u8>>
where
    S: AsyncRead + Unpin + ?Sized,
{
    let mut length_bytes = [0u8; 4];
    stream.read_exact(&mut length_bytes).await?;
    let length = usize::try_from(u32::from_be_bytes(length_bytes))
        .map_err(|_| io::Error::new(ErrorKind::InvalidData, "INF_PRIVATE_FRAME_LENGTH"))?;
    if length == 0 || length > max_frame_bytes {
        return Err(io::Error::new(
            ErrorKind::InvalidData,
            "INF_PRIVATE_FRAME_BOUND",
        ));
    }
    let mut bytes = vec![0u8; length];
    stream.read_exact(&mut bytes).await?;
    Ok(bytes)
}

async fn write_private_frame<S>(
    stream: &mut S,
    bytes: &[u8],
    max_frame_bytes: usize,
) -> io::Result<()>
where
    S: AsyncWrite + Unpin + ?Sized,
{
    if bytes.is_empty() || bytes.len() > max_frame_bytes {
        return Err(io::Error::new(
            ErrorKind::InvalidData,
            "INF_PRIVATE_FRAME_BOUND",
        ));
    }
    let length = u32::try_from(bytes.len())
        .map_err(|_| io::Error::new(ErrorKind::InvalidData, "INF_PRIVATE_FRAME_LENGTH"))?;
    stream.write_all(&length.to_be_bytes()).await?;
    stream.write_all(bytes).await?;
    stream.flush().await
}

fn duration_ms(duration: Duration) -> Result<u64> {
    u64::try_from(duration.as_millis()).map_err(|_| InferError::InvalidCapability)
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use codex_hepta_infer_core::AgentId;
    use codex_hepta_infer_core::AuthoritySnapshot;
    use codex_hepta_infer_core::ControllerConfig;
    use codex_hepta_infer_core::InferenceRequest;
    use codex_hepta_infer_core::LifecycleState;
    use codex_hepta_infer_core::RequestIdentity;
    use codex_hepta_infer_core::ResourceBudgetId;
    use codex_hepta_infer_core::TaskId;
    use codex_hepta_infer_core::TenantId;
    use codex_hepta_infer_core::WorkerHandshakeFence;
    use codex_hepta_infer_core::WorkspaceId;
    use codex_hepta_infer_core::advance_token_chain_digest;
    use codex_hepta_infer_core::generate_worker_nonce_digest;
    use codex_hepta_infer_core::initial_token_chain_digest;
    use temp_file::TempDir;
    use tokio::io::DuplexStream;

    use super::*;

    fn must<T, E: std::fmt::Display>(result: std::result::Result<T, E>) -> T {
        match result {
            Ok(value) => value,
            Err(error) => panic!("unexpected error: {error}"),
        }
    }

    fn digest(fill: char) -> Digest {
        must(Digest::parse(&format!(
            "sha256:{}",
            fill.to_string().repeat(64)
        )))
    }

    fn request(tuple: Digest) -> InferenceRequest {
        InferenceRequest {
            identity: RequestIdentity {
                tenant_id: must(TenantId::parse("tenant-a")),
                workspace_id: must(WorkspaceId::parse("workspace-a")),
                agent_id: must(AgentId::parse("agent-a")),
                task_id: must(TaskId::parse("task-a")),
                request_id: must(RequestId::parse("request-private-e2e")),
            },
            agent_generation: 1,
            request_generation: 1,
            cancel_generation: 0,
            deadline_unix_ms: 10_000,
            model_tuple_digest: tuple,
            policy_digest: digest('b'),
            resource_budget_id: must(ResourceBudgetId::parse("budget-a")),
            prompt_digest: digest('c'),
            prompt_byte_length: 12,
            output_token_limit: 4,
            authority: AuthoritySnapshot::qualification_only_closed(),
        }
    }

    fn controller(tuple: Digest) -> Controller {
        let mut registered_tuples = HashSet::new();
        registered_tuples.insert(tuple);
        must(Controller::new(
            ControllerConfig {
                max_queue: 4,
                max_per_tenant: 2,
                registered_tuples,
                authority: AuthoritySnapshot::qualification_only_closed(),
            },
            7,
        ))
    }

    async fn send_worker(stream: &mut DuplexStream, message: &WorkerToDaemon) {
        let bytes = must(message.encode_canonical());
        must(write_private_frame(stream, &bytes, MAX_PRIVATE_FRAME_BYTES).await);
    }

    async fn receive_daemon(stream: &mut DuplexStream) -> DaemonToWorker {
        let bytes = must(read_private_frame(stream, MAX_PRIVATE_FRAME_BYTES).await);
        must(DaemonToWorker::decode_canonical(&bytes))
    }

    #[tokio::test]
    async fn inherited_private_stream_executes_one_fenced_request_and_persists_receipt() {
        let tuple = digest('a');
        let request = request(tuple.clone());
        let request_id = request.identity.request_id.clone();
        let controller = Arc::new(Mutex::new(controller(tuple)));
        must(controller.lock().await.admit(request.clone(), 1));

        let temp = must(TempDir::new());
        let receipt_dir = temp.path().join("receipts");
        must(tokio::fs::create_dir_all(&receipt_dir).await);
        let store = Arc::new(must(ReceiptStore::open(receipt_dir, 16, 1_048_576).await));
        let (server, token) = must(PrivateWorkerServer::new_random(
            7,
            PrivateWorkerConfig::default(),
        ));
        let worker_key = must(token.into_capability_key());
        let (daemon_stream, mut worker_stream) = tokio::io::duplex(2 * MAX_PRIVATE_FRAME_BYTES);
        let task = tokio::spawn(run_private_worker_channel(
            PrivateWorkerAttachment::new(server, daemon_stream),
            Arc::clone(&controller),
            Arc::clone(&store),
        ));

        let worker_nonce_digest = must(generate_worker_nonce_digest());
        send_worker(
            &mut worker_stream,
            &WorkerToDaemon::Hello {
                worker_pid: 42,
                backend_generation: 7,
                worker_nonce_digest: worker_nonce_digest.clone(),
            },
        )
        .await;
        let (daemon_challenge_digest, expires_at_unix_ms) = match receive_daemon(&mut worker_stream).await {
            DaemonToWorker::Challenge {
                backend_generation,
                daemon_challenge_digest,
                expires_at_unix_ms,
            } => {
                assert_eq!(backend_generation, 7);
                (daemon_challenge_digest, expires_at_unix_ms)
            }
            other => panic!("unexpected challenge response: {other:?}"),
        };
        assert!(expires_at_unix_ms > 0);
        let authentication = must(worker_key.derive_worker_authentication(
            WorkerHandshakeFence {
                worker_pid: 42,
                backend_generation: 7,
                worker_nonce_digest: &worker_nonce_digest,
                daemon_challenge_digest: &daemon_challenge_digest,
            },
        ));
        send_worker(
            &mut worker_stream,
            &WorkerToDaemon::Authenticate {
                worker_pid: 42,
                backend_generation: 7,
                worker_nonce_digest,
                daemon_challenge_digest,
                authentication,
            },
        )
        .await;
        let session = match receive_daemon(&mut worker_stream).await {
            DaemonToWorker::Authenticated {
                worker_session_digest,
            } => worker_session_digest,
            other => panic!("unexpected authentication response: {other:?}"),
        };

        send_worker(
            &mut worker_stream,
            &WorkerToDaemon::Ready {
                worker_session_digest: session.clone(),
            },
        )
        .await;
        let grant = match receive_daemon(&mut worker_stream).await {
            DaemonToWorker::Dispatch {
                request: dispatched,
                backend_generation,
                worker_session_digest,
                grant,
            } => {
                assert_eq!(dispatched, request);
                assert_eq!(backend_generation, 7);
                assert_eq!(worker_session_digest, session);
                grant
            }
            other => panic!("unexpected dispatch response: {other:?}"),
        };

        send_worker(
            &mut worker_stream,
            &WorkerToDaemon::StartAck {
                request_id: request_id.clone(),
                request_generation: 1,
                backend_generation: 7,
                sequence: 2,
                worker_session_digest: session.clone(),
                grant,
            },
        )
        .await;
        match receive_daemon(&mut worker_stream).await {
            DaemonToWorker::State(event) => {
                assert_eq!(event.sequence, 2);
                assert_eq!(event.state, LifecycleState::Running);
            }
            other => panic!("unexpected start response: {other:?}"),
        }

        let token_digest = digest('d');
        let initial = must(initial_token_chain_digest(&request, 7));
        let final_digest = must(advance_token_chain_digest(
            &initial,
            3,
            &token_digest,
            2,
        ));
        send_worker(
            &mut worker_stream,
            &WorkerToDaemon::Token {
                request_id: request_id.clone(),
                request_generation: 1,
                backend_generation: 7,
                sequence: 3,
                worker_session_digest: session.clone(),
                token_digest,
                token_byte_length: 2,
            },
        )
        .await;
        match receive_daemon(&mut worker_stream).await {
            DaemonToWorker::State(event) => assert_eq!(event.sequence, 3),
            other => panic!("unexpected token response: {other:?}"),
        }

        send_worker(
            &mut worker_stream,
            &WorkerToDaemon::Complete {
                request_id: request_id.clone(),
                request_generation: 1,
                backend_generation: 7,
                sequence: 4,
                worker_session_digest: session,
                result_digest: final_digest.clone(),
                output_tokens: 1,
            },
        )
        .await;
        let receipt = match receive_daemon(&mut worker_stream).await {
            DaemonToWorker::Receipt(receipt) => receipt,
            other => panic!("unexpected completion response: {other:?}"),
        };
        assert_eq!(receipt.result_digest, Some(final_digest));
        assert_eq!(receipt.accepted_token_count, 1);
        assert_eq!(
            must(store.lookup(&request_id, 1, 7, 4).await),
            receipt
        );
        assert!(matches!(
            controller.lock().await.terminal_receipt(&request_id),
            Err(InferError::UnknownRequest)
        ));

        task.abort();
        let _ = task.await;
    }

    #[test]
    fn wrong_bootstrap_and_generation_rotation_fail_closed() {
        let tuple = digest('a');
        let mut controller = controller(tuple);
        let (mut server, _token) = must(PrivateWorkerServer::new_random(
            7,
            PrivateWorkerConfig::default(),
        ));
        let wrong_key = must(CapabilityKey::generate());
        let nonce = must(generate_worker_nonce_digest());
        let challenge = match must(server.process_message(
            &mut controller,
            WorkerToDaemon::Hello {
                worker_pid: 42,
                backend_generation: 7,
                worker_nonce_digest: nonce.clone(),
            },
            100,
        )) {
            DaemonToWorker::Challenge {
                daemon_challenge_digest,
                ..
            } => daemon_challenge_digest,
            other => panic!("unexpected challenge: {other:?}"),
        };
        let authentication = must(wrong_key.derive_worker_authentication(
            WorkerHandshakeFence {
                worker_pid: 42,
                backend_generation: 7,
                worker_nonce_digest: &nonce,
                daemon_challenge_digest: &challenge,
            },
        ));
        assert!(matches!(
            server.process_message(
                &mut controller,
                WorkerToDaemon::Authenticate {
                    worker_pid: 42,
                    backend_generation: 7,
                    worker_nonce_digest: nonce,
                    daemon_challenge_digest: challenge,
                    authentication,
                },
                101,
            ),
            Err(InferError::InvalidCapability)
        ));
        let _new_token = must(server.rotate_generation(8));
        assert_eq!(server.backend_generation(), 8);
        assert_eq!(server.active_leases(), 0);
        assert!(matches!(
            server.process_message(
                &mut controller,
                WorkerToDaemon::Hello {
                    worker_pid: 42,
                    backend_generation: 7,
                    worker_nonce_digest: digest('e'),
                },
                102,
            ),
            Err(InferError::StaleBackendGeneration)
        ));
    }
}
'''


def replace_once(path: Path, old: str, new: str) -> None:
    text = path.read_text(encoding="utf-8")
    count = text.count(old)
    if count != 1:
        raise SystemExit(f"{path}: expected one occurrence, found {count}: {old[:100]!r}")
    path.write_text(text.replace(old, new, 1), encoding="utf-8")


def main() -> None:
    (ROOT / "codex-rs/hepta-inferd/src/private.rs").write_text(PRIVATE, encoding="utf-8")

    controller = ROOT / "codex-rs/hepta-infer-core/src/controller.rs"
    replace_once(
        controller,
        '''    pub const fn inflight_requests(&self) -> usize {
        self.inflight_requests
    }

''',
        '''    pub const fn inflight_requests(&self) -> usize {
        self.inflight_requests
    }

    pub fn next_queued_request(&self) -> Result<Option<InferenceRequest>> {
        let Some(request_id) = self.queue.front() else {
            return Ok(None);
        };
        let record = self.records.get(request_id).ok_or(InferError::QueueInvariant)?;
        if record.state != LifecycleState::Queued {
            return Err(InferError::QueueInvariant);
        }
        Ok(Some(record.request.clone()))
    }

    pub fn request_clone(
        &self,
        request_id: &RequestId,
        request_generation: u64,
    ) -> Result<InferenceRequest> {
        Ok(self.record(request_id, request_generation)?.request.clone())
    }

    pub fn expected_next_sequence(
        &self,
        request_id: &RequestId,
        request_generation: u64,
    ) -> Result<u64> {
        self.record(request_id, request_generation)?
            .last_sequence
            .checked_add(1)
            .ok_or(InferError::SequenceOverflow)
    }

''',
    )
    replace_once(
        controller,
        '''fn initial_token_chain(request: &InferenceRequest, backend_generation: u64) -> Result<Digest> {
''',
        '''pub fn initial_token_chain_digest(
    request: &InferenceRequest,
    backend_generation: u64,
) -> Result<Digest> {
    initial_token_chain(request, backend_generation)
}

pub fn advance_token_chain_digest(
    previous: &Digest,
    sequence: u64,
    token_digest: &Digest,
    token_byte_length: u64,
) -> Result<Digest> {
    next_token_chain(previous, sequence, token_digest, token_byte_length)
}

fn initial_token_chain(request: &InferenceRequest, backend_generation: u64) -> Result<Digest> {
''',
    )

    core_root = ROOT / "codex-rs/hepta-infer-core/src/lib.rs"
    replace_once(
        core_root,
        'pub use controller::EventFence;\n',
        'pub use controller::EventFence;\npub use controller::advance_token_chain_digest;\npub use controller::initial_token_chain_digest;\n',
    )

    daemon = ROOT / "codex-rs/hepta-inferd/src/lib.rs"
    replace_once(
        daemon,
        '''//! commands require future private capability channels and fail closed here.

''',
        '''//! commands fail closed on the public endpoint. An authenticated worker lane is
//! available only through an explicitly injected inherited bidirectional stream.

mod private;

pub use private::PrivateWorkerConfig;
pub use private::PrivateWorkerServer;

''',
    )
    old = '''pub async fn serve_with_shutdown<F>(config: DaemonConfig, shutdown: F) -> io::Result<()>
where
    F: Future<Output = ()>,
{
    config.validate()?;
'''
    new = '''pub async fn serve_with_shutdown<F>(config: DaemonConfig, shutdown: F) -> io::Result<()>
where
    F: Future<Output = ()>,
{
    serve_runtime(config, shutdown, None).await
}

pub async fn serve_with_shutdown_and_private_worker<F, S>(
    config: DaemonConfig,
    shutdown: F,
    private_worker: PrivateWorkerServer,
    stream: S,
) -> io::Result<()>
where
    F: Future<Output = ()>,
    S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin + Send + 'static,
{
    serve_runtime(
        config,
        shutdown,
        Some(private::PrivateWorkerAttachment::new(private_worker, stream)),
    )
    .await
}

async fn serve_runtime<F>(
    config: DaemonConfig,
    shutdown: F,
    private_worker: Option<private::PrivateWorkerAttachment>,
) -> io::Result<()>
where
    F: Future<Output = ()>,
{
    config.validate()?;
'''
    replace_once(daemon, old, new)
    replace_once(
        daemon,
        '''    let mut deadline_sweep = time::interval(config.deadline_sweep_interval);
    deadline_sweep.set_missed_tick_behavior(MissedTickBehavior::Delay);
    tokio::pin!(shutdown);

''',
        '''    let mut deadline_sweep = time::interval(config.deadline_sweep_interval);
    deadline_sweep.set_missed_tick_behavior(MissedTickBehavior::Delay);
    if let Some(attachment) = private_worker {
        let private_controller = Arc::clone(&shared);
        let private_receipt_store = Arc::clone(&receipt_store);
        tasks.spawn(async move {
            private::run_private_worker_channel(
                attachment,
                private_controller,
                private_receipt_store,
            )
            .await
            .map_err(ConnectionTaskError::Infrastructure)
        });
    }
    tokio::pin!(shutdown);

''',
    )


if __name__ == "__main__":
    main()
