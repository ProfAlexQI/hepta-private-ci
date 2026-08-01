#[test]
fn hepta_memory_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_plan_accepts_plan_without_memory_or_external_side_effects()
 {
    let body = route_contract_body(HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_MINIMAL_SCOPED_MEMORY_REAL_WRITE_CANARY_DURABLE_STORE_WRITE_PLAN_BOUNDARY_ENDPOINT);

    let value: serde_json::Value = serde_json::from_str(&body)
        .expect("minimal scoped Memory real-write canary durable store write plan json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_MINIMAL_SCOPED_MEMORY_REAL_WRITE_CANARY_DURABLE_STORE_WRITE_PLAN_BOUNDARY_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-plan-boundary --json"
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
        value["memory_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_plan_boundary_ready"],
        true
    );
    assert_eq!(
        value["minimal_scoped_memory_real_write_canary_durable_store_write_plan_ready"],
        true
    );
    assert_eq!(
        value["minimal_scoped_memory_real_write_canary_durable_store_write_plan_performed"],
        true
    );
    assert_eq!(
        value["scoped_memory_real_write_canary_mode"],
        "minimal_scoped_memory_real_write_canary_durable_store_write_plan_report_only"
    );
    assert_eq!(
        value["source_minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_boundary_ready"],
        true
    );
    assert_eq!(
        value["source_accepted_minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_fixture_count"],
        1
    );
    assert_eq!(
        value["source_blocked_minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_fixture_count"],
        9
    );
    assert_eq!(
        value["source_tombstone_cleanup_acceptance_result_accepted_count"],
        1
    );
    assert_eq!(value["source_tombstone_cleanup_executed_count"], 0);
    assert_eq!(value["source_artifact_cleanup_performed_count"], 0);
    assert_eq!(
        value["source_durable_memory_store_write_performed_count"],
        0
    );
    assert_eq!(value["source_memory_store_write_performed_count"], 0);
    assert_eq!(value["approved_namespace"], "hepta.memory.canary");
    assert_eq!(value["approved_store"], "wal-receipt-canary-artifact");
    assert_eq!(value["approved_scope"], "session");
    assert_ne!(value["source_tombstone_cleanup_acceptance_hash_sha256"], "");
    assert_ne!(value["source_tombstone_cleanup_receipt_linkage_sha256"], "");
    assert_eq!(
        value["durable_store_write_target_id"],
        "hepta-minimal-scoped-memory-real-write-canary-durable-store-write-target-v1"
    );
    assert_eq!(
        value["durable_store_target_store_id"],
        "hepta-memory-durable-store-canary-plan-only"
    );
    assert_ne!(value["durable_store_write_payload_digest_sha256"], "");
    assert_ne!(value["durable_store_write_target_sha256"], "");
    assert_ne!(value["durable_store_write_envelope_sha256"], "");
    assert_ne!(value["durable_store_write_wal_receipt_plan_sha256"], "");
    assert_ne!(value["durable_store_write_readback_plan_sha256"], "");
    assert_ne!(value["durable_store_write_rollback_plan_sha256"], "");
    assert_ne!(
        value["durable_store_write_tombstone_cleanup_plan_sha256"],
        ""
    );
    assert_ne!(value["durable_store_write_operator_handoff_sha256"], "");
    assert_ne!(value["durable_store_write_plan_hash_sha256"], "");
    assert_eq!(
        value["durable_store_write_plan_receipt_linkage_verified"],
        true
    );
    assert_eq!(
        value["durable_store_write_plan_rollback_tombstone_cleanup_verified"],
        true
    );
    assert_eq!(
        value["required_minimal_scoped_memory_real_write_canary_durable_store_write_plan_surface_count"],
        12
    );
    assert_eq!(
        value["ready_minimal_scoped_memory_real_write_canary_durable_store_write_plan_surface_count"],
        12
    );
    assert_eq!(
        value["minimal_scoped_memory_real_write_canary_durable_store_write_plan_fixture_count"],
        10
    );
    assert_eq!(
        value["accepted_minimal_scoped_memory_real_write_canary_durable_store_write_plan_fixture_count"],
        1
    );
    assert_eq!(
        value["blocked_minimal_scoped_memory_real_write_canary_durable_store_write_plan_fixture_count"],
        9
    );
    assert_eq!(
        value["denied_by_minimal_scoped_memory_real_write_canary_durable_store_write_plan_boundary_count"],
        30
    );
    for key in [
        "minimal_scoped_memory_real_write_canary_durable_store_write_plan_accepted_count",
        "durable_store_write_plan_authority_accepted_count",
        "source_tombstone_cleanup_acceptance_bound_count",
        "tombstone_cleanup_acceptance_hash_bound_count",
        "tombstone_cleanup_receipt_linkage_bound_count",
        "durable_store_target_bound_count",
        "durable_store_write_envelope_bound_count",
        "durable_store_write_payload_digest_bound_count",
        "durable_store_write_wal_receipt_plan_bound_count",
        "durable_store_write_readback_plan_bound_count",
        "durable_store_write_rollback_plan_bound_count",
        "durable_store_write_tombstone_cleanup_plan_bound_count",
        "durable_store_write_operator_handoff_bound_count",
        "durable_store_write_plan_result_recorded_count",
        "durable_store_write_plan_result_accepted_count",
    ] {
        assert_eq!(
            value[key], 1,
            "durable store write plan count should be one: {key}"
        );
    }
    for key in [
        "single_use_nonce_consumed_count",
        "explicit_command_dispatched_count",
        "durable_store_write_plan_executed_count",
        "wal_write_performed_count",
        "receipt_recorded_count",
        "receipt_persisted_count",
        "receipt_materialized_count",
        "artifact_cleanup_performed_count",
        "post_write_readback_performed_count",
        "readback_result_accepted_count",
        "rollback_performed_count",
        "rollback_result_accepted_count",
        "tombstone_cleanup_executed_count",
        "tombstone_written_count",
        "compensating_memory_write_performed_count",
        "durable_memory_store_read_performed_count",
        "durable_memory_store_write_performed_count",
        "durable_memory_store_rollback_performed_count",
        "memory_store_write_performed_count",
        "memory_store_mutation_performed_count",
        "raw_payload_plaintext_recorded_count",
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
            "durable store write plan side-effect count should stay zero: {key}"
        );
    }
    for key in [
        "durable_store_write_plan_performed",
        "durable_store_write_plan_result_recorded",
        "durable_store_write_plan_result_accepted",
        "durable_store_target_bound",
        "durable_store_write_envelope_bound",
        "durable_store_write_payload_digest_bound",
        "durable_store_write_wal_receipt_plan_bound",
        "durable_store_write_readback_plan_bound",
        "durable_store_write_rollback_plan_bound",
        "durable_store_write_tombstone_cleanup_plan_bound",
        "durable_store_write_operator_handoff_bound",
        "minimal_scoped_memory_real_write_canary_durable_store_write_plan_accepted",
        "source_tombstone_cleanup_acceptance_required",
        "approved_namespace_bound",
        "approved_store_bound",
        "approved_scope_bound",
        "tombstone_cleanup_acceptance_hash_bound",
        "tombstone_cleanup_receipt_linkage_bound",
        "durable_memory_write_forbidden",
        "memory_store_mutation_forbidden",
        "wal_write_forbidden_on_report_route",
        "receipt_persist_forbidden_on_report_route",
        "post_write_readback_forbidden_on_report_route",
        "rollback_execution_forbidden",
        "tombstone_write_forbidden",
        "artifact_cleanup_forbidden",
    ] {
        assert_eq!(
            value[key], true,
            "durable store write plan field should be true: {key}"
        );
    }
    for key in [
        "single_use_nonce_consumed",
        "explicit_command_dispatched",
        "durable_store_write_plan_executed",
        "wal_write_performed",
        "receipt_recorded",
        "receipt_persisted",
        "receipt_materialized",
        "artifact_cleanup_performed",
        "post_write_readback_performed",
        "readback_result_accepted",
        "rollback_executed",
        "rollback_performed",
        "tombstone_cleanup_executed",
        "tombstone_written",
        "compensating_memory_write_performed",
        "memory_write_execution_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "durable_memory_store_write_performed",
        "durable_memory_store_read_performed",
        "durable_memory_store_rollback_performed",
        "raw_payload_plaintext_recorded",
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
            "durable store write plan external or Memory field should stay false: {key}"
        );
    }
    let fixtures =
        value["minimal_scoped_memory_real_write_canary_durable_store_write_plan_fixtures"]
            .as_array()
            .expect("minimal scoped Memory durable store write plan fixtures");
    assert_eq!(fixtures.len(), 10);
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["minimal_scoped_memory_real_write_canary_durable_store_write_plan_accepted"]
                    == true
            })
            .count(),
        1
    );
    let denied =
            value["denied_by_minimal_scoped_memory_real_write_canary_durable_store_write_plan_boundary"]
                .as_array()
                .expect("minimal scoped Memory durable store write plan denials");
    assert_eq!(denied.len(), 30);
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "run_minimal_scoped_memory_real_write_canary_durable_store_write_plan_boundary_require_live_gate"
    );
    assert_eq!(
        value["allowed_next_actions"][0]["writes_durable_memory"],
        false
    );
    assert_eq!(
        value["allowed_next_actions"][0]["mutates_memory_store"],
        false
    );
    assert_eq!(
        value["allowed_next_actions"][1]["action"],
        "prepare_minimal_scoped_memory_real_write_canary_durable_store_write_preflight_boundary"
    );
    assert_eq!(
        value["allowed_next_actions"][1]["requires_minimal_scoped_memory_real_write_canary_durable_store_write_plan"],
        true
    );
    let side_effects = value["side_effects"]
        .as_object()
        .expect("minimal scoped Memory durable store write plan side effects");
    assert_eq!(
        side_effects["durable_store_write_plan_performed"].as_bool(),
        Some(true)
    );
    assert_eq!(
        side_effects["durable_store_write_plan_result_accepted"].as_bool(),
        Some(true)
    );
    assert_eq!(
        side_effects["durable_store_write_plan_executed"].as_bool(),
        Some(false)
    );
    assert_eq!(
        side_effects["durable_memory_store_write_performed"].as_bool(),
        Some(false)
    );
    assert_eq!(
        side_effects["memory_store_write_performed"].as_bool(),
        Some(false)
    );
    assert_eq!(
        side_effects["external_send_performed"].as_bool(),
        Some(false)
    );
}

