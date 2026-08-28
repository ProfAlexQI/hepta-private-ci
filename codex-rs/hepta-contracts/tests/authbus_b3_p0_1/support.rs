use std::collections::VecDeque;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use codex_hepta_contracts::authbus_b3_adapter::{
    B3AdapterError, ProcessBoundSecret, ProcessBoundSecretRefAdapter, ProviderAdapterError,
    ProviderRefreshResult, ProviderRotationResult, ProviderStatusResult,
    QualificationSecretBackend, SecretRefProvider,
};
use codex_hepta_contracts::{
    OpaqueSecretRef, RefreshStatusByOperationKeyRequest, RefreshWithSecretRefRequest,
    RotateSecretRefRequest, SecretProviderStatus, SecretRefOutcome, SecretRefState, Sha256Digest,
    derive_refresh_operation_id, derive_refresh_operation_key,
};

fn digest(label: &str) -> Sha256Digest {
    Sha256Digest::for_bytes(label.as_bytes())
}

fn secret_ref(key: &str, bytes: &[u8]) -> OpaqueSecretRef {
    OpaqueSecretRef::new(
        "qualification-backend",
        "oauth",
        key,
        1,
        Sha256Digest::for_bytes(bytes),
    )
    .unwrap_or_else(|error| panic!("valid secret reference: {error:?}"))
}

fn refresh_request(reference: OpaqueSecretRef, idempotency_key: &str) -> RefreshWithSecretRefRequest {
    let provider_id = "provider-p0";
    let profile_id = "profile-p0";
    let token_family_id = "family-p0";
    let payload_digest = digest(&format!("payload:{idempotency_key}"));
    let policy_digest = digest("policy");
    let scope_digest = digest("scope");
    let purpose_digest = digest("purpose");
    let fencing_token = digest("fence");
    let refresh_operation_key = derive_refresh_operation_key(
        provider_id,
        profile_id,
        token_family_id,
        idempotency_key,
        1,
        &scope_digest,
        &purpose_digest,
        &payload_digest,
        &policy_digest,
        2,
        3,
        4,
        &fencing_token,
    );
    let operation_id =
        derive_refresh_operation_id(&refresh_operation_key, idempotency_key, &payload_digest);
    RefreshWithSecretRefRequest {
        schema_version: 1,
        operation_id,
        refresh_operation_key,
        command_id: format!("command:{idempotency_key}"),
        run_id: format!("run:{idempotency_key}"),
        profile_id: profile_id.to_string(),
        provider_id: provider_id.to_string(),
        token_family_id: token_family_id.to_string(),
        secret_ref: reference,
        expected_secret_revision: 1,
        idempotency_key: idempotency_key.to_string(),
        payload_digest,
        policy_digest,
        scope_digest,
        authority_epoch: 2,
        owner_epoch: 3,
        generation: 4,
        fencing_token,
        logical_clock: 5,
        causal_parent_event_id: "event:parent".to_string(),
        deadline_at: 100,
        purpose_digest,
        audience: "hepta.auth.local-mode".to_string(),
    }
}

fn status_request(request: &RefreshWithSecretRefRequest) -> RefreshStatusByOperationKeyRequest {
    RefreshStatusByOperationKeyRequest {
        schema_version: request.schema_version,
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
        deadline_at: request.deadline_at,
        audience: request.audience.clone(),
        expected_execution_mode: "qualification".to_string(),
        policy_digest: request.policy_digest.clone(),
    }
}

fn successful_refresh(request: &RefreshWithSecretRefRequest) -> ProviderRefreshResult {
    ProviderRefreshResult {
        response_id: "provider:refresh:success".to_string(),
        provider_status: SecretProviderStatus::Rotated,
        access_secret_ref: Some(secret_ref("access-v2", b"access-v2")),
        refresh_secret_ref: Some(secret_ref("refresh-v2", b"refresh-v2")),
        secret_revision: Some(request.expected_secret_revision + 1),
        response_digest: digest("provider:refresh:success"),
    }
}

