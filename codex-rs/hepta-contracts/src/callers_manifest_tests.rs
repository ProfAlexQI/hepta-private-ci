use std::collections::BTreeSet;

use serde::Deserialize;

const EXPECTED_UPSTREAM_CUTOFF: &str = "74004b5397b24662a87a5264a6ae80664168c7f3";

#[derive(Debug, Deserialize)]
struct CallersManifest {
    schema_version: u32,
    rule: String,
    candidate_base: String,
    frozen_oracle: String,
    surface: Vec<Surface>,
    excluded: Vec<Excluded>,
    qualification: Vec<toml::Value>,
}

#[derive(Debug, Deserialize)]
struct Surface {
    #[serde(rename = "crate")]
    crate_name: String,
    role: String,
    public_items: Vec<String>,
    product_callers: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Excluded {
    #[serde(rename = "crate")]
    crate_name: String,
    reason: String,
}

fn manifest() -> CallersManifest {
    toml::from_str(include_str!("../CALLERS.toml")).expect("CALLERS.toml must parse")
}

fn surface<'a>(manifest: &'a CallersManifest, crate_name: &str) -> &'a Surface {
    manifest
        .surface
        .iter()
        .find(|surface| surface.crate_name == crate_name)
        .unwrap_or_else(|| panic!("missing product surface {crate_name}"))
}

#[test]
fn callers_manifest_schema_and_cutoff_are_frozen() {
    let manifest = manifest();

    assert_eq!(manifest.schema_version, 1);
    assert_eq!(manifest.candidate_base, EXPECTED_UPSTREAM_CUTOFF);
    assert_eq!(manifest.frozen_oracle.len(), 40);
    assert!(
        manifest
            .frozen_oracle
            .chars()
            .all(|ch| ch.is_ascii_hexdigit())
    );
    assert!(manifest.rule.contains("qualification-only"));
    assert!(!manifest.qualification.is_empty());
}

#[test]
fn callers_manifest_entries_are_unique_and_well_formed() {
    let manifest = manifest();
    let mut surfaces = BTreeSet::new();
    let mut excluded = BTreeSet::new();

    for surface in &manifest.surface {
        assert!(
            surfaces.insert(surface.crate_name.as_str()),
            "duplicate surface {}",
            surface.crate_name
        );
        assert!(
            !surface.role.is_empty(),
            "{} has an empty role",
            surface.crate_name
        );
        assert!(
            !surface.public_items.is_empty(),
            "{} has no public items",
            surface.crate_name
        );
        assert!(
            !surface.product_callers.is_empty(),
            "{} has no product callers",
            surface.crate_name
        );

        let mut public_items = BTreeSet::new();
        for item in &surface.public_items {
            assert!(
                public_items.insert(item),
                "{} repeats public item {item}",
                surface.crate_name
            );
        }

        let mut product_callers = BTreeSet::new();
        for caller in &surface.product_callers {
            assert!(
                product_callers.insert(caller),
                "{} repeats caller {caller}",
                surface.crate_name
            );
            assert!(
                caller.starts_with("codex-rs/"),
                "caller is not repo-relative: {caller}"
            );
            assert!(
                caller.ends_with(".rs"),
                "caller is not a Rust source path: {caller}"
            );
            assert!(
                !caller.contains(".."),
                "caller escapes the repository: {caller}"
            );
        }
    }

    for entry in &manifest.excluded {
        assert!(
            excluded.insert(entry.crate_name.as_str()),
            "duplicate exclusion {}",
            entry.crate_name
        );
        assert!(
            !entry.reason.is_empty(),
            "{} has no exclusion reason",
            entry.crate_name
        );
    }

    assert!(
        surfaces.is_disjoint(&excluded),
        "a crate cannot be both product and excluded"
    );
}

#[test]
fn callers_manifest_records_the_live_product_chain() {
    let manifest = manifest();

    let paths = surface(&manifest, "codex-hepta-paths");
    assert_eq!(paths.public_items, ["HeptaStateRoot", "HeptaStateLayout"]);
    assert_eq!(
        paths.product_callers,
        [
            "codex-rs/hepta-runtime/src/lib.rs",
            "codex-rs/hepta-native-gateway/src/lib.rs",
        ]
    );

    let runtime = surface(&manifest, "codex-hepta-runtime");
    assert_eq!(runtime.public_items, ["HeptaRuntime", "RuntimeStatus"]);
    assert_eq!(
        runtime.product_callers,
        ["codex-rs/hepta-native-gateway/src/lib.rs"]
    );

    let gateway = surface(&manifest, "codex-hepta-native-gateway");
    assert_eq!(
        gateway.public_items,
        [
            "print_live_shell_contract_if_requested",
            "run_serve_ui_if_requested",
        ]
    );
    assert_eq!(gateway.product_callers, ["codex-rs/cli/src/main.rs"]);
}

#[test]
fn qualification_crates_are_excluded_from_product_callers() {
    let manifest = manifest();
    let excluded = manifest
        .excluded
        .iter()
        .map(|entry| entry.crate_name.as_str())
        .collect::<BTreeSet<_>>();

    assert!(excluded.contains("codex-hepta-shadow-qualification"));
    assert!(excluded.contains("codex-hepta-operator-acceptance"));
    assert!(excluded.contains("codex-hepta-channel-adapter"));
    assert!(manifest.surface.iter().all(|surface| {
        surface.crate_name != "codex-hepta-shadow-qualification"
            && surface.crate_name != "codex-hepta-operator-acceptance"
    }));
}

