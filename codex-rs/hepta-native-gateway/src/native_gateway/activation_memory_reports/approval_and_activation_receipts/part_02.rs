fn hepta_memory_live_mutation_operator_write_approval_packet_boundary_report() -> serde_json::Value
{
    let route_matrix = control_ui_route_parity_report();
    let closure_index = hepta_full_live_activation_closure_index_report();
    let minimal_canary =
        hepta_minimal_memory_canary_scoped_operator_packet_write_readback_rollback_idempotency_receipt_report();
    let durable_boundary = hepta_scoped_memory_canary_durable_receipt_boundary_report();
    let truth_index = hepta_memory_intelligence_kg_activation_truth_index_report();

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
    let closure_index_ready = json_bool(&closure_index, "full_live_activation_closure_index_ready")
        && json_u64(&closure_index, "closure_source_count") == 8
        && json_u64(&closure_index, "ready_closure_source_count") == 8
        && json_u64(&closure_index, "closure_blocker_count") == 13
        && json_u64(
            &closure_index,
            "accepted_unrestricted_activation_blocker_count",
        ) == 0
        && json_u64(
            &closure_index,
            "remaining_unrestricted_activation_blocker_count",
        ) == 13
        && !json_bool(&closure_index, "unrestricted_full_live_activation_enabled")
        && !json_bool(&closure_index, "unrestricted_full_live_activation_allowed")
        && side_effects_all_false(&closure_index);
    let minimal_canary_ready = json_bool(&minimal_canary, "minimal_memory_canary_ready")
        && json_bool(
            &minimal_canary,
            "scoped_operator_packet_accepted_for_ephemeral_canary",
        )
        && json_bool(&minimal_canary, "ephemeral_memory_store_write_performed")
        && json_bool(&minimal_canary, "ephemeral_memory_readback_performed")
        && json_bool(&minimal_canary, "ephemeral_memory_rollback_performed")
        && json_bool(&minimal_canary, "idempotency_receipt_generated")
        && !json_bool(&minimal_canary, "durable_memory_store_write_performed")
        && !json_bool(&minimal_canary, "memory_store_mutated")
        && side_effects_all_false(&minimal_canary);
    let durable_boundary_ready = json_bool(
        &durable_boundary,
        "scoped_memory_canary_durable_receipt_boundary_ready",
    ) && json_u64(
        &durable_boundary,
        "accepted_durable_receipt_candidate_count",
    ) == 0
        && !json_bool(&durable_boundary, "durable_receipt_accepted")
        && !json_bool(&durable_boundary, "durable_memory_store_write_performed")
        && side_effects_all_false(&durable_boundary);
    let truth_index_ready = json_bool(&truth_index, "hepta_core_connected")
        && json_bool(&truth_index, "hepta_core_full_fusion_complete")
        && json_bool(&truth_index, "operator_approved_lanes_ready")
        && json_bool(&truth_index, "full_live_activation_blocked")
        && !json_bool(&truth_index, "full_live_activation_enabled")
        && json_bool(&truth_index, "explicit_command_required_for_execution")
        && json_bool(&truth_index, "readiness_index_side_effects_all_false")
        && side_effects_all_false(&truth_index);

    let required_fields = vec![
        "approval_packet_id",
        "memory_write_request_id",
        "operator_approval_id",
        "operator_identity_hash",
        "operator_approval_signature_hash",
        "operator_approval_captured_at_unix",
        "single_surface_activation_scope",
        "memory_namespace",
        "memory_write_operation",
        "memory_retention_class",
        "record_intent",
        "raw_payload_sha256",
        "redacted_payload_summary_sha256",
        "accepted_redaction_proof_id",
        "source_full_live_activation_closure_index_sha256",
        "source_minimal_memory_canary_report_sha256",
        "source_scoped_memory_canary_durable_receipt_boundary_sha256",
        "fresh_pre_activation_soak_evidence_id",
        "rollback_plan_id",
        "post_write_validation_plan_id",
        "no_public_claim_no_external_send_decision",
    ];
    let inherited_allowed_memory_write_operations = vec![
        "append_daily_memory_note",
        "append_project_memory_note",
        "promote_long_term_memory_summary",
        "redact_or_supersede_memory_record",
    ];
    let required_before_acceptance = vec![
        "operator_approval_id",
        "operator_identity_hash",
        "operator_approval_signature_hash",
        "operator_approval_timestamp",
        "single_surface_activation_scope",
        "allowed_memory_write_operation",
        "accepted_redaction_proof_id",
        "source_full_live_activation_closure_index_hash_binding",
        "source_minimal_memory_canary_hash_binding",
        "source_scoped_memory_canary_durable_receipt_hash_binding",
        "fresh_pre_activation_soak_evidence_id",
        "rollback_plan_id",
        "post_write_validation_plan_id",
        "no_public_claim_no_external_send_decision",
    ];
    let denied_by = vec![
        "memory_write_approval_packet_not_recorded",
        "memory_write_approval_packet_not_persisted",
        "memory_write_approval_packet_not_accepted",
        "memory_write_request_not_recorded",
        "operator_approval_not_recorded",
        "operator_identity_hash_not_recorded",
        "operator_approval_signature_hash_not_recorded",
        "single_surface_activation_scope_not_recorded",
        "memory_namespace_not_recorded",
        "memory_write_operation_not_recorded",
        "memory_write_operation_not_allowed",
        "accepted_redaction_proof_not_recorded",
        "source_full_live_activation_closure_index_hash_not_bound",
        "source_minimal_memory_canary_hash_not_bound",
        "source_scoped_memory_canary_durable_receipt_hash_not_bound",
        "fresh_pre_activation_soak_evidence_not_recorded",
        "rollback_plan_not_recorded",
        "post_write_validation_plan_not_recorded",
        "no_public_claim_no_external_send_decision_not_recorded",
        "raw_payload_plaintext_recording_denied",
        "memory_store_mutation_denied",
        "external_send_public_claim_release_artifact_denied",
    ];
    let denied_fixtures = vec![
        serde_json::json!({
            "id": "empty-memory-write-approval-packet",
            "recorded_memory_write_approval_packet_field_count": 0,
            "packet_accepted": false,
            "memory_write_request_accepted": false,
            "memory_store_mutated": false,
            "activation_allowed": false,
            "reason": "approval_packet_not_recorded"
        }),
        serde_json::json!({
            "id": "operator-approval-without-identity-signature",
            "recorded_memory_write_approval_packet_field_count": 7,
            "operator_approval_recorded": true,
            "operator_identity_hash_recorded": false,
            "operator_approval_signature_hash_recorded": false,
            "packet_accepted": false,
            "memory_write_request_accepted": false,
            "memory_store_mutated": false,
            "activation_allowed": false,
            "reason": "operator_identity_and_signature_required"
        }),
        serde_json::json!({
            "id": "disallowed-memory-write-operation",
            "recorded_memory_write_approval_packet_field_count": 12,
            "memory_write_operation": "raw_secret_or_credential_persistence",
            "memory_write_operation_allowed": false,
            "packet_accepted": false,
            "memory_write_request_accepted": false,
            "memory_store_mutated": false,
            "activation_allowed": false,
            "reason": "memory_write_operation_not_in_allowlist"
        }),
        serde_json::json!({
            "id": "memory-write-without-accepted-redaction-proof",
            "recorded_memory_write_approval_packet_field_count": 15,
            "accepted_redaction_proof_recorded": false,
            "accepted_redaction_proof_count": 0,
            "packet_accepted": false,
            "memory_write_request_accepted": false,
            "memory_store_mutated": false,
            "activation_allowed": false,
            "reason": "accepted_redaction_proof_required"
        }),
        serde_json::json!({
            "id": "memory-write-without-fresh-soak-rollback-or-validation",
            "recorded_memory_write_approval_packet_field_count": 18,
            "fresh_pre_activation_soak_evidence_recorded": false,
            "rollback_plan_recorded": false,
            "post_write_validation_plan_recorded": false,
            "packet_accepted": false,
            "memory_write_request_accepted": false,
            "memory_store_mutated": false,
            "activation_allowed": false,
            "reason": "fresh_soak_rollback_and_validation_required"
        }),
        serde_json::json!({
            "id": "raw-secret-marker-memory-approval-packet",
            "recorded_memory_write_approval_packet_field_count": 21,
            "raw_secret_marker_detected": true,
            "raw_payload_plaintext_recorded": true,
            "packet_accepted": false,
            "memory_write_request_accepted": false,
            "memory_store_mutated": false,
            "activation_allowed": false,
            "reason": "raw_secret_or_plaintext_payload_denied"
        }),
        serde_json::json!({
            "id": "external-send-public-claim-release-artifact-memory-packet",
            "recorded_memory_write_approval_packet_field_count": 21,
            "external_send_requested": true,
            "public_claim_requested": true,
            "release_artifact_write_requested": true,
            "packet_accepted": false,
            "memory_write_request_accepted": false,
            "memory_store_mutated": false,
            "activation_allowed": false,
            "external_send_allowed": false,
            "public_claim_allowed": false,
            "release_artifact_write_allowed": false,
            "reason": "external_send_public_claim_and_release_artifact_denied"
        }),
        serde_json::json!({
            "id": "direct-memory-store-mutation-at-approval-packet-layer",
            "recorded_memory_write_approval_packet_field_count": 21,
            "memory_store_mutation_requested": true,
            "packet_accepted": false,
            "memory_write_request_accepted": false,
            "memory_store_mutated": false,
            "activation_allowed": false,
            "reason": "approval_packet_layer_cannot_execute_memory_store_mutation"
        }),
    ];

    let source_closure_index_report_sha256 = sha256_json_value(&closure_index);
    let source_minimal_memory_canary_report_sha256 = sha256_json_value(&minimal_canary);
    let source_scoped_memory_canary_durable_receipt_boundary_report_sha256 =
        sha256_json_value(&durable_boundary);
    let source_truth_index_report_sha256 = sha256_json_value(&truth_index);
    let boundary_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-live-mutation-operator-write-approval-packet-boundary-v1:{}:{}:{}:{}:{}",
        route_matrix.route_count,
        source_closure_index_report_sha256,
        source_minimal_memory_canary_report_sha256,
        source_scoped_memory_canary_durable_receipt_boundary_report_sha256,
        source_truth_index_report_sha256,
    ));

    let mut side_effects = serde_json::Map::new();
    for key in [
        "memory_store_mutated",
        "durable_memory_store_write_performed",
        "durable_memory_store_read_performed",
        "durable_memory_store_rollback_performed",
        "memory_write_request_recorded",
        "memory_write_request_persisted",
        "memory_write_approval_packet_recorded",
        "memory_write_approval_packet_persisted",
        "memory_write_approval_packet_accepted",
        "operator_approval_recorded",
        "operator_identity_hash_recorded",
        "operator_approval_signature_hash_recorded",
        "payload_plaintext_persisted",
        "raw_payload_inspected",
        "capability_registry_mutated",
        "plugin_registry_mutated",
        "skill_workshop_written",
        "kg_adapter_read_performed",
        "live_kg_write_performed",
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
        "public_release_claimed",
        "approval_record_persisted",
        "receipt_persisted",
        "launchd_mutated",
        "service_restarted",
        "install_executed",
        "active_binary_mutated",
        "rollback_executed",
        "credential_read",
        "secret_file_read",
    ] {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }

    let report_ready = route_matrix.ready
        && route_count_source_command_accepted
        && closure_index_ready
        && minimal_canary_ready
        && durable_boundary_ready
        && truth_index_ready
        && required_fields.len() == 21
        && inherited_allowed_memory_write_operations.len() == 4
        && required_before_acceptance.len() == 14
        && denied_by.len() == 22
        && denied_fixtures.len() == 8;

    let allowed_next_actions = serde_json::json!([
        {
            "action": "run_memory_write_approval_packet_boundary_require_live_gate",
            "status": "allowed_verification_only",
            "records_operator_approval": false,
            "writes_memory": false,
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
            "action": "prepare_memory_write_execution_preflight_denial_boundary",
            "status": "allowed_report_only_next_slice",
            "requires_accepted_operator_packet_before_execution": true,
            "writes_memory": false,
            "writes_kg": false,
            "invokes_provider": false,
            "sends_externally": false
        }
    ]);

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
        "hepta_memory_live_mutation_operator_write_approval_packet_boundary_route"
    );
    insert_report_json!(
        "endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_APPROVAL_PACKET_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_command",
        "/hepta-memory-live-mutation-operator-write-approval-packet-boundary --json"
    );
    insert_report_json!("native_route", true);
    insert_report_json!("side_effect_free", true);
    insert_report_json!("audit_date", "2026-07-02");
    insert_report_json!(
        "memory_write_approval_packet_boundary_schema_version",
        "memory_write_approval_packet_boundary_v1"
    );
    insert_report_json!("memory_write_approval_packet_boundary_ready", report_ready);
    insert_report_json!(
        "approval_packet_mode",
        "memory_write_operator_approval_packet_shape_no_recording_no_execution"
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
        "source_full_live_activation_closure_index_endpoint",
        HEPTA_FULL_LIVE_ACTIVATION_CLOSURE_INDEX_ENDPOINT
    );
    insert_report_json!(
        "source_full_live_activation_closure_index_ready",
        closure_index_ready
    );
    insert_report_json!(
        "source_full_live_activation_closure_index_report_sha256",
        source_closure_index_report_sha256
    );
    insert_report_json!(
        "source_minimal_memory_canary_endpoint",
        HEPTA_MINIMAL_MEMORY_CANARY_SCOPED_OPERATOR_PACKET_WRITE_READBACK_ROLLBACK_IDEMPOTENCY_RECEIPT_ENDPOINT
    );
    insert_report_json!("source_minimal_memory_canary_ready", minimal_canary_ready);
    insert_report_json!(
        "source_minimal_memory_canary_report_sha256",
        source_minimal_memory_canary_report_sha256
    );
    insert_report_json!(
        "source_scoped_memory_canary_durable_receipt_boundary_endpoint",
        HEPTA_SCOPED_MEMORY_CANARY_DURABLE_RECEIPT_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_scoped_memory_canary_durable_receipt_boundary_ready",
        durable_boundary_ready
    );
    insert_report_json!(
        "source_scoped_memory_canary_durable_receipt_boundary_report_sha256",
        source_scoped_memory_canary_durable_receipt_boundary_report_sha256
    );
    insert_report_json!(
        "source_memory_intelligence_kg_truth_index_report_sha256",
        source_truth_index_report_sha256
    );
    insert_report_json!("minimum_required_samples", 24);
    insert_report_json!(
        "closure_blocker_count",
        json_u64(&closure_index, "closure_blocker_count")
    );
    insert_report_json!(
        "remaining_unrestricted_activation_blocker_count",
        json_u64(
            &closure_index,
            "remaining_unrestricted_activation_blocker_count"
        )
    );
    insert_report_json!("memory_write_approval_packet_shape_ready", true);
    insert_report_json!("memory_write_approval_packet_recorded", false);
    insert_report_json!("memory_write_approval_packet_persisted", false);
    insert_report_json!("memory_write_approval_packet_accepted", false);
    insert_report_json!("memory_write_request_recorded", false);
    insert_report_json!("memory_write_request_accepted", false);
    insert_report_json!("memory_write_request_persisted", false);
    insert_report_json!("operator_approval_recorded", false);
    insert_report_json!("operator_identity_hash_recorded", false);
    insert_report_json!("operator_approval_signature_hash_recorded", false);
    insert_report_json!("operator_approval_timestamp_recorded", false);
    insert_report_json!("single_surface_activation_scope_recorded", false);
    insert_report_json!("memory_namespace_recorded", false);
    insert_report_json!("memory_write_operation_recorded", false);
    insert_report_json!("memory_write_operation_allowed", false);
    insert_report_json!("memory_retention_class_recorded", false);
    insert_report_json!("record_intent_recorded", false);
    insert_report_json!("raw_payload_sha256_recorded", false);
    insert_report_json!("redacted_payload_summary_sha256_recorded", false);
    insert_report_json!("accepted_redaction_proof_recorded", false);
    insert_report_json!("accepted_redaction_proof_count", 0);
    insert_report_json!(
        "source_full_live_activation_closure_index_hash_bound",
        false
    );
    insert_report_json!("source_minimal_memory_canary_hash_bound", false);
    insert_report_json!(
        "source_scoped_memory_canary_durable_receipt_hash_bound",
        false
    );
    insert_report_json!("fresh_pre_activation_soak_evidence_recorded", false);
    insert_report_json!("rollback_plan_recorded", false);
    insert_report_json!("post_write_validation_plan_recorded", false);
    insert_report_json!("no_public_claim_no_external_send_decision_recorded", false);
    insert_report_json!("raw_payload_plaintext_recorded", false);
    insert_report_json!("raw_payload_plaintext_persisted", false);
    insert_report_json!("memory_store_mutation_allowed", false);
    insert_report_json!("memory_store_mutated", false);
    insert_report_json!("durable_memory_store_write_performed", false);
    insert_report_json!("memory_write_execution_ready", false);
    insert_report_json!("live_mutation_execution_ready", false);
    insert_report_json!("provider_prompt_replay_enabled", false);
    insert_report_json!("external_send_enabled", false);
    insert_report_json!("public_claim_or_release_artifact_write_enabled", false);
    insert_report_json!(
        "required_memory_write_approval_packet_field_count",
        required_fields.len()
    );
    insert_report_json!("recorded_memory_write_approval_packet_field_count", 0);
    insert_report_json!("inherited_required_memory_write_request_field_count", 17);
    report.insert(
        "inherited_allowed_memory_write_operations".to_string(),
        serde_json::json!(inherited_allowed_memory_write_operations),
    );
    report.insert(
        "required_memory_write_approval_packet_fields".to_string(),
        serde_json::json!(required_fields),
    );
    report.insert(
        "denied_memory_write_approval_packet_fixtures".to_string(),
        serde_json::Value::Array(denied_fixtures),
    );
    insert_report_json!("denied_memory_write_approval_packet_fixture_count", 8);
    report.insert(
        "denied_by_memory_write_approval_packet_boundary".to_string(),
        serde_json::json!(denied_by),
    );
    insert_report_json!("denied_by_memory_write_approval_packet_boundary_count", 22);
    report.insert(
        "required_before_memory_write_approval_packet_acceptance".to_string(),
        serde_json::json!(required_before_acceptance),
    );
    insert_report_json!(
        "required_before_memory_write_approval_packet_acceptance_count",
        14
    );
    insert_report_json!("provider_invoked", false);
    insert_report_json!("model_invoked", false);
    insert_report_json!("credential_read", false);
    insert_report_json!("secret_file_read", false);
    insert_report_json!("kg_adapter_read_performed", false);
    insert_report_json!("live_kg_write_performed", false);
    insert_report_json!("channel_send_performed", false);
    insert_report_json!("telegram_send_performed", false);
    insert_report_json!("external_send_performed", false);
    insert_report_json!("release_artifact_written", false);
    insert_report_json!("public_artifact_written", false);
    insert_report_json!("public_release_claimed", false);
    insert_report_json!("install_executed", false);
    insert_report_json!("service_restarted", false);
    insert_report_json!("active_binary_mutated", false);
    report.insert("allowed_next_actions".to_string(), allowed_next_actions);
    report.insert(
        "side_effects".to_string(),
        serde_json::Value::Object(side_effects),
    );

    serde_json::Value::Object(report)
}

