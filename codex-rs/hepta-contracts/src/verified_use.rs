//! Final-payload, operation-bound verification for irreversible capability use.
//!
//! A broad [`Authorized`] capability is necessary but not sufficient at a
//! physical boundary. This module composes the existing per-use authority
//! verifier with an external current-revision check, then returns a private,
//! non-cloneable and non-serializable token for exactly one operation and one
//! final payload. Boundary adapters consume that token by value.

use std::fmt;
use std::marker::PhantomData;

use serde::Serialize;

use crate::AuthorityAction;
use crate::AuthorityCapability;
use crate::AuthorityError;
use crate::Authorized;
use crate::CapabilityUseVerifier;
use crate::OperationId;
use crate::RuntimeAuthorityContext;
use crate::Sha256Digest;
use crate::verify_capability_use;

pub const VERIFIED_USE_SCHEMA_VERSION: u32 = 1;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PhysicalCapabilityKind {
    CognitiveStateWrite,
    ModelInvocation,
    ProviderDispatch,
    ExternalEffect,
    ToolProcessSpawn,
    OutboundNetworkConnect,
    ExternalFilesystemMutation,
    SecretOperation,
    MatrixSend,
    FleetMutation,
    OperatorAcceptance,
    ReleasePromotion,
}

impl PhysicalCapabilityKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CognitiveStateWrite => "cognitive_state_write",
            Self::ModelInvocation => "model_invocation",
            Self::ProviderDispatch => "provider_dispatch",
            Self::ExternalEffect => "external_effect",
            Self::ToolProcessSpawn => "tool_process_spawn",
            Self::OutboundNetworkConnect => "outbound_network_connect",
            Self::ExternalFilesystemMutation => "external_filesystem_mutation",
            Self::SecretOperation => "secret_operation",
            Self::MatrixSend => "matrix_send",
            Self::FleetMutation => "fleet_mutation",
            Self::OperatorAcceptance => "operator_acceptance",
            Self::ReleasePromotion => "release_promotion",
        }
    }

    pub const fn authority_action(self) -> AuthorityAction {
        match self {
            Self::CognitiveStateWrite => AuthorityAction::WriteCognitiveState,
            Self::ModelInvocation => AuthorityAction::InvokeModel,
            Self::ProviderDispatch => AuthorityAction::DispatchProvider,
            Self::ExternalEffect
            | Self::ToolProcessSpawn
            | Self::OutboundNetworkConnect
            | Self::ExternalFilesystemMutation
            | Self::SecretOperation
            | Self::MatrixSend => AuthorityAction::ExternalEffect,
            Self::FleetMutation => AuthorityAction::MutateFleet,
            Self::OperatorAcceptance => AuthorityAction::AcceptOperator,
            Self::ReleasePromotion => AuthorityAction::PromoteRelease,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub struct RevocationRevision(u64);

impl RevocationRevision {
    pub fn new(value: u64) -> Result<Self, VerifiedUseError> {
        if value == 0 {
            return Err(VerifiedUseError::InvalidRevocationRevision);
        }
        Ok(Self(value))
    }

    pub const fn get(self) -> u64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PhysicalUseWindow {
    verified_at_unix_seconds: u64,
    requested_expires_at_unix_seconds: u64,
}

impl PhysicalUseWindow {
    pub fn new(
        verified_at_unix_seconds: u64,
        requested_expires_at_unix_seconds: u64,
    ) -> Result<Self, VerifiedUseError> {
        if verified_at_unix_seconds == 0
            || requested_expires_at_unix_seconds <= verified_at_unix_seconds
        {
            return Err(VerifiedUseError::InvalidVerificationWindow);
        }
        Ok(Self {
            verified_at_unix_seconds,
            requested_expires_at_unix_seconds,
        })
    }

    pub const fn verified_at_unix_seconds(self) -> u64 {
        self.verified_at_unix_seconds
    }

    pub const fn requested_expires_at_unix_seconds(self) -> u64 {
        self.requested_expires_at_unix_seconds
    }
}

/// Exact final-payload request presented to the physical-use verifier.
#[derive(Clone, Copy, Debug)]
pub struct PhysicalUseVerificationRequest<'a> {
    kind: PhysicalCapabilityKind,
    operation_id: &'a OperationId,
    final_payload_sha256: &'a Sha256Digest,
    runtime_context: &'a RuntimeAuthorityContext,
    expected_revocation_revision: RevocationRevision,
    window: PhysicalUseWindow,
}

impl<'a> PhysicalUseVerificationRequest<'a> {
    pub fn new(
        kind: PhysicalCapabilityKind,
        operation_id: &'a OperationId,
        final_payload_sha256: &'a Sha256Digest,
        runtime_context: &'a RuntimeAuthorityContext,
        expected_revocation_revision: RevocationRevision,
        window: PhysicalUseWindow,
    ) -> Self {
        Self {
            kind,
            operation_id,
            final_payload_sha256,
            runtime_context,
            expected_revocation_revision,
            window,
        }
    }

    pub const fn kind(&self) -> PhysicalCapabilityKind {
        self.kind
    }

    pub const fn action(&self) -> AuthorityAction {
        self.kind.authority_action()
    }

    pub const fn operation_id(&self) -> &'a OperationId {
        self.operation_id
    }

    pub const fn final_payload_sha256(&self) -> &'a Sha256Digest {
        self.final_payload_sha256
    }

    pub const fn runtime_context(&self) -> &'a RuntimeAuthorityContext {
        self.runtime_context
    }

    pub const fn expected_revocation_revision(&self) -> RevocationRevision {
        self.expected_revocation_revision
    }

    pub const fn window(&self) -> PhysicalUseWindow {
        self.window
    }
}

