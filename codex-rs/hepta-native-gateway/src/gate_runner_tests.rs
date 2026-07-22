use super::*;

fn repo_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("repo root")
}

#[test]
fn resolves_compatibility_scripts_inside_the_repo_only() {
    let gate = resolve_compatibility_script(
        repo_root(),
        "hepta-full-live-activation-closure-index",
        GateScriptKind::Gate,
    )
    .expect("registered route gate");
    let report = resolve_compatibility_script(
        repo_root(),
        "hepta-systems-controlled-live-readiness-audit",
        GateScriptKind::Report,
    )
    .expect("legacy report compatibility wrapper");

    assert!(gate.ends_with("scripts/hepta-full-live-activation-closure-index-route-gate.sh"));
    assert!(report.ends_with("scripts/hepta-systems-controlled-live-readiness-audit-report.sh"));
    assert!(
        resolve_compatibility_script(repo_root(), "../escape", GateScriptKind::Gate)
            .expect_err("path traversal must fail")
            .to_string()
            .contains("invalid Hepta gate id")
    );
}

#[test]
fn shell_catalog_derives_legacy_pair_counts_from_scripts() {
    let migrated_count = migrated_pair_specs().expect("migrated pair specs").len() as u64;
    let value: serde_json::Value = serde_json::from_str(
        &shell_gate_catalog_json_for_root(repo_root()).expect("shell catalog json"),
    )
    .expect("shell catalog value");

    assert_eq!(value["status"], "ready");
    assert_eq!(value["runner"], "hepta gate");
    assert!(value["gate_count"].as_u64().is_some_and(|count| count > 0));
    assert!(
        value["report_count"]
            .as_u64()
            .is_some_and(|count| count > 0)
    );
    assert!(
        value["exact_pair_count"]
            .as_u64()
            .is_some_and(|count| count > 0)
    );
    assert_eq!(value["thin_wrapper_pair_count"], migrated_count);
    assert_eq!(
        value["legacy_pair_count"].as_u64(),
        value["exact_pair_count"]
            .as_u64()
            .map(|count| count - migrated_count)
    );
    assert_eq!(value["execution_requires_explicit_flag"], true);
    assert_eq!(value["repo_root_required"], true);
}

#[test]
fn shell_snapshot_is_deterministic_and_content_addressed() {
    let migrated_count = migrated_pair_specs().expect("migrated pair specs").len() as u64;
    let first: serde_json::Value = serde_json::from_str(
        &shell_gate_snapshot_json_for_root(repo_root()).expect("first shell snapshot"),
    )
    .expect("first shell snapshot value");
    let second: serde_json::Value = serde_json::from_str(
        &shell_gate_snapshot_json_for_root(repo_root()).expect("second shell snapshot"),
    )
    .expect("second shell snapshot value");
    let catalog: serde_json::Value = serde_json::from_str(
        &shell_gate_catalog_json_for_root(repo_root()).expect("shell catalog"),
    )
    .expect("shell catalog value");

    assert_eq!(
        first["schema_version"],
        "hepta_shell_gate_parity_snapshot_v1"
    );
    assert_eq!(first["gate_count"], catalog["gate_count"]);
    assert_eq!(first["report_count"], catalog["report_count"]);
    assert_eq!(first["exact_pair_count"], catalog["exact_pair_count"]);
    assert_eq!(first["entry_count"], catalog["entry_count"]);
    assert_eq!(first["catalog_sha256"], second["catalog_sha256"]);
    assert_eq!(
        first["exact_pair_id_sha256"],
        second["exact_pair_id_sha256"]
    );
    assert_eq!(first["script_execution_performed"], false);
    assert_eq!(first["side_effect_free"], true);
    assert_eq!(first["thin_wrapper_pair_count"], migrated_count);
    assert_eq!(
        first["legacy_pair_count"].as_u64(),
        first["exact_pair_count"]
            .as_u64()
            .map(|count| count - migrated_count)
    );
    assert_eq!(first["catalog_sha256"].as_str().map(str::len), Some(64));
    assert_eq!(
        first["exact_pair_id_sha256"].as_str().map(str::len),
        Some(64)
    );
    assert!(first["entries"].as_array().is_some_and(|entries| {
        entries.iter().all(|entry| {
            let gate_ready = entry["gate_path"].is_null()
                || (entry["gate_path"]
                    .as_str()
                    .is_some_and(|path| path.starts_with("scripts/"))
                    && entry["gate_sha256"].as_str().map(str::len) == Some(64));
            let report_ready = entry["report_path"].is_null()
                || (entry["report_path"]
                    .as_str()
                    .is_some_and(|path| path.starts_with("scripts/"))
                    && entry["report_sha256"].as_str().map(str::len) == Some(64));
            gate_ready && report_ready
        })
    }));
    assert_eq!(
        first["entries"].as_array().map(|entries| entries
            .iter()
            .filter(|entry| entry["thin_wrapper_migrated"] == true)
            .count()),
        Some(migrated_count as usize)
    );
}

#[test]
fn migrated_pair_specs_use_the_receipt_state_machine() {
    let specs = migrated_pair_specs().expect("migrated pair specs");
    assert!(!specs.is_empty());
    assert!(specs.values().all(|spec| {
        ReceiptStateMachine::classify_fields(
            &spec.capability,
            &spec.source_report,
            &spec.side_effect_boundary,
        )
        .is_some_and(|state| state.as_str() == spec.receipt_state)
    }));
    let id = "hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-signing-final-ack-readback";
    let value: serde_json::Value = serde_json::from_str(
        &migrated_pair_spec_json(id)
            .expect("migrated pair lookup")
            .expect("migrated pair json"),
    )
    .expect("migrated pair value");
    assert_eq!(value["mode"], "declarative_shell_pair_migration");
    assert_eq!(value["template"], "signing_final_ack_readback");
    assert_eq!(value["receipt_state"], "terminal");
    assert_eq!(value["report_execution_performed"], false);
}

#[test]
fn shell_snapshot_matches_the_append_only_parity_ledger() {
    let snapshot: serde_json::Value = serde_json::from_str(
        &shell_gate_snapshot_json_for_root(repo_root()).expect("shell snapshot"),
    )
    .expect("shell snapshot value");
    let baseline_path =
        repo_root().join("docs/architecture/HEPTA_SHELL_GATE_PARITY_BASELINE_V1.json");
    let baseline: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(&baseline_path)
            .with_context(|| format!("failed to read {}", baseline_path.display()))
            .expect("parity baseline"),
    )
    .expect("parity baseline value");
    let ledger_path = repo_root().join("docs/architecture/HEPTA_SHELL_GATE_PARITY_LEDGER_V1.json");
    let ledger: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(&ledger_path)
            .with_context(|| format!("failed to read {}", ledger_path.display()))
            .expect("parity ledger"),
    )
    .expect("parity ledger value");
    let latest = ledger["batches"]
        .as_array()
        .and_then(|batches| batches.last())
        .expect("latest parity batch");

    for field in [
        "gate_count",
        "report_count",
        "exact_pair_count",
        "catalog_sha256",
        "exact_pair_id_sha256",
    ] {
        assert_eq!(
            ledger["baseline"][field], baseline[field],
            "baseline {field}"
        );
    }
    assert_eq!(snapshot["gate_count"], latest["gate_count"]);
    assert_eq!(snapshot["report_count"], latest["report_count"]);
    assert_eq!(snapshot["exact_pair_count"], latest["exact_pair_count"]);
    // The full catalog content hash is batch-local evidence: unrelated
    // gate/report fixes legitimately change it after the migration batch.
    // Pair membership is the durable parity contract across later commits.
    assert_eq!(
        snapshot["exact_pair_id_sha256"],
        latest["post_migration_exact_pair_id_sha256"]
    );
    assert_eq!(
        snapshot["thin_wrapper_pair_count"],
        latest["migrated_pair_count"]
    );
    assert_eq!(
        snapshot["legacy_pair_count"],
        latest["remaining_legacy_pair_count"]
    );
    assert_eq!(latest["remaining_legacy_pair_count"], 0);
    assert_eq!(latest["all_exact_pairs_thin_wrappers"], true);
    assert_eq!(latest["pre_migration_content_snapshot_ready"], true);
    assert_eq!(latest["compatibility_payload_all_hash_verified"], true);
    assert_eq!(latest["receipt_state_machine_derived"], true);
    assert_eq!(
        latest["captured_payload_source_hash_binding_count"].as_u64(),
        latest["captured_compatibility_pair_count"]
            .as_u64()
            .map(|count| count * 2)
    );
    assert_eq!(
        latest["existing_template_output_byte_parity_count"].as_u64(),
        latest["template_pair_count"]
            .as_u64()
            .map(|count| count * 2)
    );

    let migration_input_path =
        repo_root().join("docs/architecture/HEPTA_SHELL_GATE_MIGRATION_INPUT_V1.json");
    let migration_input: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(&migration_input_path)
            .with_context(|| format!("failed to read {}", migration_input_path.display()))
            .expect("migration input snapshot"),
    )
    .expect("migration input value");
    let input_entries = migration_input["entries"]
        .as_array()
        .expect("migration input entries")
        .iter()
        .filter(|entry| entry["exact_pair"] == true)
        .map(|entry| {
            (
                entry["id"]
                    .as_str()
                    .expect("migration input id")
                    .to_string(),
                entry,
            )
        })
        .collect::<BTreeMap<_, _>>();
    let specs = migrated_pair_specs().expect("migrated pair specs");
    assert_eq!(input_entries.len(), 1282);
    assert_eq!(specs.len(), input_entries.len());
    assert!(specs.keys().all(|id| input_entries.contains_key(id)));

    let captured_specs = specs
        .values()
        .filter(|spec| spec.template == "captured_shell_compat_v1")
        .collect::<Vec<_>>();
    assert_eq!(captured_specs.len(), 1255);
    for spec in captured_specs {
        let input = input_entries.get(&spec.id).expect("captured pair input");
        assert_eq!(
            spec.gate_source_sha256.as_deref(),
            input["gate_sha256"].as_str(),
            "captured gate source parity for {}",
            spec.id
        );
        assert_eq!(
            spec.report_source_sha256.as_deref(),
            input["report_sha256"].as_str(),
            "captured report source parity for {}",
            spec.id
        );
    }
}

