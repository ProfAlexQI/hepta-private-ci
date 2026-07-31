fn hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_audit_trail_immutable_evidence_denial_boundary_report()
-> serde_json::Value {
    const AUDIT_SURFACES: &[&str] = &[
        "source_cancellation_supersession_report_required",
        "audit_trail_request_shape_denied",
        "immutable_evidence_request_shape_denied",
        "append_only_audit_log_recording_denied",
        "evidence_hash_chain_recording_denied",
        "attestation_witness_recording_denied",
        "audit_trail_materialization_denied",
        "immutable_evidence_persistence_denied",
        "ledger_index_delivery_evidence_denied",
        "activation_from_audit_evidence_denied",
        "memory_write_rollback_secret_provider_evidence_denied",
        "external_public_install_restart_evidence_denied",
    ];
    const DENIED_BY: &[&str] = &[
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
        "memory_write_evidence_denied",
        "live_mutation_evidence_denied",
        "rollback_evidence_denied",
        "secret_material_evidence_denied",
        "provider_prompt_evidence_denied",
        "external_public_install_restart_active_binary_evidence_denied",
    ];
    const FALSE_KEYS: &[&str] = &[
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
        "activation_command_result_receipt_delivery_evidence_recorded",
        "activation_command_result_receipt_cancellation_allowed",
        "activation_command_result_receipt_cancellation_recorded",
        "activation_command_result_receipt_cancellation_persisted",
        "activation_command_result_receipt_supersession_allowed",
        "activation_command_result_receipt_supersession_recorded",
        "activation_command_result_receipt_supersession_persisted",
        "activation_command_result_receipt_replacement_receipt_accepted",
        "activation_command_result_receipt_replacement_hash_accepted",
        "activation_command_result_receipt_tombstone_recorded",
        "activation_command_result_receipt_delete_marker_recorded",
        "activation_command_result_receipt_ordering_allowed",
        "activation_command_result_receipt_ordering_recorded",
        "activation_command_result_receipt_ordering_persisted",
        "activation_command_result_receipt_sequence_cursor_accepted",
        "activation_command_result_receipt_monotonicity_state_recorded",
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
        "activation_allowed_by_result_receipt_audit_trail",
        "activation_allowed_by_result_receipt_immutable_evidence",
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

    fn audit_fixture(
        id: &str,
        reason: &str,
        audit_evidence_status: &str,
        extra: serde_json::Value,
    ) -> serde_json::Value {
        let mut base = serde_json::Map::new();
        macro_rules! insert_fixture_json {
            ($key:literal, $value:expr) => {
                base.insert($key.to_string(), serde_json::json!($value));
            };
        }
        insert_fixture_json!("id", id);
        insert_fixture_json!("audit_trail_requested", true);
        insert_fixture_json!("immutable_evidence_requested", false);
        insert_fixture_json!("audit_evidence_status", audit_evidence_status);
        insert_fixture_json!("source_cancellation_supersession_present", true);
        insert_fixture_json!("source_cancellation_supersession_ready", true);
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
            "delivery_evidence_recorded",
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
    let source = std::thread::Builder::new()
        .name("hepta-memory-write-result-receipt-cancellation-source-report".to_string())
        .stack_size(8 * 1024 * 1024)
        .spawn(
            hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_cancellation_supersession_denial_boundary_report,
        )
        .ok()
        .and_then(|handle| handle.join().ok())
        .unwrap_or_else(|| {
            serde_json::json!({
                "status": "blocked",
                "memory_write_execution_activation_command_result_receipt_cancellation_supersession_denial_boundary_ready": false,
                "memory_write_execution_activation_command_result_receipt_cancellation_supersession_denial_ready": false,
                "source_cancellation_source_report_thread_failed": true
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
    let source_ready = json_str(&source, "status") == "ready"
        && json_bool(
            &source,
            "memory_write_execution_activation_command_result_receipt_cancellation_supersession_denial_boundary_ready",
        )
        && json_bool(
            &source,
            "memory_write_execution_activation_command_result_receipt_cancellation_supersession_denial_ready",
        )
        && json_bool(
            &source,
            "memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_ready",
        )
        && json_u64(
            &source,
            "activation_command_result_receipt_cancellation_supersession_fixture_count",
        ) == 10
        && json_u64(
            &source,
            "blocked_activation_command_result_receipt_cancellation_supersession_fixture_count",
        ) == 10
        && json_u64(
            &source,
            "accepted_activation_command_result_receipt_cancellation_supersession_fixture_count",
        ) == 0
        && json_u64(
            &source,
            "activation_command_result_receipt_cancellation_performed_count",
        ) == 0
        && json_u64(
            &source,
            "activation_command_result_receipt_supersession_performed_count",
        ) == 0
        && json_u64(
            &source,
            "denied_by_activation_command_result_receipt_cancellation_supersession_count",
        ) == 24
        && !json_bool(
            &source,
            "activation_command_result_receipt_cancellation_recorded",
        )
        && !json_bool(
            &source,
            "activation_command_result_receipt_supersession_recorded",
        )
        && !json_bool(
            &source,
            "activation_command_result_receipt_replacement_receipt_accepted",
        )
        && !json_bool(
            &source,
            "activation_command_result_receipt_tombstone_recorded",
        )
        && !json_bool(&source, "activation_command_result_receipt_recorded")
        && !json_bool(&source, "activation_command_result_receipt_persisted")
        && !json_bool(&source, "activation_command_result_receipt_accepted")
        && !json_bool(&source, "activation_allowed")
        && !json_bool(&source, "live_mutation_execution_performed")
        && !json_bool(&source, "memory_store_write_performed")
        && !json_bool(&source, "memory_store_mutated")
        && !json_bool(&source, "rollback_executed")
        && !json_bool(&source, "provider_invoked")
        && !json_bool(&source, "model_invoked")
        && !json_bool(&source, "external_send_performed")
        && !json_bool(&source, "release_artifact_written")
        && !json_bool(&source, "active_binary_mutated")
        && side_effects_all_false(&source);

    let audit_fixtures = serde_json::Value::Array(vec![
        audit_fixture(
            "activation-result-receipt-audit-missing-source-cancellation-report",
            "source_cancellation_supersession_report_required",
            "blocked_noop",
            serde_json::json!({
                "source_cancellation_supersession_present": false,
                "source_cancellation_supersession_ready": false
            }),
        ),
        audit_fixture(
            "activation-result-receipt-audit-trail-append-request",
            "audit_trail_append_request_denied",
            "blocked_noop",
            serde_json::json!({
                "audit_trail_request_shape": "append_blocked_noop_receipt_audit_trail"
            }),
        ),
        audit_fixture(
            "activation-result-receipt-immutable-evidence-packet",
            "immutable_evidence_packet_request_denied",
            "blocked_evidence_noop",
            serde_json::json!({
                "immutable_evidence_requested": true,
                "audit_trail_requested": false,
                "immutable_evidence_request_shape": "seal_blocked_noop_receipt_as_immutable_evidence"
            }),
        ),
        audit_fixture(
            "activation-result-receipt-hash-chain-merkle-root",
            "hash_chain_merkle_root_evidence_denied",
            "blocked_evidence_noop",
            serde_json::json!({
                "immutable_evidence_requested": true,
                "audit_trail_requested": false,
                "hash_chain_requested": true,
                "merkle_root_requested": true
            }),
        ),
        audit_fixture(
            "activation-result-receipt-attestation-witness-notary",
            "attestation_witness_notary_evidence_denied",
            "blocked_evidence_noop",
            serde_json::json!({
                "immutable_evidence_requested": true,
                "audit_trail_requested": false,
                "attestation_requested": true,
                "witness_requested": true,
                "notary_requested": true
            }),
        ),
        audit_fixture(
            "activation-result-receipt-audit-trail-materialization",
            "audit_trail_materialization_filesystem_denied",
            "blocked_noop",
            serde_json::json!({
                "audit_trail_materialization_requested": true,
                "audit_trail_filesystem_write_requested": true
            }),
        ),
        audit_fixture(
            "activation-result-receipt-ledger-index-delivery-evidence",
            "ledger_index_delivery_evidence_denied",
            "blocked_noop",
            serde_json::json!({
                "ledger_evidence_requested": true,
                "index_evidence_requested": true,
                "delivery_evidence_requested": true
            }),
        ),
        audit_fixture(
            "activation-result-receipt-activation-from-audit-evidence",
            "activation_from_audit_evidence_denied",
            "blocked_evidence_noop",
            serde_json::json!({
                "immutable_evidence_requested": true,
                "audit_trail_requested": false,
                "activation_from_audit_evidence_requested": true
            }),
        ),
        audit_fixture(
            "activation-result-receipt-memory-rollback-secret-provider-evidence",
            "memory_write_rollback_secret_provider_evidence_denied",
            "blocked_evidence_noop",
            serde_json::json!({
                "immutable_evidence_requested": true,
                "audit_trail_requested": false,
                "memory_write_evidence_requested": true,
                "rollback_evidence_requested": true,
                "secret_material_evidence_requested": true,
                "provider_prompt_evidence_requested": true
            }),
        ),
        audit_fixture(
            "activation-result-receipt-external-public-install-evidence",
            "external_public_install_restart_active_binary_evidence_denied",
            "blocked_evidence_noop",
            serde_json::json!({
                "immutable_evidence_requested": true,
                "audit_trail_requested": false,
                "external_send_evidence_requested": true,
                "public_claim_evidence_requested": true,
                "release_artifact_evidence_requested": true,
                "install_evidence_requested": true,
                "service_restart_evidence_requested": true,
                "active_binary_mutation_evidence_requested": true
            }),
        ),
    ]);

    let source_report_sha256 = sha256_json_value(&source);
    let boundary_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-audit-trail-immutable-evidence-denial-boundary-v1:{}:{}",
        route_matrix.route_count, source_report_sha256,
    ));
    let report_ready = route_matrix.ready
        && route_count_source_command_accepted
        && source_ready
        && AUDIT_SURFACES.len() == 12
        && audit_fixtures.as_array().map(std::vec::Vec::len) == Some(10)
        && DENIED_BY.len() == 24;

    let allowed_next_actions = serde_json::json!([
        {
            "action": "run_memory_write_execution_activation_command_result_receipt_audit_trail_immutable_evidence_denial_boundary_require_live_gate",
            "status": "allowed_verification_only",
            "records_audit_trail": false,
            "records_immutable_evidence": false,
            "persists_evidence": false,
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
            "action": "prepare_memory_write_execution_activation_command_result_receipt_retention_expiry_garbage_collection_denial_boundary",
            "status": "allowed_report_only_next_slice",
            "records_retention_policy": false,
            "executes_expiry_or_gc": false,
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
        "hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_audit_trail_immutable_evidence_denial_boundary_route"
    );
    insert_report_json!(
        "endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_COMMAND_RESULT_RECEIPT_AUDIT_TRAIL_IMMUTABLE_EVIDENCE_DENIAL_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_command",
        "/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-audit-trail-immutable-evidence-denial-boundary --json"
    );
    insert_report_json!("native_route", true);
    insert_report_json!("side_effect_free", true);
    insert_report_json!("audit_date", "2026-07-03");
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_audit_trail_immutable_evidence_denial_boundary_schema_version",
        "memory_write_execution_activation_command_result_receipt_audit_trail_immutable_evidence_denial_boundary_v1"
    );
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_audit_trail_immutable_evidence_denial_boundary_ready",
        report_ready
    );
    insert_report_json!(
        "activation_command_result_receipt_audit_trail_immutable_evidence_mode",
        "memory_write_execution_activation_command_result_receipt_audit_trail_immutable_evidence_denial"
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
        "source_activation_command_result_receipt_cancellation_supersession_boundary_endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_COMMAND_RESULT_RECEIPT_CANCELLATION_SUPERSESSION_DENIAL_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_activation_command_result_receipt_cancellation_supersession_boundary_ready",
        source_ready
    );
    insert_report_json!(
        "source_activation_command_result_receipt_cancellation_supersession_ready",
        json_bool(
            &source,
            "memory_write_execution_activation_command_result_receipt_cancellation_supersession_denial_ready"
        )
    );
    insert_report_json!(
        "source_activation_command_result_receipt_cancellation_supersession_boundary_report_sha256",
        source_report_sha256
    );
    insert_report_json!(
        "source_activation_command_result_receipt_cancellation_supersession_report_sha256",
        source_report_sha256
    );
    for key in [
        "source_activation_command_result_receipt_ordering_monotonicity_boundary_report_sha256",
        "source_activation_command_result_receipt_ordering_monotonicity_report_sha256",
        "source_activation_command_result_receipt_replay_idempotency_boundary_report_sha256",
        "source_activation_command_result_receipt_no_persistence_boundary_report_sha256",
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
            source
                .get(key)
                .cloned()
                .unwrap_or_else(|| serde_json::json!("")),
        );
    }
    insert_report_json!("minimum_required_samples", 24);
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready",
        true
    );
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
        "required_activation_command_result_receipt_audit_trail_immutable_evidence_surface_count",
        12
    );
    insert_report_json!(
        "ready_activation_command_result_receipt_audit_trail_immutable_evidence_surface_count",
        12
    );
    insert_report_json!(
        "side_effect_free_activation_command_result_receipt_audit_trail_immutable_evidence_surface_count",
        12
    );
    insert_report_json!(
        "required_activation_command_result_receipt_audit_trail_immutable_evidence_fixture_count",
        10
    );
    insert_report_json!(
        "activation_command_result_receipt_audit_trail_immutable_evidence_fixture_count",
        10
    );
    insert_report_json!(
        "blocked_activation_command_result_receipt_audit_trail_immutable_evidence_fixture_count",
        10
    );
    insert_report_json!(
        "noop_activation_command_result_receipt_audit_trail_immutable_evidence_fixture_count",
        10
    );
    insert_report_json!(
        "allowed_activation_command_result_receipt_audit_trail_immutable_evidence_fixture_count",
        0
    );
    insert_report_json!(
        "accepted_activation_command_result_receipt_audit_trail_immutable_evidence_fixture_count",
        0
    );
    insert_report_json!(
        "activation_command_result_receipt_audit_trail_denied_count",
        10
    );
    insert_report_json!(
        "activation_command_result_receipt_immutable_evidence_denied_count",
        10
    );
    insert_report_json!(
        "activation_command_result_receipt_audit_trail_performed_count",
        0
    );
    insert_report_json!(
        "activation_command_result_receipt_immutable_evidence_performed_count",
        0
    );
    insert_report_json!("memory_store_write_performed_count", 0);

    for &key in FALSE_KEYS {
        report.insert(key.to_string(), serde_json::json!(false));
    }
    report.insert(
        "activation_command_result_receipt_audit_trail_immutable_evidence_surfaces".to_string(),
        serde_json::json!(AUDIT_SURFACES),
    );
    report.insert(
        "activation_command_result_receipt_audit_trail_immutable_evidence_fixtures".to_string(),
        audit_fixtures,
    );
    report.insert(
        "denied_by_activation_command_result_receipt_audit_trail_immutable_evidence".to_string(),
        serde_json::json!(DENIED_BY),
    );
    insert_report_json!(
        "denied_by_activation_command_result_receipt_audit_trail_immutable_evidence_count",
        24
    );
    report.insert("allowed_next_actions".to_string(), allowed_next_actions);
    report.insert(
        "side_effects".to_string(),
        serde_json::Value::Object(side_effects),
    );

    serde_json::Value::Object(report)
}

fn hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_retention_expiry_garbage_collection_denial_boundary_report()
-> serde_json::Value {
    const RETENTION_GC_SURFACES: &[&str] = &[
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
        "memory_write_rollback_secret_provider_gc_denied",
        "external_public_install_restart_active_binary_gc_denied",
    ];
    const DENIED_BY: &[&str] = &[
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
        "memory_write_gc_denied",
        "rollback_gc_denied",
        "secret_material_gc_denied",
        "provider_prompt_gc_denied",
        "external_public_install_restart_active_binary_gc_denied",
    ];
    const FALSE_KEYS: &[&str] = &[
        "activation_command_result_receipt_retention_policy_allowed",
        "activation_command_result_receipt_retention_policy_recorded",
        "activation_command_result_receipt_retention_policy_persisted",
        "activation_command_result_receipt_retention_policy_materialized",
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
        "activation_command_result_receipt_delivery_retention_recorded",
        "activation_command_result_receipt_audit_trail_recorded",
        "activation_command_result_receipt_audit_trail_persisted",
        "activation_command_result_receipt_immutable_evidence_recorded",
        "activation_command_result_receipt_immutable_evidence_persisted",
        "activation_command_result_receipt_hash_chain_recorded",
        "activation_command_result_receipt_merkle_root_recorded",
        "activation_command_result_receipt_attestation_recorded",
        "activation_command_result_receipt_witness_recorded",
        "activation_command_result_receipt_notary_recorded",
        "activation_command_result_receipt_cancellation_allowed",
        "activation_command_result_receipt_cancellation_recorded",
        "activation_command_result_receipt_supersession_allowed",
        "activation_command_result_receipt_supersession_recorded",
        "activation_command_result_receipt_ordering_allowed",
        "activation_command_result_receipt_ordering_recorded",
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
        "activation_allowed_by_result_receipt_retention",
        "activation_allowed_by_result_receipt_expiry",
        "activation_allowed_by_result_receipt_garbage_collection",
        "activation_allowed_by_result_receipt_audit_trail",
        "activation_allowed_by_result_receipt_immutable_evidence",
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

    fn retention_gc_fixture(
        id: &str,
        status: &str,
        reason: &str,
        extra: serde_json::Value,
    ) -> serde_json::Value {
        let mut base = serde_json::Map::new();
        macro_rules! insert_fixture_json {
            ($key:literal, $value:expr) => {
                base.insert($key.to_string(), serde_json::json!($value));
            };
        }
        insert_fixture_json!("id", id);
        insert_fixture_json!("retention_gc_status", status);
        insert_fixture_json!("reason", reason);
        insert_fixture_json!("retention_requested", true);
        insert_fixture_json!("expiry_requested", false);
        insert_fixture_json!("garbage_collection_requested", false);
        insert_fixture_json!("source_audit_evidence_present", true);
        insert_fixture_json!("source_audit_evidence_ready", true);
        for key in [
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
            "memory_write_gc_evidence_requested",
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
            base.insert(key.to_string(), serde_json::json!(false));
        }
        for key in [
            "retention_policy_allowed",
            "retention_policy_recorded",
            "retention_policy_persisted",
            "retention_policy_materialized",
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
            "delivery_retention_recorded",
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
        if let Some(extra) = extra.as_object() {
            for (key, value) in extra {
                base.insert(key.clone(), value.clone());
            }
        }
        serde_json::Value::Object(base)
    }

    let route_matrix = control_ui_route_parity_report();
    let source = std::thread::Builder::new()
        .name("hepta-memory-write-result-receipt-audit-evidence-source-report".to_string())
        .stack_size(8 * 1024 * 1024)
        .spawn(
            hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_audit_trail_immutable_evidence_denial_boundary_report,
        )
        .ok()
        .and_then(|handle| handle.join().ok())
        .unwrap_or_else(|| {
            serde_json::json!({
                "status": "blocked",
                "memory_write_execution_activation_command_result_receipt_audit_trail_immutable_evidence_denial_boundary_ready": false,
                "memory_write_execution_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready": false,
                "source_audit_evidence_source_report_thread_failed": true
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
    let source_ready = json_str(&source, "status") == "ready"
        && json_bool(
            &source,
            "memory_write_execution_activation_command_result_receipt_audit_trail_immutable_evidence_denial_boundary_ready",
        )
        && json_bool(
            &source,
            "memory_write_execution_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready",
        )
        && json_u64(
            &source,
            "activation_command_result_receipt_audit_trail_immutable_evidence_fixture_count",
        ) == 10
        && json_u64(
            &source,
            "blocked_activation_command_result_receipt_audit_trail_immutable_evidence_fixture_count",
        ) == 10
        && json_u64(
            &source,
            "accepted_activation_command_result_receipt_audit_trail_immutable_evidence_fixture_count",
        ) == 0
        && json_u64(
            &source,
            "activation_command_result_receipt_audit_trail_performed_count",
        ) == 0
        && json_u64(
            &source,
            "activation_command_result_receipt_immutable_evidence_performed_count",
        ) == 0
        && json_u64(
            &source,
            "denied_by_activation_command_result_receipt_audit_trail_immutable_evidence_count",
        ) == 24
        && !json_bool(
            &source,
            "activation_command_result_receipt_audit_trail_recorded",
        )
        && !json_bool(
            &source,
            "activation_command_result_receipt_immutable_evidence_recorded",
        )
        && !json_bool(
            &source,
            "activation_command_result_receipt_hash_chain_recorded",
        )
        && !json_bool(
            &source,
            "activation_command_result_receipt_merkle_root_recorded",
        )
        && !json_bool(
            &source,
            "activation_command_result_receipt_attestation_recorded",
        )
        && !json_bool(
            &source,
            "activation_command_result_receipt_witness_recorded",
        )
        && !json_bool(&source, "activation_command_result_receipt_notary_recorded")
        && !json_bool(&source, "activation_command_result_receipt_recorded")
        && !json_bool(&source, "activation_command_result_receipt_persisted")
        && !json_bool(&source, "activation_allowed")
        && !json_bool(&source, "live_mutation_execution_performed")
        && !json_bool(&source, "memory_store_write_performed")
        && !json_bool(&source, "memory_store_mutated")
        && !json_bool(&source, "rollback_executed")
        && !json_bool(&source, "provider_invoked")
        && !json_bool(&source, "model_invoked")
        && !json_bool(&source, "external_send_performed")
        && !json_bool(&source, "release_artifact_written")
        && !json_bool(&source, "active_binary_mutated")
        && side_effects_all_false(&source);

    let retention_gc_fixtures = serde_json::Value::Array(vec![
        retention_gc_fixture(
            "activation-result-receipt-retention-missing-source-audit-evidence",
            "blocked_noop",
            "source_audit_trail_immutable_evidence_report_required",
            serde_json::json!({
                "source_audit_evidence_present": false,
                "source_audit_evidence_ready": false
            }),
        ),
        retention_gc_fixture(
            "activation-result-receipt-retention-policy-write-request",
            "blocked_noop",
            "retention_policy_write_request_denied",
            serde_json::json!({
                "retention_policy_request_shape": "record_blocked_noop_receipt_retention_policy"
            }),
        ),
        retention_gc_fixture(
            "activation-result-receipt-retention-index-record",
            "blocked_noop",
            "retention_index_recording_denied",
            serde_json::json!({
                "retention_index_requested": true
            }),
        ),
        retention_gc_fixture(
            "activation-result-receipt-expiry-scheduler-timer",
            "blocked_expiry_noop",
            "expiry_scheduler_timer_denied",
            serde_json::json!({
                "retention_requested": false,
                "expiry_requested": true,
                "expiry_schedule_requested": true,
                "expiry_timer_requested": true
            }),
        ),
        retention_gc_fixture(
            "activation-result-receipt-ttl-update-extension",
            "blocked_expiry_noop",
            "ttl_update_extension_denied",
            serde_json::json!({
                "retention_requested": false,
                "expiry_requested": true,
                "ttl_update_requested": true,
                "ttl_extension_requested": true
            }),
        ),
        retention_gc_fixture(
            "activation-result-receipt-garbage-collection-scan",
            "blocked_gc_noop",
            "garbage_collection_scan_denied",
            serde_json::json!({
                "retention_requested": false,
                "garbage_collection_requested": true,
                "garbage_collection_scan_requested": true
            }),
        ),
        retention_gc_fixture(
            "activation-result-receipt-delete-tombstone-sweep",
            "blocked_gc_noop",
            "delete_tombstone_sweep_denied",
            serde_json::json!({
                "retention_requested": false,
                "garbage_collection_requested": true,
                "delete_requested": true,
                "tombstone_requested": true,
                "sweep_requested": true
            }),
        ),
        retention_gc_fixture(
            "activation-result-receipt-archive-compaction",
            "blocked_gc_noop",
            "archive_compaction_denied",
            serde_json::json!({
                "retention_requested": false,
                "garbage_collection_requested": true,
                "archive_requested": true,
                "compaction_requested": true
            }),
        ),
        retention_gc_fixture(
            "activation-result-receipt-activation-memory-provider-retention-gc",
            "blocked_gc_noop",
            "activation_memory_provider_retention_gc_denied",
            serde_json::json!({
                "retention_requested": false,
                "expiry_requested": true,
                "garbage_collection_requested": true,
                "activation_from_retention_gc_requested": true,
                "memory_write_gc_evidence_requested": true,
                "rollback_gc_evidence_requested": true,
                "secret_material_gc_evidence_requested": true,
                "provider_prompt_gc_evidence_requested": true
            }),
        ),
        retention_gc_fixture(
            "activation-result-receipt-external-public-install-retention-gc",
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
                "active_binary_gc_evidence_requested": true
            }),
        ),
    ]);

    let source_report_sha256 = sha256_json_value(&source);
    let boundary_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-retention-expiry-garbage-collection-denial-boundary-v1:{}:{}",
        route_matrix.route_count, source_report_sha256,
    ));
    let report_ready = route_matrix.ready
        && route_count_source_command_accepted
        && source_ready
        && RETENTION_GC_SURFACES.len() == 12
        && retention_gc_fixtures.as_array().map(std::vec::Vec::len) == Some(10)
        && DENIED_BY.len() == 29;

    let allowed_next_actions = serde_json::json!([
        {
            "action": "run_memory_write_execution_activation_command_result_receipt_retention_expiry_garbage_collection_denial_boundary_require_live_gate",
            "status": "allowed_verification_only",
            "records_retention_policy": false,
            "executes_expiry_or_gc": false,
            "deletes_memory": false,
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
            "action": "prepare_memory_write_execution_activation_command_result_receipt_export_query_observability_denial_boundary",
            "status": "allowed_report_only_next_slice",
            "exports_receipt": false,
            "registers_query": false,
            "records_observability": false,
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
        "hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_retention_expiry_garbage_collection_denial_boundary_route"
    );
    insert_report_json!(
        "endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_COMMAND_RESULT_RECEIPT_RETENTION_EXPIRY_GARBAGE_COLLECTION_DENIAL_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_command",
        "/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-retention-expiry-garbage-collection-denial-boundary --json"
    );
    insert_report_json!("native_route", true);
    insert_report_json!("side_effect_free", true);
    insert_report_json!("audit_date", "2026-07-03");
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_retention_expiry_garbage_collection_denial_boundary_schema_version",
        "memory_write_execution_activation_command_result_receipt_retention_expiry_garbage_collection_denial_boundary_v1"
    );
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_retention_expiry_garbage_collection_denial_boundary_ready",
        report_ready
    );
    insert_report_json!(
        "activation_command_result_receipt_retention_expiry_garbage_collection_mode",
        "memory_write_execution_activation_command_result_receipt_retention_expiry_garbage_collection_denial"
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
        "source_activation_command_result_receipt_audit_trail_immutable_evidence_boundary_endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_COMMAND_RESULT_RECEIPT_AUDIT_TRAIL_IMMUTABLE_EVIDENCE_DENIAL_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_activation_command_result_receipt_audit_trail_immutable_evidence_boundary_ready",
        source_ready
    );
    insert_report_json!(
        "source_activation_command_result_receipt_audit_trail_immutable_evidence_ready",
        json_bool(
            &source,
            "memory_write_execution_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready"
        )
    );
    insert_report_json!(
        "source_activation_command_result_receipt_audit_trail_immutable_evidence_boundary_report_sha256",
        source_report_sha256
    );
    insert_report_json!(
        "source_activation_command_result_receipt_audit_trail_immutable_evidence_report_sha256",
        source_report_sha256
    );
    for key in [
        "source_activation_command_result_receipt_cancellation_supersession_boundary_report_sha256",
        "source_activation_command_result_receipt_cancellation_supersession_report_sha256",
        "source_activation_command_result_receipt_ordering_monotonicity_boundary_report_sha256",
        "source_activation_command_result_receipt_ordering_monotonicity_report_sha256",
        "source_activation_command_result_receipt_replay_idempotency_boundary_report_sha256",
        "source_activation_command_result_receipt_no_persistence_boundary_report_sha256",
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
            source
                .get(key)
                .cloned()
                .unwrap_or_else(|| serde_json::json!("")),
        );
    }
    insert_report_json!("minimum_required_samples", 24);
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready",
        true
    );
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
        "required_activation_command_result_receipt_retention_expiry_garbage_collection_surface_count",
        12
    );
    insert_report_json!(
        "ready_activation_command_result_receipt_retention_expiry_garbage_collection_surface_count",
        12
    );
    insert_report_json!(
        "side_effect_free_activation_command_result_receipt_retention_expiry_garbage_collection_surface_count",
        12
    );
    insert_report_json!(
        "required_activation_command_result_receipt_retention_expiry_garbage_collection_fixture_count",
        10
    );
    insert_report_json!(
        "activation_command_result_receipt_retention_expiry_garbage_collection_fixture_count",
        10
    );
    insert_report_json!(
        "blocked_activation_command_result_receipt_retention_expiry_garbage_collection_fixture_count",
        10
    );
    insert_report_json!(
        "noop_activation_command_result_receipt_retention_expiry_garbage_collection_fixture_count",
        10
    );
    insert_report_json!(
        "allowed_activation_command_result_receipt_retention_expiry_garbage_collection_fixture_count",
        0
    );
    insert_report_json!(
        "accepted_activation_command_result_receipt_retention_expiry_garbage_collection_fixture_count",
        0
    );
    insert_report_json!(
        "activation_command_result_receipt_retention_denied_count",
        10
    );
    insert_report_json!("activation_command_result_receipt_expiry_denied_count", 10);
    insert_report_json!(
        "activation_command_result_receipt_garbage_collection_denied_count",
        10
    );
    insert_report_json!(
        "activation_command_result_receipt_retention_performed_count",
        0
    );
    insert_report_json!(
        "activation_command_result_receipt_expiry_performed_count",
        0
    );
    insert_report_json!(
        "activation_command_result_receipt_garbage_collection_performed_count",
        0
    );
    insert_report_json!("memory_store_write_performed_count", 0);

    for &key in FALSE_KEYS {
        report.insert(key.to_string(), serde_json::json!(false));
    }
    report.insert(
        "activation_command_result_receipt_retention_expiry_garbage_collection_surfaces"
            .to_string(),
        serde_json::json!(RETENTION_GC_SURFACES),
    );
    report.insert(
        "activation_command_result_receipt_retention_expiry_garbage_collection_fixtures"
            .to_string(),
        retention_gc_fixtures,
    );
    report.insert(
        "denied_by_activation_command_result_receipt_retention_expiry_garbage_collection"
            .to_string(),
        serde_json::json!(DENIED_BY),
    );
    insert_report_json!(
        "denied_by_activation_command_result_receipt_retention_expiry_garbage_collection_count",
        29
    );
    report.insert("allowed_next_actions".to_string(), allowed_next_actions);
    report.insert(
        "side_effects".to_string(),
        serde_json::Value::Object(side_effects),
    );

    serde_json::Value::Object(report)
}

