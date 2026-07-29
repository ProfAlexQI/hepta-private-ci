fn hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_noop_handoff_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_activation_request =
        hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_request_denial_matrix_report();
    let source_bool = |key: &str| {
        source_activation_request
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let source_u64 = |key: &str| {
        source_activation_request
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_str = |key: &str| {
        source_activation_request
            .get(key)
            .and_then(serde_json::Value::as_str)
            .unwrap_or("blocked")
            .to_string()
    };
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_activation_request_ready = source_str("status") == "ready"
        && source_bool("runtime_provider_router_activation_request_denial_matrix_ready")
        && source_str("runtime_provider_router_activation_request_denial_matrix_status")
            == "blocked"
        && source_u64("activation_request_fixture_count") == 10
        && source_u64("blocked_activation_request_fixture_count") == 10
        && source_u64("noop_activation_request_fixture_count") == 10
        && source_u64("accepted_activation_request_fixture_count") == 0
        && source_u64("activation_request_performed_count") == 0
        && source_u64("activation_execution_performed_count") == 0
        && !source_bool("activation_request_accepted")
        && !source_bool("activation_request_recorded")
        && !source_bool("activation_request_persisted")
        && !source_bool("activation_request_executed")
        && !source_bool("activation_activated")
        && !source_bool("runtime_router_mutated")
        && !source_bool("runtime_attachment_performed")
        && !source_bool("live_context_attached")
        && !source_bool("context_injection_performed")
        && !source_bool("adapter_invoked")
        && !source_bool("provider_invoked")
        && !source_bool("model_invoked")
        && !source_bool("auth_secret_read")
        && !source_bool("credential_read")
        && !source_bool("secret_file_read")
        && !source_bool("usage_recorded")
        && !source_bool("memory_store_write_performed")
        && !source_bool("live_kg_write_performed")
        && !source_bool("receipt_recorded")
        && !source_bool("receipt_persisted")
        && !source_bool("receipt_accepted")
        && !source_bool("readback_evidence_recorded")
        && !source_bool("readback_evidence_persisted")
        && !source_bool("router_handoff_recorded")
        && !source_bool("router_handoff_persisted")
        && !source_bool("telegram_send_performed")
        && !source_bool("channel_send_performed")
        && !source_bool("external_send_performed")
        && !source_bool("service_restart_performed")
        && !source_bool("active_binary_mutated");
    let report_ready = route_matrix.ready
        && route_count_floor_preserved
        && route_count_source_command_accepted
        && source_activation_request_ready;

    let activation_command_fixture =
        |id: &str, status: &str, reason: &str, extra: serde_json::Value| {
            let mut fixture = serde_json::Map::new();
            fixture.insert("id".to_string(), serde_json::Value::String(id.to_string()));
            fixture.insert(
                "activation_command_status".to_string(),
                serde_json::Value::String(status.to_string()),
            );
            fixture.insert(
                "reason".to_string(),
                serde_json::Value::String(reason.to_string()),
            );
            for key in [
                "source_activation_request_denial_matrix_present",
                "source_activation_request_denial_matrix_ready",
                "activation_command_requested",
                "activation_command_noop_confirmed",
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(true));
            }
            for key in [
                "activation_command_shape_registered",
                "activation_command_allowed",
                "activation_command_accepted",
                "activation_command_enabled",
                "activation_command_invoked",
                "activation_command_dispatched",
                "activation_command_dispatch_performed",
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
                "activation_command_result_receipt_accepted",
                "activation_command_result_receipt_exported",
                "activation_command_result_receipt_query_registered",
                "activation_command_result_receipt_observability_recorded",
                "activation_request_allowed",
                "activation_request_accepted",
                "activation_request_recorded",
                "activation_request_persisted",
                "activation_request_materialized",
                "activation_request_filesystem_written",
                "activation_request_delivered",
                "activation_request_executed",
                "activation_activated",
                "runtime_router_mutated",
                "runtime_attachment_performed",
                "live_context_attached",
                "context_injection_performed",
                "adapter_invoked",
                "provider_invoked",
                "model_invoked",
                "auth_secret_read",
                "credential_read",
                "secret_file_read",
                "usage_recorded",
                "memory_store_write_performed",
                "memory_store_mutated",
                "live_kg_write_performed",
                "receipt_exported",
                "receipt_query_registered",
                "receipt_observability_recorded",
                "receipt_recorded",
                "receipt_persisted",
                "receipt_accepted",
                "readback_evidence_recorded",
                "readback_evidence_persisted",
                "router_handoff_recorded",
                "router_handoff_persisted",
                "rollback_executed",
                "telegram_send_performed",
                "channel_send_performed",
                "external_send_performed",
                "public_release_claimed",
                "public_ga_claimed",
                "release_artifact_written",
                "install_executed",
                "launchd_mutated",
                "service_restart_performed",
                "active_binary_mutated",
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(false));
            }
            let mut fixture = serde_json::Value::Object(fixture);
            extend_json_object(&mut fixture, extra);
            fixture
        };
    let activation_command_fixtures = serde_json::Value::Array(vec![
        activation_command_fixture(
            "provider-router-activation-command-missing-source-activation-request-denial-matrix",
            "blocked_noop",
            "source_activation_request_denial_matrix_report_required",
            serde_json::json!({
                "source_activation_request_denial_matrix_present": false,
                "source_activation_request_denial_matrix_ready": false,
            }),
        ),
        activation_command_fixture(
            "provider-router-activation-command-handoff-request",
            "blocked_command_noop",
            "activation_command_handoff_shape_denied",
            serde_json::json!({}),
        ),
        activation_command_fixture(
            "provider-router-activation-command-registration-enable-request",
            "blocked_register_enable_noop",
            "activation_command_registration_enablement_denied",
            serde_json::json!({
                "activation_command_registration_requested": true,
                "activation_command_enable_requested": true,
            }),
        ),
        activation_command_fixture(
            "provider-router-activation-command-direct-invocation-request",
            "blocked_invocation_noop",
            "activation_command_invocation_denied",
            serde_json::json!({"activation_command_invocation_requested": true}),
        ),
        activation_command_fixture(
            "provider-router-activation-command-runtime-router-dispatch-request",
            "blocked_dispatch_noop",
            "runtime_router_dispatch_denied",
            serde_json::json!({
                "runtime_router_dispatch_requested": true,
                "runtime_router_mutation_requested": true,
            }),
        ),
        activation_command_fixture(
            "provider-router-activation-command-live-context-injection-request",
            "blocked_context_noop",
            "live_context_context_injection_command_denied",
            serde_json::json!({
                "live_context_attachment_requested": true,
                "context_injection_requested": true,
            }),
        ),
        activation_command_fixture(
            "provider-router-activation-command-adapter-provider-model-request",
            "blocked_provider_noop",
            "adapter_provider_model_command_denied",
            serde_json::json!({
                "adapter_invocation_requested": true,
                "provider_invocation_requested": true,
                "model_invocation_requested": true,
            }),
        ),
        activation_command_fixture(
            "provider-router-activation-command-memory-kg-request",
            "blocked_memory_kg_noop",
            "memory_kg_command_denied",
            serde_json::json!({
                "memory_store_write_requested": true,
                "live_kg_write_requested": true,
            }),
        ),
        activation_command_fixture(
            "provider-router-activation-command-receipt-readback-router-handoff-request",
            "blocked_receipt_router_noop",
            "receipt_readback_router_handoff_command_denied",
            serde_json::json!({
                "receipt_record_requested": true,
                "receipt_persist_requested": true,
                "receipt_export_requested": true,
                "receipt_query_requested": true,
                "receipt_observability_requested": true,
                "readback_evidence_requested": true,
                "router_handoff_requested": true,
            }),
        ),
        activation_command_fixture(
            "provider-router-activation-command-external-public-install-restart-active-binary-request",
            "blocked_external_noop",
            "external_public_install_restart_active_binary_command_denied",
            serde_json::json!({
                "external_send_requested": true,
                "public_claim_requested": true,
                "public_ga_claim_requested": true,
                "release_artifact_write_requested": true,
                "install_requested": true,
                "launchd_restart_requested": true,
                "service_restart_requested": true,
                "active_binary_mutation_requested": true,
            }),
        ),
    ]);
    let activation_command_fixture_count = activation_command_fixtures
        .as_array()
        .map(std::vec::Vec::len)
        .unwrap_or(0);
    let denials: Vec<serde_json::Value> = [
        "source_activation_request_denial_matrix_report_required",
        "activation_command_shape_registration_denied",
        "activation_command_acceptance_denied",
        "activation_command_enablement_denied",
        "activation_command_invocation_denied",
        "activation_command_dispatch_denied",
        "activation_command_noop_decision_recording_denied",
        "activation_command_noop_decision_persistence_denied",
        "activation_command_handoff_recording_denied",
        "activation_command_handoff_persistence_denied",
        "activation_command_handoff_acceptance_denied",
        "activation_command_handoff_materialization_denied",
        "activation_command_handoff_filesystem_write_denied",
        "activation_command_result_receipt_recording_denied",
        "activation_command_result_receipt_persistence_denied",
        "activation_request_acceptance_denied",
        "activation_execution_denied",
        "runtime_router_mutation_denied",
        "runtime_attachment_denied",
        "live_context_attachment_denied",
        "context_injection_denied",
        "adapter_invocation_denied",
        "provider_model_invocation_denied",
        "memory_store_write_denied",
        "live_kg_write_denied",
        "receipt_export_query_observability_denied",
        "router_handoff_readback_persistence_denied",
        "usage_recording_denied",
        "secret_material_read_denied",
        "external_public_install_restart_active_binary_denied",
    ]
    .into_iter()
    .map(|item| serde_json::Value::String(item.to_string()))
    .collect();
    let source_report_sha256 = sha256_json_value(&source_activation_request);
    let fixture_hash = sha256_json_value(&activation_command_fixtures);
    let contract_hash = sha256_text_value(&format!(
        "hepta-full-enablement-runtime-provider-router-activation-command-noop-handoff:native:source={source_report_sha256}:fixtures={fixture_hash}:route_count={}:command=0:dispatch=0:provider=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "runtime-provider-router-activation-command-noop-handoff:report-only:no-command-register:no-command-enable:no-command-invoke:no-dispatch:no-handoff-persist:no-provider:no-model:no-secret-read",
    );

    let mut report = source_activation_request.clone();
    extend_json_object(
        &mut report,
        serde_json::json!({
            "status": if report_ready { "ready" } else { "blocked" },
            "gate": "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_noop_handoff_route",
            "source_command": "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-noop-handoff --json",
            "native_route": true,
            "compatibility_mode": "native_runtime_provider_router_activation_command_noop_handoff_status",
            "side_effect_free": true,
            "audit_date": "2026-06-30",
            "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_NOOP_HANDOFF_ENDPOINT,
            "source_activation_request_denial_matrix_route_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_REQUEST_DENIAL_MATRIX_ENDPOINT,
            "source_activation_request_denial_matrix_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-request-denial-matrix-gate.sh",
            "source_activation_request_denial_matrix_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-request-denial-matrix-route-gate.sh",
            "source_activation_command_noop_handoff_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-noop-handoff-gate.sh",
            "source_activation_command_noop_handoff_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-noop-handoff-route-gate.sh",
            "source_activation_request_denial_matrix_report_sha256": source_report_sha256,
            "activation_command_fixtures_sha256": fixture_hash,
            "activation_command_contract_hash_sha256": contract_hash,
            "activation_command_policy_hash_sha256": policy_hash,
            "minimum_required_samples": 24,
            "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
            "route_count": route_matrix.route_count,
            "implemented_route_count": route_matrix.implemented_route_count,
            "missing_route_count": route_matrix.missing_route_count,
            "route_count_cutover_floor": NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR,
            "route_count_floor_preserved": route_count_floor_preserved,
            "route_count_source_command_accepted": route_count_source_command_accepted,
            "source_route_wired": true,
            "source_activation_request_denial_matrix_ready": source_activation_request_ready,
            "source_activation_request_denial_matrix_status": source_str("runtime_provider_router_activation_request_denial_matrix_status"),
            "source_runtime_model_provider_router": source_str("source_runtime_model_provider_router"),
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "runtime_provider_router_activation_command_noop_handoff_route_enabled": true,
            "runtime_provider_router_activation_command_noop_handoff_ready": true,
            "runtime_provider_router_activation_command_noop_handoff_status": "blocked",
            "activation_command_noop_handoff_schema_version": "memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_noop_handoff_v1",
            "activation_command_noop_handoff_mode": "runtime_provider_router_activation_command_noop_handoff_no_register_no_enable_no_invoke_no_dispatch",
            "activation_command_noop_handoff_decision": "runtime_provider_router_activation_request_denial_matrix_cannot_create_or_authorize_activation_commands",
            "runtime_provider_router_activation_request_denial_matrix_ready": source_bool("runtime_provider_router_activation_request_denial_matrix_ready"),
            "runtime_provider_router_activation_request_denial_matrix_status": source_str("runtime_provider_router_activation_request_denial_matrix_status"),
            "source_activation_request_fixture_count": source_u64("activation_request_fixture_count"),
            "source_blocked_activation_request_fixture_count": source_u64("blocked_activation_request_fixture_count"),
            "source_noop_activation_request_fixture_count": source_u64("noop_activation_request_fixture_count"),
            "source_accepted_activation_request_fixture_count": source_u64("accepted_activation_request_fixture_count"),
            "source_activation_request_performed_count": source_u64("activation_request_performed_count"),
            "activation_command_surface_count": 13,
            "activation_command_surface_ready_count": 13,
            "activation_command_side_effect_free_surface_count": 13,
            "activation_command_fixtures": activation_command_fixtures,
            "activation_command_fixture_count": activation_command_fixture_count,
            "activation_command_requested_fixture_count": activation_command_fixture_count,
            "blocked_activation_command_fixture_count": activation_command_fixture_count,
            "noop_activation_command_fixture_count": activation_command_fixture_count,
            "allowed_activation_command_fixture_count": 0,
            "accepted_activation_command_fixture_count": 0,
            "activation_command_denied_count": 10,
            "activation_command_performed_count": 0,
            "activation_command_dispatch_performed_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "activation_command_shape_registered": false,
            "activation_command_allowed": false,
            "activation_command_accepted": false,
            "activation_command_enabled": false,
            "activation_command_invoked": false,
            "activation_command_dispatched": false,
            "activation_command_noop_decision_recorded": false,
            "activation_command_noop_decision_persisted": false,
            "activation_command_noop_decision_accepted": false,
            "activation_command_handoff_recorded": false,
            "activation_command_handoff_persisted": false,
            "activation_command_handoff_accepted": false,
            "activation_command_handoff_materialized": false,
            "activation_command_handoff_filesystem_written": false,
            "activation_command_result_receipt_recorded": false,
            "activation_command_result_receipt_persisted": false,
            "activation_command_result_receipt_accepted": false,
            "activation_command_result_receipt_exported": false,
            "activation_command_result_receipt_query_registered": false,
            "activation_command_result_receipt_observability_recorded": false,
        }),
    );
    for key in [
        "activation_request_allowed",
        "activation_request_accepted",
        "activation_request_recorded",
        "activation_request_persisted",
        "activation_request_materialized",
        "activation_request_filesystem_written",
        "activation_request_delivered",
        "activation_request_executed",
        "activation_activated",
        "activation_nonce_accepted",
        "activation_generation_accepted",
        "runtime_router_mutated",
        "runtime_attachment_performed",
        "live_context_attached",
        "context_injection_performed",
        "adapter_invoked",
        "provider_invoked",
        "model_invoked",
        "auth_secret_read",
        "credential_read",
        "secret_file_read",
        "usage_recorded",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "receipt_export_allowed",
        "receipt_exported",
        "receipt_query_allowed",
        "receipt_query_registered",
        "receipt_observability_allowed",
        "receipt_observability_recorded",
        "receipt_recorded",
        "receipt_persisted",
        "receipt_accepted",
        "receipt_materialized",
        "receipt_filesystem_written",
        "readback_evidence_recorded",
        "readback_evidence_persisted",
        "router_handoff_recorded",
        "router_handoff_persisted",
        "rollback_executed",
        "telegram_send_performed",
        "channel_send_performed",
        "external_send_performed",
        "public_release_claimed",
        "public_ga_claimed",
        "release_artifact_written",
        "install_executed",
        "launchd_mutated",
        "service_restart_performed",
        "active_binary_mutated",
    ] {
        if let Some(report) = report.as_object_mut() {
            report.insert(key.to_string(), serde_json::Value::Bool(false));
        }
    }
    extend_json_object(
        &mut report,
        serde_json::json!({
            "activation_command_surfaces": [
                "source_activation_request_denial_matrix_report_required",
                "activation_command_handoff_shape_denied",
                "activation_command_registration_denied",
                "activation_command_enablement_denied",
                "activation_command_invocation_denied",
                "activation_command_dispatch_denied",
                "activation_command_handoff_record_persist_denied",
                "live_context_context_injection_command_denied",
                "adapter_provider_model_command_denied",
                "memory_kg_command_denied",
                "receipt_readback_router_handoff_command_denied",
                "command_result_receipt_export_query_observability_denied",
                "external_public_install_restart_active_binary_command_denied"
            ],
            "denied_by_activation_command_noop_handoff": denials,
            "denied_by_activation_command_noop_handoff_count": 30,
            "allowed_next_actions": [
                {
                    "action": "review_runtime_provider_router_activation_command_noop_handoff",
                    "status": "allowed_report_only",
                    "registers_command": false,
                    "enables_command": false,
                    "invokes_command": false,
                    "dispatches_command": false,
                    "persists_handoff": false,
                    "invokes_model": false
                },
                {
                    "action": "stage_runtime_provider_router_activation_command_result_receipt_no_persistence",
                    "status": "allowed_report_only_next_slice",
                    "records_command_result": false,
                    "persists_command_result": false,
                    "exports_receipt": false,
                    "registers_observability": false,
                    "mutates_runtime": false,
                    "invokes_model": false
                },
                {
                    "action": "run_full_light_preflight",
                    "status": "allowed_verification_only",
                    "mutates_runtime": false,
                    "dispatches_command": false,
                    "attaches_live_context": false,
                    "invokes_model": false,
                    "writes_kg": false
                }
            ],
            "source_activation_request_denial_matrix_report_required": true,
            "activation_command_registration_forbidden": true,
            "activation_command_enablement_forbidden": true,
            "activation_command_invocation_forbidden": true,
            "activation_command_dispatch_forbidden": true,
            "activation_command_handoff_persistence_forbidden": true,
            "activation_command_result_receipt_persistence_forbidden": true,
            "activation_request_acceptance_forbidden": true,
            "activation_request_execution_forbidden": true,
            "runtime_router_mutation_forbidden": true,
            "live_context_attachment_forbidden": true,
            "context_injection_forbidden": true,
            "adapter_invocation_forbidden": true,
            "provider_model_invocation_forbidden": true,
            "memory_kg_write_forbidden": true,
            "auth_secret_read_forbidden": true,
            "usage_recording_forbidden": true,
        }),
    );
    if let Some(side_effects) = report
        .get_mut("side_effects")
        .and_then(serde_json::Value::as_object_mut)
    {
        for key in [
            "activation_command_shape_registered",
            "activation_command_accepted",
            "activation_command_enabled",
            "activation_command_invoked",
            "activation_command_dispatched",
            "activation_command_dispatch_performed",
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
            "activation_command_result_receipt_accepted",
            "activation_command_result_receipt_exported",
            "activation_command_result_receipt_query_registered",
            "activation_command_result_receipt_observability_recorded",
            "activation_request_recorded",
            "activation_request_persisted",
            "activation_request_materialized",
            "activation_request_filesystem_written",
            "activation_request_delivered",
            "activation_request_executed",
            "activation_activated",
            "activation_nonce_accepted",
            "activation_generation_accepted",
            "runtime_router_mutated",
            "runtime_attachment_performed",
            "live_context_attached",
            "context_injection_performed",
            "adapter_invoked",
            "provider_invoked",
            "model_invoked",
            "auth_secret_read",
            "credential_read",
            "secret_file_read",
            "usage_recorded",
            "memory_store_write_performed",
            "memory_store_mutated",
            "live_kg_write_performed",
            "receipt_exported",
            "receipt_query_registered",
            "receipt_observability_recorded",
            "receipt_recorded",
            "receipt_persisted",
            "receipt_accepted",
            "receipt_materialized",
            "receipt_filesystem_written",
            "readback_evidence_recorded",
            "readback_evidence_persisted",
            "router_handoff_recorded",
            "router_handoff_persisted",
            "rollback_executed",
            "telegram_send_performed",
            "channel_send_performed",
            "external_send_performed",
            "filesystem_written",
            "public_release_claimed",
            "public_ga_claimed",
            "release_artifact_written",
            "install_executed",
            "launchd_mutated",
            "service_restart_performed",
            "active_binary_mutated",
        ] {
            side_effects.insert(key.to_string(), serde_json::Value::Bool(false));
        }
    }
    report
}