#[test]
fn parameterized_route_gate_fixtures_match_the_canonical_gate_specs() {
    let fixture_path = repo_root().join("scripts/hepta-route-gate-specs-v1.json");
    let manifest: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(&fixture_path)
            .with_context(|| format!("failed to read {}", fixture_path.display()))
            .expect("route gate fixture registry"),
    )
    .expect("route gate fixture value");

    assert_eq!(manifest["schema_version"], "hepta_route_gate_specs_v1");
    assert_eq!(
        manifest["receipt_state_machine"],
        serde_json::json!(ReceiptStateMachine::ORDERED_STATES)
    );
    let fixtures = manifest["families"][0]["specs"]
        .as_array()
        .expect("first parameterized route gate family");
    assert_eq!(fixtures.len(), 3);

    for fixture in fixtures {
        let id = fixture["id"].as_str().expect("route gate fixture id");
        let gate_spec = &fixture["gate_spec"];
        let canonical = crate::route_registry::CONTROL_UI_ROUTE_SPECS
            .iter()
            .find(|spec| spec.capability == id)
            .unwrap_or_else(|| panic!("missing canonical GateSpec for {id}"));

        assert_eq!(gate_spec["method"], canonical.method);
        assert_eq!(gate_spec["pattern"], canonical.pattern);
        assert_eq!(gate_spec["source_command"], canonical.source_command);
        assert_eq!(gate_spec["capability"], canonical.capability);
        assert_eq!(
            gate_spec["side_effect_boundary"],
            canonical.side_effect_boundary
        );
        assert_eq!(
            gate_spec["receipt_state"].as_str(),
            canonical
                .receipt_state()
                .map(super::super::gate_spec::ReceiptState::as_str)
        );

        let wrapper_path = repo_root().join(format!("scripts/{id}-route-gate.sh"));
        let wrapper = fs::read_to_string(&wrapper_path)
            .with_context(|| format!("failed to read {}", wrapper_path.display()))
            .expect("parameterized route gate wrapper");
        assert_eq!(wrapper.lines().count(), 5, "thin wrapper for {id}");
        assert!(wrapper.contains("scripts/hepta-route-gate-runner"));
        assert!(wrapper.contains(id));
        assert!(
            fixture["baseline_normalized_output_sha256"]
                .as_str()
                .is_some_and(is_sha256)
        );
    }
}

#[test]
fn parameterized_core_activation_chain_fixtures_match_canonical_gate_specs() {
    let fixture_path = repo_root().join("scripts/hepta-core-activation-chain-gate-specs-v1.json");
    let manifest: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(&fixture_path)
            .with_context(|| format!("failed to read {}", fixture_path.display()))
            .expect("core activation chain fixture registry"),
    )
    .expect("core activation chain fixture value");

    assert_eq!(
        manifest["schema_version"],
        "hepta_core_activation_chain_gate_specs_v1"
    );
    assert_eq!(
        manifest["receipt_state_machine"],
        serde_json::json!(ReceiptStateMachine::ORDERED_STATES)
    );
    let fixtures = manifest["specs"]
        .as_array()
        .expect("parameterized core activation chain family");
    assert_eq!(fixtures.len(), CORE_ACTIVATION_CHAIN_GATE_SPECS.len());

    for fixture in fixtures {
        let id = fixture["id"]
            .as_str()
            .expect("core activation chain fixture id");
        let gate_spec = &fixture["gate_spec"];
        let canonical = CORE_ACTIVATION_CHAIN_GATE_SPECS
            .iter()
            .find(|spec| spec.capability == id)
            .unwrap_or_else(|| panic!("missing canonical GateSpec for {id}"));

        assert_eq!(gate_spec["method"], canonical.method);
        assert_eq!(gate_spec["pattern"], canonical.pattern);
        assert_eq!(gate_spec["source_command"], canonical.source_command);
        assert_eq!(gate_spec["capability"], canonical.capability);
        assert_eq!(
            gate_spec["side_effect_boundary"],
            canonical.side_effect_boundary
        );
        assert_eq!(
            gate_spec["receipt_state"].as_str(),
            canonical
                .receipt_state()
                .map(super::super::gate_spec::ReceiptState::as_str)
        );

        let wrapper_path = repo_root().join(format!("scripts/{id}-gate.sh"));
        let wrapper = fs::read_to_string(&wrapper_path)
            .with_context(|| format!("failed to read {}", wrapper_path.display()))
            .expect("parameterized core activation chain wrapper");
        assert_eq!(wrapper.lines().count(), 5, "thin wrapper for {id}");
        assert!(wrapper.contains("scripts/hepta-core-activation-chain-gate-runner"));
        assert!(wrapper.contains(id));
        assert!(
            fixture["baseline_normalized_output_sha256"]
                .as_str()
                .is_some_and(is_sha256)
        );
    }
}

#[test]
fn parameterized_provider_router_activation_chain_fixtures_match_canonical_gate_specs() {
    let fixture_path =
        repo_root().join("scripts/hepta-provider-router-activation-chain-gate-specs-v1.json");
    let manifest: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(&fixture_path)
            .with_context(|| format!("failed to read {}", fixture_path.display()))
            .expect("provider-router activation chain fixture registry"),
    )
    .expect("provider-router activation chain fixture value");

    assert_eq!(
        manifest["schema_version"],
        "hepta_provider_router_activation_chain_gate_specs_v1"
    );
    assert_eq!(
        manifest["receipt_state_machine"],
        serde_json::json!(ReceiptStateMachine::ORDERED_STATES)
    );
    let fixtures = manifest["specs"]
        .as_array()
        .expect("parameterized provider-router activation chain family");
    assert_eq!(
        fixtures.len(),
        PROVIDER_ROUTER_ACTIVATION_CHAIN_GATE_SPECS.len()
    );

    for fixture in fixtures {
        let id = fixture["id"]
            .as_str()
            .expect("provider-router activation chain fixture id");
        let gate_spec = &fixture["gate_spec"];
        let canonical = PROVIDER_ROUTER_ACTIVATION_CHAIN_GATE_SPECS
            .iter()
            .find(|spec| spec.capability == id)
            .unwrap_or_else(|| panic!("missing canonical GateSpec for {id}"));

        assert_eq!(gate_spec["method"], canonical.method);
        assert_eq!(gate_spec["pattern"], canonical.pattern);
        assert_eq!(gate_spec["source_command"], canonical.source_command);
        assert_eq!(gate_spec["capability"], canonical.capability);
        assert_eq!(
            gate_spec["side_effect_boundary"],
            canonical.side_effect_boundary
        );
        assert_eq!(
            gate_spec["receipt_state"].as_str(),
            canonical
                .receipt_state()
                .map(super::super::gate_spec::ReceiptState::as_str)
        );

        let wrapper_path = repo_root().join(format!("scripts/{id}-gate.sh"));
        let wrapper = fs::read_to_string(&wrapper_path)
            .with_context(|| format!("failed to read {}", wrapper_path.display()))
            .expect("parameterized provider-router activation chain wrapper");
        assert_eq!(wrapper.lines().count(), 5, "thin wrapper for {id}");
        assert!(wrapper.contains("scripts/hepta-provider-router-activation-chain-gate-runner"));
        assert!(wrapper.contains(id));
        assert!(
            fixture["baseline_normalized_output_sha256"]
                .as_str()
                .is_some_and(is_sha256)
        );
    }
}