/// Current authority facts returned by a verifier after checking the final
/// operation and payload.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PhysicalUseVerification {
    current_revocation_revision: RevocationRevision,
    valid_until_unix_seconds: u64,
    verifier_receipt_sha256: Sha256Digest,
}

impl PhysicalUseVerification {
    pub fn new(
        current_revocation_revision: RevocationRevision,
        valid_until_unix_seconds: u64,
        verifier_receipt_sha256: Sha256Digest,
    ) -> Result<Self, VerifiedUseError> {
        if valid_until_unix_seconds == 0 {
            return Err(VerifiedUseError::VerifierValidityExpired {
                valid_until: valid_until_unix_seconds,
            });
        }
        Ok(Self {
            current_revocation_revision,
            valid_until_unix_seconds,
            verifier_receipt_sha256,
        })
    }

    pub const fn current_revocation_revision(&self) -> RevocationRevision {
        self.current_revocation_revision
    }

    pub const fn valid_until_unix_seconds(&self) -> u64 {
        self.valid_until_unix_seconds
    }

    pub const fn verifier_receipt_sha256(&self) -> &Sha256Digest {
        &self.verifier_receipt_sha256
    }
}

/// External current-authority verifier for one physical operation.
///
/// Implementations also satisfy [`CapabilityUseVerifier`], so the existing
/// broad per-use checks and the final-payload check are both mandatory.
pub trait PhysicalUseVerifier: CapabilityUseVerifier {
    fn verify_physical_use(
        &self,
        request: &PhysicalUseVerificationRequest<'_>,
    ) -> Result<PhysicalUseVerification, String>;
}

