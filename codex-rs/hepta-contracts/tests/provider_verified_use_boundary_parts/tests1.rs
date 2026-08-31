#[tokio::test]
async fn dual_claims_and_witness_pair_are_durable_before_provider_dispatch() {
    let payload = b"exact-final-provider-payload";
    let intent = intent(payload);
    let attempted = Arc::new(AtomicBool::new(false));
    let clock = TestClock::at(110);
    let store = InMemoryClaimStore::default();
    let mut coordinator = coordinator(&intent, Arc::clone(&attempted), &clock);
    let now = || clock.read();
    let claim = |kind, operation_scope, claim, token, request, claimed_at| {
        store.claim(
            kind,
            operation_scope,
            claim,
            token,
            request,
            claimed_at,
        )
    };
    let mut persisted = Vec::new();

    let (receipt, provider_witness, effect_witness) = coordinator
        .dispatch_once_with_payload(
            intent,
            payload,
            revision(),
            revision(),
            PhysicalUseWindow::new(180).expect("window must build"),
            &now,
            &claim,
            |provider, effect| {
                assert!(!attempted.load(Ordering::SeqCst));
                assert_witness_pair(provider, effect);
                persisted.extend([provider.kind(), effect.kind()]);
                Ok(())
            },
        )
        .await
        .expect("verified dispatch must settle");

    assert!(attempted.load(Ordering::SeqCst));
    assert_eq!(receipt.operation_phase, OperationPhase::Acknowledged);
    assert!(receipt.provider.physical_dispatch_attempted);
    assert_witness_pair(&provider_witness, &effect_witness);
    assert_eq!(
        persisted,
        vec![
            PhysicalCapabilityKind::ProviderDispatch,
            PhysicalCapabilityKind::ExternalEffect,
        ]
    );
    assert_eq!(store.calls.load(Ordering::SeqCst), 2);
    assert_eq!(store.count(), 2);
}

#[tokio::test]
async fn witness_pair_persistence_failure_prevents_physical_send_and_replay() {
    let payload = b"exact-final-provider-payload";
    let intent = intent(payload);
    let attempted = Arc::new(AtomicBool::new(false));
    let clock = TestClock::at(110);
    let store = InMemoryClaimStore::default();
    let mut coordinator = coordinator(&intent, Arc::clone(&attempted), &clock);
    let now = || clock.read();
    let claim = |kind, operation_scope, claim, token, request, claimed_at| {
        store.claim(
            kind,
            operation_scope,
            claim,
            token,
            request,
            claimed_at,
        )
    };

    let result = coordinator
        .dispatch_once_with_payload(
            intent.clone(),
            payload,
            revision(),
            revision(),
            PhysicalUseWindow::new(180).expect("window must build"),
            &now,
            &claim,
            |_provider, _effect| Err("witness transaction unavailable".to_string()),
        )
        .await;

    assert!(matches!(result, Err(ProviderOperationError::Authority(_))));
    assert!(!attempted.load(Ordering::SeqCst));
    assert_eq!(coordinator.operation().phase, OperationPhase::OutboxPending);
    assert_eq!(store.count(), 2, "both pre-send claims remain durable");

    let replay = coordinator
        .dispatch_once_with_payload(
            intent,
            payload,
            revision(),
            revision(),
            PhysicalUseWindow::new(180).expect("window must build"),
            &now,
            &claim,
            |_provider, _effect| Ok(()),
        )
        .await;
    assert!(matches!(replay, Err(ProviderOperationError::Authority(_))));
    assert!(!attempted.load(Ordering::SeqCst));
}

