//! Final-payload, operation-bound verification for irreversible capability use.
//!
//! A broad [`Authorized`] capability is necessary but not sufficient at a
//! physical boundary. This module composes the existing per-use authority
//! verifier with a trusted clock, a current-revision verifier, and an atomic
//! durable single-use claim. The resulting boundary permit is private,
//! non-cloneable, and non-serializable.

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

pub const VERIFIED_USE_SCHEMA_VERSION: u32 = 2;

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

/// Caller-selected upper bound. The verification and boundary times are
/// always obtained from [`TrustedPhysicalClock`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PhysicalUseWindow {
    requested_expires_at_unix_seconds: u64,
}

impl PhysicalUseWindow {
    pub fn new(requested_expires_at_unix_seconds: u64) -> Result<Self, VerifiedUseError> {
        if requested_expires_at_unix_seconds == 0 {
            return Err(VerifiedUseError::InvalidVerificationWindow);
        }
        Ok(Self {
            requested_expires_at_unix_seconds,
        })
    }

    pub const fn requested_expires_at_unix_seconds(self) -> u64 {
        self.requested_expires_at_unix_seconds
    }
}

/// Exact final-payload request presented to the physical-use verifier.
///
/// `observed_at_unix_seconds` is filled by the kernel from a trusted clock. It
/// is never accepted from an ordinary caller.
#[derive(Clone, Copy, Debug)]
pub struct PhysicalUseVerificationRequest<'a> {
    kind: PhysicalCapabilityKind,
    operation_id: &'a OperationId,
    final_payload_sha256: &'a Sha256Digest,
    runtime_context: &'a RuntimeAuthorityContext,
    expected_revocation_revision: RevocationRevision,
    observed_at_unix_seconds: u64,
    requested_expires_at_unix_seconds: u64,
}

impl<'a> PhysicalUseVerificationRequest<'a> {
    fn new(
        kind: PhysicalCapabilityKind,
        operation_id: &'a OperationId,
        final_payload_sha256: &'a Sha256Digest,
        runtime_context: &'a RuntimeAuthorityContext,
        expected_revocation_revision: RevocationRevision,
        observed_at_unix_seconds: u64,
        requested_expires_at_unix_seconds: u64,
    ) -> Self {
        Self {
            kind,
            operation_id,
            final_payload_sha256,
            runtime_context,
            expected_revocation_revision,
            observed_at_unix_seconds,
            requested_expires_at_unix_seconds,
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

    pub const fn observed_at_unix_seconds(&self) -> u64 {
        self.observed_at_unix_seconds
    }

    pub const fn requested_expires_at_unix_seconds(&self) -> u64 {
        self.requested_expires_at_unix_seconds
    }
}

/// Current authority facts returned after checking the exact operation and
/// final payload.
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

/// Trusted time source owned by the authority/boundary composition root.
pub trait TrustedPhysicalClock {
    fn now_unix_seconds(&self) -> Result<u64, String>;
}

/// External current-authority verifier for one physical operation.
///
/// The same verifier is called once while issuing the token and again at the
/// final boundary. The final call is mandatory and cannot be replaced by a
/// caller-supplied revision.
pub trait PhysicalUseVerifier: CapabilityUseVerifier {
    fn verify_physical_use(
        &self,
        request: &PhysicalUseVerificationRequest<'_>,
    ) -> Result<PhysicalUseVerification, String>;
}

/// Exact caller-owned final facts. Time and current revocation revision are
/// deliberately absent and are obtained from trusted boundary dependencies.
#[derive(Clone, Copy, Debug)]
pub struct PhysicalUseFinalCheck<'a> {
    kind: PhysicalCapabilityKind,
    operation_id: &'a OperationId,
    final_payload_sha256: &'a Sha256Digest,
    runtime_context: &'a RuntimeAuthorityContext,
}

impl<'a> PhysicalUseFinalCheck<'a> {
    pub fn new(
        kind: PhysicalCapabilityKind,
        operation_id: &'a OperationId,
        final_payload_sha256: &'a Sha256Digest,
        runtime_context: &'a RuntimeAuthorityContext,
    ) -> Self {
        Self {
            kind,
            operation_id,
            final_payload_sha256,
            runtime_context,
        }
    }