/// Issues a one-operation token after broad and final-payload verification.
pub fn verify_physical_capability_use<C, V>(
    capability: &Authorized<C>,
    request: PhysicalUseVerificationRequest<'_>,
    verifier: &V,
) -> Result<VerifiedUseToken<C>, VerifiedUseError>
where
    C: AuthorityCapability,
    V: PhysicalUseVerifier + ?Sized,
{
    let expected_action = request.kind.authority_action();
    if C::ACTION != expected_action || capability.action() != expected_action {
        return Err(VerifiedUseError::CapabilityKindActionMismatch {
            kind: request.kind,
            capability_action: capability.action(),
        });
    }

    let binding = capability
        .external_lease_binding()
        .ok_or(VerifiedUseError::ExternalAuthorityRequired(request.kind))?;
    verify_capability_use(
        capability,
        request.runtime_context,
        request.window.verified_at_unix_seconds,
        verifier,
    )?;

    if request.window.requested_expires_at_unix_seconds
        > binding.expires_at_unix_seconds()
    {
        return Err(VerifiedUseError::RequestedWindowExceedsAuthorityLease {
            requested_expires_at: request.window.requested_expires_at_unix_seconds,
            authority_expires_at: binding.expires_at_unix_seconds(),
        });
    }

    let verification = verifier
        .verify_physical_use(&request)
        .map_err(VerifiedUseError::PhysicalVerificationRejected)?;
    if verification.current_revocation_revision != request.expected_revocation_revision {
        return Err(VerifiedUseError::RevocationRevisionDrift {
            expected: request.expected_revocation_revision,
            current: verification.current_revocation_revision,
        });
    }
    if verification.valid_until_unix_seconds <= request.window.verified_at_unix_seconds {
        return Err(VerifiedUseError::VerifierValidityExpired {
            valid_until: verification.valid_until_unix_seconds,
        });
    }

    let expires_at_unix_seconds = request
        .window
        .requested_expires_at_unix_seconds
        .min(verification.valid_until_unix_seconds)
        .min(binding.expires_at_unix_seconds());
    if expires_at_unix_seconds <= request.window.verified_at_unix_seconds {
        return Err(VerifiedUseError::VerifierValidityExpired {
            valid_until: expires_at_unix_seconds,
        });
    }

    let runtime_authority_context_sha256 = request.runtime_context.digest();
    let token_sha256 = token_digest(
        request.kind,
        expected_action,
        request.operation_id,
        request.final_payload_sha256,
        &runtime_authority_context_sha256,
        request.expected_revocation_revision,
        request.window.verified_at_unix_seconds,
        expires_at_unix_seconds,
        &verification.verifier_receipt_sha256,
    );
    Ok(VerifiedUseToken {
        schema_version: VERIFIED_USE_SCHEMA_VERSION,
        kind: request.kind,
        action: expected_action,
        operation_id: request.operation_id.clone(),
        final_payload_sha256: request.final_payload_sha256.clone(),
        runtime_authority_context_sha256,
        revocation_revision: request.expected_revocation_revision,
        verified_at_unix_seconds: request.window.verified_at_unix_seconds,
        expires_at_unix_seconds,
        verifier_receipt_sha256: verification.verifier_receipt_sha256,
        token_sha256,
        marker: PhantomData,
    })
}

/// Final facts observed by the separate boundary adapter.
#[derive(Clone, Copy, Debug)]
pub struct PhysicalUseFinalCheck<'a> {
    kind: PhysicalCapabilityKind,
    operation_id: &'a OperationId,
    final_payload_sha256: &'a Sha256Digest,
    runtime_context: &'a RuntimeAuthorityContext,
    current_revocation_revision: RevocationRevision,
    crossed_at_unix_seconds: u64,
}

impl<'a> PhysicalUseFinalCheck<'a> {
    pub fn new(
        kind: PhysicalCapabilityKind,
        operation_id: &'a OperationId,
        final_payload_sha256: &'a Sha256Digest,
        runtime_context: &'a RuntimeAuthorityContext,
        current_revocation_revision: RevocationRevision,
        crossed_at_unix_seconds: u64,
    ) -> Self {
        Self {
            kind,
            operation_id,
            final_payload_sha256,
            runtime_context,
            current_revocation_revision,
            crossed_at_unix_seconds,
        }
    }
}

/// Non-cloneable and non-serializable one-operation physical-use token.
pub struct VerifiedUseToken<C>
where
    C: AuthorityCapability,
{
    schema_version: u32,
    kind: PhysicalCapabilityKind,
    action: AuthorityAction,
    operation_id: OperationId,
    final_payload_sha256: Sha256Digest,
    runtime_authority_context_sha256: Sha256Digest,
    revocation_revision: RevocationRevision,
    verified_at_unix_seconds: u64,
    expires_at_unix_seconds: u64,
    verifier_receipt_sha256: Sha256Digest,
    token_sha256: Sha256Digest,
    marker: PhantomData<C>,
}

