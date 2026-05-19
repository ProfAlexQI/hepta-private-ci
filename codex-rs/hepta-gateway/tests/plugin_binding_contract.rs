use hepta_core::DoctorStatus;
use hepta_gateway::{
    GatewayEnvelope, GatewayPluginResolutionSnapshot, GatewayResolvedPluginCandidate,
    GatewayResolvedPluginTier, GatewayRouteIntegritySnapshot, GatewaySurface, GatewayTransport,
};
use hepta_plugins::{
    EchoGatewayPluginAdapter, GatewayPluginBinding, GatewayPluginBindingCatalog,
    GatewayPluginBindingTier, GatewayPluginExecutionPlan, PluginCatalog, PluginIntegritySnapshot,
    StaticPlugin,
};

#[test]
fn gateway_and_plugin_lookup_intent_notes_share_command_contract_fields() {
    let draft = GatewaySurface.plugin_handoff_draft(&GatewayEnvelope::new(
        " Hepta ",
        "user-7",
        GatewayTransport::Webhook,
        " /Status --json ",
    ));
    let lookup_resolution =
        GatewayPluginBindingCatalog::new().resolve_lookup_keys(draft.binding_lookup_keys());

    let gateway_note = draft.lookup_intent_note();
    let plugin_note = lookup_resolution.lookup_intent_note();

    assert_eq!(gateway_note.surface_id, plugin_note.surface_id);
    assert_eq!(gateway_note.transport_key, plugin_note.transport_key);
    assert_eq!(gateway_note.command_selector, plugin_note.command_selector);
    assert_eq!(
        gateway_note.requested_lookup_keys,
        plugin_note.requested_lookup_keys
    );
    assert_eq!(
        gateway_note.requested_tier_labels,
        plugin_note.requested_tier_labels
    );
    assert_eq!(gateway_note.intent_label, plugin_note.intent_label);
    assert_eq!(gateway_note.explanation, plugin_note.explanation);
}

#[test]
fn gateway_and_plugin_lookup_intent_notes_share_transport_fallback_contract_fields() {
    let draft = GatewaySurface.plugin_handoff_draft(&GatewayEnvelope::new(
        "telegram",
        "user-9",
        GatewayTransport::Cli,
        " hello there ",
    ));
    let lookup_resolution =
        GatewayPluginBindingCatalog::new().resolve_lookup_keys(draft.binding_lookup_keys());

    let gateway_note = draft.lookup_intent_note();
    let plugin_note = lookup_resolution.lookup_intent_note();

    assert_eq!(gateway_note.surface_id, plugin_note.surface_id);
    assert_eq!(gateway_note.transport_key, plugin_note.transport_key);
    assert_eq!(gateway_note.command_selector, plugin_note.command_selector);
    assert_eq!(
        gateway_note.requested_lookup_keys,
        plugin_note.requested_lookup_keys
    );
    assert_eq!(
        gateway_note.requested_tier_labels,
        plugin_note.requested_tier_labels
    );
    assert_eq!(gateway_note.intent_label, plugin_note.intent_label);
    assert_eq!(gateway_note.explanation, plugin_note.explanation);
}

#[test]
fn handoff_draft_and_binding_catalog_share_the_same_lookup_contract() {
    let draft = GatewaySurface.plugin_handoff_draft(&GatewayEnvelope::new(
        " Hepta ",
        "user-7",
        GatewayTransport::Webhook,
        " /Status --json ",
    ));

    let mut catalog = GatewayPluginBindingCatalog::new();
    catalog.register(
        GatewayPluginBinding::new("status-plugin", "hepta", "webhook", "status commands")
            .with_command_selector("/status"),
    );
    catalog.register(GatewayPluginBinding::new(
        "fallback-plugin",
        "hepta",
        "webhook",
        "general ingress hooks",
    ));
    catalog.register(
        GatewayPluginBinding::new("help-plugin", "hepta", "webhook", "help commands")
            .with_command_selector("/help"),
    );

    let matches = catalog.bindings_for_route(
        &draft.surface_id,
        &draft.transport_key,
        draft.command_selector.as_deref(),
    );

    assert_eq!(
        draft.binding_lookup_keys(),
        vec![
            "surface=hepta|transport=webhook|command=/status",
            "surface=hepta|transport=webhook",
            "surface=hepta",
        ]
    );
    assert_eq!(
        matches
            .iter()
            .map(|binding| binding.lookup_key())
            .collect::<Vec<_>>(),
        vec![
            "surface=hepta|transport=webhook|command=/status".to_string(),
            "surface=hepta|transport=webhook".to_string(),
        ]
    );
    assert_eq!(
        matches
            .iter()
            .map(|binding| binding.plugin_id.as_str())
            .collect::<Vec<_>>(),
        vec!["status-plugin", "fallback-plugin"]
    );
}

