#[test]
fn hepta_memory_write_execution_scoped_memory_real_write_canary_rollback_tombstone_dry_run_boundary_blocks_rollback_tombstone_and_writes()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_MEMORY_REAL_WRITE_CANARY_ROLLBACK_TOMBSTONE_DRY_RUN_BOUNDARY_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body)
        .expect("scoped Memory real-write canary rollback/tombstone dry-run boundary json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_MEMORY_REAL_WRITE_CANARY_ROLLBACK_TOMBSTONE_DRY_RUN_BOUNDARY_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-live-mutation-operator-write-execution-scoped-memory-real-write-canary-rollback-tombstone-dry-run-boundary --json"
    );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(value["route_count"], NATIVE_GATEWAY_SOURCE_COMMAND_COUNT);
    assert_eq!(
        value["implemented_route_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(
        value["memory_write_execution_scoped_memory_real_write_canary_rollback_tombstone_dry_run_boundary_ready"],
        true
    );
    assert_eq!(
        value["scoped_memory_real_write_canary_rollback_tombstone_dry_run_ready"],
        true
    );
    assert_eq!(
        value["source_scoped_memory_real_write_canary_readback_validation_dry_run_boundary_ready"],
        true
    );
    assert_eq!(
        value["source_scoped_memory_real_write_canary_readback_validation_dry_run_ready"],
        true
    );
    assert_ne!(
        value["source_scoped_memory_real_write_canary_readback_validation_report_sha256"],
        ""
    );
    assert_eq!(
        value["source_scoped_memory_real_write_canary_readback_fixture_count"],
        10
    );
    assert_eq!(
        value["source_accepted_scoped_memory_real_write_canary_readback_fixture_count"],
        0
    );
    assert_eq!(
        value["source_scoped_memory_real_write_canary_readback_denial_count"],
        26
    );
    assert_eq!(value["source_readback_performed_count"], 0);
    assert_eq!(value["source_readback_result_accepted_count"], 0);
    assert_eq!(value["source_durable_memory_store_read_performed_count"], 0);
    assert_eq!(value["source_memory_store_write_performed_count"], 0);
    assert_eq!(value["source_rollback_tombstone_handoff_accepted_count"], 0);
    assert_eq!(
        value["scoped_memory_real_write_canary_mode"],
        "scoped_memory_real_write_canary_rollback_tombstone_dry_run_no_rollback_no_write"
    );
    assert_eq!(
        value["required_scoped_memory_real_write_canary_rollback_tombstone_surface_count"],
        12
    );
    assert_eq!(
        value["ready_scoped_memory_real_write_canary_rollback_tombstone_surface_count"],
        12
    );
    assert_eq!(
        value["side_effect_free_scoped_memory_real_write_canary_rollback_tombstone_surface_count"],
        12
    );
    assert_eq!(
        value["required_scoped_memory_real_write_canary_rollback_tombstone_fixture_count"],
        10
    );
    assert_eq!(
        value["scoped_memory_real_write_canary_rollback_tombstone_fixture_count"],
        10
    );
    assert_eq!(
        value["blocked_scoped_memory_real_write_canary_rollback_tombstone_fixture_count"],
        10
    );
    assert_eq!(
        value["accepted_scoped_memory_real_write_canary_rollback_tombstone_fixture_count"],
        0
    );
    for key in [
        "rollback_plan_accepted_count",
        "rollback_tombstone_plan_accepted_count",
        "tombstone_plan_accepted_count",
        "rollback_target_bound_count",
        "rollback_receipt_linked_count",
        "rollback_ordering_guard_accepted_count",
        "rollback_idempotency_guard_accepted_count",
        "rollback_audit_evidence_recorded_count",
        "rollback_audit_evidence_persisted_count",
        "operator_review_handoff_accepted_count",
        "minimal_real_write_handoff_accepted_count",
        "rollback_performed_count",
        "rollback_result_recorded_count",
        "rollback_result_persisted_count",
        "rollback_result_accepted_count",
        "tombstone_written_count",
        "compensating_memory_write_performed_count",
        "durable_memory_store_read_performed_count",
        "durable_memory_store_rollback_performed_count",
        "durable_memory_store_write_performed_count",
        "memory_store_write_performed_count",
    ] {
        assert_eq!(
            value[key], 0,
            "scoped Memory canary rollback/tombstone dry-run count should stay zero: {key}"
        );
    }
    assert_eq!(
        value["required_before_scoped_memory_real_write_canary_rollback_tombstone_acceptance_count"],
        15
    );
    assert_eq!(
        value["required_scoped_memory_real_write_canary_rollback_tombstone_fields"]
            .as_array()
            .expect("required scoped Memory canary rollback/tombstone fields")
            .len(),
        15
    );
    let fixtures = value["scoped_memory_real_write_canary_rollback_tombstone_fixtures"]
        .as_array()
        .expect("scoped Memory canary rollback/tombstone fixtures");
    assert_eq!(fixtures.len(), 10);
    assert!(fixtures.iter().all(|fixture| {
        fixture["scoped_memory_real_write_canary_rollback_tombstone_status"]
            .as_str()
            .is_some_and(|status| status.starts_with("blocked"))
            && fixture["readback_result_accepted"].as_bool() == Some(false)
            && fixture["rollback_tombstone_handoff_accepted"].as_bool() == Some(false)
            && fixture["rollback_plan_accepted"].as_bool() == Some(false)
            && fixture["rollback_tombstone_plan_accepted"].as_bool() == Some(false)
            && fixture["rollback_target_bound"].as_bool() == Some(false)
            && fixture["rollback_receipt_linked"].as_bool() == Some(false)
            && fixture["rollback_idempotency_guard_accepted"].as_bool() == Some(false)
            && fixture["rollback_ordering_guard_accepted"].as_bool() == Some(false)
            && fixture["rollback_audit_evidence_recorded"].as_bool() == Some(false)
            && fixture["operator_review_handoff_accepted"].as_bool() == Some(false)
            && fixture["minimal_real_write_handoff_accepted"].as_bool() == Some(false)
            && fixture["rollback_executed"].as_bool() == Some(false)
            && fixture["tombstone_written"].as_bool() == Some(false)
            && fixture["durable_memory_store_read_performed"].as_bool() == Some(false)
            && fixture["durable_memory_store_write_performed"].as_bool() == Some(false)
            && fixture["durable_memory_store_rollback_performed"].as_bool() == Some(false)
            && fixture["memory_store_mutated"].as_bool() == Some(false)
            && fixture["live_kg_write_performed"].as_bool() == Some(false)
            && fixture["provider_invoked"].as_bool() == Some(false)
            && fixture["model_invoked"].as_bool() == Some(false)
            && fixture["credential_read"].as_bool() == Some(false)
            && fixture["channel_send_performed"].as_bool() == Some(false)
            && fixture["external_send_performed"].as_bool() == Some(false)
            && fixture["release_artifact_written"].as_bool() == Some(false)
            && fixture["install_executed"].as_bool() == Some(false)
            && fixture["active_binary_mutated"].as_bool() == Some(false)
            && fixture["scoped_canary_rollback_tombstone_dry_run_noop_confirmed"].as_bool()
                == Some(true)
    }));
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["rollback_plan_requested"] == true)
            .count(),
        2
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["tombstone_plan_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["rollback_execution_requested"] == true
                    && fixture["tombstone_write_requested"] == true
                    && fixture["durable_memory_rollback_requested"] == true
            })
            .count(),
        1
    );
    let denied = value["denied_by_scoped_memory_real_write_canary_rollback_tombstone_dry_run"]
        .as_array()
        .expect("scoped Memory canary rollback/tombstone denials");
    assert_eq!(denied.len(), 28);
    assert_eq!(
        value["denied_by_scoped_memory_real_write_canary_rollback_tombstone_dry_run_count"],
        28
    );
    for key in [
        "fresh_operator_approval_packet_accepted",
        "single_use_nonce_consumed",
        "explicit_command_dispatched",
        "receipt_persisted",
        "post_write_readback_performed",
        "readback_result_accepted",
        "rollback_tombstone_handoff_accepted",
        "rollback_plan_accepted",
        "rollback_tombstone_plan_accepted",
        "rollback_target_bound",
        "rollback_receipt_linked",
        "rollback_ordering_guard_accepted",
        "rollback_idempotency_guard_accepted",
        "rollback_audit_evidence_recorded",
        "rollback_audit_evidence_persisted",
        "operator_review_handoff_accepted",
        "minimal_real_write_handoff_accepted",
        "rollback_executed",
        "rollback_result_recorded",
        "rollback_result_persisted",
        "rollback_result_accepted",
        "tombstone_written",
        "compensating_memory_write_performed",
        "activation_allowed",
        "live_mutation_execution_performed",
        "memory_write_execution_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "durable_memory_store_write_performed",
        "durable_memory_store_read_performed",
        "durable_memory_store_rollback_performed",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "channel_send_performed",
        "external_send_performed",
        "release_artifact_written",
        "install_executed",
        "service_restarted",
        "active_binary_mutated",
    ] {
        assert_eq!(
            value[key], false,
            "scoped Memory canary rollback/tombstone dry-run field should stay false: {key}"
        );
    }
    assert_eq!(value["rollback_plan_required"], true);
    assert_eq!(value["tombstone_plan_required"], true);
    assert_eq!(value["rollback_target_binding_required"], true);
    assert_eq!(value["rollback_receipt_linkage_required"], true);
    assert_eq!(value["rollback_idempotency_guard_required"], true);
    assert_eq!(value["rollback_ordering_guard_required"], true);
    assert_eq!(value["rollback_audit_evidence_required"], true);
    assert_eq!(value["operator_review_handoff_required"], true);
    assert_eq!(value["minimal_real_write_handoff_required"], true);
    assert_eq!(value["rollback_execution_forbidden"], true);
    assert_eq!(value["tombstone_write_forbidden"], true);
    assert_eq!(value["durable_memory_read_forbidden"], true);
    assert_eq!(value["durable_memory_write_forbidden"], true);
    assert_eq!(value["durable_memory_rollback_forbidden"], true);
    assert_eq!(value["memory_store_mutation_forbidden"], true);
    assert_eq!(value["kg_live_write_forbidden"], true);
    assert_eq!(value["provider_model_invocation_forbidden"], true);
    assert_eq!(value["credential_read_forbidden"], true);
    assert_eq!(value["channel_external_send_forbidden"], true);
    assert_eq!(value["public_claim_release_artifact_forbidden"], true);
    assert_eq!(
        value["install_restart_active_binary_mutation_forbidden"],
        true
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "run_scoped_memory_real_write_canary_rollback_tombstone_dry_run_boundary_require_live_gate"
    );
    assert_eq!(value["allowed_next_actions"][0]["reads_memory"], false);
    assert_eq!(value["allowed_next_actions"][0]["writes_memory"], false);
    assert_eq!(value["allowed_next_actions"][0]["executes_rollback"], false);
    assert_eq!(
        value["allowed_next_actions"][1]["action"],
        "prepare_minimal_scoped_memory_real_write_canary_accepted_gate"
    );
    let side_effects = value["side_effects"]
        .as_object()
        .expect("scoped Memory canary rollback/tombstone dry-run side effects");
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_write_execution_minimal_scoped_memory_real_write_canary_operator_approval_nonce_command_accepted_gate_accepts_authority_without_write()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_MINIMAL_SCOPED_MEMORY_REAL_WRITE_CANARY_OPERATOR_APPROVAL_NONCE_COMMAND_ACCEPTED_GATE_BOUNDARY_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body)
        .expect("minimal scoped Memory real-write canary accepted-gate boundary json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_MINIMAL_SCOPED_MEMORY_REAL_WRITE_CANARY_OPERATOR_APPROVAL_NONCE_COMMAND_ACCEPTED_GATE_BOUNDARY_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-operator-approval-nonce-command-accepted-gate-boundary --json"
    );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(value["route_count"], NATIVE_GATEWAY_SOURCE_COMMAND_COUNT);
    assert_eq!(
        value["implemented_route_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(
        value["memory_write_execution_minimal_scoped_memory_real_write_canary_operator_approval_nonce_command_accepted_gate_boundary_ready"],
        true
    );
    assert_eq!(
        value["minimal_scoped_memory_real_write_canary_accepted_gate_ready"],
        true
    );
    assert_eq!(
        value["minimal_scoped_memory_real_write_canary_authority_accepted_no_write"],
        true
    );
    assert_eq!(
        value["source_scoped_memory_real_write_canary_rollback_tombstone_dry_run_boundary_ready"],
        true
    );
    assert_eq!(
        value["source_scoped_memory_real_write_canary_rollback_tombstone_fixture_count"],
        10
    );
    assert_eq!(
        value["source_accepted_scoped_memory_real_write_canary_rollback_tombstone_fixture_count"],
        0
    );
    assert_eq!(
        value["source_scoped_memory_real_write_canary_rollback_tombstone_denial_count"],
        28
    );
    assert_eq!(value["source_rollback_performed_count"], 0);
    assert_eq!(value["source_tombstone_written_count"], 0);
    assert_eq!(value["source_durable_memory_store_read_performed_count"], 0);
    assert_eq!(
        value["source_durable_memory_store_write_performed_count"],
        0
    );
    assert_eq!(
        value["source_durable_memory_store_rollback_performed_count"],
        0
    );
    assert_eq!(value["source_memory_store_write_performed_count"], 0);
    assert_eq!(
        value["scoped_memory_real_write_canary_mode"],
        "minimal_scoped_memory_real_write_canary_accepted_gate_authority_no_write"
    );
    assert_eq!(
        value["required_minimal_scoped_memory_real_write_canary_accepted_gate_surface_count"],
        12
    );
    assert_eq!(
        value["ready_minimal_scoped_memory_real_write_canary_accepted_gate_surface_count"],
        12
    );
    assert_eq!(
        value["side_effect_free_minimal_scoped_memory_real_write_canary_accepted_gate_surface_count"],
        12
    );
    assert_eq!(
        value["required_minimal_scoped_memory_real_write_canary_accepted_gate_fixture_count"],
        10
    );
    assert_eq!(
        value["minimal_scoped_memory_real_write_canary_accepted_gate_fixture_count"],
        10
    );
    assert_eq!(
        value["accepted_minimal_scoped_memory_real_write_canary_accepted_gate_fixture_count"],
        1
    );
    assert_eq!(
        value["blocked_minimal_scoped_memory_real_write_canary_accepted_gate_fixture_count"],
        9
    );
    for key in [
        "fresh_operator_approval_artifact_accepted_count",
        "operator_identity_session_bound_count",
        "single_use_nonce_authority_accepted_count",
        "explicit_command_accepted_count",
        "canary_namespace_store_scope_bound_count",
        "payload_digest_redaction_bound_count",
        "active_binary_sha_route_count_bound_count",
        "wal_receipt_binding_accepted_count",
        "post_write_readback_binding_accepted_count",
        "rollback_tombstone_proof_binding_accepted_count",
        "minimal_scoped_memory_real_write_canary_authority_accepted_count",
    ] {
        assert_eq!(
            value[key], 1,
            "minimal scoped Memory real-write canary accepted-gate count should be one: {key}"
        );
    }
    for key in [
        "single_use_nonce_consumed_count",
        "explicit_command_dispatched_count",
        "wal_write_performed_count",
        "receipt_recorded_count",
        "receipt_persisted_count",
        "receipt_materialized_count",
        "receipt_delivered_count",
        "post_write_readback_performed_count",
        "readback_result_recorded_count",
        "readback_result_persisted_count",
        "readback_result_accepted_count",
        "rollback_performed_count",
        "rollback_result_recorded_count",
        "rollback_result_persisted_count",
        "rollback_result_accepted_count",
        "tombstone_written_count",
        "compensating_memory_write_performed_count",
        "durable_memory_store_read_performed_count",
        "durable_memory_store_rollback_performed_count",
        "durable_memory_store_write_performed_count",
        "memory_store_write_performed_count",
    ] {
        assert_eq!(
            value[key], 0,
            "minimal scoped Memory real-write canary accepted-gate side-effect count should stay zero: {key}"
        );
    }
    assert_eq!(
        value["required_before_minimal_scoped_memory_real_write_canary_execution_count"],
        16
    );
    assert_eq!(
        value["required_minimal_scoped_memory_real_write_canary_accepted_gate_fields"]
            .as_array()
            .expect("required minimal scoped Memory accepted-gate fields")
            .len(),
        16
    );
    let fixtures = value["minimal_scoped_memory_real_write_canary_accepted_gate_fixtures"]
        .as_array()
        .expect("minimal scoped Memory accepted-gate fixtures");
    assert_eq!(fixtures.len(), 10);
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["minimal_real_write_authority_accepted"] == true)
            .count(),
        1
    );
    assert!(fixtures.iter().all(|fixture| {
        fixture["single_use_nonce_consumed"].as_bool() == Some(false)
            && fixture["explicit_command_dispatched"].as_bool() == Some(false)
            && fixture["wal_write_performed"].as_bool() == Some(false)
            && fixture["receipt_persisted"].as_bool() == Some(false)
            && fixture["post_write_readback_performed"].as_bool() == Some(false)
            && fixture["rollback_executed"].as_bool() == Some(false)
            && fixture["tombstone_written"].as_bool() == Some(false)
            && fixture["memory_write_execution_performed"].as_bool() == Some(false)
            && fixture["memory_store_write_performed"].as_bool() == Some(false)
            && fixture["memory_store_mutated"].as_bool() == Some(false)
            && fixture["durable_memory_store_read_performed"].as_bool() == Some(false)
            && fixture["durable_memory_store_write_performed"].as_bool() == Some(false)
            && fixture["durable_memory_store_rollback_performed"].as_bool() == Some(false)
            && fixture["live_kg_write_performed"].as_bool() == Some(false)
            && fixture["provider_invoked"].as_bool() == Some(false)
            && fixture["model_invoked"].as_bool() == Some(false)
            && fixture["credential_read"].as_bool() == Some(false)
            && fixture["channel_send_performed"].as_bool() == Some(false)
            && fixture["external_send_performed"].as_bool() == Some(false)
            && fixture["release_artifact_written"].as_bool() == Some(false)
            && fixture["install_executed"].as_bool() == Some(false)
            && fixture["active_binary_mutated"].as_bool() == Some(false)
            && fixture["accepted_authority_envelope_noop_confirmed"].as_bool() == Some(true)
    }));
    let accepted = fixtures
        .iter()
        .find(|fixture| fixture["minimal_real_write_authority_accepted"] == true)
        .expect("accepted authority fixture");
    assert_eq!(
        accepted["minimal_scoped_memory_real_write_canary_accepted_gate_status"],
        "accepted_authority_noop"
    );
    assert_eq!(accepted["fresh_operator_approval_artifact_accepted"], true);
    assert_eq!(accepted["operator_identity_bound"], true);
    assert_eq!(accepted["operator_session_bound"], true);
    assert_eq!(accepted["single_use_nonce_authority_accepted"], true);
    assert_eq!(accepted["explicit_command_accepted"], true);
    assert_eq!(accepted["canary_namespace_bound"], true);
    assert_eq!(accepted["canary_store_bound"], true);
    assert_eq!(accepted["canary_scope_bound"], true);
    assert_eq!(accepted["wal_receipt_binding_accepted"], true);
    assert_eq!(accepted["post_write_readback_binding_accepted"], true);
    assert_eq!(accepted["rollback_tombstone_proof_binding_accepted"], true);
    let denied = value["denied_by_minimal_scoped_memory_real_write_canary_accepted_gate_boundary"]
        .as_array()
        .expect("minimal scoped Memory accepted-gate denials");
    assert_eq!(denied.len(), 26);
    assert_eq!(
        value["denied_by_minimal_scoped_memory_real_write_canary_accepted_gate_boundary_count"],
        26
    );
    for key in [
        "source_rollback_tombstone_dry_run_required",
        "fresh_operator_approval_artifact_accepted",
        "operator_identity_bound",
        "operator_session_bound",
        "single_use_nonce_authority_accepted",
        "explicit_command_accepted",
        "canary_namespace_bound",
        "canary_store_bound",
        "canary_scope_bound",
        "payload_digest_bound",
        "payload_redaction_bound",
        "active_binary_sha_bound",
        "route_count_bound",
        "wal_receipt_binding_accepted",
        "post_write_readback_binding_accepted",
        "rollback_tombstone_proof_binding_accepted",
        "minimal_real_write_authority_accepted",
        "nonce_consumption_forbidden_on_report_route",
        "explicit_command_dispatch_forbidden_on_report_route",
        "durable_memory_read_forbidden",
        "durable_memory_write_forbidden",
        "durable_memory_rollback_forbidden",
        "memory_store_mutation_forbidden",
        "wal_write_forbidden",
        "receipt_persistence_forbidden",
        "rollback_execution_forbidden",
        "tombstone_write_forbidden",
        "kg_live_write_forbidden",
        "provider_model_invocation_forbidden",
        "credential_read_forbidden",
        "channel_external_send_forbidden",
        "public_claim_release_artifact_forbidden",
        "install_restart_active_binary_mutation_forbidden",
    ] {
        assert_eq!(
            value[key], true,
            "minimal scoped Memory accepted-gate field should be true: {key}"
        );
    }
    for key in [
        "single_use_nonce_consumed",
        "explicit_command_dispatched",
        "wal_write_performed",
        "receipt_recorded",
        "receipt_persisted",
        "receipt_materialized",
        "post_write_readback_performed",
        "readback_result_accepted",
        "rollback_executed",
        "tombstone_written",
        "compensating_memory_write_performed",
        "activation_performed",
        "live_mutation_execution_performed",
        "memory_write_execution_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "durable_memory_store_write_performed",
        "durable_memory_store_read_performed",
        "durable_memory_store_rollback_performed",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "channel_send_performed",
        "external_send_performed",
        "release_artifact_written",
        "install_executed",
        "service_restarted",
        "active_binary_mutated",
    ] {
        assert_eq!(
            value[key], false,
            "minimal scoped Memory accepted-gate side-effect field should stay false: {key}"
        );
    }
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "run_minimal_scoped_memory_real_write_canary_accepted_gate_boundary_require_live_gate"
    );
    assert_eq!(value["allowed_next_actions"][0]["writes_memory"], false);
    assert_eq!(value["allowed_next_actions"][0]["consumes_nonce"], false);
    assert_eq!(
        value["allowed_next_actions"][1]["action"],
        "prepare_minimal_scoped_memory_real_write_canary_wal_receipt_binding_boundary"
    );
    assert_eq!(
        value["allowed_next_actions"][1]["requires_minimal_scoped_memory_real_write_canary_accepted_gate"],
        true
    );
    assert_eq!(value["allowed_next_actions"][1]["writes_memory"], false);
    let side_effects = value["side_effects"]
        .as_object()
        .expect("minimal scoped Memory accepted-gate side effects");
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_write_execution_minimal_scoped_memory_real_write_canary_wal_receipt_binding_accepts_bindings_without_writes()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_MINIMAL_SCOPED_MEMORY_REAL_WRITE_CANARY_WAL_RECEIPT_BINDING_BOUNDARY_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body)
        .expect("minimal scoped Memory real-write canary WAL/receipt binding boundary json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_MINIMAL_SCOPED_MEMORY_REAL_WRITE_CANARY_WAL_RECEIPT_BINDING_BOUNDARY_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-wal-receipt-binding-boundary --json"
    );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(value["route_count"], NATIVE_GATEWAY_SOURCE_COMMAND_COUNT);
    assert_eq!(
        value["implemented_route_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(
        value["memory_write_execution_minimal_scoped_memory_real_write_canary_wal_receipt_binding_boundary_ready"],
        true
    );
    assert_eq!(
        value["minimal_scoped_memory_real_write_canary_wal_receipt_binding_ready"],
        true
    );
    assert_eq!(
        value["minimal_scoped_memory_real_write_canary_wal_receipt_binding_accepted_no_write"],
        true
    );
    assert_eq!(
        value["source_minimal_scoped_memory_real_write_canary_accepted_gate_boundary_ready"],
        true
    );
    assert_eq!(
        value["source_accepted_minimal_scoped_memory_real_write_canary_accepted_gate_fixture_count"],
        1
    );
    assert_eq!(
        value["source_blocked_minimal_scoped_memory_real_write_canary_accepted_gate_fixture_count"],
        9
    );
    assert_eq!(
        value["source_minimal_scoped_memory_real_write_canary_authority_accepted_count"],
        1
    );
    assert_eq!(value["source_single_use_nonce_consumed_count"], 0);
    assert_eq!(value["source_explicit_command_dispatched_count"], 0);
    assert_eq!(value["source_wal_write_performed_count"], 0);
    assert_eq!(value["source_receipt_persisted_count"], 0);
    assert_eq!(
        value["scoped_memory_real_write_canary_mode"],
        "minimal_scoped_memory_real_write_canary_wal_receipt_binding_no_write"
    );
    assert_eq!(
        value["required_minimal_scoped_memory_real_write_canary_wal_receipt_binding_surface_count"],
        12
    );
    assert_eq!(
        value["ready_minimal_scoped_memory_real_write_canary_wal_receipt_binding_surface_count"],
        12
    );
    assert_eq!(
        value["side_effect_free_minimal_scoped_memory_real_write_canary_wal_receipt_binding_surface_count"],
        12
    );
    assert_eq!(
        value["required_minimal_scoped_memory_real_write_canary_wal_receipt_binding_fixture_count"],
        10
    );
    assert_eq!(
        value["minimal_scoped_memory_real_write_canary_wal_receipt_binding_fixture_count"],
        10
    );
    assert_eq!(
        value["accepted_minimal_scoped_memory_real_write_canary_wal_receipt_binding_fixture_count"],
        1
    );
    assert_eq!(
        value["blocked_minimal_scoped_memory_real_write_canary_wal_receipt_binding_fixture_count"],
        9
    );
    for key in [
        "wal_receipt_binding_authority_accepted_count",
        "wal_namespace_store_scope_bound_count",
        "wal_record_id_bound_count",
        "wal_sequence_guard_bound_count",
        "wal_idempotency_key_bound_count",
        "wal_payload_digest_redaction_bound_count",
        "receipt_id_bound_count",
        "receipt_hash_chain_bound_count",
        "receipt_replay_guard_bound_count",
        "receipt_audit_evidence_bound_count",
        "post_write_readback_handoff_bound_count",
        "minimal_scoped_memory_real_write_canary_wal_receipt_binding_accepted_count",
    ] {
        assert_eq!(
            value[key], 1,
            "minimal scoped Memory WAL/receipt binding count should be one: {key}"
        );
    }
    for key in [
        "single_use_nonce_consumed_count",
        "explicit_command_dispatched_count",
        "wal_write_performed_count",
        "wal_recorded_count",
        "wal_persisted_count",
        "receipt_recorded_count",
        "receipt_persisted_count",
        "receipt_materialized_count",
        "receipt_delivered_count",
        "post_write_readback_performed_count",
        "rollback_performed_count",
        "tombstone_written_count",
        "durable_memory_store_read_performed_count",
        "durable_memory_store_rollback_performed_count",
        "durable_memory_store_write_performed_count",
        "memory_store_write_performed_count",
    ] {
        assert_eq!(
            value[key], 0,
            "minimal scoped Memory WAL/receipt binding side-effect count should stay zero: {key}"
        );
    }
    assert_eq!(
        value["required_before_minimal_scoped_memory_real_write_canary_wal_receipt_binding_count"],
        17
    );
    assert_eq!(
        value["required_minimal_scoped_memory_real_write_canary_wal_receipt_binding_fields"]
            .as_array()
            .expect("required minimal scoped Memory WAL/receipt fields")
            .len(),
        17
    );
    let fixtures = value["minimal_scoped_memory_real_write_canary_wal_receipt_binding_fixtures"]
        .as_array()
        .expect("minimal scoped Memory WAL/receipt fixtures");
    assert_eq!(fixtures.len(), 10);
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["minimal_scoped_memory_real_write_canary_wal_receipt_binding_accepted"]
                    == true
            })
            .count(),
        1
    );
    assert!(fixtures.iter().all(|fixture| {
        fixture["single_use_nonce_consumed"].as_bool() == Some(false)
            && fixture["explicit_command_dispatched"].as_bool() == Some(false)
            && fixture["wal_write_performed"].as_bool() == Some(false)
            && fixture["wal_persisted"].as_bool() == Some(false)
            && fixture["receipt_recorded"].as_bool() == Some(false)
            && fixture["receipt_persisted"].as_bool() == Some(false)
            && fixture["receipt_materialized"].as_bool() == Some(false)
            && fixture["receipt_delivered"].as_bool() == Some(false)
            && fixture["post_write_readback_performed"].as_bool() == Some(false)
            && fixture["rollback_executed"].as_bool() == Some(false)
            && fixture["tombstone_written"].as_bool() == Some(false)
            && fixture["memory_write_execution_performed"].as_bool() == Some(false)
            && fixture["memory_store_write_performed"].as_bool() == Some(false)
            && fixture["memory_store_mutated"].as_bool() == Some(false)
            && fixture["durable_memory_store_read_performed"].as_bool() == Some(false)
            && fixture["durable_memory_store_write_performed"].as_bool() == Some(false)
            && fixture["durable_memory_store_rollback_performed"].as_bool() == Some(false)
            && fixture["live_kg_write_performed"].as_bool() == Some(false)
            && fixture["provider_invoked"].as_bool() == Some(false)
            && fixture["model_invoked"].as_bool() == Some(false)
            && fixture["credential_read"].as_bool() == Some(false)
            && fixture["channel_send_performed"].as_bool() == Some(false)
            && fixture["external_send_performed"].as_bool() == Some(false)
            && fixture["release_artifact_written"].as_bool() == Some(false)
            && fixture["install_executed"].as_bool() == Some(false)
            && fixture["active_binary_mutated"].as_bool() == Some(false)
            && fixture["wal_receipt_binding_noop_confirmed"].as_bool() == Some(true)
    }));
    let accepted = fixtures
        .iter()
        .find(|fixture| {
            fixture["minimal_scoped_memory_real_write_canary_wal_receipt_binding_accepted"] == true
        })
        .expect("accepted WAL/receipt binding fixture");
    assert_eq!(
        accepted["minimal_scoped_memory_real_write_canary_wal_receipt_binding_status"],
        "accepted_wal_receipt_binding_noop"
    );
    assert_eq!(accepted["wal_namespace_bound"], true);
    assert_eq!(accepted["wal_store_bound"], true);
    assert_eq!(accepted["wal_scope_bound"], true);
    assert_eq!(accepted["wal_record_id_bound"], true);
    assert_eq!(accepted["wal_sequence_guard_bound"], true);
    assert_eq!(accepted["wal_idempotency_key_bound"], true);
    assert_eq!(accepted["receipt_id_bound"], true);
    assert_eq!(accepted["receipt_hash_chain_bound"], true);
    assert_eq!(accepted["receipt_replay_guard_bound"], true);
    assert_eq!(accepted["receipt_audit_evidence_bound"], true);
    assert_eq!(accepted["post_write_readback_handoff_bound"], true);
    let denied =
        value["denied_by_minimal_scoped_memory_real_write_canary_wal_receipt_binding_boundary"]
            .as_array()
            .expect("minimal scoped Memory WAL/receipt denials");
    assert_eq!(denied.len(), 28);
    assert_eq!(
        value["denied_by_minimal_scoped_memory_real_write_canary_wal_receipt_binding_boundary_count"],
        28
    );
    for key in [
        "source_minimal_scoped_memory_real_write_canary_accepted_gate_required",
        "minimal_scoped_memory_real_write_canary_wal_receipt_binding_accepted",
        "wal_namespace_bound",
        "wal_store_bound",
        "wal_scope_bound",
        "wal_record_id_bound",
        "wal_sequence_guard_bound",
        "wal_idempotency_key_bound",
        "wal_payload_digest_bound",
        "wal_payload_redaction_bound",
        "receipt_id_bound",
        "receipt_hash_chain_bound",
        "receipt_replay_guard_bound",
        "receipt_audit_evidence_bound",
        "post_write_readback_handoff_bound",
        "nonce_consumption_forbidden_on_report_route",
        "explicit_command_dispatch_forbidden_on_report_route",
        "wal_write_forbidden",
        "wal_persistence_forbidden",
        "receipt_recording_forbidden",
        "receipt_persistence_forbidden",
        "receipt_materialization_forbidden",
        "receipt_delivery_forbidden",
        "durable_memory_read_forbidden",
        "durable_memory_write_forbidden",
        "durable_memory_rollback_forbidden",
        "memory_store_mutation_forbidden",
        "rollback_execution_forbidden",
        "tombstone_write_forbidden",
        "kg_live_write_forbidden",
        "provider_model_invocation_forbidden",
        "credential_channel_public_release_forbidden",
        "install_restart_active_binary_filesystem_mutation_forbidden",
    ] {
        assert_eq!(
            value[key], true,
            "minimal scoped Memory WAL/receipt field should be true: {key}"
        );
    }
    for key in [
        "single_use_nonce_consumed",
        "explicit_command_dispatched",
        "wal_write_performed",
        "wal_recorded",
        "wal_persisted",
        "receipt_recorded",
        "receipt_persisted",
        "receipt_materialized",
        "receipt_delivered",
        "post_write_readback_performed",
        "rollback_executed",
        "tombstone_written",
        "activation_performed",
        "live_mutation_execution_performed",
        "memory_write_execution_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "durable_memory_store_write_performed",
        "durable_memory_store_read_performed",
        "durable_memory_store_rollback_performed",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "channel_send_performed",
        "external_send_performed",
        "release_artifact_written",
        "install_executed",
        "service_restarted",
        "active_binary_mutated",
    ] {
        assert_eq!(
            value[key], false,
            "minimal scoped Memory WAL/receipt side-effect field should stay false: {key}"
        );
    }
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "run_minimal_scoped_memory_real_write_canary_wal_receipt_binding_boundary_require_live_gate"
    );
    assert_eq!(value["allowed_next_actions"][0]["writes_memory"], false);
    assert_eq!(value["allowed_next_actions"][0]["writes_wal"], false);
    assert_eq!(
        value["allowed_next_actions"][1]["action"],
        "prepare_minimal_scoped_memory_real_write_canary_post_write_readback_binding_boundary"
    );
    assert_eq!(
        value["allowed_next_actions"][1]["requires_minimal_scoped_memory_real_write_canary_wal_receipt_binding"],
        true
    );
    assert_eq!(value["allowed_next_actions"][1]["persists_receipt"], false);
    let side_effects = value["side_effects"]
        .as_object()
        .expect("minimal scoped Memory WAL/receipt side effects");
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_write_execution_minimal_scoped_memory_real_write_canary_post_write_readback_binding_accepts_bindings_without_reads_or_writes()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_MINIMAL_SCOPED_MEMORY_REAL_WRITE_CANARY_POST_WRITE_READBACK_BINDING_BOUNDARY_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body)
        .expect("minimal scoped Memory real-write canary post-write readback binding json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_MINIMAL_SCOPED_MEMORY_REAL_WRITE_CANARY_POST_WRITE_READBACK_BINDING_BOUNDARY_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-post-write-readback-binding-boundary --json"
    );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(value["route_count"], NATIVE_GATEWAY_SOURCE_COMMAND_COUNT);
    assert_eq!(
        value["implemented_route_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(
        value["memory_write_execution_minimal_scoped_memory_real_write_canary_post_write_readback_binding_boundary_ready"],
        true
    );
    assert_eq!(
        value["minimal_scoped_memory_real_write_canary_post_write_readback_binding_ready"],
        true
    );
    assert_eq!(
        value["minimal_scoped_memory_real_write_canary_post_write_readback_binding_accepted_no_read_or_write"],
        true
    );
    assert_eq!(
        value["source_minimal_scoped_memory_real_write_canary_wal_receipt_binding_boundary_ready"],
        true
    );
    assert_eq!(
        value["source_accepted_minimal_scoped_memory_real_write_canary_wal_receipt_binding_fixture_count"],
        1
    );
    assert_eq!(
        value["source_blocked_minimal_scoped_memory_real_write_canary_wal_receipt_binding_fixture_count"],
        9
    );
    assert_eq!(
        value["source_wal_receipt_binding_authority_accepted_count"],
        1
    );
    assert_eq!(value["source_post_write_readback_handoff_bound_count"], 1);
    assert_eq!(value["source_single_use_nonce_consumed_count"], 0);
    assert_eq!(value["source_explicit_command_dispatched_count"], 0);
    assert_eq!(value["source_wal_write_performed_count"], 0);
    assert_eq!(value["source_receipt_persisted_count"], 0);
    assert_eq!(value["source_post_write_readback_performed_count"], 0);
    assert_eq!(
        value["scoped_memory_real_write_canary_mode"],
        "minimal_scoped_memory_real_write_canary_post_write_readback_binding_no_read_or_write"
    );
    assert_eq!(
        value["required_minimal_scoped_memory_real_write_canary_post_write_readback_binding_surface_count"],
        12
    );
    assert_eq!(
        value["ready_minimal_scoped_memory_real_write_canary_post_write_readback_binding_surface_count"],
        12
    );
    assert_eq!(
        value["side_effect_free_minimal_scoped_memory_real_write_canary_post_write_readback_binding_surface_count"],
        12
    );
    assert_eq!(
        value["required_minimal_scoped_memory_real_write_canary_post_write_readback_binding_fixture_count"],
        10
    );
    assert_eq!(
        value["minimal_scoped_memory_real_write_canary_post_write_readback_binding_fixture_count"],
        10
    );
    assert_eq!(
        value["accepted_minimal_scoped_memory_real_write_canary_post_write_readback_binding_fixture_count"],
        1
    );
    assert_eq!(
        value["blocked_minimal_scoped_memory_real_write_canary_post_write_readback_binding_fixture_count"],
        9
    );
    for key in [
        "post_write_readback_binding_authority_accepted_count",
        "post_write_readback_plan_bound_count",
        "readback_result_identity_bound_count",
        "readback_receipt_linkage_bound_count",
        "readback_payload_digest_comparison_bound_count",
        "readback_namespace_store_scope_bound_count",
        "readback_redaction_secret_scan_bound_count",
        "readback_stale_guard_bound_count",
        "readback_phantom_guard_bound_count",
        "readback_operator_review_handoff_bound_count",
        "rollback_tombstone_handoff_bound_count",
        "minimal_scoped_memory_real_write_canary_post_write_readback_binding_accepted_count",
    ] {
        assert_eq!(
            value[key], 1,
            "minimal scoped Memory post-write readback binding count should be one: {key}"
        );
    }
    for key in [
        "single_use_nonce_consumed_count",
        "explicit_command_dispatched_count",
        "wal_write_performed_count",
        "wal_recorded_count",
        "wal_persisted_count",
        "receipt_recorded_count",
        "receipt_persisted_count",
        "receipt_materialized_count",
        "receipt_delivered_count",
        "post_write_readback_performed_count",
        "readback_result_recorded_count",
        "readback_result_persisted_count",
        "readback_result_accepted_count",
        "rollback_performed_count",
        "tombstone_written_count",
        "durable_memory_store_read_performed_count",
        "durable_memory_store_rollback_performed_count",
        "durable_memory_store_write_performed_count",
        "memory_store_write_performed_count",
    ] {
        assert_eq!(
            value[key], 0,
            "minimal scoped Memory post-write readback binding side-effect count should stay zero: {key}"
        );
    }
    assert_eq!(
        value["required_before_minimal_scoped_memory_real_write_canary_post_write_readback_binding_count"],
        19
    );
    assert_eq!(
            value["required_minimal_scoped_memory_real_write_canary_post_write_readback_binding_fields"]
                .as_array()
                .expect("required minimal scoped Memory post-write readback fields")
                .len(),
            19
        );
    let fixtures =
        value["minimal_scoped_memory_real_write_canary_post_write_readback_binding_fixtures"]
            .as_array()
            .expect("minimal scoped Memory post-write readback fixtures");
    assert_eq!(fixtures.len(), 10);
    assert_eq!(
            fixtures
                .iter()
                .filter(|fixture| {
                    fixture["minimal_scoped_memory_real_write_canary_post_write_readback_binding_accepted"]
                        == true
                })
                .count(),
            1
        );
    assert!(fixtures.iter().all(|fixture| {
        fixture["single_use_nonce_consumed"].as_bool() == Some(false)
            && fixture["explicit_command_dispatched"].as_bool() == Some(false)
            && fixture["wal_write_performed"].as_bool() == Some(false)
            && fixture["wal_persisted"].as_bool() == Some(false)
            && fixture["receipt_recorded"].as_bool() == Some(false)
            && fixture["receipt_persisted"].as_bool() == Some(false)
            && fixture["receipt_materialized"].as_bool() == Some(false)
            && fixture["receipt_delivered"].as_bool() == Some(false)
            && fixture["post_write_readback_performed"].as_bool() == Some(false)
            && fixture["readback_result_recorded"].as_bool() == Some(false)
            && fixture["readback_result_persisted"].as_bool() == Some(false)
            && fixture["readback_result_accepted"].as_bool() == Some(false)
            && fixture["rollback_executed"].as_bool() == Some(false)
            && fixture["tombstone_written"].as_bool() == Some(false)
            && fixture["memory_write_execution_performed"].as_bool() == Some(false)
            && fixture["memory_store_write_performed"].as_bool() == Some(false)
            && fixture["memory_store_mutated"].as_bool() == Some(false)
            && fixture["durable_memory_store_read_performed"].as_bool() == Some(false)
            && fixture["durable_memory_store_write_performed"].as_bool() == Some(false)
            && fixture["durable_memory_store_rollback_performed"].as_bool() == Some(false)
            && fixture["live_kg_write_performed"].as_bool() == Some(false)
            && fixture["provider_invoked"].as_bool() == Some(false)
            && fixture["model_invoked"].as_bool() == Some(false)
            && fixture["credential_read"].as_bool() == Some(false)
            && fixture["channel_send_performed"].as_bool() == Some(false)
            && fixture["external_send_performed"].as_bool() == Some(false)
            && fixture["release_artifact_written"].as_bool() == Some(false)
            && fixture["install_executed"].as_bool() == Some(false)
            && fixture["active_binary_mutated"].as_bool() == Some(false)
            && fixture["post_write_readback_binding_noop_confirmed"].as_bool() == Some(true)
    }));
    let accepted = fixtures
        .iter()
        .find(|fixture| {
            fixture["minimal_scoped_memory_real_write_canary_post_write_readback_binding_accepted"]
                == true
        })
        .expect("accepted post-write readback binding fixture");
    assert_eq!(
        accepted["minimal_scoped_memory_real_write_canary_post_write_readback_binding_status"],
        "accepted_post_write_readback_binding_no_read_or_write"
    );
    assert_eq!(accepted["post_write_readback_plan_bound"], true);
    assert_eq!(accepted["readback_result_identity_bound"], true);
    assert_eq!(accepted["readback_receipt_linkage_bound"], true);
    assert_eq!(accepted["readback_payload_digest_comparison_bound"], true);
    assert_eq!(accepted["readback_namespace_store_scope_bound"], true);
    assert_eq!(accepted["readback_redaction_secret_scan_bound"], true);
    assert_eq!(accepted["readback_stale_guard_bound"], true);
    assert_eq!(accepted["readback_phantom_guard_bound"], true);
    assert_eq!(accepted["readback_operator_review_handoff_bound"], true);
    assert_eq!(accepted["rollback_tombstone_handoff_bound"], true);
    let denied =
            value["denied_by_minimal_scoped_memory_real_write_canary_post_write_readback_binding_boundary"]
                .as_array()
                .expect("minimal scoped Memory post-write readback denials");
    assert_eq!(denied.len(), 30);
    assert_eq!(
        value["denied_by_minimal_scoped_memory_real_write_canary_post_write_readback_binding_boundary_count"],
        30
    );
    for key in [
        "source_minimal_scoped_memory_real_write_canary_wal_receipt_binding_required",
        "minimal_scoped_memory_real_write_canary_post_write_readback_binding_accepted",
        "post_write_readback_plan_bound",
        "readback_result_identity_bound",
        "readback_receipt_linkage_bound",
        "readback_payload_digest_comparison_bound",
        "readback_namespace_store_scope_bound",
        "readback_redaction_secret_scan_bound",
        "readback_stale_guard_bound",
        "readback_phantom_guard_bound",
        "readback_operator_review_handoff_bound",
        "rollback_tombstone_handoff_bound",
        "nonce_consumption_forbidden_on_report_route",
        "explicit_command_dispatch_forbidden_on_report_route",
        "wal_write_forbidden",
        "wal_persistence_forbidden",
        "receipt_recording_forbidden",
        "receipt_persistence_forbidden",
        "post_write_readback_forbidden_on_report_route",
        "readback_result_recording_forbidden",
        "readback_result_persistence_forbidden",
        "readback_acceptance_forbidden",
        "durable_memory_read_forbidden",
        "durable_memory_write_forbidden",
        "durable_memory_rollback_forbidden",
        "memory_store_mutation_forbidden",
        "rollback_execution_forbidden",
        "tombstone_write_forbidden",
        "kg_live_write_forbidden",
        "provider_model_invocation_forbidden",
        "credential_channel_public_release_forbidden",
        "install_restart_active_binary_filesystem_mutation_forbidden",
    ] {
        assert_eq!(
            value[key], true,
            "minimal scoped Memory post-write readback field should be true: {key}"
        );
    }
    for key in [
        "single_use_nonce_consumed",
        "explicit_command_dispatched",
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
        "tombstone_written",
        "activation_performed",
        "live_mutation_execution_performed",
        "memory_write_execution_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "durable_memory_store_write_performed",
        "durable_memory_store_read_performed",
        "durable_memory_store_rollback_performed",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "channel_send_performed",
        "external_send_performed",
        "release_artifact_written",
        "install_executed",
        "service_restarted",
        "active_binary_mutated",
    ] {
        assert_eq!(
            value[key], false,
            "minimal scoped Memory post-write readback side-effect field should stay false: {key}"
        );
    }
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "run_minimal_scoped_memory_real_write_canary_post_write_readback_binding_boundary_require_live_gate"
    );
    assert_eq!(value["allowed_next_actions"][0]["reads_memory"], false);
    assert_eq!(value["allowed_next_actions"][0]["records_readback"], false);
    assert_eq!(
        value["allowed_next_actions"][1]["action"],
        "prepare_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_boundary"
    );
    assert_eq!(
        value["allowed_next_actions"][1]["requires_minimal_scoped_memory_real_write_canary_post_write_readback_binding"],
        true
    );
    assert_eq!(value["allowed_next_actions"][1]["executes_rollback"], false);
    let side_effects = value["side_effects"]
        .as_object()
        .expect("minimal scoped Memory post-write readback side effects");
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_write_execution_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_accepts_proof_without_rollback_or_writes()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_MINIMAL_SCOPED_MEMORY_REAL_WRITE_CANARY_ROLLBACK_TOMBSTONE_PROOF_BOUNDARY_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body)
        .expect("minimal scoped Memory real-write canary rollback/tombstone proof json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_MINIMAL_SCOPED_MEMORY_REAL_WRITE_CANARY_ROLLBACK_TOMBSTONE_PROOF_BOUNDARY_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-rollback-tombstone-proof-boundary --json"
    );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(value["route_count"], NATIVE_GATEWAY_SOURCE_COMMAND_COUNT);
    assert_eq!(
        value["implemented_route_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(
        value["memory_write_execution_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_boundary_ready"],
        true
    );
    assert_eq!(
        value["minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_ready"],
        true
    );
    assert_eq!(
        value["minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_accepted_no_rollback_or_write"],
        true
    );
    assert_eq!(
        value["source_minimal_scoped_memory_real_write_canary_post_write_readback_binding_boundary_ready"],
        true
    );
    assert_eq!(
        value["source_accepted_minimal_scoped_memory_real_write_canary_post_write_readback_binding_fixture_count"],
        1
    );
    assert_eq!(
        value["source_blocked_minimal_scoped_memory_real_write_canary_post_write_readback_binding_fixture_count"],
        9
    );
    assert_eq!(
        value["source_post_write_readback_binding_authority_accepted_count"],
        1
    );
    assert_eq!(value["source_rollback_tombstone_handoff_bound_count"], 1);
    for key in [
        "source_single_use_nonce_consumed_count",
        "source_explicit_command_dispatched_count",
        "source_wal_write_performed_count",
        "source_receipt_persisted_count",
        "source_post_write_readback_performed_count",
        "source_readback_result_recorded_count",
        "source_readback_result_persisted_count",
        "source_readback_result_accepted_count",
        "source_rollback_performed_count",
        "source_tombstone_written_count",
        "source_durable_memory_store_read_performed_count",
        "source_durable_memory_store_write_performed_count",
        "source_durable_memory_store_rollback_performed_count",
        "source_memory_store_write_performed_count",
    ] {
        assert_eq!(
            value[key], 0,
            "minimal scoped Memory rollback/tombstone proof source side-effect count should stay zero: {key}"
        );
    }
    assert_eq!(
        value["scoped_memory_real_write_canary_mode"],
        "minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_no_rollback_or_write"
    );
    assert_eq!(
        value["required_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_surface_count"],
        12
    );
    assert_eq!(
        value["ready_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_surface_count"],
        12
    );
    assert_eq!(
        value["side_effect_free_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_surface_count"],
        12
    );
    assert_eq!(
        value["required_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_fixture_count"],
        10
    );
    assert_eq!(
        value["minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_fixture_count"],
        10
    );
    assert_eq!(
        value["accepted_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_fixture_count"],
        1
    );
    assert_eq!(
        value["blocked_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_fixture_count"],
        9
    );
    for key in [
        "rollback_tombstone_proof_authority_accepted_count",
        "rollback_plan_proof_bound_count",
        "tombstone_plan_proof_bound_count",
        "rollback_target_proof_bound_count",
        "tombstone_target_proof_bound_count",
        "rollback_receipt_linkage_proof_bound_count",
        "tombstone_receipt_linkage_proof_bound_count",
        "rollback_idempotency_guard_proof_bound_count",
        "tombstone_idempotency_guard_proof_bound_count",
        "rollback_tombstone_audit_evidence_proof_bound_count",
        "operator_review_handoff_proof_bound_count",
        "minimal_real_write_canary_handoff_proof_bound_count",
        "minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_accepted_count",
    ] {
        assert_eq!(
            value[key], 1,
            "minimal scoped Memory rollback/tombstone proof binding count should be one: {key}"
        );
    }
    for key in [
        "single_use_nonce_consumed_count",
        "explicit_command_dispatched_count",
        "wal_write_performed_count",
        "receipt_persisted_count",
        "post_write_readback_performed_count",
        "readback_result_recorded_count",
        "readback_result_persisted_count",
        "readback_result_accepted_count",
        "rollback_performed_count",
        "rollback_result_recorded_count",
        "rollback_result_persisted_count",
        "rollback_result_accepted_count",
        "tombstone_written_count",
        "compensating_memory_write_performed_count",
        "durable_memory_store_read_performed_count",
        "durable_memory_store_rollback_performed_count",
        "durable_memory_store_write_performed_count",
        "memory_store_write_performed_count",
    ] {
        assert_eq!(
            value[key], 0,
            "minimal scoped Memory rollback/tombstone proof side-effect count should stay zero: {key}"
        );
    }
    assert_eq!(
        value["required_before_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_count"],
        15
    );
    assert_eq!(
        value["required_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_fields"]
            .as_array()
            .expect("required minimal scoped Memory rollback/tombstone proof fields")
            .len(),
        15
    );
    let fixtures =
        value["minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_fixtures"]
            .as_array()
            .expect("minimal scoped Memory rollback/tombstone proof fixtures");
    assert_eq!(fixtures.len(), 10);
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_accepted"]
                    == true
            })
            .count(),
        1
    );
    assert!(fixtures.iter().all(|fixture| {
        fixture["single_use_nonce_consumed"].as_bool() == Some(false)
            && fixture["explicit_command_dispatched"].as_bool() == Some(false)
            && fixture["wal_write_performed"].as_bool() == Some(false)
            && fixture["receipt_persisted"].as_bool() == Some(false)
            && fixture["post_write_readback_performed"].as_bool() == Some(false)
            && fixture["readback_result_recorded"].as_bool() == Some(false)
            && fixture["readback_result_persisted"].as_bool() == Some(false)
            && fixture["readback_result_accepted"].as_bool() == Some(false)
            && fixture["rollback_executed"].as_bool() == Some(false)
            && fixture["rollback_result_recorded"].as_bool() == Some(false)
            && fixture["rollback_result_persisted"].as_bool() == Some(false)
            && fixture["rollback_result_accepted"].as_bool() == Some(false)
            && fixture["tombstone_written"].as_bool() == Some(false)
            && fixture["compensating_memory_write_performed"].as_bool() == Some(false)
            && fixture["memory_write_execution_performed"].as_bool() == Some(false)
            && fixture["memory_store_write_performed"].as_bool() == Some(false)
            && fixture["memory_store_mutated"].as_bool() == Some(false)
            && fixture["durable_memory_store_read_performed"].as_bool() == Some(false)
            && fixture["durable_memory_store_write_performed"].as_bool() == Some(false)
            && fixture["durable_memory_store_rollback_performed"].as_bool() == Some(false)
            && fixture["live_kg_write_performed"].as_bool() == Some(false)
            && fixture["provider_invoked"].as_bool() == Some(false)
            && fixture["model_invoked"].as_bool() == Some(false)
            && fixture["credential_read"].as_bool() == Some(false)
            && fixture["channel_send_performed"].as_bool() == Some(false)
            && fixture["external_send_performed"].as_bool() == Some(false)
            && fixture["release_artifact_written"].as_bool() == Some(false)
            && fixture["install_executed"].as_bool() == Some(false)
            && fixture["active_binary_mutated"].as_bool() == Some(false)
            && fixture["rollback_tombstone_proof_noop_confirmed"].as_bool() == Some(true)
    }));
    let accepted = fixtures
        .iter()
        .find(|fixture| {
            fixture["minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_accepted"]
                == true
        })
        .expect("accepted rollback/tombstone proof fixture");
    assert_eq!(
        accepted["minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_status"],
        "accepted_rollback_tombstone_proof_no_rollback_or_write"
    );
    assert_eq!(accepted["rollback_plan_proof_bound"], true);
    assert_eq!(accepted["tombstone_plan_proof_bound"], true);
    assert_eq!(accepted["rollback_target_proof_bound"], true);
    assert_eq!(accepted["tombstone_target_proof_bound"], true);
    assert_eq!(accepted["rollback_receipt_linkage_proof_bound"], true);
    assert_eq!(accepted["tombstone_receipt_linkage_proof_bound"], true);
    assert_eq!(accepted["rollback_idempotency_guard_proof_bound"], true);
    assert_eq!(accepted["tombstone_idempotency_guard_proof_bound"], true);
    assert_eq!(
        accepted["rollback_tombstone_audit_evidence_proof_bound"],
        true
    );
    assert_eq!(accepted["operator_review_handoff_proof_bound"], true);
    assert_eq!(
        accepted["minimal_real_write_canary_handoff_proof_bound"],
        true
    );
    let denied =
            value["denied_by_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_boundary"]
                .as_array()
                .expect("minimal scoped Memory rollback/tombstone proof denials");
    assert_eq!(denied.len(), 36);
    assert_eq!(
        value["denied_by_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_boundary_count"],
        36
    );
    for key in [
        "source_minimal_scoped_memory_real_write_canary_post_write_readback_binding_required",
        "minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_accepted",
        "rollback_plan_proof_bound",
        "tombstone_plan_proof_bound",
        "rollback_target_proof_bound",
        "tombstone_target_proof_bound",
        "rollback_receipt_linkage_proof_bound",
        "tombstone_receipt_linkage_proof_bound",
        "rollback_idempotency_guard_proof_bound",
        "tombstone_idempotency_guard_proof_bound",
        "rollback_tombstone_audit_evidence_proof_bound",
        "operator_review_handoff_proof_bound",
        "minimal_real_write_canary_handoff_proof_bound",
        "nonce_consumption_forbidden_on_report_route",
        "explicit_command_dispatch_forbidden_on_report_route",
        "wal_write_forbidden",
        "receipt_persistence_forbidden",
        "post_write_readback_forbidden_on_report_route",
        "readback_result_recording_forbidden",
        "readback_result_persistence_forbidden",
        "readback_acceptance_forbidden",
        "rollback_execution_forbidden",
        "tombstone_write_forbidden",
        "durable_memory_read_forbidden",
        "durable_memory_write_forbidden",
        "durable_memory_rollback_forbidden",
        "memory_store_mutation_forbidden",
        "kg_live_write_forbidden",
        "provider_model_invocation_forbidden",
        "credential_channel_public_release_forbidden",
        "install_restart_active_binary_filesystem_mutation_forbidden",
    ] {
        assert_eq!(
            value[key], true,
            "minimal scoped Memory rollback/tombstone proof field should be true: {key}"
        );
    }
    for key in [
        "single_use_nonce_consumed",
        "explicit_command_dispatched",
        "wal_write_performed",
        "receipt_persisted",
        "post_write_readback_performed",
        "readback_result_recorded",
        "readback_result_persisted",
        "readback_result_accepted",
        "rollback_executed",
        "rollback_result_recorded",
        "rollback_result_persisted",
        "rollback_result_accepted",
        "tombstone_written",
        "compensating_memory_write_performed",
        "activation_performed",
        "live_mutation_execution_performed",
        "memory_write_execution_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "durable_memory_store_write_performed",
        "durable_memory_store_read_performed",
        "durable_memory_store_rollback_performed",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "channel_send_performed",
        "external_send_performed",
        "release_artifact_written",
        "install_executed",
        "service_restarted",
        "active_binary_mutated",
    ] {
        assert_eq!(
            value[key], false,
            "minimal scoped Memory rollback/tombstone proof side-effect field should stay false: {key}"
        );
    }
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "run_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_boundary_require_live_gate"
    );
    assert_eq!(value["allowed_next_actions"][0]["executes_rollback"], false);
    assert_eq!(value["allowed_next_actions"][0]["writes_tombstone"], false);
    assert_eq!(
        value["allowed_next_actions"][1]["action"],
        "prepare_minimal_scoped_memory_real_write_canary_execution_boundary"
    );
    assert_eq!(
        value["allowed_next_actions"][1]["requires_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof"],
        true
    );
    assert_eq!(value["allowed_next_actions"][1]["writes_memory"], false);
    let side_effects = value["side_effects"]
        .as_object()
        .expect("minimal scoped Memory rollback/tombstone proof side effects");
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_write_execution_minimal_scoped_memory_real_write_canary_execution_writes_reads_and_rolls_back_scoped_store_without_external_side_effects()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_MINIMAL_SCOPED_MEMORY_REAL_WRITE_CANARY_EXECUTION_BOUNDARY_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body)
        .expect("minimal scoped Memory real-write canary execution json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_MINIMAL_SCOPED_MEMORY_REAL_WRITE_CANARY_EXECUTION_BOUNDARY_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-execution-boundary --json"
    );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(value["route_count"], NATIVE_GATEWAY_SOURCE_COMMAND_COUNT);
    assert_eq!(
        value["implemented_route_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(
        value["memory_write_execution_minimal_scoped_memory_real_write_canary_execution_boundary_ready"],
        true
    );
    assert_eq!(
        value["minimal_scoped_memory_real_write_canary_execution_ready"],
        true
    );
    assert_eq!(
        value["minimal_scoped_memory_real_write_canary_execution_performed"],
        true
    );
    assert_eq!(
        value["scoped_memory_real_write_canary_mode"],
        "minimal_scoped_memory_real_write_canary_execution_isolated_in_memory_store_write_readback_rollback"
    );
    assert_eq!(
        value["source_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_boundary_ready"],
        true
    );
    assert_eq!(
        value["source_accepted_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_fixture_count"],
        1
    );
    assert_eq!(
        value["source_blocked_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_fixture_count"],
        9
    );
    assert_eq!(
        value["source_rollback_tombstone_proof_authority_accepted_count"],
        1
    );
    assert_eq!(
        value["source_minimal_real_write_canary_handoff_proof_bound_count"],
        1
    );
    for key in [
        "source_single_use_nonce_consumed_count",
        "source_explicit_command_dispatched_count",
        "source_wal_write_performed_count",
        "source_receipt_persisted_count",
        "source_post_write_readback_performed_count",
        "source_readback_result_accepted_count",
        "source_rollback_performed_count",
        "source_tombstone_written_count",
        "source_durable_memory_store_read_performed_count",
        "source_durable_memory_store_write_performed_count",
        "source_durable_memory_store_rollback_performed_count",
        "source_memory_store_write_performed_count",
    ] {
        assert_eq!(
            value[key], 0,
            "execution source proof side-effect count should stay zero: {key}"
        );
    }
    assert_eq!(value["approved_namespace"], "hepta.memory.canary");
    assert_eq!(value["approved_store"], "in-memory-reference");
    assert_eq!(value["approved_scope"], "session");
    assert_eq!(
        value["canary_record_id"],
        "hepta-minimal-scoped-memory-real-write-canary-execution-record-v1"
    );
    assert_eq!(value["canary_payload_plaintext_recorded"], false);
    assert_eq!(value["pre_write_snapshot_memory_count"], 0);
    assert_eq!(value["post_write_snapshot_memory_count"], 1);
    assert_eq!(value["post_write_readback_hit_count"], 1);
    assert_eq!(value["post_write_readback_identity_match"], true);
    assert_eq!(value["post_write_readback_digest_match"], true);
    assert_eq!(value["rollback_restore_result"], true);
    assert_eq!(value["post_rollback_snapshot_memory_count"], 0);
    assert_eq!(value["post_rollback_absence_confirmed"], true);
    assert_eq!(
        value["minimal_scoped_memory_real_write_canary_execution_isolated_store_restored"],
        true
    );
    assert_eq!(
        value["required_minimal_scoped_memory_real_write_canary_execution_surface_count"],
        12
    );
    assert_eq!(
        value["ready_minimal_scoped_memory_real_write_canary_execution_surface_count"],
        12
    );
    assert_eq!(
        value["minimal_scoped_memory_real_write_canary_execution_fixture_count"],
        10
    );
    assert_eq!(
        value["accepted_minimal_scoped_memory_real_write_canary_execution_fixture_count"],
        1
    );
    assert_eq!(
        value["blocked_minimal_scoped_memory_real_write_canary_execution_fixture_count"],
        9
    );
    for key in [
        "minimal_scoped_memory_real_write_canary_execution_accepted_count",
        "isolated_memory_store_write_bound_count",
        "post_write_readback_bound_count",
        "rollback_restore_bound_count",
        "post_rollback_absence_bound_count",
        "live_mutation_execution_performed_count",
        "memory_write_execution_performed_count",
        "memory_store_write_performed_count",
        "post_write_readback_performed_count",
        "readback_result_recorded_count",
        "readback_result_accepted_count",
        "rollback_performed_count",
        "rollback_result_recorded_count",
        "rollback_result_accepted_count",
    ] {
        assert_eq!(
            value[key], 1,
            "execution accepted/write/readback/rollback count should be one: {key}"
        );
    }
    for key in [
        "single_use_nonce_consumed_count",
        "explicit_command_dispatched_count",
        "wal_write_performed_count",
        "receipt_persisted_count",
        "readback_result_persisted_count",
        "tombstone_written_count",
        "durable_memory_store_read_performed_count",
        "durable_memory_store_write_performed_count",
        "durable_memory_store_rollback_performed_count",
        "kg_live_write_performed_count",
        "provider_invoked_count",
        "model_invoked_count",
        "credential_read_count",
        "channel_send_performed_count",
        "external_send_performed_count",
        "release_artifact_written_count",
        "install_executed_count",
        "service_restarted_count",
        "active_binary_mutated_count",
    ] {
        assert_eq!(
            value[key], 0,
            "execution external/durable side-effect count should stay zero: {key}"
        );
    }
    for key in [
        "live_mutation_execution_performed",
        "memory_write_execution_performed",
        "memory_store_write_path_enabled",
        "memory_store_write_allowed",
        "memory_store_write_performed",
        "memory_store_mutation_allowed",
        "memory_store_mutated",
        "post_write_readback_performed",
        "readback_result_recorded",
        "readback_result_accepted",
        "rollback_executed",
        "rollback_result_recorded",
        "rollback_result_accepted",
    ] {
        assert_eq!(
            value[key], true,
            "execution isolated store side-effect field should be true: {key}"
        );
    }
    for key in [
        "single_use_nonce_consumed",
        "explicit_command_dispatched",
        "wal_write_performed",
        "receipt_persisted",
        "readback_result_persisted",
        "tombstone_written",
        "durable_memory_store_write_performed",
        "durable_memory_store_read_performed",
        "durable_memory_store_rollback_performed",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "channel_send_performed",
        "external_send_performed",
        "release_artifact_written",
        "install_executed",
        "service_restarted",
        "active_binary_mutated",
        "filesystem_written",
    ] {
        assert_eq!(
            value[key], false,
            "execution external/durable side-effect field should stay false: {key}"
        );
    }
    let fixtures = value["minimal_scoped_memory_real_write_canary_execution_fixtures"]
        .as_array()
        .expect("minimal scoped Memory execution fixtures");
    assert_eq!(fixtures.len(), 10);
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["minimal_scoped_memory_real_write_canary_execution_accepted"] == true
            })
            .count(),
        1
    );
    let denied = value["denied_by_minimal_scoped_memory_real_write_canary_execution_boundary"]
        .as_array()
        .expect("minimal scoped Memory execution denials");
    assert_eq!(denied.len(), 25);
    assert_eq!(
        value["denied_by_minimal_scoped_memory_real_write_canary_execution_boundary_count"],
        25
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "run_minimal_scoped_memory_real_write_canary_execution_boundary_require_live_gate"
    );
    assert_eq!(value["allowed_next_actions"][0]["writes_memory"], true);
    assert_eq!(value["allowed_next_actions"][0]["reads_memory"], true);
    assert_eq!(value["allowed_next_actions"][0]["executes_rollback"], true);
    assert_eq!(
        value["allowed_next_actions"][0]["writes_durable_memory"],
        false
    );
    assert_eq!(
        value["allowed_next_actions"][1]["action"],
        "prepare_minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_boundary"
    );
    assert_eq!(
        value["allowed_next_actions"][1]["requires_minimal_scoped_memory_real_write_canary_execution_boundary"],
        true
    );
    let side_effects = value["side_effects"]
        .as_object()
        .expect("minimal scoped Memory execution side effects");
    assert_eq!(
        side_effects["memory_store_write_performed"].as_bool(),
        Some(true)
    );
    assert_eq!(
        side_effects["durable_memory_store_write_performed"].as_bool(),
        Some(false)
    );
    assert_eq!(
        side_effects["external_send_performed"].as_bool(),
        Some(false)
    );
}

