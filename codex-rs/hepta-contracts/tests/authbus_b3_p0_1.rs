#![cfg(feature = "authbus-local-qualification")]

use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use codex_hepta_contracts::OpaqueSecretRef;
use codex_hepta_contracts::RefreshStatusByOperationKeyRequest;
use codex_hepta_contracts::RefreshWithSecretRefRequest;
use codex_hepta_contracts::RotateSecretRefRequest;
use codex_hepta_contracts::SecretProviderStatus;
use codex_hepta_contracts::SecretRefOutcome;
use codex_hepta_contracts::SecretRefState;
use codex_hepta_contracts::Sha256Digest;
use codex_hepta_contracts::authbus_b3_adapter::B3AdapterError;
use codex_hepta_contracts::authbus_b3_adapter::ProcessBoundSecret;
use codex_hepta_contracts::authbus_b3_adapter::ProcessBoundSecretRefAdapter;
use codex_hepta_contracts::authbus_b3_adapter::ProviderAdapterError;
use codex_hepta_contracts::authbus_b3_adapter::ProviderRefreshResult;
use codex_hepta_contracts::authbus_b3_adapter::ProviderRotationResult;
use codex_hepta_contracts::authbus_b3_adapter::ProviderStatusResult;
use codex_hepta_contracts::authbus_b3_adapter::QualificationSecretBackend;
use codex_hepta_contracts::authbus_b3_adapter::SecretRefProvider;
use codex_hepta_contracts::derive_refresh_operation_id;
use codex_hepta_contracts::derive_refresh_operation_key;

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
    .unwrap_or_else(|error| panic!("valid opaque reference: {error:?}"))
}

