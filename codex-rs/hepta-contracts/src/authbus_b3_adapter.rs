//! Qualification-only process-bound adapter for the AuthBus B3 contracts.
//!
//! This module is deliberately a local safety harness, not an OpenBao client,
//! durable writer, listener, or production caller. It keeps raw secret bytes
//! process-bound, classifies provider-call uncertainty conservatively, and
//! models explicit retry versus lookup-only reconciliation.

use std::collections::BTreeMap;
use std::fmt;

use zeroize::Zeroizing;

use crate::AuthBusContractError;
use crate::Sha256Digest;
use crate::authbus::b3::{
    OpaqueSecretRef, RefreshStatusByOperationKeyRequest, RefreshStatusByOperationKeyResponse,
    RefreshWithSecretRefRequest, RefreshWithSecretRefResponse, RotateSecretRefRequest,
    RotateSecretRefResponse, SecretProviderStatus, SecretRefCallbackFence, SecretRefEvent,
    SecretRefOperationRecord, SecretRefOutcome, SecretRefState,
};

/// This implementation is a local qualification seam only.
pub const AUTHBUS_B3_ADAPTER_QUALIFICATION_ONLY: bool = true;
pub const AUTHBUS_B3_ADAPTER_AUTHORITY: bool = false;
pub const AUTHBUS_B3_ADAPTER_EFFECT_AUTHORITY: bool = false;
pub const AUTHBUS_B3_ADAPTER_PRODUCTION_CALLER: bool = false;
pub const AUTHBUS_B3_ADAPTER_PRODUCTION_WRITER: bool = false;
pub const AUTHBUS_B3_ADAPTER_OPERATOR_ACCEPTANCE: bool = false;
pub const AUTHBUS_B3_ADAPTER_PROMOTION: bool = false;
pub const AUTHBUS_B3_ADAPTER_G5_ALLOWED: bool = false;
pub const AUTHBUS_B3_ADAPTER_EXECUTE_ALLOWED: bool = false;

const _: () = {
    assert!(AUTHBUS_B3_ADAPTER_QUALIFICATION_ONLY);
    assert!(!AUTHBUS_B3_ADAPTER_AUTHORITY);
    assert!(!AUTHBUS_B3_ADAPTER_EFFECT_AUTHORITY);
    assert!(!AUTHBUS_B3_ADAPTER_PRODUCTION_CALLER);
    assert!(!AUTHBUS_B3_ADAPTER_PRODUCTION_WRITER);
    assert!(!AUTHBUS_B3_ADAPTER_OPERATOR_ACCEPTANCE);
    assert!(!AUTHBUS_B3_ADAPTER_PROMOTION);
    assert!(!AUTHBUS_B3_ADAPTER_G5_ALLOWED);
    assert!(!AUTHBUS_B3_ADAPTER_EXECUTE_ALLOWED);
};

/// Errors returned before a provider call is reached.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SecretBackendError {
    NotFound,
    Unauthorized,
    Timeout,
    Unavailable,
    Sealed,
    InvalidReference,
}

impl fmt::Display for SecretBackendError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Self::NotFound => "secret backend did not find the reference",
            Self::Unauthorized => "secret backend denied the reference",
            Self::Timeout => "secret backend timed out",
            Self::Unavailable => "secret backend is unavailable",
            Self::Sealed => "secret backend is sealed or standby",
            Self::InvalidReference => "secret reference is invalid",
        };
        formatter.write_str(label)
    }
}

impl SecretBackendError {
    fn status(self) -> SecretProviderStatus {
        match self {
            Self::NotFound | Self::Unavailable => SecretProviderStatus::Unavailable,
            Self::Unauthorized => SecretProviderStatus::Unauthorized,
            Self::Timeout => SecretProviderStatus::Timeout,
            Self::Sealed => SecretProviderStatus::Sealed,
            Self::InvalidReference => SecretProviderStatus::SchemaInvalid,
        }
    }

    /// Backend failures happen before the provider boundary and are therefore
    /// safe to classify as retryable failures rather than unknown effects.
    fn outcome(self) -> SecretRefOutcome {
        SecretRefOutcome::TransientFailure
    }

    fn event(self) -> SecretRefEvent {
        SecretRefEvent::TransientFailure
    }
}

/// Errors observed after the provider adapter method has been entered.
///
/// Timeout, transport unavailability, malformed response schema, and an
/// explicitly unknown outcome are conservative post-dispatch uncertainty:
/// they require status lookup and never authorize a blind retry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProviderAdapterError {
    InvalidGrant,
    Unauthorized,
    Conflict,
    Timeout,
    Unavailable,
    Sealed,
    StaleFence,
    SchemaInvalid,
    Unknown,
}

impl fmt::Display for ProviderAdapterError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Self::InvalidGrant => "provider rejected the grant",
            Self::Unauthorized => "provider denied the request",
            Self::Conflict => "provider reported a conflict",
            Self::Timeout => "provider outcome is unknown after timeout",
            Self::Unavailable => "provider outcome is unknown after transport unavailability",
            Self::Sealed => "provider backend is sealed",
            Self::StaleFence => "provider rejected a stale fence",
            Self::SchemaInvalid => "provider response schema is invalid after the call",
            Self::Unknown => "provider outcome is unknown",
        };
        formatter.write_str(label)
    }
}

impl ProviderAdapterError {
    fn status(self) -> SecretProviderStatus {
        match self {
            Self::InvalidGrant => SecretProviderStatus::InvalidGrant,
            Self::Unauthorized => SecretProviderStatus::Unauthorized,
            Self::Conflict => SecretProviderStatus::Conflict,
            Self::Timeout | Self::Unavailable | Self::SchemaInvalid | Self::Unknown => {
                SecretProviderStatus::Unknown
            }
            Self::Sealed => SecretProviderStatus::Sealed,
            Self::StaleFence => SecretProviderStatus::StaleFence,
        }
    }

    fn outcome(self) -> SecretRefOutcome {
        match self {
            Self::InvalidGrant => SecretRefOutcome::Quarantined,
            Self::Timeout | Self::Unavailable | Self::SchemaInvalid | Self::Unknown => {
                SecretRefOutcome::Indeterminate
            }
            Self::Unauthorized | Self::Conflict | Self::Sealed | Self::StaleFence => {
                SecretRefOutcome::TransientFailure
            }
        }
    }

    fn event(self) -> SecretRefEvent {
        match self.outcome() {
            SecretRefOutcome::Succeeded => unreachable!("provider errors cannot be success"),
            SecretRefOutcome::Quarantined => SecretRefEvent::InvalidGrant,
            SecretRefOutcome::TransientFailure => SecretRefEvent::TransientFailure,
            SecretRefOutcome::Indeterminate => SecretRefEvent::ResponseUnknown,
        }
    }
}

