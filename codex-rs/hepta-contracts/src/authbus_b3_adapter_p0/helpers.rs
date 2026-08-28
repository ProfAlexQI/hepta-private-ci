trait RequestFence {
    fn callback_fence(&self) -> Result<SecretRefCallbackFence, AuthBusContractError>;
}

impl RequestFence for RefreshWithSecretRefRequest {
    fn callback_fence(&self) -> Result<SecretRefCallbackFence, AuthBusContractError> {
        SecretRefCallbackFence::new(
            self.authority_epoch,
            self.owner_epoch,
            self.generation,
            self.fencing_token.clone(),
        )
    }
}

impl RequestFence for RotateSecretRefRequest {
    fn callback_fence(&self) -> Result<SecretRefCallbackFence, AuthBusContractError> {
        SecretRefCallbackFence::new(
            self.authority_epoch,
            self.owner_epoch,
            self.generation,
            self.fencing_token.clone(),
        )
    }
}

fn transition(
    record: &mut SecretRefOperationRecord,
    event: SecretRefEvent,
) -> Result<(), B3AdapterError> {
    record
        .transition(event)
        .map_err(|error| B3AdapterError::InvalidState(error.to_string()))
}

fn transition_with_fence(
    record: &mut SecretRefOperationRecord,
    event: SecretRefEvent,
    fence: &SecretRefCallbackFence,
) -> Result<(), B3AdapterError> {
    record
        .transition_with_fence(event, fence)
        .map_err(|error| B3AdapterError::InvalidState(error.to_string()))
}

fn retry_available(record: &SecretRefOperationRecord) -> bool {
    record.attempt <= record.retry_budget
}

fn provider_error_outcome(error: ProviderAdapterError) -> SecretRefOutcome {
    match error {
        ProviderAdapterError::InvalidGrant => SecretRefOutcome::Quarantined,
        ProviderAdapterError::Timeout
        | ProviderAdapterError::Unavailable
        | ProviderAdapterError::SchemaInvalid
        | ProviderAdapterError::Unknown => SecretRefOutcome::Indeterminate,
        ProviderAdapterError::Unauthorized
        | ProviderAdapterError::Conflict
        | ProviderAdapterError::Sealed
        | ProviderAdapterError::StaleFence => SecretRefOutcome::TransientFailure,
    }
}

fn provider_error_status(error: ProviderAdapterError) -> SecretProviderStatus {
    match error {
        ProviderAdapterError::InvalidGrant => SecretProviderStatus::InvalidGrant,
        ProviderAdapterError::Unauthorized => SecretProviderStatus::Unauthorized,
        ProviderAdapterError::Conflict => SecretProviderStatus::Conflict,
        ProviderAdapterError::Sealed => SecretProviderStatus::Sealed,
        ProviderAdapterError::StaleFence => SecretProviderStatus::StaleFence,
        ProviderAdapterError::Timeout
        | ProviderAdapterError::Unavailable
        | ProviderAdapterError::SchemaInvalid
        | ProviderAdapterError::Unknown => SecretProviderStatus::Unknown,
    }
}

fn backend_status(error: SecretBackendError) -> SecretProviderStatus {
    match error {
        SecretBackendError::NotFound | SecretBackendError::Unavailable => {
            SecretProviderStatus::Unavailable
        }
        SecretBackendError::Unauthorized => SecretProviderStatus::Unauthorized,
        SecretBackendError::Timeout => SecretProviderStatus::Timeout,
        SecretBackendError::Sealed => SecretProviderStatus::Sealed,
        SecretBackendError::InvalidReference => SecretProviderStatus::SchemaInvalid,
    }
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

    let matches = match &entry.request {
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
    if !matches {
        return Err(B3AdapterError::Conflict);
    }
    Ok(())
}

fn build_status_response(
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
        new_access_secret_ref: if outcome == SecretRefOutcome::Succeeded {
            result.new_access_secret_ref
        } else {
            None
        },
        new_refresh_secret_ref: if outcome == SecretRefOutcome::Succeeded {
            result.new_refresh_secret_ref
        } else {
            None
        },
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

fn refresh_error_response(
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

fn rotate_error_response(
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

fn claim_key(provider_id: &str, profile_id: &str, token_family_id: &str) -> String {
    let mut bytes = Vec::new();
    for value in [provider_id, profile_id, token_family_id] {
        bytes.extend_from_slice(&(value.len() as u64).to_be_bytes());
        bytes.extend_from_slice(value.as_bytes());
    }
    format!("claim:{}", Sha256Digest::for_bytes(&bytes).as_str())
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