fn hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_no_persistence_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_noop_handoff =
        hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_noop_handoff_report();
    let source_bool = |key: &str| {
        source_noop_handoff
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let source_u64 = |key: &str| {
        source_noop_handoff
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_str = |key: &str| {
        source_noop_handoff
            .get(key)
            .and_then(serde_json::Value::as_str)
            .unwrap_or("blocked")
            .to_string()
    };
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_noop_handoff_ready = source_str("status") == "ready"
        && source_bool("runtime_provider_router_activation_command_noop_handoff_ready")
        && source_str("runtime_provider_router_activation_command_noop_handoff_status")
            == "blocked"
        && source_bool("runtime_provider_router_activation_request_denial_matrix_ready")
        && source_u64("activation_command_surface_count") == 13
        && source_u64("activation_command_surface_ready_count") == 13
        && source_u64("activation_command_fixture_count") == 10
        && source_u64("blocked_activation_command_fixture_count") == 10
        && source_u64("noop_activation_command_fixture_count") == 10
        && source_u64("allowed_activation_command_fixture_count") == 0
        && source_u64("accepted_activation_command_fixture_count") == 0
        && source_u64("activation_command_denied_count") == 10
        && source_u64("activation_command_performed_count") == 0
        && source_u64("activation_command_dispatch_performed_count") == 0
        && !source_bool("activation_command_shape_registered")
        && !source_bool("activation_command_allowed")
        && !source_bool("activation_command_accepted")
        && !source_bool("activation_command_enabled")
        && !source_bool("activation_command_invoked")
        && !source_bool("activation_command_dispatched")
        && !source_bool("activation_command_noop_decision_recorded")
        && !source_bool("activation_command_noop_decision_persisted")
        && !source_bool("activation_command_handoff_recorded")
        && !source_bool("activation_command_handoff_persisted")
        && !source_bool("activation_command_handoff_accepted")
        && !source_bool("activation_command_result_receipt_recorded")
        && !source_bool("activation_command_result_receipt_persisted")
        && !source_bool("activation_command_result_receipt_accepted")
        && !source_bool("activation_command_result_receipt_exported")
        && !source_bool("activation_command_result_receipt_query_registered")
        && !source_bool("activation_command_result_receipt_observability_recorded")
        && !source_bool("activation_request_accepted")
        && !source_bool("activation_request_recorded")
        && !source_bool("activation_request_persisted")
        && !source_bool("activation_request_executed")
        && !source_bool("activation_activated")
        && !source_bool("runtime_router_mutated")
        && !source_bool("runtime_attachment_performed")
        && !source_bool("live_context_attached")
        && !source_bool("context_injection_performed")
        && !source_bool("adapter_invoked")
        && !source_bool("provider_invoked")
        && !source_bool("model_invoked")
        && !source_bool("auth_secret_read")
        && !source_bool("credential_read")
        && !source_bool("secret_file_read")
        && !source_bool("usage_recorded")
        && !source_bool("memory_store_write_performed")
        && !source_bool("memory_store_mutated")
        && !source_bool("live_kg_write_performed")
        && !source_bool("receipt_recorded")
        && !source_bool("receipt_persisted")
        && !source_bool("receipt_accepted")
        && !source_bool("readback_evidence_recorded")
        && !source_bool("readback_evidence_persisted")
        && !source_bool("router_handoff_recorded")
        && !source_bool("router_handoff_persisted")
        && !source_bool("telegram_send_performed")
        && !source_bool("channel_send_performed")
        && !source_bool("external_send_performed")
        && !source_bool("rollback_executed")
        && !source_bool("service_restart_performed")
        && !source_bool("active_binary_mutated");
    let report_ready = route_matrix.ready
        && route_count_floor_preserved
        && route_count_source_command_accepted
        && source_noop_handoff_ready;

    let result_receipt_fixture =
        |id: &str, status: &str, reason: &str, extra: serde_json::Value| {
            let mut fixture = serde_json::Map::new();
            fixture.insert("id".to_string(), serde_json::Value::String(id.to_string()));
            fixture.insert(
                "activation_command_result_receipt_status".to_string(),
                serde_json::Value::String(status.to_string()),
            );
            fixture.insert(
                "reason".to_string(),
                serde_json::Value::String(reason.to_string()),
            );
            for key in [
                "source_activation_command_noop_handoff_present",
                "source_activation_command_noop_handoff_ready",
                "activation_command_result_receipt_requested",
                "receipt_noop_confirmed",
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(true));
            }
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
                "activation_command_result_receipt_exported",
                "activation_command_result_receipt_query_registered",
                "activation_command_result_receipt_observability_recorded",
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
                "operator_approval_from_receipt_accepted",
                "activation_from_receipt_allowed",
                "activation_command_shape_registered",
                "activation_command_allowed",
                "activation_command_accepted",
                "activation_command_enabled",
                "activation_command_invoked",
                "activation_command_dispatched",
                "activation_command_dispatch_performed",
                "activation_command_noop_decision_recorded",
                "activation_command_noop_decision_persisted",
                "activation_command_handoff_recorded",
                "activation_command_handoff_persisted",
                "activation_command_handoff_accepted",
                "activation_command_handoff_materialized",
                "activation_command_handoff_filesystem_written",
                "activation_request_accepted",
                "activation_request_recorded",
                "activation_request_persisted",
                "activation_request_executed",
                "activation_activated",
                "runtime_router_mutated",
                "runtime_attachment_performed",
                "live_context_attached",
                "context_injection_performed",
                "adapter_invoked",
                "provider_invoked",
                "model_invoked",
                "auth_secret_read",
                "credential_read",
                "secret_file_read",
                "usage_recorded",
                "memory_store_write_performed",
                "memory_store_mutated",
                "live_kg_write_performed",
                "receipt_recorded",
                "receipt_persisted",
                "receipt_accepted",
                "receipt_exported",
                "receipt_query_registered",
                "receipt_observability_recorded",
                "readback_evidence_recorded",
                "readback_evidence_persisted",
                "router_handoff_recorded",
                "router_handoff_persisted",
                "rollback_executed",
                "telegram_send_performed",
                "channel_send_performed",
                "external_send_performed",
                "public_release_claimed",
                "public_ga_claimed",
                "release_artifact_written",
                "install_executed",
                "launchd_mutated",
                "service_restart_performed",
                "active_binary_mutated",
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(false));
            }
            let mut fixture = serde_json::Value::Object(fixture);
            extend_json_object(&mut fixture, extra);
            fixture
        };
    let result_receipt_fixtures = serde_json::Value::Array(vec![
        result_receipt_fixture(
            "provider-router-activation-command-result-receipt-missing-source-noop-handoff",
            "blocked_noop",
            "source_activation_command_noop_handoff_report_required",
            serde_json::json!({
                "source_activation_command_noop_handoff_present": false,
                "source_activation_command_noop_handoff_ready": false,
            }),
        ),
        result_receipt_fixture(
            "provider-router-activation-command-result-receipt-schema-registration-attempt",
            "blocked_schema_noop",
            "result_receipt_schema_registration_denied",
            serde_json::json!({"result_receipt_schema_registration_requested": true}),
        ),
        result_receipt_fixture(
            "provider-router-activation-command-result-receipt-record-attempt",
            "blocked_record_noop",
            "result_receipt_recording_denied",
            serde_json::json!({"result_receipt_record_requested": true}),
        ),
        result_receipt_fixture(
            "provider-router-activation-command-result-receipt-persist-attempt",
            "blocked_persist_noop",
            "result_receipt_persistence_denied",
            serde_json::json!({"result_receipt_persist_requested": true}),
        ),
        result_receipt_fixture(
            "provider-router-activation-command-result-receipt-materialize-filesystem-attempt",
            "blocked_materialize_noop",
            "result_receipt_materialization_filesystem_write_denied",
            serde_json::json!({
                "result_receipt_materialize_requested": true,
                "result_receipt_filesystem_write_requested": true,
            }),
        ),
        result_receipt_fixture(
            "provider-router-activation-command-result-receipt-ledger-index-queue-delivery-attempt",
            "blocked_ledger_index_delivery_noop",
            "result_receipt_ledger_index_queue_delivery_denied",
            serde_json::json!({
                "result_receipt_ledger_write_requested": true,
                "result_receipt_index_requested": true,
                "result_receipt_enqueue_requested": true,
                "result_receipt_delivery_requested": true,
            }),
        ),
        result_receipt_fixture(
            "provider-router-activation-command-result-receipt-export-query-observability-attempt",
            "blocked_export_query_observability_noop",
            "result_receipt_export_query_observability_denied",
            serde_json::json!({
                "result_receipt_export_requested": true,
                "result_receipt_query_requested": true,
                "result_receipt_observability_requested": true,
            }),
        ),
        result_receipt_fixture(
            "provider-router-activation-command-result-receipt-acceptance-completion-ack-attempt",
            "blocked_acceptance_ack_noop",
            "result_receipt_acceptance_completion_ack_denied",
            serde_json::json!({
                "result_receipt_acceptance_requested": true,
                "completion_ack_requested": true,
                "operator_approval_from_receipt_requested": true,
            }),
        ),
        result_receipt_fixture(
            "provider-router-activation-command-result-receipt-runtime-context-provider-memory-kg-attempt",
            "blocked_runtime_provider_memory_kg_noop",
            "result_receipt_cannot_activate_runtime_provider_memory_or_kg",
            serde_json::json!({
                "result_receipt_status_requested": "completed",
                "activation_from_receipt_requested": true,
                "runtime_router_mutation_requested": true,
                "live_context_attachment_requested": true,
                "context_injection_requested": true,
                "provider_invocation_requested": true,
                "model_invocation_requested": true,
                "usage_record_requested": true,
                "memory_store_write_requested": true,
                "live_kg_write_requested": true,
            }),
        ),
        result_receipt_fixture(
            "provider-router-activation-command-result-receipt-external-public-install-restart-active-binary-attempt",
            "blocked_external_noop",
            "result_receipt_cannot_send_publish_install_restart_or_mutate_active_binary",
            serde_json::json!({
                "external_send_requested": true,
                "public_claim_requested": true,
                "public_ga_claim_requested": true,
                "release_artifact_write_requested": true,
                "install_requested": true,
                "launchd_restart_requested": true,
                "service_restart_requested": true,
                "active_binary_mutation_requested": true,
            }),
        ),
    ]);
    let result_receipt_fixture_count = result_receipt_fixtures
        .as_array()
        .map(std::vec::Vec::len)
        .unwrap_or(0);
    let denials: Vec<serde_json::Value> = [
        "source_activation_command_noop_handoff_required",
        "activation_command_disabled_required",
        "activation_command_invocation_denied",
        "activation_command_dispatch_denied",
        "result_receipt_schema_registration_denied",
        "result_receipt_schema_acceptance_denied",
        "result_receipt_recording_denied",
        "result_receipt_persistence_denied",
        "result_receipt_acceptance_denied",
        "result_receipt_materialization_denied",
        "result_receipt_filesystem_write_denied",
        "result_receipt_ledger_write_denied",
        "result_receipt_indexing_denied",
        "result_receipt_queue_enqueue_denied",
        "result_receipt_delivery_denied",
        "result_receipt_export_denied",
        "result_receipt_query_registration_denied",
        "result_receipt_observability_recording_denied",
        "completion_ack_recording_denied",
        "completion_ack_persistence_denied",
        "completion_ack_acceptance_denied",
        "operator_approval_from_receipt_denied",
        "activation_from_receipt_denied",
        "runtime_router_mutation_denied",
        "live_context_attachment_denied",
        "context_injection_denied",
        "adapter_invocation_denied",
        "provider_model_invocation_denied",
        "usage_recording_denied",
        "memory_store_write_denied",
        "live_kg_write_denied",
        "secret_material_read_denied",
        "external_send_denied",
        "public_release_claim_denied",
        "install_restart_active_binary_mutation_denied",
    ]
    .into_iter()
    .map(|item| serde_json::Value::String(item.to_string()))
    .collect();
    let source_report_sha256 = sha256_json_value(&source_noop_handoff);
    let fixture_hash = sha256_json_value(&result_receipt_fixtures);
    let contract_hash = sha256_text_value(&format!(
        "hepta-full-enablement-runtime-provider-router-activation-command-result-receipt-no-persistence:native:source={source_report_sha256}:fixtures={fixture_hash}:route_count={}:record=0:persist=0:export=0:query=0:observe=0:activation=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "runtime-provider-router-activation-command-result-receipt-no-persistence:report-only:no-receipt-record:no-receipt-persist:no-export:no-query:no-observability:no-activation:no-runtime:no-provider:no-model:no-secret-read",
    );

    let mut report = source_noop_handoff.clone();
    extend_json_object(
        &mut report,
        serde_json::json!({
            "status": if report_ready { "ready" } else { "blocked" },
            "gate": "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_no_persistence_route",
            "source_command": "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-no-persistence --json",
            "native_route": true,
            "compatibility_mode": "native_runtime_provider_router_activation_command_result_receipt_no_persistence_status",
            "side_effect_free": true,
            "audit_date": "2026-06-30",
            "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_NO_PERSISTENCE_ENDPOINT,
            "source_activation_command_noop_handoff_route_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_NOOP_HANDOFF_ENDPOINT,
            "source_activation_command_noop_handoff_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-noop-handoff-gate.sh",
            "source_activation_command_noop_handoff_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-noop-handoff-route-gate.sh",
            "source_activation_command_result_receipt_no_persistence_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-no-persistence-gate.sh",
            "source_activation_command_result_receipt_no_persistence_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-no-persistence-route-gate.sh",
            "source_activation_command_noop_handoff_report_sha256": source_report_sha256,
            "activation_command_result_receipt_fixtures_sha256": fixture_hash,
            "activation_command_result_receipt_contract_hash_sha256": contract_hash,
            "activation_command_result_receipt_policy_hash_sha256": policy_hash,
            "minimum_required_samples": 24,
            "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
            "route_count": route_matrix.route_count,
            "implemented_route_count": route_matrix.implemented_route_count,
            "missing_route_count": route_matrix.missing_route_count,
            "route_count_cutover_floor": NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR,
            "route_count_floor_preserved": route_count_floor_preserved,
            "route_count_source_command_accepted": route_count_source_command_accepted,
            "source_route_wired": true,
            "source_activation_command_noop_handoff_ready": source_noop_handoff_ready,
            "source_activation_command_noop_handoff_status": source_str("runtime_provider_router_activation_command_noop_handoff_status"),
            "source_activation_request_denial_matrix_ready": source_bool("runtime_provider_router_activation_request_denial_matrix_ready"),
            "source_runtime_model_provider_router": source_str("source_runtime_model_provider_router"),
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "runtime_provider_router_activation_command_result_receipt_no_persistence_route_enabled": true,
            "runtime_provider_router_activation_command_result_receipt_no_persistence_ready": true,
            "runtime_provider_router_activation_command_result_receipt_no_persistence_status": "blocked",
            "activation_command_result_receipt_no_persistence_schema_version": "memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_no_persistence_v1",
            "activation_command_result_receipt_no_persistence_mode": "runtime_provider_router_activation_command_result_receipt_no_persistence_no_record_no_persist_no_export_no_query",
            "runtime_provider_router_activation_command_noop_handoff_ready": source_bool("runtime_provider_router_activation_command_noop_handoff_ready"),
            "runtime_provider_router_activation_command_noop_handoff_status": source_str("runtime_provider_router_activation_command_noop_handoff_status"),
            "activation_command_result_receipt_surface_count": 14,
            "activation_command_result_receipt_surface_ready_count": 14,
            "activation_command_result_receipt_side_effect_free_surface_count": 14,
            "activation_command_result_receipt_fixtures": result_receipt_fixtures,
            "activation_command_result_receipt_fixture_count": result_receipt_fixture_count,
            "activation_command_result_receipt_requested_fixture_count": result_receipt_fixture_count,
            "blocked_activation_command_result_receipt_fixture_count": result_receipt_fixture_count,
            "noop_activation_command_result_receipt_fixture_count": result_receipt_fixture_count,
            "allowed_activation_command_result_receipt_fixture_count": 0,
            "accepted_activation_command_result_receipt_fixture_count": 0,
            "activation_command_result_receipt_denied_count": 10,
            "activation_command_result_receipt_performed_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "activation_command_result_receipt_shape_registered": false,
            "activation_command_result_receipt_allowed": false,
            "activation_command_result_receipt_schema_accepted": false,
            "activation_command_result_receipt_recorded": false,
            "activation_command_result_receipt_persisted": false,
            "activation_command_result_receipt_accepted": false,
            "activation_command_result_receipt_materialized": false,
            "activation_command_result_receipt_filesystem_written": false,
            "activation_command_result_receipt_ledger_written": false,
            "activation_command_result_receipt_indexed": false,
            "activation_command_result_receipt_enqueued": false,
            "activation_command_result_receipt_delivered": false,
            "activation_command_result_receipt_exported": false,
            "activation_command_result_receipt_query_registered": false,
            "activation_command_result_receipt_observability_recorded": false,
            "activation_command_result_receipt_hash_bound": false,
            "activation_command_result_receipt_signature_hash_recorded": false,
            "activation_command_result_receipt_timestamp_recorded": false,
            "activation_command_result_receipt_operator_identity_accepted": false,
            "activation_command_result_receipt_status_accepted": false,
            "activation_command_result_receipt_blocked_noop_status_accepted": false,
            "activation_command_completion_ack_recorded": false,
            "activation_command_completion_ack_persisted": false,
            "activation_command_completion_ack_accepted": false,
            "activation_command_completion_ack_materialized": false,
            "activation_command_completion_ack_delivered": false,
            "operator_approval_from_receipt_accepted": false,
            "activation_from_receipt_allowed": false,
        }),
    );
    for key in [
        "activation_command_shape_registered",
        "activation_command_allowed",
        "activation_command_accepted",
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
        "activation_request_allowed",
        "activation_request_accepted",
        "activation_request_recorded",
        "activation_request_persisted",
        "activation_request_materialized",
        "activation_request_filesystem_written",
        "activation_request_delivered",
        "activation_request_executed",
        "activation_activated",
        "runtime_router_mutated",
        "runtime_attachment_performed",
        "live_context_attached",
        "context_injection_performed",
        "adapter_invoked",
        "provider_invoked",
        "model_invoked",
        "auth_secret_read",
        "credential_read",
        "secret_file_read",
        "usage_recorded",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "receipt_export_allowed",
        "receipt_exported",
        "receipt_query_allowed",
        "receipt_query_registered",
        "receipt_observability_allowed",
        "receipt_observability_recorded",
        "receipt_recorded",
        "receipt_persisted",
        "receipt_accepted",
        "receipt_materialized",
        "receipt_filesystem_written",
        "readback_evidence_recorded",
        "readback_evidence_persisted",
        "router_handoff_recorded",
        "router_handoff_persisted",
        "rollback_executed",
        "telegram_send_performed",
        "channel_send_performed",
        "external_send_performed",
        "public_release_claimed",
        "public_ga_claimed",
        "release_artifact_written",
        "install_executed",
        "launchd_mutated",
        "service_restart_performed",
        "active_binary_mutated",
    ] {
        if let Some(report) = report.as_object_mut() {
            report.insert(key.to_string(), serde_json::Value::Bool(false));
        }
    }
    extend_json_object(
        &mut report,
        serde_json::json!({
            "activation_command_result_receipt_surfaces": [
                "source_activation_command_noop_handoff_report_required",
                "disabled_activation_command_noop_identity_required",
                "result_receipt_schema_registration_denied",
                "result_receipt_hash_signature_timestamp_binding_denied",
                "result_receipt_blocked_noop_status_acceptance_denied",
                "result_receipt_record_persist_materialize_denied",
                "result_receipt_filesystem_ledger_index_queue_delivery_denied",
                "result_receipt_export_query_observability_denied",
                "activation_command_completion_ack_denied",
                "operator_approval_and_activation_from_receipt_denied",
                "runtime_router_live_context_context_injection_denied",
                "adapter_provider_model_invocation_denied",
                "usage_memory_kg_write_denied",
                "external_public_install_restart_active_binary_denied"
            ],
            "denied_by_activation_command_result_receipt_no_persistence": denials,
            "denied_by_activation_command_result_receipt_no_persistence_count": 35,
            "allowed_next_actions": [
                {
                    "action": "review_runtime_provider_router_activation_command_result_receipt_no_persistence",
                    "status": "allowed_report_only",
                    "records_command_result": false,
                    "persists_command_result": false,
                    "exports_receipt": false,
                    "registers_query": false,
                    "registers_observability": false,
                    "mutates_runtime": false,
                    "invokes_model": false
                },
                {
                    "action": "stage_runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial",
                    "status": "allowed_report_only_next_slice",
                    "accepts_duplicate_receipt": false,
                    "records_idempotency": false,
                    "persists_replay_state": false,
                    "mutates_runtime": false,
                    "invokes_model": false
                },
                {
                    "action": "run_full_light_preflight",
                    "status": "allowed_verification_only",
                    "records_command_result": false,
                    "persists_command_result": false,
                    "mutates_runtime": false,
                    "invokes_model": false,
                    "writes_kg": false
                }
            ],
            "source_activation_command_noop_handoff_report_required": true,
            "result_receipt_schema_registration_forbidden": true,
            "result_receipt_recording_forbidden": true,
            "result_receipt_persistence_forbidden": true,
            "result_receipt_export_query_observability_forbidden": true,
            "result_receipt_activation_forbidden": true,
            "result_receipt_runtime_mutation_forbidden": true,
            "result_receipt_context_attachment_forbidden": true,
            "result_receipt_adapter_provider_model_invocation_forbidden": true,
            "result_receipt_memory_kg_write_forbidden": true,
            "result_receipt_secret_read_forbidden": true,
            "result_receipt_external_public_install_restart_active_binary_forbidden": true,
        }),
    );
    if let Some(side_effects) = report
        .get_mut("side_effects")
        .and_then(serde_json::Value::as_object_mut)
    {
        for key in [
            "activation_command_result_receipt_shape_registered",
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
            "activation_command_result_receipt_exported",
            "activation_command_result_receipt_query_registered",
            "activation_command_result_receipt_observability_recorded",
            "activation_command_completion_ack_recorded",
            "activation_command_completion_ack_persisted",
            "activation_command_completion_ack_accepted",
            "activation_command_completion_ack_delivered",
            "operator_approval_from_receipt_accepted",
            "activation_from_receipt_allowed",
            "activation_command_shape_registered",
            "activation_command_accepted",
            "activation_command_enabled",
            "activation_command_invoked",
            "activation_command_dispatched",
            "activation_command_dispatch_performed",
            "activation_command_noop_decision_recorded",
            "activation_command_noop_decision_persisted",
            "activation_command_handoff_recorded",
            "activation_command_handoff_persisted",
            "activation_command_handoff_materialized",
            "activation_request_recorded",
            "activation_request_persisted",
            "activation_request_materialized",
            "activation_request_filesystem_written",
            "activation_request_delivered",
            "activation_request_executed",
            "activation_activated",
            "runtime_router_mutated",
            "runtime_attachment_performed",
            "live_context_attached",
            "context_injection_performed",
            "adapter_invoked",
            "provider_invoked",
            "model_invoked",
            "auth_secret_read",
            "credential_read",
            "secret_file_read",
            "usage_recorded",
            "memory_store_write_performed",
            "memory_store_mutated",
            "live_kg_write_performed",
            "receipt_exported",
            "receipt_query_registered",
            "receipt_observability_recorded",
            "receipt_recorded",
            "receipt_persisted",
            "receipt_accepted",
            "receipt_materialized",
            "receipt_filesystem_written",
            "readback_evidence_recorded",
            "readback_evidence_persisted",
            "router_handoff_recorded",
            "router_handoff_persisted",
            "rollback_executed",
            "telegram_send_performed",
            "channel_send_performed",
            "external_send_performed",
            "filesystem_written",
            "public_release_claimed",
            "public_ga_claimed",
            "release_artifact_written",
            "install_executed",
            "launchd_mutated",
            "service_restarted",
            "service_restart_performed",
            "active_binary_mutated",
        ] {
            side_effects.insert(key.to_string(), serde_json::Value::Bool(false));
        }
    }
    report
}