#[test]
fn parameterized_operator_canary_activation_chain_fixtures_match_canonical_gate_specs() {
    let fixture_path =
        repo_root().join("scripts/hepta-operator-canary-activation-chain-gate-specs-v1.json");
    let manifest: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(&fixture_path)
            .with_context(|| format!("failed to read {}", fixture_path.display()))
            .expect("operator-canary activation chain fixture registry"),
    )
    .expect("operator-canary activation chain fixture value");

    assert_eq!(
        manifest["schema_version"],
        "hepta_operator_canary_activation_chain_gate_specs_v1"
    );
    assert_eq!(
        manifest["receipt_state_machine"],
        serde_json::json!(ReceiptStateMachine::ORDERED_STATES)
    );
    let fixtures = manifest["specs"]
        .as_array()
        .expect("parameterized operator-canary activation chain family");
    assert_eq!(
        fixtures.len(),
        OPERATOR_CANARY_ACTIVATION_CHAIN_GATE_SPECS.len()
    );

    for fixture in fixtures {
        let id = fixture["id"]
            .as_str()
            .expect("operator-canary activation chain fixture id");
        let gate_spec = &fixture["gate_spec"];
        let canonical = OPERATOR_CANARY_ACTIVATION_CHAIN_GATE_SPECS
            .iter()
            .find(|spec| spec.capability == id)
            .unwrap_or_else(|| panic!("missing canonical GateSpec for {id}"));

        assert_eq!(gate_spec["method"], canonical.method);
        assert_eq!(gate_spec["pattern"], canonical.pattern);
        assert_eq!(gate_spec["source_command"], canonical.source_command);
        assert_eq!(gate_spec["capability"], canonical.capability);
        assert_eq!(
            gate_spec["side_effect_boundary"],
            canonical.side_effect_boundary
        );
        assert_eq!(
            gate_spec["receipt_state"].as_str(),
            canonical
                .receipt_state()
                .map(super::super::gate_spec::ReceiptState::as_str)
        );

        let wrapper_path = repo_root().join(format!("scripts/{id}-gate.sh"));
        let wrapper = fs::read_to_string(&wrapper_path)
            .with_context(|| format!("failed to read {}", wrapper_path.display()))
            .expect("parameterized operator-canary activation chain wrapper");
        assert_eq!(wrapper.lines().count(), 5, "thin wrapper for {id}");
        assert!(wrapper.contains("scripts/hepta-operator-canary-activation-chain-gate-runner"));
        assert!(wrapper.contains(id));
        assert!(
            fixture["baseline_normalized_output_sha256"]
                .as_str()
                .is_some_and(is_sha256)
        );
    }
}

#[test]
fn parameterized_release_publication_result_receipt_fixtures_match_canonical_gate_specs() {
    let fixture_path =
        repo_root().join("scripts/hepta-release-publication-result-receipt-gate-specs-v1.json");
    let manifest: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(&fixture_path)
            .with_context(|| format!("failed to read {}", fixture_path.display()))
            .expect("release-publication result-receipt fixture registry"),
    )
    .expect("release-publication result-receipt fixture value");

    assert_eq!(
        manifest["schema_version"],
        "hepta_release_publication_result_receipt_gate_specs_v1"
    );
    assert_eq!(
        manifest["receipt_state_machine"],
        serde_json::json!(ReceiptStateMachine::ORDERED_STATES)
    );
    let fixtures = manifest["specs"]
        .as_array()
        .expect("parameterized release-publication result-receipt family");
    assert_eq!(
        fixtures.len(),
        RELEASE_PUBLICATION_RESULT_RECEIPT_GATE_SPECS.len()
    );

    for fixture in fixtures {
        let id = fixture["id"]
            .as_str()
            .expect("release-publication result-receipt fixture id");
        let gate_spec = &fixture["gate_spec"];
        let canonical = RELEASE_PUBLICATION_RESULT_RECEIPT_GATE_SPECS
            .iter()
            .find(|spec| spec.capability == id)
            .unwrap_or_else(|| panic!("missing canonical GateSpec for {id}"));

        assert_eq!(gate_spec["method"], canonical.method);
        assert_eq!(gate_spec["pattern"], canonical.pattern);
        assert_eq!(gate_spec["source_command"], canonical.source_command);
        assert_eq!(gate_spec["capability"], canonical.capability);
        assert_eq!(
            gate_spec["side_effect_boundary"],
            canonical.side_effect_boundary
        );
        assert_eq!(
            gate_spec["receipt_state"].as_str(),
            canonical
                .receipt_state()
                .map(super::super::gate_spec::ReceiptState::as_str)
        );

        let wrapper_path = repo_root().join(format!("scripts/{id}-gate.sh"));
        let wrapper = fs::read_to_string(&wrapper_path)
            .with_context(|| format!("failed to read {}", wrapper_path.display()))
            .expect("parameterized release-publication result-receipt wrapper");
        assert_eq!(wrapper.lines().count(), 5, "thin wrapper for {id}");
        assert!(wrapper.contains("scripts/hepta-release-publication-result-receipt-gate-runner"));
        assert!(wrapper.contains(id));
        assert!(
            fixture["baseline_normalized_output_sha256"]
                .as_str()
                .is_some_and(is_sha256)
        );
    }
}

#[test]
fn parameterized_artifact_download_install_affordance_result_receipt_fixtures_match_canonical_gate_specs()
 {
    let fixture_path = repo_root().join(
        "scripts/hepta-artifact-download-install-affordance-result-receipt-gate-specs-v1.json",
    );
    let manifest: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(&fixture_path)
            .with_context(|| format!("failed to read {}", fixture_path.display()))
            .expect("artifact-download/install-affordance result-receipt fixture registry"),
    )
    .expect("artifact-download/install-affordance result-receipt fixture value");

    assert_eq!(
        manifest["schema_version"],
        "hepta_artifact_download_install_affordance_result_receipt_gate_specs_v1"
    );
    assert_eq!(
        manifest["receipt_state_machine"],
        serde_json::json!(ReceiptStateMachine::ORDERED_STATES)
    );
    let fixtures = manifest["specs"]
        .as_array()
        .expect("parameterized artifact-download/install-affordance result-receipt family");
    assert_eq!(
        fixtures.len(),
        ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_GATE_SPECS.len()
    );

    for fixture in fixtures {
        let id = fixture["id"]
            .as_str()
            .expect("artifact-download/install-affordance result-receipt fixture id");
        let gate_spec = &fixture["gate_spec"];
        let canonical = ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_GATE_SPECS
            .iter()
            .find(|spec| spec.capability == id)
            .unwrap_or_else(|| panic!("missing canonical GateSpec for {id}"));

        assert_eq!(gate_spec["method"], canonical.method);
        assert_eq!(gate_spec["pattern"], canonical.pattern);
        assert_eq!(gate_spec["source_command"], canonical.source_command);
        assert_eq!(gate_spec["capability"], canonical.capability);
        assert_eq!(
            gate_spec["side_effect_boundary"],
            canonical.side_effect_boundary
        );
        assert_eq!(
            gate_spec["receipt_state"].as_str(),
            canonical
                .receipt_state()
                .map(super::super::gate_spec::ReceiptState::as_str)
        );

        let wrapper_path = repo_root().join(format!("scripts/{id}-gate.sh"));
        let wrapper = fs::read_to_string(&wrapper_path)
            .with_context(|| format!("failed to read {}", wrapper_path.display()))
            .expect("parameterized artifact-download/install-affordance result-receipt wrapper");
        assert_eq!(wrapper.lines().count(), 5, "thin wrapper for {id}");
        assert!(wrapper.contains(
            "scripts/hepta-artifact-download-install-affordance-result-receipt-gate-runner"
        ));
        assert!(wrapper.contains(id));
        assert!(
            fixture["baseline_normalized_output_sha256"]
                .as_str()
                .is_some_and(is_sha256)
        );
    }
}

