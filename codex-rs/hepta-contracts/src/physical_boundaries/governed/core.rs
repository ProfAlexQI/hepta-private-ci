use std::fmt;
use std::future::Future;
use std::pin::Pin;

use serde::Serialize;

use super::GovernedBoundaryIntent;
use crate::AuthorityCapability;
use crate::AuthorityError;
use crate::Authorized;
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

const MAX_IDENTITY_BYTES: usize = 2_048;
const MAX_REASON_CODE_BYTES: usize = 256;
const MAX_FINAL_PAYLOAD_BYTES: u64 = 64 * 1024 * 1024;

/// Request visible to a governed physical adapter. Construction is private to
/// the checked core, so an adapter cannot be called without the claim/witness
/// sequence.
pub struct GovernedBoundaryDispatch<'a> {
    intent: &'a GovernedBoundaryIntent,
    final_payload: &'a [u8],
    verified_use_witness_sha256: &'a Sha256Digest,
}

impl<'a> GovernedBoundaryDispatch<'a> {
    pub const fn intent(&self) -> &'a GovernedBoundaryIntent {
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

impl fmt::Debug for GovernedBoundaryDispatch<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GovernedBoundaryDispatch")
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

/// A checked adapter can report only a digest-bound external receipt, an
/// explicit no-crossing rejection, or uncertainty. Recording a receipt does
/// not let repository source self-issue operator acceptance or promotion.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub enum GovernedBoundaryOutcome {
    ExternalReceiptRecorded {
        external_receipt_sha256: Sha256Digest,
        receipt_bytes: u64,
    },
    RejectedNoCrossing {
        reason_code: String,
    },
    Indeterminate {
        reason_code: String,
    },
}

impl GovernedBoundaryOutcome {
    pub fn external_receipt_recorded(
        receipt: &[u8],
    ) -> Result<Self, GovernedBoundaryError> {
        let receipt_bytes = payload_len(receipt)?;
        Ok(Self::ExternalReceiptRecorded {
            external_receipt_sha256: Sha256Digest::for_bytes(receipt),
            receipt_bytes,
        })
    }

    pub fn rejected_no_crossing(
        reason_code: impl Into<String>,
    ) -> Result<Self, GovernedBoundaryError> {
        let reason_code = reason_code.into();
        validate_reason_code(&reason_code)?;
        Ok(Self::RejectedNoCrossing { reason_code })
    }

    pub fn indeterminate(reason_code: impl Into<String>) -> Result<Self, GovernedBoundaryError> {
        let reason_code = reason_code.into();
        validate_reason_code(&reason_code)?;
        Ok(Self::Indeterminate { reason_code })
    }

    pub fn validate(&self) -> Result<(), GovernedBoundaryError> {
        match self {
            Self::ExternalReceiptRecorded {
                external_receipt_sha256,
                receipt_bytes,
            } => {
                validate_digest("external receipt", external_receipt_sha256)?;
                validate_payload_size(*receipt_bytes)?;
            }
            Self::RejectedNoCrossing { reason_code } | Self::Indeterminate { reason_code } => {
                validate_reason_code(reason_code)?;
            }
        }
        Ok(())
    }
}

pub type GovernedBoundaryFuture<'a> =
    Pin<Box<dyn Future<Output = Result<GovernedBoundaryOutcome, String>> + Send + 'a>>;

pub trait GovernedBoundaryAdapter: Send {
    fn cross<'a>(
        &'a mut self,
        dispatch: GovernedBoundaryDispatch<'a>,
    ) -> GovernedBoundaryFuture<'a>;
}

pub(super) struct GovernedBoundaryCore<C, A, V>
where
    C: AuthorityCapability,
    A: GovernedBoundaryAdapter,
    V: PhysicalUseVerifier,
{
    adapter: A,
    capability: Authorized<C>,
    runtime_authority: RuntimeAuthorityContext,
    verifier: V,
}

