
fn hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary_report()
-> serde_json::Value {
    const CANCELLATION_SURFACES: &[&str] = &[
        "source_ordering_monotonicity_denial_boundary_required",
        "source_ordering_monotonicity_result_required",
        "dry_run_execution_result_receipt_cancellation_supersession_matrix_required",
        "dry_run_execution_result_receipt_cancellation_policy_required",
        "dry_run_execution_result_receipt_supersession_policy_required",
        "dry_run_execution_result_receipt_replacement_receipt_denial_required",
        "dry_run_execution_result_receipt_tombstone_delete_marker_denial_required",
        "dry_run_execution_result_receipt_latest_replacement_denial_required",
        "dry_run_execution_result_receipt_completion_ack_replacement_denial_required",
        "dry_run_execution_result_receipt_export_query_replacement_denial_required",
        "dry_run_execution_result_receipt_cancellation_supersession_handoff_required",
        "dry_run_execution_result_receipt_cancellation_state_persistence_forbidden",
        "dry_run_execution_result_receipt_supersession_state_persistence_forbidden",
        "dry_run_execution_execution_forbidden_on_cancellation_supersession_route",
        "production_write_execution_forbidden_on_cancellation_supersession_route",
        "kg_provider_channel_release_install_active_binary_forbidden",
    ];
    const CANCELLATION_DENIALS: &[&str] = &[
        "source_ordering_monotonicity_denial_boundary_required",
        "source_ordering_monotonicity_result_hash_required",
        "source_ordering_policy_hash_required",
        "source_ordering_matrix_required",
        "source_sequence_policy_required",
        "source_late_receipt_denial_required",
        "source_future_receipt_denial_required",
        "source_rollback_sequence_denial_required",
        "source_same_sequence_replacement_denial_required",
        "source_latest_wins_promotion_denial_required",
        "source_sequence_gap_denial_required",
        "approved_production_namespace_required",
        "approved_production_store_required",
        "approved_production_scope_required",
        "production_durable_memory_target_required",
        "cancellation_supersession_matrix_required",
        "cancellation_policy_required",
        "supersession_policy_required",
        "cancellation_request_acceptance_denied",
        "cancellation_recording_denied",
        "cancellation_persistence_denied",
        "cancellation_ledger_write_denied",
        "supersession_request_acceptance_denied",
        "supersession_recording_denied",
        "supersession_persistence_denied",
        "supersession_ledger_write_denied",
        "replacement_receipt_acceptance_denied",
        "replacement_receipt_recording_denied",
        "replacement_receipt_persistence_denied",
        "replacement_receipt_materialization_denied",
        "replacement_receipt_filesystem_write_denied",
        "replacement_receipt_ledger_write_denied",
        "tombstone_delete_marker_acceptance_denied",
        "tombstone_delete_marker_write_denied",
        "latest_replacement_promotion_denied",
        "completion_ack_replacement_denied",
        "export_query_replacement_denied",
        "cancellation_supersession_state_persistence_denied",
        "replacement_authority_derivation_denied",
        "result_receipt_supersession_authority_denied",
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
    const FALSE_CANCELLATION_SIDE_EFFECT_KEYS: &[&str] = &[
        "dry_run_execution_result_receipt_cancellation_request_accepted",
        "dry_run_execution_result_receipt_cancellation_recorded",
        "dry_run_execution_result_receipt_cancellation_persisted",
        "dry_run_execution_result_receipt_cancellation_ledger_written",
        "dry_run_execution_result_receipt_supersession_request_accepted",
        "dry_run_execution_result_receipt_supersession_recorded",
        "dry_run_execution_result_receipt_supersession_persisted",
        "dry_run_execution_result_receipt_supersession_ledger_written",
        "dry_run_execution_result_receipt_replacement_receipt_accepted",
        "dry_run_execution_result_receipt_replacement_receipt_recorded",
        "dry_run_execution_result_receipt_replacement_receipt_persisted",
        "dry_run_execution_result_receipt_replacement_receipt_materialized",
        "dry_run_execution_result_receipt_replacement_receipt_filesystem_written",
        "dry_run_execution_result_receipt_replacement_receipt_ledger_written",
        "dry_run_execution_result_receipt_tombstone_delete_marker_accepted",
        "dry_run_execution_result_receipt_tombstone_delete_marker_written",
        "dry_run_execution_result_receipt_latest_replacement_promoted",
        "dry_run_execution_result_receipt_completion_ack_replaced",
        "dry_run_execution_result_receipt_export_query_replaced",
        "dry_run_execution_result_receipt_cancellation_supersession_state_persisted",
        "dry_run_execution_result_receipt_cancellation_supersession_ledger_written",
        "dry_run_execution_result_receipt_ordering_cursor_persisted",
        "dry_run_execution_result_receipt_monotonic_sequence_recorded",
        "dry_run_execution_result_receipt_replay_state_persisted",
        "dry_run_execution_result_receipt_idempotency_ledger_written",
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
    const TRUE_CANCELLATION_KEYS: &[&str] = &[
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary_performed",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_result_recorded",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_result_accepted",
        "source_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_accepted",
        "dry_run_execution_result_receipt_cancellation_supersession_denial_matrix_bound",
        "dry_run_execution_result_receipt_cancellation_policy_bound",
        "dry_run_execution_result_receipt_supersession_policy_bound",
        "dry_run_execution_result_receipt_cancellation_request_denied",
        "dry_run_execution_result_receipt_supersession_request_denied",
        "dry_run_execution_result_receipt_replacement_receipt_denied",
        "dry_run_execution_result_receipt_tombstone_delete_marker_denied",
        "dry_run_execution_result_receipt_latest_replacement_denied",
        "dry_run_execution_result_receipt_completion_ack_replacement_denied",
        "dry_run_execution_result_receipt_export_query_replacement_denied",
        "dry_run_execution_result_receipt_cancellation_supersession_handoff_bound",
        "dry_run_execution_result_receipt_cancellation_supersession_state_persistence_forbidden",
        "dry_run_execution_result_receipt_replacement_receipt_persistence_forbidden",
        "dry_run_execution_execution_forbidden_on_cancellation_supersession_route",
        "production_write_execution_forbidden_on_cancellation_supersession_route",
        "kg_provider_channel_release_install_active_binary_forbidden",
    ];

    let route_matrix = control_ui_route_parity_report();
    let source = std::thread::Builder::new()
        .name("hepta-memory-production-durable-dry-run-result-receipt-cancellation-source-report".to_string())
        .stack_size(8 * 1024 * 1024)
        .spawn(
            hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_report,
        )
        .ok()
        .and_then(|handle| handle.join().ok())
        .unwrap_or_else(|| {
            serde_json::json!({
                "status": "blocked",
                "memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_ready": false,
                "scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_accepted": false,
                "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_source_report_thread_failed": true
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
    let source_next_action_cancellation = source
        .get("allowed_next_actions")
        .and_then(serde_json::Value::as_array)
        .and_then(|items| items.get(1))
        .map(|item| {
            item.get("action").and_then(serde_json::Value::as_str)
                == Some("prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary")
                && item
                    .get("requires_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary")
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
                .get("scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_result_accepted")
                .and_then(serde_json::Value::as_bool)
                == Some(true)
                && effects
                    .get("dry_run_execution_result_receipt_ordering_cursor_persisted")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("dry_run_execution_result_receipt_monotonic_sequence_recorded")
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
            "memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_ready",
        )
        && json_bool(
            &source,
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_accepted",
        )
        && json_u64(
            &source,
            "accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_fixture_count",
        ) == 1
        && json_u64(
            &source,
            "blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_fixture_count",
        ) == 9
        && json_u64(
            &source,
            "denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_count",
        ) == 55
        && !json_bool(
            &source,
            "dry_run_execution_result_receipt_ordering_cursor_persisted",
        )
        && !json_bool(
            &source,
            "dry_run_execution_result_receipt_monotonic_sequence_recorded",
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
        && source_next_action_cancellation
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
    let source_ordering_boundary_hash_sha256 = json_str(
        &source,
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_hash_sha256",
    );
    let source_ordering_policy_hash_sha256 = json_str(
        &source,
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_policy_hash_sha256",
    );
    let source_ordering_matrix_hash_sha256 = json_str(
        &source,
        "dry_run_execution_result_receipt_ordering_monotonicity_denial_matrix_hash_sha256",
    );
    let source_ordering_sequence_policy_hash_sha256 = json_str(
        &source,
        "dry_run_execution_result_receipt_ordering_sequence_policy_hash_sha256",
    );
    let source_late_receipt_denial_hash_sha256 = json_str(
        &source,
        "dry_run_execution_result_receipt_late_receipt_denial_hash_sha256",
    );
    let source_future_receipt_denial_hash_sha256 = json_str(
        &source,
        "dry_run_execution_result_receipt_future_receipt_denial_hash_sha256",
    );
    let source_rollback_sequence_denial_hash_sha256 = json_str(
        &source,
        "dry_run_execution_result_receipt_rollback_sequence_denial_hash_sha256",
    );
    let source_same_sequence_replacement_denial_hash_sha256 = json_str(
        &source,
        "dry_run_execution_result_receipt_same_sequence_replacement_denial_hash_sha256",
    );
    let source_latest_wins_promotion_denial_hash_sha256 = json_str(
        &source,
        "dry_run_execution_result_receipt_latest_wins_promotion_denial_hash_sha256",
    );
    let source_sequence_gap_denial_hash_sha256 = json_str(
        &source,
        "dry_run_execution_result_receipt_sequence_gap_denial_hash_sha256",
    );
    let source_ordering_handoff_hash_sha256 = json_str(
        &source,
        "dry_run_execution_result_receipt_ordering_handoff_hash_sha256",
    );
    let source_ordering_result_hash_sha256 = json_str(
        &source,
        "dry_run_execution_result_receipt_ordering_result_hash_sha256",
    );
    let cancellation_matrix_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-cancellation-supersession-denial-matrix:v1:source-ordering={source_ordering_result_hash_sha256}:late={source_late_receipt_denial_hash_sha256}:future={source_future_receipt_denial_hash_sha256}:rollback={source_rollback_sequence_denial_hash_sha256}:same={source_same_sequence_replacement_denial_hash_sha256}:latest={source_latest_wins_promotion_denial_hash_sha256}:gap={source_sequence_gap_denial_hash_sha256}:cancel=deny:supersede=deny:replacement=deny:tombstone=deny:persist=false"
    ));
    let cancellation_policy_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-cancellation-policy:v1:matrix={cancellation_matrix_hash_sha256}:source-policy={source_ordering_policy_hash_sha256}:record=false:persist=false:ledger=false"
    ));
    let supersession_policy_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-supersession-policy:v1:matrix={cancellation_matrix_hash_sha256}:source-sequence-policy={source_ordering_sequence_policy_hash_sha256}:supersede=false:replace=false:tombstone=false"
    ));
    let replacement_receipt_denial_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-replacement-denial:v1:policy={supersession_policy_hash_sha256}:accepted=false"
    ));
    let tombstone_delete_marker_denial_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-tombstone-delete-marker-denial:v1:policy={supersession_policy_hash_sha256}:accepted=false"
    ));
    let latest_replacement_denial_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-latest-replacement-denial:v1:policy={supersession_policy_hash_sha256}:accepted=false"
    ));
    let completion_ack_replacement_denial_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-completion-ack-replacement-denial:v1:policy={supersession_policy_hash_sha256}:accepted=false"
    ));
    let export_query_replacement_denial_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-export-query-replacement-denial:v1:policy={supersession_policy_hash_sha256}:accepted=false"
    ));
    let cancellation_handoff_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-cancellation-supersession-handoff:v1:replacement={replacement_receipt_denial_hash_sha256}:tombstone={tombstone_delete_marker_denial_hash_sha256}:latest={latest_replacement_denial_hash_sha256}:completion={completion_ack_replacement_denial_hash_sha256}:export={export_query_replacement_denial_hash_sha256}:next=audit-trail-immutable-evidence-denial-boundary"
    ));
    let cancellation_result_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-cancellation-supersession-result:v1:matrix={cancellation_matrix_hash_sha256}:handoff={cancellation_handoff_hash_sha256}:accepted=true:cancel=false:supersede=false:replace=false:tombstone=false:executed=false:production-write=false"
    ));
    let cancellation_boundary_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-cancellation-supersession-denial-boundary:v1:source={source_report_sha256}:result={cancellation_result_hash_sha256}:fixtures=10:accepted=1:denials={}:cancel=false:supersede=false:replacement=false:tombstone=false:dry-run-executed=false:production-write=false",
        CANCELLATION_DENIALS.len()
    ));
    let cancellation_boundary_policy_hash_sha256 = sha256_text_value(
        "scoped-production-durable-memory-write-dry-run-result-receipt-cancellation-supersession-denial-policy:v1:bind-source-ordering-matrix-cancel-supersede-replacement-tombstone-latest-completion-export-handoff:no-cancel:no-supersede:no-replacement:no-tombstone:no-execution:no-production-write:no-kg:no-provider:no-channel:no-release:no-install",
    );
    let cancellation_bound = !source_ordering_boundary_hash_sha256.is_empty()
        && !source_ordering_policy_hash_sha256.is_empty()
        && !source_ordering_matrix_hash_sha256.is_empty()
        && !source_ordering_sequence_policy_hash_sha256.is_empty()
        && !source_late_receipt_denial_hash_sha256.is_empty()
        && !source_future_receipt_denial_hash_sha256.is_empty()
        && !source_rollback_sequence_denial_hash_sha256.is_empty()
        && !source_same_sequence_replacement_denial_hash_sha256.is_empty()
        && !source_latest_wins_promotion_denial_hash_sha256.is_empty()
        && !source_sequence_gap_denial_hash_sha256.is_empty()
        && !source_ordering_handoff_hash_sha256.is_empty()
        && !source_ordering_result_hash_sha256.is_empty()
        && approved_production_namespace == "hepta.memory.production.scoped"
        && approved_production_store == "hepta-memory-durable-store-production-preflight-only"
        && approved_production_scope == "operator-approved-session"
        && production_durable_memory_target_id
            == "hepta-scoped-production-durable-memory-write-target-v1"
        && production_durable_memory_payload_class
            == "redacted-minimal-operator-approved-memory-fact";
    let surfaces_ready = source_ready && cancellation_bound;
    let report_ready = route_count_source_command_accepted && surfaces_ready;
    let accepted_fixture_count = if report_ready { 1 } else { 0 };
    let blocked_fixture_count = 10 - accepted_fixture_count;

    let mut fixtures = Vec::new();
    fixtures.push(serde_json::json!({
        "id": "scoped-production-durable-memory-write-dry-run-execution-result-receipt-cancellation-supersession-denial",
        "fixture_id": "scoped-production-durable-memory-write-dry-run-execution-result-receipt-cancellation-supersession-denial",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_accepted": report_ready,
        "reason": if report_ready { "dry_run_execution_result_receipt_cancellation_supersession_denial_bound_without_cancellation_supersession_replacement_tombstone_execution_or_production_write" } else { "source_ordering_monotonicity_or_route_count_not_ready" },
        "source_ordering_monotonicity_denial_boundary_bound": report_ready,
        "dry_run_execution_result_receipt_cancellation_supersession_denial_matrix_bound": report_ready,
        "dry_run_execution_result_receipt_cancellation_request_denied": report_ready,
        "dry_run_execution_result_receipt_supersession_request_denied": report_ready,
        "dry_run_execution_result_receipt_replacement_receipt_denied": report_ready,
        "dry_run_execution_result_receipt_tombstone_delete_marker_denied": report_ready,
        "dry_run_execution_result_receipt_latest_replacement_denied": report_ready,
        "dry_run_execution_result_receipt_completion_ack_replacement_denied": report_ready,
        "dry_run_execution_result_receipt_export_query_replacement_denied": report_ready,
        "dry_run_execution_result_receipt_cancellation_recorded": false,
        "dry_run_execution_result_receipt_supersession_recorded": false,
        "dry_run_execution_result_receipt_replacement_receipt_persisted": false,
        "dry_run_execution_executed": false,
        "production_durable_memory_store_write_performed": false,
        "external_send_performed": false
    }));
    for id in [
        "missing-ordering-monotonicity-source",
        "missing-ordering-result-hash",
        "missing-cancellation-supersession-matrix",
        "cancellation-request-attempt",
        "supersession-request-attempt",
        "replacement-receipt-attempt",
        "tombstone-delete-marker-attempt",
        "latest-replacement-promotion-attempt",
        "completion-ack-or-export-query-replacement-attempt",
    ] {
        fixtures.push(serde_json::json!({
            "id": id,
            "fixture_id": id,
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_accepted": false,
            "reason": "blocked_noop",
            "dry_run_execution_result_receipt_cancellation_recorded": false,
            "dry_run_execution_result_receipt_supersession_recorded": false,
            "dry_run_execution_result_receipt_replacement_receipt_persisted": false,
            "dry_run_execution_executed": false,
            "production_durable_memory_store_write_performed": false,
            "external_send_performed": false
        }));
    }

    let mut side_effects = serde_json::Map::new();
    for &key in FALSE_CANCELLATION_SIDE_EFFECT_KEYS {
        side_effects.insert(key.to_string(), serde_json::json!(false));
        side_effects.insert(format!("{key}_count"), serde_json::json!(0));
    }
    for &key in TRUE_CANCELLATION_KEYS {
        side_effects.insert(key.to_string(), serde_json::json!(report_ready));
        side_effects.insert(
            format!("{key}_count"),
            serde_json::json!(if report_ready { 1 } else { 0 }),
        );
    }

    let mut report = source.clone();
    extend_json_object(
        &mut report,
        serde_json::json!({
            "gate": "hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary_route",
            "status": if report_ready { "ready" } else { "blocked" },
            "endpoint": HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_PRODUCTION_DURABLE_MEMORY_WRITE_DRY_RUN_EXECUTION_RESULT_RECEIPT_CANCELLATION_SUPERSESSION_DENIAL_BOUNDARY_ENDPOINT,
            "source_command": "/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-cancellation-supersession-denial-boundary --json",
            "native_route": true,
            "compatibility_mode": "native_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary_status",
            "side_effect_free": false,
            "external_side_effect_free": true,
            "audit_date": "2026-07-05",
            "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
            "route_count": route_matrix.route_count,
            "implemented_route_count": route_matrix.implemented_route_count,
            "missing_route_count": route_matrix.missing_route_count,
            "route_count_source_command_accepted": route_count_source_command_accepted,
            "memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary_ready": report_ready,
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_ready": report_ready,
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_performed": report_ready,
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_accepted": report_ready,
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_mode": "dry_run_execution_result_receipt_cancellation_supersession_denial_boundary_no_cancel_no_supersede_no_replacement_no_tombstone_no_execution_no_production_durable_memory_mutation",
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_ready": source_ready,
            "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_report_sha256": source_report_sha256,
            "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_accepted_count": json_u64(&source, "scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_result_accepted_count"),
            "source_accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_fixture_count": json_u64(&source, "accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_fixture_count"),
            "source_blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_fixture_count": json_u64(&source, "blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_fixture_count"),
            "source_denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_count": json_u64(&source, "denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_count"),
            "approved_production_namespace": approved_production_namespace,
            "approved_production_store": approved_production_store,
            "approved_production_scope": approved_production_scope,
            "production_durable_memory_target_id": production_durable_memory_target_id,
            "production_durable_memory_payload_class": production_durable_memory_payload_class,
            "operator_packet_scope": operator_packet_scope,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_hash_sha256": source_ordering_boundary_hash_sha256,
            "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_policy_hash_sha256": source_ordering_policy_hash_sha256,
            "source_dry_run_execution_result_receipt_ordering_monotonicity_denial_matrix_hash_sha256": source_ordering_matrix_hash_sha256,
            "source_dry_run_execution_result_receipt_ordering_sequence_policy_hash_sha256": source_ordering_sequence_policy_hash_sha256,
            "source_dry_run_execution_result_receipt_late_receipt_denial_hash_sha256": source_late_receipt_denial_hash_sha256,
            "source_dry_run_execution_result_receipt_future_receipt_denial_hash_sha256": source_future_receipt_denial_hash_sha256,
            "source_dry_run_execution_result_receipt_rollback_sequence_denial_hash_sha256": source_rollback_sequence_denial_hash_sha256,
            "source_dry_run_execution_result_receipt_same_sequence_replacement_denial_hash_sha256": source_same_sequence_replacement_denial_hash_sha256,
            "source_dry_run_execution_result_receipt_latest_wins_promotion_denial_hash_sha256": source_latest_wins_promotion_denial_hash_sha256,
            "source_dry_run_execution_result_receipt_sequence_gap_denial_hash_sha256": source_sequence_gap_denial_hash_sha256,
            "source_dry_run_execution_result_receipt_ordering_handoff_hash_sha256": source_ordering_handoff_hash_sha256,
            "source_dry_run_execution_result_receipt_ordering_result_hash_sha256": source_ordering_result_hash_sha256,
            "dry_run_execution_result_receipt_cancellation_supersession_denial_matrix_hash_sha256": cancellation_matrix_hash_sha256,
            "dry_run_execution_result_receipt_cancellation_policy_hash_sha256": cancellation_policy_hash_sha256,
            "dry_run_execution_result_receipt_supersession_policy_hash_sha256": supersession_policy_hash_sha256,
            "dry_run_execution_result_receipt_replacement_receipt_denial_hash_sha256": replacement_receipt_denial_hash_sha256,
            "dry_run_execution_result_receipt_tombstone_delete_marker_denial_hash_sha256": tombstone_delete_marker_denial_hash_sha256,
            "dry_run_execution_result_receipt_latest_replacement_denial_hash_sha256": latest_replacement_denial_hash_sha256,
            "dry_run_execution_result_receipt_completion_ack_replacement_denial_hash_sha256": completion_ack_replacement_denial_hash_sha256,
            "dry_run_execution_result_receipt_export_query_replacement_denial_hash_sha256": export_query_replacement_denial_hash_sha256,
            "dry_run_execution_result_receipt_cancellation_supersession_handoff_hash_sha256": cancellation_handoff_hash_sha256,
            "dry_run_execution_result_receipt_cancellation_supersession_result_hash_sha256": cancellation_result_hash_sha256,
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary_hash_sha256": cancellation_boundary_hash_sha256,
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_policy_hash_sha256": cancellation_boundary_policy_hash_sha256,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "required_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_surface_count": CANCELLATION_SURFACES.len(),
            "ready_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_surface_count": if surfaces_ready { CANCELLATION_SURFACES.len() } else { 0 },
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_surfaces": CANCELLATION_SURFACES,
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_fixture_count": fixtures.len(),
            "accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_fixture_count": accepted_fixture_count,
            "blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_fixture_count": blocked_fixture_count,
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_fixtures": fixtures,
            "denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary": CANCELLATION_DENIALS,
            "denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary_count": CANCELLATION_DENIALS.len(),
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [
                {
                    "action": "run_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary_require_live_gate",
                    "status": "allowed_verification_only",
                    "accepts_cancellation_supersession_denial_matrix": true,
                    "records_cancellation": false,
                    "persists_cancellation": false,
                    "records_supersession": false,
                    "persists_supersession": false,
                    "accepts_replacement_receipt": false,
                    "writes_tombstone_delete_marker": false,
                    "executes_dry_run": false,
                    "persists_dry_run_result_receipt": false,
                    "writes_production_durable_memory": false,
                    "writes_memory_store": false,
                    "writes_wal": false,
                    "persists_receipt": false
                },
                {
                    "action": "prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary",
                    "status": "requires_separate_result_receipt_audit_trail_immutable_evidence_denial_gate",
                    "requires_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary": true,
                    "records_cancellation": false,
                    "persists_cancellation": false,
                    "records_supersession": false,
                    "persists_supersession": false,
                    "executes_dry_run": false,
                    "writes_production_durable_memory": false,
                    "persists_dry_run_result_receipt": false
                }
            ]
        }),
    );
    let report_object = report.as_object_mut().expect(
        "scoped production durable Memory write dry-run execution result receipt cancellation/supersession denial report object",
    );
    for &key in FALSE_CANCELLATION_SIDE_EFFECT_KEYS {
        report_object.insert(key.to_string(), serde_json::json!(false));
        report_object.insert(format!("{key}_count"), serde_json::json!(0));
    }
    for &key in TRUE_CANCELLATION_KEYS {
        report_object.insert(key.to_string(), serde_json::json!(report_ready));
        report_object.insert(
            format!("{key}_count"),
            serde_json::json!(if report_ready { 1 } else { 0 }),
        );
    }
    for key in [
        "source_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_bound",
        "approved_production_namespace_bound",
        "approved_production_store_bound",
        "approved_production_scope_bound",
        "production_durable_memory_target_bound",
        "dry_run_execution_result_receipt_cancellation_supersession_denial_matrix_bound",
        "dry_run_execution_result_receipt_cancellation_policy_bound",
        "dry_run_execution_result_receipt_supersession_policy_bound",
        "dry_run_execution_result_receipt_cancellation_request_denied",
        "dry_run_execution_result_receipt_supersession_request_denied",
        "dry_run_execution_result_receipt_replacement_receipt_denied",
        "dry_run_execution_result_receipt_tombstone_delete_marker_denied",
        "dry_run_execution_result_receipt_latest_replacement_denied",
        "dry_run_execution_result_receipt_completion_ack_replacement_denied",
        "dry_run_execution_result_receipt_export_query_replacement_denied",
        "dry_run_execution_result_receipt_cancellation_supersession_handoff_bound",
        "dry_run_execution_result_receipt_cancellation_supersession_state_persistence_forbidden",
        "dry_run_execution_result_receipt_replacement_receipt_persistence_forbidden",
        "dry_run_execution_execution_forbidden_on_cancellation_supersession_route",
        "dry_run_execution_result_receipt_persistence_forbidden_on_cancellation_supersession_route",
        "production_write_execution_forbidden_on_cancellation_supersession_route",
        "production_durable_memory_write_forbidden",
        "memory_store_mutation_forbidden",
        "wal_write_forbidden_on_cancellation_supersession_route",
        "receipt_persist_forbidden_on_cancellation_supersession_route",
        "rollback_execution_forbidden_on_cancellation_supersession_route",
        "tombstone_write_forbidden_on_cancellation_supersession_route",
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

fn hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary_report()
-> serde_json::Value {
    const AUDIT_SURFACES: &[&str] = &[
        "source_cancellation_supersession_denial_boundary_required",
        "source_cancellation_supersession_result_required",
        "dry_run_execution_result_receipt_audit_trail_request_denied",
        "dry_run_execution_result_receipt_immutable_evidence_request_denied",
        "dry_run_execution_result_receipt_hash_chain_recording_denied",
        "dry_run_execution_result_receipt_merkle_root_recording_denied",
        "dry_run_execution_result_receipt_attestation_witness_notary_denied",
        "dry_run_execution_result_receipt_audit_materialization_denied",
        "dry_run_execution_result_receipt_immutable_evidence_persistence_denied",
        "dry_run_execution_result_receipt_ledger_index_delivery_evidence_denied",
        "dry_run_execution_result_receipt_authority_promotion_from_audit_evidence_denied",
        "dry_run_execution_result_receipt_memory_kg_provider_channel_evidence_denied",
        "dry_run_execution_result_receipt_release_install_active_binary_evidence_denied",
        "dry_run_execution_execution_forbidden_on_audit_evidence_route",
        "production_write_execution_forbidden_on_audit_evidence_route",
        "kg_provider_channel_release_install_active_binary_forbidden_on_audit_evidence_route",
    ];
    const AUDIT_DENIALS: &[&str] = &[
        "source_cancellation_supersession_denial_boundary_required",
        "source_cancellation_supersession_result_hash_required",
        "source_cancellation_supersession_policy_hash_required",
        "source_cancellation_supersession_matrix_required",
        "source_replacement_receipt_denial_required",
        "source_tombstone_delete_marker_denial_required",
        "approved_production_namespace_required",
        "approved_production_store_required",
        "approved_production_scope_required",
        "production_durable_memory_target_required",
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
        "audit_evidence_persistence_denied",
        "hash_chain_recording_denied",
        "hash_chain_persistence_denied",
        "merkle_root_recording_denied",
        "merkle_root_persistence_denied",
        "attestation_recording_denied",
        "attestation_persistence_denied",
        "witness_recording_denied",
        "witness_persistence_denied",
        "notary_recording_denied",
        "notary_persistence_denied",
        "ledger_evidence_recording_denied",
        "ledger_evidence_persistence_denied",
        "index_evidence_recording_denied",
        "delivery_evidence_recording_denied",
        "export_evidence_recording_denied",
        "query_evidence_recording_denied",
        "readback_evidence_recording_denied",
        "audit_evidence_authority_derivation_denied",
        "immutable_evidence_authority_derivation_denied",
        "result_receipt_authority_promotion_from_audit_denied",
        "result_receipt_authority_promotion_from_immutable_evidence_denied",
        "cancellation_supersession_authority_from_audit_denied",
        "replacement_receipt_authority_from_evidence_denied",
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
        "credential_secret_read_denied",
        "channel_external_send_denied",
        "release_public_artifact_write_denied",
        "install_restart_authority_denied",
        "active_binary_mutation_denied",
        "unrestricted_full_live_activation_denied",
    ];
    const FALSE_AUDIT_SIDE_EFFECT_KEYS: &[&str] = &[
        "dry_run_execution_result_receipt_audit_trail_accepted",
        "dry_run_execution_result_receipt_audit_trail_recorded",
        "dry_run_execution_result_receipt_audit_trail_persisted",
        "dry_run_execution_result_receipt_audit_trail_materialized",
        "dry_run_execution_result_receipt_audit_trail_filesystem_written",
        "dry_run_execution_result_receipt_immutable_evidence_accepted",
        "dry_run_execution_result_receipt_immutable_evidence_recorded",
        "dry_run_execution_result_receipt_immutable_evidence_persisted",
        "dry_run_execution_result_receipt_immutable_evidence_materialized",
        "dry_run_execution_result_receipt_immutable_evidence_filesystem_written",
        "dry_run_execution_result_receipt_hash_chain_recorded",
        "dry_run_execution_result_receipt_hash_chain_persisted",
        "dry_run_execution_result_receipt_merkle_root_recorded",
        "dry_run_execution_result_receipt_merkle_root_persisted",
        "dry_run_execution_result_receipt_attestation_recorded",
        "dry_run_execution_result_receipt_attestation_persisted",
        "dry_run_execution_result_receipt_witness_recorded",
        "dry_run_execution_result_receipt_witness_persisted",
        "dry_run_execution_result_receipt_notary_recorded",
        "dry_run_execution_result_receipt_notary_persisted",
        "dry_run_execution_result_receipt_ledger_evidence_recorded",
        "dry_run_execution_result_receipt_ledger_evidence_persisted",
        "dry_run_execution_result_receipt_index_evidence_recorded",
        "dry_run_execution_result_receipt_delivery_evidence_recorded",
        "dry_run_execution_result_receipt_export_evidence_recorded",
        "dry_run_execution_result_receipt_query_evidence_recorded",
        "dry_run_execution_result_receipt_readback_evidence_recorded",
        "dry_run_execution_result_receipt_authority_promoted_from_audit_trail",
        "dry_run_execution_result_receipt_authority_promoted_from_immutable_evidence",
        "dry_run_execution_result_receipt_cancellation_request_accepted",
        "dry_run_execution_result_receipt_cancellation_recorded",
        "dry_run_execution_result_receipt_cancellation_persisted",
        "dry_run_execution_result_receipt_supersession_request_accepted",
        "dry_run_execution_result_receipt_supersession_recorded",
        "dry_run_execution_result_receipt_supersession_persisted",
        "dry_run_execution_result_receipt_replacement_receipt_accepted",
        "dry_run_execution_result_receipt_replacement_receipt_recorded",
        "dry_run_execution_result_receipt_replacement_receipt_persisted",
        "dry_run_execution_result_receipt_tombstone_delete_marker_accepted",
        "dry_run_execution_result_receipt_tombstone_delete_marker_written",
        "dry_run_execution_result_receipt_ordering_cursor_persisted",
        "dry_run_execution_result_receipt_monotonic_sequence_recorded",
        "dry_run_execution_result_receipt_replay_state_persisted",
        "dry_run_execution_result_receipt_idempotency_ledger_written",
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
    const TRUE_AUDIT_KEYS: &[&str] = &[
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary_performed",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_result_recorded",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_result_accepted",
        "source_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary_accepted",
        "dry_run_execution_result_receipt_audit_trail_denial_matrix_bound",
        "dry_run_execution_result_receipt_immutable_evidence_denial_matrix_bound",
        "dry_run_execution_result_receipt_audit_trail_request_denied",
        "dry_run_execution_result_receipt_immutable_evidence_request_denied",
        "dry_run_execution_result_receipt_hash_chain_denied",
        "dry_run_execution_result_receipt_merkle_root_denied",
        "dry_run_execution_result_receipt_attestation_denied",
        "dry_run_execution_result_receipt_witness_denied",
        "dry_run_execution_result_receipt_notary_denied",
        "dry_run_execution_result_receipt_ledger_index_delivery_evidence_denied",
        "dry_run_execution_result_receipt_audit_evidence_authority_denied",
        "dry_run_execution_result_receipt_audit_evidence_handoff_bound",
        "dry_run_execution_result_receipt_audit_evidence_persistence_forbidden",
        "dry_run_execution_execution_forbidden_on_audit_evidence_route",
        "production_write_execution_forbidden_on_audit_evidence_route",
        "kg_provider_channel_release_install_active_binary_forbidden_on_audit_evidence_route",
    ];

    let route_matrix = control_ui_route_parity_report();
    let source = std::thread::Builder::new()
        .name("hepta-memory-production-durable-dry-run-result-receipt-audit-source-report".to_string())
        .stack_size(8 * 1024 * 1024)
        .spawn(
            hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary_report,
        )
        .ok()
        .and_then(|handle| handle.join().ok())
        .unwrap_or_else(|| {
            serde_json::json!({
                "status": "blocked",
                "memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary_ready": false,
                "scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_accepted": false,
                "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_source_report_thread_failed": true
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
    let source_next_action_audit = source
        .get("allowed_next_actions")
        .and_then(serde_json::Value::as_array)
        .and_then(|items| items.get(1))
        .map(|item| {
            item.get("action").and_then(serde_json::Value::as_str)
                == Some("prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary")
                && item
                    .get("requires_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary")
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
                .get("scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_result_accepted")
                .and_then(serde_json::Value::as_bool)
                == Some(true)
                && effects
                    .get("dry_run_execution_result_receipt_cancellation_request_accepted")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("dry_run_execution_result_receipt_supersession_request_accepted")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("dry_run_execution_result_receipt_replacement_receipt_persisted")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("dry_run_execution_result_receipt_tombstone_delete_marker_written")
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
            "memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary_ready",
        )
        && json_bool(
            &source,
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_accepted",
        )
        && json_u64(
            &source,
            "accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_fixture_count",
        ) == 1
        && json_u64(
            &source,
            "blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_fixture_count",
        ) == 9
        && json_u64(
            &source,
            "denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary_count",
        ) == 65
        && !json_bool(
            &source,
            "dry_run_execution_result_receipt_cancellation_request_accepted",
        )
        && !json_bool(
            &source,
            "dry_run_execution_result_receipt_supersession_request_accepted",
        )
        && !json_bool(
            &source,
            "dry_run_execution_result_receipt_replacement_receipt_persisted",
        )
        && !json_bool(
            &source,
            "dry_run_execution_result_receipt_tombstone_delete_marker_written",
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
        && source_next_action_audit
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
    let source_cancellation_boundary_hash_sha256 = json_str(
        &source,
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary_hash_sha256",
    );
    let source_cancellation_policy_hash_sha256 = json_str(
        &source,
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_policy_hash_sha256",
    );
    let source_cancellation_matrix_hash_sha256 = json_str(
        &source,
        "dry_run_execution_result_receipt_cancellation_supersession_denial_matrix_hash_sha256",
    );
    let source_cancellation_result_hash_sha256 = json_str(
        &source,
        "dry_run_execution_result_receipt_cancellation_supersession_result_hash_sha256",
    );
    let source_cancellation_handoff_hash_sha256 = json_str(
        &source,
        "dry_run_execution_result_receipt_cancellation_supersession_handoff_hash_sha256",
    );
    let source_replacement_receipt_denial_hash_sha256 = json_str(
        &source,
        "dry_run_execution_result_receipt_replacement_receipt_denial_hash_sha256",
    );
    let source_tombstone_delete_marker_denial_hash_sha256 = json_str(
        &source,
        "dry_run_execution_result_receipt_tombstone_delete_marker_denial_hash_sha256",
    );

    let audit_trail_denial_matrix_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-audit-trail-denial-matrix:v1:source={source_cancellation_result_hash_sha256}:audit=deny:record=false:persist=false:materialize=false:filesystem=false"
    ));
    let immutable_evidence_denial_matrix_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-immutable-evidence-denial-matrix:v1:source={source_cancellation_result_hash_sha256}:immutable=deny:hash-chain=false:merkle=false:attestation=false:witness=false:notary=false:persist=false"
    ));
    let audit_hash_chain_denial_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-audit-hash-chain-denial:v1:audit={audit_trail_denial_matrix_hash_sha256}:immutable={immutable_evidence_denial_matrix_hash_sha256}:record=false"
    ));
    let audit_attestation_denial_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-audit-attestation-denial:v1:hash-chain={audit_hash_chain_denial_hash_sha256}:attestation=false:witness=false:notary=false"
    ));
    let audit_ledger_evidence_denial_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-audit-ledger-evidence-denial:v1:attestation={audit_attestation_denial_hash_sha256}:ledger=false:index=false:delivery=false:export=false:query=false:readback=false"
    ));
    let audit_evidence_handoff_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-audit-evidence-handoff:v1:ledger={audit_ledger_evidence_denial_hash_sha256}:next=retention-expiry-garbage-collection-denial-boundary"
    ));
    let audit_evidence_result_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-audit-evidence-result:v1:audit={audit_trail_denial_matrix_hash_sha256}:immutable={immutable_evidence_denial_matrix_hash_sha256}:handoff={audit_evidence_handoff_hash_sha256}:accepted=true:record=false:persist=false:authority=false:execution=false:production-write=false"
    ));
    let audit_evidence_boundary_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-audit-trail-immutable-evidence-denial-boundary:v1:source={source_report_sha256}:result={audit_evidence_result_hash_sha256}:fixtures=10:accepted=1:denials=72:audit=false:immutable=false:authority=false:dry-run-executed=false:production-write=false"
    ));
    let audit_evidence_boundary_policy_hash_sha256 = sha256_text_value(
        "scoped-production-durable-memory-write-dry-run-result-receipt-audit-trail-immutable-evidence-denial-policy:v1:bind-source-cancellation-supersession-no-audit-log-no-immutable-evidence-no-hash-chain-no-merkle-root-no-attestation-no-ledger-no-authority-no-execution-no-production-write-no-kg-no-provider-no-channel-no-release-no-install",
    );

    let audit_bound = !source_report_sha256.is_empty()
        && !source_cancellation_boundary_hash_sha256.is_empty()
        && !source_cancellation_policy_hash_sha256.is_empty()
        && !source_cancellation_matrix_hash_sha256.is_empty()
        && !source_cancellation_result_hash_sha256.is_empty()
        && !source_cancellation_handoff_hash_sha256.is_empty()
        && !source_replacement_receipt_denial_hash_sha256.is_empty()
        && !source_tombstone_delete_marker_denial_hash_sha256.is_empty()
        && approved_production_namespace == "hepta.memory.production.scoped"
        && approved_production_store == "hepta-memory-durable-store-production-preflight-only"
        && approved_production_scope == "operator-approved-session"
        && production_durable_memory_target_id
            == "hepta-scoped-production-durable-memory-write-target-v1"
        && production_durable_memory_payload_class
            == "redacted-minimal-operator-approved-memory-fact";
    let surfaces_ready = source_ready && audit_bound;
    let report_ready = route_count_source_command_accepted && surfaces_ready;
    let accepted_fixture_count = if report_ready { 1 } else { 0 };
    let blocked_fixture_count = 10 - accepted_fixture_count;

    let mut fixtures = Vec::new();
    fixtures.push(serde_json::json!({
        "id": "scoped-production-durable-memory-write-dry-run-execution-result-receipt-audit-trail-immutable-evidence-denial",
        "fixture_id": "scoped-production-durable-memory-write-dry-run-execution-result-receipt-audit-trail-immutable-evidence-denial",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_accepted": report_ready,
        "reason": if report_ready { "dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_bound_without_audit_evidence_persistence_authority_execution_or_production_write" } else { "source_cancellation_supersession_or_route_count_not_ready" },
        "source_cancellation_supersession_denial_boundary_bound": report_ready,
        "dry_run_execution_result_receipt_audit_trail_denied": report_ready,
        "dry_run_execution_result_receipt_immutable_evidence_denied": report_ready,
        "dry_run_execution_result_receipt_hash_chain_denied": report_ready,
        "dry_run_execution_result_receipt_merkle_root_denied": report_ready,
        "dry_run_execution_result_receipt_attestation_denied": report_ready,
        "dry_run_execution_result_receipt_ledger_index_delivery_evidence_denied": report_ready,
        "dry_run_execution_result_receipt_audit_trail_recorded": false,
        "dry_run_execution_result_receipt_immutable_evidence_persisted": false,
        "dry_run_execution_result_receipt_hash_chain_recorded": false,
        "dry_run_execution_result_receipt_authority_promoted_from_audit_trail": false,
        "dry_run_execution_executed": false,
        "production_durable_memory_store_write_performed": false,
        "external_send_performed": false
    }));
    for id in [
        "missing-cancellation-supersession-source",
        "missing-cancellation-result-hash",
        "audit-trail-append-request-attempt",
        "immutable-evidence-seal-request-attempt",
        "hash-chain-merkle-root-attempt",
        "attestation-witness-notary-attempt",
        "ledger-index-delivery-evidence-attempt",
        "memory-kg-provider-channel-evidence-attempt",
        "release-install-active-binary-evidence-attempt",
    ] {
        fixtures.push(serde_json::json!({
            "id": id,
            "fixture_id": id,
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_accepted": false,
            "reason": "blocked_noop",
            "dry_run_execution_result_receipt_audit_trail_recorded": false,
            "dry_run_execution_result_receipt_immutable_evidence_persisted": false,
            "dry_run_execution_result_receipt_hash_chain_recorded": false,
            "dry_run_execution_result_receipt_authority_promoted_from_audit_trail": false,
            "dry_run_execution_executed": false,
            "production_durable_memory_store_write_performed": false,
            "external_send_performed": false
        }));
    }

    let mut side_effects = serde_json::Map::new();
    for &key in FALSE_AUDIT_SIDE_EFFECT_KEYS {
        side_effects.insert(key.to_string(), serde_json::json!(false));
        side_effects.insert(format!("{key}_count"), serde_json::json!(0));
    }
    for &key in TRUE_AUDIT_KEYS {
        side_effects.insert(key.to_string(), serde_json::json!(report_ready));
        side_effects.insert(
            format!("{key}_count"),
            serde_json::json!(if report_ready { 1 } else { 0 }),
        );
    }

    let mut report = source.clone();
    extend_json_object(
        &mut report,
        serde_json::json!({
            "gate": "hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary_route",
            "status": if report_ready { "ready" } else { "blocked" },
            "endpoint": HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_PRODUCTION_DURABLE_MEMORY_WRITE_DRY_RUN_EXECUTION_RESULT_RECEIPT_AUDIT_TRAIL_IMMUTABLE_EVIDENCE_DENIAL_BOUNDARY_ENDPOINT,
            "source_command": "/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-audit-trail-immutable-evidence-denial-boundary --json",
            "native_route": true,
            "compatibility_mode": "native_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary_status",
            "side_effect_free": false,
            "external_side_effect_free": true,
            "audit_date": "2026-07-05",
            "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
            "route_count": route_matrix.route_count,
            "implemented_route_count": route_matrix.implemented_route_count,
            "missing_route_count": route_matrix.missing_route_count,
            "route_count_source_command_accepted": route_count_source_command_accepted,
            "memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary_ready": report_ready,
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_ready": report_ready,
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_performed": report_ready,
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_accepted": report_ready,
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_mode": "dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary_no_audit_no_immutable_evidence_no_hash_chain_no_attestation_no_authority_no_execution_no_production_durable_memory_mutation",
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary_ready": source_ready,
            "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary_report_sha256": source_report_sha256,
            "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary_accepted_count": json_u64(&source, "scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_result_accepted_count"),
            "source_accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_fixture_count": json_u64(&source, "accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_fixture_count"),
            "source_blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_fixture_count": json_u64(&source, "blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_fixture_count"),
            "source_denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary_count": json_u64(&source, "denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary_count"),
            "approved_production_namespace": approved_production_namespace,
            "approved_production_store": approved_production_store,
            "approved_production_scope": approved_production_scope,
            "production_durable_memory_target_id": production_durable_memory_target_id,
            "production_durable_memory_payload_class": production_durable_memory_payload_class,
            "operator_packet_scope": operator_packet_scope,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary_hash_sha256": source_cancellation_boundary_hash_sha256,
            "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_policy_hash_sha256": source_cancellation_policy_hash_sha256,
            "source_dry_run_execution_result_receipt_cancellation_supersession_denial_matrix_hash_sha256": source_cancellation_matrix_hash_sha256,
            "source_dry_run_execution_result_receipt_cancellation_supersession_result_hash_sha256": source_cancellation_result_hash_sha256,
            "source_dry_run_execution_result_receipt_cancellation_supersession_handoff_hash_sha256": source_cancellation_handoff_hash_sha256,
            "source_dry_run_execution_result_receipt_replacement_receipt_denial_hash_sha256": source_replacement_receipt_denial_hash_sha256,
            "source_dry_run_execution_result_receipt_tombstone_delete_marker_denial_hash_sha256": source_tombstone_delete_marker_denial_hash_sha256,
            "dry_run_execution_result_receipt_audit_trail_denial_matrix_hash_sha256": audit_trail_denial_matrix_hash_sha256,
            "dry_run_execution_result_receipt_immutable_evidence_denial_matrix_hash_sha256": immutable_evidence_denial_matrix_hash_sha256,
            "dry_run_execution_result_receipt_audit_hash_chain_denial_hash_sha256": audit_hash_chain_denial_hash_sha256,
            "dry_run_execution_result_receipt_audit_attestation_denial_hash_sha256": audit_attestation_denial_hash_sha256,
            "dry_run_execution_result_receipt_audit_ledger_evidence_denial_hash_sha256": audit_ledger_evidence_denial_hash_sha256,
            "dry_run_execution_result_receipt_audit_evidence_handoff_hash_sha256": audit_evidence_handoff_hash_sha256,
            "dry_run_execution_result_receipt_audit_evidence_result_hash_sha256": audit_evidence_result_hash_sha256,
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary_hash_sha256": audit_evidence_boundary_hash_sha256,
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_policy_hash_sha256": audit_evidence_boundary_policy_hash_sha256,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "required_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_surface_count": AUDIT_SURFACES.len(),
            "ready_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_surface_count": if surfaces_ready { AUDIT_SURFACES.len() } else { 0 },
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_surfaces": AUDIT_SURFACES,
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_fixture_count": fixtures.len(),
            "accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_fixture_count": accepted_fixture_count,
            "blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_fixture_count": blocked_fixture_count,
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_fixtures": fixtures,
            "denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary": AUDIT_DENIALS,
            "denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary_count": AUDIT_DENIALS.len(),
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [
                {
                    "action": "run_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary_require_live_gate",
                    "status": "allowed_verification_only",
                    "accepts_audit_trail_immutable_evidence_denial_matrix": true,
                    "records_audit_trail": false,
                    "records_immutable_evidence": false,
                    "persists_evidence": false,
                    "records_hash_chain": false,
                    "records_attestation": false,
                    "promotes_authority": false,
                    "executes_dry_run": false,
                    "persists_dry_run_result_receipt": false,
                    "writes_production_durable_memory": false,
                    "writes_memory_store": false,
                    "writes_wal": false,
                    "persists_receipt": false
                },
                {
                    "action": "prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary",
                    "status": "requires_separate_result_receipt_retention_expiry_garbage_collection_denial_gate",
                    "requires_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary": true,
                    "records_audit_trail": false,
                    "records_immutable_evidence": false,
                    "persists_evidence": false,
                    "executes_dry_run": false,
                    "writes_production_durable_memory": false,
                    "persists_dry_run_result_receipt": false
                }
            ]
        }),
    );
    let report_object = report.as_object_mut().expect(
        "scoped production durable Memory write dry-run execution result receipt audit trail/immutable evidence denial report object",
    );
    for &key in FALSE_AUDIT_SIDE_EFFECT_KEYS {
        report_object.insert(key.to_string(), serde_json::json!(false));
        report_object.insert(format!("{key}_count"), serde_json::json!(0));
    }
    for &key in TRUE_AUDIT_KEYS {
        report_object.insert(key.to_string(), serde_json::json!(report_ready));
        report_object.insert(
            format!("{key}_count"),
            serde_json::json!(if report_ready { 1 } else { 0 }),
        );
    }
    for key in [
        "source_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary_bound",
        "approved_production_namespace_bound",
        "approved_production_store_bound",
        "approved_production_scope_bound",
        "production_durable_memory_target_bound",
        "dry_run_execution_result_receipt_audit_trail_denial_matrix_bound",
        "dry_run_execution_result_receipt_immutable_evidence_denial_matrix_bound",
        "dry_run_execution_result_receipt_audit_trail_request_denied",
        "dry_run_execution_result_receipt_immutable_evidence_request_denied",
        "dry_run_execution_result_receipt_hash_chain_denied",
        "dry_run_execution_result_receipt_merkle_root_denied",
        "dry_run_execution_result_receipt_attestation_denied",
        "dry_run_execution_result_receipt_witness_denied",
        "dry_run_execution_result_receipt_notary_denied",
        "dry_run_execution_result_receipt_ledger_index_delivery_evidence_denied",
        "dry_run_execution_result_receipt_audit_evidence_authority_denied",
        "dry_run_execution_result_receipt_audit_evidence_handoff_bound",
        "dry_run_execution_result_receipt_audit_evidence_persistence_forbidden",
        "dry_run_execution_execution_forbidden_on_audit_evidence_route",
        "dry_run_execution_result_receipt_persistence_forbidden_on_audit_evidence_route",
        "production_write_execution_forbidden_on_audit_evidence_route",
        "production_durable_memory_write_forbidden",
        "memory_store_mutation_forbidden",
        "wal_write_forbidden_on_audit_evidence_route",
        "receipt_persist_forbidden_on_audit_evidence_route",
        "rollback_execution_forbidden_on_audit_evidence_route",
        "tombstone_write_forbidden_on_audit_evidence_route",
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

fn hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary_report()
-> serde_json::Value {
    const RETENTION_SURFACES: &[&str] = &[
        "source_audit_trail_immutable_evidence_denial_boundary_required",
        "source_audit_evidence_result_required",
        "dry_run_execution_result_receipt_retention_policy_request_denied",
        "dry_run_execution_result_receipt_retention_index_denied",
        "dry_run_execution_result_receipt_ttl_lease_update_extension_denied",
        "dry_run_execution_result_receipt_expiry_timestamp_scheduler_timer_ack_denied",
        "dry_run_execution_result_receipt_expiry_state_persistence_denied",
        "dry_run_execution_result_receipt_garbage_collection_queue_scan_candidate_denied",
        "dry_run_execution_result_receipt_garbage_collection_decision_state_denied",
        "dry_run_execution_result_receipt_delete_tombstone_sweep_denied",
        "dry_run_execution_result_receipt_archive_compaction_denied",
        "dry_run_execution_result_receipt_audit_immutable_hash_attestation_retention_denied",
        "dry_run_execution_result_receipt_ledger_index_delivery_retention_denied",
        "dry_run_execution_result_receipt_memory_kg_provider_channel_retention_denied",
        "dry_run_execution_result_receipt_release_install_active_binary_gc_denied",
        "dry_run_execution_production_write_and_authority_forbidden_on_retention_gc_route",
    ];
    const RETENTION_DENIALS: &[&str] = &[
        "source_audit_trail_immutable_evidence_denial_boundary_required",
        "source_audit_evidence_result_hash_required",
        "source_audit_evidence_policy_hash_required",
        "source_audit_evidence_handoff_hash_required",
        "approved_production_namespace_required",
        "approved_production_store_required",
        "approved_production_scope_required",
        "production_durable_memory_target_required",
        "retention_policy_request_acceptance_denied",
        "retention_policy_recording_denied",
        "retention_policy_persistence_denied",
        "retention_policy_materialization_denied",
        "retention_policy_filesystem_write_denied",
        "retention_index_recording_denied",
        "retention_index_persistence_denied",
        "ttl_lease_recording_denied",
        "ttl_lease_persistence_denied",
        "ttl_update_denied",
        "ttl_extension_denied",
        "expiry_timestamp_recording_denied",
        "expiry_scheduler_registration_denied",
        "expiry_timer_start_denied",
        "expiry_ack_recording_denied",
        "expiry_state_persistence_denied",
        "garbage_collection_queue_recording_denied",
        "garbage_collection_queue_persistence_denied",
        "garbage_collection_scan_denied",
        "garbage_collection_candidate_recording_denied",
        "garbage_collection_decision_recording_denied",
        "garbage_collection_state_persistence_denied",
        "delete_marker_gc_denied",
        "tombstone_gc_denied",
        "sweep_execution_denied",
        "archive_write_denied",
        "compaction_execution_denied",
        "compaction_artifact_write_denied",
        "audit_evidence_retention_recording_denied",
        "immutable_evidence_retention_persistence_denied",
        "hash_attestation_retention_denied",
        "ledger_index_delivery_retention_denied",
        "result_receipt_retention_authority_promotion_denied",
        "retention_expiry_gc_authority_promotion_denied",
        "dry_run_execution_execution_denied",
        "dry_run_execution_result_receipt_persistence_denied",
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
        "credential_secret_read_denied",
        "channel_external_send_denied",
        "release_public_artifact_write_denied",
        "install_restart_authority_denied",
        "active_binary_mutation_denied",
        "unrestricted_full_live_activation_denied",
    ];
    const FALSE_RETENTION_SIDE_EFFECT_KEYS: &[&str] = &[
        "dry_run_execution_result_receipt_retention_policy_accepted",
        "dry_run_execution_result_receipt_retention_policy_recorded",
        "dry_run_execution_result_receipt_retention_policy_persisted",
        "dry_run_execution_result_receipt_retention_policy_materialized",
        "dry_run_execution_result_receipt_retention_policy_filesystem_written",
        "dry_run_execution_result_receipt_retention_index_recorded",
        "dry_run_execution_result_receipt_retention_index_persisted",
        "dry_run_execution_result_receipt_ttl_lease_recorded",
        "dry_run_execution_result_receipt_ttl_lease_persisted",
        "dry_run_execution_result_receipt_ttl_update_applied",
        "dry_run_execution_result_receipt_ttl_extension_applied",
        "dry_run_execution_result_receipt_expiry_timestamp_recorded",
        "dry_run_execution_result_receipt_expiry_scheduler_registered",
        "dry_run_execution_result_receipt_expiry_timer_started",
        "dry_run_execution_result_receipt_expiry_ack_recorded",
        "dry_run_execution_result_receipt_expiry_state_persisted",
        "dry_run_execution_result_receipt_garbage_collection_queue_recorded",
        "dry_run_execution_result_receipt_garbage_collection_queue_persisted",
        "dry_run_execution_result_receipt_garbage_collection_scan_performed",
        "dry_run_execution_result_receipt_garbage_collection_candidate_recorded",
        "dry_run_execution_result_receipt_garbage_collection_decision_recorded",
        "dry_run_execution_result_receipt_garbage_collection_state_persisted",
        "dry_run_execution_result_receipt_delete_marker_garbage_collected",
        "dry_run_execution_result_receipt_tombstone_garbage_collected",
        "dry_run_execution_result_receipt_sweep_performed",
        "dry_run_execution_result_receipt_archive_written",
        "dry_run_execution_result_receipt_compaction_performed",
        "dry_run_execution_result_receipt_compaction_artifact_written",
        "dry_run_execution_result_receipt_audit_evidence_retention_recorded",
        "dry_run_execution_result_receipt_immutable_evidence_retention_persisted",
        "dry_run_execution_result_receipt_hash_attestation_retention_recorded",
        "dry_run_execution_result_receipt_ledger_index_delivery_retention_recorded",
        "dry_run_execution_result_receipt_authority_promoted_from_retention_policy",
        "dry_run_execution_result_receipt_authority_promoted_from_expiry",
        "dry_run_execution_result_receipt_authority_promoted_from_garbage_collection",
        "dry_run_execution_result_receipt_audit_trail_recorded",
        "dry_run_execution_result_receipt_immutable_evidence_persisted",
        "dry_run_execution_result_receipt_hash_chain_recorded",
        "dry_run_execution_result_receipt_persisted",
        "dry_run_execution_result_receipt_filesystem_written",
        "dry_run_execution_result_receipt_ledger_recorded",
        "dry_run_execution_result_receipt_delivered",
        "dry_run_execution_result_receipt_materialized",
        "dry_run_execution_executed",
        "dry_run_execution_result_persisted",
        "acceptance_receipt_persisted",
        "operator_packet_persisted",
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
    const TRUE_RETENTION_KEYS: &[&str] = &[
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary_performed",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_result_recorded",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_result_accepted",
        "source_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary_accepted",
        "dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_matrix_bound",
        "dry_run_execution_result_receipt_retention_policy_request_denied",
        "dry_run_execution_result_receipt_retention_index_denied",
        "dry_run_execution_result_receipt_ttl_lease_update_extension_denied",
        "dry_run_execution_result_receipt_expiry_request_denied",
        "dry_run_execution_result_receipt_expiry_scheduler_timer_denied",
        "dry_run_execution_result_receipt_garbage_collection_request_denied",
        "dry_run_execution_result_receipt_garbage_collection_scan_denied",
        "dry_run_execution_result_receipt_delete_tombstone_sweep_denied",
        "dry_run_execution_result_receipt_archive_compaction_denied",
        "dry_run_execution_result_receipt_retention_gc_authority_denied",
        "dry_run_execution_result_receipt_retention_expiry_garbage_collection_handoff_bound",
    ];

    let route_matrix = control_ui_route_parity_report();
    let source = std::thread::Builder::new()
        .name("hepta-memory-production-durable-dry-run-result-receipt-retention-source-report".to_string())
        .stack_size(8 * 1024 * 1024)
        .spawn(
            hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary_report,
        )
        .ok()
        .and_then(|handle| handle.join().ok())
        .unwrap_or_else(|| {
            serde_json::json!({
                "status": "blocked",
                "memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary_ready": false,
                "scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_accepted": false,
                "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_source_report_thread_failed": true
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
    let source_next_action_retention = source
        .get("allowed_next_actions")
        .and_then(serde_json::Value::as_array)
        .and_then(|items| items.get(1))
        .map(|item| {
            item.get("action").and_then(serde_json::Value::as_str)
                == Some("prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary")
                && item
                    .get("requires_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary")
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
                .get("scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_result_accepted")
                .and_then(serde_json::Value::as_bool)
                == Some(true)
                && effects
                    .get("dry_run_execution_result_receipt_audit_trail_recorded")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("dry_run_execution_result_receipt_immutable_evidence_persisted")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("dry_run_execution_result_receipt_hash_chain_recorded")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("dry_run_execution_result_receipt_authority_promoted_from_audit_trail")
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
            "memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary_ready",
        )
        && json_bool(
            &source,
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_accepted",
        )
        && json_u64(
            &source,
            "accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_fixture_count",
        ) == 1
        && json_u64(
            &source,
            "blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_fixture_count",
        ) == 9
        && json_u64(
            &source,
            "denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary_count",
        ) == 72
        && !json_bool(
            &source,
            "dry_run_execution_result_receipt_audit_trail_recorded",
        )
        && !json_bool(
            &source,
            "dry_run_execution_result_receipt_audit_trail_persisted",
        )
        && !json_bool(
            &source,
            "dry_run_execution_result_receipt_immutable_evidence_recorded",
        )
        && !json_bool(
            &source,
            "dry_run_execution_result_receipt_immutable_evidence_persisted",
        )
        && !json_bool(
            &source,
            "dry_run_execution_result_receipt_hash_chain_recorded",
        )
        && !json_bool(
            &source,
            "dry_run_execution_result_receipt_authority_promoted_from_audit_trail",
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
        && source_next_action_retention
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
    let source_audit_boundary_hash_sha256 = json_str(
        &source,
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary_hash_sha256",
    );
    let source_audit_policy_hash_sha256 = json_str(
        &source,
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_policy_hash_sha256",
    );
    let source_audit_result_hash_sha256 = json_str(
        &source,
        "dry_run_execution_result_receipt_audit_evidence_result_hash_sha256",
    );
    let source_audit_handoff_hash_sha256 = json_str(
        &source,
        "dry_run_execution_result_receipt_audit_evidence_handoff_hash_sha256",
    );
    let source_audit_trail_matrix_hash_sha256 = json_str(
        &source,
        "dry_run_execution_result_receipt_audit_trail_denial_matrix_hash_sha256",
    );
    let source_immutable_evidence_matrix_hash_sha256 = json_str(
        &source,
        "dry_run_execution_result_receipt_immutable_evidence_denial_matrix_hash_sha256",
    );

    let retention_policy_denial_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-retention-policy-denial:v1:source={source_audit_result_hash_sha256}:retention-policy=false:record=false:persist=false:materialize=false:filesystem=false"
    ));
    let retention_index_denial_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-retention-index-denial:v1:policy={retention_policy_denial_hash_sha256}:index=false:record=false:persist=false"
    ));
    let expiry_lifecycle_denial_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-expiry-lifecycle-denial:v1:retention={retention_index_denial_hash_sha256}:ttl=false:expiry=false:scheduler=false:timer=false:ack=false:persist=false"
    ));
    let garbage_collection_denial_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-garbage-collection-denial:v1:expiry={expiry_lifecycle_denial_hash_sha256}:queue=false:scan=false:candidate=false:decision=false:delete=false:tombstone=false:sweep=false"
    ));
    let archive_compaction_denial_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-archive-compaction-denial:v1:gc={garbage_collection_denial_hash_sha256}:archive=false:compaction=false:artifact=false"
    ));
    let retention_evidence_handoff_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-retention-expiry-garbage-collection-handoff:v1:archive={archive_compaction_denial_hash_sha256}:next=export-query-observability-denial-boundary"
    ));
    let retention_result_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-retention-expiry-garbage-collection-result:v1:retention={retention_policy_denial_hash_sha256}:expiry={expiry_lifecycle_denial_hash_sha256}:gc={garbage_collection_denial_hash_sha256}:handoff={retention_evidence_handoff_hash_sha256}:accepted=true:record=false:persist=false:delete=false:authority=false:execution=false:production-write=false"
    ));
    let retention_boundary_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-retention-expiry-garbage-collection-denial-boundary:v1:source={source_report_sha256}:result={retention_result_hash_sha256}:fixtures=10:accepted=1:denials=62:retention=false:expiry=false:gc=false:delete=false:archive=false:compaction=false:authority=false:dry-run-executed=false:production-write=false"
    ));
    let retention_boundary_policy_hash_sha256 = sha256_text_value(
        "scoped-production-durable-memory-write-dry-run-result-receipt-retention-expiry-garbage-collection-denial-policy:v1:bind-source-audit-evidence-no-retention-policy-no-index-no-ttl-no-expiry-scheduler-no-gc-queue-no-scan-no-delete-no-tombstone-no-sweep-no-archive-no-compaction-no-authority-no-execution-no-production-write-no-kg-no-provider-no-channel-no-release-no-install",
    );

    let retention_bound = !source_report_sha256.is_empty()
        && !source_audit_boundary_hash_sha256.is_empty()
        && !source_audit_policy_hash_sha256.is_empty()
        && !source_audit_result_hash_sha256.is_empty()
        && !source_audit_handoff_hash_sha256.is_empty()
        && !source_audit_trail_matrix_hash_sha256.is_empty()
        && !source_immutable_evidence_matrix_hash_sha256.is_empty()
        && approved_production_namespace == "hepta.memory.production.scoped"
        && approved_production_store == "hepta-memory-durable-store-production-preflight-only"
        && approved_production_scope == "operator-approved-session"
        && production_durable_memory_target_id
            == "hepta-scoped-production-durable-memory-write-target-v1"
        && production_durable_memory_payload_class
            == "redacted-minimal-operator-approved-memory-fact";
    let surfaces_ready = source_ready && retention_bound;
    let report_ready = route_count_source_command_accepted && surfaces_ready;
    let accepted_fixture_count = if report_ready { 1 } else { 0 };
    let blocked_fixture_count = 10 - accepted_fixture_count;

    let mut fixtures = Vec::new();
    fixtures.push(serde_json::json!({
        "id": "scoped-production-durable-memory-write-dry-run-execution-result-receipt-retention-expiry-garbage-collection-denial",
        "fixture_id": "scoped-production-durable-memory-write-dry-run-execution-result-receipt-retention-expiry-garbage-collection-denial",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_accepted": report_ready,
        "reason": if report_ready { "dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_bound_without_lifecycle_persistence_delete_authority_execution_or_production_write" } else { "source_audit_evidence_or_route_count_not_ready" },
        "source_audit_trail_immutable_evidence_denial_boundary_bound": report_ready,
        "dry_run_execution_result_receipt_retention_policy_denied": report_ready,
        "dry_run_execution_result_receipt_expiry_request_denied": report_ready,
        "dry_run_execution_result_receipt_garbage_collection_request_denied": report_ready,
        "dry_run_execution_result_receipt_retention_policy_recorded": false,
        "dry_run_execution_result_receipt_expiry_scheduler_registered": false,
        "dry_run_execution_result_receipt_garbage_collection_scan_performed": false,
        "dry_run_execution_result_receipt_delete_marker_garbage_collected": false,
        "dry_run_execution_result_receipt_tombstone_garbage_collected": false,
        "dry_run_execution_result_receipt_archive_written": false,
        "dry_run_execution_result_receipt_compaction_performed": false,
        "dry_run_execution_executed": false,
        "production_durable_memory_store_write_performed": false,
        "external_send_performed": false
    }));
    for id in [
        "missing-audit-evidence-source",
        "retention-policy-record-request-attempt",
        "retention-index-request-attempt",
        "ttl-lease-update-extension-attempt",
        "expiry-scheduler-timer-ack-attempt",
        "garbage-collection-queue-scan-attempt",
        "delete-tombstone-sweep-attempt",
        "archive-compaction-attempt",
        "memory-kg-provider-channel-release-install-gc-evidence-attempt",
    ] {
        fixtures.push(serde_json::json!({
            "id": id,
            "fixture_id": id,
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_accepted": false,
            "reason": "blocked_noop",
            "dry_run_execution_result_receipt_retention_policy_recorded": false,
            "dry_run_execution_result_receipt_retention_policy_persisted": false,
            "dry_run_execution_result_receipt_expiry_scheduler_registered": false,
            "dry_run_execution_result_receipt_expiry_timer_started": false,
            "dry_run_execution_result_receipt_garbage_collection_scan_performed": false,
            "dry_run_execution_result_receipt_delete_marker_garbage_collected": false,
            "dry_run_execution_result_receipt_tombstone_garbage_collected": false,
            "dry_run_execution_result_receipt_sweep_performed": false,
            "dry_run_execution_result_receipt_archive_written": false,
            "dry_run_execution_result_receipt_compaction_performed": false,
            "dry_run_execution_executed": false,
            "production_durable_memory_store_write_performed": false,
            "external_send_performed": false
        }));
    }

    let mut side_effects = serde_json::Map::new();
    for &key in FALSE_RETENTION_SIDE_EFFECT_KEYS {
        side_effects.insert(key.to_string(), serde_json::json!(false));
        side_effects.insert(format!("{key}_count"), serde_json::json!(0));
    }
    for &key in TRUE_RETENTION_KEYS {
        side_effects.insert(key.to_string(), serde_json::json!(report_ready));
        side_effects.insert(
            format!("{key}_count"),
            serde_json::json!(if report_ready { 1 } else { 0 }),
        );
    }

    let mut report = source.clone();
    extend_json_object(
        &mut report,
        serde_json::json!({
            "gate": "hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary_route",
            "status": if report_ready { "ready" } else { "blocked" },
            "endpoint": HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_PRODUCTION_DURABLE_MEMORY_WRITE_DRY_RUN_EXECUTION_RESULT_RECEIPT_RETENTION_EXPIRY_GARBAGE_COLLECTION_DENIAL_BOUNDARY_ENDPOINT,
            "source_command": "/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-retention-expiry-garbage-collection-denial-boundary --json",
            "native_route": true,
            "compatibility_mode": "native_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary_status",
            "side_effect_free": false,
            "external_side_effect_free": true,
            "audit_date": "2026-07-05",
            "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
            "route_count": route_matrix.route_count,
            "implemented_route_count": route_matrix.implemented_route_count,
            "missing_route_count": route_matrix.missing_route_count,
            "route_count_source_command_accepted": route_count_source_command_accepted,
            "memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary_ready": report_ready,
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_ready": report_ready,
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_performed": report_ready,
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_accepted": report_ready,
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_mode": "dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary_no_retention_no_expiry_no_gc_no_delete_no_archive_no_compaction_no_authority_no_execution_no_production_durable_memory_mutation",
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary_ready": source_ready,
            "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary_report_sha256": source_report_sha256,
            "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary_accepted_count": json_u64(&source, "scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_result_accepted_count"),
            "source_accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_fixture_count": json_u64(&source, "accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_fixture_count"),
            "source_blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_fixture_count": json_u64(&source, "blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_fixture_count"),
            "source_denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary_count": json_u64(&source, "denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary_count"),
            "approved_production_namespace": approved_production_namespace,
            "approved_production_store": approved_production_store,
            "approved_production_scope": approved_production_scope,
            "production_durable_memory_target_id": production_durable_memory_target_id,
            "production_durable_memory_payload_class": production_durable_memory_payload_class,
            "operator_packet_scope": operator_packet_scope,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary_hash_sha256": source_audit_boundary_hash_sha256,
            "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_policy_hash_sha256": source_audit_policy_hash_sha256,
            "source_dry_run_execution_result_receipt_audit_evidence_result_hash_sha256": source_audit_result_hash_sha256,
            "source_dry_run_execution_result_receipt_audit_evidence_handoff_hash_sha256": source_audit_handoff_hash_sha256,
            "source_dry_run_execution_result_receipt_audit_trail_denial_matrix_hash_sha256": source_audit_trail_matrix_hash_sha256,
            "source_dry_run_execution_result_receipt_immutable_evidence_denial_matrix_hash_sha256": source_immutable_evidence_matrix_hash_sha256,
            "dry_run_execution_result_receipt_retention_policy_denial_hash_sha256": retention_policy_denial_hash_sha256,
            "dry_run_execution_result_receipt_retention_index_denial_hash_sha256": retention_index_denial_hash_sha256,
            "dry_run_execution_result_receipt_expiry_lifecycle_denial_hash_sha256": expiry_lifecycle_denial_hash_sha256,
            "dry_run_execution_result_receipt_garbage_collection_denial_hash_sha256": garbage_collection_denial_hash_sha256,
            "dry_run_execution_result_receipt_archive_compaction_denial_hash_sha256": archive_compaction_denial_hash_sha256,
            "dry_run_execution_result_receipt_retention_expiry_garbage_collection_handoff_hash_sha256": retention_evidence_handoff_hash_sha256,
            "dry_run_execution_result_receipt_retention_expiry_garbage_collection_result_hash_sha256": retention_result_hash_sha256,
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary_hash_sha256": retention_boundary_hash_sha256,
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_policy_hash_sha256": retention_boundary_policy_hash_sha256,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "required_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_surface_count": RETENTION_SURFACES.len(),
            "ready_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_surface_count": if surfaces_ready { RETENTION_SURFACES.len() } else { 0 },
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_surfaces": RETENTION_SURFACES,
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_fixture_count": fixtures.len(),
            "accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_fixture_count": accepted_fixture_count,
            "blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_fixture_count": blocked_fixture_count,
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_fixtures": fixtures,
            "denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary": RETENTION_DENIALS,
            "denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary_count": RETENTION_DENIALS.len(),
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [
                {
                    "action": "run_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary_require_live_gate",
                    "status": "allowed_verification_only",
                    "accepts_retention_expiry_garbage_collection_denial_matrix": true,
                    "records_retention_policy": false,
                    "persists_retention_policy": false,
                    "registers_expiry_scheduler": false,
                    "starts_expiry_timer": false,
                    "records_garbage_collection_queue": false,
                    "performs_garbage_collection_scan": false,
                    "deletes_memory": false,
                    "writes_tombstone": false,
                    "performs_sweep": false,
                    "writes_archive": false,
                    "performs_compaction": false,
                    "promotes_authority": false,
                    "executes_dry_run": false,
                    "persists_dry_run_result_receipt": false,
                    "writes_production_durable_memory": false,
                    "writes_memory_store": false,
                    "writes_wal": false,
                    "persists_receipt": false
                },
                {
                    "action": "prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_boundary",
                    "status": "requires_separate_result_receipt_export_query_observability_denial_gate",
                    "requires_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary": true,
                    "records_retention_policy": false,
                    "persists_retention_policy": false,
                    "performs_garbage_collection_scan": false,
                    "deletes_memory": false,
                    "executes_dry_run": false,
                    "writes_production_durable_memory": false,
                    "persists_dry_run_result_receipt": false
                }
            ]
        }),
    );
    let report_object = report.as_object_mut().expect(
        "scoped production durable Memory write dry-run execution result receipt retention/expiry/garbage-collection denial report object",
    );
    for &key in FALSE_RETENTION_SIDE_EFFECT_KEYS {
        report_object.insert(key.to_string(), serde_json::json!(false));
        report_object.insert(format!("{key}_count"), serde_json::json!(0));
    }
    for &key in TRUE_RETENTION_KEYS {
        report_object.insert(key.to_string(), serde_json::json!(report_ready));
        report_object.insert(
            format!("{key}_count"),
            serde_json::json!(if report_ready { 1 } else { 0 }),
        );
    }
    for key in [
        "source_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary_bound",
        "approved_production_namespace_bound",
        "approved_production_store_bound",
        "approved_production_scope_bound",
        "production_durable_memory_target_bound",
        "dry_run_execution_result_receipt_retention_policy_denial_bound",
        "dry_run_execution_result_receipt_retention_index_denial_bound",
        "dry_run_execution_result_receipt_expiry_lifecycle_denial_bound",
        "dry_run_execution_result_receipt_garbage_collection_denial_bound",
        "dry_run_execution_result_receipt_archive_compaction_denial_bound",
        "dry_run_execution_result_receipt_retention_policy_request_denied",
        "dry_run_execution_result_receipt_retention_index_denied",
        "dry_run_execution_result_receipt_ttl_lease_update_extension_denied",
        "dry_run_execution_result_receipt_expiry_request_denied",
        "dry_run_execution_result_receipt_expiry_scheduler_timer_denied",
        "dry_run_execution_result_receipt_garbage_collection_request_denied",
        "dry_run_execution_result_receipt_garbage_collection_scan_denied",
        "dry_run_execution_result_receipt_delete_tombstone_sweep_denied",
        "dry_run_execution_result_receipt_archive_compaction_denied",
        "dry_run_execution_result_receipt_retention_gc_authority_denied",
        "dry_run_execution_result_receipt_retention_expiry_garbage_collection_handoff_bound",
        "dry_run_execution_result_receipt_retention_expiry_garbage_collection_persistence_forbidden",
        "dry_run_execution_execution_forbidden_on_retention_gc_route",
        "dry_run_execution_result_receipt_persistence_forbidden_on_retention_gc_route",
        "production_write_execution_forbidden_on_retention_gc_route",
        "production_durable_memory_write_forbidden",
        "memory_store_mutation_forbidden",
        "wal_write_forbidden_on_retention_gc_route",
        "receipt_persist_forbidden_on_retention_gc_route",
        "rollback_execution_forbidden_on_retention_gc_route",
        "tombstone_write_forbidden_on_retention_gc_route",
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

fn hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_boundary_report()
-> serde_json::Value {
    const EXPORT_QUERY_OBSERVABILITY_SURFACES: &[&str] = &[
        "source_retention_expiry_garbage_collection_denial_boundary_required",
        "source_retention_expiry_garbage_collection_result_required",
        "dry_run_execution_result_receipt_export_request_denied",
        "dry_run_execution_result_receipt_export_snapshot_denied",
        "dry_run_execution_result_receipt_export_file_stream_denied",
        "dry_run_execution_result_receipt_query_registration_denied",
        "dry_run_execution_result_receipt_query_execution_denied",
        "dry_run_execution_result_receipt_query_result_recording_denied",
        "dry_run_execution_result_receipt_query_index_cache_denied",
        "dry_run_execution_result_receipt_observability_metric_log_denied",
        "dry_run_execution_result_receipt_observability_trace_span_event_denied",
        "dry_run_execution_result_receipt_dashboard_alert_slo_denied",
        "dry_run_execution_result_receipt_operator_summary_readback_denied",
        "dry_run_execution_result_receipt_ledger_index_delivery_observability_denied",
        "dry_run_execution_result_receipt_memory_kg_provider_channel_observability_denied",
        "dry_run_execution_production_write_and_authority_forbidden_on_export_query_observability_route",
    ];
    const EXPORT_QUERY_OBSERVABILITY_DENIALS: &[&str] = &[
        "source_retention_expiry_garbage_collection_denial_boundary_required",
        "source_retention_expiry_garbage_collection_result_hash_required",
        "source_retention_expiry_garbage_collection_policy_hash_required",
        "source_retention_expiry_garbage_collection_handoff_hash_required",
        "approved_production_namespace_required",
        "approved_production_store_required",
        "approved_production_scope_required",
        "production_durable_memory_target_required",
        "export_request_acceptance_denied",
        "export_request_recording_denied",
        "export_request_persistence_denied",
        "export_snapshot_materialization_denied",
        "export_file_write_denied",
        "export_stream_open_denied",
        "export_delivery_denied",
        "export_query_authority_promotion_denied",
        "query_registration_denied",
        "query_execution_denied",
        "query_result_recording_denied",
        "query_result_persistence_denied",
        "query_result_materialization_denied",
        "query_endpoint_materialization_denied",
        "query_index_recording_denied",
        "query_cache_write_denied",
        "search_index_write_denied",
        "observability_metric_recording_denied",
        "observability_log_recording_denied",
        "observability_trace_recording_denied",
        "observability_span_recording_denied",
        "observability_event_recording_denied",
        "observability_dashboard_materialization_denied",
        "observability_alert_registration_denied",
        "observability_slo_recording_denied",
        "operator_summary_recording_denied",
        "operator_summary_persistence_denied",
        "operator_summary_delivery_denied",
        "readback_evidence_recording_denied",
        "ledger_observability_recording_denied",
        "index_observability_recording_denied",
        "delivery_observability_recording_denied",
        "result_receipt_export_authority_promotion_denied",
        "result_receipt_query_authority_promotion_denied",
        "result_receipt_observability_authority_promotion_denied",
        "dry_run_execution_execution_denied",
        "dry_run_execution_result_receipt_persistence_denied",
        "retention_expiry_gc_state_mutation_denied",
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
        "credential_secret_read_denied",
        "channel_external_send_denied",
        "release_public_artifact_write_denied",
        "install_restart_authority_denied",
        "active_binary_mutation_denied",
        "unrestricted_full_live_activation_denied",
    ];
    const FALSE_EXPORT_QUERY_OBSERVABILITY_SIDE_EFFECT_KEYS: &[&str] = &[
        "dry_run_execution_result_receipt_export_request_accepted",
        "dry_run_execution_result_receipt_export_recorded",
        "dry_run_execution_result_receipt_export_persisted",
        "dry_run_execution_result_receipt_export_snapshot_materialized",
        "dry_run_execution_result_receipt_export_file_written",
        "dry_run_execution_result_receipt_export_stream_opened",
        "dry_run_execution_result_receipt_export_delivered",
        "dry_run_execution_result_receipt_query_registered",
        "dry_run_execution_result_receipt_query_executed",
        "dry_run_execution_result_receipt_query_result_recorded",
        "dry_run_execution_result_receipt_query_result_persisted",
        "dry_run_execution_result_receipt_query_result_materialized",
        "dry_run_execution_result_receipt_query_endpoint_materialized",
        "dry_run_execution_result_receipt_query_index_recorded",
        "dry_run_execution_result_receipt_query_cache_written",
        "dry_run_execution_result_receipt_search_index_written",
        "dry_run_execution_result_receipt_observability_metric_recorded",
        "dry_run_execution_result_receipt_observability_log_recorded",
        "dry_run_execution_result_receipt_observability_trace_recorded",
        "dry_run_execution_result_receipt_observability_span_recorded",
        "dry_run_execution_result_receipt_observability_event_recorded",
        "dry_run_execution_result_receipt_observability_dashboard_materialized",
        "dry_run_execution_result_receipt_observability_alert_registered",
        "dry_run_execution_result_receipt_observability_slo_recorded",
        "dry_run_execution_result_receipt_operator_summary_recorded",
        "dry_run_execution_result_receipt_operator_summary_persisted",
        "dry_run_execution_result_receipt_operator_summary_delivered",
        "dry_run_execution_result_receipt_readback_evidence_recorded",
        "dry_run_execution_result_receipt_ledger_observability_recorded",
        "dry_run_execution_result_receipt_index_observability_recorded",
        "dry_run_execution_result_receipt_delivery_observability_recorded",
        "dry_run_execution_result_receipt_authority_promoted_from_export",
        "dry_run_execution_result_receipt_authority_promoted_from_query",
        "dry_run_execution_result_receipt_authority_promoted_from_observability",
        "dry_run_execution_result_receipt_retention_policy_recorded",
        "dry_run_execution_result_receipt_retention_policy_persisted",
        "dry_run_execution_result_receipt_expiry_scheduler_registered",
        "dry_run_execution_result_receipt_garbage_collection_scan_performed",
        "dry_run_execution_result_receipt_delete_marker_garbage_collected",
        "dry_run_execution_result_receipt_tombstone_garbage_collected",
        "dry_run_execution_result_receipt_persisted",
        "dry_run_execution_result_receipt_filesystem_written",
        "dry_run_execution_result_receipt_ledger_recorded",
        "dry_run_execution_result_receipt_delivered",
        "dry_run_execution_result_receipt_materialized",
        "dry_run_execution_executed",
        "dry_run_execution_result_persisted",
        "acceptance_receipt_persisted",
        "operator_packet_persisted",
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
    const TRUE_EXPORT_QUERY_OBSERVABILITY_KEYS: &[&str] = &[
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_boundary_performed",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_result_recorded",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_result_accepted",
        "source_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary_accepted",
        "dry_run_execution_result_receipt_export_query_observability_denial_matrix_bound",
        "dry_run_execution_result_receipt_export_request_denied",
        "dry_run_execution_result_receipt_export_file_stream_denied",
        "dry_run_execution_result_receipt_query_registration_execution_denied",
        "dry_run_execution_result_receipt_query_index_cache_denied",
        "dry_run_execution_result_receipt_observability_metric_log_trace_event_denied",
        "dry_run_execution_result_receipt_dashboard_alert_slo_denied",
        "dry_run_execution_result_receipt_operator_summary_readback_denied",
        "dry_run_execution_result_receipt_export_query_observability_authority_denied",
        "dry_run_execution_result_receipt_export_query_observability_handoff_bound",
    ];

    let route_matrix = control_ui_route_parity_report();
    let source = std::thread::Builder::new()
        .name("hepta-memory-production-durable-dry-run-result-receipt-export-query-observability-source-report".to_string())
        .stack_size(8 * 1024 * 1024)
        .spawn(
            hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary_report,
        )
        .ok()
        .and_then(|handle| handle.join().ok())
        .unwrap_or_else(|| {
            serde_json::json!({
                "status": "blocked",
                "memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary_ready": false,
                "scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_accepted": false,
                "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_source_report_thread_failed": true
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
    let source_next_action_export_query_observability = source
        .get("allowed_next_actions")
        .and_then(serde_json::Value::as_array)
        .and_then(|items| items.get(1))
        .map(|item| {
            item.get("action").and_then(serde_json::Value::as_str)
                == Some("prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_boundary")
                && item
                    .get("requires_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary")
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
                .get("scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_result_accepted")
                .and_then(serde_json::Value::as_bool)
                == Some(true)
                && effects
                    .get("dry_run_execution_result_receipt_retention_policy_recorded")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("dry_run_execution_result_receipt_expiry_scheduler_registered")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("dry_run_execution_result_receipt_garbage_collection_scan_performed")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("dry_run_execution_result_receipt_authority_promoted_from_garbage_collection")
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
            "memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary_ready",
        )
        && json_bool(
            &source,
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_accepted",
        )
        && json_u64(
            &source,
            "accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_fixture_count",
        ) == 1
        && json_u64(
            &source,
            "blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_fixture_count",
        ) == 9
        && json_u64(
            &source,
            "denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary_count",
        ) == 62
        && !json_bool(
            &source,
            "dry_run_execution_result_receipt_retention_policy_recorded",
        )
        && !json_bool(
            &source,
            "dry_run_execution_result_receipt_retention_policy_persisted",
        )
        && !json_bool(
            &source,
            "dry_run_execution_result_receipt_expiry_scheduler_registered",
        )
        && !json_bool(
            &source,
            "dry_run_execution_result_receipt_garbage_collection_scan_performed",
        )
        && !json_bool(
            &source,
            "dry_run_execution_result_receipt_authority_promoted_from_garbage_collection",
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
        && source_next_action_export_query_observability
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
    let source_retention_boundary_hash_sha256 = json_str(
        &source,
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary_hash_sha256",
    );
    let source_retention_policy_hash_sha256 = json_str(
        &source,
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_policy_hash_sha256",
    );
    let source_retention_result_hash_sha256 = json_str(
        &source,
        "dry_run_execution_result_receipt_retention_expiry_garbage_collection_result_hash_sha256",
    );
    let source_retention_handoff_hash_sha256 = json_str(
        &source,
        "dry_run_execution_result_receipt_retention_expiry_garbage_collection_handoff_hash_sha256",
    );
    let source_garbage_collection_denial_hash_sha256 = json_str(
        &source,
        "dry_run_execution_result_receipt_garbage_collection_denial_hash_sha256",
    );

    let export_denial_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-export-denial:v1:source={source_retention_result_hash_sha256}:export-request=false:snapshot=false:file=false:stream=false:delivery=false"
    ));
    let query_denial_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-query-denial:v1:export={export_denial_hash_sha256}:register=false:execute=false:result=false:index=false:cache=false:search-index=false"
    ));
    let observability_denial_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-observability-denial:v1:query={query_denial_hash_sha256}:metric=false:log=false:trace=false:span=false:event=false:dashboard=false:alert=false:slo=false:operator-summary=false:readback=false"
    ));
    let export_query_observability_handoff_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-export-query-observability-handoff:v1:observability={observability_denial_hash_sha256}:next=operator-facing-summary-briefing-non-persistence-denial-boundary"
    ));
    let export_query_observability_result_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-export-query-observability-result:v1:export={export_denial_hash_sha256}:query={query_denial_hash_sha256}:observability={observability_denial_hash_sha256}:handoff={export_query_observability_handoff_hash_sha256}:accepted=true:persist=false:authority=false:execution=false:production-write=false"
    ));
    let export_query_observability_boundary_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-export-query-observability-denial-boundary:v1:source={source_report_sha256}:result={export_query_observability_result_hash_sha256}:fixtures=10:accepted=1:denials=64:export=false:query=false:observability=false:authority=false:dry-run-executed=false:production-write=false"
    ));
    let export_query_observability_policy_hash_sha256 = sha256_text_value(
        "scoped-production-durable-memory-write-dry-run-result-receipt-export-query-observability-denial-policy:v1:bind-source-retention-gc-no-export-no-query-no-search-index-no-metric-no-log-no-trace-no-event-no-dashboard-no-alert-no-slo-no-operator-summary-no-readback-evidence-no-authority-no-execution-no-production-write-no-kg-no-provider-no-channel-no-release-no-install",
    );
    let export_query_observability_matrix_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-export-query-observability-denial-matrix:v1:export={export_denial_hash_sha256}:query={query_denial_hash_sha256}:observability={observability_denial_hash_sha256}:denials={}",
        EXPORT_QUERY_OBSERVABILITY_DENIALS.len()
    ));

    let export_query_observability_bound = !source_report_sha256.is_empty()
        && !source_retention_boundary_hash_sha256.is_empty()
        && !source_retention_policy_hash_sha256.is_empty()
        && !source_retention_result_hash_sha256.is_empty()
        && !source_retention_handoff_hash_sha256.is_empty()
        && !source_garbage_collection_denial_hash_sha256.is_empty()
        && approved_production_namespace == "hepta.memory.production.scoped"
        && approved_production_store == "hepta-memory-durable-store-production-preflight-only"
        && approved_production_scope == "operator-approved-session"
        && production_durable_memory_target_id
            == "hepta-scoped-production-durable-memory-write-target-v1"
        && production_durable_memory_payload_class
            == "redacted-minimal-operator-approved-memory-fact";
    let surfaces_ready = source_ready && export_query_observability_bound;
    let report_ready = route_count_source_command_accepted && surfaces_ready;
    let accepted_fixture_count = if report_ready { 1 } else { 0 };
    let blocked_fixture_count = 10 - accepted_fixture_count;

    let mut fixtures = Vec::new();
    fixtures.push(serde_json::json!({
        "id": "scoped-production-durable-memory-write-dry-run-execution-result-receipt-export-query-observability-denial",
        "fixture_id": "scoped-production-durable-memory-write-dry-run-execution-result-receipt-export-query-observability-denial",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_accepted": report_ready,
        "reason": if report_ready { "dry_run_execution_result_receipt_export_query_observability_denial_bound_without_reporting_surface_persistence_authority_execution_or_production_write" } else { "source_retention_gc_or_route_count_not_ready" },
        "source_retention_expiry_garbage_collection_denial_boundary_bound": report_ready,
        "dry_run_execution_result_receipt_export_request_denied": report_ready,
        "dry_run_execution_result_receipt_query_registration_execution_denied": report_ready,
        "dry_run_execution_result_receipt_observability_metric_log_trace_event_denied": report_ready,
        "dry_run_execution_result_receipt_export_recorded": false,
        "dry_run_execution_result_receipt_export_file_written": false,
        "dry_run_execution_result_receipt_query_registered": false,
        "dry_run_execution_result_receipt_query_executed": false,
        "dry_run_execution_result_receipt_observability_metric_recorded": false,
        "dry_run_execution_result_receipt_observability_log_recorded": false,
        "dry_run_execution_result_receipt_observability_trace_recorded": false,
        "dry_run_execution_result_receipt_observability_dashboard_materialized": false,
        "dry_run_execution_result_receipt_authority_promoted_from_observability": false,
        "dry_run_execution_executed": false,
        "production_durable_memory_store_write_performed": false,
        "external_send_performed": false
    }));
    for id in [
        "missing-retention-expiry-garbage-collection-source",
        "export-request-snapshot-attempt",
        "export-file-stream-attempt",
        "query-registration-execution-attempt",
        "query-result-index-cache-attempt",
        "observability-metric-log-attempt",
        "observability-trace-span-event-attempt",
        "dashboard-alert-slo-attempt",
        "operator-summary-readback-authority-attempt",
    ] {
        fixtures.push(serde_json::json!({
            "id": id,
            "fixture_id": id,
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_accepted": false,
            "reason": "blocked_noop",
            "dry_run_execution_result_receipt_export_recorded": false,
            "dry_run_execution_result_receipt_export_persisted": false,
            "dry_run_execution_result_receipt_export_file_written": false,
            "dry_run_execution_result_receipt_export_stream_opened": false,
            "dry_run_execution_result_receipt_query_registered": false,
            "dry_run_execution_result_receipt_query_executed": false,
            "dry_run_execution_result_receipt_query_index_recorded": false,
            "dry_run_execution_result_receipt_query_cache_written": false,
            "dry_run_execution_result_receipt_observability_metric_recorded": false,
            "dry_run_execution_result_receipt_observability_log_recorded": false,
            "dry_run_execution_result_receipt_observability_trace_recorded": false,
            "dry_run_execution_result_receipt_observability_dashboard_materialized": false,
            "dry_run_execution_result_receipt_observability_alert_registered": false,
            "dry_run_execution_result_receipt_operator_summary_recorded": false,
            "dry_run_execution_result_receipt_readback_evidence_recorded": false,
            "dry_run_execution_executed": false,
            "production_durable_memory_store_write_performed": false,
            "external_send_performed": false
        }));
    }

    let mut side_effects = serde_json::Map::new();
    for &key in FALSE_EXPORT_QUERY_OBSERVABILITY_SIDE_EFFECT_KEYS {
        side_effects.insert(key.to_string(), serde_json::json!(false));
        side_effects.insert(format!("{key}_count"), serde_json::json!(0));
    }
    for &key in TRUE_EXPORT_QUERY_OBSERVABILITY_KEYS {
        side_effects.insert(key.to_string(), serde_json::json!(report_ready));
        side_effects.insert(
            format!("{key}_count"),
            serde_json::json!(if report_ready { 1 } else { 0 }),
        );
    }

    let mut report = source.clone();
    extend_json_object(
        &mut report,
        serde_json::json!({
            "gate": "hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_boundary_route",
            "status": if report_ready { "ready" } else { "blocked" },
            "endpoint": HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_PRODUCTION_DURABLE_MEMORY_WRITE_DRY_RUN_EXECUTION_RESULT_RECEIPT_EXPORT_QUERY_OBSERVABILITY_DENIAL_BOUNDARY_ENDPOINT,
            "source_command": "/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-export-query-observability-denial-boundary --json",
            "native_route": true,
            "compatibility_mode": "native_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_boundary_status",
            "side_effect_free": false,
            "external_side_effect_free": true,
            "audit_date": "2026-07-05",
            "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
            "route_count": route_matrix.route_count,
            "implemented_route_count": route_matrix.implemented_route_count,
            "missing_route_count": route_matrix.missing_route_count,
            "route_count_source_command_accepted": route_count_source_command_accepted,
            "memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_boundary_ready": report_ready,
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_ready": report_ready,
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_performed": report_ready,
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_accepted": report_ready,
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_mode": "dry_run_execution_result_receipt_export_query_observability_denial_boundary_no_export_no_query_no_observability_no_dashboard_no_alert_no_operator_summary_no_authority_no_execution_no_production_durable_memory_mutation",
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary_ready": source_ready,
            "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary_report_sha256": source_report_sha256,
            "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary_accepted_count": json_u64(&source, "scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_result_accepted_count"),
            "source_accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_fixture_count": json_u64(&source, "accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_fixture_count"),
            "source_blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_fixture_count": json_u64(&source, "blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_fixture_count"),
            "source_denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary_count": json_u64(&source, "denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary_count"),
            "approved_production_namespace": approved_production_namespace,
            "approved_production_store": approved_production_store,
            "approved_production_scope": approved_production_scope,
            "production_durable_memory_target_id": production_durable_memory_target_id,
            "production_durable_memory_payload_class": production_durable_memory_payload_class,
            "operator_packet_scope": operator_packet_scope,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary_hash_sha256": source_retention_boundary_hash_sha256,
            "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_policy_hash_sha256": source_retention_policy_hash_sha256,
            "source_dry_run_execution_result_receipt_retention_expiry_garbage_collection_result_hash_sha256": source_retention_result_hash_sha256,
            "source_dry_run_execution_result_receipt_retention_expiry_garbage_collection_handoff_hash_sha256": source_retention_handoff_hash_sha256,
            "source_dry_run_execution_result_receipt_garbage_collection_denial_hash_sha256": source_garbage_collection_denial_hash_sha256,
            "dry_run_execution_result_receipt_export_denial_hash_sha256": export_denial_hash_sha256,
            "dry_run_execution_result_receipt_query_denial_hash_sha256": query_denial_hash_sha256,
            "dry_run_execution_result_receipt_observability_denial_hash_sha256": observability_denial_hash_sha256,
            "dry_run_execution_result_receipt_export_query_observability_denial_matrix_hash_sha256": export_query_observability_matrix_hash_sha256,
            "dry_run_execution_result_receipt_export_query_observability_handoff_hash_sha256": export_query_observability_handoff_hash_sha256,
            "dry_run_execution_result_receipt_export_query_observability_result_hash_sha256": export_query_observability_result_hash_sha256,
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_boundary_hash_sha256": export_query_observability_boundary_hash_sha256,
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_policy_hash_sha256": export_query_observability_policy_hash_sha256,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "required_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_surface_count": EXPORT_QUERY_OBSERVABILITY_SURFACES.len(),
            "ready_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_surface_count": if surfaces_ready { EXPORT_QUERY_OBSERVABILITY_SURFACES.len() } else { 0 },
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_surfaces": EXPORT_QUERY_OBSERVABILITY_SURFACES,
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_fixture_count": fixtures.len(),
            "accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_fixture_count": accepted_fixture_count,
            "blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_fixture_count": blocked_fixture_count,
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_fixtures": fixtures,
            "denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_boundary": EXPORT_QUERY_OBSERVABILITY_DENIALS,
            "denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_boundary_count": EXPORT_QUERY_OBSERVABILITY_DENIALS.len(),
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [
                {
                    "action": "run_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_boundary_require_live_gate",
                    "status": "allowed_verification_only",
                    "accepts_export_query_observability_denial_matrix": true,
                    "exports_receipt": false,
                    "materializes_export_snapshot": false,
                    "opens_export_stream": false,
                    "registers_query": false,
                    "executes_query": false,
                    "records_query_result": false,
                    "writes_search_index": false,
                    "records_observability": false,
                    "materializes_dashboard": false,
                    "registers_alert": false,
                    "records_slo": false,
                    "records_operator_summary": false,
                    "records_readback_evidence": false,
                    "promotes_authority": false,
                    "executes_dry_run": false,
                    "persists_dry_run_result_receipt": false,
                    "writes_production_durable_memory": false,
                    "writes_memory_store": false,
                    "writes_wal": false,
                    "persists_receipt": false
                },
                {
                    "action": "prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary",
                    "status": "requires_separate_result_receipt_operator_summary_briefing_denial_gate",
                    "requires_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_boundary": true,
                    "exports_receipt": false,
                    "registers_query": false,
                    "records_observability": false,
                    "persists_operator_summary": false,
                    "delivers_operator_summary": false,
                    "executes_dry_run": false,
                    "writes_production_durable_memory": false,
                    "persists_dry_run_result_receipt": false
                }
            ]
        }),
    );
    let report_object = report.as_object_mut().expect(
        "scoped production durable Memory write dry-run execution result receipt export/query/observability denial report object",
    );
    for &key in FALSE_EXPORT_QUERY_OBSERVABILITY_SIDE_EFFECT_KEYS {
        report_object.insert(key.to_string(), serde_json::json!(false));
        report_object.insert(format!("{key}_count"), serde_json::json!(0));
    }
    for &key in TRUE_EXPORT_QUERY_OBSERVABILITY_KEYS {
        report_object.insert(key.to_string(), serde_json::json!(report_ready));
        report_object.insert(
            format!("{key}_count"),
            serde_json::json!(if report_ready { 1 } else { 0 }),
        );
    }
    for key in [
        "source_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary_bound",
        "approved_production_namespace_bound",
        "approved_production_store_bound",
        "approved_production_scope_bound",
        "production_durable_memory_target_bound",
        "dry_run_execution_result_receipt_export_denial_bound",
        "dry_run_execution_result_receipt_query_denial_bound",
        "dry_run_execution_result_receipt_observability_denial_bound",
        "dry_run_execution_result_receipt_export_request_denied",
        "dry_run_execution_result_receipt_export_file_stream_denied",
        "dry_run_execution_result_receipt_query_registration_execution_denied",
        "dry_run_execution_result_receipt_query_index_cache_denied",
        "dry_run_execution_result_receipt_observability_metric_log_trace_event_denied",
        "dry_run_execution_result_receipt_dashboard_alert_slo_denied",
        "dry_run_execution_result_receipt_operator_summary_readback_denied",
        "dry_run_execution_result_receipt_export_query_observability_authority_denied",
        "dry_run_execution_result_receipt_export_query_observability_handoff_bound",
        "dry_run_execution_result_receipt_export_query_observability_persistence_forbidden",
        "dry_run_execution_execution_forbidden_on_export_query_observability_route",
        "dry_run_execution_result_receipt_persistence_forbidden_on_export_query_observability_route",
        "production_write_execution_forbidden_on_export_query_observability_route",
        "production_durable_memory_write_forbidden",
        "memory_store_mutation_forbidden",
        "wal_write_forbidden_on_export_query_observability_route",
        "receipt_persist_forbidden_on_export_query_observability_route",
        "rollback_execution_forbidden_on_export_query_observability_route",
        "tombstone_write_forbidden_on_export_query_observability_route",
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

fn hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_report()
-> serde_json::Value {
    const OPERATOR_SUMMARY_BRIEFING_SURFACES: &[&str] = &[
        "source_export_query_observability_denial_boundary_required",
        "source_export_query_observability_result_required",
        "operator_facing_summary_request_denied",
        "operator_briefing_request_denied",
        "operator_facing_summary_materialization_denied",
        "operator_briefing_materialization_denied",
        "operator_facing_summary_persistence_denied",
        "operator_briefing_persistence_denied",
        "operator_facing_summary_delivery_denied",
        "operator_briefing_delivery_denied",
        "operator_readout_handoff_denied",
        "final_acknowledgement_decision_status_denied",
        "operator_summary_briefing_authority_promotion_denied",
        "dry_run_execution_production_write_and_authority_forbidden_on_operator_summary_briefing_route",
    ];
    const DENIED_BY: &[&str] = &[
        "source_export_query_observability_denial_boundary_required",
        "source_export_query_observability_result_hash_required",
        "source_export_query_observability_policy_hash_required",
        "source_export_query_observability_handoff_hash_required",
        "approved_production_namespace_required",
        "approved_production_store_required",
        "approved_production_scope_required",
        "production_durable_memory_target_required",
        "operator_facing_summary_request_acceptance_denied",
        "operator_facing_summary_recording_denied",
        "operator_facing_summary_persistence_denied",
        "operator_facing_summary_materialization_denied",
        "operator_facing_summary_filesystem_write_denied",
        "operator_facing_summary_delivery_denied",
        "operator_facing_summary_channel_delivery_denied",
        "operator_briefing_request_acceptance_denied",
        "operator_briefing_recording_denied",
        "operator_briefing_persistence_denied",
        "operator_briefing_materialization_denied",
        "operator_briefing_filesystem_write_denied",
        "operator_briefing_delivery_denied",
        "operator_briefing_channel_delivery_denied",
        "operator_readout_recording_denied",
        "operator_readout_persistence_denied",
        "operator_readout_materialization_denied",
        "operator_readout_delivery_denied",
        "operator_readout_readback_evidence_denied",
        "operator_handoff_recording_denied",
        "operator_handoff_persistence_denied",
        "operator_handoff_delivery_denied",
        "final_operator_acknowledgement_recording_denied",
        "final_operator_acknowledgement_persistence_denied",
        "final_operator_acknowledgement_acceptance_denied",
        "final_operator_acknowledgement_delivery_denied",
        "terminal_operator_decision_recording_denied",
        "terminal_operator_decision_persistence_denied",
        "terminal_operator_decision_acceptance_denied",
        "terminal_operator_status_recording_denied",
        "terminal_operator_status_persistence_denied",
        "terminal_operator_status_promotion_denied",
        "result_receipt_operator_summary_authority_promotion_denied",
        "result_receipt_operator_briefing_authority_promotion_denied",
        "result_receipt_operator_readout_authority_promotion_denied",
        "result_receipt_final_acknowledgement_authority_promotion_denied",
        "dry_run_execution_execution_denied",
        "dry_run_execution_result_receipt_persistence_denied",
        "export_query_observability_state_mutation_denied",
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
        "credential_secret_read_denied",
        "telegram_channel_delivery_denied",
        "external_send_denied",
        "release_public_artifact_write_denied",
        "install_restart_authority_denied",
        "active_binary_mutation_denied",
        "unrestricted_full_live_activation_denied",
    ];
    const FALSE_KEYS: &[&str] = &[
        "dry_run_execution_result_receipt_operator_facing_summary_allowed",
        "dry_run_execution_result_receipt_operator_facing_summary_request_accepted",
        "dry_run_execution_result_receipt_operator_facing_summary_recorded",
        "dry_run_execution_result_receipt_operator_facing_summary_persisted",
        "dry_run_execution_result_receipt_operator_facing_summary_materialized",
        "dry_run_execution_result_receipt_operator_facing_summary_filesystem_written",
        "dry_run_execution_result_receipt_operator_facing_summary_delivered",
        "dry_run_execution_result_receipt_operator_facing_summary_channel_delivery_performed",
        "dry_run_execution_result_receipt_operator_briefing_allowed",
        "dry_run_execution_result_receipt_operator_briefing_request_accepted",
        "dry_run_execution_result_receipt_operator_briefing_recorded",
        "dry_run_execution_result_receipt_operator_briefing_persisted",
        "dry_run_execution_result_receipt_operator_briefing_materialized",
        "dry_run_execution_result_receipt_operator_briefing_filesystem_written",
        "dry_run_execution_result_receipt_operator_briefing_delivered",
        "dry_run_execution_result_receipt_operator_briefing_channel_delivery_performed",
        "dry_run_execution_result_receipt_operator_summary_briefing_channel_delivery_performed",
        "dry_run_execution_result_receipt_operator_readout_recorded",
        "dry_run_execution_result_receipt_operator_readout_persisted",
        "dry_run_execution_result_receipt_operator_readout_materialized",
        "dry_run_execution_result_receipt_operator_readout_delivered",
        "dry_run_execution_result_receipt_operator_readback_evidence_recorded",
        "dry_run_execution_result_receipt_operator_handoff_recorded",
        "dry_run_execution_result_receipt_operator_handoff_persisted",
        "dry_run_execution_result_receipt_operator_handoff_delivered",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_recorded",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_persisted",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_accepted",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_delivered",
        "dry_run_execution_result_receipt_terminal_operator_decision_recorded",
        "dry_run_execution_result_receipt_terminal_operator_decision_persisted",
        "dry_run_execution_result_receipt_terminal_operator_decision_accepted",
        "dry_run_execution_result_receipt_terminal_operator_status_recorded",
        "dry_run_execution_result_receipt_terminal_operator_status_persisted",
        "dry_run_execution_result_receipt_terminal_operator_status_promoted",
        "dry_run_execution_result_receipt_authority_promoted_from_operator_summary",
        "dry_run_execution_result_receipt_authority_promoted_from_operator_briefing",
        "dry_run_execution_result_receipt_authority_promoted_from_operator_readout",
        "dry_run_execution_result_receipt_authority_promoted_from_final_acknowledgement",
        "dry_run_execution_result_receipt_export_recorded",
        "dry_run_execution_result_receipt_export_persisted",
        "dry_run_execution_result_receipt_export_snapshot_materialized",
        "dry_run_execution_result_receipt_export_file_written",
        "dry_run_execution_result_receipt_export_stream_opened",
        "dry_run_execution_result_receipt_query_registered",
        "dry_run_execution_result_receipt_query_executed",
        "dry_run_execution_result_receipt_query_result_recorded",
        "dry_run_execution_result_receipt_query_index_recorded",
        "dry_run_execution_result_receipt_query_cache_written",
        "dry_run_execution_result_receipt_observability_metric_recorded",
        "dry_run_execution_result_receipt_observability_log_recorded",
        "dry_run_execution_result_receipt_observability_trace_recorded",
        "dry_run_execution_result_receipt_observability_event_recorded",
        "dry_run_execution_result_receipt_observability_dashboard_materialized",
        "dry_run_execution_result_receipt_observability_alert_registered",
        "dry_run_execution_result_receipt_observability_slo_recorded",
        "dry_run_execution_result_receipt_operator_summary_recorded",
        "dry_run_execution_result_receipt_readback_evidence_recorded",
        "dry_run_execution_result_receipt_authority_promoted_from_export",
        "dry_run_execution_result_receipt_authority_promoted_from_query",
        "dry_run_execution_result_receipt_authority_promoted_from_observability",
        "dry_run_execution_result_receipt_persisted",
        "dry_run_execution_result_receipt_filesystem_written",
        "dry_run_execution_result_receipt_ledger_recorded",
        "dry_run_execution_result_receipt_delivered",
        "dry_run_execution_result_receipt_materialized",
        "dry_run_execution_executed",
        "dry_run_execution_result_persisted",
        "acceptance_receipt_persisted",
        "operator_packet_persisted",
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
        "filesystem_written",
    ];
    const TRUE_KEYS: &[&str] = &[
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_performed",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_result_recorded",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_result_accepted",
        "source_dry_run_execution_result_receipt_export_query_observability_denial_boundary_bound",
        "dry_run_execution_result_receipt_operator_facing_summary_request_denied",
        "dry_run_execution_result_receipt_operator_briefing_request_denied",
        "dry_run_execution_result_receipt_operator_summary_briefing_materialization_denied",
        "dry_run_execution_result_receipt_operator_summary_briefing_persistence_denied",
        "dry_run_execution_result_receipt_operator_summary_briefing_delivery_denied",
        "dry_run_execution_result_receipt_operator_readout_handoff_denied",
        "dry_run_execution_result_receipt_final_acknowledgement_decision_status_denied",
        "dry_run_execution_result_receipt_operator_summary_briefing_authority_denied",
        "dry_run_execution_result_receipt_operator_summary_briefing_handoff_bound",
    ];

    fn operator_summary_briefing_fixture(
        id: &str,
        status: &str,
        reason: &str,
        accepted: bool,
        extra: serde_json::Value,
    ) -> serde_json::Value {
        let mut base = serde_json::Map::new();
        base.insert("id".to_string(), serde_json::json!(id));
        base.insert("fixture_id".to_string(), serde_json::json!(id));
        base.insert(
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_status".to_string(),
            serde_json::json!(status),
        );
        base.insert("reason".to_string(), serde_json::json!(reason));
        base.insert(
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_accepted".to_string(),
            serde_json::json!(accepted),
        );
        base.insert(
            "source_export_query_observability_present".to_string(),
            serde_json::json!(true),
        );
        base.insert(
            "source_export_query_observability_ready".to_string(),
            serde_json::json!(true),
        );
        base.insert(
            "summary_briefing_noop_confirmed".to_string(),
            serde_json::json!(true),
        );
        for key in [
            "operator_summary_requested",
            "operator_briefing_requested",
            "operator_summary_materialization_requested",
            "operator_briefing_materialization_requested",
            "operator_summary_persistence_requested",
            "operator_summary_filesystem_write_requested",
            "operator_briefing_persistence_requested",
            "operator_briefing_filesystem_write_requested",
            "operator_readout_requested",
            "operator_handoff_requested",
            "final_operator_acknowledgement_requested",
            "terminal_operator_decision_requested",
            "terminal_operator_status_requested",
            "channel_delivery_requested",
            "telegram_send_requested",
            "authority_promotion_requested",
            "dry_run_execution_requested",
            "production_write_requested",
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
            "operator_readout_recorded",
            "operator_readout_persisted",
            "operator_readout_materialized",
            "operator_readout_delivered",
            "operator_handoff_recorded",
            "operator_handoff_persisted",
            "operator_handoff_delivered",
            "final_operator_acknowledgement_recorded",
            "final_operator_acknowledgement_persisted",
            "final_operator_acknowledgement_accepted",
            "final_operator_acknowledgement_delivered",
            "terminal_operator_decision_recorded",
            "terminal_operator_decision_persisted",
            "terminal_operator_decision_accepted",
            "terminal_operator_status_recorded",
            "terminal_operator_status_persisted",
            "terminal_operator_status_promoted",
            "authority_promoted",
            "telegram_send_performed",
            "channel_send_performed",
            "external_send_performed",
            "receipt_recorded",
            "receipt_persisted",
            "receipt_accepted",
            "receipt_materialized",
            "receipt_filesystem_written",
            "dry_run_execution_executed",
            "production_durable_memory_store_write_performed",
            "memory_store_write_performed",
            "memory_store_mutated",
            "wal_write_performed",
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
        .name("hepta-memory-production-durable-dry-run-result-receipt-summary-briefing-source-report".to_string())
        .stack_size(8 * 1024 * 1024)
        .spawn(
            hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_boundary_report,
        )
        .ok()
        .and_then(|handle| handle.join().ok())
        .unwrap_or_else(|| {
            serde_json::json!({
                "status": "blocked",
                "memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_boundary_ready": false,
                "scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_accepted": false,
                "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_summary_briefing_source_report_thread_failed": true
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
    let source_next_action_operator_summary_briefing = source
        .get("allowed_next_actions")
        .and_then(serde_json::Value::as_array)
        .and_then(|items| items.get(1))
        .map(|item| {
            item.get("action").and_then(serde_json::Value::as_str)
                == Some("prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary")
                && item
                    .get("requires_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_boundary")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
                && item
                    .get("persists_operator_summary")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && item
                    .get("delivers_operator_summary")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
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
                .get("scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_result_accepted")
                .and_then(serde_json::Value::as_bool)
                == Some(true)
                && effects
                    .get("dry_run_execution_result_receipt_export_recorded")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("dry_run_execution_result_receipt_query_registered")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("dry_run_execution_result_receipt_observability_metric_recorded")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("dry_run_execution_result_receipt_observability_dashboard_materialized")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("dry_run_execution_result_receipt_authority_promoted_from_observability")
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
    let source_ready = json_str(&source, "status") == "ready"
        && json_bool(
            &source,
            "memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_boundary_ready",
        )
        && json_bool(
            &source,
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_accepted",
        )
        && json_u64(
            &source,
            "accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_fixture_count",
        ) == 1
        && json_u64(
            &source,
            "blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_fixture_count",
        ) == 9
        && json_u64(
            &source,
            "denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_boundary_count",
        ) == 64
        && !json_bool(&source, "dry_run_execution_result_receipt_export_recorded")
        && !json_bool(&source, "dry_run_execution_result_receipt_export_persisted")
        && !json_bool(
            &source,
            "dry_run_execution_result_receipt_export_file_written",
        )
        && !json_bool(&source, "dry_run_execution_result_receipt_query_registered")
        && !json_bool(&source, "dry_run_execution_result_receipt_query_executed")
        && !json_bool(
            &source,
            "dry_run_execution_result_receipt_query_index_recorded",
        )
        && !json_bool(
            &source,
            "dry_run_execution_result_receipt_query_cache_written",
        )
        && !json_bool(
            &source,
            "dry_run_execution_result_receipt_observability_metric_recorded",
        )
        && !json_bool(
            &source,
            "dry_run_execution_result_receipt_observability_dashboard_materialized",
        )
        && !json_bool(
            &source,
            "dry_run_execution_result_receipt_operator_summary_recorded",
        )
        && !json_bool(
            &source,
            "dry_run_execution_result_receipt_authority_promoted_from_observability",
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
        && source_next_action_operator_summary_briefing
        && source_side_effects_ok;

    let fixtures = serde_json::Value::Array(vec![
        operator_summary_briefing_fixture(
            "scoped-production-durable-memory-write-dry-run-result-receipt-operator-summary-briefing-report-only-denial",
            "accepted_non_persistent_operator_summary_briefing_denial",
            "source_export_query_observability_denial_bound_without_summary_briefing_persistence_delivery_or_authority",
            true,
            serde_json::json!({}),
        ),
        operator_summary_briefing_fixture(
            "scoped-production-durable-memory-write-dry-run-result-receipt-operator-summary-briefing-missing-source",
            "blocked_noop",
            "source_export_query_observability_report_required",
            false,
            serde_json::json!({
                "source_export_query_observability_present": false,
                "source_export_query_observability_ready": false,
                "operator_summary_requested": true
            }),
        ),
        operator_summary_briefing_fixture(
            "scoped-production-durable-memory-write-dry-run-result-receipt-operator-summary-request",
            "blocked_summary_noop",
            "operator_facing_summary_request_shape_denied",
            false,
            serde_json::json!({"operator_summary_requested": true}),
        ),
        operator_summary_briefing_fixture(
            "scoped-production-durable-memory-write-dry-run-result-receipt-operator-briefing-request",
            "blocked_briefing_noop",
            "operator_briefing_request_shape_denied",
            false,
            serde_json::json!({"operator_briefing_requested": true}),
        ),
        operator_summary_briefing_fixture(
            "scoped-production-durable-memory-write-dry-run-result-receipt-operator-summary-briefing-materialization-request",
            "blocked_materialization_noop",
            "summary_briefing_materialization_denied",
            false,
            serde_json::json!({
                "operator_summary_requested": true,
                "operator_briefing_requested": true,
                "operator_summary_materialization_requested": true,
                "operator_briefing_materialization_requested": true
            }),
        ),
        operator_summary_briefing_fixture(
            "scoped-production-durable-memory-write-dry-run-result-receipt-operator-summary-briefing-persistence-filesystem-write-request",
            "blocked_persistence_noop",
            "summary_briefing_persistence_filesystem_write_denied",
            false,
            serde_json::json!({
                "operator_summary_requested": true,
                "operator_briefing_requested": true,
                "operator_summary_persistence_requested": true,
                "operator_summary_filesystem_write_requested": true,
                "operator_briefing_persistence_requested": true,
                "operator_briefing_filesystem_write_requested": true
            }),
        ),
        operator_summary_briefing_fixture(
            "scoped-production-durable-memory-write-dry-run-result-receipt-operator-summary-briefing-channel-delivery-request",
            "blocked_delivery_noop",
            "summary_briefing_channel_delivery_denied",
            false,
            serde_json::json!({
                "operator_summary_requested": true,
                "operator_briefing_requested": true,
                "channel_delivery_requested": true,
                "telegram_send_requested": true
            }),
        ),
        operator_summary_briefing_fixture(
            "scoped-production-durable-memory-write-dry-run-result-receipt-operator-readout-handoff-request",
            "blocked_readout_noop",
            "operator_readout_handoff_denied",
            false,
            serde_json::json!({
                "operator_readout_requested": true,
                "operator_handoff_requested": true
            }),
        ),
        operator_summary_briefing_fixture(
            "scoped-production-durable-memory-write-dry-run-result-receipt-final-ack-decision-status-request",
            "blocked_ack_decision_status_noop",
            "final_acknowledgement_decision_status_denied",
            false,
            serde_json::json!({
                "final_operator_acknowledgement_requested": true,
                "terminal_operator_decision_requested": true,
                "terminal_operator_status_requested": true
            }),
        ),
        operator_summary_briefing_fixture(
            "scoped-production-durable-memory-write-dry-run-result-receipt-operator-summary-briefing-authority-memory-provider-external-request",
            "blocked_authority_noop",
            "operator_summary_briefing_authority_memory_provider_external_denied",
            false,
            serde_json::json!({
                "operator_summary_requested": true,
                "operator_briefing_requested": true,
                "authority_promotion_requested": true,
                "dry_run_execution_requested": true,
                "production_write_requested": true,
                "memory_write_summary_requested": true,
                "rollback_summary_requested": true,
                "secret_material_summary_requested": true,
                "provider_prompt_summary_requested": true,
                "external_send_summary_requested": true,
                "public_claim_summary_requested": true,
                "release_artifact_summary_requested": true,
                "install_summary_requested": true,
                "service_restart_summary_requested": true,
                "active_binary_summary_requested": true
            }),
        ),
    ]);
    let accepted_fixture_count = fixtures
        .as_array()
        .map(|items| {
            items
                .iter()
                .filter(|item| {
                    item.get("scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_accepted")
                        .and_then(serde_json::Value::as_bool)
                        == Some(true)
                })
                .count()
        })
        .unwrap_or(0);
    let blocked_fixture_count = fixtures
        .as_array()
        .map(std::vec::Vec::len)
        .unwrap_or(0)
        .saturating_sub(accepted_fixture_count);

    let source_report_sha256 = sha256_json_value(&source);
    let fixtures_hash_sha256 = sha256_json_value(&fixtures);
    let source_export_query_observability_boundary_hash_sha256 = json_str(
        &source,
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_boundary_hash_sha256",
    );
    let source_export_query_observability_policy_hash_sha256 = json_str(
        &source,
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_policy_hash_sha256",
    );
    let source_export_query_observability_result_hash_sha256 = json_str(
        &source,
        "dry_run_execution_result_receipt_export_query_observability_result_hash_sha256",
    );
    let source_export_query_observability_handoff_hash_sha256 = json_str(
        &source,
        "dry_run_execution_result_receipt_export_query_observability_handoff_hash_sha256",
    );
    let summary_denial_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-operator-summary-denial:v1:source={source_export_query_observability_result_hash_sha256}:record=false:persist=false:materialize=false:deliver=false:authority=false"
    ));
    let briefing_denial_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-operator-briefing-denial:v1:summary={summary_denial_hash_sha256}:record=false:persist=false:materialize=false:deliver=false:authority=false"
    ));
    let readout_ack_denial_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-operator-readout-ack-denial:v1:briefing={briefing_denial_hash_sha256}:readout=false:handoff=false:ack=false:decision=false:status=false:authority=false"
    ));
    let operator_summary_briefing_handoff_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-operator-summary-briefing-handoff:v1:readout_ack={readout_ack_denial_hash_sha256}:next=final-operator-acknowledgement-non-acceptance-denial-boundary"
    ));
    let operator_summary_briefing_result_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-operator-summary-briefing-result:v1:summary={summary_denial_hash_sha256}:briefing={briefing_denial_hash_sha256}:readout_ack={readout_ack_denial_hash_sha256}:handoff={operator_summary_briefing_handoff_hash_sha256}:accepted=true:persist=false:delivery=false:authority=false:execution=false:production-write=false"
    ));
    let operator_summary_briefing_boundary_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-operator-summary-briefing-non-persistence-denial-boundary:v1:source={source_report_sha256}:fixtures={fixtures_hash_sha256}:result={operator_summary_briefing_result_hash_sha256}:accepted=1:blocked=9:summary=false:briefing=false:ack=false:authority=false:execution=false:production-write=false"
    ));
    let operator_summary_briefing_policy_hash_sha256 = sha256_text_value(
        "scoped-production-durable-memory-write-dry-run-result-receipt-operator-summary-briefing-non-persistence-denial-policy:v1:bind-source-export-query-observability-no-summary-recording-no-briefing-recording-no-materialization-no-filesystem-no-channel-no-readout-no-handoff-no-final-ack-no-decision-status-no-authority-no-execution-no-production-write-no-kg-no-provider-no-release-no-install",
    );
    let operator_summary_briefing_matrix_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-operator-summary-briefing-denial-matrix:v1:summary={summary_denial_hash_sha256}:briefing={briefing_denial_hash_sha256}:readout_ack={readout_ack_denial_hash_sha256}:fixtures={fixtures_hash_sha256}"
    ));
    let report_ready = route_matrix.ready
        && route_count_source_command_accepted
        && source_ready
        && OPERATOR_SUMMARY_BRIEFING_SURFACES.len() == 14
        && fixtures.as_array().map(std::vec::Vec::len) == Some(10)
        && accepted_fixture_count == 1
        && blocked_fixture_count == 9
        && DENIED_BY.len() >= 60;

    let mut side_effects = serde_json::Map::new();
    for &key in FALSE_KEYS {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    side_effects.insert(
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_performed".to_string(),
        serde_json::json!(report_ready),
    );
    side_effects.insert(
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_result_accepted".to_string(),
        serde_json::json!(report_ready),
    );

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
        "hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_route"
    );
    insert_report_json!(
        "endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_PRODUCTION_DURABLE_MEMORY_WRITE_DRY_RUN_EXECUTION_RESULT_RECEIPT_OPERATOR_FACING_SUMMARY_BRIEFING_NON_PERSISTENCE_DENIAL_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_command",
        "/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-operator-facing-summary-briefing-non-persistence-denial-boundary --json"
    );
    insert_report_json!("native_route", true);
    insert_report_json!("side_effect_free", true);
    insert_report_json!("audit_date", "2026-07-05");
    insert_report_json!(
        "memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_schema_version",
        "memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_v1"
    );
    insert_report_json!(
        "memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_ready",
        report_ready
    );
    insert_report_json!(
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready",
        report_ready
    );
    insert_report_json!(
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_accepted",
        report_ready
    );
    insert_report_json!(
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_mode",
        "dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_no_summary_no_briefing_no_delivery_no_ack_no_authority_no_execution_no_production_durable_memory_mutation"
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
    insert_report_json!(
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_boundary_ready",
        source_ready
    );
    insert_report_json!(
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_boundary_accepted_count",
        if source_ready { 1 } else { 0 }
    );
    insert_report_json!(
        "source_accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_fixture_count",
        json_u64(
            &source,
            "accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_fixture_count"
        )
    );
    insert_report_json!(
        "source_blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_fixture_count",
        json_u64(
            &source,
            "blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_fixture_count"
        )
    );
    insert_report_json!(
        "source_denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_boundary_count",
        json_u64(
            &source,
            "denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_boundary_count"
        )
    );
    insert_report_json!(
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_boundary_report_sha256",
        source_report_sha256
    );
    insert_report_json!(
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_boundary_hash_sha256",
        source_export_query_observability_boundary_hash_sha256
    );
    insert_report_json!(
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_policy_hash_sha256",
        source_export_query_observability_policy_hash_sha256
    );
    insert_report_json!(
        "source_dry_run_execution_result_receipt_export_query_observability_result_hash_sha256",
        source_export_query_observability_result_hash_sha256
    );
    insert_report_json!(
        "source_dry_run_execution_result_receipt_export_query_observability_handoff_hash_sha256",
        source_export_query_observability_handoff_hash_sha256
    );
    insert_report_json!(
        "dry_run_execution_result_receipt_operator_summary_denial_hash_sha256",
        summary_denial_hash_sha256
    );
    insert_report_json!(
        "dry_run_execution_result_receipt_operator_briefing_denial_hash_sha256",
        briefing_denial_hash_sha256
    );
    insert_report_json!(
        "dry_run_execution_result_receipt_operator_readout_ack_denial_hash_sha256",
        readout_ack_denial_hash_sha256
    );
    insert_report_json!(
        "dry_run_execution_result_receipt_operator_summary_briefing_denial_matrix_hash_sha256",
        operator_summary_briefing_matrix_hash_sha256
    );
    insert_report_json!(
        "dry_run_execution_result_receipt_operator_summary_briefing_handoff_hash_sha256",
        operator_summary_briefing_handoff_hash_sha256
    );
    insert_report_json!(
        "dry_run_execution_result_receipt_operator_summary_briefing_result_hash_sha256",
        operator_summary_briefing_result_hash_sha256
    );
    insert_report_json!(
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_hash_sha256",
        operator_summary_briefing_boundary_hash_sha256
    );
    insert_report_json!(
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_policy_hash_sha256",
        operator_summary_briefing_policy_hash_sha256
    );
    insert_report_json!(
        "required_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_surface_count",
        OPERATOR_SUMMARY_BRIEFING_SURFACES.len()
    );
    insert_report_json!(
        "ready_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_surface_count",
        if report_ready {
            OPERATOR_SUMMARY_BRIEFING_SURFACES.len()
        } else {
            0
        }
    );
    insert_report_json!(
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_surfaces",
        OPERATOR_SUMMARY_BRIEFING_SURFACES
    );
    insert_report_json!(
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_fixture_count",
        fixtures.as_array().map(std::vec::Vec::len).unwrap_or(0)
    );
    insert_report_json!(
        "accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_fixture_count",
        accepted_fixture_count
    );
    insert_report_json!(
        "blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_fixture_count",
        blocked_fixture_count
    );
    insert_report_json!(
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_fixtures",
        fixtures
    );
    insert_report_json!(
        "denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary",
        DENIED_BY
    );
    insert_report_json!(
        "denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_count",
        DENIED_BY.len()
    );
    insert_report_json!(
        "allowed_next_actions",
        serde_json::json!([
            {
                "action": "run_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_require_live_gate",
                "status": "allowed_verification_only",
                "records_operator_summary": false,
                "persists_operator_summary": false,
                "materializes_operator_summary": false,
                "writes_operator_summary_filesystem": false,
                "records_operator_briefing": false,
                "persists_operator_briefing": false,
                "materializes_operator_briefing": false,
                "writes_operator_briefing_filesystem": false,
                "records_operator_readout": false,
                "records_final_acknowledgement": false,
                "records_terminal_decision": false,
                "delivers_notification": false,
                "sends_telegram": false,
                "promotes_authority": false,
                "executes_dry_run": false,
                "writes_production_durable_memory": false,
                "writes_memory_store": false,
                "writes_wal": false,
                "persists_receipt": false
            },
            {
                "action": "prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary",
                "status": "requires_separate_result_receipt_final_operator_acknowledgement_denial_gate",
                "requires_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary": true,
                "accepts_operator_acknowledgement": false,
                "persists_acknowledgement": false,
                "records_terminal_decision": false,
                "promotes_authority": false,
                "executes_dry_run": false,
                "writes_production_durable_memory": false,
                "persists_dry_run_result_receipt": false
            }
        ])
    );

    for &key in FALSE_KEYS {
        report.insert(key.to_string(), serde_json::json!(false));
        report.insert(format!("{key}_count"), serde_json::json!(0));
    }
    for &key in TRUE_KEYS {
        report.insert(key.to_string(), serde_json::json!(report_ready));
        report.insert(
            format!("{key}_count"),
            serde_json::json!(if report_ready { 1 } else { 0 }),
        );
    }
    for key in [
        "source_dry_run_execution_result_receipt_export_query_observability_denial_boundary_bound",
        "approved_production_namespace_bound",
        "approved_production_store_bound",
        "approved_production_scope_bound",
        "production_durable_memory_target_bound",
        "dry_run_execution_result_receipt_operator_summary_denial_bound",
        "dry_run_execution_result_receipt_operator_briefing_denial_bound",
        "dry_run_execution_result_receipt_operator_readout_ack_denial_bound",
        "dry_run_execution_result_receipt_operator_facing_summary_briefing_request_denied",
        "dry_run_execution_result_receipt_operator_summary_briefing_materialization_denied",
        "dry_run_execution_result_receipt_operator_summary_briefing_persistence_denied",
        "dry_run_execution_result_receipt_operator_summary_briefing_delivery_denied",
        "dry_run_execution_result_receipt_operator_readout_handoff_denied",
        "dry_run_execution_result_receipt_final_acknowledgement_decision_status_denied",
        "dry_run_execution_result_receipt_operator_summary_briefing_authority_denied",
        "dry_run_execution_result_receipt_operator_summary_briefing_handoff_bound",
        "dry_run_execution_result_receipt_operator_summary_briefing_persistence_forbidden",
        "dry_run_execution_execution_forbidden_on_operator_summary_briefing_route",
        "dry_run_execution_result_receipt_persistence_forbidden_on_operator_summary_briefing_route",
        "production_write_execution_forbidden_on_operator_summary_briefing_route",
        "production_durable_memory_write_forbidden",
        "memory_store_mutation_forbidden",
        "wal_write_forbidden_on_operator_summary_briefing_route",
        "receipt_persist_forbidden_on_operator_summary_briefing_route",
        "rollback_execution_forbidden_on_operator_summary_briefing_route",
        "tombstone_write_forbidden_on_operator_summary_briefing_route",
        "kg_live_write_forbidden",
        "provider_model_invocation_forbidden",
        "credential_channel_public_release_forbidden",
        "install_restart_active_binary_mutation_forbidden",
    ] {
        report.insert(key.to_string(), serde_json::json!(report_ready));
    }
    report.insert(
        "side_effects".to_string(),
        serde_json::Value::Object(side_effects),
    );
    serde_json::Value::Object(report)
}
