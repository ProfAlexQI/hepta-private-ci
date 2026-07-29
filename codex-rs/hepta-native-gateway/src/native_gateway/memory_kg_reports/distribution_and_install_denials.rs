fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_external_delivery_non_persistence_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_report =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_queue_artifact_availability_status_denial_report();
    let source_ready = source_report["status"].as_str() == Some("ready")
        && source_report["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_queue_artifact_availability_status_denial_ready"]
            .as_bool()
            .unwrap_or(false);
    let source_u64 = |key: &str| source_report[key].as_u64().unwrap_or(0);
    let source_report_sha256 = sha256_text_value(&source_report.to_string());
    let source_contract_hash = source_report
        ["release_publication_result_receipt_terminal_distribution_artifact_status_contract_hash_sha256"]
        .as_str()
        .unwrap_or("unknown")
        .to_string();
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let terminal_distribution_delivery_receipt_surfaces = [
        "publication_result_receipt_distribution_delivery_receipt_creation",
        "publication_result_receipt_distribution_delivery_receipt_recording",
        "publication_result_receipt_distribution_delivery_receipt_persistence",
        "publication_result_receipt_distribution_delivery_receipt_filesystem_materialization",
        "publication_result_receipt_distribution_delivery_receipt_ledger_index",
        "publication_result_receipt_distribution_queue_delivery_ack",
        "publication_result_receipt_artifact_download_delivery_ack",
        "publication_result_receipt_package_index_delivery_ack",
        "publication_result_receipt_update_feed_delivery_ack",
        "publication_result_receipt_cdn_mirror_delivery_ack",
        "publication_result_receipt_release_channel_delivery_ack",
        "publication_result_receipt_public_bucket_delivery_ack",
        "publication_result_receipt_status_endpoint_delivery_receipt",
        "publication_result_receipt_dashboard_delivery_badge",
        "publication_result_receipt_channel_delivery_receipt",
        "publication_result_receipt_external_webhook_delivery_receipt",
        "publication_result_receipt_telegram_delivery_receipt",
        "publication_result_receipt_authority_live_active_binary_delivery_receipt",
    ]
    .into_iter()
    .map(|surface| {
        let mut surface_report = serde_json::json!({
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_surface": surface,
            "source_terminal_distribution_artifact_status_ready": true,
            "terminal_distribution_delivery_receipt_attempted": true,
            "terminal_distribution_delivery_receipt_allowed": false,
            "terminal_distribution_delivery_receipt_request_accepted": false,
            "terminal_distribution_delivery_receipt_accepted": false,
            "terminal_distribution_delivery_receipt_recorded": false,
            "terminal_distribution_delivery_receipt_persisted": false,
            "terminal_distribution_delivery_receipt_materialized": false,
            "terminal_distribution_delivery_receipt_filesystem_written": false,
            "terminal_distribution_delivery_receipt_ledger_written": false,
            "terminal_distribution_delivery_receipt_index_written": false,
            "terminal_distribution_delivery_receipt_queued": false,
            "terminal_distribution_delivery_receipt_delivered": false,
            "terminal_distribution_delivery_receipt_externally_sent": false,
            "terminal_distribution_delivery_receipt_channel_sent": false,
            "terminal_distribution_delivery_receipt_webhook_sent": false,
            "terminal_distribution_delivery_receipt_telegram_sent": false,
            "status_endpoint_delivery_receipt_exposed": false,
            "dashboard_delivery_receipt_exposed": false,
            "delivery_confirmation_recorded": false,
            "delivery_ack_recorded": false,
            "receipt_echo_delivered": false,
            "downstream_consumer_notified": false,
        });
        extend_json_object(
            &mut surface_report,
            serde_json::json!({
                "delivery_receipt_acceptance_recorded": false,
                "operator_approval_derived": false,
                "release_publication_authority_derived": false,
                "activation_authority_derived": false,
                "activation_command_derived": false,
                "live_execution_allowed": false,
                "activation_performed": false,
                "install_executed": false,
                "service_restarted": false,
                "launchd_mutated": false,
                "active_binary_mutated": false,
                "release_artifact_written": false,
                "public_artifact_written": false,
                "memory_store_write_performed": false,
                "memory_store_mutated": false,
                "live_kg_write_performed": false,
                "provider_invoked": false,
                "model_invoked": false,
                "credential_read": false,
                "secret_file_read": false,
                "external_send_performed": false,
                "terminal_distribution_delivery_receipt_noop_confirmed": true,
                "terminal_distribution_delivery_receipt_status": "terminal_distribution_delivery_receipt_denied"
            }),
        );
        surface_report
    })
    .collect::<Vec<_>>();
    let terminal_distribution_delivery_receipt_surface_count =
        terminal_distribution_delivery_receipt_surfaces.len();
    let contract_hash = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-external-delivery-non-persistence-denial:native:source={source_report_sha256}:surfaces={terminal_distribution_delivery_receipt_surface_count}:route_count={}:delivery=0:external=0:authority=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "release-publication-result-receipt-terminal-distribution-delivery-receipt-denial:no-delivery-receipt:no-ledger:no-index:no-channel:no-webhook:no-telegram:no-authority",
    );
    let denials = vec![
        "source_terminal_distribution_artifact_status_report_required",
        "delivery_receipt_request_acceptance_denied",
        "delivery_receipt_acceptance_denied",
        "delivery_receipt_recording_denied",
        "delivery_receipt_persistence_denied",
        "delivery_receipt_materialization_denied",
        "delivery_receipt_filesystem_write_denied",
        "delivery_receipt_ledger_write_denied",
        "delivery_receipt_index_write_denied",
        "delivery_receipt_queue_denied",
        "delivery_receipt_delivery_denied",
        "delivery_receipt_external_send_denied",
        "delivery_receipt_channel_send_denied",
        "delivery_receipt_webhook_send_denied",
        "delivery_receipt_telegram_send_denied",
        "distribution_queue_delivery_ack_denied",
        "artifact_download_delivery_ack_denied",
        "package_index_delivery_ack_denied",
        "update_feed_delivery_ack_denied",
        "cdn_mirror_delivery_ack_denied",
        "release_channel_delivery_ack_denied",
        "public_bucket_delivery_ack_denied",
        "status_endpoint_delivery_receipt_denied",
        "dashboard_delivery_receipt_denied",
        "delivery_confirmation_recording_denied",
        "delivery_ack_recording_denied",
        "receipt_echo_delivery_denied",
        "downstream_consumer_notification_denied",
        "release_artifact_write_denied",
        "public_artifact_write_denied",
        "acceptance_from_delivery_receipt_denied",
        "operator_approval_from_delivery_receipt_denied",
        "release_publication_authority_from_delivery_receipt_denied",
        "activation_live_from_delivery_receipt_denied",
        "install_restart_active_binary_from_delivery_receipt_denied",
        "memory_provider_kg_from_delivery_receipt_denied",
    ];
    let denied_count = denials.len();
    let report_ready = source_ready
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_artifact_status_surface_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_artifact_status_attempt_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_artifact_status_accepted_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_artifact_status_recorded_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_artifact_status_persisted_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_artifact_status_delivered_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_artifact_status_exposed_count",
        ) == 0
        && source_u64("release_publication_result_receipt_distribution_queue_enqueued_count") == 0
        && source_u64("release_publication_result_receipt_distribution_worker_dispatched_count")
            == 0
        && source_u64("release_publication_result_receipt_channel_status_delivered_count") == 0
        && source_u64("release_publication_result_receipt_external_status_sent_count") == 0
        && source_u64("release_publication_result_receipt_telegram_status_sent_count") == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_artifact_status_release_publication_authority_derived_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_artifact_status_activation_authority_derived_count",
        ) == 0
        && terminal_distribution_delivery_receipt_surface_count == 18
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_external_delivery_non_persistence_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_EXTERNAL_DELIVERY_NON_PERSISTENCE_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-external-delivery-non-persistence-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-18",
        "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_external_delivery_non_persistence_denial_v1",
        "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_mode": "native_route_denied_terminal_distribution_status_cannot_become_delivery_receipt_or_external_delivery",
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_queue_artifact_availability_status_denial_route",
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_ready": source_ready,
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_report_sha256": source_report_sha256,
        "source_release_publication_result_receipt_terminal_distribution_artifact_status_contract_hash_sha256": source_contract_hash,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_contract_hash_sha256": contract_hash,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_policy_hash_sha256": policy_hash,
        "minimum_required_samples": 24,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "source_route_wired": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_external_delivery_non_persistence_denial_route_enabled": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_external_delivery_non_persistence_denial_ready": report_ready,
    });

    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_release_publication_result_receipt_terminal_distribution_artifact_status_surface_count": source_u64("release_publication_result_receipt_terminal_distribution_artifact_status_surface_count"),
            "source_release_publication_result_receipt_terminal_distribution_artifact_status_attempt_count": source_u64("release_publication_result_receipt_terminal_distribution_artifact_status_attempt_count"),
            "source_release_publication_result_receipt_terminal_distribution_artifact_status_accepted_count": source_u64("release_publication_result_receipt_terminal_distribution_artifact_status_accepted_count"),
            "source_release_publication_result_receipt_terminal_distribution_artifact_status_recorded_count": source_u64("release_publication_result_receipt_terminal_distribution_artifact_status_recorded_count"),
            "source_release_publication_result_receipt_terminal_distribution_artifact_status_persisted_count": source_u64("release_publication_result_receipt_terminal_distribution_artifact_status_persisted_count"),
            "source_release_publication_result_receipt_terminal_distribution_artifact_status_delivered_count": source_u64("release_publication_result_receipt_terminal_distribution_artifact_status_delivered_count"),
            "source_release_publication_result_receipt_terminal_distribution_artifact_status_exposed_count": source_u64("release_publication_result_receipt_terminal_distribution_artifact_status_exposed_count"),
            "source_release_publication_result_receipt_distribution_queue_enqueued_count": source_u64("release_publication_result_receipt_distribution_queue_enqueued_count"),
            "source_release_publication_result_receipt_distribution_worker_dispatched_count": source_u64("release_publication_result_receipt_distribution_worker_dispatched_count"),
            "source_release_publication_result_receipt_artifact_download_url_exposed_count": source_u64("release_publication_result_receipt_artifact_download_url_exposed_count"),
            "source_release_publication_result_receipt_channel_status_delivered_count": source_u64("release_publication_result_receipt_channel_status_delivered_count"),
            "source_release_publication_result_receipt_external_status_sent_count": source_u64("release_publication_result_receipt_external_status_sent_count"),
            "source_release_publication_result_receipt_telegram_status_sent_count": source_u64("release_publication_result_receipt_telegram_status_sent_count"),
            "source_release_publication_result_receipt_terminal_distribution_artifact_status_release_publication_authority_derived_count": source_u64("release_publication_result_receipt_terminal_distribution_artifact_status_release_publication_authority_derived_count"),
            "source_release_publication_result_receipt_terminal_distribution_artifact_status_activation_authority_derived_count": source_u64("release_publication_result_receipt_terminal_distribution_artifact_status_activation_authority_derived_count"),
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_surface_count": terminal_distribution_delivery_receipt_surface_count,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_attempt_count": terminal_distribution_delivery_receipt_surface_count,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_allowed_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_request_accepted_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_accepted_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_recorded_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_persisted_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_materialized_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_filesystem_written_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_ledger_written_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_index_written_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_queued_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_delivered_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_externally_sent_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_channel_sent_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_webhook_sent_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_sent_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_status_endpoint_delivery_receipt_exposed_count": 0,
            "release_publication_result_receipt_dashboard_delivery_receipt_exposed_count": 0,
            "release_publication_result_receipt_delivery_confirmation_recorded_count": 0,
            "release_publication_result_receipt_delivery_ack_recorded_count": 0,
            "release_publication_result_receipt_receipt_echo_delivered_count": 0,
            "release_publication_result_receipt_downstream_consumer_notified_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_acceptance_recorded_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_approval_derived_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_release_publication_authority_derived_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_activation_authority_derived_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_activation_command_derived_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_live_execution_allowed_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_install_executed_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_service_restarted_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_active_binary_mutated_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_release_artifact_written_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_public_artifact_written_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_surfaces": terminal_distribution_delivery_receipt_surfaces,
            "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt": denials,
            "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_count": denied_count,
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_denial_gate",
                    "status": "allowed_report_only_next_slice",
                    "records_delivery_receipt": false,
                    "persists_delivery_receipt": false,
                    "sends_externally": false,
                    "records_operator_acceptance": false,
                    "derives_release_publication_authority": false,
                    "derives_activation_authority": false,
                    "activates_live": false,
                    "mutates_memory_store": false,
                    "writes_kg": false
                }
            ],
        }),
    );

    let delivery_receipt_false_keys = [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_delivered",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_distribution_queue_enqueued",
        "packet_acceptance_receipt_release_publication_result_receipt_distribution_worker_dispatched",
        "packet_acceptance_receipt_release_publication_result_receipt_artifact_download_url_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_channel_status_delivered",
        "packet_acceptance_receipt_release_publication_result_receipt_external_status_sent",
        "packet_acceptance_receipt_release_publication_result_receipt_telegram_status_sent",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_allowed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_request_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_materialized",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_filesystem_written",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_ledger_written",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_index_written",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_queued",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_delivered",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_externally_sent",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_channel_sent",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_webhook_sent",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_sent",
        "packet_acceptance_receipt_release_publication_result_receipt_status_endpoint_delivery_receipt_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_dashboard_delivery_receipt_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_delivery_confirmation_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_delivery_ack_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_receipt_echo_delivered",
        "packet_acceptance_receipt_release_publication_result_receipt_downstream_consumer_notified",
        "packet_acceptance_receipt_release_publication_result_receipt_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_persisted",
        "packet_acceptance_receipt_release_publication_recorded",
        "operator_acceptance_recorded",
        "operator_approval_recorded",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "activation_command_derived",
        "activation_allowed",
        "activation_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
        "public_release_claimed",
        "public_ga_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "external_send_performed",
        "filesystem_written",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in delivery_receipt_false_keys {
            report_object.insert(key.to_string(), serde_json::json!(false));
        }
    }

    let mut side_effects = serde_json::Map::new();
    for key in delivery_receipt_false_keys {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_report =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_external_delivery_non_persistence_denial_report();
    let source_ready = source_report["status"].as_str() == Some("ready")
        && source_report["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_external_delivery_non_persistence_denial_ready"]
            .as_bool()
            .unwrap_or(false);
    let source_u64 = |key: &str| source_report[key].as_u64().unwrap_or(0);
    let source_report_sha256 = sha256_json_value(&source_report);
    let source_contract_hash = source_report
        ["release_publication_result_receipt_terminal_distribution_delivery_receipt_contract_hash_sha256"]
        .as_str()
        .unwrap_or("unknown")
        .to_string();
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let surface_specs: Vec<(&str, &str, &str, Vec<&str>)> = vec![
        (
            "publication_result_receipt_delivery_receipt_query_registration",
            "blocked_delivery_receipt_query_registration_noop",
            "delivery_receipt_query_registration_denied",
            vec!["query_registration_requested"],
        ),
        (
            "publication_result_receipt_delivery_receipt_query_execution",
            "blocked_delivery_receipt_query_execution_noop",
            "delivery_receipt_query_execution_denied",
            vec!["query_execution_requested"],
        ),
        (
            "publication_result_receipt_delivery_receipt_query_result",
            "blocked_delivery_receipt_query_result_noop",
            "delivery_receipt_query_result_denied",
            vec!["query_result_requested"],
        ),
        (
            "publication_result_receipt_delivery_receipt_search_index",
            "blocked_delivery_receipt_search_index_noop",
            "delivery_receipt_search_index_denied",
            vec!["search_index_requested"],
        ),
        (
            "publication_result_receipt_delivery_receipt_export_request",
            "blocked_delivery_receipt_export_request_noop",
            "delivery_receipt_export_request_denied",
            vec!["export_request_requested"],
        ),
        (
            "publication_result_receipt_delivery_receipt_export_snapshot",
            "blocked_delivery_receipt_export_snapshot_noop",
            "delivery_receipt_export_snapshot_denied",
            vec!["export_snapshot_requested"],
        ),
        (
            "publication_result_receipt_delivery_receipt_export_file",
            "blocked_delivery_receipt_export_file_noop",
            "delivery_receipt_export_file_denied",
            vec!["export_file_requested"],
        ),
        (
            "publication_result_receipt_delivery_receipt_export_stream",
            "blocked_delivery_receipt_export_stream_noop",
            "delivery_receipt_export_stream_denied",
            vec!["export_stream_requested"],
        ),
        (
            "publication_result_receipt_delivery_receipt_observability_metric",
            "blocked_delivery_receipt_observability_metric_noop",
            "delivery_receipt_observability_metric_denied",
            vec!["observability_metric_requested"],
        ),
        (
            "publication_result_receipt_delivery_receipt_observability_log",
            "blocked_delivery_receipt_observability_log_noop",
            "delivery_receipt_observability_log_denied",
            vec!["observability_log_requested"],
        ),
        (
            "publication_result_receipt_delivery_receipt_observability_trace",
            "blocked_delivery_receipt_observability_trace_noop",
            "delivery_receipt_observability_trace_denied",
            vec!["observability_trace_requested"],
        ),
        (
            "publication_result_receipt_delivery_receipt_observability_event",
            "blocked_delivery_receipt_observability_event_noop",
            "delivery_receipt_observability_event_denied",
            vec!["observability_event_requested"],
        ),
        (
            "publication_result_receipt_delivery_receipt_dashboard_panel",
            "blocked_delivery_receipt_dashboard_panel_noop",
            "delivery_receipt_dashboard_panel_denied",
            vec!["dashboard_panel_requested"],
        ),
        (
            "publication_result_receipt_delivery_receipt_alert_slo",
            "blocked_delivery_receipt_alert_slo_noop",
            "delivery_receipt_alert_slo_denied",
            vec!["alert_slo_requested"],
        ),
        (
            "publication_result_receipt_delivery_receipt_operator_readback",
            "blocked_delivery_receipt_operator_readback_noop",
            "delivery_receipt_operator_readback_denied",
            vec!["operator_readback_requested"],
        ),
        (
            "publication_result_receipt_delivery_receipt_audit_view",
            "blocked_delivery_receipt_audit_view_noop",
            "delivery_receipt_audit_view_denied",
            vec!["audit_view_requested"],
        ),
        (
            "publication_result_receipt_release_publication_authority_observability",
            "blocked_release_publication_authority_observability_noop",
            "release_publication_authority_from_observability_denied",
            vec!["release_publication_authority_observability_requested"],
        ),
        (
            "publication_result_receipt_activation_live_active_binary_observability",
            "blocked_activation_live_active_binary_observability_noop",
            "activation_live_active_binary_from_observability_denied",
            vec![
                "activation_live_observability_requested",
                "install_restart_active_binary_observability_requested",
            ],
        ),
    ];

    let query_export_observability_surfaces = surface_specs
        .into_iter()
        .map(|(surface, status, reason, requested_flags)| {
            let mut surface_report = serde_json::json!({
                "release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_surface": surface,
                "source_terminal_distribution_delivery_receipt_ready": true,
                "query_export_observability_attempted": true,
                "query_export_observability_status": status,
                "reason": reason,
            });
            extend_json_object(
                &mut surface_report,
                serde_json::json!({
                "query_export_observability_allowed": false,
                "query_export_observability_request_accepted": false,
                "query_export_observability_accepted": false,
                "query_export_observability_recorded": false,
                "query_export_observability_persisted": false,
                "query_export_observability_materialized": false,
                "query_export_observability_filesystem_written": false,
                "query_export_observability_delivered": false,
                "query_export_observability_exposed": false,
                }),
            );
            extend_json_object(
                &mut surface_report,
                serde_json::json!({
                "query_registration_performed": false,
                "query_execution_performed": false,
                "query_result_exposed": false,
                "search_index_written": false,
                "export_request_accepted": false,
                "export_snapshot_recorded": false,
                "export_file_written": false,
                "export_stream_opened": false,
                "observability_metric_recorded": false,
                "observability_log_recorded": false,
                "observability_trace_recorded": false,
                "observability_event_recorded": false,
                "dashboard_panel_exposed": false,
                "alert_slo_recorded": false,
                "operator_readback_exposed": false,
                "audit_view_exposed": false,
                "delivery_receipt_status_evidence_exposed": false,
                }),
            );
            extend_json_object(
                &mut surface_report,
                serde_json::json!({
                "acceptance_recorded": false,
                "operator_approval_derived": false,
                "release_publication_authority_derived": false,
                "activation_authority_derived": false,
                "activation_command_derived": false,
                "live_execution_allowed": false,
                "activation_performed": false,
                "install_executed": false,
                "service_restarted": false,
                "launchd_mutated": false,
                "active_binary_mutated": false,
                "release_artifact_written": false,
                "public_artifact_written": false,
                "memory_store_write_performed": false,
                "memory_store_mutated": false,
                "live_kg_write_performed": false,
                }),
            );
            extend_json_object(
                &mut surface_report,
                serde_json::json!({
                "provider_invoked": false,
                "model_invoked": false,
                "credential_read": false,
                "secret_file_read": false,
                "external_send_performed": false,
                "query_export_observability_noop_confirmed": true,
                }),
            );
            if let Some(surface_object) = surface_report.as_object_mut() {
                for flag in requested_flags {
                    surface_object.insert(flag.to_string(), serde_json::json!(true));
                }
            }
            surface_report
        })
        .collect::<Vec<_>>();
    let query_export_observability_surface_count = query_export_observability_surfaces.len();
    let contract_hash = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-query-export-observability-denial:native:source={source_report_sha256}:surfaces={query_export_observability_surface_count}:route_count={}:query=0:export=0:observability=0:authority=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "release-publication-result-receipt-terminal-distribution-delivery-receipt-query-export-observability-denial:no-query:no-export:no-observability:no-dashboard:no-alert:no-authority",
    );
    let denials = vec![
        "source_terminal_distribution_delivery_receipt_report_required",
        "query_export_observability_request_acceptance_denied",
        "query_export_observability_acceptance_denied",
        "query_export_observability_recording_denied",
        "query_export_observability_persistence_denied",
        "query_export_observability_materialization_denied",
        "query_export_observability_filesystem_write_denied",
        "query_export_observability_delivery_denied",
        "query_export_observability_exposure_denied",
        "delivery_receipt_query_registration_denied",
        "delivery_receipt_query_execution_denied",
        "delivery_receipt_query_result_exposure_denied",
        "delivery_receipt_search_index_write_denied",
        "delivery_receipt_export_request_denied",
        "delivery_receipt_export_snapshot_denied",
        "delivery_receipt_export_file_write_denied",
        "delivery_receipt_export_stream_denied",
        "delivery_receipt_observability_metric_denied",
        "delivery_receipt_observability_log_denied",
        "delivery_receipt_observability_trace_denied",
        "delivery_receipt_observability_event_denied",
        "delivery_receipt_dashboard_panel_denied",
        "delivery_receipt_alert_slo_denied",
        "delivery_receipt_operator_readback_denied",
        "delivery_receipt_audit_view_denied",
        "delivery_receipt_status_evidence_denied",
        "acceptance_from_delivery_receipt_observability_denied",
        "operator_approval_from_delivery_receipt_observability_denied",
        "release_publication_authority_from_delivery_receipt_observability_denied",
        "activation_live_from_delivery_receipt_observability_denied",
        "install_restart_active_binary_from_delivery_receipt_observability_denied",
        "release_artifact_write_denied",
        "public_artifact_write_denied",
        "memory_provider_kg_from_delivery_receipt_observability_denied",
    ];
    let denied_count = denials.len();
    let report_ready = source_ready
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_surface_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_attempt_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_accepted_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_recorded_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_persisted_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_delivered_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_externally_sent_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_sent_count",
        ) == 0
        && source_u64("release_publication_result_receipt_downstream_consumer_notified_count") == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_release_publication_authority_derived_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_activation_authority_derived_count",
        ) == 0
        && query_export_observability_surface_count == 18
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_QUERY_EXPORT_OBSERVABILITY_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-query-export-observability-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-18",
        "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_denial_v1",
        "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_mode": "native_route_denied_delivery_receipt_cannot_become_query_export_observability_or_authority",
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_external_delivery_non_persistence_denial_route",
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_ready": source_ready,
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_report_sha256": source_report_sha256,
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_contract_hash_sha256": source_contract_hash,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_contract_hash_sha256": contract_hash,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_policy_hash_sha256": policy_hash,
        "minimum_required_samples": 24,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "source_route_wired": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_denial_route_enabled": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_denial_ready": report_ready,
    });

    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_surface_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_surface_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_attempt_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_attempt_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_accepted_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_accepted_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_recorded_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_recorded_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_persisted_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_persisted_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_delivered_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_delivered_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_externally_sent_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_externally_sent_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_sent_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_sent_count"),
            "source_release_publication_result_receipt_downstream_consumer_notified_count": source_u64("release_publication_result_receipt_downstream_consumer_notified_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_release_publication_authority_derived_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_release_publication_authority_derived_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_activation_authority_derived_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_activation_authority_derived_count"),
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_surface_count": query_export_observability_surface_count,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_attempt_count": query_export_observability_surface_count,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_allowed_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_request_accepted_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_accepted_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_recorded_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_persisted_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_materialized_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_filesystem_written_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_delivered_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_exposed_count": 0,
            "release_publication_result_receipt_delivery_receipt_query_registered_count": 0,
            "release_publication_result_receipt_delivery_receipt_query_executed_count": 0,
            "release_publication_result_receipt_delivery_receipt_query_result_exposed_count": 0,
            "release_publication_result_receipt_delivery_receipt_search_index_written_count": 0,
            "release_publication_result_receipt_delivery_receipt_export_requested_count": 0,
            "release_publication_result_receipt_delivery_receipt_export_snapshot_recorded_count": 0,
            "release_publication_result_receipt_delivery_receipt_export_file_written_count": 0,
            "release_publication_result_receipt_delivery_receipt_export_stream_opened_count": 0,
            "release_publication_result_receipt_delivery_receipt_observability_metric_recorded_count": 0,
            "release_publication_result_receipt_delivery_receipt_observability_log_recorded_count": 0,
            "release_publication_result_receipt_delivery_receipt_observability_trace_recorded_count": 0,
            "release_publication_result_receipt_delivery_receipt_observability_event_recorded_count": 0,
            "release_publication_result_receipt_delivery_receipt_dashboard_panel_exposed_count": 0,
            "release_publication_result_receipt_delivery_receipt_alert_slo_recorded_count": 0,
            "release_publication_result_receipt_delivery_receipt_operator_readback_exposed_count": 0,
            "release_publication_result_receipt_delivery_receipt_audit_view_exposed_count": 0,
            "release_publication_result_receipt_delivery_receipt_status_evidence_exposed_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_acceptance_recorded_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_operator_approval_derived_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_release_publication_authority_derived_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_activation_authority_derived_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_activation_command_derived_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_live_execution_allowed_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_install_executed_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_service_restarted_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_active_binary_mutated_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_release_artifact_written_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_public_artifact_written_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_surfaces": query_export_observability_surfaces,
            "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability": denials,
            "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_count": denied_count,
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_denial_gate",
                    "status": "allowed_report_only_next_slice",
                    "exposes_delivery_receipt_query": false,
                    "exports_delivery_receipt": false,
                    "records_observability": false,
                    "records_operator_acceptance": false,
                    "derives_release_publication_authority": false,
                    "derives_activation_authority": false,
                    "activates_live": false,
                    "mutates_memory_store": false,
                    "writes_kg": false,
                    "sends_externally": false
                }
            ],
        }),
    );

    let query_export_observability_false_keys = [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_delivered",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_externally_sent",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_sent",
        "packet_acceptance_receipt_release_publication_result_receipt_downstream_consumer_notified",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_allowed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_request_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_materialized",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_filesystem_written",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_delivered",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_query_registered",
        "packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_query_executed",
        "packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_query_result_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_search_index_written",
        "packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_export_requested",
        "packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_export_snapshot_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_export_file_written",
        "packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_export_stream_opened",
        "packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_observability_metric_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_observability_log_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_observability_trace_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_observability_event_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_dashboard_panel_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_alert_slo_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_operator_readback_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_audit_view_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_status_evidence_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_persisted",
        "packet_acceptance_receipt_release_publication_recorded",
        "operator_acceptance_recorded",
        "operator_approval_recorded",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "activation_command_derived",
        "activation_allowed",
        "activation_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
        "public_release_claimed",
        "public_ga_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "external_send_performed",
        "filesystem_written",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in query_export_observability_false_keys {
            report_object.insert(key.to_string(), serde_json::json!(false));
        }
    }

    let mut side_effects = serde_json::Map::new();
    for key in query_export_observability_false_keys {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_report =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_denial_report();
    let source_ready = source_report["status"].as_str() == Some("ready")
        && source_report["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_denial_ready"]
            .as_bool()
            .unwrap_or(false);
    let source_u64 = |key: &str| source_report[key].as_u64().unwrap_or(0);
    let source_report_sha256 = sha256_json_value(&source_report);
    let source_contract_hash = source_report
        ["release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_contract_hash_sha256"]
        .as_str()
        .unwrap_or("unknown")
        .to_string();
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let surface_specs: Vec<(&str, &str, &str, Vec<&str>)> = vec![
        (
            "delivery_receipt_distribution_artifact_manifest_status",
            "blocked_distribution_artifact_manifest_status_noop",
            "distribution_artifact_manifest_status_denied",
            vec!["distribution_artifact_manifest_status_requested"],
        ),
        (
            "delivery_receipt_package_manifest_status",
            "blocked_package_manifest_status_noop",
            "package_manifest_status_denied",
            vec!["package_manifest_status_requested"],
        ),
        (
            "delivery_receipt_checksum_index_status",
            "blocked_checksum_index_status_noop",
            "checksum_index_status_denied",
            vec!["checksum_index_status_requested"],
        ),
        (
            "delivery_receipt_artifact_metadata_status",
            "blocked_artifact_metadata_status_noop",
            "artifact_metadata_status_denied",
            vec!["artifact_metadata_status_requested"],
        ),
        (
            "delivery_receipt_cdn_artifact_metadata_status",
            "blocked_cdn_artifact_metadata_status_noop",
            "cdn_artifact_metadata_status_denied",
            vec!["cdn_artifact_metadata_status_requested"],
        ),
        (
            "delivery_receipt_update_feed_artifact_metadata_status",
            "blocked_update_feed_artifact_metadata_status_noop",
            "update_feed_artifact_metadata_status_denied",
            vec!["update_feed_artifact_metadata_status_requested"],
        ),
        (
            "delivery_receipt_package_signing_status",
            "blocked_package_signing_status_noop",
            "package_signing_status_denied",
            vec!["package_signing_status_requested"],
        ),
        (
            "delivery_receipt_notarization_status",
            "blocked_notarization_status_noop",
            "notarization_status_denied",
            vec!["notarization_status_requested"],
        ),
        (
            "delivery_receipt_stapling_status",
            "blocked_stapling_status_noop",
            "stapling_status_denied",
            vec!["stapling_status_requested"],
        ),
        (
            "delivery_receipt_provenance_attestation_status",
            "blocked_provenance_attestation_status_noop",
            "provenance_attestation_status_denied",
            vec!["provenance_attestation_status_requested"],
        ),
        (
            "delivery_receipt_sbom_manifest_status",
            "blocked_sbom_manifest_status_noop",
            "sbom_manifest_status_denied",
            vec!["sbom_manifest_status_requested"],
        ),
        (
            "delivery_receipt_artifact_digest_manifest_status",
            "blocked_artifact_digest_manifest_status_noop",
            "artifact_digest_manifest_status_denied",
            vec!["artifact_digest_manifest_status_requested"],
        ),
        (
            "delivery_receipt_release_asset_manifest_status",
            "blocked_release_asset_manifest_status_noop",
            "release_asset_manifest_status_denied",
            vec!["release_asset_manifest_status_requested"],
        ),
        (
            "delivery_receipt_installer_package_manifest_status",
            "blocked_installer_package_manifest_status_noop",
            "installer_package_manifest_status_denied",
            vec!["installer_package_manifest_status_requested"],
        ),
        (
            "delivery_receipt_package_channel_manifest_status",
            "blocked_package_channel_manifest_status_noop",
            "package_channel_manifest_status_denied",
            vec!["package_channel_manifest_status_requested"],
        ),
        (
            "delivery_receipt_external_telegram_artifact_manifest_status",
            "blocked_external_telegram_artifact_manifest_status_noop",
            "external_telegram_artifact_manifest_status_denied",
            vec![
                "external_artifact_manifest_status_requested",
                "telegram_artifact_manifest_status_requested",
            ],
        ),
        (
            "delivery_receipt_release_publication_authority_artifact_manifest_status",
            "blocked_release_publication_authority_artifact_manifest_status_noop",
            "release_publication_authority_from_artifact_manifest_status_denied",
            vec!["release_publication_authority_artifact_manifest_status_requested"],
        ),
        (
            "delivery_receipt_activation_live_install_restart_active_binary_artifact_manifest_status",
            "blocked_activation_live_install_restart_active_binary_artifact_manifest_status_noop",
            "activation_live_install_restart_active_binary_from_artifact_manifest_status_denied",
            vec![
                "activation_live_artifact_manifest_status_requested",
                "install_restart_active_binary_artifact_manifest_status_requested",
            ],
        ),
    ];

    let distribution_artifact_manifest_status_surfaces = surface_specs
        .into_iter()
        .map(|(surface, status, reason, requested_flags)| {
            let mut surface_report = serde_json::json!({
                "release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_surface": surface,
                "source_terminal_distribution_delivery_receipt_query_export_observability_ready": true,
                "distribution_artifact_manifest_status_attempted": true,
                "distribution_artifact_manifest_status_status": status,
                "reason": reason,
            });
            extend_json_object(
                &mut surface_report,
                serde_json::json!({
                    "distribution_artifact_manifest_status_allowed": false,
                    "distribution_artifact_manifest_status_request_accepted": false,
                    "distribution_artifact_manifest_status_accepted": false,
                    "distribution_artifact_manifest_status_recorded": false,
                    "distribution_artifact_manifest_status_persisted": false,
                    "distribution_artifact_manifest_status_materialized": false,
                    "distribution_artifact_manifest_status_filesystem_written": false,
                    "distribution_artifact_manifest_status_delivered": false,
                    "distribution_artifact_manifest_status_exposed": false,
                }),
            );
            extend_json_object(
                &mut surface_report,
                serde_json::json!({
                    "distribution_artifact_manifest_exposed": false,
                    "package_manifest_status_exposed": false,
                    "checksum_index_status_exposed": false,
                    "artifact_metadata_status_exposed": false,
                    "cdn_artifact_metadata_status_exposed": false,
                    "update_feed_artifact_metadata_status_exposed": false,
                    "package_signing_status_exposed": false,
                    "notarization_status_exposed": false,
                    "stapling_status_exposed": false,
                    "provenance_attestation_status_exposed": false,
                    "sbom_manifest_status_exposed": false,
                    "artifact_digest_manifest_status_exposed": false,
                    "release_asset_manifest_status_exposed": false,
                    "installer_package_manifest_status_exposed": false,
                    "package_channel_manifest_status_exposed": false,
                    "external_artifact_manifest_status_sent": false,
                    "telegram_artifact_manifest_status_sent": false,
                }),
            );
            extend_json_object(
                &mut surface_report,
                serde_json::json!({
                    "public_release_claimed": false,
                    "public_ga_claimed": false,
                    "acceptance_recorded": false,
                    "operator_approval_derived": false,
                    "release_publication_authority_derived": false,
                    "activation_authority_derived": false,
                    "activation_command_derived": false,
                    "live_execution_allowed": false,
                    "activation_performed": false,
                    "install_executed": false,
                    "service_restarted": false,
                    "launchd_mutated": false,
                    "active_binary_mutated": false,
                    "memory_store_write_performed": false,
                    "memory_store_mutated": false,
                    "live_kg_write_performed": false,
                    "provider_invoked": false,
                    "model_invoked": false,
                    "credential_read": false,
                    "secret_file_read": false,
                    "distribution_artifact_manifest_status_noop_confirmed": true,
                }),
            );
            if let Some(surface_object) = surface_report.as_object_mut() {
                for flag in requested_flags {
                    surface_object.insert(flag.to_string(), serde_json::json!(true));
                }
            }
            surface_report
        })
        .collect::<Vec<_>>();
    let distribution_artifact_manifest_status_surface_count =
        distribution_artifact_manifest_status_surfaces.len();
    let contract_hash = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-distribution-artifact-manifest-status-denial:native:source={source_report_sha256}:surfaces={distribution_artifact_manifest_status_surface_count}:route_count={}:manifest=0:signing=0:notarization=0:authority=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "release-publication-result-receipt-terminal-distribution-delivery-receipt-distribution-artifact-manifest-status-denial:no-artifact-manifest:no-package-manifest:no-checksum-index:no-cdn-update-feed-metadata:no-signing-no-notarization-no-live",
    );
    let denials = vec![
        "source_terminal_distribution_delivery_receipt_query_export_observability_report_required",
        "distribution_artifact_manifest_status_request_acceptance_denied",
        "distribution_artifact_manifest_status_acceptance_denied",
        "distribution_artifact_manifest_status_recording_denied",
        "distribution_artifact_manifest_status_persistence_denied",
        "distribution_artifact_manifest_status_materialization_denied",
        "distribution_artifact_manifest_status_filesystem_write_denied",
        "distribution_artifact_manifest_status_delivery_denied",
        "distribution_artifact_manifest_status_exposure_denied",
        "distribution_artifact_manifest_exposure_denied",
        "package_manifest_status_exposure_denied",
        "checksum_index_status_exposure_denied",
        "artifact_metadata_status_exposure_denied",
        "cdn_artifact_metadata_status_exposure_denied",
        "update_feed_artifact_metadata_status_exposure_denied",
        "package_signing_status_exposure_denied",
        "notarization_status_exposure_denied",
        "stapling_status_exposure_denied",
        "provenance_attestation_status_exposure_denied",
        "sbom_manifest_status_exposure_denied",
        "artifact_digest_manifest_status_exposure_denied",
        "release_asset_manifest_status_exposure_denied",
        "installer_package_manifest_status_exposure_denied",
        "package_channel_manifest_status_exposure_denied",
        "external_artifact_manifest_status_send_denied",
        "telegram_artifact_manifest_status_send_denied",
        "public_release_claim_from_artifact_manifest_status_denied",
        "public_ga_claim_from_artifact_manifest_status_denied",
        "acceptance_from_artifact_manifest_status_denied",
        "operator_approval_from_artifact_manifest_status_denied",
        "release_publication_authority_from_artifact_manifest_status_denied",
        "activation_live_from_artifact_manifest_status_denied",
        "install_restart_active_binary_from_artifact_manifest_status_denied",
        "release_artifact_write_from_artifact_manifest_status_denied",
        "public_artifact_write_from_artifact_manifest_status_denied",
        "memory_provider_kg_from_artifact_manifest_status_denied",
    ];
    let denied_count = denials.len();
    let report_ready = source_ready
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_surface_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_attempt_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_accepted_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_recorded_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_persisted_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_exposed_count",
        ) == 0
        && source_u64("release_publication_result_receipt_delivery_receipt_query_registered_count")
            == 0
        && source_u64(
            "release_publication_result_receipt_delivery_receipt_export_file_written_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_delivery_receipt_observability_metric_recorded_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_release_publication_authority_derived_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_activation_authority_derived_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_live_execution_allowed_count",
        ) == 0
        && distribution_artifact_manifest_status_surface_count == 18
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_DISTRIBUTION_ARTIFACT_MANIFEST_STATUS_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-distribution-artifact-manifest-status-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-25",
        "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_denial_route_v1",
        "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_mode": "native_route_denied_delivery_receipt_cannot_become_distribution_artifact_manifest_package_manifest_checksum_index_metadata_signing_notarization_or_live_status",
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_denial_route",
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_ready": source_ready,
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_report_sha256": source_report_sha256,
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_contract_hash_sha256": source_contract_hash,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_contract_hash_sha256": contract_hash,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_policy_hash_sha256": policy_hash,
        "minimum_required_samples": 24,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "source_route_wired": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_denial_route_enabled": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_denial_ready": report_ready,
    });

    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_surface_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_surface_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_attempt_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_attempt_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_accepted_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_accepted_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_recorded_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_recorded_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_persisted_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_persisted_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_exposed_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_exposed_count"),
            "source_release_publication_result_receipt_delivery_receipt_query_registered_count": source_u64("release_publication_result_receipt_delivery_receipt_query_registered_count"),
            "source_release_publication_result_receipt_delivery_receipt_export_file_written_count": source_u64("release_publication_result_receipt_delivery_receipt_export_file_written_count"),
            "source_release_publication_result_receipt_delivery_receipt_observability_metric_recorded_count": source_u64("release_publication_result_receipt_delivery_receipt_observability_metric_recorded_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_release_publication_authority_derived_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_release_publication_authority_derived_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_activation_authority_derived_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_activation_authority_derived_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_live_execution_allowed_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_live_execution_allowed_count"),
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_surface_count": distribution_artifact_manifest_status_surface_count,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_attempt_count": distribution_artifact_manifest_status_surface_count,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_allowed_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_request_accepted_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_accepted_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_recorded_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_persisted_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_materialized_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_filesystem_written_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_delivered_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_exposed_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_exposed_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_package_manifest_status_exposed_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_checksum_index_status_exposed_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_metadata_status_exposed_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_cdn_artifact_metadata_status_exposed_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_update_feed_artifact_metadata_status_exposed_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_package_signing_status_exposed_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_notarization_status_exposed_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_stapling_status_exposed_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_provenance_attestation_status_exposed_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_sbom_manifest_status_exposed_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_digest_manifest_status_exposed_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_release_asset_manifest_status_exposed_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_installer_package_manifest_status_exposed_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_package_channel_manifest_status_exposed_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_external_artifact_manifest_status_sent_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_artifact_manifest_status_sent_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_acceptance_recorded_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_operator_approval_derived_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_release_publication_authority_derived_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_activation_authority_derived_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_activation_command_derived_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_live_execution_allowed_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_install_executed_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_service_restarted_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_active_binary_mutated_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_release_artifact_written_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_public_artifact_written_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_surfaces": distribution_artifact_manifest_status_surfaces,
            "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status": denials,
            "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_count": denied_count,
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_denial_gate",
                    "status": "allowed_report_only_next_slice",
                    "exposes_artifact_manifest_status": false,
                    "exposes_package_manifest_status": false,
                    "exposes_checksum_index_status": false,
                    "exposes_signing_status": false,
                    "exposes_notarization_status": false,
                    "derives_release_publication_authority": false,
                    "derives_activation_authority": false,
                    "activates_live": false,
                    "installs_or_restarts": false,
                    "mutates_active_binary": false,
                    "mutates_memory_store": false,
                    "writes_kg": false,
                    "invokes_provider": false,
                    "sends_externally": false
                }
            ],
        }),
    );

    let distribution_artifact_manifest_status_false_keys = [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_query_registered",
        "packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_export_file_written",
        "packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_observability_metric_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_allowed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_request_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_materialized",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_filesystem_written",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_delivered",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_manifest_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_checksum_index_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_metadata_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_cdn_artifact_metadata_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_update_feed_artifact_metadata_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_signing_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_notarization_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_stapling_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_provenance_attestation_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_sbom_manifest_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_digest_manifest_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_release_asset_manifest_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_installer_package_manifest_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_channel_manifest_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_external_artifact_manifest_status_sent",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_artifact_manifest_status_sent",
        "operator_approval_recorded",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "activation_command_derived",
        "activation_allowed",
        "activation_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
        "release_artifact_written",
        "public_artifact_written",
        "public_release_claimed",
        "public_ga_claimed",
        "external_send_performed",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in distribution_artifact_manifest_status_false_keys {
            report_object.insert(key.to_string(), serde_json::json!(false));
        }
    }

    let mut side_effects = serde_json::Map::new();
    for key in distribution_artifact_manifest_status_false_keys {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_report =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_denial_report();
    let source_ready = source_report["status"].as_str() == Some("ready")
        && source_report["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_denial_ready"]
            .as_bool()
            .unwrap_or(false);
    let source_u64 = |key: &str| source_report[key].as_u64().unwrap_or(0);
    let source_report_sha256 = sha256_json_value(&source_report);
    let source_contract_hash = source_report
        ["release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_contract_hash_sha256"]
        .as_str()
        .unwrap_or("unknown")
        .to_string();
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let surface_specs: Vec<(&str, &str, &str, Vec<&str>)> = vec![
        (
            "delivery_receipt_artifact_signing_execution",
            "blocked_artifact_signing_execution_noop",
            "artifact_signing_execution_denied",
            vec!["artifact_signing_requested"],
        ),
        (
            "delivery_receipt_package_signing_execution",
            "blocked_package_signing_execution_noop",
            "package_signing_execution_denied",
            vec!["package_signing_requested"],
        ),
        (
            "delivery_receipt_signature_manifest_write",
            "blocked_signature_manifest_write_noop",
            "signature_manifest_write_denied",
            vec!["signature_manifest_write_requested"],
        ),
        (
            "delivery_receipt_signature_checksum_binding",
            "blocked_signature_checksum_binding_noop",
            "signature_checksum_binding_denied",
            vec!["signature_checksum_binding_requested"],
        ),
        (
            "delivery_receipt_notarization_submission",
            "blocked_notarization_submission_noop",
            "notarization_submission_denied",
            vec!["notarization_submission_requested"],
        ),
        (
            "delivery_receipt_notarization_ticket_record",
            "blocked_notarization_ticket_record_noop",
            "notarization_ticket_recording_denied",
            vec!["notarization_ticket_record_requested"],
        ),
        (
            "delivery_receipt_stapling_execution",
            "blocked_stapling_execution_noop",
            "stapling_execution_denied",
            vec!["stapling_execution_requested"],
        ),
        (
            "delivery_receipt_installer_signing_execution",
            "blocked_installer_signing_execution_noop",
            "installer_signing_execution_denied",
            vec!["installer_signing_requested"],
        ),
        (
            "delivery_receipt_provenance_attestation_publication",
            "blocked_provenance_attestation_publication_noop",
            "provenance_attestation_publication_denied",
            vec!["provenance_attestation_publication_requested"],
        ),
        (
            "delivery_receipt_sbom_manifest_publication",
            "blocked_sbom_manifest_publication_noop",
            "sbom_manifest_publication_denied",
            vec!["sbom_manifest_publication_requested"],
        ),
        (
            "delivery_receipt_release_asset_packaging",
            "blocked_release_asset_packaging_noop",
            "release_asset_packaging_denied",
            vec!["release_asset_packaging_requested"],
        ),
        (
            "delivery_receipt_artifact_bundle_packaging",
            "blocked_artifact_bundle_packaging_noop",
            "artifact_bundle_packaging_denied",
            vec!["artifact_bundle_packaging_requested"],
        ),
        (
            "delivery_receipt_cdn_artifact_write",
            "blocked_cdn_artifact_write_noop",
            "cdn_artifact_write_denied",
            vec!["cdn_artifact_write_requested"],
        ),
        (
            "delivery_receipt_update_feed_artifact_write",
            "blocked_update_feed_artifact_write_noop",
            "update_feed_artifact_write_denied",
            vec!["update_feed_artifact_write_requested"],
        ),
        (
            "delivery_receipt_package_registry_artifact_publish",
            "blocked_package_registry_artifact_publish_noop",
            "package_registry_artifact_publish_denied",
            vec!["package_registry_artifact_publish_requested"],
        ),
        (
            "delivery_receipt_external_telegram_package_channel_publication",
            "blocked_external_telegram_package_channel_publication_noop",
            "external_telegram_package_channel_publication_denied",
            vec![
                "external_package_channel_publication_requested",
                "telegram_package_channel_publication_requested",
            ],
        ),
        (
            "delivery_receipt_release_publication_authority_signing_status",
            "blocked_release_publication_authority_signing_status_noop",
            "release_publication_authority_from_signing_status_denied",
            vec!["release_publication_authority_signing_status_requested"],
        ),
        (
            "delivery_receipt_activation_live_install_restart_active_binary_signing_path",
            "blocked_activation_live_install_restart_active_binary_signing_path_noop",
            "activation_live_install_restart_active_binary_from_signing_path_denied",
            vec![
                "activation_live_signing_path_requested",
                "install_restart_active_binary_signing_path_requested",
            ],
        ),
    ];

    let artifact_distribution_signing_notarization_surfaces = surface_specs
        .into_iter()
        .map(|(surface, status, reason, requested_flags)| {
            let mut surface_report = serde_json::json!({
                "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface": surface,
                "source_distribution_artifact_manifest_status_ready": true,
                "artifact_distribution_signing_notarization_surface_attempted": true,
                "artifact_distribution_signing_notarization_surface_status": status,
                "reason": reason,
            });
            extend_json_object(
                &mut surface_report,
                serde_json::json!({
                    "artifact_distribution_signing_notarization_surface_allowed": false,
                    "artifact_distribution_signing_notarization_surface_request_accepted": false,
                    "artifact_distribution_signing_notarization_surface_accepted": false,
                    "artifact_distribution_signing_notarization_surface_recorded": false,
                    "artifact_distribution_signing_notarization_surface_persisted": false,
                    "artifact_distribution_signing_notarization_surface_materialized": false,
                    "artifact_distribution_signing_notarization_surface_filesystem_written": false,
                    "artifact_distribution_signing_notarization_surface_delivered": false,
                    "artifact_distribution_signing_notarization_surface_exposed": false,
                    "artifact_distribution_signing_notarization_surface_executed": false,
                }),
            );
            extend_json_object(
                &mut surface_report,
                serde_json::json!({
                    "artifact_signing_executed": false,
                    "package_signing_executed": false,
                    "signature_manifest_written": false,
                    "signature_checksum_bound": false,
                    "notarization_submitted": false,
                    "notarization_ticket_recorded": false,
                    "stapling_executed": false,
                    "installer_signing_executed": false,
                    "provenance_attestation_published": false,
                    "sbom_manifest_published": false,
                    "release_asset_packaged": false,
                    "artifact_bundle_packaged": false,
                    "cdn_artifact_written": false,
                    "update_feed_artifact_written": false,
                    "package_registry_artifact_published": false,
                    "external_package_channel_published": false,
                    "telegram_package_channel_published": false,
                    "public_release_claimed": false,
                    "public_ga_claimed": false,
                }),
            );
            extend_json_object(
                &mut surface_report,
                serde_json::json!({
                    "acceptance_recorded": false,
                    "operator_approval_derived": false,
                    "release_publication_authority_derived": false,
                    "activation_authority_derived": false,
                    "activation_command_derived": false,
                    "live_execution_allowed": false,
                    "activation_performed": false,
                    "install_executed": false,
                    "service_restarted": false,
                    "launchd_mutated": false,
                    "active_binary_mutated": false,
                    "memory_store_write_performed": false,
                    "memory_store_mutated": false,
                    "live_kg_write_performed": false,
                    "provider_invoked": false,
                    "model_invoked": false,
                    "credential_read": false,
                    "secret_file_read": false,
                    "artifact_distribution_signing_notarization_surface_noop_confirmed": true,
                }),
            );
            if let Some(surface_object) = surface_report.as_object_mut() {
                for flag in requested_flags {
                    surface_object.insert(flag.to_string(), serde_json::json!(true));
                }
            }
            surface_report
        })
        .collect::<Vec<_>>();
    let artifact_distribution_signing_notarization_surface_count =
        artifact_distribution_signing_notarization_surfaces.len();
    let contract_hash = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-distribution-signing-notarization-surface-denial:native:source={source_report_sha256}:surfaces={artifact_distribution_signing_notarization_surface_count}:route_count={}:signing=0:notarization=0:authority=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-distribution-signing-notarization-surface-denial:no-signing-execution:no-notarization:no-stapling:no-provenance-publication:no-sbom-publication:no-release-asset-packaging:no-cdn-update-feed-write:no-live",
    );
    let denials = vec![
        "source_distribution_artifact_manifest_status_report_required",
        "artifact_distribution_signing_notarization_surface_request_acceptance_denied",
        "artifact_distribution_signing_notarization_surface_acceptance_denied",
        "artifact_distribution_signing_notarization_surface_recording_denied",
        "artifact_distribution_signing_notarization_surface_persistence_denied",
        "artifact_distribution_signing_notarization_surface_materialization_denied",
        "artifact_distribution_signing_notarization_surface_filesystem_write_denied",
        "artifact_distribution_signing_notarization_surface_delivery_denied",
        "artifact_distribution_signing_notarization_surface_exposure_denied",
        "artifact_signing_execution_denied",
        "package_signing_execution_denied",
        "signature_manifest_write_denied",
        "signature_checksum_binding_denied",
        "notarization_submission_denied",
        "notarization_ticket_recording_denied",
        "stapling_execution_denied",
        "installer_signing_execution_denied",
        "provenance_attestation_publication_denied",
        "sbom_manifest_publication_denied",
        "release_asset_packaging_denied",
        "artifact_bundle_packaging_denied",
        "cdn_artifact_write_denied",
        "update_feed_artifact_write_denied",
        "package_registry_artifact_publish_denied",
        "external_package_channel_publication_denied",
        "telegram_package_channel_publication_denied",
        "public_release_claim_from_signing_notarization_denied",
        "public_ga_claim_from_signing_notarization_denied",
        "acceptance_from_signing_notarization_denied",
        "operator_approval_from_signing_notarization_denied",
        "release_publication_authority_from_signing_notarization_denied",
        "activation_live_from_signing_notarization_denied",
        "install_restart_active_binary_from_signing_notarization_denied",
        "memory_provider_kg_from_signing_notarization_denied",
    ];
    let denied_count = denials.len();
    let report_ready = source_ready
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_surface_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_attempt_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_exposed_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_package_signing_status_exposed_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_notarization_status_exposed_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_stapling_status_exposed_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_provenance_attestation_status_exposed_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_sbom_manifest_status_exposed_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_release_publication_authority_derived_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_activation_authority_derived_count",
        ) == 0
        && artifact_distribution_signing_notarization_surface_count == 18
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DISTRIBUTION_SIGNING_NOTARIZATION_SURFACE_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-distribution-signing-notarization-surface-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-26",
        "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_denial_v1",
        "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_mode": "native_route_denied_distribution_artifact_manifest_status_cannot_execute_signing_notarization_stapling_provenance_sbom_packaging_channel_publication_authority_or_live_install",
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_gate": source_report["gate"].clone(),
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_ready": source_ready,
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_report_sha256": source_report_sha256,
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_contract_hash_sha256": source_contract_hash,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_contract_hash_sha256": contract_hash,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_policy_hash_sha256": policy_hash,
        "minimum_required_samples": 24,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "source_route_wired": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_denial_route_enabled": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_denial_ready": report_ready,
    });

    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_surface_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_surface_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_attempt_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_attempt_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_exposed_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_exposed_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_signing_status_exposed_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_package_signing_status_exposed_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_notarization_status_exposed_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_notarization_status_exposed_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_stapling_status_exposed_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_stapling_status_exposed_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_provenance_attestation_status_exposed_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_provenance_attestation_status_exposed_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_sbom_manifest_status_exposed_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_sbom_manifest_status_exposed_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_release_publication_authority_derived_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_release_publication_authority_derived_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_activation_authority_derived_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_activation_authority_derived_count"),
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_count": artifact_distribution_signing_notarization_surface_count,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_attempt_count": artifact_distribution_signing_notarization_surface_count,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_denied_count": artifact_distribution_signing_notarization_surface_count,
        }),
    );

    for key in [
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_allowed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_request_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_materialized_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_filesystem_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_delivered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_exposed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_signing_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_package_signing_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_signature_manifest_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_signature_checksum_bound_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_notarization_submitted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_notarization_ticket_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_stapling_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_installer_signing_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_provenance_attestation_published_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_sbom_manifest_published_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_release_asset_packaged_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_bundle_packaged_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_cdn_artifact_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_update_feed_artifact_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_package_registry_artifact_published_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_external_package_channel_published_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_package_channel_published_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_acceptance_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_operator_approval_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_activation_command_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_live_execution_allowed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_service_restarted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_active_binary_mutated_count",
    ] {
        if let Some(report_object) = report.as_object_mut() {
            report_object.insert(key.to_string(), serde_json::json!(0));
        }
    }

    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_surfaces": artifact_distribution_signing_notarization_surfaces,
            "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface": denials,
            "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_count": denied_count,
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_denial_gate",
                    "status": "allowed_report_only_next_slice",
                    "executes_signing": false,
                    "executes_notarization": false,
                    "executes_stapling": false,
                    "publishes_provenance": false,
                    "publishes_sbom": false,
                    "packages_release_asset": false,
                    "writes_cdn_artifact": false,
                    "writes_update_feed_artifact": false,
                    "publishes_external_package_channel": false,
                    "records_operator_acceptance": false,
                    "derives_release_publication_authority": false,
                    "derives_activation_authority": false,
                    "activates_live": false,
                    "mutates_memory_store": false,
                    "writes_kg": false,
                    "sends_externally": false
                }
            ],
        }),
    );

    let false_keys = [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_allowed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_request_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_materialized",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_filesystem_written",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_delivered",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_executed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_signing_executed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_signing_executed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_signature_manifest_written",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_signature_checksum_bound",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_notarization_submitted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_notarization_ticket_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_stapling_executed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_installer_signing_executed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_provenance_attestation_published",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_sbom_manifest_published",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_release_asset_packaged",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_bundle_packaged",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_cdn_artifact_written",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_update_feed_artifact_written",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_registry_artifact_published",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_external_package_channel_published",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_package_channel_published",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_persisted",
        "packet_acceptance_receipt_release_publication_recorded",
        "operator_acceptance_recorded",
        "operator_approval_recorded",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "activation_command_derived",
        "activation_allowed",
        "activation_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
        "public_release_claimed",
        "public_ga_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "external_send_performed",
        "filesystem_written",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in false_keys {
            report_object.insert(key.to_string(), serde_json::json!(false));
        }
    }

    let mut side_effects = serde_json::Map::new();
    for key in false_keys {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_report =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_denial_report();
    let source_ready = source_report["status"].as_str() == Some("ready")
        && source_report["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_denial_ready"]
            .as_bool()
            .unwrap_or(false);
    let source_u64 = |key: &str| source_report[key].as_u64().unwrap_or(0);
    let source_report_sha256 = sha256_json_value(&source_report);
    let source_contract_hash = source_report
        ["release_publication_result_receipt_terminal_public_claim_status_exposure_contract_hash_sha256"]
        .as_str()
        .unwrap_or("unknown")
        .to_string();
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let surface_specs: Vec<(&str, &str, &str, Vec<&str>)> = vec![
        (
            "delivery_receipt_package_index_status",
            "blocked_package_index_status_noop",
            "package_index_status_exposure_denied",
            vec!["package_index_status_requested"],
        ),
        (
            "delivery_receipt_package_registry_status",
            "blocked_package_registry_status_noop",
            "package_registry_status_exposure_denied",
            vec!["package_registry_status_requested"],
        ),
        (
            "delivery_receipt_package_metadata_endpoint_status",
            "blocked_package_metadata_endpoint_status_noop",
            "package_metadata_endpoint_status_exposure_denied",
            vec!["package_metadata_endpoint_status_requested"],
        ),
        (
            "delivery_receipt_update_feed_status",
            "blocked_update_feed_status_noop",
            "update_feed_status_exposure_denied",
            vec!["update_feed_status_requested"],
        ),
        (
            "delivery_receipt_cdn_mirror_status",
            "blocked_cdn_mirror_status_noop",
            "cdn_mirror_status_exposure_denied",
            vec!["cdn_mirror_status_requested"],
        ),
        (
            "delivery_receipt_release_channel_status",
            "blocked_release_channel_status_noop",
            "release_channel_status_exposure_denied",
            vec!["release_channel_status_requested"],
        ),
        (
            "delivery_receipt_distribution_artifact_status",
            "blocked_distribution_artifact_status_noop",
            "distribution_artifact_status_exposure_denied",
            vec!["distribution_artifact_status_requested"],
        ),
        (
            "delivery_receipt_artifact_catalog_status",
            "blocked_artifact_catalog_status_noop",
            "artifact_catalog_status_exposure_denied",
            vec!["artifact_catalog_status_requested"],
        ),
        (
            "delivery_receipt_version_manifest_status",
            "blocked_version_manifest_status_noop",
            "version_manifest_status_exposure_denied",
            vec!["version_manifest_status_requested"],
        ),
        (
            "delivery_receipt_installer_manifest_status",
            "blocked_installer_manifest_status_noop",
            "installer_manifest_status_exposure_denied",
            vec!["installer_manifest_status_requested"],
        ),
        (
            "delivery_receipt_checksum_manifest_status",
            "blocked_checksum_manifest_status_noop",
            "checksum_manifest_status_exposure_denied",
            vec!["checksum_manifest_status_requested"],
        ),
        (
            "delivery_receipt_download_page_status",
            "blocked_download_page_status_noop",
            "download_page_status_exposure_denied",
            vec!["download_page_status_requested"],
        ),
        (
            "delivery_receipt_release_notes_package_status",
            "blocked_release_notes_package_status_noop",
            "release_notes_package_status_exposure_denied",
            vec!["release_notes_package_status_requested"],
        ),
        (
            "delivery_receipt_channel_announcement_status",
            "blocked_channel_announcement_status_noop",
            "channel_announcement_status_exposure_denied",
            vec!["channel_announcement_status_requested"],
        ),
        (
            "delivery_receipt_channel_external_telegram_package_status",
            "blocked_channel_external_telegram_package_status_noop",
            "channel_external_telegram_package_status_denied",
            vec![
                "channel_status_requested",
                "external_status_requested",
                "telegram_status_requested",
            ],
        ),
        (
            "delivery_receipt_release_publication_authority_package_status",
            "blocked_release_publication_authority_package_status_noop",
            "release_publication_authority_from_package_status_denied",
            vec!["release_publication_authority_package_status_requested"],
        ),
        (
            "delivery_receipt_activation_live_package_status",
            "blocked_activation_live_package_status_noop",
            "activation_live_from_package_status_denied",
            vec!["activation_live_package_status_requested"],
        ),
        (
            "delivery_receipt_install_restart_active_binary_package_status",
            "blocked_active_binary_package_status_noop",
            "install_restart_active_binary_from_package_status_denied",
            vec!["install_restart_active_binary_package_status_requested"],
        ),
    ];

    let surfaces = surface_specs
        .into_iter()
        .map(|(surface, status, reason, requested_flags)| {
            let mut surface_report = serde_json::json!({
                "release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_surface": surface,
                "source_terminal_public_claim_status_exposure_ready": true,
                "package_release_channel_status_exposure_attempted": true,
                "package_release_channel_status_exposure_status": status,
                "reason": reason,
            });
            extend_json_object(
                &mut surface_report,
                serde_json::json!({
                    "package_release_channel_status_exposure_allowed": false,
                    "package_release_channel_status_exposure_request_accepted": false,
                    "package_release_channel_status_exposure_accepted": false,
                    "package_release_channel_status_exposure_recorded": false,
                    "package_release_channel_status_exposure_persisted": false,
                    "package_release_channel_status_exposure_materialized": false,
                    "package_release_channel_status_exposure_filesystem_written": false,
                    "package_release_channel_status_exposure_delivered": false,
                    "package_release_channel_status_exposed": false,
                    "package_index_status_exposed": false,
                    "package_registry_status_exposed": false,
                    "package_metadata_endpoint_status_exposed": false,
                    "update_feed_status_exposed": false,
                    "cdn_mirror_status_exposed": false,
                    "release_channel_status_exposed": false,
                    "distribution_artifact_status_exposed": false,
                    "artifact_catalog_status_exposed": false,
                }),
            );
            extend_json_object(
                &mut surface_report,
                serde_json::json!({
                    "version_manifest_status_exposed": false,
                    "installer_manifest_status_exposed": false,
                    "checksum_manifest_status_exposed": false,
                    "download_page_status_exposed": false,
                    "release_notes_package_status_exposed": false,
                    "channel_announcement_status_exposed": false,
                    "channel_status_delivered": false,
                    "external_status_sent": false,
                    "telegram_status_sent": false,
                    "public_release_claimed": false,
                    "public_ga_claimed": false,
                    "acceptance_recorded": false,
                    "operator_approval_derived": false,
                    "release_publication_authority_derived": false,
                    "activation_authority_derived": false,
                    "activation_command_derived": false,
                    "live_execution_allowed": false,
                    "activation_performed": false,
                }),
            );
            extend_json_object(
                &mut surface_report,
                serde_json::json!({
                    "install_executed": false,
                    "service_restarted": false,
                    "launchd_mutated": false,
                    "active_binary_mutated": false,
                    "memory_store_write_performed": false,
                    "memory_store_mutated": false,
                    "live_kg_write_performed": false,
                    "provider_invoked": false,
                    "model_invoked": false,
                    "credential_read": false,
                    "secret_file_read": false,
                    "package_release_channel_status_exposure_noop_confirmed": true,
                }),
            );
            if let Some(surface_object) = surface_report.as_object_mut() {
                for flag in requested_flags {
                    surface_object.insert(flag.to_string(), serde_json::json!(true));
                }
            }
            surface_report
        })
        .collect::<Vec<_>>();
    let surface_count = surfaces.len();
    let contract_hash = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-package-release-channel-status-exposure-denial:native:source={source_report_sha256}:surfaces={surface_count}:route_count={}:package=0:channel=0:authority=0:install=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "release-publication-result-receipt-terminal-distribution-delivery-receipt-package-release-channel-status-exposure-denial:no-package-index:no-update-feed:no-cdn:no-release-channel:no-distribution-artifact:no-live",
    );
    let denials = vec![
        "source_terminal_public_claim_status_exposure_report_required",
        "package_release_channel_status_request_acceptance_denied",
        "package_release_channel_status_acceptance_denied",
        "package_release_channel_status_recording_denied",
        "package_release_channel_status_persistence_denied",
        "package_release_channel_status_materialization_denied",
        "package_release_channel_status_filesystem_write_denied",
        "package_release_channel_status_delivery_denied",
        "package_release_channel_status_exposure_denied",
        "package_index_status_exposure_denied",
        "package_registry_status_exposure_denied",
        "package_metadata_endpoint_status_exposure_denied",
        "update_feed_status_exposure_denied",
        "cdn_mirror_status_exposure_denied",
        "release_channel_status_exposure_denied",
        "distribution_artifact_status_exposure_denied",
        "artifact_catalog_status_exposure_denied",
        "version_manifest_status_exposure_denied",
        "installer_manifest_status_exposure_denied",
        "checksum_manifest_status_exposure_denied",
        "download_page_status_exposure_denied",
        "release_notes_package_status_exposure_denied",
        "channel_announcement_status_exposure_denied",
        "channel_status_delivery_denied",
        "external_status_send_denied",
        "telegram_status_send_denied",
        "public_release_claim_from_package_status_denied",
        "public_ga_claim_from_package_status_denied",
        "acceptance_from_package_status_denied",
        "operator_approval_from_package_status_denied",
        "release_publication_authority_from_package_status_denied",
        "activation_live_from_package_status_denied",
        "install_restart_active_binary_from_package_status_denied",
        "memory_provider_kg_from_package_status_denied",
    ];
    let denied_count = denials.len();
    let report_ready = source_ready
        && source_u64(
            "release_publication_result_receipt_terminal_public_claim_status_exposure_surface_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_public_claim_status_exposure_attempt_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_public_claim_status_exposed_count",
        ) == 0
        && source_u64("release_publication_result_receipt_public_status_claimed_count") == 0
        && source_u64("release_publication_result_receipt_public_release_claimed_count") == 0
        && source_u64("release_publication_result_receipt_public_ga_claimed_count") == 0
        && source_u64(
            "release_publication_result_receipt_artifact_availability_status_exposed_count",
        ) == 0
        && source_u64("release_publication_result_receipt_distribution_queue_status_exposed_count")
            == 0
        && source_u64("release_publication_result_receipt_channel_status_delivered_count") == 0
        && source_u64("release_publication_result_receipt_external_status_sent_count") == 0
        && source_u64("release_publication_result_receipt_telegram_status_sent_count") == 0
        && source_u64(
            "release_publication_result_receipt_terminal_public_claim_status_exposure_release_publication_authority_derived_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_public_claim_status_exposure_activation_authority_derived_count",
        ) == 0
        && surface_count == 18
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_PACKAGE_RELEASE_CHANNEL_STATUS_EXPOSURE_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-package-release-channel-status-exposure-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-26",
        "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_denial_route_v1",
        "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_mode": "native_route_denied_public_status_cannot_be_exposed_as_package_release_channel_distribution_version_download_or_install_status",
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_gate": source_report["gate"].clone(),
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_ready": source_ready,
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_report_sha256": source_report_sha256,
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_contract_hash_sha256": source_contract_hash,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_contract_hash_sha256": contract_hash,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_policy_hash_sha256": policy_hash,
        "minimum_required_samples": 24,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "source_route_wired": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_denial_route_enabled": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_denial_ready": report_ready,
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_surface_count": source_u64("release_publication_result_receipt_terminal_public_claim_status_exposure_surface_count"),
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_attempt_count": source_u64("release_publication_result_receipt_terminal_public_claim_status_exposure_attempt_count"),
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposed_count": source_u64("release_publication_result_receipt_terminal_public_claim_status_exposed_count"),
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_public_status_claimed_count": source_u64("release_publication_result_receipt_public_status_claimed_count"),
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_public_release_claimed_count": source_u64("release_publication_result_receipt_public_release_claimed_count"),
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_public_ga_claimed_count": source_u64("release_publication_result_receipt_public_ga_claimed_count"),
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_availability_status_exposed_count": source_u64("release_publication_result_receipt_artifact_availability_status_exposed_count"),
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_queue_status_exposed_count": source_u64("release_publication_result_receipt_distribution_queue_status_exposed_count"),
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_channel_status_delivered_count": source_u64("release_publication_result_receipt_channel_status_delivered_count"),
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_external_status_sent_count": source_u64("release_publication_result_receipt_external_status_sent_count"),
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_status_sent_count": source_u64("release_publication_result_receipt_telegram_status_sent_count"),
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_release_publication_authority_derived_count": source_u64("release_publication_result_receipt_terminal_public_claim_status_exposure_release_publication_authority_derived_count"),
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_activation_authority_derived_count": source_u64("release_publication_result_receipt_terminal_public_claim_status_exposure_activation_authority_derived_count"),
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_surface_count": surface_count,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_attempt_count": surface_count,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_denied_count": surface_count,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_surfaces": surfaces,
        "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure": denials,
        "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_count": denied_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
        "allowed_next_actions": [
            {
                "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_denial_gate",
                "status": "allowed_report_only_next_slice",
                "exposes_package_channel_status": false,
                "writes_package_index": false,
                "writes_update_feed": false,
                "writes_cdn_mirror": false,
                "writes_release_channel": false,
                "records_operator_acceptance": false,
                "derives_release_publication_authority": false,
                "derives_activation_authority": false,
                "activates_live": false,
                "mutates_memory_store": false,
                "writes_kg": false,
                "sends_externally": false
            }
        ],
        }),
    );

    let zero_keys = [
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_allowed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_request_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_materialized_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_filesystem_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_delivered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_package_index_status_exposed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_package_registry_status_exposed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_package_metadata_endpoint_status_exposed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_update_feed_status_exposed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_cdn_mirror_status_exposed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_release_channel_status_exposed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_status_exposed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_catalog_status_exposed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_version_manifest_status_exposed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_installer_manifest_status_exposed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_checksum_manifest_status_exposed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_download_page_status_exposed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_release_notes_package_status_exposed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_channel_announcement_status_exposed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_delivered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_external_status_sent_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_telegram_status_sent_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_acceptance_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_operator_approval_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_activation_command_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_live_execution_allowed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_service_restarted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_active_binary_mutated_count",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in zero_keys {
            report_object.insert(key.to_string(), serde_json::json!(0));
        }
    }

    let false_keys = [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_public_badge_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_status_endpoint_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_availability_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_queue_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_allowed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_request_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_materialized",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_filesystem_written",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_delivered",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_index_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_registry_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_metadata_endpoint_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_update_feed_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_cdn_mirror_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_release_channel_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_catalog_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_version_manifest_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_installer_manifest_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_checksum_manifest_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_download_page_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_release_notes_package_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_channel_announcement_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_persisted",
        "packet_acceptance_receipt_release_publication_recorded",
        "operator_acceptance_recorded",
        "operator_approval_recorded",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "activation_command_derived",
        "activation_allowed",
        "activation_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
        "public_release_claimed",
        "public_ga_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "external_send_performed",
        "filesystem_written",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in false_keys {
            report_object.insert(key.to_string(), serde_json::json!(false));
        }
    }

    let mut side_effects = serde_json::Map::new();
    for key in false_keys {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_report =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_denial_report();
    let source_ready = source_report["status"].as_str() == Some("ready")
        && source_report["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_denial_ready"]
            .as_bool()
            .unwrap_or(false);
    let source_u64 = |key: &str| source_report[key].as_u64().unwrap_or(0);
    let source_report_sha256 = sha256_json_value(&source_report);
    let source_contract_hash = source_report
        ["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_contract_hash_sha256"]
        .as_str()
        .unwrap_or("unknown")
        .to_string();
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let surface_specs: Vec<(&str, &str, &str, Vec<&str>)> = vec![
        (
            "delivery_receipt_artifact_download_button",
            "blocked_artifact_download_button_noop",
            "artifact_download_button_denied",
            vec!["artifact_download_button_requested"],
        ),
        (
            "delivery_receipt_direct_download_url",
            "blocked_direct_download_url_noop",
            "direct_download_url_denied",
            vec!["direct_download_url_requested"],
        ),
        (
            "delivery_receipt_checksum_copy_prompt",
            "blocked_checksum_copy_prompt_noop",
            "checksum_copy_prompt_denied",
            vec!["checksum_copy_prompt_requested"],
        ),
        (
            "delivery_receipt_package_manager_install_command",
            "blocked_package_manager_install_command_noop",
            "package_manager_install_command_denied",
            vec!["package_manager_install_command_requested"],
        ),
        (
            "delivery_receipt_curl_pipe_shell_snippet",
            "blocked_curl_pipe_shell_snippet_noop",
            "curl_pipe_shell_snippet_denied",
            vec!["curl_pipe_shell_snippet_requested"],
        ),
        (
            "delivery_receipt_installer_launch_prompt",
            "blocked_installer_launch_prompt_noop",
            "installer_launch_prompt_denied",
            vec!["installer_launch_prompt_requested"],
        ),
        (
            "delivery_receipt_auto_update_offer",
            "blocked_auto_update_offer_noop",
            "auto_update_offer_denied",
            vec!["auto_update_offer_requested"],
        ),
        (
            "delivery_receipt_release_channel_subscribe_prompt",
            "blocked_release_channel_subscribe_prompt_noop",
            "release_channel_subscribe_prompt_denied",
            vec!["release_channel_subscribe_prompt_requested"],
        ),
        (
            "delivery_receipt_update_feed_consumer_hint",
            "blocked_update_feed_consumer_hint_noop",
            "update_feed_consumer_hint_denied",
            vec!["update_feed_consumer_hint_requested"],
        ),
        (
            "delivery_receipt_package_registry_install_badge",
            "blocked_package_registry_install_badge_noop",
            "package_registry_install_badge_denied",
            vec!["package_registry_install_badge_requested"],
        ),
        (
            "delivery_receipt_cdn_mirror_download_link",
            "blocked_cdn_mirror_download_link_noop",
            "cdn_mirror_download_link_denied",
            vec!["cdn_mirror_download_link_requested"],
        ),
        (
            "delivery_receipt_sbom_provenance_download_link",
            "blocked_sbom_provenance_download_link_noop",
            "sbom_provenance_download_link_denied",
            vec!["sbom_provenance_download_link_requested"],
        ),
        (
            "delivery_receipt_notarization_ticket_download_link",
            "blocked_notarization_ticket_download_link_noop",
            "notarization_ticket_download_link_denied",
            vec!["notarization_ticket_download_link_requested"],
        ),
        (
            "delivery_receipt_signature_verification_command",
            "blocked_signature_verification_command_noop",
            "signature_verification_command_denied",
            vec!["signature_verification_command_requested"],
        ),
        (
            "delivery_receipt_one_click_install_deep_link",
            "blocked_one_click_install_deep_link_noop",
            "one_click_install_deep_link_denied",
            vec!["one_click_install_deep_link_requested"],
        ),
        (
            "delivery_receipt_external_telegram_install_message",
            "blocked_external_telegram_install_message_noop",
            "external_telegram_install_message_denied",
            vec![
                "external_install_message_requested",
                "telegram_install_message_requested",
            ],
        ),
        (
            "delivery_receipt_release_publication_authority_install_affordance",
            "blocked_release_publication_authority_install_affordance_noop",
            "release_publication_authority_from_install_affordance_denied",
            vec!["release_publication_authority_install_affordance_requested"],
        ),
        (
            "delivery_receipt_activation_live_install_restart_active_binary_affordance",
            "blocked_activation_live_install_restart_active_binary_affordance_noop",
            "activation_live_install_restart_active_binary_from_install_affordance_denied",
            vec![
                "activation_live_install_affordance_requested",
                "install_restart_active_binary_affordance_requested",
            ],
        ),
    ];

    let artifact_download_install_affordance_surfaces = surface_specs
        .into_iter()
        .map(|(surface, status, reason, requested_flags)| {
            let mut surface_report = serde_json::json!({
                "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_surface": surface,
                "source_artifact_distribution_signing_notarization_surface_ready": true,
                "artifact_download_install_affordance_attempted": true,
                "artifact_download_install_affordance_status": status,
                "reason": reason,
            });
            extend_json_object(
                &mut surface_report,
                serde_json::json!({
                    "artifact_download_install_affordance_allowed": false,
                    "artifact_download_install_affordance_request_accepted": false,
                    "artifact_download_install_affordance_accepted": false,
                    "artifact_download_install_affordance_recorded": false,
                    "artifact_download_install_affordance_persisted": false,
                    "artifact_download_install_affordance_materialized": false,
                    "artifact_download_install_affordance_filesystem_written": false,
                    "artifact_download_install_affordance_delivered": false,
                    "artifact_download_install_affordance_exposed": false,
                    "artifact_download_install_affordance_published": false,
                    "artifact_download_install_affordance_executed": false,
                }),
            );
            extend_json_object(
                &mut surface_report,
                serde_json::json!({
                    "download_button_rendered": false,
                    "direct_download_url_exposed": false,
                    "checksum_prompt_rendered": false,
                    "package_manager_install_command_rendered": false,
                    "curl_pipe_shell_snippet_rendered": false,
                    "installer_launch_prompt_rendered": false,
                    "auto_update_offer_rendered": false,
                    "release_channel_subscribe_prompt_rendered": false,
                    "update_feed_consumer_hint_rendered": false,
                    "package_registry_install_badge_rendered": false,
                    "cdn_mirror_download_link_rendered": false,
                    "sbom_provenance_download_link_rendered": false,
                    "notarization_ticket_download_link_rendered": false,
                    "signature_verification_command_rendered": false,
                    "one_click_install_deep_link_rendered": false,
                    "external_install_message_sent": false,
                    "telegram_install_message_sent": false,
                }),
            );
            extend_json_object(
                &mut surface_report,
                serde_json::json!({
                    "acceptance_recorded": false,
                    "operator_approval_derived": false,
                    "release_publication_authority_derived": false,
                    "activation_authority_derived": false,
                    "activation_command_derived": false,
                    "live_execution_allowed": false,
                    "activation_performed": false,
                    "install_executed": false,
                    "service_restarted": false,
                    "launchd_mutated": false,
                    "active_binary_mutated": false,
                    "release_artifact_written": false,
                    "public_artifact_written": false,
                    "memory_store_write_performed": false,
                    "memory_store_mutated": false,
                    "live_kg_write_performed": false,
                }),
            );
            extend_json_object(
                &mut surface_report,
                serde_json::json!({
                    "provider_invoked": false,
                    "model_invoked": false,
                    "credential_read": false,
                    "secret_file_read": false,
                    "external_send_performed": false,
                    "artifact_download_install_affordance_noop_confirmed": true,
                }),
            );
            if let Some(surface_object) = surface_report.as_object_mut() {
                for flag in requested_flags {
                    surface_object.insert(flag.to_string(), serde_json::json!(true));
                }
            }
            surface_report
        })
        .collect::<Vec<_>>();
    let artifact_download_install_affordance_surface_count =
        artifact_download_install_affordance_surfaces.len();
    let contract_hash = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-denial:native:source={source_report_sha256}:surfaces={artifact_download_install_affordance_surface_count}:route_count={}:download=0:install=0:authority=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-denial:no-download-link:no-install-command:no-installer-prompt:no-update-offer:no-external-install-message:no-live",
    );
    let denials = vec![
        "source_artifact_distribution_signing_notarization_surface_report_required",
        "artifact_download_install_affordance_request_acceptance_denied",
        "artifact_download_install_affordance_acceptance_denied",
        "artifact_download_install_affordance_recording_denied",
        "artifact_download_install_affordance_persistence_denied",
        "artifact_download_install_affordance_materialization_denied",
        "artifact_download_install_affordance_filesystem_write_denied",
        "artifact_download_install_affordance_delivery_denied",
        "artifact_download_install_affordance_exposure_denied",
        "artifact_download_install_affordance_publication_denied",
        "artifact_download_install_affordance_execution_denied",
        "download_button_rendering_denied",
        "direct_download_url_exposure_denied",
        "package_manager_install_command_denied",
        "curl_pipe_shell_snippet_denied",
        "installer_launch_prompt_denied",
        "auto_update_offer_denied",
        "release_channel_subscribe_prompt_denied",
        "update_feed_consumer_hint_denied",
        "package_registry_install_badge_denied",
        "cdn_mirror_download_link_denied",
        "sbom_provenance_download_link_denied",
        "notarization_ticket_download_link_denied",
        "signature_verification_command_denied",
        "one_click_install_deep_link_denied",
        "external_install_message_denied",
        "telegram_install_message_denied",
        "operator_approval_from_install_affordance_denied",
        "release_publication_authority_from_install_affordance_denied",
        "activation_live_from_install_affordance_denied",
        "install_restart_active_binary_from_install_affordance_denied",
        "memory_provider_kg_from_install_affordance_denied",
    ];
    let denied_count = denials.len();
    let report_ready = source_ready
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_attempt_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_accepted_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_recorded_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_persisted_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_exposed_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_executed_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_signing_executed_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_notarization_submitted_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_stapling_executed_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_provenance_attestation_published_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_sbom_manifest_published_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_release_publication_authority_derived_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_activation_authority_derived_count",
        ) == 0
        && artifact_download_install_affordance_surface_count == 18
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-18",
        "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_denial_v1",
        "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_mode": "native_route_denied_artifact_distribution_signing_notarization_surface_cannot_become_download_link_install_command_installer_prompt_update_offer_external_install_message_or_live_authority",
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_gate": source_report["gate"].clone(),
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_ready": source_ready,
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_report_sha256": source_report_sha256,
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_contract_hash_sha256": source_contract_hash,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_contract_hash_sha256": contract_hash,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_policy_hash_sha256": policy_hash,
        "minimum_required_samples": 24,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "source_route_wired": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_denial_route_enabled": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_denial_ready": report_ready,
    });

    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_attempt_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_attempt_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_denied_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_denied_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_accepted_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_accepted_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_recorded_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_recorded_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_persisted_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_persisted_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_exposed_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_exposed_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_executed_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_executed_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_signing_executed_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_signing_executed_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_notarization_submitted_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_notarization_submitted_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_stapling_executed_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_stapling_executed_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_provenance_attestation_published_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_provenance_attestation_published_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_sbom_manifest_published_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_sbom_manifest_published_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_release_publication_authority_derived_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_release_publication_authority_derived_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_activation_authority_derived_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_activation_authority_derived_count"),
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_surface_count": artifact_download_install_affordance_surface_count,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_count": artifact_download_install_affordance_surface_count,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_attempt_count": artifact_download_install_affordance_surface_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_allowed_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_request_accepted_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_accepted_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_recorded_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_persisted_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_materialized_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_filesystem_written_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_delivered_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_exposed_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_published_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_executed_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_download_button_rendered_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_direct_download_url_exposed_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_checksum_prompt_rendered_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_package_manager_install_command_rendered_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_curl_pipe_shell_snippet_rendered_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_installer_launch_prompt_rendered_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_auto_update_offer_rendered_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_release_channel_subscribe_prompt_rendered_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_update_feed_consumer_hint_rendered_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_package_registry_install_badge_rendered_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_cdn_mirror_download_link_rendered_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_sbom_provenance_download_link_rendered_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_notarization_ticket_download_link_rendered_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_signature_verification_command_rendered_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_one_click_install_deep_link_rendered_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_external_install_message_sent_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_install_message_sent_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_acceptance_recorded_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_approval_derived_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_release_publication_authority_derived_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_activation_authority_derived_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_activation_command_derived_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_live_execution_allowed_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_install_executed_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_service_restarted_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_active_binary_mutated_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_release_artifact_written_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_public_artifact_written_count": 0,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_surfaces": artifact_download_install_affordance_surfaces,
            "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance": denials,
            "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_count": denied_count,
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_no_persistence_gate",
                    "status": "allowed_report_only_next_slice",
                    "renders_download_link": false,
                    "emits_install_command": false,
                    "prompts_installer": false,
                    "publishes_update_offer": false,
                    "sends_external_install_message": false,
                    "records_operator_acceptance": false,
                    "derives_release_publication_authority": false,
                    "derives_activation_authority": false,
                    "activates_live": false,
                    "installs_or_restarts": false,
                    "mutates_active_binary": false,
                    "mutates_memory_store": false,
                    "writes_kg": false,
                    "sends_externally": false
                }
            ],
        }),
    );

    let artifact_download_install_affordance_false_keys = [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_executed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_signing_executed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_notarization_submitted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_stapling_executed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_provenance_attestation_published",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_sbom_manifest_published",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_allowed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_request_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_materialized",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_filesystem_written",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_delivered",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_published",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_executed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_download_button_rendered",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_direct_download_url_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_checksum_prompt_rendered",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_manager_install_command_rendered",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_curl_pipe_shell_snippet_rendered",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_installer_launch_prompt_rendered",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_auto_update_offer_rendered",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_release_channel_subscribe_prompt_rendered",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_update_feed_consumer_hint_rendered",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_registry_install_badge_rendered",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_cdn_mirror_download_link_rendered",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_sbom_provenance_download_link_rendered",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_notarization_ticket_download_link_rendered",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_signature_verification_command_rendered",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_one_click_install_deep_link_rendered",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_external_install_message_sent",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_install_message_sent",
        "packet_acceptance_receipt_release_publication_result_receipt_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_persisted",
        "packet_acceptance_receipt_release_publication_recorded",
        "operator_acceptance_recorded",
        "operator_approval_recorded",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "activation_command_derived",
        "activation_allowed",
        "activation_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
        "public_release_claimed",
        "public_ga_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "external_send_performed",
        "filesystem_written",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in artifact_download_install_affordance_false_keys {
            report_object.insert(key.to_string(), serde_json::json!(false));
        }
    }

    let mut side_effects = serde_json::Map::new();
    for key in artifact_download_install_affordance_false_keys {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_no_persistence_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_report =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_denial_report();
    let source_ready = source_report["status"].as_str() == Some("ready")
        && source_report["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_denial_ready"]
            .as_bool()
            .unwrap_or(false);
    let source_u64 = |key: &str| source_report[key].as_u64().unwrap_or(0);
    let source_report_sha256 = sha256_json_value(&source_report);
    let source_contract_hash = source_report
        ["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_contract_hash_sha256"]
        .as_str()
        .unwrap_or("unknown")
        .to_string();
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let surface_specs: Vec<(&str, &str, &str, Vec<&str>)> = vec![
        (
            "source_artifact_download_install_affordance_report_required",
            "blocked_source_report_required_noop",
            "source_artifact_download_install_affordance_report_required",
            vec!["source_artifact_download_install_affordance_report_required"],
        ),
        (
            "download_button_result_receipt_recording",
            "blocked_download_button_result_receipt_recording_noop",
            "download_button_result_receipt_recording_denied",
            vec!["download_button_result_receipt_record_requested"],
        ),
        (
            "direct_download_url_result_receipt_persistence",
            "blocked_direct_download_url_result_receipt_persistence_noop",
            "direct_download_url_result_receipt_persistence_denied",
            vec!["direct_download_url_result_receipt_persist_requested"],
        ),
        (
            "checksum_prompt_result_receipt_materialization",
            "blocked_checksum_prompt_result_receipt_materialization_noop",
            "checksum_prompt_result_receipt_materialization_denied",
            vec!["checksum_prompt_result_receipt_materialize_requested"],
        ),
        (
            "package_manager_install_command_result_receipt",
            "blocked_package_manager_install_command_result_receipt_noop",
            "package_manager_install_command_result_receipt_denied",
            vec!["package_manager_install_command_result_receipt_requested"],
        ),
        (
            "curl_pipe_shell_result_receipt",
            "blocked_curl_pipe_shell_result_receipt_noop",
            "curl_pipe_shell_result_receipt_denied",
            vec!["curl_pipe_shell_result_receipt_requested"],
        ),
        (
            "installer_launch_prompt_result_receipt",
            "blocked_installer_launch_prompt_result_receipt_noop",
            "installer_launch_prompt_result_receipt_denied",
            vec!["installer_launch_prompt_result_receipt_requested"],
        ),
        (
            "auto_update_offer_result_receipt",
            "blocked_auto_update_offer_result_receipt_noop",
            "auto_update_offer_result_receipt_denied",
            vec!["auto_update_offer_result_receipt_requested"],
        ),
        (
            "release_channel_subscription_result_receipt",
            "blocked_release_channel_subscription_result_receipt_noop",
            "release_channel_subscription_result_receipt_denied",
            vec!["release_channel_subscription_result_receipt_requested"],
        ),
        (
            "update_feed_hint_result_receipt",
            "blocked_update_feed_hint_result_receipt_noop",
            "update_feed_hint_result_receipt_denied",
            vec!["update_feed_hint_result_receipt_requested"],
        ),
        (
            "package_registry_badge_result_receipt",
            "blocked_package_registry_badge_result_receipt_noop",
            "package_registry_badge_result_receipt_denied",
            vec!["package_registry_badge_result_receipt_requested"],
        ),
        (
            "cdn_mirror_download_result_receipt",
            "blocked_cdn_mirror_download_result_receipt_noop",
            "cdn_mirror_download_result_receipt_denied",
            vec!["cdn_mirror_download_result_receipt_requested"],
        ),
        (
            "sbom_provenance_notarization_result_receipt",
            "blocked_sbom_provenance_notarization_result_receipt_noop",
            "sbom_provenance_notarization_result_receipt_denied",
            vec!["sbom_provenance_notarization_result_receipt_requested"],
        ),
        (
            "signature_verification_command_result_receipt",
            "blocked_signature_verification_command_result_receipt_noop",
            "signature_verification_command_result_receipt_denied",
            vec!["signature_verification_command_result_receipt_requested"],
        ),
        (
            "one_click_install_deep_link_result_receipt",
            "blocked_one_click_install_deep_link_result_receipt_noop",
            "one_click_install_deep_link_result_receipt_denied",
            vec!["one_click_install_deep_link_result_receipt_requested"],
        ),
        (
            "external_telegram_install_message_result_receipt",
            "blocked_external_telegram_install_message_result_receipt_noop",
            "external_telegram_install_message_result_receipt_denied",
            vec![
                "external_install_message_result_receipt_requested",
                "telegram_install_message_result_receipt_requested",
            ],
        ),
        (
            "release_publication_authority_install_affordance_result_receipt",
            "blocked_release_publication_authority_install_affordance_result_receipt_noop",
            "release_publication_authority_install_affordance_result_receipt_denied",
            vec!["release_publication_authority_install_affordance_result_receipt_requested"],
        ),
        (
            "activation_live_install_restart_active_binary_result_receipt",
            "blocked_activation_live_install_restart_active_binary_result_receipt_noop",
            "activation_live_install_restart_active_binary_result_receipt_denied",
            vec![
                "activation_live_install_result_receipt_requested",
                "install_restart_active_binary_result_receipt_requested",
            ],
        ),
    ];

    let result_receipt_surfaces = surface_specs
        .into_iter()
        .map(|(surface, status, reason, requested_flags)| {
            let mut surface_report = serde_json::json!({
                "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_surface": surface,
                "source_artifact_download_install_affordance_denial_ready": true,
                "source_artifact_download_install_affordance_noop_confirmed": true,
                "artifact_download_install_affordance_result_receipt_attempted": true,
                "artifact_download_install_affordance_result_receipt_status": status,
                "reason": reason,
            });
            extend_json_object(
                &mut surface_report,
                serde_json::json!({
                    "artifact_download_install_affordance_result_receipt_allowed": false,
                    "artifact_download_install_affordance_result_receipt_schema_accepted": false,
                    "artifact_download_install_affordance_result_receipt_accepted": false,
                    "artifact_download_install_affordance_result_receipt_recorded": false,
                    "artifact_download_install_affordance_result_receipt_persisted": false,
                    "artifact_download_install_affordance_result_receipt_materialized": false,
                    "artifact_download_install_affordance_result_receipt_filesystem_written": false,
                    "artifact_download_install_affordance_result_receipt_ledger_written": false,
                    "artifact_download_install_affordance_result_receipt_indexed": false,
                    "artifact_download_install_affordance_result_receipt_enqueued": false,
                    "artifact_download_install_affordance_result_receipt_delivered": false,
                    "artifact_download_install_affordance_result_receipt_exported": false,
                    "artifact_download_install_affordance_result_receipt_query_registered": false,
                    "artifact_download_install_affordance_result_receipt_observability_recorded": false,
                    "artifact_download_install_affordance_result_receipt_hash_bound": false,
                    "artifact_download_install_affordance_result_receipt_status_accepted": false,
                }),
            );
            extend_json_object(
                &mut surface_report,
                serde_json::json!({
                    "artifact_download_install_affordance_completion_ack_recorded": false,
                    "artifact_download_install_affordance_completion_ack_persisted": false,
                    "artifact_download_install_affordance_completion_ack_accepted": false,
                    "artifact_download_install_affordance_completion_ack_materialized": false,
                    "artifact_download_install_affordance_completion_ack_delivered": false,
                    "download_button_rendered": false,
                    "direct_download_url_exposed": false,
                    "package_manager_install_command_rendered": false,
                    "curl_pipe_shell_snippet_rendered": false,
                    "installer_launch_prompt_rendered": false,
                    "auto_update_offer_rendered": false,
                    "external_install_message_sent": false,
                    "telegram_install_message_sent": false,
                }),
            );
            extend_json_object(
                &mut surface_report,
                serde_json::json!({
                    "acceptance_recorded": false,
                    "operator_approval_from_receipt_accepted": false,
                    "release_publication_authority_from_receipt_derived": false,
                    "activation_authority_from_receipt_derived": false,
                    "activation_command_from_receipt_derived": false,
                    "live_execution_from_receipt_allowed": false,
                    "activation_from_receipt_performed": false,
                    "install_from_receipt_executed": false,
                    "service_restart_from_receipt_performed": false,
                    "launchd_from_receipt_mutated": false,
                    "active_binary_from_receipt_mutated": false,
                    "memory_store_write_performed": false,
                    "memory_store_mutated": false,
                    "live_kg_write_performed": false,
                    "provider_invoked": false,
                    "model_invoked": false,
                    "credential_read": false,
                    "secret_file_read": false,
                    "receipt_noop_confirmed": true,
                }),
            );
            if let Some(surface_object) = surface_report.as_object_mut() {
                for flag in requested_flags {
                    surface_object.insert(flag.to_string(), serde_json::json!(true));
                }
            }
            surface_report
        })
        .collect::<Vec<_>>();
    let result_receipt_surface_count = result_receipt_surfaces.len();
    let contract_hash = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-no-persistence-denial:native:source={source_report_sha256}:surfaces={result_receipt_surface_count}:route_count={}:persist=0:ledger=0:authority=0:install=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-no-persistence-denial:no-receipt-record:no-receipt-persist:no-ledger:no-index:no-export:no-query:no-observability:no-idempotency:no-install:no-live",
    );
    let denials = vec![
        "source_artifact_download_install_affordance_report_required",
        "download_button_result_receipt_recording_denied",
        "direct_download_url_result_receipt_persistence_denied",
        "checksum_prompt_result_receipt_materialization_denied",
        "package_manager_install_command_result_receipt_denied",
        "curl_pipe_shell_result_receipt_denied",
        "installer_launch_prompt_result_receipt_denied",
        "auto_update_offer_result_receipt_denied",
        "release_channel_subscription_result_receipt_denied",
        "update_feed_hint_result_receipt_denied",
        "package_registry_badge_result_receipt_denied",
        "cdn_mirror_download_result_receipt_denied",
        "sbom_provenance_notarization_result_receipt_denied",
        "signature_verification_command_result_receipt_denied",
        "one_click_install_deep_link_result_receipt_denied",
        "external_telegram_install_message_result_receipt_denied",
        "release_publication_authority_install_affordance_result_receipt_denied",
        "activation_live_install_restart_active_binary_result_receipt_denied",
        "result_receipt_schema_acceptance_denied",
        "result_receipt_recording_denied",
        "result_receipt_persistence_denied",
        "result_receipt_ledger_index_queue_delivery_denied",
        "result_receipt_export_query_observability_denied",
        "result_receipt_hash_status_identity_binding_denied",
        "completion_ack_from_result_receipt_denied",
        "operator_approval_from_result_receipt_denied",
        "release_publication_authority_from_result_receipt_denied",
        "activation_authority_from_result_receipt_denied",
        "install_restart_active_binary_from_result_receipt_denied",
        "memory_provider_kg_from_result_receipt_denied",
    ];
    let denied_count = denials.len();
    let report_ready = source_ready
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_attempt_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_allowed_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_recorded_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_persisted_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_executed_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_download_button_rendered_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_direct_download_url_exposed_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_package_manager_install_command_rendered_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_external_install_message_sent_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_release_publication_authority_derived_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_activation_authority_derived_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_live_execution_allowed_count",
        ) == 0
        && result_receipt_surface_count == 18
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_no_persistence_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_NO_PERSISTENCE_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-no-persistence-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-18",
        "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_no_persistence_denial_v1",
        "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_mode": "native_route_denied_download_install_affordance_cannot_emit_or_persist_a_result_receipt_or_install_activation_evidence",
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_denial_route",
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_ready": source_ready,
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_report_sha256": source_report_sha256,
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_contract_hash_sha256": source_contract_hash,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_contract_hash_sha256": contract_hash,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_policy_hash_sha256": policy_hash,
        "minimum_required_samples": 24,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "source_route_wired": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_no_persistence_denial_route_enabled": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_no_persistence_denial_ready": report_ready,
    });

    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_attempt_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_attempt_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_allowed_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_allowed_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_recorded_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_recorded_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_persisted_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_persisted_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_executed_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_executed_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_download_button_rendered_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_download_button_rendered_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_direct_download_url_exposed_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_direct_download_url_exposed_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_manager_install_command_rendered_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_package_manager_install_command_rendered_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_external_install_message_sent_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_external_install_message_sent_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_release_publication_authority_derived_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_release_publication_authority_derived_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_activation_authority_derived_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_activation_authority_derived_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_live_execution_allowed_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_live_execution_allowed_count"),
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_surface_count": result_receipt_surface_count,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_attempt_count": result_receipt_surface_count,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_surfaces": result_receipt_surfaces,
            "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt": denials,
            "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_count": denied_count,
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_denial_gate",
                    "status": "allowed_report_only_next_slice",
                    "records_result_receipt": false,
                    "persists_result_receipt": false,
                    "records_idempotency": false,
                    "accepts_duplicate_receipt": false,
                    "renders_download_link": false,
                    "emits_install_command": false,
                    "prompts_installer": false,
                    "publishes_update_offer": false,
                    "sends_external_install_message": false,
                    "records_operator_acceptance": false,
                    "derives_release_publication_authority": false,
                    "derives_activation_authority": false,
                    "activates_live": false,
                    "installs_or_restarts": false,
                    "mutates_active_binary": false,
                    "mutates_memory_store": false,
                    "writes_kg": false,
                    "sends_externally": false
                }
            ],
        }),
    );

    let result_receipt_zero_keys = [
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_allowed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_schema_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_materialized_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_filesystem_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ledger_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_indexed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_enqueued_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_delivered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_exported_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_query_registered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_observability_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_hash_bound_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_status_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_completion_ack_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_completion_ack_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_completion_ack_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_completion_ack_materialized_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_completion_ack_delivered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_acceptance_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_approval_from_receipt_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_release_publication_authority_from_receipt_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_activation_authority_from_receipt_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_activation_command_from_receipt_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_live_execution_from_receipt_allowed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_install_from_receipt_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_service_restart_from_receipt_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_active_binary_from_receipt_mutated_count",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in result_receipt_zero_keys {
            report_object.insert(key.to_string(), serde_json::json!(0));
        }
    }

    let result_receipt_false_keys = [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_allowed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_schema_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_materialized",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_filesystem_written",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ledger_written",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_indexed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_enqueued",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_delivered",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_exported",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_query_registered",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_observability_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_hash_bound",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_status_accepted",
        "artifact_download_install_affordance_completion_ack_recorded",
        "artifact_download_install_affordance_completion_ack_persisted",
        "artifact_download_install_affordance_completion_ack_accepted",
        "artifact_download_install_affordance_completion_ack_materialized",
        "artifact_download_install_affordance_completion_ack_delivered",
        "operator_acceptance_recorded",
        "operator_approval_recorded",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "activation_command_derived",
        "activation_allowed",
        "activation_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
        "public_release_claimed",
        "public_ga_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "external_send_performed",
        "filesystem_written",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in result_receipt_false_keys {
            report_object.insert(key.to_string(), serde_json::json!(false));
        }
    }

    let mut side_effects = serde_json::Map::new();
    for key in result_receipt_false_keys {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_report =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_no_persistence_denial_report();
    let source_ready = source_report["status"].as_str() == Some("ready")
        && source_report["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_no_persistence_denial_ready"]
            .as_bool()
            .unwrap_or(false);
    let source_u64 = |key: &str| source_report[key].as_u64().unwrap_or(0);
    let source_report_sha256 = sha256_json_value(&source_report);
    let source_contract_hash = source_report
        ["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_contract_hash_sha256"]
        .as_str()
        .unwrap_or("unknown")
        .to_string();
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let replay_surface_names = [
        "source_result_receipt_no_persistence_report_required",
        "download_button_result_receipt_duplicate_identity_replay",
        "direct_download_url_result_receipt_replay_acceptance",
        "checksum_prompt_result_receipt_idempotency_key",
        "package_manager_install_command_result_receipt_idempotency_state",
        "curl_pipe_shell_result_receipt_cross_scope_reuse",
        "installer_launch_prompt_result_receipt_stale_nonce",
        "auto_update_offer_result_receipt_out_of_order_replay",
        "release_channel_subscription_result_receipt_completion_ack_replay",
        "update_feed_hint_result_receipt_ledger_index_delivery_replay",
        "package_registry_badge_result_receipt_export_query_observability_replay",
        "cdn_mirror_download_result_receipt_hash_status_rebind",
        "sbom_provenance_notarization_result_receipt_signature_timestamp_replay",
        "signature_verification_command_result_receipt_operator_identity_reuse",
        "one_click_install_deep_link_result_receipt_activation_authority_replay",
        "external_telegram_install_message_result_receipt_external_delivery_replay",
        "release_publication_authority_install_affordance_result_receipt_replay",
        "activation_live_install_restart_active_binary_result_receipt_replay",
    ];
    let replay_surface_false_keys = [
        "artifact_download_install_affordance_result_receipt_replay_allowed",
        "artifact_download_install_affordance_result_receipt_replay_recorded",
        "artifact_download_install_affordance_result_receipt_replay_persisted",
        "artifact_download_install_affordance_result_receipt_replay_performed",
        "artifact_download_install_affordance_result_receipt_duplicate_accepted",
        "artifact_download_install_affordance_result_receipt_duplicate_recorded",
        "artifact_download_install_affordance_result_receipt_duplicate_persisted",
        "artifact_download_install_affordance_result_receipt_idempotency_key_accepted",
        "artifact_download_install_affordance_result_receipt_idempotency_key_recorded",
        "artifact_download_install_affordance_result_receipt_idempotency_state_recorded",
        "artifact_download_install_affordance_result_receipt_idempotency_state_persisted",
        "artifact_download_install_affordance_result_receipt_idempotency_state_materialized",
        "artifact_download_install_affordance_result_receipt_idempotency_filesystem_written",
        "artifact_download_install_affordance_result_receipt_replay_nonce_accepted",
        "artifact_download_install_affordance_result_receipt_replay_nonce_recorded",
        "artifact_download_install_affordance_result_receipt_cross_scope_reuse_accepted",
        "artifact_download_install_affordance_result_receipt_status_upgrade_accepted",
        "artifact_download_install_affordance_result_receipt_completed_status_accepted",
        "artifact_download_install_affordance_result_receipt_ack_replay_accepted",
        "artifact_download_install_affordance_result_receipt_ledger_replay_accepted",
        "artifact_download_install_affordance_result_receipt_index_replay_accepted",
        "artifact_download_install_affordance_result_receipt_delivery_replay_accepted",
        "artifact_download_install_affordance_result_receipt_query_replay_accepted",
        "artifact_download_install_affordance_result_receipt_observability_replay_accepted",
        "artifact_download_install_affordance_result_receipt_hash_rebind_accepted",
        "artifact_download_install_affordance_result_receipt_status_rebind_accepted",
        "artifact_download_install_affordance_result_receipt_signature_timestamp_replay_accepted",
        "artifact_download_install_affordance_result_receipt_operator_identity_reuse_accepted",
        "artifact_download_install_affordance_result_receipt_recorded",
        "artifact_download_install_affordance_result_receipt_persisted",
        "artifact_download_install_affordance_result_receipt_accepted",
        "artifact_download_install_affordance_result_receipt_materialized",
        "artifact_download_install_affordance_result_receipt_filesystem_written",
        "artifact_download_install_affordance_result_receipt_ledger_written",
        "artifact_download_install_affordance_result_receipt_indexed",
        "artifact_download_install_affordance_result_receipt_enqueued",
        "artifact_download_install_affordance_result_receipt_delivered",
        "artifact_download_install_affordance_result_receipt_exported",
        "artifact_download_install_affordance_result_receipt_query_registered",
        "artifact_download_install_affordance_result_receipt_observability_recorded",
        "artifact_download_install_affordance_completion_ack_recorded",
        "artifact_download_install_affordance_completion_ack_persisted",
        "artifact_download_install_affordance_completion_ack_accepted",
        "artifact_download_install_affordance_completion_ack_materialized",
        "artifact_download_install_affordance_completion_ack_delivered",
        "download_button_rendered",
        "direct_download_url_exposed",
        "package_manager_install_command_rendered",
        "curl_pipe_shell_snippet_rendered",
        "installer_launch_prompt_rendered",
        "auto_update_offer_rendered",
        "external_install_message_sent",
        "telegram_install_message_sent",
        "operator_approval_from_replay_accepted",
        "release_publication_authority_from_replay_derived",
        "activation_authority_from_replay_derived",
        "activation_command_from_replay_derived",
        "live_execution_from_replay_allowed",
        "install_from_replay_executed",
        "service_restart_from_replay_performed",
        "launchd_from_replay_mutated",
        "active_binary_from_replay_mutated",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
    ];
    let replay_surfaces = replay_surface_names
        .iter()
        .map(|surface| {
            let mut surface_report = serde_json::json!({
                "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_surface": surface,
                "source_result_receipt_no_persistence_present": true,
                "source_result_receipt_no_persistence_ready": true,
                "source_result_receipt_no_persistence_noop_confirmed": true,
                "canonical_noop_result_receipt_identity_required": true,
                "artifact_download_install_affordance_result_receipt_replay_requested": true,
                "artifact_download_install_affordance_result_receipt_replay_status": format!("blocked_{surface}_noop"),
                "reason": "artifact_download_install_affordance_result_receipt_replay_idempotency_denied",
                "receipt_noop_confirmed": true,
            });
            if let Some(surface_object) = surface_report.as_object_mut() {
                for key in replay_surface_false_keys {
                    surface_object.insert(key.to_string(), serde_json::json!(false));
                }
            }
            surface_report
        })
        .collect::<Vec<_>>();
    let replay_surface_count = replay_surfaces.len();
    let contract_hash = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-replay-idempotency-denial:native:source={source_report_sha256}:surfaces={replay_surface_count}:route_count={}:replay=0:idempotency=0:authority=0:install=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-replay-idempotency-denial:no-duplicate:no-replay:no-idempotency-record:no-idempotency-persist:no-ledger:no-index:no-delivery:no-query:no-observability:no-authority:no-install:no-live",
    );
    let denials = vec![
        "source_result_receipt_no_persistence_report_required",
        "duplicate_result_receipt_identity_replay_denied",
        "result_receipt_replay_acceptance_denied",
        "idempotency_key_recording_denied",
        "idempotency_state_record_persist_materialize_denied",
        "idempotency_filesystem_write_denied",
        "cross_scope_result_receipt_reuse_denied",
        "stale_nonce_result_receipt_replay_denied",
        "out_of_order_result_receipt_replay_denied",
        "completion_ack_replay_denied",
        "ledger_index_delivery_replay_denied",
        "export_query_observability_replay_denied",
        "hash_status_rebind_denied",
        "signature_timestamp_replay_denied",
        "operator_identity_reuse_denied",
        "release_publication_authority_replay_denied",
        "activation_authority_replay_denied",
        "external_delivery_replay_denied",
        "install_restart_active_binary_replay_denied",
        "memory_provider_kg_replay_denied",
    ];
    let denied_count = denials.len();
    let report_ready = source_ready
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_surface_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_attempt_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_recorded_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_persisted_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_materialized_count",
        ) == 0
        && replay_surface_count == 18
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_REPLAY_IDEMPOTENCY_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-replay-idempotency-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-18",
        "minimum_required_samples": 24,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "source_route_wired": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_denial_route_enabled": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_denial_ready": report_ready,
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
        "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_denial_v1",
        "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_mode": "native_route_denied_duplicate_replay_and_idempotency_state_for_download_install_affordance_result_receipts_no_record_no_persist_no_authority_no_live",
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_no_persistence_route": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_no_persistence_denial_route",
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_no_persistence_ready": source_ready,
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_no_persistence_report_sha256": source_report_sha256,
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_contract_hash_sha256": source_contract_hash,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_contract_hash_sha256": contract_hash,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_policy_hash_sha256": policy_hash,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_surface_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_surface_count"),
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_attempt_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_attempt_count"),
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_recorded_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_recorded_count"),
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_persisted_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_persisted_count"),
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_materialized_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_materialized_count"),
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_surface_count": replay_surface_count,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_attempt_count": replay_surface_count,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_surfaces": replay_surfaces,
        "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency": denials,
        "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_count": denied_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
        "allowed_next_actions": [
            {
                "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_denial_gate",
                "status": "allowed_report_only_next_slice",
                "records_result_receipt": false,
                "persists_result_receipt": false,
                "records_idempotency": false,
                "accepts_duplicate_receipt": false,
                "accepts_replay": false,
                "accepts_cross_scope_reuse": false,
                "records_operator_acceptance": false,
                "derives_release_publication_authority": false,
                "derives_activation_authority": false,
                "activates_live": false,
                "installs_or_restarts": false,
                "mutates_active_binary": false,
                "mutates_memory_store": false,
                "writes_kg": false,
                "sends_externally": false
            }
        ],
        }),
    );

    let replay_zero_keys = [
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_allowed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_duplicate_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_duplicate_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_duplicate_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_idempotency_key_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_idempotency_key_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_idempotency_state_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_idempotency_state_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_idempotency_state_materialized_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_idempotency_filesystem_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_nonce_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_nonce_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cross_scope_reuse_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_status_upgrade_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_completed_status_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ack_replay_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ledger_replay_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_index_replay_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_delivery_replay_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_query_replay_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_observability_replay_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_hash_rebind_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_status_rebind_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_signature_timestamp_replay_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_reuse_accepted_count",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in replay_zero_keys {
            report_object.insert(key.to_string(), serde_json::json!(0));
        }
    }

    let replay_false_keys = [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_allowed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_duplicate_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_idempotency_key_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_idempotency_key_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_idempotency_state_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_idempotency_filesystem_written",
        "artifact_download_install_affordance_result_receipt_recorded",
        "artifact_download_install_affordance_result_receipt_persisted",
        "artifact_download_install_affordance_result_receipt_accepted",
        "artifact_download_install_affordance_result_receipt_materialized",
        "artifact_download_install_affordance_result_receipt_filesystem_written",
        "artifact_download_install_affordance_completion_ack_recorded",
        "artifact_download_install_affordance_completion_ack_accepted",
        "operator_acceptance_recorded",
        "operator_approval_recorded",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "activation_command_derived",
        "activation_allowed",
        "activation_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
        "public_release_claimed",
        "public_ga_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "external_send_performed",
        "filesystem_written",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in replay_false_keys {
            report_object.insert(key.to_string(), serde_json::json!(false));
        }
    }

    let mut side_effects = serde_json::Map::new();
    for key in replay_false_keys {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_report =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_denial_report();
    let source_ready = source_report["status"].as_str() == Some("ready")
        && source_report["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_denial_ready"]
            .as_bool()
            .unwrap_or(false);
    let source_u64 = |key: &str| source_report[key].as_u64().unwrap_or(0);
    let source_report_sha256 = sha256_json_value(&source_report);
    let source_contract_hash = source_report
        ["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_contract_hash_sha256"]
        .as_str()
        .unwrap_or("unknown")
        .to_string();
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let ordering_surface_specs = [
        (
            "source_replay_idempotency_report_required",
            "blocked_source_report_required_noop",
            "source_replay_idempotency_report_required",
        ),
        (
            "download_button_result_receipt_sequence_cursor_recording",
            "blocked_sequence_cursor_recording_noop",
            "download_button_result_receipt_sequence_cursor_recording_denied",
        ),
        (
            "direct_download_url_result_receipt_out_of_order_sequence",
            "blocked_out_of_order_sequence_noop",
            "direct_download_url_result_receipt_out_of_order_sequence_denied",
        ),
        (
            "checksum_prompt_result_receipt_sequence_gap_skip",
            "blocked_sequence_gap_noop",
            "checksum_prompt_result_receipt_sequence_gap_skip_denied",
        ),
        (
            "package_manager_install_command_result_receipt_timestamp_rollback",
            "blocked_timestamp_rollback_noop",
            "package_manager_install_command_result_receipt_timestamp_rollback_denied",
        ),
        (
            "curl_pipe_shell_result_receipt_epoch_rollback",
            "blocked_epoch_rollback_noop",
            "curl_pipe_shell_result_receipt_epoch_rollback_denied",
        ),
        (
            "installer_launch_prompt_result_receipt_same_sequence_different_hash",
            "blocked_same_sequence_hash_noop",
            "installer_launch_prompt_result_receipt_same_sequence_different_hash_denied",
        ),
        (
            "auto_update_offer_result_receipt_latest_wins_overwrite",
            "blocked_latest_wins_overwrite_noop",
            "auto_update_offer_result_receipt_latest_wins_overwrite_denied",
        ),
        (
            "release_channel_subscription_result_receipt_ack_before_noop",
            "blocked_ack_before_noop_noop",
            "release_channel_subscription_result_receipt_ack_before_noop_denied",
        ),
        (
            "update_feed_hint_result_receipt_stage_transition_bypass",
            "blocked_stage_transition_ordering_noop",
            "update_feed_hint_result_receipt_stage_transition_bypass_denied",
        ),
        (
            "package_registry_badge_result_receipt_ledger_index_delivery_bypass",
            "blocked_ledger_index_delivery_ordering_noop",
            "package_registry_badge_result_receipt_ledger_index_delivery_bypass_denied",
        ),
        (
            "cdn_mirror_download_result_receipt_export_query_observability_bypass",
            "blocked_export_query_observability_ordering_noop",
            "cdn_mirror_download_result_receipt_export_query_observability_bypass_denied",
        ),
        (
            "sbom_provenance_notarization_result_receipt_hash_status_ordering_rebind",
            "blocked_hash_status_ordering_rebind_noop",
            "sbom_provenance_notarization_result_receipt_hash_status_ordering_rebind_denied",
        ),
        (
            "signature_verification_command_result_receipt_signature_timestamp_ordering",
            "blocked_signature_timestamp_ordering_noop",
            "signature_verification_command_result_receipt_signature_timestamp_ordering_denied",
        ),
        (
            "one_click_install_deep_link_result_receipt_activation_authority_ordering_bypass",
            "blocked_activation_authority_ordering_noop",
            "one_click_install_deep_link_result_receipt_activation_authority_ordering_bypass_denied",
        ),
        (
            "external_telegram_install_message_result_receipt_external_ordering_bypass",
            "blocked_external_delivery_ordering_noop",
            "external_telegram_install_message_result_receipt_external_ordering_bypass_denied",
        ),
        (
            "release_publication_authority_install_affordance_result_receipt_ordering_bypass",
            "blocked_release_publication_authority_ordering_noop",
            "release_publication_authority_install_affordance_result_receipt_ordering_bypass_denied",
        ),
        (
            "activation_live_install_restart_active_binary_result_receipt_ordering_bypass",
            "blocked_activation_live_install_restart_active_binary_ordering_noop",
            "activation_live_install_restart_active_binary_result_receipt_ordering_bypass_denied",
        ),
    ];
    let ordering_surface_false_keys = [
        "artifact_download_install_affordance_result_receipt_ordering_allowed",
        "artifact_download_install_affordance_result_receipt_ordering_recorded",
        "artifact_download_install_affordance_result_receipt_ordering_persisted",
        "artifact_download_install_affordance_result_receipt_ordering_performed",
        "artifact_download_install_affordance_result_receipt_sequence_cursor_accepted",
        "artifact_download_install_affordance_result_receipt_sequence_cursor_recorded",
        "artifact_download_install_affordance_result_receipt_sequence_cursor_persisted",
        "artifact_download_install_affordance_result_receipt_monotonicity_state_recorded",
        "artifact_download_install_affordance_result_receipt_monotonicity_state_persisted",
        "artifact_download_install_affordance_result_receipt_monotonicity_state_materialized",
        "artifact_download_install_affordance_result_receipt_monotonicity_filesystem_written",
        "artifact_download_install_affordance_result_receipt_timestamp_ordering_accepted",
        "artifact_download_install_affordance_result_receipt_epoch_ordering_accepted",
        "artifact_download_install_affordance_result_receipt_stage_ordering_accepted",
        "artifact_download_install_affordance_result_receipt_same_sequence_hash_override_accepted",
        "artifact_download_install_affordance_result_receipt_latest_wins_overwrite_accepted",
        "artifact_download_install_affordance_result_receipt_gap_fill_accepted",
        "artifact_download_install_affordance_result_receipt_ack_before_noop_accepted",
        "artifact_download_install_affordance_result_receipt_ledger_ordering_bypass_accepted",
        "artifact_download_install_affordance_result_receipt_index_ordering_bypass_accepted",
        "artifact_download_install_affordance_result_receipt_delivery_ordering_bypass_accepted",
        "artifact_download_install_affordance_result_receipt_export_ordering_bypass_accepted",
        "artifact_download_install_affordance_result_receipt_query_ordering_bypass_accepted",
        "artifact_download_install_affordance_result_receipt_observability_ordering_bypass_accepted",
        "artifact_download_install_affordance_result_receipt_runtime_ordering_bypass_accepted",
        "artifact_download_install_affordance_result_receipt_provider_ordering_bypass_accepted",
        "artifact_download_install_affordance_result_receipt_memory_kg_ordering_bypass_accepted",
        "artifact_download_install_affordance_result_receipt_external_public_install_ordering_bypass_accepted",
        "artifact_download_install_affordance_result_receipt_replay_allowed",
        "artifact_download_install_affordance_result_receipt_duplicate_accepted",
        "artifact_download_install_affordance_result_receipt_idempotency_key_accepted",
        "artifact_download_install_affordance_result_receipt_idempotency_state_recorded",
        "artifact_download_install_affordance_result_receipt_recorded",
        "artifact_download_install_affordance_result_receipt_persisted",
        "artifact_download_install_affordance_result_receipt_accepted",
        "artifact_download_install_affordance_result_receipt_materialized",
        "artifact_download_install_affordance_result_receipt_filesystem_written",
        "artifact_download_install_affordance_result_receipt_ledger_written",
        "artifact_download_install_affordance_result_receipt_indexed",
        "artifact_download_install_affordance_result_receipt_enqueued",
        "artifact_download_install_affordance_result_receipt_delivered",
        "artifact_download_install_affordance_result_receipt_exported",
        "artifact_download_install_affordance_result_receipt_query_registered",
        "artifact_download_install_affordance_result_receipt_observability_recorded",
        "artifact_download_install_affordance_completion_ack_recorded",
        "artifact_download_install_affordance_completion_ack_accepted",
        "download_button_rendered",
        "direct_download_url_exposed",
        "package_manager_install_command_rendered",
        "curl_pipe_shell_snippet_rendered",
        "installer_launch_prompt_rendered",
        "auto_update_offer_rendered",
        "external_install_message_sent",
        "telegram_install_message_sent",
        "operator_approval_from_ordering_accepted",
        "release_publication_authority_from_ordering_derived",
        "activation_authority_from_ordering_derived",
        "activation_command_from_ordering_derived",
        "activation_from_ordering_allowed",
        "live_execution_from_ordering_allowed",
        "install_from_ordering_executed",
        "service_restart_from_ordering_performed",
        "launchd_from_ordering_mutated",
        "active_binary_from_ordering_mutated",
        "activation_activated",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "telegram_send_performed",
        "channel_send_performed",
        "external_send_performed",
        "public_release_claimed",
        "public_ga_claimed",
        "release_artifact_written",
        "public_artifact_written",
    ];
    let ordering_surfaces = ordering_surface_specs
        .iter()
        .map(|(surface, status, reason)| {
            let mut surface_report = serde_json::json!({
                "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_surface": surface,
                "source_replay_idempotency_present": true,
                "source_replay_idempotency_ready": true,
                "source_replay_idempotency_noop_confirmed": true,
                "canonical_noop_result_receipt_order_identity_required": true,
                "artifact_download_install_affordance_result_receipt_ordering_requested": true,
                "artifact_download_install_affordance_result_receipt_ordering_status": status,
                "receipt_noop_confirmed": true,
                "reason": reason,
            });
            if let Some(surface_object) = surface_report.as_object_mut() {
                for key in ordering_surface_false_keys {
                    surface_object.insert(key.to_string(), serde_json::json!(false));
                }
                match *surface {
                    "source_replay_idempotency_report_required" => {
                        surface_object.insert(
                            "source_replay_idempotency_report_required".to_string(),
                            serde_json::json!(true),
                        );
                    }
                    "download_button_result_receipt_sequence_cursor_recording" => {
                        surface_object.insert(
                            "sequence_cursor_recording_requested".to_string(),
                            serde_json::json!(true),
                        );
                    }
                    "direct_download_url_result_receipt_out_of_order_sequence" => {
                        surface_object.insert(
                            "out_of_order_sequence_requested".to_string(),
                            serde_json::json!(true),
                        );
                        surface_object.insert(
                            "requested_sequence".to_string(),
                            serde_json::json!(2),
                        );
                        surface_object.insert(
                            "observed_previous_sequence".to_string(),
                            serde_json::json!(3),
                        );
                    }
                    "checksum_prompt_result_receipt_sequence_gap_skip" => {
                        surface_object
                            .insert("sequence_gap_requested".to_string(), serde_json::json!(true));
                        surface_object.insert(
                            "requested_sequence".to_string(),
                            serde_json::json!(5),
                        );
                        surface_object.insert(
                            "expected_next_sequence".to_string(),
                            serde_json::json!(1),
                        );
                    }
                    "package_manager_install_command_result_receipt_timestamp_rollback" => {
                        surface_object.insert(
                            "timestamp_rollback_requested".to_string(),
                            serde_json::json!(true),
                        );
                    }
                    "curl_pipe_shell_result_receipt_epoch_rollback" => {
                        surface_object
                            .insert("epoch_rollback_requested".to_string(), serde_json::json!(true));
                    }
                    "installer_launch_prompt_result_receipt_same_sequence_different_hash" => {
                        surface_object.insert(
                            "same_sequence_different_hash_requested".to_string(),
                            serde_json::json!(true),
                        );
                    }
                    "auto_update_offer_result_receipt_latest_wins_overwrite" => {
                        surface_object.insert(
                            "latest_wins_overwrite_requested".to_string(),
                            serde_json::json!(true),
                        );
                    }
                    "release_channel_subscription_result_receipt_ack_before_noop" => {
                        surface_object.insert(
                            "completion_ack_before_noop_requested".to_string(),
                            serde_json::json!(true),
                        );
                    }
                    "update_feed_hint_result_receipt_stage_transition_bypass" => {
                        surface_object.insert(
                            "stage_transition_ordering_bypass_requested".to_string(),
                            serde_json::json!(true),
                        );
                    }
                    "package_registry_badge_result_receipt_ledger_index_delivery_bypass" => {
                        for key in [
                            "ledger_ordering_bypass_requested",
                            "index_ordering_bypass_requested",
                            "delivery_ordering_bypass_requested",
                        ] {
                            surface_object.insert(key.to_string(), serde_json::json!(true));
                        }
                    }
                    "cdn_mirror_download_result_receipt_export_query_observability_bypass" => {
                        for key in [
                            "export_ordering_bypass_requested",
                            "query_ordering_bypass_requested",
                            "observability_ordering_bypass_requested",
                        ] {
                            surface_object.insert(key.to_string(), serde_json::json!(true));
                        }
                    }
                    "sbom_provenance_notarization_result_receipt_hash_status_ordering_rebind" => {
                        surface_object.insert(
                            "hash_ordering_rebind_requested".to_string(),
                            serde_json::json!(true),
                        );
                        surface_object.insert(
                            "status_ordering_rebind_requested".to_string(),
                            serde_json::json!(true),
                        );
                    }
                    "signature_verification_command_result_receipt_signature_timestamp_ordering" => {
                        surface_object.insert(
                            "signature_ordering_requested".to_string(),
                            serde_json::json!(true),
                        );
                        surface_object.insert(
                            "timestamp_ordering_requested".to_string(),
                            serde_json::json!(true),
                        );
                    }
                    "one_click_install_deep_link_result_receipt_activation_authority_ordering_bypass" => {
                        surface_object.insert(
                            "activation_authority_ordering_bypass_requested".to_string(),
                            serde_json::json!(true),
                        );
                    }
                    "external_telegram_install_message_result_receipt_external_ordering_bypass" => {
                        surface_object.insert(
                            "external_delivery_ordering_bypass_requested".to_string(),
                            serde_json::json!(true),
                        );
                        surface_object.insert(
                            "telegram_delivery_ordering_bypass_requested".to_string(),
                            serde_json::json!(true),
                        );
                    }
                    "release_publication_authority_install_affordance_result_receipt_ordering_bypass" => {
                        surface_object.insert(
                            "release_publication_authority_ordering_bypass_requested".to_string(),
                            serde_json::json!(true),
                        );
                    }
                    "activation_live_install_restart_active_binary_result_receipt_ordering_bypass" => {
                        for key in [
                            "activation_ordering_bypass_requested",
                            "install_ordering_bypass_requested",
                            "service_restart_ordering_bypass_requested",
                            "active_binary_ordering_bypass_requested",
                        ] {
                            surface_object.insert(key.to_string(), serde_json::json!(true));
                        }
                    }
                    _ => {}
                }
            }
            surface_report
        })
        .collect::<Vec<_>>();
    let ordering_surface_count = ordering_surfaces.len();
    let contract_hash = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-ordering-monotonicity-denial:native:source={source_report_sha256}:surfaces={ordering_surface_count}:route_count={}:ordering=0:cursor=0:monotonicity=0:authority=0:install=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-ordering-monotonicity-denial:no-sequence-cursor:no-monotonicity-state:no-out-of-order:no-gap-fill:no-latest-wins:no-authority:no-install:no-live",
    );
    let denials = vec![
        "source_result_receipt_replay_idempotency_report_required",
        "canonical_noop_result_receipt_order_identity_required",
        "sequence_cursor_acceptance_denied",
        "sequence_cursor_recording_denied",
        "sequence_cursor_persistence_denied",
        "monotonicity_state_recording_denied",
        "monotonicity_state_persistence_denied",
        "monotonicity_state_materialization_denied",
        "out_of_order_sequence_denied",
        "sequence_gap_or_skip_denied",
        "timestamp_rollback_denied",
        "epoch_rollback_denied",
        "same_sequence_different_hash_denied",
        "latest_wins_overwrite_denied",
        "completion_ack_before_noop_denied",
        "stage_transition_ordering_denied",
        "ledger_index_delivery_ordering_bypass_denied",
        "export_query_observability_ordering_bypass_denied",
        "hash_status_ordering_rebind_denied",
        "signature_timestamp_ordering_denied",
        "operator_identity_reuse_ordering_denied",
        "release_publication_authority_ordering_denied",
        "activation_authority_ordering_denied",
        "runtime_provider_memory_kg_ordering_bypass_denied",
        "external_public_release_ordering_bypass_denied",
        "install_restart_active_binary_ordering_bypass_denied",
    ];
    let denied_count = denials.len();
    let report_ready = source_ready
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_surface_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_attempt_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_allowed_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_duplicate_accepted_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_idempotency_state_recorded_count",
        ) == 0
        && ordering_surface_count == 18
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_ORDERING_MONOTONICITY_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-ordering-monotonicity-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-18",
        "minimum_required_samples": 24,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "source_route_wired": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_denial_route_enabled": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_denial_ready": report_ready,
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
        "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_denial_v1",
        "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_mode": "native_route_denied_ordering_cursor_monotonicity_or_latest_wins_attempt_cannot_create_result_receipt_or_install_activation_authority",
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_route": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_denial_route",
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_ready": source_ready,
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_report_sha256": source_report_sha256,
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_contract_hash_sha256": source_contract_hash,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_contract_hash_sha256": contract_hash,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_policy_hash_sha256": policy_hash,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_surface_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_surface_count"),
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_attempt_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_attempt_count"),
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_allowed_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_allowed_count"),
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_duplicate_accepted_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_duplicate_accepted_count"),
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_idempotency_state_recorded_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_idempotency_state_recorded_count"),
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_surface_count": ordering_surface_count,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_attempt_count": ordering_surface_count,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_denied_count": ordering_surface_count,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_surfaces": ordering_surfaces,
        "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity": denials,
        "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_count": denied_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
        "allowed_next_actions": [
            {
                "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_denial_gate",
                "status": "allowed_report_only_next_slice",
                "accepts_cancellation": false,
                "accepts_supersession": false,
                "accepts_out_of_order_receipt": false,
                "records_result_receipt": false,
                "persists_replacement_receipt": false,
                "renders_download_link": false,
                "emits_install_command": false,
                "installs_or_restarts": false,
                "mutates_active_binary": false,
                "mutates_memory_store": false,
                "writes_kg": false,
                "sends_externally": false
            }
        ],
        }),
    );

    let ordering_zero_keys = [
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_allowed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_sequence_cursor_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_sequence_cursor_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_sequence_cursor_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_monotonicity_state_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_monotonicity_state_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_monotonicity_state_materialized_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_monotonicity_filesystem_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_timestamp_ordering_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_epoch_ordering_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_stage_ordering_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_same_sequence_hash_override_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_latest_wins_overwrite_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_gap_fill_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ack_before_noop_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ledger_ordering_bypass_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_index_ordering_bypass_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_delivery_ordering_bypass_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_ordering_bypass_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_query_ordering_bypass_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_observability_ordering_bypass_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_runtime_ordering_bypass_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_provider_ordering_bypass_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_memory_kg_ordering_bypass_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_external_public_install_ordering_bypass_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_materialized_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_filesystem_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ledger_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_indexed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_delivered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_exported_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_query_registered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_observability_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_completion_ack_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_completion_ack_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_approval_from_ordering_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_release_publication_authority_from_ordering_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_activation_authority_from_ordering_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_install_from_ordering_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_service_restart_from_ordering_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_active_binary_from_ordering_mutated_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_memory_store_ordering_performed_count",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in ordering_zero_keys {
            report_object.insert(key.to_string(), serde_json::json!(0));
        }
        for key in [
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_live_kg_ordering_performed_count",
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_provider_ordering_performed_count",
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_model_ordering_performed_count",
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_credential_read_from_ordering_count",
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_secret_read_from_ordering_count",
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_external_send_from_ordering_count",
        ] {
            report_object.insert(key.to_string(), serde_json::json!(0));
        }
    }

    let ordering_false_keys = [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_allowed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_sequence_cursor_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_monotonicity_state_persisted",
        "artifact_download_install_affordance_result_receipt_recorded",
        "artifact_download_install_affordance_result_receipt_persisted",
        "artifact_download_install_affordance_result_receipt_accepted",
        "artifact_download_install_affordance_result_receipt_materialized",
        "artifact_download_install_affordance_result_receipt_filesystem_written",
        "artifact_download_install_affordance_completion_ack_recorded",
        "artifact_download_install_affordance_completion_ack_accepted",
        "operator_acceptance_recorded",
        "operator_approval_recorded",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "activation_command_derived",
        "activation_allowed",
        "activation_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
        "public_release_claimed",
        "public_ga_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "external_send_performed",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in ordering_false_keys {
            report_object.insert(key.to_string(), serde_json::json!(false));
        }
    }

    let side_effect_false_keys = [
        "artifact_download_install_affordance_result_receipt_ordering_allowed",
        "artifact_download_install_affordance_result_receipt_ordering_recorded",
        "artifact_download_install_affordance_result_receipt_ordering_persisted",
        "artifact_download_install_affordance_result_receipt_ordering_performed",
        "artifact_download_install_affordance_result_receipt_sequence_cursor_accepted",
        "artifact_download_install_affordance_result_receipt_sequence_cursor_recorded",
        "artifact_download_install_affordance_result_receipt_sequence_cursor_persisted",
        "artifact_download_install_affordance_result_receipt_monotonicity_state_recorded",
        "artifact_download_install_affordance_result_receipt_monotonicity_state_persisted",
        "artifact_download_install_affordance_result_receipt_monotonicity_state_materialized",
        "artifact_download_install_affordance_result_receipt_monotonicity_filesystem_written",
        "artifact_download_install_affordance_result_receipt_recorded",
        "artifact_download_install_affordance_result_receipt_persisted",
        "artifact_download_install_affordance_result_receipt_accepted",
        "artifact_download_install_affordance_result_receipt_materialized",
        "artifact_download_install_affordance_result_receipt_filesystem_written",
        "artifact_download_install_affordance_completion_ack_recorded",
        "artifact_download_install_affordance_completion_ack_accepted",
        "operator_acceptance_recorded",
        "operator_approval_recorded",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "activation_command_derived",
        "activation_allowed",
        "activation_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
        "release_artifact_written",
        "public_artifact_written",
        "public_release_claimed",
        "public_ga_claimed",
        "telegram_send_performed",
        "channel_send_performed",
        "external_send_performed",
        "filesystem_written",
    ];
    let mut side_effects = serde_json::Map::new();
    for key in side_effect_false_keys {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_report =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_denial_report();
    let source_ready = source_report["status"].as_str() == Some("ready")
        && source_report["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_denial_ready"]
            .as_bool()
            .unwrap_or(false);
    let source_u64 = |key: &str| source_report[key].as_u64().unwrap_or(0);
    let source_report_sha256 = sha256_json_value(&source_report);
    let source_contract_hash = source_report
        ["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_contract_hash_sha256"]
        .as_str()
        .unwrap_or("unknown")
        .to_string();
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let cancellation_surface_specs = [
        (
            "source_ordering_monotonicity_report_required",
            "blocked_source_report_required_noop",
            "source_ordering_monotonicity_report_required",
        ),
        (
            "download_button_result_receipt_cancel_claim",
            "blocked_cancellation_acceptance_noop",
            "download_button_result_receipt_cancel_claim_denied",
        ),
        (
            "direct_download_url_result_receipt_revoke_claim",
            "blocked_revocation_acceptance_noop",
            "direct_download_url_result_receipt_revoke_claim_denied",
        ),
        (
            "checksum_prompt_result_receipt_withdraw_claim",
            "blocked_withdrawal_acceptance_noop",
            "checksum_prompt_result_receipt_withdraw_claim_denied",
        ),
        (
            "package_manager_install_command_result_receipt_supersede_claim",
            "blocked_supersession_acceptance_noop",
            "package_manager_install_command_result_receipt_supersede_claim_denied",
        ),
        (
            "curl_pipe_shell_result_receipt_replacement_receipt",
            "blocked_replacement_receipt_noop",
            "curl_pipe_shell_result_receipt_replacement_receipt_denied",
        ),
        (
            "installer_launch_prompt_result_receipt_tombstone_claim",
            "blocked_tombstone_noop",
            "installer_launch_prompt_result_receipt_tombstone_claim_denied",
        ),
        (
            "auto_update_offer_result_receipt_delete_marker_claim",
            "blocked_delete_marker_noop",
            "auto_update_offer_result_receipt_delete_marker_claim_denied",
        ),
        (
            "release_channel_subscription_result_receipt_latest_replacement",
            "blocked_latest_replacement_noop",
            "release_channel_subscription_result_receipt_latest_replacement_denied",
        ),
        (
            "update_feed_hint_result_receipt_ack_replacement",
            "blocked_ack_replacement_noop",
            "update_feed_hint_result_receipt_ack_replacement_denied",
        ),
        (
            "package_registry_badge_result_receipt_query_export_observability_replacement",
            "blocked_query_export_observability_replacement_noop",
            "package_registry_badge_result_receipt_query_export_observability_replacement_denied",
        ),
        (
            "cdn_mirror_download_result_receipt_ordering_replacement_bypass",
            "blocked_ordering_replacement_bypass_noop",
            "cdn_mirror_download_result_receipt_ordering_replacement_bypass_denied",
        ),
        (
            "sbom_provenance_notarization_result_receipt_idempotency_replacement_bypass",
            "blocked_idempotency_replacement_bypass_noop",
            "sbom_provenance_notarization_result_receipt_idempotency_replacement_bypass_denied",
        ),
        (
            "signature_verification_command_result_receipt_completion_ack_replacement",
            "blocked_completion_ack_replacement_noop",
            "signature_verification_command_result_receipt_completion_ack_replacement_denied",
        ),
        (
            "one_click_install_deep_link_result_receipt_activation_authority_replacement",
            "blocked_activation_authority_replacement_noop",
            "one_click_install_deep_link_result_receipt_activation_authority_replacement_denied",
        ),
        (
            "external_telegram_install_message_result_receipt_external_replacement",
            "blocked_external_replacement_noop",
            "external_telegram_install_message_result_receipt_external_replacement_denied",
        ),
        (
            "release_publication_authority_install_affordance_result_receipt_replacement",
            "blocked_release_publication_authority_replacement_noop",
            "release_publication_authority_install_affordance_result_receipt_replacement_denied",
        ),
        (
            "activation_live_install_restart_active_binary_result_receipt_replacement",
            "blocked_activation_live_install_restart_active_binary_replacement_noop",
            "activation_live_install_restart_active_binary_result_receipt_replacement_denied",
        ),
    ];
    let cancellation_surface_false_keys = [
        "artifact_download_install_affordance_result_receipt_cancellation_supersession_allowed",
        "artifact_download_install_affordance_result_receipt_cancellation_supersession_recorded",
        "artifact_download_install_affordance_result_receipt_cancellation_supersession_persisted",
        "artifact_download_install_affordance_result_receipt_cancellation_supersession_performed",
        "artifact_download_install_affordance_result_receipt_cancellation_accepted",
        "artifact_download_install_affordance_result_receipt_cancellation_recorded",
        "artifact_download_install_affordance_result_receipt_cancellation_persisted",
        "artifact_download_install_affordance_result_receipt_revocation_accepted",
        "artifact_download_install_affordance_result_receipt_withdrawal_accepted",
        "artifact_download_install_affordance_result_receipt_supersession_accepted",
        "artifact_download_install_affordance_result_receipt_supersession_recorded",
        "artifact_download_install_affordance_result_receipt_supersession_persisted",
        "artifact_download_install_affordance_result_receipt_replacement_receipt_accepted",
        "artifact_download_install_affordance_result_receipt_replacement_receipt_recorded",
        "artifact_download_install_affordance_result_receipt_replacement_receipt_persisted",
        "artifact_download_install_affordance_result_receipt_tombstone_recorded",
        "artifact_download_install_affordance_result_receipt_tombstone_persisted",
        "artifact_download_install_affordance_result_receipt_delete_marker_recorded",
        "artifact_download_install_affordance_result_receipt_latest_replacement_accepted",
        "artifact_download_install_affordance_result_receipt_ack_replacement_accepted",
        "artifact_download_install_affordance_result_receipt_query_replacement_registered",
        "artifact_download_install_affordance_result_receipt_export_replacement_recorded",
        "artifact_download_install_affordance_result_receipt_observability_replacement_recorded",
        "artifact_download_install_affordance_result_receipt_ordering_recorded",
        "artifact_download_install_affordance_result_receipt_sequence_cursor_recorded",
        "artifact_download_install_affordance_result_receipt_monotonicity_state_recorded",
        "artifact_download_install_affordance_result_receipt_replay_recorded",
        "artifact_download_install_affordance_result_receipt_idempotency_state_recorded",
        "artifact_download_install_affordance_result_receipt_recorded",
        "artifact_download_install_affordance_result_receipt_persisted",
        "artifact_download_install_affordance_result_receipt_accepted",
        "artifact_download_install_affordance_result_receipt_materialized",
        "artifact_download_install_affordance_result_receipt_filesystem_written",
        "artifact_download_install_affordance_result_receipt_ledger_written",
        "artifact_download_install_affordance_result_receipt_indexed",
        "artifact_download_install_affordance_result_receipt_enqueued",
        "artifact_download_install_affordance_result_receipt_delivered",
        "artifact_download_install_affordance_result_receipt_exported",
        "artifact_download_install_affordance_result_receipt_query_registered",
        "artifact_download_install_affordance_result_receipt_observability_recorded",
        "artifact_download_install_affordance_completion_ack_recorded",
        "artifact_download_install_affordance_completion_ack_accepted",
        "download_button_rendered",
        "direct_download_url_exposed",
        "package_manager_install_command_rendered",
        "curl_pipe_shell_snippet_rendered",
        "installer_launch_prompt_rendered",
        "auto_update_offer_rendered",
        "external_install_message_sent",
        "telegram_install_message_sent",
        "operator_approval_from_cancellation_supersession_accepted",
        "release_publication_authority_from_cancellation_supersession_derived",
        "activation_authority_from_cancellation_supersession_derived",
        "activation_command_from_cancellation_supersession_derived",
        "activation_from_cancellation_supersession_allowed",
        "live_execution_from_cancellation_supersession_allowed",
        "install_from_cancellation_supersession_executed",
        "service_restart_from_cancellation_supersession_performed",
        "launchd_from_cancellation_supersession_mutated",
        "active_binary_from_cancellation_supersession_mutated",
        "activation_activated",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "telegram_send_performed",
        "channel_send_performed",
        "external_send_performed",
        "public_release_claimed",
        "public_ga_claimed",
        "release_artifact_written",
        "public_artifact_written",
    ];
    let cancellation_surfaces = cancellation_surface_specs
        .iter()
        .map(|(surface, status, reason)| {
            let mut surface_report = serde_json::json!({
                "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_surface": surface,
                "source_ordering_monotonicity_present": true,
                "source_ordering_monotonicity_ready": true,
                "source_ordering_noop_confirmed": true,
                "canonical_noop_result_receipt_replacement_identity_required": true,
                "artifact_download_install_affordance_result_receipt_cancellation_supersession_requested": true,
                "artifact_download_install_affordance_result_receipt_cancellation_supersession_status": status,
                "cancellation_supersession_noop_confirmed": true,
                "reason": reason,
            });
            if let Some(surface_object) = surface_report.as_object_mut() {
                for key in cancellation_surface_false_keys {
                    surface_object.insert(key.to_string(), serde_json::json!(false));
                }
                match *surface {
                    "source_ordering_monotonicity_report_required" => {
                        surface_object.insert(
                            "source_ordering_monotonicity_report_required".to_string(),
                            serde_json::json!(true),
                        );
                    }
                    "download_button_result_receipt_cancel_claim" => {
                        surface_object.insert(
                            "cancellation_claim_requested".to_string(),
                            serde_json::json!(true),
                        );
                    }
                    "direct_download_url_result_receipt_revoke_claim" => {
                        surface_object.insert(
                            "revocation_claim_requested".to_string(),
                            serde_json::json!(true),
                        );
                    }
                    "checksum_prompt_result_receipt_withdraw_claim" => {
                        surface_object.insert(
                            "withdrawal_claim_requested".to_string(),
                            serde_json::json!(true),
                        );
                    }
                    "package_manager_install_command_result_receipt_supersede_claim" => {
                        surface_object.insert(
                            "supersession_claim_requested".to_string(),
                            serde_json::json!(true),
                        );
                    }
                    "curl_pipe_shell_result_receipt_replacement_receipt" => {
                        surface_object.insert(
                            "replacement_receipt_requested".to_string(),
                            serde_json::json!(true),
                        );
                    }
                    "installer_launch_prompt_result_receipt_tombstone_claim" => {
                        surface_object
                            .insert("tombstone_requested".to_string(), serde_json::json!(true));
                    }
                    "auto_update_offer_result_receipt_delete_marker_claim" => {
                        surface_object.insert(
                            "delete_marker_requested".to_string(),
                            serde_json::json!(true),
                        );
                    }
                    "release_channel_subscription_result_receipt_latest_replacement" => {
                        surface_object.insert(
                            "latest_replacement_requested".to_string(),
                            serde_json::json!(true),
                        );
                    }
                    "update_feed_hint_result_receipt_ack_replacement" => {
                        surface_object.insert(
                            "ack_replacement_requested".to_string(),
                            serde_json::json!(true),
                        );
                    }
                    "package_registry_badge_result_receipt_query_export_observability_replacement" => {
                        for key in [
                            "query_replacement_requested",
                            "export_replacement_requested",
                            "observability_replacement_requested",
                        ] {
                            surface_object.insert(key.to_string(), serde_json::json!(true));
                        }
                    }
                    "cdn_mirror_download_result_receipt_ordering_replacement_bypass" => {
                        for key in [
                            "ordering_replacement_bypass_requested",
                            "sequence_cursor_replacement_requested",
                            "monotonicity_state_replacement_requested",
                        ] {
                            surface_object.insert(key.to_string(), serde_json::json!(true));
                        }
                    }
                    "sbom_provenance_notarization_result_receipt_idempotency_replacement_bypass" => {
                        surface_object.insert(
                            "idempotency_replacement_bypass_requested".to_string(),
                            serde_json::json!(true),
                        );
                    }
                    "signature_verification_command_result_receipt_completion_ack_replacement" => {
                        surface_object.insert(
                            "completion_ack_replacement_requested".to_string(),
                            serde_json::json!(true),
                        );
                    }
                    "one_click_install_deep_link_result_receipt_activation_authority_replacement" => {
                        surface_object.insert(
                            "activation_authority_replacement_requested".to_string(),
                            serde_json::json!(true),
                        );
                    }
                    "external_telegram_install_message_result_receipt_external_replacement" => {
                        for key in [
                            "external_replacement_requested",
                            "telegram_replacement_requested",
                        ] {
                            surface_object.insert(key.to_string(), serde_json::json!(true));
                        }
                    }
                    "release_publication_authority_install_affordance_result_receipt_replacement" => {
                        surface_object.insert(
                            "release_publication_authority_replacement_requested".to_string(),
                            serde_json::json!(true),
                        );
                    }
                    "activation_live_install_restart_active_binary_result_receipt_replacement" => {
                        for key in [
                            "activation_replacement_requested",
                            "install_replacement_requested",
                            "service_restart_replacement_requested",
                            "active_binary_replacement_requested",
                        ] {
                            surface_object.insert(key.to_string(), serde_json::json!(true));
                        }
                    }
                    _ => {}
                }
            }
            surface_report
        })
        .collect::<Vec<_>>();
    let cancellation_surface_count = cancellation_surfaces.len();
    let contract_hash = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-cancellation-supersession-denial:native:source={source_report_sha256}:surfaces={cancellation_surface_count}:route_count={}:cancel=0:revoke=0:supersede=0:replacement=0:tombstone=0:authority=0:install=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-cancellation-supersession-denial:no-cancel:no-revoke:no-supersede:no-replacement:no-tombstone:no-authority:no-install:no-live",
    );
    let denials = vec![
        "source_result_receipt_ordering_monotonicity_report_required",
        "canonical_noop_result_receipt_replacement_identity_required",
        "cancellation_acceptance_denied",
        "cancellation_recording_denied",
        "cancellation_persistence_denied",
        "revocation_acceptance_denied",
        "withdrawal_acceptance_denied",
        "supersession_acceptance_denied",
        "supersession_recording_denied",
        "supersession_persistence_denied",
        "replacement_receipt_acceptance_denied",
        "replacement_receipt_recording_denied",
        "replacement_receipt_persistence_denied",
        "tombstone_recording_denied",
        "tombstone_persistence_denied",
        "delete_marker_recording_denied",
        "latest_replacement_denied",
        "ack_replacement_denied",
        "query_export_observability_replacement_denied",
        "ordering_monotonicity_replacement_bypass_denied",
        "idempotency_replacement_bypass_denied",
        "completion_ack_replacement_denied",
        "operator_approval_from_cancellation_supersession_denied",
        "release_publication_authority_from_cancellation_supersession_denied",
        "activation_authority_from_cancellation_supersession_denied",
        "external_public_release_replacement_denied",
        "install_restart_active_binary_replacement_denied",
    ];
    let denied_count = denials.len();
    let report_ready = source_ready
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_surface_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_attempt_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_denied_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_recorded_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_persisted_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_sequence_cursor_recorded_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_monotonicity_state_recorded_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_release_publication_authority_from_ordering_derived_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_activation_authority_from_ordering_derived_count",
        ) == 0
        && cancellation_surface_count == 18
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_CANCELLATION_SUPERSESSION_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-cancellation-supersession-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-19",
        "minimum_required_samples": 24,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "source_route_wired": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_denial_route_enabled": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_denial_ready": report_ready,
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
        "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_denial_v1",
        "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_mode": "native_route_denied_cancel_revoke_supersede_or_replacement_attempt_cannot_create_result_receipt_or_install_activation_authority",
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_route": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_denial_route",
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_ready": source_ready,
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_report_sha256": source_report_sha256,
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_contract_hash_sha256": source_contract_hash,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_contract_hash_sha256": contract_hash,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_policy_hash_sha256": policy_hash,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_surface_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_surface_count"),
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_attempt_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_attempt_count"),
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_denied_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_denied_count"),
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_recorded_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_recorded_count"),
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_persisted_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_persisted_count"),
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_sequence_cursor_recorded_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_sequence_cursor_recorded_count"),
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_monotonicity_state_recorded_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_monotonicity_state_recorded_count"),
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_release_publication_authority_from_ordering_derived_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_release_publication_authority_from_ordering_derived_count"),
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_activation_authority_from_ordering_derived_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_activation_authority_from_ordering_derived_count"),
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_surface_count": cancellation_surface_count,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_attempt_count": cancellation_surface_count,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_denied_count": cancellation_surface_count,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_surfaces": cancellation_surfaces,
        "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession": denials,
        "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_count": denied_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
        "allowed_next_actions": [
            {
                "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_immutable_evidence_denial_gate",
                "status": "allowed_report_only_next_slice",
                "accepts_cancellation": false,
                "accepts_supersession": false,
                "accepts_replacement_receipt": false,
                "records_tombstone": false,
                "records_delete_marker": false,
                "records_audit_trail": false,
                "persists_immutable_evidence": false,
                "records_result_receipt": false,
                "persists_result_receipt": false,
                "renders_download_link": false,
                "emits_install_command": false,
                "installs_or_restarts": false,
                "mutates_active_binary": false,
                "mutates_memory_store": false,
                "writes_kg": false,
                "sends_externally": false
            }
        ],
        }),
    );

    let cancellation_zero_keys = [
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_allowed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_revocation_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_withdrawal_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_supersession_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_supersession_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_supersession_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replacement_receipt_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replacement_receipt_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replacement_receipt_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_tombstone_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_tombstone_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_delete_marker_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_latest_replacement_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ack_replacement_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_query_replacement_registered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_replacement_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_observability_replacement_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_acceptance_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_approval_from_cancellation_supersession_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_release_publication_authority_from_cancellation_supersession_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_activation_authority_from_cancellation_supersession_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_activation_command_from_cancellation_supersession_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_live_execution_from_cancellation_supersession_allowed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_install_from_cancellation_supersession_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_service_restart_from_cancellation_supersession_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_active_binary_from_cancellation_supersession_mutated_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_memory_store_cancellation_supersession_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_live_kg_cancellation_supersession_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_provider_cancellation_supersession_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_model_cancellation_supersession_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_credential_read_from_cancellation_supersession_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_secret_read_from_cancellation_supersession_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_external_send_from_cancellation_supersession_count",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in cancellation_zero_keys {
            report_object.insert(key.to_string(), serde_json::json!(0));
        }
    }

    let cancellation_false_keys = [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_allowed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_supersession_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replacement_receipt_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_tombstone_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_delete_marker_recorded",
        "artifact_download_install_affordance_result_receipt_recorded",
        "artifact_download_install_affordance_result_receipt_persisted",
        "artifact_download_install_affordance_result_receipt_accepted",
        "artifact_download_install_affordance_result_receipt_materialized",
        "artifact_download_install_affordance_result_receipt_filesystem_written",
        "artifact_download_install_affordance_completion_ack_recorded",
        "artifact_download_install_affordance_completion_ack_accepted",
        "operator_acceptance_recorded",
        "operator_approval_recorded",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "activation_command_derived",
        "activation_allowed",
        "activation_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
        "public_release_claimed",
        "public_ga_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "external_send_performed",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in cancellation_false_keys {
            report_object.insert(key.to_string(), serde_json::json!(false));
        }
    }

    let side_effect_false_keys = [
        "artifact_download_install_affordance_result_receipt_cancellation_supersession_allowed",
        "artifact_download_install_affordance_result_receipt_cancellation_supersession_recorded",
        "artifact_download_install_affordance_result_receipt_cancellation_supersession_persisted",
        "artifact_download_install_affordance_result_receipt_cancellation_accepted",
        "artifact_download_install_affordance_result_receipt_cancellation_recorded",
        "artifact_download_install_affordance_result_receipt_cancellation_persisted",
        "artifact_download_install_affordance_result_receipt_revocation_accepted",
        "artifact_download_install_affordance_result_receipt_withdrawal_accepted",
        "artifact_download_install_affordance_result_receipt_supersession_accepted",
        "artifact_download_install_affordance_result_receipt_supersession_recorded",
        "artifact_download_install_affordance_result_receipt_supersession_persisted",
        "artifact_download_install_affordance_result_receipt_replacement_receipt_accepted",
        "artifact_download_install_affordance_result_receipt_replacement_receipt_recorded",
        "artifact_download_install_affordance_result_receipt_replacement_receipt_persisted",
        "artifact_download_install_affordance_result_receipt_tombstone_recorded",
        "artifact_download_install_affordance_result_receipt_tombstone_persisted",
        "artifact_download_install_affordance_result_receipt_delete_marker_recorded",
        "artifact_download_install_affordance_result_receipt_latest_replacement_accepted",
        "artifact_download_install_affordance_result_receipt_ack_replacement_accepted",
        "artifact_download_install_affordance_result_receipt_query_replacement_registered",
        "artifact_download_install_affordance_result_receipt_export_replacement_recorded",
        "artifact_download_install_affordance_result_receipt_observability_replacement_recorded",
        "artifact_download_install_affordance_result_receipt_recorded",
        "artifact_download_install_affordance_result_receipt_persisted",
        "artifact_download_install_affordance_result_receipt_accepted",
        "artifact_download_install_affordance_result_receipt_materialized",
        "artifact_download_install_affordance_result_receipt_filesystem_written",
        "artifact_download_install_affordance_completion_ack_recorded",
        "artifact_download_install_affordance_completion_ack_accepted",
        "operator_acceptance_recorded",
        "operator_approval_recorded",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "activation_command_derived",
        "activation_allowed",
        "activation_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
        "release_artifact_written",
        "public_artifact_written",
        "public_release_claimed",
        "public_ga_claimed",
        "telegram_send_performed",
        "channel_send_performed",
        "external_send_performed",
        "filesystem_written",
    ];
    let mut side_effects = serde_json::Map::new();
    for key in side_effect_false_keys {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_immutable_evidence_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_report =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_denial_report();
    let source_ready = source_report["status"].as_str() == Some("ready")
        && source_report["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_denial_ready"]
            .as_bool()
            .unwrap_or(false);
    let source_u64 = |key: &str| source_report[key].as_u64().unwrap_or(0);
    let source_report_sha256 = sha256_json_value(&source_report);
    let source_contract_hash = source_report
        ["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_contract_hash_sha256"]
        .as_str()
        .unwrap_or("unknown")
        .to_string();
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let audit_surface_specs = [
        (
            "source_cancellation_supersession_report_required",
            "blocked_source_report_required_noop",
            "source_cancellation_supersession_report_required",
        ),
        (
            "download_button_result_receipt_audit_trail_append_claim",
            "blocked_audit_trail_append_noop",
            "download_button_result_receipt_audit_trail_append_claim_denied",
        ),
        (
            "direct_download_url_result_receipt_immutable_evidence_packet",
            "blocked_immutable_evidence_noop",
            "direct_download_url_result_receipt_immutable_evidence_packet_denied",
        ),
        (
            "checksum_prompt_result_receipt_hash_chain_merkle_root",
            "blocked_hash_chain_merkle_root_noop",
            "checksum_prompt_result_receipt_hash_chain_merkle_root_denied",
        ),
        (
            "package_manager_install_command_result_receipt_attestation_witness_notary",
            "blocked_attestation_witness_notary_noop",
            "package_manager_install_command_result_receipt_attestation_witness_notary_denied",
        ),
        (
            "curl_pipe_shell_result_receipt_audit_materialization_filesystem",
            "blocked_audit_materialization_noop",
            "curl_pipe_shell_result_receipt_audit_materialization_filesystem_denied",
        ),
        (
            "installer_launch_prompt_result_receipt_ledger_index_delivery_evidence",
            "blocked_ledger_index_delivery_noop",
            "installer_launch_prompt_result_receipt_ledger_index_delivery_evidence_denied",
        ),
        (
            "auto_update_offer_result_receipt_export_query_observability_evidence",
            "blocked_export_query_observability_noop",
            "auto_update_offer_result_receipt_export_query_observability_evidence_denied",
        ),
        (
            "release_channel_subscription_result_receipt_readback_evidence",
            "blocked_readback_evidence_noop",
            "release_channel_subscription_result_receipt_readback_evidence_denied",
        ),
        (
            "update_feed_hint_result_receipt_completion_ack_audit_evidence",
            "blocked_completion_ack_audit_evidence_noop",
            "update_feed_hint_result_receipt_completion_ack_audit_evidence_denied",
        ),
        (
            "package_registry_badge_result_receipt_cancellation_supersession_audit_evidence",
            "blocked_cancellation_supersession_audit_evidence_noop",
            "package_registry_badge_result_receipt_cancellation_supersession_audit_evidence_denied",
        ),
        (
            "cdn_mirror_download_result_receipt_ordering_monotonicity_audit_evidence",
            "blocked_ordering_monotonicity_audit_evidence_noop",
            "cdn_mirror_download_result_receipt_ordering_monotonicity_audit_evidence_denied",
        ),
        (
            "sbom_provenance_notarization_result_receipt_replay_idempotency_audit_evidence",
            "blocked_replay_idempotency_audit_evidence_noop",
            "sbom_provenance_notarization_result_receipt_replay_idempotency_audit_evidence_denied",
        ),
        (
            "signature_verification_command_result_receipt_release_publication_authority_evidence",
            "blocked_release_publication_authority_evidence_noop",
            "signature_verification_command_result_receipt_release_publication_authority_evidence_denied",
        ),
        (
            "one_click_install_deep_link_result_receipt_activation_authority_evidence",
            "blocked_activation_authority_evidence_noop",
            "one_click_install_deep_link_result_receipt_activation_authority_evidence_denied",
        ),
        (
            "external_telegram_install_message_result_receipt_external_evidence",
            "blocked_external_audit_evidence_noop",
            "external_telegram_install_message_result_receipt_external_evidence_denied",
        ),
        (
            "release_publication_authority_install_affordance_result_receipt_public_release_evidence",
            "blocked_public_release_artifact_evidence_noop",
            "release_publication_authority_install_affordance_result_receipt_public_release_evidence_denied",
        ),
        (
            "activation_live_install_restart_active_binary_result_receipt_live_evidence",
            "blocked_live_install_restart_active_binary_evidence_noop",
            "activation_live_install_restart_active_binary_result_receipt_live_evidence_denied",
        ),
    ];
    let audit_surface_false_keys = [
        "artifact_download_install_affordance_result_receipt_audit_trail_accepted",
        "artifact_download_install_affordance_result_receipt_audit_trail_recorded",
        "artifact_download_install_affordance_result_receipt_audit_trail_persisted",
        "artifact_download_install_affordance_result_receipt_audit_trail_materialized",
        "artifact_download_install_affordance_result_receipt_audit_trail_filesystem_written",
        "artifact_download_install_affordance_result_receipt_immutable_evidence_accepted",
        "artifact_download_install_affordance_result_receipt_immutable_evidence_recorded",
        "artifact_download_install_affordance_result_receipt_immutable_evidence_persisted",
        "artifact_download_install_affordance_result_receipt_immutable_evidence_materialized",
        "artifact_download_install_affordance_result_receipt_immutable_evidence_filesystem_written",
        "artifact_download_install_affordance_result_receipt_hash_chain_recorded",
        "artifact_download_install_affordance_result_receipt_hash_chain_persisted",
        "artifact_download_install_affordance_result_receipt_merkle_root_recorded",
        "artifact_download_install_affordance_result_receipt_merkle_root_persisted",
        "artifact_download_install_affordance_result_receipt_attestation_recorded",
        "artifact_download_install_affordance_result_receipt_attestation_persisted",
        "artifact_download_install_affordance_result_receipt_witness_recorded",
        "artifact_download_install_affordance_result_receipt_witness_persisted",
        "artifact_download_install_affordance_result_receipt_notary_recorded",
        "artifact_download_install_affordance_result_receipt_notary_persisted",
        "artifact_download_install_affordance_result_receipt_ledger_evidence_recorded",
        "artifact_download_install_affordance_result_receipt_index_evidence_recorded",
        "artifact_download_install_affordance_result_receipt_delivery_evidence_recorded",
        "artifact_download_install_affordance_result_receipt_export_evidence_recorded",
        "artifact_download_install_affordance_result_receipt_query_evidence_registered",
        "artifact_download_install_affordance_result_receipt_observability_evidence_recorded",
        "artifact_download_install_affordance_result_receipt_readback_evidence_recorded",
        "artifact_download_install_affordance_result_receipt_audit_evidence_acceptance_recorded",
        "artifact_download_install_affordance_result_receipt_completion_ack_from_audit_evidence_recorded",
        "artifact_download_install_affordance_result_receipt_cancellation_recorded",
        "artifact_download_install_affordance_result_receipt_supersession_recorded",
        "artifact_download_install_affordance_result_receipt_replacement_receipt_recorded",
        "artifact_download_install_affordance_result_receipt_tombstone_recorded",
        "artifact_download_install_affordance_result_receipt_delete_marker_recorded",
        "artifact_download_install_affordance_result_receipt_ordering_recorded",
        "artifact_download_install_affordance_result_receipt_monotonicity_state_recorded",
        "artifact_download_install_affordance_result_receipt_replay_recorded",
        "artifact_download_install_affordance_result_receipt_idempotency_state_recorded",
        "artifact_download_install_affordance_result_receipt_recorded",
        "artifact_download_install_affordance_result_receipt_persisted",
        "artifact_download_install_affordance_completion_ack_recorded",
        "download_button_rendered",
        "direct_download_url_exposed",
        "package_manager_install_command_rendered",
        "curl_pipe_shell_snippet_rendered",
        "installer_launch_prompt_rendered",
        "auto_update_offer_rendered",
        "external_install_message_sent",
        "telegram_install_message_sent",
        "operator_approval_from_audit_evidence_accepted",
        "release_publication_authority_from_audit_evidence_derived",
        "activation_authority_from_audit_evidence_derived",
        "activation_command_from_audit_evidence_derived",
        "activation_from_audit_evidence_allowed",
        "live_execution_from_audit_evidence_allowed",
        "install_from_audit_evidence_executed",
        "service_restart_from_audit_evidence_performed",
        "launchd_from_audit_evidence_mutated",
        "active_binary_from_audit_evidence_mutated",
        "activation_activated",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "telegram_send_performed",
        "channel_send_performed",
        "external_send_performed",
        "public_release_claimed",
        "public_ga_claimed",
        "release_artifact_written",
        "public_artifact_written",
    ];
    let audit_surfaces = audit_surface_specs
        .iter()
        .map(|(surface, status, reason)| {
            let mut surface_report = serde_json::json!({
                "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_evidence_surface": surface,
                "source_cancellation_supersession_present": true,
                "source_cancellation_supersession_ready": true,
                "source_cancellation_supersession_noop_confirmed": true,
                "audit_or_evidence_attempted": true,
                "artifact_download_install_affordance_result_receipt_audit_trail_requested": true,
                "artifact_download_install_affordance_result_receipt_immutable_evidence_requested": false,
                "artifact_download_install_affordance_result_receipt_audit_evidence_status": status,
                "audit_evidence_noop_confirmed": true,
                "reason": reason,
            });
            if let Some(surface_object) = surface_report.as_object_mut() {
                for key in audit_surface_false_keys {
                    surface_object.insert(key.to_string(), serde_json::json!(false));
                }
                match *surface {
                    "source_cancellation_supersession_report_required" => {
                        surface_object.insert(
                            "source_cancellation_supersession_report_required".to_string(),
                            serde_json::json!(true),
                        );
                    }
                    "download_button_result_receipt_audit_trail_append_claim" => {
                        surface_object.insert(
                            "audit_trail_append_requested".to_string(),
                            serde_json::json!(true),
                        );
                    }
                    "direct_download_url_result_receipt_immutable_evidence_packet" => {
                        surface_object.insert(
                            "artifact_download_install_affordance_result_receipt_immutable_evidence_requested".to_string(),
                            serde_json::json!(true),
                        );
                        surface_object.insert(
                            "artifact_download_install_affordance_result_receipt_audit_trail_requested".to_string(),
                            serde_json::json!(false),
                        );
                    }
                    "checksum_prompt_result_receipt_hash_chain_merkle_root" => {
                        for key in [
                            "artifact_download_install_affordance_result_receipt_immutable_evidence_requested",
                            "hash_chain_requested",
                            "merkle_root_requested",
                        ] {
                            surface_object.insert(key.to_string(), serde_json::json!(true));
                        }
                        surface_object.insert(
                            "artifact_download_install_affordance_result_receipt_audit_trail_requested".to_string(),
                            serde_json::json!(false),
                        );
                    }
                    "package_manager_install_command_result_receipt_attestation_witness_notary" => {
                        for key in [
                            "artifact_download_install_affordance_result_receipt_immutable_evidence_requested",
                            "attestation_requested",
                            "witness_requested",
                            "notary_requested",
                        ] {
                            surface_object.insert(key.to_string(), serde_json::json!(true));
                        }
                        surface_object.insert(
                            "artifact_download_install_affordance_result_receipt_audit_trail_requested".to_string(),
                            serde_json::json!(false),
                        );
                    }
                    "curl_pipe_shell_result_receipt_audit_materialization_filesystem" => {
                        for key in ["audit_materialization_requested", "audit_filesystem_write_requested"] {
                            surface_object.insert(key.to_string(), serde_json::json!(true));
                        }
                    }
                    "installer_launch_prompt_result_receipt_ledger_index_delivery_evidence" => {
                        for key in [
                            "ledger_evidence_requested",
                            "index_evidence_requested",
                            "delivery_evidence_requested",
                        ] {
                            surface_object.insert(key.to_string(), serde_json::json!(true));
                        }
                    }
                    "auto_update_offer_result_receipt_export_query_observability_evidence" => {
                        for key in [
                            "export_evidence_requested",
                            "query_evidence_requested",
                            "observability_evidence_requested",
                        ] {
                            surface_object.insert(key.to_string(), serde_json::json!(true));
                        }
                    }
                    "release_channel_subscription_result_receipt_readback_evidence" => {
                        surface_object.insert(
                            "readback_evidence_requested".to_string(),
                            serde_json::json!(true),
                        );
                    }
                    "update_feed_hint_result_receipt_completion_ack_audit_evidence" => {
                        surface_object.insert(
                            "completion_ack_audit_evidence_requested".to_string(),
                            serde_json::json!(true),
                        );
                    }
                    "package_registry_badge_result_receipt_cancellation_supersession_audit_evidence" => {
                        for key in [
                            "cancellation_audit_evidence_requested",
                            "supersession_audit_evidence_requested",
                        ] {
                            surface_object.insert(key.to_string(), serde_json::json!(true));
                        }
                    }
                    "cdn_mirror_download_result_receipt_ordering_monotonicity_audit_evidence" => {
                        for key in [
                            "ordering_audit_evidence_requested",
                            "monotonicity_audit_evidence_requested",
                        ] {
                            surface_object.insert(key.to_string(), serde_json::json!(true));
                        }
                    }
                    "sbom_provenance_notarization_result_receipt_replay_idempotency_audit_evidence" => {
                        for key in [
                            "replay_audit_evidence_requested",
                            "idempotency_audit_evidence_requested",
                        ] {
                            surface_object.insert(key.to_string(), serde_json::json!(true));
                        }
                    }
                    "signature_verification_command_result_receipt_release_publication_authority_evidence" => {
                        surface_object.insert(
                            "release_publication_authority_evidence_requested".to_string(),
                            serde_json::json!(true),
                        );
                    }
                    "one_click_install_deep_link_result_receipt_activation_authority_evidence" => {
                        surface_object.insert(
                            "activation_authority_evidence_requested".to_string(),
                            serde_json::json!(true),
                        );
                    }
                    "external_telegram_install_message_result_receipt_external_evidence" => {
                        for key in [
                            "external_audit_evidence_requested",
                            "telegram_audit_evidence_requested",
                        ] {
                            surface_object.insert(key.to_string(), serde_json::json!(true));
                        }
                    }
                    "release_publication_authority_install_affordance_result_receipt_public_release_evidence" => {
                        for key in [
                            "public_release_evidence_requested",
                            "release_artifact_evidence_requested",
                            "public_artifact_evidence_requested",
                        ] {
                            surface_object.insert(key.to_string(), serde_json::json!(true));
                        }
                    }
                    "activation_live_install_restart_active_binary_result_receipt_live_evidence" => {
                        for key in [
                            "activation_evidence_requested",
                            "install_evidence_requested",
                            "service_restart_evidence_requested",
                            "active_binary_evidence_requested",
                        ] {
                            surface_object.insert(key.to_string(), serde_json::json!(true));
                        }
                    }
                    _ => {}
                }
            }
            surface_report
        })
        .collect::<Vec<_>>();
    let audit_surface_count = audit_surfaces.len();
    let contract_hash = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-audit-trail-immutable-evidence-denial:native:source={source_report_sha256}:surfaces={audit_surface_count}:route_count={}:audit=0:evidence=0:hashchain=0:attestation=0:authority=0:install=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-audit-trail-immutable-evidence-denial:no-audit:no-immutable-evidence:no-hash-chain:no-attestation:no-authority:no-install:no-live",
    );
    let denials = vec![
        "artifact_download_install_affordance_result_receipt_audit_trail_acceptance_denied",
        "artifact_download_install_affordance_result_receipt_audit_trail_recording_denied",
        "artifact_download_install_affordance_result_receipt_audit_trail_persistence_denied",
        "artifact_download_install_affordance_result_receipt_audit_trail_materialization_denied",
        "artifact_download_install_affordance_result_receipt_immutable_evidence_acceptance_denied",
        "artifact_download_install_affordance_result_receipt_immutable_evidence_recording_denied",
        "artifact_download_install_affordance_result_receipt_immutable_evidence_persistence_denied",
        "artifact_download_install_affordance_result_receipt_immutable_evidence_materialization_denied",
        "artifact_download_install_affordance_result_receipt_hash_chain_recording_denied",
        "artifact_download_install_affordance_result_receipt_hash_chain_persistence_denied",
        "artifact_download_install_affordance_result_receipt_merkle_root_recording_denied",
        "artifact_download_install_affordance_result_receipt_merkle_root_persistence_denied",
        "artifact_download_install_affordance_result_receipt_attestation_recording_denied",
        "artifact_download_install_affordance_result_receipt_witness_recording_denied",
        "artifact_download_install_affordance_result_receipt_notary_recording_denied",
        "artifact_download_install_affordance_result_receipt_ledger_index_delivery_evidence_denied",
        "artifact_download_install_affordance_result_receipt_export_query_observability_evidence_denied",
        "artifact_download_install_affordance_result_receipt_readback_evidence_denied",
        "artifact_download_install_affordance_result_receipt_completion_ack_from_audit_evidence_denied",
        "artifact_download_install_affordance_result_receipt_cancellation_supersession_audit_evidence_denied",
        "artifact_download_install_affordance_result_receipt_ordering_monotonicity_audit_evidence_denied",
        "artifact_download_install_affordance_result_receipt_replay_idempotency_audit_evidence_denied",
        "artifact_download_install_affordance_result_receipt_acceptance_from_audit_evidence_denied",
        "artifact_download_install_affordance_release_publication_authority_from_audit_evidence_denied",
        "artifact_download_install_affordance_activation_authority_from_audit_evidence_denied",
        "artifact_download_install_affordance_external_send_from_audit_evidence_denied",
        "artifact_download_install_affordance_public_release_artifact_from_audit_evidence_denied",
        "artifact_download_install_affordance_install_restart_active_binary_from_audit_evidence_denied",
    ];
    let denied_count = denials.len();
    let report_ready = source_ready
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_surface_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_attempt_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_denied_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_accepted_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_supersession_accepted_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replacement_receipt_accepted_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_tombstone_recorded_count",
        ) == 0
        && audit_surface_count == 18
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_immutable_evidence_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_AUDIT_TRAIL_IMMUTABLE_EVIDENCE_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-audit-trail-immutable-evidence-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-19",
        "minimum_required_samples": 24,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "source_route_wired": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_immutable_evidence_denial_route_enabled": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_immutable_evidence_denial_ready": report_ready,
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
        "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_immutable_evidence_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_immutable_evidence_denial_v1",
        "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_immutable_evidence_mode": "native_route_denied_artifact_download_install_result_receipt_cannot_become_audit_trail_immutable_evidence_or_authority",
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_route": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_denial_route",
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_ready": source_ready,
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_report_sha256": source_report_sha256,
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_contract_hash_sha256": source_contract_hash,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_immutable_evidence_contract_hash_sha256": contract_hash,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_immutable_evidence_policy_hash_sha256": policy_hash,
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_surface_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_surface_count"),
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_attempt_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_attempt_count"),
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_denied_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_denied_count"),
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_accepted_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_accepted_count"),
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_supersession_accepted_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_supersession_accepted_count"),
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replacement_receipt_accepted_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replacement_receipt_accepted_count"),
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_tombstone_recorded_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_tombstone_recorded_count"),
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_evidence_surface_count": audit_surface_count,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_evidence_attempt_count": audit_surface_count,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_evidence_denied_count": audit_surface_count,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_evidence_surfaces": audit_surfaces,
        "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_immutable_evidence": denials,
        "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_immutable_evidence_count": denied_count,
        "allowed_next_actions": [
            {
                "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_denial_gate",
                "status": "allowed_report_only_next_slice",
                "records_audit_trail": false,
                "persists_immutable_evidence": false,
                "records_hash_chain": false,
                "records_ledger_evidence": false,
                "records_result_receipt": false,
                "records_completion_ack": false,
                "derives_release_publication_authority": false,
                "derives_activation_authority": false,
                "renders_download_link": false,
                "emits_install_command": false,
                "installs_or_restarts": false,
                "mutates_active_binary": false,
                "mutates_memory_store": false,
                "writes_kg": false,
                "sends_externally": false
            }
        ],
        }),
    );

    let audit_zero_keys = [
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_materialized_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_immutable_evidence_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_immutable_evidence_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_immutable_evidence_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_immutable_evidence_materialized_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_hash_chain_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_hash_chain_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_merkle_root_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_merkle_root_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_attestation_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_witness_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_notary_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ledger_evidence_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_index_evidence_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_delivery_evidence_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_evidence_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_query_evidence_registered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_observability_evidence_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_readback_evidence_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_completion_ack_from_audit_evidence_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_evidence_acceptance_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_audit_evidence_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_audit_evidence_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_audit_evidence_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_audit_evidence_service_restart_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_audit_evidence_active_binary_mutated_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_audit_evidence_memory_store_write_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_audit_evidence_live_kg_write_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_audit_evidence_provider_invoked_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_audit_evidence_model_invoked_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_audit_evidence_credential_read_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_audit_evidence_secret_read_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_audit_evidence_external_send_count",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in audit_zero_keys {
            report_object.insert(key.to_string(), serde_json::json!(0));
        }
    }

    let audit_false_keys = [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_immutable_evidence_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_immutable_evidence_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_immutable_evidence_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_hash_chain_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_merkle_root_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_attestation_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_witness_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_notary_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ledger_evidence_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_index_evidence_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_delivery_evidence_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_evidence_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_query_evidence_registered",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_observability_evidence_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_readback_evidence_recorded",
        "artifact_download_install_affordance_result_receipt_recorded",
        "artifact_download_install_affordance_result_receipt_persisted",
        "artifact_download_install_affordance_completion_ack_recorded",
        "operator_acceptance_recorded",
        "operator_approval_recorded",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "activation_command_derived",
        "activation_allowed",
        "activation_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
        "public_release_claimed",
        "public_ga_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "external_send_performed",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in audit_false_keys {
            report_object.insert(key.to_string(), serde_json::json!(false));
        }
    }

    let side_effect_false_keys = [
        "artifact_download_install_affordance_result_receipt_audit_trail_accepted",
        "artifact_download_install_affordance_result_receipt_audit_trail_recorded",
        "artifact_download_install_affordance_result_receipt_audit_trail_persisted",
        "artifact_download_install_affordance_result_receipt_audit_trail_materialized",
        "artifact_download_install_affordance_result_receipt_audit_trail_filesystem_written",
        "artifact_download_install_affordance_result_receipt_immutable_evidence_accepted",
        "artifact_download_install_affordance_result_receipt_immutable_evidence_recorded",
        "artifact_download_install_affordance_result_receipt_immutable_evidence_persisted",
        "artifact_download_install_affordance_result_receipt_immutable_evidence_materialized",
        "artifact_download_install_affordance_result_receipt_immutable_evidence_filesystem_written",
        "artifact_download_install_affordance_result_receipt_hash_chain_recorded",
        "artifact_download_install_affordance_result_receipt_hash_chain_persisted",
        "artifact_download_install_affordance_result_receipt_merkle_root_recorded",
        "artifact_download_install_affordance_result_receipt_attestation_recorded",
        "artifact_download_install_affordance_result_receipt_witness_recorded",
        "artifact_download_install_affordance_result_receipt_notary_recorded",
        "artifact_download_install_affordance_result_receipt_ledger_evidence_recorded",
        "artifact_download_install_affordance_result_receipt_index_evidence_recorded",
        "artifact_download_install_affordance_result_receipt_delivery_evidence_recorded",
        "artifact_download_install_affordance_result_receipt_export_evidence_recorded",
        "artifact_download_install_affordance_result_receipt_query_evidence_registered",
        "artifact_download_install_affordance_result_receipt_observability_evidence_recorded",
        "artifact_download_install_affordance_result_receipt_readback_evidence_recorded",
        "artifact_download_install_affordance_result_receipt_recorded",
        "artifact_download_install_affordance_result_receipt_persisted",
        "artifact_download_install_affordance_completion_ack_recorded",
        "operator_acceptance_recorded",
        "operator_approval_recorded",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "activation_command_derived",
        "activation_allowed",
        "activation_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
        "release_artifact_written",
        "public_artifact_written",
        "public_release_claimed",
        "public_ga_claimed",
        "telegram_send_performed",
        "channel_send_performed",
        "external_send_performed",
        "filesystem_written",
    ];
    let mut side_effects = serde_json::Map::new();
    for key in side_effect_false_keys {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_report =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_immutable_evidence_denial_report();
    let source_ready = source_report["status"].as_str() == Some("ready")
        && source_report["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_immutable_evidence_denial_ready"]
            .as_bool()
            .unwrap_or(false);
    let source_u64 = |key: &str| source_report[key].as_u64().unwrap_or(0);
    let source_report_sha256 = sha256_json_value(&source_report);
    let source_contract_hash = source_report
        ["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_immutable_evidence_contract_hash_sha256"]
        .as_str()
        .unwrap_or("unknown")
        .to_string();
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let retention_surface_specs = [
        (
            "source_audit_trail_immutable_evidence_report_required",
            "blocked_source_audit_evidence_report_required_noop",
            "source_audit_trail_immutable_evidence_report_required",
        ),
        (
            "download_button_result_receipt_retention_state_claim",
            "blocked_retention_state_noop",
            "download_button_result_receipt_retention_state_claim_denied",
        ),
        (
            "direct_download_url_result_receipt_expiry_state_claim",
            "blocked_expiry_state_noop",
            "direct_download_url_result_receipt_expiry_state_claim_denied",
        ),
        (
            "checksum_prompt_result_receipt_ttl_claim",
            "blocked_ttl_noop",
            "checksum_prompt_result_receipt_ttl_claim_denied",
        ),
        (
            "package_manager_install_command_result_receipt_lease_claim",
            "blocked_lease_noop",
            "package_manager_install_command_result_receipt_lease_claim_denied",
        ),
        (
            "curl_pipe_shell_result_receipt_gc_queue_claim",
            "blocked_gc_queue_noop",
            "curl_pipe_shell_result_receipt_gc_queue_claim_denied",
        ),
        (
            "installer_launch_prompt_result_receipt_tombstone_gc_claim",
            "blocked_tombstone_gc_noop",
            "installer_launch_prompt_result_receipt_tombstone_gc_claim_denied",
        ),
        (
            "auto_update_offer_result_receipt_delete_marker_gc_claim",
            "blocked_delete_marker_gc_noop",
            "auto_update_offer_result_receipt_delete_marker_gc_claim_denied",
        ),
        (
            "release_channel_subscription_result_receipt_retention_policy_claim",
            "blocked_retention_policy_noop",
            "release_channel_subscription_result_receipt_retention_policy_claim_denied",
        ),
        (
            "update_feed_hint_result_receipt_expiry_extension_claim",
            "blocked_expiry_extension_noop",
            "update_feed_hint_result_receipt_expiry_extension_claim_denied",
        ),
        (
            "package_registry_badge_result_receipt_audit_evidence_retention_claim",
            "blocked_audit_evidence_retention_noop",
            "package_registry_badge_result_receipt_audit_evidence_retention_claim_denied",
        ),
        (
            "cdn_mirror_download_result_receipt_ordering_replay_retention_claim",
            "blocked_ordering_replay_retention_noop",
            "cdn_mirror_download_result_receipt_ordering_replay_retention_claim_denied",
        ),
        (
            "sbom_provenance_notarization_result_receipt_hash_attestation_retention_claim",
            "blocked_hash_attestation_retention_noop",
            "sbom_provenance_notarization_result_receipt_hash_attestation_retention_claim_denied",
        ),
        (
            "signature_verification_command_result_receipt_completion_ack_retention_claim",
            "blocked_completion_ack_retention_noop",
            "signature_verification_command_result_receipt_completion_ack_retention_claim_denied",
        ),
        (
            "one_click_install_deep_link_result_receipt_activation_authority_retention_claim",
            "blocked_activation_authority_retention_noop",
            "one_click_install_deep_link_result_receipt_activation_authority_retention_claim_denied",
        ),
        (
            "external_telegram_install_message_result_receipt_external_gc_claim",
            "blocked_external_gc_noop",
            "external_telegram_install_message_result_receipt_external_gc_claim_denied",
        ),
        (
            "release_publication_authority_install_affordance_result_receipt_public_release_retention_claim",
            "blocked_public_release_retention_noop",
            "release_publication_authority_install_affordance_result_receipt_public_release_retention_claim_denied",
        ),
        (
            "activation_live_install_restart_active_binary_result_receipt_live_gc_claim",
            "blocked_live_gc_noop",
            "activation_live_install_restart_active_binary_result_receipt_live_gc_claim_denied",
        ),
    ];
    let surface_false_keys = [
        "artifact_download_install_affordance_result_receipt_retention_policy_accepted",
        "artifact_download_install_affordance_result_receipt_retention_policy_recorded",
        "artifact_download_install_affordance_result_receipt_retention_policy_persisted",
        "artifact_download_install_affordance_result_receipt_retention_policy_materialized",
        "artifact_download_install_affordance_result_receipt_retention_index_recorded",
        "artifact_download_install_affordance_result_receipt_retention_index_persisted",
        "artifact_download_install_affordance_result_receipt_retention_ledger_recorded",
        "artifact_download_install_affordance_result_receipt_retention_ledger_persisted",
        "artifact_download_install_affordance_result_receipt_ttl_update_accepted",
        "artifact_download_install_affordance_result_receipt_ttl_update_recorded",
        "artifact_download_install_affordance_result_receipt_ttl_update_persisted",
        "artifact_download_install_affordance_result_receipt_ttl_extension_accepted",
        "artifact_download_install_affordance_result_receipt_ttl_extension_recorded",
        "artifact_download_install_affordance_result_receipt_ttl_extension_persisted",
        "artifact_download_install_affordance_result_receipt_expiry_accepted",
        "artifact_download_install_affordance_result_receipt_expiry_recorded",
        "artifact_download_install_affordance_result_receipt_expiry_persisted",
        "artifact_download_install_affordance_result_receipt_expiry_scheduler_registered",
        "artifact_download_install_affordance_result_receipt_expiry_timer_started",
        "artifact_download_install_affordance_result_receipt_expiry_ack_recorded",
        "artifact_download_install_affordance_result_receipt_garbage_collection_accepted",
        "artifact_download_install_affordance_result_receipt_gc_queue_enqueued",
        "artifact_download_install_affordance_result_receipt_garbage_collection_scan_performed",
        "artifact_download_install_affordance_result_receipt_garbage_collection_candidate_recorded",
        "artifact_download_install_affordance_result_receipt_garbage_collection_decision_recorded",
        "artifact_download_install_affordance_result_receipt_garbage_collection_persisted",
        "artifact_download_install_affordance_result_receipt_delete_accepted",
        "artifact_download_install_affordance_result_receipt_delete_marker_recorded",
        "artifact_download_install_affordance_result_receipt_delete_performed",
        "artifact_download_install_affordance_result_receipt_tombstone_recorded",
        "artifact_download_install_affordance_result_receipt_tombstone_persisted",
        "artifact_download_install_affordance_result_receipt_sweep_performed",
        "artifact_download_install_affordance_result_receipt_archive_written",
        "artifact_download_install_affordance_result_receipt_archive_persisted",
        "artifact_download_install_affordance_result_receipt_compaction_performed",
        "artifact_download_install_affordance_result_receipt_compaction_artifact_written",
        "artifact_download_install_affordance_result_receipt_compaction_artifact_persisted",
        "artifact_download_install_affordance_result_receipt_audit_evidence_retention_recorded",
        "artifact_download_install_affordance_result_receipt_ordering_replay_retention_recorded",
        "artifact_download_install_affordance_result_receipt_hash_attestation_retention_recorded",
        "artifact_download_install_affordance_result_receipt_completion_ack_retention_recorded",
        "artifact_download_install_affordance_result_receipt_recorded",
        "artifact_download_install_affordance_result_receipt_persisted",
        "artifact_download_install_affordance_result_receipt_accepted",
        "artifact_download_install_affordance_result_receipt_materialized",
        "artifact_download_install_affordance_result_receipt_filesystem_written",
        "artifact_download_install_affordance_result_receipt_ledger_written",
        "artifact_download_install_affordance_result_receipt_indexed",
        "artifact_download_install_affordance_result_receipt_enqueued",
        "artifact_download_install_affordance_result_receipt_delivered",
        "artifact_download_install_affordance_result_receipt_exported",
        "artifact_download_install_affordance_result_receipt_query_registered",
        "artifact_download_install_affordance_result_receipt_observability_recorded",
        "artifact_download_install_affordance_completion_ack_recorded",
        "artifact_download_install_affordance_completion_ack_accepted",
        "download_button_rendered",
        "direct_download_url_exposed",
        "package_manager_install_command_rendered",
        "curl_pipe_shell_snippet_rendered",
        "installer_launch_prompt_rendered",
        "auto_update_offer_rendered",
        "external_install_message_sent",
        "telegram_install_message_sent",
        "operator_approval_from_retention_expiry_garbage_collection_accepted",
        "release_publication_authority_from_retention_expiry_garbage_collection_derived",
        "activation_authority_from_retention_expiry_garbage_collection_derived",
        "activation_command_from_retention_expiry_garbage_collection_derived",
        "activation_from_retention_expiry_garbage_collection_allowed",
        "live_execution_from_retention_expiry_garbage_collection_allowed",
        "install_from_retention_expiry_garbage_collection_executed",
        "service_restart_from_retention_expiry_garbage_collection_performed",
        "launchd_from_retention_expiry_garbage_collection_mutated",
        "active_binary_from_retention_expiry_garbage_collection_mutated",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "telegram_send_performed",
        "channel_send_performed",
        "external_send_performed",
        "public_release_claimed",
        "public_ga_claimed",
        "release_artifact_written",
        "public_artifact_written",
    ];
    let retention_surfaces: Vec<serde_json::Value> = retention_surface_specs
        .iter()
        .map(|(surface, status, reason)| {
            let mut surface_report = serde_json::json!({
                "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_surface": surface,
                "source_audit_trail_immutable_evidence_present": true,
                "source_audit_trail_immutable_evidence_ready": true,
                "source_audit_evidence_noop_confirmed": true,
                "artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_attempted": true,
                "artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_status": status,
                "retention_expiry_garbage_collection_noop_confirmed": true,
                "reason": reason,
            });
            if let Some(surface_object) = surface_report.as_object_mut() {
                for key in surface_false_keys.iter() {
                    surface_object.insert((*key).to_string(), serde_json::json!(false));
                }
                match *surface {
                    "source_audit_trail_immutable_evidence_report_required" => {
                        surface_object.insert(
                            "source_audit_trail_immutable_evidence_report_required".to_string(),
                            serde_json::json!(true),
                        );
                    }
                    "download_button_result_receipt_retention_state_claim"
                    | "release_channel_subscription_result_receipt_retention_policy_claim" => {
                        surface_object.insert(
                            "artifact_download_install_affordance_result_receipt_retention_policy_requested".to_string(),
                            serde_json::json!(true),
                        );
                    }
                    "direct_download_url_result_receipt_expiry_state_claim" => {
                        surface_object.insert(
                            "artifact_download_install_affordance_result_receipt_expiry_requested".to_string(),
                            serde_json::json!(true),
                        );
                    }
                    "checksum_prompt_result_receipt_ttl_claim" => {
                        surface_object.insert(
                            "artifact_download_install_affordance_result_receipt_ttl_requested".to_string(),
                            serde_json::json!(true),
                        );
                    }
                    "package_manager_install_command_result_receipt_lease_claim" => {
                        surface_object.insert(
                            "artifact_download_install_affordance_result_receipt_ttl_requested"
                                .to_string(),
                            serde_json::json!(true),
                        );
                        surface_object
                            .insert("retention_lease_requested".to_string(), serde_json::json!(true));
                    }
                    "curl_pipe_shell_result_receipt_gc_queue_claim" => {
                        surface_object.insert(
                            "artifact_download_install_affordance_result_receipt_garbage_collection_requested".to_string(),
                            serde_json::json!(true),
                        );
                        surface_object.insert("gc_queue_requested".to_string(), serde_json::json!(true));
                    }
                    "installer_launch_prompt_result_receipt_tombstone_gc_claim" => {
                        surface_object.insert(
                            "artifact_download_install_affordance_result_receipt_tombstone_requested".to_string(),
                            serde_json::json!(true),
                        );
                        surface_object.insert(
                            "artifact_download_install_affordance_result_receipt_garbage_collection_requested".to_string(),
                            serde_json::json!(true),
                        );
                    }
                    "auto_update_offer_result_receipt_delete_marker_gc_claim" => {
                        surface_object.insert(
                            "artifact_download_install_affordance_result_receipt_delete_requested".to_string(),
                            serde_json::json!(true),
                        );
                        surface_object.insert(
                            "artifact_download_install_affordance_result_receipt_garbage_collection_requested".to_string(),
                            serde_json::json!(true),
                        );
                    }
                    "update_feed_hint_result_receipt_expiry_extension_claim" => {
                        surface_object.insert(
                            "artifact_download_install_affordance_result_receipt_expiry_requested".to_string(),
                            serde_json::json!(true),
                        );
                        surface_object
                            .insert("expiry_extension_requested".to_string(), serde_json::json!(true));
                    }
                    "package_registry_badge_result_receipt_audit_evidence_retention_claim" => {
                        surface_object.insert(
                            "audit_evidence_retention_requested".to_string(),
                            serde_json::json!(true),
                        );
                    }
                    "cdn_mirror_download_result_receipt_ordering_replay_retention_claim" => {
                        surface_object.insert(
                            "ordering_replay_retention_requested".to_string(),
                            serde_json::json!(true),
                        );
                    }
                    "sbom_provenance_notarization_result_receipt_hash_attestation_retention_claim" => {
                        surface_object.insert(
                            "hash_attestation_retention_requested".to_string(),
                            serde_json::json!(true),
                        );
                    }
                    "signature_verification_command_result_receipt_completion_ack_retention_claim" => {
                        surface_object.insert(
                            "completion_ack_retention_requested".to_string(),
                            serde_json::json!(true),
                        );
                    }
                    "one_click_install_deep_link_result_receipt_activation_authority_retention_claim" => {
                        surface_object.insert(
                            "activation_authority_retention_requested".to_string(),
                            serde_json::json!(true),
                        );
                    }
                    "external_telegram_install_message_result_receipt_external_gc_claim" => {
                        surface_object.insert(
                            "external_gc_requested".to_string(),
                            serde_json::json!(true),
                        );
                        surface_object.insert(
                            "telegram_gc_requested".to_string(),
                            serde_json::json!(true),
                        );
                    }
                    "release_publication_authority_install_affordance_result_receipt_public_release_retention_claim" => {
                        surface_object.insert(
                            "public_release_retention_requested".to_string(),
                            serde_json::json!(true),
                        );
                        surface_object.insert(
                            "release_artifact_retention_requested".to_string(),
                            serde_json::json!(true),
                        );
                        surface_object.insert(
                            "public_artifact_retention_requested".to_string(),
                            serde_json::json!(true),
                        );
                    }
                    "activation_live_install_restart_active_binary_result_receipt_live_gc_claim" => {
                        surface_object.insert(
                            "activation_retention_requested".to_string(),
                            serde_json::json!(true),
                        );
                        surface_object
                            .insert("install_gc_requested".to_string(), serde_json::json!(true));
                        surface_object.insert(
                            "service_restart_gc_requested".to_string(),
                            serde_json::json!(true),
                        );
                        surface_object.insert(
                            "active_binary_gc_requested".to_string(),
                            serde_json::json!(true),
                        );
                    }
                    _ => {}
                }
            }
            surface_report
        })
        .collect();
    let retention_surface_count = retention_surfaces.len();
    let denials = serde_json::json!([
        "artifact_download_install_affordance_result_receipt_retention_policy_acceptance_denied",
        "artifact_download_install_affordance_result_receipt_retention_policy_recording_denied",
        "artifact_download_install_affordance_result_receipt_retention_policy_persistence_denied",
        "artifact_download_install_affordance_result_receipt_retention_policy_materialization_denied",
        "artifact_download_install_affordance_result_receipt_retention_index_recording_denied",
        "artifact_download_install_affordance_result_receipt_retention_ledger_recording_denied",
        "artifact_download_install_affordance_result_receipt_ttl_update_denied",
        "artifact_download_install_affordance_result_receipt_ttl_extension_denied",
        "artifact_download_install_affordance_result_receipt_expiry_acceptance_denied",
        "artifact_download_install_affordance_result_receipt_expiry_recording_denied",
        "artifact_download_install_affordance_result_receipt_expiry_scheduler_denied",
        "artifact_download_install_affordance_result_receipt_expiry_timer_denied",
        "artifact_download_install_affordance_result_receipt_expiry_ack_denied",
        "artifact_download_install_affordance_result_receipt_garbage_collection_acceptance_denied",
        "artifact_download_install_affordance_result_receipt_gc_queue_denied",
        "artifact_download_install_affordance_result_receipt_garbage_collection_scan_denied",
        "artifact_download_install_affordance_result_receipt_garbage_collection_candidate_denied",
        "artifact_download_install_affordance_result_receipt_garbage_collection_decision_denied",
        "artifact_download_install_affordance_result_receipt_delete_denied",
        "artifact_download_install_affordance_result_receipt_tombstone_denied",
        "artifact_download_install_affordance_result_receipt_sweep_denied",
        "artifact_download_install_affordance_result_receipt_archive_denied",
        "artifact_download_install_affordance_result_receipt_compaction_denied",
        "artifact_download_install_affordance_result_receipt_audit_evidence_retention_denied",
        "artifact_download_install_affordance_result_receipt_ordering_replay_retention_denied",
        "artifact_download_install_affordance_result_receipt_hash_attestation_retention_denied",
        "artifact_download_install_affordance_result_receipt_completion_ack_from_retention_denied",
        "artifact_download_install_affordance_result_receipt_record_from_retention_denied",
        "artifact_download_install_affordance_release_publication_authority_from_retention_denied",
        "artifact_download_install_affordance_activation_authority_from_retention_denied",
        "artifact_download_install_affordance_download_install_affordance_from_retention_denied",
        "artifact_download_install_affordance_install_restart_active_binary_from_retention_denied",
        "artifact_download_install_affordance_memory_provider_secret_external_send_from_retention_denied"
    ]);
    let denied_count = denials.as_array().map_or(0, Vec::len);
    let contract_hash = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-retention-expiry-garbage-collection-denial:native:source={source_report_sha256}:surfaces={retention_surface_count}:route_count={NATIVE_GATEWAY_SOURCE_COMMAND_COUNT}:retention=0:expiry=0:gc=0:authority=0:live=0"
    ));
    let policy_hash = sha256_text_value(
        "artifact-download-install-affordance-result-receipt-retention-expiry-garbage-collection-denial:no-retention:no-expiry:no-gc:no-receipt:no-authority:no-install:no-live",
    );
    let report_ready = source_ready
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_evidence_surface_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_evidence_denied_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_recorded_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_immutable_evidence_recorded_count",
        ) == 0
        && retention_surface_count == 18
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_RETENTION_EXPIRY_GARBAGE_COLLECTION_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-retention-expiry-garbage-collection-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-19",
        "minimum_required_samples": 24,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "source_route_wired": true,
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_denial_route_enabled": true,
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_denial_ready": report_ready,
            "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_denial_v1",
            "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_mode": "native_route_denied_artifact_download_install_result_receipt_cannot_create_retention_expiry_garbage_collection_state_or_authority",
            "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_immutable_evidence_route": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_immutable_evidence_denial_route",
            "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_immutable_evidence_ready": source_ready,
            "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_immutable_evidence_report_sha256": source_report_sha256,
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_immutable_evidence_contract_hash_sha256": source_contract_hash,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_contract_hash_sha256": contract_hash,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_policy_hash_sha256": policy_hash,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_evidence_surface_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_evidence_surface_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_evidence_denied_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_evidence_denied_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_recorded_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_recorded_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_immutable_evidence_recorded_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_immutable_evidence_recorded_count"),
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_surface_count": retention_surface_count,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_attempt_count": retention_surface_count,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_denied_count": retention_surface_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_surfaces": retention_surfaces,
            "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection": denials,
            "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_count": denied_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [serde_json::json!({
                "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_denial_gate",
                "status": "allowed_report_only_next_slice",
                "records_retention_policy": false,
                "records_expiry": false,
                "performs_garbage_collection": false,
                "exports_receipt": false,
                "registers_query": false,
                "records_observability": false,
                "renders_download_link": false,
                "emits_install_command": false,
                "installs_or_restarts": false,
                "mutates_active_binary": false,
                "mutates_memory_store": false,
                "writes_kg": false,
                "sends_externally": false,
            })],
        }),
    );

    let retention_zero_keys = [
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_policy_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_policy_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_policy_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_policy_materialized_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_index_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_index_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_ledger_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_ledger_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ttl_update_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ttl_update_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ttl_update_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ttl_extension_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ttl_extension_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ttl_extension_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_expiry_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_expiry_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_expiry_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_expiry_scheduler_registered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_expiry_timer_started_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_expiry_ack_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_garbage_collection_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_gc_queue_enqueued_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_garbage_collection_scan_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_garbage_collection_candidate_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_garbage_collection_decision_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_garbage_collection_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_delete_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_delete_marker_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_delete_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_tombstone_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_tombstone_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_sweep_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_archive_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_archive_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_compaction_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_compaction_artifact_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_compaction_artifact_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_evidence_retention_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_replay_retention_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_hash_attestation_retention_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_completion_ack_retention_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_acceptance_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_retention_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_retention_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_retention_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_retention_service_restart_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_retention_active_binary_mutated_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_retention_memory_store_write_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_retention_live_kg_write_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_retention_provider_invoked_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_retention_model_invoked_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_retention_credential_read_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_retention_secret_read_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_retention_external_send_count",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in retention_zero_keys {
            report_object.insert(key.to_string(), serde_json::json!(0));
        }
    }

    let retention_false_keys = [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_policy_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ttl_update_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_expiry_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_garbage_collection_scan_performed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_delete_marker_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_tombstone_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_archive_written",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_compaction_performed",
        "artifact_download_install_affordance_result_receipt_recorded",
        "artifact_download_install_affordance_result_receipt_persisted",
        "artifact_download_install_affordance_completion_ack_recorded",
        "download_button_rendered",
        "direct_download_url_exposed",
        "package_manager_install_command_rendered",
        "curl_pipe_shell_snippet_rendered",
        "installer_launch_prompt_rendered",
        "auto_update_offer_rendered",
        "external_install_message_sent",
        "telegram_install_message_sent",
        "operator_acceptance_recorded",
        "operator_approval_recorded",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "activation_allowed",
        "activation_performed",
        "download_link_rendered",
        "install_command_rendered",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
        "release_artifact_written",
        "public_artifact_written",
        "external_send_performed",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in retention_false_keys {
            report_object.insert(key.to_string(), serde_json::json!(false));
        }
    }

    let mut side_effects = serde_json::Map::new();
    for key in surface_false_keys {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    side_effects.insert("filesystem_written".to_string(), serde_json::json!(false));
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_denial_report();
    let source_report_sha256 = sha256_json_value(&source);
    let source_ready = source
        .get("memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_denial_ready")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let source_u64 = |key: &str| -> u64 {
        source
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let surface_false_keys = [
        "artifact_download_install_affordance_result_receipt_query_registered",
        "artifact_download_install_affordance_result_receipt_query_executed",
        "artifact_download_install_affordance_result_receipt_query_result_recorded",
        "artifact_download_install_affordance_result_receipt_query_result_persisted",
        "artifact_download_install_affordance_result_receipt_search_index_recorded",
        "artifact_download_install_affordance_result_receipt_search_index_persisted",
        "artifact_download_install_affordance_result_receipt_export_accepted",
        "artifact_download_install_affordance_result_receipt_export_snapshot_recorded",
        "artifact_download_install_affordance_result_receipt_export_snapshot_persisted",
        "artifact_download_install_affordance_result_receipt_export_file_written",
        "artifact_download_install_affordance_result_receipt_export_stream_opened",
        "artifact_download_install_affordance_result_receipt_observability_metric_recorded",
        "artifact_download_install_affordance_result_receipt_observability_log_recorded",
        "artifact_download_install_affordance_result_receipt_observability_trace_recorded",
        "artifact_download_install_affordance_result_receipt_observability_event_recorded",
        "artifact_download_install_affordance_result_receipt_dashboard_panel_recorded",
        "artifact_download_install_affordance_result_receipt_alert_registered",
        "artifact_download_install_affordance_result_receipt_slo_recorded",
        "artifact_download_install_affordance_result_receipt_operator_summary_recorded",
        "artifact_download_install_affordance_result_receipt_readback_surface_recorded",
        "artifact_download_install_affordance_result_receipt_audit_view_recorded",
        "artifact_download_install_affordance_result_receipt_ledger_observability_recorded",
        "artifact_download_install_affordance_result_receipt_index_observability_recorded",
        "artifact_download_install_affordance_result_receipt_delivery_observability_recorded",
        "artifact_download_install_affordance_result_receipt_retention_policy_recorded",
        "artifact_download_install_affordance_result_receipt_expiry_recorded",
        "artifact_download_install_affordance_result_receipt_garbage_collection_scan_performed",
        "artifact_download_install_affordance_result_receipt_audit_trail_recorded",
        "artifact_download_install_affordance_result_receipt_immutable_evidence_recorded",
        "artifact_download_install_affordance_result_receipt_hash_chain_recorded",
        "artifact_download_install_affordance_result_receipt_completion_ack_recorded",
        "artifact_download_install_affordance_result_receipt_recorded",
        "artifact_download_install_affordance_result_receipt_persisted",
        "download_button_rendered",
        "direct_download_url_exposed",
        "package_manager_install_command_rendered",
        "curl_pipe_shell_snippet_rendered",
        "installer_launch_prompt_rendered",
        "auto_update_offer_rendered",
        "external_install_message_sent",
        "telegram_install_message_sent",
        "operator_approval_from_export_query_observability_accepted",
        "release_publication_authority_from_export_query_observability_derived",
        "activation_authority_from_export_query_observability_derived",
        "activation_command_from_export_query_observability_derived",
        "activation_from_export_query_observability_allowed",
        "live_execution_from_export_query_observability_allowed",
        "install_from_export_query_observability_executed",
        "service_restart_from_export_query_observability_performed",
        "launchd_from_export_query_observability_mutated",
        "active_binary_from_export_query_observability_mutated",
        "activation_activated",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "telegram_send_performed",
        "channel_send_performed",
        "external_send_performed",
        "public_release_claimed",
        "public_ga_claimed",
        "release_artifact_written",
        "public_artifact_written",
    ];
    let export_query_surface_specs = [
        (
            "source_retention_expiry_garbage_collection_report_required",
            "blocked_source_retention_report_required_noop",
            "source_retention_expiry_garbage_collection_report_required",
        ),
        (
            "download_button_result_receipt_query_registration_claim",
            "blocked_query_registration_noop",
            "download_button_result_receipt_query_registration_claim_denied",
        ),
        (
            "direct_download_url_result_receipt_query_execution_claim",
            "blocked_query_execution_noop",
            "direct_download_url_result_receipt_query_execution_claim_denied",
        ),
        (
            "checksum_prompt_result_receipt_query_result_claim",
            "blocked_query_result_noop",
            "checksum_prompt_result_receipt_query_result_claim_denied",
        ),
        (
            "package_manager_install_command_result_receipt_search_index_claim",
            "blocked_search_index_noop",
            "package_manager_install_command_result_receipt_search_index_claim_denied",
        ),
        (
            "curl_pipe_shell_result_receipt_export_request_claim",
            "blocked_export_request_noop",
            "curl_pipe_shell_result_receipt_export_request_claim_denied",
        ),
        (
            "installer_launch_prompt_result_receipt_export_snapshot_claim",
            "blocked_export_snapshot_noop",
            "installer_launch_prompt_result_receipt_export_snapshot_claim_denied",
        ),
        (
            "auto_update_offer_result_receipt_export_file_claim",
            "blocked_export_file_noop",
            "auto_update_offer_result_receipt_export_file_claim_denied",
        ),
        (
            "release_channel_subscription_result_receipt_export_stream_claim",
            "blocked_export_stream_noop",
            "release_channel_subscription_result_receipt_export_stream_claim_denied",
        ),
        (
            "update_feed_hint_result_receipt_observability_metric_log_claim",
            "blocked_metric_log_noop",
            "update_feed_hint_result_receipt_observability_metric_log_claim_denied",
        ),
        (
            "package_registry_badge_result_receipt_observability_trace_event_claim",
            "blocked_trace_event_noop",
            "package_registry_badge_result_receipt_observability_trace_event_claim_denied",
        ),
        (
            "cdn_mirror_download_result_receipt_dashboard_panel_claim",
            "blocked_dashboard_panel_noop",
            "cdn_mirror_download_result_receipt_dashboard_panel_claim_denied",
        ),
        (
            "sbom_provenance_notarization_result_receipt_alert_slo_claim",
            "blocked_alert_slo_noop",
            "sbom_provenance_notarization_result_receipt_alert_slo_claim_denied",
        ),
        (
            "signature_verification_command_result_receipt_operator_summary_readback_claim",
            "blocked_operator_summary_readback_noop",
            "signature_verification_command_result_receipt_operator_summary_readback_claim_denied",
        ),
        (
            "one_click_install_deep_link_result_receipt_audit_view_claim",
            "blocked_audit_view_noop",
            "one_click_install_deep_link_result_receipt_audit_view_claim_denied",
        ),
        (
            "external_telegram_install_message_result_receipt_external_observability_claim",
            "blocked_external_observability_noop",
            "external_telegram_install_message_result_receipt_external_observability_claim_denied",
        ),
        (
            "release_publication_authority_install_affordance_result_receipt_authority_view_claim",
            "blocked_authority_view_noop",
            "release_publication_authority_install_affordance_result_receipt_authority_view_claim_denied",
        ),
        (
            "activation_live_install_restart_active_binary_result_receipt_live_view_claim",
            "blocked_live_view_noop",
            "activation_live_install_restart_active_binary_result_receipt_live_view_claim_denied",
        ),
    ];
    let export_query_surfaces = export_query_surface_specs
        .iter()
        .map(|(surface, status, reason)| {
            let mut surface_report = serde_json::json!({
                "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_surface": surface,
                "source_retention_expiry_garbage_collection_present": true,
                "source_retention_expiry_garbage_collection_ready": source_ready,
                "source_retention_expiry_garbage_collection_noop_confirmed": true,
                "artifact_download_install_affordance_result_receipt_export_query_observability_attempted": true,
                "artifact_download_install_affordance_result_receipt_query_requested": false,
                "artifact_download_install_affordance_result_receipt_export_requested": false,
                "artifact_download_install_affordance_result_receipt_observability_requested": false,
                "export_query_observability_noop_confirmed": true,
                "artifact_download_install_affordance_result_receipt_export_query_observability_status": status,
                "reason": reason,
            });
            if let Some(surface_object) = surface_report.as_object_mut() {
                for key in surface_false_keys {
                    surface_object.insert(key.to_string(), serde_json::json!(false));
                }
                match *surface {
                    "source_retention_expiry_garbage_collection_report_required" => {
                        surface_object.insert(
                            "source_retention_expiry_garbage_collection_report_required"
                                .to_string(),
                            serde_json::json!(true),
                        );
                    }
                    "download_button_result_receipt_query_registration_claim"
                    | "direct_download_url_result_receipt_query_execution_claim"
                    | "checksum_prompt_result_receipt_query_result_claim" => {
                        surface_object.insert(
                            "artifact_download_install_affordance_result_receipt_query_requested"
                                .to_string(),
                            serde_json::json!(true),
                        );
                    }
                    "package_manager_install_command_result_receipt_search_index_claim" => {
                        surface_object
                            .insert("search_index_requested".to_string(), serde_json::json!(true));
                    }
                    "curl_pipe_shell_result_receipt_export_request_claim" => {
                        surface_object.insert(
                            "artifact_download_install_affordance_result_receipt_export_requested"
                                .to_string(),
                            serde_json::json!(true),
                        );
                    }
                    "installer_launch_prompt_result_receipt_export_snapshot_claim" => {
                        surface_object.insert(
                            "artifact_download_install_affordance_result_receipt_export_requested"
                                .to_string(),
                            serde_json::json!(true),
                        );
                        surface_object
                            .insert("export_snapshot_requested".to_string(), serde_json::json!(true));
                    }
                    "auto_update_offer_result_receipt_export_file_claim" => {
                        surface_object.insert(
                            "artifact_download_install_affordance_result_receipt_export_requested"
                                .to_string(),
                            serde_json::json!(true),
                        );
                        surface_object
                            .insert("export_file_requested".to_string(), serde_json::json!(true));
                    }
                    "release_channel_subscription_result_receipt_export_stream_claim" => {
                        surface_object.insert(
                            "artifact_download_install_affordance_result_receipt_export_requested"
                                .to_string(),
                            serde_json::json!(true),
                        );
                        surface_object
                            .insert("export_stream_requested".to_string(), serde_json::json!(true));
                    }
                    "update_feed_hint_result_receipt_observability_metric_log_claim" => {
                        for key in [
                            "artifact_download_install_affordance_result_receipt_observability_requested",
                            "metric_observability_requested",
                            "log_observability_requested",
                        ] {
                            surface_object.insert(key.to_string(), serde_json::json!(true));
                        }
                    }
                    "package_registry_badge_result_receipt_observability_trace_event_claim" => {
                        for key in [
                            "artifact_download_install_affordance_result_receipt_observability_requested",
                            "trace_observability_requested",
                            "event_observability_requested",
                        ] {
                            surface_object.insert(key.to_string(), serde_json::json!(true));
                        }
                    }
                    "cdn_mirror_download_result_receipt_dashboard_panel_claim" => {
                        surface_object.insert(
                            "dashboard_panel_requested".to_string(),
                            serde_json::json!(true),
                        );
                    }
                    "sbom_provenance_notarization_result_receipt_alert_slo_claim" => {
                        surface_object
                            .insert("alert_slo_requested".to_string(), serde_json::json!(true));
                    }
                    "signature_verification_command_result_receipt_operator_summary_readback_claim" => {
                        surface_object.insert(
                            "operator_summary_readback_requested".to_string(),
                            serde_json::json!(true),
                        );
                    }
                    "one_click_install_deep_link_result_receipt_audit_view_claim" => {
                        surface_object
                            .insert("audit_view_requested".to_string(), serde_json::json!(true));
                    }
                    "external_telegram_install_message_result_receipt_external_observability_claim" => {
                        for key in [
                            "external_observability_requested",
                            "telegram_observability_requested",
                        ] {
                            surface_object.insert(key.to_string(), serde_json::json!(true));
                        }
                    }
                    "release_publication_authority_install_affordance_result_receipt_authority_view_claim" => {
                        surface_object
                            .insert("authority_view_requested".to_string(), serde_json::json!(true));
                    }
                    "activation_live_install_restart_active_binary_result_receipt_live_view_claim" => {
                        for key in [
                            "live_view_requested",
                            "install_view_requested",
                            "service_restart_view_requested",
                            "active_binary_view_requested",
                        ] {
                            surface_object.insert(key.to_string(), serde_json::json!(true));
                        }
                    }
                    _ => {}
                }
            }
            surface_report
        })
        .collect::<Vec<_>>();
    let export_query_surface_count = export_query_surfaces.len();
    let contract_hash = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-export-query-observability-denial:native:source={source_report_sha256}:surfaces={export_query_surface_count}:route_count={}:query=0:export=0:observability=0:authority=0:install=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-export-query-observability-denial:no-query:no-export:no-observability:no-readback:no-authority:no-install:no-live",
    );
    let denials = vec![
        "artifact_download_install_affordance_result_receipt_query_registration_denied",
        "artifact_download_install_affordance_result_receipt_query_execution_denied",
        "artifact_download_install_affordance_result_receipt_query_result_recording_denied",
        "artifact_download_install_affordance_result_receipt_query_result_persistence_denied",
        "artifact_download_install_affordance_result_receipt_search_index_recording_denied",
        "artifact_download_install_affordance_result_receipt_search_index_persistence_denied",
        "artifact_download_install_affordance_result_receipt_export_request_denied",
        "artifact_download_install_affordance_result_receipt_export_acceptance_denied",
        "artifact_download_install_affordance_result_receipt_export_snapshot_recording_denied",
        "artifact_download_install_affordance_result_receipt_export_snapshot_persistence_denied",
        "artifact_download_install_affordance_result_receipt_export_file_write_denied",
        "artifact_download_install_affordance_result_receipt_export_stream_open_denied",
        "artifact_download_install_affordance_result_receipt_observability_metric_denied",
        "artifact_download_install_affordance_result_receipt_observability_log_denied",
        "artifact_download_install_affordance_result_receipt_observability_trace_denied",
        "artifact_download_install_affordance_result_receipt_observability_event_denied",
        "artifact_download_install_affordance_result_receipt_dashboard_panel_denied",
        "artifact_download_install_affordance_result_receipt_alert_slo_denied",
        "artifact_download_install_affordance_result_receipt_operator_summary_denied",
        "artifact_download_install_affordance_result_receipt_readback_surface_denied",
        "artifact_download_install_affordance_result_receipt_audit_view_denied",
        "artifact_download_install_affordance_result_receipt_ledger_index_delivery_observability_denied",
        "artifact_download_install_affordance_result_receipt_completion_ack_from_view_denied",
        "artifact_download_install_affordance_result_receipt_acceptance_from_view_denied",
        "artifact_download_install_affordance_release_publication_authority_from_view_denied",
        "artifact_download_install_affordance_activation_authority_from_view_denied",
        "artifact_download_install_affordance_download_install_affordance_from_view_denied",
        "artifact_download_install_affordance_install_restart_active_binary_from_view_denied",
        "artifact_download_install_affordance_memory_provider_secret_external_send_from_view_denied",
    ];
    let denied_count = denials.len();
    let report_ready = source_ready
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_surface_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_attempt_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_denied_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_policy_recorded_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_expiry_recorded_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_garbage_collection_scan_performed_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_retention_release_publication_authority_derived_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_retention_activation_authority_derived_count",
        ) == 0
        && export_query_surface_count == 18
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_EXPORT_QUERY_OBSERVABILITY_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-export-query-observability-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-19",
        "minimum_required_samples": 24,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "source_route_wired": true,
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_denial_route_enabled": true,
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_denial_ready": report_ready,
            "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_denial_v1",
            "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_mode": "native_route_denied_artifact_download_install_result_receipt_cannot_create_export_query_observability_view_or_authority",
            "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_route": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_denial_route",
            "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_ready": source_ready,
            "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_report_sha256": source_report_sha256,
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_contract_hash_sha256": source["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_contract_hash_sha256"].clone(),
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_contract_hash_sha256": contract_hash,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_policy_hash_sha256": policy_hash,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_surface_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_surface_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_attempt_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_attempt_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_denied_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_denied_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_policy_recorded_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_policy_recorded_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_expiry_recorded_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_expiry_recorded_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_garbage_collection_scan_performed_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_garbage_collection_scan_performed_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_retention_release_publication_authority_derived_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_retention_release_publication_authority_derived_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_retention_activation_authority_derived_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_retention_activation_authority_derived_count"),
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_surface_count": export_query_surface_count,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_attempt_count": export_query_surface_count,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_denied_count": export_query_surface_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_surfaces": export_query_surfaces,
            "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability": denials,
            "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_count": denied_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [serde_json::json!({
                "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_non_persistence_denial_gate",
                "status": "allowed_report_only_next_slice",
                "exports_receipt": false,
                "registers_query": false,
                "records_observability": false,
                "records_summary": false,
                "records_briefing": false,
                "renders_download_link": false,
                "emits_install_command": false,
                "installs_or_restarts": false,
                "mutates_active_binary": false,
                "mutates_memory_store": false,
                "writes_kg": false,
                "sends_externally": false,
            })],
        }),
    );

    let zero_keys = [
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_query_registered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_query_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_query_result_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_query_result_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_search_index_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_search_index_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_snapshot_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_snapshot_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_file_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_stream_opened_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_observability_metric_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_observability_log_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_observability_trace_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_observability_event_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_dashboard_panel_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_alert_registered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_slo_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_summary_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_readback_surface_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_view_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ledger_observability_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_index_observability_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_delivery_observability_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_acceptance_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_export_query_observability_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_export_query_observability_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_export_query_observability_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_export_query_observability_service_restart_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_export_query_observability_active_binary_mutated_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_export_query_observability_memory_store_write_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_export_query_observability_live_kg_write_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_export_query_observability_provider_invoked_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_export_query_observability_model_invoked_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_export_query_observability_credential_read_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_export_query_observability_secret_read_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_export_query_observability_external_send_count",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in zero_keys {
            report_object.insert(key.to_string(), serde_json::json!(0));
        }
    }

    let false_keys = [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_query_registered",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_query_executed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_query_result_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_search_index_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_snapshot_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_file_written",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_stream_opened",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_observability_metric_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_observability_log_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_observability_trace_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_observability_event_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_dashboard_panel_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_summary_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_readback_surface_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_view_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ledger_observability_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_index_observability_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_delivery_observability_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_policy_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_expiry_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_garbage_collection_scan_performed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_immutable_evidence_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_hash_chain_recorded",
        "artifact_download_install_affordance_result_receipt_recorded",
        "artifact_download_install_affordance_result_receipt_persisted",
        "artifact_download_install_affordance_result_receipt_exported",
        "artifact_download_install_affordance_result_receipt_query_registered",
        "artifact_download_install_affordance_result_receipt_observability_recorded",
        "artifact_download_install_affordance_completion_ack_recorded",
        "download_button_rendered",
        "direct_download_url_exposed",
        "package_manager_install_command_rendered",
        "curl_pipe_shell_snippet_rendered",
        "installer_launch_prompt_rendered",
        "auto_update_offer_rendered",
        "external_install_message_sent",
        "telegram_install_message_sent",
        "operator_acceptance_recorded",
        "operator_approval_recorded",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "activation_allowed",
        "activation_performed",
        "download_link_rendered",
        "install_command_rendered",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
        "public_release_claimed",
        "public_ga_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "external_send_performed",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in false_keys {
            report_object.insert(key.to_string(), serde_json::json!(false));
        }
    }

    let mut side_effects = serde_json::Map::new();
    for key in surface_false_keys {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    side_effects.insert("filesystem_written".to_string(), serde_json::json!(false));
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_non_persistence_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_denial_report();
    let source_report_sha256 = sha256_json_value(&source);
    let source_ready = source
        .get("memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_denial_ready")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let source_u64 = |key: &str| -> u64 {
        source
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let surface_false_keys = [
        "operator_summary_allowed",
        "operator_summary_request_accepted",
        "operator_summary_recorded",
        "operator_summary_persisted",
        "operator_summary_materialized",
        "operator_summary_filesystem_written",
        "operator_summary_delivered",
        "operator_briefing_allowed",
        "operator_briefing_request_accepted",
        "operator_briefing_recorded",
        "operator_briefing_persisted",
        "operator_briefing_materialized",
        "operator_briefing_filesystem_written",
        "operator_briefing_delivered",
        "readback_digest_recorded",
        "readback_digest_persisted",
        "final_note_recorded",
        "final_note_persisted",
        "status_banner_recorded",
        "status_banner_persisted",
        "dashboard_annotation_recorded",
        "notification_preview_recorded",
        "timeline_entry_recorded",
        "audit_narrative_recorded",
        "privacy_review_narrative_recorded",
        "alert_explanation_recorded",
        "slo_report_recorded",
        "query_registration_from_summary_recorded",
        "export_snapshot_from_summary_recorded",
        "observability_event_from_summary_recorded",
        "completion_ack_from_summary_recorded",
        "result_receipt_from_summary_recorded",
        "result_receipt_from_summary_persisted",
        "operator_acceptance_from_summary_recorded",
        "operator_approval_from_summary_derived",
        "release_publication_authority_from_summary_derived",
        "activation_authority_from_summary_derived",
        "activation_command_from_summary_derived",
        "activation_from_summary_allowed",
        "live_execution_from_summary_allowed",
        "download_link_from_summary_rendered",
        "install_command_from_summary_rendered",
        "install_from_summary_executed",
        "service_restart_from_summary_performed",
        "launchd_from_summary_mutated",
        "active_binary_from_summary_mutated",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "telegram_send_performed",
        "channel_send_performed",
        "external_send_performed",
        "public_release_claimed",
        "public_ga_claimed",
        "release_artifact_written",
        "public_artifact_written",
    ];
    let summary_briefing_surface_specs = [
        (
            "source_export_query_observability_report_required",
            "blocked_source_export_query_observability_required_noop",
            "source_export_query_observability_report_required",
        ),
        (
            "download_button_operator_summary_request_claim",
            "blocked_summary_request_noop",
            "download_button_operator_summary_request_claim_denied",
        ),
        (
            "direct_download_url_operator_briefing_request_claim",
            "blocked_briefing_request_noop",
            "direct_download_url_operator_briefing_request_claim_denied",
        ),
        (
            "checksum_prompt_summary_readback_digest_claim",
            "blocked_readback_digest_noop",
            "checksum_prompt_summary_readback_digest_claim_denied",
        ),
        (
            "package_manager_install_command_briefing_status_banner_claim",
            "blocked_status_banner_noop",
            "package_manager_install_command_briefing_status_banner_claim_denied",
        ),
        (
            "curl_pipe_shell_summary_exported_text_claim",
            "blocked_summary_export_text_noop",
            "curl_pipe_shell_summary_exported_text_claim_denied",
        ),
        (
            "installer_launch_prompt_briefing_materialization_claim",
            "blocked_briefing_materialization_noop",
            "installer_launch_prompt_briefing_materialization_claim_denied",
        ),
        (
            "auto_update_offer_summary_persistence_claim",
            "blocked_summary_persistence_noop",
            "auto_update_offer_summary_persistence_claim_denied",
        ),
        (
            "release_channel_subscription_briefing_persistence_claim",
            "blocked_briefing_persistence_noop",
            "release_channel_subscription_briefing_persistence_claim_denied",
        ),
        (
            "update_feed_hint_notification_preview_claim",
            "blocked_notification_preview_noop",
            "update_feed_hint_notification_preview_claim_denied",
        ),
        (
            "package_registry_badge_timeline_entry_claim",
            "blocked_timeline_entry_noop",
            "package_registry_badge_timeline_entry_claim_denied",
        ),
        (
            "cdn_mirror_download_dashboard_annotation_claim",
            "blocked_dashboard_annotation_noop",
            "cdn_mirror_download_dashboard_annotation_claim_denied",
        ),
        (
            "sbom_provenance_notarization_audit_narrative_claim",
            "blocked_audit_narrative_noop",
            "sbom_provenance_notarization_audit_narrative_claim_denied",
        ),
        (
            "signature_verification_command_operator_summary_readback_claim",
            "blocked_operator_summary_readback_noop",
            "signature_verification_command_operator_summary_readback_claim_denied",
        ),
        (
            "one_click_install_deep_link_operator_approval_summary_claim",
            "blocked_operator_approval_summary_noop",
            "one_click_install_deep_link_operator_approval_summary_claim_denied",
        ),
        (
            "external_telegram_install_message_operator_briefing_delivery_claim",
            "blocked_briefing_delivery_noop",
            "external_telegram_install_message_operator_briefing_delivery_claim_denied",
        ),
        (
            "release_publication_authority_install_affordance_operator_authority_briefing_claim",
            "blocked_authority_briefing_noop",
            "release_publication_authority_install_affordance_operator_authority_briefing_claim_denied",
        ),
        (
            "activation_live_install_restart_active_binary_operator_briefing_claim",
            "blocked_live_briefing_noop",
            "activation_live_install_restart_active_binary_operator_briefing_claim_denied",
        ),
    ];
    let operator_summary_briefing_surfaces = summary_briefing_surface_specs
        .iter()
        .map(|(surface, status, reason)| {
            let mut surface_report = serde_json::json!({
                "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_surface": surface,
                "source_export_query_observability_ready": source_ready,
                "artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_attempted": true,
                "operator_summary_requested": false,
                "operator_briefing_requested": false,
                "operator_readback_requested": false,
                "status_promotion_requested": false,
                "operator_summary_briefing_noop_confirmed": true,
                "operator_summary_briefing_status": status,
                "reason": reason,
            });
            if let Some(surface_object) = surface_report.as_object_mut() {
                for key in surface_false_keys {
                    surface_object.insert(key.to_string(), serde_json::json!(false));
                }
                match *surface {
                    "source_export_query_observability_report_required" => {
                        surface_object.insert(
                            "source_export_query_observability_report_required".to_string(),
                            serde_json::json!(true),
                        );
                    }
                    "download_button_operator_summary_request_claim" => {
                        surface_object
                            .insert("operator_summary_requested".to_string(), serde_json::json!(true));
                    }
                    "direct_download_url_operator_briefing_request_claim" => {
                        surface_object
                            .insert("operator_briefing_requested".to_string(), serde_json::json!(true));
                    }
                    "checksum_prompt_summary_readback_digest_claim" => {
                        for key in ["operator_summary_requested", "operator_readback_requested"] {
                            surface_object.insert(key.to_string(), serde_json::json!(true));
                        }
                    }
                    "package_manager_install_command_briefing_status_banner_claim" => {
                        for key in ["operator_briefing_requested", "status_promotion_requested"] {
                            surface_object.insert(key.to_string(), serde_json::json!(true));
                        }
                    }
                    "curl_pipe_shell_summary_exported_text_claim" => {
                        for key in ["operator_summary_requested", "summary_export_text_requested"] {
                            surface_object.insert(key.to_string(), serde_json::json!(true));
                        }
                    }
                    "installer_launch_prompt_briefing_materialization_claim" => {
                        for key in [
                            "operator_briefing_requested",
                            "briefing_materialization_requested",
                        ] {
                            surface_object.insert(key.to_string(), serde_json::json!(true));
                        }
                    }
                    "auto_update_offer_summary_persistence_claim" => {
                        for key in ["operator_summary_requested", "summary_persistence_requested"] {
                            surface_object.insert(key.to_string(), serde_json::json!(true));
                        }
                    }
                    "release_channel_subscription_briefing_persistence_claim" => {
                        for key in ["operator_briefing_requested", "briefing_persistence_requested"] {
                            surface_object.insert(key.to_string(), serde_json::json!(true));
                        }
                    }
                    "update_feed_hint_notification_preview_claim" => {
                        for key in ["operator_summary_requested", "notification_preview_requested"] {
                            surface_object.insert(key.to_string(), serde_json::json!(true));
                        }
                    }
                    "package_registry_badge_timeline_entry_claim" => {
                        for key in ["operator_briefing_requested", "timeline_entry_requested"] {
                            surface_object.insert(key.to_string(), serde_json::json!(true));
                        }
                    }
                    "cdn_mirror_download_dashboard_annotation_claim" => {
                        for key in ["operator_summary_requested", "dashboard_annotation_requested"] {
                            surface_object.insert(key.to_string(), serde_json::json!(true));
                        }
                    }
                    "sbom_provenance_notarization_audit_narrative_claim" => {
                        for key in ["operator_briefing_requested", "audit_narrative_requested"] {
                            surface_object.insert(key.to_string(), serde_json::json!(true));
                        }
                    }
                    "signature_verification_command_operator_summary_readback_claim" => {
                        for key in ["operator_summary_requested", "operator_readback_requested"] {
                            surface_object.insert(key.to_string(), serde_json::json!(true));
                        }
                    }
                    "one_click_install_deep_link_operator_approval_summary_claim" => {
                        for key in ["operator_summary_requested", "operator_approval_summary_requested"] {
                            surface_object.insert(key.to_string(), serde_json::json!(true));
                        }
                    }
                    "external_telegram_install_message_operator_briefing_delivery_claim" => {
                        for key in [
                            "operator_briefing_requested",
                            "channel_delivery_requested",
                            "telegram_delivery_requested",
                        ] {
                            surface_object.insert(key.to_string(), serde_json::json!(true));
                        }
                    }
                    "release_publication_authority_install_affordance_operator_authority_briefing_claim" => {
                        for key in ["operator_briefing_requested", "authority_briefing_requested"] {
                            surface_object.insert(key.to_string(), serde_json::json!(true));
                        }
                    }
                    "activation_live_install_restart_active_binary_operator_briefing_claim" => {
                        for key in [
                            "operator_briefing_requested",
                            "live_install_restart_active_binary_briefing_requested",
                        ] {
                            surface_object.insert(key.to_string(), serde_json::json!(true));
                        }
                    }
                    _ => {}
                }
            }
            surface_report
        })
        .collect::<Vec<_>>();
    let operator_summary_briefing_surface_count = operator_summary_briefing_surfaces.len();
    let contract_hash = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-operator-facing-summary-briefing-non-persistence-denial:native:source={source_report_sha256}:surfaces={operator_summary_briefing_surface_count}:route_count={}:summary=0:briefing=0:readback=0:authority=0:install=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "artifact-download-install-affordance-result-receipt-operator-facing-summary-briefing-non-persistence:no-summary:no-briefing:no-readback:no-status:no-delivery:no-approval:no-authority:no-install:no-live",
    );
    let denials = vec![
        "artifact_download_install_affordance_result_receipt_operator_summary_acceptance_denied",
        "artifact_download_install_affordance_result_receipt_operator_summary_recording_denied",
        "artifact_download_install_affordance_result_receipt_operator_summary_persistence_denied",
        "artifact_download_install_affordance_result_receipt_operator_summary_materialization_denied",
        "artifact_download_install_affordance_result_receipt_operator_summary_filesystem_write_denied",
        "artifact_download_install_affordance_result_receipt_operator_summary_delivery_denied",
        "artifact_download_install_affordance_result_receipt_operator_briefing_acceptance_denied",
        "artifact_download_install_affordance_result_receipt_operator_briefing_recording_denied",
        "artifact_download_install_affordance_result_receipt_operator_briefing_persistence_denied",
        "artifact_download_install_affordance_result_receipt_operator_briefing_materialization_denied",
        "artifact_download_install_affordance_result_receipt_operator_briefing_filesystem_write_denied",
        "artifact_download_install_affordance_result_receipt_operator_briefing_delivery_denied",
        "artifact_download_install_affordance_result_receipt_readback_digest_denied",
        "artifact_download_install_affordance_result_receipt_final_note_denied",
        "artifact_download_install_affordance_result_receipt_status_banner_denied",
        "artifact_download_install_affordance_result_receipt_dashboard_annotation_denied",
        "artifact_download_install_affordance_result_receipt_notification_timeline_denied",
        "artifact_download_install_affordance_result_receipt_audit_privacy_narrative_denied",
        "artifact_download_install_affordance_result_receipt_alert_slo_explanation_denied",
        "artifact_download_install_affordance_result_receipt_query_export_observability_from_summary_denied",
        "artifact_download_install_affordance_result_receipt_completion_ack_from_summary_denied",
        "artifact_download_install_affordance_result_receipt_acceptance_from_summary_denied",
        "artifact_download_install_affordance_operator_approval_from_summary_denied",
        "artifact_download_install_affordance_release_publication_authority_from_summary_denied",
        "artifact_download_install_affordance_activation_authority_from_summary_denied",
        "artifact_download_install_affordance_download_install_affordance_from_summary_denied",
        "artifact_download_install_affordance_install_restart_active_binary_from_summary_denied",
        "artifact_download_install_affordance_memory_provider_secret_external_send_from_summary_denied",
    ];
    let denied_count = denials.len();
    let report_ready = source_ready
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_surface_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_attempt_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_denied_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_query_registered_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_snapshot_recorded_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_observability_metric_recorded_count",
        ) == 0
        && operator_summary_briefing_surface_count == 18
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_non_persistence_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_FACING_SUMMARY_BRIEFING_NON_PERSISTENCE_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-operator-facing-summary-briefing-non-persistence-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-19",
        "minimum_required_samples": 24,
        "compatibility_mode": "native_full_live_activation_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_non_persistence_denial_status",
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "source_route_wired": true,
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_non_persistence_denial_route_enabled": true,
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready": report_ready,
            "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_non_persistence_denial_v1",
            "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_mode": "native_route_denied_artifact_download_install_result_receipt_cannot_create_operator_summary_briefing_status_or_authority",
            "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_denial_route",
            "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_ready": source_ready,
            "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_report_sha256": source_report_sha256,
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_contract_hash_sha256": source["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_contract_hash_sha256"].clone(),
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_contract_hash_sha256": contract_hash,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_policy_hash_sha256": policy_hash,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_surface_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_surface_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_attempt_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_attempt_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_denied_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_denied_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_query_registered_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_query_registered_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_snapshot_recorded_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_snapshot_recorded_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_observability_metric_recorded_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_observability_metric_recorded_count"),
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_surface_count": operator_summary_briefing_surface_count,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_attempt_count": operator_summary_briefing_surface_count,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_denied_count": operator_summary_briefing_surface_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_surfaces": operator_summary_briefing_surfaces,
            "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing": denials,
            "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_count": denied_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [serde_json::json!({
                "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_non_acceptance_denial_gate",
                "status": "allowed_report_only_next_slice",
                "accepts_operator_acknowledgement": false,
                "persists_acknowledgement": false,
                "records_summary": false,
                "records_briefing": false,
                "derives_authority": false,
                "renders_download_link": false,
                "emits_install_command": false,
                "installs_or_restarts": false,
                "mutates_active_binary": false,
                "mutates_memory_store": false,
                "writes_kg": false,
                "sends_externally": false,
            })],
        }),
    );

    let zero_keys = [
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_summary_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_summary_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_summary_materialized_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_summary_filesystem_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_summary_delivered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_briefing_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_briefing_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_briefing_materialized_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_briefing_filesystem_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_briefing_delivered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_readback_digest_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_note_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_status_banner_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_dashboard_annotation_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_notification_preview_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_timeline_entry_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_narrative_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_privacy_review_narrative_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_alert_explanation_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_slo_report_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_completion_ack_from_summary_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_acceptance_from_summary_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_summary_briefing_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_summary_briefing_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_summary_briefing_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_summary_briefing_service_restart_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_summary_briefing_active_binary_mutated_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_summary_briefing_external_send_count",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in zero_keys {
            report_object.insert(key.to_string(), serde_json::json!(0));
        }
    }

    let false_keys = [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_summary_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_summary_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_briefing_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_briefing_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_readback_digest_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_note_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_status_banner_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_dashboard_annotation_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_notification_preview_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_timeline_entry_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_narrative_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_privacy_review_narrative_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_alert_explanation_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_slo_report_recorded",
        "artifact_download_install_affordance_result_receipt_recorded",
        "artifact_download_install_affordance_result_receipt_persisted",
        "artifact_download_install_affordance_result_receipt_exported",
        "artifact_download_install_affordance_result_receipt_query_registered",
        "artifact_download_install_affordance_result_receipt_observability_recorded",
        "artifact_download_install_affordance_completion_ack_recorded",
        "download_button_rendered",
        "direct_download_url_exposed",
        "package_manager_install_command_rendered",
        "curl_pipe_shell_snippet_rendered",
        "installer_launch_prompt_rendered",
        "auto_update_offer_rendered",
        "external_install_message_sent",
        "telegram_install_message_sent",
        "operator_acceptance_recorded",
        "operator_approval_recorded",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "activation_allowed",
        "activation_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
        "public_release_claimed",
        "public_ga_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "external_send_performed",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in false_keys {
            report_object.insert(key.to_string(), serde_json::json!(false));
        }
    }

    let mut side_effects = serde_json::Map::new();
    for key in surface_false_keys {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    side_effects.insert("filesystem_written".to_string(), serde_json::json!(false));
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_non_acceptance_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_non_persistence_denial_report();
    let source_report_sha256 = sha256_json_value(&source);
    let source_ready = source
        .get("memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let source_u64 = |key: &str| {
        source
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_contract_hash = source
        .get("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_contract_hash_sha256")
        .cloned()
        .unwrap_or_else(|| serde_json::json!(""));
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let surface_false_keys = [
        "final_operator_acknowledgement_requested",
        "final_operator_acknowledgement_allowed",
        "final_operator_acknowledgement_request_accepted",
        "final_operator_acknowledgement_accepted",
        "final_operator_acknowledgement_recorded",
        "final_operator_acknowledgement_persisted",
        "final_operator_acknowledgement_materialized",
        "final_operator_acknowledgement_filesystem_written",
        "final_operator_acknowledgement_delivered",
        "operator_received_recorded",
        "operator_confirmed_recorded",
        "operator_read_recorded",
        "operator_seen_recorded",
        "final_response_recorded",
        "completion_acknowledgement_recorded",
        "status_acknowledgement_recorded",
        "summary_acknowledgement_recorded",
        "briefing_acknowledgement_recorded",
        "readback_digest_acknowledgement_recorded",
        "dashboard_acknowledgement_recorded",
        "notification_acknowledgement_recorded",
        "channel_acknowledgement_delivered",
        "external_acknowledgement_sent",
        "telegram_acknowledgement_sent",
        "acknowledgement_acceptance_recorded",
        "operator_approval_from_acknowledgement_derived",
        "release_publication_authority_from_acknowledgement_derived",
        "activation_authority_from_acknowledgement_derived",
        "activation_command_from_acknowledgement_derived",
        "activation_from_acknowledgement_allowed",
        "live_execution_from_acknowledgement_allowed",
        "download_link_from_acknowledgement_rendered",
        "install_command_from_acknowledgement_rendered",
        "install_from_acknowledgement_executed",
        "service_restart_from_acknowledgement_performed",
        "launchd_from_acknowledgement_mutated",
        "active_binary_from_acknowledgement_mutated",
        "result_receipt_from_acknowledgement_recorded",
        "result_receipt_from_acknowledgement_persisted",
        "operator_summary_from_acknowledgement_recorded",
        "operator_briefing_from_acknowledgement_recorded",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "telegram_send_performed",
        "channel_send_performed",
        "external_send_performed",
        "public_release_claimed",
        "public_ga_claimed",
        "release_artifact_written",
        "public_artifact_written",
    ];
    let final_acknowledgement_surface_specs: [(&str, &str, &str, &[&str]); 18] = [
        (
            "source_operator_facing_summary_briefing_report_required",
            "blocked_source_summary_briefing_required_noop",
            "source_operator_facing_summary_briefing_report_required",
            &["source_operator_facing_summary_briefing_report_required"],
        ),
        (
            "download_button_final_operator_acknowledgement_claim",
            "blocked_final_ack_noop",
            "download_button_final_operator_acknowledgement_claim_denied",
            &["final_operator_acknowledgement_requested"],
        ),
        (
            "direct_download_url_operator_received_claim",
            "blocked_operator_received_noop",
            "direct_download_url_operator_received_claim_denied",
            &["operator_received_requested"],
        ),
        (
            "checksum_prompt_operator_confirmed_claim",
            "blocked_operator_confirmed_noop",
            "checksum_prompt_operator_confirmed_claim_denied",
            &["operator_confirmed_requested"],
        ),
        (
            "package_manager_install_command_operator_read_claim",
            "blocked_operator_read_noop",
            "package_manager_install_command_operator_read_claim_denied",
            &["operator_read_requested"],
        ),
        (
            "curl_pipe_shell_operator_seen_claim",
            "blocked_operator_seen_noop",
            "curl_pipe_shell_operator_seen_claim_denied",
            &["operator_seen_requested"],
        ),
        (
            "installer_launch_prompt_final_response_claim",
            "blocked_final_response_noop",
            "installer_launch_prompt_final_response_claim_denied",
            &["final_response_requested"],
        ),
        (
            "auto_update_offer_completion_acknowledgement_claim",
            "blocked_completion_ack_noop",
            "auto_update_offer_completion_acknowledgement_claim_denied",
            &["completion_acknowledgement_requested"],
        ),
        (
            "release_channel_subscription_status_acknowledgement_claim",
            "blocked_status_ack_noop",
            "release_channel_subscription_status_acknowledgement_claim_denied",
            &["status_acknowledgement_requested"],
        ),
        (
            "update_feed_hint_summary_acknowledgement_claim",
            "blocked_summary_ack_noop",
            "update_feed_hint_summary_acknowledgement_claim_denied",
            &["summary_acknowledgement_requested"],
        ),
        (
            "package_registry_badge_briefing_acknowledgement_claim",
            "blocked_briefing_ack_noop",
            "package_registry_badge_briefing_acknowledgement_claim_denied",
            &["briefing_acknowledgement_requested"],
        ),
        (
            "cdn_mirror_download_readback_digest_acknowledgement_claim",
            "blocked_readback_ack_noop",
            "cdn_mirror_download_readback_digest_acknowledgement_claim_denied",
            &["readback_digest_acknowledgement_requested"],
        ),
        (
            "sbom_provenance_notarization_dashboard_notification_acknowledgement_claim",
            "blocked_dashboard_notification_ack_noop",
            "sbom_provenance_notarization_dashboard_notification_acknowledgement_claim_denied",
            &[
                "dashboard_acknowledgement_requested",
                "notification_acknowledgement_requested",
            ],
        ),
        (
            "signature_verification_command_channel_acknowledgement_claim",
            "blocked_channel_ack_noop",
            "signature_verification_command_channel_acknowledgement_claim_denied",
            &["channel_acknowledgement_requested"],
        ),
        (
            "one_click_install_deep_link_operator_approval_acknowledgement_claim",
            "blocked_operator_approval_ack_noop",
            "one_click_install_deep_link_operator_approval_acknowledgement_claim_denied",
            &["operator_approval_acknowledgement_requested"],
        ),
        (
            "external_telegram_install_message_external_telegram_acknowledgement_claim",
            "blocked_external_telegram_ack_noop",
            "external_telegram_install_message_external_telegram_acknowledgement_claim_denied",
            &[
                "external_acknowledgement_requested",
                "telegram_acknowledgement_requested",
            ],
        ),
        (
            "release_publication_authority_install_affordance_authority_acknowledgement_claim",
            "blocked_authority_ack_noop",
            "release_publication_authority_install_affordance_authority_acknowledgement_claim_denied",
            &["authority_acknowledgement_requested"],
        ),
        (
            "activation_live_install_restart_active_binary_acknowledgement_claim",
            "blocked_live_ack_noop",
            "activation_live_install_restart_active_binary_acknowledgement_claim_denied",
            &[
                "live_acknowledgement_requested",
                "install_restart_active_binary_acknowledgement_requested",
            ],
        ),
    ];
    let final_acknowledgement_surfaces = final_acknowledgement_surface_specs
        .iter()
        .map(|(surface, status, reason, extra_true_keys)| {
            let mut surface_report = serde_json::json!({
                "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_surface": surface,
                "source_operator_facing_summary_briefing_ready": source_ready,
                "artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_attempted": true,
                "final_operator_acknowledgement_noop_confirmed": true,
                "final_operator_acknowledgement_status": status,
                "reason": reason,
            });
            if let Some(surface_object) = surface_report.as_object_mut() {
                for key in &surface_false_keys {
                    surface_object.insert((*key).to_string(), serde_json::json!(false));
                }
                for key in extra_true_keys.iter() {
                    surface_object.insert((*key).to_string(), serde_json::json!(true));
                }
            }
            surface_report
        })
        .collect::<Vec<_>>();
    let final_acknowledgement_surface_count = final_acknowledgement_surfaces.len();
    let contract_hash = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-final-operator-acknowledgement-non-acceptance-denial:native:source={source_report_sha256}:surfaces={final_acknowledgement_surface_count}:route_count={}:ack=0:authority=0:install=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "artifact-download-install-affordance-result-receipt-final-operator-acknowledgement-non-acceptance:no-ack:no-received:no-confirmed:no-read:no-seen:no-final-response:no-status:no-authority:no-install:no-live",
    );
    let denials = vec![
        "artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_acceptance_denied",
        "artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_recording_denied",
        "artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_persistence_denied",
        "artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_materialization_denied",
        "artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_filesystem_write_denied",
        "artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_delivery_denied",
        "artifact_download_install_affordance_result_receipt_operator_received_denied",
        "artifact_download_install_affordance_result_receipt_operator_confirmed_denied",
        "artifact_download_install_affordance_result_receipt_operator_read_denied",
        "artifact_download_install_affordance_result_receipt_operator_seen_denied",
        "artifact_download_install_affordance_result_receipt_final_response_denied",
        "artifact_download_install_affordance_result_receipt_completion_acknowledgement_denied",
        "artifact_download_install_affordance_result_receipt_status_acknowledgement_denied",
        "artifact_download_install_affordance_result_receipt_summary_briefing_acknowledgement_denied",
        "artifact_download_install_affordance_result_receipt_readback_dashboard_notification_acknowledgement_denied",
        "artifact_download_install_affordance_result_receipt_channel_external_telegram_acknowledgement_denied",
        "artifact_download_install_affordance_result_receipt_acceptance_from_acknowledgement_denied",
        "artifact_download_install_affordance_operator_approval_from_acknowledgement_denied",
        "artifact_download_install_affordance_release_publication_authority_from_acknowledgement_denied",
        "artifact_download_install_affordance_activation_authority_from_acknowledgement_denied",
        "artifact_download_install_affordance_download_install_affordance_from_acknowledgement_denied",
        "artifact_download_install_affordance_install_restart_active_binary_from_acknowledgement_denied",
        "artifact_download_install_affordance_memory_provider_secret_external_send_from_acknowledgement_denied",
    ];
    let denied_count = denials.len();
    let report_ready = source_ready
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_surface_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_denied_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_summary_recorded_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_briefing_recorded_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_readback_digest_recorded_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_summary_briefing_release_publication_authority_derived_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_summary_briefing_activation_authority_derived_count",
        ) == 0
        && final_acknowledgement_surface_count == 18
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_non_acceptance_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_FINAL_OPERATOR_ACKNOWLEDGEMENT_NON_ACCEPTANCE_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-final-operator-acknowledgement-non-acceptance-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-19",
        "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_non_acceptance_denial_v1",
        "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_mode": "native_route_denied_artifact_download_install_result_receipt_cannot_create_final_operator_acknowledgement_acceptance_or_authority",
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_gate": source["gate"].clone(),
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_ready": source_ready,
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_report_sha256": source_report_sha256,
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_contract_hash_sha256": source_contract_hash,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_contract_hash_sha256": contract_hash,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_policy_hash_sha256": policy_hash,
        "minimum_required_samples": 24,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "source_route_wired": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_non_acceptance_denial_route_enabled": true,
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready": report_ready,
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_surface_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_surface_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_attempt_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_attempt_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_denied_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_denied_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_summary_recorded_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_summary_recorded_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_briefing_recorded_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_briefing_recorded_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_readback_digest_recorded_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_readback_digest_recorded_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_summary_briefing_release_publication_authority_derived_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_summary_briefing_release_publication_authority_derived_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_summary_briefing_activation_authority_derived_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_summary_briefing_activation_authority_derived_count"),
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_surface_count": final_acknowledgement_surface_count,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_attempt_count": final_acknowledgement_surface_count,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_denied_count": final_acknowledgement_surface_count,
        }),
    );

    let zero_keys = [
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_materialized_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_filesystem_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_delivered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_received_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_confirmed_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_read_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_seen_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_response_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_completion_acknowledgement_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_status_acknowledgement_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_summary_acknowledgement_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_briefing_acknowledgement_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_readback_digest_acknowledgement_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_dashboard_acknowledgement_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_notification_acknowledgement_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_channel_acknowledgement_delivered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_external_acknowledgement_sent_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_telegram_acknowledgement_sent_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_acceptance_from_acknowledgement_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_final_acknowledgement_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_final_acknowledgement_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_final_acknowledgement_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_final_acknowledgement_service_restart_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_final_acknowledgement_active_binary_mutated_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_final_acknowledgement_external_send_count",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in &zero_keys {
            report_object.insert((*key).to_string(), serde_json::json!(0));
        }
    }
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_surfaces": final_acknowledgement_surfaces,
            "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement": denials,
            "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_count": denied_count,
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_denial_gate",
                    "status": "allowed_report_only_next_slice",
                    "records_terminal_decision": false,
                    "records_status_promotion": false,
                    "accepts_operator_acknowledgement": false,
                    "derives_authority": false,
                    "renders_download_link": false,
                    "emits_install_command": false,
                    "installs_or_restarts": false,
                    "mutates_active_binary": false,
                    "mutates_memory_store": false,
                    "writes_kg": false,
                    "sends_externally": false
                }
            ],
        }),
    );

    let false_keys = [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_received_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_confirmed_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_read_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_seen_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_response_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_completion_acknowledgement_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_status_acknowledgement_recorded",
        "artifact_download_install_affordance_result_receipt_recorded",
        "artifact_download_install_affordance_result_receipt_persisted",
        "artifact_download_install_affordance_completion_ack_recorded",
        "download_button_rendered",
        "direct_download_url_exposed",
        "package_manager_install_command_rendered",
        "curl_pipe_shell_snippet_rendered",
        "installer_launch_prompt_rendered",
        "auto_update_offer_rendered",
        "external_install_message_sent",
        "telegram_install_message_sent",
        "operator_acceptance_recorded",
        "operator_approval_recorded",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "activation_allowed",
        "activation_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
        "public_release_claimed",
        "public_ga_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "external_send_performed",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in &false_keys {
            report_object.insert((*key).to_string(), serde_json::json!(false));
        }
    }

    let mut side_effects = serde_json::Map::new();
    for key in &surface_false_keys {
        side_effects.insert((*key).to_string(), serde_json::json!(false));
    }
    side_effects.insert("filesystem_written".to_string(), serde_json::json!(false));
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_non_acceptance_denial_report();
    let source_report_sha256 = sha256_json_value(&source);
    let source_ready = source
        .get("memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let source_u64 = |key: &str| {
        source
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_contract_hash = source
        .get("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_contract_hash_sha256")
        .cloned()
        .unwrap_or_else(|| serde_json::json!(""));
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let surface_false_keys = [
        "terminal_decision_requested",
        "status_promotion_requested",
        "terminal_decision_allowed",
        "status_promotion_allowed",
        "terminal_decision_accepted",
        "terminal_decision_recorded",
        "terminal_decision_persisted",
        "terminal_decision_materialized",
        "terminal_decision_filesystem_written",
        "terminal_decision_delivered",
        "terminal_status_recorded",
        "terminal_status_persisted",
        "terminal_status_materialized",
        "terminal_status_filesystem_written",
        "status_promotion_recorded",
        "delivery_status_promoted",
        "operator_acknowledgement_status_promoted",
        "operator_received_status_promoted",
        "operator_confirmed_status_promoted",
        "operator_read_status_promoted",
        "operator_seen_status_promoted",
        "final_response_status_promoted",
        "completion_status_promoted",
        "summary_status_promoted",
        "briefing_status_promoted",
        "readback_status_promoted",
        "dashboard_status_promoted",
        "notification_status_promoted",
        "channel_decision_delivered",
        "external_decision_sent",
        "telegram_decision_sent",
        "acceptance_from_terminal_decision_recorded",
        "operator_approval_from_terminal_status_derived",
        "release_publication_authority_from_terminal_decision_derived",
        "activation_authority_from_terminal_status_derived",
        "download_link_from_terminal_status_rendered",
        "install_command_from_terminal_status_rendered",
        "install_from_terminal_status_executed",
        "service_restart_from_terminal_status_performed",
        "launchd_from_terminal_status_mutated",
        "active_binary_from_terminal_status_mutated",
        "result_receipt_from_terminal_status_recorded",
        "result_receipt_from_terminal_status_persisted",
        "final_acknowledgement_from_terminal_decision_recorded",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "telegram_send_performed",
        "channel_send_performed",
        "external_send_performed",
        "public_release_claimed",
        "public_ga_claimed",
        "release_artifact_written",
        "public_artifact_written",
    ];
    let surface_specs: [(&str, &str, &str, &[&str]); 18] = [
        (
            "source_final_operator_acknowledgement_report_required",
            "blocked_source_final_acknowledgement_required_noop",
            "source_final_operator_acknowledgement_report_required",
            &["source_final_operator_acknowledgement_report_required"],
        ),
        (
            "download_button_terminal_decision_promotion_claim",
            "blocked_terminal_decision_noop",
            "download_button_terminal_decision_promotion_claim_denied",
            &["terminal_decision_requested"],
        ),
        (
            "direct_download_url_status_promoted_claim",
            "blocked_direct_url_status_noop",
            "direct_download_url_status_promoted_claim_denied",
            &["status_promotion_requested"],
        ),
        (
            "checksum_prompt_terminal_status_verified_claim",
            "blocked_checksum_status_noop",
            "checksum_prompt_terminal_status_verified_claim_denied",
            &["terminal_status_verification_requested"],
        ),
        (
            "package_manager_install_command_decision_record_claim",
            "blocked_package_manager_decision_noop",
            "package_manager_install_command_decision_record_claim_denied",
            &["decision_record_requested"],
        ),
        (
            "curl_pipe_shell_status_banner_promotion_claim",
            "blocked_status_banner_noop",
            "curl_pipe_shell_status_banner_promotion_claim_denied",
            &["status_banner_promotion_requested"],
        ),
        (
            "installer_launch_prompt_final_decision_claim",
            "blocked_installer_decision_noop",
            "installer_launch_prompt_final_decision_claim_denied",
            &["final_decision_requested"],
        ),
        (
            "auto_update_offer_completion_status_promotion_claim",
            "blocked_completion_status_noop",
            "auto_update_offer_completion_status_promotion_claim_denied",
            &["completion_status_promotion_requested"],
        ),
        (
            "release_channel_subscription_terminal_status_claim",
            "blocked_release_channel_terminal_status_noop",
            "release_channel_subscription_terminal_status_claim_denied",
            &["release_channel_terminal_status_requested"],
        ),
        (
            "update_feed_hint_decision_summary_claim",
            "blocked_update_feed_decision_summary_noop",
            "update_feed_hint_decision_summary_claim_denied",
            &["decision_summary_requested"],
        ),
        (
            "package_registry_badge_status_badge_claim",
            "blocked_package_registry_status_badge_noop",
            "package_registry_badge_status_badge_claim_denied",
            &["status_badge_requested"],
        ),
        (
            "cdn_mirror_download_terminal_readback_claim",
            "blocked_cdn_terminal_readback_noop",
            "cdn_mirror_download_terminal_readback_claim_denied",
            &["terminal_readback_requested"],
        ),
        (
            "sbom_provenance_notarization_dashboard_status_claim",
            "blocked_dashboard_status_noop",
            "sbom_provenance_notarization_dashboard_status_claim_denied",
            &["dashboard_status_requested"],
        ),
        (
            "signature_verification_command_channel_decision_claim",
            "blocked_channel_decision_noop",
            "signature_verification_command_channel_decision_claim_denied",
            &["channel_decision_requested"],
        ),
        (
            "one_click_install_deep_link_operator_approval_status_claim",
            "blocked_operator_approval_status_noop",
            "one_click_install_deep_link_operator_approval_status_claim_denied",
            &["operator_approval_status_requested"],
        ),
        (
            "external_telegram_install_message_external_decision_claim",
            "blocked_external_telegram_decision_noop",
            "external_telegram_install_message_external_decision_claim_denied",
            &["external_decision_requested", "telegram_decision_requested"],
        ),
        (
            "release_publication_authority_terminal_decision_claim",
            "blocked_authority_decision_noop",
            "release_publication_authority_terminal_decision_claim_denied",
            &["authority_terminal_decision_requested"],
        ),
        (
            "activation_live_install_restart_active_binary_status_promotion_claim",
            "blocked_live_status_promotion_noop",
            "activation_live_install_restart_active_binary_status_promotion_claim_denied",
            &[
                "live_status_promotion_requested",
                "install_restart_active_binary_status_requested",
            ],
        ),
    ];
    let terminal_decision_status_promotion_surfaces = surface_specs
        .iter()
        .map(|(surface, status, reason, extra_true_keys)| {
            let mut surface_report = serde_json::json!({
                "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_surface": surface,
                "source_final_operator_acknowledgement_ready": source_ready,
                "artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_attempted": true,
                "terminal_decision_status_promotion_noop_confirmed": true,
                "terminal_decision_status_promotion_status": status,
                "reason": reason,
            });
            if let Some(surface_object) = surface_report.as_object_mut() {
                for key in &surface_false_keys {
                    surface_object.insert((*key).to_string(), serde_json::json!(false));
                }
                for key in extra_true_keys.iter() {
                    surface_object.insert((*key).to_string(), serde_json::json!(true));
                }
            }
            surface_report
        })
        .collect::<Vec<_>>();
    let terminal_decision_status_promotion_surface_count =
        terminal_decision_status_promotion_surfaces.len();
    let contract_hash = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-terminal-decision-status-promotion-denial:native:source={source_report_sha256}:surfaces={terminal_decision_status_promotion_surface_count}:route_count={}:terminal=0:status=0:authority=0:install=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "artifact-download-install-affordance-result-receipt-terminal-decision-status-promotion:no-terminal-decision:no-status-promotion:no-delivery-status:no-approval:no-authority:no-install:no-live",
    );
    let denials = vec![
        "artifact_download_install_affordance_result_receipt_terminal_decision_acceptance_denied",
        "artifact_download_install_affordance_result_receipt_terminal_decision_recording_denied",
        "artifact_download_install_affordance_result_receipt_terminal_decision_persistence_denied",
        "artifact_download_install_affordance_result_receipt_terminal_decision_materialization_denied",
        "artifact_download_install_affordance_result_receipt_terminal_decision_filesystem_write_denied",
        "artifact_download_install_affordance_result_receipt_terminal_decision_delivery_denied",
        "artifact_download_install_affordance_result_receipt_terminal_status_recording_denied",
        "artifact_download_install_affordance_result_receipt_terminal_status_promotion_denied",
        "artifact_download_install_affordance_result_receipt_delivery_status_promotion_denied",
        "artifact_download_install_affordance_result_receipt_operator_acknowledgement_status_promotion_denied",
        "artifact_download_install_affordance_result_receipt_completion_status_promotion_denied",
        "artifact_download_install_affordance_result_receipt_summary_briefing_status_promotion_denied",
        "artifact_download_install_affordance_result_receipt_readback_dashboard_notification_status_promotion_denied",
        "artifact_download_install_affordance_result_receipt_channel_external_telegram_decision_denied",
        "artifact_download_install_affordance_result_receipt_acceptance_from_terminal_decision_denied",
        "artifact_download_install_affordance_operator_approval_from_terminal_status_denied",
        "artifact_download_install_affordance_release_publication_authority_from_terminal_decision_denied",
        "artifact_download_install_affordance_activation_authority_from_terminal_status_denied",
        "artifact_download_install_affordance_download_install_affordance_from_terminal_status_denied",
        "artifact_download_install_affordance_install_restart_active_binary_from_terminal_status_denied",
        "artifact_download_install_affordance_memory_provider_secret_external_send_from_terminal_status_denied",
    ];
    let denied_count = denials.len();
    let report_ready = source_ready
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_surface_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_denied_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_recorded_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_status_acknowledgement_recorded_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_final_acknowledgement_release_publication_authority_derived_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_final_acknowledgement_activation_authority_derived_count",
        ) == 0
        && terminal_decision_status_promotion_surface_count == 18
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_TERMINAL_DECISION_STATUS_PROMOTION_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-terminal-decision-status-promotion-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-19",
        "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_denial_v1",
        "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_mode": "native_route_denied_artifact_download_install_result_receipt_acknowledgement_cannot_create_terminal_decision_or_status_promotion",
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_gate": source["gate"].clone(),
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_ready": source_ready,
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_report_sha256": source_report_sha256,
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_contract_hash_sha256": source_contract_hash,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_contract_hash_sha256": contract_hash,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_policy_hash_sha256": policy_hash,
        "minimum_required_samples": 24,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "source_route_wired": true,
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_denial_route_enabled": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_denial_ready": report_ready,
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_surface_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_surface_count"),
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_attempt_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_attempt_count"),
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_denied_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_denied_count"),
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_recorded_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_recorded_count"),
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_status_acknowledgement_recorded_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_status_acknowledgement_recorded_count"),
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_surface_count": terminal_decision_status_promotion_surface_count,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_attempt_count": terminal_decision_status_promotion_surface_count,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_denied_count": terminal_decision_status_promotion_surface_count,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_surfaces": terminal_decision_status_promotion_surfaces,
        "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion": denials,
        "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_count": denied_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
        "allowed_next_actions": [
            {
                "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_denial_gate",
                "status": "allowed_report_only_next_slice",
                "records_operator_intent": false,
                "records_operator_consent": false,
                "records_terminal_decision": false,
                "records_status_promotion": false,
                "derives_authority": false,
                "renders_download_link": false,
                "emits_install_command": false,
                "installs_or_restarts": false,
                "mutates_active_binary": false,
                "mutates_memory_store": false,
                "writes_kg": false,
                "sends_externally": false
            }
        ],
        }),
    );

    let zero_keys = [
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_materialized_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_filesystem_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_delivered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_status_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_status_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_status_materialized_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_status_filesystem_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_status_promotion_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_delivery_status_promoted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_acknowledgement_status_promoted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_completion_status_promoted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_summary_status_promoted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_briefing_status_promoted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_readback_status_promoted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_dashboard_status_promoted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_notification_status_promoted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_channel_decision_delivered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_external_decision_sent_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_telegram_decision_sent_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_acceptance_from_terminal_decision_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_terminal_decision_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_terminal_status_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_terminal_status_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_terminal_status_service_restart_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_terminal_status_active_binary_mutated_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_terminal_status_external_send_count",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in &zero_keys {
            report_object.insert((*key).to_string(), serde_json::json!(0));
        }
    }

    let false_keys = [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_status_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_status_promotion_recorded",
        "artifact_download_install_affordance_result_receipt_recorded",
        "artifact_download_install_affordance_result_receipt_persisted",
        "artifact_download_install_affordance_completion_ack_recorded",
        "download_button_rendered",
        "direct_download_url_exposed",
        "package_manager_install_command_rendered",
        "curl_pipe_shell_snippet_rendered",
        "installer_launch_prompt_rendered",
        "auto_update_offer_rendered",
        "external_install_message_sent",
        "telegram_install_message_sent",
        "operator_acceptance_recorded",
        "operator_approval_recorded",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "activation_allowed",
        "activation_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
        "public_release_claimed",
        "public_ga_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "external_send_performed",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in &false_keys {
            report_object.insert((*key).to_string(), serde_json::json!(false));
        }
    }

    let mut side_effects = serde_json::Map::new();
    for key in &surface_false_keys {
        side_effects.insert((*key).to_string(), serde_json::json!(false));
    }
    side_effects.insert("filesystem_written".to_string(), serde_json::json!(false));
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_denial_report();
    let source_report_sha256 = sha256_json_value(&source);
    let source_ready = source
        .get("memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_denial_ready")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let source_u64 = |key: &str| {
        source
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_contract_hash = source
        .get("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_contract_hash_sha256")
        .cloned()
        .unwrap_or_else(|| serde_json::json!(""));
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let surface_false_keys = [
        "operator_intent_reconfirmation_requested",
        "operator_consent_reconfirmation_requested",
        "operator_intent_reconfirmation_allowed",
        "operator_consent_reconfirmation_allowed",
        "operator_intent_reconfirmed",
        "operator_consent_reconfirmed",
        "operator_intent_recorded",
        "operator_intent_persisted",
        "operator_intent_materialized",
        "operator_intent_filesystem_written",
        "operator_consent_recorded",
        "operator_consent_persisted",
        "operator_consent_materialized",
        "operator_consent_filesystem_written",
        "consent_reconfirmation_recorded",
        "consent_reconfirmation_persisted",
        "intent_hash_recorded",
        "consent_token_recorded",
        "intent_nonce_recorded",
        "double_confirm_recorded",
        "explicit_intent_status_promoted",
        "explicit_consent_status_promoted",
        "operator_approval_from_intent_consent_derived",
        "acceptance_from_intent_consent_recorded",
        "terminal_decision_from_intent_consent_recorded",
        "terminal_status_from_intent_consent_recorded",
        "release_publication_authority_from_intent_consent_derived",
        "activation_authority_from_intent_consent_derived",
        "download_link_from_intent_consent_rendered",
        "install_command_from_intent_consent_rendered",
        "install_from_intent_consent_executed",
        "service_restart_from_intent_consent_performed",
        "launchd_from_intent_consent_mutated",
        "active_binary_from_intent_consent_mutated",
        "result_receipt_from_intent_consent_recorded",
        "result_receipt_from_intent_consent_persisted",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "telegram_send_performed",
        "channel_send_performed",
        "external_send_performed",
        "public_release_claimed",
        "public_ga_claimed",
        "release_artifact_written",
        "public_artifact_written",
    ];
    let surface_specs: [(&str, &str, &str, &[&str]); 18] = [
        (
            "source_terminal_decision_status_promotion_report_required",
            "blocked_source_terminal_status_required_noop",
            "source_terminal_decision_status_promotion_report_required",
            &["source_terminal_decision_status_promotion_report_required"],
        ),
        (
            "download_button_operator_intent_reconfirmation_claim",
            "blocked_operator_intent_noop",
            "download_button_operator_intent_reconfirmation_claim_denied",
            &["operator_intent_reconfirmation_requested"],
        ),
        (
            "direct_download_url_operator_consent_reconfirmation_claim",
            "blocked_operator_consent_noop",
            "direct_download_url_operator_consent_reconfirmation_claim_denied",
            &["operator_consent_reconfirmation_requested"],
        ),
        (
            "checksum_prompt_explicit_intent_hash_claim",
            "blocked_intent_hash_noop",
            "checksum_prompt_explicit_intent_hash_claim_denied",
            &["intent_hash_requested"],
        ),
        (
            "package_manager_install_command_consent_token_claim",
            "blocked_consent_token_noop",
            "package_manager_install_command_consent_token_claim_denied",
            &["consent_token_requested"],
        ),
        (
            "curl_pipe_shell_intent_nonce_claim",
            "blocked_intent_nonce_noop",
            "curl_pipe_shell_intent_nonce_claim_denied",
            &["intent_nonce_requested"],
        ),
        (
            "installer_launch_prompt_double_confirm_claim",
            "blocked_double_confirm_noop",
            "installer_launch_prompt_double_confirm_claim_denied",
            &["double_confirm_requested"],
        ),
        (
            "auto_update_offer_consent_refresh_claim",
            "blocked_consent_refresh_noop",
            "auto_update_offer_consent_refresh_claim_denied",
            &["consent_refresh_requested"],
        ),
        (
            "release_channel_subscription_intent_status_claim",
            "blocked_intent_status_noop",
            "release_channel_subscription_intent_status_claim_denied",
            &["intent_status_requested"],
        ),
        (
            "update_feed_hint_consent_summary_claim",
            "blocked_consent_summary_noop",
            "update_feed_hint_consent_summary_claim_denied",
            &["consent_summary_requested"],
        ),
        (
            "package_registry_badge_operator_intent_badge_claim",
            "blocked_intent_badge_noop",
            "package_registry_badge_operator_intent_badge_claim_denied",
            &["operator_intent_badge_requested"],
        ),
        (
            "cdn_mirror_download_consent_readback_claim",
            "blocked_consent_readback_noop",
            "cdn_mirror_download_consent_readback_claim_denied",
            &["consent_readback_requested"],
        ),
        (
            "sbom_provenance_notarization_consent_dashboard_claim",
            "blocked_consent_dashboard_noop",
            "sbom_provenance_notarization_consent_dashboard_claim_denied",
            &["consent_dashboard_requested"],
        ),
        (
            "signature_verification_command_channel_consent_claim",
            "blocked_channel_consent_noop",
            "signature_verification_command_channel_consent_claim_denied",
            &["channel_consent_requested"],
        ),
        (
            "one_click_install_deep_link_operator_intent_approval_claim",
            "blocked_intent_approval_noop",
            "one_click_install_deep_link_operator_intent_approval_claim_denied",
            &["operator_intent_approval_requested"],
        ),
        (
            "external_telegram_install_message_external_consent_claim",
            "blocked_external_telegram_consent_noop",
            "external_telegram_install_message_external_consent_claim_denied",
            &["external_consent_requested", "telegram_consent_requested"],
        ),
        (
            "release_publication_authority_intent_consent_claim",
            "blocked_authority_intent_consent_noop",
            "release_publication_authority_intent_consent_claim_denied",
            &["authority_intent_consent_requested"],
        ),
        (
            "activation_live_install_restart_active_binary_consent_claim",
            "blocked_live_consent_noop",
            "activation_live_install_restart_active_binary_consent_claim_denied",
            &[
                "live_consent_requested",
                "install_restart_active_binary_consent_requested",
            ],
        ),
    ];
    let operator_intent_consent_reconfirmation_surfaces = surface_specs
        .iter()
        .map(|(surface, status, reason, extra_true_keys)| {
            let mut surface_report = serde_json::json!({
                "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_surface": surface,
                "source_terminal_decision_status_promotion_ready": source_ready,
                "artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_attempted": true,
                "operator_intent_consent_reconfirmation_noop_confirmed": true,
                "operator_intent_consent_reconfirmation_status": status,
                "reason": reason,
            });
            if let Some(surface_object) = surface_report.as_object_mut() {
                for key in &surface_false_keys {
                    surface_object.insert((*key).to_string(), serde_json::json!(false));
                }
                for key in extra_true_keys.iter() {
                    surface_object.insert((*key).to_string(), serde_json::json!(true));
                }
            }
            surface_report
        })
        .collect::<Vec<_>>();
    let operator_intent_consent_reconfirmation_surface_count =
        operator_intent_consent_reconfirmation_surfaces.len();
    let contract_hash = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-reconfirmation-denial:native:source={source_report_sha256}:surfaces={operator_intent_consent_reconfirmation_surface_count}:route_count={}:intent=0:consent=0:authority=0:install=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "artifact-download-install-affordance-result-receipt-operator-intent-consent-reconfirmation:no-intent:no-consent:no-reconfirmation:no-approval:no-authority:no-install:no-live",
    );
    let denials = vec![
        "artifact_download_install_affordance_result_receipt_operator_intent_reconfirmation_denied",
        "artifact_download_install_affordance_result_receipt_operator_consent_reconfirmation_denied",
        "artifact_download_install_affordance_result_receipt_operator_intent_recording_denied",
        "artifact_download_install_affordance_result_receipt_operator_intent_persistence_denied",
        "artifact_download_install_affordance_result_receipt_operator_consent_recording_denied",
        "artifact_download_install_affordance_result_receipt_operator_consent_persistence_denied",
        "artifact_download_install_affordance_result_receipt_consent_reconfirmation_recording_denied",
        "artifact_download_install_affordance_result_receipt_intent_hash_recording_denied",
        "artifact_download_install_affordance_result_receipt_consent_token_recording_denied",
        "artifact_download_install_affordance_result_receipt_intent_nonce_recording_denied",
        "artifact_download_install_affordance_result_receipt_double_confirm_recording_denied",
        "artifact_download_install_affordance_result_receipt_explicit_intent_consent_status_promotion_denied",
        "artifact_download_install_affordance_result_receipt_acceptance_from_intent_consent_denied",
        "artifact_download_install_affordance_operator_approval_from_intent_consent_denied",
        "artifact_download_install_affordance_release_publication_authority_from_intent_consent_denied",
        "artifact_download_install_affordance_activation_authority_from_intent_consent_denied",
        "artifact_download_install_affordance_download_install_from_intent_consent_denied",
        "artifact_download_install_affordance_install_restart_active_binary_from_intent_consent_denied",
        "artifact_download_install_affordance_memory_provider_secret_external_send_from_intent_consent_denied",
    ];
    let denied_count = denials.len();
    let report_ready = source_ready
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_surface_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_denied_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_recorded_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_status_promotion_recorded_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_terminal_decision_release_publication_authority_derived_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_terminal_status_activation_authority_derived_count",
        ) == 0
        && operator_intent_consent_reconfirmation_surface_count == 18
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_INTENT_CONSENT_RECONFIRMATION_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-operator-intent-consent-reconfirmation-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-20",
        "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_denial_v1",
        "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_mode": "native_route_denied_terminal_status_cannot_create_operator_intent_consent_reconfirmation_or_authority",
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_gate": source["gate"].clone(),
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_ready": source_ready,
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_report_sha256": source_report_sha256,
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_contract_hash_sha256": source_contract_hash,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_contract_hash_sha256": contract_hash,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_policy_hash_sha256": policy_hash,
        "minimum_required_samples": 24,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "source_route_wired": true,
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_denial_route_enabled": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_denial_ready": report_ready,
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_surface_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_surface_count"),
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_attempt_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_attempt_count"),
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_denied_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_denied_count"),
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_recorded_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_recorded_count"),
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_status_promotion_recorded_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_status_promotion_recorded_count"),
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_surface_count": operator_intent_consent_reconfirmation_surface_count,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_attempt_count": operator_intent_consent_reconfirmation_surface_count,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_denied_count": operator_intent_consent_reconfirmation_surface_count,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_surfaces": operator_intent_consent_reconfirmation_surfaces,
        "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation": denials,
        "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_count": denied_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
        "allowed_next_actions": [
            {
                "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_denial_gate",
                "status": "allowed_report_only_next_slice",
                "records_operator_identity": false,
                "records_operator_session": false,
                "records_operator_intent": false,
                "records_operator_consent": false,
                "derives_authority": false,
                "renders_download_link": false,
                "emits_install_command": false,
                "installs_or_restarts": false,
                "mutates_active_binary": false,
                "mutates_memory_store": false,
                "writes_kg": false,
                "sends_externally": false
            }
        ],
        }),
    );

    let zero_keys = [
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_reconfirmed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_consent_reconfirmed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_persisted_count",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in &zero_keys {
            report_object.insert((*key).to_string(), serde_json::json!(0));
        }
    }

    let additional_zero_keys = [
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_materialized_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_filesystem_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_consent_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_consent_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_consent_materialized_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_consent_filesystem_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_consent_reconfirmation_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_consent_reconfirmation_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_intent_hash_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_consent_token_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_intent_nonce_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_double_confirm_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_explicit_intent_status_promoted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_explicit_consent_status_promoted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_approval_from_intent_consent_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_acceptance_from_intent_consent_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_intent_consent_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_intent_consent_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_intent_consent_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_intent_consent_service_restart_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_intent_consent_active_binary_mutated_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_intent_consent_external_send_count",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in &additional_zero_keys {
            report_object.insert((*key).to_string(), serde_json::json!(0));
        }
    }

    let false_keys = [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_consent_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_consent_reconfirmation_recorded",
        "artifact_download_install_affordance_result_receipt_recorded",
        "artifact_download_install_affordance_result_receipt_persisted",
        "artifact_download_install_affordance_completion_ack_recorded",
        "download_button_rendered",
        "direct_download_url_exposed",
        "package_manager_install_command_rendered",
        "curl_pipe_shell_snippet_rendered",
        "installer_launch_prompt_rendered",
        "auto_update_offer_rendered",
        "external_install_message_sent",
        "telegram_install_message_sent",
        "operator_acceptance_recorded",
        "operator_approval_recorded",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "activation_allowed",
        "activation_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
        "public_release_claimed",
        "public_ga_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "external_send_performed",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in &false_keys {
            report_object.insert((*key).to_string(), serde_json::json!(false));
        }
    }

    let mut side_effects = serde_json::Map::new();
    for key in &surface_false_keys {
        side_effects.insert((*key).to_string(), serde_json::json!(false));
    }
    side_effects.insert("filesystem_written".to_string(), serde_json::json!(false));
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_denial_report();
    let source_report_sha256 = sha256_json_value(&source);
    let source_ready = source
        .get("memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_denial_ready")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let source_u64 = |key: &str| {
        source
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_contract_hash = source
        .get("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_contract_hash_sha256")
        .cloned()
        .unwrap_or_else(|| serde_json::json!(""));
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let surface_false_keys = [
        "operator_identity_binding_requested",
        "operator_session_binding_requested",
        "operator_identity_binding_allowed",
        "operator_session_binding_allowed",
        "operator_identity_accepted",
        "operator_identity_recorded",
        "operator_identity_persisted",
        "operator_identity_materialized",
        "operator_identity_filesystem_written",
        "operator_session_accepted",
        "operator_session_recorded",
        "operator_session_persisted",
        "operator_session_materialized",
        "operator_session_filesystem_written",
        "session_binding_recorded",
        "session_binding_persisted",
        "identity_hash_recorded",
        "session_token_recorded",
        "identity_fingerprint_recorded",
        "identity_nonce_recorded",
        "device_session_recorded",
        "identity_status_promoted",
        "session_status_promoted",
        "operator_approval_from_identity_session_derived",
        "acceptance_from_identity_session_recorded",
        "terminal_decision_from_identity_session_recorded",
        "terminal_status_from_identity_session_recorded",
        "release_publication_authority_from_identity_session_derived",
        "activation_authority_from_identity_session_derived",
        "download_link_from_identity_session_rendered",
        "install_command_from_identity_session_rendered",
        "install_from_identity_session_executed",
        "service_restart_from_identity_session_performed",
        "launchd_from_identity_session_mutated",
        "active_binary_from_identity_session_mutated",
        "result_receipt_from_identity_session_recorded",
        "result_receipt_from_identity_session_persisted",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "telegram_send_performed",
        "channel_send_performed",
        "external_send_performed",
        "public_release_claimed",
        "public_ga_claimed",
        "release_artifact_written",
        "public_artifact_written",
    ];
    let surface_specs: [(&str, &str, &str, &[&str]); 18] = [
        (
            "source_operator_intent_consent_reconfirmation_report_required",
            "blocked_source_intent_consent_required_noop",
            "source_operator_intent_consent_reconfirmation_report_required",
            &["source_operator_intent_consent_reconfirmation_report_required"],
        ),
        (
            "download_button_operator_identity_hash_claim",
            "blocked_identity_hash_noop",
            "download_button_operator_identity_hash_claim_denied",
            &[
                "operator_identity_binding_requested",
                "identity_hash_requested",
            ],
        ),
        (
            "direct_download_url_operator_session_binding_claim",
            "blocked_session_binding_noop",
            "direct_download_url_operator_session_binding_claim_denied",
            &["operator_session_binding_requested"],
        ),
        (
            "checksum_prompt_identity_fingerprint_claim",
            "blocked_identity_fingerprint_noop",
            "checksum_prompt_identity_fingerprint_claim_denied",
            &["identity_fingerprint_requested"],
        ),
        (
            "package_manager_install_command_session_token_claim",
            "blocked_session_token_noop",
            "package_manager_install_command_session_token_claim_denied",
            &["session_token_requested"],
        ),
        (
            "curl_pipe_shell_identity_nonce_binding_claim",
            "blocked_identity_nonce_noop",
            "curl_pipe_shell_identity_nonce_binding_claim_denied",
            &["identity_nonce_requested"],
        ),
        (
            "installer_launch_prompt_device_session_claim",
            "blocked_device_session_noop",
            "installer_launch_prompt_device_session_claim_denied",
            &["device_session_requested"],
        ),
        (
            "auto_update_offer_operator_session_refresh_claim",
            "blocked_session_refresh_noop",
            "auto_update_offer_operator_session_refresh_claim_denied",
            &["session_refresh_requested"],
        ),
        (
            "release_channel_subscription_identity_status_claim",
            "blocked_identity_status_noop",
            "release_channel_subscription_identity_status_claim_denied",
            &["identity_status_requested"],
        ),
        (
            "update_feed_hint_session_summary_claim",
            "blocked_session_summary_noop",
            "update_feed_hint_session_summary_claim_denied",
            &["session_summary_requested"],
        ),
        (
            "package_registry_badge_operator_identity_badge_claim",
            "blocked_identity_badge_noop",
            "package_registry_badge_operator_identity_badge_claim_denied",
            &["operator_identity_badge_requested"],
        ),
        (
            "cdn_mirror_download_session_readback_claim",
            "blocked_session_readback_noop",
            "cdn_mirror_download_session_readback_claim_denied",
            &["session_readback_requested"],
        ),
        (
            "sbom_provenance_notarization_identity_dashboard_claim",
            "blocked_identity_dashboard_noop",
            "sbom_provenance_notarization_identity_dashboard_claim_denied",
            &["identity_dashboard_requested"],
        ),
        (
            "signature_verification_command_channel_session_claim",
            "blocked_channel_session_noop",
            "signature_verification_command_channel_session_claim_denied",
            &["channel_session_requested"],
        ),
        (
            "one_click_install_deep_link_operator_identity_approval_claim",
            "blocked_identity_approval_noop",
            "one_click_install_deep_link_operator_identity_approval_claim_denied",
            &["operator_identity_approval_requested"],
        ),
        (
            "external_telegram_install_message_external_identity_session_claim",
            "blocked_external_telegram_identity_session_noop",
            "external_telegram_install_message_external_identity_session_claim_denied",
            &[
                "external_identity_session_requested",
                "telegram_identity_session_requested",
            ],
        ),
        (
            "release_publication_authority_identity_session_claim",
            "blocked_authority_identity_session_noop",
            "release_publication_authority_identity_session_claim_denied",
            &["authority_identity_session_requested"],
        ),
        (
            "activation_live_install_restart_active_binary_session_claim",
            "blocked_live_session_noop",
            "activation_live_install_restart_active_binary_session_claim_denied",
            &[
                "live_session_requested",
                "install_restart_active_binary_session_requested",
            ],
        ),
    ];
    let operator_identity_session_binding_surfaces = surface_specs
        .iter()
        .map(|(surface, status, reason, extra_true_keys)| {
            let mut surface_report = serde_json::json!({
                "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_surface": surface,
                "source_operator_intent_consent_reconfirmation_ready": source_ready,
                "artifact_download_install_affordance_result_receipt_operator_identity_session_binding_attempted": true,
                "operator_identity_session_binding_noop_confirmed": true,
                "operator_identity_session_binding_status": status,
                "reason": reason,
            });
            if let Some(surface_object) = surface_report.as_object_mut() {
                for key in &surface_false_keys {
                    surface_object.insert((*key).to_string(), serde_json::json!(false));
                }
                for key in extra_true_keys.iter() {
                    surface_object.insert((*key).to_string(), serde_json::json!(true));
                }
            }
            surface_report
        })
        .collect::<Vec<_>>();
    let operator_identity_session_binding_surface_count =
        operator_identity_session_binding_surfaces.len();
    let contract_hash = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-binding-denial:native:source={source_report_sha256}:surfaces={operator_identity_session_binding_surface_count}:route_count={}:identity=0:session=0:binding=0:authority=0:install=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "artifact-download-install-affordance-result-receipt-operator-identity-session-binding:no-identity:no-session:no-binding:no-approval:no-authority:no-install:no-live",
    );
    let denials = vec![
        "artifact_download_install_affordance_result_receipt_operator_identity_acceptance_denied",
        "artifact_download_install_affordance_result_receipt_operator_identity_recording_denied",
        "artifact_download_install_affordance_result_receipt_operator_identity_persistence_denied",
        "artifact_download_install_affordance_result_receipt_operator_session_acceptance_denied",
        "artifact_download_install_affordance_result_receipt_operator_session_recording_denied",
        "artifact_download_install_affordance_result_receipt_operator_session_persistence_denied",
        "artifact_download_install_affordance_result_receipt_session_binding_recording_denied",
        "artifact_download_install_affordance_result_receipt_identity_hash_recording_denied",
        "artifact_download_install_affordance_result_receipt_session_token_recording_denied",
        "artifact_download_install_affordance_result_receipt_identity_fingerprint_recording_denied",
        "artifact_download_install_affordance_result_receipt_identity_nonce_recording_denied",
        "artifact_download_install_affordance_result_receipt_device_session_recording_denied",
        "artifact_download_install_affordance_result_receipt_identity_session_status_promotion_denied",
        "artifact_download_install_affordance_result_receipt_acceptance_from_identity_session_denied",
        "artifact_download_install_affordance_operator_approval_from_identity_session_denied",
        "artifact_download_install_affordance_release_publication_authority_from_identity_session_denied",
        "artifact_download_install_affordance_activation_authority_from_identity_session_denied",
        "artifact_download_install_affordance_download_install_from_identity_session_denied",
        "artifact_download_install_affordance_install_restart_active_binary_from_identity_session_denied",
        "artifact_download_install_affordance_memory_provider_secret_external_send_from_identity_session_denied",
    ];
    let denied_count = denials.len();
    let report_ready = source_ready
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_surface_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_denied_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_recorded_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_consent_recorded_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_approval_from_intent_consent_derived_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_intent_consent_activation_authority_derived_count",
        ) == 0
        && operator_identity_session_binding_surface_count == 18
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_BINDING_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-operator-identity-session-binding-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-20",
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
        "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_denial_v1",
        "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_mode": "native_route_denied_intent_consent_cannot_create_operator_identity_session_binding_or_authority",
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_gate": source["gate"].clone(),
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_ready": source_ready,
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_report_sha256": source_report_sha256,
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_contract_hash_sha256": source_contract_hash,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_contract_hash_sha256": contract_hash,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_policy_hash_sha256": policy_hash,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
        "minimum_required_samples": 24,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "source_route_wired": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_denial_route_enabled": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_denial_ready": report_ready,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_surface_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_surface_count"),
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_attempt_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_attempt_count"),
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_denied_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_denied_count"),
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_recorded_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_recorded_count"),
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_consent_recorded_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_consent_recorded_count"),
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_surface_count": operator_identity_session_binding_surface_count,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_attempt_count": operator_identity_session_binding_surface_count,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_denied_count": operator_identity_session_binding_surface_count,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_surfaces": operator_identity_session_binding_surfaces,
        "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding": denials,
        "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_count": denied_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
        "allowed_next_actions": [
            {
                "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_denial_gate",
                "status": "allowed_report_only_next_slice",
                "records_operator_identity": false,
                "records_operator_session": false,
                "records_session_binding": false,
                "accepts_replay": false,
                "accepts_cross_binding": false,
                "accepts_cross_session_binding": false,
                "derives_authority": false,
                "renders_download_link": false,
                "emits_install_command": false,
                "installs_or_restarts": false,
                "mutates_active_binary": false,
                "mutates_memory_store": false,
                "writes_kg": false,
                "sends_externally": false
            }
        ],
        }),
    );

    let zero_keys = [
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_session_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_session_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_session_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_binding_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_binding_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_identity_hash_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_token_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_identity_fingerprint_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_identity_nonce_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_device_session_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_identity_status_promoted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_status_promoted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_approval_from_identity_session_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_acceptance_from_identity_session_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_identity_session_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_identity_session_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_identity_session_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_identity_session_service_restart_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_identity_session_active_binary_mutated_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_identity_session_external_send_count",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in &zero_keys {
            report_object.insert((*key).to_string(), serde_json::json!(0));
        }
    }

    let false_keys = [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_session_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_binding_recorded",
        "artifact_download_install_affordance_result_receipt_recorded",
        "artifact_download_install_affordance_result_receipt_persisted",
        "artifact_download_install_affordance_completion_ack_recorded",
        "download_button_rendered",
        "direct_download_url_exposed",
        "package_manager_install_command_rendered",
        "curl_pipe_shell_snippet_rendered",
        "installer_launch_prompt_rendered",
        "auto_update_offer_rendered",
        "external_install_message_sent",
        "telegram_install_message_sent",
        "operator_acceptance_recorded",
        "operator_approval_recorded",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "activation_allowed",
        "activation_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
        "public_release_claimed",
        "public_ga_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "external_send_performed",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in &false_keys {
            report_object.insert((*key).to_string(), serde_json::json!(false));
        }
    }

    let mut side_effects = serde_json::Map::new();
    for key in &surface_false_keys {
        side_effects.insert((*key).to_string(), serde_json::json!(false));
    }
    side_effects.insert("filesystem_written".to_string(), serde_json::json!(false));
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_denial_report();
    let source_report_sha256 = sha256_json_value(&source);
    let source_ready = source
        .get("memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_denial_ready")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let source_u64 = |key: &str| {
        source
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_contract_hash = source
        .get("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_contract_hash_sha256")
        .cloned()
        .unwrap_or_else(|| serde_json::json!(""));
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let surface_false_keys = [
        "operator_identity_replay_requested",
        "operator_session_replay_requested",
        "operator_identity_cross_binding_requested",
        "operator_session_cross_binding_requested",
        "cross_session_binding_requested",
        "operator_identity_replay_accepted",
        "operator_session_replay_accepted",
        "cross_session_binding_accepted",
        "operator_identity_replay_recorded",
        "operator_identity_replay_persisted",
        "operator_session_replay_recorded",
        "operator_session_replay_persisted",
        "cross_session_binding_recorded",
        "cross_session_binding_persisted",
        "identity_hash_replay_recorded",
        "session_token_replay_recorded",
        "identity_fingerprint_cross_binding_recorded",
        "session_token_cross_binding_recorded",
        "identity_nonce_replay_recorded",
        "device_session_rebind_recorded",
        "session_refresh_replay_recorded",
        "identity_status_replay_promoted",
        "session_summary_cross_binding_promoted",
        "operator_approval_from_replay_cross_binding_derived",
        "acceptance_from_replay_cross_binding_recorded",
        "terminal_decision_from_replay_cross_binding_recorded",
        "terminal_status_from_replay_cross_binding_recorded",
        "release_publication_authority_from_replay_cross_binding_derived",
        "activation_authority_from_replay_cross_binding_derived",
        "download_link_from_replay_cross_binding_rendered",
        "install_command_from_replay_cross_binding_rendered",
        "install_from_replay_cross_binding_executed",
        "service_restart_from_replay_cross_binding_performed",
        "launchd_from_replay_cross_binding_mutated",
        "active_binary_from_replay_cross_binding_mutated",
        "result_receipt_from_replay_cross_binding_recorded",
        "result_receipt_from_replay_cross_binding_persisted",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "telegram_send_performed",
        "channel_send_performed",
        "external_send_performed",
        "public_release_claimed",
        "public_ga_claimed",
        "release_artifact_written",
        "public_artifact_written",
    ];
    let surface_specs: [(&str, &str, &str, &[&str]); 18] = [
        (
            "source_operator_identity_session_binding_report_required",
            "blocked_source_identity_session_required_noop",
            "source_operator_identity_session_binding_report_required",
            &["source_operator_identity_session_binding_report_required"],
        ),
        (
            "download_button_identity_hash_replay_claim",
            "blocked_identity_hash_replay_noop",
            "download_button_identity_hash_replay_claim_denied",
            &[
                "operator_identity_replay_requested",
                "identity_hash_replay_requested",
            ],
        ),
        (
            "direct_download_url_session_token_replay_claim",
            "blocked_session_token_replay_noop",
            "direct_download_url_session_token_replay_claim_denied",
            &[
                "operator_session_replay_requested",
                "session_token_replay_requested",
            ],
        ),
        (
            "checksum_identity_fingerprint_cross_binding_claim",
            "blocked_identity_fingerprint_cross_binding_noop",
            "checksum_identity_fingerprint_cross_binding_claim_denied",
            &[
                "operator_identity_cross_binding_requested",
                "identity_fingerprint_cross_binding_requested",
            ],
        ),
        (
            "package_manager_install_command_session_token_cross_binding_claim",
            "blocked_session_token_cross_binding_noop",
            "package_manager_install_command_session_token_cross_binding_claim_denied",
            &[
                "operator_session_cross_binding_requested",
                "session_token_cross_binding_requested",
            ],
        ),
        (
            "curl_pipe_shell_identity_nonce_replay_claim",
            "blocked_identity_nonce_replay_noop",
            "curl_pipe_shell_identity_nonce_replay_claim_denied",
            &[
                "operator_identity_replay_requested",
                "identity_nonce_replay_requested",
            ],
        ),
        (
            "installer_launch_prompt_device_session_rebind_claim",
            "blocked_device_session_rebind_noop",
            "installer_launch_prompt_device_session_rebind_claim_denied",
            &[
                "cross_session_binding_requested",
                "device_session_rebind_requested",
            ],
        ),
        (
            "auto_update_offer_operator_session_refresh_replay_claim",
            "blocked_session_refresh_replay_noop",
            "auto_update_offer_operator_session_refresh_replay_claim_denied",
            &[
                "operator_session_replay_requested",
                "session_refresh_replay_requested",
            ],
        ),
        (
            "release_channel_subscription_identity_status_replay_claim",
            "blocked_identity_status_replay_noop",
            "release_channel_subscription_identity_status_replay_claim_denied",
            &[
                "operator_identity_replay_requested",
                "identity_status_replay_requested",
            ],
        ),
        (
            "update_feed_hint_session_summary_cross_binding_claim",
            "blocked_session_summary_cross_binding_noop",
            "update_feed_hint_session_summary_cross_binding_claim_denied",
            &[
                "operator_session_cross_binding_requested",
                "session_summary_cross_binding_requested",
            ],
        ),
        (
            "package_registry_badge_operator_identity_badge_replay_claim",
            "blocked_identity_badge_replay_noop",
            "package_registry_badge_operator_identity_badge_replay_claim_denied",
            &[
                "operator_identity_replay_requested",
                "operator_identity_badge_replay_requested",
            ],
        ),
        (
            "cdn_mirror_download_session_readback_rebind_claim",
            "blocked_session_readback_rebind_noop",
            "cdn_mirror_download_session_readback_rebind_claim_denied",
            &[
                "cross_session_binding_requested",
                "session_readback_rebind_requested",
            ],
        ),
        (
            "sbom_provenance_notarization_identity_dashboard_cross_binding_claim",
            "blocked_identity_dashboard_cross_binding_noop",
            "sbom_provenance_notarization_identity_dashboard_cross_binding_claim_denied",
            &[
                "operator_identity_cross_binding_requested",
                "identity_dashboard_cross_binding_requested",
            ],
        ),
        (
            "signature_verification_command_channel_session_replay_claim",
            "blocked_channel_session_replay_noop",
            "signature_verification_command_channel_session_replay_claim_denied",
            &[
                "operator_session_replay_requested",
                "channel_session_replay_requested",
            ],
        ),
        (
            "one_click_install_deep_link_operator_identity_approval_replay_claim",
            "blocked_identity_approval_replay_noop",
            "one_click_install_deep_link_operator_identity_approval_replay_claim_denied",
            &[
                "operator_identity_replay_requested",
                "operator_identity_approval_replay_requested",
            ],
        ),
        (
            "external_telegram_install_message_external_identity_session_cross_binding_claim",
            "blocked_external_telegram_identity_session_cross_binding_noop",
            "external_telegram_install_message_external_identity_session_cross_binding_claim_denied",
            &[
                "operator_identity_cross_binding_requested",
                "operator_session_cross_binding_requested",
                "telegram_identity_session_cross_binding_requested",
            ],
        ),
        (
            "release_publication_authority_identity_session_replay_claim",
            "blocked_authority_identity_session_replay_noop",
            "release_publication_authority_identity_session_replay_claim_denied",
            &[
                "operator_identity_replay_requested",
                "operator_session_replay_requested",
                "authority_identity_session_replay_requested",
            ],
        ),
        (
            "activation_live_install_restart_active_binary_session_rebind_claim",
            "blocked_live_session_rebind_noop",
            "activation_live_install_restart_active_binary_session_rebind_claim_denied",
            &[
                "cross_session_binding_requested",
                "live_session_rebind_requested",
                "install_restart_active_binary_session_rebind_requested",
            ],
        ),
    ];
    let operator_identity_session_replay_cross_binding_surfaces = surface_specs
        .iter()
        .map(|(surface, status, reason, extra_true_keys)| {
            let mut surface_report = serde_json::json!({
                "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_surface": surface,
                "source_operator_identity_session_binding_ready": source_ready,
                "artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_attempted": true,
                "operator_identity_session_replay_cross_binding_noop_confirmed": true,
                "operator_identity_session_replay_cross_binding_status": status,
                "reason": reason,
            });
            if let Some(surface_object) = surface_report.as_object_mut() {
                for key in &surface_false_keys {
                    surface_object.insert((*key).to_string(), serde_json::json!(false));
                }
                for key in extra_true_keys.iter() {
                    surface_object.insert((*key).to_string(), serde_json::json!(true));
                }
            }
            surface_report
        })
        .collect::<Vec<_>>();
    let operator_identity_session_replay_cross_binding_surface_count =
        operator_identity_session_replay_cross_binding_surfaces.len();
    let contract_hash = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-replay-cross-binding-denial:native:source={source_report_sha256}:surfaces={operator_identity_session_replay_cross_binding_surface_count}:route_count={}:replay=0:cross_binding=0:authority=0:install=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "artifact-download-install-affordance-result-receipt-operator-identity-session-replay-cross-binding:no-replay:no-cross-session-binding:no-approval:no-authority:no-install:no-live",
    );
    let denials = vec![
        "artifact_download_install_affordance_result_receipt_operator_identity_replay_acceptance_denied",
        "artifact_download_install_affordance_result_receipt_operator_identity_replay_recording_denied",
        "artifact_download_install_affordance_result_receipt_operator_session_replay_acceptance_denied",
        "artifact_download_install_affordance_result_receipt_operator_session_replay_recording_denied",
        "artifact_download_install_affordance_result_receipt_cross_session_binding_acceptance_denied",
        "artifact_download_install_affordance_result_receipt_cross_session_binding_recording_denied",
        "artifact_download_install_affordance_result_receipt_identity_hash_replay_denied",
        "artifact_download_install_affordance_result_receipt_session_token_replay_denied",
        "artifact_download_install_affordance_result_receipt_identity_fingerprint_cross_binding_denied",
        "artifact_download_install_affordance_result_receipt_session_token_cross_binding_denied",
        "artifact_download_install_affordance_result_receipt_identity_nonce_replay_denied",
        "artifact_download_install_affordance_result_receipt_device_session_rebind_denied",
        "artifact_download_install_affordance_result_receipt_identity_session_replay_status_promotion_denied",
        "artifact_download_install_affordance_result_receipt_acceptance_from_replay_cross_binding_denied",
        "artifact_download_install_affordance_operator_approval_from_replay_cross_binding_denied",
        "artifact_download_install_affordance_release_publication_authority_from_replay_cross_binding_denied",
        "artifact_download_install_affordance_activation_authority_from_replay_cross_binding_denied",
        "artifact_download_install_affordance_memory_provider_secret_external_send_from_replay_cross_binding_denied",
    ];
    let denied_count = denials.len();
    let report_ready = source_ready
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_surface_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_denied_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_recorded_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_session_recorded_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_binding_recorded_count",
        ) == 0
        && operator_identity_session_replay_cross_binding_surface_count == 18
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REPLAY_CROSS_BINDING_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-operator-identity-session-replay-cross-binding-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-20",
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
        "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_denial_v1",
        "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_mode": "native_route_denied_identity_session_binding_cannot_be_replayed_or_cross_bound_into_authority",
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_gate": source["gate"].clone(),
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_ready": source_ready,
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_report_sha256": source_report_sha256,
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_contract_hash_sha256": source_contract_hash,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_contract_hash_sha256": contract_hash,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_policy_hash_sha256": policy_hash,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
        "minimum_required_samples": 24,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "source_route_wired": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_denial_route_enabled": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_denial_ready": report_ready,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_surface_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_surface_count"),
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_attempt_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_attempt_count"),
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_denied_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_denied_count"),
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_recorded_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_recorded_count"),
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_session_recorded_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_session_recorded_count"),
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_surface_count": operator_identity_session_replay_cross_binding_surface_count,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_attempt_count": operator_identity_session_replay_cross_binding_surface_count,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_denied_count": operator_identity_session_replay_cross_binding_surface_count,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_surfaces": operator_identity_session_replay_cross_binding_surfaces,
        "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding": denials,
        "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_count": denied_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
        "allowed_next_actions": [
            {
                "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_denial_gate",
                "status": "allowed_report_only_next_slice",
                "records_operator_identity": false,
                "records_operator_session": false,
                "records_session_binding": false,
                "accepts_replay": false,
                "accepts_cross_session_binding": false,
                "records_revocation": false,
                "records_logout": false,
                "derives_authority": false,
                "renders_download_link": false,
                "emits_install_command": false,
                "installs_or_restarts": false,
                "mutates_active_binary": false,
                "mutates_memory_store": false,
                "writes_kg": false,
                "sends_externally": false
            }
        ],
        }),
    );

    let zero_keys = [
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_replay_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_replay_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_replay_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_session_replay_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_session_replay_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_session_replay_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cross_session_binding_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cross_session_binding_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cross_session_binding_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_identity_hash_replay_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_token_replay_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_identity_fingerprint_cross_binding_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_token_cross_binding_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_identity_nonce_replay_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_device_session_rebind_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_refresh_replay_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_identity_status_replay_promoted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_summary_cross_binding_promoted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_approval_from_replay_cross_binding_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_acceptance_from_replay_cross_binding_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_replay_cross_binding_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_replay_cross_binding_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_replay_cross_binding_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_replay_cross_binding_service_restart_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_replay_cross_binding_active_binary_mutated_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_replay_cross_binding_external_send_count",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in &zero_keys {
            report_object.insert((*key).to_string(), serde_json::json!(0));
        }
    }

    let false_keys = [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_replay_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_session_replay_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cross_session_binding_recorded",
        "artifact_download_install_affordance_result_receipt_recorded",
        "artifact_download_install_affordance_result_receipt_persisted",
        "artifact_download_install_affordance_completion_ack_recorded",
        "download_button_rendered",
        "direct_download_url_exposed",
        "package_manager_install_command_rendered",
        "curl_pipe_shell_snippet_rendered",
        "installer_launch_prompt_rendered",
        "auto_update_offer_rendered",
        "external_install_message_sent",
        "telegram_install_message_sent",
        "operator_acceptance_recorded",
        "operator_approval_recorded",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "activation_allowed",
        "activation_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
        "public_release_claimed",
        "public_ga_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "external_send_performed",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in &false_keys {
            report_object.insert((*key).to_string(), serde_json::json!(false));
        }
    }

    let mut side_effects = serde_json::Map::new();
    for key in &surface_false_keys {
        side_effects.insert((*key).to_string(), serde_json::json!(false));
    }
    side_effects.insert("filesystem_written".to_string(), serde_json::json!(false));
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}

include!("distribution_and_install_denials/session_revocation.rs");
