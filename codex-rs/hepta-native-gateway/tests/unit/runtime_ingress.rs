use super::*;

#[test]
fn runtime_ingress_registry_covers_all_declared_routes_and_serializes_lifecycle_fields() {
    let registry = runtime_ingress_lifecycle_registry();
    assert_eq!(
        registry.len(),
        SPECIAL_INGRESS_LIFECYCLES.len()
            + CONTROL_UI_STATIC_ASSET_LIFECYCLES.len()
            + CONTROL_UI_ROUTE_SPECS.len()
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
        if route.is_quarantined_transitive_effect() {
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
fn governed_backend_keeps_legacy_control_ui_posts_plan_only() {
    let legacy_posts = CONTROL_UI_ROUTE_SPECS
        .iter()
        .filter(|route| route.method == "POST")
        .collect::<Vec<_>>();
    assert_eq!(legacy_posts.len(), 12);
    for route in legacy_posts {
        let path = route
            .pattern
            .replace("<action>", "probe")
            .replace("<id>", "probe");
        let lifecycle = runtime_ingress_lifecycle(route.method, &path)
            .unwrap_or_else(|| panic!("missing compatibility POST lifecycle: {}", route.pattern));
        assert_eq!(lifecycle.effect_class, IngressEffectClass::MutationPlan);
        assert_eq!(
            lifecycle.default_enablement,
            IngressDefaultEnablement::PlanOnlyEnabled
        );
        assert_eq!(
            lifecycle.authority_owner,
            IngressAuthorityOwner::RuntimeKernelRequestBinding
        );
        assert!(hepta_authority::governed_mutation_spec(route.pattern).is_none());
    }
    assert!(hepta_gateway::NATIVE_POST_REAL_HANDLER_PLAN_KINDS.is_empty());
    assert_eq!(
        hepta_gateway::NATIVE_POST_COMPATIBILITY_HARNESS_PLAN_KINDS,
        ["approval_apply", "task_publish", "chat_send"]
    );
}

#[test]
fn governed_backend_real_mutations_match_the_typed_allowlist() {
    for endpoint in [
        PREFERENCE_COMMIT_ENDPOINT,
        OPERATOR_MUTATION_COMMIT_ENDPOINT,
        TELEGRAM_AUTHORITY_COMMIT_ENDPOINT,
    ] {
        let mutation = hepta_authority::governed_mutation_spec(endpoint)
            .unwrap_or_else(|| panic!("missing governed mutation: {endpoint}"));
        let lifecycle = runtime_ingress_lifecycle("POST", endpoint)
            .unwrap_or_else(|| panic!("missing governed lifecycle: {endpoint}"));
        assert_ne!(lifecycle.effect_class, IngressEffectClass::MutationPlan);
        assert_eq!(
            lifecycle.default_enablement == IngressDefaultEnablement::AuthenticatedEnabled,
            mutation.default_enabled
        );
    }
    assert_eq!(
        hepta_authority::governed_mutation_spec(TELEGRAM_AUTHORITY_COMMIT_ENDPOINT)
            .map(|mutation| mutation.disposition),
        Some(hepta_authority::GovernedMutationDisposition::ControlledLiveDeferred)
    );
}

#[test]
fn runtime_ingress_registry_covers_detached_reports_and_watchdog_readbacks() {
    for path in DETACHED_CONTROL_UI_REPORT_PATHS {
        assert_eq!(
            runtime_ingress_lifecycle("GET", path)
                .map(|lifecycle| (lifecycle.effect_class, lifecycle.disposition())),
            Some((
                IngressEffectClass::MetadataRead,
                RuntimeRequestDisposition::ReadOnlyDispatch
            )),
            "unclassified or unsafe detached report: GET {path}"
        );
    }

    for path in [
        "/health",
        CONTROL_UI_ROUTE_PARITY_ENDPOINT,
        "/api/operator-security",
        "/api/telegram-owner-handoff",
        "/api/telegram-poll-loop",
        "/api/native-post-activation-plan",
        "/api/native-post-execution-stores",
        hepta_gateway::HEPTA_ENGINE_ADAPTER_BOUNDARY_ENDPOINT,
        hepta_gateway::HEPTA_CODEX_ENGINE_ADAPTER_BOUNDARY_ENDPOINT,
        hepta_gateway::HEPTA_CORE_FUSION_READINESS_ENDPOINT,
        hepta_gateway::HEPTA_NAME_REPOSITORY_CLOSURE_ENDPOINT,
        hepta_gateway::HEPTA_ENGINE_DEPENDENCY_CLOSURE_ENDPOINT,
    ] {
        assert_eq!(
            runtime_ingress_lifecycle("GET", path).map(|lifecycle| (
                lifecycle.disposition(),
                lifecycle.effect_class.performs_effect()
            )),
            Some((RuntimeRequestDisposition::ReadOnlyDispatch, false)),
            "unclassified or unsafe watchdog readback: GET {path}"
        );
    }
}

#[test]
fn runtime_ingress_registry_derives_all_telegram_live_soak_aliases() {
    let registry = runtime_ingress_lifecycle_registry();
    for path in TELEGRAM_LIVE_SOAK_ROUTE.paths() {
        let lifecycle = runtime_ingress_lifecycle("GET", path)
            .unwrap_or_else(|| panic!("unclassified Telegram live-soak route: {path}"));
        assert_eq!(lifecycle.path_pattern, path);
        assert_eq!(lifecycle.effect_class, IngressEffectClass::MetadataRead);
        assert_eq!(
            lifecycle.disposition(),
            RuntimeRequestDisposition::ReadOnlyDispatch
        );
        assert_eq!(
            registry
                .iter()
                .filter(|candidate| candidate.method == "GET" && candidate.path_pattern == path)
                .count(),
            1,
            "Telegram live-soak route must have exactly one lifecycle: {path}"
        );
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
        CONTROL_UI_ROUTE_SPECS
            .iter()
            .filter(|route| route.is_quarantined_transitive_effect())
            .count()
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
