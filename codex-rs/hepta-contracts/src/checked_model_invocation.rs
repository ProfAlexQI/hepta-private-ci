//! Final-payload, revocation-checked model invocation boundary.
//!
//! A broad [`Authorized<ModelInvocationCapability>`] is necessary but is not
//! sufficient to submit a model request. The final route and exact wire bytes
//! are bound into a physical payload digest, current authority is reverified,
//! a durable single-use claim is committed, and the resulting witness is
//! persisted before the adapter is called.
//!
//! The adapter remains private to the checked coordinator. A transport error
//! after the call boundary is [`ModelInvocationOutcome::Indeterminate`]; this
//! module exposes no blind-retry API and never records raw prompt bytes.

use std::fmt;
use std::future::Future;
use std::pin::Pin;

use serde::Serialize;

use crate::AuthorityError;
use crate::Authorized;
use crate::ModelInvocationCapability;
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

pub const MODEL_INVOCATION_BOUNDARY_SCHEMA_VERSION: u32 = 1;
pub const B1B_MODEL_BOUNDARY_RUNTIME_REGISTERED: bool = false;
pub const B1B_MODEL_BOUNDARY_PRODUCTION_CALLER: bool = false;
pub const B1B_MODEL_BOUNDARY_PRODUCTION_WRITER: bool = false;
pub const B1B_MODEL_BOUNDARY_MODEL_INVOCATION: bool = false;
pub const B1B_MODEL_BOUNDARY_PROVIDER_DISPATCH: bool = false;
pub const B1B_MODEL_BOUNDARY_EXTERNAL_EFFECT: bool = false;
pub const B1B_MODEL_BOUNDARY_OPERATOR_ACCEPTANCE: bool = false;
pub const B1B_MODEL_BOUNDARY_PROMOTION: bool = false;
pub const B1B_MODEL_BOUNDARY_RELEASE: bool = false;

const MAX_IDENTITY_BYTES: usize = 512;
const MAX_CONTENT_TYPE_BYTES: usize = 128;
const MAX_WIRE_PAYLOAD_BYTES: u64 = 32 * 1024 * 1024;
const MAX_REASON_CODE_BYTES: usize = 256;

/// Secret-free identity of the exact physical model route.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ModelInvocationRoute {
    provider_id: String,
    model_id: String,
    endpoint_identity_sha256: Sha256Digest,
    routing_policy_sha256: Sha256Digest,
}

impl ModelInvocationRoute {
    pub fn new(
        provider_id: impl Into<String>,
        model_id: impl Into<String>,
        endpoint_identity_sha256: Sha256Digest,
        routing_policy_sha256: Sha256Digest,
    ) -> Result<Self, ModelInvocationError> {
        let route = Self {
            provider_id: provider_id.into(),
            model_id: model_id.into(),
            endpoint_identity_sha256,
            routing_policy_sha256,
        };
        route.validate()?;
        Ok(route)
    }

    pub fn provider_id(&self) -> &str {
        &self.provider_id
    }

    pub fn model_id(&self) -> &str {
        &self.model_id
    }

    pub const fn endpoint_identity_sha256(&self) -> &Sha256Digest {
        &self.endpoint_identity_sha256
    }

    pub const fn routing_policy_sha256(&self) -> &Sha256Digest {
        &self.routing_policy_sha256
    }

    fn validate(&self) -> Result<(), ModelInvocationError> {
        validate_identity("provider id", &self.provider_id)?;
        validate_identity("model id", &self.model_id)?;
        validate_digest("endpoint identity", &self.endpoint_identity_sha256)?;
        validate_digest("routing policy", &self.routing_policy_sha256)?;
        Ok(())
    }
}

/// Durable caller-owned intent for one exact model request.
///
/// The intent stores only identities, byte counts and digests. Raw prompts,
/// tool arguments, images and credentials are deliberately absent.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ModelInvocationIntent {
    schema_version: u32,
    operation_id: OperationId,
    route: ModelInvocationRoute,
    wire_payload_sha256: Sha256Digest,
    wire_payload_bytes: u64,
    request_content_type: String,
    response_contract_sha256: Sha256Digest,
    tool_contract_sha256: Option<Sha256Digest>,
    streaming: bool,
}