#[test]
fn parameterized_artifact_download_install_affordance_result_receipt_route_fixtures_match_canonical_gate_specs()
 {
    let fixture_path = repo_root().join(
        "scripts/hepta-artifact-download-install-affordance-result-receipt-route-gate-specs-v1.json",
    );
    let manifest: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(&fixture_path)
            .with_context(|| format!("failed to read {}", fixture_path.display()))
            .expect("artifact-download/install-affordance result-receipt route fixture registry"),
    )
    .expect("artifact-download/install-affordance result-receipt route fixture value");

    assert_eq!(
        manifest["schema_version"],
        "hepta_artifact_download_install_affordance_result_receipt_route_gate_specs_v1"
    );
    assert_eq!(
        manifest["receipt_state_machine"],
        serde_json::json!(ReceiptStateMachine::ORDERED_STATES)
    );
    let fixtures = manifest["specs"]
        .as_array()
        .expect("parameterized artifact-download/install-affordance result-receipt route family");
    assert_eq!(fixtures.len(), 3);

    let unified_fixture_path = repo_root().join("scripts/hepta-route-gate-specs-v1.json");
    let unified_manifest: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(&unified_fixture_path)
            .with_context(|| format!("failed to read {}", unified_fixture_path.display()))
            .expect("unified route gate family registry"),
    )
    .expect("unified route gate family registry value");
    let unified_family = unified_manifest["families"]
        .as_array()
        .expect("unified route gate families")
        .iter()
        .find(|family| family["id"] == "artifact_download_install_affordance_result_receipt_v1")
        .expect("artifact download/install affordance family projected into unified registry");
    assert_eq!(
        unified_family["execution_profile"],
        "native_requirements_report_v1"
    );
    assert_eq!(unified_family["source_capture_mode"], "spec_source_v1");
    assert_eq!(
        unified_family["live_validation_mode"],
        "structured_fields_v1"
    );
    assert_eq!(unified_family["include_source_report_sha_field"], true);
    assert_eq!(unified_family["include_expected_route_count"], false);

    let compatibility_runner_path = repo_root().join(
        "scripts/hepta-artifact-download-install-affordance-result-receipt-route-gate-runner",
    );
    let compatibility_runner = fs::read_to_string(&compatibility_runner_path)
        .with_context(|| format!("failed to read {}", compatibility_runner_path.display()))
        .expect("artifact download/install affordance compatibility route runner");
    assert!(compatibility_runner.lines().count() <= 40);
    assert!(compatibility_runner.contains("scripts/hepta-route-gate-runner"));

    for fixture in fixtures {
        let id = fixture["id"]
            .as_str()
            .expect("artifact-download/install-affordance result-receipt route fixture id");
        let gate_spec = &fixture["gate_spec"];
        let canonical = crate::route_registry::CONTROL_UI_ROUTE_SPECS
            .iter()
            .find(|spec| spec.capability == id)
            .unwrap_or_else(|| panic!("missing canonical route GateSpec for {id}"));

        assert_eq!(gate_spec["method"], canonical.method);
        assert_eq!(gate_spec["pattern"], canonical.pattern);
        assert_eq!(gate_spec["source_command"], canonical.source_command);
        assert_eq!(gate_spec["capability"], canonical.capability);
        assert_eq!(
            gate_spec["side_effect_boundary"],
            canonical.side_effect_boundary
        );
        assert_eq!(
            gate_spec["receipt_state"].as_str(),
            canonical
                .receipt_state()
                .map(super::super::gate_spec::ReceiptState::as_str)
        );

        let wrapper_path = repo_root().join(
            fixture["wrapper"]
                .as_str()
                .expect("artifact-download/install-affordance result-receipt route wrapper path"),
        );
        let wrapper = fs::read_to_string(&wrapper_path)
            .with_context(|| format!("failed to read {}", wrapper_path.display()))
            .expect(
                "parameterized artifact-download/install-affordance result-receipt route wrapper",
            );
        assert_eq!(wrapper.lines().count(), 4, "thin route wrapper for {id}");
        assert!(wrapper.contains(
            "scripts/hepta-artifact-download-install-affordance-result-receipt-route-gate-runner"
        ));
        assert!(wrapper.contains(id));
        assert!(
            fixture["baseline_normalized_output_sha256"]
                .as_str()
                .is_some_and(is_sha256)
        );
    }
}

#[test]
fn parameterized_memory_live_mutation_activation_command_result_receipt_route_fixtures_match_canonical_gate_specs()
 {
    let fixture_path = repo_root().join(
        "scripts/hepta-memory-live-mutation-activation-command-result-receipt-route-gate-specs-v1.json",
    );
    let manifest: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(&fixture_path)
            .with_context(|| format!("failed to read {}", fixture_path.display()))
            .expect(
                "Memory live-mutation activation-command result-receipt route fixture registry",
            ),
    )
    .expect("Memory live-mutation activation-command result-receipt route fixture value");

    assert_eq!(
        manifest["schema_version"],
        "hepta_memory_live_mutation_activation_command_result_receipt_route_gate_specs_v1"
    );
    assert_eq!(
        manifest["receipt_state_machine"],
        serde_json::json!(ReceiptStateMachine::ORDERED_STATES)
    );
    let fixtures = manifest["specs"].as_array().expect(
        "parameterized Memory live-mutation activation-command result-receipt route family",
    );
    assert_eq!(fixtures.len(), 3);

    let unified_fixture_path = repo_root().join("scripts/hepta-route-gate-specs-v1.json");
    let unified_manifest: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(&unified_fixture_path)
            .with_context(|| format!("failed to read {}", unified_fixture_path.display()))
            .expect("unified route gate family registry"),
    )
    .expect("unified route gate family registry value");
    let unified_family = unified_manifest["families"]
        .as_array()
        .expect("unified route gate families")
        .iter()
        .find(|family| family["id"] == "memory_live_mutation_activation_command_result_receipt_v1")
        .expect("Memory live-mutation family projected into unified route registry");
    assert_eq!(
        unified_family["execution_profile"],
        "native_requirements_report_v1"
    );
    assert_eq!(
        unified_family["spec_registry"],
        "scripts/hepta-memory-live-mutation-activation-command-result-receipt-route-gate-specs-v1.json"
    );
    assert_eq!(
        unified_family["compatibility_executor"],
        "scripts/hepta-memory-live-mutation-activation-command-result-receipt-route-gate-runner"
    );

    let compatibility_runner_path = repo_root().join(
        "scripts/hepta-memory-live-mutation-activation-command-result-receipt-route-gate-runner",
    );
    let compatibility_runner = fs::read_to_string(&compatibility_runner_path)
        .with_context(|| format!("failed to read {}", compatibility_runner_path.display()))
        .expect("Memory live-mutation compatibility route runner");
    assert!(compatibility_runner.lines().count() <= 40);
    assert!(compatibility_runner.contains("scripts/hepta-route-gate-runner"));

    for fixture in fixtures {
        let id = fixture["id"]
            .as_str()
            .expect("Memory live-mutation activation-command result-receipt route fixture id");
        let gate_spec = &fixture["gate_spec"];
        let canonical = crate::route_registry::CONTROL_UI_ROUTE_SPECS
            .iter()
            .find(|spec| spec.capability == id)
            .unwrap_or_else(|| panic!("missing canonical route GateSpec for {id}"));

        assert_eq!(gate_spec["method"], canonical.method);
        assert_eq!(gate_spec["pattern"], canonical.pattern);
        assert_eq!(gate_spec["source_command"], canonical.source_command);
        assert_eq!(gate_spec["capability"], canonical.capability);
        assert_eq!(
            gate_spec["side_effect_boundary"],
            canonical.side_effect_boundary
        );
        assert_eq!(
            gate_spec["receipt_state"].as_str(),
            canonical
                .receipt_state()
                .map(super::super::gate_spec::ReceiptState::as_str)
        );

        let wrapper_path =
            repo_root().join(fixture["wrapper"].as_str().expect(
                "Memory live-mutation activation-command result-receipt route wrapper path",
            ));
        let wrapper = fs::read_to_string(&wrapper_path)
            .with_context(|| format!("failed to read {}", wrapper_path.display()))
            .expect(
                "parameterized Memory live-mutation activation-command result-receipt route wrapper",
            );
        assert_eq!(wrapper.lines().count(), 4, "thin route wrapper for {id}");
        assert!(wrapper.contains(
            "scripts/hepta-memory-live-mutation-activation-command-result-receipt-route-gate-runner"
        ));
        assert!(wrapper.contains(id));
        assert!(
            fixture["baseline_normalized_output_sha256"]
                .as_str()
                .is_some_and(is_sha256)
        );
    }
}

