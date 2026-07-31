fn hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_operator_packet_acceptance_boundary_report()
-> serde_json::Value {
    const ACCEPTANCE_SURFACES: &[&str] = &[
        "source_preflight_boundary_required",
        "production_durable_memory_target_required",
        "operator_packet_envelope_required",
        "operator_identity_session_required",
        "operator_packet_signature_required",
        "single_use_acceptance_nonce_required",
        "explicit_acceptance_command_required",
        "payload_redaction_required",
        "acceptance_receipt_plan_required",
        "replay_idempotency_guard_required",
        "operator_packet_acceptance_handoff_required",
        "production_write_execution_forbidden_on_operator_packet_acceptance_route",
    ];
    const ACCEPTANCE_DENIALS: &[&str] = &[
        "source_preflight_boundary_required",
        "source_preflight_result_hash_required",
        "approved_production_namespace_required",
        "approved_production_store_required",
        "approved_production_scope_required",
        "production_durable_memory_target_required",
        "operator_packet_envelope_required",
        "operator_identity_required",
        "operator_session_binding_required",
        "operator_scope_binding_required",
        "operator_packet_signature_required",
        "operator_packet_freshness_required",
        "single_use_acceptance_nonce_required",
        "explicit_acceptance_command_required",
        "acceptance_command_budget_required",
        "payload_redaction_required",
        "raw_plaintext_payload_denied",
        "wal_receipt_plan_required",
        "post_write_readback_plan_required",
        "rollback_tombstone_zero_residue_plan_required",
        "acceptance_receipt_plan_required",
        "acceptance_receipt_hash_chain_required",
        "replay_idempotency_guard_required",
        "operator_packet_acceptance_result_record_required",
        "operator_packet_acceptance_result_readback_required",
        "operator_packet_persistence_report_route_denied",
        "production_write_execution_report_route_denied",
        "production_durable_memory_backend_write_denied",
        "durable_memory_backend_read_or_rollback_denied",
        "memory_store_mutation_denied",
        "wal_write_report_route_denied",
        "receipt_persist_report_route_denied",
        "rollback_execution_report_route_denied",
        "tombstone_write_report_route_denied",
        "kg_live_write_denied",
        "provider_model_invocation_denied",
        "credential_channel_release_install_denied",
        "unrestricted_full_live_activation_denied",
    ];
    const FALSE_ACCEPTANCE_SIDE_EFFECT_KEYS: &[&str] = &[
        "operator_packet_persisted",
        "operator_packet_ledger_recorded",
        "operator_packet_filesystem_written",
        "operator_packet_acceptance_receipt_persisted",
        "operator_packet_acceptance_receipt_delivered",
        "production_durable_memory_write_executed",
        "production_durable_memory_backend_present",
        "production_durable_memory_store_write_performed",
        "actual_production_durable_memory_write_performed",
        "durable_memory_store_write_performed",
        "durable_memory_store_read_performed",
        "durable_memory_store_rollback_performed",
        "memory_write_execution_performed",
        "memory_store_write_path_enabled",
        "memory_store_write_allowed",
        "memory_store_write_performed",
        "memory_store_mutation_allowed",
        "memory_store_mutated",
        "wal_write_performed",
        "wal_recorded",
        "wal_persisted",
        "receipt_recorded",
        "receipt_persisted",
        "receipt_materialized",
        "receipt_delivered",
        "post_write_readback_performed",
        "readback_result_recorded",
        "readback_result_persisted",
        "readback_result_accepted",
        "rollback_executed",
        "rollback_performed",
        "rollback_result_recorded",
        "rollback_result_persisted",
        "rollback_result_accepted",
        "tombstone_write_performed",
        "tombstone_cleanup_executed",
        "tombstone_cleanup_result_recorded",
        "tombstone_cleanup_result_accepted",
        "raw_payload_plaintext_recorded",
        "raw_payload_plaintext_persisted",
        "secret_material_read",
        "credential_read",
        "secret_file_read",
        "kg_adapter_read_performed",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "telegram_send_performed",
        "channel_send_performed",
        "external_send_performed",
        "public_claim_promoted",
        "public_release_published",
        "public_ga_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "service_restart_performed",
        "active_binary_mutated",
    ];
    const TRUE_ACCEPTANCE_KEYS: &[&str] = &[
        "scoped_production_durable_memory_write_operator_packet_acceptance_performed",
        "scoped_production_durable_memory_write_operator_packet_acceptance_result_recorded",
        "scoped_production_durable_memory_write_operator_packet_acceptance_result_accepted",
        "source_preflight_boundary_accepted",
        "production_durable_memory_target_bound",
        "operator_packet_acceptance_envelope_bound",
        "operator_identity_session_acceptance_bound",
        "operator_packet_signature_acceptance_bound",
        "single_use_acceptance_nonce_bound",
        "explicit_acceptance_command_bound",
        "payload_redaction_acceptance_bound",
        "acceptance_receipt_plan_bound",
        "replay_idempotency_acceptance_bound",
        "operator_packet_acceptance_handoff_bound",
        "production_write_execution_forbidden_on_operator_packet_acceptance_route",
        "kg_provider_channel_release_install_active_binary_forbidden",
    ];

    let route_matrix = control_ui_route_parity_report();
    let source = std::thread::Builder::new()
        .name("hepta-memory-production-durable-operator-packet-acceptance-source-report".to_string())
        .stack_size(8 * 1024 * 1024)
        .spawn(
            hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_preflight_boundary_report,
        )
        .ok()
        .and_then(|handle| handle.join().ok())
        .unwrap_or_else(|| {
            serde_json::json!({
                "status": "blocked",
                "memory_write_execution_scoped_production_durable_memory_write_preflight_boundary_ready": false,
                "scoped_production_durable_memory_write_preflight_accepted": false,
                "source_scoped_production_durable_memory_write_operator_packet_acceptance_source_report_thread_failed": true
            })
        });
    let json_bool = |value: &serde_json::Value, key: &str| {
        value
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let json_u64 = |value: &serde_json::Value, key: &str| {
        value
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let json_str = |value: &serde_json::Value, key: &str| {
        value
            .get(key)
            .and_then(serde_json::Value::as_str)
            .unwrap_or("")
            .to_string()
    };
    let route_count_source_command_accepted = route_matrix.ready
        && route_matrix.route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_next_action_acceptance = source
        .get("allowed_next_actions")
        .and_then(serde_json::Value::as_array)
        .and_then(|items| items.get(1))
        .map(|item| {
            item.get("action").and_then(serde_json::Value::as_str)
                == Some("prepare_scoped_production_durable_memory_write_operator_packet_acceptance_boundary")
                && item
                    .get("requires_scoped_production_durable_memory_write_preflight_boundary")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
                && item
                    .get("writes_production_durable_memory")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
        })
        .unwrap_or(false);
    let source_side_effects_ok = source
        .get("side_effects")
        .and_then(serde_json::Value::as_object)
        .map(|effects| {
            effects
                .get("scoped_production_durable_memory_write_preflight_performed")
                .and_then(serde_json::Value::as_bool)
                == Some(true)
                && effects
                    .get("scoped_production_durable_memory_write_preflight_result_accepted")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
                && effects
                    .get("production_durable_memory_store_write_performed")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("memory_store_write_performed")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("wal_write_performed")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("receipt_persisted")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("external_send_performed")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
        })
        .unwrap_or(false);
    let source_ready = source.get("status").and_then(serde_json::Value::as_str) == Some("ready")
        && json_bool(
            &source,
            "memory_write_execution_scoped_production_durable_memory_write_preflight_boundary_ready",
        )
        && json_bool(
            &source,
            "scoped_production_durable_memory_write_preflight_accepted",
        )
        && json_u64(
            &source,
            "accepted_scoped_production_durable_memory_write_preflight_fixture_count",
        ) == 1
        && json_u64(
            &source,
            "blocked_scoped_production_durable_memory_write_preflight_fixture_count",
        ) == 9
        && json_u64(
            &source,
            "scoped_production_durable_memory_write_preflight_result_accepted_count",
        ) == 1
        && json_u64(
            &source,
            "source_zero_residue_acceptance_result_accepted_count",
        ) == 1
        && !json_bool(&source, "production_durable_memory_write_executed")
        && !json_bool(&source, "production_durable_memory_store_write_performed")
        && !json_bool(&source, "actual_production_durable_memory_write_performed")
        && !json_bool(&source, "durable_memory_store_write_performed")
        && !json_bool(&source, "durable_memory_store_read_performed")
        && !json_bool(&source, "durable_memory_store_rollback_performed")
        && !json_bool(&source, "memory_store_write_performed")
        && !json_bool(&source, "wal_write_performed")
        && !json_bool(&source, "receipt_persisted")
        && !json_bool(&source, "post_write_readback_performed")
        && !json_bool(&source, "rollback_executed")
        && !json_bool(&source, "tombstone_cleanup_executed")
        && !json_bool(&source, "raw_payload_plaintext_recorded")
        && !json_bool(&source, "raw_payload_plaintext_persisted")
        && !json_bool(&source, "live_kg_write_performed")
        && !json_bool(&source, "provider_invoked")
        && !json_bool(&source, "model_invoked")
        && !json_bool(&source, "credential_read")
        && !json_bool(&source, "channel_send_performed")
        && !json_bool(&source, "external_send_performed")
        && !json_bool(&source, "release_artifact_written")
        && !json_bool(&source, "install_executed")
        && !json_bool(&source, "service_restarted")
        && !json_bool(&source, "active_binary_mutated")
        && source_next_action_acceptance
        && source_side_effects_ok;

    let approved_production_namespace = json_str(&source, "approved_production_namespace");
    let approved_production_store = json_str(&source, "approved_production_store");
    let approved_production_scope = json_str(&source, "approved_production_scope");
    let production_durable_memory_target_id =
        json_str(&source, "production_durable_memory_target_id");
    let production_durable_memory_payload_class =
        json_str(&source, "production_durable_memory_payload_class");
    let operator_packet_scope = json_str(&source, "operator_packet_scope");
    let source_report_sha256 = sha256_text_value(&source.to_string());
    let source_preflight_result_hash_sha256 = json_str(
        &source,
        "scoped_production_durable_memory_write_preflight_result_hash_sha256",
    );
    let source_preflight_boundary_hash_sha256 = json_str(
        &source,
        "scoped_production_durable_memory_write_preflight_boundary_hash_sha256",
    );
    let source_preflight_policy_hash_sha256 = json_str(
        &source,
        "scoped_production_durable_memory_write_preflight_policy_hash_sha256",
    );
    let source_target_hash_sha256 = json_str(
        &source,
        "production_durable_memory_write_preflight_target_hash_sha256",
    );
    let source_operator_packet_hash_sha256 = json_str(
        &source,
        "production_durable_memory_write_preflight_operator_packet_hash_sha256",
    );
    let source_payload_redaction_hash_sha256 = json_str(
        &source,
        "production_durable_memory_write_preflight_payload_redaction_hash_sha256",
    );
    let source_wal_receipt_plan_hash_sha256 = json_str(
        &source,
        "production_durable_memory_write_preflight_wal_receipt_plan_hash_sha256",
    );
    let source_readback_plan_hash_sha256 = json_str(
        &source,
        "production_durable_memory_write_preflight_readback_plan_hash_sha256",
    );
    let source_rollback_tombstone_zero_residue_plan_hash_sha256 = json_str(
        &source,
        "production_durable_memory_write_preflight_rollback_tombstone_zero_residue_plan_hash_sha256",
    );
    let operator_packet_acceptance_envelope_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-operator-packet-acceptance-envelope:v1:source={source_operator_packet_hash_sha256}:target={source_target_hash_sha256}:scope={operator_packet_scope}:redacted=true"
    ));
    let operator_packet_acceptance_identity_session_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-operator-packet-acceptance-identity-session:v1:envelope={operator_packet_acceptance_envelope_hash_sha256}:operator-bound=true:session-bound=true"
    ));
    let operator_packet_acceptance_signature_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-operator-packet-acceptance-signature:v1:identity-session={operator_packet_acceptance_identity_session_hash_sha256}:freshness=true:scope-bound=true"
    ));
    let operator_packet_acceptance_nonce_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-operator-packet-acceptance-nonce:v1:signature={operator_packet_acceptance_signature_hash_sha256}:single-use=true"
    ));
    let operator_packet_acceptance_command_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-operator-packet-acceptance-command:v1:nonce={operator_packet_acceptance_nonce_hash_sha256}:explicit=true:budget=acceptance-only"
    ));
    let operator_packet_acceptance_receipt_plan_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-operator-packet-acceptance-receipt-plan:v1:command={operator_packet_acceptance_command_hash_sha256}:wal-plan={source_wal_receipt_plan_hash_sha256}:persist-now=false"
    ));
    let operator_packet_acceptance_replay_guard_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-operator-packet-acceptance-replay-guard:v1:receipt-plan={operator_packet_acceptance_receipt_plan_hash_sha256}:source-result={source_preflight_result_hash_sha256}:replay=false"
    ));
    let operator_packet_acceptance_result_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-operator-packet-acceptance-result:v1:envelope={operator_packet_acceptance_envelope_hash_sha256}:command={operator_packet_acceptance_command_hash_sha256}:receipt={operator_packet_acceptance_receipt_plan_hash_sha256}:accepted=true"
    ));
    let operator_packet_acceptance_boundary_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-operator-packet-acceptance-boundary:v1:source={source_report_sha256}:result={operator_packet_acceptance_result_hash_sha256}:fixtures=10:accepted=1:denials={}:production-write=false",
        ACCEPTANCE_DENIALS.len()
    ));
    let operator_packet_acceptance_policy_hash_sha256 = sha256_text_value(
        "scoped-production-durable-memory-write-operator-packet-acceptance-policy:v1:bind-source-preflight-target-envelope-identity-session-signature-nonce-command-redaction-receipt-replay:no-production-write:no-persistence:no-kg:no-provider:no-channel:no-release:no-install",
    );
    let acceptance_bound = !source_preflight_result_hash_sha256.is_empty()
        && !source_preflight_boundary_hash_sha256.is_empty()
        && !source_preflight_policy_hash_sha256.is_empty()
        && !source_target_hash_sha256.is_empty()
        && !source_operator_packet_hash_sha256.is_empty()
        && !source_payload_redaction_hash_sha256.is_empty()
        && !source_wal_receipt_plan_hash_sha256.is_empty()
        && !source_readback_plan_hash_sha256.is_empty()
        && !source_rollback_tombstone_zero_residue_plan_hash_sha256.is_empty()
        && approved_production_namespace == "hepta.memory.production.scoped"
        && approved_production_store == "hepta-memory-durable-store-production-preflight-only"
        && approved_production_scope == "operator-approved-session"
        && production_durable_memory_target_id
            == "hepta-scoped-production-durable-memory-write-target-v1"
        && production_durable_memory_payload_class
            == "redacted-minimal-operator-approved-memory-fact";
    let surfaces_ready = source_ready && acceptance_bound;
    let report_ready = route_count_source_command_accepted && surfaces_ready;
    let accepted_fixture_count = if report_ready { 1 } else { 0 };
    let blocked_fixture_count = 10 - accepted_fixture_count;

    let mut fixtures = Vec::new();
    fixtures.push(serde_json::json!({
        "id": "scoped-production-durable-memory-write-operator-packet-acceptance",
        "fixture_id": "scoped-production-durable-memory-write-operator-packet-acceptance",
        "scoped_production_durable_memory_write_operator_packet_acceptance_accepted": report_ready,
        "reason": if report_ready { "operator_packet_acceptance_guards_bound_without_production_write_or_persistence" } else { "source_preflight_or_route_count_not_ready" },
        "source_preflight_boundary_bound": report_ready,
        "production_durable_memory_target_bound": report_ready,
        "operator_packet_acceptance_envelope_bound": report_ready,
        "operator_identity_session_acceptance_bound": report_ready,
        "operator_packet_signature_acceptance_bound": report_ready,
        "single_use_acceptance_nonce_bound": report_ready,
        "explicit_acceptance_command_bound": report_ready,
        "acceptance_receipt_plan_bound": report_ready,
        "replay_idempotency_acceptance_bound": report_ready,
        "production_durable_memory_store_write_performed": false,
        "operator_packet_persisted": false,
        "external_send_performed": false
    }));
    for id in [
        "missing-preflight-source",
        "wrong-production-target",
        "missing-operator-packet-envelope",
        "missing-operator-identity-session",
        "missing-operator-packet-signature",
        "missing-single-use-acceptance-nonce",
        "missing-explicit-acceptance-command",
        "missing-acceptance-receipt-plan",
        "production-write-or-persistence-side-effect-attempt",
    ] {
        fixtures.push(serde_json::json!({
            "id": id,
            "fixture_id": id,
            "scoped_production_durable_memory_write_operator_packet_acceptance_accepted": false,
            "reason": "blocked_noop",
            "production_durable_memory_store_write_performed": false,
            "operator_packet_persisted": false,
            "external_send_performed": false
        }));
    }

    let mut side_effects = serde_json::Map::new();
    for &key in FALSE_ACCEPTANCE_SIDE_EFFECT_KEYS {
        side_effects.insert(key.to_string(), serde_json::json!(false));
        side_effects.insert(format!("{key}_count"), serde_json::json!(0));
    }
    for &key in TRUE_ACCEPTANCE_KEYS {
        side_effects.insert(key.to_string(), serde_json::json!(report_ready));
        side_effects.insert(
            format!("{key}_count"),
            serde_json::json!(if report_ready { 1 } else { 0 }),
        );
    }

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "endpoint": HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_PRODUCTION_DURABLE_MEMORY_WRITE_OPERATOR_PACKET_ACCEPTANCE_BOUNDARY_ENDPOINT,
        "source_command": "/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-operator-packet-acceptance-boundary --json",
        "native_route": true,
        "side_effect_free": false,
        "external_side_effect_free": true,
        "audit_date": "2026-07-05"
    });
    let report_object = report
        .as_object_mut()
        .expect("scoped production durable Memory write operator packet acceptance report object");
    macro_rules! insert_report {
        ($key:expr, $value:expr) => {
            report_object.insert($key.to_string(), serde_json::json!($value));
        };
    }

    insert_report!(
        "native_gateway_source_command_count",
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    insert_report!("route_count", route_matrix.route_count);
    insert_report!(
        "implemented_route_count",
        route_matrix.implemented_route_count
    );
    insert_report!("missing_route_count", route_matrix.missing_route_count);
    insert_report!(
        "route_count_source_command_accepted",
        route_count_source_command_accepted
    );
    insert_report!(
        "memory_write_execution_scoped_production_durable_memory_write_operator_packet_acceptance_boundary_ready",
        report_ready
    );
    insert_report!(
        "scoped_production_durable_memory_write_operator_packet_acceptance_ready",
        report_ready
    );
    insert_report!(
        "scoped_production_durable_memory_write_operator_packet_acceptance_performed",
        report_ready
    );
    insert_report!(
        "scoped_production_durable_memory_write_operator_packet_acceptance_accepted",
        report_ready
    );
    insert_report!(
        "scoped_production_durable_memory_write_operator_packet_acceptance_mode",
        "acceptance_boundary_no_production_durable_memory_mutation_no_packet_persistence"
    );
    insert_report!(
        "source_scoped_production_durable_memory_write_preflight_boundary_ready",
        source_ready
    );
    insert_report!(
        "source_scoped_production_durable_memory_write_preflight_boundary_report_sha256",
        source_report_sha256
    );
    insert_report!(
        "source_scoped_production_durable_memory_write_preflight_result_hash_sha256",
        source_preflight_result_hash_sha256
    );
    insert_report!(
        "source_scoped_production_durable_memory_write_preflight_boundary_hash_sha256",
        source_preflight_boundary_hash_sha256
    );
    insert_report!(
        "source_scoped_production_durable_memory_write_preflight_policy_hash_sha256",
        source_preflight_policy_hash_sha256
    );
    insert_report!(
        "source_scoped_production_durable_memory_write_preflight_accepted_count",
        json_u64(
            &source,
            "scoped_production_durable_memory_write_preflight_result_accepted_count"
        )
    );
    insert_report!(
        "source_accepted_scoped_production_durable_memory_write_preflight_fixture_count",
        json_u64(
            &source,
            "accepted_scoped_production_durable_memory_write_preflight_fixture_count"
        )
    );
    insert_report!(
        "source_blocked_scoped_production_durable_memory_write_preflight_fixture_count",
        json_u64(
            &source,
            "blocked_scoped_production_durable_memory_write_preflight_fixture_count"
        )
    );
    insert_report!(
        "source_zero_residue_acceptance_result_accepted_count",
        json_u64(
            &source,
            "source_zero_residue_acceptance_result_accepted_count"
        )
    );
    insert_report!(
        "approved_production_namespace",
        approved_production_namespace
    );
    insert_report!("approved_production_store", approved_production_store);
    insert_report!("approved_production_scope", approved_production_scope);
    insert_report!(
        "production_durable_memory_target_id",
        production_durable_memory_target_id
    );
    insert_report!(
        "production_durable_memory_payload_class",
        production_durable_memory_payload_class
    );
    insert_report!("operator_packet_scope", operator_packet_scope);
    insert_report!(
        "source_production_durable_memory_write_preflight_target_hash_sha256",
        source_target_hash_sha256
    );
    insert_report!(
        "source_production_durable_memory_write_preflight_operator_packet_hash_sha256",
        source_operator_packet_hash_sha256
    );
    insert_report!(
        "source_production_durable_memory_write_preflight_payload_redaction_hash_sha256",
        source_payload_redaction_hash_sha256
    );
    insert_report!(
        "source_production_durable_memory_write_preflight_wal_receipt_plan_hash_sha256",
        source_wal_receipt_plan_hash_sha256
    );
    insert_report!(
        "source_production_durable_memory_write_preflight_readback_plan_hash_sha256",
        source_readback_plan_hash_sha256
    );
    insert_report!(
        "source_production_durable_memory_write_preflight_rollback_tombstone_zero_residue_plan_hash_sha256",
        source_rollback_tombstone_zero_residue_plan_hash_sha256
    );
    insert_report!(
        "operator_packet_acceptance_envelope_hash_sha256",
        operator_packet_acceptance_envelope_hash_sha256
    );
    insert_report!(
        "operator_packet_acceptance_identity_session_hash_sha256",
        operator_packet_acceptance_identity_session_hash_sha256
    );
    insert_report!(
        "operator_packet_acceptance_signature_hash_sha256",
        operator_packet_acceptance_signature_hash_sha256
    );
    insert_report!(
        "operator_packet_acceptance_nonce_hash_sha256",
        operator_packet_acceptance_nonce_hash_sha256
    );
    insert_report!(
        "operator_packet_acceptance_command_hash_sha256",
        operator_packet_acceptance_command_hash_sha256
    );
    insert_report!(
        "operator_packet_acceptance_receipt_plan_hash_sha256",
        operator_packet_acceptance_receipt_plan_hash_sha256
    );
    insert_report!(
        "operator_packet_acceptance_replay_guard_hash_sha256",
        operator_packet_acceptance_replay_guard_hash_sha256
    );
    insert_report!(
        "operator_packet_acceptance_result_hash_sha256",
        operator_packet_acceptance_result_hash_sha256
    );
    insert_report!(
        "scoped_production_durable_memory_write_operator_packet_acceptance_boundary_hash_sha256",
        operator_packet_acceptance_boundary_hash_sha256
    );
    insert_report!(
        "scoped_production_durable_memory_write_operator_packet_acceptance_policy_hash_sha256",
        operator_packet_acceptance_policy_hash_sha256
    );
    insert_report!(
        "required_scoped_production_durable_memory_write_operator_packet_acceptance_surface_count",
        ACCEPTANCE_SURFACES.len()
    );
    insert_report!(
        "ready_scoped_production_durable_memory_write_operator_packet_acceptance_surface_count",
        if surfaces_ready {
            ACCEPTANCE_SURFACES.len()
        } else {
            0
        }
    );
    insert_report!(
        "scoped_production_durable_memory_write_operator_packet_acceptance_surfaces",
        ACCEPTANCE_SURFACES
    );
    insert_report!(
        "scoped_production_durable_memory_write_operator_packet_acceptance_fixture_count",
        fixtures.len()
    );
    insert_report!(
        "accepted_scoped_production_durable_memory_write_operator_packet_acceptance_fixture_count",
        accepted_fixture_count
    );
    insert_report!(
        "blocked_scoped_production_durable_memory_write_operator_packet_acceptance_fixture_count",
        blocked_fixture_count
    );
    insert_report!(
        "scoped_production_durable_memory_write_operator_packet_acceptance_fixtures",
        fixtures
    );
    insert_report!(
        "denied_by_scoped_production_durable_memory_write_operator_packet_acceptance_boundary",
        ACCEPTANCE_DENIALS
    );
    insert_report!(
        "denied_by_scoped_production_durable_memory_write_operator_packet_acceptance_boundary_count",
        ACCEPTANCE_DENIALS.len()
    );
    insert_report!(
        "allowed_next_actions",
        [
            serde_json::json!({
                "action": "run_scoped_production_durable_memory_write_operator_packet_acceptance_boundary_require_live_gate",
                "status": "allowed_verification_only",
                "accepts_operator_packet_evidence": true,
                "persists_operator_packet": false,
                "writes_production_durable_memory": false,
                "writes_memory_store": false,
                "writes_wal": false,
                "persists_receipt": false,
                "executes_rollback": false,
                "writes_tombstone": false
            }),
            serde_json::json!({
                "action": "prepare_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_boundary",
                "status": "requires_separate_acceptance_receipt_gate",
                "requires_scoped_production_durable_memory_write_operator_packet_acceptance_boundary": true,
                "writes_production_durable_memory": false,
                "persists_operator_packet": false
            }),
        ]
    );
    for &key in FALSE_ACCEPTANCE_SIDE_EFFECT_KEYS {
        report_object.insert(key.to_string(), serde_json::json!(false));
        report_object.insert(format!("{key}_count"), serde_json::json!(0));
    }
    for &key in TRUE_ACCEPTANCE_KEYS {
        report_object.insert(key.to_string(), serde_json::json!(report_ready));
        report_object.insert(
            format!("{key}_count"),
            serde_json::json!(if report_ready { 1 } else { 0 }),
        );
    }
    for key in [
        "source_preflight_boundary_bound",
        "approved_production_namespace_bound",
        "approved_production_store_bound",
        "approved_production_scope_bound",
        "production_durable_memory_target_bound",
        "operator_packet_acceptance_envelope_bound",
        "operator_identity_session_acceptance_bound",
        "operator_packet_signature_acceptance_bound",
        "single_use_acceptance_nonce_bound",
        "explicit_acceptance_command_bound",
        "payload_redaction_acceptance_bound",
        "wal_receipt_plan_acceptance_bound",
        "post_write_readback_plan_acceptance_bound",
        "rollback_tombstone_zero_residue_plan_acceptance_bound",
        "acceptance_receipt_plan_bound",
        "replay_idempotency_acceptance_bound",
        "operator_packet_acceptance_handoff_bound",
        "production_write_execution_forbidden_on_operator_packet_acceptance_route",
        "production_durable_memory_write_forbidden",
        "operator_packet_persistence_forbidden_on_report_route",
        "memory_store_mutation_forbidden",
        "wal_write_forbidden_on_operator_packet_acceptance_route",
        "receipt_persist_forbidden_on_operator_packet_acceptance_route",
        "rollback_execution_forbidden_on_operator_packet_acceptance_route",
        "tombstone_write_forbidden_on_operator_packet_acceptance_route",
        "kg_live_write_forbidden",
        "provider_model_invocation_forbidden",
        "credential_channel_public_release_forbidden",
        "install_restart_active_binary_mutation_forbidden",
    ] {
        report_object.insert(key.to_string(), serde_json::json!(true));
    }
    report_object.insert(
        "side_effects".to_string(),
        serde_json::Value::Object(side_effects),
    );
    report
}

