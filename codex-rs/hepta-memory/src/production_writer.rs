//! Production durable writer/outbox capability.
//!
//! This module is the boundary between an externally-authorized supervisor
//! grant and the Agent-local durable journal. The older
//! `local_lease_outbox` API remains available for qualification and replay
//! tests; this wrapper refuses to open without an independently verified
//! authority lease and a WAL/FULL SQLite store. It does not invent a provider
//! or target effect: an embedding must explicitly attach a
//! `ProductionOutboxTarget` to dispatch a queued row.

use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use codex_hepta_contracts::AgentId;
use codex_hepta_contracts::Sha256Digest;
use serde::Deserialize;
use serde::Serialize;
use sha2::Digest;
use sha2::Sha256;

use crate::CognitiveStore;
use crate::LocalAdmission;
use crate::LocalLease;
use crate::LocalLeaseHeadDisposition;
use crate::LocalLeaseOutbox;
use crate::LocalLeaseOutboxError;
use crate::LocalOutcomeReceipt;
use crate::LocalOutcomeState;
use crate::LocalReplayFinalization;
use crate::QueuedReceipt;

/// Schema version of the externally-authorized H4 writer boundary.
pub const PRODUCTION_DURABLE_WRITER_SCHEMA_VERSION: u32 = 1;
/// Stable provenance namespace for production writer receipts.
pub const PRODUCTION_DURABLE_WRITER_NAMESPACE: &str = "production_durable_writer";
/// The store opened by `CognitiveStore` must use this journal mode.
pub const PRODUCTION_DURABLE_WRITER_JOURNAL_MODE: &str = "wal";
/// SQLite `PRAGMA synchronous` value for FULL.
pub const PRODUCTION_DURABLE_WRITER_SYNCHRONOUS_FULL: i64 = 2;

/// Errors returned by the production writer and dispatcher boundary.
#[derive(Debug, thiserror::Error)]
pub enum ProductionWriterError {
    #[error(transparent)]
    Local(#[from] LocalLeaseOutboxError),
    #[error("production authority rejected: {0}")]
    AuthorityRejected(String),
    #[error("production authority lease expired at {deadline}")]
    AuthorityExpired { deadline: u64 },
    #[error("production authority lease does not match Agent {0}")]
    AuthorityAgentMismatch(AgentId),
    #[error("production writer input is invalid: {0}")]
    Invalid(String),
    #[error("production writer durability precondition failed: {0}")]
    Durability(String),
    #[error("production writer receipt is stale or belongs to another authority")]
    StaleReceipt,
}

/// Opaque authority token supplied by an external grant verifier.
///
/// There is deliberately no seed/random/default constructor. The only public
/// constructor is named `from_verified_bytes`; callers are expected to obtain
/// the bytes from a supervisor/grant verifier. The token is never rendered or
/// serialized; only a one-way digest is used as the local SQLite fencing token.
#[derive(Clone, Eq, PartialEq)]
pub struct ProductionAuthorityToken(Arc<[u8]>);

impl ProductionAuthorityToken {
    pub fn from_verified_bytes(bytes: impl Into<Vec<u8>>) -> Result<Self, ProductionWriterError> {
        let bytes = bytes.into();
        if bytes.is_empty() || bytes.len() > 4096 || bytes.contains(&0) {
            return Err(ProductionWriterError::Invalid(
                "authority token must contain 1..=4096 non-NUL bytes".to_string(),
            ));
        }
        Ok(Self(Arc::from(bytes)))
    }

    fn fencing_digest(&self) -> Sha256Digest {
        let mut hasher = Sha256::new();
        hasher.update(b"hepta:production-authority-token:v1\0");
        hasher.update(self.0.as_ref());
        Sha256Digest::for_bytes(&hasher.finalize())
    }
}

impl fmt::Debug for ProductionAuthorityToken {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ProductionAuthorityToken")
            .field("digest", &self.fencing_digest())
            .finish()
    }
}

/// Minimum externally supplied authority material needed by H4.
///
/// `grant_digest` identifies the signed supervisor/OPE grant. The opaque token
/// is supplied by that verifier and is not derived from a local seed. Epochs
/// and expiry are persisted in the lease chain and checked on each mutation.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProductionAuthorityLease {
    pub agent_id: AgentId,
    pub grant_digest: Sha256Digest,
    pub authority_epoch: u64,
    pub owner_epoch: u64,
    pub lease_expires_at_unix_seconds: u64,
    #[serde(skip)]
    token: Option<ProductionAuthorityToken>,
}

