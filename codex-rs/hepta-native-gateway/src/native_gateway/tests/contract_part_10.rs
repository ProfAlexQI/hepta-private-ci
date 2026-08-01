#[test]
fn hepta_memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_blocks_cancel_supersede_replace_tombstone_execution_or_production_side_effects()
 {
    let body = route_contract_body(HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_PRODUCTION_DURABLE_MEMORY_WRITE_DRY_RUN_EXECUTION_RESULT_RECEIPT_CANCELLATION_SUPERSESSION_DENIAL_BOUNDARY_ENDPOINT);

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "scoped production durable Memory write dry-run execution result receipt cancellation/supersession denial json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_PRODUCTION_DURABLE_MEMORY_WRITE_DRY_RUN_EXECUTION_RESULT_RECEIPT_CANCELLATION_SUPERSESSION_DENIAL_BOUNDARY_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-cancellation-supersession-denial-boundary --json"
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
        value["memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary_ready"],
        true
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_ready"],
        true
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_accepted"],
        true
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_mode"],
        "dry_run_execution_result_receipt_cancellation_supersession_denial_boundary_no_cancel_no_supersede_no_replacement_no_tombstone_no_execution_no_production_durable_memory_mutation"
    );
    assert_eq!(
        value["source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_ready"],
        true
    );
    assert_eq!(
        value["source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_accepted_count"],
        1
    );
    assert_eq!(
        value["source_accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_fixture_count"],
        1
    );
    assert_eq!(
        value["source_blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_fixture_count"],
        9
    );
    assert_eq!(
        value["source_denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_count"],
        55
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
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_report_sha256",
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_hash_sha256",
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_ordering_monotonicity_denial_policy_hash_sha256",
        "source_dry_run_execution_result_receipt_ordering_monotonicity_denial_matrix_hash_sha256",
        "source_dry_run_execution_result_receipt_ordering_sequence_policy_hash_sha256",
        "source_dry_run_execution_result_receipt_late_receipt_denial_hash_sha256",
        "source_dry_run_execution_result_receipt_future_receipt_denial_hash_sha256",
        "source_dry_run_execution_result_receipt_rollback_sequence_denial_hash_sha256",
        "source_dry_run_execution_result_receipt_same_sequence_replacement_denial_hash_sha256",
        "source_dry_run_execution_result_receipt_latest_wins_promotion_denial_hash_sha256",
        "source_dry_run_execution_result_receipt_sequence_gap_denial_hash_sha256",
        "source_dry_run_execution_result_receipt_ordering_handoff_hash_sha256",
        "source_dry_run_execution_result_receipt_ordering_result_hash_sha256",
        "dry_run_execution_result_receipt_cancellation_supersession_denial_matrix_hash_sha256",
        "dry_run_execution_result_receipt_cancellation_policy_hash_sha256",
        "dry_run_execution_result_receipt_supersession_policy_hash_sha256",
        "dry_run_execution_result_receipt_replacement_receipt_denial_hash_sha256",
        "dry_run_execution_result_receipt_tombstone_delete_marker_denial_hash_sha256",
        "dry_run_execution_result_receipt_latest_replacement_denial_hash_sha256",
        "dry_run_execution_result_receipt_completion_ack_replacement_denial_hash_sha256",
        "dry_run_execution_result_receipt_export_query_replacement_denial_hash_sha256",
        "dry_run_execution_result_receipt_cancellation_supersession_handoff_hash_sha256",
        "dry_run_execution_result_receipt_cancellation_supersession_result_hash_sha256",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary_hash_sha256",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_policy_hash_sha256",
    ] {
        assert_ne!(
            value[key], "",
            "dry-run execution result receipt cancellation/supersession denial hash should be present: {key}"
        );
    }
    assert_eq!(
        value["required_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_surface_count"],
        16
    );
    assert_eq!(
        value["ready_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_surface_count"],
        16
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_fixture_count"],
        10
    );
    assert_eq!(
        value["accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_fixture_count"],
        1
    );
    assert_eq!(
        value["blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_fixture_count"],
        9
    );
    assert_eq!(
        value["denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary_count"],
        65
    );
    for key in [
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary_performed_count",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_result_recorded_count",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_result_accepted_count",
        "source_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_accepted_count",
        "dry_run_execution_result_receipt_cancellation_supersession_denial_matrix_bound_count",
        "dry_run_execution_result_receipt_cancellation_policy_bound_count",
        "dry_run_execution_result_receipt_supersession_policy_bound_count",
        "dry_run_execution_result_receipt_cancellation_request_denied_count",
        "dry_run_execution_result_receipt_supersession_request_denied_count",
        "dry_run_execution_result_receipt_replacement_receipt_denied_count",
        "dry_run_execution_result_receipt_tombstone_delete_marker_denied_count",
        "dry_run_execution_result_receipt_latest_replacement_denied_count",
        "dry_run_execution_result_receipt_completion_ack_replacement_denied_count",
        "dry_run_execution_result_receipt_export_query_replacement_denied_count",
        "dry_run_execution_result_receipt_cancellation_supersession_handoff_bound_count",
    ] {
        assert_eq!(
            value[key], 1,
            "cancellation/supersession denial count should be one: {key}"
        );
    }
    for key in [
        "dry_run_execution_result_receipt_cancellation_request_accepted_count",
        "dry_run_execution_result_receipt_cancellation_recorded_count",
        "dry_run_execution_result_receipt_cancellation_persisted_count",
        "dry_run_execution_result_receipt_cancellation_ledger_written_count",
        "dry_run_execution_result_receipt_supersession_request_accepted_count",
        "dry_run_execution_result_receipt_supersession_recorded_count",
        "dry_run_execution_result_receipt_supersession_persisted_count",
        "dry_run_execution_result_receipt_supersession_ledger_written_count",
        "dry_run_execution_result_receipt_replacement_receipt_accepted_count",
        "dry_run_execution_result_receipt_replacement_receipt_recorded_count",
        "dry_run_execution_result_receipt_replacement_receipt_persisted_count",
        "dry_run_execution_result_receipt_replacement_receipt_materialized_count",
        "dry_run_execution_result_receipt_replacement_receipt_filesystem_written_count",
        "dry_run_execution_result_receipt_replacement_receipt_ledger_written_count",
        "dry_run_execution_result_receipt_tombstone_delete_marker_accepted_count",
        "dry_run_execution_result_receipt_tombstone_delete_marker_written_count",
        "dry_run_execution_result_receipt_latest_replacement_promoted_count",
        "dry_run_execution_result_receipt_completion_ack_replaced_count",
        "dry_run_execution_result_receipt_export_query_replaced_count",
        "dry_run_execution_result_receipt_cancellation_supersession_state_persisted_count",
        "dry_run_execution_result_receipt_cancellation_supersession_ledger_written_count",
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
            "cancellation/supersession denial side-effect count should stay zero: {key}"
        );
    }
    for key in [
        "source_dry_run_execution_result_receipt_ordering_monotonicity_denial_boundary_bound",
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
        "production_durable_memory_write_forbidden",
        "memory_store_mutation_forbidden",
        "kg_live_write_forbidden",
        "provider_model_invocation_forbidden",
        "credential_channel_public_release_forbidden",
        "install_restart_active_binary_mutation_forbidden",
    ] {
        assert_eq!(
            value[key], true,
            "cancellation/supersession denial field should be true: {key}"
        );
    }
    for key in [
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
        "dry_run_execution_result_receipt_persisted",
        "dry_run_execution_executed",
        "production_durable_memory_write_executed",
        "production_durable_memory_store_write_performed",
        "actual_production_durable_memory_write_performed",
        "durable_memory_store_write_performed",
        "durable_memory_store_read_performed",
        "durable_memory_store_rollback_performed",
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
            "cancellation/supersession persistence, mutation, execution, or external field should stay false: {key}"
        );
    }
    let fixtures = value
            ["scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_fixtures"]
            .as_array()
            .expect(
                "scoped production durable Memory write dry-run execution result receipt cancellation/supersession denial fixtures",
            );
    assert_eq!(fixtures.len(), 10);
    assert_eq!(
            fixtures
                .iter()
                .filter(|fixture| {
                    fixture["scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_accepted"]
                        == true
                })
                .count(),
            1
        );
    let denied = value
            ["denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary"]
            .as_array()
            .expect("scoped production durable Memory write dry-run execution result receipt cancellation/supersession denials");
    assert_eq!(denied.len(), 65);
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "run_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary_require_live_gate"
    );
    assert_eq!(
        value["allowed_next_actions"][0]["records_cancellation"],
        false
    );
    assert_eq!(
        value["allowed_next_actions"][0]["records_supersession"],
        false
    );
    assert_eq!(
        value["allowed_next_actions"][0]["accepts_replacement_receipt"],
        false
    );
    assert_eq!(
        value["allowed_next_actions"][0]["writes_tombstone_delete_marker"],
        false
    );
    assert_eq!(value["allowed_next_actions"][0]["executes_dry_run"], false);
    assert_eq!(
        value["allowed_next_actions"][0]["writes_production_durable_memory"],
        false
    );
    assert_eq!(
        value["allowed_next_actions"][1]["action"],
        "prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary"
    );
    assert_eq!(
        value["allowed_next_actions"][1]["requires_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary"],
        true
    );
    let side_effects = value["side_effects"].as_object().expect(
            "scoped production durable Memory write dry-run execution result receipt cancellation/supersession side effects",
        );
    assert_eq!(
            side_effects["scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary_performed"].as_bool(),
            Some(true)
        );
    assert_eq!(
            side_effects["scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_result_accepted"].as_bool(),
            Some(true)
        );
    assert_eq!(
        side_effects["dry_run_execution_result_receipt_cancellation_request_accepted"].as_bool(),
        Some(false)
    );
    assert_eq!(
        side_effects["dry_run_execution_result_receipt_supersession_request_accepted"].as_bool(),
        Some(false)
    );
    assert_eq!(
        side_effects["dry_run_execution_result_receipt_replacement_receipt_persisted"].as_bool(),
        Some(false)
    );
    assert_eq!(
        side_effects["dry_run_execution_result_receipt_tombstone_delete_marker_written"].as_bool(),
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
fn hepta_memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_blocks_audit_evidence_authority_execution_or_production_side_effects()
 {
    let body = route_contract_body(HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_PRODUCTION_DURABLE_MEMORY_WRITE_DRY_RUN_EXECUTION_RESULT_RECEIPT_AUDIT_TRAIL_IMMUTABLE_EVIDENCE_DENIAL_BOUNDARY_ENDPOINT);

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "scoped production durable Memory write dry-run execution result receipt audit trail/immutable evidence denial json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_PRODUCTION_DURABLE_MEMORY_WRITE_DRY_RUN_EXECUTION_RESULT_RECEIPT_AUDIT_TRAIL_IMMUTABLE_EVIDENCE_DENIAL_BOUNDARY_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-audit-trail-immutable-evidence-denial-boundary --json"
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
        value["memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary_ready"],
        true
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_ready"],
        true
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_accepted"],
        true
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_mode"],
        "dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary_no_audit_no_immutable_evidence_no_hash_chain_no_attestation_no_authority_no_execution_no_production_durable_memory_mutation"
    );
    assert_eq!(
        value["source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary_ready"],
        true
    );
    assert_eq!(
        value["source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary_accepted_count"],
        1
    );
    assert_eq!(
        value["source_accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_fixture_count"],
        1
    );
    assert_eq!(
        value["source_blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_fixture_count"],
        9
    );
    assert_eq!(
        value["source_denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary_count"],
        65
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
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary_report_sha256",
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary_hash_sha256",
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_policy_hash_sha256",
        "source_dry_run_execution_result_receipt_cancellation_supersession_denial_matrix_hash_sha256",
        "source_dry_run_execution_result_receipt_cancellation_supersession_result_hash_sha256",
        "source_dry_run_execution_result_receipt_cancellation_supersession_handoff_hash_sha256",
        "source_dry_run_execution_result_receipt_replacement_receipt_denial_hash_sha256",
        "source_dry_run_execution_result_receipt_tombstone_delete_marker_denial_hash_sha256",
        "dry_run_execution_result_receipt_audit_trail_denial_matrix_hash_sha256",
        "dry_run_execution_result_receipt_immutable_evidence_denial_matrix_hash_sha256",
        "dry_run_execution_result_receipt_audit_hash_chain_denial_hash_sha256",
        "dry_run_execution_result_receipt_audit_attestation_denial_hash_sha256",
        "dry_run_execution_result_receipt_audit_ledger_evidence_denial_hash_sha256",
        "dry_run_execution_result_receipt_audit_evidence_handoff_hash_sha256",
        "dry_run_execution_result_receipt_audit_evidence_result_hash_sha256",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary_hash_sha256",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_policy_hash_sha256",
    ] {
        assert_ne!(
            value[key], "",
            "dry-run execution result receipt audit trail/immutable evidence denial hash should be present: {key}"
        );
    }
    assert_eq!(
        value["required_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_surface_count"],
        16
    );
    assert_eq!(
        value["ready_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_surface_count"],
        16
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_fixture_count"],
        10
    );
    assert_eq!(
        value["accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_fixture_count"],
        1
    );
    assert_eq!(
        value["blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_fixture_count"],
        9
    );
    assert_eq!(
        value["denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary_count"],
        72
    );
    for key in [
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary_performed_count",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_result_recorded_count",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_result_accepted_count",
        "source_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary_accepted_count",
        "dry_run_execution_result_receipt_audit_trail_denial_matrix_bound_count",
        "dry_run_execution_result_receipt_immutable_evidence_denial_matrix_bound_count",
        "dry_run_execution_result_receipt_audit_trail_request_denied_count",
        "dry_run_execution_result_receipt_immutable_evidence_request_denied_count",
        "dry_run_execution_result_receipt_hash_chain_denied_count",
        "dry_run_execution_result_receipt_merkle_root_denied_count",
        "dry_run_execution_result_receipt_attestation_denied_count",
        "dry_run_execution_result_receipt_witness_denied_count",
        "dry_run_execution_result_receipt_notary_denied_count",
        "dry_run_execution_result_receipt_ledger_index_delivery_evidence_denied_count",
        "dry_run_execution_result_receipt_audit_evidence_authority_denied_count",
        "dry_run_execution_result_receipt_audit_evidence_handoff_bound_count",
    ] {
        assert_eq!(
            value[key], 1,
            "audit trail/immutable evidence denial count should be one: {key}"
        );
    }
    for key in [
        "dry_run_execution_result_receipt_audit_trail_accepted_count",
        "dry_run_execution_result_receipt_audit_trail_recorded_count",
        "dry_run_execution_result_receipt_audit_trail_persisted_count",
        "dry_run_execution_result_receipt_immutable_evidence_accepted_count",
        "dry_run_execution_result_receipt_immutable_evidence_recorded_count",
        "dry_run_execution_result_receipt_immutable_evidence_persisted_count",
        "dry_run_execution_result_receipt_hash_chain_recorded_count",
        "dry_run_execution_result_receipt_merkle_root_recorded_count",
        "dry_run_execution_result_receipt_attestation_recorded_count",
        "dry_run_execution_result_receipt_witness_recorded_count",
        "dry_run_execution_result_receipt_notary_recorded_count",
        "dry_run_execution_result_receipt_ledger_evidence_recorded_count",
        "dry_run_execution_result_receipt_index_evidence_recorded_count",
        "dry_run_execution_result_receipt_delivery_evidence_recorded_count",
        "dry_run_execution_result_receipt_authority_promoted_from_audit_trail_count",
        "dry_run_execution_result_receipt_persisted_count",
        "dry_run_execution_executed_count",
        "production_durable_memory_write_executed_count",
        "production_durable_memory_store_write_performed_count",
        "actual_production_durable_memory_write_performed_count",
        "durable_memory_store_write_performed_count",
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
            "audit trail/immutable evidence side-effect count should stay zero: {key}"
        );
    }
    for key in [
        "source_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary_bound",
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
        "production_durable_memory_write_forbidden",
        "memory_store_mutation_forbidden",
        "kg_live_write_forbidden",
        "provider_model_invocation_forbidden",
        "credential_channel_public_release_forbidden",
        "install_restart_active_binary_mutation_forbidden",
    ] {
        assert_eq!(
            value[key], true,
            "audit trail/immutable evidence denial field should be true: {key}"
        );
    }
    for key in [
        "dry_run_execution_result_receipt_audit_trail_accepted",
        "dry_run_execution_result_receipt_audit_trail_recorded",
        "dry_run_execution_result_receipt_audit_trail_persisted",
        "dry_run_execution_result_receipt_immutable_evidence_accepted",
        "dry_run_execution_result_receipt_immutable_evidence_recorded",
        "dry_run_execution_result_receipt_immutable_evidence_persisted",
        "dry_run_execution_result_receipt_hash_chain_recorded",
        "dry_run_execution_result_receipt_merkle_root_recorded",
        "dry_run_execution_result_receipt_attestation_recorded",
        "dry_run_execution_result_receipt_witness_recorded",
        "dry_run_execution_result_receipt_notary_recorded",
        "dry_run_execution_result_receipt_ledger_evidence_recorded",
        "dry_run_execution_result_receipt_index_evidence_recorded",
        "dry_run_execution_result_receipt_delivery_evidence_recorded",
        "dry_run_execution_result_receipt_authority_promoted_from_audit_trail",
        "dry_run_execution_result_receipt_authority_promoted_from_immutable_evidence",
        "dry_run_execution_result_receipt_persisted",
        "dry_run_execution_executed",
        "production_durable_memory_write_executed",
        "production_durable_memory_store_write_performed",
        "actual_production_durable_memory_write_performed",
        "durable_memory_store_write_performed",
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
            "audit/evidence persistence, authority, mutation, execution, or external field should stay false: {key}"
        );
    }
    let fixtures = value
            ["scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_fixtures"]
            .as_array()
            .expect(
                "scoped production durable Memory write dry-run execution result receipt audit trail/immutable evidence denial fixtures",
            );
    assert_eq!(fixtures.len(), 10);
    assert_eq!(
            fixtures
                .iter()
                .filter(|fixture| {
                    fixture["scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_accepted"]
                        == true
                })
                .count(),
            1
        );
    let denied = value
            ["denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary"]
            .as_array()
            .expect("scoped production durable Memory write dry-run execution result receipt audit trail/immutable evidence denials");
    assert_eq!(denied.len(), 72);
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "run_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary_require_live_gate"
    );
    assert_eq!(
        value["allowed_next_actions"][0]["records_audit_trail"],
        false
    );
    assert_eq!(
        value["allowed_next_actions"][0]["records_immutable_evidence"],
        false
    );
    assert_eq!(
        value["allowed_next_actions"][0]["promotes_authority"],
        false
    );
    assert_eq!(value["allowed_next_actions"][0]["executes_dry_run"], false);
    assert_eq!(
        value["allowed_next_actions"][0]["writes_production_durable_memory"],
        false
    );
    assert_eq!(
        value["allowed_next_actions"][1]["action"],
        "prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary"
    );
    assert_eq!(
        value["allowed_next_actions"][1]["requires_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary"],
        true
    );
    let side_effects = value["side_effects"].as_object().expect(
            "scoped production durable Memory write dry-run execution result receipt audit trail/immutable evidence side effects",
        );
    assert_eq!(
            side_effects["scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary_performed"].as_bool(),
            Some(true)
        );
    assert_eq!(
            side_effects["scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_result_accepted"].as_bool(),
            Some(true)
        );
    assert_eq!(
        side_effects["dry_run_execution_result_receipt_audit_trail_recorded"].as_bool(),
        Some(false)
    );
    assert_eq!(
        side_effects["dry_run_execution_result_receipt_immutable_evidence_persisted"].as_bool(),
        Some(false)
    );
    assert_eq!(
        side_effects["dry_run_execution_result_receipt_hash_chain_recorded"].as_bool(),
        Some(false)
    );
    assert_eq!(
        side_effects["dry_run_execution_result_receipt_authority_promoted_from_audit_trail"]
            .as_bool(),
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
fn hepta_memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_blocks_lifecycle_delete_authority_execution_or_production_side_effects()
 {
    let body = route_contract_body(HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_PRODUCTION_DURABLE_MEMORY_WRITE_DRY_RUN_EXECUTION_RESULT_RECEIPT_RETENTION_EXPIRY_GARBAGE_COLLECTION_DENIAL_BOUNDARY_ENDPOINT);

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "scoped production durable Memory write dry-run execution result receipt retention/expiry/garbage-collection denial json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_PRODUCTION_DURABLE_MEMORY_WRITE_DRY_RUN_EXECUTION_RESULT_RECEIPT_RETENTION_EXPIRY_GARBAGE_COLLECTION_DENIAL_BOUNDARY_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-retention-expiry-garbage-collection-denial-boundary --json"
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
        value["memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary_ready"],
        true
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_accepted"],
        true
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_mode"],
        "dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary_no_retention_no_expiry_no_gc_no_delete_no_archive_no_compaction_no_authority_no_execution_no_production_durable_memory_mutation"
    );
    assert_eq!(
        value["source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary_ready"],
        true
    );
    assert_eq!(
        value["source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary_accepted_count"],
        1
    );
    assert_eq!(
        value["source_accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_fixture_count"],
        1
    );
    assert_eq!(
        value["source_blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_fixture_count"],
        9
    );
    assert_eq!(
        value["source_denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary_count"],
        72
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
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary_report_sha256",
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary_hash_sha256",
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_policy_hash_sha256",
        "source_dry_run_execution_result_receipt_audit_evidence_result_hash_sha256",
        "source_dry_run_execution_result_receipt_audit_evidence_handoff_hash_sha256",
        "dry_run_execution_result_receipt_retention_policy_denial_hash_sha256",
        "dry_run_execution_result_receipt_retention_index_denial_hash_sha256",
        "dry_run_execution_result_receipt_expiry_lifecycle_denial_hash_sha256",
        "dry_run_execution_result_receipt_garbage_collection_denial_hash_sha256",
        "dry_run_execution_result_receipt_archive_compaction_denial_hash_sha256",
        "dry_run_execution_result_receipt_retention_expiry_garbage_collection_handoff_hash_sha256",
        "dry_run_execution_result_receipt_retention_expiry_garbage_collection_result_hash_sha256",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary_hash_sha256",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_policy_hash_sha256",
    ] {
        assert_ne!(
            value[key], "",
            "dry-run execution result receipt retention/expiry/garbage-collection denial hash should be present: {key}"
        );
    }
    assert_eq!(
        value["required_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_surface_count"],
        16
    );
    assert_eq!(
        value["ready_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_surface_count"],
        16
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_fixture_count"],
        10
    );
    assert_eq!(
        value["accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_fixture_count"],
        1
    );
    assert_eq!(
        value["blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_fixture_count"],
        9
    );
    assert_eq!(
        value["denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary_count"],
        62
    );
    for key in [
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary_performed_count",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_result_recorded_count",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_result_accepted_count",
        "source_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary_accepted_count",
        "dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_matrix_bound_count",
        "dry_run_execution_result_receipt_retention_policy_request_denied_count",
        "dry_run_execution_result_receipt_retention_index_denied_count",
        "dry_run_execution_result_receipt_ttl_lease_update_extension_denied_count",
        "dry_run_execution_result_receipt_expiry_request_denied_count",
        "dry_run_execution_result_receipt_expiry_scheduler_timer_denied_count",
        "dry_run_execution_result_receipt_garbage_collection_request_denied_count",
        "dry_run_execution_result_receipt_garbage_collection_scan_denied_count",
        "dry_run_execution_result_receipt_delete_tombstone_sweep_denied_count",
        "dry_run_execution_result_receipt_archive_compaction_denied_count",
        "dry_run_execution_result_receipt_retention_gc_authority_denied_count",
        "dry_run_execution_result_receipt_retention_expiry_garbage_collection_handoff_bound_count",
    ] {
        assert_eq!(
            value[key], 1,
            "retention/expiry/garbage-collection denial count should be one: {key}"
        );
    }
    for key in [
        "dry_run_execution_result_receipt_retention_policy_recorded_count",
        "dry_run_execution_result_receipt_retention_policy_persisted_count",
        "dry_run_execution_result_receipt_retention_index_recorded_count",
        "dry_run_execution_result_receipt_expiry_scheduler_registered_count",
        "dry_run_execution_result_receipt_expiry_timer_started_count",
        "dry_run_execution_result_receipt_garbage_collection_queue_recorded_count",
        "dry_run_execution_result_receipt_garbage_collection_scan_performed_count",
        "dry_run_execution_result_receipt_delete_marker_garbage_collected_count",
        "dry_run_execution_result_receipt_tombstone_garbage_collected_count",
        "dry_run_execution_result_receipt_sweep_performed_count",
        "dry_run_execution_result_receipt_archive_written_count",
        "dry_run_execution_result_receipt_compaction_performed_count",
        "dry_run_execution_result_receipt_authority_promoted_from_retention_policy_count",
        "dry_run_execution_result_receipt_authority_promoted_from_garbage_collection_count",
        "dry_run_execution_result_receipt_persisted_count",
        "dry_run_execution_executed_count",
        "production_durable_memory_write_executed_count",
        "production_durable_memory_store_write_performed_count",
        "actual_production_durable_memory_write_performed_count",
        "durable_memory_store_write_performed_count",
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
            "retention/expiry/garbage-collection side-effect count should stay zero: {key}"
        );
    }
    for key in [
        "source_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary_bound",
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
        "production_write_execution_forbidden_on_retention_gc_route",
        "production_durable_memory_write_forbidden",
        "memory_store_mutation_forbidden",
        "kg_live_write_forbidden",
        "provider_model_invocation_forbidden",
        "credential_channel_public_release_forbidden",
        "install_restart_active_binary_mutation_forbidden",
    ] {
        assert_eq!(
            value[key], true,
            "retention/expiry/garbage-collection denial field should be true: {key}"
        );
    }
    for key in [
        "dry_run_execution_result_receipt_retention_policy_recorded",
        "dry_run_execution_result_receipt_retention_policy_persisted",
        "dry_run_execution_result_receipt_retention_index_recorded",
        "dry_run_execution_result_receipt_ttl_lease_recorded",
        "dry_run_execution_result_receipt_ttl_update_applied",
        "dry_run_execution_result_receipt_ttl_extension_applied",
        "dry_run_execution_result_receipt_expiry_timestamp_recorded",
        "dry_run_execution_result_receipt_expiry_scheduler_registered",
        "dry_run_execution_result_receipt_expiry_timer_started",
        "dry_run_execution_result_receipt_expiry_ack_recorded",
        "dry_run_execution_result_receipt_garbage_collection_queue_recorded",
        "dry_run_execution_result_receipt_garbage_collection_scan_performed",
        "dry_run_execution_result_receipt_garbage_collection_candidate_recorded",
        "dry_run_execution_result_receipt_garbage_collection_decision_recorded",
        "dry_run_execution_result_receipt_delete_marker_garbage_collected",
        "dry_run_execution_result_receipt_tombstone_garbage_collected",
        "dry_run_execution_result_receipt_sweep_performed",
        "dry_run_execution_result_receipt_archive_written",
        "dry_run_execution_result_receipt_compaction_performed",
        "dry_run_execution_result_receipt_compaction_artifact_written",
        "dry_run_execution_result_receipt_authority_promoted_from_retention_policy",
        "dry_run_execution_result_receipt_authority_promoted_from_expiry",
        "dry_run_execution_result_receipt_authority_promoted_from_garbage_collection",
        "dry_run_execution_result_receipt_persisted",
        "dry_run_execution_executed",
        "production_durable_memory_write_executed",
        "production_durable_memory_store_write_performed",
        "actual_production_durable_memory_write_performed",
        "durable_memory_store_write_performed",
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
            "retention/expiry/garbage-collection lifecycle, authority, mutation, execution, or external field should stay false: {key}"
        );
    }
    let fixtures = value
            ["scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_fixtures"]
            .as_array()
            .expect(
                "scoped production durable Memory write dry-run execution result receipt retention/expiry/garbage-collection denial fixtures",
            );
    assert_eq!(fixtures.len(), 10);
    assert_eq!(
            fixtures
                .iter()
                .filter(|fixture| {
                    fixture["scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_accepted"]
                        == true
                })
                .count(),
            1
        );
    let denied = value
            ["denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary"]
            .as_array()
            .expect("scoped production durable Memory write dry-run execution result receipt retention/expiry/garbage-collection denials");
    assert_eq!(denied.len(), 62);
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "run_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary_require_live_gate"
    );
    assert_eq!(
        value["allowed_next_actions"][0]["records_retention_policy"],
        false
    );
    assert_eq!(
        value["allowed_next_actions"][0]["performs_garbage_collection_scan"],
        false
    );
    assert_eq!(value["allowed_next_actions"][0]["deletes_memory"], false);
    assert_eq!(
        value["allowed_next_actions"][0]["performs_compaction"],
        false
    );
    assert_eq!(value["allowed_next_actions"][0]["executes_dry_run"], false);
    assert_eq!(
        value["allowed_next_actions"][0]["writes_production_durable_memory"],
        false
    );
    assert_eq!(
        value["allowed_next_actions"][1]["action"],
        "prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_boundary"
    );
    assert_eq!(
        value["allowed_next_actions"][1]["requires_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary"],
        true
    );
    let side_effects = value["side_effects"].as_object().expect(
            "scoped production durable Memory write dry-run execution result receipt retention/expiry/garbage-collection side effects",
        );
    assert_eq!(
            side_effects["scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary_performed"].as_bool(),
            Some(true)
        );
    assert_eq!(
            side_effects["scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_result_accepted"].as_bool(),
            Some(true)
        );
    assert_eq!(
        side_effects["dry_run_execution_result_receipt_retention_policy_recorded"].as_bool(),
        Some(false)
    );
    assert_eq!(
        side_effects["dry_run_execution_result_receipt_expiry_scheduler_registered"].as_bool(),
        Some(false)
    );
    assert_eq!(
        side_effects["dry_run_execution_result_receipt_garbage_collection_scan_performed"]
            .as_bool(),
        Some(false)
    );
    assert_eq!(
        side_effects["dry_run_execution_result_receipt_delete_marker_garbage_collected"].as_bool(),
        Some(false)
    );
    assert_eq!(
        side_effects["dry_run_execution_result_receipt_archive_written"].as_bool(),
        Some(false)
    );
    assert_eq!(
        side_effects["dry_run_execution_result_receipt_authority_promoted_from_garbage_collection"]
            .as_bool(),
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
fn hepta_memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_blocks_reporting_surfaces_without_persistence_authority_execution_or_production_side_effects()
 {
    let body = route_contract_body(HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_PRODUCTION_DURABLE_MEMORY_WRITE_DRY_RUN_EXECUTION_RESULT_RECEIPT_EXPORT_QUERY_OBSERVABILITY_DENIAL_BOUNDARY_ENDPOINT);

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "scoped production durable Memory write dry-run execution result receipt export/query/observability denial json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_PRODUCTION_DURABLE_MEMORY_WRITE_DRY_RUN_EXECUTION_RESULT_RECEIPT_EXPORT_QUERY_OBSERVABILITY_DENIAL_BOUNDARY_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-export-query-observability-denial-boundary --json"
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
        value["memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_boundary_ready"],
        true
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_accepted"],
        true
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_mode"],
        "dry_run_execution_result_receipt_export_query_observability_denial_boundary_no_export_no_query_no_observability_no_dashboard_no_alert_no_operator_summary_no_authority_no_execution_no_production_durable_memory_mutation"
    );
    assert_eq!(
        value["source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary_ready"],
        true
    );
    assert_eq!(
        value["source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary_accepted_count"],
        1
    );
    assert_eq!(
        value["source_accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_fixture_count"],
        1
    );
    assert_eq!(
        value["source_blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_fixture_count"],
        9
    );
    assert_eq!(
        value["source_denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary_count"],
        62
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
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary_report_sha256",
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary_hash_sha256",
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_policy_hash_sha256",
        "source_dry_run_execution_result_receipt_retention_expiry_garbage_collection_result_hash_sha256",
        "source_dry_run_execution_result_receipt_retention_expiry_garbage_collection_handoff_hash_sha256",
        "dry_run_execution_result_receipt_export_denial_hash_sha256",
        "dry_run_execution_result_receipt_query_denial_hash_sha256",
        "dry_run_execution_result_receipt_observability_denial_hash_sha256",
        "dry_run_execution_result_receipt_export_query_observability_denial_matrix_hash_sha256",
        "dry_run_execution_result_receipt_export_query_observability_handoff_hash_sha256",
        "dry_run_execution_result_receipt_export_query_observability_result_hash_sha256",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_boundary_hash_sha256",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_policy_hash_sha256",
    ] {
        assert_ne!(
            value[key], "",
            "dry-run execution result receipt export/query/observability denial hash should be present: {key}"
        );
    }
    assert_eq!(
        value["required_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_surface_count"],
        16
    );
    assert_eq!(
        value["ready_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_surface_count"],
        16
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_fixture_count"],
        10
    );
    assert_eq!(
        value["accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_fixture_count"],
        1
    );
    assert_eq!(
        value["blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_fixture_count"],
        9
    );
    assert_eq!(
        value["denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_boundary_count"],
        64
    );
    for key in [
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_boundary_performed_count",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_result_recorded_count",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_result_accepted_count",
        "source_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary_accepted_count",
        "dry_run_execution_result_receipt_export_query_observability_denial_matrix_bound_count",
        "dry_run_execution_result_receipt_export_request_denied_count",
        "dry_run_execution_result_receipt_export_file_stream_denied_count",
        "dry_run_execution_result_receipt_query_registration_execution_denied_count",
        "dry_run_execution_result_receipt_query_index_cache_denied_count",
        "dry_run_execution_result_receipt_observability_metric_log_trace_event_denied_count",
        "dry_run_execution_result_receipt_dashboard_alert_slo_denied_count",
        "dry_run_execution_result_receipt_operator_summary_readback_denied_count",
        "dry_run_execution_result_receipt_export_query_observability_authority_denied_count",
        "dry_run_execution_result_receipt_export_query_observability_handoff_bound_count",
    ] {
        assert_eq!(
            value[key], 1,
            "export/query/observability denial count should be one: {key}"
        );
    }
    for key in [
        "dry_run_execution_result_receipt_export_recorded_count",
        "dry_run_execution_result_receipt_export_persisted_count",
        "dry_run_execution_result_receipt_export_snapshot_materialized_count",
        "dry_run_execution_result_receipt_export_file_written_count",
        "dry_run_execution_result_receipt_export_stream_opened_count",
        "dry_run_execution_result_receipt_query_registered_count",
        "dry_run_execution_result_receipt_query_executed_count",
        "dry_run_execution_result_receipt_query_result_recorded_count",
        "dry_run_execution_result_receipt_query_index_recorded_count",
        "dry_run_execution_result_receipt_query_cache_written_count",
        "dry_run_execution_result_receipt_observability_metric_recorded_count",
        "dry_run_execution_result_receipt_observability_log_recorded_count",
        "dry_run_execution_result_receipt_observability_trace_recorded_count",
        "dry_run_execution_result_receipt_observability_event_recorded_count",
        "dry_run_execution_result_receipt_observability_dashboard_materialized_count",
        "dry_run_execution_result_receipt_observability_alert_registered_count",
        "dry_run_execution_result_receipt_observability_slo_recorded_count",
        "dry_run_execution_result_receipt_operator_summary_recorded_count",
        "dry_run_execution_result_receipt_readback_evidence_recorded_count",
        "dry_run_execution_result_receipt_authority_promoted_from_export_count",
        "dry_run_execution_result_receipt_authority_promoted_from_query_count",
        "dry_run_execution_result_receipt_authority_promoted_from_observability_count",
        "dry_run_execution_result_receipt_persisted_count",
        "dry_run_execution_executed_count",
        "production_durable_memory_write_executed_count",
        "production_durable_memory_store_write_performed_count",
        "actual_production_durable_memory_write_performed_count",
        "durable_memory_store_write_performed_count",
        "memory_store_write_performed_count",
        "wal_write_performed_count",
        "receipt_persisted_count",
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
            "export/query/observability side-effect count should stay zero: {key}"
        );
    }
    for key in [
        "source_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary_bound",
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
        "production_write_execution_forbidden_on_export_query_observability_route",
        "production_durable_memory_write_forbidden",
        "memory_store_mutation_forbidden",
        "kg_live_write_forbidden",
        "provider_model_invocation_forbidden",
        "credential_channel_public_release_forbidden",
        "install_restart_active_binary_mutation_forbidden",
    ] {
        assert_eq!(
            value[key], true,
            "export/query/observability denial field should be true: {key}"
        );
    }
    for key in [
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
        "dry_run_execution_executed",
        "production_durable_memory_write_executed",
        "production_durable_memory_store_write_performed",
        "actual_production_durable_memory_write_performed",
        "durable_memory_store_write_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "wal_write_performed",
        "receipt_persisted",
        "post_write_readback_performed",
        "rollback_executed",
        "rollback_performed",
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
            "export/query/observability reporting, authority, mutation, execution, or external field should stay false: {key}"
        );
    }
    let fixtures = value
            ["scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_fixtures"]
            .as_array()
            .expect(
                "scoped production durable Memory write dry-run execution result receipt export/query/observability denial fixtures",
            );
    assert_eq!(fixtures.len(), 10);
    assert_eq!(
            fixtures
                .iter()
                .filter(|fixture| {
                    fixture["scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_accepted"]
                        == true
                })
                .count(),
            1
        );
    let denied = value
            ["denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_boundary"]
            .as_array()
            .expect("scoped production durable Memory write dry-run execution result receipt export/query/observability denials");
    assert_eq!(denied.len(), 64);
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "run_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_boundary_require_live_gate"
    );
    assert_eq!(value["allowed_next_actions"][0]["exports_receipt"], false);
    assert_eq!(value["allowed_next_actions"][0]["registers_query"], false);
    assert_eq!(
        value["allowed_next_actions"][0]["records_observability"],
        false
    );
    assert_eq!(
        value["allowed_next_actions"][0]["materializes_dashboard"],
        false
    );
    assert_eq!(
        value["allowed_next_actions"][0]["promotes_authority"],
        false
    );
    assert_eq!(value["allowed_next_actions"][0]["executes_dry_run"], false);
    assert_eq!(
        value["allowed_next_actions"][0]["writes_production_durable_memory"],
        false
    );
    assert_eq!(
        value["allowed_next_actions"][1]["action"],
        "prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary"
    );
    assert_eq!(
        value["allowed_next_actions"][1]["requires_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_boundary"],
        true
    );
    let side_effects = value["side_effects"].as_object().expect(
            "scoped production durable Memory write dry-run execution result receipt export/query/observability side effects",
        );
    assert_eq!(
            side_effects["scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_boundary_performed"].as_bool(),
            Some(true)
        );
    assert_eq!(
            side_effects["scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_result_accepted"].as_bool(),
            Some(true)
        );
    assert_eq!(
        side_effects["dry_run_execution_result_receipt_export_recorded"].as_bool(),
        Some(false)
    );
    assert_eq!(
        side_effects["dry_run_execution_result_receipt_query_registered"].as_bool(),
        Some(false)
    );
    assert_eq!(
        side_effects["dry_run_execution_result_receipt_observability_metric_recorded"].as_bool(),
        Some(false)
    );
    assert_eq!(
        side_effects["dry_run_execution_result_receipt_observability_dashboard_materialized"]
            .as_bool(),
        Some(false)
    );
    assert_eq!(
        side_effects["dry_run_execution_result_receipt_authority_promoted_from_observability"]
            .as_bool(),
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
fn hepta_memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_summary_briefing_non_persistence_denial_blocks_delivery_ack_authority_execution_and_production_side_effects()
 {
    let body = route_contract_body(HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_PRODUCTION_DURABLE_MEMORY_WRITE_DRY_RUN_EXECUTION_RESULT_RECEIPT_OPERATOR_FACING_SUMMARY_BRIEFING_NON_PERSISTENCE_DENIAL_BOUNDARY_ENDPOINT);

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "scoped production durable Memory write dry-run execution result receipt operator summary/briefing non-persistence denial json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_PRODUCTION_DURABLE_MEMORY_WRITE_DRY_RUN_EXECUTION_RESULT_RECEIPT_OPERATOR_FACING_SUMMARY_BRIEFING_NON_PERSISTENCE_DENIAL_BOUNDARY_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-operator-facing-summary-briefing-non-persistence-denial-boundary --json"
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
        value["memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_ready"],
        true
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_accepted"],
        true
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_mode"],
        "dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_no_summary_no_briefing_no_delivery_no_ack_no_authority_no_execution_no_production_durable_memory_mutation"
    );
    assert_eq!(
        value["source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_boundary_ready"],
        true
    );
    assert_eq!(
        value["source_accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_fixture_count"],
        1
    );
    assert_eq!(
        value["source_blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_fixture_count"],
        9
    );
    assert_eq!(
        value["source_denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_boundary_count"],
        64
    );
    for key in [
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_boundary_report_sha256",
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_boundary_hash_sha256",
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_export_query_observability_denial_policy_hash_sha256",
        "source_dry_run_execution_result_receipt_export_query_observability_result_hash_sha256",
        "source_dry_run_execution_result_receipt_export_query_observability_handoff_hash_sha256",
        "dry_run_execution_result_receipt_operator_summary_denial_hash_sha256",
        "dry_run_execution_result_receipt_operator_briefing_denial_hash_sha256",
        "dry_run_execution_result_receipt_operator_readout_ack_denial_hash_sha256",
        "dry_run_execution_result_receipt_operator_summary_briefing_denial_matrix_hash_sha256",
        "dry_run_execution_result_receipt_operator_summary_briefing_handoff_hash_sha256",
        "dry_run_execution_result_receipt_operator_summary_briefing_result_hash_sha256",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_hash_sha256",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_policy_hash_sha256",
    ] {
        assert_ne!(
            value[key], "",
            "operator summary/briefing hash missing: {key}"
        );
    }
    assert_eq!(
        value["required_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_surface_count"],
        14
    );
    assert_eq!(
        value["ready_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_surface_count"],
        14
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_fixture_count"],
        10
    );
    assert_eq!(
        value["accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_fixture_count"],
        1
    );
    assert_eq!(
        value["blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_fixture_count"],
        9
    );
    assert!(
            value["denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_count"]
                .as_u64()
                .unwrap_or(0)
                >= 60
        );
    for key in [
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_performed_count",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_result_recorded_count",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_result_accepted_count",
        "source_dry_run_execution_result_receipt_export_query_observability_denial_boundary_bound_count",
        "dry_run_execution_result_receipt_operator_facing_summary_request_denied_count",
        "dry_run_execution_result_receipt_operator_briefing_request_denied_count",
        "dry_run_execution_result_receipt_operator_summary_briefing_materialization_denied_count",
        "dry_run_execution_result_receipt_operator_summary_briefing_persistence_denied_count",
        "dry_run_execution_result_receipt_operator_summary_briefing_delivery_denied_count",
        "dry_run_execution_result_receipt_operator_readout_handoff_denied_count",
        "dry_run_execution_result_receipt_final_acknowledgement_decision_status_denied_count",
        "dry_run_execution_result_receipt_operator_summary_briefing_authority_denied_count",
        "dry_run_execution_result_receipt_operator_summary_briefing_handoff_bound_count",
    ] {
        assert_eq!(
            value[key], 1,
            "operator summary/briefing denial count should be one: {key}"
        );
    }
    for key in [
        "dry_run_execution_result_receipt_operator_facing_summary_recorded_count",
        "dry_run_execution_result_receipt_operator_facing_summary_persisted_count",
        "dry_run_execution_result_receipt_operator_facing_summary_materialized_count",
        "dry_run_execution_result_receipt_operator_facing_summary_filesystem_written_count",
        "dry_run_execution_result_receipt_operator_facing_summary_delivered_count",
        "dry_run_execution_result_receipt_operator_briefing_recorded_count",
        "dry_run_execution_result_receipt_operator_briefing_persisted_count",
        "dry_run_execution_result_receipt_operator_briefing_materialized_count",
        "dry_run_execution_result_receipt_operator_briefing_filesystem_written_count",
        "dry_run_execution_result_receipt_operator_briefing_delivered_count",
        "dry_run_execution_result_receipt_operator_readout_recorded_count",
        "dry_run_execution_result_receipt_operator_handoff_recorded_count",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_recorded_count",
        "dry_run_execution_result_receipt_terminal_operator_decision_recorded_count",
        "dry_run_execution_result_receipt_terminal_operator_status_recorded_count",
        "dry_run_execution_result_receipt_authority_promoted_from_operator_summary_count",
        "dry_run_execution_result_receipt_authority_promoted_from_operator_briefing_count",
        "dry_run_execution_result_receipt_authority_promoted_from_final_acknowledgement_count",
        "dry_run_execution_executed_count",
        "production_durable_memory_write_executed_count",
        "production_durable_memory_store_write_performed_count",
        "memory_store_write_performed_count",
        "wal_write_performed_count",
        "receipt_persisted_count",
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
            "operator summary/briefing side-effect count should stay zero: {key}"
        );
    }
    for key in [
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
        "dry_run_execution_result_receipt_final_operator_acknowledgement_recorded",
        "dry_run_execution_result_receipt_terminal_operator_decision_recorded",
        "dry_run_execution_result_receipt_terminal_operator_status_recorded",
        "dry_run_execution_result_receipt_authority_promoted_from_operator_summary",
        "dry_run_execution_result_receipt_authority_promoted_from_operator_briefing",
        "dry_run_execution_result_receipt_authority_promoted_from_final_acknowledgement",
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
        "install_executed",
        "service_restarted",
        "active_binary_mutated",
    ] {
        assert_eq!(
            value[key], false,
            "operator summary/briefing reporting, ack, authority, mutation, execution, or external field should stay false: {key}"
        );
    }
    let fixtures = value
            ["scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_fixtures"]
            .as_array()
            .expect(
                "scoped production durable Memory write dry-run execution result receipt operator summary/briefing denial fixtures",
            );
    assert_eq!(fixtures.len(), 10);
    assert_eq!(
            fixtures
                .iter()
                .filter(|fixture| {
                    fixture["scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_accepted"]
                        == true
                })
                .count(),
            1
        );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["telegram_send_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["final_operator_acknowledgement_requested"] == true
                    && fixture["terminal_operator_decision_requested"] == true
                    && fixture["terminal_operator_status_requested"] == true
            })
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["authority_promotion_requested"] == true
                    && fixture["production_write_requested"] == true
                    && fixture["active_binary_summary_requested"] == true
            })
            .count(),
        1
    );
    let denied = value
            ["denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary"]
            .as_array()
            .expect("scoped production durable Memory write dry-run execution result receipt operator summary/briefing denials");
    assert!(denied.len() >= 60);
    assert_eq!(
        value["denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "run_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_require_live_gate"
    );
    assert_eq!(
        value["allowed_next_actions"][0]["delivers_notification"],
        false
    );
    assert_eq!(
        value["allowed_next_actions"][1]["action"],
        "prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary"
    );
    assert_eq!(
        value["allowed_next_actions"][1]["accepts_operator_acknowledgement"],
        false
    );
    let side_effects = value["side_effects"].as_object().expect(
            "scoped production durable Memory write dry-run execution result receipt operator summary/briefing side effects",
        );
    assert_eq!(
            side_effects["scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_performed"].as_bool(),
            Some(true)
        );
    assert_eq!(
            side_effects["scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_result_accepted"].as_bool(),
            Some(true)
        );
    for key in [
        "dry_run_execution_result_receipt_operator_facing_summary_recorded",
        "dry_run_execution_result_receipt_operator_briefing_recorded",
        "dry_run_execution_result_receipt_operator_readout_recorded",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_recorded",
        "dry_run_execution_result_receipt_authority_promoted_from_final_acknowledgement",
        "dry_run_execution_executed",
        "production_durable_memory_store_write_performed",
        "memory_store_write_performed",
        "wal_write_performed",
        "receipt_persisted",
        "external_send_performed",
    ] {
        assert_eq!(side_effects[key].as_bool(), Some(false), "{key}");
    }
}