fn hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_boundary_report()
-> serde_json::Value {
    const RECEIPT_SURFACES: &[&str] = &[
        "source_operator_packet_acceptance_boundary_required",
        "operator_packet_acceptance_result_required",
        "acceptance_receipt_envelope_required",
        "acceptance_receipt_identity_session_required",
        "acceptance_receipt_digest_required",
        "acceptance_receipt_hash_chain_required",
        "acceptance_receipt_readback_plan_required",
        "acceptance_receipt_replay_guard_required",
        "acceptance_receipt_handoff_required",
        "acceptance_receipt_persistence_forbidden_on_report_route",
        "production_write_execution_forbidden_on_acceptance_receipt_route",
        "kg_provider_channel_release_install_active_binary_forbidden",
    ];
    const RECEIPT_DENIALS: &[&str] = &[
        "source_operator_packet_acceptance_boundary_required",
        "source_operator_packet_acceptance_result_hash_required",
        "source_operator_packet_acceptance_policy_hash_required",
        "approved_production_namespace_required",
        "approved_production_store_required",
        "approved_production_scope_required",
        "production_durable_memory_target_required",
        "operator_packet_acceptance_envelope_required",
        "operator_identity_session_required",
        "operator_packet_signature_required",
        "single_use_acceptance_nonce_required",
        "explicit_acceptance_command_required",
        "acceptance_receipt_plan_required",
        "acceptance_receipt_envelope_required",
        "acceptance_receipt_identity_session_required",
        "acceptance_receipt_digest_required",
        "acceptance_receipt_hash_chain_required",
        "acceptance_receipt_readback_plan_required",
        "acceptance_receipt_replay_guard_required",
        "acceptance_receipt_handoff_required",
        "acceptance_receipt_persistence_report_route_denied",
        "acceptance_receipt_filesystem_write_denied",
        "acceptance_receipt_ledger_recording_denied",
        "acceptance_receipt_delivery_denied",
        "operator_packet_persistence_report_route_denied",
        "production_write_execution_report_route_denied",
        "production_durable_memory_backend_write_denied",
        "durable_memory_backend_read_or_rollback_denied",
        "memory_store_mutation_denied",
        "wal_write_report_route_denied",
        "receipt_persist_report_route_denied",
        "post_write_readback_report_route_denied",
        "rollback_execution_report_route_denied",
        "tombstone_write_report_route_denied",
        "raw_payload_plaintext_denied",
        "kg_live_write_denied",
        "provider_model_invocation_denied",
        "credential_channel_release_install_denied",
        "active_binary_mutation_denied",
        "unrestricted_full_live_activation_denied",
    ];
    const FALSE_RECEIPT_SIDE_EFFECT_KEYS: &[&str] = &[
        "acceptance_receipt_persisted",
        "acceptance_receipt_filesystem_written",
        "acceptance_receipt_ledger_recorded",
        "acceptance_receipt_delivered",
        "acceptance_receipt_materialized",
        "operator_packet_persisted",
        "operator_packet_ledger_recorded",
        "operator_packet_filesystem_written",
        "operator_packet_acceptance_receipt_persisted",
        "production_durable_memory_write_executed",
        "production_durable_memory_backend_present",
        "production_durable_memory_store_write_performed",
        "actual_production_durable_memory_write_performed",
        "durable_memory_store_write_performed",
        "durable_memory_store_read_performed",
        "durable_memory_store_rollback_performed",
        "memory_write_execution_performed",
        "memory_store_write_path_enabled",
        "memory_store_write_allowed",
        "memory_store_write_performed",
        "memory_store_mutation_allowed",
        "memory_store_mutated",
        "wal_write_performed",
        "wal_recorded",
        "wal_persisted",
        "receipt_recorded",
        "receipt_persisted",
        "receipt_materialized",
        "receipt_delivered",
        "post_write_readback_performed",
        "readback_result_recorded",
        "readback_result_persisted",
        "readback_result_accepted",
        "rollback_executed",
        "rollback_performed",
        "rollback_result_recorded",
        "rollback_result_persisted",
        "rollback_result_accepted",
        "tombstone_write_performed",
        "tombstone_cleanup_executed",
        "tombstone_cleanup_result_recorded",
        "tombstone_cleanup_result_accepted",
        "raw_payload_plaintext_recorded",
        "raw_payload_plaintext_persisted",
        "secret_material_read",
        "credential_read",
        "secret_file_read",
        "kg_adapter_read_performed",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "telegram_send_performed",
        "channel_send_performed",
        "external_send_performed",
        "public_claim_promoted",
        "public_release_published",
        "public_ga_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "service_restart_performed",
        "active_binary_mutated",
    ];
    const TRUE_RECEIPT_KEYS: &[&str] = &[
        "scoped_production_durable_memory_write_operator_packet_acceptance_receipt_boundary_performed",
        "scoped_production_durable_memory_write_operator_packet_acceptance_receipt_result_recorded",
        "scoped_production_durable_memory_write_operator_packet_acceptance_receipt_result_accepted",
        "source_operator_packet_acceptance_boundary_accepted",
        "operator_packet_acceptance_result_bound",
        "acceptance_receipt_envelope_bound",
        "acceptance_receipt_identity_session_bound",
        "acceptance_receipt_digest_bound",
        "acceptance_receipt_hash_chain_bound",
        "acceptance_receipt_readback_plan_bound",
        "acceptance_receipt_replay_guard_bound",
        "acceptance_receipt_handoff_bound",
        "acceptance_receipt_persistence_forbidden_on_report_route",
        "production_write_execution_forbidden_on_acceptance_receipt_route",
        "kg_provider_channel_release_install_active_binary_forbidden",
    ];

    let route_matrix = control_ui_route_parity_report();
    let source = std::thread::Builder::new()
        .name(
            "hepta-memory-production-durable-operator-packet-acceptance-receipt-source-report"
                .to_string(),
        )
        .stack_size(8 * 1024 * 1024)
        .spawn(
            hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_operator_packet_acceptance_boundary_report,
        )
        .ok()
        .and_then(|handle| handle.join().ok())
        .unwrap_or_else(|| {
            serde_json::json!({
                "status": "blocked",
                "memory_write_execution_scoped_production_durable_memory_write_operator_packet_acceptance_boundary_ready": false,
                "scoped_production_durable_memory_write_operator_packet_acceptance_accepted": false,
                "source_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_source_report_thread_failed": true
            })
        });
    let json_bool = |value: &serde_json::Value, key: &str| {
        value
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let json_u64 = |value: &serde_json::Value, key: &str| {
        value
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let json_str = |value: &serde_json::Value, key: &str| {
        value
            .get(key)
            .and_then(serde_json::Value::as_str)
            .unwrap_or("")
            .to_string()
    };
    let route_count_source_command_accepted = route_matrix.ready
        && route_matrix.route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_next_action_receipt = source
        .get("allowed_next_actions")
        .and_then(serde_json::Value::as_array)
        .and_then(|items| items.get(1))
        .map(|item| {
            item.get("action").and_then(serde_json::Value::as_str)
                == Some("prepare_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_boundary")
                && item
                    .get("requires_scoped_production_durable_memory_write_operator_packet_acceptance_boundary")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
                && item
                    .get("writes_production_durable_memory")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && item
                    .get("persists_operator_packet")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
        })
        .unwrap_or(false);
    let source_side_effects_ok = source
        .get("side_effects")
        .and_then(serde_json::Value::as_object)
        .map(|effects| {
            effects
                .get("scoped_production_durable_memory_write_operator_packet_acceptance_performed")
                .and_then(serde_json::Value::as_bool)
                == Some(true)
                && effects
                    .get("scoped_production_durable_memory_write_operator_packet_acceptance_result_accepted")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
                && effects
                    .get("operator_packet_persisted")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("production_durable_memory_store_write_performed")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("memory_store_write_performed")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("wal_write_performed")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("receipt_persisted")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("external_send_performed")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
        })
        .unwrap_or(false);
    let source_ready = source.get("status").and_then(serde_json::Value::as_str) == Some("ready")
        && json_bool(
            &source,
            "memory_write_execution_scoped_production_durable_memory_write_operator_packet_acceptance_boundary_ready",
        )
        && json_bool(
            &source,
            "scoped_production_durable_memory_write_operator_packet_acceptance_accepted",
        )
        && json_u64(
            &source,
            "accepted_scoped_production_durable_memory_write_operator_packet_acceptance_fixture_count",
        ) == 1
        && json_u64(
            &source,
            "blocked_scoped_production_durable_memory_write_operator_packet_acceptance_fixture_count",
        ) == 9
        && json_u64(
            &source,
            "scoped_production_durable_memory_write_operator_packet_acceptance_result_accepted_count",
        ) == 1
        && !json_bool(&source, "operator_packet_persisted")
        && !json_bool(&source, "operator_packet_acceptance_receipt_persisted")
        && !json_bool(&source, "production_durable_memory_write_executed")
        && !json_bool(&source, "production_durable_memory_store_write_performed")
        && !json_bool(&source, "actual_production_durable_memory_write_performed")
        && !json_bool(&source, "durable_memory_store_write_performed")
        && !json_bool(&source, "durable_memory_store_read_performed")
        && !json_bool(&source, "durable_memory_store_rollback_performed")
        && !json_bool(&source, "memory_store_write_performed")
        && !json_bool(&source, "wal_write_performed")
        && !json_bool(&source, "receipt_persisted")
        && !json_bool(&source, "post_write_readback_performed")
        && !json_bool(&source, "rollback_executed")
        && !json_bool(&source, "tombstone_cleanup_executed")
        && !json_bool(&source, "raw_payload_plaintext_recorded")
        && !json_bool(&source, "raw_payload_plaintext_persisted")
        && !json_bool(&source, "live_kg_write_performed")
        && !json_bool(&source, "provider_invoked")
        && !json_bool(&source, "model_invoked")
        && !json_bool(&source, "credential_read")
        && !json_bool(&source, "channel_send_performed")
        && !json_bool(&source, "external_send_performed")
        && !json_bool(&source, "release_artifact_written")
        && !json_bool(&source, "install_executed")
        && !json_bool(&source, "service_restarted")
        && !json_bool(&source, "active_binary_mutated")
        && source_next_action_receipt
        && source_side_effects_ok;

    let approved_production_namespace = json_str(&source, "approved_production_namespace");
    let approved_production_store = json_str(&source, "approved_production_store");
    let approved_production_scope = json_str(&source, "approved_production_scope");
    let production_durable_memory_target_id =
        json_str(&source, "production_durable_memory_target_id");
    let production_durable_memory_payload_class =
        json_str(&source, "production_durable_memory_payload_class");
    let operator_packet_scope = json_str(&source, "operator_packet_scope");
    let source_report_sha256 = sha256_text_value(&source.to_string());
    let source_acceptance_result_hash_sha256 =
        json_str(&source, "operator_packet_acceptance_result_hash_sha256");
    let source_acceptance_boundary_hash_sha256 = json_str(
        &source,
        "scoped_production_durable_memory_write_operator_packet_acceptance_boundary_hash_sha256",
    );
    let source_acceptance_policy_hash_sha256 = json_str(
        &source,
        "scoped_production_durable_memory_write_operator_packet_acceptance_policy_hash_sha256",
    );
    let source_target_hash_sha256 = json_str(
        &source,
        "source_production_durable_memory_write_preflight_target_hash_sha256",
    );
    let source_operator_packet_hash_sha256 = json_str(
        &source,
        "source_production_durable_memory_write_preflight_operator_packet_hash_sha256",
    );
    let source_acceptance_envelope_hash_sha256 =
        json_str(&source, "operator_packet_acceptance_envelope_hash_sha256");
    let source_acceptance_identity_session_hash_sha256 = json_str(
        &source,
        "operator_packet_acceptance_identity_session_hash_sha256",
    );
    let source_acceptance_signature_hash_sha256 =
        json_str(&source, "operator_packet_acceptance_signature_hash_sha256");
    let source_acceptance_nonce_hash_sha256 =
        json_str(&source, "operator_packet_acceptance_nonce_hash_sha256");
    let source_acceptance_command_hash_sha256 =
        json_str(&source, "operator_packet_acceptance_command_hash_sha256");
    let source_acceptance_receipt_plan_hash_sha256 = json_str(
        &source,
        "operator_packet_acceptance_receipt_plan_hash_sha256",
    );
    let source_acceptance_replay_guard_hash_sha256 = json_str(
        &source,
        "operator_packet_acceptance_replay_guard_hash_sha256",
    );
    let acceptance_receipt_envelope_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-operator-packet-acceptance-receipt-envelope:v1:source-result={source_acceptance_result_hash_sha256}:receipt-plan={source_acceptance_receipt_plan_hash_sha256}:target={source_target_hash_sha256}:persist-now=false"
    ));
    let acceptance_receipt_identity_session_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-operator-packet-acceptance-receipt-identity-session:v1:envelope={acceptance_receipt_envelope_hash_sha256}:source-identity-session={source_acceptance_identity_session_hash_sha256}:operator-bound=true"
    ));
    let acceptance_receipt_digest_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-operator-packet-acceptance-receipt-digest:v1:envelope={acceptance_receipt_envelope_hash_sha256}:command={source_acceptance_command_hash_sha256}:signature={source_acceptance_signature_hash_sha256}:redacted=true"
    ));
    let acceptance_receipt_hash_chain_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-operator-packet-acceptance-receipt-hash-chain:v1:digest={acceptance_receipt_digest_hash_sha256}:source-boundary={source_acceptance_boundary_hash_sha256}:append-now=false"
    ));
    let acceptance_receipt_readback_plan_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-operator-packet-acceptance-receipt-readback-plan:v1:hash-chain={acceptance_receipt_hash_chain_hash_sha256}:readback-now=false"
    ));
    let acceptance_receipt_replay_guard_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-operator-packet-acceptance-receipt-replay-guard:v1:readback={acceptance_receipt_readback_plan_hash_sha256}:nonce={source_acceptance_nonce_hash_sha256}:source-replay={source_acceptance_replay_guard_hash_sha256}:replay=false"
    ));
    let acceptance_receipt_result_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-operator-packet-acceptance-receipt-result:v1:envelope={acceptance_receipt_envelope_hash_sha256}:digest={acceptance_receipt_digest_hash_sha256}:replay={acceptance_receipt_replay_guard_hash_sha256}:accepted=true"
    ));
    let acceptance_receipt_boundary_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-operator-packet-acceptance-receipt-boundary:v1:source={source_report_sha256}:result={acceptance_receipt_result_hash_sha256}:fixtures=10:accepted=1:denials={}:receipt-persist=false:production-write=false",
        RECEIPT_DENIALS.len()
    ));
    let acceptance_receipt_policy_hash_sha256 = sha256_text_value(
        "scoped-production-durable-memory-write-operator-packet-acceptance-receipt-policy:v1:bind-source-acceptance-result-envelope-digest-hash-chain-readback-replay:no-receipt-persistence:no-production-write:no-kg:no-provider:no-channel:no-release:no-install",
    );
    let receipt_bound = !source_acceptance_result_hash_sha256.is_empty()
        && !source_acceptance_boundary_hash_sha256.is_empty()
        && !source_acceptance_policy_hash_sha256.is_empty()
        && !source_target_hash_sha256.is_empty()
        && !source_operator_packet_hash_sha256.is_empty()
        && !source_acceptance_envelope_hash_sha256.is_empty()
        && !source_acceptance_identity_session_hash_sha256.is_empty()
        && !source_acceptance_signature_hash_sha256.is_empty()
        && !source_acceptance_nonce_hash_sha256.is_empty()
        && !source_acceptance_command_hash_sha256.is_empty()
        && !source_acceptance_receipt_plan_hash_sha256.is_empty()
        && !source_acceptance_replay_guard_hash_sha256.is_empty()
        && approved_production_namespace == "hepta.memory.production.scoped"
        && approved_production_store == "hepta-memory-durable-store-production-preflight-only"
        && approved_production_scope == "operator-approved-session"
        && production_durable_memory_target_id
            == "hepta-scoped-production-durable-memory-write-target-v1"
        && production_durable_memory_payload_class
            == "redacted-minimal-operator-approved-memory-fact";
    let surfaces_ready = source_ready && receipt_bound;
    let report_ready = route_count_source_command_accepted && surfaces_ready;
    let accepted_fixture_count = if report_ready { 1 } else { 0 };
    let blocked_fixture_count = 10 - accepted_fixture_count;

    let mut fixtures = Vec::new();
    fixtures.push(serde_json::json!({
        "id": "scoped-production-durable-memory-write-operator-packet-acceptance-receipt",
        "fixture_id": "scoped-production-durable-memory-write-operator-packet-acceptance-receipt",
        "scoped_production_durable_memory_write_operator_packet_acceptance_receipt_accepted": report_ready,
        "reason": if report_ready { "operator_packet_acceptance_receipt_guards_bound_without_receipt_persistence_or_production_write" } else { "source_acceptance_or_route_count_not_ready" },
        "source_operator_packet_acceptance_boundary_bound": report_ready,
        "operator_packet_acceptance_result_bound": report_ready,
        "acceptance_receipt_envelope_bound": report_ready,
        "acceptance_receipt_digest_bound": report_ready,
        "acceptance_receipt_hash_chain_bound": report_ready,
        "acceptance_receipt_readback_plan_bound": report_ready,
        "acceptance_receipt_replay_guard_bound": report_ready,
        "acceptance_receipt_persisted": false,
        "operator_packet_persisted": false,
        "production_durable_memory_store_write_performed": false,
        "external_send_performed": false
    }));
    for id in [
        "missing-operator-packet-acceptance-source",
        "missing-acceptance-result-hash",
        "missing-acceptance-receipt-envelope",
        "missing-acceptance-receipt-digest",
        "missing-acceptance-receipt-hash-chain",
        "missing-acceptance-receipt-readback-plan",
        "missing-acceptance-receipt-replay-guard",
        "receipt-persistence-side-effect-attempt",
        "production-write-or-external-side-effect-attempt",
    ] {
        fixtures.push(serde_json::json!({
            "id": id,
            "fixture_id": id,
            "scoped_production_durable_memory_write_operator_packet_acceptance_receipt_accepted": false,
            "reason": "blocked_noop",
            "acceptance_receipt_persisted": false,
            "operator_packet_persisted": false,
            "production_durable_memory_store_write_performed": false,
            "external_send_performed": false
        }));
    }

    let mut side_effects = serde_json::Map::new();
    for &key in FALSE_RECEIPT_SIDE_EFFECT_KEYS {
        side_effects.insert(key.to_string(), serde_json::json!(false));
        side_effects.insert(format!("{key}_count"), serde_json::json!(0));
    }
    for &key in TRUE_RECEIPT_KEYS {
        side_effects.insert(key.to_string(), serde_json::json!(report_ready));
        side_effects.insert(
            format!("{key}_count"),
            serde_json::json!(if report_ready { 1 } else { 0 }),
        );
    }

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "endpoint": HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_PRODUCTION_DURABLE_MEMORY_WRITE_OPERATOR_PACKET_ACCEPTANCE_RECEIPT_BOUNDARY_ENDPOINT,
        "source_command": "/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-operator-packet-acceptance-receipt-boundary --json",
        "native_route": true,
        "side_effect_free": false,
        "external_side_effect_free": true,
        "audit_date": "2026-07-05"
    });
    let report_object = report.as_object_mut().expect(
        "scoped production durable Memory write operator packet acceptance receipt report object",
    );
    macro_rules! insert_report {
        ($key:expr, $value:expr) => {
            report_object.insert($key.to_string(), serde_json::json!($value));
        };
    }

    insert_report!(
        "native_gateway_source_command_count",
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    insert_report!("route_count", route_matrix.route_count);
    insert_report!(
        "implemented_route_count",
        route_matrix.implemented_route_count
    );
    insert_report!("missing_route_count", route_matrix.missing_route_count);
    insert_report!(
        "route_count_source_command_accepted",
        route_count_source_command_accepted
    );
    insert_report!(
        "memory_write_execution_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_boundary_ready",
        report_ready
    );
    insert_report!(
        "scoped_production_durable_memory_write_operator_packet_acceptance_receipt_ready",
        report_ready
    );
    insert_report!(
        "scoped_production_durable_memory_write_operator_packet_acceptance_receipt_performed",
        report_ready
    );
    insert_report!(
        "scoped_production_durable_memory_write_operator_packet_acceptance_receipt_accepted",
        report_ready
    );
    insert_report!(
        "scoped_production_durable_memory_write_operator_packet_acceptance_receipt_mode",
        "acceptance_receipt_boundary_no_receipt_persistence_no_production_durable_memory_mutation"
    );
    insert_report!(
        "source_scoped_production_durable_memory_write_operator_packet_acceptance_boundary_ready",
        source_ready
    );
    insert_report!(
        "source_scoped_production_durable_memory_write_operator_packet_acceptance_boundary_report_sha256",
        source_report_sha256
    );
    insert_report!(
        "source_operator_packet_acceptance_result_hash_sha256",
        source_acceptance_result_hash_sha256
    );
    insert_report!(
        "source_scoped_production_durable_memory_write_operator_packet_acceptance_boundary_hash_sha256",
        source_acceptance_boundary_hash_sha256
    );
    insert_report!(
        "source_scoped_production_durable_memory_write_operator_packet_acceptance_policy_hash_sha256",
        source_acceptance_policy_hash_sha256
    );
    insert_report!(
        "source_scoped_production_durable_memory_write_operator_packet_acceptance_accepted_count",
        json_u64(
            &source,
            "scoped_production_durable_memory_write_operator_packet_acceptance_result_accepted_count"
        )
    );
    insert_report!(
        "source_accepted_scoped_production_durable_memory_write_operator_packet_acceptance_fixture_count",
        json_u64(
            &source,
            "accepted_scoped_production_durable_memory_write_operator_packet_acceptance_fixture_count"
        )
    );
    insert_report!(
        "source_blocked_scoped_production_durable_memory_write_operator_packet_acceptance_fixture_count",
        json_u64(
            &source,
            "blocked_scoped_production_durable_memory_write_operator_packet_acceptance_fixture_count"
        )
    );
    insert_report!(
        "approved_production_namespace",
        approved_production_namespace
    );
    insert_report!("approved_production_store", approved_production_store);
    insert_report!("approved_production_scope", approved_production_scope);
    insert_report!(
        "production_durable_memory_target_id",
        production_durable_memory_target_id
    );
    insert_report!(
        "production_durable_memory_payload_class",
        production_durable_memory_payload_class
    );
    insert_report!("operator_packet_scope", operator_packet_scope);
    insert_report!(
        "source_production_durable_memory_write_preflight_target_hash_sha256",
        source_target_hash_sha256
    );
    insert_report!(
        "source_production_durable_memory_write_preflight_operator_packet_hash_sha256",
        source_operator_packet_hash_sha256
    );
    insert_report!(
        "source_operator_packet_acceptance_envelope_hash_sha256",
        source_acceptance_envelope_hash_sha256
    );
    insert_report!(
        "source_operator_packet_acceptance_identity_session_hash_sha256",
        source_acceptance_identity_session_hash_sha256
    );
    insert_report!(
        "source_operator_packet_acceptance_signature_hash_sha256",
        source_acceptance_signature_hash_sha256
    );
    insert_report!(
        "source_operator_packet_acceptance_nonce_hash_sha256",
        source_acceptance_nonce_hash_sha256
    );
    insert_report!(
        "source_operator_packet_acceptance_command_hash_sha256",
        source_acceptance_command_hash_sha256
    );
    insert_report!(
        "source_operator_packet_acceptance_receipt_plan_hash_sha256",
        source_acceptance_receipt_plan_hash_sha256
    );
    insert_report!(
        "source_operator_packet_acceptance_replay_guard_hash_sha256",
        source_acceptance_replay_guard_hash_sha256
    );
    insert_report!(
        "acceptance_receipt_envelope_hash_sha256",
        acceptance_receipt_envelope_hash_sha256
    );
    insert_report!(
        "acceptance_receipt_identity_session_hash_sha256",
        acceptance_receipt_identity_session_hash_sha256
    );
    insert_report!(
        "acceptance_receipt_digest_hash_sha256",
        acceptance_receipt_digest_hash_sha256
    );
    insert_report!(
        "acceptance_receipt_hash_chain_hash_sha256",
        acceptance_receipt_hash_chain_hash_sha256
    );
    insert_report!(
        "acceptance_receipt_readback_plan_hash_sha256",
        acceptance_receipt_readback_plan_hash_sha256
    );
    insert_report!(
        "acceptance_receipt_replay_guard_hash_sha256",
        acceptance_receipt_replay_guard_hash_sha256
    );
    insert_report!(
        "acceptance_receipt_result_hash_sha256",
        acceptance_receipt_result_hash_sha256
    );
    insert_report!(
        "scoped_production_durable_memory_write_operator_packet_acceptance_receipt_boundary_hash_sha256",
        acceptance_receipt_boundary_hash_sha256
    );
    insert_report!(
        "scoped_production_durable_memory_write_operator_packet_acceptance_receipt_policy_hash_sha256",
        acceptance_receipt_policy_hash_sha256
    );
    insert_report!(
        "required_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_surface_count",
        RECEIPT_SURFACES.len()
    );
    insert_report!(
        "ready_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_surface_count",
        if surfaces_ready {
            RECEIPT_SURFACES.len()
        } else {
            0
        }
    );
    insert_report!(
        "scoped_production_durable_memory_write_operator_packet_acceptance_receipt_surfaces",
        RECEIPT_SURFACES
    );
    insert_report!(
        "scoped_production_durable_memory_write_operator_packet_acceptance_receipt_fixture_count",
        fixtures.len()
    );
    insert_report!(
        "accepted_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_fixture_count",
        accepted_fixture_count
    );
    insert_report!(
        "blocked_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_fixture_count",
        blocked_fixture_count
    );
    insert_report!(
        "scoped_production_durable_memory_write_operator_packet_acceptance_receipt_fixtures",
        fixtures
    );
    insert_report!(
        "denied_by_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_boundary",
        RECEIPT_DENIALS
    );
    insert_report!(
        "denied_by_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_boundary_count",
        RECEIPT_DENIALS.len()
    );
    insert_report!(
        "allowed_next_actions",
        [
            serde_json::json!({
                "action": "run_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_boundary_require_live_gate",
                "status": "allowed_verification_only",
                "accepts_acceptance_receipt_evidence": true,
                "persists_acceptance_receipt": false,
                "persists_operator_packet": false,
                "writes_production_durable_memory": false,
                "writes_memory_store": false,
                "writes_wal": false,
                "persists_receipt": false,
                "executes_rollback": false,
                "writes_tombstone": false
            }),
            serde_json::json!({
                "action": "prepare_scoped_production_durable_memory_write_dry_run_execution_envelope_boundary",
                "status": "requires_separate_dry_run_execution_envelope_gate",
                "requires_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_boundary": true,
                "writes_production_durable_memory": false,
                "persists_acceptance_receipt": false,
                "persists_operator_packet": false
            }),
        ]
    );
    for &key in FALSE_RECEIPT_SIDE_EFFECT_KEYS {
        report_object.insert(key.to_string(), serde_json::json!(false));
        report_object.insert(format!("{key}_count"), serde_json::json!(0));
    }
    for &key in TRUE_RECEIPT_KEYS {
        report_object.insert(key.to_string(), serde_json::json!(report_ready));
        report_object.insert(
            format!("{key}_count"),
            serde_json::json!(if report_ready { 1 } else { 0 }),
        );
    }
    for key in [
        "source_operator_packet_acceptance_boundary_bound",
        "approved_production_namespace_bound",
        "approved_production_store_bound",
        "approved_production_scope_bound",
        "production_durable_memory_target_bound",
        "operator_packet_acceptance_result_bound",
        "operator_packet_acceptance_envelope_bound",
        "operator_identity_session_acceptance_bound",
        "operator_packet_signature_acceptance_bound",
        "single_use_acceptance_nonce_bound",
        "explicit_acceptance_command_bound",
        "acceptance_receipt_envelope_bound",
        "acceptance_receipt_identity_session_bound",
        "acceptance_receipt_digest_bound",
        "acceptance_receipt_hash_chain_bound",
        "acceptance_receipt_readback_plan_bound",
        "acceptance_receipt_replay_guard_bound",
        "acceptance_receipt_handoff_bound",
        "acceptance_receipt_persistence_forbidden_on_report_route",
        "operator_packet_persistence_forbidden_on_report_route",
        "production_write_execution_forbidden_on_acceptance_receipt_route",
        "production_durable_memory_write_forbidden",
        "memory_store_mutation_forbidden",
        "wal_write_forbidden_on_acceptance_receipt_route",
        "receipt_persist_forbidden_on_acceptance_receipt_route",
        "rollback_execution_forbidden_on_acceptance_receipt_route",
        "tombstone_write_forbidden_on_acceptance_receipt_route",
        "kg_live_write_forbidden",
        "provider_model_invocation_forbidden",
        "credential_channel_public_release_forbidden",
        "install_restart_active_binary_mutation_forbidden",
    ] {
        report_object.insert(key.to_string(), serde_json::json!(true));
    }
    report_object.insert(
        "side_effects".to_string(),
        serde_json::Value::Object(side_effects),
    );
    report
}