#[test]
fn parameterized_runtime_provider_router_activation_command_result_receipt_route_fixtures_match_canonical_gate_specs()
 {
    let fixture_path = repo_root().join(
        "scripts/hepta-runtime-provider-router-activation-command-result-receipt-route-gate-specs-v1.json",
    );
    let manifest: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(&fixture_path)
            .with_context(|| format!("failed to read {}", fixture_path.display()))
            .expect(
                "runtime provider-router activation-command result-receipt route fixture registry",
            ),
    )
    .expect("runtime provider-router activation-command result-receipt route fixture value");

    assert_eq!(
        manifest["schema_version"],
        "hepta_runtime_provider_router_activation_command_result_receipt_route_gate_specs_v1"
    );
    assert_eq!(
        manifest["receipt_state_machine"],
        serde_json::json!(ReceiptStateMachine::ORDERED_STATES)
    );
    let fixtures = manifest["specs"].as_array().expect(
        "parameterized runtime provider-router activation-command result-receipt route family",
    );
    assert_eq!(fixtures.len(), 3);

    let unified_fixture_path = repo_root().join("scripts/hepta-route-gate-specs-v1.json");
    let unified_manifest: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(&unified_fixture_path)
            .with_context(|| format!("failed to read {}", unified_fixture_path.display()))
            .expect("unified route gate family registry"),
    )
    .expect("unified route gate family registry value");
    let unified_family = unified_manifest["families"]
        .as_array()
        .expect("unified route gate families")
        .iter()
        .find(|family| {
            family["id"] == "runtime_provider_router_activation_command_result_receipt_v1"
        })
        .expect("runtime provider-router family projected into unified route registry");
    assert_eq!(
        unified_family["execution_profile"],
        "native_requirements_report_v1"
    );
    assert_eq!(
        unified_family["spec_registry"],
        "scripts/hepta-runtime-provider-router-activation-command-result-receipt-route-gate-specs-v1.json"
    );
    assert_eq!(unified_family["terminal_coverage_required"], false);
    assert_eq!(unified_family["include_route_gate_ready"], false);
    assert_eq!(unified_family["minimum_long_soak_samples"], 24);

    let compatibility_runner_path = repo_root().join(
        "scripts/hepta-runtime-provider-router-activation-command-result-receipt-route-gate-runner",
    );
    let compatibility_runner = fs::read_to_string(&compatibility_runner_path)
        .with_context(|| format!("failed to read {}", compatibility_runner_path.display()))
        .expect("runtime provider-router compatibility route runner");
    assert!(compatibility_runner.lines().count() <= 40);
    assert!(compatibility_runner.contains("scripts/hepta-route-gate-runner"));

    for fixture in fixtures {
        let id = fixture["id"]
            .as_str()
            .expect("runtime provider-router activation-command result-receipt route fixture id");
        let gate_spec = &fixture["gate_spec"];
        let canonical = crate::route_registry::CONTROL_UI_ROUTE_SPECS
            .iter()
            .find(|spec| spec.capability == id)
            .unwrap_or_else(|| panic!("missing canonical route GateSpec for {id}"));

        assert_eq!(gate_spec["method"], canonical.method);
        assert_eq!(gate_spec["pattern"], canonical.pattern);
        assert_eq!(gate_spec["source_command"], canonical.source_command);
        assert_eq!(gate_spec["capability"], canonical.capability);
        assert_eq!(
            gate_spec["side_effect_boundary"],
            canonical.side_effect_boundary
        );
        assert_eq!(
            gate_spec["receipt_state"].as_str(),
            canonical
                .receipt_state()
                .map(super::super::gate_spec::ReceiptState::as_str)
        );

        let wrapper_path = repo_root().join(
            fixture["wrapper"]
                .as_str()
                .expect("runtime provider-router result-receipt route wrapper path"),
        );
        let wrapper = fs::read_to_string(&wrapper_path)
            .with_context(|| format!("failed to read {}", wrapper_path.display()))
            .expect("parameterized runtime provider-router result-receipt route wrapper");
        assert_eq!(wrapper.lines().count(), 4, "thin route wrapper for {id}");
        assert!(wrapper.contains(
            "scripts/hepta-runtime-provider-router-activation-command-result-receipt-route-gate-runner"
        ));
        assert!(wrapper.contains(id));
        assert!(
            fixture["baseline_normalized_output_sha256"]
                .as_str()
                .is_some_and(is_sha256)
        );
    }
}

#[test]
fn parameterized_release_publication_result_receipt_route_fixtures_match_canonical_gate_specs() {
    let fixture_path = repo_root()
        .join("scripts/hepta-release-publication-result-receipt-route-gate-specs-v1.json");
    let manifest: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(&fixture_path)
            .with_context(|| format!("failed to read {}", fixture_path.display()))
            .expect("release-publication result-receipt route fixture registry"),
    )
    .expect("release-publication result-receipt route fixture value");

    assert_eq!(
        manifest["schema_version"],
        "hepta_release_publication_result_receipt_route_gate_specs_v1"
    );
    assert_eq!(
        manifest["receipt_state_machine"],
        serde_json::json!(ReceiptStateMachine::ORDERED_STATES)
    );
    let fixtures = manifest["specs"]
        .as_array()
        .expect("parameterized release-publication result-receipt route family");
    assert_eq!(fixtures.len(), 3);

    for fixture in fixtures {
        let id = fixture["id"]
            .as_str()
            .expect("release-publication result-receipt route fixture id");
        let gate_spec = &fixture["gate_spec"];
        let canonical = crate::route_registry::CONTROL_UI_ROUTE_SPECS
            .iter()
            .find(|spec| spec.capability == id)
            .unwrap_or_else(|| panic!("missing canonical route GateSpec for {id}"));

        assert_eq!(gate_spec["method"], canonical.method);
        assert_eq!(gate_spec["pattern"], canonical.pattern);
        assert_eq!(gate_spec["source_command"], canonical.source_command);
        assert_eq!(gate_spec["capability"], canonical.capability);
        assert_eq!(
            gate_spec["side_effect_boundary"],
            canonical.side_effect_boundary
        );
        assert_eq!(
            gate_spec["receipt_state"].as_str(),
            canonical
                .receipt_state()
                .map(super::super::gate_spec::ReceiptState::as_str)
        );

        let wrapper_path = repo_root().join(
            fixture["wrapper"]
                .as_str()
                .expect("release-publication result-receipt route wrapper path"),
        );
        let wrapper = fs::read_to_string(&wrapper_path)
            .with_context(|| format!("failed to read {}", wrapper_path.display()))
            .expect("parameterized release-publication result-receipt route wrapper");
        assert_eq!(wrapper.lines().count(), 4, "thin route wrapper for {id}");
        assert!(
            wrapper.contains("scripts/hepta-release-publication-result-receipt-route-gate-runner")
        );
        assert!(wrapper.contains(id));
        assert!(
            fixture["baseline_normalized_output_sha256"]
                .as_str()
                .is_some_and(is_sha256)
        );
    }
}

#[test]
fn parameterized_artifact_signing_receipt_route_fixtures_match_canonical_gate_specs() {
    let fixture_path =
        repo_root().join("scripts/hepta-artifact-signing-receipt-route-gate-specs-v1.json");
    let manifest: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(&fixture_path)
            .with_context(|| format!("failed to read {}", fixture_path.display()))
            .expect("artifact-signing receipt route fixture registry"),
    )
    .expect("artifact-signing receipt route fixture value");

    assert_eq!(
        manifest["schema_version"],
        "hepta_artifact_signing_receipt_route_gate_specs_v1"
    );
    assert_eq!(
        manifest["receipt_state_machine"],
        serde_json::json!(ReceiptStateMachine::ORDERED_STATES)
    );
    let fixtures = manifest["specs"]
        .as_array()
        .expect("parameterized artifact-signing receipt route family");
    assert_eq!(fixtures.len(), 3);

    for fixture in fixtures {
        let id = fixture["id"]
            .as_str()
            .expect("artifact-signing receipt route fixture id");
        let gate_spec = &fixture["gate_spec"];
        let canonical = crate::route_registry::CONTROL_UI_ROUTE_SPECS
            .iter()
            .find(|spec| spec.capability == id)
            .unwrap_or_else(|| panic!("missing canonical route GateSpec for {id}"));

        assert_eq!(gate_spec["method"], canonical.method);
        assert_eq!(gate_spec["pattern"], canonical.pattern);
        assert_eq!(gate_spec["source_command"], canonical.source_command);
        assert_eq!(gate_spec["capability"], canonical.capability);
        assert_eq!(
            gate_spec["side_effect_boundary"],
            canonical.side_effect_boundary
        );
        assert_eq!(
            gate_spec["receipt_state"].as_str(),
            canonical
                .receipt_state()
                .map(super::super::gate_spec::ReceiptState::as_str)
        );

        let wrapper_path = repo_root().join(
            fixture["wrapper"]
                .as_str()
                .expect("artifact-signing receipt route wrapper path"),
        );
        let wrapper = fs::read_to_string(&wrapper_path)
            .with_context(|| format!("failed to read {}", wrapper_path.display()))
            .expect("parameterized artifact-signing receipt route wrapper");
        assert_eq!(wrapper.lines().count(), 4, "thin route wrapper for {id}");
        assert!(wrapper.contains("scripts/hepta-artifact-signing-receipt-route-gate-runner"));
        assert!(wrapper.contains(id));
        assert!(
            fixture["baseline_normalized_output_sha256"]
                .as_str()
                .is_some_and(is_sha256)
        );
    }
}

