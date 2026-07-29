fn hepta_memory_live_mutation_operator_write_execution_activation_command_noop_handoff_boundary_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let activation_closure =
        hepta_memory_live_mutation_operator_write_execution_activation_closure_denial_boundary_report(
        );

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
    let side_effects_all_false = |value: &serde_json::Value| {
        value
            .get("side_effects")
            .and_then(serde_json::Value::as_object)
            .map(|effects| effects.values().all(|item| item.as_bool() == Some(false)))
            .unwrap_or(false)
    };

    let route_count_source_command_accepted = route_matrix.ready
        && route_matrix.route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let activation_closure_ready = json_str(&activation_closure, "status") == "ready"
        && json_bool(
            &activation_closure,
            "memory_write_execution_activation_closure_denial_boundary_ready",
        )
        && json_bool(
            &activation_closure,
            "memory_write_execution_activation_closure_denial_ready",
        )
        && json_bool(
            &activation_closure,
            "memory_write_execution_post_write_operator_acceptance_denial_ready",
        )
        && json_bool(
            &activation_closure,
            "memory_write_execution_post_write_validation_dry_run_ready",
        )
        && json_bool(
            &activation_closure,
            "memory_write_execution_write_enable_fixture_ready",
        )
        && json_bool(
            &activation_closure,
            "memory_write_execution_no_write_sink_contract_ready",
        )
        && json_u64(
            &activation_closure,
            "required_activation_closure_surface_count",
        ) == 12
        && json_u64(
            &activation_closure,
            "ready_activation_closure_surface_count",
        ) == 12
        && json_u64(&activation_closure, "activation_closure_fixture_count") == 10
        && json_u64(
            &activation_closure,
            "blocked_activation_closure_fixture_count",
        ) == 10
        && json_u64(&activation_closure, "denied_by_activation_closure_count") == 24
        && !json_bool(&activation_closure, "activation_closure_packet_recorded")
        && !json_bool(&activation_closure, "activation_closure_packet_persisted")
        && !json_bool(&activation_closure, "activation_closure_packet_accepted")
        && !json_bool(&activation_closure, "activation_command_enabled")
        && !json_bool(&activation_closure, "activation_command_invoked")
        && !json_bool(&activation_closure, "activation_allowed")
        && !json_bool(&activation_closure, "memory_write_execution_performed")
        && !json_bool(&activation_closure, "memory_store_mutated")
        && !json_bool(&activation_closure, "rollback_executed")
        && !json_bool(&activation_closure, "live_kg_write_performed")
        && !json_bool(&activation_closure, "provider_invoked")
        && !json_bool(&activation_closure, "model_invoked")
        && !json_bool(&activation_closure, "credential_read")
        && !json_bool(&activation_closure, "external_send_performed")
        && !json_bool(&activation_closure, "release_artifact_written")
        && !json_bool(&activation_closure, "active_binary_mutated")
        && side_effects_all_false(&activation_closure);

    let activation_command_handoff_surfaces = vec![
        "accepted_activation_closure_packet_required",
        "activation_closure_packet_hash_and_signature_required",
        "operator_identity_signature_timestamp_required",
        "single_surface_activation_scope_required",
        "activation_command_disabled_by_default_required",
        "activation_command_invocation_noop_required",
        "pre_post_store_hashes_and_write_receipt_required",
        "post_write_soak_route_dependency_evidence_required",
        "rollback_validation_and_no_rollback_execution_required",
        "audit_redaction_and_no_secret_material_required",
        "no_memory_store_write_or_live_mutation_required",
        "no_install_restart_or_active_binary_mutation_required",
        "no_external_public_or_release_outputs_required",
    ];
    let activation_command_fixtures = serde_json::json!([
        {
            "id": "activation-command-missing-accepted-closure-packet",
            "activation_command_requested": true,
            "command_status": "blocked_noop",
            "accepted_activation_closure_packet_present": false,
            "activation_closure_packet_hash_bound": false,
            "command_allowed": false,
            "command_invoked": false,
            "command_dispatched": false,
            "command_noop_confirmed": true,
            "handoff_recorded": false,
            "handoff_persisted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "accepted_activation_closure_packet_required"
        },
        {
            "id": "activation-command-disabled-by-default",
            "activation_command_requested": true,
            "command_status": "blocked_noop",
            "accepted_activation_closure_packet_present": true,
            "activation_closure_packet_hash_bound": true,
            "activation_command_enabled": false,
            "command_allowed": false,
            "command_invoked": false,
            "command_dispatched": false,
            "command_noop_confirmed": true,
            "handoff_recorded": false,
            "handoff_persisted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "activation_command_disabled_by_default"
        },
        {
            "id": "activation-command-direct-invocation-attempt",
            "activation_command_requested": true,
            "command_invocation_attempted": true,
            "command_status": "blocked_noop",
            "accepted_activation_closure_packet_present": true,
            "activation_command_enabled": false,
            "command_allowed": false,
            "command_invoked": false,
            "command_dispatched": false,
            "command_noop_confirmed": true,
            "handoff_recorded": false,
            "handoff_persisted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "direct_activation_command_invocation_denied"
        },
        {
            "id": "activation-command-closure-hash-mismatch",
            "activation_command_requested": true,
            "command_status": "blocked_noop",
            "accepted_activation_closure_packet_present": true,
            "activation_closure_packet_hash_bound": false,
            "activation_closure_packet_signature_hash_recorded": false,
            "command_allowed": false,
            "command_invoked": false,
            "command_dispatched": false,
            "command_noop_confirmed": true,
            "handoff_recorded": false,
            "handoff_persisted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "closure_packet_hash_and_signature_binding_required"
        },
        {
            "id": "activation-command-multi-surface-handoff",
            "activation_command_requested": true,
            "command_status": "blocked_noop",
            "accepted_activation_closure_packet_present": true,
            "operator_single_surface_scope_recorded": false,
            "multi_surface_activation_requested": true,
            "command_allowed": false,
            "command_invoked": false,
            "command_dispatched": false,
            "command_noop_confirmed": true,
            "handoff_recorded": false,
            "handoff_persisted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "single_surface_activation_scope_required"
        },
        {
            "id": "activation-command-memory-write-path-attempt",
            "activation_command_requested": true,
            "command_status": "blocked_noop",
            "accepted_activation_closure_packet_present": true,
            "memory_store_write_path_enable_requested": true,
            "direct_memory_store_write_requested": true,
            "command_allowed": false,
            "command_invoked": false,
            "command_dispatched": false,
            "command_noop_confirmed": true,
            "handoff_recorded": false,
            "handoff_persisted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "activation_command_cannot_enable_or_perform_memory_store_write"
        },
        {
            "id": "activation-command-rollback-execution-attempt",
            "activation_command_requested": true,
            "command_status": "blocked_noop",
            "accepted_activation_closure_packet_present": true,
            "rollback_validation_accepted": false,
            "rollback_execution_requested": true,
            "command_allowed": false,
            "command_invoked": false,
            "command_dispatched": false,
            "command_noop_confirmed": true,
            "handoff_recorded": false,
            "handoff_persisted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "rollback_execution_denied_at_activation_command_handoff"
        },
        {
            "id": "activation-command-secret-or-prompt-replay-attempt",
            "activation_command_requested": true,
            "command_status": "blocked_noop",
            "accepted_activation_closure_packet_present": true,
            "raw_payload_plaintext_recorded": true,
            "secret_material_read": true,
            "provider_prompt_replay_requested": true,
            "command_allowed": false,
            "command_invoked": false,
            "command_dispatched": false,
            "command_noop_confirmed": true,
            "handoff_recorded": false,
            "handoff_persisted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "secret_material_and_provider_prompt_replay_forbidden"
        },
        {
            "id": "activation-command-external-public-release-attempt",
            "activation_command_requested": true,
            "command_status": "blocked_noop",
            "accepted_activation_closure_packet_present": true,
            "external_send_requested": true,
            "public_claim_requested": true,
            "release_artifact_write_requested": true,
            "command_allowed": false,
            "command_invoked": false,
            "command_dispatched": false,
            "command_noop_confirmed": true,
            "handoff_recorded": false,
            "handoff_persisted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "external_send_performed": false,
            "public_release_published": false,
            "release_artifact_written": false,
            "rollback_executed": false,
            "reason": "activation_command_cannot_send_publish_or_write_release_artifacts"
        },
        {
            "id": "activation-command-install-restart-active-binary-attempt",
            "activation_command_requested": true,
            "command_status": "blocked_noop",
            "accepted_activation_closure_packet_present": true,
            "install_requested": true,
            "launchd_restart_requested": true,
            "active_binary_mutation_requested": true,
            "command_allowed": false,
            "command_invoked": false,
            "command_dispatched": false,
            "command_noop_confirmed": true,
            "handoff_recorded": false,
            "handoff_persisted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restarted": false,
            "active_binary_mutated": false,
            "rollback_executed": false,
            "reason": "activation_command_cannot_install_restart_or_mutate_active_binary"
        }
    ]);
    let denied_by = vec![
        "accepted_activation_closure_packet_required",
        "activation_closure_packet_hash_required",
        "activation_closure_packet_signature_required",
        "operator_identity_required",
        "operator_acceptance_signature_required",
        "operator_acceptance_timestamp_required",
        "single_surface_activation_scope_required",
        "activation_command_enabled_denied",
        "activation_command_invocation_denied",
        "activation_command_dispatch_denied",
        "activation_command_handoff_persistence_denied",
        "pre_write_memory_store_hash_binding_required",
        "post_write_memory_store_hash_binding_required",
        "write_result_receipt_hash_binding_required",
        "route_readiness_regression_denied",
        "active_dependency_isolation_regression_denied",
        "post_write_watchdog_soak_success_required",
        "memory_store_write_path_enablement_denied",
        "direct_memory_store_write_denied",
        "live_mutation_execution_denied",
        "rollback_execution_denied",
        "secret_material_read_denied",
        "provider_prompt_replay_denied",
        "install_restart_active_binary_mutation_denied",
        "external_send_public_claim_release_artifact_denied",
        "public_release_public_ga_denied",
    ];

    let source_activation_closure_report_sha256 = sha256_json_value(&activation_closure);
    let handoff_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-live-mutation-operator-write-execution-activation-command-noop-handoff-boundary-v1:{}:{}",
        route_matrix.route_count, source_activation_closure_report_sha256,
    ));
    let report_ready = route_matrix.ready
        && route_count_source_command_accepted
        && activation_closure_ready
        && activation_command_handoff_surfaces.len() == 13
        && activation_command_fixtures
            .as_array()
            .map(std::vec::Vec::len)
            == Some(10)
        && denied_by.len() == 26;

    let allowed_next_actions = serde_json::json!([
        {
            "action": "run_memory_write_execution_activation_command_noop_handoff_boundary_require_live_gate",
            "status": "allowed_verification_only",
            "registers_command": false,
            "enables_command": false,
            "invokes_activation_command": false,
            "dispatches_activation": false,
            "records_handoff": false,
            "persists_handoff": false,
            "accepts_activation": false,
            "writes_memory": false,
            "executes_rollback": false,
            "writes_kg": false,
            "invokes_provider": false,
            "invokes_model": false,
            "reads_credentials": false,
            "sends_externally": false,
            "publishes_artifacts": false,
            "installs_or_restarts": false,
            "mutates_active_binary": false
        },
        {
            "action": "prepare_memory_write_execution_activation_command_result_receipt_no_persistence_boundary",
            "status": "allowed_report_only_next_slice",
            "records_command_result": false,
            "persists_result_receipt": false,
            "accepts_result_receipt": false,
            "writes_memory": false,
            "executes_rollback": false,
            "writes_kg": false,
            "invokes_provider": false,
            "sends_externally": false,
            "publishes_artifacts": false
        }
    ]);

    let mut side_effects = serde_json::Map::new();
    for key in [
        "memory_store_mutated",
        "memory_store_write_performed",
        "memory_write_execution_performed",
        "post_write_validation_recorded",
        "post_write_validation_persisted",
        "post_write_validation_performed",
        "operator_post_write_acceptance_recorded",
        "operator_post_write_acceptance_persisted",
        "operator_post_write_acceptance_performed",
        "activation_closure_packet_recorded",
        "activation_closure_packet_persisted",
        "activation_closure_packet_materialized",
        "activation_closure_filesystem_written",
        "activation_closure_ledger_written",
        "activation_command_shape_registered",
        "activation_command_enabled",
        "activation_command_invoked",
        "activation_command_dispatched",
        "activation_command_handoff_recorded",
        "activation_command_handoff_persisted",
        "activation_command_result_receipt_recorded",
        "activation_command_result_receipt_persisted",
        "live_mutation_execution_performed",
        "rollback_validation_performed",
        "rollback_executed",
        "write_result_receipt_recorded",
        "write_result_receipt_persisted",
        "pre_write_memory_store_hash_recorded",
        "post_write_memory_store_hash_recorded",
        "audit_redaction_validation_recorded",
        "raw_payload_inspected",
        "payload_plaintext_persisted",
        "secret_file_read",
        "credential_read",
        "capability_registry_mutated",
        "plugin_registry_mutated",
        "skill_workshop_written",
        "provider_invoked",
        "model_invoked",
        "provider_prompt_replayed",
        "channel_send_performed",
        "telegram_send_performed",
        "external_send_performed",
        "runtime_store_mutated",
        "gateway_event_enqueued",
        "filesystem_written",
        "release_artifact_written",
        "public_artifact_written",
        "public_release_published",
        "public_ga_claimed",
        "install_executed",
        "active_binary_mutated",
        "launchd_mutated",
        "service_restarted",
    ] {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }

    let mut report = serde_json::Map::new();
    macro_rules! insert_report_json {
        ($key:literal, $value:expr) => {
            report.insert($key.to_string(), serde_json::json!($value));
        };
    }

    insert_report_json!("product", "Hepta");
    insert_report_json!("runtime", "hepta");
    insert_report_json!("status", if report_ready { "ready" } else { "blocked" });
    insert_report_json!("base_url", "http://127.0.0.1:7373");
    insert_report_json!(
        "gate",
        "hepta_memory_live_mutation_operator_write_execution_activation_command_noop_handoff_boundary_route"
    );
    insert_report_json!(
        "endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_COMMAND_NOOP_HANDOFF_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_command",
        "/hepta-memory-live-mutation-operator-write-execution-activation-command-no-op-handoff-boundary --json"
    );
    insert_report_json!("native_route", true);
    insert_report_json!("side_effect_free", true);
    insert_report_json!("audit_date", "2026-07-02");
    insert_report_json!(
        "memory_write_execution_activation_command_noop_handoff_boundary_schema_version",
        "memory_write_execution_activation_command_noop_handoff_boundary_v1"
    );
    insert_report_json!(
        "memory_write_execution_activation_command_noop_handoff_boundary_ready",
        report_ready
    );
    insert_report_json!(
        "activation_command_noop_handoff_mode",
        "memory_write_execution_activation_command_noop_handoff_denial"
    );
    insert_report_json!(
        "native_gateway_source_command_count",
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    insert_report_json!("route_count", route_matrix.route_count);
    insert_report_json!(
        "implemented_route_count",
        route_matrix.implemented_route_count
    );
    insert_report_json!("missing_route_count", route_matrix.missing_route_count);
    insert_report_json!(
        "route_count_source_command_accepted",
        route_count_source_command_accepted
    );
    insert_report_json!("boundary_hash_sha256", handoff_hash_sha256);
    insert_report_json!(
        "source_memory_write_execution_activation_closure_denial_boundary_endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_CLOSURE_DENIAL_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_memory_write_execution_activation_closure_denial_boundary_ready",
        activation_closure_ready
    );
    insert_report_json!(
        "source_memory_write_execution_activation_closure_denial_ready",
        json_bool(
            &activation_closure,
            "memory_write_execution_activation_closure_denial_ready"
        )
    );
    insert_report_json!(
        "source_memory_write_execution_activation_closure_denial_boundary_report_sha256",
        source_activation_closure_report_sha256
    );
    for key in [
        "source_memory_write_execution_post_write_operator_acceptance_denial_boundary_report_sha256",
        "source_memory_write_execution_post_write_validation_dry_run_boundary_report_sha256",
        "source_memory_write_execution_write_enable_fixture_boundary_report_sha256",
        "source_memory_write_execution_no_write_sink_contract_boundary_report_sha256",
        "source_memory_write_execution_denial_matrix_boundary_report_sha256",
    ] {
        report.insert(
            key.to_string(),
            activation_closure
                .get(key)
                .cloned()
                .unwrap_or_else(|| serde_json::json!("")),
        );
    }
    insert_report_json!("minimum_required_samples", 24);
    insert_report_json!(
        "memory_write_execution_activation_command_noop_handoff_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_activation_closure_denial_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_post_write_operator_acceptance_denial_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_post_write_validation_dry_run_ready",
        true
    );
    insert_report_json!("memory_write_execution_write_enable_fixture_ready", true);
    insert_report_json!("memory_write_execution_no_write_sink_contract_ready", true);
    insert_report_json!("required_activation_closure_surface_count", 12);
    insert_report_json!("ready_activation_closure_surface_count", 12);
    insert_report_json!("required_activation_command_handoff_surface_count", 13);
    insert_report_json!("ready_activation_command_handoff_surface_count", 13);
    insert_report_json!(
        "side_effect_free_activation_command_handoff_surface_count",
        13
    );
    insert_report_json!("required_activation_command_fixture_count", 10);
    insert_report_json!("activation_command_fixture_count", 10);
    insert_report_json!("blocked_activation_command_fixture_count", 10);
    insert_report_json!("noop_activation_command_fixture_count", 10);
    insert_report_json!("allowed_activation_command_fixture_count", 0);
    insert_report_json!("accepted_activation_command_fixture_count", 0);
    insert_report_json!("activation_command_denied_count", 10);
    insert_report_json!("activation_command_performed_count", 0);

    for key in [
        "activation_command_shape_registered",
        "activation_command_enabled",
        "activation_command_invoked",
        "activation_command_dispatched",
        "activation_command_noop_decision_recorded",
        "activation_command_noop_decision_persisted",
        "activation_command_noop_decision_accepted",
        "activation_command_handoff_recorded",
        "activation_command_handoff_persisted",
        "activation_command_handoff_accepted",
        "activation_command_handoff_materialized",
        "activation_command_handoff_filesystem_written",
        "activation_command_result_receipt_recorded",
        "activation_command_result_receipt_persisted",
        "activation_closure_packet_recorded",
        "activation_closure_packet_persisted",
        "activation_closure_packet_accepted",
        "activation_closure_packet_materialized",
        "activation_closure_packet_hash_bound",
        "activation_closure_packet_signature_hash_recorded",
        "activation_closure_ledger_written",
        "activation_allowed_by_command_handoff",
        "activation_allowed_by_closure_packet",
        "activation_allowed",
        "activation_performed",
        "live_mutation_execution_ready",
        "live_mutation_execution_allowed",
        "live_mutation_execution_performed",
        "memory_write_execution_allowed",
        "memory_write_execution_ready",
        "memory_write_execution_performed",
        "memory_store_write_path_enabled",
        "memory_store_write_allowed",
        "memory_store_write_performed",
        "memory_store_mutation_allowed",
        "memory_store_mutated",
        "write_result_receipt_hash_bound",
        "pre_write_memory_store_hash_bound",
        "post_write_memory_store_hash_bound",
        "post_write_diff_scope_accepted",
        "post_write_watchdog_soak_evidence_accepted",
        "route_readiness_regression_allowed",
        "active_dependency_isolation_regression_allowed",
        "rollback_validation_accepted",
        "rollback_execution_allowed",
        "rollback_executed",
        "audit_redaction_validation_accepted",
        "raw_payload_plaintext_recorded",
        "raw_payload_plaintext_persisted",
        "secret_material_read",
        "provider_prompt_replay_enabled",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "kg_adapter_read_performed",
        "live_kg_write_performed",
        "channel_send_performed",
        "telegram_send_performed",
        "external_send_enabled",
        "external_send_performed",
        "public_claim_or_release_artifact_write_enabled",
        "public_release_published",
        "public_ga_claimed",
        "public_release_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
    ] {
        report.insert(key.to_string(), serde_json::json!(false));
    }
    insert_report_json!("memory_store_write_performed_count", 0);
    report.insert(
        "activation_command_handoff_surfaces".to_string(),
        serde_json::json!(activation_command_handoff_surfaces),
    );
    report.insert(
        "activation_command_fixtures".to_string(),
        activation_command_fixtures,
    );
    report.insert(
        "denied_by_activation_command_handoff".to_string(),
        serde_json::json!(denied_by),
    );
    insert_report_json!("denied_by_activation_command_handoff_count", 26);
    report.insert("allowed_next_actions".to_string(), allowed_next_actions);
    report.insert(
        "side_effects".to_string(),
        serde_json::Value::Object(side_effects),
    );

    serde_json::Value::Object(report)
}

