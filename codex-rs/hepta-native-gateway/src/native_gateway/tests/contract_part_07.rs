#[test]
fn hepta_memory_intelligence_kg_full_live_activation_artifact_signing_receipt_export_query_observability_endpoint_blocks_views_and_authority()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_RECEIPT_EXPORT_QUERY_OBSERVABILITY_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body)
        .expect("artifact signing receipt export query observability route json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_RECEIPT_EXPORT_QUERY_OBSERVABILITY_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-export-query-observability-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_export_query_observability_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_export_query_observability_denial_ready"],
        true
    );
    assert_eq!(
        value["source_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_ready"],
        true
    );
    assert_eq!(
        value["source_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_surface_count"],
        18
    );
    assert_eq!(
        value["source_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_denied_count"],
        18
    );
    assert_eq!(
        value["artifact_distribution_signing_notarization_receipt_export_query_observability_surface_count"],
        18
    );
    assert_eq!(
        value["artifact_distribution_signing_notarization_receipt_export_query_observability_attempt_count"],
        18
    );
    assert_eq!(
        value["artifact_distribution_signing_notarization_receipt_export_query_observability_denied_count"],
        18
    );

    for key in [
        "source_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_accepted_count",
        "source_artifact_distribution_signing_notarization_receipt_retention_policy_recorded_count",
        "source_artifact_distribution_signing_notarization_receipt_garbage_collection_scan_performed_count",
        "source_release_publication_authority_from_signing_receipt_retention_derived_count",
        "source_activation_authority_from_signing_receipt_retention_derived_count",
        "artifact_distribution_signing_notarization_receipt_export_query_observability_allowed_count",
        "artifact_distribution_signing_notarization_receipt_export_query_observability_accepted_count",
        "artifact_distribution_signing_notarization_receipt_export_query_observability_recorded_count",
        "artifact_distribution_signing_notarization_receipt_export_query_observability_persisted_count",
        "artifact_distribution_signing_notarization_receipt_query_registered_count",
        "artifact_distribution_signing_notarization_receipt_query_executed_count",
        "artifact_distribution_signing_notarization_receipt_query_result_recorded_count",
        "artifact_distribution_signing_notarization_receipt_search_index_recorded_count",
        "artifact_distribution_signing_notarization_receipt_export_accepted_count",
        "artifact_distribution_signing_notarization_receipt_export_snapshot_recorded_count",
        "artifact_distribution_signing_notarization_receipt_export_file_written_count",
        "artifact_distribution_signing_notarization_receipt_export_stream_opened_count",
        "artifact_distribution_signing_notarization_receipt_observability_metric_recorded_count",
        "artifact_distribution_signing_notarization_receipt_observability_trace_recorded_count",
        "artifact_distribution_signing_notarization_receipt_dashboard_panel_recorded_count",
        "artifact_distribution_signing_notarization_receipt_alert_registered_count",
        "artifact_distribution_signing_notarization_receipt_operator_summary_recorded_count",
        "artifact_distribution_signing_notarization_receipt_readback_surface_recorded_count",
        "artifact_distribution_signing_notarization_receipt_audit_view_recorded_count",
        "artifact_distribution_signing_notarization_receipt_delivery_observability_recorded_count",
        "release_publication_authority_from_signing_receipt_export_query_observability_derived_count",
        "activation_authority_from_signing_receipt_export_query_observability_derived_count",
        "install_from_signing_receipt_export_query_observability_executed_count",
        "service_restart_from_signing_receipt_export_query_observability_performed_count",
        "active_binary_from_signing_receipt_export_query_observability_mutated_count",
        "memory_store_write_performed_count",
        "live_kg_write_performed_count",
        "provider_invoked_count",
        "model_invoked_count",
        "credential_read_count",
        "secret_file_read_count",
        "external_send_performed_count",
    ] {
        assert_eq!(value[key], 0, "{key}");
    }

    let surfaces = value[
            "artifact_distribution_signing_notarization_receipt_export_query_observability_surfaces"
        ]
        .as_array()
        .expect("artifact signing receipt export query observability surfaces");
    assert_eq!(surfaces.len(), 18);
    assert_eq!(
        surfaces[0]["surface"],
        "source_signing_receipt_retention_expiry_garbage_collection_report_required"
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["query_requested"] == true)
            .count(),
        3
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["export_requested"] == true)
            .count(),
        4
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["observability_requested"] == true)
            .count(),
        5
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["external_telegram_observability_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["install_restart_active_binary_view_requested"] == true)
            .count(),
        1
    );
    for surface in surfaces {
        assert_eq!(
            surface["artifact_distribution_signing_notarization_receipt_export_query_observability_attempted"],
            true
        );
        assert_eq!(
            surface["artifact_distribution_signing_notarization_receipt_export_query_observability_noop_confirmed"],
            true
        );
        for key in [
            "artifact_distribution_signing_notarization_receipt_export_query_observability_allowed",
            "artifact_distribution_signing_notarization_receipt_export_query_observability_accepted",
            "artifact_distribution_signing_notarization_receipt_export_query_observability_recorded",
            "artifact_distribution_signing_notarization_receipt_export_query_observability_persisted",
            "query_registered",
            "query_executed",
            "query_result_recorded",
            "search_index_recorded",
            "export_accepted",
            "export_snapshot_recorded",
            "export_file_written",
            "export_stream_opened",
            "observability_metric_recorded",
            "observability_trace_recorded",
            "dashboard_panel_recorded",
            "alert_registered",
            "operator_summary_recorded",
            "readback_surface_recorded",
            "audit_view_recorded",
            "release_publication_authority_from_export_query_observability_derived",
            "activation_authority_from_export_query_observability_derived",
            "install_from_export_query_observability_executed",
            "service_restart_from_export_query_observability_performed",
            "active_binary_from_export_query_observability_mutated",
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

    let denied = value[
            "denied_by_artifact_distribution_signing_notarization_receipt_export_query_observability"
        ]
        .as_array()
        .expect("artifact signing receipt export query observability denials");
    assert_eq!(denied.len(), 19);
    assert_eq!(
        value["denied_by_artifact_distribution_signing_notarization_receipt_export_query_observability_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_non_persistence_denial_gate"
    );
    for key in [
        "artifact_distribution_signing_notarization_receipt_export_query_observability_accepted",
        "artifact_distribution_signing_notarization_receipt_export_query_observability_recorded",
        "artifact_distribution_signing_notarization_receipt_export_query_observability_persisted",
        "artifact_distribution_signing_notarization_receipt_query_registered",
        "artifact_distribution_signing_notarization_receipt_query_executed",
        "artifact_distribution_signing_notarization_receipt_query_result_recorded",
        "artifact_distribution_signing_notarization_receipt_search_index_recorded",
        "artifact_distribution_signing_notarization_receipt_export_accepted",
        "artifact_distribution_signing_notarization_receipt_export_file_written",
        "artifact_distribution_signing_notarization_receipt_export_stream_opened",
        "artifact_distribution_signing_notarization_receipt_observability_recorded",
        "artifact_distribution_signing_notarization_receipt_dashboard_panel_recorded",
        "artifact_distribution_signing_notarization_receipt_alert_registered",
        "artifact_distribution_signing_notarization_receipt_operator_summary_recorded",
        "artifact_distribution_signing_notarization_receipt_readback_surface_recorded",
        "artifact_distribution_signing_notarization_receipt_audit_view_recorded",
        "public_release_claimed",
        "public_ga_claimed",
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
        "external_send_performed",
        "release_artifact_written",
        "public_artifact_written",
        "filesystem_written",
    ] {
        assert_eq!(value[key], false, "{key}");
    }
    let side_effects = value["side_effects"]
        .as_object()
        .expect("artifact signing receipt export query observability side effects");
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_artifact_signing_receipt_operator_summary_briefing_endpoint_blocks_delivery_and_authority()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_RECEIPT_OPERATOR_FACING_SUMMARY_BRIEFING_NON_PERSISTENCE_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body)
        .expect("artifact signing receipt operator summary briefing route json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_RECEIPT_OPERATOR_FACING_SUMMARY_BRIEFING_NON_PERSISTENCE_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-operator-facing-summary-briefing-non-persistence-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_operator_facing_summary_briefing_non_persistence_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_operator_facing_summary_briefing_non_persistence_denial_ready"],
        true
    );
    assert_eq!(
        value["source_artifact_distribution_signing_notarization_receipt_export_query_observability_ready"],
        true
    );
    assert_eq!(
        value["source_artifact_distribution_signing_notarization_receipt_export_query_observability_surface_count"],
        18
    );
    assert_eq!(
        value["source_artifact_distribution_signing_notarization_receipt_export_query_observability_denied_count"],
        18
    );
    assert_eq!(
        value["artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_surface_count"],
        18
    );
    assert_eq!(
        value["artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_attempt_count"],
        18
    );
    assert_eq!(
        value["artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_denied_count"],
        18
    );

    for key in [
        "source_artifact_distribution_signing_notarization_receipt_export_query_observability_accepted_count",
        "source_artifact_distribution_signing_notarization_receipt_operator_summary_recorded_count",
        "source_release_publication_authority_from_signing_receipt_export_query_observability_derived_count",
        "source_activation_authority_from_signing_receipt_export_query_observability_derived_count",
        "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_allowed_count",
        "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_accepted_count",
        "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_recorded_count",
        "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_persisted_count",
        "artifact_distribution_signing_notarization_receipt_operator_summary_recorded_count",
        "artifact_distribution_signing_notarization_receipt_operator_briefing_recorded_count",
        "artifact_distribution_signing_notarization_receipt_readback_recorded_count",
        "artifact_distribution_signing_notarization_receipt_status_banner_recorded_count",
        "artifact_distribution_signing_notarization_receipt_briefing_delivery_recorded_count",
        "artifact_distribution_signing_notarization_receipt_summary_briefing_acceptance_recorded_count",
        "operator_approval_from_signing_receipt_summary_briefing_derived_count",
        "release_publication_authority_from_signing_receipt_summary_briefing_derived_count",
        "activation_authority_from_signing_receipt_summary_briefing_derived_count",
        "install_from_signing_receipt_summary_briefing_executed_count",
        "service_restart_from_signing_receipt_summary_briefing_performed_count",
        "active_binary_from_signing_receipt_summary_briefing_mutated_count",
        "memory_store_write_performed_count",
        "live_kg_write_performed_count",
        "provider_invoked_count",
        "model_invoked_count",
        "credential_read_count",
        "secret_file_read_count",
        "external_send_performed_count",
    ] {
        assert_eq!(value[key], 0, "{key}");
    }

    let surfaces = value[
            "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_surfaces"
        ]
        .as_array()
        .expect("artifact signing receipt operator summary briefing surfaces");
    assert_eq!(surfaces.len(), 18);
    assert_eq!(
        surfaces[0]["surface"],
        "source_signing_receipt_export_query_observability_report_required"
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["summary_requested"] == true)
            .count(),
        4
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["briefing_requested"] == true)
            .count(),
        7
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["external_briefing_requested"] == true
                && surface["telegram_briefing_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["authority_briefing_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["live_status_briefing_requested"] == true)
            .count(),
        2
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
            surface["artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_attempted"],
            true
        );
        assert_eq!(
            surface["artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_noop_confirmed"],
            true
        );
        for key in [
            "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_allowed",
            "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_accepted",
            "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_recorded",
            "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_persisted",
            "operator_summary_recorded",
            "operator_summary_persisted",
            "operator_briefing_recorded",
            "operator_briefing_persisted",
            "signing_receipt_readback_recorded",
            "status_banner_recorded",
            "briefing_delivery_recorded",
            "external_briefing_delivered",
            "telegram_briefing_delivered",
            "authority_briefing_recorded",
            "live_status_briefing_recorded",
            "operator_acceptance_from_summary_recorded",
            "operator_acceptance_from_briefing_recorded",
            "operator_approval_from_summary_derived",
            "operator_approval_from_briefing_derived",
            "release_publication_authority_from_summary_briefing_derived",
            "activation_authority_from_summary_briefing_derived",
            "install_from_summary_briefing_executed",
            "service_restart_from_summary_briefing_performed",
            "active_binary_from_summary_briefing_mutated",
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

    let denied = value[
            "denied_by_artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing"
        ]
        .as_array()
        .expect("artifact signing receipt operator summary briefing denials");
    assert_eq!(denied.len(), 19);
    assert_eq!(
        value["denied_by_artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_non_acceptance_denial_gate"
    );
    for key in [
        "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_accepted",
        "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_recorded",
        "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_persisted",
        "artifact_distribution_signing_notarization_receipt_operator_summary_recorded",
        "artifact_distribution_signing_notarization_receipt_operator_briefing_recorded",
        "artifact_distribution_signing_notarization_receipt_readback_recorded",
        "artifact_distribution_signing_notarization_receipt_status_banner_recorded",
        "artifact_distribution_signing_notarization_receipt_briefing_delivery_recorded",
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
        "external_send_performed",
        "release_artifact_written",
        "public_artifact_written",
        "filesystem_written",
    ] {
        assert_eq!(value[key], false, "{key}");
    }
    let side_effects = value["side_effects"]
        .as_object()
        .expect("artifact signing receipt operator summary briefing side effects");
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_artifact_signing_receipt_final_operator_acknowledgement_endpoint_blocks_acceptance_and_authority()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_RECEIPT_FINAL_OPERATOR_ACKNOWLEDGEMENT_NON_ACCEPTANCE_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body)
        .expect("artifact signing receipt final operator acknowledgement route json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_RECEIPT_FINAL_OPERATOR_ACKNOWLEDGEMENT_NON_ACCEPTANCE_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-final-operator-acknowledgement-non-acceptance-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_final_operator_acknowledgement_non_acceptance_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_final_operator_acknowledgement_non_acceptance_denial_ready"],
        true
    );
    assert_eq!(
        value["source_artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_ready"],
        true
    );
    assert_eq!(
        value["source_artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_surface_count"],
        18
    );
    assert_eq!(
        value["source_artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_denied_count"],
        18
    );
    assert_eq!(
        value["artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_surface_count"],
        18
    );
    assert_eq!(
        value["artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_attempt_count"],
        18
    );
    assert_eq!(
        value["artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_denied_count"],
        18
    );

    for key in [
        "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_accepted_count",
        "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_recorded_count",
        "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_persisted_count",
        "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_delivered_count",
        "artifact_distribution_signing_notarization_receipt_operator_received_recorded_count",
        "artifact_distribution_signing_notarization_receipt_operator_confirmed_recorded_count",
        "artifact_distribution_signing_notarization_receipt_operator_read_recorded_count",
        "artifact_distribution_signing_notarization_receipt_operator_seen_recorded_count",
        "artifact_distribution_signing_notarization_receipt_final_response_recorded_count",
        "artifact_distribution_signing_notarization_receipt_completion_acknowledgement_recorded_count",
        "artifact_distribution_signing_notarization_receipt_status_acknowledgement_recorded_count",
        "artifact_distribution_signing_notarization_receipt_external_acknowledgement_sent_count",
        "artifact_distribution_signing_notarization_receipt_telegram_acknowledgement_sent_count",
        "operator_approval_from_signing_receipt_acknowledgement_derived_count",
        "release_publication_authority_from_signing_receipt_acknowledgement_derived_count",
        "activation_authority_from_signing_receipt_acknowledgement_derived_count",
        "install_from_signing_receipt_acknowledgement_executed_count",
        "service_restart_from_signing_receipt_acknowledgement_performed_count",
        "active_binary_from_signing_receipt_acknowledgement_mutated_count",
        "memory_store_write_performed_count",
        "live_kg_write_performed_count",
        "provider_invoked_count",
        "model_invoked_count",
        "credential_read_count",
        "secret_file_read_count",
        "external_send_performed_count",
    ] {
        assert_eq!(value[key], 0, "{key}");
    }

    let surfaces = value[
            "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_surfaces"
        ]
        .as_array()
        .expect("artifact signing receipt final operator acknowledgement surfaces");
    assert_eq!(surfaces.len(), 18);
    assert_eq!(
        surfaces[0]["artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_surface"],
        "source_signing_receipt_summary_briefing_report_required"
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
            .filter(|surface| surface["authority_acknowledgement_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["live_acknowledgement_requested"] == true)
            .count(),
        2
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(
                |surface| surface["install_restart_active_binary_acknowledgement_requested"]
                    == true
            )
            .count(),
        1
    );
    for surface in surfaces {
        assert_eq!(
            surface["artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_attempted"],
            true
        );
        assert_eq!(
            surface["artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_allowed"],
            false
        );
        assert_eq!(
            surface["artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_noop_confirmed"],
            true
        );
        for key in [
            "final_operator_acknowledgement_accepted",
            "final_operator_acknowledgement_recorded",
            "final_operator_acknowledgement_persisted",
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

    let denied = value[
            "denied_by_artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement"
        ]
        .as_array()
        .expect("artifact signing receipt final operator acknowledgement denials");
    assert_eq!(denied.len(), 19);
    assert_eq!(
        value["denied_by_artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_decision_status_promotion_denial_gate"
    );
    for key in [
        "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_accepted",
        "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_recorded",
        "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_persisted",
        "artifact_distribution_signing_notarization_receipt_operator_received_recorded",
        "artifact_distribution_signing_notarization_receipt_operator_confirmed_recorded",
        "artifact_distribution_signing_notarization_receipt_operator_read_recorded",
        "artifact_distribution_signing_notarization_receipt_operator_seen_recorded",
        "artifact_distribution_signing_notarization_receipt_final_response_recorded",
        "artifact_distribution_signing_notarization_receipt_completion_acknowledgement_recorded",
        "artifact_distribution_signing_notarization_receipt_status_acknowledgement_recorded",
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
        "external_send_performed",
        "release_artifact_written",
        "public_artifact_written",
        "filesystem_written",
    ] {
        assert_eq!(value[key], false, "{key}");
    }
    let side_effects = value["side_effects"]
        .as_object()
        .expect("artifact signing receipt final operator acknowledgement side effects");
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_artifact_signing_receipt_terminal_public_claim_status_exposure_endpoint_blocks_public_exposure_and_authority()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_RECEIPT_TERMINAL_PUBLIC_CLAIM_STATUS_EXPOSURE_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body)
        .expect("artifact signing receipt terminal public claim/status exposure route json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_RECEIPT_TERMINAL_PUBLIC_CLAIM_STATUS_EXPOSURE_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-terminal-public-claim-status-exposure-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_terminal_public_claim_status_exposure_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_terminal_public_claim_status_exposure_denial_ready"],
        true
    );
    assert_eq!(
        value["source_artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_ready"],
        true
    );
    assert_eq!(
        value["source_artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_surface_count"],
        18
    );
    assert_eq!(
        value["source_artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_denied_count"],
        18
    );
    assert_eq!(
        value["artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_surface_count"],
        18
    );
    assert_eq!(
        value["artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_attempt_count"],
        18
    );
    assert_eq!(
        value["artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_denied_count"],
        18
    );

    for key in [
        "source_artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_accepted_count",
        "source_artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_recorded_count",
        "source_release_publication_authority_from_signing_receipt_acknowledgement_derived_count",
        "source_activation_authority_from_signing_receipt_acknowledgement_derived_count",
        "artifact_distribution_signing_notarization_receipt_public_claim_recorded_count",
        "artifact_distribution_signing_notarization_receipt_public_claim_persisted_count",
        "artifact_distribution_signing_notarization_receipt_status_exposure_recorded_count",
        "artifact_distribution_signing_notarization_receipt_status_exposure_persisted_count",
        "artifact_distribution_signing_notarization_receipt_channel_status_exposure_delivered_count",
        "artifact_distribution_signing_notarization_receipt_external_status_exposure_sent_count",
        "artifact_distribution_signing_notarization_receipt_telegram_status_exposure_sent_count",
        "release_artifact_written_count",
        "public_artifact_written_count",
        "operator_approval_from_signing_receipt_public_claim_derived_count",
        "release_publication_authority_from_signing_receipt_public_claim_derived_count",
        "activation_authority_from_signing_receipt_status_exposure_derived_count",
        "download_link_from_signing_receipt_status_exposure_rendered_count",
        "install_command_from_signing_receipt_status_exposure_emitted_count",
        "install_from_signing_receipt_status_exposure_executed_count",
        "service_restart_from_signing_receipt_status_exposure_performed_count",
        "active_binary_from_signing_receipt_status_exposure_mutated_count",
        "memory_store_write_performed_count",
        "live_kg_write_performed_count",
        "provider_invoked_count",
        "model_invoked_count",
        "credential_read_count",
        "secret_file_read_count",
        "external_send_performed_count",
    ] {
        assert_eq!(value[key], 0, "{key}");
    }

    let surfaces = value[
            "artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_surfaces"
        ]
        .as_array()
        .expect("artifact signing receipt terminal public claim/status exposure surfaces");
    assert_eq!(surfaces.len(), 18);
    assert_eq!(
        surfaces[0]["artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_surface"],
        "source_signing_receipt_final_operator_acknowledgement_report_required"
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["public_claim_requested"] == true)
            .count(),
        6
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["status_exposure_requested"] == true)
            .count(),
        12
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["telegram_status_exposure_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["release_publication_status_exposure_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(
                |surface| surface["install_restart_active_binary_status_exposure_requested"]
                    == true
            )
            .count(),
        1
    );
    for surface in surfaces {
        assert_eq!(
            surface["artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_attempted"],
            true
        );
        assert_eq!(
            surface["artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_noop_confirmed"],
            true
        );
        for key in [
            "public_claim_allowed",
            "status_exposure_allowed",
            "public_release_claim_allowed",
            "public_status_exposure_allowed",
            "public_claim_recorded",
            "public_claim_persisted",
            "status_exposure_recorded",
            "status_exposure_persisted",
            "channel_status_exposure_delivered",
            "external_status_exposure_sent",
            "telegram_status_exposure_sent",
            "release_artifact_written",
            "public_artifact_written",
            "operator_approval_from_public_claim_derived",
            "release_publication_authority_from_public_claim_derived",
            "activation_authority_from_status_exposure_derived",
            "download_link_from_status_exposure_rendered",
            "install_command_from_status_exposure_emitted",
            "install_from_status_exposure_executed",
            "service_restart_from_status_exposure_performed",
            "active_binary_from_status_exposure_mutated",
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

    let denied = value[
            "denied_by_artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure"
        ]
        .as_array()
        .expect("artifact signing receipt terminal public claim/status exposure denials");
    assert_eq!(denied.len(), 12);
    assert_eq!(
        value["denied_by_artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_denial_gate"
    );
    for key in [
        "artifact_distribution_signing_notarization_receipt_public_claim_recorded",
        "artifact_distribution_signing_notarization_receipt_status_exposure_recorded",
        "artifact_distribution_signing_notarization_receipt_public_status_exposed",
        "public_release_claimed",
        "public_ga_claimed",
        "operator_approval_recorded",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "download_link_rendered",
        "install_command_emitted",
        "activation_allowed",
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
        "filesystem_written",
    ] {
        assert_eq!(value[key], false, "{key}");
    }
    let side_effects = value["side_effects"]
        .as_object()
        .expect("artifact signing receipt terminal public claim/status exposure side effects");
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_artifact_signing_receipt_terminal_public_claim_delivery_readback_endpoint_blocks_delivery_receipts_and_authority()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_RECEIPT_TERMINAL_PUBLIC_CLAIM_DELIVERY_READBACK_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body)
        .expect("artifact signing receipt terminal public claim delivery/readback route json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_RECEIPT_TERMINAL_PUBLIC_CLAIM_DELIVERY_READBACK_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-terminal-public-claim-delivery-readback-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_terminal_public_claim_delivery_readback_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_terminal_public_claim_delivery_readback_denial_ready"],
        true
    );
    assert_eq!(
        value["source_artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_ready"],
        true
    );
    assert_eq!(
        value["source_artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_surface_count"],
        18
    );
    assert_eq!(
        value["source_artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_denied_count"],
        18
    );
    assert_eq!(
        value["artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_surface_count"],
        18
    );
    assert_eq!(
        value["artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_attempt_count"],
        18
    );
    assert_eq!(
        value["artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_denied_count"],
        18
    );

    for key in [
        "source_artifact_distribution_signing_notarization_receipt_public_claim_recorded_count",
        "source_artifact_distribution_signing_notarization_receipt_status_exposure_recorded_count",
        "source_artifact_distribution_signing_notarization_receipt_channel_status_exposure_delivered_count",
        "source_artifact_distribution_signing_notarization_receipt_external_status_exposure_sent_count",
        "source_artifact_distribution_signing_notarization_receipt_telegram_status_exposure_sent_count",
        "artifact_distribution_signing_notarization_receipt_public_claim_delivery_recorded_count",
        "artifact_distribution_signing_notarization_receipt_public_claim_delivery_persisted_count",
        "artifact_distribution_signing_notarization_receipt_status_readback_recorded_count",
        "artifact_distribution_signing_notarization_receipt_status_readback_persisted_count",
        "artifact_distribution_signing_notarization_receipt_channel_delivery_recorded_count",
        "artifact_distribution_signing_notarization_receipt_channel_delivery_persisted_count",
        "artifact_distribution_signing_notarization_receipt_channel_status_readback_delivered_count",
        "artifact_distribution_signing_notarization_receipt_external_delivery_readback_sent_count",
        "artifact_distribution_signing_notarization_receipt_telegram_delivery_readback_sent_count",
        "delivery_receipt_recorded_count",
        "delivery_receipt_persisted_count",
        "readback_receipt_recorded_count",
        "readback_receipt_persisted_count",
        "release_artifact_written_count",
        "public_artifact_written_count",
        "operator_approval_from_delivery_readback_derived_count",
        "release_publication_authority_from_delivery_readback_derived_count",
        "activation_authority_from_delivery_readback_derived_count",
        "download_link_from_delivery_readback_rendered_count",
        "install_command_from_delivery_readback_emitted_count",
        "install_from_delivery_readback_executed_count",
        "service_restart_from_delivery_readback_performed_count",
        "active_binary_from_delivery_readback_mutated_count",
        "memory_store_write_performed_count",
        "live_kg_write_performed_count",
        "provider_invoked_count",
        "model_invoked_count",
        "credential_read_count",
        "secret_file_read_count",
        "external_send_performed_count",
    ] {
        assert_eq!(value[key], 0, "{key}");
    }

    let surfaces = value[
            "artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_surfaces"
        ]
        .as_array()
        .expect("artifact signing receipt terminal public claim delivery/readback surfaces");
    assert_eq!(surfaces.len(), 18);
    assert_eq!(
        surfaces[0]["artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_surface"],
        "source_public_claim_status_exposure_report_required"
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["public_claim_delivery_requested"] == true)
            .count(),
        6
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["status_readback_requested"] == true)
            .count(),
        12
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["channel_delivery_requested"] == true)
            .count(),
        6
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
            .filter(|surface| surface["release_publication_delivery_readback_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["install_restart_active_binary_readback_requested"] == true)
            .count(),
        1
    );
    for surface in surfaces {
        assert_eq!(
            surface["artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_attempted"],
            true
        );
        assert_eq!(
            surface["artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_noop_confirmed"],
            true
        );
        for key in [
            "public_claim_delivery_allowed",
            "status_readback_allowed",
            "channel_delivery_allowed",
            "telegram_delivery_allowed",
            "public_claim_delivery_recorded",
            "public_claim_delivery_persisted",
            "status_readback_recorded",
            "status_readback_persisted",
            "channel_delivery_recorded",
            "channel_delivery_persisted",
            "channel_status_readback_delivered",
            "external_delivery_readback_sent",
            "telegram_delivery_readback_sent",
            "delivery_receipt_recorded",
            "delivery_receipt_persisted",
            "readback_receipt_recorded",
            "readback_receipt_persisted",
            "release_artifact_written",
            "public_artifact_written",
            "operator_approval_from_delivery_readback_derived",
            "release_publication_authority_from_delivery_readback_derived",
            "activation_authority_from_delivery_readback_derived",
            "download_link_from_delivery_readback_rendered",
            "install_command_from_delivery_readback_emitted",
            "install_from_delivery_readback_executed",
            "service_restart_from_delivery_readback_performed",
            "active_binary_from_delivery_readback_mutated",
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

    let denied = value[
            "denied_by_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback"
        ]
        .as_array()
        .expect("artifact signing receipt terminal public claim delivery/readback denials");
    assert_eq!(denied.len(), 14);
    assert_eq!(
        value["denied_by_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "prepare_artifact_distribution_signing_notarization_receipt_release_public_artifact_publication_denial_gate"
    );
    for key in [
        "artifact_distribution_signing_notarization_receipt_public_claim_delivery_recorded",
        "artifact_distribution_signing_notarization_receipt_status_readback_recorded",
        "artifact_distribution_signing_notarization_receipt_channel_delivery_recorded",
        "delivery_receipt_recorded",
        "delivery_receipt_persisted",
        "readback_receipt_recorded",
        "readback_receipt_persisted",
        "public_release_claimed",
        "public_ga_claimed",
        "operator_approval_recorded",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "download_link_rendered",
        "install_command_emitted",
        "activation_allowed",
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
        "filesystem_written",
    ] {
        assert_eq!(value[key], false, "{key}");
    }
    let side_effects = value["side_effects"]
        .as_object()
        .expect("artifact signing receipt terminal public claim delivery/readback side effects");
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_artifact_signing_receipt_release_public_artifact_publication_endpoint_blocks_publication_and_authority()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_RECEIPT_RELEASE_PUBLIC_ARTIFACT_PUBLICATION_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body)
        .expect("artifact signing receipt release/public artifact publication route json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_RECEIPT_RELEASE_PUBLIC_ARTIFACT_PUBLICATION_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-release-public-artifact-publication-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_release_public_artifact_publication_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_release_public_artifact_publication_denial_ready"],
        true
    );
    assert_eq!(
        value["source_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_ready"],
        true
    );
    assert_eq!(
        value["source_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_surface_count"],
        18
    );
    assert_eq!(
        value["source_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_denied_count"],
        18
    );
    assert_eq!(
        value["artifact_distribution_signing_notarization_receipt_release_public_artifact_publication_surface_count"],
        18
    );
    assert_eq!(
        value["artifact_distribution_signing_notarization_receipt_release_public_artifact_publication_attempt_count"],
        18
    );
    assert_eq!(
        value["artifact_distribution_signing_notarization_receipt_release_public_artifact_publication_denied_count"],
        18
    );

    for key in [
        "source_artifact_distribution_signing_notarization_receipt_public_claim_delivery_recorded_count",
        "source_artifact_distribution_signing_notarization_receipt_status_readback_recorded_count",
        "source_artifact_distribution_signing_notarization_receipt_channel_delivery_recorded_count",
        "source_delivery_receipt_recorded_count",
        "source_readback_receipt_recorded_count",
        "source_release_publication_authority_from_delivery_readback_derived_count",
        "source_activation_authority_from_delivery_readback_derived_count",
        "release_artifact_written_count",
        "public_artifact_written_count",
        "publication_queue_enqueued_count",
        "publication_manifest_written_count",
        "public_distribution_performed_count",
        "package_registry_artifact_published_count",
        "external_package_channel_published_count",
        "telegram_package_channel_published_count",
        "public_release_claimed_count",
        "public_ga_claimed_count",
        "release_notes_materialized_count",
        "changelog_materialized_count",
        "operator_approval_from_publication_derived_count",
        "release_publication_authority_from_publication_derived_count",
        "activation_authority_from_publication_derived_count",
        "download_link_from_publication_rendered_count",
        "install_command_from_publication_emitted_count",
        "install_from_publication_executed_count",
        "service_restart_from_publication_performed_count",
        "active_binary_from_publication_mutated_count",
        "memory_store_write_performed_count",
        "live_kg_write_performed_count",
        "provider_invoked_count",
        "model_invoked_count",
        "credential_read_count",
        "secret_file_read_count",
        "external_send_performed_count",
    ] {
        assert_eq!(value[key], 0, "{key}");
    }

    let surfaces = value[
            "artifact_distribution_signing_notarization_receipt_release_public_artifact_publication_surfaces"
        ]
        .as_array()
        .expect("artifact signing receipt release/public artifact publication surfaces");
    assert_eq!(surfaces.len(), 18);
    assert_eq!(
        surfaces[0]["artifact_distribution_signing_notarization_receipt_release_public_artifact_publication_surface"],
        "source_terminal_public_claim_delivery_readback_report_required"
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["release_artifact_publication_requested"] == true)
            .count(),
        6
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["public_artifact_publication_requested"] == true)
            .count(),
        8
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["public_distribution_requested"] == true)
            .count(),
        6
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["telegram_package_channel_publication_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(
                |surface| surface["install_restart_active_binary_publication_requested"] == true
            )
            .count(),
        2
    );
    for surface in surfaces {
        assert_eq!(
            surface["artifact_distribution_signing_notarization_receipt_release_public_artifact_publication_attempted"],
            true
        );
        assert_eq!(
            surface["artifact_distribution_signing_notarization_receipt_release_public_artifact_publication_noop_confirmed"],
            true
        );
        for key in [
            "release_artifact_publication_allowed",
            "public_artifact_publication_allowed",
            "publication_queue_allowed",
            "publication_manifest_allowed",
            "public_distribution_allowed",
            "package_registry_publication_allowed",
            "external_package_channel_publication_allowed",
            "telegram_package_channel_publication_allowed",
            "public_release_claim_allowed",
            "public_ga_claim_allowed",
            "release_notes_materialization_allowed",
            "changelog_materialization_allowed",
            "release_artifact_written",
            "public_artifact_written",
            "publication_queue_enqueued",
            "publication_manifest_written",
            "public_distribution_performed",
            "package_registry_artifact_published",
            "external_package_channel_published",
            "telegram_package_channel_published",
            "release_notes_materialized",
            "changelog_materialized",
            "operator_approval_from_publication_derived",
            "release_publication_authority_from_publication_derived",
            "activation_authority_from_publication_derived",
            "download_link_from_publication_rendered",
            "install_command_from_publication_emitted",
            "install_from_publication_executed",
            "service_restart_from_publication_performed",
            "active_binary_from_publication_mutated",
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

    let denied = value[
            "denied_by_artifact_distribution_signing_notarization_receipt_release_public_artifact_publication"
        ]
        .as_array()
        .expect("artifact signing receipt release/public artifact publication denials");
    assert_eq!(denied.len(), 17);
    assert_eq!(
        value["denied_by_artifact_distribution_signing_notarization_receipt_release_public_artifact_publication_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "prepare_first_model_positive_approval_packet_boundary_gate"
    );
    for key in [
        "release_artifact_written",
        "public_artifact_written",
        "publication_queue_enqueued",
        "publication_manifest_written",
        "public_distribution_performed",
        "package_registry_artifact_published",
        "external_package_channel_published",
        "telegram_package_channel_published",
        "public_release_claimed",
        "public_ga_claimed",
        "release_notes_materialized",
        "changelog_materialized",
        "operator_approval_recorded",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "download_link_rendered",
        "install_command_emitted",
        "activation_allowed",
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
        "channel_send_performed",
        "telegram_send_performed",
        "external_send_performed",
        "filesystem_written",
    ] {
        assert_eq!(value[key], false, "{key}");
    }
    let side_effects = value["side_effects"]
        .as_object()
        .expect("artifact signing receipt release/public artifact publication side effects");
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_minimal_memory_canary_endpoint_runs_ephemeral_write_readback_rollback_idempotency_without_durable_side_effects()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MINIMAL_MEMORY_CANARY_SCOPED_OPERATOR_PACKET_WRITE_READBACK_ROLLBACK_IDEMPOTENCY_RECEIPT_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value =
        serde_json::from_str(&body).expect("minimal memory canary route json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MINIMAL_MEMORY_CANARY_SCOPED_OPERATOR_PACKET_WRITE_READBACK_ROLLBACK_IDEMPOTENCY_RECEIPT_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-minimal-memory-canary-scoped-operator-packet-write-readback-rollback-idempotency-receipt --json"
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
    assert_eq!(value["minimal_memory_canary_route_enabled"], true);
    assert_eq!(value["minimal_memory_canary_ready"], true);
    assert_eq!(
        value["canary_execution_mode"],
        "ephemeral_isolated_fixture_no_durable_store_mutation"
    );
    assert_eq!(
        value["source_operator_intent_consent_evidence_persistence_ready"],
        true
    );
    assert_eq!(value["single_scoped_operator_packet_count"], 1);
    assert_eq!(value["scoped_operator_packet_present"], true);
    assert_eq!(
        value["scoped_operator_packet_accepted_for_ephemeral_canary"],
        true
    );
    assert_eq!(value["operator_packet_persisted"], false);
    assert_eq!(value["operator_approval_recorded"], false);
    assert_eq!(value["memory_namespace"], "hepta_canary_ephemeral");
    assert_eq!(
        value["memory_write_operation"],
        "single_ephemeral_canary_write"
    );
    assert_eq!(value["ephemeral_memory_store_write_performed"], true);
    assert_eq!(value["ephemeral_memory_store_write_count"], 1);
    assert_eq!(value["ephemeral_memory_readback_performed"], true);
    assert_eq!(value["ephemeral_memory_readback_hit_count"], 1);
    assert_eq!(
        value["ephemeral_memory_readback_payload_hash_matched"],
        true
    );
    assert_eq!(value["ephemeral_memory_rollback_performed"], true);
    assert_eq!(value["ephemeral_memory_post_rollback_hit_count"], 0);
    assert_eq!(value["idempotency_required"], true);
    assert_eq!(value["idempotency_replay_performed"], true);
    assert_eq!(value["idempotency_duplicate_write_suppressed"], true);
    assert_eq!(value["idempotency_effective_write_count"], 1);
    assert_eq!(value["idempotency_receipt_generated"], true);
    assert_eq!(value["idempotency_receipt_persisted"], false);
    assert_eq!(
        value["pre_write_store_hash_sha256"],
        value["post_rollback_store_hash_sha256"]
    );
    assert_ne!(
        value["pre_write_store_hash_sha256"],
        value["post_write_store_hash_sha256"]
    );

    let steps = value["canary_steps"]
        .as_array()
        .expect("minimal memory canary steps");
    assert_eq!(steps.len(), 5);
    assert_eq!(steps[0]["step"], "scoped_operator_packet_acceptance");
    assert_eq!(steps[1]["step"], "single_ephemeral_memory_write");
    assert_eq!(steps[2]["step"], "readback_validation");
    assert_eq!(steps[3]["step"], "rollback_to_empty");
    assert_eq!(steps[4]["step"], "idempotency_receipt");
    assert_eq!(steps[1]["ephemeral_memory_store_write_performed"], true);
    assert_eq!(steps[1]["durable_memory_store_write_performed"], false);
    assert_eq!(steps[2]["readback_hit_count"], 1);
    assert_eq!(steps[3]["post_rollback_readback_hit_count"], 0);
    assert_eq!(steps[4]["idempotency_duplicate_write_suppressed"], true);

    for key in [
        "durable_memory_store_write_performed",
        "durable_memory_store_read_performed",
        "durable_memory_store_rollback_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "memory_write_receipt_recorded",
        "memory_write_receipt_persisted",
        "rollback_executed",
        "live_kg_write_performed",
        "kg_adapter_read_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "channel_send_performed",
        "telegram_send_performed",
        "external_send_performed",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
        "release_artifact_written",
        "public_artifact_written",
        "public_release_claimed",
        "public_ga_claimed",
    ] {
        assert_eq!(value[key], false, "{key}");
    }
    let side_effects = value["side_effects"]
        .as_object()
        .expect("minimal memory canary side effects");
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "hepta_intelligence_bounded_context_attachment_preview_readback"
    );
}

#[test]
fn hepta_intelligence_bounded_context_preview_endpoint_renders_readback_without_provider_or_kg_side_effects()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
        "GET",
        HEPTA_INTELLIGENCE_BOUNDED_CONTEXT_ATTACHMENT_PREVIEW_READBACK_ENDPOINT,
        &options,
    );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value =
        serde_json::from_str(&body).expect("intelligence bounded context preview route json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
        value["endpoint"],
        HEPTA_INTELLIGENCE_BOUNDED_CONTEXT_ATTACHMENT_PREVIEW_READBACK_ENDPOINT
    );
    assert_eq!(
        value["source_command"],
        "/hepta-intelligence-bounded-context-attachment-preview-readback --json"
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
    assert_eq!(value["source_minimal_memory_canary_ready"], true);
    assert_eq!(
        value["source_hepta_intelligence_context_attachment_lane_ready"],
        true
    );
    assert_eq!(
        value["canary_execution_mode"],
        "bounded_context_preview_readback_no_provider_prompt_injection"
    );
    assert_eq!(
        value["intelligence_bounded_context_preview_route_enabled"],
        true
    );
    assert_eq!(value["intelligence_bounded_context_preview_ready"], true);
    assert_eq!(
        value["context_scope"],
        "operator_scoped_bounded_context_preview"
    );
    assert_eq!(value["context_attachment_budget_tokens"], 512);
    assert_eq!(value["bounded_context_source_count"], 2);
    assert_eq!(value["bounded_context_preview_item_count"], 3);
    assert_eq!(value["bounded_context_attachment_preview_rendered"], true);
    assert_eq!(value["bounded_context_readback_performed"], true);
    assert_eq!(value["bounded_context_readback_hash_matched"], true);
    assert_eq!(value["readback_receipt_persisted"], false);
    assert_eq!(value["raw_context_materialized"], false);
    assert_eq!(value["raw_prompt_payload_materialized"], false);
    assert_eq!(value["prompt_payload_materialized"], false);
    assert_eq!(value["provider_prompt_injection_performed"], false);
    assert_eq!(value["context_injection_performed"], false);

    let steps = value["preview_steps"]
        .as_array()
        .expect("intelligence bounded context preview steps");
    assert_eq!(steps.len(), 4);
    assert_eq!(steps[0]["step"], "source_memory_canary_receipt_binding");
    assert_eq!(
        steps[1]["step"],
        "bounded_context_attachment_package_preview"
    );
    assert_eq!(steps[2]["step"], "preview_readback_hash_validation");
    assert_eq!(steps[3]["step"], "provider_model_kg_channel_denial_check");
    assert_eq!(
        steps[1]["bounded_context_attachment_preview_rendered"],
        true
    );
    assert_eq!(steps[1]["raw_context_materialized"], false);
    assert_eq!(steps[1]["provider_prompt_injection_performed"], false);
    assert_eq!(steps[2]["bounded_context_readback_hash_matched"], true);
    assert_eq!(steps[2]["readback_receipt_persisted"], false);

    for key in [
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "kg_adapter_read_performed",
        "live_kg_write_performed",
        "channel_send_performed",
        "telegram_send_performed",
        "external_send_performed",
    ] {
        assert_eq!(value[key], false, "{key}");
    }
    let side_effects = value["side_effects"]
        .as_object()
        .expect("intelligence bounded context preview side effects");
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "hepta_kg_read_only_adapter_shadow_rank_canary"
    );
}

