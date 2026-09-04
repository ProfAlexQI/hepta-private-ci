use super::*;

fn id(value: &str) -> StableId {
    let Ok(value) = StableId::new(value) else {
        panic!("test identifier must be valid");
    };
    value
}

fn digest(value: &[u8]) -> Digest32 {
    Digest32::of_bytes(value)
}

fn factor(source: FactorSource) -> PromptFactor {
    PromptFactor {
        factor_id: id("factor:1"),
        proposer_id: id("proposer:1"),
        semantic_version: id("v1"),
        content_digest: digest(b"factor"),
        source,
        lifecycle: Lifecycle::Draft,
    }
}

fn registry() -> PromptRegistry {
    let Ok(registry) = PromptRegistry::new(32) else {
        panic!("test registry must initialize");
    };
    registry
}

#[test]
fn external_material_cannot_admit_itself() {
    let mut registry = registry();
    assert!(
        registry
            .register_factor(factor(FactorSource::ExternalUntrusted))
            .is_ok()
    );
    assert_eq!(
        registry.admit_factor(&id("factor:1"), &id("reviewer:1"), digest(b"evidence")),
        Err(Error::ExternalSelfAdmission)
    );
}

#[test]
fn independent_admission_enables_realization_registration() {
    let mut registry = registry();
    assert!(
        registry
            .register_factor(factor(FactorSource::GovernedInternal))
            .is_ok()
    );
    let Ok(receipt) =
        registry.admit_factor(&id("factor:1"), &id("reviewer:1"), digest(b"evidence"))
    else {
        panic!("independent admission must succeed");
    };
    assert!(!receipt.authority.grants_any());

    let realization = PromptRealization {
        realization_id: id("realization:1"),
        factor_id: id("factor:1"),
        model_digest: digest(b"model"),
        tokenizer_digest: digest(b"tokenizer"),
        content_digest: digest(b"realization"),
        active: true,
    };
    assert!(registry.register_realization(realization).is_ok());
}

#[test]
fn proposer_cannot_self_review() {
    let mut registry = registry();
    assert!(
        registry
            .register_factor(factor(FactorSource::GovernedInternal))
            .is_ok()
    );
    assert_eq!(
        registry.admit_factor(&id("factor:1"), &id("proposer:1"), digest(b"evidence")),
        Err(Error::SelfReview)
    );
}

#[test]
fn revocation_cascades_and_is_terminal() {
    let mut registry = registry();
    assert!(
        registry
            .register_factor(factor(FactorSource::GovernedInternal))
            .is_ok()
    );
    assert!(
        registry
            .admit_factor(&id("factor:1"), &id("reviewer:1"), digest(b"evidence"))
            .is_ok()
    );
    let realization = PromptRealization {
        realization_id: id("realization:1"),
        factor_id: id("factor:1"),
        model_digest: digest(b"model"),
        tokenizer_digest: digest(b"tokenizer"),
        content_digest: digest(b"realization"),
        active: true,
    };
    assert!(registry.register_realization(realization).is_ok());
    assert!(registry.revoke_factor(&id("factor:1")).is_ok());
    let Some(record) = registry.realization(&id("realization:1")) else {
        panic!("realization must remain interpretable");
    };
    assert!(!record.active);
    assert_eq!(
        registry.admit_factor(&id("factor:1"), &id("reviewer:2"), digest(b"evidence:2")),
        Err(Error::InvalidTransition)
    );
}

#[test]
fn conflicting_identity_is_rejected() {
    let mut registry = registry();
    let value = factor(FactorSource::GovernedInternal);
    assert!(registry.register_factor(value.clone()).is_ok());
    let mut drifted = value;
    drifted.content_digest = digest(b"drift");
    assert_eq!(
        registry.register_factor(drifted),
        Err(Error::FactorConflict("factor:1".to_string()))
    );
}
