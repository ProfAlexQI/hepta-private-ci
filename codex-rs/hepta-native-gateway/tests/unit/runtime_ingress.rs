use super::*;

#[test]
fn runtime_ingress_registry_covers_all_declared_routes_and_serializes_lifecycle_fields() {
    let registry = runtime_ingress_lifecycle_registry();
    assert_eq!(
        registry.len(),
        SPECIAL_INGRESS_LIFECYCLES.len() + CONTROL_UI_ROUTE_SPECS.len()
    );
    for route in CONTROL_UI_ROUTE_SPECS {
        let probe = route
            .pattern
            .replace("<action>", "probe")
            .replace("<id>", "probe")
            .replace("<query>", "probe")
            .replace("<task_id>", "probe")
            .replace("<cursor>", "probe");
        let registered = registry
            .iter()
            .find(|lifecycle| {
                lifecycle.method == route.method && lifecycle.path_pattern == route.pattern
            })
            .copied()
            .unwrap_or_else(|| panic!("unregistered route: {} {}", route.method, route.pattern));
        if QUARANTINED_TRANSITIVE_CANARY_EFFECT_PATHS.contains(&route.pattern) {
            assert_eq!(
                registered.effect_class,
                IngressEffectClass::QuarantinedLegacyMutation
            );
            assert_eq!(
                registered.authority_owner,
                IngressAuthorityOwner::UnassignedLegacyMutation
            );
            assert_eq!(
                validate_lifecycle(registered),
                Err("legacy_mutation_route_quarantined")
            );
            assert!(runtime_ingress_lifecycle(route.method, &probe).is_none());
        } else {
            let lifecycle = runtime_ingress_lifecycle(route.method, &probe).unwrap_or_else(|| {
                panic!("unclassified route: {} {}", route.method, route.pattern)
            });
            assert_eq!(lifecycle, registered);
            assert!(validate_lifecycle(lifecycle).is_ok());
        }
    }
    assert_eq!(
        runtime_ingress_lifecycle_registry_digest()
            .expect("serialize ingress lifecycle registry")
            .len(),
        64
    );
    let json = serde_json::to_value(&registry).expect("serialize lifecycle registry");
    let first = &json.as_array().expect("registry array")[0];
    for field in [
        "method",
        "path_pattern",
        "effect_class",
        "mutates_state",
        "external_effect",
        "credential_access",
        "authority_owner",
        "secret_access",
        "config_access",
        "network_access",
        "durable_intent",
        "effect_ack",
        "terminal_receipt",
        "default_enablement",
    ] {
        assert!(first.get(field).is_some(), "missing field {field}");
    }
}

#[test]
fn runtime_ingress_registry_rejects_unknown_get_effect_and_incomplete_authority() {
    assert!(runtime_ingress_lifecycle("GET", "/api/unregistered").is_none());
    assert!(runtime_ingress_lifecycle("POST", "/api/unregistered").is_none());

    let get_effect = IngressLifecycleSpec {
        method: "GET",
        path_pattern: "/api/invalid-get-effect",
        effect_class: IngressEffectClass::CredentialedNetworkRead,
        ..metadata_read("/api/invalid-get-effect")
    };
    assert_eq!(validate_lifecycle(get_effect), Err("get_effect_surface"));

    let missing_authority = IngressLifecycleSpec {
        method: "POST",
        path_pattern: "/api/invalid-effect-authority",
        effect_class: IngressEffectClass::RuntimeKernelLocalMutation,
        durable_intent: IngressLifecycleRequirement::RequiredBeforeEffect,
        effect_ack: IngressLifecycleRequirement::RequiredAfterEffect,
        terminal_receipt: IngressLifecycleRequirement::RequiredForTerminal,
        ..metadata_read("/api/invalid-effect-authority")
    };
    assert_eq!(
        validate_lifecycle(missing_authority),
        Err("effect_authority_owner_missing")
    );
}