#[test]
fn handoff_draft_can_fall_back_to_surface_wildcard_bindings() {
    let draft = GatewaySurface.plugin_handoff_draft(&GatewayEnvelope::new(
        " Hepta ",
        "user-7",
        GatewayTransport::Webhook,
        " /Status --json ",
    ));

    let mut catalog = GatewayPluginBindingCatalog::new();
    catalog.register(GatewayPluginBinding::for_surface(
        "surface-plugin",
        "hepta",
        "all hepta ingress",
    ));
    catalog.register(GatewayPluginBinding::new(
        "fallback-plugin",
        "hepta",
        "webhook",
        "general ingress hooks",
    ));
    catalog.register(
        GatewayPluginBinding::new("status-plugin", "hepta", "webhook", "status commands")
            .with_command_selector("/status"),
    );

    let matches = catalog.bindings_for_route(
        &draft.surface_id,
        &draft.transport_key,
        draft.command_selector.as_deref(),
    );

    assert_eq!(
        draft.binding_lookup_keys(),
        vec![
            "surface=hepta|transport=webhook|command=/status",
            "surface=hepta|transport=webhook",
            "surface=hepta",
        ]
    );
    assert_eq!(
        matches
            .iter()
            .map(|binding| binding.lookup_key())
            .collect::<Vec<_>>(),
        vec![
            "surface=hepta|transport=webhook|command=/status".to_string(),
            "surface=hepta|transport=webhook".to_string(),
            "surface=hepta".to_string(),
        ]
    );
    assert_eq!(
        matches
            .iter()
            .map(|binding| binding.plugin_id.as_str())
            .collect::<Vec<_>>(),
        vec!["status-plugin", "fallback-plugin", "surface-plugin"]
    );
}

#[test]
fn free_text_handoff_still_matches_transport_level_fallback_bindings() {
    let draft = GatewaySurface.plugin_handoff_draft(&GatewayEnvelope::new(
        "telegram",
        "user-9",
        GatewayTransport::Cli,
        " hello there ",
    ));

    let mut catalog = GatewayPluginBindingCatalog::new();
    catalog.register(GatewayPluginBinding::new(
        "fallback-plugin",
        "telegram",
        "cli",
        "general cli hooks",
    ));
    catalog.register(
        GatewayPluginBinding::new("status-plugin", "telegram", "cli", "status commands")
            .with_command_selector("/status"),
    );

    let matches = catalog.bindings_for_route(
        &draft.surface_id,
        &draft.transport_key,
        draft.command_selector.as_deref(),
    );

    assert_eq!(
        draft.binding_lookup_keys(),
        vec!["surface=telegram|transport=cli", "surface=telegram",]
    );
    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].lookup_key(), "surface=telegram|transport=cli");
    assert_eq!(matches[0].plugin_id, "fallback-plugin");
}

#[test]
fn plugin_resolution_snapshot_can_be_copied_into_gateway_without_runtime_wiring() {
    let draft = GatewaySurface.plugin_handoff_draft(&GatewayEnvelope::new(
        " Hepta ",
        "user-7",
        GatewayTransport::Webhook,
        " /Status --json ",
    ));

    let mut catalog = GatewayPluginBindingCatalog::new();
    catalog.register(GatewayPluginBinding::for_surface(
        "surface-plugin",
        "hepta",
        "all hepta ingress",
    ));
    catalog.register(GatewayPluginBinding::new(
        "fallback-plugin",
        "hepta",
        "webhook",
        "general ingress hooks",
    ));
    catalog.register(
        GatewayPluginBinding::new("status-plugin", "hepta", "webhook", "status commands")
            .with_command_selector("/status"),
    );

    let resolution = catalog.resolve_route(
        &draft.surface_id,
        &draft.transport_key,
        draft.command_selector.as_deref(),
    );
    let snapshot = GatewayPluginResolutionSnapshot::from_handoff_draft(&draft).with_candidates(
        resolution
            .matches
            .iter()
            .map(|binding| {
                GatewayResolvedPluginCandidate::new(
                    &binding.plugin_id,
                    &binding.lookup_key,
                    binding.specificity_score,
                )
            })
            .collect::<Vec<_>>(),
    );

    assert_eq!(
        snapshot.binding_lookup_keys,
        vec![
            "surface=hepta|transport=webhook|command=/status",
            "surface=hepta|transport=webhook",
            "surface=hepta",
        ]
    );
    assert!(snapshot.has_candidates());
    assert_eq!(
        resolution.lookup_keys(),
        vec![
            "surface=hepta|transport=webhook|command=/status",
            "surface=hepta|transport=webhook",
            "surface=hepta",
        ]
    );
    assert_eq!(
        snapshot.plugin_ids(),
        vec!["status-plugin", "fallback-plugin", "surface-plugin"]
    );
    assert_eq!(
        resolution.match_tiers(),
        vec![
            Some(GatewayPluginBindingTier::Command),
            Some(GatewayPluginBindingTier::Transport),
            Some(GatewayPluginBindingTier::Surface),
        ]
    );
    assert_eq!(
        snapshot.candidate_tiers(),
        vec![
            Some(GatewayResolvedPluginTier::Command),
            Some(GatewayResolvedPluginTier::Transport),
            Some(GatewayResolvedPluginTier::Surface),
        ]
    );
}