impl ModelInvocationIntent {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        operation_id: OperationId,
        route: ModelInvocationRoute,
        wire_payload: &[u8],
        request_content_type: impl Into<String>,
        response_contract_sha256: Sha256Digest,
        tool_contract_sha256: Option<Sha256Digest>,
        streaming: bool,
    ) -> Result<Self, ModelInvocationError> {
        let wire_payload_bytes = u64::try_from(wire_payload.len())
            .map_err(|_| ModelInvocationError::WirePayloadSize)?;
        let intent = Self {
            schema_version: MODEL_INVOCATION_BOUNDARY_SCHEMA_VERSION,
            operation_id,
            route,
            wire_payload_sha256: Sha256Digest::for_bytes(wire_payload),
            wire_payload_bytes,
            request_content_type: request_content_type.into(),
            response_contract_sha256,
            tool_contract_sha256,
            streaming,
        };
        intent.validate()?;
        Ok(intent)
    }

    pub const fn schema_version(&self) -> u32 {
        self.schema_version
    }

    pub const fn operation_id(&self) -> &OperationId {
        &self.operation_id
    }

    pub const fn route(&self) -> &ModelInvocationRoute {
        &self.route
    }

    pub const fn wire_payload_sha256(&self) -> &Sha256Digest {
        &self.wire_payload_sha256
    }

    pub const fn wire_payload_bytes(&self) -> u64 {
        self.wire_payload_bytes
    }

    pub fn request_content_type(&self) -> &str {
        &self.request_content_type
    }

    pub const fn response_contract_sha256(&self) -> &Sha256Digest {
        &self.response_contract_sha256
    }

    pub const fn tool_contract_sha256(&self) -> Option<&Sha256Digest> {
        self.tool_contract_sha256.as_ref()
    }

    pub const fn streaming(&self) -> bool {
        self.streaming
    }

    pub fn validate_wire_payload(&self, wire_payload: &[u8]) -> Result<(), ModelInvocationError> {
        self.validate()?;
        let observed_bytes = u64::try_from(wire_payload.len())
            .map_err(|_| ModelInvocationError::WirePayloadSize)?;
        if observed_bytes != self.wire_payload_bytes
            || Sha256Digest::for_bytes(wire_payload) != self.wire_payload_sha256
        {
            return Err(ModelInvocationError::WirePayloadDrift);
        }
        Ok(())
    }

    /// Digest presented to the verified-use kernel. It binds the final route,
    /// exact wire payload, response/tool contracts and streaming mode.
    pub fn physical_payload_sha256(&self) -> Result<Sha256Digest, ModelInvocationError> {
        self.validate()?;
        let mut bytes = Vec::new();
        frame(&mut bytes, b"hepta:model-invocation-physical-payload:v1");
        frame(&mut bytes, &self.schema_version.to_be_bytes());
        frame(&mut bytes, self.operation_id.as_str().as_bytes());
        frame(&mut bytes, self.route.provider_id.as_bytes());
        frame(&mut bytes, self.route.model_id.as_bytes());
        frame(
            &mut bytes,
            self.route.endpoint_identity_sha256.as_str().as_bytes(),
        );
        frame(
            &mut bytes,
            self.route.routing_policy_sha256.as_str().as_bytes(),
        );
        frame(&mut bytes, self.wire_payload_sha256.as_str().as_bytes());
        frame(&mut bytes, &self.wire_payload_bytes.to_be_bytes());
        frame(&mut bytes, self.request_content_type.as_bytes());
        frame(
            &mut bytes,
            self.response_contract_sha256.as_str().as_bytes(),
        );
        match &self.tool_contract_sha256 {
            Some(digest) => {
                frame(&mut bytes, b"tool-contract-present");
                frame(&mut bytes, digest.as_str().as_bytes());
            }
            None => frame(&mut bytes, b"tool-contract-absent"),
        }
        frame(&mut bytes, &[u8::from(self.streaming)]);
        Ok(Sha256Digest::for_bytes(&bytes))
    }

    fn validate(&self) -> Result<(), ModelInvocationError> {
        if self.schema_version != MODEL_INVOCATION_BOUNDARY_SCHEMA_VERSION {
            return Err(ModelInvocationError::SchemaVersion);
        }
        self.route.validate()?;
        validate_digest("wire payload", &self.wire_payload_sha256)?;
        validate_digest("response contract", &self.response_contract_sha256)?;
        if let Some(digest) = &self.tool_contract_sha256 {
            validate_digest("tool contract", digest)?;
        }
        if self.wire_payload_bytes == 0 || self.wire_payload_bytes > MAX_WIRE_PAYLOAD_BYTES {
            return Err(ModelInvocationError::WirePayloadSize);
        }
        if self.request_content_type.is_empty()
            || self.request_content_type.len() > MAX_CONTENT_TYPE_BYTES
            || !self.request_content_type.is_ascii()
            || self.request_content_type.chars().any(char::is_whitespace)
        {
            return Err(ModelInvocationError::InvalidContentType);
        }
        Ok(())
    }
}

