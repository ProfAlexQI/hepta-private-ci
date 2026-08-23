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

#[test]
fn g4_matrix_robrix_truth_matches_paired_exact_candidate_qualification() {
    let manifest = manifest();
    let g4 = manifest
        .qualification
        .iter()
        .find(|entry| {
            entry.get("slice").and_then(toml::Value::as_str) == Some("R2_matrix_robrix_closed_loop")
        })
        .expect("missing G4 Matrix/Robrix qualification truth");

    assert_eq!(
        g4.get("qualification_base_sha")
            .and_then(toml::Value::as_str),
        Some("e0c4b0bbf403af99143ad9691385aea1d4992fa0")
    );
    assert_eq!(
        g4.get("development_status").and_then(toml::Value::as_str),
        Some("qualified_exact_candidate")
    );
    assert_eq!(
        g4.get("real_synapse_runtime_skip_allowed")
            .and_then(toml::Value::as_bool),
        Some(false)
    );
    assert_eq!(
        g4.get("real_synapse_oci_image_ref")
            .and_then(toml::Value::as_str),
        Some(
            "matrixdotorg/synapse@sha256:467a587a5052dadd5d0bf1f8d89f043cc652d5201bca510307340f8dddb6b312"
        )
    );
    assert_eq!(
        g4.get("robrix_supervisord_allowed_methods")
            .and_then(toml::Value::as_array)
            .expect("G4 Robrix methods must be explicit")
            .iter()
            .filter_map(toml::Value::as_str)
            .collect::<Vec<_>>(),
        ["health", "roster", "snapshot"]
    );
    assert_eq!(
        g4.get("qualification_lifecycle_actions")
            .and_then(toml::Value::as_array)
            .expect("G4 lifecycle actions must be explicit")
            .iter()
            .filter_map(toml::Value::as_str)
            .collect::<Vec<_>>(),
        [
            "network_disconnect",
            "matrix_sidecar_sigkill",
            "agent_process_sigkill",
            "supervisor_managed_agent_restart_generation_rollover",
            "paired_stop",
        ]
    );

    for field in [
        "required_matrix_bridge_atomic_reconcile_and_add",
        "required_matrix_bridge_durable_client_binding",
        "required_matrix_bridge_durable_dispatch_owner_fence",
        "required_thread_hard_delete_durable_operation_journal",
        "required_thread_hard_delete_managed_agentd_single_writer_topology",
        "required_robrix_matrixd_control_second_local_confirmation",
        "required_generated_backend_protocol_projection",
        "bounded_ui_patch_ledger_required",
        "matrix_bridge_reconcile_response_revalidated",
        "stop_the_world_queue_dispatch_cutover_required",
    ] {
        assert_eq!(
            g4.get(field).and_then(toml::Value::as_bool),
            Some(true),
            "{field} must remain a hard G4 requirement",
        );
    }
    for field in [
        "matrix_bridge_atomic_reconcile_and_add_qualified",
        "matrix_bridge_durable_client_binding_qualified",
        "matrix_bridge_durable_dispatch_owner_fence_qualified",
        "thread_hard_delete_durable_operation_journal_qualified",
        "thread_hard_delete_managed_agentd_single_writer_topology_qualified",
        "matrix_bridge_conflicting_client_id_fails_before_atomic_admission",
        "dual_agent_e2ee_inbound_and_outbound",
        "matrix_transaction_id_deduplicated",
        "loopback_network_disconnect_recovery",
        "matrix_sidecar_restart_recovery",
        "agent_generation_rollover",
        "dual_agent_matrix_store_and_provider_isolation",
        "final_provider_and_room_counts_frozen_after_sidecar_stop",
        "robrix_matrixd_control_requires_second_local_confirmation",
        "generated_backend_protocol_projection",
        "matrix_companion_lifecycle_qualified_in_g4",
    ] {
        assert_eq!(
            g4.get(field).and_then(toml::Value::as_bool),
            Some(true),
            "{field} must be true after the paired exact-head G4 candidate passes",
        );
    }
    for field in [
        "generic_multi_app_server_hard_delete_qualified",
        "matrix_bridge_client_side_queue_turn_scan_authoritative",
        "matrix_bridge_queue_and_turn_all_pages_scanned",
        "old_binary_parallel_queue_dispatch_supported",
        "unbound_queue_exact_dispatch_qualified",
        "matrix_timeline_control_authority",
        "whole_tree_ui_merge_allowed",
        "general_fleet_lifecycle_qualified_in_g4",
        "cross_agent_memory_federation_qualified_in_g4",
        "automation_qualified_in_g4",
        "new_general_fleet_lifecycle_surface_added",
        "new_automation_authority_added",
        "provider_physical_sampling_exactly_once",
        "operator_acceptance_recorded",
        "promotion_eligible",
    ] {
        assert_eq!(
            g4.get(field).and_then(toml::Value::as_bool),
            Some(false),
            "{field} must stay false until its exact-head G4 gate passes",
        );
    }

    assert_eq!(
        g4.get("matrix_bridge_exact_bound_input_scope")
            .and_then(toml::Value::as_str),
        Some("text_only")
    );
    assert_eq!(
        g4.get("matrix_bridge_dispatch_scope")
            .and_then(toml::Value::as_str),
        Some("same_exact_head_same_host_shared_sqlite_home_exact_bindings")
    );
    assert_eq!(
        g4.get("thread_hard_delete_qualification_scope")
            .and_then(toml::Value::as_str),
        Some("agentd_managed_one_embedded_app_server_per_canonical_agent_home")
    );
    let worktree_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .expect("hepta-contracts must live under codex-rs");
    for callers_field in ["product_callers", "qualification_callers"] {
        for caller in g4
            .get(callers_field)
            .and_then(toml::Value::as_array)
            .unwrap_or_else(|| panic!("G4 {callers_field} must be explicit"))
            .iter()
            .map(|value| {
                value
                    .as_str()
                    .unwrap_or_else(|| panic!("G4 {callers_field} contains a non-string"))
            })
        {
            assert!(
                worktree_root.join(caller).is_file(),
                "G4 {callers_field} path does not exist: {caller}"
            );
        }
    }
}