#[test]
fn plugin_and_gateway_snapshots_share_tier_labels_without_direct_runtime_wiring() {
    let draft = GatewaySurface.plugin_handoff_draft(&GatewayEnvelope::new(
        "Hepta",
        "user-7",
        GatewayTransport::Webhook,
        "/status --json",
    ));

    let mut catalog = GatewayPluginBindingCatalog::new();
    catalog.register(GatewayPluginBinding::for_surface(
        "surface-plugin",
        "hepta",
        "all hepta ingress",
    ));
    catalog.register(GatewayPluginBinding::new(
        "fallback-plugin",
        "hepta",
        "webhook",
        "general ingress hooks",
    ));
    catalog.register(
        GatewayPluginBinding::new("status-plugin", "hepta", "webhook", "status commands")
            .with_command_selector("/status"),
    );

    let resolution = catalog.resolve_route(
        &draft.surface_id,
        &draft.transport_key,
        draft.command_selector.as_deref(),
    );
    let snapshot = GatewayPluginResolutionSnapshot::from_handoff_draft(&draft).with_candidates(
        resolution
            .matches
            .iter()
            .map(|binding| {
                GatewayResolvedPluginCandidate::new(
                    &binding.plugin_id,
                    &binding.lookup_key,
                    binding.specificity_score,
                )
            })
            .collect::<Vec<_>>(),
    );

    assert_eq!(
        resolution
            .match_tiers()
            .into_iter()
            .map(|tier| tier.map(|tier| tier.as_str()))
            .collect::<Vec<_>>(),
        vec![Some("command"), Some("transport"), Some("surface")]
    );
    assert_eq!(
        snapshot
            .candidate_tiers()
            .into_iter()
            .map(|tier| tier.map(|tier| tier.as_str()))
            .collect::<Vec<_>>(),
        vec![Some("command"), Some("transport"), Some("surface")]
    );
}

#[test]
fn exact_lookup_key_resolution_matches_route_resolution_without_runtime_bridge() {
    let draft = GatewaySurface.plugin_handoff_draft(&GatewayEnvelope::new(
        "Hepta",
        "user-7",
        GatewayTransport::Webhook,
        "/status --json",
    ));

    let mut catalog = GatewayPluginBindingCatalog::new();
    catalog.register(GatewayPluginBinding::for_surface(
        "surface-plugin",
        "hepta",
        "all hepta ingress",
    ));
    catalog.register(GatewayPluginBinding::new(
        "fallback-plugin",
        "hepta",
        "webhook",
        "general ingress hooks",
    ));
    catalog.register(
        GatewayPluginBinding::new("status-plugin", "hepta", "webhook", "status commands")
            .with_command_selector("/status"),
    );

    let route_resolution = catalog.resolve_route(
        &draft.surface_id,
        &draft.transport_key,
        draft.command_selector.as_deref(),
    );
    let lookup_resolution = catalog.resolve_lookup_keys(draft.binding_lookup_keys());

    assert_eq!(
        lookup_resolution.plugin_ids(),
        route_resolution.plugin_ids()
    );
    assert_eq!(
        lookup_resolution.matched_lookup_keys(),
        route_resolution.lookup_keys()
    );
    assert_eq!(
        lookup_resolution.match_tiers(),
        route_resolution.match_tiers()
    );
    assert!(lookup_resolution.unmatched_lookup_keys().is_empty());
}

