
fn hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let source_replay =
        hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_report();
    let source_status = source_replay
        .get("operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_status")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("unknown");
    let source_replay_ready = source_status == "blocked"
        && source_replay
            .get("operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_ready")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
        && source_replay
            .get("accepted_replay_idempotency_fixture_count")
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(1)
            == 0
        && source_replay
            .get("replay_idempotency_performed_count")
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(1)
            == 0;
    let report_ready = source_replay_ready && route_count_source_command_accepted;
    let source_u64 = |key: &str| {
        source_replay
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };

    let ordering_fixture = |fixture_id: &str,
                            status: &str,
                            reason: &str,
                            extra: serde_json::Value| {
        let mut fixture = serde_json::Map::new();
        for (key, value) in [
            ("fixture_id", fixture_id),
            ("ordering_monotonicity_status", status),
            ("denial_reason", reason),
        ] {
            fixture.insert(
                key.to_string(),
                serde_json::Value::String(value.to_string()),
            );
        }
        fixture.insert(
            "source_replay_idempotency_present".to_string(),
            serde_json::Value::Bool(true),
        );
        fixture.insert(
            "source_replay_idempotency_ready".to_string(),
            serde_json::Value::Bool(true),
        );
        fixture.insert(
            "ordering_requested".to_string(),
            serde_json::Value::Bool(true),
        );
        fixture.insert(
            "canonical_blocked_noop_result_receipt_order_identity_required".to_string(),
            serde_json::Value::Bool(true),
        );
        fixture.insert(
            "receipt_noop_confirmed".to_string(),
            serde_json::Value::Bool(true),
        );
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
            "activation_command_result_receipt_out_of_order_accepted",
            "activation_command_result_receipt_stale_sequence_accepted",
            "activation_command_result_receipt_future_sequence_accepted",
            "activation_command_result_receipt_sequence_gap_accepted",
            "activation_command_result_receipt_timestamp_rollback_accepted",
            "activation_command_result_receipt_epoch_rollback_accepted",
            "activation_command_result_receipt_same_sequence_hash_override_accepted",
            "activation_command_result_receipt_latest_wins_overwrite_accepted",
            "activation_command_result_receipt_ack_before_noop_accepted",
            "activation_command_result_receipt_ledger_ordering_bypass_accepted",
            "activation_command_result_receipt_index_ordering_bypass_accepted",
            "activation_command_result_receipt_delivery_ordering_bypass_accepted",
            "activation_command_result_receipt_export_ordering_bypass_accepted",
            "activation_command_result_receipt_query_ordering_bypass_accepted",
            "activation_command_result_receipt_observability_ordering_bypass_accepted",
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
            "activation_command_completion_ack_recorded",
            "activation_command_completion_ack_accepted",
            "operator_approval_from_ordering_accepted",
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
            "operator_approval_recorded",
            "dispatch_performed",
            "execution_performed",
            "context_injection_performed",
            "provider_invoked",
            "model_invoked",
            "memory_store_write_performed",
            "external_kg_adapter_read_performed",
            "live_kg_write_performed",
            "credential_read",
            "secret_file_read",
            "channel_send_performed",
            "external_send_performed",
            "public_claim_performed",
            "install_performed",
            "service_restarted",
            "active_binary_mutated",
            "upstream_fetch_performed",
            "upstream_merge_performed",
        ] {
            fixture.insert(key.to_string(), serde_json::Value::Bool(false));
        }
        let mut fixture = serde_json::Value::Object(fixture);
        extend_json_object(&mut fixture, extra);
        fixture
    };
    let ordering_monotonicity_fixtures = serde_json::Value::Array(vec![
        ordering_fixture(
            "missing-source-replay-idempotency-report",
            "blocked_noop",
            "source_result_receipt_replay_idempotency_report_required",
            serde_json::json!({
                "source_replay_idempotency_present": false,
                "source_replay_idempotency_ready": false,
            }),
        ),
        ordering_fixture(
            "sequence-cursor-recording-attempt",
            "blocked_sequence_cursor_noop",
            "sequence_cursor_recording_denied",
            serde_json::json!({
                "sequence_cursor_recording_requested": true,
                "requested_sequence_cursor": "operator_canary_ack_result_receipt_sequence_1",
            }),
        ),
        ordering_fixture(
            "out-of-order-sequence-attempt",
            "blocked_out_of_order_noop",
            "out_of_order_result_receipt_sequence_denied",
            serde_json::json!({
                "out_of_order_sequence_requested": true,
                "requested_sequence": 2,
                "observed_previous_sequence": 3,
            }),
        ),
        ordering_fixture(
            "stale-sequence-replay-attempt",
            "blocked_stale_sequence_noop",
            "stale_sequence_result_receipt_replay_denied",
            serde_json::json!({
                "stale_sequence_requested": true,
                "requested_sequence": 1,
                "observed_previous_sequence": 3,
            }),
        ),
        ordering_fixture(
            "future-sequence-gap-attempt",
            "blocked_future_sequence_noop",
            "future_sequence_gap_result_receipt_denied",
            serde_json::json!({
                "future_sequence_requested": true,
                "requested_sequence": 5,
                "expected_next_sequence": 1,
            }),
        ),
        ordering_fixture(
            "timestamp-epoch-rollback-attempt",
            "blocked_rollback_noop",
            "timestamp_epoch_rollback_result_receipt_denied",
            serde_json::json!({
                "timestamp_rollback_requested": true,
                "epoch_rollback_requested": true,
            }),
        ),
        ordering_fixture(
            "same-sequence-different-hash-attempt",
            "blocked_same_sequence_hash_noop",
            "same_sequence_different_hash_result_receipt_denied",
            serde_json::json!({
                "same_sequence_different_hash_requested": true,
                "requested_sequence": 1,
                "requested_hash_relation": "different_hash_for_same_sequence",
            }),
        ),
        ordering_fixture(
            "latest-wins-overwrite-attempt",
            "blocked_latest_wins_noop",
            "latest_wins_result_receipt_overwrite_denied",
            serde_json::json!({
                "latest_wins_overwrite_requested": true,
                "overwrite_existing_noop_requested": true,
            }),
        ),
        ordering_fixture(
            "ack-ledger-index-delivery-ordering-bypass-attempt",
            "blocked_ledger_delivery_noop",
            "ack_ledger_index_delivery_ordering_bypass_denied",
            serde_json::json!({
                "completion_ack_before_noop_requested": true,
                "ledger_ordering_bypass_requested": true,
                "index_ordering_bypass_requested": true,
                "delivery_ordering_bypass_requested": true,
                "export_ordering_bypass_requested": true,
                "query_ordering_bypass_requested": true,
                "observability_ordering_bypass_requested": true,
            }),
        ),
        ordering_fixture(
            "activation-provider-memory-kg-external-ordering-bypass-attempt",
            "blocked_activation_provider_memory_kg_external_noop",
            "activation_provider_memory_kg_external_ordering_bypass_denied",
            serde_json::json!({
                "operator_approval_from_ordering_requested": true,
                "activation_from_ordering_requested": true,
                "context_injection_ordering_bypass_requested": true,
                "provider_ordering_bypass_requested": true,
                "model_ordering_bypass_requested": true,
                "memory_store_ordering_bypass_requested": true,
                "external_kg_ordering_bypass_requested": true,
                "live_kg_ordering_bypass_requested": true,
                "external_send_ordering_bypass_requested": true,
                "public_claim_ordering_bypass_requested": true,
                "install_ordering_bypass_requested": true,
                "service_restart_ordering_bypass_requested": true,
                "active_binary_mutation_ordering_bypass_requested": true,
                "upstream_ordering_bypass_requested": true,
                "credential_ordering_bypass_requested": true,
                "secret_value_ordering_bypass_requested": true,
            }),
        ),
    ]);
    let ordering_monotonicity_fixture_count = ordering_monotonicity_fixtures
        .as_array()
        .map(std::vec::Vec::len)
        .unwrap_or(0);
    let ordering_monotonicity_fixtures_sha256 = sha256_json_value(&ordering_monotonicity_fixtures);
    let source_replay_report_sha256 = sha256_json_value(&source_replay);
    let source_replay_contract_hash_sha256 = source_replay
        .get("replay_idempotency_contract_hash_sha256")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("")
        .to_string();
    let source_result_receipt_no_persistence_hash_sha256 = source_replay
        .get("source_result_receipt_no_persistence_hash_sha256")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("")
        .to_string();
    let ordering_monotonicity_contract_hash_sha256 = sha256_text_value(&format!(
        "hepta-canary-operator-review-acknowledgement-activation-command-result-receipt-ordering-monotonicity-denial:v1:source={source_replay_report_sha256}:replay={source_replay_contract_hash_sha256}:receipt={source_result_receipt_no_persistence_hash_sha256}:fixtures={ordering_monotonicity_fixtures_sha256}:ordering=0:cursor=0:monotonicity=0:persist=0:authority=0:live=0"
    ));
    let ordering_monotonicity_policy_hash_sha256 = sha256_text_value(
        "memory-intelligence-kg-operator-canary-harness-operator-review-acknowledgement-activation-command-result-receipt-ordering-monotonicity-denial:v1:no-ordering:no-sequence-cursor:no-monotonicity-state:no-latest-wins:no-ack-ledger-bypass:no-authority:no-live",
    );
    let side_effect_hash_sha256 = sha256_text_value(
        "operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_side_effects=false;fixtures=10;ordering=0;cursor=0;monotonicity=0;record=0;persist=0;activation=0;provider=0;model=0;memory=0;kg=0;secret=0",
    );

    let mut denials = source_replay
        .get("denied_by_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency")
        .and_then(serde_json::Value::as_array)
        .cloned()
        .unwrap_or_default();
    for denial in [
        "source_result_receipt_replay_idempotency_report_required",
        "canonical_blocked_noop_result_receipt_order_identity_required",
        "sequence_cursor_acceptance_denied",
        "sequence_cursor_recording_denied",
        "sequence_cursor_persistence_denied",
        "monotonicity_state_recording_denied",
        "monotonicity_state_persistence_denied",
        "monotonicity_state_materialization_denied",
        "monotonicity_filesystem_write_denied",
        "out_of_order_sequence_denied",
        "stale_sequence_denied",
        "future_sequence_denied",
        "sequence_gap_denied",
        "timestamp_rollback_denied",
        "epoch_rollback_denied",
        "same_sequence_different_hash_denied",
        "latest_wins_overwrite_denied",
        "completion_ack_before_noop_denied",
        "ledger_index_delivery_ordering_bypass_denied",
        "export_query_observability_ordering_bypass_denied",
        "operator_approval_from_ordering_denied",
        "activation_from_ordering_denied",
        "context_injection_ordering_bypass_denied",
        "provider_model_ordering_bypass_denied",
        "memory_kg_ordering_bypass_denied",
        "credential_secret_ordering_bypass_denied",
        "external_public_install_restart_ordering_bypass_denied",
        "active_binary_mutation_ordering_bypass_denied",
        "upstream_ordering_bypass_denied",
    ] {
        denials.push(serde_json::Value::String(denial.to_string()));
    }
    let denied_count = denials.len();

    let mut report = source_replay.clone();
    extend_json_object(
        &mut report,
        serde_json::json!({
            "status": if report_ready { "ready" } else { "blocked" },
            "source_command": "/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-ordering-monotonicity-denial --json",
            "native_route": true,
            "compatibility_mode": "native_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_status",
            "side_effect_free": true,
            "audit_date": "2026-06-13",
            "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_ORDERING_MONOTONICITY_DENIAL_ENDPOINT,
            "source_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_route_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_REPLAY_IDEMPOTENCY_DENIAL_ENDPOINT,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_doc": "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_ORDERING_MONOTONICITY_DENIAL_GATE.md",
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_route_doc": "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_ORDERING_MONOTONICITY_DENIAL_ROUTE_GATE.md",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-replay-idempotency-denial-route-gate.sh",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-ordering-monotonicity-denial-gate.sh",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-ordering-monotonicity-denial-route-gate.sh",
            "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
            "route_count": route_matrix.route_count,
            "implemented_route_count": route_matrix.implemented_route_count,
            "missing_route_count": route_matrix.missing_route_count,
            "route_count_cutover_floor": NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR,
            "route_count_floor_preserved": route_count_floor_preserved,
            "route_count_source_command_accepted": route_count_source_command_accepted,
            "source_route_wired": true,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_route_status": source_status,
            "source_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_route_ready": source_replay_ready,
            "source_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_report_sha256": source_replay_report_sha256,
            "source_replay_idempotency_contract_hash_sha256": source_replay_contract_hash_sha256,
            "source_result_receipt_no_persistence_hash_sha256": source_result_receipt_no_persistence_hash_sha256,
            "operator_authorization_scope": "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_no_sequence_cursor_no_monotonicity_state_no_ordering_record_no_persist_no_authority_no_context_memory_kg_provider_model_credential_channel_install_restart_binary_or_public_release",
            "operator_authorization_received": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_route_enabled": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_ready": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_status": "blocked",
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_schema_version": "memory_intelligence_kg_operator_canary_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_v1",
            "ordering_monotonicity_mode": "native_route_stdout_only_sequence_cursor_and_monotonicity_denial_no_record_no_persist_no_authority_no_live",
            "ordering_monotonicity_decision": "operator_review_acknowledgement_activation_command_result_receipt_cannot_create_ordering_sequence_cursor_or_monotonicity_authority",
            "minimum_required_samples": 24,
            "ordering_monotonicity_fixtures_sha256": ordering_monotonicity_fixtures_sha256,
            "ordering_monotonicity_contract_hash_sha256": ordering_monotonicity_contract_hash_sha256,
            "ordering_monotonicity_policy_hash_sha256": ordering_monotonicity_policy_hash_sha256,
            "side_effect_hash_sha256": side_effect_hash_sha256,
            "source_activation_command_result_receipt_surface_count": source_u64("source_activation_command_result_receipt_surface_count"),
            "source_activation_command_result_receipt_fixture_count": source_u64("source_activation_command_result_receipt_fixture_count"),
            "source_accepted_activation_command_result_receipt_fixture_count": source_u64("source_accepted_activation_command_result_receipt_fixture_count"),
            "source_replay_idempotency_fixture_count": source_u64("replay_idempotency_fixture_count"),
            "source_blocked_replay_idempotency_fixture_count": source_u64("blocked_replay_idempotency_fixture_count"),
            "source_noop_replay_idempotency_fixture_count": source_u64("noop_replay_idempotency_fixture_count"),
            "source_accepted_replay_idempotency_fixture_count": source_u64("accepted_replay_idempotency_fixture_count"),
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "ordering_monotonicity_surface_count": 14,
            "ordering_monotonicity_surface_ready_count": 14,
            "ordering_monotonicity_side_effect_free_surface_count": 14,
            "ordering_monotonicity_fixtures": ordering_monotonicity_fixtures,
            "ordering_monotonicity_fixture_count": ordering_monotonicity_fixture_count,
            "blocked_ordering_monotonicity_fixture_count": ordering_monotonicity_fixture_count,
            "noop_ordering_monotonicity_fixture_count": ordering_monotonicity_fixture_count,
            "allowed_ordering_monotonicity_fixture_count": 0,
            "accepted_ordering_monotonicity_fixture_count": 0,
            "sequence_cursor_recording_fixture_count": 1,
            "out_of_order_sequence_fixture_count": 1,
            "stale_sequence_fixture_count": 1,
            "future_sequence_gap_fixture_count": 1,
            "timestamp_epoch_rollback_fixture_count": 1,
            "same_sequence_hash_fixture_count": 1,
            "latest_wins_overwrite_fixture_count": 1,
            "ack_ledger_index_delivery_bypass_fixture_count": 1,
            "activation_provider_memory_kg_external_bypass_fixture_count": 1,
            "ordering_monotonicity_denied_count": ordering_monotonicity_fixture_count,
            "ordering_monotonicity_performed_count": 0,
            "sequence_cursor_accepted_count": 0,
            "sequence_cursor_recorded_count": 0,
            "monotonicity_state_recorded_count": 0,
            "monotonicity_state_persisted_count": 0,
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
            "activation_command_result_receipt_out_of_order_accepted",
            "activation_command_result_receipt_stale_sequence_accepted",
            "activation_command_result_receipt_future_sequence_accepted",
            "activation_command_result_receipt_sequence_gap_accepted",
            "activation_command_result_receipt_timestamp_rollback_accepted",
            "activation_command_result_receipt_epoch_rollback_accepted",
            "activation_command_result_receipt_same_sequence_hash_override_accepted",
            "activation_command_result_receipt_latest_wins_overwrite_accepted",
            "activation_command_result_receipt_ack_before_noop_accepted",
            "activation_command_result_receipt_ledger_ordering_bypass_accepted",
            "activation_command_result_receipt_index_ordering_bypass_accepted",
            "activation_command_result_receipt_delivery_ordering_bypass_accepted",
            "activation_command_result_receipt_export_ordering_bypass_accepted",
            "activation_command_result_receipt_query_ordering_bypass_accepted",
            "activation_command_result_receipt_observability_ordering_bypass_accepted",
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
            "activation_command_completion_ack_recorded",
            "activation_command_completion_ack_persisted",
            "activation_command_completion_ack_accepted",
            "operator_approval_from_ordering_accepted",
            "operator_approval_from_replay_accepted",
            "operator_approval_from_receipt_accepted",
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
            "operator_approval_recorded",
        ] {
            report_object.insert(key.to_string(), serde_json::json!(false));
        }
    }
    extend_json_object(
        &mut report,
        serde_json::json!({
            "dispatch_performed_count": 0,
            "execution_performed_count": 0,
            "context_injection_performed_count": 0,
            "provider_invoked_count": 0,
            "model_invoked_count": 0,
            "memory_store_write_performed_count": 0,
            "external_kg_adapter_read_performed_count": 0,
            "live_kg_write_performed_count": 0,
            "credential_read_count": 0,
            "secret_file_read_count": 0,
            "channel_send_performed_count": 0,
            "install_performed_count": 0,
            "service_restarted_count": 0,
            "active_binary_mutated_count": 0,
            "upstream_fetch_performed_count": 0,
            "upstream_merge_performed_count": 0,
            "canary_harness_armed": false,
            "canary_harness_executable": false,
            "canary_live_enabled": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "live_mutation_enabled_count": 1,
            "current_live_enabled_lane_count": 20,
            "enablement_lane_count": 23,
            "ready_enablement_lane_count": 23,
            "denied_by_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity": denials,
            "denied_by_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_count": denied_count,
            "allowed_next_actions": [
                {
                    "action": "review_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial",
                    "status": "allowed_report_only",
                    "records_sequence_cursor": false,
                    "persists_ordering_state": false,
                    "mutates_runtime": false,
                    "invokes_model": false,
                    "writes_memory_or_kg": false
                },
                {
                    "action": "stage_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial",
                    "status": "allowed_report_only_next_slice",
                    "accepts_cancellation": false,
                    "accepts_supersession": false,
                    "persists_ordering_state": false,
                    "mutates_runtime": false,
                    "invokes_model": false,
                    "writes_memory_or_kg": false
                }
            ],
        }),
    );
    if let Some(side_effects) = report
        .get_mut("side_effects")
        .and_then(serde_json::Value::as_object_mut)
    {
        for key in [
            "workspace_written",
            "filesystem_written",
            "activation_command_result_receipt_ordering_recorded",
            "activation_command_result_receipt_ordering_persisted",
            "activation_command_result_receipt_ordering_performed",
            "activation_command_result_receipt_sequence_cursor_recorded",
            "activation_command_result_receipt_sequence_cursor_persisted",
            "activation_command_result_receipt_monotonicity_state_recorded",
            "activation_command_result_receipt_monotonicity_state_persisted",
            "activation_command_result_receipt_monotonicity_state_materialized",
            "activation_command_result_receipt_monotonicity_filesystem_written",
            "activation_command_result_receipt_out_of_order_accepted",
            "activation_command_result_receipt_stale_sequence_accepted",
            "activation_command_result_receipt_future_sequence_accepted",
            "activation_command_result_receipt_sequence_gap_accepted",
            "activation_command_result_receipt_timestamp_rollback_accepted",
            "activation_command_result_receipt_epoch_rollback_accepted",
            "activation_command_result_receipt_same_sequence_hash_override_accepted",
            "activation_command_result_receipt_latest_wins_overwrite_accepted",
            "activation_command_result_receipt_ack_before_noop_accepted",
            "activation_command_result_receipt_ledger_ordering_bypass_accepted",
            "activation_command_result_receipt_index_ordering_bypass_accepted",
            "activation_command_result_receipt_delivery_ordering_bypass_accepted",
            "activation_command_result_receipt_export_ordering_bypass_accepted",
            "activation_command_result_receipt_query_ordering_bypass_accepted",
            "activation_command_result_receipt_observability_ordering_bypass_accepted",
            "activation_command_result_receipt_provider_ordering_bypass_accepted",
            "activation_command_result_receipt_memory_kg_ordering_bypass_accepted",
            "activation_command_result_receipt_external_public_install_ordering_bypass_accepted",
            "activation_command_result_receipt_replay_recorded",
            "activation_command_result_receipt_replay_persisted",
            "activation_command_result_receipt_duplicate_accepted",
            "activation_command_result_receipt_idempotency_state_recorded",
            "activation_command_result_receipt_idempotency_state_persisted",
            "activation_command_result_receipt_recorded",
            "activation_command_result_receipt_persisted",
            "activation_command_result_receipt_accepted",
            "activation_command_completion_ack_recorded",
            "operator_approval_from_ordering_accepted",
            "activation_from_ordering_allowed",
            "activation_from_replay_allowed",
            "activation_from_receipt_allowed",
            "activation_command_enabled",
            "activation_command_invoked",
            "activation_command_dispatched",
            "activation_request_recorded",
            "activation_request_persisted",
            "activation_request_executed",
            "operator_approval_recorded",
            "dispatch_performed",
            "execution_performed",
            "context_injection_performed",
            "provider_invoked",
            "model_invoked",
            "memory_store_write_performed",
            "memory_store_mutated",
            "external_kg_adapter_read_performed",
            "live_kg_write_performed",
            "credential_read",
            "secret_file_read",
            "channel_send_performed",
            "telegram_send_performed",
            "external_send_performed",
            "public_claim_performed",
            "install_performed",
            "service_restarted",
            "active_binary_mutated",
            "upstream_fetch_performed",
            "upstream_merge_performed",
            "public_release_claimed",
            "public_ga_claimed",
        ] {
            side_effects.insert(key.to_string(), serde_json::json!(false));
        }
    }
    report
}