#[test]
fn parameterized_durable_memory_receipt_boundary_route_fixtures_match_canonical_gate_specs() {
    let fixture_path =
        repo_root().join("scripts/hepta-durable-memory-receipt-boundary-route-gate-specs-v1.json");
    let manifest: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(&fixture_path)
            .with_context(|| format!("failed to read {}", fixture_path.display()))
            .expect("durable Memory receipt-boundary route fixture registry"),
    )
    .expect("durable Memory receipt-boundary route fixture value");

    assert_eq!(
        manifest["schema_version"],
        "hepta_durable_memory_receipt_boundary_route_gate_specs_v1"
    );
    assert_eq!(
        manifest["receipt_state_machine"],
        serde_json::json!(ReceiptStateMachine::ORDERED_STATES)
    );
    let fixtures = manifest["specs"]
        .as_array()
        .expect("parameterized durable Memory receipt-boundary route family");
    assert_eq!(fixtures.len(), 3);

    for fixture in fixtures {
        let id = fixture["id"]
            .as_str()
            .expect("durable Memory receipt-boundary route fixture id");
        let gate_spec = &fixture["gate_spec"];
        let canonical = crate::route_registry::CONTROL_UI_ROUTE_SPECS
            .iter()
            .find(|spec| spec.capability == id)
            .unwrap_or_else(|| panic!("missing canonical route GateSpec for {id}"));

        assert_eq!(gate_spec["method"], canonical.method);
        assert_eq!(gate_spec["pattern"], canonical.pattern);
        assert_eq!(gate_spec["source_command"], canonical.source_command);
        assert_eq!(gate_spec["capability"], canonical.capability);
        assert_eq!(
            gate_spec["side_effect_boundary"],
            canonical.side_effect_boundary
        );
        assert_eq!(
            gate_spec["receipt_state"].as_str(),
            canonical
                .receipt_state()
                .map(super::super::gate_spec::ReceiptState::as_str)
        );

        let wrapper_path = repo_root().join(
            fixture["wrapper"]
                .as_str()
                .expect("durable Memory receipt-boundary route wrapper path"),
        );
        let wrapper = fs::read_to_string(&wrapper_path)
            .with_context(|| format!("failed to read {}", wrapper_path.display()))
            .expect("parameterized durable Memory receipt-boundary route wrapper");
        assert_eq!(wrapper.lines().count(), 4, "thin route wrapper for {id}");
        assert!(
            wrapper.contains("scripts/hepta-durable-memory-receipt-boundary-route-gate-runner")
        );
        assert!(wrapper.contains(id));
        assert!(
            fixture["baseline_normalized_output_sha256"]
                .as_str()
                .is_some_and(is_sha256)
        );
    }
}

#[test]
fn parameterized_packet_acceptance_receipt_route_fixtures_match_canonical_gate_specs() {
    let fixture_path =
        repo_root().join("scripts/hepta-packet-acceptance-receipt-route-gate-specs-v1.json");
    let manifest: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(&fixture_path)
            .with_context(|| format!("failed to read {}", fixture_path.display()))
            .expect("packet-acceptance receipt route fixture registry"),
    )
    .expect("packet-acceptance receipt route fixture value");

    assert_eq!(
        manifest["schema_version"],
        "hepta_packet_acceptance_receipt_route_gate_specs_v1"
    );
    assert_eq!(
        manifest["receipt_state_machine"],
        serde_json::json!(ReceiptStateMachine::ORDERED_STATES)
    );
    let fixtures = manifest["specs"]
        .as_array()
        .expect("parameterized packet-acceptance receipt route family");
    assert_eq!(fixtures.len(), 3);

    for fixture in fixtures {
        let id = fixture["id"]
            .as_str()
            .expect("packet-acceptance receipt route fixture id");
        let gate_spec = &fixture["gate_spec"];
        let canonical = crate::route_registry::CONTROL_UI_ROUTE_SPECS
            .iter()
            .find(|spec| spec.capability == id)
            .unwrap_or_else(|| panic!("missing canonical route GateSpec for {id}"));

        assert_eq!(gate_spec["method"], canonical.method);
        assert_eq!(gate_spec["pattern"], canonical.pattern);
        assert_eq!(gate_spec["source_command"], canonical.source_command);
        assert_eq!(gate_spec["capability"], canonical.capability);
        assert_eq!(
            gate_spec["side_effect_boundary"],
            canonical.side_effect_boundary
        );
        assert_eq!(
            gate_spec["receipt_state"].as_str(),
            canonical
                .receipt_state()
                .map(super::super::gate_spec::ReceiptState::as_str)
        );

        let wrapper_path = repo_root().join(
            fixture["wrapper"]
                .as_str()
                .expect("packet-acceptance receipt route wrapper path"),
        );
        let wrapper = fs::read_to_string(&wrapper_path)
            .with_context(|| format!("failed to read {}", wrapper_path.display()))
            .expect("parameterized packet-acceptance receipt route wrapper");
        assert_eq!(wrapper.lines().count(), 4, "thin route wrapper for {id}");
        assert!(wrapper.contains("scripts/hepta-packet-acceptance-receipt-route-gate-runner"));
        assert!(wrapper.contains(id));
        assert!(
            fixture["baseline_normalized_output_sha256"]
                .as_str()
                .is_some_and(is_sha256)
        );
    }
}

#[test]
fn parameterized_operator_canary_receipt_route_fixtures_match_canonical_gate_specs() {
    let fixture_path =
        repo_root().join("scripts/hepta-operator-canary-receipt-route-gate-specs-v1.json");
    let manifest: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(&fixture_path)
            .with_context(|| format!("failed to read {}", fixture_path.display()))
            .expect("operator-canary receipt route fixture registry"),
    )
    .expect("operator-canary receipt route fixture value");

    assert_eq!(
        manifest["schema_version"],
        "hepta_operator_canary_receipt_route_gate_specs_v1"
    );
    assert_eq!(
        manifest["receipt_state_machine"],
        serde_json::json!(ReceiptStateMachine::ORDERED_STATES)
    );
    let fixtures = manifest["specs"]
        .as_array()
        .expect("parameterized operator-canary receipt route family");
    assert_eq!(fixtures.len(), 3);

    for fixture in fixtures {
        let id = fixture["id"]
            .as_str()
            .expect("operator-canary receipt route fixture id");
        let gate_spec = &fixture["gate_spec"];
        let canonical = crate::route_registry::CONTROL_UI_ROUTE_SPECS
            .iter()
            .find(|spec| spec.capability == id)
            .unwrap_or_else(|| panic!("missing canonical route GateSpec for {id}"));

        assert_eq!(gate_spec["method"], canonical.method);
        assert_eq!(gate_spec["pattern"], canonical.pattern);
        assert_eq!(gate_spec["source_command"], canonical.source_command);
        assert_eq!(gate_spec["capability"], canonical.capability);
        assert_eq!(
            gate_spec["side_effect_boundary"],
            canonical.side_effect_boundary
        );
        assert_eq!(
            gate_spec["receipt_state"].as_str(),
            canonical
                .receipt_state()
                .map(super::super::gate_spec::ReceiptState::as_str)
        );

        let wrapper_path = repo_root().join(
            fixture["wrapper"]
                .as_str()
                .expect("operator-canary receipt route wrapper path"),
        );
        let wrapper = fs::read_to_string(&wrapper_path)
            .with_context(|| format!("failed to read {}", wrapper_path.display()))
            .expect("parameterized operator-canary receipt route wrapper");
        assert_eq!(wrapper.lines().count(), 4, "thin route wrapper for {id}");
        assert!(wrapper.contains("scripts/hepta-operator-canary-receipt-route-gate-runner"));
        assert!(wrapper.contains(id));
        assert!(
            fixture["baseline_normalized_output_sha256"]
                .as_str()
                .is_some_and(is_sha256)
        );
    }
}

#[test]
fn parameterized_operator_identity_session_replay_reinstatement_route_fixtures_match_canonical_gate_specs()
 {
    let fixture_path = repo_root().join(
        "scripts/hepta-operator-identity-session-replay-reinstatement-route-gate-specs-v1.json",
    );
    let manifest: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(&fixture_path)
            .with_context(|| format!("failed to read {}", fixture_path.display()))
            .expect("operator identity/session replay/reinstatement route fixture registry"),
    )
    .expect("operator identity/session replay/reinstatement route fixture value");

    assert_eq!(
        manifest["schema_version"],
        "hepta_operator_identity_session_replay_reinstatement_route_gate_specs_v1"
    );
    assert_eq!(
        manifest["receipt_state_machine"],
        serde_json::json!(ReceiptStateMachine::ORDERED_STATES)
    );
    let fixtures = manifest["specs"]
        .as_array()
        .expect("parameterized operator identity/session replay/reinstatement route family");
    assert_eq!(fixtures.len(), 3);

    for fixture in fixtures {
        let id = fixture["id"]
            .as_str()
            .expect("operator identity/session replay/reinstatement route fixture id");
        let gate_spec = &fixture["gate_spec"];
        let canonical = crate::route_registry::CONTROL_UI_ROUTE_SPECS
            .iter()
            .find(|spec| spec.capability == id)
            .unwrap_or_else(|| panic!("missing canonical route GateSpec for {id}"));

        assert_eq!(gate_spec["method"], canonical.method);
        assert_eq!(gate_spec["pattern"], canonical.pattern);
        assert_eq!(gate_spec["source_command"], canonical.source_command);
        assert_eq!(gate_spec["capability"], canonical.capability);
        assert_eq!(
            gate_spec["side_effect_boundary"],
            canonical.side_effect_boundary
        );
        assert_eq!(
            gate_spec["receipt_state"].as_str(),
            canonical
                .receipt_state()
                .map(super::super::gate_spec::ReceiptState::as_str)
        );

        let wrapper_path = repo_root().join(
            fixture["wrapper"]
                .as_str()
                .expect("operator identity/session replay/reinstatement route wrapper path"),
        );
        let wrapper = fs::read_to_string(&wrapper_path)
            .with_context(|| format!("failed to read {}", wrapper_path.display()))
            .expect("parameterized operator identity/session replay/reinstatement route wrapper");
        assert_eq!(wrapper.lines().count(), 4, "thin route wrapper for {id}");
        assert!(wrapper.contains(
            "scripts/hepta-operator-identity-session-replay-reinstatement-route-gate-runner"
        ));
        assert!(wrapper.contains(id));
        assert!(
            fixture["baseline_normalized_output_sha256"]
                .as_str()
                .is_some_and(is_sha256)
        );
    }
}

