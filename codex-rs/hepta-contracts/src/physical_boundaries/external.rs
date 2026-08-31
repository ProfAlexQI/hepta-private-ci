//! Checked tool, network, and external-filesystem physical boundaries.
//!
//! All three boundary classes reuse the common B0 verified-use kernel but bind
//! different final facts. A broad [`ExternalEffectCapability`] cannot cross any
//! boundary until the exact final payload is known, current authority is
//! reverified, a durable single-use claim is committed, and the witness is
//! persisted by the caller.
//!
//! The raw adapter is private to [`CheckedExternalBoundary`]. Adapter errors
//! after entry are indeterminate and this API exposes no automatic retry path.

use std::fmt;
use std::future::Future;
use std::pin::Pin;

use serde::Serialize;

use crate::AuthorityError;
use crate::Authorized;
use crate::ExternalEffectCapability;
use crate::OperationId;
use crate::PhysicalCapabilityKind;
use crate::PhysicalUseClaimReceipt;
use crate::PhysicalUseClaimRequest;
use crate::PhysicalUseClaimStore;
use crate::PhysicalUseClaimStoreError;
use crate::PhysicalUseFinalCheck;
use crate::PhysicalUseVerifier;
use crate::PhysicalUseWindow;
use crate::RevocationRevision;
use crate::RuntimeAuthorityContext;
use crate::Sha256Digest;
use crate::TrustedPhysicalClock;
use crate::VerifiedUseError;
use crate::VerifiedUseWitness;
use crate::verify_physical_capability_use;

pub const EXTERNAL_BOUNDARY_SCHEMA_VERSION: u32 = 1;
pub const B2_EXTERNAL_BOUNDARIES_RUNTIME_REGISTERED: bool = false;
pub const B2_EXTERNAL_BOUNDARIES_PRODUCTION_CALLER: bool = false;
pub const B2_EXTERNAL_BOUNDARIES_PRODUCTION_WRITER: bool = false;
pub const B2_EXTERNAL_BOUNDARIES_TOOL_EXECUTION: bool = false;
pub const B2_EXTERNAL_BOUNDARIES_NETWORK_CONNECT: bool = false;
pub const B2_EXTERNAL_BOUNDARIES_FILESYSTEM_MUTATION: bool = false;
pub const B2_EXTERNAL_BOUNDARIES_EXTERNAL_EFFECT: bool = false;
pub const B2_EXTERNAL_BOUNDARIES_OPERATOR_ACCEPTANCE: bool = false;
pub const B2_EXTERNAL_BOUNDARIES_PROMOTION: bool = false;
pub const B2_EXTERNAL_BOUNDARIES_RELEASE: bool = false;

const MAX_IDENTITY_BYTES: usize = 2_048;
const MAX_PROTOCOL_BYTES: usize = 32;
const MAX_REASON_CODE_BYTES: usize = 256;
const MAX_FINAL_PAYLOAD_BYTES: u64 = 64 * 1024 * 1024;
const MAX_ARG_COUNT: u32 = 4_096;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NetworkProtocol {
    Http,
    Https,
    Tcp,
    Tls,
    Quic,
}

impl NetworkProtocol {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Http => "http",
            Self::Https => "https",
            Self::Tcp => "tcp",
            Self::Tls => "tls",
            Self::Quic => "quic",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FilesystemMutationClass {
    CreateFile,
    ReplaceFile,
    AppendFile,
    RemoveFile,
    CreateDirectory,
    RemoveDirectory,
    Rename,
    SetMetadata,
}

impl FilesystemMutationClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CreateFile => "create_file",
            Self::ReplaceFile => "replace_file",
            Self::AppendFile => "append_file",
            Self::RemoveFile => "remove_file",
            Self::CreateDirectory => "create_directory",
            Self::RemoveDirectory => "remove_directory",
            Self::Rename => "rename",
            Self::SetMetadata => "set_metadata",
        }
    }
}

/// Exact tool-process launch identity. It stores no raw argv or environment.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ToolProcessIntent {
    schema_version: u32,
    operation_id: OperationId,
    executable_identity_sha256: Sha256Digest,
    executable_file_sha256: Sha256Digest,
    argv_sha256: Sha256Digest,
    argv_count: u32,
    cwd_identity_sha256: Sha256Digest,
    environment_policy_sha256: Sha256Digest,
    sandbox_policy_sha256: Sha256Digest,
    approval_sha256: Sha256Digest,
    final_payload_sha256: Sha256Digest,
    final_payload_bytes: u64,
}