/// Process-local secret material. The underlying bytes are not serializable or
/// printable and are zeroized when this value is dropped.
pub struct ProcessBoundSecret(Zeroizing<Vec<u8>>);

impl ProcessBoundSecret {
    pub fn from_bytes(bytes: impl AsRef<[u8]>) -> Self {
        Self(Zeroizing::new(bytes.as_ref().to_vec()))
    }

    pub fn with_bytes<T>(&self, operation: impl FnOnce(&[u8]) -> T) -> T {
        operation(self.0.as_slice())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl fmt::Debug for ProcessBoundSecret {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ProcessBoundSecret")
            .field("len", &self.0.len())
            .finish_non_exhaustive()
    }
}

/// Process-bound backend boundary.
pub trait SecretRefBackend: Send + Sync {
    fn resolve(
        &self,
        secret_ref: &OpaqueSecretRef,
    ) -> Result<ProcessBoundSecret, SecretBackendError>;
}

/// Provider boundary used by the local adapter.
pub trait SecretRefProvider: Send + Sync {
    fn refresh(
        &self,
        request: &RefreshWithSecretRefRequest,
        secret: &ProcessBoundSecret,
    ) -> Result<ProviderRefreshResult, ProviderAdapterError>;

    fn rotate(
        &self,
        request: &RotateSecretRefRequest,
        secret: &ProcessBoundSecret,
    ) -> Result<ProviderRotationResult, ProviderAdapterError>;

    fn status_by_operation_key(
        &self,
        request: &RefreshStatusByOperationKeyRequest,
    ) -> Result<ProviderStatusResult, ProviderAdapterError>;