fn hepta_memory_live_mutation_operator_write_execution_preflight_boundary_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let approval_boundary =
        hepta_memory_live_mutation_operator_write_approval_packet_boundary_report();

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
    let approval_boundary_ready = json_bool(
        &approval_boundary,
        "memory_write_approval_packet_boundary_ready",
    ) && json_bool(
        &approval_boundary,
        "memory_write_approval_packet_shape_ready",
    ) && json_u64(
        &approval_boundary,
        "required_memory_write_approval_packet_field_count",
    ) == 21
        && json_u64(
            &approval_boundary,
            "denied_memory_write_approval_packet_fixture_count",
        ) == 8
        && json_u64(
            &approval_boundary,
            "denied_by_memory_write_approval_packet_boundary_count",
        ) == 22
        && !json_bool(&approval_boundary, "memory_write_approval_packet_recorded")
        && !json_bool(&approval_boundary, "memory_write_approval_packet_persisted")
        && !json_bool(&approval_boundary, "memory_write_approval_packet_accepted")
        && !json_bool(&approval_boundary, "memory_write_request_recorded")
        && !json_bool(&approval_boundary, "memory_write_request_accepted")
        && !json_bool(&approval_boundary, "durable_memory_store_write_performed")
        && !json_bool(&approval_boundary, "memory_store_mutated")
        && !json_bool(&approval_boundary, "live_kg_write_performed")
        && !json_bool(&approval_boundary, "provider_invoked")
        && !json_bool(&approval_boundary, "model_invoked")
        && !json_bool(&approval_boundary, "credential_read")
        && !json_bool(&approval_boundary, "external_send_performed")
        && !json_bool(&approval_boundary, "release_artifact_written")
        && !json_bool(&approval_boundary, "public_release_claimed")
        && !json_bool(&approval_boundary, "active_binary_mutated")
        && side_effects_all_false(&approval_boundary);

    let required_checks = vec![
        "approval_packet_hash_binding",
        "memory_write_request_hash_binding",
        "operator_approval_signature_verification",
        "single_surface_scope_verification",
        "memory_namespace_allowlist_verification",
        "memory_write_operation_allowlist_verification",
        "retention_class_allowlist_verification",
        "redaction_proof_acceptance_verification",
        "raw_payload_sha256_binding",
        "redacted_payload_summary_sha256_binding",
        "source_full_live_activation_closure_index_hash_binding",
        "source_minimal_memory_canary_hash_binding",
        "source_scoped_memory_canary_durable_receipt_hash_binding",
        "fresh_pre_activation_soak_verification",
        "rollback_plan_verification",
        "post_write_validation_plan_verification",
        "no_public_claim_no_external_send_verification",
    ];
    let required_before_execution = vec![
        "accepted_memory_write_approval_packet",
        "approval_packet_hash_binding",
        "memory_write_request_hash_binding",
        "operator_approval_signature_verification",
        "operator_approval_timestamp_freshness",
        "single_surface_activation_scope_verification",
        "memory_namespace_allowlist_verification",
        "memory_write_operation_allowlist_verification",
        "retention_class_allowlist_verification",
        "accepted_redaction_proof_freshness",
        "raw_payload_hash_binding_without_plaintext",
        "redacted_payload_summary_hash_binding",
        "source_report_hash_bindings",
        "fresh_pre_activation_soak_evidence",
        "rollback_plan",
        "post_write_validation_plan",
        "no_public_claim_no_external_send_decision",
    ];
    let denied_by = vec![
        "accepted_approval_packet_required",
        "approval_packet_hash_binding_required",
        "memory_write_request_hash_binding_required",
        "valid_operator_signature_and_fresh_timestamp_required",
        "single_surface_scope_verification_required",
        "memory_namespace_allowlist_required",
        "memory_write_operation_allowlist_required",
        "retention_class_allowlist_required",
        "fresh_accepted_redaction_proof_required",
        "raw_payload_sha256_binding_required",
        "redacted_payload_summary_sha256_binding_required",
        "source_full_live_activation_closure_index_hash_binding_required",
        "source_minimal_memory_canary_hash_binding_required",
        "source_scoped_memory_canary_durable_receipt_hash_binding_required",
        "fresh_pre_activation_soak_verification_required",
        "rollback_plan_verification_required",
        "post_write_validation_plan_verification_required",
        "no_public_claim_no_external_send_verification_required",
        "payload_plaintext_recording_denied",
        "memory_store_mutation_denied",
        "rollback_execution_denied",
        "external_send_public_claim_release_artifact_denied",
    ];
    let denied_fixtures = vec![
        serde_json::json!({
            "id": "no-accepted-approval-packet",
            "recorded_pre_execution_validation_check_count": 0,
            "packet_accepted": false,
            "execution_allowed": false,
            "memory_store_mutated": false,
            "activation_allowed": false,
            "reason": "accepted_approval_packet_required"
        }),
        serde_json::json!({
            "id": "missing-approval-packet-hash-binding",
            "recorded_pre_execution_validation_check_count": 3,
            "source_memory_write_approval_packet_hash_bound": false,
            "execution_allowed": false,
            "memory_store_mutated": false,
            "activation_allowed": false,
            "reason": "approval_packet_hash_binding_required"
        }),
        serde_json::json!({
            "id": "operator-signature-or-timestamp-invalid",
            "recorded_pre_execution_validation_check_count": 5,
            "operator_approval_signature_verified": false,
            "operator_approval_timestamp_fresh": false,
            "execution_allowed": false,
            "memory_store_mutated": false,
            "activation_allowed": false,
            "reason": "valid_operator_signature_and_fresh_timestamp_required"
        }),
        serde_json::json!({
            "id": "namespace-operation-or-retention-not-allowlisted",
            "recorded_pre_execution_validation_check_count": 8,
            "memory_namespace_allowed": false,
            "memory_write_operation_allowed": false,
            "memory_retention_class_allowed": false,
            "execution_allowed": false,
            "memory_store_mutated": false,
            "activation_allowed": false,
            "reason": "memory_namespace_operation_and_retention_allowlists_required"
        }),
        serde_json::json!({
            "id": "redaction-proof-missing-or-stale",
            "recorded_pre_execution_validation_check_count": 9,
            "accepted_redaction_proof_recorded": false,
            "accepted_redaction_proof_fresh": false,
            "execution_allowed": false,
            "memory_store_mutated": false,
            "activation_allowed": false,
            "reason": "fresh_accepted_redaction_proof_required"
        }),
        serde_json::json!({
            "id": "payload-hash-mismatch-or-plaintext-present",
            "recorded_pre_execution_validation_check_count": 11,
            "raw_payload_sha256_bound": false,
            "redacted_payload_summary_sha256_bound": false,
            "raw_payload_plaintext_recorded": true,
            "execution_allowed": false,
            "memory_store_mutated": false,
            "activation_allowed": false,
            "reason": "payload_hash_binding_without_plaintext_required"
        }),
        serde_json::json!({
            "id": "fresh-soak-rollback-or-validation-missing",
            "recorded_pre_execution_validation_check_count": 14,
            "fresh_pre_activation_soak_evidence_recorded": false,
            "rollback_plan_recorded": false,
            "post_write_validation_plan_recorded": false,
            "execution_allowed": false,
            "memory_store_mutated": false,
            "activation_allowed": false,
            "reason": "fresh_soak_rollback_and_validation_required"
        }),
        serde_json::json!({
            "id": "external-send-public-claim-or-release-artifact",
            "recorded_pre_execution_validation_check_count": 17,
            "external_send_requested": true,
            "public_claim_requested": true,
            "release_artifact_write_requested": true,
            "execution_allowed": false,
            "memory_store_mutated": false,
            "activation_allowed": false,
            "reason": "external_send_public_claim_and_release_artifact_denied"
        }),
        serde_json::json!({
            "id": "direct-memory-store-execution-at-preflight-layer",
            "recorded_pre_execution_validation_check_count": 17,
            "memory_store_mutation_requested": true,
            "rollback_execution_requested": true,
            "execution_allowed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "activation_allowed": false,
            "reason": "execution_preflight_layer_cannot_mutate_memory_or_execute_rollback"
        }),
    ];

    let source_memory_write_approval_packet_boundary_report_sha256 =
        sha256_json_value(&approval_boundary);
    let boundary_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-live-mutation-operator-write-execution-preflight-boundary-v1:{}:{}",
        route_matrix.route_count, source_memory_write_approval_packet_boundary_report_sha256,
    ));

    let mut side_effects = serde_json::Map::new();
    for key in [
        "memory_store_mutated",
        "durable_memory_store_write_performed",
        "durable_memory_store_read_performed",
        "durable_memory_store_rollback_performed",
        "memory_write_request_recorded",
        "memory_write_request_persisted",
        "memory_write_approval_packet_recorded",
        "memory_write_approval_packet_persisted",
        "memory_write_approval_packet_accepted",
        "memory_write_execution_preflight_recorded",
        "memory_write_execution_preflight_persisted",
        "memory_write_execution_preflight_accepted",
        "pre_execution_validation_recorded",
        "pre_execution_validation_persisted",
        "pre_execution_validation_accepted",
        "operator_approval_recorded",
        "operator_identity_hash_recorded",
        "operator_approval_signature_hash_recorded",
        "payload_plaintext_persisted",
        "raw_payload_inspected",
        "capability_registry_mutated",
        "plugin_registry_mutated",
        "skill_workshop_written",
        "kg_adapter_read_performed",
        "live_kg_write_performed",
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
        "public_release_claimed",
        "approval_record_persisted",
        "preflight_record_persisted",
        "receipt_persisted",
        "launchd_mutated",
        "service_restarted",
        "install_executed",
        "active_binary_mutated",
        "rollback_executed",
        "credential_read",
        "secret_file_read",
    ] {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }

    let report_ready = route_matrix.ready
        && route_count_source_command_accepted
        && approval_boundary_ready
        && required_checks.len() == 17
        && required_before_execution.len() == 17
        && denied_by.len() == 22
        && denied_fixtures.len() == 9;

    let allowed_next_actions = serde_json::json!([
        {
            "action": "run_memory_write_execution_preflight_boundary_require_live_gate",
            "status": "allowed_verification_only",
            "records_preflight": false,
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
            "action": "prepare_memory_write_execution_denial_matrix_boundary",
            "status": "allowed_report_only_next_slice",
            "requires_accepted_operator_packet_before_execution": true,
            "writes_memory": false,
            "writes_kg": false,
            "invokes_provider": false,
            "sends_externally": false
        }
    ]);

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
        "hepta_memory_live_mutation_operator_write_execution_preflight_boundary_route"
    );
    insert_report_json!(
        "endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_PREFLIGHT_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_command",
        "/hepta-memory-live-mutation-operator-write-execution-preflight-boundary --json"
    );
    insert_report_json!("native_route", true);
    insert_report_json!("side_effect_free", true);
    insert_report_json!("audit_date", "2026-07-02");
    insert_report_json!(
        "memory_write_execution_preflight_boundary_schema_version",
        "memory_write_execution_preflight_boundary_v1"
    );
    insert_report_json!(
        "memory_write_execution_preflight_boundary_ready",
        report_ready
    );
    insert_report_json!(
        "execution_preflight_mode",
        "memory_write_execution_preflight_no_approval_no_mutation"
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
        "source_memory_write_approval_packet_boundary_endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_APPROVAL_PACKET_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_memory_write_approval_packet_boundary_ready",
        approval_boundary_ready
    );
    insert_report_json!(
        "source_memory_write_approval_packet_boundary_report_sha256",
        source_memory_write_approval_packet_boundary_report_sha256
    );
    insert_report_json!("minimum_required_samples", 24);
    insert_report_json!("memory_write_execution_preflight_shape_ready", true);
    insert_report_json!("memory_write_execution_preflight_recorded", false);
    insert_report_json!("memory_write_execution_preflight_persisted", false);
    insert_report_json!("memory_write_execution_preflight_accepted", false);
    insert_report_json!("pre_execution_validation_shape_ready", true);
    insert_report_json!("pre_execution_validation_recorded", false);
    insert_report_json!("pre_execution_validation_persisted", false);
    insert_report_json!("pre_execution_validation_accepted", false);
    insert_report_json!("memory_write_approval_packet_recorded", false);
    insert_report_json!("memory_write_approval_packet_persisted", false);
    insert_report_json!("memory_write_approval_packet_accepted", false);
    insert_report_json!("memory_write_request_recorded", false);
    insert_report_json!("memory_write_request_accepted", false);
    insert_report_json!("memory_write_request_persisted", false);
    insert_report_json!("operator_approval_recorded", false);
    insert_report_json!("operator_identity_hash_recorded", false);
    insert_report_json!("operator_approval_signature_hash_recorded", false);
    insert_report_json!("operator_approval_timestamp_recorded", false);
    insert_report_json!("single_surface_activation_scope_recorded", false);
    insert_report_json!("memory_namespace_recorded", false);
    insert_report_json!("memory_write_operation_recorded", false);
    insert_report_json!("memory_write_operation_allowed", false);
    insert_report_json!("memory_retention_class_recorded", false);
    insert_report_json!("accepted_redaction_proof_recorded", false);
    insert_report_json!("accepted_redaction_proof_count", 0);
    insert_report_json!("source_memory_write_approval_packet_hash_bound", false);
    insert_report_json!("source_memory_write_request_hash_bound", false);
    insert_report_json!(
        "source_full_live_activation_closure_index_hash_bound",
        false
    );
    insert_report_json!("source_minimal_memory_canary_hash_bound", false);
    insert_report_json!(
        "source_scoped_memory_canary_durable_receipt_hash_bound",
        false
    );
    insert_report_json!("raw_payload_sha256_bound", false);
    insert_report_json!("redacted_payload_summary_sha256_bound", false);
    insert_report_json!("fresh_pre_activation_soak_evidence_recorded", false);
    insert_report_json!("rollback_plan_recorded", false);
    insert_report_json!("post_write_validation_plan_recorded", false);
    insert_report_json!("no_public_claim_no_external_send_decision_recorded", false);
    insert_report_json!("memory_write_execution_allowed", false);
    insert_report_json!("memory_write_execution_ready", false);
    insert_report_json!("memory_store_mutation_allowed", false);
    insert_report_json!("memory_store_mutated", false);
    insert_report_json!("durable_memory_store_write_performed", false);
    insert_report_json!("live_mutation_execution_ready", false);
    insert_report_json!("rollback_execution_allowed", false);
    insert_report_json!("rollback_executed", false);
    insert_report_json!("provider_prompt_replay_enabled", false);
    insert_report_json!("external_send_enabled", false);
    insert_report_json!("public_claim_or_release_artifact_write_enabled", false);
    insert_report_json!(
        "required_pre_execution_validation_check_count",
        required_checks.len()
    );
    insert_report_json!("recorded_pre_execution_validation_check_count", 0);
    report.insert(
        "required_pre_execution_validation_checks".to_string(),
        serde_json::json!(required_checks),
    );
    report.insert(
        "denied_memory_write_execution_preflight_fixtures".to_string(),
        serde_json::Value::Array(denied_fixtures),
    );
    insert_report_json!("denied_memory_write_execution_preflight_fixture_count", 9);
    report.insert(
        "denied_by_memory_write_execution_preflight_boundary".to_string(),
        serde_json::json!(denied_by),
    );
    insert_report_json!(
        "denied_by_memory_write_execution_preflight_boundary_count",
        22
    );
    report.insert(
        "required_before_memory_write_execution".to_string(),
        serde_json::json!(required_before_execution),
    );
    insert_report_json!("required_before_memory_write_execution_count", 17);
    insert_report_json!("provider_invoked", false);
    insert_report_json!("model_invoked", false);
    insert_report_json!("credential_read", false);
    insert_report_json!("secret_file_read", false);
    insert_report_json!("kg_adapter_read_performed", false);
    insert_report_json!("live_kg_write_performed", false);
    insert_report_json!("channel_send_performed", false);
    insert_report_json!("telegram_send_performed", false);
    insert_report_json!("external_send_performed", false);
    insert_report_json!("release_artifact_written", false);
    insert_report_json!("public_artifact_written", false);
    insert_report_json!("public_release_claimed", false);
    insert_report_json!("install_executed", false);
    insert_report_json!("service_restarted", false);
    insert_report_json!("active_binary_mutated", false);
    report.insert("allowed_next_actions".to_string(), allowed_next_actions);
    report.insert(
        "side_effects".to_string(),
        serde_json::Value::Object(side_effects),
    );

    serde_json::Value::Object(report)
}

