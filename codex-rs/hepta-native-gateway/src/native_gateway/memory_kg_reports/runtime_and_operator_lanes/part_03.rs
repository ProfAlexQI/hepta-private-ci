fn hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_report();
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
            "runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready",
        )
        && source_str(
            "runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_status",
        ) == "blocked"
        && source_bool(
            "runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready",
        )
        && source_bool(
            "runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready",
        )
        && source_bool(
            "runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready",
        )
        && source_bool(
            "runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready",
        )
        && source_bool(
            "runtime_provider_router_activation_command_result_receipt_no_persistence_ready",
        )
        && source_u64("retention_expiry_garbage_collection_fixture_count") == 10
        && source_u64("accepted_retention_expiry_garbage_collection_fixture_count") == 0
        && source_u64("retention_performed_count") == 0
        && source_u64("expiry_performed_count") == 0
        && source_u64("garbage_collection_performed_count") == 0
        && !source_bool("activation_command_result_receipt_retention_policy_recorded")
        && !source_bool("activation_command_result_receipt_expiry_recorded")
        && !source_bool("activation_command_result_receipt_garbage_collection_scan_performed")
        && !source_bool("activation_command_result_receipt_accepted")
        && !source_bool("activation_allowed_by_result_receipt")
        && !source_bool("activation_activated")
        && !source_bool("runtime_router_mutated")
        && !source_bool("provider_invoked")
        && !source_bool("model_invoked")
        && !source_bool("credential_read")
        && !source_bool("secret_file_read")
        && !source_bool("memory_store_write_performed")
        && !source_bool("live_kg_write_performed")
        && !source_bool("external_send_performed")
        && !source_bool("install_executed")
        && !source_bool("service_restart_performed")
        && !source_bool("active_binary_mutated");
    let report_ready = source_ready && route_count_source_command_accepted;

    let export_query_observability_fixture =
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
                "export_query_observability_status".to_string(),
                serde_json::Value::String(status.to_string()),
            );
            fixture.insert(
                "reason".to_string(),
                serde_json::Value::String(reason.to_string()),
            );
            fixture.insert(
                "denial_reason".to_string(),
                serde_json::Value::String(reason.to_string()),
            );
            for key in [
                "source_retention_expiry_gc_present",
                "source_retention_expiry_gc_ready",
                "receipt_noop_confirmed",
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(true));
            }
            for key in [
                "export_requested",
                "query_requested",
                "observability_requested",
                "export_file_requested",
                "export_stream_requested",
                "query_endpoint_requested",
                "query_index_requested",
                "query_cache_requested",
                "metric_requested",
                "trace_requested",
                "span_requested",
                "log_requested",
                "event_requested",
                "dashboard_requested",
                "alert_requested",
                "slo_requested",
                "activation_from_observability_requested",
                "memory_store_observability_requested",
                "live_kg_observability_requested",
                "rollback_observability_requested",
                "secret_material_observability_requested",
                "provider_prompt_observability_requested",
                "ledger_observability_requested",
                "index_observability_requested",
                "delivery_observability_requested",
                "external_send_observability_requested",
                "public_claim_observability_requested",
                "release_artifact_observability_requested",
                "install_observability_requested",
                "service_restart_observability_requested",
                "active_binary_observability_requested",
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(false));
            }
            for key in [
                "export_allowed",
                "export_request_accepted",
                "export_recorded",
                "export_persisted",
                "export_artifact_written",
                "export_stream_opened",
                "export_filesystem_written",
                "query_allowed",
                "query_registered",
                "query_endpoint_materialized",
                "query_index_recorded",
                "query_cache_written",
                "query_result_materialized",
                "observability_allowed",
                "observability_metric_emitted",
                "observability_log_recorded",
                "observability_trace_recorded",
                "observability_span_recorded",
                "observability_event_recorded",
                "observability_dashboard_materialized",
                "observability_alert_registered",
                "observability_slo_recorded",
                "ledger_observability_recorded",
                "index_observability_recorded",
                "delivery_observability_recorded",
                "activation_command_result_receipt_retention_policy_recorded",
                "activation_command_result_receipt_expiry_recorded",
                "activation_command_result_receipt_garbage_collection_scan_performed",
                "activation_command_result_receipt_audit_trail_recorded",
                "activation_command_result_receipt_immutable_evidence_recorded",
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
                "activation_from_export_allowed",
                "activation_from_query_allowed",
                "activation_from_observability_allowed",
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

    let export_query_observability_fixtures = serde_json::Value::Array(vec![
        export_query_observability_fixture(
            "provider-router-activation-command-result-receipt-export-missing-source-retention-gc",
            "blocked_noop",
            "source_retention_expiry_garbage_collection_report_required",
            serde_json::json!({
                "source_retention_expiry_gc_present": false,
                "source_retention_expiry_gc_ready": false,
                "export_requested": true,
            }),
        ),
        export_query_observability_fixture(
            "provider-router-activation-command-result-receipt-export-artifact-request",
            "blocked_export_noop",
            "export_artifact_write_denied",
            serde_json::json!({
                "export_requested": true,
                "export_file_requested": true,
            }),
        ),
        export_query_observability_fixture(
            "provider-router-activation-command-result-receipt-export-stream-request",
            "blocked_export_noop",
            "export_stream_open_denied",
            serde_json::json!({
                "export_requested": true,
                "export_stream_requested": true,
            }),
        ),
        export_query_observability_fixture(
            "provider-router-activation-command-result-receipt-query-endpoint-request",
            "blocked_query_noop",
            "query_endpoint_materialization_denied",
            serde_json::json!({
                "query_requested": true,
                "query_endpoint_requested": true,
            }),
        ),
        export_query_observability_fixture(
            "provider-router-activation-command-result-receipt-query-index-cache-request",
            "blocked_query_noop",
            "query_index_cache_recording_denied",
            serde_json::json!({
                "query_requested": true,
                "query_index_requested": true,
                "query_cache_requested": true,
            }),
        ),
        export_query_observability_fixture(
            "provider-router-activation-command-result-receipt-observability-metric-request",
            "blocked_observability_noop",
            "observability_metric_emission_denied",
            serde_json::json!({
                "observability_requested": true,
                "metric_requested": true,
            }),
        ),
        export_query_observability_fixture(
            "provider-router-activation-command-result-receipt-observability-trace-log-event-request",
            "blocked_observability_noop",
            "trace_span_log_event_recording_denied",
            serde_json::json!({
                "observability_requested": true,
                "trace_requested": true,
                "span_requested": true,
                "log_requested": true,
                "event_requested": true,
            }),
        ),
        export_query_observability_fixture(
            "provider-router-activation-command-result-receipt-dashboard-alert-slo-request",
            "blocked_observability_noop",
            "dashboard_alert_slo_materialization_denied",
            serde_json::json!({
                "observability_requested": true,
                "dashboard_requested": true,
                "alert_requested": true,
                "slo_requested": true,
            }),
        ),
        export_query_observability_fixture(
            "provider-router-activation-command-result-receipt-activation-memory-kg-provider-observability",
            "blocked_observability_noop",
            "activation_memory_kg_provider_observability_denied",
            serde_json::json!({
                "observability_requested": true,
                "activation_from_observability_requested": true,
                "memory_store_observability_requested": true,
                "live_kg_observability_requested": true,
                "rollback_observability_requested": true,
                "secret_material_observability_requested": true,
                "provider_prompt_observability_requested": true,
            }),
        ),
        export_query_observability_fixture(
            "provider-router-activation-command-result-receipt-external-public-install-observability",
            "blocked_observability_noop",
            "external_public_install_restart_active_binary_observability_denied",
            serde_json::json!({
                "observability_requested": true,
                "ledger_observability_requested": true,
                "index_observability_requested": true,
                "delivery_observability_requested": true,
                "external_send_observability_requested": true,
                "public_claim_observability_requested": true,
                "release_artifact_observability_requested": true,
                "install_observability_requested": true,
                "service_restart_observability_requested": true,
                "active_binary_observability_requested": true,
            }),
        ),
    ]);
    let export_query_observability_fixture_count = export_query_observability_fixtures
        .as_array()
        .map(std::vec::Vec::len)
        .unwrap_or(0);
    let fixtures_hash = sha256_json_value(&export_query_observability_fixtures);
    let source_report_sha256 = sha256_json_value(&source);
    let contract_hash = sha256_text_value(&format!(
        "runtime-provider-router-activation-command-result-receipt-export-query-observability-denial:v1:source={source_report_sha256}:fixtures={fixtures_hash}:export=0:query=0:observability=0:authority=0"
    ));
    let policy_hash = sha256_text_value(
        "runtime-provider-router-activation-command-result-receipt-export-query-observability-denial:v1:no-export:no-query:no-observability:no-provider-model-memory-kg-secret-external-install-restart-binary-public-authority",
    );
    let side_effect_hash = sha256_text_value(
        "runtime-provider-router-export-query-observability-side-effects=false;fixtures=10;export=0;query=0;observability=0;record=0;persist=0;activation=0;provider=0;model=0;memory=0;kg=0;secret=0;external=0;install=0",
    );
    let denials = vec![
        "source_retention_expiry_garbage_collection_report_required",
        "export_request_acceptance_denied",
        "export_recording_denied",
        "export_persistence_denied",
        "export_artifact_write_denied",
        "export_stream_open_denied",
        "query_request_acceptance_denied",
        "query_registration_denied",
        "query_endpoint_materialization_denied",
        "query_index_recording_denied",
        "query_cache_write_denied",
        "query_result_materialization_denied",
        "observability_request_acceptance_denied",
        "metric_emission_denied",
        "log_recording_denied",
        "trace_recording_denied",
        "span_recording_denied",
        "event_recording_denied",
        "dashboard_materialization_denied",
        "alert_registration_denied",
        "slo_recording_denied",
        "ledger_observability_recording_denied",
        "index_observability_recording_denied",
        "delivery_observability_recording_denied",
        "activation_from_export_query_observability_denied",
        "memory_kg_observability_denied",
        "rollback_observability_denied",
        "secret_material_observability_denied",
        "provider_prompt_observability_denied",
        "external_public_install_restart_active_binary_observability_denied",
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
            "gate": "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_route",
            "source_command": "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-export-query-observability-denial --json",
            "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_EXPORT_QUERY_OBSERVABILITY_DENIAL_ENDPOINT,
            "native_route": true,
            "compatibility_mode": "native_runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_status",
            "side_effect_free": true,
            "base_url": "native",
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_activation_command_result_receipt_retention_expiry_garbage_collection_route_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_RETENTION_EXPIRY_GARBAGE_COLLECTION_DENIAL_ENDPOINT,
            "source_activation_command_result_receipt_retention_expiry_garbage_collection_gate": source_str("gate"),
            "source_activation_command_result_receipt_retention_expiry_garbage_collection_ready": source_ready,
            "source_activation_command_result_receipt_retention_expiry_garbage_collection_status": source_str("runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_status"),
            "source_activation_command_result_receipt_retention_expiry_garbage_collection_report_sha256": source_report_sha256,
            "source_retention_expiry_garbage_collection_contract_hash_sha256": source_hash_str("retention_expiry_garbage_collection_contract_hash_sha256"),
            "source_retention_expiry_garbage_collection_policy_hash_sha256": source_hash_str("retention_expiry_garbage_collection_policy_hash_sha256"),
            "source_activation_command_result_receipt_audit_trail_immutable_evidence_ready": source_bool("runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready"),
            "source_activation_command_result_receipt_audit_trail_immutable_evidence_report_sha256": source_hash_str("source_activation_command_result_receipt_audit_trail_immutable_evidence_report_sha256"),
            "source_activation_command_result_receipt_cancellation_supersession_ready": source_bool("runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready"),
            "source_activation_command_result_receipt_cancellation_supersession_report_sha256": source_hash_str("source_activation_command_result_receipt_cancellation_supersession_report_sha256"),
            "source_activation_command_result_receipt_ordering_monotonicity_ready": source_bool("runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready"),
            "source_activation_command_result_receipt_ordering_monotonicity_report_sha256": source_hash_str("source_activation_command_result_receipt_ordering_monotonicity_report_sha256"),
            "source_activation_command_result_receipt_replay_idempotency_ready": source_bool("runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready"),
            "source_activation_command_result_receipt_replay_idempotency_report_sha256": source_hash_str("source_activation_command_result_receipt_replay_idempotency_report_sha256"),
            "source_activation_command_result_receipt_no_persistence_ready": source_bool("runtime_provider_router_activation_command_result_receipt_no_persistence_ready"),
            "source_activation_command_result_receipt_no_persistence_report_sha256": source_hash_str("source_activation_command_result_receipt_no_persistence_report_sha256"),
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
            "runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_route_enabled": true,
            "runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_ready": true,
            "runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_status": "blocked",
            "runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_no_persistence_ready": source_bool("runtime_provider_router_activation_command_result_receipt_no_persistence_ready"),
            "activation_command_result_receipt_export_query_observability_schema_version": "memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_v1",
            "activation_command_result_receipt_export_query_observability_mode": "runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_no_export_no_query_no_observability",
            "activation_command_result_receipt_export_query_observability_decision": "runtime_provider_router_activation_command_result_receipt_cannot_be_exported_queried_observed_or_promoted_into_authority",
            "minimum_required_samples": 24,
            "export_query_observability_fixtures_sha256": fixtures_hash,
            "export_query_observability_contract_hash_sha256": contract_hash,
            "export_query_observability_policy_hash_sha256": policy_hash,
            "side_effect_hash_sha256": side_effect_hash,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "retention_expiry_garbage_collection_surface_count": source_u64("retention_expiry_garbage_collection_surface_count"),
            "retention_expiry_garbage_collection_fixture_count": source_u64("retention_expiry_garbage_collection_fixture_count"),
            "source_blocked_retention_expiry_garbage_collection_fixture_count": source_u64("blocked_retention_expiry_garbage_collection_fixture_count"),
            "source_accepted_retention_expiry_garbage_collection_fixture_count": source_u64("accepted_retention_expiry_garbage_collection_fixture_count"),
            "source_retention_performed_count": source_u64("retention_performed_count"),
            "source_expiry_performed_count": source_u64("expiry_performed_count"),
            "source_garbage_collection_performed_count": source_u64("garbage_collection_performed_count"),
            "export_query_observability_surface_count": 12,
            "export_query_observability_surface_ready_count": 12,
            "export_query_observability_side_effect_free_surface_count": 12,
            "export_query_observability_surfaces": [
                "source_retention_expiry_garbage_collection_report_required",
                "export_request_shape_denied",
                "export_artifact_write_denied",
                "export_stream_open_denied",
                "query_endpoint_materialization_denied",
                "query_index_cache_recording_denied",
                "observability_metric_emission_denied",
                "trace_span_log_event_recording_denied",
                "dashboard_alert_slo_materialization_denied",
                "ledger_index_delivery_observability_evidence_denied",
                "activation_memory_kg_provider_observability_denied",
                "external_public_install_restart_active_binary_observability_denied"
            ],
            "export_query_observability_fixtures": export_query_observability_fixtures,
            "export_query_observability_fixture_count": export_query_observability_fixture_count,
            "blocked_export_query_observability_fixture_count": export_query_observability_fixture_count,
            "noop_export_query_observability_fixture_count": export_query_observability_fixture_count,
            "allowed_export_query_observability_fixture_count": 0,
            "accepted_export_query_observability_fixture_count": 0,
            "export_denied_count": export_query_observability_fixture_count,
            "query_denied_count": export_query_observability_fixture_count,
            "observability_denied_count": export_query_observability_fixture_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "export_performed_count": 0,
            "query_performed_count": 0,
            "observability_performed_count": 0,
            "export_recorded_count": 0,
            "export_persisted_count": 0,
            "export_artifact_written_count": 0,
            "export_stream_opened_count": 0,
            "query_registered_count": 0,
            "query_endpoint_materialized_count": 0,
            "query_index_recorded_count": 0,
            "query_cache_written_count": 0,
            "query_result_materialized_count": 0,
            "observability_metric_emitted_count": 0,
            "observability_log_recorded_count": 0,
            "observability_trace_recorded_count": 0,
            "observability_span_recorded_count": 0,
            "observability_event_recorded_count": 0,
            "observability_dashboard_materialized_count": 0,
            "observability_alert_registered_count": 0,
            "observability_slo_recorded_count": 0,
            "ledger_observability_recorded_count": 0,
            "index_observability_recorded_count": 0,
            "delivery_observability_recorded_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "denied_by_export_query_observability": denials_value,
            "denied_by_export_query_observability_count": denied_count,
            "denied_by_activation_command_result_receipt_export_query_observability": denials_value,
            "denied_by_activation_command_result_receipt_export_query_observability_count": denied_count,
            "allowed_next_actions": [
                {
                    "action": "review_runtime_provider_router_activation_command_result_receipt_export_query_observability_denial",
                    "status": "allowed_report_only",
                    "exports_receipt": false,
                    "registers_query": false,
                    "records_observability": false,
                    "mutates_runtime": false,
                    "invokes_model": false
                },
                {
                    "action": "stage_runtime_provider_router_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial",
                    "status": "allowed_report_only_next_slice",
                    "persists_summary": false,
                    "persists_briefing": false,
                    "delivers_summary": false,
                    "mutates_runtime": false,
                    "invokes_model": false
                },
                {
                    "action": "run_full_light_preflight",
                    "status": "allowed_verification_only",
                    "exports_receipt": false,
                    "registers_query": false,
                    "records_observability": false,
                    "mutates_runtime": false,
                    "invokes_model": false,
                    "writes_kg": false
                }
            ],
            "source_retention_expiry_garbage_collection_report_required": true,
            "export_acceptance_forbidden": true,
            "export_recording_forbidden": true,
            "export_persistence_forbidden": true,
            "export_artifact_write_forbidden": true,
            "export_stream_forbidden": true,
            "query_registration_forbidden": true,
            "query_endpoint_materialization_forbidden": true,
            "query_index_cache_forbidden": true,
            "observability_metric_forbidden": true,
            "observability_trace_log_event_forbidden": true,
            "dashboard_alert_slo_forbidden": true,
            "activation_from_export_query_observability_forbidden": true,
            "runtime_provider_memory_kg_observability_forbidden": true,
            "secret_read_forbidden": true,
            "external_public_install_restart_active_binary_observability_forbidden": true,
        }),
    );

    if let Some(report_object) = report.as_object_mut() {
        for key in [
            "activation_command_result_receipt_export_allowed",
            "activation_command_result_receipt_export_request_accepted",
            "activation_command_result_receipt_export_recorded",
            "activation_command_result_receipt_export_persisted",
            "activation_command_result_receipt_export_artifact_written",
            "activation_command_result_receipt_export_stream_opened",
            "activation_command_result_receipt_export_filesystem_written",
            "activation_command_result_receipt_query_allowed",
            "activation_command_result_receipt_query_registered",
            "activation_command_result_receipt_query_endpoint_materialized",
            "activation_command_result_receipt_query_index_recorded",
            "activation_command_result_receipt_query_cache_written",
            "activation_command_result_receipt_query_result_materialized",
            "activation_command_result_receipt_observability_allowed",
            "activation_command_result_receipt_observability_metric_emitted",
            "activation_command_result_receipt_observability_log_recorded",
            "activation_command_result_receipt_observability_trace_recorded",
            "activation_command_result_receipt_observability_span_recorded",
            "activation_command_result_receipt_observability_event_recorded",
            "activation_command_result_receipt_observability_dashboard_materialized",
            "activation_command_result_receipt_observability_alert_registered",
            "activation_command_result_receipt_observability_slo_recorded",
            "activation_command_result_receipt_ledger_observability_recorded",
            "activation_command_result_receipt_index_observability_recorded",
            "activation_command_result_receipt_delivery_observability_recorded",
            "activation_command_result_receipt_retention_policy_recorded",
            "activation_command_result_receipt_retention_index_recorded",
            "activation_command_result_receipt_expiry_recorded",
            "activation_command_result_receipt_garbage_collection_scan_performed",
            "activation_command_result_receipt_audit_trail_recorded",
            "activation_command_result_receipt_immutable_evidence_recorded",
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
            "activation_allowed_by_result_receipt_export",
            "activation_allowed_by_result_receipt_query",
            "activation_allowed_by_result_receipt_observability",
            "activation_allowed_by_result_receipt_retention",
            "activation_allowed_by_result_receipt_expiry",
            "activation_allowed_by_result_receipt_garbage_collection",
            "activation_allowed_by_result_receipt_audit_trail",
            "activation_allowed_by_result_receipt_immutable_evidence",
            "activation_allowed_by_result_receipt",
            "activation_from_export_allowed",
            "activation_from_query_allowed",
            "activation_from_observability_allowed",
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
            "activation_command_result_receipt_export_recorded",
            "activation_command_result_receipt_export_persisted",
            "activation_command_result_receipt_export_artifact_written",
            "activation_command_result_receipt_export_stream_opened",
            "activation_command_result_receipt_export_filesystem_written",
            "activation_command_result_receipt_query_registered",
            "activation_command_result_receipt_query_endpoint_materialized",
            "activation_command_result_receipt_query_index_recorded",
            "activation_command_result_receipt_query_cache_written",
            "activation_command_result_receipt_query_result_materialized",
            "activation_command_result_receipt_observability_metric_emitted",
            "activation_command_result_receipt_observability_log_recorded",
            "activation_command_result_receipt_observability_trace_recorded",
            "activation_command_result_receipt_observability_span_recorded",
            "activation_command_result_receipt_observability_event_recorded",
            "activation_command_result_receipt_observability_dashboard_materialized",
            "activation_command_result_receipt_observability_alert_registered",
            "activation_command_result_receipt_observability_slo_recorded",
            "activation_command_result_receipt_ledger_observability_recorded",
            "activation_command_result_receipt_index_observability_recorded",
            "activation_command_result_receipt_delivery_observability_recorded",
            "activation_command_result_receipt_retention_policy_recorded",
            "activation_command_result_receipt_retention_index_recorded",
            "activation_command_result_receipt_expiry_recorded",
            "activation_command_result_receipt_garbage_collection_scan_performed",
            "activation_command_result_receipt_audit_trail_recorded",
            "activation_command_result_receipt_immutable_evidence_recorded",
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

fn hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_report();
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
            "runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_ready",
        )
        && source_str(
            "runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_status",
        ) == "blocked"
        && source_bool(
            "runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready",
        )
        && source_bool(
            "runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready",
        )
        && source_bool(
            "runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready",
        )
        && source_bool(
            "runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready",
        )
        && source_bool(
            "runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready",
        )
        && source_bool(
            "runtime_provider_router_activation_command_result_receipt_no_persistence_ready",
        )
        && source_u64("export_query_observability_surface_count") == 12
        && source_u64("export_query_observability_fixture_count") == 10
        && source_u64("accepted_export_query_observability_fixture_count") == 0
        && source_u64("export_performed_count") == 0
        && source_u64("query_performed_count") == 0
        && source_u64("observability_performed_count") == 0
        && !source_bool("activation_command_result_receipt_export_recorded")
        && !source_bool("activation_command_result_receipt_query_registered")
        && !source_bool("activation_command_result_receipt_observability_metric_emitted")
        && !source_bool("activation_command_result_receipt_recorded")
        && !source_bool("activation_command_result_receipt_persisted")
        && !source_bool("activation_command_result_receipt_accepted")
        && !source_bool("activation_allowed_by_result_receipt")
        && !source_bool("activation_activated")
        && !source_bool("runtime_router_mutated")
        && !source_bool("provider_invoked")
        && !source_bool("model_invoked")
        && !source_bool("credential_read")
        && !source_bool("secret_file_read")
        && !source_bool("memory_store_write_performed")
        && !source_bool("live_kg_write_performed")
        && !source_bool("telegram_send_performed")
        && !source_bool("channel_send_performed")
        && !source_bool("external_send_performed")
        && !source_bool("install_executed")
        && !source_bool("service_restart_performed")
        && !source_bool("active_binary_mutated");
    let report_ready = source_ready && route_count_source_command_accepted;

    let operator_summary_briefing_fixture =
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
                "operator_summary_briefing_status".to_string(),
                serde_json::Value::String(status.to_string()),
            );
            fixture.insert(
                "operator_facing_summary_briefing_status".to_string(),
                serde_json::Value::String(status.to_string()),
            );
            fixture.insert(
                "reason".to_string(),
                serde_json::Value::String(reason.to_string()),
            );
            for key in [
                "source_export_query_observability_present",
                "source_export_query_observability_ready",
                "summary_briefing_noop_confirmed",
                "receipt_noop_confirmed",
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(true));
            }
            for key in [
                "operator_summary_requested",
                "operator_briefing_requested",
                "operator_summary_materialization_requested",
                "operator_briefing_materialization_requested",
                "operator_summary_persistence_requested",
                "operator_briefing_persistence_requested",
                "operator_summary_filesystem_write_requested",
                "operator_briefing_filesystem_write_requested",
                "channel_delivery_requested",
                "telegram_send_requested",
                "activation_from_summary_briefing_requested",
                "memory_store_summary_requested",
                "live_kg_summary_requested",
                "rollback_summary_requested",
                "secret_material_summary_requested",
                "provider_prompt_summary_requested",
                "external_send_summary_requested",
                "public_claim_summary_requested",
                "release_artifact_summary_requested",
                "install_summary_requested",
                "service_restart_summary_requested",
                "active_binary_summary_requested",
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(false));
            }
            for key in [
                "operator_summary_allowed",
                "operator_summary_request_accepted",
                "operator_summary_recorded",
                "operator_summary_persisted",
                "operator_summary_materialized",
                "operator_summary_filesystem_written",
                "operator_summary_delivered",
                "operator_summary_channel_delivery_performed",
                "operator_briefing_allowed",
                "operator_briefing_request_accepted",
                "operator_briefing_recorded",
                "operator_briefing_persisted",
                "operator_briefing_materialized",
                "operator_briefing_filesystem_written",
                "operator_briefing_delivered",
                "operator_briefing_channel_delivery_performed",
                "telegram_send_performed",
                "channel_send_performed",
                "external_send_performed",
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
                "activation_allowed_by_result_receipt_operator_summary",
                "activation_allowed_by_result_receipt_operator_briefing",
                "activation_allowed_by_result_receipt_summary_briefing",
                "activation_allowed_by_result_receipt",
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

    let operator_summary_briefing_fixtures = serde_json::Value::Array(vec![
        operator_summary_briefing_fixture(
            "provider-router-activation-command-result-receipt-operator-summary-missing-source-export-query-observability",
            "blocked_noop",
            "source_export_query_observability_report_required",
            serde_json::json!({
                "source_export_query_observability_present": false,
                "source_export_query_observability_ready": false,
                "operator_summary_requested": true
            }),
        ),
        operator_summary_briefing_fixture(
            "provider-router-activation-command-result-receipt-operator-summary-request",
            "blocked_summary_noop",
            "operator_summary_request_shape_denied",
            serde_json::json!({"operator_summary_requested": true}),
        ),
        operator_summary_briefing_fixture(
            "provider-router-activation-command-result-receipt-operator-briefing-request",
            "blocked_briefing_noop",
            "operator_briefing_request_shape_denied",
            serde_json::json!({"operator_briefing_requested": true}),
        ),
        operator_summary_briefing_fixture(
            "provider-router-activation-command-result-receipt-operator-summary-materialization-request",
            "blocked_summary_noop",
            "operator_summary_materialization_denied",
            serde_json::json!({
                "operator_summary_requested": true,
                "operator_summary_materialization_requested": true
            }),
        ),
        operator_summary_briefing_fixture(
            "provider-router-activation-command-result-receipt-operator-briefing-materialization-request",
            "blocked_briefing_noop",
            "operator_briefing_materialization_denied",
            serde_json::json!({
                "operator_briefing_requested": true,
                "operator_briefing_materialization_requested": true
            }),
        ),
        operator_summary_briefing_fixture(
            "provider-router-activation-command-result-receipt-operator-summary-persistence-filesystem-request",
            "blocked_summary_noop",
            "operator_summary_persistence_filesystem_write_denied",
            serde_json::json!({
                "operator_summary_requested": true,
                "operator_summary_persistence_requested": true,
                "operator_summary_filesystem_write_requested": true
            }),
        ),
        operator_summary_briefing_fixture(
            "provider-router-activation-command-result-receipt-operator-briefing-persistence-filesystem-request",
            "blocked_briefing_noop",
            "operator_briefing_persistence_filesystem_write_denied",
            serde_json::json!({
                "operator_briefing_requested": true,
                "operator_briefing_persistence_requested": true,
                "operator_briefing_filesystem_write_requested": true
            }),
        ),
        operator_summary_briefing_fixture(
            "provider-router-activation-command-result-receipt-operator-summary-briefing-channel-delivery-request",
            "blocked_delivery_noop",
            "operator_summary_briefing_channel_delivery_denied",
            serde_json::json!({
                "operator_summary_requested": true,
                "operator_briefing_requested": true,
                "channel_delivery_requested": true,
                "telegram_send_requested": true
            }),
        ),
        operator_summary_briefing_fixture(
            "provider-router-activation-command-result-receipt-operator-summary-briefing-activation-memory-kg-provider",
            "blocked_summary_noop",
            "activation_memory_kg_rollback_secret_provider_summary_briefing_denied",
            serde_json::json!({
                "operator_summary_requested": true,
                "operator_briefing_requested": true,
                "activation_from_summary_briefing_requested": true,
                "memory_store_summary_requested": true,
                "live_kg_summary_requested": true,
                "rollback_summary_requested": true,
                "secret_material_summary_requested": true,
                "provider_prompt_summary_requested": true
            }),
        ),
        operator_summary_briefing_fixture(
            "provider-router-activation-command-result-receipt-operator-summary-briefing-external-public-install",
            "blocked_delivery_noop",
            "external_public_install_restart_active_binary_summary_briefing_denied",
            serde_json::json!({
                "operator_summary_requested": true,
                "operator_briefing_requested": true,
                "external_send_summary_requested": true,
                "public_claim_summary_requested": true,
                "release_artifact_summary_requested": true,
                "install_summary_requested": true,
                "service_restart_summary_requested": true,
                "active_binary_summary_requested": true
            }),
        ),
    ]);
    let operator_summary_briefing_fixture_count = operator_summary_briefing_fixtures
        .as_array()
        .map(std::vec::Vec::len)
        .unwrap_or(0);
    let source_report_sha256 = sha256_json_value(&source);
    let fixtures_hash = sha256_json_value(&operator_summary_briefing_fixtures);
    let contract_hash = sha256_text_value(&format!(
        "runtime-provider-router-activation-command-result-receipt-operator-facing-summary-briefing-non-persistence-denial:v1:source={source_report_sha256}:fixtures={fixtures_hash}:summary=0:briefing=0:delivery=0:authority=0"
    ));
    let policy_hash = sha256_text_value(
        "runtime-provider-router-activation-command-result-receipt-operator-facing-summary-briefing-non-persistence-denial:v1:no-summary:no-briefing:no-record:no-persist:no-materialize:no-delivery:no-authority:no-provider-model-memory-kg-secret-external-install-restart-binary",
    );
    let side_effect_hash = sha256_text_value(
        "runtime-provider-router-operator-facing-summary-briefing-side-effects=false;summary=0;briefing=0;delivery=0;record=0;persist=0;activation=0;provider=0;model=0;memory=0;kg=0;secret=0;external=0;install=0",
    );
    let denials = vec![
        "source_export_query_observability_report_required",
        "operator_summary_request_acceptance_denied",
        "operator_briefing_request_acceptance_denied",
        "operator_summary_recording_denied",
        "operator_briefing_recording_denied",
        "operator_summary_persistence_denied",
        "operator_briefing_persistence_denied",
        "operator_summary_materialization_denied",
        "operator_briefing_materialization_denied",
        "operator_summary_filesystem_write_denied",
        "operator_briefing_filesystem_write_denied",
        "operator_summary_delivery_denied",
        "operator_briefing_delivery_denied",
        "telegram_send_denied",
        "channel_delivery_denied",
        "activation_from_summary_briefing_denied",
        "memory_kg_summary_briefing_denied",
        "rollback_summary_briefing_denied",
        "secret_material_summary_briefing_denied",
        "provider_prompt_summary_briefing_denied",
        "external_public_install_restart_active_binary_summary_briefing_denied",
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
            "gate": "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_route",
            "source_command": "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-operator-facing-summary-briefing-non-persistence-denial --json",
            "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_OPERATOR_FACING_SUMMARY_BRIEFING_NON_PERSISTENCE_DENIAL_ENDPOINT,
            "native_route": true,
            "compatibility_mode": "native_runtime_provider_router_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_status",
            "side_effect_free": true,
            "base_url": "native",
            "source_activation_command_result_receipt_export_query_observability_route_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_EXPORT_QUERY_OBSERVABILITY_DENIAL_ENDPOINT,
            "source_activation_command_result_receipt_export_query_observability_gate": source_str("gate"),
            "source_activation_command_result_receipt_export_query_observability_ready": source_ready,
            "source_activation_command_result_receipt_export_query_observability_status": source_str("runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_status"),
            "source_activation_command_result_receipt_export_query_observability_report_sha256": source_report_sha256,
            "source_export_query_observability_contract_hash_sha256": source_hash_str("export_query_observability_contract_hash_sha256"),
            "source_export_query_observability_policy_hash_sha256": source_hash_str("export_query_observability_policy_hash_sha256"),
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
            "runtime_provider_router_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_route_enabled": true,
            "runtime_provider_router_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready": true,
            "runtime_provider_router_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_status": "blocked",
            "runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_no_persistence_ready": source_bool("runtime_provider_router_activation_command_result_receipt_no_persistence_ready"),
            "activation_command_result_receipt_operator_facing_summary_briefing_schema_version": "memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_v1",
            "activation_command_result_receipt_operator_facing_summary_briefing_mode": "runtime_provider_router_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_no_summary_no_briefing_no_delivery",
            "activation_command_result_receipt_operator_facing_summary_briefing_decision": "runtime_provider_router_activation_command_result_receipt_cannot_record_persist_materialize_deliver_or_promote_operator_summary_briefing_into_authority",
            "minimum_required_samples": 24,
            "operator_summary_briefing_fixtures_sha256": fixtures_hash,
            "operator_summary_briefing_contract_hash_sha256": contract_hash,
            "operator_summary_briefing_policy_hash_sha256": policy_hash,
            "side_effect_hash_sha256": side_effect_hash,
            "export_query_observability_surface_count": source_u64("export_query_observability_surface_count"),
            "export_query_observability_fixture_count": source_u64("export_query_observability_fixture_count"),
            "operator_facing_summary_briefing_surface_count": 12,
            "operator_facing_summary_briefing_surface_ready_count": 12,
            "operator_facing_summary_briefing_side_effect_free_surface_count": 12,
            "operator_facing_summary_briefing_surfaces": [
                "source_export_query_observability_report_required",
                "operator_summary_request_shape_denied",
                "operator_briefing_request_shape_denied",
                "summary_materialization_denied",
                "briefing_materialization_denied",
                "summary_persistence_denied",
                "briefing_persistence_denied",
                "summary_delivery_denied",
                "briefing_delivery_denied",
                "activation_from_summary_briefing_denied",
                "memory_kg_rollback_secret_provider_summary_briefing_denied",
                "external_public_install_restart_active_binary_summary_briefing_denied"
            ],
            "operator_facing_summary_briefing_fixtures": operator_summary_briefing_fixtures,
            "operator_facing_summary_briefing_fixture_count": operator_summary_briefing_fixture_count,
            "blocked_operator_facing_summary_briefing_fixture_count": operator_summary_briefing_fixture_count,
            "noop_operator_facing_summary_briefing_fixture_count": operator_summary_briefing_fixture_count,
            "allowed_operator_facing_summary_briefing_fixture_count": 0,
            "accepted_operator_facing_summary_briefing_fixture_count": 0,
            "operator_summary_denied_count": operator_summary_briefing_fixture_count,
            "operator_briefing_denied_count": operator_summary_briefing_fixture_count,
            "operator_summary_performed_count": 0,
            "operator_briefing_performed_count": 0,
            "operator_summary_briefing_delivery_performed_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "activation_command_result_receipt_operator_summary_allowed": false,
            "activation_command_result_receipt_operator_summary_request_accepted": false,
            "activation_command_result_receipt_operator_summary_recorded": false,
            "activation_command_result_receipt_operator_summary_persisted": false,
            "activation_command_result_receipt_operator_summary_materialized": false,
            "activation_command_result_receipt_operator_summary_filesystem_written": false,
            "activation_command_result_receipt_operator_summary_delivered": false,
            "activation_command_result_receipt_operator_summary_channel_delivery_performed": false,
            "activation_command_result_receipt_operator_briefing_allowed": false,
            "activation_command_result_receipt_operator_briefing_request_accepted": false,
            "activation_command_result_receipt_operator_briefing_recorded": false,
            "activation_command_result_receipt_operator_briefing_persisted": false,
            "activation_command_result_receipt_operator_briefing_materialized": false,
            "activation_command_result_receipt_operator_briefing_filesystem_written": false,
            "activation_command_result_receipt_operator_briefing_delivered": false,
            "activation_command_result_receipt_operator_briefing_channel_delivery_performed": false,
            "activation_command_result_receipt_operator_summary_briefing_channel_delivery_performed": false,
            "operator_summary_recorded_count": 0,
            "operator_summary_persisted_count": 0,
            "operator_briefing_recorded_count": 0,
            "operator_briefing_persisted_count": 0,
            "operator_summary_delivered_count": 0,
            "operator_briefing_delivered_count": 0,
            "operator_summary_briefing_channel_delivery_count": 0,
            "operator_summary_briefing_external_send_count": 0,
            "operator_summary_briefing_telegram_send_count": 0,
            "operator_summary_briefing_activation_authority_derived_count": 0,
            "operator_summary_briefing_live_execution_allowed_count": 0,
            "activation_allowed_by_result_receipt_operator_summary": false,
            "activation_allowed_by_result_receipt_operator_briefing": false,
            "activation_allowed_by_result_receipt_summary_briefing": false,
            "activation_from_summary_briefing_forbidden": true,
            "runtime_provider_memory_kg_summary_briefing_forbidden": true,
            "external_public_install_restart_active_binary_summary_briefing_forbidden": true,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "denied_by_operator_facing_summary_briefing": denials_value,
            "denied_by_operator_facing_summary_briefing_count": denied_count,
            "denied_by_activation_command_result_receipt_operator_facing_summary_briefing": denials_value,
            "denied_by_activation_command_result_receipt_operator_facing_summary_briefing_count": denied_count,
            "allowed_next_actions": [
                {
                    "action": "review_runtime_provider_router_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial",
                    "status": "allowed_report_only",
                    "persists_summary": false,
                    "persists_briefing": false,
                    "delivers_summary": false,
                    "mutates_runtime": false,
                    "invokes_model": false
                },
                {
                    "action": "stage_runtime_provider_router_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial",
                    "status": "allowed_report_only_next_slice",
                    "accepts_operator_acknowledgement": false,
                    "persists_acknowledgement": false,
                    "activates_runtime": false,
                    "invokes_model": false,
                    "writes_kg": false
                },
                {
                    "action": "run_full_light_preflight",
                    "status": "allowed_verification_only",
                    "persists_summary": false,
                    "persists_briefing": false,
                    "delivers_summary": false,
                    "mutates_runtime": false,
                    "invokes_model": false,
                    "writes_kg": false
                }
            ],
            "source_export_query_observability_report_required": true,
            "operator_summary_acceptance_forbidden": true,
            "operator_briefing_acceptance_forbidden": true,
            "operator_summary_recording_forbidden": true,
            "operator_briefing_recording_forbidden": true,
            "operator_summary_persistence_forbidden": true,
            "operator_briefing_persistence_forbidden": true,
            "operator_summary_materialization_forbidden": true,
            "operator_briefing_materialization_forbidden": true,
            "operator_summary_filesystem_write_forbidden": true,
            "operator_briefing_filesystem_write_forbidden": true,
            "operator_summary_delivery_forbidden": true,
            "operator_briefing_delivery_forbidden": true,
            "telegram_send_forbidden": true,
            "channel_delivery_forbidden": true,
            "secret_read_forbidden": true,
        }),
    );

    if let Some(report_object) = report.as_object_mut() {
        for key in [
            "activation_command_result_receipt_operator_summary_allowed",
            "activation_command_result_receipt_operator_summary_request_accepted",
            "activation_command_result_receipt_operator_summary_recorded",
            "activation_command_result_receipt_operator_summary_persisted",
            "activation_command_result_receipt_operator_summary_materialized",
            "activation_command_result_receipt_operator_summary_filesystem_written",
            "activation_command_result_receipt_operator_summary_delivered",
            "activation_command_result_receipt_operator_summary_channel_delivery_performed",
            "activation_command_result_receipt_operator_briefing_allowed",
            "activation_command_result_receipt_operator_briefing_request_accepted",
            "activation_command_result_receipt_operator_briefing_recorded",
            "activation_command_result_receipt_operator_briefing_persisted",
            "activation_command_result_receipt_operator_briefing_materialized",
            "activation_command_result_receipt_operator_briefing_filesystem_written",
            "activation_command_result_receipt_operator_briefing_delivered",
            "activation_command_result_receipt_operator_briefing_channel_delivery_performed",
            "activation_command_result_receipt_operator_summary_briefing_channel_delivery_performed",
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
            "activation_allowed_by_result_receipt_operator_summary",
            "activation_allowed_by_result_receipt_operator_briefing",
            "activation_allowed_by_result_receipt_summary_briefing",
            "activation_allowed_by_result_receipt",
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
            "activation_command_result_receipt_operator_summary_recorded",
            "activation_command_result_receipt_operator_summary_persisted",
            "activation_command_result_receipt_operator_summary_materialized",
            "activation_command_result_receipt_operator_summary_filesystem_written",
            "activation_command_result_receipt_operator_summary_delivered",
            "activation_command_result_receipt_operator_briefing_recorded",
            "activation_command_result_receipt_operator_briefing_persisted",
            "activation_command_result_receipt_operator_briefing_materialized",
            "activation_command_result_receipt_operator_briefing_filesystem_written",
            "activation_command_result_receipt_operator_briefing_delivered",
            "activation_command_result_receipt_operator_summary_briefing_channel_delivery_performed",
            "activation_command_result_receipt_recorded",
            "activation_command_result_receipt_persisted",
            "activation_command_result_receipt_accepted",
            "activation_activated",
            "runtime_router_mutated",
            "provider_invoked",
            "model_invoked",
            "credential_read",
            "secret_file_read",
            "memory_store_mutated",
            "live_kg_write_performed",
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

fn hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_summary =
        hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_report();
    let source_bool = |key: &str| {
        source_summary
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let source_u64 = |key: &str| {
        source_summary
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_str = |key: &str| {
        source_summary
            .get(key)
            .and_then(serde_json::Value::as_str)
            .unwrap_or("blocked")
            .to_string()
    };
    let source_hash_str = |key: &str| {
        source_summary
            .get(key)
            .and_then(serde_json::Value::as_str)
            .unwrap_or("")
            .to_string()
    };
    let source_status = source_str(
        "runtime_provider_router_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_status",
    );
    let source_ready = source_str("status") == "ready"
        && source_status == "blocked"
        && source_bool(
            "runtime_provider_router_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready",
        )
        && source_u64("operator_facing_summary_briefing_surface_count") == 12
        && source_u64("operator_facing_summary_briefing_fixture_count") == 10
        && source_u64("accepted_operator_facing_summary_briefing_fixture_count") == 0
        && source_u64("operator_summary_performed_count") == 0
        && source_u64("operator_briefing_performed_count") == 0
        && source_u64("operator_summary_briefing_delivery_performed_count") == 0
        && !source_bool("activation_command_result_receipt_operator_summary_recorded")
        && !source_bool("activation_command_result_receipt_operator_summary_persisted")
        && !source_bool("activation_command_result_receipt_operator_summary_delivered")
        && !source_bool("activation_command_result_receipt_operator_briefing_recorded")
        && !source_bool("activation_command_result_receipt_operator_briefing_persisted")
        && !source_bool("activation_command_result_receipt_operator_briefing_delivered")
        && !source_bool(
            "activation_command_result_receipt_operator_summary_briefing_channel_delivery_performed",
        )
        && !source_bool("telegram_send_performed")
        && !source_bool("channel_send_performed")
        && !source_bool("external_send_performed")
        && !source_bool("activation_command_result_receipt_recorded")
        && !source_bool("activation_command_result_receipt_persisted")
        && !source_bool("activation_command_result_receipt_accepted")
        && !source_bool("activation_allowed_by_result_receipt_summary_briefing")
        && !source_bool("activation_allowed_by_result_receipt")
        && !source_bool("activation_activated")
        && !source_bool("runtime_router_mutated")
        && !source_bool("provider_invoked")
        && !source_bool("model_invoked")
        && !source_bool("credential_read")
        && !source_bool("secret_file_read")
        && !source_bool("memory_store_write_performed")
        && !source_bool("memory_store_mutated")
        && !source_bool("live_kg_write_performed")
        && !source_bool("install_executed")
        && !source_bool("service_restart_performed")
        && !source_bool("active_binary_mutated");
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let report_ready = source_ready && route_count_source_command_accepted;
    let source_report_sha256 = sha256_json_value(&source_summary);

    let final_ack_fixture =
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
                "final_operator_acknowledgement_requested".to_string(),
                serde_json::Value::Bool(false),
            );
            fixture.insert(
                "final_operator_acknowledgement_status".to_string(),
                serde_json::Value::String(status.to_string()),
            );
            fixture.insert(
                "reason".to_string(),
                serde_json::Value::String(reason.to_string()),
            );
            for key in [
                "source_summary_briefing_present",
                "source_summary_briefing_ready",
                "final_acknowledgement_noop_confirmed",
                "receipt_noop_confirmed",
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(true));
            }
            for key in [
                "acknowledgement_acceptance_requested",
                "acknowledgement_recording_requested",
                "acknowledgement_persistence_requested",
                "acknowledgement_filesystem_write_requested",
                "operator_identity_acceptance_requested",
                "operator_signature_acceptance_requested",
                "operator_timestamp_acceptance_requested",
                "acknowledgement_delivery_requested",
                "telegram_send_requested",
                "channel_delivery_requested",
                "final_state_promotion_requested",
                "completion_promotion_requested",
                "activation_from_acknowledgement_requested",
                "memory_store_acknowledgement_requested",
                "live_kg_acknowledgement_requested",
                "rollback_acknowledgement_requested",
                "secret_material_acknowledgement_requested",
                "provider_prompt_acknowledgement_requested",
                "external_send_acknowledgement_requested",
                "public_claim_acknowledgement_requested",
                "release_artifact_acknowledgement_requested",
                "install_acknowledgement_requested",
                "service_restart_acknowledgement_requested",
                "active_binary_acknowledgement_requested",
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(false));
            }
            for key in [
                "acknowledgement_allowed",
                "acknowledgement_request_accepted",
                "acknowledgement_accepted",
                "acknowledgement_recorded",
                "acknowledgement_persisted",
                "acknowledgement_materialized",
                "acknowledgement_filesystem_written",
                "acknowledgement_delivered",
                "acknowledgement_channel_delivery_performed",
                "acknowledgement_identity_accepted",
                "acknowledgement_signature_accepted",
                "acknowledgement_timestamp_accepted",
                "acknowledgement_final_state_promoted",
                "acknowledgement_completion_promoted",
                "operator_final_acceptance_recorded",
                "operator_final_acceptance_persisted",
                "operator_final_acceptance_materialized",
                "telegram_send_performed",
                "channel_send_performed",
                "external_send_performed",
                "activation_command_result_receipt_recorded",
                "activation_command_result_receipt_persisted",
                "activation_command_result_receipt_accepted",
                "activation_command_result_receipt_materialized",
                "activation_command_result_receipt_filesystem_written",
                "activation_command_completion_ack_recorded",
                "activation_command_completion_ack_persisted",
                "activation_command_completion_ack_accepted",
                "activation_command_completion_ack_delivered",
                "activation_allowed",
                "activation_performed",
                "live_mutation_execution_performed",
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
                "secret_material_read",
                "memory_store_write_performed",
                "memory_store_mutated",
                "live_kg_write_performed",
                "rollback_executed",
                "public_release_claimed",
                "public_release_published",
                "public_ga_claimed",
                "release_artifact_written",
                "install_executed",
                "launchd_mutated",
                "service_restart_performed",
                "service_restarted",
                "active_binary_mutated",
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(false));
            }
            let mut fixture = serde_json::Value::Object(fixture);
            extend_json_object(&mut fixture, extra);
            fixture
        };

    let final_acknowledgement_fixtures = serde_json::Value::Array(vec![
        final_ack_fixture(
            "provider-router-activation-result-receipt-final-ack-missing-source-summary-briefing",
            "blocked_noop",
            "source_summary_briefing_report_required",
            serde_json::json!({
                "source_summary_briefing_present": false,
                "source_summary_briefing_ready": false,
                "final_operator_acknowledgement_requested": true
            }),
        ),
        final_ack_fixture(
            "provider-router-activation-result-receipt-final-ack-request",
            "blocked_ack_noop",
            "final_operator_acknowledgement_request_shape_denied",
            serde_json::json!({"final_operator_acknowledgement_requested": true}),
        ),
        final_ack_fixture(
            "provider-router-activation-result-receipt-final-ack-acceptance-request",
            "blocked_acceptance_noop",
            "final_operator_acknowledgement_acceptance_denied",
            serde_json::json!({
                "final_operator_acknowledgement_requested": true,
                "acknowledgement_acceptance_requested": true
            }),
        ),
        final_ack_fixture(
            "provider-router-activation-result-receipt-final-ack-recording-request",
            "blocked_ack_noop",
            "final_operator_acknowledgement_recording_denied",
            serde_json::json!({
                "final_operator_acknowledgement_requested": true,
                "acknowledgement_recording_requested": true
            }),
        ),
        final_ack_fixture(
            "provider-router-activation-result-receipt-final-ack-persistence-filesystem-write-request",
            "blocked_ack_noop",
            "final_operator_acknowledgement_persistence_filesystem_write_denied",
            serde_json::json!({
                "final_operator_acknowledgement_requested": true,
                "acknowledgement_persistence_requested": true,
                "acknowledgement_filesystem_write_requested": true
            }),
        ),
        final_ack_fixture(
            "provider-router-activation-result-receipt-final-ack-identity-signature-timestamp-request",
            "blocked_acceptance_noop",
            "operator_identity_signature_timestamp_acknowledgement_acceptance_denied",
            serde_json::json!({
                "final_operator_acknowledgement_requested": true,
                "operator_identity_acceptance_requested": true,
                "operator_signature_acceptance_requested": true,
                "operator_timestamp_acceptance_requested": true
            }),
        ),
        final_ack_fixture(
            "provider-router-activation-result-receipt-final-ack-delivery-request",
            "blocked_delivery_noop",
            "final_operator_acknowledgement_delivery_denied",
            serde_json::json!({
                "final_operator_acknowledgement_requested": true,
                "acknowledgement_delivery_requested": true,
                "telegram_send_requested": true,
                "channel_delivery_requested": true
            }),
        ),
        final_ack_fixture(
            "provider-router-activation-result-receipt-final-state-promotion-request",
            "blocked_promotion_noop",
            "final_state_completion_promotion_denied",
            serde_json::json!({
                "final_operator_acknowledgement_requested": true,
                "final_state_promotion_requested": true,
                "completion_promotion_requested": true
            }),
        ),
        final_ack_fixture(
            "provider-router-activation-result-receipt-final-ack-activation-memory-kg-provider-request",
            "blocked_ack_noop",
            "activation_memory_kg_rollback_secret_provider_acknowledgement_denied",
            serde_json::json!({
                "final_operator_acknowledgement_requested": true,
                "activation_from_acknowledgement_requested": true,
                "memory_store_acknowledgement_requested": true,
                "live_kg_acknowledgement_requested": true,
                "rollback_acknowledgement_requested": true,
                "secret_material_acknowledgement_requested": true,
                "provider_prompt_acknowledgement_requested": true
            }),
        ),
        final_ack_fixture(
            "provider-router-activation-result-receipt-final-ack-external-public-install-request",
            "blocked_delivery_noop",
            "external_public_install_restart_active_binary_acknowledgement_denied",
            serde_json::json!({
                "final_operator_acknowledgement_requested": true,
                "external_send_acknowledgement_requested": true,
                "public_claim_acknowledgement_requested": true,
                "release_artifact_acknowledgement_requested": true,
                "install_acknowledgement_requested": true,
                "service_restart_acknowledgement_requested": true,
                "active_binary_acknowledgement_requested": true
            }),
        ),
    ]);
    let final_acknowledgement_fixture_count = final_acknowledgement_fixtures
        .as_array()
        .map(std::vec::Vec::len)
        .unwrap_or(0);
    let fixtures_hash = sha256_json_value(&final_acknowledgement_fixtures);
    let contract_hash = sha256_text_value(&format!(
        "runtime-provider-router-activation-command-result-receipt-final-operator-acknowledgement-non-acceptance-denial:v1:source={source_report_sha256}:fixtures={fixtures_hash}:ack=0:accept=0:record=0:persist=0:deliver=0:promote=0:live=0"
    ));
    let policy_hash = sha256_text_value(
        "runtime-provider-router-activation-command-result-receipt-final-operator-acknowledgement-non-acceptance-denial:v1:no-ack:no-accept:no-record:no-persist:no-materialize:no-deliver:no-final-state:no-provider-model-memory-kg-secret-external-install-restart-binary-public-claim",
    );
    let side_effect_hash = sha256_text_value(
        "runtime-provider-router-final-operator-acknowledgement-side-effects=false;ack=0;accept=0;record=0;persist=0;deliver=0;promotion=0;activation=0;provider=0;model=0;memory=0;kg=0;secret=0;external=0;install=0",
    );
    let mut denials = source_summary
        .get("denied_by_activation_command_result_receipt_operator_facing_summary_briefing")
        .and_then(serde_json::Value::as_array)
        .cloned()
        .unwrap_or_default();
    for denial in [
        "source_operator_facing_summary_briefing_report_required",
        "final_operator_acknowledgement_request_acceptance_denied",
        "final_operator_acknowledgement_acceptance_denied",
        "final_operator_acknowledgement_recording_denied",
        "final_operator_acknowledgement_persistence_denied",
        "final_operator_acknowledgement_materialization_denied",
        "final_operator_acknowledgement_filesystem_write_denied",
        "operator_identity_signature_timestamp_acknowledgement_acceptance_denied",
        "final_operator_acknowledgement_delivery_denied",
        "telegram_send_denied",
        "final_state_completion_promotion_denied",
        "activation_from_final_operator_acknowledgement_denied",
        "memory_kg_acknowledgement_denied",
        "rollback_acknowledgement_denied",
        "secret_material_acknowledgement_denied",
        "provider_prompt_acknowledgement_denied",
        "external_public_install_restart_active_binary_acknowledgement_denied",
    ] {
        denials.push(serde_json::Value::String(denial.to_string()));
    }
    let denied_count = denials.len();

    let mut report = source_summary.clone();
    extend_json_object(
        &mut report,
        serde_json::json!({
            "status": if report_ready { "ready" } else { "blocked" },
            "gate": "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_route",
            "source_command": "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-final-operator-acknowledgement-non-acceptance-denial --json",
            "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_FINAL_OPERATOR_ACKNOWLEDGEMENT_NON_ACCEPTANCE_DENIAL_ENDPOINT,
            "native_route": true,
            "compatibility_mode": "native_runtime_provider_router_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_status",
            "side_effect_free": true,
            "base_url": "native",
            "source_activation_command_result_receipt_operator_facing_summary_briefing_route_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_OPERATOR_FACING_SUMMARY_BRIEFING_NON_PERSISTENCE_DENIAL_ENDPOINT,
            "source_activation_command_result_receipt_operator_facing_summary_briefing_gate": source_str("gate"),
            "source_activation_command_result_receipt_operator_facing_summary_briefing_ready": source_ready,
            "source_activation_command_result_receipt_operator_facing_summary_briefing_status": source_status,
            "source_activation_command_result_receipt_operator_facing_summary_briefing_report_sha256": source_report_sha256,
            "source_operator_summary_briefing_contract_hash_sha256": source_hash_str("operator_summary_briefing_contract_hash_sha256"),
            "source_operator_summary_briefing_policy_hash_sha256": source_hash_str("operator_summary_briefing_policy_hash_sha256"),
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
            "runtime_provider_router_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_route_enabled": true,
            "runtime_provider_router_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready": true,
            "runtime_provider_router_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_status": "blocked",
            "runtime_provider_router_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_no_persistence_ready": source_bool("runtime_provider_router_activation_command_result_receipt_no_persistence_ready"),
            "activation_command_result_receipt_final_operator_acknowledgement_schema_version": "memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_v1",
            "activation_command_result_receipt_final_operator_acknowledgement_mode": "native_route_runtime_provider_router_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_no_ack_no_accept_no_record_no_deliver_no_authority_no_live",
            "activation_command_result_receipt_final_operator_acknowledgement_decision": "runtime_provider_router_activation_command_result_receipt_cannot_accept_record_persist_deliver_or_promote_final_operator_acknowledgement_into_authority",
            "source_operator_facing_summary_briefing_fixture_count": source_u64("operator_facing_summary_briefing_fixture_count"),
            "source_blocked_operator_facing_summary_briefing_fixture_count": source_u64("blocked_operator_facing_summary_briefing_fixture_count"),
            "source_accepted_operator_facing_summary_briefing_fixture_count": source_u64("accepted_operator_facing_summary_briefing_fixture_count"),
            "source_operator_summary_performed_count": source_u64("operator_summary_performed_count"),
            "source_operator_briefing_performed_count": source_u64("operator_briefing_performed_count"),
            "final_acknowledgement_fixtures_sha256": fixtures_hash,
            "final_acknowledgement_contract_hash_sha256": contract_hash,
            "final_acknowledgement_policy_hash_sha256": policy_hash,
            "side_effect_hash_sha256": side_effect_hash,
            "required_activation_command_result_receipt_final_operator_acknowledgement_surface_count": 12,
            "ready_activation_command_result_receipt_final_operator_acknowledgement_surface_count": 12,
            "side_effect_free_activation_command_result_receipt_final_operator_acknowledgement_surface_count": 12,
            "required_activation_command_result_receipt_final_operator_acknowledgement_fixture_count": 10,
            "activation_command_result_receipt_final_operator_acknowledgement_surfaces": [
                "source_operator_facing_summary_briefing_report_required",
                "final_operator_acknowledgement_request_shape_denied",
                "final_operator_acknowledgement_acceptance_denied",
                "final_operator_acknowledgement_recording_denied",
                "final_operator_acknowledgement_persistence_denied",
                "final_operator_acknowledgement_materialization_denied",
                "operator_identity_signature_timestamp_acknowledgement_acceptance_denied",
                "final_operator_acknowledgement_delivery_denied",
                "final_state_completion_promotion_denied",
                "activation_from_final_operator_acknowledgement_denied",
                "memory_kg_rollback_secret_provider_acknowledgement_denied",
                "external_public_install_restart_active_binary_acknowledgement_denied"
            ],
            "activation_command_result_receipt_final_operator_acknowledgement_fixtures": final_acknowledgement_fixtures,
            "activation_command_result_receipt_final_operator_acknowledgement_fixture_count": final_acknowledgement_fixture_count,
            "blocked_activation_command_result_receipt_final_operator_acknowledgement_fixture_count": final_acknowledgement_fixture_count,
            "noop_activation_command_result_receipt_final_operator_acknowledgement_fixture_count": final_acknowledgement_fixture_count,
            "allowed_activation_command_result_receipt_final_operator_acknowledgement_fixture_count": 0,
            "accepted_activation_command_result_receipt_final_operator_acknowledgement_fixture_count": 0,
            "activation_command_result_receipt_final_operator_acknowledgement_denied_count": final_acknowledgement_fixture_count,
            "activation_command_result_receipt_final_operator_acknowledgement_performed_count": 0,
            "activation_command_result_receipt_final_operator_acknowledgement_recorded_count": 0,
            "activation_command_result_receipt_final_operator_acknowledgement_persisted_count": 0,
            "activation_command_result_receipt_final_operator_acknowledgement_delivered_count": 0,
            "activation_command_result_receipt_final_operator_acknowledgement_final_state_promoted_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "live_mutation_execution_ready": false,
            "live_mutation_execution_allowed": false,
            "memory_write_execution_allowed": false,
            "memory_write_execution_ready": false,
            "memory_store_write_path_enabled": false,
            "memory_store_write_allowed": false,
            "memory_store_write_performed_count": 0,
            "memory_store_mutation_allowed": false,
            "rollback_execution_allowed": false,
            "raw_payload_plaintext_recorded": false,
            "raw_payload_plaintext_persisted": false,
            "provider_prompt_replay_enabled": false,
            "external_send_enabled": false,
            "public_claim_or_release_artifact_write_enabled": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "denied_by_activation_command_result_receipt_final_operator_acknowledgement": denials,
            "denied_by_activation_command_result_receipt_final_operator_acknowledgement_count": denied_count,
            "source_operator_facing_summary_briefing_denial_count": source_u64("denied_by_activation_command_result_receipt_operator_facing_summary_briefing_count"),
            "final_operator_acknowledgement_acceptance_forbidden": true,
            "final_operator_acknowledgement_recording_forbidden": true,
            "final_operator_acknowledgement_persistence_forbidden": true,
            "final_operator_acknowledgement_delivery_forbidden": true,
            "final_operator_acknowledgement_promotion_forbidden": true,
            "activation_from_final_operator_acknowledgement_forbidden": true,
            "runtime_provider_memory_kg_final_operator_acknowledgement_forbidden": true,
            "external_public_install_restart_active_binary_final_operator_acknowledgement_forbidden": true,
            "allowed_next_actions": [
                {
                    "action": "review_runtime_provider_router_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial",
                    "status": "allowed_report_only",
                    "accepts_operator_acknowledgement": false,
                    "persists_acknowledgement": false,
                    "delivers_acknowledgement": false,
                    "promotes_final_state": false,
                    "activates_runtime": false,
                    "invokes_model": false,
                    "writes_memory_or_kg": false
                },
                {
                    "action": "stage_runtime_provider_router_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial",
                    "status": "allowed_report_only_next_slice",
                    "accepts_terminal_decision": false,
                    "claims_public_release": false,
                    "writes_release_artifact": false,
                    "activates_runtime": false,
                    "invokes_model": false,
                    "writes_memory_or_kg": false
                },
                {
                    "action": "run_full_light_preflight",
                    "status": "allowed_verification_only",
                    "accepts_operator_acknowledgement": false,
                    "promotes_final_state": false,
                    "mutates_runtime": false,
                    "invokes_model": false,
                    "writes_kg": false
                }
            ],
        }),
    );

    if let Some(report_object) = report.as_object_mut() {
        for key in [
            "activation_command_result_receipt_final_operator_acknowledgement_allowed",
            "activation_command_result_receipt_final_operator_acknowledgement_request_accepted",
            "activation_command_result_receipt_final_operator_acknowledgement_accepted",
            "activation_command_result_receipt_final_operator_acknowledgement_recorded",
            "activation_command_result_receipt_final_operator_acknowledgement_persisted",
            "activation_command_result_receipt_final_operator_acknowledgement_materialized",
            "activation_command_result_receipt_final_operator_acknowledgement_filesystem_written",
            "activation_command_result_receipt_final_operator_acknowledgement_delivered",
            "activation_command_result_receipt_final_operator_acknowledgement_channel_delivery_performed",
            "activation_command_result_receipt_final_operator_acknowledgement_identity_accepted",
            "activation_command_result_receipt_final_operator_acknowledgement_signature_accepted",
            "activation_command_result_receipt_final_operator_acknowledgement_timestamp_accepted",
            "activation_command_result_receipt_final_operator_acknowledgement_final_state_promoted",
            "activation_command_result_receipt_final_operator_acknowledgement_completion_promoted",
            "activation_command_result_receipt_operator_final_acceptance_recorded",
            "activation_command_result_receipt_operator_final_acceptance_persisted",
            "activation_command_result_receipt_operator_final_acceptance_materialized",
            "activation_command_result_receipt_recorded",
            "activation_command_result_receipt_persisted",
            "activation_command_result_receipt_accepted",
            "activation_command_result_receipt_materialized",
            "activation_command_result_receipt_filesystem_written",
            "activation_command_completion_ack_recorded",
            "activation_command_completion_ack_persisted",
            "activation_command_completion_ack_accepted",
            "activation_command_completion_ack_delivered",
            "activation_allowed_by_result_receipt_final_operator_acknowledgement",
            "activation_allowed_by_result_receipt_summary_briefing",
            "activation_allowed_by_result_receipt",
            "activation_allowed",
            "activation_performed",
            "activation_activated",
            "live_mutation_execution_performed",
            "memory_write_execution_performed",
            "memory_store_write_performed",
            "memory_store_mutated",
            "live_kg_write_performed",
            "rollback_executed",
            "auth_secret_read",
            "credential_read",
            "secret_file_read",
            "secret_material_read",
            "provider_invoked",
            "model_invoked",
            "telegram_send_performed",
            "channel_send_performed",
            "external_send_performed",
            "public_release_claimed",
            "public_release_published",
            "public_ga_claimed",
            "release_artifact_written",
            "install_executed",
            "launchd_mutated",
            "service_restart_performed",
            "service_restarted",
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
            "activation_command_result_receipt_final_operator_acknowledgement_recorded",
            "activation_command_result_receipt_final_operator_acknowledgement_persisted",
            "activation_command_result_receipt_final_operator_acknowledgement_materialized",
            "activation_command_result_receipt_final_operator_acknowledgement_filesystem_written",
            "activation_command_result_receipt_final_operator_acknowledgement_delivered",
            "activation_command_result_receipt_final_operator_acknowledgement_channel_delivery_performed",
            "activation_command_result_receipt_final_operator_acknowledgement_identity_accepted",
            "activation_command_result_receipt_final_operator_acknowledgement_signature_accepted",
            "activation_command_result_receipt_final_operator_acknowledgement_timestamp_accepted",
            "activation_command_result_receipt_final_operator_acknowledgement_final_state_promoted",
            "activation_command_result_receipt_final_operator_acknowledgement_completion_promoted",
            "activation_command_result_receipt_operator_final_acceptance_recorded",
            "activation_command_result_receipt_operator_final_acceptance_persisted",
            "activation_command_result_receipt_operator_final_acceptance_materialized",
            "activation_command_result_receipt_operator_summary_recorded",
            "activation_command_result_receipt_operator_summary_persisted",
            "activation_command_result_receipt_operator_summary_materialized",
            "activation_command_result_receipt_operator_summary_filesystem_written",
            "activation_command_result_receipt_operator_summary_delivered",
            "activation_command_result_receipt_operator_briefing_recorded",
            "activation_command_result_receipt_operator_briefing_persisted",
            "activation_command_result_receipt_operator_briefing_materialized",
            "activation_command_result_receipt_operator_briefing_filesystem_written",
            "activation_command_result_receipt_operator_briefing_delivered",
            "activation_command_result_receipt_operator_summary_briefing_channel_delivery_performed",
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
            "telegram_send_performed",
            "channel_send_performed",
            "external_send_performed",
            "activation_performed",
            "live_mutation_execution_performed",
            "memory_write_execution_performed",
            "memory_store_write_performed",
            "memory_store_mutated",
            "live_kg_write_performed",
            "rollback_executed",
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
            "secret_material_read",
            "filesystem_written",
            "public_release_claimed",
            "public_release_published",
            "public_ga_claimed",
            "release_artifact_written",
            "public_artifact_written",
            "install_executed",
            "launchd_mutated",
            "service_restart_performed",
            "service_restarted",
            "active_binary_mutated",
        ] {
            side_effects.insert(key.to_string(), serde_json::Value::Bool(false));
        }
    }
    report
}