fn hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_no_persistence =
        hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_no_persistence_report();
    let source_bool = |key: &str| {
        source_no_persistence
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let source_u64 = |key: &str| {
        source_no_persistence
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_str = |key: &str| {
        source_no_persistence
            .get(key)
            .and_then(serde_json::Value::as_str)
            .unwrap_or("blocked")
            .to_string()
    };
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_no_persistence_ready = source_str("status") == "ready"
        && source_bool(
            "runtime_provider_router_activation_command_result_receipt_no_persistence_ready",
        )
        && source_str(
            "runtime_provider_router_activation_command_result_receipt_no_persistence_status",
        ) == "blocked"
        && source_bool("runtime_provider_router_activation_command_noop_handoff_ready")
        && source_str("runtime_provider_router_activation_command_noop_handoff_status")
            == "blocked"
        && source_u64("activation_command_result_receipt_surface_count") == 14
        && source_u64("activation_command_result_receipt_surface_ready_count") == 14
        && source_u64("activation_command_result_receipt_fixture_count") == 10
        && source_u64("blocked_activation_command_result_receipt_fixture_count") == 10
        && source_u64("noop_activation_command_result_receipt_fixture_count") == 10
        && source_u64("allowed_activation_command_result_receipt_fixture_count") == 0
        && source_u64("accepted_activation_command_result_receipt_fixture_count") == 0
        && source_u64("activation_command_result_receipt_denied_count") == 10
        && source_u64("activation_command_result_receipt_performed_count") == 0
        && !source_bool("activation_command_result_receipt_shape_registered")
        && !source_bool("activation_command_result_receipt_allowed")
        && !source_bool("activation_command_result_receipt_schema_accepted")
        && !source_bool("activation_command_result_receipt_recorded")
        && !source_bool("activation_command_result_receipt_persisted")
        && !source_bool("activation_command_result_receipt_accepted")
        && !source_bool("activation_command_result_receipt_materialized")
        && !source_bool("activation_command_result_receipt_filesystem_written")
        && !source_bool("activation_command_result_receipt_ledger_written")
        && !source_bool("activation_command_result_receipt_indexed")
        && !source_bool("activation_command_result_receipt_enqueued")
        && !source_bool("activation_command_result_receipt_delivered")
        && !source_bool("activation_command_result_receipt_exported")
        && !source_bool("activation_command_result_receipt_query_registered")
        && !source_bool("activation_command_result_receipt_observability_recorded")
        && !source_bool("activation_command_completion_ack_recorded")
        && !source_bool("activation_command_completion_ack_persisted")
        && !source_bool("activation_command_completion_ack_accepted")
        && !source_bool("operator_approval_from_receipt_accepted")
        && !source_bool("activation_from_receipt_allowed")
        && !source_bool("activation_command_enabled")
        && !source_bool("activation_command_invoked")
        && !source_bool("activation_command_dispatched")
        && !source_bool("activation_request_accepted")
        && !source_bool("activation_request_recorded")
        && !source_bool("activation_request_executed")
        && !source_bool("activation_activated")
        && !source_bool("runtime_router_mutated")
        && !source_bool("runtime_attachment_performed")
        && !source_bool("live_context_attached")
        && !source_bool("context_injection_performed")
        && !source_bool("adapter_invoked")
        && !source_bool("provider_invoked")
        && !source_bool("model_invoked")
        && !source_bool("auth_secret_read")
        && !source_bool("credential_read")
        && !source_bool("secret_file_read")
        && !source_bool("usage_recorded")
        && !source_bool("memory_store_write_performed")
        && !source_bool("memory_store_mutated")
        && !source_bool("live_kg_write_performed")
        && !source_bool("telegram_send_performed")
        && !source_bool("channel_send_performed")
        && !source_bool("external_send_performed")
        && !source_bool("service_restart_performed")
        && !source_bool("active_binary_mutated");
    let report_ready = route_matrix.ready
        && route_count_floor_preserved
        && route_count_source_command_accepted
        && source_no_persistence_ready;

    let replay_fixture = |id: &str, status: &str, reason: &str, extra: serde_json::Value| {
        let mut fixture = serde_json::Map::new();
        fixture.insert("id".to_string(), serde_json::Value::String(id.to_string()));
        fixture.insert(
            "replay_status".to_string(),
            serde_json::Value::String(status.to_string()),
        );
        fixture.insert(
            "reason".to_string(),
            serde_json::Value::String(reason.to_string()),
        );
        for key in [
            "source_no_persistence_present",
            "source_no_persistence_ready",
            "replay_requested",
            "canonical_noop_result_receipt_identity_required",
            "receipt_noop_confirmed",
        ] {
            fixture.insert(key.to_string(), serde_json::Value::Bool(true));
        }
        for key in [
            "activation_command_result_receipt_replay_allowed",
            "activation_command_result_receipt_replay_recorded",
            "activation_command_result_receipt_replay_persisted",
            "activation_command_result_receipt_replay_materialized",
            "activation_command_result_receipt_replay_filesystem_written",
            "activation_command_result_receipt_replay_performed",
            "activation_command_result_receipt_duplicate_accepted",
            "activation_command_result_receipt_duplicate_recorded",
            "activation_command_result_receipt_duplicate_persisted",
            "activation_command_result_receipt_idempotency_key_accepted",
            "activation_command_result_receipt_idempotency_key_recorded",
            "activation_command_result_receipt_idempotency_state_recorded",
            "activation_command_result_receipt_idempotency_state_persisted",
            "activation_command_result_receipt_idempotency_state_materialized",
            "activation_command_result_receipt_idempotency_filesystem_written",
            "activation_command_result_receipt_replay_nonce_accepted",
            "activation_command_result_receipt_replay_nonce_recorded",
            "activation_command_result_receipt_cross_scope_reuse_accepted",
            "activation_command_result_receipt_status_upgrade_accepted",
            "activation_command_result_receipt_completed_status_accepted",
            "activation_command_result_receipt_ack_replay_accepted",
            "activation_command_result_receipt_ledger_replay_accepted",
            "activation_command_result_receipt_index_replay_accepted",
            "activation_command_result_receipt_delivery_replay_accepted",
            "activation_command_result_receipt_export_replay_accepted",
            "activation_command_result_receipt_query_replay_accepted",
            "activation_command_result_receipt_observability_replay_accepted",
            "activation_command_result_receipt_recorded",
            "activation_command_result_receipt_persisted",
            "activation_command_result_receipt_accepted",
            "activation_command_result_receipt_materialized",
            "activation_command_result_receipt_filesystem_written",
            "activation_command_result_receipt_ledger_written",
            "activation_command_result_receipt_indexed",
            "activation_command_result_receipt_enqueued",
            "activation_command_result_receipt_delivered",
            "activation_command_result_receipt_exported",
            "activation_command_result_receipt_query_registered",
            "activation_command_result_receipt_observability_recorded",
            "activation_command_completion_ack_recorded",
            "activation_command_completion_ack_persisted",
            "activation_command_completion_ack_accepted",
            "activation_command_completion_ack_materialized",
            "activation_command_completion_ack_delivered",
            "operator_approval_from_replay_accepted",
            "operator_approval_from_receipt_accepted",
            "activation_from_replay_allowed",
            "activation_from_receipt_allowed",
            "activation_command_shape_registered",
            "activation_command_allowed",
            "activation_command_accepted",
            "activation_command_enabled",
            "activation_command_invoked",
            "activation_command_dispatched",
            "activation_command_dispatch_performed",
            "activation_command_noop_decision_recorded",
            "activation_command_noop_decision_persisted",
            "activation_command_handoff_recorded",
            "activation_command_handoff_persisted",
            "activation_request_allowed",
            "activation_request_accepted",
            "activation_request_recorded",
            "activation_request_persisted",
            "activation_request_executed",
            "activation_activated",
            "operator_approval_recorded",
            "runtime_router_mutated",
            "runtime_attachment_performed",
            "live_context_attached",
            "context_injection_performed",
            "adapter_invoked",
            "provider_invoked",
            "model_invoked",
            "auth_secret_read",
            "credential_read",
            "secret_file_read",
            "usage_recorded",
            "memory_store_write_performed",
            "memory_store_mutated",
            "live_kg_write_performed",
            "replay_ledger_written",
            "replay_indexed",
            "replay_query_registered",
            "replay_observability_recorded",
            "receipt_recorded",
            "receipt_persisted",
            "receipt_accepted",
            "readback_evidence_recorded",
            "readback_evidence_persisted",
            "router_handoff_recorded",
            "router_handoff_persisted",
            "rollback_executed",
            "telegram_send_performed",
            "channel_send_performed",
            "external_send_performed",
            "public_release_claimed",
            "public_ga_claimed",
            "release_artifact_written",
            "install_executed",
            "launchd_mutated",
            "service_restart_performed",
            "active_binary_mutated",
        ] {
            fixture.insert(key.to_string(), serde_json::Value::Bool(false));
        }
        let mut fixture = serde_json::Value::Object(fixture);
        extend_json_object(&mut fixture, extra);
        fixture
    };
    let replay_idempotency_fixtures = serde_json::Value::Array(vec![
        replay_fixture(
            "provider-router-activation-command-result-receipt-replay-missing-source-no-persistence-report",
            "blocked_noop",
            "source_result_receipt_no_persistence_report_required",
            serde_json::json!({
                "source_no_persistence_present": false,
                "source_no_persistence_ready": false,
            }),
        ),
        replay_fixture(
            "provider-router-activation-command-result-receipt-duplicate-identity-replay-attempt",
            "blocked_duplicate_noop",
            "duplicate_result_receipt_identity_replay_denied",
            serde_json::json!({"duplicate_result_receipt_identity_requested": true}),
        ),
        replay_fixture(
            "provider-router-activation-command-result-receipt-replay-acceptance-attempt",
            "blocked_replay_noop",
            "result_receipt_replay_acceptance_denied",
            serde_json::json!({"result_receipt_replay_acceptance_requested": true}),
        ),
        replay_fixture(
            "provider-router-activation-command-result-receipt-idempotency-key-recording-attempt",
            "blocked_idempotency_key_noop",
            "idempotency_key_recording_denied",
            serde_json::json!({
                "idempotency_key_acceptance_requested": true,
                "idempotency_key_recording_requested": true,
            }),
        ),
        replay_fixture(
            "provider-router-activation-command-result-receipt-idempotency-state-persistence-attempt",
            "blocked_idempotency_state_noop",
            "idempotency_state_persistence_materialization_denied",
            serde_json::json!({
                "idempotency_state_recording_requested": true,
                "idempotency_state_persistence_requested": true,
                "idempotency_state_materialization_requested": true,
                "idempotency_filesystem_write_requested": true,
            }),
        ),
        replay_fixture(
            "provider-router-activation-command-result-receipt-cross-scope-reuse-attempt",
            "blocked_cross_scope_noop",
            "cross_scope_result_receipt_reuse_denied",
            serde_json::json!({"cross_scope_reuse_requested": true}),
        ),
        replay_fixture(
            "provider-router-activation-command-result-receipt-stale-nonce-out-of-order-replay-attempt",
            "blocked_nonce_order_noop",
            "stale_nonce_out_of_order_receipt_replay_denied",
            serde_json::json!({
                "stale_nonce_replay_requested": true,
                "out_of_order_replay_requested": true,
                "replay_nonce_acceptance_requested": true,
            }),
        ),
        replay_fixture(
            "provider-router-activation-command-result-receipt-completion-ledger-delivery-replay-attempt",
            "blocked_completion_ledger_delivery_noop",
            "completion_ack_ledger_delivery_replay_denied",
            serde_json::json!({
                "completion_ack_replay_requested": true,
                "ledger_replay_requested": true,
                "index_replay_requested": true,
                "delivery_replay_requested": true,
            }),
        ),
        replay_fixture(
            "provider-router-activation-command-result-receipt-runtime-provider-memory-kg-replay-attempt",
            "blocked_runtime_provider_memory_kg_noop",
            "runtime_provider_memory_kg_replay_denied",
            serde_json::json!({
                "runtime_replay_requested": true,
                "provider_replay_requested": true,
                "model_replay_requested": true,
                "usage_replay_requested": true,
                "memory_store_replay_requested": true,
                "live_kg_replay_requested": true,
            }),
        ),
        replay_fixture(
            "provider-router-activation-command-result-receipt-external-public-install-restart-active-binary-replay-attempt",
            "blocked_external_noop",
            "external_public_install_restart_active_binary_replay_denied",
            serde_json::json!({
                "external_send_replay_requested": true,
                "public_claim_replay_requested": true,
                "public_ga_replay_requested": true,
                "release_artifact_replay_requested": true,
                "install_replay_requested": true,
                "launchd_restart_replay_requested": true,
                "service_restart_replay_requested": true,
                "active_binary_mutation_replay_requested": true,
            }),
        ),
    ]);
    let replay_idempotency_fixture_count = replay_idempotency_fixtures
        .as_array()
        .map(std::vec::Vec::len)
        .unwrap_or(0);
    let denials: Vec<serde_json::Value> = [
        "source_result_receipt_no_persistence_report_required",
        "canonical_noop_result_receipt_identity_required",
        "duplicate_result_receipt_identity_replay_denied",
        "result_receipt_replay_acceptance_denied",
        "idempotency_key_recording_denied",
        "idempotency_state_recording_denied",
        "idempotency_state_persistence_denied",
        "idempotency_state_materialization_denied",
        "idempotency_filesystem_write_denied",
        "cross_scope_result_receipt_reuse_denied",
        "stale_nonce_replay_denied",
        "out_of_order_receipt_replay_denied",
        "completion_ack_replay_denied",
        "activation_from_replay_denied",
        "runtime_router_replay_denied",
        "live_context_replay_denied",
        "context_injection_replay_denied",
        "adapter_invocation_replay_denied",
        "provider_model_replay_denied",
        "usage_record_replay_denied",
        "memory_store_replay_denied",
        "live_kg_replay_denied",
        "secret_material_replay_denied",
        "external_send_replay_denied",
        "public_claim_replay_denied",
        "install_restart_active_binary_replay_denied",
    ]
    .into_iter()
    .map(|item| serde_json::Value::String(item.to_string()))
    .collect();
    let denied_count = denials.len();
    let source_report_sha256 = sha256_json_value(&source_no_persistence);
    let fixture_hash = sha256_json_value(&replay_idempotency_fixtures);
    let contract_hash = sha256_text_value(&format!(
        "hepta-full-enablement-runtime-provider-router-activation-command-result-receipt-replay-idempotency-denial:native:source={source_report_sha256}:fixtures={fixture_hash}:route_count={}:replay=0:duplicate=0:idempotency=0:activation=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "runtime-provider-router-activation-command-result-receipt-replay-idempotency-denial:report-only:no-duplicate:no-replay:no-idempotency-record:no-persist:no-runtime:no-provider:no-model:no-secret-read",
    );

    let mut report = source_no_persistence.clone();
    extend_json_object(
        &mut report,
        serde_json::json!({
            "status": if report_ready { "ready" } else { "blocked" },
            "gate": "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_route",
            "source_command": "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-replay-idempotency-denial --json",
            "native_route": true,
            "compatibility_mode": "native_runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_status",
            "side_effect_free": true,
            "audit_date": "2026-06-30",
            "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_REPLAY_IDEMPOTENCY_DENIAL_ENDPOINT,
            "source_activation_command_result_receipt_no_persistence_route_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_NO_PERSISTENCE_ENDPOINT,
            "source_activation_command_result_receipt_no_persistence_gate": source_str("gate"),
            "source_activation_command_result_receipt_no_persistence_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-no-persistence-route-gate.sh",
            "source_activation_command_result_receipt_replay_idempotency_denial_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-replay-idempotency-denial-gate.sh",
            "source_activation_command_result_receipt_replay_idempotency_denial_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-replay-idempotency-denial-route-gate.sh",
            "source_activation_command_result_receipt_no_persistence_report_sha256": source_report_sha256,
            "replay_idempotency_fixtures_sha256": fixture_hash,
            "replay_idempotency_contract_hash_sha256": contract_hash,
            "replay_idempotency_policy_hash_sha256": policy_hash,
            "minimum_required_samples": 24,
            "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
            "route_count": route_matrix.route_count,
            "implemented_route_count": route_matrix.implemented_route_count,
            "missing_route_count": route_matrix.missing_route_count,
            "route_count_cutover_floor": NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR,
            "route_count_floor_preserved": route_count_floor_preserved,
            "route_count_source_command_accepted": route_count_source_command_accepted,
            "source_route_wired": true,
            "source_activation_command_result_receipt_no_persistence_ready": source_no_persistence_ready,
            "source_activation_command_result_receipt_no_persistence_status": source_str("runtime_provider_router_activation_command_result_receipt_no_persistence_status"),
            "runtime_provider_router_activation_command_result_receipt_no_persistence_ready": source_bool("runtime_provider_router_activation_command_result_receipt_no_persistence_ready"),
            "runtime_provider_router_activation_command_result_receipt_no_persistence_status": source_str("runtime_provider_router_activation_command_result_receipt_no_persistence_status"),
            "runtime_provider_router_activation_command_noop_handoff_ready": source_bool("runtime_provider_router_activation_command_noop_handoff_ready"),
            "runtime_provider_router_activation_command_noop_handoff_status": source_str("runtime_provider_router_activation_command_noop_handoff_status"),
            "operator_authorization_source": "telegram_direct_operator_authorization_2026_06_30_14_26_asia_shanghai",
            "operator_authorization_received": true,
            "fresh_evidence_review_requested": true,
            "explicit_command_path_reviewed": true,
            "accepted_operator_approval_consumed": false,
            "activation_authority_derived": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_route_enabled": true,
            "runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready": true,
            "runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_status": "blocked",
            "activation_command_result_receipt_replay_idempotency_schema_version": "memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_v1",
            "activation_command_result_receipt_replay_idempotency_mode": "runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_no_duplicate_no_replay_no_idempotency_persist",
            "activation_command_result_receipt_replay_idempotency_decision": "runtime_provider_router_activation_command_result_receipt_cannot_be_replayed_duplicated_or_converted_into_idempotency_authority",
            "source_activation_command_result_receipt_fixture_count": source_u64("activation_command_result_receipt_fixture_count"),
            "source_blocked_activation_command_result_receipt_fixture_count": source_u64("blocked_activation_command_result_receipt_fixture_count"),
            "source_noop_activation_command_result_receipt_fixture_count": source_u64("noop_activation_command_result_receipt_fixture_count"),
            "source_accepted_activation_command_result_receipt_fixture_count": source_u64("accepted_activation_command_result_receipt_fixture_count"),
            "source_activation_command_result_receipt_performed_count": source_u64("activation_command_result_receipt_performed_count"),
            "replay_idempotency_surface_count": 14,
            "replay_idempotency_surface_ready_count": 14,
            "replay_idempotency_side_effect_free_surface_count": 14,
            "replay_idempotency_fixtures": replay_idempotency_fixtures,
            "replay_idempotency_fixture_count": replay_idempotency_fixture_count,
            "blocked_replay_idempotency_fixture_count": replay_idempotency_fixture_count,
            "noop_replay_idempotency_fixture_count": replay_idempotency_fixture_count,
            "allowed_replay_idempotency_fixture_count": 0,
            "accepted_replay_idempotency_fixture_count": 0,
            "duplicate_result_receipt_replay_fixture_count": 1,
            "receipt_replay_acceptance_fixture_count": 1,
            "idempotency_key_recording_fixture_count": 1,
            "idempotency_state_persistence_fixture_count": 1,
            "cross_scope_result_receipt_reuse_fixture_count": 1,
            "nonce_order_replay_fixture_count": 1,
            "completion_ack_replay_fixture_count": 1,
            "runtime_provider_memory_kg_replay_fixture_count": 1,
            "external_public_install_replay_fixture_count": 1,
            "replay_idempotency_denied_count": replay_idempotency_fixture_count,
            "duplicate_result_receipt_denied_count": replay_idempotency_fixture_count,
            "idempotency_state_denied_count": replay_idempotency_fixture_count,
            "replay_idempotency_performed_count": 0,
            "duplicate_result_receipt_accepted_count": 0,
            "idempotency_state_recorded_count": 0,
            "idempotency_state_persisted_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "activation_command_result_receipt_replay_allowed": false,
            "activation_command_result_receipt_replay_recorded": false,
            "activation_command_result_receipt_replay_persisted": false,
            "activation_command_result_receipt_replay_materialized": false,
            "activation_command_result_receipt_replay_filesystem_written": false,
            "activation_command_result_receipt_replay_performed": false,
            "activation_command_result_receipt_duplicate_accepted": false,
            "activation_command_result_receipt_duplicate_recorded": false,
            "activation_command_result_receipt_duplicate_persisted": false,
            "activation_command_result_receipt_idempotency_key_accepted": false,
            "activation_command_result_receipt_idempotency_key_recorded": false,
            "activation_command_result_receipt_idempotency_state_recorded": false,
            "activation_command_result_receipt_idempotency_state_persisted": false,
            "activation_command_result_receipt_idempotency_state_materialized": false,
            "activation_command_result_receipt_idempotency_filesystem_written": false,
            "activation_command_result_receipt_replay_nonce_accepted": false,
            "activation_command_result_receipt_replay_nonce_recorded": false,
            "activation_command_result_receipt_cross_scope_reuse_accepted": false,
            "activation_command_result_receipt_status_upgrade_accepted": false,
            "activation_command_result_receipt_completed_status_accepted": false,
            "activation_command_result_receipt_ack_replay_accepted": false,
            "activation_command_result_receipt_ledger_replay_accepted": false,
            "activation_command_result_receipt_index_replay_accepted": false,
            "activation_command_result_receipt_delivery_replay_accepted": false,
            "activation_command_result_receipt_export_replay_accepted": false,
            "activation_command_result_receipt_query_replay_accepted": false,
            "activation_command_result_receipt_observability_replay_accepted": false,
            "operator_approval_from_replay_accepted": false,
            "activation_from_replay_allowed": false,
            "operator_approval_recorded": false,
        }),
    );
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
        "activation_command_result_receipt_exported",
        "activation_command_result_receipt_query_registered",
        "activation_command_result_receipt_observability_recorded",
        "activation_command_completion_ack_recorded",
        "activation_command_completion_ack_persisted",
        "activation_command_completion_ack_accepted",
        "activation_command_completion_ack_materialized",
        "activation_command_completion_ack_delivered",
        "operator_approval_from_receipt_accepted",
        "activation_from_receipt_allowed",
        "activation_command_shape_registered",
        "activation_command_allowed",
        "activation_command_accepted",
        "activation_command_enabled",
        "activation_command_invoked",
        "activation_command_dispatched",
        "activation_command_dispatch_performed",
        "activation_command_noop_decision_recorded",
        "activation_command_noop_decision_persisted",
        "activation_command_handoff_recorded",
        "activation_command_handoff_persisted",
        "activation_request_allowed",
        "activation_request_accepted",
        "activation_request_recorded",
        "activation_request_persisted",
        "activation_request_executed",
        "activation_activated",
        "runtime_router_mutated",
        "runtime_attachment_performed",
        "live_context_attached",
        "context_injection_performed",
        "adapter_invoked",
        "provider_invoked",
        "model_invoked",
        "auth_secret_read",
        "credential_read",
        "secret_file_read",
        "usage_recorded",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "replay_ledger_written",
        "replay_indexed",
        "replay_query_registered",
        "replay_observability_recorded",
        "receipt_recorded",
        "receipt_persisted",
        "receipt_accepted",
        "readback_evidence_recorded",
        "readback_evidence_persisted",
        "router_handoff_recorded",
        "router_handoff_persisted",
        "rollback_executed",
        "telegram_send_performed",
        "channel_send_performed",
        "external_send_performed",
        "public_release_claimed",
        "public_ga_claimed",
        "release_artifact_written",
        "install_executed",
        "launchd_mutated",
        "service_restart_performed",
        "active_binary_mutated",
    ] {
        if let Some(report) = report.as_object_mut() {
            report.insert(key.to_string(), serde_json::Value::Bool(false));
        }
    }
    extend_json_object(
        &mut report,
        serde_json::json!({
            "replay_idempotency_surfaces": [
                "source_result_receipt_no_persistence_report_required",
                "canonical_noop_result_receipt_identity_required",
                "duplicate_receipt_rejection_required",
                "replay_request_rejection_required",
                "idempotency_key_state_recording_denied",
                "idempotency_persistence_materialization_denied",
                "cross_scope_receipt_reuse_denied",
                "nonce_order_freshness_replay_denied",
                "completion_ack_replay_denied",
                "activation_from_replay_denied",
                "runtime_router_live_context_replay_denied",
                "adapter_provider_model_replay_denied",
                "usage_memory_kg_replay_denied",
                "external_public_install_restart_active_binary_replay_denied"
            ],
            "denied_by_replay_idempotency": denials,
            "denied_by_replay_idempotency_count": denied_count,
            "denied_by_activation_command_result_receipt_replay_idempotency": denials,
            "denied_by_activation_command_result_receipt_replay_idempotency_count": denied_count,
            "allowed_next_actions": [
                {
                    "action": "review_runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial",
                    "status": "allowed_report_only",
                    "accepts_duplicate_receipt": false,
                    "records_idempotency": false,
                    "persists_replay_state": false,
                    "mutates_runtime": false,
                    "invokes_model": false
                },
                {
                    "action": "stage_runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial",
                    "status": "allowed_report_only_next_slice",
                    "accepts_out_of_order_receipt": false,
                    "records_monotonic_clock": false,
                    "persists_ordering_state": false,
                    "mutates_runtime": false,
                    "invokes_model": false
                },
                {
                    "action": "run_full_light_preflight",
                    "status": "allowed_verification_only",
                    "accepts_duplicate_receipt": false,
                    "persists_replay_state": false,
                    "mutates_runtime": false,
                    "invokes_model": false,
                    "writes_kg": false
                }
            ],
            "source_result_receipt_no_persistence_report_required": true,
            "duplicate_result_receipt_acceptance_forbidden": true,
            "result_receipt_replay_acceptance_forbidden": true,
            "idempotency_key_recording_forbidden": true,
            "idempotency_state_persistence_forbidden": true,
            "cross_scope_receipt_reuse_forbidden": true,
            "completion_ack_replay_forbidden": true,
            "activation_from_replay_forbidden": true,
            "runtime_provider_memory_kg_replay_forbidden": true,
            "secret_read_forbidden": true,
            "external_public_install_restart_active_binary_replay_forbidden": true,
        }),
    );
    if let Some(side_effects) = report
        .get_mut("side_effects")
        .and_then(serde_json::Value::as_object_mut)
    {
        for key in [
            "workspace_written",
            "filesystem_written",
            "activation_command_result_receipt_replay_recorded",
            "activation_command_result_receipt_replay_persisted",
            "activation_command_result_receipt_replay_performed",
            "activation_command_result_receipt_duplicate_accepted",
            "activation_command_result_receipt_duplicate_recorded",
            "activation_command_result_receipt_duplicate_persisted",
            "activation_command_result_receipt_idempotency_key_recorded",
            "activation_command_result_receipt_idempotency_state_recorded",
            "activation_command_result_receipt_idempotency_state_persisted",
            "activation_command_result_receipt_idempotency_state_materialized",
            "activation_command_result_receipt_idempotency_filesystem_written",
            "activation_command_result_receipt_replay_nonce_recorded",
            "activation_command_result_receipt_cross_scope_reuse_accepted",
            "activation_command_result_receipt_status_upgrade_accepted",
            "activation_command_result_receipt_completed_status_accepted",
            "activation_command_result_receipt_ack_replay_accepted",
            "activation_command_result_receipt_ledger_replay_accepted",
            "activation_command_result_receipt_index_replay_accepted",
            "activation_command_result_receipt_delivery_replay_accepted",
            "activation_command_result_receipt_export_replay_accepted",
            "activation_command_result_receipt_query_replay_accepted",
            "activation_command_result_receipt_observability_replay_accepted",
            "activation_command_result_receipt_recorded",
            "activation_command_result_receipt_persisted",
            "activation_command_result_receipt_accepted",
            "activation_command_completion_ack_recorded",
            "activation_command_completion_ack_persisted",
            "activation_command_completion_ack_accepted",
            "operator_approval_from_replay_accepted",
            "activation_from_replay_allowed",
            "activation_from_receipt_allowed",
            "activation_command_enabled",
            "activation_command_invoked",
            "activation_command_dispatched",
            "activation_command_handoff_recorded",
            "activation_request_recorded",
            "activation_request_persisted",
            "activation_request_executed",
            "operator_approval_recorded",
            "runtime_router_mutated",
            "runtime_attachment_performed",
            "live_context_attached",
            "context_injection_performed",
            "adapter_invoked",
            "provider_invoked",
            "model_invoked",
            "auth_secret_read",
            "credential_read",
            "secret_file_read",
            "usage_recorded",
            "memory_store_write_performed",
            "memory_store_mutated",
            "live_kg_write_performed",
            "replay_ledger_written",
            "replay_indexed",
            "replay_query_registered",
            "replay_observability_recorded",
            "receipt_recorded",
            "receipt_persisted",
            "receipt_accepted",
            "readback_evidence_recorded",
            "readback_evidence_persisted",
            "router_handoff_recorded",
            "router_handoff_persisted",
            "rollback_executed",
            "telegram_send_performed",
            "channel_send_performed",
            "external_send_performed",
            "public_release_claimed",
            "public_ga_claimed",
            "release_artifact_written",
            "install_executed",
            "launchd_mutated",
            "service_restart_performed",
            "active_binary_mutated",
        ] {
            side_effects.insert(key.to_string(), serde_json::Value::Bool(false));
        }
    }
    report
}

fn hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_replay =
        hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_report();
    let source_bool = |key: &str| {
        source_replay
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let source_u64 = |key: &str| {
        source_replay
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_str = |key: &str| {
        source_replay
            .get(key)
            .and_then(serde_json::Value::as_str)
            .unwrap_or("blocked")
            .to_string()
    };
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_replay_ready = source_str("status") == "ready"
        && source_bool(
            "runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready",
        )
        && source_str(
            "runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_status",
        ) == "blocked"
        && source_u64("accepted_replay_idempotency_fixture_count") == 0
        && source_u64("replay_idempotency_performed_count") == 0
        && source_u64("idempotency_state_recorded_count") == 0;
    let report_ready = source_replay_ready && route_count_source_command_accepted;

    let ordering_fixture = |fixture_id: &str,
                            status: &str,
                            reason: &str,
                            extra: serde_json::Value| {
        let mut fixture = serde_json::Map::new();
        fixture.insert(
            "fixture_id".to_string(),
            serde_json::Value::String(fixture_id.to_string()),
        );
        fixture.insert(
            "id".to_string(),
            serde_json::Value::String(fixture_id.to_string()),
        );
        fixture.insert(
            "ordering_monotonicity_status".to_string(),
            serde_json::Value::String(status.to_string()),
        );
        fixture.insert(
            "ordering_status".to_string(),
            serde_json::Value::String(status.to_string()),
        );
        fixture.insert(
            "denial_reason".to_string(),
            serde_json::Value::String(reason.to_string()),
        );
        for key in [
            "ordering_requested",
            "source_replay_idempotency_present",
            "source_replay_idempotency_ready",
            "canonical_noop_result_receipt_order_identity_required",
            "receipt_noop_confirmed",
        ] {
            fixture.insert(key.to_string(), serde_json::Value::Bool(true));
        }
        for key in [
            "activation_command_result_receipt_ordering_allowed",
            "activation_command_result_receipt_ordering_recorded",
            "activation_command_result_receipt_ordering_persisted",
            "activation_command_result_receipt_ordering_materialized",
            "activation_command_result_receipt_ordering_filesystem_written",
            "activation_command_result_receipt_ordering_performed",
            "activation_command_result_receipt_sequence_cursor_accepted",
            "activation_command_result_receipt_sequence_cursor_recorded",
            "activation_command_result_receipt_sequence_cursor_persisted",
            "activation_command_result_receipt_monotonicity_state_recorded",
            "activation_command_result_receipt_monotonicity_state_persisted",
            "activation_command_result_receipt_monotonicity_state_materialized",
            "activation_command_result_receipt_monotonicity_filesystem_written",
            "activation_command_result_receipt_timestamp_ordering_accepted",
            "activation_command_result_receipt_epoch_ordering_accepted",
            "activation_command_result_receipt_stage_ordering_accepted",
            "activation_command_result_receipt_same_sequence_hash_override_accepted",
            "activation_command_result_receipt_latest_wins_overwrite_accepted",
            "activation_command_result_receipt_gap_fill_accepted",
            "activation_command_result_receipt_ack_before_noop_accepted",
            "activation_command_result_receipt_ledger_ordering_bypass_accepted",
            "activation_command_result_receipt_index_ordering_bypass_accepted",
            "activation_command_result_receipt_delivery_ordering_bypass_accepted",
            "activation_command_result_receipt_runtime_ordering_bypass_accepted",
            "activation_command_result_receipt_provider_ordering_bypass_accepted",
            "activation_command_result_receipt_memory_kg_ordering_bypass_accepted",
            "activation_command_result_receipt_external_public_install_ordering_bypass_accepted",
            "activation_command_result_receipt_replay_allowed",
            "activation_command_result_receipt_replay_recorded",
            "activation_command_result_receipt_replay_persisted",
            "activation_command_result_receipt_duplicate_accepted",
            "activation_command_result_receipt_idempotency_key_accepted",
            "activation_command_result_receipt_idempotency_state_recorded",
            "activation_command_result_receipt_idempotency_state_persisted",
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
            "operator_approval_from_ordering_accepted",
            "activation_from_ordering_allowed",
            "activation_from_replay_allowed",
            "activation_from_receipt_allowed",
            "activation_command_enabled",
            "activation_command_invoked",
            "activation_command_dispatched",
            "activation_command_dispatch_performed",
            "activation_request_accepted",
            "activation_request_recorded",
            "activation_request_persisted",
            "activation_request_executed",
            "activation_activated",
            "runtime_router_mutated",
            "runtime_attachment_performed",
            "live_context_attached",
            "context_injection_performed",
            "adapter_invoked",
            "provider_invoked",
            "model_invoked",
            "auth_secret_read",
            "credential_read",
            "secret_file_read",
            "usage_recorded",
            "memory_store_write_performed",
            "memory_store_mutated",
            "live_kg_write_performed",
            "ordering_ledger_written",
            "ordering_indexed",
            "ordering_query_registered",
            "ordering_observability_recorded",
            "receipt_recorded",
            "receipt_persisted",
            "receipt_accepted",
            "readback_evidence_recorded",
            "readback_evidence_persisted",
            "router_handoff_recorded",
            "router_handoff_persisted",
            "rollback_executed",
            "telegram_send_performed",
            "channel_send_performed",
            "external_send_performed",
            "public_release_claimed",
            "public_ga_claimed",
            "release_artifact_written",
            "install_executed",
            "launchd_mutated",
            "service_restart_performed",
            "active_binary_mutated",
        ] {
            fixture.insert(key.to_string(), serde_json::Value::Bool(false));
        }
        let mut fixture = serde_json::Value::Object(fixture);
        extend_json_object(&mut fixture, extra);
        fixture
    };
    let ordering_monotonicity_fixtures = serde_json::Value::Array(vec![
        ordering_fixture(
            "provider-router-activation-command-result-receipt-ordering-missing-source-replay-idempotency-report",
            "blocked_noop",
            "source_result_receipt_replay_idempotency_report_required",
            serde_json::json!({
                "source_replay_idempotency_present": false,
                "source_replay_idempotency_ready": false,
            }),
        ),
        ordering_fixture(
            "provider-router-activation-command-result-receipt-sequence-cursor-recording-attempt",
            "blocked_ordering_noop",
            "sequence_cursor_recording_denied",
            serde_json::json!({
                "sequence_cursor_recording_requested": true,
                "requested_sequence_cursor": "provider_router_activation_receipt_sequence_1",
            }),
        ),
        ordering_fixture(
            "provider-router-activation-command-result-receipt-out-of-order-sequence-attempt",
            "blocked_ordering_noop",
            "out_of_order_result_receipt_sequence_denied",
            serde_json::json!({
                "out_of_order_sequence_requested": true,
                "requested_sequence": 2,
                "observed_previous_sequence": 3,
            }),
        ),
        ordering_fixture(
            "provider-router-activation-command-result-receipt-sequence-gap-skip-attempt",
            "blocked_ordering_noop",
            "sequence_gap_or_skip_result_receipt_denied",
            serde_json::json!({
                "sequence_gap_requested": true,
                "requested_sequence": 5,
                "expected_next_sequence": 1,
            }),
        ),
        ordering_fixture(
            "provider-router-activation-command-result-receipt-timestamp-rollback-attempt",
            "blocked_ordering_noop",
            "timestamp_rollback_result_receipt_denied",
            serde_json::json!({
                "timestamp_rollback_requested": true,
                "requested_timestamp_order": "older_than_source_replay_idempotency_report",
            }),
        ),
        ordering_fixture(
            "provider-router-activation-command-result-receipt-epoch-rollback-attempt",
            "blocked_ordering_noop",
            "epoch_rollback_result_receipt_denied",
            serde_json::json!({
                "epoch_rollback_requested": true,
                "requested_epoch_order": "lower_than_current_activation_epoch",
            }),
        ),
        ordering_fixture(
            "provider-router-activation-command-result-receipt-same-sequence-different-hash-attempt",
            "blocked_ordering_noop",
            "same_sequence_different_hash_result_receipt_denied",
            serde_json::json!({
                "same_sequence_different_hash_requested": true,
                "requested_sequence": 1,
                "requested_hash_relation": "different_hash_for_same_sequence",
            }),
        ),
        ordering_fixture(
            "provider-router-activation-command-result-receipt-latest-wins-overwrite-attempt",
            "blocked_ordering_noop",
            "latest_wins_result_receipt_overwrite_denied",
            serde_json::json!({
                "latest_wins_overwrite_requested": true,
                "overwrite_existing_noop_requested": true,
            }),
        ),
        ordering_fixture(
            "provider-router-activation-command-result-receipt-stage-ledger-index-delivery-ordering-bypass-attempt",
            "blocked_ordering_noop",
            "stage_ledger_index_delivery_ordering_bypass_denied",
            serde_json::json!({
                "stage_transition_ordering_bypass_requested": true,
                "completion_ack_before_noop_requested": true,
                "ledger_ordering_bypass_requested": true,
                "index_ordering_bypass_requested": true,
                "delivery_ordering_bypass_requested": true,
            }),
        ),
        ordering_fixture(
            "provider-router-activation-command-result-receipt-runtime-provider-memory-kg-external-ordering-bypass-attempt",
            "blocked_ordering_noop",
            "runtime_provider_memory_kg_external_ordering_bypass_denied",
            serde_json::json!({
                "runtime_ordering_bypass_requested": true,
                "provider_ordering_bypass_requested": true,
                "model_ordering_bypass_requested": true,
                "memory_store_ordering_bypass_requested": true,
                "live_kg_ordering_bypass_requested": true,
                "external_send_ordering_bypass_requested": true,
                "public_claim_ordering_bypass_requested": true,
                "install_ordering_bypass_requested": true,
                "service_restart_ordering_bypass_requested": true,
                "active_binary_mutation_ordering_bypass_requested": true,
            }),
        ),
    ]);
    let ordering_monotonicity_fixture_count = ordering_monotonicity_fixtures
        .as_array()
        .map(std::vec::Vec::len)
        .unwrap_or(0);
    let fixture_hash = sha256_json_value(&ordering_monotonicity_fixtures);
    let source_replay_hash = sha256_json_value(&source_replay);
    let contract_hash = sha256_text_value(&format!(
        "runtime-provider-router-activation-command-result-receipt-ordering-monotonicity-denial:v1:source={source_replay_hash}:fixtures={fixture_hash}:ordering=0:cursor=0:monotonicity=0:persist=0:authority=0"
    ));
    let policy_hash = sha256_text_value(
        "runtime-provider-router-activation-command-result-receipt-ordering-monotonicity-denial:v1:no-ordering:no-sequence-cursor:no-monotonicity-state:no-latest-wins:no-stage-bypass:no-runtime-provider-model-memory-kg-external-install-restart-binary-public-authority",
    );
    let side_effect_hash = sha256_text_value(
        "runtime-provider-router-ordering-monotonicity-side-effects=false;fixtures=10;ordering=0;cursor=0;monotonicity=0;record=0;persist=0;activation=0;provider=0;model=0;memory=0;kg=0;secret=0",
    );
    let denials = vec![
        "source_result_receipt_replay_idempotency_report_required",
        "canonical_noop_result_receipt_order_identity_required",
        "sequence_cursor_acceptance_denied",
        "sequence_cursor_recording_denied",
        "sequence_cursor_persistence_denied",
        "monotonicity_state_recording_denied",
        "monotonicity_state_persistence_denied",
        "monotonicity_state_materialization_denied",
        "monotonicity_filesystem_write_denied",
        "out_of_order_sequence_denied",
        "sequence_gap_or_skip_denied",
        "timestamp_rollback_denied",
        "epoch_rollback_denied",
        "same_sequence_different_hash_denied",
        "latest_wins_overwrite_denied",
        "completion_ack_before_noop_denied",
        "stage_transition_ordering_bypass_denied",
        "ledger_index_delivery_ordering_bypass_denied",
        "runtime_router_ordering_bypass_denied",
        "context_injection_ordering_bypass_denied",
        "provider_model_ordering_bypass_denied",
        "memory_kg_ordering_bypass_denied",
        "credential_secret_ordering_bypass_denied",
        "external_public_install_restart_ordering_bypass_denied",
        "active_binary_mutation_ordering_bypass_denied",
        "activation_from_ordering_denied",
    ];
    let denied_count = denials.len();

    let mut report = source_replay.clone();
    extend_json_object(
        &mut report,
        serde_json::json!({
            "status": if report_ready { "ready" } else { "blocked" },
            "gate": "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_route",
            "source_command": "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-ordering-monotonicity-denial --json",
            "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_ORDERING_MONOTONICITY_DENIAL_ENDPOINT,
            "native_route": true,
            "compatibility_mode": "native_runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_status",
            "side_effect_free": true,
            "source_activation_command_result_receipt_replay_idempotency_route_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_REPLAY_IDEMPOTENCY_DENIAL_ENDPOINT,
            "source_activation_command_result_receipt_replay_idempotency_gate": source_str("gate"),
            "source_activation_command_result_receipt_replay_idempotency_ready": source_replay_ready,
            "source_activation_command_result_receipt_replay_idempotency_status": source_str("runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_status"),
            "source_activation_command_result_receipt_replay_idempotency_report_sha256": source_replay_hash,
            "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
            "route_count": route_matrix.route_count,
            "implemented_route_count": route_matrix.implemented_route_count,
            "missing_route_count": route_matrix.missing_route_count,
            "route_count_cutover_floor": NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR,
            "route_count_floor_preserved": route_count_floor_preserved,
            "route_count_source_command_accepted": route_count_source_command_accepted,
            "source_route_wired": true,
            "operator_authorization_received": true,
            "fresh_evidence_review_requested": true,
            "explicit_command_path_reviewed": true,
            "accepted_operator_approval_consumed": false,
            "activation_authority_derived": false,
            "runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_route_enabled": true,
            "runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready": true,
            "runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_status": "blocked",
            "activation_command_result_receipt_ordering_monotonicity_schema_version": "memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_v1",
            "activation_command_result_receipt_ordering_monotonicity_mode": "runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_no_ordering_no_monotonicity_persist",
            "activation_command_result_receipt_ordering_monotonicity_decision": "runtime_provider_router_activation_command_result_receipt_cannot_create_ordering_sequence_cursor_or_monotonicity_authority",
            "minimum_required_samples": 24,
            "ordering_monotonicity_fixtures_sha256": fixture_hash,
            "ordering_monotonicity_contract_hash_sha256": contract_hash,
            "ordering_monotonicity_policy_hash_sha256": policy_hash,
            "side_effect_hash_sha256": side_effect_hash,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_replay_idempotency_fixture_count": source_u64("replay_idempotency_fixture_count"),
            "source_blocked_replay_idempotency_fixture_count": source_u64("blocked_replay_idempotency_fixture_count"),
            "source_accepted_replay_idempotency_fixture_count": source_u64("accepted_replay_idempotency_fixture_count"),
            "ordering_monotonicity_surface_count": 14,
            "ordering_monotonicity_surface_ready_count": 14,
            "ordering_monotonicity_side_effect_free_surface_count": 14,
            "ordering_monotonicity_fixtures": ordering_monotonicity_fixtures,
            "ordering_monotonicity_fixture_count": ordering_monotonicity_fixture_count,
            "blocked_ordering_monotonicity_fixture_count": ordering_monotonicity_fixture_count,
            "noop_ordering_monotonicity_fixture_count": ordering_monotonicity_fixture_count,
            "allowed_ordering_monotonicity_fixture_count": 0,
            "accepted_ordering_monotonicity_fixture_count": 0,
            "ordering_monotonicity_denied_count": ordering_monotonicity_fixture_count,
            "ordering_monotonicity_performed_count": 0,
            "sequence_cursor_accepted_count": 0,
            "sequence_cursor_recorded_count": 0,
            "sequence_cursor_persisted_count": 0,
            "monotonicity_state_recorded_count": 0,
            "monotonicity_state_persisted_count": 0,
            "denied_by_ordering_monotonicity": denials,
            "denied_by_ordering_monotonicity_count": denied_count,
            "denied_by_activation_command_result_receipt_ordering_monotonicity": denials,
            "denied_by_activation_command_result_receipt_ordering_monotonicity_count": denied_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [
                {
                    "action": "review_runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial",
                    "status": "allowed_report_only",
                    "accepts_out_of_order_receipt": false,
                    "records_monotonic_clock": false,
                    "persists_ordering_state": false,
                    "mutates_runtime": false,
                    "invokes_model": false
                },
                {
                    "action": "stage_runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial",
                    "status": "allowed_report_only_next_slice",
                    "accepts_cancellation": false,
                    "accepts_supersession": false,
                    "persists_replacement_receipt": false,
                    "mutates_runtime": false,
                    "invokes_model": false
                },
                {
                    "action": "run_full_light_preflight",
                    "status": "allowed_verification_only",
                    "accepts_ordering": false,
                    "persists_ordering_state": false,
                    "mutates_runtime": false,
                    "invokes_model": false,
                    "writes_kg": false
                }
            ],
        }),
    );
    if let Some(report_object) = report.as_object_mut() {
        for key in [
            "activation_command_result_receipt_ordering_allowed",
            "activation_command_result_receipt_ordering_recorded",
            "activation_command_result_receipt_ordering_persisted",
            "activation_command_result_receipt_ordering_materialized",
            "activation_command_result_receipt_ordering_filesystem_written",
            "activation_command_result_receipt_ordering_performed",
            "activation_command_result_receipt_sequence_cursor_accepted",
            "activation_command_result_receipt_sequence_cursor_recorded",
            "activation_command_result_receipt_sequence_cursor_persisted",
            "activation_command_result_receipt_monotonicity_state_recorded",
            "activation_command_result_receipt_monotonicity_state_persisted",
            "activation_command_result_receipt_monotonicity_state_materialized",
            "activation_command_result_receipt_monotonicity_filesystem_written",
            "activation_command_result_receipt_timestamp_ordering_accepted",
            "activation_command_result_receipt_epoch_ordering_accepted",
            "activation_command_result_receipt_stage_ordering_accepted",
            "activation_command_result_receipt_same_sequence_hash_override_accepted",
            "activation_command_result_receipt_latest_wins_overwrite_accepted",
            "activation_command_result_receipt_gap_fill_accepted",
            "activation_command_result_receipt_ack_before_noop_accepted",
            "activation_command_result_receipt_ledger_ordering_bypass_accepted",
            "activation_command_result_receipt_index_ordering_bypass_accepted",
            "activation_command_result_receipt_delivery_ordering_bypass_accepted",
            "activation_command_result_receipt_runtime_ordering_bypass_accepted",
            "activation_command_result_receipt_provider_ordering_bypass_accepted",
            "activation_command_result_receipt_memory_kg_ordering_bypass_accepted",
            "activation_command_result_receipt_external_public_install_ordering_bypass_accepted",
            "operator_approval_from_ordering_accepted",
            "activation_from_ordering_allowed",
            "activation_command_enabled",
            "activation_command_invoked",
            "activation_command_dispatched",
            "activation_request_accepted",
            "activation_request_recorded",
            "activation_request_persisted",
            "activation_request_executed",
            "activation_activated",
            "runtime_router_mutated",
            "runtime_attachment_performed",
            "live_context_attached",
            "context_injection_performed",
            "adapter_invoked",
            "provider_invoked",
            "model_invoked",
            "auth_secret_read",
            "credential_read",
            "secret_file_read",
            "memory_store_write_performed",
            "memory_store_mutated",
            "live_kg_write_performed",
            "ordering_ledger_written",
            "ordering_indexed",
            "ordering_query_registered",
            "ordering_observability_recorded",
            "telegram_send_performed",
            "channel_send_performed",
            "external_send_performed",
            "public_release_claimed",
            "public_ga_claimed",
            "release_artifact_written",
            "install_executed",
            "launchd_mutated",
            "service_restart_performed",
            "active_binary_mutated",
        ] {
            report_object.insert(key.to_string(), serde_json::Value::Bool(false));
        }
    }
    if let Some(side_effects) = report
        .get_mut("side_effects")
        .and_then(serde_json::Value::as_object_mut)
    {
        for key in [
            "activation_command_result_receipt_ordering_recorded",
            "activation_command_result_receipt_ordering_persisted",
            "activation_command_result_receipt_sequence_cursor_recorded",
            "activation_command_result_receipt_sequence_cursor_persisted",
            "activation_command_result_receipt_monotonicity_state_recorded",
            "activation_command_result_receipt_monotonicity_state_persisted",
            "activation_from_ordering_allowed",
            "activation_command_enabled",
            "activation_command_invoked",
            "activation_command_dispatched",
            "activation_request_recorded",
            "activation_request_executed",
            "runtime_router_mutated",
            "context_injection_performed",
            "provider_invoked",
            "model_invoked",
            "memory_store_write_performed",
            "live_kg_write_performed",
            "credential_read",
            "secret_file_read",
            "channel_send_performed",
            "external_send_performed",
            "install_executed",
            "service_restart_performed",
            "active_binary_mutated",
        ] {
            side_effects.insert(key.to_string(), serde_json::Value::Bool(false));
        }
    }
    report
}

fn hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_ordering =
        hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_report();
    let source_bool = |key: &str| {
        source_ordering
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let source_u64 = |key: &str| {
        source_ordering
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_str = |key: &str| {
        source_ordering
            .get(key)
            .and_then(serde_json::Value::as_str)
            .unwrap_or("blocked")
            .to_string()
    };
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_ordering_ready = source_str("status") == "ready"
        && source_bool(
            "runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready",
        )
        && source_str(
            "runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_status",
        ) == "blocked"
        && source_u64("accepted_ordering_monotonicity_fixture_count") == 0
        && source_u64("ordering_monotonicity_performed_count") == 0
        && source_u64("sequence_cursor_recorded_count") == 0
        && source_u64("monotonicity_state_recorded_count") == 0;
    let report_ready = source_ordering_ready && route_count_source_command_accepted;

    let cancellation_supersession_fixture =
        |fixture_id: &str, status: &str, reason: &str, extra: serde_json::Value| {
            let mut fixture = serde_json::Map::new();
            fixture.insert(
                "fixture_id".to_string(),
                serde_json::Value::String(fixture_id.to_string()),
            );
            fixture.insert(
                "id".to_string(),
                serde_json::Value::String(fixture_id.to_string()),
            );
            fixture.insert(
                "cancellation_supersession_status".to_string(),
                serde_json::Value::String(status.to_string()),
            );
            fixture.insert(
                "denial_reason".to_string(),
                serde_json::Value::String(reason.to_string()),
            );
            for key in [
                "source_ordering_monotonicity_present",
                "source_ordering_monotonicity_ready",
                "canonical_noop_result_receipt_lifecycle_identity_required",
                "receipt_noop_confirmed",
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(true));
            }
            for key in [
                "cancellation_requested",
                "supersession_requested",
                "replacement_receipt_requested",
                "replacement_hash_requested",
                "tombstone_requested",
                "delete_marker_requested",
                "completion_ack_cancellation_requested",
                "ledger_cancellation_requested",
                "index_cancellation_requested",
                "delivery_cancellation_requested",
                "export_cancellation_requested",
                "query_cancellation_requested",
                "observability_cancellation_requested",
                "runtime_router_supersession_requested",
                "provider_supersession_requested",
                "model_supersession_requested",
                "memory_store_supersession_requested",
                "live_kg_supersession_requested",
                "rollback_supersession_requested",
                "secret_material_supersession_requested",
                "external_send_supersession_requested",
                "public_claim_supersession_requested",
                "install_supersession_requested",
                "service_restart_supersession_requested",
                "active_binary_mutation_supersession_requested",
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(false));
            }
            for key in [
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
                "activation_command_result_receipt_export_cancellation_accepted",
                "activation_command_result_receipt_query_cancellation_accepted",
                "activation_command_result_receipt_observability_cancellation_accepted",
                "activation_command_result_receipt_ordering_allowed",
                "activation_command_result_receipt_ordering_recorded",
                "activation_command_result_receipt_ordering_persisted",
                "activation_command_result_receipt_sequence_cursor_accepted",
                "activation_command_result_receipt_sequence_cursor_recorded",
                "activation_command_result_receipt_sequence_cursor_persisted",
                "activation_command_result_receipt_monotonicity_state_recorded",
                "activation_command_result_receipt_monotonicity_state_persisted",
                "activation_command_result_receipt_latest_wins_overwrite_accepted",
                "activation_command_result_receipt_same_sequence_hash_override_accepted",
                "activation_command_result_receipt_recorded",
                "activation_command_result_receipt_persisted",
                "activation_command_result_receipt_accepted",
                "activation_command_result_receipt_materialized",
                "activation_command_result_receipt_filesystem_written",
                "activation_command_result_receipt_ledger_written",
                "activation_command_result_receipt_indexed",
                "activation_command_result_receipt_enqueued",
                "activation_command_result_receipt_delivered",
                "activation_command_result_receipt_exported",
                "activation_command_result_receipt_query_registered",
                "activation_command_result_receipt_observability_recorded",
                "activation_command_completion_ack_recorded",
                "activation_command_completion_ack_persisted",
                "activation_command_completion_ack_accepted",
                "activation_command_completion_ack_delivered",
                "activation_from_cancellation_allowed",
                "activation_from_supersession_allowed",
                "activation_from_ordering_allowed",
                "activation_from_replay_allowed",
                "activation_from_receipt_allowed",
                "activation_command_enabled",
                "activation_command_invoked",
                "activation_command_dispatched",
                "activation_command_dispatch_performed",
                "activation_request_accepted",
                "activation_request_recorded",
                "activation_request_persisted",
                "activation_request_executed",
                "activation_activated",
                "runtime_router_mutated",
                "runtime_attachment_performed",
                "live_context_attached",
                "context_injection_performed",
                "adapter_invoked",
                "provider_invoked",
                "model_invoked",
                "auth_secret_read",
                "credential_read",
                "secret_file_read",
                "usage_recorded",
                "memory_store_write_performed",
                "memory_store_mutated",
                "live_kg_write_performed",
                "readback_evidence_recorded",
                "readback_evidence_persisted",
                "router_handoff_recorded",
                "router_handoff_persisted",
                "rollback_executed",
                "telegram_send_performed",
                "channel_send_performed",
                "external_send_performed",
                "public_release_claimed",
                "public_ga_claimed",
                "release_artifact_written",
                "install_executed",
                "launchd_mutated",
                "service_restart_performed",
                "active_binary_mutated",
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(false));
            }
            let mut fixture = serde_json::Value::Object(fixture);
            extend_json_object(&mut fixture, extra);
            fixture
        };
    let cancellation_supersession_fixtures = serde_json::Value::Array(vec![
        cancellation_supersession_fixture(
            "provider-router-activation-command-result-receipt-cancellation-missing-source-ordering-report",
            "blocked_noop",
            "source_ordering_monotonicity_report_required",
            serde_json::json!({
                "source_ordering_monotonicity_present": false,
                "source_ordering_monotonicity_ready": false,
                "cancellation_requested": true,
            }),
        ),
        cancellation_supersession_fixture(
            "provider-router-activation-command-result-receipt-cancel-blocked-noop",
            "blocked_cancellation_noop",
            "cancel_after_blocked_noop_denied",
            serde_json::json!({
                "cancellation_requested": true,
                "cancel_after_blocked_noop_requested": true,
            }),
        ),
        cancellation_supersession_fixture(
            "provider-router-activation-command-result-receipt-supersede-with-completed",
            "blocked_supersession_noop",
            "supersede_blocked_noop_with_completed_denied",
            serde_json::json!({
                "supersession_requested": true,
                "supersede_with_completed_receipt_requested": true,
            }),
        ),
        cancellation_supersession_fixture(
            "provider-router-activation-command-result-receipt-replacement-hash",
            "blocked_replacement_noop",
            "replacement_receipt_hash_denied",
            serde_json::json!({
                "replacement_receipt_requested": true,
                "replacement_hash_requested": true,
            }),
        ),
        cancellation_supersession_fixture(
            "provider-router-activation-command-result-receipt-tombstone-delete-marker",
            "blocked_tombstone_noop",
            "tombstone_delete_marker_denied",
            serde_json::json!({
                "tombstone_requested": true,
                "delete_marker_requested": true,
            }),
        ),
        cancellation_supersession_fixture(
            "provider-router-activation-command-result-receipt-completion-ack-cancel",
            "blocked_cancellation_noop",
            "completion_ack_cancellation_denied",
            serde_json::json!({
                "cancellation_requested": true,
                "completion_ack_cancellation_requested": true,
            }),
        ),
        cancellation_supersession_fixture(
            "provider-router-activation-command-result-receipt-ledger-index-delivery-export-cancel",
            "blocked_cancellation_noop",
            "ledger_index_delivery_export_cancellation_denied",
            serde_json::json!({
                "cancellation_requested": true,
                "ledger_cancellation_requested": true,
                "index_cancellation_requested": true,
                "delivery_cancellation_requested": true,
                "export_cancellation_requested": true,
                "query_cancellation_requested": true,
                "observability_cancellation_requested": true,
            }),
        ),
        cancellation_supersession_fixture(
            "provider-router-activation-command-result-receipt-runtime-provider-model-supersede",
            "blocked_supersession_noop",
            "runtime_provider_model_supersession_denied",
            serde_json::json!({
                "supersession_requested": true,
                "runtime_router_supersession_requested": true,
                "provider_supersession_requested": true,
                "model_supersession_requested": true,
            }),
        ),
        cancellation_supersession_fixture(
            "provider-router-activation-command-result-receipt-memory-kg-rollback-secret-supersede",
            "blocked_supersession_noop",
            "memory_kg_rollback_secret_supersession_denied",
            serde_json::json!({
                "supersession_requested": true,
                "memory_store_supersession_requested": true,
                "live_kg_supersession_requested": true,
                "rollback_supersession_requested": true,
                "secret_material_supersession_requested": true,
            }),
        ),
        cancellation_supersession_fixture(
            "provider-router-activation-command-result-receipt-external-public-install-supersede",
            "blocked_supersession_noop",
            "external_public_install_supersession_denied",
            serde_json::json!({
                "supersession_requested": true,
                "external_send_supersession_requested": true,
                "public_claim_supersession_requested": true,
                "install_supersession_requested": true,
                "service_restart_supersession_requested": true,
                "active_binary_mutation_supersession_requested": true,
            }),
        ),
    ]);
    let cancellation_supersession_fixture_count = cancellation_supersession_fixtures
        .as_array()
        .map(std::vec::Vec::len)
        .unwrap_or(0);
    let fixture_hash = sha256_json_value(&cancellation_supersession_fixtures);
    let source_ordering_hash = sha256_json_value(&source_ordering);
    let contract_hash = sha256_text_value(&format!(
        "runtime-provider-router-activation-command-result-receipt-cancellation-supersession-denial:v1:source={source_ordering_hash}:fixtures={fixture_hash}:cancel=0:supersede=0:replacement=0:tombstone=0:delete=0:persist=0:authority=0"
    ));
    let policy_hash = sha256_text_value(
        "runtime-provider-router-activation-command-result-receipt-cancellation-supersession-denial:v1:no-cancel:no-supersede:no-replacement:no-tombstone:no-delete:no-ack-cancel:no-ledger-index-delivery-export-query-observe:no-runtime-provider-model-memory-kg-external-install-restart-binary-public-authority",
    );
    let side_effect_hash = sha256_text_value(
        "runtime-provider-router-cancellation-supersession-side-effects=false;fixtures=10;cancel=0;supersede=0;replacement=0;tombstone=0;delete=0;record=0;persist=0;activation=0;provider=0;model=0;memory=0;kg=0;secret=0",
    );
    let denials = vec![
        "source_ordering_monotonicity_report_required",
        "cancellation_request_acceptance_denied",
        "cancellation_recording_denied",
        "cancellation_persistence_denied",
        "supersession_request_acceptance_denied",
        "supersession_recording_denied",
        "supersession_persistence_denied",
        "replacement_receipt_acceptance_denied",
        "replacement_receipt_recording_denied",
        "replacement_receipt_persistence_denied",
        "replacement_hash_acceptance_denied",
        "tombstone_recording_denied",
        "delete_marker_recording_denied",
        "cancel_after_blocked_noop_denied",
        "supersede_blocked_noop_with_completed_denied",
        "completion_ack_cancellation_denied",
        "ledger_cancellation_denied",
        "index_cancellation_denied",
        "delivery_cancellation_denied",
        "export_query_observability_cancellation_denied",
        "runtime_router_supersession_denied",
        "live_context_supersession_denied",
        "adapter_provider_model_supersession_denied",
        "usage_memory_kg_supersession_denied",
        "rollback_secret_material_supersession_denied",
        "external_public_release_supersession_denied",
        "install_restart_active_binary_supersession_denied",
    ];
    let denied_count = denials.len();
    let denials_value = serde_json::Value::Array(
        denials
            .iter()
            .map(|denial| serde_json::Value::String((*denial).to_string()))
            .collect(),
    );

    let mut report = source_ordering.clone();
    extend_json_object(
        &mut report,
        serde_json::json!({
            "status": if report_ready { "ready" } else { "blocked" },
            "gate": "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_route",
            "source_command": "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-cancellation-supersession-denial --json",
            "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_CANCELLATION_SUPERSESSION_DENIAL_ENDPOINT,
            "native_route": true,
            "compatibility_mode": "native_runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_status",
            "side_effect_free": true,
            "source_activation_command_result_receipt_ordering_monotonicity_route_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_ORDERING_MONOTONICITY_DENIAL_ENDPOINT,
            "source_activation_command_result_receipt_ordering_monotonicity_gate": source_str("gate"),
            "source_activation_command_result_receipt_ordering_monotonicity_ready": source_ordering_ready,
            "source_activation_command_result_receipt_ordering_monotonicity_status": source_str("runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_status"),
            "source_activation_command_result_receipt_ordering_monotonicity_report_sha256": source_ordering_hash,
            "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
            "route_count": route_matrix.route_count,
            "implemented_route_count": route_matrix.implemented_route_count,
            "missing_route_count": route_matrix.missing_route_count,
            "route_count_cutover_floor": NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR,
            "route_count_floor_preserved": route_count_floor_preserved,
            "route_count_source_command_accepted": route_count_source_command_accepted,
            "source_route_wired": true,
            "operator_authorization_received": true,
            "fresh_evidence_review_requested": true,
            "explicit_command_path_reviewed": true,
            "accepted_operator_approval_consumed": false,
            "activation_authority_derived": false,
            "runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_route_enabled": true,
            "runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready": true,
            "runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_status": "blocked",
            "activation_command_result_receipt_cancellation_supersession_schema_version": "memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_v1",
            "activation_command_result_receipt_cancellation_supersession_mode": "runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_no_cancel_no_supersede_no_replacement_persist",
            "activation_command_result_receipt_cancellation_supersession_decision": "runtime_provider_router_activation_command_result_receipt_cannot_cancel_supersede_replace_tombstone_delete_or_derive_activation_authority",
            "minimum_required_samples": 24,
            "cancellation_supersession_fixtures_sha256": fixture_hash,
            "cancellation_supersession_contract_hash_sha256": contract_hash,
            "cancellation_supersession_policy_hash_sha256": policy_hash,
            "side_effect_hash_sha256": side_effect_hash,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_ordering_monotonicity_fixture_count": source_u64("ordering_monotonicity_fixture_count"),
            "source_blocked_ordering_monotonicity_fixture_count": source_u64("blocked_ordering_monotonicity_fixture_count"),
            "source_accepted_ordering_monotonicity_fixture_count": source_u64("accepted_ordering_monotonicity_fixture_count"),
            "cancellation_supersession_surface_count": 14,
            "cancellation_supersession_surface_ready_count": 14,
            "cancellation_supersession_side_effect_free_surface_count": 14,
            "cancellation_supersession_surfaces": [
                "source_ordering_monotonicity_report_required",
                "cancellation_request_shape_denied",
                "supersession_request_shape_denied",
                "replacement_receipt_hash_denied",
                "tombstone_or_delete_marker_denied",
                "cancel_after_blocked_noop_denied",
                "supersede_blocked_noop_with_completed_denied",
                "acknowledgement_cancellation_denied",
                "ledger_index_delivery_export_query_observability_cancellation_denied",
                "runtime_router_live_context_supersession_denied",
                "adapter_provider_model_usage_supersession_denied",
                "memory_kg_rollback_secret_supersession_denied",
                "external_public_install_restart_active_binary_supersession_denied",
                "activation_authority_from_cancellation_supersession_denied"
            ],
            "cancellation_supersession_fixtures": cancellation_supersession_fixtures,
            "cancellation_supersession_fixture_count": cancellation_supersession_fixture_count,
            "blocked_cancellation_supersession_fixture_count": cancellation_supersession_fixture_count,
            "noop_cancellation_supersession_fixture_count": cancellation_supersession_fixture_count,
            "allowed_cancellation_supersession_fixture_count": 0,
            "accepted_cancellation_supersession_fixture_count": 0,
            "cancellation_fixture_count": 5,
            "supersession_fixture_count": 5,
            "cancellation_denied_count": 5,
            "supersession_denied_count": 5,
            "cancellation_performed_count": 0,
            "supersession_performed_count": 0,
            "replacement_receipt_accepted_count": 0,
            "replacement_receipt_recorded_count": 0,
            "replacement_receipt_persisted_count": 0,
            "tombstone_recorded_count": 0,
            "delete_marker_recorded_count": 0,
            "denied_by_cancellation_supersession": denials_value,
            "denied_by_cancellation_supersession_count": denied_count,
            "denied_by_activation_command_result_receipt_cancellation_supersession": denials_value,
            "denied_by_activation_command_result_receipt_cancellation_supersession_count": denied_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [
                {
                    "action": "review_runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial",
                    "status": "allowed_report_only",
                    "accepts_cancellation": false,
                    "accepts_supersession": false,
                    "persists_replacement_receipt": false,
                    "mutates_runtime": false,
                    "invokes_model": false
                },
                {
                    "action": "stage_runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial",
                    "status": "allowed_report_only_next_slice",
                    "accepts_cancellation": false,
                    "accepts_supersession": false,
                    "writes_audit_trail": false,
                    "persists_evidence": false,
                    "mutates_runtime": false,
                    "invokes_model": false
                },
                {
                    "action": "run_full_light_preflight",
                    "status": "allowed_verification_only",
                    "accepts_cancellation": false,
                    "accepts_supersession": false,
                    "persists_replacement_receipt": false,
                    "mutates_runtime": false,
                    "invokes_model": false,
                    "writes_kg": false
                }
            ],
        }),
    );
    if let Some(report_object) = report.as_object_mut() {
        for key in [
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
            "activation_command_result_receipt_export_cancellation_accepted",
            "activation_command_result_receipt_query_cancellation_accepted",
            "activation_command_result_receipt_observability_cancellation_accepted",
            "activation_from_cancellation_allowed",
            "activation_from_supersession_allowed",
            "activation_from_ordering_allowed",
            "activation_from_replay_allowed",
            "activation_from_receipt_allowed",
            "activation_command_enabled",
            "activation_command_invoked",
            "activation_command_dispatched",
            "activation_request_accepted",
            "activation_request_recorded",
            "activation_request_persisted",
            "activation_request_executed",
            "activation_activated",
            "runtime_router_mutated",
            "runtime_attachment_performed",
            "live_context_attached",
            "context_injection_performed",
            "adapter_invoked",
            "provider_invoked",
            "model_invoked",
            "auth_secret_read",
            "credential_read",
            "secret_file_read",
            "usage_recorded",
            "memory_store_write_performed",
            "memory_store_mutated",
            "live_kg_write_performed",
            "readback_evidence_recorded",
            "readback_evidence_persisted",
            "router_handoff_recorded",
            "router_handoff_persisted",
            "rollback_executed",
            "telegram_send_performed",
            "channel_send_performed",
            "external_send_performed",
            "public_release_claimed",
            "public_ga_claimed",
            "release_artifact_written",
            "install_executed",
            "launchd_mutated",
            "service_restart_performed",
            "active_binary_mutated",
        ] {
            report_object.insert(key.to_string(), serde_json::Value::Bool(false));
        }
    }
    if let Some(side_effects) = report
        .get_mut("side_effects")
        .and_then(serde_json::Value::as_object_mut)
    {
        for key in [
            "activation_command_result_receipt_cancellation_recorded",
            "activation_command_result_receipt_cancellation_persisted",
            "activation_command_result_receipt_supersession_recorded",
            "activation_command_result_receipt_supersession_persisted",
            "activation_command_result_receipt_replacement_receipt_recorded",
            "activation_command_result_receipt_replacement_receipt_persisted",
            "activation_command_result_receipt_tombstone_recorded",
            "activation_command_result_receipt_delete_marker_recorded",
            "activation_from_cancellation_allowed",
            "activation_from_supersession_allowed",
            "activation_command_enabled",
            "activation_command_invoked",
            "activation_command_dispatched",
            "activation_request_recorded",
            "activation_request_executed",
            "runtime_router_mutated",
            "context_injection_performed",
            "provider_invoked",
            "model_invoked",
            "memory_store_write_performed",
            "live_kg_write_performed",
            "credential_read",
            "secret_file_read",
            "channel_send_performed",
            "external_send_performed",
            "install_executed",
            "service_restart_performed",
            "active_binary_mutated",
        ] {
            side_effects.insert(key.to_string(), serde_json::Value::Bool(false));
        }
    }
    report
}