impl ToolProcessIntent {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        operation_id: OperationId,
        executable_identity_sha256: Sha256Digest,
        executable_file_sha256: Sha256Digest,
        argv_sha256: Sha256Digest,
        argv_count: u32,
        cwd_identity_sha256: Sha256Digest,
        environment_policy_sha256: Sha256Digest,
        sandbox_policy_sha256: Sha256Digest,
        approval_sha256: Sha256Digest,
        final_payload: &[u8],
    ) -> Result<Self, ExternalBoundaryError> {
        let intent = Self {
            schema_version: EXTERNAL_BOUNDARY_SCHEMA_VERSION,
            operation_id,
            executable_identity_sha256,
            executable_file_sha256,
            argv_sha256,
            argv_count,
            cwd_identity_sha256,
            environment_policy_sha256,
            sandbox_policy_sha256,
            approval_sha256,
            final_payload_sha256: Sha256Digest::for_bytes(final_payload),
            final_payload_bytes: payload_len(final_payload)?,
        };
        intent.validate()?;
        Ok(intent)
    }

    pub const fn operation_id(&self) -> &OperationId {
        &self.operation_id
    }

    pub const fn final_payload_sha256(&self) -> &Sha256Digest {
        &self.final_payload_sha256
    }

    pub const fn final_payload_bytes(&self) -> u64 {
        self.final_payload_bytes
    }

    pub fn physical_payload_sha256(&self) -> Result<Sha256Digest, ExternalBoundaryError> {
        self.validate()?;
        let mut bytes = Vec::new();
        frame(&mut bytes, b"hepta:tool-process-physical-payload:v1");
        frame(&mut bytes, &self.schema_version.to_be_bytes());
        frame(&mut bytes, self.operation_id.as_str().as_bytes());
        frame_digest(&mut bytes, &self.executable_identity_sha256);
        frame_digest(&mut bytes, &self.executable_file_sha256);
        frame_digest(&mut bytes, &self.argv_sha256);
        frame(&mut bytes, &self.argv_count.to_be_bytes());
        frame_digest(&mut bytes, &self.cwd_identity_sha256);
        frame_digest(&mut bytes, &self.environment_policy_sha256);
        frame_digest(&mut bytes, &self.sandbox_policy_sha256);
        frame_digest(&mut bytes, &self.approval_sha256);
        frame_digest(&mut bytes, &self.final_payload_sha256);
        frame(&mut bytes, &self.final_payload_bytes.to_be_bytes());
        Ok(Sha256Digest::for_bytes(&bytes))
    }

    fn validate(&self) -> Result<(), ExternalBoundaryError> {
        if self.schema_version != EXTERNAL_BOUNDARY_SCHEMA_VERSION {
            return Err(ExternalBoundaryError::SchemaVersion);
        }
        if self.argv_count > MAX_ARG_COUNT {
            return Err(ExternalBoundaryError::InvalidArgCount);
        }
        validate_payload_size(self.final_payload_bytes)?;
        for (field, digest) in [
            ("executable identity", &self.executable_identity_sha256),
            ("executable file", &self.executable_file_sha256),
            ("argv", &self.argv_sha256),
            ("cwd identity", &self.cwd_identity_sha256),
            ("environment policy", &self.environment_policy_sha256),
            ("sandbox policy", &self.sandbox_policy_sha256),
            ("approval", &self.approval_sha256),
            ("tool final payload", &self.final_payload_sha256),
        ] {
            validate_digest(field, digest)?;
        }
        Ok(())
    }
}

/// Exact outbound network connection and request identity.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OutboundNetworkIntent {
    schema_version: u32,
    operation_id: OperationId,
    protocol: NetworkProtocol,
    canonical_destination: String,
    resolved_ip_set_sha256: Sha256Digest,
    dns_policy_sha256: Sha256Digest,
    proxy_policy_sha256: Sha256Digest,
    tls_policy_sha256: Sha256Digest,
    request_headers_sha256: Sha256Digest,
    final_payload_sha256: Sha256Digest,
    final_payload_bytes: u64,
}

