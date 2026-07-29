#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_endpoint_blocks_replay_reinstatement_and_session_lifecycle_authority()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator readiness packet template packet acceptance receipt release publication result receipt terminal distribution delivery receipt artifact download install affordance result receipt operator identity session revocation logout replay reinstatement route json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_denial_ready"],
        true
    );
    assert_eq!(
        value["source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_ready"],
        true
    );
    assert_eq!(
        value["source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_surface_count"],
        18
    );
    assert_eq!(
        value["source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_denied_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_surface_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_attempt_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_denied_count"],
        18
    );

    for key in [
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_revocation_logout_replay_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_revocation_logout_replay_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_logout_replay_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_logout_replay_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_identity_reinstatement_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_reinstatement_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_reinstatement_token_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_reinstatement_nonce_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_device_session_reinstatement_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_logout_replay_refresh_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_approval_from_revocation_logout_replay_reinstatement_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_replay_reinstatement_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_replay_reinstatement_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_replay_reinstatement_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_replay_reinstatement_service_restart_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_replay_reinstatement_active_binary_mutated_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_replay_reinstatement_external_send_count",
    ] {
        assert_eq!(value[key], 0, "{key}");
    }

    let surfaces = value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_surfaces"]
            .as_array()
            .expect("artifact download install affordance result receipt operator identity session revocation logout replay reinstatement surfaces");
    assert_eq!(surfaces.len(), 18);
    assert_eq!(
        surfaces[0]["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_surface"],
        "source_operator_identity_session_revocation_logout_report_required"
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["identity_revocation_replay_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["logout_replay_requested"] == true)
            .count(),
        3
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["identity_reinstatement_requested"] == true)
            .count(),
        5
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["session_reinstatement_requested"] == true)
            .count(),
        7
    );
    for surface in surfaces {
        assert_eq!(
            surface["artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_attempted"],
            true
        );
        assert_eq!(
            surface["operator_identity_session_revocation_logout_replay_reinstatement_noop_confirmed"],
            true
        );
        for key in [
            "revocation_logout_replay_accepted",
            "revocation_logout_replay_recorded",
            "revocation_logout_replay_persisted",
            "logout_replay_accepted",
            "logout_replay_recorded",
            "logout_replay_persisted",
            "identity_reinstatement_recorded",
            "identity_reinstatement_persisted",
            "session_reinstatement_recorded",
            "session_reinstatement_persisted",
            "reinstatement_token_recorded",
            "reinstatement_nonce_recorded",
            "device_session_reinstatement_recorded",
            "session_logout_replay_refresh_recorded",
            "identity_revocation_replay_status_promoted",
            "session_reinstatement_summary_promoted",
            "operator_approval_from_revocation_logout_replay_reinstatement_derived",
            "acceptance_from_revocation_logout_replay_reinstatement_recorded",
            "terminal_decision_from_revocation_logout_replay_reinstatement_recorded",
            "terminal_status_from_revocation_logout_replay_reinstatement_recorded",
            "release_publication_authority_from_revocation_logout_replay_reinstatement_derived",
            "activation_authority_from_revocation_logout_replay_reinstatement_derived",
            "download_link_from_revocation_logout_replay_reinstatement_rendered",
            "install_command_from_revocation_logout_replay_reinstatement_rendered",
            "install_from_revocation_logout_replay_reinstatement_executed",
            "service_restart_from_revocation_logout_replay_reinstatement_performed",
            "active_binary_from_revocation_logout_replay_reinstatement_mutated",
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

    let denied = value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement"]
            .as_array()
            .expect("artifact download install affordance result receipt operator identity session revocation logout replay reinstatement denials");
    assert_eq!(denied.len(), 17);
    assert_eq!(
        value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_denial_gate"
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
            "release publication result receipt terminal distribution delivery receipt artifact download install affordance result receipt operator identity session revocation logout replay reinstatement side effects",
        );
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_endpoint_blocks_ordering_monotonicity_and_lifecycle_authority()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_ORDERING_MONOTONICITY_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator readiness packet template packet acceptance receipt release publication result receipt terminal distribution delivery receipt artifact download install affordance result receipt operator identity session revocation logout replay reinstatement ordering monotonicity route json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_ORDERING_MONOTONICITY_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-ordering-monotonicity-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_denial_ready"],
        true
    );
    assert_eq!(
        value["source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ready"],
        true
    );
    assert_eq!(
        value["source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_surface_count"],
        18
    );
    assert_eq!(
        value["source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_denied_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_surface_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_attempt_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_denied_count"],
        18
    );

    for key in [
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_sequence_cursor_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_monotonicity_state_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_timestamp_rollback_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_latest_wins_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordered_delivery_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_approval_from_revocation_logout_replay_reinstatement_ordering_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_replay_reinstatement_ordering_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_replay_reinstatement_ordering_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_replay_reinstatement_ordering_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_replay_reinstatement_ordering_service_restart_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_replay_reinstatement_ordering_active_binary_mutated_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_replay_reinstatement_ordering_external_send_count",
    ] {
        assert_eq!(value[key], 0, "{key}");
    }

    let surfaces = value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_surfaces"]
            .as_array()
            .expect("artifact download install affordance result receipt operator identity session revocation logout replay reinstatement ordering monotonicity surfaces");
    assert_eq!(surfaces.len(), 18);
    assert_eq!(
        surfaces[0]["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_surface"],
        "source_operator_identity_session_revocation_logout_replay_reinstatement_report_required"
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["revocation_logout_replay_ordering_requested"] == true)
            .count(),
        5
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["logout_replay_sequence_requested"] == true)
            .count(),
        4
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["identity_reinstatement_ordering_requested"] == true)
            .count(),
        5
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["session_reinstatement_ordering_requested"] == true)
            .count(),
        7
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["telegram_ordered_delivery_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["install_restart_active_binary_ordering_requested"] == true)
            .count(),
        1
    );
    for surface in surfaces {
        assert_eq!(
            surface["artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_attempted"],
            true
        );
        assert_eq!(
            surface["operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_noop_confirmed"],
            true
        );
        for key in [
            "revocation_logout_replay_ordering_accepted",
            "logout_replay_sequence_accepted",
            "identity_reinstatement_ordering_accepted",
            "session_reinstatement_ordering_accepted",
            "ordering_recorded",
            "ordering_persisted",
            "ordering_materialized",
            "ordering_filesystem_written",
            "sequence_cursor_recorded",
            "sequence_cursor_persisted",
            "monotonicity_state_recorded",
            "monotonicity_state_persisted",
            "monotonicity_state_materialized",
            "timestamp_rollback_accepted",
            "epoch_rollback_accepted",
            "same_sequence_different_nonce_accepted",
            "late_arrival_accepted",
            "future_sequence_gap_accepted",
            "latest_wins_accepted",
            "monotonic_cursor_accepted",
            "ordered_delivery_accepted",
            "completion_order_recorded",
            "operator_approval_from_revocation_logout_replay_reinstatement_ordering_derived",
            "release_publication_authority_from_revocation_logout_replay_reinstatement_ordering_derived",
            "activation_authority_from_revocation_logout_replay_reinstatement_ordering_derived",
            "install_from_revocation_logout_replay_reinstatement_ordering_executed",
            "service_restart_from_revocation_logout_replay_reinstatement_ordering_performed",
            "active_binary_from_revocation_logout_replay_reinstatement_ordering_mutated",
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

    let denied = value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity"]
            .as_array()
            .expect("artifact download install affordance result receipt operator identity session revocation logout replay reinstatement ordering monotonicity denials");
    assert_eq!(denied.len(), 22);
    assert_eq!(
        value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_denial_gate"
    );
    for key in [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_ordering_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_monotonicity_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_lifecycle_latest_state_recorded",
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
            "release publication result receipt terminal distribution delivery receipt artifact download install affordance result receipt operator identity session revocation logout replay reinstatement ordering monotonicity side effects",
        );
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_endpoint_blocks_cancellation_supersession_and_lifecycle_authority()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_CANCELLATION_SUPERSESSION_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "operator readiness packet template packet acceptance receipt release publication result receipt terminal distribution delivery receipt artifact download install affordance result receipt operator identity session revocation logout replay reinstatement cancellation supersession route json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_CANCELLATION_SUPERSESSION_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-cancellation-supersession-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_denial_ready"],
        true
    );
    assert_eq!(
        value["source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_ready"],
        true
    );
    assert_eq!(
        value["source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_surface_count"],
        18
    );
    assert_eq!(
        value["source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_denied_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_surface_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_attempt_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_denied_count"],
        18
    );

    for key in [
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_supersession_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_supersession_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_replacement_receipt_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_replacement_receipt_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_tombstone_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_delete_marker_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_latest_replacement_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ack_replacement_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancelled_query_registered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_superseded_export_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_replacement_observability_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_lifecycle_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_approval_from_cancellation_supersession_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_cancellation_supersession_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_cancellation_supersession_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_cancellation_supersession_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_cancellation_supersession_service_restart_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_cancellation_supersession_active_binary_mutated_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_cancellation_supersession_external_send_count",
    ] {
        assert_eq!(value[key], 0, "{key}");
    }

    let surfaces = value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_surfaces"]
            .as_array()
            .expect("artifact download install affordance result receipt operator identity session revocation logout replay reinstatement cancellation supersession surfaces");
    assert_eq!(surfaces.len(), 18);
    assert_eq!(
        surfaces[0]["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_surface"],
        "source_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_report_required"
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["revocation_logout_replay_cancellation_requested"] == true)
            .count(),
        5
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["identity_reinstatement_withdrawal_requested"] == true)
            .count(),
        5
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["session_reinstatement_supersession_requested"] == true)
            .count(),
        8
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["replacement_receipt_requested"] == true)
            .count(),
        4
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["telegram_cancellation_supersession_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(
                |surface| surface["install_restart_active_binary_supersession_requested"] == true
            )
            .count(),
        1
    );
    for surface in surfaces {
        assert_eq!(
            surface["artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_attempted"],
            true
        );
        assert_eq!(
            surface["operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_noop_confirmed"],
            true
        );
        for key in [
            "cancellation_accepted",
            "cancellation_recorded",
            "cancellation_persisted",
            "supersession_accepted",
            "supersession_recorded",
            "supersession_persisted",
            "replacement_receipt_accepted",
            "replacement_receipt_recorded",
            "replacement_receipt_persisted",
            "tombstone_recorded",
            "tombstone_persisted",
            "delete_marker_recorded",
            "delete_marker_persisted",
            "latest_replacement_accepted",
            "ack_replacement_accepted",
            "cancelled_query_registered",
            "superseded_export_recorded",
            "replacement_observability_recorded",
            "ordering_replacement_accepted",
            "monotonicity_replacement_accepted",
            "lifecycle_cancellation_supersession_recorded",
            "result_receipt_from_cancellation_supersession_recorded",
            "operator_approval_from_cancellation_supersession_derived",
            "release_publication_authority_from_cancellation_supersession_derived",
            "activation_authority_from_cancellation_supersession_derived",
            "install_from_cancellation_supersession_executed",
            "service_restart_from_cancellation_supersession_performed",
            "active_binary_from_cancellation_supersession_mutated",
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

    let denied = value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession"]
            .as_array()
            .expect("artifact download install affordance result receipt operator identity session revocation logout replay reinstatement cancellation supersession denials");
    assert_eq!(denied.len(), 20);
    assert_eq!(
        value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_denial_gate"
    );
    for key in [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_cancellation_recorded",
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
            "release publication result receipt terminal distribution delivery receipt artifact download install affordance result receipt operator identity session revocation logout replay reinstatement cancellation supersession side effects",
        );
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_endpoint_blocks_audit_evidence_and_authority()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_AUDIT_EVIDENCE_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "artifact download install affordance result receipt operator identity session revocation logout replay reinstatement audit evidence route json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_AUDIT_EVIDENCE_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-audit-evidence-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_denial_ready"],
        true
    );
    assert_eq!(
        value["source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_ready"],
        true
    );
    assert_eq!(
        value["source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_surface_count"],
        18
    );
    assert_eq!(
        value["source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_denied_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_surface_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_attempt_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_denied_count"],
        18
    );

    for key in [
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_trail_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_trail_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_immutable_evidence_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_hash_chain_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ledger_evidence_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_acceptance_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_service_restart_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_active_binary_mutated_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_external_send_count",
    ] {
        assert_eq!(value[key], 0, "{key}");
    }

    let surfaces = value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_surfaces"]
            .as_array()
            .expect("artifact download install affordance result receipt operator identity session revocation logout replay reinstatement audit evidence surfaces");
    assert_eq!(surfaces.len(), 18);
    assert_eq!(
        surfaces[0]["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_surface"],
        "source_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_report_required"
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["audit_trail_append_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["immutable_evidence_requested"] == true)
            .count(),
        3
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["hash_chain_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["attestation_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["telegram_audit_evidence_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| {
                surface["install_evidence_requested"] == true
                    && surface["service_restart_evidence_requested"] == true
                    && surface["active_binary_evidence_requested"] == true
            })
            .count(),
        1
    );
    for surface in surfaces {
        assert_eq!(
            surface["artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_attempted"],
            true
        );
        assert_eq!(
            surface["operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_noop_confirmed"],
            true
        );
        for key in [
            "audit_trail_accepted",
            "audit_trail_recorded",
            "audit_trail_persisted",
            "audit_trail_filesystem_written",
            "immutable_evidence_accepted",
            "immutable_evidence_recorded",
            "immutable_evidence_persisted",
            "hash_chain_recorded",
            "merkle_root_recorded",
            "attestation_recorded",
            "witness_recorded",
            "notary_recorded",
            "ledger_evidence_recorded",
            "index_evidence_recorded",
            "delivery_evidence_recorded",
            "export_evidence_recorded",
            "query_evidence_registered",
            "observability_evidence_recorded",
            "readback_evidence_recorded",
            "completion_ack_from_audit_evidence_recorded",
            "cancellation_supersession_evidence_recorded",
            "ordering_monotonicity_evidence_recorded",
            "replay_idempotency_evidence_recorded",
            "audit_evidence_acceptance_recorded",
            "operator_approval_from_audit_evidence_derived",
            "release_publication_authority_from_audit_evidence_derived",
            "activation_authority_from_audit_evidence_derived",
            "install_from_audit_evidence_executed",
            "service_restart_from_audit_evidence_performed",
            "active_binary_from_audit_evidence_mutated",
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

    let denied = value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence"]
            .as_array()
            .expect("artifact download install affordance result receipt operator identity session revocation logout replay reinstatement audit evidence denials");
    assert_eq!(denied.len(), 22);
    assert_eq!(
        value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_denial_gate"
    );
    for key in [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_trail_recorded",
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
            "release publication result receipt terminal distribution delivery receipt artifact download install affordance result receipt operator identity session revocation logout replay reinstatement audit evidence side effects",
        );
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_endpoint_blocks_retention_gc_and_authority()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_RETENTION_EXPIRY_GARBAGE_COLLECTION_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "artifact download install affordance result receipt operator identity session revocation logout replay reinstatement retention expiry garbage collection route json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_RETENTION_EXPIRY_GARBAGE_COLLECTION_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-retention-expiry-garbage-collection-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_denial_ready"],
        true
    );
    assert_eq!(
        value["source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_ready"],
        true
    );
    assert_eq!(
        value["source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_surface_count"],
        18
    );
    assert_eq!(
        value["source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_denied_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_surface_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_attempt_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_denied_count"],
        18
    );

    for key in [
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_policy_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_policy_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_expiry_scheduler_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_garbage_collection_queue_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_garbage_collection_scan_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_delete_marker_gc_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_archive_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_compaction_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_acceptance_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_service_restart_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_active_binary_mutated_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_external_send_count",
    ] {
        assert_eq!(value[key], 0, "{key}");
    }

    let surfaces = value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_surfaces"]
            .as_array()
            .expect("artifact download install affordance result receipt operator identity session revocation logout replay reinstatement retention expiry garbage collection surfaces");
    assert_eq!(surfaces.len(), 18);
    assert_eq!(
        surfaces[0]["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_surface"],
        "source_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_report_required"
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["retention_policy_requested"] == true)
            .count(),
        5
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["expiry_scheduler_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["garbage_collection_queue_requested"] == true)
            .count(),
        2
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["external_telegram_retention_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["live_install_gc_evidence_requested"] == true)
            .count(),
        1
    );
    for surface in surfaces {
        assert_eq!(
            surface["artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_attempted"],
            true
        );
        assert_eq!(
            surface["operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_noop_confirmed"],
            true
        );
        for key in [
            "retention_policy_accepted",
            "retention_policy_recorded",
            "retention_policy_persisted",
            "expiry_scheduler_recorded",
            "expiry_timer_started",
            "expiry_ack_recorded",
            "garbage_collection_queue_recorded",
            "garbage_collection_scan_performed",
            "garbage_collection_candidate_recorded",
            "garbage_collection_decision_recorded",
            "tombstone_gc_recorded",
            "delete_marker_gc_recorded",
            "archive_recorded",
            "compaction_recorded",
            "result_receipt_from_retention_recorded",
            "result_receipt_from_retention_persisted",
            "operator_approval_from_retention_derived",
            "release_publication_authority_from_retention_derived",
            "activation_authority_from_retention_derived",
            "install_from_retention_executed",
            "service_restart_from_retention_performed",
            "active_binary_from_retention_mutated",
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

    let denied = value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection"]
            .as_array()
            .expect("artifact download install affordance result receipt operator identity session revocation logout replay reinstatement retention expiry garbage collection denials");
    assert_eq!(denied.len(), 22);
    assert_eq!(
        value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_denial_gate"
    );
    for key in [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_expiry_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_garbage_collection_recorded",
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
            "release publication result receipt terminal distribution delivery receipt artifact download install affordance result receipt operator identity session revocation logout replay reinstatement retention expiry garbage collection side effects",
        );
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_endpoint_blocks_views_and_authority()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_EXPORT_QUERY_OBSERVABILITY_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "artifact download install affordance result receipt operator identity session revocation logout replay reinstatement export query observability route json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_EXPORT_QUERY_OBSERVABILITY_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-export-query-observability-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_denial_ready"],
        true
    );
    assert_eq!(
        value["source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_ready"],
        true
    );
    assert_eq!(
        value["source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_surface_count"],
        18
    );
    assert_eq!(
        value["source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_denied_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_surface_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_attempt_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_denied_count"],
        18
    );

    for key in [
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_query_registered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_query_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_file_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_observability_metric_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_dashboard_panel_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_acceptance_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_service_restart_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_active_binary_mutated_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_external_send_count",
    ] {
        assert_eq!(value[key], 0, "{key}");
    }

    let surfaces = value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_surfaces"]
            .as_array()
            .expect("artifact download install affordance result receipt operator identity session revocation logout replay reinstatement export query observability surfaces");
    assert_eq!(surfaces.len(), 18);
    assert_eq!(
        surfaces[0]["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_surface"],
        "source_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_report_required"
    );
    assert_eq!(
            surfaces
                .iter()
                .filter(|surface| surface["operator_identity_session_revocation_logout_replay_reinstatement_query_requested"] == true)
                .count(),
            3
        );
    assert_eq!(
            surfaces
                .iter()
                .filter(|surface| surface["operator_identity_session_revocation_logout_replay_reinstatement_export_requested"] == true)
                .count(),
            4
        );
    assert_eq!(
            surfaces
                .iter()
                .filter(|surface| surface["operator_identity_session_revocation_logout_replay_reinstatement_observability_requested"] == true)
                .count(),
            3
        );
    assert_eq!(
        surfaces
            .iter()
            .filter(
                |surface| surface["external_observability_requested"] == true
                    && surface["telegram_observability_requested"] == true
            )
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["live_view_requested"] == true
                && surface["install_view_requested"] == true
                && surface["active_binary_view_requested"] == true)
            .count(),
        1
    );
    for surface in surfaces {
        assert_eq!(
            surface["artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_attempted"],
            true
        );
        assert_eq!(
            surface["operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_noop_confirmed"],
            true
        );
        for key in [
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
            "operator_summary_recorded",
            "result_receipt_recorded",
            "result_receipt_persisted",
            "operator_approval_from_export_query_observability_accepted",
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

    let denied = value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability"]
            .as_array()
            .expect("artifact download install affordance result receipt operator identity session revocation logout replay reinstatement export query observability denials");
    assert_eq!(denied.len(), 22);
    assert_eq!(
        value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_facing_summary_briefing_non_persistence_denial_gate"
    );
    for key in [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_query_registered",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_query_executed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_observability_metric_recorded",
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
            "release publication result receipt terminal distribution delivery receipt artifact download install affordance result receipt operator identity session revocation logout replay reinstatement export query observability side effects",
        );
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_facing_summary_briefing_endpoint_blocks_briefings_and_delivery()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_FACING_SUMMARY_BRIEFING_NON_PERSISTENCE_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "artifact download install affordance result receipt operator identity session revocation logout replay reinstatement operator facing summary briefing route json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_FACING_SUMMARY_BRIEFING_NON_PERSISTENCE_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-facing-summary-briefing-non-persistence-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_facing_summary_briefing_non_persistence_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_facing_summary_briefing_non_persistence_denial_ready"],
        true
    );
    assert_eq!(
        value["source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_ready"],
        true
    );
    assert_eq!(
        value["source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_surface_count"],
        18
    );
    assert_eq!(
        value["source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_denied_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_facing_summary_briefing_surface_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_facing_summary_briefing_attempt_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_facing_summary_briefing_denied_count"],
        18
    );

    for key in [
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_summary_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_briefing_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_readback_digest_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_status_banner_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_briefing_delivery_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_external_briefing_sent_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_telegram_briefing_sent_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_summary_briefing_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_summary_briefing_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_summary_briefing_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_summary_briefing_service_restart_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_summary_briefing_active_binary_mutated_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_summary_briefing_external_send_count",
    ] {
        assert_eq!(value[key], 0, "{key}");
    }

    let surfaces = value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_facing_summary_briefing_surfaces"]
            .as_array()
            .expect("artifact download install affordance result receipt operator identity session revocation logout replay reinstatement operator facing summary briefing surfaces");
    assert_eq!(surfaces.len(), 18);
    assert_eq!(
        surfaces[0]["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_facing_summary_briefing_surface"],
        "source_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_report_required"
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["operator_summary_requested"] == true)
            .count(),
        4
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["operator_briefing_requested"] == true)
            .count(),
        5
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["readback_digest_requested"] == true)
            .count(),
        3
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
            .filter(|surface| surface["live_status_briefing_requested"] == true
                && surface["install_status_briefing_requested"] == true
                && surface["active_binary_status_briefing_requested"] == true)
            .count(),
        1
    );
    for surface in surfaces {
        assert_eq!(
            surface["artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_facing_summary_briefing_attempted"],
            true
        );
        assert_eq!(
            surface["operator_identity_session_revocation_logout_replay_reinstatement_operator_facing_summary_briefing_noop_confirmed"],
            true
        );
        for key in [
            "operator_summary_recorded",
            "operator_summary_persisted",
            "operator_briefing_recorded",
            "operator_briefing_persisted",
            "readback_digest_recorded",
            "status_banner_recorded",
            "exported_summary_text_recorded",
            "operator_briefing_card_materialized",
            "briefing_delivery_recorded",
            "briefing_delivery_performed",
            "external_briefing_sent",
            "telegram_briefing_sent",
            "summary_briefing_acceptance_recorded",
            "operator_approval_from_summary_briefing_accepted",
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

    let denied = value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_facing_summary_briefing"]
            .as_array()
            .expect("artifact download install affordance result receipt operator identity session revocation logout replay reinstatement operator facing summary briefing denials");
    assert_eq!(denied.len(), 18);
    assert_eq!(
        value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_facing_summary_briefing_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_non_acceptance_denial_gate"
    );
    for key in [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_summary_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_briefing_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_readback_digest_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_status_banner_recorded",
        "artifact_download_install_affordance_result_receipt_recorded",
        "artifact_download_install_affordance_result_receipt_persisted",
        "artifact_download_install_affordance_result_receipt_operator_summary_recorded",
        "artifact_download_install_affordance_result_receipt_operator_briefing_recorded",
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
            "release publication result receipt terminal distribution delivery receipt artifact download install affordance result receipt operator identity session revocation logout replay reinstatement operator facing summary briefing side effects",
        );
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_endpoint_blocks_acknowledgement_and_authority()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_FINAL_OPERATOR_ACKNOWLEDGEMENT_NON_ACCEPTANCE_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "artifact download install affordance result receipt operator identity session revocation logout replay reinstatement final operator acknowledgement route json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_FINAL_OPERATOR_ACKNOWLEDGEMENT_NON_ACCEPTANCE_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-final-operator-acknowledgement-non-acceptance-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_non_acceptance_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_non_acceptance_denial_ready"],
        true
    );
    assert_eq!(
        value["source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_facing_summary_briefing_ready"],
        true
    );
    assert_eq!(
        value["source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_facing_summary_briefing_surface_count"],
        18
    );
    assert_eq!(
        value["source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_facing_summary_briefing_denied_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_surface_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_attempt_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_denied_count"],
        18
    );

    for key in [
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_received_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_confirmed_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_read_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_seen_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_response_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_completion_acknowledgement_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_status_acknowledgement_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_final_acknowledgement_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_final_acknowledgement_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_final_acknowledgement_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_final_acknowledgement_service_restart_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_final_acknowledgement_active_binary_mutated_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_final_acknowledgement_external_send_count",
    ] {
        assert_eq!(value[key], 0, "{key}");
    }

    let surfaces = value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_surfaces"]
            .as_array()
            .expect("artifact download install affordance result receipt operator identity session revocation logout replay reinstatement final operator acknowledgement surfaces");
    assert_eq!(surfaces.len(), 18);
    assert_eq!(
        surfaces[0]["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_surface"],
        "source_operator_identity_session_revocation_logout_replay_reinstatement_summary_briefing_report_required"
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
            .filter(
                |surface| surface["install_restart_active_binary_acknowledgement_requested"]
                    == true
            )
            .count(),
        1
    );
    for surface in surfaces {
        assert_eq!(
            surface["artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_attempted"],
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

    let denied = value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement"]
            .as_array()
            .expect("artifact download install affordance result receipt operator identity session revocation logout replay reinstatement final operator acknowledgement denials");
    assert_eq!(denied.len(), 23);
    assert_eq!(
        value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_denial_gate"
    );
    for key in [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_persisted",
        "artifact_download_install_affordance_result_receipt_recorded",
        "artifact_download_install_affordance_result_receipt_persisted",
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
            "release publication result receipt terminal distribution delivery receipt artifact download install affordance result receipt operator identity session revocation logout replay reinstatement final operator acknowledgement side effects",
        );
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_endpoint_blocks_status_promotion_and_authority()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_TERMINAL_DECISION_STATUS_PROMOTION_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "artifact download install affordance result receipt operator identity session revocation logout replay reinstatement terminal decision status promotion route json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_TERMINAL_DECISION_STATUS_PROMOTION_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-terminal-decision-status-promotion-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_denial_ready"],
        true
    );
    assert_eq!(
        value["source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_ready"],
        true
    );
    assert_eq!(
        value["source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_surface_count"],
        18
    );
    assert_eq!(
        value["source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_final_operator_acknowledgement_denied_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_surface_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_attempt_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_denied_count"],
        18
    );

    for key in [
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_status_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_status_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_status_promotion_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_channel_decision_delivered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_external_decision_sent_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_telegram_decision_sent_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_terminal_status_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_terminal_status_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_terminal_status_active_binary_mutated_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_terminal_status_external_send_count",
    ] {
        assert_eq!(value[key], 0, "{key}");
    }

    let surfaces = value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_surfaces"]
            .as_array()
            .expect("artifact download install affordance result receipt operator identity session revocation logout replay reinstatement terminal decision status promotion surfaces");
    assert_eq!(surfaces.len(), 18);
    assert_eq!(
        surfaces[0]["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_surface"],
        "source_operator_identity_session_revocation_logout_replay_reinstatement_final_acknowledgement_report_required"
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
            surface["artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_attempted"],
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
            "terminal_status_recorded",
            "terminal_status_persisted",
            "status_promotion_recorded",
            "revocation_replay_terminal_decision_recorded",
            "logout_replay_status_promoted",
            "identity_reinstatement_terminal_status_recorded",
            "session_reinstatement_decision_recorded",
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

    let denied = value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion"]
            .as_array()
            .expect("artifact download install affordance result receipt operator identity session revocation logout replay reinstatement terminal decision status promotion denials");
    assert_eq!(denied.len(), 22);
    assert_eq!(
        value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_denial_gate"
    );
    let next_actions = value["allowed_next_actions"]
        .as_array()
        .expect("terminal decision status promotion next actions");
    assert!(next_actions.iter().any(|action| {
            action["action"]
                == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_denial_gate"
                && action["records_operator_intent"] == serde_json::json!(false)
                && action["records_operator_consent"] == serde_json::json!(false)
        }));
    for key in [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_status_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_status_promotion_recorded",
        "artifact_download_install_affordance_result_receipt_recorded",
        "artifact_download_install_affordance_result_receipt_persisted",
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
            "release publication result receipt terminal distribution delivery receipt artifact download install affordance result receipt operator identity session revocation logout replay reinstatement terminal decision status promotion side effects",
        );
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_endpoint_blocks_public_exposure_and_authority()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_TERMINAL_PUBLIC_CLAIM_STATUS_EXPOSURE_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "artifact download install affordance result receipt operator identity session revocation logout replay reinstatement terminal public claim status exposure route json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_TERMINAL_PUBLIC_CLAIM_STATUS_EXPOSURE_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-terminal-public-claim-status-exposure-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_denial_ready"],
        true
    );
    assert_eq!(
        value["source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_ready"],
        true
    );
    assert_eq!(
        value["source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_surface_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_surface_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_attempt_count"],
        18
    );
    for key in [
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_recorded_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_status_recorded_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_status_promotion_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposed_count",
        "release_publication_result_receipt_public_status_claimed_count",
        "release_publication_result_receipt_public_release_claimed_count",
        "release_publication_result_receipt_public_ga_claimed_count",
        "release_publication_result_receipt_status_endpoint_exposed_count",
        "release_publication_result_receipt_query_status_exposed_count",
        "release_publication_result_receipt_export_status_exposed_count",
        "release_publication_result_receipt_observability_status_exposed_count",
        "release_publication_result_receipt_channel_status_delivered_count",
        "release_publication_result_receipt_external_status_sent_count",
        "release_publication_result_receipt_telegram_status_sent_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_live_execution_allowed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_service_restarted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_active_binary_mutated_count",
    ] {
        assert_eq!(value[key], 0, "{key}");
    }

    let surfaces = value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_surfaces"]
            .as_array()
            .expect("artifact download install affordance result receipt operator identity session revocation logout replay reinstatement terminal public claim status exposure surfaces");
    assert_eq!(surfaces.len(), 18);
    assert_eq!(
        surfaces[0]["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_surface"],
        "revocation_replay_public_claim_status_claim"
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["telegram_status_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(
                |surface| surface["install_restart_active_binary_public_status_requested"] == true,
            )
            .count(),
        1
    );
    for surface in surfaces {
        assert_eq!(surface["public_claim_status_exposure_attempted"], true);
        assert_eq!(surface["public_claim_status_exposure_noop_confirmed"], true);
        for key in [
            "public_claim_status_exposure_accepted",
            "public_claim_status_exposure_recorded",
            "public_claim_status_exposed",
            "public_status_claimed",
            "public_release_claimed",
            "public_ga_claimed",
            "release_status_exposed",
            "publication_status_exposed",
            "dashboard_status_exposed",
            "status_endpoint_exposed",
            "query_status_exposed",
            "export_status_exposed",
            "observability_status_exposed",
            "channel_status_delivered",
            "external_status_sent",
            "telegram_status_sent",
            "operator_approval_derived",
            "release_publication_authority_derived",
            "activation_authority_derived",
            "live_execution_allowed",
            "install_executed",
            "service_restarted",
            "active_binary_mutated",
            "memory_store_write_performed",
            "live_kg_write_performed",
            "provider_invoked",
            "model_invoked",
            "credential_read",
            "secret_file_read",
        ] {
            assert_eq!(surface[key], false, "{key}");
        }
    }

    let denied = value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure"]
            .as_array()
            .expect("artifact download install affordance result receipt operator identity session revocation logout replay reinstatement terminal public claim status exposure denials");
    assert_eq!(denied.len(), 34);
    assert_eq!(
        value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_denial_gate"
    );
    for key in [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_status_endpoint_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_query_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_export_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_observability_status_exposed",
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
            "release publication result receipt terminal distribution delivery receipt artifact download install affordance result receipt operator identity session revocation logout replay reinstatement terminal public claim status exposure side effects",
        );
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_endpoint_blocks_reconfirmation_and_authority()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_RECONFIRMATION_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "artifact download install affordance result receipt operator identity session revocation logout replay reinstatement operator intent consent route json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_RECONFIRMATION_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-reconfirmation-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_denial_ready"],
        true
    );
    assert_eq!(
        value["source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_ready"],
        true
    );
    assert_eq!(
        value["source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_surface_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_surface_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_attempt_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_denied_count"],
        18
    );
    for key in [
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_recorded_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposed_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_release_publication_authority_derived_count",
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_public_claim_status_exposure_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_reconfirmed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_consent_reconfirmed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_consent_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_consent_reconfirmation_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_approval_from_intent_consent_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_intent_consent_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_intent_consent_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_intent_consent_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_intent_consent_service_restart_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_intent_consent_active_binary_mutated_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_intent_consent_external_send_count",
    ] {
        assert_eq!(value[key], 0, "{key}");
    }

    let surfaces = value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_surfaces"]
            .as_array()
            .expect("operator identity session revocation logout replay reinstatement operator intent consent surfaces");
    assert_eq!(surfaces.len(), 18);
    assert_eq!(
        surfaces[0]["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_surface"],
        "source_terminal_public_claim_status_exposure_report_required"
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
            surface["artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_attempted"],
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
            "operator_consent_recorded",
            "consent_reconfirmation_recorded",
            "operator_approval_from_intent_consent_derived",
            "release_publication_authority_from_intent_consent_derived",
            "activation_authority_from_intent_consent_derived",
            "install_from_intent_consent_executed",
            "service_restart_from_intent_consent_performed",
            "active_binary_from_intent_consent_mutated",
            "memory_store_write_performed",
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

    let denied = value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation"]
            .as_array()
            .expect("operator identity session revocation logout replay reinstatement operator intent consent denials");
    assert_eq!(denied.len(), 19);
    assert_eq!(
        value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_denial_gate"
    );
    for key in [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_consent_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_consent_reconfirmation_recorded",
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
        "external_send_performed",
    ] {
        assert_eq!(value[key], false, "{key}");
    }
    let side_effects = value["side_effects"].as_object().expect(
            "release publication result receipt terminal distribution delivery receipt artifact download install affordance result receipt operator identity session revocation logout replay reinstatement operator intent consent side effects",
        );
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_endpoint_blocks_evidence_and_authority()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_PERSISTENCE_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect(
            "artifact download install affordance result receipt operator identity session revocation logout replay reinstatement operator intent consent evidence persistence route json",
        );
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_PERSISTENCE_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-persistence-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_denial_ready"],
        true
    );
    assert_eq!(
        value["source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_ready"],
        true
    );
    assert_eq!(
        value["source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_surface_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_surface_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_attempt_count"],
        18
    );
    assert_eq!(
        value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_denied_count"],
        18
    );
    for key in [
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_evidence_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_consent_evidence_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_intent_consent_evidence_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_intent_consent_evidence_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_evidence_receipt_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_evidence_receipt_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_evidence_materialized_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_evidence_filesystem_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_evidence_ledger_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_evidence_indexed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_evidence_exported_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_evidence_query_registered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_evidence_observability_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_identity_session_binding_from_evidence_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_approval_from_evidence_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_evidence_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_evidence_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_evidence_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_evidence_active_binary_mutated_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_evidence_external_send_count",
    ] {
        assert_eq!(value[key], 0, "{key}");
    }

    let surfaces = value["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_surfaces"]
            .as_array()
            .expect("operator identity session revocation logout replay reinstatement operator intent consent evidence persistence surfaces");
    assert_eq!(surfaces.len(), 18);
    assert_eq!(
        surfaces[0]["release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_surface"],
        "source_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_report_required"
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["intent_evidence_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["consent_evidence_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["telegram_consent_evidence_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["install_restart_active_binary_evidence_requested"] == true)
            .count(),
        1
    );
    for surface in surfaces {
        assert_eq!(
            surface["artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_attempted"],
            true
        );
        assert_eq!(
            surface["operator_intent_consent_evidence_persistence_noop_confirmed"],
            true
        );
        for key in [
            "operator_intent_evidence_recorded",
            "operator_consent_evidence_recorded",
            "intent_consent_evidence_recorded",
            "intent_consent_evidence_persisted",
            "evidence_receipt_recorded",
            "evidence_receipt_persisted",
            "evidence_materialized",
            "evidence_filesystem_written",
            "evidence_ledger_written",
            "evidence_indexed",
            "evidence_exported",
            "evidence_query_registered",
            "evidence_observability_recorded",
            "identity_session_binding_from_evidence_recorded",
            "operator_approval_from_evidence_derived",
            "release_publication_authority_from_evidence_derived",
            "activation_authority_from_evidence_derived",
            "install_from_evidence_executed",
            "service_restart_from_evidence_performed",
            "active_binary_from_evidence_mutated",
            "memory_store_write_performed",
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

    let denied = value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence"]
            .as_array()
            .expect("operator identity session revocation logout replay reinstatement operator intent consent evidence persistence denials");
    assert_eq!(denied.len(), 18);
    assert_eq!(
        value["denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_export_query_observability_denial_gate"
    );
    for key in [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_evidence_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_consent_evidence_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_intent_consent_evidence_persisted",
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
        "external_send_performed",
    ] {
        assert_eq!(value[key], false, "{key}");
    }
    let side_effects = value["side_effects"].as_object().expect(
            "release publication result receipt terminal distribution delivery receipt artifact download install affordance result receipt operator identity session revocation logout replay reinstatement operator intent consent evidence persistence side effects",
        );
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_artifact_signing_receipt_no_persistence_endpoint_blocks_receipts()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_RECEIPT_NO_PERSISTENCE_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value =
        serde_json::from_str(&body).expect("artifact signing receipt no-persistence route json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_RECEIPT_NO_PERSISTENCE_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-no-persistence-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_no_persistence_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_no_persistence_denial_ready"],
        true
    );
    assert_eq!(
        value["source_operator_intent_consent_evidence_persistence_ready"],
        true
    );
    assert_eq!(
        value["source_artifact_distribution_signing_notarization_surface_ready"],
        true
    );
    assert_eq!(
        value["source_artifact_distribution_signing_notarization_surface_count"],
        18
    );
    assert_eq!(value["source_artifact_signing_executed_count"], 0);
    assert_eq!(value["source_package_signing_executed_count"], 0);
    assert_eq!(value["source_notarization_submitted_count"], 0);
    assert_eq!(
        value["source_release_publication_authority_from_signing_status_derived_count"],
        0
    );
    assert_eq!(
        value["source_activation_authority_from_signing_status_derived_count"],
        0
    );
    assert_eq!(
        value["artifact_distribution_signing_notarization_result_receipt_surface_count"],
        18
    );
    assert_eq!(
        value["artifact_distribution_signing_notarization_result_receipt_surface_attempt_count"],
        18
    );
    assert_eq!(
        value["artifact_distribution_signing_notarization_result_receipt_surface_denied_count"],
        18
    );
    for key in [
        "artifact_distribution_signing_notarization_result_receipt_surface_allowed_count",
        "artifact_distribution_signing_notarization_result_receipt_surface_request_accepted_count",
        "artifact_distribution_signing_notarization_result_receipt_surface_accepted_count",
        "artifact_distribution_signing_notarization_result_receipt_surface_recorded_count",
        "artifact_distribution_signing_notarization_result_receipt_surface_persisted_count",
        "artifact_distribution_signing_notarization_result_receipt_surface_materialized_count",
        "artifact_distribution_signing_notarization_result_receipt_surface_filesystem_written_count",
        "artifact_distribution_signing_notarization_result_receipt_surface_delivered_count",
        "artifact_distribution_signing_notarization_result_receipt_surface_indexed_count",
        "artifact_distribution_signing_notarization_result_receipt_surface_exported_count",
        "artifact_distribution_signing_notarization_result_receipt_surface_query_registered_count",
        "artifact_distribution_signing_notarization_result_receipt_surface_observability_recorded_count",
        "artifact_distribution_signing_notarization_result_receipt_surface_status_exposed_count",
        "artifact_signing_receipt_accepted_count",
        "package_signing_receipt_accepted_count",
        "notarization_submission_receipt_persisted_count",
        "stapling_receipt_filesystem_written_count",
        "installer_signing_receipt_delivered_count",
        "release_publication_authority_from_signing_receipt_derived_count",
        "activation_authority_from_signing_receipt_derived_count",
        "install_from_signing_receipt_executed_count",
        "service_restart_from_signing_receipt_performed_count",
        "active_binary_from_signing_receipt_mutated_count",
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

    let surfaces = value["artifact_distribution_signing_notarization_result_receipt_surfaces"]
        .as_array()
        .expect("artifact signing receipt no-persistence surfaces");
    assert_eq!(surfaces.len(), 18);
    assert_eq!(
        surfaces[0]["surface"],
        "source_signing_notarization_surface_report_required"
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["artifact_signing_receipt_acceptance_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(
                |surface| surface["notarization_submission_receipt_persistence_requested"] == true
            )
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["telegram_signing_receipt_delivery_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(
                |surface| surface["install_restart_active_binary_from_signing_receipt_requested"]
                    == true
            )
            .count(),
        1
    );
    for surface in surfaces {
        assert_eq!(
            surface["artifact_distribution_signing_notarization_result_receipt_surface_attempted"],
            true
        );
        assert_eq!(
            surface["artifact_distribution_signing_notarization_result_receipt_surface_noop_confirmed"],
            true
        );
        for key in [
            "artifact_distribution_signing_notarization_result_receipt_surface_allowed",
            "artifact_distribution_signing_notarization_result_receipt_surface_request_accepted",
            "artifact_distribution_signing_notarization_result_receipt_surface_accepted",
            "artifact_distribution_signing_notarization_result_receipt_surface_recorded",
            "artifact_distribution_signing_notarization_result_receipt_surface_persisted",
            "artifact_distribution_signing_notarization_result_receipt_surface_materialized",
            "artifact_distribution_signing_notarization_result_receipt_surface_filesystem_written",
            "artifact_distribution_signing_notarization_result_receipt_surface_delivered",
            "artifact_distribution_signing_notarization_result_receipt_surface_status_exposed",
            "artifact_signing_receipt_accepted",
            "package_signing_receipt_accepted",
            "notarization_submission_receipt_persisted",
            "stapling_receipt_filesystem_written",
            "installer_signing_receipt_delivered",
            "release_publication_authority_derived",
            "activation_authority_derived",
            "download_link_rendered",
            "install_command_rendered",
            "install_executed",
            "service_restarted",
            "active_binary_mutated",
            "memory_store_write_performed",
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

    let denied = value["denied_by_artifact_distribution_signing_notarization_result_receipt"]
        .as_array()
        .expect("artifact signing receipt no-persistence denials");
    assert_eq!(denied.len(), 31);
    assert_eq!(
        value["denied_by_artifact_distribution_signing_notarization_result_receipt_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_replay_idempotency_denial_gate"
    );
    for key in [
        "artifact_distribution_signing_notarization_result_receipt_accepted",
        "artifact_distribution_signing_notarization_result_receipt_recorded",
        "artifact_distribution_signing_notarization_result_receipt_persisted",
        "artifact_distribution_signing_notarization_result_receipt_materialized",
        "artifact_distribution_signing_notarization_result_receipt_delivered",
        "artifact_distribution_signing_notarization_result_receipt_status_exposed",
        "artifact_signing_receipt_accepted",
        "package_signing_receipt_accepted",
        "notarization_submission_receipt_persisted",
        "stapling_receipt_filesystem_written",
        "installer_signing_receipt_delivered",
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
        "telegram_send_performed",
        "channel_send_performed",
        "external_send_performed",
        "release_artifact_written",
        "public_artifact_written",
        "filesystem_written",
    ] {
        assert_eq!(value[key], false, "{key}");
    }
    let side_effects = value["side_effects"]
        .as_object()
        .expect("artifact signing receipt no-persistence side effects");
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_artifact_signing_receipt_replay_idempotency_endpoint_blocks_replay_and_authority()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_RECEIPT_REPLAY_IDEMPOTENCY_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body)
        .expect("artifact signing receipt replay idempotency route json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_RECEIPT_REPLAY_IDEMPOTENCY_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-replay-idempotency-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_replay_idempotency_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_replay_idempotency_denial_ready"],
        true
    );
    assert_eq!(
        value["source_artifact_distribution_signing_notarization_result_receipt_ready"],
        true
    );
    assert_eq!(
        value["source_artifact_distribution_signing_notarization_result_receipt_surface_count"],
        18
    );
    assert_eq!(
        value["artifact_distribution_signing_notarization_receipt_replay_idempotency_surface_count"],
        18
    );
    assert_eq!(
        value["artifact_distribution_signing_notarization_receipt_replay_idempotency_attempt_count"],
        18
    );
    assert_eq!(
        value["artifact_distribution_signing_notarization_receipt_replay_idempotency_denied_count"],
        18
    );
    for key in [
        "source_artifact_distribution_signing_notarization_result_receipt_surface_recorded_count",
        "source_artifact_distribution_signing_notarization_result_receipt_surface_persisted_count",
        "source_artifact_distribution_signing_notarization_result_receipt_surface_status_exposed_count",
        "source_release_publication_authority_from_signing_receipt_derived_count",
        "source_activation_authority_from_signing_receipt_derived_count",
        "artifact_distribution_signing_notarization_receipt_replay_idempotency_allowed_count",
        "artifact_distribution_signing_notarization_receipt_replay_allowed_count",
        "artifact_distribution_signing_notarization_receipt_replay_accepted_count",
        "artifact_distribution_signing_notarization_receipt_replay_recorded_count",
        "artifact_distribution_signing_notarization_receipt_replay_persisted_count",
        "artifact_distribution_signing_notarization_receipt_duplicate_accepted_count",
        "artifact_distribution_signing_notarization_receipt_duplicate_recorded_count",
        "artifact_distribution_signing_notarization_receipt_idempotency_key_accepted_count",
        "artifact_distribution_signing_notarization_receipt_idempotency_state_recorded_count",
        "artifact_distribution_signing_notarization_receipt_idempotency_state_persisted_count",
        "artifact_distribution_signing_notarization_receipt_hash_status_rebind_accepted_count",
        "release_publication_authority_from_signing_receipt_replay_derived_count",
        "activation_authority_from_signing_receipt_replay_derived_count",
        "install_from_signing_receipt_replay_executed_count",
        "service_restart_from_signing_receipt_replay_performed_count",
        "active_binary_from_signing_receipt_replay_mutated_count",
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

    let surfaces =
        value["artifact_distribution_signing_notarization_receipt_replay_idempotency_surfaces"]
            .as_array()
            .expect("artifact signing receipt replay idempotency surfaces");
    assert_eq!(surfaces.len(), 18);
    assert_eq!(
        surfaces[0]["surface"],
        "source_signing_notarization_result_receipt_no_persistence_report_required"
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(
                |surface| surface["signature_manifest_receipt_idempotency_key_requested"] == true
            )
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["notarization_ticket_stale_nonce_replay_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["telegram_signing_receipt_delivery_replay_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(
                |surface| surface["install_restart_active_binary_replay_path_requested"] == true
            )
            .count(),
        1
    );
    for surface in surfaces {
        assert_eq!(
            surface["artifact_distribution_signing_notarization_receipt_replay_idempotency_attempted"],
            true
        );
        assert_eq!(
            surface["artifact_distribution_signing_notarization_receipt_replay_idempotency_noop_confirmed"],
            true
        );
        for key in [
            "artifact_distribution_signing_notarization_receipt_replay_idempotency_allowed",
            "artifact_distribution_signing_notarization_receipt_replay_allowed",
            "artifact_distribution_signing_notarization_receipt_replay_accepted",
            "artifact_distribution_signing_notarization_receipt_replay_recorded",
            "artifact_distribution_signing_notarization_receipt_replay_persisted",
            "artifact_distribution_signing_notarization_receipt_duplicate_accepted",
            "artifact_distribution_signing_notarization_receipt_idempotency_key_accepted",
            "artifact_distribution_signing_notarization_receipt_idempotency_state_persisted",
            "artifact_distribution_signing_notarization_receipt_replay_nonce_accepted",
            "artifact_distribution_signing_notarization_receipt_cross_scope_reuse_accepted",
            "artifact_distribution_signing_notarization_receipt_status_upgrade_accepted",
            "artifact_distribution_signing_notarization_receipt_hash_status_rebind_accepted",
            "release_publication_authority_derived",
            "activation_authority_derived",
            "download_link_rendered",
            "install_command_rendered",
            "install_executed",
            "service_restarted",
            "active_binary_mutated",
            "memory_store_write_performed",
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

    let denied =
        value["denied_by_artifact_distribution_signing_notarization_receipt_replay_idempotency"]
            .as_array()
            .expect("artifact signing receipt replay idempotency denials");
    assert_eq!(denied.len(), 18);
    assert_eq!(
        value["denied_by_artifact_distribution_signing_notarization_receipt_replay_idempotency_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_ordering_monotonicity_denial_gate"
    );
    for key in [
        "artifact_distribution_signing_notarization_receipt_replay_idempotency_allowed",
        "artifact_distribution_signing_notarization_receipt_replay_allowed",
        "artifact_distribution_signing_notarization_receipt_replay_accepted",
        "artifact_distribution_signing_notarization_receipt_replay_recorded",
        "artifact_distribution_signing_notarization_receipt_replay_persisted",
        "artifact_distribution_signing_notarization_receipt_duplicate_accepted",
        "artifact_distribution_signing_notarization_receipt_idempotency_key_accepted",
        "artifact_distribution_signing_notarization_receipt_idempotency_state_persisted",
        "artifact_distribution_signing_notarization_receipt_cross_scope_reuse_accepted",
        "artifact_distribution_signing_notarization_receipt_status_upgrade_accepted",
        "artifact_distribution_signing_notarization_receipt_completed_status_accepted",
        "artifact_distribution_signing_notarization_receipt_hash_status_rebind_accepted",
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
        .expect("artifact signing receipt replay idempotency side effects");
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_artifact_signing_receipt_ordering_monotonicity_endpoint_blocks_ordering_and_authority()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_RECEIPT_ORDERING_MONOTONICITY_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body)
        .expect("artifact signing receipt ordering monotonicity route json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_RECEIPT_ORDERING_MONOTONICITY_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-ordering-monotonicity-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_ordering_monotonicity_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_ordering_monotonicity_denial_ready"],
        true
    );
    assert_eq!(
        value["source_artifact_distribution_signing_notarization_receipt_replay_idempotency_ready"],
        true
    );
    assert_eq!(
        value["source_artifact_distribution_signing_notarization_receipt_replay_idempotency_surface_count"],
        18
    );
    assert_eq!(
        value["artifact_distribution_signing_notarization_receipt_ordering_monotonicity_surface_count"],
        18
    );
    assert_eq!(
        value["artifact_distribution_signing_notarization_receipt_ordering_monotonicity_attempt_count"],
        18
    );
    assert_eq!(
        value["artifact_distribution_signing_notarization_receipt_ordering_monotonicity_denied_count"],
        18
    );
    for key in [
        "source_artifact_distribution_signing_notarization_receipt_replay_idempotency_allowed_count",
        "source_artifact_distribution_signing_notarization_receipt_replay_accepted_count",
        "source_artifact_distribution_signing_notarization_receipt_idempotency_state_persisted_count",
        "source_release_publication_authority_from_signing_receipt_replay_derived_count",
        "source_activation_authority_from_signing_receipt_replay_derived_count",
        "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_allowed_count",
        "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_accepted_count",
        "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_recorded_count",
        "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_persisted_count",
        "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_filesystem_written_count",
        "artifact_distribution_signing_notarization_receipt_sequence_cursor_recorded_count",
        "artifact_distribution_signing_notarization_receipt_sequence_cursor_persisted_count",
        "artifact_distribution_signing_notarization_receipt_monotonicity_state_recorded_count",
        "artifact_distribution_signing_notarization_receipt_monotonicity_state_persisted_count",
        "artifact_distribution_signing_notarization_receipt_duplicate_sequence_accepted_count",
        "artifact_distribution_signing_notarization_receipt_stale_sequence_accepted_count",
        "artifact_distribution_signing_notarization_receipt_late_arrival_accepted_count",
        "artifact_distribution_signing_notarization_receipt_future_gap_accepted_count",
        "artifact_distribution_signing_notarization_receipt_timestamp_rollback_accepted_count",
        "artifact_distribution_signing_notarization_receipt_epoch_rollback_accepted_count",
        "artifact_distribution_signing_notarization_receipt_same_sequence_different_hash_accepted_count",
        "artifact_distribution_signing_notarization_receipt_latest_wins_overwrite_accepted_count",
        "artifact_distribution_signing_notarization_receipt_ordered_delivery_accepted_count",
        "artifact_distribution_signing_notarization_receipt_ordered_status_accepted_count",
        "artifact_signing_receipt_ordering_accepted_count",
        "package_signing_receipt_ordering_accepted_count",
        "release_publication_authority_from_signing_receipt_ordering_derived_count",
        "activation_authority_from_signing_receipt_ordering_derived_count",
        "install_from_signing_receipt_ordering_executed_count",
        "service_restart_from_signing_receipt_ordering_performed_count",
        "active_binary_from_signing_receipt_ordering_mutated_count",
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

    let surfaces =
        value["artifact_distribution_signing_notarization_receipt_ordering_monotonicity_surfaces"]
            .as_array()
            .expect("artifact signing receipt ordering monotonicity surfaces");
    assert_eq!(surfaces.len(), 18);
    assert_eq!(
        surfaces[0]["surface"],
        "source_signing_receipt_replay_idempotency_report_required"
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(
                |surface| surface["notarization_submission_receipt_future_gap_requested"] == true
            )
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["stapling_receipt_epoch_rollback_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["telegram_ordered_delivery_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(
                |surface| surface["install_restart_active_binary_ordering_path_requested"] == true
            )
            .count(),
        1
    );
    for surface in surfaces {
        assert_eq!(
            surface["artifact_distribution_signing_notarization_receipt_ordering_monotonicity_attempted"],
            true
        );
        assert_eq!(
            surface["artifact_distribution_signing_notarization_receipt_ordering_monotonicity_noop_confirmed"],
            true
        );
        for key in [
            "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_allowed",
            "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_accepted",
            "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_recorded",
            "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_persisted",
            "artifact_distribution_signing_notarization_receipt_sequence_cursor_recorded",
            "artifact_distribution_signing_notarization_receipt_monotonicity_state_persisted",
            "artifact_distribution_signing_notarization_receipt_duplicate_sequence_accepted",
            "artifact_distribution_signing_notarization_receipt_late_arrival_accepted",
            "artifact_distribution_signing_notarization_receipt_future_gap_accepted",
            "artifact_distribution_signing_notarization_receipt_latest_wins_overwrite_accepted",
            "release_publication_authority_derived",
            "activation_authority_derived",
            "download_link_rendered",
            "install_command_rendered",
            "install_executed",
            "service_restarted",
            "active_binary_mutated",
            "memory_store_write_performed",
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

    let denied =
        value["denied_by_artifact_distribution_signing_notarization_receipt_ordering_monotonicity"]
            .as_array()
            .expect("artifact signing receipt ordering monotonicity denials");
    assert_eq!(denied.len(), 18);
    assert_eq!(
        value["denied_by_artifact_distribution_signing_notarization_receipt_ordering_monotonicity_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_cancellation_supersession_denial_gate"
    );
    for key in [
        "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_accepted",
        "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_recorded",
        "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_persisted",
        "artifact_distribution_signing_notarization_receipt_sequence_cursor_recorded",
        "artifact_distribution_signing_notarization_receipt_sequence_cursor_persisted",
        "artifact_distribution_signing_notarization_receipt_monotonicity_state_recorded",
        "artifact_distribution_signing_notarization_receipt_monotonicity_state_persisted",
        "artifact_distribution_signing_notarization_receipt_latest_wins_overwrite_accepted",
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
        .expect("artifact signing receipt ordering monotonicity side effects");
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_artifact_signing_receipt_cancellation_supersession_endpoint_blocks_lifecycle_and_authority()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_RECEIPT_CANCELLATION_SUPERSESSION_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body)
        .expect("artifact signing receipt cancellation supersession route json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_RECEIPT_CANCELLATION_SUPERSESSION_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-cancellation-supersession-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_cancellation_supersession_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_cancellation_supersession_denial_ready"],
        true
    );
    assert_eq!(
        value["source_artifact_distribution_signing_notarization_receipt_ordering_monotonicity_ready"],
        true
    );
    assert_eq!(
        value["source_artifact_distribution_signing_notarization_receipt_ordering_monotonicity_surface_count"],
        18
    );
    assert_eq!(
        value["source_artifact_distribution_signing_notarization_receipt_ordering_monotonicity_attempt_count"],
        18
    );
    assert_eq!(
        value["source_artifact_distribution_signing_notarization_receipt_ordering_monotonicity_denied_count"],
        18
    );
    assert_eq!(
        value["artifact_distribution_signing_notarization_receipt_cancellation_supersession_surface_count"],
        18
    );
    assert_eq!(
        value["artifact_distribution_signing_notarization_receipt_cancellation_supersession_attempt_count"],
        18
    );
    assert_eq!(
        value["artifact_distribution_signing_notarization_receipt_cancellation_supersession_denied_count"],
        18
    );

    for key in [
        "source_artifact_distribution_signing_notarization_receipt_ordering_monotonicity_accepted_count",
        "source_artifact_distribution_signing_notarization_receipt_monotonicity_state_persisted_count",
        "source_release_publication_authority_from_signing_receipt_ordering_derived_count",
        "source_activation_authority_from_signing_receipt_ordering_derived_count",
        "artifact_distribution_signing_notarization_receipt_cancellation_supersession_allowed_count",
        "artifact_distribution_signing_notarization_receipt_cancellation_supersession_accepted_count",
        "artifact_distribution_signing_notarization_receipt_cancellation_supersession_recorded_count",
        "artifact_distribution_signing_notarization_receipt_cancellation_supersession_persisted_count",
        "artifact_distribution_signing_notarization_receipt_cancellation_supersession_filesystem_written_count",
        "artifact_distribution_signing_notarization_receipt_cancellation_accepted_count",
        "artifact_distribution_signing_notarization_receipt_cancellation_recorded_count",
        "artifact_distribution_signing_notarization_receipt_supersession_accepted_count",
        "artifact_distribution_signing_notarization_receipt_replacement_receipt_accepted_count",
        "artifact_distribution_signing_notarization_receipt_tombstone_recorded_count",
        "artifact_distribution_signing_notarization_receipt_delete_marker_recorded_count",
        "artifact_distribution_signing_notarization_receipt_lifecycle_cancellation_supersession_persisted_count",
        "release_publication_authority_from_signing_receipt_cancellation_derived_count",
        "activation_authority_from_signing_receipt_supersession_derived_count",
        "install_from_signing_receipt_cancellation_executed_count",
        "service_restart_from_signing_receipt_supersession_performed_count",
        "active_binary_from_signing_receipt_cancellation_mutated_count",
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

    let surfaces = value["artifact_distribution_signing_notarization_receipt_cancellation_supersession_surfaces"]
            .as_array()
            .expect("artifact signing receipt cancellation supersession surfaces");
    assert_eq!(surfaces.len(), 18);
    assert_eq!(
        surfaces[0]["surface"],
        "source_signing_receipt_ordering_monotonicity_report_required"
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(
                |surface| surface["signature_manifest_late_arrival_withdrawal_requested"] == true
            )
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["stapling_epoch_rollback_tombstone_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["telegram_supersession_delivery_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(
                |surface| surface["install_restart_active_binary_cancellation_path_requested"]
                    == true
            )
            .count(),
        1
    );
    for surface in surfaces {
        assert_eq!(
            surface["artifact_distribution_signing_notarization_receipt_cancellation_supersession_attempted"],
            true
        );
        assert_eq!(
            surface["artifact_distribution_signing_notarization_receipt_cancellation_supersession_noop_confirmed"],
            true
        );
        for key in [
            "artifact_distribution_signing_notarization_receipt_cancellation_supersession_allowed",
            "artifact_distribution_signing_notarization_receipt_cancellation_supersession_accepted",
            "artifact_distribution_signing_notarization_receipt_cancellation_supersession_recorded",
            "artifact_distribution_signing_notarization_receipt_cancellation_supersession_persisted",
            "artifact_distribution_signing_notarization_receipt_cancellation_accepted",
            "artifact_distribution_signing_notarization_receipt_withdrawal_accepted",
            "artifact_distribution_signing_notarization_receipt_supersession_accepted",
            "artifact_distribution_signing_notarization_receipt_replacement_receipt_accepted",
            "artifact_distribution_signing_notarization_receipt_tombstone_recorded",
            "artifact_distribution_signing_notarization_receipt_delete_marker_recorded",
            "artifact_distribution_signing_notarization_receipt_lifecycle_cancellation_supersession_persisted",
            "external_supersession_delivery_accepted",
            "telegram_supersession_delivery_accepted",
            "release_publication_authority_derived",
            "activation_authority_derived",
            "download_link_rendered",
            "install_command_rendered",
            "install_executed",
            "service_restarted",
            "active_binary_mutated",
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

    let denied = value["denied_by_artifact_distribution_signing_notarization_receipt_cancellation_supersession"]
            .as_array()
            .expect("artifact signing receipt cancellation supersession denials");
    assert_eq!(denied.len(), 17);
    assert_eq!(
        value["denied_by_artifact_distribution_signing_notarization_receipt_cancellation_supersession_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_audit_evidence_denial_gate"
    );
    for key in [
        "artifact_distribution_signing_notarization_receipt_cancellation_supersession_accepted",
        "artifact_distribution_signing_notarization_receipt_cancellation_supersession_recorded",
        "artifact_distribution_signing_notarization_receipt_cancellation_supersession_persisted",
        "artifact_distribution_signing_notarization_receipt_cancellation_accepted",
        "artifact_distribution_signing_notarization_receipt_cancellation_recorded",
        "artifact_distribution_signing_notarization_receipt_cancellation_persisted",
        "artifact_distribution_signing_notarization_receipt_withdrawal_accepted",
        "artifact_distribution_signing_notarization_receipt_supersession_accepted",
        "artifact_distribution_signing_notarization_receipt_supersession_recorded",
        "artifact_distribution_signing_notarization_receipt_supersession_persisted",
        "artifact_distribution_signing_notarization_receipt_replacement_receipt_accepted",
        "artifact_distribution_signing_notarization_receipt_tombstone_recorded",
        "artifact_distribution_signing_notarization_receipt_delete_marker_recorded",
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
        .expect("artifact signing receipt cancellation supersession side effects");
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_artifact_signing_receipt_audit_evidence_endpoint_blocks_audit_and_authority()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_RECEIPT_AUDIT_EVIDENCE_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value =
        serde_json::from_str(&body).expect("artifact signing receipt audit evidence route json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_RECEIPT_AUDIT_EVIDENCE_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-audit-evidence-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_audit_evidence_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_audit_evidence_denial_ready"],
        true
    );
    assert_eq!(
        value["source_artifact_distribution_signing_notarization_receipt_cancellation_supersession_ready"],
        true
    );
    assert_eq!(
        value["source_artifact_distribution_signing_notarization_receipt_cancellation_supersession_surface_count"],
        18
    );
    assert_eq!(
        value["source_artifact_distribution_signing_notarization_receipt_cancellation_supersession_denied_count"],
        18
    );
    assert_eq!(
        value["artifact_distribution_signing_notarization_receipt_audit_evidence_surface_count"],
        18
    );
    assert_eq!(
        value["artifact_distribution_signing_notarization_receipt_audit_evidence_attempt_count"],
        18
    );
    assert_eq!(
        value["artifact_distribution_signing_notarization_receipt_audit_evidence_denied_count"],
        18
    );

    for key in [
        "source_artifact_distribution_signing_notarization_receipt_cancellation_supersession_accepted_count",
        "source_artifact_distribution_signing_notarization_receipt_lifecycle_cancellation_supersession_persisted_count",
        "source_release_publication_authority_from_signing_receipt_cancellation_derived_count",
        "source_activation_authority_from_signing_receipt_supersession_derived_count",
        "artifact_distribution_signing_notarization_receipt_audit_evidence_allowed_count",
        "artifact_distribution_signing_notarization_receipt_audit_evidence_accepted_count",
        "artifact_distribution_signing_notarization_receipt_audit_evidence_recorded_count",
        "artifact_distribution_signing_notarization_receipt_audit_evidence_persisted_count",
        "artifact_distribution_signing_notarization_receipt_audit_evidence_filesystem_written_count",
        "artifact_distribution_signing_notarization_receipt_audit_trail_recorded_count",
        "artifact_distribution_signing_notarization_receipt_immutable_evidence_recorded_count",
        "artifact_distribution_signing_notarization_receipt_hash_chain_recorded_count",
        "artifact_distribution_signing_notarization_receipt_attestation_recorded_count",
        "artifact_distribution_signing_notarization_receipt_ledger_recorded_count",
        "artifact_distribution_signing_notarization_receipt_delivery_evidence_delivered_count",
        "release_publication_authority_from_signing_receipt_audit_evidence_derived_count",
        "activation_authority_from_signing_receipt_audit_evidence_derived_count",
        "install_from_signing_receipt_audit_evidence_executed_count",
        "service_restart_from_signing_receipt_audit_evidence_performed_count",
        "active_binary_from_signing_receipt_audit_evidence_mutated_count",
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

    let surfaces =
        value["artifact_distribution_signing_notarization_receipt_audit_evidence_surfaces"]
            .as_array()
            .expect("artifact signing receipt audit evidence surfaces");
    assert_eq!(surfaces.len(), 18);
    assert_eq!(
        surfaces[0]["surface"],
        "source_signing_receipt_cancellation_supersession_report_required"
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["signature_manifest_withdrawal_hash_chain_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(
                |surface| surface["notarization_ticket_supersession_witness_notary_requested"]
                    == true
            )
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["telegram_audit_evidence_delivery_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["install_restart_active_binary_audit_path_requested"] == true)
            .count(),
        1
    );
    for surface in surfaces {
        assert_eq!(
            surface["artifact_distribution_signing_notarization_receipt_audit_evidence_attempted"],
            true
        );
        assert_eq!(
            surface["artifact_distribution_signing_notarization_receipt_audit_evidence_noop_confirmed"],
            true
        );
        for key in [
            "artifact_distribution_signing_notarization_receipt_audit_evidence_allowed",
            "artifact_distribution_signing_notarization_receipt_audit_evidence_accepted",
            "artifact_distribution_signing_notarization_receipt_audit_evidence_recorded",
            "artifact_distribution_signing_notarization_receipt_audit_evidence_persisted",
            "artifact_distribution_signing_notarization_receipt_audit_trail_recorded",
            "artifact_distribution_signing_notarization_receipt_immutable_evidence_recorded",
            "artifact_distribution_signing_notarization_receipt_hash_chain_recorded",
            "artifact_distribution_signing_notarization_receipt_ledger_recorded",
            "artifact_distribution_signing_notarization_receipt_delivery_evidence_delivered",
            "release_publication_authority_derived",
            "activation_authority_derived",
            "download_link_rendered",
            "install_command_rendered",
            "install_executed",
            "service_restarted",
            "active_binary_mutated",
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

    let denied =
        value["denied_by_artifact_distribution_signing_notarization_receipt_audit_evidence"]
            .as_array()
            .expect("artifact signing receipt audit evidence denials");
    assert_eq!(denied.len(), 14);
    assert_eq!(
        value["denied_by_artifact_distribution_signing_notarization_receipt_audit_evidence_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_denial_gate"
    );
    for key in [
        "artifact_distribution_signing_notarization_receipt_audit_evidence_accepted",
        "artifact_distribution_signing_notarization_receipt_audit_evidence_recorded",
        "artifact_distribution_signing_notarization_receipt_audit_evidence_persisted",
        "artifact_distribution_signing_notarization_receipt_audit_trail_recorded",
        "artifact_distribution_signing_notarization_receipt_immutable_evidence_recorded",
        "artifact_distribution_signing_notarization_receipt_hash_chain_recorded",
        "artifact_distribution_signing_notarization_receipt_attestation_recorded",
        "artifact_distribution_signing_notarization_receipt_ledger_recorded",
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
        .expect("artifact signing receipt audit evidence side effects");
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}

#[test]
fn hepta_memory_intelligence_kg_full_live_activation_artifact_signing_receipt_retention_expiry_garbage_collection_endpoint_blocks_lifecycle_mutation()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_RECEIPT_RETENTION_EXPIRY_GARBAGE_COLLECTION_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body)
        .expect("artifact signing receipt retention expiry garbage collection route json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_RECEIPT_RETENTION_EXPIRY_GARBAGE_COLLECTION_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-retention-expiry-garbage-collection-denial --json"
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
        value["memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_retention_expiry_garbage_collection_denial_route_enabled"],
        true
    );
    assert_eq!(
        value["memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_retention_expiry_garbage_collection_denial_ready"],
        true
    );
    assert_eq!(
        value["source_artifact_distribution_signing_notarization_receipt_audit_evidence_ready"],
        true
    );
    assert_eq!(
        value["source_artifact_distribution_signing_notarization_receipt_audit_evidence_surface_count"],
        18
    );
    assert_eq!(
        value["source_artifact_distribution_signing_notarization_receipt_audit_evidence_denied_count"],
        18
    );
    assert_eq!(
        value["artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_surface_count"],
        18
    );
    assert_eq!(
        value["artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_attempt_count"],
        18
    );
    assert_eq!(
        value["artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_denied_count"],
        18
    );

    for key in [
        "source_artifact_distribution_signing_notarization_receipt_audit_evidence_recorded_count",
        "source_artifact_distribution_signing_notarization_receipt_audit_trail_recorded_count",
        "source_artifact_distribution_signing_notarization_receipt_immutable_evidence_recorded_count",
        "source_artifact_distribution_signing_notarization_receipt_ledger_recorded_count",
        "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_allowed_count",
        "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_accepted_count",
        "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_recorded_count",
        "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_persisted_count",
        "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_filesystem_written_count",
        "artifact_distribution_signing_notarization_receipt_retention_policy_recorded_count",
        "artifact_distribution_signing_notarization_receipt_ttl_lease_recorded_count",
        "artifact_distribution_signing_notarization_receipt_expiry_timestamp_recorded_count",
        "artifact_distribution_signing_notarization_receipt_expiry_scheduler_recorded_count",
        "artifact_distribution_signing_notarization_receipt_expiry_timer_started_count",
        "artifact_distribution_signing_notarization_receipt_expiry_ack_recorded_count",
        "artifact_distribution_signing_notarization_receipt_garbage_collection_queue_recorded_count",
        "artifact_distribution_signing_notarization_receipt_garbage_collection_scan_performed_count",
        "artifact_distribution_signing_notarization_receipt_garbage_collection_decision_recorded_count",
        "artifact_distribution_signing_notarization_receipt_archive_recorded_count",
        "artifact_distribution_signing_notarization_receipt_compaction_recorded_count",
        "release_publication_authority_from_signing_receipt_retention_derived_count",
        "activation_authority_from_signing_receipt_retention_derived_count",
        "install_from_signing_receipt_retention_executed_count",
        "service_restart_from_signing_receipt_retention_performed_count",
        "active_binary_from_signing_receipt_retention_mutated_count",
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
            "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_surfaces"
        ]
        .as_array()
        .expect("artifact signing receipt retention expiry garbage collection surfaces");
    assert_eq!(surfaces.len(), 18);
    assert_eq!(
        surfaces[0]["surface"],
        "source_signing_receipt_audit_evidence_report_required"
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["retention_policy_requested"] == true)
            .count(),
        5
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["ttl_lease_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| {
                surface["expiry_scheduler_requested"] == true
                    && surface["expiry_timer_requested"] == true
                    && surface["expiry_ack_requested"] == true
            })
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["garbage_collection_queue_requested"] == true)
            .count(),
        2
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["archive_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["compaction_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["external_telegram_retention_requested"] == true)
            .count(),
        1
    );
    assert_eq!(
        surfaces
            .iter()
            .filter(|surface| surface["live_install_gc_evidence_requested"] == true)
            .count(),
        2
    );
    for surface in surfaces {
        assert_eq!(
            surface["artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_attempted"],
            true
        );
        assert_eq!(
            surface["artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_noop_confirmed"],
            true
        );
        for key in [
            "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_allowed",
            "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_accepted",
            "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_recorded",
            "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_persisted",
            "retention_policy_recorded",
            "ttl_lease_recorded",
            "expiry_timestamp_recorded",
            "expiry_scheduler_recorded",
            "expiry_timer_started",
            "garbage_collection_queue_recorded",
            "garbage_collection_scan_performed",
            "garbage_collection_decision_recorded",
            "archive_recorded",
            "compaction_recorded",
            "release_publication_authority_from_retention_derived",
            "activation_authority_from_retention_derived",
            "install_from_retention_executed",
            "service_restart_from_retention_performed",
            "active_binary_from_retention_mutated",
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
            "denied_by_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection"
        ]
        .as_array()
        .expect("artifact signing receipt retention expiry garbage collection denials");
    assert_eq!(denied.len(), 20);
    assert_eq!(
        value["denied_by_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_count"],
        serde_json::json!(denied.len())
    );
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_export_query_observability_denial_gate"
    );
    for key in [
        "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_accepted",
        "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_recorded",
        "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_persisted",
        "artifact_distribution_signing_notarization_receipt_retention_policy_recorded",
        "artifact_distribution_signing_notarization_receipt_expiry_recorded",
        "artifact_distribution_signing_notarization_receipt_garbage_collection_recorded",
        "artifact_distribution_signing_notarization_receipt_archive_recorded",
        "artifact_distribution_signing_notarization_receipt_compaction_recorded",
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
        .expect("artifact signing receipt retention expiry garbage collection side effects");
    assert!(
        side_effects
            .values()
            .all(|item| item.as_bool() == Some(false))
    );
}