    fn status_by_effect_key(
        &self,
        request: &RefreshStatusByOperationKeyRequest,
    ) -> Result<ProviderStatusResult, ProviderAdapterError> {
        self.status_by_operation_key(request)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderRefreshResult {
    pub response_id: String,
    pub provider_status: SecretProviderStatus,
    pub access_secret_ref: Option<OpaqueSecretRef>,
    pub refresh_secret_ref: Option<OpaqueSecretRef>,
    pub secret_revision: Option<u64>,
    pub response_digest: Sha256Digest,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderRotationResult {
    pub response_id: String,
    pub provider_status: SecretProviderStatus,
    pub new_refresh_secret_ref: Option<OpaqueSecretRef>,
    pub secret_revision: Option<u64>,
    pub response_digest: Sha256Digest,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderStatusResult {
    pub response_id: String,
    pub provider_status: SecretProviderStatus,
    pub secret_revision: u64,
    pub response_digest: Sha256Digest,
    pub status_revision: u64,
    pub observed_at: u64,
    pub provider_query_receipt_digest: Sha256Digest,
    pub new_access_secret_ref: Option<OpaqueSecretRef>,
    pub new_refresh_secret_ref: Option<OpaqueSecretRef>,
}

/// Deterministic in-memory backend used by qualification tests.
#[derive(Default)]
pub struct QualificationSecretBackend {
    entries: BTreeMap<String, (OpaqueSecretRef, Zeroizing<Vec<u8>>)>,
    forced_error: Option<SecretBackendError>,
}

impl QualificationSecretBackend {
    pub fn insert(
        &mut self,
        secret_ref: OpaqueSecretRef,
        bytes: impl AsRef<[u8]>,
    ) -> Result<(), SecretBackendError> {
        secret_ref
            .validate()
            .map_err(|_| SecretBackendError::InvalidReference)?;
        let bytes = bytes.as_ref();
        if Sha256Digest::for_bytes(bytes) != secret_ref.secret_digest {
            return Err(SecretBackendError::InvalidReference);
        }
        let key = secret_ref
            .digest()
            .map_err(|_| SecretBackendError::InvalidReference)?
            .as_str()
            .to_string();
        self.entries
            .insert(key, (secret_ref, Zeroizing::new(bytes.to_vec())));
        Ok(())
    }

    pub fn set_error(&mut self, error: Option<SecretBackendError>) {
        self.forced_error = error;
    }

    pub fn clear_error(&mut self) {
        self.forced_error = None;
    }
}

impl SecretRefBackend for QualificationSecretBackend {
    fn resolve(
        &self,
        secret_ref: &OpaqueSecretRef,
    ) -> Result<ProcessBoundSecret, SecretBackendError> {
        secret_ref
            .validate()
            .map_err(|_| SecretBackendError::InvalidReference)?;
        if let Some(error) = self.forced_error {
            return Err(error);
        }
        let key = secret_ref
            .digest()
            .map_err(|_| SecretBackendError::InvalidReference)?;
        let Some((stored_ref, bytes)) = self.entries.get(key.as_str()) else {
            return Err(SecretBackendError::NotFound);
        };
        if stored_ref != secret_ref
            || Sha256Digest::for_bytes(bytes.as_slice()) != secret_ref.secret_digest
        {
            return Err(SecretBackendError::InvalidReference);
        }
        Ok(ProcessBoundSecret::from_bytes(bytes.as_slice()))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum B3AdapterError {
    InvalidRequest(String),
    Backend(SecretBackendError),
    Provider(ProviderAdapterError),
    ProviderResponseInvalid(String),
    Conflict,
    SingleflightConflict,
    ReconcileRequired,
    RetryRequired,
    RetryNotAllowed,
    RetryBudgetExhausted,
    ManualEvidenceRequired,
    StatusRevisionConflict,
    OperationNotFound,
    AlreadyTerminal,
    InvalidState(String),
}

impl fmt::Display for B3AdapterError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRequest(message) => write!(formatter, "invalid B3 request: {message}"),
            Self::Backend(error) => write!(formatter, "secret backend error: {error}"),
            Self::Provider(error) => write!(formatter, "provider adapter error: {error}"),
            Self::ProviderResponseInvalid(message) => {
                write!(formatter, "invalid provider response: {message}")
            }
            Self::Conflict => formatter.write_str("B3 operation conflict"),
            Self::SingleflightConflict => {
                formatter.write_str("B3 token-family singleflight conflict")
            }
            Self::ReconcileRequired => {
                formatter.write_str("B3 operation requires lookup-only reconciliation")
            }
            Self::RetryRequired => {
                formatter.write_str("B3 operation requires explicit retry, not status lookup")
            }
            Self::RetryNotAllowed => formatter.write_str("B3 retry is not allowed in this state"),
            Self::RetryBudgetExhausted => {
                formatter.write_str("B3 retry budget is exhausted; manual evidence is required")
            }
            Self::ManualEvidenceRequired => {
                formatter.write_str("B3 operation requires explicit manual evidence")
            }
            Self::StatusRevisionConflict => {
                formatter.write_str("B3 status observation is stale or conflicting")
            }
            Self::OperationNotFound => formatter.write_str("B3 operation was not found"),
            Self::AlreadyTerminal => formatter.write_str("B3 operation is already terminal"),
            Self::InvalidState(message) => {
                write!(formatter, "invalid B3 operation state: {message}")
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum OperationKind {
    Refresh,
    Rotate,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum StoredRequest {
    Refresh(RefreshWithSecretRefRequest),
    Rotate(RotateSecretRefRequest),
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum StoredResponse {
    Refresh(RefreshWithSecretRefResponse),
    Rotate(RotateSecretRefResponse),
}

#[derive(Clone)]
struct OperationEntry {
    request_digest: Sha256Digest,
    claim_key: String,
    kind: OperationKind,
    request: StoredRequest,
    record: SecretRefOperationRecord,
    response: Option<StoredResponse>,
    last_status_revision: Option<u64>,
}

/// Local process-bound adapter. All state remains in memory and carries no
/// production or durability authority.
pub struct ProcessBoundSecretRefAdapter<B, P> {
    backend: B,
    provider: P,
    retry_budget: u32,
    operations: BTreeMap<String, OperationEntry>,
    claims: BTreeMap<String, String>,
}

trait AdapterRequest: Clone {
    fn kind(&self) -> OperationKind;
    fn operation_id(&self) -> &str;
    fn provider_id(&self) -> &str;
    fn profile_id(&self) -> &str;
    fn token_family_id(&self) -> &str;
    fn digest_for_adapter(&self) -> Result<Sha256Digest, AuthBusContractError>;
    fn operation_record(
        &self,
        retry_budget: u32,
    ) -> Result<SecretRefOperationRecord, AuthBusContractError>;
    fn stored_request(&self) -> StoredRequest;
}

impl AdapterRequest for RefreshWithSecretRefRequest {
    fn kind(&self) -> OperationKind {
        OperationKind::Refresh
    }

    fn operation_id(&self) -> &str {
        self.operation_id.as_str()
    }

    fn provider_id(&self) -> &str {
        self.provider_id.as_str()
    }

    fn profile_id(&self) -> &str {
        self.profile_id.as_str()
    }

    fn token_family_id(&self) -> &str {
        self.token_family_id.as_str()
    }

    fn digest_for_adapter(&self) -> Result<Sha256Digest, AuthBusContractError> {
        self.digest()
    }

    fn operation_record(
        &self,
        retry_budget: u32,
    ) -> Result<SecretRefOperationRecord, AuthBusContractError> {
        SecretRefOperationRecord::from_refresh_request(self, retry_budget)
    }

    fn stored_request(&self) -> StoredRequest {
        StoredRequest::Refresh(self.clone())
    }
}

impl AdapterRequest for RotateSecretRefRequest {
    fn kind(&self) -> OperationKind {
        OperationKind::Rotate
    }

    fn operation_id(&self) -> &str {
        self.operation_id.as_str()
    }

    fn provider_id(&self) -> &str {
        self.provider_id.as_str()
    }

    fn profile_id(&self) -> &str {
        self.profile_id.as_str()
    }

    fn token_family_id(&self) -> &str {
        self.token_family_id.as_str()
    }

    fn digest_for_adapter(&self) -> Result<Sha256Digest, AuthBusContractError> {
        self.digest()
    }

    fn operation_record(
        &self,
        retry_budget: u32,
    ) -> Result<SecretRefOperationRecord, AuthBusContractError> {
        let refresh = RefreshWithSecretRefRequest {
            schema_version: self.schema_version,
            operation_id: self.operation_id.clone(),
            refresh_operation_key: self.refresh_operation_key.clone(),
            command_id: self.command_id.clone(),
            run_id: self.run_id.clone(),
            profile_id: self.profile_id.clone(),
            provider_id: self.provider_id.clone(),
            token_family_id: self.token_family_id.clone(),
            secret_ref: self.secret_ref.clone(),
            expected_secret_revision: self.expected_secret_revision,
            idempotency_key: self.idempotency_key.clone(),
            payload_digest: self.payload_digest.clone(),
            policy_digest: self.policy_digest.clone(),
            scope_digest: self.scope_digest.clone(),
            authority_epoch: self.authority_epoch,
            owner_epoch: self.owner_epoch,
            generation: self.generation,
            fencing_token: self.fencing_token.clone(),
            logical_clock: self.logical_clock,
            causal_parent_event_id: self.causal_parent_event_id.clone(),
            deadline_at: self.deadline_at,
            purpose_digest: self.purpose_digest.clone(),
            audience: self.audience.clone(),
        };
        SecretRefOperationRecord::from_refresh_request(&refresh, retry_budget)
    }

    fn stored_request(&self) -> StoredRequest {
        StoredRequest::Rotate(self.clone())
    }
}

impl<B, P> ProcessBoundSecretRefAdapter<B, P>
where
    B: SecretRefBackend,
    P: SecretRefProvider,
{
    pub fn new(backend: B, provider: P) -> Self {
        Self::with_retry_budget(backend, provider, 1)
    }

    pub fn with_retry_budget(backend: B, provider: P, retry_budget: u32) -> Self {
        Self {
            backend,
            provider,
            retry_budget,
            operations: BTreeMap::new(),
            claims: BTreeMap::new(),
        }
    }

    pub fn backend(&self) -> &B {
        &self.backend
    }

    pub fn backend_mut(&mut self) -> &mut B {
        &mut self.backend
    }

    pub fn provider(&self) -> &P {
        &self.provider
    }

    pub fn provider_mut(&mut self) -> &mut P {
        &mut self.provider
    }

    pub fn operation_state(&self, operation_id: &str) -> Option<SecretRefState> {
        self.operations
            .get(operation_id)
            .map(|entry| entry.record.state)
    }

    pub fn operation_attempt(&self, operation_id: &str) -> Option<u32> {
        self.operations
            .get(operation_id)
            .map(|entry| entry.record.attempt)
    }

    pub fn operation_count(&self) -> usize {
        self.operations.len()
    }

    /// Initial refresh. Repeated calls replay the current recorded result and
    /// never turn a non-terminal observation into an implicit retry.
    pub fn refresh(
        &mut self,
        request: RefreshWithSecretRefRequest,
    ) -> Result<RefreshWithSecretRefResponse, B3AdapterError> {
        request
            .validate()
            .map_err(|error| B3AdapterError::InvalidRequest(error.to_string()))?;
        if let Some(replay) = self.begin_or_replay(&request)? {
            return match replay {
                StoredResponse::Refresh(response) => Ok(response),
                StoredResponse::Rotate(_) => Err(B3AdapterError::Conflict),
            };
        }
        self.call_refresh(&request)
    }

    /// Explicit retry after a verified transient failure or a completed status
    /// lookup has moved the operation to backoff.
    pub fn retry_refresh(
        &mut self,
        request: RefreshWithSecretRefRequest,
    ) -> Result<RefreshWithSecretRefResponse, B3AdapterError> {
        request
            .validate()
            .map_err(|error| B3AdapterError::InvalidRequest(error.to_string()))?;
        self.prepare_retry(&request)?;
        self.call_refresh(&request)
    }

    pub fn rotate(
        &mut self,
        request: RotateSecretRefRequest,
    ) -> Result<RotateSecretRefResponse, B3AdapterError> {
        request
            .validate()
            .map_err(|error| B3AdapterError::InvalidRequest(error.to_string()))?;
        if let Some(replay) = self.begin_or_replay(&request)? {
            return match replay {
                StoredResponse::Rotate(response) => Ok(response),
                StoredResponse::Refresh(_) => Err(B3AdapterError::Conflict),
            };
        }
        self.call_rotate(&request)
    }

    pub fn retry_rotate(
        &mut self,
        request: RotateSecretRefRequest,
    ) -> Result<RotateSecretRefResponse, B3AdapterError> {
        request
            .validate()
            .map_err(|error| B3AdapterError::InvalidRequest(error.to_string()))?;
        self.prepare_retry(&request)?;
        self.call_rotate(&request)
    }

    /// Lookup-only reconciliation. It is accepted only for indeterminate or
    /// already-reconciling operations. Transient failures use the explicit
    /// retry methods; manual holds require a separate evidence ceremony.
    pub fn status_by_operation_key(
        &mut self,
        request: RefreshStatusByOperationKeyRequest,
    ) -> Result<RefreshStatusByOperationKeyResponse, B3AdapterError> {
        request
            .validate()
            .map_err(|error| B3AdapterError::InvalidRequest(error.to_string()))?;

        let state = {
            let entry = self
                .operations
                .get(request.operation_id.as_str())
                .ok_or(B3AdapterError::OperationNotFound)?;
            Self::validate_status_request(entry, &request)?;
            entry.record.state
        };

        match state {
            SecretRefState::Succeeded | SecretRefState::Quarantined => {
                return Err(B3AdapterError::AlreadyTerminal);
            }
            SecretRefState::Indeterminate | SecretRefState::Reconciling => {}
            SecretRefState::TransientFailure | SecretRefState::Backoff => {
                return Err(B3AdapterError::RetryRequired);
            }
            SecretRefState::ManualRequired => {
                return Err(B3AdapterError::ManualEvidenceRequired);
            }
            SecretRefState::Idle | SecretRefState::Claimed | SecretRefState::InFlight => {
                return Err(B3AdapterError::ReconcileRequired);
            }
        }

        let provider_result = self
            .provider
            .status_by_effect_key(&request)
            .map_err(B3AdapterError::Provider)?;
        let response = self.build_status_response(&request, provider_result)?;
        self.validate_status_progress(request.operation_id.as_str(), &response)?;

        let reconciled_replay = if response.outcome.is_terminal() {
            Some(self.build_reconciled_replay(request.operation_id.as_str(), &response)?)
        } else {
            None
        };

        self.apply_status_transition(&request, response.outcome)?;

        let entry = self
            .operations
            .get_mut(request.operation_id.as_str())
            .ok_or(B3AdapterError::OperationNotFound)?;
        entry.last_status_revision = Some(response.status_revision);
        if let Some(replay) = reconciled_replay {
            entry.response = Some(replay);
        }
        Ok(response)
    }

    pub fn status_by_effect_key(
        &mut self,
        request: RefreshStatusByOperationKeyRequest,
    ) -> Result<RefreshStatusByOperationKeyResponse, B3AdapterError> {
        self.status_by_operation_key(request)
    }

    fn begin_or_replay<R>(&mut self, request: &R) -> Result<Option<StoredResponse>, B3AdapterError>
    where
        R: AdapterRequest,
    {
        let request_digest = request
            .digest_for_adapter()
            .map_err(|error| B3AdapterError::InvalidRequest(error.to_string()))?;
        if let Some(existing) = self.operations.get(request.operation_id()) {
            if existing.kind != request.kind() || existing.request_digest != request_digest {
                return Err(B3AdapterError::Conflict);
            }
            return existing
                .response
                .clone()
                .map(Some)
                .ok_or(B3AdapterError::ReconcileRequired);
        }

        let claim_key = claim_key(
            request.provider_id(),
            request.profile_id(),
            request.token_family_id(),
        );
        if let Some(existing_operation) = self.claims.get(&claim_key)
            && existing_operation != request.operation_id()
        {
            return Err(B3AdapterError::SingleflightConflict);
        }

        let mut record = request
            .operation_record(self.retry_budget)
            .map_err(|error| B3AdapterError::InvalidRequest(error.to_string()))?;
        record
            .transition(SecretRefEvent::Claim)
            .map_err(|error| B3AdapterError::InvalidState(error.to_string()))?;
        record
            .transition(SecretRefEvent::Dispatch)
            .map_err(|error| B3AdapterError::InvalidState(error.to_string()))?;

        self.operations.insert(
            request.operation_id().to_string(),
            OperationEntry {
                request_digest,
                claim_key: claim_key.clone(),
                kind: request.kind(),
                request: request.stored_request(),
                record,
                response: None,
                last_status_revision: None,
            },
        );
        self.claims
            .insert(claim_key, request.operation_id().to_string());
        Ok(None)
    }

    fn prepare_retry<R>(&mut self, request: &R) -> Result<(), B3AdapterError>
    where
        R: AdapterRequest,
    {
        let request_digest = request
            .digest_for_adapter()
            .map_err(|error| B3AdapterError::InvalidRequest(error.to_string()))?;
        let entry = self
            .operations
            .get_mut(request.operation_id())
            .ok_or(B3AdapterError::OperationNotFound)?;
        if entry.kind != request.kind() || entry.request_digest != request_digest {
            return Err(B3AdapterError::Conflict);
        }

        match entry.record.state {
            SecretRefState::Succeeded | SecretRefState::Quarantined => {
                return Err(B3AdapterError::AlreadyTerminal);
            }
            SecretRefState::Indeterminate | SecretRefState::Reconciling => {
                return Err(B3AdapterError::ReconcileRequired);
            }
            SecretRefState::ManualRequired => {
                return Err(B3AdapterError::RetryBudgetExhausted);
            }
            SecretRefState::TransientFailure | SecretRefState::Backoff => {}
            SecretRefState::Idle | SecretRefState::Claimed | SecretRefState::InFlight => {
                return Err(B3AdapterError::RetryNotAllowed);
            }
        }

        if !retry_available(&entry.record) {
            if entry.record.state == SecretRefState::TransientFailure {
                entry
                    .record
                    .transition(SecretRefEvent::RetryBudgetExhausted)
                    .map_err(|error| B3AdapterError::InvalidState(error.to_string()))?;
            }
            return Err(B3AdapterError::RetryBudgetExhausted);
        }

        if entry.record.state == SecretRefState::TransientFailure {
            entry
                .record
                .transition(SecretRefEvent::RetryScheduled)
                .map_err(|error| B3AdapterError::InvalidState(error.to_string()))?;
        }
        entry
            .record
            .transition(SecretRefEvent::ClaimAgain)
            .map_err(|error| B3AdapterError::InvalidState(error.to_string()))?;
        entry
            .record
            .transition(SecretRefEvent::Dispatch)
            .map_err(|error| B3AdapterError::InvalidState(error.to_string()))?;
        entry.response = None;
        Ok(())
    }

    fn call_refresh(
        &mut self,
        request: &RefreshWithSecretRefRequest,
    ) -> Result<RefreshWithSecretRefResponse, B3AdapterError> {
        let secret = match self.backend.resolve(&request.secret_ref) {
            Ok(secret) => secret,
            Err(error) => return self.finish_refresh_backend_error(request, error),
        };
        let result = self.provider.refresh(request, &secret);
        drop(secret);
        match result {
            Ok(result) => self.finish_refresh(request, result),
            Err(error) => self.finish_refresh_provider_error(request, error),
        }
    }

    fn call_rotate(
        &mut self,
        request: &RotateSecretRefRequest,
    ) -> Result<RotateSecretRefResponse, B3AdapterError> {
        let secret = match self.backend.resolve(&request.secret_ref) {
            Ok(secret) => secret,
            Err(error) => return self.finish_rotate_backend_error(request, error),
        };
        let result = self.provider.rotate(request, &secret);
        drop(secret);
        match result {
            Ok(result) => self.finish_rotate(request, result),
            Err(error) => self.finish_rotate_provider_error(request, error),
        }
    }

    fn finish_refresh(
        &mut self,
        request: &RefreshWithSecretRefRequest,
        result: ProviderRefreshResult,
    ) -> Result<RefreshWithSecretRefResponse, B3AdapterError> {
        let outcome = outcome_for_status(result.provider_status);
        let response = RefreshWithSecretRefResponse {
            schema_version: request.schema_version,
            response_id: result.response_id,
            operation_id: request.operation_id.clone(),
            provider_id: request.provider_id.clone(),
            profile_id: request.profile_id.clone(),
            token_family_id: request.token_family_id.clone(),
            outcome,
            access_secret_ref: (outcome == SecretRefOutcome::Succeeded)
                .then_some(result.access_secret_ref)
                .flatten(),
            refresh_secret_ref: (outcome == SecretRefOutcome::Succeeded)
                .then_some(result.refresh_secret_ref)
                .flatten(),
            secret_revision: result.secret_revision,
            refresh_operation_key: request.refresh_operation_key.clone(),
            provider_status: result.provider_status,
            response_digest: result.response_digest,
            idempotency_key: request.idempotency_key.clone(),
            payload_digest: request.payload_digest.clone(),
            expected_secret_revision: request.expected_secret_revision,
            authority_epoch: request.authority_epoch,
            owner_epoch: request.owner_epoch,
            generation: request.generation,
            fencing_token: request.fencing_token.clone(),
        };
        if let Err(error) = response.validate_against(request) {
            self.mark_response_unknown(request.operation_id.as_str())?;
            return Err(B3AdapterError::ProviderResponseInvalid(error.to_string()));
        }
        self.finish_refresh_response(request, response)
    }

    fn finish_refresh_backend_error(
        &mut self,
        request: &RefreshWithSecretRefRequest,
        error: SecretBackendError,
    ) -> Result<RefreshWithSecretRefResponse, B3AdapterError> {
        let response = local_refresh_error_response(
            request,
            error.outcome(),
            error.status(),
            "backend-error",
        );
        response
            .validate_against(request)
            .map_err(|validation| B3AdapterError::ProviderResponseInvalid(validation.to_string()))?;
        self.apply_event(request.operation_id.as_str(), request, error.event())?;
        self.mark_retry_exhausted_if_needed(request.operation_id.as_str())?;
        self.set_response(
            request.operation_id.as_str(),
            StoredResponse::Refresh(response.clone()),
        )?;
        Ok(response)
    }

    fn finish_refresh_provider_error(
        &mut self,
        request: &RefreshWithSecretRefRequest,
        error: ProviderAdapterError,
    ) -> Result<RefreshWithSecretRefResponse, B3AdapterError> {
        let response = local_refresh_error_response(
            request,
            error.outcome(),
            error.status(),
            "provider-error",
        );
        response
            .validate_against(request)
            .map_err(|validation| B3AdapterError::ProviderResponseInvalid(validation.to_string()))?;
        self.apply_event(request.operation_id.as_str(), request, error.event())?;
        self.mark_retry_exhausted_if_needed(request.operation_id.as_str())?;
        self.set_response(
            request.operation_id.as_str(),
            StoredResponse::Refresh(response.clone()),
        )?;
        Ok(response)
    }

    fn finish_refresh_response(
        &mut self,
        request: &RefreshWithSecretRefRequest,
        response: RefreshWithSecretRefResponse,
    ) -> Result<RefreshWithSecretRefResponse, B3AdapterError> {
        self.apply_event(
            request.operation_id.as_str(),
            request,
            event_for_outcome(response.outcome),
        )?;
        self.mark_retry_exhausted_if_needed(request.operation_id.as_str())?;
        self.set_response(
            request.operation_id.as_str(),
            StoredResponse::Refresh(response.clone()),
        )?;
        Ok(response)
    }

    fn finish_rotate(
        &mut self,
        request: &RotateSecretRefRequest,
        result: ProviderRotationResult,
    ) -> Result<RotateSecretRefResponse, B3AdapterError> {
        let outcome = outcome_for_status(result.provider_status);
        let response = RotateSecretRefResponse {
            schema_version: request.schema_version,
            response_id: result.response_id,
            operation_id: request.operation_id.clone(),
            provider_id: request.provider_id.clone(),
            profile_id: request.profile_id.clone(),
            token_family_id: request.token_family_id.clone(),
            outcome,
            new_refresh_secret_ref: (outcome == SecretRefOutcome::Succeeded)
                .then_some(result.new_refresh_secret_ref)
                .flatten(),
            secret_revision: result.secret_revision,
            refresh_operation_key: request.refresh_operation_key.clone(),
            response_digest: result.response_digest,
            idempotency_key: request.idempotency_key.clone(),
            payload_digest: request.payload_digest.clone(),
            expected_secret_revision: request.expected_secret_revision,
            authority_epoch: request.authority_epoch,
            owner_epoch: request.owner_epoch,
            generation: request.generation,
            fencing_token: request.fencing_token.clone(),
        };
        if let Err(error) = response.validate_against(request) {
            self.mark_response_unknown(request.operation_id.as_str())?;
            return Err(B3AdapterError::ProviderResponseInvalid(error.to_string()));
        }
        self.finish_rotate_response(request, response)
    }

    fn finish_rotate_backend_error(
        &mut self,
        request: &RotateSecretRefRequest,
        error: SecretBackendError,
    ) -> Result<RotateSecretRefResponse, B3AdapterError> {
        let response =
            local_rotate_error_response(request, error.outcome(), error.status(), "backend-error");
        response
            .validate_against(request)
            .map_err(|validation| B3AdapterError::ProviderResponseInvalid(validation.to_string()))?;
        self.apply_event(request.operation_id.as_str(), request, error.event())?;
        self.mark_retry_exhausted_if_needed(request.operation_id.as_str())?;
        self.set_response(
            request.operation_id.as_str(),
            StoredResponse::Rotate(response.clone()),
        )?;
        Ok(response)
    }

    fn finish_rotate_provider_error(
        &mut self,
        request: &RotateSecretRefRequest,
        error: ProviderAdapterError,
    ) -> Result<RotateSecretRefResponse, B3AdapterError> {
        let response =
            local_rotate_error_response(request, error.outcome(), error.status(), "provider-error");
        response
            .validate_against(request)
            .map_err(|validation| B3AdapterError::ProviderResponseInvalid(validation.to_string()))?;
        self.apply_event(request.operation_id.as_str(), request, error.event())?;
        self.mark_retry_exhausted_if_needed(request.operation_id.as_str())?;
        self.set_response(
            request.operation_id.as_str(),
            StoredResponse::Rotate(response.clone()),
        )?;
        Ok(response)
    }

    fn finish_rotate_response(
        &mut self,
        request: &RotateSecretRefRequest,
        response: RotateSecretRefResponse,
    ) -> Result<RotateSecretRefResponse, B3AdapterError> {
        self.apply_event(
            request.operation_id.as_str(),
            request,
            event_for_outcome(response.outcome),
        )?;
        self.mark_retry_exhausted_if_needed(request.operation_id.as_str())?;
        self.set_response(
            request.operation_id.as_str(),
            StoredResponse::Rotate(response.clone()),
        )?;
        Ok(response)
    }

    fn build_status_response(
        &self,
        request: &RefreshStatusByOperationKeyRequest,
        result: ProviderStatusResult,
    ) -> Result<RefreshStatusByOperationKeyResponse, B3AdapterError> {
        let outcome = outcome_for_status(result.provider_status);
        let mut response = RefreshStatusByOperationKeyResponse {
            schema_version: request.schema_version,
            response_id: result.response_id,
            operation_id: request.operation_id.clone(),
            provider_id: request.provider_id.clone(),
            profile_id: request.profile_id.clone(),
            token_family_id: request.token_family_id.clone(),
            refresh_operation_key: request.refresh_operation_key.clone(),
            idempotency_key: request.idempotency_key.clone(),
            payload_digest: request.payload_digest.clone(),
            expected_secret_revision: request.expected_secret_revision,
            authority_epoch: request.authority_epoch,
            owner_epoch: request.owner_epoch,
            generation: request.generation,
            fencing_token: request.fencing_token.clone(),
            outcome,
            secret_revision: result.secret_revision,
            response_digest: result.response_digest,
            provider_status: result.provider_status,
            status_revision: result.status_revision,
            observed_at: result.observed_at,
            binding_digest: local_response_digest(
                "status-binding-placeholder",
                request.operation_id.as_str(),
                result.provider_status,
            ),
            evidence_profile: "local-qualification-provider-status".to_string(),
            provider_query_receipt_digest: result.provider_query_receipt_digest,
            execution_mode: request.expected_execution_mode.clone(),
            mode_attestation_digest: Sha256Digest::for_bytes(
                b"hepta-authbus-b3-local-status-mode",
            ),
            policy_digest: request.policy_digest.clone(),
            audience: request.audience.clone(),
            key_epoch: 0,
            issuer: "local-mode-registry".to_string(),
            new_access_secret_ref: (outcome == SecretRefOutcome::Succeeded)
                .then_some(result.new_access_secret_ref)
                .flatten(),
            new_refresh_secret_ref: (outcome == SecretRefOutcome::Succeeded)
                .then_some(result.new_refresh_secret_ref)
                .flatten(),
            signature: None,
            key_id: None,
            issued_at: None,
            expires_at: None,
        };
        response.binding_digest = response
            .expected_binding_digest()
            .map_err(|error| B3AdapterError::ProviderResponseInvalid(error.to_string()))?;
        response
            .validate_against(request)
            .map_err(|error| B3AdapterError::ProviderResponseInvalid(error.to_string()))?;
        Ok(response)
    }

    fn validate_status_request(
        entry: &OperationEntry,
        request: &RefreshStatusByOperationKeyRequest,
    ) -> Result<(), B3AdapterError> {
        if entry.record.refresh_operation_key != request.refresh_operation_key
            || entry.record.provider_id != request.provider_id
            || entry.record.profile_id != request.profile_id
            || entry.record.token_family_id != request.token_family_id
            || entry.record.expected_secret_revision != request.expected_secret_revision
            || entry.record.authority_epoch != request.authority_epoch
            || entry.record.owner_epoch != request.owner_epoch
            || entry.record.generation != request.generation
            || entry.record.fencing_token != request.fencing_token
        {
            return Err(B3AdapterError::Conflict);
        }

        let matches_request = match &entry.request {
            StoredRequest::Refresh(original) => {
                original.idempotency_key == request.idempotency_key
                    && original.payload_digest == request.payload_digest
                    && original.policy_digest == request.policy_digest
                    && original.audience == request.audience
            }
            StoredRequest::Rotate(original) => {
                original.idempotency_key == request.idempotency_key
                    && original.payload_digest == request.payload_digest
                    && original.policy_digest == request.policy_digest
                    && original.audience == request.audience
            }
        };
        if !matches_request {
            return Err(B3AdapterError::Conflict);
        }
        Ok(())
    }

    fn validate_status_progress(
        &self,
        operation_id: &str,
        response: &RefreshStatusByOperationKeyResponse,
    ) -> Result<(), B3AdapterError> {
        let entry = self
            .operations
            .get(operation_id)
            .ok_or(B3AdapterError::OperationNotFound)?;
        if entry
            .last_status_revision
            .is_some_and(|previous_revision| response.status_revision <= previous_revision)
        {
            return Err(B3AdapterError::StatusRevisionConflict);
        }
        Ok(())
    }

    fn build_reconciled_replay(
        &self,
        operation_id: &str,
        status: &RefreshStatusByOperationKeyResponse,
    ) -> Result<StoredResponse, B3AdapterError> {
        let entry = self
            .operations
            .get(operation_id)
            .ok_or(B3AdapterError::OperationNotFound)?;
        match &entry.request {
            StoredRequest::Refresh(request) => {
                let response = RefreshWithSecretRefResponse {
                    schema_version: request.schema_version,
                    response_id: status.response_id.clone(),
                    operation_id: request.operation_id.clone(),
                    provider_id: request.provider_id.clone(),
                    profile_id: request.profile_id.clone(),
                    token_family_id: request.token_family_id.clone(),
                    outcome: status.outcome,
                    access_secret_ref: status.new_access_secret_ref.clone(),
                    refresh_secret_ref: status.new_refresh_secret_ref.clone(),
                    secret_revision: Some(status.secret_revision),
                    refresh_operation_key: request.refresh_operation_key.clone(),
                    provider_status: status.provider_status,
                    response_digest: status.response_digest.clone(),
                    idempotency_key: request.idempotency_key.clone(),
                    payload_digest: request.payload_digest.clone(),
                    expected_secret_revision: request.expected_secret_revision,
                    authority_epoch: request.authority_epoch,
                    owner_epoch: request.owner_epoch,
                    generation: request.generation,
                    fencing_token: request.fencing_token.clone(),
                };
                response.validate_against(request).map_err(|error| {
                    B3AdapterError::ProviderResponseInvalid(error.to_string())
                })?;
                Ok(StoredResponse::Refresh(response))
            }
            StoredRequest::Rotate(request) => {
                let response = RotateSecretRefResponse {
                    schema_version: request.schema_version,
                    response_id: status.response_id.clone(),
                    operation_id: request.operation_id.clone(),
                    provider_id: request.provider_id.clone(),
                    profile_id: request.profile_id.clone(),
                    token_family_id: request.token_family_id.clone(),
                    outcome: status.outcome,
                    new_refresh_secret_ref: status.new_refresh_secret_ref.clone(),
                    secret_revision: Some(status.secret_revision),
                    refresh_operation_key: request.refresh_operation_key.clone(),
                    response_digest: status.response_digest.clone(),
                    idempotency_key: request.idempotency_key.clone(),
                    payload_digest: request.payload_digest.clone(),
                    expected_secret_revision: request.expected_secret_revision,
                    authority_epoch: request.authority_epoch,
                    owner_epoch: request.owner_epoch,
                    generation: request.generation,
                    fencing_token: request.fencing_token.clone(),
                };
                response.validate_against(request).map_err(|error| {
                    B3AdapterError::ProviderResponseInvalid(error.to_string())
                })?;
                Ok(StoredResponse::Rotate(response))
            }
        }
    }

    fn apply_status_transition(
        &mut self,
        request: &RefreshStatusByOperationKeyRequest,
        outcome: SecretRefOutcome,
    ) -> Result<(), B3AdapterError> {
        let fence = SecretRefCallbackFence::new(
            request.authority_epoch,
            request.owner_epoch,
            request.generation,
            request.fencing_token.clone(),
        )
        .map_err(|error| B3AdapterError::InvalidRequest(error.to_string()))?;
        let entry = self
            .operations
            .get_mut(request.operation_id.as_str())
            .ok_or(B3AdapterError::OperationNotFound)?;

        if entry.record.state == SecretRefState::Indeterminate {
            entry
                .record
                .transition(SecretRefEvent::Lookup)
                .map_err(|error| B3AdapterError::InvalidState(error.to_string()))?;
        } else if entry.record.state != SecretRefState::Reconciling {
            let state = entry.record.state;
            return Err(B3AdapterError::InvalidState(format!(
                "status lookup cannot advance {state:?}"
            )));
        }

        let event = match outcome {
            SecretRefOutcome::Succeeded => SecretRefEvent::LookupRotated,
            SecretRefOutcome::Quarantined => SecretRefEvent::LookupInvalidGrant,
            SecretRefOutcome::TransientFailure | SecretRefOutcome::Indeterminate
                if !retry_available(&entry.record) =>
            {
                SecretRefEvent::ManualRequired
            }
            SecretRefOutcome::TransientFailure => SecretRefEvent::LookupTransientFailure,
            SecretRefOutcome::Indeterminate => SecretRefEvent::LookupRetryable,
        };
        entry
            .record
            .transition_with_fence(event, &fence)
            .map_err(|error| B3AdapterError::InvalidState(error.to_string()))?;
        let terminal_claim = entry
            .record
            .state
            .is_terminal()
            .then(|| entry.claim_key.clone());
        if let Some(claim_key) = terminal_claim {
            self.claims.remove(&claim_key);
        }
        Ok(())
    }

    fn apply_event<R: RequestIdentity>(
        &mut self,
        operation_id: &str,
        request: &R,
        event: SecretRefEvent,
    ) -> Result<(), B3AdapterError> {
        let fence = request
            .callback_fence()
            .map_err(|error| B3AdapterError::InvalidRequest(error.to_string()))?;
        let entry = self
            .operations
            .get_mut(operation_id)
            .ok_or(B3AdapterError::OperationNotFound)?;
        if event.requires_current_fence() {
            entry
                .record
                .transition_with_fence(event, &fence)
                .map_err(|error| B3AdapterError::InvalidState(error.to_string()))?;
        } else {
            entry
                .record
                .transition(event)
                .map_err(|error| B3AdapterError::InvalidState(error.to_string()))?;
        }
        let terminal_claim = entry
            .record
            .state
            .is_terminal()
            .then(|| entry.claim_key.clone());
        if let Some(claim_key) = terminal_claim {
            self.claims.remove(&claim_key);
        }
        Ok(())
    }

    fn mark_retry_exhausted_if_needed(
        &mut self,
        operation_id: &str,
    ) -> Result<(), B3AdapterError> {
        let entry = self
            .operations
            .get_mut(operation_id)
            .ok_or(B3AdapterError::OperationNotFound)?;
        if entry.record.state == SecretRefState::TransientFailure
            && !retry_available(&entry.record)
        {
            entry
                .record
                .transition(SecretRefEvent::RetryBudgetExhausted)
                .map_err(|error| B3AdapterError::InvalidState(error.to_string()))?;
        }
        Ok(())
    }

    fn mark_response_unknown(&mut self, operation_id: &str) -> Result<(), B3AdapterError> {
        let entry = self
            .operations
            .get_mut(operation_id)
            .ok_or(B3AdapterError::OperationNotFound)?;
        let fence = SecretRefCallbackFence::new(
            entry.record.authority_epoch,
            entry.record.owner_epoch,
            entry.record.generation,
            entry.record.fencing_token.clone(),
        )
        .map_err(|error| B3AdapterError::InvalidState(error.to_string()))?;
        if entry.record.state == SecretRefState::InFlight {
            entry
                .record
                .transition_with_fence(SecretRefEvent::ResponseUnknown, &fence)
                .map_err(|error| B3AdapterError::InvalidState(error.to_string()))?;
        }
        Ok(())
    }

    fn set_response(
        &mut self,
        operation_id: &str,
        response: StoredResponse,
    ) -> Result<(), B3AdapterError> {
        let entry = self
            .operations
            .get_mut(operation_id)
            .ok_or(B3AdapterError::OperationNotFound)?;
        entry.response = Some(response);
        Ok(())
    }
}

trait RequestIdentity {
    fn callback_fence(&self) -> Result<SecretRefCallbackFence, AuthBusContractError>;
}

impl RequestIdentity for RefreshWithSecretRefRequest {
    fn callback_fence(&self) -> Result<SecretRefCallbackFence, AuthBusContractError> {
        SecretRefCallbackFence::new(
            self.authority_epoch,
            self.owner_epoch,
            self.generation,
            self.fencing_token.clone(),
        )
    }
}

impl RequestIdentity for RotateSecretRefRequest {
    fn callback_fence(&self) -> Result<SecretRefCallbackFence, AuthBusContractError> {
        SecretRefCallbackFence::new(
            self.authority_epoch,
            self.owner_epoch,
            self.generation,
            self.fencing_token.clone(),
        )
    }
}

fn retry_available(record: &SecretRefOperationRecord) -> bool {
    record.attempt <= record.retry_budget
}

fn outcome_for_status(status: SecretProviderStatus) -> SecretRefOutcome {
    match status {
        SecretProviderStatus::Succeeded | SecretProviderStatus::Rotated => {
            SecretRefOutcome::Succeeded
        }
        SecretProviderStatus::InvalidGrant | SecretProviderStatus::Quarantined => {
            SecretRefOutcome::Quarantined
        }
        SecretProviderStatus::Unknown => SecretRefOutcome::Indeterminate,
        SecretProviderStatus::Unauthorized
        | SecretProviderStatus::Conflict
        | SecretProviderStatus::Timeout
        | SecretProviderStatus::Unavailable
        | SecretProviderStatus::Sealed
        | SecretProviderStatus::StaleFence
        | SecretProviderStatus::SchemaInvalid
        | SecretProviderStatus::TransientFailure => SecretRefOutcome::TransientFailure,
    }
}

fn event_for_outcome(outcome: SecretRefOutcome) -> SecretRefEvent {
    match outcome {
        SecretRefOutcome::Succeeded => SecretRefEvent::Rotated,
        SecretRefOutcome::Quarantined => SecretRefEvent::InvalidGrant,
        SecretRefOutcome::TransientFailure => SecretRefEvent::TransientFailure,
        SecretRefOutcome::Indeterminate => SecretRefEvent::ResponseUnknown,
    }
}

fn claim_key(provider_id: &str, profile_id: &str, token_family_id: &str) -> String {
    let mut bytes = Vec::new();
    for value in [provider_id, profile_id, token_family_id] {
        bytes.extend_from_slice(&(value.len() as u64).to_be_bytes());
        bytes.extend_from_slice(value.as_bytes());
    }
    format!("claim:{}", Sha256Digest::for_bytes(&bytes).as_str())
}

fn local_refresh_error_response(
    request: &RefreshWithSecretRefRequest,
    outcome: SecretRefOutcome,
    status: SecretProviderStatus,
    source: &str,
) -> RefreshWithSecretRefResponse {
    RefreshWithSecretRefResponse {
        schema_version: request.schema_version,
        response_id: local_response_id(source, request.operation_id.as_str()),
        operation_id: request.operation_id.clone(),
        provider_id: request.provider_id.clone(),
        profile_id: request.profile_id.clone(),
        token_family_id: request.token_family_id.clone(),
        outcome,
        access_secret_ref: None,
        refresh_secret_ref: None,
        secret_revision: None,
        refresh_operation_key: request.refresh_operation_key.clone(),
        provider_status: status,
        response_digest: local_response_digest(
            source,
            request.operation_id.as_str(),
            status,
        ),
        idempotency_key: request.idempotency_key.clone(),
        payload_digest: request.payload_digest.clone(),
        expected_secret_revision: request.expected_secret_revision,
        authority_epoch: request.authority_epoch,
        owner_epoch: request.owner_epoch,
        generation: request.generation,
        fencing_token: request.fencing_token.clone(),
    }
}

fn local_rotate_error_response(
    request: &RotateSecretRefRequest,
    outcome: SecretRefOutcome,
    status: SecretProviderStatus,
    source: &str,
) -> RotateSecretRefResponse {
    RotateSecretRefResponse {
        schema_version: request.schema_version,
        response_id: local_response_id(source, request.operation_id.as_str()),
        operation_id: request.operation_id.clone(),
        provider_id: request.provider_id.clone(),
        profile_id: request.profile_id.clone(),
        token_family_id: request.token_family_id.clone(),
        outcome,
        new_refresh_secret_ref: None,
        secret_revision: None,
        refresh_operation_key: request.refresh_operation_key.clone(),
        response_digest: local_response_digest(
            source,
            request.operation_id.as_str(),
            status,
        ),
        idempotency_key: request.idempotency_key.clone(),
        payload_digest: request.payload_digest.clone(),
        expected_secret_revision: request.expected_secret_revision,
        authority_epoch: request.authority_epoch,
        owner_epoch: request.owner_epoch,
        generation: request.generation,
        fencing_token: request.fencing_token.clone(),
    }
}

fn local_response_id(kind: &str, operation_id: &str) -> String {
    format!(
        "b3-{kind}-response:{}",
        Sha256Digest::for_bytes(format!("{kind}:{operation_id}").as_bytes()).as_str()
    )
}

fn local_response_digest(
    kind: &str,
    operation_id: &str,
    status: SecretProviderStatus,
) -> Sha256Digest {
    Sha256Digest::for_bytes(format!("{kind}:{operation_id}:{status:?}").as_bytes())
}