#[test]
fn gateway_snapshot_can_expose_unmatched_lookup_keys_for_partial_plugin_coverage() {
    let draft = GatewaySurface.plugin_handoff_draft(&GatewayEnvelope::new(
        "telegram",
        "user-9",
        GatewayTransport::Cli,
        "/status",
    ));

    let mut catalog = GatewayPluginBindingCatalog::new();
    catalog.register(GatewayPluginBinding::new(
        "fallback-plugin",
        "telegram",
        "cli",
        "general cli hooks",
    ));

    let lookup_resolution = catalog.resolve_lookup_keys(draft.binding_lookup_keys());
    let snapshot = GatewayPluginResolutionSnapshot::from_handoff_draft(&draft).with_candidates(
        lookup_resolution
            .matches
            .iter()
            .map(|binding| {
                GatewayResolvedPluginCandidate::new(
                    &binding.plugin_id,
                    &binding.lookup_key,
                    binding.specificity_score,
                )
            })
            .collect::<Vec<_>>(),
    );

    assert_eq!(snapshot.plugin_ids(), vec!["fallback-plugin"]);
    assert_eq!(
        snapshot.matched_lookup_keys(),
        vec!["surface=telegram|transport=cli"]
    );
    assert_eq!(
        snapshot.unmatched_lookup_keys(),
        vec![
            "surface=telegram|transport=cli|command=/status",
            "surface=telegram",
        ]
    );
    assert_eq!(
        lookup_resolution.unmatched_lookup_keys(),
        snapshot.unmatched_lookup_keys()
    );
}

#[test]
fn plugin_and_gateway_diagnostic_notes_share_lookup_and_tier_contracts() {
    let draft = GatewaySurface.plugin_handoff_draft(&GatewayEnvelope::new(
        "Hepta",
        "user-7",
        GatewayTransport::Webhook,
        "/status --json",
    ));

    let mut catalog = GatewayPluginBindingCatalog::new();
    catalog.register(GatewayPluginBinding::for_surface(
        "surface-plugin",
        "hepta",
        "all hepta ingress",
    ));
    catalog.register(GatewayPluginBinding::new(
        "fallback-plugin",
        "hepta",
        "webhook",
        "general ingress hooks",
    ));
    catalog.register(
        GatewayPluginBinding::new("status-plugin", "hepta", "webhook", "status commands")
            .with_command_selector("/status"),
    );

    let resolution = catalog.resolve_route(
        &draft.surface_id,
        &draft.transport_key,
        draft.command_selector.as_deref(),
    );
    let snapshot = GatewayPluginResolutionSnapshot::from_handoff_draft(&draft).with_candidates(
        resolution
            .matches
            .iter()
            .map(|binding| {
                GatewayResolvedPluginCandidate::new(
                    &binding.plugin_id,
                    &binding.lookup_key,
                    binding.specificity_score,
                )
            })
            .collect::<Vec<_>>(),
    );

    assert_eq!(
        resolution
            .diagnostic_notes()
            .iter()
            .map(|note| {
                (
                    note.plugin_id.as_str(),
                    note.lookup_key.as_str(),
                    note.tier_label.as_deref(),
                )
            })
            .collect::<Vec<_>>(),
        snapshot
            .diagnostic_notes()
            .iter()
            .map(|note| {
                (
                    note.plugin_id.as_str(),
                    note.lookup_key.as_str(),
                    note.tier_label.as_deref(),
                )
            })
            .collect::<Vec<_>>()
    );
}

#[test]
fn plugin_and_gateway_gap_notes_share_lookup_and_tier_contracts() {
    let draft = GatewaySurface.plugin_handoff_draft(&GatewayEnvelope::new(
        "telegram",
        "user-9",
        GatewayTransport::Cli,
        "/status",
    ));

    let mut catalog = GatewayPluginBindingCatalog::new();
    catalog.register(GatewayPluginBinding::new(
        "fallback-plugin",
        "telegram",
        "cli",
        "general cli hooks",
    ));

    let lookup_resolution = catalog.resolve_lookup_keys(draft.binding_lookup_keys());
    let snapshot = GatewayPluginResolutionSnapshot::from_handoff_draft(&draft).with_candidates(
        lookup_resolution
            .matches
            .iter()
            .map(|binding| {
                GatewayResolvedPluginCandidate::new(
                    &binding.plugin_id,
                    &binding.lookup_key,
                    binding.specificity_score,
                )
            })
            .collect::<Vec<_>>(),
    );

    assert_eq!(
        lookup_resolution
            .coverage_gap_notes()
            .iter()
            .map(|note| (note.lookup_key.as_str(), note.tier_label.as_deref()))
            .collect::<Vec<_>>(),
        snapshot
            .coverage_gap_notes()
            .iter()
            .map(|note| (note.lookup_key.as_str(), note.tier_label.as_deref()))
            .collect::<Vec<_>>()
    );
}