#[test]
fn hepta_memory_write_execution_scoped_production_durable_memory_write_preflight_binds_target_without_production_or_external_side_effects()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_PRODUCTION_DURABLE_MEMORY_WRITE_PREFLIGHT_BOUNDARY_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body)
        .expect("scoped production durable Memory write preflight boundary json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_PRODUCTION_DURABLE_MEMORY_WRITE_PREFLIGHT_BOUNDARY_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-preflight-boundary --json"
    );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(value["route_count"], NATIVE_GATEWAY_SOURCE_COMMAND_COUNT);
    assert_eq!(
        value["implemented_route_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(
        value["memory_write_execution_scoped_production_durable_memory_write_preflight_boundary_ready"],
        true
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_preflight_ready"],
        true
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_preflight_performed"],
        true
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_preflight_accepted"],
        true
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_preflight_mode"],
        "preflight_only_no_production_durable_memory_mutation"
    );
    assert_eq!(
        value["source_minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_boundary_ready"],
        true
    );
    assert_eq!(
        value["source_accepted_zero_residue_acceptance_fixture_count"],
        1
    );
    assert_eq!(
        value["source_blocked_zero_residue_acceptance_fixture_count"],
        9
    );
    assert_eq!(
        value["source_zero_residue_acceptance_result_accepted_count"],
        1
    );
    assert_eq!(
        value["source_single_shot_memory_store_write_performed_count"],
        1
    );
    assert_eq!(value["source_single_shot_wal_write_performed_count"], 1);
    assert_eq!(value["source_single_shot_receipt_persisted_count"], 1);
    assert_eq!(
        value["source_single_shot_post_write_readback_performed_count"],
        1
    );
    assert_eq!(value["source_single_shot_rollback_executed_count"], 1);
    assert_eq!(
        value["source_single_shot_tombstone_cleanup_executed_count"],
        1
    );
    assert_eq!(
        value["source_single_shot_canary_post_rollback_memory_count"],
        0
    );
    assert_eq!(
        value["source_single_shot_canary_artifact_post_cleanup_count"],
        0
    );
    assert_eq!(
        value["source_single_shot_canary_artifact_zero_residue_confirmed"],
        true
    );
    assert_eq!(
        value["source_current_memory_store_write_performed_count"],
        0
    );
    assert_eq!(value["source_current_wal_write_performed_count"], 0);
    assert_eq!(value["source_current_receipt_persisted_count"], 0);
    assert_eq!(value["source_current_rollback_executed_count"], 0);
    assert_eq!(value["source_current_tombstone_cleanup_executed_count"], 0);
    assert_eq!(
        value["source_current_durable_memory_store_write_performed_count"],
        0
    );
    assert_eq!(value["source_current_external_send_performed_count"], 0);
    assert_eq!(
        value["approved_production_namespace"],
        "hepta.memory.production.scoped"
    );
    assert_eq!(
        value["approved_production_store"],
        "hepta-memory-durable-store-production-preflight-only"
    );
    assert_eq!(
        value["approved_production_scope"],
        "operator-approved-session"
    );
    assert_eq!(
        value["production_durable_memory_target_id"],
        "hepta-scoped-production-durable-memory-write-target-v1"
    );
    assert_eq!(
        value["production_durable_memory_payload_class"],
        "redacted-minimal-operator-approved-memory-fact"
    );
    for key in [
        "source_zero_residue_acceptance_boundary_hash_sha256",
        "source_zero_residue_acceptance_policy_hash_sha256",
        "source_zero_residue_acceptance_hash_sha256",
        "production_durable_memory_write_preflight_target_hash_sha256",
        "production_durable_memory_write_preflight_operator_packet_hash_sha256",
        "production_durable_memory_write_preflight_nonce_hash_sha256",
        "production_durable_memory_write_preflight_command_hash_sha256",
        "production_durable_memory_write_preflight_payload_redaction_hash_sha256",
        "production_durable_memory_write_preflight_wal_receipt_plan_hash_sha256",
        "production_durable_memory_write_preflight_readback_plan_hash_sha256",
        "production_durable_memory_write_preflight_rollback_tombstone_zero_residue_plan_hash_sha256",
        "scoped_production_durable_memory_write_preflight_result_hash_sha256",
        "scoped_production_durable_memory_write_preflight_boundary_hash_sha256",
        "scoped_production_durable_memory_write_preflight_policy_hash_sha256",
    ] {
        assert_ne!(
            value[key], "",
            "scoped production durable Memory write preflight hash should be present: {key}"
        );
    }
    assert_eq!(
        value["required_scoped_production_durable_memory_write_preflight_surface_count"],
        12
    );
    assert_eq!(
        value["ready_scoped_production_durable_memory_write_preflight_surface_count"],
        12
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_preflight_fixture_count"],
        10
    );
    assert_eq!(
        value["accepted_scoped_production_durable_memory_write_preflight_fixture_count"],
        1
    );
    assert_eq!(
        value["blocked_scoped_production_durable_memory_write_preflight_fixture_count"],
        9
    );
    assert_eq!(
        value["denied_by_scoped_production_durable_memory_write_preflight_boundary_count"],
        36
    );
    for key in [
        "scoped_production_durable_memory_write_preflight_performed_count",
        "scoped_production_durable_memory_write_preflight_result_recorded_count",
        "scoped_production_durable_memory_write_preflight_result_accepted_count",
        "source_zero_residue_acceptance_boundary_accepted_count",
        "production_durable_memory_target_bound_count",
        "operator_approval_packet_preflight_bound_count",
        "single_use_nonce_preflight_bound_count",
        "explicit_command_preflight_bound_count",
        "payload_redaction_preflight_bound_count",
        "wal_receipt_preflight_bound_count",
        "post_write_readback_preflight_bound_count",
        "rollback_tombstone_zero_residue_preflight_bound_count",
        "replay_idempotency_preflight_bound_count",
    ] {
        assert_eq!(
            value[key], 1,
            "scoped production durable Memory write preflight count should be one: {key}"
        );
    }
    for key in [
        "production_durable_memory_write_executed_count",
        "production_durable_memory_store_write_performed_count",
        "actual_production_durable_memory_write_performed_count",
        "durable_memory_store_write_performed_count",
        "durable_memory_store_read_performed_count",
        "durable_memory_store_rollback_performed_count",
        "memory_store_write_performed_count",
        "wal_write_performed_count",
        "receipt_persisted_count",
        "post_write_readback_performed_count",
        "rollback_performed_count",
        "tombstone_cleanup_executed_count",
        "live_kg_write_performed_count",
        "provider_invoked_count",
        "model_invoked_count",
        "credential_read_count",
        "channel_send_performed_count",
        "external_send_performed_count",
        "release_artifact_written_count",
        "install_executed_count",
        "service_restarted_count",
        "active_binary_mutated_count",
    ] {
        assert_eq!(
            value[key], 0,
            "scoped production durable Memory write preflight side-effect count should stay zero: {key}"
        );
    }
    for key in [
        "source_zero_residue_acceptance_boundary_bound",
        "approved_production_namespace_bound",
        "approved_production_store_bound",
        "approved_production_scope_bound",
        "production_durable_memory_target_bound",
        "operator_approval_packet_preflight_bound",
        "operator_identity_session_preflight_bound",
        "single_use_nonce_preflight_bound",
        "explicit_command_preflight_bound",
        "payload_redaction_preflight_bound",
        "wal_receipt_preflight_bound",
        "post_write_readback_preflight_bound",
        "rollback_tombstone_zero_residue_preflight_bound",
        "replay_idempotency_preflight_bound",
        "production_write_execution_forbidden_on_preflight_route",
        "production_durable_memory_write_forbidden",
        "memory_store_mutation_forbidden",
        "kg_live_write_forbidden",
        "provider_model_invocation_forbidden",
        "credential_channel_public_release_forbidden",
        "install_restart_active_binary_mutation_forbidden",
    ] {
        assert_eq!(
            value[key], true,
            "scoped production durable Memory write preflight field should be true: {key}"
        );
    }
    for key in [
        "production_durable_memory_write_executed",
        "production_durable_memory_store_write_performed",
        "actual_production_durable_memory_write_performed",
        "durable_memory_store_write_performed",
        "durable_memory_store_read_performed",
        "durable_memory_store_rollback_performed",
        "memory_write_execution_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "wal_write_performed",
        "wal_recorded",
        "wal_persisted",
        "receipt_recorded",
        "receipt_persisted",
        "receipt_materialized",
        "post_write_readback_performed",
        "rollback_executed",
        "rollback_performed",
        "tombstone_write_performed",
        "tombstone_cleanup_executed",
        "raw_payload_plaintext_recorded",
        "raw_payload_plaintext_persisted",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "channel_send_performed",
        "external_send_performed",
        "release_artifact_written",
        "install_executed",
        "service_restarted",
        "active_binary_mutated",
    ] {
        assert_eq!(
            value[key], false,
            "scoped production durable Memory write preflight external or mutation field should stay false: {key}"
        );
    }
    let fixtures = value["scoped_production_durable_memory_write_preflight_fixtures"]
        .as_array()
        .expect("scoped production durable Memory write preflight fixtures");
    assert_eq!(fixtures.len(), 10);
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["scoped_production_durable_memory_write_preflight_accepted"] == true
            })
            .count(),
        1
    );
    let denied = value["denied_by_scoped_production_durable_memory_write_preflight_boundary"]
        .as_array()
        .expect("scoped production durable Memory write preflight denials");
    assert_eq!(denied.len(), 36);
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "run_scoped_production_durable_memory_write_preflight_boundary_require_live_gate"
    );
    assert_eq!(
        value["allowed_next_actions"][0]["writes_production_durable_memory"],
        false
    );
    assert_eq!(
        value["allowed_next_actions"][0]["writes_memory_store"],
        false
    );
    assert_eq!(
        value["allowed_next_actions"][1]["action"],
        "prepare_scoped_production_durable_memory_write_operator_packet_acceptance_boundary"
    );
    assert_eq!(
        value["allowed_next_actions"][1]["requires_scoped_production_durable_memory_write_preflight_boundary"],
        true
    );
    let side_effects = value["side_effects"]
        .as_object()
        .expect("scoped production durable Memory write preflight side effects");
    assert_eq!(
        side_effects["scoped_production_durable_memory_write_preflight_performed"].as_bool(),
        Some(true)
    );
    assert_eq!(
        side_effects["scoped_production_durable_memory_write_preflight_result_accepted"].as_bool(),
        Some(true)
    );
    assert_eq!(
        side_effects["production_durable_memory_store_write_performed"].as_bool(),
        Some(false)
    );
    assert_eq!(
        side_effects["memory_store_write_performed"].as_bool(),
        Some(false)
    );
    assert_eq!(side_effects["wal_write_performed"].as_bool(), Some(false));
    assert_eq!(side_effects["receipt_persisted"].as_bool(), Some(false));
    assert_eq!(
        side_effects["external_send_performed"].as_bool(),
        Some(false)
    );
}