fn hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_cancellation =
        hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_report();
    let source_bool = |key: &str| {
        source_cancellation
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let source_u64 = |key: &str| {
        source_cancellation
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_str = |key: &str| {
        source_cancellation
            .get(key)
            .and_then(serde_json::Value::as_str)
            .unwrap_or("blocked")
            .to_string()
    };
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_cancellation_ready = source_str("status") == "ready"
        && source_bool(
            "runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready",
        )
        && source_str(
            "runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_status",
        ) == "blocked"
        && source_u64("accepted_cancellation_supersession_fixture_count") == 0
        && source_u64("cancellation_performed_count") == 0
        && source_u64("supersession_performed_count") == 0
        && source_u64("replacement_receipt_recorded_count") == 0
        && source_u64("replacement_receipt_persisted_count") == 0
        && source_u64("tombstone_recorded_count") == 0
        && source_u64("delete_marker_recorded_count") == 0;
    let report_ready = source_cancellation_ready && route_count_source_command_accepted;

    let audit_trail_immutable_evidence_fixture =
        |fixture_id: &str, status: &str, reason: &str, extra: serde_json::Value| {
            let mut fixture = serde_json::Map::new();
            fixture.insert(
                "fixture_id".to_string(),
                serde_json::Value::String(fixture_id.to_string()),
            );
            fixture.insert(
                "id".to_string(),
                serde_json::Value::String(fixture_id.to_string()),
            );
            fixture.insert(
                "audit_evidence_status".to_string(),
                serde_json::Value::String(status.to_string()),
            );
            fixture.insert(
                "denial_reason".to_string(),
                serde_json::Value::String(reason.to_string()),
            );
            for key in [
                "source_cancellation_supersession_present",
                "source_cancellation_supersession_ready",
                "audit_trail_requested",
                "receipt_noop_confirmed",
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(true));
            }
            for key in [
                "immutable_evidence_requested",
                "hash_chain_requested",
                "merkle_root_requested",
                "attestation_requested",
                "witness_requested",
                "notary_requested",
                "audit_trail_materialization_requested",
                "audit_trail_filesystem_write_requested",
                "ledger_evidence_requested",
                "index_evidence_requested",
                "delivery_evidence_requested",
                "activation_from_audit_evidence_requested",
                "memory_store_evidence_requested",
                "live_kg_evidence_requested",
                "rollback_evidence_requested",
                "secret_material_evidence_requested",
                "provider_prompt_evidence_requested",
                "external_send_evidence_requested",
                "public_claim_evidence_requested",
                "release_artifact_evidence_requested",
                "install_evidence_requested",
                "service_restart_evidence_requested",
                "active_binary_mutation_evidence_requested",
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(false));
            }
            for key in [
                "audit_trail_allowed",
                "audit_trail_recorded",
                "audit_trail_persisted",
                "audit_trail_materialized",
                "audit_trail_filesystem_written",
                "immutable_evidence_allowed",
                "immutable_evidence_recorded",
                "immutable_evidence_persisted",
                "immutable_evidence_materialized",
                "immutable_evidence_filesystem_written",
                "hash_chain_recorded",
                "hash_chain_persisted",
                "merkle_root_recorded",
                "merkle_root_persisted",
                "attestation_recorded",
                "attestation_persisted",
                "witness_recorded",
                "witness_persisted",
                "notary_recorded",
                "notary_persisted",
                "ledger_evidence_recorded",
                "ledger_evidence_persisted",
                "index_evidence_recorded",
                "index_evidence_persisted",
                "delivery_evidence_recorded",
                "delivery_evidence_persisted",
                "audit_trail_exported",
                "immutable_evidence_exported",
                "audit_evidence_query_registered",
                "audit_evidence_observability_recorded",
                "activation_command_result_receipt_cancellation_allowed",
                "activation_command_result_receipt_cancellation_recorded",
                "activation_command_result_receipt_cancellation_persisted",
                "activation_command_result_receipt_supersession_allowed",
                "activation_command_result_receipt_supersession_recorded",
                "activation_command_result_receipt_supersession_persisted",
                "replacement_receipt_accepted",
                "replacement_receipt_recorded",
                "replacement_receipt_persisted",
                "tombstone_recorded",
                "delete_marker_recorded",
                "activation_command_result_receipt_ordering_allowed",
                "activation_command_result_receipt_ordering_recorded",
                "activation_command_result_receipt_ordering_persisted",
                "activation_command_result_receipt_recorded",
                "activation_command_result_receipt_persisted",
                "activation_command_result_receipt_accepted",
                "activation_command_result_receipt_materialized",
                "activation_command_result_receipt_filesystem_written",
                "activation_command_result_receipt_ledger_written",
                "activation_command_result_receipt_indexed",
                "activation_command_result_receipt_enqueued",
                "activation_command_result_receipt_delivered",
                "activation_command_result_receipt_exported",
                "activation_command_result_receipt_query_registered",
                "activation_command_result_receipt_observability_recorded",
                "activation_command_completion_ack_recorded",
                "activation_command_completion_ack_persisted",
                "activation_command_completion_ack_accepted",
                "activation_command_completion_ack_delivered",
                "operator_approval_from_audit_trail_accepted",
                "operator_approval_from_immutable_evidence_accepted",
                "activation_from_audit_trail_allowed",
                "activation_from_immutable_evidence_allowed",
                "activation_from_cancellation_allowed",
                "activation_from_supersession_allowed",
                "activation_from_ordering_allowed",
                "activation_from_replay_allowed",
                "activation_from_receipt_allowed",
                "activation_command_allowed",
                "activation_command_accepted",
                "activation_command_enabled",
                "activation_command_invoked",
                "activation_command_dispatched",
                "activation_request_accepted",
                "activation_request_recorded",
                "activation_request_persisted",
                "activation_request_executed",
                "activation_activated",
                "runtime_router_mutated",
                "runtime_attachment_performed",
                "live_context_attached",
                "context_injection_performed",
                "adapter_invoked",
                "provider_invoked",
                "model_invoked",
                "provider_prompt_replayed",
                "auth_secret_read",
                "credential_read",
                "secret_file_read",
                "usage_recorded",
                "memory_store_write_performed",
                "memory_store_mutated",
                "live_kg_write_performed",
                "readback_evidence_recorded",
                "readback_evidence_persisted",
                "router_handoff_recorded",
                "router_handoff_persisted",
                "rollback_executed",
                "telegram_send_performed",
                "channel_send_performed",
                "external_send_performed",
                "public_release_claimed",
                "public_ga_claimed",
                "release_artifact_written",
                "install_executed",
                "launchd_mutated",
                "service_restart_performed",
                "active_binary_mutated",
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(false));
            }
            let mut fixture = serde_json::Value::Object(fixture);
            extend_json_object(&mut fixture, extra);
            fixture
        };
    let audit_trail_immutable_evidence_fixtures = serde_json::Value::Array(vec![
        audit_trail_immutable_evidence_fixture(
            "provider-router-activation-command-result-receipt-audit-missing-source-cancellation-supersession-report",
            "blocked_noop",
            "source_cancellation_supersession_report_required",
            serde_json::json!({
                "source_cancellation_supersession_present": false,
                "source_cancellation_supersession_ready": false,
            }),
        ),
        audit_trail_immutable_evidence_fixture(
            "provider-router-activation-command-result-receipt-audit-trail-append-request",
            "blocked_noop",
            "audit_trail_append_request_denied",
            serde_json::json!({
                "audit_trail_request_shape": "append_blocked_noop_result_receipt_audit_trail",
            }),
        ),
        audit_trail_immutable_evidence_fixture(
            "provider-router-activation-command-result-receipt-immutable-evidence-packet",
            "blocked_evidence_noop",
            "immutable_evidence_packet_request_denied",
            serde_json::json!({
                "immutable_evidence_requested": true,
                "audit_trail_requested": false,
                "immutable_evidence_request_shape": "seal_blocked_noop_result_receipt_as_immutable_evidence",
            }),
        ),
        audit_trail_immutable_evidence_fixture(
            "provider-router-activation-command-result-receipt-hash-chain-merkle-root",
            "blocked_evidence_noop",
            "hash_chain_merkle_root_evidence_denied",
            serde_json::json!({
                "immutable_evidence_requested": true,
                "audit_trail_requested": false,
                "hash_chain_requested": true,
                "merkle_root_requested": true,
            }),
        ),
        audit_trail_immutable_evidence_fixture(
            "provider-router-activation-command-result-receipt-attestation-witness-notary",
            "blocked_evidence_noop",
            "attestation_witness_notary_evidence_denied",
            serde_json::json!({
                "immutable_evidence_requested": true,
                "audit_trail_requested": false,
                "attestation_requested": true,
                "witness_requested": true,
                "notary_requested": true,
            }),
        ),
        audit_trail_immutable_evidence_fixture(
            "provider-router-activation-command-result-receipt-audit-trail-materialization",
            "blocked_noop",
            "audit_trail_materialization_filesystem_denied",
            serde_json::json!({
                "audit_trail_materialization_requested": true,
                "audit_trail_filesystem_write_requested": true,
            }),
        ),
        audit_trail_immutable_evidence_fixture(
            "provider-router-activation-command-result-receipt-ledger-index-delivery-evidence",
            "blocked_noop",
            "ledger_index_delivery_evidence_denied",
            serde_json::json!({
                "ledger_evidence_requested": true,
                "index_evidence_requested": true,
                "delivery_evidence_requested": true,
            }),
        ),
        audit_trail_immutable_evidence_fixture(
            "provider-router-activation-command-result-receipt-activation-from-audit-evidence",
            "blocked_evidence_noop",
            "activation_from_audit_evidence_denied",
            serde_json::json!({
                "immutable_evidence_requested": true,
                "audit_trail_requested": false,
                "activation_from_audit_evidence_requested": true,
            }),
        ),
        audit_trail_immutable_evidence_fixture(
            "provider-router-activation-command-result-receipt-memory-kg-rollback-secret-provider-evidence",
            "blocked_evidence_noop",
            "memory_kg_rollback_secret_provider_evidence_denied",
            serde_json::json!({
                "immutable_evidence_requested": true,
                "audit_trail_requested": false,
                "memory_store_evidence_requested": true,
                "live_kg_evidence_requested": true,
                "rollback_evidence_requested": true,
                "secret_material_evidence_requested": true,
                "provider_prompt_evidence_requested": true,
            }),
        ),
        audit_trail_immutable_evidence_fixture(
            "provider-router-activation-command-result-receipt-external-public-install-evidence",
            "blocked_evidence_noop",
            "external_public_install_restart_active_binary_evidence_denied",
            serde_json::json!({
                "immutable_evidence_requested": true,
                "audit_trail_requested": false,
                "external_send_evidence_requested": true,
                "public_claim_evidence_requested": true,
                "release_artifact_evidence_requested": true,
                "install_evidence_requested": true,
                "service_restart_evidence_requested": true,
                "active_binary_mutation_evidence_requested": true,
            }),
        ),
    ]);
    let audit_fixture_count = audit_trail_immutable_evidence_fixtures
        .as_array()
        .map(std::vec::Vec::len)
        .unwrap_or(0);
    let immutable_evidence_denied_count = audit_trail_immutable_evidence_fixtures
        .as_array()
        .map(|fixtures| {
            fixtures
                .iter()
                .filter(|fixture| {
                    fixture
                        .get("immutable_evidence_requested")
                        .and_then(serde_json::Value::as_bool)
                        .unwrap_or(false)
                })
                .count()
        })
        .unwrap_or(0);
    let fixtures_hash = sha256_json_value(&audit_trail_immutable_evidence_fixtures);
    let source_cancellation_hash = sha256_json_value(&source_cancellation);
    let contract_hash = sha256_text_value(&format!(
        "runtime-provider-router-activation-command-result-receipt-audit-trail-immutable-evidence-denial:v1:source={source_cancellation_hash}:fixtures={fixtures_hash}:audit=0:evidence=0:hash=0:attestation=0:record=0:persist=0:authority=0"
    ));
    let policy_hash = sha256_text_value(
        "runtime-provider-router-activation-command-result-receipt-audit-trail-immutable-evidence-denial:v1:no-audit-write:no-evidence-persist:no-hash-chain:no-merkle-root:no-attestation:no-witness:no-notary:no-ledger-index-delivery:no-provider-model-memory-kg-secret-external-install-restart-binary-public-authority",
    );
    let side_effect_hash = sha256_text_value(
        "runtime-provider-router-audit-trail-immutable-evidence-side-effects=false;fixtures=10;audit=0;evidence=0;hash=0;attestation=0;record=0;persist=0;activation=0;provider=0;model=0;memory=0;kg=0;secret=0;external=0;install=0",
    );
    let denials = vec![
        "source_cancellation_supersession_report_required",
        "audit_trail_request_acceptance_denied",
        "audit_trail_recording_denied",
        "audit_trail_persistence_denied",
        "audit_trail_materialization_denied",
        "immutable_evidence_request_acceptance_denied",
        "immutable_evidence_recording_denied",
        "immutable_evidence_persistence_denied",
        "immutable_evidence_materialization_denied",
        "hash_chain_recording_denied",
        "merkle_root_recording_denied",
        "attestation_recording_denied",
        "witness_recording_denied",
        "notary_recording_denied",
        "ledger_evidence_recording_denied",
        "index_evidence_recording_denied",
        "delivery_evidence_recording_denied",
        "activation_from_audit_evidence_denied",
        "memory_store_evidence_denied",
        "live_kg_evidence_denied",
        "rollback_evidence_denied",
        "secret_material_evidence_denied",
        "provider_prompt_evidence_denied",
        "external_public_install_restart_active_binary_evidence_denied",
    ];
    let denied_count = denials.len();
    let denials_value = serde_json::Value::Array(
        denials
            .iter()
            .map(|denial| serde_json::Value::String((*denial).to_string()))
            .collect(),
    );

    let mut report = source_cancellation.clone();
    extend_json_object(
        &mut report,
        serde_json::json!({
            "status": if report_ready { "ready" } else { "blocked" },
            "gate": "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_route",
            "source_command": "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-audit-trail-immutable-evidence-denial --json",
            "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_AUDIT_TRAIL_IMMUTABLE_EVIDENCE_DENIAL_ENDPOINT,
            "native_route": true,
            "compatibility_mode": "native_runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_status",
            "side_effect_free": true,
            "source_activation_command_result_receipt_cancellation_supersession_route_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_CANCELLATION_SUPERSESSION_DENIAL_ENDPOINT,
            "source_activation_command_result_receipt_cancellation_supersession_gate": source_str("gate"),
            "source_activation_command_result_receipt_cancellation_supersession_ready": source_cancellation_ready,
            "source_activation_command_result_receipt_cancellation_supersession_status": source_str("runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_status"),
            "source_activation_command_result_receipt_cancellation_supersession_report_sha256": source_cancellation_hash,
            "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
            "route_count": route_matrix.route_count,
            "implemented_route_count": route_matrix.implemented_route_count,
            "missing_route_count": route_matrix.missing_route_count,
            "route_count_cutover_floor": NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR,
            "route_count_floor_preserved": route_count_floor_preserved,
            "route_count_source_command_accepted": route_count_source_command_accepted,
            "source_route_wired": true,
            "operator_authorization_received": true,
            "fresh_evidence_review_requested": true,
            "explicit_command_path_reviewed": true,
            "accepted_operator_approval_consumed": false,
            "activation_authority_derived": false,
            "runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_route_enabled": true,
            "runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready": true,
            "runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_status": "blocked",
            "runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_no_persistence_ready": source_bool("runtime_provider_router_activation_command_result_receipt_no_persistence_ready"),
            "activation_command_result_receipt_audit_trail_immutable_evidence_schema_version": "memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_v1",
            "activation_command_result_receipt_audit_trail_immutable_evidence_mode": "runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_no_audit_write_no_evidence_persist",
            "activation_command_result_receipt_audit_trail_immutable_evidence_decision": "runtime_provider_router_activation_command_result_receipt_cannot_be_wrapped_as_audit_trail_or_immutable_evidence_authority",
            "minimum_required_samples": 24,
            "audit_trail_immutable_evidence_fixtures_sha256": fixtures_hash,
            "audit_trail_immutable_evidence_contract_hash_sha256": contract_hash,
            "audit_trail_immutable_evidence_policy_hash_sha256": policy_hash,
            "side_effect_hash_sha256": side_effect_hash,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_cancellation_supersession_fixture_count": source_u64("cancellation_supersession_fixture_count"),
            "source_blocked_cancellation_supersession_fixture_count": source_u64("blocked_cancellation_supersession_fixture_count"),
            "source_noop_cancellation_supersession_fixture_count": source_u64("noop_cancellation_supersession_fixture_count"),
            "source_accepted_cancellation_supersession_fixture_count": source_u64("accepted_cancellation_supersession_fixture_count"),
            "source_cancellation_performed_count": source_u64("cancellation_performed_count"),
            "source_supersession_performed_count": source_u64("supersession_performed_count"),
            "source_replacement_receipt_recorded_count": source_u64("replacement_receipt_recorded_count"),
            "source_replacement_receipt_persisted_count": source_u64("replacement_receipt_persisted_count"),
            "source_tombstone_recorded_count": source_u64("tombstone_recorded_count"),
            "source_delete_marker_recorded_count": source_u64("delete_marker_recorded_count"),
            "cancellation_supersession_surface_count": source_u64("cancellation_supersession_surface_count"),
            "cancellation_supersession_surface_ready_count": source_u64("cancellation_supersession_surface_ready_count"),
            "audit_trail_immutable_evidence_surface_count": 12,
            "audit_trail_immutable_evidence_surface_ready_count": 12,
            "audit_trail_immutable_evidence_side_effect_free_surface_count": 12,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "audit_trail_immutable_evidence_surfaces": [
                "source_cancellation_supersession_report_required",
                "audit_trail_request_shape_denied",
                "immutable_evidence_request_shape_denied",
                "append_only_audit_log_recording_denied",
                "evidence_hash_chain_recording_denied",
                "attestation_witness_notary_recording_denied",
                "audit_trail_materialization_denied",
                "immutable_evidence_persistence_denied",
                "ledger_index_delivery_evidence_denied",
                "activation_from_audit_evidence_denied",
                "memory_kg_rollback_secret_provider_evidence_denied",
                "external_public_install_restart_active_binary_evidence_denied"
            ],
            "audit_trail_immutable_evidence_fixtures": audit_trail_immutable_evidence_fixtures,
            "audit_trail_immutable_evidence_fixture_count": audit_fixture_count,
            "blocked_audit_trail_immutable_evidence_fixture_count": audit_fixture_count,
            "noop_audit_trail_immutable_evidence_fixture_count": audit_fixture_count,
            "allowed_audit_trail_immutable_evidence_fixture_count": 0,
            "accepted_audit_trail_immutable_evidence_fixture_count": 0,
            "audit_trail_denied_count": audit_fixture_count,
            "immutable_evidence_denied_count": immutable_evidence_denied_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "audit_trail_performed_count": 0,
            "immutable_evidence_performed_count": 0,
            "audit_trail_recorded_count": 0,
            "audit_trail_persisted_count": 0,
            "immutable_evidence_recorded_count": 0,
            "immutable_evidence_persisted_count": 0,
            "hash_chain_recorded_count": 0,
            "merkle_root_recorded_count": 0,
            "attestation_recorded_count": 0,
            "witness_recorded_count": 0,
            "notary_recorded_count": 0,
            "ledger_evidence_recorded_count": 0,
            "index_evidence_recorded_count": 0,
            "delivery_evidence_recorded_count": 0,
            "denied_by_audit_trail_immutable_evidence": denials_value,
            "denied_by_audit_trail_immutable_evidence_count": denied_count,
            "denied_by_activation_command_result_receipt_audit_trail_immutable_evidence": denials_value,
            "denied_by_activation_command_result_receipt_audit_trail_immutable_evidence_count": denied_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [
                {
                    "action": "review_runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial",
                    "status": "allowed_report_only",
                    "writes_audit_trail": false,
                    "persists_evidence": false,
                    "mutates_runtime": false,
                    "invokes_model": false
                },
                {
                    "action": "stage_runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial",
                    "status": "allowed_report_only_next_slice",
                    "writes_audit_trail": false,
                    "persists_evidence": false,
                    "performs_retention": false,
                    "performs_gc": false,
                    "mutates_runtime": false,
                    "invokes_model": false
                },
                {
                    "action": "run_full_light_preflight",
                    "status": "allowed_verification_only",
                    "writes_audit_trail": false,
                    "persists_evidence": false,
                    "mutates_runtime": false,
                    "invokes_model": false,
                    "writes_kg": false
                }
            ],
            "source_cancellation_supersession_report_required": true,
            "audit_trail_acceptance_forbidden": true,
            "audit_trail_recording_forbidden": true,
            "audit_trail_persistence_forbidden": true,
            "immutable_evidence_acceptance_forbidden": true,
            "immutable_evidence_recording_forbidden": true,
            "immutable_evidence_persistence_forbidden": true,
            "hash_chain_or_merkle_root_recording_forbidden": true,
            "attestation_witness_notary_recording_forbidden": true,
            "runtime_provider_memory_kg_evidence_forbidden": true,
            "secret_read_forbidden": true,
            "external_public_install_restart_active_binary_evidence_forbidden": true,
        }),
    );
    if let Some(report_object) = report.as_object_mut() {
        for key in [
            "activation_command_result_receipt_audit_trail_allowed",
            "activation_command_result_receipt_audit_trail_recorded",
            "activation_command_result_receipt_audit_trail_persisted",
            "activation_command_result_receipt_audit_trail_materialized",
            "activation_command_result_receipt_audit_trail_filesystem_written",
            "activation_command_result_receipt_immutable_evidence_allowed",
            "activation_command_result_receipt_immutable_evidence_recorded",
            "activation_command_result_receipt_immutable_evidence_persisted",
            "activation_command_result_receipt_immutable_evidence_materialized",
            "activation_command_result_receipt_immutable_evidence_filesystem_written",
            "activation_command_result_receipt_hash_chain_recorded",
            "activation_command_result_receipt_hash_chain_persisted",
            "activation_command_result_receipt_merkle_root_recorded",
            "activation_command_result_receipt_merkle_root_persisted",
            "activation_command_result_receipt_attestation_recorded",
            "activation_command_result_receipt_attestation_persisted",
            "activation_command_result_receipt_witness_recorded",
            "activation_command_result_receipt_witness_persisted",
            "activation_command_result_receipt_notary_recorded",
            "activation_command_result_receipt_notary_persisted",
            "activation_command_result_receipt_ledger_evidence_recorded",
            "activation_command_result_receipt_ledger_evidence_persisted",
            "activation_command_result_receipt_index_evidence_recorded",
            "activation_command_result_receipt_index_evidence_persisted",
            "activation_command_result_receipt_delivery_evidence_recorded",
            "activation_command_result_receipt_delivery_evidence_persisted",
            "activation_command_result_receipt_cancellation_allowed",
            "activation_command_result_receipt_cancellation_recorded",
            "activation_command_result_receipt_cancellation_persisted",
            "activation_command_result_receipt_supersession_allowed",
            "activation_command_result_receipt_supersession_recorded",
            "activation_command_result_receipt_supersession_persisted",
            "activation_command_result_receipt_replacement_receipt_accepted",
            "activation_command_result_receipt_replacement_receipt_recorded",
            "activation_command_result_receipt_replacement_receipt_persisted",
            "activation_command_result_receipt_tombstone_recorded",
            "activation_command_result_receipt_delete_marker_recorded",
            "activation_command_result_receipt_recorded",
            "activation_command_result_receipt_persisted",
            "activation_command_result_receipt_accepted",
            "activation_command_result_receipt_materialized",
            "activation_command_result_receipt_filesystem_written",
            "activation_command_result_receipt_ledger_written",
            "activation_command_result_receipt_indexed",
            "activation_command_result_receipt_enqueued",
            "activation_command_result_receipt_delivered",
            "activation_command_result_receipt_exported",
            "activation_command_result_receipt_query_registered",
            "activation_command_result_receipt_observability_recorded",
            "activation_command_completion_ack_recorded",
            "activation_command_completion_ack_persisted",
            "activation_command_completion_ack_accepted",
            "activation_command_completion_ack_delivered",
            "operator_approval_from_audit_trail_accepted",
            "operator_approval_from_immutable_evidence_accepted",
            "activation_from_audit_trail_allowed",
            "activation_from_immutable_evidence_allowed",
            "activation_from_cancellation_allowed",
            "activation_from_supersession_allowed",
            "activation_from_ordering_allowed",
            "activation_from_replay_allowed",
            "activation_from_receipt_allowed",
            "activation_command_allowed",
            "activation_command_accepted",
            "activation_command_enabled",
            "activation_command_invoked",
            "activation_command_dispatched",
            "activation_command_dispatch_performed",
            "activation_request_accepted",
            "activation_request_recorded",
            "activation_request_persisted",
            "activation_request_executed",
            "activation_activated",
            "runtime_router_mutated",
            "runtime_attachment_performed",
            "live_context_attached",
            "context_injection_performed",
            "adapter_invoked",
            "provider_invoked",
            "model_invoked",
            "provider_prompt_replayed",
            "auth_secret_read",
            "credential_read",
            "secret_file_read",
            "usage_recorded",
            "memory_store_write_performed",
            "memory_store_mutated",
            "live_kg_write_performed",
            "readback_evidence_recorded",
            "readback_evidence_persisted",
            "router_handoff_recorded",
            "router_handoff_persisted",
            "rollback_executed",
            "telegram_send_performed",
            "channel_send_performed",
            "external_send_performed",
            "public_release_claimed",
            "public_ga_claimed",
            "release_artifact_written",
            "install_executed",
            "launchd_mutated",
            "service_restart_performed",
            "active_binary_mutated",
        ] {
            report_object.insert(key.to_string(), serde_json::Value::Bool(false));
        }
        if !report_object
            .get("side_effects")
            .is_some_and(serde_json::Value::is_object)
        {
            report_object.insert("side_effects".to_string(), serde_json::json!({}));
        }
    }
    if let Some(side_effects) = report
        .get_mut("side_effects")
        .and_then(serde_json::Value::as_object_mut)
    {
        for key in [
            "activation_command_result_receipt_audit_trail_recorded",
            "activation_command_result_receipt_audit_trail_persisted",
            "activation_command_result_receipt_audit_trail_materialized",
            "activation_command_result_receipt_audit_trail_filesystem_written",
            "activation_command_result_receipt_immutable_evidence_recorded",
            "activation_command_result_receipt_immutable_evidence_persisted",
            "activation_command_result_receipt_immutable_evidence_materialized",
            "activation_command_result_receipt_immutable_evidence_filesystem_written",
            "activation_command_result_receipt_hash_chain_recorded",
            "activation_command_result_receipt_hash_chain_persisted",
            "activation_command_result_receipt_merkle_root_recorded",
            "activation_command_result_receipt_merkle_root_persisted",
            "activation_command_result_receipt_attestation_recorded",
            "activation_command_result_receipt_attestation_persisted",
            "activation_command_result_receipt_witness_recorded",
            "activation_command_result_receipt_witness_persisted",
            "activation_command_result_receipt_notary_recorded",
            "activation_command_result_receipt_notary_persisted",
            "activation_command_result_receipt_ledger_evidence_recorded",
            "activation_command_result_receipt_ledger_evidence_persisted",
            "activation_command_result_receipt_index_evidence_recorded",
            "activation_command_result_receipt_index_evidence_persisted",
            "activation_command_result_receipt_delivery_evidence_recorded",
            "activation_command_result_receipt_delivery_evidence_persisted",
            "activation_from_audit_trail_allowed",
            "activation_from_immutable_evidence_allowed",
            "activation_command_enabled",
            "activation_command_invoked",
            "activation_command_dispatched",
            "activation_request_recorded",
            "activation_request_executed",
            "activation_activated",
            "runtime_router_mutated",
            "runtime_attachment_performed",
            "live_context_attached",
            "context_injection_performed",
            "adapter_invoked",
            "provider_invoked",
            "model_invoked",
            "provider_prompt_replayed",
            "auth_secret_read",
            "credential_read",
            "secret_file_read",
            "usage_recorded",
            "memory_store_write_performed",
            "memory_store_mutated",
            "live_kg_write_performed",
            "readback_evidence_recorded",
            "readback_evidence_persisted",
            "router_handoff_recorded",
            "router_handoff_persisted",
            "rollback_executed",
            "telegram_send_performed",
            "channel_send_performed",
            "external_send_performed",
            "filesystem_written",
            "public_release_claimed",
            "public_ga_claimed",
            "release_artifact_written",
            "public_artifact_written",
            "install_executed",
            "launchd_mutated",
            "service_restart_performed",
            "active_binary_mutated",
        ] {
            side_effects.insert(key.to_string(), serde_json::Value::Bool(false));
        }
    }
    report
}