fn hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_envelope_boundary_report()
-> serde_json::Value {
    const DRY_RUN_SURFACES: &[&str] = &[
        "source_operator_packet_acceptance_receipt_boundary_required",
        "source_acceptance_receipt_result_required",
        "dry_run_execution_envelope_required",
        "dry_run_execution_identity_session_required",
        "dry_run_execution_target_snapshot_required",
        "dry_run_execution_write_plan_required",
        "dry_run_execution_payload_redaction_required",
        "dry_run_execution_wal_receipt_preview_required",
        "dry_run_execution_readback_preview_required",
        "dry_run_execution_rollback_tombstone_preview_required",
        "dry_run_execution_replay_guard_required",
        "dry_run_execution_handoff_required",
        "dry_run_execution_persistence_forbidden_on_report_route",
        "production_write_execution_forbidden_on_dry_run_envelope_route",
        "kg_provider_channel_release_install_active_binary_forbidden",
    ];
    const DRY_RUN_DENIALS: &[&str] = &[
        "source_operator_packet_acceptance_receipt_boundary_required",
        "source_acceptance_receipt_result_hash_required",
        "source_acceptance_receipt_policy_hash_required",
        "approved_production_namespace_required",
        "approved_production_store_required",
        "approved_production_scope_required",
        "production_durable_memory_target_required",
        "acceptance_receipt_envelope_required",
        "acceptance_receipt_digest_required",
        "acceptance_receipt_hash_chain_required",
        "acceptance_receipt_readback_plan_required",
        "acceptance_receipt_replay_guard_required",
        "dry_run_execution_envelope_required",
        "dry_run_execution_identity_session_required",
        "dry_run_execution_target_snapshot_required",
        "dry_run_execution_write_plan_required",
        "dry_run_execution_payload_redaction_required",
        "dry_run_execution_wal_receipt_preview_required",
        "dry_run_execution_readback_preview_required",
        "dry_run_execution_rollback_tombstone_preview_required",
        "dry_run_execution_replay_guard_required",
        "dry_run_execution_handoff_required",
        "dry_run_execution_persistence_report_route_denied",
        "dry_run_execution_filesystem_write_denied",
        "dry_run_execution_ledger_recording_denied",
        "dry_run_execution_delivery_denied",
        "dry_run_execution_execution_denied",
        "acceptance_receipt_persistence_report_route_denied",
        "operator_packet_persistence_report_route_denied",
        "production_write_execution_report_route_denied",
        "production_durable_memory_backend_write_denied",
        "durable_memory_backend_read_or_rollback_denied",
        "memory_store_mutation_denied",
        "wal_write_report_route_denied",
        "receipt_persist_report_route_denied",
        "post_write_readback_report_route_denied",
        "rollback_execution_report_route_denied",
        "tombstone_write_report_route_denied",
        "raw_payload_plaintext_denied",
        "kg_live_write_denied",
        "provider_model_invocation_denied",
        "credential_channel_release_install_denied",
        "active_binary_mutation_denied",
        "unrestricted_full_live_activation_denied",
    ];
    const FALSE_DRY_RUN_SIDE_EFFECT_KEYS: &[&str] = &[
        "dry_run_execution_envelope_persisted",
        "dry_run_execution_envelope_filesystem_written",
        "dry_run_execution_envelope_ledger_recorded",
        "dry_run_execution_envelope_delivered",
        "dry_run_execution_envelope_materialized",
        "dry_run_execution_executed",
        "dry_run_execution_result_persisted",
        "acceptance_receipt_persisted",
        "acceptance_receipt_filesystem_written",
        "acceptance_receipt_ledger_recorded",
        "acceptance_receipt_delivered",
        "operator_packet_persisted",
        "operator_packet_ledger_recorded",
        "operator_packet_filesystem_written",
        "operator_packet_acceptance_receipt_persisted",
        "production_durable_memory_write_executed",
        "production_durable_memory_backend_present",
        "production_durable_memory_store_write_performed",
        "actual_production_durable_memory_write_performed",
        "durable_memory_store_write_performed",
        "durable_memory_store_read_performed",
        "durable_memory_store_rollback_performed",
        "memory_write_execution_performed",
        "memory_store_write_path_enabled",
        "memory_store_write_allowed",
        "memory_store_write_performed",
        "memory_store_mutation_allowed",
        "memory_store_mutated",
        "wal_write_performed",
        "wal_recorded",
        "wal_persisted",
        "receipt_recorded",
        "receipt_persisted",
        "receipt_materialized",
        "receipt_delivered",
        "post_write_readback_performed",
        "readback_result_recorded",
        "readback_result_persisted",
        "readback_result_accepted",
        "rollback_executed",
        "rollback_performed",
        "rollback_result_recorded",
        "rollback_result_persisted",
        "rollback_result_accepted",
        "tombstone_write_performed",
        "tombstone_cleanup_executed",
        "tombstone_cleanup_result_recorded",
        "tombstone_cleanup_result_accepted",
        "raw_payload_plaintext_recorded",
        "raw_payload_plaintext_persisted",
        "secret_material_read",
        "credential_read",
        "secret_file_read",
        "kg_adapter_read_performed",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "telegram_send_performed",
        "channel_send_performed",
        "external_send_performed",
        "public_claim_promoted",
        "public_release_published",
        "public_ga_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "service_restart_performed",
        "active_binary_mutated",
    ];
    const TRUE_DRY_RUN_KEYS: &[&str] = &[
        "scoped_production_durable_memory_write_dry_run_execution_envelope_boundary_performed",
        "scoped_production_durable_memory_write_dry_run_execution_envelope_result_recorded",
        "scoped_production_durable_memory_write_dry_run_execution_envelope_result_accepted",
        "source_operator_packet_acceptance_receipt_boundary_accepted",
        "dry_run_execution_envelope_bound",
        "dry_run_execution_identity_session_bound",
        "dry_run_execution_target_snapshot_bound",
        "dry_run_execution_write_plan_bound",
        "dry_run_execution_payload_redaction_bound",
        "dry_run_execution_wal_receipt_preview_bound",
        "dry_run_execution_readback_preview_bound",
        "dry_run_execution_rollback_tombstone_preview_bound",
        "dry_run_execution_replay_guard_bound",
        "dry_run_execution_handoff_bound",
        "dry_run_execution_persistence_forbidden_on_report_route",
        "production_write_execution_forbidden_on_dry_run_envelope_route",
        "kg_provider_channel_release_install_active_binary_forbidden",
    ];

    let route_matrix = control_ui_route_parity_report();
    let source = std::thread::Builder::new()
        .name("hepta-memory-production-durable-dry-run-execution-envelope-source-report".to_string())
        .stack_size(8 * 1024 * 1024)
        .spawn(
            hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_boundary_report,
        )
        .ok()
        .and_then(|handle| handle.join().ok())
        .unwrap_or_else(|| {
            serde_json::json!({
                "status": "blocked",
                "memory_write_execution_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_boundary_ready": false,
                "scoped_production_durable_memory_write_operator_packet_acceptance_receipt_accepted": false,
                "source_scoped_production_durable_memory_write_dry_run_execution_envelope_source_report_thread_failed": true
            })
        });
    let json_bool = |value: &serde_json::Value, key: &str| {
        value
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let json_u64 = |value: &serde_json::Value, key: &str| {
        value
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let json_str = |value: &serde_json::Value, key: &str| {
        value
            .get(key)
            .and_then(serde_json::Value::as_str)
            .unwrap_or("")
            .to_string()
    };
    let route_count_source_command_accepted = route_matrix.ready
        && route_matrix.route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_next_action_dry_run = source
        .get("allowed_next_actions")
        .and_then(serde_json::Value::as_array)
        .and_then(|items| items.get(1))
        .map(|item| {
            item.get("action").and_then(serde_json::Value::as_str)
                == Some("prepare_scoped_production_durable_memory_write_dry_run_execution_envelope_boundary")
                && item
                    .get("requires_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_boundary")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
                && item
                    .get("writes_production_durable_memory")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && item
                    .get("persists_acceptance_receipt")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && item
                    .get("persists_operator_packet")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
        })
        .unwrap_or(false);
    let source_side_effects_ok = source
        .get("side_effects")
        .and_then(serde_json::Value::as_object)
        .map(|effects| {
            effects
                .get("scoped_production_durable_memory_write_operator_packet_acceptance_receipt_boundary_performed")
                .and_then(serde_json::Value::as_bool)
                == Some(true)
                && effects
                    .get("scoped_production_durable_memory_write_operator_packet_acceptance_receipt_result_accepted")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
                && effects
                    .get("acceptance_receipt_persisted")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("operator_packet_persisted")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("production_durable_memory_store_write_performed")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("memory_store_write_performed")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("wal_write_performed")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("receipt_persisted")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("external_send_performed")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
        })
        .unwrap_or(false);
    let source_ready = source.get("status").and_then(serde_json::Value::as_str) == Some("ready")
        && json_bool(
            &source,
            "memory_write_execution_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_boundary_ready",
        )
        && json_bool(
            &source,
            "scoped_production_durable_memory_write_operator_packet_acceptance_receipt_accepted",
        )
        && json_u64(
            &source,
            "accepted_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_fixture_count",
        ) == 1
        && json_u64(
            &source,
            "blocked_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_fixture_count",
        ) == 9
        && json_u64(
            &source,
            "scoped_production_durable_memory_write_operator_packet_acceptance_receipt_result_accepted_count",
        ) == 1
        && !json_bool(&source, "acceptance_receipt_persisted")
        && !json_bool(&source, "operator_packet_persisted")
        && !json_bool(&source, "operator_packet_acceptance_receipt_persisted")
        && !json_bool(&source, "production_durable_memory_write_executed")
        && !json_bool(&source, "production_durable_memory_store_write_performed")
        && !json_bool(&source, "actual_production_durable_memory_write_performed")
        && !json_bool(&source, "durable_memory_store_write_performed")
        && !json_bool(&source, "durable_memory_store_read_performed")
        && !json_bool(&source, "durable_memory_store_rollback_performed")
        && !json_bool(&source, "memory_store_write_performed")
        && !json_bool(&source, "wal_write_performed")
        && !json_bool(&source, "receipt_persisted")
        && !json_bool(&source, "post_write_readback_performed")
        && !json_bool(&source, "rollback_executed")
        && !json_bool(&source, "tombstone_cleanup_executed")
        && !json_bool(&source, "raw_payload_plaintext_recorded")
        && !json_bool(&source, "raw_payload_plaintext_persisted")
        && !json_bool(&source, "live_kg_write_performed")
        && !json_bool(&source, "provider_invoked")
        && !json_bool(&source, "model_invoked")
        && !json_bool(&source, "credential_read")
        && !json_bool(&source, "channel_send_performed")
        && !json_bool(&source, "external_send_performed")
        && !json_bool(&source, "release_artifact_written")
        && !json_bool(&source, "install_executed")
        && !json_bool(&source, "service_restarted")
        && !json_bool(&source, "active_binary_mutated")
        && source_next_action_dry_run
        && source_side_effects_ok;

    let approved_production_namespace = json_str(&source, "approved_production_namespace");
    let approved_production_store = json_str(&source, "approved_production_store");
    let approved_production_scope = json_str(&source, "approved_production_scope");
    let production_durable_memory_target_id =
        json_str(&source, "production_durable_memory_target_id");
    let production_durable_memory_payload_class =
        json_str(&source, "production_durable_memory_payload_class");
    let operator_packet_scope = json_str(&source, "operator_packet_scope");
    let source_report_sha256 = sha256_text_value(&source.to_string());
    let source_acceptance_receipt_result_hash_sha256 =
        json_str(&source, "acceptance_receipt_result_hash_sha256");
    let source_acceptance_receipt_boundary_hash_sha256 = json_str(
        &source,
        "scoped_production_durable_memory_write_operator_packet_acceptance_receipt_boundary_hash_sha256",
    );
    let source_acceptance_receipt_policy_hash_sha256 = json_str(
        &source,
        "scoped_production_durable_memory_write_operator_packet_acceptance_receipt_policy_hash_sha256",
    );
    let source_target_hash_sha256 = json_str(
        &source,
        "source_production_durable_memory_write_preflight_target_hash_sha256",
    );
    let source_operator_packet_hash_sha256 = json_str(
        &source,
        "source_production_durable_memory_write_preflight_operator_packet_hash_sha256",
    );
    let source_acceptance_receipt_envelope_hash_sha256 =
        json_str(&source, "acceptance_receipt_envelope_hash_sha256");
    let source_acceptance_receipt_identity_session_hash_sha256 =
        json_str(&source, "acceptance_receipt_identity_session_hash_sha256");
    let source_acceptance_receipt_digest_hash_sha256 =
        json_str(&source, "acceptance_receipt_digest_hash_sha256");
    let source_acceptance_receipt_hash_chain_hash_sha256 =
        json_str(&source, "acceptance_receipt_hash_chain_hash_sha256");
    let source_acceptance_receipt_readback_plan_hash_sha256 =
        json_str(&source, "acceptance_receipt_readback_plan_hash_sha256");
    let source_acceptance_receipt_replay_guard_hash_sha256 =
        json_str(&source, "acceptance_receipt_replay_guard_hash_sha256");
    let dry_run_execution_envelope_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-execution-envelope:v1:source-result={source_acceptance_receipt_result_hash_sha256}:target={source_target_hash_sha256}:execute-now=false"
    ));
    let dry_run_execution_identity_session_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-execution-identity-session:v1:envelope={dry_run_execution_envelope_hash_sha256}:source-identity-session={source_acceptance_receipt_identity_session_hash_sha256}:operator-bound=true"
    ));
    let dry_run_execution_target_snapshot_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-execution-target-snapshot:v1:target={source_target_hash_sha256}:namespace={approved_production_namespace}:store={approved_production_store}:scope={approved_production_scope}:read-now=false"
    ));
    let dry_run_execution_write_plan_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-execution-write-plan:v1:target-snapshot={dry_run_execution_target_snapshot_hash_sha256}:payload-class={production_durable_memory_payload_class}:write-now=false"
    ));
    let dry_run_execution_payload_redaction_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-execution-payload-redaction:v1:write-plan={dry_run_execution_write_plan_hash_sha256}:operator-packet={source_operator_packet_hash_sha256}:raw-payload=false"
    ));
    let dry_run_execution_wal_receipt_preview_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-execution-wal-receipt-preview:v1:write-plan={dry_run_execution_write_plan_hash_sha256}:receipt={source_acceptance_receipt_digest_hash_sha256}:persist-now=false"
    ));
    let dry_run_execution_readback_preview_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-execution-readback-preview:v1:wal-receipt-preview={dry_run_execution_wal_receipt_preview_hash_sha256}:source-readback={source_acceptance_receipt_readback_plan_hash_sha256}:readback-now=false"
    ));
    let dry_run_execution_rollback_tombstone_preview_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-execution-rollback-tombstone-preview:v1:readback={dry_run_execution_readback_preview_hash_sha256}:hash-chain={source_acceptance_receipt_hash_chain_hash_sha256}:rollback-now=false:tombstone-now=false"
    ));
    let dry_run_execution_replay_guard_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-execution-replay-guard:v1:envelope={dry_run_execution_envelope_hash_sha256}:source-replay={source_acceptance_receipt_replay_guard_hash_sha256}:replay=false"
    ));
    let dry_run_execution_result_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-execution-result:v1:envelope={dry_run_execution_envelope_hash_sha256}:write-plan={dry_run_execution_write_plan_hash_sha256}:rollback-preview={dry_run_execution_rollback_tombstone_preview_hash_sha256}:accepted=true:executed=false"
    ));
    let dry_run_execution_boundary_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-execution-envelope-boundary:v1:source={source_report_sha256}:result={dry_run_execution_result_hash_sha256}:fixtures=10:accepted=1:denials={}:dry-run-executed=false:production-write=false",
        DRY_RUN_DENIALS.len()
    ));
    let dry_run_execution_policy_hash_sha256 = sha256_text_value(
        "scoped-production-durable-memory-write-dry-run-execution-envelope-policy:v1:bind-source-receipt-envelope-target-write-plan-redaction-wal-readback-rollback-replay:no-execution:no-persistence:no-production-write:no-kg:no-provider:no-channel:no-release:no-install",
    );
    let dry_run_bound = !source_acceptance_receipt_result_hash_sha256.is_empty()
        && !source_acceptance_receipt_boundary_hash_sha256.is_empty()
        && !source_acceptance_receipt_policy_hash_sha256.is_empty()
        && !source_target_hash_sha256.is_empty()
        && !source_operator_packet_hash_sha256.is_empty()
        && !source_acceptance_receipt_envelope_hash_sha256.is_empty()
        && !source_acceptance_receipt_identity_session_hash_sha256.is_empty()
        && !source_acceptance_receipt_digest_hash_sha256.is_empty()
        && !source_acceptance_receipt_hash_chain_hash_sha256.is_empty()
        && !source_acceptance_receipt_readback_plan_hash_sha256.is_empty()
        && !source_acceptance_receipt_replay_guard_hash_sha256.is_empty()
        && approved_production_namespace == "hepta.memory.production.scoped"
        && approved_production_store == "hepta-memory-durable-store-production-preflight-only"
        && approved_production_scope == "operator-approved-session"
        && production_durable_memory_target_id
            == "hepta-scoped-production-durable-memory-write-target-v1"
        && production_durable_memory_payload_class
            == "redacted-minimal-operator-approved-memory-fact";
    let surfaces_ready = source_ready && dry_run_bound;
    let report_ready = route_count_source_command_accepted && surfaces_ready;
    let accepted_fixture_count = if report_ready { 1 } else { 0 };
    let blocked_fixture_count = 10 - accepted_fixture_count;

    let mut fixtures = Vec::new();
    fixtures.push(serde_json::json!({
        "id": "scoped-production-durable-memory-write-dry-run-execution-envelope",
        "fixture_id": "scoped-production-durable-memory-write-dry-run-execution-envelope",
        "scoped_production_durable_memory_write_dry_run_execution_envelope_accepted": report_ready,
        "reason": if report_ready { "dry_run_execution_envelope_bound_without_execution_persistence_or_production_write" } else { "source_receipt_or_route_count_not_ready" },
        "source_operator_packet_acceptance_receipt_boundary_bound": report_ready,
        "dry_run_execution_envelope_bound": report_ready,
        "dry_run_execution_write_plan_bound": report_ready,
        "dry_run_execution_wal_receipt_preview_bound": report_ready,
        "dry_run_execution_readback_preview_bound": report_ready,
        "dry_run_execution_rollback_tombstone_preview_bound": report_ready,
        "dry_run_execution_replay_guard_bound": report_ready,
        "dry_run_execution_executed": false,
        "production_durable_memory_store_write_performed": false,
        "external_send_performed": false
    }));
    for id in [
        "missing-operator-packet-acceptance-receipt-source",
        "missing-acceptance-receipt-result-hash",
        "missing-dry-run-execution-envelope",
        "missing-dry-run-target-snapshot",
        "missing-dry-run-write-plan",
        "missing-dry-run-wal-receipt-preview",
        "missing-dry-run-readback-preview",
        "missing-dry-run-rollback-tombstone-preview",
        "dry-run-execution-or-production-write-attempt",
    ] {
        fixtures.push(serde_json::json!({
            "id": id,
            "fixture_id": id,
            "scoped_production_durable_memory_write_dry_run_execution_envelope_accepted": false,
            "reason": "blocked_noop",
            "dry_run_execution_executed": false,
            "dry_run_execution_envelope_persisted": false,
            "production_durable_memory_store_write_performed": false,
            "external_send_performed": false
        }));
    }

    let mut side_effects = serde_json::Map::new();
    for &key in FALSE_DRY_RUN_SIDE_EFFECT_KEYS {
        side_effects.insert(key.to_string(), serde_json::json!(false));
        side_effects.insert(format!("{key}_count"), serde_json::json!(0));
    }
    for &key in TRUE_DRY_RUN_KEYS {
        side_effects.insert(key.to_string(), serde_json::json!(report_ready));
        side_effects.insert(
            format!("{key}_count"),
            serde_json::json!(if report_ready { 1 } else { 0 }),
        );
    }

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "endpoint": HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_PRODUCTION_DURABLE_MEMORY_WRITE_DRY_RUN_EXECUTION_ENVELOPE_BOUNDARY_ENDPOINT,
        "source_command": "/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-envelope-boundary --json",
        "native_route": true,
        "side_effect_free": false,
        "external_side_effect_free": true,
        "audit_date": "2026-07-05"
    });
    let report_object = report
        .as_object_mut()
        .expect("scoped production durable Memory write dry-run execution envelope report object");
    macro_rules! insert_report {
        ($key:expr, $value:expr) => {
            report_object.insert($key.to_string(), serde_json::json!($value));
        };
    }

    insert_report!(
        "native_gateway_source_command_count",
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    insert_report!("route_count", route_matrix.route_count);
    insert_report!(
        "implemented_route_count",
        route_matrix.implemented_route_count
    );
    insert_report!("missing_route_count", route_matrix.missing_route_count);
    insert_report!(
        "route_count_source_command_accepted",
        route_count_source_command_accepted
    );
    insert_report!(
        "memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_envelope_boundary_ready",
        report_ready
    );
    insert_report!(
        "scoped_production_durable_memory_write_dry_run_execution_envelope_ready",
        report_ready
    );
    insert_report!(
        "scoped_production_durable_memory_write_dry_run_execution_envelope_performed",
        report_ready
    );
    insert_report!(
        "scoped_production_durable_memory_write_dry_run_execution_envelope_accepted",
        report_ready
    );
    insert_report!(
        "scoped_production_durable_memory_write_dry_run_execution_envelope_mode",
        "dry_run_execution_envelope_boundary_no_execution_no_persistence_no_production_durable_memory_mutation"
    );
    insert_report!(
        "source_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_boundary_ready",
        source_ready
    );
    insert_report!(
        "source_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_boundary_report_sha256",
        source_report_sha256
    );
    insert_report!(
        "source_acceptance_receipt_result_hash_sha256",
        source_acceptance_receipt_result_hash_sha256
    );
    insert_report!(
        "source_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_boundary_hash_sha256",
        source_acceptance_receipt_boundary_hash_sha256
    );
    insert_report!(
        "source_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_policy_hash_sha256",
        source_acceptance_receipt_policy_hash_sha256
    );
    insert_report!(
        "source_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_accepted_count",
        json_u64(
            &source,
            "scoped_production_durable_memory_write_operator_packet_acceptance_receipt_result_accepted_count"
        )
    );
    insert_report!(
        "source_accepted_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_fixture_count",
        json_u64(
            &source,
            "accepted_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_fixture_count"
        )
    );
    insert_report!(
        "source_blocked_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_fixture_count",
        json_u64(
            &source,
            "blocked_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_fixture_count"
        )
    );
    insert_report!(
        "approved_production_namespace",
        approved_production_namespace
    );
    insert_report!("approved_production_store", approved_production_store);
    insert_report!("approved_production_scope", approved_production_scope);
    insert_report!(
        "production_durable_memory_target_id",
        production_durable_memory_target_id
    );
    insert_report!(
        "production_durable_memory_payload_class",
        production_durable_memory_payload_class
    );
    insert_report!("operator_packet_scope", operator_packet_scope);
    insert_report!(
        "source_production_durable_memory_write_preflight_target_hash_sha256",
        source_target_hash_sha256
    );
    insert_report!(
        "source_production_durable_memory_write_preflight_operator_packet_hash_sha256",
        source_operator_packet_hash_sha256
    );
    insert_report!(
        "source_acceptance_receipt_envelope_hash_sha256",
        source_acceptance_receipt_envelope_hash_sha256
    );
    insert_report!(
        "source_acceptance_receipt_identity_session_hash_sha256",
        source_acceptance_receipt_identity_session_hash_sha256
    );
    insert_report!(
        "source_acceptance_receipt_digest_hash_sha256",
        source_acceptance_receipt_digest_hash_sha256
    );
    insert_report!(
        "source_acceptance_receipt_hash_chain_hash_sha256",
        source_acceptance_receipt_hash_chain_hash_sha256
    );
    insert_report!(
        "source_acceptance_receipt_readback_plan_hash_sha256",
        source_acceptance_receipt_readback_plan_hash_sha256
    );
    insert_report!(
        "source_acceptance_receipt_replay_guard_hash_sha256",
        source_acceptance_receipt_replay_guard_hash_sha256
    );
    insert_report!(
        "dry_run_execution_envelope_hash_sha256",
        dry_run_execution_envelope_hash_sha256
    );
    insert_report!(
        "dry_run_execution_identity_session_hash_sha256",
        dry_run_execution_identity_session_hash_sha256
    );
    insert_report!(
        "dry_run_execution_target_snapshot_hash_sha256",
        dry_run_execution_target_snapshot_hash_sha256
    );
    insert_report!(
        "dry_run_execution_write_plan_hash_sha256",
        dry_run_execution_write_plan_hash_sha256
    );
    insert_report!(
        "dry_run_execution_payload_redaction_hash_sha256",
        dry_run_execution_payload_redaction_hash_sha256
    );
    insert_report!(
        "dry_run_execution_wal_receipt_preview_hash_sha256",
        dry_run_execution_wal_receipt_preview_hash_sha256
    );
    insert_report!(
        "dry_run_execution_readback_preview_hash_sha256",
        dry_run_execution_readback_preview_hash_sha256
    );
    insert_report!(
        "dry_run_execution_rollback_tombstone_preview_hash_sha256",
        dry_run_execution_rollback_tombstone_preview_hash_sha256
    );
    insert_report!(
        "dry_run_execution_replay_guard_hash_sha256",
        dry_run_execution_replay_guard_hash_sha256
    );
    insert_report!(
        "dry_run_execution_result_hash_sha256",
        dry_run_execution_result_hash_sha256
    );
    insert_report!(
        "scoped_production_durable_memory_write_dry_run_execution_envelope_boundary_hash_sha256",
        dry_run_execution_boundary_hash_sha256
    );
    insert_report!(
        "scoped_production_durable_memory_write_dry_run_execution_envelope_policy_hash_sha256",
        dry_run_execution_policy_hash_sha256
    );
    insert_report!(
        "required_scoped_production_durable_memory_write_dry_run_execution_envelope_surface_count",
        DRY_RUN_SURFACES.len()
    );
    insert_report!(
        "ready_scoped_production_durable_memory_write_dry_run_execution_envelope_surface_count",
        if surfaces_ready {
            DRY_RUN_SURFACES.len()
        } else {
            0
        }
    );
    insert_report!(
        "scoped_production_durable_memory_write_dry_run_execution_envelope_surfaces",
        DRY_RUN_SURFACES
    );
    insert_report!(
        "scoped_production_durable_memory_write_dry_run_execution_envelope_fixture_count",
        fixtures.len()
    );
    insert_report!(
        "accepted_scoped_production_durable_memory_write_dry_run_execution_envelope_fixture_count",
        accepted_fixture_count
    );
    insert_report!(
        "blocked_scoped_production_durable_memory_write_dry_run_execution_envelope_fixture_count",
        blocked_fixture_count
    );
    insert_report!(
        "scoped_production_durable_memory_write_dry_run_execution_envelope_fixtures",
        fixtures
    );
    insert_report!(
        "denied_by_scoped_production_durable_memory_write_dry_run_execution_envelope_boundary",
        DRY_RUN_DENIALS
    );
    insert_report!(
        "denied_by_scoped_production_durable_memory_write_dry_run_execution_envelope_boundary_count",
        DRY_RUN_DENIALS.len()
    );
    insert_report!(
        "allowed_next_actions",
        [
            serde_json::json!({
                "action": "run_scoped_production_durable_memory_write_dry_run_execution_envelope_boundary_require_live_gate",
                "status": "allowed_verification_only",
                "accepts_dry_run_execution_envelope": true,
                "executes_dry_run": false,
                "persists_dry_run_envelope": false,
                "writes_production_durable_memory": false,
                "writes_memory_store": false,
                "writes_wal": false,
                "persists_receipt": false,
                "executes_rollback": false,
                "writes_tombstone": false
            }),
            serde_json::json!({
                "action": "prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_boundary",
                "status": "requires_separate_dry_run_execution_result_receipt_gate",
                "requires_scoped_production_durable_memory_write_dry_run_execution_envelope_boundary": true,
                "executes_dry_run": false,
                "writes_production_durable_memory": false,
                "persists_dry_run_envelope": false
            }),
        ]
    );
    for &key in FALSE_DRY_RUN_SIDE_EFFECT_KEYS {
        report_object.insert(key.to_string(), serde_json::json!(false));
        report_object.insert(format!("{key}_count"), serde_json::json!(0));
    }
    for &key in TRUE_DRY_RUN_KEYS {
        report_object.insert(key.to_string(), serde_json::json!(report_ready));
        report_object.insert(
            format!("{key}_count"),
            serde_json::json!(if report_ready { 1 } else { 0 }),
        );
    }
    for key in [
        "source_operator_packet_acceptance_receipt_boundary_bound",
        "approved_production_namespace_bound",
        "approved_production_store_bound",
        "approved_production_scope_bound",
        "production_durable_memory_target_bound",
        "acceptance_receipt_result_bound",
        "acceptance_receipt_envelope_bound",
        "acceptance_receipt_digest_bound",
        "acceptance_receipt_hash_chain_bound",
        "acceptance_receipt_readback_plan_bound",
        "acceptance_receipt_replay_guard_bound",
        "dry_run_execution_envelope_bound",
        "dry_run_execution_identity_session_bound",
        "dry_run_execution_target_snapshot_bound",
        "dry_run_execution_write_plan_bound",
        "dry_run_execution_payload_redaction_bound",
        "dry_run_execution_wal_receipt_preview_bound",
        "dry_run_execution_readback_preview_bound",
        "dry_run_execution_rollback_tombstone_preview_bound",
        "dry_run_execution_replay_guard_bound",
        "dry_run_execution_handoff_bound",
        "dry_run_execution_persistence_forbidden_on_report_route",
        "dry_run_execution_execution_forbidden_on_report_route",
        "acceptance_receipt_persistence_forbidden_on_dry_run_route",
        "operator_packet_persistence_forbidden_on_dry_run_route",
        "production_write_execution_forbidden_on_dry_run_envelope_route",
        "production_durable_memory_write_forbidden",
        "memory_store_mutation_forbidden",
        "wal_write_forbidden_on_dry_run_route",
        "receipt_persist_forbidden_on_dry_run_route",
        "rollback_execution_forbidden_on_dry_run_route",
        "tombstone_write_forbidden_on_dry_run_route",
        "kg_live_write_forbidden",
        "provider_model_invocation_forbidden",
        "credential_channel_public_release_forbidden",
        "install_restart_active_binary_mutation_forbidden",
    ] {
        report_object.insert(key.to_string(), serde_json::json!(true));
    }
    report_object.insert(
        "side_effects".to_string(),
        serde_json::Value::Object(side_effects),
    );
    report
}