#[test]
fn hepta_bounded_intelligence_context_handoff_prompt_preview_boundary_endpoint_blocks_prompt_injection_and_provider_invocation()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
        "GET",
        HEPTA_BOUNDED_INTELLIGENCE_CONTEXT_HANDOFF_PROMPT_PREVIEW_BOUNDARY_ENDPOINT,
        &options,
    );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body)
        .expect("bounded Intelligence handoff prompt preview boundary route json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
        value["endpoint"],
        HEPTA_BOUNDED_INTELLIGENCE_CONTEXT_HANDOFF_PROMPT_PREVIEW_BOUNDARY_ENDPOINT
    );
    assert_eq!(
        value["source_command"],
        "/hepta-bounded-intelligence-context-handoff-prompt-preview-boundary --json"
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
        value["bounded_intelligence_context_handoff_prompt_preview_boundary_route_enabled"],
        true
    );
    assert_eq!(
        value["bounded_intelligence_context_handoff_prompt_preview_boundary_ready"],
        true
    );
    assert_eq!(
        value["bounded_intelligence_context_handoff_prompt_preview_boundary_status"],
        "blocked_report_only"
    );
    assert_eq!(
        value["source_scoped_memory_canary_durable_receipt_boundary_ready"],
        true
    );
    assert_eq!(
        value["source_hepta_intelligence_context_attachment_lane_ready"],
        true
    );
    assert_eq!(value["redacted_receipt_reference_count"], 4);
    assert_eq!(value["context_handoff_candidate_count"], 8);
    assert_eq!(value["accepted_context_handoff_candidate_count"], 0);
    assert_eq!(value["prompt_preview_candidate_count"], 6);
    assert_eq!(value["rendered_prompt_preview_candidate_count"], 0);
    assert_eq!(value["accepted_prompt_preview_candidate_count"], 0);
    assert_eq!(
        value["denied_by_bounded_intelligence_context_handoff_prompt_preview_boundary_count"],
        18
    );
    assert_eq!(value["uses_redacted_receipt_hashes_only"], true);
    assert_eq!(value["bounded_context_handoff_preview_generated"], true);
    assert_eq!(value["prompt_preview_boundary_generated"], true);
    assert_eq!(value["boundary_readback_performed"], true);
    assert_eq!(value["boundary_readback_hash_matched"], true);
    assert_eq!(value["readback_receipt_persisted"], false);
    assert_eq!(value["context_handoff_recorded"], false);
    assert_eq!(value["context_handoff_persisted"], false);
    assert_eq!(value["context_handoff_accepted"], false);
    assert_eq!(value["prompt_preview_rendered_by_report_route"], false);
    assert_eq!(value["raw_context_materialized"], false);
    assert_eq!(value["raw_prompt_payload_materialized"], false);
    assert_eq!(value["prompt_payload_materialized"], false);
    assert_eq!(value["provider_prompt_injection_performed"], false);
    assert_eq!(value["context_injection_performed"], false);

    let references = value["redacted_receipt_references"]
        .as_array()
        .expect("bounded Intelligence redacted receipt references");
    assert_eq!(references.len(), 4);
    assert!(
        references
            .iter()
            .all(|item| item["raw_payload_materialized"].as_bool() == Some(false))
    );
    assert!(
        references
            .iter()
            .all(|item| item["accepted"].as_bool() == Some(false))
    );
    let handoff_candidates = value["context_handoff_candidates"]
        .as_array()
        .expect("bounded Intelligence context handoff candidates");
    assert_eq!(handoff_candidates.len(), 8);
    assert!(
        handoff_candidates
            .iter()
            .all(|item| item["accepted"].as_bool() == Some(false))
    );
    let prompt_candidates = value["prompt_preview_candidates"]
        .as_array()
        .expect("bounded Intelligence prompt preview candidates");
    assert_eq!(prompt_candidates.len(), 6);
    assert!(
        prompt_candidates
            .iter()
            .all(|item| item["rendered"].as_bool() == Some(false))
    );
    let denied = value["denied_by_bounded_intelligence_context_handoff_prompt_preview_boundary"]
        .as_array()
        .expect("bounded Intelligence boundary denials");
    assert_eq!(denied.len(), 18);
    let steps = value["boundary_steps"]
        .as_array()
        .expect("bounded Intelligence boundary steps");
    assert_eq!(steps.len(), 4);
    assert_eq!(
        steps[0]["step"],
        "scoped_memory_durable_receipt_boundary_binding"
    );
    assert_eq!(
        steps[1]["step"],
        "intelligence_context_handoff_lane_binding"
    );
    assert_eq!(steps[2]["step"], "bounded_handoff_prompt_preview_boundary");
    assert_eq!(steps[3]["step"], "boundary_readback_and_side_effect_denial");

    for key in [
        "durable_memory_store_read_performed",
        "durable_memory_store_write_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "hepta_intelligence_context_attached",
        "hepta_intelligence_context_attached_to_provider_prompt",
        "bounded_context_attachment_preview_rendered",
        "prompt_preview_rendered",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "kg_adapter_read_performed",
        "live_kg_write_performed",
        "channel_send_performed",
        "telegram_send_performed",
        "external_send_performed",
        "install_executed",
        "service_restarted",
        "active_binary_mutated",
        "filesystem_written",
    ] {
        assert_eq!(value[key], false, "{key}");
    }
    let side_effects = value["side_effects"]
        .as_object()
        .expect("bounded Intelligence handoff boundary side effects");
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "hepta_kg_read_only_adapter_shadow_rank_canary"
    );
    assert_eq!(
        value["allowed_next_actions"][0]["uses_bounded_intelligence_context_handoff_prompt_preview_boundary"],
        true
    );
    assert_eq!(
        value["allowed_next_actions"][0]["renders_prompt_payload"],
        false
    );
}