impl OutboundNetworkIntent {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        operation_id: OperationId,
        protocol: NetworkProtocol,
        canonical_destination: impl Into<String>,
        resolved_ip_set_sha256: Sha256Digest,
        dns_policy_sha256: Sha256Digest,
        proxy_policy_sha256: Sha256Digest,
        tls_policy_sha256: Sha256Digest,
        request_headers_sha256: Sha256Digest,
        final_payload: &[u8],
    ) -> Result<Self, ExternalBoundaryError> {
        let intent = Self {
            schema_version: EXTERNAL_BOUNDARY_SCHEMA_VERSION,
            operation_id,
            protocol,
            canonical_destination: canonical_destination.into(),
            resolved_ip_set_sha256,
            dns_policy_sha256,
            proxy_policy_sha256,
            tls_policy_sha256,
            request_headers_sha256,
            final_payload_sha256: Sha256Digest::for_bytes(final_payload),
            final_payload_bytes: payload_len(final_payload)?,
        };
        intent.validate()?;
        Ok(intent)
    }

    pub const fn operation_id(&self) -> &OperationId {
        &self.operation_id
    }

    pub const fn protocol(&self) -> NetworkProtocol {
        self.protocol
    }

    pub fn canonical_destination(&self) -> &str {
        &self.canonical_destination
    }

    pub const fn final_payload_sha256(&self) -> &Sha256Digest {
        &self.final_payload_sha256
    }

    pub const fn final_payload_bytes(&self) -> u64 {
        self.final_payload_bytes
    }

    pub fn physical_payload_sha256(&self) -> Result<Sha256Digest, ExternalBoundaryError> {
        self.validate()?;
        let mut bytes = Vec::new();
        frame(&mut bytes, b"hepta:outbound-network-physical-payload:v1");
        frame(&mut bytes, &self.schema_version.to_be_bytes());
        frame(&mut bytes, self.operation_id.as_str().as_bytes());
        frame(&mut bytes, self.protocol.as_str().as_bytes());
        frame(&mut bytes, self.canonical_destination.as_bytes());
        frame_digest(&mut bytes, &self.resolved_ip_set_sha256);
        frame_digest(&mut bytes, &self.dns_policy_sha256);
        frame_digest(&mut bytes, &self.proxy_policy_sha256);
        frame_digest(&mut bytes, &self.tls_policy_sha256);
        frame_digest(&mut bytes, &self.request_headers_sha256);
        frame_digest(&mut bytes, &self.final_payload_sha256);
        frame(&mut bytes, &self.final_payload_bytes.to_be_bytes());
        Ok(Sha256Digest::for_bytes(&bytes))
    }

    fn validate(&self) -> Result<(), ExternalBoundaryError> {
        if self.schema_version != EXTERNAL_BOUNDARY_SCHEMA_VERSION {
            return Err(ExternalBoundaryError::SchemaVersion);
        }
        validate_identity("canonical destination", &self.canonical_destination)?;
        if self.protocol.as_str().len() > MAX_PROTOCOL_BYTES {
            return Err(ExternalBoundaryError::InvalidProtocol);
        }
        validate_payload_size(self.final_payload_bytes)?;
        for (field, digest) in [
            ("resolved IP set", &self.resolved_ip_set_sha256),
            ("DNS policy", &self.dns_policy_sha256),
            ("proxy policy", &self.proxy_policy_sha256),
            ("TLS policy", &self.tls_policy_sha256),
            ("request headers", &self.request_headers_sha256),
            ("network final payload", &self.final_payload_sha256),
        ] {
            validate_digest(field, digest)?;
        }
        Ok(())
    }
}

/// Exact mutation outside the Agent-owned filesystem root.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExternalFilesystemMutationIntent {
    schema_version: u32,
    operation_id: OperationId,
    canonical_target_sha256: Sha256Digest,
    device_mount_identity_sha256: Sha256Digest,
    no_follow: bool,
    mutation_class: FilesystemMutationClass,
    expected_prior_state_sha256: Sha256Digest,
    final_payload_sha256: Sha256Digest,
    final_payload_bytes: u64,
}