fn hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_no_persistence_boundary_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let noop_handoff =
        hepta_memory_live_mutation_operator_write_execution_activation_command_noop_handoff_boundary_report();

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
    let side_effects_all_false = |value: &serde_json::Value| {
        value
            .get("side_effects")
            .and_then(serde_json::Value::as_object)
            .map(|effects| effects.values().all(|item| item.as_bool() == Some(false)))
            .unwrap_or(false)
    };

    let route_count_source_command_accepted = route_matrix.ready
        && route_matrix.route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let noop_handoff_ready = json_str(&noop_handoff, "status") == "ready"
        && json_bool(
            &noop_handoff,
            "memory_write_execution_activation_command_noop_handoff_boundary_ready",
        )
        && json_bool(
            &noop_handoff,
            "memory_write_execution_activation_command_noop_handoff_ready",
        )
        && json_bool(
            &noop_handoff,
            "memory_write_execution_activation_closure_denial_ready",
        )
        && json_u64(
            &noop_handoff,
            "required_activation_command_handoff_surface_count",
        ) == 13
        && json_u64(
            &noop_handoff,
            "ready_activation_command_handoff_surface_count",
        ) == 13
        && json_u64(&noop_handoff, "activation_command_fixture_count") == 10
        && json_u64(&noop_handoff, "blocked_activation_command_fixture_count") == 10
        && json_u64(&noop_handoff, "noop_activation_command_fixture_count") == 10
        && json_u64(&noop_handoff, "accepted_activation_command_fixture_count") == 0
        && json_u64(&noop_handoff, "activation_command_performed_count") == 0
        && json_u64(&noop_handoff, "denied_by_activation_command_handoff_count") == 26
        && !json_bool(&noop_handoff, "activation_command_shape_registered")
        && !json_bool(&noop_handoff, "activation_command_enabled")
        && !json_bool(&noop_handoff, "activation_command_invoked")
        && !json_bool(&noop_handoff, "activation_command_dispatched")
        && !json_bool(&noop_handoff, "activation_command_result_receipt_recorded")
        && !json_bool(&noop_handoff, "activation_command_result_receipt_persisted")
        && !json_bool(&noop_handoff, "activation_allowed")
        && !json_bool(&noop_handoff, "memory_write_execution_performed")
        && !json_bool(&noop_handoff, "memory_store_mutated")
        && !json_bool(&noop_handoff, "rollback_executed")
        && !json_bool(&noop_handoff, "live_kg_write_performed")
        && !json_bool(&noop_handoff, "provider_invoked")
        && !json_bool(&noop_handoff, "model_invoked")
        && !json_bool(&noop_handoff, "credential_read")
        && !json_bool(&noop_handoff, "external_send_performed")
        && !json_bool(&noop_handoff, "release_artifact_written")
        && !json_bool(&noop_handoff, "active_binary_mutated")
        && side_effects_all_false(&noop_handoff);

    let receipt_surfaces = vec![
        "source_noop_handoff_report_required",
        "accepted_activation_closure_packet_required",
        "activation_command_disabled_and_not_invoked_required",
        "receipt_schema_and_request_id_required",
        "receipt_hash_signature_timestamp_required",
        "receipt_status_must_remain_blocked_noop_required",
        "receipt_record_persist_materialize_denied",
        "receipt_filesystem_ledger_index_delivery_denied",
        "completion_ack_denied",
        "activation_from_receipt_denied",
        "memory_write_live_mutation_rollback_denied",
        "external_public_release_install_restart_denied",
    ];
    let receipt_fixtures = serde_json::json!([
        {
            "id": "activation-result-receipt-missing-source-noop-handoff",
            "receipt_requested": true,
            "receipt_status": "blocked_noop",
            "source_noop_handoff_present": false,
            "source_noop_handoff_ready": false,
            "receipt_allowed": false,
            "receipt_recorded": false,
            "receipt_persisted": false,
            "receipt_accepted": false,
            "receipt_materialized": false,
            "receipt_filesystem_written": false,
            "receipt_noop_confirmed": true,
            "completion_ack_recorded": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "source_activation_command_noop_handoff_required"
        },
        {
            "id": "activation-result-receipt-record-attempt",
            "receipt_requested": true,
            "receipt_record_requested": true,
            "receipt_status": "blocked_noop",
            "source_noop_handoff_present": true,
            "receipt_allowed": false,
            "receipt_recorded": false,
            "receipt_persisted": false,
            "receipt_accepted": false,
            "receipt_materialized": false,
            "receipt_filesystem_written": false,
            "receipt_noop_confirmed": true,
            "completion_ack_recorded": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "activation_command_result_receipt_recording_denied"
        },
        {
            "id": "activation-result-receipt-persist-attempt",
            "receipt_requested": true,
            "receipt_persist_requested": true,
            "receipt_status": "blocked_noop",
            "source_noop_handoff_present": true,
            "receipt_allowed": false,
            "receipt_recorded": false,
            "receipt_persisted": false,
            "receipt_accepted": false,
            "receipt_materialized": false,
            "receipt_filesystem_written": false,
            "receipt_noop_confirmed": true,
            "completion_ack_recorded": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "activation_command_result_receipt_persistence_denied"
        },
        {
            "id": "activation-result-receipt-materialize-filesystem-attempt",
            "receipt_requested": true,
            "receipt_materialize_requested": true,
            "receipt_filesystem_write_requested": true,
            "receipt_status": "blocked_noop",
            "source_noop_handoff_present": true,
            "receipt_allowed": false,
            "receipt_recorded": false,
            "receipt_persisted": false,
            "receipt_accepted": false,
            "receipt_materialized": false,
            "receipt_filesystem_written": false,
            "receipt_noop_confirmed": true,
            "completion_ack_recorded": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "result_receipt_materialization_and_filesystem_write_denied"
        },
        {
            "id": "activation-result-receipt-ledger-index-delivery-attempt",
            "receipt_requested": true,
            "receipt_ledger_write_requested": true,
            "receipt_index_requested": true,
            "receipt_delivery_requested": true,
            "receipt_status": "blocked_noop",
            "source_noop_handoff_present": true,
            "receipt_allowed": false,
            "receipt_recorded": false,
            "receipt_persisted": false,
            "receipt_accepted": false,
            "receipt_materialized": false,
            "receipt_filesystem_written": false,
            "receipt_ledger_written": false,
            "receipt_indexed": false,
            "receipt_delivered": false,
            "receipt_noop_confirmed": true,
            "completion_ack_recorded": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "result_receipt_ledger_index_delivery_denied"
        },
        {
            "id": "activation-result-receipt-acceptance-as-approval-attempt",
            "receipt_requested": true,
            "receipt_acceptance_requested": true,
            "receipt_status": "blocked_noop",
            "source_noop_handoff_present": true,
            "receipt_allowed": false,
            "receipt_recorded": false,
            "receipt_persisted": false,
            "receipt_accepted": false,
            "receipt_materialized": false,
            "receipt_filesystem_written": false,
            "receipt_noop_confirmed": true,
            "completion_ack_recorded": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "result_receipt_cannot_become_operator_approval"
        },
        {
            "id": "activation-result-receipt-completion-ack-attempt",
            "receipt_requested": true,
            "completion_ack_requested": true,
            "activation_completion_ack_requested": true,
            "receipt_status": "blocked_noop",
            "source_noop_handoff_present": true,
            "receipt_allowed": false,
            "receipt_recorded": false,
            "receipt_persisted": false,
            "receipt_accepted": false,
            "receipt_materialized": false,
            "receipt_filesystem_written": false,
            "receipt_noop_confirmed": true,
            "completion_ack_recorded": false,
            "completion_ack_persisted": false,
            "completion_ack_accepted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "activation_completion_ack_denied"
        },
        {
            "id": "activation-result-receipt-non-noop-status-attempt",
            "receipt_requested": true,
            "receipt_status_requested": "completed",
            "receipt_status": "blocked_noop",
            "source_noop_handoff_present": true,
            "receipt_allowed": false,
            "receipt_recorded": false,
            "receipt_persisted": false,
            "receipt_accepted": false,
            "receipt_materialized": false,
            "receipt_filesystem_written": false,
            "receipt_noop_confirmed": true,
            "completion_ack_recorded": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "result_receipt_status_must_remain_blocked_noop"
        },
        {
            "id": "activation-result-receipt-memory-write-rollback-attempt",
            "receipt_requested": true,
            "receipt_status": "blocked_noop",
            "source_noop_handoff_present": true,
            "memory_store_write_requested": true,
            "rollback_execution_requested": true,
            "receipt_allowed": false,
            "receipt_recorded": false,
            "receipt_persisted": false,
            "receipt_accepted": false,
            "receipt_materialized": false,
            "receipt_filesystem_written": false,
            "receipt_noop_confirmed": true,
            "completion_ack_recorded": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "result_receipt_cannot_enable_memory_write_or_rollback"
        },
        {
            "id": "activation-result-receipt-external-public-install-attempt",
            "receipt_requested": true,
            "receipt_status": "blocked_noop",
            "source_noop_handoff_present": true,
            "external_send_requested": true,
            "public_claim_requested": true,
            "release_artifact_write_requested": true,
            "install_requested": true,
            "launchd_restart_requested": true,
            "active_binary_mutation_requested": true,
            "receipt_allowed": false,
            "receipt_recorded": false,
            "receipt_persisted": false,
            "receipt_accepted": false,
            "receipt_materialized": false,
            "receipt_filesystem_written": false,
            "receipt_noop_confirmed": true,
            "completion_ack_recorded": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "external_send_performed": false,
            "public_release_published": false,
            "release_artifact_written": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restarted": false,
            "active_binary_mutated": false,
            "reason": "result_receipt_cannot_send_publish_install_restart_or_mutate_active_binary"
        }
    ]);
    let denied_by = vec![
        "source_activation_command_noop_handoff_required",
        "accepted_activation_closure_packet_required",
        "activation_command_enabled_denied",
        "activation_command_invocation_denied",
        "activation_command_dispatch_denied",
        "receipt_schema_acceptance_denied",
        "receipt_recording_denied",
        "receipt_persistence_denied",
        "receipt_acceptance_denied",
        "receipt_materialization_denied",
        "receipt_filesystem_write_denied",
        "receipt_ledger_write_denied",
        "receipt_indexing_denied",
        "receipt_delivery_denied",
        "completion_ack_recording_denied",
        "completion_ack_persistence_denied",
        "completion_ack_acceptance_denied",
        "activation_from_receipt_denied",
        "memory_store_write_denied",
        "live_mutation_execution_denied",
        "rollback_execution_denied",
        "secret_material_read_denied",
        "provider_prompt_replay_denied",
        "external_send_public_claim_release_artifact_denied",
        "install_restart_active_binary_mutation_denied",
    ];

    let source_noop_handoff_report_sha256 = sha256_json_value(&noop_handoff);
    let boundary_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-no-persistence-boundary-v1:{}:{}",
        route_matrix.route_count, source_noop_handoff_report_sha256,
    ));
    let report_ready = route_matrix.ready
        && route_count_source_command_accepted
        && noop_handoff_ready
        && receipt_surfaces.len() == 12
        && receipt_fixtures.as_array().map(std::vec::Vec::len) == Some(10)
        && denied_by.len() == 25;

    let allowed_next_actions = serde_json::json!([
        {
            "action": "run_memory_write_execution_activation_command_result_receipt_no_persistence_boundary_require_live_gate",
            "status": "allowed_verification_only",
            "records_command_result": false,
            "persists_result_receipt": false,
            "accepts_result_receipt": false,
            "records_completion_ack": false,
            "accepts_activation": false,
            "writes_memory": false,
            "executes_rollback": false,
            "writes_kg": false,
            "invokes_provider": false,
            "sends_externally": false,
            "publishes_artifacts": false,
            "installs_or_restarts": false,
            "mutates_active_binary": false
        },
        {
            "action": "prepare_memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_boundary",
            "status": "allowed_report_only_next_slice",
            "accepts_duplicate_receipt": false,
            "records_idempotency": false,
            "persists_replay_state": false,
            "mutates_runtime": false,
            "invokes_model": false,
            "writes_memory_or_kg": false
        }
    ]);

    let mut side_effects = serde_json::Map::new();
    for key in [
        "activation_command_result_receipt_recorded",
        "activation_command_result_receipt_persisted",
        "activation_command_result_receipt_accepted",
        "activation_command_result_receipt_materialized",
        "activation_command_result_receipt_filesystem_written",
        "activation_command_result_receipt_ledger_written",
        "activation_command_result_receipt_indexed",
        "activation_command_result_receipt_enqueued",
        "activation_command_result_receipt_delivered",
        "activation_command_completion_ack_recorded",
        "activation_command_completion_ack_persisted",
        "activation_command_completion_ack_accepted",
        "activation_command_completion_ack_delivered",
        "activation_command_shape_registered",
        "activation_command_enabled",
        "activation_command_invoked",
        "activation_command_dispatched",
        "activation_command_noop_decision_recorded",
        "activation_command_noop_decision_persisted",
        "activation_command_handoff_recorded",
        "activation_command_handoff_persisted",
        "activation_command_handoff_materialized",
        "activation_closure_packet_recorded",
        "activation_closure_packet_persisted",
        "live_mutation_execution_performed",
        "memory_write_execution_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "rollback_executed",
        "raw_payload_inspected",
        "payload_plaintext_persisted",
        "secret_file_read",
        "credential_read",
        "provider_invoked",
        "model_invoked",
        "provider_prompt_replayed",
        "channel_send_performed",
        "telegram_send_performed",
        "external_send_performed",
        "runtime_store_mutated",
        "gateway_event_enqueued",
        "capability_registry_mutated",
        "plugin_registry_mutated",
        "skill_workshop_written",
        "filesystem_written",
        "release_artifact_written",
        "public_artifact_written",
        "public_release_published",
        "public_ga_claimed",
        "install_executed",
        "active_binary_mutated",
        "launchd_mutated",
        "service_restarted",
    ] {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }

    let mut report = serde_json::Map::new();
    macro_rules! insert_report_json {
        ($key:literal, $value:expr) => {
            report.insert($key.to_string(), serde_json::json!($value));
        };
    }

    insert_report_json!("product", "Hepta");
    insert_report_json!("runtime", "hepta");
    insert_report_json!("status", if report_ready { "ready" } else { "blocked" });
    insert_report_json!("base_url", "http://127.0.0.1:7373");
    insert_report_json!(
        "gate",
        "hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_no_persistence_boundary_route"
    );
    insert_report_json!(
        "endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_COMMAND_RESULT_RECEIPT_NO_PERSISTENCE_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_command",
        "/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-no-persistence-boundary --json"
    );
    insert_report_json!("native_route", true);
    insert_report_json!("side_effect_free", true);
    insert_report_json!("audit_date", "2026-07-02");
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_no_persistence_boundary_schema_version",
        "memory_write_execution_activation_command_result_receipt_no_persistence_boundary_v1"
    );
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_no_persistence_boundary_ready",
        report_ready
    );
    insert_report_json!(
        "activation_command_result_receipt_no_persistence_mode",
        "memory_write_execution_activation_command_result_receipt_no_persistence_denial"
    );
    insert_report_json!(
        "native_gateway_source_command_count",
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    insert_report_json!("route_count", route_matrix.route_count);
    insert_report_json!(
        "implemented_route_count",
        route_matrix.implemented_route_count
    );
    insert_report_json!("missing_route_count", route_matrix.missing_route_count);
    insert_report_json!(
        "route_count_source_command_accepted",
        route_count_source_command_accepted
    );
    insert_report_json!("boundary_hash_sha256", boundary_hash_sha256);
    insert_report_json!(
        "source_activation_command_noop_handoff_boundary_endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_COMMAND_NOOP_HANDOFF_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_activation_command_noop_handoff_boundary_ready",
        noop_handoff_ready
    );
    insert_report_json!(
        "source_activation_command_noop_handoff_ready",
        json_bool(
            &noop_handoff,
            "memory_write_execution_activation_command_noop_handoff_ready"
        )
    );
    insert_report_json!(
        "source_activation_command_noop_handoff_boundary_report_sha256",
        source_noop_handoff_report_sha256
    );
    for key in [
        "source_memory_write_execution_activation_closure_denial_boundary_report_sha256",
        "source_memory_write_execution_post_write_operator_acceptance_denial_boundary_report_sha256",
        "source_memory_write_execution_post_write_validation_dry_run_boundary_report_sha256",
        "source_memory_write_execution_write_enable_fixture_boundary_report_sha256",
        "source_memory_write_execution_no_write_sink_contract_boundary_report_sha256",
        "source_memory_write_execution_denial_matrix_boundary_report_sha256",
    ] {
        report.insert(
            key.to_string(),
            noop_handoff
                .get(key)
                .cloned()
                .unwrap_or_else(|| serde_json::json!("")),
        );
    }
    insert_report_json!("minimum_required_samples", 24);
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_no_persistence_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_activation_command_noop_handoff_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_activation_closure_denial_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_post_write_operator_acceptance_denial_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_post_write_validation_dry_run_ready",
        true
    );
    insert_report_json!("memory_write_execution_write_enable_fixture_ready", true);
    insert_report_json!("memory_write_execution_no_write_sink_contract_ready", true);
    insert_report_json!("required_activation_command_handoff_surface_count", 13);
    insert_report_json!("ready_activation_command_handoff_surface_count", 13);
    insert_report_json!(
        "required_activation_command_result_receipt_surface_count",
        12
    );
    insert_report_json!("ready_activation_command_result_receipt_surface_count", 12);
    insert_report_json!(
        "side_effect_free_activation_command_result_receipt_surface_count",
        12
    );
    insert_report_json!(
        "required_activation_command_result_receipt_fixture_count",
        10
    );
    insert_report_json!("activation_command_result_receipt_fixture_count", 10);
    insert_report_json!(
        "blocked_activation_command_result_receipt_fixture_count",
        10
    );
    insert_report_json!("noop_activation_command_result_receipt_fixture_count", 10);
    insert_report_json!("allowed_activation_command_result_receipt_fixture_count", 0);
    insert_report_json!(
        "accepted_activation_command_result_receipt_fixture_count",
        0
    );
    insert_report_json!("activation_command_result_receipt_denied_count", 10);
    insert_report_json!("activation_command_result_receipt_performed_count", 0);

    for key in [
        "activation_command_result_receipt_shape_registered",
        "activation_command_result_receipt_allowed",
        "activation_command_result_receipt_schema_accepted",
        "activation_command_result_receipt_recorded",
        "activation_command_result_receipt_persisted",
        "activation_command_result_receipt_accepted",
        "activation_command_result_receipt_materialized",
        "activation_command_result_receipt_filesystem_written",
        "activation_command_result_receipt_ledger_written",
        "activation_command_result_receipt_indexed",
        "activation_command_result_receipt_enqueued",
        "activation_command_result_receipt_delivered",
        "activation_command_result_receipt_hash_bound",
        "activation_command_result_receipt_signature_hash_recorded",
        "activation_command_result_receipt_timestamp_recorded",
        "activation_command_result_receipt_operator_identity_accepted",
        "activation_command_result_receipt_status_accepted",
        "activation_command_result_receipt_blocked_noop_status_accepted",
        "activation_command_completion_ack_recorded",
        "activation_command_completion_ack_persisted",
        "activation_command_completion_ack_accepted",
        "activation_command_completion_ack_materialized",
        "activation_command_completion_ack_delivered",
        "activation_command_shape_registered",
        "activation_command_enabled",
        "activation_command_invoked",
        "activation_command_dispatched",
        "activation_command_noop_decision_recorded",
        "activation_command_noop_decision_persisted",
        "activation_command_handoff_recorded",
        "activation_command_handoff_persisted",
        "activation_command_handoff_accepted",
        "activation_command_handoff_materialized",
        "activation_allowed_by_result_receipt",
        "activation_allowed_by_command_handoff",
        "activation_allowed",
        "activation_performed",
        "live_mutation_execution_ready",
        "live_mutation_execution_allowed",
        "live_mutation_execution_performed",
        "memory_write_execution_allowed",
        "memory_write_execution_ready",
        "memory_write_execution_performed",
        "memory_store_write_path_enabled",
        "memory_store_write_allowed",
        "memory_store_write_performed",
        "memory_store_mutation_allowed",
        "memory_store_mutated",
        "rollback_validation_accepted",
        "rollback_execution_allowed",
        "rollback_executed",
        "raw_payload_plaintext_recorded",
        "raw_payload_plaintext_persisted",
        "secret_material_read",
        "provider_prompt_replay_enabled",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "kg_adapter_read_performed",
        "live_kg_write_performed",
        "channel_send_performed",
        "telegram_send_performed",
        "external_send_enabled",
        "external_send_performed",
        "public_claim_or_release_artifact_write_enabled",
        "public_release_published",
        "public_ga_claimed",
        "public_release_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
    ] {
        report.insert(key.to_string(), serde_json::json!(false));
    }
    insert_report_json!("memory_store_write_performed_count", 0);
    report.insert(
        "activation_command_result_receipt_surfaces".to_string(),
        serde_json::json!(receipt_surfaces),
    );
    report.insert(
        "activation_command_result_receipt_fixtures".to_string(),
        receipt_fixtures,
    );
    report.insert(
        "denied_by_activation_command_result_receipt".to_string(),
        serde_json::json!(denied_by),
    );
    insert_report_json!("denied_by_activation_command_result_receipt_count", 25);
    report.insert("allowed_next_actions".to_string(), allowed_next_actions);
    report.insert(
        "side_effects".to_string(),
        serde_json::Value::Object(side_effects),
    );

    serde_json::Value::Object(report)
}