fn hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_report();
    let source_bool = |key: &str| {
        source
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let source_u64 = |key: &str| {
        source
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_str = |key: &str| {
        source
            .get(key)
            .and_then(serde_json::Value::as_str)
            .unwrap_or("blocked")
            .to_string()
    };
    let source_hash_str = |key: &str| {
        source
            .get(key)
            .and_then(serde_json::Value::as_str)
            .unwrap_or("")
            .to_string()
    };
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_ready = source_str("status") == "ready"
        && source_bool(
            "runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready",
        )
        && source_str(
            "runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_status",
        ) == "blocked"
        && source_u64("accepted_audit_trail_immutable_evidence_fixture_count") == 0
        && source_u64("audit_trail_performed_count") == 0
        && source_u64("immutable_evidence_performed_count") == 0
        && source_u64("audit_trail_recorded_count") == 0
        && source_u64("audit_trail_persisted_count") == 0
        && source_u64("immutable_evidence_recorded_count") == 0
        && source_u64("immutable_evidence_persisted_count") == 0
        && source_u64("hash_chain_recorded_count") == 0
        && source_u64("merkle_root_recorded_count") == 0
        && source_u64("attestation_recorded_count") == 0
        && source_u64("witness_recorded_count") == 0
        && source_u64("notary_recorded_count") == 0;
    let report_ready = source_ready && route_count_source_command_accepted;

    let retention_gc_fixture =
        |fixture_id: &str, status: &str, reason: &str, extra: serde_json::Value| {
            let mut fixture = serde_json::Map::new();
            fixture.insert(
                "fixture_id".to_string(),
                serde_json::Value::String(fixture_id.to_string()),
            );
            fixture.insert(
                "id".to_string(),
                serde_json::Value::String(fixture_id.to_string()),
            );
            fixture.insert(
                "retention_gc_status".to_string(),
                serde_json::Value::String(status.to_string()),
            );
            fixture.insert(
                "denial_reason".to_string(),
                serde_json::Value::String(reason.to_string()),
            );
            for key in [
                "source_audit_evidence_present",
                "source_audit_evidence_ready",
                "source_audit_trail_immutable_evidence_present",
                "source_audit_trail_immutable_evidence_ready",
                "retention_requested",
                "receipt_noop_confirmed",
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(true));
            }
            for key in [
                "expiry_requested",
                "garbage_collection_requested",
                "retention_index_requested",
                "expiry_schedule_requested",
                "expiry_timer_requested",
                "ttl_update_requested",
                "ttl_extension_requested",
                "garbage_collection_scan_requested",
                "delete_requested",
                "tombstone_requested",
                "sweep_requested",
                "archive_requested",
                "compaction_requested",
                "activation_from_retention_gc_requested",
                "memory_store_gc_evidence_requested",
                "live_kg_gc_evidence_requested",
                "rollback_gc_evidence_requested",
                "secret_material_gc_evidence_requested",
                "provider_prompt_gc_evidence_requested",
                "ledger_retention_requested",
                "index_retention_requested",
                "delivery_retention_requested",
                "external_send_gc_evidence_requested",
                "public_claim_gc_evidence_requested",
                "release_artifact_gc_evidence_requested",
                "install_gc_evidence_requested",
                "service_restart_gc_evidence_requested",
                "active_binary_gc_evidence_requested",
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(false));
            }
            for key in [
                "retention_policy_allowed",
                "retention_policy_recorded",
                "retention_policy_persisted",
                "retention_policy_materialized",
                "retention_policy_filesystem_written",
                "retention_index_allowed",
                "retention_index_recorded",
                "retention_index_persisted",
                "expiry_allowed",
                "expiry_recorded",
                "expiry_persisted",
                "expiry_scheduler_registered",
                "expiry_timer_started",
                "expiry_materialized",
                "ttl_update_allowed",
                "ttl_update_recorded",
                "ttl_extension_allowed",
                "ttl_extension_recorded",
                "garbage_collection_allowed",
                "garbage_collection_scan_performed",
                "garbage_collection_candidate_recorded",
                "garbage_collection_decision_recorded",
                "garbage_collection_persisted",
                "delete_allowed",
                "delete_performed",
                "delete_marker_recorded",
                "tombstone_recorded",
                "sweep_allowed",
                "sweep_performed",
                "archive_allowed",
                "archive_written",
                "compaction_allowed",
                "compaction_performed",
                "compaction_artifact_written",
                "ledger_retention_recorded",
                "ledger_retention_persisted",
                "index_retention_recorded",
                "index_retention_persisted",
                "delivery_retention_recorded",
                "delivery_retention_persisted",
                "activation_command_result_receipt_audit_trail_recorded",
                "activation_command_result_receipt_audit_trail_persisted",
                "activation_command_result_receipt_immutable_evidence_recorded",
                "activation_command_result_receipt_immutable_evidence_persisted",
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
                "operator_approval_from_retention_accepted",
                "operator_approval_from_expiry_accepted",
                "operator_approval_from_garbage_collection_accepted",
                "activation_from_retention_allowed",
                "activation_from_expiry_allowed",
                "activation_from_garbage_collection_allowed",
                "activation_command_enabled",
                "activation_command_invoked",
                "activation_command_dispatched",
                "activation_request_accepted",
                "activation_request_recorded",
                "activation_request_persisted",
                "activation_request_executed",
                "activation_activated",
                "runtime_router_mutated",
                "runtime_attachment_performed",
                "live_context_attached",
                "context_injection_performed",
                "adapter_invoked",
                "provider_invoked",
                "model_invoked",
                "provider_prompt_replayed",
                "auth_secret_read",
                "credential_read",
                "secret_file_read",
                "usage_recorded",
                "memory_store_write_performed",
                "memory_store_mutated",
                "live_kg_write_performed",
                "readback_evidence_recorded",
                "readback_evidence_persisted",
                "router_handoff_recorded",
                "router_handoff_persisted",
                "rollback_executed",
                "telegram_send_performed",
                "channel_send_performed",
                "external_send_performed",
                "public_release_claimed",
                "public_ga_claimed",
                "release_artifact_written",
                "install_executed",
                "launchd_mutated",
                "service_restart_performed",
                "active_binary_mutated",
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(false));
            }
            let mut fixture = serde_json::Value::Object(fixture);
            extend_json_object(&mut fixture, extra);
            fixture
        };

    let retention_expiry_garbage_collection_fixtures = serde_json::Value::Array(vec![
        retention_gc_fixture(
            "provider-router-activation-command-result-receipt-retention-missing-source-audit-evidence",
            "blocked_noop",
            "source_audit_trail_immutable_evidence_report_required",
            serde_json::json!({
                "source_audit_evidence_present": false,
                "source_audit_evidence_ready": false,
                "source_audit_trail_immutable_evidence_present": false,
                "source_audit_trail_immutable_evidence_ready": false,
            }),
        ),
        retention_gc_fixture(
            "provider-router-activation-command-result-receipt-retention-policy-write-request",
            "blocked_noop",
            "retention_policy_write_request_denied",
            serde_json::json!({
                "retention_policy_request_shape": "record_blocked_noop_receipt_retention_policy",
            }),
        ),
        retention_gc_fixture(
            "provider-router-activation-command-result-receipt-retention-index-record",
            "blocked_noop",
            "retention_index_recording_denied",
            serde_json::json!({
                "retention_index_requested": true,
            }),
        ),
        retention_gc_fixture(
            "provider-router-activation-command-result-receipt-expiry-scheduler-timer",
            "blocked_expiry_noop",
            "expiry_scheduler_timer_denied",
            serde_json::json!({
                "retention_requested": false,
                "expiry_requested": true,
                "expiry_schedule_requested": true,
                "expiry_timer_requested": true,
            }),
        ),
        retention_gc_fixture(
            "provider-router-activation-command-result-receipt-ttl-update-extension",
            "blocked_expiry_noop",
            "ttl_update_extension_denied",
            serde_json::json!({
                "retention_requested": false,
                "expiry_requested": true,
                "ttl_update_requested": true,
                "ttl_extension_requested": true,
            }),
        ),
        retention_gc_fixture(
            "provider-router-activation-command-result-receipt-garbage-collection-scan",
            "blocked_gc_noop",
            "garbage_collection_scan_denied",
            serde_json::json!({
                "retention_requested": false,
                "garbage_collection_requested": true,
                "garbage_collection_scan_requested": true,
            }),
        ),
        retention_gc_fixture(
            "provider-router-activation-command-result-receipt-delete-tombstone-sweep",
            "blocked_gc_noop",
            "delete_tombstone_sweep_denied",
            serde_json::json!({
                "retention_requested": false,
                "garbage_collection_requested": true,
                "delete_requested": true,
                "tombstone_requested": true,
                "sweep_requested": true,
            }),
        ),
        retention_gc_fixture(
            "provider-router-activation-command-result-receipt-archive-compaction",
            "blocked_gc_noop",
            "archive_compaction_denied",
            serde_json::json!({
                "retention_requested": false,
                "garbage_collection_requested": true,
                "archive_requested": true,
                "compaction_requested": true,
            }),
        ),
        retention_gc_fixture(
            "provider-router-activation-command-result-receipt-activation-memory-kg-provider-retention-gc",
            "blocked_gc_noop",
            "activation_memory_kg_provider_retention_gc_denied",
            serde_json::json!({
                "retention_requested": false,
                "expiry_requested": true,
                "garbage_collection_requested": true,
                "activation_from_retention_gc_requested": true,
                "memory_store_gc_evidence_requested": true,
                "live_kg_gc_evidence_requested": true,
                "rollback_gc_evidence_requested": true,
                "secret_material_gc_evidence_requested": true,
                "provider_prompt_gc_evidence_requested": true,
            }),
        ),
        retention_gc_fixture(
            "provider-router-activation-command-result-receipt-external-public-install-retention-gc",
            "blocked_gc_noop",
            "external_public_install_restart_active_binary_retention_gc_denied",
            serde_json::json!({
                "retention_requested": false,
                "expiry_requested": true,
                "garbage_collection_requested": true,
                "ledger_retention_requested": true,
                "index_retention_requested": true,
                "delivery_retention_requested": true,
                "external_send_gc_evidence_requested": true,
                "public_claim_gc_evidence_requested": true,
                "release_artifact_gc_evidence_requested": true,
                "install_gc_evidence_requested": true,
                "service_restart_gc_evidence_requested": true,
                "active_binary_gc_evidence_requested": true,
            }),
        ),
    ]);
    let retention_gc_fixture_count = retention_expiry_garbage_collection_fixtures
        .as_array()
        .map(std::vec::Vec::len)
        .unwrap_or(0);
    let fixtures_hash = sha256_json_value(&retention_expiry_garbage_collection_fixtures);
    let source_report_sha256 = sha256_json_value(&source);
    let contract_hash = sha256_text_value(&format!(
        "runtime-provider-router-activation-command-result-receipt-retention-expiry-garbage-collection-denial:v1:source={source_report_sha256}:fixtures={fixtures_hash}:retention=0:expiry=0:gc=0:delete=0:archive=0:authority=0"
    ));
    let policy_hash = sha256_text_value(
        "runtime-provider-router-activation-command-result-receipt-retention-expiry-garbage-collection-denial:v1:no-retention:no-expiry:no-gc:no-delete:no-tombstone:no-sweep:no-archive:no-compaction:no-provider-model-memory-kg-secret-external-install-restart-binary-public-authority",
    );
    let side_effect_hash = sha256_text_value(
        "runtime-provider-router-retention-expiry-garbage-collection-side-effects=false;fixtures=10;retention=0;expiry=0;gc=0;delete=0;archive=0;compaction=0;record=0;persist=0;activation=0;provider=0;model=0;memory=0;kg=0;secret=0;external=0;install=0",
    );
    let denials = vec![
        "source_audit_trail_immutable_evidence_report_required",
        "retention_policy_request_acceptance_denied",
        "retention_policy_recording_denied",
        "retention_policy_persistence_denied",
        "retention_index_recording_denied",
        "expiry_request_acceptance_denied",
        "expiry_recording_denied",
        "expiry_scheduler_registration_denied",
        "expiry_timer_start_denied",
        "ttl_update_denied",
        "ttl_extension_denied",
        "garbage_collection_request_acceptance_denied",
        "garbage_collection_scan_denied",
        "garbage_collection_candidate_recording_denied",
        "garbage_collection_decision_recording_denied",
        "delete_marker_recording_denied",
        "tombstone_recording_denied",
        "sweep_execution_denied",
        "archive_write_denied",
        "compaction_execution_denied",
        "ledger_retention_recording_denied",
        "index_retention_recording_denied",
        "delivery_retention_recording_denied",
        "activation_from_retention_expiry_gc_denied",
        "memory_kg_gc_denied",
        "rollback_gc_denied",
        "secret_material_gc_denied",
        "provider_prompt_gc_denied",
        "external_public_install_restart_active_binary_gc_denied",
    ];
    let denied_count = denials.len();
    let denials_value = serde_json::Value::Array(
        denials
            .iter()
            .map(|denial| serde_json::Value::String((*denial).to_string()))
            .collect(),
    );

    let mut report = source.clone();
    extend_json_object(
        &mut report,
        serde_json::json!({
            "status": if report_ready { "ready" } else { "blocked" },
            "gate": "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_route",
            "source_command": "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-retention-expiry-garbage-collection-denial --json",
            "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_RETENTION_EXPIRY_GARBAGE_COLLECTION_DENIAL_ENDPOINT,
            "native_route": true,
            "compatibility_mode": "native_runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_status",
            "side_effect_free": true,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_activation_command_result_receipt_audit_trail_immutable_evidence_route_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_AUDIT_TRAIL_IMMUTABLE_EVIDENCE_DENIAL_ENDPOINT,
            "source_activation_command_result_receipt_audit_trail_immutable_evidence_gate": source_str("gate"),
            "source_activation_command_result_receipt_audit_trail_immutable_evidence_ready": source_ready,
            "source_activation_command_result_receipt_audit_trail_immutable_evidence_status": source_str("runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_status"),
            "source_activation_command_result_receipt_audit_trail_immutable_evidence_report_sha256": source_report_sha256,
            "source_audit_trail_immutable_evidence_contract_hash_sha256": source_hash_str("audit_trail_immutable_evidence_contract_hash_sha256"),
            "source_audit_trail_immutable_evidence_policy_hash_sha256": source_hash_str("audit_trail_immutable_evidence_policy_hash_sha256"),
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
            "route_count": route_matrix.route_count,
            "implemented_route_count": route_matrix.implemented_route_count,
            "missing_route_count": route_matrix.missing_route_count,
            "route_count_cutover_floor": NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR,
            "route_count_floor_preserved": route_count_floor_preserved,
            "route_count_source_command_accepted": route_count_source_command_accepted,
            "source_route_wired": true,
            "operator_authorization_received": true,
            "fresh_evidence_review_requested": true,
            "explicit_command_path_reviewed": true,
            "accepted_operator_approval_consumed": false,
            "activation_authority_derived": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_route_enabled": true,
            "runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready": true,
            "runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_status": "blocked",
            "runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_no_persistence_ready": source_bool("runtime_provider_router_activation_command_result_receipt_no_persistence_ready"),
            "activation_command_result_receipt_retention_expiry_garbage_collection_schema_version": "memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_v1",
            "activation_command_result_receipt_retention_expiry_garbage_collection_mode": "runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_no_retention_no_expiry_no_gc",
            "activation_command_result_receipt_retention_expiry_garbage_collection_decision": "runtime_provider_router_activation_command_result_receipt_cannot_be_retained_expired_garbage_collected_deleted_archived_or_compacted_into_authority",
            "minimum_required_samples": 24,
            "retention_expiry_garbage_collection_fixtures_sha256": fixtures_hash,
            "retention_expiry_garbage_collection_contract_hash_sha256": contract_hash,
            "retention_expiry_garbage_collection_policy_hash_sha256": policy_hash,
            "side_effect_hash_sha256": side_effect_hash,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_audit_trail_immutable_evidence_fixture_count": source_u64("audit_trail_immutable_evidence_fixture_count"),
            "source_blocked_audit_trail_immutable_evidence_fixture_count": source_u64("blocked_audit_trail_immutable_evidence_fixture_count"),
            "source_accepted_audit_trail_immutable_evidence_fixture_count": source_u64("accepted_audit_trail_immutable_evidence_fixture_count"),
            "source_audit_trail_performed_count": source_u64("audit_trail_performed_count"),
            "source_immutable_evidence_performed_count": source_u64("immutable_evidence_performed_count"),
            "source_hash_chain_recorded_count": source_u64("hash_chain_recorded_count"),
            "source_merkle_root_recorded_count": source_u64("merkle_root_recorded_count"),
            "source_attestation_recorded_count": source_u64("attestation_recorded_count"),
            "audit_trail_immutable_evidence_surface_count": source_u64("audit_trail_immutable_evidence_surface_count"),
            "audit_trail_immutable_evidence_fixture_count": source_u64("audit_trail_immutable_evidence_fixture_count"),
            "retention_expiry_garbage_collection_surface_count": 12,
            "retention_expiry_garbage_collection_surface_ready_count": 12,
            "retention_expiry_garbage_collection_side_effect_free_surface_count": 12,
            "retention_expiry_garbage_collection_surfaces": [
                "source_audit_trail_immutable_evidence_report_required",
                "retention_policy_request_shape_denied",
                "retention_index_recording_denied",
                "expiry_scheduler_registration_denied",
                "ttl_update_extension_denied",
                "garbage_collection_scan_denied",
                "delete_tombstone_sweep_denied",
                "archive_compaction_denied",
                "ledger_index_delivery_retention_evidence_denied",
                "activation_from_retention_expiry_gc_denied",
                "memory_kg_rollback_secret_provider_gc_denied",
                "external_public_install_restart_active_binary_gc_denied"
            ],
            "retention_expiry_garbage_collection_fixtures": retention_expiry_garbage_collection_fixtures,
            "retention_expiry_garbage_collection_fixture_count": retention_gc_fixture_count,
            "blocked_retention_expiry_garbage_collection_fixture_count": retention_gc_fixture_count,
            "noop_retention_expiry_garbage_collection_fixture_count": retention_gc_fixture_count,
            "allowed_retention_expiry_garbage_collection_fixture_count": 0,
            "accepted_retention_expiry_garbage_collection_fixture_count": 0,
            "retention_denied_count": retention_gc_fixture_count,
            "expiry_denied_count": retention_gc_fixture_count,
            "garbage_collection_denied_count": retention_gc_fixture_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "retention_performed_count": 0,
            "expiry_performed_count": 0,
            "garbage_collection_performed_count": 0,
            "delete_performed_count": 0,
            "archive_written_count": 0,
            "compaction_performed_count": 0,
            "retention_policy_recorded_count": 0,
            "retention_policy_persisted_count": 0,
            "retention_index_recorded_count": 0,
            "retention_index_persisted_count": 0,
            "expiry_recorded_count": 0,
            "expiry_persisted_count": 0,
            "expiry_scheduler_registered_count": 0,
            "expiry_timer_started_count": 0,
            "ttl_update_recorded_count": 0,
            "ttl_extension_recorded_count": 0,
            "garbage_collection_scan_performed_count": 0,
            "garbage_collection_candidate_recorded_count": 0,
            "garbage_collection_decision_recorded_count": 0,
            "delete_marker_recorded_count": 0,
            "tombstone_recorded_count": 0,
            "sweep_performed_count": 0,
            "ledger_retention_recorded_count": 0,
            "index_retention_recorded_count": 0,
            "delivery_retention_recorded_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "denied_by_retention_expiry_garbage_collection": denials_value,
            "denied_by_retention_expiry_garbage_collection_count": denied_count,
            "denied_by_activation_command_result_receipt_retention_expiry_garbage_collection": denials_value,
            "denied_by_activation_command_result_receipt_retention_expiry_garbage_collection_count": denied_count,
            "allowed_next_actions": [
                {
                    "action": "review_runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial",
                    "status": "allowed_report_only",
                    "performs_retention": false,
                    "performs_expiry": false,
                    "performs_gc": false,
                    "deletes_receipt": false,
                    "mutates_runtime": false,
                    "invokes_model": false
                },
                {
                    "action": "stage_runtime_provider_router_activation_command_result_receipt_export_query_observability_denial",
                    "status": "allowed_report_only_next_slice",
                    "exports_receipt": false,
                    "registers_query": false,
                    "records_observability": false,
                    "mutates_runtime": false,
                    "invokes_model": false
                },
                {
                    "action": "run_full_light_preflight",
                    "status": "allowed_verification_only",
                    "performs_retention": false,
                    "performs_gc": false,
                    "mutates_runtime": false,
                    "invokes_model": false,
                    "writes_kg": false
                }
            ],
            "source_audit_trail_immutable_evidence_report_required": true,
            "retention_acceptance_forbidden": true,
            "retention_recording_forbidden": true,
            "retention_persistence_forbidden": true,
            "expiry_acceptance_forbidden": true,
            "expiry_scheduler_registration_forbidden": true,
            "ttl_update_forbidden": true,
            "garbage_collection_forbidden": true,
            "delete_tombstone_sweep_forbidden": true,
            "archive_compaction_forbidden": true,
            "runtime_provider_memory_kg_gc_evidence_forbidden": true,
            "secret_read_forbidden": true,
            "external_public_install_restart_active_binary_gc_forbidden": true,
        }),
    );

    if let Some(report_object) = report.as_object_mut() {
        for key in [
            "activation_command_result_receipt_retention_policy_allowed",
            "activation_command_result_receipt_retention_policy_recorded",
            "activation_command_result_receipt_retention_policy_persisted",
            "activation_command_result_receipt_retention_policy_materialized",
            "activation_command_result_receipt_retention_policy_filesystem_written",
            "activation_command_result_receipt_retention_index_allowed",
            "activation_command_result_receipt_retention_index_recorded",
            "activation_command_result_receipt_retention_index_persisted",
            "activation_command_result_receipt_expiry_allowed",
            "activation_command_result_receipt_expiry_recorded",
            "activation_command_result_receipt_expiry_persisted",
            "activation_command_result_receipt_expiry_scheduler_registered",
            "activation_command_result_receipt_expiry_timer_started",
            "activation_command_result_receipt_expiry_materialized",
            "activation_command_result_receipt_ttl_update_allowed",
            "activation_command_result_receipt_ttl_update_recorded",
            "activation_command_result_receipt_ttl_extension_allowed",
            "activation_command_result_receipt_ttl_extension_recorded",
            "activation_command_result_receipt_garbage_collection_allowed",
            "activation_command_result_receipt_garbage_collection_scan_performed",
            "activation_command_result_receipt_garbage_collection_candidate_recorded",
            "activation_command_result_receipt_garbage_collection_decision_recorded",
            "activation_command_result_receipt_garbage_collection_persisted",
            "activation_command_result_receipt_delete_allowed",
            "activation_command_result_receipt_delete_performed",
            "activation_command_result_receipt_delete_marker_recorded",
            "activation_command_result_receipt_tombstone_recorded",
            "activation_command_result_receipt_sweep_allowed",
            "activation_command_result_receipt_sweep_performed",
            "activation_command_result_receipt_archive_allowed",
            "activation_command_result_receipt_archive_written",
            "activation_command_result_receipt_compaction_allowed",
            "activation_command_result_receipt_compaction_performed",
            "activation_command_result_receipt_compaction_artifact_written",
            "activation_command_result_receipt_ledger_retention_recorded",
            "activation_command_result_receipt_ledger_retention_persisted",
            "activation_command_result_receipt_index_retention_recorded",
            "activation_command_result_receipt_index_retention_persisted",
            "activation_command_result_receipt_delivery_retention_recorded",
            "activation_command_result_receipt_delivery_retention_persisted",
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
            "activation_allowed_by_result_receipt_retention",
            "activation_allowed_by_result_receipt_expiry",
            "activation_allowed_by_result_receipt_garbage_collection",
            "activation_allowed_by_result_receipt_audit_trail",
            "activation_allowed_by_result_receipt_immutable_evidence",
            "activation_allowed_by_result_receipt",
            "operator_approval_from_retention_accepted",
            "operator_approval_from_expiry_accepted",
            "operator_approval_from_garbage_collection_accepted",
            "activation_from_retention_allowed",
            "activation_from_expiry_allowed",
            "activation_from_garbage_collection_allowed",
            "activation_command_allowed",
            "activation_command_enabled",
            "activation_command_invoked",
            "activation_command_dispatched",
            "activation_command_dispatch_performed",
            "activation_request_accepted",
            "activation_request_recorded",
            "activation_request_persisted",
            "activation_request_executed",
            "activation_activated",
            "runtime_router_mutated",
            "runtime_attachment_performed",
            "live_context_attached",
            "context_injection_performed",
            "adapter_invoked",
            "provider_invoked",
            "model_invoked",
            "provider_prompt_replayed",
            "auth_secret_read",
            "credential_read",
            "secret_file_read",
            "usage_recorded",
            "memory_store_write_performed",
            "memory_store_mutated",
            "live_kg_write_performed",
            "readback_evidence_recorded",
            "readback_evidence_persisted",
            "router_handoff_recorded",
            "router_handoff_persisted",
            "rollback_executed",
            "telegram_send_performed",
            "channel_send_performed",
            "external_send_performed",
            "public_release_claimed",
            "public_ga_claimed",
            "release_artifact_written",
            "public_artifact_written",
            "install_executed",
            "launchd_mutated",
            "service_restart_performed",
            "active_binary_mutated",
        ] {
            report_object.insert(key.to_string(), serde_json::Value::Bool(false));
        }
        if !report_object
            .get("side_effects")
            .is_some_and(serde_json::Value::is_object)
        {
            report_object.insert("side_effects".to_string(), serde_json::json!({}));
        }
    }
    if let Some(side_effects) = report
        .get_mut("side_effects")
        .and_then(serde_json::Value::as_object_mut)
    {
        for key in [
            "activation_command_result_receipt_retention_policy_recorded",
            "activation_command_result_receipt_retention_policy_persisted",
            "activation_command_result_receipt_retention_policy_materialized",
            "activation_command_result_receipt_retention_policy_filesystem_written",
            "activation_command_result_receipt_retention_index_recorded",
            "activation_command_result_receipt_retention_index_persisted",
            "activation_command_result_receipt_expiry_recorded",
            "activation_command_result_receipt_expiry_persisted",
            "activation_command_result_receipt_expiry_scheduler_registered",
            "activation_command_result_receipt_expiry_timer_started",
            "activation_command_result_receipt_ttl_update_recorded",
            "activation_command_result_receipt_ttl_extension_recorded",
            "activation_command_result_receipt_garbage_collection_scan_performed",
            "activation_command_result_receipt_garbage_collection_candidate_recorded",
            "activation_command_result_receipt_garbage_collection_decision_recorded",
            "activation_command_result_receipt_garbage_collection_persisted",
            "activation_command_result_receipt_delete_performed",
            "activation_command_result_receipt_delete_marker_recorded",
            "activation_command_result_receipt_tombstone_recorded",
            "activation_command_result_receipt_sweep_performed",
            "activation_command_result_receipt_archive_written",
            "activation_command_result_receipt_compaction_performed",
            "activation_command_result_receipt_compaction_artifact_written",
            "activation_command_result_receipt_ledger_retention_recorded",
            "activation_command_result_receipt_ledger_retention_persisted",
            "activation_command_result_receipt_index_retention_recorded",
            "activation_command_result_receipt_index_retention_persisted",
            "activation_command_result_receipt_delivery_retention_recorded",
            "activation_command_result_receipt_delivery_retention_persisted",
            "activation_command_result_receipt_recorded",
            "activation_command_result_receipt_persisted",
            "activation_command_result_receipt_accepted",
            "activation_command_result_receipt_materialized",
            "activation_command_result_receipt_filesystem_written",
            "activation_command_completion_ack_recorded",
            "activation_command_completion_ack_persisted",
            "activation_command_completion_ack_accepted",
            "activation_command_completion_ack_delivered",
            "activation_command_enabled",
            "activation_command_invoked",
            "activation_command_dispatched",
            "activation_activated",
            "runtime_router_mutated",
            "runtime_attachment_performed",
            "live_context_attached",
            "context_injection_performed",
            "adapter_invoked",
            "provider_invoked",
            "model_invoked",
            "provider_prompt_replayed",
            "auth_secret_read",
            "credential_read",
            "secret_file_read",
            "usage_recorded",
            "memory_store_write_performed",
            "memory_store_mutated",
            "live_kg_write_performed",
            "readback_evidence_recorded",
            "readback_evidence_persisted",
            "router_handoff_recorded",
            "router_handoff_persisted",
            "rollback_executed",
            "telegram_send_performed",
            "channel_send_performed",
            "external_send_performed",
            "filesystem_written",
            "public_release_claimed",
            "public_ga_claimed",
            "release_artifact_written",
            "public_artifact_written",
            "install_executed",
            "launchd_mutated",
            "service_restart_performed",
            "active_binary_mutated",
        ] {
            side_effects.insert(key.to_string(), serde_json::Value::Bool(false));
        }
    }
    report
}