#[test]
fn plugin_and_gateway_coverage_digests_share_lookup_and_tier_contracts() {
    let draft = GatewaySurface.plugin_handoff_draft(&GatewayEnvelope::new(
        "telegram",
        "user-9",
        GatewayTransport::Cli,
        "/status",
    ));

    let mut catalog = GatewayPluginBindingCatalog::new();
    catalog.register(GatewayPluginBinding::new(
        "fallback-plugin",
        "telegram",
        "cli",
        "general cli hooks",
    ));

    let lookup_resolution = catalog.resolve_lookup_keys(draft.binding_lookup_keys());
    let snapshot = GatewayPluginResolutionSnapshot::from_handoff_draft(&draft).with_candidates(
        lookup_resolution
            .matches
            .iter()
            .map(|binding| {
                GatewayResolvedPluginCandidate::new(
                    &binding.plugin_id,
                    &binding.lookup_key,
                    binding.specificity_score,
                )
            })
            .collect::<Vec<_>>(),
    );

    let plugin_digest = lookup_resolution.coverage_digest();
    let gateway_digest = snapshot.coverage_digest();

    assert_eq!(
        (
            plugin_digest.requested_lookup_keys,
            plugin_digest.requested_tier_labels,
            plugin_digest.matched_lookup_keys,
            plugin_digest.matched_tier_labels,
            plugin_digest.unmatched_lookup_keys,
            plugin_digest.unmatched_tier_labels,
            plugin_digest.full_coverage,
        ),
        (
            gateway_digest.requested_lookup_keys,
            gateway_digest.requested_tier_labels,
            gateway_digest.matched_lookup_keys,
            gateway_digest.matched_tier_labels,
            gateway_digest.unmatched_lookup_keys,
            gateway_digest.unmatched_tier_labels,
            gateway_digest.full_coverage,
        )
    );
}

#[test]
fn plugin_and_gateway_binding_scaffold_notes_share_lookup_contracts() {
    let draft = GatewaySurface.plugin_handoff_draft(&GatewayEnvelope::new(
        "telegram",
        "user-9",
        GatewayTransport::Cli,
        "/status",
    ));

    let mut catalog = GatewayPluginBindingCatalog::new();
    catalog.register(GatewayPluginBinding::new(
        "fallback-plugin",
        "telegram",
        "cli",
        "general cli hooks",
    ));

    let lookup_resolution = catalog.resolve_lookup_keys(draft.binding_lookup_keys());
    let snapshot = GatewayPluginResolutionSnapshot::from_handoff_draft(&draft).with_candidates(
        lookup_resolution
            .matches
            .iter()
            .map(|binding| {
                GatewayResolvedPluginCandidate::new(
                    &binding.plugin_id,
                    &binding.lookup_key,
                    binding.specificity_score,
                )
            })
            .collect::<Vec<_>>(),
    );

    assert_eq!(
        lookup_resolution
            .binding_scaffold_notes()
            .iter()
            .map(|note| {
                (
                    note.lookup_key.as_str(),
                    note.tier_label.as_deref(),
                    note.binding_kind.as_deref(),
                    note.surface_id.as_deref(),
                    note.transport_key.as_deref(),
                    note.command_selector.as_deref(),
                    note.note.as_str(),
                )
            })
            .collect::<Vec<_>>(),
        snapshot
            .binding_scaffold_notes()
            .iter()
            .map(|note| {
                (
                    note.lookup_key.as_str(),
                    note.tier_label.as_deref(),
                    note.binding_kind.as_deref(),
                    note.surface_id.as_deref(),
                    note.transport_key.as_deref(),
                    note.command_selector.as_deref(),
                    note.note.as_str(),
                )
            })
            .collect::<Vec<_>>()
    );
}

#[test]
fn plugin_and_gateway_plugin_scaffold_stubs_share_lookup_contracts() {
    let draft = GatewaySurface.plugin_handoff_draft(&GatewayEnvelope::new(
        "telegram",
        "user-9",
        GatewayTransport::Cli,
        "/status",
    ));

    let mut catalog = GatewayPluginBindingCatalog::new();
    catalog.register(GatewayPluginBinding::new(
        "fallback-plugin",
        "telegram",
        "cli",
        "general cli hooks",
    ));

    let lookup_resolution = catalog.resolve_lookup_keys(draft.binding_lookup_keys());
    let snapshot = GatewayPluginResolutionSnapshot::from_handoff_draft(&draft).with_candidates(
        lookup_resolution
            .matches
            .iter()
            .map(|binding| {
                GatewayResolvedPluginCandidate::new(
                    &binding.plugin_id,
                    &binding.lookup_key,
                    binding.specificity_score,
                )
            })
            .collect::<Vec<_>>(),
    );

    assert_eq!(
        lookup_resolution
            .plugin_scaffold_stubs()
            .iter()
            .map(|stub| {
                (
                    stub.lookup_key.as_str(),
                    stub.tier_label.as_deref(),
                    stub.binding_kind.as_deref(),
                    stub.plugin_id_hint.as_str(),
                    stub.module_stub_name.as_str(),
                    stub.description_hint.as_str(),
                    stub.surface_id.as_deref(),
                    stub.transport_key.as_deref(),
                    stub.command_selector.as_deref(),
                )
            })
            .collect::<Vec<_>>(),
        snapshot
            .plugin_scaffold_stubs()
            .iter()
            .map(|stub| {
                (
                    stub.lookup_key.as_str(),
                    stub.tier_label.as_deref(),
                    stub.binding_kind.as_deref(),
                    stub.plugin_id_hint.as_str(),
                    stub.module_stub_name.as_str(),
                    stub.description_hint.as_str(),
                    stub.surface_id.as_deref(),
                    stub.transport_key.as_deref(),
                    stub.command_selector.as_deref(),
                )
            })
            .collect::<Vec<_>>()
    );
}