    pub const fn kind(&self) -> PhysicalCapabilityKind {
        self.kind
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
}

/// Stable claim identity. `operation_scope_sha256` is unique for one
/// `(capability kind, operation id)` pair; `claim_sha256` additionally binds
/// the exact final payload. A durable store must reject both replay and
/// same-operation/different-payload conflict.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PhysicalUseClaimKey {
    operation_scope_sha256: Sha256Digest,
    claim_sha256: Sha256Digest,
}

impl PhysicalUseClaimKey {
    fn for_operation(
        kind: PhysicalCapabilityKind,
        operation_id: &OperationId,
        final_payload_sha256: &Sha256Digest,
    ) -> Self {
        let mut scope_bytes = Vec::new();
        frame(&mut scope_bytes, b"hepta:physical-use-operation-scope:v2");
        frame(&mut scope_bytes, kind.as_str().as_bytes());
        frame(&mut scope_bytes, operation_id.as_str().as_bytes());
        let operation_scope_sha256 = Sha256Digest::for_bytes(&scope_bytes);

        let mut claim_bytes = Vec::new();
        frame(&mut claim_bytes, b"hepta:physical-use-claim-key:v2");
        frame(
            &mut claim_bytes,
            operation_scope_sha256.as_str().as_bytes(),
        );
        frame(
            &mut claim_bytes,
            final_payload_sha256.as_str().as_bytes(),
        );
        let claim_sha256 = Sha256Digest::for_bytes(&claim_bytes);

        Self {
            operation_scope_sha256,
            claim_sha256,
        }
    }

    pub const fn operation_scope_sha256(&self) -> &Sha256Digest {
        &self.operation_scope_sha256
    }

    pub const fn claim_sha256(&self) -> &Sha256Digest {
        &self.claim_sha256
    }
}

/// Request to atomically and durably claim the irreversible boundary.
///
/// A production store must durably persist this request before returning. A
/// crash after success is an indeterminate boundary attempt and must not be
/// retried by issuing another claim.
#[derive(Clone, Copy, Debug)]
pub struct PhysicalUseClaimRequest<'a> {
    claim_key: &'a PhysicalUseClaimKey,
    token_sha256: &'a Sha256Digest,
    kind: PhysicalCapabilityKind,
    operation_id: &'a OperationId,
    final_payload_sha256: &'a Sha256Digest,
    runtime_authority_context_sha256: &'a Sha256Digest,
    revocation_revision: RevocationRevision,
    claimed_at_unix_seconds: u64,
}

impl<'a> PhysicalUseClaimRequest<'a> {
    #[allow(clippy::too_many_arguments)]
    fn new(
        claim_key: &'a PhysicalUseClaimKey,
        token_sha256: &'a Sha256Digest,
        kind: PhysicalCapabilityKind,
        operation_id: &'a OperationId,
        final_payload_sha256: &'a Sha256Digest,
        runtime_authority_context_sha256: &'a Sha256Digest,
        revocation_revision: RevocationRevision,
        claimed_at_unix_seconds: u64,
    ) -> Self {
        Self {
            claim_key,
            token_sha256,
            kind,
            operation_id,
            final_payload_sha256,
            runtime_authority_context_sha256,
            revocation_revision,
            claimed_at_unix_seconds,
        }
    }

    pub const fn claim_key(&self) -> &'a PhysicalUseClaimKey {
        self.claim_key
    }

    pub const fn token_sha256(&self) -> &'a Sha256Digest {
        self.token_sha256
    }

    pub const fn kind(&self) -> PhysicalCapabilityKind {
        self.kind
    }

    pub const fn operation_id(&self) -> &'a OperationId {
        self.operation_id
    }

    pub const fn final_payload_sha256(&self) -> &'a Sha256Digest {
        self.final_payload_sha256
    }

    pub const fn runtime_authority_context_sha256(&self) -> &'a Sha256Digest {
        self.runtime_authority_context_sha256
    }

    pub const fn revocation_revision(&self) -> RevocationRevision {
        self.revocation_revision
    }

    pub const fn claimed_at_unix_seconds(&self) -> u64 {
        self.claimed_at_unix_seconds
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PhysicalUseClaimStoreError {
    AlreadyClaimed,
    OperationPayloadConflict,
    Unavailable(String),
    Rejected(String),
}

impl fmt::Display for PhysicalUseClaimStoreError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AlreadyClaimed => formatter.write_str("physical-use operation is already claimed"),
            Self::OperationPayloadConflict => {
                formatter.write_str("physical-use operation was claimed for another payload")
            }
            Self::Unavailable(reason) => {
                write!(formatter, "physical-use claim store unavailable: {reason}")
            }
            Self::Rejected(reason) => {
                write!(formatter, "physical-use claim store rejected claim: {reason}")
            }
        }
    }
}