impl<C> VerifiedUseToken<C>
where
    C: AuthorityCapability,
{
    pub const fn kind(&self) -> PhysicalCapabilityKind {
        self.kind
    }

    pub const fn action(&self) -> AuthorityAction {
        self.action
    }

    pub const fn expires_at_unix_seconds(&self) -> u64 {
        self.expires_at_unix_seconds
    }

    pub const fn token_sha256(&self) -> &Sha256Digest {
        &self.token_sha256
    }

    /// Rechecks the actual boundary facts and consumes this token by value.
    pub fn consume(
        self,
        final_check: PhysicalUseFinalCheck<'_>,
    ) -> Result<VerifiedUseWitness, VerifiedUseError> {
        if self.schema_version != VERIFIED_USE_SCHEMA_VERSION {
            return Err(VerifiedUseError::TokenIntegrityDrift);
        }
        if final_check.kind != self.kind {
            return Err(VerifiedUseError::FinalCapabilityKindDrift {
                verified: self.kind,
                observed: final_check.kind,
            });
        }
        if final_check.operation_id != &self.operation_id {
            return Err(VerifiedUseError::FinalOperationDrift);
        }
        if final_check.final_payload_sha256 != &self.final_payload_sha256 {
            return Err(VerifiedUseError::FinalPayloadDrift);
        }
        let observed_context_sha256 = final_check.runtime_context.digest();
        if observed_context_sha256 != self.runtime_authority_context_sha256 {
            return Err(VerifiedUseError::FinalRuntimeContextDrift);
        }
        if final_check.current_revocation_revision != self.revocation_revision {
            return Err(VerifiedUseError::FinalRevocationRevisionDrift {
                verified: self.revocation_revision,
                observed: final_check.current_revocation_revision,
            });
        }
        if final_check.crossed_at_unix_seconds < self.verified_at_unix_seconds {
            return Err(VerifiedUseError::CrossedBeforeVerification {
                verified_at: self.verified_at_unix_seconds,
                crossed_at: final_check.crossed_at_unix_seconds,
            });
        }
        if final_check.crossed_at_unix_seconds >= self.expires_at_unix_seconds {
            return Err(VerifiedUseError::TokenExpired {
                expires_at: self.expires_at_unix_seconds,
                crossed_at: final_check.crossed_at_unix_seconds,
            });
        }

        let recomputed_token_sha256 = token_digest(
            self.kind,
            self.action,
            &self.operation_id,
            &self.final_payload_sha256,
            &self.runtime_authority_context_sha256,
            self.revocation_revision,
            self.verified_at_unix_seconds,
            self.expires_at_unix_seconds,
            &self.verifier_receipt_sha256,
        );
        if recomputed_token_sha256 != self.token_sha256 {
            return Err(VerifiedUseError::TokenIntegrityDrift);
        }

        VerifiedUseWitness::new(
            self.kind,
            self.action,
            self.operation_id,
            self.final_payload_sha256,
            self.runtime_authority_context_sha256,
            self.revocation_revision,
            self.verified_at_unix_seconds,
            self.expires_at_unix_seconds,
            final_check.crossed_at_unix_seconds,
            self.verifier_receipt_sha256,
            self.token_sha256,
        )
    }
}

impl<C> fmt::Debug for VerifiedUseToken<C>
where
    C: AuthorityCapability,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("VerifiedUseToken")
            .field("schema_version", &self.schema_version)
            .field("kind", &self.kind)
            .field("action", &self.action)
            .field("operation_id", &self.operation_id.as_str())
            .field("revocation_revision", &self.revocation_revision)
            .field("verified_at_unix_seconds", &self.verified_at_unix_seconds)
            .field("expires_at_unix_seconds", &self.expires_at_unix_seconds)
            .field("token_sha256", &self.token_sha256)
            .finish_non_exhaustive()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VerifiedUseWitness {
    schema_version: u32,
    kind: PhysicalCapabilityKind,
    action: AuthorityAction,
    operation_id: OperationId,
    final_payload_sha256: Sha256Digest,
    runtime_authority_context_sha256: Sha256Digest,
    revocation_revision: RevocationRevision,
    verified_at_unix_seconds: u64,
    expires_at_unix_seconds: u64,
    crossed_at_unix_seconds: u64,
    verifier_receipt_sha256: Sha256Digest,
    token_sha256: Sha256Digest,
    witness_sha256: Sha256Digest,
}

