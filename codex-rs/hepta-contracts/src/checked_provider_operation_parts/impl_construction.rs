impl<A, V> ProviderOperationCoordinator<A, V>
where
    A: ProviderEffectAdapter,
    V: PhysicalUseVerifier,
{
    /// Builds a checked provider boundary using trusted time for the initial
    /// broad-capability validation. Construction performs no physical effect.
    pub fn new<N>(
        adapter: A,
        operation: provider_operation::ProviderOperationRecord,
        provider_dispatch: Authorized<ProviderDispatchCapability>,
        external_effect: Authorized<ExternalEffectCapability>,
        verifier: V,
        now_unix_seconds: &N,
    ) -> Result<Self, provider_operation::ProviderOperationError>
    where
        N: Fn() -> Result<u64, String> + Sync + ?Sized,
    {
        let clock = ClosureClock(now_unix_seconds);
        let observed_at_unix_seconds = trusted_observed_at(&clock)?;
        Self::with_provider(
            ProviderEffectCoordinator::new(adapter),
            operation,
            provider_dispatch,
            external_effect,
            verifier,
            observed_at_unix_seconds,
        )
    }

    /// Restores a checked boundary around an existing provider journal. The
    /// caller still cannot dispatch until the final-payload B0 checks pass.
    pub fn with_existing_provider<N>(
        provider: ProviderEffectCoordinator<A>,
        operation: provider_operation::ProviderOperationRecord,
        provider_dispatch: Authorized<ProviderDispatchCapability>,
        external_effect: Authorized<ExternalEffectCapability>,
        verifier: V,
        now_unix_seconds: &N,
    ) -> Result<Self, provider_operation::ProviderOperationError>
    where
        N: Fn() -> Result<u64, String> + Sync + ?Sized,
    {
        let clock = ClosureClock(now_unix_seconds);
        let observed_at_unix_seconds = trusted_observed_at(&clock)?;
        Self::with_provider(
            provider,
            operation,
            provider_dispatch,
            external_effect,
            verifier,
            observed_at_unix_seconds,
        )
    }

    fn with_provider(
        provider: ProviderEffectCoordinator<A>,
        operation: provider_operation::ProviderOperationRecord,
        provider_dispatch: Authorized<ProviderDispatchCapability>,
        external_effect: Authorized<ExternalEffectCapability>,
        verifier: V,
        observed_at_unix_seconds: u64,
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
}