fn hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let source_ordering =
        hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_report();
    let source_status = source_ordering
        .get("operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_status")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("unknown");
    let source_ordering_ready = source_status == "blocked"
        && source_ordering
            .get("operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_ready")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
        && source_ordering
            .get("accepted_ordering_monotonicity_fixture_count")
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(1)
            == 0
        && source_ordering
            .get("ordering_monotonicity_performed_count")
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(1)
            == 0
        && source_ordering
            .get("sequence_cursor_recorded_count")
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(1)
            == 0
        && source_ordering
            .get("monotonicity_state_recorded_count")
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(1)
            == 0;
    let report_ready = source_ordering_ready && route_count_source_command_accepted;
    let source_u64 = |key: &str| {
        source_ordering
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };

    let cancellation_supersession_fixture =
        |fixture_id: &str, status: &str, reason: &str, extra: serde_json::Value| {
            let mut fixture = serde_json::Map::new();
            for (key, value) in [
                ("fixture_id", fixture_id),
                ("cancellation_supersession_status", status),
                ("denial_reason", reason),
            ] {
                fixture.insert(
                    key.to_string(),
                    serde_json::Value::String(value.to_string()),
                );
            }
            for (key, value) in [
                ("source_ordering_monotonicity_present", true),
                ("source_ordering_monotonicity_ready", true),
                ("cancellation_requested", true),
                ("supersession_requested", false),
                (
                    "canonical_blocked_noop_result_receipt_identity_required",
                    true,
                ),
                ("receipt_noop_confirmed", true),
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(value));
            }
            for key in [
                "activation_command_result_receipt_cancellation_allowed",
                "activation_command_result_receipt_cancellation_recorded",
                "activation_command_result_receipt_cancellation_persisted",
                "activation_command_result_receipt_cancellation_materialized",
                "activation_command_result_receipt_cancellation_filesystem_written",
                "activation_command_result_receipt_cancellation_request_accepted",
                "activation_command_result_receipt_supersession_allowed",
                "activation_command_result_receipt_supersession_recorded",
                "activation_command_result_receipt_supersession_persisted",
                "activation_command_result_receipt_supersession_materialized",
                "activation_command_result_receipt_supersession_filesystem_written",
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
                "operator_approval_from_cancellation_accepted",
                "operator_approval_from_supersession_accepted",
                "operator_approval_from_ordering_accepted",
                "operator_approval_from_replay_accepted",
                "operator_approval_from_receipt_accepted",
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
                "operator_approval_recorded",
                "dispatch_performed",
                "execution_performed",
                "context_injection_performed",
                "provider_invoked",
                "model_invoked",
                "memory_store_write_performed",
                "memory_store_mutated",
                "external_kg_adapter_read_performed",
                "live_kg_write_performed",
                "credential_read",
                "secret_file_read",
                "auth_secret_read",
                "secret_value_read",
                "raw_payload_plaintext_recorded",
                "raw_payload_plaintext_persisted",
                "channel_send_performed",
                "telegram_send_performed",
                "external_send_performed",
                "public_claim_performed",
                "public_release_claimed",
                "public_ga_claimed",
                "release_artifact_written",
                "install_performed",
                "install_executed",
                "launchd_mutated",
                "service_restarted",
                "service_restart_performed",
                "active_binary_mutated",
                "upstream_fetch_performed",
                "upstream_merge_performed",
                "rollback_executed",
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(false));
            }
            let mut fixture = serde_json::Value::Object(fixture);
            extend_json_object(&mut fixture, extra);
            fixture
        };
    let cancellation_supersession_fixtures = serde_json::Value::Array(vec![
        cancellation_supersession_fixture(
            "missing-source-ordering-monotonicity-report",
            "blocked_noop",
            "source_result_receipt_ordering_monotonicity_report_required",
            serde_json::json!({
                "source_ordering_monotonicity_present": false,
                "source_ordering_monotonicity_ready": false,
            }),
        ),
        cancellation_supersession_fixture(
            "cancel-blocked-noop-result-receipt",
            "blocked_cancellation_noop",
            "cancellation_of_blocked_noop_result_receipt_denied",
            serde_json::json!({
                "cancellation_request_shape": "cancel_blocked_noop_result_receipt",
            }),
        ),
        cancellation_supersession_fixture(
            "supersede-blocked-noop-with-completed-result-receipt",
            "blocked_supersession_noop",
            "supersession_of_blocked_noop_with_completed_result_receipt_denied",
            serde_json::json!({
                "cancellation_requested": false,
                "supersession_requested": true,
                "requested_replacement_status": "completed",
            }),
        ),
        cancellation_supersession_fixture(
            "replacement-receipt-recording-persistence-attempt",
            "blocked_supersession_noop",
            "replacement_receipt_recording_persistence_denied",
            serde_json::json!({
                "cancellation_requested": false,
                "supersession_requested": true,
                "replacement_receipt_requested": true,
                "replacement_hash_requested": true,
                "requested_hash_relation": "different_hash_for_same_receipt_identity",
            }),
        ),
        cancellation_supersession_fixture(
            "tombstone-delete-marker-attempt",
            "blocked_cancellation_noop",
            "tombstone_delete_marker_denied",
            serde_json::json!({
                "tombstone_requested": true,
                "delete_marker_requested": true,
            }),
        ),
        cancellation_supersession_fixture(
            "completion-acknowledgement-cancellation-replacement-attempt",
            "blocked_cancellation_supersession_noop",
            "completion_acknowledgement_cancellation_replacement_denied",
            serde_json::json!({
                "completion_ack_cancellation_requested": true,
                "ack_cancellation_requested": true,
                "supersession_requested": true,
                "requested_ack_replacement_status": "accepted",
            }),
        ),
        cancellation_supersession_fixture(
            "ledger-index-delivery-export-query-observability-bypass-attempt",
            "blocked_ledger_index_delivery_noop",
            "ledger_index_delivery_export_query_observability_cancellation_supersession_bypass_denied",
            serde_json::json!({
                "ledger_cancellation_requested": true,
                "index_cancellation_requested": true,
                "delivery_cancellation_requested": true,
                "export_cancellation_requested": true,
                "query_cancellation_requested": true,
                "observability_cancellation_requested": true,
            }),
        ),
        cancellation_supersession_fixture(
            "context-provider-model-memory-kg-supersession-attempt",
            "blocked_context_provider_memory_kg_noop",
            "context_provider_model_memory_kg_supersession_denied",
            serde_json::json!({
                "cancellation_requested": false,
                "supersession_requested": true,
                "context_injection_supersession_requested": true,
                "provider_supersession_requested": true,
                "model_supersession_requested": true,
                "memory_store_supersession_requested": true,
                "external_kg_supersession_requested": true,
                "live_kg_supersession_requested": true,
            }),
        ),
        cancellation_supersession_fixture(
            "rollback-secret-external-public-install-supersession-attempt",
            "blocked_secret_external_install_noop",
            "rollback_secret_external_public_install_supersession_denied",
            serde_json::json!({
                "cancellation_requested": false,
                "supersession_requested": true,
                "rollback_supersession_requested": true,
                "credential_secret_supersession_requested": true,
                "external_send_supersession_requested": true,
                "public_claim_supersession_requested": true,
                "release_artifact_supersession_requested": true,
                "install_supersession_requested": true,
                "service_restart_supersession_requested": true,
                "active_binary_mutation_supersession_requested": true,
                "upstream_supersession_requested": true,
            }),
        ),
        cancellation_supersession_fixture(
            "latest-wins-sequence-cursor-cancellation-supersession-bypass-attempt",
            "blocked_latest_wins_cursor_noop",
            "latest_wins_sequence_cursor_cancellation_supersession_bypass_denied",
            serde_json::json!({
                "latest_wins_cancellation_bypass_requested": true,
                "latest_wins_supersession_bypass_requested": true,
                "sequence_cursor_cancellation_bypass_requested": true,
                "monotonicity_state_supersession_bypass_requested": true,
            }),
        ),
    ]);
    let cancellation_supersession_fixture_count = cancellation_supersession_fixtures
        .as_array()
        .map(std::vec::Vec::len)
        .unwrap_or(0);
    let cancellation_fixture_count = cancellation_supersession_fixtures
        .as_array()
        .map(|fixtures| {
            fixtures
                .iter()
                .filter(|fixture| {
                    fixture
                        .get("cancellation_requested")
                        .and_then(serde_json::Value::as_bool)
                        .unwrap_or(false)
                })
                .count()
        })
        .unwrap_or(0);
    let supersession_fixture_count = cancellation_supersession_fixtures
        .as_array()
        .map(|fixtures| {
            fixtures
                .iter()
                .filter(|fixture| {
                    fixture
                        .get("supersession_requested")
                        .and_then(serde_json::Value::as_bool)
                        .unwrap_or(false)
                })
                .count()
        })
        .unwrap_or(0);
    let cancellation_supersession_fixtures_sha256 =
        sha256_json_value(&cancellation_supersession_fixtures);
    let source_ordering_report_sha256 = sha256_json_value(&source_ordering);
    let source_ordering_contract_hash_sha256 = source_ordering
        .get("ordering_monotonicity_contract_hash_sha256")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("")
        .to_string();
    let source_ordering_policy_hash_sha256 = source_ordering
        .get("ordering_monotonicity_policy_hash_sha256")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("")
        .to_string();
    let source_replay_idempotency_report_sha256 = source_ordering
        .get("source_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_report_sha256")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("")
        .to_string();
    let cancellation_supersession_contract_hash_sha256 = sha256_text_value(&format!(
        "hepta-canary-operator-review-acknowledgement-activation-command-result-receipt-cancellation-supersession-denial:v1:source={source_ordering_report_sha256}:ordering={source_ordering_contract_hash_sha256}:replay={source_replay_idempotency_report_sha256}:fixtures={cancellation_supersession_fixtures_sha256}:cancel=0:supersede=0:replace=0:persist=0:authority=0:live=0"
    ));
    let cancellation_supersession_policy_hash_sha256 = sha256_text_value(
        "memory-intelligence-kg-operator-canary-harness-operator-review-acknowledgement-activation-command-result-receipt-cancellation-supersession-denial:v1:no-cancel:no-supersede:no-replacement:no-tombstone:no-delete:no-authority:no-live",
    );
    let side_effect_hash_sha256 = sha256_text_value(
        "operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_side_effects=false;fixtures=10;cancel=0;supersede=0;replacement=0;tombstone=0;record=0;persist=0;activation=0;provider=0;model=0;memory=0;kg=0;secret=0",
    );

    let mut denials = source_ordering
        .get("denied_by_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity")
        .and_then(serde_json::Value::as_array)
        .cloned()
        .unwrap_or_default();
    for denial in [
        "source_result_receipt_ordering_monotonicity_report_required",
        "canonical_blocked_noop_result_receipt_identity_required",
        "cancellation_request_acceptance_denied",
        "cancellation_recording_denied",
        "cancellation_persistence_denied",
        "cancellation_materialization_denied",
        "cancellation_filesystem_write_denied",
        "supersession_request_acceptance_denied",
        "supersession_recording_denied",
        "supersession_persistence_denied",
        "supersession_materialization_denied",
        "supersession_filesystem_write_denied",
        "replacement_receipt_acceptance_denied",
        "replacement_receipt_recording_denied",
        "replacement_receipt_persistence_denied",
        "replacement_hash_acceptance_denied",
        "tombstone_recording_denied",
        "delete_marker_recording_denied",
        "completion_acknowledgement_cancellation_denied",
        "ledger_index_delivery_cancellation_denied",
        "export_query_observability_cancellation_denied",
        "context_provider_model_supersession_denied",
        "memory_kg_supersession_denied",
        "rollback_secret_supersession_denied",
        "external_public_release_supersession_denied",
        "install_restart_active_binary_supersession_denied",
        "upstream_supersession_denied",
        "latest_wins_cancellation_supersession_bypass_denied",
        "sequence_cursor_cancellation_supersession_bypass_denied",
        "operator_approval_from_cancellation_supersession_denied",
        "activation_from_cancellation_supersession_denied",
    ] {
        denials.push(serde_json::Value::String(denial.to_string()));
    }
    let denied_count = denials.len();

    let mut report = source_ordering.clone();
    extend_json_object(
        &mut report,
        serde_json::json!({
            "status": if report_ready { "ready" } else { "blocked" },
            "source_command": "/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-cancellation-supersession-denial --json",
            "native_route": true,
            "compatibility_mode": "native_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_status",
            "side_effect_free": true,
            "audit_date": "2026-06-13",
            "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_CANCELLATION_SUPERSESSION_DENIAL_ENDPOINT,
            "source_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_route_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_ORDERING_MONOTONICITY_DENIAL_ENDPOINT,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_doc": "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_CANCELLATION_SUPERSESSION_DENIAL_GATE.md",
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_route_doc": "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_CANCELLATION_SUPERSESSION_DENIAL_ROUTE_GATE.md",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-ordering-monotonicity-denial-route-gate.sh",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-cancellation-supersession-denial-gate.sh",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-cancellation-supersession-denial-route-gate.sh",
            "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
            "route_count": route_matrix.route_count,
            "implemented_route_count": route_matrix.implemented_route_count,
            "missing_route_count": route_matrix.missing_route_count,
            "route_count_cutover_floor": NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR,
            "route_count_floor_preserved": route_count_floor_preserved,
            "route_count_source_command_accepted": route_count_source_command_accepted,
            "source_route_wired": true,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_gate": "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_gate",
            "source_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_status": source_status,
            "source_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_route_ready": source_ordering_ready,
            "source_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_report_sha256": source_ordering_report_sha256,
            "source_ordering_monotonicity_contract_hash_sha256": source_ordering_contract_hash_sha256,
            "source_ordering_monotonicity_policy_hash_sha256": source_ordering_policy_hash_sha256,
            "source_replay_idempotency_report_sha256": source_replay_idempotency_report_sha256,
            "operator_authorization_scope": "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_no_cancel_no_supersede_no_replacement_no_tombstone_no_persist_no_authority_no_context_memory_kg_provider_model_credential_channel_install_restart_binary_or_public_release",
            "operator_authorization_received": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_route_enabled": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_ready": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_status": "blocked",
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_schema_version": "memory_intelligence_kg_operator_canary_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_v1",
            "cancellation_supersession_mode": "native_route_stdout_only_cancellation_supersession_denial_no_record_no_persist_no_replacement_no_authority_no_live",
            "cancellation_supersession_decision": "blocked_noop_activation_command_result_receipt_cannot_be_cancelled_superseded_replaced_or_promoted_to_authority",
            "minimum_required_samples": 24,
            "cancellation_supersession_fixtures_sha256": cancellation_supersession_fixtures_sha256,
            "cancellation_supersession_contract_hash_sha256": cancellation_supersession_contract_hash_sha256,
            "cancellation_supersession_policy_hash_sha256": cancellation_supersession_policy_hash_sha256,
            "side_effect_hash_sha256": side_effect_hash_sha256,
            "source_ordering_monotonicity_fixture_count": source_u64("ordering_monotonicity_fixture_count"),
            "source_blocked_ordering_monotonicity_fixture_count": source_u64("blocked_ordering_monotonicity_fixture_count"),
            "source_noop_ordering_monotonicity_fixture_count": source_u64("noop_ordering_monotonicity_fixture_count"),
            "source_accepted_ordering_monotonicity_fixture_count": source_u64("accepted_ordering_monotonicity_fixture_count"),
            "source_ordering_monotonicity_performed_count": source_u64("ordering_monotonicity_performed_count"),
            "source_sequence_cursor_accepted_count": source_u64("sequence_cursor_accepted_count"),
            "source_sequence_cursor_recorded_count": source_u64("sequence_cursor_recorded_count"),
            "source_monotonicity_state_recorded_count": source_u64("monotonicity_state_recorded_count"),
            "source_monotonicity_state_persisted_count": source_u64("monotonicity_state_persisted_count"),
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "cancellation_supersession_surface_count": 14,
            "cancellation_supersession_surface_ready_count": 14,
            "cancellation_supersession_side_effect_free_surface_count": 14,
            "cancellation_supersession_fixtures": cancellation_supersession_fixtures,
            "cancellation_supersession_fixture_count": cancellation_supersession_fixture_count,
            "blocked_cancellation_supersession_fixture_count": cancellation_supersession_fixture_count,
            "noop_cancellation_supersession_fixture_count": cancellation_supersession_fixture_count,
            "allowed_cancellation_supersession_fixture_count": 0,
            "accepted_cancellation_supersession_fixture_count": 0,
            "cancellation_fixture_count": cancellation_fixture_count,
            "supersession_fixture_count": supersession_fixture_count,
            "cancellation_denied_count": cancellation_fixture_count,
            "supersession_denied_count": supersession_fixture_count,
            "cancellation_performed_count": 0,
            "supersession_performed_count": 0,
            "replacement_receipt_accepted_count": 0,
            "replacement_receipt_recorded_count": 0,
            "replacement_receipt_persisted_count": 0,
            "tombstone_recorded_count": 0,
            "delete_marker_recorded_count": 0,
        }),
    );
    if let Some(report_object) = report.as_object_mut() {
        for key in [
            "activation_command_result_receipt_cancellation_allowed",
            "activation_command_result_receipt_cancellation_recorded",
            "activation_command_result_receipt_cancellation_persisted",
            "activation_command_result_receipt_cancellation_materialized",
            "activation_command_result_receipt_cancellation_filesystem_written",
            "activation_command_result_receipt_cancellation_request_accepted",
            "activation_command_result_receipt_supersession_allowed",
            "activation_command_result_receipt_supersession_recorded",
            "activation_command_result_receipt_supersession_persisted",
            "activation_command_result_receipt_supersession_materialized",
            "activation_command_result_receipt_supersession_filesystem_written",
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
            "operator_approval_from_cancellation_accepted",
            "operator_approval_from_supersession_accepted",
            "operator_approval_from_ordering_accepted",
            "operator_approval_from_replay_accepted",
            "operator_approval_from_receipt_accepted",
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
            "operator_approval_recorded",
        ] {
            report_object.insert(key.to_string(), serde_json::json!(false));
        }
    }
    extend_json_object(
        &mut report,
        serde_json::json!({
            "dispatch_performed_count": 0,
            "execution_performed_count": 0,
            "context_injection_performed_count": 0,
            "provider_invoked_count": 0,
            "model_invoked_count": 0,
            "memory_store_write_performed_count": 0,
            "external_kg_adapter_read_performed_count": 0,
            "live_kg_write_performed_count": 0,
            "credential_read_count": 0,
            "secret_file_read_count": 0,
            "channel_send_performed_count": 0,
            "install_performed_count": 0,
            "service_restarted_count": 0,
            "active_binary_mutated_count": 0,
            "upstream_fetch_performed_count": 0,
            "upstream_merge_performed_count": 0,
            "canary_harness_armed": false,
            "canary_harness_executable": false,
            "canary_live_enabled": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "live_mutation_enabled_count": 1,
            "current_live_enabled_lane_count": 21,
            "enablement_lane_count": 24,
            "ready_enablement_lane_count": 24,
            "denied_by_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession": denials,
            "denied_by_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_count": denied_count,
            "allowed_next_actions": [
                {
                    "action": "review_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial",
                    "status": "allowed_report_only",
                    "accepts_cancellation": false,
                    "accepts_supersession": false,
                    "persists_replacement_receipt": false,
                    "mutates_runtime": false,
                    "invokes_model": false,
                    "writes_memory_or_kg": false
                },
                {
                    "action": "stage_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial",
                    "status": "allowed_report_only_next_slice",
                    "accepts_cancellation": false,
                    "accepts_supersession": false,
                    "writes_audit_trail": false,
                    "persists_evidence": false,
                    "mutates_runtime": false,
                    "invokes_model": false,
                    "writes_memory_or_kg": false
                }
            ],
        }),
    );
    if let Some(side_effects) = report
        .get_mut("side_effects")
        .and_then(serde_json::Value::as_object_mut)
    {
        for key in [
            "workspace_written",
            "filesystem_written",
            "activation_command_result_receipt_cancellation_recorded",
            "activation_command_result_receipt_cancellation_persisted",
            "activation_command_result_receipt_cancellation_materialized",
            "activation_command_result_receipt_cancellation_filesystem_written",
            "activation_command_result_receipt_supersession_recorded",
            "activation_command_result_receipt_supersession_persisted",
            "activation_command_result_receipt_supersession_materialized",
            "activation_command_result_receipt_supersession_filesystem_written",
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
            "activation_command_result_receipt_ordering_recorded",
            "activation_command_result_receipt_ordering_persisted",
            "activation_command_result_receipt_sequence_cursor_recorded",
            "activation_command_result_receipt_sequence_cursor_persisted",
            "activation_command_result_receipt_monotonicity_state_recorded",
            "activation_command_result_receipt_monotonicity_state_persisted",
            "activation_command_result_receipt_recorded",
            "activation_command_result_receipt_persisted",
            "activation_command_result_receipt_accepted",
            "activation_command_result_receipt_materialized",
            "activation_command_completion_ack_recorded",
            "activation_command_completion_ack_accepted",
            "operator_approval_from_cancellation_accepted",
            "operator_approval_from_supersession_accepted",
            "activation_from_cancellation_allowed",
            "activation_from_supersession_allowed",
            "activation_command_enabled",
            "activation_command_invoked",
            "activation_command_dispatched",
            "activation_request_recorded",
            "activation_request_persisted",
            "activation_request_executed",
            "operator_approval_recorded",
            "dispatch_performed",
            "execution_performed",
            "context_injection_performed",
            "provider_invoked",
            "model_invoked",
            "memory_store_write_performed",
            "memory_store_mutated",
            "external_kg_adapter_read_performed",
            "live_kg_write_performed",
            "credential_read",
            "secret_file_read",
            "channel_send_performed",
            "telegram_send_performed",
            "external_send_performed",
            "public_claim_performed",
            "install_performed",
            "service_restarted",
            "active_binary_mutated",
            "upstream_fetch_performed",
            "upstream_merge_performed",
            "public_release_claimed",
            "public_ga_claimed",
        ] {
            side_effects.insert(key.to_string(), serde_json::json!(false));
        }
    }
    report
}