impl std::error::Error for PhysicalUseClaimStoreError {}

/// Receipt returned only after an atomic durable claim has been committed.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PhysicalUseClaimReceipt {
    claim_key: PhysicalUseClaimKey,
    claim_revision: u64,
    claimed_at_unix_seconds: u64,
    store_receipt_sha256: Sha256Digest,
}

impl PhysicalUseClaimReceipt {
    pub fn new(
        claim_key: PhysicalUseClaimKey,
        claim_revision: u64,
        claimed_at_unix_seconds: u64,
        store_receipt_sha256: Sha256Digest,
    ) -> Result<Self, PhysicalUseClaimStoreError> {
        if claim_revision == 0 || claimed_at_unix_seconds == 0 {
            return Err(PhysicalUseClaimStoreError::Rejected(
                "claim revision and claim time must be non-zero".to_string(),
            ));
        }
        Ok(Self {
            claim_key,
            claim_revision,
            claimed_at_unix_seconds,
            store_receipt_sha256,
        })
    }

    pub const fn claim_key(&self) -> &PhysicalUseClaimKey {
        &self.claim_key
    }

    pub const fn claim_revision(&self) -> u64 {
        self.claim_revision
    }

    pub const fn claimed_at_unix_seconds(&self) -> u64 {
        self.claimed_at_unix_seconds
    }

    pub const fn store_receipt_sha256(&self) -> &Sha256Digest {
        &self.store_receipt_sha256
    }

    fn validate_for(
        &self,
        request: &PhysicalUseClaimRequest<'_>,
    ) -> Result<(), VerifiedUseError> {
        if &self.claim_key != request.claim_key
            || self.claim_revision == 0
            || self.claimed_at_unix_seconds != request.claimed_at_unix_seconds
        {
            return Err(VerifiedUseError::ClaimReceiptIntegrityDrift);
        }
        Ok(())
    }
}

/// Atomic durable uniqueness boundary for irreversible use.
///
/// Implementations must enforce one successful claim per operation scope.
/// Repeating the same claim returns [`PhysicalUseClaimStoreError::AlreadyClaimed`];
/// changing the payload under the same operation scope returns
/// [`PhysicalUseClaimStoreError::OperationPayloadConflict`].
pub trait PhysicalUseClaimStore {
    fn claim_once(
        &self,
        request: &PhysicalUseClaimRequest<'_>,
    ) -> Result<PhysicalUseClaimReceipt, PhysicalUseClaimStoreError>;
}