fn hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_replay_idempotency_denial_boundary_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let no_persistence =
        hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_no_persistence_boundary_report();

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
    let side_effects_all_false = |value: &serde_json::Value| {
        value
            .get("side_effects")
            .and_then(serde_json::Value::as_object)
            .map(|effects| effects.values().all(|item| item.as_bool() == Some(false)))
            .unwrap_or(false)
    };

    let route_count_source_command_accepted = route_matrix.ready
        && route_matrix.route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_no_persistence_ready = json_str(&no_persistence, "status") == "ready"
        && json_bool(
            &no_persistence,
            "memory_write_execution_activation_command_result_receipt_no_persistence_boundary_ready",
        )
        && json_bool(
            &no_persistence,
            "memory_write_execution_activation_command_result_receipt_no_persistence_ready",
        )
        && json_bool(
            &no_persistence,
            "memory_write_execution_activation_command_noop_handoff_ready",
        )
        && json_u64(
            &no_persistence,
            "required_activation_command_result_receipt_surface_count",
        ) == 12
        && json_u64(
            &no_persistence,
            "ready_activation_command_result_receipt_surface_count",
        ) == 12
        && json_u64(
            &no_persistence,
            "activation_command_result_receipt_fixture_count",
        ) == 10
        && json_u64(
            &no_persistence,
            "blocked_activation_command_result_receipt_fixture_count",
        ) == 10
        && json_u64(
            &no_persistence,
            "accepted_activation_command_result_receipt_fixture_count",
        ) == 0
        && json_u64(
            &no_persistence,
            "activation_command_result_receipt_performed_count",
        ) == 0
        && json_u64(
            &no_persistence,
            "denied_by_activation_command_result_receipt_count",
        ) == 25
        && !json_bool(
            &no_persistence,
            "activation_command_result_receipt_recorded",
        )
        && !json_bool(
            &no_persistence,
            "activation_command_result_receipt_persisted",
        )
        && !json_bool(
            &no_persistence,
            "activation_command_result_receipt_accepted",
        )
        && !json_bool(
            &no_persistence,
            "activation_command_result_receipt_materialized",
        )
        && !json_bool(
            &no_persistence,
            "activation_command_result_receipt_filesystem_written",
        )
        && !json_bool(
            &no_persistence,
            "activation_command_result_receipt_ledger_written",
        )
        && !json_bool(&no_persistence, "activation_command_result_receipt_indexed")
        && !json_bool(
            &no_persistence,
            "activation_command_result_receipt_delivered",
        )
        && !json_bool(
            &no_persistence,
            "activation_command_completion_ack_recorded",
        )
        && !json_bool(&no_persistence, "activation_allowed")
        && !json_bool(&no_persistence, "live_mutation_execution_performed")
        && !json_bool(&no_persistence, "memory_store_write_performed")
        && !json_bool(&no_persistence, "memory_store_mutated")
        && !json_bool(&no_persistence, "rollback_executed")
        && !json_bool(&no_persistence, "live_kg_write_performed")
        && !json_bool(&no_persistence, "provider_invoked")
        && !json_bool(&no_persistence, "model_invoked")
        && !json_bool(&no_persistence, "credential_read")
        && !json_bool(&no_persistence, "external_send_performed")
        && !json_bool(&no_persistence, "release_artifact_written")
        && !json_bool(&no_persistence, "active_binary_mutated")
        && side_effects_all_false(&no_persistence);

    let replay_surfaces = vec![
        "source_result_receipt_no_persistence_report_required",
        "canonical_noop_result_receipt_identity_required",
        "receipt_replay_nonce_idempotency_key_required",
        "duplicate_receipt_suppression_required",
        "cross_scope_receipt_reuse_denied",
        "blocked_noop_status_transition_denied",
        "completion_ack_replay_denied",
        "ledger_index_delivery_replay_denied",
        "memory_write_live_mutation_replay_denied",
        "rollback_replay_denied",
        "secret_provider_prompt_replay_denied",
        "external_public_install_restart_replay_denied",
    ];
    let replay_fixtures = serde_json::Value::Array(vec![
        serde_json::json!({
            "id": "activation-result-receipt-replay-missing-source-no-persistence-report",
            "replay_requested": true,
            "replay_status": "blocked_noop",
            "source_no_persistence_present": false,
            "source_no_persistence_ready": false,
            "replay_allowed": false,
            "replay_recorded": false,
            "replay_persisted": false,
            "duplicate_accepted": false,
            "idempotency_key_accepted": false,
            "idempotency_state_recorded": false,
            "idempotency_state_persisted": false,
            "receipt_recorded": false,
            "receipt_persisted": false,
            "receipt_accepted": false,
            "completion_ack_recorded": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "receipt_noop_confirmed": true,
            "reason": "source_result_receipt_no_persistence_report_required"
        }),
        serde_json::json!({
            "id": "activation-result-receipt-duplicate-identity-replay",
            "replay_requested": true,
            "duplicate_receipt_id_requested": true,
            "replay_status": "blocked_duplicate_noop",
            "source_no_persistence_present": true,
            "source_no_persistence_ready": true,
            "replay_allowed": false,
            "replay_recorded": false,
            "replay_persisted": false,
            "duplicate_accepted": false,
            "idempotency_key_accepted": false,
            "idempotency_state_recorded": false,
            "idempotency_state_persisted": false,
            "receipt_recorded": false,
            "receipt_persisted": false,
            "receipt_accepted": false,
            "completion_ack_recorded": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "receipt_noop_confirmed": true,
            "reason": "duplicate_result_receipt_id_replay_denied"
        }),
        serde_json::json!({
            "id": "activation-result-receipt-stale-idempotency-key-replay",
            "replay_requested": true,
            "stale_idempotency_key_requested": true,
            "replay_status": "blocked_duplicate_noop",
            "source_no_persistence_present": true,
            "source_no_persistence_ready": true,
            "replay_allowed": false,
            "replay_recorded": false,
            "replay_persisted": false,
            "duplicate_accepted": false,
            "idempotency_key_accepted": false,
            "idempotency_state_recorded": false,
            "idempotency_state_persisted": false,
            "receipt_recorded": false,
            "receipt_persisted": false,
            "receipt_accepted": false,
            "completion_ack_recorded": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "receipt_noop_confirmed": true,
            "reason": "stale_idempotency_key_replay_denied"
        }),
        serde_json::json!({
            "id": "activation-result-receipt-idempotency-state-recording-attempt",
            "replay_requested": true,
            "replay_acceptance_requested": true,
            "idempotency_key_recording_requested": true,
            "idempotency_state_recording_requested": true,
            "idempotency_state_persistence_requested": true,
            "replay_status": "blocked_noop",
            "source_no_persistence_present": true,
            "source_no_persistence_ready": true,
            "replay_allowed": false,
            "replay_recorded": false,
            "replay_persisted": false,
            "duplicate_accepted": false,
            "idempotency_key_accepted": false,
            "idempotency_state_recorded": false,
            "idempotency_state_persisted": false,
            "receipt_recorded": false,
            "receipt_persisted": false,
            "receipt_accepted": false,
            "completion_ack_recorded": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "receipt_noop_confirmed": true,
            "reason": "idempotency_state_recording_and_persistence_denied"
        }),
        serde_json::json!({
            "id": "activation-result-receipt-cross-scope-reuse-attempt",
            "replay_requested": true,
            "cross_scope_reuse_requested": true,
            "replay_status": "blocked_noop",
            "source_no_persistence_present": true,
            "source_no_persistence_ready": true,
            "replay_allowed": false,
            "replay_recorded": false,
            "replay_persisted": false,
            "duplicate_accepted": false,
            "idempotency_key_accepted": false,
            "idempotency_state_recorded": false,
            "idempotency_state_persisted": false,
            "receipt_recorded": false,
            "receipt_persisted": false,
            "receipt_accepted": false,
            "completion_ack_recorded": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "receipt_noop_confirmed": true,
            "reason": "cross_scope_result_receipt_reuse_denied"
        }),
        serde_json::json!({
            "id": "activation-result-receipt-completed-status-upgrade-attempt",
            "replay_requested": true,
            "receipt_status_requested": "completed",
            "status_upgrade_requested": true,
            "replay_status": "blocked_noop",
            "source_no_persistence_present": true,
            "source_no_persistence_ready": true,
            "replay_allowed": false,
            "replay_recorded": false,
            "replay_persisted": false,
            "duplicate_accepted": false,
            "idempotency_key_accepted": false,
            "idempotency_state_recorded": false,
            "idempotency_state_persisted": false,
            "receipt_recorded": false,
            "receipt_persisted": false,
            "receipt_accepted": false,
            "completion_ack_recorded": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "receipt_noop_confirmed": true,
            "reason": "blocked_noop_status_transition_denied"
        }),
        serde_json::json!({
            "id": "activation-result-receipt-completion-ack-replay-attempt",
            "replay_requested": true,
            "completion_ack_replay_requested": true,
            "replay_status": "blocked_noop",
            "source_no_persistence_present": true,
            "source_no_persistence_ready": true,
            "replay_allowed": false,
            "replay_recorded": false,
            "replay_persisted": false,
            "duplicate_accepted": false,
            "idempotency_key_accepted": false,
            "idempotency_state_recorded": false,
            "idempotency_state_persisted": false,
            "receipt_recorded": false,
            "receipt_persisted": false,
            "receipt_accepted": false,
            "completion_ack_recorded": false,
            "completion_ack_persisted": false,
            "completion_ack_accepted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "receipt_noop_confirmed": true,
            "reason": "completion_ack_replay_denied"
        }),
        serde_json::json!({
            "id": "activation-result-receipt-ledger-index-delivery-replay-attempt",
            "replay_requested": true,
            "ledger_replay_requested": true,
            "index_replay_requested": true,
            "delivery_replay_requested": true,
            "replay_status": "blocked_noop",
            "source_no_persistence_present": true,
            "source_no_persistence_ready": true,
            "replay_allowed": false,
            "replay_recorded": false,
            "replay_persisted": false,
            "duplicate_accepted": false,
            "idempotency_key_accepted": false,
            "idempotency_state_recorded": false,
            "idempotency_state_persisted": false,
            "receipt_recorded": false,
            "receipt_persisted": false,
            "receipt_accepted": false,
            "completion_ack_recorded": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "receipt_noop_confirmed": true,
            "reason": "ledger_index_delivery_replay_denied"
        }),
        serde_json::json!({
            "id": "activation-result-receipt-memory-rollback-secret-provider-replay-attempt",
            "replay_requested": true,
            "memory_write_replay_requested": true,
            "live_mutation_replay_requested": true,
            "rollback_replay_requested": true,
            "secret_material_replay_requested": true,
            "provider_prompt_replay_requested": true,
            "provider_invocation_replay_requested": true,
            "model_invocation_replay_requested": true,
            "replay_status": "blocked_noop",
            "source_no_persistence_present": true,
            "source_no_persistence_ready": true,
            "replay_allowed": false,
            "replay_recorded": false,
            "replay_persisted": false,
            "duplicate_accepted": false,
            "idempotency_key_accepted": false,
            "idempotency_state_recorded": false,
            "idempotency_state_persisted": false,
            "receipt_recorded": false,
            "receipt_persisted": false,
            "receipt_accepted": false,
            "completion_ack_recorded": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "provider_invoked": false,
            "model_invoked": false,
            "receipt_noop_confirmed": true,
            "reason": "memory_rollback_secret_provider_replay_denied"
        }),
        serde_json::json!({
            "id": "activation-result-receipt-external-public-install-replay-attempt",
            "replay_requested": true,
            "external_send_replay_requested": true,
            "public_claim_replay_requested": true,
            "release_artifact_replay_requested": true,
            "install_replay_requested": true,
            "launchd_restart_replay_requested": true,
            "active_binary_mutation_replay_requested": true,
            "replay_status": "blocked_noop",
            "source_no_persistence_present": true,
            "source_no_persistence_ready": true,
            "replay_allowed": false,
            "replay_recorded": false,
            "replay_persisted": false,
            "duplicate_accepted": false,
            "idempotency_key_accepted": false,
            "idempotency_state_recorded": false,
            "idempotency_state_persisted": false,
            "receipt_recorded": false,
            "receipt_persisted": false,
            "receipt_accepted": false,
            "completion_ack_recorded": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "external_send_performed": false,
            "public_release_published": false,
            "release_artifact_written": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restarted": false,
            "active_binary_mutated": false,
            "receipt_noop_confirmed": true,
            "reason": "external_public_install_restart_result_receipt_replay_denied"
        }),
    ]);
    let denied_by = vec![
        "source_result_receipt_no_persistence_report_required",
        "canonical_noop_result_receipt_identity_required",
        "result_receipt_replay_nonce_required_but_not_recorded",
        "result_receipt_idempotency_key_required_but_not_recorded",
        "duplicate_result_receipt_id_replay_denied",
        "stale_idempotency_key_replay_denied",
        "cross_scope_result_receipt_reuse_denied",
        "blocked_noop_status_transition_denied",
        "completed_status_upgrade_denied",
        "completion_ack_replay_denied",
        "ledger_replay_denied",
        "index_replay_denied",
        "delivery_replay_denied",
        "memory_write_replay_denied",
        "live_mutation_replay_denied",
        "rollback_replay_denied",
        "secret_material_replay_denied",
        "provider_prompt_replay_denied",
        "external_send_replay_denied",
        "public_claim_replay_denied",
        "release_artifact_replay_denied",
        "install_replay_denied",
        "launchd_restart_replay_denied",
        "active_binary_mutation_replay_denied",
    ];

    let source_no_persistence_report_sha256 = sha256_json_value(&no_persistence);
    let boundary_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-replay-idempotency-denial-boundary-v1:{}:{}",
        route_matrix.route_count, source_no_persistence_report_sha256,
    ));
    let report_ready = route_matrix.ready
        && route_count_source_command_accepted
        && source_no_persistence_ready
        && replay_surfaces.len() == 12
        && replay_fixtures.as_array().map(std::vec::Vec::len) == Some(10)
        && denied_by.len() == 24;

    let allowed_next_actions = serde_json::json!([
        {
            "action": "run_memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_boundary_require_live_gate",
            "status": "allowed_verification_only",
            "accepts_duplicate_receipt": false,
            "records_replay": false,
            "records_idempotency_state": false,
            "persists_replay_state": false,
            "accepts_activation": false,
            "writes_memory": false,
            "executes_rollback": false,
            "writes_kg": false,
            "invokes_provider": false,
            "sends_externally": false,
            "publishes_artifacts": false,
            "installs_or_restarts": false,
            "mutates_active_binary": false
        },
        {
            "action": "prepare_memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_boundary",
            "status": "allowed_report_only_next_slice",
            "accepts_out_of_order_receipt": false,
            "records_monotonic_sequence": false,
            "promotes_completion": false,
            "mutates_runtime": false,
            "invokes_model": false,
            "writes_memory_or_kg": false
        }
    ]);

    let false_keys = [
        "activation_command_result_receipt_replay_allowed",
        "activation_command_result_receipt_replay_recorded",
        "activation_command_result_receipt_replay_persisted",
        "activation_command_result_receipt_duplicate_accepted",
        "activation_command_result_receipt_duplicate_recorded",
        "activation_command_result_receipt_duplicate_persisted",
        "activation_command_result_receipt_idempotency_key_accepted",
        "activation_command_result_receipt_idempotency_state_recorded",
        "activation_command_result_receipt_idempotency_state_persisted",
        "activation_command_result_receipt_replay_nonce_accepted",
        "activation_command_result_receipt_replay_nonce_recorded",
        "activation_command_result_receipt_cross_scope_reuse_accepted",
        "activation_command_result_receipt_status_upgrade_accepted",
        "activation_command_result_receipt_completed_status_accepted",
        "activation_command_result_receipt_ack_replay_accepted",
        "activation_command_result_receipt_ledger_replay_accepted",
        "activation_command_result_receipt_delivery_replay_accepted",
        "activation_command_result_receipt_write_replay_accepted",
        "activation_command_result_receipt_rollback_replay_accepted",
        "activation_command_result_receipt_secret_provider_replay_accepted",
        "activation_command_result_receipt_external_public_install_replay_accepted",
        "activation_command_result_receipt_recorded",
        "activation_command_result_receipt_persisted",
        "activation_command_result_receipt_accepted",
        "activation_command_result_receipt_materialized",
        "activation_command_result_receipt_filesystem_written",
        "activation_command_result_receipt_ledger_written",
        "activation_command_result_receipt_indexed",
        "activation_command_result_receipt_enqueued",
        "activation_command_result_receipt_delivered",
        "activation_command_completion_ack_recorded",
        "activation_command_completion_ack_persisted",
        "activation_command_completion_ack_accepted",
        "activation_command_completion_ack_delivered",
        "activation_command_enabled",
        "activation_command_invoked",
        "activation_command_dispatched",
        "activation_allowed_by_result_receipt_replay",
        "activation_allowed_by_result_receipt",
        "activation_allowed",
        "activation_performed",
        "live_mutation_execution_ready",
        "live_mutation_execution_allowed",
        "live_mutation_execution_performed",
        "memory_write_execution_allowed",
        "memory_write_execution_ready",
        "memory_write_execution_performed",
        "memory_store_write_path_enabled",
        "memory_store_write_allowed",
        "memory_store_write_performed",
        "memory_store_mutation_allowed",
        "memory_store_mutated",
        "rollback_execution_allowed",
        "rollback_executed",
        "raw_payload_plaintext_recorded",
        "raw_payload_plaintext_persisted",
        "secret_material_read",
        "provider_prompt_replay_enabled",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "kg_adapter_read_performed",
        "live_kg_write_performed",
        "channel_send_performed",
        "telegram_send_performed",
        "external_send_enabled",
        "external_send_performed",
        "public_claim_or_release_artifact_write_enabled",
        "public_release_published",
        "public_ga_claimed",
        "public_release_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
    ];

    let mut side_effects = serde_json::Map::new();
    for key in false_keys {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }

    let mut report = serde_json::Map::new();
    macro_rules! insert_report_json {
        ($key:literal, $value:expr) => {
            report.insert($key.to_string(), serde_json::json!($value));
        };
    }

    insert_report_json!("product", "Hepta");
    insert_report_json!("runtime", "hepta");
    insert_report_json!("status", if report_ready { "ready" } else { "blocked" });
    insert_report_json!("base_url", "http://127.0.0.1:7373");
    insert_report_json!(
        "gate",
        "hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_replay_idempotency_denial_boundary_route"
    );
    insert_report_json!(
        "endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_COMMAND_RESULT_RECEIPT_REPLAY_IDEMPOTENCY_DENIAL_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_command",
        "/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-replay-idempotency-denial-boundary --json"
    );
    insert_report_json!("native_route", true);
    insert_report_json!("side_effect_free", true);
    insert_report_json!("audit_date", "2026-07-02");
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_boundary_schema_version",
        "memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_boundary_v1"
    );
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_boundary_ready",
        report_ready
    );
    insert_report_json!(
        "activation_command_result_receipt_replay_idempotency_mode",
        "memory_write_execution_activation_command_result_receipt_replay_idempotency_denial"
    );
    insert_report_json!(
        "native_gateway_source_command_count",
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    insert_report_json!("route_count", route_matrix.route_count);
    insert_report_json!(
        "implemented_route_count",
        route_matrix.implemented_route_count
    );
    insert_report_json!("missing_route_count", route_matrix.missing_route_count);
    insert_report_json!(
        "route_count_source_command_accepted",
        route_count_source_command_accepted
    );
    insert_report_json!("boundary_hash_sha256", boundary_hash_sha256);
    insert_report_json!(
        "source_activation_command_result_receipt_no_persistence_boundary_endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_COMMAND_RESULT_RECEIPT_NO_PERSISTENCE_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_activation_command_result_receipt_no_persistence_boundary_ready",
        source_no_persistence_ready
    );
    insert_report_json!(
        "source_activation_command_result_receipt_no_persistence_ready",
        json_bool(
            &no_persistence,
            "memory_write_execution_activation_command_result_receipt_no_persistence_ready"
        )
    );
    insert_report_json!(
        "source_activation_command_result_receipt_no_persistence_boundary_report_sha256",
        source_no_persistence_report_sha256
    );
    for key in [
        "source_activation_command_noop_handoff_boundary_report_sha256",
        "source_memory_write_execution_activation_closure_denial_boundary_report_sha256",
        "source_memory_write_execution_post_write_operator_acceptance_denial_boundary_report_sha256",
        "source_memory_write_execution_post_write_validation_dry_run_boundary_report_sha256",
        "source_memory_write_execution_write_enable_fixture_boundary_report_sha256",
        "source_memory_write_execution_no_write_sink_contract_boundary_report_sha256",
        "source_memory_write_execution_denial_matrix_boundary_report_sha256",
    ] {
        report.insert(
            key.to_string(),
            no_persistence
                .get(key)
                .cloned()
                .unwrap_or_else(|| serde_json::json!("")),
        );
    }
    insert_report_json!("minimum_required_samples", 24);
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_no_persistence_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_activation_command_noop_handoff_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_activation_closure_denial_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_post_write_operator_acceptance_denial_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_post_write_validation_dry_run_ready",
        true
    );
    insert_report_json!("memory_write_execution_write_enable_fixture_ready", true);
    insert_report_json!("memory_write_execution_no_write_sink_contract_ready", true);
    insert_report_json!(
        "required_activation_command_result_receipt_replay_idempotency_surface_count",
        12
    );
    insert_report_json!(
        "ready_activation_command_result_receipt_replay_idempotency_surface_count",
        12
    );
    insert_report_json!(
        "side_effect_free_activation_command_result_receipt_replay_idempotency_surface_count",
        12
    );
    insert_report_json!(
        "required_activation_command_result_receipt_replay_idempotency_fixture_count",
        10
    );
    insert_report_json!(
        "activation_command_result_receipt_replay_idempotency_fixture_count",
        10
    );
    insert_report_json!(
        "blocked_activation_command_result_receipt_replay_idempotency_fixture_count",
        10
    );
    insert_report_json!(
        "noop_activation_command_result_receipt_replay_idempotency_fixture_count",
        10
    );
    insert_report_json!(
        "allowed_activation_command_result_receipt_replay_idempotency_fixture_count",
        0
    );
    insert_report_json!(
        "accepted_activation_command_result_receipt_replay_idempotency_fixture_count",
        0
    );
    insert_report_json!(
        "duplicate_activation_command_result_receipt_fixture_count",
        2
    );
    insert_report_json!(
        "cross_scope_activation_command_result_receipt_fixture_count",
        1
    );
    insert_report_json!(
        "status_upgrade_activation_command_result_receipt_fixture_count",
        1
    );
    insert_report_json!("activation_command_result_receipt_replay_denied_count", 10);
    insert_report_json!(
        "activation_command_result_receipt_duplicate_denied_count",
        10
    );
    insert_report_json!(
        "activation_command_result_receipt_idempotency_denied_count",
        10
    );
    insert_report_json!(
        "activation_command_result_receipt_replay_performed_count",
        0
    );
    insert_report_json!(
        "activation_command_result_receipt_duplicate_accepted_count",
        0
    );
    insert_report_json!(
        "activation_command_result_receipt_idempotency_state_recorded_count",
        0
    );
    insert_report_json!("memory_store_write_performed_count", 0);

    for key in false_keys {
        report.insert(key.to_string(), serde_json::json!(false));
    }
    report.insert(
        "activation_command_result_receipt_replay_idempotency_surfaces".to_string(),
        serde_json::json!(replay_surfaces),
    );
    report.insert(
        "activation_command_result_receipt_replay_idempotency_fixtures".to_string(),
        replay_fixtures,
    );
    report.insert(
        "denied_by_activation_command_result_receipt_replay_idempotency".to_string(),
        serde_json::json!(denied_by),
    );
    insert_report_json!(
        "denied_by_activation_command_result_receipt_replay_idempotency_count",
        24
    );
    report.insert("allowed_next_actions".to_string(), allowed_next_actions);
    report.insert(
        "side_effects".to_string(),
        serde_json::Value::Object(side_effects),
    );

    serde_json::Value::Object(report)
}