#[test]
fn parameterized_memory_live_mutation_result_receipt_fixtures_match_canonical_gate_specs() {
    let fixture_path =
        repo_root().join("scripts/hepta-memory-live-mutation-result-receipt-gate-specs-v1.json");
    let manifest: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(&fixture_path)
            .with_context(|| format!("failed to read {}", fixture_path.display()))
            .expect("Memory live-mutation result-receipt fixture registry"),
    )
    .expect("Memory live-mutation result-receipt fixture value");

    assert_eq!(
        manifest["schema_version"],
        "hepta_memory_live_mutation_result_receipt_gate_specs_v1"
    );
    assert_eq!(
        manifest["receipt_state_machine"],
        serde_json::json!(ReceiptStateMachine::ORDERED_STATES)
    );
    let fixtures = manifest["specs"]
        .as_array()
        .expect("parameterized Memory live-mutation result-receipt family");
    assert_eq!(
        fixtures.len(),
        MEMORY_LIVE_MUTATION_RESULT_RECEIPT_GATE_SPECS.len()
    );

    for fixture in fixtures {
        let id = fixture["id"]
            .as_str()
            .expect("Memory live-mutation result-receipt fixture id");
        let gate_spec = &fixture["gate_spec"];
        let canonical = MEMORY_LIVE_MUTATION_RESULT_RECEIPT_GATE_SPECS
            .iter()
            .find(|spec| spec.capability == id)
            .unwrap_or_else(|| panic!("missing canonical GateSpec for {id}"));

        assert_eq!(gate_spec["method"], canonical.method);
        assert_eq!(gate_spec["pattern"], canonical.pattern);
        assert_eq!(gate_spec["source_command"], canonical.source_command);
        assert_eq!(gate_spec["capability"], canonical.capability);
        assert_eq!(
            gate_spec["side_effect_boundary"],
            canonical.side_effect_boundary
        );
        assert_eq!(
            gate_spec["receipt_state"].as_str(),
            canonical
                .receipt_state()
                .map(super::super::gate_spec::ReceiptState::as_str)
        );

        let wrapper_path = repo_root().join(format!("scripts/{id}-gate.sh"));
        let wrapper = fs::read_to_string(&wrapper_path)
            .with_context(|| format!("failed to read {}", wrapper_path.display()))
            .expect("parameterized Memory live-mutation result-receipt wrapper");
        assert_eq!(wrapper.lines().count(), 4, "thin wrapper for {id}");
        assert!(wrapper.contains("scripts/hepta-memory-live-mutation-result-receipt-gate-runner"));
        assert!(wrapper.contains(id));
        assert!(
            fixture["baseline_normalized_output_sha256"]
                .as_str()
                .is_some_and(is_sha256)
        );
    }
}

#[test]
fn parameterized_terminal_public_claim_delivery_receipt_fixtures_match_canonical_gate_specs() {
    let fixture_path =
        repo_root().join("scripts/hepta-terminal-public-claim-delivery-receipt-gate-specs-v1.json");
    let manifest: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(&fixture_path)
            .with_context(|| format!("failed to read {}", fixture_path.display()))
            .expect("terminal-public-claim delivery-receipt fixture registry"),
    )
    .expect("terminal-public-claim delivery-receipt fixture value");

    assert_eq!(
        manifest["schema_version"],
        "hepta_terminal_public_claim_delivery_receipt_gate_specs_v1"
    );
    assert_eq!(
        manifest["receipt_state_machine"],
        serde_json::json!(ReceiptStateMachine::ORDERED_STATES)
    );
    let fixtures = manifest["specs"]
        .as_array()
        .expect("parameterized terminal-public-claim delivery-receipt family");
    assert_eq!(
        fixtures.len(),
        TERMINAL_PUBLIC_CLAIM_DELIVERY_RECEIPT_GATE_SPECS.len()
    );

    for fixture in fixtures {
        let id = fixture["id"]
            .as_str()
            .expect("terminal-public-claim delivery-receipt fixture id");
        let gate_spec = &fixture["gate_spec"];
        let canonical = TERMINAL_PUBLIC_CLAIM_DELIVERY_RECEIPT_GATE_SPECS
            .iter()
            .find(|spec| spec.capability == id)
            .unwrap_or_else(|| panic!("missing canonical GateSpec for {id}"));

        assert_eq!(gate_spec["method"], canonical.method);
        assert_eq!(gate_spec["pattern"], canonical.pattern);
        assert_eq!(gate_spec["source_command"], canonical.source_command);
        assert_eq!(gate_spec["capability"], canonical.capability);
        assert_eq!(
            gate_spec["side_effect_boundary"],
            canonical.side_effect_boundary
        );
        assert_eq!(
            gate_spec["receipt_state"].as_str(),
            canonical
                .receipt_state()
                .map(super::super::gate_spec::ReceiptState::as_str)
        );

        let wrapper_path = repo_root().join(format!("scripts/{id}-gate.sh"));
        let wrapper = fs::read_to_string(&wrapper_path)
            .with_context(|| format!("failed to read {}", wrapper_path.display()))
            .expect("parameterized terminal-public-claim delivery-receipt wrapper");
        assert_eq!(wrapper.lines().count(), 3, "thin wrapper for {id}");
        assert!(
            wrapper.contains("scripts/hepta-terminal-public-claim-delivery-receipt-gate-runner")
        );
        assert!(wrapper.contains(id));
        assert!(
            fixture["baseline_normalized_output_sha256"]
                .as_str()
                .is_some_and(is_sha256)
        );
    }
}

#[test]
fn parameterized_packet_acceptance_receipt_fixtures_match_canonical_gate_specs() {
    let fixture_path =
        repo_root().join("scripts/hepta-packet-acceptance-receipt-gate-specs-v1.json");
    let manifest: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(&fixture_path)
            .with_context(|| format!("failed to read {}", fixture_path.display()))
            .expect("packet-acceptance receipt fixture registry"),
    )
    .expect("packet-acceptance receipt fixture value");

    assert_eq!(
        manifest["schema_version"],
        "hepta_packet_acceptance_receipt_gate_specs_v1"
    );
    assert_eq!(
        manifest["receipt_state_machine"],
        serde_json::json!(ReceiptStateMachine::ORDERED_STATES)
    );
    let fixtures = manifest["specs"]
        .as_array()
        .expect("parameterized packet-acceptance receipt family");
    assert_eq!(fixtures.len(), PACKET_ACCEPTANCE_RECEIPT_GATE_SPECS.len());

    for fixture in fixtures {
        let id = fixture["id"]
            .as_str()
            .expect("packet-acceptance receipt fixture id");
        let gate_spec = &fixture["gate_spec"];
        let canonical = PACKET_ACCEPTANCE_RECEIPT_GATE_SPECS
            .iter()
            .find(|spec| spec.capability == id)
            .unwrap_or_else(|| panic!("missing canonical GateSpec for {id}"));

        assert_eq!(gate_spec["method"], canonical.method);
        assert_eq!(gate_spec["pattern"], canonical.pattern);
        assert_eq!(gate_spec["source_command"], canonical.source_command);
        assert_eq!(gate_spec["capability"], canonical.capability);
        assert_eq!(
            gate_spec["side_effect_boundary"],
            canonical.side_effect_boundary
        );
        assert_eq!(
            gate_spec["receipt_state"].as_str(),
            canonical
                .receipt_state()
                .map(super::super::gate_spec::ReceiptState::as_str)
        );

        let wrapper_path = repo_root().join(format!("scripts/{id}-gate.sh"));
        let wrapper = fs::read_to_string(&wrapper_path)
            .with_context(|| format!("failed to read {}", wrapper_path.display()))
            .expect("parameterized packet-acceptance receipt wrapper");
        assert_eq!(wrapper.lines().count(), 5, "thin wrapper for {id}");
        assert!(wrapper.contains("scripts/hepta-packet-acceptance-receipt-gate-runner"));
        assert!(wrapper.contains(id));
        assert!(
            fixture["baseline_normalized_output_sha256"]
                .as_str()
                .is_some_and(is_sha256)
        );
    }
}