#[test]
fn hepta_memory_write_execution_scoped_production_durable_memory_write_operator_packet_acceptance_binds_packet_without_persistence_or_production_side_effects()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_PRODUCTION_DURABLE_MEMORY_WRITE_OPERATOR_PACKET_ACCEPTANCE_BOUNDARY_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body)
        .expect("scoped production durable Memory write operator packet acceptance boundary json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_PRODUCTION_DURABLE_MEMORY_WRITE_OPERATOR_PACKET_ACCEPTANCE_BOUNDARY_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-operator-packet-acceptance-boundary --json"
    );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(value["route_count"], NATIVE_GATEWAY_SOURCE_COMMAND_COUNT);
    assert_eq!(
        value["implemented_route_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(
        value["memory_write_execution_scoped_production_durable_memory_write_operator_packet_acceptance_boundary_ready"],
        true
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_operator_packet_acceptance_ready"],
        true
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_operator_packet_acceptance_performed"],
        true
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_operator_packet_acceptance_accepted"],
        true
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_operator_packet_acceptance_mode"],
        "acceptance_boundary_no_production_durable_memory_mutation_no_packet_persistence"
    );
    assert_eq!(
        value["source_scoped_production_durable_memory_write_preflight_boundary_ready"],
        true
    );
    assert_eq!(
        value["source_scoped_production_durable_memory_write_preflight_accepted_count"],
        1
    );
    assert_eq!(
        value["source_accepted_scoped_production_durable_memory_write_preflight_fixture_count"],
        1
    );
    assert_eq!(
        value["source_blocked_scoped_production_durable_memory_write_preflight_fixture_count"],
        9
    );
    assert_eq!(
        value["source_zero_residue_acceptance_result_accepted_count"],
        1
    );
    assert_eq!(
        value["approved_production_namespace"],
        "hepta.memory.production.scoped"
    );
    assert_eq!(
        value["approved_production_store"],
        "hepta-memory-durable-store-production-preflight-only"
    );
    assert_eq!(
        value["approved_production_scope"],
        "operator-approved-session"
    );
    assert_eq!(
        value["production_durable_memory_target_id"],
        "hepta-scoped-production-durable-memory-write-target-v1"
    );
    assert_eq!(
        value["production_durable_memory_payload_class"],
        "redacted-minimal-operator-approved-memory-fact"
    );
    for key in [
        "source_scoped_production_durable_memory_write_preflight_result_hash_sha256",
        "source_scoped_production_durable_memory_write_preflight_boundary_hash_sha256",
        "source_scoped_production_durable_memory_write_preflight_policy_hash_sha256",
        "source_production_durable_memory_write_preflight_target_hash_sha256",
        "source_production_durable_memory_write_preflight_operator_packet_hash_sha256",
        "source_production_durable_memory_write_preflight_payload_redaction_hash_sha256",
        "source_production_durable_memory_write_preflight_wal_receipt_plan_hash_sha256",
        "source_production_durable_memory_write_preflight_readback_plan_hash_sha256",
        "source_production_durable_memory_write_preflight_rollback_tombstone_zero_residue_plan_hash_sha256",
        "operator_packet_acceptance_envelope_hash_sha256",
        "operator_packet_acceptance_identity_session_hash_sha256",
        "operator_packet_acceptance_signature_hash_sha256",
        "operator_packet_acceptance_nonce_hash_sha256",
        "operator_packet_acceptance_command_hash_sha256",
        "operator_packet_acceptance_receipt_plan_hash_sha256",
        "operator_packet_acceptance_replay_guard_hash_sha256",
        "operator_packet_acceptance_result_hash_sha256",
        "scoped_production_durable_memory_write_operator_packet_acceptance_boundary_hash_sha256",
        "scoped_production_durable_memory_write_operator_packet_acceptance_policy_hash_sha256",
    ] {
        assert_ne!(
            value[key], "",
            "scoped production durable Memory write operator packet acceptance hash should be present: {key}"
        );
    }
    assert_eq!(
        value["required_scoped_production_durable_memory_write_operator_packet_acceptance_surface_count"],
        12
    );
    assert_eq!(
        value["ready_scoped_production_durable_memory_write_operator_packet_acceptance_surface_count"],
        12
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_operator_packet_acceptance_fixture_count"],
        10
    );
    assert_eq!(
        value["accepted_scoped_production_durable_memory_write_operator_packet_acceptance_fixture_count"],
        1
    );
    assert_eq!(
        value["blocked_scoped_production_durable_memory_write_operator_packet_acceptance_fixture_count"],
        9
    );
    assert_eq!(
        value["denied_by_scoped_production_durable_memory_write_operator_packet_acceptance_boundary_count"],
        38
    );
    for key in [
        "scoped_production_durable_memory_write_operator_packet_acceptance_performed_count",
        "scoped_production_durable_memory_write_operator_packet_acceptance_result_recorded_count",
        "scoped_production_durable_memory_write_operator_packet_acceptance_result_accepted_count",
        "source_preflight_boundary_accepted_count",
        "production_durable_memory_target_bound_count",
        "operator_packet_acceptance_envelope_bound_count",
        "operator_identity_session_acceptance_bound_count",
        "operator_packet_signature_acceptance_bound_count",
        "single_use_acceptance_nonce_bound_count",
        "explicit_acceptance_command_bound_count",
        "payload_redaction_acceptance_bound_count",
        "acceptance_receipt_plan_bound_count",
        "replay_idempotency_acceptance_bound_count",
    ] {
        assert_eq!(
            value[key], 1,
            "operator packet acceptance count should be one: {key}"
        );
    }
    for key in [
        "operator_packet_persisted_count",
        "operator_packet_ledger_recorded_count",
        "operator_packet_filesystem_written_count",
        "operator_packet_acceptance_receipt_persisted_count",
        "production_durable_memory_write_executed_count",
        "production_durable_memory_store_write_performed_count",
        "actual_production_durable_memory_write_performed_count",
        "durable_memory_store_write_performed_count",
        "durable_memory_store_read_performed_count",
        "durable_memory_store_rollback_performed_count",
        "memory_store_write_performed_count",
        "wal_write_performed_count",
        "receipt_persisted_count",
        "post_write_readback_performed_count",
        "rollback_performed_count",
        "tombstone_cleanup_executed_count",
        "live_kg_write_performed_count",
        "provider_invoked_count",
        "model_invoked_count",
        "credential_read_count",
        "channel_send_performed_count",
        "external_send_performed_count",
        "release_artifact_written_count",
        "install_executed_count",
        "service_restarted_count",
        "active_binary_mutated_count",
    ] {
        assert_eq!(
            value[key], 0,
            "operator packet acceptance side-effect count should stay zero: {key}"
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
        "kg_live_write_forbidden",
        "provider_model_invocation_forbidden",
        "credential_channel_public_release_forbidden",
        "install_restart_active_binary_mutation_forbidden",
    ] {
        assert_eq!(
            value[key], true,
            "operator packet acceptance field should be true: {key}"
        );
    }
    for key in [
        "operator_packet_persisted",
        "operator_packet_ledger_recorded",
        "operator_packet_filesystem_written",
        "operator_packet_acceptance_receipt_persisted",
        "production_durable_memory_write_executed",
        "production_durable_memory_store_write_performed",
        "actual_production_durable_memory_write_performed",
        "durable_memory_store_write_performed",
        "durable_memory_store_read_performed",
        "durable_memory_store_rollback_performed",
        "memory_write_execution_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "wal_write_performed",
        "receipt_persisted",
        "post_write_readback_performed",
        "rollback_executed",
        "rollback_performed",
        "tombstone_write_performed",
        "tombstone_cleanup_executed",
        "raw_payload_plaintext_recorded",
        "raw_payload_plaintext_persisted",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "channel_send_performed",
        "external_send_performed",
        "release_artifact_written",
        "install_executed",
        "service_restarted",
        "active_binary_mutated",
    ] {
        assert_eq!(
            value[key], false,
            "operator packet acceptance external, persistence, or mutation field should stay false: {key}"
        );
    }
    let fixtures =
        value["scoped_production_durable_memory_write_operator_packet_acceptance_fixtures"]
            .as_array()
            .expect("scoped production durable Memory write operator packet acceptance fixtures");
    assert_eq!(fixtures.len(), 10);
    assert_eq!(
            fixtures
                .iter()
                .filter(|fixture| {
                    fixture["scoped_production_durable_memory_write_operator_packet_acceptance_accepted"]
                        == true
                })
                .count(),
            1
        );
    let denied = value
            ["denied_by_scoped_production_durable_memory_write_operator_packet_acceptance_boundary"]
            .as_array()
            .expect("scoped production durable Memory write operator packet acceptance denials");
    assert_eq!(denied.len(), 38);
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "run_scoped_production_durable_memory_write_operator_packet_acceptance_boundary_require_live_gate"
    );
    assert_eq!(
        value["allowed_next_actions"][0]["persists_operator_packet"],
        false
    );
    assert_eq!(
        value["allowed_next_actions"][0]["writes_production_durable_memory"],
        false
    );
    assert_eq!(
        value["allowed_next_actions"][1]["action"],
        "prepare_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_boundary"
    );
    assert_eq!(
        value["allowed_next_actions"][1]["requires_scoped_production_durable_memory_write_operator_packet_acceptance_boundary"],
        true
    );
    let side_effects = value["side_effects"]
        .as_object()
        .expect("scoped production durable Memory write operator packet acceptance side effects");
    assert_eq!(
        side_effects["scoped_production_durable_memory_write_operator_packet_acceptance_performed"]
            .as_bool(),
        Some(true)
    );
    assert_eq!(
            side_effects["scoped_production_durable_memory_write_operator_packet_acceptance_result_accepted"]
                .as_bool(),
            Some(true)
        );
    assert_eq!(
        side_effects["operator_packet_persisted"].as_bool(),
        Some(false)
    );
    assert_eq!(
        side_effects["production_durable_memory_store_write_performed"].as_bool(),
        Some(false)
    );
    assert_eq!(
        side_effects["memory_store_write_performed"].as_bool(),
        Some(false)
    );
    assert_eq!(side_effects["wal_write_performed"].as_bool(), Some(false));
    assert_eq!(side_effects["receipt_persisted"].as_bool(), Some(false));
    assert_eq!(
        side_effects["external_send_performed"].as_bool(),
        Some(false)
    );
}