#[test]
fn hepta_memory_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_preflight_accepts_preflight_without_memory_or_external_side_effects()
 {
    let body = route_contract_body(HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_MINIMAL_SCOPED_MEMORY_REAL_WRITE_CANARY_DURABLE_STORE_WRITE_PREFLIGHT_BOUNDARY_ENDPOINT);

    let value: serde_json::Value = serde_json::from_str(&body)
        .expect("minimal scoped Memory real-write canary durable store write preflight json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_MINIMAL_SCOPED_MEMORY_REAL_WRITE_CANARY_DURABLE_STORE_WRITE_PREFLIGHT_BOUNDARY_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-preflight-boundary --json"
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
        value["memory_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_preflight_boundary_ready"],
        true
    );
    assert_eq!(
        value["minimal_scoped_memory_real_write_canary_durable_store_write_preflight_ready"],
        true
    );
    assert_eq!(
        value["minimal_scoped_memory_real_write_canary_durable_store_write_preflight_performed"],
        true
    );
    assert_eq!(
        value["scoped_memory_real_write_canary_mode"],
        "minimal_scoped_memory_real_write_canary_durable_store_write_preflight_report_only"
    );
    assert_eq!(
        value["source_minimal_scoped_memory_real_write_canary_durable_store_write_plan_boundary_ready"],
        true
    );
    assert_eq!(
        value["source_accepted_minimal_scoped_memory_real_write_canary_durable_store_write_plan_fixture_count"],
        1
    );
    assert_eq!(
        value["source_blocked_minimal_scoped_memory_real_write_canary_durable_store_write_plan_fixture_count"],
        9
    );
    assert_eq!(
        value["source_durable_store_write_plan_result_accepted_count"],
        1
    );
    assert_eq!(value["source_durable_store_write_plan_executed_count"], 0);
    assert_eq!(
        value["source_durable_memory_store_write_performed_count"],
        0
    );
    assert_eq!(value["source_memory_store_write_performed_count"], 0);
    assert_eq!(value["approved_namespace"], "hepta.memory.canary");
    assert_eq!(value["approved_store"], "wal-receipt-canary-artifact");
    assert_eq!(value["approved_scope"], "session");
    assert_eq!(
        value["durable_store_write_target_id"],
        "hepta-minimal-scoped-memory-real-write-canary-durable-store-write-target-v1"
    );
    assert_eq!(
        value["durable_store_target_store_id"],
        "hepta-memory-durable-store-canary-plan-only"
    );
    for key in [
        "source_durable_store_write_plan_hash_sha256",
        "durable_store_write_preflight_target_reachability_sha256",
        "durable_store_write_preflight_namespace_scope_sha256",
        "durable_store_write_preflight_redaction_sha256",
        "durable_store_write_preflight_wal_receipt_sha256",
        "durable_store_write_preflight_readback_sha256",
        "durable_store_write_preflight_rollback_sha256",
        "durable_store_write_preflight_tombstone_cleanup_sha256",
        "durable_store_write_preflight_idempotency_replay_guard_sha256",
        "durable_store_write_preflight_operator_handoff_sha256",
        "durable_store_write_preflight_hash_sha256",
    ] {
        assert_ne!(
            value[key], "",
            "durable store write preflight hash should be present: {key}"
        );
    }
    assert_eq!(
        value["required_minimal_scoped_memory_real_write_canary_durable_store_write_preflight_surface_count"],
        12
    );
    assert_eq!(
        value["ready_minimal_scoped_memory_real_write_canary_durable_store_write_preflight_surface_count"],
        12
    );
    assert_eq!(
        value["minimal_scoped_memory_real_write_canary_durable_store_write_preflight_fixture_count"],
        10
    );
    assert_eq!(
        value["accepted_minimal_scoped_memory_real_write_canary_durable_store_write_preflight_fixture_count"],
        1
    );
    assert_eq!(
        value["blocked_minimal_scoped_memory_real_write_canary_durable_store_write_preflight_fixture_count"],
        9
    );
    assert_eq!(
        value["denied_by_minimal_scoped_memory_real_write_canary_durable_store_write_preflight_boundary_count"],
        30
    );
    for key in [
        "durable_store_write_preflight_performed",
        "durable_store_write_preflight_result_recorded",
        "durable_store_write_preflight_result_accepted",
        "durable_store_target_reachability_checked",
        "approved_namespace_store_scope_preflight_verified",
        "durable_store_write_envelope_preflight_verified",
        "durable_store_write_payload_digest_preflight_verified",
        "payload_redaction_preflight_verified",
        "payload_secret_plaintext_scan_passed",
        "durable_store_write_wal_receipt_preflight_bound",
        "durable_store_write_readback_preflight_bound",
        "durable_store_write_rollback_preflight_bound",
        "durable_store_write_tombstone_cleanup_preflight_bound",
        "durable_store_write_idempotency_replay_guard_preflight_bound",
        "durable_store_write_operator_preflight_handoff_bound",
        "minimal_scoped_memory_real_write_canary_durable_store_write_preflight_accepted",
        "durable_memory_write_forbidden",
        "memory_store_mutation_forbidden",
        "wal_write_forbidden_on_report_route",
        "receipt_persist_forbidden_on_report_route",
        "post_write_readback_forbidden_on_report_route",
        "rollback_execution_forbidden",
        "tombstone_write_forbidden",
        "artifact_cleanup_forbidden",
    ] {
        assert_eq!(
            value[key], true,
            "durable store write preflight field should be true: {key}"
        );
    }
    for key in [
        "durable_store_write_preflight_executed",
        "durable_store_write_plan_executed",
        "wal_write_performed",
        "receipt_persisted",
        "post_write_readback_performed",
        "rollback_executed",
        "tombstone_cleanup_executed",
        "tombstone_written",
        "durable_memory_store_write_performed",
        "durable_memory_store_read_performed",
        "durable_memory_store_rollback_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "raw_payload_plaintext_recorded",
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
            "durable store write preflight side-effect field should stay false: {key}"
        );
    }
    let fixtures =
        value["minimal_scoped_memory_real_write_canary_durable_store_write_preflight_fixtures"]
            .as_array()
            .expect("minimal scoped Memory durable store write preflight fixtures");
    assert_eq!(fixtures.len(), 10);
    assert_eq!(
            fixtures
                .iter()
                .filter(|fixture| {
                    fixture["minimal_scoped_memory_real_write_canary_durable_store_write_preflight_accepted"]
                        == true
                })
                .count(),
            1
        );
    let denied =
            value["denied_by_minimal_scoped_memory_real_write_canary_durable_store_write_preflight_boundary"]
                .as_array()
                .expect("minimal scoped Memory durable store write preflight denials");
    assert_eq!(denied.len(), 30);
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "run_minimal_scoped_memory_real_write_canary_durable_store_write_preflight_boundary_require_live_gate"
    );
    assert_eq!(
        value["allowed_next_actions"][0]["writes_durable_memory"],
        false
    );
    assert_eq!(
        value["allowed_next_actions"][0]["mutates_memory_store"],
        false
    );
    assert_eq!(
        value["allowed_next_actions"][1]["action"],
        "prepare_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_boundary"
    );
    assert_eq!(
        value["allowed_next_actions"][1]["requires_minimal_scoped_memory_real_write_canary_durable_store_write_preflight"],
        true
    );
    let side_effects = value["side_effects"]
        .as_object()
        .expect("minimal scoped Memory durable store write preflight side effects");
    assert_eq!(
        side_effects["durable_store_write_preflight_performed"].as_bool(),
        Some(true)
    );
    assert_eq!(
        side_effects["durable_store_write_preflight_result_accepted"].as_bool(),
        Some(true)
    );
    assert_eq!(
        side_effects["durable_store_write_preflight_executed"].as_bool(),
        Some(false)
    );
    assert_eq!(
        side_effects["durable_memory_store_write_performed"].as_bool(),
        Some(false)
    );
    assert_eq!(
        side_effects["memory_store_write_performed"].as_bool(),
        Some(false)
    );
    assert_eq!(
        side_effects["external_send_performed"].as_bool(),
        Some(false)
    );
}