#[test]
fn plugin_and_gateway_lookup_trace_steps_share_lookup_contracts() {
    let draft = GatewaySurface.plugin_handoff_draft(&GatewayEnvelope::new(
        "telegram",
        "user-9",
        GatewayTransport::Cli,
        "/status",
    ));

    let mut catalog = GatewayPluginBindingCatalog::new();
    catalog.register(GatewayPluginBinding::new(
        "fallback-plugin",
        "telegram",
        "cli",
        "general cli hooks",
    ));

    let lookup_resolution = catalog.resolve_lookup_keys(draft.binding_lookup_keys());
    let snapshot = GatewayPluginResolutionSnapshot::from_handoff_draft(&draft).with_candidates(
        lookup_resolution
            .matches
            .iter()
            .map(|binding| {
                GatewayResolvedPluginCandidate::new(
                    &binding.plugin_id,
                    &binding.lookup_key,
                    binding.specificity_score,
                )
            })
            .collect::<Vec<_>>(),
    );

    assert_eq!(
        lookup_resolution
            .lookup_trace_steps()
            .iter()
            .map(|step| {
                (
                    step.lookup_key.as_str(),
                    step.tier_label.as_deref(),
                    step.matched_plugin_ids.clone(),
                    step.matched,
                    step.note.as_str(),
                )
            })
            .collect::<Vec<_>>(),
        snapshot
            .lookup_trace_steps()
            .iter()
            .map(|step| {
                (
                    step.lookup_key.as_str(),
                    step.tier_label.as_deref(),
                    step.matched_plugin_ids.clone(),
                    step.matched,
                    step.note.as_str(),
                )
            })
            .collect::<Vec<_>>()
    );
}

#[test]
fn plugin_and_gateway_contract_reports_share_lookup_contracts() {
    let draft = GatewaySurface.plugin_handoff_draft(&GatewayEnvelope::new(
        "telegram",
        "user-9",
        GatewayTransport::Cli,
        "/status",
    ));

    let mut catalog = GatewayPluginBindingCatalog::new();
    catalog.register(GatewayPluginBinding::new(
        "fallback-plugin",
        "telegram",
        "cli",
        "general cli hooks",
    ));

    let lookup_resolution = catalog.resolve_lookup_keys(draft.binding_lookup_keys());
    let snapshot = GatewayPluginResolutionSnapshot::from_handoff_draft(&draft).with_candidates(
        lookup_resolution
            .matches
            .iter()
            .map(|binding| {
                GatewayResolvedPluginCandidate::new(
                    &binding.plugin_id,
                    &binding.lookup_key,
                    binding.specificity_score,
                )
            })
            .collect::<Vec<_>>(),
    );

    let plugin_report = lookup_resolution.contract_report();
    let gateway_report = snapshot.contract_report();

    assert_eq!(
        (
            plugin_report.requested_lookup_keys,
            plugin_report.requested_tier_labels,
            plugin_report.matched_plugin_ids,
            plugin_report.matched_lookup_keys,
            plugin_report.matched_tier_labels,
            plugin_report.unmatched_lookup_keys,
            plugin_report.unmatched_tier_labels,
            plugin_report.full_coverage,
        ),
        (
            gateway_report.requested_lookup_keys,
            gateway_report.requested_tier_labels,
            gateway_report.matched_plugin_ids,
            gateway_report.matched_lookup_keys,
            gateway_report.matched_tier_labels,
            gateway_report.unmatched_lookup_keys,
            gateway_report.unmatched_tier_labels,
            gateway_report.full_coverage,
        )
    );

    assert_eq!(
        plugin_report
            .diagnostic_notes
            .iter()
            .map(|note| {
                (
                    note.plugin_id.as_str(),
                    note.lookup_key.as_str(),
                    note.tier_label.as_deref(),
                )
            })
            .collect::<Vec<_>>(),
        gateway_report
            .diagnostic_notes
            .iter()
            .map(|note| {
                (
                    note.plugin_id.as_str(),
                    note.lookup_key.as_str(),
                    note.tier_label.as_deref(),
                )
            })
            .collect::<Vec<_>>()
    );

    assert_eq!(
        plugin_report
            .coverage_gap_notes
            .iter()
            .map(|note| (note.lookup_key.as_str(), note.tier_label.as_deref()))
            .collect::<Vec<_>>(),
        gateway_report
            .coverage_gap_notes
            .iter()
            .map(|note| (note.lookup_key.as_str(), note.tier_label.as_deref()))
            .collect::<Vec<_>>()
    );

    assert_eq!(
        plugin_report
            .binding_scaffold_notes
            .iter()
            .map(|note| {
                (
                    note.lookup_key.as_str(),
                    note.tier_label.as_deref(),
                    note.binding_kind.as_deref(),
                    note.surface_id.as_deref(),
                    note.transport_key.as_deref(),
                    note.command_selector.as_deref(),
                )
            })
            .collect::<Vec<_>>(),
        gateway_report
            .binding_scaffold_notes
            .iter()
            .map(|note| {
                (
                    note.lookup_key.as_str(),
                    note.tier_label.as_deref(),
                    note.binding_kind.as_deref(),
                    note.surface_id.as_deref(),
                    note.transport_key.as_deref(),
                    note.command_selector.as_deref(),
                )
            })
            .collect::<Vec<_>>()
    );

    assert_eq!(
        plugin_report
            .lookup_trace_steps
            .iter()
            .map(|step| {
                (
                    step.lookup_key.as_str(),
                    step.tier_label.as_deref(),
                    step.matched_plugin_ids.clone(),
                    step.matched,
                    step.note.as_str(),
                )
            })
            .collect::<Vec<_>>(),
        gateway_report
            .lookup_trace_steps
            .iter()
            .map(|step| {
                (
                    step.lookup_key.as_str(),
                    step.tier_label.as_deref(),
                    step.matched_plugin_ids.clone(),
                    step.matched,
                    step.note.as_str(),
                )
            })
            .collect::<Vec<_>>()
    );
}

