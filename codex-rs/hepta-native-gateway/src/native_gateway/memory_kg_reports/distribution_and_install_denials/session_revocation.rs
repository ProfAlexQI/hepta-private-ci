fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let source = hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_denial_report();
    let source_report_sha256 = sha256_text_value(&source.to_string());
    let source_contract_hash = source
        .get("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_contract_hash_sha256")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("")
        .to_string();
    let source_bool = |key: &str| -> bool {
        source
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let source_u64 = |key: &str| -> u64 {
        source
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_ready = source_bool(
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_denial_ready",
    );

    let surface_false_keys = [
        "operator_identity_revocation_requested",
        "operator_session_logout_requested",
        "session_revocation_requested",
        "session_logout_requested",
        "session_lifecycle_mutation_requested",
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
        "session_revocation_refresh_recorded",
        "identity_revocation_status_promoted",
        "session_logout_summary_promoted",
        "operator_approval_from_revocation_logout_derived",
        "acceptance_from_revocation_logout_recorded",
        "terminal_decision_from_revocation_logout_recorded",
        "terminal_status_from_revocation_logout_recorded",
        "release_publication_authority_from_revocation_logout_derived",
        "activation_authority_from_revocation_logout_derived",
        "download_link_from_revocation_logout_rendered",
        "install_command_from_revocation_logout_rendered",
        "install_from_revocation_logout_executed",
        "service_restart_from_revocation_logout_performed",
        "launchd_from_revocation_logout_mutated",
        "active_binary_from_revocation_logout_mutated",
        "result_receipt_from_revocation_logout_recorded",
        "result_receipt_from_revocation_logout_persisted",
        "identity_invalidation_requested",
        "revocation_token_requested",
        "logout_nonce_requested",
        "device_session_logout_requested",
        "session_revocation_refresh_requested",
        "identity_revocation_status_requested",
        "session_logout_summary_requested",
        "identity_badge_revocation_requested",
        "session_readback_logout_requested",
        "identity_dashboard_revocation_requested",
        "channel_session_logout_requested",
        "operator_identity_approval_revocation_requested",
        "telegram_identity_session_logout_revocation_requested",
        "authority_revocation_logout_requested",
        "live_session_revocation_requested",
        "install_restart_active_binary_session_revocation_requested",
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
    let surface_specs = [
        (
            "source_operator_identity_session_replay_cross_binding_report_required",
            "blocked_source_replay_cross_binding_required_noop",
            "source_operator_identity_session_replay_cross_binding_report_required",
            &["source_operator_identity_session_replay_cross_binding_report_required"][..],
        ),
        (
            "download_button_identity_revocation_claim",
            "blocked_identity_revocation_noop",
            "download_button_identity_revocation_claim_denied",
            &[
                "operator_identity_revocation_requested",
                "session_lifecycle_mutation_requested",
            ][..],
        ),
        (
            "direct_download_url_session_logout_claim",
            "blocked_session_logout_noop",
            "direct_download_url_session_logout_claim_denied",
            &[
                "operator_session_logout_requested",
                "session_logout_requested",
                "session_lifecycle_mutation_requested",
            ][..],
        ),
        (
            "checksum_prompt_identity_invalidation_claim",
            "blocked_identity_invalidation_noop",
            "checksum_prompt_identity_invalidation_claim_denied",
            &[
                "operator_identity_revocation_requested",
                "identity_invalidation_requested",
            ][..],
        ),
        (
            "package_manager_install_command_session_revocation_token_claim",
            "blocked_session_revocation_token_noop",
            "package_manager_install_command_session_revocation_token_claim_denied",
            &["session_revocation_requested", "revocation_token_requested"][..],
        ),
        (
            "curl_pipe_shell_logout_nonce_claim",
            "blocked_logout_nonce_noop",
            "curl_pipe_shell_logout_nonce_claim_denied",
            &[
                "operator_session_logout_requested",
                "logout_nonce_requested",
            ][..],
        ),
        (
            "installer_launch_prompt_device_session_logout_claim",
            "blocked_device_session_logout_noop",
            "installer_launch_prompt_device_session_logout_claim_denied",
            &[
                "operator_session_logout_requested",
                "device_session_logout_requested",
            ][..],
        ),
        (
            "auto_update_offer_session_revocation_refresh_claim",
            "blocked_session_revocation_refresh_noop",
            "auto_update_offer_session_revocation_refresh_claim_denied",
            &[
                "session_revocation_requested",
                "session_revocation_refresh_requested",
            ][..],
        ),
        (
            "release_channel_subscription_identity_revocation_status_claim",
            "blocked_identity_revocation_status_noop",
            "release_channel_subscription_identity_revocation_status_claim_denied",
            &[
                "operator_identity_revocation_requested",
                "identity_revocation_status_requested",
            ][..],
        ),
        (
            "update_feed_hint_session_logout_summary_claim",
            "blocked_session_logout_summary_noop",
            "update_feed_hint_session_logout_summary_claim_denied",
            &[
                "operator_session_logout_requested",
                "session_logout_summary_requested",
            ][..],
        ),
        (
            "package_registry_badge_operator_identity_badge_revocation_claim",
            "blocked_identity_badge_revocation_noop",
            "package_registry_badge_operator_identity_badge_revocation_claim_denied",
            &[
                "operator_identity_revocation_requested",
                "identity_badge_revocation_requested",
            ][..],
        ),
        (
            "cdn_mirror_download_session_readback_logout_claim",
            "blocked_session_readback_logout_noop",
            "cdn_mirror_download_session_readback_logout_claim_denied",
            &[
                "operator_session_logout_requested",
                "session_readback_logout_requested",
            ][..],
        ),
        (
            "sbom_provenance_notarization_identity_dashboard_revocation_claim",
            "blocked_identity_dashboard_revocation_noop",
            "sbom_provenance_notarization_identity_dashboard_revocation_claim_denied",
            &[
                "operator_identity_revocation_requested",
                "identity_dashboard_revocation_requested",
            ][..],
        ),
        (
            "signature_verification_command_channel_session_logout_claim",
            "blocked_channel_session_logout_noop",
            "signature_verification_command_channel_session_logout_claim_denied",
            &[
                "operator_session_logout_requested",
                "channel_session_logout_requested",
            ][..],
        ),
        (
            "one_click_install_deep_link_operator_identity_approval_revocation_claim",
            "blocked_identity_approval_revocation_noop",
            "one_click_install_deep_link_operator_identity_approval_revocation_claim_denied",
            &[
                "operator_identity_revocation_requested",
                "operator_identity_approval_revocation_requested",
            ][..],
        ),
        (
            "external_telegram_install_message_identity_session_logout_revocation_claim",
            "blocked_external_telegram_logout_revocation_noop",
            "external_telegram_install_message_identity_session_logout_revocation_claim_denied",
            &[
                "operator_identity_revocation_requested",
                "operator_session_logout_requested",
                "telegram_identity_session_logout_revocation_requested",
            ][..],
        ),
        (
            "release_publication_authority_identity_session_revocation_logout_claim",
            "blocked_authority_revocation_logout_noop",
            "release_publication_authority_identity_session_revocation_logout_claim_denied",
            &[
                "operator_identity_revocation_requested",
                "operator_session_logout_requested",
                "authority_revocation_logout_requested",
            ][..],
        ),
        (
            "activation_live_install_restart_active_binary_session_revocation_claim",
            "blocked_live_session_revocation_noop",
            "activation_live_install_restart_active_binary_session_revocation_claim_denied",
            &[
                "session_revocation_requested",
                "live_session_revocation_requested",
                "install_restart_active_binary_session_revocation_requested",
            ][..],
        ),
    ];
    let operator_identity_session_revocation_logout_surfaces = surface_specs
        .iter()
        .map(|(surface, status, reason, extra_true_keys)| {
            let mut surface_report = serde_json::json!({
                "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_surface": surface,
                "source_operator_identity_session_replay_cross_binding_ready": source_ready,
                "artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_attempted": true,
                "operator_identity_session_revocation_logout_noop_confirmed": true,
                "operator_identity_session_revocation_logout_status": status,
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
    let operator_identity_session_revocation_logout_surface_count =
        operator_identity_session_revocation_logout_surfaces.len();
    let contract_hash = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-denial:native:source={source_report_sha256}:surfaces={operator_identity_session_revocation_logout_surface_count}:route_count={}:revocation=0:logout=0:lifecycle=0:authority=0:install=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout:no-revocation:no-logout:no-lifecycle:no-approval:no-authority:no-install:no-live",
    );
    let denials = vec![
        "artifact_download_install_affordance_result_receipt_operator_identity_revocation_acceptance_denied",
        "artifact_download_install_affordance_result_receipt_operator_identity_revocation_recording_denied",
        "artifact_download_install_affordance_result_receipt_operator_session_logout_acceptance_denied",
        "artifact_download_install_affordance_result_receipt_operator_session_logout_recording_denied",
        "artifact_download_install_affordance_result_receipt_session_revocation_recording_denied",
        "artifact_download_install_affordance_result_receipt_session_logout_recording_denied",
        "artifact_download_install_affordance_result_receipt_identity_invalidation_recording_denied",
        "artifact_download_install_affordance_result_receipt_revocation_token_recording_denied",
        "artifact_download_install_affordance_result_receipt_logout_nonce_recording_denied",
        "artifact_download_install_affordance_result_receipt_device_session_logout_denied",
        "artifact_download_install_affordance_result_receipt_session_lifecycle_status_promotion_denied",
        "artifact_download_install_affordance_result_receipt_acceptance_from_revocation_logout_denied",
        "artifact_download_install_affordance_operator_approval_from_revocation_logout_denied",
        "artifact_download_install_affordance_release_publication_authority_from_revocation_logout_denied",
        "artifact_download_install_affordance_activation_authority_from_revocation_logout_denied",
        "artifact_download_install_affordance_download_install_from_revocation_logout_denied",
        "artifact_download_install_affordance_memory_provider_secret_external_send_from_revocation_logout_denied",
        "artifact_download_install_affordance_session_lifecycle_mutation_from_denied_receipt_denied",
    ];
    let denied_count = denials.len();
    let report_ready = source_ready
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_surface_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_denied_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_replay_recorded_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_session_replay_recorded_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cross_session_binding_recorded_count",
        ) == 0
        && operator_identity_session_revocation_logout_surface_count == 18
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-20",
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
        "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_denial_v1",
        "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_mode": "native_route_denied_replay_cross_binding_cannot_create_revocation_logout_or_session_lifecycle_authority",
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_gate": source["gate"].clone(),
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_ready": source_ready,
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_report_sha256": source_report_sha256,
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_contract_hash_sha256": source_contract_hash,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_contract_hash_sha256": contract_hash,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_policy_hash_sha256": policy_hash,
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
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_denial_route_enabled": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_denial_ready": report_ready,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_surface_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_surface_count"),
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_attempt_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_attempt_count"),
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_denied_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_denied_count"),
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_replay_recorded_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_replay_recorded_count"),
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_session_replay_recorded_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_session_replay_recorded_count"),
        "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cross_session_binding_recorded_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cross_session_binding_recorded_count"),
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_surface_count": operator_identity_session_revocation_logout_surface_count,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_attempt_count": operator_identity_session_revocation_logout_surface_count,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_denied_count": operator_identity_session_revocation_logout_surface_count,
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_surfaces": operator_identity_session_revocation_logout_surfaces,
        "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout": denials,
        "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_count": denied_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
        "allowed_next_actions": [
            {
                "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_denial_gate",
                "status": "allowed_report_only_next_slice",
                "records_operator_identity": false,
                "records_operator_session": false,
                "records_session_binding": false,
                "accepts_replay": false,
                "accepts_cross_session_binding": false,
                "records_revocation": false,
                "records_logout": false,
                "accepts_revocation_replay": false,
                "records_reinstatement": false,
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
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_revocation_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_revocation_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_revocation_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_session_logout_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_session_logout_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_session_logout_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_revocation_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_revocation_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_logout_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_logout_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_identity_invalidation_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_revocation_token_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_logout_nonce_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_device_session_logout_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_revocation_refresh_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_identity_revocation_status_promoted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_logout_summary_promoted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_approval_from_revocation_logout_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_acceptance_from_revocation_logout_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_service_restart_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_active_binary_mutated_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_external_send_count",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in &zero_keys {
            report_object.insert((*key).to_string(), serde_json::json!(0));
        }
    }

    let false_keys = [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_revocation_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_session_logout_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_lifecycle_recorded",
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
fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source = hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_denial_report();
    let source_report_sha256 = sha256_text_value(&source.to_string());
    let source_contract_hash = source
        .get("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_contract_hash_sha256")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("")
        .to_string();
    let source_bool = |key: &str| -> bool {
        source
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let source_u64 = |key: &str| -> u64 {
        source
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_ready = source_bool(
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_denial_ready",
    );

    let surface_false_keys = [
        "revocation_logout_replay_requested",
        "identity_revocation_replay_requested",
        "logout_replay_requested",
        "session_logout_replay_requested",
        "identity_reinstatement_requested",
        "session_reinstatement_requested",
        "session_lifecycle_mutation_requested",
        "reinstatement_token_requested",
        "revocation_replay_nonce_requested",
        "device_session_reinstatement_requested",
        "session_logout_replay_refresh_requested",
        "identity_revocation_replay_status_requested",
        "reinstatement_summary_requested",
        "identity_badge_reinstatement_requested",
        "session_readback_logout_replay_requested",
        "identity_dashboard_reinstatement_requested",
        "channel_session_reinstatement_requested",
        "operator_identity_approval_reinstatement_requested",
        "telegram_identity_session_reinstatement_requested",
        "authority_revocation_logout_replay_reinstatement_requested",
        "live_session_reinstatement_requested",
        "install_restart_active_binary_session_reinstatement_requested",
        "source_operator_identity_session_revocation_logout_report_required",
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
        "launchd_from_revocation_logout_replay_reinstatement_mutated",
        "active_binary_from_revocation_logout_replay_reinstatement_mutated",
        "result_receipt_from_revocation_logout_replay_reinstatement_recorded",
        "result_receipt_from_revocation_logout_replay_reinstatement_persisted",
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
    let surface_specs = [
        (
            "source_operator_identity_session_revocation_logout_report_required",
            "blocked_source_revocation_logout_required_noop",
            "source_operator_identity_session_revocation_logout_report_required",
            &["source_operator_identity_session_revocation_logout_report_required"][..],
        ),
        (
            "download_button_identity_revocation_replay_claim",
            "blocked_identity_revocation_replay_noop",
            "download_button_identity_revocation_replay_claim_denied",
            &[
                "revocation_logout_replay_requested",
                "identity_revocation_replay_requested",
                "session_lifecycle_mutation_requested",
            ][..],
        ),
        (
            "direct_download_url_session_logout_replay_claim",
            "blocked_session_logout_replay_noop",
            "direct_download_url_session_logout_replay_claim_denied",
            &[
                "logout_replay_requested",
                "session_logout_replay_requested",
                "session_lifecycle_mutation_requested",
            ][..],
        ),
        (
            "checksum_prompt_identity_reinstatement_claim",
            "blocked_identity_reinstatement_noop",
            "checksum_prompt_identity_reinstatement_claim_denied",
            &[
                "identity_reinstatement_requested",
                "session_lifecycle_mutation_requested",
            ][..],
        ),
        (
            "package_manager_install_command_session_reinstatement_token_claim",
            "blocked_session_reinstatement_token_noop",
            "package_manager_install_command_session_reinstatement_token_claim_denied",
            &[
                "session_reinstatement_requested",
                "reinstatement_token_requested",
            ][..],
        ),
        (
            "curl_pipe_shell_revocation_replay_nonce_claim",
            "blocked_revocation_replay_nonce_noop",
            "curl_pipe_shell_revocation_replay_nonce_claim_denied",
            &[
                "revocation_logout_replay_requested",
                "revocation_replay_nonce_requested",
            ][..],
        ),
        (
            "installer_launch_prompt_device_session_reinstatement_claim",
            "blocked_device_session_reinstatement_noop",
            "installer_launch_prompt_device_session_reinstatement_claim_denied",
            &[
                "session_reinstatement_requested",
                "device_session_reinstatement_requested",
            ][..],
        ),
        (
            "auto_update_offer_session_logout_replay_refresh_claim",
            "blocked_session_logout_replay_refresh_noop",
            "auto_update_offer_session_logout_replay_refresh_claim_denied",
            &[
                "logout_replay_requested",
                "session_logout_replay_refresh_requested",
            ][..],
        ),
        (
            "release_channel_subscription_identity_revocation_replay_status_claim",
            "blocked_identity_revocation_replay_status_noop",
            "release_channel_subscription_identity_revocation_replay_status_claim_denied",
            &[
                "revocation_logout_replay_requested",
                "identity_revocation_replay_status_requested",
            ][..],
        ),
        (
            "update_feed_hint_session_reinstatement_summary_claim",
            "blocked_session_reinstatement_summary_noop",
            "update_feed_hint_session_reinstatement_summary_claim_denied",
            &[
                "session_reinstatement_requested",
                "reinstatement_summary_requested",
            ][..],
        ),
        (
            "package_registry_badge_operator_identity_badge_reinstatement_claim",
            "blocked_identity_badge_reinstatement_noop",
            "package_registry_badge_operator_identity_badge_reinstatement_claim_denied",
            &[
                "identity_reinstatement_requested",
                "identity_badge_reinstatement_requested",
            ][..],
        ),
        (
            "cdn_mirror_download_session_readback_logout_replay_claim",
            "blocked_session_readback_logout_replay_noop",
            "cdn_mirror_download_session_readback_logout_replay_claim_denied",
            &[
                "logout_replay_requested",
                "session_readback_logout_replay_requested",
            ][..],
        ),
        (
            "sbom_provenance_notarization_identity_dashboard_reinstatement_claim",
            "blocked_identity_dashboard_reinstatement_noop",
            "sbom_provenance_notarization_identity_dashboard_reinstatement_claim_denied",
            &[
                "identity_reinstatement_requested",
                "identity_dashboard_reinstatement_requested",
            ][..],
        ),
        (
            "signature_verification_command_channel_session_reinstatement_claim",
            "blocked_channel_session_reinstatement_noop",
            "signature_verification_command_channel_session_reinstatement_claim_denied",
            &[
                "session_reinstatement_requested",
                "channel_session_reinstatement_requested",
            ][..],
        ),
        (
            "one_click_install_deep_link_operator_identity_approval_reinstatement_claim",
            "blocked_identity_approval_reinstatement_noop",
            "one_click_install_deep_link_operator_identity_approval_reinstatement_claim_denied",
            &[
                "identity_reinstatement_requested",
                "operator_identity_approval_reinstatement_requested",
            ][..],
        ),
        (
            "external_telegram_install_message_identity_session_reinstatement_claim",
            "blocked_external_telegram_reinstatement_noop",
            "external_telegram_install_message_identity_session_reinstatement_claim_denied",
            &[
                "identity_reinstatement_requested",
                "session_reinstatement_requested",
                "telegram_identity_session_reinstatement_requested",
            ][..],
        ),
        (
            "release_publication_authority_revocation_logout_replay_reinstatement_claim",
            "blocked_authority_revocation_logout_replay_reinstatement_noop",
            "release_publication_authority_revocation_logout_replay_reinstatement_claim_denied",
            &[
                "revocation_logout_replay_requested",
                "session_reinstatement_requested",
                "authority_revocation_logout_replay_reinstatement_requested",
            ][..],
        ),
        (
            "activation_live_install_restart_active_binary_session_reinstatement_claim",
            "blocked_live_session_reinstatement_noop",
            "activation_live_install_restart_active_binary_session_reinstatement_claim_denied",
            &[
                "session_reinstatement_requested",
                "live_session_reinstatement_requested",
                "install_restart_active_binary_session_reinstatement_requested",
            ][..],
        ),
    ];
    let surfaces = surface_specs
        .iter()
        .map(|(surface, status, reason, extra_true_keys)| {
            let mut surface_report = serde_json::json!({
                "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_surface": surface,
                "source_operator_identity_session_revocation_logout_ready": source_ready,
                "artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_attempted": true,
                "operator_identity_session_revocation_logout_replay_reinstatement_noop_confirmed": true,
                "operator_identity_session_revocation_logout_replay_reinstatement_status": status,
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
    let surface_count = surfaces.len();
    let contract_hash = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-denial:native:source={source_report_sha256}:surfaces={surface_count}:route_count={}:replay=0:reinstatement=0:lifecycle=0:authority=0:install=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement:no-replay:no-reinstatement:no-lifecycle:no-approval:no-authority:no-install:no-live",
    );
    let denials = vec![
        "artifact_download_install_affordance_result_receipt_revocation_replay_acceptance_denied",
        "artifact_download_install_affordance_result_receipt_revocation_replay_recording_denied",
        "artifact_download_install_affordance_result_receipt_logout_replay_acceptance_denied",
        "artifact_download_install_affordance_result_receipt_logout_replay_recording_denied",
        "artifact_download_install_affordance_result_receipt_identity_reinstatement_recording_denied",
        "artifact_download_install_affordance_result_receipt_session_reinstatement_recording_denied",
        "artifact_download_install_affordance_result_receipt_reinstatement_token_recording_denied",
        "artifact_download_install_affordance_result_receipt_reinstatement_nonce_recording_denied",
        "artifact_download_install_affordance_result_receipt_device_session_reinstatement_denied",
        "artifact_download_install_affordance_result_receipt_reinstatement_status_promotion_denied",
        "artifact_download_install_affordance_result_receipt_acceptance_from_revocation_logout_replay_reinstatement_denied",
        "artifact_download_install_affordance_operator_approval_from_revocation_logout_replay_reinstatement_denied",
        "artifact_download_install_affordance_release_publication_authority_from_revocation_logout_replay_reinstatement_denied",
        "artifact_download_install_affordance_activation_authority_from_revocation_logout_replay_reinstatement_denied",
        "artifact_download_install_affordance_download_install_from_revocation_logout_replay_reinstatement_denied",
        "artifact_download_install_affordance_memory_provider_secret_external_send_from_revocation_logout_replay_reinstatement_denied",
        "artifact_download_install_affordance_session_lifecycle_mutation_from_denied_receipt_denied",
    ];
    let report_ready = source_ready
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_surface_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_denied_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_revocation_recorded_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_session_logout_recorded_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_revocation_recorded_count",
        ) == 0
        && surface_count == 18
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-20",
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
            "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_denial_v1",
            "receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_mode": "native_route_denied_revocation_logout_cannot_create_replay_reinstatement_or_session_lifecycle_authority",
            "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_gate": source["gate"].clone(),
            "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_ready": source_ready,
            "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_report_sha256": source_report_sha256,
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_contract_hash_sha256": source_contract_hash,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_contract_hash_sha256": contract_hash,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_policy_hash_sha256": policy_hash,
            "minimum_required_samples": 24,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
            "route_count": route_matrix.route_count,
            "implemented_route_count": route_matrix.implemented_route_count,
            "missing_route_count": route_matrix.missing_route_count,
            "route_count_source_command_accepted": route_count_source_command_accepted,
            "source_route_wired": true,
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_denial_route_enabled": true,
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_denial_ready": report_ready,
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_surface_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_surface_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_attempt_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_attempt_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_denied_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_denied_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_revocation_recorded_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_revocation_recorded_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_session_logout_recorded_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_session_logout_recorded_count"),
            "source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_revocation_recorded_count": source_u64("release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_revocation_recorded_count"),
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_surface_count": surface_count,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_attempt_count": surface_count,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_denied_count": surface_count,
            "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_surfaces": surfaces,
            "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement": denials,
            "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_count": denials.len(),
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_denial_gate",
                    "status": "allowed_report_only_next_slice",
                    "records_operator_identity": false,
                    "records_operator_session": false,
                    "records_session_binding": false,
                    "accepts_replay": false,
                    "accepts_cross_session_binding": false,
                    "records_revocation": false,
                    "records_logout": false,
                    "accepts_revocation_replay": false,
                    "records_reinstatement": false,
                    "accepts_reinstatement_replay": false,
                    "records_ordering": false,
                    "records_monotonicity": false,
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
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_revocation_logout_replay_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_revocation_logout_replay_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_revocation_logout_replay_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_logout_replay_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_logout_replay_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_logout_replay_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_identity_reinstatement_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_identity_reinstatement_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_reinstatement_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_reinstatement_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_reinstatement_token_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_reinstatement_nonce_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_device_session_reinstatement_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_logout_replay_refresh_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_identity_revocation_replay_status_promoted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_reinstatement_summary_promoted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_approval_from_revocation_logout_replay_reinstatement_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_acceptance_from_revocation_logout_replay_reinstatement_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_replay_reinstatement_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_replay_reinstatement_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_replay_reinstatement_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_replay_reinstatement_service_restart_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_replay_reinstatement_active_binary_mutated_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_replay_reinstatement_external_send_count",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in &zero_keys {
            report_object.insert((*key).to_string(), serde_json::json!(0));
        }
    }
    let false_keys = [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_revocation_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_session_logout_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_lifecycle_recorded",
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