#[test]
fn hepta_memory_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_accepts_readiness_without_memory_or_external_side_effects()
 {
    let body = route_contract_body(HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_MINIMAL_SCOPED_MEMORY_REAL_WRITE_CANARY_DURABLE_STORE_WRITE_GUARDED_EXECUTION_READINESS_BOUNDARY_ENDPOINT);

    let value: serde_json::Value = serde_json::from_str(&body)
            .expect("minimal scoped Memory real-write canary durable store write guarded execution readiness json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_MINIMAL_SCOPED_MEMORY_REAL_WRITE_CANARY_DURABLE_STORE_WRITE_GUARDED_EXECUTION_READINESS_BOUNDARY_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-guarded-execution-readiness-boundary --json"
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
        value["memory_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_boundary_ready"],
        true
    );
    assert_eq!(
        value["minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_ready"],
        true
    );
    assert_eq!(
        value["minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_performed"],
        true
    );
    assert_eq!(
        value["scoped_memory_real_write_canary_mode"],
        "minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_report_only"
    );
    assert_eq!(
        value["source_minimal_scoped_memory_real_write_canary_durable_store_write_preflight_boundary_ready"],
        true
    );
    assert_eq!(
        value["source_accepted_minimal_scoped_memory_real_write_canary_durable_store_write_preflight_fixture_count"],
        1
    );
    assert_eq!(
        value["source_blocked_minimal_scoped_memory_real_write_canary_durable_store_write_preflight_fixture_count"],
        9
    );
    assert_eq!(
        value["source_durable_store_write_preflight_result_accepted_count"],
        1
    );
    assert_eq!(
        value["source_durable_store_write_preflight_executed_count"],
        0
    );
    assert_eq!(value["approved_namespace"], "hepta.memory.canary");
    assert_eq!(value["approved_store"], "wal-receipt-canary-artifact");
    assert_eq!(value["approved_scope"], "session");
    assert_eq!(
        value["durable_store_write_target_id"],
        "hepta-minimal-scoped-memory-real-write-canary-durable-store-write-target-v1"
    );
    assert_eq!(
        value["durable_store_target_store_id"],
        "hepta-memory-durable-store-canary-plan-only"
    );
    for key in [
        "source_durable_store_write_preflight_hash_sha256",
        "source_durable_store_write_preflight_operator_handoff_sha256",
        "guarded_execution_envelope_sha256",
        "single_use_nonce_guard_sha256",
        "explicit_command_guard_sha256",
        "single_write_budget_guard_sha256",
        "wal_receipt_guard_sha256",
        "readback_guard_sha256",
        "rollback_guard_sha256",
        "tombstone_cleanup_guard_sha256",
        "idempotency_replay_guard_sha256",
        "operator_guarded_execution_handoff_sha256",
        "guarded_execution_readiness_hash_sha256",
    ] {
        assert_ne!(
            value[key], "",
            "durable store write guarded execution readiness hash should be present: {key}"
        );
    }
    assert_eq!(
        value["required_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_surface_count"],
        12
    );
    assert_eq!(
        value["ready_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_surface_count"],
        12
    );
    assert_eq!(
        value["minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_fixture_count"],
        10
    );
    assert_eq!(
        value["accepted_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_fixture_count"],
        1
    );
    assert_eq!(
        value["blocked_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_fixture_count"],
        9
    );
    assert_eq!(
        value["denied_by_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_boundary_count"],
        32
    );
    for key in [
        "durable_store_write_guarded_execution_readiness_performed",
        "durable_store_write_guarded_execution_readiness_result_recorded",
        "durable_store_write_guarded_execution_readiness_result_accepted",
        "source_durable_store_write_preflight_bound",
        "source_durable_store_write_preflight_hash_bound",
        "source_durable_store_write_preflight_result_accepted",
        "approved_namespace_store_scope_guard_verified",
        "durable_store_target_guard_verified",
        "guarded_execution_envelope_bound",
        "single_use_nonce_guard_bound",
        "explicit_command_guard_bound",
        "single_write_budget_guard_bound",
        "wal_receipt_guard_bound",
        "post_write_readback_guard_bound",
        "rollback_guard_bound",
        "tombstone_cleanup_guard_bound",
        "idempotency_replay_guard_bound",
        "operator_guarded_execution_handoff_bound",
        "durable_memory_write_forbidden_until_guarded_execution_boundary",
        "memory_store_mutation_forbidden_until_guarded_execution_boundary",
        "kg_provider_channel_release_install_active_binary_forbidden",
        "minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_accepted",
    ] {
        assert_eq!(
            value[key], true,
            "durable store write guarded execution readiness field should be true: {key}"
        );
    }
    for key in [
        "durable_store_write_guarded_execution_readiness_executed",
        "durable_store_write_guarded_execution_executed",
        "durable_store_write_execution_performed",
        "durable_store_write_preflight_executed",
        "wal_write_performed",
        "receipt_persisted",
        "post_write_readback_performed",
        "rollback_executed",
        "tombstone_cleanup_executed",
        "tombstone_written",
        "durable_memory_store_write_performed",
        "durable_memory_store_read_performed",
        "durable_memory_store_rollback_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "raw_payload_plaintext_recorded",
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
            "durable store write guarded execution readiness side-effect field should stay false: {key}"
        );
    }
    let fixtures = value
            ["minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_fixtures"]
            .as_array()
            .expect("minimal scoped Memory durable store write guarded execution readiness fixtures");
    assert_eq!(fixtures.len(), 10);
    assert_eq!(
            fixtures
                .iter()
                .filter(|fixture| {
                    fixture["minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_accepted"]
                        == true
                })
                .count(),
            1
        );
    let denied = value
            ["denied_by_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_boundary"]
            .as_array()
            .expect("minimal scoped Memory durable store write guarded execution readiness denials");
    assert_eq!(denied.len(), 32);
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "run_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_boundary_require_live_gate"
    );
    assert_eq!(
        value["allowed_next_actions"][0]["writes_durable_memory"],
        false
    );
    assert_eq!(
        value["allowed_next_actions"][1]["action"],
        "prepare_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_boundary"
    );
    assert_eq!(
        value["allowed_next_actions"][1]["requires_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness"],
        true
    );
    let side_effects = value["side_effects"].as_object().expect(
        "minimal scoped Memory durable store write guarded execution readiness side effects",
    );
    assert_eq!(
        side_effects["durable_store_write_guarded_execution_readiness_performed"].as_bool(),
        Some(true)
    );
    assert_eq!(
        side_effects["durable_store_write_guarded_execution_readiness_result_accepted"].as_bool(),
        Some(true)
    );
    assert_eq!(
        side_effects["durable_store_write_guarded_execution_executed"].as_bool(),
        Some(false)
    );
    assert_eq!(
        side_effects["durable_memory_store_write_performed"].as_bool(),
        Some(false)
    );
    assert_eq!(
        side_effects["memory_store_write_performed"].as_bool(),
        Some(false)
    );
    assert_eq!(
        side_effects["external_send_performed"].as_bool(),
        Some(false)
    );
}

#[test]
fn hepta_memory_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_accepts_boundary_without_memory_or_external_side_effects()
 {
    let body = route_contract_body(HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_MINIMAL_SCOPED_MEMORY_REAL_WRITE_CANARY_DURABLE_STORE_WRITE_GUARDED_EXECUTION_BOUNDARY_ENDPOINT);

    let value: serde_json::Value = serde_json::from_str(&body).expect(
        "minimal scoped Memory real-write canary durable store write guarded execution json",
    );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_MINIMAL_SCOPED_MEMORY_REAL_WRITE_CANARY_DURABLE_STORE_WRITE_GUARDED_EXECUTION_BOUNDARY_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-guarded-execution-boundary --json"
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
        value["memory_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_boundary_ready"],
        true
    );
    assert_eq!(
        value["minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_ready"],
        true
    );
    assert_eq!(
        value["minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_performed"],
        true
    );
    assert_eq!(
        value["minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_accepted"],
        true
    );
    assert_eq!(
        value["scoped_memory_real_write_canary_mode"],
        "minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_report_only"
    );
    assert_eq!(
        value["source_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_boundary_ready"],
        true
    );
    assert_eq!(
        value["source_accepted_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_fixture_count"],
        1
    );
    assert_eq!(
        value["source_blocked_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_fixture_count"],
        9
    );
    assert_eq!(
        value["source_durable_store_write_guarded_execution_readiness_result_accepted_count"],
        1
    );
    assert_eq!(
        value["source_durable_store_write_guarded_execution_readiness_executed_count"],
        0
    );
    assert_eq!(
        value["source_durable_store_write_guarded_execution_executed_count"],
        0
    );
    assert_eq!(value["approved_namespace"], "hepta.memory.canary");
    assert_eq!(value["approved_store"], "wal-receipt-canary-artifact");
    assert_eq!(value["approved_scope"], "session");
    assert_eq!(
        value["durable_store_write_target_id"],
        "hepta-minimal-scoped-memory-real-write-canary-durable-store-write-target-v1"
    );
    assert_eq!(
        value["durable_store_target_store_id"],
        "hepta-memory-durable-store-canary-plan-only"
    );
    for key in [
        "source_guarded_execution_readiness_hash_sha256",
        "source_guarded_execution_envelope_sha256",
        "source_single_use_nonce_guard_sha256",
        "source_explicit_command_guard_sha256",
        "source_single_write_budget_guard_sha256",
        "source_wal_receipt_guard_sha256",
        "source_readback_guard_sha256",
        "source_rollback_guard_sha256",
        "source_tombstone_cleanup_guard_sha256",
        "source_idempotency_replay_guard_sha256",
        "source_operator_guarded_execution_handoff_sha256",
        "guarded_execution_boundary_envelope_sha256",
        "guarded_execution_boundary_nonce_sha256",
        "guarded_execution_boundary_command_sha256",
        "guarded_execution_boundary_budget_sha256",
        "guarded_execution_boundary_wal_receipt_sha256",
        "guarded_execution_boundary_readback_sha256",
        "guarded_execution_boundary_rollback_sha256",
        "guarded_execution_boundary_tombstone_cleanup_sha256",
        "guarded_execution_boundary_idempotency_replay_sha256",
        "operator_guarded_execution_boundary_handoff_sha256",
        "guarded_execution_boundary_hash_sha256",
    ] {
        assert_ne!(
            value[key], "",
            "durable store write guarded execution boundary hash should be present: {key}"
        );
    }
    assert_eq!(
        value["required_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_surface_count"],
        12
    );
    assert_eq!(
        value["ready_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_surface_count"],
        12
    );
    assert_eq!(
        value["minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_fixture_count"],
        10
    );
    assert_eq!(
        value["accepted_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_fixture_count"],
        1
    );
    assert_eq!(
        value["blocked_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_fixture_count"],
        9
    );
    assert_eq!(
        value["denied_by_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_boundary_count"],
        34
    );
    for key in [
        "durable_store_write_guarded_execution_boundary_performed",
        "durable_store_write_guarded_execution_boundary_result_recorded",
        "durable_store_write_guarded_execution_boundary_result_accepted",
        "source_durable_store_write_guarded_execution_readiness_bound",
        "source_durable_store_write_guarded_execution_readiness_hash_bound",
        "source_durable_store_write_guarded_execution_readiness_result_accepted",
        "approved_namespace_store_scope_execution_guard_verified",
        "durable_store_target_execution_guard_verified",
        "guarded_execution_boundary_envelope_bound",
        "single_use_nonce_execution_guard_verified",
        "explicit_command_execution_guard_verified",
        "single_write_budget_execution_guard_verified",
        "wal_receipt_execution_guard_verified",
        "post_write_readback_execution_guard_verified",
        "rollback_execution_guard_verified",
        "tombstone_cleanup_execution_guard_verified",
        "idempotency_replay_execution_guard_verified",
        "operator_guarded_execution_boundary_handoff_bound",
        "durable_memory_write_forbidden_until_single_shot_execution",
        "memory_store_mutation_forbidden_until_single_shot_execution",
        "kg_provider_channel_release_install_active_binary_forbidden",
        "minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_accepted",
    ] {
        assert_eq!(
            value[key], true,
            "durable store write guarded execution boundary field should be true: {key}"
        );
    }
    for key in [
        "durable_store_write_guarded_execution_boundary_executed",
        "durable_store_write_guarded_execution_executed",
        "durable_store_write_execution_performed",
        "durable_memory_store_write_performed",
        "durable_memory_store_read_performed",
        "durable_memory_store_rollback_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "wal_write_performed",
        "receipt_persisted",
        "post_write_readback_performed",
        "rollback_executed",
        "tombstone_cleanup_executed",
        "tombstone_written",
        "raw_payload_plaintext_recorded",
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
            "durable store write guarded execution boundary side-effect field should stay false: {key}"
        );
    }
    let fixtures = value
            ["minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_fixtures"]
            .as_array()
            .expect("minimal scoped Memory durable store write guarded execution fixtures");
    assert_eq!(fixtures.len(), 10);
    assert_eq!(
            fixtures
                .iter()
                .filter(|fixture| {
                    fixture["minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_accepted"]
                        == true
                })
                .count(),
            1
        );
    let denied = value
            ["denied_by_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_boundary"]
            .as_array()
            .expect("minimal scoped Memory durable store write guarded execution denials");
    assert_eq!(denied.len(), 34);
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "run_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_boundary_require_live_gate"
    );
    assert_eq!(
        value["allowed_next_actions"][0]["writes_durable_memory"],
        false
    );
    assert_eq!(
        value["allowed_next_actions"][1]["action"],
        "prepare_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_boundary"
    );
    assert_eq!(
        value["allowed_next_actions"][1]["requires_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_boundary"],
        true
    );
    assert_eq!(
        value["allowed_next_actions"][1]["actual_write_requires_separate_explicit_command"],
        true
    );
    let side_effects = value["side_effects"]
        .as_object()
        .expect("minimal scoped Memory durable store write guarded execution side effects");
    assert_eq!(
        side_effects["durable_store_write_guarded_execution_boundary_performed"].as_bool(),
        Some(true)
    );
    assert_eq!(
        side_effects["durable_store_write_guarded_execution_boundary_result_accepted"].as_bool(),
        Some(true)
    );
    assert_eq!(
        side_effects["durable_store_write_guarded_execution_executed"].as_bool(),
        Some(false)
    );
    assert_eq!(
        side_effects["durable_memory_store_write_performed"].as_bool(),
        Some(false)
    );
    assert_eq!(
        side_effects["memory_store_write_performed"].as_bool(),
        Some(false)
    );
    assert_eq!(
        side_effects["external_send_performed"].as_bool(),
        Some(false)
    );
}