fn hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source = hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_report();
    let source_status = source
        .get("operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_status")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("unknown")
        .to_string();
    let source_ready = source_status == "blocked"
        && source
            .get("operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_ready")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false);
    let source_u64 = |key: &str| {
        source
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_report_sha256 = sha256_json_value(&source);
    let source_contract_hash_sha256 = source
        .get("cancellation_supersession_contract_hash_sha256")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("")
        .to_string();
    let source_policy_hash_sha256 = source
        .get("cancellation_supersession_policy_hash_sha256")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("")
        .to_string();
    let source_ordering_report_sha256 = source
        .get("source_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_report_sha256")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("")
        .to_string();

    let audit_fixture =
        |id: &str, status: &str, reason: &str, extra: serde_json::Value| -> serde_json::Value {
            let mut fixture = serde_json::Map::new();
            fixture.insert("fixture_id".to_string(), serde_json::json!(id));
            fixture.insert(
                "audit_evidence_status".to_string(),
                serde_json::json!(status),
            );
            fixture.insert(
                "source_cancellation_supersession_present".to_string(),
                serde_json::json!(true),
            );
            fixture.insert(
                "source_cancellation_supersession_ready".to_string(),
                serde_json::json!(true),
            );
            fixture.insert("audit_trail_requested".to_string(), serde_json::json!(true));
            fixture.insert(
                "immutable_evidence_requested".to_string(),
                serde_json::json!(false),
            );
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
                "export_evidence_recorded",
                "query_evidence_registered",
                "observability_evidence_recorded",
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
                "activation_command_completion_ack_recorded",
                "activation_command_completion_ack_persisted",
                "activation_command_completion_ack_accepted",
                "operator_approval_from_audit_trail_accepted",
                "operator_approval_from_immutable_evidence_accepted",
                "activation_from_audit_trail_allowed",
                "activation_from_immutable_evidence_allowed",
                "activation_from_cancellation_allowed",
                "activation_from_supersession_allowed",
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
                "operator_approval_recorded",
                "dispatch_performed",
                "execution_performed",
                "context_injection_performed",
                "provider_invoked",
                "model_invoked",
                "provider_prompt_replayed",
                "memory_store_write_performed",
                "memory_store_mutated",
                "external_kg_adapter_read_performed",
                "live_kg_write_performed",
                "readback_evidence_recorded",
                "readback_evidence_persisted",
                "credential_read",
                "secret_file_read",
                "auth_secret_read",
                "secret_value_read",
                "raw_payload_plaintext_recorded",
                "raw_payload_plaintext_persisted",
                "channel_send_performed",
                "telegram_send_performed",
                "external_send_performed",
                "public_claim_performed",
                "public_release_claimed",
                "public_ga_claimed",
                "release_artifact_written",
                "install_performed",
                "install_executed",
                "launchd_mutated",
                "service_restarted",
                "service_restart_performed",
                "active_binary_mutated",
                "upstream_fetch_performed",
                "upstream_merge_performed",
                "rollback_executed",
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(false));
            }
            fixture.insert(
                "receipt_noop_confirmed".to_string(),
                serde_json::json!(true),
            );
            fixture.insert("denial_reason".to_string(), serde_json::json!(reason));
            let mut fixture = serde_json::Value::Object(fixture);
            extend_json_object(&mut fixture, extra);
            fixture
        };

    let audit_trail_immutable_evidence_fixtures = serde_json::Value::Array(vec![
        audit_fixture(
            "missing-source-cancellation-supersession-report",
            "blocked_noop",
            "source_result_receipt_cancellation_supersession_report_required",
            serde_json::json!({
                "source_cancellation_supersession_present": false,
                "source_cancellation_supersession_ready": false,
            }),
        ),
        audit_fixture(
            "append-audit-trail-to-blocked-noop-result-receipt",
            "blocked_audit_noop",
            "audit_trail_append_request_denied",
            serde_json::json!({
                "audit_trail_request_shape": "append_blocked_noop_result_receipt",
            }),
        ),
        audit_fixture(
            "seal-blocked-noop-as-immutable-evidence",
            "blocked_evidence_noop",
            "immutable_evidence_packet_request_denied",
            serde_json::json!({
                "immutable_evidence_requested": true,
                "audit_trail_requested": false,
                "immutable_evidence_request_shape": "seal_blocked_noop_result_receipt",
            }),
        ),
        audit_fixture(
            "hash-chain-merkle-root-evidence-attempt",
            "blocked_evidence_noop",
            "hash_chain_merkle_root_recording_denied",
            serde_json::json!({
                "immutable_evidence_requested": true,
                "audit_trail_requested": false,
                "hash_chain_requested": true,
                "merkle_root_requested": true,
            }),
        ),
        audit_fixture(
            "attestation-witness-notary-evidence-attempt",
            "blocked_evidence_noop",
            "attestation_witness_notary_recording_denied",
            serde_json::json!({
                "immutable_evidence_requested": true,
                "audit_trail_requested": false,
                "attestation_requested": true,
                "witness_requested": true,
                "notary_requested": true,
            }),
        ),
        audit_fixture(
            "audit-trail-materialization-filesystem-attempt",
            "blocked_audit_noop",
            "audit_trail_materialization_filesystem_denied",
            serde_json::json!({
                "audit_trail_materialization_requested": true,
                "audit_trail_filesystem_write_requested": true,
            }),
        ),
        audit_fixture(
            "ledger-index-delivery-export-query-observability-evidence-attempt",
            "blocked_evidence_noop",
            "ledger_index_delivery_export_query_observability_evidence_denied",
            serde_json::json!({
                "ledger_evidence_requested": true,
                "index_evidence_requested": true,
                "delivery_evidence_requested": true,
                "export_evidence_requested": true,
                "query_evidence_requested": true,
                "observability_evidence_requested": true,
            }),
        ),
        audit_fixture(
            "activation-from-audit-evidence-attempt",
            "blocked_evidence_noop",
            "activation_from_audit_evidence_denied",
            serde_json::json!({
                "immutable_evidence_requested": true,
                "audit_trail_requested": false,
                "activation_from_audit_evidence_requested": true,
            }),
        ),
        audit_fixture(
            "context-provider-model-memory-kg-readback-evidence-attempt",
            "blocked_evidence_noop",
            "context_provider_model_memory_kg_readback_evidence_denied",
            serde_json::json!({
                "immutable_evidence_requested": true,
                "audit_trail_requested": false,
                "context_evidence_requested": true,
                "provider_prompt_evidence_requested": true,
                "model_output_evidence_requested": true,
                "memory_store_evidence_requested": true,
                "external_kg_evidence_requested": true,
                "live_kg_evidence_requested": true,
                "readback_evidence_requested": true,
            }),
        ),
        audit_fixture(
            "rollback-secret-external-public-install-evidence-attempt",
            "blocked_evidence_noop",
            "rollback_secret_external_public_install_evidence_denied",
            serde_json::json!({
                "immutable_evidence_requested": true,
                "audit_trail_requested": false,
                "rollback_evidence_requested": true,
                "credential_secret_evidence_requested": true,
                "external_send_evidence_requested": true,
                "public_claim_evidence_requested": true,
                "release_artifact_evidence_requested": true,
                "install_evidence_requested": true,
                "service_restart_evidence_requested": true,
                "active_binary_mutation_evidence_requested": true,
                "upstream_evidence_requested": true,
            }),
        ),
    ]);
    let audit_fixture_count = audit_trail_immutable_evidence_fixtures
        .as_array()
        .map(std::vec::Vec::len)
        .unwrap_or(0);
    let audit_trail_denied_count = audit_trail_immutable_evidence_fixtures
        .as_array()
        .map(|fixtures| {
            fixtures
                .iter()
                .filter(|fixture| {
                    fixture
                        .get("audit_trail_requested")
                        .and_then(serde_json::Value::as_bool)
                        .unwrap_or(false)
                })
                .count()
        })
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
    let fixtures_sha256 = sha256_json_value(&audit_trail_immutable_evidence_fixtures);
    let contract_hash_sha256 = sha256_text_value(&format!(
        "hepta-canary-operator-review-acknowledgement-activation-command-result-receipt-audit-trail-immutable-evidence-denial:v1:source={source_report_sha256}:cancellation={source_contract_hash_sha256}:ordering={source_ordering_report_sha256}:fixtures={fixtures_sha256}:audit=0:evidence=0:hashchain=0:authority=0:live=0"
    ));
    let policy_hash_sha256 = sha256_text_value(
        "memory-intelligence-kg-operator-canary-harness-operator-review-acknowledgement-activation-command-result-receipt-audit-trail-immutable-evidence-denial:v1:no-audit-write:no-evidence-persist:no-hash-chain:no-attestation:no-authority:no-live",
    );
    let side_effect_hash_sha256 = sha256_text_value(
        "operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_side_effects=false;fixtures=10;audit=0;evidence=0;hashchain=0;attestation=0;record=0;persist=0;activation=0;provider=0;model=0;memory=0;kg=0;secret=0",
    );

    let mut denials = source
        .get("denied_by_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession")
        .and_then(serde_json::Value::as_array)
        .cloned()
        .unwrap_or_default();
    for denial in [
        "source_result_receipt_cancellation_supersession_report_required",
        "audit_trail_request_acceptance_denied",
        "audit_trail_recording_denied",
        "audit_trail_persistence_denied",
        "audit_trail_materialization_denied",
        "audit_trail_filesystem_write_denied",
        "immutable_evidence_request_acceptance_denied",
        "immutable_evidence_recording_denied",
        "immutable_evidence_persistence_denied",
        "immutable_evidence_materialization_denied",
        "immutable_evidence_filesystem_write_denied",
        "hash_chain_recording_denied",
        "merkle_root_recording_denied",
        "attestation_recording_denied",
        "witness_recording_denied",
        "notary_recording_denied",
        "ledger_index_delivery_evidence_denied",
        "export_query_observability_evidence_denied",
        "activation_from_audit_trail_denied",
        "activation_from_immutable_evidence_denied",
        "operator_approval_from_audit_trail_denied",
        "operator_approval_from_immutable_evidence_denied",
        "context_provider_model_evidence_denied",
        "memory_kg_readback_evidence_denied",
        "rollback_secret_evidence_denied",
        "external_public_install_restart_active_binary_evidence_denied",
    ] {
        denials.push(serde_json::Value::String(denial.to_string()));
    }
    let denied_count = denials.len();

    let mut report = source.clone();
    extend_json_object(
        &mut report,
        serde_json::json!({
            "status": if source_ready { "ready" } else { "blocked" },
            "source_command": "/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-audit-trail-immutable-evidence-denial --json",
            "native_route": true,
            "compatibility_mode": "native_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_status",
            "side_effect_free": true,
            "audit_date": "2026-06-13",
            "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_AUDIT_TRAIL_IMMUTABLE_EVIDENCE_DENIAL_ENDPOINT,
            "source_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_route_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_CANCELLATION_SUPERSESSION_DENIAL_ENDPOINT,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_doc": "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_AUDIT_TRAIL_IMMUTABLE_EVIDENCE_DENIAL_GATE.md",
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_route_doc": "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_AUDIT_TRAIL_IMMUTABLE_EVIDENCE_DENIAL_ROUTE_GATE.md",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-cancellation-supersession-denial-route-gate.sh",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-audit-trail-immutable-evidence-denial-gate.sh",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-audit-trail-immutable-evidence-denial-route-gate.sh",
            "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
            "route_count": route_matrix.route_count,
            "implemented_route_count": route_matrix.implemented_route_count,
            "missing_route_count": route_matrix.missing_route_count,
            "route_count_cutover_floor": NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR,
            "route_count_floor_preserved": route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR,
            "route_count_source_command_accepted": route_matrix.route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
            "source_route_wired": true,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_gate": "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_gate",
            "source_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_status": source_status,
            "source_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_route_ready": source_ready,
            "source_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_report_sha256": source_report_sha256,
            "source_cancellation_supersession_contract_hash_sha256": source_contract_hash_sha256,
            "source_cancellation_supersession_policy_hash_sha256": source_policy_hash_sha256,
            "source_ordering_monotonicity_report_sha256": source_ordering_report_sha256,
            "operator_authorization_scope": "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_no_audit_no_evidence_no_hash_chain_no_attestation_no_persist_no_authority_no_context_memory_kg_provider_model_credential_channel_install_restart_binary_or_public_release",
            "operator_authorization_received": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_route_enabled": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_status": "blocked",
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_schema_version": "memory_intelligence_kg_operator_canary_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_v1",
            "audit_trail_immutable_evidence_mode": "native_route_stdout_only_audit_trail_immutable_evidence_denial_no_record_no_persist_no_authority_no_live",
            "audit_trail_immutable_evidence_decision": "blocked_noop_activation_command_result_receipt_cannot_be_wrapped_as_audit_trail_or_immutable_evidence_authority",
            "minimum_required_samples": 24,
            "audit_trail_immutable_evidence_fixtures_sha256": fixtures_sha256,
            "audit_trail_immutable_evidence_contract_hash_sha256": contract_hash_sha256,
            "audit_trail_immutable_evidence_policy_hash_sha256": policy_hash_sha256,
            "side_effect_hash_sha256": side_effect_hash_sha256,
            "source_cancellation_supersession_fixture_count": source_u64("cancellation_supersession_fixture_count"),
            "source_blocked_cancellation_supersession_fixture_count": source_u64("blocked_cancellation_supersession_fixture_count"),
            "source_noop_cancellation_supersession_fixture_count": source_u64("noop_cancellation_supersession_fixture_count"),
            "source_accepted_cancellation_supersession_fixture_count": source_u64("accepted_cancellation_supersession_fixture_count"),
            "source_cancellation_performed_count": source_u64("cancellation_performed_count"),
            "source_supersession_performed_count": source_u64("supersession_performed_count"),
            "source_replacement_receipt_accepted_count": source_u64("replacement_receipt_accepted_count"),
            "source_replacement_receipt_recorded_count": source_u64("replacement_receipt_recorded_count"),
            "source_replacement_receipt_persisted_count": source_u64("replacement_receipt_persisted_count"),
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "audit_trail_immutable_evidence_surface_count": 12,
            "audit_trail_immutable_evidence_surface_ready_count": 12,
            "audit_trail_immutable_evidence_side_effect_free_surface_count": 12,
            "audit_trail_immutable_evidence_fixtures": audit_trail_immutable_evidence_fixtures,
            "audit_trail_immutable_evidence_fixture_count": audit_fixture_count,
            "blocked_audit_trail_immutable_evidence_fixture_count": audit_fixture_count,
            "noop_audit_trail_immutable_evidence_fixture_count": audit_fixture_count,
            "allowed_audit_trail_immutable_evidence_fixture_count": 0,
            "accepted_audit_trail_immutable_evidence_fixture_count": 0,
            "audit_trail_denied_count": audit_trail_denied_count,
            "immutable_evidence_denied_count": immutable_evidence_denied_count,
            "audit_trail_performed_count": 0,
            "immutable_evidence_performed_count": 0,
            "hash_chain_recorded_count": 0,
            "merkle_root_recorded_count": 0,
            "attestation_recorded_count": 0,
            "witness_recorded_count": 0,
            "notary_recorded_count": 0,
            "ledger_evidence_recorded_count": 0,
            "index_evidence_recorded_count": 0,
            "delivery_evidence_recorded_count": 0,
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
            "activation_command_result_receipt_recorded",
            "activation_command_result_receipt_persisted",
            "activation_command_result_receipt_accepted",
            "activation_command_result_receipt_materialized",
            "activation_command_completion_ack_recorded",
            "activation_command_completion_ack_accepted",
            "operator_approval_from_audit_trail_accepted",
            "operator_approval_from_immutable_evidence_accepted",
            "activation_from_audit_trail_allowed",
            "activation_from_immutable_evidence_allowed",
            "activation_from_cancellation_allowed",
            "activation_from_supersession_allowed",
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
            "operator_approval_recorded",
        ] {
            report_object.insert(key.to_string(), serde_json::json!(false));
        }
    }
    extend_json_object(
        &mut report,
        serde_json::json!({
            "dispatch_performed_count": 0,
            "execution_performed_count": 0,
            "context_injection_performed_count": 0,
            "provider_invoked_count": 0,
            "model_invoked_count": 0,
            "memory_store_write_performed_count": 0,
            "external_kg_adapter_read_performed_count": 0,
            "live_kg_write_performed_count": 0,
            "readback_evidence_recorded_count": 0,
            "credential_read_count": 0,
            "secret_file_read_count": 0,
            "channel_send_performed_count": 0,
            "install_performed_count": 0,
            "service_restarted_count": 0,
            "active_binary_mutated_count": 0,
            "upstream_fetch_performed_count": 0,
            "upstream_merge_performed_count": 0,
            "canary_harness_armed": false,
            "canary_harness_executable": false,
            "canary_live_enabled": false,
            "live_mutation_enabled_count": 1,
            "current_live_enabled_lane_count": 22,
            "enablement_lane_count": 25,
            "ready_enablement_lane_count": 25,
            "denied_by_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence": denials,
            "denied_by_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_count": denied_count,
            "allowed_next_actions": [
                {
                    "action": "review_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial",
                    "status": "allowed_report_only",
                    "writes_audit_trail": false,
                    "persists_evidence": false,
                    "mutates_runtime": false,
                    "invokes_model": false,
                    "writes_memory_or_kg": false
                },
                {
                    "action": "stage_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_denial",
                    "status": "allowed_report_only_next_slice",
                    "writes_audit_trail": false,
                    "persists_evidence": false,
                    "performs_retention": false,
                    "performs_gc": false,
                    "mutates_runtime": false,
                    "invokes_model": false,
                    "writes_memory_or_kg": false
                }
            ],
        }),
    );
    if let Some(side_effects) = report
        .get_mut("side_effects")
        .and_then(serde_json::Value::as_object_mut)
    {
        for key in [
            "workspace_written",
            "filesystem_written",
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
            "activation_command_result_receipt_index_evidence_recorded",
            "activation_command_result_receipt_delivery_evidence_recorded",
            "activation_command_result_receipt_recorded",
            "activation_command_result_receipt_persisted",
            "activation_command_result_receipt_accepted",
            "activation_command_completion_ack_recorded",
            "activation_command_completion_ack_accepted",
            "operator_approval_from_audit_trail_accepted",
            "operator_approval_from_immutable_evidence_accepted",
            "activation_from_audit_trail_allowed",
            "activation_from_immutable_evidence_allowed",
            "activation_command_enabled",
            "activation_command_invoked",
            "activation_command_dispatched",
            "activation_request_recorded",
            "activation_request_persisted",
            "activation_request_executed",
            "operator_approval_recorded",
            "dispatch_performed",
            "execution_performed",
            "context_injection_performed",
            "provider_invoked",
            "model_invoked",
            "provider_prompt_replayed",
            "memory_store_write_performed",
            "memory_store_mutated",
            "external_kg_adapter_read_performed",
            "live_kg_write_performed",
            "readback_evidence_recorded",
            "credential_read",
            "secret_file_read",
            "auth_secret_read",
            "channel_send_performed",
            "telegram_send_performed",
            "external_send_performed",
            "public_claim_performed",
            "install_performed",
            "service_restarted",
            "active_binary_mutated",
            "upstream_fetch_performed",
            "upstream_merge_performed",
        ] {
            side_effects.insert(key.to_string(), serde_json::json!(false));
        }
    }
    report
}

