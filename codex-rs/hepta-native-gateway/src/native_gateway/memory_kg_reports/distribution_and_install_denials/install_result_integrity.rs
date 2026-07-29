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