#[test]
fn hepta_memory_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_executes_canary_store_with_zero_residue_without_production_or_external_side_effects()
 {
    let body = route_contract_body(HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_MINIMAL_SCOPED_MEMORY_REAL_WRITE_CANARY_DURABLE_STORE_WRITE_SINGLE_SHOT_EXECUTION_BOUNDARY_ENDPOINT);

    let value: serde_json::Value = serde_json::from_str(&body).expect(
        "minimal scoped Memory real-write canary durable store write single-shot execution json",
    );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_MINIMAL_SCOPED_MEMORY_REAL_WRITE_CANARY_DURABLE_STORE_WRITE_SINGLE_SHOT_EXECUTION_BOUNDARY_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-single-shot-execution-boundary --json"
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
        value["memory_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_boundary_ready"],
        true
    );
    assert_eq!(
        value["minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_ready"],
        true
    );
    assert_eq!(
        value["minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_performed"],
        true
    );
    assert_eq!(
        value["minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_accepted"],
        true
    );
    assert_eq!(
        value["scoped_memory_real_write_canary_mode"],
        "minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_request_local_canary_store"
    );
    assert_eq!(
        value["durable_store_write_execution_scope"],
        "request_local_canary_store_with_request_local_wal_receipt_artifacts"
    );
    assert_eq!(
        value["source_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_boundary_ready"],
        true
    );
    assert_eq!(
        value["source_accepted_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_fixture_count"],
        1
    );
    assert_eq!(
        value["source_blocked_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_fixture_count"],
        9
    );
    assert_eq!(
        value["source_durable_store_write_guarded_execution_boundary_result_accepted_count"],
        1
    );
    assert_eq!(value["approved_namespace"], "hepta.memory.canary");
    assert_eq!(value["approved_store"], "wal-receipt-canary-artifact");
    assert_eq!(value["approved_scope"], "session");
    assert_eq!(
        value["durable_store_write_target_id"],
        "hepta-minimal-scoped-memory-real-write-canary-durable-store-write-target-v1"
    );
    assert_eq!(
        value["durable_store_target_store_id"],
        "hepta-memory-durable-store-canary-plan-only"
    );
    for key in [
        "source_guarded_execution_boundary_hash_sha256",
        "source_guarded_execution_boundary_report_hash_sha256",
        "source_guarded_execution_boundary_handoff_sha256",
        "source_guarded_execution_boundary_wal_receipt_sha256",
        "source_guarded_execution_boundary_readback_sha256",
        "source_guarded_execution_boundary_rollback_sha256",
        "source_guarded_execution_boundary_tombstone_cleanup_sha256",
        "source_guarded_execution_boundary_replay_sha256",
        "canary_payload_digest_sha256",
        "single_shot_execution_envelope_sha256",
        "single_shot_nonce_sha256",
        "single_shot_command_sha256",
        "single_shot_budget_sha256",
        "single_shot_wal_hash_sha256",
        "single_shot_receipt_hash_sha256",
        "single_shot_receipt_hash_chain_sha256",
        "single_shot_cleanup_receipt_hash_sha256",
        "single_shot_execution_hash_sha256",
        "minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_boundary_hash_sha256",
        "minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_policy_hash_sha256",
    ] {
        assert_ne!(
            value[key], "",
            "single-shot execution hash should be present: {key}"
        );
    }
    assert_eq!(value["single_shot_canary_pre_write_memory_count"], 0);
    assert_eq!(value["single_shot_canary_post_write_memory_count"], 1);
    assert_eq!(value["single_shot_canary_readback_hit_count"], 1);
    assert_eq!(value["single_shot_canary_rollback_restored"], true);
    assert_eq!(value["single_shot_canary_post_rollback_memory_count"], 0);
    assert_eq!(
        value["single_shot_canary_post_rollback_absence_confirmed"],
        true
    );
    assert_eq!(value["single_shot_canary_artifact_pre_count"], 0);
    assert_eq!(value["single_shot_canary_artifact_write_count"], 3);
    assert_eq!(value["single_shot_canary_artifact_readback_count"], 3);
    assert_eq!(
        value["single_shot_canary_artifact_cleanup_removed_count"],
        3
    );
    assert_eq!(value["single_shot_canary_artifact_post_cleanup_count"], 0);
    assert_eq!(
        value["single_shot_canary_artifact_zero_residue_confirmed"],
        true
    );
    assert_eq!(
        value["required_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_surface_count"],
        12
    );
    assert_eq!(
        value["ready_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_surface_count"],
        12
    );
    assert_eq!(
        value["minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_fixture_count"],
        10
    );
    assert_eq!(
        value["accepted_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_fixture_count"],
        1
    );
    assert_eq!(
        value["blocked_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_fixture_count"],
        9
    );
    assert_eq!(
        value["durable_store_write_single_shot_execution_result_accepted_count"],
        1
    );
    assert_eq!(
        value["denied_by_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_boundary_count"],
        36
    );
    for key in [
        "durable_store_write_execution_performed",
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
        "readback_result_recorded",
        "readback_result_accepted",
        "rollback_executed",
        "rollback_performed",
        "rollback_result_recorded",
        "rollback_result_accepted",
        "tombstone_cleanup_executed",
        "tombstone_cleanup_result_recorded",
        "tombstone_cleanup_result_accepted",
        "single_shot_canary_nonce_consumed",
        "single_shot_canary_explicit_command_accepted",
        "single_shot_canary_receipt_hash_chain_verified",
        "single_shot_canary_zero_residue_confirmed",
        "operator_single_shot_execution_handoff_bound",
        "kg_provider_channel_release_install_active_binary_forbidden",
    ] {
        assert_eq!(
            value[key], true,
            "single-shot canary execution field should be true: {key}"
        );
    }
    for key in [
        "production_durable_memory_backend_present",
        "production_durable_memory_store_write_performed",
        "actual_production_durable_memory_write_performed",
        "durable_memory_store_write_performed",
        "durable_memory_store_read_performed",
        "durable_memory_store_rollback_performed",
        "raw_payload_plaintext_recorded",
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
            "single-shot production/external side effect should stay false: {key}"
        );
    }
    let fixtures = value
            ["minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_fixtures"]
            .as_array()
            .expect("minimal scoped Memory durable store write single-shot fixtures");
    assert_eq!(fixtures.len(), 10);
    assert_eq!(
            fixtures
                .iter()
                .filter(|fixture| {
                    fixture["minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_accepted"]
                        == true
                })
                .count(),
            1
        );
    let denied = value
            ["denied_by_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_boundary"]
            .as_array()
            .expect("minimal scoped Memory durable store write single-shot denials");
    assert_eq!(denied.len(), 36);
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "run_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_boundary_require_live_gate"
    );
    assert_eq!(
        value["allowed_next_actions"][0]["writes_production_durable_memory"],
        false
    );
    assert_eq!(
        value["allowed_next_actions"][0]["mutates_request_local_canary_store"],
        true
    );
    assert_eq!(
        value["allowed_next_actions"][1]["action"],
        "prepare_minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_boundary"
    );
    assert_eq!(
        value["allowed_next_actions"][1]["requires_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_boundary"],
        true
    );
    let side_effects = value["side_effects"]
        .as_object()
        .expect("minimal scoped Memory durable store write single-shot side effects");
    assert_eq!(
        side_effects["durable_store_write_single_shot_execution_result_accepted"].as_bool(),
        Some(true)
    );
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
fn hepta_memory_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_accepts_single_shot_receipt_without_new_memory_or_external_side_effects()
 {
    let body = route_contract_body(HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_MINIMAL_SCOPED_MEMORY_REAL_WRITE_CANARY_DURABLE_STORE_WRITE_RECEIPT_ACCEPTANCE_BOUNDARY_ENDPOINT);

    let value: serde_json::Value = serde_json::from_str(&body).expect(
        "minimal scoped Memory real-write canary durable store write receipt acceptance json",
    );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_MINIMAL_SCOPED_MEMORY_REAL_WRITE_CANARY_DURABLE_STORE_WRITE_RECEIPT_ACCEPTANCE_BOUNDARY_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-receipt-acceptance-boundary --json"
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
        value["memory_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_boundary_ready"],
        true
    );
    assert_eq!(
        value["minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_ready"],
        true
    );
    assert_eq!(
        value["minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_performed"],
        true
    );
    assert_eq!(
        value["minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_accepted"],
        true
    );
    assert_eq!(
        value["scoped_memory_real_write_canary_mode"],
        "minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_report_only"
    );
    assert_eq!(
        value["source_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_boundary_ready"],
        true
    );
    assert_eq!(
        value["source_accepted_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_fixture_count"],
        1
    );
    assert_eq!(
        value["source_blocked_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_fixture_count"],
        9
    );
    assert_eq!(
        value["source_durable_store_write_single_shot_execution_result_accepted_count"],
        1
    );
    assert_eq!(value["source_memory_store_write_performed_count"], 1);
    assert_eq!(value["source_wal_write_performed_count"], 1);
    assert_eq!(value["source_receipt_persisted_count"], 1);
    assert_eq!(value["source_post_write_readback_performed_count"], 1);
    assert_eq!(value["source_rollback_executed_count"], 1);
    assert_eq!(value["source_tombstone_cleanup_executed_count"], 1);
    assert_eq!(
        value["source_production_durable_memory_store_write_performed_count"],
        0
    );
    assert_eq!(
        value["source_durable_memory_store_write_performed_count"],
        0
    );
    assert_eq!(value["source_live_kg_write_performed_count"], 0);
    assert_eq!(value["source_external_send_performed_count"], 0);
    assert_eq!(value["approved_namespace"], "hepta.memory.canary");
    assert_eq!(value["approved_store"], "wal-receipt-canary-artifact");
    assert_eq!(value["approved_scope"], "session");
    assert_eq!(
        value["durable_store_write_target_id"],
        "hepta-minimal-scoped-memory-real-write-canary-durable-store-write-target-v1"
    );
    assert_eq!(
        value["durable_store_target_store_id"],
        "hepta-memory-durable-store-canary-plan-only"
    );
    assert_eq!(
        value["canary_record_id"],
        "hepta-minimal-scoped-memory-real-write-canary-durable-store-write-single-shot-record-v1"
    );
    for key in [
        "source_single_shot_boundary_hash_sha256",
        "source_single_shot_policy_hash_sha256",
        "canary_payload_digest_sha256",
        "single_shot_execution_envelope_sha256",
        "single_shot_nonce_sha256",
        "single_shot_command_sha256",
        "single_shot_budget_sha256",
        "single_shot_wal_hash_sha256",
        "single_shot_receipt_hash_sha256",
        "single_shot_receipt_hash_chain_sha256",
        "single_shot_cleanup_receipt_hash_sha256",
        "single_shot_execution_hash_sha256",
        "receipt_acceptance_record_hash_sha256",
        "receipt_acceptance_readback_hash_sha256",
        "receipt_acceptance_hash_sha256",
        "minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_boundary_hash_sha256",
        "minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_policy_hash_sha256",
    ] {
        assert_ne!(
            value[key], "",
            "durable store write receipt acceptance hash should be present: {key}"
        );
    }
    assert_eq!(
        value["source_single_shot_canary_post_write_memory_count"],
        1
    );
    assert_eq!(value["source_single_shot_canary_readback_hit_count"], 1);
    assert_eq!(value["source_single_shot_canary_rollback_restored"], true);
    assert_eq!(
        value["source_single_shot_canary_post_rollback_memory_count"],
        0
    );
    assert_eq!(
        value["source_single_shot_canary_post_rollback_absence_confirmed"],
        true
    );
    assert_eq!(value["source_single_shot_canary_artifact_write_count"], 3);
    assert_eq!(
        value["source_single_shot_canary_artifact_readback_count"],
        3
    );
    assert_eq!(
        value["source_single_shot_canary_artifact_cleanup_removed_count"],
        3
    );
    assert_eq!(
        value["source_single_shot_canary_artifact_post_cleanup_count"],
        0
    );
    assert_eq!(
        value["source_single_shot_canary_artifact_zero_residue_confirmed"],
        true
    );
    assert_eq!(value["receipt_readback_digest_match"], true);
    assert_eq!(value["receipt_hash_chain_verified"], true);
    assert_eq!(
        value["single_shot_rollback_cleanup_zero_residue_verified"],
        true
    );
    assert_eq!(
        value["required_minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_surface_count"],
        12
    );
    assert_eq!(
        value["ready_minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_surface_count"],
        12
    );
    assert_eq!(
        value["minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_fixture_count"],
        10
    );
    assert_eq!(
        value["accepted_minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_fixture_count"],
        1
    );
    assert_eq!(
        value["blocked_minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_fixture_count"],
        9
    );
    assert_eq!(
        value["denied_by_minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_boundary_count"],
        32
    );
    for key in [
        "minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_accepted_count",
        "durable_store_write_receipt_acceptance_authority_accepted_count",
        "source_single_shot_execution_bound_count",
        "single_shot_receipt_identity_bound_count",
        "single_shot_receipt_hash_chain_bound_count",
        "single_shot_readback_evidence_bound_count",
        "single_shot_rollback_cleanup_zero_residue_bound_count",
        "receipt_acceptance_record_bound_count",
        "receipt_acceptance_result_recorded_count",
        "receipt_acceptance_result_accepted_count",
        "receipt_acceptance_replay_guard_accepted_count",
        "operator_receipt_acceptance_handoff_bound_count",
        "rollback_tombstone_zero_residue_handoff_bound_count",
    ] {
        assert_eq!(
            value[key], 1,
            "durable store write receipt acceptance count should be one: {key}"
        );
    }
    for key in [
        "durable_store_write_execution_performed_count",
        "durable_store_write_single_shot_execution_performed_count",
        "memory_write_execution_performed_count",
        "memory_store_write_performed_count",
        "wal_write_performed_count",
        "receipt_persisted_count",
        "post_write_readback_performed_count",
        "readback_result_accepted_count",
        "rollback_performed_count",
        "tombstone_cleanup_executed_count",
        "durable_memory_store_read_performed_count",
        "durable_memory_store_write_performed_count",
        "durable_memory_store_rollback_performed_count",
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
            "durable store write receipt acceptance side-effect count should stay zero: {key}"
        );
    }
    for key in [
        "durable_store_write_receipt_acceptance_performed",
        "durable_store_write_receipt_acceptance_result_recorded",
        "durable_store_write_receipt_acceptance_result_accepted",
        "single_shot_receipt_identity_accepted",
        "single_shot_receipt_hash_chain_accepted",
        "single_shot_readback_evidence_accepted",
        "single_shot_rollback_cleanup_zero_residue_evidence_accepted",
        "receipt_acceptance_recorded",
        "receipt_acceptance_replay_guard_accepted",
        "operator_receipt_acceptance_handoff_bound",
        "rollback_tombstone_zero_residue_handoff_bound",
        "approved_namespace_bound",
        "approved_store_bound",
        "approved_scope_bound",
        "single_shot_receipt_identity_bound",
        "single_shot_receipt_hash_chain_bound",
        "single_shot_readback_evidence_bound",
        "single_shot_rollback_cleanup_zero_residue_bound",
        "new_canary_store_write_forbidden_on_report_route",
        "wal_rewrite_forbidden_on_report_route",
        "receipt_repersist_forbidden_on_report_route",
        "kg_provider_channel_release_install_active_binary_forbidden",
    ] {
        assert_eq!(
            value[key], true,
            "durable store write receipt acceptance field should be true: {key}"
        );
    }
    for key in [
        "durable_store_write_execution_performed",
        "durable_store_write_single_shot_execution_performed",
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
        "readback_result_accepted",
        "rollback_executed",
        "rollback_performed",
        "tombstone_cleanup_executed",
        "production_durable_memory_store_write_performed",
        "actual_production_durable_memory_write_performed",
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
            "durable store write receipt acceptance external or Memory field should stay false: {key}"
        );
    }
    let fixtures =
            value["minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_fixtures"]
                .as_array()
                .expect("minimal scoped Memory durable store write receipt acceptance fixtures");
    assert_eq!(fixtures.len(), 10);
    assert_eq!(
            fixtures
                .iter()
                .filter(|fixture| {
                    fixture["minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_accepted"]
                        == true
                })
                .count(),
            1
        );
    let denied =
            value["denied_by_minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_boundary"]
                .as_array()
                .expect("minimal scoped Memory durable store write receipt acceptance denials");
    assert_eq!(denied.len(), 32);
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "run_minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_boundary_require_live_gate"
    );
    assert_eq!(
        value["allowed_next_actions"][0]["accepts_single_shot_receipt"],
        true
    );
    assert_eq!(
        value["allowed_next_actions"][0]["writes_new_canary_store_record"],
        false
    );
    assert_eq!(
        value["allowed_next_actions"][0]["writes_production_durable_memory"],
        false
    );
    assert_eq!(
        value["allowed_next_actions"][1]["action"],
        "prepare_minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_boundary"
    );
    assert_eq!(
        value["allowed_next_actions"][1]["requires_minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_boundary"],
        true
    );
    let side_effects = value["side_effects"]
        .as_object()
        .expect("minimal scoped Memory durable store write receipt acceptance side effects");
    assert_eq!(
        side_effects["durable_store_write_receipt_acceptance_performed"].as_bool(),
        Some(true)
    );
    assert_eq!(
        side_effects["durable_store_write_receipt_acceptance_result_accepted"].as_bool(),
        Some(true)
    );
    assert_eq!(
        side_effects["memory_store_write_performed"].as_bool(),
        Some(false)
    );
    assert_eq!(side_effects["wal_write_performed"].as_bool(), Some(false));
    assert_eq!(side_effects["receipt_persisted"].as_bool(), Some(false));
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
fn hepta_memory_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_accepts_cleanup_evidence_without_new_memory_or_external_side_effects()
 {
    let body = route_contract_body(HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_MINIMAL_SCOPED_MEMORY_REAL_WRITE_CANARY_DURABLE_STORE_WRITE_ROLLBACK_TOMBSTONE_ZERO_RESIDUE_ACCEPTANCE_BOUNDARY_ENDPOINT);

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "minimal scoped Memory real-write canary durable store write rollback/tombstone zero-residue acceptance json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_MINIMAL_SCOPED_MEMORY_REAL_WRITE_CANARY_DURABLE_STORE_WRITE_ROLLBACK_TOMBSTONE_ZERO_RESIDUE_ACCEPTANCE_BOUNDARY_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-rollback-tombstone-zero-residue-acceptance-boundary --json"
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
        value["memory_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_boundary_ready"],
        true
    );
    assert_eq!(
        value["minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_ready"],
        true
    );
    assert_eq!(
        value["minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_performed"],
        true
    );
    assert_eq!(
        value["minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_accepted"],
        true
    );
    assert_eq!(
        value["scoped_memory_real_write_canary_mode"],
        "minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_report_only"
    );
    assert_eq!(
        value["source_minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_boundary_ready"],
        true
    );
    assert_eq!(
        value["source_accepted_minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_fixture_count"],
        1
    );
    assert_eq!(
        value["source_blocked_minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_fixture_count"],
        9
    );
    assert_eq!(value["source_receipt_acceptance_result_accepted_count"], 1);
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
    assert_eq!(value["approved_namespace"], "hepta.memory.canary");
    assert_eq!(value["approved_store"], "wal-receipt-canary-artifact");
    assert_eq!(value["approved_scope"], "session");
    assert_eq!(
        value["durable_store_write_target_id"],
        "hepta-minimal-scoped-memory-real-write-canary-durable-store-write-target-v1"
    );
    assert_eq!(
        value["durable_store_target_store_id"],
        "hepta-memory-durable-store-canary-plan-only"
    );
    assert_eq!(
        value["canary_record_id"],
        "hepta-minimal-scoped-memory-real-write-canary-durable-store-write-single-shot-record-v1"
    );
    for key in [
        "source_receipt_acceptance_boundary_hash_sha256",
        "source_receipt_acceptance_policy_hash_sha256",
        "canary_payload_digest_sha256",
        "single_shot_receipt_hash_sha256",
        "single_shot_receipt_hash_chain_sha256",
        "single_shot_cleanup_receipt_hash_sha256",
        "single_shot_execution_hash_sha256",
        "receipt_acceptance_record_hash_sha256",
        "receipt_acceptance_readback_hash_sha256",
        "receipt_acceptance_hash_sha256",
        "zero_residue_acceptance_record_hash_sha256",
        "zero_residue_acceptance_readback_hash_sha256",
        "zero_residue_acceptance_hash_sha256",
        "minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_boundary_hash_sha256",
        "minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_policy_hash_sha256",
    ] {
        assert_ne!(
            value[key], "",
            "rollback/tombstone zero-residue acceptance hash should be present: {key}"
        );
    }
    assert_eq!(
        value["source_single_shot_canary_post_write_memory_count"],
        1
    );
    assert_eq!(value["source_single_shot_canary_readback_hit_count"], 1);
    assert_eq!(value["source_single_shot_canary_rollback_restored"], true);
    assert_eq!(
        value["source_single_shot_canary_post_rollback_memory_count"],
        0
    );
    assert_eq!(
        value["source_single_shot_canary_post_rollback_absence_confirmed"],
        true
    );
    assert_eq!(value["source_single_shot_canary_artifact_write_count"], 3);
    assert_eq!(
        value["source_single_shot_canary_artifact_readback_count"],
        3
    );
    assert_eq!(
        value["source_single_shot_canary_artifact_cleanup_removed_count"],
        3
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
        value["single_shot_rollback_cleanup_zero_residue_verified"],
        true
    );
    assert_eq!(value["rollback_tombstone_cleanup_absence_verified"], true);
    assert_eq!(value["artifact_zero_residue_verified"], true);
    assert_eq!(
        value["required_minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_surface_count"],
        12
    );
    assert_eq!(
        value["ready_minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_surface_count"],
        12
    );
    assert_eq!(
        value["minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_fixture_count"],
        10
    );
    assert_eq!(
        value["accepted_minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_fixture_count"],
        1
    );
    assert_eq!(
        value["blocked_minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_fixture_count"],
        9
    );
    assert_eq!(
        value["denied_by_minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_boundary_count"],
        34
    );
    for key in [
        "minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_accepted_count",
        "durable_store_write_rollback_tombstone_zero_residue_acceptance_authority_accepted_count",
        "source_receipt_acceptance_boundary_bound_count",
        "source_receipt_acceptance_hash_bound_count",
        "single_shot_rollback_cleanup_zero_residue_bound_count",
        "single_shot_artifact_zero_residue_bound_count",
        "rollback_tombstone_cleanup_absence_bound_count",
        "zero_residue_acceptance_record_bound_count",
        "zero_residue_acceptance_result_recorded_count",
        "zero_residue_acceptance_result_accepted_count",
        "zero_residue_acceptance_replay_guard_accepted_count",
        "operator_zero_residue_acceptance_handoff_bound_count",
    ] {
        assert_eq!(
            value[key], 1,
            "rollback/tombstone zero-residue acceptance count should be one: {key}"
        );
    }
    for key in [
        "durable_store_write_execution_performed_count",
        "durable_store_write_single_shot_execution_performed_count",
        "durable_store_write_receipt_acceptance_performed_count",
        "memory_store_write_performed_count",
        "wal_write_performed_count",
        "receipt_persisted_count",
        "post_write_readback_performed_count",
        "rollback_performed_count",
        "tombstone_cleanup_executed_count",
        "durable_memory_store_read_performed_count",
        "durable_memory_store_write_performed_count",
        "durable_memory_store_rollback_performed_count",
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
            "rollback/tombstone zero-residue acceptance side-effect count should stay zero: {key}"
        );
    }
    for key in [
        "durable_store_write_rollback_tombstone_zero_residue_acceptance_performed",
        "durable_store_write_rollback_tombstone_zero_residue_acceptance_result_recorded",
        "durable_store_write_rollback_tombstone_zero_residue_acceptance_result_accepted",
        "source_receipt_acceptance_boundary_accepted",
        "single_shot_rollback_cleanup_zero_residue_evidence_accepted",
        "single_shot_artifact_zero_residue_evidence_accepted",
        "rollback_tombstone_cleanup_absence_accepted",
        "zero_residue_acceptance_recorded",
        "zero_residue_acceptance_replay_guard_accepted",
        "operator_zero_residue_acceptance_handoff_bound",
        "source_receipt_acceptance_boundary_required",
        "approved_namespace_bound",
        "approved_store_bound",
        "approved_scope_bound",
        "receipt_acceptance_hash_bound",
        "single_shot_rollback_cleanup_zero_residue_bound",
        "single_shot_artifact_zero_residue_bound",
        "rollback_tombstone_cleanup_absence_bound",
        "zero_residue_acceptance_record_bound",
        "new_canary_store_write_forbidden_on_report_route",
        "rollback_execution_forbidden_on_report_route",
        "tombstone_write_forbidden_on_report_route",
        "kg_provider_channel_release_install_active_binary_forbidden",
    ] {
        assert_eq!(
            value[key], true,
            "rollback/tombstone zero-residue acceptance field should be true: {key}"
        );
    }
    for key in [
        "durable_store_write_execution_performed",
        "durable_store_write_single_shot_execution_performed",
        "durable_store_write_receipt_acceptance_performed",
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
        "readback_result_accepted",
        "rollback_executed",
        "rollback_performed",
        "tombstone_write_performed",
        "tombstone_cleanup_executed",
        "production_durable_memory_store_write_performed",
        "actual_production_durable_memory_write_performed",
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
            "rollback/tombstone zero-residue acceptance external or Memory field should stay false: {key}"
        );
    }
    let fixtures =
            value["minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_fixtures"]
                .as_array()
                .expect("minimal scoped Memory durable store write rollback/tombstone zero-residue acceptance fixtures");
    assert_eq!(fixtures.len(), 10);
    assert_eq!(
            fixtures
                .iter()
                .filter(|fixture| {
                    fixture["minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_accepted"]
                        == true
                })
                .count(),
            1
        );
    let denied =
            value["denied_by_minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_boundary"]
                .as_array()
                .expect("minimal scoped Memory durable store write rollback/tombstone zero-residue acceptance denials");
    assert_eq!(denied.len(), 34);
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "run_minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_boundary_require_live_gate"
    );
    assert_eq!(
        value["allowed_next_actions"][0]["accepts_zero_residue_evidence"],
        true
    );
    assert_eq!(
        value["allowed_next_actions"][0]["writes_new_canary_store_record"],
        false
    );
    assert_eq!(value["allowed_next_actions"][0]["executes_rollback"], false);
    assert_eq!(
        value["allowed_next_actions"][1]["action"],
        "prepare_scoped_production_durable_memory_write_preflight_boundary"
    );
    assert_eq!(
        value["allowed_next_actions"][1]["requires_minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_boundary"],
        true
    );
    let side_effects = value["side_effects"]
            .as_object()
            .expect("minimal scoped Memory durable store write rollback/tombstone zero-residue acceptance side effects");
    assert_eq!(
        side_effects["durable_store_write_rollback_tombstone_zero_residue_acceptance_performed"]
            .as_bool(),
        Some(true)
    );
    assert_eq!(
            side_effects["durable_store_write_rollback_tombstone_zero_residue_acceptance_result_accepted"].as_bool(),
            Some(true)
        );
    assert_eq!(
        side_effects["memory_store_write_performed"].as_bool(),
        Some(false)
    );
    assert_eq!(side_effects["wal_write_performed"].as_bool(), Some(false));
    assert_eq!(side_effects["receipt_persisted"].as_bool(), Some(false));
    assert_eq!(side_effects["rollback_executed"].as_bool(), Some(false));
    assert_eq!(
        side_effects["tombstone_cleanup_executed"].as_bool(),
        Some(false)
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
fn hepta_upstream_codex_latest_multisurface_absorption_endpoint_classifies_without_fetch_merge_or_activation_side_effects()
 {
    let body = route_contract_body(HEPTA_UPSTREAM_CODEX_LATEST_MULTISURFACE_ABSORPTION_ENDPOINT);

    let value: serde_json::Value = serde_json::from_str(&body)
        .expect("upstream Codex latest multisurface absorption native route json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
        value["endpoint"],
        HEPTA_UPSTREAM_CODEX_LATEST_MULTISURFACE_ABSORPTION_ENDPOINT
    );
    assert_eq!(
        value["source_command"],
        "/hepta-upstream-codex-latest-multisurface-absorption --json"
    );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(
        value["route_count"],
        serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
    );
    assert_eq!(
        value["implemented_route_count"],
        serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(
        value["native_route_mode"],
        "native_route_latest_upstream_delta_classification_no_fetch_no_merge_no_activation"
    );
    assert_eq!(
        value["baseline_upstream_head"],
        "9f42c89c0112771dc29100a6f3fc904049b2655f"
    );
    assert_eq!(
        value["target_upstream_head"],
        "8a94430bb273623be42b68f144f1ab1df343bb53"
    );
    assert_eq!(value["target_ref"], "refs/remotes/openai-codex/latest");
    assert_eq!(
        value["latest_multisurface_decision"],
        "classified_as_oracle_only_without_merge_rebase_or_active_wiring"
    );
    assert_eq!(value["commit_count"], 12);
    assert_eq!(value["changed_file_count"], 57);
    assert_eq!(value["provider_security_changed_file_count"], 0);
    assert_eq!(value["runtime_appserver_changed_file_count"], 11);
    assert_eq!(value["legacy_cli_tui_changed_file_count"], 47);
    assert_eq!(value["product_governance_changed_file_count"], 2);
    assert_eq!(value["populated_bucket_count"], 3);
    assert_eq!(value["all_buckets_populated"], false);
    assert_eq!(value["family_count"], 5);
    assert_eq!(value["ready_family_count"], 5);
    assert_eq!(value["activation_blocking_family_count"], 5);

    let families = value["family_inventory"]
        .as_array()
        .expect("latest multisurface family inventory");
    assert_eq!(families.len(), 5);
    assert!(
        families
            .iter()
            .all(|family| family["ready"] == true && family["promotion_allowed"] == false)
    );
    assert_eq!(families[0]["id"], "doctor-thread-inventory-audit");
    assert_eq!(families[1]["id"], "appserver-remote-status");
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "run_upstream_codex_latest_active_safety_regression_gate"
    );
    for key in [
        "active_runtime_promotion_allowed",
        "active_appserver_promotion_allowed",
        "active_tui_promotion_allowed",
        "active_process_hardening_env_mutation_allowed",
        "upstream_fetch_performed_by_native_route",
        "upstream_fetch_performed_by_gate",
        "upstream_merge_performed",
        "upstream_checkout_performed",
        "active_runtime_auto_rebase_allowed",
        "active_runtime_dependency_allowed",
        "active_binary_mutation_allowed",
        "active_service_restart_allowed",
        "launchd_mutation_allowed",
        "provider_model_invocation_allowed",
        "channel_delivery_allowed",
        "public_release_claim_allowed",
        "public_ga_claim_allowed",
        "release_artifact_write_allowed",
        "evidence_persistence_allowed",
    ] {
        assert_eq!(value[key], false, "{key}");
    }
    assert_eq!(value["latest_multisurface_denied_by_count"], 13);
    let denied = value["denied_by_latest_multisurface_absorption"]
        .as_array()
        .expect("latest multisurface denials");
    assert_eq!(denied.len(), 13);

    let side_effects = value["side_effects"]
        .as_object()
        .expect("latest multisurface native route side effects");
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

mod first_model_invocation_tests {
    use super::*;

    include!("first_model_invocation.rs");
}
mod runtime_ingress_tests {
    use super::*;
    include!("runtime_ingress.rs");
}
#[derive(Debug)]
struct RuntimeProviderRouterDenialCase {
    endpoint: &'static str,
    source_command: &'static str,
    route_stem: &'static str,
    schema_key: &'static str,
    schema_version: &'static str,
    expected_sha256: &'static str,
}

fn assert_runtime_provider_router_denial_case(case: RuntimeProviderRouterDenialCase) {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request("GET", case.endpoint, &options);
    assert_eq!(status, "200 OK", "{case:?}");
    assert_eq!(content_type, "application/json; charset=utf-8", "{case:?}");

    let mut value: serde_json::Value =
        serde_json::from_str(&body).expect("runtime provider-router denial route json");
    assert_eq!(value["runtime"], "hepta", "{case:?}");
    assert_eq!(value["status"], "ready", "{case:?}");
    assert_eq!(value["endpoint"], case.endpoint, "{case:?}");
    assert_eq!(value["source_command"], case.source_command, "{case:?}");
    assert_eq!(
        value["native_gateway_source_command_count"], NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "{case:?}"
    );
    assert_eq!(
        value["route_count"], NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "{case:?}"
    );
    assert_eq!(
        value["implemented_route_count"], NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "{case:?}"
    );
    assert_eq!(value["missing_route_count"], 0, "{case:?}");
    assert_eq!(
        value["route_count_source_command_accepted"], true,
        "{case:?}"
    );

    let route_enabled_key = format!("{}_route_enabled", case.route_stem);
    let route_ready_key = format!("{}_ready", case.route_stem);
    let route_status_key = format!("{}_status", case.route_stem);
    assert_eq!(value[&route_enabled_key], true, "{case:?}");
    assert_eq!(value[&route_ready_key], true, "{case:?}");
    assert_eq!(value[&route_status_key], "blocked", "{case:?}");
    assert_eq!(value[case.schema_key], case.schema_version, "{case:?}");

    let side_effects = value["side_effects"]
        .as_object()
        .expect("runtime provider-router denial side effects");
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false)),
        "{case:?}"
    );

    // These values intentionally follow the global route registry. Assert them above, then
    // normalize them so a new unrelated route does not churn every denial snapshot.
    for key in [
        "native_gateway_source_command_count",
        "route_count",
        "implemented_route_count",
    ] {
        value[key] = serde_json::json!(0);
    }
    let normalized =
        serde_json::to_vec(&value).expect("serialize runtime provider-router denial snapshot");
    let actual_sha256 = format!("{:x}", Sha256::digest(&normalized));
    assert!(
        !case.expected_sha256.is_empty(),
        "record runtime provider-router denial snapshot for {}: {}",
        case.endpoint,
        actual_sha256
    );
    assert_eq!(actual_sha256, case.expected_sha256, "{case:?}");
}