fn successful_status(request: &RefreshStatusByOperationKeyRequest) -> ProviderStatusResult {
    ProviderStatusResult {
        response_id: "provider:status:success".to_string(),
        provider_status: SecretProviderStatus::Rotated,
        secret_revision: request.expected_secret_revision + 1,
        response_digest: digest("provider:status:success"),
        status_revision: 1,
        observed_at: 10,
        provider_query_receipt_digest: digest("provider:status:receipt"),
        new_access_secret_ref: Some(secret_ref("status-access-v2", b"status-access-v2")),
        new_refresh_secret_ref: Some(secret_ref("status-refresh-v2", b"status-refresh-v2")),
    }
}

fn unknown_status(request: &RefreshStatusByOperationKeyRequest) -> ProviderStatusResult {
    ProviderStatusResult {
        response_id: "provider:status:unknown".to_string(),
        provider_status: SecretProviderStatus::Unknown,
        secret_revision: request.expected_secret_revision,
        response_digest: digest("provider:status:unknown"),
        status_revision: 1,
        observed_at: 10,
        provider_query_receipt_digest: digest("provider:status:unknown:receipt"),
        new_access_secret_ref: None,
        new_refresh_secret_ref: None,
    }
}

fn backend(reference: &OpaqueSecretRef, bytes: &[u8]) -> QualificationSecretBackend {
    let mut backend = QualificationSecretBackend::default();
    backend
        .insert(reference.clone(), bytes)
        .unwrap_or_else(|error| panic!("insert qualification secret: {error:?}"));
    backend
}

#[derive(Clone, Default)]
struct ScriptedProvider {
    refresh_results: Arc<Mutex<VecDeque<Result<ProviderRefreshResult, ProviderAdapterError>>>>,
    status_results: Arc<Mutex<VecDeque<Result<ProviderStatusResult, ProviderAdapterError>>>>,
    refresh_calls: Arc<AtomicUsize>,
    status_calls: Arc<AtomicUsize>,
}

impl ScriptedProvider {
    fn push_refresh(&self, result: Result<ProviderRefreshResult, ProviderAdapterError>) {
        self.refresh_results
            .lock()
            .unwrap_or_else(|_| panic!("refresh queue poisoned"))
            .push_back(result);
    }

    fn push_status(&self, result: Result<ProviderStatusResult, ProviderAdapterError>) {
        self.status_results
            .lock()
            .unwrap_or_else(|_| panic!("status queue poisoned"))
            .push_back(result);
    }

    fn refresh_calls(&self) -> usize {
        self.refresh_calls.load(Ordering::SeqCst)
    }

    fn status_calls(&self) -> usize {
        self.status_calls.load(Ordering::SeqCst)
    }
}

impl SecretRefProvider for ScriptedProvider {
    fn refresh(
        &self,
        _request: &RefreshWithSecretRefRequest,
        _secret: &ProcessBoundSecret,
    ) -> Result<ProviderRefreshResult, ProviderAdapterError> {
        self.refresh_calls.fetch_add(1, Ordering::SeqCst);
        self.refresh_results
            .lock()
            .unwrap_or_else(|_| panic!("refresh queue poisoned"))
            .pop_front()
            .unwrap_or(Err(ProviderAdapterError::Unknown))
    }

    fn rotate(
        &self,
        _request: &RotateSecretRefRequest,
        _secret: &ProcessBoundSecret,
    ) -> Result<ProviderRotationResult, ProviderAdapterError> {
        Err(ProviderAdapterError::Unknown)
    }

    fn status_by_operation_key(
        &self,
        _request: &RefreshStatusByOperationKeyRequest,
    ) -> Result<ProviderStatusResult, ProviderAdapterError> {
        self.status_calls.fetch_add(1, Ordering::SeqCst);
        self.status_results
            .lock()
            .unwrap_or_else(|_| panic!("status queue poisoned"))
            .pop_front()
            .unwrap_or(Err(ProviderAdapterError::Unknown))
    }
}
