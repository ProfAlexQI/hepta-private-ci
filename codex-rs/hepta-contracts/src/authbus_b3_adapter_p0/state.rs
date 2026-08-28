impl<B, P> ProcessBoundSecretRefAdapter<B, P>
where
    B: SecretRefBackend,
    P: SecretRefProvider,
{
    fn validate_or_mark_unknown<F>(
        &mut self,
        operation_id: &str,
        validation: F,
    ) -> Result<(), B3AdapterError>
    where
        F: FnOnce() -> Result<(), AuthBusContractError>,
    {
        if let Err(error) = validation() {
            self.mark_response_unknown(operation_id)?;
            return Err(B3AdapterError::ProviderResponseInvalid(error.to_string()));
        }
        Ok(())
    }

    fn validate_status_revision(
        &self,
        operation_id: &str,
        revision: u64,
    ) -> Result<(), B3AdapterError> {
        let entry = self
            .operations
            .get(operation_id)
            .ok_or(B3AdapterError::OperationNotFound)?;
        if entry
            .last_status_revision
            .is_some_and(|previous| revision <= previous)
        {
            return Err(B3AdapterError::StatusRevisionConflict);
        }
        Ok(())
    }

    fn build_terminal_replay(
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
            transition(&mut entry.record, SecretRefEvent::Lookup)?;
        } else if entry.record.state != SecretRefState::Reconciling {
            return Err(B3AdapterError::InvalidState(format!(
                "status lookup cannot advance {:?}",
                entry.record.state
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
        transition_with_fence(&mut entry.record, event, &fence)?;
        let claim = entry
            .record
            .state
            .is_terminal()
            .then(|| entry.claim_key.clone());
        if let Some(claim) = claim {
            self.claims.remove(&claim);
        }
        Ok(())
    }

    fn apply_request_event<R>(
        &mut self,
        operation_id: &str,
        request: &R,
        event: SecretRefEvent,
    ) -> Result<(), B3AdapterError>
    where
        R: RequestFence,
    {
        let fence = request
            .callback_fence()
            .map_err(|error| B3AdapterError::InvalidRequest(error.to_string()))?;
        let entry = self
            .operations
            .get_mut(operation_id)
            .ok_or(B3AdapterError::OperationNotFound)?;
        if event.requires_current_fence() {
            transition_with_fence(&mut entry.record, event, &fence)?;
        } else {
            transition(&mut entry.record, event)?;
        }
        let claim = entry
            .record
            .state
            .is_terminal()
            .then(|| entry.claim_key.clone());
        if let Some(claim) = claim {
            self.claims.remove(&claim);
        }
        Ok(())
    }

    fn mark_retry_exhausted(&mut self, operation_id: &str) -> Result<(), B3AdapterError> {
        let entry = self
            .operations
            .get_mut(operation_id)
            .ok_or(B3AdapterError::OperationNotFound)?;
        if entry.record.state == SecretRefState::TransientFailure
            && !retry_available(&entry.record)
        {
            transition(&mut entry.record, SecretRefEvent::RetryBudgetExhausted)?;
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
            transition_with_fence(
                &mut entry.record,
                SecretRefEvent::ResponseUnknown,
                &fence,
            )?;
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