impl fmt::Debug for ProductionAuthorityLease {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ProductionAuthorityLease")
            .field("agent_id", &self.agent_id)
            .field("grant_digest", &self.grant_digest)
            .field("authority_epoch", &self.authority_epoch)
            .field("owner_epoch", &self.owner_epoch)
            .field("lease_expires_at_unix_seconds", &self.lease_expires_at_unix_seconds)
            .field("token", &self.token.as_ref().map(|token| token.fencing_digest()))
            .finish()
    }
}

impl ProductionAuthorityLease {
    /// Construct lease material after an external verifier has checked the
    /// signed grant and supplied its opaque token. This performs shape checks
    /// only; `ProductionDurableWriter::open` still requires a verifier.
    pub fn from_verified_parts(
        agent_id: AgentId,
        grant_digest: Sha256Digest,
        authority_epoch: u64,
        owner_epoch: u64,
        lease_expires_at_unix_seconds: u64,
        token: ProductionAuthorityToken,
    ) -> Result<Self, ProductionWriterError> {
        if authority_epoch == 0 || owner_epoch == 0 {
            return Err(ProductionWriterError::Invalid(
                "authority and owner epochs must be non-zero".to_string(),
            ));
        }
        if lease_expires_at_unix_seconds == 0 {
            return Err(ProductionWriterError::Invalid(
                "authority lease expiry must be non-zero".to_string(),
            ));
        }
        Ok(Self {
            agent_id,
            grant_digest,
            authority_epoch,
            owner_epoch,
            lease_expires_at_unix_seconds,
            token: Some(token),
        })
    }

    pub fn fencing_token_digest(&self) -> Result<Sha256Digest, ProductionWriterError> {
        self.token
            .as_ref()
            .map(ProductionAuthorityToken::fencing_digest)
            .ok_or_else(|| {
                ProductionWriterError::AuthorityRejected(
                    "deserialized authority lease has no opaque token".to_string(),
                )
            })
    }

    pub fn is_expired_at(&self, now_unix_seconds: u64) -> bool {
        now_unix_seconds >= self.lease_expires_at_unix_seconds
    }

    fn validate_for_agent(&self, agent_id: &AgentId) -> Result<(), ProductionWriterError> {
        if &self.agent_id != agent_id {
            return Err(ProductionWriterError::AuthorityAgentMismatch(agent_id.clone()));
        }
        let now = now_unix_seconds()?;
        if self.is_expired_at(now) {
            return Err(ProductionWriterError::AuthorityExpired {
                deadline: self.lease_expires_at_unix_seconds,
            });
        }
        let _ = self.fencing_token_digest()?;
        Ok(())
    }
}

/// External verifier hook. Implementations should verify the signed grant,
/// scope, epoch, and opaque token before returning `Ok(())`.
///
/// The writer never treats a boolean field on the lease as authority and has
/// no built-in/self-signing implementation of this trait.
pub trait ProductionAuthorityVerifier: Send + Sync {
    fn verify(
        &self,
        authority: &ProductionAuthorityLease,
        expected_agent: &AgentId,
    ) -> Result<(), String>;
}

impl<F> ProductionAuthorityVerifier for F
where
    F: Fn(&ProductionAuthorityLease, &AgentId) -> Result<(), String> + Send + Sync,
{
    fn verify(
        &self,
        authority: &ProductionAuthorityLease,
        expected_agent: &AgentId,
    ) -> Result<(), String> {
        self(authority, expected_agent)
    }
}

/// Durable writer bound to one externally-authorized lease.
#[derive(Clone)]
pub struct ProductionDurableWriter {
    store: CognitiveStore,
    authority: ProductionAuthorityLease,
    lease: LocalLeaseOutbox,
    lease_id: Arc<str>,
}

impl fmt::Debug for ProductionDurableWriter {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ProductionDurableWriter")
            .field("lease_id", &self.lease_id)
            .field("authority", &self.authority)
            .field("lease", &self.lease)
            .finish()
    }
}