fn hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source = hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_report();
    let source_status = source
        .get("operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_status")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("unknown")
        .to_string();
    let source_ready = source_status == "blocked"
        && source
            .get("operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false);
    let source_u64 = |key: &str| {
        source
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_report_sha256 = sha256_json_value(&source);
    let source_contract_hash_sha256 = source
        .get("audit_trail_immutable_evidence_contract_hash_sha256")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("")
        .to_string();
    let source_policy_hash_sha256 = source
        .get("audit_trail_immutable_evidence_policy_hash_sha256")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("")
        .to_string();

    let retention_gc_fixture =
        |id: &str, status: &str, reason: &str, extra: serde_json::Value| -> serde_json::Value {
            let mut fixture = serde_json::Map::new();
            fixture.insert("fixture_id".to_string(), serde_json::json!(id));
            fixture.insert("retention_gc_status".to_string(), serde_json::json!(status));
            fixture.insert(
                "source_audit_trail_immutable_evidence_present".to_string(),
                serde_json::json!(true),
            );
            fixture.insert(
                "source_audit_trail_immutable_evidence_ready".to_string(),
                serde_json::json!(true),
            );
            fixture.insert("retention_requested".to_string(), serde_json::json!(true));
            fixture.insert("expiry_requested".to_string(), serde_json::json!(false));
            fixture.insert(
                "garbage_collection_requested".to_string(),
                serde_json::json!(false),
            );
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
                "audit_trail_recorded",
                "audit_trail_persisted",
                "immutable_evidence_recorded",
                "immutable_evidence_persisted",
                "hash_chain_recorded",
                "merkle_root_recorded",
                "attestation_recorded",
                "witness_recorded",
                "notary_recorded",
                "activation_command_result_receipt_recorded",
                "activation_command_result_receipt_persisted",
                "activation_command_result_receipt_accepted",
                "activation_command_result_receipt_materialized",
                "activation_command_result_receipt_filesystem_written",
                "activation_command_completion_ack_recorded",
                "activation_command_completion_ack_persisted",
                "activation_command_completion_ack_accepted",
                "operator_approval_from_retention_accepted",
                "operator_approval_from_expiry_accepted",
                "operator_approval_from_garbage_collection_accepted",
                "activation_from_retention_allowed",
                "activation_from_expiry_allowed",
                "activation_from_garbage_collection_allowed",
                "activation_from_receipt_allowed",
                "activation_command_allowed",
                "activation_command_enabled",
                "activation_command_invoked",
                "activation_command_dispatched",
                "activation_request_accepted",
                "activation_request_recorded",
                "activation_request_persisted",
                "activation_request_executed",
                "operator_approval_recorded",
                "dispatch_performed",
                "execution_performed",
                "context_injection_performed",
                "provider_invoked",
                "model_invoked",
                "provider_prompt_replayed",
                "memory_store_write_performed",
                "memory_store_mutated",
                "external_kg_adapter_read_performed",
                "live_kg_write_performed",
                "readback_evidence_recorded",
                "readback_evidence_persisted",
                "rollback_executed",
                "credential_read",
                "secret_file_read",
                "auth_secret_read",
                "channel_send_performed",
                "telegram_send_performed",
                "external_send_performed",
                "public_release_claimed",
                "public_ga_claimed",
                "release_artifact_written",
                "install_executed",
                "launchd_mutated",
                "service_restarted",
                "active_binary_mutated",
                "upstream_fetch_performed",
                "upstream_merge_performed",
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(false));
            }
            fixture.insert(
                "receipt_noop_confirmed".to_string(),
                serde_json::json!(true),
            );
            fixture.insert("denial_reason".to_string(), serde_json::json!(reason));
            let mut fixture = serde_json::Value::Object(fixture);
            extend_json_object(&mut fixture, extra);
            fixture
        };

    let retention_expiry_garbage_collection_fixtures = serde_json::Value::Array(vec![
        retention_gc_fixture(
            "missing-source-audit-trail-immutable-evidence-report",
            "blocked_noop",
            "source_audit_trail_immutable_evidence_report_required",
            serde_json::json!({
                "source_audit_trail_immutable_evidence_present": false,
                "source_audit_trail_immutable_evidence_ready": false,
            }),
        ),
        retention_gc_fixture(
            "retention-policy-write-request",
            "blocked_noop",
            "retention_policy_write_request_denied",
            serde_json::json!({
                "retention_policy_request_shape": "record_blocked_noop_receipt_retention_policy",
            }),
        ),
        retention_gc_fixture(
            "retention-index-record-request",
            "blocked_noop",
            "retention_index_recording_denied",
            serde_json::json!({
                "retention_index_requested": true,
            }),
        ),
        retention_gc_fixture(
            "expiry-scheduler-timer-request",
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
            "ttl-update-extension-request",
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
            "garbage-collection-scan-request",
            "blocked_gc_noop",
            "garbage_collection_scan_denied",
            serde_json::json!({
                "retention_requested": false,
                "garbage_collection_requested": true,
                "garbage_collection_scan_requested": true,
            }),
        ),
        retention_gc_fixture(
            "delete-tombstone-sweep-request",
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
            "archive-compaction-request",
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
            "activation-provider-memory-kg-retention-gc-attempt",
            "blocked_gc_noop",
            "activation_provider_memory_kg_retention_gc_denied",
            serde_json::json!({
                "retention_requested": false,
                "expiry_requested": true,
                "garbage_collection_requested": true,
                "activation_from_retention_gc_requested": true,
                "provider_prompt_gc_evidence_requested": true,
                "memory_store_gc_evidence_requested": true,
                "external_kg_gc_evidence_requested": true,
                "live_kg_gc_evidence_requested": true,
                "readback_gc_evidence_requested": true,
            }),
        ),
        retention_gc_fixture(
            "rollback-secret-external-public-install-retention-gc-attempt",
            "blocked_gc_noop",
            "rollback_secret_external_public_install_retention_gc_denied",
            serde_json::json!({
                "retention_requested": false,
                "expiry_requested": true,
                "garbage_collection_requested": true,
                "ledger_retention_requested": true,
                "index_retention_requested": true,
                "delivery_retention_requested": true,
                "rollback_gc_evidence_requested": true,
                "credential_secret_gc_evidence_requested": true,
                "external_send_gc_evidence_requested": true,
                "public_claim_gc_evidence_requested": true,
                "release_artifact_gc_evidence_requested": true,
                "install_gc_evidence_requested": true,
                "service_restart_gc_evidence_requested": true,
                "active_binary_gc_evidence_requested": true,
                "upstream_gc_evidence_requested": true,
            }),
        ),
    ]);
    let retention_gc_fixture_count = retention_expiry_garbage_collection_fixtures
        .as_array()
        .map(std::vec::Vec::len)
        .unwrap_or(0);
    let fixtures_sha256 = sha256_json_value(&retention_expiry_garbage_collection_fixtures);
    let contract_hash_sha256 = sha256_text_value(&format!(
        "hepta-canary-operator-review-acknowledgement-activation-command-result-receipt-retention-expiry-garbage-collection-denial:v1:source={source_report_sha256}:audit={source_contract_hash_sha256}:fixtures={fixtures_sha256}:retention=0:expiry=0:gc=0:delete=0:authority=0:live=0"
    ));
    let policy_hash_sha256 = sha256_text_value(
        "memory-intelligence-kg-operator-canary-harness-operator-review-acknowledgement-activation-command-result-receipt-retention-expiry-garbage-collection-denial:v1:no-retention:no-expiry:no-gc:no-delete:no-archive:no-authority:no-live",
    );
    let side_effect_hash_sha256 = sha256_text_value(
        "operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_side_effects=false;fixtures=10;retention=0;expiry=0;gc=0;delete=0;record=0;persist=0;activation=0;provider=0;model=0;memory=0;kg=0;secret=0",
    );

    let mut denials = source
        .get("denied_by_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence")
        .and_then(serde_json::Value::as_array)
        .cloned()
        .unwrap_or_default();
    for denial in [
        "source_audit_trail_immutable_evidence_report_required",
        "retention_policy_request_acceptance_denied",
        "retention_policy_recording_denied",
        "retention_policy_persistence_denied",
        "retention_policy_materialization_denied",
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
        "delete_execution_denied",
        "delete_marker_recording_denied",
        "tombstone_recording_denied",
        "sweep_execution_denied",
        "archive_write_denied",
        "compaction_execution_denied",
        "ledger_retention_recording_denied",
        "index_retention_recording_denied",
        "delivery_retention_recording_denied",
        "operator_approval_from_retention_expiry_gc_denied",
        "activation_from_retention_expiry_gc_denied",
        "provider_model_memory_kg_gc_evidence_denied",
        "rollback_secret_external_public_install_restart_active_binary_gc_denied",
    ] {
        denials.push(serde_json::Value::String(denial.to_string()));
    }
    let denied_count = denials.len();

    let mut report = source.clone();
    extend_json_object(
        &mut report,
        serde_json::json!({
            "status": if source_ready { "ready" } else { "blocked" },
            "gate": "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_denial_gate",
            "source_command": "/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-retention-expiry-garbage-collection-denial --json",
            "native_route": true,
            "compatibility_mode": "native_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_denial_status",
            "side_effect_free": true,
            "audit_date": "2026-06-14",
            "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_RETENTION_EXPIRY_GARBAGE_COLLECTION_DENIAL_ENDPOINT,
            "source_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_route_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_AUDIT_TRAIL_IMMUTABLE_EVIDENCE_DENIAL_ENDPOINT,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_denial_doc": "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_RETENTION_EXPIRY_GARBAGE_COLLECTION_DENIAL_GATE.md",
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_denial_route_doc": "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_RETENTION_EXPIRY_GARBAGE_COLLECTION_DENIAL_ROUTE_GATE.md",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-audit-trail-immutable-evidence-denial-route-gate.sh",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_denial_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-retention-expiry-garbage-collection-denial-gate.sh",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_denial_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-retention-expiry-garbage-collection-denial-route-gate.sh",
            "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
            "route_count": route_matrix.route_count,
            "implemented_route_count": route_matrix.implemented_route_count,
            "missing_route_count": route_matrix.missing_route_count,
            "route_count_cutover_floor": NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR,
            "route_count_floor_preserved": route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR,
            "route_count_source_command_accepted": route_matrix.route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
            "source_route_wired": true,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_gate": "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_gate",
            "source_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_status": source_status,
            "source_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_route_ready": source_ready,
            "source_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_report_sha256": source_report_sha256,
            "source_audit_trail_immutable_evidence_contract_hash_sha256": source_contract_hash_sha256,
            "source_audit_trail_immutable_evidence_policy_hash_sha256": source_policy_hash_sha256,
            "operator_authorization_scope": "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_denial_no_retention_no_expiry_no_gc_no_delete_no_archive_no_compaction_no_authority_no_context_memory_kg_provider_model_credential_channel_install_restart_binary_or_public_release",
            "operator_authorization_received": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_denial_route_enabled": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_denial_status": "blocked",
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_schema_version": "memory_intelligence_kg_operator_canary_harness_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_denial_v1",
            "retention_expiry_garbage_collection_mode": "native_route_stdout_only_retention_expiry_garbage_collection_denial_no_schedule_no_scan_no_delete_no_authority_no_live",
            "retention_expiry_garbage_collection_decision": "blocked_noop_activation_command_result_receipt_cannot_be_retained_expired_garbage_collected_or_deleted_into_authority",
            "minimum_required_samples": 24,
            "retention_expiry_garbage_collection_fixtures_sha256": fixtures_sha256,
            "retention_expiry_garbage_collection_contract_hash_sha256": contract_hash_sha256,
            "retention_expiry_garbage_collection_policy_hash_sha256": policy_hash_sha256,
            "side_effect_hash_sha256": side_effect_hash_sha256,
            "source_audit_trail_immutable_evidence_fixture_count": source_u64("audit_trail_immutable_evidence_fixture_count"),
            "source_blocked_audit_trail_immutable_evidence_fixture_count": source_u64("blocked_audit_trail_immutable_evidence_fixture_count"),
            "source_accepted_audit_trail_immutable_evidence_fixture_count": source_u64("accepted_audit_trail_immutable_evidence_fixture_count"),
            "source_audit_trail_performed_count": source_u64("audit_trail_performed_count"),
            "source_immutable_evidence_performed_count": source_u64("immutable_evidence_performed_count"),
            "source_hash_chain_recorded_count": source_u64("hash_chain_recorded_count"),
            "source_attestation_recorded_count": source_u64("attestation_recorded_count"),
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "retention_expiry_garbage_collection_surface_count": 12,
            "retention_expiry_garbage_collection_surface_ready_count": 12,
            "retention_expiry_garbage_collection_side_effect_free_surface_count": 12,
            "retention_expiry_garbage_collection_fixtures": retention_expiry_garbage_collection_fixtures,
            "retention_expiry_garbage_collection_fixture_count": retention_gc_fixture_count,
            "blocked_retention_expiry_garbage_collection_fixture_count": retention_gc_fixture_count,
            "noop_retention_expiry_garbage_collection_fixture_count": retention_gc_fixture_count,
            "allowed_retention_expiry_garbage_collection_fixture_count": 0,
            "accepted_retention_expiry_garbage_collection_fixture_count": 0,
            "retention_denied_count": retention_gc_fixture_count,
            "expiry_denied_count": retention_gc_fixture_count,
            "garbage_collection_denied_count": retention_gc_fixture_count,
            "retention_performed_count": 0,
            "expiry_performed_count": 0,
            "garbage_collection_performed_count": 0,
            "delete_performed_count": 0,
            "archive_written_count": 0,
            "compaction_performed_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "dispatch_performed_count": 0,
            "execution_performed_count": 0,
            "context_injection_performed_count": 0,
            "provider_invoked_count": 0,
            "model_invoked_count": 0,
            "memory_store_write_performed_count": 0,
            "external_kg_adapter_read_performed_count": 0,
            "live_kg_write_performed_count": 0,
            "readback_evidence_recorded_count": 0,
            "credential_read_count": 0,
            "secret_file_read_count": 0,
            "channel_send_performed_count": 0,
            "install_performed_count": 0,
            "service_restarted_count": 0,
            "active_binary_mutated_count": 0,
            "upstream_fetch_performed_count": 0,
            "upstream_merge_performed_count": 0,
            "canary_harness_armed": false,
            "canary_harness_executable": false,
            "canary_live_enabled": false,
            "live_mutation_enabled_count": 1,
            "current_live_enabled_lane_count": 23,
            "enablement_lane_count": 26,
            "ready_enablement_lane_count": 26,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "denied_by_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection": denials,
            "denied_by_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_count": denied_count,
            "allowed_next_actions": [
                {
                    "action": "review_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_denial",
                    "status": "allowed_report_only",
                    "performs_retention": false,
                    "performs_expiry": false,
                    "performs_gc": false,
                    "deletes_receipt": false,
                    "mutates_runtime": false,
                    "invokes_model": false,
                    "writes_memory_or_kg": false
                },
                {
                    "action": "stage_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_denial",
                    "status": "allowed_report_only_next_slice",
                    "performs_retention": false,
                    "performs_expiry": false,
                    "performs_gc": false,
                    "deletes_receipt": false,
                    "mutates_runtime": false,
                    "invokes_model": false,
                    "writes_memory_or_kg": false
                }
            ],
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
            "activation_command_completion_ack_recorded",
            "activation_command_completion_ack_accepted",
            "operator_approval_from_retention_accepted",
            "operator_approval_from_expiry_accepted",
            "operator_approval_from_garbage_collection_accepted",
            "activation_allowed_by_result_receipt_retention",
            "activation_allowed_by_result_receipt_expiry",
            "activation_allowed_by_result_receipt_garbage_collection",
            "activation_allowed_by_result_receipt_audit_trail",
            "activation_allowed_by_result_receipt_immutable_evidence",
            "activation_allowed_by_result_receipt",
            "activation_command_allowed",
            "activation_command_enabled",
            "activation_command_invoked",
            "activation_command_dispatched",
            "activation_request_accepted",
            "activation_request_recorded",
            "activation_request_persisted",
            "activation_request_executed",
            "operator_approval_recorded",
        ] {
            report_object.insert(key.to_string(), serde_json::json!(false));
        }
    }
    if let Some(side_effects) = report
        .get_mut("side_effects")
        .and_then(serde_json::Value::as_object_mut)
    {
        for key in [
            "workspace_written",
            "filesystem_written",
            "retention_policy_recorded",
            "retention_policy_persisted",
            "retention_index_recorded",
            "expiry_recorded",
            "expiry_scheduler_registered",
            "expiry_timer_started",
            "ttl_update_recorded",
            "ttl_extension_recorded",
            "garbage_collection_scan_performed",
            "garbage_collection_candidate_recorded",
            "garbage_collection_decision_recorded",
            "delete_performed",
            "delete_marker_recorded",
            "tombstone_recorded",
            "sweep_performed",
            "archive_written",
            "compaction_performed",
            "ledger_retention_recorded",
            "index_retention_recorded",
            "delivery_retention_recorded",
            "activation_command_result_receipt_recorded",
            "activation_command_result_receipt_persisted",
            "activation_command_result_receipt_accepted",
            "activation_command_completion_ack_recorded",
            "activation_command_enabled",
            "activation_command_invoked",
            "activation_command_dispatched",
            "operator_approval_recorded",
            "dispatch_performed",
            "execution_performed",
            "context_injection_performed",
            "provider_invoked",
            "model_invoked",
            "memory_store_write_performed",
            "memory_store_mutated",
            "external_kg_adapter_read_performed",
            "live_kg_write_performed",
            "credential_read",
            "secret_file_read",
            "channel_send_performed",
            "telegram_send_performed",
            "external_send_performed",
            "public_claim_performed",
            "release_artifact_written",
            "install_performed",
            "service_restarted",
            "active_binary_mutated",
            "upstream_fetch_performed",
            "upstream_merge_performed",
        ] {
            side_effects.insert(key.to_string(), serde_json::json!(false));
        }
    }
    report
}

