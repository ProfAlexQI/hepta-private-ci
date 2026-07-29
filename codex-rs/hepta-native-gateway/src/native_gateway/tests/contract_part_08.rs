#[test]
fn hepta_memory_write_execution_activation_command_noop_handoff_boundary_endpoint_blocks_command_handoff_without_activation()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_COMMAND_NOOP_HANDOFF_BOUNDARY_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body)
        .expect("memory write execution activation command no-op handoff boundary json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_COMMAND_NOOP_HANDOFF_BOUNDARY_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-live-mutation-operator-write-execution-activation-command-no-op-handoff-boundary --json"
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
        value["memory_write_execution_activation_command_noop_handoff_boundary_ready"],
        true
    );
    assert_eq!(
        value["activation_command_noop_handoff_mode"],
        "memory_write_execution_activation_command_noop_handoff_denial"
    );
    assert_eq!(
        value["source_memory_write_execution_activation_closure_denial_boundary_ready"],
        true
    );
    assert_eq!(
        value["source_memory_write_execution_activation_closure_denial_ready"],
        true
    );
    assert_eq!(value["minimum_required_samples"], 24);
    assert_eq!(
        value["memory_write_execution_activation_command_noop_handoff_ready"],
        true
    );
    assert_eq!(
        value["memory_write_execution_activation_closure_denial_ready"],
        true
    );
    assert_eq!(value["required_activation_closure_surface_count"], 12);
    assert_eq!(value["ready_activation_closure_surface_count"], 12);
    assert_eq!(
        value["required_activation_command_handoff_surface_count"],
        13
    );
    assert_eq!(value["ready_activation_command_handoff_surface_count"], 13);
    assert_eq!(
        value["side_effect_free_activation_command_handoff_surface_count"],
        13
    );
    assert_eq!(value["required_activation_command_fixture_count"], 10);
    assert_eq!(value["activation_command_fixture_count"], 10);
    assert_eq!(value["blocked_activation_command_fixture_count"], 10);
    assert_eq!(value["noop_activation_command_fixture_count"], 10);
    assert_eq!(value["allowed_activation_command_fixture_count"], 0);
    assert_eq!(value["accepted_activation_command_fixture_count"], 0);
    assert_eq!(value["activation_command_denied_count"], 10);
    assert_eq!(value["activation_command_performed_count"], 0);

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
        "rollback_execution_allowed",
        "rollback_executed",
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
        assert_eq!(value[key], false, "{key}");
    }
    assert_eq!(value["memory_store_write_performed_count"], 0);

    let surfaces = value["activation_command_handoff_surfaces"]
        .as_array()
        .expect("activation command handoff surfaces");
    assert_eq!(surfaces.len(), 13);
    let fixtures = value["activation_command_fixtures"]
        .as_array()
        .expect("activation command fixtures");
    assert_eq!(fixtures.len(), 10);
    assert!(fixtures.iter().all(|fixture| {
        fixture["activation_command_requested"].as_bool() == Some(true)
            && fixture["command_status"].as_str() == Some("blocked_noop")
            && fixture["command_allowed"].as_bool() == Some(false)
            && fixture["command_invoked"].as_bool() == Some(false)
            && fixture["command_dispatched"].as_bool() == Some(false)
            && fixture["command_noop_confirmed"].as_bool() == Some(true)
            && fixture["handoff_recorded"].as_bool() == Some(false)
            && fixture["handoff_persisted"].as_bool() == Some(false)
            && fixture["activation_allowed"].as_bool() == Some(false)
            && fixture["live_mutation_execution_performed"].as_bool() == Some(false)
            && fixture["memory_store_write_performed"].as_bool() == Some(false)
            && fixture["memory_store_mutated"].as_bool() == Some(false)
            && fixture["rollback_executed"].as_bool() == Some(false)
    }));
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["command_invocation_attempted"] == true)
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["memory_store_write_path_enable_requested"] == true
                    && fixture["direct_memory_store_write_requested"] == true
            })
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["raw_payload_plaintext_recorded"] == true
                    && fixture["secret_material_read"] == true
            })
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["external_send_requested"] == true
                    && fixture["release_artifact_write_requested"] == true
            })
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["install_requested"] == true
                    && fixture["launchd_restart_requested"] == true
                    && fixture["active_binary_mutation_requested"] == true
            })
            .count(),
        1
    );
    assert_eq!(value["denied_by_activation_command_handoff_count"], 26);
    assert_eq!(
        value["denied_by_activation_command_handoff"]
            .as_array()
            .expect("activation command handoff denials")
            .len(),
        26
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "run_memory_write_execution_activation_command_noop_handoff_boundary_require_live_gate"
    );
    assert_eq!(value["allowed_next_actions"][0]["writes_memory"], false);
    assert_eq!(
        value["allowed_next_actions"][1]["action"],
        "prepare_memory_write_execution_activation_command_result_receipt_no_persistence_boundary"
    );
    let side_effects = value["side_effects"]
        .as_object()
        .expect("memory write execution activation command no-op handoff boundary side effects");
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_write_execution_activation_command_result_receipt_no_persistence_boundary_endpoint_blocks_receipt_persistence()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_COMMAND_RESULT_RECEIPT_NO_PERSISTENCE_BOUNDARY_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect(
        "memory write execution activation command result receipt no-persistence boundary json",
    );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_COMMAND_RESULT_RECEIPT_NO_PERSISTENCE_BOUNDARY_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-no-persistence-boundary --json"
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
        value["memory_write_execution_activation_command_result_receipt_no_persistence_boundary_ready"],
        true
    );
    assert_eq!(
        value["activation_command_result_receipt_no_persistence_mode"],
        "memory_write_execution_activation_command_result_receipt_no_persistence_denial"
    );
    assert_eq!(
        value["source_activation_command_noop_handoff_boundary_ready"],
        true
    );
    assert_eq!(value["source_activation_command_noop_handoff_ready"], true);
    assert_eq!(
        value["memory_write_execution_activation_command_result_receipt_no_persistence_ready"],
        true
    );
    assert_eq!(
        value["memory_write_execution_activation_command_noop_handoff_ready"],
        true
    );
    assert_eq!(
        value["required_activation_command_handoff_surface_count"],
        13
    );
    assert_eq!(value["ready_activation_command_handoff_surface_count"], 13);
    assert_eq!(
        value["required_activation_command_result_receipt_surface_count"],
        12
    );
    assert_eq!(
        value["ready_activation_command_result_receipt_surface_count"],
        12
    );
    assert_eq!(
        value["side_effect_free_activation_command_result_receipt_surface_count"],
        12
    );
    assert_eq!(
        value["required_activation_command_result_receipt_fixture_count"],
        10
    );
    assert_eq!(value["activation_command_result_receipt_fixture_count"], 10);
    assert_eq!(
        value["blocked_activation_command_result_receipt_fixture_count"],
        10
    );
    assert_eq!(
        value["noop_activation_command_result_receipt_fixture_count"],
        10
    );
    assert_eq!(
        value["allowed_activation_command_result_receipt_fixture_count"],
        0
    );
    assert_eq!(
        value["accepted_activation_command_result_receipt_fixture_count"],
        0
    );
    assert_eq!(value["activation_command_result_receipt_denied_count"], 10);
    assert_eq!(
        value["activation_command_result_receipt_performed_count"],
        0
    );

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
        "activation_command_completion_ack_recorded",
        "activation_command_completion_ack_persisted",
        "activation_command_completion_ack_accepted",
        "activation_command_enabled",
        "activation_command_invoked",
        "activation_command_dispatched",
        "activation_command_handoff_recorded",
        "activation_command_handoff_persisted",
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
        "secret_material_read",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "kg_adapter_read_performed",
        "live_kg_write_performed",
        "channel_send_performed",
        "telegram_send_performed",
        "external_send_performed",
        "public_release_published",
        "public_ga_claimed",
        "release_artifact_written",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
    ] {
        assert_eq!(value[key], false, "{key}");
    }
    assert_eq!(value["memory_store_write_performed_count"], 0);

    let surfaces = value["activation_command_result_receipt_surfaces"]
        .as_array()
        .expect("activation command result receipt surfaces");
    assert_eq!(surfaces.len(), 12);
    let fixtures = value["activation_command_result_receipt_fixtures"]
        .as_array()
        .expect("activation command result receipt fixtures");
    assert_eq!(fixtures.len(), 10);
    assert!(fixtures.iter().all(|fixture| {
        fixture["receipt_requested"].as_bool() == Some(true)
            && fixture["receipt_status"].as_str() == Some("blocked_noop")
            && fixture["receipt_allowed"].as_bool() == Some(false)
            && fixture["receipt_recorded"].as_bool() == Some(false)
            && fixture["receipt_persisted"].as_bool() == Some(false)
            && fixture["receipt_accepted"].as_bool() == Some(false)
            && fixture["receipt_materialized"].as_bool() == Some(false)
            && fixture["receipt_filesystem_written"].as_bool() == Some(false)
            && fixture["receipt_noop_confirmed"].as_bool() == Some(true)
            && fixture["completion_ack_recorded"].as_bool() == Some(false)
            && fixture["activation_allowed"].as_bool() == Some(false)
            && fixture["live_mutation_execution_performed"].as_bool() == Some(false)
            && fixture["memory_store_write_performed"].as_bool() == Some(false)
            && fixture["memory_store_mutated"].as_bool() == Some(false)
            && fixture["rollback_executed"].as_bool() == Some(false)
    }));
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["receipt_record_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["receipt_persist_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["receipt_materialize_requested"] == true
                    && fixture["receipt_filesystem_write_requested"] == true
            })
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["receipt_ledger_write_requested"] == true
                    && fixture["receipt_index_requested"] == true
                    && fixture["receipt_delivery_requested"] == true
            })
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["completion_ack_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["receipt_status_requested"] == "completed")
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["memory_store_write_requested"] == true
                    && fixture["rollback_execution_requested"] == true
            })
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["external_send_requested"] == true
                    && fixture["install_requested"] == true
                    && fixture["active_binary_mutation_requested"] == true
            })
            .count(),
        1
    );
    assert_eq!(
        value["denied_by_activation_command_result_receipt_count"],
        25
    );
    assert_eq!(
        value["denied_by_activation_command_result_receipt"]
            .as_array()
            .expect("activation command result receipt denials")
            .len(),
        25
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "run_memory_write_execution_activation_command_result_receipt_no_persistence_boundary_require_live_gate"
    );
    assert_eq!(value["allowed_next_actions"][0]["writes_memory"], false);
    assert_eq!(
        value["allowed_next_actions"][1]["action"],
        "prepare_memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_boundary"
    );
    let side_effects = value["side_effects"].as_object().expect(
            "memory write execution activation command result receipt no-persistence boundary side effects",
        );
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_boundary_endpoint_blocks_replay_and_idempotency()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_COMMAND_RESULT_RECEIPT_REPLAY_IDEMPOTENCY_DENIAL_BOUNDARY_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "memory write execution activation command result receipt replay/idempotency denial boundary json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_COMMAND_RESULT_RECEIPT_REPLAY_IDEMPOTENCY_DENIAL_BOUNDARY_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-replay-idempotency-denial-boundary --json"
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
        value["memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_boundary_ready"],
        true
    );
    assert_eq!(
        value["activation_command_result_receipt_replay_idempotency_mode"],
        "memory_write_execution_activation_command_result_receipt_replay_idempotency_denial"
    );
    assert_eq!(
        value["source_activation_command_result_receipt_no_persistence_boundary_ready"],
        true
    );
    assert_eq!(
        value["source_activation_command_result_receipt_no_persistence_ready"],
        true
    );
    assert_ne!(
        value["source_activation_command_result_receipt_no_persistence_boundary_report_sha256"],
        ""
    );
    assert_eq!(
        value["memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_ready"],
        true
    );
    assert_eq!(
        value["memory_write_execution_activation_command_result_receipt_no_persistence_ready"],
        true
    );
    assert_eq!(
        value["required_activation_command_result_receipt_replay_idempotency_surface_count"],
        12
    );
    assert_eq!(
        value["ready_activation_command_result_receipt_replay_idempotency_surface_count"],
        12
    );
    assert_eq!(
        value["side_effect_free_activation_command_result_receipt_replay_idempotency_surface_count"],
        12
    );
    assert_eq!(
        value["required_activation_command_result_receipt_replay_idempotency_fixture_count"],
        10
    );
    assert_eq!(
        value["activation_command_result_receipt_replay_idempotency_fixture_count"],
        10
    );
    assert_eq!(
        value["blocked_activation_command_result_receipt_replay_idempotency_fixture_count"],
        10
    );
    assert_eq!(
        value["noop_activation_command_result_receipt_replay_idempotency_fixture_count"],
        10
    );
    assert_eq!(
        value["allowed_activation_command_result_receipt_replay_idempotency_fixture_count"],
        0
    );
    assert_eq!(
        value["accepted_activation_command_result_receipt_replay_idempotency_fixture_count"],
        0
    );
    assert_eq!(
        value["duplicate_activation_command_result_receipt_fixture_count"],
        2
    );
    assert_eq!(
        value["cross_scope_activation_command_result_receipt_fixture_count"],
        1
    );
    assert_eq!(
        value["status_upgrade_activation_command_result_receipt_fixture_count"],
        1
    );
    assert_eq!(
        value["activation_command_result_receipt_replay_denied_count"],
        10
    );
    assert_eq!(
        value["activation_command_result_receipt_duplicate_denied_count"],
        10
    );
    assert_eq!(
        value["activation_command_result_receipt_idempotency_denied_count"],
        10
    );
    assert_eq!(
        value["activation_command_result_receipt_replay_performed_count"],
        0
    );
    assert_eq!(
        value["activation_command_result_receipt_duplicate_accepted_count"],
        0
    );
    assert_eq!(
        value["activation_command_result_receipt_idempotency_state_recorded_count"],
        0
    );

    for key in [
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
        "public_release_published",
        "public_ga_claimed",
        "release_artifact_written",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
    ] {
        assert_eq!(value[key], false, "{key}");
    }
    assert_eq!(value["memory_store_write_performed_count"], 0);

    let surfaces = value["activation_command_result_receipt_replay_idempotency_surfaces"]
        .as_array()
        .expect("activation command result receipt replay/idempotency surfaces");
    assert_eq!(surfaces.len(), 12);
    let fixtures = value["activation_command_result_receipt_replay_idempotency_fixtures"]
        .as_array()
        .expect("activation command result receipt replay/idempotency fixtures");
    assert_eq!(fixtures.len(), 10);
    assert!(fixtures.iter().all(|fixture| {
        (fixture["replay_status"].as_str() == Some("blocked_noop")
            || fixture["replay_status"].as_str() == Some("blocked_duplicate_noop"))
            && fixture["replay_requested"].as_bool() == Some(true)
            && fixture["replay_allowed"].as_bool() == Some(false)
            && fixture["replay_recorded"].as_bool() == Some(false)
            && fixture["replay_persisted"].as_bool() == Some(false)
            && fixture["duplicate_accepted"].as_bool() == Some(false)
            && fixture["idempotency_key_accepted"].as_bool() == Some(false)
            && fixture["idempotency_state_recorded"].as_bool() == Some(false)
            && fixture["idempotency_state_persisted"].as_bool() == Some(false)
            && fixture["receipt_recorded"].as_bool() == Some(false)
            && fixture["receipt_persisted"].as_bool() == Some(false)
            && fixture["receipt_accepted"].as_bool() == Some(false)
            && fixture["completion_ack_recorded"].as_bool() == Some(false)
            && fixture["activation_allowed"].as_bool() == Some(false)
            && fixture["live_mutation_execution_performed"].as_bool() == Some(false)
            && fixture["memory_store_write_performed"].as_bool() == Some(false)
            && fixture["memory_store_mutated"].as_bool() == Some(false)
            && fixture["rollback_executed"].as_bool() == Some(false)
            && fixture["receipt_noop_confirmed"].as_bool() == Some(true)
    }));
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["duplicate_receipt_id_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["stale_idempotency_key_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["cross_scope_reuse_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["receipt_status_requested"] == "completed")
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["completion_ack_replay_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["ledger_replay_requested"] == true
                    && fixture["index_replay_requested"] == true
                    && fixture["delivery_replay_requested"] == true
            })
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["memory_write_replay_requested"] == true
                    && fixture["live_mutation_replay_requested"] == true
            })
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["rollback_replay_requested"] == true
                    && fixture["secret_material_replay_requested"] == true
                    && fixture["provider_prompt_replay_requested"] == true
            })
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["external_send_replay_requested"] == true
                    && fixture["install_replay_requested"] == true
                    && fixture["active_binary_mutation_replay_requested"] == true
            })
            .count(),
        1
    );
    assert_eq!(
        value["denied_by_activation_command_result_receipt_replay_idempotency_count"],
        24
    );
    assert_eq!(
        value["denied_by_activation_command_result_receipt_replay_idempotency"]
            .as_array()
            .expect("activation command result receipt replay/idempotency denials")
            .len(),
        24
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "run_memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_boundary_require_live_gate"
    );
    assert_eq!(value["allowed_next_actions"][0]["writes_memory"], false);
    assert_eq!(
        value["allowed_next_actions"][1]["action"],
        "prepare_memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_boundary"
    );
    let side_effects = value["side_effects"].as_object().expect(
            "memory write execution activation command result receipt replay/idempotency boundary side effects",
        );
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_boundary_endpoint_blocks_ordering_and_monotonicity()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_COMMAND_RESULT_RECEIPT_ORDERING_MONOTONICITY_DENIAL_BOUNDARY_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "memory write execution activation command result receipt ordering/monotonicity denial boundary json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_COMMAND_RESULT_RECEIPT_ORDERING_MONOTONICITY_DENIAL_BOUNDARY_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-ordering-monotonicity-denial-boundary --json"
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
        value["memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_boundary_ready"],
        true
    );
    assert_eq!(
        value["activation_command_result_receipt_ordering_monotonicity_mode"],
        "memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial"
    );
    assert_eq!(
        value["source_activation_command_result_receipt_replay_idempotency_boundary_ready"],
        true
    );
    assert_eq!(
        value["source_activation_command_result_receipt_replay_idempotency_ready"],
        true
    );
    assert_ne!(
        value["source_activation_command_result_receipt_replay_idempotency_boundary_report_sha256"],
        ""
    );
    assert_eq!(
        value["memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_ready"],
        true
    );
    assert_eq!(
        value["memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_ready"],
        true
    );
    assert_eq!(
        value["required_activation_command_result_receipt_ordering_monotonicity_surface_count"],
        12
    );
    assert_eq!(
        value["ready_activation_command_result_receipt_ordering_monotonicity_surface_count"],
        12
    );
    assert_eq!(
        value["side_effect_free_activation_command_result_receipt_ordering_monotonicity_surface_count"],
        12
    );
    assert_eq!(
        value["activation_command_result_receipt_ordering_monotonicity_fixture_count"],
        10
    );
    assert_eq!(
        value["blocked_activation_command_result_receipt_ordering_monotonicity_fixture_count"],
        10
    );
    assert_eq!(
        value["noop_activation_command_result_receipt_ordering_monotonicity_fixture_count"],
        10
    );
    assert_eq!(
        value["allowed_activation_command_result_receipt_ordering_monotonicity_fixture_count"],
        0
    );
    assert_eq!(
        value["accepted_activation_command_result_receipt_ordering_monotonicity_fixture_count"],
        0
    );
    assert_eq!(
        value["activation_command_result_receipt_ordering_violation_denied_count"],
        10
    );
    assert_eq!(
        value["activation_command_result_receipt_monotonicity_violation_denied_count"],
        10
    );
    assert_eq!(
        value["activation_command_result_receipt_ordering_performed_count"],
        0
    );
    assert_eq!(
        value["activation_command_result_receipt_sequence_cursor_accepted_count"],
        0
    );
    assert_eq!(
        value["activation_command_result_receipt_sequence_cursor_recorded_count"],
        0
    );
    assert_eq!(
        value["activation_command_result_receipt_monotonicity_state_recorded_count"],
        0
    );

    for key in [
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
        "activation_allowed_by_result_receipt",
        "activation_allowed",
        "activation_performed",
        "live_mutation_execution_performed",
        "memory_write_execution_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "rollback_executed",
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
        assert_eq!(value[key], false, "{key}");
    }
    assert_eq!(value["memory_store_write_performed_count"], 0);

    let surfaces = value["activation_command_result_receipt_ordering_monotonicity_surfaces"]
        .as_array()
        .expect("activation command result receipt ordering/monotonicity surfaces");
    assert_eq!(surfaces.len(), 12);
    let fixtures = value["activation_command_result_receipt_ordering_monotonicity_fixtures"]
        .as_array()
        .expect("activation command result receipt ordering/monotonicity fixtures");
    assert_eq!(fixtures.len(), 10);
    assert!(fixtures.iter().all(|fixture| {
        (fixture["ordering_status"].as_str() == Some("blocked_noop")
            || fixture["ordering_status"].as_str() == Some("blocked_ordering_noop"))
            && fixture["ordering_allowed"].as_bool() == Some(false)
            && fixture["ordering_recorded"].as_bool() == Some(false)
            && fixture["ordering_persisted"].as_bool() == Some(false)
            && fixture["sequence_cursor_accepted"].as_bool() == Some(false)
            && fixture["sequence_cursor_recorded"].as_bool() == Some(false)
            && fixture["sequence_cursor_persisted"].as_bool() == Some(false)
            && fixture["monotonicity_state_recorded"].as_bool() == Some(false)
            && fixture["monotonicity_state_persisted"].as_bool() == Some(false)
            && fixture["receipt_recorded"].as_bool() == Some(false)
            && fixture["receipt_persisted"].as_bool() == Some(false)
            && fixture["receipt_accepted"].as_bool() == Some(false)
            && fixture["completion_ack_recorded"].as_bool() == Some(false)
            && fixture["activation_allowed"].as_bool() == Some(false)
            && fixture["live_mutation_execution_performed"].as_bool() == Some(false)
            && fixture["memory_store_write_performed"].as_bool() == Some(false)
            && fixture["memory_store_mutated"].as_bool() == Some(false)
            && fixture["rollback_executed"].as_bool() == Some(false)
            && fixture["receipt_noop_confirmed"].as_bool() == Some(true)
    }));
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["out_of_order_sequence_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["sequence_gap_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["timestamp_rollback_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["epoch_rollback_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["same_sequence_different_hash_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["latest_wins_overwrite_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["completion_ack_before_noop_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["ledger_ordering_bypass_requested"] == true
                    && fixture["index_ordering_bypass_requested"] == true
                    && fixture["delivery_ordering_bypass_requested"] == true
            })
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["external_send_ordering_bypass_requested"] == true
                    && fixture["install_ordering_bypass_requested"] == true
                    && fixture["active_binary_mutation_ordering_bypass_requested"] == true
            })
            .count(),
        1
    );
    assert_eq!(
        value["denied_by_activation_command_result_receipt_ordering_monotonicity_count"],
        24
    );
    assert_eq!(
        value["denied_by_activation_command_result_receipt_ordering_monotonicity"]
            .as_array()
            .expect("activation command result receipt ordering/monotonicity denials")
            .len(),
        24
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "run_memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_boundary_require_live_gate"
    );
    assert_eq!(value["allowed_next_actions"][0]["writes_memory"], false);
    assert_eq!(
        value["allowed_next_actions"][1]["action"],
        "prepare_memory_write_execution_activation_command_result_receipt_cancellation_supersession_denial_boundary"
    );
    let side_effects = value["side_effects"].as_object().expect(
            "memory write execution activation command result receipt ordering/monotonicity boundary side effects",
        );
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_write_execution_activation_command_result_receipt_cancellation_supersession_denial_boundary_endpoint_blocks_lifecycle()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_COMMAND_RESULT_RECEIPT_CANCELLATION_SUPERSESSION_DENIAL_BOUNDARY_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "memory write execution activation command result receipt cancellation/supersession denial boundary json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_COMMAND_RESULT_RECEIPT_CANCELLATION_SUPERSESSION_DENIAL_BOUNDARY_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-cancellation-supersession-denial-boundary --json"
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
        value["memory_write_execution_activation_command_result_receipt_cancellation_supersession_denial_boundary_ready"],
        true
    );
    assert_eq!(
        value["activation_command_result_receipt_cancellation_supersession_mode"],
        "memory_write_execution_activation_command_result_receipt_cancellation_supersession_denial"
    );
    assert_eq!(
        value["source_activation_command_result_receipt_ordering_monotonicity_boundary_ready"],
        true
    );
    assert_eq!(
        value["source_activation_command_result_receipt_ordering_monotonicity_ready"],
        true
    );
    assert_ne!(
        value["source_activation_command_result_receipt_ordering_monotonicity_boundary_report_sha256"],
        ""
    );
    assert_eq!(
        value["memory_write_execution_activation_command_result_receipt_cancellation_supersession_denial_ready"],
        true
    );
    assert_eq!(
        value["memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_ready"],
        true
    );
    assert_eq!(
        value["memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_ready"],
        true
    );
    assert_eq!(
        value["required_activation_command_result_receipt_cancellation_supersession_surface_count"],
        12
    );
    assert_eq!(
        value["ready_activation_command_result_receipt_cancellation_supersession_surface_count"],
        12
    );
    assert_eq!(
        value["side_effect_free_activation_command_result_receipt_cancellation_supersession_surface_count"],
        12
    );
    assert_eq!(
        value["activation_command_result_receipt_cancellation_supersession_fixture_count"],
        10
    );
    assert_eq!(
        value["blocked_activation_command_result_receipt_cancellation_supersession_fixture_count"],
        10
    );
    assert_eq!(
        value["noop_activation_command_result_receipt_cancellation_supersession_fixture_count"],
        10
    );
    assert_eq!(
        value["allowed_activation_command_result_receipt_cancellation_supersession_fixture_count"],
        0
    );
    assert_eq!(
        value["accepted_activation_command_result_receipt_cancellation_supersession_fixture_count"],
        0
    );
    assert_eq!(
        value["activation_command_result_receipt_cancellation_denied_count"],
        10
    );
    assert_eq!(
        value["activation_command_result_receipt_supersession_denied_count"],
        10
    );
    assert_eq!(
        value["activation_command_result_receipt_cancellation_performed_count"],
        0
    );
    assert_eq!(
        value["activation_command_result_receipt_supersession_performed_count"],
        0
    );

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
        assert_eq!(value[key], false, "{key}");
    }
    assert_eq!(value["memory_store_write_performed_count"], 0);

    let surfaces = value["activation_command_result_receipt_cancellation_supersession_surfaces"]
        .as_array()
        .expect("activation command result receipt cancellation/supersession surfaces");
    assert_eq!(surfaces.len(), 12);
    let fixtures = value["activation_command_result_receipt_cancellation_supersession_fixtures"]
        .as_array()
        .expect("activation command result receipt cancellation/supersession fixtures");
    assert_eq!(fixtures.len(), 10);
    assert!(fixtures.iter().all(|fixture| {
        (fixture["cancellation_supersession_status"].as_str() == Some("blocked_noop")
            || fixture["cancellation_supersession_status"].as_str()
                == Some("blocked_supersession_noop"))
            && fixture["cancellation_allowed"].as_bool() == Some(false)
            && fixture["cancellation_recorded"].as_bool() == Some(false)
            && fixture["cancellation_persisted"].as_bool() == Some(false)
            && fixture["supersession_allowed"].as_bool() == Some(false)
            && fixture["supersession_recorded"].as_bool() == Some(false)
            && fixture["supersession_persisted"].as_bool() == Some(false)
            && fixture["replacement_receipt_accepted"].as_bool() == Some(false)
            && fixture["replacement_hash_accepted"].as_bool() == Some(false)
            && fixture["tombstone_recorded"].as_bool() == Some(false)
            && fixture["receipt_recorded"].as_bool() == Some(false)
            && fixture["receipt_persisted"].as_bool() == Some(false)
            && fixture["receipt_accepted"].as_bool() == Some(false)
            && fixture["completion_ack_recorded"].as_bool() == Some(false)
            && fixture["activation_allowed"].as_bool() == Some(false)
            && fixture["live_mutation_execution_performed"].as_bool() == Some(false)
            && fixture["memory_store_write_performed"].as_bool() == Some(false)
            && fixture["memory_store_mutated"].as_bool() == Some(false)
            && fixture["rollback_executed"].as_bool() == Some(false)
            && fixture["receipt_noop_confirmed"].as_bool() == Some(true)
    }));
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["source_ordering_monotonicity_present"] == false)
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["cancellation_request_shape"] == "cancel_blocked_noop_receipt"
            })
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["requested_replacement_status"] == "completed")
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["replacement_hash_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["tombstone_requested"] == true && fixture["delete_marker_requested"] == true
            })
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["ledger_cancellation_requested"] == true
                    && fixture["index_cancellation_requested"] == true
                    && fixture["delivery_cancellation_requested"] == true
            })
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["memory_write_supersession_requested"] == true
                    && fixture["live_mutation_supersession_requested"] == true
            })
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["rollback_supersession_requested"] == true
                    && fixture["secret_material_supersession_requested"] == true
                    && fixture["provider_prompt_supersession_requested"] == true
            })
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["external_send_supersession_requested"] == true
                    && fixture["install_supersession_requested"] == true
                    && fixture["active_binary_mutation_supersession_requested"] == true
            })
            .count(),
        1
    );
    assert_eq!(
        value["denied_by_activation_command_result_receipt_cancellation_supersession_count"],
        24
    );
    assert_eq!(
        value["denied_by_activation_command_result_receipt_cancellation_supersession"]
            .as_array()
            .expect("activation command result receipt cancellation/supersession denials")
            .len(),
        24
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "run_memory_write_execution_activation_command_result_receipt_cancellation_supersession_denial_boundary_require_live_gate"
    );
    assert_eq!(value["allowed_next_actions"][0]["writes_memory"], false);
    assert_eq!(
        value["allowed_next_actions"][1]["action"],
        "prepare_memory_write_execution_activation_command_result_receipt_audit_trail_immutable_evidence_denial_boundary"
    );
    let side_effects = value["side_effects"].as_object().expect(
            "memory write execution activation command result receipt cancellation/supersession boundary side effects",
        );
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_write_execution_activation_command_result_receipt_audit_trail_immutable_evidence_denial_boundary_endpoint_blocks_evidence()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_COMMAND_RESULT_RECEIPT_AUDIT_TRAIL_IMMUTABLE_EVIDENCE_DENIAL_BOUNDARY_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "memory write execution activation command result receipt audit trail/immutable evidence denial boundary json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_COMMAND_RESULT_RECEIPT_AUDIT_TRAIL_IMMUTABLE_EVIDENCE_DENIAL_BOUNDARY_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-audit-trail-immutable-evidence-denial-boundary --json"
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
        value["memory_write_execution_activation_command_result_receipt_audit_trail_immutable_evidence_denial_boundary_ready"],
        true
    );
    assert_eq!(
        value["activation_command_result_receipt_audit_trail_immutable_evidence_mode"],
        "memory_write_execution_activation_command_result_receipt_audit_trail_immutable_evidence_denial"
    );
    assert_eq!(
        value["source_activation_command_result_receipt_cancellation_supersession_boundary_ready"],
        true
    );
    assert_eq!(
        value["source_activation_command_result_receipt_cancellation_supersession_ready"],
        true
    );
    assert_ne!(
        value["source_activation_command_result_receipt_cancellation_supersession_boundary_report_sha256"],
        ""
    );
    assert_eq!(
        value["memory_write_execution_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready"],
        true
    );
    assert_eq!(
        value["memory_write_execution_activation_command_result_receipt_cancellation_supersession_denial_ready"],
        true
    );
    assert_eq!(
        value["memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_ready"],
        true
    );
    assert_eq!(
        value["memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_ready"],
        true
    );
    assert_eq!(
        value["required_activation_command_result_receipt_audit_trail_immutable_evidence_surface_count"],
        12
    );
    assert_eq!(
        value["ready_activation_command_result_receipt_audit_trail_immutable_evidence_surface_count"],
        12
    );
    assert_eq!(
        value["side_effect_free_activation_command_result_receipt_audit_trail_immutable_evidence_surface_count"],
        12
    );
    assert_eq!(
        value["activation_command_result_receipt_audit_trail_immutable_evidence_fixture_count"],
        10
    );
    assert_eq!(
        value["blocked_activation_command_result_receipt_audit_trail_immutable_evidence_fixture_count"],
        10
    );
    assert_eq!(
        value["noop_activation_command_result_receipt_audit_trail_immutable_evidence_fixture_count"],
        10
    );
    assert_eq!(
        value["allowed_activation_command_result_receipt_audit_trail_immutable_evidence_fixture_count"],
        0
    );
    assert_eq!(
        value["accepted_activation_command_result_receipt_audit_trail_immutable_evidence_fixture_count"],
        0
    );
    assert_eq!(
        value["activation_command_result_receipt_audit_trail_denied_count"],
        10
    );
    assert_eq!(
        value["activation_command_result_receipt_immutable_evidence_denied_count"],
        10
    );
    assert_eq!(
        value["activation_command_result_receipt_audit_trail_performed_count"],
        0
    );
    assert_eq!(
        value["activation_command_result_receipt_immutable_evidence_performed_count"],
        0
    );

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
        "activation_command_result_receipt_delivery_evidence_recorded",
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
        "activation_command_completion_ack_recorded",
        "activation_command_completion_ack_persisted",
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
    ] {
        assert_eq!(value[key], false, "{key}");
    }
    assert_eq!(value["memory_store_write_performed_count"], 0);

    let surfaces =
        value["activation_command_result_receipt_audit_trail_immutable_evidence_surfaces"]
            .as_array()
            .expect("activation command result receipt audit trail/immutable evidence surfaces");
    assert_eq!(surfaces.len(), 12);
    let fixtures =
        value["activation_command_result_receipt_audit_trail_immutable_evidence_fixtures"]
            .as_array()
            .expect("activation command result receipt audit trail/immutable evidence fixtures");
    assert_eq!(fixtures.len(), 10);
    assert!(fixtures.iter().all(|fixture| {
        (fixture["audit_evidence_status"].as_str() == Some("blocked_noop")
            || fixture["audit_evidence_status"].as_str() == Some("blocked_evidence_noop"))
            && fixture["audit_trail_allowed"].as_bool() == Some(false)
            && fixture["audit_trail_recorded"].as_bool() == Some(false)
            && fixture["audit_trail_persisted"].as_bool() == Some(false)
            && fixture["immutable_evidence_allowed"].as_bool() == Some(false)
            && fixture["immutable_evidence_recorded"].as_bool() == Some(false)
            && fixture["immutable_evidence_persisted"].as_bool() == Some(false)
            && fixture["hash_chain_recorded"].as_bool() == Some(false)
            && fixture["merkle_root_recorded"].as_bool() == Some(false)
            && fixture["attestation_recorded"].as_bool() == Some(false)
            && fixture["witness_recorded"].as_bool() == Some(false)
            && fixture["notary_recorded"].as_bool() == Some(false)
            && fixture["ledger_evidence_recorded"].as_bool() == Some(false)
            && fixture["index_evidence_recorded"].as_bool() == Some(false)
            && fixture["delivery_evidence_recorded"].as_bool() == Some(false)
            && fixture["receipt_recorded"].as_bool() == Some(false)
            && fixture["receipt_persisted"].as_bool() == Some(false)
            && fixture["receipt_accepted"].as_bool() == Some(false)
            && fixture["activation_allowed"].as_bool() == Some(false)
            && fixture["live_mutation_execution_performed"].as_bool() == Some(false)
            && fixture["memory_store_write_performed"].as_bool() == Some(false)
            && fixture["memory_store_mutated"].as_bool() == Some(false)
            && fixture["rollback_executed"].as_bool() == Some(false)
            && fixture["receipt_noop_confirmed"].as_bool() == Some(true)
    }));
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["source_cancellation_supersession_present"] == false)
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["audit_trail_request_shape"] == "append_blocked_noop_receipt_audit_trail"
            })
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["immutable_evidence_request_shape"]
                    == "seal_blocked_noop_receipt_as_immutable_evidence"
            })
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["hash_chain_requested"] == true && fixture["merkle_root_requested"] == true
            })
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["attestation_requested"] == true
                    && fixture["witness_requested"] == true
                    && fixture["notary_requested"] == true
            })
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["ledger_evidence_requested"] == true
                    && fixture["index_evidence_requested"] == true
                    && fixture["delivery_evidence_requested"] == true
            })
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["memory_write_evidence_requested"] == true
                    && fixture["rollback_evidence_requested"] == true
                    && fixture["secret_material_evidence_requested"] == true
                    && fixture["provider_prompt_evidence_requested"] == true
            })
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["external_send_evidence_requested"] == true
                    && fixture["install_evidence_requested"] == true
                    && fixture["active_binary_mutation_evidence_requested"] == true
            })
            .count(),
        1
    );
    assert_eq!(
        value["denied_by_activation_command_result_receipt_audit_trail_immutable_evidence_count"],
        24
    );
    assert_eq!(
        value["denied_by_activation_command_result_receipt_audit_trail_immutable_evidence"]
            .as_array()
            .expect("activation command result receipt audit trail/immutable evidence denials")
            .len(),
        24
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "run_memory_write_execution_activation_command_result_receipt_audit_trail_immutable_evidence_denial_boundary_require_live_gate"
    );
    assert_eq!(value["allowed_next_actions"][0]["writes_memory"], false);
    assert_eq!(
        value["allowed_next_actions"][1]["action"],
        "prepare_memory_write_execution_activation_command_result_receipt_retention_expiry_garbage_collection_denial_boundary"
    );
    let side_effects = value["side_effects"].as_object().expect(
            "memory write execution activation command result receipt audit trail/immutable evidence boundary side effects",
        );
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_write_execution_activation_command_result_receipt_retention_expiry_garbage_collection_denial_boundary_endpoint_blocks_lifecycle()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_COMMAND_RESULT_RECEIPT_RETENTION_EXPIRY_GARBAGE_COLLECTION_DENIAL_BOUNDARY_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "memory write execution activation command result receipt retention/expiry/garbage-collection denial boundary json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_COMMAND_RESULT_RECEIPT_RETENTION_EXPIRY_GARBAGE_COLLECTION_DENIAL_BOUNDARY_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-retention-expiry-garbage-collection-denial-boundary --json"
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
        value["memory_write_execution_activation_command_result_receipt_retention_expiry_garbage_collection_denial_boundary_ready"],
        true
    );
    assert_eq!(
        value["activation_command_result_receipt_retention_expiry_garbage_collection_mode"],
        "memory_write_execution_activation_command_result_receipt_retention_expiry_garbage_collection_denial"
    );
    assert_eq!(
        value["source_activation_command_result_receipt_audit_trail_immutable_evidence_boundary_ready"],
        true
    );
    assert_eq!(
        value["source_activation_command_result_receipt_audit_trail_immutable_evidence_ready"],
        true
    );
    assert_ne!(
        value["source_activation_command_result_receipt_audit_trail_immutable_evidence_boundary_report_sha256"],
        ""
    );
    assert_eq!(
        value["memory_write_execution_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready"],
        true
    );
    assert_eq!(
        value["memory_write_execution_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready"],
        true
    );
    assert_eq!(
        value["memory_write_execution_activation_command_result_receipt_cancellation_supersession_denial_ready"],
        true
    );
    assert_eq!(
        value["memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_ready"],
        true
    );
    assert_eq!(
        value["memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_ready"],
        true
    );
    assert_eq!(
        value["required_activation_command_result_receipt_retention_expiry_garbage_collection_surface_count"],
        12
    );
    assert_eq!(
        value["ready_activation_command_result_receipt_retention_expiry_garbage_collection_surface_count"],
        12
    );
    assert_eq!(
        value["side_effect_free_activation_command_result_receipt_retention_expiry_garbage_collection_surface_count"],
        12
    );
    assert_eq!(
        value["activation_command_result_receipt_retention_expiry_garbage_collection_fixture_count"],
        10
    );
    assert_eq!(
        value["blocked_activation_command_result_receipt_retention_expiry_garbage_collection_fixture_count"],
        10
    );
    assert_eq!(
        value["noop_activation_command_result_receipt_retention_expiry_garbage_collection_fixture_count"],
        10
    );
    assert_eq!(
        value["allowed_activation_command_result_receipt_retention_expiry_garbage_collection_fixture_count"],
        0
    );
    assert_eq!(
        value["accepted_activation_command_result_receipt_retention_expiry_garbage_collection_fixture_count"],
        0
    );
    assert_eq!(
        value["activation_command_result_receipt_retention_denied_count"],
        10
    );
    assert_eq!(
        value["activation_command_result_receipt_expiry_denied_count"],
        10
    );
    assert_eq!(
        value["activation_command_result_receipt_garbage_collection_denied_count"],
        10
    );
    assert_eq!(
        value["activation_command_result_receipt_retention_performed_count"],
        0
    );
    assert_eq!(
        value["activation_command_result_receipt_expiry_performed_count"],
        0
    );
    assert_eq!(
        value["activation_command_result_receipt_garbage_collection_performed_count"],
        0
    );
    assert_eq!(value["memory_store_write_performed_count"], 0);

    for key in [
        "activation_command_result_receipt_retention_policy_allowed",
        "activation_command_result_receipt_retention_policy_recorded",
        "activation_command_result_receipt_retention_policy_persisted",
        "activation_command_result_receipt_retention_index_allowed",
        "activation_command_result_receipt_retention_index_recorded",
        "activation_command_result_receipt_expiry_allowed",
        "activation_command_result_receipt_expiry_recorded",
        "activation_command_result_receipt_expiry_scheduler_registered",
        "activation_command_result_receipt_expiry_timer_started",
        "activation_command_result_receipt_ttl_update_allowed",
        "activation_command_result_receipt_ttl_extension_allowed",
        "activation_command_result_receipt_garbage_collection_allowed",
        "activation_command_result_receipt_garbage_collection_scan_performed",
        "activation_command_result_receipt_garbage_collection_candidate_recorded",
        "activation_command_result_receipt_garbage_collection_decision_recorded",
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
        "activation_command_result_receipt_index_retention_recorded",
        "activation_command_result_receipt_delivery_retention_recorded",
        "activation_command_result_receipt_audit_trail_recorded",
        "activation_command_result_receipt_immutable_evidence_recorded",
        "activation_command_result_receipt_recorded",
        "activation_command_result_receipt_persisted",
        "activation_command_result_receipt_accepted",
        "activation_command_result_receipt_materialized",
        "activation_command_completion_ack_recorded",
        "activation_command_completion_ack_persisted",
        "activation_allowed_by_result_receipt_retention",
        "activation_allowed_by_result_receipt_expiry",
        "activation_allowed_by_result_receipt_garbage_collection",
        "activation_allowed_by_result_receipt_audit_trail",
        "activation_allowed_by_result_receipt_immutable_evidence",
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
        assert_eq!(value[key], false, "{key}");
    }

    let surfaces =
        value["activation_command_result_receipt_retention_expiry_garbage_collection_surfaces"]
            .as_array()
            .expect(
                "activation command result receipt retention/expiry/garbage-collection surfaces",
            );
    assert_eq!(surfaces.len(), 12);
    let fixtures =
        value["activation_command_result_receipt_retention_expiry_garbage_collection_fixtures"]
            .as_array()
            .expect(
                "activation command result receipt retention/expiry/garbage-collection fixtures",
            );
    assert_eq!(fixtures.len(), 10);
    assert!(fixtures.iter().all(|fixture| {
        (fixture["retention_gc_status"].as_str() == Some("blocked_noop")
            || fixture["retention_gc_status"].as_str() == Some("blocked_expiry_noop")
            || fixture["retention_gc_status"].as_str() == Some("blocked_gc_noop"))
            && fixture["retention_policy_recorded"].as_bool() == Some(false)
            && fixture["retention_policy_persisted"].as_bool() == Some(false)
            && fixture["expiry_recorded"].as_bool() == Some(false)
            && fixture["expiry_scheduler_registered"].as_bool() == Some(false)
            && fixture["garbage_collection_allowed"].as_bool() == Some(false)
            && fixture["garbage_collection_scan_performed"].as_bool() == Some(false)
            && fixture["delete_performed"].as_bool() == Some(false)
            && fixture["tombstone_recorded"].as_bool() == Some(false)
            && fixture["sweep_performed"].as_bool() == Some(false)
            && fixture["archive_written"].as_bool() == Some(false)
            && fixture["compaction_performed"].as_bool() == Some(false)
            && fixture["receipt_recorded"].as_bool() == Some(false)
            && fixture["receipt_persisted"].as_bool() == Some(false)
            && fixture["receipt_accepted"].as_bool() == Some(false)
            && fixture["activation_allowed"].as_bool() == Some(false)
            && fixture["live_mutation_execution_performed"].as_bool() == Some(false)
            && fixture["memory_store_write_performed"].as_bool() == Some(false)
            && fixture["memory_store_mutated"].as_bool() == Some(false)
            && fixture["rollback_executed"].as_bool() == Some(false)
            && fixture["receipt_noop_confirmed"].as_bool() == Some(true)
    }));
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["source_audit_evidence_present"] == false)
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["retention_policy_request_shape"]
                    == "record_blocked_noop_receipt_retention_policy"
            })
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["retention_index_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["expiry_schedule_requested"] == true
                    && fixture["expiry_timer_requested"] == true
            })
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["ttl_update_requested"] == true
                    && fixture["ttl_extension_requested"] == true
            })
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["garbage_collection_scan_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["delete_requested"] == true
                    && fixture["tombstone_requested"] == true
                    && fixture["sweep_requested"] == true
            })
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["archive_requested"] == true && fixture["compaction_requested"] == true
            })
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["activation_from_retention_gc_requested"] == true
                    && fixture["memory_write_gc_evidence_requested"] == true
                    && fixture["rollback_gc_evidence_requested"] == true
                    && fixture["provider_prompt_gc_evidence_requested"] == true
            })
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["external_send_gc_evidence_requested"] == true
                    && fixture["install_gc_evidence_requested"] == true
                    && fixture["active_binary_gc_evidence_requested"] == true
            })
            .count(),
        1
    );
    assert_eq!(
        value["denied_by_activation_command_result_receipt_retention_expiry_garbage_collection_count"],
        29
    );
    assert_eq!(
        value["denied_by_activation_command_result_receipt_retention_expiry_garbage_collection"]
            .as_array()
            .expect("activation command result receipt retention/expiry/garbage-collection denials")
            .len(),
        29
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "run_memory_write_execution_activation_command_result_receipt_retention_expiry_garbage_collection_denial_boundary_require_live_gate"
    );
    assert_eq!(value["allowed_next_actions"][0]["deletes_memory"], false);
    assert_eq!(
        value["allowed_next_actions"][1]["action"],
        "prepare_memory_write_execution_activation_command_result_receipt_export_query_observability_denial_boundary"
    );
    let side_effects = value["side_effects"].as_object().expect(
            "memory write execution activation command result receipt retention/expiry/garbage-collection boundary side effects",
        );
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_write_execution_activation_command_result_receipt_export_query_observability_denial_boundary_endpoint_blocks_visibility()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_COMMAND_RESULT_RECEIPT_EXPORT_QUERY_OBSERVABILITY_DENIAL_BOUNDARY_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "memory write execution activation command result receipt export/query/observability denial boundary json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_COMMAND_RESULT_RECEIPT_EXPORT_QUERY_OBSERVABILITY_DENIAL_BOUNDARY_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-export-query-observability-denial-boundary --json"
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
        value["memory_write_execution_activation_command_result_receipt_export_query_observability_denial_boundary_ready"],
        true
    );
    assert_eq!(
        value["activation_command_result_receipt_export_query_observability_mode"],
        "memory_write_execution_activation_command_result_receipt_export_query_observability_denial"
    );
    assert_eq!(
        value["source_activation_command_result_receipt_retention_expiry_garbage_collection_boundary_ready"],
        true
    );
    assert_eq!(
        value["source_activation_command_result_receipt_retention_expiry_garbage_collection_ready"],
        true
    );
    assert_ne!(
        value["source_activation_command_result_receipt_retention_expiry_garbage_collection_boundary_report_sha256"],
        ""
    );
    assert_eq!(
        value["memory_write_execution_activation_command_result_receipt_export_query_observability_denial_ready"],
        true
    );
    assert_eq!(
        value["memory_write_execution_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready"],
        true
    );
    assert_eq!(
        value["memory_write_execution_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready"],
        true
    );
    assert_eq!(
        value["memory_write_execution_activation_command_result_receipt_cancellation_supersession_denial_ready"],
        true
    );
    assert_eq!(
        value["memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_ready"],
        true
    );
    assert_eq!(
        value["memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_ready"],
        true
    );
    assert_eq!(
        value["required_activation_command_result_receipt_export_query_observability_surface_count"],
        12
    );
    assert_eq!(
        value["ready_activation_command_result_receipt_export_query_observability_surface_count"],
        12
    );
    assert_eq!(
        value["side_effect_free_activation_command_result_receipt_export_query_observability_surface_count"],
        12
    );
    assert_eq!(
        value["activation_command_result_receipt_export_query_observability_fixture_count"],
        10
    );
    assert_eq!(
        value["blocked_activation_command_result_receipt_export_query_observability_fixture_count"],
        10
    );
    assert_eq!(
        value["noop_activation_command_result_receipt_export_query_observability_fixture_count"],
        10
    );
    assert_eq!(
        value["allowed_activation_command_result_receipt_export_query_observability_fixture_count"],
        0
    );
    assert_eq!(
        value["accepted_activation_command_result_receipt_export_query_observability_fixture_count"],
        0
    );
    assert_eq!(
        value["activation_command_result_receipt_export_denied_count"],
        10
    );
    assert_eq!(
        value["activation_command_result_receipt_query_denied_count"],
        10
    );
    assert_eq!(
        value["activation_command_result_receipt_observability_denied_count"],
        10
    );
    assert_eq!(
        value["activation_command_result_receipt_export_performed_count"],
        0
    );
    assert_eq!(
        value["activation_command_result_receipt_query_performed_count"],
        0
    );
    assert_eq!(
        value["activation_command_result_receipt_observability_performed_count"],
        0
    );
    assert_eq!(value["memory_store_write_performed_count"], 0);

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
        "export_request_accepted",
        "export_recorded",
        "export_persisted",
        "export_artifact_written",
        "export_stream_opened",
        "query_registered",
        "query_endpoint_materialized",
        "query_index_recorded",
        "query_cache_written",
        "query_result_materialized",
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
        "activation_command_result_receipt_garbage_collection_scan_performed",
        "activation_command_result_receipt_recorded",
        "activation_command_result_receipt_persisted",
        "activation_command_result_receipt_accepted",
        "activation_command_result_receipt_materialized",
        "activation_allowed_by_result_receipt_export",
        "activation_allowed_by_result_receipt_query",
        "activation_allowed_by_result_receipt_observability",
        "activation_allowed_by_result_receipt_retention",
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
        assert_eq!(value[key], false, "{key}");
    }

    let surfaces = value["activation_command_result_receipt_export_query_observability_surfaces"]
        .as_array()
        .expect("activation command result receipt export/query/observability surfaces");
    assert_eq!(surfaces.len(), 12);
    let fixtures = value["activation_command_result_receipt_export_query_observability_fixtures"]
        .as_array()
        .expect("activation command result receipt export/query/observability fixtures");
    assert_eq!(fixtures.len(), 10);
    assert!(fixtures.iter().all(|fixture| {
        (fixture["export_query_observability_status"].as_str() == Some("blocked_noop")
            || fixture["export_query_observability_status"].as_str() == Some("blocked_export_noop")
            || fixture["export_query_observability_status"].as_str() == Some("blocked_query_noop")
            || fixture["export_query_observability_status"].as_str()
                == Some("blocked_observability_noop"))
            && fixture["export_request_accepted"].as_bool() == Some(false)
            && fixture["export_artifact_written"].as_bool() == Some(false)
            && fixture["export_stream_opened"].as_bool() == Some(false)
            && fixture["query_registered"].as_bool() == Some(false)
            && fixture["query_endpoint_materialized"].as_bool() == Some(false)
            && fixture["query_index_recorded"].as_bool() == Some(false)
            && fixture["query_cache_written"].as_bool() == Some(false)
            && fixture["observability_metric_emitted"].as_bool() == Some(false)
            && fixture["observability_log_recorded"].as_bool() == Some(false)
            && fixture["observability_trace_recorded"].as_bool() == Some(false)
            && fixture["observability_dashboard_materialized"].as_bool() == Some(false)
            && fixture["observability_alert_registered"].as_bool() == Some(false)
            && fixture["observability_slo_recorded"].as_bool() == Some(false)
            && fixture["ledger_observability_recorded"].as_bool() == Some(false)
            && fixture["index_observability_recorded"].as_bool() == Some(false)
            && fixture["delivery_observability_recorded"].as_bool() == Some(false)
            && fixture["receipt_recorded"].as_bool() == Some(false)
            && fixture["receipt_persisted"].as_bool() == Some(false)
            && fixture["receipt_accepted"].as_bool() == Some(false)
            && fixture["activation_allowed"].as_bool() == Some(false)
            && fixture["live_mutation_execution_performed"].as_bool() == Some(false)
            && fixture["memory_store_write_performed"].as_bool() == Some(false)
            && fixture["memory_store_mutated"].as_bool() == Some(false)
            && fixture["rollback_executed"].as_bool() == Some(false)
            && fixture["receipt_noop_confirmed"].as_bool() == Some(true)
    }));
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["source_retention_expiry_gc_present"] == false)
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["export_file_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["export_stream_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["query_endpoint_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["query_index_requested"] == true && fixture["query_cache_requested"] == true
            })
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["metric_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["trace_requested"] == true
                    && fixture["span_requested"] == true
                    && fixture["log_requested"] == true
            })
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["dashboard_requested"] == true
                    && fixture["alert_requested"] == true
                    && fixture["slo_requested"] == true
            })
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["activation_from_observability_requested"] == true
                    && fixture["memory_write_observability_requested"] == true
                    && fixture["rollback_observability_requested"] == true
                    && fixture["provider_prompt_observability_requested"] == true
            })
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["external_send_observability_requested"] == true
                    && fixture["install_observability_requested"] == true
                    && fixture["active_binary_observability_requested"] == true
            })
            .count(),
        1
    );
    assert_eq!(
        value["denied_by_activation_command_result_receipt_export_query_observability_count"],
        30
    );
    assert_eq!(
        value["denied_by_activation_command_result_receipt_export_query_observability"]
            .as_array()
            .expect("activation command result receipt export/query/observability denials")
            .len(),
        30
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "run_memory_write_execution_activation_command_result_receipt_export_query_observability_denial_boundary_require_live_gate"
    );
    assert_eq!(
        value["allowed_next_actions"][0]["writes_export_artifact"],
        false
    );
    assert_eq!(
        value["allowed_next_actions"][1]["action"],
        "prepare_memory_write_execution_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary"
    );
    let side_effects = value["side_effects"].as_object().expect(
            "memory write execution activation command result receipt export/query/observability boundary side effects",
        );
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_write_execution_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_endpoint_blocks_summary_delivery()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_COMMAND_RESULT_RECEIPT_OPERATOR_FACING_SUMMARY_BRIEFING_NON_PERSISTENCE_DENIAL_BOUNDARY_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "memory write execution activation command result receipt operator summary/briefing non-persistence denial boundary json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_COMMAND_RESULT_RECEIPT_OPERATOR_FACING_SUMMARY_BRIEFING_NON_PERSISTENCE_DENIAL_BOUNDARY_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-operator-facing-summary-briefing-non-persistence-denial-boundary --json"
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
        value["memory_write_execution_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_ready"],
        true
    );
    assert_eq!(
        value["activation_command_result_receipt_operator_facing_summary_briefing_mode"],
        "memory_write_execution_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial"
    );
    assert_eq!(
        value["source_activation_command_result_receipt_export_query_observability_boundary_ready"],
        true
    );
    assert_eq!(
        value["source_activation_command_result_receipt_export_query_observability_ready"],
        true
    );
    assert_ne!(
        value["source_activation_command_result_receipt_export_query_observability_boundary_report_sha256"],
        ""
    );
    assert_eq!(
        value["memory_write_execution_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready"],
        true
    );
    assert_eq!(
        value["memory_write_execution_activation_command_result_receipt_export_query_observability_denial_ready"],
        true
    );
    assert_eq!(
        value["memory_write_execution_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready"],
        true
    );
    assert_eq!(
        value["memory_write_execution_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready"],
        true
    );
    assert_eq!(
        value["memory_write_execution_activation_command_result_receipt_cancellation_supersession_denial_ready"],
        true
    );
    assert_eq!(
        value["memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_ready"],
        true
    );
    assert_eq!(
        value["memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_ready"],
        true
    );
    assert_eq!(
        value["required_activation_command_result_receipt_operator_facing_summary_briefing_surface_count"],
        12
    );
    assert_eq!(
        value["ready_activation_command_result_receipt_operator_facing_summary_briefing_surface_count"],
        12
    );
    assert_eq!(
        value["side_effect_free_activation_command_result_receipt_operator_facing_summary_briefing_surface_count"],
        12
    );
    assert_eq!(
        value["activation_command_result_receipt_operator_facing_summary_briefing_fixture_count"],
        10
    );
    assert_eq!(
        value["blocked_activation_command_result_receipt_operator_facing_summary_briefing_fixture_count"],
        10
    );
    assert_eq!(
        value["noop_activation_command_result_receipt_operator_facing_summary_briefing_fixture_count"],
        10
    );
    assert_eq!(
        value["allowed_activation_command_result_receipt_operator_facing_summary_briefing_fixture_count"],
        0
    );
    assert_eq!(
        value["accepted_activation_command_result_receipt_operator_facing_summary_briefing_fixture_count"],
        0
    );
    assert_eq!(
        value["activation_command_result_receipt_operator_summary_denied_count"],
        10
    );
    assert_eq!(
        value["activation_command_result_receipt_operator_briefing_denied_count"],
        10
    );
    assert_eq!(
        value["activation_command_result_receipt_operator_summary_performed_count"],
        0
    );
    assert_eq!(
        value["activation_command_result_receipt_operator_briefing_performed_count"],
        0
    );
    assert_eq!(value["memory_store_write_performed_count"], 0);

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
        "activation_allowed_by_result_receipt_operator_summary",
        "activation_allowed_by_result_receipt_operator_briefing",
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
        "secret_material_read",
        "provider_prompt_replay_enabled",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "kg_adapter_read_performed",
        "live_kg_write_performed",
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
        assert_eq!(value[key], false, "{key}");
    }

    let surfaces =
        value["activation_command_result_receipt_operator_facing_summary_briefing_surfaces"]
            .as_array()
            .expect("activation command result receipt operator summary/briefing surfaces");
    assert_eq!(surfaces.len(), 12);
    let fixtures =
        value["activation_command_result_receipt_operator_facing_summary_briefing_fixtures"]
            .as_array()
            .expect("activation command result receipt operator summary/briefing fixtures");
    assert_eq!(fixtures.len(), 10);
    assert!(fixtures.iter().all(|fixture| {
        (fixture["operator_summary_briefing_status"].as_str() == Some("blocked_noop")
            || fixture["operator_summary_briefing_status"].as_str() == Some("blocked_summary_noop")
            || fixture["operator_summary_briefing_status"].as_str()
                == Some("blocked_briefing_noop")
            || fixture["operator_summary_briefing_status"].as_str()
                == Some("blocked_delivery_noop"))
            && fixture["operator_summary_recorded"].as_bool() == Some(false)
            && fixture["operator_summary_persisted"].as_bool() == Some(false)
            && fixture["operator_summary_materialized"].as_bool() == Some(false)
            && fixture["operator_summary_filesystem_written"].as_bool() == Some(false)
            && fixture["operator_summary_delivered"].as_bool() == Some(false)
            && fixture["operator_briefing_recorded"].as_bool() == Some(false)
            && fixture["operator_briefing_persisted"].as_bool() == Some(false)
            && fixture["operator_briefing_materialized"].as_bool() == Some(false)
            && fixture["operator_briefing_filesystem_written"].as_bool() == Some(false)
            && fixture["operator_briefing_delivered"].as_bool() == Some(false)
            && fixture["telegram_send_performed"].as_bool() == Some(false)
            && fixture["receipt_recorded"].as_bool() == Some(false)
            && fixture["receipt_persisted"].as_bool() == Some(false)
            && fixture["receipt_accepted"].as_bool() == Some(false)
            && fixture["activation_allowed"].as_bool() == Some(false)
            && fixture["live_mutation_execution_performed"].as_bool() == Some(false)
            && fixture["memory_store_write_performed"].as_bool() == Some(false)
            && fixture["memory_store_mutated"].as_bool() == Some(false)
            && fixture["rollback_executed"].as_bool() == Some(false)
            && fixture["receipt_noop_confirmed"].as_bool() == Some(true)
    }));
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["source_export_query_observability_present"] == false)
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["operator_summary_requested"] == true)
            .count(),
        7
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["operator_briefing_requested"] == true)
            .count(),
        6
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["channel_delivery_requested"] == true
                    && fixture["telegram_send_requested"] == true
            })
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["activation_from_summary_briefing_requested"] == true
                    && fixture["memory_write_summary_requested"] == true
                    && fixture["rollback_summary_requested"] == true
                    && fixture["provider_prompt_summary_requested"] == true
            })
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["external_send_summary_requested"] == true
                    && fixture["install_summary_requested"] == true
                    && fixture["active_binary_summary_requested"] == true
            })
            .count(),
        1
    );
    assert_eq!(
        value["denied_by_activation_command_result_receipt_operator_facing_summary_briefing_count"],
        20
    );
    assert_eq!(
        value["denied_by_activation_command_result_receipt_operator_facing_summary_briefing"]
            .as_array()
            .expect("activation command result receipt operator summary/briefing denials")
            .len(),
        20
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "run_memory_write_execution_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_require_live_gate"
    );
    assert_eq!(
        value["allowed_next_actions"][0]["delivers_notification"],
        false
    );
    assert_eq!(
        value["allowed_next_actions"][1]["action"],
        "prepare_memory_write_execution_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary"
    );
    assert_eq!(
        value["allowed_next_actions"][1]["accepts_operator_acknowledgement"],
        false
    );
    let side_effects = value["side_effects"].as_object().expect(
            "memory write execution activation command result receipt operator summary/briefing boundary side effects",
        );
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_write_execution_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_endpoint_blocks_acceptance_and_authority()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_COMMAND_RESULT_RECEIPT_FINAL_OPERATOR_ACKNOWLEDGEMENT_NON_ACCEPTANCE_DENIAL_BOUNDARY_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "memory write execution activation command result receipt final operator acknowledgement non-acceptance denial boundary json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_COMMAND_RESULT_RECEIPT_FINAL_OPERATOR_ACKNOWLEDGEMENT_NON_ACCEPTANCE_DENIAL_BOUNDARY_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-final-operator-acknowledgement-non-acceptance-denial-boundary --json"
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
        value["memory_write_execution_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_ready"],
        true
    );
    assert_eq!(
        value["activation_command_result_receipt_final_operator_acknowledgement_mode"],
        "memory_write_execution_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial"
    );
    assert_eq!(
        value["source_activation_command_result_receipt_operator_facing_summary_briefing_boundary_ready"],
        true
    );
    assert_eq!(
        value["source_activation_command_result_receipt_operator_facing_summary_briefing_ready"],
        true
    );
    assert_ne!(
        value["source_activation_command_result_receipt_operator_facing_summary_briefing_boundary_report_sha256"],
        ""
    );
    assert_eq!(
        value["memory_write_execution_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready"],
        true
    );
    assert_eq!(
        value["memory_write_execution_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready"],
        true
    );
    assert_eq!(
        value["memory_write_execution_activation_command_result_receipt_export_query_observability_denial_ready"],
        true
    );
    assert_eq!(
        value["memory_write_execution_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready"],
        true
    );
    assert_eq!(
        value["memory_write_execution_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready"],
        true
    );
    assert_eq!(
        value["memory_write_execution_activation_command_result_receipt_cancellation_supersession_denial_ready"],
        true
    );
    assert_eq!(
        value["memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_ready"],
        true
    );
    assert_eq!(
        value["memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_ready"],
        true
    );
    assert_eq!(
        value["source_operator_facing_summary_briefing_fixture_count"],
        10
    );
    assert_eq!(
        value["source_accepted_operator_facing_summary_briefing_fixture_count"],
        0
    );
    assert_eq!(value["source_operator_summary_performed_count"], 0);
    assert_eq!(value["source_operator_briefing_performed_count"], 0);
    assert_eq!(
        value["required_activation_command_result_receipt_final_operator_acknowledgement_surface_count"],
        12
    );
    assert_eq!(
        value["ready_activation_command_result_receipt_final_operator_acknowledgement_surface_count"],
        12
    );
    assert_eq!(
        value["side_effect_free_activation_command_result_receipt_final_operator_acknowledgement_surface_count"],
        12
    );
    assert_eq!(
        value["required_activation_command_result_receipt_final_operator_acknowledgement_fixture_count"],
        10
    );
    assert_eq!(
        value["activation_command_result_receipt_final_operator_acknowledgement_fixture_count"],
        10
    );
    assert_eq!(
        value["blocked_activation_command_result_receipt_final_operator_acknowledgement_fixture_count"],
        10
    );
    assert_eq!(
        value["noop_activation_command_result_receipt_final_operator_acknowledgement_fixture_count"],
        10
    );
    assert_eq!(
        value["allowed_activation_command_result_receipt_final_operator_acknowledgement_fixture_count"],
        0
    );
    assert_eq!(
        value["accepted_activation_command_result_receipt_final_operator_acknowledgement_fixture_count"],
        0
    );
    assert_eq!(
        value["activation_command_result_receipt_final_operator_acknowledgement_denied_count"],
        10
    );
    for key in [
        "activation_command_result_receipt_final_operator_acknowledgement_performed_count",
        "activation_command_result_receipt_final_operator_acknowledgement_recorded_count",
        "activation_command_result_receipt_final_operator_acknowledgement_persisted_count",
        "activation_command_result_receipt_final_operator_acknowledgement_delivered_count",
        "activation_command_result_receipt_final_operator_acknowledgement_final_state_promoted_count",
        "memory_store_write_performed_count",
    ] {
        assert_eq!(
            value[key], 0,
            "memory final operator acknowledgement count should stay zero: {key}"
        );
    }
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
    ] {
        assert_eq!(
            value[key], false,
            "memory final operator acknowledgement field should stay false: {key}"
        );
    }

    let surfaces =
        value["activation_command_result_receipt_final_operator_acknowledgement_surfaces"]
            .as_array()
            .expect("activation command result receipt final operator acknowledgement surfaces");
    assert_eq!(surfaces.len(), 12);
    let fixtures =
        value["activation_command_result_receipt_final_operator_acknowledgement_fixtures"]
            .as_array()
            .expect("activation command result receipt final operator acknowledgement fixtures");
    assert_eq!(fixtures.len(), 10);
    assert!(fixtures.iter().all(|fixture| {
        fixture["final_operator_acknowledgement_status"]
            .as_str()
            .is_some_and(|status| status.starts_with("blocked"))
            && fixture["acknowledgement_recorded"].as_bool() == Some(false)
            && fixture["acknowledgement_persisted"].as_bool() == Some(false)
            && fixture["acknowledgement_materialized"].as_bool() == Some(false)
            && fixture["acknowledgement_filesystem_written"].as_bool() == Some(false)
            && fixture["acknowledgement_delivered"].as_bool() == Some(false)
            && fixture["acknowledgement_accepted"].as_bool() == Some(false)
            && fixture["acknowledgement_final_state_promoted"].as_bool() == Some(false)
            && fixture["operator_final_acceptance_recorded"].as_bool() == Some(false)
            && fixture["operator_final_acceptance_persisted"].as_bool() == Some(false)
            && fixture["telegram_send_performed"].as_bool() == Some(false)
            && fixture["receipt_recorded"].as_bool() == Some(false)
            && fixture["receipt_persisted"].as_bool() == Some(false)
            && fixture["receipt_accepted"].as_bool() == Some(false)
            && fixture["activation_allowed"].as_bool() == Some(false)
            && fixture["live_mutation_execution_performed"].as_bool() == Some(false)
            && fixture["memory_store_write_performed"].as_bool() == Some(false)
            && fixture["memory_store_mutated"].as_bool() == Some(false)
            && fixture["rollback_executed"].as_bool() == Some(false)
            && fixture["provider_invoked"].as_bool() == Some(false)
            && fixture["model_invoked"].as_bool() == Some(false)
            && fixture["final_acknowledgement_noop_confirmed"].as_bool() == Some(true)
    }));
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["source_summary_briefing_present"] == false)
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["final_operator_acknowledgement_requested"] == true)
            .count(),
        10
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["acknowledgement_delivery_requested"] == true
                    && fixture["telegram_send_requested"] == true
                    && fixture["channel_delivery_requested"] == true
            })
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["final_state_promotion_requested"] == true
                    && fixture["completion_promotion_requested"] == true
            })
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["activation_from_acknowledgement_requested"] == true
                    && fixture["memory_write_acknowledgement_requested"] == true
                    && fixture["provider_prompt_acknowledgement_requested"] == true
            })
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["external_send_acknowledgement_requested"] == true
                    && fixture["release_artifact_acknowledgement_requested"] == true
                    && fixture["install_acknowledgement_requested"] == true
                    && fixture["active_binary_acknowledgement_requested"] == true
            })
            .count(),
        1
    );
    assert_eq!(
        value["source_operator_facing_summary_briefing_denial_count"],
        20
    );
    let denied =
        value["denied_by_activation_command_result_receipt_final_operator_acknowledgement"]
            .as_array()
            .expect("memory final operator acknowledgement denials");
    assert_eq!(denied.len(), 37);
    assert_eq!(
        value["denied_by_activation_command_result_receipt_final_operator_acknowledgement_count"],
        37
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "run_memory_write_execution_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_require_live_gate"
    );
    assert_eq!(
        value["allowed_next_actions"][0]["accepts_operator_acknowledgement"],
        false
    );
    assert_eq!(
        value["allowed_next_actions"][0]["promotes_final_state"],
        false
    );
    assert_eq!(
        value["allowed_next_actions"][1]["action"],
        "prepare_memory_write_execution_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary"
    );
    assert_eq!(
        value["allowed_next_actions"][1]["claims_public_release"],
        false
    );
    let side_effects = value["side_effects"].as_object().expect(
            "memory write execution activation command result receipt final operator acknowledgement boundary side effects",
        );
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_write_execution_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_endpoint_blocks_public_claim_and_authority()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_COMMAND_RESULT_RECEIPT_TERMINAL_OPERATOR_DECISION_PUBLIC_CLAIM_NON_PROMOTION_DENIAL_BOUNDARY_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "memory write execution activation command result receipt terminal operator decision public claim non-promotion denial boundary json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_COMMAND_RESULT_RECEIPT_TERMINAL_OPERATOR_DECISION_PUBLIC_CLAIM_NON_PROMOTION_DENIAL_BOUNDARY_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial-boundary --json"
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
        value["memory_write_execution_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_ready"],
        true
    );
    assert_eq!(
        value["activation_command_result_receipt_terminal_operator_decision_public_claim_mode"],
        "memory_write_execution_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial"
    );
    assert_eq!(
        value["source_activation_command_result_receipt_final_operator_acknowledgement_boundary_ready"],
        true
    );
    assert_eq!(
        value["source_activation_command_result_receipt_final_operator_acknowledgement_ready"],
        true
    );
    assert_ne!(
        value["source_activation_command_result_receipt_final_operator_acknowledgement_report_sha256"],
        ""
    );
    assert_eq!(
        value["source_final_operator_acknowledgement_fixture_count"],
        10
    );
    assert_eq!(
        value["source_blocked_final_operator_acknowledgement_fixture_count"],
        10
    );
    assert_eq!(
        value["source_accepted_final_operator_acknowledgement_fixture_count"],
        0
    );
    assert_eq!(
        value["source_final_operator_acknowledgement_performed_count"],
        0
    );
    assert_eq!(
        value["source_final_operator_acknowledgement_denial_count"],
        37
    );
    assert_eq!(
        value["memory_write_execution_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_ready"],
        true
    );
    assert_eq!(
        value["memory_write_execution_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready"],
        true
    );
    assert_eq!(
        value["memory_write_execution_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready"],
        true
    );
    assert_eq!(
        value["memory_write_execution_activation_command_result_receipt_export_query_observability_denial_ready"],
        true
    );
    assert_eq!(
        value["memory_write_execution_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready"],
        true
    );
    assert_eq!(
        value["memory_write_execution_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready"],
        true
    );
    assert_eq!(
        value["memory_write_execution_activation_command_result_receipt_cancellation_supersession_denial_ready"],
        true
    );
    assert_eq!(
        value["memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_ready"],
        true
    );
    assert_eq!(
        value["memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_ready"],
        true
    );
    assert_eq!(
        value["required_activation_command_result_receipt_terminal_operator_decision_public_claim_surface_count"],
        12
    );
    assert_eq!(
        value["ready_activation_command_result_receipt_terminal_operator_decision_public_claim_surface_count"],
        12
    );
    assert_eq!(
        value["side_effect_free_activation_command_result_receipt_terminal_operator_decision_public_claim_surface_count"],
        12
    );
    assert_eq!(
        value["required_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count"],
        10
    );
    assert_eq!(
        value["activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count"],
        10
    );
    assert_eq!(
        value["blocked_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count"],
        10
    );
    assert_eq!(
        value["noop_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count"],
        10
    );
    assert_eq!(
        value["allowed_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count"],
        0
    );
    assert_eq!(
        value["accepted_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count"],
        0
    );
    for key in [
        "activation_command_result_receipt_terminal_operator_decision_performed_count",
        "activation_command_result_receipt_public_claim_promotion_performed_count",
        "memory_store_write_performed_count",
    ] {
        assert_eq!(
            value[key], 0,
            "memory terminal decision/public claim count should stay zero: {key}"
        );
    }
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
        "telegram_send_performed",
        "channel_send_performed",
        "external_send_performed",
        "activation_allowed_by_result_receipt_terminal_operator_decision",
        "activation_allowed_by_result_receipt_final_operator_acknowledgement",
        "activation_allowed_by_result_receipt",
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
        "live_kg_write_performed",
        "public_release_published",
        "public_ga_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
    ] {
        assert_eq!(
            value[key], false,
            "memory terminal decision/public claim field should stay false: {key}"
        );
    }

    let surfaces =
        value["activation_command_result_receipt_terminal_operator_decision_public_claim_surfaces"]
            .as_array()
            .expect("activation command result receipt terminal decision/public claim surfaces");
    assert_eq!(surfaces.len(), 12);
    let fixtures =
        value["activation_command_result_receipt_terminal_operator_decision_public_claim_fixtures"]
            .as_array()
            .expect("activation command result receipt terminal decision/public claim fixtures");
    assert_eq!(fixtures.len(), 10);
    assert!(fixtures.iter().all(|fixture| {
        fixture["terminal_operator_decision_status"]
            .as_str()
            .is_some_and(|status| status.starts_with("blocked"))
            && fixture["terminal_decision_recorded"].as_bool() == Some(false)
            && fixture["terminal_decision_persisted"].as_bool() == Some(false)
            && fixture["terminal_decision_materialized"].as_bool() == Some(false)
            && fixture["terminal_decision_filesystem_written"].as_bool() == Some(false)
            && fixture["terminal_decision_delivered"].as_bool() == Some(false)
            && fixture["terminal_decision_accepted"].as_bool() == Some(false)
            && fixture["terminal_decision_final_state_promoted"].as_bool() == Some(false)
            && fixture["public_claim_promoted"].as_bool() == Some(false)
            && fixture["public_release_published"].as_bool() == Some(false)
            && fixture["public_ga_claimed"].as_bool() == Some(false)
            && fixture["telegram_send_performed"].as_bool() == Some(false)
            && fixture["receipt_recorded"].as_bool() == Some(false)
            && fixture["receipt_persisted"].as_bool() == Some(false)
            && fixture["receipt_accepted"].as_bool() == Some(false)
            && fixture["activation_allowed"].as_bool() == Some(false)
            && fixture["live_mutation_execution_performed"].as_bool() == Some(false)
            && fixture["memory_store_write_performed"].as_bool() == Some(false)
            && fixture["memory_store_mutated"].as_bool() == Some(false)
            && fixture["rollback_executed"].as_bool() == Some(false)
            && fixture["provider_invoked"].as_bool() == Some(false)
            && fixture["model_invoked"].as_bool() == Some(false)
            && fixture["terminal_operator_decision_noop_confirmed"].as_bool() == Some(true)
    }));
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["source_final_acknowledgement_present"] == false)
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["terminal_operator_decision_requested"] == true)
            .count(),
        10
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["public_claim_requested"] == true
                    && fixture["public_claim_promotion_requested"] == true
            })
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["public_ga_claim_requested"] == true
                    && fixture["public_release_publish_requested"] == true
                    && fixture["release_artifact_write_requested"] == true
            })
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["activation_from_terminal_decision_requested"] == true
                    && fixture["memory_write_terminal_decision_requested"] == true
                    && fixture["provider_prompt_terminal_decision_requested"] == true
            })
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["external_send_decision_requested"] == true
                    && fixture["release_artifact_decision_requested"] == true
                    && fixture["install_decision_requested"] == true
                    && fixture["active_binary_decision_requested"] == true
            })
            .count(),
        1
    );
    let denied =
            value["denied_by_activation_command_result_receipt_terminal_operator_decision_public_claim"]
                .as_array()
                .expect("memory terminal decision public claim denials");
    assert_eq!(denied.len(), 55);
    assert_eq!(
        value["denied_by_activation_command_result_receipt_terminal_operator_decision_public_claim_count"],
        55
    );
    assert_eq!(
        value["terminal_operator_decision_acceptance_forbidden"],
        true
    );
    assert_eq!(value["public_claim_promotion_forbidden"], true);
    assert_eq!(value["public_release_publication_forbidden"], true);
    assert_eq!(value["release_artifact_publication_forbidden"], true);
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "run_memory_write_execution_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_require_live_gate"
    );
    assert_eq!(
        value["allowed_next_actions"][0]["accepts_terminal_decision"],
        false
    );
    assert_eq!(
        value["allowed_next_actions"][0]["promotes_public_claim"],
        false
    );
    assert_eq!(
        value["allowed_next_actions"][1]["action"],
        "prepare_memory_write_execution_activation_command_result_receipt_release_artifact_publication_denial_boundary"
    );
    assert_eq!(
        value["allowed_next_actions"][1]["publishes_release_artifact"],
        false
    );
    let side_effects = value["side_effects"].as_object().expect(
            "memory write execution activation command result receipt terminal decision public claim boundary side effects",
        );
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_write_execution_activation_command_result_receipt_release_artifact_publication_denial_boundary_endpoint_blocks_publication_and_authority()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_COMMAND_RESULT_RECEIPT_RELEASE_ARTIFACT_PUBLICATION_DENIAL_BOUNDARY_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "memory write execution activation command result receipt release artifact publication denial boundary json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_COMMAND_RESULT_RECEIPT_RELEASE_ARTIFACT_PUBLICATION_DENIAL_BOUNDARY_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-release-artifact-publication-denial-boundary --json"
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
        value["memory_write_execution_activation_command_result_receipt_release_artifact_publication_denial_boundary_ready"],
        true
    );
    assert_eq!(
        value["activation_command_result_receipt_release_artifact_publication_mode"],
        "memory_write_execution_activation_command_result_receipt_release_artifact_publication_denial"
    );
    assert_eq!(
        value["source_activation_command_result_receipt_terminal_operator_decision_public_claim_boundary_ready"],
        true
    );
    assert_eq!(
        value["source_activation_command_result_receipt_terminal_operator_decision_public_claim_ready"],
        true
    );
    assert_ne!(
        value["source_activation_command_result_receipt_terminal_operator_decision_public_claim_report_sha256"],
        ""
    );
    assert_eq!(
        value["source_terminal_operator_decision_public_claim_fixture_count"],
        10
    );
    assert_eq!(
        value["source_accepted_terminal_operator_decision_public_claim_fixture_count"],
        0
    );
    assert_eq!(
        value["source_terminal_operator_decision_performed_count"],
        0
    );
    assert_eq!(value["source_public_claim_promotion_performed_count"], 0);
    assert_eq!(
        value["source_terminal_operator_decision_public_claim_denial_count"],
        55
    );
    assert_eq!(
        value["memory_write_execution_activation_command_result_receipt_release_artifact_publication_denial_ready"],
        true
    );
    assert_eq!(
        value["memory_write_execution_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_ready"],
        true
    );
    assert_eq!(
        value["memory_write_execution_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready"],
        true
    );
    assert_eq!(
        value["memory_write_execution_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready"],
        true
    );
    assert_eq!(
        value["required_activation_command_result_receipt_release_artifact_publication_surface_count"],
        12
    );
    assert_eq!(
        value["ready_activation_command_result_receipt_release_artifact_publication_surface_count"],
        12
    );
    assert_eq!(
        value["side_effect_free_activation_command_result_receipt_release_artifact_publication_surface_count"],
        12
    );
    assert_eq!(
        value["required_activation_command_result_receipt_release_artifact_publication_fixture_count"],
        10
    );
    assert_eq!(
        value["activation_command_result_receipt_release_artifact_publication_fixture_count"],
        10
    );
    assert_eq!(
        value["blocked_activation_command_result_receipt_release_artifact_publication_fixture_count"],
        10
    );
    assert_eq!(
        value["noop_activation_command_result_receipt_release_artifact_publication_fixture_count"],
        10
    );
    assert_eq!(
        value["allowed_activation_command_result_receipt_release_artifact_publication_fixture_count"],
        0
    );
    assert_eq!(
        value["accepted_activation_command_result_receipt_release_artifact_publication_fixture_count"],
        0
    );
    for key in [
        "activation_command_result_receipt_release_artifact_publication_performed_count",
        "release_artifact_written_count",
        "public_artifact_written_count",
        "public_distribution_performed_count",
        "publication_manifest_written_count",
        "publication_queue_enqueued_count",
        "memory_store_write_performed_count",
    ] {
        assert_eq!(
            value[key], 0,
            "memory release artifact publication count should stay zero: {key}"
        );
    }
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
        "activation_allowed_by_release_artifact_publication",
        "activation_allowed_by_terminal_operator_decision",
        "activation_allowed_by_result_receipt",
        "activation_allowed",
        "activation_performed",
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
        "install_executed",
        "launchd_mutated",
        "service_restart_performed",
        "service_restarted",
        "active_binary_mutated",
    ] {
        assert_eq!(
            value[key], false,
            "memory release artifact publication field should stay false: {key}"
        );
    }

    let fixtures = value["activation_command_result_receipt_release_artifact_publication_fixtures"]
        .as_array()
        .expect("memory release artifact publication fixtures");
    assert_eq!(fixtures.len(), 10);
    assert!(fixtures.iter().all(|fixture| {
        fixture["release_artifact_publication_status"]
            .as_str()
            .is_some_and(|status| status.starts_with("blocked"))
            && fixture["release_artifact_publication_allowed"].as_bool() == Some(false)
            && fixture["release_artifact_publication_accepted"].as_bool() == Some(false)
            && fixture["release_artifact_publication_recorded"].as_bool() == Some(false)
            && fixture["release_artifact_publication_persisted"].as_bool() == Some(false)
            && fixture["release_artifact_filesystem_written"].as_bool() == Some(false)
            && fixture["release_artifact_written"].as_bool() == Some(false)
            && fixture["public_artifact_written"].as_bool() == Some(false)
            && fixture["publication_queue_enqueued"].as_bool() == Some(false)
            && fixture["publication_manifest_written"].as_bool() == Some(false)
            && fixture["public_distribution_performed"].as_bool() == Some(false)
            && fixture["public_release_published"].as_bool() == Some(false)
            && fixture["public_ga_claimed"].as_bool() == Some(false)
            && fixture["public_claim_promoted"].as_bool() == Some(false)
            && fixture["telegram_send_performed"].as_bool() == Some(false)
            && fixture["channel_send_performed"].as_bool() == Some(false)
            && fixture["external_send_performed"].as_bool() == Some(false)
            && fixture["receipt_recorded"].as_bool() == Some(false)
            && fixture["receipt_persisted"].as_bool() == Some(false)
            && fixture["receipt_accepted"].as_bool() == Some(false)
            && fixture["activation_allowed"].as_bool() == Some(false)
            && fixture["live_mutation_execution_performed"].as_bool() == Some(false)
            && fixture["memory_store_write_performed"].as_bool() == Some(false)
            && fixture["memory_store_mutated"].as_bool() == Some(false)
            && fixture["rollback_executed"].as_bool() == Some(false)
            && fixture["release_artifact_publication_noop_confirmed"].as_bool() == Some(true)
    }));
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["source_terminal_operator_decision_present"] == false)
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["release_artifact_publication_requested"] == true)
            .count(),
        5
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["release_artifact_write_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["public_artifact_write_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["artifact_signature_requested"] == true
                && fixture["artifact_notarization_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(
                |fixture| fixture["service_restart_publication_requested"] == true
                    && fixture["active_binary_publication_requested"] == true
            )
            .count(),
        1
    );

    let denied = value["denied_by_activation_command_result_receipt_release_artifact_publication"]
        .as_array()
        .expect("memory release artifact publication denials");
    assert_eq!(denied.len(), 69);
    assert_eq!(
        value["denied_by_activation_command_result_receipt_release_artifact_publication_count"],
        69
    );
    assert_eq!(value["release_artifact_publication_forbidden"], true);
    assert_eq!(value["release_artifact_write_forbidden"], true);
    assert_eq!(value["public_artifact_write_forbidden"], true);
    assert_eq!(value["public_distribution_forbidden"], true);
    assert_eq!(value["public_release_publication_forbidden"], true);
    assert_eq!(value["public_ga_claim_forbidden"], true);
    assert_eq!(
        value["terminal_operator_decision_release_approval_forbidden"],
        true
    );
    assert_eq!(
        value["activation_from_release_artifact_publication_forbidden"],
        true
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "run_memory_write_execution_activation_command_result_receipt_release_artifact_publication_denial_boundary_require_live_gate"
    );
    assert_eq!(
        value["allowed_next_actions"][0]["publishes_release_artifact"],
        false
    );
    assert_eq!(
        value["allowed_next_actions"][0]["writes_release_artifact"],
        false
    );
    assert_eq!(
        value["allowed_next_actions"][1]["action"],
        "prepare_scoped_memory_real_write_canary_operator_approval_packet"
    );
    assert_eq!(
        value["allowed_next_actions"][1]["requires_explicit_operator_acceptance"],
        true
    );
    let side_effects = value["side_effects"].as_object().expect(
            "memory write execution activation command result receipt release artifact publication side effects",
        );
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_write_execution_scoped_memory_real_write_canary_operator_approval_packet_nonce_command_dry_run_boundary_blocks_real_write()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_MEMORY_REAL_WRITE_CANARY_OPERATOR_APPROVAL_PACKET_NONCE_COMMAND_DRY_RUN_BOUNDARY_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "scoped memory real-write canary operator approval packet nonce command dry-run boundary json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_MEMORY_REAL_WRITE_CANARY_OPERATOR_APPROVAL_PACKET_NONCE_COMMAND_DRY_RUN_BOUNDARY_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-live-mutation-operator-write-execution-scoped-memory-real-write-canary-operator-approval-packet-nonce-command-dry-run-boundary --json"
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
        value["memory_write_execution_scoped_memory_real_write_canary_operator_approval_packet_nonce_command_dry_run_boundary_ready"],
        true
    );
    assert_eq!(
        value["scoped_memory_real_write_canary_operator_approval_packet_nonce_command_dry_run_ready"],
        true
    );
    assert_eq!(
        value["scoped_memory_real_write_canary_mode"],
        "scoped_memory_real_write_canary_operator_approval_packet_nonce_command_dry_run_no_write"
    );
    assert_eq!(
        value["source_activation_command_result_receipt_release_artifact_publication_boundary_ready"],
        true
    );
    assert_eq!(
        value["source_activation_command_result_receipt_release_artifact_publication_ready"],
        true
    );
    assert_ne!(
        value["source_activation_command_result_receipt_release_artifact_publication_report_sha256"],
        ""
    );
    assert_eq!(
        value["source_release_artifact_publication_fixture_count"],
        10
    );
    assert_eq!(
        value["source_accepted_release_artifact_publication_fixture_count"],
        0
    );
    assert_eq!(
        value["source_release_artifact_publication_performed_count"],
        0
    );
    assert_eq!(
        value["source_release_artifact_publication_denial_count"],
        69
    );
    assert_eq!(value["minimum_required_samples"], 24);
    assert_eq!(
        value["required_scoped_memory_real_write_canary_operator_approval_surface_count"],
        12
    );
    assert_eq!(
        value["ready_scoped_memory_real_write_canary_operator_approval_surface_count"],
        12
    );
    assert_eq!(
        value["side_effect_free_scoped_memory_real_write_canary_operator_approval_surface_count"],
        12
    );
    assert_eq!(
        value["required_scoped_memory_real_write_canary_operator_approval_fixture_count"],
        10
    );
    assert_eq!(
        value["scoped_memory_real_write_canary_operator_approval_fixture_count"],
        10
    );
    assert_eq!(
        value["blocked_scoped_memory_real_write_canary_operator_approval_fixture_count"],
        10
    );
    assert_eq!(
        value["noop_scoped_memory_real_write_canary_operator_approval_fixture_count"],
        10
    );
    assert_eq!(
        value["allowed_scoped_memory_real_write_canary_operator_approval_fixture_count"],
        0
    );
    assert_eq!(
        value["accepted_scoped_memory_real_write_canary_operator_approval_fixture_count"],
        0
    );
    for key in [
        "scoped_memory_real_write_canary_approval_packet_accepted_count",
        "single_use_nonce_consumed_count",
        "explicit_command_dispatched_count",
        "wal_write_performed_count",
        "receipt_persisted_count",
        "post_write_readback_performed_count",
        "rollback_tombstone_performed_count",
        "memory_store_write_performed_count",
    ] {
        assert_eq!(
            value[key], 0,
            "scoped Memory canary dry-run count should stay zero: {key}"
        );
    }
    assert_eq!(
        value["required_before_scoped_memory_real_write_canary_acceptance_count"],
        18
    );
    assert_eq!(
        value["required_scoped_memory_real_write_canary_operator_approval_fields"]
            .as_array()
            .expect("required scoped Memory canary approval fields")
            .len(),
        18
    );

    for key in [
        "fresh_operator_approval_packet_recorded",
        "fresh_operator_approval_packet_persisted",
        "fresh_operator_approval_packet_accepted",
        "operator_identity_bound",
        "operator_session_bound",
        "operator_signature_verified",
        "single_use_nonce_issued",
        "single_use_nonce_consumed",
        "explicit_command_accepted",
        "explicit_command_dispatched",
        "explicit_command_performed",
        "canary_scope_bound",
        "canary_namespace_bound",
        "canary_store_bound",
        "payload_digest_bound",
        "active_binary_sha_bound",
        "route_count_bound",
        "fresh_long_soak_evidence_accepted",
        "wal_receipt_plan_accepted",
        "wal_write_performed",
        "receipt_recorded",
        "receipt_persisted",
        "receipt_accepted",
        "post_write_readback_plan_accepted",
        "post_write_readback_performed",
        "rollback_tombstone_plan_accepted",
        "rollback_executed",
        "tombstone_written",
        "activation_allowed",
        "activation_performed",
        "live_mutation_execution_performed",
        "memory_write_execution_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "durable_memory_store_write_performed",
        "durable_memory_store_read_performed",
        "durable_memory_store_rollback_performed",
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
        "active_binary_mutated",
    ] {
        assert_eq!(
            value[key], false,
            "scoped Memory canary dry-run field should stay false: {key}"
        );
    }

    let surfaces = value["scoped_memory_real_write_canary_operator_approval_surfaces"]
        .as_array()
        .expect("scoped Memory canary approval surfaces");
    assert_eq!(surfaces.len(), 12);
    let fixtures = value["scoped_memory_real_write_canary_operator_approval_fixtures"]
        .as_array()
        .expect("scoped Memory canary approval fixtures");
    assert_eq!(fixtures.len(), 10);
    assert!(fixtures.iter().all(|fixture| {
        fixture["scoped_memory_real_write_canary_status"]
            .as_str()
            .is_some_and(|status| status.starts_with("blocked"))
            && fixture["approval_packet_accepted"].as_bool() == Some(false)
            && fixture["single_use_nonce_consumed"].as_bool() == Some(false)
            && fixture["explicit_command_dispatched"].as_bool() == Some(false)
            && fixture["receipt_persisted"].as_bool() == Some(false)
            && fixture["post_write_readback_performed"].as_bool() == Some(false)
            && fixture["rollback_executed"].as_bool() == Some(false)
            && fixture["memory_store_write_performed"].as_bool() == Some(false)
            && fixture["memory_store_mutated"].as_bool() == Some(false)
            && fixture["durable_memory_store_write_performed"].as_bool() == Some(false)
            && fixture["live_kg_write_performed"].as_bool() == Some(false)
            && fixture["provider_invoked"].as_bool() == Some(false)
            && fixture["model_invoked"].as_bool() == Some(false)
            && fixture["credential_read"].as_bool() == Some(false)
            && fixture["channel_send_performed"].as_bool() == Some(false)
            && fixture["external_send_performed"].as_bool() == Some(false)
            && fixture["release_artifact_written"].as_bool() == Some(false)
            && fixture["install_executed"].as_bool() == Some(false)
            && fixture["active_binary_mutated"].as_bool() == Some(false)
            && fixture["scoped_canary_dry_run_noop_confirmed"].as_bool() == Some(true)
    }));
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["source_release_artifact_publication_denial_present"] == false
            })
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["nonce_consume_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["explicit_command_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["durable_memory_write_requested"] == true)
            .count(),
        1
    );
    let denied = value
            ["denied_by_scoped_memory_real_write_canary_operator_approval_packet_nonce_command_dry_run"]
            .as_array()
            .expect("scoped Memory canary approval denials");
    assert_eq!(denied.len(), 32);
    assert_eq!(
        value["denied_by_scoped_memory_real_write_canary_operator_approval_packet_nonce_command_dry_run_count"],
        32
    );
    assert_eq!(value["fresh_operator_approval_packet_required"], true);
    assert_eq!(value["single_use_nonce_required"], true);
    assert_eq!(value["explicit_command_required"], true);
    assert_eq!(value["durable_memory_write_forbidden"], true);
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
        "run_scoped_memory_real_write_canary_operator_approval_packet_nonce_command_dry_run_boundary_require_live_gate"
    );
    assert_eq!(
        value["allowed_next_actions"][0]["accepts_operator_approval_packet"],
        false
    );
    assert_eq!(value["allowed_next_actions"][0]["consumes_nonce"], false);
    assert_eq!(
        value["allowed_next_actions"][0]["dispatches_command"],
        false
    );
    assert_eq!(
        value["allowed_next_actions"][1]["action"],
        "prepare_scoped_memory_real_write_canary_readback_rollback_tombstone_dry_run_boundary"
    );
    let side_effects = value["side_effects"]
        .as_object()
        .expect("scoped Memory canary dry-run side effects");
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_write_execution_scoped_memory_real_write_canary_readback_validation_dry_run_boundary_blocks_reads_writes_and_rollback()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_MEMORY_REAL_WRITE_CANARY_READBACK_VALIDATION_DRY_RUN_BOUNDARY_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body)
        .expect("scoped Memory real-write canary readback validation dry-run boundary json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_MEMORY_REAL_WRITE_CANARY_READBACK_VALIDATION_DRY_RUN_BOUNDARY_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-live-mutation-operator-write-execution-scoped-memory-real-write-canary-readback-validation-dry-run-boundary --json"
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
        value["memory_write_execution_scoped_memory_real_write_canary_readback_validation_dry_run_boundary_ready"],
        true
    );
    assert_eq!(
        value["scoped_memory_real_write_canary_readback_validation_dry_run_ready"],
        true
    );
    assert_eq!(
        value["source_scoped_memory_real_write_canary_operator_approval_packet_nonce_command_dry_run_boundary_ready"],
        true
    );
    assert_eq!(
        value["source_scoped_memory_real_write_canary_operator_approval_packet_nonce_command_dry_run_ready"],
        true
    );
    assert_ne!(
        value["source_scoped_memory_real_write_canary_operator_approval_packet_nonce_command_report_sha256"],
        ""
    );
    assert_eq!(
        value["source_scoped_memory_real_write_canary_operator_approval_fixture_count"],
        10
    );
    assert_eq!(
        value["source_accepted_scoped_memory_real_write_canary_operator_approval_fixture_count"],
        0
    );
    assert_eq!(
        value["source_scoped_memory_real_write_canary_approval_packet_accepted_count"],
        0
    );
    assert_eq!(value["source_single_use_nonce_consumed_count"], 0);
    assert_eq!(value["source_explicit_command_dispatched_count"], 0);
    assert_eq!(value["source_post_write_readback_performed_count"], 0);
    assert_eq!(value["source_memory_store_write_performed_count"], 0);
    assert_eq!(
        value["scoped_memory_real_write_canary_mode"],
        "scoped_memory_real_write_canary_readback_validation_dry_run_no_read_no_write"
    );
    assert_eq!(
        value["required_scoped_memory_real_write_canary_readback_surface_count"],
        12
    );
    assert_eq!(
        value["ready_scoped_memory_real_write_canary_readback_surface_count"],
        12
    );
    assert_eq!(
        value["side_effect_free_scoped_memory_real_write_canary_readback_surface_count"],
        12
    );
    assert_eq!(
        value["required_scoped_memory_real_write_canary_readback_fixture_count"],
        10
    );
    assert_eq!(
        value["scoped_memory_real_write_canary_readback_fixture_count"],
        10
    );
    assert_eq!(
        value["blocked_scoped_memory_real_write_canary_readback_fixture_count"],
        10
    );
    assert_eq!(
        value["accepted_scoped_memory_real_write_canary_readback_fixture_count"],
        0
    );
    for key in [
        "readback_plan_accepted_count",
        "readback_performed_count",
        "readback_result_recorded_count",
        "readback_result_persisted_count",
        "readback_result_accepted_count",
        "readback_payload_digest_compared_count",
        "readback_redaction_proof_accepted_count",
        "readback_secret_plaintext_scan_performed_count",
        "durable_memory_store_read_performed_count",
        "memory_store_write_performed_count",
        "rollback_tombstone_handoff_accepted_count",
    ] {
        assert_eq!(
            value[key], 0,
            "scoped Memory canary readback dry-run count should stay zero: {key}"
        );
    }
    assert_eq!(
        value["required_before_scoped_memory_real_write_canary_readback_acceptance_count"],
        16
    );
    assert_eq!(
        value["required_scoped_memory_real_write_canary_readback_fields"]
            .as_array()
            .expect("required scoped Memory canary readback fields")
            .len(),
        16
    );
    let fixtures = value["scoped_memory_real_write_canary_readback_fixtures"]
        .as_array()
        .expect("scoped Memory canary readback fixtures");
    assert_eq!(fixtures.len(), 10);
    assert!(fixtures.iter().all(|fixture| {
        fixture["scoped_memory_real_write_canary_readback_status"]
            .as_str()
            .is_some_and(|status| status.starts_with("blocked"))
            && fixture["post_write_readback_performed"].as_bool() == Some(false)
            && fixture["readback_result_recorded"].as_bool() == Some(false)
            && fixture["readback_result_persisted"].as_bool() == Some(false)
            && fixture["durable_memory_store_read_performed"].as_bool() == Some(false)
            && fixture["durable_memory_store_write_performed"].as_bool() == Some(false)
            && fixture["rollback_executed"].as_bool() == Some(false)
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
            && fixture["scoped_canary_readback_dry_run_noop_confirmed"].as_bool() == Some(true)
    }));
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["durable_memory_read_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["rollback_tombstone_handoff_requested"] == true)
            .count(),
        1
    );
    let denied = value["denied_by_scoped_memory_real_write_canary_readback_validation_dry_run"]
        .as_array()
        .expect("scoped Memory canary readback denials");
    assert_eq!(denied.len(), 26);
    assert_eq!(
        value["denied_by_scoped_memory_real_write_canary_readback_validation_dry_run_count"],
        26
    );
    for key in [
        "fresh_operator_approval_packet_accepted",
        "single_use_nonce_consumed",
        "explicit_command_dispatched",
        "receipt_persisted",
        "post_write_readback_plan_accepted",
        "post_write_readback_performed",
        "readback_result_recorded",
        "readback_result_persisted",
        "readback_result_accepted",
        "readback_payload_digest_compared",
        "readback_payload_digest_matched",
        "readback_redaction_proof_accepted",
        "readback_secret_plaintext_scan_performed",
        "readback_secret_plaintext_found",
        "rollback_tombstone_handoff_accepted",
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
            "scoped Memory canary readback dry-run field should stay false: {key}"
        );
    }
    assert_eq!(value["post_write_readback_plan_required"], true);
    assert_eq!(value["receipt_linkage_required"], true);
    assert_eq!(value["payload_digest_comparison_required"], true);
    assert_eq!(value["redaction_secret_scan_required"], true);
    assert_eq!(value["rollback_tombstone_handoff_required"], true);
    assert_eq!(value["durable_memory_read_forbidden"], true);
    assert_eq!(value["durable_memory_write_forbidden"], true);
    assert_eq!(value["rollback_execution_forbidden"], true);
    assert_eq!(
        value["allowed_next_actions"][1]["action"],
        "prepare_scoped_memory_real_write_canary_rollback_tombstone_dry_run_boundary"
    );
    let side_effects = value["side_effects"]
        .as_object()
        .expect("scoped Memory canary readback dry-run side effects");
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}