/// Request visible to a physical model adapter. Its constructor is private, so
/// an adapter implementation can only receive it through the checked boundary.
pub struct ModelInvocationDispatch<'a> {
    intent: &'a ModelInvocationIntent,
    wire_payload: &'a [u8],
    verified_use_witness_sha256: &'a Sha256Digest,
}

impl<'a> ModelInvocationDispatch<'a> {
    pub const fn intent(&self) -> &'a ModelInvocationIntent {
        self.intent
    }

    pub const fn wire_payload(&self) -> &'a [u8] {
        self.wire_payload
    }

    pub const fn verified_use_witness_sha256(&self) -> &'a Sha256Digest {
        self.verified_use_witness_sha256
    }
}

impl fmt::Debug for ModelInvocationDispatch<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ModelInvocationDispatch")
            .field("operation_id", &self.intent.operation_id.as_str())
            .field("provider_id", &self.intent.route.provider_id)
            .field("model_id", &self.intent.route.model_id)
            .field("wire_payload_bytes", &self.intent.wire_payload_bytes)
            .field("wire_payload_sha256", &self.intent.wire_payload_sha256)
            .field(
                "verified_use_witness_sha256",
                self.verified_use_witness_sha256,
            )
            .finish_non_exhaustive()
    }
}

/// Model-boundary result. Transport errors after adapter entry are explicitly
/// indeterminate and cannot be turned into a new dispatch by this API.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub enum ModelInvocationOutcome {
    Completed {
        response_sha256: Sha256Digest,
        response_bytes: u64,
    },
    RejectedNoDispatch {
        reason_code: String,
    },
    Indeterminate {
        reason_code: String,
    },
}

impl ModelInvocationOutcome {
    pub fn completed(response: &[u8]) -> Result<Self, ModelInvocationError> {
        let response_bytes = u64::try_from(response.len())
            .map_err(|_| ModelInvocationError::AdapterOutcome("response size overflow".into()))?;
        if response_bytes == 0 || response_bytes > MAX_WIRE_PAYLOAD_BYTES {
            return Err(ModelInvocationError::AdapterOutcome(
                "response size is outside the bounded contract".into(),
            ));
        }
        Ok(Self::Completed {
            response_sha256: Sha256Digest::for_bytes(response),
            response_bytes,
        })
    }

    pub fn rejected_no_dispatch(
        reason_code: impl Into<String>,
    ) -> Result<Self, ModelInvocationError> {
        let reason_code = reason_code.into();
        validate_reason_code(&reason_code)?;
        Ok(Self::RejectedNoDispatch { reason_code })
    }

    pub fn indeterminate(reason_code: impl Into<String>) -> Result<Self, ModelInvocationError> {
        let reason_code = reason_code.into();
        validate_reason_code(&reason_code)?;
        Ok(Self::Indeterminate { reason_code })
    }

    pub fn validate(&self) -> Result<(), ModelInvocationError> {
        match self {
            Self::Completed {
                response_sha256,
                response_bytes,
            } => {
                validate_digest("model response", response_sha256)?;
                if *response_bytes == 0 || *response_bytes > MAX_WIRE_PAYLOAD_BYTES {
                    return Err(ModelInvocationError::AdapterOutcome(
                        "response size is outside the bounded contract".into(),
                    ));
                }
            }
            Self::RejectedNoDispatch { reason_code } | Self::Indeterminate { reason_code } => {
                validate_reason_code(reason_code)?;
            }
        }
        Ok(())
    }
}

pub type ModelInvocationFuture<'a> =
    Pin<Box<dyn Future<Output = Result<ModelInvocationOutcome, String>> + Send + 'a>>;

