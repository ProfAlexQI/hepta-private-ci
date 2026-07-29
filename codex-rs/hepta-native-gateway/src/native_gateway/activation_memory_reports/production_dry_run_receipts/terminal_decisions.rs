fn hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_report()
-> serde_json::Value {
    const FINAL_OPERATOR_ACKNOWLEDGEMENT_SURFACES: &[&str] = &[
        "source_operator_summary_briefing_denial_boundary_required",
        "source_operator_summary_briefing_result_required",
        "final_operator_acknowledgement_request_denied",
        "final_operator_acknowledgement_acceptance_denied",
        "final_operator_acknowledgement_recording_denied",
        "final_operator_acknowledgement_persistence_materialization_denied",
        "final_operator_acknowledgement_readback_receipt_denied",
        "final_operator_acknowledgement_delivery_denied",
        "terminal_operator_decision_recording_denied",
        "terminal_operator_status_recording_denied",
        "final_operator_acknowledgement_authority_promotion_denied",
        "dry_run_execution_production_write_and_receipt_persistence_forbidden_on_final_acknowledgement_route",
        "kg_provider_channel_release_install_active_binary_acknowledgement_denied",
        "final_operator_acknowledgement_non_acceptance_handoff_bound",
    ];
    const DENIED_BY: &[&str] = &[
        "source_operator_summary_briefing_denial_boundary_required",
        "source_operator_summary_briefing_result_hash_required",
        "source_operator_summary_briefing_policy_hash_required",
        "source_operator_summary_briefing_handoff_hash_required",
        "approved_production_namespace_required",
        "approved_production_store_required",
        "approved_production_scope_required",
        "production_durable_memory_target_required",
        "final_operator_acknowledgement_request_acceptance_denied",
        "final_operator_acknowledgement_recording_denied",
        "final_operator_acknowledgement_acceptance_denied",
        "final_operator_acknowledgement_persistence_denied",
        "final_operator_acknowledgement_materialization_denied",
        "final_operator_acknowledgement_filesystem_write_denied",
        "final_operator_acknowledgement_delivery_denied",
        "final_operator_acknowledgement_channel_delivery_denied",
        "final_operator_acknowledgement_readback_recording_denied",
        "final_operator_acknowledgement_readback_persistence_denied",
        "final_operator_acknowledgement_receipt_recording_denied",
        "final_operator_acknowledgement_receipt_persistence_denied",
        "terminal_operator_decision_recording_denied",
        "terminal_operator_decision_persistence_denied",
        "terminal_operator_decision_acceptance_denied",
        "terminal_operator_decision_promotion_denied",
        "terminal_operator_status_recording_denied",
        "terminal_operator_status_persistence_denied",
        "terminal_operator_status_acceptance_denied",
        "terminal_operator_status_promotion_denied",
        "result_receipt_operator_summary_authority_promotion_denied",
        "result_receipt_operator_briefing_authority_promotion_denied",
        "result_receipt_operator_readout_authority_promotion_denied",
        "result_receipt_operator_handoff_authority_promotion_denied",
        "result_receipt_final_acknowledgement_authority_promotion_denied",
        "result_receipt_terminal_decision_authority_promotion_denied",
        "dry_run_execution_execution_denied",
        "dry_run_execution_result_persistence_denied",
        "dry_run_execution_result_receipt_persistence_denied",
        "operator_summary_briefing_state_mutation_denied",
        "export_query_observability_state_mutation_denied",
        "retention_expiry_garbage_collection_state_mutation_denied",
        "audit_evidence_state_mutation_denied",
        "cancellation_supersession_state_mutation_denied",
        "ordering_monotonicity_state_mutation_denied",
        "replay_idempotency_state_mutation_denied",
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
        "install_authority_denied",
        "restart_authority_denied",
        "active_binary_mutation_denied",
        "unrestricted_full_live_activation_denied",
        "operator_final_acknowledgement_non_acceptance_denial_only",
        "operator_terminal_decision_public_claim_non_promotion_required_next",
    ];
    const FALSE_KEYS: &[&str] = &[
        "dry_run_execution_result_receipt_final_operator_acknowledgement_allowed",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_requested",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_request_accepted",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_recorded",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_accepted",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_persisted",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_materialized",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_filesystem_written",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_delivered",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_channel_delivery_performed",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_readback_recorded",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_readback_persisted",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_receipt_recorded",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_receipt_persisted",
        "dry_run_execution_result_receipt_operator_final_acceptance_recorded",
        "dry_run_execution_result_receipt_operator_final_acceptance_persisted",
        "dry_run_execution_result_receipt_operator_final_acceptance_materialized",
        "dry_run_execution_result_receipt_terminal_operator_decision_recorded",
        "dry_run_execution_result_receipt_terminal_operator_decision_persisted",
        "dry_run_execution_result_receipt_terminal_operator_decision_accepted",
        "dry_run_execution_result_receipt_terminal_operator_decision_promoted",
        "dry_run_execution_result_receipt_terminal_operator_status_recorded",
        "dry_run_execution_result_receipt_terminal_operator_status_persisted",
        "dry_run_execution_result_receipt_terminal_operator_status_accepted",
        "dry_run_execution_result_receipt_terminal_operator_status_promoted",
        "dry_run_execution_result_receipt_authority_promoted_from_operator_summary",
        "dry_run_execution_result_receipt_authority_promoted_from_operator_briefing",
        "dry_run_execution_result_receipt_authority_promoted_from_operator_readout",
        "dry_run_execution_result_receipt_authority_promoted_from_operator_handoff",
        "dry_run_execution_result_receipt_authority_promoted_from_final_acknowledgement",
        "dry_run_execution_result_receipt_authority_promoted_from_terminal_decision",
        "dry_run_execution_result_receipt_operator_facing_summary_recorded",
        "dry_run_execution_result_receipt_operator_facing_summary_persisted",
        "dry_run_execution_result_receipt_operator_facing_summary_materialized",
        "dry_run_execution_result_receipt_operator_facing_summary_filesystem_written",
        "dry_run_execution_result_receipt_operator_facing_summary_delivered",
        "dry_run_execution_result_receipt_operator_briefing_recorded",
        "dry_run_execution_result_receipt_operator_briefing_persisted",
        "dry_run_execution_result_receipt_operator_briefing_materialized",
        "dry_run_execution_result_receipt_operator_briefing_filesystem_written",
        "dry_run_execution_result_receipt_operator_briefing_delivered",
        "dry_run_execution_result_receipt_operator_readout_recorded",
        "dry_run_execution_result_receipt_operator_handoff_recorded",
        "dry_run_execution_result_receipt_export_recorded",
        "dry_run_execution_result_receipt_export_persisted",
        "dry_run_execution_result_receipt_export_file_written",
        "dry_run_execution_result_receipt_query_registered",
        "dry_run_execution_result_receipt_query_executed",
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
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_performed",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_result_recorded",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_result_accepted",
        "source_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_bound",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_request_denied",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_bound",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_readback_denied",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_receipt_denied",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_delivery_denied",
        "dry_run_execution_result_receipt_terminal_operator_decision_status_denied",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_authority_denied",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_handoff_bound",
    ];

    fn final_operator_acknowledgement_fixture(
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
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_status".to_string(),
            serde_json::json!(status),
        );
        base.insert(
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_accepted".to_string(),
            serde_json::json!(accepted),
        );
        base.insert("reason".to_string(), serde_json::json!(reason));
        base.insert(
            "source_operator_summary_briefing_present".to_string(),
            serde_json::json!(true),
        );
        base.insert(
            "source_operator_summary_briefing_ready".to_string(),
            serde_json::json!(true),
        );
        base.insert(
            "final_acknowledgement_noop_confirmed".to_string(),
            serde_json::json!(true),
        );
        for key in [
            "final_operator_acknowledgement_requested",
            "acknowledgement_acceptance_requested",
            "acknowledgement_recording_requested",
            "acknowledgement_persistence_requested",
            "acknowledgement_materialization_requested",
            "acknowledgement_filesystem_write_requested",
            "acknowledgement_readback_requested",
            "acknowledgement_receipt_requested",
            "acknowledgement_delivery_requested",
            "telegram_send_requested",
            "channel_delivery_requested",
            "terminal_operator_decision_requested",
            "terminal_operator_status_requested",
            "terminal_operator_decision_promotion_requested",
            "terminal_operator_status_promotion_requested",
            "authority_promotion_requested",
            "dry_run_execution_requested",
            "production_write_requested",
            "memory_write_acknowledgement_requested",
            "rollback_acknowledgement_requested",
            "kg_write_acknowledgement_requested",
            "provider_prompt_acknowledgement_requested",
            "credential_acknowledgement_requested",
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
            "acknowledgement_readback_recorded",
            "acknowledgement_readback_persisted",
            "acknowledgement_receipt_recorded",
            "acknowledgement_receipt_persisted",
            "terminal_operator_decision_recorded",
            "terminal_operator_decision_persisted",
            "terminal_operator_decision_accepted",
            "terminal_operator_decision_promoted",
            "terminal_operator_status_recorded",
            "terminal_operator_status_persisted",
            "terminal_operator_status_accepted",
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
            "live_kg_write_performed",
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
        .name("hepta-memory-production-durable-dry-run-result-receipt-final-operator-ack-source-report".to_string())
        .stack_size(8 * 1024 * 1024)
        .spawn(
            hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_report,
        )
        .ok()
        .and_then(|handle| handle.join().ok())
        .unwrap_or_else(|| {
            serde_json::json!({
                "status": "blocked",
                "memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_ready": false,
                "scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_accepted": false,
                "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_source_report_thread_failed": true
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
    let source_next_action_final_ack = source
        .get("allowed_next_actions")
        .and_then(serde_json::Value::as_array)
        .and_then(|items| items.get(1))
        .map(|item| {
            item.get("action").and_then(serde_json::Value::as_str)
                == Some("prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary")
                && item
                    .get("accepts_operator_acknowledgement")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && item
                    .get("persists_acknowledgement")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && item
                    .get("records_terminal_decision")
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
                .get("scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_result_accepted")
                .and_then(serde_json::Value::as_bool)
                == Some(true)
                && effects
                    .get("dry_run_execution_result_receipt_operator_facing_summary_recorded")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("dry_run_execution_result_receipt_operator_briefing_recorded")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("dry_run_execution_result_receipt_final_operator_acknowledgement_recorded")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("dry_run_execution_result_receipt_authority_promoted_from_final_acknowledgement")
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
            "memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_ready",
        )
        && json_bool(
            &source,
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_accepted",
        )
        && json_u64(
            &source,
            "accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_fixture_count",
        ) == 1
        && json_u64(
            &source,
            "blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_fixture_count",
        ) == 9
        && json_u64(
            &source,
            "denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_count",
        ) >= 60
        && !json_bool(
            &source,
            "dry_run_execution_result_receipt_operator_facing_summary_recorded",
        )
        && !json_bool(
            &source,
            "dry_run_execution_result_receipt_operator_briefing_recorded",
        )
        && !json_bool(
            &source,
            "dry_run_execution_result_receipt_operator_readout_recorded",
        )
        && !json_bool(
            &source,
            "dry_run_execution_result_receipt_operator_handoff_recorded",
        )
        && !json_bool(
            &source,
            "dry_run_execution_result_receipt_final_operator_acknowledgement_recorded",
        )
        && !json_bool(
            &source,
            "dry_run_execution_result_receipt_terminal_operator_decision_recorded",
        )
        && !json_bool(
            &source,
            "dry_run_execution_result_receipt_terminal_operator_status_recorded",
        )
        && !json_bool(
            &source,
            "dry_run_execution_result_receipt_authority_promoted_from_final_acknowledgement",
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
        && source_next_action_final_ack
        && source_side_effects_ok;

    let fixtures = serde_json::Value::Array(vec![
        final_operator_acknowledgement_fixture(
            "scoped-production-durable-memory-write-dry-run-result-receipt-final-operator-ack-non-acceptance-report-only-denial",
            "accepted_final_operator_acknowledgement_non_acceptance_denial",
            "source_operator_summary_briefing_denial_bound_without_final_acknowledgement_acceptance_terminal_decision_authority_execution_or_production_write",
            true,
            serde_json::json!({}),
        ),
        final_operator_acknowledgement_fixture(
            "scoped-production-durable-memory-write-dry-run-result-receipt-final-operator-ack-missing-source",
            "blocked_noop",
            "source_operator_summary_briefing_denial_boundary_required",
            false,
            serde_json::json!({
                "source_operator_summary_briefing_present": false,
                "source_operator_summary_briefing_ready": false,
                "final_operator_acknowledgement_requested": true
            }),
        ),
        final_operator_acknowledgement_fixture(
            "scoped-production-durable-memory-write-dry-run-result-receipt-final-operator-ack-request",
            "blocked_ack_noop",
            "final_operator_acknowledgement_request_shape_denied",
            false,
            serde_json::json!({"final_operator_acknowledgement_requested": true}),
        ),
        final_operator_acknowledgement_fixture(
            "scoped-production-durable-memory-write-dry-run-result-receipt-final-operator-ack-acceptance-request",
            "blocked_acceptance_noop",
            "final_operator_acknowledgement_acceptance_denied",
            false,
            serde_json::json!({
                "final_operator_acknowledgement_requested": true,
                "acknowledgement_acceptance_requested": true
            }),
        ),
        final_operator_acknowledgement_fixture(
            "scoped-production-durable-memory-write-dry-run-result-receipt-final-operator-ack-recording-request",
            "blocked_recording_noop",
            "final_operator_acknowledgement_recording_denied",
            false,
            serde_json::json!({
                "final_operator_acknowledgement_requested": true,
                "acknowledgement_recording_requested": true
            }),
        ),
        final_operator_acknowledgement_fixture(
            "scoped-production-durable-memory-write-dry-run-result-receipt-final-operator-ack-persistence-materialization-request",
            "blocked_persistence_noop",
            "final_operator_acknowledgement_persistence_materialization_denied",
            false,
            serde_json::json!({
                "final_operator_acknowledgement_requested": true,
                "acknowledgement_persistence_requested": true,
                "acknowledgement_materialization_requested": true,
                "acknowledgement_filesystem_write_requested": true
            }),
        ),
        final_operator_acknowledgement_fixture(
            "scoped-production-durable-memory-write-dry-run-result-receipt-final-operator-ack-readback-receipt-request",
            "blocked_readback_receipt_noop",
            "final_operator_acknowledgement_readback_receipt_denied",
            false,
            serde_json::json!({
                "final_operator_acknowledgement_requested": true,
                "acknowledgement_readback_requested": true,
                "acknowledgement_receipt_requested": true
            }),
        ),
        final_operator_acknowledgement_fixture(
            "scoped-production-durable-memory-write-dry-run-result-receipt-final-operator-ack-delivery-request",
            "blocked_delivery_noop",
            "final_operator_acknowledgement_delivery_denied",
            false,
            serde_json::json!({
                "final_operator_acknowledgement_requested": true,
                "acknowledgement_delivery_requested": true,
                "telegram_send_requested": true,
                "channel_delivery_requested": true
            }),
        ),
        final_operator_acknowledgement_fixture(
            "scoped-production-durable-memory-write-dry-run-result-receipt-terminal-operator-decision-status-request",
            "blocked_terminal_decision_status_noop",
            "terminal_operator_decision_status_denied",
            false,
            serde_json::json!({
                "final_operator_acknowledgement_requested": true,
                "terminal_operator_decision_requested": true,
                "terminal_operator_status_requested": true,
                "terminal_operator_decision_promotion_requested": true,
                "terminal_operator_status_promotion_requested": true
            }),
        ),
        final_operator_acknowledgement_fixture(
            "scoped-production-durable-memory-write-dry-run-result-receipt-final-operator-ack-authority-memory-provider-external-request",
            "blocked_authority_noop",
            "final_operator_acknowledgement_authority_memory_provider_external_denied",
            false,
            serde_json::json!({
                "final_operator_acknowledgement_requested": true,
                "authority_promotion_requested": true,
                "dry_run_execution_requested": true,
                "production_write_requested": true,
                "memory_write_acknowledgement_requested": true,
                "rollback_acknowledgement_requested": true,
                "kg_write_acknowledgement_requested": true,
                "provider_prompt_acknowledgement_requested": true,
                "credential_acknowledgement_requested": true,
                "external_send_acknowledgement_requested": true,
                "public_claim_acknowledgement_requested": true,
                "release_artifact_acknowledgement_requested": true,
                "install_acknowledgement_requested": true,
                "service_restart_acknowledgement_requested": true,
                "active_binary_acknowledgement_requested": true
            }),
        ),
    ]);
    let accepted_fixture_count = fixtures
        .as_array()
        .map(|items| {
            items
                .iter()
                .filter(|item| {
                    item.get("scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_accepted")
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
    let source_operator_summary_briefing_boundary_hash_sha256 = json_str(
        &source,
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_hash_sha256",
    );
    let source_operator_summary_briefing_policy_hash_sha256 = json_str(
        &source,
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_policy_hash_sha256",
    );
    let source_operator_summary_briefing_result_hash_sha256 = json_str(
        &source,
        "dry_run_execution_result_receipt_operator_summary_briefing_result_hash_sha256",
    );
    let source_operator_summary_briefing_handoff_hash_sha256 = json_str(
        &source,
        "dry_run_execution_result_receipt_operator_summary_briefing_handoff_hash_sha256",
    );
    let final_acknowledgement_denial_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-final-operator-acknowledgement-denial:v1:source={source_operator_summary_briefing_result_hash_sha256}:request=false:accept=false:record=false:persist=false:materialize=false:deliver=false"
    ));
    let final_acknowledgement_readback_denial_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-final-operator-acknowledgement-readback-denial:v1:ack={final_acknowledgement_denial_hash_sha256}:readback=false:receipt=false:persist=false"
    ));
    let final_acknowledgement_receipt_denial_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-final-operator-acknowledgement-receipt-denial:v1:readback={final_acknowledgement_readback_denial_hash_sha256}:receipt-record=false:receipt-persist=false"
    ));
    let terminal_decision_status_denial_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-terminal-operator-decision-status-denial:v1:ack={final_acknowledgement_denial_hash_sha256}:decision=false:status=false:promotion=false"
    ));
    let final_acknowledgement_matrix_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-final-operator-acknowledgement-non-acceptance-matrix:v1:ack={final_acknowledgement_denial_hash_sha256}:readback={final_acknowledgement_readback_denial_hash_sha256}:receipt={final_acknowledgement_receipt_denial_hash_sha256}:terminal={terminal_decision_status_denial_hash_sha256}:fixtures={fixtures_hash_sha256}"
    ));
    let final_acknowledgement_handoff_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-final-operator-acknowledgement-non-acceptance-handoff:v1:matrix={final_acknowledgement_matrix_hash_sha256}:next=terminal-operator-decision-public-claim-non-promotion-denial-boundary"
    ));
    let final_acknowledgement_result_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-final-operator-acknowledgement-non-acceptance-result:v1:ack={final_acknowledgement_denial_hash_sha256}:terminal={terminal_decision_status_denial_hash_sha256}:handoff={final_acknowledgement_handoff_hash_sha256}:accepted=true:persist=false:delivery=false:authority=false:execution=false:production-write=false"
    ));
    let final_acknowledgement_boundary_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-final-operator-acknowledgement-non-acceptance-denial-boundary:v1:source={source_report_sha256}:fixtures={fixtures_hash_sha256}:result={final_acknowledgement_result_hash_sha256}:accepted=1:blocked=9:ack=false:terminal=false:authority=false:execution=false:production-write=false"
    ));
    let final_acknowledgement_policy_hash_sha256 = sha256_text_value(
        "scoped-production-durable-memory-write-dry-run-result-receipt-final-operator-acknowledgement-non-acceptance-denial-policy:v1:bind-source-operator-summary-briefing-no-final-ack-request-acceptance-no-recording-no-persistence-no-materialization-no-filesystem-no-channel-no-readback-receipt-no-terminal-decision-status-no-authority-no-execution-no-production-write-no-kg-no-provider-no-release-no-install",
    );
    let report_ready = route_matrix.ready
        && route_count_source_command_accepted
        && source_ready
        && FINAL_OPERATOR_ACKNOWLEDGEMENT_SURFACES.len() == 14
        && fixtures.as_array().map(std::vec::Vec::len) == Some(10)
        && accepted_fixture_count == 1
        && blocked_fixture_count == 9
        && DENIED_BY.len() >= 64;

    let mut side_effects = serde_json::Map::new();
    for &key in FALSE_KEYS {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    side_effects.insert(
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_performed".to_string(),
        serde_json::json!(report_ready),
    );
    side_effects.insert(
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_result_accepted".to_string(),
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
        "hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_route"
    );
    insert_report_json!(
        "endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_PRODUCTION_DURABLE_MEMORY_WRITE_DRY_RUN_EXECUTION_RESULT_RECEIPT_FINAL_OPERATOR_ACKNOWLEDGEMENT_NON_ACCEPTANCE_DENIAL_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_command",
        "/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-final-operator-acknowledgement-non-acceptance-denial-boundary --json"
    );
    insert_report_json!("native_route", true);
    insert_report_json!("side_effect_free", true);
    insert_report_json!("audit_date", "2026-07-05");
    insert_report_json!(
        "memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_schema_version",
        "memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_v1"
    );
    insert_report_json!(
        "memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_ready",
        report_ready
    );
    insert_report_json!(
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready",
        report_ready
    );
    insert_report_json!(
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_accepted",
        report_ready
    );
    insert_report_json!(
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_mode",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_no_ack_acceptance_no_terminal_decision_no_authority_no_execution_no_production_durable_memory_mutation"
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
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_ready",
        source_ready
    );
    insert_report_json!(
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_accepted_count",
        if source_ready { 1 } else { 0 }
    );
    insert_report_json!(
        "source_accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_fixture_count",
        json_u64(
            &source,
            "accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_fixture_count"
        )
    );
    insert_report_json!(
        "source_blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_fixture_count",
        json_u64(
            &source,
            "blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_fixture_count"
        )
    );
    insert_report_json!(
        "source_denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_count",
        json_u64(
            &source,
            "denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_count"
        )
    );
    insert_report_json!(
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_report_sha256",
        source_report_sha256
    );
    insert_report_json!(
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_hash_sha256",
        source_operator_summary_briefing_boundary_hash_sha256
    );
    insert_report_json!(
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_policy_hash_sha256",
        source_operator_summary_briefing_policy_hash_sha256
    );
    insert_report_json!(
        "source_dry_run_execution_result_receipt_operator_summary_briefing_result_hash_sha256",
        source_operator_summary_briefing_result_hash_sha256
    );
    insert_report_json!(
        "source_dry_run_execution_result_receipt_operator_summary_briefing_handoff_hash_sha256",
        source_operator_summary_briefing_handoff_hash_sha256
    );
    insert_report_json!(
        "dry_run_execution_result_receipt_final_operator_acknowledgement_denial_hash_sha256",
        final_acknowledgement_denial_hash_sha256
    );
    insert_report_json!(
        "dry_run_execution_result_receipt_final_operator_acknowledgement_readback_denial_hash_sha256",
        final_acknowledgement_readback_denial_hash_sha256
    );
    insert_report_json!(
        "dry_run_execution_result_receipt_final_operator_acknowledgement_receipt_denial_hash_sha256",
        final_acknowledgement_receipt_denial_hash_sha256
    );
    insert_report_json!(
        "dry_run_execution_result_receipt_terminal_operator_decision_status_denial_hash_sha256",
        terminal_decision_status_denial_hash_sha256
    );
    insert_report_json!(
        "dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_matrix_hash_sha256",
        final_acknowledgement_matrix_hash_sha256
    );
    insert_report_json!(
        "dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_handoff_hash_sha256",
        final_acknowledgement_handoff_hash_sha256
    );
    insert_report_json!(
        "dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_result_hash_sha256",
        final_acknowledgement_result_hash_sha256
    );
    insert_report_json!(
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_hash_sha256",
        final_acknowledgement_boundary_hash_sha256
    );
    insert_report_json!(
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_policy_hash_sha256",
        final_acknowledgement_policy_hash_sha256
    );
    insert_report_json!(
        "required_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_surface_count",
        FINAL_OPERATOR_ACKNOWLEDGEMENT_SURFACES.len()
    );
    insert_report_json!(
        "ready_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_surface_count",
        if report_ready {
            FINAL_OPERATOR_ACKNOWLEDGEMENT_SURFACES.len()
        } else {
            0
        }
    );
    insert_report_json!(
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_surfaces",
        FINAL_OPERATOR_ACKNOWLEDGEMENT_SURFACES
    );
    insert_report_json!(
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_fixture_count",
        fixtures.as_array().map(std::vec::Vec::len).unwrap_or(0)
    );
    insert_report_json!(
        "accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_fixture_count",
        accepted_fixture_count
    );
    insert_report_json!(
        "blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_fixture_count",
        blocked_fixture_count
    );
    insert_report_json!(
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_fixtures",
        fixtures
    );
    insert_report_json!(
        "denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary",
        DENIED_BY
    );
    insert_report_json!(
        "denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_count",
        DENIED_BY.len()
    );
    insert_report_json!(
        "allowed_next_actions",
        serde_json::json!([
            {
                "action": "run_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_require_live_gate",
                "status": "allowed_verification_only",
                "accepts_operator_acknowledgement": false,
                "records_acknowledgement": false,
                "persists_acknowledgement": false,
                "materializes_acknowledgement": false,
                "delivers_acknowledgement": false,
                "records_terminal_decision": false,
                "records_terminal_status": false,
                "promotes_authority": false,
                "executes_dry_run": false,
                "writes_production_durable_memory": false,
                "writes_memory_store": false,
                "writes_wal": false,
                "persists_receipt": false
            },
            {
                "action": "prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary",
                "status": "requires_separate_result_receipt_terminal_operator_decision_denial_gate",
                "requires_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary": true,
                "accepts_terminal_decision": false,
                "records_terminal_status": false,
                "claims_public_release": false,
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
        "source_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_bound",
        "approved_production_namespace_bound",
        "approved_production_store_bound",
        "approved_production_scope_bound",
        "production_durable_memory_target_bound",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_denial_bound",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_readback_denial_bound",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_receipt_denial_bound",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_request_denied",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_bound",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_readback_denied",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_receipt_denied",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_delivery_denied",
        "dry_run_execution_result_receipt_terminal_operator_decision_status_denied",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_authority_denied",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_handoff_bound",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_persistence_forbidden",
        "dry_run_execution_execution_forbidden_on_final_acknowledgement_route",
        "dry_run_execution_result_receipt_persistence_forbidden_on_final_acknowledgement_route",
        "production_write_execution_forbidden_on_final_acknowledgement_route",
        "production_durable_memory_write_forbidden",
        "memory_store_mutation_forbidden",
        "wal_write_forbidden_on_final_acknowledgement_route",
        "receipt_persist_forbidden_on_final_acknowledgement_route",
        "rollback_execution_forbidden_on_final_acknowledgement_route",
        "tombstone_write_forbidden_on_final_acknowledgement_route",
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

fn hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_report()
-> serde_json::Value {
    fn terminal_operator_decision_public_claim_fixture(
        id: &str,
        status: &str,
        accepted: bool,
        reason: &str,
        extra: serde_json::Value,
    ) -> serde_json::Value {
        let mut fixture = serde_json::Map::new();
        macro_rules! insert_fixture_json {
            ($key:literal, $value:expr) => {
                fixture.insert($key.to_string(), serde_json::json!($value));
            };
        }
        insert_fixture_json!("id", id);
        insert_fixture_json!("fixture_id", id);
        insert_fixture_json!(
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_status",
            status
        );
        insert_fixture_json!(
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_accepted",
            accepted
        );
        insert_fixture_json!("source_final_acknowledgement_present", true);
        insert_fixture_json!("source_final_acknowledgement_ready", true);
        insert_fixture_json!(
            "terminal_operator_decision_public_claim_noop_confirmed",
            true
        );
        insert_fixture_json!("reason", reason);
        for key in [
            "terminal_operator_decision_requested",
            "terminal_operator_status_requested",
            "terminal_operator_decision_recording_requested",
            "terminal_operator_decision_persistence_requested",
            "terminal_operator_decision_materialization_requested",
            "terminal_operator_decision_filesystem_write_requested",
            "terminal_operator_decision_delivery_requested",
            "terminal_operator_decision_acceptance_requested",
            "terminal_operator_status_recording_requested",
            "terminal_operator_status_persistence_requested",
            "terminal_operator_status_acceptance_requested",
            "terminal_operator_decision_promotion_requested",
            "terminal_operator_status_promotion_requested",
            "public_claim_requested",
            "public_claim_promotion_requested",
            "public_ga_claim_requested",
            "public_release_claim_requested",
            "public_distribution_requested",
            "public_artifact_write_requested",
            "release_artifact_write_requested",
            "channel_delivery_requested",
            "telegram_send_requested",
            "external_send_requested",
            "dry_run_execution_requested",
            "production_write_requested",
            "memory_write_decision_requested",
            "rollback_decision_requested",
            "kg_write_decision_requested",
            "provider_prompt_decision_requested",
            "credential_decision_requested",
            "install_decision_requested",
            "service_restart_decision_requested",
            "active_binary_decision_requested",
        ] {
            fixture.insert(key.to_string(), serde_json::json!(false));
        }
        for key in [
            "terminal_operator_decision_allowed",
            "terminal_operator_decision_request_accepted",
            "terminal_operator_decision_accepted",
            "terminal_operator_decision_recorded",
            "terminal_operator_decision_persisted",
            "terminal_operator_decision_materialized",
            "terminal_operator_decision_filesystem_written",
            "terminal_operator_decision_delivered",
            "terminal_operator_decision_channel_delivery_performed",
            "terminal_operator_status_recorded",
            "terminal_operator_status_persisted",
            "terminal_operator_status_materialized",
            "terminal_operator_status_filesystem_written",
            "terminal_operator_status_accepted",
            "terminal_operator_decision_promoted",
            "terminal_operator_status_promoted",
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
            "dry_run_execution_executed",
            "production_durable_memory_store_write_performed",
            "memory_store_write_performed",
            "memory_store_mutated",
            "wal_write_performed",
            "rollback_executed",
            "live_kg_write_performed",
            "provider_invoked",
            "model_invoked",
            "credential_read",
            "install_executed",
            "service_restarted",
            "active_binary_mutated",
        ] {
            fixture.insert(key.to_string(), serde_json::json!(false));
        }
        if let Some(extra) = extra.as_object() {
            for (key, value) in extra {
                fixture.insert(key.clone(), value.clone());
            }
        }
        serde_json::Value::Object(fixture)
    }

    let route_matrix = control_ui_route_parity_report();
    let source = std::thread::Builder::new()
        .name("hepta-scoped-production-memory-final-ack-source-report".to_string())
        .stack_size(8 * 1024 * 1024)
        .spawn(hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_report)
        .ok()
        .and_then(|handle| handle.join().ok())
        .unwrap_or_else(|| {
            serde_json::json!({
                "status": "blocked",
                "memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_ready": false,
                "source_final_acknowledgement_non_acceptance_report_thread_failed": true
            })
        });

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
            .unwrap_or("")
            .to_string()
    };

    let route_count_source_command_accepted = route_matrix.ready
        && route_matrix.route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_ready = source_str("status") == "ready"
        && source_bool(
            "memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_ready",
        )
        && source_bool(
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_accepted",
        )
        && source_u64(
            "accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_fixture_count",
        ) == 1
        && source_u64(
            "blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_fixture_count",
        ) == 9
        && source_u64(
            "denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_count",
        ) >= 64
        && !source_bool("dry_run_execution_result_receipt_final_operator_acknowledgement_recorded")
        && !source_bool("dry_run_execution_result_receipt_final_operator_acknowledgement_accepted")
        && !source_bool("dry_run_execution_result_receipt_terminal_operator_decision_recorded")
        && !source_bool("dry_run_execution_result_receipt_terminal_operator_status_recorded")
        && !source_bool(
            "dry_run_execution_result_receipt_authority_promoted_from_final_acknowledgement",
        )
        && !source_bool("dry_run_execution_executed")
        && !source_bool("production_durable_memory_store_write_performed")
        && !source_bool("memory_store_write_performed")
        && !source_bool("wal_write_performed")
        && !source_bool("receipt_persisted")
        && !source_bool("live_kg_write_performed")
        && !source_bool("provider_invoked")
        && !source_bool("model_invoked")
        && !source_bool("credential_read")
        && !source_bool("channel_send_performed")
        && !source_bool("external_send_performed")
        && !source_bool("release_artifact_written")
        && !source_bool("install_executed")
        && !source_bool("service_restarted")
        && !source_bool("active_binary_mutated");

    let fixtures = serde_json::Value::Array(vec![
        terminal_operator_decision_public_claim_fixture(
            "scoped-production-durable-memory-write-dry-run-result-receipt-terminal-decision-public-claim-report-only-binding",
            "accepted_report_only",
            true,
            "terminal_operator_decision_public_claim_non_promotion_denial_matrix_bound_without_terminal_decision_or_public_claim",
            serde_json::json!({}),
        ),
        terminal_operator_decision_public_claim_fixture(
            "scoped-production-durable-memory-write-dry-run-result-receipt-terminal-decision-public-claim-missing-source-final-acknowledgement",
            "blocked_noop",
            false,
            "source_final_operator_acknowledgement_non_acceptance_report_required",
            serde_json::json!({
                "source_final_acknowledgement_present": false,
                "source_final_acknowledgement_ready": false,
                "terminal_operator_decision_requested": true
            }),
        ),
        terminal_operator_decision_public_claim_fixture(
            "scoped-production-durable-memory-write-dry-run-result-receipt-terminal-operator-decision-request",
            "blocked_decision_noop",
            false,
            "terminal_operator_decision_request_denied",
            serde_json::json!({
                "terminal_operator_decision_requested": true,
                "terminal_operator_decision_recording_requested": true,
                "terminal_operator_decision_acceptance_requested": true
            }),
        ),
        terminal_operator_decision_public_claim_fixture(
            "scoped-production-durable-memory-write-dry-run-result-receipt-terminal-operator-status-request",
            "blocked_status_noop",
            false,
            "terminal_operator_status_recording_denied",
            serde_json::json!({
                "terminal_operator_status_requested": true,
                "terminal_operator_status_recording_requested": true,
                "terminal_operator_status_acceptance_requested": true
            }),
        ),
        terminal_operator_decision_public_claim_fixture(
            "scoped-production-durable-memory-write-dry-run-result-receipt-terminal-decision-persistence-request",
            "blocked_decision_noop",
            false,
            "terminal_operator_decision_persistence_materialization_filesystem_write_denied",
            serde_json::json!({
                "terminal_operator_decision_requested": true,
                "terminal_operator_decision_persistence_requested": true,
                "terminal_operator_decision_materialization_requested": true,
                "terminal_operator_decision_filesystem_write_requested": true
            }),
        ),
        terminal_operator_decision_public_claim_fixture(
            "scoped-production-durable-memory-write-dry-run-result-receipt-terminal-decision-delivery-request",
            "blocked_delivery_noop",
            false,
            "terminal_operator_decision_delivery_denied",
            serde_json::json!({
                "terminal_operator_decision_requested": true,
                "terminal_operator_decision_delivery_requested": true,
                "telegram_send_requested": true,
                "channel_delivery_requested": true,
                "external_send_requested": true
            }),
        ),
        terminal_operator_decision_public_claim_fixture(
            "scoped-production-durable-memory-write-dry-run-result-receipt-terminal-decision-public-claim-promotion-request",
            "blocked_public_claim_noop",
            false,
            "public_claim_public_release_public_ga_promotion_denied",
            serde_json::json!({
                "terminal_operator_decision_requested": true,
                "terminal_operator_decision_promotion_requested": true,
                "terminal_operator_status_promotion_requested": true,
                "public_claim_requested": true,
                "public_claim_promotion_requested": true,
                "public_ga_claim_requested": true,
                "public_release_claim_requested": true
            }),
        ),
        terminal_operator_decision_public_claim_fixture(
            "scoped-production-durable-memory-write-dry-run-result-receipt-release-artifact-publication-request",
            "blocked_publication_noop",
            false,
            "release_artifact_public_artifact_public_distribution_denied",
            serde_json::json!({
                "terminal_operator_decision_requested": true,
                "public_distribution_requested": true,
                "public_artifact_write_requested": true,
                "release_artifact_write_requested": true
            }),
        ),
        terminal_operator_decision_public_claim_fixture(
            "scoped-production-durable-memory-write-dry-run-result-receipt-terminal-decision-production-memory-provider-request",
            "blocked_authority_noop",
            false,
            "production_memory_rollback_kg_provider_credential_authority_denied",
            serde_json::json!({
                "terminal_operator_decision_requested": true,
                "dry_run_execution_requested": true,
                "production_write_requested": true,
                "memory_write_decision_requested": true,
                "rollback_decision_requested": true,
                "kg_write_decision_requested": true,
                "provider_prompt_decision_requested": true,
                "credential_decision_requested": true
            }),
        ),
        terminal_operator_decision_public_claim_fixture(
            "scoped-production-durable-memory-write-dry-run-result-receipt-terminal-decision-install-restart-active-binary-request",
            "blocked_install_noop",
            false,
            "install_restart_active_binary_terminal_decision_denied",
            serde_json::json!({
                "terminal_operator_decision_requested": true,
                "install_decision_requested": true,
                "service_restart_decision_requested": true,
                "active_binary_decision_requested": true
            }),
        ),
    ]);

    let source_report_sha256 = sha256_json_value(&source);
    let fixtures_hash_sha256 = sha256_json_value(&fixtures);
    let terminal_decision_denial_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-terminal-operator-decision-denial:v1:source={}:decision=false:record=false:persist=false:deliver=false",
        source_str(
            "dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_result_hash_sha256"
        )
    ));
    let public_claim_denial_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-terminal-operator-public-claim-denial:v1:decision={terminal_decision_denial_hash_sha256}:claim=false:ga=false:release=false:artifact=false"
    ));
    let terminal_public_claim_matrix_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-terminal-operator-decision-public-claim-non-promotion-matrix:v1:decision={terminal_decision_denial_hash_sha256}:public={public_claim_denial_hash_sha256}:fixtures={fixtures_hash_sha256}"
    ));
    let terminal_public_claim_handoff_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-terminal-operator-decision-public-claim-non-promotion-handoff:v1:matrix={terminal_public_claim_matrix_hash_sha256}:next=release-artifact-publication-denial-boundary"
    ));
    let terminal_public_claim_result_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-terminal-operator-decision-public-claim-non-promotion-result:v1:decision={terminal_decision_denial_hash_sha256}:public={public_claim_denial_hash_sha256}:handoff={terminal_public_claim_handoff_hash_sha256}:accepted=true:terminal=false:public=false:execution=false:production-write=false"
    ));
    let terminal_public_claim_boundary_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial-boundary:v1:source={source_report_sha256}:result={terminal_public_claim_result_hash_sha256}:accepted=1:blocked=9:decision=false:public=false:authority=false:execution=false:production-write=false"
    ));
    let terminal_public_claim_policy_hash_sha256 = sha256_text_value(
        "scoped-production-durable-memory-write-dry-run-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial-policy:v1:no-terminal-decision-recording-no-terminal-status-no-public-claim-no-publication-no-release-artifact-no-authority-no-execution-no-production-write-no-kg-no-provider-no-release-no-install",
    );

    let mut denials = source
        .get("denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary")
        .and_then(serde_json::Value::as_array)
        .cloned()
        .unwrap_or_default();
    for denial in [
        "terminal_operator_decision_request_denied",
        "terminal_operator_decision_recording_denied",
        "terminal_operator_decision_persistence_denied",
        "terminal_operator_decision_materialization_denied",
        "terminal_operator_decision_filesystem_write_denied",
        "terminal_operator_decision_delivery_denied",
        "terminal_operator_status_recording_denied",
        "terminal_operator_status_persistence_denied",
        "terminal_operator_decision_promotion_denied",
        "terminal_operator_status_promotion_denied",
        "public_claim_recording_denied",
        "public_claim_promotion_denied",
        "public_ga_claim_denied",
        "public_release_publication_denied",
        "public_distribution_denied",
        "public_artifact_write_denied",
        "release_artifact_write_denied",
        "dry_run_execution_and_production_memory_write_denied_by_terminal_decision_boundary",
        "kg_provider_channel_release_install_active_binary_terminal_decision_denied",
    ] {
        denials.push(serde_json::json!(denial));
    }
    let denial_count = denials.len();

    let report_ready = source_ready
        && route_count_source_command_accepted
        && fixtures.as_array().map(Vec::len) == Some(10)
        && denial_count >= 80;

    let mut side_effects = serde_json::Map::new();
    side_effects.insert(
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_performed".to_string(),
        serde_json::json!(true),
    );
    side_effects.insert(
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_result_accepted".to_string(),
        serde_json::json!(true),
    );
    for key in [
        "dry_run_execution_result_receipt_terminal_operator_decision_requested",
        "dry_run_execution_result_receipt_terminal_operator_decision_recorded",
        "dry_run_execution_result_receipt_terminal_operator_decision_accepted",
        "dry_run_execution_result_receipt_terminal_operator_decision_persisted",
        "dry_run_execution_result_receipt_terminal_operator_decision_materialized",
        "dry_run_execution_result_receipt_terminal_operator_decision_filesystem_written",
        "dry_run_execution_result_receipt_terminal_operator_decision_delivered",
        "dry_run_execution_result_receipt_terminal_operator_status_recorded",
        "dry_run_execution_result_receipt_terminal_operator_status_persisted",
        "dry_run_execution_result_receipt_terminal_operator_decision_promoted",
        "dry_run_execution_result_receipt_terminal_operator_status_promoted",
        "dry_run_execution_result_receipt_public_claim_recorded",
        "dry_run_execution_result_receipt_public_claim_promoted",
        "dry_run_execution_result_receipt_public_ga_claimed",
        "dry_run_execution_result_receipt_public_release_published",
        "dry_run_execution_result_receipt_public_distribution_performed",
        "dry_run_execution_result_receipt_public_artifact_written",
        "dry_run_execution_result_receipt_release_artifact_written",
        "dry_run_execution_result_receipt_authority_promoted_from_terminal_operator_decision",
        "dry_run_execution_executed",
        "production_durable_memory_write_executed",
        "production_durable_memory_store_write_performed",
        "actual_production_durable_memory_write_performed",
        "durable_memory_store_write_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "wal_write_performed",
        "receipt_persisted",
        "rollback_executed",
        "tombstone_cleanup_executed",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "channel_send_performed",
        "external_send_performed",
        "release_artifact_written",
        "public_artifact_written",
        "public_ga_claimed",
        "public_release_claimed",
        "public_release_published",
        "public_distribution_performed",
        "install_executed",
        "service_restarted",
        "active_binary_mutated",
    ] {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "endpoint": HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_PRODUCTION_DURABLE_MEMORY_WRITE_DRY_RUN_EXECUTION_RESULT_RECEIPT_TERMINAL_OPERATOR_DECISION_PUBLIC_CLAIM_NON_PROMOTION_DENIAL_BOUNDARY_ENDPOINT,
        "source_command": "/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial-boundary --json",
        "native_route": true,
        "side_effect_free": true,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_ready": report_ready,
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_ready": report_ready,
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_accepted": report_ready,
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_mode": "dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_no_terminal_decision_no_public_claim_no_authority_no_execution_no_production_durable_memory_mutation",
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_ready": source_bool("memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_ready"),
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_report_sha256": source_report_sha256,
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_hash_sha256": source_str("scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_hash_sha256"),
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_policy_hash_sha256": source_str("scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_policy_hash_sha256"),
        "source_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_result_hash_sha256": source_str("dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_result_hash_sha256"),
        "source_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_handoff_hash_sha256": source_str("dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_handoff_hash_sha256"),
        "source_accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_fixture_count": source_u64("accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_fixture_count"),
        "source_blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_fixture_count": source_u64("blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_fixture_count"),
        "source_denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_count": source_u64("denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_count"),
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
        "dry_run_execution_result_receipt_terminal_operator_decision_denial_hash_sha256": terminal_decision_denial_hash_sha256,
        "dry_run_execution_result_receipt_public_claim_denial_hash_sha256": public_claim_denial_hash_sha256,
        "dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_matrix_hash_sha256": terminal_public_claim_matrix_hash_sha256,
        "dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_handoff_hash_sha256": terminal_public_claim_handoff_hash_sha256,
        "dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_result_hash_sha256": terminal_public_claim_result_hash_sha256,
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_hash_sha256": terminal_public_claim_boundary_hash_sha256,
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_policy_hash_sha256": terminal_public_claim_policy_hash_sha256,
        "required_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_surface_count": 14,
        "ready_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_surface_count": 14,
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_fixture_count": 10,
        "accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_fixture_count": 1,
        "blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_fixture_count": 9,
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_fixtures": fixtures,
        "denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary": denials,
        "denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_count": denial_count,
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_performed_count": 1,
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_result_accepted_count": 1,
        "source_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_bound_count": 1,
        "dry_run_execution_result_receipt_terminal_operator_decision_request_denied_count": 1,
        "dry_run_execution_result_receipt_terminal_operator_decision_status_denied_count": 1,
        "dry_run_execution_result_receipt_public_claim_denied_count": 1,
        "dry_run_execution_result_receipt_public_release_publication_denied_count": 1,
        "dry_run_execution_result_receipt_terminal_operator_decision_public_claim_authority_denied_count": 1,
        "dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_handoff_bound_count": 1,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
        "dry_run_execution_result_receipt_terminal_operator_decision_recorded_count": 0,
        "dry_run_execution_result_receipt_terminal_operator_decision_accepted_count": 0,
        "dry_run_execution_result_receipt_terminal_operator_decision_persisted_count": 0,
        "dry_run_execution_result_receipt_terminal_operator_status_recorded_count": 0,
        "dry_run_execution_result_receipt_public_claim_recorded_count": 0,
        "dry_run_execution_result_receipt_public_claim_promoted_count": 0,
        "dry_run_execution_result_receipt_public_ga_claimed_count": 0,
        "dry_run_execution_result_receipt_public_release_published_count": 0,
        "dry_run_execution_result_receipt_release_artifact_written_count": 0,
        "dry_run_execution_result_receipt_authority_promoted_from_terminal_operator_decision_count": 0,
        "dry_run_execution_executed_count": 0,
        "production_durable_memory_write_executed_count": 0,
        "production_durable_memory_store_write_performed_count": 0,
        "memory_store_write_performed_count": 0,
        "wal_write_performed_count": 0,
        "receipt_persisted_count": 0,
        "live_kg_write_performed_count": 0,
        "provider_invoked_count": 0,
        "model_invoked_count": 0,
        "credential_read_count": 0,
        "channel_send_performed_count": 0,
        "external_send_performed_count": 0,
        "release_artifact_written_count": 0,
        "install_executed_count": 0,
        "service_restarted_count": 0,
        "active_binary_mutated_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
        "dry_run_execution_result_receipt_terminal_operator_decision_requested": false,
        "dry_run_execution_result_receipt_terminal_operator_decision_recorded": false,
        "dry_run_execution_result_receipt_terminal_operator_decision_accepted": false,
        "dry_run_execution_result_receipt_terminal_operator_decision_persisted": false,
        "dry_run_execution_result_receipt_terminal_operator_status_recorded": false,
        "dry_run_execution_result_receipt_terminal_operator_status_persisted": false,
        "dry_run_execution_result_receipt_terminal_operator_decision_promoted": false,
        "dry_run_execution_result_receipt_terminal_operator_status_promoted": false,
        "dry_run_execution_result_receipt_public_claim_recorded": false,
        "dry_run_execution_result_receipt_public_claim_promoted": false,
        "dry_run_execution_result_receipt_public_ga_claimed": false,
        "dry_run_execution_result_receipt_public_release_published": false,
        "dry_run_execution_result_receipt_release_artifact_written": false,
        "dry_run_execution_result_receipt_authority_promoted_from_terminal_operator_decision": false,
        "dry_run_execution_executed": false,
        "production_durable_memory_write_executed": false,
        "production_durable_memory_store_write_performed": false,
        "actual_production_durable_memory_write_performed": false,
        "durable_memory_store_write_performed": false,
        "memory_store_write_performed": false,
        "memory_store_mutated": false,
        "wal_write_performed": false,
        "receipt_persisted": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
        "rollback_executed": false,
        "tombstone_cleanup_executed": false,
        "live_kg_write_performed": false,
        "provider_invoked": false,
        "model_invoked": false,
        "credential_read": false,
        "channel_send_performed": false,
        "external_send_performed": false,
        "release_artifact_written": false,
        "install_executed": false,
        "service_restarted": false,
        "active_binary_mutated": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
        "allowed_next_actions": [
            {
                "action": "run_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_require_live_gate",
                "status": "allowed_verification_only",
                "accepts_terminal_decision": false,
                "records_terminal_decision": false,
                "promotes_public_claim": false,
                "claims_public_release": false,
                "writes_release_artifact": false,
                "executes_dry_run": false,
                "writes_production_durable_memory": false,
                "writes_memory_or_kg": false,
                "invokes_provider": false,
                "sends_externally": false,
                "installs_or_restarts": false,
                "mutates_active_binary": false
            },
            {
                "action": "prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary",
                "status": "allowed_report_only_next_slice",
                "publishes_release_artifact": false,
                "writes_release_artifact": false,
                "claims_public_release": false,
                "mutates_runtime": false,
                "invokes_model": false,
                "writes_memory_or_kg": false
            }
        ],
        "side_effects": side_effects
        }),
    );
    report
}