fn hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_retention =
        hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_denial_report();
    let source_bool = |key: &str| {
        source_retention
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let source_u64 = |key: &str| {
        source_retention
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_status = source_retention
        .get("status")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("blocked");
    let route_count_floor_preserved =
        route_matrix.route_count >= NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_retention_expiry_gc_ready = source_status == "ready"
        && source_bool(
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_denial_route_enabled",
        )
        && source_bool(
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready",
        )
        && source_retention
            .get("operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_denial_status")
            .and_then(serde_json::Value::as_str)
            == Some("blocked")
        && source_u64("retention_expiry_garbage_collection_fixture_count") == 10
        && source_u64("blocked_retention_expiry_garbage_collection_fixture_count") == 10
        && source_u64("noop_retention_expiry_garbage_collection_fixture_count") == 10
        && source_u64("accepted_retention_expiry_garbage_collection_fixture_count") == 0
        && source_u64("retention_performed_count") == 0
        && source_u64("expiry_performed_count") == 0
        && source_u64("garbage_collection_performed_count") == 0
        && source_u64("delete_performed_count") == 0
        && source_u64("archive_written_count") == 0
        && source_u64("compaction_performed_count") == 0
        && !source_bool("activation_command_result_receipt_retention_policy_recorded")
        && !source_bool("activation_command_result_receipt_retention_index_recorded")
        && !source_bool("activation_command_result_receipt_expiry_scheduler_registered")
        && !source_bool("activation_command_result_receipt_expiry_timer_started")
        && !source_bool("activation_command_result_receipt_garbage_collection_scan_performed")
        && !source_bool("activation_command_result_receipt_delete_performed")
        && !source_bool("activation_command_result_receipt_tombstone_recorded")
        && !source_bool("activation_command_result_receipt_archive_written")
        && !source_bool("activation_command_result_receipt_compaction_performed")
        && !source_bool("activation_command_result_receipt_recorded")
        && !source_bool("activation_command_result_receipt_persisted")
        && !source_bool("activation_command_result_receipt_accepted")
        && !source_bool("activation_allowed_by_result_receipt_retention")
        && !source_bool("activation_allowed_by_result_receipt_expiry")
        && !source_bool("activation_allowed_by_result_receipt_garbage_collection")
        && !source_bool("activation_command_invoked")
        && !source_bool("activation_command_dispatched")
        && !source_bool("activation_request_accepted")
        && !source_bool("activation_request_executed")
        && source_u64("dispatch_performed_count") == 0
        && source_u64("execution_performed_count") == 0
        && source_u64("provider_invoked_count") == 0
        && source_u64("model_invoked_count") == 0
        && source_u64("memory_store_write_performed_count") == 0
        && source_u64("external_kg_adapter_read_performed_count") == 0
        && source_u64("live_kg_write_performed_count") == 0
        && source_u64("credential_read_count") == 0
        && source_u64("secret_file_read_count") == 0
        && source_u64("channel_send_performed_count") == 0
        && !source_bool("canary_harness_armed")
        && !source_bool("canary_harness_executable")
        && !source_bool("canary_live_enabled");
    let report_ready = route_matrix.ready
        && route_count_floor_preserved
        && route_count_source_command_accepted
        && source_retention_expiry_gc_ready;

    let export_query_observability_fixture =
        |fixture_id: &str, status: &str, denial_reason: &str, extra: serde_json::Value| {
            let mut fixture = serde_json::Map::new();
            fixture.insert(
                "fixture_id".to_string(),
                serde_json::Value::String(fixture_id.to_string()),
            );
            fixture.insert(
                "export_query_observability_status".to_string(),
                serde_json::Value::String(status.to_string()),
            );
            fixture.insert(
                "denial_reason".to_string(),
                serde_json::Value::String(denial_reason.to_string()),
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
                "operator_approval_from_export_accepted",
                "operator_approval_from_query_accepted",
                "operator_approval_from_observability_accepted",
                "activation_from_export_allowed",
                "activation_from_query_allowed",
                "activation_from_observability_allowed",
                "activation_from_retention_allowed",
                "activation_from_expiry_allowed",
                "activation_from_garbage_collection_allowed",
                "activation_command_allowed",
                "activation_command_enabled",
                "activation_command_invoked",
                "activation_command_dispatched",
                "activation_request_accepted",
                "activation_request_recorded",
                "activation_request_persisted",
                "activation_request_executed",
                "operator_approval_recorded",
                "dispatch_performed",
                "execution_performed",
                "runtime_router_mutated",
                "runtime_attachment_performed",
                "live_context_attached",
                "context_injection_performed",
                "adapter_invoked",
                "provider_invoked",
                "model_invoked",
                "provider_prompt_replayed",
                "usage_recorded",
                "memory_store_write_performed",
                "memory_store_mutated",
                "external_kg_adapter_read_performed",
                "live_kg_write_performed",
                "readback_evidence_recorded",
                "readback_evidence_persisted",
                "router_handoff_recorded",
                "router_handoff_persisted",
                "rollback_executed",
                "auth_secret_read",
                "credential_read",
                "secret_file_read",
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
                "upstream_fetch_performed",
                "upstream_merge_performed",
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(false));
            }
            let mut fixture = serde_json::Value::Object(fixture);
            extend_json_object(&mut fixture, extra);
            fixture
        };
    let export_query_observability_fixtures = serde_json::Value::Array(vec![
        export_query_observability_fixture(
            "missing-source-retention-expiry-garbage-collection-report",
            "blocked_noop",
            "source_retention_expiry_garbage_collection_report_required",
            serde_json::json!({
                "source_retention_expiry_gc_present": false,
                "source_retention_expiry_gc_ready": false,
                "export_requested": true,
            }),
        ),
        export_query_observability_fixture(
            "export-artifact-request",
            "blocked_export_noop",
            "export_artifact_write_denied",
            serde_json::json!({"export_requested": true, "export_file_requested": true}),
        ),
        export_query_observability_fixture(
            "export-stream-request",
            "blocked_export_noop",
            "export_stream_open_denied",
            serde_json::json!({"export_requested": true, "export_stream_requested": true}),
        ),
        export_query_observability_fixture(
            "query-endpoint-request",
            "blocked_query_noop",
            "query_endpoint_materialization_denied",
            serde_json::json!({"query_requested": true, "query_endpoint_requested": true}),
        ),
        export_query_observability_fixture(
            "query-index-cache-request",
            "blocked_query_noop",
            "query_index_cache_recording_denied",
            serde_json::json!({
                "query_requested": true,
                "query_index_requested": true,
                "query_cache_requested": true,
            }),
        ),
        export_query_observability_fixture(
            "observability-metric-request",
            "blocked_observability_noop",
            "observability_metric_emission_denied",
            serde_json::json!({"observability_requested": true, "metric_requested": true}),
        ),
        export_query_observability_fixture(
            "observability-trace-log-event-request",
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
            "dashboard-alert-slo-request",
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
            "activation-provider-memory-kg-observability",
            "blocked_observability_noop",
            "activation_provider_memory_kg_observability_denied",
            serde_json::json!({
                "observability_requested": true,
                "activation_from_observability_requested": true,
                "memory_store_observability_requested": true,
                "external_kg_observability_requested": true,
                "live_kg_observability_requested": true,
                "rollback_observability_requested": true,
                "secret_material_observability_requested": true,
                "provider_prompt_observability_requested": true,
            }),
        ),
        export_query_observability_fixture(
            "ledger-index-delivery-external-public-install-observability",
            "blocked_observability_noop",
            "ledger_index_delivery_external_public_install_observability_denied",
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
                "upstream_observability_requested": true,
            }),
        ),
    ]);
    let export_query_observability_fixture_count = export_query_observability_fixtures
        .as_array()
        .map(std::vec::Vec::len)
        .unwrap_or(0);
    let mut denials = source_retention
        .get("denied_by_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection")
        .and_then(serde_json::Value::as_array)
        .cloned()
        .unwrap_or_default();
    for denial in [
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
        "operator_approval_from_export_query_observability_denied",
        "activation_from_export_query_observability_denied",
        "provider_model_memory_kg_observability_denied",
        "rollback_secret_external_public_install_restart_active_binary_observability_denied",
    ] {
        denials.push(serde_json::Value::String(denial.to_string()));
    }
    let denied_count = denials.len();

    let mut report = source_retention.clone();
    extend_json_object(
        &mut report,
        serde_json::json!({
            "status": if report_ready { "ready" } else { "blocked" },
            "source_command": "/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-export-query-observability-denial --json",
            "native_route": true,
            "compatibility_mode": "native_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_denial_status",
            "side_effect_free": true,
            "audit_date": "2026-06-14",
            "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_EXPORT_QUERY_OBSERVABILITY_DENIAL_ENDPOINT,
            "source_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_route_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_RETENTION_EXPIRY_GARBAGE_COLLECTION_DENIAL_ENDPOINT,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_route_doc": "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_EXPORT_QUERY_OBSERVABILITY_DENIAL_ROUTE_GATE.md",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-retention-expiry-garbage-collection-denial-route-gate.sh",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-export-query-observability-denial-gate.sh",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-export-query-observability-denial-route-gate.sh",
            "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
            "route_count": route_matrix.route_count,
            "implemented_route_count": route_matrix.implemented_route_count,
            "missing_route_count": route_matrix.missing_route_count,
            "route_count_cutover_floor": NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR,
            "route_count_floor_preserved": route_count_floor_preserved,
            "route_count_source_command_accepted": route_count_source_command_accepted,
            "source_route_wired": true,
            "source_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_route_status": source_status,
            "source_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_route_ready": source_retention_expiry_gc_ready,
            "operator_authorization_scope": "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_no_export_no_query_no_metrics_logs_traces_events_dashboards_alerts_slo_no_authority_no_context_memory_kg_provider_model_credential_channel_install_restart_binary_or_public_release",
            "operator_authorization_received": true,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_denial_route_enabled": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_denial_ready": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_denial_status": "blocked",
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_schema_version": "memory_intelligence_kg_operator_canary_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_denial_v1",
            "export_query_observability_mode": "native_route_stdout_only_export_query_observability_denial_no_export_no_query_no_observability_no_authority_no_live",
            "export_query_observability_decision": "blocked_noop_activation_command_result_receipt_cannot_be_exported_queried_or_observed_into_authority",
            "source_retention_expiry_garbage_collection_fixture_count": source_u64("retention_expiry_garbage_collection_fixture_count"),
            "source_blocked_retention_expiry_garbage_collection_fixture_count": source_u64("blocked_retention_expiry_garbage_collection_fixture_count"),
            "source_accepted_retention_expiry_garbage_collection_fixture_count": source_u64("accepted_retention_expiry_garbage_collection_fixture_count"),
            "source_retention_performed_count": source_u64("retention_performed_count"),
            "source_expiry_performed_count": source_u64("expiry_performed_count"),
            "source_garbage_collection_performed_count": source_u64("garbage_collection_performed_count"),
            "export_query_observability_surface_count": 12,
            "export_query_observability_surface_ready_count": 12,
            "export_query_observability_side_effect_free_surface_count": 12,
            "export_query_observability_fixtures": export_query_observability_fixtures,
            "export_query_observability_fixture_count": export_query_observability_fixture_count,
            "blocked_export_query_observability_fixture_count": export_query_observability_fixture_count,
            "noop_export_query_observability_fixture_count": export_query_observability_fixture_count,
            "allowed_export_query_observability_fixture_count": 0,
            "accepted_export_query_observability_fixture_count": 0,
            "export_denied_count": export_query_observability_fixture_count,
            "query_denied_count": export_query_observability_fixture_count,
            "observability_denied_count": export_query_observability_fixture_count,
            "export_performed_count": 0,
            "query_performed_count": 0,
            "observability_performed_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "activation_command_result_receipt_export_allowed": false,
            "activation_command_result_receipt_export_request_accepted": false,
            "activation_command_result_receipt_export_recorded": false,
            "activation_command_result_receipt_export_persisted": false,
            "activation_command_result_receipt_export_artifact_written": false,
            "activation_command_result_receipt_export_stream_opened": false,
            "activation_command_result_receipt_export_filesystem_written": false,
            "activation_command_result_receipt_query_allowed": false,
            "activation_command_result_receipt_query_registered": false,
            "activation_command_result_receipt_query_endpoint_materialized": false,
            "activation_command_result_receipt_query_index_recorded": false,
            "activation_command_result_receipt_query_cache_written": false,
            "activation_command_result_receipt_query_result_materialized": false,
            "activation_command_result_receipt_observability_allowed": false,
            "activation_command_result_receipt_observability_metric_emitted": false,
            "activation_command_result_receipt_observability_log_recorded": false,
            "activation_command_result_receipt_observability_trace_recorded": false,
            "activation_command_result_receipt_observability_span_recorded": false,
            "activation_command_result_receipt_observability_event_recorded": false,
            "activation_command_result_receipt_observability_dashboard_materialized": false,
            "activation_command_result_receipt_observability_alert_registered": false,
            "activation_command_result_receipt_observability_slo_recorded": false,
            "activation_command_result_receipt_ledger_observability_recorded": false,
            "activation_command_result_receipt_index_observability_recorded": false,
            "activation_command_result_receipt_delivery_observability_recorded": false,
            "operator_approval_from_export_accepted": false,
            "operator_approval_from_query_accepted": false,
            "operator_approval_from_observability_accepted": false,
            "activation_allowed_by_result_receipt_export": false,
            "activation_allowed_by_result_receipt_query": false,
            "activation_allowed_by_result_receipt_observability": false,
            "activation_allowed_by_result_receipt": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "activation_command_enabled": false,
            "activation_command_invoked": false,
            "activation_command_dispatched": false,
            "activation_request_accepted": false,
            "activation_request_recorded": false,
            "activation_request_persisted": false,
            "activation_request_executed": false,
            "operator_approval_recorded": false,
            "dispatch_performed_count": 0,
            "execution_performed_count": 0,
            "runtime_router_mutated_count": 0,
            "runtime_attachment_performed_count": 0,
            "live_context_attached_count": 0,
            "context_injection_performed_count": 0,
            "adapter_invoked_count": 0,
            "provider_invoked_count": 0,
            "model_invoked_count": 0,
            "memory_store_write_performed_count": 0,
            "external_kg_adapter_read_performed_count": 0,
            "live_kg_write_performed_count": 0,
            "credential_read_count": 0,
            "secret_file_read_count": 0,
            "channel_send_performed_count": 0,
            "install_performed_count": 0,
            "service_restarted_count": 0,
            "active_binary_mutated_count": 0,
            "upstream_fetch_performed_count": 0,
            "upstream_merge_performed_count": 0,
            "canary_harness_armed": false,
            "canary_harness_executable": false,
            "canary_live_enabled": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "denied_by_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability": denials,
            "denied_by_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_count": denied_count,
            "current_live_enabled_lane_count": 24,
            "enablement_lane_count": 27,
            "ready_enablement_lane_count": 27,
            "allowed_next_actions": [
                {
                    "action": "review_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_denial",
                    "status": "allowed_report_only",
                    "exports_receipt": false,
                    "registers_query": false,
                    "records_observability": false,
                    "mutates_runtime": false,
                    "invokes_model": false,
                    "writes_memory_or_kg": false
                },
                {
                    "action": "stage_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial",
                    "status": "allowed_report_only_next_slice",
                    "persists_summary": false,
                    "persists_briefing": false,
                    "delivers_summary": false,
                    "mutates_runtime": false,
                    "invokes_model": false,
                    "writes_memory_or_kg": false
                }
            ],
        }),
    );
    if let Some(side_effects) = report
        .get_mut("side_effects")
        .and_then(serde_json::Value::as_object_mut)
    {
        for key in [
            "workspace_written",
            "filesystem_written",
            "export_recorded",
            "export_persisted",
            "export_artifact_written",
            "export_stream_opened",
            "query_registered",
            "query_endpoint_materialized",
            "query_index_recorded",
            "query_cache_written",
            "observability_metric_emitted",
            "observability_log_recorded",
            "observability_trace_recorded",
            "observability_span_recorded",
            "observability_event_recorded",
            "observability_dashboard_materialized",
            "observability_alert_registered",
            "observability_slo_recorded",
            "activation_command_result_receipt_recorded",
            "activation_command_result_receipt_persisted",
            "activation_command_result_receipt_accepted",
            "activation_command_completion_ack_recorded",
            "operator_approval_recorded",
            "dispatch_performed",
            "execution_performed",
            "context_injection_performed",
            "provider_invoked",
            "model_invoked",
            "memory_store_write_performed",
            "memory_store_mutated",
            "external_kg_adapter_read_performed",
            "live_kg_write_performed",
            "credential_read",
            "secret_file_read",
            "channel_send_performed",
            "telegram_send_performed",
            "external_send_performed",
            "public_claim_performed",
            "release_artifact_written",
            "install_performed",
            "service_restarted",
            "active_binary_mutated",
            "upstream_fetch_performed",
            "upstream_merge_performed",
        ] {
            side_effects.insert(key.to_string(), serde_json::json!(false));
        }
    }
    report
}

