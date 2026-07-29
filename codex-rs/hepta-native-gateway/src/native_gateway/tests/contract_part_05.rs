#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_endpoint_blocks_observability_exports()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_QUERY_EXPORT_OBSERVABILITY_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator readiness packet template packet acceptance receipt release publication result receipt terminal distribution delivery receipt query export observability route json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_QUERY_EXPORT_OBSERVABILITY_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-query-export-observability-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_denial_ready"],
        true
    );
    assert_eq!(
        value["source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_ready"],
        true
    );
    assert_eq!(
        value["source_release_publication_result_receipt_terminal_distribution_delivery_receipt_surface_count"],
        18
    );
    assert_eq!(
        value["source_release_publication_result_receipt_terminal_distribution_delivery_receipt_attempt_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_surface_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_attempt_count"],
        18
    );
    for key in [
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_accepted_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_recorded_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_persisted_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_delivered_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_externally_sent_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_sent_count",
        "source_release_publication_result_receipt_downstream_consumer_notified_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_release_publication_authority_derived_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_allowed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_request_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_materialized_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_filesystem_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_delivered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_exposed_count",
        "release_publication_result_receipt_delivery_receipt_query_registered_count",
        "release_publication_result_receipt_delivery_receipt_query_executed_count",
        "release_publication_result_receipt_delivery_receipt_query_result_exposed_count",
        "release_publication_result_receipt_delivery_receipt_search_index_written_count",
        "release_publication_result_receipt_delivery_receipt_export_requested_count",
        "release_publication_result_receipt_delivery_receipt_export_snapshot_recorded_count",
        "release_publication_result_receipt_delivery_receipt_export_file_written_count",
        "release_publication_result_receipt_delivery_receipt_export_stream_opened_count",
        "release_publication_result_receipt_delivery_receipt_observability_metric_recorded_count",
        "release_publication_result_receipt_delivery_receipt_observability_log_recorded_count",
        "release_publication_result_receipt_delivery_receipt_observability_trace_recorded_count",
        "release_publication_result_receipt_delivery_receipt_observability_event_recorded_count",
        "release_publication_result_receipt_delivery_receipt_dashboard_panel_exposed_count",
        "release_publication_result_receipt_delivery_receipt_alert_slo_recorded_count",
        "release_publication_result_receipt_delivery_receipt_operator_readback_exposed_count",
        "release_publication_result_receipt_delivery_receipt_audit_view_exposed_count",
        "release_publication_result_receipt_delivery_receipt_status_evidence_exposed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_acceptance_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_operator_approval_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_activation_command_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_live_execution_allowed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_service_restarted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_active_binary_mutated_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_release_artifact_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_public_artifact_written_count",
    ] {
        assert_eq!(value[key], 0, "{key}");
    }

    let surfaces = value["release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_surfaces"]
            .as_array()
            .expect("packet acceptance receipt release publication result receipt terminal distribution delivery receipt query export observability surfaces");
    assert_eq!(surfaces.len(), 18);
    assert_eq!(
        surfaces[0]["release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_surface"],
        "publication_result_receipt_delivery_receipt_query_registration"
    );
    assert_eq!(surfaces[0]["query_registration_requested"], true);
    for surface in surfaces {
        assert_eq!(surface["query_export_observability_attempted"], true);
        assert_eq!(surface["query_export_observability_allowed"], false);
        assert_eq!(
            surface["query_export_observability_request_accepted"],
            false
        );
        assert_eq!(surface["query_export_observability_accepted"], false);
        assert_eq!(surface["query_export_observability_recorded"], false);
        assert_eq!(surface["query_export_observability_persisted"], false);
        assert_eq!(
            surface["query_export_observability_filesystem_written"],
            false
        );
        assert_eq!(surface["query_export_observability_delivered"], false);
        assert_eq!(surface["query_export_observability_exposed"], false);
        assert_eq!(surface["query_registration_performed"], false);
        assert_eq!(surface["query_execution_performed"], false);
        assert_eq!(surface["query_result_exposed"], false);
        assert_eq!(surface["search_index_written"], false);
        assert_eq!(surface["export_request_accepted"], false);
        assert_eq!(surface["export_snapshot_recorded"], false);
        assert_eq!(surface["export_file_written"], false);
        assert_eq!(surface["export_stream_opened"], false);
        assert_eq!(surface["observability_metric_recorded"], false);
        assert_eq!(surface["observability_log_recorded"], false);
        assert_eq!(surface["observability_trace_recorded"], false);
        assert_eq!(surface["observability_event_recorded"], false);
        assert_eq!(surface["dashboard_panel_exposed"], false);
        assert_eq!(surface["alert_slo_recorded"], false);
        assert_eq!(surface["operator_readback_exposed"], false);
        assert_eq!(surface["audit_view_exposed"], false);
        assert_eq!(surface["delivery_receipt_status_evidence_exposed"], false);
        assert_eq!(surface["release_publication_authority_derived"], false);
        assert_eq!(surface["activation_authority_derived"], false);
        assert_eq!(surface["live_execution_allowed"], false);
        assert_eq!(surface["release_artifact_written"], false);
        assert_eq!(surface["public_artifact_written"], false);
        assert_eq!(surface["query_export_observability_noop_confirmed"], true);
    }

    let denied = value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability"]
            .as_array()
            .expect("packet acceptance receipt release publication result receipt terminal distribution delivery receipt query export observability denials");
    assert_eq!(denied.len(), 34);
    assert_eq!(
        value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_denial_gate"
    );
    for key in [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_filesystem_written",
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
        "external_send_performed",
    ] {
        assert_eq!(value[key], false, "{key}");
    }
    let side_effects = value["side_effects"].as_object().expect(
            "release publication result receipt terminal distribution delivery receipt query export observability side effects",
        );
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_endpoint_blocks_manifest_status_surfaces()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_DISTRIBUTION_ARTIFACT_MANIFEST_STATUS_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator readiness packet template packet acceptance receipt release publication result receipt terminal distribution delivery receipt distribution artifact manifest status route json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_DISTRIBUTION_ARTIFACT_MANIFEST_STATUS_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-distribution-artifact-manifest-status-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_denial_ready"],
        true
    );
    assert_eq!(
        value["source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_ready"],
        true
    );
    assert_eq!(
        value["source_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_surface_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_surface_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_attempt_count"],
        18
    );
    for key in [
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_accepted_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_recorded_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_persisted_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_exposed_count",
        "source_release_publication_result_receipt_delivery_receipt_query_registered_count",
        "source_release_publication_result_receipt_delivery_receipt_export_file_written_count",
        "source_release_publication_result_receipt_delivery_receipt_observability_metric_recorded_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_release_publication_authority_derived_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_activation_authority_derived_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_live_execution_allowed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_allowed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_request_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_materialized_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_filesystem_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_delivered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_exposed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_exposed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_package_manifest_status_exposed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_checksum_index_status_exposed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_metadata_status_exposed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_cdn_artifact_metadata_status_exposed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_update_feed_artifact_metadata_status_exposed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_package_signing_status_exposed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_notarization_status_exposed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_stapling_status_exposed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_provenance_attestation_status_exposed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_sbom_manifest_status_exposed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_digest_manifest_status_exposed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_release_asset_manifest_status_exposed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_installer_package_manifest_status_exposed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_package_channel_manifest_status_exposed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_external_artifact_manifest_status_sent_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_artifact_manifest_status_sent_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_acceptance_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_operator_approval_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_activation_command_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_live_execution_allowed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_service_restarted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_active_binary_mutated_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_release_artifact_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_public_artifact_written_count",
    ] {
        assert_eq!(value[key], 0, "{key}");
    }

    let surfaces = value["release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_surfaces"]
            .as_array()
            .expect("packet acceptance receipt release publication result receipt terminal distribution delivery receipt distribution artifact manifest status surfaces");
    assert_eq!(surfaces.len(), 18);
    assert_eq!(
        surfaces[0]["release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_surface"],
        "delivery_receipt_distribution_artifact_manifest_status"
    );
    assert_eq!(
        surfaces[0]["distribution_artifact_manifest_status_requested"],
        true
    );
    for surface in surfaces {
        assert_eq!(
            surface["distribution_artifact_manifest_status_attempted"],
            true
        );
        assert_eq!(
            surface["distribution_artifact_manifest_status_allowed"],
            false
        );
        assert_eq!(
            surface["distribution_artifact_manifest_status_request_accepted"],
            false
        );
        assert_eq!(
            surface["distribution_artifact_manifest_status_accepted"],
            false
        );
        assert_eq!(
            surface["distribution_artifact_manifest_status_recorded"],
            false
        );
        assert_eq!(
            surface["distribution_artifact_manifest_status_persisted"],
            false
        );
        assert_eq!(
            surface["distribution_artifact_manifest_status_filesystem_written"],
            false
        );
        assert_eq!(
            surface["distribution_artifact_manifest_status_exposed"],
            false
        );
        assert_eq!(surface["distribution_artifact_manifest_exposed"], false);
        assert_eq!(surface["package_manifest_status_exposed"], false);
        assert_eq!(surface["checksum_index_status_exposed"], false);
        assert_eq!(surface["artifact_metadata_status_exposed"], false);
        assert_eq!(surface["package_signing_status_exposed"], false);
        assert_eq!(surface["notarization_status_exposed"], false);
        assert_eq!(surface["provenance_attestation_status_exposed"], false);
        assert_eq!(surface["sbom_manifest_status_exposed"], false);
        assert_eq!(surface["external_artifact_manifest_status_sent"], false);
        assert_eq!(surface["telegram_artifact_manifest_status_sent"], false);
        assert_eq!(surface["release_publication_authority_derived"], false);
        assert_eq!(surface["activation_authority_derived"], false);
        assert_eq!(surface["live_execution_allowed"], false);
        assert_eq!(surface["install_executed"], false);
        assert_eq!(surface["active_binary_mutated"], false);
        assert_eq!(surface["memory_store_write_performed"], false);
        assert_eq!(surface["live_kg_write_performed"], false);
        assert_eq!(surface["provider_invoked"], false);
        assert_eq!(surface["credential_read"], false);
        assert_eq!(
            surface["distribution_artifact_manifest_status_noop_confirmed"],
            true
        );
    }

    let denied = value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status"]
            .as_array()
            .expect("packet acceptance receipt release publication result receipt terminal distribution delivery receipt distribution artifact manifest status denials");
    assert_eq!(denied.len(), 36);
    assert_eq!(
        value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_denial_gate"
    );
    for key in [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_filesystem_written",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_manifest_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_checksum_index_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_metadata_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_signing_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_notarization_status_exposed",
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
    ] {
        assert_eq!(value[key], false, "{key}");
    }
    let side_effects = value["side_effects"].as_object().expect(
            "release publication result receipt terminal distribution delivery receipt distribution artifact manifest status side effects",
        );
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_endpoint_blocks_package_channel_status_exposure()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_PACKAGE_RELEASE_CHANNEL_STATUS_EXPOSURE_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator readiness packet template packet acceptance receipt release publication result receipt terminal distribution delivery receipt package release channel status exposure route json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_PACKAGE_RELEASE_CHANNEL_STATUS_EXPOSURE_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-package-release-channel-status-exposure-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_denial_ready"],
        true
    );
    assert_eq!(
        value["source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_ready"],
        true
    );
    assert_eq!(
        value["source_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_surface_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_surface_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_attempt_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_denied_count"],
        18
    );
    for key in [
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposed_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_public_status_claimed_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_public_release_claimed_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_public_ga_claimed_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_availability_status_exposed_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_queue_status_exposed_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_channel_status_delivered_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_external_status_sent_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_status_sent_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_release_publication_authority_derived_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_activation_authority_derived_count",
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
    ] {
        assert_eq!(value[key], 0, "{key}");
    }

    let surfaces = value["release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_surfaces"]
            .as_array()
            .expect("packet acceptance receipt release publication result receipt terminal distribution delivery receipt package release channel status exposure surfaces");
    assert_eq!(surfaces.len(), 18);
    assert_eq!(
        surfaces[0]["release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_surface"],
        "delivery_receipt_package_index_status"
    );
    assert_eq!(surfaces[0]["package_index_status_requested"], true);
    for surface in surfaces {
        assert_eq!(
            surface["package_release_channel_status_exposure_attempted"],
            true
        );
        for key in [
            "package_release_channel_status_exposure_allowed",
            "package_release_channel_status_exposure_request_accepted",
            "package_release_channel_status_exposure_accepted",
            "package_release_channel_status_exposure_recorded",
            "package_release_channel_status_exposure_persisted",
            "package_release_channel_status_exposure_materialized",
            "package_release_channel_status_exposure_filesystem_written",
            "package_release_channel_status_exposure_delivered",
            "package_release_channel_status_exposed",
            "package_index_status_exposed",
            "package_registry_status_exposed",
            "package_metadata_endpoint_status_exposed",
            "update_feed_status_exposed",
            "cdn_mirror_status_exposed",
            "release_channel_status_exposed",
            "distribution_artifact_status_exposed",
            "artifact_catalog_status_exposed",
            "version_manifest_status_exposed",
            "installer_manifest_status_exposed",
            "checksum_manifest_status_exposed",
            "download_page_status_exposed",
            "release_notes_package_status_exposed",
            "channel_announcement_status_exposed",
            "channel_status_delivered",
            "external_status_sent",
            "telegram_status_sent",
            "release_publication_authority_derived",
            "activation_authority_derived",
            "live_execution_allowed",
            "install_executed",
            "active_binary_mutated",
            "memory_store_write_performed",
            "live_kg_write_performed",
            "provider_invoked",
            "credential_read",
        ] {
            assert_eq!(surface[key], false, "{key}");
        }
        assert_eq!(
            surface["package_release_channel_status_exposure_noop_confirmed"],
            true
        );
    }

    let denied = value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure"]
            .as_array()
            .expect("packet acceptance receipt release publication result receipt terminal distribution delivery receipt package release channel status denials");
    assert_eq!(denied.len(), 34);
    assert_eq!(
        value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_denial_gate"
    );
    for key in [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_index_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_registry_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_update_feed_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_release_channel_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_channel_announcement_status_exposed",
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
    ] {
        assert_eq!(value[key], false, "{key}");
    }
    let side_effects = value["side_effects"].as_object().expect(
            "release publication result receipt terminal distribution delivery receipt package release channel status side effects",
        );
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_endpoint_blocks_signing_notarization_surfaces()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DISTRIBUTION_SIGNING_NOTARIZATION_SURFACE_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator readiness packet template packet acceptance receipt release publication result receipt terminal distribution delivery receipt artifact distribution signing notarization route json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DISTRIBUTION_SIGNING_NOTARIZATION_SURFACE_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-distribution-signing-notarization-surface-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_denial_ready"],
        true
    );
    assert_eq!(
        value["source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_ready"],
        true
    );
    assert_eq!(
        value["source_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_surface_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_attempt_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_denied_count"],
        18
    );
    for key in [
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_exposed_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_signing_status_exposed_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_notarization_status_exposed_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_stapling_status_exposed_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_provenance_attestation_status_exposed_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_sbom_manifest_status_exposed_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_release_publication_authority_derived_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_activation_authority_derived_count",
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
        assert_eq!(value[key], 0, "{key}");
    }

    let surfaces = value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_surfaces"]
            .as_array()
            .expect("packet acceptance receipt release publication result receipt terminal distribution delivery receipt artifact distribution signing notarization surfaces");
    assert_eq!(surfaces.len(), 18);
    assert_eq!(
        surfaces[0]["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface"],
        "delivery_receipt_artifact_signing_execution"
    );
    assert_eq!(surfaces[0]["artifact_signing_requested"], true);
    for surface in surfaces {
        assert_eq!(
            surface["artifact_distribution_signing_notarization_surface_attempted"],
            true
        );
        for key in [
            "artifact_distribution_signing_notarization_surface_allowed",
            "artifact_distribution_signing_notarization_surface_request_accepted",
            "artifact_distribution_signing_notarization_surface_accepted",
            "artifact_distribution_signing_notarization_surface_recorded",
            "artifact_distribution_signing_notarization_surface_persisted",
            "artifact_distribution_signing_notarization_surface_materialized",
            "artifact_distribution_signing_notarization_surface_filesystem_written",
            "artifact_distribution_signing_notarization_surface_delivered",
            "artifact_distribution_signing_notarization_surface_exposed",
            "artifact_distribution_signing_notarization_surface_executed",
            "artifact_signing_executed",
            "package_signing_executed",
            "signature_manifest_written",
            "signature_checksum_bound",
            "notarization_submitted",
            "notarization_ticket_recorded",
            "stapling_executed",
            "installer_signing_executed",
            "provenance_attestation_published",
            "sbom_manifest_published",
            "release_asset_packaged",
            "artifact_bundle_packaged",
            "cdn_artifact_written",
            "update_feed_artifact_written",
            "package_registry_artifact_published",
            "external_package_channel_published",
            "telegram_package_channel_published",
            "public_release_claimed",
            "public_ga_claimed",
            "acceptance_recorded",
            "operator_approval_derived",
            "release_publication_authority_derived",
            "activation_authority_derived",
            "activation_command_derived",
            "live_execution_allowed",
            "activation_performed",
            "install_executed",
            "service_restarted",
            "launchd_mutated",
            "active_binary_mutated",
            "memory_store_write_performed",
            "memory_store_mutated",
            "live_kg_write_performed",
            "provider_invoked",
            "model_invoked",
            "credential_read",
            "secret_file_read",
        ] {
            assert_eq!(surface[key], false, "{key}");
        }
        assert_eq!(
            surface["artifact_distribution_signing_notarization_surface_noop_confirmed"],
            true
        );
    }

    let denied = value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface"]
            .as_array()
            .expect("packet acceptance receipt release publication result receipt terminal distribution delivery receipt artifact distribution signing notarization denials");
    assert_eq!(denied.len(), 34);
    assert_eq!(
        value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_denial_gate"
    );
    for key in [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_executed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_signing_executed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_signing_executed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_signature_manifest_written",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_signature_checksum_bound",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_notarization_submitted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_notarization_ticket_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_stapling_executed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_provenance_attestation_published",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_sbom_manifest_published",
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
        "external_send_performed",
    ] {
        assert_eq!(value[key], false, "{key}");
    }
    let side_effects = value["side_effects"].as_object().expect(
            "release publication result receipt terminal distribution delivery receipt artifact distribution signing notarization side effects",
        );
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_endpoint_blocks_download_and_install_surfaces()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator readiness packet template packet acceptance receipt release publication result receipt terminal distribution delivery receipt artifact download install affordance route json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_denial_ready"],
        true
    );
    assert_eq!(
        value["source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_ready"],
        true
    );
    assert_eq!(
        value["source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_surface_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_attempt_count"],
        18
    );
    for key in [
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_accepted_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_recorded_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_persisted_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_exposed_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_executed_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_signing_executed_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_notarization_submitted_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_stapling_executed_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_provenance_attestation_published_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_sbom_manifest_published_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_release_publication_authority_derived_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_allowed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_request_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_materialized_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_filesystem_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_delivered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_exposed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_published_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_download_button_rendered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_direct_download_url_exposed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_package_manager_install_command_rendered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_curl_pipe_shell_snippet_rendered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_installer_launch_prompt_rendered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_auto_update_offer_rendered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_external_install_message_sent_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_install_message_sent_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_acceptance_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_approval_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_activation_command_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_live_execution_allowed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_service_restarted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_active_binary_mutated_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_release_artifact_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_public_artifact_written_count",
    ] {
        assert_eq!(value[key], 0, "{key}");
    }

    let surfaces = value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_surfaces"]
            .as_array()
            .expect("packet acceptance receipt release publication result receipt terminal distribution delivery receipt artifact download install affordance surfaces");
    assert_eq!(surfaces.len(), 18);
    assert_eq!(
        surfaces[0]["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_surface"],
        "delivery_receipt_artifact_download_button"
    );
    assert_eq!(surfaces[0]["artifact_download_button_requested"], true);
    for surface in surfaces {
        assert_eq!(
            surface["artifact_download_install_affordance_attempted"],
            true
        );
        assert_eq!(
            surface["artifact_download_install_affordance_allowed"],
            false
        );
        assert_eq!(
            surface["artifact_download_install_affordance_request_accepted"],
            false
        );
        assert_eq!(
            surface["artifact_download_install_affordance_accepted"],
            false
        );
        assert_eq!(
            surface["artifact_download_install_affordance_recorded"],
            false
        );
        assert_eq!(
            surface["artifact_download_install_affordance_persisted"],
            false
        );
        assert_eq!(
            surface["artifact_download_install_affordance_exposed"],
            false
        );
        assert_eq!(
            surface["artifact_download_install_affordance_published"],
            false
        );
        assert_eq!(
            surface["artifact_download_install_affordance_executed"],
            false
        );
        assert_eq!(surface["download_button_rendered"], false);
        assert_eq!(surface["direct_download_url_exposed"], false);
        assert_eq!(surface["package_manager_install_command_rendered"], false);
        assert_eq!(surface["curl_pipe_shell_snippet_rendered"], false);
        assert_eq!(surface["installer_launch_prompt_rendered"], false);
        assert_eq!(surface["auto_update_offer_rendered"], false);
        assert_eq!(surface["external_install_message_sent"], false);
        assert_eq!(surface["telegram_install_message_sent"], false);
        assert_eq!(surface["release_publication_authority_derived"], false);
        assert_eq!(surface["activation_authority_derived"], false);
        assert_eq!(surface["live_execution_allowed"], false);
        assert_eq!(surface["install_executed"], false);
        assert_eq!(surface["service_restarted"], false);
        assert_eq!(surface["active_binary_mutated"], false);
        assert_eq!(
            surface["artifact_download_install_affordance_noop_confirmed"],
            true
        );
    }

    let denied = value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance"]
            .as_array()
            .expect("packet acceptance receipt release publication result receipt terminal distribution delivery receipt artifact download install affordance denials");
    assert_eq!(denied.len(), 32);
    assert_eq!(
        value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_no_persistence_gate"
    );
    for key in [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_published",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_executed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_download_button_rendered",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_direct_download_url_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_manager_install_command_rendered",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_curl_pipe_shell_snippet_rendered",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_installer_launch_prompt_rendered",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_auto_update_offer_rendered",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_external_install_message_sent",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_install_message_sent",
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
        "external_send_performed",
    ] {
        assert_eq!(value[key], false, "{key}");
    }
    let side_effects = value["side_effects"].as_object().expect(
            "release publication result receipt terminal distribution delivery receipt artifact download install affordance side effects",
        );
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_endpoint_blocks_persistence()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_NO_PERSISTENCE_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator readiness packet template packet acceptance receipt release publication result receipt terminal distribution delivery receipt artifact download install affordance result receipt no persistence route json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_NO_PERSISTENCE_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-no-persistence-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_no_persistence_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_no_persistence_denial_ready"],
        true
    );
    assert_eq!(
        value["source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_ready"],
        true
    );
    assert_eq!(
        value["source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_surface_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_attempt_count"],
        18
    );
    for key in [
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_allowed_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_recorded_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_persisted_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_executed_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_download_button_rendered_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_direct_download_url_exposed_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_manager_install_command_rendered_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_external_install_message_sent_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_release_publication_authority_derived_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_activation_authority_derived_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_live_execution_allowed_count",
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
    ] {
        assert_eq!(value[key], 0, "{key}");
    }

    let surfaces = value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_surfaces"]
            .as_array()
            .expect("packet acceptance receipt release publication result receipt terminal distribution delivery receipt artifact download install affordance result receipt surfaces");
    assert_eq!(surfaces.len(), 18);
    assert_eq!(
        surfaces[0]["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_surface"],
        "source_artifact_download_install_affordance_report_required"
    );
    for surface in surfaces {
        assert_eq!(
            surface["artifact_download_install_affordance_result_receipt_attempted"],
            true
        );
        assert_eq!(
            surface["artifact_download_install_affordance_result_receipt_allowed"],
            false
        );
        assert_eq!(
            surface["artifact_download_install_affordance_result_receipt_schema_accepted"],
            false
        );
        assert_eq!(
            surface["artifact_download_install_affordance_result_receipt_recorded"],
            false
        );
        assert_eq!(
            surface["artifact_download_install_affordance_result_receipt_persisted"],
            false
        );
        assert_eq!(
            surface["artifact_download_install_affordance_result_receipt_ledger_written"],
            false
        );
        assert_eq!(
            surface["artifact_download_install_affordance_result_receipt_query_registered"],
            false
        );
        assert_eq!(
            surface["artifact_download_install_affordance_result_receipt_observability_recorded"],
            false
        );
        assert_eq!(surface["operator_approval_from_receipt_accepted"], false);
        assert_eq!(
            surface["release_publication_authority_from_receipt_derived"],
            false
        );
        assert_eq!(surface["activation_authority_from_receipt_derived"], false);
        assert_eq!(surface["live_execution_from_receipt_allowed"], false);
        assert_eq!(surface["install_from_receipt_executed"], false);
        assert_eq!(surface["active_binary_from_receipt_mutated"], false);
        assert_eq!(surface["receipt_noop_confirmed"], true);
    }

    let denied = value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt"]
            .as_array()
            .expect("packet acceptance receipt release publication result receipt terminal distribution delivery receipt artifact download install affordance result receipt denials");
    assert_eq!(denied.len(), 30);
    assert_eq!(
        value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_denial_gate"
    );
    for key in [
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
        "release_artifact_written",
        "public_artifact_written",
        "external_send_performed",
    ] {
        assert_eq!(value[key], false, "{key}");
    }
    let side_effects = value["side_effects"].as_object().expect(
            "release publication result receipt terminal distribution delivery receipt artifact download install affordance result receipt side effects",
        );
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_endpoint_blocks_replay_and_authority()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_REPLAY_IDEMPOTENCY_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator readiness packet template packet acceptance receipt release publication result receipt terminal distribution delivery receipt artifact download install affordance result receipt replay idempotency route json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_REPLAY_IDEMPOTENCY_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-replay-idempotency-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_denial_ready"],
        true
    );
    assert_eq!(
        value["source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_no_persistence_ready"],
        true
    );
    assert_eq!(
        value["source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_surface_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_surface_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_attempt_count"],
        18
    );

    for key in [
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_recorded_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_persisted_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_materialized_count",
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
    ] {
        assert_eq!(value[key], 0, "{key}");
    }

    let surfaces = value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_surfaces"]
            .as_array()
            .expect("packet acceptance receipt release publication result receipt terminal distribution delivery receipt artifact download install affordance result receipt replay idempotency surfaces");
    assert_eq!(surfaces.len(), 18);
    assert_eq!(
        surfaces[0]["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_surface"],
        "source_result_receipt_no_persistence_report_required"
    );
    for surface in surfaces {
        assert_eq!(
            surface["artifact_download_install_affordance_result_receipt_replay_requested"],
            true
        );
        assert_eq!(
            surface["artifact_download_install_affordance_result_receipt_replay_allowed"],
            false
        );
        assert_eq!(
            surface["artifact_download_install_affordance_result_receipt_replay_recorded"],
            false
        );
        assert_eq!(
            surface["artifact_download_install_affordance_result_receipt_replay_persisted"],
            false
        );
        assert_eq!(
            surface["artifact_download_install_affordance_result_receipt_replay_performed"],
            false
        );
        assert_eq!(
            surface["artifact_download_install_affordance_result_receipt_duplicate_accepted"],
            false
        );
        assert_eq!(
            surface["artifact_download_install_affordance_result_receipt_idempotency_key_accepted"],
            false
        );
        assert_eq!(
            surface["artifact_download_install_affordance_result_receipt_idempotency_state_persisted"],
            false
        );
        assert_eq!(
            surface["artifact_download_install_affordance_result_receipt_ack_replay_accepted"],
            false
        );
        assert_eq!(
            surface["artifact_download_install_affordance_result_receipt_ledger_replay_accepted"],
            false
        );
        assert_eq!(
            surface["artifact_download_install_affordance_result_receipt_query_replay_accepted"],
            false
        );
        assert_eq!(
            surface["artifact_download_install_affordance_result_receipt_observability_replay_accepted"],
            false
        );
        assert_eq!(
            surface["artifact_download_install_affordance_result_receipt_hash_rebind_accepted"],
            false
        );
        assert_eq!(
            surface["artifact_download_install_affordance_result_receipt_operator_identity_reuse_accepted"],
            false
        );
        assert_eq!(surface["operator_approval_from_replay_accepted"], false);
        assert_eq!(
            surface["release_publication_authority_from_replay_derived"],
            false
        );
        assert_eq!(surface["activation_authority_from_replay_derived"], false);
        assert_eq!(surface["live_execution_from_replay_allowed"], false);
        assert_eq!(surface["install_from_replay_executed"], false);
        assert_eq!(surface["active_binary_from_replay_mutated"], false);
        assert_eq!(surface["receipt_noop_confirmed"], true);
    }

    let denied = value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency"]
            .as_array()
            .expect("packet acceptance receipt release publication result receipt terminal distribution delivery receipt artifact download install affordance result receipt replay idempotency denials");
    assert_eq!(denied.len(), 20);
    assert_eq!(
        value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_denial_gate"
    );
    for key in [
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
        "external_send_performed",
    ] {
        assert_eq!(value[key], false, "{key}");
    }
    let side_effects = value["side_effects"].as_object().expect(
            "release publication result receipt terminal distribution delivery receipt artifact download install affordance result receipt replay idempotency side effects",
        );
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_endpoint_blocks_ordering_and_authority()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_ORDERING_MONOTONICITY_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator readiness packet template packet acceptance receipt release publication result receipt terminal distribution delivery receipt artifact download install affordance result receipt ordering monotonicity route json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_ORDERING_MONOTONICITY_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-ordering-monotonicity-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_denial_ready"],
        true
    );
    assert_eq!(
        value["source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_ready"],
        true
    );
    assert_eq!(
        value["source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_surface_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_surface_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_attempt_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_denied_count"],
        18
    );

    for key in [
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_allowed_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_duplicate_accepted_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_idempotency_state_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_allowed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_sequence_cursor_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_sequence_cursor_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_monotonicity_state_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_monotonicity_state_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_latest_wins_overwrite_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ack_before_noop_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_approval_from_ordering_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_release_publication_authority_from_ordering_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_activation_authority_from_ordering_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_install_from_ordering_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_active_binary_from_ordering_mutated_count",
    ] {
        assert_eq!(value[key], 0, "{key}");
    }

    let surfaces = value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_surfaces"]
            .as_array()
            .expect("packet acceptance receipt release publication result receipt terminal distribution delivery receipt artifact download install affordance result receipt ordering monotonicity surfaces");
    assert_eq!(surfaces.len(), 18);
    assert_eq!(
        surfaces[0]["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_surface"],
        "source_replay_idempotency_report_required"
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["sequence_cursor_recording_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["out_of_order_sequence_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["latest_wins_overwrite_requested"] == true)
            .count(),
        1
    );
    for surface in surfaces {
        assert_eq!(
            surface["artifact_download_install_affordance_result_receipt_ordering_requested"],
            true
        );
        assert_eq!(
            surface["artifact_download_install_affordance_result_receipt_ordering_allowed"],
            false
        );
        assert_eq!(
            surface["artifact_download_install_affordance_result_receipt_sequence_cursor_recorded"],
            false
        );
        assert_eq!(
            surface["artifact_download_install_affordance_result_receipt_monotonicity_state_recorded"],
            false
        );
        assert_eq!(
            surface["artifact_download_install_affordance_result_receipt_latest_wins_overwrite_accepted"],
            false
        );
        assert_eq!(
            surface["artifact_download_install_affordance_result_receipt_ack_before_noop_accepted"],
            false
        );
        assert_eq!(surface["operator_approval_from_ordering_accepted"], false);
        assert_eq!(
            surface["release_publication_authority_from_ordering_derived"],
            false
        );
        assert_eq!(surface["activation_authority_from_ordering_derived"], false);
        assert_eq!(surface["install_from_ordering_executed"], false);
        assert_eq!(surface["active_binary_from_ordering_mutated"], false);
        assert_eq!(surface["receipt_noop_confirmed"], true);
    }

    let denied = value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity"]
            .as_array()
            .expect("packet acceptance receipt release publication result receipt terminal distribution delivery receipt artifact download install affordance result receipt ordering monotonicity denials");
    assert_eq!(denied.len(), 26);
    assert_eq!(
        value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_denial_gate"
    );
    for key in [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_allowed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_sequence_cursor_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_monotonicity_state_persisted",
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
        "external_send_performed",
    ] {
        assert_eq!(value[key], false, "{key}");
    }
    let side_effects = value["side_effects"].as_object().expect(
            "release publication result receipt terminal distribution delivery receipt artifact download install affordance result receipt ordering monotonicity side effects",
        );
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_endpoint_blocks_cancellation_and_authority()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_CANCELLATION_SUPERSESSION_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator readiness packet template packet acceptance receipt release publication result receipt terminal distribution delivery receipt artifact download install affordance result receipt cancellation supersession route json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_CANCELLATION_SUPERSESSION_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-cancellation-supersession-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_denial_ready"],
        true
    );
    assert_eq!(
        value["source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_ready"],
        true
    );
    assert_eq!(
        value["source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_surface_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_surface_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_attempt_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_denied_count"],
        18
    );

    for key in [
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_recorded_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_persisted_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_sequence_cursor_recorded_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_monotonicity_state_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_allowed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_revocation_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_withdrawal_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_supersession_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replacement_receipt_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_tombstone_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_delete_marker_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_latest_replacement_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ack_replacement_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_approval_from_cancellation_supersession_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_release_publication_authority_from_cancellation_supersession_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_activation_authority_from_cancellation_supersession_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_install_from_cancellation_supersession_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_active_binary_from_cancellation_supersession_mutated_count",
    ] {
        assert_eq!(value[key], 0, "{key}");
    }

    let surfaces = value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_surfaces"]
            .as_array()
            .expect("packet acceptance receipt release publication result receipt terminal distribution delivery receipt artifact download install affordance result receipt cancellation supersession surfaces");
    assert_eq!(surfaces.len(), 18);
    assert_eq!(
        surfaces[0]["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_surface"],
        "source_ordering_monotonicity_report_required"
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["replacement_receipt_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["tombstone_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["active_binary_replacement_requested"] == true)
            .count(),
        1
    );
    for surface in surfaces {
        assert_eq!(
            surface["artifact_download_install_affordance_result_receipt_cancellation_supersession_requested"],
            true
        );
        assert_eq!(
            surface["artifact_download_install_affordance_result_receipt_cancellation_supersession_allowed"],
            false
        );
        assert_eq!(
            surface["artifact_download_install_affordance_result_receipt_cancellation_recorded"],
            false
        );
        assert_eq!(
            surface["artifact_download_install_affordance_result_receipt_supersession_recorded"],
            false
        );
        assert_eq!(
            surface["artifact_download_install_affordance_result_receipt_replacement_receipt_recorded"],
            false
        );
        assert_eq!(
            surface["artifact_download_install_affordance_result_receipt_tombstone_recorded"],
            false
        );
        assert_eq!(
            surface["artifact_download_install_affordance_result_receipt_delete_marker_recorded"],
            false
        );
        assert_eq!(
            surface["artifact_download_install_affordance_result_receipt_latest_replacement_accepted"],
            false
        );
        assert_eq!(
            surface["operator_approval_from_cancellation_supersession_accepted"],
            false
        );
        assert_eq!(
            surface["release_publication_authority_from_cancellation_supersession_derived"],
            false
        );
        assert_eq!(
            surface["activation_authority_from_cancellation_supersession_derived"],
            false
        );
        assert_eq!(
            surface["install_from_cancellation_supersession_executed"],
            false
        );
        assert_eq!(
            surface["active_binary_from_cancellation_supersession_mutated"],
            false
        );
        assert_eq!(surface["cancellation_supersession_noop_confirmed"], true);
    }

    let denied = value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession"]
            .as_array()
            .expect("packet acceptance receipt release publication result receipt terminal distribution delivery receipt artifact download install affordance result receipt cancellation supersession denials");
    assert_eq!(denied.len(), 27);
    assert_eq!(
        value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_immutable_evidence_denial_gate"
    );
    for key in [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_allowed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_supersession_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replacement_receipt_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_tombstone_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_delete_marker_recorded",
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
        "external_send_performed",
    ] {
        assert_eq!(value[key], false, "{key}");
    }
    let side_effects = value["side_effects"].as_object().expect(
            "release publication result receipt terminal distribution delivery receipt artifact download install affordance result receipt cancellation supersession side effects",
        );
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_immutable_evidence_endpoint_blocks_evidence_and_authority()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_AUDIT_TRAIL_IMMUTABLE_EVIDENCE_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator readiness packet template packet acceptance receipt release publication result receipt terminal distribution delivery receipt artifact download install affordance result receipt audit trail immutable evidence route json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_AUDIT_TRAIL_IMMUTABLE_EVIDENCE_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-audit-trail-immutable-evidence-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_immutable_evidence_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_immutable_evidence_denial_ready"],
        true
    );
    assert_eq!(
        value["source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_ready"],
        true
    );
    assert_eq!(
        value["source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_surface_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_evidence_surface_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_evidence_attempt_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_evidence_denied_count"],
        18
    );

    for key in [
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_immutable_evidence_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_immutable_evidence_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_immutable_evidence_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_hash_chain_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_merkle_root_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_attestation_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ledger_evidence_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_readback_evidence_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_evidence_acceptance_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_audit_evidence_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_audit_evidence_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_audit_evidence_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_audit_evidence_active_binary_mutated_count",
    ] {
        assert_eq!(value[key], 0, "{key}");
    }

    let surfaces = value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_evidence_surfaces"]
            .as_array()
            .expect("artifact download install affordance result receipt audit evidence surfaces");
    assert_eq!(surfaces.len(), 18);
    assert_eq!(
        surfaces[0]["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_evidence_surface"],
        "source_cancellation_supersession_report_required"
    );
    assert_eq!(
            surfaces
                .iter()
                .filter(|surface| surface["artifact_download_install_affordance_result_receipt_immutable_evidence_requested"] == true)
                .count(),
            3
        );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["active_binary_evidence_requested"] == true)
            .count(),
        1
    );
    for surface in surfaces {
        assert_eq!(surface["audit_or_evidence_attempted"], true);
        assert_eq!(surface["audit_evidence_noop_confirmed"], true);
        assert_eq!(
            surface["artifact_download_install_affordance_result_receipt_audit_trail_recorded"],
            false
        );
        assert_eq!(
            surface["artifact_download_install_affordance_result_receipt_immutable_evidence_recorded"],
            false
        );
        assert_eq!(
            surface["artifact_download_install_affordance_result_receipt_hash_chain_recorded"],
            false
        );
        assert_eq!(
            surface["artifact_download_install_affordance_result_receipt_ledger_evidence_recorded"],
            false
        );
        assert_eq!(
            surface["release_publication_authority_from_audit_evidence_derived"],
            false
        );
        assert_eq!(
            surface["activation_authority_from_audit_evidence_derived"],
            false
        );
        assert_eq!(surface["install_from_audit_evidence_executed"], false);
        assert_eq!(surface["active_binary_from_audit_evidence_mutated"], false);
        assert_eq!(surface["external_send_performed"], false);
    }

    let denied = value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_immutable_evidence"]
            .as_array()
            .expect("artifact download install affordance result receipt audit evidence denials");
    assert_eq!(denied.len(), 28);
    assert_eq!(
        value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_immutable_evidence_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_denial_gate"
    );
    for key in [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_immutable_evidence_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_hash_chain_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ledger_evidence_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_readback_evidence_recorded",
        "artifact_download_install_affordance_result_receipt_recorded",
        "artifact_download_install_affordance_result_receipt_persisted",
        "artifact_download_install_affordance_completion_ack_recorded",
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
        "release_artifact_written",
        "public_artifact_written",
        "external_send_performed",
    ] {
        assert_eq!(value[key], false, "{key}");
    }
    let side_effects = value["side_effects"].as_object().expect(
            "release publication result receipt terminal distribution delivery receipt artifact download install affordance result receipt audit evidence side effects",
        );
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_endpoint_blocks_lifecycle_mutation()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_RETENTION_EXPIRY_GARBAGE_COLLECTION_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator readiness packet template packet acceptance receipt release publication result receipt terminal distribution delivery receipt artifact download install affordance result receipt retention expiry garbage collection route json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_RETENTION_EXPIRY_GARBAGE_COLLECTION_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-retention-expiry-garbage-collection-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_denial_ready"],
        true
    );
    assert_eq!(
        value["source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_immutable_evidence_ready"],
        true
    );
    assert_eq!(
        value["source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_evidence_surface_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_surface_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_attempt_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_denied_count"],
        18
    );

    for key in [
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_policy_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_policy_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_policy_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_expiry_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_expiry_scheduler_registered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_garbage_collection_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_gc_queue_enqueued_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_garbage_collection_scan_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_delete_marker_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_tombstone_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_archive_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_compaction_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_evidence_retention_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_hash_attestation_retention_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_retention_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_retention_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_retention_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_retention_active_binary_mutated_count",
    ] {
        assert_eq!(value[key], 0, "{key}");
    }

    let surfaces = value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_surfaces"]
            .as_array()
            .expect("artifact download install affordance result receipt retention expiry garbage collection surfaces");
    assert_eq!(surfaces.len(), 18);
    assert_eq!(
        surfaces[0]["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_surface"],
        "source_audit_trail_immutable_evidence_report_required"
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["gc_queue_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["audit_evidence_retention_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["hash_attestation_retention_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["install_gc_requested"] == true)
            .count(),
        1
    );
    for surface in surfaces {
        assert_eq!(
            surface["artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_attempted"],
            true
        );
        assert_eq!(
            surface["retention_expiry_garbage_collection_noop_confirmed"],
            true
        );
        for key in [
            "artifact_download_install_affordance_result_receipt_retention_policy_recorded",
            "artifact_download_install_affordance_result_receipt_ttl_update_recorded",
            "artifact_download_install_affordance_result_receipt_expiry_recorded",
            "artifact_download_install_affordance_result_receipt_garbage_collection_scan_performed",
            "artifact_download_install_affordance_result_receipt_gc_queue_enqueued",
            "artifact_download_install_affordance_result_receipt_delete_marker_recorded",
            "artifact_download_install_affordance_result_receipt_tombstone_recorded",
            "artifact_download_install_affordance_result_receipt_archive_written",
            "artifact_download_install_affordance_result_receipt_compaction_performed",
            "artifact_download_install_affordance_result_receipt_audit_evidence_retention_recorded",
            "artifact_download_install_affordance_result_receipt_hash_attestation_retention_recorded",
            "artifact_download_install_affordance_result_receipt_recorded",
            "artifact_download_install_affordance_result_receipt_persisted",
            "release_publication_authority_from_retention_expiry_garbage_collection_derived",
            "activation_authority_from_retention_expiry_garbage_collection_derived",
            "install_from_retention_expiry_garbage_collection_executed",
            "service_restart_from_retention_expiry_garbage_collection_performed",
            "active_binary_from_retention_expiry_garbage_collection_mutated",
            "external_send_performed",
        ] {
            assert_eq!(surface[key], false, "{key}");
        }
    }

    let denied = value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection"]
            .as_array()
            .expect("artifact download install affordance result receipt retention denials");
    assert_eq!(denied.len(), 33);
    assert_eq!(
        value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_denial_gate"
    );
    for key in [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_policy_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_expiry_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_garbage_collection_scan_performed",
        "artifact_download_install_affordance_result_receipt_recorded",
        "artifact_download_install_affordance_result_receipt_persisted",
        "artifact_download_install_affordance_completion_ack_recorded",
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
        "release_artifact_written",
        "public_artifact_written",
        "external_send_performed",
    ] {
        assert_eq!(value[key], false, "{key}");
    }
    let side_effects = value["side_effects"].as_object().expect(
            "release publication result receipt terminal distribution delivery receipt artifact download install affordance result receipt retention side effects",
        );
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_endpoint_blocks_views()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_EXPORT_QUERY_OBSERVABILITY_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator readiness packet template packet acceptance receipt release publication result receipt terminal distribution delivery receipt artifact download install affordance result receipt export query observability route json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_EXPORT_QUERY_OBSERVABILITY_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-export-query-observability-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_denial_ready"],
        true
    );
    assert_eq!(
        value["source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_ready"],
        true
    );
    assert_eq!(
        value["source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_surface_count"],
        18
    );
    assert_eq!(
        value["source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_policy_recorded_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_surface_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_attempt_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_denied_count"],
        18
    );

    for key in [
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_query_registered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_query_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_query_result_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_search_index_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_snapshot_recorded_count",
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
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_export_query_observability_active_binary_mutated_count",
    ] {
        assert_eq!(value[key], 0, "{key}");
    }

    let surfaces = value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_surfaces"]
            .as_array()
            .expect("artifact download install affordance result receipt export query observability surfaces");
    assert_eq!(surfaces.len(), 18);
    assert_eq!(
        surfaces[0]["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_surface"],
        "source_retention_expiry_garbage_collection_report_required"
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["export_file_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["trace_observability_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["audit_view_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["install_view_requested"] == true)
            .count(),
        1
    );
    for surface in surfaces {
        assert_eq!(
            surface["artifact_download_install_affordance_result_receipt_export_query_observability_attempted"],
            true
        );
        assert_eq!(surface["export_query_observability_noop_confirmed"], true);
        for key in [
            "artifact_download_install_affordance_result_receipt_query_registered",
            "artifact_download_install_affordance_result_receipt_query_executed",
            "artifact_download_install_affordance_result_receipt_query_result_recorded",
            "artifact_download_install_affordance_result_receipt_search_index_recorded",
            "artifact_download_install_affordance_result_receipt_export_accepted",
            "artifact_download_install_affordance_result_receipt_export_snapshot_recorded",
            "artifact_download_install_affordance_result_receipt_export_file_written",
            "artifact_download_install_affordance_result_receipt_export_stream_opened",
            "artifact_download_install_affordance_result_receipt_observability_metric_recorded",
            "artifact_download_install_affordance_result_receipt_observability_log_recorded",
            "artifact_download_install_affordance_result_receipt_observability_trace_recorded",
            "artifact_download_install_affordance_result_receipt_observability_event_recorded",
            "artifact_download_install_affordance_result_receipt_dashboard_panel_recorded",
            "artifact_download_install_affordance_result_receipt_operator_summary_recorded",
            "artifact_download_install_affordance_result_receipt_readback_surface_recorded",
            "artifact_download_install_affordance_result_receipt_audit_view_recorded",
            "artifact_download_install_affordance_result_receipt_recorded",
            "artifact_download_install_affordance_result_receipt_persisted",
            "release_publication_authority_from_export_query_observability_derived",
            "activation_authority_from_export_query_observability_derived",
            "install_from_export_query_observability_executed",
            "service_restart_from_export_query_observability_performed",
            "active_binary_from_export_query_observability_mutated",
            "external_send_performed",
        ] {
            assert_eq!(surface[key], false, "{key}");
        }
    }

    let denied = value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability"]
            .as_array()
            .expect("artifact download install affordance result receipt export query observability denials");
    assert_eq!(denied.len(), 29);
    assert_eq!(
        value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_non_persistence_denial_gate"
    );
    for key in [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_query_registered",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_query_executed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_file_written",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_observability_metric_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_observability_trace_recorded",
        "artifact_download_install_affordance_result_receipt_recorded",
        "artifact_download_install_affordance_result_receipt_persisted",
        "artifact_download_install_affordance_result_receipt_exported",
        "artifact_download_install_affordance_result_receipt_query_registered",
        "artifact_download_install_affordance_result_receipt_observability_recorded",
        "artifact_download_install_affordance_completion_ack_recorded",
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
        "release_artifact_written",
        "public_artifact_written",
        "external_send_performed",
    ] {
        assert_eq!(value[key], false, "{key}");
    }
    let side_effects = value["side_effects"].as_object().expect(
            "release publication result receipt terminal distribution delivery receipt artifact download install affordance result receipt export query observability side effects",
        );
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_endpoint_blocks_delivery()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_FACING_SUMMARY_BRIEFING_NON_PERSISTENCE_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator readiness packet template packet acceptance receipt release publication result receipt terminal distribution delivery receipt artifact download install affordance result receipt operator-facing summary briefing route json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_FACING_SUMMARY_BRIEFING_NON_PERSISTENCE_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-operator-facing-summary-briefing-non-persistence-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_non_persistence_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready"],
        true
    );
    assert_eq!(
        value["source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_ready"],
        true
    );
    assert_eq!(
        value["source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_surface_count"],
        18
    );
    assert_eq!(
        value["source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_query_registered_count"],
        0
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_surface_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_attempt_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_denied_count"],
        18
    );

    for key in [
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
    ] {
        assert_eq!(value[key], 0, "{key}");
    }

    let surfaces = value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_surfaces"]
            .as_array()
            .expect("artifact download install affordance result receipt operator-facing summary briefing surfaces");
    assert_eq!(surfaces.len(), 18);
    assert_eq!(
        surfaces[0]["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_surface"],
        "source_export_query_observability_report_required"
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["operator_summary_requested"] == true)
            .count(),
        8
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["operator_briefing_requested"] == true)
            .count(),
        9
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["telegram_delivery_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| {
                surface["live_install_restart_active_binary_briefing_requested"] == true
            })
            .count(),
        1
    );
    for surface in surfaces {
        assert_eq!(
            surface["artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_attempted"],
            true
        );
        assert_eq!(surface["operator_summary_briefing_noop_confirmed"], true);
        for key in [
            "operator_summary_recorded",
            "operator_summary_persisted",
            "operator_summary_filesystem_written",
            "operator_summary_delivered",
            "operator_briefing_recorded",
            "operator_briefing_persisted",
            "operator_briefing_filesystem_written",
            "operator_briefing_delivered",
            "readback_digest_recorded",
            "final_note_recorded",
            "status_banner_recorded",
            "dashboard_annotation_recorded",
            "notification_preview_recorded",
            "timeline_entry_recorded",
            "audit_narrative_recorded",
            "privacy_review_narrative_recorded",
            "alert_explanation_recorded",
            "slo_report_recorded",
            "completion_ack_from_summary_recorded",
            "operator_acceptance_from_summary_recorded",
            "operator_approval_from_summary_derived",
            "release_publication_authority_from_summary_derived",
            "activation_authority_from_summary_derived",
            "download_link_from_summary_rendered",
            "install_command_from_summary_rendered",
            "install_from_summary_executed",
            "service_restart_from_summary_performed",
            "active_binary_from_summary_mutated",
            "external_send_performed",
        ] {
            assert_eq!(surface[key], false, "{key}");
        }
    }

    let denied = value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing"]
            .as_array()
            .expect("artifact download install affordance result receipt operator-facing summary briefing denials");
    assert_eq!(denied.len(), 28);
    assert_eq!(
        value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_non_acceptance_denial_gate"
    );
    for key in [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_summary_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_summary_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_briefing_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_briefing_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_readback_digest_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_note_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_status_banner_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_dashboard_annotation_recorded",
        "artifact_download_install_affordance_result_receipt_recorded",
        "artifact_download_install_affordance_result_receipt_persisted",
        "artifact_download_install_affordance_result_receipt_exported",
        "artifact_download_install_affordance_result_receipt_query_registered",
        "artifact_download_install_affordance_result_receipt_observability_recorded",
        "artifact_download_install_affordance_completion_ack_recorded",
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
        "release_artifact_written",
        "public_artifact_written",
        "external_send_performed",
    ] {
        assert_eq!(value[key], false, "{key}");
    }
    let side_effects = value["side_effects"].as_object().expect(
            "release publication result receipt terminal distribution delivery receipt artifact download install affordance result receipt operator-facing summary briefing side effects",
        );
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_endpoint_blocks_acknowledgement_and_authority()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_FINAL_OPERATOR_ACKNOWLEDGEMENT_NON_ACCEPTANCE_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator readiness packet template packet acceptance receipt release publication result receipt terminal distribution delivery receipt artifact download install affordance result receipt final operator acknowledgement route json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_FINAL_OPERATOR_ACKNOWLEDGEMENT_NON_ACCEPTANCE_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-final-operator-acknowledgement-non-acceptance-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_non_acceptance_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready"],
        true
    );
    assert_eq!(
        value["source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_ready"],
        true
    );
    assert_eq!(
        value["source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_surface_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_surface_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_attempt_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_denied_count"],
        18
    );

    for key in [
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_summary_recorded_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_briefing_recorded_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_summary_briefing_release_publication_authority_derived_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_summary_briefing_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_filesystem_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_delivered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_received_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_confirmed_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_channel_acknowledgement_delivered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_external_acknowledgement_sent_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_final_acknowledgement_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_final_acknowledgement_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_final_acknowledgement_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_final_acknowledgement_active_binary_mutated_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_final_acknowledgement_external_send_count",
    ] {
        assert_eq!(value[key], 0, "{key}");
    }

    let surfaces = value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_surfaces"]
            .as_array()
            .expect("artifact download install affordance result receipt final operator acknowledgement surfaces");
    assert_eq!(surfaces.len(), 18);
    assert_eq!(
        surfaces[0]["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_surface"],
        "source_operator_facing_summary_briefing_report_required"
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["final_operator_acknowledgement_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["telegram_acknowledgement_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| {
                surface["install_restart_active_binary_acknowledgement_requested"] == true
            })
            .count(),
        1
    );
    for surface in surfaces {
        assert_eq!(
            surface["artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_attempted"],
            true
        );
        assert_eq!(
            surface["final_operator_acknowledgement_noop_confirmed"],
            true
        );
        for key in [
            "final_operator_acknowledgement_accepted",
            "final_operator_acknowledgement_recorded",
            "final_operator_acknowledgement_persisted",
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
            "channel_acknowledgement_delivered",
            "external_acknowledgement_sent",
            "telegram_acknowledgement_sent",
            "operator_approval_from_acknowledgement_derived",
            "release_publication_authority_from_acknowledgement_derived",
            "activation_authority_from_acknowledgement_derived",
            "install_from_acknowledgement_executed",
            "service_restart_from_acknowledgement_performed",
            "active_binary_from_acknowledgement_mutated",
            "memory_store_write_performed",
            "memory_store_mutated",
            "live_kg_write_performed",
            "provider_invoked",
            "model_invoked",
            "credential_read",
            "secret_file_read",
            "external_send_performed",
        ] {
            assert_eq!(surface[key], false, "{key}");
        }
    }

    let denied = value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement"]
            .as_array()
            .expect("artifact download install affordance result receipt final operator acknowledgement denials");
    assert_eq!(denied.len(), 23);
    assert_eq!(
        value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_denial_gate"
    );
    for key in [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_received_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_confirmed_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_read_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_seen_recorded",
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
        "release_artifact_written",
        "public_artifact_written",
        "external_send_performed",
    ] {
        assert_eq!(value[key], false, "{key}");
    }
    let side_effects = value["side_effects"].as_object().expect(
            "release publication result receipt terminal distribution delivery receipt artifact download install affordance result receipt final operator acknowledgement side effects",
        );
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_endpoint_blocks_status_and_authority()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_TERMINAL_DECISION_STATUS_PROMOTION_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator readiness packet template packet acceptance receipt release publication result receipt terminal distribution delivery receipt artifact download install affordance result receipt terminal decision status route json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_TERMINAL_DECISION_STATUS_PROMOTION_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-terminal-decision-status-promotion-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_denial_ready"],
        true
    );
    assert_eq!(
        value["source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_ready"],
        true
    );
    assert_eq!(
        value["source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_surface_count"],
        18
    );
    assert_eq!(
        value["source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_denied_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_surface_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_attempt_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_denied_count"],
        18
    );

    for key in [
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_recorded_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_status_acknowledgement_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_filesystem_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_delivered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_status_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_status_persisted_count",
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
    ] {
        assert_eq!(value[key], 0, "{key}");
    }

    let surfaces = value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_surfaces"]
            .as_array()
            .expect("artifact download install affordance result receipt terminal decision status promotion surfaces");
    assert_eq!(surfaces.len(), 18);
    assert_eq!(
        surfaces[0]["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_surface"],
        "source_final_operator_acknowledgement_report_required"
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["terminal_decision_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["status_promotion_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["telegram_decision_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["install_restart_active_binary_status_requested"] == true)
            .count(),
        1
    );
    for surface in surfaces {
        assert_eq!(
            surface["artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_attempted"],
            true
        );
        assert_eq!(
            surface["terminal_decision_status_promotion_noop_confirmed"],
            true
        );
        for key in [
            "terminal_decision_accepted",
            "terminal_decision_recorded",
            "terminal_decision_persisted",
            "terminal_decision_filesystem_written",
            "terminal_decision_delivered",
            "terminal_status_recorded",
            "terminal_status_persisted",
            "status_promotion_recorded",
            "delivery_status_promoted",
            "operator_acknowledgement_status_promoted",
            "completion_status_promoted",
            "summary_status_promoted",
            "briefing_status_promoted",
            "readback_status_promoted",
            "dashboard_status_promoted",
            "notification_status_promoted",
            "channel_decision_delivered",
            "external_decision_sent",
            "telegram_decision_sent",
            "operator_approval_from_terminal_status_derived",
            "release_publication_authority_from_terminal_decision_derived",
            "activation_authority_from_terminal_status_derived",
            "install_from_terminal_status_executed",
            "service_restart_from_terminal_status_performed",
            "active_binary_from_terminal_status_mutated",
            "memory_store_write_performed",
            "memory_store_mutated",
            "live_kg_write_performed",
            "provider_invoked",
            "model_invoked",
            "credential_read",
            "secret_file_read",
            "external_send_performed",
        ] {
            assert_eq!(surface[key], false, "{key}");
        }
    }

    let denied = value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion"]
            .as_array()
            .expect("artifact download install affordance result receipt terminal decision status promotion denials");
    assert_eq!(denied.len(), 21);
    assert_eq!(
        value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_denial_gate"
    );
    for key in [
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
    ] {
        assert_eq!(value[key], false, "{key}");
    }
    let side_effects = value["side_effects"].as_object().expect(
            "release publication result receipt terminal distribution delivery receipt artifact download install affordance result receipt terminal decision status promotion side effects",
        );
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_endpoint_blocks_reconfirmation_and_authority()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_INTENT_CONSENT_RECONFIRMATION_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator readiness packet template packet acceptance receipt release publication result receipt terminal distribution delivery receipt artifact download install affordance result receipt operator intent consent route json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_INTENT_CONSENT_RECONFIRMATION_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-operator-intent-consent-reconfirmation-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_denial_ready"],
        true
    );
    assert_eq!(
        value["source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_ready"],
        true
    );
    assert_eq!(
        value["source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_surface_count"],
        18
    );
    assert_eq!(
        value["source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_denied_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_surface_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_attempt_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_denied_count"],
        18
    );

    for key in [
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_recorded_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_status_promotion_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_reconfirmed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_consent_reconfirmed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_consent_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_consent_persisted_count",
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
    ] {
        assert_eq!(value[key], 0, "{key}");
    }

    let surfaces = value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_surfaces"]
            .as_array()
            .expect("artifact download install affordance result receipt operator intent consent surfaces");
    assert_eq!(surfaces.len(), 18);
    assert_eq!(
        surfaces[0]["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_surface"],
        "source_terminal_decision_status_promotion_report_required"
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["operator_intent_reconfirmation_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["operator_consent_reconfirmation_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["telegram_consent_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["install_restart_active_binary_consent_requested"] == true)
            .count(),
        1
    );
    for surface in surfaces {
        assert_eq!(
            surface["artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_attempted"],
            true
        );
        assert_eq!(
            surface["operator_intent_consent_reconfirmation_noop_confirmed"],
            true
        );
        for key in [
            "operator_intent_reconfirmed",
            "operator_consent_reconfirmed",
            "operator_intent_recorded",
            "operator_intent_persisted",
            "operator_consent_recorded",
            "operator_consent_persisted",
            "consent_reconfirmation_recorded",
            "consent_reconfirmation_persisted",
            "intent_hash_recorded",
            "consent_token_recorded",
            "intent_nonce_recorded",
            "double_confirm_recorded",
            "explicit_intent_status_promoted",
            "explicit_consent_status_promoted",
            "operator_approval_from_intent_consent_derived",
            "release_publication_authority_from_intent_consent_derived",
            "activation_authority_from_intent_consent_derived",
            "install_from_intent_consent_executed",
            "service_restart_from_intent_consent_performed",
            "active_binary_from_intent_consent_mutated",
            "memory_store_write_performed",
            "memory_store_mutated",
            "live_kg_write_performed",
            "provider_invoked",
            "model_invoked",
            "credential_read",
            "secret_file_read",
            "external_send_performed",
        ] {
            assert_eq!(surface[key], false, "{key}");
        }
    }

    let denied = value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation"]
            .as_array()
            .expect("artifact download install affordance result receipt operator intent consent denials");
    assert_eq!(denied.len(), 19);
    assert_eq!(
        value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_denial_gate"
    );
    for key in [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_consent_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_consent_reconfirmation_recorded",
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
        "release_artifact_written",
        "public_artifact_written",
        "external_send_performed",
    ] {
        assert_eq!(value[key], false, "{key}");
    }
    let side_effects = value["side_effects"].as_object().expect(
            "release publication result receipt terminal distribution delivery receipt artifact download install affordance result receipt operator intent consent side effects",
        );
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_endpoint_blocks_binding_and_authority()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_BINDING_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator readiness packet template packet acceptance receipt release publication result receipt terminal distribution delivery receipt artifact download install affordance result receipt operator identity session route json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_BINDING_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-operator-identity-session-binding-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_denial_ready"],
        true
    );
    assert_eq!(
        value["source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_ready"],
        true
    );
    assert_eq!(
        value["source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_surface_count"],
        18
    );
    assert_eq!(
        value["source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_denied_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_surface_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_attempt_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_denied_count"],
        18
    );

    for key in [
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_recorded_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_consent_recorded_count",
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
    ] {
        assert_eq!(value[key], 0, "{key}");
    }

    let surfaces = value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_surfaces"]
            .as_array()
            .expect("artifact download install affordance result receipt operator identity session surfaces");
    assert_eq!(surfaces.len(), 18);
    assert_eq!(
        surfaces[0]["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_surface"],
        "source_operator_intent_consent_reconfirmation_report_required"
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["operator_identity_binding_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["operator_session_binding_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["telegram_identity_session_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["install_restart_active_binary_session_requested"] == true)
            .count(),
        1
    );
    for surface in surfaces {
        assert_eq!(
            surface["artifact_download_install_affordance_result_receipt_operator_identity_session_binding_attempted"],
            true
        );
        assert_eq!(
            surface["operator_identity_session_binding_noop_confirmed"],
            true
        );
        for key in [
            "operator_identity_accepted",
            "operator_identity_recorded",
            "operator_identity_persisted",
            "operator_session_accepted",
            "operator_session_recorded",
            "operator_session_persisted",
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
            "release_publication_authority_from_identity_session_derived",
            "activation_authority_from_identity_session_derived",
            "install_from_identity_session_executed",
            "service_restart_from_identity_session_performed",
            "active_binary_from_identity_session_mutated",
            "memory_store_write_performed",
            "memory_store_mutated",
            "live_kg_write_performed",
            "provider_invoked",
            "model_invoked",
            "credential_read",
            "secret_file_read",
            "external_send_performed",
        ] {
            assert_eq!(surface[key], false, "{key}");
        }
    }

    let denied = value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding"]
            .as_array()
            .expect("artifact download install affordance result receipt operator identity session denials");
    assert_eq!(denied.len(), 20);
    assert_eq!(
        value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_denial_gate"
    );
    for key in [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_session_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_binding_recorded",
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
        "release_artifact_written",
        "public_artifact_written",
        "external_send_performed",
    ] {
        assert_eq!(value[key], false, "{key}");
    }
    let side_effects = value["side_effects"].as_object().expect(
            "release publication result receipt terminal distribution delivery receipt artifact download install affordance result receipt operator identity session side effects",
        );
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_endpoint_blocks_replay_and_cross_binding_authority()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REPLAY_CROSS_BINDING_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator readiness packet template packet acceptance receipt release publication result receipt terminal distribution delivery receipt artifact download install affordance result receipt operator identity session replay cross binding route json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REPLAY_CROSS_BINDING_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-operator-identity-session-replay-cross-binding-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_denial_ready"],
        true
    );
    assert_eq!(
        value["source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_ready"],
        true
    );
    assert_eq!(
        value["source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_surface_count"],
        18
    );
    assert_eq!(
        value["source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_denied_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_surface_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_attempt_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_denied_count"],
        18
    );

    for key in [
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_replay_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_replay_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_session_replay_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_session_replay_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cross_session_binding_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cross_session_binding_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_identity_hash_replay_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_token_replay_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_identity_fingerprint_cross_binding_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_token_cross_binding_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_identity_nonce_replay_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_device_session_rebind_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_approval_from_replay_cross_binding_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_replay_cross_binding_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_replay_cross_binding_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_replay_cross_binding_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_replay_cross_binding_service_restart_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_replay_cross_binding_active_binary_mutated_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_replay_cross_binding_external_send_count",
    ] {
        assert_eq!(value[key], 0, "{key}");
    }

    let surfaces = value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_surfaces"]
            .as_array()
            .expect("artifact download install affordance result receipt operator identity session replay cross binding surfaces");
    assert_eq!(surfaces.len(), 18);
    assert_eq!(
        surfaces[0]["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_surface"],
        "source_operator_identity_session_binding_report_required"
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["operator_identity_replay_requested"] == true)
            .count(),
        6
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["operator_session_replay_requested"] == true)
            .count(),
        4
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["cross_session_binding_requested"] == true)
            .count(),
        3
    );
    for surface in surfaces {
        assert_eq!(
            surface["artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_attempted"],
            true
        );
        assert_eq!(
            surface["operator_identity_session_replay_cross_binding_noop_confirmed"],
            true
        );
        for key in [
            "operator_identity_replay_accepted",
            "operator_session_replay_accepted",
            "cross_session_binding_accepted",
            "operator_identity_replay_recorded",
            "operator_session_replay_recorded",
            "cross_session_binding_recorded",
            "identity_hash_replay_recorded",
            "session_token_replay_recorded",
            "identity_fingerprint_cross_binding_recorded",
            "session_token_cross_binding_recorded",
            "identity_nonce_replay_recorded",
            "device_session_rebind_recorded",
            "operator_approval_from_replay_cross_binding_derived",
            "release_publication_authority_from_replay_cross_binding_derived",
            "activation_authority_from_replay_cross_binding_derived",
            "install_from_replay_cross_binding_executed",
            "service_restart_from_replay_cross_binding_performed",
            "active_binary_from_replay_cross_binding_mutated",
            "memory_store_write_performed",
            "memory_store_mutated",
            "live_kg_write_performed",
            "provider_invoked",
            "model_invoked",
            "credential_read",
            "secret_file_read",
            "external_send_performed",
        ] {
            assert_eq!(surface[key], false, "{key}");
        }
    }

    let denied = value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding"]
            .as_array()
            .expect("artifact download install affordance result receipt operator identity session replay cross binding denials");
    assert_eq!(denied.len(), 18);
    assert_eq!(
        value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_denial_gate"
    );
    for key in [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_replay_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_session_replay_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cross_session_binding_recorded",
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
        "release_artifact_written",
        "public_artifact_written",
        "external_send_performed",
    ] {
        assert_eq!(value[key], false, "{key}");
    }
    let side_effects = value["side_effects"].as_object().expect(
            "release publication result receipt terminal distribution delivery receipt artifact download install affordance result receipt operator identity session replay cross binding side effects",
        );
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_endpoint_blocks_revocation_logout_and_session_lifecycle_authority()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator readiness packet template packet acceptance receipt release publication result receipt terminal distribution delivery receipt artifact download install affordance result receipt operator identity session revocation logout route json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_denial_ready"],
        true
    );
    assert_eq!(
        value["source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_ready"],
        true
    );
    assert_eq!(
        value["source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_surface_count"],
        18
    );
    assert_eq!(
        value["source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_denied_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_surface_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_attempt_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_denied_count"],
        18
    );

    for key in [
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_revocation_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_revocation_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_session_logout_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_session_logout_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_revocation_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_logout_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_identity_invalidation_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_revocation_token_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_logout_nonce_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_device_session_logout_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_approval_from_revocation_logout_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_service_restart_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_active_binary_mutated_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_external_send_count",
    ] {
        assert_eq!(value[key], 0, "{key}");
    }

    let surfaces = value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_surfaces"]
            .as_array()
            .expect("artifact download install affordance result receipt operator identity session revocation logout surfaces");
    assert_eq!(surfaces.len(), 18);
    assert_eq!(
        surfaces[0]["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_surface"],
        "source_operator_identity_session_replay_cross_binding_report_required"
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["operator_identity_revocation_requested"] == true)
            .count(),
        8
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["operator_session_logout_requested"] == true)
            .count(),
        8
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["session_revocation_requested"] == true)
            .count(),
        3
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(
                |surface| surface["telegram_identity_session_logout_revocation_requested"] == true
            )
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(
                |surface| surface["install_restart_active_binary_session_revocation_requested"]
                    == true
            )
            .count(),
        1
    );
    for surface in surfaces {
        assert_eq!(
            surface["artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_attempted"],
            true
        );
        assert_eq!(
            surface["operator_identity_session_revocation_logout_noop_confirmed"],
            true
        );
        for key in [
            "operator_identity_revocation_accepted",
            "operator_identity_revocation_recorded",
            "operator_identity_revocation_persisted",
            "operator_session_logout_accepted",
            "operator_session_logout_recorded",
            "operator_session_logout_persisted",
            "session_revocation_recorded",
            "session_revocation_persisted",
            "session_logout_recorded",
            "session_logout_persisted",
            "identity_invalidation_recorded",
            "revocation_token_recorded",
            "logout_nonce_recorded",
            "device_session_logout_recorded",
            "operator_approval_from_revocation_logout_derived",
            "release_publication_authority_from_revocation_logout_derived",
            "activation_authority_from_revocation_logout_derived",
            "install_from_revocation_logout_executed",
            "service_restart_from_revocation_logout_performed",
            "active_binary_from_revocation_logout_mutated",
            "memory_store_write_performed",
            "memory_store_mutated",
            "live_kg_write_performed",
            "provider_invoked",
            "model_invoked",
            "credential_read",
            "secret_file_read",
            "external_send_performed",
        ] {
            assert_eq!(surface[key], false, "{key}");
        }
    }

    let denied = value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout"]
            .as_array()
            .expect("artifact download install affordance result receipt operator identity session revocation logout denials");
    assert_eq!(denied.len(), 18);
    assert_eq!(
        value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_denial_gate"
    );
    for key in [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_revocation_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_session_logout_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_lifecycle_recorded",
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
        "release_artifact_written",
        "public_artifact_written",
        "external_send_performed",
    ] {
        assert_eq!(value[key], false, "{key}");
    }
    let side_effects = value["side_effects"].as_object().expect(
            "release publication result receipt terminal distribution delivery receipt artifact download install affordance result receipt operator identity session revocation logout side effects",
        );
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}
