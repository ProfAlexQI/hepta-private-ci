impl<B, P> ProcessBoundSecretRefAdapter<B, P>
where
    B: SecretRefBackend,
    P: SecretRefProvider,
{
    fn finish_refresh_result(
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
            access_secret_ref: if outcome == SecretRefOutcome::Succeeded {
                result.access_secret_ref
            } else {
                None
            },
            refresh_secret_ref: if outcome == SecretRefOutcome::Succeeded {
                result.refresh_secret_ref
            } else {
                None
            },
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
        self.validate_or_mark_unknown(request.operation_id.as_str(), || {
            response.validate_against(request)
        })?;
        self.finish_refresh_response(request, response)
    }

    fn finish_refresh_backend_error(
        &mut self,
        request: &RefreshWithSecretRefRequest,
        error: SecretBackendError,
    ) -> Result<RefreshWithSecretRefResponse, B3AdapterError> {
        let status = backend_status(error);
        let response = refresh_error_response(
            request,
            SecretRefOutcome::TransientFailure,
            status,
            "backend-error",
        );
        response
            .validate_against(request)
            .map_err(|error| B3AdapterError::ProviderResponseInvalid(error.to_string()))?;
        self.apply_request_event(
            request.operation_id.as_str(),
            request,
            SecretRefEvent::TransientFailure,
        )?;
        self.finish_refresh_after_event(request.operation_id.as_str(), response)
    }

    fn finish_refresh_provider_error(
        &mut self,
        request: &RefreshWithSecretRefRequest,
        error: ProviderAdapterError,
    ) -> Result<RefreshWithSecretRefResponse, B3AdapterError> {
        let outcome = provider_error_outcome(error);
        let response = refresh_error_response(
            request,
            outcome,
            provider_error_status(error),
            "provider-error",
        );
        response
            .validate_against(request)
            .map_err(|validation| {
                B3AdapterError::ProviderResponseInvalid(validation.to_string())
            })?;
        self.apply_request_event(
            request.operation_id.as_str(),
            request,
            event_for_outcome(outcome),
        )?;
        self.finish_refresh_after_event(request.operation_id.as_str(), response)
    }

    fn finish_refresh_response(
        &mut self,
        request: &RefreshWithSecretRefRequest,
        response: RefreshWithSecretRefResponse,
    ) -> Result<RefreshWithSecretRefResponse, B3AdapterError> {
        self.apply_request_event(
            request.operation_id.as_str(),
            request,
            event_for_outcome(response.outcome),
        )?;
        self.finish_refresh_after_event(request.operation_id.as_str(), response)
    }

    fn finish_refresh_after_event(
        &mut self,
        operation_id: &str,
        response: RefreshWithSecretRefResponse,
    ) -> Result<RefreshWithSecretRefResponse, B3AdapterError> {
        self.mark_retry_exhausted(operation_id)?;
        self.set_response(operation_id, StoredResponse::Refresh(response.clone()))?;
        Ok(response)
    }

    fn finish_rotate_result(
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
            new_refresh_secret_ref: if outcome == SecretRefOutcome::Succeeded {
                result.new_refresh_secret_ref
            } else {
                None
            },
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
        self.validate_or_mark_unknown(request.operation_id.as_str(), || {
            response.validate_against(request)
        })?;
        self.finish_rotate_response(request, response)
    }

    fn finish_rotate_backend_error(
        &mut self,
        request: &RotateSecretRefRequest,
        error: SecretBackendError,
    ) -> Result<RotateSecretRefResponse, B3AdapterError> {
        let response = rotate_error_response(
            request,
            SecretRefOutcome::TransientFailure,
            backend_status(error),
            "backend-error",
        );
        response
            .validate_against(request)
            .map_err(|validation| {
                B3AdapterError::ProviderResponseInvalid(validation.to_string())
            })?;
        self.apply_request_event(
            request.operation_id.as_str(),
            request,
            SecretRefEvent::TransientFailure,
        )?;
        self.finish_rotate_after_event(request.operation_id.as_str(), response)
    }

    fn finish_rotate_provider_error(
        &mut self,
        request: &RotateSecretRefRequest,
        error: ProviderAdapterError,
    ) -> Result<RotateSecretRefResponse, B3AdapterError> {
        let outcome = provider_error_outcome(error);
        let response = rotate_error_response(
            request,
            outcome,
            provider_error_status(error),
            "provider-error",
        );
        response
            .validate_against(request)
            .map_err(|validation| {
                B3AdapterError::ProviderResponseInvalid(validation.to_string())
            })?;
        self.apply_request_event(
            request.operation_id.as_str(),
            request,
            event_for_outcome(outcome),
        )?;
        self.finish_rotate_after_event(request.operation_id.as_str(), response)
    }

    fn finish_rotate_response(
        &mut self,
        request: &RotateSecretRefRequest,
        response: RotateSecretRefResponse,
    ) -> Result<RotateSecretRefResponse, B3AdapterError> {
        self.apply_request_event(
            request.operation_id.as_str(),
            request,
            event_for_outcome(response.outcome),
        )?;
        self.finish_rotate_after_event(request.operation_id.as_str(), response)
    }

    fn finish_rotate_after_event(
        &mut self,
        operation_id: &str,
        response: RotateSecretRefResponse,
    ) -> Result<RotateSecretRefResponse, B3AdapterError> {
        self.mark_retry_exhausted(operation_id)?;
        self.set_response(operation_id, StoredResponse::Rotate(response.clone()))?;
        Ok(response)
    }
}