#[test]
fn legacy_caller_zero_surfaces_are_explicitly_retired() {
    let manifest = manifest();
    let retirement = manifest
        .qualification
        .iter()
        .find(|entry| {
            entry.get("slice").and_then(toml::Value::as_str) == Some("legacy_surface_retirement")
        })
        .expect("missing legacy surface retirement decision");

    for field in [
        "legacy_channel_adapter_retired",
        "legacy_memory_mutation_writer_retired",
        "legacy_proof_command_retired",
        "legacy_outbound_delivery_retired",
    ] {
        assert_eq!(
            retirement.get(field).and_then(toml::Value::as_bool),
            Some(true),
            "{field} must stay retired",
        );
    }
    for field in [
        "observed_live_outcome_rows",
        "observed_live_effect_ack_rows",
        "observed_live_preference_rows",
    ] {
        assert_eq!(
            retirement.get(field).and_then(toml::Value::as_integer),
            Some(0),
            "{field} must match the audited live store",
        );
    }
}

#[test]
fn g3_intelligence_kg_truth_bounds_qualification_without_hiding_later_candidates() {
    let manifest = manifest();
    let memory = surface(&manifest, "codex-hepta-memory");
    assert_eq!(
        memory.role,
        "agent_local_versioned_memory_intelligence_and_kg"
    );
    assert!(
        memory
            .product_callers
            .iter()
            .any(|caller| caller.ends_with("/cognitive/tools.rs"))
    );

    let g3 = manifest
        .qualification
        .iter()
        .find(|entry| {
            entry.get("slice").and_then(toml::Value::as_str)
                == Some("R2_intelligence_kg_closed_loop")
        })
        .expect("missing G3 Intelligence/KG qualification truth");
    for field in [
        "product_caller_named",
        "governance_required_for_write",
        "mutation_writer_composed",
        "source_memory_revision_and_kg_projection_one_transaction",
        "immutable_revision_fact_header_required_even_when_empty",
        "exact_memory_citation_bound",
        "correction_and_forget_advance_projection_generation",
        "multi_provenance_entity_occurrences_preserved",
        "retrieval_single_sqlite_snapshot",
        "physical_provider_attachment_revalidated",
        "attachment_channels_bound_to_physical_send",
        "active_ephemeral_input_forces_sensitive_http",
        "preexisting_later_stage_candidate_surfaces_present",
    ] {
        assert_eq!(
            g3.get(field).and_then(toml::Value::as_bool),
            Some(true),
            "{field} must describe the composed G3 product path",
        );
    }
    let product_callers = g3
        .get("product_callers")
        .and_then(toml::Value::as_array)
        .expect("G3 product callers must be explicit");
    assert!(
        product_callers
            .iter()
            .any(|caller| { caller.as_str() == Some("codex-rs/hepta-agentd/src/runtime.rs") })
    );
    let qualification_callers = g3
        .get("qualification_callers")
        .and_then(toml::Value::as_array)
        .expect("G3 qualification callers must be explicit");
    assert_eq!(
        qualification_callers
            .iter()
            .filter_map(toml::Value::as_str)
            .collect::<Vec<_>>(),
        ["codex-rs/hepta-agentd/tests/cognitive_product_e2e.rs"]
    );
    let qualification_tests = g3
        .get("qualification_tests")
        .and_then(toml::Value::as_array)
        .expect("G3 qualification tests must be exact");
    assert_eq!(
        qualification_tests
            .iter()
            .filter_map(toml::Value::as_str)
            .collect::<Vec<_>>(),
        [
            "real_agentd_remember_recall_correct_and_forget_revalidate_physical_sends",
            "two_real_agentd_app_servers_never_cross_recall",
            "unavailable_cognitive_store_keeps_read_tools_and_omits_write_tools",
        ]
    );
    assert_eq!(
        g3.get("qualification_lifecycle_actions")
            .and_then(toml::Value::as_array)
            .expect("G3 qualification lifecycle actions must be explicit")
            .iter()
            .filter_map(toml::Value::as_str)
            .collect::<Vec<_>>(),
        ["kill", "restart"]
    );
    for field in [
        "write_feature_default_enabled",
        "unavailable_runtime_mutation_tools_exposed",
        "cross_agent_memory_federation_qualified_in_g3",
        "matrix_or_robrix_qualified_in_g3",
        "fleet_lifecycle_qualified_in_g3",
        "automation_qualified_in_g3",
        "new_cross_agent_memory_federation_authority_added",
        "new_matrix_or_robrix_authority_added",
        "new_fleet_lifecycle_surface_added",
        "new_product_fleet_lifecycle_authority_added",
        "new_automation_authority_added",
        "provider_physical_sampling_exactly_once",
        "websocket_ephemeral_input_exact_conformance",
        "provider_send_linearizable_with_cognitive_mutation",
        "legacy_persistent_kg_database_compatibility",
        "old_binary_reopen_after_migration_supported",
        "whole_store_historical_reopen_scale_qualified",
        "operator_acceptance_recorded",
        "promotion_eligible",
    ] {
        assert_eq!(
            g3.get(field).and_then(toml::Value::as_bool),
            Some(false),
            "{field} must remain false in the bounded G3 qualification truth",
        );
    }
}