#[test]
fn hepta_memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_blocks_ack_decision_authority_execution_and_production_side_effects()
 {
    let body = route_contract_body(HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_PRODUCTION_DURABLE_MEMORY_WRITE_DRY_RUN_EXECUTION_RESULT_RECEIPT_FINAL_OPERATOR_ACKNOWLEDGEMENT_NON_ACCEPTANCE_DENIAL_BOUNDARY_ENDPOINT);

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "scoped production durable Memory write dry-run execution result receipt final operator acknowledgement non-acceptance denial json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_PRODUCTION_DURABLE_MEMORY_WRITE_DRY_RUN_EXECUTION_RESULT_RECEIPT_FINAL_OPERATOR_ACKNOWLEDGEMENT_NON_ACCEPTANCE_DENIAL_BOUNDARY_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-final-operator-acknowledgement-non-acceptance-denial-boundary --json"
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
        value["memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_ready"],
        true
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_accepted"],
        true
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_mode"],
        "dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_no_ack_acceptance_no_terminal_decision_no_authority_no_execution_no_production_durable_memory_mutation"
    );
    assert_eq!(
        value["source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_ready"],
        true
    );
    assert_eq!(
        value["source_accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_fixture_count"],
        1
    );
    assert_eq!(
        value["source_blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_fixture_count"],
        9
    );
    assert!(
            value["source_denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_count"]
                .as_u64()
                .unwrap_or(0)
                >= 60
        );
    for key in [
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_report_sha256",
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_hash_sha256",
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_policy_hash_sha256",
        "source_dry_run_execution_result_receipt_operator_summary_briefing_result_hash_sha256",
        "source_dry_run_execution_result_receipt_operator_summary_briefing_handoff_hash_sha256",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_denial_hash_sha256",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_readback_denial_hash_sha256",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_receipt_denial_hash_sha256",
        "dry_run_execution_result_receipt_terminal_operator_decision_status_denial_hash_sha256",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_matrix_hash_sha256",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_handoff_hash_sha256",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_result_hash_sha256",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_hash_sha256",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_policy_hash_sha256",
    ] {
        assert_ne!(
            value[key], "",
            "final operator acknowledgement denial hash missing: {key}"
        );
    }
    assert_eq!(
        value["required_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_surface_count"],
        14
    );
    assert_eq!(
        value["ready_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_surface_count"],
        14
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_fixture_count"],
        10
    );
    assert_eq!(
        value["accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_fixture_count"],
        1
    );
    assert_eq!(
        value["blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_fixture_count"],
        9
    );
    assert!(
            value["denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_count"]
                .as_u64()
                .unwrap_or(0)
                >= 64
        );
    for key in [
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_performed_count",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_result_recorded_count",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_result_accepted_count",
        "source_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_bound_count",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_request_denied_count",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_bound_count",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_readback_denied_count",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_receipt_denied_count",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_delivery_denied_count",
        "dry_run_execution_result_receipt_terminal_operator_decision_status_denied_count",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_authority_denied_count",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_handoff_bound_count",
    ] {
        assert_eq!(
            value[key], 1,
            "final operator acknowledgement denial count should be one: {key}"
        );
    }
    for key in [
        "dry_run_execution_result_receipt_final_operator_acknowledgement_recorded_count",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_accepted_count",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_persisted_count",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_materialized_count",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_filesystem_written_count",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_delivered_count",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_readback_recorded_count",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_receipt_persisted_count",
        "dry_run_execution_result_receipt_terminal_operator_decision_recorded_count",
        "dry_run_execution_result_receipt_terminal_operator_status_recorded_count",
        "dry_run_execution_result_receipt_terminal_operator_decision_promoted_count",
        "dry_run_execution_result_receipt_terminal_operator_status_promoted_count",
        "dry_run_execution_result_receipt_authority_promoted_from_final_acknowledgement_count",
        "dry_run_execution_executed_count",
        "production_durable_memory_write_executed_count",
        "production_durable_memory_store_write_performed_count",
        "memory_store_write_performed_count",
        "wal_write_performed_count",
        "receipt_persisted_count",
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
            "final acknowledgement side-effect count should stay zero: {key}"
        );
    }
    for key in [
        "dry_run_execution_result_receipt_final_operator_acknowledgement_requested",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_recorded",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_accepted",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_persisted",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_materialized",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_filesystem_written",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_delivered",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_readback_recorded",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_receipt_persisted",
        "dry_run_execution_result_receipt_terminal_operator_decision_recorded",
        "dry_run_execution_result_receipt_terminal_operator_status_recorded",
        "dry_run_execution_result_receipt_terminal_operator_decision_promoted",
        "dry_run_execution_result_receipt_terminal_operator_status_promoted",
        "dry_run_execution_result_receipt_authority_promoted_from_final_acknowledgement",
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
        "install_executed",
        "service_restarted",
        "active_binary_mutated",
    ] {
        assert_eq!(
            value[key], false,
            "final acknowledgement, decision, authority, mutation, execution, or external field should stay false: {key}"
        );
    }
    let fixtures = value
            ["scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_fixtures"]
            .as_array()
            .expect(
                "scoped production durable Memory write dry-run execution result receipt final operator acknowledgement denial fixtures",
            );
    assert_eq!(fixtures.len(), 10);
    assert_eq!(
            fixtures
                .iter()
                .filter(|fixture| {
                    fixture["scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_accepted"]
                        == true
                })
                .count(),
            1
        );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["source_operator_summary_briefing_present"] == false)
            .count(),
        1
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
                fixture["terminal_operator_decision_requested"] == true
                    && fixture["terminal_operator_status_requested"] == true
                    && fixture["terminal_operator_decision_promotion_requested"] == true
            })
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["authority_promotion_requested"] == true
                    && fixture["production_write_requested"] == true
                    && fixture["active_binary_acknowledgement_requested"] == true
            })
            .count(),
        1
    );
    let denied = value
            ["denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary"]
            .as_array()
            .expect("scoped production durable Memory write dry-run execution result receipt final acknowledgement denials");
    assert!(denied.len() >= 64);
    assert_eq!(
        value["denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "run_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_require_live_gate"
    );
    assert_eq!(
        value["allowed_next_actions"][0]["accepts_operator_acknowledgement"],
        false
    );
    assert_eq!(
        value["allowed_next_actions"][1]["action"],
        "prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary"
    );
    assert_eq!(
        value["allowed_next_actions"][1]["accepts_terminal_decision"],
        false
    );
    let side_effects = value["side_effects"].as_object().expect(
            "scoped production durable Memory write dry-run execution result receipt final acknowledgement side effects",
        );
    assert_eq!(
            side_effects["scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_performed"].as_bool(),
            Some(true)
        );
    assert_eq!(
            side_effects["scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_result_accepted"].as_bool(),
            Some(true)
        );
    for key in [
        "dry_run_execution_result_receipt_final_operator_acknowledgement_recorded",
        "dry_run_execution_result_receipt_final_operator_acknowledgement_accepted",
        "dry_run_execution_result_receipt_terminal_operator_decision_recorded",
        "dry_run_execution_result_receipt_terminal_operator_status_recorded",
        "dry_run_execution_result_receipt_authority_promoted_from_final_acknowledgement",
        "dry_run_execution_executed",
        "production_durable_memory_store_write_performed",
        "memory_store_write_performed",
        "wal_write_performed",
        "receipt_persisted",
        "external_send_performed",
    ] {
        assert_eq!(side_effects[key].as_bool(), Some(false), "{key}");
    }
}