fn hepta_memory_live_mutation_operator_write_execution_denial_matrix_boundary_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let preflight_boundary =
        hepta_memory_live_mutation_operator_write_execution_preflight_boundary_report();

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
    let preflight_boundary_ready =
        json_bool(
            &preflight_boundary,
            "memory_write_execution_preflight_boundary_ready",
        ) && json_bool(
            &preflight_boundary,
            "memory_write_execution_preflight_shape_ready",
        ) && json_bool(&preflight_boundary, "pre_execution_validation_shape_ready")
            && json_u64(
                &preflight_boundary,
                "required_pre_execution_validation_check_count",
            ) == 17
            && json_u64(
                &preflight_boundary,
                "recorded_pre_execution_validation_check_count",
            ) == 0
            && json_u64(
                &preflight_boundary,
                "denied_memory_write_execution_preflight_fixture_count",
            ) == 9
            && json_u64(
                &preflight_boundary,
                "denied_by_memory_write_execution_preflight_boundary_count",
            ) == 22
            && json_u64(
                &preflight_boundary,
                "required_before_memory_write_execution_count",
            ) == 17
            && !json_bool(
                &preflight_boundary,
                "memory_write_execution_preflight_recorded",
            )
            && !json_bool(
                &preflight_boundary,
                "memory_write_execution_preflight_persisted",
            )
            && !json_bool(
                &preflight_boundary,
                "memory_write_execution_preflight_accepted",
            )
            && !json_bool(&preflight_boundary, "pre_execution_validation_recorded")
            && !json_bool(&preflight_boundary, "pre_execution_validation_persisted")
            && !json_bool(&preflight_boundary, "pre_execution_validation_accepted")
            && !json_bool(&preflight_boundary, "memory_write_approval_packet_accepted")
            && !json_bool(&preflight_boundary, "memory_write_request_accepted")
            && !json_bool(&preflight_boundary, "memory_write_execution_allowed")
            && !json_bool(&preflight_boundary, "memory_write_execution_ready")
            && !json_bool(&preflight_boundary, "memory_store_mutated")
            && !json_bool(&preflight_boundary, "rollback_executed")
            && !json_bool(&preflight_boundary, "live_kg_write_performed")
            && !json_bool(&preflight_boundary, "provider_invoked")
            && !json_bool(&preflight_boundary, "model_invoked")
            && !json_bool(&preflight_boundary, "credential_read")
            && !json_bool(&preflight_boundary, "external_send_performed")
            && !json_bool(&preflight_boundary, "release_artifact_written")
            && !json_bool(&preflight_boundary, "public_release_claimed")
            && !json_bool(&preflight_boundary, "active_binary_mutated")
            && side_effects_all_false(&preflight_boundary);

    let required_before_execution = preflight_boundary
        .get("required_before_memory_write_execution")
        .and_then(serde_json::Value::as_array)
        .cloned()
        .unwrap_or_default();
    let denied_by = vec![
        "memory_write_execution_denial_matrix_recording_denied",
        "memory_write_execution_denial_matrix_persistence_denied",
        "memory_write_execution_denial_matrix_materialization_denied",
        "memory_write_execution_denial_matrix_filesystem_write_denied",
        "accepted_approval_packet_required",
        "all_pre_execution_validation_checks_required",
        "approval_packet_hash_binding_required",
        "memory_write_request_hash_binding_required",
        "operator_signature_and_timestamp_required",
        "single_surface_scope_verification_required",
        "namespace_operation_and_retention_allowlists_required",
        "fresh_accepted_redaction_proof_required",
        "payload_hash_binding_without_plaintext_required",
        "source_report_hash_bindings_required",
        "fresh_pre_activation_soak_verification_required",
        "rollback_plan_verification_required",
        "post_write_validation_plan_verification_required",
        "no_public_claim_no_external_send_verification_required",
        "memory_write_execution_denied",
        "memory_store_mutation_denied",
        "rollback_execution_denied",
        "external_send_public_claim_release_artifact_denied",
    ];
    let execution_denial_fixtures = vec![
        serde_json::json!({
            "id": "missing-accepted-approval-packet-execution-attempt",
            "execution_requested": true,
            "accepted_approval_packet_present": false,
            "accepted_pre_execution_validation_check_count": 0,
            "execution_allowed": false,
            "execution_performed": false,
            "memory_store_mutated": false,
            "activation_allowed": false,
            "reason": "accepted_approval_packet_required"
        }),
        serde_json::json!({
            "id": "partial-pre-execution-validation-execution-attempt",
            "execution_requested": true,
            "accepted_approval_packet_present": true,
            "accepted_pre_execution_validation_check_count": 8,
            "required_pre_execution_validation_check_count": 17,
            "execution_allowed": false,
            "execution_performed": false,
            "memory_store_mutated": false,
            "activation_allowed": false,
            "reason": "all_pre_execution_validation_checks_required"
        }),
        serde_json::json!({
            "id": "namespace-operation-retention-not-allowlisted-execution-attempt",
            "execution_requested": true,
            "accepted_pre_execution_validation_check_count": 17,
            "memory_namespace_allowed": false,
            "memory_write_operation_allowed": false,
            "memory_retention_class_allowed": false,
            "execution_allowed": false,
            "execution_performed": false,
            "memory_store_mutated": false,
            "activation_allowed": false,
            "reason": "namespace_operation_and_retention_allowlists_required"
        }),
        serde_json::json!({
            "id": "payload-hash-mismatch-or-plaintext-execution-attempt",
            "execution_requested": true,
            "accepted_pre_execution_validation_check_count": 17,
            "raw_payload_sha256_bound": false,
            "redacted_payload_summary_sha256_bound": false,
            "raw_payload_plaintext_recorded": true,
            "execution_allowed": false,
            "execution_performed": false,
            "memory_store_mutated": false,
            "activation_allowed": false,
            "reason": "payload_hash_binding_without_plaintext_required"
        }),
        serde_json::json!({
            "id": "stale-soak-or-missing-rollback-validation-execution-attempt",
            "execution_requested": true,
            "accepted_pre_execution_validation_check_count": 17,
            "fresh_pre_activation_soak_evidence_recorded": false,
            "rollback_plan_recorded": false,
            "post_write_validation_plan_recorded": false,
            "execution_allowed": false,
            "execution_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "activation_allowed": false,
            "reason": "fresh_soak_rollback_and_validation_required"
        }),
        serde_json::json!({
            "id": "external-send-public-claim-release-artifact-execution-attempt",
            "execution_requested": true,
            "accepted_pre_execution_validation_check_count": 17,
            "external_send_requested": true,
            "public_claim_requested": true,
            "release_artifact_write_requested": true,
            "execution_allowed": false,
            "execution_performed": false,
            "memory_store_mutated": false,
            "external_send_performed": false,
            "public_release_published": false,
            "release_artifact_written": false,
            "activation_allowed": false,
            "reason": "external_send_public_claim_and_release_artifact_denied"
        }),
        serde_json::json!({
            "id": "direct-memory-store-mutation-or-rollback-execution-attempt",
            "execution_requested": true,
            "accepted_pre_execution_validation_check_count": 17,
            "memory_store_mutation_requested": true,
            "rollback_execution_requested": true,
            "execution_allowed": false,
            "execution_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "activation_allowed": false,
            "reason": "execution_denial_matrix_layer_cannot_mutate_memory_or_execute_rollback"
        }),
    ];

    let source_memory_write_execution_preflight_boundary_report_sha256 =
        sha256_json_value(&preflight_boundary);
    let boundary_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-live-mutation-operator-write-execution-denial-matrix-boundary-v1:{}:{}",
        route_matrix.route_count, source_memory_write_execution_preflight_boundary_report_sha256,
    ));

    let mut side_effects = serde_json::Map::new();
    for key in [
        "memory_store_mutated",
        "durable_memory_store_write_performed",
        "durable_memory_store_read_performed",
        "durable_memory_store_rollback_performed",
        "memory_write_request_recorded",
        "memory_write_request_persisted",
        "memory_write_approval_packet_recorded",
        "memory_write_approval_packet_persisted",
        "memory_write_approval_packet_accepted",
        "memory_write_execution_preflight_recorded",
        "memory_write_execution_preflight_persisted",
        "memory_write_execution_preflight_accepted",
        "memory_write_execution_denial_matrix_recorded",
        "memory_write_execution_denial_matrix_persisted",
        "memory_write_execution_denial_matrix_materialized",
        "memory_write_execution_denial_matrix_filesystem_written",
        "pre_execution_validation_recorded",
        "pre_execution_validation_persisted",
        "pre_execution_validation_accepted",
        "operator_approval_recorded",
        "operator_identity_hash_recorded",
        "operator_approval_signature_hash_recorded",
        "payload_plaintext_persisted",
        "raw_payload_inspected",
        "capability_registry_mutated",
        "plugin_registry_mutated",
        "skill_workshop_written",
        "kg_adapter_read_performed",
        "live_kg_write_performed",
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
        "public_release_claimed",
        "approval_record_persisted",
        "preflight_record_persisted",
        "denial_matrix_persisted",
        "receipt_persisted",
        "launchd_mutated",
        "service_restarted",
        "install_executed",
        "active_binary_mutated",
        "rollback_executed",
        "credential_read",
        "secret_file_read",
    ] {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }

    let report_ready = route_matrix.ready
        && route_count_source_command_accepted
        && preflight_boundary_ready
        && required_before_execution.len() == 17
        && denied_by.len() == 22
        && execution_denial_fixtures.len() == 7;

    let allowed_next_actions = serde_json::json!([
        {
            "action": "run_memory_write_execution_denial_matrix_boundary_require_live_gate",
            "status": "allowed_verification_only",
            "records_denial_matrix": false,
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
            "action": "prepare_memory_write_execution_no_write_sink_contract_boundary",
            "status": "allowed_report_only_next_slice",
            "requires_accepted_operator_packet_before_execution": true,
            "writes_memory": false,
            "writes_kg": false,
            "invokes_provider": false,
            "sends_externally": false
        }
    ]);

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
        "hepta_memory_live_mutation_operator_write_execution_denial_matrix_boundary_route"
    );
    insert_report_json!(
        "endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_DENIAL_MATRIX_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_command",
        "/hepta-memory-live-mutation-operator-write-execution-denial-matrix-boundary --json"
    );
    insert_report_json!("native_route", true);
    insert_report_json!("side_effect_free", true);
    insert_report_json!("audit_date", "2026-07-02");
    insert_report_json!(
        "memory_write_execution_denial_matrix_boundary_schema_version",
        "memory_write_execution_denial_matrix_boundary_v1"
    );
    insert_report_json!(
        "memory_write_execution_denial_matrix_boundary_ready",
        report_ready
    );
    insert_report_json!(
        "execution_denial_matrix_mode",
        "memory_write_execution_attempt_denial_matrix_no_store_mutation"
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
        "source_memory_write_execution_preflight_boundary_endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_PREFLIGHT_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_memory_write_execution_preflight_boundary_ready",
        preflight_boundary_ready
    );
    insert_report_json!(
        "source_memory_write_execution_preflight_boundary_report_sha256",
        source_memory_write_execution_preflight_boundary_report_sha256
    );
    insert_report_json!("minimum_required_samples", 24);
    insert_report_json!("memory_write_execution_denial_matrix_ready", true);
    insert_report_json!("memory_write_execution_denial_matrix_recorded", false);
    insert_report_json!("memory_write_execution_denial_matrix_persisted", false);
    insert_report_json!("memory_write_execution_denial_matrix_materialized", false);
    insert_report_json!(
        "memory_write_execution_denial_matrix_filesystem_written",
        false
    );
    insert_report_json!("pre_execution_validation_shape_ready", true);
    insert_report_json!(
        "required_pre_execution_validation_check_count",
        json_u64(
            &preflight_boundary,
            "required_pre_execution_validation_check_count"
        )
    );
    insert_report_json!("recorded_pre_execution_validation_check_count", 0);
    insert_report_json!("accepted_pre_execution_validation_check_count", 0);
    insert_report_json!("future_pre_execution_validation_check_slot_count", 17);
    insert_report_json!("memory_write_execution_attempt_requested_count", 7);
    insert_report_json!("memory_write_execution_attempt_performed_count", 0);
    insert_report_json!("memory_write_execution_allowed_count", 0);
    insert_report_json!("memory_write_execution_denied_count", 7);
    insert_report_json!("blocked_execution_fixture_count", 7);
    insert_report_json!("allowed_execution_fixture_count", 0);
    insert_report_json!("required_execution_denial_fixture_count", 7);
    insert_report_json!("execution_denial_fixture_count", 7);
    insert_report_json!("pre_execution_validation_recorded", false);
    insert_report_json!("pre_execution_validation_persisted", false);
    insert_report_json!("pre_execution_validation_accepted", false);
    insert_report_json!("memory_write_approval_packet_recorded", false);
    insert_report_json!("memory_write_approval_packet_persisted", false);
    insert_report_json!("memory_write_approval_packet_accepted", false);
    insert_report_json!("memory_write_request_recorded", false);
    insert_report_json!("memory_write_request_accepted", false);
    insert_report_json!("memory_write_request_persisted", false);
    insert_report_json!("operator_approval_recorded", false);
    insert_report_json!("operator_identity_hash_recorded", false);
    insert_report_json!("operator_approval_signature_hash_recorded", false);
    insert_report_json!("operator_approval_timestamp_recorded", false);
    insert_report_json!("single_surface_activation_scope_recorded", false);
    insert_report_json!("memory_namespace_recorded", false);
    insert_report_json!("memory_write_operation_recorded", false);
    insert_report_json!("memory_write_operation_allowed", false);
    insert_report_json!("memory_retention_class_recorded", false);
    insert_report_json!("accepted_redaction_proof_recorded", false);
    insert_report_json!("accepted_redaction_proof_count", 0);
    insert_report_json!("source_memory_write_approval_packet_hash_bound", false);
    insert_report_json!("source_memory_write_request_hash_bound", false);
    insert_report_json!("source_memory_write_execution_preflight_hash_bound", false);
    insert_report_json!("source_memory_write_contract_hash_bound", false);
    insert_report_json!("source_memory_intelligence_hash_bound", false);
    insert_report_json!(
        "source_payload_redaction_acceptance_matrix_hash_bound",
        false
    );
    insert_report_json!("source_payload_redaction_proof_hash_bound", false);
    insert_report_json!("raw_payload_sha256_bound", false);
    insert_report_json!("redacted_payload_summary_sha256_bound", false);
    insert_report_json!("raw_payload_plaintext_recorded", false);
    insert_report_json!("raw_payload_plaintext_persisted", false);
    insert_report_json!("fresh_pre_activation_soak_evidence_recorded", false);
    insert_report_json!("rollback_plan_recorded", false);
    insert_report_json!("post_write_validation_plan_recorded", false);
    insert_report_json!("no_public_claim_no_external_send_decision_recorded", false);
    insert_report_json!("memory_write_execution_allowed", false);
    insert_report_json!("memory_write_execution_ready", false);
    insert_report_json!("memory_write_execution_performed", false);
    insert_report_json!("memory_store_mutation_allowed", false);
    insert_report_json!("memory_store_mutated", false);
    insert_report_json!("durable_memory_store_write_performed", false);
    insert_report_json!("live_mutation_execution_ready", false);
    insert_report_json!("rollback_execution_allowed", false);
    insert_report_json!("rollback_executed", false);
    insert_report_json!("provider_prompt_replay_enabled", false);
    insert_report_json!("external_send_enabled", false);
    insert_report_json!("external_send_performed", false);
    insert_report_json!("public_claim_or_release_artifact_write_enabled", false);
    insert_report_json!("public_release_published", false);
    insert_report_json!("release_artifact_written", false);
    report.insert(
        "execution_denial_fixtures".to_string(),
        serde_json::Value::Array(execution_denial_fixtures),
    );
    report.insert(
        "denied_by_memory_write_execution_denial_matrix".to_string(),
        serde_json::json!(denied_by),
    );
    insert_report_json!("denied_by_memory_write_execution_denial_matrix_count", 22);
    report.insert(
        "required_before_memory_write_execution".to_string(),
        serde_json::Value::Array(required_before_execution),
    );
    insert_report_json!("required_before_memory_write_execution_count", 17);
    insert_report_json!("provider_invoked", false);
    insert_report_json!("model_invoked", false);
    insert_report_json!("credential_read", false);
    insert_report_json!("secret_file_read", false);
    insert_report_json!("kg_adapter_read_performed", false);
    insert_report_json!("live_kg_write_performed", false);
    insert_report_json!("channel_send_performed", false);
    insert_report_json!("telegram_send_performed", false);
    insert_report_json!("external_send_performed", false);
    insert_report_json!("public_artifact_written", false);
    insert_report_json!("public_release_claimed", false);
    insert_report_json!("install_executed", false);
    insert_report_json!("service_restarted", false);
    insert_report_json!("active_binary_mutated", false);
    report.insert("allowed_next_actions".to_string(), allowed_next_actions);
    report.insert(
        "side_effects".to_string(),
        serde_json::Value::Object(side_effects),
    );

    serde_json::Value::Object(report)
}