macro_rules! runtime_provider_router_denial_case {
    (
                $endpoint:expr,
                $source_command:expr,
                $route_stem:expr,
                $schema_key:expr,
                $schema_version:expr,
                $expected_sha256:expr
            ) => {
        assert_runtime_provider_router_denial_case(RuntimeProviderRouterDenialCase {
            endpoint: $endpoint,
            source_command: $source_command,
            route_stem: $route_stem,
            schema_key: $schema_key,
            schema_version: $schema_version,
            expected_sha256: $expected_sha256,
        })
    };
}

#[test]
fn hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_operator_acknowledgement_non_acceptance_endpoint_blocks_acknowledgement_side_effects()
 {
    runtime_provider_router_denial_case!(
                HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_OPERATOR_ACKNOWLEDGEMENT_NON_ACCEPTANCE_ENDPOINT,
                "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-acknowledgement-non-acceptance --json",
                "runtime_provider_router_operator_acknowledgement_non_acceptance",
                "operator_acknowledgement_non_acceptance_schema_version",
                "memory_intelligence_kg_full_enablement_runtime_provider_router_operator_acknowledgement_non_acceptance_v1",
                "67f9f0da3cb43af77e371223aa436d8f704c055a4cc916e08691b4189197021f"
            );
}

#[test]
fn hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_request_denial_matrix_endpoint_blocks_activation_requests()
 {
    runtime_provider_router_denial_case!(
                HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_REQUEST_DENIAL_MATRIX_ENDPOINT,
                "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-request-denial-matrix --json",
                "runtime_provider_router_activation_request_denial_matrix",
                "activation_request_denial_matrix_schema_version",
                "memory_intelligence_kg_full_enablement_runtime_provider_router_activation_request_denial_matrix_v1",
                "2681b2505a25995e414e4063e85ce644e79c67f31ea61b273e1b8fd89b89b4ab"
            );
}