#[test]
fn hepta_memory_write_execution_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_binds_receipt_without_persistence_or_production_side_effects()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_PRODUCTION_DURABLE_MEMORY_WRITE_OPERATOR_PACKET_ACCEPTANCE_RECEIPT_BOUNDARY_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect(
        "scoped production durable Memory write operator packet acceptance receipt boundary json",
    );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_PRODUCTION_DURABLE_MEMORY_WRITE_OPERATOR_PACKET_ACCEPTANCE_RECEIPT_BOUNDARY_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-operator-packet-acceptance-receipt-boundary --json"
    );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(value["route_count"], NATIVE_GATEWAY_SOURCE_COMMAND_COUNT);
    assert_eq!(
        value["implemented_route_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(
        value["memory_write_execution_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_boundary_ready"],
        true
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_operator_packet_acceptance_receipt_ready"],
        true
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_operator_packet_acceptance_receipt_performed"],
        true
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_operator_packet_acceptance_receipt_accepted"],
        true
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_operator_packet_acceptance_receipt_mode"],
        "acceptance_receipt_boundary_no_receipt_persistence_no_production_durable_memory_mutation"
    );
    assert_eq!(
        value["source_scoped_production_durable_memory_write_operator_packet_acceptance_boundary_ready"],
        true
    );
    assert_eq!(
        value["source_scoped_production_durable_memory_write_operator_packet_acceptance_accepted_count"],
        1
    );
    assert_eq!(
        value["source_accepted_scoped_production_durable_memory_write_operator_packet_acceptance_fixture_count"],
        1
    );
    assert_eq!(
        value["source_blocked_scoped_production_durable_memory_write_operator_packet_acceptance_fixture_count"],
        9
    );
    assert_eq!(
        value["approved_production_namespace"],
        "hepta.memory.production.scoped"
    );
    assert_eq!(
        value["approved_production_store"],
        "hepta-memory-durable-store-production-preflight-only"
    );
    assert_eq!(
        value["approved_production_scope"],
        "operator-approved-session"
    );
    assert_eq!(
        value["production_durable_memory_target_id"],
        "hepta-scoped-production-durable-memory-write-target-v1"
    );
    assert_eq!(
        value["production_durable_memory_payload_class"],
        "redacted-minimal-operator-approved-memory-fact"
    );
    for key in [
        "source_operator_packet_acceptance_result_hash_sha256",
        "source_scoped_production_durable_memory_write_operator_packet_acceptance_boundary_hash_sha256",
        "source_scoped_production_durable_memory_write_operator_packet_acceptance_policy_hash_sha256",
        "source_production_durable_memory_write_preflight_target_hash_sha256",
        "source_production_durable_memory_write_preflight_operator_packet_hash_sha256",
        "source_operator_packet_acceptance_envelope_hash_sha256",
        "source_operator_packet_acceptance_identity_session_hash_sha256",
        "source_operator_packet_acceptance_signature_hash_sha256",
        "source_operator_packet_acceptance_nonce_hash_sha256",
        "source_operator_packet_acceptance_command_hash_sha256",
        "source_operator_packet_acceptance_receipt_plan_hash_sha256",
        "source_operator_packet_acceptance_replay_guard_hash_sha256",
        "acceptance_receipt_envelope_hash_sha256",
        "acceptance_receipt_identity_session_hash_sha256",
        "acceptance_receipt_digest_hash_sha256",
        "acceptance_receipt_hash_chain_hash_sha256",
        "acceptance_receipt_readback_plan_hash_sha256",
        "acceptance_receipt_replay_guard_hash_sha256",
        "acceptance_receipt_result_hash_sha256",
        "scoped_production_durable_memory_write_operator_packet_acceptance_receipt_boundary_hash_sha256",
        "scoped_production_durable_memory_write_operator_packet_acceptance_receipt_policy_hash_sha256",
    ] {
        assert_ne!(
            value[key], "",
            "scoped production durable Memory write operator packet acceptance receipt hash should be present: {key}"
        );
    }
    assert_eq!(
        value["required_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_surface_count"],
        12
    );
    assert_eq!(
        value["ready_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_surface_count"],
        12
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_operator_packet_acceptance_receipt_fixture_count"],
        10
    );
    assert_eq!(
        value["accepted_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_fixture_count"],
        1
    );
    assert_eq!(
        value["blocked_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_fixture_count"],
        9
    );
    assert_eq!(
        value["denied_by_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_boundary_count"],
        40
    );
    for key in [
        "scoped_production_durable_memory_write_operator_packet_acceptance_receipt_boundary_performed_count",
        "scoped_production_durable_memory_write_operator_packet_acceptance_receipt_result_recorded_count",
        "scoped_production_durable_memory_write_operator_packet_acceptance_receipt_result_accepted_count",
        "source_operator_packet_acceptance_boundary_accepted_count",
        "operator_packet_acceptance_result_bound_count",
        "acceptance_receipt_envelope_bound_count",
        "acceptance_receipt_identity_session_bound_count",
        "acceptance_receipt_digest_bound_count",
        "acceptance_receipt_hash_chain_bound_count",
        "acceptance_receipt_readback_plan_bound_count",
        "acceptance_receipt_replay_guard_bound_count",
    ] {
        assert_eq!(
            value[key], 1,
            "operator packet acceptance receipt count should be one: {key}"
        );
    }
    for key in [
        "acceptance_receipt_persisted_count",
        "acceptance_receipt_filesystem_written_count",
        "acceptance_receipt_ledger_recorded_count",
        "acceptance_receipt_delivered_count",
        "operator_packet_persisted_count",
        "operator_packet_ledger_recorded_count",
        "operator_packet_filesystem_written_count",
        "operator_packet_acceptance_receipt_persisted_count",
        "production_durable_memory_write_executed_count",
        "production_durable_memory_store_write_performed_count",
        "actual_production_durable_memory_write_performed_count",
        "durable_memory_store_write_performed_count",
        "durable_memory_store_read_performed_count",
        "durable_memory_store_rollback_performed_count",
        "memory_store_write_performed_count",
        "wal_write_performed_count",
        "receipt_persisted_count",
        "post_write_readback_performed_count",
        "rollback_performed_count",
        "tombstone_cleanup_executed_count",
        "live_kg_write_performed_count",
        "provider_invoked_count",
        "model_invoked_count",
        "credential_read_count",
        "channel_send_performed_count",
        "external_send_performed_count",
        "release_artifact_written_count",
        "install_executed_count",
        "service_restarted_count",
        "active_binary_mutated_count",
    ] {
        assert_eq!(
            value[key], 0,
            "operator packet acceptance receipt side-effect count should stay zero: {key}"
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
        "kg_live_write_forbidden",
        "provider_model_invocation_forbidden",
        "credential_channel_public_release_forbidden",
        "install_restart_active_binary_mutation_forbidden",
    ] {
        assert_eq!(
            value[key], true,
            "operator packet acceptance receipt field should be true: {key}"
        );
    }
    for key in [
        "acceptance_receipt_persisted",
        "acceptance_receipt_filesystem_written",
        "acceptance_receipt_ledger_recorded",
        "acceptance_receipt_delivered",
        "operator_packet_persisted",
        "operator_packet_ledger_recorded",
        "operator_packet_filesystem_written",
        "operator_packet_acceptance_receipt_persisted",
        "production_durable_memory_write_executed",
        "production_durable_memory_store_write_performed",
        "actual_production_durable_memory_write_performed",
        "durable_memory_store_write_performed",
        "durable_memory_store_read_performed",
        "durable_memory_store_rollback_performed",
        "memory_write_execution_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "wal_write_performed",
        "receipt_persisted",
        "post_write_readback_performed",
        "rollback_executed",
        "rollback_performed",
        "tombstone_write_performed",
        "tombstone_cleanup_executed",
        "raw_payload_plaintext_recorded",
        "raw_payload_plaintext_persisted",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "channel_send_performed",
        "external_send_performed",
        "release_artifact_written",
        "install_executed",
        "service_restarted",
        "active_binary_mutated",
    ] {
        assert_eq!(
            value[key], false,
            "operator packet acceptance receipt persistence, mutation, or external field should stay false: {key}"
        );
    }
    let fixtures = value
            ["scoped_production_durable_memory_write_operator_packet_acceptance_receipt_fixtures"]
            .as_array()
            .expect(
                "scoped production durable Memory write operator packet acceptance receipt fixtures",
            );
    assert_eq!(fixtures.len(), 10);
    assert_eq!(
            fixtures
                .iter()
                .filter(|fixture| {
                    fixture["scoped_production_durable_memory_write_operator_packet_acceptance_receipt_accepted"]
                        == true
                })
                .count(),
            1
        );
    let denied = value
            ["denied_by_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_boundary"]
            .as_array()
            .expect("scoped production durable Memory write operator packet acceptance receipt denials");
    assert_eq!(denied.len(), 40);
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "run_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_boundary_require_live_gate"
    );
    assert_eq!(
        value["allowed_next_actions"][0]["persists_acceptance_receipt"],
        false
    );
    assert_eq!(
        value["allowed_next_actions"][0]["writes_production_durable_memory"],
        false
    );
    assert_eq!(
        value["allowed_next_actions"][1]["action"],
        "prepare_scoped_production_durable_memory_write_dry_run_execution_envelope_boundary"
    );
    assert_eq!(
        value["allowed_next_actions"][1]["requires_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_boundary"],
        true
    );
    let side_effects = value["side_effects"].as_object().expect(
        "scoped production durable Memory write operator packet acceptance receipt side effects",
    );
    assert_eq!(
            side_effects["scoped_production_durable_memory_write_operator_packet_acceptance_receipt_boundary_performed"].as_bool(),
            Some(true)
        );
    assert_eq!(
            side_effects["scoped_production_durable_memory_write_operator_packet_acceptance_receipt_result_accepted"].as_bool(),
            Some(true)
        );
    assert_eq!(
        side_effects["acceptance_receipt_persisted"].as_bool(),
        Some(false)
    );
    assert_eq!(
        side_effects["operator_packet_persisted"].as_bool(),
        Some(false)
    );
    assert_eq!(
        side_effects["production_durable_memory_store_write_performed"].as_bool(),
        Some(false)
    );
    assert_eq!(
        side_effects["memory_store_write_performed"].as_bool(),
        Some(false)
    );
    assert_eq!(side_effects["wal_write_performed"].as_bool(), Some(false));
    assert_eq!(side_effects["receipt_persisted"].as_bool(), Some(false));
    assert_eq!(
        side_effects["external_send_performed"].as_bool(),
        Some(false)
    );
}

