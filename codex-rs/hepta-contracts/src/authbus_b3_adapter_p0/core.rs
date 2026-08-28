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

    pub fn refresh(
        &mut self,
        request: RefreshWithSecretRefRequest,
    ) -> Result<RefreshWithSecretRefResponse, B3AdapterError> {
        self.validate_adapter_request(&request)?;
        if let Some(replay) = self.begin_or_replay(&request)? {
            return match replay {
                StoredResponse::Refresh(response) => Ok(response),
                StoredResponse::Rotate(_) => Err(B3AdapterError::Conflict),
            };
        }
        self.call_refresh(&request)
    }

    pub fn retry_refresh(
        &mut self,
        request: RefreshWithSecretRefRequest,
    ) -> Result<RefreshWithSecretRefResponse, B3AdapterError> {
        self.validate_adapter_request(&request)?;
        self.prepare_retry(&request)?;
        self.call_refresh(&request)
    }

    pub fn rotate(
        &mut self,
        request: RotateSecretRefRequest,
    ) -> Result<RotateSecretRefResponse, B3AdapterError> {
        self.validate_adapter_request(&request)?;
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
        self.validate_adapter_request(&request)?;
        self.prepare_retry(&request)?;
        self.call_rotate(&request)
    }

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
            validate_status_request(entry, &request)?;
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
        let response = build_status_response(&request, provider_result)?;
        self.validate_status_revision(request.operation_id.as_str(), response.status_revision)?;
        let replay = self.build_status_replay(request.operation_id.as_str(), &response)?;

        self.apply_status_transition(&request, response.outcome)?;
        let entry = self
            .operations
            .get_mut(request.operation_id.as_str())
            .ok_or(B3AdapterError::OperationNotFound)?;
        entry.last_status_revision = Some(response.status_revision);
        entry.response = Some(replay);
        Ok(response)
    }

    pub fn status_by_effect_key(
        &mut self,
        request: RefreshStatusByOperationKeyRequest,
    ) -> Result<RefreshStatusByOperationKeyResponse, B3AdapterError> {
        self.status_by_operation_key(request)
    }

    fn validate_adapter_request<R>(&self, request: &R) -> Result<(), B3AdapterError>
    where
        R: AdapterRequest,
    {
        request
            .validate_request()
            .map_err(|error| B3AdapterError::InvalidRequest(error.to_string()))
    }

    fn begin_or_replay<R>(&mut self, request: &R) -> Result<Option<StoredResponse>, B3AdapterError>
    where
        R: AdapterRequest,
    {
        let request_digest = request
            .digest_request()
            .map_err(|error| B3AdapterError::InvalidRequest(error.to_string()))?;
        if let Some(entry) = self.operations.get(request.operation_id()) {
            if entry.kind != request.kind() || entry.request_digest != request_digest {
                return Err(B3AdapterError::Conflict);
            }
            return entry
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
        if let Some(operation_id) = self.claims.get(&claim_key)
            && operation_id != request.operation_id()
        {
            return Err(B3AdapterError::SingleflightConflict);
        }

        let mut record = request
            .operation_record(self.retry_budget)
            .map_err(|error| B3AdapterError::InvalidRequest(error.to_string()))?;
        transition(&mut record, SecretRefEvent::Claim)?;
        transition(&mut record, SecretRefEvent::Dispatch)?;

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
            .digest_request()
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
                transition(&mut entry.record, SecretRefEvent::RetryBudgetExhausted)?;
            }
            return Err(B3AdapterError::RetryBudgetExhausted);
        }

        if entry.record.state == SecretRefState::TransientFailure {
            transition(&mut entry.record, SecretRefEvent::RetryScheduled)?;
        }
        transition(&mut entry.record, SecretRefEvent::ClaimAgain)?;
        transition(&mut entry.record, SecretRefEvent::Dispatch)?;
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
            Ok(result) => self.finish_refresh_result(request, result),
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
            Ok(result) => self.finish_rotate_result(request, result),
            Err(error) => self.finish_rotate_provider_error(request, error),
        }
    }
}