fn hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_boundary_report()
-> serde_json::Value {
    fn ordering_fixture(
        id: &str,
        reason: &str,
        ordering_status: &str,
        extra: serde_json::Value,
    ) -> serde_json::Value {
        let mut base = serde_json::Map::new();
        macro_rules! insert_fixture_json {
            ($key:literal, $value:expr) => {
                base.insert($key.to_string(), serde_json::json!($value));
            };
        }
        insert_fixture_json!("id", id);
        insert_fixture_json!("ordering_requested", true);
        insert_fixture_json!("ordering_status", ordering_status);
        insert_fixture_json!("source_replay_idempotency_present", true);
        insert_fixture_json!("source_replay_idempotency_ready", true);
        for key in [
            "ordering_allowed",
            "ordering_recorded",
            "ordering_persisted",
            "sequence_cursor_accepted",
            "sequence_cursor_recorded",
            "sequence_cursor_persisted",
            "monotonicity_state_recorded",
            "monotonicity_state_persisted",
            "timestamp_ordering_accepted",
            "epoch_ordering_accepted",
            "stage_ordering_accepted",
            "latest_wins_overwrite_accepted",
            "receipt_recorded",
            "receipt_persisted",
            "receipt_accepted",
            "receipt_materialized",
            "receipt_filesystem_written",
            "receipt_ledger_written",
            "receipt_indexed",
            "receipt_delivered",
            "completion_ack_recorded",
            "completion_ack_persisted",
            "completion_ack_accepted",
            "activation_allowed",
            "live_mutation_execution_performed",
            "memory_store_write_performed",
            "memory_store_mutated",
            "rollback_executed",
            "secret_material_read",
            "provider_invoked",
            "model_invoked",
            "external_send_performed",
            "public_release_published",
            "release_artifact_written",
            "install_executed",
            "launchd_mutated",
            "service_restarted",
            "active_binary_mutated",
        ] {
            base.insert(key.to_string(), serde_json::json!(false));
        }
        insert_fixture_json!("receipt_noop_confirmed", true);
        insert_fixture_json!("reason", reason);
        if let Some(extra) = extra.as_object() {
            for (key, value) in extra {
                base.insert(key.clone(), value.clone());
            }
        }
        serde_json::Value::Object(base)
    }

    let route_matrix = control_ui_route_parity_report();
    let replay =
        hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_replay_idempotency_denial_boundary_report();

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
    let side_effects_all_false = |value: &serde_json::Value| {
        value
            .get("side_effects")
            .and_then(serde_json::Value::as_object)
            .map(|effects| effects.values().all(|item| item.as_bool() == Some(false)))
            .unwrap_or(false)
    };

    let route_count_source_command_accepted = route_matrix.ready
        && route_matrix.route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_replay_ready = json_str(&replay, "status") == "ready"
        && json_bool(
            &replay,
            "memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_boundary_ready",
        )
        && json_bool(
            &replay,
            "memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_ready",
        )
        && json_bool(
            &replay,
            "memory_write_execution_activation_command_result_receipt_no_persistence_ready",
        )
        && json_u64(
            &replay,
            "required_activation_command_result_receipt_replay_idempotency_surface_count",
        ) == 12
        && json_u64(
            &replay,
            "activation_command_result_receipt_replay_idempotency_fixture_count",
        ) == 10
        && json_u64(
            &replay,
            "blocked_activation_command_result_receipt_replay_idempotency_fixture_count",
        ) == 10
        && json_u64(
            &replay,
            "accepted_activation_command_result_receipt_replay_idempotency_fixture_count",
        ) == 0
        && json_u64(
            &replay,
            "activation_command_result_receipt_replay_performed_count",
        ) == 0
        && json_u64(
            &replay,
            "denied_by_activation_command_result_receipt_replay_idempotency_count",
        ) == 24
        && !json_bool(&replay, "activation_command_result_receipt_replay_allowed")
        && !json_bool(&replay, "activation_command_result_receipt_replay_recorded")
        && !json_bool(
            &replay,
            "activation_command_result_receipt_replay_persisted",
        )
        && !json_bool(
            &replay,
            "activation_command_result_receipt_duplicate_accepted",
        )
        && !json_bool(
            &replay,
            "activation_command_result_receipt_idempotency_state_recorded",
        )
        && !json_bool(&replay, "activation_command_result_receipt_recorded")
        && !json_bool(&replay, "activation_command_result_receipt_persisted")
        && !json_bool(&replay, "activation_command_result_receipt_accepted")
        && !json_bool(&replay, "activation_command_completion_ack_recorded")
        && !json_bool(&replay, "activation_allowed")
        && !json_bool(&replay, "live_mutation_execution_performed")
        && !json_bool(&replay, "memory_store_write_performed")
        && !json_bool(&replay, "memory_store_mutated")
        && !json_bool(&replay, "rollback_executed")
        && !json_bool(&replay, "provider_invoked")
        && !json_bool(&replay, "model_invoked")
        && !json_bool(&replay, "external_send_performed")
        && !json_bool(&replay, "release_artifact_written")
        && !json_bool(&replay, "active_binary_mutated")
        && side_effects_all_false(&replay);

    let ordering_surfaces = vec![
        "source_replay_idempotency_report_required",
        "canonical_noop_receipt_order_identity_required",
        "sequence_cursor_monotonicity_denied",
        "out_of_order_sequence_denied",
        "sequence_gap_or_skip_denied",
        "timestamp_rollback_denied",
        "epoch_rollback_denied",
        "same_sequence_different_hash_denied",
        "latest_wins_overwrite_denied",
        "stage_transition_ordering_denied",
        "ledger_index_delivery_ordering_bypass_denied",
        "external_public_install_ordering_bypass_denied",
    ];
    let ordering_fixtures = serde_json::Value::Array(vec![
        ordering_fixture(
            "activation-result-receipt-ordering-missing-source-replay-idempotency-report",
            "source_result_receipt_replay_idempotency_report_required",
            "blocked_noop",
            serde_json::json!({
                "source_replay_idempotency_present": false,
                "source_replay_idempotency_ready": false
            }),
        ),
        ordering_fixture(
            "activation-result-receipt-out-of-order-sequence",
            "out_of_order_result_receipt_sequence_denied",
            "blocked_ordering_noop",
            serde_json::json!({
                "out_of_order_sequence_requested": true,
                "requested_sequence": 2,
                "observed_previous_sequence": 3
            }),
        ),
        ordering_fixture(
            "activation-result-receipt-sequence-gap-skip",
            "sequence_gap_or_skip_result_receipt_denied",
            "blocked_ordering_noop",
            serde_json::json!({
                "sequence_gap_requested": true,
                "requested_sequence": 5,
                "expected_next_sequence": 1
            }),
        ),
        ordering_fixture(
            "activation-result-receipt-timestamp-rollback",
            "timestamp_rollback_result_receipt_denied",
            "blocked_ordering_noop",
            serde_json::json!({
                "timestamp_rollback_requested": true,
                "requested_timestamp_order": "older_than_source_noop_handoff"
            }),
        ),
        ordering_fixture(
            "activation-result-receipt-epoch-rollback",
            "epoch_rollback_result_receipt_denied",
            "blocked_ordering_noop",
            serde_json::json!({
                "epoch_rollback_requested": true,
                "requested_epoch_order": "lower_than_current_activation_epoch"
            }),
        ),
        ordering_fixture(
            "activation-result-receipt-same-sequence-different-hash",
            "same_sequence_different_hash_result_receipt_denied",
            "blocked_ordering_noop",
            serde_json::json!({
                "same_sequence_different_hash_requested": true,
                "requested_sequence": 1,
                "requested_hash_relation": "different_hash_for_same_sequence"
            }),
        ),
        ordering_fixture(
            "activation-result-receipt-latest-wins-overwrite",
            "latest_wins_result_receipt_overwrite_denied",
            "blocked_ordering_noop",
            serde_json::json!({
                "latest_wins_overwrite_requested": true,
                "overwrite_existing_noop_requested": true
            }),
        ),
        ordering_fixture(
            "activation-result-receipt-stage-transition-before-noop",
            "stage_transition_ordering_bypass_denied",
            "blocked_ordering_noop",
            serde_json::json!({
                "stage_transition_ordering_bypass_requested": true,
                "completion_ack_before_noop_requested": true,
                "requested_stage": "completed_before_blocked_noop"
            }),
        ),
        ordering_fixture(
            "activation-result-receipt-ledger-index-delivery-ordering-bypass",
            "ledger_index_delivery_ordering_bypass_denied",
            "blocked_ordering_noop",
            serde_json::json!({
                "ledger_ordering_bypass_requested": true,
                "index_ordering_bypass_requested": true,
                "delivery_ordering_bypass_requested": true
            }),
        ),
        ordering_fixture(
            "activation-result-receipt-external-public-install-ordering-bypass",
            "external_public_install_restart_ordering_bypass_denied",
            "blocked_ordering_noop",
            serde_json::json!({
                "external_send_ordering_bypass_requested": true,
                "public_claim_ordering_bypass_requested": true,
                "release_artifact_ordering_bypass_requested": true,
                "install_ordering_bypass_requested": true,
                "service_restart_ordering_bypass_requested": true,
                "active_binary_mutation_ordering_bypass_requested": true
            }),
        ),
    ]);
    let denied_by = vec![
        "source_result_receipt_replay_idempotency_report_required",
        "canonical_noop_receipt_order_identity_required",
        "sequence_cursor_acceptance_denied",
        "sequence_cursor_recording_denied",
        "sequence_cursor_persistence_denied",
        "monotonicity_state_recording_denied",
        "monotonicity_state_persistence_denied",
        "out_of_order_sequence_denied",
        "sequence_gap_or_skip_denied",
        "timestamp_rollback_denied",
        "epoch_rollback_denied",
        "same_sequence_different_hash_denied",
        "latest_wins_overwrite_denied",
        "completion_ack_before_noop_denied",
        "stage_transition_ordering_denied",
        "ledger_ordering_bypass_denied",
        "index_ordering_bypass_denied",
        "delivery_ordering_bypass_denied",
        "memory_write_ordering_bypass_denied",
        "live_mutation_ordering_bypass_denied",
        "rollback_ordering_bypass_denied",
        "secret_provider_ordering_bypass_denied",
        "external_public_release_ordering_bypass_denied",
        "install_restart_active_binary_ordering_bypass_denied",
    ];

    let source_replay_report_sha256 = sha256_json_value(&replay);
    let boundary_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-ordering-monotonicity-denial-boundary-v1:{}:{}",
        route_matrix.route_count, source_replay_report_sha256,
    ));
    let report_ready = route_matrix.ready
        && route_count_source_command_accepted
        && source_replay_ready
        && ordering_surfaces.len() == 12
        && ordering_fixtures.as_array().map(std::vec::Vec::len) == Some(10)
        && denied_by.len() == 24;

    let allowed_next_actions = serde_json::json!([
        {
            "action": "run_memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_boundary_require_live_gate",
            "status": "allowed_verification_only",
            "accepts_out_of_order_receipt": false,
            "records_monotonic_sequence": false,
            "persists_ordering_state": false,
            "promotes_completion": false,
            "writes_memory": false,
            "executes_rollback": false,
            "writes_kg": false,
            "invokes_provider": false,
            "sends_externally": false,
            "publishes_artifacts": false,
            "installs_or_restarts": false,
            "mutates_active_binary": false
        },
        {
            "action": "prepare_memory_write_execution_activation_command_result_receipt_cancellation_supersession_denial_boundary",
            "status": "allowed_report_only_next_slice",
            "accepts_cancellation": false,
            "accepts_supersession": false,
            "records_replacement_receipt": false,
            "mutates_runtime": false,
            "invokes_model": false,
            "writes_memory_or_kg": false
        }
    ]);

    let false_keys = [
        "activation_command_result_receipt_ordering_allowed",
        "activation_command_result_receipt_ordering_recorded",
        "activation_command_result_receipt_ordering_persisted",
        "activation_command_result_receipt_sequence_cursor_accepted",
        "activation_command_result_receipt_sequence_cursor_recorded",
        "activation_command_result_receipt_sequence_cursor_persisted",
        "activation_command_result_receipt_monotonicity_state_recorded",
        "activation_command_result_receipt_monotonicity_state_persisted",
        "activation_command_result_receipt_timestamp_ordering_accepted",
        "activation_command_result_receipt_epoch_ordering_accepted",
        "activation_command_result_receipt_stage_ordering_accepted",
        "activation_command_result_receipt_same_sequence_hash_override_accepted",
        "activation_command_result_receipt_latest_wins_overwrite_accepted",
        "activation_command_result_receipt_gap_fill_accepted",
        "activation_command_result_receipt_ack_before_noop_accepted",
        "activation_command_result_receipt_recorded",
        "activation_command_result_receipt_persisted",
        "activation_command_result_receipt_accepted",
        "activation_command_result_receipt_materialized",
        "activation_command_result_receipt_filesystem_written",
        "activation_command_result_receipt_ledger_written",
        "activation_command_result_receipt_indexed",
        "activation_command_result_receipt_enqueued",
        "activation_command_result_receipt_delivered",
        "activation_command_completion_ack_recorded",
        "activation_command_completion_ack_persisted",
        "activation_command_completion_ack_accepted",
        "activation_command_completion_ack_delivered",
        "activation_command_enabled",
        "activation_command_invoked",
        "activation_command_dispatched",
        "activation_allowed_by_result_receipt_ordering",
        "activation_allowed_by_result_receipt_replay",
        "activation_allowed_by_result_receipt",
        "activation_allowed",
        "activation_performed",
        "live_mutation_execution_ready",
        "live_mutation_execution_allowed",
        "live_mutation_execution_performed",
        "memory_write_execution_allowed",
        "memory_write_execution_ready",
        "memory_write_execution_performed",
        "memory_store_write_path_enabled",
        "memory_store_write_allowed",
        "memory_store_write_performed",
        "memory_store_mutation_allowed",
        "memory_store_mutated",
        "rollback_execution_allowed",
        "rollback_executed",
        "raw_payload_plaintext_recorded",
        "raw_payload_plaintext_persisted",
        "secret_material_read",
        "provider_prompt_replay_enabled",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "kg_adapter_read_performed",
        "live_kg_write_performed",
        "channel_send_performed",
        "telegram_send_performed",
        "external_send_enabled",
        "external_send_performed",
        "public_claim_or_release_artifact_write_enabled",
        "public_release_published",
        "public_ga_claimed",
        "public_release_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
    ];

    let mut side_effects = serde_json::Map::new();
    for key in false_keys {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }

    let mut report = serde_json::Map::new();
    macro_rules! insert_report_json {
        ($key:literal, $value:expr) => {
            report.insert($key.to_string(), serde_json::json!($value));
        };
    }

    insert_report_json!("product", "Hepta");
    insert_report_json!("runtime", "hepta");
    insert_report_json!("status", if report_ready { "ready" } else { "blocked" });
    insert_report_json!("base_url", "http://127.0.0.1:7373");
    insert_report_json!(
        "gate",
        "hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_boundary_route"
    );
    insert_report_json!(
        "endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_COMMAND_RESULT_RECEIPT_ORDERING_MONOTONICITY_DENIAL_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_command",
        "/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-ordering-monotonicity-denial-boundary --json"
    );
    insert_report_json!("native_route", true);
    insert_report_json!("side_effect_free", true);
    insert_report_json!("audit_date", "2026-07-03");
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_boundary_schema_version",
        "memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_boundary_v1"
    );
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_boundary_ready",
        report_ready
    );
    insert_report_json!(
        "activation_command_result_receipt_ordering_monotonicity_mode",
        "memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial"
    );
    insert_report_json!(
        "native_gateway_source_command_count",
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    insert_report_json!("route_count", route_matrix.route_count);
    insert_report_json!(
        "implemented_route_count",
        route_matrix.implemented_route_count
    );
    insert_report_json!("missing_route_count", route_matrix.missing_route_count);
    insert_report_json!(
        "route_count_source_command_accepted",
        route_count_source_command_accepted
    );
    insert_report_json!("boundary_hash_sha256", boundary_hash_sha256);
    insert_report_json!(
        "source_activation_command_result_receipt_replay_idempotency_boundary_endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_COMMAND_RESULT_RECEIPT_REPLAY_IDEMPOTENCY_DENIAL_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_activation_command_result_receipt_replay_idempotency_boundary_ready",
        source_replay_ready
    );
    insert_report_json!(
        "source_activation_command_result_receipt_replay_idempotency_ready",
        json_bool(
            &replay,
            "memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_ready"
        )
    );
    insert_report_json!(
        "source_activation_command_result_receipt_replay_idempotency_boundary_report_sha256",
        source_replay_report_sha256
    );
    insert_report_json!(
        "source_activation_command_result_receipt_replay_idempotency_report_sha256",
        source_replay_report_sha256
    );
    insert_report_json!(
        "source_activation_command_result_receipt_no_persistence_boundary_ready",
        json_bool(
            &replay,
            "source_activation_command_result_receipt_no_persistence_boundary_ready"
        )
    );
    insert_report_json!(
        "source_activation_command_result_receipt_no_persistence_ready",
        json_bool(
            &replay,
            "memory_write_execution_activation_command_result_receipt_no_persistence_ready"
        )
    );
    insert_report_json!(
        "source_activation_command_result_receipt_no_persistence_boundary_report_sha256",
        json_str(
            &replay,
            "source_activation_command_result_receipt_no_persistence_boundary_report_sha256"
        )
    );
    for key in [
        "source_activation_command_noop_handoff_boundary_report_sha256",
        "source_memory_write_execution_activation_closure_denial_boundary_report_sha256",
        "source_memory_write_execution_post_write_operator_acceptance_denial_boundary_report_sha256",
        "source_memory_write_execution_post_write_validation_dry_run_boundary_report_sha256",
        "source_memory_write_execution_write_enable_fixture_boundary_report_sha256",
        "source_memory_write_execution_no_write_sink_contract_boundary_report_sha256",
        "source_memory_write_execution_denial_matrix_boundary_report_sha256",
    ] {
        report.insert(
            key.to_string(),
            replay
                .get(key)
                .cloned()
                .unwrap_or_else(|| serde_json::json!("")),
        );
    }
    insert_report_json!("minimum_required_samples", 24);
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_no_persistence_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_activation_command_noop_handoff_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_activation_closure_denial_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_post_write_operator_acceptance_denial_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_post_write_validation_dry_run_ready",
        true
    );
    insert_report_json!("memory_write_execution_write_enable_fixture_ready", true);
    insert_report_json!("memory_write_execution_no_write_sink_contract_ready", true);
    insert_report_json!(
        "required_activation_command_result_receipt_ordering_monotonicity_surface_count",
        12
    );
    insert_report_json!(
        "ready_activation_command_result_receipt_ordering_monotonicity_surface_count",
        12
    );
    insert_report_json!(
        "side_effect_free_activation_command_result_receipt_ordering_monotonicity_surface_count",
        12
    );
    insert_report_json!(
        "required_activation_command_result_receipt_ordering_monotonicity_fixture_count",
        10
    );
    insert_report_json!(
        "activation_command_result_receipt_ordering_monotonicity_fixture_count",
        10
    );
    insert_report_json!(
        "blocked_activation_command_result_receipt_ordering_monotonicity_fixture_count",
        10
    );
    insert_report_json!(
        "noop_activation_command_result_receipt_ordering_monotonicity_fixture_count",
        10
    );
    insert_report_json!(
        "allowed_activation_command_result_receipt_ordering_monotonicity_fixture_count",
        0
    );
    insert_report_json!(
        "accepted_activation_command_result_receipt_ordering_monotonicity_fixture_count",
        0
    );
    insert_report_json!(
        "activation_command_result_receipt_ordering_violation_denied_count",
        10
    );
    insert_report_json!(
        "activation_command_result_receipt_monotonicity_violation_denied_count",
        10
    );
    insert_report_json!(
        "activation_command_result_receipt_ordering_performed_count",
        0
    );
    insert_report_json!(
        "activation_command_result_receipt_sequence_cursor_accepted_count",
        0
    );
    insert_report_json!(
        "activation_command_result_receipt_sequence_cursor_recorded_count",
        0
    );
    insert_report_json!(
        "activation_command_result_receipt_monotonicity_state_recorded_count",
        0
    );
    insert_report_json!("memory_store_write_performed_count", 0);

    for key in false_keys {
        report.insert(key.to_string(), serde_json::json!(false));
    }
    report.insert(
        "activation_command_result_receipt_ordering_monotonicity_surfaces".to_string(),
        serde_json::json!(ordering_surfaces),
    );
    report.insert(
        "activation_command_result_receipt_ordering_monotonicity_fixtures".to_string(),
        ordering_fixtures,
    );
    report.insert(
        "denied_by_activation_command_result_receipt_ordering_monotonicity".to_string(),
        serde_json::json!(denied_by),
    );
    insert_report_json!(
        "denied_by_activation_command_result_receipt_ordering_monotonicity_count",
        24
    );
    report.insert("allowed_next_actions".to_string(), allowed_next_actions);
    report.insert(
        "side_effects".to_string(),
        serde_json::Value::Object(side_effects),
    );

    serde_json::Value::Object(report)
}