#[test]
fn hepta_memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_blocks_terminal_decision_public_claim_authority_execution_and_production_side_effects()
 {
    let body = route_contract_body(HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_PRODUCTION_DURABLE_MEMORY_WRITE_DRY_RUN_EXECUTION_RESULT_RECEIPT_TERMINAL_OPERATOR_DECISION_PUBLIC_CLAIM_NON_PROMOTION_DENIAL_BOUNDARY_ENDPOINT);

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "scoped production durable Memory write dry-run execution result receipt terminal operator decision public claim non-promotion denial json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_PRODUCTION_DURABLE_MEMORY_WRITE_DRY_RUN_EXECUTION_RESULT_RECEIPT_TERMINAL_OPERATOR_DECISION_PUBLIC_CLAIM_NON_PROMOTION_DENIAL_BOUNDARY_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial-boundary --json"
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
        value["memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_ready"],
        true
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_accepted"],
        true
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_mode"],
        "dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_no_terminal_decision_no_public_claim_no_authority_no_execution_no_production_durable_memory_mutation"
    );
    assert_eq!(
        value["source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_ready"],
        true
    );
    assert_eq!(
        value["source_accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_fixture_count"],
        1
    );
    assert_eq!(
        value["source_blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_fixture_count"],
        9
    );
    assert!(
            value["source_denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_count"]
                .as_u64()
                .unwrap_or(0)
                >= 64
        );
    for key in [
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_report_sha256",
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_hash_sha256",
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_policy_hash_sha256",
        "source_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_result_hash_sha256",
        "source_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_handoff_hash_sha256",
        "dry_run_execution_result_receipt_terminal_operator_decision_denial_hash_sha256",
        "dry_run_execution_result_receipt_public_claim_denial_hash_sha256",
        "dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_matrix_hash_sha256",
        "dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_handoff_hash_sha256",
        "dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_result_hash_sha256",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_hash_sha256",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_policy_hash_sha256",
    ] {
        assert_ne!(
            value[key], "",
            "terminal decision public claim denial hash missing: {key}"
        );
    }
    assert_eq!(
        value["required_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_surface_count"],
        14
    );
    assert_eq!(
        value["ready_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_surface_count"],
        14
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_fixture_count"],
        10
    );
    assert_eq!(
        value["accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_fixture_count"],
        1
    );
    assert_eq!(
        value["blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_fixture_count"],
        9
    );
    assert!(
            value["denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_count"]
                .as_u64()
                .unwrap_or(0)
                >= 80
        );
    for key in [
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_performed_count",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_result_accepted_count",
        "source_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_bound_count",
        "dry_run_execution_result_receipt_terminal_operator_decision_request_denied_count",
        "dry_run_execution_result_receipt_terminal_operator_decision_status_denied_count",
        "dry_run_execution_result_receipt_public_claim_denied_count",
        "dry_run_execution_result_receipt_public_release_publication_denied_count",
        "dry_run_execution_result_receipt_terminal_operator_decision_public_claim_authority_denied_count",
        "dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_handoff_bound_count",
    ] {
        assert_eq!(
            value[key], 1,
            "terminal decision public claim denial count should be one: {key}"
        );
    }
    for key in [
        "dry_run_execution_result_receipt_terminal_operator_decision_recorded_count",
        "dry_run_execution_result_receipt_terminal_operator_decision_accepted_count",
        "dry_run_execution_result_receipt_terminal_operator_decision_persisted_count",
        "dry_run_execution_result_receipt_terminal_operator_status_recorded_count",
        "dry_run_execution_result_receipt_public_claim_recorded_count",
        "dry_run_execution_result_receipt_public_claim_promoted_count",
        "dry_run_execution_result_receipt_public_ga_claimed_count",
        "dry_run_execution_result_receipt_public_release_published_count",
        "dry_run_execution_result_receipt_release_artifact_written_count",
        "dry_run_execution_result_receipt_authority_promoted_from_terminal_operator_decision_count",
        "dry_run_execution_executed_count",
        "production_durable_memory_write_executed_count",
        "production_durable_memory_store_write_performed_count",
        "memory_store_write_performed_count",
        "wal_write_performed_count",
        "receipt_persisted_count",
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
            "terminal decision, public claim, execution, or mutation count should stay zero: {key}"
        );
    }
    for key in [
        "dry_run_execution_result_receipt_terminal_operator_decision_requested",
        "dry_run_execution_result_receipt_terminal_operator_decision_recorded",
        "dry_run_execution_result_receipt_terminal_operator_decision_accepted",
        "dry_run_execution_result_receipt_terminal_operator_decision_persisted",
        "dry_run_execution_result_receipt_terminal_operator_status_recorded",
        "dry_run_execution_result_receipt_terminal_operator_status_persisted",
        "dry_run_execution_result_receipt_terminal_operator_decision_promoted",
        "dry_run_execution_result_receipt_terminal_operator_status_promoted",
        "dry_run_execution_result_receipt_public_claim_recorded",
        "dry_run_execution_result_receipt_public_claim_promoted",
        "dry_run_execution_result_receipt_public_ga_claimed",
        "dry_run_execution_result_receipt_public_release_published",
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
        "install_executed",
        "service_restarted",
        "active_binary_mutated",
    ] {
        assert_eq!(
            value[key], false,
            "terminal decision, public claim, authority, mutation, execution, or external field should stay false: {key}"
        );
    }
    let fixtures = value
            ["scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_fixtures"]
            .as_array()
            .expect(
                "scoped production durable Memory write dry-run execution result receipt terminal operator decision public claim denial fixtures",
            );
    assert_eq!(fixtures.len(), 10);
    assert_eq!(
            fixtures
                .iter()
                .filter(|fixture| {
                    fixture["scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_accepted"]
                        == true
                })
                .count(),
            1
        );
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
            .filter(|fixture| { fixture["terminal_operator_decision_requested"] == true })
            .count(),
        8
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["terminal_operator_decision_requested"] == true
                    && fixture["terminal_operator_decision_recording_requested"] == true
            })
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["terminal_operator_status_requested"] == true
                    && fixture["terminal_operator_status_recording_requested"] == true
            })
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["public_claim_requested"] == true
                    && fixture["public_ga_claim_requested"] == true
                    && fixture["public_release_claim_requested"] == true
            })
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["release_artifact_write_requested"] == true
                    && fixture["public_artifact_write_requested"] == true
            })
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["install_decision_requested"] == true
                    && fixture["service_restart_decision_requested"] == true
                    && fixture["active_binary_decision_requested"] == true
            })
            .count(),
        1
    );
    let denied = value
            ["denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary"]
            .as_array()
            .expect("scoped production durable Memory write dry-run execution result receipt terminal decision public claim denials");
    assert!(denied.len() >= 80);
    assert_eq!(
        value["denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "run_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_require_live_gate"
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
        "prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary"
    );
    assert_eq!(
        value["allowed_next_actions"][1]["writes_release_artifact"],
        false
    );
    let side_effects = value["side_effects"].as_object().expect(
            "scoped production durable Memory write dry-run execution result receipt terminal operator decision public claim side effects",
        );
    assert_eq!(
            side_effects["scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_performed"].as_bool(),
            Some(true)
        );
    assert_eq!(
            side_effects["scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_result_accepted"].as_bool(),
            Some(true)
        );
    for key in [
        "dry_run_execution_result_receipt_terminal_operator_decision_recorded",
        "dry_run_execution_result_receipt_terminal_operator_decision_accepted",
        "dry_run_execution_result_receipt_terminal_operator_status_recorded",
        "dry_run_execution_result_receipt_public_claim_recorded",
        "dry_run_execution_result_receipt_public_claim_promoted",
        "dry_run_execution_result_receipt_public_release_published",
        "dry_run_execution_result_receipt_authority_promoted_from_terminal_operator_decision",
        "dry_run_execution_executed",
        "production_durable_memory_store_write_performed",
        "memory_store_write_performed",
        "wal_write_performed",
        "receipt_persisted",
        "external_send_performed",
        "release_artifact_written",
        "install_executed",
        "service_restarted",
        "active_binary_mutated",
    ] {
        assert_eq!(side_effects[key].as_bool(), Some(false), "{key}");
    }
}