fn hepta_memory_live_mutation_operator_write_execution_no_write_sink_contract_boundary_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let denial_matrix =
        hepta_memory_live_mutation_operator_write_execution_denial_matrix_boundary_report();

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
    let denial_matrix_ready =
        json_bool(
            &denial_matrix,
            "memory_write_execution_denial_matrix_boundary_ready",
        ) && json_bool(&denial_matrix, "memory_write_execution_denial_matrix_ready")
            && json_u64(
                &denial_matrix,
                "memory_write_execution_attempt_requested_count",
            ) == 7
            && json_u64(
                &denial_matrix,
                "memory_write_execution_attempt_performed_count",
            ) == 0
            && json_u64(&denial_matrix, "memory_write_execution_denied_count") == 7
            && json_u64(&denial_matrix, "execution_denial_fixture_count") == 7
            && json_u64(
                &denial_matrix,
                "denied_by_memory_write_execution_denial_matrix_count",
            ) == 22
            && json_u64(
                &denial_matrix,
                "required_before_memory_write_execution_count",
            ) == 17
            && !json_bool(
                &denial_matrix,
                "memory_write_execution_denial_matrix_recorded",
            )
            && !json_bool(
                &denial_matrix,
                "memory_write_execution_denial_matrix_persisted",
            )
            && !json_bool(
                &denial_matrix,
                "memory_write_execution_denial_matrix_materialized",
            )
            && !json_bool(
                &denial_matrix,
                "memory_write_execution_denial_matrix_filesystem_written",
            )
            && !json_bool(&denial_matrix, "memory_write_execution_performed")
            && !json_bool(&denial_matrix, "memory_store_mutated")
            && !json_bool(&denial_matrix, "rollback_executed")
            && !json_bool(&denial_matrix, "live_kg_write_performed")
            && !json_bool(&denial_matrix, "provider_invoked")
            && !json_bool(&denial_matrix, "model_invoked")
            && !json_bool(&denial_matrix, "credential_read")
            && !json_bool(&denial_matrix, "external_send_performed")
            && !json_bool(&denial_matrix, "release_artifact_written")
            && !json_bool(&denial_matrix, "public_release_claimed")
            && !json_bool(&denial_matrix, "active_binary_mutated")
            && side_effects_all_false(&denial_matrix);

    let no_write_sink_surfaces = vec![
        "redacted_execution_request_envelope_validation",
        "source_report_hash_binding_validation",
        "operator_approval_preflight_validation_requirement",
        "memory_namespace_operation_retention_allowlist_requirement",
        "payload_hash_binding_without_plaintext_requirement",
        "fresh_soak_rollback_validation_requirement",
        "external_send_public_claim_artifact_rejection",
        "store_write_path_disabled_by_default",
    ];
    let required_before_any_memory_write_execution = vec![
        "accepted_operator_approval_packet",
        "accepted_pre_execution_validation_record",
        "operator_identity_hash",
        "operator_approval_signature_hash",
        "operator_approval_timestamp",
        "single_surface_activation_scope",
        "namespace_operation_retention_allowlist_match",
        "accepted_redaction_proof_id",
        "source_report_hash_bindings",
        "raw_payload_sha256_without_plaintext",
        "redacted_payload_summary_sha256",
        "fresh_pre_activation_soak_evidence",
        "rollback_plan_id",
        "post_write_validation_plan_id",
        "no_public_claim_no_external_send_decision",
        "explicit_write_path_enablement",
        "post_write_watchdog_soak_plan",
    ];
    let denied_by = vec![
        "execution_remains_disabled",
        "store_write_path_disabled_by_default",
        "memory_store_mutation_denied",
        "rollback_execution_denied",
        "external_send_denied",
        "public_claim_denied",
        "release_artifact_write_denied",
        "plaintext_payload_recording_denied",
        "secret_read_denied",
        "service_restart_denied",
    ];
    let no_write_sink_fixtures = vec![
        serde_json::json!({
            "id": "redacted-execution-envelope-validation-shape",
            "sink_status": "accepted_for_no_write_validation",
            "redacted_execution_request_envelope_present": true,
            "source_report_hash_bindings_present": true,
            "execution_requested": true,
            "write_requested": false,
            "execution_allowed": false,
            "execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "activation_allowed": false,
            "reason": "redacted_shape_can_be_validated_but_execution_remains_disabled"
        }),
        serde_json::json!({
            "id": "source-report-hash-bound-validation-shape",
            "sink_status": "accepted_for_no_write_validation",
            "source_memory_write_execution_denial_matrix_report_sha256_bound": true,
            "source_memory_write_execution_preflight_report_sha256_bound": true,
            "source_payload_redaction_proof_report_sha256_bound": true,
            "execution_requested": true,
            "write_requested": false,
            "execution_allowed": false,
            "execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "activation_allowed": false,
            "reason": "source_hash_shape_can_be_validated_but_not_executed"
        }),
        serde_json::json!({
            "id": "approval-preflight-allowlist-validation-shape",
            "sink_status": "accepted_for_no_write_validation",
            "operator_approval_required": true,
            "all_pre_execution_validation_checks_required": true,
            "namespace_operation_retention_allowlist_required": true,
            "payload_hash_binding_without_plaintext_required": true,
            "fresh_soak_rollback_validation_required": true,
            "execution_requested": true,
            "write_requested": false,
            "execution_allowed": false,
            "execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "activation_allowed": false,
            "reason": "future_requirements_can_be_described_but_no_write_sink_keeps_execution_off"
        }),
        serde_json::json!({
            "id": "store-write-path-disabled-mutation-attempt",
            "sink_status": "rejected",
            "execution_requested": true,
            "write_requested": true,
            "memory_store_mutation_requested": true,
            "no_write_sink_write_path_enabled_by_default": false,
            "execution_allowed": false,
            "execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "activation_allowed": false,
            "reason": "store_write_path_disabled_by_default"
        }),
        serde_json::json!({
            "id": "external-send-public-artifact-attempt",
            "sink_status": "rejected",
            "execution_requested": true,
            "write_requested": true,
            "external_send_requested": true,
            "public_claim_requested": true,
            "release_artifact_write_requested": true,
            "execution_allowed": false,
            "execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "external_send_performed": false,
            "public_release_published": false,
            "release_artifact_written": false,
            "activation_allowed": false,
            "reason": "external_send_public_claim_and_release_artifact_denied"
        }),
        serde_json::json!({
            "id": "rollback-or-direct-store-execution-attempt",
            "sink_status": "rejected",
            "execution_requested": true,
            "write_requested": true,
            "rollback_execution_requested": true,
            "memory_store_mutation_requested": true,
            "execution_allowed": false,
            "execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "activation_allowed": false,
            "reason": "no_write_sink_cannot_execute_rollback_or_direct_store_mutation"
        }),
    ];

    let accepted_validation_fixture_count = no_write_sink_fixtures
        .iter()
        .filter(|fixture| {
            fixture
                .get("sink_status")
                .and_then(serde_json::Value::as_str)
                == Some("accepted_for_no_write_validation")
        })
        .count();
    let rejected_execution_fixture_count = no_write_sink_fixtures
        .iter()
        .filter(|fixture| {
            fixture
                .get("sink_status")
                .and_then(serde_json::Value::as_str)
                == Some("rejected")
        })
        .count();
    let write_request_fixture_count = no_write_sink_fixtures
        .iter()
        .filter(|fixture| {
            fixture
                .get("write_requested")
                .and_then(serde_json::Value::as_bool)
                == Some(true)
        })
        .count();

    let source_memory_write_execution_denial_matrix_boundary_report_sha256 =
        sha256_json_value(&denial_matrix);
    let boundary_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-live-mutation-operator-write-execution-no-write-sink-contract-boundary-v1:{}:{}",
        route_matrix.route_count,
        source_memory_write_execution_denial_matrix_boundary_report_sha256,
    ));

    let mut side_effects = serde_json::Map::new();
    for key in [
        "memory_store_mutated",
        "memory_store_write_performed",
        "durable_memory_store_write_performed",
        "memory_write_request_recorded",
        "memory_write_request_persisted",
        "memory_write_approval_packet_recorded",
        "memory_write_approval_packet_persisted",
        "memory_write_execution_preflight_recorded",
        "memory_write_execution_preflight_persisted",
        "memory_write_execution_denial_matrix_recorded",
        "memory_write_execution_denial_matrix_persisted",
        "memory_write_execution_no_write_sink_contract_recorded",
        "memory_write_execution_no_write_sink_contract_persisted",
        "memory_write_execution_no_write_sink_contract_materialized",
        "memory_write_execution_no_write_sink_contract_filesystem_written",
        "pre_execution_validation_recorded",
        "pre_execution_validation_persisted",
        "payload_plaintext_persisted",
        "raw_payload_inspected",
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
        "public_release_claimed",
        "approval_record_persisted",
        "preflight_record_persisted",
        "denial_matrix_persisted",
        "no_write_sink_contract_persisted",
        "receipt_persisted",
        "launchd_mutated",
        "service_restarted",
        "install_executed",
        "active_binary_mutated",
        "rollback_executed",
        "credential_read",
        "secret_file_read",
    ] {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }

    let report_ready = route_matrix.ready
        && route_count_source_command_accepted
        && denial_matrix_ready
        && no_write_sink_surfaces.len() == 8
        && no_write_sink_fixtures.len() == 6
        && accepted_validation_fixture_count == 3
        && rejected_execution_fixture_count == 3
        && write_request_fixture_count == 3
        && denied_by.len() == 10
        && required_before_any_memory_write_execution.len() == 17;

    let allowed_next_actions = serde_json::json!([
        {
            "action": "run_memory_write_execution_no_write_sink_contract_boundary_require_live_gate",
            "status": "allowed_verification_only",
            "records_no_write_sink_contract": false,
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
            "action": "prepare_memory_write_execution_write_enable_fixture_boundary",
            "status": "allowed_report_only_next_slice",
            "requires_accepted_operator_packet_before_execution": true,
            "writes_memory": false,
            "writes_kg": false,
            "invokes_provider": false,
            "sends_externally": false
        }
    ]);

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
        "hepta_memory_live_mutation_operator_write_execution_no_write_sink_contract_boundary_route"
    );
    insert_report_json!(
        "endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_NO_WRITE_SINK_CONTRACT_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_command",
        "/hepta-memory-live-mutation-operator-write-execution-no-write-sink-contract-boundary --json"
    );
    insert_report_json!("native_route", true);
    insert_report_json!("side_effect_free", true);
    insert_report_json!("audit_date", "2026-07-02");
    insert_report_json!(
        "memory_write_execution_no_write_sink_contract_boundary_schema_version",
        "memory_write_execution_no_write_sink_contract_boundary_v1"
    );
    insert_report_json!(
        "memory_write_execution_no_write_sink_contract_boundary_ready",
        report_ready
    );
    insert_report_json!(
        "no_write_sink_contract_mode",
        "memory_write_execution_no_write_sink_contract_no_store_mutation"
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
        "source_memory_write_execution_denial_matrix_boundary_endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_DENIAL_MATRIX_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_memory_write_execution_denial_matrix_boundary_ready",
        denial_matrix_ready
    );
    insert_report_json!(
        "source_memory_write_execution_denial_matrix_boundary_report_sha256",
        source_memory_write_execution_denial_matrix_boundary_report_sha256
    );
    insert_report_json!("minimum_required_samples", 24);
    insert_report_json!("memory_write_execution_no_write_sink_contract_ready", true);
    insert_report_json!("memory_write_execution_denial_matrix_ready", true);
    insert_report_json!("pre_execution_validation_shape_ready", true);
    insert_report_json!(
        "required_pre_execution_validation_check_count",
        json_u64(
            &denial_matrix,
            "required_pre_execution_validation_check_count"
        )
    );
    insert_report_json!("accepted_pre_execution_validation_check_count", 0);
    insert_report_json!("required_no_write_sink_surface_count", 8);
    insert_report_json!("ready_no_write_sink_surface_count", 8);
    insert_report_json!("side_effect_free_no_write_sink_surface_count", 8);
    insert_report_json!("no_write_sink_fixture_count", 6);
    insert_report_json!("no_write_sink_accepted_validation_fixture_count", 3);
    insert_report_json!("no_write_sink_rejected_execution_fixture_count", 3);
    insert_report_json!("no_write_sink_execution_request_fixture_count", 6);
    insert_report_json!("no_write_sink_write_request_fixture_count", 3);
    insert_report_json!("no_write_sink_allowed_write_fixture_count", 0);
    insert_report_json!("no_write_sink_rejected_write_fixture_count", 3);
    insert_report_json!("no_write_sink_accepts_redacted_execution_envelope", true);
    insert_report_json!("no_write_sink_accepts_source_report_hash_bindings", true);
    insert_report_json!(
        "no_write_sink_requires_operator_approval_and_preflight_validation",
        true
    );
    insert_report_json!(
        "no_write_sink_requires_namespace_operation_retention_allowlist",
        true
    );
    insert_report_json!(
        "no_write_sink_requires_payload_hash_binding_without_plaintext",
        true
    );
    insert_report_json!(
        "no_write_sink_requires_fresh_soak_rollback_validation",
        true
    );
    insert_report_json!(
        "no_write_sink_rejects_external_send_public_claim_artifact",
        true
    );
    insert_report_json!("no_write_sink_rejects_store_write_execution", true);
    insert_report_json!("no_write_sink_write_path_enabled_by_default", false);
    insert_report_json!("no_write_sink_persistence_enabled_by_default", false);
    insert_report_json!("memory_write_execution_denial_matrix_recorded", false);
    insert_report_json!("memory_write_execution_denial_matrix_persisted", false);
    insert_report_json!(
        "memory_write_execution_no_write_sink_contract_recorded",
        false
    );
    insert_report_json!(
        "memory_write_execution_no_write_sink_contract_persisted",
        false
    );
    insert_report_json!(
        "memory_write_execution_no_write_sink_contract_materialized",
        false
    );
    insert_report_json!(
        "memory_write_execution_no_write_sink_contract_filesystem_written",
        false
    );
    insert_report_json!("pre_execution_validation_recorded", false);
    insert_report_json!("pre_execution_validation_persisted", false);
    insert_report_json!("pre_execution_validation_accepted", false);
    insert_report_json!("memory_write_approval_packet_recorded", false);
    insert_report_json!("memory_write_approval_packet_persisted", false);
    insert_report_json!("memory_write_approval_packet_accepted", false);
    insert_report_json!("memory_write_request_recorded", false);
    insert_report_json!("memory_write_request_accepted", false);
    insert_report_json!("memory_write_request_persisted", false);
    insert_report_json!("operator_approval_recorded", false);
    insert_report_json!("operator_identity_hash_recorded", false);
    insert_report_json!("operator_approval_signature_hash_recorded", false);
    insert_report_json!("operator_approval_timestamp_recorded", false);
    insert_report_json!("memory_namespace_recorded", false);
    insert_report_json!("memory_write_operation_recorded", false);
    insert_report_json!("memory_retention_class_recorded", false);
    insert_report_json!("accepted_redaction_proof_recorded", false);
    insert_report_json!("accepted_redaction_proof_count", 0);
    insert_report_json!(
        "source_memory_write_execution_denial_matrix_hash_bound",
        false
    );
    insert_report_json!("source_memory_write_approval_packet_hash_bound", false);
    insert_report_json!("source_memory_write_contract_hash_bound", false);
    insert_report_json!("source_memory_intelligence_hash_bound", false);
    insert_report_json!(
        "source_payload_redaction_acceptance_matrix_hash_bound",
        false
    );
    insert_report_json!("source_payload_redaction_proof_hash_bound", false);
    insert_report_json!("raw_payload_sha256_bound", false);
    insert_report_json!("redacted_payload_summary_sha256_bound", false);
    insert_report_json!("raw_payload_plaintext_recorded", false);
    insert_report_json!("raw_payload_plaintext_persisted", false);
    insert_report_json!("fresh_pre_activation_soak_evidence_recorded", false);
    insert_report_json!("rollback_plan_recorded", false);
    insert_report_json!("post_write_validation_plan_recorded", false);
    insert_report_json!("memory_write_execution_allowed", false);
    insert_report_json!("memory_write_execution_ready", false);
    insert_report_json!("memory_write_execution_performed", false);
    insert_report_json!("memory_write_execution_performed_count", 0);
    insert_report_json!("memory_write_execution_allowed_count", 0);
    insert_report_json!("memory_write_execution_denied_count", 6);
    insert_report_json!("memory_store_write_path_enabled", false);
    insert_report_json!("memory_store_write_performed_count", 0);
    insert_report_json!("memory_store_mutation_allowed", false);
    insert_report_json!("memory_store_mutated", false);
    insert_report_json!("durable_memory_store_write_performed", false);
    insert_report_json!("live_mutation_execution_ready", false);
    insert_report_json!("rollback_execution_allowed", false);
    insert_report_json!("rollback_executed", false);
    insert_report_json!("provider_prompt_replay_enabled", false);
    insert_report_json!("external_send_enabled", false);
    insert_report_json!("external_send_performed", false);
    insert_report_json!("public_claim_or_release_artifact_write_enabled", false);
    insert_report_json!("public_release_published", false);
    insert_report_json!("release_artifact_written", false);
    report.insert(
        "no_write_sink_surfaces".to_string(),
        serde_json::json!(no_write_sink_surfaces),
    );
    report.insert(
        "no_write_sink_fixtures".to_string(),
        serde_json::Value::Array(no_write_sink_fixtures),
    );
    report.insert(
        "denied_by_no_write_sink_contract".to_string(),
        serde_json::json!(denied_by),
    );
    insert_report_json!("denied_by_no_write_sink_contract_count", 10);
    report.insert(
        "required_before_any_memory_write_execution".to_string(),
        serde_json::json!(required_before_any_memory_write_execution),
    );
    insert_report_json!("required_before_any_memory_write_execution_count", 17);
    insert_report_json!("provider_invoked", false);
    insert_report_json!("model_invoked", false);
    insert_report_json!("credential_read", false);
    insert_report_json!("secret_file_read", false);
    insert_report_json!("kg_adapter_read_performed", false);
    insert_report_json!("live_kg_write_performed", false);
    insert_report_json!("channel_send_performed", false);
    insert_report_json!("telegram_send_performed", false);
    insert_report_json!("external_send_performed", false);
    insert_report_json!("public_artifact_written", false);
    insert_report_json!("public_release_claimed", false);
    insert_report_json!("install_executed", false);
    insert_report_json!("service_restarted", false);
    insert_report_json!("active_binary_mutated", false);
    report.insert("allowed_next_actions".to_string(), allowed_next_actions);
    report.insert(
        "side_effects".to_string(),
        serde_json::Value::Object(side_effects),
    );

    serde_json::Value::Object(report)
}