fn hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_export_query_observability_denial_boundary_report()
-> serde_json::Value {
    const EXPORT_QUERY_OBSERVABILITY_SURFACES: &[&str] = &[
        "source_retention_expiry_garbage_collection_report_required",
        "export_request_shape_denied",
        "export_artifact_write_denied",
        "query_endpoint_materialization_denied",
        "query_index_cache_recording_denied",
        "observability_metric_emission_denied",
        "trace_span_log_event_recording_denied",
        "dashboard_alert_slo_materialization_denied",
        "ledger_index_delivery_observability_evidence_denied",
        "activation_from_export_query_observability_denied",
        "memory_write_rollback_secret_provider_observability_denied",
        "external_public_install_restart_active_binary_observability_denied",
    ];
    const DENIED_BY: &[&str] = &[
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
        "memory_write_observability_denied",
        "rollback_observability_denied",
        "secret_material_observability_denied",
        "provider_prompt_observability_denied",
        "external_public_install_restart_active_binary_observability_denied",
    ];
    const FALSE_KEYS: &[&str] = &[
        "activation_command_result_receipt_export_allowed",
        "activation_command_result_receipt_export_request_accepted",
        "activation_command_result_receipt_export_recorded",
        "activation_command_result_receipt_export_persisted",
        "activation_command_result_receipt_export_artifact_written",
        "activation_command_result_receipt_export_stream_opened",
        "activation_command_result_receipt_export_filesystem_written",
        "activation_command_result_receipt_query_allowed",
        "activation_command_result_receipt_query_request_accepted",
        "activation_command_result_receipt_query_registered",
        "activation_command_result_receipt_query_endpoint_materialized",
        "activation_command_result_receipt_query_index_recorded",
        "activation_command_result_receipt_query_cache_written",
        "activation_command_result_receipt_query_result_materialized",
        "activation_command_result_receipt_observability_allowed",
        "activation_command_result_receipt_observability_request_accepted",
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
        "activation_command_result_receipt_retention_policy_persisted",
        "activation_command_result_receipt_retention_index_recorded",
        "activation_command_result_receipt_expiry_recorded",
        "activation_command_result_receipt_expiry_scheduler_registered",
        "activation_command_result_receipt_expiry_timer_started",
        "activation_command_result_receipt_garbage_collection_scan_performed",
        "activation_command_result_receipt_delete_performed",
        "activation_command_result_receipt_tombstone_recorded",
        "activation_command_result_receipt_sweep_performed",
        "activation_command_result_receipt_archive_written",
        "activation_command_result_receipt_compaction_performed",
        "activation_command_result_receipt_audit_trail_recorded",
        "activation_command_result_receipt_immutable_evidence_recorded",
        "activation_command_result_receipt_cancellation_allowed",
        "activation_command_result_receipt_supersession_allowed",
        "activation_command_result_receipt_ordering_allowed",
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
        "activation_allowed_by_result_receipt_export",
        "activation_allowed_by_result_receipt_query",
        "activation_allowed_by_result_receipt_observability",
        "activation_allowed_by_result_receipt_retention",
        "activation_allowed_by_result_receipt_expiry",
        "activation_allowed_by_result_receipt_garbage_collection",
        "activation_allowed_by_result_receipt_audit_trail",
        "activation_allowed_by_result_receipt_immutable_evidence",
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

    fn export_query_observability_fixture(
        id: &str,
        status: &str,
        reason: &str,
        extra: serde_json::Value,
    ) -> serde_json::Value {
        let mut base = serde_json::Map::new();
        macro_rules! insert_fixture_json {
            ($key:literal, $value:expr) => {
                base.insert($key.to_string(), serde_json::json!($value));
            };
        }
        insert_fixture_json!("id", id);
        insert_fixture_json!("export_query_observability_status", status);
        insert_fixture_json!("reason", reason);
        insert_fixture_json!("source_retention_expiry_gc_present", true);
        insert_fixture_json!("source_retention_expiry_gc_ready", true);
        for key in [
            "export_requested",
            "export_file_requested",
            "export_stream_requested",
            "query_requested",
            "query_endpoint_requested",
            "query_index_requested",
            "query_cache_requested",
            "observability_requested",
            "metric_requested",
            "log_requested",
            "trace_requested",
            "span_requested",
            "dashboard_requested",
            "alert_requested",
            "slo_requested",
            "ledger_observability_requested",
            "index_observability_requested",
            "delivery_observability_requested",
            "activation_from_observability_requested",
            "memory_write_observability_requested",
            "rollback_observability_requested",
            "secret_material_observability_requested",
            "provider_prompt_observability_requested",
            "external_send_observability_requested",
            "public_claim_observability_requested",
            "release_artifact_observability_requested",
            "install_observability_requested",
            "service_restart_observability_requested",
            "active_binary_observability_requested",
        ] {
            base.insert(key.to_string(), serde_json::json!(false));
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
        if let Some(extra) = extra.as_object() {
            for (key, value) in extra {
                base.insert(key.clone(), value.clone());
            }
        }
        serde_json::Value::Object(base)
    }

    let route_matrix = control_ui_route_parity_report();
    let source = std::thread::Builder::new()
        .name("hepta-memory-write-result-receipt-retention-gc-source-report".to_string())
        .stack_size(8 * 1024 * 1024)
        .spawn(
            hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_retention_expiry_garbage_collection_denial_boundary_report,
        )
        .ok()
        .and_then(|handle| handle.join().ok())
        .unwrap_or_else(|| {
            serde_json::json!({
                "status": "blocked",
                "memory_write_execution_activation_command_result_receipt_retention_expiry_garbage_collection_denial_boundary_ready": false,
                "memory_write_execution_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready": false,
                "source_retention_expiry_gc_source_report_thread_failed": true
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
    let source_ready = json_str(&source, "status") == "ready"
        && json_bool(
            &source,
            "memory_write_execution_activation_command_result_receipt_retention_expiry_garbage_collection_denial_boundary_ready",
        )
        && json_bool(
            &source,
            "memory_write_execution_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready",
        )
        && json_u64(
            &source,
            "activation_command_result_receipt_retention_expiry_garbage_collection_fixture_count",
        ) == 10
        && json_u64(
            &source,
            "blocked_activation_command_result_receipt_retention_expiry_garbage_collection_fixture_count",
        ) == 10
        && json_u64(
            &source,
            "accepted_activation_command_result_receipt_retention_expiry_garbage_collection_fixture_count",
        ) == 0
        && json_u64(
            &source,
            "activation_command_result_receipt_retention_performed_count",
        ) == 0
        && json_u64(
            &source,
            "activation_command_result_receipt_expiry_performed_count",
        ) == 0
        && json_u64(
            &source,
            "activation_command_result_receipt_garbage_collection_performed_count",
        ) == 0
        && json_u64(
            &source,
            "denied_by_activation_command_result_receipt_retention_expiry_garbage_collection_count",
        ) == 29
        && !json_bool(
            &source,
            "activation_command_result_receipt_retention_policy_recorded",
        )
        && !json_bool(
            &source,
            "activation_command_result_receipt_expiry_scheduler_registered",
        )
        && !json_bool(
            &source,
            "activation_command_result_receipt_garbage_collection_scan_performed",
        )
        && !json_bool(
            &source,
            "activation_command_result_receipt_delete_performed",
        )
        && !json_bool(
            &source,
            "activation_command_result_receipt_compaction_performed",
        )
        && !json_bool(&source, "activation_command_result_receipt_recorded")
        && !json_bool(&source, "activation_command_result_receipt_persisted")
        && !json_bool(&source, "activation_allowed")
        && !json_bool(&source, "live_mutation_execution_performed")
        && !json_bool(&source, "memory_store_write_performed")
        && !json_bool(&source, "memory_store_mutated")
        && !json_bool(&source, "rollback_executed")
        && !json_bool(&source, "provider_invoked")
        && !json_bool(&source, "model_invoked")
        && !json_bool(&source, "external_send_performed")
        && !json_bool(&source, "release_artifact_written")
        && !json_bool(&source, "active_binary_mutated")
        && side_effects_all_false(&source);

    let export_query_observability_fixtures = serde_json::Value::Array(vec![
        export_query_observability_fixture(
            "activation-result-receipt-export-missing-source-retention-gc",
            "blocked_noop",
            "source_retention_expiry_gc_report_required",
            serde_json::json!({
                "source_retention_expiry_gc_present": false,
                "source_retention_expiry_gc_ready": false,
                "export_requested": true
            }),
        ),
        export_query_observability_fixture(
            "activation-result-receipt-export-artifact-request",
            "blocked_export_noop",
            "export_artifact_write_denied",
            serde_json::json!({
                "export_requested": true,
                "export_file_requested": true
            }),
        ),
        export_query_observability_fixture(
            "activation-result-receipt-export-stream-request",
            "blocked_export_noop",
            "export_stream_open_denied",
            serde_json::json!({
                "export_requested": true,
                "export_stream_requested": true
            }),
        ),
        export_query_observability_fixture(
            "activation-result-receipt-query-endpoint-request",
            "blocked_query_noop",
            "query_endpoint_materialization_denied",
            serde_json::json!({
                "query_requested": true,
                "query_endpoint_requested": true
            }),
        ),
        export_query_observability_fixture(
            "activation-result-receipt-query-index-cache-request",
            "blocked_query_noop",
            "query_index_cache_recording_denied",
            serde_json::json!({
                "query_requested": true,
                "query_index_requested": true,
                "query_cache_requested": true
            }),
        ),
        export_query_observability_fixture(
            "activation-result-receipt-observability-metric-request",
            "blocked_observability_noop",
            "observability_metric_emission_denied",
            serde_json::json!({
                "observability_requested": true,
                "metric_requested": true
            }),
        ),
        export_query_observability_fixture(
            "activation-result-receipt-observability-trace-log-request",
            "blocked_observability_noop",
            "trace_span_log_recording_denied",
            serde_json::json!({
                "observability_requested": true,
                "trace_requested": true,
                "span_requested": true,
                "log_requested": true
            }),
        ),
        export_query_observability_fixture(
            "activation-result-receipt-dashboard-alert-slo-request",
            "blocked_observability_noop",
            "dashboard_alert_slo_materialization_denied",
            serde_json::json!({
                "observability_requested": true,
                "dashboard_requested": true,
                "alert_requested": true,
                "slo_requested": true
            }),
        ),
        export_query_observability_fixture(
            "activation-result-receipt-activation-memory-provider-observability",
            "blocked_observability_noop",
            "activation_memory_provider_observability_denied",
            serde_json::json!({
                "observability_requested": true,
                "activation_from_observability_requested": true,
                "memory_write_observability_requested": true,
                "rollback_observability_requested": true,
                "secret_material_observability_requested": true,
                "provider_prompt_observability_requested": true
            }),
        ),
        export_query_observability_fixture(
            "activation-result-receipt-external-public-install-observability",
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
                "active_binary_observability_requested": true
            }),
        ),
    ]);

    let source_report_sha256 = sha256_json_value(&source);
    let boundary_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-export-query-observability-denial-boundary-v1:{}:{}",
        route_matrix.route_count, source_report_sha256,
    ));
    let report_ready = route_matrix.ready
        && route_count_source_command_accepted
        && source_ready
        && EXPORT_QUERY_OBSERVABILITY_SURFACES.len() == 12
        && export_query_observability_fixtures
            .as_array()
            .map(std::vec::Vec::len)
            == Some(10)
        && DENIED_BY.len() == 30;

    let allowed_next_actions = serde_json::json!([
        {
            "action": "run_memory_write_execution_activation_command_result_receipt_export_query_observability_denial_boundary_require_live_gate",
            "status": "allowed_verification_only",
            "exports_receipt": false,
            "writes_export_artifact": false,
            "opens_export_stream": false,
            "registers_query": false,
            "writes_query_index_or_cache": false,
            "emits_observability": false,
            "materializes_dashboard_or_alert": false,
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
            "action": "prepare_memory_write_execution_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary",
            "status": "allowed_report_only_next_slice",
            "records_operator_summary": false,
            "persists_briefing": false,
            "delivers_notification": false,
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
        "hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_export_query_observability_denial_boundary_route"
    );
    insert_report_json!(
        "endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_COMMAND_RESULT_RECEIPT_EXPORT_QUERY_OBSERVABILITY_DENIAL_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_command",
        "/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-export-query-observability-denial-boundary --json"
    );
    insert_report_json!("native_route", true);
    insert_report_json!("side_effect_free", true);
    insert_report_json!("audit_date", "2026-07-03");
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_export_query_observability_denial_boundary_schema_version",
        "memory_write_execution_activation_command_result_receipt_export_query_observability_denial_boundary_v1"
    );
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_export_query_observability_denial_boundary_ready",
        report_ready
    );
    insert_report_json!(
        "activation_command_result_receipt_export_query_observability_mode",
        "memory_write_execution_activation_command_result_receipt_export_query_observability_denial"
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
        "source_activation_command_result_receipt_retention_expiry_garbage_collection_boundary_endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_COMMAND_RESULT_RECEIPT_RETENTION_EXPIRY_GARBAGE_COLLECTION_DENIAL_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_activation_command_result_receipt_retention_expiry_garbage_collection_boundary_ready",
        source_ready
    );
    insert_report_json!(
        "source_activation_command_result_receipt_retention_expiry_garbage_collection_ready",
        json_bool(
            &source,
            "memory_write_execution_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready"
        )
    );
    insert_report_json!(
        "source_activation_command_result_receipt_retention_expiry_garbage_collection_boundary_report_sha256",
        source_report_sha256
    );
    insert_report_json!(
        "source_activation_command_result_receipt_retention_expiry_garbage_collection_report_sha256",
        source_report_sha256
    );
    for key in [
        "source_activation_command_result_receipt_audit_trail_immutable_evidence_boundary_report_sha256",
        "source_activation_command_result_receipt_audit_trail_immutable_evidence_report_sha256",
        "source_activation_command_result_receipt_cancellation_supersession_boundary_report_sha256",
        "source_activation_command_result_receipt_cancellation_supersession_report_sha256",
        "source_activation_command_result_receipt_ordering_monotonicity_boundary_report_sha256",
        "source_activation_command_result_receipt_ordering_monotonicity_report_sha256",
        "source_activation_command_result_receipt_replay_idempotency_boundary_report_sha256",
        "source_activation_command_result_receipt_no_persistence_boundary_report_sha256",
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
            source
                .get(key)
                .cloned()
                .unwrap_or_else(|| serde_json::json!("")),
        );
    }
    insert_report_json!("minimum_required_samples", 24);
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_export_query_observability_denial_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready",
        true
    );
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
        "required_activation_command_result_receipt_export_query_observability_surface_count",
        12
    );
    insert_report_json!(
        "ready_activation_command_result_receipt_export_query_observability_surface_count",
        12
    );
    insert_report_json!(
        "side_effect_free_activation_command_result_receipt_export_query_observability_surface_count",
        12
    );
    insert_report_json!(
        "required_activation_command_result_receipt_export_query_observability_fixture_count",
        10
    );
    insert_report_json!(
        "activation_command_result_receipt_export_query_observability_fixture_count",
        10
    );
    insert_report_json!(
        "blocked_activation_command_result_receipt_export_query_observability_fixture_count",
        10
    );
    insert_report_json!(
        "noop_activation_command_result_receipt_export_query_observability_fixture_count",
        10
    );
    insert_report_json!(
        "allowed_activation_command_result_receipt_export_query_observability_fixture_count",
        0
    );
    insert_report_json!(
        "accepted_activation_command_result_receipt_export_query_observability_fixture_count",
        0
    );
    insert_report_json!("activation_command_result_receipt_export_denied_count", 10);
    insert_report_json!("activation_command_result_receipt_query_denied_count", 10);
    insert_report_json!(
        "activation_command_result_receipt_observability_denied_count",
        10
    );
    insert_report_json!(
        "activation_command_result_receipt_export_performed_count",
        0
    );
    insert_report_json!("activation_command_result_receipt_query_performed_count", 0);
    insert_report_json!(
        "activation_command_result_receipt_observability_performed_count",
        0
    );
    insert_report_json!("memory_store_write_performed_count", 0);

    for &key in FALSE_KEYS {
        report.insert(key.to_string(), serde_json::json!(false));
    }
    report.insert(
        "activation_command_result_receipt_export_query_observability_surfaces".to_string(),
        serde_json::json!(EXPORT_QUERY_OBSERVABILITY_SURFACES),
    );
    report.insert(
        "activation_command_result_receipt_export_query_observability_fixtures".to_string(),
        export_query_observability_fixtures,
    );
    report.insert(
        "denied_by_activation_command_result_receipt_export_query_observability".to_string(),
        serde_json::json!(DENIED_BY),
    );
    insert_report_json!(
        "denied_by_activation_command_result_receipt_export_query_observability_count",
        30
    );
    report.insert("allowed_next_actions".to_string(), allowed_next_actions);
    report.insert(
        "side_effects".to_string(),
        serde_json::Value::Object(side_effects),
    );

    serde_json::Value::Object(report)
}