#[test]
fn hepta_kg_read_only_adapter_shadow_rank_canary_endpoint_compares_without_live_kg_or_secret_side_effects()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
        "GET",
        HEPTA_KG_READ_ONLY_ADAPTER_SHADOW_RANK_CANARY_ENDPOINT,
        &options,
    );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value =
        serde_json::from_str(&body).expect("KG read-only shadow rank canary route json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
        value["endpoint"],
        HEPTA_KG_READ_ONLY_ADAPTER_SHADOW_RANK_CANARY_ENDPOINT
    );
    assert_eq!(
        value["source_command"],
        "/hepta-kg-read-only-adapter-shadow-rank-canary --json"
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
        value["source_intelligence_bounded_context_preview_ready"],
        true
    );
    assert_eq!(
        value["source_bounded_intelligence_context_handoff_prompt_preview_boundary_ready"],
        true
    );
    assert_eq!(
        value["source_kg_prompt_preview_read_only_adapter_lane_ready"],
        true
    );
    assert_eq!(
        value["canary_execution_mode"],
        "kg_read_only_adapter_shadow_rank_fixture_no_credential_value_read_no_kg_write"
    );
    assert_eq!(
        value["kg_read_only_adapter_shadow_rank_canary_route_enabled"],
        true
    );
    assert_eq!(value["kg_read_only_adapter_shadow_rank_canary_ready"], true);
    assert_eq!(value["kg_adapter_name"], "graphiti");
    assert_eq!(value["kg_adapter_allowlist_enforced"], true);
    assert_eq!(value["credential_reference_required"], true);
    assert_eq!(value["credential_reference_provided"], true);
    assert_eq!(value["credential_reference_kind"], "opaque_reference_only");
    assert_eq!(value["credential_value_read"], false);
    assert_eq!(value["credential_read"], false);
    assert_eq!(value["secret_file_read"], false);
    assert_eq!(
        value["kg_adapter_read_mode"],
        "read_only_shadow_fixture_no_network"
    );
    assert_eq!(value["kg_read_only_adapter_shadow_envelope_rendered"], true);
    assert_eq!(value["kg_adapter_live_read_performed"], false);
    assert_eq!(value["kg_adapter_read_performed"], false);
    assert_eq!(value["external_network_call_performed"], false);
    assert_eq!(value["kg_shadow_rank_result_count"], 3);
    assert_eq!(
        value["kg_shadow_rank_compared_to_transcript_baseline"],
        true
    );
    assert_eq!(
        value["kg_shadow_rank_compared_to_durable_memory_baseline"],
        true
    );
    assert_eq!(value["kg_shadow_rank_vs_transcript_baseline_delta"], 0);
    assert_eq!(value["kg_shadow_rank_vs_durable_memory_baseline_delta"], 0);
    assert_eq!(value["kg_shadow_rank_readback_performed"], true);
    assert_eq!(value["kg_shadow_rank_readback_hash_matched"], true);
    assert_eq!(value["shadow_rank_receipt_persisted"], false);
    assert_eq!(value["live_kg_write_performed"], false);
    assert_eq!(value["provider_invoked"], false);
    assert_eq!(value["model_invoked"], false);
    assert_eq!(value["channel_send_performed"], false);
    assert_eq!(value["telegram_send_performed"], false);
    assert_eq!(value["external_send_performed"], false);

    let steps = value["comparison_steps"]
        .as_array()
        .expect("KG shadow rank comparison steps");
    assert_eq!(steps.len(), 4);
    assert_eq!(
        steps[0]["step"],
        "explicit_adapter_allowlist_and_credential_reference_binding"
    );
    assert_eq!(steps[1]["step"], "read_only_shadow_rank_fixture_projection");
    assert_eq!(steps[2]["step"], "baseline_rank_comparison");
    assert_eq!(steps[3]["step"], "readback_and_side_effect_denial_check");
    assert_eq!(steps[0]["adapter_allowlist_enforced"], true);
    assert_eq!(steps[0]["credential_value_read"], false);
    assert_eq!(steps[1]["kg_adapter_live_read_performed"], false);
    assert_eq!(steps[2]["kg_shadow_rank_vs_transcript_baseline_delta"], 0);
    assert_eq!(steps[3]["kg_shadow_rank_readback_hash_matched"], true);
    assert_eq!(steps[3]["live_kg_write_performed"], false);

    let side_effects = value["side_effects"]
        .as_object()
        .expect("KG read-only shadow rank canary side effects");
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "provider_router_dry_run_envelope_readback_audit"
    );
}

