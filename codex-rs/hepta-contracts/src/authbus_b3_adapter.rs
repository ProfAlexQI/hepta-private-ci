//! AuthBus B3 P0.1 qualification adapter.
//!
//! This module supersedes the first in-memory adapter for feature-on
//! qualification. It reuses the secret/backend/provider boundary types from
//! the legacy harness, but tightens provider-call uncertainty, explicit retry,
//! lookup-only reconciliation, retry-budget exhaustion, and current-result
//! replay. It remains non-durable and carries no runtime or production
//! authority.

use std::collections::BTreeMap;
use std::fmt;

use crate::AuthBusContractError;
use crate::Sha256Digest;
use crate::authbus::b3::{
    OpaqueSecretRef, RefreshStatusByOperationKeyRequest, RefreshStatusByOperationKeyResponse,
    RefreshWithSecretRefRequest, RefreshWithSecretRefResponse, RotateSecretRefRequest,
    RotateSecretRefResponse, SecretProviderStatus, SecretRefCallbackFence, SecretRefEvent,
    SecretRefOperationRecord, SecretRefOutcome, SecretRefState,
};

#[path = "authbus_b3_adapter_legacy.rs"]
mod legacy;

pub use legacy::{
    AUTHBUS_B3_ADAPTER_AUTHORITY, AUTHBUS_B3_ADAPTER_EFFECT_AUTHORITY,
    AUTHBUS_B3_ADAPTER_EXECUTE_ALLOWED, AUTHBUS_B3_ADAPTER_G5_ALLOWED,
    AUTHBUS_B3_ADAPTER_OPERATOR_ACCEPTANCE, AUTHBUS_B3_ADAPTER_PRODUCTION_CALLER,
    AUTHBUS_B3_ADAPTER_PRODUCTION_WRITER, AUTHBUS_B3_ADAPTER_PROMOTION,
    AUTHBUS_B3_ADAPTER_QUALIFICATION_ONLY, ProcessBoundSecret, ProviderAdapterError,
    ProviderRefreshResult, ProviderRotationResult, ProviderStatusResult,
    QualificationSecretBackend, SecretBackendError, SecretRefBackend, SecretRefProvider,
};

#[doc(hidden)]
pub use legacy::{
    B3AdapterError as LegacyB3AdapterError,
    ProcessBoundSecretRefAdapter as LegacyProcessBoundSecretRefAdapter,
};

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
                formatter.write_str("B3 operation requires explicit retry")
            }
            Self::RetryNotAllowed => {
                formatter.write_str("B3 retry is not allowed in the current state")
            }
            Self::RetryBudgetExhausted => {
                formatter.write_str("B3 retry budget is exhausted")
            }
            Self::ManualEvidenceRequired => {
                formatter.write_str("B3 operation requires explicit manual evidence")
            }
            Self::StatusRevisionConflict => {
                formatter.write_str("B3 status revision is stale or conflicting")
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

struct OperationEntry {
    request_digest: Sha256Digest,
    claim_key: String,
    kind: OperationKind,
    request: StoredRequest,
    record: SecretRefOperationRecord,
    response: Option<StoredResponse>,
    last_status_revision: Option<u64>,
}

pub struct ProcessBoundSecretRefAdapter<B, P> {
    backend: B,
    provider: P,
    retry_budget: u32,
    operations: BTreeMap<String, OperationEntry>,
    claims: BTreeMap<String, String>,
}

trait AdapterRequest: Clone {
    fn validate_request(&self) -> Result<(), AuthBusContractError>;
    fn digest_request(&self) -> Result<Sha256Digest, AuthBusContractError>;
    fn operation_record(
        &self,
        retry_budget: u32,
    ) -> Result<SecretRefOperationRecord, AuthBusContractError>;
    fn kind(&self) -> OperationKind;
    fn operation_id(&self) -> &str;
    fn provider_id(&self) -> &str;
    fn profile_id(&self) -> &str;
    fn token_family_id(&self) -> &str;
    fn stored_request(&self) -> StoredRequest;
}

impl AdapterRequest for RefreshWithSecretRefRequest {
    fn validate_request(&self) -> Result<(), AuthBusContractError> {
        self.validate()
    }

    fn digest_request(&self) -> Result<Sha256Digest, AuthBusContractError> {
        self.digest()
    }

    fn operation_record(
        &self,
        retry_budget: u32,
    ) -> Result<SecretRefOperationRecord, AuthBusContractError> {
        SecretRefOperationRecord::from_refresh_request(self, retry_budget)
    }

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

    fn stored_request(&self) -> StoredRequest {
        StoredRequest::Refresh(self.clone())
    }
}

impl AdapterRequest for RotateSecretRefRequest {
    fn validate_request(&self) -> Result<(), AuthBusContractError> {
        self.validate()
    }

    fn digest_request(&self) -> Result<Sha256Digest, AuthBusContractError> {
        self.digest()
    }

    fn operation_record(
        &self,
        retry_budget: u32,
    ) -> Result<SecretRefOperationRecord, AuthBusContractError> {
        let request = RefreshWithSecretRefRequest {
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
        SecretRefOperationRecord::from_refresh_request(&request, retry_budget)
    }

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

    fn stored_request(&self) -> StoredRequest {
        StoredRequest::Rotate(self.clone())
    }
}

include!("authbus_b3_adapter_p0/core.rs");
include!("authbus_b3_adapter_p0/results.rs");
include!("authbus_b3_adapter_p0/state.rs");
include!("authbus_b3_adapter_p0/helpers.rs");