#[test]
fn hepta_memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_envelope_binds_envelope_without_execution_persistence_or_production_side_effects()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_PRODUCTION_DURABLE_MEMORY_WRITE_DRY_RUN_EXECUTION_ENVELOPE_BOUNDARY_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body)
        .expect("scoped production durable Memory write dry-run execution envelope json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_PRODUCTION_DURABLE_MEMORY_WRITE_DRY_RUN_EXECUTION_ENVELOPE_BOUNDARY_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-envelope-boundary --json"
    );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(value["route_count"], NATIVE_GATEWAY_SOURCE_COMMAND_COUNT);
    assert_eq!(
        value["implemented_route_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(
        value["memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_envelope_boundary_ready"],
        true
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_dry_run_execution_envelope_ready"],
        true
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_dry_run_execution_envelope_accepted"],
        true
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_dry_run_execution_envelope_mode"],
        "dry_run_execution_envelope_boundary_no_execution_no_persistence_no_production_durable_memory_mutation"
    );
    assert_eq!(
        value["source_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_boundary_ready"],
        true
    );
    assert_eq!(
        value["source_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_accepted_count"],
        1
    );
    assert_eq!(
        value["source_accepted_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_fixture_count"],
        1
    );
    assert_eq!(
        value["source_blocked_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_fixture_count"],
        9
    );
    assert_eq!(
        value["approved_production_namespace"],
        "hepta.memory.production.scoped"
    );
    assert_eq!(
        value["approved_production_store"],
        "hepta-memory-durable-store-production-preflight-only"
    );
    assert_eq!(
        value["approved_production_scope"],
        "operator-approved-session"
    );
    assert_eq!(
        value["production_durable_memory_target_id"],
        "hepta-scoped-production-durable-memory-write-target-v1"
    );
    for key in [
        "source_acceptance_receipt_result_hash_sha256",
        "source_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_boundary_hash_sha256",
        "source_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_policy_hash_sha256",
        "source_acceptance_receipt_envelope_hash_sha256",
        "source_acceptance_receipt_digest_hash_sha256",
        "source_acceptance_receipt_hash_chain_hash_sha256",
        "source_acceptance_receipt_readback_plan_hash_sha256",
        "source_acceptance_receipt_replay_guard_hash_sha256",
        "dry_run_execution_envelope_hash_sha256",
        "dry_run_execution_identity_session_hash_sha256",
        "dry_run_execution_target_snapshot_hash_sha256",
        "dry_run_execution_write_plan_hash_sha256",
        "dry_run_execution_payload_redaction_hash_sha256",
        "dry_run_execution_wal_receipt_preview_hash_sha256",
        "dry_run_execution_readback_preview_hash_sha256",
        "dry_run_execution_rollback_tombstone_preview_hash_sha256",
        "dry_run_execution_replay_guard_hash_sha256",
        "dry_run_execution_result_hash_sha256",
        "scoped_production_durable_memory_write_dry_run_execution_envelope_boundary_hash_sha256",
        "scoped_production_durable_memory_write_dry_run_execution_envelope_policy_hash_sha256",
    ] {
        assert_ne!(
            value[key], "",
            "dry-run execution envelope hash should be present: {key}"
        );
    }
    assert_eq!(
        value["required_scoped_production_durable_memory_write_dry_run_execution_envelope_surface_count"],
        15
    );
    assert_eq!(
        value["ready_scoped_production_durable_memory_write_dry_run_execution_envelope_surface_count"],
        15
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_dry_run_execution_envelope_fixture_count"],
        10
    );
    assert_eq!(
        value["accepted_scoped_production_durable_memory_write_dry_run_execution_envelope_fixture_count"],
        1
    );
    assert_eq!(
        value["blocked_scoped_production_durable_memory_write_dry_run_execution_envelope_fixture_count"],
        9
    );
    assert_eq!(
        value["denied_by_scoped_production_durable_memory_write_dry_run_execution_envelope_boundary_count"],
        44
    );
    for key in [
        "scoped_production_durable_memory_write_dry_run_execution_envelope_boundary_performed_count",
        "scoped_production_durable_memory_write_dry_run_execution_envelope_result_recorded_count",
        "scoped_production_durable_memory_write_dry_run_execution_envelope_result_accepted_count",
        "source_operator_packet_acceptance_receipt_boundary_accepted_count",
        "dry_run_execution_envelope_bound_count",
        "dry_run_execution_identity_session_bound_count",
        "dry_run_execution_target_snapshot_bound_count",
        "dry_run_execution_write_plan_bound_count",
        "dry_run_execution_payload_redaction_bound_count",
        "dry_run_execution_wal_receipt_preview_bound_count",
        "dry_run_execution_readback_preview_bound_count",
        "dry_run_execution_rollback_tombstone_preview_bound_count",
        "dry_run_execution_replay_guard_bound_count",
    ] {
        assert_eq!(value[key], 1, "dry-run count should be one: {key}");
    }
    for key in [
        "dry_run_execution_envelope_persisted_count",
        "dry_run_execution_envelope_filesystem_written_count",
        "dry_run_execution_envelope_ledger_recorded_count",
        "dry_run_execution_envelope_delivered_count",
        "dry_run_execution_executed_count",
        "dry_run_execution_result_persisted_count",
        "acceptance_receipt_persisted_count",
        "operator_packet_persisted_count",
        "production_durable_memory_write_executed_count",
        "production_durable_memory_store_write_performed_count",
        "actual_production_durable_memory_write_performed_count",
        "durable_memory_store_write_performed_count",
        "durable_memory_store_read_performed_count",
        "durable_memory_store_rollback_performed_count",
        "memory_store_write_performed_count",
        "wal_write_performed_count",
        "receipt_persisted_count",
        "post_write_readback_performed_count",
        "rollback_performed_count",
        "tombstone_cleanup_executed_count",
        "live_kg_write_performed_count",
        "provider_invoked_count",
        "model_invoked_count",
        "credential_read_count",
        "channel_send_performed_count",
        "external_send_performed_count",
        "release_artifact_written_count",
        "install_executed_count",
        "service_restarted_count",
        "active_binary_mutated_count",
    ] {
        assert_eq!(
            value[key], 0,
            "dry-run execution envelope side-effect count should stay zero: {key}"
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
        "production_write_execution_forbidden_on_dry_run_envelope_route",
        "production_durable_memory_write_forbidden",
        "memory_store_mutation_forbidden",
        "kg_live_write_forbidden",
        "provider_model_invocation_forbidden",
        "credential_channel_public_release_forbidden",
        "install_restart_active_binary_mutation_forbidden",
    ] {
        assert_eq!(value[key], true, "dry-run field should be true: {key}");
    }
    for key in [
        "dry_run_execution_envelope_persisted",
        "dry_run_execution_envelope_filesystem_written",
        "dry_run_execution_envelope_ledger_recorded",
        "dry_run_execution_envelope_delivered",
        "dry_run_execution_executed",
        "dry_run_execution_result_persisted",
        "acceptance_receipt_persisted",
        "operator_packet_persisted",
        "operator_packet_acceptance_receipt_persisted",
        "production_durable_memory_write_executed",
        "production_durable_memory_store_write_performed",
        "actual_production_durable_memory_write_performed",
        "durable_memory_store_write_performed",
        "durable_memory_store_read_performed",
        "durable_memory_store_rollback_performed",
        "memory_write_execution_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "wal_write_performed",
        "receipt_persisted",
        "post_write_readback_performed",
        "rollback_executed",
        "rollback_performed",
        "tombstone_write_performed",
        "tombstone_cleanup_executed",
        "raw_payload_plaintext_recorded",
        "raw_payload_plaintext_persisted",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "channel_send_performed",
        "external_send_performed",
        "release_artifact_written",
        "install_executed",
        "service_restarted",
        "active_binary_mutated",
    ] {
        assert_eq!(
            value[key], false,
            "dry-run persistence, mutation, execution, or external field should stay false: {key}"
        );
    }
    let fixtures =
        value["scoped_production_durable_memory_write_dry_run_execution_envelope_fixtures"]
            .as_array()
            .expect("scoped production durable Memory write dry-run execution envelope fixtures");
    assert_eq!(fixtures.len(), 10);
    assert_eq!(
            fixtures
                .iter()
                .filter(|fixture| {
                    fixture["scoped_production_durable_memory_write_dry_run_execution_envelope_accepted"]
                        == true
                })
                .count(),
            1
        );
    let denied = value
            ["denied_by_scoped_production_durable_memory_write_dry_run_execution_envelope_boundary"]
            .as_array()
            .expect("scoped production durable Memory write dry-run execution envelope denials");
    assert_eq!(denied.len(), 44);
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "run_scoped_production_durable_memory_write_dry_run_execution_envelope_boundary_require_live_gate"
    );
    assert_eq!(value["allowed_next_actions"][0]["executes_dry_run"], false);
    assert_eq!(
        value["allowed_next_actions"][0]["writes_production_durable_memory"],
        false
    );
    assert_eq!(
        value["allowed_next_actions"][1]["action"],
        "prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_boundary"
    );
    assert_eq!(
        value["allowed_next_actions"][1]["requires_scoped_production_durable_memory_write_dry_run_execution_envelope_boundary"],
        true
    );
    let side_effects = value["side_effects"]
        .as_object()
        .expect("scoped production durable Memory write dry-run execution envelope side effects");
    assert_eq!(
            side_effects["scoped_production_durable_memory_write_dry_run_execution_envelope_boundary_performed"].as_bool(),
            Some(true)
        );
    assert_eq!(
            side_effects["scoped_production_durable_memory_write_dry_run_execution_envelope_result_accepted"].as_bool(),
            Some(true)
        );
    assert_eq!(
        side_effects["dry_run_execution_executed"].as_bool(),
        Some(false)
    );
    assert_eq!(
        side_effects["production_durable_memory_store_write_performed"].as_bool(),
        Some(false)
    );
    assert_eq!(
        side_effects["memory_store_write_performed"].as_bool(),
        Some(false)
    );
    assert_eq!(side_effects["wal_write_performed"].as_bool(), Some(false));
    assert_eq!(side_effects["receipt_persisted"].as_bool(), Some(false));
    assert_eq!(
        side_effects["external_send_performed"].as_bool(),
        Some(false)
    );
}