#[test]
fn hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_noop_handoff_endpoint_blocks_activation_commands()
 {
    runtime_provider_router_denial_case!(
                HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_NOOP_HANDOFF_ENDPOINT,
                "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-noop-handoff --json",
                "runtime_provider_router_activation_command_noop_handoff",
                "activation_command_noop_handoff_schema_version",
                "memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_noop_handoff_v1",
                "e71f3c6b1b55f02a98abe1a07c937f85383060df4f1f919814f016a2b3d56100"
            );
}

#[test]
fn hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_no_persistence_endpoint_blocks_receipts()
 {
    runtime_provider_router_denial_case!(
                HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_NO_PERSISTENCE_ENDPOINT,
                "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-no-persistence --json",
                "runtime_provider_router_activation_command_result_receipt_no_persistence",
                "activation_command_result_receipt_no_persistence_schema_version",
                "memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_no_persistence_v1",
                "d501c4a69311acff945748e16f2bc05b4e75d4361f3533b95c177780431ba8ab"
            );
}

#[test]
fn hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_replay_idempotency_endpoint_blocks_replay()
 {
    runtime_provider_router_denial_case!(
                HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_REPLAY_IDEMPOTENCY_DENIAL_ENDPOINT,
                "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-replay-idempotency-denial --json",
                "runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial",
                "activation_command_result_receipt_replay_idempotency_schema_version",
                "memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_v1",
                "911b3b212b816e14b01e4bd0e48152f4062a8ff8407a8bbb6d30e28a321ee3fd"
            );
}