fn hepta_memory_live_mutation_operator_write_execution_write_enable_fixture_boundary_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let no_write_sink =
        hepta_memory_live_mutation_operator_write_execution_no_write_sink_contract_boundary_report(
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
    let no_write_sink_ready = json_bool(
        &no_write_sink,
        "memory_write_execution_no_write_sink_contract_boundary_ready",
    ) && json_bool(
        &no_write_sink,
        "memory_write_execution_no_write_sink_contract_ready",
    ) && json_u64(&no_write_sink, "required_no_write_sink_surface_count")
        == 8
        && json_u64(&no_write_sink, "ready_no_write_sink_surface_count") == 8
        && json_u64(&no_write_sink, "no_write_sink_fixture_count") == 6
        && json_u64(
            &no_write_sink,
            "no_write_sink_accepted_validation_fixture_count",
        ) == 3
        && json_u64(
            &no_write_sink,
            "no_write_sink_rejected_execution_fixture_count",
        ) == 3
        && json_u64(&no_write_sink, "denied_by_no_write_sink_contract_count") == 10
        && json_u64(
            &no_write_sink,
            "required_before_any_memory_write_execution_count",
        ) == 17
        && !json_bool(
            &no_write_sink,
            "memory_write_execution_no_write_sink_contract_recorded",
        )
        && !json_bool(
            &no_write_sink,
            "memory_write_execution_no_write_sink_contract_persisted",
        )
        && !json_bool(
            &no_write_sink,
            "memory_write_execution_no_write_sink_contract_materialized",
        )
        && !json_bool(
            &no_write_sink,
            "memory_write_execution_no_write_sink_contract_filesystem_written",
        )
        && !json_bool(&no_write_sink, "memory_write_execution_performed")
        && !json_bool(&no_write_sink, "memory_store_mutated")
        && !json_bool(&no_write_sink, "rollback_executed")
        && !json_bool(&no_write_sink, "live_kg_write_performed")
        && !json_bool(&no_write_sink, "provider_invoked")
        && !json_bool(&no_write_sink, "model_invoked")
        && !json_bool(&no_write_sink, "credential_read")
        && !json_bool(&no_write_sink, "external_send_performed")
        && !json_bool(&no_write_sink, "release_artifact_written")
        && !json_bool(&no_write_sink, "public_release_claimed")
        && !json_bool(&no_write_sink, "active_binary_mutated")
        && side_effects_all_false(&no_write_sink);

    let write_enable_surfaces = vec![
        "accepted_operator_approval_packet_required",
        "accepted_pre_execution_validation_record_required",
        "operator_identity_signature_timestamp_required",
        "single_surface_activation_scope_required",
        "namespace_operation_retention_allowlist_required",
        "accepted_redaction_proof_and_payload_hash_bindings_required",
        "source_report_hash_bindings_required",
        "fresh_soak_rollback_validation_required",
        "explicit_write_path_enablement_required",
        "post_write_watchdog_soak_plan_required",
    ];
    let denied_by = vec![
        "accepted_operator_approval_packet_required",
        "accepted_pre_execution_validation_record_required",
        "operator_identity_signature_scope_required",
        "namespace_operation_retention_allowlists_required",
        "accepted_redaction_proof_required",
        "payload_hash_binding_without_plaintext_required",
        "source_report_hash_bindings_required",
        "fresh_soak_rollback_validation_required",
        "external_send_public_claim_release_artifact_denied",
        "direct_store_mutation_denied",
        "rollback_execution_denied",
        "post_write_watchdog_soak_plan_required",
        "live_mutation_execution_denied",
    ];
    let write_enable_fixtures = serde_json::json!([
        {
            "id": "write-enable-missing-approval-preflight",
            "explicit_write_enable_requested": true,
            "write_enable_status": "blocked",
            "accepted_operator_approval_packet_present": false,
            "accepted_pre_execution_validation_record_present": false,
            "memory_store_write_requested": true,
            "execution_allowed": false,
            "execution_performed": false,
            "memory_store_write_allowed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "activation_allowed": false,
            "reason": "accepted_operator_approval_packet_and_pre_execution_validation_required"
        },
        {
            "id": "write-enable-missing-operator-scope",
            "explicit_write_enable_requested": true,
            "write_enable_status": "blocked",
            "accepted_operator_approval_packet_present": true,
            "accepted_pre_execution_validation_record_present": true,
            "operator_identity_hash_recorded": false,
            "operator_approval_signature_hash_recorded": false,
            "single_surface_activation_scope_recorded": false,
            "memory_store_write_requested": true,
            "execution_allowed": false,
            "execution_performed": false,
            "memory_store_write_allowed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "activation_allowed": false,
            "reason": "operator_identity_signature_and_single_surface_scope_required"
        },
        {
            "id": "write-enable-allowlist-mismatch",
            "explicit_write_enable_requested": true,
            "write_enable_status": "blocked",
            "accepted_operator_approval_packet_present": true,
            "accepted_pre_execution_validation_record_present": true,
            "operator_identity_hash_recorded": true,
            "single_surface_activation_scope_recorded": true,
            "memory_namespace_allowed": false,
            "memory_write_operation_allowed": false,
            "memory_retention_class_allowed": false,
            "memory_store_write_requested": true,
            "execution_allowed": false,
            "execution_performed": false,
            "memory_store_write_allowed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "activation_allowed": false,
            "reason": "namespace_operation_and_retention_allowlists_required"
        },
        {
            "id": "write-enable-payload-binding-missing-or-plaintext",
            "explicit_write_enable_requested": true,
            "write_enable_status": "blocked",
            "accepted_operator_approval_packet_present": true,
            "accepted_pre_execution_validation_record_present": true,
            "operator_identity_hash_recorded": true,
            "single_surface_activation_scope_recorded": true,
            "memory_namespace_allowed": true,
            "memory_write_operation_allowed": true,
            "memory_retention_class_allowed": true,
            "accepted_redaction_proof_count": 0,
            "raw_payload_sha256_bound": false,
            "redacted_payload_summary_sha256_bound": false,
            "raw_payload_plaintext_recorded": true,
            "memory_store_write_requested": true,
            "execution_allowed": false,
            "execution_performed": false,
            "memory_store_write_allowed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "activation_allowed": false,
            "reason": "accepted_redaction_proof_and_payload_hash_binding_without_plaintext_required"
        },
        {
            "id": "write-enable-stale-soak-or-missing-rollback-validation",
            "explicit_write_enable_requested": true,
            "write_enable_status": "blocked",
            "accepted_operator_approval_packet_present": true,
            "accepted_pre_execution_validation_record_present": true,
            "operator_identity_hash_recorded": true,
            "single_surface_activation_scope_recorded": true,
            "memory_namespace_allowed": true,
            "memory_write_operation_allowed": true,
            "memory_retention_class_allowed": true,
            "accepted_redaction_proof_count": 1,
            "raw_payload_sha256_bound": true,
            "redacted_payload_summary_sha256_bound": true,
            "fresh_pre_activation_soak_evidence_recorded": false,
            "rollback_plan_recorded": false,
            "post_write_validation_plan_recorded": false,
            "memory_store_write_requested": true,
            "execution_allowed": false,
            "execution_performed": false,
            "memory_store_write_allowed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "activation_allowed": false,
            "reason": "fresh_soak_rollback_and_post_write_validation_required"
        },
        {
            "id": "write-enable-public-artifact-or-external-send",
            "explicit_write_enable_requested": true,
            "write_enable_status": "blocked",
            "accepted_operator_approval_packet_present": true,
            "accepted_pre_execution_validation_record_present": true,
            "accepted_redaction_proof_count": 1,
            "external_send_requested": true,
            "public_claim_requested": true,
            "release_artifact_write_requested": true,
            "memory_store_write_requested": true,
            "execution_allowed": false,
            "execution_performed": false,
            "memory_store_write_allowed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "external_send_performed": false,
            "public_release_published": false,
            "release_artifact_written": false,
            "activation_allowed": false,
            "reason": "external_send_public_claim_and_release_artifact_denied"
        },
        {
            "id": "write-enable-direct-store-or-rollback-execution",
            "explicit_write_enable_requested": true,
            "write_enable_status": "blocked",
            "accepted_operator_approval_packet_present": true,
            "accepted_pre_execution_validation_record_present": true,
            "accepted_redaction_proof_count": 1,
            "explicit_write_path_enablement_recorded": true,
            "post_write_watchdog_soak_plan_recorded": false,
            "memory_store_mutation_requested": true,
            "rollback_execution_requested": true,
            "memory_store_write_requested": true,
            "execution_allowed": false,
            "execution_performed": false,
            "memory_store_write_allowed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "activation_allowed": false,
            "reason": "post_write_watchdog_soak_plan_and_live_activation_boundary_required"
        }
    ]);
    let source_no_write_sink_boundary_report_sha256 = sha256_json_value(&no_write_sink);
    let boundary_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-live-mutation-operator-write-execution-write-enable-fixture-boundary-v1:{}:{}",
        route_matrix.route_count, source_no_write_sink_boundary_report_sha256,
    ));
    let report_ready = route_matrix.ready
        && route_count_source_command_accepted
        && no_write_sink_ready
        && write_enable_surfaces.len() == 10
        && write_enable_fixtures.as_array().map(std::vec::Vec::len) == Some(7)
        && denied_by.len() == 13;

    let allowed_next_actions = serde_json::json!([
        {
            "action": "run_memory_write_execution_write_enable_fixture_boundary_require_live_gate",
            "status": "allowed_verification_only",
            "records_write_enable_fixture": false,
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
            "action": "prepare_memory_write_execution_post_write_validation_dry_run_boundary",
            "status": "allowed_report_only_next_slice",
            "writes_memory": false,
            "writes_kg": false,
            "invokes_provider": false,
            "sends_externally": false
        }
    ]);

    let mut side_effects = serde_json::Map::new();
    for key in [
        "memory_store_mutated",
        "memory_store_write_performed",
        "memory_write_request_recorded",
        "memory_write_request_persisted",
        "memory_write_approval_packet_recorded",
        "memory_write_approval_packet_persisted",
        "memory_write_execution_preflight_recorded",
        "memory_write_execution_preflight_persisted",
        "memory_write_execution_denial_matrix_recorded",
        "memory_write_execution_denial_matrix_persisted",
        "memory_write_execution_no_write_sink_contract_recorded",
        "memory_write_execution_no_write_sink_contract_persisted",
        "memory_write_execution_write_enable_fixture_recorded",
        "memory_write_execution_write_enable_fixture_persisted",
        "memory_write_execution_write_enable_fixture_materialized",
        "memory_write_execution_write_enable_fixture_filesystem_written",
        "explicit_write_enablement_recorded",
        "explicit_write_enablement_persisted",
        "pre_execution_validation_recorded",
        "pre_execution_validation_persisted",
        "payload_plaintext_persisted",
        "raw_payload_inspected",
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
        "approval_record_persisted",
        "preflight_record_persisted",
        "denial_matrix_persisted",
        "no_write_sink_contract_persisted",
        "write_enable_fixture_persisted",
        "receipt_persisted",
        "launchd_mutated",
        "service_restarted",
        "install_executed",
        "active_binary_mutated",
        "rollback_executed",
        "credential_read",
        "secret_file_read",
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
        "hepta_memory_live_mutation_operator_write_execution_write_enable_fixture_boundary_route"
    );
    insert_report_json!(
        "endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_WRITE_ENABLE_FIXTURE_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_command",
        "/hepta-memory-live-mutation-operator-write-execution-write-enable-fixture-boundary --json"
    );
    insert_report_json!("native_route", true);
    insert_report_json!("side_effect_free", true);
    insert_report_json!("audit_date", "2026-07-02");
    insert_report_json!(
        "memory_write_execution_write_enable_fixture_boundary_schema_version",
        "memory_write_execution_write_enable_fixture_boundary_v1"
    );
    insert_report_json!(
        "memory_write_execution_write_enable_fixture_boundary_ready",
        report_ready
    );
    insert_report_json!(
        "write_enable_fixture_mode",
        "memory_write_execution_write_enable_fixture_non_activation"
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
        "source_memory_write_execution_no_write_sink_contract_boundary_endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_NO_WRITE_SINK_CONTRACT_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_memory_write_execution_no_write_sink_contract_boundary_ready",
        no_write_sink_ready
    );
    insert_report_json!(
        "source_memory_write_execution_no_write_sink_contract_boundary_report_sha256",
        source_no_write_sink_boundary_report_sha256
    );
    report.insert(
        "source_memory_write_execution_denial_matrix_boundary_report_sha256".to_string(),
        no_write_sink
            .get("source_memory_write_execution_denial_matrix_boundary_report_sha256")
            .cloned()
            .unwrap_or_else(|| serde_json::json!("")),
    );
    insert_report_json!("minimum_required_samples", 24);
    insert_report_json!("memory_write_execution_write_enable_fixture_ready", true);
    insert_report_json!("memory_write_execution_no_write_sink_contract_ready", true);
    insert_report_json!("memory_write_execution_denial_matrix_ready", true);
    insert_report_json!(
        "required_pre_execution_validation_check_count",
        json_u64(
            &no_write_sink,
            "required_pre_execution_validation_check_count"
        )
    );
    insert_report_json!("accepted_pre_execution_validation_check_count", 0);
    insert_report_json!("required_write_enable_surface_count", 10);
    insert_report_json!("ready_write_enable_surface_count", 10);
    insert_report_json!("side_effect_free_write_enable_surface_count", 10);
    insert_report_json!("required_write_enable_fixture_count", 7);
    insert_report_json!("write_enable_fixture_count", 7);
    insert_report_json!("blocked_write_enable_fixture_count", 7);
    insert_report_json!("allowed_write_enable_fixture_count", 0);
    insert_report_json!("explicit_write_enable_requested_fixture_count", 7);
    insert_report_json!("write_enable_denied_missing_approval_preflight_count", 1);
    insert_report_json!("write_enable_denied_missing_operator_scope_count", 1);
    insert_report_json!("write_enable_denied_allowlist_mismatch_count", 1);
    insert_report_json!("write_enable_denied_payload_binding_count", 1);
    insert_report_json!(
        "write_enable_denied_stale_soak_rollback_validation_count",
        1
    );
    insert_report_json!("write_enable_denied_public_artifact_count", 1);
    insert_report_json!("write_enable_denied_store_or_rollback_execution_count", 1);
    insert_report_json!("memory_write_execution_denied_count", 7);
    insert_report_json!("memory_write_execution_allowed_count", 0);
    insert_report_json!("memory_write_execution_performed_count", 0);
    insert_report_json!("memory_store_write_requested_fixture_count", 7);
    insert_report_json!("memory_store_write_allowed_count", 0);
    insert_report_json!("memory_store_write_performed_count", 0);

    for key in [
        "memory_store_mutation_allowed",
        "memory_store_mutated",
        "explicit_write_enablement_recorded",
        "explicit_write_enablement_persisted",
        "explicit_write_enablement_accepted",
        "write_enable_fixture_recorded",
        "write_enable_fixture_persisted",
        "write_enable_fixture_materialized",
        "write_enable_fixture_filesystem_written",
        "memory_write_execution_no_write_sink_contract_recorded",
        "memory_write_execution_no_write_sink_contract_persisted",
        "pre_execution_validation_recorded",
        "pre_execution_validation_persisted",
        "pre_execution_validation_accepted",
        "memory_write_approval_packet_recorded",
        "memory_write_approval_packet_persisted",
        "memory_write_approval_packet_accepted",
        "memory_write_request_recorded",
        "memory_write_request_accepted",
        "memory_write_request_persisted",
        "operator_approval_recorded",
        "operator_identity_hash_recorded",
        "operator_approval_signature_hash_recorded",
        "operator_approval_timestamp_recorded",
        "single_surface_activation_scope_recorded",
        "memory_namespace_recorded",
        "memory_write_operation_recorded",
        "memory_retention_class_recorded",
        "accepted_redaction_proof_recorded",
        "source_report_hash_bindings_recorded",
        "raw_payload_sha256_bound",
        "redacted_payload_summary_sha256_bound",
        "raw_payload_plaintext_recorded",
        "raw_payload_plaintext_persisted",
        "fresh_pre_activation_soak_evidence_recorded",
        "rollback_plan_recorded",
        "post_write_validation_plan_recorded",
        "post_write_watchdog_soak_plan_recorded",
        "memory_write_execution_allowed",
        "memory_write_execution_ready",
        "memory_write_execution_performed",
        "memory_store_write_path_enabled",
        "no_write_sink_write_path_enabled_by_default",
        "live_mutation_execution_ready",
        "rollback_execution_allowed",
        "rollback_executed",
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
        "public_release_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "install_executed",
        "service_restarted",
        "active_binary_mutated",
    ] {
        report.insert(key.to_string(), serde_json::json!(false));
    }
    insert_report_json!("accepted_redaction_proof_count", 0);
    report.insert(
        "write_enable_surfaces".to_string(),
        serde_json::json!(write_enable_surfaces),
    );
    report.insert("write_enable_fixtures".to_string(), write_enable_fixtures);
    report.insert(
        "denied_by_write_enable_fixture".to_string(),
        serde_json::json!(denied_by),
    );
    insert_report_json!("denied_by_write_enable_fixture_count", 13);
    report.insert("allowed_next_actions".to_string(), allowed_next_actions);
    report.insert(
        "side_effects".to_string(),
        serde_json::Value::Object(side_effects),
    );

    serde_json::Value::Object(report)
}