fn hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_report()
-> serde_json::Value {
    const OPERATOR_SUMMARY_BRIEFING_SURFACES: &[&str] = &[
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
        "memory_write_rollback_secret_provider_summary_briefing_denied",
        "external_public_install_restart_active_binary_summary_briefing_denied",
    ];
    const DENIED_BY: &[&str] = &[
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
        "activation_from_summary_briefing_denied",
        "memory_write_summary_briefing_denied",
        "rollback_summary_briefing_denied",
        "secret_material_summary_briefing_denied",
        "provider_prompt_summary_briefing_denied",
        "external_public_install_restart_active_binary_summary_briefing_denied",
    ];
    const FALSE_KEYS: &[&str] = &[
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
        "activation_command_result_receipt_export_recorded",
        "activation_command_result_receipt_export_persisted",
        "activation_command_result_receipt_export_artifact_written",
        "activation_command_result_receipt_export_stream_opened",
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
        "activation_command_result_receipt_retention_policy_recorded",
        "activation_command_result_receipt_expiry_recorded",
        "activation_command_result_receipt_garbage_collection_scan_performed",
        "activation_command_result_receipt_audit_trail_recorded",
        "activation_command_result_receipt_immutable_evidence_recorded",
        "activation_command_result_receipt_cancellation_allowed",
        "activation_command_result_receipt_supersession_allowed",
        "activation_command_result_receipt_ordering_allowed",
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
        "activation_allowed_by_result_receipt_operator_summary",
        "activation_allowed_by_result_receipt_operator_briefing",
        "activation_allowed_by_result_receipt_summary_briefing",
        "activation_allowed_by_result_receipt_export",
        "activation_allowed_by_result_receipt_query",
        "activation_allowed_by_result_receipt_observability",
        "activation_allowed_by_result_receipt_retention",
        "activation_allowed_by_result_receipt_expiry",
        "activation_allowed_by_result_receipt_garbage_collection",
        "activation_allowed_by_result_receipt_audit_trail",
        "activation_allowed_by_result_receipt_immutable_evidence",
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
        "telegram_send_performed",
        "channel_send_performed",
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
        "filesystem_written",
    ];

    fn operator_summary_briefing_fixture(
        id: &str,
        status: &str,
        reason: &str,
        extra: serde_json::Value,
    ) -> serde_json::Value {
        let mut base = serde_json::Map::new();
        macro_rules! insert_fixture_json {
            ($key:literal, $value:expr) => {
                base.insert($key.to_string(), serde_json::json!($value));
            };
        }
        insert_fixture_json!("id", id);
        insert_fixture_json!("fixture_id", id);
        insert_fixture_json!("operator_summary_briefing_status", status);
        insert_fixture_json!("operator_facing_summary_briefing_status", status);
        insert_fixture_json!("reason", reason);
        insert_fixture_json!("source_export_query_observability_present", true);
        insert_fixture_json!("source_export_query_observability_ready", true);
        insert_fixture_json!("receipt_noop_confirmed", true);
        insert_fixture_json!("summary_briefing_noop_confirmed", true);
        for key in [
            "operator_summary_requested",
            "operator_briefing_requested",
            "operator_summary_materialization_requested",
            "operator_briefing_materialization_requested",
            "operator_summary_persistence_requested",
            "operator_summary_filesystem_write_requested",
            "operator_briefing_persistence_requested",
            "operator_briefing_filesystem_write_requested",
            "channel_delivery_requested",
            "telegram_send_requested",
            "activation_from_summary_briefing_requested",
            "memory_write_summary_requested",
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
            base.insert(key.to_string(), serde_json::json!(false));
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
            "receipt_recorded",
            "receipt_persisted",
            "receipt_accepted",
            "receipt_materialized",
            "receipt_filesystem_written",
            "activation_command_result_receipt_recorded",
            "activation_command_result_receipt_persisted",
            "activation_command_result_receipt_accepted",
            "activation_command_result_receipt_materialized",
            "activation_command_result_receipt_filesystem_written",
            "activation_command_result_receipt_ledger_written",
            "activation_command_result_receipt_indexed",
            "activation_command_result_receipt_enqueued",
            "activation_command_result_receipt_delivered",
            "completion_ack_recorded",
            "completion_ack_persisted",
            "completion_ack_accepted",
            "completion_ack_delivered",
            "activation_allowed",
            "live_mutation_execution_performed",
            "memory_write_execution_performed",
            "memory_store_write_performed",
            "memory_store_mutated",
            "rollback_executed",
            "secret_material_read",
            "provider_invoked",
            "model_invoked",
            "credential_read",
            "secret_file_read",
            "public_release_published",
            "release_artifact_written",
            "install_executed",
            "launchd_mutated",
            "service_restarted",
            "active_binary_mutated",
        ] {
            base.insert(key.to_string(), serde_json::json!(false));
        }
        if let Some(extra) = extra.as_object() {
            for (key, value) in extra {
                base.insert(key.clone(), value.clone());
            }
        }
        serde_json::Value::Object(base)
    }

    let route_matrix = control_ui_route_parity_report();
    let source = std::thread::Builder::new()
        .name("hepta-memory-write-result-receipt-export-query-observability-source-report".to_string())
        .stack_size(8 * 1024 * 1024)
        .spawn(
            hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_export_query_observability_denial_boundary_report,
        )
        .ok()
        .and_then(|handle| handle.join().ok())
        .unwrap_or_else(|| {
            serde_json::json!({
                "status": "blocked",
                "memory_write_execution_activation_command_result_receipt_export_query_observability_denial_boundary_ready": false,
                "memory_write_execution_activation_command_result_receipt_export_query_observability_denial_ready": false,
                "source_export_query_observability_source_report_thread_failed": true
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
    let source_ready = json_str(&source, "status") == "ready"
        && json_bool(
            &source,
            "memory_write_execution_activation_command_result_receipt_export_query_observability_denial_boundary_ready",
        )
        && json_bool(
            &source,
            "memory_write_execution_activation_command_result_receipt_export_query_observability_denial_ready",
        )
        && json_bool(
            &source,
            "memory_write_execution_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready",
        )
        && json_u64(
            &source,
            "activation_command_result_receipt_export_query_observability_fixture_count",
        ) == 10
        && json_u64(
            &source,
            "blocked_activation_command_result_receipt_export_query_observability_fixture_count",
        ) == 10
        && json_u64(
            &source,
            "accepted_activation_command_result_receipt_export_query_observability_fixture_count",
        ) == 0
        && json_u64(
            &source,
            "activation_command_result_receipt_export_performed_count",
        ) == 0
        && json_u64(
            &source,
            "activation_command_result_receipt_query_performed_count",
        ) == 0
        && json_u64(
            &source,
            "activation_command_result_receipt_observability_performed_count",
        ) == 0
        && json_u64(
            &source,
            "denied_by_activation_command_result_receipt_export_query_observability_count",
        ) == 30
        && !json_bool(
            &source,
            "activation_command_result_receipt_export_artifact_written",
        )
        && !json_bool(
            &source,
            "activation_command_result_receipt_export_stream_opened",
        )
        && !json_bool(
            &source,
            "activation_command_result_receipt_query_endpoint_materialized",
        )
        && !json_bool(
            &source,
            "activation_command_result_receipt_query_index_recorded",
        )
        && !json_bool(
            &source,
            "activation_command_result_receipt_observability_metric_emitted",
        )
        && !json_bool(&source, "activation_command_result_receipt_recorded")
        && !json_bool(&source, "activation_command_result_receipt_persisted")
        && !json_bool(&source, "activation_allowed_by_result_receipt")
        && !json_bool(&source, "activation_allowed")
        && !json_bool(&source, "live_mutation_execution_performed")
        && !json_bool(&source, "memory_store_write_performed")
        && !json_bool(&source, "memory_store_mutated")
        && !json_bool(&source, "rollback_executed")
        && !json_bool(&source, "provider_invoked")
        && !json_bool(&source, "model_invoked")
        && !json_bool(&source, "external_send_performed")
        && !json_bool(&source, "release_artifact_written")
        && !json_bool(&source, "active_binary_mutated")
        && side_effects_all_false(&source);

    let operator_summary_briefing_fixtures = serde_json::Value::Array(vec![
        operator_summary_briefing_fixture(
            "activation-result-receipt-operator-summary-briefing-missing-source-export-query-observability",
            "blocked_noop",
            "source_export_query_observability_report_required",
            serde_json::json!({
                "source_export_query_observability_present": false,
                "source_export_query_observability_ready": false,
                "operator_summary_requested": true
            }),
        ),
        operator_summary_briefing_fixture(
            "activation-result-receipt-operator-summary-request",
            "blocked_summary_noop",
            "operator_summary_request_shape_denied",
            serde_json::json!({"operator_summary_requested": true}),
        ),
        operator_summary_briefing_fixture(
            "activation-result-receipt-operator-briefing-request",
            "blocked_briefing_noop",
            "operator_briefing_request_shape_denied",
            serde_json::json!({"operator_briefing_requested": true}),
        ),
        operator_summary_briefing_fixture(
            "activation-result-receipt-operator-summary-materialization-request",
            "blocked_summary_noop",
            "summary_materialization_denied",
            serde_json::json!({
                "operator_summary_requested": true,
                "operator_summary_materialization_requested": true
            }),
        ),
        operator_summary_briefing_fixture(
            "activation-result-receipt-operator-briefing-materialization-request",
            "blocked_briefing_noop",
            "briefing_materialization_denied",
            serde_json::json!({
                "operator_briefing_requested": true,
                "operator_briefing_materialization_requested": true
            }),
        ),
        operator_summary_briefing_fixture(
            "activation-result-receipt-operator-summary-persistence-filesystem-write-request",
            "blocked_summary_noop",
            "summary_persistence_filesystem_write_denied",
            serde_json::json!({
                "operator_summary_requested": true,
                "operator_summary_persistence_requested": true,
                "operator_summary_filesystem_write_requested": true
            }),
        ),
        operator_summary_briefing_fixture(
            "activation-result-receipt-operator-briefing-persistence-filesystem-write-request",
            "blocked_briefing_noop",
            "briefing_persistence_filesystem_write_denied",
            serde_json::json!({
                "operator_briefing_requested": true,
                "operator_briefing_persistence_requested": true,
                "operator_briefing_filesystem_write_requested": true
            }),
        ),
        operator_summary_briefing_fixture(
            "activation-result-receipt-operator-summary-briefing-channel-delivery-request",
            "blocked_delivery_noop",
            "summary_briefing_channel_delivery_denied",
            serde_json::json!({
                "operator_summary_requested": true,
                "operator_briefing_requested": true,
                "channel_delivery_requested": true,
                "telegram_send_requested": true
            }),
        ),
        operator_summary_briefing_fixture(
            "activation-result-receipt-operator-summary-briefing-activation-memory-provider-request",
            "blocked_summary_noop",
            "activation_memory_rollback_secret_provider_summary_briefing_denied",
            serde_json::json!({
                "operator_summary_requested": true,
                "operator_briefing_requested": true,
                "activation_from_summary_briefing_requested": true,
                "memory_write_summary_requested": true,
                "rollback_summary_requested": true,
                "secret_material_summary_requested": true,
                "provider_prompt_summary_requested": true
            }),
        ),
        operator_summary_briefing_fixture(
            "activation-result-receipt-operator-summary-briefing-external-public-install-request",
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

    let source_report_sha256 = sha256_json_value(&source);
    let fixtures_hash_sha256 = sha256_json_value(&operator_summary_briefing_fixtures);
    let boundary_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-operator-facing-summary-briefing-non-persistence-denial-boundary-v1:{}:{}:{}",
        route_matrix.route_count, source_report_sha256, fixtures_hash_sha256,
    ));
    let report_ready = route_matrix.ready
        && route_count_source_command_accepted
        && source_ready
        && OPERATOR_SUMMARY_BRIEFING_SURFACES.len() == 12
        && operator_summary_briefing_fixtures
            .as_array()
            .map(std::vec::Vec::len)
            == Some(10)
        && DENIED_BY.len() == 20;

    let allowed_next_actions = serde_json::json!([
        {
            "action": "run_memory_write_execution_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_require_live_gate",
            "status": "allowed_verification_only",
            "records_operator_summary": false,
            "persists_operator_summary": false,
            "materializes_operator_summary": false,
            "writes_operator_summary_filesystem": false,
            "records_operator_briefing": false,
            "persists_operator_briefing": false,
            "materializes_operator_briefing": false,
            "writes_operator_briefing_filesystem": false,
            "delivers_notification": false,
            "sends_telegram": false,
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
            "action": "prepare_memory_write_execution_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary",
            "status": "allowed_report_only_next_slice",
            "accepts_operator_acknowledgement": false,
            "persists_acknowledgement": false,
            "mutates_runtime": false,
            "invokes_model": false,
            "writes_memory_or_kg": false
        }
    ]);

    let mut side_effects = serde_json::Map::new();
    for &key in FALSE_KEYS {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }

    let mut report = source
        .as_object()
        .cloned()
        .unwrap_or_else(serde_json::Map::new);
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
        "hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_route"
    );
    insert_report_json!(
        "endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_COMMAND_RESULT_RECEIPT_OPERATOR_FACING_SUMMARY_BRIEFING_NON_PERSISTENCE_DENIAL_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_command",
        "/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-operator-facing-summary-briefing-non-persistence-denial-boundary --json"
    );
    insert_report_json!("native_route", true);
    insert_report_json!("side_effect_free", true);
    insert_report_json!("audit_date", "2026-07-03");
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_schema_version",
        "memory_write_execution_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_v1"
    );
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_ready",
        report_ready
    );
    insert_report_json!(
        "activation_command_result_receipt_operator_facing_summary_briefing_mode",
        "memory_write_execution_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial"
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
        "operator_summary_briefing_fixtures_hash_sha256",
        fixtures_hash_sha256
    );
    insert_report_json!(
        "source_activation_command_result_receipt_export_query_observability_boundary_endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_COMMAND_RESULT_RECEIPT_EXPORT_QUERY_OBSERVABILITY_DENIAL_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_activation_command_result_receipt_export_query_observability_boundary_ready",
        source_ready
    );
    insert_report_json!(
        "source_activation_command_result_receipt_export_query_observability_ready",
        json_bool(
            &source,
            "memory_write_execution_activation_command_result_receipt_export_query_observability_denial_ready"
        )
    );
    insert_report_json!(
        "source_activation_command_result_receipt_export_query_observability_boundary_report_sha256",
        source_report_sha256
    );
    insert_report_json!(
        "source_activation_command_result_receipt_export_query_observability_report_sha256",
        source_report_sha256
    );
    insert_report_json!("minimum_required_samples", 24);
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_export_query_observability_denial_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready",
        true
    );
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
        "required_activation_command_result_receipt_operator_facing_summary_briefing_surface_count",
        12
    );
    insert_report_json!(
        "ready_activation_command_result_receipt_operator_facing_summary_briefing_surface_count",
        12
    );
    insert_report_json!(
        "side_effect_free_activation_command_result_receipt_operator_facing_summary_briefing_surface_count",
        12
    );
    insert_report_json!(
        "required_activation_command_result_receipt_operator_facing_summary_briefing_fixture_count",
        10
    );
    insert_report_json!(
        "activation_command_result_receipt_operator_facing_summary_briefing_fixture_count",
        10
    );
    insert_report_json!(
        "blocked_activation_command_result_receipt_operator_facing_summary_briefing_fixture_count",
        10
    );
    insert_report_json!(
        "noop_activation_command_result_receipt_operator_facing_summary_briefing_fixture_count",
        10
    );
    insert_report_json!(
        "allowed_activation_command_result_receipt_operator_facing_summary_briefing_fixture_count",
        0
    );
    insert_report_json!(
        "accepted_activation_command_result_receipt_operator_facing_summary_briefing_fixture_count",
        0
    );
    insert_report_json!("operator_facing_summary_briefing_surface_count", 12);
    insert_report_json!("operator_facing_summary_briefing_fixture_count", 10);
    insert_report_json!("blocked_operator_facing_summary_briefing_fixture_count", 10);
    insert_report_json!("accepted_operator_facing_summary_briefing_fixture_count", 0);
    insert_report_json!(
        "activation_command_result_receipt_operator_summary_denied_count",
        10
    );
    insert_report_json!(
        "activation_command_result_receipt_operator_briefing_denied_count",
        10
    );
    insert_report_json!("operator_summary_denied_count", 10);
    insert_report_json!("operator_briefing_denied_count", 10);
    insert_report_json!(
        "activation_command_result_receipt_operator_summary_performed_count",
        0
    );
    insert_report_json!(
        "activation_command_result_receipt_operator_briefing_performed_count",
        0
    );
    insert_report_json!("operator_summary_performed_count", 0);
    insert_report_json!("operator_briefing_performed_count", 0);
    insert_report_json!("operator_summary_briefing_delivery_performed_count", 0);
    insert_report_json!("operator_summary_recorded_count", 0);
    insert_report_json!("operator_summary_persisted_count", 0);
    insert_report_json!("operator_briefing_recorded_count", 0);
    insert_report_json!("operator_briefing_persisted_count", 0);
    insert_report_json!("operator_summary_delivered_count", 0);
    insert_report_json!("operator_briefing_delivered_count", 0);
    insert_report_json!("operator_summary_briefing_channel_delivery_count", 0);
    insert_report_json!("operator_summary_briefing_external_send_count", 0);
    insert_report_json!("operator_summary_briefing_telegram_send_count", 0);
    insert_report_json!(
        "operator_summary_briefing_activation_authority_derived_count",
        0
    );
    insert_report_json!("operator_summary_briefing_live_execution_allowed_count", 0);
    insert_report_json!("memory_store_write_performed_count", 0);

    for &key in FALSE_KEYS {
        report.insert(key.to_string(), serde_json::json!(false));
    }
    report.insert(
        "activation_command_result_receipt_operator_facing_summary_briefing_surfaces".to_string(),
        serde_json::json!(OPERATOR_SUMMARY_BRIEFING_SURFACES),
    );
    report.insert(
        "activation_command_result_receipt_operator_facing_summary_briefing_fixtures".to_string(),
        operator_summary_briefing_fixtures.clone(),
    );
    report.insert(
        "operator_facing_summary_briefing_fixtures".to_string(),
        operator_summary_briefing_fixtures,
    );
    report.insert(
        "denied_by_activation_command_result_receipt_operator_facing_summary_briefing".to_string(),
        serde_json::json!(DENIED_BY),
    );
    report.insert(
        "denied_by_operator_facing_summary_briefing".to_string(),
        serde_json::json!(DENIED_BY),
    );
    insert_report_json!(
        "denied_by_activation_command_result_receipt_operator_facing_summary_briefing_count",
        20
    );
    insert_report_json!("denied_by_operator_facing_summary_briefing_count", 20);
    report.insert("allowed_next_actions".to_string(), allowed_next_actions);
    report.insert(
        "side_effects".to_string(),
        serde_json::Value::Object(side_effects),
    );

    serde_json::Value::Object(report)
}