impl VerifiedUseWitness {
    #[allow(clippy::too_many_arguments)]
    fn new(
        kind: PhysicalCapabilityKind,
        action: AuthorityAction,
        operation_id: OperationId,
        final_payload_sha256: Sha256Digest,
        runtime_authority_context_sha256: Sha256Digest,
        revocation_revision: RevocationRevision,
        verified_at_unix_seconds: u64,
        expires_at_unix_seconds: u64,
        crossed_at_unix_seconds: u64,
        verifier_receipt_sha256: Sha256Digest,
        token_sha256: Sha256Digest,
    ) -> Result<Self, VerifiedUseError> {
        let witness_sha256 = witness_digest(
            kind,
            action,
            &operation_id,
            &final_payload_sha256,
            &runtime_authority_context_sha256,
            revocation_revision,
            verified_at_unix_seconds,
            expires_at_unix_seconds,
            crossed_at_unix_seconds,
            &verifier_receipt_sha256,
            &token_sha256,
        );
        let witness = Self {
            schema_version: VERIFIED_USE_SCHEMA_VERSION,
            kind,
            action,
            operation_id,
            final_payload_sha256,
            runtime_authority_context_sha256,
            revocation_revision,
            verified_at_unix_seconds,
            expires_at_unix_seconds,
            crossed_at_unix_seconds,
            verifier_receipt_sha256,
            token_sha256,
            witness_sha256,
        };
        witness.validate()?;
        Ok(witness)
    }

    pub const fn schema_version(&self) -> u32 {
        self.schema_version
    }

    pub const fn kind(&self) -> PhysicalCapabilityKind {
        self.kind
    }

    pub const fn action(&self) -> AuthorityAction {
        self.action
    }

    pub const fn operation_id(&self) -> &OperationId {
        &self.operation_id
    }

    pub const fn final_payload_sha256(&self) -> &Sha256Digest {
        &self.final_payload_sha256
    }

    pub const fn runtime_authority_context_sha256(&self) -> &Sha256Digest {
        &self.runtime_authority_context_sha256
    }

    pub const fn revocation_revision(&self) -> RevocationRevision {
        self.revocation_revision
    }

    pub const fn verified_at_unix_seconds(&self) -> u64 {
        self.verified_at_unix_seconds
    }

    pub const fn expires_at_unix_seconds(&self) -> u64 {
        self.expires_at_unix_seconds
    }

    pub const fn crossed_at_unix_seconds(&self) -> u64 {
        self.crossed_at_unix_seconds
    }

    pub const fn verifier_receipt_sha256(&self) -> &Sha256Digest {
        &self.verifier_receipt_sha256
    }

    pub const fn token_sha256(&self) -> &Sha256Digest {
        &self.token_sha256
    }

    pub const fn witness_sha256(&self) -> &Sha256Digest {
        &self.witness_sha256
    }

