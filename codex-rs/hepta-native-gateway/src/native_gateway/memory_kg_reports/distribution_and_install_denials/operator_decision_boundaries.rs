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

include!("session_revocation.rs");
