use super::*;

fn repo_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("repo root")
}

fn assert_route_gate_alias(wrapper_path: &Path, id: &str, runner: &str) {
    assert!(
        fs::symlink_metadata(wrapper_path)
            .expect("route gate alias metadata")
            .file_type()
            .is_symlink()
    );
    assert_eq!(
        fs::read_link(wrapper_path).expect("route gate alias target"),
        Path::new("hepta-route-gate-alias-launch")
    );

    let registry_path = repo_root().join("scripts/hepta-route-gate-aliases-v1.json");
    let registry: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(&registry_path)
            .with_context(|| format!("failed to read {}", registry_path.display()))
            .expect("route gate alias registry"),
    )
    .expect("route gate alias registry value");
    let logical_path = wrapper_path
        .strip_prefix(repo_root())
        .expect("route gate alias under repo root")
        .to_string_lossy();
    let entry = registry["entries"]
        .as_array()
        .expect("route gate alias entries")
        .iter()
        .find(|entry| entry["logical_path"] == logical_path.as_ref())
        .unwrap_or_else(|| panic!("missing route gate alias registry entry for {logical_path}"));
    assert_eq!(entry["id"], id);
    assert_eq!(entry["runner"], runner);
}

fn assert_gate_alias(wrapper_path: &Path, id: &str, runner: &str) {
    assert!(
        fs::symlink_metadata(wrapper_path)
            .expect("gate alias metadata")
            .file_type()
            .is_symlink()
    );
    assert_eq!(
        fs::read_link(wrapper_path).expect("gate alias target"),
        Path::new("hepta-gate-alias-launch")
    );

    let registry_path = repo_root().join("scripts/hepta-gate-aliases-v1.json");
    let registry: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(&registry_path)
            .with_context(|| format!("failed to read {}", registry_path.display()))
            .expect("gate alias registry"),
    )
    .expect("gate alias registry value");
    let logical_path = wrapper_path
        .strip_prefix(repo_root())
        .expect("gate alias under repo root")
        .to_string_lossy();
    let entry = registry["entries"]
        .as_array()
        .expect("gate alias entries")
        .iter()
        .find(|entry| entry["logical_path"] == logical_path.as_ref())
        .unwrap_or_else(|| panic!("missing gate alias registry entry for {logical_path}"));
    assert_eq!(entry["id"], id);
    assert_eq!(entry["runner"], runner);
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
fn migrated_and_promoted_pair_specs_use_the_receipt_state_machine() {
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

    let id = "hepta-systems-work-graph-unified-projection-enforcement-readiness-runtime-wal-write-boundary-execution-rerun-preview";
    let registry_path = repo_root().join("scripts/hepta-workgraph-source-report-specs-v1.json");
    let registry: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(&registry_path)
            .with_context(|| format!("failed to read {}", registry_path.display()))
            .expect("promoted WorkGraph registry"),
    )
    .expect("promoted WorkGraph registry value");
    let value = registry["entries"]
        .as_array()
        .expect("promoted WorkGraph registry entries")
        .iter()
        .find(|entry| entry["id"] == id)
        .expect("promoted WorkGraph pair");
    assert_eq!(
        registry["schema_version"],
        "hepta_workgraph_source_report_specs_v1"
    );
    assert_eq!(
        registry["execution_mode"],
        "source_report_smoke_plus_targeted_rust_test"
    );
    assert_eq!(
        value["source_stem"],
        "work_graph_unified_projection_enforcement_readiness_runtime_wal_write_boundary_execution_rerun_preview"
    );
    assert_eq!(value["report_sha256"].as_str().map(str::len), Some(64));
    assert!(
        value["compatibility_alias"]
            .as_str()
            .is_some_and(|path| path.ends_with(".report"))
    );
}