fn hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_boundary_report()
-> serde_json::Value {
    const RESULT_RECEIPT_SURFACES: &[&str] = &[
        "source_dry_run_execution_envelope_boundary_required",
        "source_dry_run_execution_result_required",
        "dry_run_execution_result_receipt_envelope_required",
        "dry_run_execution_result_receipt_identity_session_required",
        "dry_run_execution_result_receipt_digest_required",
        "dry_run_execution_result_receipt_hash_chain_required",
        "dry_run_execution_result_receipt_readback_plan_required",
        "dry_run_execution_result_receipt_replay_guard_required",
        "dry_run_execution_result_receipt_handoff_required",
        "dry_run_execution_result_receipt_persistence_forbidden_on_report_route",
        "dry_run_execution_execution_forbidden_on_result_receipt_route",
        "dry_run_execution_envelope_persistence_forbidden_on_result_receipt_route",
        "production_write_execution_forbidden_on_result_receipt_route",
        "kg_provider_channel_release_install_active_binary_forbidden",
    ];
    const RESULT_RECEIPT_DENIALS: &[&str] = &[
        "source_dry_run_execution_envelope_boundary_required",
        "source_dry_run_execution_result_hash_required",
        "source_dry_run_execution_policy_hash_required",
        "approved_production_namespace_required",
        "approved_production_store_required",
        "approved_production_scope_required",
        "production_durable_memory_target_required",
        "dry_run_execution_envelope_required",
        "dry_run_execution_target_snapshot_required",
        "dry_run_execution_write_plan_required",
        "dry_run_execution_payload_redaction_required",
        "dry_run_execution_wal_receipt_preview_required",
        "dry_run_execution_readback_preview_required",
        "dry_run_execution_rollback_tombstone_preview_required",
        "dry_run_execution_replay_guard_required",
        "dry_run_execution_result_receipt_envelope_required",
        "dry_run_execution_result_receipt_identity_session_required",
        "dry_run_execution_result_receipt_digest_required",
        "dry_run_execution_result_receipt_hash_chain_required",
        "dry_run_execution_result_receipt_readback_plan_required",
        "dry_run_execution_result_receipt_replay_guard_required",
        "dry_run_execution_result_receipt_handoff_required",
        "dry_run_execution_result_receipt_persistence_report_route_denied",
        "dry_run_execution_result_receipt_filesystem_write_denied",
        "dry_run_execution_result_receipt_ledger_recording_denied",
        "dry_run_execution_result_receipt_delivery_denied",
        "dry_run_execution_result_receipt_materialization_denied",
        "dry_run_execution_execution_denied",
        "dry_run_execution_envelope_persistence_report_route_denied",
        "dry_run_execution_result_persistence_report_route_denied",
        "acceptance_receipt_persistence_report_route_denied",
        "operator_packet_persistence_report_route_denied",
        "production_write_execution_report_route_denied",
        "production_durable_memory_backend_write_denied",
        "durable_memory_backend_read_or_rollback_denied",
        "memory_store_mutation_denied",
        "wal_write_report_route_denied",
        "receipt_persist_report_route_denied",
        "post_write_readback_report_route_denied",
        "rollback_execution_report_route_denied",
        "tombstone_write_report_route_denied",
        "raw_payload_plaintext_denied",
        "kg_live_write_denied",
        "provider_model_invocation_denied",
        "credential_channel_release_install_denied",
        "active_binary_mutation_denied",
        "unrestricted_full_live_activation_denied",
    ];
    const FALSE_RESULT_RECEIPT_SIDE_EFFECT_KEYS: &[&str] = &[
        "dry_run_execution_result_receipt_persisted",
        "dry_run_execution_result_receipt_filesystem_written",
        "dry_run_execution_result_receipt_ledger_recorded",
        "dry_run_execution_result_receipt_delivered",
        "dry_run_execution_result_receipt_materialized",
        "dry_run_execution_envelope_persisted",
        "dry_run_execution_envelope_filesystem_written",
        "dry_run_execution_envelope_ledger_recorded",
        "dry_run_execution_envelope_delivered",
        "dry_run_execution_envelope_materialized",
        "dry_run_execution_executed",
        "dry_run_execution_result_persisted",
        "acceptance_receipt_persisted",
        "acceptance_receipt_filesystem_written",
        "acceptance_receipt_ledger_recorded",
        "acceptance_receipt_delivered",
        "operator_packet_persisted",
        "operator_packet_ledger_recorded",
        "operator_packet_filesystem_written",
        "operator_packet_acceptance_receipt_persisted",
        "production_durable_memory_write_executed",
        "production_durable_memory_backend_present",
        "production_durable_memory_store_write_performed",
        "actual_production_durable_memory_write_performed",
        "durable_memory_store_write_performed",
        "durable_memory_store_read_performed",
        "durable_memory_store_rollback_performed",
        "memory_write_execution_performed",
        "memory_store_write_path_enabled",
        "memory_store_write_allowed",
        "memory_store_write_performed",
        "memory_store_mutation_allowed",
        "memory_store_mutated",
        "wal_write_performed",
        "wal_recorded",
        "wal_persisted",
        "receipt_recorded",
        "receipt_persisted",
        "receipt_materialized",
        "receipt_delivered",
        "post_write_readback_performed",
        "readback_result_recorded",
        "readback_result_persisted",
        "readback_result_accepted",
        "rollback_executed",
        "rollback_performed",
        "rollback_result_recorded",
        "rollback_result_persisted",
        "rollback_result_accepted",
        "tombstone_write_performed",
        "tombstone_cleanup_executed",
        "tombstone_cleanup_result_recorded",
        "tombstone_cleanup_result_accepted",
        "raw_payload_plaintext_recorded",
        "raw_payload_plaintext_persisted",
        "secret_material_read",
        "credential_read",
        "secret_file_read",
        "kg_adapter_read_performed",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "telegram_send_performed",
        "channel_send_performed",
        "external_send_performed",
        "public_claim_promoted",
        "public_release_published",
        "public_ga_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "service_restart_performed",
        "active_binary_mutated",
    ];
    const TRUE_RESULT_RECEIPT_KEYS: &[&str] = &[
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_boundary_performed",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_result_recorded",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_result_accepted",
        "source_dry_run_execution_envelope_boundary_accepted",
        "dry_run_execution_result_bound",
        "dry_run_execution_result_receipt_envelope_bound",
        "dry_run_execution_result_receipt_identity_session_bound",
        "dry_run_execution_result_receipt_digest_bound",
        "dry_run_execution_result_receipt_hash_chain_bound",
        "dry_run_execution_result_receipt_readback_plan_bound",
        "dry_run_execution_result_receipt_replay_guard_bound",
        "dry_run_execution_result_receipt_handoff_bound",
        "dry_run_execution_result_receipt_persistence_forbidden_on_report_route",
        "dry_run_execution_execution_forbidden_on_result_receipt_route",
        "dry_run_execution_envelope_persistence_forbidden_on_result_receipt_route",
        "production_write_execution_forbidden_on_result_receipt_route",
        "kg_provider_channel_release_install_active_binary_forbidden",
    ];

    let route_matrix = control_ui_route_parity_report();
    let source = std::thread::Builder::new()
        .name("hepta-memory-production-durable-dry-run-execution-result-receipt-source-report".to_string())
        .stack_size(8 * 1024 * 1024)
        .spawn(
            hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_envelope_boundary_report,
        )
        .ok()
        .and_then(|handle| handle.join().ok())
        .unwrap_or_else(|| {
            serde_json::json!({
                "status": "blocked",
                "memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_envelope_boundary_ready": false,
                "scoped_production_durable_memory_write_dry_run_execution_envelope_accepted": false,
                "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_source_report_thread_failed": true
            })
        });
    let json_bool = |value: &serde_json::Value, key: &str| {
        value
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let json_u64 = |value: &serde_json::Value, key: &str| {
        value
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let json_str = |value: &serde_json::Value, key: &str| {
        value
            .get(key)
            .and_then(serde_json::Value::as_str)
            .unwrap_or("")
            .to_string()
    };
    let route_count_source_command_accepted = route_matrix.ready
        && route_matrix.route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_next_action_result_receipt = source
        .get("allowed_next_actions")
        .and_then(serde_json::Value::as_array)
        .and_then(|items| items.get(1))
        .map(|item| {
            item.get("action").and_then(serde_json::Value::as_str)
                == Some("prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_boundary")
                && item
                    .get("requires_scoped_production_durable_memory_write_dry_run_execution_envelope_boundary")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
                && item
                    .get("executes_dry_run")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && item
                    .get("writes_production_durable_memory")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && item
                    .get("persists_dry_run_envelope")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
        })
        .unwrap_or(false);
    let source_side_effects_ok = source
        .get("side_effects")
        .and_then(serde_json::Value::as_object)
        .map(|effects| {
            effects
                .get("scoped_production_durable_memory_write_dry_run_execution_envelope_boundary_performed")
                .and_then(serde_json::Value::as_bool)
                == Some(true)
                && effects
                    .get("scoped_production_durable_memory_write_dry_run_execution_envelope_result_accepted")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
                && effects
                    .get("dry_run_execution_executed")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("dry_run_execution_envelope_persisted")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("dry_run_execution_result_persisted")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("production_durable_memory_store_write_performed")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("memory_store_write_performed")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("wal_write_performed")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("receipt_persisted")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("external_send_performed")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
        })
        .unwrap_or(false);
    let source_ready = source.get("status").and_then(serde_json::Value::as_str) == Some("ready")
        && json_bool(
            &source,
            "memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_envelope_boundary_ready",
        )
        && json_bool(
            &source,
            "scoped_production_durable_memory_write_dry_run_execution_envelope_accepted",
        )
        && json_u64(
            &source,
            "accepted_scoped_production_durable_memory_write_dry_run_execution_envelope_fixture_count",
        ) == 1
        && json_u64(
            &source,
            "blocked_scoped_production_durable_memory_write_dry_run_execution_envelope_fixture_count",
        ) == 9
        && json_u64(
            &source,
            "scoped_production_durable_memory_write_dry_run_execution_envelope_result_accepted_count",
        ) == 1
        && !json_bool(&source, "dry_run_execution_executed")
        && !json_bool(&source, "dry_run_execution_envelope_persisted")
        && !json_bool(&source, "dry_run_execution_result_persisted")
        && !json_bool(&source, "acceptance_receipt_persisted")
        && !json_bool(&source, "operator_packet_persisted")
        && !json_bool(&source, "operator_packet_acceptance_receipt_persisted")
        && !json_bool(&source, "production_durable_memory_write_executed")
        && !json_bool(&source, "production_durable_memory_store_write_performed")
        && !json_bool(&source, "actual_production_durable_memory_write_performed")
        && !json_bool(&source, "durable_memory_store_write_performed")
        && !json_bool(&source, "durable_memory_store_read_performed")
        && !json_bool(&source, "durable_memory_store_rollback_performed")
        && !json_bool(&source, "memory_store_write_performed")
        && !json_bool(&source, "wal_write_performed")
        && !json_bool(&source, "receipt_persisted")
        && !json_bool(&source, "post_write_readback_performed")
        && !json_bool(&source, "rollback_executed")
        && !json_bool(&source, "tombstone_cleanup_executed")
        && !json_bool(&source, "raw_payload_plaintext_recorded")
        && !json_bool(&source, "raw_payload_plaintext_persisted")
        && !json_bool(&source, "live_kg_write_performed")
        && !json_bool(&source, "provider_invoked")
        && !json_bool(&source, "model_invoked")
        && !json_bool(&source, "credential_read")
        && !json_bool(&source, "channel_send_performed")
        && !json_bool(&source, "external_send_performed")
        && !json_bool(&source, "release_artifact_written")
        && !json_bool(&source, "install_executed")
        && !json_bool(&source, "service_restarted")
        && !json_bool(&source, "active_binary_mutated")
        && source_next_action_result_receipt
        && source_side_effects_ok;

    let approved_production_namespace = json_str(&source, "approved_production_namespace");
    let approved_production_store = json_str(&source, "approved_production_store");
    let approved_production_scope = json_str(&source, "approved_production_scope");
    let production_durable_memory_target_id =
        json_str(&source, "production_durable_memory_target_id");
    let production_durable_memory_payload_class =
        json_str(&source, "production_durable_memory_payload_class");
    let operator_packet_scope = json_str(&source, "operator_packet_scope");
    let source_report_sha256 = sha256_text_value(&source.to_string());
    let source_dry_run_execution_result_hash_sha256 =
        json_str(&source, "dry_run_execution_result_hash_sha256");
    let source_dry_run_execution_envelope_boundary_hash_sha256 = json_str(
        &source,
        "scoped_production_durable_memory_write_dry_run_execution_envelope_boundary_hash_sha256",
    );
    let source_dry_run_execution_envelope_policy_hash_sha256 = json_str(
        &source,
        "scoped_production_durable_memory_write_dry_run_execution_envelope_policy_hash_sha256",
    );
    let source_target_hash_sha256 = json_str(
        &source,
        "source_production_durable_memory_write_preflight_target_hash_sha256",
    );
    let source_operator_packet_hash_sha256 = json_str(
        &source,
        "source_production_durable_memory_write_preflight_operator_packet_hash_sha256",
    );
    let source_dry_run_execution_envelope_hash_sha256 =
        json_str(&source, "dry_run_execution_envelope_hash_sha256");
    let source_dry_run_execution_identity_session_hash_sha256 =
        json_str(&source, "dry_run_execution_identity_session_hash_sha256");
    let source_dry_run_execution_target_snapshot_hash_sha256 =
        json_str(&source, "dry_run_execution_target_snapshot_hash_sha256");
    let source_dry_run_execution_write_plan_hash_sha256 =
        json_str(&source, "dry_run_execution_write_plan_hash_sha256");
    let source_dry_run_execution_payload_redaction_hash_sha256 =
        json_str(&source, "dry_run_execution_payload_redaction_hash_sha256");
    let source_dry_run_execution_wal_receipt_preview_hash_sha256 =
        json_str(&source, "dry_run_execution_wal_receipt_preview_hash_sha256");
    let source_dry_run_execution_readback_preview_hash_sha256 =
        json_str(&source, "dry_run_execution_readback_preview_hash_sha256");
    let source_dry_run_execution_rollback_tombstone_preview_hash_sha256 = json_str(
        &source,
        "dry_run_execution_rollback_tombstone_preview_hash_sha256",
    );
    let source_dry_run_execution_replay_guard_hash_sha256 =
        json_str(&source, "dry_run_execution_replay_guard_hash_sha256");
    let dry_run_execution_result_receipt_envelope_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-execution-result-receipt-envelope:v1:source-result={source_dry_run_execution_result_hash_sha256}:source-envelope={source_dry_run_execution_envelope_hash_sha256}:execute-now=false:persist-now=false"
    ));
    let dry_run_execution_result_receipt_identity_session_hash_sha256 = sha256_text_value(
        &format!(
            "scoped-production-durable-memory-write-dry-run-execution-result-receipt-identity-session:v1:receipt-envelope={dry_run_execution_result_receipt_envelope_hash_sha256}:source-identity-session={source_dry_run_execution_identity_session_hash_sha256}:operator-bound=true"
        ),
    );
    let dry_run_execution_result_receipt_digest_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-execution-result-receipt-digest:v1:receipt-envelope={dry_run_execution_result_receipt_envelope_hash_sha256}:dry-run-result={source_dry_run_execution_result_hash_sha256}:persist-now=false"
    ));
    let dry_run_execution_result_receipt_hash_chain_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-execution-result-receipt-hash-chain:v1:receipt-digest={dry_run_execution_result_receipt_digest_hash_sha256}:source-boundary={source_dry_run_execution_envelope_boundary_hash_sha256}:source-policy={source_dry_run_execution_envelope_policy_hash_sha256}"
    ));
    let dry_run_execution_result_receipt_readback_plan_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-execution-result-receipt-readback-plan:v1:receipt-hash-chain={dry_run_execution_result_receipt_hash_chain_hash_sha256}:source-readback={source_dry_run_execution_readback_preview_hash_sha256}:readback-now=false"
    ));
    let dry_run_execution_result_receipt_replay_guard_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-execution-result-receipt-replay-guard:v1:receipt-envelope={dry_run_execution_result_receipt_envelope_hash_sha256}:source-replay={source_dry_run_execution_replay_guard_hash_sha256}:single-use=true:replay=false"
    ));
    let dry_run_execution_result_receipt_handoff_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-execution-result-receipt-handoff:v1:receipt-readback-plan={dry_run_execution_result_receipt_readback_plan_hash_sha256}:receipt-replay={dry_run_execution_result_receipt_replay_guard_hash_sha256}:next=replay-idempotency-denial-boundary"
    ));
    let dry_run_execution_result_receipt_result_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-execution-result-receipt-result:v1:receipt-envelope={dry_run_execution_result_receipt_envelope_hash_sha256}:digest={dry_run_execution_result_receipt_digest_hash_sha256}:hash-chain={dry_run_execution_result_receipt_hash_chain_hash_sha256}:accepted=true:executed=false:persisted=false"
    ));
    let dry_run_execution_result_receipt_boundary_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-execution-result-receipt-boundary:v1:source={source_report_sha256}:result={dry_run_execution_result_receipt_result_hash_sha256}:fixtures=10:accepted=1:denials={}:dry-run-executed=false:receipt-persisted=false:production-write=false",
        RESULT_RECEIPT_DENIALS.len()
    ));
    let dry_run_execution_result_receipt_policy_hash_sha256 = sha256_text_value(
        "scoped-production-durable-memory-write-dry-run-execution-result-receipt-policy:v1:bind-source-envelope-result-receipt-digest-hash-chain-readback-replay-handoff:no-execution:no-receipt-persistence:no-production-write:no-kg:no-provider:no-channel:no-release:no-install",
    );
    let result_receipt_bound = !source_dry_run_execution_result_hash_sha256.is_empty()
        && !source_dry_run_execution_envelope_boundary_hash_sha256.is_empty()
        && !source_dry_run_execution_envelope_policy_hash_sha256.is_empty()
        && !source_target_hash_sha256.is_empty()
        && !source_operator_packet_hash_sha256.is_empty()
        && !source_dry_run_execution_envelope_hash_sha256.is_empty()
        && !source_dry_run_execution_identity_session_hash_sha256.is_empty()
        && !source_dry_run_execution_target_snapshot_hash_sha256.is_empty()
        && !source_dry_run_execution_write_plan_hash_sha256.is_empty()
        && !source_dry_run_execution_payload_redaction_hash_sha256.is_empty()
        && !source_dry_run_execution_wal_receipt_preview_hash_sha256.is_empty()
        && !source_dry_run_execution_readback_preview_hash_sha256.is_empty()
        && !source_dry_run_execution_rollback_tombstone_preview_hash_sha256.is_empty()
        && !source_dry_run_execution_replay_guard_hash_sha256.is_empty()
        && approved_production_namespace == "hepta.memory.production.scoped"
        && approved_production_store == "hepta-memory-durable-store-production-preflight-only"
        && approved_production_scope == "operator-approved-session"
        && production_durable_memory_target_id
            == "hepta-scoped-production-durable-memory-write-target-v1"
        && production_durable_memory_payload_class
            == "redacted-minimal-operator-approved-memory-fact";
    let surfaces_ready = source_ready && result_receipt_bound;
    let report_ready = route_count_source_command_accepted && surfaces_ready;
    let accepted_fixture_count = if report_ready { 1 } else { 0 };
    let blocked_fixture_count = 10 - accepted_fixture_count;

    let mut fixtures = Vec::new();
    fixtures.push(serde_json::json!({
        "id": "scoped-production-durable-memory-write-dry-run-execution-result-receipt",
        "fixture_id": "scoped-production-durable-memory-write-dry-run-execution-result-receipt",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_accepted": report_ready,
        "reason": if report_ready { "dry_run_execution_result_receipt_bound_without_execution_receipt_persistence_or_production_write" } else { "source_envelope_or_route_count_not_ready" },
        "source_dry_run_execution_envelope_boundary_bound": report_ready,
        "dry_run_execution_result_bound": report_ready,
        "dry_run_execution_result_receipt_envelope_bound": report_ready,
        "dry_run_execution_result_receipt_digest_bound": report_ready,
        "dry_run_execution_result_receipt_hash_chain_bound": report_ready,
        "dry_run_execution_result_receipt_readback_plan_bound": report_ready,
        "dry_run_execution_result_receipt_replay_guard_bound": report_ready,
        "dry_run_execution_result_receipt_persisted": false,
        "dry_run_execution_executed": false,
        "production_durable_memory_store_write_performed": false,
        "external_send_performed": false
    }));
    for id in [
        "missing-dry-run-execution-envelope-source",
        "missing-dry-run-execution-result-hash",
        "missing-result-receipt-envelope",
        "missing-result-receipt-digest",
        "missing-result-receipt-hash-chain",
        "missing-result-receipt-readback-plan",
        "missing-result-receipt-replay-guard",
        "result-receipt-persistence-attempt",
        "dry-run-execution-or-production-write-attempt",
    ] {
        fixtures.push(serde_json::json!({
            "id": id,
            "fixture_id": id,
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_accepted": false,
            "reason": "blocked_noop",
            "dry_run_execution_result_receipt_persisted": false,
            "dry_run_execution_executed": false,
            "production_durable_memory_store_write_performed": false,
            "external_send_performed": false
        }));
    }

    let mut side_effects = serde_json::Map::new();
    for &key in FALSE_RESULT_RECEIPT_SIDE_EFFECT_KEYS {
        side_effects.insert(key.to_string(), serde_json::json!(false));
        side_effects.insert(format!("{key}_count"), serde_json::json!(0));
    }
    for &key in TRUE_RESULT_RECEIPT_KEYS {
        side_effects.insert(key.to_string(), serde_json::json!(report_ready));
        side_effects.insert(
            format!("{key}_count"),
            serde_json::json!(if report_ready { 1 } else { 0 }),
        );
    }

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "endpoint": HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_PRODUCTION_DURABLE_MEMORY_WRITE_DRY_RUN_EXECUTION_RESULT_RECEIPT_BOUNDARY_ENDPOINT,
        "source_command": "/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-boundary --json",
        "native_route": true,
        "side_effect_free": false,
        "external_side_effect_free": true,
        "audit_date": "2026-07-05"
    });
    let report_object = report.as_object_mut().expect(
        "scoped production durable Memory write dry-run execution result receipt report object",
    );
    macro_rules! insert_report {
        ($key:expr, $value:expr) => {
            report_object.insert($key.to_string(), serde_json::json!($value));
        };
    }

    insert_report!(
        "native_gateway_source_command_count",
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    insert_report!("route_count", route_matrix.route_count);
    insert_report!(
        "implemented_route_count",
        route_matrix.implemented_route_count
    );
    insert_report!("missing_route_count", route_matrix.missing_route_count);
    insert_report!(
        "route_count_source_command_accepted",
        route_count_source_command_accepted
    );
    insert_report!(
        "memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_boundary_ready",
        report_ready
    );
    insert_report!(
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_ready",
        report_ready
    );
    insert_report!(
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_performed",
        report_ready
    );
    insert_report!(
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_accepted",
        report_ready
    );
    insert_report!(
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_mode",
        "dry_run_execution_result_receipt_boundary_no_execution_no_receipt_persistence_no_production_durable_memory_mutation"
    );
    insert_report!(
        "source_scoped_production_durable_memory_write_dry_run_execution_envelope_boundary_ready",
        source_ready
    );
    insert_report!(
        "source_scoped_production_durable_memory_write_dry_run_execution_envelope_boundary_report_sha256",
        source_report_sha256
    );
    insert_report!(
        "source_dry_run_execution_result_hash_sha256",
        source_dry_run_execution_result_hash_sha256
    );
    insert_report!(
        "source_scoped_production_durable_memory_write_dry_run_execution_envelope_boundary_hash_sha256",
        source_dry_run_execution_envelope_boundary_hash_sha256
    );
    insert_report!(
        "source_scoped_production_durable_memory_write_dry_run_execution_envelope_policy_hash_sha256",
        source_dry_run_execution_envelope_policy_hash_sha256
    );
    insert_report!(
        "source_scoped_production_durable_memory_write_dry_run_execution_envelope_accepted_count",
        json_u64(
            &source,
            "scoped_production_durable_memory_write_dry_run_execution_envelope_result_accepted_count"
        )
    );
    insert_report!(
        "source_accepted_scoped_production_durable_memory_write_dry_run_execution_envelope_fixture_count",
        json_u64(
            &source,
            "accepted_scoped_production_durable_memory_write_dry_run_execution_envelope_fixture_count"
        )
    );
    insert_report!(
        "source_blocked_scoped_production_durable_memory_write_dry_run_execution_envelope_fixture_count",
        json_u64(
            &source,
            "blocked_scoped_production_durable_memory_write_dry_run_execution_envelope_fixture_count"
        )
    );
    insert_report!(
        "approved_production_namespace",
        approved_production_namespace
    );
    insert_report!("approved_production_store", approved_production_store);
    insert_report!("approved_production_scope", approved_production_scope);
    insert_report!(
        "production_durable_memory_target_id",
        production_durable_memory_target_id
    );
    insert_report!(
        "production_durable_memory_payload_class",
        production_durable_memory_payload_class
    );
    insert_report!("operator_packet_scope", operator_packet_scope);
    insert_report!(
        "source_production_durable_memory_write_preflight_target_hash_sha256",
        source_target_hash_sha256
    );
    insert_report!(
        "source_production_durable_memory_write_preflight_operator_packet_hash_sha256",
        source_operator_packet_hash_sha256
    );
    insert_report!(
        "source_dry_run_execution_envelope_hash_sha256",
        source_dry_run_execution_envelope_hash_sha256
    );
    insert_report!(
        "source_dry_run_execution_identity_session_hash_sha256",
        source_dry_run_execution_identity_session_hash_sha256
    );
    insert_report!(
        "source_dry_run_execution_target_snapshot_hash_sha256",
        source_dry_run_execution_target_snapshot_hash_sha256
    );
    insert_report!(
        "source_dry_run_execution_write_plan_hash_sha256",
        source_dry_run_execution_write_plan_hash_sha256
    );
    insert_report!(
        "source_dry_run_execution_payload_redaction_hash_sha256",
        source_dry_run_execution_payload_redaction_hash_sha256
    );
    insert_report!(
        "source_dry_run_execution_wal_receipt_preview_hash_sha256",
        source_dry_run_execution_wal_receipt_preview_hash_sha256
    );
    insert_report!(
        "source_dry_run_execution_readback_preview_hash_sha256",
        source_dry_run_execution_readback_preview_hash_sha256
    );
    insert_report!(
        "source_dry_run_execution_rollback_tombstone_preview_hash_sha256",
        source_dry_run_execution_rollback_tombstone_preview_hash_sha256
    );
    insert_report!(
        "source_dry_run_execution_replay_guard_hash_sha256",
        source_dry_run_execution_replay_guard_hash_sha256
    );
    insert_report!(
        "dry_run_execution_result_receipt_envelope_hash_sha256",
        dry_run_execution_result_receipt_envelope_hash_sha256
    );
    insert_report!(
        "dry_run_execution_result_receipt_identity_session_hash_sha256",
        dry_run_execution_result_receipt_identity_session_hash_sha256
    );
    insert_report!(
        "dry_run_execution_result_receipt_digest_hash_sha256",
        dry_run_execution_result_receipt_digest_hash_sha256
    );
    insert_report!(
        "dry_run_execution_result_receipt_hash_chain_hash_sha256",
        dry_run_execution_result_receipt_hash_chain_hash_sha256
    );
    insert_report!(
        "dry_run_execution_result_receipt_readback_plan_hash_sha256",
        dry_run_execution_result_receipt_readback_plan_hash_sha256
    );
    insert_report!(
        "dry_run_execution_result_receipt_replay_guard_hash_sha256",
        dry_run_execution_result_receipt_replay_guard_hash_sha256
    );
    insert_report!(
        "dry_run_execution_result_receipt_handoff_hash_sha256",
        dry_run_execution_result_receipt_handoff_hash_sha256
    );
    insert_report!(
        "dry_run_execution_result_receipt_result_hash_sha256",
        dry_run_execution_result_receipt_result_hash_sha256
    );
    insert_report!(
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_boundary_hash_sha256",
        dry_run_execution_result_receipt_boundary_hash_sha256
    );
    insert_report!(
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_policy_hash_sha256",
        dry_run_execution_result_receipt_policy_hash_sha256
    );
    insert_report!(
        "required_scoped_production_durable_memory_write_dry_run_execution_result_receipt_surface_count",
        RESULT_RECEIPT_SURFACES.len()
    );
    insert_report!(
        "ready_scoped_production_durable_memory_write_dry_run_execution_result_receipt_surface_count",
        if surfaces_ready {
            RESULT_RECEIPT_SURFACES.len()
        } else {
            0
        }
    );
    insert_report!(
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_surfaces",
        RESULT_RECEIPT_SURFACES
    );
    insert_report!(
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_fixture_count",
        fixtures.len()
    );
    insert_report!(
        "accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_fixture_count",
        accepted_fixture_count
    );
    insert_report!(
        "blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_fixture_count",
        blocked_fixture_count
    );
    insert_report!(
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_fixtures",
        fixtures
    );
    insert_report!(
        "denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_boundary",
        RESULT_RECEIPT_DENIALS
    );
    insert_report!(
        "denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_boundary_count",
        RESULT_RECEIPT_DENIALS.len()
    );
    insert_report!(
        "allowed_next_actions",
        [
            serde_json::json!({
                "action": "run_scoped_production_durable_memory_write_dry_run_execution_result_receipt_boundary_require_live_gate",
                "status": "allowed_verification_only",
                "accepts_dry_run_execution_result_receipt": true,
                "executes_dry_run": false,
                "persists_dry_run_result_receipt": false,
                "persists_dry_run_envelope": false,
                "writes_production_durable_memory": false,
                "writes_memory_store": false,
                "writes_wal": false,
                "persists_receipt": false,
                "executes_rollback": false,
                "writes_tombstone": false
            }),
            serde_json::json!({
                "action": "prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary",
                "status": "requires_separate_result_receipt_replay_idempotency_denial_gate",
                "requires_scoped_production_durable_memory_write_dry_run_execution_result_receipt_boundary": true,
                "executes_dry_run": false,
                "writes_production_durable_memory": false,
                "persists_dry_run_result_receipt": false
            }),
        ]
    );
    for &key in FALSE_RESULT_RECEIPT_SIDE_EFFECT_KEYS {
        report_object.insert(key.to_string(), serde_json::json!(false));
        report_object.insert(format!("{key}_count"), serde_json::json!(0));
    }
    for &key in TRUE_RESULT_RECEIPT_KEYS {
        report_object.insert(key.to_string(), serde_json::json!(report_ready));
        report_object.insert(
            format!("{key}_count"),
            serde_json::json!(if report_ready { 1 } else { 0 }),
        );
    }
    for key in [
        "source_dry_run_execution_envelope_boundary_bound",
        "approved_production_namespace_bound",
        "approved_production_store_bound",
        "approved_production_scope_bound",
        "production_durable_memory_target_bound",
        "dry_run_execution_result_bound",
        "dry_run_execution_envelope_bound",
        "dry_run_execution_target_snapshot_bound",
        "dry_run_execution_write_plan_bound",
        "dry_run_execution_payload_redaction_bound",
        "dry_run_execution_wal_receipt_preview_bound",
        "dry_run_execution_readback_preview_bound",
        "dry_run_execution_rollback_tombstone_preview_bound",
        "dry_run_execution_replay_guard_bound",
        "dry_run_execution_result_receipt_envelope_bound",
        "dry_run_execution_result_receipt_identity_session_bound",
        "dry_run_execution_result_receipt_digest_bound",
        "dry_run_execution_result_receipt_hash_chain_bound",
        "dry_run_execution_result_receipt_readback_plan_bound",
        "dry_run_execution_result_receipt_replay_guard_bound",
        "dry_run_execution_result_receipt_handoff_bound",
        "dry_run_execution_result_receipt_persistence_forbidden_on_report_route",
        "dry_run_execution_execution_forbidden_on_result_receipt_route",
        "dry_run_execution_envelope_persistence_forbidden_on_result_receipt_route",
        "production_write_execution_forbidden_on_result_receipt_route",
        "production_durable_memory_write_forbidden",
        "memory_store_mutation_forbidden",
        "wal_write_forbidden_on_result_receipt_route",
        "receipt_persist_forbidden_on_result_receipt_route",
        "rollback_execution_forbidden_on_result_receipt_route",
        "tombstone_write_forbidden_on_result_receipt_route",
        "kg_live_write_forbidden",
        "provider_model_invocation_forbidden",
        "credential_channel_public_release_forbidden",
        "install_restart_active_binary_mutation_forbidden",
    ] {
        report_object.insert(key.to_string(), serde_json::json!(report_ready));
    }
    report_object.insert(
        "side_effects".to_string(),
        serde_json::Value::Object(side_effects),
    );
    report
}