#[test]
fn parameterized_artifact_signing_receipt_fixtures_match_canonical_gate_specs() {
    let fixture_path =
        repo_root().join("scripts/hepta-artifact-signing-receipt-gate-specs-v1.json");
    let manifest: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(&fixture_path)
            .with_context(|| format!("failed to read {}", fixture_path.display()))
            .expect("artifact-signing receipt fixture registry"),
    )
    .expect("artifact-signing receipt fixture value");

    assert_eq!(
        manifest["schema_version"],
        "hepta_artifact_signing_receipt_gate_specs_v1"
    );
    assert_eq!(
        manifest["receipt_state_machine"],
        serde_json::json!(ReceiptStateMachine::ORDERED_STATES)
    );
    let fixtures = manifest["specs"]
        .as_array()
        .expect("parameterized artifact-signing receipt family");
    assert_eq!(fixtures.len(), ARTIFACT_SIGNING_RECEIPT_GATE_SPECS.len());

    for fixture in fixtures {
        let id = fixture["id"]
            .as_str()
            .expect("artifact-signing receipt fixture id");
        let gate_spec = &fixture["gate_spec"];
        let canonical = ARTIFACT_SIGNING_RECEIPT_GATE_SPECS
            .iter()
            .find(|spec| spec.capability == id)
            .unwrap_or_else(|| panic!("missing canonical GateSpec for {id}"));

        assert_eq!(gate_spec["method"], canonical.method);
        assert_eq!(gate_spec["pattern"], canonical.pattern);
        assert_eq!(gate_spec["source_command"], canonical.source_command);
        assert_eq!(gate_spec["capability"], canonical.capability);
        assert_eq!(
            gate_spec["side_effect_boundary"],
            canonical.side_effect_boundary
        );
        assert_eq!(
            gate_spec["receipt_state"].as_str(),
            canonical
                .receipt_state()
                .map(super::super::gate_spec::ReceiptState::as_str)
        );

        let wrapper_path = repo_root().join(format!("scripts/{id}-gate.sh"));
        let wrapper = fs::read_to_string(&wrapper_path)
            .with_context(|| format!("failed to read {}", wrapper_path.display()))
            .expect("parameterized artifact-signing receipt wrapper");
        assert_eq!(wrapper.lines().count(), 4, "thin wrapper for {id}");
        assert!(wrapper.contains("scripts/hepta-artifact-signing-receipt-gate-runner"));
        assert!(wrapper.contains(id));
        assert!(
            fixture["baseline_normalized_output_sha256"]
                .as_str()
                .is_some_and(is_sha256)
        );
    }
}

#[test]
fn parameterized_operator_identity_session_replay_reinstatement_fixtures_match_canonical_gate_specs()
 {
    let fixture_path = repo_root()
        .join("scripts/hepta-operator-identity-session-replay-reinstatement-gate-specs-v1.json");
    let manifest: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(&fixture_path)
            .with_context(|| format!("failed to read {}", fixture_path.display()))
            .expect("operator identity/session replay/reinstatement fixture registry"),
    )
    .expect("operator identity/session replay/reinstatement fixture value");

    assert_eq!(
        manifest["schema_version"],
        "hepta_operator_identity_session_replay_reinstatement_gate_specs_v1"
    );
    assert_eq!(
        manifest["receipt_state_machine"],
        serde_json::json!(ReceiptStateMachine::ORDERED_STATES)
    );
    let fixtures = manifest["specs"]
        .as_array()
        .expect("parameterized operator identity/session replay/reinstatement family");
    assert_eq!(
        fixtures.len(),
        OPERATOR_IDENTITY_SESSION_REPLAY_REINSTATEMENT_GATE_SPECS.len()
    );

    for fixture in fixtures {
        let id = fixture["id"]
            .as_str()
            .expect("operator identity/session replay/reinstatement fixture id");
        let gate_spec = &fixture["gate_spec"];
        let canonical = OPERATOR_IDENTITY_SESSION_REPLAY_REINSTATEMENT_GATE_SPECS
            .iter()
            .find(|spec| spec.capability == id)
            .unwrap_or_else(|| panic!("missing canonical GateSpec for {id}"));

        assert_eq!(gate_spec["method"], canonical.method);
        assert_eq!(gate_spec["pattern"], canonical.pattern);
        assert_eq!(gate_spec["source_command"], canonical.source_command);
        assert_eq!(gate_spec["capability"], canonical.capability);
        assert_eq!(
            gate_spec["side_effect_boundary"],
            canonical.side_effect_boundary
        );
        assert_eq!(
            gate_spec["receipt_state"].as_str(),
            canonical
                .receipt_state()
                .map(super::super::gate_spec::ReceiptState::as_str)
        );

        let wrapper_path = repo_root().join(format!("scripts/{id}-gate.sh"));
        let wrapper = fs::read_to_string(&wrapper_path)
            .with_context(|| format!("failed to read {}", wrapper_path.display()))
            .expect("parameterized operator identity/session replay/reinstatement wrapper");
        assert_eq!(wrapper.lines().count(), 4, "thin wrapper for {id}");
        assert!(
            wrapper.contains(
                "scripts/hepta-operator-identity-session-replay-reinstatement-gate-runner"
            )
        );
        assert!(wrapper.contains(id));
        assert!(
            fixture["baseline_normalized_output_sha256"]
                .as_str()
                .is_some_and(is_sha256)
        );
    }
}

#[test]
fn parameterized_durable_memory_dry_run_result_receipt_boundary_fixtures_match_canonical_gate_specs()
 {
    let fixture_path = repo_root()
        .join("scripts/hepta-durable-memory-dry-run-result-receipt-boundary-gate-specs-v1.json");
    let manifest: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(&fixture_path)
            .with_context(|| format!("failed to read {}", fixture_path.display()))
            .expect("durable Memory dry-run result-receipt boundary fixture registry"),
    )
    .expect("durable Memory dry-run result-receipt boundary fixture value");

    assert_eq!(
        manifest["schema_version"],
        "hepta_durable_memory_dry_run_result_receipt_boundary_gate_specs_v1"
    );
    assert_eq!(
        manifest["receipt_state_machine"],
        serde_json::json!(ReceiptStateMachine::ORDERED_STATES)
    );
    let fixtures = manifest["specs"]
        .as_array()
        .expect("parameterized durable Memory dry-run result-receipt boundary family");
    assert_eq!(
        fixtures.len(),
        DURABLE_MEMORY_DRY_RUN_RESULT_RECEIPT_BOUNDARY_GATE_SPECS.len()
    );

    for fixture in fixtures {
        let id = fixture["id"]
            .as_str()
            .expect("durable Memory dry-run result-receipt boundary fixture id");
        let gate_spec = &fixture["gate_spec"];
        let canonical = DURABLE_MEMORY_DRY_RUN_RESULT_RECEIPT_BOUNDARY_GATE_SPECS
            .iter()
            .find(|spec| spec.capability == id)
            .unwrap_or_else(|| panic!("missing canonical GateSpec for {id}"));

        assert_eq!(gate_spec["method"], canonical.method);
        assert_eq!(gate_spec["pattern"], canonical.pattern);
        assert_eq!(gate_spec["source_command"], canonical.source_command);
        assert_eq!(gate_spec["capability"], canonical.capability);
        assert_eq!(
            gate_spec["side_effect_boundary"],
            canonical.side_effect_boundary
        );
        assert_eq!(
            gate_spec["receipt_state"].as_str(),
            canonical
                .receipt_state()
                .map(super::super::gate_spec::ReceiptState::as_str)
        );

        let wrapper_path = repo_root().join(format!("scripts/{id}-gate.sh"));
        let wrapper = fs::read_to_string(&wrapper_path)
            .with_context(|| format!("failed to read {}", wrapper_path.display()))
            .expect("parameterized durable Memory dry-run result-receipt boundary wrapper");
        assert_eq!(wrapper.lines().count(), 4, "thin wrapper for {id}");
        assert!(
            wrapper.contains(
                "scripts/hepta-durable-memory-dry-run-result-receipt-boundary-gate-runner"
            )
        );
        assert!(wrapper.contains(id));
        assert!(
            fixture["baseline_normalized_output_sha256"]
                .as_str()
                .is_some_and(is_sha256)
        );
    }
}