#[test]
fn hepta_memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_binds_receipt_without_execution_persistence_or_production_side_effects()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_PRODUCTION_DURABLE_MEMORY_WRITE_DRY_RUN_EXECUTION_RESULT_RECEIPT_BOUNDARY_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body)
        .expect("scoped production durable Memory write dry-run execution result receipt json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_PRODUCTION_DURABLE_MEMORY_WRITE_DRY_RUN_EXECUTION_RESULT_RECEIPT_BOUNDARY_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-boundary --json"
    );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(value["route_count"], NATIVE_GATEWAY_SOURCE_COMMAND_COUNT);
    assert_eq!(
        value["implemented_route_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(
        value["memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_boundary_ready"],
        true
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_dry_run_execution_result_receipt_ready"],
        true
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_dry_run_execution_result_receipt_accepted"],
        true
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_dry_run_execution_result_receipt_mode"],
        "dry_run_execution_result_receipt_boundary_no_execution_no_receipt_persistence_no_production_durable_memory_mutation"
    );
    assert_eq!(
        value["source_scoped_production_durable_memory_write_dry_run_execution_envelope_boundary_ready"],
        true
    );
    assert_eq!(
        value["source_scoped_production_durable_memory_write_dry_run_execution_envelope_accepted_count"],
        1
    );
    assert_eq!(
        value["source_accepted_scoped_production_durable_memory_write_dry_run_execution_envelope_fixture_count"],
        1
    );
    assert_eq!(
        value["source_blocked_scoped_production_durable_memory_write_dry_run_execution_envelope_fixture_count"],
        9
    );
    assert_eq!(
        value["approved_production_namespace"],
        "hepta.memory.production.scoped"
    );
    assert_eq!(
        value["approved_production_store"],
        "hepta-memory-durable-store-production-preflight-only"
    );
    assert_eq!(
        value["approved_production_scope"],
        "operator-approved-session"
    );
    assert_eq!(
        value["production_durable_memory_target_id"],
        "hepta-scoped-production-durable-memory-write-target-v1"
    );
    for key in [
        "source_dry_run_execution_result_hash_sha256",
        "source_scoped_production_durable_memory_write_dry_run_execution_envelope_boundary_hash_sha256",
        "source_scoped_production_durable_memory_write_dry_run_execution_envelope_policy_hash_sha256",
        "source_dry_run_execution_envelope_hash_sha256",
        "source_dry_run_execution_target_snapshot_hash_sha256",
        "source_dry_run_execution_write_plan_hash_sha256",
        "source_dry_run_execution_payload_redaction_hash_sha256",
        "source_dry_run_execution_wal_receipt_preview_hash_sha256",
        "source_dry_run_execution_readback_preview_hash_sha256",
        "source_dry_run_execution_rollback_tombstone_preview_hash_sha256",
        "source_dry_run_execution_replay_guard_hash_sha256",
        "dry_run_execution_result_receipt_envelope_hash_sha256",
        "dry_run_execution_result_receipt_identity_session_hash_sha256",
        "dry_run_execution_result_receipt_digest_hash_sha256",
        "dry_run_execution_result_receipt_hash_chain_hash_sha256",
        "dry_run_execution_result_receipt_readback_plan_hash_sha256",
        "dry_run_execution_result_receipt_replay_guard_hash_sha256",
        "dry_run_execution_result_receipt_handoff_hash_sha256",
        "dry_run_execution_result_receipt_result_hash_sha256",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_boundary_hash_sha256",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_policy_hash_sha256",
    ] {
        assert_ne!(
            value[key], "",
            "dry-run execution result receipt hash should be present: {key}"
        );
    }
    assert_eq!(
        value["required_scoped_production_durable_memory_write_dry_run_execution_result_receipt_surface_count"],
        14
    );
    assert_eq!(
        value["ready_scoped_production_durable_memory_write_dry_run_execution_result_receipt_surface_count"],
        14
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_dry_run_execution_result_receipt_fixture_count"],
        10
    );
    assert_eq!(
        value["accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_fixture_count"],
        1
    );
    assert_eq!(
        value["blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_fixture_count"],
        9
    );
    assert_eq!(
        value["denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_boundary_count"],
        47
    );
    for key in [
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_boundary_performed_count",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_result_recorded_count",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_result_accepted_count",
        "source_dry_run_execution_envelope_boundary_accepted_count",
        "dry_run_execution_result_bound_count",
        "dry_run_execution_result_receipt_envelope_bound_count",
        "dry_run_execution_result_receipt_digest_bound_count",
        "dry_run_execution_result_receipt_hash_chain_bound_count",
        "dry_run_execution_result_receipt_readback_plan_bound_count",
        "dry_run_execution_result_receipt_replay_guard_bound_count",
    ] {
        assert_eq!(value[key], 1, "result receipt count should be one: {key}");
    }
    for key in [
        "dry_run_execution_result_receipt_persisted_count",
        "dry_run_execution_result_receipt_filesystem_written_count",
        "dry_run_execution_result_receipt_ledger_recorded_count",
        "dry_run_execution_result_receipt_delivered_count",
        "dry_run_execution_result_receipt_materialized_count",
        "dry_run_execution_envelope_persisted_count",
        "dry_run_execution_executed_count",
        "dry_run_execution_result_persisted_count",
        "acceptance_receipt_persisted_count",
        "operator_packet_persisted_count",
        "production_durable_memory_write_executed_count",
        "production_durable_memory_store_write_performed_count",
        "actual_production_durable_memory_write_performed_count",
        "durable_memory_store_write_performed_count",
        "durable_memory_store_read_performed_count",
        "durable_memory_store_rollback_performed_count",
        "memory_store_write_performed_count",
        "wal_write_performed_count",
        "receipt_persisted_count",
        "post_write_readback_performed_count",
        "rollback_performed_count",
        "tombstone_cleanup_executed_count",
        "live_kg_write_performed_count",
        "provider_invoked_count",
        "model_invoked_count",
        "credential_read_count",
        "channel_send_performed_count",
        "external_send_performed_count",
        "release_artifact_written_count",
        "install_executed_count",
        "service_restarted_count",
        "active_binary_mutated_count",
    ] {
        assert_eq!(
            value[key], 0,
            "dry-run result receipt side-effect count should stay zero: {key}"
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
        "dry_run_execution_write_plan_bound",
        "dry_run_execution_readback_preview_bound",
        "dry_run_execution_rollback_tombstone_preview_bound",
        "dry_run_execution_replay_guard_bound",
        "dry_run_execution_result_receipt_envelope_bound",
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
        "kg_live_write_forbidden",
        "provider_model_invocation_forbidden",
        "credential_channel_public_release_forbidden",
        "install_restart_active_binary_mutation_forbidden",
    ] {
        assert_eq!(
            value[key], true,
            "result receipt field should be true: {key}"
        );
    }
    for key in [
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
        "production_durable_memory_store_write_performed",
        "actual_production_durable_memory_write_performed",
        "durable_memory_store_write_performed",
        "durable_memory_store_read_performed",
        "durable_memory_store_rollback_performed",
        "memory_write_execution_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "wal_write_performed",
        "receipt_persisted",
        "post_write_readback_performed",
        "rollback_executed",
        "rollback_performed",
        "tombstone_write_performed",
        "tombstone_cleanup_executed",
        "raw_payload_plaintext_recorded",
        "raw_payload_plaintext_persisted",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "channel_send_performed",
        "external_send_performed",
        "release_artifact_written",
        "install_executed",
        "service_restarted",
        "active_binary_mutated",
    ] {
        assert_eq!(
            value[key], false,
            "dry-run result receipt persistence, mutation, execution, or external field should stay false: {key}"
        );
    }
    let fixtures =
        value["scoped_production_durable_memory_write_dry_run_execution_result_receipt_fixtures"]
            .as_array()
            .expect(
                "scoped production durable Memory write dry-run execution result receipt fixtures",
            );
    assert_eq!(fixtures.len(), 10);
    assert_eq!(
            fixtures
                .iter()
                .filter(|fixture| {
                    fixture["scoped_production_durable_memory_write_dry_run_execution_result_receipt_accepted"]
                        == true
                })
                .count(),
            1
        );
    let denied = value
            ["denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_boundary"]
            .as_array()
            .expect("scoped production durable Memory write dry-run execution result receipt denials");
    assert_eq!(denied.len(), 47);
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "run_scoped_production_durable_memory_write_dry_run_execution_result_receipt_boundary_require_live_gate"
    );
    assert_eq!(value["allowed_next_actions"][0]["executes_dry_run"], false);
    assert_eq!(
        value["allowed_next_actions"][0]["writes_production_durable_memory"],
        false
    );
    assert_eq!(
        value["allowed_next_actions"][1]["action"],
        "prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary"
    );
    assert_eq!(
        value["allowed_next_actions"][1]["requires_scoped_production_durable_memory_write_dry_run_execution_result_receipt_boundary"],
        true
    );
    let side_effects = value["side_effects"].as_object().expect(
        "scoped production durable Memory write dry-run execution result receipt side effects",
    );
    assert_eq!(
            side_effects["scoped_production_durable_memory_write_dry_run_execution_result_receipt_boundary_performed"].as_bool(),
            Some(true)
        );
    assert_eq!(
            side_effects["scoped_production_durable_memory_write_dry_run_execution_result_receipt_result_accepted"].as_bool(),
            Some(true)
        );
    assert_eq!(
        side_effects["dry_run_execution_result_receipt_persisted"].as_bool(),
        Some(false)
    );
    assert_eq!(
        side_effects["dry_run_execution_executed"].as_bool(),
        Some(false)
    );
    assert_eq!(
        side_effects["production_durable_memory_store_write_performed"].as_bool(),
        Some(false)
    );
    assert_eq!(
        side_effects["memory_store_write_performed"].as_bool(),
        Some(false)
    );
    assert_eq!(side_effects["wal_write_performed"].as_bool(), Some(false));
    assert_eq!(side_effects["receipt_persisted"].as_bool(), Some(false));
    assert_eq!(
        side_effects["external_send_performed"].as_bool(),
        Some(false)
    );
}