#[test]
fn g5_fleet_automation_truth_matches_bounded_candidate_and_stays_fail_closed() {
    let manifest = manifest();
    let g5 = manifest
        .qualification
        .iter()
        .find(|entry| {
            entry.get("slice").and_then(toml::Value::as_str)
                == Some("R2_fleet_automation_bounded")
        })
        .expect("missing G5 fleet/automation bounded qualification truth");

    assert_eq!(
        g5.get("requires")
            .and_then(toml::Value::as_array)
            .expect("G5 predecessor requirement must be explicit")
            .iter()
            .filter_map(toml::Value::as_str)
            .collect::<Vec<_>>(),
        ["R2_matrix_robrix_closed_loop"]
    );
    for (field, expected) in [
        ("qualification_base_sha", "445d1cdc50c9e86d09041b17888245b8c5937bda"),
        ("qualification_base_tree", "2ba0062706e4bc652ee0433ef6b3b90696e3f1e3"),
        ("evidence_head", "73ff3b438a25d88201169aed7c7c79cf5d9644a8"),
        ("evidence_tree", "4070f421a63311c66a77d08491c4a9ab1fd52c65"),
        (
            "evidence_status_sha256",
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        ),
        (
            "evidence_aggregate_sha256",
            "57688579a2fe4bca6494cca8dcbc051d53f911086835be34618aafd5fda05cd7",
        ),
        (
            "evidence_authority_review_sha256",
            "12130e8c31fd0e2014782136ce9ff7a35024b0c3e265eb622c67a5173123221c",
        ),
        (
            "g4_pair_receipt_sha256",
            "f36ce3f41cc8734f4392070a01ac53cbdf753dee5a1bb8b352feb1bc886e8064",
        ),
        (
            "g4_pair_binding_sha256",
            "8d439937b36b60e573f3887c14e99801550c0a0c995da9646ec7690222ddee5f",
        ),
        (
            "g4_pair_verification_sha256",
            "206a62c39c664259792a39f98572df6f56eb3155a67d6d83caabb50249d3b7b5",
        ),
    ] {
        assert_eq!(
            g5.get(field).and_then(toml::Value::as_str),
            Some(expected),
            "G5 {field} must bind the immutable evidence packet",
        );
    }
    assert_eq!(
        g5.get("development_status").and_then(toml::Value::as_str),
        Some("qualified_exact_bounded_candidate")
    );
    assert_eq!(
        g5.get("product_caller_named").and_then(toml::Value::as_bool),
        Some(false)
    );
    for field in [
        "g5_minimum_six_bounded_gates_complete",
        "five_agent_process_isolation",
        "generation_cas_stale_fence_qualified",
        "explicit_memory_grant_revoke_qualified",
        "automation_occurrence_client_id_idempotency_qualified",
        "stale_lease_no_resurrection_qualified",
        "store_failure_peer_liveness_qualified",
        "target_agent_only_upgrade_and_explicit_rollback_qualified",
        "target_agent_only_automatic_rollback_qualified",
        "same_fence_concurrency_has_one_winner",
        "five_agent_daemon_restart_preserves_peer_runtime",
        "owner_written_consumer_read_only_and_scope_exact",
        "revoke_revalidated_before_physical_send",
        "five_agent_private_store_and_explicit_consumer_isolation",
        "target_agent_typed_storage_quarantine",
        "four_peer_process_ids_and_readiness_unchanged",
        "four_peer_automation_store_create_list_cancel_available",
        "four_peer_normal_app_server_turns_completed",
        "source_delta_test_only",
    ] {
        assert_eq!(
            g5.get(field).and_then(toml::Value::as_bool),
            Some(true),
            "G5 {field} must be explicitly bounded and receipt-backed",
        );
    }
    for field in [
        "general_fleet_lifecycle_qualified_in_g5",
        "cross_agent_memory_federation_qualified_in_g5",
        "fleet_or_automation_authority",
        "automation_authority",
        "new_general_fleet_lifecycle_surface_added",
        "new_product_fleet_lifecycle_authority_added",
        "new_automation_authority_added",
        "whole_tree_fleet_merge_allowed",
        "whole_tree_automation_merge_allowed",
        "provider_physical_sampling_exactly_once",
        "operator_acceptance_recorded",
        "promotion_eligible",
        "g5_complete",
        "g5_allowed",
        "fleet_and_automation_unfrozen",
    ] {
        assert_eq!(
            g5.get(field).and_then(toml::Value::as_bool),
            Some(false),
            "G5 {field} must remain false in the bounded evidence entry",
        );
    }

    let worktree_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .expect("hepta-contracts must live under codex-rs");
    let callers = g5
        .get("qualification_callers")
        .and_then(toml::Value::as_array)
        .expect("G5 qualification callers must be explicit");
    assert_eq!(callers.len(), 7);
    for caller in callers {
        let path = caller
            .as_str()
            .expect("G5 qualification caller must be a string");
        assert!(
            worktree_root.join(path).is_file(),
            "G5 qualification caller path does not exist: {path}"
        );
    }
    assert_eq!(
        g5.get("qualification_manifest_test")
            .and_then(toml::Value::as_str),
        Some("codex-rs/hepta-contracts/src/callers_manifest_tests.rs")
    );
}