#[test]
fn hepta_provider_router_dry_run_envelope_readback_audit_endpoint_builds_preview_without_provider_model_or_persistence_side_effects()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
        "GET",
        HEPTA_PROVIDER_ROUTER_DRY_RUN_ENVELOPE_READBACK_AUDIT_ENDPOINT,
        &options,
    );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body)
        .expect("provider-router dry-run envelope readback audit route json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
        value["endpoint"],
        HEPTA_PROVIDER_ROUTER_DRY_RUN_ENVELOPE_READBACK_AUDIT_ENDPOINT
    );
    assert_eq!(
        value["source_command"],
        "/hepta-provider-router-dry-run-envelope-readback-audit --json"
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
        value["source_kg_read_only_adapter_shadow_rank_canary_ready"],
        true
    );
    assert_eq!(
        value["source_bounded_provider_router_dry_run_envelope_readback_audit_receipt_lane_ready"],
        true
    );
    assert_eq!(
        value["canary_execution_mode"],
        "provider_router_dry_run_envelope_preview_readback_fixture_no_provider_model_invocation"
    );
    assert_eq!(
        value["provider_router_dry_run_envelope_readback_audit_route_enabled"],
        true
    );
    assert_eq!(
        value["provider_router_dry_run_envelope_readback_audit_ready"],
        true
    );
    assert_eq!(
        value["provider_router_target"],
        "hepta-provider-router:dry-run:bounded-context-shadow-rank"
    );
    assert_eq!(
        value["dry_run_budget_binding"],
        "provider_invocation_budget=0:model_invocation_budget=0"
    );
    assert_eq!(value["provider_invocation_budget"], 0);
    assert_eq!(value["model_invocation_budget"], 0);
    assert_eq!(value["dry_run_envelope_preview_constructed"], true);
    assert_eq!(value["dry_run_envelope_preview_redacted"], true);
    assert_eq!(value["dry_run_envelope_readback_audit_performed"], true);
    assert_eq!(value["dry_run_envelope_readback_hash_matched"], true);
    assert_eq!(value["dry_run_envelope_receipt_rendered"], true);
    assert_eq!(value["dry_run_envelope_receipt_persisted"], false);
    assert_eq!(value["dry_run_envelope_receipt_accepted"], false);
    assert_eq!(value["dry_run_envelope_receipt_ledger_recorded"], false);
    assert_eq!(value["dry_run_envelope_receipt_filesystem_written"], false);
    assert_eq!(value["dry_run_envelope_executed"], false);
    assert_eq!(value["provider_router_prompt_mutated"], false);
    assert_eq!(value["provider_router_context_packet_materialized"], false);
    assert_eq!(value["provider_prompt_injection_performed"], false);
    assert_eq!(value["context_injection_performed"], false);
    assert_eq!(value["provider_invoked"], false);
    assert_eq!(value["model_invoked"], false);
    assert_eq!(value["credential_value_read"], false);
    assert_eq!(value["credential_read"], false);
    assert_eq!(value["secret_file_read"], false);
    assert_eq!(value["kg_adapter_read_performed"], false);
    assert_eq!(value["live_kg_write_performed"], false);
    assert_eq!(value["memory_store_write_performed"], false);
    assert_eq!(value["channel_send_performed"], false);
    assert_eq!(value["telegram_send_performed"], false);
    assert_eq!(value["external_send_performed"], false);

    let steps = value["audit_steps"]
        .as_array()
        .expect("provider-router dry-run envelope audit steps");
    assert_eq!(steps.len(), 4);
    assert_eq!(steps[0]["step"], "kg_shadow_rank_source_binding");
    assert_eq!(steps[1]["step"], "bounded_provider_router_lane_binding");
    assert_eq!(steps[2]["step"], "dry_run_envelope_preview_and_readback");
    assert_eq!(steps[3]["step"], "receipt_and_side_effect_denial_check");
    assert_eq!(steps[2]["dry_run_envelope_preview_constructed"], true);
    assert_eq!(steps[2]["dry_run_envelope_readback_hash_matched"], true);
    assert_eq!(steps[3]["dry_run_envelope_receipt_persisted"], false);
    assert_eq!(steps[3]["dry_run_envelope_executed"], false);
    assert_eq!(steps[3]["provider_invoked"], false);
    assert_eq!(steps[3]["model_invoked"], false);

    let side_effects = value["side_effects"]
        .as_object()
        .expect("provider-router dry-run envelope side effects");
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "first_model_invocation_separate_approval_slice"
    );
    assert_eq!(
        value["allowed_next_actions"][0]["requires_fresh_operator_approval"],
        true
    );
}