fn hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_export =
        hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_denial_report();
    let source_bool = |key: &str| {
        source_export
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let source_u64 = |key: &str| {
        source_export
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_status = source_export
        .get("operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_denial_status")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("blocked");
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_export_query_observability_ready = source_status == "blocked"
        && source_bool(
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_denial_route_enabled",
        )
        && source_bool(
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_denial_ready",
        )
        && source_u64("export_query_observability_fixture_count") == 10
        && source_u64("blocked_export_query_observability_fixture_count") == 10
        && source_u64("noop_export_query_observability_fixture_count") == 10
        && source_u64("accepted_export_query_observability_fixture_count") == 0
        && source_u64("export_performed_count") == 0
        && source_u64("query_performed_count") == 0
        && source_u64("observability_performed_count") == 0
        && !source_bool("activation_command_result_receipt_export_recorded")
        && !source_bool("activation_command_result_receipt_export_artifact_written")
        && !source_bool("activation_command_result_receipt_export_stream_opened")
        && !source_bool("activation_command_result_receipt_query_registered")
        && !source_bool("activation_command_result_receipt_query_endpoint_materialized")
        && !source_bool("activation_command_result_receipt_observability_metric_emitted")
        && !source_bool("activation_command_result_receipt_observability_log_recorded")
        && !source_bool("activation_command_result_receipt_observability_trace_recorded")
        && !source_bool("activation_command_result_receipt_observability_dashboard_materialized")
        && !source_bool("activation_command_result_receipt_observability_alert_registered")
        && !source_bool("activation_command_result_receipt_recorded")
        && !source_bool("activation_command_result_receipt_persisted")
        && !source_bool("activation_command_result_receipt_accepted")
        && !source_bool("activation_allowed_by_result_receipt_export")
        && !source_bool("activation_allowed_by_result_receipt_query")
        && !source_bool("activation_allowed_by_result_receipt_observability")
        && !source_bool("activation_command_invoked")
        && !source_bool("activation_command_dispatched")
        && source_u64("provider_invoked_count") == 0
        && source_u64("model_invoked_count") == 0
        && source_u64("memory_store_write_performed_count") == 0
        && source_u64("live_kg_write_performed_count") == 0
        && source_u64("credential_read_count") == 0
        && source_u64("secret_file_read_count") == 0
        && source_u64("channel_send_performed_count") == 0
        && source_u64("install_performed_count") == 0
        && source_u64("service_restarted_count") == 0
        && source_u64("active_binary_mutated_count") == 0;
    let report_ready = route_matrix.ready
        && route_count_source_command_accepted
        && source_export_query_observability_ready;

    let operator_summary_briefing_fixture =
        |fixture_id: &str, status: &str, denial_reason: &str, extra: serde_json::Value| {
            let mut fixture = serde_json::Map::new();
            fixture.insert(
                "fixture_id".to_string(),
                serde_json::Value::String(fixture_id.to_string()),
            );
            fixture.insert(
                "operator_summary_briefing_status".to_string(),
                serde_json::Value::String(status.to_string()),
            );
            fixture.insert(
                "denial_reason".to_string(),
                serde_json::Value::String(denial_reason.to_string()),
            );
            for key in [
                "source_export_query_observability_present",
                "source_export_query_observability_ready",
                "summary_briefing_noop_confirmed",
            ] {
                fixture.insert(key.to_string(), serde_json::Value::Bool(true));
            }
            for key in [
                "operator_summary_requested",
                "operator_briefing_requested",
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
                "activation_command_completion_ack_recorded",
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
            "operator-summary-missing-source-export-query-observability",
            "blocked_noop",
            "source_export_query_observability_report_required",
            serde_json::json!({
                "source_export_query_observability_present": false,
                "source_export_query_observability_ready": false,
                "operator_summary_requested": true,
            }),
        ),
        operator_summary_briefing_fixture(
            "operator-summary-request",
            "blocked_summary_noop",
            "operator_summary_request_shape_denied",
            serde_json::json!({"operator_summary_requested": true}),
        ),
        operator_summary_briefing_fixture(
            "operator-briefing-request",
            "blocked_briefing_noop",
            "operator_briefing_request_shape_denied",
            serde_json::json!({"operator_briefing_requested": true}),
        ),
        operator_summary_briefing_fixture(
            "operator-summary-materialization-request",
            "blocked_summary_noop",
            "operator_summary_materialization_denied",
            serde_json::json!({
                "operator_summary_requested": true,
                "operator_summary_materialization_requested": true,
            }),
        ),
        operator_summary_briefing_fixture(
            "operator-briefing-materialization-request",
            "blocked_briefing_noop",
            "operator_briefing_materialization_denied",
            serde_json::json!({
                "operator_briefing_requested": true,
                "operator_briefing_materialization_requested": true,
            }),
        ),
        operator_summary_briefing_fixture(
            "operator-summary-persistence-filesystem-request",
            "blocked_summary_noop",
            "operator_summary_persistence_filesystem_write_denied",
            serde_json::json!({
                "operator_summary_requested": true,
                "operator_summary_persistence_requested": true,
                "operator_summary_filesystem_write_requested": true,
            }),
        ),
        operator_summary_briefing_fixture(
            "operator-briefing-persistence-filesystem-request",
            "blocked_briefing_noop",
            "operator_briefing_persistence_filesystem_write_denied",
            serde_json::json!({
                "operator_briefing_requested": true,
                "operator_briefing_persistence_requested": true,
                "operator_briefing_filesystem_write_requested": true,
            }),
        ),
        operator_summary_briefing_fixture(
            "operator-summary-briefing-channel-delivery-request",
            "blocked_delivery_noop",
            "operator_summary_briefing_channel_delivery_denied",
            serde_json::json!({
                "operator_summary_requested": true,
                "operator_briefing_requested": true,
                "channel_delivery_requested": true,
                "telegram_send_requested": true,
            }),
        ),
        operator_summary_briefing_fixture(
            "operator-summary-briefing-activation-memory-kg-provider",
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
                "provider_prompt_summary_requested": true,
            }),
        ),
        operator_summary_briefing_fixture(
            "operator-summary-briefing-external-public-install",
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
                "active_binary_summary_requested": true,
            }),
        ),
    ]);
    let operator_summary_briefing_fixture_count = operator_summary_briefing_fixtures
        .as_array()
        .map(std::vec::Vec::len)
        .unwrap_or(0);
    let mut denials = source_export
        .get("denied_by_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability")
        .and_then(serde_json::Value::as_array)
        .cloned()
        .unwrap_or_default();
    for denial in [
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
    ] {
        denials.push(serde_json::Value::String(denial.to_string()));
    }
    let denied_count = denials.len();

    let mut report = source_export.clone();
    extend_json_object(
        &mut report,
        serde_json::json!({
            "gate": "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_route",
            "status": if report_ready { "ready" } else { "blocked" },
            "source_command": "/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-operator-facing-summary-briefing-non-persistence-denial --json",
            "native_route": true,
            "compatibility_mode": "native_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_status",
            "side_effect_free": true,
            "audit_date": "2026-06-14",
            "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_OPERATOR_FACING_SUMMARY_BRIEFING_NON_PERSISTENCE_DENIAL_ENDPOINT,
            "source_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_route_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_EXPORT_QUERY_OBSERVABILITY_DENIAL_ENDPOINT,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_route_doc": "docs/architecture/i3-b7b8388443e0f436f3d6f6d6.md",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-export-query-observability-denial-route-gate.sh",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-operator-facing-summary-briefing-non-persistence-denial-gate.sh",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-operator-facing-summary-briefing-non-persistence-denial-route-gate.sh",
            "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
            "route_count": route_matrix.route_count,
            "implemented_route_count": route_matrix.implemented_route_count,
            "missing_route_count": route_matrix.missing_route_count,
            "route_count_source_command_accepted": route_count_source_command_accepted,
            "source_route_wired": true,
            "source_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_route_ready": source_export_query_observability_ready,
            "operator_authorization_scope": "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_no_summary_no_briefing_no_delivery_no_authority_no_context_memory_kg_provider_model_credential_channel_install_restart_binary_or_public_release",
            "operator_authorization_received": true,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_route_enabled": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_status": "blocked",
            "activation_command_result_receipt_operator_facing_summary_briefing_schema_version": "memory_intelligence_kg_operator_canary_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_v1",
            "operator_facing_summary_briefing_mode": "native_route_stdout_only_operator_facing_summary_briefing_non_persistence_denial_no_summary_no_briefing_no_delivery_no_authority_no_live",
            "operator_facing_summary_briefing_decision": "blocked_noop_activation_command_result_receipt_cannot_be_summarized_briefed_delivered_or_promoted_into_authority",
            "source_export_query_observability_fixture_count": source_u64("export_query_observability_fixture_count"),
            "source_blocked_export_query_observability_fixture_count": source_u64("blocked_export_query_observability_fixture_count"),
            "source_accepted_export_query_observability_fixture_count": source_u64("accepted_export_query_observability_fixture_count"),
            "source_export_performed_count": source_u64("export_performed_count"),
            "source_query_performed_count": source_u64("query_performed_count"),
            "source_observability_performed_count": source_u64("observability_performed_count"),
            "operator_facing_summary_briefing_surface_count": 12,
            "operator_facing_summary_briefing_surface_ready_count": 12,
            "operator_facing_summary_briefing_side_effect_free_surface_count": 12,
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
            "telegram_send_performed": false,
            "channel_send_performed": false,
            "external_send_performed": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "activation_allowed_by_result_receipt_operator_summary": false,
            "activation_allowed_by_result_receipt_operator_briefing": false,
            "activation_allowed_by_result_receipt_summary_briefing": false,
            "activation_allowed_by_result_receipt": false,
            "activation_command_enabled": false,
            "activation_command_invoked": false,
            "activation_command_dispatched": false,
            "activation_activated": false,
            "runtime_router_mutated": false,
            "runtime_attachment_performed": false,
            "live_context_attached": false,
            "context_injection_performed": false,
            "adapter_invoked": false,
            "provider_invoked": false,
            "model_invoked": false,
            "provider_prompt_replayed": false,
            "auth_secret_read": false,
            "credential_read": false,
            "secret_file_read": false,
            "usage_recorded": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "live_kg_write_performed": false,
            "readback_evidence_recorded": false,
            "readback_evidence_persisted": false,
            "router_handoff_recorded": false,
            "router_handoff_persisted": false,
            "rollback_executed": false,
            "public_release_claimed": false,
            "public_ga_claimed": false,
            "release_artifact_written": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restart_performed": false,
            "active_binary_mutated": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "denied_by_operator_facing_summary_briefing": denials,
            "denied_by_operator_facing_summary_briefing_count": denied_count,
            "current_live_enabled_lane_count": 25,
            "enablement_lane_count": 28,
            "ready_enablement_lane_count": 28,
            "allowed_next_actions": [
                {
                    "action": "review_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial",
                    "status": "allowed_report_only",
                    "persists_summary": false,
                    "persists_briefing": false,
                    "delivers_summary": false,
                    "mutates_runtime": false,
                    "invokes_model": false,
                    "writes_memory_or_kg": false
                },
                {
                    "action": "stage_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial",
                    "status": "allowed_report_only_next_slice",
                    "accepts_operator_acknowledgement": false,
                    "persists_acknowledgement": false,
                    "activates_runtime": false,
                    "invokes_model": false,
                    "writes_kg": false
                }
            ],
        }),
    );
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
            "activation_command_result_receipt_operator_summary_channel_delivery_performed",
            "activation_command_result_receipt_operator_briefing_recorded",
            "activation_command_result_receipt_operator_briefing_persisted",
            "activation_command_result_receipt_operator_briefing_materialized",
            "activation_command_result_receipt_operator_briefing_filesystem_written",
            "activation_command_result_receipt_operator_briefing_delivered",
            "activation_command_result_receipt_operator_briefing_channel_delivery_performed",
            "activation_command_result_receipt_operator_summary_briefing_channel_delivery_performed",
            "telegram_send_performed",
            "channel_send_performed",
            "external_send_performed",
            "activation_command_result_receipt_recorded",
            "activation_command_result_receipt_persisted",
            "activation_command_result_receipt_accepted",
            "activation_command_result_receipt_materialized",
            "activation_command_result_receipt_filesystem_written",
            "activation_command_completion_ack_recorded",
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
            side_effects.insert(key.to_string(), serde_json::json!(false));
        }
    }
    report
}

