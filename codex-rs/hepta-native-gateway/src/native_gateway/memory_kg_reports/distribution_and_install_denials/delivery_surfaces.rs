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