#[test]
fn hepta_activation_evidence_no_write_provider_router_dry_run_boundary_endpoint_blocks_evidence_persistence_and_invocation_side_effects()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
        "GET",
        HEPTA_ACTIVATION_EVIDENCE_NO_WRITE_PROVIDER_ROUTER_DRY_RUN_BOUNDARY_ENDPOINT,
        &options,
    );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body)
        .expect("activation evidence no-write provider-router boundary route json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
        value["endpoint"],
        HEPTA_ACTIVATION_EVIDENCE_NO_WRITE_PROVIDER_ROUTER_DRY_RUN_BOUNDARY_ENDPOINT
    );
    assert_eq!(
        value["source_command"],
        "/hepta-activation-evidence-no-write-provider-router-dry-run-boundary --json"
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
        value["activation_evidence_no_write_provider_router_dry_run_boundary_route_enabled"],
        true
    );
    assert_eq!(
        value["activation_evidence_no_write_provider_router_dry_run_boundary_ready"],
        true
    );
    assert_eq!(
        value["activation_evidence_no_write_provider_router_dry_run_boundary_status"],
        "blocked_report_only"
    );
    assert_eq!(
        value["source_provider_router_dry_run_envelope_readback_audit_ready"],
        true
    );
    assert_eq!(value["activation_evidence_candidate_count"], 8);
    assert_eq!(value["accepted_activation_evidence_candidate_count"], 0);
    assert_eq!(value["required_materialization_field_count"], 20);
    assert_eq!(value["recorded_materialization_field_count"], 0);
    assert_eq!(value["required_no_write_sink_surface_count"], 6);
    assert_eq!(value["ready_no_write_sink_surface_count"], 6);
    assert_eq!(value["materialization_fixture_count"], 3);
    assert_eq!(value["blocked_materialization_fixture_count"], 3);
    assert_eq!(value["allowed_materialization_fixture_count"], 0);
    assert_eq!(value["output_path_allowlist_entry_count"], 6);
    assert_eq!(value["output_path_binding_count"], 8);
    assert_eq!(value["recorded_output_path_binding_count"], 0);
    assert_eq!(value["redacted_or_hashed_output_path_binding_count"], 8);
    assert_eq!(value["boundary_readback_performed"], true);
    assert_eq!(value["boundary_readback_hash_matched"], true);
    assert_eq!(value["long_soak_executed_by_this_route"], false);
    assert_eq!(value["long_soak_evidence_recorded"], false);
    assert_eq!(value["activation_evidence_recorded"], false);
    assert_eq!(value["activation_evidence_persisted"], false);
    assert_eq!(value["activation_evidence_materialized"], false);
    assert_eq!(value["activation_evidence_filesystem_written"], false);
    assert_eq!(value["receipt_materialized"], false);
    assert_eq!(value["receipt_persisted"], false);
    assert_eq!(value["receipt_ledger_recorded"], false);
    assert_eq!(value["output_path_selected"], false);
    assert_eq!(value["output_path_bound_to_fresh_evidence"], false);
    assert_eq!(value["fresh_long_soak_evidence_accepted"], false);
    assert_eq!(value["operator_approval_recorded"], false);
    assert_eq!(value["filesystem_persistence_approval_recorded"], false);
    assert_eq!(value["filesystem_persistence_allowed"], false);
    assert_eq!(value["filesystem_persistence_execution_performed"], false);
    assert_eq!(value["activation_allowed"], false);
    assert_eq!(value["active_wiring_allowed"], false);

    let candidates = value["activation_evidence_candidates"]
        .as_array()
        .expect("activation evidence candidates");
    assert_eq!(candidates.len(), 8);
    assert!(
        candidates
            .iter()
            .all(|item| item["accepted"].as_bool() == Some(false))
    );
    let fields = value["required_materialization_fields"]
        .as_array()
        .expect("materialization fields");
    assert_eq!(fields.len(), 20);
    let denied = value["denied_by_activation_evidence_no_write_provider_router_dry_run_boundary"]
        .as_array()
        .expect("activation evidence no-write denials");
    assert_eq!(denied.len(), 20);
    let steps = value["boundary_steps"]
        .as_array()
        .expect("activation evidence no-write boundary steps");
    assert_eq!(steps.len(), 4);
    assert_eq!(steps[0]["step"], "provider_router_dry_run_source_binding");
    assert_eq!(
        steps[1]["step"],
        "activation_evidence_no_write_sink_binding"
    );
    assert_eq!(
        steps[2]["step"],
        "materialization_and_output_path_dry_run_boundary"
    );
    assert_eq!(
        steps[3]["step"],
        "activation_evidence_readback_and_side_effect_denial"
    );
    assert_eq!(steps[2]["output_path_selected"], false);
    assert_eq!(steps[2]["filesystem_persistence_allowed"], false);
    assert_eq!(steps[3]["receipt_persisted"], false);
    assert_eq!(steps[3]["provider_invoked"], false);
    assert_eq!(steps[3]["model_invoked"], false);

    for key in [
        "provider_invoked",
        "model_invoked",
        "credential_value_read",
        "credential_read",
        "secret_file_read",
        "kg_adapter_read_performed",
        "live_kg_write_performed",
        "memory_store_write_performed",
        "channel_send_performed",
        "telegram_send_performed",
        "external_send_performed",
        "install_executed",
        "service_restarted",
        "active_binary_mutated",
        "release_artifact_written",
        "public_artifact_written",
        "public_release_claimed",
        "public_ga_claimed",
    ] {
        assert_eq!(value[key], false, "{key}");
    }
    let side_effects = value["side_effects"]
        .as_object()
        .expect("activation evidence no-write boundary side effects");
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "first_model_invocation_separate_approval_slice"
    );
    assert_eq!(
        value["allowed_next_actions"][0]["uses_activation_evidence_no_write_provider_router_dry_run_boundary"],
        true
    );
    assert_eq!(
        value["allowed_next_actions"][0]["requires_fresh_operator_approval"],
        true
    );
    assert_eq!(
        value["allowed_next_actions"][0]["requires_fresh_long_soak_evidence"],
        true
    );
    assert_eq!(value["allowed_next_actions"][0]["invokes_provider"], false);
    assert_eq!(value["allowed_next_actions"][0]["invokes_model"], false);
}

#[test]
fn hepta_first_model_invocation_explicit_approval_evidence_no_invocation_boundary_endpoint_blocks_approval_acceptance_and_invocation_side_effects()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
        "GET",
        HEPTA_FIRST_MODEL_INVOCATION_EXPLICIT_APPROVAL_EVIDENCE_NO_INVOCATION_BOUNDARY_ENDPOINT,
        &options,
    );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body)
        .expect("first-model explicit approval evidence no-invocation boundary route json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
        value["endpoint"],
        HEPTA_FIRST_MODEL_INVOCATION_EXPLICIT_APPROVAL_EVIDENCE_NO_INVOCATION_BOUNDARY_ENDPOINT
    );
    assert_eq!(
        value["source_command"],
        "/hepta-first-model-invocation-explicit-approval-evidence-no-invocation-boundary --json"
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
        value["first_model_invocation_explicit_approval_evidence_no_invocation_boundary_route_enabled"],
        true
    );
    assert_eq!(
        value["first_model_invocation_explicit_approval_evidence_no_invocation_boundary_ready"],
        true
    );
    assert_eq!(
        value["first_model_invocation_explicit_approval_evidence_no_invocation_boundary_status"],
        "blocked_report_only"
    );
    assert_eq!(
        value["source_activation_evidence_no_write_provider_router_dry_run_boundary_ready"],
        true
    );
    assert_eq!(
        value["source_first_model_invocation_separate_approval_slice_preflight_ready"],
        true
    );
    assert_eq!(value["approval_evidence_candidate_count"], 10);
    assert_eq!(value["accepted_approval_evidence_candidate_count"], 0);
    assert_eq!(value["required_approval_evidence_field_count"], 18);
    assert_eq!(value["recorded_approval_evidence_field_count"], 0);
    assert_eq!(value["fresh_operator_approval_required"], true);
    assert_eq!(value["explicit_invocation_command_required"], true);
    assert_eq!(value["single_use_approval_nonce_required"], true);
    assert_eq!(value["operator_identity_session_binding_required"], true);
    assert_eq!(value["fresh_long_soak_evidence_required"], true);
    assert_eq!(value["fresh_operator_approval_artifact_present"], false);
    assert_eq!(value["fresh_operator_approval_artifact_verified"], false);
    assert_eq!(value["operator_approval_artifact_accepted"], false);
    assert_eq!(value["operator_approval_recorded"], false);
    assert_eq!(value["single_use_approval_nonce_verified"], false);
    assert_eq!(value["single_use_approval_nonce_consumed"], false);
    assert_eq!(value["operator_identity_session_binding_verified"], false);
    assert_eq!(value["operator_identity_session_bound"], false);
    assert_eq!(value["explicit_invocation_command_accepted"], false);
    assert_eq!(value["explicit_invocation_command_consumed"], false);
    assert_eq!(value["fresh_long_soak_evidence_accepted"], false);
    assert_eq!(value["explicit_approval_evidence_recorded"], false);
    assert_eq!(value["explicit_approval_evidence_persisted"], false);
    assert_eq!(value["explicit_approval_evidence_accepted"], false);
    assert_eq!(
        value["explicit_approval_evidence_filesystem_written"],
        false
    );
    assert_eq!(value["approval_authority_derived"], false);
    assert_eq!(value["activation_authority_derived"], false);
    assert_eq!(value["candidate_provider_invocation_requested"], true);
    assert_eq!(value["candidate_model_invocation_requested"], true);
    assert_eq!(value["provider_invocation_authorized"], false);
    assert_eq!(value["model_invocation_authorized"], false);
    assert_eq!(value["provider_invocation_budget"], 0);
    assert_eq!(value["model_invocation_budget"], 0);
    assert_eq!(value["provider_router_live_envelope_executed"], false);
    assert_eq!(value["provider_invoked"], false);
    assert_eq!(value["model_invoked"], false);
    assert_eq!(value["credential_read"], false);
    assert_eq!(value["live_kg_write_performed"], false);
    assert_eq!(value["memory_store_write_performed"], false);
    assert_eq!(value["channel_send_performed"], false);
    assert_eq!(value["external_send_performed"], false);
    assert_eq!(value["install_executed"], false);
    assert_eq!(value["service_restarted"], false);
    assert_eq!(value["active_binary_mutated"], false);
    assert_eq!(value["release_artifact_written"], false);
    assert_eq!(value["public_artifact_written"], false);
    assert_eq!(value["public_release_claimed"], false);
    assert_eq!(value["public_ga_claimed"], false);

    let candidates = value["approval_evidence_candidates"]
        .as_array()
        .expect("approval evidence candidates");
    assert_eq!(candidates.len(), 10);
    assert!(
        candidates
            .iter()
            .all(|item| item["accepted"].as_bool() == Some(false))
    );
    let fields = value["required_approval_evidence_fields"]
        .as_array()
        .expect("required approval evidence fields");
    assert_eq!(fields.len(), 18);
    let denied =
        value["denied_by_first_model_invocation_explicit_approval_evidence_no_invocation_boundary"]
            .as_array()
            .expect("explicit approval evidence no-invocation denials");
    assert_eq!(denied.len(), 24);
    let steps = value["boundary_steps"]
        .as_array()
        .expect("explicit approval evidence no-invocation boundary steps");
    assert_eq!(steps.len(), 5);
    assert_eq!(
        steps[0]["step"],
        "activation_evidence_no_write_source_binding"
    );
    assert_eq!(
        steps[1]["step"],
        "first_model_approval_preflight_source_binding"
    );
    assert_eq!(steps[2]["step"], "explicit_approval_evidence_review");
    assert_eq!(steps[3]["step"], "invocation_authorization_guard");
    assert_eq!(steps[4]["step"], "side_effect_denial_readback");
    assert_eq!(steps[2]["explicit_approval_evidence_accepted"], false);
    assert_eq!(steps[3]["provider_invocation_authorized"], false);
    assert_eq!(steps[3]["model_invocation_authorized"], false);
    assert_eq!(steps[4]["provider_invoked"], false);
    assert_eq!(steps[4]["model_invoked"], false);

    let side_effects = value["side_effects"]
        .as_object()
        .expect("explicit approval evidence no-invocation side effects");
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "first_model_invocation_operator_approval_packet_review_acceptance_denial_preflight"
    );
    assert_eq!(
        value["allowed_next_actions"][0]["uses_activation_evidence_no_write_provider_router_dry_run_boundary"],
        true
    );
    assert_eq!(
        value["allowed_next_actions"][0]["requires_fresh_operator_approval"],
        true
    );
    assert_eq!(
        value["allowed_next_actions"][0]["requires_explicit_command"],
        true
    );
    assert_eq!(value["allowed_next_actions"][0]["invokes_provider"], false);
    assert_eq!(value["allowed_next_actions"][0]["invokes_model"], false);
}