fn refresh_request(
    reference: OpaqueSecretRef,
    idempotency_key: &str,
) -> RefreshWithSecretRefRequest {
    let provider_id = "provider-p0-1";
    let profile_id = "profile-p0-1";
    let token_family_id = "family-p0-1";
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

fn status_success(request: &RefreshStatusByOperationKeyRequest) -> ProviderStatusResult {
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

fn status_unknown(request: &RefreshStatusByOperationKeyRequest) -> ProviderStatusResult {
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
    refresh_results:
        Arc<Mutex<VecDeque<Result<ProviderRefreshResult, ProviderAdapterError>>>>,
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

    fn refresh_call_count(&self) -> usize {
        self.refresh_calls.load(Ordering::SeqCst)
    }

    fn status_call_count(&self) -> usize {
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

#[test]
fn provider_timeout_is_indeterminate_lookup_only_and_terminal_replay_updates() {
    let bytes = b"timeout-secret";
    let reference = secret_ref("timeout", bytes);
    let request = refresh_request(reference.clone(), "timeout");
    let provider = ScriptedProvider::default();
    provider.push_refresh(Err(ProviderAdapterError::Timeout));
    provider.push_status(Ok(status_success(&status_request(&request))));
    let mut adapter =
        ProcessBoundSecretRefAdapter::new(backend(&reference, bytes), provider.clone());

    let first = adapter.refresh(request.clone()).expect("timeout observation");
    assert_eq!(first.outcome, SecretRefOutcome::Indeterminate);
    assert_eq!(first.provider_status, SecretProviderStatus::Unknown);
    assert_eq!(
        adapter.operation_state(&request.operation_id),
        Some(SecretRefState::Indeterminate)
    );
    assert_eq!(
        adapter.retry_refresh(request.clone()),
        Err(B3AdapterError::ReconcileRequired)
    );
    assert_eq!(provider.refresh_call_count(), 1);

    let status = adapter
        .status_by_operation_key(status_request(&request))
        .expect("lookup-only status");
    assert_eq!(status.outcome, SecretRefOutcome::Succeeded);
    assert_eq!(
        adapter.operation_state(&request.operation_id),
        Some(SecretRefState::Succeeded)
    );

    let replay = adapter.refresh(request).expect("terminal replay");
    assert_eq!(replay.outcome, SecretRefOutcome::Succeeded);
    assert!(replay.access_secret_ref.is_some());
    assert!(replay.refresh_secret_ref.is_some());
    assert_eq!(provider.refresh_call_count(), 1);
    assert_eq!(provider.status_call_count(), 1);
}

#[test]
fn verified_transient_replays_locally_until_explicit_retry() {
    let bytes = b"transient-secret";
    let reference = secret_ref("transient", bytes);
    let request = refresh_request(reference.clone(), "transient");
    let provider = ScriptedProvider::default();
    provider.push_refresh(Err(ProviderAdapterError::Unauthorized));
    provider.push_refresh(Ok(successful_refresh(&request)));
    let mut adapter =
        ProcessBoundSecretRefAdapter::new(backend(&reference, bytes), provider.clone());

    let first = adapter.refresh(request.clone()).expect("transient response");
    assert_eq!(first.outcome, SecretRefOutcome::TransientFailure);
    assert_eq!(
        adapter.operation_state(&request.operation_id),
        Some(SecretRefState::TransientFailure)
    );

    assert_eq!(
        adapter.refresh(request.clone()).expect("local replay"),
        first
    );
    assert_eq!(provider.refresh_call_count(), 1);
    assert_eq!(
        adapter.status_by_operation_key(status_request(&request)),
        Err(B3AdapterError::RetryRequired)
    );
    assert_eq!(provider.status_call_count(), 0);

    let retried = adapter.retry_refresh(request).expect("explicit retry");
    assert_eq!(retried.outcome, SecretRefOutcome::Succeeded);
    assert_eq!(provider.refresh_call_count(), 2);
}

#[test]
fn zero_retry_budget_moves_verified_failure_to_manual_required() {
    let bytes = b"manual-secret";
    let reference = secret_ref("manual", bytes);
    let request = refresh_request(reference.clone(), "manual");
    let provider = ScriptedProvider::default();
    provider.push_refresh(Err(ProviderAdapterError::Unauthorized));
    let mut adapter = ProcessBoundSecretRefAdapter::with_retry_budget(
        backend(&reference, bytes),
        provider.clone(),
        0,
    );

    let response = adapter.refresh(request.clone()).expect("transient response");
    assert_eq!(response.outcome, SecretRefOutcome::TransientFailure);
    assert_eq!(
        adapter.operation_state(&request.operation_id),
        Some(SecretRefState::ManualRequired)
    );
    assert_eq!(
        adapter.retry_refresh(request.clone()),
        Err(B3AdapterError::RetryBudgetExhausted)
    );
    assert_eq!(
        adapter.status_by_operation_key(status_request(&request)),
        Err(B3AdapterError::ManualEvidenceRequired)
    );
    assert_eq!(provider.refresh_call_count(), 1);
    assert_eq!(provider.status_call_count(), 0);
}

#[test]
fn provider_schema_error_is_indeterminate_not_retryable() {
    let bytes = b"schema-secret";
    let reference = secret_ref("schema", bytes);
    let request = refresh_request(reference.clone(), "schema");
    let provider = ScriptedProvider::default();
    provider.push_refresh(Err(ProviderAdapterError::SchemaInvalid));
    let mut adapter =
        ProcessBoundSecretRefAdapter::new(backend(&reference, bytes), provider.clone());

    let response = adapter.refresh(request.clone()).expect("schema observation");
    assert_eq!(response.outcome, SecretRefOutcome::Indeterminate);
    assert_eq!(response.provider_status, SecretProviderStatus::Unknown);
    assert_eq!(
        adapter.retry_refresh(request),
        Err(B3AdapterError::ReconcileRequired)
    );
    assert_eq!(provider.refresh_call_count(), 1);
}

#[test]
fn unknown_lookup_moves_to_backoff_before_explicit_retry() {
    let bytes = b"lookup-unknown-secret";
    let reference = secret_ref("lookup-unknown", bytes);
    let request = refresh_request(reference.clone(), "lookup-unknown");
    let provider = ScriptedProvider::default();
    provider.push_refresh(Err(ProviderAdapterError::Timeout));
    provider.push_status(Ok(status_unknown(&status_request(&request))));
    provider.push_refresh(Ok(successful_refresh(&request)));
    let mut adapter =
        ProcessBoundSecretRefAdapter::new(backend(&reference, bytes), provider.clone());

    let initial = adapter.refresh(request.clone()).expect("initial unknown");
    let status = adapter
        .status_by_operation_key(status_request(&request))
        .expect("unknown status");
    assert_eq!(status.outcome, SecretRefOutcome::Indeterminate);
    assert_eq!(
        adapter.operation_state(&request.operation_id),
        Some(SecretRefState::Backoff)
    );
    assert_eq!(
        adapter.refresh(request.clone()).expect("nonterminal replay"),
        initial
    );

    let retried = adapter.retry_refresh(request).expect("retry after lookup");
    assert_eq!(retried.outcome, SecretRefOutcome::Succeeded);
    assert_eq!(provider.refresh_call_count(), 2);
    assert_eq!(provider.status_call_count(), 1);
}

#[test]
fn one_retry_budget_exhausts_after_second_verified_failure() {
    let bytes = b"budget-secret";
    let reference = secret_ref("budget", bytes);
    let request = refresh_request(reference.clone(), "budget");
    let provider = ScriptedProvider::default();
    provider.push_refresh(Err(ProviderAdapterError::Unauthorized));
    provider.push_refresh(Err(ProviderAdapterError::Conflict));
    let mut adapter =
        ProcessBoundSecretRefAdapter::new(backend(&reference, bytes), provider.clone());

    adapter.refresh(request.clone()).expect("first transient");
    let second = adapter
        .retry_refresh(request.clone())
        .expect("second transient");
    assert_eq!(second.outcome, SecretRefOutcome::TransientFailure);
    assert_eq!(adapter.operation_attempt(&request.operation_id), Some(2));
    assert_eq!(
        adapter.operation_state(&request.operation_id),
        Some(SecretRefState::ManualRequired)
    );
    assert_eq!(
        adapter.retry_refresh(request),
        Err(B3AdapterError::RetryBudgetExhausted)
    );
    assert_eq!(provider.refresh_call_count(), 2);
}