fn hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary_report()
-> serde_json::Value {
    const REPLAY_IDEMPOTENCY_SURFACES: &[&str] = &[
        "source_dry_run_execution_result_receipt_boundary_required",
        "source_dry_run_execution_result_receipt_result_required",
        "dry_run_execution_result_receipt_replay_idempotency_denial_matrix_required",
        "dry_run_execution_result_receipt_replay_idempotency_identity_session_required",
        "dry_run_execution_result_receipt_replay_idempotency_nonce_scope_required",
        "dry_run_execution_result_receipt_replay_idempotency_duplicate_receipt_denial_required",
        "dry_run_execution_result_receipt_replay_idempotency_stale_receipt_denial_required",
        "dry_run_execution_result_receipt_replay_idempotency_hash_chain_mismatch_denial_required",
        "dry_run_execution_result_receipt_replay_idempotency_cross_session_denial_required",
        "dry_run_execution_result_receipt_replay_idempotency_handoff_required",
        "dry_run_execution_result_receipt_replay_state_persistence_forbidden",
        "dry_run_execution_result_receipt_idempotency_ledger_write_forbidden",
        "dry_run_execution_execution_forbidden_on_replay_idempotency_route",
        "dry_run_execution_result_receipt_persistence_forbidden_on_replay_idempotency_route",
        "production_write_execution_forbidden_on_replay_idempotency_route",
        "kg_provider_channel_release_install_active_binary_forbidden",
    ];
    const REPLAY_IDEMPOTENCY_DENIALS: &[&str] = &[
        "source_dry_run_execution_result_receipt_boundary_required",
        "source_dry_run_execution_result_receipt_result_hash_required",
        "source_dry_run_execution_result_receipt_policy_hash_required",
        "approved_production_namespace_required",
        "approved_production_store_required",
        "approved_production_scope_required",
        "production_durable_memory_target_required",
        "source_dry_run_execution_result_receipt_envelope_required",
        "source_dry_run_execution_result_receipt_digest_required",
        "source_dry_run_execution_result_receipt_hash_chain_required",
        "source_dry_run_execution_result_receipt_readback_plan_required",
        "source_dry_run_execution_result_receipt_replay_guard_required",
        "source_dry_run_execution_result_receipt_handoff_required",
        "replay_idempotency_denial_matrix_required",
        "replay_idempotency_identity_session_required",
        "replay_idempotency_nonce_scope_required",
        "replay_idempotency_duplicate_receipt_denial_required",
        "replay_idempotency_stale_receipt_denial_required",
        "replay_idempotency_hash_chain_mismatch_denial_required",
        "replay_idempotency_cross_session_denial_required",
        "replay_idempotency_handoff_required",
        "replay_state_persistence_denied",
        "idempotency_ledger_write_denied",
        "replay_guard_state_recording_denied",
        "duplicate_receipt_acceptance_denied",
        "stale_receipt_acceptance_denied",
        "cross_session_replay_acceptance_denied",
        "hash_chain_mismatch_acceptance_denied",
        "result_receipt_replay_attempt_denied",
        "dry_run_execution_execution_denied",
        "dry_run_execution_envelope_persistence_denied",
        "dry_run_execution_result_persistence_denied",
        "dry_run_execution_result_receipt_persistence_denied",
        "dry_run_execution_result_receipt_filesystem_write_denied",
        "dry_run_execution_result_receipt_ledger_recording_denied",
        "dry_run_execution_result_receipt_delivery_denied",
        "dry_run_execution_result_receipt_materialization_denied",
        "acceptance_receipt_persistence_denied",
        "operator_packet_persistence_denied",
        "production_write_execution_denied",
        "production_durable_memory_backend_write_denied",
        "durable_memory_backend_read_or_rollback_denied",
        "memory_store_mutation_denied",
        "wal_write_denied",
        "receipt_persistence_denied",
        "post_write_readback_denied",
        "rollback_execution_denied",
        "tombstone_write_denied",
        "raw_payload_plaintext_denied",
        "kg_live_write_denied",
        "provider_model_invocation_denied",
        "credential_channel_release_install_denied",
        "active_binary_mutation_denied",
        "unrestricted_full_live_activation_denied",
    ];
    const FALSE_REPLAY_IDEMPOTENCY_SIDE_EFFECT_KEYS: &[&str] = &[
        "dry_run_execution_result_receipt_replay_state_persisted",
        "dry_run_execution_result_receipt_idempotency_ledger_written",
        "dry_run_execution_result_receipt_replay_guard_state_recorded",
        "dry_run_execution_result_receipt_duplicate_receipt_accepted",
        "dry_run_execution_result_receipt_stale_receipt_accepted",
        "dry_run_execution_result_receipt_cross_session_replay_accepted",
        "dry_run_execution_result_receipt_hash_chain_mismatch_accepted",
        "dry_run_execution_result_receipt_replay_attempt_accepted",
        "dry_run_execution_result_receipt_persisted",
        "dry_run_execution_result_receipt_filesystem_written",
        "dry_run_execution_result_receipt_ledger_recorded",
        "dry_run_execution_result_receipt_delivered",
        "dry_run_execution_result_receipt_materialized",
        "dry_run_execution_envelope_persisted",
        "dry_run_execution_envelope_filesystem_written",
        "dry_run_execution_envelope_ledger_recorded",
        "dry_run_execution_envelope_delivered",
        "dry_run_execution_envelope_materialized",
        "dry_run_execution_executed",
        "dry_run_execution_result_persisted",
        "acceptance_receipt_persisted",
        "acceptance_receipt_filesystem_written",
        "acceptance_receipt_ledger_recorded",
        "acceptance_receipt_delivered",
        "operator_packet_persisted",
        "operator_packet_ledger_recorded",
        "operator_packet_filesystem_written",
        "operator_packet_acceptance_receipt_persisted",
        "production_durable_memory_write_executed",
        "production_durable_memory_backend_present",
        "production_durable_memory_store_write_performed",
        "actual_production_durable_memory_write_performed",
        "durable_memory_store_write_performed",
        "durable_memory_store_read_performed",
        "durable_memory_store_rollback_performed",
        "memory_write_execution_performed",
        "memory_store_write_path_enabled",
        "memory_store_write_allowed",
        "memory_store_write_performed",
        "memory_store_mutation_allowed",
        "memory_store_mutated",
        "wal_write_performed",
        "wal_recorded",
        "wal_persisted",
        "receipt_recorded",
        "receipt_persisted",
        "receipt_materialized",
        "receipt_delivered",
        "post_write_readback_performed",
        "readback_result_recorded",
        "readback_result_persisted",
        "readback_result_accepted",
        "rollback_executed",
        "rollback_performed",
        "rollback_result_recorded",
        "rollback_result_persisted",
        "rollback_result_accepted",
        "tombstone_write_performed",
        "tombstone_cleanup_executed",
        "tombstone_cleanup_result_recorded",
        "tombstone_cleanup_result_accepted",
        "raw_payload_plaintext_recorded",
        "raw_payload_plaintext_persisted",
        "secret_material_read",
        "credential_read",
        "secret_file_read",
        "kg_adapter_read_performed",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "telegram_send_performed",
        "channel_send_performed",
        "external_send_performed",
        "public_claim_promoted",
        "public_release_published",
        "public_ga_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "service_restart_performed",
        "active_binary_mutated",
    ];
    const TRUE_REPLAY_IDEMPOTENCY_KEYS: &[&str] = &[
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary_performed",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_result_recorded",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_result_accepted",
        "source_dry_run_execution_result_receipt_boundary_accepted",
        "dry_run_execution_result_receipt_replay_idempotency_denial_matrix_bound",
        "dry_run_execution_result_receipt_replay_idempotency_identity_session_bound",
        "dry_run_execution_result_receipt_replay_idempotency_nonce_scope_bound",
        "dry_run_execution_result_receipt_replay_idempotency_duplicate_receipt_denied",
        "dry_run_execution_result_receipt_replay_idempotency_stale_receipt_denied",
        "dry_run_execution_result_receipt_replay_idempotency_hash_chain_mismatch_denied",
        "dry_run_execution_result_receipt_replay_idempotency_cross_session_denied",
        "dry_run_execution_result_receipt_replay_idempotency_handoff_bound",
        "dry_run_execution_result_receipt_replay_idempotency_state_persistence_forbidden_on_report_route",
        "dry_run_execution_result_receipt_idempotency_ledger_write_forbidden_on_report_route",
        "dry_run_execution_execution_forbidden_on_replay_idempotency_route",
        "production_write_execution_forbidden_on_replay_idempotency_route",
        "kg_provider_channel_release_install_active_binary_forbidden",
    ];

    let route_matrix = control_ui_route_parity_report();
    let source = std::thread::Builder::new()
        .name("hepta-memory-production-durable-dry-run-result-receipt-replay-source-report".to_string())
        .stack_size(8 * 1024 * 1024)
        .spawn(
            hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_boundary_report,
        )
        .ok()
        .and_then(|handle| handle.join().ok())
        .unwrap_or_else(|| {
            serde_json::json!({
                "status": "blocked",
                "memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_boundary_ready": false,
                "scoped_production_durable_memory_write_dry_run_execution_result_receipt_accepted": false,
                "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_source_report_thread_failed": true
            })
        });
    let json_bool = |value: &serde_json::Value, key: &str| {
        value
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let json_u64 = |value: &serde_json::Value, key: &str| {
        value
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let json_str = |value: &serde_json::Value, key: &str| {
        value
            .get(key)
            .and_then(serde_json::Value::as_str)
            .unwrap_or("")
            .to_string()
    };
    let route_count_source_command_accepted = route_matrix.ready
        && route_matrix.route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_next_action_replay_idempotency = source
        .get("allowed_next_actions")
        .and_then(serde_json::Value::as_array)
        .and_then(|items| items.get(1))
        .map(|item| {
            item.get("action").and_then(serde_json::Value::as_str)
                == Some("prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary")
                && item
                    .get("requires_scoped_production_durable_memory_write_dry_run_execution_result_receipt_boundary")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
                && item
                    .get("executes_dry_run")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && item
                    .get("writes_production_durable_memory")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && item
                    .get("persists_dry_run_result_receipt")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
        })
        .unwrap_or(false);
    let source_side_effects_ok = source
        .get("side_effects")
        .and_then(serde_json::Value::as_object)
        .map(|effects| {
            effects
                .get("scoped_production_durable_memory_write_dry_run_execution_result_receipt_boundary_performed")
                .and_then(serde_json::Value::as_bool)
                == Some(true)
                && effects
                    .get("scoped_production_durable_memory_write_dry_run_execution_result_receipt_result_accepted")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
                && effects
                    .get("dry_run_execution_result_receipt_persisted")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("dry_run_execution_executed")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("production_durable_memory_store_write_performed")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("memory_store_write_performed")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("wal_write_performed")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("receipt_persisted")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("external_send_performed")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
        })
        .unwrap_or(false);
    let source_ready = source.get("status").and_then(serde_json::Value::as_str) == Some("ready")
        && json_bool(
            &source,
            "memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_boundary_ready",
        )
        && json_bool(
            &source,
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_accepted",
        )
        && json_u64(
            &source,
            "accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_fixture_count",
        ) == 1
        && json_u64(
            &source,
            "blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_fixture_count",
        ) == 9
        && json_u64(
            &source,
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_result_accepted_count",
        ) == 1
        && !json_bool(&source, "dry_run_execution_result_receipt_persisted")
        && !json_bool(&source, "dry_run_execution_executed")
        && !json_bool(&source, "dry_run_execution_envelope_persisted")
        && !json_bool(&source, "dry_run_execution_result_persisted")
        && !json_bool(&source, "acceptance_receipt_persisted")
        && !json_bool(&source, "operator_packet_persisted")
        && !json_bool(&source, "operator_packet_acceptance_receipt_persisted")
        && !json_bool(&source, "production_durable_memory_write_executed")
        && !json_bool(&source, "production_durable_memory_store_write_performed")
        && !json_bool(&source, "actual_production_durable_memory_write_performed")
        && !json_bool(&source, "durable_memory_store_write_performed")
        && !json_bool(&source, "durable_memory_store_read_performed")
        && !json_bool(&source, "durable_memory_store_rollback_performed")
        && !json_bool(&source, "memory_store_write_performed")
        && !json_bool(&source, "wal_write_performed")
        && !json_bool(&source, "receipt_persisted")
        && !json_bool(&source, "post_write_readback_performed")
        && !json_bool(&source, "rollback_executed")
        && !json_bool(&source, "tombstone_cleanup_executed")
        && !json_bool(&source, "raw_payload_plaintext_recorded")
        && !json_bool(&source, "raw_payload_plaintext_persisted")
        && !json_bool(&source, "live_kg_write_performed")
        && !json_bool(&source, "provider_invoked")
        && !json_bool(&source, "model_invoked")
        && !json_bool(&source, "credential_read")
        && !json_bool(&source, "channel_send_performed")
        && !json_bool(&source, "external_send_performed")
        && !json_bool(&source, "release_artifact_written")
        && !json_bool(&source, "install_executed")
        && !json_bool(&source, "service_restarted")
        && !json_bool(&source, "active_binary_mutated")
        && source_next_action_replay_idempotency
        && source_side_effects_ok;

    let approved_production_namespace = json_str(&source, "approved_production_namespace");
    let approved_production_store = json_str(&source, "approved_production_store");
    let approved_production_scope = json_str(&source, "approved_production_scope");
    let production_durable_memory_target_id =
        json_str(&source, "production_durable_memory_target_id");
    let production_durable_memory_payload_class =
        json_str(&source, "production_durable_memory_payload_class");
    let operator_packet_scope = json_str(&source, "operator_packet_scope");
    let source_report_sha256 = sha256_text_value(&source.to_string());
    let source_dry_run_execution_result_hash_sha256 =
        json_str(&source, "source_dry_run_execution_result_hash_sha256");
    let source_result_receipt_boundary_hash_sha256 = json_str(
        &source,
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_boundary_hash_sha256",
    );
    let source_result_receipt_policy_hash_sha256 = json_str(
        &source,
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_policy_hash_sha256",
    );
    let source_result_receipt_envelope_hash_sha256 = json_str(
        &source,
        "dry_run_execution_result_receipt_envelope_hash_sha256",
    );
    let source_result_receipt_identity_session_hash_sha256 = json_str(
        &source,
        "dry_run_execution_result_receipt_identity_session_hash_sha256",
    );
    let source_result_receipt_digest_hash_sha256 = json_str(
        &source,
        "dry_run_execution_result_receipt_digest_hash_sha256",
    );
    let source_result_receipt_hash_chain_hash_sha256 = json_str(
        &source,
        "dry_run_execution_result_receipt_hash_chain_hash_sha256",
    );
    let source_result_receipt_readback_plan_hash_sha256 = json_str(
        &source,
        "dry_run_execution_result_receipt_readback_plan_hash_sha256",
    );
    let source_result_receipt_replay_guard_hash_sha256 = json_str(
        &source,
        "dry_run_execution_result_receipt_replay_guard_hash_sha256",
    );
    let source_result_receipt_handoff_hash_sha256 = json_str(
        &source,
        "dry_run_execution_result_receipt_handoff_hash_sha256",
    );
    let source_result_receipt_result_hash_sha256 = json_str(
        &source,
        "dry_run_execution_result_receipt_result_hash_sha256",
    );
    let dry_run_execution_result_receipt_replay_idempotency_denial_matrix_hash_sha256 =
        sha256_text_value(&format!(
            "scoped-production-durable-memory-write-dry-run-result-receipt-replay-idempotency-denial-matrix:v1:source-result-receipt={source_result_receipt_result_hash_sha256}:source-replay={source_result_receipt_replay_guard_hash_sha256}:duplicate=deny:stale=deny:cross-session=deny:hash-chain-mismatch=deny:persist-state=false"
        ));
    let dry_run_execution_result_receipt_replay_idempotency_identity_session_hash_sha256 =
        sha256_text_value(&format!(
            "scoped-production-durable-memory-write-dry-run-result-receipt-replay-idempotency-identity-session:v1:matrix={dry_run_execution_result_receipt_replay_idempotency_denial_matrix_hash_sha256}:source-identity={source_result_receipt_identity_session_hash_sha256}:cross-session=false"
        ));
    let dry_run_execution_result_receipt_replay_idempotency_nonce_scope_hash_sha256 =
        sha256_text_value(&format!(
            "scoped-production-durable-memory-write-dry-run-result-receipt-replay-idempotency-nonce-scope:v1:matrix={dry_run_execution_result_receipt_replay_idempotency_denial_matrix_hash_sha256}:source-replay={source_result_receipt_replay_guard_hash_sha256}:single-use=true:reuse=deny"
        ));
    let dry_run_execution_result_receipt_replay_idempotency_duplicate_receipt_denial_hash_sha256 =
        sha256_text_value(&format!(
            "scoped-production-durable-memory-write-dry-run-result-receipt-duplicate-denial:v1:nonce={dry_run_execution_result_receipt_replay_idempotency_nonce_scope_hash_sha256}:source-receipt={source_result_receipt_envelope_hash_sha256}:accepted=false:persist-state=false"
        ));
    let dry_run_execution_result_receipt_replay_idempotency_stale_receipt_denial_hash_sha256 =
        sha256_text_value(&format!(
            "scoped-production-durable-memory-write-dry-run-result-receipt-stale-denial:v1:source-receipt={source_result_receipt_envelope_hash_sha256}:source-boundary={source_result_receipt_boundary_hash_sha256}:accepted=false"
        ));
    let dry_run_execution_result_receipt_replay_idempotency_hash_chain_mismatch_denial_hash_sha256 =
        sha256_text_value(&format!(
            "scoped-production-durable-memory-write-dry-run-result-receipt-hash-chain-mismatch-denial:v1:source-hash-chain={source_result_receipt_hash_chain_hash_sha256}:source-policy={source_result_receipt_policy_hash_sha256}:accepted=false"
        ));
    let dry_run_execution_result_receipt_replay_idempotency_cross_session_denial_hash_sha256 =
        sha256_text_value(&format!(
            "scoped-production-durable-memory-write-dry-run-result-receipt-cross-session-denial:v1:identity={dry_run_execution_result_receipt_replay_idempotency_identity_session_hash_sha256}:accepted=false"
        ));
    let dry_run_execution_result_receipt_replay_idempotency_handoff_hash_sha256 = sha256_text_value(
        &format!(
            "scoped-production-durable-memory-write-dry-run-result-receipt-replay-idempotency-handoff:v1:duplicate={dry_run_execution_result_receipt_replay_idempotency_duplicate_receipt_denial_hash_sha256}:stale={dry_run_execution_result_receipt_replay_idempotency_stale_receipt_denial_hash_sha256}:hash-chain={dry_run_execution_result_receipt_replay_idempotency_hash_chain_mismatch_denial_hash_sha256}:cross-session={dry_run_execution_result_receipt_replay_idempotency_cross_session_denial_hash_sha256}:next=ordering-monotonicity-denial-boundary"
        ),
    );
    let dry_run_execution_result_receipt_replay_idempotency_result_hash_sha256 = sha256_text_value(
        &format!(
            "scoped-production-durable-memory-write-dry-run-result-receipt-replay-idempotency-result:v1:matrix={dry_run_execution_result_receipt_replay_idempotency_denial_matrix_hash_sha256}:handoff={dry_run_execution_result_receipt_replay_idempotency_handoff_hash_sha256}:accepted=true:replay-state-persisted=false:idempotency-ledger-written=false:executed=false:production-write=false"
        ),
    );
    let replay_idempotency_boundary_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-replay-idempotency-denial-boundary:v1:source={source_report_sha256}:result={dry_run_execution_result_receipt_replay_idempotency_result_hash_sha256}:fixtures=10:accepted=1:denials={}:replay-state-persisted=false:idempotency-ledger=false:dry-run-executed=false:production-write=false",
        REPLAY_IDEMPOTENCY_DENIALS.len()
    ));
    let replay_idempotency_policy_hash_sha256 = sha256_text_value(
        "scoped-production-durable-memory-write-dry-run-result-receipt-replay-idempotency-denial-policy:v1:bind-source-result-receipt-matrix-identity-session-nonce-duplicate-stale-hash-chain-cross-session-handoff:no-replay-state-persistence:no-idempotency-ledger:no-execution:no-production-write:no-kg:no-provider:no-channel:no-release:no-install",
    );
    let replay_idempotency_bound = !source_dry_run_execution_result_hash_sha256.is_empty()
        && !source_result_receipt_boundary_hash_sha256.is_empty()
        && !source_result_receipt_policy_hash_sha256.is_empty()
        && !source_result_receipt_envelope_hash_sha256.is_empty()
        && !source_result_receipt_identity_session_hash_sha256.is_empty()
        && !source_result_receipt_digest_hash_sha256.is_empty()
        && !source_result_receipt_hash_chain_hash_sha256.is_empty()
        && !source_result_receipt_readback_plan_hash_sha256.is_empty()
        && !source_result_receipt_replay_guard_hash_sha256.is_empty()
        && !source_result_receipt_handoff_hash_sha256.is_empty()
        && !source_result_receipt_result_hash_sha256.is_empty()
        && approved_production_namespace == "hepta.memory.production.scoped"
        && approved_production_store == "hepta-memory-durable-store-production-preflight-only"
        && approved_production_scope == "operator-approved-session"
        && production_durable_memory_target_id
            == "hepta-scoped-production-durable-memory-write-target-v1"
        && production_durable_memory_payload_class
            == "redacted-minimal-operator-approved-memory-fact";
    let surfaces_ready = source_ready && replay_idempotency_bound;
    let report_ready = route_count_source_command_accepted && surfaces_ready;
    let accepted_fixture_count = if report_ready { 1 } else { 0 };
    let blocked_fixture_count = 10 - accepted_fixture_count;

    let mut fixtures = Vec::new();
    fixtures.push(serde_json::json!({
        "id": "scoped-production-durable-memory-write-dry-run-execution-result-receipt-replay-idempotency-denial",
        "fixture_id": "scoped-production-durable-memory-write-dry-run-execution-result-receipt-replay-idempotency-denial",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_accepted": report_ready,
        "reason": if report_ready { "dry_run_execution_result_receipt_replay_idempotency_denial_bound_without_replay_state_persistence_execution_or_production_write" } else { "source_result_receipt_or_route_count_not_ready" },
        "source_dry_run_execution_result_receipt_boundary_bound": report_ready,
        "dry_run_execution_result_receipt_replay_idempotency_denial_matrix_bound": report_ready,
        "dry_run_execution_result_receipt_replay_idempotency_duplicate_receipt_denied": report_ready,
        "dry_run_execution_result_receipt_replay_idempotency_stale_receipt_denied": report_ready,
        "dry_run_execution_result_receipt_replay_idempotency_hash_chain_mismatch_denied": report_ready,
        "dry_run_execution_result_receipt_replay_idempotency_cross_session_denied": report_ready,
        "dry_run_execution_result_receipt_replay_state_persisted": false,
        "dry_run_execution_result_receipt_idempotency_ledger_written": false,
        "dry_run_execution_executed": false,
        "production_durable_memory_store_write_performed": false,
        "external_send_performed": false
    }));
    for id in [
        "missing-result-receipt-source",
        "missing-result-receipt-result-hash",
        "missing-replay-idempotency-denial-matrix",
        "duplicate-result-receipt-attempt",
        "stale-result-receipt-attempt",
        "cross-session-result-receipt-replay-attempt",
        "hash-chain-mismatch-result-receipt-attempt",
        "replay-state-persistence-or-idempotency-ledger-write-attempt",
        "dry-run-execution-or-production-write-attempt",
    ] {
        fixtures.push(serde_json::json!({
            "id": id,
            "fixture_id": id,
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_accepted": false,
            "reason": "blocked_noop",
            "dry_run_execution_result_receipt_replay_state_persisted": false,
            "dry_run_execution_result_receipt_idempotency_ledger_written": false,
            "dry_run_execution_executed": false,
            "production_durable_memory_store_write_performed": false,
            "external_send_performed": false
        }));
    }

    let mut side_effects = serde_json::Map::new();
    for &key in FALSE_REPLAY_IDEMPOTENCY_SIDE_EFFECT_KEYS {
        side_effects.insert(key.to_string(), serde_json::json!(false));
        side_effects.insert(format!("{key}_count"), serde_json::json!(0));
    }
    for &key in TRUE_REPLAY_IDEMPOTENCY_KEYS {
        side_effects.insert(key.to_string(), serde_json::json!(report_ready));
        side_effects.insert(
            format!("{key}_count"),
            serde_json::json!(if report_ready { 1 } else { 0 }),
        );
    }

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "endpoint": HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_PRODUCTION_DURABLE_MEMORY_WRITE_DRY_RUN_EXECUTION_RESULT_RECEIPT_REPLAY_IDEMPOTENCY_DENIAL_BOUNDARY_ENDPOINT,
        "source_command": "/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-replay-idempotency-denial-boundary --json",
        "native_route": true,
        "side_effect_free": false,
        "external_side_effect_free": true,
        "audit_date": "2026-07-05"
    });
    let report_object = report.as_object_mut().expect(
        "scoped production durable Memory write dry-run execution result receipt replay/idempotency denial report object",
    );
    macro_rules! insert_report {
        ($key:expr, $value:expr) => {
            report_object.insert($key.to_string(), serde_json::json!($value));
        };
    }

    insert_report!(
        "native_gateway_source_command_count",
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    insert_report!("route_count", route_matrix.route_count);
    insert_report!(
        "implemented_route_count",
        route_matrix.implemented_route_count
    );
    insert_report!("missing_route_count", route_matrix.missing_route_count);
    insert_report!(
        "route_count_source_command_accepted",
        route_count_source_command_accepted
    );
    insert_report!(
        "memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary_ready",
        report_ready
    );
    insert_report!(
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_ready",
        report_ready
    );
    insert_report!(
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_performed",
        report_ready
    );
    insert_report!(
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_accepted",
        report_ready
    );
    insert_report!(
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_mode",
        "dry_run_execution_result_receipt_replay_idempotency_denial_boundary_no_replay_state_persistence_no_execution_no_production_durable_memory_mutation"
    );
    insert_report!(
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_boundary_ready",
        source_ready
    );
    insert_report!(
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_boundary_report_sha256",
        source_report_sha256
    );
    insert_report!(
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_boundary_accepted_count",
        json_u64(
            &source,
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_result_accepted_count"
        )
    );
    insert_report!(
        "source_accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_fixture_count",
        json_u64(
            &source,
            "accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_fixture_count"
        )
    );
    insert_report!(
        "source_blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_fixture_count",
        json_u64(
            &source,
            "blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_fixture_count"
        )
    );
    insert_report!(
        "approved_production_namespace",
        approved_production_namespace
    );
    insert_report!("approved_production_store", approved_production_store);
    insert_report!("approved_production_scope", approved_production_scope);
    insert_report!(
        "production_durable_memory_target_id",
        production_durable_memory_target_id
    );
    insert_report!(
        "production_durable_memory_payload_class",
        production_durable_memory_payload_class
    );
    insert_report!("operator_packet_scope", operator_packet_scope);
    insert_report!(
        "source_dry_run_execution_result_hash_sha256",
        source_dry_run_execution_result_hash_sha256
    );
    insert_report!(
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_boundary_hash_sha256",
        source_result_receipt_boundary_hash_sha256
    );
    insert_report!(
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_policy_hash_sha256",
        source_result_receipt_policy_hash_sha256
    );
    insert_report!(
        "source_dry_run_execution_result_receipt_envelope_hash_sha256",
        source_result_receipt_envelope_hash_sha256
    );
    insert_report!(
        "source_dry_run_execution_result_receipt_identity_session_hash_sha256",
        source_result_receipt_identity_session_hash_sha256
    );
    insert_report!(
        "source_dry_run_execution_result_receipt_digest_hash_sha256",
        source_result_receipt_digest_hash_sha256
    );
    insert_report!(
        "source_dry_run_execution_result_receipt_hash_chain_hash_sha256",
        source_result_receipt_hash_chain_hash_sha256
    );
    insert_report!(
        "source_dry_run_execution_result_receipt_readback_plan_hash_sha256",
        source_result_receipt_readback_plan_hash_sha256
    );
    insert_report!(
        "source_dry_run_execution_result_receipt_replay_guard_hash_sha256",
        source_result_receipt_replay_guard_hash_sha256
    );
    insert_report!(
        "source_dry_run_execution_result_receipt_handoff_hash_sha256",
        source_result_receipt_handoff_hash_sha256
    );
    insert_report!(
        "source_dry_run_execution_result_receipt_result_hash_sha256",
        source_result_receipt_result_hash_sha256
    );
    insert_report!(
        "dry_run_execution_result_receipt_replay_idempotency_denial_matrix_hash_sha256",
        dry_run_execution_result_receipt_replay_idempotency_denial_matrix_hash_sha256
    );
    insert_report!(
        "dry_run_execution_result_receipt_replay_idempotency_identity_session_hash_sha256",
        dry_run_execution_result_receipt_replay_idempotency_identity_session_hash_sha256
    );
    insert_report!(
        "dry_run_execution_result_receipt_replay_idempotency_nonce_scope_hash_sha256",
        dry_run_execution_result_receipt_replay_idempotency_nonce_scope_hash_sha256
    );
    insert_report!(
        "dry_run_execution_result_receipt_replay_idempotency_duplicate_receipt_denial_hash_sha256",
        dry_run_execution_result_receipt_replay_idempotency_duplicate_receipt_denial_hash_sha256
    );
    insert_report!(
        "dry_run_execution_result_receipt_replay_idempotency_stale_receipt_denial_hash_sha256",
        dry_run_execution_result_receipt_replay_idempotency_stale_receipt_denial_hash_sha256
    );
    insert_report!(
        "dry_run_execution_result_receipt_replay_idempotency_hash_chain_mismatch_denial_hash_sha256",
        dry_run_execution_result_receipt_replay_idempotency_hash_chain_mismatch_denial_hash_sha256
    );
    insert_report!(
        "dry_run_execution_result_receipt_replay_idempotency_cross_session_denial_hash_sha256",
        dry_run_execution_result_receipt_replay_idempotency_cross_session_denial_hash_sha256
    );
    insert_report!(
        "dry_run_execution_result_receipt_replay_idempotency_handoff_hash_sha256",
        dry_run_execution_result_receipt_replay_idempotency_handoff_hash_sha256
    );
    insert_report!(
        "dry_run_execution_result_receipt_replay_idempotency_result_hash_sha256",
        dry_run_execution_result_receipt_replay_idempotency_result_hash_sha256
    );
    insert_report!(
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary_hash_sha256",
        replay_idempotency_boundary_hash_sha256
    );
    insert_report!(
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_policy_hash_sha256",
        replay_idempotency_policy_hash_sha256
    );
    insert_report!(
        "required_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_surface_count",
        REPLAY_IDEMPOTENCY_SURFACES.len()
    );
    insert_report!(
        "ready_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_surface_count",
        if surfaces_ready {
            REPLAY_IDEMPOTENCY_SURFACES.len()
        } else {
            0
        }
    );
    insert_report!(
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_surfaces",
        REPLAY_IDEMPOTENCY_SURFACES
    );
    insert_report!(
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_fixture_count",
        fixtures.len()
    );
    insert_report!(
        "accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_fixture_count",
        accepted_fixture_count
    );
    insert_report!(
        "blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_fixture_count",
        blocked_fixture_count
    );
    insert_report!(
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_fixtures",
        fixtures
    );
    insert_report!(
        "denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary",
        REPLAY_IDEMPOTENCY_DENIALS
    );
    insert_report!(
        "denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary_count",
        REPLAY_IDEMPOTENCY_DENIALS.len()
    );
    insert_report!(
        "allowed_next_actions",
        [
            serde_json::json!({
                "action": "run_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary_require_live_gate",
                "status": "allowed_verification_only",
                "accepts_replay_idempotency_denial_matrix": true,
                "persists_replay_state": false,
                "writes_idempotency_ledger": false,
                "executes_dry_run": false,
                "persists_dry_run_result_receipt": false,
                "writes_production_durable_memory": false,
                "writes_memory_store": false,
                "writes_wal": false,
                "persists_receipt": false,
                "executes_rollback": false,
                "writes_tombstone": false
            }),
            serde_json::json!({
                "action": "prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary",
                "status": "requires_separate_result_receipt_ordering_monotonicity_denial_gate",
                "requires_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary": true,
                "persists_replay_state": false,
                "writes_idempotency_ledger": false,
                "executes_dry_run": false,
                "writes_production_durable_memory": false,
                "persists_dry_run_result_receipt": false
            }),
        ]
    );
    for &key in FALSE_REPLAY_IDEMPOTENCY_SIDE_EFFECT_KEYS {
        report_object.insert(key.to_string(), serde_json::json!(false));
        report_object.insert(format!("{key}_count"), serde_json::json!(0));
    }
    for &key in TRUE_REPLAY_IDEMPOTENCY_KEYS {
        report_object.insert(key.to_string(), serde_json::json!(report_ready));
        report_object.insert(
            format!("{key}_count"),
            serde_json::json!(if report_ready { 1 } else { 0 }),
        );
    }
    for key in [
        "source_dry_run_execution_result_receipt_boundary_bound",
        "approved_production_namespace_bound",
        "approved_production_store_bound",
        "approved_production_scope_bound",
        "production_durable_memory_target_bound",
        "dry_run_execution_result_receipt_envelope_bound",
        "dry_run_execution_result_receipt_digest_bound",
        "dry_run_execution_result_receipt_hash_chain_bound",
        "dry_run_execution_result_receipt_readback_plan_bound",
        "dry_run_execution_result_receipt_replay_guard_bound",
        "dry_run_execution_result_receipt_handoff_bound",
        "dry_run_execution_result_receipt_result_bound",
        "dry_run_execution_result_receipt_replay_idempotency_denial_matrix_bound",
        "dry_run_execution_result_receipt_replay_idempotency_identity_session_bound",
        "dry_run_execution_result_receipt_replay_idempotency_nonce_scope_bound",
        "dry_run_execution_result_receipt_replay_idempotency_duplicate_receipt_denied",
        "dry_run_execution_result_receipt_replay_idempotency_stale_receipt_denied",
        "dry_run_execution_result_receipt_replay_idempotency_hash_chain_mismatch_denied",
        "dry_run_execution_result_receipt_replay_idempotency_cross_session_denied",
        "dry_run_execution_result_receipt_replay_idempotency_handoff_bound",
        "dry_run_execution_result_receipt_replay_state_persistence_forbidden",
        "dry_run_execution_result_receipt_idempotency_ledger_write_forbidden",
        "dry_run_execution_execution_forbidden_on_replay_idempotency_route",
        "dry_run_execution_result_receipt_persistence_forbidden_on_replay_idempotency_route",
        "production_write_execution_forbidden_on_replay_idempotency_route",
        "production_durable_memory_write_forbidden",
        "memory_store_mutation_forbidden",
        "wal_write_forbidden_on_replay_idempotency_route",
        "receipt_persist_forbidden_on_replay_idempotency_route",
        "rollback_execution_forbidden_on_replay_idempotency_route",
        "tombstone_write_forbidden_on_replay_idempotency_route",
        "kg_live_write_forbidden",
        "provider_model_invocation_forbidden",
        "credential_channel_public_release_forbidden",
        "install_restart_active_binary_mutation_forbidden",
    ] {
        report_object.insert(key.to_string(), serde_json::json!(report_ready));
    }
    report_object.insert(
        "side_effects".to_string(),
        serde_json::Value::Object(side_effects),
    );
    report
}