impl ProductionDurableWriter {
    /// Open a writer only after external authority verification and a WAL/FULL
    /// durability check. Acquisition/reopen is CAS-protected by the
    /// append-only lease chain.
    pub async fn open<V>(
        store: CognitiveStore,
        authority: ProductionAuthorityLease,
        verifier: &V,
        lease_id: impl Into<String>,
        generation: u64,
    ) -> Result<Self, ProductionWriterError>
    where
        V: ProductionAuthorityVerifier + ?Sized,
    {
        let lease_id = lease_id.into();
        validate_text(&lease_id, "production lease id", 512)?;
        verifier
            .verify(&authority, store.owner_agent_id())
            .map_err(ProductionWriterError::AuthorityRejected)?;
        authority.validate_for_agent(store.owner_agent_id())?;
        verify_durable_store(&store).await?;
        let fencing_token = authority.fencing_token_digest()?.as_str().to_string();
        let binding = (
            authority.authority_epoch,
            authority.owner_epoch,
            authority.lease_expires_at_unix_seconds,
        );

        let inspection = store.inspect_local_lease_head(&lease_id).await?;
        let lease = match (inspection.disposition, inspection.head) {
            (LocalLeaseHeadDisposition::Missing, None) => {
                store
                    .acquire_host_bound_lease(
                        &lease_id,
                        binding.0,
                        binding.1,
                        generation,
                        fencing_token,
                        binding.2,
                    )
                    .await?
                    .into_handle()
            }
            (LocalLeaseHeadDisposition::Active, Some(head)) => {
                if head.generation != generation
                    || head.fencing_token != authority.fencing_token_digest()?.as_str()
                    || head.authority_epoch != Some(binding.0)
                    || head.owner_epoch != Some(binding.1)
                    || head.lease_expires_at_unix_seconds != Some(binding.2)
                {
                    return Err(ProductionWriterError::StaleReceipt);
                }
                store
                    .reopen_host_bound_lease(head, binding.0, binding.1, binding.2)
                    .await?
            }
            (LocalLeaseHeadDisposition::ExpiredActive, Some(_)) => {
                return Err(ProductionWriterError::AuthorityExpired {
                    deadline: authority.lease_expires_at_unix_seconds,
                });
            }
            (
                LocalLeaseHeadDisposition::Released | LocalLeaseHeadDisposition::RolledBack,
                Some(head),
            ) => {
                store
                    .acquire_host_bound_lease_after_head(
                        &lease_id,
                        head,
                        binding.0,
                        binding.1,
                        generation,
                        fencing_token,
                        binding.2,
                    )
                    .await?
                    .into_handle()
            }
            (LocalLeaseHeadDisposition::Missing, Some(_)) => {
                return Err(ProductionWriterError::StaleReceipt);
            }
            (_, None) => return Err(ProductionWriterError::StaleReceipt),
        };
        Ok(Self {
            store,
            authority,
            lease,
            lease_id: Arc::from(lease_id),
        })
    }

    pub fn authority(&self) -> &ProductionAuthorityLease {
        &self.authority
    }

    pub fn lease_id(&self) -> &str {
        &self.lease_id
    }

    pub fn generation(&self) -> u64 {
        self.lease.generation()
    }

    pub fn store(&self) -> &CognitiveStore {
        &self.store
    }

    pub async fn admit(
        &self,
        occurrence_key: impl Into<String>,
        topic: impl Into<String>,
        payload_json: impl Into<String>,
    ) -> Result<ProductionQueuedReceipt, ProductionWriterError> {
        self.verify_authority().await?;
        let occurrence_key = occurrence_key.into();
        let topic = topic.into();
        let payload_json = payload_json.into();
        let admission = self.lease.admit(&occurrence_key, &topic, &payload_json).await?;
        let (receipt, replayed) = match admission {
            LocalAdmission::Queued(receipt) => (receipt, false),
            LocalAdmission::Replay(receipt) => (receipt, true),
        };
        ProductionQueuedReceipt::from_local(
            &self.authority,
            &topic,
            &payload_json,
            receipt,
            replayed,
        )
    }

    pub async fn recover(
        &self,
        occurrence_key: impl Into<String>,
    ) -> Result<ProductionRecoveryReceipt, ProductionWriterError> {
        self.verify_authority().await?;
        let result = self.lease.finalize_replayed_occurrence(occurrence_key).await?;
        Ok(ProductionRecoveryReceipt::from_local(
            &self.authority,
            self.lease_id(),
            result,
        ))
    }