#[test]
fn hepta_full_live_activation_closure_index_endpoint_summarizes_blockers_without_side_effects() {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
        "GET",
        HEPTA_FULL_LIVE_ACTIVATION_CLOSURE_INDEX_ENDPOINT,
        &options,
    );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value =
        serde_json::from_str(&body).expect("full live activation closure index json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
        value["endpoint"],
        HEPTA_FULL_LIVE_ACTIVATION_CLOSURE_INDEX_ENDPOINT
    );
    assert_eq!(
        value["source_command"],
        "/hepta-full-live-activation-closure-index --json"
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
    assert_eq!(value["full_live_activation_closure_index_ready"], true);
    assert_eq!(
        value["full_live_activation_closure_index_status"],
        "blocked_report_only"
    );
    assert_eq!(value["hepta_core_connected"], true);
    assert_eq!(value["hepta_core_full_fusion_complete"], true);
    assert_eq!(value["operator_approved_lanes_ready"], true);
    assert_eq!(value["unrestricted_full_live_activation_enabled"], false);
    assert_eq!(value["unrestricted_full_live_activation_allowed"], false);
    assert_eq!(
        value["unrestricted_full_live_activation_status"],
        "blocked_report_only"
    );
    assert_eq!(value["closure_source_count"], 8);
    assert_eq!(value["ready_closure_source_count"], 8);
    assert_eq!(value["closure_blocker_count"], 13);
    assert_eq!(value["accepted_unrestricted_activation_blocker_count"], 0);
    assert_eq!(value["remaining_unrestricted_activation_blocker_count"], 13);
    assert_eq!(value["canary_ladder_phase_count"], 5);
    assert_eq!(
        value["first_model_positive_approval_packet_boundary_ready"],
        true
    );
    assert_eq!(
        value["scoped_memory_canary_durable_receipt_boundary_ready"],
        true
    );
    assert_eq!(
        value["bounded_intelligence_context_handoff_prompt_preview_boundary_ready"],
        true
    );
    assert_eq!(value["kg_read_only_adapter_shadow_rank_canary_ready"], true);
    assert_eq!(
        value["provider_router_dry_run_envelope_readback_audit_ready"],
        true
    );
    assert_eq!(
        value["activation_evidence_no_write_provider_router_dry_run_boundary_ready"],
        true
    );
    assert_eq!(
        value["first_model_invocation_explicit_approval_evidence_no_invocation_boundary_ready"],
        true
    );
    assert_eq!(value["fresh_operator_approval_artifact_verified"], false);
    assert_eq!(value["single_use_nonce_consumed"], false);
    assert_eq!(value["operator_identity_session_bound"], false);
    assert_eq!(value["explicit_command_accepted"], false);
    assert_eq!(value["fresh_long_soak_evidence_accepted"], false);
    assert_eq!(value["activation_evidence_recorded"], false);
    assert_eq!(value["activation_evidence_persisted"], false);
    assert_eq!(value["durable_memory_store_write_performed"], false);
    assert_eq!(value["bounded_context_handoff_accepted"], false);
    assert_eq!(value["kg_adapter_read_performed"], false);
    assert_eq!(value["live_kg_write_performed"], false);
    assert_eq!(value["provider_invocation_authorized"], false);
    assert_eq!(value["model_invocation_authorized"], false);
    assert_eq!(value["provider_invoked"], false);
    assert_eq!(value["model_invoked"], false);
    assert_eq!(value["credential_read"], false);
    assert_eq!(value["channel_send_performed"], false);
    assert_eq!(value["external_send_performed"], false);
    assert_eq!(value["release_artifact_written"], false);
    assert_eq!(value["public_artifact_written"], false);
    assert_eq!(value["install_executed"], false);
    assert_eq!(value["service_restarted"], false);
    assert_eq!(value["active_binary_mutated"], false);

    let sources = value["closure_sources"]
        .as_array()
        .expect("closure sources");
    assert_eq!(sources.len(), 8);
    assert!(
        sources
            .iter()
            .all(|source| source["ready"].as_bool() == Some(true))
    );
    let blockers = value["closure_blockers"]
        .as_array()
        .expect("closure blockers");
    assert_eq!(blockers.len(), 13);
    assert!(
        blockers
            .iter()
            .all(|blocker| blocker["accepted"].as_bool() == Some(false))
    );
    let ladder = value["canary_ladder"].as_array().expect("canary ladder");
    assert_eq!(ladder.len(), 5);
    assert_eq!(ladder[0]["phase"], "source_closure_index");
    assert_eq!(ladder[4]["phase"], "unrestricted_full_live");
    assert_eq!(ladder[4]["enabled"], false);
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "run_full_live_activation_closure_index_require_live_gate"
    );
    assert_eq!(value["allowed_next_actions"][0]["invokes_provider"], false);
    assert_eq!(
        value["allowed_next_actions"][1]["action"],
        "prepare_scoped_live_canary_operator_packet"
    );
    assert_eq!(
        value["allowed_next_actions"][2]["action"],
        "continue_first_model_invocation_operator_approval_packet_review_acceptance_denial_preflight"
    );
    let side_effects = value["side_effects"]
        .as_object()
        .expect("full live activation closure index side effects");
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_write_approval_packet_boundary_endpoint_exposes_packet_shape_without_execution() {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
        "GET",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_APPROVAL_PACKET_BOUNDARY_ENDPOINT,
        &options,
    );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value =
        serde_json::from_str(&body).expect("memory write approval packet boundary json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
        value["endpoint"],
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_APPROVAL_PACKET_BOUNDARY_ENDPOINT
    );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-live-mutation-operator-write-approval-packet-boundary --json"
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
    assert_eq!(value["memory_write_approval_packet_boundary_ready"], true);
    assert_eq!(
        value["approval_packet_mode"],
        "memory_write_operator_approval_packet_shape_no_recording_no_execution"
    );
    assert_eq!(
        value["source_full_live_activation_closure_index_ready"],
        true
    );
    assert_eq!(value["source_minimal_memory_canary_ready"], true);
    assert_eq!(
        value["source_scoped_memory_canary_durable_receipt_boundary_ready"],
        true
    );
    assert_eq!(value["minimum_required_samples"], 24);
    assert_eq!(value["closure_blocker_count"], 13);
    assert_eq!(value["remaining_unrestricted_activation_blocker_count"], 13);
    assert_eq!(value["memory_write_approval_packet_shape_ready"], true);
    assert_eq!(value["memory_write_approval_packet_recorded"], false);
    assert_eq!(value["memory_write_approval_packet_persisted"], false);
    assert_eq!(value["memory_write_approval_packet_accepted"], false);
    assert_eq!(value["memory_write_request_recorded"], false);
    assert_eq!(value["memory_write_request_accepted"], false);
    assert_eq!(value["operator_approval_recorded"], false);
    assert_eq!(value["operator_identity_hash_recorded"], false);
    assert_eq!(value["operator_approval_signature_hash_recorded"], false);
    assert_eq!(value["single_surface_activation_scope_recorded"], false);
    assert_eq!(value["memory_write_operation_allowed"], false);
    assert_eq!(value["accepted_redaction_proof_recorded"], false);
    assert_eq!(
        value["source_full_live_activation_closure_index_hash_bound"],
        false
    );
    assert_eq!(value["source_minimal_memory_canary_hash_bound"], false);
    assert_eq!(
        value["source_scoped_memory_canary_durable_receipt_hash_bound"],
        false
    );
    assert_eq!(value["fresh_pre_activation_soak_evidence_recorded"], false);
    assert_eq!(value["rollback_plan_recorded"], false);
    assert_eq!(value["post_write_validation_plan_recorded"], false);
    assert_eq!(
        value["no_public_claim_no_external_send_decision_recorded"],
        false
    );
    assert_eq!(value["raw_payload_plaintext_recorded"], false);
    assert_eq!(value["memory_store_mutation_allowed"], false);
    assert_eq!(value["memory_store_mutated"], false);
    assert_eq!(value["durable_memory_store_write_performed"], false);
    assert_eq!(value["memory_write_execution_ready"], false);
    assert_eq!(value["live_mutation_execution_ready"], false);
    assert_eq!(value["provider_prompt_replay_enabled"], false);
    assert_eq!(value["external_send_enabled"], false);
    assert_eq!(
        value["public_claim_or_release_artifact_write_enabled"],
        false
    );
    assert_eq!(
        value["required_memory_write_approval_packet_field_count"],
        21
    );
    assert_eq!(
        value["recorded_memory_write_approval_packet_field_count"],
        0
    );
    assert_eq!(
        value["inherited_required_memory_write_request_field_count"],
        17
    );
    assert_eq!(
        value["inherited_allowed_memory_write_operations"]
            .as_array()
            .expect("allowed memory write operations")
            .len(),
        4
    );
    assert_eq!(
        value["required_memory_write_approval_packet_fields"]
            .as_array()
            .expect("required approval packet fields")
            .len(),
        21
    );
    let fixtures = value["denied_memory_write_approval_packet_fixtures"]
        .as_array()
        .expect("approval packet denial fixtures");
    assert_eq!(fixtures.len(), 8);
    assert!(fixtures.iter().all(|fixture| {
        fixture["packet_accepted"].as_bool() == Some(false)
            && fixture["memory_write_request_accepted"].as_bool() == Some(false)
            && fixture["memory_store_mutated"].as_bool() == Some(false)
            && fixture["activation_allowed"].as_bool() == Some(false)
    }));
    assert_eq!(
        value["denied_by_memory_write_approval_packet_boundary_count"],
        22
    );
    assert_eq!(
        value["required_before_memory_write_approval_packet_acceptance_count"],
        14
    );
    assert_eq!(value["provider_invoked"], false);
    assert_eq!(value["model_invoked"], false);
    assert_eq!(value["credential_read"], false);
    assert_eq!(value["secret_file_read"], false);
    assert_eq!(value["kg_adapter_read_performed"], false);
    assert_eq!(value["live_kg_write_performed"], false);
    assert_eq!(value["channel_send_performed"], false);
    assert_eq!(value["telegram_send_performed"], false);
    assert_eq!(value["external_send_performed"], false);
    assert_eq!(value["release_artifact_written"], false);
    assert_eq!(value["public_artifact_written"], false);
    assert_eq!(value["public_release_claimed"], false);
    assert_eq!(value["install_executed"], false);
    assert_eq!(value["service_restarted"], false);
    assert_eq!(value["active_binary_mutated"], false);
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "run_memory_write_approval_packet_boundary_require_live_gate"
    );
    assert_eq!(value["allowed_next_actions"][0]["writes_memory"], false);
    assert_eq!(
        value["allowed_next_actions"][1]["action"],
        "prepare_memory_write_execution_preflight_denial_boundary"
    );
    let side_effects = value["side_effects"]
        .as_object()
        .expect("memory write approval packet boundary side effects");
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_write_execution_preflight_boundary_endpoint_exposes_preflight_without_execution() {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
        "GET",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_PREFLIGHT_BOUNDARY_ENDPOINT,
        &options,
    );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value =
        serde_json::from_str(&body).expect("memory write execution preflight boundary json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
        value["endpoint"],
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_PREFLIGHT_BOUNDARY_ENDPOINT
    );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-live-mutation-operator-write-execution-preflight-boundary --json"
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
        value["memory_write_execution_preflight_boundary_ready"],
        true
    );
    assert_eq!(
        value["execution_preflight_mode"],
        "memory_write_execution_preflight_no_approval_no_mutation"
    );
    assert_eq!(
        value["source_memory_write_approval_packet_boundary_ready"],
        true
    );
    assert_eq!(value["minimum_required_samples"], 24);
    assert_eq!(value["memory_write_execution_preflight_shape_ready"], true);
    assert_eq!(value["memory_write_execution_preflight_recorded"], false);
    assert_eq!(value["memory_write_execution_preflight_persisted"], false);
    assert_eq!(value["memory_write_execution_preflight_accepted"], false);
    assert_eq!(value["pre_execution_validation_shape_ready"], true);
    assert_eq!(value["pre_execution_validation_recorded"], false);
    assert_eq!(value["pre_execution_validation_persisted"], false);
    assert_eq!(value["pre_execution_validation_accepted"], false);
    assert_eq!(value["memory_write_approval_packet_recorded"], false);
    assert_eq!(value["memory_write_approval_packet_persisted"], false);
    assert_eq!(value["memory_write_approval_packet_accepted"], false);
    assert_eq!(value["memory_write_request_recorded"], false);
    assert_eq!(value["memory_write_request_accepted"], false);
    assert_eq!(value["operator_approval_recorded"], false);
    assert_eq!(value["operator_identity_hash_recorded"], false);
    assert_eq!(value["operator_approval_signature_hash_recorded"], false);
    assert_eq!(value["single_surface_activation_scope_recorded"], false);
    assert_eq!(value["memory_namespace_recorded"], false);
    assert_eq!(value["memory_write_operation_allowed"], false);
    assert_eq!(value["accepted_redaction_proof_recorded"], false);
    assert_eq!(
        value["source_memory_write_approval_packet_hash_bound"],
        false
    );
    assert_eq!(value["source_memory_write_request_hash_bound"], false);
    assert_eq!(
        value["source_full_live_activation_closure_index_hash_bound"],
        false
    );
    assert_eq!(value["source_minimal_memory_canary_hash_bound"], false);
    assert_eq!(
        value["source_scoped_memory_canary_durable_receipt_hash_bound"],
        false
    );
    assert_eq!(value["raw_payload_sha256_bound"], false);
    assert_eq!(value["redacted_payload_summary_sha256_bound"], false);
    assert_eq!(value["fresh_pre_activation_soak_evidence_recorded"], false);
    assert_eq!(value["rollback_plan_recorded"], false);
    assert_eq!(value["post_write_validation_plan_recorded"], false);
    assert_eq!(
        value["no_public_claim_no_external_send_decision_recorded"],
        false
    );
    assert_eq!(value["memory_write_execution_allowed"], false);
    assert_eq!(value["memory_write_execution_ready"], false);
    assert_eq!(value["memory_store_mutation_allowed"], false);
    assert_eq!(value["memory_store_mutated"], false);
    assert_eq!(value["durable_memory_store_write_performed"], false);
    assert_eq!(value["live_mutation_execution_ready"], false);
    assert_eq!(value["rollback_execution_allowed"], false);
    assert_eq!(value["rollback_executed"], false);
    assert_eq!(value["provider_prompt_replay_enabled"], false);
    assert_eq!(value["external_send_enabled"], false);
    assert_eq!(
        value["public_claim_or_release_artifact_write_enabled"],
        false
    );
    assert_eq!(value["required_pre_execution_validation_check_count"], 17);
    assert_eq!(value["recorded_pre_execution_validation_check_count"], 0);
    assert_eq!(
        value["required_pre_execution_validation_checks"]
            .as_array()
            .expect("pre-execution checks")
            .len(),
        17
    );
    let fixtures = value["denied_memory_write_execution_preflight_fixtures"]
        .as_array()
        .expect("execution preflight denial fixtures");
    assert_eq!(fixtures.len(), 9);
    assert!(fixtures.iter().all(|fixture| {
        fixture["execution_allowed"].as_bool() == Some(false)
            && fixture["memory_store_mutated"].as_bool() == Some(false)
            && fixture["activation_allowed"].as_bool() == Some(false)
    }));
    assert_eq!(
        value["denied_by_memory_write_execution_preflight_boundary_count"],
        22
    );
    assert_eq!(value["required_before_memory_write_execution_count"], 17);
    assert_eq!(value["provider_invoked"], false);
    assert_eq!(value["model_invoked"], false);
    assert_eq!(value["credential_read"], false);
    assert_eq!(value["secret_file_read"], false);
    assert_eq!(value["kg_adapter_read_performed"], false);
    assert_eq!(value["live_kg_write_performed"], false);
    assert_eq!(value["channel_send_performed"], false);
    assert_eq!(value["telegram_send_performed"], false);
    assert_eq!(value["external_send_performed"], false);
    assert_eq!(value["release_artifact_written"], false);
    assert_eq!(value["public_artifact_written"], false);
    assert_eq!(value["public_release_claimed"], false);
    assert_eq!(value["install_executed"], false);
    assert_eq!(value["service_restarted"], false);
    assert_eq!(value["active_binary_mutated"], false);
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "run_memory_write_execution_preflight_boundary_require_live_gate"
    );
    assert_eq!(value["allowed_next_actions"][0]["writes_memory"], false);
    assert_eq!(
        value["allowed_next_actions"][1]["action"],
        "prepare_memory_write_execution_denial_matrix_boundary"
    );
    let side_effects = value["side_effects"]
        .as_object()
        .expect("memory write execution preflight boundary side effects");
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_write_execution_denial_matrix_boundary_endpoint_exposes_attempt_denials_without_execution()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
        "GET",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_DENIAL_MATRIX_BOUNDARY_ENDPOINT,
        &options,
    );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value =
        serde_json::from_str(&body).expect("memory write execution denial matrix boundary json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
        value["endpoint"],
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_DENIAL_MATRIX_BOUNDARY_ENDPOINT
    );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-live-mutation-operator-write-execution-denial-matrix-boundary --json"
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
        value["memory_write_execution_denial_matrix_boundary_ready"],
        true
    );
    assert_eq!(
        value["execution_denial_matrix_mode"],
        "memory_write_execution_attempt_denial_matrix_no_store_mutation"
    );
    assert_eq!(
        value["source_memory_write_execution_preflight_boundary_ready"],
        true
    );
    assert_eq!(value["minimum_required_samples"], 24);
    assert_eq!(value["memory_write_execution_denial_matrix_ready"], true);
    assert_eq!(
        value["memory_write_execution_denial_matrix_recorded"],
        false
    );
    assert_eq!(
        value["memory_write_execution_denial_matrix_persisted"],
        false
    );
    assert_eq!(
        value["memory_write_execution_denial_matrix_materialized"],
        false
    );
    assert_eq!(
        value["memory_write_execution_denial_matrix_filesystem_written"],
        false
    );
    assert_eq!(value["pre_execution_validation_shape_ready"], true);
    assert_eq!(value["required_pre_execution_validation_check_count"], 17);
    assert_eq!(value["recorded_pre_execution_validation_check_count"], 0);
    assert_eq!(value["accepted_pre_execution_validation_check_count"], 0);
    assert_eq!(
        value["future_pre_execution_validation_check_slot_count"],
        17
    );
    assert_eq!(value["memory_write_execution_attempt_requested_count"], 7);
    assert_eq!(value["memory_write_execution_attempt_performed_count"], 0);
    assert_eq!(value["memory_write_execution_allowed_count"], 0);
    assert_eq!(value["memory_write_execution_denied_count"], 7);
    assert_eq!(value["blocked_execution_fixture_count"], 7);
    assert_eq!(value["allowed_execution_fixture_count"], 0);
    assert_eq!(value["required_execution_denial_fixture_count"], 7);
    assert_eq!(value["execution_denial_fixture_count"], 7);
    assert_eq!(value["pre_execution_validation_recorded"], false);
    assert_eq!(value["pre_execution_validation_persisted"], false);
    assert_eq!(value["pre_execution_validation_accepted"], false);
    assert_eq!(value["memory_write_approval_packet_accepted"], false);
    assert_eq!(value["memory_write_request_accepted"], false);
    assert_eq!(value["operator_approval_recorded"], false);
    assert_eq!(
        value["source_memory_write_execution_preflight_hash_bound"],
        false
    );
    assert_eq!(value["raw_payload_sha256_bound"], false);
    assert_eq!(value["redacted_payload_summary_sha256_bound"], false);
    assert_eq!(value["raw_payload_plaintext_recorded"], false);
    assert_eq!(value["raw_payload_plaintext_persisted"], false);
    assert_eq!(value["fresh_pre_activation_soak_evidence_recorded"], false);
    assert_eq!(value["rollback_plan_recorded"], false);
    assert_eq!(value["post_write_validation_plan_recorded"], false);
    assert_eq!(
        value["no_public_claim_no_external_send_decision_recorded"],
        false
    );
    assert_eq!(value["memory_write_execution_allowed"], false);
    assert_eq!(value["memory_write_execution_ready"], false);
    assert_eq!(value["memory_write_execution_performed"], false);
    assert_eq!(value["memory_store_mutation_allowed"], false);
    assert_eq!(value["memory_store_mutated"], false);
    assert_eq!(value["durable_memory_store_write_performed"], false);
    assert_eq!(value["live_mutation_execution_ready"], false);
    assert_eq!(value["rollback_execution_allowed"], false);
    assert_eq!(value["rollback_executed"], false);
    assert_eq!(value["provider_prompt_replay_enabled"], false);
    assert_eq!(value["external_send_enabled"], false);
    assert_eq!(value["external_send_performed"], false);
    assert_eq!(
        value["public_claim_or_release_artifact_write_enabled"],
        false
    );
    assert_eq!(value["public_release_published"], false);
    assert_eq!(value["release_artifact_written"], false);
    let fixtures = value["execution_denial_fixtures"]
        .as_array()
        .expect("execution denial fixtures");
    assert_eq!(fixtures.len(), 7);
    assert!(fixtures.iter().all(|fixture| {
        fixture["execution_requested"].as_bool() == Some(true)
            && fixture["execution_allowed"].as_bool() == Some(false)
            && fixture["execution_performed"].as_bool() == Some(false)
            && fixture["memory_store_mutated"].as_bool() == Some(false)
            && fixture["activation_allowed"].as_bool() == Some(false)
    }));
    assert_eq!(
        value["denied_by_memory_write_execution_denial_matrix_count"],
        22
    );
    assert_eq!(value["required_before_memory_write_execution_count"], 17);
    assert_eq!(value["provider_invoked"], false);
    assert_eq!(value["model_invoked"], false);
    assert_eq!(value["credential_read"], false);
    assert_eq!(value["secret_file_read"], false);
    assert_eq!(value["kg_adapter_read_performed"], false);
    assert_eq!(value["live_kg_write_performed"], false);
    assert_eq!(value["channel_send_performed"], false);
    assert_eq!(value["telegram_send_performed"], false);
    assert_eq!(value["external_send_performed"], false);
    assert_eq!(value["public_artifact_written"], false);
    assert_eq!(value["public_release_claimed"], false);
    assert_eq!(value["install_executed"], false);
    assert_eq!(value["service_restarted"], false);
    assert_eq!(value["active_binary_mutated"], false);
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "run_memory_write_execution_denial_matrix_boundary_require_live_gate"
    );
    assert_eq!(value["allowed_next_actions"][0]["writes_memory"], false);
    assert_eq!(
        value["allowed_next_actions"][1]["action"],
        "prepare_memory_write_execution_no_write_sink_contract_boundary"
    );
    let side_effects = value["side_effects"]
        .as_object()
        .expect("memory write execution denial matrix boundary side effects");
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_write_execution_no_write_sink_contract_boundary_endpoint_exposes_validation_without_store_write()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_NO_WRITE_SINK_CONTRACT_BOUNDARY_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body)
        .expect("memory write execution no-write sink contract boundary json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_NO_WRITE_SINK_CONTRACT_BOUNDARY_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-live-mutation-operator-write-execution-no-write-sink-contract-boundary --json"
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
        value["memory_write_execution_no_write_sink_contract_boundary_ready"],
        true
    );
    assert_eq!(
        value["no_write_sink_contract_mode"],
        "memory_write_execution_no_write_sink_contract_no_store_mutation"
    );
    assert_eq!(
        value["source_memory_write_execution_denial_matrix_boundary_ready"],
        true
    );
    assert_eq!(value["minimum_required_samples"], 24);
    assert_eq!(
        value["memory_write_execution_no_write_sink_contract_ready"],
        true
    );
    assert_eq!(value["memory_write_execution_denial_matrix_ready"], true);
    assert_eq!(value["pre_execution_validation_shape_ready"], true);
    assert_eq!(value["required_pre_execution_validation_check_count"], 17);
    assert_eq!(value["accepted_pre_execution_validation_check_count"], 0);
    assert_eq!(value["required_no_write_sink_surface_count"], 8);
    assert_eq!(value["ready_no_write_sink_surface_count"], 8);
    assert_eq!(value["side_effect_free_no_write_sink_surface_count"], 8);
    assert_eq!(value["no_write_sink_fixture_count"], 6);
    assert_eq!(value["no_write_sink_accepted_validation_fixture_count"], 3);
    assert_eq!(value["no_write_sink_rejected_execution_fixture_count"], 3);
    assert_eq!(value["no_write_sink_execution_request_fixture_count"], 6);
    assert_eq!(value["no_write_sink_write_request_fixture_count"], 3);
    assert_eq!(value["no_write_sink_allowed_write_fixture_count"], 0);
    assert_eq!(value["no_write_sink_rejected_write_fixture_count"], 3);
    assert_eq!(
        value["no_write_sink_accepts_redacted_execution_envelope"],
        true
    );
    assert_eq!(
        value["no_write_sink_accepts_source_report_hash_bindings"],
        true
    );
    assert_eq!(
        value["no_write_sink_requires_operator_approval_and_preflight_validation"],
        true
    );
    assert_eq!(
        value["no_write_sink_requires_namespace_operation_retention_allowlist"],
        true
    );
    assert_eq!(
        value["no_write_sink_requires_payload_hash_binding_without_plaintext"],
        true
    );
    assert_eq!(
        value["no_write_sink_requires_fresh_soak_rollback_validation"],
        true
    );
    assert_eq!(
        value["no_write_sink_rejects_external_send_public_claim_artifact"],
        true
    );
    assert_eq!(value["no_write_sink_rejects_store_write_execution"], true);
    assert_eq!(value["no_write_sink_write_path_enabled_by_default"], false);
    assert_eq!(value["no_write_sink_persistence_enabled_by_default"], false);
    assert_eq!(
        value["memory_write_execution_no_write_sink_contract_recorded"],
        false
    );
    assert_eq!(
        value["memory_write_execution_no_write_sink_contract_persisted"],
        false
    );
    assert_eq!(
        value["memory_write_execution_no_write_sink_contract_materialized"],
        false
    );
    assert_eq!(
        value["memory_write_execution_no_write_sink_contract_filesystem_written"],
        false
    );
    assert_eq!(value["memory_write_approval_packet_accepted"], false);
    assert_eq!(value["memory_write_request_accepted"], false);
    assert_eq!(value["raw_payload_plaintext_recorded"], false);
    assert_eq!(value["raw_payload_plaintext_persisted"], false);
    assert_eq!(value["memory_write_execution_allowed"], false);
    assert_eq!(value["memory_write_execution_ready"], false);
    assert_eq!(value["memory_write_execution_performed"], false);
    assert_eq!(value["memory_write_execution_performed_count"], 0);
    assert_eq!(value["memory_write_execution_allowed_count"], 0);
    assert_eq!(value["memory_write_execution_denied_count"], 6);
    assert_eq!(value["memory_store_write_path_enabled"], false);
    assert_eq!(value["memory_store_write_performed_count"], 0);
    assert_eq!(value["memory_store_mutation_allowed"], false);
    assert_eq!(value["memory_store_mutated"], false);
    assert_eq!(value["durable_memory_store_write_performed"], false);
    assert_eq!(value["rollback_execution_allowed"], false);
    assert_eq!(value["rollback_executed"], false);
    assert_eq!(value["external_send_enabled"], false);
    assert_eq!(value["external_send_performed"], false);
    assert_eq!(
        value["public_claim_or_release_artifact_write_enabled"],
        false
    );
    assert_eq!(value["public_release_published"], false);
    assert_eq!(value["release_artifact_written"], false);
    let surfaces = value["no_write_sink_surfaces"]
        .as_array()
        .expect("no-write sink surfaces");
    assert_eq!(surfaces.len(), 8);
    let fixtures = value["no_write_sink_fixtures"]
        .as_array()
        .expect("no-write sink fixtures");
    assert_eq!(fixtures.len(), 6);
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["sink_status"] == "accepted_for_no_write_validation")
            .count(),
        3
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["sink_status"] == "rejected")
            .count(),
        3
    );
    assert!(fixtures.iter().all(|fixture| {
        fixture["execution_requested"].as_bool() == Some(true)
            && fixture["execution_allowed"].as_bool() == Some(false)
            && fixture["execution_performed"].as_bool() == Some(false)
            && fixture["memory_store_write_performed"].as_bool() == Some(false)
            && fixture["memory_store_mutated"].as_bool() == Some(false)
            && fixture["activation_allowed"].as_bool() == Some(false)
    }));
    assert_eq!(value["denied_by_no_write_sink_contract_count"], 10);
    assert_eq!(
        value["required_before_any_memory_write_execution_count"],
        17
    );
    assert_eq!(value["provider_invoked"], false);
    assert_eq!(value["model_invoked"], false);
    assert_eq!(value["credential_read"], false);
    assert_eq!(value["secret_file_read"], false);
    assert_eq!(value["kg_adapter_read_performed"], false);
    assert_eq!(value["live_kg_write_performed"], false);
    assert_eq!(value["channel_send_performed"], false);
    assert_eq!(value["telegram_send_performed"], false);
    assert_eq!(value["external_send_performed"], false);
    assert_eq!(value["public_artifact_written"], false);
    assert_eq!(value["public_release_claimed"], false);
    assert_eq!(value["install_executed"], false);
    assert_eq!(value["service_restarted"], false);
    assert_eq!(value["active_binary_mutated"], false);
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "run_memory_write_execution_no_write_sink_contract_boundary_require_live_gate"
    );
    assert_eq!(value["allowed_next_actions"][0]["writes_memory"], false);
    assert_eq!(
        value["allowed_next_actions"][1]["action"],
        "prepare_memory_write_execution_write_enable_fixture_boundary"
    );
    let side_effects = value["side_effects"]
        .as_object()
        .expect("memory write execution no-write sink contract boundary side effects");
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_write_execution_write_enable_fixture_boundary_endpoint_blocks_write_enablement_without_store_write()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
        "GET",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_WRITE_ENABLE_FIXTURE_BOUNDARY_ENDPOINT,
        &options,
    );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body)
        .expect("memory write execution write-enable fixture boundary json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
        value["endpoint"],
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_WRITE_ENABLE_FIXTURE_BOUNDARY_ENDPOINT
    );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-live-mutation-operator-write-execution-write-enable-fixture-boundary --json"
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
        value["memory_write_execution_write_enable_fixture_boundary_ready"],
        true
    );
    assert_eq!(
        value["write_enable_fixture_mode"],
        "memory_write_execution_write_enable_fixture_non_activation"
    );
    assert_eq!(
        value["source_memory_write_execution_no_write_sink_contract_boundary_ready"],
        true
    );
    assert_eq!(value["minimum_required_samples"], 24);
    assert_eq!(
        value["memory_write_execution_write_enable_fixture_ready"],
        true
    );
    assert_eq!(
        value["memory_write_execution_no_write_sink_contract_ready"],
        true
    );
    assert_eq!(value["memory_write_execution_denial_matrix_ready"], true);
    assert_eq!(value["required_pre_execution_validation_check_count"], 17);
    assert_eq!(value["accepted_pre_execution_validation_check_count"], 0);
    assert_eq!(value["required_write_enable_surface_count"], 10);
    assert_eq!(value["ready_write_enable_surface_count"], 10);
    assert_eq!(value["side_effect_free_write_enable_surface_count"], 10);
    assert_eq!(value["required_write_enable_fixture_count"], 7);
    assert_eq!(value["write_enable_fixture_count"], 7);
    assert_eq!(value["blocked_write_enable_fixture_count"], 7);
    assert_eq!(value["allowed_write_enable_fixture_count"], 0);
    assert_eq!(value["explicit_write_enable_requested_fixture_count"], 7);
    assert_eq!(
        value["write_enable_denied_missing_approval_preflight_count"],
        1
    );
    assert_eq!(value["write_enable_denied_missing_operator_scope_count"], 1);
    assert_eq!(value["write_enable_denied_allowlist_mismatch_count"], 1);
    assert_eq!(value["write_enable_denied_payload_binding_count"], 1);
    assert_eq!(
        value["write_enable_denied_stale_soak_rollback_validation_count"],
        1
    );
    assert_eq!(value["write_enable_denied_public_artifact_count"], 1);
    assert_eq!(
        value["write_enable_denied_store_or_rollback_execution_count"],
        1
    );
    assert_eq!(value["memory_write_execution_denied_count"], 7);
    assert_eq!(value["memory_write_execution_allowed_count"], 0);
    assert_eq!(value["memory_write_execution_performed_count"], 0);
    assert_eq!(value["memory_store_write_requested_fixture_count"], 7);
    assert_eq!(value["memory_store_write_allowed_count"], 0);
    assert_eq!(value["memory_store_write_performed_count"], 0);
    assert_eq!(value["memory_store_mutation_allowed"], false);
    assert_eq!(value["memory_store_mutated"], false);
    assert_eq!(value["explicit_write_enablement_recorded"], false);
    assert_eq!(value["explicit_write_enablement_persisted"], false);
    assert_eq!(value["explicit_write_enablement_accepted"], false);
    assert_eq!(value["write_enable_fixture_recorded"], false);
    assert_eq!(value["write_enable_fixture_persisted"], false);
    assert_eq!(value["write_enable_fixture_materialized"], false);
    assert_eq!(value["write_enable_fixture_filesystem_written"], false);
    assert_eq!(value["memory_write_approval_packet_accepted"], false);
    assert_eq!(value["memory_write_request_accepted"], false);
    assert_eq!(value["operator_approval_recorded"], false);
    assert_eq!(value["accepted_redaction_proof_count"], 0);
    assert_eq!(value["source_report_hash_bindings_recorded"], false);
    assert_eq!(value["raw_payload_sha256_bound"], false);
    assert_eq!(value["redacted_payload_summary_sha256_bound"], false);
    assert_eq!(value["raw_payload_plaintext_recorded"], false);
    assert_eq!(value["raw_payload_plaintext_persisted"], false);
    assert_eq!(value["memory_write_execution_allowed"], false);
    assert_eq!(value["memory_write_execution_ready"], false);
    assert_eq!(value["memory_write_execution_performed"], false);
    assert_eq!(value["memory_store_write_path_enabled"], false);
    assert_eq!(value["no_write_sink_write_path_enabled_by_default"], false);
    assert_eq!(value["live_mutation_execution_ready"], false);
    assert_eq!(value["rollback_execution_allowed"], false);
    assert_eq!(value["rollback_executed"], false);
    assert_eq!(value["external_send_enabled"], false);
    assert_eq!(value["external_send_performed"], false);
    assert_eq!(
        value["public_claim_or_release_artifact_write_enabled"],
        false
    );
    assert_eq!(value["public_release_published"], false);
    assert_eq!(value["release_artifact_written"], false);
    let surfaces = value["write_enable_surfaces"]
        .as_array()
        .expect("write-enable surfaces");
    assert_eq!(surfaces.len(), 10);
    let fixtures = value["write_enable_fixtures"]
        .as_array()
        .expect("write-enable fixtures");
    assert_eq!(fixtures.len(), 7);
    assert!(fixtures.iter().all(|fixture| {
        fixture["explicit_write_enable_requested"].as_bool() == Some(true)
            && fixture["write_enable_status"].as_str() == Some("blocked")
            && fixture["execution_allowed"].as_bool() == Some(false)
            && fixture["execution_performed"].as_bool() == Some(false)
            && fixture["memory_store_write_allowed"].as_bool() == Some(false)
            && fixture["memory_store_write_performed"].as_bool() == Some(false)
            && fixture["memory_store_mutated"].as_bool() == Some(false)
            && fixture["activation_allowed"].as_bool() == Some(false)
    }));
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["raw_payload_plaintext_recorded"] == true)
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["public_claim_requested"] == true
                    && fixture["release_artifact_write_requested"] == true
            })
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["rollback_execution_requested"] == true)
            .count(),
        1
    );
    assert_eq!(value["denied_by_write_enable_fixture_count"], 13);
    assert_eq!(value["provider_invoked"], false);
    assert_eq!(value["model_invoked"], false);
    assert_eq!(value["credential_read"], false);
    assert_eq!(value["secret_file_read"], false);
    assert_eq!(value["kg_adapter_read_performed"], false);
    assert_eq!(value["live_kg_write_performed"], false);
    assert_eq!(value["channel_send_performed"], false);
    assert_eq!(value["telegram_send_performed"], false);
    assert_eq!(value["external_send_performed"], false);
    assert_eq!(value["public_artifact_written"], false);
    assert_eq!(value["public_release_claimed"], false);
    assert_eq!(value["install_executed"], false);
    assert_eq!(value["service_restarted"], false);
    assert_eq!(value["active_binary_mutated"], false);
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "run_memory_write_execution_write_enable_fixture_boundary_require_live_gate"
    );
    assert_eq!(value["allowed_next_actions"][0]["writes_memory"], false);
    assert_eq!(
        value["allowed_next_actions"][1]["action"],
        "prepare_memory_write_execution_post_write_validation_dry_run_boundary"
    );
    let side_effects = value["side_effects"]
        .as_object()
        .expect("memory write execution write-enable fixture boundary side effects");
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_write_execution_post_write_validation_dry_run_boundary_endpoint_blocks_validation_without_store_write()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_POST_WRITE_VALIDATION_DRY_RUN_BOUNDARY_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body)
        .expect("memory write execution post-write validation dry-run boundary json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_POST_WRITE_VALIDATION_DRY_RUN_BOUNDARY_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-live-mutation-operator-write-execution-post-write-validation-dry-run-boundary --json"
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
        value["memory_write_execution_post_write_validation_dry_run_boundary_ready"],
        true
    );
    assert_eq!(
        value["post_write_validation_mode"],
        "memory_write_execution_post_write_validation_dry_run_non_activation"
    );
    assert_eq!(
        value["source_memory_write_execution_write_enable_fixture_boundary_ready"],
        true
    );
    assert_eq!(value["minimum_required_samples"], 24);
    assert_eq!(
        value["memory_write_execution_post_write_validation_dry_run_ready"],
        true
    );
    assert_eq!(
        value["memory_write_execution_write_enable_fixture_ready"],
        true
    );
    assert_eq!(
        value["memory_write_execution_no_write_sink_contract_ready"],
        true
    );
    assert_eq!(value["required_write_enable_surface_count"], 10);
    assert_eq!(value["ready_write_enable_surface_count"], 10);
    assert_eq!(value["required_post_write_validation_surface_count"], 9);
    assert_eq!(value["ready_post_write_validation_surface_count"], 9);
    assert_eq!(
        value["side_effect_free_post_write_validation_surface_count"],
        9
    );
    assert_eq!(value["required_post_write_validation_fixture_count"], 8);
    assert_eq!(value["post_write_validation_fixture_count"], 8);
    assert_eq!(value["blocked_post_write_validation_fixture_count"], 8);
    assert_eq!(value["allowed_post_write_validation_fixture_count"], 0);
    assert_eq!(value["passed_post_write_validation_fixture_count"], 0);
    assert_eq!(value["post_write_validation_denied_count"], 8);
    assert_eq!(value["post_write_validation_performed_count"], 0);

    for key in [
        "post_write_validation_recorded",
        "post_write_validation_persisted",
        "post_write_validation_accepted",
        "post_write_validation_performed",
        "post_write_validation_report_written",
        "post_write_watchdog_soak_performed",
        "post_write_watchdog_soak_passed",
        "post_write_route_regression_check_performed",
        "post_write_dependency_isolation_check_performed",
        "post_write_memory_store_hash_recorded",
        "post_write_memory_store_hash_changed",
        "write_result_receipt_hash_recorded",
        "rollback_validation_performed",
        "rollback_validation_passed",
        "audit_redaction_validation_recorded",
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
        "external_send_enabled",
        "external_send_performed",
        "public_claim_or_release_artifact_write_enabled",
        "public_release_published",
        "release_artifact_written",
    ] {
        assert_eq!(value[key], false, "{key}");
    }
    assert_eq!(value["memory_store_write_performed_count"], 0);

    let surfaces = value["post_write_validation_surfaces"]
        .as_array()
        .expect("post-write validation surfaces");
    assert_eq!(surfaces.len(), 9);
    let fixtures = value["post_write_validation_fixtures"]
        .as_array()
        .expect("post-write validation fixtures");
    assert_eq!(fixtures.len(), 8);
    assert!(fixtures.iter().all(|fixture| {
        fixture["post_write_validation_requested"].as_bool() == Some(true)
            && fixture["validation_status"].as_str() == Some("blocked")
            && fixture["validation_allowed"].as_bool() == Some(false)
            && fixture["validation_performed"].as_bool() == Some(false)
            && fixture["validation_passed"].as_bool() == Some(false)
            && fixture["memory_store_write_performed"].as_bool() == Some(false)
            && fixture["memory_store_mutated"].as_bool() == Some(false)
            && fixture["rollback_executed"].as_bool() == Some(false)
            && fixture["activation_allowed"].as_bool() == Some(false)
    }));
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["route_readiness_regression_detected"] == true)
            .count(),
        1
    );
    assert!(
        fixtures
            .iter()
            .filter(|fixture| fixture["post_write_watchdog_soak_passed"] == false)
            .count()
            >= 1
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
    assert_eq!(value["denied_by_post_write_validation_count"], 14);
    assert_eq!(value["provider_invoked"], false);
    assert_eq!(value["model_invoked"], false);
    assert_eq!(value["credential_read"], false);
    assert_eq!(value["secret_file_read"], false);
    assert_eq!(value["kg_adapter_read_performed"], false);
    assert_eq!(value["live_kg_write_performed"], false);
    assert_eq!(value["channel_send_performed"], false);
    assert_eq!(value["telegram_send_performed"], false);
    assert_eq!(value["external_send_performed"], false);
    assert_eq!(value["public_artifact_written"], false);
    assert_eq!(value["public_release_claimed"], false);
    assert_eq!(value["install_executed"], false);
    assert_eq!(value["service_restarted"], false);
    assert_eq!(value["active_binary_mutated"], false);
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "run_memory_write_execution_post_write_validation_dry_run_boundary_require_live_gate"
    );
    assert_eq!(value["allowed_next_actions"][0]["writes_memory"], false);
    assert_eq!(
        value["allowed_next_actions"][1]["action"],
        "prepare_memory_write_execution_post_write_operator_acceptance_denial_boundary"
    );
    let side_effects = value["side_effects"]
        .as_object()
        .expect("memory write execution post-write validation dry-run boundary side effects");
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_write_execution_post_write_operator_acceptance_denial_boundary_endpoint_blocks_operator_acceptance_without_activation()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_POST_WRITE_OPERATOR_ACCEPTANCE_DENIAL_BOUNDARY_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body)
        .expect("memory write execution post-write operator acceptance denial boundary json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_POST_WRITE_OPERATOR_ACCEPTANCE_DENIAL_BOUNDARY_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-live-mutation-operator-write-execution-post-write-operator-acceptance-denial-boundary --json"
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
        value["memory_write_execution_post_write_operator_acceptance_denial_boundary_ready"],
        true
    );
    assert_eq!(
        value["operator_acceptance_denial_mode"],
        "memory_write_execution_post_write_operator_acceptance_denial_non_activation"
    );
    assert_eq!(
        value["source_memory_write_execution_post_write_validation_dry_run_boundary_ready"],
        true
    );
    assert_eq!(
        value["source_memory_write_execution_post_write_validation_dry_run_ready"],
        true
    );
    assert_eq!(value["minimum_required_samples"], 24);
    assert_eq!(
        value["memory_write_execution_post_write_operator_acceptance_denial_ready"],
        true
    );
    assert_eq!(
        value["memory_write_execution_post_write_validation_dry_run_ready"],
        true
    );
    assert_eq!(value["required_post_write_validation_surface_count"], 9);
    assert_eq!(value["ready_post_write_validation_surface_count"], 9);
    assert_eq!(value["required_operator_acceptance_surface_count"], 11);
    assert_eq!(value["ready_operator_acceptance_surface_count"], 11);
    assert_eq!(
        value["side_effect_free_operator_acceptance_surface_count"],
        11
    );
    assert_eq!(value["required_operator_acceptance_fixture_count"], 9);
    assert_eq!(value["operator_acceptance_fixture_count"], 9);
    assert_eq!(value["blocked_operator_acceptance_fixture_count"], 9);
    assert_eq!(value["allowed_operator_acceptance_fixture_count"], 0);
    assert_eq!(value["accepted_operator_acceptance_fixture_count"], 0);
    assert_eq!(value["operator_acceptance_denied_count"], 9);
    assert_eq!(value["operator_acceptance_performed_count"], 0);

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
        assert_eq!(value[key], false, "{key}");
    }
    assert_eq!(value["memory_store_write_performed_count"], 0);

    let surfaces = value["operator_acceptance_surfaces"]
        .as_array()
        .expect("operator acceptance surfaces");
    assert_eq!(surfaces.len(), 11);
    let fixtures = value["operator_acceptance_fixtures"]
        .as_array()
        .expect("operator acceptance fixtures");
    assert_eq!(fixtures.len(), 9);
    assert!(fixtures.iter().all(|fixture| {
        fixture["operator_acceptance_requested"].as_bool() == Some(true)
            && fixture["acceptance_status"].as_str() == Some("blocked")
            && fixture["acceptance_allowed"].as_bool() == Some(false)
            && fixture["acceptance_performed"].as_bool() == Some(false)
            && fixture["acceptance_accepted"].as_bool() == Some(false)
            && fixture["activation_allowed"].as_bool() == Some(false)
            && fixture["live_mutation_execution_performed"].as_bool() == Some(false)
            && fixture["memory_store_write_performed"].as_bool() == Some(false)
            && fixture["memory_store_mutated"].as_bool() == Some(false)
            && fixture["rollback_executed"].as_bool() == Some(false)
    }));
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["route_readiness_regression_detected"] == true)
            .count(),
        1
    );
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| fixture["direct_live_mutation_execution_requested"] == true)
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
    assert_eq!(value["denied_by_operator_acceptance_count"], 21);
    assert_eq!(
        value["denied_by_operator_acceptance"]
            .as_array()
            .expect("operator acceptance denials")
            .len(),
        21
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "run_memory_write_execution_post_write_operator_acceptance_denial_boundary_require_live_gate"
    );
    assert_eq!(value["allowed_next_actions"][0]["writes_memory"], false);
    assert_eq!(
        value["allowed_next_actions"][1]["action"],
        "prepare_memory_write_execution_activation_closure_denial_boundary"
    );
    let side_effects = value["side_effects"].as_object().expect(
        "memory write execution post-write operator acceptance denial boundary side effects",
    );
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_write_execution_activation_closure_denial_boundary_endpoint_blocks_closure_without_activation()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_CLOSURE_DENIAL_BOUNDARY_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body)
        .expect("memory write execution activation closure denial boundary json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_CLOSURE_DENIAL_BOUNDARY_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-live-mutation-operator-write-execution-activation-closure-denial-boundary --json"
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
        value["memory_write_execution_activation_closure_denial_boundary_ready"],
        true
    );
    assert_eq!(
        value["activation_closure_denial_mode"],
        "memory_write_execution_activation_closure_packet_no_write_denial"
    );
    assert_eq!(
        value["source_memory_write_execution_post_write_operator_acceptance_denial_boundary_ready"],
        true
    );
    assert_eq!(
        value["source_memory_write_execution_post_write_operator_acceptance_denial_ready"],
        true
    );
    assert_eq!(value["minimum_required_samples"], 24);
    assert_eq!(
        value["memory_write_execution_activation_closure_denial_ready"],
        true
    );
    assert_eq!(
        value["memory_write_execution_post_write_operator_acceptance_denial_ready"],
        true
    );
    assert_eq!(value["required_operator_acceptance_surface_count"], 11);
    assert_eq!(value["ready_operator_acceptance_surface_count"], 11);
    assert_eq!(value["required_activation_closure_surface_count"], 12);
    assert_eq!(value["ready_activation_closure_surface_count"], 12);
    assert_eq!(
        value["side_effect_free_activation_closure_surface_count"],
        12
    );
    assert_eq!(value["required_activation_closure_fixture_count"], 10);
    assert_eq!(value["activation_closure_fixture_count"], 10);
    assert_eq!(value["blocked_activation_closure_fixture_count"], 10);
    assert_eq!(value["allowed_activation_closure_fixture_count"], 0);
    assert_eq!(value["accepted_activation_closure_fixture_count"], 0);
    assert_eq!(value["activation_closure_denied_count"], 10);
    assert_eq!(value["activation_closure_performed_count"], 0);

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
        assert_eq!(value[key], false, "{key}");
    }
    assert_eq!(value["memory_store_write_performed_count"], 0);

    let surfaces = value["activation_closure_surfaces"]
        .as_array()
        .expect("activation closure surfaces");
    assert_eq!(surfaces.len(), 12);
    let fixtures = value["activation_closure_fixtures"]
        .as_array()
        .expect("activation closure fixtures");
    assert_eq!(fixtures.len(), 10);
    assert!(fixtures.iter().all(|fixture| {
        fixture["activation_closure_requested"].as_bool() == Some(true)
            && fixture["closure_status"].as_str() == Some("blocked")
            && fixture["closure_allowed"].as_bool() == Some(false)
            && fixture["closure_recorded"].as_bool() == Some(false)
            && fixture["closure_persisted"].as_bool() == Some(false)
            && fixture["closure_accepted"].as_bool() == Some(false)
            && fixture["activation_allowed"].as_bool() == Some(false)
            && fixture["live_mutation_execution_performed"].as_bool() == Some(false)
            && fixture["memory_store_write_performed"].as_bool() == Some(false)
            && fixture["memory_store_mutated"].as_bool() == Some(false)
            && fixture["rollback_executed"].as_bool() == Some(false)
    }));
    assert_eq!(
        fixtures
            .iter()
            .filter(|fixture| {
                fixture["direct_live_mutation_execution_requested"] == true
                    && fixture["activation_command_invoked"] == true
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
                fixture["activation_closure_filesystem_write_requested"] == true
                    && fixture["activation_closure_ledger_write_requested"] == true
            })
            .count(),
        1
    );
    assert_eq!(value["denied_by_activation_closure_count"], 24);
    assert_eq!(
        value["denied_by_activation_closure"]
            .as_array()
            .expect("activation closure denials")
            .len(),
        24
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "run_memory_write_execution_activation_closure_denial_boundary_require_live_gate"
    );
    assert_eq!(value["allowed_next_actions"][0]["writes_memory"], false);
    assert_eq!(
        value["allowed_next_actions"][1]["action"],
        "prepare_memory_write_execution_activation_command_no_op_handoff_boundary"
    );
    let side_effects = value["side_effects"]
        .as_object()
        .expect("memory write execution activation closure denial boundary side effects");
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}