    pub fn validate(&self) -> Result<(), VerifiedUseError> {
        if self.schema_version != VERIFIED_USE_SCHEMA_VERSION
            || self.kind.authority_action() != self.action
            || self.verified_at_unix_seconds == 0
            || self.crossed_at_unix_seconds < self.verified_at_unix_seconds
            || self.expires_at_unix_seconds <= self.crossed_at_unix_seconds
        {
            return Err(VerifiedUseError::WitnessIntegrityDrift);
        }
        let expected_token_sha256 = token_digest(
            self.kind,
            self.action,
            &self.operation_id,
            &self.final_payload_sha256,
            &self.runtime_authority_context_sha256,
            self.revocation_revision,
            self.verified_at_unix_seconds,
            self.expires_at_unix_seconds,
            &self.verifier_receipt_sha256,
        );
        if expected_token_sha256 != self.token_sha256 {
            return Err(VerifiedUseError::WitnessIntegrityDrift);
        }
        let expected_witness_sha256 = witness_digest(
            self.kind,
            self.action,
            &self.operation_id,
            &self.final_payload_sha256,
            &self.runtime_authority_context_sha256,
            self.revocation_revision,
            self.verified_at_unix_seconds,
            self.expires_at_unix_seconds,
            self.crossed_at_unix_seconds,
            &self.verifier_receipt_sha256,
            &self.token_sha256,
        );
        if expected_witness_sha256 != self.witness_sha256 {
            return Err(VerifiedUseError::WitnessIntegrityDrift);
        }
        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum VerifiedUseError {
    InvalidRevocationRevision,
    InvalidVerificationWindow,
    CapabilityKindActionMismatch {
        kind: PhysicalCapabilityKind,
        capability_action: AuthorityAction,
    },
    ExternalAuthorityRequired(PhysicalCapabilityKind),
    RequestedWindowExceedsAuthorityLease {
        requested_expires_at: u64,
        authority_expires_at: u64,
    },
    Authority(AuthorityError),
    PhysicalVerificationRejected(String),
    RevocationRevisionDrift {
        expected: RevocationRevision,
        current: RevocationRevision,
    },
    VerifierValidityExpired {
        valid_until: u64,
    },
    FinalCapabilityKindDrift {
        verified: PhysicalCapabilityKind,
        observed: PhysicalCapabilityKind,
    },
    FinalOperationDrift,
    FinalPayloadDrift,
    FinalRuntimeContextDrift,
    FinalRevocationRevisionDrift {
        verified: RevocationRevision,
        observed: RevocationRevision,
    },
    CrossedBeforeVerification {
        verified_at: u64,
        crossed_at: u64,
    },
    TokenExpired {
        expires_at: u64,
        crossed_at: u64,
    },
    TokenIntegrityDrift,
    WitnessIntegrityDrift,
}

impl fmt::Display for VerifiedUseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRevocationRevision => {
                formatter.write_str("revocation revision must be non-zero")
            }
            Self::InvalidVerificationWindow => formatter.write_str(
                "verified-use window requires non-zero verification time and later expiry",
            ),
            Self::CapabilityKindActionMismatch {
                kind,
                capability_action,
            } => write!(
                formatter,
                "physical capability kind {} requires action {}, not {}",
                kind.as_str(),
                kind.authority_action().as_str(),
                capability_action.as_str()
            ),
            Self::ExternalAuthorityRequired(kind) => write!(
                formatter,
                "physical capability kind {} requires an externally verified lease",
                kind.as_str()
            ),
            Self::RequestedWindowExceedsAuthorityLease {
                requested_expires_at,
                authority_expires_at,
            } => write!(
                formatter,
                "requested verified-use expiry {requested_expires_at} exceeds authority lease expiry {authority_expires_at}"
            ),
            Self::Authority(error) => write!(formatter, "authority verification failed: {error}"),
            Self::PhysicalVerificationRejected(reason) => {
                write!(formatter, "physical-use verifier rejected request: {reason}")
            }
            Self::RevocationRevisionDrift { expected, current } => write!(
                formatter,
                "revocation revision drifted from {} to {}",
                expected.get(),
                current.get()
            ),
            Self::VerifierValidityExpired { valid_until } => write!(
                formatter,
                "physical-use verifier validity does not extend beyond verification time: {valid_until}"
            ),
            Self::FinalCapabilityKindDrift { verified, observed } => write!(
                formatter,
                "physical capability kind drifted from {} to {}",
                verified.as_str(),
                observed.as_str()
            ),
            Self::FinalOperationDrift => {
                formatter.write_str("physical operation id drifted before boundary crossing")
            }
            Self::FinalPayloadDrift => {
                formatter.write_str("final payload digest drifted before boundary crossing")
            }
            Self::FinalRuntimeContextDrift => formatter
                .write_str("runtime authority context drifted before boundary crossing"),
            Self::FinalRevocationRevisionDrift { verified, observed } => write!(
                formatter,
                "revocation revision drifted before boundary crossing from {} to {}",
                verified.get(),
                observed.get()
            ),
            Self::CrossedBeforeVerification {
                verified_at,
                crossed_at,
            } => write!(
                formatter,
                "boundary crossing time {crossed_at} precedes verification time {verified_at}"
            ),
            Self::TokenExpired {
                expires_at,
                crossed_at,
            } => write!(
                formatter,
                "verified-use token expired at {expires_at} before crossing at {crossed_at}"
            ),
            Self::TokenIntegrityDrift => {
                formatter.write_str("verified-use token integrity drifted")
            }
            Self::WitnessIntegrityDrift => {
                formatter.write_str("verified-use witness integrity drifted")
            }
        }
    }
}