    pub async fn status(
        &self,
        occurrence_key: impl Into<String>,
    ) -> Result<LocalOutcomeState, ProductionWriterError> {
        self.verify_authority().await?;
        Ok(self.lease.status(occurrence_key).await?)
    }

    pub async fn mark_indeterminate(
        &self,
        occurrence_key: impl Into<String>,
        reason: impl Into<String>,
    ) -> Result<ProductionOutcomeReceipt, ProductionWriterError> {
        self.verify_authority().await?;
        Ok(self.lease.mark_indeterminate(occurrence_key, reason).await?.into())
    }

    pub async fn apply(
        &self,
        occurrence_key: impl Into<String>,
        receipt: impl Into<String>,
    ) -> Result<ProductionOutcomeReceipt, ProductionWriterError> {
        self.verify_authority().await?;
        Ok(self.lease.apply(occurrence_key, receipt).await?.into())
    }

    pub async fn reject(
        &self,
        occurrence_key: impl Into<String>,
        reason: impl Into<String>,
    ) -> Result<ProductionOutcomeReceipt, ProductionWriterError> {
        self.verify_authority().await?;
        Ok(self.lease.reject(occurrence_key, reason).await?.into())
    }

    pub async fn rollback_occurrence(
        &self,
        occurrence_key: impl Into<String>,
        reason: impl Into<String>,
    ) -> Result<ProductionOutcomeReceipt, ProductionWriterError> {
        self.verify_authority().await?;
        Ok(self.lease.rollback_occurrence(occurrence_key, reason).await?.into())
    }

    pub async fn release(&self) -> Result<ProductionLeaseReceipt, ProductionWriterError> {
        self.verify_authority().await?;
        let head = self.lease.release().await?;
        Ok(ProductionLeaseReceipt::new(&self.authority, head, "released"))
    }

    pub async fn rollback_lease(&self) -> Result<ProductionLeaseReceipt, ProductionWriterError> {
        self.verify_authority().await?;
        let head = self.lease.rollback_lease().await?;
        Ok(ProductionLeaseReceipt::new(&self.authority, head, "rolled_back"))
    }

    async fn verify_authority(&self) -> Result<(), ProductionWriterError> {
        self.authority.validate_for_agent(self.store.owner_agent_id())?;
        verify_durable_store(&self.store).await?;
        self.lease.verify_current().await?;
        Ok(())
    }

    fn validate_queued_receipt(
        &self,
        receipt: &ProductionQueuedReceipt,
    ) -> Result<(), ProductionWriterError> {
        if receipt.schema_version != PRODUCTION_DURABLE_WRITER_SCHEMA_VERSION
            || receipt.namespace != PRODUCTION_DURABLE_WRITER_NAMESPACE
            || receipt.lease_id != self.lease_id()
            || receipt.owner_agent_id != *self.store.owner_agent_id()
            || receipt.authority_grant_digest != self.authority.grant_digest
            || receipt.authority_epoch != self.authority.authority_epoch
            || receipt.owner_epoch != self.authority.owner_epoch
            || receipt.generation != self.lease.generation()
            || receipt.fencing_token_digest != self.authority.fencing_token_digest()?
        {
            return Err(ProductionWriterError::StaleReceipt);
        }
        if Sha256Digest::for_bytes(receipt.payload_json.as_bytes()) != receipt.payload_sha256 {
            return Err(ProductionWriterError::StaleReceipt);
        }
        Ok(())
    }