#[test]
fn hepta_memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_blocks_replay_without_state_persistence_execution_or_production_side_effects()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_PRODUCTION_DURABLE_MEMORY_WRITE_DRY_RUN_EXECUTION_RESULT_RECEIPT_REPLAY_IDEMPOTENCY_DENIAL_BOUNDARY_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "scoped production durable Memory write dry-run execution result receipt replay/idempotency denial json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_PRODUCTION_DURABLE_MEMORY_WRITE_DRY_RUN_EXECUTION_RESULT_RECEIPT_REPLAY_IDEMPOTENCY_DENIAL_BOUNDARY_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-replay-idempotency-denial-boundary --json"
    );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(value["route_count"], NATIVE_GATEWAY_SOURCE_COMMAND_COUNT);
    assert_eq!(
        value["implemented_route_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(
        value["memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary_ready"],
        true
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_ready"],
        true
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_accepted"],
        true
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_mode"],
        "dry_run_execution_result_receipt_replay_idempotency_denial_boundary_no_replay_state_persistence_no_execution_no_production_durable_memory_mutation"
    );
    assert_eq!(
        value["source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_boundary_ready"],
        true
    );
    assert_eq!(
        value["source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_boundary_accepted_count"],
        1
    );
    assert_eq!(
        value["source_accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_fixture_count"],
        1
    );
    assert_eq!(
        value["source_blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_fixture_count"],
        9
    );
    assert_eq!(
        value["approved_production_namespace"],
        "hepta.memory.production.scoped"
    );
    assert_eq!(
        value["approved_production_store"],
        "hepta-memory-durable-store-production-preflight-only"
    );
    assert_eq!(
        value["approved_production_scope"],
        "operator-approved-session"
    );
    assert_eq!(
        value["production_durable_memory_target_id"],
        "hepta-scoped-production-durable-memory-write-target-v1"
    );
    for key in [
        "source_dry_run_execution_result_hash_sha256",
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_boundary_hash_sha256",
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_policy_hash_sha256",
        "source_dry_run_execution_result_receipt_envelope_hash_sha256",
        "source_dry_run_execution_result_receipt_identity_session_hash_sha256",
        "source_dry_run_execution_result_receipt_digest_hash_sha256",
        "source_dry_run_execution_result_receipt_hash_chain_hash_sha256",
        "source_dry_run_execution_result_receipt_readback_plan_hash_sha256",
        "source_dry_run_execution_result_receipt_replay_guard_hash_sha256",
        "source_dry_run_execution_result_receipt_handoff_hash_sha256",
        "source_dry_run_execution_result_receipt_result_hash_sha256",
        "dry_run_execution_result_receipt_replay_idempotency_denial_matrix_hash_sha256",
        "dry_run_execution_result_receipt_replay_idempotency_identity_session_hash_sha256",
        "dry_run_execution_result_receipt_replay_idempotency_nonce_scope_hash_sha256",
        "dry_run_execution_result_receipt_replay_idempotency_duplicate_receipt_denial_hash_sha256",
        "dry_run_execution_result_receipt_replay_idempotency_stale_receipt_denial_hash_sha256",
        "dry_run_execution_result_receipt_replay_idempotency_hash_chain_mismatch_denial_hash_sha256",
        "dry_run_execution_result_receipt_replay_idempotency_cross_session_denial_hash_sha256",
        "dry_run_execution_result_receipt_replay_idempotency_handoff_hash_sha256",
        "dry_run_execution_result_receipt_replay_idempotency_result_hash_sha256",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary_hash_sha256",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_policy_hash_sha256",
    ] {
        assert_ne!(
            value[key], "",
            "dry-run execution result receipt replay/idempotency denial hash should be present: {key}"
        );
    }
    assert_eq!(
        value["required_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_surface_count"],
        16
    );
    assert_eq!(
        value["ready_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_surface_count"],
        16
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_fixture_count"],
        10
    );
    assert_eq!(
        value["accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_fixture_count"],
        1
    );
    assert_eq!(
        value["blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_fixture_count"],
        9
    );
    assert_eq!(
        value["denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary_count"],
        54
    );
    for key in [
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary_performed_count",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_result_recorded_count",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_result_accepted_count",
        "source_dry_run_execution_result_receipt_boundary_accepted_count",
        "dry_run_execution_result_receipt_replay_idempotency_denial_matrix_bound_count",
        "dry_run_execution_result_receipt_replay_idempotency_identity_session_bound_count",
        "dry_run_execution_result_receipt_replay_idempotency_nonce_scope_bound_count",
        "dry_run_execution_result_receipt_replay_idempotency_duplicate_receipt_denied_count",
        "dry_run_execution_result_receipt_replay_idempotency_stale_receipt_denied_count",
        "dry_run_execution_result_receipt_replay_idempotency_hash_chain_mismatch_denied_count",
        "dry_run_execution_result_receipt_replay_idempotency_cross_session_denied_count",
        "dry_run_execution_result_receipt_replay_idempotency_handoff_bound_count",
    ] {
        assert_eq!(
            value[key], 1,
            "replay/idempotency denial count should be one: {key}"
        );
    }
    for key in [
        "dry_run_execution_result_receipt_replay_state_persisted_count",
        "dry_run_execution_result_receipt_idempotency_ledger_written_count",
        "dry_run_execution_result_receipt_replay_guard_state_recorded_count",
        "dry_run_execution_result_receipt_duplicate_receipt_accepted_count",
        "dry_run_execution_result_receipt_stale_receipt_accepted_count",
        "dry_run_execution_result_receipt_cross_session_replay_accepted_count",
        "dry_run_execution_result_receipt_hash_chain_mismatch_accepted_count",
        "dry_run_execution_result_receipt_persisted_count",
        "dry_run_execution_executed_count",
        "dry_run_execution_result_persisted_count",
        "operator_packet_persisted_count",
        "production_durable_memory_write_executed_count",
        "production_durable_memory_store_write_performed_count",
        "actual_production_durable_memory_write_performed_count",
        "durable_memory_store_write_performed_count",
        "durable_memory_store_read_performed_count",
        "durable_memory_store_rollback_performed_count",
        "memory_store_write_performed_count",
        "wal_write_performed_count",
        "receipt_persisted_count",
        "post_write_readback_performed_count",
        "rollback_performed_count",
        "tombstone_cleanup_executed_count",
        "live_kg_write_performed_count",
        "provider_invoked_count",
        "model_invoked_count",
        "credential_read_count",
        "channel_send_performed_count",
        "external_send_performed_count",
        "release_artifact_written_count",
        "install_executed_count",
        "service_restarted_count",
        "active_binary_mutated_count",
    ] {
        assert_eq!(
            value[key], 0,
            "replay/idempotency denial side-effect count should stay zero: {key}"
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
        "kg_live_write_forbidden",
        "provider_model_invocation_forbidden",
        "credential_channel_public_release_forbidden",
        "install_restart_active_binary_mutation_forbidden",
    ] {
        assert_eq!(
            value[key], true,
            "replay/idempotency denial field should be true: {key}"
        );
    }
    for key in [
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
        "dry_run_execution_executed",
        "dry_run_execution_result_persisted",
        "acceptance_receipt_persisted",
        "operator_packet_persisted",
        "operator_packet_acceptance_receipt_persisted",
        "production_durable_memory_write_executed",
        "production_durable_memory_store_write_performed",
        "actual_production_durable_memory_write_performed",
        "durable_memory_store_write_performed",
        "durable_memory_store_read_performed",
        "durable_memory_store_rollback_performed",
        "memory_write_execution_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "wal_write_performed",
        "receipt_persisted",
        "post_write_readback_performed",
        "rollback_executed",
        "rollback_performed",
        "tombstone_write_performed",
        "tombstone_cleanup_executed",
        "raw_payload_plaintext_recorded",
        "raw_payload_plaintext_persisted",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "channel_send_performed",
        "external_send_performed",
        "release_artifact_written",
        "install_executed",
        "service_restarted",
        "active_binary_mutated",
    ] {
        assert_eq!(
            value[key], false,
            "replay/idempotency persistence, mutation, execution, or external field should stay false: {key}"
        );
    }
    let fixtures = value
            ["scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_fixtures"]
            .as_array()
            .expect(
                "scoped production durable Memory write dry-run execution result receipt replay/idempotency denial fixtures",
            );
    assert_eq!(fixtures.len(), 10);
    assert_eq!(
            fixtures
                .iter()
                .filter(|fixture| {
                    fixture["scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_accepted"]
                        == true
                })
                .count(),
            1
        );
    let denied = value
            ["denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary"]
            .as_array()
            .expect("scoped production durable Memory write dry-run execution result receipt replay/idempotency denials");
    assert_eq!(denied.len(), 54);
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "run_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary_require_live_gate"
    );
    assert_eq!(
        value["allowed_next_actions"][0]["persists_replay_state"],
        false
    );
    assert_eq!(
        value["allowed_next_actions"][0]["writes_idempotency_ledger"],
        false
    );
    assert_eq!(value["allowed_next_actions"][0]["executes_dry_run"], false);
    assert_eq!(
        value["allowed_next_actions"][0]["writes_production_durable_memory"],
        false
    );
    assert_eq!(
        value["allowed_next_actions"][1]["action"],
        "prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary"
    );
    assert_eq!(
        value["allowed_next_actions"][1]["requires_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary"],
        true
    );
    let side_effects = value["side_effects"].as_object().expect(
            "scoped production durable Memory write dry-run execution result receipt replay/idempotency side effects",
        );
    assert_eq!(
            side_effects["scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary_performed"].as_bool(),
            Some(true)
        );
    assert_eq!(
            side_effects["scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_result_accepted"].as_bool(),
            Some(true)
        );
    assert_eq!(
        side_effects["dry_run_execution_result_receipt_replay_state_persisted"].as_bool(),
        Some(false)
    );
    assert_eq!(
        side_effects["dry_run_execution_result_receipt_idempotency_ledger_written"].as_bool(),
        Some(false)
    );
    assert_eq!(
        side_effects["dry_run_execution_result_receipt_duplicate_receipt_accepted"].as_bool(),
        Some(false)
    );
    assert_eq!(
        side_effects["dry_run_execution_executed"].as_bool(),
        Some(false)
    );
    assert_eq!(
        side_effects["production_durable_memory_store_write_performed"].as_bool(),
        Some(false)
    );
    assert_eq!(
        side_effects["memory_store_write_performed"].as_bool(),
        Some(false)
    );
    assert_eq!(side_effects["wal_write_performed"].as_bool(), Some(false));
    assert_eq!(side_effects["receipt_persisted"].as_bool(), Some(false));
    assert_eq!(
        side_effects["external_send_performed"].as_bool(),
        Some(false)
    );
}

