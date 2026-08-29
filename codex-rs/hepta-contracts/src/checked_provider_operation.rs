//! Final-payload, revocation-checked provider operation boundary.
//!
//! The lower-level provider coordinator remains private to this crate. Product
//! callers can only obtain this wrapper, which owns distinct provider-dispatch
//! and external-effect capabilities. Every physical dispatch verifies both
//! capabilities against the final wire payload, consumes both one-use B0
//! tokens, durably hands the resulting witnesses to the caller-supplied sink,
//! and only then crosses the adapter boundary.

use crate::AuthorityError;
use crate::Authorized;
use crate::ExternalEffectCapability;
use crate::PhysicalCapabilityKind;
use crate::PhysicalUseFinalCheck;
use crate::PhysicalUseVerificationRequest;
use crate::PhysicalUseVerifier;
use crate::PhysicalUseWindow;
use crate::ProviderDispatchCapability;
use crate::ProviderEffectAdapter;
use crate::ProviderEffectCoordinator;
use crate::ProviderEffectIntent;
use crate::ProviderEffectState;
use crate::RevocationRevision;
use crate::RuntimeAuthorityContext;
use crate::Sha256Digest;
use crate::VerifiedUseError;
use crate::VerifiedUseWitness;
use crate::provider_operation;
use crate::verify_capability_use;
use crate::verify_physical_capability_use;

pub struct ProviderOperationCoordinator<A, V>
where
    A: ProviderEffectAdapter,
    V: PhysicalUseVerifier,
{
    inner: provider_operation::ProviderOperationCoordinator<A>,
    provider_dispatch: Authorized<ProviderDispatchCapability>,
    external_effect: Authorized<ExternalEffectCapability>,
    provider_runtime_authority: RuntimeAuthorityContext,
    effect_runtime_authority: RuntimeAuthorityContext,
    verifier: V,
}