    async fn dispatch_target<T: ProductionOutboxTarget + ?Sized>(
        &self,
        target: &T,
        receipt: ProductionQueuedReceipt,
    ) -> Result<ProductionDispatchReceipt, ProductionWriterError> {
        self.verify_authority().await?;
        self.validate_queued_receipt(&receipt)?;
        if self.status(&receipt.occurrence_key).await? != LocalOutcomeState::Queued {
            return Err(ProductionWriterError::StaleReceipt);
        }
        let request = ProductionDispatchRequest {
            schema_version: PRODUCTION_DURABLE_WRITER_SCHEMA_VERSION,
            namespace: PRODUCTION_DURABLE_WRITER_NAMESPACE.to_string(),
            lease_id: receipt.lease_id.clone(),
            occurrence_key: receipt.occurrence_key.clone(),
            topic: receipt.topic.clone(),
            payload_json: receipt.payload_json.clone(),
            payload_sha256: receipt.payload_sha256.clone(),
            idempotency_key: receipt.occurrence_key.clone(),
            operation_digest: operation_digest(&self.authority, &receipt),
        };
        let outcome = target.dispatch(request.clone()).await;
        match outcome {
            ProductionTargetOutcome::Committed { receipt: target_receipt } => {
                let applied = self.apply(&receipt.occurrence_key, target_receipt.clone()).await;
                match applied {
                    Ok(local) => Ok(ProductionDispatchReceipt {
                        request,
                        state: LocalOutcomeState::Committed,
                        target_receipt: Some(target_receipt),
                        local_event_id: local.event_id,
                        external_effect: true,
                    }),
                    Err(error) => {
                        let _ = self
                            .mark_indeterminate(
                                &receipt.occurrence_key,
                                "target_committed_local_receipt_write_failed",
                            )
                            .await;
                        Err(error)
                    }
                }
            }
            ProductionTargetOutcome::Rejected { reason } => {
                let local = self.reject(&receipt.occurrence_key, &reason).await?;
                Ok(ProductionDispatchReceipt {
                    request,
                    state: LocalOutcomeState::Rejected,
                    target_receipt: None,
                    local_event_id: local.event_id,
                    external_effect: false,
                })
            }
            ProductionTargetOutcome::Indeterminate { reason } => {
                let local = self.mark_indeterminate(&receipt.occurrence_key, &reason).await?;
                Ok(ProductionDispatchReceipt {
                    request,
                    state: LocalOutcomeState::Indeterminate,
                    target_receipt: None,
                    local_event_id: local.event_id,
                    external_effect: false,
                })
            }
        }
    }
}

/// Queue receipt returned by the production writer. It carries enough data to
/// dispatch after a process restart without trusting mutable process state.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProductionQueuedReceipt {
    pub schema_version: u32,
    pub namespace: String,
    pub lease_id: String,
    pub occurrence_key: String,
    pub event_id: String,
    pub outbox_id: String,
    pub owner_agent_id: AgentId,
    pub authority_grant_digest: Sha256Digest,
    pub authority_epoch: u64,
    pub owner_epoch: u64,
    pub generation: u64,
    pub fencing_token_digest: Sha256Digest,
    pub topic: String,
    pub payload_json: String,
    pub payload_sha256: Sha256Digest,
    pub replayed: bool,
    /// Always false until an explicitly attached target returns committed.
    pub external_effect: bool,
}

impl ProductionQueuedReceipt {
    fn from_local(
        authority: &ProductionAuthorityLease,
        topic: &str,
        payload_json: &str,
        receipt: QueuedReceipt,
        replayed: bool,
    ) -> Result<Self, ProductionWriterError> {
        Ok(Self {
            schema_version: PRODUCTION_DURABLE_WRITER_SCHEMA_VERSION,
            namespace: PRODUCTION_DURABLE_WRITER_NAMESPACE.to_string(),
            lease_id: receipt.lease_id,
            occurrence_key: receipt.occurrence_key,
            event_id: receipt.event_id,
            outbox_id: receipt.outbox_id,
            owner_agent_id: receipt.owner_agent_id,
            authority_grant_digest: authority.grant_digest.clone(),
            authority_epoch: authority.authority_epoch,
            owner_epoch: authority.owner_epoch,
            generation: receipt.generation,
            fencing_token_digest: authority.fencing_token_digest()?,
            topic: topic.to_string(),
            payload_json: payload_json.to_string(),
            payload_sha256: receipt.payload_sha256,
            replayed,
            external_effect: false,
        })
    }
}

/// Explicit host-to-provider dispatch request. The target must implement its
/// own idempotency/status contract; this seam never silently retries unknown
/// outcomes.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProductionDispatchRequest {
    pub schema_version: u32,
    pub namespace: String,
    pub lease_id: String,
    pub occurrence_key: String,
    pub topic: String,
    pub payload_json: String,
    pub payload_sha256: Sha256Digest,
    pub idempotency_key: String,
    pub operation_digest: Sha256Digest,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProductionTargetOutcome {
    Committed { receipt: String },
    Rejected { reason: String },
    /// Unknown/timeout/provider ambiguity must remain quarantined.
    Indeterminate { reason: String },
}