fn hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_report()
-> serde_json::Value {
    const FINAL_OPERATOR_ACKNOWLEDGEMENT_SURFACES: &[&str] = &[
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
        "memory_rollback_secret_provider_acknowledgement_denied",
        "external_public_install_restart_active_binary_acknowledgement_denied",
    ];
    const FINAL_ACKNOWLEDGEMENT_DENIALS: &[&str] = &[
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
        "memory_write_acknowledgement_denied",
        "rollback_acknowledgement_denied",
        "secret_material_acknowledgement_denied",
        "provider_prompt_acknowledgement_denied",
        "external_public_install_restart_active_binary_acknowledgement_denied",
    ];
    const FALSE_KEYS: &[&str] = &[
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
        "activation_allowed_by_result_receipt_final_operator_acknowledgement",
        "activation_allowed_by_result_receipt_summary_briefing",
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
        "external_send_enabled",
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
        "filesystem_written",
    ];

    fn final_operator_acknowledgement_fixture(
        id: &str,
        status: &str,
        reason: &str,
        extra: serde_json::Value,
    ) -> serde_json::Value {
        let mut base = serde_json::Map::new();
        macro_rules! insert_fixture_json {
            ($key:literal, $value:expr) => {
                base.insert($key.to_string(), serde_json::json!($value));
            };
        }
        insert_fixture_json!("id", id);
        insert_fixture_json!("fixture_id", id);
        insert_fixture_json!("final_operator_acknowledgement_requested", false);
        insert_fixture_json!("final_operator_acknowledgement_status", status);
        insert_fixture_json!("source_summary_briefing_present", true);
        insert_fixture_json!("source_summary_briefing_ready", true);
        insert_fixture_json!("receipt_noop_confirmed", true);
        insert_fixture_json!("final_acknowledgement_noop_confirmed", true);
        insert_fixture_json!("reason", reason);
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
            "memory_write_acknowledgement_requested",
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
            base.insert(key.to_string(), serde_json::json!(false));
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
            "rollback_executed",
            "secret_material_read",
            "provider_invoked",
            "model_invoked",
            "credential_read",
            "secret_file_read",
            "public_release_published",
            "release_artifact_written",
            "install_executed",
            "launchd_mutated",
            "service_restarted",
            "active_binary_mutated",
        ] {
            base.insert(key.to_string(), serde_json::json!(false));
        }
        if let Some(extra) = extra.as_object() {
            for (key, value) in extra {
                base.insert(key.clone(), value.clone());
            }
        }
        serde_json::Value::Object(base)
    }

    let route_matrix = control_ui_route_parity_report();
    let source = std::thread::Builder::new()
        .name(
            "hepta-memory-write-result-receipt-operator-summary-briefing-source-report"
                .to_string(),
        )
        .stack_size(8 * 1024 * 1024)
        .spawn(
            hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_report,
        )
        .ok()
        .and_then(|handle| handle.join().ok())
        .unwrap_or_else(|| {
            serde_json::json!({
                "status": "blocked",
                "memory_write_execution_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_ready": false,
                "memory_write_execution_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready": false,
                "source_operator_summary_briefing_source_report_thread_failed": true
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
    let source_ready = json_str(&source, "status") == "ready"
        && json_bool(
            &source,
            "memory_write_execution_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_ready",
        )
        && json_bool(
            &source,
            "memory_write_execution_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready",
        )
        && json_bool(
            &source,
            "memory_write_execution_activation_command_result_receipt_export_query_observability_denial_ready",
        )
        && json_u64(
            &source,
            "activation_command_result_receipt_operator_facing_summary_briefing_fixture_count",
        ) == 10
        && json_u64(
            &source,
            "blocked_activation_command_result_receipt_operator_facing_summary_briefing_fixture_count",
        ) == 10
        && json_u64(
            &source,
            "accepted_activation_command_result_receipt_operator_facing_summary_briefing_fixture_count",
        ) == 0
        && json_u64(
            &source,
            "activation_command_result_receipt_operator_summary_performed_count",
        ) == 0
        && json_u64(
            &source,
            "activation_command_result_receipt_operator_briefing_performed_count",
        ) == 0
        && json_u64(
            &source,
            "denied_by_activation_command_result_receipt_operator_facing_summary_briefing_count",
        ) == 20
        && !json_bool(
            &source,
            "activation_command_result_receipt_operator_summary_recorded",
        )
        && !json_bool(
            &source,
            "activation_command_result_receipt_operator_summary_persisted",
        )
        && !json_bool(
            &source,
            "activation_command_result_receipt_operator_summary_delivered",
        )
        && !json_bool(
            &source,
            "activation_command_result_receipt_operator_briefing_recorded",
        )
        && !json_bool(
            &source,
            "activation_command_result_receipt_operator_briefing_persisted",
        )
        && !json_bool(
            &source,
            "activation_command_result_receipt_operator_briefing_delivered",
        )
        && !json_bool(
            &source,
            "activation_command_result_receipt_operator_summary_briefing_channel_delivery_performed",
        )
        && !json_bool(&source, "telegram_send_performed")
        && !json_bool(&source, "channel_send_performed")
        && !json_bool(&source, "external_send_performed")
        && !json_bool(&source, "activation_command_result_receipt_recorded")
        && !json_bool(&source, "activation_command_result_receipt_persisted")
        && !json_bool(&source, "activation_command_result_receipt_accepted")
        && !json_bool(
            &source,
            "activation_allowed_by_result_receipt_summary_briefing",
        )
        && !json_bool(&source, "activation_allowed_by_result_receipt")
        && !json_bool(&source, "activation_allowed")
        && !json_bool(&source, "live_mutation_execution_performed")
        && !json_bool(&source, "memory_store_write_performed")
        && !json_bool(&source, "memory_store_mutated")
        && !json_bool(&source, "rollback_executed")
        && !json_bool(&source, "provider_invoked")
        && !json_bool(&source, "model_invoked")
        && !json_bool(&source, "external_send_performed")
        && !json_bool(&source, "release_artifact_written")
        && !json_bool(&source, "active_binary_mutated")
        && side_effects_all_false(&source);

    let final_acknowledgement_fixtures = serde_json::Value::Array(vec![
        final_operator_acknowledgement_fixture(
            "activation-result-receipt-final-ack-missing-source-summary-briefing",
            "blocked_noop",
            "source_summary_briefing_report_required",
            serde_json::json!({
                "source_summary_briefing_present": false,
                "source_summary_briefing_ready": false,
                "final_operator_acknowledgement_requested": true
            }),
        ),
        final_operator_acknowledgement_fixture(
            "activation-result-receipt-final-ack-request",
            "blocked_ack_noop",
            "final_operator_acknowledgement_request_shape_denied",
            serde_json::json!({"final_operator_acknowledgement_requested": true}),
        ),
        final_operator_acknowledgement_fixture(
            "activation-result-receipt-final-ack-acceptance-request",
            "blocked_acceptance_noop",
            "acknowledgement_acceptance_denied",
            serde_json::json!({
                "final_operator_acknowledgement_requested": true,
                "acknowledgement_acceptance_requested": true
            }),
        ),
        final_operator_acknowledgement_fixture(
            "activation-result-receipt-final-ack-recording-request",
            "blocked_ack_noop",
            "acknowledgement_recording_denied",
            serde_json::json!({
                "final_operator_acknowledgement_requested": true,
                "acknowledgement_recording_requested": true
            }),
        ),
        final_operator_acknowledgement_fixture(
            "activation-result-receipt-final-ack-persistence-filesystem-write-request",
            "blocked_ack_noop",
            "acknowledgement_persistence_filesystem_write_denied",
            serde_json::json!({
                "final_operator_acknowledgement_requested": true,
                "acknowledgement_persistence_requested": true,
                "acknowledgement_filesystem_write_requested": true
            }),
        ),
        final_operator_acknowledgement_fixture(
            "activation-result-receipt-final-ack-identity-signature-timestamp-request",
            "blocked_acceptance_noop",
            "operator_identity_signature_timestamp_acknowledgement_acceptance_denied",
            serde_json::json!({
                "final_operator_acknowledgement_requested": true,
                "operator_identity_acceptance_requested": true,
                "operator_signature_acceptance_requested": true,
                "operator_timestamp_acceptance_requested": true
            }),
        ),
        final_operator_acknowledgement_fixture(
            "activation-result-receipt-final-ack-delivery-request",
            "blocked_delivery_noop",
            "acknowledgement_delivery_denied",
            serde_json::json!({
                "final_operator_acknowledgement_requested": true,
                "acknowledgement_delivery_requested": true,
                "telegram_send_requested": true,
                "channel_delivery_requested": true
            }),
        ),
        final_operator_acknowledgement_fixture(
            "activation-result-receipt-final-state-promotion-request",
            "blocked_promotion_noop",
            "final_state_completion_promotion_denied",
            serde_json::json!({
                "final_operator_acknowledgement_requested": true,
                "final_state_promotion_requested": true,
                "completion_promotion_requested": true
            }),
        ),
        final_operator_acknowledgement_fixture(
            "activation-result-receipt-final-ack-activation-memory-provider-request",
            "blocked_ack_noop",
            "activation_memory_rollback_secret_provider_acknowledgement_denied",
            serde_json::json!({
                "final_operator_acknowledgement_requested": true,
                "activation_from_acknowledgement_requested": true,
                "memory_write_acknowledgement_requested": true,
                "rollback_acknowledgement_requested": true,
                "secret_material_acknowledgement_requested": true,
                "provider_prompt_acknowledgement_requested": true
            }),
        ),
        final_operator_acknowledgement_fixture(
            "activation-result-receipt-final-ack-external-public-install-request",
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
    let fixture_count = final_acknowledgement_fixtures
        .as_array()
        .map(std::vec::Vec::len)
        .unwrap_or(0);
    let source_report_sha256 = sha256_json_value(&source);
    let fixtures_hash_sha256 = sha256_json_value(&final_acknowledgement_fixtures);
    let boundary_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-final-operator-acknowledgement-non-acceptance-denial-boundary-v1:{}:{}:{}",
        route_matrix.route_count, source_report_sha256, fixtures_hash_sha256,
    ));
    let report_ready = route_matrix.ready
        && route_count_source_command_accepted
        && source_ready
        && FINAL_OPERATOR_ACKNOWLEDGEMENT_SURFACES.len() == 12
        && fixture_count == 10
        && FINAL_ACKNOWLEDGEMENT_DENIALS.len() == 17;

    let mut denials = source
        .get("denied_by_activation_command_result_receipt_operator_facing_summary_briefing")
        .and_then(serde_json::Value::as_array)
        .cloned()
        .unwrap_or_default();
    for &denial in FINAL_ACKNOWLEDGEMENT_DENIALS {
        denials.push(serde_json::json!(denial));
    }
    let denied_count = denials.len();

    let mut side_effects = source
        .get("side_effects")
        .and_then(serde_json::Value::as_object)
        .cloned()
        .unwrap_or_default();
    for &key in FALSE_KEYS {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }

    let allowed_next_actions = serde_json::json!([
        {
            "action": "run_memory_write_execution_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_require_live_gate",
            "status": "allowed_verification_only",
            "accepts_operator_acknowledgement": false,
            "records_acknowledgement": false,
            "persists_acknowledgement": false,
            "delivers_acknowledgement": false,
            "promotes_final_state": false,
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
            "action": "prepare_memory_write_execution_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary",
            "status": "allowed_report_only_next_slice",
            "accepts_terminal_decision": false,
            "claims_public_release": false,
            "writes_release_artifact": false,
            "mutates_runtime": false,
            "invokes_model": false,
            "writes_memory_or_kg": false
        }
    ]);

    let mut report = source
        .as_object()
        .cloned()
        .unwrap_or_else(serde_json::Map::new);
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
        "hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_route"
    );
    insert_report_json!(
        "endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_COMMAND_RESULT_RECEIPT_FINAL_OPERATOR_ACKNOWLEDGEMENT_NON_ACCEPTANCE_DENIAL_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_command",
        "/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-final-operator-acknowledgement-non-acceptance-denial-boundary --json"
    );
    insert_report_json!("native_route", true);
    insert_report_json!("side_effect_free", true);
    insert_report_json!("audit_date", "2026-07-03");
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_schema_version",
        "memory_write_execution_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_v1"
    );
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_ready",
        report_ready
    );
    insert_report_json!(
        "activation_command_result_receipt_final_operator_acknowledgement_mode",
        "memory_write_execution_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial"
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
        "final_operator_acknowledgement_fixtures_hash_sha256",
        fixtures_hash_sha256
    );
    insert_report_json!(
        "source_activation_command_result_receipt_operator_facing_summary_briefing_boundary_endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_COMMAND_RESULT_RECEIPT_OPERATOR_FACING_SUMMARY_BRIEFING_NON_PERSISTENCE_DENIAL_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_activation_command_result_receipt_operator_facing_summary_briefing_boundary_ready",
        source_ready
    );
    insert_report_json!(
        "source_activation_command_result_receipt_operator_facing_summary_briefing_ready",
        json_bool(
            &source,
            "memory_write_execution_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready"
        )
    );
    insert_report_json!(
        "source_activation_command_result_receipt_operator_facing_summary_briefing_boundary_report_sha256",
        source_report_sha256
    );
    insert_report_json!(
        "source_activation_command_result_receipt_operator_facing_summary_briefing_report_sha256",
        source_report_sha256
    );
    insert_report_json!(
        "source_operator_facing_summary_briefing_fixture_count",
        json_u64(
            &source,
            "activation_command_result_receipt_operator_facing_summary_briefing_fixture_count"
        )
    );
    insert_report_json!(
        "source_accepted_operator_facing_summary_briefing_fixture_count",
        json_u64(
            &source,
            "accepted_activation_command_result_receipt_operator_facing_summary_briefing_fixture_count"
        )
    );
    insert_report_json!(
        "source_operator_summary_performed_count",
        json_u64(
            &source,
            "activation_command_result_receipt_operator_summary_performed_count"
        )
    );
    insert_report_json!(
        "source_operator_briefing_performed_count",
        json_u64(
            &source,
            "activation_command_result_receipt_operator_briefing_performed_count"
        )
    );
    insert_report_json!("minimum_required_samples", 24);
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_export_query_observability_denial_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready",
        true
    );
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
        "required_activation_command_result_receipt_final_operator_acknowledgement_surface_count",
        12
    );
    insert_report_json!(
        "ready_activation_command_result_receipt_final_operator_acknowledgement_surface_count",
        12
    );
    insert_report_json!(
        "side_effect_free_activation_command_result_receipt_final_operator_acknowledgement_surface_count",
        12
    );
    insert_report_json!(
        "required_activation_command_result_receipt_final_operator_acknowledgement_fixture_count",
        10
    );
    insert_report_json!(
        "activation_command_result_receipt_final_operator_acknowledgement_fixture_count",
        fixture_count
    );
    insert_report_json!(
        "blocked_activation_command_result_receipt_final_operator_acknowledgement_fixture_count",
        fixture_count
    );
    insert_report_json!(
        "noop_activation_command_result_receipt_final_operator_acknowledgement_fixture_count",
        fixture_count
    );
    insert_report_json!(
        "allowed_activation_command_result_receipt_final_operator_acknowledgement_fixture_count",
        0
    );
    insert_report_json!(
        "accepted_activation_command_result_receipt_final_operator_acknowledgement_fixture_count",
        0
    );
    insert_report_json!(
        "activation_command_result_receipt_final_operator_acknowledgement_denied_count",
        fixture_count
    );
    insert_report_json!(
        "activation_command_result_receipt_final_operator_acknowledgement_performed_count",
        0
    );
    insert_report_json!(
        "activation_command_result_receipt_final_operator_acknowledgement_recorded_count",
        0
    );
    insert_report_json!(
        "activation_command_result_receipt_final_operator_acknowledgement_persisted_count",
        0
    );
    insert_report_json!(
        "activation_command_result_receipt_final_operator_acknowledgement_delivered_count",
        0
    );
    insert_report_json!(
        "activation_command_result_receipt_final_operator_acknowledgement_final_state_promoted_count",
        0
    );
    insert_report_json!("memory_store_write_performed_count", 0);
    insert_report_json!(
        "source_operator_facing_summary_briefing_denial_count",
        json_u64(
            &source,
            "denied_by_activation_command_result_receipt_operator_facing_summary_briefing_count"
        )
    );

    for &key in FALSE_KEYS {
        report.insert(key.to_string(), serde_json::json!(false));
    }
    report.insert(
        "activation_command_result_receipt_final_operator_acknowledgement_surfaces".to_string(),
        serde_json::json!(FINAL_OPERATOR_ACKNOWLEDGEMENT_SURFACES),
    );
    report.insert(
        "activation_command_result_receipt_final_operator_acknowledgement_fixtures".to_string(),
        final_acknowledgement_fixtures,
    );
    report.insert(
        "denied_by_activation_command_result_receipt_final_operator_acknowledgement".to_string(),
        serde_json::Value::Array(denials),
    );
    insert_report_json!(
        "denied_by_activation_command_result_receipt_final_operator_acknowledgement_count",
        denied_count
    );
    report.insert("allowed_next_actions".to_string(), allowed_next_actions);
    report.insert(
        "side_effects".to_string(),
        serde_json::Value::Object(side_effects),
    );

    serde_json::Value::Object(report)
}