impl<C, A, V> GovernedBoundaryCore<C, A, V>
where
    C: AuthorityCapability,
    A: GovernedBoundaryAdapter,
    V: PhysicalUseVerifier,
{
    pub(super) fn new(
        adapter: A,
        capability: Authorized<C>,
        verifier: V,
    ) -> Result<Self, GovernedBoundaryError> {
        let binding = capability
            .external_lease_binding()
            .ok_or(GovernedBoundaryError::ExternalAuthorityRequired)?;
        let runtime_authority = RuntimeAuthorityContext::from_external_binding(binding)?;
        Ok(Self {
            adapter,
            capability,
            runtime_authority,
            verifier,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) async fn cross_once<N, S, P>(
        &mut self,
        intent: &GovernedBoundaryIntent,
        final_payload: &[u8],
        expected_revocation_revision: RevocationRevision,
        window: PhysicalUseWindow,
        now_unix_seconds: &N,
        claim_once: &S,
        persist_witness: P,
    ) -> Result<(GovernedBoundaryOutcome, VerifiedUseWitness), GovernedBoundaryError>
    where
        N: Fn() -> Result<u64, String> + Sync + ?Sized,
        S: Fn(
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
        persist_witness(&witness).map_err(GovernedBoundaryError::WitnessPersistence)?;

        let dispatch = GovernedBoundaryDispatch {
            intent,
            final_payload,
            verified_use_witness_sha256: witness.witness_sha256(),
        };
        let outcome = match self.adapter.cross(dispatch).await {
            Ok(outcome) => outcome,
            Err(reason) => GovernedBoundaryOutcome::indeterminate(normalize_reason(&reason))?,
        };
        outcome.validate()?;
        Ok((outcome, witness))
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum GovernedBoundaryError {
    SchemaVersion,
    EmptyIdentity(&'static str),
    InvalidIdentity(&'static str),
    InvalidDigest(&'static str),
    InvalidPayloadSize,
    FinalPayloadDrift,
    ZeroRevision(&'static str),
    InvalidDeadline,
    CandidateIdentityInvalid,
    ExternalAuthorityRequired,
    CapabilityBindingDrift,
    WitnessPersistence(String),
    InvalidReasonCode,
    Authority(AuthorityError),
    VerifiedUse(VerifiedUseError),
}

impl fmt::Display for GovernedBoundaryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersion => formatter.write_str("governed boundary schema version drift"),
            Self::EmptyIdentity(field) => write!(formatter, "{field} must not be empty"),
            Self::InvalidIdentity(field) => write!(formatter, "{field} is not canonical"),
            Self::InvalidDigest(field) => write!(formatter, "{field} SHA-256 is invalid"),
            Self::InvalidPayloadSize => {
                formatter.write_str("governed boundary payload size is invalid")
            }
            Self::FinalPayloadDrift => {
                formatter.write_str("governed boundary final payload drifted from intent")
            }
            Self::ZeroRevision(field) => write!(formatter, "{field} must be non-zero"),
            Self::InvalidDeadline => formatter.write_str("operation deadline is invalid"),
            Self::CandidateIdentityInvalid => {
                formatter.write_str("candidate commit/tree identity is invalid")
            }
            Self::ExternalAuthorityRequired => {
                formatter.write_str("governed boundary requires externally verified authority")
            }
            Self::CapabilityBindingDrift => {
                formatter.write_str("governed capability drifted from runtime authority")
            }
            Self::WitnessPersistence(reason) => {
                write!(
                    formatter,
                    "governed verified-use witness persistence failed: {reason}"
                )
            }
            Self::InvalidReasonCode => {
                formatter.write_str("governed boundary reason code is invalid")
            }
            Self::Authority(error) => write!(formatter, "governed authority error: {error}"),
            Self::VerifiedUse(error) => write!(formatter, "governed verified-use error: {error}"),
        }
    }
}

impl std::error::Error for GovernedBoundaryError {}

impl From<AuthorityError> for GovernedBoundaryError {
    fn from(error: AuthorityError) -> Self {
        Self::Authority(error)
    }
}

impl From<VerifiedUseError> for GovernedBoundaryError {
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

struct ClosureClaimStore<'a, S>(&'a S)
where
    S: Fn(
            PhysicalCapabilityKind,
            &Sha256Digest,
            &Sha256Digest,
            &Sha256Digest,
            &Sha256Digest,
            u64,
        ) -> Result<(u64, Sha256Digest), String>
        + Sync
        + ?Sized;

impl<S> PhysicalUseClaimStore for ClosureClaimStore<'_, S>
where
    S: Fn(
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

fn validate_capability_binding<C>(
    capability: &Authorized<C>,
    runtime_authority: &RuntimeAuthorityContext,
) -> Result<(), GovernedBoundaryError>
where
    C: AuthorityCapability,
{
    let binding = capability
        .external_lease_binding()
        .ok_or(GovernedBoundaryError::ExternalAuthorityRequired)?;
    if capability.subject_agent_id() != runtime_authority.subject_agent_id()
        || capability.generation() != runtime_authority.generation()
        || binding.authority_epoch() != runtime_authority.authority_epoch()
        || binding.owner_epoch() != runtime_authority.owner_epoch()
        || binding.fencing_token_sha256() != runtime_authority.fencing_token_sha256()
        || binding.grant_sha256() != runtime_authority.authority_grant_sha256()
    {
        return Err(GovernedBoundaryError::CapabilityBindingDrift);
    }
    Ok(())
}

fn claim_request_digest(request: &PhysicalUseClaimRequest<'_>) -> Sha256Digest {
    let mut bytes = Vec::new();
    frame(&mut bytes, b"hepta:b3-governed-claim-request:v1");
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

pub(super) fn validate_final_payload(
    expected_sha256: &Sha256Digest,
    expected_bytes: u64,
    final_payload: &[u8],
) -> Result<(), GovernedBoundaryError> {
    let observed_bytes = payload_len(final_payload)?;
    if observed_bytes != expected_bytes || Sha256Digest::for_bytes(final_payload) != *expected_sha256
    {
        return Err(GovernedBoundaryError::FinalPayloadDrift);
    }
    Ok(())
}

pub(super) fn payload_len(payload: &[u8]) -> Result<u64, GovernedBoundaryError> {
    let bytes = u64::try_from(payload.len()).map_err(|_| GovernedBoundaryError::InvalidPayloadSize)?;
    validate_payload_size(bytes)?;
    Ok(bytes)
}

pub(super) fn validate_payload_size(bytes: u64) -> Result<(), GovernedBoundaryError> {
    if bytes == 0 || bytes > MAX_FINAL_PAYLOAD_BYTES {
        return Err(GovernedBoundaryError::InvalidPayloadSize);
    }
    Ok(())
}

pub(super) fn validate_identity(
    field: &'static str,
    value: &str,
) -> Result<(), GovernedBoundaryError> {
    if value.is_empty() {
        return Err(GovernedBoundaryError::EmptyIdentity(field));
    }
    if value.len() > MAX_IDENTITY_BYTES
        || !value.is_ascii()
        || value.trim() != value
        || value.chars().any(char::is_control)
    {
        return Err(GovernedBoundaryError::InvalidIdentity(field));
    }
    Ok(())
}

pub(super) fn validate_digest(
    field: &'static str,
    digest: &Sha256Digest,
) -> Result<(), GovernedBoundaryError> {
    Sha256Digest::parse(digest.as_str())
        .map(|_| ())
        .map_err(|_| GovernedBoundaryError::InvalidDigest(field))
}

pub(super) fn require_nonzero(
    field: &'static str,
    value: u64,
) -> Result<(), GovernedBoundaryError> {
    if value == 0 {
        return Err(GovernedBoundaryError::ZeroRevision(field));
    }
    Ok(())
}

fn validate_reason_code(reason_code: &str) -> Result<(), GovernedBoundaryError> {
    if reason_code.is_empty()
        || reason_code.len() > MAX_REASON_CODE_BYTES
        || !reason_code.is_ascii()
        || reason_code.trim() != reason_code
        || reason_code
            .bytes()
            .any(|byte| !(byte.is_ascii_alphanumeric() || b"._:-".contains(&byte)))
    {
        return Err(GovernedBoundaryError::InvalidReasonCode);
    }
    Ok(())
}

fn normalize_reason(reason: &str) -> String {
    let mut normalized = String::with_capacity(reason.len().min(MAX_REASON_CODE_BYTES));
    for byte in reason.bytes().take(MAX_REASON_CODE_BYTES) {
        if byte.is_ascii_alphanumeric() || b"._:-".contains(&byte) {
            normalized.push(char::from(byte));
        } else {
            normalized.push('_');
        }
    }
    if normalized.is_empty() {
        "governed_boundary_unknown".to_string()
    } else {
        normalized
    }
}

pub(super) fn frame_digest(target: &mut Vec<u8>, digest: &Sha256Digest) {
    frame(target, digest.as_str().as_bytes());
}

pub(super) fn frame(target: &mut Vec<u8>, value: &[u8]) {
    target.extend_from_slice(&(value.len() as u64).to_be_bytes());
    target.extend_from_slice(value);
}