impl ExternalFilesystemMutationIntent {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        operation_id: OperationId,
        canonical_target_sha256: Sha256Digest,
        device_mount_identity_sha256: Sha256Digest,
        no_follow: bool,
        mutation_class: FilesystemMutationClass,
        expected_prior_state_sha256: Sha256Digest,
        final_payload: &[u8],
    ) -> Result<Self, ExternalBoundaryError> {
        let intent = Self {
            schema_version: EXTERNAL_BOUNDARY_SCHEMA_VERSION,
            operation_id,
            canonical_target_sha256,
            device_mount_identity_sha256,
            no_follow,
            mutation_class,
            expected_prior_state_sha256,
            final_payload_sha256: Sha256Digest::for_bytes(final_payload),
            final_payload_bytes: payload_len(final_payload)?,
        };
        intent.validate()?;
        Ok(intent)
    }

    pub const fn operation_id(&self) -> &OperationId {
        &self.operation_id
    }

    pub const fn no_follow(&self) -> bool {
        self.no_follow
    }

    pub const fn mutation_class(&self) -> FilesystemMutationClass {
        self.mutation_class
    }

    pub const fn final_payload_sha256(&self) -> &Sha256Digest {
        &self.final_payload_sha256
    }

    pub const fn final_payload_bytes(&self) -> u64 {
        self.final_payload_bytes
    }

    pub fn physical_payload_sha256(&self) -> Result<Sha256Digest, ExternalBoundaryError> {
        self.validate()?;
        let mut bytes = Vec::new();
        frame(
            &mut bytes,
            b"hepta:external-filesystem-physical-payload:v1",
        );
        frame(&mut bytes, &self.schema_version.to_be_bytes());
        frame(&mut bytes, self.operation_id.as_str().as_bytes());
        frame_digest(&mut bytes, &self.canonical_target_sha256);
        frame_digest(&mut bytes, &self.device_mount_identity_sha256);
        frame(&mut bytes, &[u8::from(self.no_follow)]);
        frame(&mut bytes, self.mutation_class.as_str().as_bytes());
        frame_digest(&mut bytes, &self.expected_prior_state_sha256);
        frame_digest(&mut bytes, &self.final_payload_sha256);
        frame(&mut bytes, &self.final_payload_bytes.to_be_bytes());
        Ok(Sha256Digest::for_bytes(&bytes))
    }

    fn validate(&self) -> Result<(), ExternalBoundaryError> {
        if self.schema_version != EXTERNAL_BOUNDARY_SCHEMA_VERSION {
            return Err(ExternalBoundaryError::SchemaVersion);
        }
        if !self.no_follow {
            return Err(ExternalBoundaryError::NoFollowRequired);
        }
        validate_payload_size(self.final_payload_bytes)?;
        for (field, digest) in [
            ("canonical target", &self.canonical_target_sha256),
            ("device/mount identity", &self.device_mount_identity_sha256),
            ("expected prior state", &self.expected_prior_state_sha256),
            ("filesystem final payload", &self.final_payload_sha256),
        ] {
            validate_digest(field, digest)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "boundary", rename_all = "snake_case")]
pub enum ExternalBoundaryIntent {
    ToolProcess(ToolProcessIntent),
    OutboundNetwork(OutboundNetworkIntent),
    ExternalFilesystemMutation(ExternalFilesystemMutationIntent),
}

impl ExternalBoundaryIntent {
    pub const fn kind(&self) -> PhysicalCapabilityKind {
        match self {
            Self::ToolProcess(_) => PhysicalCapabilityKind::ToolProcessSpawn,
            Self::OutboundNetwork(_) => PhysicalCapabilityKind::OutboundNetworkConnect,
            Self::ExternalFilesystemMutation(_) => {
                PhysicalCapabilityKind::ExternalFilesystemMutation
            }
        }
    }

    pub const fn operation_id(&self) -> &OperationId {
        match self {
            Self::ToolProcess(intent) => intent.operation_id(),
            Self::OutboundNetwork(intent) => intent.operation_id(),
            Self::ExternalFilesystemMutation(intent) => intent.operation_id(),
        }
    }

    pub const fn final_payload_sha256(&self) -> &Sha256Digest {
        match self {
            Self::ToolProcess(intent) => intent.final_payload_sha256(),
            Self::OutboundNetwork(intent) => intent.final_payload_sha256(),
            Self::ExternalFilesystemMutation(intent) => intent.final_payload_sha256(),
        }
    }

    pub const fn final_payload_bytes(&self) -> u64 {
        match self {
            Self::ToolProcess(intent) => intent.final_payload_bytes(),
            Self::OutboundNetwork(intent) => intent.final_payload_bytes(),
            Self::ExternalFilesystemMutation(intent) => intent.final_payload_bytes(),
        }
    }