/// Physical model adapter. The raw adapter value is never exposed by the
/// checked coordinator, and its request cannot be constructed externally.
pub trait ModelInvocationAdapter: Send {
    fn invoke<'a>(&'a mut self, dispatch: ModelInvocationDispatch<'a>) -> ModelInvocationFuture<'a>;
}

/// Checked model invocation composition. There is deliberately no adapter
/// accessor, restoration constructor or `into_parts` escape.
pub struct CheckedModelInvocation<A, V>
where
    A: ModelInvocationAdapter,
    V: PhysicalUseVerifier,
{
    adapter: A,
    capability: Authorized<ModelInvocationCapability>,
    runtime_authority: RuntimeAuthorityContext,
    verifier: V,
}

impl<A, V> CheckedModelInvocation<A, V>
where
    A: ModelInvocationAdapter,
    V: PhysicalUseVerifier,
{
    pub fn new(
        adapter: A,
        capability: Authorized<ModelInvocationCapability>,
        verifier: V,
    ) -> Result<Self, ModelInvocationError> {
        let binding = capability
            .external_lease_binding()
            .ok_or(ModelInvocationError::ExternalAuthorityRequired)?;
        let runtime_authority = RuntimeAuthorityContext::from_external_binding(binding)?;
        Ok(Self {
            adapter,
            capability,
            runtime_authority,
            verifier,
        })
    }

    /// Performs exactly one model invocation for one final wire payload.
    ///
    /// Ordering is fail-closed:
    ///
    /// 1. validate the final route and wire bytes;
    /// 2. issue and consume a model-specific verified-use token;
    /// 3. durably persist the witness;
    /// 4. invoke the private adapter once.
    ///
    /// After step 2 the operation is claimed. Witness persistence failure or
    /// adapter uncertainty never exposes an automatic retry path.
    #[allow(clippy::too_many_arguments)]
    pub async fn invoke_once<N, C, P>(
        &mut self,
        intent: &ModelInvocationIntent,
        wire_payload: &[u8],
        expected_revocation_revision: RevocationRevision,
        window: PhysicalUseWindow,
        now_unix_seconds: &N,
        claim_once: &C,
        persist_witness: P,
    ) -> Result<(ModelInvocationOutcome, VerifiedUseWitness), ModelInvocationError>
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
        intent.validate_wire_payload(wire_payload)?;
        validate_capability_binding(&self.capability, &self.runtime_authority)?;

        let physical_payload_sha256 = intent.physical_payload_sha256()?;
        let clock = ClosureClock(now_unix_seconds);
        let claim_store = ClosureClaimStore(claim_once);
        let token = verify_physical_capability_use(
            &self.capability,
            PhysicalCapabilityKind::ModelInvocation,
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
                PhysicalCapabilityKind::ModelInvocation,
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
        persist_witness(&witness).map_err(ModelInvocationError::WitnessPersistence)?;

        let dispatch = ModelInvocationDispatch {
            intent,
            wire_payload,
            verified_use_witness_sha256: witness.witness_sha256(),
        };
        let outcome = match self.adapter.invoke(dispatch).await {
            Ok(outcome) => outcome,
            Err(reason) => {
                ModelInvocationOutcome::indeterminate(normalize_transport_reason(&reason))?
            }
        };
        outcome.validate()?;
        Ok((outcome, witness))
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum ModelInvocationError {
    SchemaVersion,
    EmptyIdentity(&'static str),
    InvalidIdentity(&'static str),
    InvalidDigest(&'static str),
    InvalidContentType,
    WirePayloadSize,
    WirePayloadDrift,
    ExternalAuthorityRequired,
    CapabilityBindingDrift,
    WitnessPersistence(String),
    AdapterOutcome(String),
    InvalidReasonCode,
    Authority(AuthorityError),
    VerifiedUse(VerifiedUseError),
}

impl fmt::Display for ModelInvocationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersion => formatter.write_str("model invocation schema version drift"),
            Self::EmptyIdentity(field) => write!(formatter, "{field} must not be empty"),
            Self::InvalidIdentity(field) => write!(formatter, "{field} is not canonical"),
            Self::InvalidDigest(field) => write!(formatter, "{field} SHA-256 is invalid"),
            Self::InvalidContentType => formatter.write_str("request content type is invalid"),
            Self::WirePayloadSize => formatter.write_str("model wire payload size is invalid"),
            Self::WirePayloadDrift => formatter.write_str("model wire payload drifted from intent"),
            Self::ExternalAuthorityRequired => {
                formatter.write_str("model invocation requires externally verified authority")
            }
            Self::CapabilityBindingDrift => {
                formatter.write_str("model capability drifted from runtime authority")
            }
            Self::WitnessPersistence(reason) => {
                write!(
                    formatter,
                    "model verified-use witness persistence failed: {reason}"
                )
            }
            Self::AdapterOutcome(reason) => {
                write!(formatter, "invalid model adapter outcome: {reason}")
            }
            Self::InvalidReasonCode => formatter.write_str("model outcome reason code is invalid"),
            Self::Authority(error) => write!(formatter, "model authority error: {error}"),
            Self::VerifiedUse(error) => write!(formatter, "model verified-use error: {error}"),
        }
    }
}

impl std::error::Error for ModelInvocationError {}

impl From<AuthorityError> for ModelInvocationError {
    fn from(error: AuthorityError) -> Self {
        Self::Authority(error)
    }
}

impl From<VerifiedUseError> for ModelInvocationError {
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
    capability: &Authorized<ModelInvocationCapability>,
    runtime_authority: &RuntimeAuthorityContext,
) -> Result<(), ModelInvocationError> {
    let binding = capability
        .external_lease_binding()
        .ok_or(ModelInvocationError::ExternalAuthorityRequired)?;
    if capability.subject_agent_id() != runtime_authority.subject_agent_id()
        || capability.generation() != runtime_authority.generation()
        || binding.authority_epoch() != runtime_authority.authority_epoch()
        || binding.owner_epoch() != runtime_authority.owner_epoch()
        || binding.fencing_token_sha256() != runtime_authority.fencing_token_sha256()
        || binding.grant_sha256() != runtime_authority.authority_grant_sha256()
    {
        return Err(ModelInvocationError::CapabilityBindingDrift);
    }
    Ok(())
}

fn claim_request_digest(request: &PhysicalUseClaimRequest<'_>) -> Sha256Digest {
    let mut bytes = Vec::new();
    frame(&mut bytes, b"hepta:b1b-model-claim-request:v1");
    frame(&mut bytes, request.kind().as_str().as_bytes());
    frame(&mut bytes, request.operation_id().as_str().as_bytes());
    frame(
        &mut bytes,
        request.final_payload_sha256().as_str().as_bytes(),
    );
    frame(
        &mut bytes,
        request
            .runtime_authority_context_sha256()
            .as_str()
            .as_bytes(),
    );
    frame(
        &mut bytes,
        &request.revocation_revision().get().to_be_bytes(),
    );
    frame(&mut bytes, request.token_sha256().as_str().as_bytes());
    frame(
        &mut bytes,
        &request.claimed_at_unix_seconds().to_be_bytes(),
    );
    Sha256Digest::for_bytes(&bytes)
}

fn validate_identity(field: &'static str, value: &str) -> Result<(), ModelInvocationError> {
    if value.is_empty() {
        return Err(ModelInvocationError::EmptyIdentity(field));
    }
    if value.len() > MAX_IDENTITY_BYTES
        || !value.is_ascii()
        || value.trim() != value
        || value.chars().any(char::is_control)
    {
        return Err(ModelInvocationError::InvalidIdentity(field));
    }
    Ok(())
}

fn validate_digest(field: &'static str, digest: &Sha256Digest) -> Result<(), ModelInvocationError> {
    Sha256Digest::parse(digest.as_str())
        .map(|_| ())
        .map_err(|_| ModelInvocationError::InvalidDigest(field))
}

fn validate_reason_code(reason_code: &str) -> Result<(), ModelInvocationError> {
    if reason_code.is_empty()
        || reason_code.len() > MAX_REASON_CODE_BYTES
        || !reason_code.is_ascii()
        || reason_code.trim() != reason_code
        || reason_code
            .bytes()
            .any(|byte| !(byte.is_ascii_alphanumeric() || b"._:-".contains(&byte)))
    {
        return Err(ModelInvocationError::InvalidReasonCode);
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
        "model_transport_unknown".to_string()
    } else {
        normalized
    }
}

fn frame(target: &mut Vec<u8>, value: &[u8]) {
    target.extend_from_slice(&(value.len() as u64).to_be_bytes());
    target.extend_from_slice(value);
}
