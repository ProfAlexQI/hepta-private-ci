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