#[test]
fn hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_endpoint_blocks_ordering()
 {
    runtime_provider_router_denial_case!(
                HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_ORDERING_MONOTONICITY_DENIAL_ENDPOINT,
                "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-ordering-monotonicity-denial --json",
                "runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial",
                "activation_command_result_receipt_ordering_monotonicity_schema_version",
                "memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_v1",
                "12255fcbf7366dc01081a7748490ba3a9c01ec7b4974ba320ce10fa101e17e1b"
            );
}

#[test]
fn hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_cancellation_supersession_endpoint_blocks_lifecycle()
 {
    runtime_provider_router_denial_case!(
                HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_CANCELLATION_SUPERSESSION_DENIAL_ENDPOINT,
                "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-cancellation-supersession-denial --json",
                "runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial",
                "activation_command_result_receipt_cancellation_supersession_schema_version",
                "memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_v1",
                "c5e18818820a1f85decdf119143c4ae3138826211a3e063362efe8b014a59e7d"
            );
}

#[test]
fn hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_endpoint_blocks_evidence()
 {
    runtime_provider_router_denial_case!(
                HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_AUDIT_TRAIL_IMMUTABLE_EVIDENCE_DENIAL_ENDPOINT,
                "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-audit-trail-immutable-evidence-denial --json",
                "runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial",
                "activation_command_result_receipt_audit_trail_immutable_evidence_schema_version",
                "memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_v1",
                "3bd273403a8decdc37338c9676ad488bdd4d3619f67a48841536cbbe2c37ce2b"
            );
}