#[test]
fn plugin_and_gateway_scaffolding_plans_share_lookup_contracts() {
    let draft = GatewaySurface.plugin_handoff_draft(&GatewayEnvelope::new(
        "telegram",
        "user-9",
        GatewayTransport::Cli,
        "/status",
    ));

    let mut catalog = GatewayPluginBindingCatalog::new();
    catalog.register(GatewayPluginBinding::new(
        "fallback-plugin",
        "telegram",
        "cli",
        "general cli hooks",
    ));

    let lookup_resolution = catalog.resolve_lookup_keys(draft.binding_lookup_keys());
    let snapshot = GatewayPluginResolutionSnapshot::from_handoff_draft(&draft).with_candidates(
        lookup_resolution
            .matches
            .iter()
            .map(|binding| {
                GatewayResolvedPluginCandidate::new(
                    &binding.plugin_id,
                    &binding.lookup_key,
                    binding.specificity_score,
                )
            })
            .collect::<Vec<_>>(),
    );

    let plugin_plan = lookup_resolution.scaffold_plan();
    let gateway_plan = snapshot.scaffold_plan();

    assert_eq!(
        (
            plugin_plan.requested_lookup_keys,
            plugin_plan.requested_tier_labels,
            plugin_plan.unmatched_lookup_keys,
            plugin_plan.unmatched_tier_labels,
            plugin_plan.scaffolding_required,
        ),
        (
            gateway_plan.requested_lookup_keys,
            gateway_plan.requested_tier_labels,
            gateway_plan.unmatched_lookup_keys,
            gateway_plan.unmatched_tier_labels,
            gateway_plan.scaffolding_required,
        )
    );

    assert_eq!(
        plugin_plan
            .binding_scaffold_notes
            .iter()
            .map(|note| {
                (
                    note.lookup_key.as_str(),
                    note.tier_label.as_deref(),
                    note.binding_kind.as_deref(),
                    note.surface_id.as_deref(),
                    note.transport_key.as_deref(),
                    note.command_selector.as_deref(),
                    note.note.as_str(),
                )
            })
            .collect::<Vec<_>>(),
        gateway_plan
            .binding_scaffold_notes
            .iter()
            .map(|note| {
                (
                    note.lookup_key.as_str(),
                    note.tier_label.as_deref(),
                    note.binding_kind.as_deref(),
                    note.surface_id.as_deref(),
                    note.transport_key.as_deref(),
                    note.command_selector.as_deref(),
                    note.note.as_str(),
                )
            })
            .collect::<Vec<_>>()
    );

    assert_eq!(
        plugin_plan
            .plugin_scaffold_stubs
            .iter()
            .map(|stub| {
                (
                    stub.lookup_key.as_str(),
                    stub.tier_label.as_deref(),
                    stub.binding_kind.as_deref(),
                    stub.plugin_id_hint.as_str(),
                    stub.module_stub_name.as_str(),
                    stub.description_hint.as_str(),
                    stub.surface_id.as_deref(),
                    stub.transport_key.as_deref(),
                    stub.command_selector.as_deref(),
                )
            })
            .collect::<Vec<_>>(),
        gateway_plan
            .plugin_scaffold_stubs
            .iter()
            .map(|stub| {
                (
                    stub.lookup_key.as_str(),
                    stub.tier_label.as_deref(),
                    stub.binding_kind.as_deref(),
                    stub.plugin_id_hint.as_str(),
                    stub.module_stub_name.as_str(),
                    stub.description_hint.as_str(),
                    stub.surface_id.as_deref(),
                    stub.transport_key.as_deref(),
                    stub.command_selector.as_deref(),
                )
            })
            .collect::<Vec<_>>()
    );
}