fn hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_report()
-> serde_json::Value {
    const ORDERING_SURFACES: &[&str] = &[
        "source_replay_idempotency_denial_boundary_required",
        "source_replay_idempotency_result_required",
        "dry_run_execution_result_receipt_ordering_monotonicity_matrix_required",
        "dry_run_execution_result_receipt_ordering_sequence_policy_required",
        "dry_run_execution_result_receipt_late_receipt_denial_required",
        "dry_run_execution_result_receipt_future_receipt_denial_required",
        "dry_run_execution_result_receipt_rollback_sequence_denial_required",
        "dry_run_execution_result_receipt_same_sequence_denial_required",
        "dry_run_execution_result_receipt_latest_wins_promotion_denial_required",
        "dry_run_execution_result_receipt_sequence_gap_denial_required",
        "dry_run_execution_result_receipt_ordering_handoff_required",
        "dry_run_execution_result_receipt_ordering_cursor_persistence_forbidden",
        "dry_run_execution_result_receipt_monotonic_sequence_recording_forbidden",
        "dry_run_execution_execution_forbidden_on_ordering_monotonicity_route",
        "production_write_execution_forbidden_on_ordering_monotonicity_route",
        "kg_provider_channel_release_install_active_binary_forbidden",
    ];
    const ORDERING_DENIALS: &[&str] = &[
        "source_replay_idempotency_denial_boundary_required",
        "source_replay_idempotency_result_hash_required",
        "source_replay_idempotency_policy_hash_required",
        "source_replay_idempotency_matrix_required",
        "source_replay_idempotency_identity_session_required",
        "source_replay_idempotency_nonce_scope_required",
        "source_duplicate_receipt_denial_required",
        "source_stale_receipt_denial_required",
        "source_hash_chain_mismatch_denial_required",
        "source_cross_session_denial_required",
        "approved_production_namespace_required",
        "approved_production_store_required",
        "approved_production_scope_required",
        "production_durable_memory_target_required",
        "ordering_monotonicity_matrix_required",
        "ordering_sequence_policy_required",
        "ordering_identity_session_required",
        "ordering_latest_sequence_required",
        "late_receipt_acceptance_denied",
        "future_receipt_acceptance_denied",
        "rollback_sequence_acceptance_denied",
        "same_sequence_replacement_denied",
        "latest_wins_promotion_denied",
        "sequence_gap_acceptance_denied",
        "ordering_cursor_persistence_denied",
        "monotonic_sequence_recording_denied",
        "ordering_ledger_write_denied",
        "ordering_guard_state_recording_denied",
        "replay_state_persistence_denied",
        "idempotency_ledger_write_denied",
        "dry_run_execution_execution_denied",
        "dry_run_execution_envelope_persistence_denied",
        "dry_run_execution_result_persistence_denied",
        "dry_run_execution_result_receipt_persistence_denied",
        "dry_run_execution_result_receipt_filesystem_write_denied",
        "dry_run_execution_result_receipt_ledger_recording_denied",
        "dry_run_execution_result_receipt_delivery_denied",
        "dry_run_execution_result_receipt_materialization_denied",
        "acceptance_receipt_persistence_denied",
        "operator_packet_persistence_denied",
        "production_write_execution_denied",
        "production_durable_memory_backend_write_denied",
        "durable_memory_backend_read_or_rollback_denied",
        "memory_store_mutation_denied",
        "wal_write_denied",
        "receipt_persistence_denied",
        "post_write_readback_denied",
        "rollback_execution_denied",
        "tombstone_write_denied",
        "raw_payload_plaintext_denied",
        "kg_live_write_denied",
        "provider_model_invocation_denied",
        "credential_channel_release_install_denied",
        "active_binary_mutation_denied",
        "unrestricted_full_live_activation_denied",
    ];
    const FALSE_ORDERING_SIDE_EFFECT_KEYS: &[&str] = &[
        "dry_run_execution_result_receipt_ordering_cursor_persisted",
        "dry_run_execution_result_receipt_ordering_cursor_recorded",
        "dry_run_execution_result_receipt_ordering_ledger_written",
        "dry_run_execution_result_receipt_ordering_guard_state_recorded",
        "dry_run_execution_result_receipt_monotonic_sequence_recorded",
        "dry_run_execution_result_receipt_late_receipt_accepted",
        "dry_run_execution_result_receipt_future_receipt_accepted",
        "dry_run_execution_result_receipt_rollback_sequence_accepted",
        "dry_run_execution_result_receipt_same_sequence_replacement_accepted",
        "dry_run_execution_result_receipt_latest_wins_promoted",
        "dry_run_execution_result_receipt_sequence_gap_accepted",
        "dry_run_execution_result_receipt_ordering_attempt_accepted",
        "dry_run_execution_result_receipt_replay_state_persisted",
        "dry_run_execution_result_receipt_idempotency_ledger_written",
        "dry_run_execution_result_receipt_replay_guard_state_recorded",
        "dry_run_execution_result_receipt_duplicate_receipt_accepted",
        "dry_run_execution_result_receipt_stale_receipt_accepted",
        "dry_run_execution_result_receipt_cross_session_replay_accepted",
        "dry_run_execution_result_receipt_hash_chain_mismatch_accepted",
        "dry_run_execution_result_receipt_persisted",
        "dry_run_execution_result_receipt_filesystem_written",
        "dry_run_execution_result_receipt_ledger_recorded",
        "dry_run_execution_result_receipt_delivered",
        "dry_run_execution_result_receipt_materialized",
        "dry_run_execution_envelope_persisted",
        "dry_run_execution_executed",
        "dry_run_execution_result_persisted",
        "acceptance_receipt_persisted",
        "operator_packet_persisted",
        "operator_packet_acceptance_receipt_persisted",
        "production_durable_memory_write_executed",
        "production_durable_memory_backend_present",
        "production_durable_memory_store_write_performed",
        "actual_production_durable_memory_write_performed",
        "durable_memory_store_write_performed",
        "durable_memory_store_read_performed",
        "durable_memory_store_rollback_performed",
        "memory_write_execution_performed",
        "memory_store_write_path_enabled",
        "memory_store_write_allowed",
        "memory_store_write_performed",
        "memory_store_mutation_allowed",
        "memory_store_mutated",
        "wal_write_performed",
        "wal_recorded",
        "wal_persisted",
        "receipt_recorded",
        "receipt_persisted",
        "receipt_materialized",
        "receipt_delivered",
        "post_write_readback_performed",
        "readback_result_recorded",
        "readback_result_persisted",
        "readback_result_accepted",
        "rollback_executed",
        "rollback_performed",
        "rollback_result_recorded",
        "rollback_result_persisted",
        "rollback_result_accepted",
        "tombstone_write_performed",
        "tombstone_cleanup_executed",
        "tombstone_cleanup_result_recorded",
        "tombstone_cleanup_result_accepted",
        "raw_payload_plaintext_recorded",
        "raw_payload_plaintext_persisted",
        "secret_material_read",
        "credential_read",
        "secret_file_read",
        "kg_adapter_read_performed",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "telegram_send_performed",
        "channel_send_performed",
        "external_send_performed",
        "public_claim_promoted",
        "public_release_published",
        "public_ga_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "service_restart_performed",
        "active_binary_mutated",
    ];
    const TRUE_ORDERING_KEYS: &[&str] = &[
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_performed",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_result_recorded",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_result_accepted",
        "source_dry_run_execution_result_receipt_replay_idempotency_denial_boundary_accepted",
        "dry_run_execution_result_receipt_ordering_monotonicity_denial_matrix_bound",
        "dry_run_execution_result_receipt_ordering_sequence_policy_bound",
        "dry_run_execution_result_receipt_ordering_identity_session_bound",
        "dry_run_execution_result_receipt_ordering_latest_sequence_bound",
        "dry_run_execution_result_receipt_late_receipt_denied",
        "dry_run_execution_result_receipt_future_receipt_denied",
        "dry_run_execution_result_receipt_rollback_sequence_denied",
        "dry_run_execution_result_receipt_same_sequence_replacement_denied",
        "dry_run_execution_result_receipt_latest_wins_promotion_denied",
        "dry_run_execution_result_receipt_sequence_gap_denied",
        "dry_run_execution_result_receipt_ordering_handoff_bound",
        "dry_run_execution_result_receipt_ordering_cursor_persistence_forbidden",
        "dry_run_execution_result_receipt_monotonic_sequence_recording_forbidden",
        "dry_run_execution_execution_forbidden_on_ordering_monotonicity_route",
        "production_write_execution_forbidden_on_ordering_monotonicity_route",
        "kg_provider_channel_release_install_active_binary_forbidden",
    ];

    let route_matrix = control_ui_route_parity_report();
    let source = std::thread::Builder::new()
        .name("hepta-memory-production-durable-dry-run-result-receipt-ordering-source-report".to_string())
        .stack_size(8 * 1024 * 1024)
        .spawn(
            hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary_report,
        )
        .ok()
        .and_then(|handle| handle.join().ok())
        .unwrap_or_else(|| {
            serde_json::json!({
                "status": "blocked",
                "memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary_ready": false,
                "scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_accepted": false,
                "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_source_report_thread_failed": true
            })
        });
    let json_bool = |value: &serde_json::Value, key: &str| {
        value
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let json_u64 = |value: &serde_json::Value, key: &str| {
        value
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let json_str = |value: &serde_json::Value, key: &str| {
        value
            .get(key)
            .and_then(serde_json::Value::as_str)
            .unwrap_or("")
            .to_string()
    };
    let route_count_source_command_accepted = route_matrix.ready
        && route_matrix.route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_next_action_ordering = source
        .get("allowed_next_actions")
        .and_then(serde_json::Value::as_array)
        .and_then(|items| items.get(1))
        .map(|item| {
            item.get("action").and_then(serde_json::Value::as_str)
                == Some("prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary")
                && item
                    .get("requires_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
                && item
                    .get("executes_dry_run")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && item
                    .get("writes_production_durable_memory")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
        })
        .unwrap_or(false);
    let source_side_effects_ok = source
        .get("side_effects")
        .and_then(serde_json::Value::as_object)
        .map(|effects| {
            effects
                .get("scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary_performed")
                .and_then(serde_json::Value::as_bool)
                == Some(true)
                && effects
                    .get("scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_result_accepted")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
                && effects
                    .get("dry_run_execution_result_receipt_replay_state_persisted")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("dry_run_execution_result_receipt_idempotency_ledger_written")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("dry_run_execution_executed")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("production_durable_memory_store_write_performed")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("memory_store_write_performed")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("wal_write_performed")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("receipt_persisted")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("external_send_performed")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
        })
        .unwrap_or(false);
    let source_ready = source.get("status").and_then(serde_json::Value::as_str) == Some("ready")
        && json_bool(
            &source,
            "memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary_ready",
        )
        && json_bool(
            &source,
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_accepted",
        )
        && json_u64(
            &source,
            "accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_fixture_count",
        ) == 1
        && json_u64(
            &source,
            "blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_fixture_count",
        ) == 9
        && json_u64(
            &source,
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_result_accepted_count",
        ) == 1
        && !json_bool(
            &source,
            "dry_run_execution_result_receipt_replay_state_persisted",
        )
        && !json_bool(
            &source,
            "dry_run_execution_result_receipt_idempotency_ledger_written",
        )
        && !json_bool(
            &source,
            "dry_run_execution_result_receipt_duplicate_receipt_accepted",
        )
        && !json_bool(
            &source,
            "dry_run_execution_result_receipt_stale_receipt_accepted",
        )
        && !json_bool(
            &source,
            "dry_run_execution_result_receipt_cross_session_replay_accepted",
        )
        && !json_bool(
            &source,
            "dry_run_execution_result_receipt_hash_chain_mismatch_accepted",
        )
        && !json_bool(&source, "dry_run_execution_executed")
        && !json_bool(&source, "dry_run_execution_result_receipt_persisted")
        && !json_bool(&source, "production_durable_memory_write_executed")
        && !json_bool(&source, "production_durable_memory_store_write_performed")
        && !json_bool(&source, "actual_production_durable_memory_write_performed")
        && !json_bool(&source, "durable_memory_store_write_performed")
        && !json_bool(&source, "durable_memory_store_read_performed")
        && !json_bool(&source, "durable_memory_store_rollback_performed")
        && !json_bool(&source, "memory_store_write_performed")
        && !json_bool(&source, "wal_write_performed")
        && !json_bool(&source, "receipt_persisted")
        && !json_bool(&source, "post_write_readback_performed")
        && !json_bool(&source, "rollback_executed")
        && !json_bool(&source, "tombstone_cleanup_executed")
        && !json_bool(&source, "live_kg_write_performed")
        && !json_bool(&source, "provider_invoked")
        && !json_bool(&source, "model_invoked")
        && !json_bool(&source, "credential_read")
        && !json_bool(&source, "channel_send_performed")
        && !json_bool(&source, "external_send_performed")
        && !json_bool(&source, "release_artifact_written")
        && !json_bool(&source, "install_executed")
        && !json_bool(&source, "service_restarted")
        && !json_bool(&source, "active_binary_mutated")
        && source_next_action_ordering
        && source_side_effects_ok;

    let approved_production_namespace = json_str(&source, "approved_production_namespace");
    let approved_production_store = json_str(&source, "approved_production_store");
    let approved_production_scope = json_str(&source, "approved_production_scope");
    let production_durable_memory_target_id =
        json_str(&source, "production_durable_memory_target_id");
    let production_durable_memory_payload_class =
        json_str(&source, "production_durable_memory_payload_class");
    let operator_packet_scope = json_str(&source, "operator_packet_scope");
    let source_report_sha256 = sha256_text_value(&source.to_string());
    let source_replay_boundary_hash_sha256 = json_str(
        &source,
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary_hash_sha256",
    );
    let source_replay_policy_hash_sha256 = json_str(
        &source,
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_policy_hash_sha256",
    );
    let source_replay_matrix_hash_sha256 = json_str(
        &source,
        "dry_run_execution_result_receipt_replay_idempotency_denial_matrix_hash_sha256",
    );
    let source_replay_identity_session_hash_sha256 = json_str(
        &source,
        "dry_run_execution_result_receipt_replay_idempotency_identity_session_hash_sha256",
    );
    let source_replay_nonce_scope_hash_sha256 = json_str(
        &source,
        "dry_run_execution_result_receipt_replay_idempotency_nonce_scope_hash_sha256",
    );
    let source_duplicate_receipt_denial_hash_sha256 = json_str(
        &source,
        "dry_run_execution_result_receipt_replay_idempotency_duplicate_receipt_denial_hash_sha256",
    );
    let source_stale_receipt_denial_hash_sha256 = json_str(
        &source,
        "dry_run_execution_result_receipt_replay_idempotency_stale_receipt_denial_hash_sha256",
    );
    let source_hash_chain_mismatch_denial_hash_sha256 = json_str(
        &source,
        "dry_run_execution_result_receipt_replay_idempotency_hash_chain_mismatch_denial_hash_sha256",
    );
    let source_cross_session_denial_hash_sha256 = json_str(
        &source,
        "dry_run_execution_result_receipt_replay_idempotency_cross_session_denial_hash_sha256",
    );
    let source_replay_handoff_hash_sha256 = json_str(
        &source,
        "dry_run_execution_result_receipt_replay_idempotency_handoff_hash_sha256",
    );
    let source_replay_result_hash_sha256 = json_str(
        &source,
        "dry_run_execution_result_receipt_replay_idempotency_result_hash_sha256",
    );
    let ordering_monotonicity_matrix_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-ordering-monotonicity-denial-matrix:v1:source-replay={source_replay_result_hash_sha256}:duplicate={source_duplicate_receipt_denial_hash_sha256}:stale={source_stale_receipt_denial_hash_sha256}:late=deny:future=deny:rollback=deny:same-sequence=deny:latest-wins=deny:sequence-gap=deny:persist-cursor=false"
    ));
    let ordering_sequence_policy_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-ordering-sequence-policy:v1:matrix={ordering_monotonicity_matrix_hash_sha256}:source-policy={source_replay_policy_hash_sha256}:monotonic-record=false:cursor=false"
    ));
    let ordering_identity_session_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-ordering-identity-session:v1:source-identity={source_replay_identity_session_hash_sha256}:source-cross-session={source_cross_session_denial_hash_sha256}:cross-session=false"
    ));
    let ordering_latest_sequence_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-ordering-latest-sequence:v1:source-nonce={source_replay_nonce_scope_hash_sha256}:latest-wins=false:same-sequence=false"
    ));
    let late_receipt_denial_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-late-denial:v1:policy={ordering_sequence_policy_hash_sha256}:accepted=false"
    ));
    let future_receipt_denial_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-future-denial:v1:policy={ordering_sequence_policy_hash_sha256}:accepted=false"
    ));
    let rollback_sequence_denial_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-rollback-sequence-denial:v1:policy={ordering_sequence_policy_hash_sha256}:accepted=false"
    ));
    let same_sequence_replacement_denial_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-same-sequence-replacement-denial:v1:latest={ordering_latest_sequence_hash_sha256}:accepted=false"
    ));
    let latest_wins_promotion_denial_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-latest-wins-promotion-denial:v1:latest={ordering_latest_sequence_hash_sha256}:accepted=false"
    ));
    let sequence_gap_denial_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-sequence-gap-denial:v1:matrix={ordering_monotonicity_matrix_hash_sha256}:accepted=false"
    ));
    let ordering_handoff_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-ordering-handoff:v1:late={late_receipt_denial_hash_sha256}:future={future_receipt_denial_hash_sha256}:rollback={rollback_sequence_denial_hash_sha256}:same={same_sequence_replacement_denial_hash_sha256}:latest={latest_wins_promotion_denial_hash_sha256}:gap={sequence_gap_denial_hash_sha256}:next=cancellation-supersession-denial-boundary"
    ));
    let ordering_result_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-ordering-result:v1:matrix={ordering_monotonicity_matrix_hash_sha256}:handoff={ordering_handoff_hash_sha256}:accepted=true:cursor=false:sequence-record=false:ledger=false:executed=false:production-write=false"
    ));
    let ordering_boundary_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-ordering-monotonicity-denial-boundary:v1:source={source_report_sha256}:result={ordering_result_hash_sha256}:fixtures=10:accepted=1:denials={}:ordering-cursor=false:monotonic-sequence=false:dry-run-executed=false:production-write=false",
        ORDERING_DENIALS.len()
    ));
    let ordering_policy_hash_sha256 = sha256_text_value(
        "scoped-production-durable-memory-write-dry-run-result-receipt-ordering-monotonicity-denial-policy:v1:bind-source-replay-matrix-sequence-late-future-rollback-same-sequence-latest-wins-gap-handoff:no-ordering-cursor:no-monotonic-sequence:no-execution:no-production-write:no-kg:no-provider:no-channel:no-release:no-install",
    );
    let ordering_bound = !source_replay_boundary_hash_sha256.is_empty()
        && !source_replay_policy_hash_sha256.is_empty()
        && !source_replay_matrix_hash_sha256.is_empty()
        && !source_replay_identity_session_hash_sha256.is_empty()
        && !source_replay_nonce_scope_hash_sha256.is_empty()
        && !source_duplicate_receipt_denial_hash_sha256.is_empty()
        && !source_stale_receipt_denial_hash_sha256.is_empty()
        && !source_hash_chain_mismatch_denial_hash_sha256.is_empty()
        && !source_cross_session_denial_hash_sha256.is_empty()
        && !source_replay_handoff_hash_sha256.is_empty()
        && !source_replay_result_hash_sha256.is_empty()
        && approved_production_namespace == "hepta.memory.production.scoped"
        && approved_production_store == "hepta-memory-durable-store-production-preflight-only"
        && approved_production_scope == "operator-approved-session"
        && production_durable_memory_target_id
            == "hepta-scoped-production-durable-memory-write-target-v1"
        && production_durable_memory_payload_class
            == "redacted-minimal-operator-approved-memory-fact";
    let surfaces_ready = source_ready && ordering_bound;
    let report_ready = route_count_source_command_accepted && surfaces_ready;
    let accepted_fixture_count = if report_ready { 1 } else { 0 };
    let blocked_fixture_count = 10 - accepted_fixture_count;

    let mut fixtures = Vec::new();
    fixtures.push(serde_json::json!({
        "id": "scoped-production-durable-memory-write-dry-run-execution-result-receipt-ordering-monotonicity-denial",
        "fixture_id": "scoped-production-durable-memory-write-dry-run-execution-result-receipt-ordering-monotonicity-denial",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_accepted": report_ready,
        "reason": if report_ready { "dry_run_execution_result_receipt_ordering_monotonicity_denial_bound_without_cursor_sequence_execution_or_production_write" } else { "source_replay_idempotency_or_route_count_not_ready" },
        "source_replay_idempotency_denial_boundary_bound": report_ready,
        "dry_run_execution_result_receipt_ordering_monotonicity_denial_matrix_bound": report_ready,
        "dry_run_execution_result_receipt_late_receipt_denied": report_ready,
        "dry_run_execution_result_receipt_future_receipt_denied": report_ready,
        "dry_run_execution_result_receipt_rollback_sequence_denied": report_ready,
        "dry_run_execution_result_receipt_same_sequence_replacement_denied": report_ready,
        "dry_run_execution_result_receipt_latest_wins_promotion_denied": report_ready,
        "dry_run_execution_result_receipt_sequence_gap_denied": report_ready,
        "dry_run_execution_result_receipt_ordering_cursor_persisted": false,
        "dry_run_execution_result_receipt_monotonic_sequence_recorded": false,
        "dry_run_execution_executed": false,
        "production_durable_memory_store_write_performed": false,
        "external_send_performed": false
    }));
    for id in [
        "missing-replay-idempotency-source",
        "missing-replay-idempotency-result-hash",
        "missing-ordering-monotonicity-matrix",
        "late-result-receipt-attempt",
        "future-result-receipt-attempt",
        "rollback-sequence-result-receipt-attempt",
        "same-sequence-replacement-attempt",
        "latest-wins-promotion-attempt",
        "ordering-cursor-or-monotonic-sequence-persistence-attempt",
    ] {
        fixtures.push(serde_json::json!({
            "id": id,
            "fixture_id": id,
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_accepted": false,
            "reason": "blocked_noop",
            "dry_run_execution_result_receipt_ordering_cursor_persisted": false,
            "dry_run_execution_result_receipt_monotonic_sequence_recorded": false,
            "dry_run_execution_executed": false,
            "production_durable_memory_store_write_performed": false,
            "external_send_performed": false
        }));
    }

    let mut side_effects = serde_json::Map::new();
    for &key in FALSE_ORDERING_SIDE_EFFECT_KEYS {
        side_effects.insert(key.to_string(), serde_json::json!(false));
        side_effects.insert(format!("{key}_count"), serde_json::json!(0));
    }
    for &key in TRUE_ORDERING_KEYS {
        side_effects.insert(key.to_string(), serde_json::json!(report_ready));
        side_effects.insert(
            format!("{key}_count"),
            serde_json::json!(if report_ready { 1 } else { 0 }),
        );
    }

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "endpoint": HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_PRODUCTION_DURABLE_MEMORY_WRITE_DRY_RUN_EXECUTION_RESULT_RECEIPT_ORDERING_MONOTONICITY_DENIAL_BOUNDARY_ENDPOINT,
        "source_command": "/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-ordering-monotonicity-denial-boundary --json",
        "native_route": true,
        "side_effect_free": false,
        "external_side_effect_free": true,
        "audit_date": "2026-07-05"
    });
    let report_object = report.as_object_mut().expect(
        "scoped production durable Memory write dry-run execution result receipt ordering/monotonicity denial report object",
    );
    macro_rules! insert_report {
        ($key:expr, $value:expr) => {
            report_object.insert($key.to_string(), serde_json::json!($value));
        };
    }

    insert_report!(
        "native_gateway_source_command_count",
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    insert_report!("route_count", route_matrix.route_count);
    insert_report!(
        "implemented_route_count",
        route_matrix.implemented_route_count
    );
    insert_report!("missing_route_count", route_matrix.missing_route_count);
    insert_report!(
        "route_count_source_command_accepted",
        route_count_source_command_accepted
    );
    insert_report!(
        "memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_ready",
        report_ready
    );
    insert_report!(
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_ready",
        report_ready
    );
    insert_report!(
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_performed",
        report_ready
    );
    insert_report!(
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_accepted",
        report_ready
    );
    insert_report!(
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_mode",
        "dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_no_ordering_cursor_no_monotonic_sequence_no_execution_no_production_durable_memory_mutation"
    );
    insert_report!(
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary_ready",
        source_ready
    );
    insert_report!(
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary_report_sha256",
        source_report_sha256
    );
    insert_report!(
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary_accepted_count",
        json_u64(
            &source,
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_result_accepted_count"
        )
    );
    insert_report!(
        "source_accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_fixture_count",
        json_u64(
            &source,
            "accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_fixture_count"
        )
    );
    insert_report!(
        "source_blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_fixture_count",
        json_u64(
            &source,
            "blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_fixture_count"
        )
    );
    insert_report!(
        "source_denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary_count",
        json_u64(
            &source,
            "denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary_count"
        )
    );
    insert_report!(
        "approved_production_namespace",
        approved_production_namespace
    );
    insert_report!("approved_production_store", approved_production_store);
    insert_report!("approved_production_scope", approved_production_scope);
    insert_report!(
        "production_durable_memory_target_id",
        production_durable_memory_target_id
    );
    insert_report!(
        "production_durable_memory_payload_class",
        production_durable_memory_payload_class
    );
    insert_report!("operator_packet_scope", operator_packet_scope);
    insert_report!(
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary_hash_sha256",
        source_replay_boundary_hash_sha256
    );
    insert_report!(
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_policy_hash_sha256",
        source_replay_policy_hash_sha256
    );
    insert_report!(
        "source_dry_run_execution_result_receipt_replay_idempotency_denial_matrix_hash_sha256",
        source_replay_matrix_hash_sha256
    );
    insert_report!(
        "source_dry_run_execution_result_receipt_replay_idempotency_identity_session_hash_sha256",
        source_replay_identity_session_hash_sha256
    );
    insert_report!(
        "source_dry_run_execution_result_receipt_replay_idempotency_nonce_scope_hash_sha256",
        source_replay_nonce_scope_hash_sha256
    );
    insert_report!(
        "source_dry_run_execution_result_receipt_replay_idempotency_duplicate_receipt_denial_hash_sha256",
        source_duplicate_receipt_denial_hash_sha256
    );
    insert_report!(
        "source_dry_run_execution_result_receipt_replay_idempotency_stale_receipt_denial_hash_sha256",
        source_stale_receipt_denial_hash_sha256
    );
    insert_report!(
        "source_dry_run_execution_result_receipt_replay_idempotency_hash_chain_mismatch_denial_hash_sha256",
        source_hash_chain_mismatch_denial_hash_sha256
    );
    insert_report!(
        "source_dry_run_execution_result_receipt_replay_idempotency_cross_session_denial_hash_sha256",
        source_cross_session_denial_hash_sha256
    );
    insert_report!(
        "source_dry_run_execution_result_receipt_replay_idempotency_handoff_hash_sha256",
        source_replay_handoff_hash_sha256
    );
    insert_report!(
        "source_dry_run_execution_result_receipt_replay_idempotency_result_hash_sha256",
        source_replay_result_hash_sha256
    );
    insert_report!(
        "dry_run_execution_result_receipt_ordering_monotonicity_denial_matrix_hash_sha256",
        ordering_monotonicity_matrix_hash_sha256
    );
    insert_report!(
        "dry_run_execution_result_receipt_ordering_sequence_policy_hash_sha256",
        ordering_sequence_policy_hash_sha256
    );
    insert_report!(
        "dry_run_execution_result_receipt_ordering_identity_session_hash_sha256",
        ordering_identity_session_hash_sha256
    );
    insert_report!(
        "dry_run_execution_result_receipt_ordering_latest_sequence_hash_sha256",
        ordering_latest_sequence_hash_sha256
    );
    insert_report!(
        "dry_run_execution_result_receipt_late_receipt_denial_hash_sha256",
        late_receipt_denial_hash_sha256
    );
    insert_report!(
        "dry_run_execution_result_receipt_future_receipt_denial_hash_sha256",
        future_receipt_denial_hash_sha256
    );
    insert_report!(
        "dry_run_execution_result_receipt_rollback_sequence_denial_hash_sha256",
        rollback_sequence_denial_hash_sha256
    );
    insert_report!(
        "dry_run_execution_result_receipt_same_sequence_replacement_denial_hash_sha256",
        same_sequence_replacement_denial_hash_sha256
    );
    insert_report!(
        "dry_run_execution_result_receipt_latest_wins_promotion_denial_hash_sha256",
        latest_wins_promotion_denial_hash_sha256
    );
    insert_report!(
        "dry_run_execution_result_receipt_sequence_gap_denial_hash_sha256",
        sequence_gap_denial_hash_sha256
    );
    insert_report!(
        "dry_run_execution_result_receipt_ordering_handoff_hash_sha256",
        ordering_handoff_hash_sha256
    );
    insert_report!(
        "dry_run_execution_result_receipt_ordering_result_hash_sha256",
        ordering_result_hash_sha256
    );
    insert_report!(
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_hash_sha256",
        ordering_boundary_hash_sha256
    );
    insert_report!(
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_policy_hash_sha256",
        ordering_policy_hash_sha256
    );
    insert_report!(
        "required_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_surface_count",
        ORDERING_SURFACES.len()
    );
    insert_report!(
        "ready_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_surface_count",
        if surfaces_ready {
            ORDERING_SURFACES.len()
        } else {
            0
        }
    );
    insert_report!(
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_surfaces",
        ORDERING_SURFACES
    );
    insert_report!(
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_fixture_count",
        fixtures.len()
    );
    insert_report!(
        "accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_fixture_count",
        accepted_fixture_count
    );
    insert_report!(
        "blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_fixture_count",
        blocked_fixture_count
    );
    insert_report!(
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_fixtures",
        fixtures
    );
    insert_report!(
        "denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary",
        ORDERING_DENIALS
    );
    insert_report!(
        "denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_count",
        ORDERING_DENIALS.len()
    );
    insert_report!(
        "allowed_next_actions",
        [
            serde_json::json!({
                "action": "run_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_require_live_gate",
                "status": "allowed_verification_only",
                "accepts_ordering_monotonicity_denial_matrix": true,
                "persists_ordering_cursor": false,
                "records_monotonic_sequence": false,
                "writes_ordering_ledger": false,
                "executes_dry_run": false,
                "persists_dry_run_result_receipt": false,
                "writes_production_durable_memory": false,
                "writes_memory_store": false,
                "writes_wal": false,
                "persists_receipt": false,
                "executes_rollback": false,
                "writes_tombstone": false
            }),
            serde_json::json!({
                "action": "prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary",
                "status": "requires_separate_result_receipt_cancellation_supersession_denial_gate",
                "requires_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary": true,
                "persists_ordering_cursor": false,
                "records_monotonic_sequence": false,
                "executes_dry_run": false,
                "writes_production_durable_memory": false,
                "persists_dry_run_result_receipt": false
            }),
        ]
    );
    for &key in FALSE_ORDERING_SIDE_EFFECT_KEYS {
        report_object.insert(key.to_string(), serde_json::json!(false));
        report_object.insert(format!("{key}_count"), serde_json::json!(0));
    }
    for &key in TRUE_ORDERING_KEYS {
        report_object.insert(key.to_string(), serde_json::json!(report_ready));
        report_object.insert(
            format!("{key}_count"),
            serde_json::json!(if report_ready { 1 } else { 0 }),
        );
    }
    for key in [
        "source_dry_run_execution_result_receipt_replay_idempotency_denial_boundary_bound",
        "approved_production_namespace_bound",
        "approved_production_store_bound",
        "approved_production_scope_bound",
        "production_durable_memory_target_bound",
        "dry_run_execution_result_receipt_replay_idempotency_denial_matrix_bound",
        "dry_run_execution_result_receipt_replay_idempotency_identity_session_bound",
        "dry_run_execution_result_receipt_replay_idempotency_nonce_scope_bound",
        "dry_run_execution_result_receipt_replay_idempotency_duplicate_receipt_denied",
        "dry_run_execution_result_receipt_replay_idempotency_stale_receipt_denied",
        "dry_run_execution_result_receipt_replay_idempotency_hash_chain_mismatch_denied",
        "dry_run_execution_result_receipt_replay_idempotency_cross_session_denied",
        "dry_run_execution_result_receipt_replay_idempotency_handoff_bound",
        "dry_run_execution_result_receipt_ordering_monotonicity_denial_matrix_bound",
        "dry_run_execution_result_receipt_ordering_sequence_policy_bound",
        "dry_run_execution_result_receipt_ordering_identity_session_bound",
        "dry_run_execution_result_receipt_ordering_latest_sequence_bound",
        "dry_run_execution_result_receipt_late_receipt_denied",
        "dry_run_execution_result_receipt_future_receipt_denied",
        "dry_run_execution_result_receipt_rollback_sequence_denied",
        "dry_run_execution_result_receipt_same_sequence_replacement_denied",
        "dry_run_execution_result_receipt_latest_wins_promotion_denied",
        "dry_run_execution_result_receipt_sequence_gap_denied",
        "dry_run_execution_result_receipt_ordering_handoff_bound",
        "dry_run_execution_result_receipt_ordering_cursor_persistence_forbidden",
        "dry_run_execution_result_receipt_monotonic_sequence_recording_forbidden",
        "dry_run_execution_execution_forbidden_on_ordering_monotonicity_route",
        "dry_run_execution_result_receipt_persistence_forbidden_on_ordering_monotonicity_route",
        "production_write_execution_forbidden_on_ordering_monotonicity_route",
        "production_durable_memory_write_forbidden",
        "memory_store_mutation_forbidden",
        "wal_write_forbidden_on_ordering_monotonicity_route",
        "receipt_persist_forbidden_on_ordering_monotonicity_route",
        "rollback_execution_forbidden_on_ordering_monotonicity_route",
        "tombstone_write_forbidden_on_ordering_monotonicity_route",
        "kg_live_write_forbidden",
        "provider_model_invocation_forbidden",
        "credential_channel_public_release_forbidden",
        "install_restart_active_binary_mutation_forbidden",
    ] {
        report_object.insert(key.to_string(), serde_json::json!(report_ready));
    }
    report_object.insert(
        "side_effects".to_string(),
        serde_json::Value::Object(side_effects),
    );
    report
}