/// Issues a one-operation token after broad and final-payload verification.
///
/// The verification instant is read from `clock`, not supplied by the caller.
pub fn verify_physical_capability_use<C, V, T>(
    capability: &Authorized<C>,
    kind: PhysicalCapabilityKind,
    operation_id: &OperationId,
    final_payload_sha256: &Sha256Digest,
    runtime_context: &RuntimeAuthorityContext,
    expected_revocation_revision: RevocationRevision,
    window: PhysicalUseWindow,
    verifier: &V,
    clock: &T,
) -> Result<VerifiedUseToken<C>, VerifiedUseError>
where
    C: AuthorityCapability,
    V: PhysicalUseVerifier + ?Sized,
    T: TrustedPhysicalClock + ?Sized,
{
    let expected_action = kind.authority_action();
    if C::ACTION != expected_action || capability.action() != expected_action {
        return Err(VerifiedUseError::CapabilityKindActionMismatch {
            kind,
            capability_action: capability.action(),
        });
    }

    let binding = capability
        .external_lease_binding()
        .ok_or(VerifiedUseError::ExternalAuthorityRequired(kind))?;

    let verified_at_unix_seconds = trusted_now(clock)?;
    if window.requested_expires_at_unix_seconds <= verified_at_unix_seconds {
        return Err(VerifiedUseError::InvalidVerificationWindow);
    }

    verify_capability_use(
        capability,
        runtime_context,
        verified_at_unix_seconds,
        verifier,
    )?;

    if window.requested_expires_at_unix_seconds > binding.expires_at_unix_seconds() {
        return Err(VerifiedUseError::RequestedWindowExceedsAuthorityLease {
            requested_expires_at: window.requested_expires_at_unix_seconds,
            authority_expires_at: binding.expires_at_unix_seconds(),
        });
    }

    let request = PhysicalUseVerificationRequest::new(
        kind,
        operation_id,
        final_payload_sha256,
        runtime_context,
        expected_revocation_revision,
        verified_at_unix_seconds,
        window.requested_expires_at_unix_seconds,
    );
    let verification = verifier
        .verify_physical_use(&request)
        .map_err(VerifiedUseError::PhysicalVerificationRejected)?;
    validate_current_verification(
        expected_revocation_revision,
        verified_at_unix_seconds,
        &verification,
        false,
    )?;

    let expires_at_unix_seconds = window
        .requested_expires_at_unix_seconds
        .min(verification.valid_until_unix_seconds)
        .min(binding.expires_at_unix_seconds());
    if expires_at_unix_seconds <= verified_at_unix_seconds {
        return Err(VerifiedUseError::VerifierValidityExpired {
            valid_until: expires_at_unix_seconds,
        });
    }

    let runtime_authority_context_sha256 = runtime_context.digest();
    let token_sha256 = token_digest(
        kind,
        expected_action,
        operation_id,
        final_payload_sha256,
        &runtime_authority_context_sha256,
        expected_revocation_revision,
        verified_at_unix_seconds,
        expires_at_unix_seconds,
        &verification.verifier_receipt_sha256,
    );

    Ok(VerifiedUseToken {
        schema_version: VERIFIED_USE_SCHEMA_VERSION,
        kind,
        action: expected_action,
        operation_id: operation_id.clone(),
        final_payload_sha256: final_payload_sha256.clone(),
        runtime_authority_context_sha256,
        revocation_revision: expected_revocation_revision,
        verified_at_unix_seconds,
        expires_at_unix_seconds,
        issuance_verifier_receipt_sha256: verification.verifier_receipt_sha256,
        token_sha256,
        marker: PhantomData,
    })
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
    issuance_verifier_receipt_sha256: Sha256Digest,
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

    pub const fn verified_at_unix_seconds(&self) -> u64 {
        self.verified_at_unix_seconds
    }

    pub const fn expires_at_unix_seconds(&self) -> u64 {
        self.expires_at_unix_seconds
    }

    pub const fn token_sha256(&self) -> &Sha256Digest {
        &self.token_sha256
    }

    /// Re-verifies current authority, obtains trusted boundary time, and
    /// commits the durable single-use claim before returning a permit.
    ///
    /// The returned permit is the only value a physical adapter may consume.
    /// A successful claim followed by process death is indeterminate and must
    /// be reconciled; another permit must not be issued for that operation.
    pub fn consume_at_boundary<V, T, S>(
        self,
        final_check: PhysicalUseFinalCheck<'_>,
        verifier: &V,
        clock: &T,
        claim_store: &S,
    ) -> Result<VerifiedUseBoundaryPermit<C>, VerifiedUseError>
    where
        V: PhysicalUseVerifier + ?Sized,
        T: TrustedPhysicalClock + ?Sized,
        S: PhysicalUseClaimStore + ?Sized,
    {
        self.validate_integrity()?;
        self.validate_final_check(&final_check)?;

        let crossed_at_unix_seconds = trusted_now(clock)?;
        if crossed_at_unix_seconds < self.verified_at_unix_seconds {
            return Err(VerifiedUseError::ClockRollback {
                verified_at: self.verified_at_unix_seconds,
                observed_at: crossed_at_unix_seconds,
            });
        }
        if crossed_at_unix_seconds >= self.expires_at_unix_seconds {
            return Err(VerifiedUseError::TokenExpired {
                expires_at: self.expires_at_unix_seconds,
                crossed_at: crossed_at_unix_seconds,
            });
        }

        let final_request = PhysicalUseVerificationRequest::new(
            self.kind,
            &self.operation_id,
            &self.final_payload_sha256,
            final_check.runtime_context,
            self.revocation_revision,
            crossed_at_unix_seconds,
            self.expires_at_unix_seconds,
        );
        let final_verification = verifier
            .verify_physical_use(&final_request)
            .map_err(VerifiedUseError::FinalPhysicalVerificationRejected)?;
        validate_current_verification(
            self.revocation_revision,
            crossed_at_unix_seconds,
            &final_verification,
            true,
        )?;
        if final_verification.valid_until_unix_seconds <= crossed_at_unix_seconds {
            return Err(VerifiedUseError::FinalVerifierValidityExpired {
                valid_until: final_verification.valid_until_unix_seconds,
            });
        }

        let claim_key = PhysicalUseClaimKey::for_operation(
            self.kind,
            &self.operation_id,
            &self.final_payload_sha256,
        );
        let claim_request = PhysicalUseClaimRequest::new(
            &claim_key,
            &self.token_sha256,
            self.kind,
            &self.operation_id,
            &self.final_payload_sha256,
            &self.runtime_authority_context_sha256,
            self.revocation_revision,
            crossed_at_unix_seconds,
        );
        let claim_receipt = claim_store
            .claim_once(&claim_request)
            .map_err(VerifiedUseError::ClaimStore)?;
        claim_receipt.validate_for(&claim_request)?;

        let witness = VerifiedUseWitness::new(
            self.kind,
            self.action,
            self.operation_id,
            self.final_payload_sha256,
            self.runtime_authority_context_sha256,
            self.revocation_revision,
            self.verified_at_unix_seconds,
            self.expires_at_unix_seconds,
            crossed_at_unix_seconds,
            self.issuance_verifier_receipt_sha256,
            final_verification.verifier_receipt_sha256,
            self.token_sha256,
            claim_receipt,
        )?;

        Ok(VerifiedUseBoundaryPermit {
            witness,
            marker: PhantomData,
        })
    }

    fn validate_integrity(&self) -> Result<(), VerifiedUseError> {
        if self.schema_version != VERIFIED_USE_SCHEMA_VERSION {
            return Err(VerifiedUseError::TokenIntegrityDrift);
        }
        let recomputed = token_digest(
            self.kind,
            self.action,
            &self.operation_id,
            &self.final_payload_sha256,
            &self.runtime_authority_context_sha256,
            self.revocation_revision,
            self.verified_at_unix_seconds,
            self.expires_at_unix_seconds,
            &self.issuance_verifier_receipt_sha256,
        );
        if recomputed != self.token_sha256 {
            return Err(VerifiedUseError::TokenIntegrityDrift);
        }
        Ok(())
    }

    fn validate_final_check(
        &self,
        final_check: &PhysicalUseFinalCheck<'_>,
    ) -> Result<(), VerifiedUseError> {
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
        if final_check.runtime_context.digest() != self.runtime_authority_context_sha256 {
            return Err(VerifiedUseError::FinalRuntimeContextDrift);
        }
        Ok(())
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

/// Non-cloneable and non-serializable permit consumed by the physical adapter.
pub struct VerifiedUseBoundaryPermit<C>
where
    C: AuthorityCapability,
{
    witness: VerifiedUseWitness,
    marker: PhantomData<C>,
}

impl<C> VerifiedUseBoundaryPermit<C>
where
    C: AuthorityCapability,
{
    pub const fn kind(&self) -> PhysicalCapabilityKind {
        self.witness.kind
    }

    pub const fn operation_id(&self) -> &OperationId {
        &self.witness.operation_id
    }

    pub const fn final_payload_sha256(&self) -> &Sha256Digest {
        &self.witness.final_payload_sha256
    }

    pub const fn claim_key(&self) -> &PhysicalUseClaimKey {
        &self.witness.claim_key
    }

    /// Produces the durable audit witness. This witness proves only that the
    /// pre-crossing claim was committed; it is not proof of external effect
    /// completion.
    pub fn into_witness(self) -> VerifiedUseWitness {
        self.witness
    }
}

impl<C> fmt::Debug for VerifiedUseBoundaryPermit<C>
where
    C: AuthorityCapability,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("VerifiedUseBoundaryPermit")
            .field("kind", &self.witness.kind)
            .field("operation_id", &self.witness.operation_id.as_str())
            .field("claim_key", &self.witness.claim_key)
            .finish_non_exhaustive()
    }
}

