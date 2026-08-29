//! Public revocation-checked provider operation boundary.
//!
//! The lower-level provider coordinator remains private to this crate. Product
//! callers can only obtain this wrapper, which owns the current runtime
//! authority context and invokes a `CapabilityUseVerifier` before every
//! dispatch and every provider-status reconciliation.

use crate::Authorized;
use crate::CapabilityUseVerifier;
use crate::ExternalEffectCapability;
use crate::ProviderEffectAdapter;
use crate::ProviderEffectCoordinator;
use crate::ProviderEffectIntent;
use crate::ProviderEffectState;
use crate::RuntimeAuthorityContext;
use crate::provider_operation;
use crate::verify_capability_use;

pub struct ProviderOperationCoordinator<A, V>
where
    A: ProviderEffectAdapter,
    V: CapabilityUseVerifier,
{
    inner: provider_operation::ProviderOperationCoordinator<A>,
    external_effect: Authorized<ExternalEffectCapability>,
    runtime_authority: RuntimeAuthorityContext,
    verifier: V,
}

impl<A, V> ProviderOperationCoordinator<A, V>
where
    A: ProviderEffectAdapter,
    V: CapabilityUseVerifier,
{
    pub fn new(
        adapter: A,
        operation: provider_operation::ProviderOperationRecord,
        external_effect: Authorized<ExternalEffectCapability>,
        observed_at_unix_seconds: u64,
        verifier: V,
    ) -> Result<Self, provider_operation::ProviderOperationError> {
        let runtime_authority = external_effect
            .external_lease_binding()
            .ok_or(provider_operation::ProviderOperationError::ExternalAuthorityRequired)
            .and_then(|binding| {
                RuntimeAuthorityContext::from_external_binding(binding).map_err(Into::into)
            })?;
        verify_capability_use(
            &external_effect,
            &runtime_authority,
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
            external_effect,
            runtime_authority,
            verifier,
        })
    }

    pub fn with_provider(
        provider: ProviderEffectCoordinator<A>,
        operation: provider_operation::ProviderOperationRecord,
        external_effect: Authorized<ExternalEffectCapability>,
        observed_at_unix_seconds: u64,
        verifier: V,
    ) -> Result<Self, provider_operation::ProviderOperationError> {
        let runtime_authority = external_effect
            .external_lease_binding()
            .ok_or(provider_operation::ProviderOperationError::ExternalAuthorityRequired)
            .and_then(|binding| {
                RuntimeAuthorityContext::from_external_binding(binding).map_err(Into::into)
            })?;
        verify_capability_use(
            &external_effect,
            &runtime_authority,
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
            external_effect,
            runtime_authority,
            verifier,
        })
    }

    pub fn operation(&self) -> &provider_operation::ProviderOperationRecord {
        self.inner.operation()
    }

    pub fn provider(&self) -> &ProviderEffectCoordinator<A> {
        self.inner.provider()
    }

    pub fn runtime_authority(&self) -> &RuntimeAuthorityContext {
        &self.runtime_authority
    }

    pub fn verifier(&self) -> &V {
        &self.verifier
    }

    pub async fn dispatch_once(
        &mut self,
        intent: ProviderEffectIntent,
        observed_at_unix_seconds: u64,
    ) -> Result<provider_operation::ProviderOperationDispatchReceipt, provider_operation::ProviderOperationError>
    {
        self.verify_now(observed_at_unix_seconds)?;
        self.inner
            .dispatch_once(intent, observed_at_unix_seconds)
            .await
    }

    pub async fn dispatch_once_with_payload(
        &mut self,
        intent: ProviderEffectIntent,
        wire_payload: &[u8],
        observed_at_unix_seconds: u64,
    ) -> Result<provider_operation::ProviderOperationDispatchReceipt, provider_operation::ProviderOperationError>
    {
        self.verify_now(observed_at_unix_seconds)?;
        self.inner
            .dispatch_once_with_payload(intent, wire_payload, observed_at_unix_seconds)
            .await
    }

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
        Authorized<ExternalEffectCapability>,
        RuntimeAuthorityContext,
        V,
    ) {
        let (provider, operation, _inner_capability) = self.inner.into_parts();
        (
            provider,
            operation,
            self.external_effect,
            self.runtime_authority,
            self.verifier,
        )
    }

    fn verify_now(
        &self,
        observed_at_unix_seconds: u64,
    ) -> Result<(), provider_operation::ProviderOperationError> {
        verify_capability_use(
            &self.external_effect,
            &self.runtime_authority,
            observed_at_unix_seconds,
            &self.verifier,
        )
        .map_err(Into::into)
    }
}