#[test]
fn hepta_memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_blocks_publication_artifacts_authority_execution_and_production_side_effects()
 {
    let body = route_contract_body(HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_PRODUCTION_DURABLE_MEMORY_WRITE_DRY_RUN_EXECUTION_RESULT_RECEIPT_RELEASE_ARTIFACT_PUBLICATION_DENIAL_BOUNDARY_ENDPOINT);

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "scoped production durable Memory write dry-run execution result receipt release artifact publication denial json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_PRODUCTION_DURABLE_MEMORY_WRITE_DRY_RUN_EXECUTION_RESULT_RECEIPT_RELEASE_ARTIFACT_PUBLICATION_DENIAL_BOUNDARY_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-release-artifact-publication-denial-boundary --json"
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
        value["memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary_ready"],
        true
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_accepted"],
        true
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_mode"],
        "dry_run_execution_result_receipt_release_artifact_publication_denial_boundary_no_release_artifact_no_publication_no_authority_no_execution_no_production_durable_memory_mutation"
    );
    assert_eq!(
        value["source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_ready"],
        true
    );
    assert_eq!(
        value["source_accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_fixture_count"],
        1
    );
    assert_eq!(
        value["source_blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_fixture_count"],
        9
    );
    assert!(
            value["source_denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_count"]
                .as_u64()
                .unwrap_or(0)
                >= 80
        );
    for key in [
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_report_sha256",
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_hash_sha256",
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_policy_hash_sha256",
        "source_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_result_hash_sha256",
        "source_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_handoff_hash_sha256",
        "dry_run_execution_result_receipt_release_artifact_publication_denial_hash_sha256",
        "dry_run_execution_result_receipt_release_artifact_write_denial_hash_sha256",
        "dry_run_execution_result_receipt_release_artifact_publication_matrix_hash_sha256",
        "dry_run_execution_result_receipt_release_artifact_publication_handoff_hash_sha256",
        "dry_run_execution_result_receipt_release_artifact_publication_result_hash_sha256",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary_hash_sha256",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_policy_hash_sha256",
    ] {
        assert_ne!(
            value[key], "",
            "release artifact publication denial hash missing: {key}"
        );
    }
    assert_eq!(
        value["required_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_surface_count"],
        14
    );
    assert_eq!(
        value["ready_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_surface_count"],
        14
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_fixture_count"],
        10
    );
    assert_eq!(
        value["accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_fixture_count"],
        1
    );
    assert_eq!(
        value["blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_fixture_count"],
        9
    );
    assert!(
            value["denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary_count"]
                .as_u64()
                .unwrap_or(0)
                >= 100
        );
    for key in [
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary_performed_count",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_result_accepted_count",
        "source_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_bound_count",
        "dry_run_execution_result_receipt_release_artifact_publication_denied_count",
        "dry_run_execution_result_receipt_release_artifact_write_denied_count",
        "dry_run_execution_result_receipt_public_artifact_write_denied_count",
        "dry_run_execution_result_receipt_artifact_signature_notarization_denied_count",
        "dry_run_execution_result_receipt_publication_queue_manifest_denied_count",
        "dry_run_execution_result_receipt_public_distribution_denied_count",
        "dry_run_execution_result_receipt_public_release_publication_denied_count",
        "dry_run_execution_result_receipt_release_artifact_publication_authority_denied_count",
        "dry_run_execution_result_receipt_release_artifact_publication_handoff_bound_count",
    ] {
        assert_eq!(
            value[key], 1,
            "release artifact publication denial count should be one: {key}"
        );
    }
    for key in [
        "dry_run_execution_result_receipt_release_artifact_publication_recorded_count",
        "dry_run_execution_result_receipt_release_artifact_publication_accepted_count",
        "dry_run_execution_result_receipt_release_artifact_written_count",
        "dry_run_execution_result_receipt_public_artifact_written_count",
        "dry_run_execution_result_receipt_artifact_signature_accepted_count",
        "dry_run_execution_result_receipt_artifact_notarization_accepted_count",
        "dry_run_execution_result_receipt_publication_queue_enqueued_count",
        "dry_run_execution_result_receipt_publication_manifest_written_count",
        "dry_run_execution_result_receipt_public_distribution_performed_count",
        "dry_run_execution_result_receipt_public_release_published_count",
        "dry_run_execution_result_receipt_public_ga_claimed_count",
        "dry_run_execution_result_receipt_authority_promoted_from_release_artifact_publication_count",
        "activation_performed_count",
        "dry_run_execution_executed_count",
        "production_durable_memory_write_executed_count",
        "production_durable_memory_store_write_performed_count",
        "memory_store_write_performed_count",
        "wal_write_performed_count",
        "receipt_persisted_count",
        "live_kg_write_performed_count",
        "provider_invoked_count",
        "model_invoked_count",
        "credential_read_count",
        "channel_send_performed_count",
        "external_send_performed_count",
        "release_artifact_written_count",
        "public_artifact_written_count",
        "install_executed_count",
        "service_restarted_count",
        "active_binary_mutated_count",
    ] {
        assert_eq!(
            value[key], 0,
            "release artifact, publication, authority, execution, or mutation count should stay zero: {key}"
        );
    }
    for key in [
        "dry_run_execution_result_receipt_release_artifact_publication_recorded",
        "dry_run_execution_result_receipt_release_artifact_publication_accepted",
        "dry_run_execution_result_receipt_release_artifact_written",
        "dry_run_execution_result_receipt_public_artifact_written",
        "dry_run_execution_result_receipt_artifact_signature_accepted",
        "dry_run_execution_result_receipt_artifact_notarization_accepted",
        "dry_run_execution_result_receipt_publication_queue_enqueued",
        "dry_run_execution_result_receipt_publication_manifest_written",
        "dry_run_execution_result_receipt_public_distribution_performed",
        "dry_run_execution_result_receipt_public_ga_claimed",
        "dry_run_execution_result_receipt_public_release_published",
        "dry_run_execution_result_receipt_terminal_operator_decision_promoted_to_release_approval",
        "dry_run_execution_result_receipt_authority_promoted_from_release_artifact_publication",
        "activation_performed",
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
        "install_executed",
        "service_restarted",
        "active_binary_mutated",
    ] {
        assert_eq!(
            value[key], false,
            "release artifact, publication, authority, mutation, execution, or external field should stay false: {key}"
        );
    }
    let fixtures = value
            ["scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_fixtures"]
            .as_array()
            .expect(
                "scoped production durable Memory write dry-run execution result receipt release artifact publication denial fixtures",
            );
    assert_eq!(fixtures.len(), 10);
    assert_eq!(
            fixtures
                .iter()
                .filter(|fixture| {
                    fixture["scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_accepted"]
                        == true
                })
                .count(),
            1
        );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["source_terminal_operator_decision_public_claim_present"] == false
            })
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
            .filter(|fixture| {
                fixture["artifact_signature_requested"] == true
                    && fixture["artifact_notarization_requested"] == true
            })
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["service_restart_publication_requested"] == true
                    && fixture["active_binary_publication_requested"] == true
            })
            .count(),
        1
    );
    let denied = value
            ["denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary"]
            .as_array()
            .expect("scoped production durable Memory write dry-run execution result receipt release artifact publication denials");
    assert!(denied.len() >= 100);
    assert_eq!(
        value["denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "run_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary_require_live_gate"
    );
    assert_eq!(
        value["allowed_next_actions"][0]["writes_release_artifact"],
        false
    );
    assert_eq!(
        value["allowed_next_actions"][0]["promotes_activation_authority"],
        false
    );
    assert_eq!(
        value["allowed_next_actions"][1]["action"],
        "prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_boundary"
    );
    assert_eq!(
        value["allowed_next_actions"][1]["publishes_release_artifact"],
        false
    );
    let side_effects = value["side_effects"].as_object().expect(
            "scoped production durable Memory write dry-run execution result receipt release artifact publication side effects",
        );
    assert_eq!(
            side_effects["scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary_performed"].as_bool(),
            Some(true)
        );
    assert_eq!(
            side_effects["scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_result_accepted"].as_bool(),
            Some(true)
        );
    for key in [
        "dry_run_execution_result_receipt_release_artifact_publication_recorded",
        "dry_run_execution_result_receipt_release_artifact_written",
        "dry_run_execution_result_receipt_public_artifact_written",
        "dry_run_execution_result_receipt_publication_queue_enqueued",
        "dry_run_execution_result_receipt_publication_manifest_written",
        "dry_run_execution_result_receipt_public_distribution_performed",
        "dry_run_execution_result_receipt_public_release_published",
        "dry_run_execution_result_receipt_authority_promoted_from_release_artifact_publication",
        "activation_performed",
        "dry_run_execution_executed",
        "production_durable_memory_store_write_performed",
        "memory_store_write_performed",
        "wal_write_performed",
        "receipt_persisted",
        "external_send_performed",
        "release_artifact_written",
        "public_artifact_written",
        "install_executed",
        "service_restarted",
        "active_binary_mutated",
    ] {
        assert_eq!(side_effects[key].as_bool(), Some(false), "{key}");
    }
}

