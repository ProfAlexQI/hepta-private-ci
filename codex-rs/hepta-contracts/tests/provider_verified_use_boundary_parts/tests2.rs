#[tokio::test]
async fn second_capability_claim_failure_never_crosses_provider_boundary() {
    let payload = b"exact-final-provider-payload";
    let intent = intent(payload);
    let attempted = Arc::new(AtomicBool::new(false));
    let clock = TestClock::at(110);
    let store = InMemoryClaimStore::default();
    store.reject(PhysicalCapabilityKind::ExternalEffect);
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
    let persist_called = AtomicBool::new(false);

    let result = coordinator
        .dispatch_once_with_payload(
            intent,
            payload,
            revision(),
            revision(),
            PhysicalUseWindow::new(180).expect("window must build"),
            &now,
            &claim,
            |_provider, _effect| {
                persist_called.store(true, Ordering::SeqCst);
                Ok(())
            },
        )
        .await;

    assert!(matches!(result, Err(ProviderOperationError::Authority(_))));
    assert!(!attempted.load(Ordering::SeqCst));
    assert!(!persist_called.load(Ordering::SeqCst));
    assert_eq!(store.count(), 1, "the first claim is fail-closed evidence");
}

#[tokio::test]
async fn final_payload_drift_is_rejected_before_any_claim_or_send() {
    let bound_payload = b"bound-provider-payload";
    let wire_payload = b"different-wire-payload";
    let intent = intent(bound_payload);
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
            intent,
            wire_payload,
            revision(),
            revision(),
            PhysicalUseWindow::new(180).expect("window must build"),
            &now,
            &claim,
            |_provider, _effect| Ok(()),
        )
        .await;

    assert!(matches!(result, Err(ProviderOperationError::BindingDrift)));
    assert!(!attempted.load(Ordering::SeqCst));
    assert_eq!(store.calls.load(Ordering::SeqCst), 0);
    assert_eq!(store.count(), 0);
}