#[test]
fn gateway_plugin_handoff_smoke_reaches_dispatch_and_operational_readiness() {
    let surface = GatewaySurface;
    let envelope = GatewayEnvelope::new(
        " Hepta ",
        "session-95",
        GatewayTransport::Webhook,
        " /Status --json ",
    );
    let draft = surface.plugin_handoff_draft(&envelope);

    let mut bindings = GatewayPluginBindingCatalog::new();
    bindings.register(
        GatewayPluginBinding::new("status-plugin", "hepta", "webhook", "status commands")
            .with_command_selector("/status"),
    );
    bindings.register(GatewayPluginBinding::new(
        "fallback-plugin",
        "hepta",
        "webhook",
        "fallback webhook commands",
    ));
    bindings.register(GatewayPluginBinding::for_surface(
        "surface-plugin",
        "hepta",
        "surface-wide fallback",
    ));

    let lookup_resolution = bindings.resolve_lookup_keys(draft.binding_lookup_keys());
    let resolution = GatewayPluginResolutionSnapshot::from_handoff_draft(&draft).with_candidates(
        lookup_resolution
            .matches
            .iter()
            .map(|binding| {
                GatewayResolvedPluginCandidate::new(
                    &binding.plugin_id,
                    &binding.lookup_key,
                    binding.specificity_score,
                )
            })
            .collect::<Vec<_>>(),
    );

    let gateway_snapshot =
        GatewayRouteIntegritySnapshot::from_resolution(&surface, &envelope, &resolution);
    let dispatch_report = gateway_snapshot.dispatch_readiness_report(&resolution);
    assert!(dispatch_report.ready);
    assert_eq!(dispatch_report.blockers, Vec::<String>::new());
    assert_eq!(
        gateway_snapshot
            .dispatch_doctor_checks(&resolution)
            .last()
            .expect("dispatch check should exist")
            .status,
        DoctorStatus::Ok
    );

    let mut manifests = PluginCatalog::new();
    let status_plugin = StaticPlugin::new("status-plugin", "0.1.0", "status commands");
    let fallback_plugin = StaticPlugin::new("fallback-plugin", "0.1.0", "fallback commands");
    let surface_plugin = StaticPlugin::new("surface-plugin", "0.1.0", "surface fallback");
    manifests.register(&status_plugin);
    manifests.register(&fallback_plugin);
    manifests.register(&surface_plugin);

    let plugin_snapshot = PluginIntegritySnapshot::from_catalogs(&manifests, &bindings);
    let operational_report = plugin_snapshot.operational_readiness_report();
    assert!(operational_report.ready);
    assert!(operational_report.blockers.is_empty());
    assert!(operational_report.warnings.is_empty());
    assert_eq!(
        plugin_snapshot
            .operational_doctor_checks()
            .last()
            .expect("operational check should exist")
            .status,
        DoctorStatus::Ok
    );

    let execution_plan = GatewayPluginExecutionPlan::from_lookup_resolution(
        &lookup_resolution,
        &draft.normalized_text,
    );
    assert!(execution_plan.ready);
    assert_eq!(
        execution_plan
            .preferred_handoff()
            .map(|handoff| handoff.plugin_id.as_str()),
        Some("status-plugin")
    );
    let status_adapter = EchoGatewayPluginAdapter::new("status-plugin");
    let execution_attempt =
        execution_plan.execute_first_matching_adapter(&draft.normalized_text, &[&status_adapter]);
    assert!(execution_attempt.ready);
    assert_eq!(
        execution_attempt.selected_plugin_id.as_deref(),
        Some("status-plugin")
    );
    let adapter_output = execution_attempt
        .result
        .as_ref()
        .and_then(|result| result.output.as_deref())
        .unwrap_or_default();
    assert!(adapter_output.contains("status-plugin"));
    assert!(
        adapter_output
            .to_ascii_lowercase()
            .contains("/status --json")
    );
}