fn hepta_memory_live_mutation_operator_write_execution_post_write_validation_dry_run_boundary_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let write_enable =
        hepta_memory_live_mutation_operator_write_execution_write_enable_fixture_boundary_report();

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
    let write_enable_ready = json_bool(
        &write_enable,
        "memory_write_execution_write_enable_fixture_boundary_ready",
    ) && json_bool(
        &write_enable,
        "memory_write_execution_write_enable_fixture_ready",
    ) && json_u64(&write_enable, "required_write_enable_surface_count")
        == 10
        && json_u64(&write_enable, "ready_write_enable_surface_count") == 10
        && json_u64(&write_enable, "write_enable_fixture_count") == 7
        && json_u64(&write_enable, "blocked_write_enable_fixture_count") == 7
        && json_u64(&write_enable, "allowed_write_enable_fixture_count") == 0
        && json_u64(&write_enable, "denied_by_write_enable_fixture_count") == 13
        && !json_bool(&write_enable, "memory_write_execution_allowed")
        && !json_bool(&write_enable, "memory_write_execution_performed")
        && !json_bool(&write_enable, "memory_store_write_path_enabled")
        && json_u64(&write_enable, "memory_store_write_performed_count") == 0
        && !json_bool(&write_enable, "memory_store_mutated")
        && !json_bool(&write_enable, "rollback_executed")
        && !json_bool(&write_enable, "live_kg_write_performed")
        && !json_bool(&write_enable, "provider_invoked")
        && !json_bool(&write_enable, "model_invoked")
        && !json_bool(&write_enable, "credential_read")
        && !json_bool(&write_enable, "external_send_performed")
        && !json_bool(&write_enable, "release_artifact_written")
        && !json_bool(&write_enable, "public_release_claimed")
        && !json_bool(&write_enable, "active_binary_mutated")
        && side_effects_all_false(&write_enable);

    let post_write_validation_surfaces = vec![
        "pre_write_memory_store_baseline_hash_required",
        "accepted_write_result_receipt_hash_required",
        "post_write_memory_store_hash_and_diff_scope_required",
        "route_readiness_regression_check_required",
        "active_dependency_isolation_regression_check_required",
        "post_write_watchdog_soak_plan_required",
        "rollback_validation_plan_required",
        "audit_redaction_validation_required",
        "operator_post_write_acceptance_required",
    ];
    let post_write_validation_fixtures = serde_json::json!([
        {
            "id": "post-write-missing-pre-write-baseline",
            "post_write_validation_requested": true,
            "validation_status": "blocked",
            "pre_write_memory_store_hash_recorded": false,
            "write_result_receipt_hash_recorded": false,
            "post_write_memory_store_hash_recorded": false,
            "validation_allowed": false,
            "validation_performed": false,
            "validation_passed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "activation_allowed": false,
            "reason": "pre_write_baseline_and_write_receipt_required"
        },
        {
            "id": "post-write-missing-write-result-receipt",
            "post_write_validation_requested": true,
            "validation_status": "blocked",
            "pre_write_memory_store_hash_recorded": true,
            "write_result_receipt_hash_recorded": false,
            "write_result_receipt_accepted": false,
            "post_write_memory_store_hash_recorded": true,
            "validation_allowed": false,
            "validation_performed": false,
            "validation_passed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "activation_allowed": false,
            "reason": "accepted_write_result_receipt_hash_required"
        },
        {
            "id": "post-write-store-hash-mismatch",
            "post_write_validation_requested": true,
            "validation_status": "blocked",
            "pre_write_memory_store_hash_recorded": true,
            "write_result_receipt_hash_recorded": true,
            "write_result_receipt_accepted": true,
            "post_write_memory_store_hash_recorded": true,
            "post_write_memory_store_hash_changed": true,
            "diff_scope_allowlisted": false,
            "validation_allowed": false,
            "validation_performed": false,
            "validation_passed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "activation_allowed": false,
            "reason": "post_write_store_hash_change_requires_allowlisted_diff_scope"
        },
        {
            "id": "post-write-route-or-dependency-regression",
            "post_write_validation_requested": true,
            "validation_status": "blocked",
            "route_readiness_regression_detected": true,
            "active_dependency_isolation_regression_detected": true,
            "validation_allowed": false,
            "validation_performed": false,
            "validation_passed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "activation_allowed": false,
            "reason": "route_readiness_and_dependency_isolation_must_remain_ready"
        },
        {
            "id": "post-write-watchdog-soak-missing-or-failed",
            "post_write_validation_requested": true,
            "validation_status": "blocked",
            "post_write_watchdog_soak_plan_recorded": false,
            "post_write_watchdog_soak_performed": false,
            "post_write_watchdog_soak_passed": false,
            "validation_allowed": false,
            "validation_performed": false,
            "validation_passed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "activation_allowed": false,
            "reason": "post_write_watchdog_soak_plan_and_success_required"
        },
        {
            "id": "post-write-rollback-validation-missing",
            "post_write_validation_requested": true,
            "validation_status": "blocked",
            "rollback_validation_plan_recorded": false,
            "rollback_validation_performed": false,
            "rollback_validation_passed": false,
            "rollback_execution_requested": true,
            "validation_allowed": false,
            "validation_performed": false,
            "validation_passed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "activation_allowed": false,
            "reason": "rollback_validation_plan_required_without_executing_rollback"
        },
        {
            "id": "post-write-audit-redaction-or-secret-leak",
            "post_write_validation_requested": true,
            "validation_status": "blocked",
            "audit_redaction_validation_recorded": false,
            "raw_payload_plaintext_recorded": true,
            "secret_material_read": true,
            "validation_allowed": false,
            "validation_performed": false,
            "validation_passed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "activation_allowed": false,
            "reason": "audit_redaction_validation_required_and_secret_material_forbidden"
        },
        {
            "id": "post-write-external-send-or-release-artifact-attempt",
            "post_write_validation_requested": true,
            "validation_status": "blocked",
            "external_send_requested": true,
            "public_claim_requested": true,
            "release_artifact_write_requested": true,
            "validation_allowed": false,
            "validation_performed": false,
            "validation_passed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "external_send_performed": false,
            "public_release_published": false,
            "release_artifact_written": false,
            "rollback_executed": false,
            "activation_allowed": false,
            "reason": "post_write_validation_cannot_send_publish_or_write_release_artifacts"
        }
    ]);
    let denied_by = vec![
        "pre_write_memory_store_baseline_hash_required",
        "accepted_write_result_receipt_hash_required",
        "post_write_memory_store_hash_required",
        "allowlisted_diff_scope_required",
        "route_readiness_regression_denied",
        "active_dependency_isolation_regression_denied",
        "post_write_watchdog_soak_plan_required",
        "post_write_watchdog_soak_success_required",
        "rollback_validation_plan_required",
        "rollback_execution_denied",
        "audit_redaction_validation_required",
        "secret_material_read_denied",
        "external_send_public_claim_release_artifact_denied",
        "live_mutation_execution_denied",
    ];

    let source_write_enable_boundary_report_sha256 = sha256_json_value(&write_enable);
    let boundary_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-live-mutation-operator-write-execution-post-write-validation-dry-run-boundary-v1:{}:{}",
        route_matrix.route_count, source_write_enable_boundary_report_sha256,
    ));
    let report_ready = route_matrix.ready
        && route_count_source_command_accepted
        && write_enable_ready
        && post_write_validation_surfaces.len() == 9
        && post_write_validation_fixtures
            .as_array()
            .map(std::vec::Vec::len)
            == Some(8)
        && denied_by.len() == 14;

    let allowed_next_actions = serde_json::json!([
        {
            "action": "run_memory_write_execution_post_write_validation_dry_run_boundary_require_live_gate",
            "status": "allowed_verification_only",
            "records_post_write_validation": false,
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
            "action": "prepare_memory_write_execution_post_write_operator_acceptance_denial_boundary",
            "status": "allowed_report_only_next_slice",
            "writes_memory": false,
            "executes_rollback": false,
            "writes_kg": false,
            "invokes_provider": false,
            "sends_externally": false
        }
    ]);

    let mut side_effects = serde_json::Map::new();
    for key in [
        "memory_store_mutated",
        "memory_store_write_performed",
        "memory_write_execution_performed",
        "post_write_validation_recorded",
        "post_write_validation_persisted",
        "post_write_validation_accepted",
        "post_write_validation_performed",
        "post_write_validation_report_written",
        "post_write_watchdog_soak_performed",
        "post_write_route_regression_check_performed",
        "post_write_dependency_isolation_check_performed",
        "rollback_validation_performed",
        "rollback_executed",
        "write_result_receipt_recorded",
        "write_result_receipt_persisted",
        "write_result_receipt_accepted",
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
        "launchd_mutated",
        "service_restarted",
        "install_executed",
        "active_binary_mutated",
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
        "hepta_memory_live_mutation_operator_write_execution_post_write_validation_dry_run_boundary_route"
    );
    insert_report_json!(
        "endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_POST_WRITE_VALIDATION_DRY_RUN_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_command",
        "/hepta-memory-live-mutation-operator-write-execution-post-write-validation-dry-run-boundary --json"
    );
    insert_report_json!("native_route", true);
    insert_report_json!("side_effect_free", true);
    insert_report_json!("audit_date", "2026-07-02");
    insert_report_json!(
        "memory_write_execution_post_write_validation_dry_run_boundary_schema_version",
        "memory_write_execution_post_write_validation_dry_run_boundary_v1"
    );
    insert_report_json!(
        "memory_write_execution_post_write_validation_dry_run_boundary_ready",
        report_ready
    );
    insert_report_json!(
        "post_write_validation_mode",
        "memory_write_execution_post_write_validation_dry_run_non_activation"
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
        "source_memory_write_execution_write_enable_fixture_boundary_endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_WRITE_ENABLE_FIXTURE_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_memory_write_execution_write_enable_fixture_boundary_ready",
        write_enable_ready
    );
    insert_report_json!(
        "source_memory_write_execution_write_enable_fixture_boundary_report_sha256",
        source_write_enable_boundary_report_sha256
    );
    report.insert(
        "source_memory_write_execution_no_write_sink_contract_boundary_report_sha256".to_string(),
        write_enable
            .get("source_memory_write_execution_no_write_sink_contract_boundary_report_sha256")
            .cloned()
            .unwrap_or_else(|| serde_json::json!("")),
    );
    report.insert(
        "source_memory_write_execution_denial_matrix_boundary_report_sha256".to_string(),
        write_enable
            .get("source_memory_write_execution_denial_matrix_boundary_report_sha256")
            .cloned()
            .unwrap_or_else(|| serde_json::json!("")),
    );
    insert_report_json!("minimum_required_samples", 24);
    insert_report_json!(
        "memory_write_execution_post_write_validation_dry_run_ready",
        true
    );
    insert_report_json!("memory_write_execution_write_enable_fixture_ready", true);
    insert_report_json!("memory_write_execution_no_write_sink_contract_ready", true);
    insert_report_json!(
        "required_write_enable_surface_count",
        json_u64(&write_enable, "required_write_enable_surface_count")
    );
    insert_report_json!(
        "ready_write_enable_surface_count",
        json_u64(&write_enable, "ready_write_enable_surface_count")
    );
    insert_report_json!("required_post_write_validation_surface_count", 9);
    insert_report_json!("ready_post_write_validation_surface_count", 9);
    insert_report_json!("side_effect_free_post_write_validation_surface_count", 9);
    insert_report_json!("required_post_write_validation_fixture_count", 8);
    insert_report_json!("post_write_validation_fixture_count", 8);
    insert_report_json!("blocked_post_write_validation_fixture_count", 8);
    insert_report_json!("allowed_post_write_validation_fixture_count", 0);
    insert_report_json!("passed_post_write_validation_fixture_count", 0);
    insert_report_json!("post_write_validation_denied_count", 8);
    insert_report_json!("post_write_validation_performed_count", 0);

    for key in [
        "post_write_validation_recorded",
        "post_write_validation_persisted",
        "post_write_validation_accepted",
        "post_write_validation_performed",
        "post_write_validation_report_written",
        "post_write_watchdog_soak_plan_recorded",
        "post_write_watchdog_soak_plan_persisted",
        "post_write_watchdog_soak_performed",
        "post_write_watchdog_soak_passed",
        "post_write_route_regression_check_performed",
        "post_write_route_regression_passed",
        "post_write_dependency_isolation_check_performed",
        "post_write_dependency_isolation_passed",
        "post_write_memory_store_hash_recorded",
        "post_write_memory_store_hash_persisted",
        "post_write_memory_store_hash_changed",
        "pre_write_memory_store_hash_recorded",
        "write_result_receipt_hash_recorded",
        "write_result_receipt_accepted",
        "rollback_validation_plan_recorded",
        "rollback_validation_performed",
        "rollback_validation_passed",
        "audit_redaction_validation_recorded",
        "audit_redaction_validation_passed",
        "raw_payload_plaintext_recorded",
        "raw_payload_plaintext_persisted",
        "secret_material_read",
        "memory_write_execution_allowed",
        "memory_write_execution_ready",
        "memory_write_execution_performed",
        "memory_store_write_path_enabled",
        "memory_store_write_allowed",
        "memory_store_write_performed",
        "memory_store_mutation_allowed",
        "memory_store_mutated",
        "live_mutation_execution_ready",
        "rollback_execution_allowed",
        "rollback_executed",
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
        "public_release_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "install_executed",
        "service_restarted",
        "active_binary_mutated",
    ] {
        report.insert(key.to_string(), serde_json::json!(false));
    }
    insert_report_json!("memory_store_write_performed_count", 0);
    report.insert(
        "post_write_validation_surfaces".to_string(),
        serde_json::json!(post_write_validation_surfaces),
    );
    report.insert(
        "post_write_validation_fixtures".to_string(),
        post_write_validation_fixtures,
    );
    report.insert(
        "denied_by_post_write_validation".to_string(),
        serde_json::json!(denied_by),
    );
    insert_report_json!("denied_by_post_write_validation_count", 14);
    report.insert("allowed_next_actions".to_string(), allowed_next_actions);
    report.insert(
        "side_effects".to_string(),
        serde_json::Value::Object(side_effects),
    );

    serde_json::Value::Object(report)
}