fn hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_final_ack =
        hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_report();
    let source_bool = |key: &str| {
        source_final_ack
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let source_u64 = |key: &str| {
        source_final_ack
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_str = |key: &str| {
        source_final_ack
            .get(key)
            .and_then(serde_json::Value::as_str)
            .unwrap_or("")
            .to_string()
    };
    let source_status = source_str(
        "runtime_provider_router_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_status",
    );
    let source_ready = source_status == "blocked"
        && source_bool(
            "runtime_provider_router_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready",
        );
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT;
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let report_ready = source_ready
        && route_count_source_command_accepted
        && route_matrix.missing_route_count == 0
        && source_u64(
            "activation_command_result_receipt_final_operator_acknowledgement_fixture_count",
        ) == 10
        && source_u64(
            "accepted_activation_command_result_receipt_final_operator_acknowledgement_fixture_count",
        ) == 0
        && source_u64(
            "activation_command_result_receipt_final_operator_acknowledgement_performed_count",
        ) == 0
        && !source_bool("activation_allowed_by_result_receipt_final_operator_acknowledgement")
        && !source_bool("provider_invoked")
        && !source_bool("model_invoked")
        && !source_bool("memory_store_write_performed")
        && !source_bool("live_kg_write_performed")
        && !source_bool("telegram_send_performed")
        && !source_bool("external_send_performed")
        && !source_bool("public_release_published")
        && !source_bool("release_artifact_written")
        && !source_bool("install_executed")
        && !source_bool("active_binary_mutated");
    let source_report_sha256 = sha256_json_value(&source_final_ack);

    let terminal_decision_fixture =
        |id: &str, status: &str, reason: &str, extra: serde_json::Value| -> serde_json::Value {
            let mut fixture = serde_json::Map::new();
            fixture.insert("id".to_string(), serde_json::json!(id));
            fixture.insert("fixture_id".to_string(), serde_json::json!(id));
            fixture.insert(
                "terminal_operator_decision_requested".to_string(),
                serde_json::json!(false),
            );
            fixture.insert(
                "terminal_operator_decision_status".to_string(),
                serde_json::json!(status),
            );
            fixture.insert(
                "source_final_acknowledgement_present".to_string(),
                serde_json::json!(true),
            );
            fixture.insert(
                "source_final_acknowledgement_ready".to_string(),
                serde_json::json!(true),
            );
            for key in [
                "terminal_decision_allowed",
                "terminal_decision_request_accepted",
                "terminal_decision_accepted",
                "terminal_decision_recorded",
                "terminal_decision_persisted",
                "terminal_decision_materialized",
                "terminal_decision_filesystem_written",
                "terminal_decision_delivered",
                "terminal_decision_channel_delivery_performed",
                "terminal_decision_identity_accepted",
                "terminal_decision_signature_accepted",
                "terminal_decision_timestamp_accepted",
                "terminal_decision_final_state_promoted",
                "terminal_decision_completion_promoted",
                "public_claim_requested",
                "public_claim_accepted",
                "public_claim_recorded",
                "public_claim_persisted",
                "public_claim_materialized",
                "public_claim_promoted",
                "public_ga_claimed",
                "public_release_published",
                "public_distribution_performed",
                "public_artifact_written",
                "release_artifact_written",
                "telegram_send_performed",
                "channel_send_performed",
                "external_send_performed",
                "receipt_recorded",
                "receipt_persisted",
                "receipt_accepted",
                "receipt_materialized",
                "receipt_filesystem_written",
                "completion_ack_recorded",
                "completion_ack_persisted",
                "completion_ack_accepted",
                "completion_ack_delivered",
                "activation_allowed",
                "activation_performed",
                "live_mutation_execution_performed",
                "memory_write_execution_performed",
                "memory_store_write_performed",
                "memory_store_mutated",
                "live_kg_write_performed",
                "rollback_executed",
                "secret_material_read",
                "auth_secret_read",
                "credential_read",
                "secret_file_read",
                "provider_invoked",
                "model_invoked",
                "provider_prompt_replayed",
                "install_executed",
                "launchd_mutated",
                "service_restarted",
                "service_restart_performed",
                "active_binary_mutated",
            ] {
                fixture.insert(key.to_string(), serde_json::json!(false));
            }
            fixture.insert(
                "terminal_operator_decision_noop_confirmed".to_string(),
                serde_json::json!(true),
            );
            fixture.insert("reason".to_string(), serde_json::json!(reason));
            if let Some(extra_object) = extra.as_object() {
                fixture.extend(extra_object.clone());
            }
            serde_json::Value::Object(fixture)
        };

    let terminal_decision_public_claim_fixtures = serde_json::json!([
        terminal_decision_fixture(
            "provider-router-activation-result-receipt-terminal-decision-missing-final-ack",
            "blocked_noop",
            "source_final_operator_acknowledgement_report_required",
            serde_json::json!({
                "source_final_acknowledgement_present": false,
                "source_final_acknowledgement_ready": false,
                "terminal_operator_decision_requested": true,
            }),
        ),
        terminal_decision_fixture(
            "provider-router-activation-result-receipt-terminal-decision-request",
            "blocked_decision_noop",
            "terminal_operator_decision_request_shape_denied",
            serde_json::json!({"terminal_operator_decision_requested": true}),
        ),
        terminal_decision_fixture(
            "provider-router-activation-result-receipt-terminal-decision-acceptance-request",
            "blocked_acceptance_noop",
            "terminal_operator_decision_acceptance_denied",
            serde_json::json!({
                "terminal_operator_decision_requested": true,
                "terminal_decision_acceptance_requested": true,
            }),
        ),
        terminal_decision_fixture(
            "provider-router-activation-result-receipt-terminal-decision-recording-request",
            "blocked_decision_noop",
            "terminal_operator_decision_recording_denied",
            serde_json::json!({
                "terminal_operator_decision_requested": true,
                "terminal_decision_recording_requested": true,
            }),
        ),
        terminal_decision_fixture(
            "provider-router-activation-result-receipt-terminal-decision-persistence-filesystem-write-request",
            "blocked_decision_noop",
            "terminal_operator_decision_persistence_filesystem_write_denied",
            serde_json::json!({
                "terminal_operator_decision_requested": true,
                "terminal_decision_persistence_requested": true,
                "terminal_decision_filesystem_write_requested": true,
            }),
        ),
        terminal_decision_fixture(
            "provider-router-activation-result-receipt-terminal-decision-identity-signature-request",
            "blocked_acceptance_noop",
            "operator_identity_signature_terminal_decision_acceptance_denied",
            serde_json::json!({
                "terminal_operator_decision_requested": true,
                "operator_identity_acceptance_requested": true,
                "operator_signature_acceptance_requested": true,
                "operator_timestamp_acceptance_requested": true,
            }),
        ),
        terminal_decision_fixture(
            "provider-router-activation-result-receipt-terminal-decision-public-claim-request",
            "blocked_public_claim_noop",
            "public_claim_request_non_promotion_denied",
            serde_json::json!({
                "terminal_operator_decision_requested": true,
                "public_claim_requested": true,
                "public_claim_promotion_requested": true,
            }),
        ),
        terminal_decision_fixture(
            "provider-router-activation-result-receipt-terminal-decision-public-ga-release-request",
            "blocked_promotion_noop",
            "public_ga_release_publication_promotion_denied",
            serde_json::json!({
                "terminal_operator_decision_requested": true,
                "public_ga_claim_requested": true,
                "public_release_publish_requested": true,
                "public_distribution_requested": true,
                "release_artifact_write_requested": true,
            }),
        ),
        terminal_decision_fixture(
            "provider-router-activation-result-receipt-terminal-decision-activation-memory-provider-request",
            "blocked_decision_noop",
            "activation_memory_rollback_secret_provider_terminal_decision_denied",
            serde_json::json!({
                "terminal_operator_decision_requested": true,
                "activation_from_terminal_decision_requested": true,
                "memory_write_terminal_decision_requested": true,
                "rollback_terminal_decision_requested": true,
                "secret_material_terminal_decision_requested": true,
                "provider_prompt_terminal_decision_requested": true,
            }),
        ),
        terminal_decision_fixture(
            "provider-router-activation-result-receipt-terminal-decision-external-public-install-request",
            "blocked_promotion_noop",
            "external_public_install_restart_active_binary_terminal_decision_denied",
            serde_json::json!({
                "terminal_operator_decision_requested": true,
                "external_send_decision_requested": true,
                "public_claim_decision_requested": true,
                "release_artifact_decision_requested": true,
                "install_decision_requested": true,
                "service_restart_decision_requested": true,
                "active_binary_decision_requested": true,
            }),
        ),
    ]);
    let terminal_decision_public_claim_fixture_count = terminal_decision_public_claim_fixtures
        .as_array()
        .map(std::vec::Vec::len)
        .unwrap_or(0);
    let fixtures_sha256 = sha256_json_value(&terminal_decision_public_claim_fixtures);
    let contract_hash_sha256 = sha256_text_value(&format!(
        "runtime-provider-router-activation-command-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial:v1:source={source_report_sha256}:fixtures={fixtures_sha256}:decision=0:public_claim=0:publish=0:artifact=0:live=0"
    ));
    let policy_hash_sha256 = sha256_text_value(
        "runtime-provider-router-activation-command-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial:v1:no-terminal-decision-accept:no-public-claim:no-ga-release:no-artifact:no-live",
    );
    let side_effect_hash_sha256 = sha256_text_value(
        "runtime-provider-router-terminal-decision=false;public_claim=false;public_release=false;artifact=false;activation=false;provider=false;model=false;memory=false;kg=false;secret=false;install=false;restart=false;active_binary=false",
    );
    let mut denials = source_final_ack
        .get("denied_by_activation_command_result_receipt_final_operator_acknowledgement")
        .and_then(serde_json::Value::as_array)
        .cloned()
        .unwrap_or_default();
    for denial in [
        "source_final_operator_acknowledgement_report_required",
        "terminal_operator_decision_request_acceptance_denied",
        "terminal_operator_decision_acceptance_denied",
        "terminal_operator_decision_recording_denied",
        "terminal_operator_decision_persistence_denied",
        "terminal_operator_decision_materialization_denied",
        "terminal_operator_decision_filesystem_write_denied",
        "operator_identity_signature_terminal_decision_acceptance_denied",
        "terminal_operator_decision_delivery_denied",
        "telegram_send_denied",
        "public_claim_non_promotion_denied",
        "public_ga_release_publication_promotion_denied",
        "activation_from_terminal_operator_decision_denied",
        "memory_write_terminal_decision_denied",
        "rollback_terminal_decision_denied",
        "secret_material_terminal_decision_denied",
        "provider_prompt_terminal_decision_denied",
        "external_public_install_restart_active_binary_terminal_decision_denied",
    ] {
        denials.push(serde_json::Value::String(denial.to_string()));
    }
    let denied_count = denials.len();

    let mut report = source_final_ack.clone();
    extend_json_object(
        &mut report,
        serde_json::json!({
            "status": if report_ready { "ready" } else { "blocked" },
            "gate": "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_route",
            "source_command": "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial --json",
            "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_TERMINAL_OPERATOR_DECISION_PUBLIC_CLAIM_NON_PROMOTION_DENIAL_ENDPOINT,
            "native_route": true,
            "compatibility_mode": "native_runtime_provider_router_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_status",
            "side_effect_free": true,
            "base_url": "native",
            "source_activation_command_result_receipt_final_operator_acknowledgement_route_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_FINAL_OPERATOR_ACKNOWLEDGEMENT_NON_ACCEPTANCE_DENIAL_ENDPOINT,
            "source_activation_command_result_receipt_final_operator_acknowledgement_gate": source_str("gate"),
            "source_activation_command_result_receipt_final_operator_acknowledgement_ready": source_ready,
            "source_activation_command_result_receipt_final_operator_acknowledgement_status": source_status,
            "source_activation_command_result_receipt_final_operator_acknowledgement_report_sha256": source_report_sha256,
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
            "runtime_provider_router_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_route_enabled": true,
            "runtime_provider_router_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_ready": true,
            "runtime_provider_router_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_status": "blocked",
            "runtime_provider_router_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_no_persistence_ready": source_bool("runtime_provider_router_activation_command_result_receipt_no_persistence_ready"),
            "activation_command_result_receipt_terminal_operator_decision_public_claim_schema_version": "memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_v1",
            "activation_command_result_receipt_terminal_operator_decision_public_claim_mode": "native_route_runtime_provider_router_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_no_decision_no_public_claim_no_release_no_artifact_no_authority_no_live",
            "activation_command_result_receipt_terminal_operator_decision_public_claim_decision": "runtime_provider_router_activation_command_result_receipt_cannot_promote_final_acknowledgement_or_receipt_into_terminal_operator_decision_or_public_claim_authority",
            "source_final_operator_acknowledgement_fixture_count": source_u64("activation_command_result_receipt_final_operator_acknowledgement_fixture_count"),
            "source_blocked_final_operator_acknowledgement_fixture_count": source_u64("blocked_activation_command_result_receipt_final_operator_acknowledgement_fixture_count"),
            "source_accepted_final_operator_acknowledgement_fixture_count": source_u64("accepted_activation_command_result_receipt_final_operator_acknowledgement_fixture_count"),
            "source_final_operator_acknowledgement_performed_count": source_u64("activation_command_result_receipt_final_operator_acknowledgement_performed_count"),
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "terminal_operator_decision_public_claim_fixtures_sha256": fixtures_sha256,
            "terminal_operator_decision_public_claim_contract_hash_sha256": contract_hash_sha256,
            "terminal_operator_decision_public_claim_policy_hash_sha256": policy_hash_sha256,
            "side_effect_hash_sha256": side_effect_hash_sha256,
            "required_activation_command_result_receipt_terminal_operator_decision_public_claim_surface_count": 12,
            "ready_activation_command_result_receipt_terminal_operator_decision_public_claim_surface_count": 12,
            "side_effect_free_activation_command_result_receipt_terminal_operator_decision_public_claim_surface_count": 12,
            "required_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count": 10,
            "activation_command_result_receipt_terminal_operator_decision_public_claim_fixtures": terminal_decision_public_claim_fixtures,
            "activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count": terminal_decision_public_claim_fixture_count,
            "blocked_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count": terminal_decision_public_claim_fixture_count,
            "noop_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count": terminal_decision_public_claim_fixture_count,
            "allowed_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count": 0,
            "accepted_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count": 0,
            "activation_command_result_receipt_terminal_operator_decision_performed_count": 0,
            "activation_command_result_receipt_public_claim_promotion_performed_count": 0,
            "activation_command_result_receipt_terminal_operator_decision_public_claim_surfaces": [
                "source_final_operator_acknowledgement_report_required",
                "terminal_operator_decision_request_shape_denied",
                "terminal_operator_decision_acceptance_denied",
                "terminal_operator_decision_recording_denied",
                "terminal_operator_decision_persistence_denied",
                "terminal_operator_decision_materialization_denied",
                "operator_identity_signature_terminal_decision_acceptance_denied",
                "terminal_operator_decision_delivery_denied",
                "public_claim_request_non_promotion_denied",
                "public_ga_release_publication_promotion_denied",
                "activation_from_terminal_operator_decision_denied",
                "external_public_install_restart_active_binary_terminal_decision_denied"
            ],
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "activation_command_result_receipt_terminal_operator_decision_allowed": false,
            "activation_command_result_receipt_terminal_operator_decision_request_accepted": false,
            "activation_command_result_receipt_terminal_operator_decision_accepted": false,
            "activation_command_result_receipt_terminal_operator_decision_recorded": false,
            "activation_command_result_receipt_terminal_operator_decision_persisted": false,
            "activation_command_result_receipt_terminal_operator_decision_materialized": false,
            "activation_command_result_receipt_terminal_operator_decision_filesystem_written": false,
            "activation_command_result_receipt_terminal_operator_decision_delivered": false,
            "activation_command_result_receipt_terminal_operator_decision_channel_delivery_performed": false,
            "activation_command_result_receipt_terminal_operator_decision_identity_accepted": false,
            "activation_command_result_receipt_terminal_operator_decision_signature_accepted": false,
            "activation_command_result_receipt_terminal_operator_decision_timestamp_accepted": false,
            "activation_command_result_receipt_terminal_operator_decision_final_state_promoted": false,
            "activation_command_result_receipt_terminal_operator_decision_completion_promoted": false,
            "activation_command_result_receipt_public_claim_requested": false,
            "activation_command_result_receipt_public_claim_accepted": false,
            "activation_command_result_receipt_public_claim_recorded": false,
            "activation_command_result_receipt_public_claim_persisted": false,
            "activation_command_result_receipt_public_claim_materialized": false,
            "activation_command_result_receipt_public_claim_promoted": false,
            "activation_command_result_receipt_public_ga_claimed": false,
            "activation_command_result_receipt_public_release_published": false,
            "activation_command_result_receipt_public_distribution_performed": false,
            "activation_command_result_receipt_public_artifact_written": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "activation_allowed_by_result_receipt_terminal_operator_decision": false,
            "activation_allowed_by_result_receipt_final_operator_acknowledgement": false,
            "activation_allowed_by_result_receipt": false,
            "activation_allowed": false,
            "activation_performed": false,
            "live_mutation_execution_ready": false,
            "live_mutation_execution_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_write_execution_allowed": false,
            "memory_write_execution_ready": false,
            "memory_write_execution_performed": false,
            "memory_store_write_path_enabled": false,
            "memory_store_write_allowed": false,
            "memory_store_write_performed": false,
            "memory_store_write_performed_count": 0,
            "memory_store_mutation_allowed": false,
            "memory_store_mutated": false,
            "rollback_execution_allowed": false,
            "rollback_executed": false,
            "secret_material_read": false,
            "provider_prompt_replay_enabled": false,
            "provider_invoked": false,
            "model_invoked": false,
            "public_release_published": false,
            "public_ga_claimed": false,
            "release_artifact_written": false,
            "public_artifact_written": false,
            "public_distribution_performed": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restarted": false,
            "service_restart_performed": false,
            "active_binary_mutated": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "denied_by_activation_command_result_receipt_terminal_operator_decision_public_claim": denials,
            "denied_by_activation_command_result_receipt_terminal_operator_decision_public_claim_count": denied_count,
            "source_final_operator_acknowledgement_denial_count": source_u64("denied_by_activation_command_result_receipt_final_operator_acknowledgement_count"),
            "terminal_operator_decision_acceptance_forbidden": true,
            "terminal_operator_decision_recording_forbidden": true,
            "terminal_operator_decision_persistence_forbidden": true,
            "terminal_operator_decision_delivery_forbidden": true,
            "terminal_operator_decision_promotion_forbidden": true,
            "public_claim_promotion_forbidden": true,
            "public_release_publication_forbidden": true,
            "release_artifact_publication_forbidden": true,
            "activation_from_terminal_operator_decision_forbidden": true,
            "runtime_provider_memory_kg_terminal_operator_decision_forbidden": true,
            "external_public_install_restart_active_binary_terminal_operator_decision_forbidden": true,
            "allowed_next_actions": [
                {
                    "action": "review_runtime_provider_router_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial",
                    "status": "allowed_report_only",
                    "accepts_terminal_decision": false,
                    "claims_public_release": false,
                    "writes_release_artifact": false,
                    "activates_runtime": false,
                    "invokes_model": false,
                    "writes_memory_or_kg": false
                },
                {
                    "action": "stage_runtime_provider_router_activation_command_result_receipt_release_artifact_publication_denial",
                    "status": "allowed_report_only_next_slice",
                    "publishes_release_artifact": false,
                    "claims_public_release": false,
                    "writes_release_artifact": false,
                    "activates_runtime": false,
                    "invokes_model": false,
                    "writes_memory_or_kg": false
                },
                {
                    "action": "run_full_light_preflight",
                    "status": "allowed_verification_only",
                    "accepts_terminal_decision": false,
                    "claims_public_release": false,
                    "mutates_runtime": false,
                    "invokes_model": false,
                    "writes_kg": false
                }
            ],
        }),
    );

    if let Some(report_object) = report.as_object_mut() {
        for key in [
            "activation_command_result_receipt_terminal_operator_decision_allowed",
            "activation_command_result_receipt_terminal_operator_decision_request_accepted",
            "activation_command_result_receipt_terminal_operator_decision_accepted",
            "activation_command_result_receipt_terminal_operator_decision_recorded",
            "activation_command_result_receipt_terminal_operator_decision_persisted",
            "activation_command_result_receipt_terminal_operator_decision_materialized",
            "activation_command_result_receipt_terminal_operator_decision_filesystem_written",
            "activation_command_result_receipt_terminal_operator_decision_delivered",
            "activation_command_result_receipt_terminal_operator_decision_channel_delivery_performed",
            "activation_command_result_receipt_terminal_operator_decision_identity_accepted",
            "activation_command_result_receipt_terminal_operator_decision_signature_accepted",
            "activation_command_result_receipt_terminal_operator_decision_timestamp_accepted",
            "activation_command_result_receipt_terminal_operator_decision_final_state_promoted",
            "activation_command_result_receipt_terminal_operator_decision_completion_promoted",
            "activation_command_result_receipt_public_claim_requested",
            "activation_command_result_receipt_public_claim_accepted",
            "activation_command_result_receipt_public_claim_recorded",
            "activation_command_result_receipt_public_claim_persisted",
            "activation_command_result_receipt_public_claim_materialized",
            "activation_command_result_receipt_public_claim_promoted",
            "activation_command_result_receipt_public_ga_claimed",
            "activation_command_result_receipt_public_release_published",
            "activation_command_result_receipt_public_distribution_performed",
            "activation_command_result_receipt_public_artifact_written",
            "activation_command_result_receipt_recorded",
            "activation_command_result_receipt_persisted",
            "activation_command_result_receipt_accepted",
            "activation_command_result_receipt_materialized",
            "activation_command_result_receipt_filesystem_written",
            "activation_command_completion_ack_recorded",
            "activation_command_completion_ack_persisted",
            "activation_command_completion_ack_accepted",
            "activation_command_completion_ack_delivered",
            "activation_allowed_by_result_receipt_terminal_operator_decision",
            "activation_allowed_by_result_receipt_final_operator_acknowledgement",
            "activation_allowed_by_result_receipt",
            "activation_allowed",
            "activation_performed",
            "activation_activated",
            "live_mutation_execution_performed",
            "memory_write_execution_performed",
            "memory_store_write_performed",
            "memory_store_mutated",
            "live_kg_write_performed",
            "rollback_executed",
            "auth_secret_read",
            "credential_read",
            "secret_file_read",
            "secret_material_read",
            "provider_invoked",
            "model_invoked",
            "telegram_send_performed",
            "channel_send_performed",
            "external_send_performed",
            "public_release_claimed",
            "public_release_published",
            "public_ga_claimed",
            "release_artifact_written",
            "public_artifact_written",
            "public_distribution_performed",
            "install_executed",
            "launchd_mutated",
            "service_restart_performed",
            "service_restarted",
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
            "activation_command_result_receipt_terminal_operator_decision_recorded",
            "activation_command_result_receipt_terminal_operator_decision_persisted",
            "activation_command_result_receipt_terminal_operator_decision_materialized",
            "activation_command_result_receipt_terminal_operator_decision_filesystem_written",
            "activation_command_result_receipt_terminal_operator_decision_delivered",
            "activation_command_result_receipt_terminal_operator_decision_channel_delivery_performed",
            "activation_command_result_receipt_terminal_operator_decision_identity_accepted",
            "activation_command_result_receipt_terminal_operator_decision_signature_accepted",
            "activation_command_result_receipt_terminal_operator_decision_timestamp_accepted",
            "activation_command_result_receipt_terminal_operator_decision_final_state_promoted",
            "activation_command_result_receipt_terminal_operator_decision_completion_promoted",
            "activation_command_result_receipt_public_claim_recorded",
            "activation_command_result_receipt_public_claim_persisted",
            "activation_command_result_receipt_public_claim_materialized",
            "activation_command_result_receipt_public_claim_promoted",
            "activation_command_result_receipt_public_ga_claimed",
            "activation_command_result_receipt_public_release_published",
            "activation_command_result_receipt_public_distribution_performed",
            "activation_command_result_receipt_public_artifact_written",
            "telegram_send_performed",
            "channel_send_performed",
            "external_send_performed",
            "activation_command_result_receipt_final_operator_acknowledgement_recorded",
            "activation_command_result_receipt_final_operator_acknowledgement_persisted",
            "activation_command_result_receipt_final_operator_acknowledgement_materialized",
            "activation_command_result_receipt_final_operator_acknowledgement_filesystem_written",
            "activation_command_result_receipt_final_operator_acknowledgement_delivered",
            "activation_command_result_receipt_operator_final_acceptance_recorded",
            "activation_command_result_receipt_operator_final_acceptance_persisted",
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
            "activation_performed",
            "live_mutation_execution_performed",
            "memory_write_execution_performed",
            "memory_store_write_performed",
            "memory_store_mutated",
            "live_kg_write_performed",
            "rollback_executed",
            "raw_payload_plaintext_recorded",
            "raw_payload_plaintext_persisted",
            "secret_material_read",
            "auth_secret_read",
            "credential_read",
            "secret_file_read",
            "provider_prompt_replay_enabled",
            "provider_invoked",
            "model_invoked",
            "provider_prompt_replayed",
            "runtime_router_mutated",
            "runtime_attachment_performed",
            "live_context_attached",
            "context_injection_performed",
            "adapter_invoked",
            "filesystem_written",
            "public_release_published",
            "public_release_claimed",
            "public_ga_claimed",
            "release_artifact_written",
            "public_artifact_written",
            "public_distribution_performed",
            "install_executed",
            "launchd_mutated",
            "service_restarted",
            "service_restart_performed",
            "active_binary_mutated",
        ] {
            side_effects.insert(key.to_string(), serde_json::json!(false));
        }
    }
    report
}

fn hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_release_artifact_publication_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_terminal =
        hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_report();
    let source_bool = |key: &str| {
        source_terminal
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let source_u64 = |key: &str| {
        source_terminal
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_str = |key: &str| {
        source_terminal
            .get(key)
            .and_then(serde_json::Value::as_str)
            .unwrap_or("")
            .to_string()
    };
    let source_status = source_str(
        "runtime_provider_router_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_status",
    );
    let source_ready = source_status == "blocked"
        && source_bool(
            "runtime_provider_router_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_ready",
        );
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT;
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let report_ready = source_ready
        && route_count_source_command_accepted
        && route_matrix.missing_route_count == 0
        && source_u64(
            "activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count",
        ) == 10
        && source_u64(
            "accepted_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count",
        ) == 0
        && source_u64(
            "activation_command_result_receipt_terminal_operator_decision_performed_count",
        ) == 0
        && source_u64("activation_command_result_receipt_public_claim_promotion_performed_count")
            == 0
        && !source_bool("activation_command_result_receipt_public_claim_promoted")
        && !source_bool("public_release_published")
        && !source_bool("release_artifact_written")
        && !source_bool("public_artifact_written")
        && !source_bool("public_distribution_performed")
        && !source_bool("activation_allowed")
        && !source_bool("provider_invoked")
        && !source_bool("model_invoked")
        && !source_bool("memory_store_write_performed")
        && !source_bool("live_kg_write_performed")
        && !source_bool("telegram_send_performed")
        && !source_bool("external_send_performed")
        && !source_bool("install_executed")
        && !source_bool("active_binary_mutated");
    let source_report_sha256 = sha256_json_value(&source_terminal);

    let release_publication_fixture =
        |id: &str, status: &str, reason: &str, extra: serde_json::Value| -> serde_json::Value {
            let mut fixture = serde_json::Map::new();
            fixture.insert("id".to_string(), serde_json::json!(id));
            fixture.insert("fixture_id".to_string(), serde_json::json!(id));
            fixture.insert(
                "release_artifact_publication_status".to_string(),
                serde_json::json!(status),
            );
            fixture.insert(
                "source_terminal_operator_decision_present".to_string(),
                serde_json::json!(true),
            );
            fixture.insert(
                "source_terminal_operator_decision_ready".to_string(),
                serde_json::json!(true),
            );
            for key in [
                "release_artifact_publication_requested",
                "release_artifact_publication_allowed",
                "release_artifact_publication_accepted",
                "release_artifact_publication_recorded",
                "release_artifact_publication_persisted",
                "release_artifact_publication_materialized",
                "release_artifact_filesystem_written",
                "release_artifact_written",
                "public_artifact_written",
                "artifact_signature_accepted",
                "artifact_notarization_accepted",
                "publication_queue_enqueued",
                "publication_manifest_written",
                "public_distribution_performed",
                "public_release_published",
                "public_ga_claimed",
                "public_claim_promoted",
                "public_version_tag_created",
                "release_notes_materialized",
                "changelog_materialized",
                "terminal_operator_decision_promoted_to_release_approval",
                "telegram_send_performed",
                "channel_send_performed",
                "external_send_performed",
                "receipt_recorded",
                "receipt_persisted",
                "receipt_accepted",
                "receipt_materialized",
                "completion_ack_recorded",
                "activation_allowed",
                "activation_performed",
                "live_mutation_execution_performed",
                "memory_write_execution_performed",
                "memory_store_write_performed",
                "memory_store_mutated",
                "live_kg_write_performed",
                "rollback_executed",
                "secret_material_read",
                "auth_secret_read",
                "credential_read",
                "secret_file_read",
                "provider_invoked",
                "model_invoked",
                "provider_prompt_replayed",
                "install_executed",
                "launchd_mutated",
                "service_restarted",
                "service_restart_performed",
                "active_binary_mutated",
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(false));
            }
            fixture.insert(
                "release_artifact_publication_noop_confirmed".to_string(),
                serde_json::json!(true),
            );
            fixture.insert("reason".to_string(), serde_json::json!(reason));
            if let Some(extra_object) = extra.as_object() {
                fixture.extend(extra_object.clone());
            }
            serde_json::Value::Object(fixture)
        };

    let release_publication_fixtures = serde_json::json!([
        release_publication_fixture(
            "provider-router-activation-result-receipt-release-artifact-publication-missing-terminal-decision",
            "blocked_noop",
            "source_terminal_operator_decision_report_required",
            serde_json::json!({
                "source_terminal_operator_decision_present": false,
                "source_terminal_operator_decision_ready": false,
                "release_artifact_publication_requested": true,
            }),
        ),
        release_publication_fixture(
            "provider-router-activation-result-receipt-release-artifact-write-request",
            "blocked_artifact_noop",
            "release_artifact_write_denied",
            serde_json::json!({
                "release_artifact_write_requested": true,
                "release_artifact_publication_requested": true,
            }),
        ),
        release_publication_fixture(
            "provider-router-activation-result-receipt-public-artifact-write-request",
            "blocked_artifact_noop",
            "public_artifact_write_denied",
            serde_json::json!({
                "public_artifact_write_requested": true,
                "release_artifact_publication_requested": true,
            }),
        ),
        release_publication_fixture(
            "provider-router-activation-result-receipt-artifact-signature-notarization-request",
            "blocked_artifact_noop",
            "artifact_signature_notarization_acceptance_denied",
            serde_json::json!({
                "artifact_signature_requested": true,
                "artifact_notarization_requested": true,
                "release_artifact_publication_requested": true,
            }),
        ),
        release_publication_fixture(
            "provider-router-activation-result-receipt-publication-queue-request",
            "blocked_publication_noop",
            "publication_queue_enqueue_denied",
            serde_json::json!({
                "publication_queue_enqueue_requested": true,
                "publication_manifest_write_requested": true,
            }),
        ),
        release_publication_fixture(
            "provider-router-activation-result-receipt-distribution-channel-request",
            "blocked_distribution_noop",
            "public_distribution_channel_delivery_denied",
            serde_json::json!({
                "public_distribution_requested": true,
                "telegram_delivery_requested": true,
                "channel_delivery_requested": true,
                "external_delivery_requested": true,
            }),
        ),
        release_publication_fixture(
            "provider-router-activation-result-receipt-public-version-tag-request",
            "blocked_release_noop",
            "public_version_tag_release_promotion_denied",
            serde_json::json!({
                "public_version_tag_requested": true,
                "public_release_publish_requested": true,
                "public_ga_claim_requested": true,
            }),
        ),
        release_publication_fixture(
            "provider-router-activation-result-receipt-release-notes-changelog-request",
            "blocked_artifact_noop",
            "release_notes_changelog_materialization_denied",
            serde_json::json!({
                "release_notes_materialization_requested": true,
                "changelog_materialization_requested": true,
            }),
        ),
        release_publication_fixture(
            "provider-router-activation-result-receipt-terminal-decision-as-release-approval",
            "blocked_promotion_noop",
            "terminal_operator_decision_is_not_release_approval",
            serde_json::json!({
                "terminal_operator_decision_release_approval_requested": true,
                "release_artifact_publication_requested": true,
            }),
        ),
        release_publication_fixture(
            "provider-router-activation-result-receipt-release-publication-activation-memory-provider-install",
            "blocked_promotion_noop",
            "activation_memory_provider_install_restart_active_binary_publication_denied",
            serde_json::json!({
                "activation_from_release_publication_requested": true,
                "memory_write_publication_requested": true,
                "provider_prompt_publication_requested": true,
                "install_publication_requested": true,
                "service_restart_publication_requested": true,
                "active_binary_publication_requested": true,
            }),
        ),
    ]);
    let release_publication_fixture_count = release_publication_fixtures
        .as_array()
        .map(std::vec::Vec::len)
        .unwrap_or(0);
    let fixtures_sha256 = sha256_json_value(&release_publication_fixtures);
    let contract_hash_sha256 = sha256_text_value(&format!(
        "runtime-provider-router-activation-command-result-receipt-release-artifact-publication-denial:v1:source={source_report_sha256}:fixtures={fixtures_sha256}:publication=0:artifact=0:claim=0:distribution=0:install=0"
    ));
    let policy_hash_sha256 = sha256_text_value(
        "runtime-provider-router-activation-command-result-receipt-release-artifact-publication-denial:v1:no-artifact-write:no-public-artifact:no-signing:no-notarization:no-publication:no-release-claim:no-distribution",
    );
    let side_effect_hash_sha256 = sha256_text_value(
        "runtime-provider-router-release-artifact-publication=false;artifact=false;signature=false;notarization=false;queue=false;manifest=false;public_release=false;distribution=false;activation=false;provider=false;model=false;memory=false;kg=false;secret=false;install=false;restart=false;active_binary=false",
    );
    let mut denials = source_terminal
        .get("denied_by_activation_command_result_receipt_terminal_operator_decision_public_claim")
        .and_then(serde_json::Value::as_array)
        .cloned()
        .unwrap_or_default();
    for denial in [
        "source_terminal_operator_decision_report_required",
        "release_artifact_write_denied",
        "public_artifact_write_denied",
        "artifact_signature_notarization_acceptance_denied",
        "publication_queue_enqueue_denied",
        "publication_manifest_write_denied",
        "public_distribution_channel_delivery_denied",
        "public_version_tag_release_promotion_denied",
        "release_notes_changelog_materialization_denied",
        "terminal_operator_decision_is_not_release_approval",
        "activation_from_release_artifact_publication_denied",
        "memory_write_publication_denied",
        "provider_prompt_publication_denied",
        "install_restart_active_binary_publication_denied",
    ] {
        denials.push(serde_json::Value::String(denial.to_string()));
    }
    let denied_count = denials.len();

    let mut report = source_terminal.clone();
    extend_json_object(
        &mut report,
        serde_json::json!({
            "status": if report_ready { "ready" } else { "blocked" },
            "gate": "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_release_artifact_publication_denial_route",
            "source_command": "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-release-artifact-publication-denial --json",
            "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_RELEASE_ARTIFACT_PUBLICATION_DENIAL_ENDPOINT,
            "native_route": true,
            "compatibility_mode": "native_runtime_provider_router_activation_command_result_receipt_release_artifact_publication_denial_status",
            "side_effect_free": true,
            "base_url": "native",
            "source_activation_command_result_receipt_terminal_operator_decision_public_claim_route_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_TERMINAL_OPERATOR_DECISION_PUBLIC_CLAIM_NON_PROMOTION_DENIAL_ENDPOINT,
            "source_activation_command_result_receipt_terminal_operator_decision_public_claim_gate": source_str("gate"),
            "source_activation_command_result_receipt_terminal_operator_decision_public_claim_ready": source_ready,
            "source_activation_command_result_receipt_terminal_operator_decision_public_claim_status": source_status,
            "source_activation_command_result_receipt_terminal_operator_decision_public_claim_report_sha256": source_report_sha256,
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
            "runtime_provider_router_activation_command_result_receipt_release_artifact_publication_denial_route_enabled": true,
            "runtime_provider_router_activation_command_result_receipt_release_artifact_publication_denial_ready": true,
            "runtime_provider_router_activation_command_result_receipt_release_artifact_publication_denial_status": "blocked",
            "runtime_provider_router_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready": source_bool("runtime_provider_router_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready"),
            "runtime_provider_router_activation_command_result_receipt_no_persistence_ready": source_bool("runtime_provider_router_activation_command_result_receipt_no_persistence_ready"),
            "activation_command_result_receipt_release_artifact_publication_schema_version": "memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_release_artifact_publication_denial_v1",
            "activation_command_result_receipt_release_artifact_publication_mode": "native_route_runtime_provider_router_activation_command_result_receipt_release_artifact_publication_denial_no_artifact_no_publication_no_claim_no_distribution_no_authority_no_live",
            "activation_command_result_receipt_release_artifact_publication_decision": "runtime_provider_router_activation_command_result_receipt_cannot_promote_terminal_operator_decision_or_public_claim_denial_into_release_artifact_publication_authority",
            "source_terminal_operator_decision_public_claim_fixture_count": source_u64("activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count"),
            "source_accepted_terminal_operator_decision_public_claim_fixture_count": source_u64("accepted_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count"),
            "source_terminal_operator_decision_performed_count": source_u64("activation_command_result_receipt_terminal_operator_decision_performed_count"),
            "source_public_claim_promotion_performed_count": source_u64("activation_command_result_receipt_public_claim_promotion_performed_count"),
            "release_artifact_publication_fixtures_sha256": fixtures_sha256,
            "release_artifact_publication_contract_hash_sha256": contract_hash_sha256,
            "release_artifact_publication_policy_hash_sha256": policy_hash_sha256,
            "side_effect_hash_sha256": side_effect_hash_sha256,
            "required_activation_command_result_receipt_release_artifact_publication_surface_count": 12,
            "ready_activation_command_result_receipt_release_artifact_publication_surface_count": 12,
            "side_effect_free_activation_command_result_receipt_release_artifact_publication_surface_count": 12,
            "required_activation_command_result_receipt_release_artifact_publication_fixture_count": 10,
            "activation_command_result_receipt_release_artifact_publication_fixtures": release_publication_fixtures,
            "activation_command_result_receipt_release_artifact_publication_fixture_count": release_publication_fixture_count,
            "blocked_activation_command_result_receipt_release_artifact_publication_fixture_count": release_publication_fixture_count,
            "noop_activation_command_result_receipt_release_artifact_publication_fixture_count": release_publication_fixture_count,
            "allowed_activation_command_result_receipt_release_artifact_publication_fixture_count": 0,
            "accepted_activation_command_result_receipt_release_artifact_publication_fixture_count": 0,
            "activation_command_result_receipt_release_artifact_publication_performed_count": 0,
            "release_artifact_written_count": 0,
            "public_artifact_written_count": 0,
            "public_distribution_performed_count": 0,
            "publication_manifest_written_count": 0,
            "publication_queue_enqueued_count": 0,
            "activation_command_result_receipt_release_artifact_publication_surfaces": [
                "source_terminal_operator_decision_report_required",
                "release_artifact_write_denied",
                "public_artifact_write_denied",
                "artifact_signature_notarization_acceptance_denied",
                "publication_queue_enqueue_denied",
                "publication_manifest_write_denied",
                "public_distribution_channel_delivery_denied",
                "public_version_tag_release_promotion_denied",
                "release_notes_changelog_materialization_denied",
                "terminal_operator_decision_is_not_release_approval",
                "activation_from_release_artifact_publication_denied",
                "external_public_install_restart_active_binary_publication_denied"
            ],
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "activation_command_result_receipt_release_artifact_publication_allowed": false,
            "activation_command_result_receipt_release_artifact_publication_requested": false,
            "activation_command_result_receipt_release_artifact_publication_accepted": false,
            "activation_command_result_receipt_release_artifact_publication_recorded": false,
            "activation_command_result_receipt_release_artifact_publication_persisted": false,
            "activation_command_result_receipt_release_artifact_publication_materialized": false,
            "release_artifact_publication_allowed": false,
            "release_artifact_publication_requested": false,
            "release_artifact_publication_accepted": false,
            "release_artifact_publication_recorded": false,
            "release_artifact_publication_persisted": false,
            "release_artifact_publication_materialized": false,
            "release_artifact_filesystem_written": false,
            "release_artifact_written": false,
            "public_artifact_written": false,
            "artifact_signature_accepted": false,
            "artifact_notarization_accepted": false,
            "publication_queue_enqueued": false,
            "publication_manifest_written": false,
            "public_distribution_performed": false,
            "public_release_published": false,
            "public_ga_claimed": false,
            "public_claim_promoted": false,
            "public_version_tag_created": false,
            "release_notes_materialized": false,
            "changelog_materialized": false,
            "terminal_operator_decision_promoted_to_release_approval": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "activation_allowed_by_release_artifact_publication": false,
            "activation_allowed_by_terminal_operator_decision": false,
            "activation_allowed_by_result_receipt": false,
            "activation_allowed": false,
            "activation_performed": false,
            "live_mutation_execution_ready": false,
            "live_mutation_execution_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_write_execution_allowed": false,
            "memory_write_execution_ready": false,
            "memory_write_execution_performed": false,
            "memory_store_write_path_enabled": false,
            "memory_store_write_allowed": false,
            "memory_store_write_performed": false,
            "memory_store_write_performed_count": 0,
            "memory_store_mutation_allowed": false,
            "memory_store_mutated": false,
            "rollback_execution_allowed": false,
            "rollback_executed": false,
            "raw_payload_plaintext_recorded": false,
            "raw_payload_plaintext_persisted": false,
            "secret_material_read": false,
            "provider_prompt_replay_enabled": false,
            "provider_invoked": false,
            "model_invoked": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restarted": false,
            "service_restart_performed": false,
            "active_binary_mutated": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "denied_by_activation_command_result_receipt_release_artifact_publication": denials,
            "denied_by_activation_command_result_receipt_release_artifact_publication_count": denied_count,
            "source_terminal_operator_decision_public_claim_denial_count": source_u64("denied_by_activation_command_result_receipt_terminal_operator_decision_public_claim_count"),
            "release_artifact_publication_forbidden": true,
            "release_artifact_write_forbidden": true,
            "public_artifact_write_forbidden": true,
            "artifact_signature_notarization_forbidden": true,
            "publication_queue_forbidden": true,
            "publication_manifest_forbidden": true,
            "public_distribution_forbidden": true,
            "public_release_publication_forbidden": true,
            "public_ga_claim_forbidden": true,
            "terminal_operator_decision_release_approval_forbidden": true,
            "activation_from_release_artifact_publication_forbidden": true,
            "runtime_provider_memory_kg_release_artifact_publication_forbidden": true,
            "external_public_install_restart_active_binary_publication_forbidden": true,
            "allowed_next_actions": [
                {
                    "action": "review_runtime_provider_router_activation_command_result_receipt_release_artifact_publication_denial",
                    "status": "allowed_report_only",
                    "publishes_release_artifact": false,
                    "claims_public_release": false,
                    "writes_release_artifact": false,
                    "activates_runtime": false,
                    "invokes_model": false,
                    "writes_memory_or_kg": false
                },
                {
                    "action": "stage_consolidated_memory_intelligence_kg_audit",
                    "status": "allowed_read_only_next_slice",
                    "writes_release_artifact": false,
                    "claims_public_release": false,
                    "activates_runtime": false,
                    "invokes_model": false,
                    "writes_memory_or_kg": false
                },
                {
                    "action": "run_full_light_preflight",
                    "status": "allowed_verification_only",
                    "publishes_release_artifact": false,
                    "mutates_runtime": false,
                    "invokes_model": false,
                    "writes_kg": false
                }
            ],
        }),
    );

    if let Some(report_object) = report.as_object_mut() {
        for key in [
            "activation_command_result_receipt_release_artifact_publication_allowed",
            "activation_command_result_receipt_release_artifact_publication_requested",
            "activation_command_result_receipt_release_artifact_publication_accepted",
            "activation_command_result_receipt_release_artifact_publication_recorded",
            "activation_command_result_receipt_release_artifact_publication_persisted",
            "activation_command_result_receipt_release_artifact_publication_materialized",
            "release_artifact_publication_allowed",
            "release_artifact_publication_requested",
            "release_artifact_publication_accepted",
            "release_artifact_publication_recorded",
            "release_artifact_publication_persisted",
            "release_artifact_publication_materialized",
            "release_artifact_filesystem_written",
            "release_artifact_written",
            "public_artifact_written",
            "artifact_signature_accepted",
            "artifact_notarization_accepted",
            "publication_queue_enqueued",
            "publication_manifest_written",
            "public_distribution_performed",
            "public_release_published",
            "public_ga_claimed",
            "public_claim_promoted",
            "public_version_tag_created",
            "release_notes_materialized",
            "changelog_materialized",
            "terminal_operator_decision_promoted_to_release_approval",
            "telegram_send_performed",
            "channel_send_performed",
            "external_send_performed",
            "activation_command_result_receipt_recorded",
            "activation_command_result_receipt_persisted",
            "activation_command_result_receipt_accepted",
            "activation_command_result_receipt_materialized",
            "activation_command_result_receipt_filesystem_written",
            "activation_command_completion_ack_recorded",
            "activation_allowed_by_release_artifact_publication",
            "activation_allowed_by_terminal_operator_decision",
            "activation_allowed_by_result_receipt",
            "activation_allowed",
            "activation_performed",
            "activation_activated",
            "live_mutation_execution_performed",
            "memory_write_execution_performed",
            "memory_store_write_performed",
            "memory_store_mutated",
            "live_kg_write_performed",
            "rollback_executed",
            "raw_payload_plaintext_recorded",
            "raw_payload_plaintext_persisted",
            "secret_material_read",
            "auth_secret_read",
            "credential_read",
            "secret_file_read",
            "provider_prompt_replay_enabled",
            "provider_invoked",
            "model_invoked",
            "provider_prompt_replayed",
            "runtime_router_mutated",
            "runtime_attachment_performed",
            "live_context_attached",
            "context_injection_performed",
            "adapter_invoked",
            "filesystem_written",
            "install_executed",
            "launchd_mutated",
            "service_restart_performed",
            "service_restarted",
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
            "activation_command_result_receipt_release_artifact_publication_recorded",
            "activation_command_result_receipt_release_artifact_publication_persisted",
            "activation_command_result_receipt_release_artifact_publication_materialized",
            "release_artifact_publication_recorded",
            "release_artifact_publication_persisted",
            "release_artifact_publication_materialized",
            "release_artifact_filesystem_written",
            "release_artifact_written",
            "public_artifact_written",
            "artifact_signature_accepted",
            "artifact_notarization_accepted",
            "publication_queue_enqueued",
            "publication_manifest_written",
            "public_distribution_performed",
            "public_release_published",
            "public_ga_claimed",
            "public_claim_promoted",
            "public_version_tag_created",
            "release_notes_materialized",
            "changelog_materialized",
            "terminal_operator_decision_promoted_to_release_approval",
            "telegram_send_performed",
            "channel_send_performed",
            "external_send_performed",
            "activation_command_result_receipt_terminal_operator_decision_recorded",
            "activation_command_result_receipt_terminal_operator_decision_persisted",
            "activation_command_result_receipt_terminal_operator_decision_materialized",
            "activation_command_result_receipt_terminal_operator_decision_filesystem_written",
            "activation_command_result_receipt_public_claim_recorded",
            "activation_command_result_receipt_public_claim_persisted",
            "activation_command_result_receipt_public_claim_materialized",
            "activation_command_result_receipt_public_claim_promoted",
            "activation_command_result_receipt_public_release_published",
            "activation_command_result_receipt_public_distribution_performed",
            "activation_command_result_receipt_public_artifact_written",
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
            "activation_performed",
            "live_mutation_execution_performed",
            "memory_write_execution_performed",
            "memory_store_write_performed",
            "memory_store_mutated",
            "live_kg_write_performed",
            "rollback_executed",
            "raw_payload_plaintext_recorded",
            "raw_payload_plaintext_persisted",
            "secret_material_read",
            "auth_secret_read",
            "credential_read",
            "secret_file_read",
            "provider_prompt_replay_enabled",
            "provider_invoked",
            "model_invoked",
            "provider_prompt_replayed",
            "runtime_router_mutated",
            "runtime_attachment_performed",
            "live_context_attached",
            "context_injection_performed",
            "adapter_invoked",
            "filesystem_written",
            "install_executed",
            "launchd_mutated",
            "service_restarted",
            "service_restart_performed",
            "active_binary_mutated",
        ] {
            side_effects.insert(key.to_string(), serde_json::json!(false));
        }
    }
    report
}
