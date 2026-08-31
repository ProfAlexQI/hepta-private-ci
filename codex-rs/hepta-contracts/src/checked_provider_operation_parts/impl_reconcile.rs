impl<A, V> ProviderOperationCoordinator<A, V>
where
    A: ProviderEffectAdapter,
    V: PhysicalUseVerifier,
{
    /// Provider reconciliation is lookup-only. It revalidates both broad
    /// capabilities at trusted time but never mints a new physical-use token or
    /// retries a provider send.
    pub async fn reconcile<N>(
        &mut self,
        intent: &ProviderEffectIntent,
        now_unix_seconds: &N,
    ) -> Result<ProviderEffectState, provider_operation::ProviderOperationError>
    where
        N: Fn() -> Result<u64, String> + Sync + ?Sized,
    {
        let clock = ClosureClock(now_unix_seconds);
        let observed_at_unix_seconds = trusted_observed_at(&clock)?;
        self.verify_now(observed_at_unix_seconds)?;
        self.inner.reconcile(intent, observed_at_unix_seconds).await
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