/// Serializable record of a committed pre-crossing claim.
///
/// This is intentionally non-authoritative for effect completion.
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
    issuance_verifier_receipt_sha256: Sha256Digest,
    final_verifier_receipt_sha256: Sha256Digest,
    token_sha256: Sha256Digest,
    claim_key: PhysicalUseClaimKey,
    claim_revision: u64,
    claim_store_receipt_sha256: Sha256Digest,
    effect_completed: bool,
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
        issuance_verifier_receipt_sha256: Sha256Digest,
        final_verifier_receipt_sha256: Sha256Digest,
        token_sha256: Sha256Digest,
        claim_receipt: PhysicalUseClaimReceipt,
    ) -> Result<Self, VerifiedUseError> {
        let effect_completed = false;
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
            &issuance_verifier_receipt_sha256,
            &final_verifier_receipt_sha256,
            &token_sha256,
            &claim_receipt.claim_key,
            claim_receipt.claim_revision,
            &claim_receipt.store_receipt_sha256,
            effect_completed,
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
            issuance_verifier_receipt_sha256,
            final_verifier_receipt_sha256,
            token_sha256,
            claim_key: claim_receipt.claim_key,
            claim_revision: claim_receipt.claim_revision,
            claim_store_receipt_sha256: claim_receipt.store_receipt_sha256,
            effect_completed,
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

    pub const fn token_sha256(&self) -> &Sha256Digest {
        &self.token_sha256
    }

    pub const fn claim_key(&self) -> &PhysicalUseClaimKey {
        &self.claim_key
    }

    pub const fn claim_revision(&self) -> u64 {
        self.claim_revision
    }

    pub const fn claim_store_receipt_sha256(&self) -> &Sha256Digest {
        &self.claim_store_receipt_sha256
    }

    pub const fn effect_completed(&self) -> bool {
        self.effect_completed
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
            || self.claim_revision == 0
            || self.effect_completed
        {
            return Err(VerifiedUseError::WitnessIntegrityDrift);
        }

        let expected_claim_key = PhysicalUseClaimKey::for_operation(
            self.kind,
            &self.operation_id,
            &self.final_payload_sha256,
        );
        if expected_claim_key != self.claim_key {
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
            &self.issuance_verifier_receipt_sha256,
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
            &self.issuance_verifier_receipt_sha256,
            &self.final_verifier_receipt_sha256,
            &self.token_sha256,
            &self.claim_key,
            self.claim_revision,
            &self.claim_store_receipt_sha256,
            self.effect_completed,
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
    TrustedClockRejected(String),
    TrustedClockReturnedZero,
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
    FinalPhysicalVerificationRejected(String),
    RevocationRevisionDrift {
        expected: RevocationRevision,
        current: RevocationRevision,
    },
    FinalRevocationRevisionDrift {
        verified: RevocationRevision,
        current: RevocationRevision,
    },
    VerifierValidityExpired {
        valid_until: u64,
    },
    FinalVerifierValidityExpired {
        valid_until: u64,
    },
    FinalCapabilityKindDrift {
        verified: PhysicalCapabilityKind,
        observed: PhysicalCapabilityKind,
    },
    FinalOperationDrift,
    FinalPayloadDrift,
    FinalRuntimeContextDrift,
    ClockRollback {
        verified_at: u64,
        observed_at: u64,
    },
    TokenExpired {
        expires_at: u64,
        crossed_at: u64,
    },
    ClaimStore(PhysicalUseClaimStoreError),
    ClaimReceiptIntegrityDrift,
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
                "verified-use window requires a non-zero expiry later than trusted verification time",
            ),
            Self::TrustedClockRejected(reason) => {
                write!(formatter, "trusted physical clock rejected read: {reason}")
            }
            Self::TrustedClockReturnedZero => {
                formatter.write_str("trusted physical clock returned zero")
            }
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
                write!(formatter, "physical-use verifier rejected issuance: {reason}")
            }
            Self::FinalPhysicalVerificationRejected(reason) => {
                write!(formatter, "physical-use verifier rejected final boundary: {reason}")
            }
            Self::RevocationRevisionDrift { expected, current } => write!(
                formatter,
                "revocation revision drifted during issuance from {} to {}",
                expected.get(),
                current.get()
            ),
            Self::FinalRevocationRevisionDrift { verified, current } => write!(
                formatter,
                "revocation revision drifted before boundary crossing from {} to {}",
                verified.get(),
                current.get()
            ),
            Self::VerifierValidityExpired { valid_until } => write!(
                formatter,
                "physical-use verifier validity does not extend beyond issuance time: {valid_until}"
            ),
            Self::FinalVerifierValidityExpired { valid_until } => write!(
                formatter,
                "physical-use verifier validity does not extend beyond boundary time: {valid_until}"
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
            Self::FinalRuntimeContextDrift => {
                formatter.write_str("runtime authority context drifted before boundary crossing")
            }
            Self::ClockRollback {
                verified_at,
                observed_at,
            } => write!(
                formatter,
                "trusted boundary time {observed_at} precedes verification time {verified_at}"
            ),
            Self::TokenExpired {
                expires_at,
                crossed_at,
            } => write!(
                formatter,
                "verified-use token expired at {expires_at} before boundary time {crossed_at}"
            ),
            Self::ClaimStore(error) => write!(formatter, "durable physical-use claim failed: {error}"),
            Self::ClaimReceiptIntegrityDrift => {
                formatter.write_str("physical-use claim receipt integrity drifted")
            }
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
            Self::ClaimStore(error) => Some(error),
            _ => None,
        }
    }
}