#[test]
fn hepta_memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_blocks_receipt_persistence_publication_authority_execution_and_production_side_effects()
 {
    let body = route_contract_body(HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_PRODUCTION_DURABLE_MEMORY_WRITE_DRY_RUN_EXECUTION_RESULT_RECEIPT_RELEASE_ARTIFACT_PUBLICATION_RESULT_RECEIPT_NO_PERSISTENCE_BOUNDARY_ENDPOINT);

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "scoped production durable Memory write dry-run execution result receipt release artifact publication result receipt no-persistence json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_PRODUCTION_DURABLE_MEMORY_WRITE_DRY_RUN_EXECUTION_RESULT_RECEIPT_RELEASE_ARTIFACT_PUBLICATION_RESULT_RECEIPT_NO_PERSISTENCE_BOUNDARY_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-release-artifact-publication-result-receipt-no-persistence-boundary --json"
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
        value["memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_boundary_ready"],
        true
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_accepted"],
        true
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_mode"],
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_boundary_no_receipt_persistence_no_publication_no_authority_no_execution_no_production_durable_memory_mutation"
    );
    assert_eq!(
        value["source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary_ready"],
        true
    );
    assert_eq!(
        value["source_accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_fixture_count"],
        1
    );
    assert_eq!(
        value["source_blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_fixture_count"],
        9
    );
    assert!(
            value["source_denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary_count"]
                .as_u64()
                .unwrap_or(0)
                >= 100
        );
    for key in [
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary_report_sha256",
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary_hash_sha256",
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_policy_hash_sha256",
        "source_dry_run_execution_result_receipt_release_artifact_publication_result_hash_sha256",
        "source_dry_run_execution_result_receipt_release_artifact_publication_handoff_hash_sha256",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_hash_sha256",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_matrix_hash_sha256",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_handoff_hash_sha256",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_result_hash_sha256",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_boundary_hash_sha256",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_policy_hash_sha256",
    ] {
        assert_ne!(
            value[key], "",
            "publication result receipt no-persistence hash missing: {key}"
        );
    }
    assert_eq!(
        value["required_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_surface_count"],
        14
    );
    assert_eq!(
        value["ready_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_surface_count"],
        14
    );
    assert_eq!(
        value["scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_fixture_count"],
        10
    );
    assert_eq!(
        value["accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_fixture_count"],
        1
    );
    assert_eq!(
        value["blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_fixture_count"],
        9
    );
    assert!(
            value["denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_boundary_count"]
                .as_u64()
                .unwrap_or(0)
                >= 115
        );
    for key in [
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_boundary_performed_count",
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_result_accepted_count",
        "source_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary_bound_count",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_rendered_count",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_recording_denied_count",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_persistence_denied_count",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_ledger_index_denied_count",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_queue_delivery_denied_count",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_export_query_observability_denied_count",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_signature_timestamp_status_denied_count",
        "dry_run_execution_result_receipt_release_artifact_publication_completion_ack_denied_count",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_authority_denied_count",
    ] {
        assert_eq!(
            value[key], 1,
            "publication result receipt no-persistence count should be one: {key}"
        );
    }
    for key in [
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_recorded_count",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_accepted_count",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_persisted_count",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_materialized_count",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_filesystem_written_count",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_ledger_written_count",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_indexed_count",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_queued_count",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_delivered_count",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_exported_count",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_query_registered_count",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_observability_recorded_count",
        "dry_run_execution_result_receipt_release_artifact_publication_completion_ack_recorded_count",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_authority_promoted_count",
        "dry_run_execution_result_receipt_release_artifact_publication_recorded_count",
        "dry_run_execution_result_receipt_release_artifact_written_count",
        "dry_run_execution_result_receipt_public_artifact_written_count",
        "dry_run_execution_result_receipt_public_distribution_performed_count",
        "dry_run_execution_result_receipt_public_release_published_count",
        "dry_run_execution_result_receipt_public_ga_claimed_count",
        "activation_performed_count",
        "dry_run_execution_executed_count",
        "production_durable_memory_store_write_performed_count",
        "memory_store_write_performed_count",
        "wal_write_performed_count",
        "receipt_persisted_count",
        "live_kg_write_performed_count",
        "provider_invoked_count",
        "model_invoked_count",
        "credential_read_count",
        "channel_send_performed_count",
        "external_send_performed_count",
        "release_artifact_written_count",
        "public_artifact_written_count",
        "install_executed_count",
        "service_restarted_count",
        "active_binary_mutated_count",
    ] {
        assert_eq!(
            value[key], 0,
            "publication result receipt, publication, authority, execution, or mutation count should stay zero: {key}"
        );
    }
    for key in [
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_recorded",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_accepted",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_persisted",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_materialized",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_filesystem_written",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_ledger_written",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_indexed",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_queued",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_delivered",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_exported",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_query_registered",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_observability_recorded",
        "dry_run_execution_result_receipt_release_artifact_publication_completion_ack_recorded",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_authority_promoted",
        "dry_run_execution_result_receipt_release_artifact_publication_recorded",
        "dry_run_execution_result_receipt_release_artifact_written",
        "dry_run_execution_result_receipt_public_artifact_written",
        "dry_run_execution_result_receipt_publication_queue_enqueued",
        "dry_run_execution_result_receipt_publication_manifest_written",
        "dry_run_execution_result_receipt_public_distribution_performed",
        "dry_run_execution_result_receipt_public_release_published",
        "dry_run_execution_result_receipt_public_ga_claimed",
        "dry_run_execution_result_receipt_authority_promoted_from_release_artifact_publication",
        "activation_performed",
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
        "install_executed",
        "service_restarted",
        "active_binary_mutated",
    ] {
        assert_eq!(
            value[key], false,
            "publication result receipt, publication, authority, mutation, execution, or external field should stay false: {key}"
        );
    }
    let fixtures = value
            ["scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_fixtures"]
            .as_array()
            .expect(
                "scoped production durable Memory write dry-run execution result receipt release artifact publication result receipt no-persistence fixtures",
            );
    assert_eq!(fixtures.len(), 10);
    assert_eq!(
            fixtures
                .iter()
                .filter(|fixture| {
                    fixture["scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_accepted"]
                        == true
                })
                .count(),
            1
        );
    assert_eq!(
        fixtures
            .iter()
            .filter(
                |fixture| fixture["source_release_artifact_publication_denial_present"] == false
            )
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["publication_result_receipt_requested"] == true)
            .count(),
        9
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["publication_result_receipt_ledger_write_requested"] == true
                    && fixture["publication_result_receipt_index_requested"] == true
            })
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["publication_result_receipt_export_requested"] == true
                    && fixture["publication_result_receipt_query_requested"] == true
                    && fixture["publication_result_receipt_observability_requested"] == true
            })
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["release_artifact_write_requested"] == true
                    && fixture["public_artifact_write_requested"] == true
                    && fixture["public_release_publish_requested"] == true
            })
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["install_publication_result_receipt_requested"] == true
                    && fixture["service_restart_publication_result_receipt_requested"] == true
                    && fixture["active_binary_publication_result_receipt_requested"] == true
            })
            .count(),
        1
    );
    let denied = value
            ["denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_boundary"]
            .as_array()
            .expect("scoped production durable Memory write dry-run execution result receipt release artifact publication result receipt no-persistence denials");
    assert!(denied.len() >= 115);
    assert_eq!(
        value["denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_boundary_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "run_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_boundary_require_live_gate"
    );
    assert_eq!(
        value["allowed_next_actions"][0]["persists_publication_result_receipt"],
        false
    );
    assert_eq!(
        value["allowed_next_actions"][0]["publishes_release_artifact"],
        false
    );
    assert_eq!(
        value["allowed_next_actions"][1]["action"],
        "prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_replay_idempotency_denial_boundary"
    );
    assert_eq!(value["allowed_next_actions"][1]["accepts_replay"], false);
    let side_effects = value["side_effects"].as_object().expect(
            "scoped production durable Memory write dry-run execution result receipt release artifact publication result receipt no-persistence side effects",
        );
    assert_eq!(
            side_effects["scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_boundary_performed"].as_bool(),
            Some(true)
        );
    assert_eq!(
            side_effects["scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_result_accepted"].as_bool(),
            Some(true)
        );
    for key in [
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_recorded",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_persisted",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_ledger_written",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_indexed",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_delivered",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_exported",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_query_registered",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_observability_recorded",
        "dry_run_execution_result_receipt_release_artifact_publication_completion_ack_recorded",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_authority_promoted",
        "dry_run_execution_result_receipt_release_artifact_written",
        "dry_run_execution_result_receipt_public_artifact_written",
        "activation_performed",
        "dry_run_execution_executed",
        "production_durable_memory_store_write_performed",
        "memory_store_write_performed",
        "wal_write_performed",
        "receipt_persisted",
        "external_send_performed",
        "release_artifact_written",
        "public_artifact_written",
        "install_executed",
        "service_restarted",
        "active_binary_mutated",
    ] {
        assert_eq!(side_effects[key].as_bool(), Some(false), "{key}");
    }
}