#[test]
fn runtime_ingress_registry_quarantines_transitive_get_effects_and_keeps_live_surfaces_off() {
    let mut quarantined_get_effects = 0;
    for lifecycle in runtime_ingress_lifecycle_registry() {
        if lifecycle.method == "GET" {
            if lifecycle.effect_class == IngressEffectClass::QuarantinedLegacyMutation {
                quarantined_get_effects += 1;
                assert!(lifecycle.effect_class.performs_effect());
                assert_eq!(
                    lifecycle.default_enablement,
                    IngressDefaultEnablement::DisabledUnlessExplicitGate
                );
                assert!(
                    runtime_ingress_lifecycle(lifecycle.method, lifecycle.path_pattern).is_none()
                );
            } else {
                assert!(!lifecycle.effect_class.performs_effect());
            }
        }
    }
    assert_eq!(
        quarantined_get_effects,
        QUARANTINED_TRANSITIVE_CANARY_EFFECT_PATHS.len()
    );
    let telegram = runtime_ingress_lifecycle("POST", TELEGRAM_RECEIVE_ONCE_ENDPOINT)
        .expect("Telegram lifecycle");
    assert_eq!(
        telegram.default_enablement,
        IngressDefaultEnablement::DisabledUnlessExplicitGate
    );
    assert_eq!(
        telegram.authority_owner,
        IngressAuthorityOwner::RuntimeKernelTelegramRead
    );
    let mutation = runtime_ingress_lifecycle("POST", RUNTIME_MUTATION_CANARY_ENDPOINT)
        .expect("mutation lifecycle");
    assert_eq!(
        mutation.default_enablement,
        IngressDefaultEnablement::DisabledUnlessExplicitGate
    );
    let telegram_pipeline = runtime_ingress_lifecycle("POST", TELEGRAM_AUTHORITY_COMMIT_ENDPOINT)
        .expect("operator Telegram pipeline lifecycle");
    assert_eq!(
        telegram_pipeline.authority_owner,
        IngressAuthorityOwner::TelegramOperatorPipeline
    );
    assert_eq!(
        telegram_pipeline.durable_intent,
        IngressLifecycleRequirement::RequiredBeforeEffect
    );
    assert_eq!(
        telegram_pipeline.effect_ack,
        IngressLifecycleRequirement::RequiredAfterEffect
    );
    assert_eq!(
        telegram_pipeline.terminal_receipt,
        IngressLifecycleRequirement::RequiredForTerminal
    );
}

#[test]
fn runtime_ingress_registry_reports_plan_state_mutation_separately_from_external_effects() {
    let challenge = runtime_ingress_lifecycle("POST", OPERATOR_AUTHORITY_CHALLENGE_ENDPOINT)
        .expect("challenge");
    assert_eq!(
        challenge.effect_class,
        IngressEffectClass::BoundedEphemeralVerification
    );
    assert!(!challenge.effect_class.mutates_state());
    assert!(!challenge.effect_class.external_effect());
    assert_eq!(
        challenge.disposition(),
        RuntimeRequestDisposition::PlanOnlyQuarantine
    );

    let operator_plan =
        runtime_ingress_lifecycle("POST", OPERATOR_MUTATION_PLAN_ENDPOINT).expect("operator");
    assert!(operator_plan.effect_class.mutates_state());
    assert!(!operator_plan.effect_class.external_effect());
    assert!(operator_plan.credential_access());
    assert_eq!(
        operator_plan.disposition(),
        RuntimeRequestDisposition::ExactAuthorityDispatch
    );

    let quarantined_plan = runtime_ingress_lifecycle("POST", "/api/actions/probe")
        .expect("quarantined control UI plan");
    assert_eq!(
        quarantined_plan.effect_class,
        IngressEffectClass::MutationPlan
    );
    assert!(!quarantined_plan.effect_class.mutates_state());
    assert!(!quarantined_plan.effect_class.external_effect());
    assert_eq!(
        quarantined_plan.disposition(),
        RuntimeRequestDisposition::PlanOnlyQuarantine
    );

    let telegram_plan =
        runtime_ingress_lifecycle("POST", TELEGRAM_AUTHORITY_PLAN_ENDPOINT).expect("telegram");
    assert!(telegram_plan.effect_class.mutates_state());
    assert!(!telegram_plan.effect_class.external_effect());

    let telegram_pipeline =
        runtime_ingress_lifecycle("POST", TELEGRAM_AUTHORITY_COMMIT_ENDPOINT).expect("pipeline");
    assert!(telegram_pipeline.effect_class.mutates_state());
    assert!(telegram_pipeline.effect_class.external_effect());
}