impl From<AuthorityError> for VerifiedUseError {
    fn from(error: AuthorityError) -> Self {
        Self::Authority(error)
    }
}

fn trusted_now<T>(clock: &T) -> Result<u64, VerifiedUseError>
where
    T: TrustedPhysicalClock + ?Sized,
{
    let now = clock
        .now_unix_seconds()
        .map_err(VerifiedUseError::TrustedClockRejected)?;
    if now == 0 {
        return Err(VerifiedUseError::TrustedClockReturnedZero);
    }
    Ok(now)
}

fn validate_current_verification(
    expected_revision: RevocationRevision,
    observed_at_unix_seconds: u64,
    verification: &PhysicalUseVerification,
    final_boundary: bool,
) -> Result<(), VerifiedUseError> {
    if verification.current_revocation_revision != expected_revision {
        if final_boundary {
            return Err(VerifiedUseError::FinalRevocationRevisionDrift {
                verified: expected_revision,
                current: verification.current_revocation_revision,
            });
        }
        return Err(VerifiedUseError::RevocationRevisionDrift {
            expected: expected_revision,
            current: verification.current_revocation_revision,
        });
    }
    if verification.valid_until_unix_seconds <= observed_at_unix_seconds {
        if final_boundary {
            return Err(VerifiedUseError::FinalVerifierValidityExpired {
                valid_until: verification.valid_until_unix_seconds,
            });
        }
        return Err(VerifiedUseError::VerifierValidityExpired {
            valid_until: verification.valid_until_unix_seconds,
        });
    }
    Ok(())
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
    frame(&mut bytes, b"hepta:verified-use-token:v2");
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
    issuance_verifier_receipt_sha256: &Sha256Digest,
    final_verifier_receipt_sha256: &Sha256Digest,
    token_sha256: &Sha256Digest,
    claim_key: &PhysicalUseClaimKey,
    claim_revision: u64,
    claim_store_receipt_sha256: &Sha256Digest,
    effect_completed: bool,
) -> Sha256Digest {
    let mut bytes = Vec::new();
    frame(&mut bytes, b"hepta:verified-use-witness:v2");
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
    frame(
        &mut bytes,
        issuance_verifier_receipt_sha256.as_str().as_bytes(),
    );
    frame(
        &mut bytes,
        final_verifier_receipt_sha256.as_str().as_bytes(),
    );
    frame(&mut bytes, token_sha256.as_str().as_bytes());
    frame(
        &mut bytes,
        claim_key.operation_scope_sha256.as_str().as_bytes(),
    );
    frame(&mut bytes, claim_key.claim_sha256.as_str().as_bytes());
    frame(&mut bytes, &claim_revision.to_be_bytes());
    frame(
        &mut bytes,
        claim_store_receipt_sha256.as_str().as_bytes(),
    );
    frame(&mut bytes, &[u8::from(effect_completed)]);
    Sha256Digest::for_bytes(&bytes)
}

fn frame(target: &mut Vec<u8>, part: &[u8]) {
    target.extend_from_slice(&(part.len() as u64).to_be_bytes());
    target.extend_from_slice(part);
}

#[cfg(test)]
#[path = "verified_use_tests.rs"]
mod tests;