fn hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_summary =
        hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_report();
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
    let source_status = source_summary
        .get("operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_status")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("unknown")
        .to_string();
    let source_ready = source_status == "blocked"
        && source_bool(
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready",
        );
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT;
    let report_ready = source_ready
        && route_count_source_command_accepted
        && route_matrix.missing_route_count == 0;
    let source_report_sha256 = sha256_json_value(&source_summary);

    let final_ack_fixture =
        |id: &str, status: &str, reason: &str, extra: serde_json::Value| -> serde_json::Value {
            let mut fixture = serde_json::Map::new();
            fixture.insert("fixture_id".to_string(), serde_json::json!(id));
            fixture.insert(
                "final_operator_acknowledgement_requested".to_string(),
                serde_json::json!(false),
            );
            fixture.insert(
                "final_operator_acknowledgement_status".to_string(),
                serde_json::json!(status),
            );
            fixture.insert(
                "source_summary_briefing_present".to_string(),
                serde_json::json!(true),
            );
            fixture.insert(
                "source_summary_briefing_ready".to_string(),
                serde_json::json!(true),
            );
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
                "memory_store_write_performed",
                "memory_store_mutated",
                "live_kg_write_performed",
                "rollback_executed",
                "public_release_claimed",
                "public_ga_claimed",
                "release_artifact_written",
                "install_executed",
                "launchd_mutated",
                "service_restart_performed",
                "active_binary_mutated",
            ] {
                fixture.insert(key.to_string(), serde_json::json!(false));
            }
            fixture.insert(
                "final_acknowledgement_noop_confirmed".to_string(),
                serde_json::json!(true),
            );
            fixture.insert("reason".to_string(), serde_json::json!(reason));
            if let Some(extra_object) = extra.as_object() {
                fixture.extend(extra_object.clone());
            }
            serde_json::Value::Object(fixture)
        };

    let final_acknowledgement_fixtures = serde_json::json!([
        final_ack_fixture(
            "operator-canary-controlled-request-harness-final-ack-missing-source-summary-briefing",
            "blocked_noop",
            "source_summary_briefing_report_required",
            serde_json::json!({
                "source_summary_briefing_present": false,
                "source_summary_briefing_ready": false,
                "final_operator_acknowledgement_requested": true,
            }),
        ),
        final_ack_fixture(
            "operator-canary-controlled-request-harness-final-ack-request",
            "blocked_ack_noop",
            "final_operator_acknowledgement_request_shape_denied",
            serde_json::json!({"final_operator_acknowledgement_requested": true}),
        ),
        final_ack_fixture(
            "operator-canary-controlled-request-harness-final-ack-acceptance-request",
            "blocked_acceptance_noop",
            "final_operator_acknowledgement_acceptance_denied",
            serde_json::json!({
                "final_operator_acknowledgement_requested": true,
                "acknowledgement_acceptance_requested": true,
            }),
        ),
        final_ack_fixture(
            "operator-canary-controlled-request-harness-final-ack-recording-request",
            "blocked_ack_noop",
            "final_operator_acknowledgement_recording_denied",
            serde_json::json!({
                "final_operator_acknowledgement_requested": true,
                "acknowledgement_recording_requested": true,
            }),
        ),
        final_ack_fixture(
            "operator-canary-controlled-request-harness-final-ack-persistence-filesystem-write-request",
            "blocked_ack_noop",
            "final_operator_acknowledgement_persistence_filesystem_write_denied",
            serde_json::json!({
                "final_operator_acknowledgement_requested": true,
                "acknowledgement_persistence_requested": true,
                "acknowledgement_filesystem_write_requested": true,
            }),
        ),
        final_ack_fixture(
            "operator-canary-controlled-request-harness-final-ack-identity-signature-timestamp-request",
            "blocked_acceptance_noop",
            "operator_identity_signature_timestamp_acknowledgement_acceptance_denied",
            serde_json::json!({
                "final_operator_acknowledgement_requested": true,
                "operator_identity_acceptance_requested": true,
                "operator_signature_acceptance_requested": true,
                "operator_timestamp_acceptance_requested": true,
            }),
        ),
        final_ack_fixture(
            "operator-canary-controlled-request-harness-final-ack-delivery-request",
            "blocked_delivery_noop",
            "final_operator_acknowledgement_delivery_denied",
            serde_json::json!({
                "final_operator_acknowledgement_requested": true,
                "acknowledgement_delivery_requested": true,
                "telegram_send_requested": true,
                "channel_delivery_requested": true,
            }),
        ),
        final_ack_fixture(
            "operator-canary-controlled-request-harness-final-state-promotion-request",
            "blocked_promotion_noop",
            "final_state_completion_promotion_denied",
            serde_json::json!({
                "final_operator_acknowledgement_requested": true,
                "final_state_promotion_requested": true,
                "completion_promotion_requested": true,
            }),
        ),
        final_ack_fixture(
            "operator-canary-controlled-request-harness-final-ack-activation-memory-kg-provider-request",
            "blocked_ack_noop",
            "activation_memory_kg_rollback_secret_provider_acknowledgement_denied",
            serde_json::json!({
                "final_operator_acknowledgement_requested": true,
                "activation_from_acknowledgement_requested": true,
                "memory_store_acknowledgement_requested": true,
                "live_kg_acknowledgement_requested": true,
                "rollback_acknowledgement_requested": true,
                "secret_material_acknowledgement_requested": true,
                "provider_prompt_acknowledgement_requested": true,
            }),
        ),
        final_ack_fixture(
            "operator-canary-controlled-request-harness-final-ack-external-public-install-request",
            "blocked_delivery_noop",
            "external_public_install_restart_active_binary_acknowledgement_denied",
            serde_json::json!({
                "final_operator_acknowledgement_requested": true,
                "external_send_acknowledgement_requested": true,
                "public_claim_acknowledgement_requested": true,
                "release_artifact_acknowledgement_requested": true,
                "install_acknowledgement_requested": true,
                "service_restart_acknowledgement_requested": true,
                "active_binary_acknowledgement_requested": true,
            }),
        ),
    ]);
    let final_acknowledgement_fixture_count = final_acknowledgement_fixtures
        .as_array()
        .map(std::vec::Vec::len)
        .unwrap_or(0);
    let fixtures_sha256 = sha256_json_value(&final_acknowledgement_fixtures);
    let contract_hash_sha256 = sha256_text_value(&format!(
        "hepta-canary-operator-review-acknowledgement-activation-command-result-receipt-final-operator-acknowledgement-non-acceptance-denial:v1:source={source_report_sha256}:fixtures={fixtures_sha256}:ack=0:accept=0:persist=0:deliver=0:promote=0:live=0"
    ));
    let policy_hash_sha256 = sha256_text_value(
        "memory-intelligence-kg-operator-canary-harness-operator-review-acknowledgement-activation-command-result-receipt-final-operator-acknowledgement-non-acceptance-denial:v1:no-ack-accept:no-ack-record:no-ack-persist:no-ack-deliver:no-final-state-promotion:no-live",
    );
    let side_effect_hash_sha256 = sha256_text_value(
        "final_operator_acknowledgement=false;acceptance=false;record=false;persist=false;deliver=false;promotion=false;activation=false;provider=false;model=false;memory=false;kg=false;secret=false;install=false;restart=false;active_binary=false",
    );
    let mut denials = source_summary
        .get("denied_by_operator_facing_summary_briefing")
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
            "gate": "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_route",
            "status": if report_ready { "ready" } else { "blocked" },
            "source_command": "/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-final-operator-acknowledgement-non-acceptance-denial --json",
            "native_route": true,
            "compatibility_mode": "native_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_status",
            "side_effect_free": true,
            "audit_date": "2026-06-14",
            "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_FINAL_OPERATOR_ACKNOWLEDGEMENT_NON_ACCEPTANCE_DENIAL_ENDPOINT,
            "source_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_route_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_OPERATOR_FACING_SUMMARY_BRIEFING_NON_PERSISTENCE_DENIAL_ENDPOINT,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_route_doc": "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_FINAL_OPERATOR_ACKNOWLEDGEMENT_NON_ACCEPTANCE_DENIAL_ROUTE_GATE.md",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-operator-facing-summary-briefing-non-persistence-denial-route-gate.sh",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-final-operator-acknowledgement-non-acceptance-denial-gate.sh",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-final-operator-acknowledgement-non-acceptance-denial-route-gate.sh",
            "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
            "route_count": route_matrix.route_count,
            "implemented_route_count": route_matrix.implemented_route_count,
            "missing_route_count": route_matrix.missing_route_count,
            "route_count_source_command_accepted": route_count_source_command_accepted,
            "source_route_wired": true,
            "source_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_route_ready": source_ready,
            "source_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_route_status": source_status,
            "source_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_route_report_sha256": source_report_sha256,
            "operator_authorization_scope": "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_no_ack_accept_no_record_no_persist_no_delivery_no_final_state_promotion_no_context_memory_kg_provider_model_credential_channel_install_restart_binary_or_public_release",
            "operator_authorization_received": true,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_route_enabled": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_status": "blocked",
            "activation_command_result_receipt_final_operator_acknowledgement_schema_version": "memory_intelligence_kg_operator_canary_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_v1",
            "activation_command_result_receipt_final_operator_acknowledgement_mode": "native_route_stdout_only_final_operator_acknowledgement_non_acceptance_denial_no_ack_no_accept_no_record_no_deliver_no_authority_no_live",
            "activation_command_result_receipt_final_operator_acknowledgement_decision": "blocked_noop_activation_command_result_receipt_cannot_be_acknowledged_or_promoted_into_final_operator_authority",
            "source_operator_facing_summary_briefing_fixture_count": source_u64("operator_facing_summary_briefing_fixture_count"),
            "source_blocked_operator_facing_summary_briefing_fixture_count": source_u64("blocked_operator_facing_summary_briefing_fixture_count"),
            "source_accepted_operator_facing_summary_briefing_fixture_count": source_u64("accepted_operator_facing_summary_briefing_fixture_count"),
            "source_operator_summary_performed_count": source_u64("operator_summary_performed_count"),
            "source_operator_briefing_performed_count": source_u64("operator_briefing_performed_count"),
            "final_acknowledgement_fixtures_sha256": fixtures_sha256,
            "final_acknowledgement_contract_hash_sha256": contract_hash_sha256,
            "final_acknowledgement_policy_hash_sha256": policy_hash_sha256,
            "side_effect_hash_sha256": side_effect_hash_sha256,
            "required_activation_command_result_receipt_final_operator_acknowledgement_surface_count": 12,
            "ready_activation_command_result_receipt_final_operator_acknowledgement_surface_count": 12,
            "side_effect_free_activation_command_result_receipt_final_operator_acknowledgement_surface_count": 12,
            "required_activation_command_result_receipt_final_operator_acknowledgement_fixture_count": 10,
            "activation_command_result_receipt_final_operator_acknowledgement_fixtures": final_acknowledgement_fixtures,
            "activation_command_result_receipt_final_operator_acknowledgement_fixture_count": final_acknowledgement_fixture_count,
            "blocked_activation_command_result_receipt_final_operator_acknowledgement_fixture_count": final_acknowledgement_fixture_count,
            "noop_activation_command_result_receipt_final_operator_acknowledgement_fixture_count": final_acknowledgement_fixture_count,
            "allowed_activation_command_result_receipt_final_operator_acknowledgement_fixture_count": 0,
            "accepted_activation_command_result_receipt_final_operator_acknowledgement_fixture_count": 0,
            "activation_command_result_receipt_final_operator_acknowledgement_denied_count": final_acknowledgement_fixture_count,
            "activation_command_result_receipt_final_operator_acknowledgement_performed_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "activation_command_result_receipt_final_operator_acknowledgement_allowed": false,
            "activation_command_result_receipt_final_operator_acknowledgement_request_accepted": false,
            "activation_command_result_receipt_final_operator_acknowledgement_accepted": false,
            "activation_command_result_receipt_final_operator_acknowledgement_recorded": false,
            "activation_command_result_receipt_final_operator_acknowledgement_persisted": false,
            "activation_command_result_receipt_final_operator_acknowledgement_materialized": false,
            "activation_command_result_receipt_final_operator_acknowledgement_filesystem_written": false,
            "activation_command_result_receipt_final_operator_acknowledgement_delivered": false,
            "activation_command_result_receipt_final_operator_acknowledgement_channel_delivery_performed": false,
            "activation_command_result_receipt_final_operator_acknowledgement_identity_accepted": false,
            "activation_command_result_receipt_final_operator_acknowledgement_signature_accepted": false,
            "activation_command_result_receipt_final_operator_acknowledgement_timestamp_accepted": false,
            "activation_command_result_receipt_final_operator_acknowledgement_final_state_promoted": false,
            "activation_command_result_receipt_final_operator_acknowledgement_completion_promoted": false,
            "activation_command_result_receipt_operator_final_acceptance_recorded": false,
            "activation_command_result_receipt_operator_final_acceptance_persisted": false,
            "activation_command_result_receipt_operator_final_acceptance_materialized": false,
            "telegram_send_performed": false,
            "channel_send_performed": false,
            "external_send_performed": false,
            "activation_allowed_by_result_receipt_final_operator_acknowledgement": false,
            "activation_allowed_by_result_receipt_summary_briefing": false,
            "activation_allowed_by_result_receipt": false,
            "activation_allowed": false,
            "activation_performed": false,
            "live_mutation_execution_ready": false,
            "live_mutation_execution_allowed": false,
            "live_mutation_execution_performed": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "denied_by_activation_command_result_receipt_final_operator_acknowledgement": denials,
            "denied_by_activation_command_result_receipt_final_operator_acknowledgement_count": denied_count,
            "current_live_enabled_lane_count": 26,
            "enablement_lane_count": 29,
            "ready_enablement_lane_count": 29,
            "allowed_next_actions": [
                {
                    "action": "review_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial",
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
                    "action": "stage_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial",
                    "status": "allowed_report_only_next_slice",
                    "accepts_terminal_decision": false,
                    "claims_public_release": false,
                    "writes_release_artifact": false,
                    "activates_runtime": false,
                    "invokes_model": false,
                    "writes_memory_or_kg": false
                }
            ],
        }),
    );
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
            "telegram_send_performed",
            "channel_send_performed",
            "external_send_performed",
            "activation_command_result_receipt_recorded",
            "activation_command_result_receipt_persisted",
            "activation_command_result_receipt_accepted",
            "activation_command_result_receipt_materialized",
            "activation_command_result_receipt_filesystem_written",
            "activation_command_completion_ack_recorded",
            "activation_command_enabled",
            "activation_command_invoked",
            "activation_command_dispatched",
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
            "memory_store_write_performed",
            "memory_store_mutated",
            "live_kg_write_performed",
            "rollback_executed",
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
            side_effects.insert(key.to_string(), serde_json::json!(false));
        }
    }
    report
}