#[test]
fn hepta_memory_write_execution_minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_writes_reads_and_cleans_canary_artifacts_without_memory_or_external_side_effects()
 {
    let body = route_contract_body(HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_MINIMAL_SCOPED_MEMORY_REAL_WRITE_CANARY_DURABLE_WAL_RECEIPT_PERSISTENCE_BOUNDARY_ENDPOINT);

    let value: serde_json::Value = serde_json::from_str(&body)
        .expect("minimal scoped Memory real-write canary durable WAL/receipt persistence json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_MINIMAL_SCOPED_MEMORY_REAL_WRITE_CANARY_DURABLE_WAL_RECEIPT_PERSISTENCE_BOUNDARY_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-wal-receipt-persistence-boundary --json"
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
        value["memory_write_execution_minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_boundary_ready"],
        true
    );
    assert_eq!(
        value["minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_ready"],
        true
    );
    assert_eq!(
        value["minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_performed"],
        true
    );
    assert_eq!(
        value["scoped_memory_real_write_canary_mode"],
        "minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_artifact_write_readback_cleanup"
    );
    assert_eq!(
        value["source_minimal_scoped_memory_real_write_canary_execution_boundary_ready"],
        true
    );
    assert_eq!(
        value["source_accepted_minimal_scoped_memory_real_write_canary_execution_fixture_count"],
        1
    );
    assert_eq!(
        value["source_blocked_minimal_scoped_memory_real_write_canary_execution_fixture_count"],
        9
    );
    assert_eq!(value["source_memory_store_write_performed_count"], 1);
    assert_eq!(value["source_post_write_readback_performed_count"], 1);
    assert_eq!(value["source_readback_result_accepted_count"], 1);
    assert_eq!(value["source_rollback_performed_count"], 1);
    assert_eq!(value["source_rollback_result_accepted_count"], 1);
    assert_eq!(value["source_wal_write_performed_count"], 0);
    assert_eq!(value["source_receipt_persisted_count"], 0);
    assert_eq!(
        value["source_durable_memory_store_write_performed_count"],
        0
    );
    assert_eq!(value["approved_namespace"], "hepta.memory.canary");
    assert_eq!(value["approved_store"], "wal-receipt-canary-artifact");
    assert_eq!(value["approved_scope"], "session");
    assert_eq!(
        value["wal_record_id"],
        "hepta-minimal-scoped-memory-real-write-canary-durable-wal-record-v1"
    );
    assert_eq!(
        value["receipt_id"],
        "hepta-minimal-scoped-memory-real-write-canary-durable-receipt-v1"
    );
    assert_ne!(value["wal_record_sha256"], "");
    assert_ne!(value["receipt_sha256"], "");
    assert_ne!(value["receipt_hash_chain_sha256"], "");
    assert_eq!(value["receipt_hash_chain_verified"], true);
    assert_eq!(value["canary_payload_plaintext_recorded"], false);
    assert_eq!(value["pre_persistence_artifact_count"], 0);
    assert_eq!(value["post_persistence_artifact_count"], 2);
    assert_eq!(value["cleanup_removed_artifact_count"], 2);
    assert_eq!(value["post_cleanup_artifact_count"], 0);
    assert_eq!(value["canary_artifact_cleanup_confirmed"], true);
    assert_eq!(
        value["required_minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_surface_count"],
        12
    );
    assert_eq!(
        value["ready_minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_surface_count"],
        12
    );
    assert_eq!(
        value["minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_fixture_count"],
        10
    );
    assert_eq!(
        value["accepted_minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_fixture_count"],
        1
    );
    assert_eq!(
        value["blocked_minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_fixture_count"],
        9
    );
    for key in [
        "minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_accepted_count",
        "durable_wal_receipt_persistence_authority_accepted_count",
        "wal_artifact_write_bound_count",
        "wal_artifact_readback_bound_count",
        "receipt_artifact_write_bound_count",
        "receipt_artifact_readback_bound_count",
        "receipt_hash_chain_bound_count",
        "canary_artifact_cleanup_bound_count",
        "wal_write_performed_count",
        "wal_recorded_count",
        "wal_persisted_count",
        "receipt_recorded_count",
        "receipt_persisted_count",
        "receipt_materialized_count",
        "canary_artifact_filesystem_written_count",
        "artifact_readback_performed_count",
        "artifact_cleanup_performed_count",
    ] {
        assert_eq!(
            value[key], 1,
            "durable WAL/receipt persistence count should be one: {key}"
        );
    }
    for key in [
        "single_use_nonce_consumed_count",
        "explicit_command_dispatched_count",
        "receipt_delivered_count",
        "post_write_readback_performed_count",
        "readback_result_recorded_count",
        "readback_result_persisted_count",
        "readback_result_accepted_count",
        "rollback_performed_count",
        "tombstone_written_count",
        "durable_memory_store_read_performed_count",
        "durable_memory_store_write_performed_count",
        "durable_memory_store_rollback_performed_count",
        "memory_store_write_performed_count",
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
            "durable WAL/receipt persistence external or durable Memory count should stay zero: {key}"
        );
    }
    for key in [
        "wal_write_performed",
        "wal_recorded",
        "wal_persisted",
        "receipt_recorded",
        "receipt_persisted",
        "receipt_materialized",
        "canary_artifact_filesystem_written",
        "artifact_readback_performed",
        "artifact_cleanup_performed",
        "filesystem_written",
    ] {
        assert_eq!(
            value[key], true,
            "durable WAL/receipt persistence field should be true: {key}"
        );
    }
    for key in [
        "single_use_nonce_consumed",
        "explicit_command_dispatched",
        "post_write_readback_performed",
        "readback_result_accepted",
        "rollback_executed",
        "tombstone_written",
        "memory_write_execution_performed",
        "memory_store_write_performed",
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
            "durable WAL/receipt persistence external or Memory field should stay false: {key}"
        );
    }
    let fixtures =
        value["minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_fixtures"]
            .as_array()
            .expect("minimal scoped Memory durable WAL/receipt persistence fixtures");
    assert_eq!(fixtures.len(), 10);
    assert_eq!(
            fixtures
                .iter()
                .filter(|fixture| {
                    fixture["minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_accepted"]
                        == true
                })
                .count(),
            1
        );
    let denied =
            value["denied_by_minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_boundary"]
                .as_array()
                .expect("minimal scoped Memory durable WAL/receipt persistence denials");
    assert_eq!(denied.len(), 28);
    assert_eq!(
        value["denied_by_minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_boundary_count"],
        28
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "run_minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_boundary_require_live_gate"
    );
    assert_eq!(value["allowed_next_actions"][0]["writes_wal"], true);
    assert_eq!(value["allowed_next_actions"][0]["persists_receipt"], true);
    assert_eq!(value["allowed_next_actions"][0]["writes_memory"], false);
    assert_eq!(
        value["allowed_next_actions"][1]["action"],
        "prepare_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_boundary"
    );
    assert_eq!(
        value["allowed_next_actions"][1]["requires_minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence"],
        true
    );
    let side_effects = value["side_effects"]
        .as_object()
        .expect("minimal scoped Memory durable WAL/receipt persistence side effects");
    assert_eq!(side_effects["wal_write_performed"].as_bool(), Some(true));
    assert_eq!(side_effects["receipt_persisted"].as_bool(), Some(true));
    assert_eq!(
        side_effects["canary_artifact_filesystem_written"].as_bool(),
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
fn hepta_memory_write_execution_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_accepts_receipt_without_memory_or_external_side_effects()
 {
    let body = route_contract_body(HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_MINIMAL_SCOPED_MEMORY_REAL_WRITE_CANARY_DURABLE_READBACK_RECEIPT_ACCEPTANCE_BOUNDARY_ENDPOINT);

    let value: serde_json::Value = serde_json::from_str(&body)
        .expect("minimal scoped Memory real-write canary durable readback receipt acceptance json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_MINIMAL_SCOPED_MEMORY_REAL_WRITE_CANARY_DURABLE_READBACK_RECEIPT_ACCEPTANCE_BOUNDARY_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-readback-receipt-acceptance-boundary --json"
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
        value["memory_write_execution_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_boundary_ready"],
        true
    );
    assert_eq!(
        value["minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_ready"],
        true
    );
    assert_eq!(
        value["minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_performed"],
        true
    );
    assert_eq!(
        value["scoped_memory_real_write_canary_mode"],
        "minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_report_only"
    );
    assert_eq!(
        value["source_minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_boundary_ready"],
        true
    );
    assert_eq!(
        value["source_accepted_minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_fixture_count"],
        1
    );
    assert_eq!(
        value["source_blocked_minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_fixture_count"],
        9
    );
    assert_eq!(value["source_wal_write_performed_count"], 1);
    assert_eq!(value["source_receipt_persisted_count"], 1);
    assert_eq!(value["source_receipt_materialized_count"], 1);
    assert_eq!(value["source_canary_artifact_filesystem_written_count"], 1);
    assert_eq!(value["source_artifact_readback_performed_count"], 1);
    assert_eq!(value["source_artifact_cleanup_performed_count"], 1);
    assert_eq!(
        value["source_durable_memory_store_write_performed_count"],
        0
    );
    assert_eq!(value["source_memory_store_write_performed_count"], 0);
    assert_eq!(value["approved_namespace"], "hepta.memory.canary");
    assert_eq!(value["approved_store"], "wal-receipt-canary-artifact");
    assert_eq!(value["approved_scope"], "session");
    assert_eq!(
        value["wal_record_id"],
        "hepta-minimal-scoped-memory-real-write-canary-durable-wal-record-v1"
    );
    assert_eq!(
        value["receipt_id"],
        "hepta-minimal-scoped-memory-real-write-canary-durable-receipt-v1"
    );
    assert_ne!(value["wal_record_sha256"], "");
    assert_ne!(value["receipt_sha256"], "");
    assert_ne!(value["receipt_artifact_readback_sha256"], "");
    assert_eq!(value["receipt_readback_digest_match"], true);
    assert_ne!(value["receipt_hash_chain_sha256"], "");
    assert_eq!(value["receipt_hash_chain_verified"], true);
    assert_ne!(value["receipt_readback_report_sha256"], "");
    assert_ne!(value["receipt_acceptance_hash_sha256"], "");
    assert_eq!(
        value["required_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_surface_count"],
        12
    );
    assert_eq!(
        value["ready_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_surface_count"],
        12
    );
    assert_eq!(
        value["minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_fixture_count"],
        10
    );
    assert_eq!(
        value["accepted_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_fixture_count"],
        1
    );
    assert_eq!(
        value["blocked_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_fixture_count"],
        9
    );
    assert_eq!(
        value["denied_by_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_boundary_count"],
        28
    );
    for key in [
        "minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_accepted_count",
        "durable_readback_receipt_acceptance_authority_accepted_count",
        "source_durable_wal_receipt_persistence_bound_count",
        "receipt_readback_identity_bound_count",
        "receipt_readback_digest_bound_count",
        "receipt_hash_chain_acceptance_bound_count",
        "receipt_source_execution_linkage_bound_count",
        "receipt_acceptance_record_bound_count",
        "rollback_receipt_acceptance_handoff_bound_count",
        "receipt_readback_performed_count",
        "receipt_readback_result_recorded_count",
        "receipt_readback_result_accepted_count",
        "receipt_acceptance_recorded_count",
        "receipt_replay_guard_accepted_count",
    ] {
        assert_eq!(
            value[key], 1,
            "durable readback receipt acceptance count should be one: {key}"
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
        "canary_artifact_filesystem_written_count",
        "artifact_readback_performed_count",
        "artifact_cleanup_performed_count",
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
        "durable_memory_store_write_performed_count",
        "durable_memory_store_rollback_performed_count",
        "memory_store_write_performed_count",
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
            "durable readback receipt acceptance side-effect count should stay zero: {key}"
        );
    }
    for key in [
        "receipt_readback_performed",
        "receipt_readback_result_recorded",
        "receipt_readback_result_accepted",
        "receipt_identity_accepted",
        "receipt_digest_accepted",
        "receipt_hash_chain_accepted",
        "durable_readback_receipt_acceptance_accepted",
        "approved_namespace_bound",
        "approved_store_bound",
        "approved_scope_bound",
        "wal_record_identity_bound",
        "receipt_identity_bound",
        "receipt_artifact_readback_digest_bound",
        "receipt_hash_chain_bound",
        "receipt_source_execution_linkage_bound",
        "receipt_acceptance_record_bound",
        "receipt_replay_guard_bound",
        "receipt_operator_review_handoff_bound",
        "rollback_receipt_acceptance_handoff_bound",
    ] {
        assert_eq!(
            value[key], true,
            "durable readback receipt acceptance field should be true: {key}"
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
        "canary_artifact_filesystem_written",
        "artifact_readback_performed",
        "artifact_cleanup_performed",
        "filesystem_written",
        "post_write_readback_performed",
        "readback_result_accepted",
        "rollback_executed",
        "tombstone_written",
        "memory_write_execution_performed",
        "memory_store_write_performed",
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
            "durable readback receipt acceptance external or Memory field should stay false: {key}"
        );
    }
    let fixtures =
            value["minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_fixtures"]
                .as_array()
                .expect("minimal scoped Memory durable readback receipt acceptance fixtures");
    assert_eq!(fixtures.len(), 10);
    assert_eq!(
            fixtures
                .iter()
                .filter(|fixture| {
                    fixture["minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_accepted"]
                        == true
                })
                .count(),
            1
        );
    let denied =
            value["denied_by_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_boundary"]
                .as_array()
                .expect("minimal scoped Memory durable readback receipt acceptance denials");
    assert_eq!(denied.len(), 28);
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "run_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_boundary_require_live_gate"
    );
    assert_eq!(
        value["allowed_next_actions"][0]["accepts_receipt_readback"],
        true
    );
    assert_eq!(value["allowed_next_actions"][0]["writes_wal"], false);
    assert_eq!(value["allowed_next_actions"][0]["persists_receipt"], false);
    assert_eq!(value["allowed_next_actions"][0]["writes_memory"], false);
    assert_eq!(
        value["allowed_next_actions"][1]["action"],
        "prepare_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_boundary"
    );
    assert_eq!(
        value["allowed_next_actions"][1]["requires_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance"],
        true
    );
    let side_effects = value["side_effects"]
        .as_object()
        .expect("minimal scoped Memory durable readback receipt acceptance side effects");
    assert_eq!(
        side_effects["receipt_readback_performed"].as_bool(),
        Some(true)
    );
    assert_eq!(
        side_effects["receipt_readback_result_accepted"].as_bool(),
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
fn hepta_memory_write_execution_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_accepts_receipt_without_rollback_or_external_side_effects()
 {
    let body = route_contract_body(HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_MINIMAL_SCOPED_MEMORY_REAL_WRITE_CANARY_ROLLBACK_RECEIPT_ACCEPTANCE_BOUNDARY_ENDPOINT);

    let value: serde_json::Value = serde_json::from_str(&body)
        .expect("minimal scoped Memory real-write canary rollback receipt acceptance json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_MINIMAL_SCOPED_MEMORY_REAL_WRITE_CANARY_ROLLBACK_RECEIPT_ACCEPTANCE_BOUNDARY_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-rollback-receipt-acceptance-boundary --json"
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
        value["memory_write_execution_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_boundary_ready"],
        true
    );
    assert_eq!(
        value["minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_ready"],
        true
    );
    assert_eq!(
        value["minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_performed"],
        true
    );
    assert_eq!(
        value["scoped_memory_real_write_canary_mode"],
        "minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_report_only"
    );
    assert_eq!(
        value["source_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_boundary_ready"],
        true
    );
    assert_eq!(
        value["source_accepted_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_fixture_count"],
        1
    );
    assert_eq!(
        value["source_blocked_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_fixture_count"],
        9
    );
    assert_eq!(value["source_receipt_readback_performed_count"], 1);
    assert_eq!(value["source_receipt_readback_result_accepted_count"], 1);
    assert_eq!(value["source_receipt_acceptance_recorded_count"], 1);
    assert_eq!(
        value["source_rollback_receipt_acceptance_handoff_bound_count"],
        1
    );
    assert_eq!(value["source_wal_write_performed_count"], 0);
    assert_eq!(value["source_receipt_persisted_count"], 0);
    assert_eq!(
        value["source_durable_memory_store_write_performed_count"],
        0
    );
    assert_eq!(value["source_memory_store_write_performed_count"], 0);
    assert_eq!(value["approved_namespace"], "hepta.memory.canary");
    assert_eq!(value["approved_store"], "wal-receipt-canary-artifact");
    assert_eq!(value["approved_scope"], "session");
    assert_ne!(value["source_receipt_acceptance_hash_sha256"], "");
    assert_ne!(value["source_receipt_readback_report_sha256"], "");
    assert_ne!(value["source_receipt_hash_chain_sha256"], "");
    assert_eq!(
        value["rollback_receipt_id"],
        "hepta-minimal-scoped-memory-real-write-canary-rollback-receipt-v1"
    );
    assert_ne!(value["rollback_receipt_sha256"], "");
    assert_ne!(value["rollback_receipt_hash_chain_sha256"], "");
    assert_ne!(value["rollback_receipt_acceptance_hash_sha256"], "");
    assert_eq!(value["rollback_receipt_digest_match"], true);
    assert_eq!(value["rollback_receipt_hash_chain_verified"], true);
    assert_eq!(
        value["required_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_surface_count"],
        12
    );
    assert_eq!(
        value["ready_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_surface_count"],
        12
    );
    assert_eq!(
        value["minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_fixture_count"],
        10
    );
    assert_eq!(
        value["accepted_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_fixture_count"],
        1
    );
    assert_eq!(
        value["blocked_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_fixture_count"],
        9
    );
    assert_eq!(
        value["denied_by_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_boundary_count"],
        28
    );
    for key in [
        "minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_accepted_count",
        "rollback_receipt_acceptance_authority_accepted_count",
        "source_durable_readback_receipt_acceptance_bound_count",
        "receipt_acceptance_hash_bound_count",
        "rollback_receipt_identity_bound_count",
        "rollback_receipt_digest_bound_count",
        "rollback_receipt_hash_chain_bound_count",
        "rollback_receipt_source_readback_linkage_bound_count",
        "rollback_receipt_acceptance_record_bound_count",
        "rollback_receipt_replay_guard_accepted_count",
        "tombstone_cleanup_handoff_bound_count",
        "rollback_receipt_acceptance_result_recorded_count",
        "rollback_receipt_acceptance_result_accepted_count",
    ] {
        assert_eq!(
            value[key], 1,
            "rollback receipt acceptance count should be one: {key}"
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
        "canary_artifact_filesystem_written_count",
        "artifact_readback_performed_count",
        "artifact_cleanup_performed_count",
        "post_write_readback_performed_count",
        "readback_result_accepted_count",
        "rollback_performed_count",
        "rollback_result_recorded_count",
        "rollback_result_persisted_count",
        "rollback_result_accepted_count",
        "tombstone_written_count",
        "compensating_memory_write_performed_count",
        "durable_memory_store_read_performed_count",
        "durable_memory_store_write_performed_count",
        "durable_memory_store_rollback_performed_count",
        "memory_store_write_performed_count",
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
            "rollback receipt acceptance side-effect count should stay zero: {key}"
        );
    }
    for key in [
        "rollback_receipt_acceptance_performed",
        "rollback_receipt_acceptance_result_recorded",
        "rollback_receipt_acceptance_result_accepted",
        "rollback_receipt_identity_accepted",
        "rollback_receipt_digest_accepted",
        "rollback_receipt_hash_chain_accepted",
        "minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_accepted",
        "source_durable_readback_receipt_acceptance_required",
        "approved_namespace_bound",
        "approved_store_bound",
        "approved_scope_bound",
        "receipt_acceptance_hash_bound",
        "rollback_receipt_identity_bound",
        "rollback_receipt_digest_bound",
        "rollback_receipt_hash_chain_bound",
        "rollback_receipt_source_readback_linkage_bound",
        "rollback_receipt_acceptance_record_bound",
        "rollback_receipt_replay_guard_bound",
        "rollback_operator_review_handoff_bound",
        "tombstone_cleanup_handoff_bound",
    ] {
        assert_eq!(
            value[key], true,
            "rollback receipt acceptance field should be true: {key}"
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
        "canary_artifact_filesystem_written",
        "artifact_readback_performed",
        "artifact_cleanup_performed",
        "filesystem_written",
        "post_write_readback_performed",
        "readback_result_accepted",
        "rollback_executed",
        "rollback_performed",
        "rollback_result_accepted",
        "tombstone_written",
        "compensating_memory_write_performed",
        "memory_write_execution_performed",
        "memory_store_write_performed",
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
            "rollback receipt acceptance external or Memory field should stay false: {key}"
        );
    }
    let fixtures =
        value["minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_fixtures"]
            .as_array()
            .expect("minimal scoped Memory rollback receipt acceptance fixtures");
    assert_eq!(fixtures.len(), 10);
    assert_eq!(
            fixtures
                .iter()
                .filter(|fixture| {
                    fixture["minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_accepted"]
                        == true
                })
                .count(),
            1
        );
    let denied =
            value["denied_by_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_boundary"]
                .as_array()
                .expect("minimal scoped Memory rollback receipt acceptance denials");
    assert_eq!(denied.len(), 28);
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "run_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_boundary_require_live_gate"
    );
    assert_eq!(
        value["allowed_next_actions"][0]["accepts_rollback_receipt"],
        true
    );
    assert_eq!(value["allowed_next_actions"][0]["writes_wal"], false);
    assert_eq!(value["allowed_next_actions"][0]["executes_rollback"], false);
    assert_eq!(value["allowed_next_actions"][0]["writes_tombstone"], false);
    assert_eq!(
        value["allowed_next_actions"][1]["action"],
        "prepare_minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_boundary"
    );
    assert_eq!(
        value["allowed_next_actions"][1]["requires_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance"],
        true
    );
    let side_effects = value["side_effects"]
        .as_object()
        .expect("minimal scoped Memory rollback receipt acceptance side effects");
    assert_eq!(
        side_effects["rollback_receipt_acceptance_performed"].as_bool(),
        Some(true)
    );
    assert_eq!(
        side_effects["rollback_receipt_acceptance_result_accepted"].as_bool(),
        Some(true)
    );
    assert_eq!(side_effects["rollback_executed"].as_bool(), Some(false));
    assert_eq!(side_effects["tombstone_written"].as_bool(), Some(false));
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
fn hepta_memory_write_execution_minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_accepts_cleanup_evidence_without_tombstone_or_external_side_effects()
 {
    let body = route_contract_body(HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_MINIMAL_SCOPED_MEMORY_REAL_WRITE_CANARY_TOMBSTONE_CLEANUP_ACCEPTANCE_BOUNDARY_ENDPOINT);

    let value: serde_json::Value = serde_json::from_str(&body)
        .expect("minimal scoped Memory real-write canary tombstone cleanup acceptance json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_MINIMAL_SCOPED_MEMORY_REAL_WRITE_CANARY_TOMBSTONE_CLEANUP_ACCEPTANCE_BOUNDARY_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-tombstone-cleanup-acceptance-boundary --json"
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
        value["memory_write_execution_minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_boundary_ready"],
        true
    );
    assert_eq!(
        value["minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_ready"],
        true
    );
    assert_eq!(
        value["minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_performed"],
        true
    );
    assert_eq!(
        value["scoped_memory_real_write_canary_mode"],
        "minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_report_only"
    );
    assert_eq!(
        value["source_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_boundary_ready"],
        true
    );
    assert_eq!(
        value["source_accepted_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_fixture_count"],
        1
    );
    assert_eq!(
        value["source_blocked_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_fixture_count"],
        9
    );
    assert_eq!(
        value["source_rollback_receipt_acceptance_result_accepted_count"],
        1
    );
    assert_eq!(value["source_tombstone_cleanup_handoff_bound_count"], 1);
    assert_eq!(value["source_wal_write_performed_count"], 0);
    assert_eq!(value["source_receipt_persisted_count"], 0);
    assert_eq!(value["source_rollback_performed_count"], 0);
    assert_eq!(value["source_tombstone_written_count"], 0);
    assert_eq!(
        value["source_durable_memory_store_write_performed_count"],
        0
    );
    assert_eq!(value["approved_namespace"], "hepta.memory.canary");
    assert_eq!(value["approved_store"], "wal-receipt-canary-artifact");
    assert_eq!(value["approved_scope"], "session");
    assert_ne!(value["source_rollback_receipt_acceptance_hash_sha256"], "");
    assert_ne!(value["source_rollback_receipt_sha256"], "");
    assert_ne!(value["source_rollback_receipt_hash_chain_sha256"], "");
    assert_eq!(
        value["tombstone_cleanup_target_id"],
        "hepta-minimal-scoped-memory-real-write-canary-tombstone-cleanup-target-v1"
    );
    assert_ne!(value["tombstone_cleanup_plan_sha256"], "");
    assert_ne!(value["tombstone_cleanup_target_sha256"], "");
    assert_ne!(value["tombstone_cleanup_receipt_linkage_sha256"], "");
    assert_ne!(value["tombstone_cleanup_acceptance_hash_sha256"], "");
    assert_eq!(value["tombstone_cleanup_receipt_linkage_verified"], true);
    assert_eq!(value["tombstone_cleanup_idempotency_guard_verified"], true);
    assert_eq!(
        value["required_minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_surface_count"],
        12
    );
    assert_eq!(
        value["ready_minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_surface_count"],
        12
    );
    assert_eq!(
        value["minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_fixture_count"],
        10
    );
    assert_eq!(
        value["accepted_minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_fixture_count"],
        1
    );
    assert_eq!(
        value["blocked_minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_fixture_count"],
        9
    );
    assert_eq!(
        value["denied_by_minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_boundary_count"],
        28
    );
    for key in [
        "minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_accepted_count",
        "tombstone_cleanup_acceptance_authority_accepted_count",
        "source_rollback_receipt_acceptance_bound_count",
        "rollback_receipt_acceptance_hash_bound_count",
        "rollback_receipt_identity_bound_count",
        "tombstone_cleanup_plan_bound_count",
        "tombstone_cleanup_target_bound_count",
        "tombstone_cleanup_receipt_linkage_bound_count",
        "tombstone_cleanup_idempotency_guard_accepted_count",
        "tombstone_cleanup_operator_review_handoff_bound_count",
        "tombstone_cleanup_acceptance_result_recorded_count",
        "tombstone_cleanup_acceptance_result_accepted_count",
    ] {
        assert_eq!(
            value[key], 1,
            "tombstone cleanup acceptance count should be one: {key}"
        );
    }
    for key in [
        "single_use_nonce_consumed_count",
        "explicit_command_dispatched_count",
        "wal_write_performed_count",
        "receipt_recorded_count",
        "receipt_persisted_count",
        "receipt_materialized_count",
        "artifact_cleanup_performed_count",
        "tombstone_cleanup_executed_count",
        "rollback_performed_count",
        "rollback_result_accepted_count",
        "tombstone_written_count",
        "compensating_memory_write_performed_count",
        "durable_memory_store_read_performed_count",
        "durable_memory_store_write_performed_count",
        "durable_memory_store_rollback_performed_count",
        "memory_store_write_performed_count",
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
            "tombstone cleanup acceptance side-effect count should stay zero: {key}"
        );
    }
    for key in [
        "tombstone_cleanup_acceptance_performed",
        "tombstone_cleanup_acceptance_result_recorded",
        "tombstone_cleanup_acceptance_result_accepted",
        "tombstone_cleanup_plan_accepted",
        "tombstone_cleanup_target_accepted",
        "tombstone_cleanup_receipt_linkage_accepted",
        "tombstone_cleanup_idempotency_guard_accepted",
        "minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_accepted",
        "source_rollback_receipt_acceptance_required",
        "approved_namespace_bound",
        "approved_store_bound",
        "approved_scope_bound",
        "rollback_receipt_acceptance_hash_bound",
        "rollback_receipt_identity_bound",
        "tombstone_cleanup_plan_bound",
        "tombstone_cleanup_target_bound",
        "tombstone_cleanup_receipt_linkage_bound",
        "tombstone_cleanup_idempotency_guard_bound",
        "tombstone_cleanup_operator_review_handoff_bound",
        "rollback_execution_forbidden",
        "tombstone_write_forbidden",
        "artifact_cleanup_forbidden",
    ] {
        assert_eq!(
            value[key], true,
            "tombstone cleanup acceptance field should be true: {key}"
        );
    }
    for key in [
        "single_use_nonce_consumed",
        "explicit_command_dispatched",
        "wal_write_performed",
        "receipt_recorded",
        "receipt_persisted",
        "receipt_materialized",
        "artifact_cleanup_performed",
        "tombstone_cleanup_executed",
        "rollback_executed",
        "rollback_performed",
        "tombstone_written",
        "compensating_memory_write_performed",
        "memory_write_execution_performed",
        "memory_store_write_performed",
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
            "tombstone cleanup acceptance external or Memory field should stay false: {key}"
        );
    }
    let fixtures =
        value["minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_fixtures"]
            .as_array()
            .expect("minimal scoped Memory tombstone cleanup acceptance fixtures");
    assert_eq!(fixtures.len(), 10);
    assert_eq!(
            fixtures
                .iter()
                .filter(|fixture| {
                    fixture["minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_accepted"]
                        == true
                })
                .count(),
            1
        );
    let denied =
            value["denied_by_minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_boundary"]
                .as_array()
                .expect("minimal scoped Memory tombstone cleanup acceptance denials");
    assert_eq!(denied.len(), 28);
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "run_minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_boundary_require_live_gate"
    );
    assert_eq!(value["allowed_next_actions"][0]["writes_tombstone"], false);
    assert_eq!(value["allowed_next_actions"][0]["cleans_artifacts"], false);
    assert_eq!(
        value["allowed_next_actions"][1]["action"],
        "prepare_minimal_scoped_memory_real_write_canary_durable_store_write_plan_boundary"
    );
    assert_eq!(
        value["allowed_next_actions"][1]["requires_minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance"],
        true
    );
    let side_effects = value["side_effects"]
        .as_object()
        .expect("minimal scoped Memory tombstone cleanup acceptance side effects");
    assert_eq!(
        side_effects["tombstone_cleanup_acceptance_performed"].as_bool(),
        Some(true)
    );
    assert_eq!(
        side_effects["tombstone_cleanup_acceptance_result_accepted"].as_bool(),
        Some(true)
    );
    assert_eq!(
        side_effects["tombstone_cleanup_executed"].as_bool(),
        Some(false)
    );
    assert_eq!(side_effects["tombstone_written"].as_bool(), Some(false));
    assert_eq!(
        side_effects["durable_memory_store_write_performed"].as_bool(),
        Some(false)
    );
    assert_eq!(
        side_effects["external_send_performed"].as_bool(),
        Some(false)
    );
}