    pub fn validate_final_payload(
        &self,
        final_payload: &[u8],
    ) -> Result<(), ExternalBoundaryError> {
        let observed_bytes = payload_len(final_payload)?;
        if observed_bytes != self.final_payload_bytes()
            || Sha256Digest::for_bytes(final_payload) != *self.final_payload_sha256()
        {
            return Err(ExternalBoundaryError::FinalPayloadDrift);
        }
        match self {
            Self::ToolProcess(intent) => intent.validate(),
            Self::OutboundNetwork(intent) => intent.validate(),
            Self::ExternalFilesystemMutation(intent) => intent.validate(),
        }
    }

    pub fn physical_payload_sha256(&self) -> Result<Sha256Digest, ExternalBoundaryError> {
        match self {
            Self::ToolProcess(intent) => intent.physical_payload_sha256(),
            Self::OutboundNetwork(intent) => intent.physical_payload_sha256(),
            Self::ExternalFilesystemMutation(intent) => intent.physical_payload_sha256(),
        }
    }
}

impl From<ToolProcessIntent> for ExternalBoundaryIntent {
    fn from(intent: ToolProcessIntent) -> Self {
        Self::ToolProcess(intent)
    }
}

impl From<OutboundNetworkIntent> for ExternalBoundaryIntent {
    fn from(intent: OutboundNetworkIntent) -> Self {
        Self::OutboundNetwork(intent)
    }
}

impl From<ExternalFilesystemMutationIntent> for ExternalBoundaryIntent {
    fn from(intent: ExternalFilesystemMutationIntent) -> Self {
        Self::ExternalFilesystemMutation(intent)
    }
}

/// Request visible to a physical boundary adapter. Its constructor is private.
pub struct ExternalBoundaryDispatch<'a> {
    intent: &'a ExternalBoundaryIntent,
    final_payload: &'a [u8],
    verified_use_witness_sha256: &'a Sha256Digest,
}

impl<'a> ExternalBoundaryDispatch<'a> {
    pub const fn intent(&self) -> &'a ExternalBoundaryIntent {
        self.intent
    }

    pub const fn kind(&self) -> PhysicalCapabilityKind {
        self.intent.kind()
    }

    pub const fn final_payload(&self) -> &'a [u8] {
        self.final_payload
    }

    pub const fn verified_use_witness_sha256(&self) -> &'a Sha256Digest {
        self.verified_use_witness_sha256
    }
}

impl fmt::Debug for ExternalBoundaryDispatch<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ExternalBoundaryDispatch")
            .field("kind", &self.kind())
            .field("operation_id", &self.intent.operation_id().as_str())
            .field("final_payload_bytes", &self.final_payload.len())
            .field("final_payload_sha256", self.intent.final_payload_sha256())
            .field(
                "verified_use_witness_sha256",
                self.verified_use_witness_sha256,
            )
            .finish_non_exhaustive()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub enum ExternalBoundaryOutcome {
    Completed {
        result_sha256: Sha256Digest,
        result_bytes: u64,
    },
    RejectedNoCrossing {
        reason_code: String,
    },
    Indeterminate {
        reason_code: String,
    },
}

impl ExternalBoundaryOutcome {
    pub fn completed(result: &[u8]) -> Result<Self, ExternalBoundaryError> {
        let result_bytes = payload_len(result)?;
        Ok(Self::Completed {
            result_sha256: Sha256Digest::for_bytes(result),
            result_bytes,
        })
    }

    pub fn rejected_no_crossing(
        reason_code: impl Into<String>,
    ) -> Result<Self, ExternalBoundaryError> {
        let reason_code = reason_code.into();
        validate_reason_code(&reason_code)?;
        Ok(Self::RejectedNoCrossing { reason_code })
    }

    pub fn indeterminate(reason_code: impl Into<String>) -> Result<Self, ExternalBoundaryError> {
        let reason_code = reason_code.into();
        validate_reason_code(&reason_code)?;
        Ok(Self::Indeterminate { reason_code })
    }