#[test]
fn hepta_memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_blocks_ordering_without_cursor_sequence_execution_or_production_side_effects()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_PRODUCTION_DURABLE_MEMORY_WRITE_DRY_RUN_EXECUTION_RESULT_RECEIPT_ORDERING_MONOTONICITY_DENIAL_BOUNDARY_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "scoped production durable Memory write dry-run execution result receipt ordering/monotonicity denial json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_PRODUCTION_DURABLE_MEMORY_WRITE_DRY_RUN_EXECUTION_RESULT_RECEIPT_ORDERING_MONOTONICITY_DENIAL_BOUNDARY_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-ordering-monotonicity-denial-boundary --json"
    );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(value["route_count"], NATIVE_GATEWAY_SOURCE_COMMAND_COUNT);
    assert_eq!(
        value["implemented_route_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(
        value["memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_ready"],
        true
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_ready"],
        true
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_accepted"],
        true
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_mode"],
        "dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_no_ordering_cursor_no_monotonic_sequence_no_execution_no_production_durable_memory_mutation"
    );
    assert_eq!(
        value["source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary_ready"],
        true
    );
    assert_eq!(
        value["source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary_accepted_count"],
        1
    );
    assert_eq!(
        value["source_accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_fixture_count"],
        1
    );
    assert_eq!(
        value["source_blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_fixture_count"],
        9
    );
    assert_eq!(
        value["approved_production_namespace"],
        "hepta.memory.production.scoped"
    );
    assert_eq!(
        value["approved_production_store"],
        "hepta-memory-durable-store-production-preflight-only"
    );
    assert_eq!(
        value["approved_production_scope"],
        "operator-approved-session"
    );
    assert_eq!(
        value["production_durable_memory_target_id"],
        "hepta-scoped-production-durable-memory-write-target-v1"
    );
    for key in [
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_boundary_hash_sha256",
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_replay_idempotency_denial_policy_hash_sha256",
        "source_dry_run_execution_result_receipt_replay_idempotency_denial_matrix_hash_sha256",
        "source_dry_run_execution_result_receipt_replay_idempotency_identity_session_hash_sha256",
        "source_dry_run_execution_result_receipt_replay_idempotency_nonce_scope_hash_sha256",
        "source_dry_run_execution_result_receipt_replay_idempotency_duplicate_receipt_denial_hash_sha256",
        "source_dry_run_execution_result_receipt_replay_idempotency_stale_receipt_denial_hash_sha256",
        "source_dry_run_execution_result_receipt_replay_idempotency_hash_chain_mismatch_denial_hash_sha256",
        "source_dry_run_execution_result_receipt_replay_idempotency_cross_session_denial_hash_sha256",
        "source_dry_run_execution_result_receipt_replay_idempotency_handoff_hash_sha256",
        "source_dry_run_execution_result_receipt_replay_idempotency_result_hash_sha256",
        "dry_run_execution_result_receipt_ordering_monotonicity_denial_matrix_hash_sha256",
        "dry_run_execution_result_receipt_ordering_sequence_policy_hash_sha256",
        "dry_run_execution_result_receipt_ordering_identity_session_hash_sha256",
        "dry_run_execution_result_receipt_ordering_latest_sequence_hash_sha256",
        "dry_run_execution_result_receipt_late_receipt_denial_hash_sha256",
        "dry_run_execution_result_receipt_future_receipt_denial_hash_sha256",
        "dry_run_execution_result_receipt_rollback_sequence_denial_hash_sha256",
        "dry_run_execution_result_receipt_same_sequence_replacement_denial_hash_sha256",
        "dry_run_execution_result_receipt_latest_wins_promotion_denial_hash_sha256",
        "dry_run_execution_result_receipt_sequence_gap_denial_hash_sha256",
        "dry_run_execution_result_receipt_ordering_handoff_hash_sha256",
        "dry_run_execution_result_receipt_ordering_result_hash_sha256",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_hash_sha256",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_policy_hash_sha256",
    ] {
        assert_ne!(
            value[key], "",
            "dry-run execution result receipt ordering/monotonicity denial hash should be present: {key}"
        );
    }
    assert_eq!(
        value["required_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_surface_count"],
        16
    );
    assert_eq!(
        value["ready_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_surface_count"],
        16
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_fixture_count"],
        10
    );
    assert_eq!(
        value["accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_fixture_count"],
        1
    );
    assert_eq!(
        value["blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_fixture_count"],
        9
    );
    assert_eq!(
        value["denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_count"],
        55
    );
    for key in [
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_performed_count",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_result_recorded_count",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_result_accepted_count",
        "source_dry_run_execution_result_receipt_replay_idempotency_denial_boundary_accepted_count",
        "dry_run_execution_result_receipt_ordering_monotonicity_denial_matrix_bound_count",
        "dry_run_execution_result_receipt_ordering_sequence_policy_bound_count",
        "dry_run_execution_result_receipt_ordering_identity_session_bound_count",
        "dry_run_execution_result_receipt_ordering_latest_sequence_bound_count",
        "dry_run_execution_result_receipt_late_receipt_denied_count",
        "dry_run_execution_result_receipt_future_receipt_denied_count",
        "dry_run_execution_result_receipt_rollback_sequence_denied_count",
        "dry_run_execution_result_receipt_same_sequence_replacement_denied_count",
        "dry_run_execution_result_receipt_latest_wins_promotion_denied_count",
        "dry_run_execution_result_receipt_sequence_gap_denied_count",
        "dry_run_execution_result_receipt_ordering_handoff_bound_count",
    ] {
        assert_eq!(
            value[key], 1,
            "ordering/monotonicity denial count should be one: {key}"
        );
    }
    for key in [
        "dry_run_execution_result_receipt_ordering_cursor_persisted_count",
        "dry_run_execution_result_receipt_ordering_ledger_written_count",
        "dry_run_execution_result_receipt_ordering_guard_state_recorded_count",
        "dry_run_execution_result_receipt_monotonic_sequence_recorded_count",
        "dry_run_execution_result_receipt_late_receipt_accepted_count",
        "dry_run_execution_result_receipt_future_receipt_accepted_count",
        "dry_run_execution_result_receipt_rollback_sequence_accepted_count",
        "dry_run_execution_result_receipt_same_sequence_replacement_accepted_count",
        "dry_run_execution_result_receipt_latest_wins_promoted_count",
        "dry_run_execution_result_receipt_sequence_gap_accepted_count",
        "dry_run_execution_result_receipt_persisted_count",
        "dry_run_execution_executed_count",
        "production_durable_memory_write_executed_count",
        "production_durable_memory_store_write_performed_count",
        "actual_production_durable_memory_write_performed_count",
        "durable_memory_store_write_performed_count",
        "durable_memory_store_read_performed_count",
        "durable_memory_store_rollback_performed_count",
        "memory_store_write_performed_count",
        "wal_write_performed_count",
        "receipt_persisted_count",
        "post_write_readback_performed_count",
        "rollback_performed_count",
        "tombstone_cleanup_executed_count",
        "live_kg_write_performed_count",
        "provider_invoked_count",
        "model_invoked_count",
        "credential_read_count",
        "channel_send_performed_count",
        "external_send_performed_count",
        "release_artifact_written_count",
        "install_executed_count",
        "service_restarted_count",
        "active_binary_mutated_count",
    ] {
        assert_eq!(
            value[key], 0,
            "ordering/monotonicity denial side-effect count should stay zero: {key}"
        );
    }
    for key in [
        "source_dry_run_execution_result_receipt_replay_idempotency_denial_boundary_bound",
        "dry_run_execution_result_receipt_ordering_monotonicity_denial_matrix_bound",
        "dry_run_execution_result_receipt_ordering_sequence_policy_bound",
        "dry_run_execution_result_receipt_late_receipt_denied",
        "dry_run_execution_result_receipt_future_receipt_denied",
        "dry_run_execution_result_receipt_rollback_sequence_denied",
        "dry_run_execution_result_receipt_same_sequence_replacement_denied",
        "dry_run_execution_result_receipt_latest_wins_promotion_denied",
        "dry_run_execution_result_receipt_sequence_gap_denied",
        "dry_run_execution_result_receipt_ordering_cursor_persistence_forbidden",
        "dry_run_execution_result_receipt_monotonic_sequence_recording_forbidden",
        "dry_run_execution_execution_forbidden_on_ordering_monotonicity_route",
        "production_write_execution_forbidden_on_ordering_monotonicity_route",
        "production_durable_memory_write_forbidden",
        "memory_store_mutation_forbidden",
        "kg_live_write_forbidden",
        "provider_model_invocation_forbidden",
        "credential_channel_public_release_forbidden",
        "install_restart_active_binary_mutation_forbidden",
    ] {
        assert_eq!(
            value[key], true,
            "ordering/monotonicity denial field should be true: {key}"
        );
    }
    for key in [
        "dry_run_execution_result_receipt_ordering_cursor_persisted",
        "dry_run_execution_result_receipt_ordering_ledger_written",
        "dry_run_execution_result_receipt_ordering_guard_state_recorded",
        "dry_run_execution_result_receipt_monotonic_sequence_recorded",
        "dry_run_execution_result_receipt_late_receipt_accepted",
        "dry_run_execution_result_receipt_future_receipt_accepted",
        "dry_run_execution_result_receipt_rollback_sequence_accepted",
        "dry_run_execution_result_receipt_same_sequence_replacement_accepted",
        "dry_run_execution_result_receipt_latest_wins_promoted",
        "dry_run_execution_result_receipt_sequence_gap_accepted",
        "dry_run_execution_result_receipt_persisted",
        "dry_run_execution_executed",
        "production_durable_memory_write_executed",
        "production_durable_memory_store_write_performed",
        "actual_production_durable_memory_write_performed",
        "durable_memory_store_write_performed",
        "durable_memory_store_read_performed",
        "durable_memory_store_rollback_performed",
        "memory_write_execution_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "wal_write_performed",
        "receipt_persisted",
        "post_write_readback_performed",
        "rollback_executed",
        "rollback_performed",
        "tombstone_write_performed",
        "tombstone_cleanup_executed",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "channel_send_performed",
        "external_send_performed",
        "release_artifact_written",
        "install_executed",
        "service_restarted",
        "active_binary_mutated",
    ] {
        assert_eq!(
            value[key], false,
            "ordering/monotonicity persistence, mutation, execution, or external field should stay false: {key}"
        );
    }
    let fixtures = value
            ["scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_fixtures"]
            .as_array()
            .expect(
                "scoped production durable Memory write dry-run execution result receipt ordering/monotonicity denial fixtures",
            );
    assert_eq!(fixtures.len(), 10);
    assert_eq!(
            fixtures
                .iter()
                .filter(|fixture| {
                    fixture["scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_accepted"]
                        == true
                })
                .count(),
            1
        );
    let denied = value
            ["denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary"]
            .as_array()
            .expect("scoped production durable Memory write dry-run execution result receipt ordering/monotonicity denials");
    assert_eq!(denied.len(), 55);
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "run_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_require_live_gate"
    );
    assert_eq!(
        value["allowed_next_actions"][0]["persists_ordering_cursor"],
        false
    );
    assert_eq!(
        value["allowed_next_actions"][0]["records_monotonic_sequence"],
        false
    );
    assert_eq!(value["allowed_next_actions"][0]["executes_dry_run"], false);
    assert_eq!(
        value["allowed_next_actions"][0]["writes_production_durable_memory"],
        false
    );
    assert_eq!(
        value["allowed_next_actions"][1]["action"],
        "prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary"
    );
    assert_eq!(
        value["allowed_next_actions"][1]["requires_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary"],
        true
    );
    let side_effects = value["side_effects"].as_object().expect(
            "scoped production durable Memory write dry-run execution result receipt ordering/monotonicity side effects",
        );
    assert_eq!(
            side_effects["scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_performed"].as_bool(),
            Some(true)
        );
    assert_eq!(
            side_effects["scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_result_accepted"].as_bool(),
            Some(true)
        );
    assert_eq!(
        side_effects["dry_run_execution_result_receipt_ordering_cursor_persisted"].as_bool(),
        Some(false)
    );
    assert_eq!(
        side_effects["dry_run_execution_result_receipt_monotonic_sequence_recorded"].as_bool(),
        Some(false)
    );
    assert_eq!(
        side_effects["dry_run_execution_result_receipt_late_receipt_accepted"].as_bool(),
        Some(false)
    );
    assert_eq!(
        side_effects["dry_run_execution_executed"].as_bool(),
        Some(false)
    );
    assert_eq!(
        side_effects["production_durable_memory_store_write_performed"].as_bool(),
        Some(false)
    );
    assert_eq!(
        side_effects["memory_store_write_performed"].as_bool(),
        Some(false)
    );
    assert_eq!(side_effects["wal_write_performed"].as_bool(), Some(false));
    assert_eq!(side_effects["receipt_persisted"].as_bool(), Some(false));
    assert_eq!(
        side_effects["external_send_performed"].as_bool(),
        Some(false)
    );
}
