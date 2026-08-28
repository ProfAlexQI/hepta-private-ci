#[test]
fn provider_timeout_requires_lookup_and_terminal_replay_is_refreshed() {
    let bytes = b"timeout-secret";
    let reference = secret_ref("timeout", bytes);
    let request = refresh_request(reference.clone(), "timeout");
    let provider = ScriptedProvider::default();
    provider.push_refresh(Err(ProviderAdapterError::Timeout));
    provider.push_status(Ok(successful_status(&status_request(&request))));
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
    assert_eq!(provider.refresh_calls(), 1);

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
    assert_eq!(provider.refresh_calls(), 1);
    assert_eq!(provider.status_calls(), 1);
}

#[test]
fn rotation_timeout_uses_the_same_lookup_only_terminal_replay() {
    let bytes = b"rotation-timeout-secret";
    let reference = secret_ref("rotation-timeout", bytes);
    let request = rotate_request(reference.clone(), "rotation-timeout");
    let lookup = rotate_status_request(&request);
    let provider = ScriptedProvider::default();
    provider.push_rotate(Err(ProviderAdapterError::Timeout));
    provider.push_status(Ok(successful_status(&lookup)));
    let mut adapter =
        ProcessBoundSecretRefAdapter::new(backend(&reference, bytes), provider.clone());

    let first = adapter.rotate(request.clone()).expect("rotation timeout");
    assert_eq!(first.outcome, SecretRefOutcome::Indeterminate);
    assert_eq!(
        adapter.operation_state(&request.operation_id),
        Some(SecretRefState::Indeterminate)
    );
    assert_eq!(
        adapter.retry_rotate(request.clone()),
        Err(B3AdapterError::ReconcileRequired)
    );

    let status = adapter
        .status_by_operation_key(lookup)
        .expect("rotation lookup");
    assert_eq!(status.outcome, SecretRefOutcome::Succeeded);
    let replay = adapter.rotate(request).expect("rotation terminal replay");
    assert_eq!(replay.outcome, SecretRefOutcome::Succeeded);
    assert!(replay.new_refresh_secret_ref.is_some());
    assert_eq!(provider.rotate_calls(), 1);
    assert_eq!(provider.status_calls(), 1);
}

#[test]
fn verified_transient_replays_until_explicit_retry() {
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
    assert_eq!(adapter.refresh(request.clone()).expect("local replay"), first);
    assert_eq!(provider.refresh_calls(), 1);
    assert_eq!(
        adapter.status_by_operation_key(status_request(&request)),
        Err(B3AdapterError::RetryRequired)
    );
    assert_eq!(provider.status_calls(), 0);

    let retried = adapter.retry_refresh(request).expect("explicit retry");
    assert_eq!(retried.outcome, SecretRefOutcome::Succeeded);
    assert_eq!(provider.refresh_calls(), 2);
}

#[test]
fn zero_retry_budget_enters_manual_required() {
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
    assert_eq!(provider.refresh_calls(), 1);
    assert_eq!(provider.status_calls(), 0);
}

#[test]
fn schema_error_after_provider_entry_is_indeterminate() {
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
    assert_eq!(provider.refresh_calls(), 1);
}

#[test]
fn unknown_lookup_updates_replay_then_allows_explicit_retry() {
    let bytes = b"lookup-secret";
    let reference = secret_ref("lookup", bytes);
    let request = refresh_request(reference.clone(), "lookup");
    let lookup = status_request(&request);
    let provider = ScriptedProvider::default();
    provider.push_refresh(Err(ProviderAdapterError::Timeout));
    provider.push_status(Ok(unknown_status(&lookup, 1)));
    provider.push_refresh(Ok(successful_refresh(&request)));
    let mut adapter =
        ProcessBoundSecretRefAdapter::new(backend(&reference, bytes), provider.clone());

    adapter.refresh(request.clone()).expect("initial unknown");
    let status = adapter
        .status_by_operation_key(lookup)
        .expect("unknown status");
    assert_eq!(status.outcome, SecretRefOutcome::Indeterminate);
    assert_eq!(
        adapter.operation_state(&request.operation_id),
        Some(SecretRefState::Backoff)
    );
    let replay = adapter.refresh(request.clone()).expect("updated replay");
    assert_eq!(replay.outcome, SecretRefOutcome::Indeterminate);
    assert_eq!(replay.response_id, status.response_id);
    assert_eq!(replay.response_digest, status.response_digest);

    let retried = adapter.retry_refresh(request).expect("retry after lookup");
    assert_eq!(retried.outcome, SecretRefOutcome::Succeeded);
    assert_eq!(provider.refresh_calls(), 2);
    assert_eq!(provider.status_calls(), 1);
}

#[test]
fn repeated_status_revision_is_rejected_without_state_mutation() {
    let bytes = b"revision-secret";
    let reference = secret_ref("revision", bytes);
    let request = refresh_request(reference.clone(), "revision");
    let lookup = status_request(&request);
    let provider = ScriptedProvider::default();
    provider.push_refresh(Err(ProviderAdapterError::Timeout));
    provider.push_status(Ok(unknown_status(&lookup, 1)));
    provider.push_refresh(Err(ProviderAdapterError::Timeout));
    provider.push_status(Ok(unknown_status(&lookup, 1)));
    let mut adapter =
        ProcessBoundSecretRefAdapter::new(backend(&reference, bytes), provider.clone());

    adapter.refresh(request.clone()).expect("initial unknown");
    adapter
        .status_by_operation_key(lookup.clone())
        .expect("first status");
    adapter
        .retry_refresh(request.clone())
        .expect("second provider unknown");
    assert_eq!(
        adapter.status_by_operation_key(lookup),
        Err(B3AdapterError::StatusRevisionConflict)
    );
    assert_eq!(
        adapter.operation_state(&request.operation_id),
        Some(SecretRefState::Indeterminate)
    );
    assert_eq!(provider.refresh_calls(), 2);
    assert_eq!(provider.status_calls(), 2);
}

#[test]
fn retry_budget_exhausts_after_second_verified_failure() {
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
    assert_eq!(provider.refresh_calls(), 2);
}