    pub fn validate(&self) -> Result<(), ExternalBoundaryError> {
        match self {
            Self::Completed {
                result_sha256,
                result_bytes,
            } => {
                validate_digest("external boundary result", result_sha256)?;
                validate_payload_size(*result_bytes)?;
            }
            Self::RejectedNoCrossing { reason_code } | Self::Indeterminate { reason_code } => {
                validate_reason_code(reason_code)?;
            }
        }
        Ok(())
    }
}

pub type ExternalBoundaryFuture<'a> =
    Pin<Box<dyn Future<Output = Result<ExternalBoundaryOutcome, String>> + Send + 'a>>;

pub trait ExternalBoundaryAdapter: Send {
    fn cross<'a>(&'a mut self, dispatch: ExternalBoundaryDispatch<'a>)
    -> ExternalBoundaryFuture<'a>;
}

/// Checked tool/network/filesystem boundary with one private adapter and no raw
/// restoration or extraction API.
pub struct CheckedExternalBoundary<A, V>
where
    A: ExternalBoundaryAdapter,
    V: PhysicalUseVerifier,
{
    adapter: A,
    capability: Authorized<ExternalEffectCapability>,
    runtime_authority: RuntimeAuthorityContext,
    verifier: V,
}

impl<A, V> CheckedExternalBoundary<A, V>
where
    A: ExternalBoundaryAdapter,
    V: PhysicalUseVerifier,
{
    pub fn new(
        adapter: A,
        capability: Authorized<ExternalEffectCapability>,
        verifier: V,
    ) -> Result<Self, ExternalBoundaryError> {
        let binding = capability
            .external_lease_binding()
            .ok_or(ExternalBoundaryError::ExternalAuthorityRequired)?;
        let runtime_authority = RuntimeAuthorityContext::from_external_binding(binding)?;
        Ok(Self {
            adapter,
            capability,
            runtime_authority,
            verifier,
        })
    }

    /// Crosses one exact boundary exactly once after final-payload, current
    /// authority, durable claim and witness persistence gates.
    #[allow(clippy::too_many_arguments)]
    pub async fn cross_once<N, C, P>(
        &mut self,
        intent: &ExternalBoundaryIntent,
        final_payload: &[u8],
        expected_revocation_revision: RevocationRevision,
        window: PhysicalUseWindow,
        now_unix_seconds: &N,
        claim_once: &C,
        persist_witness: P,
    ) -> Result<(ExternalBoundaryOutcome, VerifiedUseWitness), ExternalBoundaryError>
    where
        N: Fn() -> Result<u64, String> + Sync + ?Sized,
        C: Fn(
                PhysicalCapabilityKind,
                &Sha256Digest,
                &Sha256Digest,
                &Sha256Digest,
                &Sha256Digest,
                u64,
            ) -> Result<(u64, Sha256Digest), String>
            + Sync
            + ?Sized,
        P: FnOnce(&VerifiedUseWitness) -> Result<(), String>,
    {
        intent.validate_final_payload(final_payload)?;
        validate_capability_binding(&self.capability, &self.runtime_authority)?;

        let physical_payload_sha256 = intent.physical_payload_sha256()?;
        let clock = ClosureClock(now_unix_seconds);
        let claim_store = ClosureClaimStore(claim_once);
        let token = verify_physical_capability_use(
            &self.capability,
            intent.kind(),
            intent.operation_id(),
            &physical_payload_sha256,
            &self.runtime_authority,
            expected_revocation_revision,
            window,
            &self.verifier,
            &clock,
        )?;
        let permit = token.consume_at_boundary(
            PhysicalUseFinalCheck::new(
                intent.kind(),
                intent.operation_id(),
                &physical_payload_sha256,
                &self.runtime_authority,
            ),
            &self.verifier,
            &clock,
            &claim_store,
        )?;
        let witness = permit.into_witness();
        witness.validate()?;
        persist_witness(&witness).map_err(ExternalBoundaryError::WitnessPersistence)?;

        let dispatch = ExternalBoundaryDispatch {
            intent,
            final_payload,
            verified_use_witness_sha256: witness.witness_sha256(),
        };
        let outcome = match self.adapter.cross(dispatch).await {
            Ok(outcome) => outcome,
            Err(reason) => {
                ExternalBoundaryOutcome::indeterminate(normalize_transport_reason(&reason))?
            }
        };
        outcome.validate()?;
        Ok((outcome, witness))
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum ExternalBoundaryError {
    SchemaVersion,
    EmptyIdentity(&'static str),
    InvalidIdentity(&'static str),
    InvalidDigest(&'static str),
    InvalidProtocol,
    InvalidArgCount,
    InvalidPayloadSize,
    FinalPayloadDrift,
    NoFollowRequired,
    ExternalAuthorityRequired,
    CapabilityBindingDrift,
    WitnessPersistence(String),
    InvalidReasonCode,
    Authority(AuthorityError),
    VerifiedUse(VerifiedUseError),
}

impl fmt::Display for ExternalBoundaryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersion => formatter.write_str("external boundary schema version drift"),
            Self::EmptyIdentity(field) => write!(formatter, "{field} must not be empty"),
            Self::InvalidIdentity(field) => write!(formatter, "{field} is not canonical"),
            Self::InvalidDigest(field) => write!(formatter, "{field} SHA-256 is invalid"),
            Self::InvalidProtocol => formatter.write_str("network protocol is invalid"),
            Self::InvalidArgCount => formatter.write_str("tool argv count is invalid"),
            Self::InvalidPayloadSize => {
                formatter.write_str("external boundary payload size is invalid")
            }
            Self::FinalPayloadDrift => {
                formatter.write_str("external boundary final payload drifted from intent")
            }
            Self::NoFollowRequired => {
                formatter.write_str("external filesystem mutation requires no-follow")
            }
            Self::ExternalAuthorityRequired => {
                formatter.write_str("external boundary requires externally verified authority")
            }
            Self::CapabilityBindingDrift => {
                formatter.write_str("external capability drifted from runtime authority")
            }
            Self::WitnessPersistence(reason) => {
                write!(
                    formatter,
                    "external verified-use witness persistence failed: {reason}"
                )
            }
            Self::InvalidReasonCode => {
                formatter.write_str("external boundary reason code is invalid")
            }
            Self::Authority(error) => write!(formatter, "external authority error: {error}"),
            Self::VerifiedUse(error) => write!(formatter, "external verified-use error: {error}"),
        }
    }
}

impl std::error::Error for ExternalBoundaryError {}

impl From<AuthorityError> for ExternalBoundaryError {
    fn from(error: AuthorityError) -> Self {
        Self::Authority(error)
    }
}

impl From<VerifiedUseError> for ExternalBoundaryError {
    fn from(error: VerifiedUseError) -> Self {
        Self::VerifiedUse(error)
    }
}

struct ClosureClock<'a, N>(&'a N)
where
    N: Fn() -> Result<u64, String> + Sync + ?Sized;

impl<N> TrustedPhysicalClock for ClosureClock<'_, N>
where
    N: Fn() -> Result<u64, String> + Sync + ?Sized,
{
    fn now_unix_seconds(&self) -> Result<u64, String> {
        (self.0)()
    }
}

struct ClosureClaimStore<'a, C>(&'a C)
where
    C: Fn(
            PhysicalCapabilityKind,
            &Sha256Digest,
            &Sha256Digest,
            &Sha256Digest,
            &Sha256Digest,
            u64,
        ) -> Result<(u64, Sha256Digest), String>
        + Sync
        + ?Sized;