fn hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_report()
-> serde_json::Value {
    const TERMINAL_DECISION_PUBLIC_CLAIM_SURFACES: &[&str] = &[
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
        "external_public_install_restart_active_binary_terminal_decision_denied",
    ];
    const TERMINAL_DECISION_PUBLIC_CLAIM_DENIALS: &[&str] = &[
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
    ];
    const FALSE_KEYS: &[&str] = &[
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
        "activation_allowed_by_result_receipt_terminal_operator_decision",
        "activation_allowed_by_result_receipt_final_operator_acknowledgement",
        "activation_allowed_by_result_receipt",
        "activation_allowed",
        "activation_performed",
        "activation_activated",
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
        "auth_secret_read",
        "credential_read",
        "secret_file_read",
        "provider_prompt_replay_enabled",
        "provider_invoked",
        "model_invoked",
        "telegram_send_performed",
        "channel_send_performed",
        "external_send_performed",
        "external_send_enabled",
        "public_claim_or_release_artifact_write_enabled",
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
        "filesystem_written",
    ];

    fn terminal_decision_fixture(
        id: &str,
        status: &str,
        reason: &str,
        extra: serde_json::Value,
    ) -> serde_json::Value {
        let mut base = serde_json::Map::new();
        macro_rules! insert_fixture_json {
            ($key:literal, $value:expr) => {
                base.insert($key.to_string(), serde_json::json!($value));
            };
        }
        insert_fixture_json!("id", id);
        insert_fixture_json!("fixture_id", id);
        insert_fixture_json!("terminal_operator_decision_requested", false);
        insert_fixture_json!("terminal_operator_decision_status", status);
        insert_fixture_json!("source_final_acknowledgement_present", true);
        insert_fixture_json!("source_final_acknowledgement_ready", true);
        insert_fixture_json!("receipt_noop_confirmed", true);
        insert_fixture_json!("terminal_operator_decision_noop_confirmed", true);
        insert_fixture_json!("reason", reason);
        for key in [
            "terminal_decision_acceptance_requested",
            "terminal_decision_recording_requested",
            "terminal_decision_persistence_requested",
            "terminal_decision_filesystem_write_requested",
            "terminal_decision_delivery_requested",
            "operator_identity_acceptance_requested",
            "operator_signature_acceptance_requested",
            "operator_timestamp_acceptance_requested",
            "public_claim_promotion_requested",
            "public_ga_claim_requested",
            "public_release_publish_requested",
            "public_distribution_requested",
            "release_artifact_write_requested",
            "activation_from_terminal_decision_requested",
            "memory_write_terminal_decision_requested",
            "rollback_terminal_decision_requested",
            "secret_material_terminal_decision_requested",
            "provider_prompt_terminal_decision_requested",
            "external_send_decision_requested",
            "public_claim_decision_requested",
            "release_artifact_decision_requested",
            "install_decision_requested",
            "service_restart_decision_requested",
            "active_binary_decision_requested",
        ] {
            base.insert(key.to_string(), serde_json::json!(false));
        }
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
            base.insert(key.to_string(), serde_json::json!(false));
        }
        if let Some(extra) = extra.as_object() {
            for (key, value) in extra {
                base.insert(key.clone(), value.clone());
            }
        }
        serde_json::Value::Object(base)
    }

    let route_matrix = control_ui_route_parity_report();
    let source = std::thread::Builder::new()
        .name(
            "hepta-memory-write-result-receipt-final-operator-ack-source-report".to_string(),
        )
        .stack_size(8 * 1024 * 1024)
        .spawn(
            hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_report,
        )
        .ok()
        .and_then(|handle| handle.join().ok())
        .unwrap_or_else(|| {
            serde_json::json!({
                "status": "blocked",
                "memory_write_execution_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_ready": false,
                "memory_write_execution_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready": false,
                "source_final_operator_acknowledgement_source_report_thread_failed": true
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
    let source_next_action_terminal_decision = source
        .get("allowed_next_actions")
        .and_then(serde_json::Value::as_array)
        .and_then(|items| items.get(1))
        .map(|item| {
            item.get("action").and_then(serde_json::Value::as_str)
                == Some(
                    "prepare_memory_write_execution_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary",
                )
                && item
                    .get("accepts_terminal_decision")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && item
                    .get("claims_public_release")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && item
                    .get("writes_release_artifact")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
        })
        .unwrap_or(false);
    let source_ready = json_str(&source, "status") == "ready"
        && json_bool(
            &source,
            "memory_write_execution_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_ready",
        )
        && json_bool(
            &source,
            "memory_write_execution_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready",
        )
        && json_u64(
            &source,
            "activation_command_result_receipt_final_operator_acknowledgement_fixture_count",
        ) == 10
        && json_u64(
            &source,
            "blocked_activation_command_result_receipt_final_operator_acknowledgement_fixture_count",
        ) == 10
        && json_u64(
            &source,
            "accepted_activation_command_result_receipt_final_operator_acknowledgement_fixture_count",
        ) == 0
        && json_u64(
            &source,
            "activation_command_result_receipt_final_operator_acknowledgement_performed_count",
        ) == 0
        && json_u64(
            &source,
            "denied_by_activation_command_result_receipt_final_operator_acknowledgement_count",
        ) == 37
        && !json_bool(
            &source,
            "activation_command_result_receipt_final_operator_acknowledgement_accepted",
        )
        && !json_bool(
            &source,
            "activation_command_result_receipt_final_operator_acknowledgement_recorded",
        )
        && !json_bool(
            &source,
            "activation_command_result_receipt_final_operator_acknowledgement_persisted",
        )
        && !json_bool(
            &source,
            "activation_command_result_receipt_final_operator_acknowledgement_delivered",
        )
        && !json_bool(
            &source,
            "activation_command_result_receipt_final_operator_acknowledgement_final_state_promoted",
        )
        && !json_bool(
            &source,
            "activation_allowed_by_result_receipt_final_operator_acknowledgement",
        )
        && !json_bool(&source, "activation_allowed_by_result_receipt")
        && !json_bool(&source, "activation_allowed")
        && !json_bool(&source, "activation_performed")
        && !json_bool(&source, "memory_store_write_performed")
        && !json_bool(&source, "memory_store_mutated")
        && !json_bool(&source, "rollback_executed")
        && !json_bool(&source, "provider_invoked")
        && !json_bool(&source, "model_invoked")
        && !json_bool(&source, "credential_read")
        && !json_bool(&source, "secret_file_read")
        && !json_bool(&source, "live_kg_write_performed")
        && !json_bool(&source, "telegram_send_performed")
        && !json_bool(&source, "channel_send_performed")
        && !json_bool(&source, "external_send_performed")
        && !json_bool(&source, "public_release_published")
        && !json_bool(&source, "release_artifact_written")
        && !json_bool(&source, "install_executed")
        && !json_bool(&source, "service_restarted")
        && !json_bool(&source, "active_binary_mutated")
        && source_next_action_terminal_decision
        && side_effects_all_false(&source);

    let terminal_decision_public_claim_fixtures = serde_json::Value::Array(vec![
        terminal_decision_fixture(
            "activation-result-receipt-terminal-decision-missing-final-ack",
            "blocked_noop",
            "source_final_operator_acknowledgement_report_required",
            serde_json::json!({
                "source_final_acknowledgement_present": false,
                "source_final_acknowledgement_ready": false,
                "terminal_operator_decision_requested": true
            }),
        ),
        terminal_decision_fixture(
            "activation-result-receipt-terminal-decision-request",
            "blocked_decision_noop",
            "terminal_operator_decision_request_shape_denied",
            serde_json::json!({"terminal_operator_decision_requested": true}),
        ),
        terminal_decision_fixture(
            "activation-result-receipt-terminal-decision-acceptance-request",
            "blocked_acceptance_noop",
            "terminal_operator_decision_acceptance_denied",
            serde_json::json!({
                "terminal_operator_decision_requested": true,
                "terminal_decision_acceptance_requested": true
            }),
        ),
        terminal_decision_fixture(
            "activation-result-receipt-terminal-decision-recording-request",
            "blocked_decision_noop",
            "terminal_operator_decision_recording_denied",
            serde_json::json!({
                "terminal_operator_decision_requested": true,
                "terminal_decision_recording_requested": true
            }),
        ),
        terminal_decision_fixture(
            "activation-result-receipt-terminal-decision-persistence-filesystem-write-request",
            "blocked_decision_noop",
            "terminal_operator_decision_persistence_filesystem_write_denied",
            serde_json::json!({
                "terminal_operator_decision_requested": true,
                "terminal_decision_persistence_requested": true,
                "terminal_decision_filesystem_write_requested": true
            }),
        ),
        terminal_decision_fixture(
            "activation-result-receipt-terminal-decision-identity-signature-request",
            "blocked_acceptance_noop",
            "operator_identity_signature_terminal_decision_acceptance_denied",
            serde_json::json!({
                "terminal_operator_decision_requested": true,
                "operator_identity_acceptance_requested": true,
                "operator_signature_acceptance_requested": true,
                "operator_timestamp_acceptance_requested": true
            }),
        ),
        terminal_decision_fixture(
            "activation-result-receipt-terminal-decision-public-claim-request",
            "blocked_public_claim_noop",
            "public_claim_request_non_promotion_denied",
            serde_json::json!({
                "terminal_operator_decision_requested": true,
                "public_claim_requested": true,
                "public_claim_promotion_requested": true
            }),
        ),
        terminal_decision_fixture(
            "activation-result-receipt-terminal-decision-public-ga-release-request",
            "blocked_promotion_noop",
            "public_ga_release_publication_promotion_denied",
            serde_json::json!({
                "terminal_operator_decision_requested": true,
                "public_ga_claim_requested": true,
                "public_release_publish_requested": true,
                "public_distribution_requested": true,
                "release_artifact_write_requested": true
            }),
        ),
        terminal_decision_fixture(
            "activation-result-receipt-terminal-decision-activation-memory-provider-request",
            "blocked_decision_noop",
            "activation_memory_rollback_secret_provider_terminal_decision_denied",
            serde_json::json!({
                "terminal_operator_decision_requested": true,
                "activation_from_terminal_decision_requested": true,
                "memory_write_terminal_decision_requested": true,
                "rollback_terminal_decision_requested": true,
                "secret_material_terminal_decision_requested": true,
                "provider_prompt_terminal_decision_requested": true
            }),
        ),
        terminal_decision_fixture(
            "activation-result-receipt-terminal-decision-external-public-install-request",
            "blocked_promotion_noop",
            "external_public_install_restart_active_binary_terminal_decision_denied",
            serde_json::json!({
                "terminal_operator_decision_requested": true,
                "external_send_decision_requested": true,
                "public_claim_decision_requested": true,
                "release_artifact_decision_requested": true,
                "install_decision_requested": true,
                "service_restart_decision_requested": true,
                "active_binary_decision_requested": true
            }),
        ),
    ]);
    let fixture_count = terminal_decision_public_claim_fixtures
        .as_array()
        .map(std::vec::Vec::len)
        .unwrap_or(0);
    let source_report_sha256 = sha256_json_value(&source);
    let fixtures_hash_sha256 = sha256_json_value(&terminal_decision_public_claim_fixtures);
    let boundary_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial-boundary-v1:{}:{}:{}",
        route_matrix.route_count, source_report_sha256, fixtures_hash_sha256,
    ));
    let report_ready = route_matrix.ready
        && route_count_source_command_accepted
        && source_ready
        && TERMINAL_DECISION_PUBLIC_CLAIM_SURFACES.len() == 12
        && fixture_count == 10
        && TERMINAL_DECISION_PUBLIC_CLAIM_DENIALS.len() == 18;

    let mut denials = source
        .get("denied_by_activation_command_result_receipt_final_operator_acknowledgement")
        .and_then(serde_json::Value::as_array)
        .cloned()
        .unwrap_or_default();
    for &denial in TERMINAL_DECISION_PUBLIC_CLAIM_DENIALS {
        denials.push(serde_json::json!(denial));
    }
    let denied_count = denials.len();

    let mut side_effects = source
        .get("side_effects")
        .and_then(serde_json::Value::as_object)
        .cloned()
        .unwrap_or_default();
    for &key in FALSE_KEYS {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }

    let allowed_next_actions = serde_json::json!([
        {
            "action": "run_memory_write_execution_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_require_live_gate",
            "status": "allowed_verification_only",
            "accepts_terminal_decision": false,
            "records_terminal_decision": false,
            "persists_terminal_decision": false,
            "delivers_terminal_decision": false,
            "promotes_public_claim": false,
            "claims_public_release": false,
            "writes_release_artifact": false,
            "writes_memory": false,
            "executes_rollback": false,
            "writes_kg": false,
            "invokes_provider": false,
            "reads_credentials": false,
            "sends_externally": false,
            "installs_or_restarts": false,
            "mutates_active_binary": false
        },
        {
            "action": "prepare_memory_write_execution_activation_command_result_receipt_release_artifact_publication_denial_boundary",
            "status": "allowed_report_only_next_slice",
            "publishes_release_artifact": false,
            "claims_public_release": false,
            "writes_release_artifact": false,
            "mutates_runtime": false,
            "invokes_model": false,
            "writes_memory_or_kg": false
        }
    ]);

    let mut report = source
        .as_object()
        .cloned()
        .unwrap_or_else(serde_json::Map::new);
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
        "hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_route"
    );
    insert_report_json!(
        "endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_COMMAND_RESULT_RECEIPT_TERMINAL_OPERATOR_DECISION_PUBLIC_CLAIM_NON_PROMOTION_DENIAL_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_command",
        "/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial-boundary --json"
    );
    insert_report_json!("native_route", true);
    insert_report_json!("side_effect_free", true);
    insert_report_json!("audit_date", "2026-07-03");
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_schema_version",
        "memory_write_execution_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_v1"
    );
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_ready",
        report_ready
    );
    insert_report_json!(
        "activation_command_result_receipt_terminal_operator_decision_public_claim_mode",
        "memory_write_execution_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial"
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
        "terminal_operator_decision_public_claim_fixtures_hash_sha256",
        fixtures_hash_sha256
    );
    insert_report_json!(
        "source_activation_command_result_receipt_final_operator_acknowledgement_boundary_endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_COMMAND_RESULT_RECEIPT_FINAL_OPERATOR_ACKNOWLEDGEMENT_NON_ACCEPTANCE_DENIAL_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_activation_command_result_receipt_final_operator_acknowledgement_boundary_ready",
        source_ready
    );
    insert_report_json!(
        "source_activation_command_result_receipt_final_operator_acknowledgement_ready",
        json_bool(
            &source,
            "memory_write_execution_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready"
        )
    );
    insert_report_json!(
        "source_activation_command_result_receipt_final_operator_acknowledgement_report_sha256",
        source_report_sha256
    );
    insert_report_json!(
        "source_final_operator_acknowledgement_fixture_count",
        json_u64(
            &source,
            "activation_command_result_receipt_final_operator_acknowledgement_fixture_count"
        )
    );
    insert_report_json!(
        "source_blocked_final_operator_acknowledgement_fixture_count",
        json_u64(
            &source,
            "blocked_activation_command_result_receipt_final_operator_acknowledgement_fixture_count"
        )
    );
    insert_report_json!(
        "source_accepted_final_operator_acknowledgement_fixture_count",
        json_u64(
            &source,
            "accepted_activation_command_result_receipt_final_operator_acknowledgement_fixture_count"
        )
    );
    insert_report_json!(
        "source_final_operator_acknowledgement_performed_count",
        json_u64(
            &source,
            "activation_command_result_receipt_final_operator_acknowledgement_performed_count"
        )
    );
    insert_report_json!(
        "source_final_operator_acknowledgement_denial_count",
        json_u64(
            &source,
            "denied_by_activation_command_result_receipt_final_operator_acknowledgement_count"
        )
    );
    insert_report_json!("minimum_required_samples", 24);
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_export_query_observability_denial_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready",
        true
    );
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
        "required_activation_command_result_receipt_terminal_operator_decision_public_claim_surface_count",
        12
    );
    insert_report_json!(
        "ready_activation_command_result_receipt_terminal_operator_decision_public_claim_surface_count",
        12
    );
    insert_report_json!(
        "side_effect_free_activation_command_result_receipt_terminal_operator_decision_public_claim_surface_count",
        12
    );
    insert_report_json!(
        "required_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count",
        10
    );
    insert_report_json!(
        "activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count",
        fixture_count
    );
    insert_report_json!(
        "blocked_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count",
        fixture_count
    );
    insert_report_json!(
        "noop_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count",
        fixture_count
    );
    insert_report_json!(
        "allowed_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count",
        0
    );
    insert_report_json!(
        "accepted_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count",
        0
    );
    insert_report_json!(
        "activation_command_result_receipt_terminal_operator_decision_performed_count",
        0
    );
    insert_report_json!(
        "activation_command_result_receipt_public_claim_promotion_performed_count",
        0
    );
    insert_report_json!("memory_store_write_performed_count", 0);

    for &key in FALSE_KEYS {
        report.insert(key.to_string(), serde_json::json!(false));
    }
    report.insert(
        "activation_command_result_receipt_terminal_operator_decision_public_claim_surfaces"
            .to_string(),
        serde_json::json!(TERMINAL_DECISION_PUBLIC_CLAIM_SURFACES),
    );
    report.insert(
        "activation_command_result_receipt_terminal_operator_decision_public_claim_fixtures"
            .to_string(),
        terminal_decision_public_claim_fixtures,
    );
    report.insert(
        "denied_by_activation_command_result_receipt_terminal_operator_decision_public_claim"
            .to_string(),
        serde_json::Value::Array(denials),
    );
    insert_report_json!(
        "denied_by_activation_command_result_receipt_terminal_operator_decision_public_claim_count",
        denied_count
    );
    insert_report_json!("terminal_operator_decision_acceptance_forbidden", true);
    insert_report_json!("terminal_operator_decision_recording_forbidden", true);
    insert_report_json!("terminal_operator_decision_persistence_forbidden", true);
    insert_report_json!("terminal_operator_decision_delivery_forbidden", true);
    insert_report_json!("terminal_operator_decision_promotion_forbidden", true);
    insert_report_json!("public_claim_promotion_forbidden", true);
    insert_report_json!("public_release_publication_forbidden", true);
    insert_report_json!("release_artifact_publication_forbidden", true);
    insert_report_json!("activation_from_terminal_operator_decision_forbidden", true);
    insert_report_json!(
        "runtime_provider_memory_kg_terminal_operator_decision_forbidden",
        true
    );
    insert_report_json!(
        "external_public_install_restart_active_binary_terminal_operator_decision_forbidden",
        true
    );
    report.insert("allowed_next_actions".to_string(), allowed_next_actions);
    report.insert(
        "side_effects".to_string(),
        serde_json::Value::Object(side_effects),
    );

    serde_json::Value::Object(report)
}