#[test]
fn supplemental_gate_pair_payloads_are_typed_and_fail_closed() {
    let mut payload = SupplementalPayloadSpec {
        path: "scripts/lib/hepta-gate-pair-compat-v1/example.gate".to_string(),
        sha256: "a".repeat(64),
        classification: "explicit_non_pair_compatibility_surface".to_string(),
        mutation_allowed: false,
        owner: "hepta-backend-maintainers".to_string(),
    };
    validate_supplemental_payloads(std::slice::from_ref(&payload))
        .expect("valid supplemental payload");

    payload.mutation_allowed = true;
    assert!(
        validate_supplemental_payloads(std::slice::from_ref(&payload))
            .expect_err("mutation-capable supplemental payload must fail closed")
            .to_string()
            .contains("invalid supplemental gate-pair payload policy")
    );
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
    assert_eq!(
        latest["migrated_pair_count"].as_u64(),
        Some(input_entries.len() as u64)
    );
    assert!(specs.keys().all(|id| input_entries.contains_key(id)));

    let retirement_path =
        repo_root().join("docs/architecture/HEPTA_GATE_COMPAT_O2_RETIREMENT_V1.json");
    let retirement: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(&retirement_path)
            .with_context(|| format!("failed to read {}", retirement_path.display()))
            .expect("O2 compatibility retirement receipt"),
    )
    .expect("O2 compatibility retirement receipt value");
    assert_eq!(retirement["schema"], "hepta_gate_compat_o2_retirement_v1");
    assert_eq!(retirement["status"], "ready");
    assert_eq!(
        retirement["source_commit"],
        "c8f1ad0e84539109415f1c57b6506a2620bb6b74"
    );
    assert_eq!(retirement["original_pair_count"], 598);
    assert_eq!(retirement["retired_pair_count"], 84);
    assert_eq!(retirement["migrated_pair_count"], 16);
    assert_eq!(retirement["remaining_pair_count"], 498);
    assert_eq!(
        retirement["original_pair_count"].as_u64(),
        retirement["retired_pair_count"]
            .as_u64()
            .zip(retirement["migrated_pair_count"].as_u64())
            .zip(retirement["remaining_pair_count"].as_u64())
            .map(|((retired, migrated), remaining)| retired + migrated + remaining)
    );

    let family_receipt_path = repo_root().join("scripts/hepta-gate-compat-family-receipt-v1.json");
    let family_receipt: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(&family_receipt_path)
            .with_context(|| format!("failed to read {}", family_receipt_path.display()))
            .expect("compatibility family receipt"),
    )
    .expect("compatibility family receipt value");
    assert_eq!(
        family_receipt["schema"],
        "hepta_gate_compat_family_current_baseline_receipt_v1"
    );
    assert_eq!(family_receipt["status"], "ready");
    assert_eq!(family_receipt["baseline"]["pair_count"], 498);
    assert_eq!(
        family_receipt["baseline"]["spec_sha256"],
        retirement["result_spec_sha256"]
    );
    assert_eq!(
        family_receipt["result"]["pair_count"].as_u64(),
        Some(specs.len() as u64)
    );

    let spec_path = repo_root().join("scripts/hepta-gate-pair-specs-v1.json");
    let spec_bytes = fs::read(&spec_path)
        .with_context(|| format!("failed to read {}", spec_path.display()))
        .expect("current gate-pair specs");
    assert_eq!(
        format!("{:x}", Sha256::digest(&spec_bytes)),
        family_receipt["result"]["spec_sha256"]
    );
    assert_eq!(
        snapshot["thin_wrapper_pair_count"].as_u64(),
        Some(specs.len() as u64)
    );
    assert_eq!(
        snapshot["physical_thin_wrapper_entrypoint_count"]
            .as_u64()
            .zip(snapshot["virtual_thin_wrapper_entrypoint_count"].as_u64())
            .map(|(physical, virtual_count)| physical + virtual_count),
        Some((specs.len() * 2) as u64)
    );
    assert_eq!(
        snapshot["legacy_pair_count"].as_u64(),
        snapshot["exact_pair_count"]
            .as_u64()
            .map(|exact| exact - specs.len() as u64)
    );

    let captured_specs = specs
        .values()
        .filter(|spec| spec.template == "captured_shell_compat_v1")
        .collect::<Vec<_>>();
    assert_eq!(
        family_receipt["result"]["captured_pair_count"].as_u64(),
        Some(captured_specs.len() as u64)
    );
    assert_eq!(
        specs
            .values()
            .filter(|spec| spec.template == "legacy_workgraph_projection_v1")
            .count(),
        0
    );
    assert_eq!(
        family_receipt["result"]["declarative_pair_count"].as_u64(),
        Some((specs.len() - captured_specs.len()) as u64)
    );
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
        assert_route_gate_alias(&wrapper_path, id, "scripts/hepta-route-gate-runner");
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
        assert_gate_alias(
            &wrapper_path,
            id,
            "scripts/hepta-core-activation-chain-gate-runner",
        );
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
        assert_gate_alias(
            &wrapper_path,
            id,
            "scripts/hepta-provider-router-activation-chain-gate-runner",
        );
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
        assert_gate_alias(
            &wrapper_path,
            id,
            "scripts/hepta-operator-canary-activation-chain-gate-runner",
        );
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
        assert_gate_alias(
            &wrapper_path,
            id,
            "scripts/hepta-release-publication-result-receipt-gate-runner",
        );
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
        assert_gate_alias(
            &wrapper_path,
            id,
            "scripts/hepta-artifact-download-install-affordance-result-receipt-gate-runner",
        );
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
        assert_route_gate_alias(
            &wrapper_path,
            id,
            "scripts/hepta-artifact-download-install-affordance-result-receipt-route-gate-runner",
        );
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
        assert_route_gate_alias(
            &wrapper_path,
            id,
            "scripts/hepta-memory-live-mutation-activation-command-result-receipt-route-gate-runner",
        );
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
        assert_route_gate_alias(
            &wrapper_path,
            id,
            "scripts/hepta-runtime-provider-router-activation-command-result-receipt-route-gate-runner",
        );
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
        assert_route_gate_alias(
            &wrapper_path,
            id,
            "scripts/hepta-release-publication-result-receipt-route-gate-runner",
        );
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
        assert_route_gate_alias(
            &wrapper_path,
            id,
            "scripts/hepta-artifact-signing-receipt-route-gate-runner",
        );
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
        assert_route_gate_alias(
            &wrapper_path,
            id,
            "scripts/hepta-durable-memory-receipt-boundary-route-gate-runner",
        );
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
        assert_route_gate_alias(
            &wrapper_path,
            id,
            "scripts/hepta-packet-acceptance-receipt-route-gate-runner",
        );
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
        assert_route_gate_alias(
            &wrapper_path,
            id,
            "scripts/hepta-operator-canary-receipt-route-gate-runner",
        );
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
        assert_route_gate_alias(
            &wrapper_path,
            id,
            "scripts/hepta-operator-identity-session-replay-reinstatement-route-gate-runner",
        );
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
        assert_gate_alias(
            &wrapper_path,
            id,
            "scripts/hepta-memory-live-mutation-result-receipt-gate-runner",
        );
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
        assert_gate_alias(
            &wrapper_path,
            id,
            "scripts/hepta-terminal-public-claim-delivery-receipt-gate-runner",
        );
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
        assert_gate_alias(
            &wrapper_path,
            id,
            "scripts/hepta-packet-acceptance-receipt-gate-runner",
        );
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
    let archive = GatePairArchive::load(repo_root(), SHELL_GATE_PAIR_SPECS_JSON.as_bytes())
        .expect("gate-pair archive");

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

        let logical_wrapper_path = format!("scripts/{id}-gate.sh");
        let wrapper_path = archive
            .long_path_entry(&logical_wrapper_path)
            .map(|entry| repo_root().join(entry.relocated_path()))
            .unwrap_or_else(|| repo_root().join(&logical_wrapper_path));
        if fs::symlink_metadata(&wrapper_path)
            .expect("artifact-signing alias metadata")
            .file_type()
            .is_symlink()
        {
            assert_eq!(
                fs::read_link(&wrapper_path).expect("artifact-signing alias"),
                Path::new("hepta-state-machine-gate-runner")
            );
        } else {
            let wrapper = fs::read_to_string(&wrapper_path)
                .with_context(|| format!("failed to read {}", wrapper_path.display()))
                .expect("parameterized artifact-signing receipt wrapper");
            assert_eq!(wrapper.lines().count(), 4, "thin wrapper for {id}");
            assert!(wrapper.contains("scripts/hepta-artifact-signing-receipt-gate-runner"));
            assert!(wrapper.contains(id));
        }
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
        assert_gate_alias(
            &wrapper_path,
            id,
            "scripts/hepta-operator-identity-session-replay-reinstatement-gate-runner",
        );
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
        assert_gate_alias(
            &wrapper_path,
            id,
            "scripts/hepta-durable-memory-dry-run-result-receipt-boundary-gate-runner",
        );
        assert!(
            fixture["baseline_normalized_output_sha256"]
                .as_str()
                .is_some_and(is_sha256)
        );
    }
}
