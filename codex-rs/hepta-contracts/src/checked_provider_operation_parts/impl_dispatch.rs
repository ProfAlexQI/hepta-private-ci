impl<A, V> ProviderOperationCoordinator<A, V>
where
    A: ProviderEffectAdapter,
    V: PhysicalUseVerifier,
{
    /// Crosses the provider dispatch boundary for one exact final wire payload.
    ///
    /// The B0 claim store is invoked once for each distinct capability kind.
    /// The pair-persistence callback must durably store both witnesses in one
    /// caller-owned transaction before this method invokes the raw adapter.
    /// The witnesses prove committed pre-crossing claims, not provider success.
    #[allow(clippy::too_many_arguments)]
    pub async fn dispatch_once_with_payload<N, C, P>(
        &mut self,
        intent: ProviderEffectIntent,
        wire_payload: &[u8],
        provider_revocation_revision: RevocationRevision,
        effect_revocation_revision: RevocationRevision,
        window: PhysicalUseWindow,
        now_unix_seconds: &N,
        claim_once: &C,
        persist_witness_pair: P,
    ) -> Result<
        (
            provider_operation::ProviderOperationDispatchReceipt,
            VerifiedUseWitness,
            VerifiedUseWitness,
        ),
        provider_operation::ProviderOperationError,
    >
    where
        N: Fn() -> Result<u64, String> + Sync + ?Sized,
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
        P: FnOnce(&VerifiedUseWitness, &VerifiedUseWitness) -> Result<(), String>,
    {
        let clock = ClosureClock(now_unix_seconds);
        let claim_store = ClosureClaimStore(claim_once);
        intent.validate()?;
        self.inner.operation().validate_for(&intent)?;
        if self.inner.operation().phase != OperationPhase::OutboxPending {
            return Err(provider_operation::ProviderOperationError::DeliveryAlreadyClaimed);
        }

        let final_payload_sha256 = Sha256Digest::for_bytes(wire_payload);
        if final_payload_sha256 != intent.payload_sha256 {
            return Err(provider_operation::ProviderOperationError::BindingDrift);
        }

        let operation_id = self
            .inner
            .operation()
            .envelope
            .binding
            .operation_id
            .clone();
        let provider_token = verify_physical_capability_use(
            &self.provider_dispatch,
            PhysicalCapabilityKind::ProviderDispatch,
            &operation_id,
            &final_payload_sha256,
            &self.provider_runtime_authority,
            provider_revocation_revision,
            window,
            &self.verifier,
            &clock,
        )
        .map_err(verified_use_error)?;
        let effect_token = verify_physical_capability_use(
            &self.external_effect,
            PhysicalCapabilityKind::ExternalEffect,
            &operation_id,
            &final_payload_sha256,
            &self.effect_runtime_authority,
            effect_revocation_revision,
            window,
            &self.verifier,
            &clock,
        )
        .map_err(verified_use_error)?;

        let provider_permit = provider_token
            .consume_at_boundary(
                PhysicalUseFinalCheck::new(
                    PhysicalCapabilityKind::ProviderDispatch,
                    &operation_id,
                    &final_payload_sha256,
                    &self.provider_runtime_authority,
                ),
                &self.verifier,
                &clock,
                &claim_store,
            )
            .map_err(verified_use_error)?;
        let effect_permit = effect_token
            .consume_at_boundary(
                PhysicalUseFinalCheck::new(
                    PhysicalCapabilityKind::ExternalEffect,
                    &operation_id,
                    &final_payload_sha256,
                    &self.effect_runtime_authority,
                ),
                &self.verifier,
                &clock,
                &claim_store,
            )
            .map_err(verified_use_error)?;

        let provider_witness = provider_permit.into_witness();
        let effect_witness = effect_permit.into_witness();
        provider_witness.validate().map_err(verified_use_error)?;
        effect_witness.validate().map_err(verified_use_error)?;
        persist_witness_pair(&provider_witness, &effect_witness)
            .map_err(witness_persistence_error)?;

        let crossed_at_unix_seconds = provider_witness
            .crossed_at_unix_seconds()
            .max(effect_witness.crossed_at_unix_seconds());
        let receipt = self
            .inner
            .dispatch_once_with_payload(intent, wire_payload, crossed_at_unix_seconds)
            .await?;
        Ok((receipt, provider_witness, effect_witness))
    }
}