impl std::error::Error for VerifiedUseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Authority(error) => Some(error),
            _ => None,
        }
    }
}

impl From<AuthorityError> for VerifiedUseError {
    fn from(error: AuthorityError) -> Self {
        Self::Authority(error)
    }
}

#[allow(clippy::too_many_arguments)]
fn token_digest(
    kind: PhysicalCapabilityKind,
    action: AuthorityAction,
    operation_id: &OperationId,
    final_payload_sha256: &Sha256Digest,
    runtime_authority_context_sha256: &Sha256Digest,
    revocation_revision: RevocationRevision,
    verified_at_unix_seconds: u64,
    expires_at_unix_seconds: u64,
    verifier_receipt_sha256: &Sha256Digest,
) -> Sha256Digest {
    let mut bytes = Vec::new();
    frame(&mut bytes, b"hepta:verified-use-token:v1");
    frame(&mut bytes, &VERIFIED_USE_SCHEMA_VERSION.to_be_bytes());
    frame(&mut bytes, kind.as_str().as_bytes());
    frame(&mut bytes, action.as_str().as_bytes());
    frame(&mut bytes, operation_id.as_str().as_bytes());
    frame(&mut bytes, final_payload_sha256.as_str().as_bytes());
    frame(
        &mut bytes,
        runtime_authority_context_sha256.as_str().as_bytes(),
    );
    frame(&mut bytes, &revocation_revision.get().to_be_bytes());
    frame(&mut bytes, &verified_at_unix_seconds.to_be_bytes());
    frame(&mut bytes, &expires_at_unix_seconds.to_be_bytes());
    frame(&mut bytes, verifier_receipt_sha256.as_str().as_bytes());
    Sha256Digest::for_bytes(&bytes)
}

#[allow(clippy::too_many_arguments)]
fn witness_digest(
    kind: PhysicalCapabilityKind,
    action: AuthorityAction,
    operation_id: &OperationId,
    final_payload_sha256: &Sha256Digest,
    runtime_authority_context_sha256: &Sha256Digest,
    revocation_revision: RevocationRevision,
    verified_at_unix_seconds: u64,
    expires_at_unix_seconds: u64,
    crossed_at_unix_seconds: u64,
    verifier_receipt_sha256: &Sha256Digest,
    token_sha256: &Sha256Digest,
) -> Sha256Digest {
    let mut bytes = Vec::new();
    frame(&mut bytes, b"hepta:verified-use-witness:v1");
    frame(&mut bytes, &VERIFIED_USE_SCHEMA_VERSION.to_be_bytes());
    frame(&mut bytes, kind.as_str().as_bytes());
    frame(&mut bytes, action.as_str().as_bytes());
    frame(&mut bytes, operation_id.as_str().as_bytes());
    frame(&mut bytes, final_payload_sha256.as_str().as_bytes());
    frame(
        &mut bytes,
        runtime_authority_context_sha256.as_str().as_bytes(),
    );
    frame(&mut bytes, &revocation_revision.get().to_be_bytes());
    frame(&mut bytes, &verified_at_unix_seconds.to_be_bytes());
    frame(&mut bytes, &expires_at_unix_seconds.to_be_bytes());
    frame(&mut bytes, &crossed_at_unix_seconds.to_be_bytes());
    frame(&mut bytes, verifier_receipt_sha256.as_str().as_bytes());
    frame(&mut bytes, token_sha256.as_str().as_bytes());
    Sha256Digest::for_bytes(&bytes)
}

fn frame(target: &mut Vec<u8>, part: &[u8]) {
    target.extend_from_slice(&(part.len() as u64).to_be_bytes());
    target.extend_from_slice(part);
}

#[cfg(test)]
#[path = "verified_use_tests.rs"]
mod tests;
