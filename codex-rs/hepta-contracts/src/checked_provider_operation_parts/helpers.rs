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

fn claim_request_digest(request: &PhysicalUseClaimRequest<'_>) -> Sha256Digest {
    let mut bytes = Vec::new();
    frame(&mut bytes, b"hepta:b1a-provider-claim-request:v1");
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

fn trusted_observed_at<T>(
    clock: &T,
) -> Result<u64, provider_operation::ProviderOperationError>
where
    T: TrustedPhysicalClock + ?Sized,
{
    let observed_at_unix_seconds = clock.now_unix_seconds().map_err(|reason| {
        provider_operation::ProviderOperationError::Authority(AuthorityError::VerificationRejected(
            format!("provider trusted clock rejected read: {reason}"),
        ))
    })?;
    if observed_at_unix_seconds == 0 {
        return Err(provider_operation::ProviderOperationError::Authority(
            AuthorityError::VerificationRejected(
                "provider trusted clock returned zero".to_string(),
            ),
        ));
    }
    Ok(observed_at_unix_seconds)
}

fn runtime_context<C>(
    capability: &Authorized<C>,
) -> Result<RuntimeAuthorityContext, provider_operation::ProviderOperationError>
where
    C: crate::AuthorityCapability,
{
    capability
        .external_lease_binding()
        .ok_or(provider_operation::ProviderOperationError::ExternalAuthorityRequired)
        .and_then(|binding| {
            RuntimeAuthorityContext::from_external_binding(binding)
                .map_err(provider_operation::ProviderOperationError::from)
        })
}

fn validate_capability_pair(
    operation: &provider_operation::ProviderOperationRecord,
    provider_dispatch: &Authorized<ProviderDispatchCapability>,
    external_effect: &Authorized<ExternalEffectCapability>,
    observed_at_unix_seconds: u64,
) -> Result<(), provider_operation::ProviderOperationError> {
    let provider_binding = provider_dispatch
        .external_lease_binding()
        .ok_or(provider_operation::ProviderOperationError::ExternalAuthorityRequired)?;
    let effect_binding = external_effect
        .external_lease_binding()
        .ok_or(provider_operation::ProviderOperationError::ExternalAuthorityRequired)?;
    let operation_binding = &operation.envelope.binding;

    if provider_dispatch.subject_agent_id() != external_effect.subject_agent_id()
        || provider_dispatch.generation() != external_effect.generation()
        || provider_dispatch.subject_agent_id() != &operation_binding.source_owner_agent_id
        || provider_dispatch.generation() != operation_binding.generation
        || provider_binding.authority_epoch() != effect_binding.authority_epoch()
        || provider_binding.owner_epoch() != effect_binding.owner_epoch()
        || provider_binding.fencing_token_sha256() != effect_binding.fencing_token_sha256()
        || provider_binding.authority_epoch() != operation_binding.authority_epoch
        || provider_binding.owner_epoch() != operation_binding.owner_epoch
        || provider_binding.fencing_token_sha256() != &operation_binding.fencing_token_sha256
        || provider_binding.is_expired_at(observed_at_unix_seconds)
        || effect_binding.is_expired_at(observed_at_unix_seconds)
    {
        return Err(provider_operation::ProviderOperationError::ExternalAuthorityRequired);
    }
    Ok(())
}

fn verified_use_error(error: VerifiedUseError) -> provider_operation::ProviderOperationError {
    provider_operation::ProviderOperationError::Authority(AuthorityError::VerificationRejected(
        format!("provider physical-use verification failed: {error}"),
    ))
}

fn witness_persistence_error(reason: String) -> provider_operation::ProviderOperationError {
    provider_operation::ProviderOperationError::Authority(AuthorityError::VerificationRejected(
        format!("provider verified-use witness persistence failed: {reason}"),
    ))
}

fn frame(target: &mut Vec<u8>, value: &[u8]) {
    target.extend_from_slice(&(value.len() as u64).to_be_bytes());
    target.extend_from_slice(value);
}