impl<C> PhysicalUseClaimStore for ClosureClaimStore<'_, C>
where
    C: Fn(
            PhysicalCapabilityKind,
            &Sha256Digest,
            &Sha256Digest,
            &Sha256Digest,
            &Sha256Digest,
            u64,
        ) -> Result<(u64, Sha256Digest), String>
        + Sync
        + ?Sized,
{
    fn claim_once(
        &self,
        request: &PhysicalUseClaimRequest<'_>,
    ) -> Result<PhysicalUseClaimReceipt, PhysicalUseClaimStoreError> {
        let request_sha256 = claim_request_digest(request);
        let (claim_revision, store_receipt_sha256) = (self.0)(
            request.kind(),
            request.claim_key().operation_scope_sha256(),
            request.claim_key().claim_sha256(),
            request.token_sha256(),
            &request_sha256,
            request.claimed_at_unix_seconds(),
        )
        .map_err(PhysicalUseClaimStoreError::Rejected)?;
        PhysicalUseClaimReceipt::new(
            request.claim_key().clone(),
            claim_revision,
            request.claimed_at_unix_seconds(),
            store_receipt_sha256,
        )
    }
}

fn validate_capability_binding(
    capability: &Authorized<ExternalEffectCapability>,
    runtime_authority: &RuntimeAuthorityContext,
) -> Result<(), ExternalBoundaryError> {
    let binding = capability
        .external_lease_binding()
        .ok_or(ExternalBoundaryError::ExternalAuthorityRequired)?;
    if capability.subject_agent_id() != runtime_authority.subject_agent_id()
        || capability.generation() != runtime_authority.generation()
        || binding.authority_epoch() != runtime_authority.authority_epoch()
        || binding.owner_epoch() != runtime_authority.owner_epoch()
        || binding.fencing_token_sha256() != runtime_authority.fencing_token_sha256()
        || binding.grant_sha256() != runtime_authority.authority_grant_sha256()
    {
        return Err(ExternalBoundaryError::CapabilityBindingDrift);
    }
    Ok(())
}