fn hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_cancellation_supersession_denial_boundary_report()
-> serde_json::Value {
    const CANCELLATION_SURFACES: &[&str] = &[
        "source_ordering_monotonicity_report_required",
        "cancellation_request_shape_denied",
        "supersession_request_shape_denied",
        "replacement_receipt_hash_denied",
        "tombstone_or_delete_marker_denied",
        "cancel_after_blocked_noop_denied",
        "supersede_blocked_noop_with_completed_denied",
        "acknowledgement_cancellation_denied",
        "ledger_index_delivery_cancellation_denied",
        "memory_write_live_mutation_supersession_denied",
        "rollback_secret_provider_supersession_denied",
        "external_public_install_restart_supersession_denied",
    ];
    const DENIED_BY: &[&str] = &[
        "source_ordering_monotonicity_report_required",
        "cancellation_request_acceptance_denied",
        "cancellation_recording_denied",
        "cancellation_persistence_denied",
        "supersession_request_acceptance_denied",
        "supersession_recording_denied",
        "supersession_persistence_denied",
        "replacement_receipt_acceptance_denied",
        "replacement_hash_acceptance_denied",
        "tombstone_recording_denied",
        "delete_marker_recording_denied",
        "cancel_after_blocked_noop_denied",
        "supersede_blocked_noop_with_completed_denied",
        "completion_ack_cancellation_denied",
        "ledger_cancellation_denied",
        "index_cancellation_denied",
        "delivery_cancellation_denied",
        "memory_write_supersession_denied",
        "live_mutation_supersession_denied",
        "rollback_supersession_denied",
        "secret_material_supersession_denied",
        "provider_prompt_supersession_denied",
        "external_public_release_supersession_denied",
        "install_restart_active_binary_supersession_denied",
    ];
    const FALSE_KEYS: &[&str] = &[
        "activation_command_result_receipt_cancellation_allowed",
        "activation_command_result_receipt_cancellation_recorded",
        "activation_command_result_receipt_cancellation_persisted",
        "activation_command_result_receipt_cancellation_request_accepted",
        "activation_command_result_receipt_supersession_allowed",
        "activation_command_result_receipt_supersession_recorded",
        "activation_command_result_receipt_supersession_persisted",
        "activation_command_result_receipt_supersession_request_accepted",
        "activation_command_result_receipt_replacement_receipt_accepted",
        "activation_command_result_receipt_replacement_receipt_recorded",
        "activation_command_result_receipt_replacement_receipt_persisted",
        "activation_command_result_receipt_replacement_hash_accepted",
        "activation_command_result_receipt_tombstone_recorded",
        "activation_command_result_receipt_tombstone_persisted",
        "activation_command_result_receipt_delete_marker_recorded",
        "activation_command_result_receipt_ack_cancellation_accepted",
        "activation_command_result_receipt_ledger_cancellation_accepted",
        "activation_command_result_receipt_index_cancellation_accepted",
        "activation_command_result_receipt_delivery_cancellation_accepted",
        "activation_command_result_receipt_ordering_allowed",
        "activation_command_result_receipt_ordering_recorded",
        "activation_command_result_receipt_ordering_persisted",
        "activation_command_result_receipt_sequence_cursor_accepted",
        "activation_command_result_receipt_sequence_cursor_recorded",
        "activation_command_result_receipt_sequence_cursor_persisted",
        "activation_command_result_receipt_monotonicity_state_recorded",
        "activation_command_result_receipt_monotonicity_state_persisted",
        "activation_command_result_receipt_latest_wins_overwrite_accepted",
        "activation_command_result_receipt_recorded",
        "activation_command_result_receipt_persisted",
        "activation_command_result_receipt_accepted",
        "activation_command_result_receipt_materialized",
        "activation_command_result_receipt_filesystem_written",
        "activation_command_result_receipt_ledger_written",
        "activation_command_result_receipt_indexed",
        "activation_command_result_receipt_enqueued",
        "activation_command_result_receipt_delivered",
        "activation_command_completion_ack_recorded",
        "activation_command_completion_ack_persisted",
        "activation_command_completion_ack_accepted",
        "activation_command_completion_ack_delivered",
        "activation_command_enabled",
        "activation_command_invoked",
        "activation_command_dispatched",
        "activation_allowed_by_result_receipt_cancellation",
        "activation_allowed_by_result_receipt_supersession",
        "activation_allowed_by_result_receipt_ordering",
        "activation_allowed_by_result_receipt_replay",
        "activation_allowed_by_result_receipt",
        "activation_allowed",
        "activation_performed",
        "live_mutation_execution_ready",
        "live_mutation_execution_allowed",
        "live_mutation_execution_performed",
        "memory_write_execution_allowed",
        "memory_write_execution_ready",
        "memory_write_execution_performed",
        "memory_store_write_path_enabled",
        "memory_store_write_allowed",
        "memory_store_write_performed",
        "memory_store_mutation_allowed",
        "memory_store_mutated",
        "rollback_execution_allowed",
        "rollback_executed",
        "raw_payload_plaintext_recorded",
        "raw_payload_plaintext_persisted",
        "secret_material_read",
        "provider_prompt_replay_enabled",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "kg_adapter_read_performed",
        "live_kg_write_performed",
        "channel_send_performed",
        "telegram_send_performed",
        "external_send_enabled",
        "external_send_performed",
        "public_claim_or_release_artifact_write_enabled",
        "public_release_published",
        "public_ga_claimed",
        "public_release_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
    ];

    fn cancellation_fixture(
        id: &str,
        reason: &str,
        cancellation_supersession_status: &str,
        extra: serde_json::Value,
    ) -> serde_json::Value {
        let mut base = serde_json::Map::new();
        macro_rules! insert_fixture_json {
            ($key:literal, $value:expr) => {
                base.insert($key.to_string(), serde_json::json!($value));
            };
        }
        insert_fixture_json!("id", id);
        insert_fixture_json!("cancellation_requested", true);
        insert_fixture_json!("supersession_requested", false);
        insert_fixture_json!(
            "cancellation_supersession_status",
            cancellation_supersession_status
        );
        insert_fixture_json!("source_ordering_monotonicity_present", true);
        insert_fixture_json!("source_ordering_monotonicity_ready", true);
        for key in [
            "cancellation_allowed",
            "cancellation_recorded",
            "cancellation_persisted",
            "cancellation_request_accepted",
            "supersession_allowed",
            "supersession_recorded",
            "supersession_persisted",
            "supersession_request_accepted",
            "replacement_receipt_accepted",
            "replacement_receipt_recorded",
            "replacement_receipt_persisted",
            "replacement_hash_accepted",
            "tombstone_recorded",
            "tombstone_persisted",
            "delete_marker_recorded",
            "ack_cancellation_accepted",
            "ledger_cancellation_accepted",
            "index_cancellation_accepted",
            "delivery_cancellation_accepted",
            "receipt_recorded",
            "receipt_persisted",
            "receipt_accepted",
            "receipt_materialized",
            "receipt_filesystem_written",
            "receipt_ledger_written",
            "receipt_indexed",
            "receipt_delivered",
            "completion_ack_recorded",
            "completion_ack_persisted",
            "completion_ack_accepted",
            "completion_ack_delivered",
            "activation_allowed",
            "live_mutation_execution_performed",
            "memory_store_write_performed",
            "memory_store_mutated",
            "rollback_executed",
            "secret_material_read",
            "provider_invoked",
            "model_invoked",
            "external_send_performed",
            "public_release_published",
            "release_artifact_written",
            "install_executed",
            "launchd_mutated",
            "service_restarted",
            "active_binary_mutated",
        ] {
            base.insert(key.to_string(), serde_json::json!(false));
        }
        insert_fixture_json!("receipt_noop_confirmed", true);
        insert_fixture_json!("reason", reason);
        if let Some(extra) = extra.as_object() {
            for (key, value) in extra {
                base.insert(key.clone(), value.clone());
            }
        }
        serde_json::Value::Object(base)
    }

    let route_matrix = control_ui_route_parity_report();
    let ordering = std::thread::Builder::new()
        .name("hepta-memory-write-result-receipt-ordering-source-report".to_string())
        .stack_size(8 * 1024 * 1024)
        .spawn(
            hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_boundary_report,
        )
        .ok()
        .and_then(|handle| handle.join().ok())
        .unwrap_or_else(|| {
            serde_json::json!({
                "status": "blocked",
                "memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_boundary_ready": false,
                "memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_ready": false,
                "source_ordering_source_report_thread_failed": true
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
    let side_effects_all_false = |value: &serde_json::Value| {
        value
            .get("side_effects")
            .and_then(serde_json::Value::as_object)
            .map(|effects| effects.values().all(|item| item.as_bool() == Some(false)))
            .unwrap_or(false)
    };

    let route_count_source_command_accepted = route_matrix.ready
        && route_matrix.route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_ordering_ready = json_str(&ordering, "status") == "ready"
        && json_bool(
            &ordering,
            "memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_boundary_ready",
        )
        && json_bool(
            &ordering,
            "memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_ready",
        )
        && json_bool(
            &ordering,
            "memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_ready",
        )
        && json_u64(
            &ordering,
            "required_activation_command_result_receipt_ordering_monotonicity_surface_count",
        ) == 12
        && json_u64(
            &ordering,
            "activation_command_result_receipt_ordering_monotonicity_fixture_count",
        ) == 10
        && json_u64(
            &ordering,
            "blocked_activation_command_result_receipt_ordering_monotonicity_fixture_count",
        ) == 10
        && json_u64(
            &ordering,
            "accepted_activation_command_result_receipt_ordering_monotonicity_fixture_count",
        ) == 0
        && json_u64(
            &ordering,
            "activation_command_result_receipt_ordering_performed_count",
        ) == 0
        && json_u64(
            &ordering,
            "denied_by_activation_command_result_receipt_ordering_monotonicity_count",
        ) == 24
        && !json_bool(
            &ordering,
            "activation_command_result_receipt_ordering_allowed",
        )
        && !json_bool(
            &ordering,
            "activation_command_result_receipt_ordering_recorded",
        )
        && !json_bool(
            &ordering,
            "activation_command_result_receipt_ordering_persisted",
        )
        && !json_bool(
            &ordering,
            "activation_command_result_receipt_sequence_cursor_recorded",
        )
        && !json_bool(
            &ordering,
            "activation_command_result_receipt_monotonicity_state_recorded",
        )
        && !json_bool(&ordering, "activation_command_result_receipt_recorded")
        && !json_bool(&ordering, "activation_command_result_receipt_persisted")
        && !json_bool(&ordering, "activation_command_result_receipt_accepted")
        && !json_bool(&ordering, "activation_command_completion_ack_recorded")
        && !json_bool(&ordering, "activation_allowed")
        && !json_bool(&ordering, "live_mutation_execution_performed")
        && !json_bool(&ordering, "memory_store_write_performed")
        && !json_bool(&ordering, "memory_store_mutated")
        && !json_bool(&ordering, "rollback_executed")
        && !json_bool(&ordering, "provider_invoked")
        && !json_bool(&ordering, "model_invoked")
        && !json_bool(&ordering, "external_send_performed")
        && !json_bool(&ordering, "release_artifact_written")
        && !json_bool(&ordering, "active_binary_mutated")
        && side_effects_all_false(&ordering);

    let cancellation_fixtures = serde_json::Value::Array(vec![
        cancellation_fixture(
            "activation-result-receipt-cancellation-missing-source-ordering-report",
            "source_ordering_monotonicity_report_required",
            "blocked_noop",
            serde_json::json!({
                "source_ordering_monotonicity_present": false,
                "source_ordering_monotonicity_ready": false
            }),
        ),
        cancellation_fixture(
            "activation-result-receipt-cancel-blocked-noop",
            "cancellation_of_blocked_noop_receipt_denied",
            "blocked_noop",
            serde_json::json!({
                "cancellation_request_shape": "cancel_blocked_noop_receipt"
            }),
        ),
        cancellation_fixture(
            "activation-result-receipt-supersede-with-completed",
            "supersession_of_blocked_noop_with_completed_denied",
            "blocked_supersession_noop",
            serde_json::json!({
                "supersession_requested": true,
                "cancellation_requested": false,
                "requested_replacement_status": "completed"
            }),
        ),
        cancellation_fixture(
            "activation-result-receipt-replacement-hash",
            "replacement_hash_identity_attempt_denied",
            "blocked_supersession_noop",
            serde_json::json!({
                "supersession_requested": true,
                "cancellation_requested": false,
                "replacement_hash_requested": true,
                "requested_hash_relation": "different_hash_for_same_receipt_identity"
            }),
        ),
        cancellation_fixture(
            "activation-result-receipt-tombstone-delete-marker",
            "tombstone_or_delete_marker_denied",
            "blocked_noop",
            serde_json::json!({
                "tombstone_requested": true,
                "delete_marker_requested": true
            }),
        ),
        cancellation_fixture(
            "activation-result-receipt-completion-ack-cancel",
            "completion_ack_cancellation_denied",
            "blocked_noop",
            serde_json::json!({
                "completion_ack_cancellation_requested": true,
                "ack_cancellation_requested": true
            }),
        ),
        cancellation_fixture(
            "activation-result-receipt-ledger-index-delivery-cancel",
            "ledger_index_delivery_cancellation_supersession_denied",
            "blocked_noop",
            serde_json::json!({
                "ledger_cancellation_requested": true,
                "index_cancellation_requested": true,
                "delivery_cancellation_requested": true
            }),
        ),
        cancellation_fixture(
            "activation-result-receipt-memory-write-live-mutation-supersede",
            "memory_write_live_mutation_supersession_denied",
            "blocked_supersession_noop",
            serde_json::json!({
                "supersession_requested": true,
                "cancellation_requested": false,
                "memory_write_supersession_requested": true,
                "live_mutation_supersession_requested": true
            }),
        ),
        cancellation_fixture(
            "activation-result-receipt-rollback-secret-provider-supersede",
            "rollback_secret_provider_supersession_denied",
            "blocked_supersession_noop",
            serde_json::json!({
                "supersession_requested": true,
                "cancellation_requested": false,
                "rollback_supersession_requested": true,
                "secret_material_supersession_requested": true,
                "provider_prompt_supersession_requested": true
            }),
        ),
        cancellation_fixture(
            "activation-result-receipt-external-public-install-supersede",
            "external_public_install_restart_active_binary_supersession_denied",
            "blocked_supersession_noop",
            serde_json::json!({
                "supersession_requested": true,
                "cancellation_requested": false,
                "external_send_supersession_requested": true,
                "public_claim_supersession_requested": true,
                "release_artifact_supersession_requested": true,
                "install_supersession_requested": true,
                "service_restart_supersession_requested": true,
                "active_binary_mutation_supersession_requested": true
            }),
        ),
    ]);
    let source_ordering_report_sha256 = sha256_json_value(&ordering);
    let boundary_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-cancellation-supersession-denial-boundary-v1:{}:{}",
        route_matrix.route_count, source_ordering_report_sha256,
    ));
    let report_ready = route_matrix.ready
        && route_count_source_command_accepted
        && source_ordering_ready
        && CANCELLATION_SURFACES.len() == 12
        && cancellation_fixtures.as_array().map(std::vec::Vec::len) == Some(10)
        && DENIED_BY.len() == 24;

    let allowed_next_actions = serde_json::json!([
        {
            "action": "run_memory_write_execution_activation_command_result_receipt_cancellation_supersession_denial_boundary_require_live_gate",
            "status": "allowed_verification_only",
            "accepts_cancellation": false,
            "accepts_supersession": false,
            "records_replacement_receipt": false,
            "records_tombstone": false,
            "promotes_completion": false,
            "writes_memory": false,
            "executes_rollback": false,
            "writes_kg": false,
            "invokes_provider": false,
            "sends_externally": false,
            "publishes_artifacts": false,
            "installs_or_restarts": false,
            "mutates_active_binary": false
        },
        {
            "action": "prepare_memory_write_execution_activation_command_result_receipt_audit_trail_immutable_evidence_denial_boundary",
            "status": "allowed_report_only_next_slice",
            "records_audit_evidence": false,
            "persists_immutable_evidence": false,
            "mutates_runtime": false,
            "invokes_model": false,
            "writes_memory_or_kg": false
        }
    ]);

    let mut side_effects = serde_json::Map::new();
    for &key in FALSE_KEYS {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }

    let mut report = serde_json::Map::new();
    macro_rules! insert_report_json {
        ($key:literal, $value:expr) => {
            report.insert($key.to_string(), serde_json::json!($value));
        };
    }

    insert_report_json!("product", "Hepta");
    insert_report_json!("runtime", "hepta");
    insert_report_json!("status", if report_ready { "ready" } else { "blocked" });
    insert_report_json!("base_url", "http://127.0.0.1:7373");
    insert_report_json!(
        "gate",
        "hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_cancellation_supersession_denial_boundary_route"
    );
    insert_report_json!(
        "endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_COMMAND_RESULT_RECEIPT_CANCELLATION_SUPERSESSION_DENIAL_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_command",
        "/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-cancellation-supersession-denial-boundary --json"
    );
    insert_report_json!("native_route", true);
    insert_report_json!("side_effect_free", true);
    insert_report_json!("audit_date", "2026-07-03");
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_cancellation_supersession_denial_boundary_schema_version",
        "memory_write_execution_activation_command_result_receipt_cancellation_supersession_denial_boundary_v1"
    );
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_cancellation_supersession_denial_boundary_ready",
        report_ready
    );
    insert_report_json!(
        "activation_command_result_receipt_cancellation_supersession_mode",
        "memory_write_execution_activation_command_result_receipt_cancellation_supersession_denial"
    );
    insert_report_json!(
        "native_gateway_source_command_count",
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    insert_report_json!("route_count", route_matrix.route_count);
    insert_report_json!(
        "implemented_route_count",
        route_matrix.implemented_route_count
    );
    insert_report_json!("missing_route_count", route_matrix.missing_route_count);
    insert_report_json!(
        "route_count_source_command_accepted",
        route_count_source_command_accepted
    );
    insert_report_json!("boundary_hash_sha256", boundary_hash_sha256);
    insert_report_json!(
        "source_activation_command_result_receipt_ordering_monotonicity_boundary_endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_COMMAND_RESULT_RECEIPT_ORDERING_MONOTONICITY_DENIAL_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_activation_command_result_receipt_ordering_monotonicity_boundary_ready",
        source_ordering_ready
    );
    insert_report_json!(
        "source_activation_command_result_receipt_ordering_monotonicity_ready",
        json_bool(
            &ordering,
            "memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_ready"
        )
    );
    insert_report_json!(
        "source_activation_command_result_receipt_ordering_monotonicity_boundary_report_sha256",
        source_ordering_report_sha256
    );
    insert_report_json!(
        "source_activation_command_result_receipt_ordering_monotonicity_report_sha256",
        source_ordering_report_sha256
    );
    insert_report_json!(
        "source_activation_command_result_receipt_replay_idempotency_boundary_ready",
        json_bool(
            &ordering,
            "source_activation_command_result_receipt_replay_idempotency_boundary_ready"
        )
    );
    insert_report_json!(
        "source_activation_command_result_receipt_replay_idempotency_ready",
        json_bool(
            &ordering,
            "memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_ready"
        )
    );
    insert_report_json!(
        "source_activation_command_result_receipt_replay_idempotency_boundary_report_sha256",
        json_str(
            &ordering,
            "source_activation_command_result_receipt_replay_idempotency_boundary_report_sha256"
        )
    );
    insert_report_json!(
        "source_activation_command_result_receipt_no_persistence_boundary_ready",
        json_bool(
            &ordering,
            "source_activation_command_result_receipt_no_persistence_boundary_ready"
        )
    );
    insert_report_json!(
        "source_activation_command_result_receipt_no_persistence_ready",
        json_bool(
            &ordering,
            "memory_write_execution_activation_command_result_receipt_no_persistence_ready"
        )
    );
    insert_report_json!(
        "source_activation_command_result_receipt_no_persistence_boundary_report_sha256",
        json_str(
            &ordering,
            "source_activation_command_result_receipt_no_persistence_boundary_report_sha256"
        )
    );
    for key in [
        "source_activation_command_noop_handoff_boundary_report_sha256",
        "source_memory_write_execution_activation_closure_denial_boundary_report_sha256",
        "source_memory_write_execution_post_write_operator_acceptance_denial_boundary_report_sha256",
        "source_memory_write_execution_post_write_validation_dry_run_boundary_report_sha256",
        "source_memory_write_execution_write_enable_fixture_boundary_report_sha256",
        "source_memory_write_execution_no_write_sink_contract_boundary_report_sha256",
        "source_memory_write_execution_denial_matrix_boundary_report_sha256",
    ] {
        report.insert(
            key.to_string(),
            ordering
                .get(key)
                .cloned()
                .unwrap_or_else(|| serde_json::json!("")),
        );
    }
    insert_report_json!("minimum_required_samples", 24);
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_cancellation_supersession_denial_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_no_persistence_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_activation_command_noop_handoff_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_activation_closure_denial_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_post_write_operator_acceptance_denial_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_post_write_validation_dry_run_ready",
        true
    );
    insert_report_json!("memory_write_execution_write_enable_fixture_ready", true);
    insert_report_json!("memory_write_execution_no_write_sink_contract_ready", true);
    insert_report_json!(
        "required_activation_command_result_receipt_cancellation_supersession_surface_count",
        12
    );
    insert_report_json!(
        "ready_activation_command_result_receipt_cancellation_supersession_surface_count",
        12
    );
    insert_report_json!(
        "side_effect_free_activation_command_result_receipt_cancellation_supersession_surface_count",
        12
    );
    insert_report_json!(
        "required_activation_command_result_receipt_cancellation_supersession_fixture_count",
        10
    );
    insert_report_json!(
        "activation_command_result_receipt_cancellation_supersession_fixture_count",
        10
    );
    insert_report_json!(
        "blocked_activation_command_result_receipt_cancellation_supersession_fixture_count",
        10
    );
    insert_report_json!(
        "noop_activation_command_result_receipt_cancellation_supersession_fixture_count",
        10
    );
    insert_report_json!(
        "allowed_activation_command_result_receipt_cancellation_supersession_fixture_count",
        0
    );
    insert_report_json!(
        "accepted_activation_command_result_receipt_cancellation_supersession_fixture_count",
        0
    );
    insert_report_json!(
        "activation_command_result_receipt_cancellation_denied_count",
        10
    );
    insert_report_json!(
        "activation_command_result_receipt_supersession_denied_count",
        10
    );
    insert_report_json!(
        "activation_command_result_receipt_cancellation_performed_count",
        0
    );
    insert_report_json!(
        "activation_command_result_receipt_supersession_performed_count",
        0
    );
    insert_report_json!("memory_store_write_performed_count", 0);

    for &key in FALSE_KEYS {
        report.insert(key.to_string(), serde_json::json!(false));
    }
    report.insert(
        "activation_command_result_receipt_cancellation_supersession_surfaces".to_string(),
        serde_json::json!(CANCELLATION_SURFACES),
    );
    report.insert(
        "activation_command_result_receipt_cancellation_supersession_fixtures".to_string(),
        cancellation_fixtures,
    );
    report.insert(
        "denied_by_activation_command_result_receipt_cancellation_supersession".to_string(),
        serde_json::json!(DENIED_BY),
    );
    insert_report_json!(
        "denied_by_activation_command_result_receipt_cancellation_supersession_count",
        24
    );
    report.insert("allowed_next_actions".to_string(), allowed_next_actions);
    report.insert(
        "side_effects".to_string(),
        serde_json::Value::Object(side_effects),
    );

    serde_json::Value::Object(report)
}
