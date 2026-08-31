impl InMemoryClaimStore {
    #[allow(clippy::too_many_arguments)]
    fn claim(
        &self,
        kind: PhysicalCapabilityKind,
        operation_scope_sha256: &Sha256Digest,
        claim_sha256: &Sha256Digest,
        token_sha256: &Sha256Digest,
        request_sha256: &Sha256Digest,
        claimed_at_unix_seconds: u64,
    ) -> Result<(u64, Sha256Digest), String> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        if self
            .reject_kind
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .as_ref()
            == Some(&kind)
        {
            return Err("selected claim partition unavailable".to_string());
        }
        if claimed_at_unix_seconds == 0 {
            return Err("claim time must be non-zero".to_string());
        }

        let scope = operation_scope_sha256.as_str().to_string();
        let claim = claim_sha256.as_str().to_string();
        let mut claims = self
            .claims
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if let Some(existing) = claims.get(&scope) {
            if existing == &claim {
                return Err("physical-use operation is already claimed".to_string());
            }
            return Err("physical-use operation payload conflict".to_string());
        }
        claims.insert(scope, claim);

        let claim_revision = self.next_revision.fetch_add(1, Ordering::SeqCst) + 1;
        Ok((
            claim_revision,
            Sha256Digest::for_bytes(
                format!(
                    "b1a-claim-{kind:?}-{claim_revision}-{}-{}",
                    token_sha256.as_str(),
                    request_sha256.as_str(),
                )
                .as_bytes(),
            ),
        ))
    }
}

fn capability<C>(grant: &[u8]) -> Authorized<C>
where
    C: AuthorityCapability,
{
    let binding = AuthorityLeaseBinding::new(
        agent(),
        Sha256Digest::for_bytes(grant),
        AUTHORITY_EPOCH,
        OWNER_EPOCH,
        GENERATION,
        Sha256Digest::for_bytes(b"b1a-provider-fence"),
        EXPIRES_AT,
    )
    .expect("binding must build");
    authorize_verified_capability::<C, _>(binding, &agent(), GENERATION, 100, &ExactVerifier)
        .expect("capability must verify")
}

#[derive(Clone)]
struct CompleteAdapter {
    attempted: Arc<AtomicBool>,
}

impl ProviderEffectAdapter for CompleteAdapter {
    fn capability(&self) -> ProviderEffectIdempotencyCapability {
        ProviderEffectIdempotencyCapability::KeyAndStatusLookup
    }

    fn dispatch<'a>(
        &'a self,
        _intent: &'a ProviderEffectIntent,
    ) -> ProviderEffectFuture<'a, ProviderEffectDispatch> {
        Box::pin(async {
            ProviderEffectDispatch::NotDispatched {
                reason_code: "verified_wire_payload_required".to_string(),
            }
        })
    }

    fn dispatch_with_payload<'a>(
        &'a self,
        intent: &'a ProviderEffectIntent,
        _wire_payload: &'a [u8],
    ) -> ProviderEffectFuture<'a, ProviderEffectDispatch> {
        self.attempted.store(true, Ordering::SeqCst);
        Box::pin(async move {
            ProviderEffectDispatch::Ack(ProviderEffectAck::new(
                intent.key.clone(),
                intent.payload_sha256.clone(),
                Sha256Digest::for_bytes(b"b1a-provider-operation"),
                ProviderEffectAckStatus::Completed,
            ))
        })
    }

    fn lookup<'a>(
        &'a self,
        _key: &'a ProviderEffectKey,
    ) -> ProviderEffectFuture<'a, ProviderEffectLookup> {
        Box::pin(async { ProviderEffectLookup::Unknown })
    }
}

fn coordinator(
    intent: &ProviderEffectIntent,
    attempted: Arc<AtomicBool>,
    clock: &TestClock,
) -> ProviderOperationCoordinator<CompleteAdapter, ExactVerifier> {
    ProviderOperationCoordinator::new(
        CompleteAdapter { attempted },
        operation(intent),
        capability::<ProviderDispatchCapability>(b"provider-dispatch-grant"),
        capability::<ExternalEffectCapability>(b"external-effect-grant"),
        ExactVerifier,
        &|| clock.read(),
    )
    .expect("checked coordinator must build")
}

fn revision() -> RevocationRevision {
    RevocationRevision::new(REVISION).expect("revision must build")
}

fn assert_witness_pair(provider: &VerifiedUseWitness, effect: &VerifiedUseWitness) {
    provider.validate().expect("provider witness must validate");
    effect.validate().expect("effect witness must validate");
    assert_eq!(provider.kind(), PhysicalCapabilityKind::ProviderDispatch);
    assert_eq!(effect.kind(), PhysicalCapabilityKind::ExternalEffect);
    assert_eq!(provider.operation_id(), effect.operation_id());
    assert_eq!(provider.final_payload_sha256(), effect.final_payload_sha256());
    assert!(!provider.effect_completed());
    assert!(!effect.effect_completed());
}