fn claim_request_digest(request: &PhysicalUseClaimRequest<'_>) -> Sha256Digest {
    let mut bytes = Vec::new();
    frame(&mut bytes, b"hepta:b2-external-claim-request:v1");
    frame(&mut bytes, request.kind().as_str().as_bytes());
    frame(&mut bytes, request.operation_id().as_str().as_bytes());
    frame_digest(&mut bytes, request.final_payload_sha256());
    frame_digest(
        &mut bytes,
        request.runtime_authority_context_sha256(),
    );
    frame(
        &mut bytes,
        &request.revocation_revision().get().to_be_bytes(),
    );
    frame_digest(&mut bytes, request.token_sha256());
    frame(
        &mut bytes,
        &request.claimed_at_unix_seconds().to_be_bytes(),
    );
    Sha256Digest::for_bytes(&bytes)
}

fn payload_len(payload: &[u8]) -> Result<u64, ExternalBoundaryError> {
    let bytes = u64::try_from(payload.len()).map_err(|_| ExternalBoundaryError::InvalidPayloadSize)?;
    validate_payload_size(bytes)?;
    Ok(bytes)
}

fn validate_payload_size(bytes: u64) -> Result<(), ExternalBoundaryError> {
    if bytes == 0 || bytes > MAX_FINAL_PAYLOAD_BYTES {
        return Err(ExternalBoundaryError::InvalidPayloadSize);
    }
    Ok(())
}

fn validate_identity(field: &'static str, value: &str) -> Result<(), ExternalBoundaryError> {
    if value.is_empty() {
        return Err(ExternalBoundaryError::EmptyIdentity(field));
    }
    if value.len() > MAX_IDENTITY_BYTES
        || !value.is_ascii()
        || value.trim() != value
        || value.chars().any(char::is_control)
    {
        return Err(ExternalBoundaryError::InvalidIdentity(field));
    }
    Ok(())
}

fn validate_digest(field: &'static str, digest: &Sha256Digest) -> Result<(), ExternalBoundaryError> {
    Sha256Digest::parse(digest.as_str())
        .map(|_| ())
        .map_err(|_| ExternalBoundaryError::InvalidDigest(field))
}

fn validate_reason_code(reason_code: &str) -> Result<(), ExternalBoundaryError> {
    if reason_code.is_empty()
        || reason_code.len() > MAX_REASON_CODE_BYTES
        || !reason_code.is_ascii()
        || reason_code.trim() != reason_code
        || reason_code
            .bytes()
            .any(|byte| !(byte.is_ascii_alphanumeric() || b"._:-".contains(&byte)))
    {
        return Err(ExternalBoundaryError::InvalidReasonCode);
    }
    Ok(())
}

fn normalize_transport_reason(reason: &str) -> String {
    let mut normalized = String::with_capacity(reason.len().min(MAX_REASON_CODE_BYTES));
    for byte in reason.bytes().take(MAX_REASON_CODE_BYTES) {
        if byte.is_ascii_alphanumeric() || b"._:-".contains(&byte) {
            normalized.push(char::from(byte));
        } else {
            normalized.push('_');
        }
    }
    if normalized.is_empty() {
        "external_boundary_unknown".to_string()
    } else {
        normalized
    }
}

fn frame_digest(target: &mut Vec<u8>, digest: &Sha256Digest) {
    frame(target, digest.as_str().as_bytes());
}

fn frame(target: &mut Vec<u8>, value: &[u8]) {
    target.extend_from_slice(&(value.len() as u64).to_be_bytes());
    target.extend_from_slice(value);
}