#[test]
fn hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_endpoint_blocks_lifecycle()
 {
    runtime_provider_router_denial_case!(
                HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_RETENTION_EXPIRY_GARBAGE_COLLECTION_DENIAL_ENDPOINT,
                "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-retention-expiry-garbage-collection-denial --json",
                "runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial",
                "activation_command_result_receipt_retention_expiry_garbage_collection_schema_version",
                "memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_v1",
                "cc2fd7e79fb7e67800c48a13c08f5b150946e726c53b55b256257f043372c6c6"
            );
}

#[test]
fn hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_export_query_observability_endpoint_blocks_reporting_surfaces()
 {
    runtime_provider_router_denial_case!(
                HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_EXPORT_QUERY_OBSERVABILITY_DENIAL_ENDPOINT,
                "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-export-query-observability-denial --json",
                "runtime_provider_router_activation_command_result_receipt_export_query_observability_denial",
                "activation_command_result_receipt_export_query_observability_schema_version",
                "memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_v1",
                "944df505c979a0f9fd7d02f4c1646b547c7ba07d4732f8eb058e06605a5ace5d"
            );
}

#[test]
fn hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_operator_summary_briefing_endpoint_blocks_delivery_and_authority()
 {
    runtime_provider_router_denial_case!(
                HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_OPERATOR_FACING_SUMMARY_BRIEFING_NON_PERSISTENCE_DENIAL_ENDPOINT,
                "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-operator-facing-summary-briefing-non-persistence-denial --json",
                "runtime_provider_router_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial",
                "activation_command_result_receipt_operator_facing_summary_briefing_schema_version",
                "memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_v1",
                "da94c8c920da98d3832c00bb6e115dd6badc44b7beb7a810270c11f13ab83177"
            );
}

#[test]
fn hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_final_operator_acknowledgement_endpoint_blocks_acceptance_and_authority()
 {
    runtime_provider_router_denial_case!(
                HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_FINAL_OPERATOR_ACKNOWLEDGEMENT_NON_ACCEPTANCE_DENIAL_ENDPOINT,
                "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-final-operator-acknowledgement-non-acceptance-denial --json",
                "runtime_provider_router_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial",
                "activation_command_result_receipt_final_operator_acknowledgement_schema_version",
                "memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_v1",
                "38d069e7ba21000ca35bc3e7c5eadf1791d395c799949447b1ee369ddcdd58b0"
            );
}

#[test]
fn hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_terminal_operator_decision_public_claim_endpoint_blocks_public_claim_and_authority()
 {
    runtime_provider_router_denial_case!(
                HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_TERMINAL_OPERATOR_DECISION_PUBLIC_CLAIM_NON_PROMOTION_DENIAL_ENDPOINT,
                "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial --json",
                "runtime_provider_router_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial",
                "activation_command_result_receipt_terminal_operator_decision_public_claim_schema_version",
                "memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_v1",
                "0f2ad1204d6e20472c8df8995ffdd159a4f1808a416fbfd0573b4ca081d94678"
            );
}

#[test]
fn hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_release_artifact_publication_endpoint_blocks_publication_and_authority()
 {
    runtime_provider_router_denial_case!(
                HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_RELEASE_ARTIFACT_PUBLICATION_DENIAL_ENDPOINT,
                "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-release-artifact-publication-denial --json",
                "runtime_provider_router_activation_command_result_receipt_release_artifact_publication_denial",
                "activation_command_result_receipt_release_artifact_publication_schema_version",
                "memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_release_artifact_publication_denial_v1",
                "5988a393cd31bfe4abccfd8d854e315cfbbd39e68a41006103ccda57e94475e5"
            );
}
#[test]
fn hepta_release_hardening_status_gate_endpoint_is_local_only() {
    let body = route_contract_body(HEPTA_RELEASE_HARDENING_STATUS_GATE_ENDPOINT);

    let value: serde_json::Value =
        serde_json::from_str(&body).expect("release hardening status gate json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(
        value["source_command"],
        "/hepta-release-hardening-status-gate --json"
    );
    assert_eq!(
        value["compatibility_mode"],
        "native_release_hardening_status_gate_inventory"
    );
    assert_eq!(value["old_release_hardening_script_family_count"], 12);
    assert_eq!(
        value["current_hepta_codex_script_total"],
        CURRENT_HEPTA_CODEX_SCRIPT_TOTAL
    );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["status_gate_count"], 12);
    assert_eq!(value["local_status_gate_ready_count"], 12);
    assert_eq!(value["live_execution_enabled_count"], 0);
    assert_eq!(value["external_production_gate_count"], 3);
    assert_eq!(value["launchd_mutation_required_count"], 3);
    assert_eq!(value["filesystem_artifact_write_required_count"], 2);
    assert_eq!(value["operator_approval_required_count"], 12);
    assert_eq!(value["release_hardening_status_gate_ready"], true);
    assert_eq!(value["old_script_execution_compatibility_claimed"], true);
    assert_eq!(value["external_production_gate_enabled"], false);
    assert_eq!(value["release_artifact_pack_enabled"], false);
    assert_eq!(value["launchd_service_mutation_enabled"], false);
    assert_eq!(value["recurring_watchdog_install_enabled"], false);
    assert_eq!(value["local_import_execution_enabled"], false);
    assert_eq!(value["autonomous_subagent_spawn_enabled"], false);
    assert_eq!(
        value["script_inventory_script"],
        "scripts/hepta-release-hardening-status-gate.sh"
    );
    let gates = value["release_hardening_gates"]
        .as_array()
        .expect("release hardening gates");
    assert_eq!(gates.len(), 12);
    assert_eq!(gates[0]["name"], "gateway-service");
    assert_eq!(gates[3]["name"], "external-production-gates");
    assert_eq!(gates[9]["name"], "local-import");
    assert_eq!(gates[11]["name"], "autonomous-coding-subagent");
    assert_eq!(value["side_effects"]["process_spawned"], false);
    assert_eq!(value["side_effects"]["filesystem_read"], false);
    assert_eq!(value["side_effects"]["filesystem_written"], false);
    assert_eq!(value["side_effects"]["release_artifact_written"], false);
    assert_eq!(value["side_effects"]["launchd_mutated"], false);
    assert_eq!(value["side_effects"]["watchdog_service_installed"], false);
    assert_eq!(value["side_effects"]["external_network_read"], false);
    assert_eq!(value["side_effects"]["external_send_performed"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["model_invoked"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(
        value["side_effects"]["telegram_owner_handoff_performed"],
        false
    );
    assert_eq!(value["side_effects"]["telegram_read_performed"], false);
    assert_eq!(value["side_effects"]["telegram_send_performed"], false);
    assert_eq!(
        value["side_effects"]["native_post_mutation_performed"],
        false
    );
    assert_eq!(value["side_effects"]["channel_read_performed"], false);
    assert_eq!(value["side_effects"]["channel_send_performed"], false);
    assert_eq!(value["side_effects"]["coding_agent_spawned"], false);
    assert_eq!(value["side_effects"]["gateway_mutation_performed"], false);
    let blockers = value["blockers"]
        .as_array()
        .expect("blockers")
        .iter()
        .filter_map(|item| item.as_str())
        .collect::<Vec<_>>();
    assert!(blockers.contains(&"release_artifact_pack_not_operator_approved"));
    assert!(blockers.contains(&"external_production_gate_not_operator_approved"));
    assert!(blockers.contains(&"launchd_service_mutation_not_operator_approved"));
    assert!(blockers.contains(&"autonomous_subagent_spawn_not_operator_approved"));
}