impl<A, V> ProviderOperationCoordinator<A, V>
where
    A: ProviderEffectAdapter,
    V: PhysicalUseVerifier,
{
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        adapter: A,
        operation: provider_operation::ProviderOperationRecord,
        provider_dispatch: Authorized<ProviderDispatchCapability>,
        external_effect: Authorized<ExternalEffectCapability>,
        observed_at_unix_seconds: u64,
        verifier: V,
    ) -> Result<Self, provider_operation::ProviderOperationError> {
        let provider_runtime_authority = runtime_context(&provider_dispatch)?;
        let effect_runtime_authority = runtime_context(&external_effect)?;
        validate_capability_pair(
            &operation,
            &provider_dispatch,
            &external_effect,
            observed_at_unix_seconds,
        )?;
        verify_capability_use(
            &provider_dispatch,
            &provider_runtime_authority,
            observed_at_unix_seconds,
            &verifier,
        )?;
        verify_capability_use(
            &external_effect,
            &effect_runtime_authority,
            observed_at_unix_seconds,
            &verifier,
        )?;
        let inner = provider_operation::ProviderOperationCoordinator::new(
            adapter,
            operation,
            external_effect.clone(),
            observed_at_unix_seconds,
        )?;
        Ok(Self {
            inner,
            provider_dispatch,
            external_effect,
            provider_runtime_authority,
            effect_runtime_authority,
            verifier,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn with_provider(
        provider: ProviderEffectCoordinator<A>,
        operation: provider_operation::ProviderOperationRecord,
        provider_dispatch: Authorized<ProviderDispatchCapability>,
        external_effect: Authorized<ExternalEffectCapability>,
        observed_at_unix_seconds: u64,
        verifier: V,
    ) -> Result<Self, provider_operation::ProviderOperationError> {
        let provider_runtime_authority = runtime_context(&provider_dispatch)?;
        let effect_runtime_authority = runtime_context(&external_effect)?;
        validate_capability_pair(
            &operation,
            &provider_dispatch,
            &external_effect,
            observed_at_unix_seconds,
        )?;
        verify_capability_use(
            &provider_dispatch,
            &provider_runtime_authority,
            observed_at_unix_seconds,
            &verifier,
        )?;
        verify_capability_use(
            &external_effect,
            &effect_runtime_authority,
            observed_at_unix_seconds,
            &verifier,
        )?;
        let inner = provider_operation::ProviderOperationCoordinator::with_provider(
            provider,
            operation,
            external_effect.clone(),
            observed_at_unix_seconds,
        )?;
        Ok(Self {
            inner,
            provider_dispatch,
            external_effect,
            provider_runtime_authority,
            effect_runtime_authority,
            verifier,
        })
    }

    pub fn operation(&self) -> &provider_operation::ProviderOperationRecord {
        self.inner.operation()
    }

    pub fn provider(&self) -> &ProviderEffectCoordinator<A> {
        self.inner.provider()
    }

    pub fn provider_runtime_authority(&self) -> &RuntimeAuthorityContext {
        &self.provider_runtime_authority
    }

    pub fn effect_runtime_authority(&self) -> &RuntimeAuthorityContext {
        &self.effect_runtime_authority
    }

    pub fn verifier(&self) -> &V {
        &self.verifier
    }

    /// Crosses the provider dispatch boundary for one exact final wire payload.
    ///
    /// The witness sink is invoked for both independent capabilities before
    /// the raw adapter can run. A sink failure is fail-closed and leaves the
    /// operation unclaimed. Timeout, transport loss, and malformed ACK handling
    /// remain owned by the inner no-blind-retry coordinator and therefore enter
    /// `Indeterminate` rather than redispatching.
    #[allow(clippy::too_many_arguments)]
    pub async fn dispatch_once_with_payload<P>(
        &mut self,
        intent: ProviderEffectIntent,
        wire_payload: &[u8],
        provider_revocation_revision: RevocationRevision,
        effect_revocation_revision: RevocationRevision,
        window: PhysicalUseWindow,
        crossed_at_unix_seconds: u64,
        mut persist_witness: P,
    ) -> Result<
        (
            provider_operation::ProviderOperationDispatchReceipt,
            VerifiedUseWitness,
            VerifiedUseWitness,
        ),
        provider_operation::ProviderOperationError,
    >
    where
        P: FnMut(&VerifiedUseWitness) -> Result<(), String>,
    {
        intent.validate()?;
        let final_payload_sha256 = Sha256Digest::for_bytes(wire_payload);
        if final_payload_sha256 != intent.payload_sha256 {
            return Err(provider_operation::ProviderOperationError::BindingDrift);
        }
        self.verify_now(window.verified_at_unix_seconds())?;

        let operation_id = &self.inner.operation().envelope.binding.operation_id;
        let provider_token = verify_physical_capability_use(
            &self.provider_dispatch,
            PhysicalUseVerificationRequest::new(
                PhysicalCapabilityKind::ProviderDispatch,
                operation_id,
                &final_payload_sha256,
                &self.provider_runtime_authority,
                provider_revocation_revision,
                window,
            ),
            &self.verifier,
        )
        .map_err(verified_use_error)?;
        let effect_token = verify_physical_capability_use(
            &self.external_effect,
            PhysicalUseVerificationRequest::new(
                PhysicalCapabilityKind::ExternalEffect,
                operation_id,
                &final_payload_sha256,
                &self.effect_runtime_authority,
                effect_revocation_revision,
                window,
            ),
            &self.verifier,
        )
        .map_err(verified_use_error)?;

        let provider_witness = provider_token
            .consume(PhysicalUseFinalCheck::new(
                PhysicalCapabilityKind::ProviderDispatch,
                operation_id,
                &final_payload_sha256,
                &self.provider_runtime_authority,
                provider_revocation_revision,
                crossed_at_unix_seconds,
            ))
            .map_err(verified_use_error)?;
        let effect_witness = effect_token
            .consume(PhysicalUseFinalCheck::new(
                PhysicalCapabilityKind::ExternalEffect,
                operation_id,
                &final_payload_sha256,
                &self.effect_runtime_authority,
                effect_revocation_revision,
                crossed_at_unix_seconds,
            ))
            .map_err(verified_use_error)?;

        persist_witness(&provider_witness).map_err(witness_persistence_error)?;
        persist_witness(&effect_witness).map_err(witness_persistence_error)?;

        let receipt = self
            .inner
            .dispatch_once_with_payload(intent, wire_payload, crossed_at_unix_seconds)
            .await?;
        Ok((receipt, provider_witness, effect_witness))
    }

    /// Provider reconciliation is lookup-only. It revalidates both broad
    /// capabilities but never mints a new dispatch token or retries a send.
    pub async fn reconcile(
        &mut self,
        intent: &ProviderEffectIntent,
        observed_at_unix_seconds: u64,
    ) -> Result<ProviderEffectState, provider_operation::ProviderOperationError> {
        self.verify_now(observed_at_unix_seconds)?;
        self.inner.reconcile(intent, observed_at_unix_seconds).await
    }

    pub fn into_parts(
        self,
    ) -> (
        ProviderEffectCoordinator<A>,
        provider_operation::ProviderOperationRecord,
        Authorized<ProviderDispatchCapability>,
        Authorized<ExternalEffectCapability>,
        RuntimeAuthorityContext,
        RuntimeAuthorityContext,
        V,
    ) {
        let (provider, operation, _inner_capability) = self.inner.into_parts();
        (
            provider,
            operation,
            self.provider_dispatch,
            self.external_effect,
            self.provider_runtime_authority,
            self.effect_runtime_authority,
            self.verifier,
        )
    }

    fn verify_now(
        &self,
        observed_at_unix_seconds: u64,
    ) -> Result<(), provider_operation::ProviderOperationError> {
        verify_capability_use(
            &self.provider_dispatch,
            &self.provider_runtime_authority,
            observed_at_unix_seconds,
            &self.verifier,
        )?;
        verify_capability_use(
            &self.external_effect,
            &self.effect_runtime_authority,
            observed_at_unix_seconds,
            &self.verifier,
        )
        .map_err(Into::into)
    }
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
        .and_then(|binding| RuntimeAuthorityContext::from_external_binding(binding).map_err(Into::into))
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