pub type ProductionDispatchFuture<'a> =
    Pin<Box<dyn Future<Output = ProductionTargetOutcome> + Send + 'a>>;

pub trait ProductionOutboxTarget: Send + Sync {
    fn dispatch<'a>(&'a self, request: ProductionDispatchRequest) -> ProductionDispatchFuture<'a>;
}

/// Dispatcher that can only be used with an explicit target attachment.
#[derive(Clone)]
pub struct ProductionOutboxDispatcher {
    target: Arc<dyn ProductionOutboxTarget>,
}

impl fmt::Debug for ProductionOutboxDispatcher {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ProductionOutboxDispatcher")
            .finish_non_exhaustive()
    }
}

impl ProductionOutboxDispatcher {
    pub fn attach(target: Arc<dyn ProductionOutboxTarget>) -> Self {
        Self { target }
    }

    pub async fn dispatch(
        &self,
        writer: &ProductionDurableWriter,
        receipt: ProductionQueuedReceipt,
    ) -> Result<ProductionDispatchReceipt, ProductionWriterError> {
        writer.dispatch_target(self.target.as_ref(), receipt).await
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProductionDispatchReceipt {
    pub request: ProductionDispatchRequest,
    pub state: LocalOutcomeState,
    pub target_receipt: Option<String>,
    pub local_event_id: String,
    pub external_effect: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProductionOutcomeReceipt {
    pub lease_id: String,
    pub occurrence_key: String,
    pub state: LocalOutcomeState,
    pub event_id: String,
    pub external_effect: bool,
}

impl From<LocalOutcomeReceipt> for ProductionOutcomeReceipt {
    fn from(value: LocalOutcomeReceipt) -> Self {
        Self {
            lease_id: value.lease_id,
            occurrence_key: value.occurrence_key,
            state: value.state,
            event_id: value.event_id,
            external_effect: false,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProductionLeaseReceipt {
    pub schema_version: u32,
    pub namespace: String,
    pub lease_id: String,
    pub owner_agent_id: AgentId,
    pub generation: u64,
    pub authority_grant_digest: Sha256Digest,
    pub authority_epoch: u64,
    pub owner_epoch: u64,
    pub state: String,
    pub lease_head_digest: Sha256Digest,
    pub external_effect: bool,
}

impl ProductionLeaseReceipt {
    fn new(authority: &ProductionAuthorityLease, head: LocalLease, state: &str) -> Self {
        Self {
            schema_version: PRODUCTION_DURABLE_WRITER_SCHEMA_VERSION,
            namespace: PRODUCTION_DURABLE_WRITER_NAMESPACE.to_string(),
            lease_id: head.lease_id,
            owner_agent_id: head.owner_agent_id,
            generation: head.generation,
            authority_grant_digest: authority.grant_digest.clone(),
            authority_epoch: authority.authority_epoch,
            owner_epoch: authority.owner_epoch,
            state: state.to_string(),
            lease_head_digest: head.lease_sha256,
            external_effect: false,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProductionRecoveryReceipt {
    pub schema_version: u32,
    pub namespace: String,
    pub lease_id: String,
    pub authority_grant_digest: Sha256Digest,
    pub state: String,
    pub occurrence_key: Option<String>,
    pub external_effect: bool,
    /// A SIGKILL/crash probe is not a physical power-loss proof.
    pub physical_power_loss_claim: bool,
}

impl ProductionRecoveryReceipt {
    fn from_local(
        authority: &ProductionAuthorityLease,
        lease_id: &str,
        result: LocalReplayFinalization,
    ) -> Self {
        let (state, occurrence_key) = match result {
            LocalReplayFinalization::NotAdmitted => ("not_admitted".to_string(), None),
            LocalReplayFinalization::Queued(receipt) => {
                ("queued".to_string(), Some(receipt.occurrence_key))
            }
            LocalReplayFinalization::Released { outcome, .. } => {
                (
                    format!(
                        "released_{}",
                        match outcome {
                            LocalOutcomeState::Queued => "queued",
                            LocalOutcomeState::Indeterminate => "indeterminate",
                            LocalOutcomeState::Committed => "committed",
                            LocalOutcomeState::Rejected => "rejected",
                            LocalOutcomeState::RolledBack => "rolled_back",
                        }
                    ),
                    None,
                )
            }
        };
        Self {
            schema_version: PRODUCTION_DURABLE_WRITER_SCHEMA_VERSION,
            namespace: PRODUCTION_DURABLE_WRITER_NAMESPACE.to_string(),
            lease_id: lease_id.to_string(),
            authority_grant_digest: authority.grant_digest.clone(),
            state,
            occurrence_key,
            external_effect: false,
            physical_power_loss_claim: false,
        }
    }
}

fn operation_digest(
    authority: &ProductionAuthorityLease,
    receipt: &ProductionQueuedReceipt,
) -> Sha256Digest {
    let mut bytes = Vec::new();
    for part in [
        b"hepta:production-outbox-operation:v1".as_slice(),
        authority.grant_digest.as_str().as_bytes(),
        receipt.lease_id.as_bytes(),
        receipt.occurrence_key.as_bytes(),
        receipt.topic.as_bytes(),
        receipt.payload_sha256.as_str().as_bytes(),
    ] {
        bytes.extend_from_slice(&(part.len() as u64).to_be_bytes());
        bytes.extend_from_slice(part);
    }
    Sha256Digest::for_bytes(&bytes)
}

async fn verify_durable_store(store: &CognitiveStore) -> Result<(), ProductionWriterError> {
    let journal_mode: String = sqlx::query_scalar("PRAGMA journal_mode")
        .fetch_one(&store.pool)
        .await
        .map_err(|error| ProductionWriterError::Durability(error.to_string()))?;
    let synchronous: i64 = sqlx::query_scalar("PRAGMA synchronous")
        .fetch_one(&store.pool)
        .await
        .map_err(|error| ProductionWriterError::Durability(error.to_string()))?;
    if !journal_mode.eq_ignore_ascii_case(PRODUCTION_DURABLE_WRITER_JOURNAL_MODE)
        || synchronous != PRODUCTION_DURABLE_WRITER_SYNCHRONOUS_FULL
    {
        return Err(ProductionWriterError::Durability(format!(
            "required journal_mode=WAL and synchronous=FULL, observed mode={journal_mode:?} synchronous={synchronous}"
        )));
    }
    Ok(())
}

fn validate_text(value: &str, label: &str, max_bytes: usize) -> Result<(), ProductionWriterError> {
    if value.is_empty() || value.len() > max_bytes || value.as_bytes().contains(&0) {
        return Err(ProductionWriterError::Invalid(format!(
            "{label} must contain 1..={max_bytes} non-NUL bytes"
        )));
    }
    Ok(())
}

fn now_unix_seconds() -> Result<u64, ProductionWriterError> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .map_err(|error| ProductionWriterError::Invalid(format!("system clock failed: {error}")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use codex_hepta_paths::HeptaFleetRoot;
    use std::sync::atomic::AtomicUsize;
    use std::sync::atomic::Ordering;
    use tempfile::TempDir;

    const OWNER: u8 = 222;

    fn agent_id(number: u8) -> AgentId {
        AgentId::parse(format!("018f4f72-5f8f-7cc1-8f55-df9fb3aa2c{number:02x}")).unwrap()
    }

    async fn store(temp: &TempDir) -> CognitiveStore {
        let fleet_root = temp.path().join("fleet");
        std::fs::create_dir_all(&fleet_root).unwrap();
        let fleet = HeptaFleetRoot::parse(fleet_root).unwrap();
        CognitiveStore::open(&fleet.layout().agent(&agent_id(OWNER))).await.unwrap()
    }

    fn authority(agent: AgentId) -> ProductionAuthorityLease {
        ProductionAuthorityLease::from_verified_parts(
            agent,
            Sha256Digest::for_bytes(b"signed-grant"),
            9,
            4,
            now_unix_seconds().unwrap() + 3_600,
            ProductionAuthorityToken::from_verified_bytes(b"opaque-supervisor-token".to_vec()).unwrap(),
        )
        .unwrap()
    }

    struct Target {
        calls: AtomicUsize,
        outcome: ProductionTargetOutcome,
    }

    struct AllowVerifier;

    impl ProductionAuthorityVerifier for AllowVerifier {
        fn verify(
            &self,
            _authority: &ProductionAuthorityLease,
            _expected_agent: &AgentId,
        ) -> Result<(), String> {
            Ok(())
        }
    }

    struct DenyVerifier;

    impl ProductionAuthorityVerifier for DenyVerifier {
        fn verify(
            &self,
            _authority: &ProductionAuthorityLease,
            _expected_agent: &AgentId,
        ) -> Result<(), String> {
            Err("no independent grant".to_string())
        }
    }

    impl ProductionOutboxTarget for Target {
        fn dispatch<'a>(&'a self, _request: ProductionDispatchRequest) -> ProductionDispatchFuture<'a> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            let outcome = self.outcome.clone();
            Box::pin(async move { outcome })
        }
    }

    #[tokio::test]
    async fn production_writer_requires_verifier_and_records_commit() {
        let temp = TempDir::new().unwrap();
        let store = store(&temp).await;
        let owner = store.owner_agent_id().clone();
        let auth = authority(owner);
        let denied = ProductionDurableWriter::open(
            store.clone(),
            auth.clone(),
            &DenyVerifier,
            "production:h4:test",
            1,
        )
        .await
        .unwrap_err();
        assert!(matches!(denied, ProductionWriterError::AuthorityRejected(_)));

        let writer = ProductionDurableWriter::open(
            store,
            auth,
            &AllowVerifier,
            "production:h4:test",
            1,
        )
        .await
        .unwrap();
        let queued = writer.admit("occurrence:1", "memory.write", "{\"x\":1}").await.unwrap();
        assert!(!queued.external_effect);
        let target = Arc::new(Target {
            calls: AtomicUsize::new(0),
            outcome: ProductionTargetOutcome::Committed { receipt: "provider-ack-1".to_string() },
        });
        let dispatcher = ProductionOutboxDispatcher::attach(target.clone());
        let dispatched = dispatcher.dispatch(&writer, queued).await.unwrap();
        assert_eq!(dispatched.state, LocalOutcomeState::Committed);
        assert!(dispatched.external_effect);
        assert_eq!(target.calls.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn unknown_target_outcome_is_indeterminate_and_replay_is_durable() {
        let temp = TempDir::new().unwrap();
        let store = store(&temp).await;
        let owner = store.owner_agent_id().clone();
        let auth = authority(owner);
        let writer = ProductionDurableWriter::open(
            store,
            auth,
            &AllowVerifier,
            "production:h4:unknown",
            1,
        )
        .await
        .unwrap();
        let queued = writer.admit("occurrence:unknown", "memory.write", "payload").await.unwrap();
        let target = Arc::new(Target {
            calls: AtomicUsize::new(0),
            outcome: ProductionTargetOutcome::Indeterminate { reason: "timeout".to_string() },
        });
        let receipt = ProductionOutboxDispatcher::attach(target).dispatch(&writer, queued).await.unwrap();
        assert_eq!(receipt.state, LocalOutcomeState::Indeterminate);
        assert!(!receipt.external_effect);
        assert_eq!(writer.status("occurrence:unknown").await.unwrap(), LocalOutcomeState::Indeterminate);
        let recovery = writer.recover("occurrence:unknown").await.unwrap();
        assert!(recovery.state.starts_with("released_"));
        assert!(!recovery.physical_power_loss_claim);
    }

    #[tokio::test]
    async fn restart_reopens_exact_bound_lease_and_replays_queue() {
        let temp = TempDir::new().unwrap();
        let store = store(&temp).await;
        let owner = store.owner_agent_id().clone();
        let auth = authority(owner);
        let writer = ProductionDurableWriter::open(
            store.clone(),
            auth.clone(),
            &AllowVerifier,
            "production:h4:restart",
            1,
        )
        .await
        .unwrap();
        let queued = writer.admit("occurrence:restart", "memory.write", "payload").await.unwrap();
        drop(writer);
        let reopened = ProductionDurableWriter::open(
            store,
            auth,
            &AllowVerifier,
            "production:h4:restart",
            1,
        )
        .await
        .unwrap();
        let replay = reopened.admit("occurrence:restart", "memory.write", "payload").await.unwrap();
        assert!(replay.replayed);
        assert_eq!(replay.event_id, queued.event_id);
        assert_eq!(replay.outbox_id, queued.outbox_id);
    }
}
