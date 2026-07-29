#[test]
fn hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_endpoint_blocks_lifecycle_mutation()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_RETENTION_EXPIRY_GARBAGE_COLLECTION_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator canary controlled request harness operator-review acknowledgement activation command result receipt retention expiry garbage collection denial json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-retention-expiry-garbage-collection-denial --json"
    );
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_RETENTION_EXPIRY_GARBAGE_COLLECTION_DENIAL_ENDPOINT
        );
    assert_eq!(
            value["source_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_route_endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_AUDIT_TRAIL_IMMUTABLE_EVIDENCE_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(
        value["route_count"],
        serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(
        value["source_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_route_ready"],
        true
    );
    assert_eq!(
        value["operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready"],
        true
    );
    assert_eq!(
        value["operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_denial_status"],
        "blocked"
    );
    assert_eq!(
        value["operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_schema_version"],
        "memory_intelligence_kg_operator_canary_harness_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_denial_v1"
    );
    assert_eq!(
        value["source_audit_trail_immutable_evidence_fixture_count"],
        10
    );
    assert_eq!(
        value["source_accepted_audit_trail_immutable_evidence_fixture_count"],
        0
    );
    assert_eq!(value["source_audit_trail_performed_count"], 0);
    assert_eq!(value["source_immutable_evidence_performed_count"], 0);
    assert_eq!(value["source_hash_chain_recorded_count"], 0);
    assert_eq!(
        value["retention_expiry_garbage_collection_surface_count"],
        12
    );
    assert_eq!(
        value["retention_expiry_garbage_collection_surface_ready_count"],
        12
    );
    assert_eq!(
        value["retention_expiry_garbage_collection_side_effect_free_surface_count"],
        12
    );
    assert_eq!(
        value["retention_expiry_garbage_collection_fixture_count"],
        10
    );
    assert_eq!(
        value["blocked_retention_expiry_garbage_collection_fixture_count"],
        10
    );
    assert_eq!(
        value["noop_retention_expiry_garbage_collection_fixture_count"],
        10
    );
    assert_eq!(
        value["allowed_retention_expiry_garbage_collection_fixture_count"],
        0
    );
    assert_eq!(
        value["accepted_retention_expiry_garbage_collection_fixture_count"],
        0
    );
    assert_eq!(value["retention_performed_count"], 0);
    assert_eq!(value["expiry_performed_count"], 0);
    assert_eq!(value["garbage_collection_performed_count"], 0);
    assert_eq!(value["delete_performed_count"], 0);
    assert_eq!(value["archive_written_count"], 0);
    assert_eq!(value["compaction_performed_count"], 0);
    assert_eq!(
        value["activation_command_result_receipt_retention_policy_recorded"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_retention_policy_persisted"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_retention_index_recorded"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_expiry_scheduler_registered"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_expiry_timer_started"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_garbage_collection_scan_performed"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_garbage_collection_decision_recorded"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_delete_performed"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_tombstone_recorded"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_archive_written"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_compaction_performed"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_ledger_retention_recorded"],
        false
    );
    assert_eq!(value["activation_command_result_receipt_recorded"], false);
    assert_eq!(value["activation_command_result_receipt_persisted"], false);
    assert_eq!(value["activation_command_result_receipt_accepted"], false);
    assert_eq!(value["operator_approval_from_retention_accepted"], false);
    assert_eq!(value["operator_approval_from_expiry_accepted"], false);
    assert_eq!(
        value["operator_approval_from_garbage_collection_accepted"],
        false
    );
    assert_eq!(
        value["activation_allowed_by_result_receipt_retention"],
        false
    );
    assert_eq!(value["activation_allowed_by_result_receipt_expiry"], false);
    assert_eq!(
        value["activation_allowed_by_result_receipt_garbage_collection"],
        false
    );
    assert_eq!(value["activation_command_enabled"], false);
    assert_eq!(value["activation_command_invoked"], false);
    assert_eq!(value["activation_command_dispatched"], false);
    assert_eq!(value["activation_request_accepted"], false);
    assert_eq!(value["activation_request_executed"], false);
    assert_eq!(value["dispatch_performed_count"], 0);
    assert_eq!(value["execution_performed_count"], 0);
    assert_eq!(value["context_injection_performed_count"], 0);
    assert_eq!(value["provider_invoked_count"], 0);
    assert_eq!(value["model_invoked_count"], 0);
    assert_eq!(value["memory_store_write_performed_count"], 0);
    assert_eq!(value["external_kg_adapter_read_performed_count"], 0);
    assert_eq!(value["live_kg_write_performed_count"], 0);
    assert_eq!(value["credential_read_count"], 0);
    assert_eq!(value["secret_file_read_count"], 0);
    assert_eq!(value["channel_send_performed_count"], 0);
    assert_eq!(value["install_performed_count"], 0);
    assert_eq!(value["service_restarted_count"], 0);
    assert_eq!(value["active_binary_mutated_count"], 0);
    assert_eq!(value["upstream_fetch_performed_count"], 0);
    assert_eq!(value["upstream_merge_performed_count"], 0);
    assert_eq!(value["canary_harness_armed"], false);
    assert_eq!(value["canary_harness_executable"], false);
    assert_eq!(value["canary_live_enabled"], false);
    assert_eq!(value["current_live_enabled_lane_count"], 23);
    assert_eq!(value["enablement_lane_count"], 26);
    assert_eq!(value["ready_enablement_lane_count"], 26);

    let fixtures = value["retention_expiry_garbage_collection_fixtures"]
        .as_array()
        .expect(
            "activation command result receipt retention expiry garbage collection denial fixtures",
        );
    assert_eq!(fixtures.len(), 10);
    for fixture in fixtures {
        assert!(
            fixture["retention_gc_status"]
                .as_str()
                .expect("retention expiry gc fixture status")
                .starts_with("blocked")
        );
        assert_eq!(fixture["retention_policy_recorded"], false);
        assert_eq!(fixture["retention_policy_persisted"], false);
        assert_eq!(fixture["expiry_recorded"], false);
        assert_eq!(fixture["expiry_scheduler_registered"], false);
        assert_eq!(fixture["garbage_collection_scan_performed"], false);
        assert_eq!(fixture["delete_performed"], false);
        assert_eq!(fixture["tombstone_recorded"], false);
        assert_eq!(fixture["archive_written"], false);
        assert_eq!(fixture["compaction_performed"], false);
        assert_eq!(fixture["activation_command_result_receipt_accepted"], false);
        assert_eq!(fixture["operator_approval_from_retention_accepted"], false);
        assert_eq!(fixture["activation_from_retention_allowed"], false);
        assert_eq!(fixture["provider_invoked"], false);
        assert_eq!(fixture["model_invoked"], false);
        assert_eq!(fixture["memory_store_write_performed"], false);
        assert_eq!(fixture["external_kg_adapter_read_performed"], false);
        assert_eq!(fixture["live_kg_write_performed"], false);
        assert_eq!(fixture["credential_read"], false);
        assert_eq!(fixture["secret_file_read"], false);
        assert_eq!(fixture["receipt_noop_confirmed"], true);
    }

    let denied = value
            ["denied_by_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection"]
            .as_array()
            .expect("denied activation command result receipt retention expiry garbage collection actions");
    assert!(denied.len() >= 220);
    assert_eq!(
        value["denied_by_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(value["side_effects"]["retention_policy_recorded"], false);
    assert_eq!(value["side_effects"]["retention_policy_persisted"], false);
    assert_eq!(value["side_effects"]["expiry_scheduler_registered"], false);
    assert_eq!(value["side_effects"]["expiry_timer_started"], false);
    assert_eq!(
        value["side_effects"]["garbage_collection_scan_performed"],
        false
    );
    assert_eq!(value["side_effects"]["delete_performed"], false);
    assert_eq!(value["side_effects"]["tombstone_recorded"], false);
    assert_eq!(value["side_effects"]["archive_written"], false);
    assert_eq!(value["side_effects"]["compaction_performed"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["model_invoked"], false);
    assert_eq!(value["side_effects"]["memory_store_write_performed"], false);
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(value["side_effects"]["secret_file_read"], false);
    assert_eq!(value["side_effects"]["channel_send_performed"], false);
    assert_eq!(value["side_effects"]["install_performed"], false);
    assert_eq!(value["side_effects"]["service_restarted"], false);
    assert_eq!(value["side_effects"]["active_binary_mutated"], false);
    assert_eq!(value["side_effects"]["upstream_fetch_performed"], false);
    assert_eq!(value["side_effects"]["upstream_merge_performed"], false);
}

#[test]
fn hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_endpoint_blocks_reporting_surfaces()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_EXPORT_QUERY_OBSERVABILITY_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator canary controlled request harness operator-review acknowledgement activation command result receipt export query observability denial json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-export-query-observability-denial --json"
    );
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_EXPORT_QUERY_OBSERVABILITY_DENIAL_ENDPOINT
        );
    assert_eq!(
            value["source_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_route_endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_RETENTION_EXPIRY_GARBAGE_COLLECTION_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(
        value["route_count"],
        serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(
        value["source_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_route_ready"],
        true
    );
    assert_eq!(
        value["operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_denial_ready"],
        true
    );
    assert_eq!(
        value["operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_denial_status"],
        "blocked"
    );
    assert_eq!(
        value["operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_schema_version"],
        "memory_intelligence_kg_operator_canary_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_denial_v1"
    );
    assert_eq!(
        value["source_retention_expiry_garbage_collection_fixture_count"],
        10
    );
    assert_eq!(
        value["source_blocked_retention_expiry_garbage_collection_fixture_count"],
        10
    );
    assert_eq!(
        value["source_accepted_retention_expiry_garbage_collection_fixture_count"],
        0
    );
    assert_eq!(value["source_retention_performed_count"], 0);
    assert_eq!(value["source_expiry_performed_count"], 0);
    assert_eq!(value["source_garbage_collection_performed_count"], 0);
    assert_eq!(value["export_query_observability_surface_count"], 12);
    assert_eq!(value["export_query_observability_surface_ready_count"], 12);
    assert_eq!(
        value["export_query_observability_side_effect_free_surface_count"],
        12
    );
    assert_eq!(value["export_query_observability_fixture_count"], 10);
    assert_eq!(
        value["blocked_export_query_observability_fixture_count"],
        10
    );
    assert_eq!(value["noop_export_query_observability_fixture_count"], 10);
    assert_eq!(value["allowed_export_query_observability_fixture_count"], 0);
    assert_eq!(
        value["accepted_export_query_observability_fixture_count"],
        0
    );
    assert_eq!(value["export_performed_count"], 0);
    assert_eq!(value["query_performed_count"], 0);
    assert_eq!(value["observability_performed_count"], 0);
    assert_eq!(
        value["activation_command_result_receipt_export_allowed"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_export_request_accepted"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_export_recorded"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_export_artifact_written"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_export_stream_opened"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_query_allowed"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_query_registered"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_query_endpoint_materialized"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_query_index_recorded"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_query_cache_written"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_observability_allowed"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_observability_metric_emitted"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_observability_log_recorded"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_observability_trace_recorded"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_observability_span_recorded"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_observability_event_recorded"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_observability_dashboard_materialized"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_observability_alert_registered"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_observability_slo_recorded"],
        false
    );
    assert_eq!(value["activation_allowed_by_result_receipt_export"], false);
    assert_eq!(value["activation_allowed_by_result_receipt_query"], false);
    assert_eq!(
        value["activation_allowed_by_result_receipt_observability"],
        false
    );
    assert_eq!(value["activation_command_enabled"], false);
    assert_eq!(value["activation_command_invoked"], false);
    assert_eq!(value["activation_command_dispatched"], false);
    assert_eq!(value["activation_request_accepted"], false);
    assert_eq!(value["activation_request_executed"], false);
    assert_eq!(value["dispatch_performed_count"], 0);
    assert_eq!(value["execution_performed_count"], 0);
    assert_eq!(value["runtime_router_mutated_count"], 0);
    assert_eq!(value["context_injection_performed_count"], 0);
    assert_eq!(value["provider_invoked_count"], 0);
    assert_eq!(value["model_invoked_count"], 0);
    assert_eq!(value["memory_store_write_performed_count"], 0);
    assert_eq!(value["external_kg_adapter_read_performed_count"], 0);
    assert_eq!(value["live_kg_write_performed_count"], 0);
    assert_eq!(value["credential_read_count"], 0);
    assert_eq!(value["secret_file_read_count"], 0);
    assert_eq!(value["channel_send_performed_count"], 0);
    assert_eq!(value["install_performed_count"], 0);
    assert_eq!(value["service_restarted_count"], 0);
    assert_eq!(value["active_binary_mutated_count"], 0);
    assert_eq!(value["upstream_fetch_performed_count"], 0);
    assert_eq!(value["upstream_merge_performed_count"], 0);
    assert_eq!(value["canary_harness_armed"], false);
    assert_eq!(value["canary_harness_executable"], false);
    assert_eq!(value["canary_live_enabled"], false);
    assert_eq!(value["current_live_enabled_lane_count"], 24);
    assert_eq!(value["enablement_lane_count"], 27);
    assert_eq!(value["ready_enablement_lane_count"], 27);

    let fixtures = value["export_query_observability_fixtures"]
        .as_array()
        .expect("activation command result receipt export query observability denial fixtures");
    assert_eq!(fixtures.len(), 10);
    for fixture in fixtures {
        assert!(
            fixture["export_query_observability_status"]
                .as_str()
                .expect("export query observability fixture status")
                .starts_with("blocked")
        );
        assert_eq!(fixture["export_recorded"], false);
        assert_eq!(fixture["export_artifact_written"], false);
        assert_eq!(fixture["export_stream_opened"], false);
        assert_eq!(fixture["query_registered"], false);
        assert_eq!(fixture["query_endpoint_materialized"], false);
        assert_eq!(fixture["query_index_recorded"], false);
        assert_eq!(fixture["query_cache_written"], false);
        assert_eq!(fixture["observability_metric_emitted"], false);
        assert_eq!(fixture["observability_log_recorded"], false);
        assert_eq!(fixture["observability_trace_recorded"], false);
        assert_eq!(fixture["observability_span_recorded"], false);
        assert_eq!(fixture["observability_event_recorded"], false);
        assert_eq!(fixture["observability_dashboard_materialized"], false);
        assert_eq!(fixture["observability_alert_registered"], false);
        assert_eq!(fixture["activation_command_result_receipt_accepted"], false);
        assert_eq!(fixture["operator_approval_from_export_accepted"], false);
        assert_eq!(fixture["activation_from_export_allowed"], false);
        assert_eq!(fixture["provider_invoked"], false);
        assert_eq!(fixture["model_invoked"], false);
        assert_eq!(fixture["memory_store_write_performed"], false);
        assert_eq!(fixture["external_kg_adapter_read_performed"], false);
        assert_eq!(fixture["live_kg_write_performed"], false);
        assert_eq!(fixture["credential_read"], false);
        assert_eq!(fixture["secret_file_read"], false);
        assert_eq!(fixture["receipt_noop_confirmed"], true);
    }

    let denied = value
            ["denied_by_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability"]
            .as_array()
            .expect("denied activation command result receipt export query observability actions");
    assert!(denied.len() >= 240);
    assert_eq!(
        value["denied_by_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(value["side_effects"]["export_recorded"], false);
    assert_eq!(value["side_effects"]["export_artifact_written"], false);
    assert_eq!(value["side_effects"]["export_stream_opened"], false);
    assert_eq!(value["side_effects"]["query_registered"], false);
    assert_eq!(value["side_effects"]["query_endpoint_materialized"], false);
    assert_eq!(value["side_effects"]["observability_metric_emitted"], false);
    assert_eq!(value["side_effects"]["observability_log_recorded"], false);
    assert_eq!(value["side_effects"]["observability_trace_recorded"], false);
    assert_eq!(
        value["side_effects"]["observability_dashboard_materialized"],
        false
    );
    assert_eq!(
        value["side_effects"]["observability_alert_registered"],
        false
    );
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["model_invoked"], false);
    assert_eq!(value["side_effects"]["memory_store_write_performed"], false);
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(value["side_effects"]["secret_file_read"], false);
    assert_eq!(value["side_effects"]["channel_send_performed"], false);
    assert_eq!(value["side_effects"]["install_performed"], false);
    assert_eq!(value["side_effects"]["service_restarted"], false);
    assert_eq!(value["side_effects"]["active_binary_mutated"], false);
    assert_eq!(value["side_effects"]["upstream_fetch_performed"], false);
    assert_eq!(value["side_effects"]["upstream_merge_performed"], false);
}

#[test]
fn hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_endpoint_blocks_delivery()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_OPERATOR_FACING_SUMMARY_BRIEFING_NON_PERSISTENCE_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator canary controlled request harness operator-review acknowledgement activation command result receipt operator-facing summary briefing non-persistence denial json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-operator-facing-summary-briefing-non-persistence-denial --json"
    );
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_OPERATOR_FACING_SUMMARY_BRIEFING_NON_PERSISTENCE_DENIAL_ENDPOINT
        );
    assert_eq!(
            value["source_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_route_endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_EXPORT_QUERY_OBSERVABILITY_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(
        value["route_count"],
        serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(
        value["source_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_route_ready"],
        true
    );
    assert_eq!(
        value["operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready"],
        true
    );
    assert_eq!(
        value["operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_status"],
        "blocked"
    );
    assert_eq!(
        value["activation_command_result_receipt_operator_facing_summary_briefing_schema_version"],
        "memory_intelligence_kg_operator_canary_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_v1"
    );
    assert_eq!(value["source_export_query_observability_fixture_count"], 10);
    assert_eq!(
        value["source_blocked_export_query_observability_fixture_count"],
        10
    );
    assert_eq!(
        value["source_accepted_export_query_observability_fixture_count"],
        0
    );
    assert_eq!(value["source_export_performed_count"], 0);
    assert_eq!(value["source_query_performed_count"], 0);
    assert_eq!(value["source_observability_performed_count"], 0);
    assert_eq!(value["operator_facing_summary_briefing_surface_count"], 12);
    assert_eq!(
        value["operator_facing_summary_briefing_surface_ready_count"],
        12
    );
    assert_eq!(
        value["operator_facing_summary_briefing_side_effect_free_surface_count"],
        12
    );
    assert_eq!(value["operator_facing_summary_briefing_fixture_count"], 10);
    assert_eq!(
        value["blocked_operator_facing_summary_briefing_fixture_count"],
        10
    );
    assert_eq!(
        value["noop_operator_facing_summary_briefing_fixture_count"],
        10
    );
    assert_eq!(
        value["allowed_operator_facing_summary_briefing_fixture_count"],
        0
    );
    assert_eq!(
        value["accepted_operator_facing_summary_briefing_fixture_count"],
        0
    );
    assert_eq!(value["operator_summary_denied_count"], 10);
    assert_eq!(value["operator_briefing_denied_count"], 10);
    assert_eq!(value["operator_summary_performed_count"], 0);
    assert_eq!(value["operator_briefing_performed_count"], 0);
    assert_eq!(
        value["activation_command_result_receipt_operator_summary_recorded"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_operator_summary_persisted"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_operator_summary_materialized"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_operator_summary_filesystem_written"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_operator_summary_delivered"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_operator_briefing_recorded"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_operator_briefing_persisted"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_operator_briefing_materialized"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_operator_briefing_filesystem_written"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_operator_briefing_delivered"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_operator_summary_briefing_channel_delivery_performed"],
        false
    );
    assert_eq!(value["telegram_send_performed"], false);
    assert_eq!(value["channel_send_performed"], false);
    assert_eq!(value["external_send_performed"], false);
    assert_eq!(
        value["activation_allowed_by_result_receipt_operator_summary"],
        false
    );
    assert_eq!(
        value["activation_allowed_by_result_receipt_operator_briefing"],
        false
    );
    assert_eq!(
        value["activation_allowed_by_result_receipt_summary_briefing"],
        false
    );
    assert_eq!(value["activation_command_invoked"], false);
    assert_eq!(value["activation_command_dispatched"], false);
    assert_eq!(value["activation_activated"], false);
    assert_eq!(value["runtime_router_mutated"], false);
    assert_eq!(value["provider_invoked"], false);
    assert_eq!(value["model_invoked"], false);
    assert_eq!(value["credential_read"], false);
    assert_eq!(value["secret_file_read"], false);
    assert_eq!(value["memory_store_write_performed"], false);
    assert_eq!(value["memory_store_mutated"], false);
    assert_eq!(value["live_kg_write_performed"], false);
    assert_eq!(value["rollback_executed"], false);
    assert_eq!(value["install_executed"], false);
    assert_eq!(value["service_restart_performed"], false);
    assert_eq!(value["active_binary_mutated"], false);
    assert_eq!(value["current_live_enabled_lane_count"], 25);
    assert_eq!(value["enablement_lane_count"], 28);
    assert_eq!(value["ready_enablement_lane_count"], 28);

    let fixtures = value["operator_facing_summary_briefing_fixtures"]
        .as_array()
        .expect("activation command result receipt operator-facing summary briefing fixtures");
    assert_eq!(fixtures.len(), 10);
    for fixture in fixtures {
        assert!(
            fixture["operator_summary_briefing_status"]
                .as_str()
                .expect("operator-facing summary briefing fixture status")
                .starts_with("blocked")
        );
        assert_eq!(fixture["operator_summary_recorded"], false);
        assert_eq!(fixture["operator_summary_persisted"], false);
        assert_eq!(fixture["operator_summary_materialized"], false);
        assert_eq!(fixture["operator_summary_filesystem_written"], false);
        assert_eq!(fixture["operator_summary_delivered"], false);
        assert_eq!(fixture["operator_briefing_recorded"], false);
        assert_eq!(fixture["operator_briefing_persisted"], false);
        assert_eq!(fixture["operator_briefing_materialized"], false);
        assert_eq!(fixture["operator_briefing_filesystem_written"], false);
        assert_eq!(fixture["operator_briefing_delivered"], false);
        assert_eq!(fixture["telegram_send_performed"], false);
        assert_eq!(fixture["channel_send_performed"], false);
        assert_eq!(fixture["external_send_performed"], false);
        assert_eq!(fixture["activation_command_result_receipt_accepted"], false);
        assert_eq!(fixture["activation_activated"], false);
        assert_eq!(fixture["provider_invoked"], false);
        assert_eq!(fixture["model_invoked"], false);
        assert_eq!(fixture["credential_read"], false);
        assert_eq!(fixture["secret_file_read"], false);
        assert_eq!(fixture["memory_store_write_performed"], false);
        assert_eq!(fixture["memory_store_mutated"], false);
        assert_eq!(fixture["live_kg_write_performed"], false);
        assert_eq!(fixture["rollback_executed"], false);
        assert_eq!(fixture["summary_briefing_noop_confirmed"], true);
    }

    let denied = value["denied_by_operator_facing_summary_briefing"]
        .as_array()
        .expect(
            "denied activation command result receipt operator-facing summary briefing actions",
        );
    assert!(denied.len() >= 260);
    assert_eq!(
        value["denied_by_operator_facing_summary_briefing_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["side_effects"]["activation_command_result_receipt_operator_summary_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["activation_command_result_receipt_operator_briefing_recorded"],
        false
    );
    assert_eq!(value["side_effects"]["telegram_send_performed"], false);
    assert_eq!(value["side_effects"]["channel_send_performed"], false);
    assert_eq!(value["side_effects"]["external_send_performed"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["model_invoked"], false);
    assert_eq!(value["side_effects"]["memory_store_write_performed"], false);
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(value["side_effects"]["secret_file_read"], false);
    assert_eq!(value["side_effects"]["install_executed"], false);
    assert_eq!(value["side_effects"]["service_restart_performed"], false);
    assert_eq!(value["side_effects"]["active_binary_mutated"], false);
}

#[test]
fn hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_endpoint_blocks_acceptance()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_FINAL_OPERATOR_ACKNOWLEDGEMENT_NON_ACCEPTANCE_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator canary controlled request harness operator-review acknowledgement activation command result receipt final operator acknowledgement non-acceptance denial json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-final-operator-acknowledgement-non-acceptance-denial --json"
    );
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_FINAL_OPERATOR_ACKNOWLEDGEMENT_NON_ACCEPTANCE_DENIAL_ENDPOINT
        );
    assert_eq!(
            value["source_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_route_endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_OPERATOR_FACING_SUMMARY_BRIEFING_NON_PERSISTENCE_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(
        value["route_count"],
        serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(
        value["source_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_route_ready"],
        true
    );
    assert_eq!(
        value["operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready"],
        true
    );
    assert_eq!(
        value["operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_status"],
        "blocked"
    );
    assert_eq!(
        value["activation_command_result_receipt_final_operator_acknowledgement_schema_version"],
        "memory_intelligence_kg_operator_canary_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_v1"
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
        value["activation_command_result_receipt_final_operator_acknowledgement_performed_count"],
        0
    );
    assert_eq!(
        value["activation_command_result_receipt_final_operator_acknowledgement_accepted"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_final_operator_acknowledgement_recorded"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_final_operator_acknowledgement_persisted"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_final_operator_acknowledgement_materialized"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_final_operator_acknowledgement_filesystem_written"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_final_operator_acknowledgement_delivered"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_final_operator_acknowledgement_channel_delivery_performed"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_final_operator_acknowledgement_identity_accepted"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_final_operator_acknowledgement_signature_accepted"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_final_operator_acknowledgement_timestamp_accepted"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_final_operator_acknowledgement_final_state_promoted"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_final_operator_acknowledgement_completion_promoted"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_operator_final_acceptance_recorded"],
        false
    );
    assert_eq!(value["telegram_send_performed"], false);
    assert_eq!(value["channel_send_performed"], false);
    assert_eq!(value["external_send_performed"], false);
    assert_eq!(
        value["activation_allowed_by_result_receipt_final_operator_acknowledgement"],
        false
    );
    assert_eq!(value["activation_allowed"], false);
    assert_eq!(value["activation_performed"], false);
    assert_eq!(value["live_mutation_execution_performed"], false);
    assert_eq!(value["activation_command_invoked"], false);
    assert_eq!(value["activation_command_dispatched"], false);
    assert_eq!(value["runtime_router_mutated"], false);
    assert_eq!(value["provider_invoked"], false);
    assert_eq!(value["model_invoked"], false);
    assert_eq!(value["credential_read"], false);
    assert_eq!(value["secret_file_read"], false);
    assert_eq!(value["memory_store_write_performed"], false);
    assert_eq!(value["memory_store_mutated"], false);
    assert_eq!(value["live_kg_write_performed"], false);
    assert_eq!(value["rollback_executed"], false);
    assert_eq!(value["install_executed"], false);
    assert_eq!(value["service_restart_performed"], false);
    assert_eq!(value["active_binary_mutated"], false);
    assert_eq!(value["current_live_enabled_lane_count"], 26);
    assert_eq!(value["enablement_lane_count"], 29);
    assert_eq!(value["ready_enablement_lane_count"], 29);

    let fixtures =
        value["activation_command_result_receipt_final_operator_acknowledgement_fixtures"]
            .as_array()
            .expect("activation command result receipt final operator acknowledgement fixtures");
    assert_eq!(fixtures.len(), 10);
    for fixture in fixtures {
        assert!(
            fixture["final_operator_acknowledgement_status"]
                .as_str()
                .expect("final operator acknowledgement fixture status")
                .starts_with("blocked")
        );
        assert_eq!(fixture["acknowledgement_accepted"], false);
        assert_eq!(fixture["acknowledgement_recorded"], false);
        assert_eq!(fixture["acknowledgement_persisted"], false);
        assert_eq!(fixture["acknowledgement_materialized"], false);
        assert_eq!(fixture["acknowledgement_filesystem_written"], false);
        assert_eq!(fixture["acknowledgement_delivered"], false);
        assert_eq!(fixture["acknowledgement_identity_accepted"], false);
        assert_eq!(fixture["acknowledgement_signature_accepted"], false);
        assert_eq!(fixture["acknowledgement_final_state_promoted"], false);
        assert_eq!(fixture["operator_final_acceptance_recorded"], false);
        assert_eq!(fixture["operator_final_acceptance_persisted"], false);
        assert_eq!(fixture["telegram_send_performed"], false);
        assert_eq!(fixture["channel_send_performed"], false);
        assert_eq!(fixture["external_send_performed"], false);
        assert_eq!(fixture["activation_command_result_receipt_accepted"], false);
        assert_eq!(fixture["activation_allowed"], false);
        assert_eq!(fixture["activation_performed"], false);
        assert_eq!(fixture["live_mutation_execution_performed"], false);
        assert_eq!(fixture["provider_invoked"], false);
        assert_eq!(fixture["model_invoked"], false);
        assert_eq!(fixture["credential_read"], false);
        assert_eq!(fixture["secret_file_read"], false);
        assert_eq!(fixture["memory_store_write_performed"], false);
        assert_eq!(fixture["memory_store_mutated"], false);
        assert_eq!(fixture["live_kg_write_performed"], false);
        assert_eq!(fixture["final_acknowledgement_noop_confirmed"], true);
    }

    let denied =
        value["denied_by_activation_command_result_receipt_final_operator_acknowledgement"]
            .as_array()
            .expect(
                "denied activation command result receipt final operator acknowledgement actions",
            );
    assert!(denied.len() >= 280);
    assert_eq!(
        value["denied_by_activation_command_result_receipt_final_operator_acknowledgement_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["side_effects"]["activation_command_result_receipt_final_operator_acknowledgement_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["activation_command_result_receipt_final_operator_acknowledgement_persisted"],
        false
    );
    assert_eq!(
        value["side_effects"]["activation_command_result_receipt_final_operator_acknowledgement_delivered"],
        false
    );
    assert_eq!(value["side_effects"]["telegram_send_performed"], false);
    assert_eq!(value["side_effects"]["channel_send_performed"], false);
    assert_eq!(value["side_effects"]["external_send_performed"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["model_invoked"], false);
    assert_eq!(value["side_effects"]["memory_store_write_performed"], false);
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(value["side_effects"]["secret_file_read"], false);
    assert_eq!(value["side_effects"]["install_executed"], false);
    assert_eq!(value["side_effects"]["service_restart_performed"], false);
    assert_eq!(value["side_effects"]["active_binary_mutated"], false);
}

#[test]
fn hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_endpoint_blocks_promotion()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_TERMINAL_OPERATOR_DECISION_PUBLIC_CLAIM_NON_PROMOTION_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator canary controlled request harness operator-review acknowledgement activation command result receipt terminal operator decision public claim non-promotion denial json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial --json"
    );
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_TERMINAL_OPERATOR_DECISION_PUBLIC_CLAIM_NON_PROMOTION_DENIAL_ENDPOINT
        );
    assert_eq!(
            value["source_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_route_endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_FINAL_OPERATOR_ACKNOWLEDGEMENT_NON_ACCEPTANCE_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(
        value["route_count"],
        serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(
        value["source_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_route_ready"],
        true
    );
    assert_eq!(
        value["operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_ready"],
        true
    );
    assert_eq!(
        value["operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_status"],
        "blocked"
    );
    assert_eq!(
        value["activation_command_result_receipt_terminal_operator_decision_public_claim_schema_version"],
        "memory_intelligence_kg_operator_canary_harness_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_v1"
    );
    assert_eq!(
        value["source_final_operator_acknowledgement_fixture_count"],
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
    assert_eq!(
        value["activation_command_result_receipt_terminal_operator_decision_performed_count"],
        0
    );
    assert_eq!(
        value["activation_command_result_receipt_public_claim_promotion_performed_count"],
        0
    );
    assert_eq!(
        value["activation_command_result_receipt_terminal_operator_decision_accepted"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_terminal_operator_decision_recorded"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_terminal_operator_decision_persisted"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_terminal_operator_decision_materialized"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_terminal_operator_decision_filesystem_written"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_terminal_operator_decision_delivered"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_terminal_operator_decision_identity_accepted"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_terminal_operator_decision_signature_accepted"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_terminal_operator_decision_final_state_promoted"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_public_claim_recorded"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_public_claim_persisted"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_public_claim_materialized"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_public_claim_promoted"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_public_ga_claimed"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_public_release_published"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_public_distribution_performed"],
        false
    );
    assert_eq!(
        value["activation_command_result_receipt_public_artifact_written"],
        false
    );
    assert_eq!(value["telegram_send_performed"], false);
    assert_eq!(value["channel_send_performed"], false);
    assert_eq!(value["external_send_performed"], false);
    assert_eq!(value["activation_command_result_receipt_recorded"], false);
    assert_eq!(value["activation_command_result_receipt_persisted"], false);
    assert_eq!(value["activation_command_result_receipt_accepted"], false);
    assert_eq!(
        value["activation_allowed_by_result_receipt_terminal_operator_decision"],
        false
    );
    assert_eq!(
        value["activation_allowed_by_result_receipt_final_operator_acknowledgement"],
        false
    );
    assert_eq!(value["activation_allowed"], false);
    assert_eq!(value["activation_performed"], false);
    assert_eq!(value["live_mutation_execution_performed"], false);
    assert_eq!(value["memory_write_execution_performed"], false);
    assert_eq!(value["memory_store_write_performed"], false);
    assert_eq!(value["memory_store_mutated"], false);
    assert_eq!(value["live_kg_write_performed"], false);
    assert_eq!(value["rollback_executed"], false);
    assert_eq!(value["provider_invoked"], false);
    assert_eq!(value["model_invoked"], false);
    assert_eq!(value["credential_read"], false);
    assert_eq!(value["secret_file_read"], false);
    assert_eq!(value["public_release_published"], false);
    assert_eq!(value["public_ga_claimed"], false);
    assert_eq!(value["release_artifact_written"], false);
    assert_eq!(value["public_artifact_written"], false);
    assert_eq!(value["install_executed"], false);
    assert_eq!(value["launchd_mutated"], false);
    assert_eq!(value["service_restarted"], false);
    assert_eq!(value["active_binary_mutated"], false);
    assert_eq!(value["current_live_enabled_lane_count"], 27);
    assert_eq!(value["enablement_lane_count"], 30);
    assert_eq!(value["ready_enablement_lane_count"], 30);

    let fixtures =
            value["activation_command_result_receipt_terminal_operator_decision_public_claim_fixtures"]
                .as_array()
                .expect("activation command result receipt terminal operator decision public claim fixtures");
    assert_eq!(fixtures.len(), 10);
    for fixture in fixtures {
        assert!(
            fixture["terminal_operator_decision_status"]
                .as_str()
                .expect("terminal operator decision fixture status")
                .starts_with("blocked")
        );
        assert_eq!(fixture["terminal_decision_accepted"], false);
        assert_eq!(fixture["terminal_decision_recorded"], false);
        assert_eq!(fixture["terminal_decision_persisted"], false);
        assert_eq!(fixture["terminal_decision_materialized"], false);
        assert_eq!(fixture["terminal_decision_filesystem_written"], false);
        assert_eq!(fixture["terminal_decision_delivered"], false);
        assert_eq!(fixture["terminal_decision_identity_accepted"], false);
        assert_eq!(fixture["terminal_decision_signature_accepted"], false);
        assert_eq!(fixture["terminal_decision_final_state_promoted"], false);
        assert_eq!(fixture["public_claim_promoted"], false);
        assert_eq!(fixture["public_release_published"], false);
        assert_eq!(fixture["public_ga_claimed"], false);
        assert_eq!(fixture["public_artifact_written"], false);
        assert_eq!(fixture["release_artifact_written"], false);
        assert_eq!(fixture["telegram_send_performed"], false);
        assert_eq!(fixture["channel_send_performed"], false);
        assert_eq!(fixture["external_send_performed"], false);
        assert_eq!(fixture["receipt_accepted"], false);
        assert_eq!(fixture["activation_allowed"], false);
        assert_eq!(fixture["activation_performed"], false);
        assert_eq!(fixture["live_mutation_execution_performed"], false);
        assert_eq!(fixture["provider_invoked"], false);
        assert_eq!(fixture["model_invoked"], false);
        assert_eq!(fixture["credential_read"], false);
        assert_eq!(fixture["secret_file_read"], false);
        assert_eq!(fixture["memory_store_write_performed"], false);
        assert_eq!(fixture["memory_store_mutated"], false);
        assert_eq!(fixture["terminal_operator_decision_noop_confirmed"], true);
    }
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
            .filter(|fixture| fixture["public_ga_claim_requested"] == true
                && fixture["public_release_publish_requested"] == true)
            .count(),
        1
    );

    let denied = value["denied_by_activation_command_result_receipt_terminal_operator_decision_public_claim"]
            .as_array()
            .expect("denied activation command result receipt terminal operator decision public claim actions");
    assert!(denied.len() >= 290);
    assert_eq!(
        value["denied_by_activation_command_result_receipt_terminal_operator_decision_public_claim_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["side_effects"]["activation_command_result_receipt_terminal_operator_decision_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["activation_command_result_receipt_terminal_operator_decision_persisted"],
        false
    );
    assert_eq!(
        value["side_effects"]["activation_command_result_receipt_terminal_operator_decision_delivered"],
        false
    );
    assert_eq!(
        value["side_effects"]["activation_command_result_receipt_public_claim_promoted"],
        false
    );
    assert_eq!(
        value["side_effects"]["activation_command_result_receipt_public_artifact_written"],
        false
    );
    assert_eq!(value["side_effects"]["telegram_send_performed"], false);
    assert_eq!(value["side_effects"]["channel_send_performed"], false);
    assert_eq!(value["side_effects"]["external_send_performed"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["model_invoked"], false);
    assert_eq!(value["side_effects"]["memory_store_write_performed"], false);
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(value["side_effects"]["secret_file_read"], false);
    assert_eq!(value["side_effects"]["install_executed"], false);
    assert_eq!(value["side_effects"]["service_restarted"], false);
    assert_eq!(value["side_effects"]["active_binary_mutated"], false);
}

#[test]
fn hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_endpoint_blocks_release()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_RELEASE_ARTIFACT_PUBLICATION_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator canary controlled request harness operator-review acknowledgement activation command result receipt release artifact publication denial json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-release-artifact-publication-denial --json"
    );
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_RELEASE_ARTIFACT_PUBLICATION_DENIAL_ENDPOINT
        );
    assert_eq!(
            value["source_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_route_endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_TERMINAL_OPERATOR_DECISION_PUBLIC_CLAIM_NON_PROMOTION_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(
        value["route_count"],
        serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(
        value["source_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_route_ready"],
        true
    );
    assert_eq!(
        value["operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_denial_ready"],
        true
    );
    assert_eq!(
        value["operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_denial_status"],
        "blocked"
    );
    assert_eq!(
        value["activation_command_result_receipt_release_artifact_publication_schema_version"],
        "memory_intelligence_kg_operator_canary_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_denial_v1"
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
    assert_eq!(
        value["activation_command_result_receipt_release_artifact_publication_performed_count"],
        0
    );
    assert_eq!(value["release_artifact_publication_allowed"], false);
    assert_eq!(value["release_artifact_publication_accepted"], false);
    assert_eq!(value["release_artifact_publication_recorded"], false);
    assert_eq!(value["release_artifact_publication_persisted"], false);
    assert_eq!(value["release_artifact_publication_materialized"], false);
    assert_eq!(value["release_artifact_filesystem_written"], false);
    assert_eq!(value["release_artifact_written"], false);
    assert_eq!(value["public_artifact_written"], false);
    assert_eq!(value["artifact_signature_accepted"], false);
    assert_eq!(value["artifact_notarization_accepted"], false);
    assert_eq!(value["publication_queue_enqueued"], false);
    assert_eq!(value["publication_manifest_written"], false);
    assert_eq!(value["public_distribution_performed"], false);
    assert_eq!(value["public_release_published"], false);
    assert_eq!(value["public_ga_claimed"], false);
    assert_eq!(value["public_claim_promoted"], false);
    assert_eq!(value["public_version_tag_created"], false);
    assert_eq!(value["release_notes_materialized"], false);
    assert_eq!(value["changelog_materialized"], false);
    assert_eq!(
        value["terminal_operator_decision_promoted_to_release_approval"],
        false
    );
    assert_eq!(value["telegram_send_performed"], false);
    assert_eq!(value["channel_send_performed"], false);
    assert_eq!(value["external_send_performed"], false);
    assert_eq!(value["activation_command_result_receipt_recorded"], false);
    assert_eq!(value["activation_command_result_receipt_persisted"], false);
    assert_eq!(value["activation_command_result_receipt_accepted"], false);
    assert_eq!(
        value["activation_allowed_by_release_artifact_publication"],
        false
    );
    assert_eq!(
        value["activation_allowed_by_terminal_operator_decision"],
        false
    );
    assert_eq!(value["activation_allowed"], false);
    assert_eq!(value["activation_performed"], false);
    assert_eq!(value["live_mutation_execution_performed"], false);
    assert_eq!(value["memory_write_execution_performed"], false);
    assert_eq!(value["memory_store_write_performed"], false);
    assert_eq!(value["memory_store_mutated"], false);
    assert_eq!(value["rollback_executed"], false);
    assert_eq!(value["provider_invoked"], false);
    assert_eq!(value["model_invoked"], false);
    assert_eq!(value["credential_read"], false);
    assert_eq!(value["secret_file_read"], false);
    assert_eq!(value["install_executed"], false);
    assert_eq!(value["launchd_mutated"], false);
    assert_eq!(value["service_restarted"], false);
    assert_eq!(value["active_binary_mutated"], false);
    assert_eq!(value["current_live_enabled_lane_count"], 28);
    assert_eq!(value["enablement_lane_count"], 31);
    assert_eq!(value["ready_enablement_lane_count"], 31);

    let fixtures = value["activation_command_result_receipt_release_artifact_publication_fixtures"]
        .as_array()
        .expect("activation command result receipt release artifact publication fixtures");
    assert_eq!(fixtures.len(), 10);
    for fixture in fixtures {
        assert!(
            fixture["release_artifact_publication_status"]
                .as_str()
                .expect("release artifact publication fixture status")
                .starts_with("blocked")
        );
        assert_eq!(fixture["release_artifact_publication_accepted"], false);
        assert_eq!(fixture["release_artifact_publication_recorded"], false);
        assert_eq!(fixture["release_artifact_publication_persisted"], false);
        assert_eq!(fixture["release_artifact_publication_materialized"], false);
        assert_eq!(fixture["release_artifact_filesystem_written"], false);
        assert_eq!(fixture["release_artifact_written"], false);
        assert_eq!(fixture["public_artifact_written"], false);
        assert_eq!(fixture["artifact_signature_accepted"], false);
        assert_eq!(fixture["artifact_notarization_accepted"], false);
        assert_eq!(fixture["publication_queue_enqueued"], false);
        assert_eq!(fixture["publication_manifest_written"], false);
        assert_eq!(fixture["public_distribution_performed"], false);
        assert_eq!(fixture["public_release_published"], false);
        assert_eq!(fixture["public_ga_claimed"], false);
        assert_eq!(fixture["public_claim_promoted"], false);
        assert_eq!(
            fixture["terminal_operator_decision_promoted_to_release_approval"],
            false
        );
        assert_eq!(fixture["telegram_send_performed"], false);
        assert_eq!(fixture["channel_send_performed"], false);
        assert_eq!(fixture["external_send_performed"], false);
        assert_eq!(fixture["activation_allowed"], false);
        assert_eq!(fixture["live_mutation_execution_performed"], false);
        assert_eq!(fixture["memory_store_write_performed"], false);
        assert_eq!(fixture["memory_store_mutated"], false);
        assert_eq!(fixture["provider_invoked"], false);
        assert_eq!(fixture["model_invoked"], false);
        assert_eq!(fixture["install_executed"], false);
        assert_eq!(fixture["service_restarted"], false);
        assert_eq!(fixture["active_binary_mutated"], false);
        assert_eq!(fixture["release_artifact_publication_noop_confirmed"], true);
    }
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
            .filter(
                |fixture| fixture["public_release_publish_requested"] == true
                    && fixture["public_ga_claim_requested"] == true
            )
            .count(),
        1
    );

    let denied = value["denied_by_activation_command_result_receipt_release_artifact_publication"]
        .as_array()
        .expect("denied activation command result receipt release artifact publication actions");
    assert!(denied.len() >= 300);
    assert_eq!(
        value["denied_by_activation_command_result_receipt_release_artifact_publication_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["side_effects"]["release_artifact_publication_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["release_artifact_publication_persisted"],
        false
    );
    assert_eq!(
        value["side_effects"]["release_artifact_filesystem_written"],
        false
    );
    assert_eq!(value["side_effects"]["release_artifact_written"], false);
    assert_eq!(value["side_effects"]["public_artifact_written"], false);
    assert_eq!(value["side_effects"]["public_release_published"], false);
    assert_eq!(value["side_effects"]["public_ga_claimed"], false);
    assert_eq!(value["side_effects"]["publication_queue_enqueued"], false);
    assert_eq!(value["side_effects"]["telegram_send_performed"], false);
    assert_eq!(value["side_effects"]["channel_send_performed"], false);
    assert_eq!(value["side_effects"]["external_send_performed"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["model_invoked"], false);
    assert_eq!(value["side_effects"]["memory_store_write_performed"], false);
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(value["side_effects"]["secret_file_read"], false);
    assert_eq!(value["side_effects"]["install_executed"], false);
    assert_eq!(value["side_effects"]["service_restarted"], false);
    assert_eq!(value["side_effects"]["active_binary_mutated"], false);
}

#[test]
fn hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_result_receipt_endpoint_blocks_persistence()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_RELEASE_ARTIFACT_PUBLICATION_RESULT_RECEIPT_NO_PERSISTENCE_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator canary controlled request harness operator-review acknowledgement activation command result receipt release artifact publication result receipt no-persistence json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-release-artifact-publication-result-receipt-no-persistence --json"
    );
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_RELEASE_ARTIFACT_PUBLICATION_RESULT_RECEIPT_NO_PERSISTENCE_ENDPOINT
        );
    assert_eq!(
            value["source_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_route_endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_RELEASE_ARTIFACT_PUBLICATION_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(
        value["route_count"],
        serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(
        value["operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_result_receipt_no_persistence_route_enabled"],
        true
    );
    assert_eq!(
        value["operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_result_receipt_no_persistence_ready"],
        true
    );
    assert_eq!(
        value["operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_result_receipt_no_persistence_status"],
        "blocked"
    );
    assert_eq!(
        value["activation_command_result_receipt_release_artifact_publication_result_receipt_schema_version"],
        "memory_intelligence_kg_operator_canary_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_result_receipt_no_persistence_v1"
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
        value["required_activation_command_result_receipt_release_artifact_publication_result_receipt_surface_count"],
        12
    );
    assert_eq!(
        value["ready_activation_command_result_receipt_release_artifact_publication_result_receipt_surface_count"],
        12
    );
    assert_eq!(
        value["side_effect_free_activation_command_result_receipt_release_artifact_publication_result_receipt_surface_count"],
        12
    );
    assert_eq!(
        value["activation_command_result_receipt_release_artifact_publication_result_receipt_fixture_count"],
        10
    );
    assert_eq!(
        value["blocked_activation_command_result_receipt_release_artifact_publication_result_receipt_fixture_count"],
        10
    );
    assert_eq!(
        value["noop_activation_command_result_receipt_release_artifact_publication_result_receipt_fixture_count"],
        10
    );
    assert_eq!(
        value["allowed_activation_command_result_receipt_release_artifact_publication_result_receipt_fixture_count"],
        0
    );
    assert_eq!(
        value["accepted_activation_command_result_receipt_release_artifact_publication_result_receipt_fixture_count"],
        0
    );
    assert_eq!(value["publication_result_receipt_allowed"], false);
    assert_eq!(value["publication_result_receipt_accepted"], false);
    assert_eq!(value["publication_result_receipt_recorded"], false);
    assert_eq!(value["publication_result_receipt_persisted"], false);
    assert_eq!(value["publication_result_receipt_materialized"], false);
    assert_eq!(
        value["publication_result_receipt_filesystem_written"],
        false
    );
    assert_eq!(value["publication_result_receipt_ledger_written"], false);
    assert_eq!(value["publication_result_receipt_indexed"], false);
    assert_eq!(value["publication_result_receipt_enqueued"], false);
    assert_eq!(value["publication_result_receipt_delivered"], false);
    assert_eq!(value["publication_result_receipt_exported"], false);
    assert_eq!(value["publication_result_receipt_query_registered"], false);
    assert_eq!(
        value["publication_result_receipt_observability_recorded"],
        false
    );
    assert_eq!(value["publication_result_receipt_hash_bound"], false);
    assert_eq!(
        value["publication_result_receipt_signature_accepted"],
        false
    );
    assert_eq!(
        value["publication_result_receipt_timestamp_accepted"],
        false
    );
    assert_eq!(value["publication_result_receipt_status_accepted"], false);
    assert_eq!(value["publication_completion_ack_recorded"], false);
    assert_eq!(value["publication_completion_ack_persisted"], false);
    assert_eq!(value["publication_completion_ack_accepted"], false);
    assert_eq!(value["release_artifact_publication_recorded"], false);
    assert_eq!(value["release_artifact_written"], false);
    assert_eq!(value["public_artifact_written"], false);
    assert_eq!(value["public_distribution_performed"], false);
    assert_eq!(value["public_release_published"], false);
    assert_eq!(value["public_ga_claimed"], false);
    assert_eq!(
        value["terminal_operator_decision_promoted_to_release_approval"],
        false
    );
    assert_eq!(value["telegram_send_performed"], false);
    assert_eq!(value["channel_send_performed"], false);
    assert_eq!(value["external_send_performed"], false);
    assert_eq!(
        value["activation_allowed_by_publication_result_receipt"],
        false
    );
    assert_eq!(value["activation_allowed"], false);
    assert_eq!(value["activation_performed"], false);
    assert_eq!(value["live_mutation_execution_performed"], false);
    assert_eq!(value["memory_write_execution_performed"], false);
    assert_eq!(value["memory_store_write_performed"], false);
    assert_eq!(value["memory_store_mutated"], false);
    assert_eq!(value["provider_invoked"], false);
    assert_eq!(value["model_invoked"], false);
    assert_eq!(value["credential_read"], false);
    assert_eq!(value["secret_file_read"], false);
    assert_eq!(value["install_executed"], false);
    assert_eq!(value["service_restarted"], false);
    assert_eq!(value["active_binary_mutated"], false);
    assert_eq!(value["current_live_enabled_lane_count"], 29);
    assert_eq!(value["enablement_lane_count"], 32);
    assert_eq!(value["ready_enablement_lane_count"], 32);

    let fixtures = value
            ["activation_command_result_receipt_release_artifact_publication_result_receipt_fixtures"]
            .as_array()
            .expect("publication result receipt fixtures");
    assert_eq!(fixtures.len(), 10);
    for fixture in fixtures {
        assert!(
            fixture["publication_result_receipt_status"]
                .as_str()
                .expect("publication result receipt fixture status")
                .starts_with("blocked")
        );
        assert_eq!(fixture["publication_result_receipt_accepted"], false);
        assert_eq!(fixture["publication_result_receipt_recorded"], false);
        assert_eq!(fixture["publication_result_receipt_persisted"], false);
        assert_eq!(
            fixture["publication_result_receipt_filesystem_written"],
            false
        );
        assert_eq!(fixture["publication_result_receipt_delivered"], false);
        assert_eq!(fixture["publication_result_receipt_exported"], false);
        assert_eq!(
            fixture["publication_result_receipt_query_registered"],
            false
        );
        assert_eq!(fixture["publication_completion_ack_recorded"], false);
        assert_eq!(fixture["release_artifact_written"], false);
        assert_eq!(fixture["public_artifact_written"], false);
        assert_eq!(fixture["public_distribution_performed"], false);
        assert_eq!(fixture["public_release_published"], false);
        assert_eq!(fixture["external_send_performed"], false);
        assert_eq!(fixture["activation_allowed"], false);
        assert_eq!(fixture["live_mutation_execution_performed"], false);
        assert_eq!(fixture["memory_store_write_performed"], false);
        assert_eq!(fixture["provider_invoked"], false);
        assert_eq!(fixture["model_invoked"], false);
        assert_eq!(fixture["install_executed"], false);
        assert_eq!(fixture["active_binary_mutated"], false);
        assert_eq!(fixture["publication_result_receipt_noop_confirmed"], true);
    }
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["source_release_artifact_publication_present"] == false)
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["publication_authority_requested"] == true)
            .count(),
        1
    );

    let denied = value
            ["denied_by_activation_command_result_receipt_release_artifact_publication_result_receipt_no_persistence"]
            .as_array()
            .expect("publication result receipt denial list");
    assert!(denied.len() >= 310);
    assert_eq!(
        value["denied_by_activation_command_result_receipt_release_artifact_publication_result_receipt_no_persistence_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["side_effects"]["publication_result_receipt_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["publication_result_receipt_persisted"],
        false
    );
    assert_eq!(
        value["side_effects"]["publication_result_receipt_delivered"],
        false
    );
    assert_eq!(
        value["side_effects"]["publication_completion_ack_recorded"],
        false
    );
    assert_eq!(value["side_effects"]["release_artifact_written"], false);
    assert_eq!(value["side_effects"]["public_release_published"], false);
    assert_eq!(value["side_effects"]["telegram_send_performed"], false);
    assert_eq!(value["side_effects"]["channel_send_performed"], false);
    assert_eq!(value["side_effects"]["external_send_performed"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["model_invoked"], false);
    assert_eq!(value["side_effects"]["memory_store_write_performed"], false);
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(value["side_effects"]["secret_file_read"], false);
    assert_eq!(value["side_effects"]["install_executed"], false);
    assert_eq!(value["side_effects"]["service_restarted"], false);
    assert_eq!(value["side_effects"]["active_binary_mutated"], false);
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_readiness_index_replay_idempotency_endpoint_blocks_authority()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_READINESS_INDEX_REPLAY_IDEMPOTENCY_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body)
        .expect("full live activation readiness index replay/idempotency denial json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_READINESS_INDEX_REPLAY_IDEMPOTENCY_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-readiness-index-replay-idempotency-denial --json"
    );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(
        value["route_count"],
        serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_readiness_index_replay_idempotency_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_readiness_index_replay_idempotency_denial_ready"],
        true
    );
    assert_eq!(value["source_readiness_index_ready"], true);
    assert_eq!(value["source_full_live_activation_enabled"], false);
    assert_eq!(
        value["source_full_live_activation_status"],
        "blocked_report_only"
    );
    assert_eq!(value["readiness_surface_count"], 10);
    assert_eq!(value["live_activation_blocker_count"], 13);
    assert_eq!(value["required_replay_idempotency_surface_count"], 12);
    assert_eq!(value["ready_replay_idempotency_surface_count"], 12);
    assert_eq!(
        value["side_effect_free_replay_idempotency_surface_count"],
        12
    );
    assert_eq!(value["replay_idempotency_fixture_count"], 10);
    assert_eq!(value["blocked_replay_idempotency_fixture_count"], 10);
    assert_eq!(value["allowed_replay_idempotency_fixture_count"], 0);
    assert_eq!(value["accepted_replay_idempotency_fixture_count"], 0);
    assert_eq!(value["replay_allowed"], false);
    assert_eq!(value["replay_accepted"], false);
    assert_eq!(value["idempotency_key_registered"], false);
    assert_eq!(value["idempotency_key_persisted"], false);
    assert_eq!(value["idempotency_cache_written"], false);
    assert_eq!(value["query_result_registered"], false);
    assert_eq!(value["index_entry_written"], false);
    assert_eq!(value["export_recorded"], false);
    assert_eq!(value["observability_recorded"], false);
    assert_eq!(value["activation_authority_derived"], false);
    assert_eq!(value["operator_acceptance_recorded"], false);
    assert_eq!(value["operator_approval_recorded"], false);
    assert_eq!(value["memory_store_write_performed"], false);
    assert_eq!(value["memory_store_mutated"], false);
    assert_eq!(value["context_injection_performed"], false);
    assert_eq!(value["provider_invoked"], false);
    assert_eq!(value["model_invoked"], false);
    assert_eq!(value["live_kg_write_performed"], false);
    assert_eq!(value["credential_read"], false);
    assert_eq!(value["secret_file_read"], false);
    assert_eq!(value["install_executed"], false);
    assert_eq!(value["service_restarted"], false);
    assert_eq!(value["active_binary_mutated"], false);
    assert_eq!(value["public_release_claimed"], false);
    assert_eq!(value["release_artifact_written"], false);
    assert_eq!(value["current_live_enabled_lane_count"], 30);
    assert_eq!(value["enablement_lane_count"], 33);
    assert_eq!(value["ready_enablement_lane_count"], 33);

    let fixtures = value["replay_idempotency_fixtures"]
        .as_array()
        .expect("readiness index replay/idempotency fixtures");
    assert_eq!(fixtures.len(), 10);
    for fixture in fixtures {
        assert_eq!(fixture["replay_allowed"], false);
        assert_eq!(fixture["replay_accepted"], false);
        assert_eq!(fixture["idempotency_key_registered"], false);
        assert_eq!(fixture["idempotency_cache_written"], false);
        assert_eq!(fixture["query_result_registered"], false);
        assert_eq!(fixture["index_entry_written"], false);
        assert_eq!(fixture["export_recorded"], false);
        assert_eq!(fixture["observability_recorded"], false);
        assert_eq!(fixture["activation_authority_derived"], false);
        assert_eq!(fixture["operator_acceptance_recorded"], false);
        assert_eq!(fixture["memory_store_write_performed"], false);
        assert_eq!(fixture["live_kg_write_performed"], false);
        assert_eq!(fixture["provider_invoked"], false);
        assert_eq!(fixture["model_invoked"], false);
        assert_eq!(fixture["credential_read"], false);
        assert_eq!(fixture["install_executed"], false);
        assert_eq!(fixture["active_binary_mutated"], false);
        assert_eq!(fixture["replay_idempotency_noop_confirmed"], true);
    }
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["source_readiness_index_present"] == false)
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["activation_authority_requested"] == true)
            .count(),
        1
    );
    let denied = value["denied_by_readiness_index_replay_idempotency"]
        .as_array()
        .expect("readiness index replay/idempotency denials");
    assert_eq!(denied.len(), 9);
    assert_eq!(
        value["denied_by_readiness_index_replay_idempotency_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(value["side_effects"]["replay_performed"], false);
    assert_eq!(value["side_effects"]["idempotency_key_registered"], false);
    assert_eq!(value["side_effects"]["idempotency_cache_written"], false);
    assert_eq!(value["side_effects"]["query_result_registered"], false);
    assert_eq!(value["side_effects"]["index_entry_written"], false);
    assert_eq!(value["side_effects"]["export_recorded"], false);
    assert_eq!(value["side_effects"]["observability_recorded"], false);
    assert_eq!(value["side_effects"]["activation_authority_derived"], false);
    assert_eq!(value["side_effects"]["operator_acceptance_recorded"], false);
    assert_eq!(value["side_effects"]["memory_store_write_performed"], false);
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(value["side_effects"]["install_executed"], false);
    assert_eq!(value["side_effects"]["active_binary_mutated"], false);
    assert_eq!(value["side_effects"]["filesystem_written"], false);
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_endpoint_blocks_authority()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body)
        .expect("full live activation operator readiness packet template json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template --json"
    );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(
        value["route_count"],
        serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_ready"],
        true
    );
    assert_eq!(
        value["source_readiness_index_replay_idempotency_denial_ready"],
        true
    );
    assert_eq!(value["source_full_live_activation_enabled"], false);
    assert_eq!(
        value["source_full_live_activation_status"],
        "blocked_report_only"
    );
    assert_eq!(value["source_replay_allowed"], false);
    assert_eq!(value["source_activation_authority_derived"], false);
    assert_eq!(value["required_operator_packet_section_count"], 10);
    assert_eq!(value["operator_packet_section_count"], 10);
    assert_eq!(value["missing_operator_packet_section_count"], 10);
    assert_eq!(value["accepted_operator_packet_section_count"], 0);
    assert_eq!(value["recorded_operator_packet_section_count"], 0);
    assert_eq!(value["operator_packet_required_field_count"], 43);
    assert_eq!(value["operator_packet_recorded_field_count"], 0);
    assert_eq!(value["operator_packet_accepted_field_count"], 0);
    assert_eq!(value["packet_template_recorded"], false);
    assert_eq!(value["packet_template_persisted"], false);
    assert_eq!(value["packet_template_materialized"], false);
    assert_eq!(value["packet_template_delivered"], false);
    assert_eq!(value["operator_acceptance_recorded"], false);
    assert_eq!(value["operator_approval_recorded"], false);
    assert_eq!(value["activation_authority_derived"], false);
    assert_eq!(value["activation_allowed"], false);
    assert_eq!(value["activation_performed"], false);
    assert_eq!(value["memory_store_write_performed"], false);
    assert_eq!(value["memory_store_mutated"], false);
    assert_eq!(value["context_injection_performed"], false);
    assert_eq!(value["provider_invoked"], false);
    assert_eq!(value["model_invoked"], false);
    assert_eq!(value["external_kg_adapter_read_performed"], false);
    assert_eq!(value["network_call_performed"], false);
    assert_eq!(value["live_kg_write_performed"], false);
    assert_eq!(value["credential_read"], false);
    assert_eq!(value["secret_file_read"], false);
    assert_eq!(value["install_executed"], false);
    assert_eq!(value["launchd_mutated"], false);
    assert_eq!(value["service_restarted"], false);
    assert_eq!(value["active_binary_mutated"], false);
    assert_eq!(value["public_release_claimed"], false);
    assert_eq!(value["release_artifact_written"], false);
    assert_eq!(value["external_send_performed"], false);
    assert_eq!(value["current_live_enabled_lane_count"], 30);
    assert_eq!(value["enablement_lane_count"], 34);
    assert_eq!(value["ready_enablement_lane_count"], 34);

    let sections = value["operator_packet_sections"]
        .as_array()
        .expect("operator readiness packet template sections");
    assert_eq!(sections.len(), 10);
    let required_field_count: usize = sections
        .iter()
        .map(|section| {
            section["required_fields"]
                .as_array()
                .expect("section required fields")
                .len()
        })
        .sum();
    assert_eq!(required_field_count, 43);
    for section in sections {
        assert_eq!(section["status"], "missing");
        assert_eq!(section["operator_input_required"], true);
        assert_eq!(section["template_only"], true);
        assert_eq!(section["report_only"], true);
        assert_eq!(section["recorded"], false);
        assert_eq!(section["persisted"], false);
        assert_eq!(section["materialized"], false);
        assert_eq!(section["accepted"], false);
        assert_eq!(section["delivered"], false);
        assert_eq!(section["activation_authority"], false);
        assert_eq!(section["mutates_memory_store"], false);
        assert_eq!(section["writes_kg"], false);
        assert_eq!(section["attaches_intelligence_context"], false);
        assert_eq!(section["invokes_provider"], false);
        assert_eq!(section["reads_credentials"], false);
        assert_eq!(section["installs_or_restarts"], false);
        assert_eq!(section["publishes_artifacts"], false);
        assert_eq!(section["sends_external"], false);
    }
    assert_eq!(
        sections
            .iter()
            .filter(|section| section["section_id"] == "operator_authority")
            .count(),
        1
    );
    assert_eq!(
        sections
            .iter()
            .filter(|section| section["section_id"] == "final_operator_review")
            .count(),
        1
    );

    let denied = value["denied_by_operator_readiness_packet_template"]
        .as_array()
        .expect("operator readiness packet template denials");
    assert_eq!(denied.len(), 12);
    assert_eq!(
        value["denied_by_operator_readiness_packet_template_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(value["side_effects"]["packet_template_recorded"], false);
    assert_eq!(value["side_effects"]["packet_template_persisted"], false);
    assert_eq!(value["side_effects"]["packet_template_materialized"], false);
    assert_eq!(value["side_effects"]["operator_acceptance_recorded"], false);
    assert_eq!(value["side_effects"]["operator_approval_recorded"], false);
    assert_eq!(value["side_effects"]["activation_authority_derived"], false);
    assert_eq!(value["side_effects"]["activation_performed"], false);
    assert_eq!(value["side_effects"]["memory_store_write_performed"], false);
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(value["side_effects"]["install_executed"], false);
    assert_eq!(value["side_effects"]["active_binary_mutated"], false);
    assert_eq!(value["side_effects"]["external_send_performed"], false);
    assert_eq!(value["side_effects"]["filesystem_written"], false);
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_non_acceptance_endpoint_blocks_replay_authority()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_NON_ACCEPTANCE_AUTHORITY_REPLAY_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body)
        .expect("operator readiness packet template non-acceptance route json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_NON_ACCEPTANCE_AUTHORITY_REPLAY_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-non-acceptance-authority-replay-denial --json"
    );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(
        value["route_count"],
        serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_non_acceptance_authority_replay_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_non_acceptance_authority_replay_denial_ready"],
        true
    );
    assert_eq!(
        value["source_operator_readiness_packet_template_ready"],
        true
    );
    assert_eq!(value["source_operator_packet_section_count"], 10);
    assert_eq!(value["source_operator_packet_required_field_count"], 43);
    assert_eq!(value["source_operator_packet_recorded_field_count"], 0);
    assert_eq!(value["source_operator_packet_accepted_field_count"], 0);
    assert_eq!(value["required_non_acceptance_surface_count"], 12);
    assert_eq!(value["ready_non_acceptance_surface_count"], 12);
    assert_eq!(value["side_effect_free_non_acceptance_surface_count"], 12);
    assert_eq!(value["required_non_acceptance_fixture_count"], 10);
    assert_eq!(value["non_acceptance_fixture_count"], 10);
    assert_eq!(value["blocked_non_acceptance_fixture_count"], 10);
    assert_eq!(value["allowed_non_acceptance_fixture_count"], 0);
    assert_eq!(value["accepted_non_acceptance_fixture_count"], 0);
    assert_eq!(value["template_view_is_acceptance"], false);
    assert_eq!(value["template_summary_is_acceptance"], false);
    assert_eq!(value["template_replay_allowed"], false);
    assert_eq!(value["template_replay_accepted"], false);
    assert_eq!(value["template_reference_registered"], false);
    assert_eq!(value["template_reference_persisted"], false);
    assert_eq!(value["template_cache_written"], false);
    assert_eq!(value["template_query_registered"], false);
    assert_eq!(value["template_export_recorded"], false);
    assert_eq!(value["template_observability_recorded"], false);
    assert_eq!(value["operator_acceptance_recorded"], false);
    assert_eq!(value["operator_approval_recorded"], false);
    assert_eq!(value["activation_authority_derived"], false);
    assert_eq!(value["activation_command_derived"], false);
    assert_eq!(value["activation_allowed"], false);
    assert_eq!(value["activation_performed"], false);

    let fixtures = value["non_acceptance_fixtures"]
        .as_array()
        .expect("operator readiness packet template non-acceptance fixtures");
    assert_eq!(fixtures.len(), 10);
    for fixture in fixtures {
        assert_eq!(fixture["operator_acceptance_recorded"], false);
        assert_eq!(fixture["operator_approval_recorded"], false);
        assert_eq!(fixture["activation_authority_derived"], false);
        assert_eq!(fixture["activation_command_derived"], false);
        assert_eq!(fixture["activation_allowed"], false);
        assert_eq!(fixture["activation_performed"], false);
        assert_eq!(fixture["memory_store_write_performed"], false);
        assert_eq!(fixture["memory_store_mutated"], false);
        assert_eq!(fixture["live_kg_write_performed"], false);
        assert_eq!(fixture["provider_invoked"], false);
        assert_eq!(fixture["model_invoked"], false);
        assert_eq!(fixture["credential_read"], false);
        assert_eq!(fixture["install_executed"], false);
        assert_eq!(fixture["active_binary_mutated"], false);
        assert_eq!(fixture["template_non_acceptance_noop_confirmed"], true);
    }
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["template_replayed"] == true)
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["activation_authority_requested"] == true)
            .count(),
        1
    );

    let denied = value["denied_by_template_non_acceptance_authority_replay"]
        .as_array()
        .expect("template non-acceptance denials");
    assert_eq!(denied.len(), 10);
    assert_eq!(
        value["denied_by_template_non_acceptance_authority_replay_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(value["side_effects"]["template_view_recorded"], false);
    assert_eq!(value["side_effects"]["template_summary_recorded"], false);
    assert_eq!(value["side_effects"]["template_replay_performed"], false);
    assert_eq!(
        value["side_effects"]["template_reference_registered"],
        false
    );
    assert_eq!(value["side_effects"]["template_cache_written"], false);
    assert_eq!(value["side_effects"]["template_query_registered"], false);
    assert_eq!(value["side_effects"]["template_export_recorded"], false);
    assert_eq!(
        value["side_effects"]["template_observability_recorded"],
        false
    );
    assert_eq!(value["side_effects"]["operator_acceptance_recorded"], false);
    assert_eq!(value["side_effects"]["operator_approval_recorded"], false);
    assert_eq!(value["side_effects"]["activation_authority_derived"], false);
    assert_eq!(value["side_effects"]["activation_command_derived"], false);
    assert_eq!(value["side_effects"]["activation_performed"], false);
    assert_eq!(value["side_effects"]["memory_store_write_performed"], false);
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(value["side_effects"]["install_executed"], false);
    assert_eq!(value["side_effects"]["active_binary_mutated"], false);
    assert_eq!(value["side_effects"]["external_send_performed"], false);
    assert_eq!(value["side_effects"]["filesystem_written"], false);
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_field_validation_endpoint_blocks_values_authority()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_FIELD_VALIDATION_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body)
        .expect("operator readiness packet template field validation route json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_FIELD_VALIDATION_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-field-validation-denial --json"
    );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(
        value["route_count"],
        serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_field_validation_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_field_validation_denial_ready"],
        true
    );
    assert_eq!(value["source_template_non_acceptance_ready"], true);
    assert_eq!(value["source_operator_packet_section_count"], 10);
    assert_eq!(value["source_operator_packet_required_field_count"], 43);
    assert_eq!(value["source_operator_packet_recorded_field_count"], 0);
    assert_eq!(value["source_operator_packet_accepted_field_count"], 0);
    assert_eq!(value["required_field_count"], 43);
    assert_eq!(value["field_validation_matrix_count"], 43);
    assert_eq!(value["missing_field_count"], 43);
    assert_eq!(value["present_field_count"], 0);
    assert_eq!(value["captured_field_value_count"], 0);
    assert_eq!(value["recorded_field_hash_count"], 0);
    assert_eq!(value["shape_validated_field_count"], 0);
    assert_eq!(value["accepted_field_count"], 0);
    assert_eq!(value["authority_derived_field_count"], 0);
    assert_eq!(value["live_execution_allowed_field_count"], 0);
    assert_eq!(value["section_validation_count"], 10);

    let fields = value["required_field_validation_matrix"]
        .as_array()
        .expect("operator readiness packet template field validation matrix");
    assert_eq!(fields.len(), 43);
    for field in fields {
        assert_eq!(field["field_required"], true);
        assert_eq!(field["field_missing"], true);
        assert_eq!(field["field_present"], false);
        assert_eq!(field["field_value_captured"], false);
        assert_eq!(field["field_value_hash_recorded"], false);
        assert_eq!(field["field_shape_validated"], false);
        assert_eq!(field["field_recorded"], false);
        assert_eq!(field["field_persisted"], false);
        assert_eq!(field["field_accepted"], false);
        assert_eq!(field["field_authority_derived"], false);
        assert_eq!(field["field_live_execution_allowed"], false);
        assert_eq!(field["validation_status"], "missing_denied");
    }
    assert_eq!(
        fields
            .iter()
            .filter(|field| field["section_id"] == "operator_authority")
            .count(),
        5
    );
    assert_eq!(
        fields
            .iter()
            .filter(|field| field["section_id"] == "final_operator_review")
            .count(),
        4
    );
    assert_eq!(
        fields
            .iter()
            .filter(|field| field["field_name"] == "manual_acceptance_channel")
            .count(),
        1
    );

    let denied = value["denied_by_field_validation"]
        .as_array()
        .expect("field validation denials");
    assert_eq!(denied.len(), 7);
    assert_eq!(
        value["denied_by_field_validation_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(value["packet_template_recorded"], false);
    assert_eq!(value["packet_template_persisted"], false);
    assert_eq!(value["operator_acceptance_recorded"], false);
    assert_eq!(value["operator_approval_recorded"], false);
    assert_eq!(value["activation_authority_derived"], false);
    assert_eq!(value["activation_command_derived"], false);
    assert_eq!(value["activation_allowed"], false);
    assert_eq!(value["activation_performed"], false);
    assert_eq!(value["memory_store_write_performed"], false);
    assert_eq!(value["memory_store_mutated"], false);
    assert_eq!(value["live_kg_write_performed"], false);
    assert_eq!(value["provider_invoked"], false);
    assert_eq!(value["model_invoked"], false);
    assert_eq!(value["credential_read"], false);
    assert_eq!(value["install_executed"], false);
    assert_eq!(value["service_restarted"], false);
    assert_eq!(value["active_binary_mutated"], false);
    assert_eq!(value["release_artifact_written"], false);
    assert_eq!(value["external_send_performed"], false);
    assert_eq!(value["side_effects"]["field_value_captured"], false);
    assert_eq!(value["side_effects"]["field_value_hash_recorded"], false);
    assert_eq!(value["side_effects"]["field_shape_accepted"], false);
    assert_eq!(value["side_effects"]["field_value_persisted"], false);
    assert_eq!(value["side_effects"]["field_authority_derived"], false);
    assert_eq!(value["side_effects"]["field_live_execution_allowed"], false);
    assert_eq!(value["side_effects"]["operator_acceptance_recorded"], false);
    assert_eq!(value["side_effects"]["activation_authority_derived"], false);
    assert_eq!(value["side_effects"]["activation_performed"], false);
    assert_eq!(value["side_effects"]["memory_store_write_performed"], false);
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(value["side_effects"]["install_executed"], false);
    assert_eq!(value["side_effects"]["external_send_performed"], false);
    assert_eq!(value["side_effects"]["filesystem_written"], false);
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_section_completion_endpoint_blocks_acceptance_authority()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_SECTION_COMPLETION_NON_ACCEPTANCE_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body)
        .expect("operator readiness packet template section completion route json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_SECTION_COMPLETION_NON_ACCEPTANCE_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-section-completion-non-acceptance --json"
    );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(
        value["route_count"],
        serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_section_completion_non_acceptance_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_section_completion_non_acceptance_ready"],
        true
    );
    assert_eq!(value["source_field_validation_ready"], true);
    assert_eq!(value["source_operator_packet_section_count"], 10);
    assert_eq!(value["source_operator_packet_required_field_count"], 43);
    assert_eq!(value["source_required_field_count"], 43);
    assert_eq!(value["source_missing_field_count"], 43);
    assert_eq!(value["section_completion_matrix_count"], 10);
    assert_eq!(value["section_completion_attempt_count"], 10);
    assert_eq!(value["section_complete_count"], 0);
    assert_eq!(value["section_ready_count"], 0);
    assert_eq!(value["section_recorded_count"], 0);
    assert_eq!(value["section_persisted_count"], 0);
    assert_eq!(value["section_accepted_count"], 0);
    assert_eq!(value["section_operator_approval_derived_count"], 0);
    assert_eq!(value["section_activation_authority_derived_count"], 0);
    assert_eq!(value["section_live_execution_allowed_count"], 0);

    let sections = value["section_completion_matrix"]
        .as_array()
        .expect("operator readiness packet template section completion matrix");
    assert_eq!(sections.len(), 10);
    for section in sections {
        let required_field_count = section["required_field_count"]
            .as_u64()
            .expect("required field count");
        assert!(required_field_count > 0);
        assert_eq!(section["missing_field_count"], required_field_count);
        assert_eq!(section["present_field_count"], 0);
        assert_eq!(section["recorded_field_count"], 0);
        assert_eq!(section["accepted_field_count"], 0);
        assert_eq!(section["authority_derived_field_count"], 0);
        assert_eq!(section["live_execution_allowed_field_count"], 0);
        assert_eq!(section["section_completion_attempted"], true);
        assert_eq!(section["section_complete"], false);
        assert_eq!(section["section_ready"], false);
        assert_eq!(section["section_recorded"], false);
        assert_eq!(section["section_persisted"], false);
        assert_eq!(section["section_accepted"], false);
        assert_eq!(section["section_operator_approval_derived"], false);
        assert_eq!(section["section_activation_authority_derived"], false);
        assert_eq!(section["section_live_execution_allowed"], false);
        assert_eq!(
            section["completion_status"],
            "completion_denied_missing_required_fields"
        );
    }
    assert_eq!(
        sections
            .iter()
            .filter(|section| section["section_id"] == "operator_authority")
            .count(),
        1
    );
    assert_eq!(
        sections
            .iter()
            .filter(|section| section["section_id"] == "final_operator_review")
            .count(),
        1
    );

    let denied = value["denied_by_section_completion"]
        .as_array()
        .expect("section completion denials");
    assert_eq!(denied.len(), 8);
    assert_eq!(
        value["denied_by_section_completion_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(value["packet_template_recorded"], false);
    assert_eq!(value["packet_template_persisted"], false);
    assert_eq!(value["section_completion_recorded"], false);
    assert_eq!(value["section_completion_persisted"], false);
    assert_eq!(value["section_completion_accepted"], false);
    assert_eq!(value["operator_acceptance_recorded"], false);
    assert_eq!(value["operator_approval_recorded"], false);
    assert_eq!(value["activation_authority_derived"], false);
    assert_eq!(value["activation_command_derived"], false);
    assert_eq!(value["activation_allowed"], false);
    assert_eq!(value["activation_performed"], false);
    assert_eq!(value["memory_store_write_performed"], false);
    assert_eq!(value["memory_store_mutated"], false);
    assert_eq!(value["live_kg_write_performed"], false);
    assert_eq!(value["provider_invoked"], false);
    assert_eq!(value["model_invoked"], false);
    assert_eq!(value["credential_read"], false);
    assert_eq!(value["install_executed"], false);
    assert_eq!(value["service_restarted"], false);
    assert_eq!(value["active_binary_mutated"], false);
    assert_eq!(value["release_artifact_written"], false);
    assert_eq!(value["external_send_performed"], false);
    assert_eq!(value["side_effects"]["section_completion_recorded"], false);
    assert_eq!(value["side_effects"]["section_completion_persisted"], false);
    assert_eq!(value["side_effects"]["section_completion_accepted"], false);
    assert_eq!(value["side_effects"]["section_ready_promoted"], false);
    assert_eq!(
        value["side_effects"]["section_operator_approval_derived"],
        false
    );
    assert_eq!(
        value["side_effects"]["section_activation_authority_derived"],
        false
    );
    assert_eq!(
        value["side_effects"]["section_live_execution_allowed"],
        false
    );
    assert_eq!(value["side_effects"]["operator_acceptance_recorded"], false);
    assert_eq!(value["side_effects"]["activation_authority_derived"], false);
    assert_eq!(value["side_effects"]["activation_performed"], false);
    assert_eq!(value["side_effects"]["memory_store_write_performed"], false);
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(value["side_effects"]["install_executed"], false);
    assert_eq!(value["side_effects"]["external_send_performed"], false);
    assert_eq!(value["side_effects"]["filesystem_written"], false);
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_assembly_endpoint_blocks_packet_authority()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ASSEMBLY_NON_ACCEPTANCE_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body)
        .expect("operator readiness packet template packet assembly route json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ASSEMBLY_NON_ACCEPTANCE_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-assembly-non-acceptance --json"
    );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(
        value["route_count"],
        serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_assembly_non_acceptance_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_assembly_non_acceptance_ready"],
        true
    );
    assert_eq!(value["source_section_completion_ready"], true);
    assert_eq!(value["source_operator_packet_section_count"], 10);
    assert_eq!(value["source_operator_packet_required_field_count"], 43);
    assert_eq!(value["source_missing_field_count"], 43);
    assert_eq!(value["source_section_completion_matrix_count"], 10);
    assert_eq!(value["source_section_complete_count"], 0);
    assert_eq!(value["source_section_ready_count"], 0);
    assert_eq!(value["packet_assembly_attempt_count"], 4);
    assert_eq!(value["packet_assembled_count"], 0);
    assert_eq!(value["packet_complete_count"], 0);
    assert_eq!(value["packet_ready_count"], 0);
    assert_eq!(value["packet_recorded_count"], 0);
    assert_eq!(value["packet_persisted_count"], 0);
    assert_eq!(value["packet_accepted_count"], 0);
    assert_eq!(value["packet_operator_approval_derived_count"], 0);
    assert_eq!(value["packet_activation_authority_derived_count"], 0);
    assert_eq!(value["packet_activation_command_derived_count"], 0);
    assert_eq!(value["packet_live_execution_allowed_count"], 0);

    let attempts = value["packet_assembly_attempts"]
        .as_array()
        .expect("operator readiness packet template packet assembly attempts");
    assert_eq!(attempts.len(), 4);
    assert_eq!(
        attempts[0]["attempt_id"],
        "assemble_all_sections_incomplete_packet"
    );
    assert_eq!(attempts[0]["attempted_section_count"], 10);
    assert_eq!(attempts[0]["complete_section_count"], 0);
    assert_eq!(attempts[0]["missing_section_count"], 10);
    for attempt in attempts {
        assert_eq!(attempt["assembled"], false);
        assert_eq!(attempt["accepted"], false);
        assert_eq!(attempt["operator_approval_derived"], false);
        assert_eq!(attempt["activation_authority_derived"], false);
        assert_eq!(attempt["live_execution_allowed"], false);
    }

    let denied = value["denied_by_packet_assembly"]
        .as_array()
        .expect("packet assembly denials");
    assert_eq!(denied.len(), 9);
    assert_eq!(
        value["denied_by_packet_assembly_count"],
        serde_json::json!(denied.len())
    );
    let next_actions = value["allowed_next_actions"]
        .as_array()
        .expect("packet assembly next actions");
    assert_eq!(
        next_actions[0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_non_persistence_gate"
    );
    assert_eq!(next_actions[0]["status"], "allowed_report_only_next_slice");
    assert_eq!(value["packet_template_recorded"], false);
    assert_eq!(value["packet_template_persisted"], false);
    assert_eq!(value["packet_assembly_performed"], false);
    assert_eq!(value["packet_assembly_recorded"], false);
    assert_eq!(value["packet_assembly_persisted"], false);
    assert_eq!(value["packet_complete"], false);
    assert_eq!(value["packet_ready"], false);
    assert_eq!(value["packet_accepted"], false);
    assert_eq!(value["operator_acceptance_recorded"], false);
    assert_eq!(value["operator_approval_recorded"], false);
    assert_eq!(value["activation_authority_derived"], false);
    assert_eq!(value["activation_command_derived"], false);
    assert_eq!(value["activation_allowed"], false);
    assert_eq!(value["activation_performed"], false);
    assert_eq!(value["memory_store_write_performed"], false);
    assert_eq!(value["memory_store_mutated"], false);
    assert_eq!(value["live_kg_write_performed"], false);
    assert_eq!(value["provider_invoked"], false);
    assert_eq!(value["model_invoked"], false);
    assert_eq!(value["credential_read"], false);
    assert_eq!(value["install_executed"], false);
    assert_eq!(value["service_restarted"], false);
    assert_eq!(value["active_binary_mutated"], false);
    assert_eq!(value["release_artifact_written"], false);
    assert_eq!(value["external_send_performed"], false);
    assert_eq!(value["side_effects"]["packet_assembly_performed"], false);
    assert_eq!(value["side_effects"]["packet_assembly_recorded"], false);
    assert_eq!(value["side_effects"]["packet_assembly_persisted"], false);
    assert_eq!(value["side_effects"]["packet_ready_promoted"], false);
    assert_eq!(value["side_effects"]["packet_acceptance_recorded"], false);
    assert_eq!(
        value["side_effects"]["packet_operator_approval_derived"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_activation_authority_derived"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_activation_command_derived"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_live_execution_allowed"],
        false
    );
    assert_eq!(value["side_effects"]["activation_authority_derived"], false);
    assert_eq!(value["side_effects"]["activation_performed"], false);
    assert_eq!(value["side_effects"]["memory_store_write_performed"], false);
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(value["side_effects"]["install_executed"], false);
    assert_eq!(value["side_effects"]["external_send_performed"], false);
    assert_eq!(value["side_effects"]["filesystem_written"], false);
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_endpoint_blocks_persistence()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_NON_PERSISTENCE_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body)
        .expect("operator readiness packet template packet acceptance receipt route json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_NON_PERSISTENCE_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-non-persistence --json"
    );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(
        value["route_count"],
        serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_non_persistence_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_non_persistence_ready"],
        true
    );
    assert_eq!(value["source_packet_assembly_ready"], true);
    assert_eq!(value["source_packet_assembly_attempt_count"], 4);
    assert_eq!(value["source_packet_assembled_count"], 0);
    assert_eq!(value["source_packet_accepted_count"], 0);
    assert_eq!(value["source_packet_activation_authority_derived_count"], 0);
    assert_eq!(value["receipt_surface_count"], 8);
    assert_eq!(value["receipt_generated_count"], 8);
    assert_eq!(value["receipt_recorded_count"], 0);
    assert_eq!(value["receipt_persisted_count"], 0);
    assert_eq!(value["receipt_materialized_count"], 0);
    assert_eq!(value["receipt_indexed_count"], 0);
    assert_eq!(value["receipt_queryable_count"], 0);
    assert_eq!(value["receipt_exportable_count"], 0);
    assert_eq!(value["receipt_observable_count"], 0);
    assert_eq!(value["receipt_delivered_count"], 0);
    assert_eq!(value["receipt_acceptance_recorded_count"], 0);
    assert_eq!(value["receipt_operator_approval_derived_count"], 0);
    assert_eq!(value["receipt_activation_authority_derived_count"], 0);
    assert_eq!(value["receipt_activation_command_derived_count"], 0);
    assert_eq!(value["receipt_live_execution_allowed_count"], 0);

    let receipts = value["receipt_surfaces"]
        .as_array()
        .expect("operator readiness packet template packet acceptance receipt surfaces");
    assert_eq!(receipts.len(), 8);
    assert_eq!(
        receipts[0]["receipt_surface"],
        "packet_assembly_denial_receipt"
    );
    for receipt in receipts {
        assert_eq!(receipt["receipt_generated"], true);
        assert_eq!(receipt["receipt_recorded"], false);
        assert_eq!(receipt["receipt_persisted"], false);
        assert_eq!(receipt["receipt_materialized"], false);
        assert_eq!(receipt["receipt_indexed"], false);
        assert_eq!(receipt["receipt_queryable"], false);
        assert_eq!(receipt["receipt_exportable"], false);
        assert_eq!(receipt["receipt_observable"], false);
        assert_eq!(receipt["receipt_delivered"], false);
        assert_eq!(receipt["receipt_acceptance_recorded"], false);
        assert_eq!(receipt["receipt_operator_approval_derived"], false);
        assert_eq!(receipt["receipt_activation_authority_derived"], false);
        assert_eq!(receipt["receipt_activation_command_derived"], false);
        assert_eq!(receipt["receipt_live_execution_allowed"], false);
        assert_eq!(receipt["receipt_status"], "non_persistent_report_only");
    }

    let denied = value["denied_by_packet_acceptance_receipt"]
        .as_array()
        .expect("packet acceptance receipt denials");
    assert_eq!(denied.len(), 10);
    assert_eq!(
        value["denied_by_packet_acceptance_receipt_count"],
        serde_json::json!(denied.len())
    );
    let next_actions = value["allowed_next_actions"]
        .as_array()
        .expect("packet acceptance receipt next actions");
    assert_eq!(
        next_actions[0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_replay_idempotency_denial_gate"
    );
    assert_eq!(next_actions[0]["status"], "allowed_report_only_next_slice");
    assert_eq!(value["packet_template_recorded"], false);
    assert_eq!(value["packet_template_persisted"], false);
    assert_eq!(value["packet_assembly_performed"], false);
    assert_eq!(value["packet_assembly_recorded"], false);
    assert_eq!(value["packet_assembly_persisted"], false);
    assert_eq!(value["packet_complete"], false);
    assert_eq!(value["packet_ready"], false);
    assert_eq!(value["packet_accepted"], false);
    assert_eq!(value["packet_acceptance_receipt_recorded"], false);
    assert_eq!(value["packet_acceptance_receipt_persisted"], false);
    assert_eq!(value["packet_acceptance_receipt_materialized"], false);
    assert_eq!(value["packet_acceptance_receipt_indexed"], false);
    assert_eq!(value["packet_acceptance_receipt_delivered"], false);
    assert_eq!(value["operator_acceptance_recorded"], false);
    assert_eq!(value["operator_approval_recorded"], false);
    assert_eq!(value["activation_authority_derived"], false);
    assert_eq!(value["activation_command_derived"], false);
    assert_eq!(value["activation_allowed"], false);
    assert_eq!(value["activation_performed"], false);
    assert_eq!(value["memory_store_write_performed"], false);
    assert_eq!(value["memory_store_mutated"], false);
    assert_eq!(value["live_kg_write_performed"], false);
    assert_eq!(value["provider_invoked"], false);
    assert_eq!(value["model_invoked"], false);
    assert_eq!(value["credential_read"], false);
    assert_eq!(value["install_executed"], false);
    assert_eq!(value["service_restarted"], false);
    assert_eq!(value["active_binary_mutated"], false);
    assert_eq!(value["release_artifact_written"], false);
    assert_eq!(value["external_send_performed"], false);
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_persisted"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_materialized"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_indexed"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_queryable"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_exportable"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_observable"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_delivered"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_acceptance_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_authority_derived"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_live_execution_allowed"],
        false
    );
    assert_eq!(value["side_effects"]["packet_assembly_performed"], false);
    assert_eq!(value["side_effects"]["packet_assembly_persisted"], false);
    assert_eq!(value["side_effects"]["packet_ready_promoted"], false);
    assert_eq!(value["side_effects"]["packet_acceptance_recorded"], false);
    assert_eq!(value["side_effects"]["activation_authority_derived"], false);
    assert_eq!(value["side_effects"]["activation_performed"], false);
    assert_eq!(value["side_effects"]["memory_store_write_performed"], false);
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(value["side_effects"]["install_executed"], false);
    assert_eq!(value["side_effects"]["external_send_performed"], false);
    assert_eq!(value["side_effects"]["filesystem_written"], false);
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_replay_idempotency_endpoint_blocks_replay()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_REPLAY_IDEMPOTENCY_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body)
            .expect("operator readiness packet template packet acceptance receipt replay idempotency route json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_REPLAY_IDEMPOTENCY_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-replay-idempotency-denial --json"
    );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(
        value["route_count"],
        serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_replay_idempotency_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_replay_idempotency_denial_ready"],
        true
    );
    assert_eq!(value["source_packet_acceptance_receipt_ready"], true);
    assert_eq!(value["source_receipt_surface_count"], 8);
    assert_eq!(value["source_receipt_generated_count"], 8);
    assert_eq!(value["source_receipt_recorded_count"], 0);
    assert_eq!(value["source_receipt_persisted_count"], 0);
    assert_eq!(value["source_receipt_acceptance_recorded_count"], 0);
    assert_eq!(
        value["source_receipt_activation_authority_derived_count"],
        0
    );
    assert_eq!(value["replay_surface_count"], 10);
    assert_eq!(value["replay_attempt_count"], 10);
    assert_eq!(value["replay_recorded_count"], 0);
    assert_eq!(value["replay_persisted_count"], 0);
    assert_eq!(value["idempotency_key_registered_count"], 0);
    assert_eq!(value["idempotency_cache_written_count"], 0);
    assert_eq!(value["cache_hit_promoted_count"], 0);
    assert_eq!(value["query_result_registered_count"], 0);
    assert_eq!(value["export_snapshot_recorded_count"], 0);
    assert_eq!(value["observability_snapshot_recorded_count"], 0);
    assert_eq!(value["replay_acceptance_recorded_count"], 0);
    assert_eq!(value["replay_operator_approval_derived_count"], 0);
    assert_eq!(value["replay_activation_authority_derived_count"], 0);
    assert_eq!(value["replay_activation_command_derived_count"], 0);
    assert_eq!(value["replay_live_execution_allowed_count"], 0);

    let surfaces = value["replay_surfaces"]
        .as_array()
        .expect("operator readiness packet template packet acceptance receipt replay surfaces");
    assert_eq!(surfaces.len(), 10);
    assert_eq!(surfaces[0]["replay_surface"], "packet_receipt_replay");
    for surface in surfaces {
        assert_eq!(surface["replay_attempted"], true);
        assert_eq!(surface["replay_recorded"], false);
        assert_eq!(surface["replay_persisted"], false);
        assert_eq!(surface["idempotency_key_registered"], false);
        assert_eq!(surface["idempotency_cache_written"], false);
        assert_eq!(surface["cache_hit_promoted"], false);
        assert_eq!(surface["query_result_registered"], false);
        assert_eq!(surface["export_snapshot_recorded"], false);
        assert_eq!(surface["observability_snapshot_recorded"], false);
        assert_eq!(surface["acceptance_recorded"], false);
        assert_eq!(surface["operator_approval_derived"], false);
        assert_eq!(surface["activation_authority_derived"], false);
        assert_eq!(surface["activation_command_derived"], false);
        assert_eq!(surface["live_execution_allowed"], false);
        assert_eq!(surface["replay_status"], "replay_idempotency_denied");
    }

    let denied = value["denied_by_packet_receipt_replay_idempotency"]
        .as_array()
        .expect("packet acceptance receipt replay idempotency denials");
    assert_eq!(denied.len(), 11);
    assert_eq!(
        value["denied_by_packet_receipt_replay_idempotency_count"],
        serde_json::json!(denied.len())
    );
    let next_actions = value["allowed_next_actions"]
        .as_array()
        .expect("packet acceptance receipt replay idempotency next actions");
    assert_eq!(
        next_actions[0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_ordering_monotonicity_denial_gate"
    );
    assert_eq!(next_actions[0]["status"], "allowed_report_only_next_slice");
    assert_eq!(value["packet_acceptance_receipt_recorded"], false);
    assert_eq!(value["packet_acceptance_receipt_persisted"], false);
    assert_eq!(value["packet_acceptance_receipt_replayed"], false);
    assert_eq!(
        value["packet_acceptance_receipt_idempotency_key_registered"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_idempotency_cache_written"],
        false
    );
    assert_eq!(value["packet_acceptance_receipt_cache_hit_promoted"], false);
    assert_eq!(
        value["packet_acceptance_receipt_query_result_registered"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_export_snapshot_recorded"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_observability_snapshot_recorded"],
        false
    );
    assert_eq!(value["operator_acceptance_recorded"], false);
    assert_eq!(value["operator_approval_recorded"], false);
    assert_eq!(value["activation_authority_derived"], false);
    assert_eq!(value["activation_command_derived"], false);
    assert_eq!(value["activation_allowed"], false);
    assert_eq!(value["activation_performed"], false);
    assert_eq!(value["memory_store_write_performed"], false);
    assert_eq!(value["memory_store_mutated"], false);
    assert_eq!(value["live_kg_write_performed"], false);
    assert_eq!(value["provider_invoked"], false);
    assert_eq!(value["model_invoked"], false);
    assert_eq!(value["credential_read"], false);
    assert_eq!(value["install_executed"], false);
    assert_eq!(value["service_restarted"], false);
    assert_eq!(value["active_binary_mutated"], false);
    assert_eq!(value["release_artifact_written"], false);
    assert_eq!(value["external_send_performed"], false);
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_replayed"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_replay_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_replay_persisted"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_idempotency_key_registered"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_idempotency_cache_written"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_cache_hit_promoted"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_query_result_registered"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_export_snapshot_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_observability_snapshot_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_acceptance_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_authority_derived"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_live_execution_allowed"],
        false
    );
    assert_eq!(value["side_effects"]["activation_authority_derived"], false);
    assert_eq!(value["side_effects"]["activation_performed"], false);
    assert_eq!(value["side_effects"]["memory_store_write_performed"], false);
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(value["side_effects"]["install_executed"], false);
    assert_eq!(value["side_effects"]["external_send_performed"], false);
    assert_eq!(value["side_effects"]["filesystem_written"], false);
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_ordering_monotonicity_endpoint_blocks_ordering()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_ORDERING_MONOTONICITY_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body)
            .expect("operator readiness packet template packet acceptance receipt ordering monotonicity route json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_ORDERING_MONOTONICITY_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-ordering-monotonicity-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_ordering_monotonicity_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_ordering_monotonicity_denial_ready"],
        true
    );
    assert_eq!(
        value["source_packet_acceptance_receipt_replay_idempotency_ready"],
        true
    );
    assert_eq!(value["source_replay_surface_count"], 10);
    assert_eq!(value["source_replay_attempt_count"], 10);
    assert_eq!(value["source_replay_recorded_count"], 0);
    assert_eq!(value["source_replay_persisted_count"], 0);
    assert_eq!(value["source_idempotency_key_registered_count"], 0);
    assert_eq!(value["source_idempotency_cache_written_count"], 0);
    assert_eq!(value["source_cache_hit_promoted_count"], 0);
    assert_eq!(value["source_replay_acceptance_recorded_count"], 0);
    assert_eq!(value["source_replay_activation_authority_derived_count"], 0);
    assert_eq!(value["ordering_surface_count"], 14);
    assert_eq!(value["ordering_attempt_count"], 14);
    assert_eq!(value["ordering_recorded_count"], 0);
    assert_eq!(value["ordering_persisted_count"], 0);
    assert_eq!(value["ordering_materialized_count"], 0);
    assert_eq!(value["sequence_cursor_accepted_count"], 0);
    assert_eq!(value["sequence_cursor_recorded_count"], 0);
    assert_eq!(value["sequence_cursor_persisted_count"], 0);
    assert_eq!(value["monotonicity_state_recorded_count"], 0);
    assert_eq!(value["monotonicity_state_persisted_count"], 0);
    assert_eq!(value["duplicate_sequence_accepted_count"], 0);
    assert_eq!(value["stale_sequence_accepted_count"], 0);
    assert_eq!(value["late_arrival_accepted_count"], 0);
    assert_eq!(value["future_sequence_gap_accepted_count"], 0);
    assert_eq!(value["timestamp_rollback_accepted_count"], 0);
    assert_eq!(value["epoch_rollback_accepted_count"], 0);
    assert_eq!(value["same_sequence_hash_override_accepted_count"], 0);
    assert_eq!(value["latest_wins_overwrite_accepted_count"], 0);
    assert_eq!(value["ordering_acceptance_recorded_count"], 0);
    assert_eq!(value["ordering_operator_approval_derived_count"], 0);
    assert_eq!(value["ordering_activation_authority_derived_count"], 0);
    assert_eq!(value["ordering_activation_command_derived_count"], 0);
    assert_eq!(value["ordering_live_execution_allowed_count"], 0);

    let surfaces = value["ordering_surfaces"]
        .as_array()
        .expect("operator readiness packet template packet acceptance receipt ordering surfaces");
    assert_eq!(surfaces.len(), 14);
    assert_eq!(
        surfaces[0]["ordering_surface"],
        "packet_receipt_duplicate_sequence_claim"
    );
    for surface in surfaces {
        assert_eq!(surface["ordering_attempted"], true);
        assert_eq!(surface["sequence_cursor_accepted"], false);
        assert_eq!(surface["sequence_cursor_recorded"], false);
        assert_eq!(surface["sequence_cursor_persisted"], false);
        assert_eq!(surface["monotonicity_state_recorded"], false);
        assert_eq!(surface["monotonicity_state_persisted"], false);
        assert_eq!(surface["ordering_recorded"], false);
        assert_eq!(surface["ordering_persisted"], false);
        assert_eq!(surface["ordering_materialized"], false);
        assert_eq!(surface["latest_wins_accepted"], false);
        assert_eq!(surface["duplicate_accepted"], false);
        assert_eq!(surface["stale_accepted"], false);
        assert_eq!(surface["late_accepted"], false);
        assert_eq!(surface["future_gap_accepted"], false);
        assert_eq!(surface["timestamp_rollback_accepted"], false);
        assert_eq!(surface["epoch_rollback_accepted"], false);
        assert_eq!(surface["same_sequence_hash_override_accepted"], false);
        assert_eq!(surface["acceptance_recorded"], false);
        assert_eq!(surface["operator_approval_derived"], false);
        assert_eq!(surface["activation_authority_derived"], false);
        assert_eq!(surface["activation_command_derived"], false);
        assert_eq!(surface["live_execution_allowed"], false);
        assert_eq!(surface["ordering_status"], "ordering_monotonicity_denied");
    }

    let denied = value["denied_by_packet_receipt_ordering_monotonicity"]
        .as_array()
        .expect("packet acceptance receipt ordering monotonicity denials");
    assert_eq!(denied.len(), 23);
    assert_eq!(
        value["denied_by_packet_receipt_ordering_monotonicity_count"],
        serde_json::json!(denied.len())
    );
    let next_actions = value["allowed_next_actions"]
        .as_array()
        .expect("packet acceptance receipt ordering monotonicity next actions");
    assert_eq!(
        next_actions[0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_cancellation_supersession_denial_gate"
    );
    assert_eq!(next_actions[0]["status"], "allowed_report_only_next_slice");
    assert_eq!(value["packet_acceptance_receipt_recorded"], false);
    assert_eq!(value["packet_acceptance_receipt_persisted"], false);
    assert_eq!(value["packet_acceptance_receipt_replayed"], false);
    assert_eq!(
        value["packet_acceptance_receipt_idempotency_key_registered"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_idempotency_cache_written"],
        false
    );
    assert_eq!(value["packet_acceptance_receipt_ordering_recorded"], false);
    assert_eq!(value["packet_acceptance_receipt_ordering_persisted"], false);
    assert_eq!(
        value["packet_acceptance_receipt_sequence_cursor_accepted"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_sequence_cursor_recorded"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_sequence_cursor_persisted"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_monotonicity_state_recorded"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_monotonicity_state_persisted"],
        false
    );
    assert_eq!(value["packet_acceptance_receipt_duplicate_accepted"], false);
    assert_eq!(value["packet_acceptance_receipt_stale_accepted"], false);
    assert_eq!(value["packet_acceptance_receipt_late_accepted"], false);
    assert_eq!(
        value["packet_acceptance_receipt_future_gap_accepted"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_timestamp_rollback_accepted"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_epoch_rollback_accepted"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_same_sequence_hash_override_accepted"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_latest_wins_overwrite_accepted"],
        false
    );
    assert_eq!(value["operator_acceptance_recorded"], false);
    assert_eq!(value["operator_approval_recorded"], false);
    assert_eq!(value["activation_authority_derived"], false);
    assert_eq!(value["activation_command_derived"], false);
    assert_eq!(value["activation_allowed"], false);
    assert_eq!(value["activation_performed"], false);
    assert_eq!(value["memory_store_write_performed"], false);
    assert_eq!(value["memory_store_mutated"], false);
    assert_eq!(value["live_kg_write_performed"], false);
    assert_eq!(value["provider_invoked"], false);
    assert_eq!(value["model_invoked"], false);
    assert_eq!(value["credential_read"], false);
    assert_eq!(value["install_executed"], false);
    assert_eq!(value["service_restarted"], false);
    assert_eq!(value["active_binary_mutated"], false);
    assert_eq!(value["release_artifact_written"], false);
    assert_eq!(value["external_send_performed"], false);
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_ordering_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_ordering_persisted"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_sequence_cursor_accepted"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_monotonicity_state_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_latest_wins_overwrite_accepted"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_live_execution_allowed"],
        false
    );
    assert_eq!(value["side_effects"]["activation_authority_derived"], false);
    assert_eq!(value["side_effects"]["activation_performed"], false);
    assert_eq!(value["side_effects"]["memory_store_write_performed"], false);
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(value["side_effects"]["install_executed"], false);
    assert_eq!(value["side_effects"]["external_send_performed"], false);
    assert_eq!(value["side_effects"]["filesystem_written"], false);
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_cancellation_supersession_endpoint_blocks_cancellation()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_CANCELLATION_SUPERSESSION_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body)
            .expect("operator readiness packet template packet acceptance receipt cancellation supersession route json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_CANCELLATION_SUPERSESSION_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-cancellation-supersession-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_cancellation_supersession_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_cancellation_supersession_denial_ready"],
        true
    );
    assert_eq!(
        value["source_packet_acceptance_receipt_ordering_monotonicity_ready"],
        true
    );
    assert_eq!(value["source_ordering_surface_count"], 14);
    assert_eq!(value["source_ordering_attempt_count"], 14);
    assert_eq!(value["source_ordering_recorded_count"], 0);
    assert_eq!(value["source_ordering_persisted_count"], 0);
    assert_eq!(value["source_sequence_cursor_recorded_count"], 0);
    assert_eq!(value["source_monotonicity_state_recorded_count"], 0);
    assert_eq!(value["source_ordering_acceptance_recorded_count"], 0);
    assert_eq!(
        value["source_ordering_activation_authority_derived_count"],
        0
    );
    assert_eq!(value["cancellation_supersession_surface_count"], 14);
    assert_eq!(value["cancellation_supersession_attempt_count"], 14);
    assert_eq!(value["cancellation_accepted_count"], 0);
    assert_eq!(value["cancellation_recorded_count"], 0);
    assert_eq!(value["cancellation_persisted_count"], 0);
    assert_eq!(value["supersession_accepted_count"], 0);
    assert_eq!(value["supersession_recorded_count"], 0);
    assert_eq!(value["supersession_persisted_count"], 0);
    assert_eq!(value["replacement_receipt_accepted_count"], 0);
    assert_eq!(value["replacement_receipt_recorded_count"], 0);
    assert_eq!(value["replacement_receipt_persisted_count"], 0);
    assert_eq!(value["tombstone_recorded_count"], 0);
    assert_eq!(value["tombstone_persisted_count"], 0);
    assert_eq!(value["delete_marker_recorded_count"], 0);
    assert_eq!(value["latest_replacement_accepted_count"], 0);
    assert_eq!(
        value["cancellation_supersession_acceptance_recorded_count"],
        0
    );
    assert_eq!(
        value["cancellation_supersession_operator_approval_derived_count"],
        0
    );
    assert_eq!(
        value["cancellation_supersession_activation_authority_derived_count"],
        0
    );
    assert_eq!(
        value["cancellation_supersession_activation_command_derived_count"],
        0
    );
    assert_eq!(
        value["cancellation_supersession_live_execution_allowed_count"],
        0
    );

    let surfaces = value["cancellation_surfaces"].as_array().expect(
        "operator readiness packet template packet acceptance receipt cancellation surfaces",
    );
    assert_eq!(surfaces.len(), 14);
    assert_eq!(
        surfaces[0]["cancellation_surface"],
        "packet_receipt_cancel_claim"
    );
    for surface in surfaces {
        assert_eq!(surface["cancellation_or_supersession_attempted"], true);
        assert_eq!(surface["cancellation_accepted"], false);
        assert_eq!(surface["cancellation_recorded"], false);
        assert_eq!(surface["cancellation_persisted"], false);
        assert_eq!(surface["supersession_accepted"], false);
        assert_eq!(surface["supersession_recorded"], false);
        assert_eq!(surface["supersession_persisted"], false);
        assert_eq!(surface["replacement_receipt_accepted"], false);
        assert_eq!(surface["replacement_receipt_recorded"], false);
        assert_eq!(surface["replacement_receipt_persisted"], false);
        assert_eq!(surface["tombstone_recorded"], false);
        assert_eq!(surface["tombstone_persisted"], false);
        assert_eq!(surface["delete_marker_recorded"], false);
        assert_eq!(surface["latest_replacement_accepted"], false);
        assert_eq!(surface["acceptance_recorded"], false);
        assert_eq!(surface["operator_approval_derived"], false);
        assert_eq!(surface["activation_authority_derived"], false);
        assert_eq!(surface["activation_command_derived"], false);
        assert_eq!(surface["live_execution_allowed"], false);
        assert_eq!(
            surface["cancellation_supersession_status"],
            "cancellation_supersession_denied"
        );
    }

    let denied = value["denied_by_packet_receipt_cancellation_supersession"]
        .as_array()
        .expect("packet acceptance receipt cancellation supersession denials");
    assert_eq!(denied.len(), 20);
    assert_eq!(
        value["denied_by_packet_receipt_cancellation_supersession_count"],
        serde_json::json!(denied.len())
    );
    let next_actions = value["allowed_next_actions"]
        .as_array()
        .expect("packet acceptance receipt cancellation supersession next actions");
    assert_eq!(
        next_actions[0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_audit_trail_immutable_evidence_denial_gate"
    );
    assert_eq!(next_actions[0]["status"], "allowed_report_only_next_slice");
    assert_eq!(value["packet_acceptance_receipt_recorded"], false);
    assert_eq!(value["packet_acceptance_receipt_persisted"], false);
    assert_eq!(value["packet_acceptance_receipt_replayed"], false);
    assert_eq!(value["packet_acceptance_receipt_ordering_recorded"], false);
    assert_eq!(
        value["packet_acceptance_receipt_cancellation_accepted"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_cancellation_recorded"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_cancellation_persisted"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_supersession_accepted"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_supersession_recorded"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_supersession_persisted"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_replacement_accepted"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_replacement_recorded"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_replacement_persisted"],
        false
    );
    assert_eq!(value["packet_acceptance_receipt_tombstone_recorded"], false);
    assert_eq!(
        value["packet_acceptance_receipt_tombstone_persisted"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_delete_marker_recorded"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_latest_replacement_accepted"],
        false
    );
    assert_eq!(value["operator_acceptance_recorded"], false);
    assert_eq!(value["operator_approval_recorded"], false);
    assert_eq!(value["activation_authority_derived"], false);
    assert_eq!(value["activation_command_derived"], false);
    assert_eq!(value["activation_allowed"], false);
    assert_eq!(value["activation_performed"], false);
    assert_eq!(value["memory_store_write_performed"], false);
    assert_eq!(value["memory_store_mutated"], false);
    assert_eq!(value["live_kg_write_performed"], false);
    assert_eq!(value["provider_invoked"], false);
    assert_eq!(value["model_invoked"], false);
    assert_eq!(value["credential_read"], false);
    assert_eq!(value["install_executed"], false);
    assert_eq!(value["service_restarted"], false);
    assert_eq!(value["active_binary_mutated"], false);
    assert_eq!(value["release_artifact_written"], false);
    assert_eq!(value["external_send_performed"], false);
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_cancellation_accepted"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_cancellation_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_supersession_accepted"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_replacement_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_tombstone_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_delete_marker_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_latest_replacement_accepted"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_live_execution_allowed"],
        false
    );
    assert_eq!(value["side_effects"]["activation_authority_derived"], false);
    assert_eq!(value["side_effects"]["activation_performed"], false);
    assert_eq!(value["side_effects"]["memory_store_write_performed"], false);
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(value["side_effects"]["install_executed"], false);
    assert_eq!(value["side_effects"]["external_send_performed"], false);
    assert_eq!(value["side_effects"]["filesystem_written"], false);
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_audit_trail_immutable_evidence_endpoint_blocks_evidence()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_AUDIT_TRAIL_IMMUTABLE_EVIDENCE_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator readiness packet template packet acceptance receipt audit trail immutable evidence route json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_AUDIT_TRAIL_IMMUTABLE_EVIDENCE_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-audit-trail-immutable-evidence-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_audit_trail_immutable_evidence_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_audit_trail_immutable_evidence_denial_ready"],
        true
    );
    assert_eq!(
        value["source_packet_acceptance_receipt_cancellation_supersession_ready"],
        true
    );
    assert_eq!(value["source_cancellation_supersession_surface_count"], 14);
    assert_eq!(value["source_cancellation_supersession_attempt_count"], 14);
    assert_eq!(value["source_cancellation_accepted_count"], 0);
    assert_eq!(value["source_supersession_accepted_count"], 0);
    assert_eq!(value["source_replacement_receipt_accepted_count"], 0);
    assert_eq!(value["source_tombstone_recorded_count"], 0);
    assert_eq!(
        value["source_cancellation_supersession_acceptance_recorded_count"],
        0
    );
    assert_eq!(
        value["source_cancellation_supersession_activation_authority_derived_count"],
        0
    );
    assert_eq!(value["audit_evidence_surface_count"], 16);
    assert_eq!(value["audit_evidence_attempt_count"], 16);
    assert_eq!(value["audit_trail_accepted_count"], 0);
    assert_eq!(value["audit_trail_recorded_count"], 0);
    assert_eq!(value["audit_trail_persisted_count"], 0);
    assert_eq!(value["audit_trail_materialized_count"], 0);
    assert_eq!(value["immutable_evidence_accepted_count"], 0);
    assert_eq!(value["immutable_evidence_recorded_count"], 0);
    assert_eq!(value["immutable_evidence_persisted_count"], 0);
    assert_eq!(value["immutable_evidence_materialized_count"], 0);
    assert_eq!(value["hash_chain_recorded_count"], 0);
    assert_eq!(value["hash_chain_persisted_count"], 0);
    assert_eq!(value["merkle_root_recorded_count"], 0);
    assert_eq!(value["merkle_root_persisted_count"], 0);
    assert_eq!(value["attestation_recorded_count"], 0);
    assert_eq!(value["attestation_persisted_count"], 0);
    assert_eq!(value["witness_recorded_count"], 0);
    assert_eq!(value["notary_recorded_count"], 0);
    assert_eq!(value["ledger_evidence_recorded_count"], 0);
    assert_eq!(value["index_evidence_recorded_count"], 0);
    assert_eq!(value["delivery_evidence_recorded_count"], 0);
    assert_eq!(value["export_evidence_recorded_count"], 0);
    assert_eq!(value["query_evidence_registered_count"], 0);
    assert_eq!(value["observability_evidence_recorded_count"], 0);
    assert_eq!(value["readback_evidence_recorded_count"], 0);
    assert_eq!(value["audit_evidence_acceptance_recorded_count"], 0);
    assert_eq!(value["audit_evidence_operator_approval_derived_count"], 0);
    assert_eq!(
        value["audit_evidence_activation_authority_derived_count"],
        0
    );
    assert_eq!(value["audit_evidence_activation_command_derived_count"], 0);
    assert_eq!(value["audit_evidence_live_execution_allowed_count"], 0);

    let surfaces = value["audit_surfaces"]
        .as_array()
        .expect("packet acceptance receipt audit evidence surfaces");
    assert_eq!(surfaces.len(), 16);
    assert_eq!(
        surfaces[0]["audit_surface"],
        "packet_receipt_audit_trail_append_claim"
    );
    for surface in surfaces {
        assert_eq!(surface["audit_or_evidence_attempted"], true);
        assert_eq!(surface["audit_trail_accepted"], false);
        assert_eq!(surface["audit_trail_recorded"], false);
        assert_eq!(surface["audit_trail_persisted"], false);
        assert_eq!(surface["immutable_evidence_accepted"], false);
        assert_eq!(surface["immutable_evidence_recorded"], false);
        assert_eq!(surface["immutable_evidence_persisted"], false);
        assert_eq!(surface["hash_chain_recorded"], false);
        assert_eq!(surface["merkle_root_recorded"], false);
        assert_eq!(surface["attestation_recorded"], false);
        assert_eq!(surface["witness_recorded"], false);
        assert_eq!(surface["notary_recorded"], false);
        assert_eq!(surface["ledger_evidence_recorded"], false);
        assert_eq!(surface["index_evidence_recorded"], false);
        assert_eq!(surface["delivery_evidence_recorded"], false);
        assert_eq!(surface["export_evidence_recorded"], false);
        assert_eq!(surface["query_evidence_registered"], false);
        assert_eq!(surface["observability_evidence_recorded"], false);
        assert_eq!(surface["readback_evidence_recorded"], false);
        assert_eq!(surface["acceptance_recorded"], false);
        assert_eq!(surface["operator_approval_derived"], false);
        assert_eq!(surface["activation_authority_derived"], false);
        assert_eq!(surface["activation_command_derived"], false);
        assert_eq!(surface["live_execution_allowed"], false);
        assert_eq!(
            surface["audit_evidence_status"],
            "audit_trail_immutable_evidence_denied"
        );
    }

    let denied = value["denied_by_packet_receipt_audit_trail_immutable_evidence"]
        .as_array()
        .expect("packet acceptance receipt audit evidence denials");
    assert_eq!(denied.len(), 21);
    assert_eq!(
        value["denied_by_packet_receipt_audit_trail_immutable_evidence_count"],
        serde_json::json!(denied.len())
    );
    let next_actions = value["allowed_next_actions"]
        .as_array()
        .expect("packet acceptance receipt audit evidence next actions");
    assert_eq!(
        next_actions[0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_retention_expiry_garbage_collection_denial_gate"
    );
    assert_eq!(next_actions[0]["status"], "allowed_report_only_next_slice");
    assert_eq!(
        value["packet_acceptance_receipt_audit_trail_accepted"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_audit_trail_recorded"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_audit_trail_persisted"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_immutable_evidence_accepted"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_immutable_evidence_recorded"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_immutable_evidence_persisted"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_hash_chain_recorded"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_merkle_root_recorded"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_attestation_recorded"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_ledger_evidence_recorded"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_readback_evidence_recorded"],
        false
    );
    assert_eq!(value["operator_acceptance_recorded"], false);
    assert_eq!(value["operator_approval_recorded"], false);
    assert_eq!(value["activation_authority_derived"], false);
    assert_eq!(value["activation_command_derived"], false);
    assert_eq!(value["activation_allowed"], false);
    assert_eq!(value["activation_performed"], false);
    assert_eq!(value["memory_store_write_performed"], false);
    assert_eq!(value["memory_store_mutated"], false);
    assert_eq!(value["live_kg_write_performed"], false);
    assert_eq!(value["provider_invoked"], false);
    assert_eq!(value["model_invoked"], false);
    assert_eq!(value["credential_read"], false);
    assert_eq!(value["install_executed"], false);
    assert_eq!(value["service_restarted"], false);
    assert_eq!(value["active_binary_mutated"], false);
    assert_eq!(value["release_artifact_written"], false);
    assert_eq!(value["external_send_performed"], false);
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_audit_trail_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_immutable_evidence_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_hash_chain_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_attestation_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_live_execution_allowed"],
        false
    );
    assert_eq!(value["side_effects"]["activation_authority_derived"], false);
    assert_eq!(value["side_effects"]["activation_performed"], false);
    assert_eq!(value["side_effects"]["memory_store_write_performed"], false);
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(value["side_effects"]["install_executed"], false);
    assert_eq!(value["side_effects"]["external_send_performed"], false);
    assert_eq!(value["side_effects"]["filesystem_written"], false);
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_retention_expiry_garbage_collection_endpoint_blocks_lifecycle_mutation()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RETENTION_EXPIRY_GARBAGE_COLLECTION_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator readiness packet template packet acceptance receipt retention expiry garbage collection route json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RETENTION_EXPIRY_GARBAGE_COLLECTION_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-retention-expiry-garbage-collection-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_retention_expiry_garbage_collection_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_retention_expiry_garbage_collection_denial_ready"],
        true
    );
    assert_eq!(
        value["source_packet_acceptance_receipt_audit_evidence_ready"],
        true
    );
    assert_eq!(value["source_audit_evidence_surface_count"], 16);
    assert_eq!(value["source_audit_evidence_attempt_count"], 16);
    assert_eq!(value["source_audit_trail_recorded_count"], 0);
    assert_eq!(value["source_immutable_evidence_recorded_count"], 0);
    assert_eq!(value["source_hash_chain_recorded_count"], 0);
    assert_eq!(value["source_ledger_evidence_recorded_count"], 0);
    assert_eq!(value["source_readback_evidence_recorded_count"], 0);
    assert_eq!(value["source_audit_evidence_acceptance_recorded_count"], 0);
    assert_eq!(
        value["source_audit_evidence_activation_authority_derived_count"],
        0
    );
    assert_eq!(value["retention_expiry_gc_surface_count"], 17);
    assert_eq!(value["retention_expiry_gc_attempt_count"], 17);
    assert_eq!(value["retention_policy_accepted_count"], 0);
    assert_eq!(value["retention_policy_recorded_count"], 0);
    assert_eq!(value["retention_policy_persisted_count"], 0);
    assert_eq!(value["retention_index_recorded_count"], 0);
    assert_eq!(value["ttl_update_accepted_count"], 0);
    assert_eq!(value["ttl_update_recorded_count"], 0);
    assert_eq!(value["ttl_extension_accepted_count"], 0);
    assert_eq!(value["ttl_extension_recorded_count"], 0);
    assert_eq!(value["expiry_accepted_count"], 0);
    assert_eq!(value["expiry_recorded_count"], 0);
    assert_eq!(value["expiry_persisted_count"], 0);
    assert_eq!(value["expiry_scheduler_registered_count"], 0);
    assert_eq!(value["expiry_timer_started_count"], 0);
    assert_eq!(value["garbage_collection_accepted_count"], 0);
    assert_eq!(value["garbage_collection_scan_performed_count"], 0);
    assert_eq!(value["garbage_collection_candidate_recorded_count"], 0);
    assert_eq!(value["garbage_collection_decision_recorded_count"], 0);
    assert_eq!(value["delete_accepted_count"], 0);
    assert_eq!(value["delete_performed_count"], 0);
    assert_eq!(value["tombstone_recorded_count"], 0);
    assert_eq!(value["sweep_performed_count"], 0);
    assert_eq!(value["archive_written_count"], 0);
    assert_eq!(value["compaction_performed_count"], 0);
    assert_eq!(value["retention_gc_acceptance_recorded_count"], 0);
    assert_eq!(value["retention_gc_operator_approval_derived_count"], 0);
    assert_eq!(value["retention_gc_activation_authority_derived_count"], 0);
    assert_eq!(value["retention_gc_activation_command_derived_count"], 0);
    assert_eq!(value["retention_gc_live_execution_allowed_count"], 0);

    let surfaces = value["retention_surfaces"]
        .as_array()
        .expect("packet acceptance receipt retention expiry garbage collection surfaces");
    assert_eq!(surfaces.len(), 17);
    assert_eq!(
        surfaces[0]["retention_surface"],
        "packet_receipt_retention_policy_claim"
    );
    for surface in surfaces {
        assert_eq!(surface["retention_expiry_or_gc_attempted"], true);
        assert_eq!(surface["retention_policy_recorded"], false);
        assert_eq!(surface["retention_policy_persisted"], false);
        assert_eq!(surface["retention_index_recorded"], false);
        assert_eq!(surface["ttl_update_accepted"], false);
        assert_eq!(surface["expiry_accepted"], false);
        assert_eq!(surface["expiry_scheduler_registered"], false);
        assert_eq!(surface["expiry_timer_started"], false);
        assert_eq!(surface["garbage_collection_accepted"], false);
        assert_eq!(surface["garbage_collection_scan_performed"], false);
        assert_eq!(surface["delete_performed"], false);
        assert_eq!(surface["tombstone_recorded"], false);
        assert_eq!(surface["archive_written"], false);
        assert_eq!(surface["compaction_performed"], false);
        assert_eq!(surface["acceptance_recorded"], false);
        assert_eq!(surface["operator_approval_derived"], false);
        assert_eq!(surface["activation_authority_derived"], false);
        assert_eq!(surface["activation_command_derived"], false);
        assert_eq!(surface["live_execution_allowed"], false);
        assert_eq!(
            surface["retention_gc_status"],
            "retention_expiry_garbage_collection_denied"
        );
    }

    let denied = value["denied_by_packet_receipt_retention_expiry_garbage_collection"]
        .as_array()
        .expect("packet acceptance receipt retention expiry garbage collection denials");
    assert_eq!(denied.len(), 21);
    assert_eq!(
        value["denied_by_packet_receipt_retention_expiry_garbage_collection_count"],
        serde_json::json!(denied.len())
    );
    let next_actions = value["allowed_next_actions"]
        .as_array()
        .expect("packet acceptance receipt retention expiry garbage collection next actions");
    assert_eq!(
        next_actions[0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_export_query_observability_denial_gate"
    );
    assert_eq!(next_actions[0]["status"], "allowed_report_only_next_slice");
    assert_eq!(
        value["packet_acceptance_receipt_retention_policy_recorded"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_retention_policy_persisted"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_expiry_scheduler_registered"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_garbage_collection_scan_performed"],
        false
    );
    assert_eq!(value["packet_acceptance_receipt_delete_performed"], false);
    assert_eq!(value["packet_acceptance_receipt_tombstone_recorded"], false);
    assert_eq!(value["packet_acceptance_receipt_archive_written"], false);
    assert_eq!(
        value["packet_acceptance_receipt_compaction_performed"],
        false
    );
    assert_eq!(value["operator_acceptance_recorded"], false);
    assert_eq!(value["operator_approval_recorded"], false);
    assert_eq!(value["activation_authority_derived"], false);
    assert_eq!(value["activation_command_derived"], false);
    assert_eq!(value["activation_allowed"], false);
    assert_eq!(value["activation_performed"], false);
    assert_eq!(value["memory_store_write_performed"], false);
    assert_eq!(value["memory_store_mutated"], false);
    assert_eq!(value["live_kg_write_performed"], false);
    assert_eq!(value["provider_invoked"], false);
    assert_eq!(value["model_invoked"], false);
    assert_eq!(value["credential_read"], false);
    assert_eq!(value["install_executed"], false);
    assert_eq!(value["service_restarted"], false);
    assert_eq!(value["active_binary_mutated"], false);
    assert_eq!(value["release_artifact_written"], false);
    assert_eq!(value["external_send_performed"], false);
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_retention_policy_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_expiry_scheduler_registered"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_garbage_collection_scan_performed"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_delete_performed"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_compaction_performed"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_live_execution_allowed"],
        false
    );
    assert_eq!(value["side_effects"]["activation_authority_derived"], false);
    assert_eq!(value["side_effects"]["activation_performed"], false);
    assert_eq!(value["side_effects"]["memory_store_write_performed"], false);
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(value["side_effects"]["install_executed"], false);
    assert_eq!(value["side_effects"]["external_send_performed"], false);
    assert_eq!(value["side_effects"]["filesystem_written"], false);
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_export_query_observability_endpoint_blocks_query_export_observability()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_EXPORT_QUERY_OBSERVABILITY_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator readiness packet template packet acceptance receipt export query observability route json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_EXPORT_QUERY_OBSERVABILITY_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-export-query-observability-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_export_query_observability_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_export_query_observability_denial_ready"],
        true
    );
    assert_eq!(
        value["source_packet_acceptance_receipt_retention_expiry_gc_ready"],
        true
    );
    assert_eq!(value["source_retention_expiry_gc_surface_count"], 17);
    assert_eq!(value["source_retention_expiry_gc_attempt_count"], 17);
    assert_eq!(value["source_retention_policy_recorded_count"], 0);
    assert_eq!(value["source_expiry_recorded_count"], 0);
    assert_eq!(value["source_garbage_collection_scan_performed_count"], 0);
    assert_eq!(value["source_archive_written_count"], 0);
    assert_eq!(value["source_compaction_performed_count"], 0);
    assert_eq!(
        value["source_retention_gc_activation_authority_derived_count"],
        0
    );
    assert_eq!(value["export_query_observability_surface_count"], 16);
    assert_eq!(value["export_query_observability_attempt_count"], 16);
    assert_eq!(value["query_registered_count"], 0);
    assert_eq!(value["query_executed_count"], 0);
    assert_eq!(value["query_result_recorded_count"], 0);
    assert_eq!(value["query_result_persisted_count"], 0);
    assert_eq!(value["search_index_recorded_count"], 0);
    assert_eq!(value["search_index_persisted_count"], 0);
    assert_eq!(value["export_requested_count"], 0);
    assert_eq!(value["export_snapshot_recorded_count"], 0);
    assert_eq!(value["export_snapshot_persisted_count"], 0);
    assert_eq!(value["export_file_written_count"], 0);
    assert_eq!(value["observability_metric_recorded_count"], 0);
    assert_eq!(value["observability_event_recorded_count"], 0);
    assert_eq!(value["dashboard_panel_recorded_count"], 0);
    assert_eq!(value["operator_summary_recorded_count"], 0);
    assert_eq!(value["readback_surface_recorded_count"], 0);
    assert_eq!(value["audit_view_recorded_count"], 0);
    assert_eq!(value["external_delivery_performed_count"], 0);
    assert_eq!(value["completion_ack_recorded_count"], 0);
    assert_eq!(
        value["export_query_observability_acceptance_recorded_count"],
        0
    );
    assert_eq!(
        value["export_query_observability_operator_approval_derived_count"],
        0
    );
    assert_eq!(
        value["export_query_observability_activation_authority_derived_count"],
        0
    );
    assert_eq!(
        value["export_query_observability_activation_command_derived_count"],
        0
    );
    assert_eq!(
        value["export_query_observability_live_execution_allowed_count"],
        0
    );

    let surfaces = value["export_query_observability_surfaces"]
        .as_array()
        .expect("packet acceptance receipt export query observability surfaces");
    assert_eq!(surfaces.len(), 16);
    assert_eq!(
        surfaces[0]["export_query_observability_surface"],
        "packet_receipt_query_registration_claim"
    );
    for surface in surfaces {
        assert_eq!(surface["export_query_or_observability_attempted"], true);
        assert_eq!(surface["query_registered"], false);
        assert_eq!(surface["query_executed"], false);
        assert_eq!(surface["query_result_recorded"], false);
        assert_eq!(surface["search_index_recorded"], false);
        assert_eq!(surface["export_requested"], false);
        assert_eq!(surface["export_snapshot_recorded"], false);
        assert_eq!(surface["export_file_written"], false);
        assert_eq!(surface["observability_metric_recorded"], false);
        assert_eq!(surface["observability_event_recorded"], false);
        assert_eq!(surface["dashboard_panel_recorded"], false);
        assert_eq!(surface["operator_summary_recorded"], false);
        assert_eq!(surface["readback_surface_recorded"], false);
        assert_eq!(surface["audit_view_recorded"], false);
        assert_eq!(surface["external_delivery_performed"], false);
        assert_eq!(surface["acceptance_recorded"], false);
        assert_eq!(surface["operator_approval_derived"], false);
        assert_eq!(surface["activation_authority_derived"], false);
        assert_eq!(surface["activation_command_derived"], false);
        assert_eq!(surface["live_execution_allowed"], false);
        assert_eq!(
            surface["export_query_observability_status"],
            "export_query_observability_denied"
        );
    }

    let denied = value["denied_by_packet_receipt_export_query_observability"]
        .as_array()
        .expect("packet acceptance receipt export query observability denials");
    assert_eq!(denied.len(), 18);
    assert_eq!(
        value["denied_by_packet_receipt_export_query_observability_count"],
        serde_json::json!(denied.len())
    );
    let next_actions = value["allowed_next_actions"]
        .as_array()
        .expect("packet acceptance receipt export query observability next actions");
    assert_eq!(
        next_actions[0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_redaction_privacy_denial_gate"
    );
    assert_eq!(next_actions[0]["status"], "allowed_report_only_next_slice");
    assert_eq!(value["packet_acceptance_receipt_query_registered"], false);
    assert_eq!(value["packet_acceptance_receipt_query_executed"], false);
    assert_eq!(
        value["packet_acceptance_receipt_query_result_recorded"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_export_snapshot_recorded"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_export_file_written"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_observability_metric_recorded"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_observability_event_recorded"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_dashboard_panel_recorded"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_operator_summary_recorded"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_readback_surface_recorded"],
        false
    );
    assert_eq!(
        value["packet_acceptance_receipt_external_delivery_performed"],
        false
    );
    assert_eq!(value["operator_acceptance_recorded"], false);
    assert_eq!(value["operator_approval_recorded"], false);
    assert_eq!(value["activation_authority_derived"], false);
    assert_eq!(value["activation_command_derived"], false);
    assert_eq!(value["activation_allowed"], false);
    assert_eq!(value["activation_performed"], false);
    assert_eq!(value["memory_store_write_performed"], false);
    assert_eq!(value["memory_store_mutated"], false);
    assert_eq!(value["live_kg_write_performed"], false);
    assert_eq!(value["provider_invoked"], false);
    assert_eq!(value["model_invoked"], false);
    assert_eq!(value["credential_read"], false);
    assert_eq!(value["install_executed"], false);
    assert_eq!(value["service_restarted"], false);
    assert_eq!(value["active_binary_mutated"], false);
    assert_eq!(value["release_artifact_written"], false);
    assert_eq!(value["external_send_performed"], false);
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_query_registered"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_export_snapshot_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_observability_metric_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_external_delivery_performed"],
        false
    );
    assert_eq!(
        value["side_effects"]["packet_acceptance_receipt_live_execution_allowed"],
        false
    );
    assert_eq!(value["side_effects"]["activation_authority_derived"], false);
    assert_eq!(value["side_effects"]["activation_performed"], false);
    assert_eq!(value["side_effects"]["memory_store_write_performed"], false);
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(value["side_effects"]["install_executed"], false);
    assert_eq!(value["side_effects"]["external_send_performed"], false);
    assert_eq!(value["side_effects"]["filesystem_written"], false);
}