fn hepta_memory_live_mutation_operator_write_execution_post_write_operator_acceptance_denial_boundary_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let post_write =
        hepta_memory_live_mutation_operator_write_execution_post_write_validation_dry_run_boundary_report();

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
    let post_write_ready =
        json_bool(
            &post_write,
            "memory_write_execution_post_write_validation_dry_run_boundary_ready",
        ) && json_bool(
            &post_write,
            "memory_write_execution_post_write_validation_dry_run_ready",
        ) && json_u64(&post_write, "required_post_write_validation_surface_count") == 9
            && json_u64(&post_write, "ready_post_write_validation_surface_count") == 9
            && json_u64(&post_write, "post_write_validation_fixture_count") == 8
            && json_u64(&post_write, "blocked_post_write_validation_fixture_count") == 8
            && json_u64(&post_write, "allowed_post_write_validation_fixture_count") == 0
            && json_u64(&post_write, "passed_post_write_validation_fixture_count") == 0
            && json_u64(&post_write, "post_write_validation_performed_count") == 0
            && !json_bool(&post_write, "post_write_validation_recorded")
            && !json_bool(&post_write, "post_write_validation_persisted")
            && !json_bool(&post_write, "post_write_validation_accepted")
            && !json_bool(&post_write, "post_write_validation_performed")
            && !json_bool(&post_write, "memory_write_execution_allowed")
            && !json_bool(&post_write, "memory_write_execution_ready")
            && !json_bool(&post_write, "memory_write_execution_performed")
            && !json_bool(&post_write, "memory_store_write_path_enabled")
            && !json_bool(&post_write, "memory_store_write_allowed")
            && !json_bool(&post_write, "memory_store_write_performed")
            && json_u64(&post_write, "memory_store_write_performed_count") == 0
            && !json_bool(&post_write, "memory_store_mutated")
            && !json_bool(&post_write, "rollback_executed")
            && !json_bool(&post_write, "live_kg_write_performed")
            && !json_bool(&post_write, "provider_invoked")
            && !json_bool(&post_write, "model_invoked")
            && !json_bool(&post_write, "credential_read")
            && !json_bool(&post_write, "external_send_performed")
            && !json_bool(&post_write, "release_artifact_written")
            && !json_bool(&post_write, "public_release_claimed")
            && !json_bool(&post_write, "active_binary_mutated")
            && side_effects_all_false(&post_write);

    let operator_acceptance_surfaces = vec![
        "accepted_post_write_validation_report_required",
        "operator_identity_signature_timestamp_required",
        "single_surface_acceptance_scope_required",
        "pre_and_post_memory_store_hash_binding_required",
        "accepted_write_result_receipt_hash_required",
        "allowlisted_diff_scope_required",
        "post_write_watchdog_soak_success_required",
        "route_and_dependency_regression_absence_required",
        "rollback_validation_and_no_rollback_execution_required",
        "audit_redaction_and_no_secret_material_required",
        "activation_closure_packet_required",
    ];
    let operator_acceptance_fixtures = serde_json::json!([
        {
            "id": "operator-acceptance-missing-post-write-validation",
            "operator_acceptance_requested": true,
            "acceptance_status": "blocked",
            "accepted_post_write_validation_report_present": false,
            "operator_identity_hash_recorded": false,
            "validation_accepted": false,
            "acceptance_allowed": false,
            "acceptance_performed": false,
            "acceptance_accepted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "accepted_post_write_validation_report_required"
        },
        {
            "id": "operator-acceptance-missing-operator-signature",
            "operator_acceptance_requested": true,
            "acceptance_status": "blocked",
            "accepted_post_write_validation_report_present": true,
            "operator_identity_hash_recorded": false,
            "operator_acceptance_signature_hash_recorded": false,
            "operator_acceptance_timestamp_recorded": false,
            "operator_single_surface_scope_recorded": false,
            "acceptance_allowed": false,
            "acceptance_performed": false,
            "acceptance_accepted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "operator_identity_signature_timestamp_and_scope_required"
        },
        {
            "id": "operator-acceptance-receipt-or-store-hash-mismatch",
            "operator_acceptance_requested": true,
            "acceptance_status": "blocked",
            "accepted_post_write_validation_report_present": true,
            "operator_identity_hash_recorded": true,
            "operator_acceptance_signature_hash_recorded": true,
            "operator_single_surface_scope_recorded": true,
            "write_result_receipt_hash_bound": false,
            "pre_write_memory_store_hash_bound": true,
            "post_write_memory_store_hash_bound": true,
            "post_write_diff_scope_accepted": false,
            "acceptance_allowed": false,
            "acceptance_performed": false,
            "acceptance_accepted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "write_receipt_store_hash_and_diff_scope_bindings_required"
        },
        {
            "id": "operator-acceptance-route-or-dependency-regression",
            "operator_acceptance_requested": true,
            "acceptance_status": "blocked",
            "accepted_post_write_validation_report_present": true,
            "operator_identity_hash_recorded": true,
            "operator_acceptance_signature_hash_recorded": true,
            "route_readiness_regression_detected": true,
            "active_dependency_isolation_regression_detected": true,
            "acceptance_allowed": false,
            "acceptance_performed": false,
            "acceptance_accepted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "route_readiness_and_dependency_isolation_must_remain_ready"
        },
        {
            "id": "operator-acceptance-watchdog-soak-missing",
            "operator_acceptance_requested": true,
            "acceptance_status": "blocked",
            "accepted_post_write_validation_report_present": true,
            "operator_identity_hash_recorded": true,
            "operator_acceptance_signature_hash_recorded": true,
            "post_write_watchdog_soak_evidence_accepted": false,
            "acceptance_allowed": false,
            "acceptance_performed": false,
            "acceptance_accepted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "post_write_watchdog_soak_success_required"
        },
        {
            "id": "operator-acceptance-rollback-validation-missing-or-execution",
            "operator_acceptance_requested": true,
            "acceptance_status": "blocked",
            "accepted_post_write_validation_report_present": true,
            "operator_identity_hash_recorded": true,
            "operator_acceptance_signature_hash_recorded": true,
            "rollback_validation_accepted": false,
            "rollback_execution_requested": true,
            "acceptance_allowed": false,
            "acceptance_performed": false,
            "acceptance_accepted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "rollback_validation_required_and_rollback_execution_denied"
        },
        {
            "id": "operator-acceptance-redaction-or-secret-violation",
            "operator_acceptance_requested": true,
            "acceptance_status": "blocked",
            "accepted_post_write_validation_report_present": true,
            "operator_identity_hash_recorded": true,
            "operator_acceptance_signature_hash_recorded": true,
            "audit_redaction_validation_accepted": false,
            "raw_payload_plaintext_recorded": true,
            "secret_material_read": true,
            "acceptance_allowed": false,
            "acceptance_performed": false,
            "acceptance_accepted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "audit_redaction_validation_required_and_secret_material_forbidden"
        },
        {
            "id": "operator-acceptance-multisurface-or-direct-activation",
            "operator_acceptance_requested": true,
            "acceptance_status": "blocked",
            "accepted_post_write_validation_report_present": true,
            "operator_identity_hash_recorded": true,
            "operator_acceptance_signature_hash_recorded": true,
            "operator_single_surface_scope_recorded": false,
            "multi_surface_activation_requested": true,
            "direct_live_mutation_execution_requested": true,
            "acceptance_allowed": false,
            "acceptance_performed": false,
            "acceptance_accepted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "single_surface_scope_required_and_direct_activation_denied"
        },
        {
            "id": "operator-acceptance-public-or-external-output-attempt",
            "operator_acceptance_requested": true,
            "acceptance_status": "blocked",
            "accepted_post_write_validation_report_present": true,
            "operator_identity_hash_recorded": true,
            "operator_acceptance_signature_hash_recorded": true,
            "external_send_requested": true,
            "public_claim_requested": true,
            "release_artifact_write_requested": true,
            "acceptance_allowed": false,
            "acceptance_performed": false,
            "acceptance_accepted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "external_send_performed": false,
            "public_release_published": false,
            "release_artifact_written": false,
            "rollback_executed": false,
            "reason": "operator_acceptance_cannot_send_publish_or_write_release_artifacts"
        }
    ]);
    let denied_by = vec![
        "accepted_post_write_validation_report_required",
        "operator_identity_required",
        "operator_acceptance_signature_required",
        "operator_acceptance_timestamp_required",
        "single_surface_acceptance_scope_required",
        "pre_write_memory_store_hash_binding_required",
        "post_write_memory_store_hash_binding_required",
        "write_result_receipt_hash_binding_required",
        "allowlisted_diff_scope_required",
        "route_readiness_regression_denied",
        "active_dependency_isolation_regression_denied",
        "post_write_watchdog_soak_success_required",
        "rollback_validation_required",
        "rollback_execution_denied",
        "audit_redaction_validation_required",
        "secret_material_read_denied",
        "multi_surface_activation_denied",
        "direct_live_mutation_execution_denied",
        "external_send_public_claim_release_artifact_denied",
        "activation_closure_packet_required",
        "live_mutation_execution_denied",
    ];

    let source_post_write_boundary_report_sha256 = sha256_json_value(&post_write);
    let boundary_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-live-mutation-operator-write-execution-post-write-operator-acceptance-denial-boundary-v1:{}:{}",
        route_matrix.route_count, source_post_write_boundary_report_sha256,
    ));
    let report_ready = route_matrix.ready
        && route_count_source_command_accepted
        && post_write_ready
        && operator_acceptance_surfaces.len() == 11
        && operator_acceptance_fixtures
            .as_array()
            .map(std::vec::Vec::len)
            == Some(9)
        && denied_by.len() == 21;

    let allowed_next_actions = serde_json::json!([
        {
            "action": "run_memory_write_execution_post_write_operator_acceptance_denial_boundary_require_live_gate",
            "status": "allowed_verification_only",
            "records_operator_acceptance": false,
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
            "action": "prepare_memory_write_execution_activation_closure_denial_boundary",
            "status": "allowed_report_only_next_slice",
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
        "post_write_validation_accepted",
        "operator_post_write_acceptance_recorded",
        "operator_post_write_acceptance_persisted",
        "operator_post_write_acceptance_performed",
        "operator_post_write_acceptance_materialized",
        "operator_post_write_acceptance_filesystem_written",
        "accepted_post_write_validation_report_recorded",
        "accepted_post_write_validation_report_persisted",
        "activation_closure_packet_recorded",
        "activation_closure_packet_persisted",
        "activation_closure_packet_materialized",
        "activation_closure_filesystem_written",
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
        "launchd_mutated",
        "service_restarted",
        "install_executed",
        "active_binary_mutated",
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
        "hepta_memory_live_mutation_operator_write_execution_post_write_operator_acceptance_denial_boundary_route"
    );
    insert_report_json!(
        "endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_POST_WRITE_OPERATOR_ACCEPTANCE_DENIAL_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_command",
        "/hepta-memory-live-mutation-operator-write-execution-post-write-operator-acceptance-denial-boundary --json"
    );
    insert_report_json!("native_route", true);
    insert_report_json!("side_effect_free", true);
    insert_report_json!("audit_date", "2026-07-02");
    insert_report_json!(
        "memory_write_execution_post_write_operator_acceptance_denial_boundary_schema_version",
        "memory_write_execution_post_write_operator_acceptance_denial_boundary_v1"
    );
    insert_report_json!(
        "memory_write_execution_post_write_operator_acceptance_denial_boundary_ready",
        report_ready
    );
    insert_report_json!(
        "operator_acceptance_denial_mode",
        "memory_write_execution_post_write_operator_acceptance_denial_non_activation"
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
        "source_memory_write_execution_post_write_validation_dry_run_boundary_endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_POST_WRITE_VALIDATION_DRY_RUN_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_memory_write_execution_post_write_validation_dry_run_boundary_ready",
        post_write_ready
    );
    insert_report_json!(
        "source_memory_write_execution_post_write_validation_dry_run_ready",
        json_bool(
            &post_write,
            "memory_write_execution_post_write_validation_dry_run_ready"
        )
    );
    insert_report_json!(
        "source_memory_write_execution_post_write_validation_dry_run_boundary_report_sha256",
        source_post_write_boundary_report_sha256
    );
    for key in [
        "source_memory_write_execution_write_enable_fixture_boundary_report_sha256",
        "source_memory_write_execution_no_write_sink_contract_boundary_report_sha256",
        "source_memory_write_execution_denial_matrix_boundary_report_sha256",
    ] {
        report.insert(
            key.to_string(),
            post_write
                .get(key)
                .cloned()
                .unwrap_or_else(|| serde_json::json!("")),
        );
    }
    insert_report_json!("minimum_required_samples", 24);
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
        "required_post_write_validation_surface_count",
        json_u64(&post_write, "required_post_write_validation_surface_count")
    );
    insert_report_json!(
        "ready_post_write_validation_surface_count",
        json_u64(&post_write, "ready_post_write_validation_surface_count")
    );
    insert_report_json!("required_operator_acceptance_surface_count", 11);
    insert_report_json!("ready_operator_acceptance_surface_count", 11);
    insert_report_json!("side_effect_free_operator_acceptance_surface_count", 11);
    insert_report_json!("required_operator_acceptance_fixture_count", 9);
    insert_report_json!("operator_acceptance_fixture_count", 9);
    insert_report_json!("blocked_operator_acceptance_fixture_count", 9);
    insert_report_json!("allowed_operator_acceptance_fixture_count", 0);
    insert_report_json!("accepted_operator_acceptance_fixture_count", 0);
    insert_report_json!("operator_acceptance_denied_count", 9);
    insert_report_json!("operator_acceptance_performed_count", 0);

    for key in [
        "operator_post_write_acceptance_recorded",
        "operator_post_write_acceptance_persisted",
        "operator_post_write_acceptance_accepted",
        "operator_post_write_acceptance_performed",
        "operator_post_write_acceptance_materialized",
        "operator_post_write_acceptance_filesystem_written",
        "operator_identity_hash_recorded",
        "operator_acceptance_signature_hash_recorded",
        "operator_acceptance_timestamp_recorded",
        "operator_single_surface_scope_recorded",
        "accepted_post_write_validation_report_recorded",
        "accepted_post_write_validation_report_persisted",
        "accepted_post_write_validation_report_accepted",
        "accepted_post_write_validation_report_hash_bound",
        "write_result_receipt_hash_bound",
        "pre_write_memory_store_hash_bound",
        "post_write_memory_store_hash_bound",
        "post_write_diff_scope_accepted",
        "post_write_watchdog_soak_evidence_accepted",
        "post_write_route_regression_check_accepted",
        "post_write_dependency_isolation_check_accepted",
        "rollback_validation_accepted",
        "rollback_execution_allowed",
        "rollback_executed",
        "audit_redaction_validation_accepted",
        "raw_payload_plaintext_recorded",
        "raw_payload_plaintext_persisted",
        "secret_material_read",
        "activation_closure_packet_recorded",
        "activation_closure_packet_persisted",
        "activation_closure_packet_accepted",
        "activation_closure_packet_materialized",
        "activation_closure_filesystem_written",
        "activation_allowed_by_operator_acceptance",
        "activation_allowed",
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
        "service_restarted",
        "active_binary_mutated",
    ] {
        report.insert(key.to_string(), serde_json::json!(false));
    }
    insert_report_json!("memory_store_write_performed_count", 0);
    report.insert(
        "operator_acceptance_surfaces".to_string(),
        serde_json::json!(operator_acceptance_surfaces),
    );
    report.insert(
        "operator_acceptance_fixtures".to_string(),
        operator_acceptance_fixtures,
    );
    report.insert(
        "denied_by_operator_acceptance".to_string(),
        serde_json::json!(denied_by),
    );
    insert_report_json!("denied_by_operator_acceptance_count", 21);
    report.insert("allowed_next_actions".to_string(), allowed_next_actions);
    report.insert(
        "side_effects".to_string(),
        serde_json::Value::Object(side_effects),
    );

    serde_json::Value::Object(report)
}

fn hepta_memory_live_mutation_operator_write_execution_activation_closure_denial_boundary_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let operator_acceptance =
        hepta_memory_live_mutation_operator_write_execution_post_write_operator_acceptance_denial_boundary_report();

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
    let operator_acceptance_ready = json_bool(
        &operator_acceptance,
        "memory_write_execution_post_write_operator_acceptance_denial_boundary_ready",
    ) && json_bool(
        &operator_acceptance,
        "memory_write_execution_post_write_operator_acceptance_denial_ready",
    ) && json_u64(
        &operator_acceptance,
        "required_operator_acceptance_surface_count",
    ) == 11
        && json_u64(
            &operator_acceptance,
            "ready_operator_acceptance_surface_count",
        ) == 11
        && json_u64(&operator_acceptance, "operator_acceptance_fixture_count") == 9
        && json_u64(
            &operator_acceptance,
            "blocked_operator_acceptance_fixture_count",
        ) == 9
        && json_u64(
            &operator_acceptance,
            "allowed_operator_acceptance_fixture_count",
        ) == 0
        && json_u64(
            &operator_acceptance,
            "accepted_operator_acceptance_fixture_count",
        ) == 0
        && json_u64(&operator_acceptance, "operator_acceptance_performed_count") == 0
        && !json_bool(
            &operator_acceptance,
            "operator_post_write_acceptance_recorded",
        )
        && !json_bool(
            &operator_acceptance,
            "operator_post_write_acceptance_persisted",
        )
        && !json_bool(
            &operator_acceptance,
            "operator_post_write_acceptance_accepted",
        )
        && !json_bool(
            &operator_acceptance,
            "operator_post_write_acceptance_performed",
        )
        && !json_bool(&operator_acceptance, "activation_closure_packet_recorded")
        && !json_bool(&operator_acceptance, "activation_closure_packet_persisted")
        && !json_bool(&operator_acceptance, "activation_closure_packet_accepted")
        && !json_bool(
            &operator_acceptance,
            "activation_closure_packet_materialized",
        )
        && !json_bool(
            &operator_acceptance,
            "activation_closure_filesystem_written",
        )
        && !json_bool(
            &operator_acceptance,
            "activation_allowed_by_operator_acceptance",
        )
        && !json_bool(&operator_acceptance, "activation_allowed")
        && !json_bool(&operator_acceptance, "live_mutation_execution_ready")
        && !json_bool(&operator_acceptance, "live_mutation_execution_allowed")
        && !json_bool(&operator_acceptance, "live_mutation_execution_performed")
        && !json_bool(&operator_acceptance, "memory_write_execution_allowed")
        && !json_bool(&operator_acceptance, "memory_write_execution_ready")
        && !json_bool(&operator_acceptance, "memory_write_execution_performed")
        && !json_bool(&operator_acceptance, "memory_store_write_path_enabled")
        && !json_bool(&operator_acceptance, "memory_store_write_allowed")
        && !json_bool(&operator_acceptance, "memory_store_write_performed")
        && json_u64(&operator_acceptance, "memory_store_write_performed_count") == 0
        && !json_bool(&operator_acceptance, "memory_store_mutated")
        && !json_bool(&operator_acceptance, "rollback_executed")
        && !json_bool(&operator_acceptance, "secret_material_read")
        && !json_bool(&operator_acceptance, "provider_invoked")
        && !json_bool(&operator_acceptance, "model_invoked")
        && !json_bool(&operator_acceptance, "credential_read")
        && !json_bool(&operator_acceptance, "external_send_performed")
        && !json_bool(&operator_acceptance, "release_artifact_written")
        && !json_bool(&operator_acceptance, "public_ga_claimed")
        && !json_bool(&operator_acceptance, "public_release_claimed")
        && !json_bool(&operator_acceptance, "active_binary_mutated")
        && side_effects_all_false(&operator_acceptance);

    let activation_closure_surfaces = vec![
        "accepted_operator_post_write_acceptance_required",
        "accepted_post_write_validation_hash_required",
        "operator_identity_signature_timestamp_required",
        "single_surface_activation_scope_required",
        "pre_post_store_hashes_and_write_receipt_required",
        "allowlisted_diff_scope_required",
        "post_write_watchdog_soak_and_regression_evidence_required",
        "rollback_validation_and_no_rollback_execution_required",
        "audit_redaction_and_no_secret_material_required",
        "activation_closure_packet_id_hash_signature_required",
        "activation_command_disabled_by_default_required",
        "no_external_public_or_release_outputs_required",
    ];
    let activation_closure_fixtures = serde_json::json!([
        {
            "id": "activation-closure-missing-operator-acceptance",
            "activation_closure_requested": true,
            "closure_status": "blocked",
            "accepted_operator_post_write_acceptance_present": false,
            "accepted_operator_post_write_acceptance_hash_bound": false,
            "closure_allowed": false,
            "closure_recorded": false,
            "closure_persisted": false,
            "closure_accepted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "accepted_operator_post_write_acceptance_required"
        },
        {
            "id": "activation-closure-missing-packet-id-or-hash",
            "activation_closure_requested": true,
            "closure_status": "blocked",
            "accepted_operator_post_write_acceptance_present": true,
            "accepted_operator_post_write_acceptance_hash_bound": true,
            "activation_closure_packet_id_recorded": false,
            "activation_closure_packet_hash_bound": false,
            "activation_closure_packet_signature_hash_recorded": false,
            "closure_allowed": false,
            "closure_recorded": false,
            "closure_persisted": false,
            "closure_accepted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "activation_closure_packet_id_hash_and_signature_required"
        },
        {
            "id": "activation-closure-missing-single-surface-scope",
            "activation_closure_requested": true,
            "closure_status": "blocked",
            "accepted_operator_post_write_acceptance_present": true,
            "activation_closure_packet_id_recorded": true,
            "activation_closure_packet_hash_bound": true,
            "operator_single_surface_scope_recorded": false,
            "multi_surface_activation_requested": true,
            "closure_allowed": false,
            "closure_recorded": false,
            "closure_persisted": false,
            "closure_accepted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "single_surface_activation_scope_required"
        },
        {
            "id": "activation-closure-store-hash-or-receipt-mismatch",
            "activation_closure_requested": true,
            "closure_status": "blocked",
            "accepted_operator_post_write_acceptance_present": true,
            "activation_closure_packet_id_recorded": true,
            "activation_closure_packet_hash_bound": true,
            "pre_write_memory_store_hash_bound": true,
            "post_write_memory_store_hash_bound": false,
            "write_result_receipt_hash_bound": false,
            "post_write_diff_scope_accepted": false,
            "closure_allowed": false,
            "closure_recorded": false,
            "closure_persisted": false,
            "closure_accepted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "store_hash_write_receipt_and_diff_scope_bindings_required"
        },
        {
            "id": "activation-closure-regression-or-soak-missing",
            "activation_closure_requested": true,
            "closure_status": "blocked",
            "accepted_operator_post_write_acceptance_present": true,
            "activation_closure_packet_id_recorded": true,
            "activation_closure_packet_hash_bound": true,
            "post_write_watchdog_soak_evidence_accepted": false,
            "route_readiness_regression_detected": true,
            "active_dependency_isolation_regression_detected": true,
            "closure_allowed": false,
            "closure_recorded": false,
            "closure_persisted": false,
            "closure_accepted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "post_write_soak_route_and_dependency_evidence_required"
        },
        {
            "id": "activation-closure-rollback-validation-missing-or-execution",
            "activation_closure_requested": true,
            "closure_status": "blocked",
            "accepted_operator_post_write_acceptance_present": true,
            "activation_closure_packet_id_recorded": true,
            "activation_closure_packet_hash_bound": true,
            "rollback_validation_accepted": false,
            "rollback_execution_requested": true,
            "closure_allowed": false,
            "closure_recorded": false,
            "closure_persisted": false,
            "closure_accepted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "rollback_validation_required_and_rollback_execution_denied"
        },
        {
            "id": "activation-closure-redaction-secret-violation",
            "activation_closure_requested": true,
            "closure_status": "blocked",
            "accepted_operator_post_write_acceptance_present": true,
            "activation_closure_packet_id_recorded": true,
            "activation_closure_packet_hash_bound": true,
            "audit_redaction_validation_accepted": false,
            "raw_payload_plaintext_recorded": true,
            "secret_material_read": true,
            "closure_allowed": false,
            "closure_recorded": false,
            "closure_persisted": false,
            "closure_accepted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "audit_redaction_validation_required_and_secret_material_forbidden"
        },
        {
            "id": "activation-closure-direct-live-mutation-request",
            "activation_closure_requested": true,
            "closure_status": "blocked",
            "accepted_operator_post_write_acceptance_present": true,
            "activation_closure_packet_id_recorded": true,
            "activation_closure_packet_hash_bound": true,
            "direct_live_mutation_execution_requested": true,
            "activation_command_invoked": true,
            "closure_allowed": false,
            "closure_recorded": false,
            "closure_persisted": false,
            "closure_accepted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "activation_command_disabled_and_direct_execution_denied"
        },
        {
            "id": "activation-closure-public-external-release-attempt",
            "activation_closure_requested": true,
            "closure_status": "blocked",
            "accepted_operator_post_write_acceptance_present": true,
            "activation_closure_packet_id_recorded": true,
            "activation_closure_packet_hash_bound": true,
            "external_send_requested": true,
            "public_claim_requested": true,
            "release_artifact_write_requested": true,
            "closure_allowed": false,
            "closure_recorded": false,
            "closure_persisted": false,
            "closure_accepted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "external_send_performed": false,
            "public_release_published": false,
            "release_artifact_written": false,
            "rollback_executed": false,
            "reason": "activation_closure_cannot_send_publish_or_write_release_artifacts"
        },
        {
            "id": "activation-closure-persistence-or-filesystem-write-attempt",
            "activation_closure_requested": true,
            "closure_status": "blocked",
            "accepted_operator_post_write_acceptance_present": true,
            "activation_closure_packet_id_recorded": true,
            "activation_closure_packet_hash_bound": true,
            "activation_closure_packet_materialization_requested": true,
            "activation_closure_filesystem_write_requested": true,
            "activation_closure_ledger_write_requested": true,
            "closure_allowed": false,
            "closure_recorded": false,
            "closure_persisted": false,
            "closure_accepted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "activation_closure_filesystem_written": false,
            "activation_closure_ledger_written": false,
            "rollback_executed": false,
            "reason": "activation_closure_packet_persistence_and_filesystem_writes_denied"
        }
    ]);
    let denied_by = vec![
        "accepted_operator_post_write_acceptance_required",
        "accepted_post_write_validation_hash_required",
        "operator_identity_required",
        "operator_acceptance_signature_required",
        "operator_acceptance_timestamp_required",
        "single_surface_activation_scope_required",
        "activation_closure_packet_id_required",
        "activation_closure_packet_hash_required",
        "activation_closure_packet_signature_required",
        "pre_write_memory_store_hash_binding_required",
        "post_write_memory_store_hash_binding_required",
        "write_result_receipt_hash_binding_required",
        "allowlisted_diff_scope_required",
        "route_readiness_regression_denied",
        "active_dependency_isolation_regression_denied",
        "post_write_watchdog_soak_success_required",
        "rollback_validation_required",
        "rollback_execution_denied",
        "audit_redaction_validation_required",
        "secret_material_read_denied",
        "activation_command_invocation_denied",
        "direct_live_mutation_execution_denied",
        "activation_closure_persistence_denied",
        "external_send_public_claim_release_artifact_denied",
    ];

    let source_operator_acceptance_boundary_report_sha256 = sha256_json_value(&operator_acceptance);
    let boundary_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-live-mutation-operator-write-execution-activation-closure-denial-boundary-v1:{}:{}",
        route_matrix.route_count, source_operator_acceptance_boundary_report_sha256,
    ));
    let report_ready = route_matrix.ready
        && route_count_source_command_accepted
        && operator_acceptance_ready
        && activation_closure_surfaces.len() == 12
        && activation_closure_fixtures
            .as_array()
            .map(std::vec::Vec::len)
            == Some(10)
        && denied_by.len() == 24;

    let allowed_next_actions = serde_json::json!([
        {
            "action": "run_memory_write_execution_activation_closure_denial_boundary_require_live_gate",
            "status": "allowed_verification_only",
            "records_activation_closure_packet": false,
            "invokes_activation_command": false,
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
            "action": "prepare_memory_write_execution_activation_command_no_op_handoff_boundary",
            "status": "allowed_report_only_next_slice",
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
        "accepted_operator_post_write_acceptance_report_recorded",
        "accepted_operator_post_write_acceptance_report_persisted",
        "activation_closure_packet_recorded",
        "activation_closure_packet_persisted",
        "activation_closure_packet_materialized",
        "activation_closure_filesystem_written",
        "activation_closure_ledger_written",
        "activation_command_invoked",
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
        "launchd_mutated",
        "service_restarted",
        "install_executed",
        "active_binary_mutated",
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
        "hepta_memory_live_mutation_operator_write_execution_activation_closure_denial_boundary_route"
    );
    insert_report_json!(
        "endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_CLOSURE_DENIAL_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_command",
        "/hepta-memory-live-mutation-operator-write-execution-activation-closure-denial-boundary --json"
    );
    insert_report_json!("native_route", true);
    insert_report_json!("side_effect_free", true);
    insert_report_json!("audit_date", "2026-07-02");
    insert_report_json!(
        "memory_write_execution_activation_closure_denial_boundary_schema_version",
        "memory_write_execution_activation_closure_denial_boundary_v1"
    );
    insert_report_json!(
        "memory_write_execution_activation_closure_denial_boundary_ready",
        report_ready
    );
    insert_report_json!(
        "activation_closure_denial_mode",
        "memory_write_execution_activation_closure_packet_no_write_denial"
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
        "source_memory_write_execution_post_write_operator_acceptance_denial_boundary_endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_POST_WRITE_OPERATOR_ACCEPTANCE_DENIAL_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_memory_write_execution_post_write_operator_acceptance_denial_boundary_ready",
        operator_acceptance_ready
    );
    insert_report_json!(
        "source_memory_write_execution_post_write_operator_acceptance_denial_ready",
        json_bool(
            &operator_acceptance,
            "memory_write_execution_post_write_operator_acceptance_denial_ready"
        )
    );
    insert_report_json!(
        "source_memory_write_execution_post_write_operator_acceptance_denial_boundary_report_sha256",
        source_operator_acceptance_boundary_report_sha256
    );
    for key in [
        "source_memory_write_execution_post_write_validation_dry_run_boundary_report_sha256",
        "source_memory_write_execution_write_enable_fixture_boundary_report_sha256",
        "source_memory_write_execution_no_write_sink_contract_boundary_report_sha256",
        "source_memory_write_execution_denial_matrix_boundary_report_sha256",
    ] {
        report.insert(
            key.to_string(),
            operator_acceptance
                .get(key)
                .cloned()
                .unwrap_or_else(|| serde_json::json!("")),
        );
    }
    insert_report_json!("minimum_required_samples", 24);
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
        "required_operator_acceptance_surface_count",
        json_u64(
            &operator_acceptance,
            "required_operator_acceptance_surface_count"
        )
    );
    insert_report_json!(
        "ready_operator_acceptance_surface_count",
        json_u64(
            &operator_acceptance,
            "ready_operator_acceptance_surface_count"
        )
    );
    insert_report_json!("required_activation_closure_surface_count", 12);
    insert_report_json!("ready_activation_closure_surface_count", 12);
    insert_report_json!("side_effect_free_activation_closure_surface_count", 12);
    insert_report_json!("required_activation_closure_fixture_count", 10);
    insert_report_json!("activation_closure_fixture_count", 10);
    insert_report_json!("blocked_activation_closure_fixture_count", 10);
    insert_report_json!("allowed_activation_closure_fixture_count", 0);
    insert_report_json!("accepted_activation_closure_fixture_count", 0);
    insert_report_json!("activation_closure_denied_count", 10);
    insert_report_json!("activation_closure_performed_count", 0);

    for key in [
        "accepted_operator_post_write_acceptance_report_recorded",
        "accepted_operator_post_write_acceptance_report_persisted",
        "accepted_operator_post_write_acceptance_report_accepted",
        "accepted_operator_post_write_acceptance_hash_bound",
        "accepted_post_write_validation_report_hash_bound",
        "operator_identity_hash_recorded",
        "operator_acceptance_signature_hash_recorded",
        "operator_acceptance_timestamp_recorded",
        "operator_single_surface_scope_recorded",
        "activation_closure_packet_recorded",
        "activation_closure_packet_persisted",
        "activation_closure_packet_accepted",
        "activation_closure_packet_materialized",
        "activation_closure_packet_id_recorded",
        "activation_closure_packet_hash_bound",
        "activation_closure_packet_signature_hash_recorded",
        "activation_closure_packet_timestamp_recorded",
        "activation_closure_filesystem_written",
        "activation_closure_ledger_written",
        "activation_command_enabled",
        "activation_command_invoked",
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
        "post_write_route_regression_check_accepted",
        "post_write_dependency_isolation_check_accepted",
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
        "service_restarted",
        "active_binary_mutated",
    ] {
        report.insert(key.to_string(), serde_json::json!(false));
    }
    insert_report_json!("memory_store_write_performed_count", 0);
    report.insert(
        "activation_closure_surfaces".to_string(),
        serde_json::json!(activation_closure_surfaces),
    );
    report.insert(
        "activation_closure_fixtures".to_string(),
        activation_closure_fixtures,
    );
    report.insert(
        "denied_by_activation_closure".to_string(),
        serde_json::json!(denied_by),
    );
    insert_report_json!("denied_by_activation_closure_count", 24);
    report.insert("allowed_next_actions".to_string(), allowed_next_actions);
    report.insert(
        "side_effects".to_string(),
        serde_json::Value::Object(side_effects),
    );

    serde_json::Value::Object(report)
}
